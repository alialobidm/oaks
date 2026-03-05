use crate::{CommentProcessor, Document, FormatResult, FormatState, FormatterConfig};
use alloc::{boxed::Box, string::String, sync::Arc, vec::Vec};
use oak_core::{
    language::Language,
    tree::{RedLeaf, RedNode, RedTree},
};

/// Formatted output
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "camelCase"))]
pub struct FormatOutput {
    /// The formatted code string
    pub content: String,
    /// Indicates if the content was changed during formatting
    pub changed: bool,
}

impl FormatOutput {
    /// Creates a new format output
    pub fn new(content: String, changed: bool) -> Self {
        Self { content, changed }
    }
}

/// A path node for efficiently recording the formatting path
#[derive(Debug)]
pub struct PathNode<L: Language> {
    /// The element type of the node
    pub kind: L::ElementType,
    /// The parent path node
    pub parent: Option<Arc<PathNode<L>>>,
}

/// Formatting context for managing state during the formatting process
/// 
/// This struct holds the state used during the formatting process, including
/// configuration, comment processing, and formatting state.
/// 
/// The `C` type parameter represents the type of language-specific configuration
/// used throughout the formatting process, which must implement `FormatterConfig`.
/// The `S` type parameter represents the type of formatting state
/// used throughout the formatting process.
#[derive(Debug, Clone)]
pub struct FormatContext<L: Language, C, S> {
    /// Language-specific configuration
    pub config: Arc<C>,
    /// Comment processor for handling comments during formatting
    pub comment_processor: Arc<CommentProcessor>,
    /// Source code content
    pub source: Option<Arc<str>>,
    /// Current nesting depth
    pub depth: usize,
    /// Path of parent node types
    pub path: Option<Arc<PathNode<L>>>,
    /// Formatting state
    pub state: S,
}

impl<L: Language, C: FormatterConfig> FormatContext<L, C, <C as FormatterConfig>::State> {
    /// Creates a new formatting context
    pub fn new(config: C) -> Self {
        let config = Arc::new(config);
        let comment_processor = Arc::new(CommentProcessor::new());
        let state = config.state();
        Self { config, comment_processor, source: None, depth: 0, path: None, state }
    }
}

impl<L: Language, C: FormatterConfig, S: Clone> FormatContext<L, C, S> {
    /// Creates a new formatting context with custom state
    pub fn new_with_state(config: C, state: S) -> Self {
        let config = Arc::new(config);
        let comment_processor = Arc::new(CommentProcessor::new());
        Self { config, comment_processor, source: None, depth: 0, path: None, state }
    }

    /// Enters a child node, increasing depth and recording the path
    pub fn enter(&self, kind: L::ElementType) -> Self {
        let path = Some(Arc::new(PathNode { kind, parent: self.path.clone() }));
        Self { 
            config: self.config.clone(), 
            comment_processor: self.comment_processor.clone(), 
            source: self.source.clone(), 
            depth: self.depth + 1, 
            path,
            state: self.state.clone(),
        }
    }

    /// Enters a child node with custom state
    pub fn enter_with_state(&self, kind: L::ElementType, state: S) -> Self {
        let path = Some(Arc::new(PathNode { kind, parent: self.path.clone() }));
        Self { 
            config: self.config.clone(), 
            comment_processor: self.comment_processor.clone(), 
            source: self.source.clone(), 
            depth: self.depth + 1, 
            path,
            state,
        }
    }

    /// Checks if the formatter is currently inside a node of a specific type
    pub fn is_inside(&self, kind: L::ElementType) -> bool {
        let mut current = self.path.as_ref();
        while let Some(node) = current {
            if node.kind == kind {
                return true;
            }
            current = node.parent.as_ref();
        }
        false
    }

    /// Gets the type of the parent node
    pub fn parent_kind(&self) -> Option<L::ElementType> {
        self.path.as_ref().and_then(|n| n.parent.as_ref()).map(|n| n.kind.clone())
    }
}

/// A generic formatter
/// 
/// This struct is used to format AST nodes using formatting functions.
/// It supports language-specific configuration and custom formatting state.
/// 
/// The `C` type parameter represents the language-specific configuration, which must implement `FormatterConfig`.
/// The `S` type parameter represents the formatting state.
pub struct Formatter<L: Language + 'static, C: FormatterConfig, S = <C as FormatterConfig>::State> {
    /// Node formatting functions
    node_formatters: Vec<Box<dyn for<'a> Fn(&RedNode<L>, &FormatContext<L, C, S>, &'a str, &dyn Fn(&RedNode<L>) -> FormatResult<Document<'a>>) -> FormatResult<Option<Document<'a>>>>>,
    /// Token formatting functions
    token_formatters: Vec<Box<dyn for<'a> Fn(&RedLeaf<L>, &FormatContext<L, C, S>, &'a str) -> FormatResult<Option<Document<'a>>>>>,
    /// Initial formatting context
    pub context: FormatContext<L, C, S>,
}

impl<L: Language + 'static, C: FormatterConfig> Formatter<L, C, <C as FormatterConfig>::State> {
    /// Creates a new formatter
    pub fn new(config: C) -> Self {
        Self { 
            node_formatters: Vec::new(), 
            token_formatters: Vec::new(), 
            context: FormatContext::new(config) 
        }
    }
}

impl<L: Language + 'static, C: FormatterConfig, S: Clone + 'static> Formatter<L, C, S> {
    /// Creates a new formatter with custom state
    pub fn new_with_state(config: C, state: S) -> Self {
        let mut formatter = Self { 
            node_formatters: Vec::new(), 
            token_formatters: Vec::new(), 
            context: FormatContext::new_with_state(config, state) 
        };

        // Add built-in formatters
        formatter.with_builtin_formatters()
    }

    /// Adds built-in formatting functions
    fn with_builtin_formatters(mut self) -> Self {
        // Add basic indentation formatter
        self.add_node_formatter(Box::new(|node, _ctx, source, format_children| {
            use oak_core::language::{ElementType, UniversalElementRole};
            if ElementType::is_universal(&node.green.kind, UniversalElementRole::Container) {
                let children_doc = format_children(node)?;
                Ok(Some(Document::group(Document::indent(Document::concat(vec![
                    Document::Line,
                    children_doc,
                ])))))
            } else {
                Ok(None)
            }
        }));

        // Add statement newline formatter
        self.add_node_formatter(Box::new(|node, _ctx, source, format_children| {
            use oak_core::language::{ElementType, UniversalElementRole};
            if ElementType::is_universal(&node.green.kind, UniversalElementRole::Statement) {
                let children_doc = format_children(node)?;
                Ok(Some(Document::concat(vec![children_doc, Document::Line])))
            } else {
                Ok(None)
            }
        }));

        // Add comma spacing formatter
        self.add_token_formatter(Box::new(|token, _ctx, source| {
            use oak_core::language::{TokenType, UniversalTokenRole};
            if TokenType::is_universal(&token.kind, UniversalTokenRole::Punctuation) {
                let text = &source[token.span.start..token.span.end];
                if text == "," {
                    let d = Document::concat(vec![Document::text(","), Document::SoftLineSpace]);
                    return Ok(Some(d))
                }
            }
            Ok(None)
        }));

        self
    }

    /// Adds a node formatting function
    pub fn add_node_formatter(&mut self, formatter: Box<dyn for<'a> Fn(&RedNode<L>, &FormatContext<L, C, S>, &'a str, &dyn Fn(&RedNode<L>) -> FormatResult<Document<'a>>) -> FormatResult<Option<Document<'a>>>>) {
        self.node_formatters.push(formatter);
    }

    /// Adds a token formatting function
    pub fn add_token_formatter(&mut self, formatter: Box<dyn for<'a> Fn(&RedLeaf<L>, &FormatContext<L, C, S>, &'a str) -> FormatResult<Option<Document<'a>>>>) {
        self.token_formatters.push(formatter);
    }

    /// Formats an AST node
    pub fn format<'a>(&mut self, root: &RedNode<L>, source: &'a str) -> FormatResult<FormatOutput> {
        self.context.source = Some(Arc::from(source));
        let doc = self.format_node(root, &self.context, source)?;
        let content = doc.render();
        let changed = content != source;
        Ok(FormatOutput::new(content, changed))
    }

    /// Recursively formats a node and generates a Document
    fn format_node<'a>(&self, node: &RedNode<L>, context: &FormatContext<L, C, S>, source: &'a str) -> FormatResult<Document<'a>> {
        // Create a new context, recording current path and depth
        let new_context = context.enter(node.green.kind.clone());

        // Create a closure for formatting child nodes
        let format_children = |n: &RedNode<L>| {
            let mut children_docs = Vec::new();
            for child in n.children() {
                match child {
                    RedTree::Node(child_node) => children_docs.push(self.format_node(&child_node, &new_context, source)?),
                    RedTree::Leaf(child_token) => children_docs.push(self.format_token(&child_token, &new_context, source)?),
                }
            }
            Ok(Document::Concat(children_docs))
        };

        // Apply node formatters
        for formatter in &self.node_formatters {
            if let Some(doc) = formatter(node, &new_context, source, &format_children)? {
                return Ok(doc);
            }
        }

        // Default logic: format all child nodes and concatenate
        format_children(node)
    }

    /// Recursively formats a Token and generates a Document
    fn format_token<'a>(&self, token: &RedLeaf<L>, context: &FormatContext<L, C, S>, source: &'a str) -> FormatResult<Document<'a>> {
        // Apply token formatters
        for formatter in &self.token_formatters {
            if let Some(doc) = formatter(token, context, source)? {
                return Ok(doc);
            }
        }

        // Default logic: output as is
        let text = &source[token.span.start..token.span.end];
        Ok(Document::Text(text.into()))
    }
}

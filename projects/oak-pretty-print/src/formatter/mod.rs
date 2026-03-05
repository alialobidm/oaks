use crate::{CommentProcessor, Document, FormatResult};
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
#[derive(Debug, Clone)]
pub struct FormatContext<L: Language, Config, State> {
    /// Language-specific configuration
    pub config: Arc<Config>,
    /// Comment processor for handling comments during formatting
    pub comment_processor: Arc<CommentProcessor>,
    /// Source code content
    pub source: Option<Arc<str>>,
    /// Current nesting depth
    pub depth: usize,
    /// Path of parent node types
    pub path: Option<Arc<PathNode<L>>>,
    /// Formatting state
    pub state: State,
}

impl<L: Language, Config, State> FormatContext<L, Config, State> {
    /// Creates a new formatting context
    pub fn new(config: Config, state: State) -> Self {
        let config = Arc::new(config);
        let comment_processor = Arc::new(CommentProcessor::new());
        Self { config, comment_processor, source: None, depth: 0, path: None, state }
    }
}

impl<L: Language, Config, State: Clone> FormatContext<L, Config, State> {
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
    pub fn enter_with_state(&self, kind: L::ElementType, state: State) -> Self {
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

/// Formatter trait for language-specific formatting
pub trait Formatter<L: Language + 'static> {
    /// The type of configuration used by this formatter
    type Config;
    /// The type of state used by this formatter
    type State;

    /// Formats an AST node
    fn format(&mut self, root: &RedNode<L>, source: &str) -> FormatResult<FormatOutput>;

    /// Gets the formatter configuration
    fn config(&self) -> &Self::Config;

    /// Gets the formatter configuration mutably
    fn config_mut(&mut self) -> &mut Self::Config;

    /// Gets the formatting state
    fn state(&self) -> &Self::State;

    /// Gets the formatting state mutably
    fn state_mut(&mut self) -> &mut Self::State;
}

/// A generic formatter implementation
pub struct GenericFormatter<L: Language + 'static, Config, State> {
    /// Node formatting functions
    node_formatters: Vec<Box<dyn for<'a> Fn(&RedNode<L>, &FormatContext<L, Config, State>, &'a str, &dyn Fn(&RedNode<L>) -> FormatResult<Document<'a>>) -> FormatResult<Option<Document<'a>>>>>,
    /// Token formatting functions
    token_formatters: Vec<Box<dyn for<'a> Fn(&RedLeaf<L>, &FormatContext<L, Config, State>, &'a str) -> FormatResult<Option<Document<'a>>>>>,
    /// Initial formatting context
    pub context: FormatContext<L, Config, State>,
}

impl<L: Language + 'static, Config: Clone, State: Clone> Formatter<L> for GenericFormatter<L, Config, State> {
    type Config = Config;
    type State = State;

    fn format(&mut self, root: &RedNode<L>, source: &str) -> FormatResult<FormatOutput> {
        self.context.source = Some(Arc::from(source));
        let doc = self.format_node(root, &self.context, source)?;
        let content = doc.render();
        let changed = content != source;
        Ok(FormatOutput::new(content, changed))
    }

    fn config(&self) -> &Self::Config {
        &self.context.config
    }

    fn config_mut(&mut self) -> &mut Self::Config {
        Arc::make_mut(&mut self.context.config)
    }

    fn state(&self) -> &Self::State {
        &self.context.state
    }

    fn state_mut(&mut self) -> &mut Self::State {
        &mut self.context.state
    }
}

impl<L: Language + 'static, Config, State: Clone> GenericFormatter<L, Config, State> {
    /// Creates a new formatter with custom state
    pub fn new_with_state(config: Config, state: State) -> Self {
        let mut formatter = Self { 
            node_formatters: Vec::new(), 
            token_formatters: Vec::new(), 
            context: FormatContext::new(config, state) 
        };

        formatter.with_builtin_formatters()
    }

    /// Adds built-in formatting functions
    fn with_builtin_formatters(mut self) -> Self {
        self.add_node_formatter(Box::new(|node, _ctx, _source, format_children| {
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

        self.add_node_formatter(Box::new(|node, _ctx, _source, format_children| {
            use oak_core::language::{ElementType, UniversalElementRole};
            if ElementType::is_universal(&node.green.kind, UniversalElementRole::Statement) {
                let children_doc = format_children(node)?;
                Ok(Some(Document::concat(vec![children_doc, Document::Line])))
            } else {
                Ok(None)
            }
        }));

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
    pub fn add_node_formatter(&mut self, formatter: Box<dyn for<'a> Fn(&RedNode<L>, &FormatContext<L, Config, State>, &'a str, &dyn Fn(&RedNode<L>) -> FormatResult<Document<'a>>) -> FormatResult<Option<Document<'a>>>>) {
        self.node_formatters.push(formatter);
    }

    /// Adds a token formatting function
    pub fn add_token_formatter(&mut self, formatter: Box<dyn for<'a> Fn(&RedLeaf<L>, &FormatContext<L, Config, State>, &'a str) -> FormatResult<Option<Document<'a>>>>) {
        self.token_formatters.push(formatter);
    }

    /// Recursively formats a node and generates a Document
    fn format_node<'a>(&self, node: &RedNode<L>, context: &FormatContext<L, Config, State>, source: &'a str) -> FormatResult<Document<'a>> {
        let new_context = context.enter(node.green.kind.clone());

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

        for formatter in &self.node_formatters {
            if let Some(doc) = formatter(node, &new_context, source, &format_children)? {
                return Ok(doc);
            }
        }

        format_children(node)
    }

    /// Recursively formats a Token and generates a Document
    fn format_token<'a>(&self, token: &RedLeaf<L>, context: &FormatContext<L, Config, State>, source: &'a str) -> FormatResult<Document<'a>> {
        for formatter in &self.token_formatters {
            if let Some(doc) = formatter(token, context, source)? {
                return Ok(doc);
            }
        }

        let text = &source[token.span.start..token.span.end];
        Ok(Document::Text(text.into()))
    }
}

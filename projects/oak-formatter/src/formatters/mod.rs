use alloc::string::String;
use oak_pretty_print::{FormatConfig, FormatState, WhitespaceProcessor};

/// Language-specific formatter trait
/// 
/// This trait defines the interface for language-specific formatters.
pub trait LanguageFormatter<L: oak_core::language::Language + 'static> {
    /// The type of formatting state used
    type State: FormatState;
    
    /// Formats the given AST node
    /// 
    /// # Parameters
    /// - `root`: The root AST node to format
    /// - `source`: The source code string
    /// 
    /// # Returns
    /// The formatted code string
    fn format(&mut self, root: &oak_core::tree::RedNode<L>, source: &str) -> crate::FormatResult<String>;

    /// Gets the formatter configuration
    fn config(&self) -> &FormatConfig;

    /// Gets the formatter configuration mutably
    fn config_mut(&mut self) -> &mut FormatConfig;

    /// Gets the formatting state
    fn state(&self) -> &Self::State;

    /// Gets the formatting state mutably
    fn state_mut(&mut self) -> &mut Self::State;
}

/// Generic formatter implementation
/// 
/// This struct provides a generic implementation of the `LanguageFormatter` trait.
pub struct GenericFormatter<L: oak_core::language::Language + 'static, S: FormatState> {
    /// Formatting configuration
    config: FormatConfig,
    /// Formatting state
    state: S,
    /// Whitespace processor
    whitespace_processor: WhitespaceProcessor,
    /// Phantom data for the language type
    _phantom: core::marker::PhantomData<L>,
}

impl<L: oak_core::language::Language + 'static, S: FormatState> GenericFormatter<L, S> {
    /// Creates a new generic formatter
    pub fn new(config: FormatConfig, state: S) -> Self {
        let trim_trailing_whitespace = config.trim_trailing_whitespace;
        let preserve_blank_lines = config.preserve_blank_lines;
        let max_blank_lines = config.max_blank_lines;
        
        Self {
            config,
            state,
            whitespace_processor: WhitespaceProcessor::new()
                .with_preserve_blank_lines(preserve_blank_lines)
                .with_max_blank_lines(max_blank_lines)
                .with_trim_trailing_whitespace(trim_trailing_whitespace),
            _phantom: core::marker::PhantomData,
        }
    }
}

impl<L: oak_core::language::Language + 'static, S: FormatState> LanguageFormatter<L> for GenericFormatter<L, S> {
    type State = S;
    
    fn format(&mut self, root: &oak_core::tree::RedNode<L>, source: &str) -> crate::FormatResult<String> {
        // Create a pretty-print formatter
        let mut pp_formatter = oak_pretty_print::Formatter::new(self.config.clone());

        // Format the node
        let output = pp_formatter.format(root, source)?;

        // Process whitespace
        let processed_content = self.whitespace_processor.process(&output.content);

        Ok(processed_content)
    }

    fn config(&self) -> &FormatConfig {
        &self.config
    }

    fn config_mut(&mut self) -> &mut FormatConfig {
        &mut self.config
    }

    fn state(&self) -> &Self::State {
        &self.state
    }

    fn state_mut(&mut self) -> &mut Self::State {
        &mut self.state
    }
}

/// Rust formatter
/// 
/// This struct provides a Rust-specific formatter implementation.
pub struct RustFormatter<L: oak_core::language::Language + 'static> {
    /// Generic formatter
    inner: GenericFormatter<L, crate::RustFormatterState>,
    /// Annotation processor
    annotation_processor: crate::AnnotationProcessor,
}

impl<L: oak_core::language::Language + 'static> RustFormatter<L> {
    /// Creates a new Rust formatter
    pub fn new(config: FormatConfig) -> Self {
        Self {
            inner: GenericFormatter::<L, crate::RustFormatterState>::new(config, crate::RustFormatterState::default()),
            annotation_processor: crate::AnnotationProcessor::new(Box::new(crate::RustAnnotationParser)),
        }
    }
}

impl<L: oak_core::language::Language + 'static> LanguageFormatter<L> for RustFormatter<L> {
    type State = crate::RustFormatterState;
    
    fn format(&mut self, root: &oak_core::tree::RedNode<L>, source: &str) -> crate::FormatResult<String> {
        // Process annotations
        let annotations = self.annotation_processor.process(source);
        self.annotation_processor.apply_annotations(&annotations, self.inner.state_mut());

        // Format using the generic formatter
        self.inner.format(root, source)
    }

    fn config(&self) -> &FormatConfig {
        self.inner.config()
    }

    fn config_mut(&mut self) -> &mut FormatConfig {
        self.inner.config_mut()
    }

    fn state(&self) -> &Self::State {
        self.inner.state()
    }

    fn state_mut(&mut self) -> &mut Self::State {
        self.inner.state_mut()
    }
}

/// TypeScript formatter
/// 
/// This struct provides a TypeScript-specific formatter implementation.
pub struct TypeScriptFormatter<L: oak_core::language::Language + 'static> {
    /// Generic formatter
    inner: GenericFormatter<L, crate::TypeScriptFormatterState>,
    /// Annotation processor
    annotation_processor: crate::AnnotationProcessor,
}

impl<L: oak_core::language::Language + 'static> TypeScriptFormatter<L> {
    /// Creates a new TypeScript formatter
    pub fn new(config: FormatConfig) -> Self {
        Self {
            inner: GenericFormatter::<L, crate::TypeScriptFormatterState>::new(config, crate::TypeScriptFormatterState::default()),
            annotation_processor: crate::AnnotationProcessor::new(Box::new(crate::TypeScriptAnnotationParser)),
        }
    }
}

impl<L: oak_core::language::Language + 'static> LanguageFormatter<L> for TypeScriptFormatter<L> {
    type State = crate::TypeScriptFormatterState;
    
    fn format(&mut self, root: &oak_core::tree::RedNode<L>, source: &str) -> crate::FormatResult<String> {
        // Process annotations
        let annotations = self.annotation_processor.process(source);
        self.annotation_processor.apply_annotations(&annotations, self.inner.state_mut());

        // Format using the generic formatter
        self.inner.format(root, source)
    }

    fn config(&self) -> &FormatConfig {
        self.inner.config()
    }

    fn config_mut(&mut self) -> &mut FormatConfig {
        self.inner.config_mut()
    }

    fn state(&self) -> &Self::State {
        self.inner.state()
    }

    fn state_mut(&mut self) -> &mut Self::State {
        self.inner.state_mut()
    }
}

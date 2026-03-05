use alloc::string::String;

/// Language-specific formatter trait
/// 
/// This trait defines the interface for language-specific formatters.
pub trait LanguageFormatter<L: oak_core::language::Language> {
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
    fn config(&self) -> &crate::FormatConfig;

    /// Gets the formatter configuration mutably
    fn config_mut(&mut self) -> &mut crate::FormatConfig;

    /// Gets the formatting state
    fn state(&self) -> &crate::FormatState;

    /// Gets the formatting state mutably
    fn state_mut(&mut self) -> &mut crate::FormatState;
}

/// Generic formatter implementation
/// 
/// This struct provides a generic implementation of the `LanguageFormatter` trait.
pub struct GenericFormatter<L: oak_core::language::Language> {
    /// Formatting configuration
    config: crate::FormatConfig,
    /// Formatting state
    state: crate::FormatState,
    /// Whitespace processor
    whitespace_processor: crate::WhitespaceProcessor,
}

impl<L: oak_core::language::Language> GenericFormatter<L> {
    /// Creates a new generic formatter
    pub fn new(config: crate::FormatConfig) -> Self {
        Self {
            config,
            state: crate::FormatState::new(),
            whitespace_processor: crate::WhitespaceProcessor::new()
                .with_preserve_blank_lines(config.preserve_blank_lines)
                .with_max_blank_lines(config.max_blank_lines)
                .with_trim_trailing_whitespace(config.trim_trailing_whitespace),
        }
    }
}

impl<L: oak_core::language::Language> LanguageFormatter<L> for GenericFormatter<L> {
    fn format(&mut self, root: &oak_core::tree::RedNode<L>, source: &str) -> crate::FormatResult<String> {
        // Create a pretty-print formatter
        let mut pp_formatter = oak_pretty_print::Formatter::new(self.config.clone());

        // Format the node
        let output = pp_formatter.format(root, source)?;

        // Process whitespace
        let processed_content = self.whitespace_processor.process(&output.content);

        Ok(processed_content)
    }

    fn config(&self) -> &crate::FormatConfig {
        &self.config
    }

    fn config_mut(&mut self) -> &mut crate::FormatConfig {
        &mut self.config
    }

    fn state(&self) -> &crate::FormatState {
        &self.state
    }

    fn state_mut(&mut self) -> &mut crate::FormatState {
        &mut self.state
    }
}

/// Rust formatter
/// 
/// This struct provides a Rust-specific formatter implementation.
pub struct RustFormatter {
    /// Generic formatter
    inner: GenericFormatter<oak_core::language::RustLanguage>,
    /// Annotation processor
    annotation_processor: crate::AnnotationProcessor,
}

impl RustFormatter {
    /// Creates a new Rust formatter
    pub fn new(config: crate::FormatConfig) -> Self {
        Self {
            inner: GenericFormatter::new(config),
            annotation_processor: crate::AnnotationProcessor::new(Box::new(crate::RustAnnotationParser)),
        }
    }
}

impl LanguageFormatter<oak_core::language::RustLanguage> for RustFormatter {
    fn format(&mut self, root: &oak_core::tree::RedNode<oak_core::language::RustLanguage>, source: &str) -> crate::FormatResult<String> {
        // Process annotations
        let annotations = self.annotation_processor.process(source);
        self.annotation_processor.apply_annotations(&annotations, self.inner.state_mut());

        // Format using the generic formatter
        self.inner.format(root, source)
    }

    fn config(&self) -> &crate::FormatConfig {
        self.inner.config()
    }

    fn config_mut(&mut self) -> &mut crate::FormatConfig {
        self.inner.config_mut()
    }

    fn state(&self) -> &crate::FormatState {
        self.inner.state()
    }

    fn state_mut(&mut self) -> &mut crate::FormatState {
        self.inner.state_mut()
    }
}

/// TypeScript formatter
/// 
/// This struct provides a TypeScript-specific formatter implementation.
pub struct TypeScriptFormatter {
    /// Generic formatter
    inner: GenericFormatter<oak_core::language::TypeScriptLanguage>,
    /// Annotation processor
    annotation_processor: crate::AnnotationProcessor,
}

impl TypeScriptFormatter {
    /// Creates a new TypeScript formatter
    pub fn new(config: crate::FormatConfig) -> Self {
        Self {
            inner: GenericFormatter::new(config),
            annotation_processor: crate::AnnotationProcessor::new(Box::new(crate::TypeScriptAnnotationParser)),
        }
    }
}

impl LanguageFormatter<oak_core::language::TypeScriptLanguage> for TypeScriptFormatter {
    fn format(&mut self, root: &oak_core::tree::RedNode<oak_core::language::TypeScriptLanguage>, source: &str) -> crate::FormatResult<String> {
        // Process annotations
        let annotations = self.annotation_processor.process(source);
        self.annotation_processor.apply_annotations(&annotations, self.inner.state_mut());

        // Format using the generic formatter
        self.inner.format(root, source)
    }

    fn config(&self) -> &crate::FormatConfig {
        self.inner.config()
    }

    fn config_mut(&mut self) -> &mut crate::FormatConfig {
        self.inner.config_mut()
    }

    fn state(&self) -> &crate::FormatState {
        self.inner.state()
    }

    fn state_mut(&mut self) -> &mut crate::FormatState {
        self.inner.state_mut()
    }
}

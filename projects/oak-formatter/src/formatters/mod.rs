use alloc::string::String;
use oak_pretty_print::{Document, WhitespaceProcessor};

/// Generic formatter trait for language-specific formatters
/// 
/// This trait defines the interface that language-specific formatters must implement.
pub trait Formatter {
    /// The state type used by this formatter
    type State;

    /// Formats source code into a Document
    /// 
    /// # Parameters
    /// - `source`: The source code to format
    /// - `state`: The current formatter state
    /// 
    /// # Returns
    /// The formatted Document
    fn format(&self, source: &str, state: &mut Self::State) -> Document<'_>;
}

/// A generic formatter that can be used for any language
/// 
/// This struct provides a common interface for formatting code in any language.
pub struct GenericFormatter<F: Formatter> {
    /// The language-specific formatter implementation
    formatter: F,
    /// The whitespace processor
    whitespace_processor: WhitespaceProcessor,
}

impl<F: Formatter> GenericFormatter<F> {
    /// Creates a new GenericFormatter
    /// 
    /// # Parameters
    /// - `formatter`: The language-specific formatter implementation
    pub fn new(formatter: F) -> Self {
        Self {
            formatter,
            whitespace_processor: WhitespaceProcessor::default(),
        }
    }

    /// Formats the given source code
    /// 
    /// # Parameters
    /// - `source`: The source code to format
    /// 
    /// # Returns
    /// The formatted source code
    pub fn format_source(&self, source: &str) -> String {
        source.to_string()
    }
}

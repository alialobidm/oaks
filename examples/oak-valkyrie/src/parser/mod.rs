pub use element_type::ValkyrieElementType;

/// Valkyrie parser.
pub struct ValkyrieParser<'config> {
    config: &'config ValkyrieLanguage,
}

impl<'config> ValkyrieParser<'config> {
    /// Create a new Valkyrie parser.
    pub fn new(config: &'config ValkyrieLanguage) -> Self {
        Self { config }
    }
}

/// Element type definitions for the Valkyrie parser.
///
/// This module provides [`ValkyrieElementType`] which defines all element
/// categories produced by the parser, representing the structural nodes
/// in the parsed abstract syntax tree.
pub mod element_type;

pub use element_type::ValkyrieElementType;

/// Valkyrie parser.
pub struct ValkyrieParser;

impl ValkyrieParser {
    /// Create a new Valkyrie parser.
    pub fn new() -> Self {
        Self
    }
}

/// Element type definitions for the Valkyrie parser.
///
/// This module provides [`ValkyrieElementType`] which defines all element
/// categories produced by the parser, representing the structural nodes
/// in the parsed abstract syntax tree.
pub mod element_type;

pub use token_type::ValkyrieTokenType;

/// Valkyrie lexer.
pub struct ValkyrieLexer;

impl ValkyrieLexer {
    /// Create a new Valkyrie lexer.
    pub fn new() -> Self {
        Self
    }
}

/// Token type definitions for the Valkyrie lexer.
///
/// This module provides [`ValkyrieTokenType`] which defines all token categories
/// recognized by the lexer, including EOF, whitespace, comments, identifiers,
/// and literals (strings and numbers).
pub mod token_type;

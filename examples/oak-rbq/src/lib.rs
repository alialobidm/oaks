#![doc = include_str!("readme.md")]
#![feature(new_range_api)]
#![warn(missing_docs)]
#![doc(html_logo_url = "https://raw.githubusercontent.com/ygg-lang/oaks/refs/heads/dev/documents/logo.svg")]
#![doc(html_favicon_url = "https://raw.githubusercontent.com/ygg-lang/oaks/refs/heads/dev/documents/logo.svg")]

/// AST module containing node definitions for the RBQ language.
pub mod ast;
/// Builder module for constructing RBQ trees.
pub mod builder;

/// Language configuration and syntax kind definitions.
pub mod language;
/// Lexer implementation for RBQ.
pub mod lexer;
/// LSP-related functionality (hover, completion, highlighting).
#[cfg(feature = "lsp")]
pub mod lsp;
/// MCP (Model Context Protocol) integration for RBQ.
#[cfg(feature = "mcp")]
pub mod mcp;

/// Parser implementation for RBQ.
pub mod parser;

pub use crate::{
    ast::RbqRoot,
    builder::RbqBuilder,
    language::RbqLanguage,
    lexer::{RbqLexer, token_type::RbqTokenType},
    parser::{RbqParser, element_type::RbqElementType},
};

/// Alias for RbqTokenType to support tests and common usage
pub type RbqSyntaxKind = RbqTokenType;

/// Highlighter implementation.
#[cfg(feature = "oak-highlight")]
pub use crate::lsp::highlighter::RbqHighlighter;

/// Formatter implementation.
#[cfg(feature = "lsp")]
pub use crate::lsp::formatter::RbqFormatter;

/// LSP implementation.
#[cfg(feature = "lsp")]
pub use crate::lsp::RbqLanguageService;

/// MCP implementation.
#[cfg(feature = "mcp")]
pub use crate::mcp::serve_rbq_mcp;

/// Parses a string into an RBQ AST.
pub fn parse(input: &str) -> Result<RbqRoot, oak_core::OakError> {
    use oak_core::{ParseCache, SourceFile, TextEdit};
    
    // Create language configuration
    let language = RbqLanguage::new();
    
    // Create parser
    let parser = RbqParser::new(&language);
    
    // Create source file
    let source = SourceFile::new("", input);
    
    // Create empty parse cache
    let mut cache = oak_core::NoParseCache::default();
    
    // Parse the input
    let output = parser.parse(&source, &[], &mut cache);
    
    // Convert the parse tree to an AST
    let root_node = output.tree.root();
    let ast = RbqRoot::lower(root_node, input);
    
    Ok(ast)
}

#![doc(html_logo_url = "https://raw.githubusercontent.com/ygg-lang/oaks/refs/heads/dev/documents/logo.svg")]
#![doc(html_favicon_url = "https://raw.githubusercontent.com/ygg-lang/oaks/refs/heads/dev/documents/logo.svg")]
#![warn(missing_docs)]
//! High-level formatting library for the Oak language framework.
//! 
//! This crate provides high-level formatting logic, including configuration management,
//! annotation processing, and language-specific formatting rules.

pub extern crate alloc;
extern crate self as oak_formatter;

// Public modules
/// Formatting configuration
pub mod config;
/// Annotation handling and processing
pub mod annotation;
/// Language-specific formatters
pub mod formatters;
/// Error types for formatting
pub mod errors;

// Re-export commonly used types
pub use crate::{
    config::{RustFormatterConfig, RustFormatterState, TypeScriptFormatterConfig, TypeScriptFormatterState},
    errors::FormatResult,
    annotation::{AnnotationParser, AnnotationProcessor, FormatAnnotation, RustAnnotationParser, TypeScriptAnnotationParser},
};

pub use oak_pretty_print::{FormatterConfig, IndentStyle, LineEnding, WhitespaceProcessor, Document};
pub use oak_core::language::Language;

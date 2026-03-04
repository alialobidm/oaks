//! Error types for source map operations.

use thiserror::Error;

/// Result type alias for source map operations.
pub type Result<T> = std::result::Result<T, SourceMapError>;

/// Error type for source map operations.
#[derive(Debug, Error)]
pub enum SourceMapError {
    /// Invalid source map version.
    #[error("Invalid source map version: expected 3, got {0}")]
    InvalidVersion(u8),

    /// Missing required field.
    #[error("Missing required field: {0}")]
    MissingField(&'static str),

    /// Invalid VLQ encoding.
    #[error("Invalid VLQ encoding at position {position}: {message}")]
    InvalidVlq {
        /// Position in the mappings string.
        position: usize,
        /// Error message.
        message: String,
    },

    /// Invalid mapping.
    #[error("Invalid mapping at line {line}, column {column}: {message}")]
    InvalidMapping {
        /// Line number.
        line: u32,
        /// Column number.
        column: u32,
        /// Error message.
        message: String,
    },

    /// JSON parsing error.
    #[error("JSON parsing error: {0}")]
    JsonError(#[from] serde_json::Error),

    /// IO error.
    #[error("IO error: {0}")]
    IoError(#[from] std::io::Error),

    /// Index out of bounds.
    #[error("Index out of bounds: {index} >= {length}")]
    IndexOutOfBounds {
        /// The index that was out of bounds.
        index: usize,
        /// The length of the collection.
        length: usize,
    },

    /// Invalid source index.
    #[error("Invalid source index: {0}")]
    InvalidSourceIndex(usize),

    /// Invalid name index.
    #[error("Invalid name index: {0}")]
    InvalidNameIndex(usize),

    /// Source map composition error.
    #[error("Source map composition error: {0}")]
    CompositionError(String),
}

impl SourceMapError {
    /// Creates a new invalid VLQ error.
    pub fn invalid_vlq(position: usize, message: impl Into<String>) -> Self {
        SourceMapError::InvalidVlq { position, message: message.into() }
    }

    /// Creates a new invalid mapping error.
    pub fn invalid_mapping(line: u32, column: u32, message: impl Into<String>) -> Self {
        SourceMapError::InvalidMapping { line, column, message: message.into() }
    }

    /// Creates a new index out of bounds error.
    pub fn index_out_of_bounds(index: usize, length: usize) -> Self {
        SourceMapError::IndexOutOfBounds { index, length }
    }
}

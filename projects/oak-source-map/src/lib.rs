//! # oak-source-map
//!
//! A high-performance Source Map v3 implementation for Rust.
//!
//! ## Features
//!
//! - Full Source Map v3 specification support
//! - VLQ Base64 encoding/decoding
//! - Zero-copy parsing where possible
//! - Builder pattern for incremental construction
//! - Source map composition and manipulation
//!
//! ## Example
//!
//! ```rust
//! use oak_source_map::{SourceMap, SourceMapBuilder};
//!
//! // Parse an existing source map
//! let json = r#"{"version":3,"sources":["foo.js"],"names":[],"mappings":"AAAA"}"#;
//! let sm = SourceMap::parse(json)?;
//!
//! // Build a new source map
//! let mut builder = SourceMapBuilder::new();
//! builder.add_source("input.ts");
//! builder.add_mapping(0, 0, Some(0), Some(0), Some(0), None);
//! let output = builder.build();
//!
//! # Ok::<(), oak_source_map::SourceMapError>(())
//! ```

#![warn(missing_docs)]
#![forbid(unsafe_code)]

mod builder;
mod composer;
mod decoder;
mod error;
mod mapping;
mod source_map;
mod vlq;

pub use builder::SourceMapBuilder;
pub use composer::SourceMapComposer;
pub use decoder::SourceMapDecoder;
pub use error::{Result, SourceMapError};
pub use mapping::{BoundedMapping, Mapping, Segment};
pub use source_map::{SourceMap, SourceMapMetadata};
pub use vlq::{vlq_decode, vlq_encode};

pub use source_map::SourceMapInput;

/// Source Map version (always 3).
pub const SOURCE_MAP_VERSION: u8 = 3;

/// The default source root.
pub const DEFAULT_SOURCE_ROOT: &str = "";

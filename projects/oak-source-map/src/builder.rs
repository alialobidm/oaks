//! Source Map Builder for incremental construction.

use crate::{Mapping, SourceMap, SOURCE_MAP_VERSION};
use crate::vlq::vlq_encode;

/// Builder for incrementally constructing source maps.
///
/// # Example
///
/// ```
/// use oak_source_map::SourceMapBuilder;
///
/// let mut builder = SourceMapBuilder::new();
///
/// // Add a source file
/// let source_idx = builder.add_source("input.ts");
///
/// // Add mappings
/// builder.add_mapping(0, 0, Some(source_idx), Some(0), Some(0), None);
/// builder.add_mapping(0, 10, Some(source_idx), Some(0), Some(10), None);
///
/// // Build the final source map
/// let source_map = builder.build();
/// ```
#[derive(Debug, Default)]
pub struct SourceMapBuilder {
    sources: Vec<String>,
    sources_content: Vec<Option<String>>,
    names: Vec<String>,
    mappings: Vec<Vec<Mapping>>,
    file: Option<String>,
    source_root: Option<String>,
    last_generated_column: u32,
    last_source_index: u32,
    last_original_line: u32,
    last_original_column: u32,
    last_name_index: u32,
}

impl SourceMapBuilder {
    /// Creates a new source map builder.
    pub fn new() -> Self {
        Self::default()
    }

    /// Sets the output file name.
    pub fn file(mut self, file: impl Into<String>) -> Self {
        self.file = Some(file.into());
        self
    }

    /// Sets the source root.
    pub fn source_root(mut self, root: impl Into<String>) -> Self {
        self.source_root = Some(root.into());
        self
    }

    /// Adds a source file and returns its index.
    pub fn add_source(&mut self, source: impl Into<String>) -> u32 {
        let source_str = source.into();
        if let Some(idx) = self.sources.iter().position(|s| s == &source_str) {
            idx as u32
        } else {
            self.sources.push(source_str);
            self.sources_content.push(None);
            (self.sources.len() - 1) as u32
        }
    }

    /// Sets the content for a source file.
    pub fn set_source_content(&mut self, index: u32, content: impl Into<String>) {
        let idx = index as usize;
        if idx < self.sources_content.len() {
            self.sources_content[idx] = Some(content.into());
        }
    }

    /// Adds a name and returns its index.
    pub fn add_name(&mut self, name: impl Into<String>) -> u32 {
        let name_str = name.into();
        if let Some(idx) = self.names.iter().position(|n| n == &name_str) {
            idx as u32
        } else {
            self.names.push(name_str);
            (self.names.len() - 1) as u32
        }
    }

    /// Adds a mapping.
    ///
    /// All line and column values are 0-indexed.
    pub fn add_mapping(
        &mut self,
        generated_line: u32,
        generated_column: u32,
        source_index: Option<u32>,
        original_line: Option<u32>,
        original_column: Option<u32>,
        name_index: Option<u32>,
    ) {
        while self.mappings.len() <= generated_line as usize {
            self.mappings.push(Vec::new());
        }

        self.mappings[generated_line as usize].push(Mapping {
            generated_line,
            generated_column,
            source_index,
            original_line,
            original_column,
            name_index,
        });
    }

    /// Adds a segment (more convenient API for simple cases).
    pub fn add_segment(
        &mut self,
        generated_line: u32,
        generated_column: u32,
        source: Option<&str>,
        original_line: Option<u32>,
        original_column: Option<u32>,
        name: Option<&str>,
    ) {
        let source_index = source.map(|s| self.add_source(s));
        let name_index = name.map(|n| self.add_name(n));

        self.add_mapping(
            generated_line,
            generated_column,
            source_index,
            original_line,
            original_column,
            name_index,
        );
    }

    /// Builds the final source map.
    pub fn build(self) -> SourceMap {
        let mappings = self.encode_mappings();

        SourceMap {
            version: SOURCE_MAP_VERSION,
            sources: self.sources,
            sources_content: self.sources_content,
            names: self.names,
            mappings,
            file: self.file,
            source_root: self.source_root,
            sections: Vec::new(),
        }
    }

    fn encode_mappings(&self) -> String {
        let mut result = String::new();

        for (line_idx, line_mappings) in self.mappings.iter().enumerate() {
            if line_idx > 0 {
                result.push(';');
            }

            let mut last_col = 0u32;
            let mut last_source = 0u32;
            let mut last_orig_line = 0u32;
            let mut last_orig_col = 0u32;
            let mut last_name = 0u32;

            for (seg_idx, mapping) in line_mappings.iter().enumerate() {
                if seg_idx > 0 {
                    result.push(',');
                }

                result.push_str(&vlq_encode(mapping.generated_column as i32 - last_col as i32));
                last_col = mapping.generated_column;

                if let Some(si) = mapping.source_index {
                    result.push_str(&vlq_encode(si as i32 - last_source as i32));
                    last_source = si;

                    if let Some(ol) = mapping.original_line {
                        result.push_str(&vlq_encode(ol as i32 - last_orig_line as i32));
                        last_orig_line = ol;
                    }

                    if let Some(oc) = mapping.original_column {
                        result.push_str(&vlq_encode(oc as i32 - last_orig_col as i32));
                        last_orig_col = oc;
                    }

                    if let Some(ni) = mapping.name_index {
                        result.push_str(&vlq_encode(ni as i32 - last_name as i32));
                        last_name = ni;
                    }
                }
            }
        }

        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_builder_basic() {
        let mut builder = SourceMapBuilder::new();
        let source_idx = builder.add_source("test.ts");
        builder.add_mapping(0, 0, Some(source_idx), Some(0), Some(0), None);

        let sm = builder.build();
        assert_eq!(sm.version, 3);
        assert_eq!(sm.sources.len(), 1);
        assert!(!sm.mappings.is_empty());
    }

    #[test]
    fn test_builder_multiple_mappings() {
        let mut builder = SourceMapBuilder::new();
        let source_idx = builder.add_source("test.ts");

        builder.add_mapping(0, 0, Some(source_idx), Some(0), Some(0), None);
        builder.add_mapping(0, 10, Some(source_idx), Some(0), Some(10), None);
        builder.add_mapping(1, 0, Some(source_idx), Some(1), Some(0), None);

        let sm = builder.build();
        let mappings = sm.parse_mappings().unwrap();
        assert_eq!(mappings.len(), 3);
    }

    #[test]
    fn test_builder_with_names() {
        let mut builder = SourceMapBuilder::new();
        let source_idx = builder.add_source("test.ts");
        let name_idx = builder.add_name("foo");

        builder.add_mapping(0, 0, Some(source_idx), Some(0), Some(0), Some(name_idx));

        let sm = builder.build();
        assert_eq!(sm.names.len(), 1);
        assert_eq!(sm.names[0], "foo");
    }

    #[test]
    fn test_builder_source_content() {
        let mut builder = SourceMapBuilder::new();
        let source_idx = builder.add_source("test.ts");
        builder.set_source_content(source_idx, "const x = 1;");

        let sm = builder.build();
        assert_eq!(sm.sources_content.len(), 1);
        assert_eq!(sm.sources_content[0], Some("const x = 1;".to_string()));
    }
}

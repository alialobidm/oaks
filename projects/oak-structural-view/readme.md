# 🚀 oak-structural-view

[![Crates.io](https://img.shields.io/crates/v/oak-structural-view.svg)](https://crates.io/crates/oak-structural-view)
[![Documentation](https://docs.rs/oak-structural-view/badge.svg)](https://docs.rs/oak-structural-view)

**Document Structure View for Oak Languages** — Hierarchical representation of document structure for outline views and breadcrumb navigation.

## 🎯 Project Vision

Structure views help developers navigate large files by showing a hierarchical outline of classes, functions, and other definitions. `oak-structural-view` provides the infrastructure for building outline views and breadcrumb navigation, similar to IntelliJ's Structure View and VS Code's Outline panel.

## ✨ Core Features

- **📊 Structure Provider Trait**: The `StructureProvider` trait for implementing language-specific structure extraction.
- **🌳 Hierarchical Items**: `StructureItem` supports nested children for accurate representation.
- **🎯 Selection Ranges**: Separate ranges for the full element and the clickable identifier.
- **📝 Detail Support**: Optional detail text for showing signatures or type information.
- **🔄 Serde Support**: Optional serialization for LSP and IDE integration.

## 🏗️ Architecture

### `StructureItem` Struct

Represents an item in the document structure:

```rust
pub struct StructureItem {
    pub name: String,                    // Item name
    pub detail: Option<String>,          // Additional detail (signature, type)
    pub role: UniversalElementRole,      // Function, class, variable, etc.
    pub range: Range<usize>,             // Full element range
    pub selection_range: Range<usize>,   // Identifier range for selection
    pub deprecated: bool,                // Deprecation status
    pub children: Vec<StructureItem>,    // Nested items
}
```

### `StructureProvider` Trait

Implement to provide document structure:

```rust
use oak_structural_view::{StructureProvider, StructureItem};
use oak_core::{Language, tree::RedNode};

struct MyStructureProvider;

impl<L: Language> StructureProvider<L> for MyStructureProvider {
    fn structure(&self, root: &RedNode<L>) -> Vec<StructureItem> {
        // Return the hierarchical structure of the document
        vec![]
    }
}
```

### IDE Integration

The structure view maps to:
- **IntelliJ**: Structure View tool window
- **VS Code**: Outline panel and breadcrumbs
- **LSP**: `textDocument/documentSymbol` request

## 🔗 Integration

`oak-structural-view` is used by:
- `oak-lsp` for `textDocument/documentSymbol` support
- IDE extensions for outline views
- Code navigation tools for quick symbol access

## 🤝 Contributing

Contributions are welcome! Please feel free to submit a Pull Request. For major changes, please open an issue first to discuss what you would like to change.

# 🚀 oak-folding

[![Crates.io](https://img.shields.io/crates/v/oak-folding.svg)](https://crates.io/crates/oak-folding)
[![Documentation](https://docs.rs/oak-folding/badge.svg)](https://docs.rs/oak-folding)

**Code Folding Support for Oak Languages** — Identify collapsible regions in source code for IDE outline views and editor folding.

## 🎯 Project Vision

Code folding helps developers manage large files by collapsing sections of code. `oak-folding` provides a trait-based interface for identifying foldable regions like functions, classes, comments, and import sections in any Oak-supported language.

## ✨ Core Features

- **📁 Folding Provider Trait**: The `FoldingProvider` trait for implementing language-specific folding logic.
- **🎯 Folding Range Types**: Built-in support for comments, imports, and custom regions.
- **📍 Precise Ranges**: Byte-offset based ranges for accurate folding in editors.
- **🔄 Serde Support**: Optional serialization for LSP integration.

## 🏗️ Architecture

### `FoldingRange` Struct

Represents a foldable region:

```rust
pub struct FoldingRange {
    pub range: Range<usize>,        // Byte range to fold
    pub kind: Option<FoldingRangeKind>, // Type of folding
}
```

### `FoldingRangeKind` Enum

Predefined folding types:

```rust
pub enum FoldingRangeKind {
    Comment,  // Comment blocks
    Imports,  // Import sections
    Region,   // Custom defined regions
}
```

### `FoldingProvider` Trait

Implement to provide folding ranges:

```rust
use oak_folding::{FoldingProvider, FoldingRange, FoldingRangeKind};
use oak_core::{Language, tree::RedNode};

struct MyFoldingProvider;

impl<L: Language> FoldingProvider<L> for MyFoldingProvider {
    fn folding_ranges(&self, root: &RedNode<L>) -> Vec<FoldingRange> {
        // Return all foldable regions in the document
        vec![]
    }
}
```

## 🔗 Integration

`oak-folding` is used by:
- `oak-lsp` for LSP `textDocument/foldingRange` support
- IDE extensions for code outline views
- Code formatters and analysis tools

## 🤝 Contributing

Contributions are welcome! Please feel free to submit a Pull Request. For major changes, please open an issue first to discuss what you would like to change.

# 🚀 oak-hover

[![Crates.io](https://img.shields.io/crates/v/oak-hover.svg)](https://crates.io/crates/oak-hover)
[![Documentation](https://docs.rs/oak-hover/badge.svg)](https://docs.rs/oak-hover)

**Hover Information Provider for the Oak Ecosystem** — A lightweight, trait-based library for providing hover information (documentation, type info, etc.) in editors and IDEs.

## 🎯 Project Vision

Hover information is essential for modern developer experience — it provides instant feedback about symbols, types, and documentation without leaving the editor. `oak-hover` provides a simple, unified interface for implementing hover support in any Oak-based language parser, enabling seamless integration with LSP clients and other editor tooling.

## ✨ Core Features

- **🎯 Trait-Based Design**: The `HoverProvider` trait provides a clean interface for implementing language-specific hover logic.
- **📝 Markdown Support**: Hover content is returned as markdown strings, enabling rich formatting for documentation.
- **📍 Range-Aware**: Optional range information allows editors to highlight the relevant span when showing hover.
- **🔄 Serde Integration**: Optional serde support for easy serialization in LSP implementations.
- **🧩 Zero Dependencies**: Minimal dependencies with optional serde support.

## 🏗️ Architecture

The crate provides two main types:

### `Hover` Struct
Represents hover information returned to the editor:
- `contents`: Markdown-formatted hover content
- `range`: Optional source range for the hover target

### `HoverProvider` Trait
Implement this trait to provide hover information for your language:

```rust
use oak_hover::{Hover, HoverProvider};
use oak_core::{Language, tree::RedNode};
use core::range::Range;

struct MyHoverProvider;

impl<L: Language> HoverProvider<L> for MyHoverProvider {
    fn hover(&self, root: &RedNode<L>, range: Range<usize>) -> Option<Hover> {
        // Find the node at the given range and return hover info
        Some(Hover {
            contents: "My hover content".to_string(),
            range: Some(range),
        })
    }
}
```

## 🔗 Integration

`oak-hover` is designed to work seamlessly with:
- `oak-lsp` for full Language Server Protocol support
- `oak-core` for syntax tree traversal
- Any editor that supports hover information display

## 🤝 Contributing

Contributions are welcome! Please feel free to submit a Pull Request. For major changes, please open an issue first to discuss what you would like to change.

# 🚀 oak-navigation

[![Crates.io](https://img.shields.io/crates/v/oak-navigation.svg)](https://crates.io/crates/oak-navigation)
[![Documentation](https://docs.rs/oak-navigation/badge.svg)](https://docs.rs/oak-navigation)

**Code Navigation Support for Oak Languages** — Traits and utilities for implementing "Go to Definition" and "Find All References" features.

## 🎯 Project Vision

Code navigation is fundamental to developer productivity. `oak-navigation` provides the building blocks for implementing navigation features in any Oak-based language tool, enabling developers to quickly jump between definitions and references across their codebase.

## ✨ Core Features

- **🎯 Definition Provider**: The `DefinitionProvider` trait for implementing "Go to Definition" functionality.
- **🔍 References Provider**: The `ReferencesProvider` trait for implementing "Find All References" functionality.
- **📍 Location Type**: A unified `Location` type for representing source positions across files.
- **🔎 Simple Reference Finder**: A built-in `SimpleReferenceFinder` for basic name-based reference search.
- **🔄 Serde Support**: Optional serialization for LSP integration.

## 🏗️ Architecture

### `Location` Struct

Represents a position in source code:

```rust
pub struct Location {
    pub uri: Arc<str>,      // File URI
    pub range: Range<usize>, // Byte range
}
```

### `DefinitionProvider` Trait

Implement to support "Go to Definition":

```rust
use oak_navigation::{DefinitionProvider, Location};
use oak_core::{Language, tree::RedNode};

struct MyDefinitionProvider;

impl<L: Language> DefinitionProvider<L> for MyDefinitionProvider {
    fn definition(&self, root: &RedNode<L>, offset: usize) -> Vec<Location> {
        // Find the definition of the symbol at `offset`
        vec![]
    }
}
```

### `ReferencesProvider` Trait

Implement to support "Find All References":

```rust
use oak_navigation::{ReferencesProvider, Location};

impl<L: Language> ReferencesProvider<L> for MyReferencesProvider {
    fn references(&self, root: &RedNode<L>, offset: usize, include_declaration: bool) -> Vec<Location> {
        // Find all references to the symbol at `offset`
        vec![]
    }
}
```

### `SimpleReferenceFinder`

A helper for basic reference finding:

```rust
use oak_navigation::SimpleReferenceFinder;

let references = SimpleReferenceFinder::find(&root, "my_function", source_text, uri);
```

## 🔗 Integration

`oak-navigation` is used by:
- `oak-lsp` for LSP navigation features
- `oak-mcp` for AI-assisted code navigation
- Custom IDE extensions and tools

## 🤝 Contributing

Contributions are welcome! Please feel free to submit a Pull Request. For major changes, please open an issue first to discuss what you would like to change.

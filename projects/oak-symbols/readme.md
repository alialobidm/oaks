# 🚀 oak-symbols

[![Crates.io](https://img.shields.io/crates/v/oak-symbols.svg)](https://crates.io/crates/oak-symbols)
[![Documentation](https://docs.rs/oak-symbols/badge.svg)](https://docs.rs/oak-symbols)

**Symbol Management for Oak Languages** — Extract and manage symbol information for document outlines, workspace search, and navigation.

## 🎯 Project Vision

Symbols are the building blocks of code — functions, classes, variables, and more. `oak-symbols` provides a unified interface for extracting symbol information from syntax trees, enabling features like document outlines, workspace symbol search, and breadcrumb navigation.

## ✨ Core Features

- **📊 Symbol Provider Trait**: The `SymbolProvider` trait for implementing symbol extraction.
- **🔍 Symbol Information**: Rich `SymbolInformation` type with name, role, location, and container.
- **🌐 Universal Provider**: Built-in `UniversalSymbolProvider` that works with any Oak language.
- **📂 Document & Workspace**: Support for both document-level and workspace-wide symbol queries.
- **🔄 Serde Support**: Optional serialization for LSP integration.

## 🏗️ Architecture

### `SymbolInformation` Struct

Represents information about a symbol:

```rust
pub struct SymbolInformation {
    pub name: String,                    // Symbol name
    pub role: UniversalElementRole,      // Function, class, variable, etc.
    pub uri: Arc<str>,                   // File URI
    pub range: Range<usize>,             // Full symbol range
    pub container_name: Option<String>,  // Parent symbol name
}
```

### `SymbolProvider` Trait

Implement to provide symbol information:

```rust
use oak_symbols::{SymbolProvider, SymbolInformation};
use oak_core::{Language, source::Source, tree::RedNode};

struct MySymbolProvider;

impl<L: Language> SymbolProvider<L> for MySymbolProvider {
    fn document_symbols<S: Source + ?Sized>(
        &self,
        uri: &str,
        root: &RedNode<L>,
        source: &S
    ) -> Vec<SymbolInformation> {
        // Return all symbols defined in the document
        vec![]
    }

    fn workspace_symbols(&self, query: &str) -> Vec<SymbolInformation> {
        // Return symbols matching the query across the workspace
        vec![]
    }
}
```

### `UniversalSymbolProvider`

A ready-to-use provider that works with any language:

```rust
use oak_symbols::UniversalSymbolProvider;

let provider = UniversalSymbolProvider::new();
let symbols = provider.document_symbols(uri, &root, &source);
```

## 🔗 Integration

`oak-symbols` is used by:
- `oak-lsp` for `textDocument/documentSymbol` and `workspace/symbol`
- `oak-structural-view` for outline views
- `oak-mcp` for AI-assisted symbol discovery

## 🤝 Contributing

Contributions are welcome! Please feel free to submit a Pull Request. For major changes, please open an issue first to discuss what you would like to change.

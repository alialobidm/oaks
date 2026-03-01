# 🚀 oaks

[![Crates.io](https://img.shields.io/crates/v/oaks.svg)](https://crates.io/crates/oaks)
[![Documentation](https://docs.rs/oaks/badge.svg)](https://docs.rs/oaks)

**The Unified Oak Library** — A single crate that re-exports all Oak components for convenient access.

## 🎯 Project Vision

While Oak is designed as a modular ecosystem, sometimes you need everything in one place. `oaks` is the unified library that re-exports all core Oak crates, providing a single dependency for projects that want the complete Oak experience.

## ✨ Core Features

- **📦 Single Dependency**: One crate to rule them all.
- **🔄 Re-exports**: Direct access to all Oak components.
- **🎯 Convenience**: No need to manage multiple crate versions.
- **🧩 Feature Flags**: Optional features for smaller compile times.

## 🏗️ Architecture

### Re-exported Crates

| Crate | Purpose |
|-------|---------|
| `oak-core` | Core parsing infrastructure |
| `oak-lsp` | Language Server Protocol support |
| `oak-vfs` | Virtual File System |
| `oak-hover` | Hover information providers |
| `oak-navigation` | Code navigation features |
| `oak-folding` | Code folding support |
| `oak-symbols` | Symbol management |
| `oak-semantic-tokens` | Semantic highlighting |
| `oak-structural-view` | Document structure views |

### Usage

```rust
// Instead of multiple dependencies:
// use oak_core::{...};
// use oak_lsp::{...};
// use oak_vfs::{...};

// Use the unified crate:
use oaks::{
    // Core
    SourceText, Parser, GreenNode, RedNode,
    // LSP
    LanguageService, LspServer,
    // VFS
    Vfs, MemoryVfs,
    // Features
    Hover, HoverProvider,
    DefinitionProvider, ReferencesProvider,
    FoldingProvider, FoldingRange,
    SymbolProvider, SymbolInformation,
    SemanticTokensProvider, SemanticToken,
    StructureProvider, StructureItem,
};
```

## 🔗 Integration

Use `oaks` when you need:
- The complete Oak toolkit
- Simplified dependency management
- Quick prototyping with all features available

Use individual crates when you need:
- Minimal dependencies
- Faster compile times
- Specific functionality only

## 🤝 Contributing

Contributions are welcome! Please feel free to submit a Pull Request. For major changes, please open an issue first to discuss what you would like to change.

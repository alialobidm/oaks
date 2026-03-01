# 🚀 oak-lsp

[![Crates.io](https://img.shields.io/crates/v/oak-lsp.svg)](https://crates.io/crates/oak-lsp)
[![Documentation](https://docs.rs/oak-lsp/badge.svg)](https://docs.rs/oak-lsp)

**Language Server Protocol Implementation for Oak** — A complete LSP server framework for building language-aware editors and IDEs.

## 🎯 Project Vision

The Language Server Protocol (LSP) has become the standard for language tooling integration. `oak-lsp` provides a complete, production-ready LSP server implementation that works with any Oak language parser, enabling features like autocomplete, go-to-definition, and diagnostics in any LSP-compatible editor.

## ✨ Core Features

- **📡 Full LSP Support**: Implements core LSP requests and notifications for language features.
- **🔧 Language Service Trait**: The `LanguageService` trait provides a unified interface for language-specific features.
- **📂 Workspace Management**: `WorkspaceManager` handles multi-file projects with change tracking.
- **🔄 Incremental Updates**: Efficient handling of text document changes with incremental parsing.
- **🛡️ Error Recovery**: Robust error handling for malformed requests and parser errors.

## 🏗️ Architecture

### Core Modules

| Module | Purpose |
|--------|---------|
| `handlers` | LSP request/notification handlers |
| `server` | LSP server implementation with JSON-RPC |
| `service` | `LanguageService` trait for language features |
| `types` | LSP-specific type definitions |
| `workspace` | File and project management |

### `LanguageService` Trait

Implement this trait to provide language-specific features:

```rust
use oak_lsp::LanguageService;
use oak_core::Language;

struct MyLanguageService;

impl<L: Language> LanguageService<L> for MyLanguageService {
    // Implement language features: completion, hover, definition, etc.
}
```

### `LspServer`

The main server that handles client communication:

```rust
use oak_lsp::LspServer;

let server = LspServer::new(MyLanguageService::new());
server.run().await?;
```

### Supported LSP Features

- `textDocument/completion` — Code completion
- `textDocument/hover` — Hover information
- `textDocument/definition` — Go to definition
- `textDocument/references` — Find all references
- `textDocument/documentSymbol` — Document outline
- `textDocument/foldingRange` — Code folding
- `textDocument/semanticTokens` — Semantic highlighting
- `textDocument/diagnostic` — Diagnostics

## 🔗 Integration

`oak-lsp` integrates with:
- `oak-vfs` for file system access
- `oak-hover`, `oak-navigation`, `oak-symbols` for language features
- Any LSP-compatible editor (VS Code, Neovim, Emacs, etc.)

## 🤝 Contributing

Contributions are welcome! Please feel free to submit a Pull Request. For major changes, please open an issue first to discuss what you would like to change.

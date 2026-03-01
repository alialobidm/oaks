# 🚀 oak-semantic-tokens

[![Crates.io](https://img.shields.io/crates/v/oak-semantic-tokens.svg)](https://crates.io/crates/oak-semantic-tokens)
[![Documentation](https://docs.rs/oak-semantic-tokens/badge.svg)](https://docs.rs/oak-semantic-tokens)

**Semantic Tokens for Oak Languages** — LSP-compatible semantic syntax highlighting for precise code coloring.

## 🎯 Project Vision

Semantic tokens go beyond syntax highlighting by understanding the meaning of identifiers — distinguishing between function calls, variable references, type names, and more. `oak-semantic-tokens` provides the infrastructure for implementing semantic highlighting that integrates with the LSP Semantic Tokens specification.

## ✨ Core Features

- **🎨 Semantic Token Type**: LSP-compatible `SemanticToken` with delta encoding for efficient transfer.
- **📊 Provider Trait**: The `SemanticTokensProvider` trait for implementing language-specific highlighting.
- **🔧 Line Map Integration**: Works with `oak-vfs::LineMap` for position conversion.
- **⚡ Efficient Encoding**: Delta-based encoding minimizes data transfer to clients.
- **🔄 Serde Support**: Optional serialization for LSP integration.

## 🏗️ Architecture

### `SemanticToken` Struct

LSP-compatible token representation:

```rust
pub struct SemanticToken {
    pub delta_line: u32,           // Line delta from previous token
    pub delta_start: u32,          // Start character delta
    pub length: u32,               // Token length in characters
    pub token_type: u32,           // Type index (function, variable, etc.)
    pub token_modifiers_bitset: u32, // Modifiers (static, readonly, etc.)
}
```

### `SemanticTokensProvider` Trait

Implement to provide semantic tokens:

```rust
use oak_semantic_tokens::{SemanticTokensProvider, SemanticToken};
use oak_core::{Language, source::Source, tree::RedNode};
use oak_vfs::LineMap;

struct MySemanticTokensProvider;

impl<L: Language> SemanticTokensProvider<L> for MySemanticTokensProvider {
    fn semantic_tokens<S: Source + ?Sized>(
        &self,
        root: &RedNode<L>,
        source: &S,
        line_map: &LineMap
    ) -> Vec<SemanticToken> {
        // Return semantic tokens for the document
        vec![]
    }
}
```

### LSP Compatibility

The token format matches LSP's `SemanticTokens` specification:
- Uses delta encoding for compact representation
- Supports token types and modifiers via legend indices
- Works with `textDocument/semanticTokens/full` and `range` requests

## 🔗 Integration

`oak-semantic-tokens` is used by:
- `oak-lsp` for `textDocument/semanticTokens` support
- IDE extensions for enhanced syntax highlighting
- Code analysis tools for semantic visualization

## 🤝 Contributing

Contributions are welcome! Please feel free to submit a Pull Request. For major changes, please open an issue first to discuss what you would like to change.

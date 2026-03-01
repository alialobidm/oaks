# 🚀 Oak WAT Parser

[![Crates.io](https://img.shields.io/crates/v/oak-wat.svg)](https://crates.io/crates/oak-wat)
[![Documentation](https://docs.rs/oak-wat/badge.svg)](https://docs.rs/oak-wat)

**WebAssembly Text Format Parsing Made Simple** — A high-performance, incremental WAT parser built on the Oak framework. Optimized for WebAssembly development, tooling, and debugging.

## 🎯 Project Vision

The WebAssembly Text Format (WAT) is the human-readable representation of WebAssembly binaries. `oak-wat` provides industrial-grade parsing support for WAT, enabling developers to build tools for WebAssembly development, debugging, and analysis with the performance and reliability of Rust.

## ✨ Core Features

- **⚡ Blazing Fast**: Leverages Rust's performance to parse WAT files with sub-millisecond latency, essential for build tools and IDEs.
- **🔄 Incremental Parsing**: Built-in support for partial updates—re-parse only what changed, ideal for large WebAssembly modules.
- **🌳 High-Fidelity AST**: Generates a comprehensive Abstract Syntax Tree capturing:
    - **Modules & Components**: Full support for WebAssembly modules and the component model.
    - **Functions & Instructions**: Detailed mapping of function definitions and instruction sequences.
    - **Memory & Tables**: Precise tracking of linear memory and table declarations.
    - **Imports & Exports**: Robust handling of module interfaces.
- **🛡️ Robust Error Recovery**: Gracefully handles malformed WAT syntax, providing precise diagnostics for debugging.
- **🧩 Deep Ecosystem Integration**: Works with `oak-lsp` for full LSP support and `oak-mcp` for intelligent WebAssembly tooling.

## 🏗️ Architecture

The parser follows the **Green/Red Tree** architecture (inspired by Roslyn):
1. **Efficient Immutability**: Share nodes across tree versions without expensive copies.
2. **Lossless Syntax Trees**: Retains all trivia (whitespace and comments), enabling faithful round-trip processing.
3. **Type Safety**: Strongly-typed "Red" nodes provide a convenient and safe API for tree traversal.

## 🤝 Contributing

Contributions are welcome! Please feel free to submit a Pull Request. For major changes, please open an issue first to discuss what you would like to change.

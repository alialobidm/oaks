# 🚀 Oak Nim Parser

[![Crates.io](https://img.shields.io/crates/v/oak-nim.svg)](https://crates.io/crates/v/oak-nim)
[![Documentation](https://docs.rs/oak-nim/badge.svg)](https://docs.rs/oak-nim)

**Efficiency Meets Expressiveness** — A high-performance, incremental Nim parser built on the Oak framework. Optimized for Nim's unique indentation-based syntax, metaprogramming features, and modern developer tooling.

## 🎯 Project Vision

Nim combines Python-like expressiveness with C-like performance, featuring indentation-based syntax, powerful metaprogramming, and compile-time execution. `oak-nim` provides a modern, Rust-powered parsing infrastructure that handles Nim's unique syntax challenges while delivering sub-millisecond performance for responsive IDE integration.

## ✨ Core Features

- **⚡ Blazing Fast**: Leverages Rust's performance to deliver sub-millisecond parsing, essential for real-time IDE feedback.
- **🔄 Incremental Parsing**: Built-in support for partial updates—re-parse only what changed, ideal for large Nim projects.
- **🌳 High-Fidelity AST**: Generates a comprehensive Abstract Syntax Tree capturing:
    - **Indentation-Based Blocks**: Precise tracking of Nim's significant whitespace.
    - **Metaprogramming**: Support for macros, templates, and compile-time code generation.
    - **Type System**: Detailed mapping of Nim's type system including generics and type classes.
    - **Pragmas**: Robust handling of Nim's pragma system for compiler directives.
- **🛡️ Robust Error Recovery**: Gracefully handles incomplete or malformed code, providing precise diagnostics.
- **🧩 Deep Ecosystem Integration**: Works with `oak-lsp` for full LSP support and `oak-mcp` for intelligent code analysis.

## 🏗️ Architecture

The parser follows the **Green/Red Tree** architecture (inspired by Roslyn):
1. **Efficient Immutability**: Share nodes across tree versions without expensive copies.
2. **Lossless Syntax Trees**: Retains all trivia (whitespace and comments), essential for Nim's indentation semantics.
3. **Type Safety**: Strongly-typed "Red" nodes provide a convenient and safe API for tree traversal.

## 🤝 Contributing

Contributions are welcome! Please feel free to submit a Pull Request. For major changes, please open an issue first to discuss what you would like to change.

# 🚀 Oak Lean Parser

[![Crates.io](https://img.shields.io/crates/v/oak-lean.svg)](https://crates.io/crates/oak-lean)
[![Documentation](https://docs.rs/oak-lean/badge.svg)](https://docs.rs/oak-lean)

**Formal Verification Meets Modern Tooling** — A high-performance, incremental Lean parser built on the Oak framework. Optimized for interactive theorem proving, formal verification workflows, and modern IDE support.

## 🎯 Project Vision

Lean is a powerful theorem prover and programming language used for formal verification and mathematical proofs. `oak-lean` brings modern parsing infrastructure to the Lean ecosystem, providing high-performance, Rust-powered analysis capabilities for building responsive IDEs, proof assistants, and verification tools.

## ✨ Core Features

- **⚡ Blazing Fast**: Leverages Rust's performance to parse complex Lean files with sub-millisecond latency.
- **🔄 Incremental Parsing**: Built-in support for partial updates—essential for interactive proof development where files change frequently.
- **🌳 High-Fidelity AST**: Generates a comprehensive Abstract Syntax Tree capturing:
    - **Definitions**: Theorems, lemmas, definitions, and inductive types.
    - **Proofs**: Detailed mapping of proof terms and tactic sequences.
    - **Type Classes**: Support for Lean's powerful type class system.
    - **Unification**: Precise tracking of implicit arguments and unification.
- **🛡️ Robust Error Recovery**: Gracefully handles incomplete proofs and malformed syntax, providing precise diagnostics.
- **🧩 Deep Ecosystem Integration**: Works with `oak-lsp` for full LSP support and `oak-mcp` for intelligent proof assistance.

## 🏗️ Architecture

The parser follows the **Green/Red Tree** architecture (inspired by Roslyn):
1. **Efficient Immutability**: Share nodes across tree versions without expensive copies.
2. **Lossless Syntax Trees**: Retains all trivia (whitespace and comments), enabling faithful code formatting.
3. **Type Safety**: Strongly-typed "Red" nodes provide a convenient and safe API for tree traversal.

## 🤝 Contributing

Contributions are welcome! Please feel free to submit a Pull Request. For major changes, please open an issue first to discuss what you would like to change.

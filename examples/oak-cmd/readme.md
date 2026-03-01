# 🚀 Oak CMD Parser

[![Crates.io](https://img.shields.io/crates/v/oak-cmd.svg)](https://crates.io/crates/oak-cmd)
[![Documentation](https://docs.rs/oak-cmd/badge.svg)](https://docs.rs/oak-cmd)

**Windows Batch Script Analysis** — A high-performance, incremental Windows batch script parser built on the Oak framework. Optimized for DevOps automation, script analysis, and IDE support.

## 🎯 Project Vision

Windows batch scripts (.bat, .cmd) remain essential for Windows automation and system administration. `oak-cmd` provides modern parsing infrastructure for batch scripts, enabling developers to build linting tools, IDE support, and analysis utilities for the Windows scripting ecosystem.

## ✨ Core Features

- **⚡ Blazing Fast**: Leverages Rust's performance to parse batch scripts with sub-millisecond latency.
- **🔄 Incremental Parsing**: Built-in support for partial updates—re-parse only what changed, ideal for IDE integration.
- **🌳 High-Fidelity AST**: Generates a comprehensive Abstract Syntax Tree capturing:
    - **Commands**: Detailed mapping of internal and external commands.
    - **Control Flow**: Support for `IF`, `FOR`, `GOTO`, and `CALL` constructs.
    - **Variables**: Precise tracking of environment variables and parameter expansion.
    - **Redirection**: Robust handling of pipes, file redirection, and command chaining.
- **🛡️ Robust Error Recovery**: Gracefully handles malformed scripts, providing precise diagnostics for common batch script issues.
- **🧩 Deep Ecosystem Integration**: Works with `oak-lsp` for full LSP support and `oak-mcp` for intelligent script analysis.

## 🏗️ Architecture

The parser follows the **Green/Red Tree** architecture (inspired by Roslyn):
1. **Efficient Immutability**: Share nodes across tree versions without expensive copies.
2. **Lossless Syntax Trees**: Retains all trivia (whitespace and comments), enabling faithful script formatting.
3. **Type Safety**: Strongly-typed "Red" nodes provide a convenient and safe API for tree traversal.

## 🤝 Contributing

Contributions are welcome! Please feel free to submit a Pull Request. For major changes, please open an issue first to discuss what you would like to change.

# 🚀 oak-pretty-print

[![Crates.io](https://img.shields.io/crates/v/oak-pretty-print.svg)](https://crates.io/crates/oak-pretty-print)
[![Documentation](https://docs.rs/oak-pretty-print/badge.svg)](https://docs.rs/oak-pretty-print)

**Code Formatting and Pretty Printing for Oak** — Lossless code formatting infrastructure for Oak language parsers.

## 🎯 Project Vision

Code formatting is essential for readability and consistency. `oak-pretty-print` provides the infrastructure for implementing code formatters that preserve all source details, including comments and whitespace, while applying consistent formatting rules.

## ✨ Core Features

- **📝 Lossless Formatting**: Preserves all source details including comments and trivia.
- **🌳 Tree-Based**: Works directly with Oak's syntax trees for accurate formatting.
- **🔧 Configurable**: Support for different formatting styles and options.
- **⚡ Incremental**: Efficient re-formatting of changed sections only.
- **🧩 Language Agnostic**: Works with any Oak language parser.

## 🏗️ Architecture

### Formatting Approach

`oak-pretty-print` operates on the Green/Red tree structure:

1. **Trivia Manipulation**: Adjust whitespace, line breaks, and comments within `GreenNode`.
2. **Rule-Based**: Apply language-specific formatting rules to tree nodes.
3. **Lossless Round-Trip**: Reconstruct source text with formatting applied.

### Integration with Oak

```rust
use oak_pretty_print::Formatter;
use oak_core::tree::GreenNode;

// Format a syntax tree
let formatted = formatter.format(&green_tree);
```

## 🔗 Integration

`oak-pretty-print` integrates with:
- `oak-lsp` for `textDocument/formatting` support
- Language-specific formatters
- Code style enforcement tools

## 🤝 Contributing

Contributions are welcome! Please feel free to submit a Pull Request. For major changes, please open an issue first to discuss what you would like to change.

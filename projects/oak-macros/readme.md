# 🚀 oak-macros

[![Crates.io](https://img.shields.io/crates/v/oak-macros.svg)](https://crates.io/crates/oak-macros)
[![Documentation](https://docs.rs/oak-macros/badge.svg)](https://docs.rs/oak-macros)

**Procedural Macros for Oak** — Derive macros and helper attributes for Oak language implementations.

## 🎯 Project Vision

Procedural macros reduce boilerplate in language implementations. `oak-macros` provides derive macros and helper attributes that simplify implementing Oak traits and generating boilerplate code for language parsers.

## ✨ Core Features

- **🔧 Derive Macros**: Automatically implement common Oak traits.
- **📝 Code Generation**: Generate repetitive code patterns.
- **🎯 Type-Safe**: Compile-time verification of generated code.
- **⚡ Zero Runtime Cost**: Macros expand at compile time.

## 🏗️ Architecture

### Available Macros

| Macro | Purpose |
|-------|---------|
| `#[derive(Language)]` | Auto-implement the `Language` trait |
| `json!` | JSON literal construction using oak-json |

### Usage Example

```rust
use oak_macros::json;

// Create JSON values with literal syntax
let value = json!({
    "name": "Oak",
    "features": ["fast", "incremental"],
    "version": 1
});
```

## 🔗 Integration

`oak-macros` is used by:
- Language parser implementations
- Test code generation
- AST construction utilities

## 🤝 Contributing

Contributions are welcome! Please feel free to submit a Pull Request. For major changes, please open an issue first to discuss what you would like to change.

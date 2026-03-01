# 🚀 oak-highlight

[![Crates.io](https://img.shields.io/crates/v/oak-highlight.svg)](https://crates.io/crates/oak-highlight)
[![Documentation](https://docs.rs/oak-highlight/badge.svg)](https://docs.rs/oak-highlight)

**Multi-Format Syntax Highlighting for Oak Languages** — A flexible syntax highlighting library with support for ANSI, HTML, CSS, and JSON output formats.

## 🎯 Project Vision

Syntax highlighting transforms code from plain text into visually structured, readable content. `oak-highlight` provides a unified highlighting infrastructure that works across all Oak language parsers, with multiple output formats suitable for terminals, web applications, and IDE integration.

## ✨ Core Features

- **🎨 Multiple Export Formats**: Built-in exporters for ANSI (terminal), HTML, CSS classes, and JSON.
- **🔤 Token-Based Highlighting**: Works with Oak's token stream for fast, accurate highlighting.
- **🎭 Theme Support**: Pluggable theming system with predefined themes and custom theme loading.
- **⚡ Zero-Allocation Design**: Highlight results borrow from source text for maximum performance.
- **🧩 Language Agnostic**: Works with any Oak language parser through the `Highlighter` trait.

## 🏗️ Architecture

### Core Types

- **`OakHighlighter`**: The main highlighter that processes source text into highlighted segments.
- **`HighlightResult`**: A collection of highlighted segments with style information.
- **`HighlightSegment`**: A single highlighted region with text and style.
- **`HighlightStyle`**: Style information including foreground/background colors and text attributes.

### Exporters

Convert `HighlightResult` to various formats:

```rust
use oak_highlight::{OakHighlighter, AnsiExporter, HtmlExporter, Exporter};

let highlighter = OakHighlighter::new();
let result = highlighter.highlight(source_code, &language);

// Export to ANSI for terminal output
let ansi_output = AnsiExporter.export(&result);

// Export to HTML for web display
let html_output = HtmlExporter.export(&result);
```

### Available Exporters

| Exporter | Output Format | Use Case |
|----------|---------------|----------|
| `AnsiExporter` | ANSI escape codes | Terminal, CLI tools |
| `HtmlExporter` | HTML with inline styles | Web applications |
| `CssExporter` | HTML with CSS classes | Web with custom styling |
| `JsonExporter` | JSON array | Tooling integration |

### Themes

```rust
use oak_highlight::{HighlightTheme, Theme};

// Load a custom theme
let theme = Theme::from_toml("theme.toml")?;

// Use with highlighter
let highlighter = OakHighlighter::with_theme(theme);
```

## 🔗 Integration

`oak-highlight` is used by:
- `oak-repl` for terminal syntax highlighting
- `oak-lsp` for semantic token support
- Language-specific parsers for preview generation

## 🤝 Contributing

Contributions are welcome! Please feel free to submit a Pull Request. For major changes, please open an issue first to discuss what you would like to change.

# 🚀 oak-diagnostic

[![Crates.io](https://img.shields.io/crates/v/oak-diagnostic.svg)](https://crates.io/crates/oak-diagnostic)
[![Documentation](https://docs.rs/oak-diagnostic/badge.svg)](https://docs.rs/oak-diagnostic)

**Diagnostic Reporting for Oak Languages** — Structured error and warning reporting for language analysis tools.

## 🎯 Project Vision

Diagnostics are the feedback mechanism that helps developers fix issues in their code. `oak-diagnostic` provides a unified infrastructure for reporting errors, warnings, and hints with precise source locations and actionable messages.

## ✨ Core Features

- **📊 Structured Diagnostics**: Rich diagnostic types with severity, message, and location.
- **📍 Precise Locations**: Byte-offset based ranges for accurate error highlighting.
- **🔧 Related Information**: Support for related locations and additional context.
- **🎯 Severity Levels**: Error, warning, information, and hint severity levels.
- **🔄 Serde Support**: Optional serialization for LSP integration.

## 🏗️ Architecture

### Diagnostic Types

```rust
pub enum DiagnosticSeverity {
    Error,       // Blocking issues
    Warning,     // Potential problems
    Information, // Informational messages
    Hint,        // Suggestions for improvement
}
```

### Integration with Oak Core

Diagnostics integrate with `oak-core`'s error types:

```rust
use oak_diagnostic::{Diagnostic, DiagnosticSeverity};
use oak_core::Range;

let diagnostic = Diagnostic {
    range: Range { start: 10, end: 20 },
    severity: DiagnosticSeverity::Error,
    message: "Unexpected token".to_string(),
    related_info: vec![],
};
```

## 🔗 Integration

`oak-diagnostic` is used by:
- `oak-lsp` for `textDocument/publishDiagnostics`
- Language parsers for syntax error reporting
- Static analysis tools for linting

## 🤝 Contributing

Contributions are welcome! Please feel free to submit a Pull Request. For major changes, please open an issue first to discuss what you would like to change.

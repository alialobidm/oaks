# 🚀 oak-repl

[![Crates.io](https://img.shields.io/crates/v/oak-repl.svg)](https://crates.io/crates/oak-repl)
[![Documentation](https://docs.rs/oak-repl/badge.svg)](https://docs.rs/oak-repl)

**Interactive REPL Framework for Oak Languages** — A terminal-based Read-Eval-Print Loop framework with multi-line support, syntax highlighting, and customizable handlers.

## 🎯 Project Vision

REPLs are essential for interactive development, experimentation, and learning. `oak-repl` provides a complete, terminal-based REPL framework that integrates deeply with Oak language features, supporting multi-line input, real-time syntax highlighting, and custom language handlers.

## ✨ Core Features

- **🖥️ Terminal Integration**: Built on `crossterm` for cross-platform terminal handling with raw mode support.
- **📝 Multi-Line Input**: Automatically detects incomplete statements and continues input across multiple lines.
- **🎨 Syntax Highlighting**: Real-time syntax highlighting using `oak-highlight` integration.
- **⌨️ Rich Editing**: Cursor movement, backspace, and auto-indentation support.
- **🔌 Customizable Handlers**: The `ReplHandler` trait allows implementing language-specific behavior.
- **🛡️ Error Recovery**: Graceful handling of errors with continuation support.

## 🏗️ Architecture

### `ReplHandler` Trait
Implement this trait to provide language-specific REPL behavior:

```rust
use oak_repl::{HandleResult, ReplError, ReplHandler, OakRepl};

struct MyHandler;

impl ReplHandler for MyHandler {
    fn prompt(&self, is_continuation: bool) -> &str {
        if is_continuation { "... " } else { ">>> " }
    }

    fn is_complete(&self, code: &str) -> bool {
        // Return true when code forms a complete statement
        code.ends_with(';')
    }

    fn handle_line(&mut self, line: &str) -> Result<HandleResult, ReplError> {
        // Process the complete input
        println!("Executing: {}", line);
        Ok(HandleResult::Continue)
    }
}
```

### `OakRepl` Struct
The main REPL engine that manages terminal interaction:

```rust
let mut repl = OakRepl::new(MyHandler);
repl.run().expect("REPL failed");
```

### `LineBuffer` Struct
Manages multi-line text input with cursor positioning and editing operations.

## 🎮 Key Bindings

| Key | Action |
|-----|--------|
| `Enter` | Submit line (if complete) or continue multi-line |
| `Backspace` | Delete character before cursor |
| `←` / `→` | Move cursor left/right |
| `Ctrl+C` | Clear current input |
| `Ctrl+D` | Exit REPL (when buffer is empty) |

## 🔗 Integration

`oak-repl` integrates with:
- `oak-highlight` for syntax highlighting
- Any language parser implementing `ReplHandler`

## 🤝 Contributing

Contributions are welcome! Please feel free to submit a Pull Request. For major changes, please open an issue first to discuss what you would like to change.

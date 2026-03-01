# 🚀 oak-testing

[![Crates.io](https://img.shields.io/crates/v/oak-testing.svg)](https://crates.io/crates/oak-testing)
[![Documentation](https://docs.rs/oak-testing/badge.svg)](https://docs.rs/oak-testing)

**Testing Utilities for Oak Parsers** — Helpers and macros for testing Oak language implementations.

## 🎯 Project Vision

Testing parsers requires specialized utilities for comparing syntax trees, checking error recovery, and validating incremental parsing. `oak-testing` provides these tools to make writing tests for Oak language parsers straightforward and maintainable.

## ✨ Core Features

- **🧪 Parse Testing**: Helpers for parsing and validating syntax trees.
- **📊 Snapshot Testing**: Compare parsed output against expected snapshots.
- **🔍 Error Checking**: Validate error recovery and diagnostic output.
- **⚡ Incremental Testing**: Test incremental parsing behavior.
- **📝 Macro Support**: Convenient macros for common test patterns.

## 🏗️ Architecture

### Test Helpers

```rust
use oak_testing::{assert_parse_ok, assert_parse_err};

// Test successful parsing
assert_parse_ok!(parser, "fn main() {}", expected_tree);

// Test error recovery
assert_parse_err!(parser, "fn main(}", expected_errors);
```

### Snapshot Testing

```rust
use oak_testing::snapshot_test;

// Compare against stored snapshot
snapshot_test!(parser, source_file, "tests/snapshots/test_name.snap");
```

## 🔗 Integration

`oak-testing` is used by:
- All Oak language parsers for unit tests
- Integration tests for LSP features
- Continuous integration pipelines

## 🤝 Contributing

Contributions are welcome! Please feel free to submit a Pull Request. For major changes, please open an issue first to discuss what you would like to change.

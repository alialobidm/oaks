# 🚀 oak-resolver

[![Crates.io](https://img.shields.io/crates/v/oak-resolver.svg)](https://crates.io/crates/oak-resolver)
[![Documentation](https://docs.rs/oak-resolver/badge.svg)](https://docs.rs/oak-resolver)

**Symbol Resolution for Oak Languages** — Cross-file symbol resolution and import handling for language analysis.

## 🎯 Project Vision

Modern codebases span multiple files and modules. `oak-resolver` provides the infrastructure for resolving symbol references across file boundaries, handling imports, and building symbol indexes for workspace-wide analysis.

## ✨ Core Features

- **🔍 Cross-File Resolution**: Resolve symbols across multiple source files.
- **📦 Import Handling**: Process import statements and module dependencies.
- **📊 Symbol Indexing**: Build and query symbol indexes for fast lookups.
- **🌐 Workspace Support**: Scale from single files to large projects.
- **🔄 Incremental Updates**: Efficiently update resolution results on file changes.

## 🏗️ Architecture

### Resolution Process

1. **Parse**: Parse source files into syntax trees.
2. **Collect**: Extract symbol definitions from each file.
3. **Index**: Build a workspace-wide symbol index.
4. **Resolve**: Match references to their definitions.

### Integration with VFS

```rust
use oak_resolver::Resolver;
use oak_vfs::Vfs;

let resolver = Resolver::new(&vfs);
let definition = resolver.resolve_definition(uri, offset);
```

## 🔗 Integration

`oak-resolver` integrates with:
- `oak-vfs` for file system access
- `oak-navigation` for definition/reference providers
- `oak-lsp` for workspace symbol support

## 🤝 Contributing

Contributions are welcome! Please feel free to submit a Pull Request. For major changes, please open an issue first to discuss what you would like to change.

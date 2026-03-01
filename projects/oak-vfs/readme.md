# 🚀 oak-vfs

[![Crates.io](https://img.shields.io/crates/v/oak-vfs.svg)](https://crates.io/crates/oak-vfs)
[![Documentation](https://docs.rs/oak-vfs/badge.svg)](https://docs.rs/oak-vfs)

**Virtual File System for Oak Language Tools** — A unified abstraction for file system operations supporting both in-memory and disk-based storage, with line/column mapping.

## 🎯 Project Vision

Language tools need to access files from various sources — local disk, in-memory buffers, remote storage, or virtual projects. `oak-vfs` provides a unified abstraction layer that decouples language analysis from storage implementation, enabling tools to work with any file source transparently.

## ✨ Core Features

- **📁 Unified Abstraction**: The `Vfs` trait provides a consistent interface for any file source.
- **💾 Memory VFS**: `MemoryVfs` for in-memory projects, testing, and sandboxed environments.
- **💿 Disk VFS**: `DiskVfs` with file watching support for real IDE integration.
- **📊 Line Mapping**: Built-in `LineMap` for translating byte offsets to line/column positions.
- **🔄 Serde Support**: Optional serialization for file metadata and line maps.

## 🏗️ Architecture

### `Vfs` Trait

The core abstraction for file system access:

```rust
use oak_vfs::{Vfs, FileMetadata, FileType, LineMap};
use oak_core::source::SourceId;

impl Vfs for MyVfs {
    type Source = MySource;

    fn get_source(&self, uri: &str) -> Option<Self::Source>;
    fn get_uri(&self, id: SourceId) -> Option<Arc<str>>;
    fn get_id(&self, uri: &str) -> Option<SourceId>;
    fn exists(&self, uri: &str) -> bool;
    fn metadata(&self, uri: &str) -> Option<FileMetadata>;
    fn read_dir(&self, uri: &str) -> Option<Vec<Arc<str>>>;
}
```

### `MemoryVfs`

In-memory file system for testing and virtual projects:

```rust
use oak_vfs::MemoryVfs;

let mut vfs = MemoryVfs::new();
vfs.write_file("file:///test.rs", "fn main() {}".into());

let source = vfs.get_source("file:///test.rs");
assert!(source.is_some());
```

### `DiskVfs` (feature: `disk`)

Real file system with watching:

```rust
use oak_vfs::{DiskVfs, DiskWatcher, VfsEvent};

let vfs = DiskVfs::new("/project/root")?;
let mut watcher = DiskWatcher::new()?;

for event in watcher.events() {
    match event {
        VfsEvent::Created(uri) => { /* handle */ }
        VfsEvent::Modified(uri) => { /* handle */ }
        VfsEvent::Deleted(uri) => { /* handle */ }
    }
}
```

### `LineMap`

Efficient line/column mapping:

```rust
use oak_vfs::LineMap;

let line_map = LineMap::from_source(&source);
let (line, column) = line_map.position(byte_offset);
let offset = line_map.offset(line, column);
```

## 🔗 Integration

`oak-vfs` is used by:
- `oak-lsp` for workspace file management
- `oak-mcp` for project analysis
- Language servers for cross-file navigation

## 🤝 Contributing

Contributions are welcome! Please feel free to submit a Pull Request. For major changes, please open an issue first to discuss what you would like to change.

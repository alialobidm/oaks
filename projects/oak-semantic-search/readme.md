# 🚀 oak-semantic-search

[![Crates.io](https://img.shields.io/crates/v/oak-semantic-search.svg)](https://crates.io/crates/oak-semantic-search)
[![Documentation](https://docs.rs/oak-semantic-search/badge.svg)](https://docs.rs/oak-semantic-search)

**Semantic Search for Oak Languages** — Search code by meaning, not just text.

## 🎯 Project Vision

Traditional text search finds literal matches, but semantic search understands code meaning. `oak-semantic-search` enables searching for concepts like "all functions that return Result" or "all implementations of this trait" by leveraging Oak's syntax tree understanding.

## ✨ Core Features

- **🔍 Semantic Queries**: Search by code structure and meaning.
- **📊 Pattern Matching**: Match code patterns across the codebase.
- **🌐 Workspace-Wide**: Search across multiple files and projects.
- **⚡ Fast Indexing**: Efficient indexing for quick searches.
- **🧩 Language Agnostic**: Works with any Oak language parser.

## 🏗️ Architecture

### Query Types

```rust
use oak_semantic_search::{SemanticQuery, SearchPattern};

// Find all function definitions
let query = SemanticQuery::FindDefinitions { kind: DefinitionKind::Function };

// Find all implementations of a trait
let query = SemanticQuery::FindImplementations { trait_name: "Display" };

// Pattern-based search
let pattern = SearchPattern::parse("fn $name($args) -> Result<$type, _>");
```

### Index Building

```rust
use oak_semantic_search::SearchIndex;

let index = SearchIndex::build(&vfs, &parsers);
let results = index.search(query);
```

## 🔗 Integration

`oak-semantic-search` integrates with:
- `oak-vfs` for file access
- `oak-symbols` for symbol extraction
- IDE extensions for advanced search

## 🤝 Contributing

Contributions are welcome! Please feel free to submit a Pull Request. For major changes, please open an issue first to discuss what you would like to change.

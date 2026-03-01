# 🚀 oak-mcp

[![Crates.io](https://img.shields.io/crates/v/oak-mcp.svg)](https://crates.io/crates/oak-mcp)
[![Documentation](https://docs.rs/oak-mcp/badge.svg)](https://docs.rs/oak-mcp)

**Model Context Protocol Integration for Oak** — Enable AI assistants to understand and analyze code through the MCP standard.

## 🎯 Project Vision

The Model Context Protocol (MCP) is an open standard that enables AI assistants to interact with development tools and codebases. `oak-mcp` bridges Oak's parsing capabilities with MCP, allowing AI agents to perform code analysis, navigation, and understanding tasks through a standardized interface.

## ✨ Core Features

- **🤖 MCP Server Implementation**: Provides a standard MCP server that exposes Oak's language analysis capabilities.
- **📊 Code Analysis Tools**: Exposes parsing, symbol extraction, and navigation as MCP tools.
- **🔍 Semantic Understanding**: AI assistants can query symbol definitions, references, and documentation.
- **📁 Project-Wide Analysis**: Integration with `oak-vfs` for multi-file project understanding.
- **🌐 Language Agnostic**: Works with any Oak language parser through the standard interface.

## 🏗️ Architecture

### MCP Tools Provided

| Tool | Description |
|------|-------------|
| `parse_file` | Parse a source file and return its AST structure |
| `find_symbols` | Search for symbols matching a query |
| `get_definition` | Get the definition location of a symbol |
| `find_references` | Find all references to a symbol |
| `get_hover` | Get hover information for a position |

### Server Setup

```rust
use oak_mcp::McpServer;

// Create an MCP server for a specific language
let server = McpServer::new(language_parser);

// Run the server (typically via stdio for MCP communication)
server.run().await?;
```

### Integration with AI Assistants

`oak-mcp` enables AI assistants to:
- Understand code structure without language-specific knowledge
- Navigate codebases using semantic information
- Provide accurate code suggestions based on actual definitions
- Analyze project dependencies and relationships

## 🔗 Integration

`oak-mcp` integrates with:
- `oak-core` for parsing infrastructure
- `oak-vfs` for file system access
- `oak-navigation` for code navigation features
- `oak-symbols` for symbol extraction
- Any MCP-compatible AI assistant

## 🤝 Contributing

Contributions are welcome! Please feel free to submit a Pull Request. For major changes, please open an issue first to discuss what you would like to change.

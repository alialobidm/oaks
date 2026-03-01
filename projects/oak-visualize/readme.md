# 🚀 oak-visualize

[![Crates.io](https://img.shields.io/crates/v/oak-visualize.svg)](https://crates.io/crates/oak-visualize)
[![Documentation](https://docs.rs/oak-visualize/badge.svg)](https://docs.rs/oak-visualize)

**AST Visualization for Oak Languages** — Render syntax trees as graphs and diagrams for debugging and documentation.

## 🎯 Project Vision

Visualizing syntax trees helps developers understand parser behavior and debug issues. `oak-visualize` provides tools for rendering Oak syntax trees as visual graphs, making it easier to inspect tree structure, debug parsing issues, and generate documentation.

## ✨ Core Features

- **📊 Graph Rendering**: Convert syntax trees to visual graph representations.
- **🎨 Multiple Output Formats**: Support for various visualization formats (SVG, DOT, etc.).
- **🔍 Tree Inspection**: Interactive exploration of tree structure.
- **📝 Documentation Generation**: Generate visual documentation for language grammars.
- **🧩 Language Agnostic**: Works with any Oak language parser.

## 🏗️ Architecture

### Visualization Output

Generate visual representations of syntax trees:

```rust
use oak_visualize::TreeVisualizer;
use oak_core::tree::RedNode;

let visualizer = TreeVisualizer::new();
let svg_output = visualizer.to_svg(&root);
let dot_output = visualizer.to_dot(&root);
```

### Supported Formats

| Format | Use Case |
|--------|----------|
| SVG | Web display, documentation |
| DOT (Graphviz) | Further processing, research |
| JSON | Tooling integration |

## 🔗 Integration

`oak-visualize` is used by:
- Parser developers for debugging
- Documentation generators
- IDE extensions for tree inspection

## 🤝 Contributing

Contributions are welcome! Please feel free to submit a Pull Request. For major changes, please open an issue first to discuss what you would like to change.

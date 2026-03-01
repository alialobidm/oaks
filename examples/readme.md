# 🌳 Oak Language Parsers

This directory contains language-specific parsers built on the Oak framework. Each parser follows a consistent architecture while providing language-specific optimizations and features.

## 📚 Available Parsers

### System & Compiled Languages

| Parser | Description | Status |
|--------|-------------|--------|
| [oak-ada](./oak-ada) | Ada parser for safety-critical systems | Active |
| [oak-c](./oak-c) | C parser for systems programming | Mature |
| [oak-cpp](./oak-cpp) | C++ parser with modern standard support | Active |
| [oak-d](./oak-d) | D parser for systems programming | Active |
| [oak-go](./oak-go) | Go parser for cloud-native development | Mature |
| [oak-nim](./oak-nim) | Nim parser with indentation support | Active |
| [oak-rust](./oak-rust) | Rust parser for the Rust ecosystem | Mature |
| [oak-swift](./oak-swift) | Swift parser for Apple platforms | Active |
| [oak-vlang](./oak-vlang) | V parser for simple systems programming | Active |
| [oak-zig](./oak-zig) | Zig parser with comptime support | Active |

### Web & Scripting Languages

| Parser | Description | Status |
|--------|-------------|--------|
| [oak-bash](./oak-bash) | Bash parser for shell scripting | Active |
| [oak-cmd](./oak-cmd) | Windows batch script parser | Active |
| [oak-css](./oak-css) | CSS parser with modern features | Active |
| [oak-dart](./oak-dart) | Dart parser for Flutter/Dart ecosystem | Active |
| [oak-html](./oak-html) | HTML parser with HTML5 support | Active |
| [oak-javascript](./oak-javascript) | JavaScript parser | Active |
| [oak-lua](./oak-lua) | Lua parser for game/embedded scripting | Active |
| [oak-perl](./oak-perl) | Perl parser | Active |
| [oak-php](./oak-php) | PHP parser with modern features | Active |
| [oak-python](./oak-python) | Python parser with type hint support | Mature |
| [oak-ruby](./oak-ruby) | Ruby parser | Active |
| [oak-sass](./oak-sass) | Sass parser | Active |
| [oak-scss](./oak-scss) | SCSS parser | Active |
| [oak-typescript](./oak-typescript) | TypeScript parser | Active |
| [oak-vue](./oak-vue) | Vue SFC parser | Active |

### Data & Configuration

| Parser | Description | Status |
|--------|-------------|--------|
| [oak-csv](./oak-csv) | CSV parser | Active |
| [oak-dsv](./oak-dsv) | Delimiter-separated values parser | Active |
| [oak-ini](./oak-ini) | INI configuration parser | Active |
| [oak-json](./oak-json) | JSON parser with JSON5 support | Mature |
| [oak-nix](./oak-nix) | Nix expression parser | Active |
| [oak-toml](./oak-toml) | TOML parser for Rust configs | Active |
| [oak-tsv](./oak-tsv) | TSV parser | Active |
| [oak-xml](./oak-xml) | XML parser | Active |
| [oak-yaml](./oak-yaml) | YAML parser with anchor support | Active |

### JVM & Functional Languages

| Parser | Description | Status |
|--------|-------------|--------|
| [oak-clojure](./oak-clojure) | Clojure parser | Active |
| [oak-elixir](./oak-elixir) | Elixir parser | Active |
| [oak-elm](./oak-elm) | Elm parser | Active |
| [oak-erlang](./oak-erlang) | Erlang parser | Active |
| [oak-fsharp](./oak-fsharp) | F# parser | Active |
| [oak-haskell](./oak-haskell) | Haskell parser | Active |
| [oak-java](./oak-java) | Java parser with modern features | Mature |
| [oak-kotlin](./oak-kotlin) | Kotlin parser | Active |
| [oak-ocaml](./oak-ocaml) | OCaml parser | Active |
| [oak-scala](./oak-scala) | Scala parser | Active |

### WebAssembly & Low-Level

| Parser | Description | Status |
|--------|-------------|--------|
| [oak-wat](./oak-wat) | WebAssembly Text Format parser | Active |
| [oak-wgsl](./oak-wgsl) | WebGPU Shading Language parser | Active |
| [oak-hlsl](./oak-hlsl) | HLSL shader parser | Active |
| [oak-gsgl](./oak-gsgl) | GSGL shader parser | Active |
| [oak-msil](./oak-msil) | MSIL/CIL parser | Active |

### Proof Assistants & Formal Methods

| Parser | Description | Status |
|--------|-------------|--------|
| [oak-coq](./oak-coq) | Coq proof assistant parser | Active |
| [oak-lean](./oak-lean) | Lean theorem prover parser | Active |

### Specialized Languages

| Parser | Description | Status |
|--------|-------------|--------|
| [oak-apl](./oak-apl) | APL array language parser | Active |
| [oak-j](./oak-j) | J array language parser | Active |
| [oak-koka](./oak-koka) | Koka effect handler parser | Active |
| [oak-r](./oak-r) | R statistical language parser | Active |
| [oak-sql](./oak-sql) | SQL parser with multi-dialect support | Active |

### Markup & Documentation

| Parser | Description | Status |
|--------|-------------|--------|
| [oak-dot](./oak-dot) | DOT graph description parser | Active |
| [oak-markdown](./oak-markdown) | Markdown parser | Active |
| [oak-tex](./oak-tex) | TeX/LaTeX parser | Active |
| [oak-typst](./oak-typst) | Typst parser | Active |

### Internal & Experimental

| Parser | Description | Status |
|--------|-------------|--------|
| [oak-c4](./oak-c4) | C4 model parser | Active |
| [oak-gsgl](./oak-gsgl) | GSGL parser | Active |
| [oak-jasm](./oak-jasm) | JASM assembly parser | Active |
| [oak-mojo](./oak-mojo) | Mojo parser | Active |
| [oak-voc](./oak-voc) | VOC parser | Active |
| [oak-voml](./oak-voml) | VOML parser | Active |
| [oak-von](./oak-von) | VON parser | Active |

## 🏗️ Common Architecture

All Oak language parsers follow a consistent architecture:

### 1. SyntaxKind Enum
Defines all possible syntax elements (tokens and nodes) for the language.

### 2. Language Implementation
Implements the `Language` trait from `oak-core`, providing:
- Token types (`TokenType`)
- Element types (`ElementType`)
- Language metadata

### 3. Lexer
Tokenizes source text into a stream of tokens with:
- Whitespace and comment handling (trivia)
- Error recovery for invalid tokens

### 4. Parser
Converts token streams into syntax trees:
- Green tree construction
- Error recovery for partial parsing
- Incremental parsing support

### 5. AST (Optional)
Typed wrappers over the syntax tree for convenient access.

## 🚀 Quick Start: Adding a New Parser

1. **Create the directory structure:**
   ```
   examples/oak-{language}/
   ├── Cargo.toml
   ├── readme.md
   └── src/
       └── lib.rs
   ```

2. **Define SyntaxKind:**
   ```rust
   #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
   pub enum SyntaxKind {
       // Tokens
       Identifier,
       Number,
       // Nodes
       FunctionDef,
       // ...
   }
   ```

3. **Implement the Language trait:**
   ```rust
   use oak_core::Language;

   pub struct MyLanguage;
   impl Language for MyLanguage {
       type TokenType = MyTokenType;
       type ElementType = MyElementType;
       // ...
   }
   ```

4. **Implement the Lexer and Parser:**
   ```rust
   pub struct MyLexer { /* ... */ }
   pub struct MyParser { /* ... */ }
   ```

5. **Add to workspace:**
   Add the new crate to the root `Cargo.toml`.

## 📖 Reference Implementations

For well-documented reference implementations, see:
- [oak-c](./oak-c) — Mature C parser
- [oak-json](./oak-json) — Clean, simple parser
- [oak-python](./oak-python) — Complex indentation-based parser
- [oak-rust](./oak-rust) — Rust parser with complex syntax

## 🤝 Contributing

Contributions are welcome! When adding a new language parser:
1. Follow the existing architecture pattern
2. Include comprehensive documentation
3. Add tests for parsing correctness
4. Ensure error recovery works properly

For major changes, please open an issue first to discuss what you would like to change.

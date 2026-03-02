# 修复项目结构规范违规 Spec

## Why
`check-structures.log` 报告了 25 个结构违规问题，涉及文件命名不规范、目录结构不符合规范等问题。这些问题影响代码库的一致性和可维护性。

## What Changes
- 重命名 AST 目录下的文件，添加 `_nodes.rs` 后缀（15 个文件）
- 重命名 Parser 目录下的文件，添加 `parse_` 前缀（8 个文件）
- 移动或删除不符合规范的目录（formatter, highlighter）
- 移动不符合规范的文件（kind.rs）

## Impact
- Affected specs: oak-dejavu, oak-fortran, oak-java, oak-jasm, oak-racket, oak-valkyrie, oak-vbnet
- Affected code: 所有涉及项目的 AST、Parser 模块及 mod.rs 文件

## ADDED Requirements

### Requirement: AST 文件命名规范
AST 目录下的所有 Rust 文件（除 mod.rs, lib.rs, element_type.rs 外）必须以 `_nodes.rs` 结尾。

#### Scenario: 重命名 AST 文件
- **WHEN** 文件位于 `src/ast/` 目录下
- **THEN** 文件名应以 `_nodes.rs` 结尾（如 `expression_nodes.rs`）

### Requirement: Parser 文件命名规范
Parser 目录下的所有 Rust 文件（除 mod.rs, lib.rs, element_type.rs, *_parser.rs 外）必须以 `parse_` 开头。

#### Scenario: 重命名 Parser 文件
- **WHEN** 文件位于 `src/parser/` 目录下
- **THEN** 文件名应以 `parse_` 开头（如 `parse_expression.rs`）

### Requirement: 目录结构规范
`src/` 下的子目录必须属于允许列表：`ast`, `builder`, `parser`, `lexer`, `language`, `lsp`, `mcp`。

#### Scenario: 处理非法目录
- **WHEN** 发现 `formatter` 或 `highlighter` 目录直接位于 `src/` 下
- **THEN** 应将其移动到 `src/lsp/` 下或删除空目录

### Requirement: 文件位置规范
`src/` 下的文件（除 lib.rs, main.rs, mod.rs 外）应放置在合适的子目录中。

#### Scenario: 移动非法文件
- **WHEN** 发现 `kind.rs` 直接位于 `src/` 下
- **THEN** 应将其移动到合适的子目录（如 `src/lsp/`）

## MODIFIED Requirements

### Requirement: 模块引用更新
所有重命名或移动的文件，其对应的 `mod.rs` 或 `lib.rs` 中的模块声明和导出语句需要同步更新。

## REMOVED Requirements
无

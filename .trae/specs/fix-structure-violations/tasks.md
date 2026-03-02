# Tasks

## Phase 1: AST 文件重命名

- [ ] Task 1: 重命名 oak-dejavu AST 文件
  - [ ] SubTask 1.1: 重命名 `structure.rs` → `structure_nodes.rs`
  - [ ] SubTask 1.2: 更新 `ast/mod.rs` 中的模块声明

- [ ] Task 2: 重命名 oak-fortran AST 文件（6 个文件）
  - [ ] SubTask 2.1: 重命名 `executable.rs` → `executable_nodes.rs`
  - [ ] SubTask 2.2: 重命名 `expression.rs` → `expression_nodes.rs`
  - [ ] SubTask 2.3: 重命名 `root.rs` → `root_nodes.rs`
  - [ ] SubTask 2.4: 重命名 `specification.rs` → `specification_nodes.rs`
  - [ ] SubTask 2.5: 重命名 `types.rs` → `types_nodes.rs`
  - [ ] SubTask 2.6: 更新 `ast/mod.rs` 中的模块声明

- [ ] Task 3: 重命名 oak-valkyrie AST 文件（6 个文件）
  - [ ] SubTask 3.1: 重命名 `common.rs` → `common_nodes.rs`
  - [ ] SubTask 3.2: 重命名 `expression.rs` → `expression_nodes.rs`
  - [ ] SubTask 3.3: 重命名 `items.rs` → `items_nodes.rs`
  - [ ] SubTask 3.4: 重命名 `pattern.rs` → `pattern_nodes.rs`
  - [ ] SubTask 3.5: 重命名 `root.rs` → `root_nodes.rs`
  - [ ] SubTask 3.6: 重命名 `statement.rs` → `statement_nodes.rs`
  - [ ] SubTask 3.7: 重命名 `types.rs` → `types_nodes.rs`
  - [ ] SubTask 3.8: 更新 `ast/mod.rs` 中的模块声明

## Phase 2: Parser 文件重命名

- [ ] Task 4: 重命名 oak-java Parser 文件（3 个文件）
  - [ ] SubTask 4.1: 重命名 `declaration.rs` → `parse_declaration.rs`
  - [ ] SubTask 4.2: 重命名 `expression.rs` → `parse_expression.rs`
  - [ ] SubTask 4.3: 重命名 `statement.rs` → `parse_statement.rs`
  - [ ] SubTask 4.4: 更新 `parser/mod.rs` 中的模块声明和引用

- [ ] Task 5: 重命名 oak-vbnet Parser 文件（4 个文件）
  - [ ] SubTask 5.1: 重命名 `declaration.rs` → `parse_declaration.rs`
  - [ ] SubTask 5.2: 重命名 `expression.rs` → `parse_expression.rs`
  - [ ] SubTask 5.3: 重命名 `member.rs` → `parse_member.rs`
  - [ ] SubTask 5.4: 重命名 `statement.rs` → `parse_statement.rs`
  - [ ] SubTask 5.5: 更新 `parser/mod.rs` 中的模块声明

- [ ] Task 6: 重命名 oak-valkyrie Parser 文件
  - [ ] SubTask 6.1: 重命名 `string_segments.rs` → `parse_string_segments.rs`
  - [ ] SubTask 6.2: 更新 `parser/mod.rs` 中的模块声明和导出

## Phase 3: 目录结构调整

- [ ] Task 7: 处理 oak-jasm 非法目录
  - [ ] SubTask 7.1: 检查 `src/formatter/` 目录内容
  - [ ] SubTask 7.2: 移动到 `src/lsp/formatter/` 或删除空目录

- [ ] Task 8: 处理 oak-racket 非法目录
  - [ ] SubTask 8.1: 检查 `src/formatter/` 目录内容
  - [ ] SubTask 8.2: 检查 `src/highlighter/` 目录内容
  - [ ] SubTask 8.3: 移动到 `src/lsp/` 下或删除空目录

- [ ] Task 9: 处理 oak-valkyrie 非法目录和文件
  - [ ] SubTask 9.1: 检查 `src/formatter/` 目录内容
  - [ ] SubTask 9.2: 检查 `src/highlighter/` 目录内容
  - [ ] SubTask 9.3: 移动 `src/kind.rs` 到 `src/lsp/kind.rs`
  - [ ] SubTask 9.4: 更新 `lib.rs` 中的模块声明和导出

## Phase 4: 验证

- [ ] Task 10: 运行结构检查脚本验证修复
  - [ ] SubTask 10.1: 运行 `npx tsx scripts/check-structures.ts`
  - [ ] SubTask 10.2: 确认无违规报告

# Task Dependencies
- Task 2, Task 3 可并行执行
- Task 4, Task 5, Task 6 可并行执行
- Task 7, Task 8, Task 9 可并行执行
- Task 10 依赖所有前置任务完成

# Checklist

## AST 文件命名规范
- [ ] oak-dejavu: `structure.rs` 已重命名为 `structure_nodes.rs`
- [ ] oak-fortran: 所有 AST 文件已添加 `_nodes.rs` 后缀
- [ ] oak-valkyrie: 所有 AST 文件已添加 `_nodes.rs` 后缀

## Parser 文件命名规范
- [ ] oak-java: 所有 Parser 文件已添加 `parse_` 前缀
- [ ] oak-vbnet: 所有 Parser 文件已添加 `parse_` 前缀
- [ ] oak-valkyrie: `string_segments.rs` 已重命名为 `parse_string_segments.rs`

## 目录结构规范
- [ ] oak-jasm: `src/formatter/` 已移动到 `src/lsp/formatter/` 或已删除
- [ ] oak-racket: `src/formatter/` 和 `src/highlighter/` 已移动到 `src/lsp/` 下或已删除
- [ ] oak-valkyrie: `src/formatter/` 和 `src/highlighter/` 已移动到 `src/lsp/` 下或已删除

## 文件位置规范
- [ ] oak-valkyrie: `src/kind.rs` 已移动到 `src/lsp/kind.rs`

## 模块引用更新
- [ ] 所有涉及的 `mod.rs` 文件已更新模块声明
- [ ] 所有涉及的 `lib.rs` 文件已更新模块导出

## 最终验证
- [ ] 运行 `npx tsx scripts/check-structures.ts` 无违规报告

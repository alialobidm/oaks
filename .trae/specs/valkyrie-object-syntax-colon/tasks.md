# Tasks

- [ ] Task 1: 修改 AST 定义
  - [ ] SubTask 1.1: 修改 `expression_nodes.rs` 中 `Expr::Object` 结构，将 `block: Block` 改为 `fields: Vec<(Identifier, Option<Expr>)>`
  - [ ] SubTask 1.2: 修改 `pattern_nodes.rs` 中 `Pattern::Class` 的字段解构语法支持
  - [ ] SubTask 1.3: 更新 `With` 表达式的字段更新语法

- [x] Task 2: 修改解析器实现
  - [x] SubTask 2.1: 修改对象构造表达式的解析逻辑，支持 `:` 分隔符
  - [x] SubTask 2.2: 修改模式匹配中对象解构的解析逻辑
  - [x] SubTask 2.3: 支持字段简写语法 `{ x, y }` 等价于 `{ x: x, y: y }`
  - [x] SubTask 2.4: 添加弃用警告（当使用 `=` 语法时）

- [x] Task 3: 修改构建器实现
  - [x] SubTask 3.1: 更新 `build_expr.rs` 中对象表达式的构建逻辑
  - [x] SubTask 3.2: 更新 `build_stmt.rs` 中相关语句的构建逻辑

- [x] Task 4: 修改 zh-hans 文档
  - [x] SubTask 4.1: 修改 `language/syntax/braces.md` - 对象构造语法示例
  - [x] SubTask 4.2: 修改 `language/syntax/definitions.md` - 添加类型定义与对象构造一致性说明
  - [x] SubTask 4.3: 修改 `language/syntax/literals.md` - 对象字面量与类型构造的区分
  - [x] SubTask 4.4: 修改 `language/function-oriented/pattern-match.md` - 对象解构语法
  - [x] SubTask 4.5: 添加设计原理说明文档

- [x] Task 5: 更新测试用例
  - [x] SubTask 5.1: 创建新语法的测试用例
  - [x] SubTask 5.2: 更新现有测试用例使用新语法
  - [x] SubTask 5.3: 添加弃用警告的测试用例

- [x] Task 6: 验证与测试
  - [x] SubTask 6.1: 运行所有测试确保无回归
  - [x] SubTask 6.2: 验证编译通过
  - [x] SubTask 6.3: 检查文档一致性

# Task Dependencies

- Task 1 是基础任务，Task 2 和 Task 3 依赖 Task 1
- Task 4 可以与 Task 1-3 并行执行
- Task 5 依赖 Task 1-3 完成
- Task 6 依赖所有其他任务完成

# Valkyrie 对象构造语法统一性改进 Spec

## Why

当前 Valkyrie 语言的对象构造语法使用等号 `=`，与主流语言和模式匹配语法不一致，存在以下问题：

1. **构造-解构不对称**：构造用 `=`，解构也用 `=`，但视觉上与类型定义割裂
2. **类型-值语义割裂**：类型定义用 `:`，值构造用 `=`，缺乏统一性
3. **生态兼容性差**：与 Rust/JSON/JavaScript/Kotlin 等主流语言的语法差异大

## What Changes

- **BREAKING**: 将对象构造语法从 `=` 改为 `:`
- **BREAKING**: 将模式匹配中的字段解构语法从 `=` 改为 `:`
- 更新 AST 定义以支持新语法
- 更新解析器实现
- 更新所有文档和测试用例

## Impact

- Affected specs: Valkyrie 语言语法规范
- Affected code:
  - `oak-valkyrie/src/ast/expression_nodes.rs` - Object 表达式定义
  - `oak-valkyrie/src/ast/pattern_nodes.rs` - Pattern::Class 定义
  - `oak-valkyrie/src/builder/` - 构建器逻辑
  - `oak-valkyrie/src/parser/` - 解析器逻辑
  - `valkyrie.rs/documentation/zh-hans/` - 所有文档

## ADDED Requirements

### Requirement: 对象构造语法统一性

所有对象构造表达式必须使用冒号 `:` 进行字段赋值，与类型定义语法保持一致。

#### Scenario: 类实例化

- **WHEN** 构造类实例时
- **THEN** 使用 `ClassName { field: value }` 语法

**修改前**:
```valkyrie
let p1 = Point { x = 0.0, y = 0.0 }
Money { amount = self.amount + other.amount, currency = self.currency }
```

**修改后**:
```valkyrie
let p1 = Point { x: 0.0, y: 0.0 }
Money { amount: self.amount + other.amount, currency: self.currency }
```

#### Scenario: 联合类型值构造

- **WHEN** 构造联合类型变体值时
- **THEN** 使用 `Variant { field: value }` 语法

**修改前**:
```valkyrie
Fine { value = 42 }
Fail { error = "error message" }
```

**修改后**:
```valkyrie
Fine { value: 42 }
Fail { error: "error message" }
```

#### Scenario: with 表达式

- **WHEN** 使用 with 表达式进行函数式记录更新时
- **THEN** 使用 `with { field: value }` 语法

**修改前**:
```valkyrie
let p2 = p1.with { x = 20.0, y = 30.0 }
```

**修改后**:
```valkyrie
let p2 = p1.with { x: 20.0, y: 30.0 }
```

#### Scenario: 字段简写

- **WHEN** 字段名与变量名相同时
- **THEN** 支持简写形式 `Point { x, y }`，等价于 `Point { x: x, y: y }`

### Requirement: 模式匹配语法统一性

模式匹配中的字段解构语法与对象构造语法保持一致。

#### Scenario: 对象解构

- **WHEN** 在 match case 中解构对象时
- **THEN** 使用 `case { field: pattern }:` 语法

**修改前**:
```valkyrie
match person {
    case { name = "Alice", age }: "Alice is {age} years old"
}
```

**修改后**:
```valkyrie
match person {
    case { name: "Alice", age }: "Alice is {age} years old"
}
```

### Requirement: 设计原理文档化

在文档中明确说明语法设计的原理。

#### Scenario: 构造-解构对称性说明

- **WHEN** 用户阅读语法文档时
- **THEN** 能够理解 `:` 语法的三大设计原理

**设计原理**:
1. **构造-解构对称性**：构造语法与模式匹配语法镜像一致
2. **类型-值统一性**：`:` 统一表示"属性描述"
3. **生态兼容性**：与 Rust/JSON/JavaScript 等主流语言一致

## MODIFIED Requirements

### Requirement: AST 结构调整

修改 `Expr::Object` 的内部结构，直接存储字段-值对而非 Block。

**修改前**:
```rust
Object {
    callee: Box<Expr>,
    block: Block,  // 内部使用赋值语句
    span: Span,
}
```

**修改后**:
```rust
Object {
    callee: Box<Expr>,
    fields: Vec<(Identifier, Option<Expr>)>,  // (字段名, 可选值)，None 表示简写
    span: Span,
}
```

### Requirement: 语法对比说明

| 场景 | 修改前 (`=`) | 修改后 (`:`) | 说明 |
|------|-------------|-------------|------|
| 类型定义 | `x: f64` | `x: f64` | 保持不变 |
| 对象构造 | `x = 1.0` | `x: 1.0` | 与类型定义一致 |
| 模式匹配 | `name = "Alice"` | `name: "Alice"` | 与构造对称 |
| 变量赋值 | `x = 1.0` | `x = 1.0` | 保持不变 |

**核心区分**：
- `:` → 声明属性（类型定义、对象构造、模式匹配）
- `=` → 执行操作（变量赋值、字段修改）

## REMOVED Requirements

### Requirement: 旧语法支持

**Reason**: 为保持语言简洁性，不再支持 `=` 语法的对象构造
**Migration**: 提供自动迁移工具或脚本，将旧语法转换为新语法

## 迁移策略

采用渐进式迁移：

1. **Phase 1**: 解析器同时支持 `=` 和 `:`，`=` 产生弃用警告
2. **Phase 2**: 默认使用 `:`，`=` 仅在兼容模式下支持
3. **Phase 3**: 完全移除 `=` 支持

## 范围限制

- **仅修改 zh-hans 文档**：zh-hant 和 en-us 是翻译组的工作，不在本次修正范围内
- **修改代码中的文档注释**：Rust 代码中的文档注释如果包含语法示例也需要修正

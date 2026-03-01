/// Valkyrie root node.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ValkyrieRoot {
    pub items: Vec<Item>,
}

/// Source code span
pub type Span = oak_core::Range<usize>;

/// Loop keyword kind for deprecation warnings.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub enum LoopKind {
    /// Using `loop` keyword (preferred).
    #[default]
    Loop,
    /// Using `for` keyword (deprecated, use `loop` instead).
    For,
}

/// Structure keyword kind for class-like definitions.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub enum StructureKind {
    /// Using `class` keyword.
    #[default]
    Class,
    /// Using `struct` keyword (deprecated, use `class` instead).
    Struct,
    /// Using `structure` keyword.
    Structure,
    /// Using `widget` keyword.
    Widget,
    /// Using `trait` keyword.
    Trait,
}

/// Enums keyword kind for deprecation warnings.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub enum EnumsKind {
    /// Using `enums` keyword (preferred).
    #[default]
    Enums,
    /// Using `enum` keyword (deprecated, use `unity` instead).
    Enum,
    /// Using `unity` keyword (preferred alternative).
    Unity,
}

/// An identifier
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Identifier {
    pub name: String,
    pub span: Span,
}

impl Default for Identifier {
    fn default() -> Self {
        Self { name: String::new(), span: Span::default() }
    }
}

/// A name path (e.g., `std::collections::HashMap`)
#[derive(Debug, Clone, PartialEq, Eq, Hash, Default)]
pub struct NamePath {
    pub parts: Vec<Identifier>,
    pub span: Span,
}

/// An item in a Valkyrie module
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Item {
    Namespace(Namespace),
    Class(Class),
    /// A value type structure (immutable, copied on assignment).
    Structure(Class),
    Flags(Flags),
    Enums(Enums),
    Trait(Trait),
    Widget(Widget),
    Using(Using),
    Micro(MicroDefinition),
    TypeFunction(TypeFunction),
    Statement(Statement),
    Variant(Variant),
    Effect(Effect),
    Property(Property),
    TemplateText { content: String, span: Span },
    TemplateControl { items: Vec<Item>, span: Span },
    TemplateInterpolation { expr: Expr, span: Span },
}

/// A namespace declaration
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Namespace {
    pub name: NamePath,
    pub annotations: Vec<Attribute>,
    pub items: Vec<Item>,
    pub span: Span,
}

/// A parent class with optional alias for renamed inheritance.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Default)]
pub struct Parent {
    /// Optional alias for disambiguation (e.g., "primary" in "primary: Parent1").
    pub alias: Option<Identifier>,
    /// Parent class name path.
    pub name: NamePath,
    /// Source span.
    pub span: Span,
}

/// A class declaration
#[derive(Debug, Clone, PartialEq, Eq, Hash, Default)]
pub struct Class {
    pub kind: StructureKind,
    pub name: Identifier,
    pub generics: Vec<GenericParam>,
    pub parents: Vec<Parent>,
    pub items: Vec<Item>,
    pub annotations: Vec<Attribute>,
    pub span: Span,
    /// Whether this class is abstract (cannot be instantiated directly).
    pub is_abstract: bool,
}

/// A flags (bitflags) declaration
#[derive(Debug, Clone, PartialEq, Eq, Hash, Default)]
pub struct Flags {
    pub name: Identifier,
    pub variants: Vec<EnumVariant>,
    pub annotations: Vec<Attribute>,
    pub span: Span,
}

/// An enum declaration
#[derive(Debug, Clone, PartialEq, Eq, Hash, Default)]
pub struct Enums {
    pub kind: EnumsKind,
    pub name: Identifier,
    pub generics: Vec<GenericParam>,
    pub variants: Vec<EnumVariant>,
    pub annotations: Vec<Attribute>,
    pub span: Span,
}

/// A trait declaration
#[derive(Debug, Clone, PartialEq, Eq, Hash, Default)]
pub struct Trait {
    pub name: Identifier,
    pub generics: Vec<GenericParam>,
    pub methods: Vec<Function>,
    pub associated_types: Vec<AssociatedType>,
    pub annotations: Vec<Attribute>,
    pub span: Span,
}

/// An associated type declaration in a trait.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Default)]
pub struct AssociatedType {
    pub name: Identifier,
    pub bounds: Vec<Type>,
    pub default: Option<Type>,
    pub annotations: Vec<Attribute>,
    pub span: Span,
}

/// A widget declaration
#[derive(Debug, Clone, PartialEq, Eq, Hash, Default)]
pub struct Widget {
    pub name: Identifier,
    pub generics: Vec<GenericParam>,
    pub items: Vec<Item>,
    pub annotations: Vec<Attribute>,
    pub span: Span,
}

/// A using (import) statement
#[derive(Debug, Clone, PartialEq, Eq, Hash, Default)]
pub struct Using {
    pub path: NamePath,
    pub alias: Option<Identifier>,
    pub span: Span,
}

/// A micro (small function) declaration
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct MicroDefinition {
    pub name: Identifier,
    pub generics: Vec<GenericParam>,
    pub params: Vec<Param>,
    pub return_type: Option<Type>,
    pub body: Block,
    pub annotations: Vec<Attribute>,
    pub span: Span,
    /// Whether this function is abstract (has no body implementation).
    pub is_abstract: bool,
}

/// A type function declaration
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct TypeFunction {
    pub name: Identifier,
    pub generics: Vec<GenericParam>,
    pub params: Vec<Param>,
    pub return_type: Option<Type>,
    pub body: Block,
    pub annotations: Vec<Attribute>,
    pub span: Span,
}

/// A variant declaration
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Variant {
    pub name: Identifier,
    pub generics: Vec<GenericParam>,
    pub cases: Vec<VariantCase>,
    pub annotations: Vec<Attribute>,
    pub span: Span,
}

/// An effect declaration
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Effect {
    pub name: Identifier,
    pub operations: Vec<Function>,
    pub annotations: Vec<Attribute>,
    pub span: Span,
}

/// The kind of a property (getter or setter).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum PropertyKind {
    /// A getter property.
    Getter,
    /// A setter property.
    Setter,
}

/// A property declaration (getter or setter).
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Property {
    /// The name of the property.
    pub name: Identifier,
    /// Whether this is a getter or setter.
    pub kind: PropertyKind,
    /// Generic parameters for the property.
    pub generics: Vec<GenericParam>,
    /// Annotations on the property.
    pub annotations: Vec<Attribute>,
    /// Parameters for the property (self for getter, self + value for setter).
    pub params: Vec<Param>,
    /// Return type for getter, None for setter.
    pub return_type: Option<Type>,
    /// The body of the property.
    pub body: Block,
    /// Source span.
    pub span: Span,
}

/// A statement
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Statement {
    Let {
        is_mutable: bool,
        pattern: Pattern,
        expr: Expr,
        ty: Option<Type>,
        annotations: Vec<Attribute>,
        span: Span,
    },
    ExprStmt{
        expr: Expr,
        semi: bool,
        annotations: Vec<Attribute>,
        span: Span,
    },
}

/// An expression
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Expr {
    Ident(Identifier),
    Path(NamePath),
    StringLiteral(StringLiteral),
    Bool { value: bool, span: Span },
    Binary { left: Box<Expr>, op: crate::kind::ValkyrieSyntaxKind, right: Box<Expr>, span: Span },
    Unary { op: crate::kind::ValkyrieSyntaxKind, expr: Box<Expr>, span: Span },
    Call { callee: Box<Expr>, args: Vec<Expr>, span: Span },
    Field { receiver: Box<Expr>, field: Identifier, span: Span },
    Index { receiver: Box<Expr>, index: Box<Expr>, span: Span },
    Paren { expr: Box<Expr>, span: Span },
    Block(Block),
    Lambda(LambdaExpr),
    Object { callee: Box<Expr>, block: Block, span: Span },
    /// Anonymous class expression.
    ///
    /// ```v
    /// let obj = class { x: 10, y: 20 }
    /// let impl_trait = class: Trait { ... }
    /// ```
    AnonymousClass {
        /// Parent traits or classes to implement/extend.
        parents: Vec<String>,
        /// Fields and methods defined in the anonymous class.
        items: Vec<Item>,
        /// Variables captured from the enclosing scope.
        captures: Vec<Identifier>,
        /// Source span.
        span: Span,
    },
    If { pattern: Option<Pattern>, condition: Box<Expr>, then_branch: Block, else_branch: Option<Block>, span: Span },
    Match { scrutinee: Box<Expr>, arms: Vec<MatchArm>, span: Span },
    Loop { kind: LoopKind, label: Option<String>, pattern: Option<Pattern>, condition: Option<Box<Expr>>, body: Block, span: Span },
    Return { expr: Option<Box<Expr>>, span: Span },
    Break { label: Option<String>, expr: Option<Box<Expr>>, span: Span },
    Continue { label: Option<String>, span: Span },
    Yield { expr: Option<Box<Expr>>, yield_from: bool, span: Span },
    Raise { expr: Box<Expr>, span: Span },
    Catch { expr: Box<Expr>, arms: Vec<MatchArm>, span: Span },
    /// With expression for functional record updates.
    ///
    /// Creates a new record by copying an existing one and updating specified fields.
    ///
    /// ```v
    /// let p2 = p1.with { x: 20.0, y: 30.0 }
    /// let updated = config.with { timeout: 60 }
    /// ```
    With {
        /// The base expression to copy from.
        base: Box<Expr>,
        /// Field updates to apply.
        updates: Vec<(Identifier, Expr)>,
        /// Source span.
        span: Span,
    },
}

/// A block of statements
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Block {
    pub statements: Vec<Statement>,
    pub span: Span,
}

/// A lambda expression
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct LambdaExpr {
    pub params: Vec<Param>,
    pub return_type: Option<Type>,
    pub body: Block,
    pub span: Span,
}

/// A match arm
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct MatchArm {
    pub pattern: Pattern,
    pub guard: Option<Expr>,
    pub body: Expr,
    pub span: Span,
}

/// A pattern for matching
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Pattern {
    Wildcard { span: Span },
    Variable { name: Identifier, span: Span },
    Literal { value: String, span: Span },
    Type { name: NamePath, span: Span },
    Class { name: NamePath, fields: Vec<(Identifier, Pattern)>, span: Span },
    Else { span: Span },
}

/// A type expression
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Type {
    Named { path: NamePath, span: Span },
    Generic { name: Identifier, span: Span },
    Tuple { elements: Vec<Type>, span: Span },
    Function { params: Vec<Type>, return_type: Box<Type>, span: Span },
    Optional { inner: Box<Type>, span: Span },
}

/// A generic parameter
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct GenericParam {
    pub name: Identifier,
    pub constraints: Vec<Type>,
    pub default: Option<Type>,
    pub span: Span,
}

/// A function parameter
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Param {
    pub name: Identifier,
    pub ty: Option<Type>,
    pub default: Option<Expr>,
    pub span: Span,
}

/// A field in a class or struct
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Field {
    pub name: Identifier,
    pub ty: Type,
    pub default: Option<Expr>,
    pub annotations: Vec<Attribute>,
    pub span: Span,
}

/// A function definition
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Function{
    pub name: Identifier,
    pub generics: Vec<GenericParam>,
    pub params: Vec<Param>,
    pub return_type: Option<Type>,
    pub body: Option<Block>,
    pub annotations: Vec<Attribute>,
    pub span: Span,
    /// Whether this function is abstract (has no body implementation).
    pub is_abstract: bool,
}

/// An enum variant
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct EnumVariant {
    pub name: Identifier,
    pub fields: Vec<Field>,
    pub annotations: Vec<Attribute>,
    pub span: Span,
}

/// A variant case
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct VariantCase{
    pub pattern: Pattern,
    pub body: Expr,
    pub span: Span,
}

/// An attribute
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Attribute{
    pub name: Identifier,
    pub args: Vec<Expr>,
    pub span: Span,
}

/// 字符串字面量节点
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct StringLiteral {
    /// DSL 前缀 (如 `s`, `f`, `r`, `sql`)
    pub prefix: Option<Identifier>,
    /// 引号数量 (1, 2, 3, 4, ...)
    pub quote_count: u8,
    /// 字符串片段
    pub segments: Vec<StringSegment>,
    /// 源码位置
    pub span: Span,
}

/// 字符串片段
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum StringSegment {
    /// 文本内容
    Text {
        /// 文本内容
        content: String,
        /// 源码位置
        span: Span,
    },
    /// 插值表达式
    Interpolation {
        /// 插值表达式
        expr: Box<Expr>,
        /// 是否为 Fluent 变量 (带有 ߷ 标记)
        is_fluent: bool,
        /// 源码位置
        span: Span,
    },
}

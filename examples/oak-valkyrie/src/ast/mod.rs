/// Valkyrie root node.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ValkyrieRoot {
    pub items: Vec<Item>,
}

/// Source code span
#[derive(Debug, Clone, PartialEq, Eq, Hash, Default)]
pub struct Span {
    pub start: u32,
    pub end: u32,
}

impl From<std::ops::Range<usize>> for Span {
    fn from(range: std::ops::Range<usize>) -> Self {
        Self { start: range.start as u32, end: range.end as u32 }
    }
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

/// A class declaration
#[derive(Debug, Clone, PartialEq, Eq, Hash, Default)]
pub struct Class {
    pub name: Identifier,
    pub generics: Vec<GenericParam>,
    pub parents: Vec<NamePath>,
    pub items: Vec<Item>,
    pub annotations: Vec<Attribute>,
    pub span: Span,
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
    ExprStmt {
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
    Literal { value: String, span: Span },
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
    AnonymousClass { parents: Vec<String>, items: Vec<Item>, span: Span },
    If { pattern: Option<Pattern>, condition: Box<Expr>, then_branch: Block, else_branch: Option<Block>, span: Span },
    Match { scrutinee: Box<Expr>, arms: Vec<MatchArm>, span: Span },
    Loop { label: Option<String>, pattern: Option<Pattern>, condition: Option<Box<Expr>>, body: Block, span: Span },
    Return { expr: Option<Box<Expr>>, span: Span },
    Break { label: Option<String>, expr: Option<Box<Expr>>, span: Span },
    Continue { label: Option<String>, span: Span },
    Yield { expr: Option<Box<Expr>>, yield_from: bool, span: Span },
    Raise { expr: Box<Expr>, span: Span },
    Catch { expr: Box<Expr>, arms: Vec<MatchArm>, span: Span },
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
pub struct Function {
    pub name: Identifier,
    pub generics: Vec<GenericParam>,
    pub params: Vec<Param>,
    pub return_type: Option<Type>,
    pub body: Option<Block>,
    pub annotations: Vec<Attribute>,
    pub span: Span,
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
pub struct VariantCase {
    pub pattern: Pattern,
    pub body: Expr,
    pub span: Span,
}

/// An attribute
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Attribute {
    pub name: Identifier,
    pub args: Vec<Expr>,
    pub span: Span,
}

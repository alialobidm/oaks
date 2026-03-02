use crate::lexer::token_type::DejavuTokenType;
use oak_core::Range;

/// Root node of a Dejavu AST.
#[derive(Debug, Clone, PartialEq)]
pub struct DejavuRoot {
    /// Top-level items in the source file.
    pub items: Vec<ItemNode>,
}

/// Identifier node representing a name.
#[derive(Debug, Clone, PartialEq)]
pub struct IdentifierNode {
    /// The identifier name.
    pub name: String,
    /// Source span of the identifier.
    pub span: Range<usize>,
}

/// Name path node representing a qualified path like `foo::bar::baz`.
#[derive(Debug, Clone, PartialEq)]
pub struct NamePathNode {
    /// Parts of the path.
    pub parts: Vec<IdentifierNode>,
    /// Source span of the path.
    pub span: Range<usize>,
}

/// Attribute node representing an annotation like `#[derive(Clone)]`.
#[derive(Debug, Clone, PartialEq)]
pub struct AttributeNode {
    /// Attribute name.
    pub name: IdentifierNode,
    /// Attribute arguments.
    pub args: Vec<ExpressionNode>,
    /// Source span of the attribute.
    pub span: Range<usize>,
}

/// Block node containing a sequence of statements.
#[derive(Debug, Clone, PartialEq)]
pub struct BlockNode {
    /// Statements in the block.
    pub statements: Vec<StatementNode>,
    /// Source span of the block.
    pub span: Range<usize>,
}

/// Parameter node for function definitions.
#[derive(Debug, Clone, PartialEq)]
pub struct ParameterNode {
    /// Parameter annotations.
    pub annotations: Vec<AttributeNode>,
    /// Parameter name.
    pub name: IdentifierNode,
    /// Optional type annotation.
    pub ty: Option<String>,
    /// Source span of the parameter.
    pub span: Range<usize>,
}

/// Item node representing top-level declarations.
#[derive(Debug, Clone, PartialEq)]
pub enum ItemNode {
    /// Namespace declaration.
    Namespace(NamespaceDeclaration),
    /// Class declaration.
    Class(ClassDeclaration),
    /// Flags declaration.
    Flags(FlagsDeclaration),
    /// Enum declaration.
    Enum(EnumDeclaration),
    /// Trait declaration.
    Trait(TraitDeclaration),
    /// Widget declaration.
    Widget(WidgetDeclaration),
    /// Using statement.
    Using(UsingStatement),
    /// Micro definition.
    Micro(MicroDefinition),
    /// Type function definition.
    TypeFunction(TypeFunctionDefinition),
    /// Statement item.
    Statement(StatementNode),
    /// Variant definition.
    Variant(VariantDefinition),
    /// Template control node.
    TemplateControl(TemplateControlNode),
    /// Template interpolation node.
    TemplateInterpolation(TemplateInterpolationNode),
}

/// Namespace declaration.
#[derive(Debug, Clone, PartialEq)]
pub struct NamespaceDeclaration {
    /// Namespace name.
    pub name: NamePathNode,
    /// Namespace annotations.
    pub annotations: Vec<AttributeNode>,
    /// Items in the namespace.
    pub items: Vec<ItemNode>,
    /// Source span of the declaration.
    pub span: Range<usize>,
}

/// Class declaration.
#[derive(Debug, Clone, PartialEq)]
pub struct ClassDeclaration {
    /// Class name.
    pub name: IdentifierNode,
    /// Class annotations.
    pub annotations: Vec<AttributeNode>,
    /// Parent classes/traits.
    pub parents: Vec<NamePathNode>,
    /// Items in the class.
    pub items: Vec<ItemNode>,
    /// Source span of the declaration.
    pub span: Range<usize>,
}

/// Flags declaration for bitflag enums.
#[derive(Debug, Clone, PartialEq)]
pub struct FlagsDeclaration {
    /// Flags name.
    pub name: IdentifierNode,
    /// Flags annotations.
    pub annotations: Vec<AttributeNode>,
    /// Flag variants.
    pub items: Vec<ItemNode>,
    /// Source span of the declaration.
    pub span: Range<usize>,
}

/// Enum declaration.
#[derive(Debug, Clone, PartialEq)]
pub struct EnumDeclaration {
    /// Enum name.
    pub name: IdentifierNode,
    /// Enum annotations.
    pub annotations: Vec<AttributeNode>,
    /// Enum variants.
    pub items: Vec<ItemNode>,
    /// Source span of the declaration.
    pub span: Range<usize>,
}

/// Trait declaration.
#[derive(Debug, Clone, PartialEq)]
pub struct TraitDeclaration {
    /// Trait name.
    pub name: IdentifierNode,
    /// Trait annotations.
    pub annotations: Vec<AttributeNode>,
    /// Parent traits.
    pub parents: Vec<NamePathNode>,
    /// Items in the trait.
    pub items: Vec<ItemNode>,
    /// Source span of the declaration.
    pub span: Range<usize>,
}

/// Widget declaration for UI components.
#[derive(Debug, Clone, PartialEq)]
pub struct WidgetDeclaration {
    /// Widget name.
    pub name: IdentifierNode,
    /// Widget annotations.
    pub annotations: Vec<AttributeNode>,
    /// Items in the widget.
    pub items: Vec<ItemNode>,
    /// Source span of the declaration.
    pub span: Range<usize>,
}

/// Using statement for imports.
#[derive(Debug, Clone, PartialEq)]
pub struct UsingStatement {
    /// Import path.
    pub path: NamePathNode,
    /// Source span of the statement.
    pub span: Range<usize>,
}

/// Micro definition for small functions.
#[derive(Debug, Clone, PartialEq)]
pub struct MicroDefinition {
    /// Function name.
    pub name: IdentifierNode,
    /// Function annotations.
    pub annotations: Vec<AttributeNode>,
    /// Function parameters.
    pub params: Vec<ParameterNode>,
    /// Return type annotation.
    pub return_type: Option<String>,
    /// Function body.
    pub body: BlockNode,
    /// Source span of the definition.
    pub span: Range<usize>,
}

/// Type function definition.
#[derive(Debug, Clone, PartialEq)]
pub struct TypeFunctionDefinition {
    /// Function name.
    pub name: IdentifierNode,
    /// Function annotations.
    pub annotations: Vec<AttributeNode>,
    /// Function parameters.
    pub params: Vec<ParameterNode>,
    /// Return type annotation.
    pub return_type: Option<String>,
    /// Function body.
    pub body: BlockNode,
    /// Source span of the definition.
    pub span: Range<usize>,
}

/// Variant definition for enum variants.
#[derive(Debug, Clone, PartialEq)]
pub struct VariantDefinition {
    /// Variant name.
    pub name: IdentifierNode,
    /// Variant annotations.
    pub annotations: Vec<AttributeNode>,
    /// Optional variant value.
    pub value: Option<ExpressionNode>,
    /// Source span of the definition.
    pub span: Range<usize>,
}

/// Template control node for template directives.
#[derive(Debug, Clone, PartialEq)]
pub struct TemplateControlNode {
    /// Template items.
    pub items: Vec<ItemNode>,
    /// Source span of the node.
    pub span: Range<usize>,
}

/// Template interpolation node for embedded expressions.
#[derive(Debug, Clone, PartialEq)]
pub struct TemplateInterpolationNode {
    /// Interpolated expression.
    pub expr: ExpressionNode,
    /// Source span of the node.
    pub span: Range<usize>,
}

/// Statement node.
#[derive(Debug, Clone, PartialEq)]
pub enum StatementNode {
    /// Let binding statement.
    Let(LetStatement),
    /// Expression statement.
    Expr(ExpressionStatement),
}

/// Let binding statement.
#[derive(Debug, Clone, PartialEq)]
pub struct LetStatement {
    /// Statement annotations.
    pub annotations: Vec<AttributeNode>,
    /// Whether the binding is mutable.
    pub is_mutable: bool,
    /// Binding pattern.
    pub pattern: PatternNode,
    /// Bound expression.
    pub expr: ExpressionNode,
    /// Source span of the statement.
    pub span: Range<usize>,
}

/// Expression statement.
#[derive(Debug, Clone, PartialEq)]
pub struct ExpressionStatement {
    /// Statement annotations.
    pub annotations: Vec<AttributeNode>,
    /// The expression.
    pub expr: ExpressionNode,
    /// Whether a semicolon is present.
    pub semi: bool,
    /// Source span of the statement.
    pub span: Range<usize>,
}

/// Expression node.
#[derive(Debug, Clone, PartialEq)]
pub enum ExpressionNode {
    /// Identifier expression.
    Ident(IdentifierNode),
    /// Path expression.
    Path(NamePathNode),
    /// Literal expression.
    Literal(LiteralExpressionNode),
    /// Boolean literal.
    Bool(BooleanLiteralNode),
    /// Parenthesized expression.
    Paren(ParenthesizedExpressionNode),
    /// Unary expression.
    Unary(UnaryExpressionNode),
    /// Binary expression.
    Binary(BinaryExpressionNode),
    /// Call expression.
    Call(CallExpressionNode),
    /// Field access expression.
    Field(FieldExpressionNode),
    /// Index expression.
    Index(IndexExpressionNode),
    /// If expression.
    If(IfExpressionNode),
    /// Match expression.
    Match(MatchExpressionNode),
    /// Lambda expression.
    Lambda(LambdaExpressionNode),
    /// Object expression.
    Object(ObjectExpressionNode),
    /// Block expression.
    Block(BlockNode),
    /// Loop expression.
    Loop(LoopExpressionNode),
    /// Return expression.
    Return(ReturnExpressionNode),
    /// Break expression.
    Break(BreakExpressionNode),
    /// Continue expression.
    Continue(ContinueExpressionNode),
    /// Yield expression.
    Yield(YieldExpressionNode),
    /// Raise expression.
    Raise(RaiseExpressionNode),
    /// Catch expression.
    Catch(CatchExpressionNode),
    /// Resume expression.
    Resume(ResumeExpressionNode),
}

/// Literal expression node.
#[derive(Debug, Clone, PartialEq)]
pub struct LiteralExpressionNode {
    /// Literal value.
    pub value: String,
    /// Source span of the literal.
    pub span: Range<usize>,
}

/// Boolean literal node.
#[derive(Debug, Clone, PartialEq)]
pub struct BooleanLiteralNode {
    /// Boolean value.
    pub value: bool,
    /// Source span of the literal.
    pub span: Range<usize>,
}

/// Parenthesized expression node.
#[derive(Debug, Clone, PartialEq)]
pub struct ParenthesizedExpressionNode {
    /// Inner expression.
    pub expr: Box<ExpressionNode>,
    /// Source span of the expression.
    pub span: Range<usize>,
}

/// Unary expression node.
#[derive(Debug, Clone, PartialEq)]
pub struct UnaryExpressionNode {
    /// Unary operator.
    pub op: DejavuTokenType,
    /// Operand expression.
    pub expr: Box<ExpressionNode>,
    /// Source span of the expression.
    pub span: Range<usize>,
}

/// Binary expression node.
#[derive(Debug, Clone, PartialEq)]
pub struct BinaryExpressionNode {
    /// Left operand.
    pub left: Box<ExpressionNode>,
    /// Binary operator.
    pub op: DejavuTokenType,
    /// Right operand.
    pub right: Box<ExpressionNode>,
    /// Source span of the expression.
    pub span: Range<usize>,
}

/// Call expression node.
#[derive(Debug, Clone, PartialEq)]
pub struct CallExpressionNode {
    /// Callee expression.
    pub callee: Box<ExpressionNode>,
    /// Call arguments.
    pub args: Vec<ExpressionNode>,
    /// Source span of the expression.
    pub span: Range<usize>,
}

/// Field access expression node.
#[derive(Debug, Clone, PartialEq)]
pub struct FieldExpressionNode {
    /// Receiver expression.
    pub receiver: Box<ExpressionNode>,
    /// Field name.
    pub field: IdentifierNode,
    /// Source span of the expression.
    pub span: Range<usize>,
}

/// Index expression node.
#[derive(Debug, Clone, PartialEq)]
pub struct IndexExpressionNode {
    /// Receiver expression.
    pub receiver: Box<ExpressionNode>,
    /// Index expression.
    pub index: Box<ExpressionNode>,
    /// Source span of the expression.
    pub span: Range<usize>,
}

/// If expression node.
#[derive(Debug, Clone, PartialEq)]
pub struct IfExpressionNode {
    /// Optional pattern for condition binding.
    pub pattern: Option<PatternNode>,
    /// Condition expression.
    pub condition: Box<ExpressionNode>,
    /// Then branch.
    pub then_branch: BlockNode,
    /// Optional else branch.
    pub else_branch: Option<BlockNode>,
    /// Source span of the expression.
    pub span: Range<usize>,
}

/// Match expression node.
#[derive
use crate::lexer::token_type::DejavuTokenType;
use oak_core::Range;

#[derive(Debug, Clone, PartialEq)]
pub struct DejavuRoot {
    pub items: Vec<ItemNode>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct IdentifierNode {
    pub name: String,
    pub span: Range<usize>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct NamePathNode {
    pub parts: Vec<IdentifierNode>,
    pub span: Range<usize>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct AttributeNode {
    pub name: IdentifierNode,
    pub args: Vec<ExpressionNode>,
    pub span: Range<usize>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct BlockNode {
    pub statements: Vec<StatementNode>,
    pub span: Range<usize>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct ParameterNode {
    pub annotations: Vec<AttributeNode>,
    pub name: IdentifierNode,
    pub ty: Option<String>,
    pub span: Range<usize>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum ItemNode {
    Namespace(NamespaceDeclaration),
    Class(ClassDeclaration),
    Flags(FlagsDeclaration),
    Enum(EnumDeclaration),
    Trait(TraitDeclaration),
    Widget(WidgetDeclaration),
    Using(UsingStatement),
    Micro(MicroDefinition),
    TypeFunction(TypeFunctionDefinition),
    Statement(StatementNode),
    Variant(VariantDefinition),
    TemplateControl(TemplateControlNode),
    TemplateInterpolation(TemplateInterpolationNode),
}

#[derive(Debug, Clone, PartialEq)]
pub struct NamespaceDeclaration {
    pub name: NamePathNode,
    pub annotations: Vec<AttributeNode>,
    pub items: Vec<ItemNode>,
    pub span: Range<usize>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct ClassDeclaration {
    pub name: IdentifierNode,
    pub annotations: Vec<AttributeNode>,
    pub parents: Vec<NamePathNode>,
    pub items: Vec<ItemNode>,
    pub span: Range<usize>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct FlagsDeclaration {
    pub name: IdentifierNode,
    pub annotations: Vec<AttributeNode>,
    pub items: Vec<ItemNode>,
    pub span: Range<usize>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct EnumDeclaration {
    pub name: IdentifierNode,
    pub annotations: Vec<AttributeNode>,
    pub items: Vec<ItemNode>,
    pub span: Range<usize>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct TraitDeclaration {
    pub name: IdentifierNode,
    pub annotations: Vec<AttributeNode>,
    pub parents: Vec<NamePathNode>,
    pub items: Vec<ItemNode>,
    pub span: Range<usize>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct WidgetDeclaration {
    pub name: IdentifierNode,
    pub annotations: Vec<AttributeNode>,
    pub items: Vec<ItemNode>,
    pub span: Range<usize>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct UsingStatement {
    pub path: NamePathNode,
    pub span: Range<usize>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct MicroDefinition {
    pub name: IdentifierNode,
    pub annotations: Vec<AttributeNode>,
    pub params: Vec<ParameterNode>,
    pub return_type: Option<String>,
    pub body: BlockNode,
    pub span: Range<usize>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct TypeFunctionDefinition {
    pub name: IdentifierNode,
    pub annotations: Vec<AttributeNode>,
    pub params: Vec<ParameterNode>,
    pub return_type: Option<String>,
    pub body: BlockNode,
    pub span: Range<usize>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct VariantDefinition {
    pub name: IdentifierNode,
    pub annotations: Vec<AttributeNode>,
    pub value: Option<ExpressionNode>,
    pub span: Range<usize>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct TemplateControlNode {
    pub items: Vec<ItemNode>,
    pub span: Range<usize>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct TemplateInterpolationNode {
    pub expr: ExpressionNode,
    pub span: Range<usize>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum StatementNode {
    Let(LetStatement),
    Expr(ExpressionStatement),
}

#[derive(Debug, Clone, PartialEq)]
pub struct LetStatement {
    pub annotations: Vec<AttributeNode>,
    pub is_mutable: bool,
    pub pattern: PatternNode,
    pub expr: ExpressionNode,
    pub span: Range<usize>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct ExpressionStatement {
    pub annotations: Vec<AttributeNode>,
    pub expr: ExpressionNode,
    pub semi: bool,
    pub span: Range<usize>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum ExpressionNode {
    Ident(IdentifierNode),
    Path(NamePathNode),
    Literal(LiteralExpressionNode),
    Bool(BooleanLiteralNode),
    Paren(ParenthesizedExpressionNode),
    Unary(UnaryExpressionNode),
    Binary(BinaryExpressionNode),
    Call(CallExpressionNode),
    Field(FieldExpressionNode),
    Index(IndexExpressionNode),
    If(IfExpressionNode),
    Match(MatchExpressionNode),
    Lambda(LambdaExpressionNode),
    Object(ObjectExpressionNode),
    Block(BlockNode),
    Loop(LoopExpressionNode),
    Return(ReturnExpressionNode),
    Break(BreakExpressionNode),
    Continue(ContinueExpressionNode),
    Yield(YieldExpressionNode),
    Raise(RaiseExpressionNode),
    Catch(CatchExpressionNode),
    Resume(ResumeExpressionNode),
}

#[derive(Debug, Clone, PartialEq)]
pub struct LiteralExpressionNode {
    pub value: String,
    pub span: Range<usize>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct BooleanLiteralNode {
    pub value: bool,
    pub span: Range<usize>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct ParenthesizedExpressionNode {
    pub expr: Box<ExpressionNode>,
    pub span: Range<usize>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct UnaryExpressionNode {
    pub op: DejavuTokenType,
    pub expr: Box<ExpressionNode>,
    pub span: Range<usize>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct BinaryExpressionNode {
    pub left: Box<ExpressionNode>,
    pub op: DejavuTokenType,
    pub right: Box<ExpressionNode>,
    pub span: Range<usize>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct CallExpressionNode {
    pub callee: Box<ExpressionNode>,
    pub args: Vec<ExpressionNode>,
    pub span: Range<usize>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct FieldExpressionNode {
    pub receiver: Box<ExpressionNode>,
    pub field: IdentifierNode,
    pub span: Range<usize>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct IndexExpressionNode {
    pub receiver: Box<ExpressionNode>,
    pub index: Box<ExpressionNode>,
    pub span: Range<usize>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct IfExpressionNode {
    pub pattern: Option<PatternNode>,
    pub condition: Box<ExpressionNode>,
    pub then_branch: BlockNode,
    pub else_branch: Option<BlockNode>,
    pub span: Range<usize>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct MatchExpressionNode {
    pub scrutinee: Box<ExpressionNode>,
    pub arms: Vec<MatchArmNode>,
    pub span: Range<usize>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct LambdaExpressionNode {
    pub params: Vec<ParameterNode>,
    pub return_type: Option<String>,
    pub body: BlockNode,
    pub span: Range<usize>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct ObjectExpressionNode {
    pub callee: Box<ExpressionNode>,
    pub block: BlockNode,
    pub span: Range<usize>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct LoopExpressionNode {
    pub label: Option<String>,
    pub pattern: Option<PatternNode>,
    pub condition: Option<Box<ExpressionNode>>,
    pub body: BlockNode,
    pub span: Range<usize>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct ReturnExpressionNode {
    pub expr: Option<Box<ExpressionNode>>,
    pub span: Range<usize>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct BreakExpressionNode {
    pub label: Option<String>,
    pub expr: Option<Box<ExpressionNode>>,
    pub span: Range<usize>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct ContinueExpressionNode {
    pub label: Option<String>,
    pub span: Range<usize>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct YieldExpressionNode {
    pub expr: Option<Box<ExpressionNode>>,
    pub yield_from: bool,
    pub span: Range<usize>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct RaiseExpressionNode {
    pub expr: Box<ExpressionNode>,
    pub span: Range<usize>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct CatchExpressionNode {
    pub return_type: Option<NamePathNode>,
    pub expr: Box<ExpressionNode>,
    pub arms: Vec<MatchArmNode>,
    pub span: Range<usize>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct ResumeExpressionNode {
    pub expr: Option<Box<ExpressionNode>>,
    pub span: Range<usize>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum PatternNode {
    Variable(VariablePatternNode),
    Wildcard(WildcardPatternNode),
    Literal(LiteralPatternNode),
    Tuple(TuplePatternNode),
    Array(ArrayPatternNode),
    Object(ObjectPatternNode),
    Or(OrPatternNode),
    Type(TypePatternNode),
    Class(ClassPatternNode),
}

#[derive(Debug, Clone, PartialEq)]
pub struct VariablePatternNode {
    pub name: IdentifierNode,
    pub span: Range<usize>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct WildcardPatternNode {
    pub span: Range<usize>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct LiteralPatternNode {
    pub value: String,
    pub span: Range<usize>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct TuplePatternNode {
    pub items: Vec<PatternNode>,
    pub span: Range<usize>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct ArrayPatternNode {
    pub items: Vec<PatternNode>,
    pub span: Range<usize>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct ObjectPatternNode {
    pub props: Vec<(IdentifierNode, PatternNode)>,
    pub span: Range<usize>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct OrPatternNode {
    pub patterns: Vec<PatternNode>,
    pub span: Range<usize>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct TypePatternNode {
    pub name: NamePathNode,
    pub span: Range<usize>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct ClassPatternNode {
    pub name: NamePathNode,
    pub fields: Vec<(IdentifierNode, PatternNode)>,
    pub span: Range<usize>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct MatchArmNode {
    pub pattern: PatternNode,
    pub guard: Option<Box<ExpressionNode>>,
    pub body: Box<ExpressionNode>,
    pub span: Range<usize>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct EffectDefinition {
    pub name: IdentifierNode,
    pub annotations: Vec<AttributeNode>,
    pub items: Vec<ItemNode>,
    pub span: Range<usize>,
}

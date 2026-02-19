use oak_core::{ElementType, UniversalElementRole};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum DejavuElementType {
    Root,
    Eof,
    Whitespace,
    LineComment,
    BlockComment,
    Error,

    // Nodes
    Attribute,
    NamePath,
    Type,
    Namespace,
    Class,
    ParameterList,
    BlockExpression,
    IdentifierExpression,
    PathExpression,

    // Statements
    Statement,
    LetStatement,
    ExprStatement,

    // Expressions
    Expression,
    CallExpression,
    Param,
    ReturnExpression,
    LiteralExpression,
    BooleanLiteral,
    BinaryExpression,
    UnaryExpression,
    ParenthesizedExpression,
    IndexExpression,
    FieldExpression,
    IfExpression,
    MatchExpression,
    LoopExpression,
    BreakExpression,
    ContinueExpression,
    YieldExpression,
    RaiseExpression,
    CatchExpression,
    ResumeExpression,
    ApplyBlock,
    ObjectExpression,

    // Definitions
    Micro,
    Mezzo,
    Macro,
    Struct,
    Enum,
    Enums,
    Trait,
    Impl,
    Field,
    Method,
    Variant,
    Flags,
    Widget,
    EffectDefinition,
    UsingStatement,

    // Template
    TemplateText,
    TemplateControl,
    Interpolation,
    TemplateComment,

    // Others
    Pattern,
    ArgList,
    GenericParameterList,
    GenericArgumentList,
    MatchArm,
    Parameter,
    AnonymousClass,
}

impl ElementType for DejavuElementType {
    type Role = UniversalElementRole;

    fn role(&self) -> Self::Role {
        match self {
            Self::Root => UniversalElementRole::Root,
            Self::Error => UniversalElementRole::Error,
            _ => UniversalElementRole::None,
        }
    }
}

impl From<crate::lexer::token_type::DejavuTokenType> for DejavuElementType {
    fn from(token: crate::lexer::token_type::DejavuTokenType) -> Self {
        match token {
            crate::lexer::token_type::DejavuTokenType::Eof => Self::Eof,
            crate::lexer::token_type::DejavuTokenType::Whitespace => Self::Whitespace,
            crate::lexer::token_type::DejavuTokenType::Error => Self::Error,
            _ => Self::Error, // Map other tokens to Error or a generic Token wrapper if needed
        }
    }
}

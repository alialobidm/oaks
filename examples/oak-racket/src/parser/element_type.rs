use crate::lexer::TokenType;
use oak_core::language::UniversalElementRole;

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum ElementType {
    // 表达式
    Expression,

    // 语句
    Statement,

    // 循环
    For,

    // 列表推导
    ListComprehension,

    // 其他
    Block,
    Identifier,
    Number,
    String,
    Boolean,
    BinaryExpression,
    UnaryExpression,
    Call,
    Index,
    Tuple,
    List,
    Map,

    // 特殊元素
    Eof,
}

impl oak_core::language::ElementType for ElementType {
    type Role = UniversalElementRole;

    fn role(&self) -> Self::Role {
        match self {
            ElementType::Expression => UniversalElementRole::Expression,
            ElementType::Statement => UniversalElementRole::Statement,
            ElementType::For => UniversalElementRole::Statement,
            ElementType::ListComprehension => UniversalElementRole::Expression,
            ElementType::Block => UniversalElementRole::Container,
            ElementType::Identifier => UniversalElementRole::Reference,
            ElementType::Number | ElementType::String | ElementType::Boolean => UniversalElementRole::Value,
            ElementType::BinaryExpression | ElementType::UnaryExpression => UniversalElementRole::Expression,
            ElementType::Call => UniversalElementRole::Call,
            ElementType::Index => UniversalElementRole::Expression,
            ElementType::Tuple | ElementType::List | ElementType::Map => UniversalElementRole::Container,
            ElementType::Eof => UniversalElementRole::None,
        }
    }
}

impl From<TokenType> for ElementType {
    fn from(token_type: TokenType) -> Self {
        match token_type {
            TokenType::For => ElementType::For,
            TokenType::In => ElementType::Expression,
            TokenType::Identifier => ElementType::Identifier,
            TokenType::Number => ElementType::Number,
            TokenType::String => ElementType::String,
            TokenType::Boolean => ElementType::Boolean,
            TokenType::LParen | TokenType::RParen | TokenType::LBracket | TokenType::RBracket | TokenType::LBrace | TokenType::RBrace | TokenType::Comma | TokenType::Dot | TokenType::Colon | TokenType::Semicolon => ElementType::Expression,
            TokenType::Plus
            | TokenType::Minus
            | TokenType::Multiply
            | TokenType::Divide
            | TokenType::Modulo
            | TokenType::Equals
            | TokenType::NotEquals
            | TokenType::LessThan
            | TokenType::LessThanOrEqual
            | TokenType::GreaterThan
            | TokenType::GreaterThanOrEqual
            | TokenType::And
            | TokenType::Or
            | TokenType::Not => ElementType::Expression,
            TokenType::Comment | TokenType::Whitespace => ElementType::Expression,
            TokenType::Require | TokenType::Provide | TokenType::Struct | TokenType::Class | TokenType::Match | TokenType::WithHandlers | TokenType::Raise => ElementType::Expression,
            TokenType::Eof => ElementType::Eof,
        }
    }
}

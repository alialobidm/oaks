use oak_core::language::UniversalTokenRole;

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum TokenType {
    // 关键字
    For,
    In,
    Require,
    Provide,
    Struct,
    Class,
    Match,
    WithHandlers,
    Raise,

    // 标识符
    Identifier,

    // 字面量
    Number,
    String,
    Boolean,

    // 标点符号
    LParen,
    RParen,
    LBracket,
    RBracket,
    LBrace,
    RBrace,
    Comma,
    Dot,
    Colon,
    Semicolon,

    // 运算符
    Plus,
    Minus,
    Multiply,
    Divide,
    Modulo,
    Equals,
    NotEquals,
    LessThan,
    LessThanOrEqual,
    GreaterThan,
    GreaterThanOrEqual,
    And,
    Or,
    Not,

    // 其他
    Comment,
    Whitespace,
    Eof,
}

impl oak_core::language::TokenType for TokenType {
    type Role = UniversalTokenRole;

    const END_OF_STREAM: Self = TokenType::Eof;

    fn role(&self) -> Self::Role {
        match self {
            TokenType::For | TokenType::In | TokenType::Require | TokenType::Provide | TokenType::Struct | TokenType::Class | TokenType::Match | TokenType::WithHandlers | TokenType::Raise => UniversalTokenRole::Keyword,
            TokenType::Identifier => UniversalTokenRole::Name,
            TokenType::Number | TokenType::String | TokenType::Boolean => UniversalTokenRole::Literal,
            TokenType::LParen | TokenType::RParen | TokenType::LBracket | TokenType::RBracket | TokenType::LBrace | TokenType::RBrace | TokenType::Comma | TokenType::Dot | TokenType::Colon | TokenType::Semicolon => UniversalTokenRole::Punctuation,
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
            | TokenType::Not => UniversalTokenRole::Operator,
            TokenType::Comment => UniversalTokenRole::Comment,
            TokenType::Whitespace => UniversalTokenRole::Whitespace,
            TokenType::Eof => UniversalTokenRole::Eof,
        }
    }
}

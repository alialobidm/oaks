use crate::lexer::DejavuKeywords;
use oak_core::{Token, TokenType, UniversalTokenRole};

/// Alias for `Token<DejavuTokenType>`.
pub type DejavuToken = Token<DejavuTokenType>;
pub type DejavuSyntaxKind = DejavuTokenType;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum DejavuTokenType {
    Eof,
    Whitespace,
    Error,

    // Literals
    IntegerLiteral,
    FloatLiteral,
    BoolLiteral,
    StringLiteral,
    CharLiteral,
    Identifier,
    Label,

    // Comments
    LineComment,
    BlockComment,

    // Keywords
    Keyword(DejavuKeywords),

    // Operators & Punctuation
    Ampersand,    // &
    AndAnd,       // &&
    Arrow,        // ->
    At,           // @
    Bang,         // !
    Bolt,         // ↯
    Caret,        // ^
    Colon,        // :
    ColonColon,   // ::
    ColonEq,      // :=
    Comma,        // ,
    Dollar,       // $
    Dot,          // .
    Eq,           // =
    EqEq,         // ==
    GreaterEq,    // >=
    GreaterThan,  // >
    LeftBrace,    // {
    LeftBracket,  // [
    LeftParen,    // (
    LeftShift,    // <<
    LessEq,       // <=
    LessThan,     // <
    Minus,        // -
    MinusEq,      // -=
    MinusMinus,   // --
    NotEq,        // !=
    OrOr,         // ||
    Percent,      // %
    PercentEq,    // %=
    Pipe,         // |
    PipeGreater,  // |>
    Plus,         // +
    PlusEq,       // +=
    PlusPlus,     // ++
    Question,     // ?
    RightBrace,   // }
    RightBracket, // ]
    RightParen,   // )
    RightShift,   // >>
    Semicolon,    // ;
    Slash,        // /
    SlashEq,      // /=
    Star,         // *
    StarEq,       // *=
    Tilde,        // ~
    Underscore,   // _

    // Template specific
    InterpolationStart,
    InterpolationEnd,
    StringPart,
    TemplateControlStart,
    TemplateControlEnd,
    TemplateCommentStart,
    TemplateCommentEnd,
}

impl TokenType for DejavuTokenType {
    type Role = UniversalTokenRole;
    const END_OF_STREAM: Self = Self::Eof;

    fn is_ignored(&self) -> bool {
        matches!(self, Self::Whitespace | Self::LineComment | Self::BlockComment)
    }

    fn role(&self) -> Self::Role {
        match self {
            Self::Whitespace => UniversalTokenRole::Whitespace,
            Self::LineComment | Self::BlockComment => UniversalTokenRole::Comment,
            Self::Eof => UniversalTokenRole::Eof,
            Self::Error => UniversalTokenRole::Error,
            Self::Keyword(_) => UniversalTokenRole::Keyword,
            Self::Identifier => UniversalTokenRole::Name,
            Self::StringLiteral | Self::CharLiteral | Self::StringPart => UniversalTokenRole::Literal,
            Self::IntegerLiteral | Self::FloatLiteral | Self::BoolLiteral => UniversalTokenRole::Literal,
            _ => UniversalTokenRole::None,
        }
    }
}

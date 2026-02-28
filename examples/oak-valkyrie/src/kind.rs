/// Valkyrie syntax kinds
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum ValkyrieKeywords {
    /// `as`
    As,
    /// `break`
    Break,
    /// `class`
    Class,
    /// `continue`
    Continue,
    /// `def`
    Def,
    /// `do`
    Do,
    /// `effect`
    Effect,
    /// `else`
    Else,
    /// `enum`
    Enums,
    /// `export`
    Export,
    /// `extends`
    Extends,
    /// `false`
    False,
    /// `final`
    Final,
    /// `flags`
    Flags,
    /// `fn`
    Fn,
    /// `for`
    For,
    /// `if`
    If,
    /// `impl`
    Impl,
    /// `import`
    Import,
    /// `in`
    In,
    /// `let`
    Let,
    /// `loop`
    Loop,
    /// `match`
    Match,
    /// `mezzo`
    Mezzo,
    /// `micro`
    Micro,
    /// `mut`
    Mut,
    /// `namespace`
    Namespace,
    /// `new`
    New,
    /// `private`
    Private,
    /// `protected`
    Protected,
    /// `public`
    Public,
    /// `raise`
    Raise,
    /// `return`
    Return,
    /// `self`
    SelfValue,
    /// `static`
    Static,
    /// `struct`
    Struct,
    /// `super`
    Super,
    /// `trait`
    Trait,
    /// `true`
    True,
    /// `type`
    Type,
    /// `use`
    Use,
    /// `using`
    Using,
    /// `var`
    Var,
    /// `variant`
    Variant,
    /// `virtual`
    Virtual,
    /// `widget`
    Widget,
    /// `while`
    While,
    /// `yield`
    Yield,
}

/// Valkyrie syntax kinds (terminal and non-terminal)
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum ValkyrieSyntaxKind {
    // Keywords
    Keyword(ValkyrieKeywords),
    // Literals
    IntegerLiteral,
    FloatLiteral,
    StringLiteral,
    CharLiteral,
    // Identifiers
    Identifier,
    NamePath,
    // Operators
    Plus,
    Minus,
    Star,
    Slash,
    Percent,
    Eq,
    EqEq,
    NotEq,
    LessThan,
    LessEq,
    GreaterThan,
    GreaterEq,
    AndAnd,
    OrOr,
    Ampersand,
    Pipe,
    Caret,
    LeftShift,
    RightShift,
    Bang,
    Tilde,
    // Delimiters
    LeftParen,
    RightParen,
    LeftBrace,
    RightBrace,
    LeftBracket,
    RightBracket,
    Comma,
    Semicolon,
    Colon,
    ColonColon,
    Dot,
    DotDot,
    Arrow,
    // Whitespace and trivia
    Whitespace,
    Newline,
    LineComment,
    BlockComment,
    // End of file
    Eof,
    // Items
    Namespace,
    Class,
    Flags,
    Enums,
    Trait,
    Widget,
    Using,
    Micro,
    Mezzo,
    Variant,
    Effect,
    // Statements
    LetStatement,
    ExpressionStatement,
    // Expressions
    Literal,
    Bool,
    Binary,
    Unary,
    Call,
    Field,
    Index,
    Paren,
    BlockExpression,
    Lambda,
    Object,
    AnonymousClass,
    If,
    Match,
    Loop,
    Return,
    Break,
    Continue,
    Yield,
    Raise,
    Catch,
    // Patterns
    PatternWildcard,
    PatternVariable,
    PatternLiteral,
    PatternType,
    PatternClass,
    PatternElse,
    // Attributes
    Attribute,
    // Types
    TypeNamed,
    TypeGeneric,
    TypeTuple,
    TypeFunction,
    TypeOptional,
    // Parameters
    GenericParam,
    Param,
    Field,
    // Match arm
    MatchArm,
}

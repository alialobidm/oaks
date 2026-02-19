#[derive(Debug, PartialEq, Eq, Clone)]
pub enum Expression {
    // 基本表达式
    Identifier(String),
    Number(String),
    String(String),
    Boolean(bool),

    // 复合表达式
    BinaryExpression(Box<BinaryExpression>),
    UnaryExpression(Box<UnaryExpression>),
    Call(Box<Call>),
    Index(Box<Index>),
    Tuple(Vec<Expression>),
    List(Vec<Expression>),
    Map(Vec<(Expression, Expression)>),

    // 循环表达式
    For(Box<For>),

    // 列表推导
    ListComprehension(Box<ListComprehension>),
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct BinaryExpression {
    pub left: Expression,
    pub operator: String,
    pub right: Expression,
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct UnaryExpression {
    pub operator: String,
    pub expression: Expression,
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Call {
    pub function: Expression,
    pub arguments: Vec<Expression>,
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Index {
    pub expression: Expression,
    pub index: Expression,
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct For {
    pub variable: String,
    pub iterable: Expression,
    pub body: Vec<Expression>,
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct ListComprehension {
    pub expression: Expression,
    pub variable: String,
    pub iterable: Expression,
    pub condition: Option<Expression>,
}

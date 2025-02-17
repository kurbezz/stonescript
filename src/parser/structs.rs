#[derive(Debug, PartialEq)]
pub enum Value {
    String(String),
    Decimal(f64),
    Integer(i64),
}


#[derive(Debug, PartialEq)]
pub enum BinaryOperator {
    Add,
    Subtract,
    Multiply,
    Divide,
    Modulo,
    Equal,
    NotEqual,
    And,
    Or,
    Greater,
    Less,
    GreaterEqual,
    LessEqual,
}


#[derive(Debug, PartialEq)]
pub enum UnaryOperator {
    Negate,
    Increment,
    Decrement,
}


#[derive(Debug, PartialEq)]
pub enum Expression {
    Value(Value),
    Identifier(String),
    Binary {
        left: Box<Expression>,
        operator: BinaryOperator,
        right: Box<Expression>,
    },
    Unary {
        operator: UnaryOperator,
        right: Box<Expression>,
    },
    Assignment {
        name: String,
        value: Box<Expression>,
    },
    Condition {
        condition: Box<Expression>,
        then_branch: Box<Expression>,
        else_branch: Option<Box<Expression>>,
    },
    Function {
        name: String,
        parameters: Vec<String>,
        body: Box<Expression>,
    },
    Block {
        statements: Vec<Expression>,
    },
    Nope,
}

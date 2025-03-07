#[derive(Debug, PartialEq, Clone)]
pub enum Token {
    NewLineIndent(u32),
    EndLine,

    If,
    Else,
    ElseIf,

    Comment(String),
    CommentBlock(String),

    Equal,
    NotEqual,
    And,
    Or,
    Greater,
    Less,
    GreaterEqual,
    LessEqual,

    Add,
    Subtract,
    Multiply,
    Divide,
    Increment,
    Decrement,
    Modulo,

    ParenthesisOpen,
    ParenthesisClose,

    SquareBracketOpen,
    SquareBracketClose,

    Comma,

    AsciiBlock(String),

    Identifier(String),

    String(String),
}

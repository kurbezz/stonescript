#[derive(Debug, PartialEq, Clone)]
pub enum TokenType {
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

#[derive(Debug, PartialEq, Clone)]
pub struct Token {
    pub token_type: TokenType,
    pub line: usize,
    pub column: usize,
}

impl PartialEq<TokenType> for Token {
    fn eq(&self, other: &TokenType) -> bool {
        self.token_type == *other
    }
}

use crate::lexer::tokens::{Token, TokenType};

#[derive(Debug, Clone)]
pub struct TokenIterator<'a> {
    iterator: std::iter::Peekable<std::slice::Iter<'a, Token>>,
}

impl<'a> TokenIterator<'a> {
    pub fn new(iterator: std::iter::Peekable<std::slice::Iter<'a, Token>>) -> Self {
        TokenIterator { iterator }
    }

    pub fn peek(&mut self) -> Option<&Token> {
        self.iterator.peek().map(|v| &**v)
    }

    pub fn next(&mut self) -> Option<&Token> {
        self.iterator.next()
    }

    pub fn peek_some(&mut self) -> Token {
        match self.peek() {
            Some(token) => token.clone(),
            None => panic!("Expected token, found None"),
        }
    }

    pub fn next_some(&mut self) -> Token {
        match self.next() {
            Some(token) => token.clone(),
            None => panic!("Expected token, found None"),
        }
    }

    pub fn next_expected(&mut self, expected: TokenType) {
        match self.next() {
            Some(token) => {
                if token.token_type != expected {
                    panic!("Expected {:?}, found {:?}", expected, token.token_type);
                }
            }
            None => panic!("Expected {:?}, found None", expected),
        }
    }
}

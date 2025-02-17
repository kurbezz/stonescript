use crate::lexer::tokens::Token;

pub type TokenIterator<'a> = std::iter::Peekable<std::slice::Iter<'a, Token>>;

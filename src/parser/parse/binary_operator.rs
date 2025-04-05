use crate::lexer::tokens::{Token, TokenType};
use crate::parser::structs::BinaryOperator;


pub fn parse_binary_operator(token: &Token) -> BinaryOperator {
    match token.token_type {
        TokenType::Add => BinaryOperator::Add,
        TokenType::Subtract => BinaryOperator::Subtract,
        TokenType::Multiply => BinaryOperator::Multiply,
        TokenType::Divide => BinaryOperator::Divide,
        TokenType::Modulo => BinaryOperator::Modulo,
        TokenType::Equal => BinaryOperator::Equal,
        TokenType::NotEqual => BinaryOperator::NotEqual,
        TokenType::And => BinaryOperator::And,
        TokenType::Or => BinaryOperator::Or,
        TokenType::Greater => BinaryOperator::Greater,
        TokenType::Less => BinaryOperator::Less,
        TokenType::GreaterEqual => BinaryOperator::GreaterEqual,
        TokenType::LessEqual => BinaryOperator::LessEqual,
        _ => panic!("Expected binary operator"),
    }
}

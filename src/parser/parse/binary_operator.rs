use crate::lexer::tokens::Token;
use crate::parser::structs::BinaryOperator;


pub fn parse_binary_operator(token: &Token) -> BinaryOperator {
    match token {
        Token::Add => BinaryOperator::Add,
        Token::Subtract => BinaryOperator::Subtract,
        Token::Multiply => BinaryOperator::Multiply,
        Token::Divide => BinaryOperator::Divide,
        Token::Modulo => BinaryOperator::Modulo,
        Token::Equal => BinaryOperator::Equal,
        Token::NotEqual => BinaryOperator::NotEqual,
        Token::And => BinaryOperator::And,
        Token::Or => BinaryOperator::Or,
        Token::Greater => BinaryOperator::Greater,
        Token::Less => BinaryOperator::Less,
        Token::GreaterEqual => BinaryOperator::GreaterEqual,
        Token::LessEqual => BinaryOperator::LessEqual,
        _ => panic!("Expected binary operator"),
    }
}

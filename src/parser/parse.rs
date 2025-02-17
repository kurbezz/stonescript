use crate::lexer::tokens::Token;
use super::structs::{BinaryOperator, Expression, Value};


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

pub fn parse_expression_value(token: &Token) -> Expression {
    match token {
        Token::Identifier(name) => {
            if let Ok(value) = name.parse::<i64>() {
                Expression::Value(Value::Integer(value))
            } else if let Ok(value) = name.parse::<f64>() {
                Expression::Value(Value::Decimal(value))
            } else {
                Expression::Identifier(name.clone())
            }
        },
        Token::String(value) => Expression::Value(Value::String(value.clone())),
        _ => panic!("Expected identifier"),
    }
}

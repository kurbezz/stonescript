use crate::{lexer::tokens::{Token, TokenType}, parser::structs::{Expression, Value}};


pub fn parse_expression_value(token: &Token) -> Expression {
    match &token.token_type {
        TokenType::Identifier(name) => {
            if let Ok(value) = name.parse::<i64>() {
                Expression::Value(Value::Integer(value))
            } else if let Ok(value) = name.parse::<f64>() {
                Expression::Value(Value::Decimal(value))
            } else {
                Expression::Identifier(name.clone())
            }
        },
        TokenType::String(value) => Expression::Value(Value::String(value.clone())),
        _ => panic!("Expected identifier"),
    }
}

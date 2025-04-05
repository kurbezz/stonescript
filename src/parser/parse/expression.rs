use crate::{lexer::tokens::TokenType, parser::{structs::Expression, types::TokenIterator}};

use super::{assigment::parse_assigment_expression, condition::parse_condition_expression, function::parse_function_expression, primary::parse_primary_expression};


pub fn parse_expression<'a>(mut iterator: TokenIterator<'a>) -> (TokenIterator<'a>, Option<Expression>) {
    match iterator.peek() {
        Some(token) => {
            match &token.token_type {
                TokenType::NotEqual | TokenType::ParenthesisOpen => {
                    let (new_iter, expr) = parse_primary_expression(iterator);
                    (new_iter, Some(expr))
                },
                TokenType::Identifier(name) => {
                    match name.as_str() {
                        "var" => parse_assigment_expression(iterator),
                        "func" => {
                            let (new_iter, expr) = parse_function_expression(iterator);
                            (new_iter, Some(expr))
                        },
                        _ => {
                            let (new_iter, expr) = parse_primary_expression(iterator);
                            (new_iter, Some(expr))
                        }
                    }
                },
                TokenType::If => parse_condition_expression(iterator),
                _ => panic!("Unexpected identifier: {:?}", token),
            }
        },
        None => todo!(),
    }
}

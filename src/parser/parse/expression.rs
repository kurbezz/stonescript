use crate::{lexer::tokens::Token, parser::{structs::Expression, types::TokenIterator}};

use super::{assigment::parse_assigment_expression, condition::parse_condition_expression, primary::parse_primary_expression};


pub fn parse_expression<'a>(mut iterator: TokenIterator<'a>) -> (TokenIterator<'a>, Option<Expression>) {
    match iterator.peek() {
        Some(token) => {
            match token {
                Token::NotEqual | Token::ParenthesisOpen => {
                    let (new_iter, expr) = parse_primary_expression(iterator);
                    (new_iter, Some(expr))
                },
                Token::Identifier(name) => {
                    match name.as_str() {
                        "var" => parse_assigment_expression(iterator),
                        "func" => parse_assigment_expression(iterator),
                        _ => {
                            let (new_iter, expr) = parse_primary_expression(iterator);
                            (new_iter, Some(expr))
                        }
                    }
                },
                Token::If => parse_condition_expression(iterator),
                _ => panic!("Unexpected identifier"),
            }
        },
        None => todo!(),
    }
}

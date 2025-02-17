use crate::{lexer::tokens::Token, parser::{structs::Expression, types::TokenIterator}};

use super::primary::parse_primary_expression;


pub fn parse_assigment_expression<'a>(
    mut iterator: TokenIterator<'a>,
) -> (TokenIterator<'a>, Option<Expression>) {
    let name = match iterator.next() {
        Some(Token::Identifier(name)) => name.clone(),
        _ => panic!("Expected identifier"),
    };

    match iterator.next() {
        Some(Token::Equal) => {},
        _ => panic!("Expected equal sign"),
    };

    let (new_iter, value) = parse_primary_expression(iterator);

    let expression = Expression::Assignment {
        name,
        value: Box::new(value),
    };

    (new_iter, Some(expression))
}

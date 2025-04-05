use crate::{lexer::tokens::TokenType, parser::{structs::Expression, types::TokenIterator}};

use super::primary::parse_primary_expression;


pub fn parse_assigment_expression<'a>(
    mut iterator: TokenIterator<'a>,
) -> (TokenIterator<'a>, Option<Expression>) {
    let name = match iterator.next_some().token_type {
        TokenType::Identifier(name) => name.clone(),
        _ => panic!("Expected identifier"),
    };

    iterator.next_expected(TokenType::Equal);

    let (new_iter, value) = parse_primary_expression(iterator);

    let expression = Expression::Assignment {
        name,
        value: Box::new(value),
    };

    (new_iter, Some(expression))
}

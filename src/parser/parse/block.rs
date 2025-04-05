use crate::{lexer::tokens::Token, parser::{structs::Expression, types::TokenIterator}};

use super::expression::parse_expression;


pub fn parse_expression_block(tokens: &[Token]) -> Expression {
    let mut iterator = TokenIterator::new(tokens.iter().peekable());

    let mut expressions = vec![];

    while iterator.peek().is_some() {
        let (new_iter, expression) = parse_expression(iterator);

        iterator = new_iter;

        match expression {
            Some(expression) => expressions.push(expression),
            None => continue,
        }
    }

    Expression::Block {
        statements: expressions,
    }
}
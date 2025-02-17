use crate::parser::{structs::Expression, types::TokenIterator};


pub fn parse_condition_expression<'a>(
    mut iterator: TokenIterator<'a>,
) -> (TokenIterator<'a>, Option<Expression>) {
    todo!()
}

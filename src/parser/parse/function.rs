use crate::{lexer::tokens::TokenType, parser::{structs::Expression, types::TokenIterator}};

use super::expression::parse_expression;


pub fn parse_function_expression<'a>(
    mut iterator: TokenIterator<'a>,
) -> (TokenIterator<'a>, Expression) {
    iterator.next_expected(TokenType::Identifier("func".to_string()));

    let name = match iterator.next_some().token_type {
        TokenType::Identifier(name) => name,
        _ => panic!("Expected identifier"),
    };

    let parameters = {
        iterator.next_expected(TokenType::ParenthesisOpen);

        let mut paramaters: Vec<String> = vec![];

        match iterator.peek_some().token_type {
            TokenType::ParenthesisClose => {
                iterator.next();
            }
            TokenType::Identifier(name) => {
                paramaters.push(name.to_string());
                iterator.next();
            }
            _ => panic!("Unexpected token"),
        }

        iterator.next_expected(TokenType::EndLine);

        paramaters
    };

    let indent = {
        match iterator.peek_some().token_type {
            TokenType::NewLineIndent(indent) => {
                indent
            }
            _ => panic!("Expected indent"),
        }
    };

    let body_statements = {
        let mut statements: Vec<Expression> = vec![];

        loop {
            match iterator.peek() {
                Some(token) => {
                    match token.token_type {
                        TokenType::NewLineIndent(i) => {
                            if i != indent {
                                break;
                            }

                            iterator.next();

                            let (new_iter, statement) = parse_expression(iterator);
                            iterator = new_iter;
                            statements.push(statement.unwrap());
                        }
                        _ => {
                            panic!("Unexpected token");
                        }
                    }
                }
                None => break,
            }
        }

        statements
    };

    (
        iterator,
        Expression::Function {
            name,
            parameters,
            body: Box::new(Expression::Block { statements: body_statements }),
        }
)
}

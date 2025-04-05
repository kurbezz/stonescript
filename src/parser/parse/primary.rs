use crate::{lexer::tokens::{Token, TokenType}, parser::{structs::{Expression, UnaryOperator}, types::TokenIterator}};

use super::{binary_operator::parse_binary_operator, value::parse_expression_value};


const LOW_LEVEL_OPERATORS: [TokenType; 2] = [
    TokenType::Add,
    TokenType::Subtract,
];

const HIGH_LEVEL_OPERATORS: [TokenType; 2] = [
    TokenType::Multiply,
    TokenType::Divide,
];


pub fn parse_primary_split_by_operators(
    tokens: &[&Token],
    operators: &[TokenType],
 ) -> Option<Expression> {
    for operator in operators {
        let mut level = 0;

        for (index, token) in tokens.iter().enumerate() {
            match token.token_type {
                TokenType::ParenthesisOpen => {
                    level += 1;
                },
                TokenType::ParenthesisClose => {
                    level -= 1;
                },
                _ => {},
            }

            if **token == *operator && level == 0 {
                let left = int_parse_primary_expression(&tokens[0..index]);
                let operator = parse_binary_operator(token);
                let right = int_parse_primary_expression(&tokens[index + 1..]);

                return Some(Expression::Binary {
                    left: Box::new(left),
                    operator,
                    right: Box::new(right)
                });
            }
        }
    }

    None
}


pub fn parse_primary_expression_remove_parenthesis(
    tokens: &[&Token],
) -> Option<Expression> {
    let mut level = 0;

    let mut first_level_1 = 0;
    let mut last_level_0 = 0;

    for (index, token) in tokens.iter().enumerate() {
        match token.token_type {
            TokenType::ParenthesisOpen => {
                level += 1;

                if level == 1 {
                    first_level_1 = index;
                }
            },
            TokenType::ParenthesisClose => {
                level -= 1;

                if level == 0 {
                    last_level_0 = index;
                }
            },
            _ => {},
        }
    }

    if first_level_1 == 0 && last_level_0 == tokens.len() - 1 {
        return Some(int_parse_primary_expression(&tokens[1..tokens.len() - 1]));
    }

    if *tokens[0] == TokenType::NotEqual && first_level_1 == 1 && last_level_0 == tokens.len() - 1 {
        return Some(Expression::Unary {
            operator: UnaryOperator::Negate,
            right: Box::new(int_parse_primary_expression(&tokens[2..tokens.len() - 1])),
        });
    }

    None
}


pub fn int_parse_primary_expression(
    tokens: &[&Token],
) -> Expression {
    match tokens.len() {
        0 => Expression::Nope,
        1 => {
            match tokens[0].token_type {
                TokenType::Identifier(_) => {
                    parse_expression_value(tokens[0])
                },
                _ => panic!("Expected identifier"),
            }
        },
        2 => {
            match (tokens[0].clone().token_type, tokens[1].clone().token_type) {
                (TokenType::ParenthesisOpen, TokenType::ParenthesisClose) => Expression::Nope,
                (TokenType::Identifier(name), TokenType::Decrement | TokenType::Increment) => {
                    Expression::Unary {
                        operator: match tokens[1].token_type {
                            TokenType::Decrement => UnaryOperator::Decrement,
                            TokenType::Increment => UnaryOperator::Increment,
                            _ => panic!("Expected increment or decrement"),
                        },
                        right: Box::new(Expression::Identifier(name.to_string())),
                    }
                },
                (TokenType::NotEqual, TokenType::Identifier(name)) => {
                    Expression::Unary {
                        operator: UnaryOperator::Negate,
                        right: Box::new(Expression::Identifier(name.to_string())),
                    }
                },
                _ => panic!("Expected unary operator"),
            }
        },
        3 => {
            match (tokens[0].clone().token_type, tokens[1].clone().token_type, tokens[2].clone().token_type) {
                (TokenType::ParenthesisOpen, _, TokenType::ParenthesisClose) => {
                    int_parse_primary_expression(&tokens[1..2])
                },
                (
                    TokenType::Identifier(_),
                    TokenType::Add
                    | TokenType::Subtract
                    | TokenType::Multiply
                    | TokenType::Divide
                    | TokenType::Modulo
                    | TokenType::Equal
                    | TokenType::NotEqual
                    | TokenType::And
                    | TokenType::Or
                    | TokenType::Greater
                    | TokenType::Less
                    | TokenType::GreaterEqual
                    | TokenType::LessEqual,
                    TokenType::Identifier(_)
                ) => {
                    let left = int_parse_primary_expression(&tokens[0..1]);
                    let operator = parse_binary_operator(&tokens[1]);
                    let right = int_parse_primary_expression(&tokens[2..3]);

                    Expression::Binary {
                        left: Box::new(left),
                        operator,
                        right: Box::new(right)
                    }
                },
                _ => panic!("Unexpected expression"),
            }
        },
        _ => {
            if let Some(expression) = parse_primary_split_by_operators(tokens, LOW_LEVEL_OPERATORS.as_ref()) {
                return expression;
            }

            if let Some(expression) = parse_primary_split_by_operators(tokens, HIGH_LEVEL_OPERATORS.as_ref()) {
                return expression;
            }

            if let Some(expression) = parse_primary_expression_remove_parenthesis(tokens) {
                return expression;
            }

            panic!("Unexpected expression");
        }
    }
}


pub fn parse_primary_expression<'a>(
    mut iterator: TokenIterator<'a>,
) -> (TokenIterator<'a>, Expression) {
    let mut tokens: Vec<Token> = vec![];

    while let Some(token) = iterator.next() {
        match token.token_type {
            TokenType::EndLine => {
                break;
            },
            _ => {
                tokens.push(token.clone());
            },
        }
    }

    let expression = int_parse_primary_expression(&tokens.iter().collect::<Vec<_>>());

    (iterator, expression)
}

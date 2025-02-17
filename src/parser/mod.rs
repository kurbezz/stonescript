pub mod structs;
pub mod parse;

use crate::lexer::tokens::Token;
use parse::parse_binary_operator;
use structs::{Expression, UnaryOperator};

type Iterator<'a> = std::iter::Peekable<std::slice::Iter<'a, Token>>;


#[derive(Debug)]
pub struct SyntaxTree {
    pub root: structs::Expression,
}


const LOW_LEVEL_OPERATORS: [Token; 2] = [
    Token::Add,
    Token::Subtract,
];

const HIGH_LEVEL_OPERATORS: [Token; 2] = [
    Token::Multiply,
    Token::Divide,
];


fn parse_primary_split_by_operators(
    tokens: &[&Token],
    operators: &[Token],
 ) -> Option<Expression> {
    for operator in operators {
        let mut level = 0;

        for (index, token) in tokens.iter().enumerate() {
            match token {
                Token::ParenthesisOpen => {
                    level += 1;
                },
                Token::ParenthesisClose => {
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


fn parse_primary_expression_remove_parenthesis(
    tokens: &[&Token],
) -> Option<Expression> {
    let mut level = 0;

    let mut first_level_1 = 0;
    let mut last_level_0 = 0;

    for (index, token) in tokens.iter().enumerate() {
        match token {
            Token::ParenthesisOpen => {
                level += 1;

                if level == 1 {
                    first_level_1 = index;
                }
            },
            Token::ParenthesisClose => {
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

    if *tokens[0] == Token::NotEqual && first_level_1 == 1 && last_level_0 == tokens.len() - 1 {
        return Some(Expression::Unary {
            operator: UnaryOperator::Negate,
            right: Box::new(int_parse_primary_expression(&tokens[2..tokens.len() - 1])),
        });
    }

    None
}


fn int_parse_primary_expression(
    tokens: &[&Token],
) -> Expression {
    match tokens.len() {
        0 => Expression::Nope,
        1 => {
            match tokens[0] {
                Token::Identifier(_) => {
                    parse::parse_expression_value(tokens[0])
                },
                _ => panic!("Expected identifier"),
            }
        },
        2 => {
            match (tokens[0], tokens[1]) {
                (Token::ParenthesisOpen, Token::ParenthesisClose) => Expression::Nope,
                (Token::Identifier(name), Token::Decrement | Token::Increment) => {
                    Expression::Unary {
                        operator: match tokens[1] {
                            Token::Decrement => UnaryOperator::Decrement,
                            Token::Increment => UnaryOperator::Increment,
                            _ => panic!("Expected increment or decrement"),
                        },
                        right: Box::new(Expression::Identifier(name.to_string())),
                    }
                },
                (Token::NotEqual, Token::Identifier(name)) => {
                    Expression::Unary {
                        operator: UnaryOperator::Negate,
                        right: Box::new(Expression::Identifier(name.to_string())),
                    }
                },
                _ => panic!("Expected unary operator"),
            }
        },
        3 => {
            match (tokens[0], tokens[1], tokens[2]) {
                (Token::ParenthesisOpen, _, Token::ParenthesisClose) => {
                    int_parse_primary_expression(&tokens[1..2])
                },
                (
                    Token::Identifier(_),
                    Token::Add
                    | Token::Subtract
                    | Token::Multiply
                    | Token::Divide
                    | Token::Modulo
                    | Token::Equal
                    | Token::NotEqual
                    | Token::And
                    | Token::Or
                    | Token::Greater
                    | Token::Less
                    | Token::GreaterEqual
                    | Token::LessEqual,
                    Token::Identifier(_)
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


fn parse_primary_expression<'a>(
    mut iterator: Iterator<'a>,
) -> (Iterator<'a>, Expression) {
    let mut tokens = vec![];

    while let Some(token) = iterator.next() {
        match token {
            Token::EndLine => {
                break;
            },
            _ => {
                tokens.push(token);
            },
        }
    }

    let expression = int_parse_primary_expression(tokens.as_slice());

    (iterator, expression)
}


fn parse_assigment_expression<'a>(
    mut iterator: Iterator<'a>,
) -> (Iterator<'a>, Option<Expression>) {
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


fn parse_function_expression<'a>(
    mut iterator: Iterator<'a>,
) -> (Iterator<'a>, Option<Expression>) {
    todo!()
}


fn parse_condition_expression<'a>(
    mut iterator: Iterator<'a>,
) -> (Iterator<'a>, Option<Expression>) {
    todo!()
}


fn parse_expression<'a>(mut iterator: Iterator<'a>) -> (Iterator<'a>, Option<Expression>) {
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
                        "func" => parse_function_expression(iterator),
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


pub fn parse_expression_block(tokens: &[Token]) -> Expression {
    let mut iterator: Iterator<'_> = tokens.iter().peekable();

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


pub fn parse(tokens: &[Token]) -> SyntaxTree {
    SyntaxTree {
        root: parse_expression_block(tokens),
    }
}


#[cfg(test)]
mod tests {
    use crate::parser::structs::{BinaryOperator, Value};

    use super::*;

    #[test]
    fn test_expression_base() {
        let tokens = vec![
            Token::Identifier("2".to_string()),
            Token::Add,
            Token::Identifier("3".to_string()),
        ];

        let syntax_tree = parse(&tokens);

        assert_eq!(
            syntax_tree.root,
            Expression::Block {
                statements: vec![
                    Expression::Binary {
                        left: Box::new(Expression::Value(Value::Integer(2))),
                        operator: BinaryOperator::Add,
                        right: Box::new(Expression::Value(Value::Integer(3))),
                    }
                ],
            }
        );
    }

    #[test]
    fn test_expression_base_5() {
        let tokens = vec![
            Token::Identifier("1".to_string()),
            Token::Add,
            Token::Identifier("2".to_string()),
            Token::Subtract,
            Token::Identifier("3".to_string()),
        ];

        let syntax_tree = parse(&tokens);

        assert_eq!(
            syntax_tree.root,
            Expression::Block {
                statements: vec![
                    Expression::Binary {
                        left: Box::new(Expression::Value(Value::Integer(1))),
                        operator: BinaryOperator::Add,
                        right: Box::new(Expression::Binary {
                            left: Box::new(Expression::Value(Value::Integer(2))),
                            operator: BinaryOperator::Subtract,
                            right: Box::new(Expression::Value(Value::Integer(3))),
                        }),
                    }
                ]
            }
        )
    }

    #[test]
    fn test_expression_high_operator() {
        let tokens = vec![
            Token::Identifier("1".to_string()),
            Token::Multiply,
            Token::Identifier("2".to_string()),
            Token::Add,
            Token::Identifier("3".to_string()),
        ];

        let syntax_tree = parse(&tokens);

        assert_eq!(
            syntax_tree.root,
            Expression::Block {
                statements: vec![
                    Expression::Binary {
                        left: Box::new(Expression::Binary {
                            left: Box::new(Expression::Value(Value::Integer(1))),
                            operator: BinaryOperator::Multiply,
                            right: Box::new(Expression::Value(Value::Integer(2))),
                        }),
                        operator: BinaryOperator::Add,
                        right: Box::new(Expression::Value(Value::Integer(3))),
                    }
                ]
            }
        )
    }

    #[test]
    fn test_expression_high_operator_2() {
        let tokens = vec![
            Token::Identifier("1".to_string()),
            Token::Multiply,
            Token::Identifier("2".to_string()),
            Token::Add,
            Token::Identifier("3".to_string()),
            Token::Add,
            Token::Identifier("4".to_string()),
        ];

        let syntax_tree = parse(&tokens);

        assert_eq!(
            syntax_tree.root,
            Expression::Block {
                statements: vec![
                    Expression::Binary {
                        left: Box::new(Expression::Binary {
                            left: Box::new(Expression::Value(Value::Integer(1))),
                            operator: BinaryOperator::Multiply,
                            right: Box::new(Expression::Value(Value::Integer(2))),
                        }),
                        operator: BinaryOperator::Add,
                        right: Box::new(Expression::Binary {
                            left: Box::new(Expression::Value(Value::Integer(3))),
                            operator: BinaryOperator::Add,
                            right: Box::new(Expression::Value(Value::Integer(4))),
                        }),
                    }
                ]
            }
        )
    }

    #[test]
    fn test_expression_brackets() {
        let tokens = vec![
            Token::ParenthesisOpen,
            Token::ParenthesisClose,
        ];

        let syntax_tree = parse(&tokens);

        assert_eq!(
            syntax_tree.root,
            Expression::Block {
                statements: vec![
                    Expression::Nope,
                ]
            }
        )
    }

    #[test]
    fn test_expression_brackets_2() {
        let tokens = vec![
            Token::ParenthesisOpen,
            Token::Identifier("1".to_string()),
            Token::ParenthesisClose,
        ];

        let syntax_tree = parse(&tokens);

        assert_eq!(
            syntax_tree.root,
            Expression::Block {
                statements: vec![
                    Expression::Value(Value::Integer(1)),
                ]
            }
        )
    }

    #[test]
    fn test_expression_brackets_3() {
        let tokens = vec![
            Token::ParenthesisOpen,
            Token::Identifier("1".to_string()),
            Token::Add,
            Token::Identifier("2".to_string()),
            Token::ParenthesisClose,
        ];

        let syntax_tree = parse(&tokens);

        assert_eq!(
            syntax_tree.root,
            Expression::Block {
                statements: vec![
                    Expression::Binary {
                        left: Box::new(Expression::Value(Value::Integer(1))),
                        operator: BinaryOperator::Add,
                        right: Box::new(Expression::Value(Value::Integer(2))),
                    }
                ]
            }
        )
    }

    #[test]
    fn test_negate() {
        let tokens = vec![
            Token::NotEqual,
            Token::Identifier("a".to_string()),
        ];

        let syntax_tree = parse(&tokens);

        assert_eq!(
            syntax_tree.root,
            Expression::Block {
                statements: vec![
                    Expression::Unary {
                        operator: UnaryOperator::Negate,
                        right: Box::new(Expression::Identifier("a".to_string())),
                    }
                ]
            }
        )
    }
}

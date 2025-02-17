pub mod structs;
pub mod parse;
pub mod types;

use crate::lexer::tokens::Token;
use structs::Expression;
use parse::block::parse_expression_block;


pub fn parse(tokens: &[Token]) -> Expression {
    parse_expression_block(tokens)
}


#[cfg(test)]
mod tests {
    use crate::parser::structs::{BinaryOperator, UnaryOperator, Value};

    use super::*;

    #[test]
    fn test_expression_base() {
        let tokens = vec![
            Token::Identifier("2".to_string()),
            Token::Add,
            Token::Identifier("3".to_string()),
        ];

        assert_eq!(
            parse(&tokens),
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

        assert_eq!(
            parse(&tokens),
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

        assert_eq!(
            parse(&tokens),
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

        assert_eq!(
            parse(&tokens),
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

        assert_eq!(
            parse(&tokens),
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

        assert_eq!(
            parse(&tokens),
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

        assert_eq!(
            parse(&tokens),
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

        assert_eq!(
            parse(&tokens),
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

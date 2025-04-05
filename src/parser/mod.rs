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
    use std::vec;

    use crate::{lexer::tokens::TokenType, parser::structs::{BinaryOperator, UnaryOperator, Value}};

    use super::*;

    #[test]
    fn test_expression_base() {
        let tokens = vec![
            Token { token_type: TokenType::Identifier("2".to_string()), line: 1, column: 1 },
            Token { token_type: TokenType::Add, line: 1, column: 2 },
            Token { token_type: TokenType::Identifier("3".to_string()), line: 1, column: 3 },
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
            Token { token_type: TokenType::Identifier("1".to_string()), line: 1, column: 1 },
            Token { token_type: TokenType::Add, line: 1, column: 2 },
            Token { token_type: TokenType::Identifier("2".to_string()), line: 1, column: 3 },
            Token { token_type: TokenType::Subtract, line: 1, column: 4 },
            Token { token_type: TokenType::Identifier("3".to_string()), line: 1, column: 5 },
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
            Token { token_type: TokenType::Identifier("1".to_string()), line: 1, column: 1 },
            Token { token_type: TokenType::Multiply, line: 1, column: 2 },
            Token { token_type: TokenType::Identifier("2".to_string()), line: 1, column: 3 },
            Token { token_type: TokenType::Add, line: 1, column: 4 },
            Token { token_type: TokenType::Identifier("3".to_string()), line: 1, column: 5 },
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
            Token { token_type: TokenType::Identifier("1".to_string()), line: 1, column: 1 },
            Token { token_type: TokenType::Multiply, line: 1, column: 2 },
            Token { token_type: TokenType::Identifier("2".to_string()), line: 1, column: 3 },
            Token { token_type: TokenType::Add, line: 1, column: 4 },
            Token { token_type: TokenType::Identifier("3".to_string()), line: 1, column: 5 },
            Token { token_type: TokenType::Add, line: 1, column: 6 },
            Token { token_type: TokenType::Identifier("4".to_string()), line: 1, column: 7 },
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
            Token { token_type: TokenType::ParenthesisOpen, line: 1, column: 1 },
            Token { token_type: TokenType::ParenthesisClose, line: 1, column: 2 },
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
            Token { token_type: TokenType::ParenthesisOpen, line: 1, column: 1 },
            Token { token_type: TokenType::Identifier("1".to_string()), line: 1, column: 2 },
            Token { token_type: TokenType::ParenthesisClose, line: 1, column: 3 },
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
            Token { token_type: TokenType::ParenthesisOpen, line: 1, column: 1 },
            Token { token_type: TokenType::Identifier("1".to_string()), line: 1, column: 2 },
            Token { token_type: TokenType::Add, line: 1, column: 3 },
            Token { token_type: TokenType::Identifier("2".to_string()), line: 1, column: 4 },
            Token { token_type: TokenType::ParenthesisClose, line: 1, column: 5 },
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
            Token { token_type: TokenType::NotEqual, line: 1, column: 1 },
            Token { token_type: TokenType::Identifier("a".to_string()), line: 1, column: 2 },
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

    #[test]
    fn test_multiple_brackets() {
        let tokens = vec![
            Token { token_type: TokenType::ParenthesisOpen, line: 1, column: 1 },
            Token { token_type: TokenType::ParenthesisOpen, line: 1, column: 2 },
            Token { token_type: TokenType::ParenthesisOpen, line: 1, column: 3 },
            Token { token_type: TokenType::Identifier("1".to_string()), line: 1, column: 4 },
            Token { token_type: TokenType::Add, line: 1, column: 5 },
            Token { token_type: TokenType::Identifier("2".to_string()), line: 1, column: 6 },
            Token { token_type: TokenType::ParenthesisClose, line: 1, column: 7 },
            Token { token_type: TokenType::ParenthesisClose, line: 1, column: 8 },
            Token { token_type: TokenType::ParenthesisClose, line: 1, column: 9 },
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
    fn test_simple_function() {
        let tokens = vec![
            Token { token_type: TokenType::Identifier("func".to_string()), line: 1, column: 1 },
            Token { token_type: TokenType::Identifier("test".to_string()), line: 1, column: 2 },
            Token { token_type: TokenType::ParenthesisOpen, line: 1, column: 3 },
            Token { token_type: TokenType::ParenthesisClose, line: 1, column: 4 },
            Token { token_type: TokenType::EndLine, line: 1, column: 5 },
            Token { token_type: TokenType::NewLineIndent(4), line: 2, column: 1 },
            Token { token_type: TokenType::Identifier("a".to_string()), line: 2, column: 5 },
        ];

        assert_eq!(
            parse(&tokens),
            Expression::Block {
                statements: vec![
                    Expression::Function {
                        name: "test".to_string(),
                        parameters: vec![],
                        body: Box::new(Expression::Block {
                            statements: vec![
                                Expression::Identifier("a".to_string())
                            ]
                        }),
                    }
                ]
            }
        )
    }

    #[test]
    fn test_multiline_function() {
        let tokens = vec![
            Token { token_type: TokenType::Identifier("func".to_string()), line: 1, column: 1 },
            Token { token_type: TokenType::Identifier("test".to_string()), line: 1, column: 5 },
            Token { token_type: TokenType::ParenthesisOpen, line: 1, column: 9 },
            Token { token_type: TokenType::ParenthesisClose, line: 1, column: 10 },
            Token { token_type: TokenType::EndLine, line: 1, column: 11 },
            Token { token_type: TokenType::NewLineIndent(4), line: 2, column: 1 },
            Token { token_type: TokenType::Identifier("a".to_string()), line: 2, column: 5 },
        ];

        assert_eq!(
            parse(&tokens),
            Expression::Block {
                statements: vec![
                    Expression::Function {
                        name: "test".to_string(),
                        parameters: vec![],
                        body: Box::new(Expression::Block {
                            statements: vec![
                                Expression::Identifier("a".to_string())
                            ]
                        }),
                    }
                ]
            }
        )
    }
}

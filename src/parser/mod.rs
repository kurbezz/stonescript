use crate::lexer::tokens::Token;


#[derive(Debug, PartialEq)]
pub enum Value {
    String(String),
    Decimal(f64),
    Integer(i64),
}


#[derive(Debug, PartialEq)]
pub enum BinaryOperator {
    Add,
    Subtract,
    Multiply,
    Divide,
    Modulo,
    Equal,
    NotEqual,
    And,
    Or,
    Greater,
    Less,
    GreaterEqual,
    LessEqual,
}


#[derive(Debug, PartialEq)]
pub enum UnaryOperator {
    Not,
    Negate,
    Increment,
    Decrement,
}


#[derive(Debug, PartialEq)]
pub enum Expression {
    Value(Value),
    Identifier(String),
    Binary {
        left: Box<Expression>,
        operator: BinaryOperator,
        right: Box<Expression>,
    },
    Unary {
        operator: UnaryOperator,
        right: Box<Expression>,
    },
    Assignment {
        name: String,
        value: Box<Expression>,
    },
    If {
        condition: Box<Expression>,
        then_branch: Box<Expression>,
        else_branch: Option<Box<Expression>>,
    },
    Function {
        name: String,
        parameters: Vec<String>,
        body: Box<Expression>,
    },
    Block {
        statements: Vec<Expression>,
    },
}


#[derive(Debug)]
pub struct SyntaxTree {
    root: Expression,
}


pub struct Parser {
    tokens: Vec<Token>,
    current: usize,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Parser {
        Parser {
            tokens,
            current: 0,
        }
    }

    fn parse_primary(&mut self) -> Expression {
        let token = self.tokens[self.current].clone();

        match token {
            Token::Identifier(name) => {
                self.current += 1;

                if let Ok(value) = name.parse::<i64>() {
                    return Expression::Value(Value::Integer(value));
                }

                if let Ok(value) = name.parse::<f64>() {
                    return Expression::Value(Value::Decimal(value));
                }

                Expression::Identifier(name)
            },
            _ => panic!("Unexpected token: {:?}", token),
        }
    }

    fn parse_expression(&mut self) -> Expression {
        self.parse_primary()
    }

    fn parse_statement(&mut self) -> Expression {
        let token = self.tokens[self.current].clone();

        match token {
            Token::Identifier(name) => {
                self.current += 1;

                if self.tokens[self.current] == Token::Equal {
                    self.current += 1;

                    let value = self.parse_expression();

                    return Expression::Assignment {
                        name,
                        value: Box::new(value),
                    };
                }
            },
            _ => {},
        }

        self.parse_expression()
    }

    pub fn parse(&mut self) -> SyntaxTree {
        let mut statements = vec![];

        while self.current < self.tokens.len() {
            statements.push(self.parse_statement());
        }

        SyntaxTree {
            root: Expression::Block {
                statements,
            },
        }
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_assigment() {
        let tokens = vec![
            Token::Identifier("a".to_string()),
            Token::Equal,
            Token::Identifier("2".to_string()),
        ];

        let mut parser = Parser::new(tokens);
        let syntax_tree = parser.parse();

        assert_eq!(
            syntax_tree.root,
            Expression::Block {
                statements: vec![
                    Expression::Assignment {
                        name: "a".to_string(),
                        value: Box::new(Expression::Value(Value::Integer(2))),
                    }
                ],
            }
        );
    }
}

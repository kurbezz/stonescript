pub mod tokens;

use tokens::Token;


pub struct Lexer<'a> {
    content_iterator: std::iter::Peekable<std::str::Chars<'a>>,
    previous_token: Option<Token>,
}

impl<'a> Lexer<'a> {
    pub fn new(content: &'a str) -> Lexer<'a> {
        Lexer {
            content_iterator: content.chars().peekable(),
            previous_token: None,
        }
    }
}

impl Iterator for Lexer<'_> {
    type Item = Token;

    fn next(&mut self) -> Option<Self::Item> {
        self.previous_token = match self.content_iterator.next() {
            Some(c) => {
                match c {
                    ' ' => {
                        if self.previous_token == Some(Token::EndLine) || self.previous_token == None {
                            let mut indent = 1;

                            while let Some(&c) = self.content_iterator.peek() {
                                if c == ' ' {
                                    indent += 1;
                                    self.content_iterator.next();
                                } else {
                                    break;
                                }
                            }

                            Some(Token::NewLineIndent(indent))
                        } else {
                            self.next()
                        }
                    },
                    '/' => {
                        match self.content_iterator.peek() {
                            Some('/') => {
                                self.content_iterator.next();

                                let mut comment = "".to_string();

                                while let Some(&c) = self.content_iterator.peek() {
                                    if c == '\n' {
                                        break;
                                    }

                                    comment.push(c);
                                    self.content_iterator.next();
                                }

                                Some(Token::Comment(comment))
                            },
                            Some('*') => {
                                self.content_iterator.next();

                                let mut comment = "".to_string();

                                while let Some(&c) = self.content_iterator.peek() {
                                    if c == '*' {
                                        self.content_iterator.next();

                                        if let Some(&c) = self.content_iterator.peek() {
                                            if c == '/' {
                                                self.content_iterator.next();
                                                break;
                                            }
                                        }
                                    }

                                    comment.push(c);
                                    self.content_iterator.next();
                                }

                                Some(Token::CommentBlock(comment))
                            },
                            _ => Some(Token::Divide),
                        }
                    },
                    '?' => Some(Token::If),
                    ':' => {
                        match self.content_iterator.peek() {
                            Some('?') => {
                                self.content_iterator.next();
                                Some(Token::ElseIf)
                            },
                            _ => Some(Token::Else),
                        }
                    },
                    '=' => Some(Token::Equal),
                    '!' => Some(Token::NotEqual),
                    '&' => Some(Token::And),
                    '|' => Some(Token::Or),
                    '>' => {
                        match self.content_iterator.peek() {
                            Some('=') => {
                                self.content_iterator.next();
                                Some(Token::GreaterEqual)
                            },
                            _ => Some(Token::Greater),
                        }
                    },
                    '<' => {
                        match self.content_iterator.peek() {
                            Some('=') => {
                                self.content_iterator.next();
                                Some(Token::LessEqual)
                            },
                            _ => Some(Token::Less),
                        }
                    },
                    '+' => {
                        match self.content_iterator.peek() {
                            Some('+') => {
                                self.content_iterator.next();
                                Some(Token::Increment)
                            },
                            _ => Some(Token::Add),
                        }
                    },
                    '-' => {
                        match self.content_iterator.peek() {
                            Some('-') => {
                                self.content_iterator.next();
                                Some(Token::Decrement)
                            },
                            _ => Some(Token::Subtract),
                        }
                    },
                    '*' => Some(Token::Multiply),
                    '%' => Some(Token::Modulo),
                    '(' => Some(Token::ParenthesisOpen),
                    ')' => Some(Token::ParenthesisClose),
                    '[' => Some(Token::SquareBracketOpen),
                    ']' => Some(Token::SquareBracketClose),
                    ',' => Some(Token::Comma),
                    'a'..='z' | 'A'..='Z' => {
                        let mut identifier = c.to_string();

                        while let Some(&c) = self.content_iterator.peek() {
                            if c.is_alphanumeric() {
                                identifier.push(c);
                                self.content_iterator.next();
                            } else if c == '\n' {
                                match identifier.as_str() {
                                    "ascii" => {
                                        self.content_iterator.next();

                                        let mut ascii_block = "".to_string();

                                        while let Some(&c) = self.content_iterator.peek() {
                                            ascii_block.push(c);
                                            self.content_iterator.next();

                                            if ascii_block.ends_with("\nasciiend") {
                                                ascii_block.truncate(ascii_block.len() - "\nasciiend".len());
                                                break;
                                            }
                                        }

                                        return Some(Token::AsciiBlock(ascii_block));
                                    },
                                    _ => break,
                                }
                            } else {
                                break;
                            }
                        }

                        Some(Token::Identifier(identifier))
                    },
                    '\n' => Some(Token::EndLine),
                    _ => panic!("Unexpected character: {}", c),
                }
            },
            None => None,
        };

        self.previous_token.clone()
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_empty() {
        let lexer = Lexer::new("");

        let tokens = lexer.collect::<Vec<Token>>();

        assert_eq!(tokens.len(), 0);
    }

    #[test]
    fn test_identifier() {
        let lexer = Lexer::new("hello");

        let tokens = lexer.collect::<Vec<Token>>();

        assert_eq!(tokens.len(), 1);
        assert_eq!(tokens[0], Token::Identifier("hello".to_string()));
    }

    #[test]
    fn test_multiple_identifiers() {
        let lexer = Lexer::new("hello world");

        let tokens = lexer.collect::<Vec<Token>>();

        assert_eq!(tokens.len(), 2);
        assert_eq!(tokens[0], Token::Identifier("hello".to_string()));
        assert_eq!(tokens[1], Token::Identifier("world".to_string()));
    }

    #[test]
    fn test_newlines() {
        let lexer = Lexer::new("hello\nworld");

        let tokens = lexer.collect::<Vec<Token>>();

        assert_eq!(
            tokens,
            vec![
                Token::Identifier("hello".to_string()),
                Token::EndLine,
                Token::Identifier("world".to_string()),
            ]
        );
    }

    #[test]
    fn test_newlines_and_indent() {
        let lexer = Lexer::new("hello\n  world");

        let tokens = lexer.collect::<Vec<Token>>();

        assert_eq!(
            tokens,
            vec![
                Token::Identifier("hello".to_string()),
                Token::EndLine,
                Token::NewLineIndent(2),
                Token::Identifier("world".to_string()),
            ]
        );
    }

    #[test]
    fn test_if() {
        let lexer = Lexer::new("?test");

        let tokens = lexer.collect::<Vec<Token>>();

        assert_eq!(
            tokens,
            vec![
                Token::If,
                Token::Identifier("test".to_string()),
            ]
        );
    }

    #[test]
    fn test_if_else() {
        let lexer = Lexer::new("?test\n  hello\n:\n  world");

        let tokens = lexer.collect::<Vec<Token>>();

        assert_eq!(
            tokens,
            vec![
                Token::If,
                Token::Identifier("test".to_string()),
                Token::EndLine,
                Token::NewLineIndent(2),
                Token::Identifier("hello".to_string()),
                Token::EndLine,
                Token::Else,
                Token::EndLine,
                Token::NewLineIndent(2),
                Token::Identifier("world".to_string()),
            ]
        );
    }

    #[test]
    fn test_if_elseif_else() {
        let lexer = Lexer::new("?test1\n  hello\n:?\n  world\n  test2\n:\n  world");

        let tokens = lexer.collect::<Vec<Token>>();

        assert_eq!(
            tokens,
            vec![
                Token::If,
                Token::Identifier("test1".to_string()),
                Token::EndLine,
                Token::NewLineIndent(2),
                Token::Identifier("hello".to_string()),
                Token::EndLine,
                Token::ElseIf,
                Token::EndLine,
                Token::NewLineIndent(2),
                Token::Identifier("world".to_string()),
                Token::EndLine,
                Token::NewLineIndent(2),
                Token::Identifier("test2".to_string()),
                Token::EndLine,
                Token::Else,
                Token::EndLine,
                Token::NewLineIndent(2),
                Token::Identifier("world".to_string()),
            ]
        );
    }

    #[test]
    fn test_comment() {
        let lexer = Lexer::new("// this is a comment");

        let tokens = lexer.collect::<Vec<Token>>();

        assert_eq!(
            tokens,
            vec![
                Token::Comment(" this is a comment".to_string()),
            ]
        );
    }

    #[test]
    fn test_comment_block() {
        let lexer = Lexer::new("/* this is a comment block */");

        let tokens = lexer.collect::<Vec<Token>>();

        assert_eq!(
            tokens,
            vec![
                Token::CommentBlock(" this is a comment block ".to_string()),
            ]
        );
    }

    #[test]
    fn test_comment_block_with_newlines() {
        let lexer = Lexer::new("/* this is a comment block\nwith newlines */");

        let tokens = lexer.collect::<Vec<Token>>();

        assert_eq!(
            tokens,
            vec![
                Token::CommentBlock(" this is a comment block\nwith newlines ".to_string()),
            ]
        );
    }

    #[test]
    fn test_equal() {
        let lexer = Lexer::new("=");

        let tokens = lexer.collect::<Vec<Token>>();

        assert_eq!(
            tokens,
            vec![
                Token::Equal,
            ]
        );
    }

    #[test]
    fn test_not_equal() {
        let lexer = Lexer::new("!");

        let tokens = lexer.collect::<Vec<Token>>();

        assert_eq!(
            tokens,
            vec![
                Token::NotEqual,
            ]
        );
    }

    #[test]
    fn test_and() {
        let lexer = Lexer::new("&");

        let tokens = lexer.collect::<Vec<Token>>();

        assert_eq!(
            tokens,
            vec![
                Token::And,
            ]
        );
    }

    #[test]
    fn test_or() {
        let lexer = Lexer::new("|");

        let tokens = lexer.collect::<Vec<Token>>();

        assert_eq!(
            tokens,
            vec![
                Token::Or,
            ]
        );
    }

    #[test]
    fn test_greater() {
        let lexer = Lexer::new(">");

        let tokens = lexer.collect::<Vec<Token>>();

        assert_eq!(
            tokens,
            vec![
                Token::Greater,
            ]
        );
    }

    #[test]
    fn test_less() {
        let lexer = Lexer::new("<");

        let tokens = lexer.collect::<Vec<Token>>();

        assert_eq!(
            tokens,
            vec![
                Token::Less,
            ]
        );
    }

    #[test]
    fn test_greater_equal() {
        let lexer = Lexer::new(">=");

        let tokens = lexer.collect::<Vec<Token>>();

        assert_eq!(
            tokens,
            vec![
                Token::GreaterEqual,
            ]
        );
    }

    #[test]
    fn test_less_equal() {
        let lexer = Lexer::new("<=");

        let tokens = lexer.collect::<Vec<Token>>();

        assert_eq!(
            tokens,
            vec![
                Token::LessEqual,
            ]
        );
    }

    #[test]
    fn test_add() {
        let lexer = Lexer::new("+");

        let tokens = lexer.collect::<Vec<Token>>();

        assert_eq!(
            tokens,
            vec![
                Token::Add,
            ]
        );
    }

    #[test]
    fn test_subtract() {
        let lexer = Lexer::new("-");

        let tokens = lexer.collect::<Vec<Token>>();

        assert_eq!(
            tokens,
            vec![
                Token::Subtract,
            ]
        );
    }

    #[test]
    fn test_multiply() {
        let lexer = Lexer::new("*");

        let tokens = lexer.collect::<Vec<Token>>();

        assert_eq!(
            tokens,
            vec![
                Token::Multiply,
            ]
        );
    }

    #[test]
    fn test_divide() {
        let lexer = Lexer::new("/");

        let tokens = lexer.collect::<Vec<Token>>();

        assert_eq!(
            tokens,
            vec![
                Token::Divide,
            ]
        );
    }

    #[test]
    fn test_increment() {
        let lexer = Lexer::new("++");

        let tokens = lexer.collect::<Vec<Token>>();

        assert_eq!(
            tokens,
            vec![
                Token::Increment,
            ]
        );
    }

    #[test]
    fn test_decrement() {
        let lexer = Lexer::new("--");

        let tokens = lexer.collect::<Vec<Token>>();

        assert_eq!(
            tokens,
            vec![
                Token::Decrement,
            ]
        );
    }

    #[test]
    fn test_modulo() {
        let lexer = Lexer::new("%");

        let tokens = lexer.collect::<Vec<Token>>();

        assert_eq!(
            tokens,
            vec![
                Token::Modulo,
            ]
        );
    }

    #[test]
    fn test_parenthesis_open() {
        let lexer = Lexer::new("(");

        let tokens = lexer.collect::<Vec<Token>>();

        assert_eq!(
            tokens,
            vec![
                Token::ParenthesisOpen,
            ]
        );
    }

    #[test]
    fn test_parenthesis_close() {
        let lexer = Lexer::new(")");

        let tokens = lexer.collect::<Vec<Token>>();

        assert_eq!(
            tokens,
            vec![
                Token::ParenthesisClose,
            ]
        );
    }

    #[test]
    fn test_square_bracket_open() {
        let lexer = Lexer::new("[");

        let tokens = lexer.collect::<Vec<Token>>();

        assert_eq!(
            tokens,
            vec![
                Token::SquareBracketOpen,
            ]
        );
    }


    #[test]
    fn test_square_bracket_close() {
        let lexer = Lexer::new("]");

        let tokens = lexer.collect::<Vec<Token>>();

        assert_eq!(
            tokens,
            vec![
                Token::SquareBracketClose,
            ]
        );
    }

    #[test]
    fn test_comma() {
        let lexer = Lexer::new(",");

        let tokens = lexer.collect::<Vec<Token>>();

        assert_eq!(
            tokens,
            vec![
                Token::Comma,
            ]
        );
    }

    #[test]
    fn test_ascii_block() {
        let lexer = Lexer::new("ascii\nhello\nasciiend");

        let tokens = lexer.collect::<Vec<Token>>();

        assert_eq!(
            tokens,
            vec![
                Token::AsciiBlock("hello".to_string()),
            ]
        );
    }
}

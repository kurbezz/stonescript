pub mod tokens;

use tokens::{Token, TokenType};


pub struct ContentIterator<'a>  {
    iterator: std::iter::Peekable<std::str::Chars<'a>>,
    line: usize,
    column: usize,
}

impl ContentIterator<'_> {
    pub fn new(content: &str) -> ContentIterator {
        ContentIterator {
            iterator: content.chars().peekable(),
            line: 1,
            column: 0,
        }
    }

    pub fn next(&mut self) -> Option<char> {
        match self.iterator.next() {
            Some(c) => {
                if c == '\n' {
                    self.line += 1;
                    self.column = 0;
                } else {
                    self.column += 1;
                }

                Some(c)
            },
            None => None,
        }
    }

    pub fn peek(&mut self) -> Option<&char> {
        self.iterator.peek()
    }
}


pub struct Lexer<'a> {
    content_iterator: ContentIterator<'a>,
    previous_token: Option<Token>,
    line: usize,
    column: usize,
}

impl<'a> Lexer<'a> {
    pub fn new(content: &'a str) -> Lexer<'a> {
        Lexer {
            content_iterator: ContentIterator::new(content),
            previous_token: None,
            line: 1,
            column: 0,
        }
    }
}

impl Lexer<'_> {
    pub fn create_token(&self, token_type: TokenType) -> Token {
        Token {
            token_type,
            line: self.line,
            column: self.column,
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
                        if self.previous_token.clone().is_some_and(|t| t.token_type == TokenType::EndLine) || self.previous_token == None {
                            let mut indent = 1;

                            while let Some(&c) = self.content_iterator.peek() {
                                if c == ' ' {
                                    indent += 1;
                                    self.content_iterator.next();
                                } else {
                                    break;
                                }
                            }

                            Some(self.create_token(TokenType::NewLineIndent(indent)))
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

                                Some(self.create_token(TokenType::Comment(comment)))
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

                                Some(self.create_token(TokenType::CommentBlock(comment)))
                            },
                            _ => Some(self.create_token(TokenType::Divide)),
                        }
                    },
                    '?' => Some(self.create_token(TokenType::If)),
                    ':' => {
                        match self.content_iterator.peek() {
                            Some('?') => {
                                self.content_iterator.next();
                                Some(self.create_token(TokenType::ElseIf))
                            },
                            _ => Some(self.create_token(TokenType::Else)),
                        }
                    },
                    '=' => Some(self.create_token(TokenType::Equal)),
                    '!' => Some(self.create_token(TokenType::NotEqual)),
                    '&' => Some(self.create_token(TokenType::And)),
                    '|' => Some(self.create_token(TokenType::Or)),
                    '>' => {
                        match self.content_iterator.peek() {
                            Some('=') => {
                                self.content_iterator.next();
                                Some(self.create_token(TokenType::GreaterEqual))
                            },
                            _ => Some(self.create_token(TokenType::Greater)),
                        }
                    },
                    '<' => {
                        match self.content_iterator.peek() {
                            Some('=') => {
                                self.content_iterator.next();
                                Some(self.create_token(TokenType::LessEqual))
                            },
                            _ => Some(self.create_token(TokenType::Less)),
                        }
                    },
                    '+' => {
                        match self.content_iterator.peek() {
                            Some('+') => {
                                self.content_iterator.next();
                                Some(self.create_token(TokenType::Increment))
                            },
                            _ => Some(self.create_token(TokenType::Add)),
                        }
                    },
                    '-' => {
                        match self.content_iterator.peek() {
                            Some('-') => {
                                self.content_iterator.next();
                                Some(self.create_token(TokenType::Decrement))
                            },
                            _ => Some(self.create_token(TokenType::Subtract)),
                        }
                    }
                    '*' => Some(self.create_token(TokenType::Multiply)),
                    '%' => Some(self.create_token(TokenType::Modulo)),
                    '(' => Some(self.create_token(TokenType::ParenthesisOpen)),
                    ')' => Some(self.create_token(TokenType::ParenthesisClose)),
                    '[' => Some(self.create_token(TokenType::SquareBracketOpen)),
                    ']' => Some(self.create_token(TokenType::SquareBracketClose)),
                    ',' => Some(self.create_token(TokenType::Comma)),
                    '"' => {
                        let mut string = "".to_string();

                        while let Some(&c) = self.content_iterator.peek() {
                            if c == '"' {
                                self.content_iterator.next();
                                break;
                            }

                            string.push(c);
                            self.content_iterator.next();
                        }

                        Some(self.create_token(TokenType::String(string)))
                    },
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

                                        return Some(self.create_token(TokenType::AsciiBlock(ascii_block)));
                                    },
                                    _ => break,
                                }
                            } else {
                                break;
                            }
                        }

                        Some(self.create_token(TokenType::Identifier(identifier)))
                    },
                    '\n' => {
                        Some(self.create_token(TokenType::EndLine))
                    },
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

        assert_eq!(
            tokens,
            vec![
                TokenType::Identifier("hello".to_string()),
            ]
        )
    }

    #[test]
    fn test_multiple_identifiers() {
        let lexer = Lexer::new("hello world");

        let tokens = lexer.collect::<Vec<Token>>();

        assert_eq!(
            tokens,
            vec![
                TokenType::Identifier("hello".to_string()),
                TokenType::Identifier("world".to_string()),
            ]
        )
    }

    #[test]
    fn test_newlines() {
        let lexer = Lexer::new("hello\nworld");

        let tokens = lexer.collect::<Vec<Token>>();

        assert_eq!(
            tokens,
            vec![
                TokenType::Identifier("hello".to_string()),
                TokenType::EndLine,
                TokenType::Identifier("world".to_string()),
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
                TokenType::Identifier("hello".to_string()),
                TokenType::EndLine,
                TokenType::NewLineIndent(2),
                TokenType::Identifier("world".to_string()),
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
                TokenType::If,
                TokenType::Identifier("test".to_string()),
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
                TokenType::If,
                TokenType::Identifier("test".to_string()),
                TokenType::EndLine,
                TokenType::NewLineIndent(2),
                TokenType::Identifier("hello".to_string()),
                TokenType::EndLine,
                TokenType::Else,
                TokenType::EndLine,
                TokenType::NewLineIndent(2),
                TokenType::Identifier("world".to_string()),
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
                TokenType::If,
                TokenType::Identifier("test1".to_string()),
                TokenType::EndLine,
                TokenType::NewLineIndent(2),
                TokenType::Identifier("hello".to_string()),
                TokenType::EndLine,
                TokenType::ElseIf,
                TokenType::EndLine,
                TokenType::NewLineIndent(2),
                TokenType::Identifier("world".to_string()),
                TokenType::EndLine,
                TokenType::NewLineIndent(2),
                TokenType::Identifier("test2".to_string()),
                TokenType::EndLine,
                TokenType::Else,
                TokenType::EndLine,
                TokenType::NewLineIndent(2),
                TokenType::Identifier("world".to_string()),
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
                TokenType::Comment(" this is a comment".to_string()),
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
                TokenType::CommentBlock(" this is a comment block ".to_string()),
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
                TokenType::CommentBlock(" this is a comment block\nwith newlines ".to_string()),
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
                TokenType::Equal,
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
                TokenType::NotEqual,
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
                TokenType::And,
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
                TokenType::Or,
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
                TokenType::Greater,
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
                TokenType::Less,
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
                TokenType::GreaterEqual,
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
                TokenType::LessEqual,
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
                TokenType::Add,
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
                TokenType::Subtract,
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
                TokenType::Multiply,
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
                TokenType::Divide,
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
                TokenType::Increment,
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
                TokenType::Decrement,
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
                TokenType::Modulo,
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
                TokenType::ParenthesisOpen,
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
                TokenType::ParenthesisClose,
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
                TokenType::SquareBracketOpen,
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
                TokenType::SquareBracketClose,
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
                TokenType::Comma,
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
                TokenType::AsciiBlock("hello".to_string()),
            ]
        );
    }

    #[test]
    fn test_string() {
        let lexer = Lexer::new("\"hello\"");

        let tokens = lexer.collect::<Vec<Token>>();

        assert_eq!(
            tokens,
            vec![
                TokenType::String("hello".to_string()),
            ]
        );
    }
}

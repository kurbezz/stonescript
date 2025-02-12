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
                    'a'..='z' | 'A'..='Z' => {
                        let mut identifier = c.to_string();

                        while let Some(&c) = self.content_iterator.peek() {
                            if c.is_alphanumeric() || c == '_' {
                                identifier.push(c);
                                self.content_iterator.next();
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
}

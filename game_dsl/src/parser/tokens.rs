use crate::{
    error::Error,
    lexer::{Token, TokenValue},
    location::Location,
};

#[derive(Clone, PartialEq, Debug)]
pub struct Tokens {
    tokens: Vec<Token>,
    last_location: Location,
}
impl Tokens {
    pub fn new(tokens: Vec<Token>, start_location: Location) -> Self {
        Self {
            tokens,
            last_location: start_location,
        }
    }

    pub fn last_location(&self) -> Location {
        self.last_location.clone()
    }

    pub fn is_empty(&self) -> bool {
        self.tokens.is_empty()
    }

    pub fn pop(&mut self) -> Option<Token> {
        if self.is_empty() {
            None
        } else {
            let token = self.tokens.remove(0);
            self.last_location = token.end_location.clone();
            Some(token)
        }
    }

    pub fn pop_expected(&mut self, expected: TokenValue) -> Result<Token, Error> {
        match self.pop() {
            Some(t) if t.value == expected => Ok(t),
            Some(t) => {
                let message = format!(
                    "Expected {}, got {}",
                    expected.display_name(),
                    t.display_name()
                );
                Err(Error {
                    message,
                    location: t.start_location,
                })
            }
            None => Err(Error {
                message: format!("Expected {}, got nothing!", expected.display_name()),
                location: self.last_location.clone(),
            }),
        }
    }

    pub fn peek_expected(&self, expected: TokenValue) -> bool {
        match self.peek() {
            Some(t) => t == &expected,
            None => false,
        }
    }

    pub fn peek_expected_nth(&self, n: usize, expected: TokenValue) -> bool {
        match self.peek_nth(n) {
            Some(t) => &t.value == &expected,
            None => false,
        }
    }

    pub fn peek_nth(&self, n: usize) -> Option<&Token> {
        self.tokens.get(n)
    }

    pub fn peek(&self) -> Option<&TokenValue> {
        match self.tokens.first() {
            Some(Token {
                value,
                start_location: _,
                end_location: _,
            }) => Some(value),
            None => None,
        }
    }

    pub fn peek_identifier(&self, id: &str) -> bool {
        match self.peek() {
            Some(TokenValue::Identifier(s)) => s == id,
            _ => false,
        }
    }

    pub fn peek_comment(&self) -> bool {
        match self.peek() {
            Some(TokenValue::Comment(_)) => true,
            _ => false,
        }
    }

    pub fn pop_comment(&mut self) -> Result<(Vec<String>, Token), Error> {
        match self.pop() {
            Some(Token {
                value: TokenValue::Comment(s),
                start_location,
                end_location,
            }) => Ok((
                s.clone(),
                Token {
                    value: TokenValue::Comment(s),
                    start_location,
                    end_location,
                },
            )),
            Some(t) => {
                let message = format!("Expected comment, got {}", t.display_name(),);
                Err(Error {
                    message,
                    location: t.start_location,
                })
            }
            None => Err(Error {
                message: "Expected comment, got nothing!".to_string(),
                location: self.last_location.clone(),
            }),
        }
    }

    pub fn pop_number(&mut self) -> Result<(f64, Token), Error> {
        match self.pop() {
            Some(Token {
                value: TokenValue::Number(n),
                start_location,
                end_location,
            }) => Ok((
                n.clone(),
                Token {
                    value: TokenValue::Number(n),
                    start_location,
                    end_location,
                },
            )),
            Some(t) => {
                let message = format!("Expected number, got {}", t.display_name(),);
                Err(Error {
                    message,
                    location: t.start_location,
                })
            }
            None => Err(Error {
                message: "Expected number, got nothing!".to_string(),
                location: self.last_location.clone(),
            }),
        }
    }

    pub fn insert_head(&mut self, token: Token) {
        self.tokens.insert(0, token);
    }

    pub fn pop_identifier(&mut self) -> Result<(String, Token), Error> {
        match self.pop() {
            Some(Token {
                value: TokenValue::Identifier(s),
                start_location,
                end_location,
            }) => Ok((
                s.clone(),
                Token {
                    value: TokenValue::Identifier(s),
                    start_location,
                    end_location,
                },
            )),
            Some(t) => {
                let message = format!("Expected identifier, got {}", t.display_name(),);
                Err(Error {
                    message,
                    location: t.start_location,
                })
            }
            None => Err(Error {
                message: "Expected identifier, got nothing!".to_string(),
                location: self.last_location.clone(),
            }),
        }
    }
}

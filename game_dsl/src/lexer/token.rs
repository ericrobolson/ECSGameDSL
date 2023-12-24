use crate::location::Location;

#[macro_export]
macro_rules! string_constructor {
    ($token_type:ident, $token_id:ident) => {
        pub fn $token_id(s: String, start_location: Location, end_location: Location) -> Self {
            Self {
                value: TokenValue::$token_type(s),
                start_location,
                end_location,
            }
        }
    };
}

#[macro_export]
macro_rules! symbol_constructor {
    ($token_type:ident, $token_id:ident) => {
        pub fn $token_id(start_location: Location, end_location: Location) -> Self {
            Self {
                value: TokenValue::$token_type,
                start_location,
                end_location,
            }
        }
    };
}

#[derive(Debug, PartialEq, Clone)]
pub enum TokenValue {
    String(String),
    Comment(Vec<String>),
    Identifier(String),
    Number(f64),
    LCurlyBrace,
    RCurlyBrace,
    Comma,
    Colon,
    LParen,
    RParen,
    LSquareBracket,
    RSquareBracket,
    Period,
    Assign,
    ExclamationMark,
    LChevron,
    RChevron,
    Asterisk,
    Plus,
    Minus,
    Slash,
    Percent,
    Ampersand,
    Pipe,
    Semicolon,
    Carrot,
    // Double token values
    PlusAssign,
    SubAssign,
    MulAssign,
    DivAssign,
    ModAssign,
    PipeAssign,
    AndAssign,
    CarrotAssign,
    PlusPlus,
    MinusMinus,
    NotNot,
    Or,
    And,
    LessThanEqual,
    GreaterThanEqual,
    Equal,
    NotEqual,
    LShift,
    RShift,
}
impl TokenValue {
    pub fn display_name(&self) -> String {
        match self {
            TokenValue::String(s) => format!("string: {}", s),
            TokenValue::Comment(c) => format!("comment: {}", c.join("\n")),
            TokenValue::Number(n) => format!("number: {}", n),
            TokenValue::Identifier(id) => format!("identifier: {id}"),
            TokenValue::LCurlyBrace => "{".to_string(),
            TokenValue::RCurlyBrace => "}".to_string(),
            TokenValue::Comma => ",".to_string(),
            TokenValue::Colon => ":".to_string(),
            TokenValue::LParen => "(".to_string(),
            TokenValue::RParen => ")".to_string(),
            TokenValue::LSquareBracket => "[".to_string(),
            TokenValue::RSquareBracket => "]".to_string(),
            TokenValue::Period => ".".to_string(),
            TokenValue::Assign => "=".to_string(),
            TokenValue::ExclamationMark => "!".to_string(),
            TokenValue::LChevron => "<".to_string(),
            TokenValue::RChevron => ">".to_string(),
            TokenValue::Asterisk => "*".to_string(),
            TokenValue::Plus => "+".to_string(),
            TokenValue::Minus => "-".to_string(),
            TokenValue::Slash => "/".to_string(),
            TokenValue::Percent => "%".to_string(),
            TokenValue::Ampersand => "&".to_string(),
            TokenValue::Pipe => "|".to_string(),
            TokenValue::Semicolon => ";".to_string(),
            TokenValue::Carrot => "^".to_string(),
            TokenValue::PlusAssign => "+=".to_string(),
            TokenValue::SubAssign => "-=".to_string(),
            TokenValue::MulAssign => "*=".to_string(),
            TokenValue::DivAssign => "/=".to_string(),
            TokenValue::ModAssign => "%=".to_string(),
            TokenValue::PipeAssign => "|=".to_string(),
            TokenValue::AndAssign => "&=".to_string(),
            TokenValue::CarrotAssign => "^=".to_string(),
            TokenValue::PlusPlus => "++".to_string(),
            TokenValue::MinusMinus => "--".to_string(),
            TokenValue::NotNot => "!!".to_string(),
            TokenValue::Or => "||".to_string(),
            TokenValue::And => "&&".to_string(),
            TokenValue::LessThanEqual => "<=".to_string(),
            TokenValue::GreaterThanEqual => ">=".to_string(),
            TokenValue::Equal => "==".to_string(),
            TokenValue::NotEqual => "!=".to_string(),
            TokenValue::LShift => "<<".to_string(),
            TokenValue::RShift => ">>".to_string(),
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct Token {
    pub value: TokenValue,
    pub start_location: Location,
    pub end_location: Location,
}
impl Token {
    symbol_constructor!(RParen, rparen);
    symbol_constructor!(LParen, lparen);
    symbol_constructor!(Colon, colon);
    symbol_constructor!(Comma, comma);
    symbol_constructor!(LCurlyBrace, lcurlybrace);
    symbol_constructor!(RCurlyBrace, rcurlybrace);
    symbol_constructor!(LSquareBracket, lsquarebracket);
    symbol_constructor!(RSquareBracket, rsquarebracket);
    symbol_constructor!(Period, period);
    symbol_constructor!(Assign, assign);
    symbol_constructor!(ExclamationMark, exclamationmark);
    symbol_constructor!(LChevron, lchevron);
    symbol_constructor!(RChevron, rchevron);
    symbol_constructor!(Asterisk, asterisk);
    symbol_constructor!(Plus, plus);
    symbol_constructor!(Minus, minus);
    symbol_constructor!(Slash, slash);
    symbol_constructor!(Percent, percent);
    symbol_constructor!(Ampersand, ampersand);
    symbol_constructor!(Pipe, pipe);
    symbol_constructor!(Semicolon, semicolon);
    symbol_constructor!(Carrot, carrot);
    symbol_constructor!(PlusAssign, plus_assign);
    symbol_constructor!(SubAssign, sub_assign);
    symbol_constructor!(MulAssign, mul_assign);
    symbol_constructor!(DivAssign, div_assign);
    symbol_constructor!(ModAssign, mod_assign);
    symbol_constructor!(PipeAssign, pipe_assign);
    symbol_constructor!(AndAssign, and_assign);
    symbol_constructor!(CarrotAssign, carrot_assign);
    symbol_constructor!(PlusPlus, plus_plus);
    symbol_constructor!(MinusMinus, minus_minus);
    symbol_constructor!(NotNot, not_not);
    symbol_constructor!(Or, or);
    symbol_constructor!(And, and);
    symbol_constructor!(LessThanEqual, less_than_equal);
    symbol_constructor!(GreaterThanEqual, greater_than_equal);
    symbol_constructor!(Equal, equal);
    symbol_constructor!(NotEqual, not_equal);
    symbol_constructor!(LShift, lshift);
    symbol_constructor!(RShift, rshift);

    string_constructor!(String, string);
    string_constructor!(Identifier, identifier);

    pub fn is_identifier(&self) -> bool {
        match &self.value {
            TokenValue::Identifier(_) => true,
            _ => false,
        }
    }

    pub fn comment(s: String, start_location: Location, end_location: Location) -> Self {
        Self {
            value: TokenValue::Comment(vec![s]),
            start_location,
            end_location,
        }
    }

    pub fn comments(s: Vec<String>, start_location: Location, end_location: Location) -> Self {
        Self {
            value: TokenValue::Comment(s),
            start_location,
            end_location,
        }
    }

    pub fn number(n: f64, start_location: Location, end_location: Location) -> Self {
        Self {
            value: TokenValue::Number(n),
            start_location,
            end_location,
        }
    }

    pub fn display_name(&self) -> String {
        self.value.display_name()
    }

    /// Checks if two tokens are back to back
    pub fn is_back_to_back(&self, other: &Self) -> bool {
        self.end_location == other.start_location
    }

    /// Checks if two tokens are on lines next to each other
    pub fn is_subsequent_line(&self, other: &Self) -> bool {
        self.end_location.line() + 1 == other.start_location.line()
    }

    pub fn finalize(&self) -> Self {
        let value = match &self.value {
            TokenValue::String(s) => TokenValue::String(s.clone()),
            TokenValue::Comment(s) => {
                let s = s.clone().iter_mut().map(|c| c.trim().to_string()).collect();
                TokenValue::Comment(s)
            }
            TokenValue::Identifier(s) => {
                let id = s.trim();

                // Check if it's a number
                if let Ok(n) = id.parse::<f64>() {
                    TokenValue::Number(n)
                } else {
                    TokenValue::Identifier(id.to_string())
                }
            }
            t => t.clone(),
        };

        // Ensure location types match
        if self.start_location.is_file() {
            assert!(
                self.end_location.is_file(),
                "start location is file and end location is not! (start: {:?}, end: {:?})",
                self.start_location,
                self.end_location
            );
        } else {
            assert!(
                self.end_location.is_text(),
                "start location is text and end location is not! (start: {:?}, end: {:?})",
                self.start_location,
                self.end_location
            );
        }

        Self {
            value,
            start_location: self.start_location.clone(),
            end_location: self.end_location.clone(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn is_back_to_back_returns_true() {
        let token1 = Token::identifier("+".to_string(), (0, 0).into(), (0, 1).into());
        let token2 = Token::identifier("=".to_string(), (0, 1).into(), (0, 2).into());
        assert!(token1.is_back_to_back(&token2));
    }

    #[test]
    fn is_back_to_back_space_returns_false() {
        let token1 = Token::identifier("+".to_string(), (0, 0).into(), (0, 1).into());
        let token2 = Token::identifier("=".to_string(), (0, 2).into(), (1, 2).into());
        assert_eq!(false, token1.is_back_to_back(&token2));
    }

    #[test]
    fn is_back_to_back_newline_returns_false() {
        let token1 = Token::identifier("+".to_string(), (0, 0).into(), (0, 1).into());
        let token2 = Token::identifier("=".to_string(), (1, 1).into(), (1, 2).into());
        assert_eq!(false, token1.is_back_to_back(&token2));
    }
}

use crate::{error::Error, lexer::TokenValue, location::Location};

use super::{primitives, Primitive, Tokens};

/// An element that may be a list or a single element.
#[derive(Debug, Clone, PartialEq)]
pub enum ListType<T> {
    Single(T),
    List { ty: T, max_size: usize },
}

/// An element that may be a list or a single element.
#[derive(Debug, Clone, PartialEq)]
pub struct Listable<T> {
    pub ty: ListType<T>,
    pub start_location: Location,
    pub end_location: Location,
}
impl<T> Listable<T> {
    pub fn inner_ty(&self) -> &T {
        match self.ty {
            ListType::Single(ref ty) => ty,
            ListType::List { ref ty, .. } => ty,
        }
    }
}

pub type ListableParser<T> = fn(&mut Tokens) -> Result<ListableParserResult<T>, Error>;
pub struct ListableParserResult<T> {
    pub value: T,
    pub start_location: Location,
    pub end_location: Location,
}

/// Parses a listable element.
pub fn parse_listable<T>(
    tokens: &mut Tokens,
    constructor: ListableParser<T>,
) -> Result<Listable<T>, Error> {
    // Check if it's a single element
    if tokens.peek() != Some(&TokenValue::LSquareBracket) {
        let result = constructor(tokens)?;
        return Ok(Listable {
            ty: ListType::Single(result.value),
            start_location: result.start_location.clone(),
            end_location: result.end_location.clone(),
        });
    }

    // Parse a list
    let token = tokens.pop_expected(TokenValue::LSquareBracket)?;
    let start_location = token.start_location.clone();

    // Pop inner type
    let ty = constructor(tokens)?;

    // Parse list size
    let max_size;
    {
        let (value, token) = tokens.pop_number()?;

        max_size = value as usize;

        if max_size == 0 {
            return Err(Error {
                message: format!("List size must be greater than 0"),
                location: token.start_location.clone(),
            });
        }
    }
    let token = tokens.pop_expected(TokenValue::RSquareBracket)?;
    let end_location = token.end_location.clone();

    Ok(Listable {
        ty: ListType::List {
            ty: ty.value,
            max_size,
        },
        start_location,
        end_location,
    })
}

/// Attempts to parse a listable primitive
pub fn parse_listable_primitive(tokens: &mut Tokens) -> Result<Listable<Primitive>, Error> {
    parse_listable(tokens, parse_listable_primitive_constructor)
}

fn parse_listable_primitive_constructor(
    tokens: &mut Tokens,
) -> Result<ListableParserResult<Primitive>, Error> {
    let value = primitives::parse(tokens)?;
    Ok(ListableParserResult {
        start_location: value.start_location.clone(),
        end_location: value.end_location.clone(),
        value,
    })
}

#[cfg(test)]
mod tests {
    use crate::{lexer, location::Location};

    use super::*;

    fn construct(tokens: &mut Tokens) -> Result<ListableParserResult<String>, Error> {
        let (id, token) = tokens.pop_identifier()?;
        Ok(ListableParserResult {
            value: id,
            start_location: token.start_location,
            end_location: token.end_location,
        })
    }

    fn lex(input: &str) -> Tokens {
        let start_location = Location::default();
        let tokens = lexer::lex(input, start_location.clone()).unwrap();
        let tokens = Tokens::new(tokens, start_location);
        tokens
    }

    #[test]
    fn parse_cant_do_list_or_single_element() {
        let mut tokens = lex(";");
        let result = parse_listable(&mut tokens, construct);
        let expected = Err(Error {
            message: "Expected identifier, got ;".to_string(),
            location: (0, 0).into(),
        });
        assert_eq!(expected, result);
    }

    #[test]
    fn parse_single_element() {
        let mut tokens = lex("int");
        let result = parse_listable(&mut tokens, construct).unwrap();
        let expected = Listable {
            ty: ListType::Single("int".to_string()),
            start_location: (0, 0).into(),
            end_location: (0, 3).into(),
        };
        assert_eq!(expected, result);
    }

    #[test]
    fn parse_list_returns_ok() {
        let mut tokens = lex("[int 256]");
        let result = parse_listable(&mut tokens, construct).unwrap();
        let expected = Listable {
            ty: ListType::List {
                ty: "int".to_string(),
                max_size: 256,
            },
            start_location: (0, 0).into(),
            end_location: (0, 9).into(),
        };
        assert_eq!(expected, result);
    }

    #[test]
    fn parse_list_size64_returns_ok() {
        let mut tokens = lex("[int 64]");
        let result = parse_listable(&mut tokens, construct).unwrap();
        let expected = Listable {
            ty: ListType::List {
                ty: "int".to_string(),
                max_size: 64,
            },
            start_location: (0, 0).into(),
            end_location: (0, 8).into(),
        };
        assert_eq!(expected, result);
    }

    #[test]
    fn parse_list_0_size_returns_err() {
        let mut tokens = lex("[int 0]");
        let result = parse_listable(&mut tokens, construct);
        let expected = Err(Error {
            message: "List size must be greater than 0".to_string(),
            location: (0, 5).into(),
        });
        assert_eq!(expected, result);
    }

    #[test]
    fn parse_list_no_size_returns_err() {
        let mut tokens = lex("[int aba]");
        let result = parse_listable(&mut tokens, construct);
        let expected = Err(Error {
            message: "Expected number, got identifier: aba".to_string(),
            location: (0, 5).into(),
        });
        assert_eq!(expected, result);
    }

    #[test]
    fn parse_list_invalid_type_returns_err() {
        let mut tokens = lex("[: 12]");
        let result = parse_listable(&mut tokens, construct);
        let expected = Err(Error {
            message: "Expected identifier, got :".to_string(),
            location: (0, 1).into(),
        });
        assert_eq!(expected, result);
    }
}

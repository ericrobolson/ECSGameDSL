use super::*;
use crate::{error::Error, location::Location};

/// Primitives are the basic building blocks of the language.
#[derive(Debug, Clone, PartialEq)]
pub struct Primitive {
    pub primitive_type: PrimitiveType,
    pub start_location: Location,
    pub end_location: Location,
}

impl Primitive {
    pub fn is_identifier(&self) -> bool {
        match &self.primitive_type {
            PrimitiveType::Identifier(_) => true,
            _ => false,
        }
    }
}

impl From<&str> for Primitive {
    fn from(value: &str) -> Self {
        Primitive {
            primitive_type: PrimitiveType::Identifier(value.to_string()),
            start_location: Location::SystemDefined,
            end_location: Location::SystemDefined,
        }
    }
}

impl From<&String> for Primitive {
    fn from(value: &String) -> Self {
        Primitive {
            primitive_type: PrimitiveType::Identifier(value.to_string()),
            start_location: Location::SystemDefined,
            end_location: Location::SystemDefined,
        }
    }
}

impl From<&str> for Listable<Primitive> {
    fn from(value: &str) -> Self {
        Primitive::from(value).into()
    }
}

impl From<&String> for Listable<Primitive> {
    fn from(value: &String) -> Self {
        Primitive::from(value).into()
    }
}
impl Into<Listable<Primitive>> for Primitive {
    fn into(self) -> Listable<Primitive> {
        Listable {
            ty: ListType::Single(self),
            start_location: Location::SystemDefined,
            end_location: Location::SystemDefined,
        }
    }
}

impl Into<Listable<Primitive>> for PrimitiveType {
    fn into(self) -> Listable<Primitive> {
        Listable {
            ty: ListType::Single(Primitive {
                primitive_type: self,
                start_location: Location::SystemDefined,
                end_location: Location::SystemDefined,
            }),
            start_location: Location::SystemDefined,
            end_location: Location::SystemDefined,
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Identifier {
    pub name: String,
    pub start_location: Location,
    pub end_location: Location,
}

#[derive(Debug, Clone, PartialEq)]
pub enum PrimitiveType {
    U32,
    U64,
    I32,
    I64,
    F32,
    F64,
    Bool,
    Char,
    Identifier(String),
}

pub fn parse(tokens: &mut Tokens) -> Result<Primitive, Error> {
    let (id, token) = tokens.pop_identifier()?;

    let ty = match id.as_str() {
        U32_ID => PrimitiveType::U32,
        U64_ID => PrimitiveType::U64,
        I32_ID => PrimitiveType::I32,
        I64_ID => PrimitiveType::I64,
        F32_ID => PrimitiveType::F32,
        F64_ID => PrimitiveType::F64,
        BOOL_ID => PrimitiveType::Bool,
        CHAR_ID => PrimitiveType::Char,
        _ => PrimitiveType::Identifier(id),
    };

    Ok(Primitive {
        primitive_type: ty,
        start_location: token.start_location,
        end_location: token.end_location,
    })
}

#[cfg(test)]
mod tests {
    use crate::lexer;

    use super::*;

    fn lex(input: &str) -> Tokens {
        let start_location = Location::default();
        let tokens = lexer::lex(input, start_location.clone()).unwrap();
        let tokens = Tokens::new(tokens, start_location);
        tokens
    }

    #[test]
    fn parse_u32() {
        let mut tokens = lex("u32");
        let primitive = parse(&mut tokens).unwrap();
        assert_eq!(
            primitive,
            Primitive {
                primitive_type: PrimitiveType::U32,
                start_location: (0, 0).into(),
                end_location: (0, 3).into(),
            }
        );
    }

    #[test]
    fn parse_u64() {
        let mut tokens = lex("u64");
        let primitive = parse(&mut tokens).unwrap();
        assert_eq!(
            primitive,
            Primitive {
                primitive_type: PrimitiveType::U64,
                start_location: (0, 0).into(),
                end_location: (0, 3).into(),
            }
        );
    }

    #[test]
    fn parse_i32() {
        let mut tokens = lex("i32");
        let primitive = parse(&mut tokens).unwrap();
        assert_eq!(
            primitive,
            Primitive {
                primitive_type: PrimitiveType::I32,
                start_location: (0, 0).into(),
                end_location: (0, 3).into(),
            }
        );
    }

    #[test]
    fn parse_i64() {
        let mut tokens = lex("i64");
        let primitive = parse(&mut tokens).unwrap();
        assert_eq!(
            primitive,
            Primitive {
                primitive_type: PrimitiveType::I64,
                start_location: (0, 0).into(),
                end_location: (0, 3).into(),
            }
        );
    }

    #[test]
    fn parse_f32() {
        let mut tokens = lex("f32");
        let primitive = parse(&mut tokens).unwrap();
        assert_eq!(
            primitive,
            Primitive {
                primitive_type: PrimitiveType::F32,
                start_location: (0, 0).into(),
                end_location: (0, 3).into(),
            }
        );
    }

    #[test]
    fn parse_f64() {
        let mut tokens = lex("f64");
        let primitive = parse(&mut tokens).unwrap();
        assert_eq!(
            primitive,
            Primitive {
                primitive_type: PrimitiveType::F64,
                start_location: (0, 0).into(),
                end_location: (0, 3).into(),
            }
        );
    }

    #[test]
    fn parse_bool() {
        let mut tokens = lex("bool");
        let primitive = parse(&mut tokens).unwrap();
        assert_eq!(
            primitive,
            Primitive {
                primitive_type: PrimitiveType::Bool,
                start_location: (0, 0).into(),
                end_location: (0, 4).into(),
            }
        );
    }

    #[test]
    fn parse_char() {
        let mut tokens = lex("char");
        let primitive = parse(&mut tokens).unwrap();
        assert_eq!(
            primitive,
            Primitive {
                primitive_type: PrimitiveType::Char,
                start_location: (0, 0).into(),
                end_location: (0, 4).into(),
            }
        );
    }

    #[test]
    fn parse_identifier() {
        let mut tokens = lex("Foo");
        let primitive = parse(&mut tokens).unwrap();
        assert_eq!(
            primitive,
            Primitive {
                primitive_type: PrimitiveType::Identifier("Foo".to_string()),
                start_location: (0, 0).into(),
                end_location: (0, 3).into(),
            }
        );
    }
}

use super::{
    is_reserved_word, parse_listable_primitive, Listable, Primitive, PrimitiveType, Tokens,
    STRUCT_ID,
};
use crate::{error::Error, lexer::TokenValue, location::Location};

#[derive(Debug, Clone, PartialEq)]
pub struct Struct {
    pub id: String,
    pub properties: StructProperties,
    pub start_location: Location,
    pub end_location: Location,
}

#[derive(Debug, Clone, PartialEq)]
pub enum StructProperties {
    None,
    Value(Listable<Primitive>),
    Multiple(Vec<StructProperty>),
}

#[derive(Debug, Clone, PartialEq)]
pub struct StructProperty {
    pub identifier: String,
    pub ty: Listable<Primitive>,
    pub start_location: Location,
    pub end_location: Location,
}

impl From<(&str, &str)> for StructProperty {
    fn from((id, ty): (&str, &str)) -> Self {
        Self {
            identifier: id.to_string(),
            ty: ty.into(),
            start_location: Location::SystemDefined,
            end_location: Location::SystemDefined,
        }
    }
}

impl From<(&str, &String)> for StructProperty {
    fn from((id, ty): (&str, &String)) -> Self {
        Self {
            identifier: id.to_string(),
            ty: ty.into(),
            start_location: Location::SystemDefined,
            end_location: Location::SystemDefined,
        }
    }
}

impl From<(&str, PrimitiveType)> for StructProperty {
    fn from((id, ty): (&str, PrimitiveType)) -> Self {
        Self {
            identifier: id.to_string(),
            ty: ty.into(),
            start_location: Location::SystemDefined,
            end_location: Location::SystemDefined,
        }
    }
}

pub fn parse(tokens: &mut Tokens) -> Result<Struct, Error> {
    let start_location;
    let end_location;

    let (struct_ty, token) = tokens.pop_identifier()?;
    start_location = token.start_location.clone();

    if struct_ty != STRUCT_ID {
        return Err(Error::new(
            format!("Expected '{STRUCT_ID}', got '{}'", struct_ty),
            token.start_location.clone(),
        ));
    }

    let (id, _) = tokens.pop_identifier()?;

    let properties = {
        // Value struct
        if tokens.peek_expected(TokenValue::LParen) {
            tokens.pop_expected(TokenValue::LParen)?;
            let property = parse_listable_primitive(tokens)?;
            tokens.pop_expected(TokenValue::RParen)?;

            let token = tokens.pop_expected(TokenValue::Semicolon)?;
            end_location = token.end_location.clone();

            StructProperties::Value(property)
        }
        // Normal struct
        else if tokens.peek_expected(TokenValue::LCurlyBrace) {
            tokens.pop_expected(TokenValue::LCurlyBrace)?;

            let mut properties = vec![];
            while !tokens.is_empty() && !tokens.peek_expected(TokenValue::RCurlyBrace) {
                let prop_type = parse_listable_primitive(tokens)?;
                let (identifier, token) = tokens.pop_identifier()?;

                let property = StructProperty {
                    identifier,
                    start_location: prop_type.start_location.clone(),
                    end_location: token.end_location.clone(),
                    ty: prop_type,
                };
                properties.push(property);
            }

            let token = tokens.pop_expected(TokenValue::RCurlyBrace)?;
            end_location = token.end_location.clone();

            if properties.is_empty() {
                StructProperties::None
            } else {
                StructProperties::Multiple(properties)
            }
        }
        // Empty struct
        else {
            let token = tokens.pop_expected(TokenValue::Semicolon)?;
            end_location = token.end_location.clone();
            StructProperties::None
        }
    };

    Ok(Struct {
        id,
        properties,
        start_location,
        end_location,
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parser::{lex, ListType, PrimitiveType, RESERVED_WORDS};

    #[test]
    fn empty_struct() {
        let input = "struct Empty;";
        let mut tokens = lex(input);
        let result = parse(&mut tokens);
        let expected = Ok(Struct {
            id: "Empty".to_string(),
            properties: StructProperties::None,
            start_location: (0, 0).into(),
            end_location: (0, 13).into(),
        });

        assert_eq!(expected, result);
    }

    #[test]
    fn empty_struct_no_semicolon_returns_err() {
        let input = "struct Empty";
        let mut tokens = lex(input);
        let result = parse(&mut tokens);
        let expected = Err(Error::new(
            "Expected ;, got nothing!".to_string(),
            (0, 12).into(),
        ));

        assert_eq!(expected, result);
    }

    #[test]
    fn empty_struct_no_closing_semicolon_returns_err() {
        let input = "struct Empty aga";
        let mut tokens = lex(input);
        let result = parse(&mut tokens);
        let expected = Err(Error::new(
            "Expected ;, got identifier: aga".to_string(),
            (0, 13).into(),
        ));

        assert_eq!(expected, result);
    }

    #[test]
    fn value_struct_no_list() {
        let input = "struct Dollar(i32);";
        let mut tokens = lex(input);
        let result = parse(&mut tokens);
        let expected = Ok(Struct {
            id: "Dollar".to_string(),
            properties: StructProperties::Value(Listable {
                ty: ListType::Single(Primitive {
                    primitive_type: PrimitiveType::I32,
                    start_location: (0, 14).into(),
                    end_location: (0, 17).into(),
                }),
                start_location: (0, 14).into(),
                end_location: (0, 17).into(),
            }),
            start_location: (0, 0).into(),
            end_location: (0, 19).into(),
        });

        assert_eq!(expected, result);
    }

    #[test]
    fn value_struct_list() {
        let input = "struct Dollar([i32 64]);";
        let mut tokens = lex(input);
        let result = parse(&mut tokens);
        let expected = Ok(Struct {
            id: "Dollar".to_string(),
            properties: StructProperties::Value(Listable {
                ty: ListType::List {
                    ty: Primitive {
                        primitive_type: PrimitiveType::I32,
                        start_location: (0, 15).into(),
                        end_location: (0, 18).into(),
                    },
                    max_size: 64,
                },
                start_location: (0, 14).into(),
                end_location: (0, 22).into(),
            }),
            start_location: (0, 0).into(),
            end_location: (0, 24).into(),
        });
        assert_eq!(expected, result);
    }

    #[test]
    fn value_struct_missing_rparen_returns_err() {
        let input = "struct Dollar([i32 64] ";
        let mut tokens = lex(input);
        let result = parse(&mut tokens);
        let expected = Err(Error::new(
            "Expected ), got nothing!".to_string(),
            (0, 22).into(),
        ));
        assert_eq!(expected, result);
    }

    #[test]
    fn value_struct_missing_semicolon_returns_err() {
        let input = "struct Dollar([i32 64]) ";
        let mut tokens = lex(input);
        let result = parse(&mut tokens);
        let expected = Err(Error::new(
            "Expected ;, got nothing!".to_string(),
            (0, 23).into(),
        ));
        assert_eq!(expected, result);
    }

    #[test]
    fn normal_no_properties_returns_empty() {
        let input = "struct Dollar {}";
        let mut tokens = lex(input);
        let result = parse(&mut tokens);
        let expected = Ok(Struct {
            id: "Dollar".to_string(),
            properties: StructProperties::None,
            start_location: (0, 0).into(),
            end_location: (0, 16).into(),
        });
        assert_eq!(expected, result);
    }

    #[test]
    fn normal_single_property() {
        let input = "struct Dollar { i32 amount }";
        let mut tokens = lex(input);
        let result = parse(&mut tokens);
        let expected = Ok(Struct {
            id: "Dollar".to_string(),
            properties: StructProperties::Multiple(vec![StructProperty {
                identifier: "amount".to_string(),
                ty: Listable {
                    ty: ListType::Single(Primitive {
                        primitive_type: PrimitiveType::I32,
                        start_location: (0, 16).into(),
                        end_location: (0, 19).into(),
                    }),
                    start_location: (0, 16).into(),
                    end_location: (0, 19).into(),
                },
                start_location: (0, 16).into(),
                end_location: (0, 26).into(),
            }]),
            start_location: (0, 0).into(),
            end_location: (0, 28).into(),
        });

        assert_eq!(expected, result);
    }

    #[test]
    fn normal_list_property() {
        let input = "struct Dollar { [i32 64] amounts }";
        let mut tokens = lex(input);
        let result = parse(&mut tokens);
        let expected = Ok(Struct {
            id: "Dollar".to_string(),
            properties: StructProperties::Multiple(vec![StructProperty {
                identifier: "amounts".to_string(),
                ty: Listable {
                    ty: ListType::List {
                        ty: Primitive {
                            primitive_type: PrimitiveType::I32,
                            start_location: (0, 17).into(),
                            end_location: (0, 20).into(),
                        },
                        max_size: 64,
                    },
                    start_location: (0, 16).into(),
                    end_location: (0, 24).into(),
                },
                start_location: (0, 16).into(),
                end_location: (0, 32).into(),
            }]),
            start_location: (0, 0).into(),
            end_location: (0, 34).into(),
        });
        assert_eq!(expected, result);
    }

    #[test]
    fn normal_multiple_props() {
        let input = "struct Dollar { i32 amount\n [i32 64] amounts }";
        let mut tokens = lex(input);
        let result = parse(&mut tokens);
        let expected = Ok(Struct {
            id: "Dollar".to_string(),
            properties: StructProperties::Multiple(vec![
                StructProperty {
                    identifier: "amount".to_string(),
                    ty: Listable {
                        ty: ListType::Single(Primitive {
                            primitive_type: PrimitiveType::I32,
                            start_location: (0, 16).into(),
                            end_location: (0, 19).into(),
                        }),
                        start_location: (0, 16).into(),
                        end_location: (0, 19).into(),
                    },
                    start_location: (0, 16).into(),
                    end_location: (0, 26).into(),
                },
                StructProperty {
                    identifier: "amounts".to_string(),
                    ty: Listable {
                        ty: ListType::List {
                            ty: Primitive {
                                primitive_type: PrimitiveType::I32,
                                start_location: (1, 2).into(),
                                end_location: (1, 5).into(),
                            },
                            max_size: 64,
                        },
                        start_location: (1, 1).into(),
                        end_location: (1, 9).into(),
                    },
                    start_location: (1, 1).into(),
                    end_location: (1, 17).into(),
                },
            ]),
            start_location: (0, 0).into(),
            end_location: (1, 19).into(),
        });

        assert_eq!(expected, result);
    }

    #[test]
    fn normal_missing_rcurly_returns_err() {
        let input = "struct Dollar { i32 amount\n [i32 64] amounts ";
        let mut tokens = lex(input);
        let result = parse(&mut tokens);
        let expected = Err(Error::new(
            "Expected }, got nothing!".to_string(),
            (1, 17).into(),
        ));

        assert_eq!(expected, result);
    }
}

use crate::{error::Error, lexer::TokenValue, location::Location};

use super::{
    is_reserved_word, parse_listable_primitive, Listable, Primitive, Tokens, COMPONENT_ID,
    SINGLE_COMPONENT_ID,
};

#[derive(Debug, Clone, PartialEq)]
pub struct Component {
    pub id: String,
    pub component_type: ComponentType,
    pub properties: ComponentProperties,
    pub start_location: Location,
    pub end_location: Location,
}

#[derive(Debug, Clone, PartialEq)]
pub enum ComponentType {
    SingleComponent,
    Component,
}

#[derive(Debug, Clone, PartialEq)]
pub enum ComponentProperties {
    None,
    Value(Listable<Primitive>),
    Multiple(Vec<ComponentProperty>),
}

#[derive(Debug, Clone, PartialEq)]
pub struct ComponentProperty {
    pub identifier: String,
    pub ty: Listable<Primitive>,
    pub start_location: Location,
    pub end_location: Location,
}

pub fn parse(tokens: &mut Tokens) -> Result<Component, Error> {
    let start_location;
    let end_location;

    let (component_type, token) = tokens.pop_identifier()?;
    start_location = token.start_location.clone();

    let component_type = match component_type.as_str() {
        COMPONENT_ID => ComponentType::Component,
        SINGLE_COMPONENT_ID => ComponentType::SingleComponent,
        other => {
            return Err(Error::new(
                format!(
                    "Expected '{COMPONENT_ID}' or '{SINGLE_COMPONENT_ID}', got '{}'",
                    other
                ),
                token.start_location.clone(),
            ))
        }
    };

    let (id, token) = tokens.pop_identifier()?;

    // Get properties
    let properties = {
        // Value type component
        if tokens.peek_expected(TokenValue::LParen) {
            tokens.pop_expected(TokenValue::LParen)?;

            let value = parse_listable_primitive(tokens)?;

            tokens.pop_expected(TokenValue::RParen)?;
            let token = tokens.pop_expected(TokenValue::Semicolon)?;
            end_location = token.end_location.clone();

            ComponentProperties::Value(value)
        }
        // Struct type component
        else if tokens.peek_expected(TokenValue::LCurlyBrace) {
            tokens.pop_expected(TokenValue::LCurlyBrace)?;

            let mut properties = vec![];
            while !tokens.is_empty() && !tokens.peek_expected(TokenValue::RCurlyBrace) {
                let prop_type = parse_listable_primitive(tokens)?;
                let (identifier, token) = tokens.pop_identifier()?;

                let property = ComponentProperty {
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
                ComponentProperties::None
            } else {
                ComponentProperties::Multiple(properties)
            }
        }
        // Tag type component
        else {
            let token = tokens.pop_expected(TokenValue::Semicolon)?;
            end_location = token.end_location.clone();
            ComponentProperties::None
        }
    };

    Ok(Component {
        id,
        properties,
        component_type,
        start_location,
        end_location,
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parser::{lex, ListType, PrimitiveType, RESERVED_WORDS};

    #[test]
    fn not_component_returns_err() {
        let input = "not_component IsAlive;";
        let mut tokens = lex(input);
        let result = parse(&mut tokens);
        let expected = Err(Error::new(
            "Expected 'component' or 'single_component', got 'not_component'".to_string(),
            (0, 0).into(),
        ));
        assert_eq!(result, expected);
    }

    #[test]
    fn component_tag_type() {
        let input = "component IsAlive;";
        let mut tokens = lex(input);
        let result = parse(&mut tokens);
        let expected = Ok(Component {
            id: "IsAlive".to_string(),
            properties: ComponentProperties::None,
            component_type: ComponentType::Component,
            start_location: (0, 0).into(),
            end_location: (0, 18).into(),
        });
        assert_eq!(result, expected);
    }

    #[test]
    fn single_component_tag_type() {
        let input = "single_component IsAlive;";
        let mut tokens = lex(input);
        let result = parse(&mut tokens);
        let expected = Ok(Component {
            id: "IsAlive".to_string(),
            properties: ComponentProperties::None,
            component_type: ComponentType::SingleComponent,
            start_location: (0, 0).into(),
            end_location: (0, 25).into(),
        });
        assert_eq!(result, expected);
    }

    #[test]
    fn tag_type_missing_semicolon_returns_err() {
        let input = "component IsAlive";
        let mut tokens = lex(input);
        let result = parse(&mut tokens);
        let expected = Err(Error::new(
            "Expected ;, got nothing!".to_string(),
            (0, 17).into(),
        ));
        assert_eq!(result, expected);
    }

    #[test]
    fn value_type_component_not_list() {
        let input = "component Hp(i32);";
        let mut tokens = lex(input);
        let result = parse(&mut tokens);
        let expected = Ok(Component {
            id: "Hp".to_string(),
            properties: ComponentProperties::Value(Listable {
                ty: ListType::Single(Primitive {
                    primitive_type: PrimitiveType::I32,
                    start_location: (0, 13).into(),
                    end_location: (0, 16).into(),
                }),
                start_location: (0, 13).into(),
                end_location: (0, 16).into(),
            }),
            component_type: ComponentType::Component,
            start_location: (0, 0).into(),
            end_location: (0, 18).into(),
        });

        assert_eq!(expected, result);
    }

    #[test]
    fn value_type_component_list() {
        let input = "component Hp([i32 100]);";
        let mut tokens = lex(input);
        let result = parse(&mut tokens);
        let expected = Ok(Component {
            id: "Hp".to_string(),
            properties: ComponentProperties::Value(Listable {
                ty: ListType::List {
                    ty: Primitive {
                        primitive_type: PrimitiveType::I32,
                        start_location: (0, 14).into(),
                        end_location: (0, 17).into(),
                    },
                    max_size: 100,
                },
                start_location: (0, 13).into(),
                end_location: (0, 22).into(),
            }),
            component_type: ComponentType::Component,
            start_location: (0, 0).into(),
            end_location: (0, 24).into(),
        });

        assert_eq!(expected, result);
    }

    #[test]
    fn value_type_component_missing_rparen_returns_err() {
        let input = "component Hp([i32 100]";
        let mut tokens = lex(input);
        let result = parse(&mut tokens);
        let expected = Err(Error::new(
            "Expected ), got nothing!".to_string(),
            (0, 22).into(),
        ));

        assert_eq!(expected, result);
    }

    #[test]
    fn value_type_component_missing_semicolon_returns_err() {
        let input = "component Hp([i32 100]) a";
        let mut tokens = lex(input);
        let result = parse(&mut tokens);
        let expected = Err(Error::new(
            "Expected ;, got identifier: a".to_string(),
            (0, 24).into(),
        ));

        assert_eq!(expected, result);
    }

    #[test]
    fn struct_type_component_empty_props_returns_tag_type() {
        let input = "component Hp{}";
        let mut tokens = lex(input);
        let result = parse(&mut tokens);
        let expected = Ok(Component {
            id: "Hp".to_string(),
            properties: ComponentProperties::None,
            component_type: ComponentType::Component,
            start_location: (0, 0).into(),
            end_location: (0, 14).into(),
        });

        assert_eq!(expected, result);
    }

    #[test]
    fn struct_type_component_single_props() {
        let input = "component Hp{ i32 value}";
        let mut tokens = lex(input);
        let result = parse(&mut tokens);
        let expected = Ok(Component {
            id: "Hp".to_string(),
            properties: ComponentProperties::Multiple(vec![ComponentProperty {
                identifier: "value".to_string(),
                ty: Listable {
                    ty: ListType::Single(Primitive {
                        primitive_type: PrimitiveType::I32,
                        start_location: (0, 14).into(),
                        end_location: (0, 17).into(),
                    }),
                    start_location: (0, 14).into(),
                    end_location: (0, 17).into(),
                },
                start_location: (0, 14).into(),
                end_location: (0, 23).into(),
            }]),
            component_type: ComponentType::Component,
            start_location: (0, 0).into(),
            end_location: (0, 24).into(),
        });

        assert_eq!(expected, result);
    }

    #[test]
    fn struct_type_component_listable_props() {
        let input = "component Hp{ [i32 64] value}";
        let mut tokens = lex(input);
        let result = parse(&mut tokens);
        let expected = Ok(Component {
            id: "Hp".to_string(),
            properties: ComponentProperties::Multiple(vec![ComponentProperty {
                identifier: "value".to_string(),
                ty: Listable {
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
                },
                start_location: (0, 14).into(),
                end_location: (0, 28).into(),
            }]),
            component_type: ComponentType::Component,
            start_location: (0, 0).into(),
            end_location: (0, 29).into(),
        });

        assert_eq!(expected, result);
    }

    #[test]
    fn struct_type_component_multiple_props() {
        let input = "component Position { i32 x \n i32 y}";
        let mut tokens = lex(input);
        let result = parse(&mut tokens);
        let expected = Ok(Component {
            id: "Position".to_string(),
            properties: ComponentProperties::Multiple(vec![
                ComponentProperty {
                    identifier: "x".to_string(),
                    ty: Listable {
                        ty: ListType::Single(Primitive {
                            primitive_type: PrimitiveType::I32,
                            start_location: (0, 21).into(),
                            end_location: (0, 24).into(),
                        }),
                        start_location: (0, 21).into(),
                        end_location: (0, 24).into(),
                    },
                    start_location: (0, 21).into(),
                    end_location: (0, 26).into(),
                },
                ComponentProperty {
                    identifier: "y".to_string(),
                    ty: Listable {
                        ty: ListType::Single(Primitive {
                            primitive_type: PrimitiveType::I32,
                            start_location: (1, 1).into(),
                            end_location: (1, 4).into(),
                        }),
                        start_location: (1, 1).into(),
                        end_location: (1, 4).into(),
                    },
                    start_location: (1, 1).into(),
                    end_location: (1, 6).into(),
                },
            ]),
            component_type: ComponentType::Component,
            start_location: (0, 0).into(),
            end_location: (1, 7).into(),
        });

        assert_eq!(expected, result);
    }
}

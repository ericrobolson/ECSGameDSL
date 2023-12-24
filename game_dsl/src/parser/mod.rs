mod comment;
mod component;
mod listable;
mod primitives;
mod strukt;
mod tokens;

use crate::lexer::{self, Token, TokenValue};
use crate::{error::Error, location::Location};
pub use comment::*;
pub use component::*;
pub use listable::*;
pub use primitives::*;
pub use strukt::*;
pub use tokens::*;

pub const COMPONENT_ID: &str = "component";
pub const SINGLE_COMPONENT_ID: &str = "single_component";
pub const STRUCT_ID: &str = "struct";
pub const U32_ID: &str = "u32";
pub const U64_ID: &str = "u64";
pub const I32_ID: &str = "i32";
pub const I64_ID: &str = "i64";
pub const F32_ID: &str = "f32";
pub const F64_ID: &str = "f64";
pub const BOOL_ID: &str = "bool";
pub const CHAR_ID: &str = "char";

pub const RESERVED_WORDS: [&str; 11] = [
    COMPONENT_ID,
    SINGLE_COMPONENT_ID,
    STRUCT_ID,
    U32_ID,
    U64_ID,
    I32_ID,
    I64_ID,
    F32_ID,
    F64_ID,
    BOOL_ID,
    CHAR_ID,
];

pub fn is_reserved_word(id: &str) -> bool {
    for reserved_word in RESERVED_WORDS.iter() {
        if id == *reserved_word {
            return true;
        }
    }

    false
}

pub fn parse(code: &str, start_location: Location) -> Result<Vec<Ast>, Error> {
    let tokens = lexer::lex(code, start_location.clone())?;

    let mut asts = vec![];
    let mut tokens = Tokens::new(tokens, start_location.clone());

    while let Some(token) = tokens.pop() {
        match &token.value {
            TokenValue::Identifier(i) => {
                if i == COMPONENT_ID || i == SINGLE_COMPONENT_ID {
                    tokens.insert_head(token.clone());

                    let component = component::parse(&mut tokens)?;
                    asts.push(Ast::Component(component));
                } else if i == STRUCT_ID {
                    tokens.insert_head(token.clone());

                    let strukt = strukt::parse(&mut tokens)?;
                    asts.push(Ast::Struct(strukt));
                } else {
                    let token = tokens.pop().unwrap();
                    return Err(Error::new(
                        format!("Unexpected identifier {:?}", i),
                        token.start_location.clone(),
                    ));
                }
            }
            TokenValue::Comment(lines) => {
                let comment = Comment {
                    lines: lines.clone(),
                    start_location: token.start_location.clone(),
                    end_location: token.end_location.clone(),
                };
                asts.push(Ast::Comment(comment));
            }
            _ => {
                return Err(Error::new(
                    format!("Unexpected token {:?}", token),
                    token.start_location.clone(),
                ));
            }
        }
    }

    Ok(asts)
}

#[cfg(test)]
pub fn lex(code: &str) -> Tokens {
    Tokens::new(
        lexer::lex(code, Location::default()).unwrap(),
        Location::default(),
    )
}

#[derive(Debug, Clone, PartialEq)]
pub enum Ast {
    Component(Component),
    Comment(Comment),
    Struct(Struct),
}

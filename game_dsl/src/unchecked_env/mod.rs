use crate::parser::{Ast, Component, Struct};

#[derive(Debug, Clone, PartialEq)]
pub struct UncheckedEnv {
    pub components: Vec<Component>,
    pub structs: Vec<Struct>,
}

pub fn build(asts: Vec<Ast>) -> UncheckedEnv {
    let mut env = UncheckedEnv {
        components: vec![],
        structs: vec![],
    };

    for ast in asts {
        match ast {
            Ast::Component(c) => {
                env.components.push(c);
            }
            Ast::Comment(c) => {
                // skip for now
            }
            Ast::Struct(s) => {
                env.structs.push(s);
            }
        }
    }

    env
}

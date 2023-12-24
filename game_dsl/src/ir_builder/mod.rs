use std::error;

use crate::{
    compiler_ir::{Commentable, Struct, StructField, IR},
    env::{Env, StructValue},
    error::Error,
    parser::{ComponentProperties, ListType, Primitive, PrimitiveType, StructProperties},
};

pub fn build(env: Env) -> Result<Vec<IR>, Vec<Error>> {
    let mut ir = vec![];
    let mut errors = vec![];

    build_components(&env, &mut ir);
    build_structs(&env, &mut ir);

    if errors.is_empty() {
        Ok(ir)
    } else {
        Err(errors)
    }
}

fn build_structs(env: &Env, ir: &mut Vec<IR>) {
    for (id, strukt) in env.structs.iter() {
        let use_components = match strukt {
            StructValue::ComponentStore(_) => true,
            StructValue::Struct(_) => false,
        };
        let strukt = strukt.strukt();
        let mut fields = vec![];

        match &strukt.properties {
            StructProperties::None => {}
            StructProperties::Value(v) => {
                fields.push(StructField {
                    id: "value".to_string(),
                    ty: build_list_type(&v.ty, use_components),
                });
            }
            StructProperties::Multiple(properties) => {
                for prop in properties {
                    let id = prop.identifier.to_string();
                    let ty = build_list_type(&prop.ty.ty, use_components);
                    fields.push(StructField { id, ty });
                }
            }
        }

        ir.push(IR::Struct(Commentable {
            comments: vec![],
            value: Struct {
                id: struct_id(&id),
                fields,
            },
        }));
    }
}

fn build_components(env: &Env, ir: &mut Vec<IR>) {
    for (id, component) in env.components.iter() {
        let mut fields = vec![];

        match &component.properties {
            ComponentProperties::None => {}
            ComponentProperties::Value(v) => {
                fields.push(StructField {
                    id: "value".to_string(),
                    ty: build_list_type(&v.ty, false),
                });
            }
            ComponentProperties::Multiple(properties) => {
                for prop in properties {
                    let id = prop.identifier.to_string();
                    let ty = build_list_type(&prop.ty.ty, false);
                    fields.push(StructField { id, ty });
                }
            }
        }

        let s = Struct {
            id: component_id(&id),
            fields,
        };
        ir.push(IR::Struct(Commentable {
            comments: vec![],
            value: s,
        }));

        // Build out ComponentManager IR class
    }
}

fn build_list_type(ty: &ListType<Primitive>, use_components: bool) -> ListType<Primitive> {
    match ty {
        ListType::Single(ty) => ListType::Single(build_primitive(ty, use_components)),
        ListType::List { ty, max_size } => ListType::List {
            ty: build_primitive(ty, use_components),
            max_size: *max_size,
        },
    }
}

/// Properly casts the primitive and updates the id.
fn build_primitive(primitive: &Primitive, use_components: bool) -> Primitive {
    if let PrimitiveType::Identifier(id) = &primitive.primitive_type {
        Primitive {
            primitive_type: PrimitiveType::Identifier(if use_components {
                component_id(id)
            } else {
                struct_id(id)
            }),
            ..primitive.clone()
        }
    } else {
        primitive.clone()
    }
}

fn component_id(id: &str) -> String {
    format!("D_COMPONENT_{}", id.to_uppercase())
}

fn struct_id(id: &str) -> String {
    format!("D_STRUCT_{}", id.to_uppercase())
}

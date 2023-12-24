#[cfg(test)]
mod tests;

use crate::{
    error::Error,
    location::Location,
    parser::{
        is_reserved_word, Component, ComponentProperties, ListType, Listable, Primitive,
        PrimitiveType, Struct, StructProperties, StructProperty,
    },
    unchecked_env::UncheckedEnv,
};
use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq)]
pub struct Env {
    pub components: HashMap<String, Component>,
    pub structs: HashMap<String, StructValue>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum StructValue {
    Struct(Struct),
    ComponentStore(Struct),
}
impl StructValue {
    pub fn strukt(&self) -> &Struct {
        match self {
            StructValue::Struct(strukt) => strukt,
            StructValue::ComponentStore(strukt) => strukt,
        }
    }
}

fn finalize_errors(mut errors: Vec<Error>) -> Vec<Error> {
    errors.dedup();
    errors.sort_by(|a, b| {
        // Sort by line number
        // then by column number
        // TODO: files
        if a.location.line() == b.location.line() {
            a.location.column().cmp(&b.location.column())
        } else {
            a.location.line().cmp(&b.location.line())
        }
    });
    errors
}

// Build out a checked environment from an unchecked environment.
// Return a list of errors.
pub fn build(unchecked: UncheckedEnv) -> Result<Env, Vec<Error>> {
    let mut env = Env {
        components: HashMap::new(),
        structs: HashMap::new(),
    };

    let mut errors = vec![];

    // Assemble components
    for component in unchecked.components {
        if let Some(existing) = env.components.get(&component.id) {
            let mut error = Error {
                message: format!("Multiple component definitions for '{}'", component.id),
                location: component.start_location,
            };
            errors.push(error.clone());
            error.location = existing.start_location.clone();
            errors.push(error);
        } else if is_reserved_word(&component.id) {
            errors.push(Error {
                message: format!("Component id '{}' is a reserved word", component.id),
                location: component.start_location,
            });
        } else {
            env.components.insert(component.id.clone(), component);
        }
    }

    // Assemble structs and struct types
    for strukt in unchecked.structs {
        if let Some(existing) = env.structs.get(&strukt.id) {
            let existing = existing.strukt();
            let mut error = Error {
                message: format!("Multiple struct definitions for '{}'", strukt.id),
                location: strukt.start_location,
            };
            errors.push(error.clone());
            error.location = existing.start_location.clone();
            errors.push(error);
        } else if is_reserved_word(&strukt.id) {
            errors.push(Error {
                message: format!("Struct id '{}' is a reserved word", strukt.id),
                location: strukt.start_location,
            });
        } else {
            env.structs
                .insert(strukt.id.clone(), StructValue::Struct(strukt));
        }
    }

    // TODO: look into abstracting the above checks into a method
    // TODO: assemble systems
    // TODO: assemble world
    // TODO: assemble expressions

    // No point to attempting to validate further as we need properly declarations for additional type checking..
    if !errors.is_empty() {
        return Err(finalize_errors(errors));
    }

    // Build out default implementations
    build_native_structures(&mut env)?;
    build_native_expressions(&mut env)?;

    if !errors.is_empty() {
        return Err(finalize_errors(errors));
    }

    // Validate components
    for component in env.components.values() {
        match &component.properties {
            ComponentProperties::None => {}
            ComponentProperties::Value(value) => {
                // Check that the type for value exists if it's an identifier
                if let PrimitiveType::Identifier(id) = &value.inner_ty().primitive_type {
                    if !env.structs.contains_key(id) {
                        errors.push(Error {
                            message: format!(
                                "Referenced struct type '{}' does not exist for value component '{}'",
                                id, component.id
                            ),
                            location: value.start_location.clone(),
                        });
                    }
                }
            }
            ComponentProperties::Multiple(properties) => {
                for (idx, property) in properties.iter().enumerate() {
                    // Check that the property id is not a reserved word
                    if is_reserved_word(&property.identifier) {
                        errors.push(Error {
                            message: format!(
                                "Property identifier '{}' is a reserved word for component '{}'",
                                property.identifier, component.id
                            ),
                            location: property.start_location.clone(),
                        });
                    }

                    // Check that duplicate property ids don't exist
                    for (idx2, property2) in properties.iter().enumerate() {
                        if idx == idx2 {
                            continue;
                        }

                        if property.identifier == property2.identifier {
                            errors.push(Error {
                                message: format!(
                                    "Duplicate property identifier '{}' for component '{}'",
                                    property.identifier, component.id
                                ),
                                location: property.start_location.clone(),
                            });
                        }
                    }

                    // Check that the type for value exists if it's an identifier
                    let prop_type = &property.ty.inner_ty().primitive_type;
                    if let PrimitiveType::Identifier(ty_id) = prop_type {
                        if !env.structs.contains_key(ty_id) {
                            errors.push(Error {
                                message: format!(
                                    "Referenced struct type '{}' does not exist for property '{}' for component '{}'",
                                    ty_id, property.identifier, component.id
                                ),
                                location: property.start_location.clone(),
                            });
                        }
                    }
                }
            }
        }
    }

    // Validate structs
    for strukt in env.structs.values() {
        let check_id = match strukt {
            StructValue::Struct(_) => struct_exists,
            StructValue::ComponentStore(_) => component_exists,
        };
        let strukt = strukt.strukt();

        match &strukt.properties {
            StructProperties::None => {}
            StructProperties::Value(value) => {
                // Check that the type for value exists if it's an identifier
                if let PrimitiveType::Identifier(id) = &value.inner_ty().primitive_type {
                    if !check_id(&env, id) {
                        errors.push(Error {
                            message: format!(
                                "Referenced struct type '{}' does not exist for value struct '{}'",
                                id, strukt.id
                            ),
                            location: value.start_location.clone(),
                        });
                    }
                }
            }
            StructProperties::Multiple(properties) => {
                for (idx, property) in properties.iter().enumerate() {
                    // Check that the property id is not a reserved word
                    if is_reserved_word(&property.identifier) {
                        errors.push(Error {
                            message: format!(
                                "Property identifier '{}' is a reserved word for struct '{}'",
                                property.identifier, strukt.id
                            ),
                            location: property.start_location.clone(),
                        });
                    }

                    // Check that duplicate property ids don't exist
                    for (idx2, property2) in properties.iter().enumerate() {
                        if idx == idx2 {
                            continue;
                        }

                        if property.identifier == property2.identifier {
                            errors.push(Error {
                                message: format!(
                                    "Duplicate property identifier '{}' for struct '{}'",
                                    property.identifier, strukt.id
                                ),
                                location: property.start_location.clone(),
                            });
                        }
                    }

                    // Check that the type for value exists if it's an identifier
                    let prop_type = &property.ty.inner_ty().primitive_type;
                    if let PrimitiveType::Identifier(ty_id) = prop_type {
                        if !check_id(&env, ty_id) {
                            errors.push(Error {
                                message: format!(
                                    "Referenced struct type '{}' does not exist for property '{}' for struct '{}'",
                                    ty_id, property.identifier, strukt.id
                                ),
                                location: property.start_location.clone(),
                            });
                        }
                    }
                }
            }
        }
    }

    // TODO: validate expressions
    // TODO: validate systems
    // TODO: validate world

    if errors.is_empty() {
        Ok(env)
    } else {
        return Err(finalize_errors(errors));
    }
}

fn struct_exists(env: &Env, id: &str) -> bool {
    env.structs.contains_key(id)
}

fn component_exists(env: &Env, id: &str) -> bool {
    env.components.contains_key(id)
}

fn build_native_structures(env: &mut Env) -> Result<(), Vec<Error>> {
    let entity = StructValue::Struct(Struct {
        id: "Entity".to_string(),
        properties: StructProperties::Value(Listable {
            ty: ListType::Single(Primitive {
                primitive_type: PrimitiveType::U64,
                start_location: Location::SystemDefined,
                end_location: Location::SystemDefined,
            }),
            start_location: Location::SystemDefined,
            end_location: Location::SystemDefined,
        }),
        start_location: Location::SystemDefined,
        end_location: Location::SystemDefined,
    });

    let mut errors = vec![];
    let mut system_structs = vec![entity];

    // Build out component stores
    for component in env.components.values() {
        // properties:
        // active_components - usize
        // components - a list of components
        // entities - a list of entities that maps to the components
        let max_components = 1024;
        let component_store = Struct {
            id: format!("{}_Store", component.id),
            properties: StructProperties::Multiple(vec![
                ("active_components", PrimitiveType::U64).into(),
                ("components", &component.id).into(),
            ]),
            start_location: Location::SystemDefined,
            end_location: Location::SystemDefined,
        };

        system_structs.push(StructValue::ComponentStore(component_store));
    }

    // Check structs
    for strukt in system_structs {
        let s = strukt.strukt();
        if let Some(existing) = env.structs.get(&s.id) {
            let existing = existing.strukt();
            let error = Error {
                message: format!("User implementation found for native struct '{}'", s.id),
                location: existing.start_location.clone(),
            };
            errors.push(error);
        } else {
            env.structs.insert(s.id.clone(), strukt);
        }
    }

    if errors.is_empty() {
        Ok(())
    } else {
        Err(errors)
    }
}

fn build_native_expressions(env: &mut Env) -> Result<(), Vec<Error>> {
    Ok(())
}

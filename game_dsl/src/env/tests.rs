use crate::{error::Error, parser::parse, unchecked_env};

use super::*;

fn build(code: &str) -> Result<Env, Vec<Error>> {
    let ast = parse(code, (0, 0).into())?;
    let unchecked_env = unchecked_env::build(ast);
    super::build(unchecked_env)
}

#[test]
fn entity_struct_returns_err() {
    let input = "struct Entity {}";
    let result = build(input);
    let expected = Err(vec![Error {
        message: "User implementation found for native struct 'Entity'".to_string(),
        location: (0, 0).into(),
    }]);

    assert_eq!(expected, result);
}

#[test]
fn duplicate_component_ids_returns_errors() {
    let input = "component A; component A;";
    let result = build(input);
    let expected = Err(vec![
        Error {
            message: "Multiple component definitions for 'A'".to_string(),
            location: (0, 0).into(),
        },
        Error {
            message: "Multiple component definitions for 'A'".to_string(),
            location: (0, 13).into(),
        },
    ]);

    assert_eq!(result, expected);
}

#[test]
fn reserved_component_id_returns_errors() {
    let input = "component u32; component i32;";
    let result = build(input);
    let expected = Err(vec![
        Error {
            message: "Component id 'u32' is a reserved word".to_string(),
            location: (0, 0).into(),
        },
        Error {
            message: "Component id 'i32' is a reserved word".to_string(),
            location: (0, 15).into(),
        },
    ]);

    assert_eq!(expected, result);
}

#[test]
fn value_component_property_type_is_missing_returns_err() {
    let input = "component Bob(NotExists);";
    let result = build(input);
    let expected = Err(vec![Error {
        message: "Referenced struct type 'NotExists' does not exist for value component 'Bob'"
            .to_string(),
        location: (0, 14).into(),
    }]);

    assert_eq!(expected, result);
}

#[test]
fn struct_component_property_type_is_missing_returns_err() {
    let input = "component Bob {NotExists property }";
    let result = build(input);
    let expected = Err(vec![Error {
        message: "Referenced struct type 'NotExists' does not exist for property 'property' for component 'Bob'"
            .to_string(),
        location: (0, 15).into(),
    }]);

    assert_eq!(expected, result);
}

#[test]
fn struct_component_property_id_is_reserved_returns_err() {
    let input = "component Bob {i32 i32 \ni32 u32 }";
    let result = build(input);
    let expected = Err(vec![
        Error {
            message: "Property identifier 'i32' is a reserved word for component 'Bob'".to_string(),
            location: (0, 15).into(),
        },
        Error {
            message: "Property identifier 'u32' is a reserved word for component 'Bob'".to_string(),
            location: (1, 0).into(),
        },
    ]);

    assert_eq!(expected, result);
}

#[test]
fn struct_component_property_id_duplicate_returns_err() {
    let input = "component Bob {i32 property \ni32 property }";
    let result = build(input);
    let expected = Err(vec![
        Error {
            message: "Duplicate property identifier 'property' for component 'Bob'".to_string(),
            location: (0, 15).into(),
        },
        Error {
            message: "Duplicate property identifier 'property' for component 'Bob'".to_string(),
            location: (1, 0).into(),
        },
    ]);

    assert_eq!(expected, result);
}

#[test]
fn duplicate_structs_return_errors() {
    let input = "struct A; struct A;";
    let result = build(input);
    let expected = Err(vec![
        Error {
            message: "Multiple struct definitions for 'A'".to_string(),
            location: (0, 0).into(),
        },
        Error {
            message: "Multiple struct definitions for 'A'".to_string(),
            location: (0, 10).into(),
        },
    ]);

    assert_eq!(result, expected);
}

#[test]
fn reserved_struct_id_returns_errors() {
    let input = "struct u32; struct i32;";
    let result = build(input);
    let expected = Err(vec![
        Error {
            message: "Struct id 'u32' is a reserved word".to_string(),
            location: (0, 0).into(),
        },
        Error {
            message: "Struct id 'i32' is a reserved word".to_string(),
            location: (0, 12).into(),
        },
    ]);

    assert_eq!(expected, result);
}

#[test]
fn value_struct_property_type_is_missing_returns_err() {
    let input = "struct Bob(NotExists);";
    let result = build(input);
    let expected = Err(vec![Error {
        message: "Referenced struct type 'NotExists' does not exist for value struct 'Bob'"
            .to_string(),
        location: (0, 11).into(),
    }]);

    assert_eq!(expected, result);
}

#[test]
fn struct_struct_property_type_is_missing_returns_err() {
    let input = "struct Bob {NotExists property }";
    let result = build(input);
    let expected = Err(vec![Error {
        message: "Referenced struct type 'NotExists' does not exist for property 'property' for struct 'Bob'"
            .to_string(),
        location: (0, 12).into(),
    }]);

    assert_eq!(expected, result);
}

#[test]
fn struct_struct_property_id_is_reserved_returns_err() {
    let input = "struct Bob {i32 i32 \ni32 u32 }";
    let result = build(input);
    let expected = Err(vec![
        Error {
            message: "Property identifier 'i32' is a reserved word for struct 'Bob'".to_string(),
            location: (0, 12).into(),
        },
        Error {
            message: "Property identifier 'u32' is a reserved word for struct 'Bob'".to_string(),
            location: (1, 0).into(),
        },
    ]);

    assert_eq!(expected, result);
}

#[test]
fn struct_struct_property_id_duplicate_returns_err() {
    let input = "struct Bob {i32 property \ni32 property }";
    let result = build(input);
    let expected = Err(vec![
        Error {
            message: "Duplicate property identifier 'property' for struct 'Bob'".to_string(),
            location: (0, 12).into(),
        },
        Error {
            message: "Duplicate property identifier 'property' for struct 'Bob'".to_string(),
            location: (1, 0).into(),
        },
    ]);

    assert_eq!(expected, result);
}

#![cfg(all(test, windows))]

#[allow(
    non_snake_case,
    non_upper_case_globals,
    non_camel_case_types,
    dead_code,
    clippy::all
)]
mod bindings;
use bindings::*;
use windows::{Win32::Foundation::*, core::*};

#[test]
fn test() -> Result<()> {
    // Create a validator with the given schema.
    let validator = JsonValidator::CreateInstance(h!(r#"{"maxLength": 5}"#))?;

    // Validate the (valid) json against the schema.
    validator.Validate(h!(r#""Hello""#))?;

    // Check validation failure provides reasonable error information.
    let error = validator.Validate(h!(r#""Hello World""#)).unwrap_err();
    assert_eq!(error.code(), E_INVALIDARG);
    assert_eq!(
        error.message(),
        r#""Hello World" is longer than 5 characters"#,
    );

    // Check schema parsing failure provides reasonable error information.
    let error = JsonValidator::CreateInstance(h!(r#"{ "invalid"#)).unwrap_err();
    assert_eq!(error.code(), E_INVALIDARG);
    assert_eq!(
        error.message(),
        "EOF while parsing a string at line 1 column 10",
    );

    // Check that JSON parsing failure provides reasonable error information.
    let error = validator.Validate(h!(r#""Hello"#)).unwrap_err();
    assert_eq!(error.code(), E_INVALIDARG);
    assert_eq!(
        error.message(),
        "EOF while parsing a string at line 1 column 6",
    );

    Ok(())
}

#[test]
fn sanitized_value() -> Result<()> {
    let schema = h!(r#"
    {
        "properties": {
            "name": {
                "type": "string"
            },
            "age": {
                "type": "integer"
            }
        }
    }
    "#);

    let value = h!(r#"
    {
        "name": "Kenny",
        "age": 21 
    }
    "#);

    // Create a validator with the given schema.
    let validator = JsonValidator::CreateInstance(schema)?;

    // Validate and check the sanitized return value.
    let sanitized = validator.Validate(value)?;
    assert_eq!(sanitized, r#"{"age":21,"name":"Kenny"}"#);
    Ok(())
}

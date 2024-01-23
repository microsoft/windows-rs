use jsonschema::JSONSchema;
use windows::{core::*, Win32::Foundation::*};

// Creates a JSON validator object with the given schema. The returned handle must be freed
// by calling `CloseJsonValidator`.
#[no_mangle]
extern "system" fn CreateJsonValidator(
    schema: *const u8,
    schema_len: usize,
    handle: *mut usize,
) -> HRESULT {
    create_validator(schema, schema_len, handle).into()
}

// Validates a JSON value against a previously-compiled schema.
#[no_mangle]
extern "system" fn ValidateJson(handle: usize, value: *const u8, value_len: usize) -> HRESULT {
    validate(handle, value, value_len).into()
}

// Closes a JSON validator object.
#[no_mangle]
extern "system" fn CloseJsonValidator(handle: usize) {
    if handle != 0 {
        unsafe {
            _ = Box::from_raw(handle as *mut JSONSchema);
        }
    }
}

// Implementation of the `CreateJsonValidator` function so we can use `Result` for simplicity.
fn create_validator(schema: *const u8, schema_len: usize, handle: *mut usize) -> Result<()> {
    let schema = json_from_raw_parts(schema, schema_len)?;

    match JSONSchema::compile(&schema) {
        Ok(compiled) => {
            if handle.is_null() {
                return Err(E_POINTER.into());
            }

            // The handle is not null so we can safely dereference it here.
            unsafe {
                *handle = Box::into_raw(Box::new(compiled)) as usize;
            }

            Ok(())
        }
        Err(error) => Err(Error::new(E_INVALIDARG, error.to_string().into())),
    }
}

// Implementation of the `ValidateJson` function so we can use `Result` for simplicity.
fn validate(handle: usize, value: *const u8, value_len: usize) -> Result<()> {
    if handle == 0 {
        return Err(E_HANDLE.into());
    }

    let value = json_from_raw_parts(value, value_len)?;

    // This looks a bit tricky but we're just turning the opaque handle into `JSONSchema` pointer
    // and then returning a reference to avoid taking ownership of it.
    let schema = unsafe { &*(handle as *const JSONSchema) };

    if schema.is_valid(&value) {
        Ok(())
    } else {
        let mut message = String::new();

        // The `validate` method returns a collection of errors. We'll just return the first
        // for simplicity.
        if let Some(error) = schema.validate(&value).unwrap_err().next() {
            message = error.to_string();
        }

        Err(Error::new(E_INVALIDARG, message.into()))
    }
}

// Takes care of all the JSON parsing and parameter validation.
fn json_from_raw_parts(value: *const u8, value_len: usize) -> Result<serde_json::Value> {
    if value.is_null() {
        return Err(E_POINTER.into());
    }

    let value = unsafe { std::slice::from_raw_parts(value, value_len) };

    let Ok(value) = std::str::from_utf8(value) else {
        return Err(ERROR_NO_UNICODE_TRANSLATION.into());
    };

    match serde_json::from_str(value) {
        Ok(value) => Ok(value),
        Err(error) => Err(Error::new(E_INVALIDARG, format!("{error}").into())),
    }
}

#[test]
fn simple() {
    // Create a validator with the given schema.
    let schema = r#"{"maxLength": 5}"#;
    let mut handle = 0;
    assert_eq!(
        S_OK,
        CreateJsonValidator(schema.as_ptr(), schema.len(), &mut handle)
    );

    // Validate the json against the schema.
    let value = r#""Hello""#;
    assert_eq!(S_OK, ValidateJson(handle, value.as_ptr(), value.len()));

    // Check check validation failure provides reasonable error information.
    let value = r#""Hello World""#;
    let code = ValidateJson(handle, value.as_ptr(), value.len());
    assert_eq!(E_INVALIDARG, code);
    assert_eq!(
        r#""Hello World" is longer than 5 characters"#,
        Error::from(code).message()
    );

    // The schema validator is reusable.
    let value = r#""World""#;
    assert_eq!(S_OK, ValidateJson(handle, value.as_ptr(), value.len()));

    // Close the validator with the given handle.
    CloseJsonValidator(handle);

    // Closing a "zero" handle is harmless.
    CloseJsonValidator(0);
}

#[test]
fn invalid_create_params() {
    // Check schema parsing failure provides reasonable error information.
    let schema = r#"{ "invalid"#;
    let mut handle = 0;
    let code = CreateJsonValidator(schema.as_ptr(), schema.len(), &mut handle);
    assert_eq!(E_INVALIDARG, code);
    assert_eq!(
        "EOF while parsing a string at line 1 column 10",
        Error::from(code).message()
    );

    // Check that schema null pointer is caught.
    let schema = r#"{"maxLength": 5}"#;
    let mut handle = 0;
    assert_eq!(
        E_POINTER,
        CreateJsonValidator(std::ptr::null(), schema.len(), &mut handle)
    );

    // Check that handle null pointer is caught.
    assert_eq!(
        E_POINTER,
        CreateJsonValidator(schema.as_ptr(), schema.len(), std::ptr::null_mut())
    );
}

#[test]
fn invalid_validate_params() {
    // Create a validator with the given schema.
    let schema = r#"{"maxLength": 5}"#;
    let mut handle = 0;
    assert_eq!(
        S_OK,
        CreateJsonValidator(schema.as_ptr(), schema.len(), &mut handle)
    );

    // Check that a zero handle is caught.
    let value = r#""Hello""#;
    assert_eq!(E_HANDLE, ValidateJson(0, value.as_ptr(), value.len()));

    // Check that a value null pointer is caught.
    assert_eq!(
        E_POINTER,
        ValidateJson(handle, std::ptr::null(), value.len())
    );

    // Check that JSON parsing failure provides reasonable error information.
    let value = r#""Hello"#;
    let code = ValidateJson(handle, value.as_ptr(), value.len());
    assert_eq!(E_INVALIDARG, code);
    assert_eq!(
        "EOF while parsing a string at line 1 column 6",
        Error::from(code).message()
    );

    // Close the validator with the given handle.
    CloseJsonValidator(handle);
}

use jsonschema::Validator;
use windows::{core::*, Win32::Foundation::*, Win32::System::Com::*};

// Creates a JSON validator object with the given schema. The returned handle must be freed
// by calling `CloseJsonValidator`.
#[no_mangle]
unsafe extern "system" fn CreateJsonValidator(
    schema: *const u8,
    schema_len: usize,
    handle: *mut usize,
) -> HRESULT {
    unsafe { create_validator(schema, schema_len, handle).into() }
}

// Validates a JSON value against a previously-compiled schema.
#[no_mangle]
unsafe extern "system" fn ValidateJson(
    handle: usize,
    value: *const u8,
    value_len: usize,
    sanitized_value: *mut *mut u8,
    sanitized_value_len: *mut usize,
) -> HRESULT {
    unsafe {
        validate(
            handle,
            value,
            value_len,
            sanitized_value,
            sanitized_value_len,
        )
        .into()
    }
}

// Closes a JSON validator object.
#[no_mangle]
unsafe extern "system" fn CloseJsonValidator(handle: usize) {
    unsafe {
        if handle != 0 {
            _ = Box::from_raw(handle as *mut Validator);
        }
    }
}

// Implementation of the `CreateJsonValidator` function so we can use `Result` for simplicity.
unsafe fn create_validator(schema: *const u8, schema_len: usize, handle: *mut usize) -> Result<()> {
    unsafe {
        let schema = json_from_raw_parts(schema, schema_len)?;

        let compiled =
            Validator::new(&schema).map_err(|error| Error::new(E_INVALIDARG, error.to_string()))?;

        if handle.is_null() {
            return Err(E_POINTER.into());
        }

        // The handle is not null so we can safely dereference it here.
        *handle = Box::into_raw(Box::new(compiled)) as usize;

        Ok(())
    }
}

// Implementation of the `ValidateJson` function so we can use `Result` for simplicity.
unsafe fn validate(
    handle: usize,
    value: *const u8,
    value_len: usize,
    sanitized_value: *mut *mut u8,
    sanitized_value_len: *mut usize,
) -> Result<()> {
    unsafe {
        if handle == 0 {
            return Err(E_HANDLE.into());
        }

        let value = json_from_raw_parts(value, value_len)?;

        // This looks a bit tricky but we're just turning the opaque handle into `Validator` pointer
        // and then returning a reference to avoid taking ownership of it.
        let schema = &*(handle as *const Validator);

        if schema.is_valid(&value) {
            if !sanitized_value.is_null() && !sanitized_value_len.is_null() {
                let value = value.to_string();

                *sanitized_value = CoTaskMemAlloc(value.len()) as _;

                if (*sanitized_value).is_null() {
                    return Err(E_OUTOFMEMORY.into());
                }

                (*sanitized_value).copy_from(value.as_ptr(), value.len());
                *sanitized_value_len = value.len();
            }

            Ok(())
        } else {
            let mut message = String::new();

            // The `validate` method returns a collection of errors. We'll just return the first
            // for simplicity.
            if let Some(error) = schema.validate(&value).unwrap_err().next() {
                message = error.to_string();
            }

            Err(Error::new(E_INVALIDARG, message))
        }
    }
}

// Takes care of all the JSON parsing and parameter validation.
unsafe fn json_from_raw_parts(value: *const u8, value_len: usize) -> Result<serde_json::Value> {
    unsafe {
        if value.is_null() {
            return Err(E_POINTER.into());
        }

        let value = std::slice::from_raw_parts(value, value_len);

        let value =
            std::str::from_utf8(value).map_err(|_| Error::from(ERROR_NO_UNICODE_TRANSLATION))?;

        serde_json::from_str(value).map_err(|error| Error::new(E_INVALIDARG, format!("{error}")))
    }
}

#[test]
fn simple() {
    unsafe {
        // Create a validator with the given schema.
        let schema = r#"{"maxLength": 5}"#;
        let mut handle = 0;
        assert_eq!(
            S_OK,
            CreateJsonValidator(schema.as_ptr(), schema.len(), &mut handle)
        );

        // Validate the json against the schema.
        let value = r#""Hello""#;
        assert_eq!(
            S_OK,
            ValidateJson(
                handle,
                value.as_ptr(),
                value.len(),
                std::ptr::null_mut(),
                std::ptr::null_mut()
            )
        );

        // Check check validation failure provides reasonable error information.
        let value = r#""Hello World""#;
        let code = ValidateJson(
            handle,
            value.as_ptr(),
            value.len(),
            std::ptr::null_mut(),
            std::ptr::null_mut(),
        );
        assert_eq!(E_INVALIDARG, code);
        assert_eq!(
            r#""Hello World" is longer than 5 characters"#,
            Error::from(code).message()
        );

        // The schema validator is reusable.
        let value = r#""World""#;
        assert_eq!(
            S_OK,
            ValidateJson(
                handle,
                value.as_ptr(),
                value.len(),
                std::ptr::null_mut(),
                std::ptr::null_mut()
            )
        );

        // Close the validator with the given handle.
        CloseJsonValidator(handle);

        // Closing a "zero" handle is harmless.
        CloseJsonValidator(0);
    }
}

#[test]
fn invalid_create_params() {
    unsafe {
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
}

#[test]
fn invalid_validate_params() {
    unsafe {
        // Create a validator with the given schema.
        let schema = r#"{"maxLength": 5}"#;
        let mut handle = 0;
        assert_eq!(
            S_OK,
            CreateJsonValidator(schema.as_ptr(), schema.len(), &mut handle)
        );

        // Check that a zero handle is caught.
        let value = r#""Hello""#;
        assert_eq!(
            E_HANDLE,
            ValidateJson(
                0,
                value.as_ptr(),
                value.len(),
                std::ptr::null_mut(),
                std::ptr::null_mut()
            )
        );

        // Check that a value null pointer is caught.
        assert_eq!(
            E_POINTER,
            ValidateJson(
                handle,
                std::ptr::null(),
                value.len(),
                std::ptr::null_mut(),
                std::ptr::null_mut()
            )
        );

        // Check that JSON parsing failure provides reasonable error information.
        let value = r#""Hello"#;
        let code = ValidateJson(
            handle,
            value.as_ptr(),
            value.len(),
            std::ptr::null_mut(),
            std::ptr::null_mut(),
        );
        assert_eq!(E_INVALIDARG, code);
        assert_eq!(
            "EOF while parsing a string at line 1 column 6",
            Error::from(code).message()
        );

        // Close the validator with the given handle.
        CloseJsonValidator(handle);
    }
}

#[test]
fn sanitized_value() {
    unsafe {
        // Create a validator with the given schema.
        let schema = r#"
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
        "#;

        let mut handle = 0;
        assert_eq!(
            S_OK,
            CreateJsonValidator(schema.as_ptr(), schema.len(), &mut handle)
        );

        // Validate and check the sanitized return value.
        let value = r#"
        {
            "name": "Kenny",
            "age": 21 
        }
        "#;
        let mut sanitized_alloc = std::ptr::null_mut();
        let mut sanitized_len = 0;

        assert_eq!(
            S_OK,
            ValidateJson(
                handle,
                value.as_ptr(),
                value.len(),
                &mut sanitized_alloc,
                &mut sanitized_len
            )
        );
        let sanitized = std::slice::from_raw_parts(sanitized_alloc, sanitized_len);
        let sanitized = String::from_utf8_lossy(sanitized).into_owned();
        CoTaskMemFree(Some(sanitized_alloc as _));
        assert_eq!(sanitized, r#"{"age":21,"name":"Kenny"}"#);

        // Close the validator with the given handle.
        CloseJsonValidator(handle);
    }
}

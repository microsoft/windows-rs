#[test]
fn from_hresult() {
    let error: windows::Error = windows::ErrorCode(0x80004004).into();

    assert_eq!(error.code(), windows::ErrorCode(0x80004004));
    assert_eq!(error.message(), "Operation aborted");
}

#[test]
fn originate() {
    let error = windows::Error::new(windows::ErrorCode(0x80004004), "test originate");

    assert_eq!(error.code(), windows::ErrorCode(0x80004004));
    assert_eq!(error.message(), "test originate");

    let code: windows::ErrorCode = error.into(); // SetErrorInfo is called before dropping the Error object.

    let error: windows::Error = code.into(); // GetErrorInfo is called to retrieve the original error info.

    assert_eq!(error.code(), windows::ErrorCode(0x80004004));
    assert_eq!(error.message(), "test originate");
}

#[test]
fn bad_uri() {
    let result = windows::foundation::Uri::create_uri("INVALID");
    let error: windows::Error = result.unwrap_err();

    assert_eq!(error.code(), windows::ErrorCode(0x80070057));
    assert_eq!(error.message(), "INVALID is not a valid absolute URI.");
}

#[test]
fn convertible() {
    fn windows_error() -> windows::Result<()> {
        Err(windows::Error::new(
            windows::ErrorCode::E_NOINTERFACE,
            "test message",
        ))
    }

    fn convertible_error() -> std::result::Result<(), Box<dyn std::error::Error>> {
        windows_error()?;
        Ok(())
    }

    let result = convertible_error();
    let format = format!("{:?}", result);

    assert_eq!(
        format,
        "Err(Error { code: 0x80004002, message: \"test message\" })"
    );
}

#[test]
fn from_hresult() {
    let error: winrt::Error = winrt::ErrorCode(0x80004004).into();

    assert_eq!(error.code(), winrt::ErrorCode(0x80004004));
    assert_eq!(error.message(), "Operation aborted");
}

#[test]
fn originate() {
    let error = winrt::Error::new(winrt::ErrorCode(0x80004004), "test originate");

    assert_eq!(error.code(), winrt::ErrorCode(0x80004004));
    assert_eq!(error.message(), "test originate");
}

#[test]
fn bad_uri() {
    let result = winrt::foundation::Uri::create_uri("INVALID");
    let error: winrt::Error = result.unwrap_err();

    assert_eq!(error.code(), winrt::ErrorCode(0x80070057));
    assert_eq!(error.message(), "INVALID is not a valid absolute URI.");
}

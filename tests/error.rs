#![allow(overflowing_literals)]

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
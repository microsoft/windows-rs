use std::convert::TryInto;
use windows::{Foundation::Uri, Win32::Foundation::E_NOINTERFACE};

#[test]
fn from_hresult() {
    assert!(helpers::set_thread_ui_language("en-US"));

    let error: windows::core::Error = windows::core::HRESULT(0x80004004).into();

    assert_eq!(error.code(), windows::core::HRESULT(0x80004004));
    let message: String = error.message().try_into().unwrap();
    assert_eq!(message.trim_end(), "Operation aborted");
}

#[test]
fn originate() {
    let error = windows::core::Error::new(windows::core::HRESULT(0x80004004), "test originate".into());

    assert_eq!(error.code(), windows::core::HRESULT(0x80004004));
    assert_eq!(error.message(), "test originate");

    let code: windows::core::HRESULT = error.into(); // SetErrorInfo is called before dropping the Error object.

    let error: windows::core::Error = code.into(); // GetErrorInfo is called to retrieve the original error info.

    assert_eq!(error.code(), windows::core::HRESULT(0x80004004));
    assert_eq!(error.message(), "test originate");
}

#[test]
fn bad_uri() {
    assert!(helpers::set_thread_ui_language("en-US"));

    let result = Uri::CreateUri("INVALID");
    let error: windows::core::Error = result.unwrap_err();

    assert_eq!(error.code(), windows::core::HRESULT(0x80070057));
    assert_eq!(error.message(), "INVALID is not a valid absolute URI.");
}

#[test]
fn convertible() {
    fn windows_error() -> windows::core::Result<()> {
        Err(windows::core::Error::new(E_NOINTERFACE, "test message".into()))
    }

    fn convertible_error() -> core::result::Result<(), Box<dyn std::error::Error>> {
        windows_error()?;
        Ok(())
    }

    let result = convertible_error();
    let format = format!("{:?}", result);

    assert_eq!(format, "Err(Error { code: 0x80004002, message: test message })");
}

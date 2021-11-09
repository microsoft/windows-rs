use std::convert::TryInto;
use test_winrt::Windows::{Foundation::Uri, Win32::Foundation::E_NOINTERFACE};

#[test]
fn from_hresult() {
    assert!(helpers::set_thread_ui_language("en-US"));

    let error: windows::runtime::Error = windows::runtime::HRESULT(0x80004004).into();

    assert_eq!(error.code(), windows::runtime::HRESULT(0x80004004));
    let message: String = error.message().try_into().unwrap();
    assert_eq!(message.trim_end(), "Operation aborted");
}

#[test]
fn originate() {
    let error = windows::runtime::Error::new(windows::runtime::HRESULT(0x80004004), "test originate".into());

    assert_eq!(error.code(), windows::runtime::HRESULT(0x80004004));
    assert_eq!(error.message(), "test originate");

    let code: windows::runtime::HRESULT = error.into(); // SetErrorInfo is called before dropping the Error object.

    let error: windows::runtime::Error = code.into(); // GetErrorInfo is called to retrieve the original error info.

    assert_eq!(error.code(), windows::runtime::HRESULT(0x80004004));
    assert_eq!(error.message(), "test originate");
}

#[test]
fn bad_uri() {
    assert!(helpers::set_thread_ui_language("en-US"));

    let result = Uri::CreateUri("INVALID");
    let error: windows::runtime::Error = result.unwrap_err();

    assert_eq!(error.code(), windows::runtime::HRESULT(0x80070057));
    assert_eq!(error.message(), "INVALID is not a valid absolute URI.");
}

#[test]
fn convertible() {
    fn windows_error() -> windows::runtime::Result<()> {
        Err(windows::runtime::Error::new(E_NOINTERFACE, "test message".into()))
    }

    fn convertible_error() -> core::result::Result<(), Box<dyn std::error::Error>> {
        windows_error()?;
        Ok(())
    }

    let result = convertible_error();
    let format = format!("{:?}", result);

    assert_eq!(format, "Err(Error { code: 0x80004002, message: test message })");
}

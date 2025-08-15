use windows::{Foundation::Uri, Win32::Foundation::E_NOINTERFACE};

#[test]
fn from_hresult() {
    helpers::set_thread_ui_language();

    let error: windows::core::Error = windows::core::HRESULT(-2147467260).into();

    assert_eq!(error.code(), windows::core::HRESULT(-2147467260));
    let message: String = error.message();
    assert_eq!(message, "Operation aborted");
}

#[test]
fn originate() {
    let error = windows::core::Error::new(windows::core::HRESULT(-2147467260), "test originate");

    assert_eq!(error.code(), windows::core::HRESULT(-2147467260));
    assert_eq!(error.message(), "test originate");

    let code: windows::core::HRESULT = error.into(); // SetErrorInfo is called before dropping the Error object.

    let error: windows::core::Error = code.into(); // GetErrorInfo is called to retrieve the original error info.

    assert_eq!(error.code(), windows::core::HRESULT(-2147467260));
    assert_eq!(error.message(), "test originate");
}

#[test]
fn bad_uri() {
    helpers::set_thread_ui_language();

    let result = Uri::CreateUri(&windows::core::HSTRING::from("INVALID"));
    let error: windows::core::Error = result.unwrap_err();

    assert_eq!(error.code(), windows::core::HRESULT(-2147024809));
    assert_eq!(error.message(), "INVALID is not a valid absolute URI.");
}

#[test]
fn convertible() {
    fn windows_error() -> windows::core::Result<()> {
        Err(windows::core::Error::new(E_NOINTERFACE, "test message"))
    }

    fn convertible_error() -> core::result::Result<(), Box<dyn std::error::Error>> {
        windows_error()?;
        Ok(())
    }

    let result = convertible_error();
    let format = format!("{result:?}");

    assert_eq!(
        format,
        r#"Err(Error { code: HRESULT(0x80004002), message: "test message" })"#
    );
}

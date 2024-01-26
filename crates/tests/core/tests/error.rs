use windows::{
    core::*, Win32::Foundation::*, Win32::Media::Audio::*, Win32::System::Com::*,
    Win32::System::Ole::*, Win32::System::WinRT::*,
};

#[test]
fn display_debug() {
    assert!(helpers::set_thread_ui_language());

    let e = Error::from(ERROR_NO_UNICODE_TRANSLATION);
    let display = format!("{e}");
    let debug = format!("{e:?}");
    assert_eq!(display, "No mapping for the Unicode character exists in the target multi-byte code page. (0x80070459)");
    assert_eq!(
        debug,
        r#"Error { code: HRESULT(0x80070459), message: "No mapping for the Unicode character exists in the target multi-byte code page." }"#
    );

    let e = Error::from(AUDCLNT_E_UNSUPPORTED_FORMAT);
    let display = format!("{e}");
    let debug = format!("{e:?}");
    assert_eq!(display, "0x88890008");
    assert_eq!(debug, r#"Error { code: HRESULT(0x88890008), message: "" }"#);
}

#[test]
fn hresult_last_error() {
    unsafe {
        assert_eq!(CRYPT_E_NOT_FOUND.0, 0x80092004u32 as i32);
        SetLastError(WIN32_ERROR(CRYPT_E_NOT_FOUND.0 as u32));
        let e = GetLastError().unwrap_err();
        assert_eq!(e.code(), CRYPT_E_NOT_FOUND);
    }
}

// Checks that non-restricted error info is reported.
#[test]
fn set_error_info() -> Result<()> {
    unsafe {
        let creator = CreateErrorInfo()?;
        creator.SetDescription(w!("message"))?;
        SetErrorInfo(0, &creator.cast::<IErrorInfo>()?)?;

        assert_eq!(Error::from(E_FAIL).message(), "message");
        SetErrorInfo(0, None)?;
        assert_eq!(Error::from(E_FAIL).message(), "Unspecified error");

        Ok(())
    }
}

// https://github.com/microsoft/cppwinrt/pull/1386
#[test]
fn suppressed_error_info() -> Result<()> {
    unsafe { RoSetErrorReportingFlags(RO_ERROR_REPORTING_SUPPRESSSETERRORINFO.0 as u32)? };

    assert_eq!(
        Error::new(E_FAIL, "message".into()).message(),
        "Unspecified error"
    );

    unsafe { RoSetErrorReportingFlags(RO_ERROR_REPORTING_USESETERRORINFO.0 as u32)? };

    assert_eq!(Error::new(E_FAIL, "message".into()).message(), "message");

    Ok(())
}

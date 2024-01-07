use windows::{core::*, Win32::Foundation::*, Win32::Media::Audio::*};

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

#[test]
fn test() {
    let e = windows::core::Error::from(windows::Win32::Foundation::ERROR_NO_UNICODE_TRANSLATION);
    let display = format!("{}", e);
    let debug = format!("{:?}", e);
    assert_eq!(display, "No mapping for the Unicode character exists in the target multi-byte code page. (0x80070459)");
    assert_eq!(debug, r#"Error { code: HRESULT(0x80070459), message: "No mapping for the Unicode character exists in the target multi-byte code page." }"#);

    let e = windows::core::Error::from(windows::Win32::Media::Audio::AUDCLNT_E_UNSUPPORTED_FORMAT);
    let display = format!("{}", e);
    let debug = format!("{:?}", e);
    assert_eq!(display, "0x88890008");
    assert_eq!(debug, r#"Error { code: HRESULT(0x88890008), message: "" }"#);
}

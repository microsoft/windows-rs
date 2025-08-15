use windows::{
    core::*, Storage::Streams::*, Win32::Foundation::*, Win32::UI::WindowsAndMessaging::*,
};

#[test]
fn nested() {
    let options = InputStreamOptions::Partial | InputStreamOptions::ReadAhead;
    assert!(options.0 == 3);

    let options = InputStreamOptions::Partial & InputStreamOptions::ReadAhead;
    assert!(options.0 == 0);

    let options = (InputStreamOptions::Partial | InputStreamOptions::ReadAhead)
        & InputStreamOptions::ReadAhead;
    assert!(options.0 == 2);
    assert!(!options.contains(InputStreamOptions::Partial));
    assert!(options.contains(InputStreamOptions::ReadAhead));

    let mut options = InputStreamOptions::Partial;
    options |= InputStreamOptions::ReadAhead;
    assert!(options.0 == 3);
    assert!(options.contains(InputStreamOptions::Partial));
    assert!(options.contains(InputStreamOptions::ReadAhead));

    options &= InputStreamOptions::ReadAhead;
    assert!(options.0 == 2);
}

#[test]
#[expect(clippy::assertions_on_constants)] // intentionally testing constant
fn const_pattern() {
    match InputStreamOptions::ReadAhead {
        InputStreamOptions::ReadAhead => assert!(true),
        _ => assert!(false),
    }
}

#[test]
fn win32_error() {
    helpers::set_thread_ui_language();

    let e: WIN32_ERROR = ERROR_ACCESS_DENIED;
    assert!(e.0 == 5);
    let h: HRESULT = ERROR_ACCESS_DENIED.into();
    assert!(h.is_err());
    assert!("WIN32_ERROR(5)" == format!("{e:?}"));

    let e: Error = h.into();
    assert_eq!(
        r#"Error { code: HRESULT(0x80070005), message: "Access is denied." }"#,
        format!("{e:?}")
    );
    let e = WIN32_ERROR::from_error(&e).unwrap();
    assert!(e == ERROR_ACCESS_DENIED);
}

#[test]
fn messagebox_style() {
    let s: MESSAGEBOX_STYLE = MB_YESNOCANCEL | MB_HELP;
    assert!(s.0 == 0x4003);
    assert!(s.contains(MB_YESNOCANCEL));
    assert!(s.contains(MB_HELP));
    assert!(!s.contains(MB_SYSTEMMODAL));
}

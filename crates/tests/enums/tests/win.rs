use windows::{
    core::*,
    Storage::Streams::*,
    Win32::Foundation::*,
    Win32::UI::WindowsAndMessaging::*,
};

#[test]
fn nested() {
    let options = InputStreamOptions::Partial | InputStreamOptions::ReadAhead;
    assert!(options.0 == 3);

    let options = InputStreamOptions::Partial & InputStreamOptions::ReadAhead;
    assert!(options.0 == 0);

    let options = (InputStreamOptions::Partial | InputStreamOptions::ReadAhead) & InputStreamOptions::ReadAhead;
    assert!(options.0 == 2);

    let mut options = InputStreamOptions::Partial;
    options |= InputStreamOptions::ReadAhead;
    assert!(options.0 == 3);

    options &= InputStreamOptions::ReadAhead;
    assert!(options.0 == 2);
}

#[test]
fn const_pattern() {
    match InputStreamOptions::ReadAhead {
        InputStreamOptions::ReadAhead => assert!(true),
        _ => assert!(false),
    }
}

#[test]
fn win32_error() {
    let e: WIN32_ERROR = ERROR_ACCESS_DENIED;
    assert!(e.0 == 5);
    let h: HRESULT = ERROR_ACCESS_DENIED.into();
    assert!(h.is_err());
    assert!("WIN32_ERROR(5)" == format!("{:?}", e));
}

#[test]
fn messagebox_style() {
    let s: MESSAGEBOX_STYLE = MB_YESNOCANCEL | MB_HELP;
    assert!(s.0 == 0x4003);
}
use windows_sys::{
    Storage::Streams::*,
    Win32::Foundation::*,
    Win32::UI::WindowsAndMessaging::*,
};

#[test]
fn nested() {
    let options: InputStreamOptions = InputStreamOptions(InputStreamOptions::Partial.0 | InputStreamOptions::ReadAhead.0);
    assert!(options.0 == 3);
}

#[test]
fn win32_error() {
    let e: WIN32_ERROR = ERROR_ACCESS_DENIED;
    assert!(e == 5);
}

#[test]
fn messagebox_style() {
    let s: MESSAGEBOX_STYLE = MB_YESNOCANCEL | MB_HELP;
    assert!(s == 0x4003);
}
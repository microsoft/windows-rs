use windows_sys::{Win32::Foundation::*, Win32::UI::WindowsAndMessaging::*};

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

use windows::Win32::{Foundation::HWND, UI::WindowsAndMessaging::CWPSTRUCT};

#[test]
fn test_struct_formatting_parameters_debug_trait() {
    let mut cwp = CWPSTRUCT::default();
    cwp.hwnd = HWND(5678);

    assert_eq!("CWPSTRUCT { lParam: LPARAM(0), wParam: WPARAM(0), message: 0, hwnd: HWND(5678) }", format!("{:?}", cwp));
}

#[test]
fn test_struct_formatting_parameters_debug_trait_with_lhex() {
    let mut cwp = CWPSTRUCT::default();
    cwp.hwnd = HWND(1234);

    assert_eq!("CWPSTRUCT { lParam: LPARAM(0), wParam: WPARAM(0), message: 0, hwnd: HWND(4d2) }", format!("{:x?}", cwp));
}

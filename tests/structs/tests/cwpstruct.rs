use test_structs::Windows::Win32::{Foundation::HWND, UI::WindowsAndMessaging::CWPSTRUCT};

#[test]
fn test_struct_formatting_parameters_debug_trait() {
    let mut cwp = CWPSTRUCT::default();
    cwp.hwnd = HWND(5678);

    assert_eq!(
        "CWPSTRUCT { lParam: LPARAM { Value: 0 }, wParam: WPARAM { Value: 0 }, \
                message: 0, hwnd: HWND { Value: 5678 } }",
        format!("{:?}", cwp)
    );
}

#[test]
fn test_struct_formatting_parameters_debug_trait_with_lhex() {
    let mut cwp = CWPSTRUCT::default();
    cwp.hwnd = HWND(1234);

    assert_eq!(
        "CWPSTRUCT { lParam: LPARAM { Value: 0 }, wParam: WPARAM { Value: 0 }, \
                message: 0, hwnd: HWND { Value: 4d2 } }",
        format!("{:x?}", cwp)
    );
}

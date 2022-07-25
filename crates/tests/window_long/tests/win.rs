use windows::Win32::UI::WindowsAndMessaging::*;

// Validates that these possibly aliased functions can be called on all supported targets.
#[test]
fn test() {
    unsafe {
        SetWindowLongPtrA(None, GWLP_USERDATA, 0);
        GetWindowLongPtrA(None, GWLP_USERDATA);
        SetWindowLongPtrW(None, GWLP_USERDATA, 0);
        GetWindowLongPtrW(None, GWLP_USERDATA);
    }
}

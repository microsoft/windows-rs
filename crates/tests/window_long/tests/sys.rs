use windows_sys::Win32::UI::WindowsAndMessaging::*;

// Validates that these possibly aliased functions can be called on all supported targets.
#[test]
fn test() {
    unsafe {
        SetWindowLongPtrA(0, GWLP_USERDATA, 0);
        GetWindowLongPtrA(0, GWLP_USERDATA);
    }
}

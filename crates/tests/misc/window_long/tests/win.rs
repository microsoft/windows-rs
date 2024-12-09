use windows::Win32::UI::WindowsAndMessaging::*;

// Validates that these possibly aliased functions can be called on all supported targets.
#[test]
fn test() {
    unsafe {
        SetWindowLongPtrA(Default::default(), GWLP_USERDATA, 0);
        GetWindowLongPtrA(Default::default(), GWLP_USERDATA);
        SetWindowLongPtrW(Default::default(), GWLP_USERDATA, 0);
        GetWindowLongPtrW(Default::default(), GWLP_USERDATA);
    }
}

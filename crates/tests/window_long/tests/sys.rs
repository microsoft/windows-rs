use windows_sys::Win32::UI::WindowsAndMessaging::*;

// Validates that these possibly aliased functions can be called on all supported targets.
#[test]
fn test() {
    unsafe {
        SetWindowLongPtrA(core::ptr::null_mut(), GWLP_USERDATA, 0);
        GetWindowLongPtrA(core::ptr::null_mut(), GWLP_USERDATA);
        SetWindowLongPtrW(core::ptr::null_mut(), GWLP_USERDATA, 0);
        GetWindowLongPtrW(core::ptr::null_mut(), GWLP_USERDATA);
    }
}

use test_bindgen::delegate_param::*;
use windows_core::*;

#[test]
fn test() {
    unsafe {
        SetConsoleCtrlHandler(Some(callback), true).unwrap();
        SetConsoleCtrlHandler(Some(callback), false).unwrap();
        SetConsoleCtrlHandler(None, true).unwrap();
    }
}

unsafe extern "system" fn callback(_ctrltype: u32) -> BOOL {
    BOOL(1)
}

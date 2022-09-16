use windows_sys::Win32::System::Threading::*;

#[test]
fn test() {
    unsafe {
        WaitOnAddress(std::ptr::null(), std::ptr::null(), 0, 0);
    }
}

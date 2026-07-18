#![cfg(windows)]
use windows_sys::Win32::*;

// Validates that the target libs resolve this function to "winspool.drv"

#[test]
fn test() {
    unsafe {
        _ = GetSpoolFileHandle(std::ptr::null_mut());
    }
}

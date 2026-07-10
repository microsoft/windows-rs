#![cfg(windows)]
use windows_sys::Win32::winspool::*;

// Validates that the target libs resolve this function to "winspool.drv"

#[test]
fn test() {
    unsafe {
        _ = GetSpoolFileHandle(std::ptr::null_mut());
    }
}

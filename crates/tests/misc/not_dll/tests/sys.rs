#![cfg(windows)]
use windows_sys::winspool::*;

// Validates that the target libs resolve this function to "winspool.drv"

#[test]
fn test() {
    unsafe {
        _ = GetSpoolFileHandle(std::ptr::null_mut());
    }
}

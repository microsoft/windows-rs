#![cfg(windows)]
use windows::Win32::{winnt::HANDLE, winspool::*};

// Validates that the target libs resolve this function to "winspool.drv"

#[test]
fn test() {
    unsafe {
        _ = GetSpoolFileHandle(HANDLE::default());
    }
}

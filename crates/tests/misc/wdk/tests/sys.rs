#![cfg(windows)]
use windows_sys::Win32::*;

#[test]
fn offline_registry() {
    unsafe {
        let mut hive = core::ptr::null_mut();
        ORCreateHive(&mut hive);
        ORCloseHive(hive);
    }
}

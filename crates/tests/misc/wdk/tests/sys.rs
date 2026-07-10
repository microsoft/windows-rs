#![cfg(windows)]
use windows_sys::Wdk::offreg::*;

#[test]
fn offline_registry() {
    unsafe {
        let mut hive = core::ptr::null_mut();
        ORCreateHive(&mut hive);
        ORCloseHive(hive);
    }
}

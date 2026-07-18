#![cfg(windows)]
use windows::Wdk::*;

#[test]
fn test() {
    unsafe {
        let mut hive = ORHKEY(core::ptr::null_mut());
        assert_eq!(ORCreateHive(&mut hive), 0);
        assert_eq!(ORCloseHive(hive), 0);
    }
}

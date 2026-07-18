#![cfg(windows)]
use windows::{Win32::*, core::Interface};

#[test]
fn test() {
    unsafe {
        let mut debug = None;
        DebugCreateEx(&IDebugClient::IID, 0, &mut debug as *mut _ as _)
            .ok()
            .unwrap();
        let _debug: IDebugClient = debug.unwrap();
    }
}

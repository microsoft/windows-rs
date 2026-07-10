#![cfg(windows)]
use windows::{core::Interface, dbgeng::*};

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

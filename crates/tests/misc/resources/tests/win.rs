#![cfg(windows)]
use windows::{Win32::commctrl::*, Win32::objidlbase::*, Win32::winuser::*};

#[test]
fn win() {
    unsafe {
        assert_eq!(IDI_APPLICATION.0 as u16, 32512);
        assert_eq!(TD_ERROR_ICON.0 as i16, -2);
        assert_eq!(COLE_DEFAULT_PRINCIPAL.0 as usize, usize::MAX);
        let icon = LoadIconW(None, IDI_APPLICATION);
        println!("icon = {icon:?}");
    }
}

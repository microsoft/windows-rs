#![cfg(windows)]
use windows::Win32::*;

#[test]
fn win() {
    unsafe {
        assert_eq!(IDI_APPLICATION.0 as u16, 32512);
        assert_eq!(TD_ERROR_ICON.0 as i16, -2);
        assert_eq!(COLE_DEFAULT_PRINCIPAL as usize, usize::MAX);
        let icon = LoadIconW(None, windows::core::PCWSTR(IDI_APPLICATION.0 as _));
        println!("icon = {icon:?}");
    }
}

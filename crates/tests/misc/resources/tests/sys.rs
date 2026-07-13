#![cfg(windows)]
use windows_sys::{commctrl::*, errhandlingapi::*, objidlbase::*, winerror::*, winuser::*};

/// These tests ensure `MAKEINTRESOURCEW` style constants an in particular negative constants like TD_ERROR_ICON
/// work as expected.
#[test]
fn sys() {
    unsafe {
        SetLastError(0);
        assert_eq!(IDI_APPLICATION as u16, 32512);
        assert_ne!(
            LoadIconW(core::ptr::null_mut(), IDI_APPLICATION),
            core::ptr::null_mut()
        );
        assert_eq!(GetLastError(), 0);

        assert_eq!(TD_ERROR_ICON as i16, -2);
        assert_eq!(
            LoadIconW(core::ptr::null_mut(), TD_ERROR_ICON),
            core::ptr::null_mut()
        );
        assert_eq!(GetLastError(), ERROR_RESOURCE_TYPE_NOT_FOUND);

        assert_eq!(COLE_DEFAULT_PRINCIPAL as usize, usize::MAX);
    }
}

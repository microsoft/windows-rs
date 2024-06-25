// This tests code paths in `windows-result` that are different on non-Windows platforms.
#![cfg(not(windows))]

use windows::core::Error;
use windows::Win32::Foundation::{E_FAIL, S_OK};

#[test]
fn basic_hresult() {
    assert!(E_FAIL.is_err());
    assert!(S_OK.is_ok());

    let ok_message = S_OK.message();
    assert_eq!(ok_message, "0x00000000");
}

#[test]
fn error_message_is_not_supported() {
    let e = Error::new(S_OK, "this gets ignored");
    let message = e.message();
    assert_eq!(message, "0x80004005");
}

#[test]
#[should_panic]
fn from_win32_panics() {
    // from_win32() is not implemented on non-Windows platforms.
    let _e = Error::from_win32();
}

#[test]
fn error_from_hresult() {
    let e = Error::from(E_FAIL);
    assert_eq!(e.code(), E_FAIL);
}

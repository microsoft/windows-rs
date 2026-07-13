#![cfg(windows)]
use windows::core::{HRESULT, WIN32_ERROR};
use windows::winerror::*;

#[test]
fn test_message() {
    helpers::set_thread_ui_language();

    let code: HRESULT = WIN32_ERROR(ERROR_SUCCESS).into();
    let message: String = code.message();
    assert_eq!(message, "The operation completed successfully.");

    let code: HRESULT = WIN32_ERROR(ERROR_IO_PENDING).into();
    let message: String = code.message();
    assert_eq!(message, "Overlapped I/O operation is in progress.");
}

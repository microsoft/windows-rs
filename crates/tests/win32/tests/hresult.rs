use std::convert::TryInto;
use windows::core::HRESULT;
use windows::Win32::Foundation::*;

#[test]
fn test_message() {
    assert!(helpers::set_thread_ui_language("en-US"));

    let code: HRESULT = ERROR_SUCCESS.into();
    let message: String = code.message().try_into().unwrap();
    assert_eq!(message, "The operation completed successfully.");

    let code: HRESULT = ERROR_IO_PENDING.into();
    let message: String = code.message().try_into().unwrap();
    assert_eq!(message, "Overlapped I/O operation is in progress.");
}

use test_win32::*;
use windows::runtime::HRESULT;
use Windows::Win32::Foundation::*;
use std::convert::TryInto;

#[test]
fn test_message() {
    assert!(helpers::set_thread_ui_language("en-US"));

    let code: HRESULT = ERROR_SUCCESS.into();
    let message: String = code.message().try_into().unwrap();
    assert_eq!(message.trim_end(), "The operation completed successfully.");

    let code: HRESULT = ERROR_IO_PENDING.into();
    let message: String = code.message().try_into().unwrap();
    assert_eq!(message.trim_end(), "Overlapped I/O operation is in progress.");
}

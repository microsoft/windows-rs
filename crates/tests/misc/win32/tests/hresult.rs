use windows::core::HRESULT;
use windows::Win32::Foundation::*;

#[test]
fn test_message() {
    helpers::set_thread_ui_language();

    let code: HRESULT = ERROR_SUCCESS.into();
    let message: String = code.message();
    assert_eq!(message, "The operation completed successfully.");

    let code: HRESULT = ERROR_IO_PENDING.into();
    let message: String = code.message();
    assert_eq!(message, "Overlapped I/O operation is in progress.");
}

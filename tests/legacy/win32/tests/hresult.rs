use test_win32::*;
use windows::HRESULT;
use Windows::Win32::Foundation::*;

#[test]
fn test_message() {
    assert!(helpers::set_thread_ui_language("en-US"));

    let code: HRESULT = ERROR_SUCCESS.into();
    assert_eq!(code.message(), "The operation completed successfully.");

    let code: HRESULT = ERROR_IO_PENDING.into();
    assert_eq!(code.message(), "Overlapped I/O operation is in progress.");
}

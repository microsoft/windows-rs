use windows::{Error, HRESULT};

#[test]
fn test_message() {
    assert!(helpers::set_thread_ui_language("en-US"));

    let code = Error::fast_error(HRESULT::from_win32(0));
    assert_eq!(code.message(), "The operation completed successfully.");

    let code = Error::fast_error(HRESULT::from_win32(997));
    assert_eq!(code.message(), "Overlapped I/O operation is in progress.");
}

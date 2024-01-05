use windows::{core::*, Win32::Foundation::*};

#[test]
fn test() {
    assert!(helpers::set_thread_ui_language());

    assert_eq!(E_FAIL.message(), "Unspecified error");

    assert_eq!(Error::new(E_FAIL, "Test \t\n\r".into()).message(), "Test");

    assert_eq!(Error::new(E_FAIL, " \t\n\r ".into()).message(), "");
}

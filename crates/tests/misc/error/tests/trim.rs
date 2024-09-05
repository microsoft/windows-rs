use windows::{core::*, Win32::Foundation::*};

#[test]
fn test() {
    helpers::set_thread_ui_language();

    assert_eq!(E_FAIL.message(), "Unspecified error");

    assert_eq!(Error::new(E_FAIL, "Test \t\n\r").message(), "Test");

    assert_eq!(Error::new(E_FAIL, " \t\n\r ").message(), "");
}

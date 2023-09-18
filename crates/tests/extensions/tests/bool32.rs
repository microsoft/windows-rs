use windows::{Win32::Foundation::*, Win32::System::Threading::*};

#[test]
fn test() {
    let status = BOOL::default();
    assert_eq!(status.0, 0);

    let status = BOOL(1);
    assert_eq!(status.0, 1);
    assert_eq!(status.as_bool(), true);
    assert_eq!(status.ok().is_ok(), true);

    unsafe {
        SetLastError(ERROR_ACCESS_DENIED);
    }
    let status = BOOL(0);
    assert_eq!(status.0, 0);
    assert_eq!(status.as_bool(), false);
    let result = status.ok();
    assert_eq!(result.is_ok(), false);
    let error = result.unwrap_err();
    assert_eq!(error.code(), E_ACCESSDENIED);
}

#[test]
#[ignore]
fn no_run() {
    unsafe {
        _ = CreateEventA(None, false, true, None);
        _ = CreateEventA(None, BOOL(0), BOOL(1), None);
    }
}

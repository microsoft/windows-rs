use windows::Win32::Foundation::*;

#[test]
#[expect(clippy::nonminimal_bool)] // explicit logic is intentionally being tested
fn test() {
    let status = VARIANT_BOOL::default();
    assert_eq!(status, VARIANT_FALSE);

    let status = VARIANT_TRUE;
    assert_eq!(status.0, -1);
    assert!(status.as_bool());
    assert!(status.ok().is_ok());

    unsafe {
        SetLastError(ERROR_ACCESS_DENIED);
    }
    let status = VARIANT_FALSE;
    assert_eq!(status.0, 0);
    assert!(!status.as_bool());
    let result = status.ok();
    assert!(!result.is_ok());
    let error = result.unwrap_err();
    assert_eq!(error.code(), E_ACCESSDENIED);
}

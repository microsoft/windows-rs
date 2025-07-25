use windows_result::*;

windows_link::link!("kernel32.dll" "system" fn SetLastError(code: u32));

const ERROR_CANCELLED: u32 = 1223;
const E_CANCELLED: HRESULT = HRESULT::from_win32(ERROR_CANCELLED);

#[test]
fn test() {
    let status = BOOL::default();
    assert_eq!(status.0, 0);

    let status = BOOL(1);
    assert_eq!(status.0, 1);
    assert!(status.as_bool());
    assert!(status.ok().is_ok());
    status.unwrap();
    status.expect("bool expected");

    unsafe {
        SetLastError(ERROR_CANCELLED);
    }

    let status = BOOL(0);
    assert_eq!(status.0, 0);
    assert!(!status.as_bool());
    let result: Result<(), HRESULT> = status.ok();
    assert!(!result.is_ok());
    let error = result.unwrap_err();
    assert_eq!(error, E_CANCELLED);
}

#[test]
#[should_panic(expected = "HRESULT(0x800704C7)")]
fn test_unwrap() {
    unsafe {
        SetLastError(ERROR_CANCELLED);
    }
    BOOL(0).unwrap();
}

#[test]
#[should_panic(expected = "cancel: HRESULT(0x800704C7)")]
fn test_expect() {
    unsafe {
        SetLastError(ERROR_CANCELLED);
    }
    BOOL(0).expect("cancel");
}

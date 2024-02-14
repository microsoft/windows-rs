use windows_result::*;

const S_OK: HRESULT = HRESULT(0);
const E_INVALIDARG: HRESULT = HRESULT(-2147024809i32);
const ERROR_CANCELLED: u32 = 1223;
const E_CANCELLED: HRESULT = HRESULT::from_win32(ERROR_CANCELLED);

windows_targets::link!("kernel32.dll" "system" fn SetLastError(code: u32));

#[test]
fn empty() {
    const EMPTY: Error = Error::empty();
    assert_eq!(EMPTY.code(), S_OK);
    assert!(EMPTY.as_ptr().is_null());
    assert_eq!(EMPTY.message(), "The operation completed successfully.");
}

#[test]
fn new() {
    let e = Error::new(E_INVALIDARG, "");
    assert_eq!(e.code(), E_INVALIDARG);
    assert!(e.as_ptr().is_null());

    let e = Error::new(E_INVALIDARG, "test message");
    assert_eq!(e.code(), E_INVALIDARG);
    assert!(!e.as_ptr().is_null());
    assert_eq!(e.message(), "test message");
}

#[test]
fn from_hresult() {
    let e = Error::from_hresult(E_INVALIDARG);
    assert_eq!(e.code(), E_INVALIDARG);
    assert!(e.as_ptr().is_null());
    assert_eq!(e.message(), "The parameter is incorrect.");
}

#[test]
fn from_win32() {
    unsafe { SetLastError(0) };

    let e = Error::from_win32();
    assert_eq!(e.code(), S_OK);
    assert!(e.as_ptr().is_null());

    unsafe { SetLastError(ERROR_CANCELLED) };

    let e = Error::from_win32();
    assert!(e.as_ptr().is_null());
    assert_eq!(e.code(), E_CANCELLED);
}

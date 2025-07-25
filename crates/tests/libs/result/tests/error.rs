use windows_result::*;

const S_OK: HRESULT = HRESULT(0);
const E_INVALIDARG: HRESULT = HRESULT(-2147024809i32);
const ERROR_INVALID_DATA: u32 = 13;

windows_link::link!("kernel32.dll" "system" fn SetLastError(code: u32));

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

    if cfg!(windows_slim_errors) {
        assert!(e.as_ptr().is_null());
        assert_eq!(e.message(), "The parameter is incorrect.");
    } else {
        assert!(!e.as_ptr().is_null());
        assert_eq!(e.message(), "test message");
    }
}

#[test]
fn from_hresult() {
    let e = Error::from_hresult(E_INVALIDARG);
    assert_eq!(e.code(), E_INVALIDARG);
    assert!(e.as_ptr().is_null());
    assert_eq!(e.message(), "The parameter is incorrect.");
}

#[test]
fn try_from_int() {
    fn call(value: usize) -> Result<u16, HRESULT> {
        Ok(value.try_into()?)
    }

    assert_eq!(call(123), Ok(123));

    let e = call(usize::MAX).unwrap_err();
    assert_eq!(e, HRESULT::from_win32(ERROR_INVALID_DATA));
}

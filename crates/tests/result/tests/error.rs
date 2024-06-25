use windows_result::*;

const E_INVALIDARG: HRESULT = HRESULT(-2147024809i32);
const E_FAIL: HRESULT = HRESULT(0x80004005_u32 as i32);
const ERROR_CANCELLED: u32 = 1223;
const ERROR_INVALID_DATA: u32 = 13;
const E_CANCELLED: HRESULT = HRESULT::from_win32(ERROR_CANCELLED);

windows_targets::link!("kernel32.dll" "system" fn SetLastError(code: u32));

#[test]
fn empty() {
    let e: Error = Error::empty();
    assert_eq!(e.code(), E_FAIL);
    assert!(e.detail().as_ptr().is_null());
    assert_eq!(e.message(), "Unspecified error");
}

#[test]
fn new() {
    let e = Error::new(E_INVALIDARG, "");
    assert_eq!(e.code(), E_INVALIDARG);
    assert!(e.detail().as_ptr().is_null());
    assert_eq!(e.message(), "The parameter is incorrect.");

    let e = Error::new(E_INVALIDARG, "test message");
    assert_eq!(e.code(), E_INVALIDARG);
    assert!(!e.detail().as_ptr().is_null());
    assert_eq!(e.message(), "test message");
}

#[test]
fn from_hresult() {
    let e = Error::from_hresult(E_INVALIDARG);
    assert_eq!(e.code(), E_INVALIDARG);
    assert!(e.detail().as_ptr().is_null());
    assert_eq!(e.message(), "The parameter is incorrect.");
}

#[test]
fn from_win32() {
    unsafe { SetLastError(0) };

    let e = Error::from_win32();
    assert_eq!(e.code(), E_FAIL);
    assert!(e.detail().as_ptr().is_null());

    unsafe { SetLastError(ERROR_CANCELLED) };

    let e = Error::from_win32();
    assert!(e.detail().as_ptr().is_null());
    assert_eq!(e.code(), E_CANCELLED);
}

#[test]
fn try_from_int() {
    fn call(value: usize) -> Result<u16> {
        Ok(value.try_into()?)
    }

    assert_eq!(call(123), Ok(123));

    let e = call(usize::MAX).unwrap_err();
    assert_eq!(e.code(), HRESULT::from_win32(ERROR_INVALID_DATA));
}

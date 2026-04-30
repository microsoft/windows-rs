#![cfg(windows)]
use windows_result::*;

const S_OK: HRESULT = HRESULT(0);
const E_INVALIDARG: HRESULT = HRESULT(-2147024809i32);
const ERROR_SUCCESS: WIN32_ERROR = WIN32_ERROR(0);
const ERROR_CANCELLED: WIN32_ERROR = WIN32_ERROR(1223);
const ERROR_INVALID_DATA: WIN32_ERROR = WIN32_ERROR(13);
const E_CANCELLED: HRESULT = ERROR_CANCELLED.to_hresult();

windows_link::link!("kernel32.dll" "system" fn SetLastError(code: WIN32_ERROR));

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
fn from_thread() {
    unsafe { SetLastError(ERROR_SUCCESS) };

    let e = Error::from_thread();
    assert_eq!(e.code(), S_OK);
    assert!(e.as_ptr().is_null());

    unsafe { SetLastError(ERROR_CANCELLED) };

    let e = Error::from_thread();
    assert!(e.as_ptr().is_null());
    assert_eq!(e.code(), E_CANCELLED);
}

// Tests the `IErrorInfo` COM round-trip and the silent-failure fallback:
//   1. `Error::new` originates an IRestrictedErrorInfo on the thread.
//   2. Converting `Error -> HRESULT` puts it back on the thread via `SetErrorInfo`.
//   3. The first `HRESULT -> Error` conversion consumes the error info; the message is preserved.
//   4. The second `HRESULT -> Error` conversion finds the thread empty (GetErrorInfo returns
//      nothing) and must fall back gracefully to the HRESULT system message.
#[test]
#[cfg(all(windows, not(windows_slim_errors)))]
fn error_info_consumed_before_conversion() {
    let original = Error::new(E_INVALIDARG, "test error info");
    assert_eq!(original.code(), E_INVALIDARG);
    assert_eq!(original.message(), "test error info");

    // Convert Error -> HRESULT (calls `into_thread`, putting IErrorInfo on the thread).
    let code: HRESULT = original.into();
    assert_eq!(code, E_INVALIDARG);

    // First conversion consumes the error info from the thread.
    let first = Error::from(code);
    assert_eq!(first.code(), E_INVALIDARG);
    assert_eq!(first.message(), "test error info");

    // Second conversion finds the thread empty (GetErrorInfo returns null).
    // Must fall back gracefully to the HRESULT system message.
    let second = Error::from(code);
    assert_eq!(second.code(), E_INVALIDARG);
    assert!(second.as_ptr().is_null());
    assert_eq!(second.message(), "The parameter is incorrect.");
}

#[test]
fn try_from_int() {
    fn call(value: usize) -> Result<u16> {
        Ok(value.try_into()?)
    }

    assert_eq!(call(123), Ok(123));

    let e = call(usize::MAX).unwrap_err();
    assert_eq!(e.code(), ERROR_INVALID_DATA.to_hresult());
}

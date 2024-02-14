use windows_result::*;

const S_OK: HRESULT = HRESULT(0);
const S_FALSE: HRESULT = HRESULT(1);
const E_INVALIDARG: HRESULT = HRESULT(-2147024809i32);
const ERROR_CANCELLED: u32 = 1223;
const E_CANCELLED: HRESULT = HRESULT::from_win32(ERROR_CANCELLED);

#[test]
fn is_ok() {
    assert!(S_OK.is_ok());
    assert!(S_FALSE.is_ok());
    assert!(!E_INVALIDARG.is_ok());
}

#[test]
fn is_err() {
    assert!(!S_OK.is_err());
    assert!(!S_FALSE.is_err());
    assert!(E_INVALIDARG.is_err());
}

#[test]
fn ok() {
    assert!(S_OK.ok().is_ok());
    assert!(S_FALSE.ok().is_ok());
    assert!(E_INVALIDARG.ok().is_err());
}

#[test]
fn map() {
    assert_eq!(123, S_OK.map(|| 123).unwrap());
    assert_eq!(E_INVALIDARG, E_INVALIDARG.map(|| 123).unwrap_err().code());
}

#[test]
fn and_then() {
    assert_eq!(123, S_OK.and_then(|| Ok(123)).unwrap());

    assert_eq!(
        E_INVALIDARG,
        E_INVALIDARG.and_then(|| Ok(123)).unwrap_err().code()
    );
}

#[test]
fn message() {
    helpers::set_thread_ui_language();
    assert_eq!(S_OK.message(), "The operation completed successfully.");

    assert_eq!(
        E_CANCELLED.message(),
        "The operation was canceled by the user."
    );

    assert_eq!(HRESULT(-1).message(), "");
}

#[test]
fn from_win32() {
    assert_eq!(E_INVALIDARG, HRESULT::from_win32(E_INVALIDARG.0 as u32));
    assert_eq!(E_CANCELLED, HRESULT::from_win32(ERROR_CANCELLED));
}

#[test]
fn from_result() {
    let result: Result<()> = Err(Error::new(E_INVALIDARG, "test message"));
    let err = HRESULT::from(result).ok().unwrap_err();
    assert_eq!(err.code(), E_INVALIDARG);
    assert_eq!(err.message(), "test message");
}

#[test]
fn display() {
    assert_eq!(E_INVALIDARG.to_string(), "0x80070057");
    assert_eq!(format!("{:?}", E_INVALIDARG), "HRESULT(0x80070057)");
}

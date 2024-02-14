use windows::{core::*, Win32::Foundation::*};

#[test]
fn ok() {
    let error = Error::new(E_FAIL, "test info");
    let code: HRESULT = error.into(); // SetErrorInfo is called before dropping the Error object.
    let result: Result<()> = code.ok(); // GetErrorInfo is called to retrieve the error info.

    let error = result.unwrap_err();
    assert_eq!(error.code(), E_FAIL);
    assert_eq!(error.message(), "test info");
}

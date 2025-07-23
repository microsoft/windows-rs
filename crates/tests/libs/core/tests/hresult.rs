use windows::{core::*, Win32::Foundation::*};

#[test]
fn from_into() {
    let error = Error::new(E_FAIL, "test info");
    let code: HRESULT = error.into(); // SetErrorInfo is called before dropping the Error object.

    let error = Error::from(code); // GetErrorInfo is called to retrieve the error info.
    assert_eq!(error.code(), E_FAIL);
    assert_eq!(error.message(), "test info");


}

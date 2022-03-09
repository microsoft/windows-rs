use windows::{core::*, Win32::Foundation::*};

#[test]
fn hresult() -> Result<()> {
    let _: HRESULT = S_OK;
    let _: HRESULT = E_INVALIDARG;
    let error: Error = E_INVALIDARG.into();

    assert_eq!(error.code(), E_INVALIDARG);
    assert_eq!(E_INVALIDARG.is_ok(), false);
    assert_eq!(E_INVALIDARG.is_err(), true);
    assert_eq!(S_OK.is_ok(), true);
    assert_eq!(S_OK.is_err(), false);

    assert_eq!(format!("{:?}", S_OK), "HRESULT(0x00000000)");
    assert_eq!(format!("{:?}", E_INVALIDARG), "HRESULT(0x80070057)");

    S_OK.ok()
}

#[test]
fn win32_error() -> Result<()> {
    let _: WIN32_ERROR = ERROR_SUCCESS;
    let _: WIN32_ERROR = ERROR_BAD_ARGUMENTS;
    let error: Error = ERROR_BAD_ARGUMENTS.into();
    let hresult: HRESULT = ERROR_BAD_ARGUMENTS.into();

    assert_eq!(error.code(), hresult);
    assert_eq!(error.win32_error(), Some(ERROR_BAD_ARGUMENTS));
    assert_eq!(ERROR_BAD_ARGUMENTS.is_ok(), false);
    assert_eq!(ERROR_BAD_ARGUMENTS.is_err(), true);
    assert_eq!(ERROR_SUCCESS.is_ok(), true);
    assert_eq!(ERROR_SUCCESS.is_err(), false);

    assert_eq!(format!("{:?}", ERROR_SUCCESS), "WIN32_ERROR(0)");
    assert_eq!(format!("{:?}", ERROR_BAD_ARGUMENTS), "WIN32_ERROR(160)");

    ERROR_SUCCESS.ok()
}

#[test]
fn ntstatus() -> Result<()> {
    let _: NTSTATUS = STATUS_SUCCESS;
    let _: NTSTATUS = STATUS_NOT_FOUND;
    let error: Error = STATUS_NOT_FOUND.into();
    let hresult: HRESULT = STATUS_NOT_FOUND.into();

    assert_eq!(error.code(), hresult);
    assert_eq!(STATUS_NOT_FOUND.is_ok(), false);
    assert_eq!(STATUS_NOT_FOUND.is_err(), true);
    assert_eq!(STATUS_SUCCESS.is_ok(), true);
    assert_eq!(STATUS_SUCCESS.is_err(), false);

    assert_eq!(format!("{:?}", STATUS_SUCCESS), "NTSTATUS(0x00000000)");
    assert_eq!(format!("{:?}", STATUS_NOT_FOUND), "NTSTATUS(0xC0000225)");

    STATUS_SUCCESS.ok()
}

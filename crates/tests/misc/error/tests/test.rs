use windows::{core::*, Win32::Foundation::*, Win32::System::Rpc::*};

#[test]
fn hresult() -> Result<()> {
    let _: HRESULT = S_OK;
    let _: HRESULT = E_INVALIDARG;
    let error: Error = E_INVALIDARG.into();

    assert_eq!(error.code(), E_INVALIDARG);
    assert!(!E_INVALIDARG.is_ok());
    assert!(E_INVALIDARG.is_err());
    assert!(S_OK.is_ok());
    assert!(!S_OK.is_err());

    assert_eq!(format!("{S_OK:?}"), "HRESULT(0x00000000)");
    assert_eq!(format!("{E_INVALIDARG:?}"), "HRESULT(0x80070057)");

    S_OK.ok()
}

#[test]
fn win32_error() -> Result<()> {
    let _: WIN32_ERROR = ERROR_SUCCESS;
    let _: WIN32_ERROR = ERROR_BAD_ARGUMENTS;
    let error: Error = ERROR_BAD_ARGUMENTS.into();
    let hresult: HRESULT = ERROR_BAD_ARGUMENTS.into();

    assert_eq!(error.code(), hresult);
    assert_eq!(WIN32_ERROR::from_error(&error), Some(ERROR_BAD_ARGUMENTS));
    assert!(!ERROR_BAD_ARGUMENTS.is_ok());
    assert!(ERROR_BAD_ARGUMENTS.is_err());
    assert!(ERROR_SUCCESS.is_ok());
    assert!(!ERROR_SUCCESS.is_err());

    assert_eq!(format!("{ERROR_SUCCESS:?}"), "WIN32_ERROR(0)");
    assert_eq!(format!("{ERROR_BAD_ARGUMENTS:?}"), "WIN32_ERROR(160)");

    ERROR_SUCCESS.ok()
}

#[test]
fn ntstatus() -> Result<()> {
    let _: NTSTATUS = STATUS_SUCCESS;
    let _: NTSTATUS = STATUS_NOT_FOUND;
    let error: Error = STATUS_NOT_FOUND.into();
    let hresult: HRESULT = STATUS_NOT_FOUND.into();

    assert_eq!(error.code(), hresult);
    assert_eq!(error.message(), "The object was not found.");
    assert!(!STATUS_NOT_FOUND.is_ok());
    assert!(STATUS_NOT_FOUND.is_err());
    assert!(STATUS_SUCCESS.is_ok());
    assert!(!STATUS_SUCCESS.is_err());

    assert_eq!(format!("{STATUS_SUCCESS:?}"), "NTSTATUS(0)");
    assert_eq!(format!("{STATUS_NOT_FOUND:?}"), "NTSTATUS(-1073741275)");

    STATUS_SUCCESS.ok()
}

#[test]
fn rpc() -> Result<()> {
    helpers::set_thread_ui_language();

    let _: RPC_STATUS = RPC_S_OK;
    assert!(RPC_S_OK.is_ok());
    assert!(!RPC_S_OK.is_err());

    let r: Result<()> = RPC_S_OK.ok();
    assert!(r.is_ok());

    assert_eq!(RPC_S_OK.to_hresult(), HRESULT(0));
    let hr: HRESULT = RPC_S_OK.into();
    assert_eq!(hr, HRESULT(0));

    let _: RPC_STATUS = RPC_S_NOT_LISTENING;
    assert!(!RPC_S_NOT_LISTENING.is_ok());
    assert!(RPC_S_NOT_LISTENING.is_err());

    let r: Result<()> = RPC_S_NOT_LISTENING.ok();
    assert!(r.is_err());

    assert_eq!(RPC_S_NOT_LISTENING.to_hresult(), HRESULT::from_win32(1715));
    let hr: HRESULT = RPC_S_NOT_LISTENING.into();
    assert_eq!(hr, HRESULT::from_win32(1715));

    let e: Error = RPC_S_NOT_LISTENING.into();
    assert_eq!(r.unwrap_err(), e);
    assert_eq!(e.code(), HRESULT::from_win32(1715));
    assert_eq!(e.message(), "The RPC server is not listening.");

    let r: Result<()> = unsafe { RpcServerListen(0, 0, 1).ok() };
    assert_eq!(r.unwrap_err().code(), RPC_S_MAX_CALLS_TOO_SMALL.into());

    RPC_S_OK.ok()
}

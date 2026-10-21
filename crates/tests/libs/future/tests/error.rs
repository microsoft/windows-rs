#![cfg(windows)]
// Cover both `GetResults` and the legacy `ErrorCode` path.
//
// This file is Windows-only because `windows_core::Error::new(code, message)` only stores the
// message text on Windows (via `IErrorInfo`/`OriginateError`); on other targets the message is
// dropped and `.message()` falls back to the HRESULT description.

use windows_core::*;
use windows_future::*;

const E_PROTOCOL_EXTENSIONS_NOT_SUPPORTED: HRESULT = HRESULT(0x83760003_u32 as _);

fn check_error_code(code: HRESULT) {
    let error = Error::from(code);
    assert_eq!(error.code(), E_PROTOCOL_EXTENSIONS_NOT_SUPPORTED);
    assert_eq!(error.message(), "async");
}

#[test]
fn action_ready() -> Result<()> {
    let a = IAsyncAction::ready(Err(Error::new(
        E_PROTOCOL_EXTENSIONS_NOT_SUPPORTED,
        "async",
    )));
    check_error_code(a.ErrorCode()?);
    let e = a.join().unwrap_err();
    assert_eq!(e.message(), "async");
    assert_eq!(e.code(), E_PROTOCOL_EXTENSIONS_NOT_SUPPORTED);
    assert_eq!(a.Status()?, AsyncStatus::Error);

    Ok(())
}

#[test]
fn action_with_progress_ready() -> Result<()> {
    let a = IAsyncActionWithProgress::<i32>::ready(Err(Error::new(
        E_PROTOCOL_EXTENSIONS_NOT_SUPPORTED,
        "async",
    )));
    let e = a.join().unwrap_err();
    assert_eq!(e.message(), "async");
    assert_eq!(e.code(), E_PROTOCOL_EXTENSIONS_NOT_SUPPORTED);
    check_error_code(a.ErrorCode()?);
    assert_eq!(a.Status()?, AsyncStatus::Error);

    Ok(())
}

#[test]
fn operation_ready() -> Result<()> {
    let a = IAsyncOperation::<i32>::ready(Err(Error::new(
        E_PROTOCOL_EXTENSIONS_NOT_SUPPORTED,
        "async",
    )));
    let e = a.join().unwrap_err();
    assert_eq!(e.message(), "async");
    assert_eq!(e.code(), E_PROTOCOL_EXTENSIONS_NOT_SUPPORTED);
    check_error_code(a.ErrorCode()?);
    assert_eq!(a.Status()?, AsyncStatus::Error);

    Ok(())
}

#[test]
fn operation_with_progress_ready() -> Result<()> {
    let a = IAsyncOperationWithProgress::<i32, i32>::ready(Err(Error::new(
        E_PROTOCOL_EXTENSIONS_NOT_SUPPORTED,
        "async",
    )));
    let e = a.join().unwrap_err();
    assert_eq!(e.message(), "async");
    assert_eq!(e.code(), E_PROTOCOL_EXTENSIONS_NOT_SUPPORTED);
    check_error_code(a.ErrorCode()?);
    assert_eq!(a.Status()?, AsyncStatus::Error);

    Ok(())
}

#[test]
fn action_spawn() -> Result<()> {
    let error_clone = Error::new(E_PROTOCOL_EXTENSIONS_NOT_SUPPORTED, "async");
    let a = IAsyncAction::spawn(move || Err(error_clone));
    // Wait for completion without consuming the stored error through GetResults.
    let (sender, receiver) = std::sync::mpsc::channel();
    a.SetCompleted(&AsyncActionCompletedHandler::new(move |_, _| {
        sender.send(()).unwrap();
        Ok(())
    }))?;
    receiver.recv().unwrap();
    check_error_code(a.ErrorCode()?);
    let e = a.join().unwrap_err();
    assert_eq!(e.message(), "async");
    assert_eq!(e.code(), E_PROTOCOL_EXTENSIONS_NOT_SUPPORTED);
    assert_eq!(a.Status()?, AsyncStatus::Error);

    Ok(())
}

#[test]
fn operation_spawn() -> Result<()> {
    let error_clone = Error::new(E_PROTOCOL_EXTENSIONS_NOT_SUPPORTED, "async");
    let a = IAsyncOperation::<i32>::spawn(move || Err(error_clone));
    let e = a.join().unwrap_err();
    assert_eq!(e.message(), "async");
    assert_eq!(e.code(), E_PROTOCOL_EXTENSIONS_NOT_SUPPORTED);
    check_error_code(a.ErrorCode()?);
    assert_eq!(a.Status()?, AsyncStatus::Error);

    Ok(())
}

#[test]
fn action_with_progress_spawn() -> Result<()> {
    let error_clone = Error::new(E_PROTOCOL_EXTENSIONS_NOT_SUPPORTED, "async");
    let a = IAsyncActionWithProgress::<i32>::spawn(move || Err(error_clone));
    let e = a.join().unwrap_err();
    assert_eq!(e.message(), "async");
    assert_eq!(e.code(), E_PROTOCOL_EXTENSIONS_NOT_SUPPORTED);
    check_error_code(a.ErrorCode()?);
    assert_eq!(a.Status()?, AsyncStatus::Error);

    Ok(())
}

#[test]
fn operation_with_progress_spawn() -> Result<()> {
    let error_clone = Error::new(E_PROTOCOL_EXTENSIONS_NOT_SUPPORTED, "async");
    let a = IAsyncOperationWithProgress::<i32, i32>::spawn(move || Err(error_clone));
    let e = a.join().unwrap_err();
    assert_eq!(e.message(), "async");
    assert_eq!(e.code(), E_PROTOCOL_EXTENSIONS_NOT_SUPPORTED);
    check_error_code(a.ErrorCode()?);
    assert_eq!(a.Status()?, AsyncStatus::Error);

    Ok(())
}

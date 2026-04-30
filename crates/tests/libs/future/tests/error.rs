#![cfg(windows)]
// Tests error propagation for all stock implementations. Here we're using `get` which calls `GetResults`
// as that is the normal path for most callers. Older callers may also use `ErrorCode` so that is tested
// as well.
//
// This file is Windows-only because `windows_core::Error::new(code, message)` only stores the
// message text on Windows (via `IErrorInfo`/`OriginateError`); on other targets the message is
// dropped and `.message()` falls back to the HRESULT description.

use windows_core::*;
use windows_future::*;

const E_PROTOCOL_EXTENSIONS_NOT_SUPPORTED: HRESULT = HRESULT(0x83760003_u32 as _);

#[test]
fn action_ready() -> Result<()> {
    let a = IAsyncAction::ready(Err(Error::new(
        E_PROTOCOL_EXTENSIONS_NOT_SUPPORTED,
        "async",
    )));
    let e = a.join().unwrap_err();
    assert_eq!(e.message(), "async");
    assert_eq!(e.code(), E_PROTOCOL_EXTENSIONS_NOT_SUPPORTED);
    assert_eq!(a.ErrorCode()?, E_PROTOCOL_EXTENSIONS_NOT_SUPPORTED);
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
    assert_eq!(a.ErrorCode()?, E_PROTOCOL_EXTENSIONS_NOT_SUPPORTED);
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
    assert_eq!(a.ErrorCode()?, E_PROTOCOL_EXTENSIONS_NOT_SUPPORTED);
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
    assert_eq!(a.ErrorCode()?, E_PROTOCOL_EXTENSIONS_NOT_SUPPORTED);
    assert_eq!(a.Status()?, AsyncStatus::Error);

    Ok(())
}

#[test]
fn action_spawn() -> Result<()> {
    let error_clone = Error::new(E_PROTOCOL_EXTENSIONS_NOT_SUPPORTED, "async");
    let a = IAsyncAction::spawn(move || Err(error_clone));
    let e = a.join().unwrap_err();
    assert_eq!(e.message(), "async");
    assert_eq!(e.code(), E_PROTOCOL_EXTENSIONS_NOT_SUPPORTED);
    assert_eq!(a.ErrorCode()?, E_PROTOCOL_EXTENSIONS_NOT_SUPPORTED);
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
    assert_eq!(a.ErrorCode()?, E_PROTOCOL_EXTENSIONS_NOT_SUPPORTED);
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
    assert_eq!(a.ErrorCode()?, E_PROTOCOL_EXTENSIONS_NOT_SUPPORTED);
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
    assert_eq!(a.ErrorCode()?, E_PROTOCOL_EXTENSIONS_NOT_SUPPORTED);
    assert_eq!(a.Status()?, AsyncStatus::Error);

    Ok(())
}

// Tests error propagation for all stock implementations. Here we're using `get` which calls `GetResults`
// as that is the normal path for most callers. Older callers may also use `ErrorCode` so that is tested
// as well.

use windows::{core::*, Win32::Foundation::*};
use windows_future::*;

#[test]
fn action_ready() -> Result<(), HRESULT> {
    let a = IAsyncAction::ready(Err(E_PROTOCOL_EXTENSIONS_NOT_SUPPORTED.into()));
    let e = a.get().unwrap_err();
    assert_eq!(e.message(), "Protocol extensions are not supported.");
    assert_eq!(e, E_PROTOCOL_EXTENSIONS_NOT_SUPPORTED);
    assert_eq!(a.ErrorCode()?, E_PROTOCOL_EXTENSIONS_NOT_SUPPORTED);
    assert_eq!(a.Status()?, AsyncStatus::Error);

    Ok(())
}

#[test]
fn action_with_progress_ready() -> Result<(), HRESULT> {
    let a = IAsyncActionWithProgress::<i32>::ready(Err(E_PROTOCOL_EXTENSIONS_NOT_SUPPORTED.into()));
    let e = a.get().unwrap_err();
    assert_eq!(e.message(), "Protocol extensions are not supported.");
    assert_eq!(e, E_PROTOCOL_EXTENSIONS_NOT_SUPPORTED);
    assert_eq!(a.ErrorCode()?, E_PROTOCOL_EXTENSIONS_NOT_SUPPORTED);
    assert_eq!(a.Status()?, AsyncStatus::Error);

    Ok(())
}

#[test]
fn operation_ready() -> Result<(), HRESULT> {
    let a = IAsyncOperation::<i32>::ready(Err(E_PROTOCOL_EXTENSIONS_NOT_SUPPORTED.into()));
    let e = a.get().unwrap_err();
    assert_eq!(e.message(), "Protocol extensions are not supported.");
    assert_eq!(e, E_PROTOCOL_EXTENSIONS_NOT_SUPPORTED);
    assert_eq!(a.ErrorCode()?, E_PROTOCOL_EXTENSIONS_NOT_SUPPORTED);
    assert_eq!(a.Status()?, AsyncStatus::Error);

    Ok(())
}

#[test]
fn operation_with_progress_ready() -> Result<(), HRESULT> {
    let a = IAsyncOperationWithProgress::<i32, i32>::ready(Err(
        E_PROTOCOL_EXTENSIONS_NOT_SUPPORTED.into(),
    ));
    let e = a.get().unwrap_err();
    assert_eq!(e.message(), "Protocol extensions are not supported.");
    assert_eq!(e, E_PROTOCOL_EXTENSIONS_NOT_SUPPORTED);
    assert_eq!(a.ErrorCode()?, E_PROTOCOL_EXTENSIONS_NOT_SUPPORTED);
    assert_eq!(a.Status()?, AsyncStatus::Error);

    Ok(())
}

#[test]
fn action_spawn() -> Result<(), HRESULT> {
    let a = IAsyncAction::spawn(move || Err(E_PROTOCOL_EXTENSIONS_NOT_SUPPORTED.into()));
    let e = a.get().unwrap_err();
    assert_eq!(e.message(), "Protocol extensions are not supported.");
    assert_eq!(e, E_PROTOCOL_EXTENSIONS_NOT_SUPPORTED);
    assert_eq!(a.ErrorCode()?, E_PROTOCOL_EXTENSIONS_NOT_SUPPORTED);
    assert_eq!(a.Status()?, AsyncStatus::Error);

    Ok(())
}

#[test]
fn operation_spawn() -> Result<(), HRESULT> {
    let a = IAsyncOperation::<i32>::spawn(move || Err(E_PROTOCOL_EXTENSIONS_NOT_SUPPORTED.into()));
    let e = a.get().unwrap_err();
    assert_eq!(e.message(), "Protocol extensions are not supported.");
    assert_eq!(e, E_PROTOCOL_EXTENSIONS_NOT_SUPPORTED);
    assert_eq!(a.ErrorCode()?, E_PROTOCOL_EXTENSIONS_NOT_SUPPORTED);
    assert_eq!(a.Status()?, AsyncStatus::Error);

    Ok(())
}

#[test]
fn action_with_progress_spawn() -> Result<(), HRESULT> {
    let a = IAsyncActionWithProgress::<i32>::spawn(move || {
        Err(E_PROTOCOL_EXTENSIONS_NOT_SUPPORTED.into())
    });
    let e = a.get().unwrap_err();
    assert_eq!(e.message(), "Protocol extensions are not supported.");
    assert_eq!(e, E_PROTOCOL_EXTENSIONS_NOT_SUPPORTED);
    assert_eq!(a.ErrorCode()?, E_PROTOCOL_EXTENSIONS_NOT_SUPPORTED);
    assert_eq!(a.Status()?, AsyncStatus::Error);

    Ok(())
}

#[test]
fn operation_with_progress_spawn() -> Result<(), HRESULT> {
    let a = IAsyncOperationWithProgress::<i32, i32>::spawn(move || {
        Err(E_PROTOCOL_EXTENSIONS_NOT_SUPPORTED.into())
    });
    let e = a.get().unwrap_err();
    assert_eq!(e.message(), "Protocol extensions are not supported.");
    assert_eq!(e, E_PROTOCOL_EXTENSIONS_NOT_SUPPORTED);
    assert_eq!(a.ErrorCode()?, E_PROTOCOL_EXTENSIONS_NOT_SUPPORTED);
    assert_eq!(a.Status()?, AsyncStatus::Error);

    Ok(())
}

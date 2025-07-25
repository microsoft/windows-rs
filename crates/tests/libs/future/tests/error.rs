// Tests error propagation for all stock implementations. Here we're using `get` which calls `GetResults`
// as that is the normal path for most callers. Older callers may also use `ErrorCode` so that is tested
// as well.

use windows::{core::*, Win32::Foundation::*};
use windows_future::*;

// Used to clear any thread error information to make the async objects preserve error information.
fn clear_thread() {
    windows_link::link!("kernel32.dll" "system" fn SetLastError(code: u32));
    windows_link::link!("oleaut32.dll" "system" fn SetErrorInfo(reserved: u32, object: usize) -> i32);

    unsafe {
        SetLastError(0);
        SetErrorInfo(0, 0);
    }
}

#[test]
fn action_ready() -> Result<(), HRESULT> {
    let a = IAsyncAction::ready(Err(Error::new(
        E_PROTOCOL_EXTENSIONS_NOT_SUPPORTED,
        "async",
    )));
    clear_thread();
    let e = Error::from(a.get().unwrap_err());
    assert_eq!(e.message(), "async");
    assert_eq!(e.code(), E_PROTOCOL_EXTENSIONS_NOT_SUPPORTED);
    assert_eq!(a.ErrorCode()?, E_PROTOCOL_EXTENSIONS_NOT_SUPPORTED);
    assert_eq!(a.Status()?, AsyncStatus::Error);

    Ok(())
}

#[test]
fn action_with_progress_ready() -> Result<(), HRESULT> {
    let a = IAsyncActionWithProgress::<i32>::ready(Err(Error::new(
        E_PROTOCOL_EXTENSIONS_NOT_SUPPORTED,
        "async",
    )));
    clear_thread();
    let e = Error::from(a.get().unwrap_err());
    assert_eq!(e.message(), "async");
    assert_eq!(e.code(), E_PROTOCOL_EXTENSIONS_NOT_SUPPORTED);
    assert_eq!(a.ErrorCode()?, E_PROTOCOL_EXTENSIONS_NOT_SUPPORTED);
    assert_eq!(a.Status()?, AsyncStatus::Error);

    Ok(())
}

#[test]
fn operation_ready() -> Result<(), HRESULT> {
    let a = IAsyncOperation::<i32>::ready(Err(Error::new(
        E_PROTOCOL_EXTENSIONS_NOT_SUPPORTED,
        "async",
    )));
    clear_thread();
    let e = Error::from(a.get().unwrap_err());
    assert_eq!(e.message(), "async");
    assert_eq!(e.code(), E_PROTOCOL_EXTENSIONS_NOT_SUPPORTED);
    assert_eq!(a.ErrorCode()?, E_PROTOCOL_EXTENSIONS_NOT_SUPPORTED);
    assert_eq!(a.Status()?, AsyncStatus::Error);

    Ok(())
}

#[test]
fn operation_with_progress_ready() -> Result<(), HRESULT> {
    let a = IAsyncOperationWithProgress::<i32, i32>::ready(Err(Error::new(
        E_PROTOCOL_EXTENSIONS_NOT_SUPPORTED,
        "async",
    )));
    clear_thread();
    let e = Error::from(a.get().unwrap_err());
    assert_eq!(e.message(), "async");
    assert_eq!(e.code(), E_PROTOCOL_EXTENSIONS_NOT_SUPPORTED);
    assert_eq!(a.ErrorCode()?, E_PROTOCOL_EXTENSIONS_NOT_SUPPORTED);
    assert_eq!(a.Status()?, AsyncStatus::Error);

    Ok(())
}

#[test]
fn action_spawn() -> Result<(), HRESULT> {
    let error_clone = Error::new(E_PROTOCOL_EXTENSIONS_NOT_SUPPORTED, "async");
    let a = IAsyncAction::spawn(move || Err(error_clone));
    let e = Error::from(a.get().unwrap_err());
    assert_eq!(e.message(), "async");
    assert_eq!(e.code(), E_PROTOCOL_EXTENSIONS_NOT_SUPPORTED);
    assert_eq!(a.ErrorCode()?, E_PROTOCOL_EXTENSIONS_NOT_SUPPORTED);
    assert_eq!(a.Status()?, AsyncStatus::Error);

    Ok(())
}

#[test]
fn operation_spawn() -> Result<(), HRESULT> {
    let error_clone = Error::new(E_PROTOCOL_EXTENSIONS_NOT_SUPPORTED, "async");
    let a = IAsyncOperation::<i32>::spawn(move || Err(error_clone));
    let e = Error::from(a.get().unwrap_err());
    assert_eq!(e.message(), "async");
    assert_eq!(e.code(), E_PROTOCOL_EXTENSIONS_NOT_SUPPORTED);
    assert_eq!(a.ErrorCode()?, E_PROTOCOL_EXTENSIONS_NOT_SUPPORTED);
    assert_eq!(a.Status()?, AsyncStatus::Error);

    Ok(())
}

#[test]
fn action_with_progress_spawn() -> Result<(), HRESULT> {
    let error_clone = Error::new(E_PROTOCOL_EXTENSIONS_NOT_SUPPORTED, "async");
    let a = IAsyncActionWithProgress::<i32>::spawn(move || Err(error_clone));
    let e = Error::from(a.get().unwrap_err());
    assert_eq!(e.message(), "async");
    assert_eq!(e.code(), E_PROTOCOL_EXTENSIONS_NOT_SUPPORTED);
    assert_eq!(a.ErrorCode()?, E_PROTOCOL_EXTENSIONS_NOT_SUPPORTED);
    assert_eq!(a.Status()?, AsyncStatus::Error);

    Ok(())
}

#[test]
fn operation_with_progress_spawn() -> Result<(), HRESULT> {
    let error_clone = Error::new(E_PROTOCOL_EXTENSIONS_NOT_SUPPORTED, "async");
    let a = IAsyncOperationWithProgress::<i32, i32>::spawn(move || Err(error_clone));
    let e = Error::from(a.get().unwrap_err());
    assert_eq!(e.message(), "async");
    assert_eq!(e.code(), E_PROTOCOL_EXTENSIONS_NOT_SUPPORTED);
    assert_eq!(a.ErrorCode()?, E_PROTOCOL_EXTENSIONS_NOT_SUPPORTED);
    assert_eq!(a.Status()?, AsyncStatus::Error);

    Ok(())
}

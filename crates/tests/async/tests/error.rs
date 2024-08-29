// Tests error propagation for all stock implementations. Here we're using `get` which calls `GetResults`
// as that is the normal path for most callers. Older callers may also use `ErrorCode` so that is tested
// as well.

use windows::{core::*, Foundation::*, Win32::Foundation::*};

#[test]
fn test() -> Result<()> {
    let error = Error::new(E_PROTOCOL_EXTENSIONS_NOT_SUPPORTED, "async");

    let a = IAsyncAction::ready(Err(error.clone()));
    let e = a.get().unwrap_err();
    assert_eq!(e.message(), "async");
    assert_eq!(e.code(), E_PROTOCOL_EXTENSIONS_NOT_SUPPORTED);
    assert_eq!(a.ErrorCode()?, E_PROTOCOL_EXTENSIONS_NOT_SUPPORTED);
    assert_eq!(a.Status()?, AsyncStatus::Error);

    let a = IAsyncActionWithProgress::<i32>::ready(Err(error.clone()));
    let e = a.get().unwrap_err();
    assert_eq!(e.message(), "async");
    assert_eq!(e.code(), E_PROTOCOL_EXTENSIONS_NOT_SUPPORTED);
    assert_eq!(a.ErrorCode()?, E_PROTOCOL_EXTENSIONS_NOT_SUPPORTED);
    assert_eq!(a.Status()?, AsyncStatus::Error);

    let a = IAsyncOperation::<i32>::ready(Err(error.clone()));
    let e = a.get().unwrap_err();
    assert_eq!(e.message(), "async");
    assert_eq!(e.code(), E_PROTOCOL_EXTENSIONS_NOT_SUPPORTED);
    assert_eq!(a.ErrorCode()?, E_PROTOCOL_EXTENSIONS_NOT_SUPPORTED);
    assert_eq!(a.Status()?, AsyncStatus::Error);

    let a = IAsyncOperationWithProgress::<i32, i32>::ready(Err(error.clone()));
    let e = a.get().unwrap_err();
    assert_eq!(e.message(), "async");
    assert_eq!(e.code(), E_PROTOCOL_EXTENSIONS_NOT_SUPPORTED);
    assert_eq!(a.ErrorCode()?, E_PROTOCOL_EXTENSIONS_NOT_SUPPORTED);
    assert_eq!(a.Status()?, AsyncStatus::Error);

    let a = IAsyncAction::spawn(move || Err(error.clone()));
    let e = a.get().unwrap_err();
    assert_eq!(e.message(), "async");
    assert_eq!(e.code(), E_PROTOCOL_EXTENSIONS_NOT_SUPPORTED);
    assert_eq!(a.ErrorCode()?, E_PROTOCOL_EXTENSIONS_NOT_SUPPORTED);
    assert_eq!(a.Status()?, AsyncStatus::Error);

    Ok(())
}

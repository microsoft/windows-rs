// The stock `ready` implementations are never in the `Started` state as they're either `Completed` and `Error`.
// This tests the `spawn` implementations to confirm that we can observe the `Started` state.
// The `GetResults` method may not be called in this state.

use windows::{core::*, Win32::Foundation::*};
use windows_future::*;

#[test]
fn action() -> Result<()> {
    let (send, recv) = std::sync::mpsc::channel::<()>();

    let a = IAsyncAction::spawn(move || {
        recv.recv().unwrap();
        Ok(())
    });

    assert_eq!(a.Status()?, AsyncStatus::Started);
    assert_eq!(a.GetResults().unwrap_err().code(), E_ILLEGAL_METHOD_CALL);
    send.send(()).unwrap();
    a.join()?;

    Ok(())
}

#[test]
fn operation() -> Result<()> {
    let (send, recv) = std::sync::mpsc::channel::<()>();

    let a = IAsyncOperation::spawn(move || {
        recv.recv().unwrap();
        Ok(123)
    });

    assert_eq!(a.Status()?, AsyncStatus::Started);
    assert_eq!(a.GetResults().unwrap_err().code(), E_ILLEGAL_METHOD_CALL);
    send.send(()).unwrap();
    assert_eq!(a.join()?, 123);

    Ok(())
}

#[test]
fn action_with_progress() -> Result<()> {
    let (send, recv) = std::sync::mpsc::channel::<()>();

    let a = IAsyncActionWithProgress::<i32>::spawn(move || {
        recv.recv().unwrap();
        Ok(())
    });

    assert_eq!(a.Status()?, AsyncStatus::Started);
    assert_eq!(a.GetResults().unwrap_err().code(), E_ILLEGAL_METHOD_CALL);
    send.send(()).unwrap();
    a.join()?;

    Ok(())
}

#[test]
fn operation_with_progress() -> Result<()> {
    let (send, recv) = std::sync::mpsc::channel::<()>();

    let a = IAsyncOperationWithProgress::<i32, i32>::spawn(move || {
        recv.recv().unwrap();
        Ok(123)
    });

    assert_eq!(a.Status()?, AsyncStatus::Started);
    assert_eq!(a.GetResults().unwrap_err().code(), E_ILLEGAL_METHOD_CALL);
    send.send(()).unwrap();
    assert_eq!(a.join()?, 123);

    Ok(())
}

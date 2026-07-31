// `GetResults` is invalid while `spawn` remains in the observable `Started` state.

use windows_core::*;
use windows_future::*;

const E_ILLEGAL_METHOD_CALL: HRESULT = HRESULT(0x8000000E_u32 as _);

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

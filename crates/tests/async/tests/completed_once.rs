// Implementations of `SetCompleted` must fail with `E_ILLEGAL_DELEGATE_ASSIGNMENT` if they are called twice.
// Also tests that any error from the handler is ignored by the implementation.

use windows::{core::*, Foundation::*, Win32::Foundation::*};

#[test]
fn test() -> Result<()> {
    let a = IAsyncAction::ready(Ok(()));
    let (send, recv) = std::sync::mpsc::channel::<()>();
    let a_clone = a.clone();

    a.SetCompleted(&AsyncActionCompletedHandler::new(move |sender, status| {
        assert_eq!(sender.unwrap(), &a_clone);
        assert_eq!(status, AsyncStatus::Completed);
        send.send(()).unwrap();
        Err(E_PROTOCOL_EXTENSIONS_NOT_SUPPORTED.into())
    }))?;

    recv.recv().unwrap();

    assert_eq!(
        a.SetCompleted(None).unwrap_err().code(),
        E_ILLEGAL_DELEGATE_ASSIGNMENT
    );

    let a = IAsyncActionWithProgress::<i32>::ready(Ok(()));
    let (send, recv) = std::sync::mpsc::channel::<()>();
    let a_clone = a.clone();

    a.SetCompleted(&AsyncActionWithProgressCompletedHandler::new(move |sender, status| {
        assert_eq!(sender.unwrap(), &a_clone);
        assert_eq!(status, AsyncStatus::Completed);
        send.send(()).unwrap();
        Err(E_PROTOCOL_EXTENSIONS_NOT_SUPPORTED.into())
    }))?;

    recv.recv().unwrap();

    assert_eq!(
        a.SetCompleted(None).unwrap_err().code(),
        E_ILLEGAL_DELEGATE_ASSIGNMENT
    );

    let a = IAsyncOperation::<i32>::ready(Ok(1));
    let (send, recv) = std::sync::mpsc::channel::<()>();
    let a_clone = a.clone();

    a.SetCompleted(&AsyncOperationCompletedHandler::new(move |sender, status| {
        assert_eq!(sender.unwrap(), &a_clone);
        assert_eq!(status, AsyncStatus::Completed);
        send.send(()).unwrap();
        Err(E_PROTOCOL_EXTENSIONS_NOT_SUPPORTED.into())
    }))?;

    recv.recv().unwrap();

    assert_eq!(
        a.SetCompleted(None).unwrap_err().code(),
        E_ILLEGAL_DELEGATE_ASSIGNMENT
    );

    let a = IAsyncOperationWithProgress::<i32, i32>::ready(Ok(1));
    let (send, recv) = std::sync::mpsc::channel::<()>();
    let a_clone = a.clone();

    a.SetCompleted(&AsyncOperationWithProgressCompletedHandler::new(move |sender, status| {
        assert_eq!(sender.unwrap(), &a_clone);
        assert_eq!(status, AsyncStatus::Completed);
        send.send(()).unwrap();
        Err(E_PROTOCOL_EXTENSIONS_NOT_SUPPORTED.into())
    }))?;

    recv.recv().unwrap();

    assert_eq!(
        a.SetCompleted(None).unwrap_err().code(),
        E_ILLEGAL_DELEGATE_ASSIGNMENT
    );

    let a = IAsyncAction::spawn(|| Ok(()));
    let (send, recv) = std::sync::mpsc::channel::<()>();
    let a_clone = a.clone();

    a.SetCompleted(&AsyncActionCompletedHandler::new(move |sender, status| {
        assert_eq!(sender.unwrap(), &a_clone);
        assert_eq!(status, AsyncStatus::Completed);
        send.send(()).unwrap();
        Err(E_PROTOCOL_EXTENSIONS_NOT_SUPPORTED.into())
    }))?;

    recv.recv().unwrap();

    assert_eq!(
        a.SetCompleted(None).unwrap_err().code(),
        E_ILLEGAL_DELEGATE_ASSIGNMENT
    );
    
    Ok(())
}

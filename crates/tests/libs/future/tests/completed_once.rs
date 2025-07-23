// Implementations of `SetCompleted` must fail with `E_ILLEGAL_DELEGATE_ASSIGNMENT` if they are called twice.
// Also tests that any error from the handler is ignored by the implementation.

use windows::{core::*, Win32::Foundation::*};
use windows_future::*;

#[test]
fn action_ready() -> Result<(), HRESULT> {
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
        a.SetCompleted(None).unwrap_err(),
        E_ILLEGAL_DELEGATE_ASSIGNMENT
    );

    a.GetResults()?;
    Ok(())
}

#[test]
fn action_with_progress_ready() -> Result<(), HRESULT> {
    let a = IAsyncActionWithProgress::<i32>::ready(Ok(()));
    let (send, recv) = std::sync::mpsc::channel::<()>();
    let a_clone = a.clone();

    a.SetCompleted(&AsyncActionWithProgressCompletedHandler::new(
        move |sender, status| {
            assert_eq!(sender.unwrap(), &a_clone);
            assert_eq!(status, AsyncStatus::Completed);
            send.send(()).unwrap();
            Err(E_PROTOCOL_EXTENSIONS_NOT_SUPPORTED.into())
        },
    ))?;

    recv.recv().unwrap();

    assert_eq!(
        a.SetCompleted(None).unwrap_err(),
        E_ILLEGAL_DELEGATE_ASSIGNMENT
    );

    a.GetResults()?;
    Ok(())
}

#[test]
fn operation_ready() -> Result<(), HRESULT> {
    let a = IAsyncOperation::<i32>::ready(Ok(123));
    let (send, recv) = std::sync::mpsc::channel::<()>();
    let a_clone = a.clone();

    a.SetCompleted(&AsyncOperationCompletedHandler::new(
        move |sender, status| {
            assert_eq!(sender.unwrap(), &a_clone);
            assert_eq!(status, AsyncStatus::Completed);
            send.send(()).unwrap();
            Err(E_PROTOCOL_EXTENSIONS_NOT_SUPPORTED.into())
        },
    ))?;

    recv.recv().unwrap();

    assert_eq!(
        a.SetCompleted(None).unwrap_err(),
        E_ILLEGAL_DELEGATE_ASSIGNMENT
    );

    assert_eq!(a.GetResults()?, 123);
    Ok(())
}

#[test]
fn operation_with_progress_ready() -> Result<(), HRESULT> {
    let a = IAsyncOperationWithProgress::<i32, i32>::ready(Ok(123));
    let (send, recv) = std::sync::mpsc::channel::<()>();
    let a_clone = a.clone();

    a.SetCompleted(&AsyncOperationWithProgressCompletedHandler::new(
        move |sender, status| {
            assert_eq!(sender.unwrap(), &a_clone);
            assert_eq!(status, AsyncStatus::Completed);
            send.send(()).unwrap();
            Err(E_PROTOCOL_EXTENSIONS_NOT_SUPPORTED.into())
        },
    ))?;

    recv.recv().unwrap();

    assert_eq!(
        a.SetCompleted(None).unwrap_err(),
        E_ILLEGAL_DELEGATE_ASSIGNMENT
    );

    assert_eq!(a.GetResults()?, 123);
    Ok(())
}

#[test]
fn action_spawn() -> Result<(), HRESULT> {
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
        a.SetCompleted(None).unwrap_err(),
        E_ILLEGAL_DELEGATE_ASSIGNMENT
    );

    a.GetResults()?;
    Ok(())
}

#[test]
fn operation_spawn() -> Result<(), HRESULT> {
    let a = IAsyncOperation::spawn(|| Ok(123));
    let (send, recv) = std::sync::mpsc::channel::<()>();
    let a_clone = a.clone();

    a.SetCompleted(&AsyncOperationCompletedHandler::new(
        move |sender, status| {
            assert_eq!(sender.unwrap(), &a_clone);
            assert_eq!(status, AsyncStatus::Completed);
            send.send(()).unwrap();
            Err(E_PROTOCOL_EXTENSIONS_NOT_SUPPORTED.into())
        },
    ))?;

    recv.recv().unwrap();

    assert_eq!(
        a.SetCompleted(None).unwrap_err(),
        E_ILLEGAL_DELEGATE_ASSIGNMENT
    );

    assert_eq!(a.GetResults()?, 123);
    Ok(())
}

#[test]
fn action_with_progress_spawn() -> Result<(), HRESULT> {
    let a = IAsyncActionWithProgress::<i32>::spawn(|| Ok(()));
    let (send, recv) = std::sync::mpsc::channel::<()>();
    let a_clone = a.clone();

    a.SetCompleted(&AsyncActionWithProgressCompletedHandler::new(
        move |sender, status| {
            assert_eq!(sender.unwrap(), &a_clone);
            assert_eq!(status, AsyncStatus::Completed);
            send.send(()).unwrap();
            Err(E_PROTOCOL_EXTENSIONS_NOT_SUPPORTED.into())
        },
    ))?;

    recv.recv().unwrap();

    assert_eq!(
        a.SetCompleted(None).unwrap_err(),
        E_ILLEGAL_DELEGATE_ASSIGNMENT
    );

    a.GetResults()?;
    Ok(())
}

#[test]
fn operation_with_progress_spawn() -> Result<(), HRESULT> {
    let a = IAsyncOperationWithProgress::<i32, i32>::spawn(|| Ok(123));
    let (send, recv) = std::sync::mpsc::channel::<()>();
    let a_clone = a.clone();

    a.SetCompleted(&AsyncOperationWithProgressCompletedHandler::new(
        move |sender, status| {
            assert_eq!(sender.unwrap(), &a_clone);
            assert_eq!(status, AsyncStatus::Completed);
            send.send(()).unwrap();
            Err(E_PROTOCOL_EXTENSIONS_NOT_SUPPORTED.into())
        },
    ))?;

    recv.recv().unwrap();

    assert_eq!(
        a.SetCompleted(None).unwrap_err(),
        E_ILLEGAL_DELEGATE_ASSIGNMENT
    );

    assert_eq!(a.GetResults()?, 123);
    Ok(())
}

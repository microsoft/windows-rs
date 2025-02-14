// The stock `spawn` implementations can receive the `Completed` handler while still in the `Started` state
// and must hold on to the handler and call it when execution completes.

use std::sync::mpsc::channel;
use std::thread;
use windows::core::*;
use windows_future::*;

#[test]
fn action() -> Result<()> {
    let (spawn_start_send, spawn_start_recv) = channel();
    let (spawn_finish_send, spawn_finish_recv) = channel();
    let (completed_send, completed_recv) = channel();
    let test_thread = thread::current().id();

    let a = IAsyncAction::spawn(move || {
        let spawn_thread = thread::current().id();
        spawn_start_recv.recv().unwrap();
        spawn_finish_send.send(spawn_thread).unwrap();
        Ok(())
    });

    let a_clone = a.clone();

    a.SetCompleted(&AsyncActionCompletedHandler::new(move |sender, status| {
        let completed_thread = thread::current().id();
        assert_eq!(sender.unwrap(), &a_clone);
        assert_eq!(status, AsyncStatus::Completed);
        completed_send.send(completed_thread).unwrap();
        Ok(())
    }))?;

    assert_eq!(a.Status()?, AsyncStatus::Started);
    assert!(spawn_finish_recv.try_recv().is_err());
    assert!(completed_recv.try_recv().is_err());

    spawn_start_send.send(()).unwrap();
    let spawn_thread = spawn_finish_recv.recv().unwrap();
    let completed_thread = completed_recv.recv().unwrap();

    assert_ne!(test_thread, spawn_thread);
    assert_ne!(test_thread, completed_thread);
    assert_eq!(spawn_thread, completed_thread);

    a.GetResults()?;
    Ok(())
}

#[test]
fn operation() -> Result<()> {
    let (spawn_start_send, spawn_start_recv) = channel();
    let (spawn_finish_send, spawn_finish_recv) = channel();
    let (completed_send, completed_recv) = channel();
    let test_thread = thread::current().id();

    let a = IAsyncOperation::spawn(move || {
        let spawn_thread = thread::current().id();
        spawn_start_recv.recv().unwrap();
        spawn_finish_send.send(spawn_thread).unwrap();
        Ok(123)
    });

    let a_clone = a.clone();

    a.SetCompleted(&AsyncOperationCompletedHandler::new(
        move |sender, status| {
            let completed_thread = thread::current().id();
            assert_eq!(sender.unwrap(), &a_clone);
            assert_eq!(status, AsyncStatus::Completed);
            completed_send.send(completed_thread).unwrap();
            Ok(())
        },
    ))?;

    assert_eq!(a.Status()?, AsyncStatus::Started);
    assert!(spawn_finish_recv.try_recv().is_err());
    assert!(completed_recv.try_recv().is_err());

    spawn_start_send.send(()).unwrap();
    let spawn_thread = spawn_finish_recv.recv().unwrap();
    let completed_thread = completed_recv.recv().unwrap();

    assert_ne!(test_thread, spawn_thread);
    assert_ne!(test_thread, completed_thread);
    assert_eq!(spawn_thread, completed_thread);

    assert_eq!(a.GetResults()?, 123);
    Ok(())
}

#[test]
fn action_with_progress() -> Result<()> {
    let (spawn_start_send, spawn_start_recv) = channel();
    let (spawn_finish_send, spawn_finish_recv) = channel();
    let (completed_send, completed_recv) = channel();
    let test_thread = thread::current().id();

    let a = IAsyncActionWithProgress::<i32>::spawn(move || {
        let spawn_thread = thread::current().id();
        spawn_start_recv.recv().unwrap();
        spawn_finish_send.send(spawn_thread).unwrap();
        Ok(())
    });

    let a_clone = a.clone();

    a.SetCompleted(&AsyncActionWithProgressCompletedHandler::new(
        move |sender, status| {
            let completed_thread = thread::current().id();
            assert_eq!(sender.unwrap(), &a_clone);
            assert_eq!(status, AsyncStatus::Completed);
            completed_send.send(completed_thread).unwrap();
            Ok(())
        },
    ))?;

    assert_eq!(a.Status()?, AsyncStatus::Started);
    assert!(spawn_finish_recv.try_recv().is_err());
    assert!(completed_recv.try_recv().is_err());

    spawn_start_send.send(()).unwrap();
    let spawn_thread = spawn_finish_recv.recv().unwrap();
    let completed_thread = completed_recv.recv().unwrap();

    assert_ne!(test_thread, spawn_thread);
    assert_ne!(test_thread, completed_thread);
    assert_eq!(spawn_thread, completed_thread);

    a.GetResults()?;
    Ok(())
}

#[test]
fn operation_with_progress() -> Result<()> {
    let (spawn_start_send, spawn_start_recv) = channel();
    let (spawn_finish_send, spawn_finish_recv) = channel();
    let (completed_send, completed_recv) = channel();
    let test_thread = thread::current().id();

    let a = IAsyncOperationWithProgress::<i32, i32>::spawn(move || {
        let spawn_thread = thread::current().id();
        spawn_start_recv.recv().unwrap();
        spawn_finish_send.send(spawn_thread).unwrap();
        Ok(123)
    });

    let a_clone = a.clone();

    a.SetCompleted(&AsyncOperationWithProgressCompletedHandler::new(
        move |sender, status| {
            let completed_thread = thread::current().id();
            assert_eq!(sender.unwrap(), &a_clone);
            assert_eq!(status, AsyncStatus::Completed);
            completed_send.send(completed_thread).unwrap();
            Ok(())
        },
    ))?;

    assert_eq!(a.Status()?, AsyncStatus::Started);
    assert!(spawn_finish_recv.try_recv().is_err());
    assert!(completed_recv.try_recv().is_err());

    spawn_start_send.send(()).unwrap();
    let spawn_thread = spawn_finish_recv.recv().unwrap();
    let completed_thread = completed_recv.recv().unwrap();

    assert_ne!(test_thread, spawn_thread);
    assert_ne!(test_thread, completed_thread);
    assert_eq!(spawn_thread, completed_thread);

    assert_eq!(a.GetResults()?, 123);
    Ok(())
}

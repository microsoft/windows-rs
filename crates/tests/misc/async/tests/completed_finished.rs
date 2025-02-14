// The stock `spawn` implementations can receive the `Completed` handler after execution has finished
// and must call the handler immediately.

use std::sync::mpsc::channel;
use std::thread;
use windows::core::*;
use windows_future::*;

#[test]
fn action() -> Result<()> {
    let (spawn_finish_send, spawn_finish_recv) = channel();
    let (completed_send, completed_recv) = channel();

    let test_thread = thread::current().id();

    let a = IAsyncAction::spawn(move || {
        let spawn_thread = thread::current().id();

        spawn_finish_send.send(spawn_thread).unwrap();
        Ok(())
    });

    let spawn_thread = spawn_finish_recv.recv().unwrap();

    while a.Status()? == AsyncStatus::Started {
        println!("yield");
        thread::yield_now();
    }

    assert_eq!(a.Status()?, AsyncStatus::Completed);
    let a_clone = a.clone();

    a.SetCompleted(&AsyncActionCompletedHandler::new(move |sender, status| {
        let completed_thread = thread::current().id();

        assert_eq!(sender.unwrap(), &a_clone);
        assert_eq!(status, AsyncStatus::Completed);

        completed_send.send(completed_thread).unwrap();
        Ok(())
    }))?;

    let completed_thread = completed_recv.recv().unwrap();

    assert_ne!(test_thread, spawn_thread);
    assert_eq!(test_thread, completed_thread);

    a.GetResults()?;
    Ok(())
}

#[test]
fn operation() -> Result<()> {
    let (spawn_finish_send, spawn_finish_recv) = channel();
    let (completed_send, completed_recv) = channel();

    let test_thread = thread::current().id();

    let a = IAsyncOperation::spawn(move || {
        let spawn_thread = thread::current().id();

        spawn_finish_send.send(spawn_thread).unwrap();
        Ok(123)
    });

    let spawn_thread = spawn_finish_recv.recv().unwrap();

    while a.Status()? == AsyncStatus::Started {
        println!("yield");
        thread::yield_now();
    }

    assert_eq!(a.Status()?, AsyncStatus::Completed);
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

    let completed_thread = completed_recv.recv().unwrap();

    assert_ne!(test_thread, spawn_thread);
    assert_eq!(test_thread, completed_thread);

    assert_eq!(a.GetResults()?, 123);
    Ok(())
}

#[test]
fn action_with_progress() -> Result<()> {
    let (spawn_finish_send, spawn_finish_recv) = channel();
    let (completed_send, completed_recv) = channel();

    let test_thread = thread::current().id();

    let a = IAsyncActionWithProgress::<i32>::spawn(move || {
        let spawn_thread = thread::current().id();

        spawn_finish_send.send(spawn_thread).unwrap();
        Ok(())
    });

    let spawn_thread = spawn_finish_recv.recv().unwrap();

    while a.Status()? == AsyncStatus::Started {
        println!("yield");
        thread::yield_now();
    }

    assert_eq!(a.Status()?, AsyncStatus::Completed);
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

    let completed_thread = completed_recv.recv().unwrap();

    assert_ne!(test_thread, spawn_thread);
    assert_eq!(test_thread, completed_thread);

    a.GetResults()?;
    Ok(())
}

#[test]
fn operation_with_progress() -> Result<()> {
    let (spawn_finish_send, spawn_finish_recv) = channel();
    let (completed_send, completed_recv) = channel();

    let test_thread = thread::current().id();

    let a = IAsyncOperationWithProgress::<i32, i32>::spawn(move || {
        let spawn_thread = thread::current().id();

        spawn_finish_send.send(spawn_thread).unwrap();
        Ok(123)
    });

    let spawn_thread = spawn_finish_recv.recv().unwrap();

    while a.Status()? == AsyncStatus::Started {
        println!("yield");
        thread::yield_now();
    }

    assert_eq!(a.Status()?, AsyncStatus::Completed);
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

    let completed_thread = completed_recv.recv().unwrap();

    assert_ne!(test_thread, spawn_thread);
    assert_eq!(test_thread, completed_thread);

    assert_eq!(a.GetResults()?, 123);
    Ok(())
}

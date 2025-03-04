// The stock `spawn` implementation hold a strong reference to the async object on the thread pool
// so the caller can drop its reference if necessary.

use std::sync::mpsc::channel;
use windows::core::*;
use windows_future::*;

#[test]
fn action() -> Result<()> {
    let (spawn_start_send, spawn_start_recv) = channel();

    let a = IAsyncAction::spawn(move || {
        spawn_start_recv.recv().unwrap();
        Ok(())
    });

    drop(a);
    spawn_start_send.send(()).unwrap();

    Ok(())
}

#[test]
fn operation() -> Result<()> {
    let (spawn_start_send, spawn_start_recv) = channel();

    let a = IAsyncOperation::spawn(move || {
        spawn_start_recv.recv().unwrap();
        Ok(123)
    });

    drop(a);
    spawn_start_send.send(()).unwrap();

    Ok(())
}

#[test]
fn action_with_progress() -> Result<()> {
    let (spawn_start_send, spawn_start_recv) = channel();

    let a = IAsyncActionWithProgress::<i32>::spawn(move || {
        spawn_start_recv.recv().unwrap();
        Ok(())
    });

    drop(a);
    spawn_start_send.send(()).unwrap();

    Ok(())
}

#[test]
fn operation_with_progress() -> Result<()> {
    let (spawn_start_send, spawn_start_recv) = channel();

    let a = IAsyncOperationWithProgress::<i32, i32>::spawn(move || {
        spawn_start_recv.recv().unwrap();
        Ok(123)
    });

    drop(a);
    spawn_start_send.send(()).unwrap();

    Ok(())
}

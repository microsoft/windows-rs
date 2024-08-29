// The stock `spawn` implementation hold a strong reference to the async object on the thread pool
// so the caller can drop its reference if necessary.

use std::sync::mpsc::channel;
use windows::{core::*, Foundation::*};

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

// The stock `spawn` implementations can receive the `Completed` handler after execution has finished
// and must call the handler immediately.

use std::sync::mpsc::channel;
use std::thread;
use windows::{core::*, Foundation::*};

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

    a.SetCompleted(&AsyncActionCompletedHandler::new(move |_, _| {
        let completed_thread = thread::current().id();

        completed_send.send(completed_thread).unwrap();
        Ok(())
    }))?;

    let completed_thread = completed_recv.recv().unwrap();

    assert_ne!(test_thread, spawn_thread);
    assert_eq!(test_thread, completed_thread);

    Ok(())
}

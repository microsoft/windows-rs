use std::sync::mpsc::channel;
use std::thread;
use windows::{core::*, Foundation::*, Win32::Foundation::*};

#[test]
fn ok() -> Result<()> {
    let (send_start, recv_start) = channel();
    let (send_finish, recv_finish) = channel();

    let a = IAsyncAction::spawn(move || {
        recv_start.recv().unwrap();
        send_finish.send(thread::current().id()).unwrap();
        Ok(())
    });

    assert_eq!(a.Id()?, 1);
    assert_eq!(a.Status()?, AsyncStatus::Started);
    assert_eq!(a.ErrorCode()?, HRESULT(0));
    assert_eq!(a.Completed(), Err(Error::empty()));
    assert_eq!(a.GetResults().unwrap_err().code(), E_ILLEGAL_METHOD_CALL);
    a.Cancel()?;
    a.Close()?;

    send_start.send(()).unwrap();
    assert_ne!(recv_finish.recv().unwrap(), thread::current().id());

    Ok(())
}

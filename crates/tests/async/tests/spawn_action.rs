use std::sync::mpsc::channel;
use std::thread;
use windows::{core::*, Foundation::*, Win32::Foundation::*};

#[test]
fn get_ok() -> Result<()> {
    let a = IAsyncAction::spawn(|| Ok(()));
    let _: () = a.get()?;

    let _: IAsyncInfo = a.cast()?;
    assert_eq!(a.Id()?, 1);
    assert_eq!(a.Status()?, AsyncStatus::Completed);
    assert_eq!(a.ErrorCode()?, HRESULT(0));
    assert_eq!(a.Completed(), Err(Error::empty()));
    assert_eq!(a.GetResults(), Ok(()));
    a.Cancel()?;
    a.Close()?;

    assert_eq!(
        a.SetCompleted(None).unwrap_err().code(),
        E_ILLEGAL_DELEGATE_ASSIGNMENT
    );

    Ok(())
}

#[test]
fn get_err() -> Result<()> {
    let a = IAsyncAction::spawn(|| Err(Error::new(E_PROTOCOL_EXTENSIONS_NOT_SUPPORTED, "async")));

    let error = a.get().unwrap_err();
    assert_eq!(error.code(), E_PROTOCOL_EXTENSIONS_NOT_SUPPORTED);
    assert_eq!(error.message(), "async");

    let _: IAsyncInfo = a.cast()?;
    assert_eq!(a.Id()?, 1);
    assert_eq!(a.Status()?, AsyncStatus::Error);
    assert_eq!(a.ErrorCode()?, E_PROTOCOL_EXTENSIONS_NOT_SUPPORTED);
    assert_eq!(a.Completed(), Err(Error::empty()));

    let error = a.GetResults().unwrap_err();
    assert_eq!(error.code(), E_PROTOCOL_EXTENSIONS_NOT_SUPPORTED);
    assert_eq!(error.message(), "async");

    assert_eq!(
        a.SetCompleted(None).unwrap_err().code(),
        E_ILLEGAL_DELEGATE_ASSIGNMENT
    );

    Ok(())
}

#[test]
fn test_channel_completed() -> Result<()> {
    let (send_start, recv_start) = channel();
    let (send_finish, recv_finish) = channel();
    let (send_completed, recv_completed) = channel();

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

    let a_clone = a.clone();

    a.SetCompleted(&AsyncActionCompletedHandler::new(move |sender, status| {
        assert_eq!(sender.unwrap(), &a_clone);
        assert_eq!(status, AsyncStatus::Completed);
        send_completed.send(()).unwrap();
        Err(E_PROTOCOL_EXTENSIONS_NOT_SUPPORTED.into()) // handler error ignored
    }))?;

    recv_completed.recv().unwrap();

    assert_eq!(a.Status()?, AsyncStatus::Completed);
    assert_eq!(a.ErrorCode()?, HRESULT(0));
    a.GetResults()?;

    Ok(())
}

// #[test]
// fn test_channel_completed_late() -> Result<()> {
//     let (send_start, recv_start) = channel();
//     let (send_finish, recv_finish) = channel();
//     let (send_completed, recv_completed) = channel();

//     let a = IAsyncAction::spawn(move || {
//         recv_start.recv().unwrap();
//         send_finish.send(thread::current().id()).unwrap();
//         Ok(())
//     });

//     assert_eq!(a.Id()?, 1);
//     assert_eq!(a.Status()?, AsyncStatus::Started);
//     assert_eq!(a.ErrorCode()?, HRESULT(0));
//     assert_eq!(a.Completed(), Err(Error::empty()));
//     assert_eq!(a.GetResults().unwrap_err().code(), E_ILLEGAL_METHOD_CALL);
//     a.Cancel()?;
//     a.Close()?;

//     send_start.send(()).unwrap();
//     assert_ne!(recv_finish.recv().unwrap(), thread::current().id());

//     let a_clone = a.clone();

//     a.SetCompleted(&AsyncActionCompletedHandler::new(move |sender, status| {
//         assert_eq!(sender.unwrap(), &a_clone);
//         assert_eq!(status, AsyncStatus::Completed);
//         send_completed.send(()).unwrap();
//         Err(E_PROTOCOL_EXTENSIONS_NOT_SUPPORTED.into()) // handler error ignored
//     }))?;

//     recv_completed.recv().unwrap();

//     assert_eq!(a.Status()?, AsyncStatus::Completed);
//     assert_eq!(a.ErrorCode()?, HRESULT(0));
//     a.GetResults()?;

//     Ok(())
// }

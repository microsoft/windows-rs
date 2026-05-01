use std::sync::mpsc::channel;
use windows_future::*;
use windows_result::*;

#[cfg(windows)]
const E_FAIL: HRESULT = HRESULT(0x80004005_u32 as _);

#[test]
fn ok() -> Result<()> {
    let a = IAsyncAction::ready(Ok(()));

    a.when(move |r| {
        assert!(r.is_ok());
    })?;

    let (send, recv) = channel();
    let a = IAsyncAction::spawn(|| Ok(()));

    a.when(move |r| {
        assert!(r.is_ok());
        send.send(()).unwrap();
    })?;

    recv.recv().unwrap();

    let a = IAsyncActionWithProgress::<i32>::ready(Ok(()));

    a.when(move |r| {
        assert!(r.is_ok());
    })?;

    let (send, recv) = channel();
    let a = IAsyncActionWithProgress::<i32>::spawn(|| Ok(()));

    a.when(move |r| {
        assert!(r.is_ok());
        send.send(()).unwrap();
    })?;

    recv.recv().unwrap();

    let a = IAsyncOperation::<i32>::ready(Ok(123));

    a.when(move |r| {
        assert_eq!(r, Ok(123));
    })?;

    let (send, recv) = channel();
    let a = IAsyncOperation::<i32>::spawn(|| Ok(456));

    a.when(move |r| {
        assert_eq!(r, Ok(456));
        send.send(()).unwrap();
    })?;

    recv.recv().unwrap();

    let a = IAsyncOperationWithProgress::<i32, i32>::ready(Ok(123));

    a.when(move |r| {
        assert_eq!(r, Ok(123));
    })?;

    let (send, recv) = channel();
    let a = IAsyncOperationWithProgress::<i32, i32>::spawn(|| Ok(456));

    a.when(move |r| {
        assert_eq!(r, Ok(456));
        send.send(()).unwrap();
    })?;

    recv.recv().unwrap();

    Ok(())
}

#[test]
#[cfg(windows)]
// `Error::new(code, message)` only preserves the message text on Windows (via `IErrorInfo`),
// so `err.message()` assertions can only be verified on that target.
fn err() -> Result<()> {
    let a = IAsyncAction::ready(Err(Error::new(E_FAIL, "IAsyncAction-ready")));

    a.when(move |r| {
        let err = r.unwrap_err();
        assert_eq!(err.code(), E_FAIL);
        assert_eq!(err.message(), "IAsyncAction-ready");
    })?;

    let (send, recv) = channel();
    let a = IAsyncAction::spawn(|| Err(Error::new(E_FAIL, "IAsyncAction-spawn")));

    a.when(move |r| {
        let err = r.unwrap_err();
        assert_eq!(err.code(), E_FAIL);
        assert_eq!(err.message(), "IAsyncAction-spawn");
        send.send(()).unwrap();
    })?;

    recv.recv().unwrap();

    let a = IAsyncActionWithProgress::<i32>::ready(Err(Error::new(
        E_FAIL,
        "IAsyncActionWithProgress-ready",
    )));

    a.when(move |r| {
        let err = r.unwrap_err();
        assert_eq!(err.code(), E_FAIL);
        assert_eq!(err.message(), "IAsyncActionWithProgress-ready");
    })?;

    let (send, recv) = channel();
    let a = IAsyncActionWithProgress::<i32>::spawn(|| {
        Err(Error::new(E_FAIL, "IAsyncActionWithProgress-spawn"))
    });

    a.when(move |r| {
        let err = r.unwrap_err();
        assert_eq!(err.code(), E_FAIL);
        assert_eq!(err.message(), "IAsyncActionWithProgress-spawn");
        send.send(()).unwrap();
    })?;

    recv.recv().unwrap();

    let a = IAsyncOperation::<i32>::ready(Err(Error::new(E_FAIL, "IAsyncOperation-ready")));

    a.when(move |r| {
        let err = r.unwrap_err();
        assert_eq!(err.code(), E_FAIL);
        assert_eq!(err.message(), "IAsyncOperation-ready");
    })?;

    let (send, recv) = channel();
    let a = IAsyncOperation::<i32>::spawn(|| Err(Error::new(E_FAIL, "IAsyncOperation-spawn")));

    a.when(move |r| {
        let err = r.unwrap_err();
        assert_eq!(err.code(), E_FAIL);
        assert_eq!(err.message(), "IAsyncOperation-spawn");
        send.send(()).unwrap();
    })?;

    recv.recv().unwrap();

    let a = IAsyncOperationWithProgress::<i32, i32>::ready(Err(Error::new(
        E_FAIL,
        "IAsyncOperationWithProgress-ready",
    )));

    a.when(move |r| {
        let err = r.unwrap_err();
        assert_eq!(err.code(), E_FAIL);
        assert_eq!(err.message(), "IAsyncOperationWithProgress-ready");
    })?;

    let (send, recv) = channel();
    let a = IAsyncOperationWithProgress::<i32, i32>::spawn(|| {
        Err(Error::new(E_FAIL, "IAsyncOperationWithProgress-spawn"))
    });

    a.when(move |r| {
        let err = r.unwrap_err();
        assert_eq!(err.code(), E_FAIL);
        assert_eq!(err.message(), "IAsyncOperationWithProgress-spawn");
        send.send(()).unwrap();
    })?;

    recv.recv().unwrap();

    Ok(())
}

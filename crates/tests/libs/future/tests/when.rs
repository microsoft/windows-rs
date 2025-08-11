use windows::Win32::Foundation::E_FAIL;
use windows_future::*;
use windows_result::*;
use windows_threading::*;

#[test]
fn ok() -> Result<()> {
    let thread = thread_id();

    let a = IAsyncAction::ready(Ok(()));

    a.when(move |r| {
        assert!(r.is_ok());
        assert_eq!(thread, thread_id());
    })?;

    let a = IAsyncAction::spawn(|| Ok(()));

    a.when(move |r| {
        assert!(r.is_ok());
        assert_ne!(thread, thread_id());
    })?;

    let a = IAsyncActionWithProgress::<i32>::ready(Ok(()));

    a.when(move |r| {
        assert!(r.is_ok());
        assert_eq!(thread, thread_id());
    })?;

    let a = IAsyncActionWithProgress::<i32>::spawn(|| Ok(()));

    a.when(move |r| {
        assert!(r.is_ok());
        assert_ne!(thread, thread_id());
    })?;

    let a = IAsyncOperation::<i32>::ready(Ok(123));

    a.when(move |r| {
        assert_eq!(r, Ok(123));
        assert_eq!(thread, thread_id());
    })?;

    let a = IAsyncOperation::<i32>::spawn(|| Ok(456));

    a.when(move |r| {
        assert_eq!(r, Ok(456));
        assert_ne!(thread, thread_id());
    })?;

    let a = IAsyncOperationWithProgress::<i32, i32>::ready(Ok(123));

    a.when(move |r| {
        assert_eq!(r, Ok(123));
        assert_eq!(thread, thread_id());
    })?;

    let a = IAsyncOperationWithProgress::<i32, i32>::spawn(|| Ok(456));

    a.when(move |r| {
        assert_eq!(r, Ok(456));
        assert_ne!(thread, thread_id());
    })?;

    Ok(())
}

#[test]
fn err() -> Result<()> {
    let thread = thread_id();

    let a = IAsyncAction::ready(Err(Error::new(E_FAIL, "IAsyncAction-ready")));

    a.when(move |r| {
        let err = r.unwrap_err();
        assert_eq!(err.code(), E_FAIL);
        assert_eq!(err.message(), "IAsyncAction-ready");
        assert_eq!(thread, thread_id());
    })?;

    let a = IAsyncAction::spawn(|| Err(Error::new(E_FAIL, "IAsyncAction-spawn")));

    a.when(move |r| {
        let err = r.unwrap_err();
        assert_eq!(err.code(), E_FAIL);
        assert_eq!(err.message(), "IAsyncAction-spawn");
        assert_ne!(thread, thread_id());
    })?;

    let a = IAsyncActionWithProgress::<i32>::ready(Err(Error::new(
        E_FAIL,
        "IAsyncActionWithProgress-ready",
    )));

    a.when(move |r| {
        let err = r.unwrap_err();
        assert_eq!(err.code(), E_FAIL);
        assert_eq!(err.message(), "IAsyncActionWithProgress-ready");
        assert_eq!(thread, thread_id());
    })?;

    let a = IAsyncActionWithProgress::<i32>::spawn(|| {
        Err(Error::new(E_FAIL, "IAsyncActionWithProgress-spawn"))
    });

    a.when(move |r| {
        let err = r.unwrap_err();
        assert_eq!(err.code(), E_FAIL);
        assert_eq!(err.message(), "IAsyncActionWithProgress-spawn");
        assert_ne!(thread, thread_id());
    })?;

    let a = IAsyncOperation::<i32>::ready(Err(Error::new(E_FAIL, "IAsyncOperation-ready")));

    a.when(move |r| {
        let err = r.unwrap_err();
        assert_eq!(err.code(), E_FAIL);
        assert_eq!(err.message(), "IAsyncOperation-ready");
        assert_eq!(thread, thread_id());
    })?;

    let a = IAsyncOperation::<i32>::spawn(|| Err(Error::new(E_FAIL, "IAsyncOperation-spawn")));

    a.when(move |r| {
        let err = r.unwrap_err();
        assert_eq!(err.code(), E_FAIL);
        assert_eq!(err.message(), "IAsyncOperation-spawn");
        assert_ne!(thread, thread_id());
    })?;

    let a = IAsyncOperationWithProgress::<i32, i32>::ready(Err(Error::new(
        E_FAIL,
        "IAsyncOperationWithProgress-ready",
    )));

    a.when(move |r| {
        let err = r.unwrap_err();
        assert_eq!(err.code(), E_FAIL);
        assert_eq!(err.message(), "IAsyncOperationWithProgress-ready");
        assert_eq!(thread, thread_id());
    })?;

    let a = IAsyncOperationWithProgress::<i32, i32>::spawn(|| {
        Err(Error::new(E_FAIL, "IAsyncOperationWithProgress-spawn"))
    });

    a.when(move |r| {
        let err = r.unwrap_err();
        assert_eq!(err.code(), E_FAIL);
        assert_eq!(err.message(), "IAsyncOperationWithProgress-spawn");
        assert_ne!(thread, thread_id());
    })?;

    Ok(())
}

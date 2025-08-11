use windows::Win32::Foundation::E_FAIL;
use windows_future::*;
use windows_result::*;

#[test]
fn ok() -> Result<()> {
    let a = IAsyncAction::ready(Ok(()));

    a.when(move |r| {
        assert!(r.is_ok());
    })?;

    let a = IAsyncAction::spawn(|| Ok(()));

    a.when(move |r| {
        assert!(r.is_ok());
    })?;

    let a = IAsyncActionWithProgress::<i32>::ready(Ok(()));

    a.when(move |r| {
        assert!(r.is_ok());
    })?;

    let a = IAsyncActionWithProgress::<i32>::spawn(|| Ok(()));

    a.when(move |r| {
        assert!(r.is_ok());
    })?;

    let a = IAsyncOperation::<i32>::ready(Ok(123));

    a.when(move |r| {
        assert_eq!(r, Ok(123));
    })?;

    let a = IAsyncOperation::<i32>::spawn(|| Ok(456));

    a.when(move |r| {
        assert_eq!(r, Ok(456));
    })?;

    let a = IAsyncOperationWithProgress::<i32, i32>::ready(Ok(123));

    a.when(move |r| {
        assert_eq!(r, Ok(123));
    })?;

    let a = IAsyncOperationWithProgress::<i32, i32>::spawn(|| Ok(456));

    a.when(move |r| {
        assert_eq!(r, Ok(456));
    })?;

    Ok(())
}

#[test]
fn err() -> Result<()> {
    let a = IAsyncAction::ready(Err(Error::new(E_FAIL, "IAsyncAction-ready")));

    a.when(move |r| {
        let err = r.unwrap_err();
        assert_eq!(err.code(), E_FAIL);
        assert_eq!(err.message(), "IAsyncAction-ready");
    })?;

    let a = IAsyncAction::spawn(|| Err(Error::new(E_FAIL, "IAsyncAction-spawn")));

    a.when(move |r| {
        let err = r.unwrap_err();
        assert_eq!(err.code(), E_FAIL);
        assert_eq!(err.message(), "IAsyncAction-spawn");
    })?;

    let a = IAsyncActionWithProgress::<i32>::ready(Err(Error::new(
        E_FAIL,
        "IAsyncActionWithProgress-ready",
    )));

    a.when(move |r| {
        let err = r.unwrap_err();
        assert_eq!(err.code(), E_FAIL);
        assert_eq!(err.message(), "IAsyncActionWithProgress-ready");
    })?;

    let a = IAsyncActionWithProgress::<i32>::spawn(|| {
        Err(Error::new(E_FAIL, "IAsyncActionWithProgress-spawn"))
    });

    a.when(move |r| {
        let err = r.unwrap_err();
        assert_eq!(err.code(), E_FAIL);
        assert_eq!(err.message(), "IAsyncActionWithProgress-spawn");
    })?;

    let a = IAsyncOperation::<i32>::ready(Err(Error::new(E_FAIL, "IAsyncOperation-ready")));

    a.when(move |r| {
        let err = r.unwrap_err();
        assert_eq!(err.code(), E_FAIL);
        assert_eq!(err.message(), "IAsyncOperation-ready");
    })?;

    let a = IAsyncOperation::<i32>::spawn(|| Err(Error::new(E_FAIL, "IAsyncOperation-spawn")));

    a.when(move |r| {
        let err = r.unwrap_err();
        assert_eq!(err.code(), E_FAIL);
        assert_eq!(err.message(), "IAsyncOperation-spawn");
    })?;

    let a = IAsyncOperationWithProgress::<i32, i32>::ready(Err(Error::new(
        E_FAIL,
        "IAsyncOperationWithProgress-ready",
    )));

    a.when(move |r| {
        let err = r.unwrap_err();
        assert_eq!(err.code(), E_FAIL);
        assert_eq!(err.message(), "IAsyncOperationWithProgress-ready");
    })?;

    let a = IAsyncOperationWithProgress::<i32, i32>::spawn(|| {
        Err(Error::new(E_FAIL, "IAsyncOperationWithProgress-spawn"))
    });

    a.when(move |r| {
        let err = r.unwrap_err();
        assert_eq!(err.code(), E_FAIL);
        assert_eq!(err.message(), "IAsyncOperationWithProgress-spawn");
    })?;

    Ok(())
}

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

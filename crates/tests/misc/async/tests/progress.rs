// All stock implementations don't support progress notifications.

use windows::core::*;
use windows_async::*;

#[test]
fn test() -> Result<()> {
    let a = IAsyncActionWithProgress::<i32>::ready(Ok(()));
    a.get()?;
    a.SetProgress(&AsyncActionProgressHandler::new(|_, _| Ok(())))?;
    assert_eq!(a.Progress(), Err(Error::empty()));

    let a = IAsyncOperationWithProgress::<i32, i32>::ready(Ok(1));
    a.get()?;
    a.SetProgress(&AsyncOperationProgressHandler::new(|_, _| Ok(())))?;
    assert_eq!(a.Progress(), Err(Error::empty()));

    let a = IAsyncActionWithProgress::<i32>::spawn(|| Ok(()));
    a.get()?;
    a.SetProgress(&AsyncActionProgressHandler::new(|_, _| Ok(())))?;
    assert_eq!(a.Progress(), Err(Error::empty()));

    let a = IAsyncOperationWithProgress::<i32, i32>::spawn(|| Ok(123));
    a.get()?;
    a.SetProgress(&AsyncOperationProgressHandler::new(|_, _| Ok(())))?;
    assert_eq!(a.Progress(), Err(Error::empty()));

    Ok(())
}

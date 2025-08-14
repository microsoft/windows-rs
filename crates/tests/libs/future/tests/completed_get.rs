// All stock implementations of `Completed` return S_OK and a null pointer giving them impression
// that they do not store the `Completed` handler for consistency.

use windows::core::*;
use windows_future::*;

#[test]
fn test() -> Result<()> {
    let a = IAsyncAction::ready(Ok(()));
    a.join()?;
    assert_eq!(a.Completed(), Err(Error::empty()));

    let a = IAsyncActionWithProgress::<i32>::ready(Ok(()));
    a.join()?;
    assert_eq!(a.Completed(), Err(Error::empty()));

    let a = IAsyncOperation::<i32>::ready(Ok(1));
    a.join()?;
    assert_eq!(a.Completed(), Err(Error::empty()));

    let a = IAsyncOperationWithProgress::<i32, i32>::ready(Ok(1));
    a.join()?;
    assert_eq!(a.Completed(), Err(Error::empty()));

    let a = IAsyncAction::spawn(|| Ok(()));
    a.join()?;
    assert_eq!(a.Completed(), Err(Error::empty()));

    let a = IAsyncOperation::spawn(|| Ok(123));
    a.join()?;
    assert_eq!(a.Completed(), Err(Error::empty()));

    let a = IAsyncActionWithProgress::<i32>::spawn(|| Ok(()));
    a.join()?;
    assert_eq!(a.Completed(), Err(Error::empty()));

    let a = IAsyncOperationWithProgress::<i32, i32>::spawn(|| Ok(123));
    a.join()?;
    assert_eq!(a.Completed(), Err(Error::empty()));

    Ok(())
}

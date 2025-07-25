// All stock implementations don't support progress notifications.

use windows::core::*;
use windows_future::*;
const S_OK: HRESULT = HRESULT(0);

#[test]
fn test() -> Result<(), HRESULT> {
    let a = IAsyncActionWithProgress::<i32>::ready(Ok(()));
    a.get()?;
    a.SetProgress(&AsyncActionProgressHandler::new(|_, _| Ok(())))?;
    assert_eq!(a.Progress(), Err(S_OK));

    let a = IAsyncOperationWithProgress::<i32, i32>::ready(Ok(1));
    a.get()?;
    a.SetProgress(&AsyncOperationProgressHandler::new(|_, _| Ok(())))?;
    assert_eq!(a.Progress(), Err(S_OK));

    let a = IAsyncActionWithProgress::<i32>::spawn(|| Ok(()));
    a.get()?;
    a.SetProgress(&AsyncActionProgressHandler::new(|_, _| Ok(())))?;
    assert_eq!(a.Progress(), Err(S_OK));

    let a = IAsyncOperationWithProgress::<i32, i32>::spawn(|| Ok(123));
    a.get()?;
    a.SetProgress(&AsyncOperationProgressHandler::new(|_, _| Ok(())))?;
    assert_eq!(a.Progress(), Err(S_OK));

    Ok(())
}

// All stock implementations of `Completed` return S_OK and a null pointer giving them impression
// that they do not store the `Completed` handler for consistency.

use windows::core::*;
use windows_future::*;
const S_OK: HRESULT = HRESULT(0);

#[test]
fn test() -> Result<(), HRESULT> {
    let a = IAsyncAction::ready(Ok(()));
    a.get()?;
    assert_eq!(a.Completed(), Err(S_OK));

    let a = IAsyncActionWithProgress::<i32>::ready(Ok(()));
    a.get()?;
    assert_eq!(a.Completed(), Err(S_OK));

    let a = IAsyncOperation::<i32>::ready(Ok(1));
    a.get()?;
    assert_eq!(a.Completed(), Err(S_OK));

    let a = IAsyncOperationWithProgress::<i32, i32>::ready(Ok(1));
    a.get()?;
    assert_eq!(a.Completed(), Err(S_OK));

    let a = IAsyncAction::spawn(|| Ok(()));
    a.get()?;
    assert_eq!(a.Completed(), Err(S_OK));

    let a = IAsyncOperation::spawn(|| Ok(123));
    a.get()?;
    assert_eq!(a.Completed(), Err(S_OK));

    let a = IAsyncActionWithProgress::<i32>::spawn(|| Ok(()));
    a.get()?;
    assert_eq!(a.Completed(), Err(S_OK));

    let a = IAsyncOperationWithProgress::<i32, i32>::spawn(|| Ok(123));
    a.get()?;
    assert_eq!(a.Completed(), Err(S_OK));

    Ok(())
}

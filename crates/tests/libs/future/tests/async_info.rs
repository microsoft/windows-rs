// Tests `IAsyncInfo` for all stock implementations. The only one that really matters is `Status`
// but the remaining methods are tested just to confirm stable behavior.

use windows::core::*;
use windows_future::*;

#[test]
fn test() -> Result<()> {
    let a = IAsyncAction::ready(Ok(()));
    a.join()?;
    async_info(&a.cast()?)?;

    let a = IAsyncActionWithProgress::<i32>::ready(Ok(()));
    a.join()?;
    async_info(&a.cast()?)?;

    let a = IAsyncOperation::<i32>::ready(Ok(123));
    assert_eq!(a.join()?, 123);
    async_info(&a.cast()?)?;

    let a = IAsyncOperationWithProgress::<i32, i32>::ready(Ok(123));
    assert_eq!(a.join()?, 123);
    async_info(&a.cast()?)?;

    let a = IAsyncAction::spawn(|| Ok(()));
    a.join()?;
    async_info(&a.cast()?)?;

    let a = IAsyncOperation::spawn(|| Ok(123));
    assert_eq!(a.join()?, 123);
    async_info(&a.cast()?)?;

    let a = IAsyncActionWithProgress::<i32>::spawn(|| Ok(()));
    a.join()?;
    async_info(&a.cast()?)?;

    let a = IAsyncOperationWithProgress::<i32, i32>::spawn(|| Ok(123));
    assert_eq!(a.join()?, 123);
    async_info(&a.cast()?)?;

    Ok(())
}

fn async_info(info: &IAsyncInfo) -> Result<()> {
    // Stock implementations  return 1
    assert_eq!(info.Id()?, 1);

    // Different status codes are tested elsewhere.
    assert_eq!(info.Status()?, AsyncStatus::Completed);

    // Different error codes are tested elsewhere.
    assert_eq!(info.ErrorCode()?, HRESULT(0));

    // Stock implementations ignore `Cancel` and `Close` requests.
    info.Cancel()?;
    info.Close()?;

    Ok(())
}

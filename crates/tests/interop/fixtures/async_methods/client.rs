use super::*;
use AsyncMethods::*;
use windows_future::*;

pub fn test() -> Result<()> {
    let widget = Widget::new()?;

    widget.Action(1)?.join()?;
    let action = widget.ActionWithProgress(1)?;
    action.SetProgress(&AsyncActionProgressHandler::new(|_, _| Ok(())))?;
    action.join()?;
    assert_eq!(widget.Operation(1)?.join()?, 1);
    let operation = widget.OperationWithProgress(1)?;
    operation.SetProgress(&AsyncOperationProgressHandler::new(|_, _| Ok(())))?;
    assert_eq!(operation.join()?, 1);

    assert_eq!(widget.Action(-1)?.join().unwrap_err().code(), E_BOUNDS);
    assert_eq!(
        widget.ActionWithProgress(-1)?.join().unwrap_err().code(),
        E_BOUNDS
    );
    assert_eq!(widget.Operation(-1)?.join().unwrap_err().code(), E_BOUNDS);
    assert_eq!(
        widget.OperationWithProgress(-1)?.join().unwrap_err().code(),
        E_BOUNDS
    );

    widget
        .AwaitAction(&IAsyncAction::spawn(|| Ok(())))?
        .join()?;
    widget
        .AwaitActionWithProgress(&IAsyncActionWithProgress::spawn(|| Ok(())))?
        .join()?;
    assert_eq!(
        widget
            .AwaitAction(&IAsyncAction::spawn(|| Err(E_BOUNDS.into())))?
            .join()
            .unwrap_err()
            .code(),
        E_BOUNDS
    );
    assert_eq!(
        widget
            .AwaitActionWithProgress(&IAsyncActionWithProgress::spawn(|| Err(E_BOUNDS.into())))?
            .join()
            .unwrap_err()
            .code(),
        E_BOUNDS
    );
    assert_eq!(
        widget
            .AwaitOperation(&IAsyncOperation::spawn(|| Ok(41)))?
            .join()?,
        42
    );
    assert_eq!(
        widget
            .AwaitOperationWithProgress(&IAsyncOperationWithProgress::spawn(|| Ok(41)))?
            .join()?,
        42
    );

    Ok(())
}

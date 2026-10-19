use super::*;
use AsyncMethods::*;
use windows_future::*;

pub fn test() -> Result<()> {
    let widget = Widget::new()?;
    widget.Action()?.join()?;
    assert_eq!(widget.Operation(1)?.join()?, 1);
    assert_eq!(widget.StringOperation(h!("a"))?.join()?, "a");

    let error = widget.Fail()?.join().unwrap_err();
    assert_eq!(error.code(), E_BOUNDS);

    let operation = IAsyncOperation::spawn(|| Ok(41));
    assert_eq!(widget.Increment(&operation)?.join()?, 42);

    Ok(())
}

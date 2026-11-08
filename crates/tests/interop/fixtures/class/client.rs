use super::*;
use Class::*;

pub fn test() -> Result<()> {
    let widget = Widget::new()?;
    assert_eq!(widget.Add(1, 2)?, 3);
    assert!(widget.Name()?.is_empty());
    assert_eq!(widget.Count()?, 0);

    widget.SetName(h!("name"))?;
    assert_eq!(widget.Name()?, "name");
    assert_eq!(widget.Count()?, 1);

    let inspectable: IInspectable = widget.cast()?;
    assert_eq!(inspectable.GetRuntimeClassName()?, "Class.Widget");
    assert_eq!(inspectable.cast::<Widget>()?.Count()?, 1);

    Ok(())
}

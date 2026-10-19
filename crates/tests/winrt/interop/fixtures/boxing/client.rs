use super::*;
use Boxing::*;
use windows_reference::IReference;

pub fn test() -> Result<()> {
    let widget = Widget::new()?;

    let value: IInspectable = IReference::from(1).into();
    assert_eq!(widget.Describe(&value)?, "i32 1");
    let value: IInspectable = IReference::from("a").into();
    assert_eq!(widget.Describe(&value)?, "String a");
    assert_eq!(widget.Describe(None)?, "null");

    let value = widget.BoxInt32(2)?.cast::<IReference<i32>>()?;
    assert_eq!(value.Value()?, 2);
    let value = widget.BoxString(h!("b"))?.cast::<IReference<HSTRING>>()?;
    assert_eq!(value.Value()?, "b");

    assert!(widget.Optional().is_err());
    widget.SetOptional(Some(3))?;
    assert_eq!(widget.Optional()?, 3);
    widget.SetOptional(None)?;
    assert!(widget.Optional().is_err());

    Ok(())
}

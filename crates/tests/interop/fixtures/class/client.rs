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

    Ok(())
}

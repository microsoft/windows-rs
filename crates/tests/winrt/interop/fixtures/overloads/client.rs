use super::*;
use Overloads::*;

pub fn test() -> Result<()> {
    let widget = Widget::new()?;
    assert_eq!(widget.Sum()?, 0);
    assert_eq!(widget.Sum2(1)?, 1);
    assert_eq!(widget.Sum3(1, 2)?, 3);
    assert_eq!(widget.Sum4(1, 2, 3)?, 6);
    assert_eq!(widget.Format(1)?, "int 1");
    assert_eq!(widget.FormatText(h!("a"))?, "text a");

    Ok(())
}

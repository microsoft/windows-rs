use super::*;
use Constructors::*;

pub fn test() -> Result<()> {
    assert_eq!(Widget::new()?.Value()?, 0);
    assert_eq!(Widget::WithValue(123)?.Value()?, 123);

    assert_eq!(Base::new()?.Value()?, 0);
    assert_eq!(Base::WithValue(456)?.Value()?, 456);

    Ok(())
}

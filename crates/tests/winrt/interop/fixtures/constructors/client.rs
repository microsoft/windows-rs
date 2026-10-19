use super::*;
use Constructors::*;

pub fn test() -> Result<()> {
    assert_eq!(Widget::new()?.Value()?, 0);
    assert_eq!(Widget::WithValue(123)?.Value()?, 123);

    assert_eq!(Base::new()?.Value()?, 0);
    assert_eq!(Base::WithValue(456)?.Value()?, 456);

    // Each constructor releases the inner object that the factory returns.
    let base = Base::new()?;
    assert_eq!(Base::Instances()?, 1);
    drop(base);
    assert_eq!(Base::Instances()?, 0);

    Ok(())
}

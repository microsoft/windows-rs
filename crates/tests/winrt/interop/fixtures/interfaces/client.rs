use super::*;
use Interfaces::*;

pub fn test() -> Result<()> {
    let widget = Widget::new()?;
    assert_eq!(widget.Area()?, 1);

    let shape: IShape = widget.cast()?;
    assert_eq!(shape.Area()?, 1);
    assert!(same(&widget.Echo(&shape)?, &widget)?);

    let shape: IShape = Shape.into();
    assert!(same(&widget.Echo(&shape)?, &shape)?);

    Ok(())
}

// Compares COM identity, which is the `IUnknown` pointer.
fn same(left: &impl Interface, right: &impl Interface) -> Result<bool> {
    Ok(left.cast::<IUnknown>()? == right.cast::<IUnknown>()?)
}

#[implement(IShape)]
struct Shape;

impl IShape_Impl for Shape_Impl {
    fn Area(&self) -> Result<i32> {
        Ok(2)
    }
}

use super::*;
use Interfaces::*;

pub fn factory(name: &str) -> Option<IActivationFactory> {
    match name {
        "Interfaces.Widget" => Some(activatable::<Widget, Interfaces::Widget>()),
        "Interfaces.Square" => Some(activatable::<Square, Interfaces::Square>()),
        _ => None,
    }
}

#[implement(Interfaces::Widget, IShape)]
#[derive(Default)]
struct Widget;

impl IWidget_Impl for Widget_Impl {
    fn Echo(&self, shape: Ref<IShape>) -> Result<IShape> {
        shape.ok().cloned()
    }
}

impl IShape_Impl for Widget_Impl {
    fn Area(&self) -> Result<i32> {
        Ok(1)
    }
}

#[implement(Interfaces::Square)]
#[derive(Default)]
struct Square;

impl IShape_Impl for Square_Impl {
    fn Area(&self) -> Result<i32> {
        Ok(4)
    }
}

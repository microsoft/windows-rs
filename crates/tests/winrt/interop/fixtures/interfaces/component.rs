use super::*;
use Interfaces::*;

pub fn factory(name: &str) -> Option<IActivationFactory> {
    (name == "Interfaces.Widget").then(activatable::<Widget, Interfaces::Widget>)
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

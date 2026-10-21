use super::*;
use ErrorRelay::*;

pub fn factory(name: &str) -> Option<IActivationFactory> {
    (name == "ErrorRelay.Widget").then(activatable::<Widget, ErrorRelay::Widget>)
}

#[implement(ErrorRelay::Widget)]
#[derive(Default)]
struct Widget;

impl IWidget_Impl for Widget_Impl {
    fn Relay(&self, callback: Ref<ICallback>, code: i32, message: &HSTRING) -> Result<()> {
        callback.ok()?.Fail(code, message)
    }
}

use super::*;
use EventDelegate::*;

pub fn factory(name: &str) -> Option<IActivationFactory> {
    (name == "EventDelegate.Widget").then(activatable::<Widget, EventDelegate::Widget>)
}

#[implement(EventDelegate::Widget)]
#[derive(Default)]
struct Widget {
    changed: Event<ChangedHandler>,
}

impl IWidget_Impl for Widget_Impl {
    fn Changed(&self, handler: Ref<ChangedHandler>) -> Result<i64> {
        self.changed.add(handler.ok()?)
    }

    fn RemoveChanged(&self, token: i64) -> Result<()> {
        self.changed.remove(token);
        Ok(())
    }

    fn Raise(&self, value: i32) -> Result<()> {
        let sender: EventDelegate::Widget = self.to_interface();
        self.changed.call(|handler| handler.Invoke(&sender, value));
        Ok(())
    }
}

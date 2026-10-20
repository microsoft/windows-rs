use super::*;
use EventGeneric::*;
use bindings::Windows::Foundation::TypedEventHandler;

pub fn factory(name: &str) -> Option<IActivationFactory> {
    (name == "EventGeneric.Widget").then(activatable::<Widget, EventGeneric::Widget>)
}

#[implement(EventGeneric::Widget)]
#[derive(Default)]
struct Widget {
    changed: Event<TypedEventHandler<EventGeneric::Widget, i32>>,
}

impl IWidget_Impl for Widget_Impl {
    fn Changed(&self, handler: Ref<TypedEventHandler<EventGeneric::Widget, i32>>) -> Result<i64> {
        self.changed.add(handler.ok()?)
    }

    fn RemoveChanged(&self, token: i64) -> Result<()> {
        self.changed.remove(token);
        Ok(())
    }

    fn Raise(&self, value: i32) -> Result<()> {
        let sender: EventGeneric::Widget = self.to_interface();
        self.changed.call(|handler| handler.Invoke(&sender, value));
        Ok(())
    }
}

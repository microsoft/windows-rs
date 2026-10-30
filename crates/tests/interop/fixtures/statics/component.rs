use super::*;
use Statics::*;
use bindings::Windows::Foundation::EventHandler;
use std::sync::atomic::{AtomicI32, Ordering};

pub fn factory(name: &str) -> Option<IActivationFactory> {
    (name == "Statics.Widget").then(|| Widget.into())
}

static VALUE: AtomicI32 = AtomicI32::new(0);
static CHANGED: Event<EventHandler<i32>> = Event::new();

/// A static class's factory implements the statics interface but cannot create instances.
#[implement(IActivationFactory, IWidgetStatics)]
struct Widget;

impl IActivationFactory_Impl for Widget_Impl {
    fn ActivateInstance(&self) -> Result<IInspectable> {
        Err(E_NOTIMPL.into())
    }
}

impl IWidgetStatics_Impl for Widget_Impl {
    fn Add(&self, a: i32, b: i32) -> Result<i32> {
        Ok(a + b)
    }

    fn Value(&self) -> Result<i32> {
        Ok(VALUE.load(Ordering::Relaxed))
    }

    fn SetValue(&self, value: i32) -> Result<()> {
        VALUE.store(value, Ordering::Relaxed);
        CHANGED.call(|handler| handler.Invoke(None, value));
        Ok(())
    }

    fn Changed(&self, handler: Ref<EventHandler<i32>>) -> Result<i64> {
        CHANGED.add(handler.ok()?)
    }

    fn RemoveChanged(&self, token: i64) -> Result<()> {
        CHANGED.remove(token);
        Ok(())
    }
}

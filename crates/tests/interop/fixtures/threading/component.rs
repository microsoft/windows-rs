use super::*;
use Threading::*;
use std::sync::atomic::{AtomicI32, Ordering};
use windows_future::*;

pub fn factory(name: &str) -> Option<IActivationFactory> {
    (name == "Threading.Widget").then(activatable::<Widget, Threading::Widget>)
}

#[implement(Threading::Widget)]
#[derive(Default)]
struct Widget {
    total: AtomicI32,
}

impl IWidget_Impl for Widget_Impl {
    fn CurrentThread(&self) -> Result<u32> {
        Ok(unsafe { GetCurrentThreadId() })
    }

    fn Increment(&self) -> Result<i32> {
        Ok(self.total.fetch_add(1, Ordering::Relaxed) + 1)
    }

    fn Count(&self) -> Result<i32> {
        Ok(self.total.load(Ordering::Relaxed))
    }

    fn InvokeOnBackground(&self, handler: Ref<Handler>, value: i32) -> Result<IAsyncAction> {
        let handler = handler.ok()?;
        check(handler.cast::<IAgileObject>().is_ok())?;
        let handler = AgileReference::new(handler)?;
        Ok(IAsyncAction::spawn(move || {
            handler.resolve()?.Invoke(value)
        }))
    }
}

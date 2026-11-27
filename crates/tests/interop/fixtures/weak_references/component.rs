use super::*;
use WeakReferences::*;
use std::sync::Mutex;

pub fn factory(name: &str) -> Option<IActivationFactory> {
    (name == "WeakReferences.Widget").then(activatable::<Widget, WeakReferences::Widget>)
}

#[implement(WeakReferences::Widget)]
#[derive(Default)]
struct Widget(Mutex<Option<AgileReference<IWeakReference>>>);

impl IWidget_Impl for Widget_Impl {
    fn Observe(&self, observer: Ref<IObserver>) -> Result<()> {
        let source = observer.ok()?.cast::<IWeakReferenceSource>()?;
        let weak = unsafe { source.GetWeakReference()? };
        let weak = AgileReference::new(&weak)?;
        let previous = self.0.lock().unwrap().replace(weak);
        drop(previous);
        Ok(())
    }

    fn Notify(&self, value: i32) -> Result<bool> {
        let weak = self.0.lock().unwrap().clone();
        let Some(weak) = weak else {
            return Ok(false);
        };
        let weak = weak.resolve()?;
        let mut observer = None;
        unsafe {
            weak.Resolve(&IInspectable::IID, &mut observer).ok()?;
        }

        match observer {
            Some(observer) => {
                observer.cast::<IObserver>()?.Notify(value)?;
                Ok(true)
            }
            None => Ok(false),
        }
    }
}

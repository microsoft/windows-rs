use super::*;
use WeakReferences::*;
use std::sync::Mutex;

pub fn factory(name: &str) -> Option<IActivationFactory> {
    (name == "WeakReferences.Widget").then(activatable::<Widget, WeakReferences::Widget>)
}

#[implement(WeakReferences::Widget)]
#[derive(Default)]
struct Widget(Mutex<Weak<IObserver>>);

impl IWidget_Impl for Widget_Impl {
    fn Observe(&self, observer: Ref<IObserver>) -> Result<()> {
        *self.0.lock().unwrap() = observer.ok()?.downgrade()?;
        Ok(())
    }

    fn Notify(&self, value: i32) -> Result<bool> {
        let observer = self.0.lock().unwrap().upgrade();

        match observer {
            Some(observer) => {
                observer.Notify(value)?;
                Ok(true)
            }
            None => Ok(false),
        }
    }
}

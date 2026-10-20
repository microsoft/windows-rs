use super::*;
use Callback::*;
use std::sync::atomic::{AtomicI32, Ordering};

pub fn factory(name: &str) -> Option<IActivationFactory> {
    (name == "Callback.Widget").then(activatable::<Widget, Callback::Widget>)
}

#[implement(Callback::Widget)]
#[derive(Default)]
struct Widget;

impl IWidget_Impl for Widget_Impl {
    fn Check(&self, value: Ref<IValue>) -> Result<()> {
        let value = value.ok()?;

        check(value.Current()? == 0)?;
        value.SetCurrent(-321)?;
        check(value.Current()? == -321)?;

        // The client's object calls the component's object, and null arrives as null.
        let other: IValue = Value(AtomicI32::new(123)).into();
        check(value.Add(&other)? == -198)?;
        check(value.Add(None).map_err(|e| e.code()) == Err(E_POINTER))?;

        // The client returns a new object of its own through an out parameter.
        let mut created = None;
        value.Create(789, &mut created)?;
        let created = created.ok_or_else(Error::empty)?;
        check(created != *value)?;
        check(created.Current()? == 789)?;
        check(created.Add(&other)? == 912)?;

        Ok(())
    }
}

#[implement(IValue)]
struct Value(AtomicI32);

impl IValue_Impl for Value_Impl {
    fn Current(&self) -> Result<i32> {
        Ok(self.0.load(Ordering::Relaxed))
    }

    fn SetCurrent(&self, current: i32) -> Result<()> {
        self.0.store(current, Ordering::Relaxed);
        Ok(())
    }

    fn Add(&self, other: Ref<IValue>) -> Result<i32> {
        Ok(self.0.load(Ordering::Relaxed) + other.ok()?.Current()?)
    }

    fn Create(&self, current: i32, result: OutRef<IValue>) -> Result<()> {
        result.write(Some(Value(AtomicI32::new(current)).into()))
    }
}

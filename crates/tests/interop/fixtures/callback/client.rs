use super::*;
use Callback::*;
use std::sync::atomic::{AtomicI32, Ordering};

pub fn test() -> Result<()> {
    Widget::new()?.Check(&IValue::from(Value(AtomicI32::new(0))))
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

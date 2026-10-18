use super::*;

pub fn factory(name: &str) -> Option<IActivationFactory> {
    (name == "Class.Widget").then(activatable::<Widget, Class::Widget>)
}

#[implement(Class::Widget)]
#[derive(Default)]
struct Widget {
    name: std::sync::RwLock<HSTRING>,
    sets: std::sync::atomic::AtomicI32,
}

impl Class::IWidget_Impl for Widget_Impl {
    fn Add(&self, left: i32, right: i32) -> Result<i32> {
        Ok(left + right)
    }

    fn Name(&self) -> Result<HSTRING> {
        Ok(self.name.read().unwrap().clone())
    }

    fn SetName(&self, value: &HSTRING) -> Result<()> {
        *self.name.write().unwrap() = value.clone();
        self.sets.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
        Ok(())
    }

    fn Count(&self) -> Result<i32> {
        Ok(self.sets.load(std::sync::atomic::Ordering::Relaxed))
    }
}

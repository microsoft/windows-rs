use super::*;
use Boxing::*;
use std::sync::Mutex;
use windows_reference::IReference;

pub fn factory(name: &str) -> Option<IActivationFactory> {
    (name == "Boxing.Widget").then(activatable::<Widget, Boxing::Widget>)
}

#[implement(Boxing::Widget)]
#[derive(Default)]
struct Widget(Mutex<Option<IReference<i32>>>);

impl IWidget_Impl for Widget_Impl {
    fn Describe(&self, value: Ref<IInspectable>) -> Result<HSTRING> {
        let Some(value) = value.as_ref() else {
            return Ok("null".into());
        };

        if let Ok(value) = value.cast::<IReference<i32>>() {
            Ok(format!("i32 {}", value.Value()?).into())
        } else {
            let value = value.cast::<IReference<HSTRING>>()?.Value()?;
            Ok(format!("String {}", value.to_string_lossy()).into())
        }
    }

    fn BoxInt32(&self, value: i32) -> Result<IInspectable> {
        Ok(IReference::from(value).into())
    }

    fn BoxString(&self, value: &HSTRING) -> Result<IInspectable> {
        Ok(IReference::from(value).into())
    }

    fn Optional(&self) -> Result<IReference<i32>> {
        self.0.lock().unwrap().clone().ok_or_else(Error::empty)
    }

    fn SetOptional(&self, value: Ref<IReference<i32>>) -> Result<()> {
        *self.0.lock().unwrap() = value.cloned();
        Ok(())
    }
}

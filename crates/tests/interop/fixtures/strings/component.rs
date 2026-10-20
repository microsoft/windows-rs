use super::*;
use Strings::*;
use std::sync::RwLock;

pub fn factory(name: &str) -> Option<IActivationFactory> {
    (name == "Strings.Widget").then(activatable::<Widget, Strings::Widget>)
}

#[implement(Strings::Widget)]
#[derive(Default)]
struct Widget {
    text: RwLock<HSTRING>,
}

impl IWidget_Impl for Widget_Impl {
    fn Length(&self, value: &HSTRING) -> Result<u32> {
        Ok(value.len() as u32)
    }

    fn Echo(&self, value: &HSTRING) -> Result<HSTRING> {
        Ok(value.clone())
    }

    fn Repeat(&self, value: &HSTRING, count: u32) -> Result<HSTRING> {
        Ok(HSTRING::from_wide(&value.repeat(count as usize)))
    }

    fn Greeting(&self) -> Result<HSTRING> {
        Ok("héllo 😀".into())
    }

    fn Text(&self) -> Result<HSTRING> {
        Ok(self.text.read().unwrap().clone())
    }

    fn SetText(&self, value: &HSTRING) -> Result<()> {
        *self.text.write().unwrap() = value.clone();
        Ok(())
    }
}

use super::*;
use Noexcept::*;
use std::sync::Mutex;

pub fn factory(name: &str) -> Option<IActivationFactory> {
    (name == "Noexcept.Widget").then(activatable::<Widget, Noexcept::Widget>)
}

#[implement(Noexcept::Widget, IValue)]
#[derive(Default)]
struct Widget(Mutex<(i32, HSTRING, Option<IValue>, i32)>);

impl IWidget_Impl for Widget_Impl {
    fn Check(&self, value: Ref<IValue>) -> Result<()> {
        let value = value.ok()?;

        check(value.Number() == 0)?;
        value.SetNumber(123);
        check(value.Number() == 123)?;

        check(value.Text().is_empty())?;
        value.SetText(h!("abc"));
        check(value.Text() == "abc")?;

        check(value.Next().is_none())?;
        value.SetNext(value);
        check(value.Next().as_ref() == Some(value))?;

        check(value.Limit() == 0)?;
        value.SetLimit(5)?;
        check(value.Limit() == 5)?;
        check(
            value
                .SetLimit(-1)
                .is_err_and(|error| error.code() == E_INVALIDARG),
        )?;
        check(value.Limit() == 5)?;

        value.Clear();
        check(value.Number() == 0)?;
        check(value.Text().is_empty())?;
        check(value.Next().is_none())?;
        check(value.Limit() == 0)?;

        Ok(())
    }
}

impl IValue_Impl for Widget_Impl {
    fn Number(&self) -> i32 {
        self.0.lock().unwrap().0
    }

    fn SetNumber(&self, value: i32) {
        self.0.lock().unwrap().0 = value;
    }

    fn Text(&self) -> HSTRING {
        self.0.lock().unwrap().1.clone()
    }

    fn SetText(&self, value: &HSTRING) {
        self.0.lock().unwrap().1 = value.clone();
    }

    fn Next(&self) -> Option<IValue> {
        self.0.lock().unwrap().2.clone()
    }

    fn SetNext(&self, value: Ref<IValue>) {
        self.0.lock().unwrap().2 = value.cloned();
    }

    fn Limit(&self) -> i32 {
        self.0.lock().unwrap().3
    }

    fn SetLimit(&self, value: i32) -> Result<()> {
        if value < 0 {
            return Err(E_INVALIDARG.into());
        }

        self.0.lock().unwrap().3 = value;
        Ok(())
    }

    fn Clear(&self) {
        *self.0.lock().unwrap() = Default::default();
    }
}

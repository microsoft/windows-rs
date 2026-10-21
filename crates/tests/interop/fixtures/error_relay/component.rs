use super::*;
use ErrorRelay::*;

pub fn factory(name: &str) -> Option<IActivationFactory> {
    (name == "ErrorRelay.Widget").then(activatable::<Widget, ErrorRelay::Widget>)
}

#[implement(ErrorRelay::Widget)]
#[derive(Default)]
struct Widget;

impl IWidget_Impl for Widget_Impl {
    fn Relay(&self, callback: Ref<ICallback>, code: i32, message: &HSTRING) -> Result<()> {
        callback.ok()?.Fail(code, message)
    }

    fn RelayPreserved(&self, callback: Ref<ICallback>, code: i32, message: &HSTRING) -> Result<()> {
        let callback = callback.ok()?;
        let original = callback
            .Fail(code, message)
            .err()
            .ok_or_else(|| Error::new(E_FAIL, "expected the original callback failure"))?;
        let intervening = callback
            .Fail(E_INVALIDARG.0, h!("intervening"))
            .err()
            .ok_or_else(|| Error::new(E_FAIL, "expected the intervening callback failure"))?;
        check(intervening.code() == E_INVALIDARG)?;
        check(intervening.message() == "intervening")?;
        callback.Checkpoint()?;
        Err(original)
    }
}

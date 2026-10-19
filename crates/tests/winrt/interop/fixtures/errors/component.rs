use super::*;
use Errors::*;

pub fn factory(name: &str) -> Option<IActivationFactory> {
    (name == "Errors.Widget").then(activatable::<Widget, Errors::Widget>)
}

#[implement(Errors::Widget)]
#[derive(Default)]
struct Widget;

impl IWidget_Impl for Widget_Impl {
    fn Fail(&self) -> Result<()> {
        Err(E_BOUNDS.into())
    }

    fn FailWithMessage(&self, message: &HSTRING) -> Result<()> {
        Err(Error::new(E_BOUNDS, message.to_string_lossy()))
    }

    fn Check(&self, callback: Ref<ICallback>) -> Result<()> {
        let error = callback.ok()?.Fail().unwrap_err();
        check(error.code() == E_BOUNDS)?;
        check(error.message() == "client")
    }
}

use super::*;
use Errors::*;

pub fn test() -> Result<()> {
    let widget = Widget::new()?;

    let error = widget.Fail().unwrap_err();
    assert_eq!(error.code(), E_BOUNDS);

    let error = widget.FailWithMessage(h!("component")).unwrap_err();
    assert_eq!(error.code(), E_BOUNDS);
    assert_eq!(error.message(), "component");

    widget.Check(&ICallback::from(Callback))
}

#[implement(ICallback)]
struct Callback;

impl ICallback_Impl for Callback_Impl {
    fn Fail(&self) -> Result<()> {
        Err(Error::new(E_BOUNDS, "client"))
    }
}

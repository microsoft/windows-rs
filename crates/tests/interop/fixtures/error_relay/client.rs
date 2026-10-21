use super::*;
use ErrorRelay::*;

pub fn test() -> Result<()> {
    let widget = Widget::new()?;
    let callback: ICallback = Callback.into();
    for (code, message) in [
        (E_BOUNDS, "relay-bounds"),
        (HRESULT(0x8004_a123_u32 as i32), "relay-custom"),
    ] {
        let error = widget
            .Relay(&callback, code.0, &HSTRING::from(message))
            .unwrap_err();
        assert_eq!(error.code(), code);
        assert_eq!(error.message(), message);
    }
    Ok(())
}

#[implement(ICallback)]
struct Callback;

impl ICallback_Impl for Callback_Impl {
    fn Fail(&self, code: i32, message: &HSTRING) -> Result<()> {
        Err(Error::new(HRESULT(code), message.to_string_lossy()))
    }
}

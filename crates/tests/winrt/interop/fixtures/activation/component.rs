use super::*;
use Activation::*;

pub fn factory(name: &str) -> Option<IActivationFactory> {
    match name {
        "Activation.Nested.Widget" => Some(activatable::<Widget, Nested::Widget>()),
        "Activation.Settings" => Some(Settings.into()),
        _ => None,
    }
}

#[implement(Nested::Widget)]
#[derive(Default)]
struct Widget;

impl Nested::IWidget_Impl for Widget_Impl {
    fn Value(&self) -> Result<i32> {
        Ok(123)
    }
}

/// A static class's factory implements the statics interface but cannot create instances.
#[implement(IActivationFactory, ISettingsStatics)]
struct Settings;

impl IActivationFactory_Impl for Settings_Impl {
    fn ActivateInstance(&self) -> Result<IInspectable> {
        Err(E_NOTIMPL.into())
    }
}

impl ISettingsStatics_Impl for Settings_Impl {
    fn Version(&self) -> Result<i32> {
        Ok(456)
    }
}

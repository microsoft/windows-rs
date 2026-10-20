use super::*;
use Activation::*;

pub fn test() -> Result<()> {
    let widget = Nested::Widget::new()?;
    assert_eq!(widget.Value()?, 123);

    assert_eq!(Settings::Version()?, 456);

    assert_eq!(Missing::new().unwrap_err().code(), REGDB_E_CLASSNOTREG);

    factory::<Nested::Widget, IActivationFactory>()?;
    let error = factory::<Nested::Widget, Nested::IWidget>().unwrap_err();
    assert_eq!(error.code(), E_NOINTERFACE);
    let error = factory::<Missing, IActivationFactory>().unwrap_err();
    assert_eq!(error.code(), REGDB_E_CLASSNOTREG);

    Ok(())
}

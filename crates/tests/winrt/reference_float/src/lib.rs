mod bindings;
use bindings::*;
use windows::{core::*, Foundation::*};

#[test]
fn test() -> Result<()> {
    let mut container = RefWithFloat::default();
    container.ReferenceFloat = Some(PropertyValue::CreateSingle(1.23)?.cast()?);

    // TODO: https://github.com/microsoft/windows-rs/issues/292 is needed to provide a stock implementation
    // of `IReference` for arbitrary types in order to test `container.ReferenceWithFloat`.

    Ok(())
}

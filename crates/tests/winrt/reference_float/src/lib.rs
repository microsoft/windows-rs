#![cfg(test)]

mod bindings;
use bindings::*;
use windows::{core::*, Foundation::*};

#[test]
fn test() -> Result<()> {
    let mut container = RefWithFloat::default();
    container.Value = Some(PropertyValue::CreateSingle(1.23)?.cast()?);
    assert_eq!(container.Value.unwrap().Value()?, 1.23);

    Ok(())
}

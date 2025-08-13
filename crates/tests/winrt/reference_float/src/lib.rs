#![cfg(test)]

mod bindings;
use bindings::*;
use windows::{core::*, Foundation::*};

#[test]
#[expect(clippy::field_reassign_with_default)] // testing unusual field assignment
fn test() -> Result<()> {
    let mut container = RefWithFloat::default();
    container.Value = Some(PropertyValue::CreateSingle(1.23)?.cast()?);
    assert_eq!(container.Value.unwrap().Value()?, 1.23);

    Ok(())
}

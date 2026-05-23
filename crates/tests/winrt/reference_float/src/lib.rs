#![cfg(all(test, windows))]

#[allow(
    non_snake_case,
    non_upper_case_globals,
    non_camel_case_types,
    dead_code,
    clippy::upper_case_acronyms,
    clippy::missing_transmute_annotations
)]
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

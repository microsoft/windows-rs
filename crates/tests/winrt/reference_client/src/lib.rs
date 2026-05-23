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
use windows::core::*;

#[test]
fn test() -> Result<()> {
    let reference = Reference::new()?;
    assert_eq!(reference.ToString()?, "Reference");
    assert_eq!(reference.Method(&reference)?, "Reference");
    Ok(())
}

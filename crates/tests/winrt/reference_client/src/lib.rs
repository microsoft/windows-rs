#![cfg(test)]

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

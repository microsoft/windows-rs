#![cfg(test)]

mod bindings;
use bindings::*;
use windows::core::*;

#[test]
fn test() -> Result<()> {
    let class = Class::new()?;
    class.SetProperty(123)?;
    assert_eq!(class.Property()?, 123);
    Ok(())
}

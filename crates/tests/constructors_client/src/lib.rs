#![cfg(test)]

mod bindings;
use bindings::*;
use windows::core::*;

#[test]
fn test() -> Result<()> {
    let activatable = Activatable::CreateInstance(123)?;
    assert_eq!(activatable.Property()?, 123);

    let composable = Composable::CreateInstance(456)?;
    assert_eq!(composable.Property()?, 456);

    Ok(())
}

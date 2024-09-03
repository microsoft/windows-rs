#![cfg(test)]

mod bindings;
use bindings::*;
use windows::core::*;

#[test]
fn test() -> Result<()> {
    let activatable = Activatable::new()?;
    assert_eq!(activatable.Property()?, 0);

    let activatable = Activatable::WithValue(123)?;
    assert_eq!(activatable.Property()?, 123);

    let composable = Composable::new()?;
    assert_eq!(composable.Property()?, 0);

    let composable = Composable::WithValue(456)?;
    assert_eq!(composable.Property()?, 456);

    Ok(())
}

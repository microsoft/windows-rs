#![cfg(test)]

mod bindings {
    pub mod microsoft_windows_system_power;
    pub mod test_nightly_component;
}

use bindings::microsoft_windows_system_power::*;
use bindings::test_nightly_component::*;
use windows::core::*;

#[test]
fn test_heuristics() -> Result<()> {
    let class = Class::new()?;
    class.SetProperty(123)?;
    assert_eq!(class.Property()?, 123);
    Ok(())
}

#[test]
fn test_explicit() -> Result<()> {
    let class = PowerManager::new()?;
    class.SetProperty(456)?;
    assert_eq!(class.Property()?, 456);
    Ok(())
}

#![allow(non_snake_case, non_upper_case_globals, non_camel_case_types, clashing_extern_declarations, unused_variables, dead_code)]
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

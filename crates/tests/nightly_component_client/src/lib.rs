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

    let inspectable: IInspectable = class.cast()?;
    assert_eq!(inspectable.GetRuntimeClassName()?, "test_nightly_component.Class");

    let inspectable: IInspectable = class.into();
    assert_eq!(inspectable.GetRuntimeClassName()?, "test_nightly_component.IClass");
    Ok(())
}

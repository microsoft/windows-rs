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

    // This explicitly queries for IInspectable.
    let inspectable: IInspectable = class.cast()?;
    // Notice GetRuntimeClassName returns the class name. 
    assert_eq!(inspectable.GetRuntimeClassName()?, "test_nightly_component.Class");

    // This just casts down to IInspectable since the vtable already includes IInspectable.
    let inspectable: IInspectable = class.into();
    // Notice GetRuntimeClassName returns the specific interface name.
    assert_eq!(inspectable.GetRuntimeClassName()?, "test_nightly_component.IClass");
    Ok(())
}

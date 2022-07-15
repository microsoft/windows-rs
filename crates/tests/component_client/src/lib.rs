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

    // Blittable array parameter passing.
    let a = [1, 2, 3];
    let mut b = [0; 3];
    let mut c = Array::new();
    let d = class.Int32Array(&a, &mut b, &mut c)?;
    assert_eq!(a, b);
    assert_eq!(a, c[..]);
    assert_eq!(a, d[..]);

    // Non-blittable array parameter passing.
    let a = [HSTRING::from("a"), HSTRING::from("b"), HSTRING::from("c")];
    let mut b = [HSTRING::new(), HSTRING::new(), HSTRING::new()];
    let mut c = Array::new();
    let d = class.StringArray(&a, &mut b, &mut c)?;
    assert_eq!(a, b);
    assert_eq!(a, c[..]);
    assert_eq!(a, d[..]);

    // This explicitly queries for IInspectable.
    let inspectable: IInspectable = class.cast()?;
    // Notice GetRuntimeClassName returns the class name.
    assert_eq!(inspectable.GetRuntimeClassName()?, "test_component.Class");

    // This just casts down to IInspectable since the vtable already includes IInspectable.
    let inspectable: IInspectable = class.into();
    // Notice GetRuntimeClassName returns the specific interface name.
    assert_eq!(inspectable.GetRuntimeClassName()?, "test_component.IClass");

    Ok(())
}

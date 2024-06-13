#![cfg(test)]

mod bindings;
use bindings::*;
use windows::{core::*, Foundation::*};

#[implement(IStringable)]
struct Stringable;

impl IStringable_Impl for Stringable_Impl {
    fn ToString(&self) -> Result<HSTRING> {
        Ok("client".into())
    }
}

#[test]
fn test() -> Result<()> {
    let class = Class::new()?;
    class.SetProperty(123)?;
    assert_eq!(class.Property()?, 123);
    assert_eq!(class.Flags()?, Flags::Ok);

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

    let c: IStringable = Stringable.into();
    let d = Callback::new(Ok);
    class.Input(&class, &class, &c, &d)?;
    assert!(class.Input(None, None, None, None).is_err());

    // This explicitly queries for IInspectable.
    let inspectable: IInspectable = class.cast()?;
    // Notice GetRuntimeClassName returns the class name.
    assert_eq!(inspectable.GetRuntimeClassName()?, "test_component.Class");

    // This just casts down to IInspectable since the vtable already includes IInspectable.
    let inspectable: &IInspectable = unsafe { std::mem::transmute(&class) };
    // Notice GetRuntimeClassName returns the specific interface name.
    assert_eq!(inspectable.GetRuntimeClassName()?, "test_component.IClass");

    Ok(())
}

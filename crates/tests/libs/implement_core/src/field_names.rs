//! User fields that `#[implement]` must not shadow on the generated `_Impl` type.

use windows_core::*;

#[interface("8c6f4b2a-0f3e-4d8e-9b1a-1c2d3e4f5a6b")]
unsafe trait IFields: IUnknown {
    fn sum(&self) -> i32;
}

#[implement(IFields)]
struct Fields {
    base: i32,
    identity: i32,
    interface1_ifields: i32,
    this: i32,
    count: i32,
}

impl IFields_Impl for Fields_Impl {
    unsafe fn sum(&self) -> i32 {
        self.base + self.identity + self.interface1_ifields + self.this + self.count
    }
}

#[test]
fn user_fields_not_shadowed() {
    let fields: IFields = Fields {
        base: 1,
        identity: 2,
        interface1_ifields: 4,
        this: 8,
        count: 16,
    }
    .into();

    assert_eq!(unsafe { fields.sum() }, 31);
}

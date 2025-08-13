#![expect(non_snake_case, non_camel_case_types)]

use windows::core::*;

// The `interface` macro defines a new local interface that does not derive from `IUnknown` and thus is not a COM interface at all.
#[interface]
unsafe trait IBase {
    unsafe fn BaseValue(&self) -> i32;
}

struct Base(i32);

impl IBase_Impl for Base {
    unsafe fn BaseValue(&self) -> i32 {
        self.0
    }
}

unsafe fn base_value(test: &IBase) -> i32 {
    unsafe { test.BaseValue() }
}

#[test]
fn base() {
    unsafe {
        // Since the interface is not rooted in `IUnknown`, there's no COM-style lifetime and the resulting implementation merely
        // exists for the lifetime of the referenced implementation.
        let test = Base(456);
        let interface = IBase::new(&test);
        assert_eq!(base_value(&interface), 456);
        assert_eq!(interface.BaseValue(), 456);
    }
}

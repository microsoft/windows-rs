#![cfg(windows)]
#![expect(non_snake_case, non_camel_case_types)]

use windows::core::*;

// This local interface does not derive from `IUnknown`.
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
        // Without `IUnknown`, the interface pointer has no COM lifetime and merely
        // exists for the lifetime of the referenced implementation.
        let test = Base(456);
        let interface = IBase::new(&test);
        assert_eq!(base_value(&interface), 456);
        assert_eq!(interface.BaseValue(), 456);
    }
}

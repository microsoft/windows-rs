#![cfg(windows)]
#![expect(non_snake_case)]

use windows::core::*;

// Two declarations of one interface stand in for separate bindings of the same COM interface.
mod first {
    use windows::core::*;

    #[interface("6f1f3a6e-4b8c-4d0a-9a43-1c4c2e6f0b11")]
    pub unsafe trait IValue: IUnknown {
        pub unsafe fn Value(&self) -> u32;
    }
}

mod second {
    use windows::core::*;

    #[interface("6f1f3a6e-4b8c-4d0a-9a43-1c4c2e6f0b11")]
    pub unsafe trait IValue: IUnknown {
        pub unsafe fn Value(&self) -> u32;
    }
}

#[implement(first::IValue)]
struct Value(u32);

impl first::IValue_Impl for Value_Impl {
    unsafe fn Value(&self) -> u32 {
        self.0
    }
}

fn ref_count(unknown: &IUnknown) -> u32 {
    unsafe {
        let raw = unknown.as_raw();
        (unknown.vtable().AddRef)(raw);
        (unknown.vtable().Release)(raw)
    }
}

#[test]
fn borrows_same_interface_from_other_bindings() {
    let value: first::IValue = Value(42).into();
    let before = ref_count(&value);

    let borrowed: &second::IValue = value.as_interface();

    assert_eq!(borrowed.as_raw(), value.as_raw());
    assert_eq!(unsafe { borrowed.Value() }, 42);
    assert_eq!(ref_count(&value), before);
}

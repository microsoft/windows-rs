#![allow(non_snake_case, non_camel_case_types, dead_code)]

include!("bindings.rs");

struct State;

impl Base_Impl for State {
    fn Read(&self, value: u32) -> i32 {
        value as i32 + 10
    }

    fn Reset(&self) {}
}

impl Derived_Impl for State {
    fn Extra(&self) -> i32 {
        20
    }
}

impl Final_Impl for State {
    fn Last(&self) -> i32 {
        30
    }
}

fn main() {
    let pointer = core::mem::size_of::<usize>();
    assert_eq!(core::mem::size_of::<Base_Vtbl>(), 2 * pointer);
    assert_eq!(core::mem::size_of::<Derived_Vtbl>(), 3 * pointer);
    assert_eq!(core::mem::size_of::<Final_Vtbl>(), 4 * pointer);
    assert_eq!(core::mem::offset_of!(Derived_Vtbl, Extra), 2 * pointer);
    assert_eq!(core::mem::offset_of!(Final_Vtbl, Last), 3 * pointer);
    let state = State;
    let object = Final::new(&state);
    unsafe {
        assert_eq!(object.Read(5), 15);
        object.Reset();
        assert_eq!(object.Extra(), 20);
        assert_eq!(object.Last(), 30);
    }
}

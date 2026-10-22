#![allow(non_snake_case, non_camel_case_types, dead_code)]

include!("bindings.rs");

use windows_core::Interface;

#[repr(C)]
struct State {
    vtable: *const Inherited_Vtbl,
    value: i32,
    calls: u32,
}

unsafe extern "system" fn read(this: *mut core::ffi::c_void, value: u32) -> i32 {
    unsafe {
        let state = &mut *this.cast::<State>();
        state.calls += 1;
        state.value + value as i32
    }
}

unsafe extern "system" fn reset(this: *mut core::ffi::c_void) {
    unsafe {
        let state = &mut *this.cast::<State>();
        state.calls += 1;
        state.value = 0;
    }
}

unsafe extern "system" fn scale(this: *mut core::ffi::c_void, value: i32) -> i32 {
    unsafe {
        let state = &mut *this.cast::<State>();
        state.calls += 1;
        state.value * value
    }
}

fn main() {
    assert!(!Inherited::UNKNOWN);
    assert!(!core::mem::needs_drop::<Inherited>());
    assert_eq!(
        core::mem::size_of::<Holder>(),
        3 * core::mem::size_of::<usize>()
    );
    let vtable = Inherited_Vtbl {
        base__: Extended_Vtbl {
            base__: Bare_Vtbl {
                Read: read,
                Reset: reset,
            },
            Scale: scale,
        },
    };
    let mut state = State {
        vtable: &vtable,
        value: 5,
        calls: 0,
    };
    let interface = unsafe { Inherited::from_raw((&raw mut state).cast()) };
    let copy = interface.clone();
    assert_eq!(state.calls, 0);
    unsafe {
        assert_eq!(copy.Read(7), 12);
        assert_eq!(copy.Scale(3), 15);
        copy.Reset();
    }
    assert_eq!(state.calls, 3);
    assert_eq!(state.value, 0);
}

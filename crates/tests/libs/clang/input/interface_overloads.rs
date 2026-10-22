#[allow(non_snake_case, non_camel_case_types)]
pub mod bindings;

use bindings::*;

pub fn check_vtable(value: &IOverloaded_Vtbl) {
    let _: unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> i32 = value.Read;
    let _: unsafe extern "system" fn(*mut core::ffi::c_void, f32) -> i32 = value.Read2;
    let _: unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> i32 = value.Read3;
}

const _: () = {
    let pointer = core::mem::size_of::<usize>();
    assert!(core::mem::offset_of!(IOverloaded_Vtbl, Read) == 3 * pointer);
    assert!(core::mem::offset_of!(IOverloaded_Vtbl, Read2) == 4 * pointer);
    assert!(core::mem::offset_of!(IOverloaded_Vtbl, Read3) == 5 * pointer);
    assert!(core::mem::offset_of!(IOverloaded_Vtbl, Tail) == 6 * pointer);
    assert!(core::mem::size_of::<IOverloaded_Vtbl>() == 7 * pointer);
};

#![allow(
    non_snake_case,
    non_upper_case_globals,
    non_camel_case_types,
    dead_code,
    clippy::all
)]

windows_targets::link!("ole32.dll" "system" fn CoIsHandlerConnected(punk : * mut core::ffi::c_void) -> BOOL);
pub type BOOL = i32;
pub const IID_IUnknown: GUID = GUID::from_u128(0x00000000_0000_0000_c000_000000000046);
#[repr(C)]
pub struct IUnknown_Vtbl {
    pub QueryInterface: unsafe extern "system" fn(
        this: *mut core::ffi::c_void,
        iid: *const GUID,
        interface: *mut *mut core::ffi::c_void,
    ) -> HRESULT,
    pub AddRef: unsafe extern "system" fn(this: *mut core::ffi::c_void) -> u32,
    pub Release: unsafe extern "system" fn(this: *mut core::ffi::c_void) -> u32,
}

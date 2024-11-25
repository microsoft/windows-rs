#![allow(
    non_snake_case,
    non_upper_case_globals,
    non_camel_case_types,
    dead_code,
    clippy::all
)]

pub const IID_IClassFactory: windows_sys::core::GUID =
    windows_sys::core::GUID::from_u128(0x00000001_0000_0000_c000_000000000046);
#[repr(C)]
pub struct IClassFactory_Vtbl {
    pub base__: windows_sys::core::IUnknown_Vtbl,
    pub CreateInstance: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut core::ffi::c_void,
        *const windows_sys::core::GUID,
        *mut *mut core::ffi::c_void,
    ) -> windows_sys::core::HRESULT,
    LockServer: usize,
}

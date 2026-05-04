#![allow(
    non_snake_case,
    non_upper_case_globals,
    non_camel_case_types,
    dead_code,
    clippy::all
)]

pub const IID_IPersist: windows_sys::core::GUID =
    windows_sys::core::GUID::from_u128(0x0000010c_0000_0000_c000_000000000046);
#[repr(C)]
pub struct IPersist_Vtbl {
    pub base__: windows_sys::core::IUnknown_Vtbl,
    pub GetClassID: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut windows_sys::core::GUID,
    ) -> windows_sys::core::HRESULT,
}
pub const IID_IPersistFile: windows_sys::core::GUID =
    windows_sys::core::GUID::from_u128(0x0000010b_0000_0000_c000_000000000046);
#[repr(C)]
pub struct IPersistFile_Vtbl {
    pub base__: IPersist_Vtbl,
    pub IsDirty: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_sys::core::HRESULT,
    Load: usize,
    pub Save: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        windows_sys::core::PCWSTR,
        windows_sys::core::BOOL,
    ) -> windows_sys::core::HRESULT,
    pub SaveCompleted: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        windows_sys::core::PCWSTR,
    ) -> windows_sys::core::HRESULT,
    pub GetCurFile: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut windows_sys::core::PWSTR,
    ) -> windows_sys::core::HRESULT,
}

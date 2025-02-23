#![allow(
    non_snake_case,
    non_upper_case_globals,
    non_camel_case_types,
    dead_code,
    clippy::all
)]

pub const IID_IAsyncAction: windows_sys::core::GUID =
    windows_sys::core::GUID::from_u128(0x5a648006_843a_4da9_865b_9d26e5dfad7b);
#[repr(C)]
pub struct IAsyncAction_Vtbl {
    pub base__: windows_sys::core::IInspectable_Vtbl,
    SetCompleted: usize,
    Completed: usize,
    pub GetResults: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_sys::core::HRESULT,
}
pub const IID_IAsyncInfo: windows_sys::core::GUID =
    windows_sys::core::GUID::from_u128(0x00000036_0000_0000_c000_000000000046);
#[repr(C)]
pub struct IAsyncInfo_Vtbl {
    pub base__: windows_sys::core::IInspectable_Vtbl,
    pub Id:
        unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_sys::core::HRESULT,
    Status: usize,
    pub ErrorCode: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut windows_sys::core::HRESULT,
    ) -> windows_sys::core::HRESULT,
    pub Cancel: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_sys::core::HRESULT,
    pub Close: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_sys::core::HRESULT,
}

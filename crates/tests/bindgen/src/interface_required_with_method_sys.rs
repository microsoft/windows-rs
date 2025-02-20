#![allow(
    non_snake_case,
    non_upper_case_globals,
    non_camel_case_types,
    dead_code,
    clippy::all
)]

#[repr(transparent)]
#[derive(Clone, Copy)]
pub struct AsyncStatus(pub i32);
impl AsyncStatus {
    pub const Canceled: Self = Self(2i32);
    pub const Completed: Self = Self(1i32);
    pub const Error: Self = Self(3i32);
    pub const Started: Self = Self(0i32);
}
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
    pub Status: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut AsyncStatus,
    ) -> windows_sys::core::HRESULT,
    pub ErrorCode: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut windows_sys::core::HRESULT,
    ) -> windows_sys::core::HRESULT,
    pub Cancel: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_sys::core::HRESULT,
    pub Close: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_sys::core::HRESULT,
}

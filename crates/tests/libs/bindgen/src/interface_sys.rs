#![allow(
    non_snake_case,
    non_upper_case_globals,
    non_camel_case_types,
    dead_code,
    clippy::all
)]

pub const IID_IStringable: windows_sys::core::GUID =
    windows_sys::core::GUID::from_u128(0x96369f54_8eb6_48f0_abce_c1b211e627c3);
#[repr(C)]
pub struct IStringable_Vtbl {
    pub base__: windows_sys::core::IInspectable_Vtbl,
    pub ToString: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut windows_sys::core::HSTRING,
    ) -> windows_sys::core::HRESULT,
}

#![allow(non_snake_case, non_camel_case_types)]
#[link(name = "windows")]
extern "system" {
    #[doc = "*Required features: `Win32_Graphics_Direct3D_Dxc`*"]
    pub fn DxcCreateInstance(rclsid: *const ::windows::runtime::GUID, riid: *const ::windows::runtime::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_Graphics_Direct3D_Dxc`, `Win32_System_Com`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub fn DxcCreateInstance2(pmalloc: ::windows::runtime::RawPtr, rclsid: *const ::windows::runtime::GUID, riid: *const ::windows::runtime::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows::runtime::HRESULT;
}

#![allow(non_snake_case, non_camel_case_types)]
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
pub mod Common;
#[link(name = "windows")]
extern "system" {
    #[doc = "*Required features: `Win32_Graphics_Dxgi`*"]
    pub fn CreateDXGIFactory(riid: *const ::windows::runtime::GUID, ppfactory: *mut *mut ::core::ffi::c_void) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_Graphics_Dxgi`*"]
    pub fn CreateDXGIFactory1(riid: *const ::windows::runtime::GUID, ppfactory: *mut *mut ::core::ffi::c_void) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_Graphics_Dxgi`*"]
    pub fn CreateDXGIFactory2(flags: u32, riid: *const ::windows::runtime::GUID, ppfactory: *mut *mut ::core::ffi::c_void) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_Graphics_Dxgi`*"]
    pub fn DXGIDeclareAdapterRemovalSupport() -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_Graphics_Dxgi`*"]
    pub fn DXGIGetDebugInterface1(flags: u32, riid: *const ::windows::runtime::GUID, pdebug: *mut *mut ::core::ffi::c_void) -> ::windows::runtime::HRESULT;
}

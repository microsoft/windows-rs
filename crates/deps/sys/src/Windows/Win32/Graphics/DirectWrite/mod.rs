#![allow(non_snake_case, non_camel_case_types)]
#[doc = "*Required features: `Win32_Graphics_DirectWrite`*"]
pub const DWRITE_ALPHA_MAX: u32 = 255u32;
#[doc = "*Required features: `Win32_Graphics_DirectWrite`*"]
pub const DWRITE_ERR_BASE: u32 = 20480u32;
#[doc = "*Required features: `Win32_Graphics_DirectWrite`*"]
pub const DWRITE_E_DOWNLOADCANCELLED: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2003283954i32 as _);
#[doc = "*Required features: `Win32_Graphics_DirectWrite`*"]
pub const DWRITE_E_DOWNLOADFAILED: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2003283953i32 as _);
#[doc = "*Required features: `Win32_Graphics_DirectWrite`*"]
pub const DWRITE_E_REMOTEFONT: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2003283955i32 as _);
#[doc = "*Required features: `Win32_Graphics_DirectWrite`*"]
pub const DWRITE_E_TOOMANYDOWNLOADS: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2003283952i32 as _);
#[doc = "*Required features: `Win32_Graphics_DirectWrite`*"]
pub const FACILITY_DWRITE: u32 = 2200u32;
#[link(name = "windows")]
extern "system" {
    #[doc = "*Required features: `Win32_Graphics_DirectWrite`*"]
    pub fn DWriteCreateFactory(factorytype: DWRITE_FACTORY_TYPE, iid: *const ::windows_sys::core::GUID, factory: *mut ::windows_sys::core::IUnknown) -> ::windows_sys::core::HRESULT;
}

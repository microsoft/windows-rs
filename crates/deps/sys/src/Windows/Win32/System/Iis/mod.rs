#![allow(non_snake_case, non_camel_case_types)]
#[link(name = "windows")]
extern "system" {
    #[doc = "*Required features: `Win32_System_Iis`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetExtensionVersion(pver: *mut HSE_VERSION_INFO) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_System_Iis`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetFilterVersion(pver: *mut HTTP_FILTER_VERSION) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_System_Iis`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn HttpExtensionProc(pecb: *const EXTENSION_CONTROL_BLOCK) -> u32;
    #[doc = "*Required features: `Win32_System_Iis`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn HttpFilterProc(pfc: *mut HTTP_FILTER_CONTEXT, notificationtype: u32, pvnotification: *mut ::core::ffi::c_void) -> u32;
}

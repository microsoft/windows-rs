#![allow(non_snake_case, non_camel_case_types)]
#[link(name = "windows")]
extern "system" {
    #[doc = "*Required features: `Win32_System_Iis`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetExtensionVersion();
    #[doc = "*Required features: `Win32_System_Iis`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetFilterVersion();
    #[doc = "*Required features: `Win32_System_Iis`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn HttpExtensionProc();
    #[doc = "*Required features: `Win32_System_Iis`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn HttpFilterProc();
}

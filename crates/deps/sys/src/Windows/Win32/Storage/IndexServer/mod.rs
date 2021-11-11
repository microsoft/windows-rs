#![allow(non_snake_case, non_camel_case_types)]
#[link(name = "windows")]
extern "system" {
    #[doc = "*Required features: `Win32_Storage_IndexServer`, `Win32_System_Com_StructuredStorage`*"]
    #[cfg(feature = "Win32_System_Com_StructuredStorage")]
    pub fn BindIFilterFromStorage();
    #[doc = "*Required features: `Win32_Storage_IndexServer`, `Win32_System_Com`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub fn BindIFilterFromStream();
    #[doc = "*Required features: `Win32_Storage_IndexServer`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn LoadIFilter();
    #[doc = "*Required features: `Win32_Storage_IndexServer`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn LoadIFilterEx();
}

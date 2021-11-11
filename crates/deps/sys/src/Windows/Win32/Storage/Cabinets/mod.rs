#![allow(non_snake_case, non_camel_case_types)]
#[link(name = "windows")]
extern "system" {
    #[doc = "*Required features: `Win32_Storage_Cabinets`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn FCIAddFile();
    #[doc = "*Required features: `Win32_Storage_Cabinets`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn FCICreate();
    #[doc = "*Required features: `Win32_Storage_Cabinets`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn FCIDestroy();
    #[doc = "*Required features: `Win32_Storage_Cabinets`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn FCIFlushCabinet();
    #[doc = "*Required features: `Win32_Storage_Cabinets`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn FCIFlushFolder();
    #[doc = "*Required features: `Win32_Storage_Cabinets`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn FDICopy();
    #[doc = "*Required features: `Win32_Storage_Cabinets`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn FDICreate();
    #[doc = "*Required features: `Win32_Storage_Cabinets`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn FDIDestroy();
    #[doc = "*Required features: `Win32_Storage_Cabinets`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn FDIIsCabinet();
    #[doc = "*Required features: `Win32_Storage_Cabinets`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn FDITruncateCabinet();
}

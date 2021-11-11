#![allow(non_snake_case, non_camel_case_types)]
#[link(name = "windows")]
extern "system" {
    #[doc = "*Required features: `Win32_Storage_Xps_Printing`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn StartXpsPrintJob();
    #[doc = "*Required features: `Win32_Storage_Xps_Printing`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn StartXpsPrintJob1();
}

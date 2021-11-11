#![allow(non_snake_case, non_camel_case_types)]
#[cfg(feature = "Win32_Media_DirectShow_Xml")]
pub mod Xml;
#[link(name = "windows")]
extern "system" {
    #[doc = "*Required features: `Win32_Media_DirectShow`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn AMGetErrorTextA();
    #[doc = "*Required features: `Win32_Media_DirectShow`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn AMGetErrorTextW();
}

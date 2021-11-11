#![allow(non_snake_case, non_camel_case_types)]
#[cfg(feature = "Win32_Media_DirectShow_Xml")]
pub mod Xml;
#[link(name = "windows")]
extern "system" {
    #[doc = "*Required features: `Win32_Media_DirectShow`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn AMGetErrorTextA(hr: ::windows::runtime::HRESULT, pbuffer: super::super::Foundation::PSTR, maxlen: u32) -> u32;
    #[doc = "*Required features: `Win32_Media_DirectShow`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn AMGetErrorTextW(hr: ::windows::runtime::HRESULT, pbuffer: super::super::Foundation::PWSTR, maxlen: u32) -> u32;
}

#![allow(non_snake_case, non_camel_case_types)]
#[cfg(feature = "Win32_Web_MsHtml")]
pub mod MsHtml;
#[link(name = "windows")]
extern "system" {}

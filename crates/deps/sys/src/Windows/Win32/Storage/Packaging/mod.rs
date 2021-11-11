#![allow(non_snake_case, non_camel_case_types)]
#[cfg(feature = "Win32_Storage_Packaging_Appx")]
pub mod Appx;
#[cfg(feature = "Win32_Storage_Packaging_Opc")]
pub mod Opc;
#[link(name = "windows")]
extern "system" {}

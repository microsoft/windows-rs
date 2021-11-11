#![allow(non_snake_case, non_camel_case_types)]
#[cfg(feature = "Win32_UI_Xaml_Diagnostics")]
pub mod Diagnostics;
#[link(name = "windows")]
extern "system" {}

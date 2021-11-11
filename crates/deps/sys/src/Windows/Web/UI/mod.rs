#![allow(non_snake_case, non_camel_case_types)]
#[cfg(feature = "Web_UI_Interop")]
pub mod Interop;
#[link(name = "windows")]
extern "system" {}

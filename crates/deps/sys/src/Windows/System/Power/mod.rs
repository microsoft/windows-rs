#![allow(non_snake_case, non_camel_case_types)]
#[cfg(feature = "System_Power_Diagnostics")]
pub mod Diagnostics;
#[link(name = "windows")]
extern "system" {}

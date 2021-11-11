#![allow(non_snake_case, non_camel_case_types)]
#[cfg(feature = "Win32_Graphics_Direct3D_Dxc")]
pub mod Dxc;
#[cfg(feature = "Win32_Graphics_Direct3D_Fxc")]
pub mod Fxc;
#[link(name = "windows")]
extern "system" {}

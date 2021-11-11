#![allow(non_snake_case, non_camel_case_types)]
#[cfg(feature = "Graphics_DirectX_Direct3D11")]
pub mod Direct3D11;
#[link(name = "windows")]
extern "system" {}

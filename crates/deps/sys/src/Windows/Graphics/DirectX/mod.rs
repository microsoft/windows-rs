#![allow(non_snake_case, non_camel_case_types)]
#[cfg(feature = "Graphics_DirectX_Direct3D11")]
pub mod Direct3D11;
#[link(name = "windows")]
extern "system" {}
pub struct DirectXAlphaMode(i32);
pub struct DirectXColorSpace(i32);
pub struct DirectXPixelFormat(i32);
pub struct DirectXPrimitiveTopology(i32);

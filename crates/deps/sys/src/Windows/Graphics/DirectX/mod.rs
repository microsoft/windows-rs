#![allow(non_snake_case, non_camel_case_types)]
#[cfg(feature = "Graphics_DirectX_Direct3D11")]
pub mod Direct3D11;
#[link(name = "windows")]
extern "system" {}
#[repr(C)]
pub struct DirectXAlphaMode(i32);
#[repr(C)]
pub struct DirectXColorSpace(i32);
#[repr(C)]
pub struct DirectXPixelFormat(i32);
#[repr(C)]
pub struct DirectXPrimitiveTopology(i32);

#![allow(non_snake_case, non_camel_case_types)]
#[link(name = "windows")]
extern "system" {}
pub struct Direct3DBindings(i32);
pub struct Direct3DMultisampleDescription(i32);
pub struct Direct3DSurfaceDescription(i32);
pub struct Direct3DUsage(i32);
pub struct IDirect3DDevice(pub *mut ::core::ffi::c_void);
pub struct IDirect3DSurface(pub *mut ::core::ffi::c_void);

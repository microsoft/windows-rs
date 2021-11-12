#![allow(non_snake_case, non_camel_case_types)]
#[link(name = "windows")]
extern "system" {}
#[repr(transparent)]
pub struct CompositorController(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ICompositorController(pub *mut ::core::ffi::c_void);

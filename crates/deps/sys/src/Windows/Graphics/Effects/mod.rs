#![allow(non_snake_case, non_camel_case_types)]
#[link(name = "windows")]
extern "system" {}
#[repr(transparent)]
pub struct IGraphicsEffect(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IGraphicsEffectSource(pub *mut ::core::ffi::c_void);

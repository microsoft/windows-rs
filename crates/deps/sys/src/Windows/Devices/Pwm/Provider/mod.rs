#![allow(non_snake_case, non_camel_case_types)]
#[link(name = "windows")]
extern "system" {}
pub struct IPwmControllerProvider(pub *mut ::core::ffi::c_void);
pub struct IPwmProvider(pub *mut ::core::ffi::c_void);

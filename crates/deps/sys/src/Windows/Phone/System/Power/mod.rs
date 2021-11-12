#![allow(non_snake_case, non_camel_case_types)]
#[link(name = "windows")]
extern "system" {}
pub struct IPowerManagerStatics(pub *mut ::core::ffi::c_void);
pub struct IPowerManagerStatics2(pub *mut ::core::ffi::c_void);
pub struct PowerManager(i32);
pub struct PowerSavingMode(i32);

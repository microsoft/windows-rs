#![allow(non_snake_case, non_camel_case_types)]
#[link(name = "windows")]
extern "system" {}
pub struct CreateProcessMethod(i32);
pub struct IDDEInitializer(pub *mut ::core::ffi::c_void);

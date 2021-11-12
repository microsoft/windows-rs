#![allow(non_snake_case, non_camel_case_types)]
#[link(name = "windows")]
extern "system" {}
#[repr(transparent)]
pub struct IAdcControllerProvider(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IAdcProvider(pub *mut ::core::ffi::c_void);
#[repr(C)]
pub struct ProviderAdcChannelMode(i32);

#![allow(non_snake_case, non_camel_case_types)]
#[link(name = "windows")]
extern "system" {}
pub struct IAdcControllerProvider(pub *mut ::core::ffi::c_void);
pub struct IAdcProvider(pub *mut ::core::ffi::c_void);
pub struct ProviderAdcChannelMode(i32);

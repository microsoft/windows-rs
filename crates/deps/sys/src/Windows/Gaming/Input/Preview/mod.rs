#![allow(non_snake_case, non_camel_case_types)]
#[link(name = "windows")]
extern "system" {}
pub struct GameControllerProviderInfo(i32);
pub struct IGameControllerProviderInfoStatics(pub *mut ::core::ffi::c_void);

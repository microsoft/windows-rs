#![allow(non_snake_case, non_camel_case_types)]
#[link(name = "windows")]
extern "system" {}
pub struct AppCapability(i32);
pub struct AppCapabilityAccessChangedEventArgs(i32);
pub struct AppCapabilityAccessStatus(i32);
pub struct IAppCapability(pub *mut ::core::ffi::c_void);
pub struct IAppCapabilityAccessChangedEventArgs(pub *mut ::core::ffi::c_void);
pub struct IAppCapabilityStatics(pub *mut ::core::ffi::c_void);

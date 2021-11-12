#![allow(non_snake_case, non_camel_case_types)]
#[link(name = "windows")]
extern "system" {}
#[repr(transparent)]
pub struct AppCapability(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct AppCapabilityAccessChangedEventArgs(pub *mut ::core::ffi::c_void);
pub struct AppCapabilityAccessStatus(i32);
#[repr(transparent)]
pub struct IAppCapability(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IAppCapabilityAccessChangedEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IAppCapabilityStatics(pub *mut ::core::ffi::c_void);

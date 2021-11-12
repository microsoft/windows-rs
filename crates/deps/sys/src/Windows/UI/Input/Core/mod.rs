#![allow(non_snake_case, non_camel_case_types)]
#[link(name = "windows")]
extern "system" {}
#[repr(transparent)]
pub struct IRadialControllerIndependentInputSource(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IRadialControllerIndependentInputSource2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IRadialControllerIndependentInputSourceStatics(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct RadialControllerIndependentInputSource(pub *mut ::core::ffi::c_void);

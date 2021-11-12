#![allow(non_snake_case, non_camel_case_types)]
#[link(name = "windows")]
extern "system" {}
#[repr(transparent)]
pub struct ILearningModelDeviceFactoryNative(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ILearningModelOperatorProviderNative(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ILearningModelSessionOptionsNative(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ITensorNative(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ITensorStaticsNative(pub *mut ::core::ffi::c_void);

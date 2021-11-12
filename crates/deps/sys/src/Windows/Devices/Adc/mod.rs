#![allow(non_snake_case, non_camel_case_types)]
#[cfg(feature = "Devices_Adc_Provider")]
pub mod Provider;
#[link(name = "windows")]
extern "system" {}
#[repr(transparent)]
pub struct AdcChannel(pub *mut ::core::ffi::c_void);
#[repr(C)]
pub struct AdcChannelMode(i32);
#[repr(transparent)]
pub struct AdcController(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IAdcChannel(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IAdcController(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IAdcControllerStatics(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IAdcControllerStatics2(pub *mut ::core::ffi::c_void);

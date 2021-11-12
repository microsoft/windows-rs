#![allow(non_snake_case, non_camel_case_types)]
#[cfg(feature = "Devices_Adc_Provider")]
pub mod Provider;
#[link(name = "windows")]
extern "system" {}
pub struct AdcChannel(i32);
pub struct AdcChannelMode(i32);
pub struct AdcController(i32);
pub struct IAdcChannel(pub *mut ::core::ffi::c_void);
pub struct IAdcController(pub *mut ::core::ffi::c_void);
pub struct IAdcControllerStatics(pub *mut ::core::ffi::c_void);
pub struct IAdcControllerStatics2(pub *mut ::core::ffi::c_void);

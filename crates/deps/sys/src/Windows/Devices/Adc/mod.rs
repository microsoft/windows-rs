#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals)]
#[cfg(feature = "Devices_Adc_Provider")]
pub mod Provider;
#[link(name = "windows")]
extern "system" {}
#[repr(transparent)]
pub struct AdcChannel(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct AdcChannelMode(pub i32);
impl AdcChannelMode {
    pub const SingleEnded: AdcChannelMode = AdcChannelMode(0i32);
    pub const Differential: AdcChannelMode = AdcChannelMode(1i32);
}
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

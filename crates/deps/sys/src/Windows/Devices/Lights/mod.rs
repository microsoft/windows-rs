#![allow(non_snake_case, non_camel_case_types)]
#[cfg(feature = "Devices_Lights_Effects")]
pub mod Effects;
#[link(name = "windows")]
extern "system" {}
pub struct ILamp(pub *mut ::core::ffi::c_void);
pub struct ILampArray(pub *mut ::core::ffi::c_void);
pub struct ILampArrayStatics(pub *mut ::core::ffi::c_void);
pub struct ILampAvailabilityChangedEventArgs(pub *mut ::core::ffi::c_void);
pub struct ILampInfo(pub *mut ::core::ffi::c_void);
pub struct ILampStatics(pub *mut ::core::ffi::c_void);
pub struct Lamp(i32);
pub struct LampArray(i32);
pub struct LampArrayKind(i32);
pub struct LampAvailabilityChangedEventArgs(i32);
pub struct LampInfo(i32);
pub struct LampPurposes(i32);

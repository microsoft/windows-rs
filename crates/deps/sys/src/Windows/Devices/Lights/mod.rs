#![allow(non_snake_case, non_camel_case_types)]
#[cfg(feature = "Devices_Lights_Effects")]
pub mod Effects;
#[link(name = "windows")]
extern "system" {}
#[repr(transparent)]
pub struct ILamp(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ILampArray(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ILampArrayStatics(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ILampAvailabilityChangedEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ILampInfo(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ILampStatics(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct Lamp(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct LampArray(pub *mut ::core::ffi::c_void);
pub struct LampArrayKind(i32);
#[repr(transparent)]
pub struct LampAvailabilityChangedEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct LampInfo(pub *mut ::core::ffi::c_void);
pub struct LampPurposes(i32);

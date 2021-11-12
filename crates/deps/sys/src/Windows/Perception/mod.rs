#![allow(non_snake_case, non_camel_case_types)]
#[cfg(feature = "Perception_Automation")]
pub mod Automation;
#[cfg(feature = "Perception_People")]
pub mod People;
#[cfg(feature = "Perception_Spatial")]
pub mod Spatial;
#[link(name = "windows")]
extern "system" {}
#[repr(transparent)]
pub struct IPerceptionTimestamp(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IPerceptionTimestamp2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IPerceptionTimestampHelperStatics(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IPerceptionTimestampHelperStatics2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct PerceptionTimestamp(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct PerceptionTimestampHelper(pub *mut ::core::ffi::c_void);

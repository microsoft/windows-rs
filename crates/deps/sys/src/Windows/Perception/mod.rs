#![allow(non_snake_case, non_camel_case_types)]
#[cfg(feature = "Perception_Automation")]
pub mod Automation;
#[cfg(feature = "Perception_People")]
pub mod People;
#[cfg(feature = "Perception_Spatial")]
pub mod Spatial;
#[link(name = "windows")]
extern "system" {}
pub struct IPerceptionTimestamp(pub *mut ::core::ffi::c_void);
pub struct IPerceptionTimestamp2(pub *mut ::core::ffi::c_void);
pub struct IPerceptionTimestampHelperStatics(pub *mut ::core::ffi::c_void);
pub struct IPerceptionTimestampHelperStatics2(pub *mut ::core::ffi::c_void);
pub struct PerceptionTimestamp(i32);
pub struct PerceptionTimestampHelper(i32);

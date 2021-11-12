#![allow(non_snake_case, non_camel_case_types)]
#[cfg(feature = "System_Power_Diagnostics")]
pub mod Diagnostics;
#[link(name = "windows")]
extern "system" {}
#[repr(C)]
pub struct BatteryStatus(i32);
#[repr(C)]
pub struct EnergySaverStatus(i32);
#[repr(transparent)]
pub struct IBackgroundEnergyManagerStatics(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IForegroundEnergyManagerStatics(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IPowerManagerStatics(pub *mut ::core::ffi::c_void);
#[repr(C)]
pub struct PowerSupplyStatus(i32);

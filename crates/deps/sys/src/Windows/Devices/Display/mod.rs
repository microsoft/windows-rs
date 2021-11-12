#![allow(non_snake_case, non_camel_case_types)]
#[cfg(feature = "Devices_Display_Core")]
pub mod Core;
#[link(name = "windows")]
extern "system" {}
#[repr(transparent)]
pub struct DisplayMonitor(pub *mut ::core::ffi::c_void);
pub struct DisplayMonitorConnectionKind(i32);
pub struct DisplayMonitorDescriptorKind(i32);
pub struct DisplayMonitorPhysicalConnectorKind(i32);
pub struct DisplayMonitorUsageKind(i32);
#[repr(transparent)]
pub struct IDisplayMonitor(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IDisplayMonitor2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IDisplayMonitorStatics(pub *mut ::core::ffi::c_void);

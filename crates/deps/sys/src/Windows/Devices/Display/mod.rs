#![allow(non_snake_case, non_camel_case_types)]
#[cfg(feature = "Devices_Display_Core")]
pub mod Core;
#[link(name = "windows")]
extern "system" {}
pub struct DisplayMonitor(i32);
pub struct DisplayMonitorConnectionKind(i32);
pub struct DisplayMonitorDescriptorKind(i32);
pub struct DisplayMonitorPhysicalConnectorKind(i32);
pub struct DisplayMonitorUsageKind(i32);
pub struct IDisplayMonitor(i32);
pub struct IDisplayMonitor2(i32);
pub struct IDisplayMonitorStatics(i32);

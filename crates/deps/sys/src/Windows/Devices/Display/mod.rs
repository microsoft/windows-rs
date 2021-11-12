#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals)]
#[cfg(feature = "Devices_Display_Core")]
pub mod Core;
#[link(name = "windows")]
extern "system" {}
#[repr(transparent)]
pub struct DisplayMonitor(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct DisplayMonitorConnectionKind(pub i32);
impl DisplayMonitorConnectionKind {
    pub const Internal: DisplayMonitorConnectionKind = DisplayMonitorConnectionKind(0i32);
    pub const Wired: DisplayMonitorConnectionKind = DisplayMonitorConnectionKind(1i32);
    pub const Wireless: DisplayMonitorConnectionKind = DisplayMonitorConnectionKind(2i32);
    pub const Virtual: DisplayMonitorConnectionKind = DisplayMonitorConnectionKind(3i32);
}
#[repr(transparent)]
pub struct DisplayMonitorDescriptorKind(pub i32);
impl DisplayMonitorDescriptorKind {
    pub const Edid: DisplayMonitorDescriptorKind = DisplayMonitorDescriptorKind(0i32);
    pub const DisplayId: DisplayMonitorDescriptorKind = DisplayMonitorDescriptorKind(1i32);
}
#[repr(transparent)]
pub struct DisplayMonitorPhysicalConnectorKind(pub i32);
impl DisplayMonitorPhysicalConnectorKind {
    pub const Unknown: DisplayMonitorPhysicalConnectorKind = DisplayMonitorPhysicalConnectorKind(0i32);
    pub const HD15: DisplayMonitorPhysicalConnectorKind = DisplayMonitorPhysicalConnectorKind(1i32);
    pub const AnalogTV: DisplayMonitorPhysicalConnectorKind = DisplayMonitorPhysicalConnectorKind(2i32);
    pub const Dvi: DisplayMonitorPhysicalConnectorKind = DisplayMonitorPhysicalConnectorKind(3i32);
    pub const Hdmi: DisplayMonitorPhysicalConnectorKind = DisplayMonitorPhysicalConnectorKind(4i32);
    pub const Lvds: DisplayMonitorPhysicalConnectorKind = DisplayMonitorPhysicalConnectorKind(5i32);
    pub const Sdi: DisplayMonitorPhysicalConnectorKind = DisplayMonitorPhysicalConnectorKind(6i32);
    pub const DisplayPort: DisplayMonitorPhysicalConnectorKind = DisplayMonitorPhysicalConnectorKind(7i32);
}
#[repr(transparent)]
pub struct DisplayMonitorUsageKind(pub i32);
impl DisplayMonitorUsageKind {
    pub const Standard: DisplayMonitorUsageKind = DisplayMonitorUsageKind(0i32);
    pub const HeadMounted: DisplayMonitorUsageKind = DisplayMonitorUsageKind(1i32);
    pub const SpecialPurpose: DisplayMonitorUsageKind = DisplayMonitorUsageKind(2i32);
}
#[repr(transparent)]
pub struct IDisplayMonitor(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IDisplayMonitor2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IDisplayMonitorStatics(pub *mut ::core::ffi::c_void);

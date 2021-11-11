#![allow(non_snake_case, non_camel_case_types)]
#[cfg(feature = "Devices_Display_Core")]
pub mod Core;
#[link(name = "windows")]
extern "system" {
    fn DisplayMonitor();
    fn DisplayMonitorConnectionKind();
    fn DisplayMonitorDescriptorKind();
    fn DisplayMonitorPhysicalConnectorKind();
    fn DisplayMonitorUsageKind();
    fn IDisplayMonitor();
    fn IDisplayMonitor2();
    fn IDisplayMonitorStatics();
}

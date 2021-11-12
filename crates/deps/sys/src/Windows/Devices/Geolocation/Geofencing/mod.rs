#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals)]
#[link(name = "windows")]
extern "system" {}
#[repr(transparent)]
pub struct Geofence(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct GeofenceMonitor(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct GeofenceMonitorStatus(pub i32);
impl GeofenceMonitorStatus {
    pub const Ready: GeofenceMonitorStatus = GeofenceMonitorStatus(0i32);
    pub const Initializing: GeofenceMonitorStatus = GeofenceMonitorStatus(1i32);
    pub const NoData: GeofenceMonitorStatus = GeofenceMonitorStatus(2i32);
    pub const Disabled: GeofenceMonitorStatus = GeofenceMonitorStatus(3i32);
    pub const NotInitialized: GeofenceMonitorStatus = GeofenceMonitorStatus(4i32);
    pub const NotAvailable: GeofenceMonitorStatus = GeofenceMonitorStatus(5i32);
}
#[repr(transparent)]
pub struct GeofenceRemovalReason(pub i32);
impl GeofenceRemovalReason {
    pub const Used: GeofenceRemovalReason = GeofenceRemovalReason(0i32);
    pub const Expired: GeofenceRemovalReason = GeofenceRemovalReason(1i32);
}
#[repr(transparent)]
pub struct GeofenceState(pub u32);
impl GeofenceState {
    pub const None: GeofenceState = GeofenceState(0u32);
    pub const Entered: GeofenceState = GeofenceState(1u32);
    pub const Exited: GeofenceState = GeofenceState(2u32);
    pub const Removed: GeofenceState = GeofenceState(4u32);
}
#[repr(transparent)]
pub struct GeofenceStateChangeReport(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IGeofence(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IGeofenceFactory(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IGeofenceMonitor(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IGeofenceMonitorStatics(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IGeofenceStateChangeReport(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct MonitoredGeofenceStates(pub u32);
impl MonitoredGeofenceStates {
    pub const None: MonitoredGeofenceStates = MonitoredGeofenceStates(0u32);
    pub const Entered: MonitoredGeofenceStates = MonitoredGeofenceStates(1u32);
    pub const Exited: MonitoredGeofenceStates = MonitoredGeofenceStates(2u32);
    pub const Removed: MonitoredGeofenceStates = MonitoredGeofenceStates(4u32);
}

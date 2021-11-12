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
    pub const Ready: Self = Self(0i32);
    pub const Initializing: Self = Self(1i32);
    pub const NoData: Self = Self(2i32);
    pub const Disabled: Self = Self(3i32);
    pub const NotInitialized: Self = Self(4i32);
    pub const NotAvailable: Self = Self(5i32);
}
#[repr(transparent)]
pub struct GeofenceRemovalReason(pub i32);
impl GeofenceRemovalReason {
    pub const Used: Self = Self(0i32);
    pub const Expired: Self = Self(1i32);
}
#[repr(transparent)]
pub struct GeofenceState(pub u32);
impl GeofenceState {
    pub const None: Self = Self(0u32);
    pub const Entered: Self = Self(1u32);
    pub const Exited: Self = Self(2u32);
    pub const Removed: Self = Self(4u32);
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
    pub const None: Self = Self(0u32);
    pub const Entered: Self = Self(1u32);
    pub const Exited: Self = Self(2u32);
    pub const Removed: Self = Self(4u32);
}

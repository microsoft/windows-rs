#![allow(non_snake_case, non_camel_case_types)]
#[link(name = "windows")]
extern "system" {}
pub struct Geofence(i32);
pub struct GeofenceMonitor(i32);
pub struct GeofenceMonitorStatus(i32);
pub struct GeofenceRemovalReason(i32);
pub struct GeofenceState(i32);
pub struct GeofenceStateChangeReport(i32);
pub struct IGeofence(pub *mut ::core::ffi::c_void);
pub struct IGeofenceFactory(pub *mut ::core::ffi::c_void);
pub struct IGeofenceMonitor(pub *mut ::core::ffi::c_void);
pub struct IGeofenceMonitorStatics(pub *mut ::core::ffi::c_void);
pub struct IGeofenceStateChangeReport(pub *mut ::core::ffi::c_void);
pub struct MonitoredGeofenceStates(i32);

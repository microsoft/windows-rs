#![allow(non_snake_case, non_camel_case_types)]
#[link(name = "windows")]
extern "system" {}
#[repr(transparent)]
pub struct Geofence(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct GeofenceMonitor(pub *mut ::core::ffi::c_void);
#[repr(C)]
pub struct GeofenceMonitorStatus(i32);
#[repr(C)]
pub struct GeofenceRemovalReason(i32);
#[repr(C)]
pub struct GeofenceState(i32);
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
#[repr(C)]
pub struct MonitoredGeofenceStates(i32);

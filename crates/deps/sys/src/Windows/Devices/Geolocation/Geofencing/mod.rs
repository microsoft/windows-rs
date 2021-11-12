#![allow(non_snake_case, non_camel_case_types)]
#[link(name = "windows")]
extern "system" {}
pub struct Geofence(i32);
pub struct GeofenceMonitor(i32);
pub struct GeofenceMonitorStatus(i32);
pub struct GeofenceRemovalReason(i32);
pub struct GeofenceState(i32);
pub struct GeofenceStateChangeReport(i32);
pub struct IGeofence(i32);
pub struct IGeofenceFactory(i32);
pub struct IGeofenceMonitor(i32);
pub struct IGeofenceMonitorStatics(i32);
pub struct IGeofenceStateChangeReport(i32);
pub struct MonitoredGeofenceStates(i32);

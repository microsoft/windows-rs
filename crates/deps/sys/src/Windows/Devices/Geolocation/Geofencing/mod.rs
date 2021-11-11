#![allow(non_snake_case, non_camel_case_types)]
#[link(name = "windows")]
extern "system" {
    fn Geofence();
    fn GeofenceMonitor();
    fn GeofenceMonitorStatus();
    fn GeofenceRemovalReason();
    fn GeofenceState();
    fn GeofenceStateChangeReport();
    fn IGeofence();
    fn IGeofenceFactory();
    fn IGeofenceMonitor();
    fn IGeofenceMonitorStatics();
    fn IGeofenceStateChangeReport();
    fn MonitoredGeofenceStates();
}

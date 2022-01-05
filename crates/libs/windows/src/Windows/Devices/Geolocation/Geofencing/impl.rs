#[cfg(feature = "implement_exclusive")]
pub trait IGeofenceImpl: Sized {
    fn StartTime(&self) -> ::windows::core::Result<super::super::super::Foundation::DateTime>;
    fn Duration(&self) -> ::windows::core::Result<super::super::super::Foundation::TimeSpan>;
    fn DwellTime(&self) -> ::windows::core::Result<super::super::super::Foundation::TimeSpan>;
    fn Id(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn MonitoredStates(&self) -> ::windows::core::Result<MonitoredGeofenceStates>;
    fn Geoshape(&self) -> ::windows::core::Result<super::IGeoshape>;
    fn SingleUse(&self) -> ::windows::core::Result<bool>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IGeofenceFactoryImpl: Sized {
    fn Create(&self, id: &::windows::core::HSTRING, geoshape: &::core::option::Option<super::IGeoshape>) -> ::windows::core::Result<Geofence>;
    fn CreateWithMonitorStates(&self, id: &::windows::core::HSTRING, geoshape: &::core::option::Option<super::IGeoshape>, monitoredstates: MonitoredGeofenceStates, singleuse: bool) -> ::windows::core::Result<Geofence>;
    fn CreateWithMonitorStatesAndDwellTime(&self, id: &::windows::core::HSTRING, geoshape: &::core::option::Option<super::IGeoshape>, monitoredstates: MonitoredGeofenceStates, singleuse: bool, dwelltime: &super::super::super::Foundation::TimeSpan) -> ::windows::core::Result<Geofence>;
    fn CreateWithMonitorStatesDwellTimeStartTimeAndDuration(&self, id: &::windows::core::HSTRING, geoshape: &::core::option::Option<super::IGeoshape>, monitoredstates: MonitoredGeofenceStates, singleuse: bool, dwelltime: &super::super::super::Foundation::TimeSpan, starttime: &super::super::super::Foundation::DateTime, duration: &super::super::super::Foundation::TimeSpan) -> ::windows::core::Result<Geofence>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IGeofenceMonitorImpl: Sized {
    fn Status(&self) -> ::windows::core::Result<GeofenceMonitorStatus>;
    fn Geofences(&self) -> ::windows::core::Result<super::super::super::Foundation::Collections::IVector<Geofence>>;
    fn LastKnownGeoposition(&self) -> ::windows::core::Result<super::Geoposition>;
    fn GeofenceStateChanged(&self, eventhandler: &::core::option::Option<super::super::super::Foundation::TypedEventHandler<GeofenceMonitor, ::windows::core::IInspectable>>) -> ::windows::core::Result<super::super::super::Foundation::EventRegistrationToken>;
    fn RemoveGeofenceStateChanged(&self, token: &super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn ReadReports(&self) -> ::windows::core::Result<super::super::super::Foundation::Collections::IVectorView<GeofenceStateChangeReport>>;
    fn StatusChanged(&self, eventhandler: &::core::option::Option<super::super::super::Foundation::TypedEventHandler<GeofenceMonitor, ::windows::core::IInspectable>>) -> ::windows::core::Result<super::super::super::Foundation::EventRegistrationToken>;
    fn RemoveStatusChanged(&self, token: &super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IGeofenceMonitorStaticsImpl: Sized {
    fn Current(&self) -> ::windows::core::Result<GeofenceMonitor>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IGeofenceStateChangeReportImpl: Sized {
    fn NewState(&self) -> ::windows::core::Result<GeofenceState>;
    fn Geofence(&self) -> ::windows::core::Result<Geofence>;
    fn Geoposition(&self) -> ::windows::core::Result<super::Geoposition>;
    fn RemovalReason(&self) -> ::windows::core::Result<GeofenceRemovalReason>;
}

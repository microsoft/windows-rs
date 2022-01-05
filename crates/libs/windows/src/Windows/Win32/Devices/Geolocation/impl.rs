pub trait ICivicAddressReportImpl: Sized + ILocationReportImpl {
    fn GetAddressLine1();
    fn GetAddressLine2();
    fn GetCity();
    fn GetStateProvince();
    fn GetPostalCode();
    fn GetCountryRegion();
    fn GetDetailLevel();
}
#[cfg(feature = "Win32_System_Com")]
pub trait ICivicAddressReportFactoryImpl: Sized + ILocationReportFactoryImpl + IDispatchImpl {
    fn CivicAddressReport();
}
pub trait IDefaultLocationImpl: Sized {
    fn SetReport();
    fn GetReport();
}
#[cfg(feature = "Win32_System_Com")]
pub trait IDispCivicAddressReportImpl: Sized + IDispatchImpl {
    fn AddressLine1();
    fn AddressLine2();
    fn City();
    fn StateProvince();
    fn PostalCode();
    fn CountryRegion();
    fn DetailLevel();
    fn Timestamp();
}
#[cfg(feature = "Win32_System_Com")]
pub trait IDispLatLongReportImpl: Sized + IDispatchImpl {
    fn Latitude();
    fn Longitude();
    fn ErrorRadius();
    fn Altitude();
    fn AltitudeError();
    fn Timestamp();
}
pub trait ILatLongReportImpl: Sized + ILocationReportImpl {
    fn GetLatitude();
    fn GetLongitude();
    fn GetErrorRadius();
    fn GetAltitude();
    fn GetAltitudeError();
}
#[cfg(feature = "Win32_System_Com")]
pub trait ILatLongReportFactoryImpl: Sized + ILocationReportFactoryImpl + IDispatchImpl {
    fn LatLongReport();
}
pub trait ILocationImpl: Sized {
    fn RegisterForReport();
    fn UnregisterForReport();
    fn GetReport();
    fn GetReportStatus();
    fn GetReportInterval();
    fn SetReportInterval();
    fn GetDesiredAccuracy();
    fn SetDesiredAccuracy();
    fn RequestPermissions();
}
pub trait ILocationEventsImpl: Sized {
    fn OnLocationChanged();
    fn OnStatusChanged();
}
pub trait ILocationPowerImpl: Sized {
    fn Connect();
    fn Disconnect();
}
pub trait ILocationReportImpl: Sized {
    fn GetSensorID();
    fn GetTimestamp();
    fn GetValue();
}
#[cfg(feature = "Win32_System_Com")]
pub trait ILocationReportFactoryImpl: Sized + IDispatchImpl {
    fn ListenForReports();
    fn StopListeningForReports();
    fn Status();
    fn ReportInterval();
    fn SetReportInterval();
    fn DesiredAccuracy();
    fn SetDesiredAccuracy();
    fn RequestPermissions();
}
#[cfg(feature = "Win32_System_Com")]
pub trait _ICivicAddressReportFactoryEventsImpl: Sized + IDispatchImpl {}
#[cfg(feature = "Win32_System_Com")]
pub trait _ILatLongReportFactoryEventsImpl: Sized + IDispatchImpl {}

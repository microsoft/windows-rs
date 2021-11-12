#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals)]
#[cfg(feature = "Devices_Geolocation_Geofencing")]
pub mod Geofencing;
#[link(name = "windows")]
extern "system" {}
#[repr(transparent)]
pub struct AltitudeReferenceSystem(pub i32);
impl AltitudeReferenceSystem {
    pub const Unspecified: AltitudeReferenceSystem = AltitudeReferenceSystem(0i32);
    pub const Terrain: AltitudeReferenceSystem = AltitudeReferenceSystem(1i32);
    pub const Ellipsoid: AltitudeReferenceSystem = AltitudeReferenceSystem(2i32);
    pub const Geoid: AltitudeReferenceSystem = AltitudeReferenceSystem(3i32);
    pub const Surface: AltitudeReferenceSystem = AltitudeReferenceSystem(4i32);
}
#[repr(C)]
pub struct BasicGeoposition(i32);
#[repr(transparent)]
pub struct CivicAddress(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct GeoboundingBox(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct Geocircle(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct Geocoordinate(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct GeocoordinateSatelliteData(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct GeolocationAccessStatus(pub i32);
impl GeolocationAccessStatus {
    pub const Unspecified: GeolocationAccessStatus = GeolocationAccessStatus(0i32);
    pub const Allowed: GeolocationAccessStatus = GeolocationAccessStatus(1i32);
    pub const Denied: GeolocationAccessStatus = GeolocationAccessStatus(2i32);
}
#[repr(transparent)]
pub struct Geolocator(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct Geopath(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct Geopoint(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct Geoposition(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct GeoshapeType(pub i32);
impl GeoshapeType {
    pub const Geopoint: GeoshapeType = GeoshapeType(0i32);
    pub const Geocircle: GeoshapeType = GeoshapeType(1i32);
    pub const Geopath: GeoshapeType = GeoshapeType(2i32);
    pub const GeoboundingBox: GeoshapeType = GeoshapeType(3i32);
}
#[repr(transparent)]
pub struct Geovisit(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct GeovisitMonitor(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct GeovisitStateChangedEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct GeovisitTriggerDetails(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ICivicAddress(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IGeoboundingBox(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IGeoboundingBoxFactory(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IGeoboundingBoxStatics(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IGeocircle(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IGeocircleFactory(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IGeocoordinate(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IGeocoordinateSatelliteData(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IGeocoordinateSatelliteData2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IGeocoordinateWithPoint(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IGeocoordinateWithPositionData(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IGeocoordinateWithPositionSourceTimestamp(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IGeocoordinateWithRemoteSource(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IGeolocator(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IGeolocator2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IGeolocatorStatics(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IGeolocatorStatics2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IGeolocatorWithScalarAccuracy(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IGeopath(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IGeopathFactory(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IGeopoint(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IGeopointFactory(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IGeoposition(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IGeoposition2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IGeoshape(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IGeovisit(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IGeovisitMonitor(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IGeovisitMonitorStatics(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IGeovisitStateChangedEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IGeovisitTriggerDetails(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IPositionChangedEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IStatusChangedEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IVenueData(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct PositionAccuracy(pub i32);
impl PositionAccuracy {
    pub const Default: PositionAccuracy = PositionAccuracy(0i32);
    pub const High: PositionAccuracy = PositionAccuracy(1i32);
}
#[repr(transparent)]
pub struct PositionChangedEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct PositionSource(pub i32);
impl PositionSource {
    pub const Cellular: PositionSource = PositionSource(0i32);
    pub const Satellite: PositionSource = PositionSource(1i32);
    pub const WiFi: PositionSource = PositionSource(2i32);
    pub const IPAddress: PositionSource = PositionSource(3i32);
    pub const Unknown: PositionSource = PositionSource(4i32);
    pub const Default: PositionSource = PositionSource(5i32);
    pub const Obfuscated: PositionSource = PositionSource(6i32);
}
#[repr(transparent)]
pub struct PositionStatus(pub i32);
impl PositionStatus {
    pub const Ready: PositionStatus = PositionStatus(0i32);
    pub const Initializing: PositionStatus = PositionStatus(1i32);
    pub const NoData: PositionStatus = PositionStatus(2i32);
    pub const Disabled: PositionStatus = PositionStatus(3i32);
    pub const NotInitialized: PositionStatus = PositionStatus(4i32);
    pub const NotAvailable: PositionStatus = PositionStatus(5i32);
}
#[repr(transparent)]
pub struct StatusChangedEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct VenueData(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct VisitMonitoringScope(pub i32);
impl VisitMonitoringScope {
    pub const Venue: VisitMonitoringScope = VisitMonitoringScope(0i32);
    pub const City: VisitMonitoringScope = VisitMonitoringScope(1i32);
}
#[repr(transparent)]
pub struct VisitStateChange(pub i32);
impl VisitStateChange {
    pub const TrackingLost: VisitStateChange = VisitStateChange(0i32);
    pub const Arrived: VisitStateChange = VisitStateChange(1i32);
    pub const Departed: VisitStateChange = VisitStateChange(2i32);
    pub const OtherMovement: VisitStateChange = VisitStateChange(3i32);
}

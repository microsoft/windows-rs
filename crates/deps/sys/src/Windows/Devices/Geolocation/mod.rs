#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals)]
#[cfg(feature = "Devices_Geolocation_Geofencing")]
pub mod Geofencing;
#[link(name = "windows")]
extern "system" {}
#[repr(transparent)]
pub struct AltitudeReferenceSystem(pub i32);
impl AltitudeReferenceSystem {
    pub const Unspecified: Self = Self(0i32);
    pub const Terrain: Self = Self(1i32);
    pub const Ellipsoid: Self = Self(2i32);
    pub const Geoid: Self = Self(3i32);
    pub const Surface: Self = Self(4i32);
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
    pub const Unspecified: Self = Self(0i32);
    pub const Allowed: Self = Self(1i32);
    pub const Denied: Self = Self(2i32);
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
    pub const Geopoint: Self = Self(0i32);
    pub const Geocircle: Self = Self(1i32);
    pub const Geopath: Self = Self(2i32);
    pub const GeoboundingBox: Self = Self(3i32);
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
    pub const Default: Self = Self(0i32);
    pub const High: Self = Self(1i32);
}
#[repr(transparent)]
pub struct PositionChangedEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct PositionSource(pub i32);
impl PositionSource {
    pub const Cellular: Self = Self(0i32);
    pub const Satellite: Self = Self(1i32);
    pub const WiFi: Self = Self(2i32);
    pub const IPAddress: Self = Self(3i32);
    pub const Unknown: Self = Self(4i32);
    pub const Default: Self = Self(5i32);
    pub const Obfuscated: Self = Self(6i32);
}
#[repr(transparent)]
pub struct PositionStatus(pub i32);
impl PositionStatus {
    pub const Ready: Self = Self(0i32);
    pub const Initializing: Self = Self(1i32);
    pub const NoData: Self = Self(2i32);
    pub const Disabled: Self = Self(3i32);
    pub const NotInitialized: Self = Self(4i32);
    pub const NotAvailable: Self = Self(5i32);
}
#[repr(transparent)]
pub struct StatusChangedEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct VenueData(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct VisitMonitoringScope(pub i32);
impl VisitMonitoringScope {
    pub const Venue: Self = Self(0i32);
    pub const City: Self = Self(1i32);
}
#[repr(transparent)]
pub struct VisitStateChange(pub i32);
impl VisitStateChange {
    pub const TrackingLost: Self = Self(0i32);
    pub const Arrived: Self = Self(1i32);
    pub const Departed: Self = Self(2i32);
    pub const OtherMovement: Self = Self(3i32);
}

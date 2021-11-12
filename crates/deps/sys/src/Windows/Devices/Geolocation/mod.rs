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
impl ::core::marker::Copy for AltitudeReferenceSystem {}
impl ::core::clone::Clone for AltitudeReferenceSystem {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct BasicGeoposition {
    pub Latitude: f64,
    pub Longitude: f64,
    pub Altitude: f64,
}
impl ::core::marker::Copy for BasicGeoposition {}
impl ::core::clone::Clone for BasicGeoposition {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct CivicAddress(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for CivicAddress {}
impl ::core::clone::Clone for CivicAddress {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct GeoboundingBox(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for GeoboundingBox {}
impl ::core::clone::Clone for GeoboundingBox {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct Geocircle(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for Geocircle {}
impl ::core::clone::Clone for Geocircle {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct Geocoordinate(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for Geocoordinate {}
impl ::core::clone::Clone for Geocoordinate {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct GeocoordinateSatelliteData(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for GeocoordinateSatelliteData {}
impl ::core::clone::Clone for GeocoordinateSatelliteData {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct GeolocationAccessStatus(pub i32);
impl GeolocationAccessStatus {
    pub const Unspecified: Self = Self(0i32);
    pub const Allowed: Self = Self(1i32);
    pub const Denied: Self = Self(2i32);
}
impl ::core::marker::Copy for GeolocationAccessStatus {}
impl ::core::clone::Clone for GeolocationAccessStatus {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct Geolocator(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for Geolocator {}
impl ::core::clone::Clone for Geolocator {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct Geopath(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for Geopath {}
impl ::core::clone::Clone for Geopath {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct Geopoint(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for Geopoint {}
impl ::core::clone::Clone for Geopoint {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct Geoposition(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for Geoposition {}
impl ::core::clone::Clone for Geoposition {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct GeoshapeType(pub i32);
impl GeoshapeType {
    pub const Geopoint: Self = Self(0i32);
    pub const Geocircle: Self = Self(1i32);
    pub const Geopath: Self = Self(2i32);
    pub const GeoboundingBox: Self = Self(3i32);
}
impl ::core::marker::Copy for GeoshapeType {}
impl ::core::clone::Clone for GeoshapeType {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct Geovisit(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for Geovisit {}
impl ::core::clone::Clone for Geovisit {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct GeovisitMonitor(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for GeovisitMonitor {}
impl ::core::clone::Clone for GeovisitMonitor {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct GeovisitStateChangedEventArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for GeovisitStateChangedEventArgs {}
impl ::core::clone::Clone for GeovisitStateChangedEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct GeovisitTriggerDetails(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for GeovisitTriggerDetails {}
impl ::core::clone::Clone for GeovisitTriggerDetails {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ICivicAddress(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ICivicAddress {}
impl ::core::clone::Clone for ICivicAddress {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IGeoboundingBox(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IGeoboundingBox {}
impl ::core::clone::Clone for IGeoboundingBox {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IGeoboundingBoxFactory(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IGeoboundingBoxFactory {}
impl ::core::clone::Clone for IGeoboundingBoxFactory {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IGeoboundingBoxStatics(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IGeoboundingBoxStatics {}
impl ::core::clone::Clone for IGeoboundingBoxStatics {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IGeocircle(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IGeocircle {}
impl ::core::clone::Clone for IGeocircle {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IGeocircleFactory(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IGeocircleFactory {}
impl ::core::clone::Clone for IGeocircleFactory {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IGeocoordinate(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IGeocoordinate {}
impl ::core::clone::Clone for IGeocoordinate {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IGeocoordinateSatelliteData(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IGeocoordinateSatelliteData {}
impl ::core::clone::Clone for IGeocoordinateSatelliteData {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IGeocoordinateSatelliteData2(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IGeocoordinateSatelliteData2 {}
impl ::core::clone::Clone for IGeocoordinateSatelliteData2 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IGeocoordinateWithPoint(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IGeocoordinateWithPoint {}
impl ::core::clone::Clone for IGeocoordinateWithPoint {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IGeocoordinateWithPositionData(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IGeocoordinateWithPositionData {}
impl ::core::clone::Clone for IGeocoordinateWithPositionData {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IGeocoordinateWithPositionSourceTimestamp(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IGeocoordinateWithPositionSourceTimestamp {}
impl ::core::clone::Clone for IGeocoordinateWithPositionSourceTimestamp {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IGeocoordinateWithRemoteSource(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IGeocoordinateWithRemoteSource {}
impl ::core::clone::Clone for IGeocoordinateWithRemoteSource {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IGeolocator(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IGeolocator {}
impl ::core::clone::Clone for IGeolocator {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IGeolocator2(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IGeolocator2 {}
impl ::core::clone::Clone for IGeolocator2 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IGeolocatorStatics(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IGeolocatorStatics {}
impl ::core::clone::Clone for IGeolocatorStatics {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IGeolocatorStatics2(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IGeolocatorStatics2 {}
impl ::core::clone::Clone for IGeolocatorStatics2 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IGeolocatorWithScalarAccuracy(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IGeolocatorWithScalarAccuracy {}
impl ::core::clone::Clone for IGeolocatorWithScalarAccuracy {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IGeopath(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IGeopath {}
impl ::core::clone::Clone for IGeopath {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IGeopathFactory(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IGeopathFactory {}
impl ::core::clone::Clone for IGeopathFactory {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IGeopoint(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IGeopoint {}
impl ::core::clone::Clone for IGeopoint {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IGeopointFactory(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IGeopointFactory {}
impl ::core::clone::Clone for IGeopointFactory {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IGeoposition(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IGeoposition {}
impl ::core::clone::Clone for IGeoposition {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IGeoposition2(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IGeoposition2 {}
impl ::core::clone::Clone for IGeoposition2 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IGeoshape(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IGeoshape {}
impl ::core::clone::Clone for IGeoshape {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IGeovisit(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IGeovisit {}
impl ::core::clone::Clone for IGeovisit {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IGeovisitMonitor(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IGeovisitMonitor {}
impl ::core::clone::Clone for IGeovisitMonitor {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IGeovisitMonitorStatics(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IGeovisitMonitorStatics {}
impl ::core::clone::Clone for IGeovisitMonitorStatics {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IGeovisitStateChangedEventArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IGeovisitStateChangedEventArgs {}
impl ::core::clone::Clone for IGeovisitStateChangedEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IGeovisitTriggerDetails(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IGeovisitTriggerDetails {}
impl ::core::clone::Clone for IGeovisitTriggerDetails {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IPositionChangedEventArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IPositionChangedEventArgs {}
impl ::core::clone::Clone for IPositionChangedEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IStatusChangedEventArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IStatusChangedEventArgs {}
impl ::core::clone::Clone for IStatusChangedEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IVenueData(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IVenueData {}
impl ::core::clone::Clone for IVenueData {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct PositionAccuracy(pub i32);
impl PositionAccuracy {
    pub const Default: Self = Self(0i32);
    pub const High: Self = Self(1i32);
}
impl ::core::marker::Copy for PositionAccuracy {}
impl ::core::clone::Clone for PositionAccuracy {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct PositionChangedEventArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for PositionChangedEventArgs {}
impl ::core::clone::Clone for PositionChangedEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
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
impl ::core::marker::Copy for PositionSource {}
impl ::core::clone::Clone for PositionSource {
    fn clone(&self) -> Self {
        *self
    }
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
impl ::core::marker::Copy for PositionStatus {}
impl ::core::clone::Clone for PositionStatus {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct StatusChangedEventArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for StatusChangedEventArgs {}
impl ::core::clone::Clone for StatusChangedEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct VenueData(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for VenueData {}
impl ::core::clone::Clone for VenueData {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct VisitMonitoringScope(pub i32);
impl VisitMonitoringScope {
    pub const Venue: Self = Self(0i32);
    pub const City: Self = Self(1i32);
}
impl ::core::marker::Copy for VisitMonitoringScope {}
impl ::core::clone::Clone for VisitMonitoringScope {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct VisitStateChange(pub i32);
impl VisitStateChange {
    pub const TrackingLost: Self = Self(0i32);
    pub const Arrived: Self = Self(1i32);
    pub const Departed: Self = Self(2i32);
    pub const OtherMovement: Self = Self(3i32);
}
impl ::core::marker::Copy for VisitStateChange {}
impl ::core::clone::Clone for VisitStateChange {
    fn clone(&self) -> Self {
        *self
    }
}

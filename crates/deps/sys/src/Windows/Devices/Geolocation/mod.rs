#![allow(non_snake_case, non_camel_case_types)]
#[cfg(feature = "Devices_Geolocation_Geofencing")]
pub mod Geofencing;
#[link(name = "windows")]
extern "system" {}
pub struct AltitudeReferenceSystem(i32);
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
pub struct GeolocationAccessStatus(i32);
#[repr(transparent)]
pub struct Geolocator(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct Geopath(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct Geopoint(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct Geoposition(pub *mut ::core::ffi::c_void);
pub struct GeoshapeType(i32);
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
pub struct PositionAccuracy(i32);
#[repr(transparent)]
pub struct PositionChangedEventArgs(pub *mut ::core::ffi::c_void);
pub struct PositionSource(i32);
pub struct PositionStatus(i32);
#[repr(transparent)]
pub struct StatusChangedEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct VenueData(pub *mut ::core::ffi::c_void);
pub struct VisitMonitoringScope(i32);
pub struct VisitStateChange(i32);

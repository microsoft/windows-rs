#[cfg(feature = "implement_exclusive")]
pub trait ICivicAddressImpl: Sized {
    fn Country(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn State(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn City(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn PostalCode(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn Timestamp(&self) -> ::windows::core::Result<super::super::Foundation::DateTime>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IGeoboundingBoxImpl: Sized + IGeoshapeImpl {
    fn NorthwestCorner(&self) -> ::windows::core::Result<BasicGeoposition>;
    fn SoutheastCorner(&self) -> ::windows::core::Result<BasicGeoposition>;
    fn Center(&self) -> ::windows::core::Result<BasicGeoposition>;
    fn MinAltitude(&self) -> ::windows::core::Result<f64>;
    fn MaxAltitude(&self) -> ::windows::core::Result<f64>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IGeoboundingBoxFactoryImpl: Sized {
    fn Create(&self, northwestcorner: &BasicGeoposition, southeastcorner: &BasicGeoposition) -> ::windows::core::Result<GeoboundingBox>;
    fn CreateWithAltitudeReference(&self, northwestcorner: &BasicGeoposition, southeastcorner: &BasicGeoposition, altitudereferencesystem: AltitudeReferenceSystem) -> ::windows::core::Result<GeoboundingBox>;
    fn CreateWithAltitudeReferenceAndSpatialReference(&self, northwestcorner: &BasicGeoposition, southeastcorner: &BasicGeoposition, altitudereferencesystem: AltitudeReferenceSystem, spatialreferenceid: u32) -> ::windows::core::Result<GeoboundingBox>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IGeoboundingBoxStaticsImpl: Sized {
    fn TryCompute(&self, positions: &::core::option::Option<super::super::Foundation::Collections::IIterable<BasicGeoposition>>) -> ::windows::core::Result<GeoboundingBox>;
    fn TryComputeWithAltitudeReference(&self, positions: &::core::option::Option<super::super::Foundation::Collections::IIterable<BasicGeoposition>>, altituderefsystem: AltitudeReferenceSystem) -> ::windows::core::Result<GeoboundingBox>;
    fn TryComputeWithAltitudeReferenceAndSpatialReference(&self, positions: &::core::option::Option<super::super::Foundation::Collections::IIterable<BasicGeoposition>>, altituderefsystem: AltitudeReferenceSystem, spatialreferenceid: u32) -> ::windows::core::Result<GeoboundingBox>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IGeocircleImpl: Sized + IGeoshapeImpl {
    fn Center(&self) -> ::windows::core::Result<BasicGeoposition>;
    fn Radius(&self) -> ::windows::core::Result<f64>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IGeocircleFactoryImpl: Sized {
    fn Create(&self, position: &BasicGeoposition, radius: f64) -> ::windows::core::Result<Geocircle>;
    fn CreateWithAltitudeReferenceSystem(&self, position: &BasicGeoposition, radius: f64, altitudereferencesystem: AltitudeReferenceSystem) -> ::windows::core::Result<Geocircle>;
    fn CreateWithAltitudeReferenceSystemAndSpatialReferenceId(&self, position: &BasicGeoposition, radius: f64, altitudereferencesystem: AltitudeReferenceSystem, spatialreferenceid: u32) -> ::windows::core::Result<Geocircle>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IGeocoordinateImpl: Sized {
    fn Latitude(&self) -> ::windows::core::Result<f64>;
    fn Longitude(&self) -> ::windows::core::Result<f64>;
    fn Altitude(&self) -> ::windows::core::Result<super::super::Foundation::IReference<f64>>;
    fn Accuracy(&self) -> ::windows::core::Result<f64>;
    fn AltitudeAccuracy(&self) -> ::windows::core::Result<super::super::Foundation::IReference<f64>>;
    fn Heading(&self) -> ::windows::core::Result<super::super::Foundation::IReference<f64>>;
    fn Speed(&self) -> ::windows::core::Result<super::super::Foundation::IReference<f64>>;
    fn Timestamp(&self) -> ::windows::core::Result<super::super::Foundation::DateTime>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IGeocoordinateSatelliteDataImpl: Sized {
    fn PositionDilutionOfPrecision(&self) -> ::windows::core::Result<super::super::Foundation::IReference<f64>>;
    fn HorizontalDilutionOfPrecision(&self) -> ::windows::core::Result<super::super::Foundation::IReference<f64>>;
    fn VerticalDilutionOfPrecision(&self) -> ::windows::core::Result<super::super::Foundation::IReference<f64>>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IGeocoordinateSatelliteData2Impl: Sized {
    fn GeometricDilutionOfPrecision(&self) -> ::windows::core::Result<super::super::Foundation::IReference<f64>>;
    fn TimeDilutionOfPrecision(&self) -> ::windows::core::Result<super::super::Foundation::IReference<f64>>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IGeocoordinateWithPointImpl: Sized {
    fn Point(&self) -> ::windows::core::Result<Geopoint>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IGeocoordinateWithPositionDataImpl: Sized + IGeocoordinateImpl {
    fn PositionSource(&self) -> ::windows::core::Result<PositionSource>;
    fn SatelliteData(&self) -> ::windows::core::Result<GeocoordinateSatelliteData>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IGeocoordinateWithPositionSourceTimestampImpl: Sized {
    fn PositionSourceTimestamp(&self) -> ::windows::core::Result<super::super::Foundation::IReference<super::super::Foundation::DateTime>>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IGeocoordinateWithRemoteSourceImpl: Sized {
    fn IsRemoteSource(&self) -> ::windows::core::Result<bool>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IGeolocatorImpl: Sized {
    fn DesiredAccuracy(&self) -> ::windows::core::Result<PositionAccuracy>;
    fn SetDesiredAccuracy(&self, value: PositionAccuracy) -> ::windows::core::Result<()>;
    fn MovementThreshold(&self) -> ::windows::core::Result<f64>;
    fn SetMovementThreshold(&self, value: f64) -> ::windows::core::Result<()>;
    fn ReportInterval(&self) -> ::windows::core::Result<u32>;
    fn SetReportInterval(&self, value: u32) -> ::windows::core::Result<()>;
    fn LocationStatus(&self) -> ::windows::core::Result<PositionStatus>;
    fn GetGeopositionAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<Geoposition>>;
    fn GetGeopositionAsyncWithAgeAndTimeout(&self, maximumage: &super::super::Foundation::TimeSpan, timeout: &super::super::Foundation::TimeSpan) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<Geoposition>>;
    fn PositionChanged(&self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<Geolocator, PositionChangedEventArgs>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemovePositionChanged(&self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn StatusChanged(&self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<Geolocator, StatusChangedEventArgs>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveStatusChanged(&self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IGeolocator2Impl: Sized {
    fn AllowFallbackToConsentlessPositions(&self) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IGeolocatorStaticsImpl: Sized {
    fn RequestAccessAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<GeolocationAccessStatus>>;
    fn GetGeopositionHistoryAsync(&self, starttime: &super::super::Foundation::DateTime) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IVectorView<Geoposition>>>;
    fn GetGeopositionHistoryWithDurationAsync(&self, starttime: &super::super::Foundation::DateTime, duration: &super::super::Foundation::TimeSpan) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IVectorView<Geoposition>>>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IGeolocatorStatics2Impl: Sized {
    fn IsDefaultGeopositionRecommended(&self) -> ::windows::core::Result<bool>;
    fn SetDefaultGeoposition(&self, value: &::core::option::Option<super::super::Foundation::IReference<BasicGeoposition>>) -> ::windows::core::Result<()>;
    fn DefaultGeoposition(&self) -> ::windows::core::Result<super::super::Foundation::IReference<BasicGeoposition>>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IGeolocatorWithScalarAccuracyImpl: Sized + IGeolocatorImpl {
    fn DesiredAccuracyInMeters(&self) -> ::windows::core::Result<super::super::Foundation::IReference<u32>>;
    fn SetDesiredAccuracyInMeters(&self, value: &::core::option::Option<super::super::Foundation::IReference<u32>>) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IGeopathImpl: Sized + IGeoshapeImpl {
    fn Positions(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<BasicGeoposition>>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IGeopathFactoryImpl: Sized {
    fn Create(&self, positions: &::core::option::Option<super::super::Foundation::Collections::IIterable<BasicGeoposition>>) -> ::windows::core::Result<Geopath>;
    fn CreateWithAltitudeReference(&self, positions: &::core::option::Option<super::super::Foundation::Collections::IIterable<BasicGeoposition>>, altitudereferencesystem: AltitudeReferenceSystem) -> ::windows::core::Result<Geopath>;
    fn CreateWithAltitudeReferenceAndSpatialReference(&self, positions: &::core::option::Option<super::super::Foundation::Collections::IIterable<BasicGeoposition>>, altitudereferencesystem: AltitudeReferenceSystem, spatialreferenceid: u32) -> ::windows::core::Result<Geopath>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IGeopointImpl: Sized + IGeoshapeImpl {
    fn Position(&self) -> ::windows::core::Result<BasicGeoposition>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IGeopointFactoryImpl: Sized {
    fn Create(&self, position: &BasicGeoposition) -> ::windows::core::Result<Geopoint>;
    fn CreateWithAltitudeReferenceSystem(&self, position: &BasicGeoposition, altitudereferencesystem: AltitudeReferenceSystem) -> ::windows::core::Result<Geopoint>;
    fn CreateWithAltitudeReferenceSystemAndSpatialReferenceId(&self, position: &BasicGeoposition, altitudereferencesystem: AltitudeReferenceSystem, spatialreferenceid: u32) -> ::windows::core::Result<Geopoint>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IGeopositionImpl: Sized {
    fn Coordinate(&self) -> ::windows::core::Result<Geocoordinate>;
    fn CivicAddress(&self) -> ::windows::core::Result<CivicAddress>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IGeoposition2Impl: Sized + IGeopositionImpl {
    fn VenueData(&self) -> ::windows::core::Result<VenueData>;
}
pub trait IGeoshapeImpl: Sized {
    fn GeoshapeType(&self) -> ::windows::core::Result<GeoshapeType>;
    fn SpatialReferenceId(&self) -> ::windows::core::Result<u32>;
    fn AltitudeReferenceSystem(&self) -> ::windows::core::Result<AltitudeReferenceSystem>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IGeovisitImpl: Sized {
    fn Position(&self) -> ::windows::core::Result<Geoposition>;
    fn StateChange(&self) -> ::windows::core::Result<VisitStateChange>;
    fn Timestamp(&self) -> ::windows::core::Result<super::super::Foundation::DateTime>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IGeovisitMonitorImpl: Sized {
    fn MonitoringScope(&self) -> ::windows::core::Result<VisitMonitoringScope>;
    fn Start(&self, value: VisitMonitoringScope) -> ::windows::core::Result<()>;
    fn Stop(&self) -> ::windows::core::Result<()>;
    fn VisitStateChanged(&self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<GeovisitMonitor, GeovisitStateChangedEventArgs>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveVisitStateChanged(&self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IGeovisitMonitorStaticsImpl: Sized {
    fn GetLastReportAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<Geovisit>>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IGeovisitStateChangedEventArgsImpl: Sized {
    fn Visit(&self) -> ::windows::core::Result<Geovisit>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IGeovisitTriggerDetailsImpl: Sized {
    fn ReadReports(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<Geovisit>>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IPositionChangedEventArgsImpl: Sized {
    fn Position(&self) -> ::windows::core::Result<Geoposition>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IStatusChangedEventArgsImpl: Sized {
    fn Status(&self) -> ::windows::core::Result<PositionStatus>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IVenueDataImpl: Sized {
    fn Id(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn Level(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
}

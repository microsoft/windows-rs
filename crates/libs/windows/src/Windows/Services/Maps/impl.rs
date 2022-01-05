#[cfg(feature = "implement_exclusive")]
pub trait IEnhancedWaypointImpl: Sized {
    fn Point(&self) -> ::windows::core::Result<super::super::Devices::Geolocation::Geopoint>;
    fn Kind(&self) -> ::windows::core::Result<WaypointKind>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IEnhancedWaypointFactoryImpl: Sized {
    fn Create(&self, point: &::core::option::Option<super::super::Devices::Geolocation::Geopoint>, kind: WaypointKind) -> ::windows::core::Result<EnhancedWaypoint>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IManeuverWarningImpl: Sized {
    fn Kind(&self) -> ::windows::core::Result<ManeuverWarningKind>;
    fn Severity(&self) -> ::windows::core::Result<ManeuverWarningSeverity>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IMapAddressImpl: Sized {
    fn BuildingName(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn BuildingFloor(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn BuildingRoom(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn BuildingWing(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn StreetNumber(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn Street(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn Neighborhood(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn District(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn Town(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn Region(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn RegionCode(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn Country(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn CountryCode(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn PostCode(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn Continent(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IMapAddress2Impl: Sized {
    fn FormattedAddress(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IMapLocationImpl: Sized {
    fn Point(&self) -> ::windows::core::Result<super::super::Devices::Geolocation::Geopoint>;
    fn DisplayName(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn Description(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn Address(&self) -> ::windows::core::Result<MapAddress>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IMapLocationFinderResultImpl: Sized {
    fn Locations(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<MapLocation>>;
    fn Status(&self) -> ::windows::core::Result<MapLocationFinderStatus>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IMapLocationFinderStaticsImpl: Sized {
    fn FindLocationsAtAsync(&self, querypoint: &::core::option::Option<super::super::Devices::Geolocation::Geopoint>) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<MapLocationFinderResult>>;
    fn FindLocationsAsync(&self, searchtext: &::windows::core::HSTRING, referencepoint: &::core::option::Option<super::super::Devices::Geolocation::Geopoint>) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<MapLocationFinderResult>>;
    fn FindLocationsWithMaxCountAsync(&self, searchtext: &::windows::core::HSTRING, referencepoint: &::core::option::Option<super::super::Devices::Geolocation::Geopoint>, maxcount: u32) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<MapLocationFinderResult>>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IMapLocationFinderStatics2Impl: Sized {
    fn FindLocationsAtWithAccuracyAsync(&self, querypoint: &::core::option::Option<super::super::Devices::Geolocation::Geopoint>, accuracy: MapLocationDesiredAccuracy) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<MapLocationFinderResult>>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IMapManagerStaticsImpl: Sized {
    fn ShowDownloadedMapsUI(&self) -> ::windows::core::Result<()>;
    fn ShowMapsUpdateUI(&self) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IMapRouteImpl: Sized {
    fn BoundingBox(&self) -> ::windows::core::Result<super::super::Devices::Geolocation::GeoboundingBox>;
    fn LengthInMeters(&self) -> ::windows::core::Result<f64>;
    fn EstimatedDuration(&self) -> ::windows::core::Result<super::super::Foundation::TimeSpan>;
    fn Path(&self) -> ::windows::core::Result<super::super::Devices::Geolocation::Geopath>;
    fn Legs(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<MapRouteLeg>>;
    fn IsTrafficBased(&self) -> ::windows::core::Result<bool>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IMapRoute2Impl: Sized {
    fn ViolatedRestrictions(&self) -> ::windows::core::Result<MapRouteRestrictions>;
    fn HasBlockedRoads(&self) -> ::windows::core::Result<bool>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IMapRoute3Impl: Sized {
    fn DurationWithoutTraffic(&self) -> ::windows::core::Result<super::super::Foundation::TimeSpan>;
    fn TrafficCongestion(&self) -> ::windows::core::Result<TrafficCongestion>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IMapRoute4Impl: Sized {
    fn IsScenic(&self) -> ::windows::core::Result<bool>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IMapRouteDrivingOptionsImpl: Sized {
    fn MaxAlternateRouteCount(&self) -> ::windows::core::Result<u32>;
    fn SetMaxAlternateRouteCount(&self, value: u32) -> ::windows::core::Result<()>;
    fn InitialHeading(&self) -> ::windows::core::Result<super::super::Foundation::IReference<f64>>;
    fn SetInitialHeading(&self, value: &::core::option::Option<super::super::Foundation::IReference<f64>>) -> ::windows::core::Result<()>;
    fn RouteOptimization(&self) -> ::windows::core::Result<MapRouteOptimization>;
    fn SetRouteOptimization(&self, value: MapRouteOptimization) -> ::windows::core::Result<()>;
    fn RouteRestrictions(&self) -> ::windows::core::Result<MapRouteRestrictions>;
    fn SetRouteRestrictions(&self, value: MapRouteRestrictions) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IMapRouteDrivingOptions2Impl: Sized {
    fn DepartureTime(&self) -> ::windows::core::Result<super::super::Foundation::IReference<super::super::Foundation::DateTime>>;
    fn SetDepartureTime(&self, value: &::core::option::Option<super::super::Foundation::IReference<super::super::Foundation::DateTime>>) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IMapRouteFinderResultImpl: Sized {
    fn Route(&self) -> ::windows::core::Result<MapRoute>;
    fn Status(&self) -> ::windows::core::Result<MapRouteFinderStatus>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IMapRouteFinderResult2Impl: Sized {
    fn AlternateRoutes(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<MapRoute>>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IMapRouteFinderStaticsImpl: Sized {
    fn GetDrivingRouteAsync(&self, startpoint: &::core::option::Option<super::super::Devices::Geolocation::Geopoint>, endpoint: &::core::option::Option<super::super::Devices::Geolocation::Geopoint>) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<MapRouteFinderResult>>;
    fn GetDrivingRouteWithOptimizationAsync(&self, startpoint: &::core::option::Option<super::super::Devices::Geolocation::Geopoint>, endpoint: &::core::option::Option<super::super::Devices::Geolocation::Geopoint>, optimization: MapRouteOptimization) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<MapRouteFinderResult>>;
    fn GetDrivingRouteWithOptimizationAndRestrictionsAsync(&self, startpoint: &::core::option::Option<super::super::Devices::Geolocation::Geopoint>, endpoint: &::core::option::Option<super::super::Devices::Geolocation::Geopoint>, optimization: MapRouteOptimization, restrictions: MapRouteRestrictions) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<MapRouteFinderResult>>;
    fn GetDrivingRouteWithOptimizationRestrictionsAndHeadingAsync(&self, startpoint: &::core::option::Option<super::super::Devices::Geolocation::Geopoint>, endpoint: &::core::option::Option<super::super::Devices::Geolocation::Geopoint>, optimization: MapRouteOptimization, restrictions: MapRouteRestrictions, headingindegrees: f64) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<MapRouteFinderResult>>;
    fn GetDrivingRouteFromWaypointsAsync(&self, waypoints: &::core::option::Option<super::super::Foundation::Collections::IIterable<super::super::Devices::Geolocation::Geopoint>>) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<MapRouteFinderResult>>;
    fn GetDrivingRouteFromWaypointsAndOptimizationAsync(&self, waypoints: &::core::option::Option<super::super::Foundation::Collections::IIterable<super::super::Devices::Geolocation::Geopoint>>, optimization: MapRouteOptimization) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<MapRouteFinderResult>>;
    fn GetDrivingRouteFromWaypointsOptimizationAndRestrictionsAsync(&self, waypoints: &::core::option::Option<super::super::Foundation::Collections::IIterable<super::super::Devices::Geolocation::Geopoint>>, optimization: MapRouteOptimization, restrictions: MapRouteRestrictions) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<MapRouteFinderResult>>;
    fn GetDrivingRouteFromWaypointsOptimizationRestrictionsAndHeadingAsync(&self, waypoints: &::core::option::Option<super::super::Foundation::Collections::IIterable<super::super::Devices::Geolocation::Geopoint>>, optimization: MapRouteOptimization, restrictions: MapRouteRestrictions, headingindegrees: f64) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<MapRouteFinderResult>>;
    fn GetWalkingRouteAsync(&self, startpoint: &::core::option::Option<super::super::Devices::Geolocation::Geopoint>, endpoint: &::core::option::Option<super::super::Devices::Geolocation::Geopoint>) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<MapRouteFinderResult>>;
    fn GetWalkingRouteFromWaypointsAsync(&self, waypoints: &::core::option::Option<super::super::Foundation::Collections::IIterable<super::super::Devices::Geolocation::Geopoint>>) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<MapRouteFinderResult>>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IMapRouteFinderStatics2Impl: Sized {
    fn GetDrivingRouteWithOptionsAsync(&self, startpoint: &::core::option::Option<super::super::Devices::Geolocation::Geopoint>, endpoint: &::core::option::Option<super::super::Devices::Geolocation::Geopoint>, options: &::core::option::Option<MapRouteDrivingOptions>) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<MapRouteFinderResult>>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IMapRouteFinderStatics3Impl: Sized {
    fn GetDrivingRouteFromEnhancedWaypointsAsync(&self, waypoints: &::core::option::Option<super::super::Foundation::Collections::IIterable<EnhancedWaypoint>>) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<MapRouteFinderResult>>;
    fn GetDrivingRouteFromEnhancedWaypointsWithOptionsAsync(&self, waypoints: &::core::option::Option<super::super::Foundation::Collections::IIterable<EnhancedWaypoint>>, options: &::core::option::Option<MapRouteDrivingOptions>) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<MapRouteFinderResult>>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IMapRouteLegImpl: Sized {
    fn BoundingBox(&self) -> ::windows::core::Result<super::super::Devices::Geolocation::GeoboundingBox>;
    fn Path(&self) -> ::windows::core::Result<super::super::Devices::Geolocation::Geopath>;
    fn LengthInMeters(&self) -> ::windows::core::Result<f64>;
    fn EstimatedDuration(&self) -> ::windows::core::Result<super::super::Foundation::TimeSpan>;
    fn Maneuvers(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<MapRouteManeuver>>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IMapRouteLeg2Impl: Sized {
    fn DurationWithoutTraffic(&self) -> ::windows::core::Result<super::super::Foundation::TimeSpan>;
    fn TrafficCongestion(&self) -> ::windows::core::Result<TrafficCongestion>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IMapRouteManeuverImpl: Sized {
    fn StartingPoint(&self) -> ::windows::core::Result<super::super::Devices::Geolocation::Geopoint>;
    fn LengthInMeters(&self) -> ::windows::core::Result<f64>;
    fn InstructionText(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn Kind(&self) -> ::windows::core::Result<MapRouteManeuverKind>;
    fn ExitNumber(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn ManeuverNotices(&self) -> ::windows::core::Result<MapManeuverNotices>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IMapRouteManeuver2Impl: Sized {
    fn StartHeading(&self) -> ::windows::core::Result<f64>;
    fn EndHeading(&self) -> ::windows::core::Result<f64>;
    fn StreetName(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IMapRouteManeuver3Impl: Sized {
    fn Warnings(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<ManeuverWarning>>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IMapServiceStaticsImpl: Sized {
    fn SetServiceToken(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn ServiceToken(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IMapServiceStatics2Impl: Sized {
    fn WorldViewRegionCode(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IMapServiceStatics3Impl: Sized {
    fn DataAttributions(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IMapServiceStatics4Impl: Sized {
    fn SetDataUsagePreference(&self, value: MapServiceDataUsagePreference) -> ::windows::core::Result<()>;
    fn DataUsagePreference(&self) -> ::windows::core::Result<MapServiceDataUsagePreference>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IPlaceInfoImpl: Sized {
    fn Show(&self, selection: &super::super::Foundation::Rect) -> ::windows::core::Result<()>;
    fn ShowWithPreferredPlacement(&self, selection: &super::super::Foundation::Rect, preferredplacement: super::super::UI::Popups::Placement) -> ::windows::core::Result<()>;
    fn Identifier(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn DisplayName(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn DisplayAddress(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn Geoshape(&self) -> ::windows::core::Result<super::super::Devices::Geolocation::IGeoshape>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IPlaceInfoCreateOptionsImpl: Sized {
    fn SetDisplayName(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn DisplayName(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetDisplayAddress(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn DisplayAddress(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IPlaceInfoStaticsImpl: Sized {
    fn Create(&self, referencepoint: &::core::option::Option<super::super::Devices::Geolocation::Geopoint>) -> ::windows::core::Result<PlaceInfo>;
    fn CreateWithGeopointAndOptions(&self, referencepoint: &::core::option::Option<super::super::Devices::Geolocation::Geopoint>, options: &::core::option::Option<PlaceInfoCreateOptions>) -> ::windows::core::Result<PlaceInfo>;
    fn CreateFromIdentifier(&self, identifier: &::windows::core::HSTRING) -> ::windows::core::Result<PlaceInfo>;
    fn CreateFromIdentifierWithOptions(&self, identifier: &::windows::core::HSTRING, defaultpoint: &::core::option::Option<super::super::Devices::Geolocation::Geopoint>, options: &::core::option::Option<PlaceInfoCreateOptions>) -> ::windows::core::Result<PlaceInfo>;
    fn CreateFromMapLocation(&self, location: &::core::option::Option<MapLocation>) -> ::windows::core::Result<PlaceInfo>;
    fn IsShowSupported(&self) -> ::windows::core::Result<bool>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IPlaceInfoStatics2Impl: Sized {
    fn CreateFromAddress(&self, displayaddress: &::windows::core::HSTRING) -> ::windows::core::Result<PlaceInfo>;
    fn CreateFromAddressWithName(&self, displayaddress: &::windows::core::HSTRING, displayname: &::windows::core::HSTRING) -> ::windows::core::Result<PlaceInfo>;
}

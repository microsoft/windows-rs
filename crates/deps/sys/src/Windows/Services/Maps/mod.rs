#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals)]
#[cfg(feature = "Services_Maps_Guidance")]
pub mod Guidance;
#[cfg(feature = "Services_Maps_LocalSearch")]
pub mod LocalSearch;
#[cfg(feature = "Services_Maps_OfflineMaps")]
pub mod OfflineMaps;
#[link(name = "windows")]
extern "system" {}
#[repr(transparent)]
pub struct EnhancedWaypoint(pub *mut ::core::ffi::c_void);
#[repr(C)]
pub struct GuidanceContract(i32);
#[repr(transparent)]
pub struct IEnhancedWaypoint(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IEnhancedWaypointFactory(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IManeuverWarning(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IMapAddress(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IMapAddress2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IMapLocation(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IMapLocationFinderResult(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IMapLocationFinderStatics(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IMapLocationFinderStatics2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IMapManagerStatics(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IMapRoute(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IMapRoute2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IMapRoute3(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IMapRoute4(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IMapRouteDrivingOptions(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IMapRouteDrivingOptions2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IMapRouteFinderResult(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IMapRouteFinderResult2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IMapRouteFinderStatics(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IMapRouteFinderStatics2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IMapRouteFinderStatics3(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IMapRouteLeg(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IMapRouteLeg2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IMapRouteManeuver(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IMapRouteManeuver2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IMapRouteManeuver3(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IMapServiceStatics(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IMapServiceStatics2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IMapServiceStatics3(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IMapServiceStatics4(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IPlaceInfo(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IPlaceInfoCreateOptions(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IPlaceInfoStatics(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IPlaceInfoStatics2(pub *mut ::core::ffi::c_void);
#[repr(C)]
pub struct LocalSearchContract(i32);
#[repr(transparent)]
pub struct ManeuverWarning(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ManeuverWarningKind(pub i32);
impl ManeuverWarningKind {
    pub const None: ManeuverWarningKind = ManeuverWarningKind(0i32);
    pub const Accident: ManeuverWarningKind = ManeuverWarningKind(1i32);
    pub const AdministrativeDivisionChange: ManeuverWarningKind = ManeuverWarningKind(2i32);
    pub const Alert: ManeuverWarningKind = ManeuverWarningKind(3i32);
    pub const BlockedRoad: ManeuverWarningKind = ManeuverWarningKind(4i32);
    pub const CheckTimetable: ManeuverWarningKind = ManeuverWarningKind(5i32);
    pub const Congestion: ManeuverWarningKind = ManeuverWarningKind(6i32);
    pub const Construction: ManeuverWarningKind = ManeuverWarningKind(7i32);
    pub const CountryChange: ManeuverWarningKind = ManeuverWarningKind(8i32);
    pub const DisabledVehicle: ManeuverWarningKind = ManeuverWarningKind(9i32);
    pub const GateAccess: ManeuverWarningKind = ManeuverWarningKind(10i32);
    pub const GetOffTransit: ManeuverWarningKind = ManeuverWarningKind(11i32);
    pub const GetOnTransit: ManeuverWarningKind = ManeuverWarningKind(12i32);
    pub const IllegalUTurn: ManeuverWarningKind = ManeuverWarningKind(13i32);
    pub const MassTransit: ManeuverWarningKind = ManeuverWarningKind(14i32);
    pub const Miscellaneous: ManeuverWarningKind = ManeuverWarningKind(15i32);
    pub const NoIncident: ManeuverWarningKind = ManeuverWarningKind(16i32);
    pub const Other: ManeuverWarningKind = ManeuverWarningKind(17i32);
    pub const OtherNews: ManeuverWarningKind = ManeuverWarningKind(18i32);
    pub const OtherTrafficIncidents: ManeuverWarningKind = ManeuverWarningKind(19i32);
    pub const PlannedEvent: ManeuverWarningKind = ManeuverWarningKind(20i32);
    pub const PrivateRoad: ManeuverWarningKind = ManeuverWarningKind(21i32);
    pub const RestrictedTurn: ManeuverWarningKind = ManeuverWarningKind(22i32);
    pub const RoadClosures: ManeuverWarningKind = ManeuverWarningKind(23i32);
    pub const RoadHazard: ManeuverWarningKind = ManeuverWarningKind(24i32);
    pub const ScheduledConstruction: ManeuverWarningKind = ManeuverWarningKind(25i32);
    pub const SeasonalClosures: ManeuverWarningKind = ManeuverWarningKind(26i32);
    pub const Tollbooth: ManeuverWarningKind = ManeuverWarningKind(27i32);
    pub const TollRoad: ManeuverWarningKind = ManeuverWarningKind(28i32);
    pub const TollZoneEnter: ManeuverWarningKind = ManeuverWarningKind(29i32);
    pub const TollZoneExit: ManeuverWarningKind = ManeuverWarningKind(30i32);
    pub const TrafficFlow: ManeuverWarningKind = ManeuverWarningKind(31i32);
    pub const TransitLineChange: ManeuverWarningKind = ManeuverWarningKind(32i32);
    pub const UnpavedRoad: ManeuverWarningKind = ManeuverWarningKind(33i32);
    pub const UnscheduledConstruction: ManeuverWarningKind = ManeuverWarningKind(34i32);
    pub const Weather: ManeuverWarningKind = ManeuverWarningKind(35i32);
}
#[repr(transparent)]
pub struct ManeuverWarningSeverity(pub i32);
impl ManeuverWarningSeverity {
    pub const None: ManeuverWarningSeverity = ManeuverWarningSeverity(0i32);
    pub const LowImpact: ManeuverWarningSeverity = ManeuverWarningSeverity(1i32);
    pub const Minor: ManeuverWarningSeverity = ManeuverWarningSeverity(2i32);
    pub const Moderate: ManeuverWarningSeverity = ManeuverWarningSeverity(3i32);
    pub const Serious: ManeuverWarningSeverity = ManeuverWarningSeverity(4i32);
}
#[repr(transparent)]
pub struct MapAddress(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct MapLocation(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct MapLocationDesiredAccuracy(pub i32);
impl MapLocationDesiredAccuracy {
    pub const High: MapLocationDesiredAccuracy = MapLocationDesiredAccuracy(0i32);
    pub const Low: MapLocationDesiredAccuracy = MapLocationDesiredAccuracy(1i32);
}
#[repr(transparent)]
pub struct MapLocationFinderResult(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct MapLocationFinderStatus(pub i32);
impl MapLocationFinderStatus {
    pub const Success: MapLocationFinderStatus = MapLocationFinderStatus(0i32);
    pub const UnknownError: MapLocationFinderStatus = MapLocationFinderStatus(1i32);
    pub const InvalidCredentials: MapLocationFinderStatus = MapLocationFinderStatus(2i32);
    pub const BadLocation: MapLocationFinderStatus = MapLocationFinderStatus(3i32);
    pub const IndexFailure: MapLocationFinderStatus = MapLocationFinderStatus(4i32);
    pub const NetworkFailure: MapLocationFinderStatus = MapLocationFinderStatus(5i32);
    pub const NotSupported: MapLocationFinderStatus = MapLocationFinderStatus(6i32);
}
#[repr(transparent)]
pub struct MapManeuverNotices(pub u32);
impl MapManeuverNotices {
    pub const None: MapManeuverNotices = MapManeuverNotices(0u32);
    pub const Toll: MapManeuverNotices = MapManeuverNotices(1u32);
    pub const Unpaved: MapManeuverNotices = MapManeuverNotices(2u32);
}
#[repr(transparent)]
pub struct MapRoute(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct MapRouteDrivingOptions(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct MapRouteFinderResult(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct MapRouteFinderStatus(pub i32);
impl MapRouteFinderStatus {
    pub const Success: MapRouteFinderStatus = MapRouteFinderStatus(0i32);
    pub const UnknownError: MapRouteFinderStatus = MapRouteFinderStatus(1i32);
    pub const InvalidCredentials: MapRouteFinderStatus = MapRouteFinderStatus(2i32);
    pub const NoRouteFound: MapRouteFinderStatus = MapRouteFinderStatus(3i32);
    pub const NoRouteFoundWithGivenOptions: MapRouteFinderStatus = MapRouteFinderStatus(4i32);
    pub const StartPointNotFound: MapRouteFinderStatus = MapRouteFinderStatus(5i32);
    pub const EndPointNotFound: MapRouteFinderStatus = MapRouteFinderStatus(6i32);
    pub const NoPedestrianRouteFound: MapRouteFinderStatus = MapRouteFinderStatus(7i32);
    pub const NetworkFailure: MapRouteFinderStatus = MapRouteFinderStatus(8i32);
    pub const NotSupported: MapRouteFinderStatus = MapRouteFinderStatus(9i32);
}
#[repr(transparent)]
pub struct MapRouteLeg(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct MapRouteManeuver(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct MapRouteManeuverKind(pub i32);
impl MapRouteManeuverKind {
    pub const None: MapRouteManeuverKind = MapRouteManeuverKind(0i32);
    pub const Start: MapRouteManeuverKind = MapRouteManeuverKind(1i32);
    pub const Stopover: MapRouteManeuverKind = MapRouteManeuverKind(2i32);
    pub const StopoverResume: MapRouteManeuverKind = MapRouteManeuverKind(3i32);
    pub const End: MapRouteManeuverKind = MapRouteManeuverKind(4i32);
    pub const GoStraight: MapRouteManeuverKind = MapRouteManeuverKind(5i32);
    pub const UTurnLeft: MapRouteManeuverKind = MapRouteManeuverKind(6i32);
    pub const UTurnRight: MapRouteManeuverKind = MapRouteManeuverKind(7i32);
    pub const TurnKeepLeft: MapRouteManeuverKind = MapRouteManeuverKind(8i32);
    pub const TurnKeepRight: MapRouteManeuverKind = MapRouteManeuverKind(9i32);
    pub const TurnLightLeft: MapRouteManeuverKind = MapRouteManeuverKind(10i32);
    pub const TurnLightRight: MapRouteManeuverKind = MapRouteManeuverKind(11i32);
    pub const TurnLeft: MapRouteManeuverKind = MapRouteManeuverKind(12i32);
    pub const TurnRight: MapRouteManeuverKind = MapRouteManeuverKind(13i32);
    pub const TurnHardLeft: MapRouteManeuverKind = MapRouteManeuverKind(14i32);
    pub const TurnHardRight: MapRouteManeuverKind = MapRouteManeuverKind(15i32);
    pub const FreewayEnterLeft: MapRouteManeuverKind = MapRouteManeuverKind(16i32);
    pub const FreewayEnterRight: MapRouteManeuverKind = MapRouteManeuverKind(17i32);
    pub const FreewayLeaveLeft: MapRouteManeuverKind = MapRouteManeuverKind(18i32);
    pub const FreewayLeaveRight: MapRouteManeuverKind = MapRouteManeuverKind(19i32);
    pub const FreewayContinueLeft: MapRouteManeuverKind = MapRouteManeuverKind(20i32);
    pub const FreewayContinueRight: MapRouteManeuverKind = MapRouteManeuverKind(21i32);
    pub const TrafficCircleLeft: MapRouteManeuverKind = MapRouteManeuverKind(22i32);
    pub const TrafficCircleRight: MapRouteManeuverKind = MapRouteManeuverKind(23i32);
    pub const TakeFerry: MapRouteManeuverKind = MapRouteManeuverKind(24i32);
}
#[repr(transparent)]
pub struct MapRouteOptimization(pub i32);
impl MapRouteOptimization {
    pub const Time: MapRouteOptimization = MapRouteOptimization(0i32);
    pub const Distance: MapRouteOptimization = MapRouteOptimization(1i32);
    pub const TimeWithTraffic: MapRouteOptimization = MapRouteOptimization(2i32);
    pub const Scenic: MapRouteOptimization = MapRouteOptimization(3i32);
}
#[repr(transparent)]
pub struct MapRouteRestrictions(pub u32);
impl MapRouteRestrictions {
    pub const None: MapRouteRestrictions = MapRouteRestrictions(0u32);
    pub const Highways: MapRouteRestrictions = MapRouteRestrictions(1u32);
    pub const TollRoads: MapRouteRestrictions = MapRouteRestrictions(2u32);
    pub const Ferries: MapRouteRestrictions = MapRouteRestrictions(4u32);
    pub const Tunnels: MapRouteRestrictions = MapRouteRestrictions(8u32);
    pub const DirtRoads: MapRouteRestrictions = MapRouteRestrictions(16u32);
    pub const Motorail: MapRouteRestrictions = MapRouteRestrictions(32u32);
}
#[repr(transparent)]
pub struct MapServiceDataUsagePreference(pub i32);
impl MapServiceDataUsagePreference {
    pub const Default: MapServiceDataUsagePreference = MapServiceDataUsagePreference(0i32);
    pub const OfflineMapDataOnly: MapServiceDataUsagePreference = MapServiceDataUsagePreference(1i32);
}
#[repr(transparent)]
pub struct PlaceInfo(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct PlaceInfoCreateOptions(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct TrafficCongestion(pub i32);
impl TrafficCongestion {
    pub const Unknown: TrafficCongestion = TrafficCongestion(0i32);
    pub const Light: TrafficCongestion = TrafficCongestion(1i32);
    pub const Mild: TrafficCongestion = TrafficCongestion(2i32);
    pub const Medium: TrafficCongestion = TrafficCongestion(3i32);
    pub const Heavy: TrafficCongestion = TrafficCongestion(4i32);
}
#[repr(transparent)]
pub struct WaypointKind(pub i32);
impl WaypointKind {
    pub const Stop: WaypointKind = WaypointKind(0i32);
    pub const Via: WaypointKind = WaypointKind(1i32);
}

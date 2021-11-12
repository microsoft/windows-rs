#![allow(non_snake_case, non_camel_case_types)]
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
pub struct LocalSearchContract(i32);
#[repr(transparent)]
pub struct ManeuverWarning(pub *mut ::core::ffi::c_void);
pub struct ManeuverWarningKind(i32);
pub struct ManeuverWarningSeverity(i32);
#[repr(transparent)]
pub struct MapAddress(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct MapLocation(pub *mut ::core::ffi::c_void);
pub struct MapLocationDesiredAccuracy(i32);
#[repr(transparent)]
pub struct MapLocationFinder(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct MapLocationFinderResult(pub *mut ::core::ffi::c_void);
pub struct MapLocationFinderStatus(i32);
#[repr(transparent)]
pub struct MapManager(pub *mut ::core::ffi::c_void);
pub struct MapManeuverNotices(i32);
#[repr(transparent)]
pub struct MapRoute(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct MapRouteDrivingOptions(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct MapRouteFinder(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct MapRouteFinderResult(pub *mut ::core::ffi::c_void);
pub struct MapRouteFinderStatus(i32);
#[repr(transparent)]
pub struct MapRouteLeg(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct MapRouteManeuver(pub *mut ::core::ffi::c_void);
pub struct MapRouteManeuverKind(i32);
pub struct MapRouteOptimization(i32);
pub struct MapRouteRestrictions(i32);
#[repr(transparent)]
pub struct MapService(pub *mut ::core::ffi::c_void);
pub struct MapServiceDataUsagePreference(i32);
#[repr(transparent)]
pub struct PlaceInfo(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct PlaceInfoCreateOptions(pub *mut ::core::ffi::c_void);
pub struct TrafficCongestion(i32);
pub struct WaypointKind(i32);

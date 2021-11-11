#![allow(non_snake_case, non_camel_case_types)]
#[cfg(feature = "Services_Maps_Guidance")]
pub mod Guidance;
#[cfg(feature = "Services_Maps_LocalSearch")]
pub mod LocalSearch;
#[cfg(feature = "Services_Maps_OfflineMaps")]
pub mod OfflineMaps;
#[link(name = "windows")]
extern "system" {
    fn EnhancedWaypoint();
    fn GuidanceContract();
    fn IEnhancedWaypoint();
    fn IEnhancedWaypointFactory();
    fn IManeuverWarning();
    fn IMapAddress();
    fn IMapAddress2();
    fn IMapLocation();
    fn IMapLocationFinderResult();
    fn IMapLocationFinderStatics();
    fn IMapLocationFinderStatics2();
    fn IMapManagerStatics();
    fn IMapRoute();
    fn IMapRoute2();
    fn IMapRoute3();
    fn IMapRoute4();
    fn IMapRouteDrivingOptions();
    fn IMapRouteDrivingOptions2();
    fn IMapRouteFinderResult();
    fn IMapRouteFinderResult2();
    fn IMapRouteFinderStatics();
    fn IMapRouteFinderStatics2();
    fn IMapRouteFinderStatics3();
    fn IMapRouteLeg();
    fn IMapRouteLeg2();
    fn IMapRouteManeuver();
    fn IMapRouteManeuver2();
    fn IMapRouteManeuver3();
    fn IMapServiceStatics();
    fn IMapServiceStatics2();
    fn IMapServiceStatics3();
    fn IMapServiceStatics4();
    fn IPlaceInfo();
    fn IPlaceInfoCreateOptions();
    fn IPlaceInfoStatics();
    fn IPlaceInfoStatics2();
    fn LocalSearchContract();
    fn ManeuverWarning();
    fn ManeuverWarningKind();
    fn ManeuverWarningSeverity();
    fn MapAddress();
    fn MapLocation();
    fn MapLocationDesiredAccuracy();
    fn MapLocationFinder();
    fn MapLocationFinderResult();
    fn MapLocationFinderStatus();
    fn MapManager();
    fn MapManeuverNotices();
    fn MapRoute();
    fn MapRouteDrivingOptions();
    fn MapRouteFinder();
    fn MapRouteFinderResult();
    fn MapRouteFinderStatus();
    fn MapRouteLeg();
    fn MapRouteManeuver();
    fn MapRouteManeuverKind();
    fn MapRouteOptimization();
    fn MapRouteRestrictions();
    fn MapService();
    fn MapServiceDataUsagePreference();
    fn PlaceInfo();
    fn PlaceInfoCreateOptions();
    fn TrafficCongestion();
    fn WaypointKind();
}

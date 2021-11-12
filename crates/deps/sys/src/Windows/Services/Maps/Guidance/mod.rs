#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals)]
#[link(name = "windows")]
extern "system" {}
#[repr(transparent)]
pub struct GuidanceAudioMeasurementSystem(pub i32);
impl GuidanceAudioMeasurementSystem {
    pub const Meters: GuidanceAudioMeasurementSystem = GuidanceAudioMeasurementSystem(0i32);
    pub const MilesAndYards: GuidanceAudioMeasurementSystem = GuidanceAudioMeasurementSystem(1i32);
    pub const MilesAndFeet: GuidanceAudioMeasurementSystem = GuidanceAudioMeasurementSystem(2i32);
}
#[repr(transparent)]
pub struct GuidanceAudioNotificationKind(pub i32);
impl GuidanceAudioNotificationKind {
    pub const Maneuver: GuidanceAudioNotificationKind = GuidanceAudioNotificationKind(0i32);
    pub const Route: GuidanceAudioNotificationKind = GuidanceAudioNotificationKind(1i32);
    pub const Gps: GuidanceAudioNotificationKind = GuidanceAudioNotificationKind(2i32);
    pub const SpeedLimit: GuidanceAudioNotificationKind = GuidanceAudioNotificationKind(3i32);
    pub const Traffic: GuidanceAudioNotificationKind = GuidanceAudioNotificationKind(4i32);
    pub const TrafficCamera: GuidanceAudioNotificationKind = GuidanceAudioNotificationKind(5i32);
}
#[repr(transparent)]
pub struct GuidanceAudioNotificationRequestedEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct GuidanceAudioNotifications(pub u32);
impl GuidanceAudioNotifications {
    pub const None: GuidanceAudioNotifications = GuidanceAudioNotifications(0u32);
    pub const Maneuver: GuidanceAudioNotifications = GuidanceAudioNotifications(1u32);
    pub const Route: GuidanceAudioNotifications = GuidanceAudioNotifications(2u32);
    pub const Gps: GuidanceAudioNotifications = GuidanceAudioNotifications(4u32);
    pub const SpeedLimit: GuidanceAudioNotifications = GuidanceAudioNotifications(8u32);
    pub const Traffic: GuidanceAudioNotifications = GuidanceAudioNotifications(16u32);
    pub const TrafficCamera: GuidanceAudioNotifications = GuidanceAudioNotifications(32u32);
}
#[repr(transparent)]
pub struct GuidanceLaneInfo(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct GuidanceLaneMarkers(pub u32);
impl GuidanceLaneMarkers {
    pub const None: GuidanceLaneMarkers = GuidanceLaneMarkers(0u32);
    pub const LightRight: GuidanceLaneMarkers = GuidanceLaneMarkers(1u32);
    pub const Right: GuidanceLaneMarkers = GuidanceLaneMarkers(2u32);
    pub const HardRight: GuidanceLaneMarkers = GuidanceLaneMarkers(4u32);
    pub const Straight: GuidanceLaneMarkers = GuidanceLaneMarkers(8u32);
    pub const UTurnLeft: GuidanceLaneMarkers = GuidanceLaneMarkers(16u32);
    pub const HardLeft: GuidanceLaneMarkers = GuidanceLaneMarkers(32u32);
    pub const Left: GuidanceLaneMarkers = GuidanceLaneMarkers(64u32);
    pub const LightLeft: GuidanceLaneMarkers = GuidanceLaneMarkers(128u32);
    pub const UTurnRight: GuidanceLaneMarkers = GuidanceLaneMarkers(256u32);
    pub const Unknown: GuidanceLaneMarkers = GuidanceLaneMarkers(4294967295u32);
}
#[repr(transparent)]
pub struct GuidanceManeuver(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct GuidanceManeuverKind(pub i32);
impl GuidanceManeuverKind {
    pub const None: GuidanceManeuverKind = GuidanceManeuverKind(0i32);
    pub const GoStraight: GuidanceManeuverKind = GuidanceManeuverKind(1i32);
    pub const UTurnRight: GuidanceManeuverKind = GuidanceManeuverKind(2i32);
    pub const UTurnLeft: GuidanceManeuverKind = GuidanceManeuverKind(3i32);
    pub const TurnKeepRight: GuidanceManeuverKind = GuidanceManeuverKind(4i32);
    pub const TurnLightRight: GuidanceManeuverKind = GuidanceManeuverKind(5i32);
    pub const TurnRight: GuidanceManeuverKind = GuidanceManeuverKind(6i32);
    pub const TurnHardRight: GuidanceManeuverKind = GuidanceManeuverKind(7i32);
    pub const KeepMiddle: GuidanceManeuverKind = GuidanceManeuverKind(8i32);
    pub const TurnKeepLeft: GuidanceManeuverKind = GuidanceManeuverKind(9i32);
    pub const TurnLightLeft: GuidanceManeuverKind = GuidanceManeuverKind(10i32);
    pub const TurnLeft: GuidanceManeuverKind = GuidanceManeuverKind(11i32);
    pub const TurnHardLeft: GuidanceManeuverKind = GuidanceManeuverKind(12i32);
    pub const FreewayEnterRight: GuidanceManeuverKind = GuidanceManeuverKind(13i32);
    pub const FreewayEnterLeft: GuidanceManeuverKind = GuidanceManeuverKind(14i32);
    pub const FreewayLeaveRight: GuidanceManeuverKind = GuidanceManeuverKind(15i32);
    pub const FreewayLeaveLeft: GuidanceManeuverKind = GuidanceManeuverKind(16i32);
    pub const FreewayKeepRight: GuidanceManeuverKind = GuidanceManeuverKind(17i32);
    pub const FreewayKeepLeft: GuidanceManeuverKind = GuidanceManeuverKind(18i32);
    pub const TrafficCircleRight1: GuidanceManeuverKind = GuidanceManeuverKind(19i32);
    pub const TrafficCircleRight2: GuidanceManeuverKind = GuidanceManeuverKind(20i32);
    pub const TrafficCircleRight3: GuidanceManeuverKind = GuidanceManeuverKind(21i32);
    pub const TrafficCircleRight4: GuidanceManeuverKind = GuidanceManeuverKind(22i32);
    pub const TrafficCircleRight5: GuidanceManeuverKind = GuidanceManeuverKind(23i32);
    pub const TrafficCircleRight6: GuidanceManeuverKind = GuidanceManeuverKind(24i32);
    pub const TrafficCircleRight7: GuidanceManeuverKind = GuidanceManeuverKind(25i32);
    pub const TrafficCircleRight8: GuidanceManeuverKind = GuidanceManeuverKind(26i32);
    pub const TrafficCircleRight9: GuidanceManeuverKind = GuidanceManeuverKind(27i32);
    pub const TrafficCircleRight10: GuidanceManeuverKind = GuidanceManeuverKind(28i32);
    pub const TrafficCircleRight11: GuidanceManeuverKind = GuidanceManeuverKind(29i32);
    pub const TrafficCircleRight12: GuidanceManeuverKind = GuidanceManeuverKind(30i32);
    pub const TrafficCircleLeft1: GuidanceManeuverKind = GuidanceManeuverKind(31i32);
    pub const TrafficCircleLeft2: GuidanceManeuverKind = GuidanceManeuverKind(32i32);
    pub const TrafficCircleLeft3: GuidanceManeuverKind = GuidanceManeuverKind(33i32);
    pub const TrafficCircleLeft4: GuidanceManeuverKind = GuidanceManeuverKind(34i32);
    pub const TrafficCircleLeft5: GuidanceManeuverKind = GuidanceManeuverKind(35i32);
    pub const TrafficCircleLeft6: GuidanceManeuverKind = GuidanceManeuverKind(36i32);
    pub const TrafficCircleLeft7: GuidanceManeuverKind = GuidanceManeuverKind(37i32);
    pub const TrafficCircleLeft8: GuidanceManeuverKind = GuidanceManeuverKind(38i32);
    pub const TrafficCircleLeft9: GuidanceManeuverKind = GuidanceManeuverKind(39i32);
    pub const TrafficCircleLeft10: GuidanceManeuverKind = GuidanceManeuverKind(40i32);
    pub const TrafficCircleLeft11: GuidanceManeuverKind = GuidanceManeuverKind(41i32);
    pub const TrafficCircleLeft12: GuidanceManeuverKind = GuidanceManeuverKind(42i32);
    pub const Start: GuidanceManeuverKind = GuidanceManeuverKind(43i32);
    pub const End: GuidanceManeuverKind = GuidanceManeuverKind(44i32);
    pub const TakeFerry: GuidanceManeuverKind = GuidanceManeuverKind(45i32);
    pub const PassTransitStation: GuidanceManeuverKind = GuidanceManeuverKind(46i32);
    pub const LeaveTransitStation: GuidanceManeuverKind = GuidanceManeuverKind(47i32);
}
#[repr(transparent)]
pub struct GuidanceMapMatchedCoordinate(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct GuidanceMode(pub i32);
impl GuidanceMode {
    pub const None: GuidanceMode = GuidanceMode(0i32);
    pub const Simulation: GuidanceMode = GuidanceMode(1i32);
    pub const Navigation: GuidanceMode = GuidanceMode(2i32);
    pub const Tracking: GuidanceMode = GuidanceMode(3i32);
}
#[repr(transparent)]
pub struct GuidanceNavigator(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct GuidanceReroutedEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct GuidanceRoadSegment(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct GuidanceRoadSignpost(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct GuidanceRoute(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct GuidanceTelemetryCollector(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct GuidanceUpdatedEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IGuidanceAudioNotificationRequestedEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IGuidanceLaneInfo(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IGuidanceManeuver(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IGuidanceMapMatchedCoordinate(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IGuidanceNavigator(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IGuidanceNavigator2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IGuidanceNavigatorStatics(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IGuidanceNavigatorStatics2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IGuidanceReroutedEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IGuidanceRoadSegment(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IGuidanceRoadSegment2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IGuidanceRoadSignpost(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IGuidanceRoute(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IGuidanceRouteStatics(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IGuidanceTelemetryCollector(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IGuidanceTelemetryCollectorStatics(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IGuidanceUpdatedEventArgs(pub *mut ::core::ffi::c_void);

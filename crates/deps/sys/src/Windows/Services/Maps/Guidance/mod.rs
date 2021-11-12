#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals)]
#[link(name = "windows")]
extern "system" {}
#[repr(transparent)]
pub struct GuidanceAudioMeasurementSystem(pub i32);
impl GuidanceAudioMeasurementSystem {
    pub const Meters: Self = Self(0i32);
    pub const MilesAndYards: Self = Self(1i32);
    pub const MilesAndFeet: Self = Self(2i32);
}
impl ::core::marker::Copy for GuidanceAudioMeasurementSystem {}
impl ::core::clone::Clone for GuidanceAudioMeasurementSystem {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct GuidanceAudioNotificationKind(pub i32);
impl GuidanceAudioNotificationKind {
    pub const Maneuver: Self = Self(0i32);
    pub const Route: Self = Self(1i32);
    pub const Gps: Self = Self(2i32);
    pub const SpeedLimit: Self = Self(3i32);
    pub const Traffic: Self = Self(4i32);
    pub const TrafficCamera: Self = Self(5i32);
}
impl ::core::marker::Copy for GuidanceAudioNotificationKind {}
impl ::core::clone::Clone for GuidanceAudioNotificationKind {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct GuidanceAudioNotificationRequestedEventArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for GuidanceAudioNotificationRequestedEventArgs {}
impl ::core::clone::Clone for GuidanceAudioNotificationRequestedEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct GuidanceAudioNotifications(pub u32);
impl GuidanceAudioNotifications {
    pub const None: Self = Self(0u32);
    pub const Maneuver: Self = Self(1u32);
    pub const Route: Self = Self(2u32);
    pub const Gps: Self = Self(4u32);
    pub const SpeedLimit: Self = Self(8u32);
    pub const Traffic: Self = Self(16u32);
    pub const TrafficCamera: Self = Self(32u32);
}
impl ::core::marker::Copy for GuidanceAudioNotifications {}
impl ::core::clone::Clone for GuidanceAudioNotifications {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct GuidanceLaneInfo(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for GuidanceLaneInfo {}
impl ::core::clone::Clone for GuidanceLaneInfo {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct GuidanceLaneMarkers(pub u32);
impl GuidanceLaneMarkers {
    pub const None: Self = Self(0u32);
    pub const LightRight: Self = Self(1u32);
    pub const Right: Self = Self(2u32);
    pub const HardRight: Self = Self(4u32);
    pub const Straight: Self = Self(8u32);
    pub const UTurnLeft: Self = Self(16u32);
    pub const HardLeft: Self = Self(32u32);
    pub const Left: Self = Self(64u32);
    pub const LightLeft: Self = Self(128u32);
    pub const UTurnRight: Self = Self(256u32);
    pub const Unknown: Self = Self(4294967295u32);
}
impl ::core::marker::Copy for GuidanceLaneMarkers {}
impl ::core::clone::Clone for GuidanceLaneMarkers {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct GuidanceManeuver(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for GuidanceManeuver {}
impl ::core::clone::Clone for GuidanceManeuver {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct GuidanceManeuverKind(pub i32);
impl GuidanceManeuverKind {
    pub const None: Self = Self(0i32);
    pub const GoStraight: Self = Self(1i32);
    pub const UTurnRight: Self = Self(2i32);
    pub const UTurnLeft: Self = Self(3i32);
    pub const TurnKeepRight: Self = Self(4i32);
    pub const TurnLightRight: Self = Self(5i32);
    pub const TurnRight: Self = Self(6i32);
    pub const TurnHardRight: Self = Self(7i32);
    pub const KeepMiddle: Self = Self(8i32);
    pub const TurnKeepLeft: Self = Self(9i32);
    pub const TurnLightLeft: Self = Self(10i32);
    pub const TurnLeft: Self = Self(11i32);
    pub const TurnHardLeft: Self = Self(12i32);
    pub const FreewayEnterRight: Self = Self(13i32);
    pub const FreewayEnterLeft: Self = Self(14i32);
    pub const FreewayLeaveRight: Self = Self(15i32);
    pub const FreewayLeaveLeft: Self = Self(16i32);
    pub const FreewayKeepRight: Self = Self(17i32);
    pub const FreewayKeepLeft: Self = Self(18i32);
    pub const TrafficCircleRight1: Self = Self(19i32);
    pub const TrafficCircleRight2: Self = Self(20i32);
    pub const TrafficCircleRight3: Self = Self(21i32);
    pub const TrafficCircleRight4: Self = Self(22i32);
    pub const TrafficCircleRight5: Self = Self(23i32);
    pub const TrafficCircleRight6: Self = Self(24i32);
    pub const TrafficCircleRight7: Self = Self(25i32);
    pub const TrafficCircleRight8: Self = Self(26i32);
    pub const TrafficCircleRight9: Self = Self(27i32);
    pub const TrafficCircleRight10: Self = Self(28i32);
    pub const TrafficCircleRight11: Self = Self(29i32);
    pub const TrafficCircleRight12: Self = Self(30i32);
    pub const TrafficCircleLeft1: Self = Self(31i32);
    pub const TrafficCircleLeft2: Self = Self(32i32);
    pub const TrafficCircleLeft3: Self = Self(33i32);
    pub const TrafficCircleLeft4: Self = Self(34i32);
    pub const TrafficCircleLeft5: Self = Self(35i32);
    pub const TrafficCircleLeft6: Self = Self(36i32);
    pub const TrafficCircleLeft7: Self = Self(37i32);
    pub const TrafficCircleLeft8: Self = Self(38i32);
    pub const TrafficCircleLeft9: Self = Self(39i32);
    pub const TrafficCircleLeft10: Self = Self(40i32);
    pub const TrafficCircleLeft11: Self = Self(41i32);
    pub const TrafficCircleLeft12: Self = Self(42i32);
    pub const Start: Self = Self(43i32);
    pub const End: Self = Self(44i32);
    pub const TakeFerry: Self = Self(45i32);
    pub const PassTransitStation: Self = Self(46i32);
    pub const LeaveTransitStation: Self = Self(47i32);
}
impl ::core::marker::Copy for GuidanceManeuverKind {}
impl ::core::clone::Clone for GuidanceManeuverKind {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct GuidanceMapMatchedCoordinate(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for GuidanceMapMatchedCoordinate {}
impl ::core::clone::Clone for GuidanceMapMatchedCoordinate {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct GuidanceMode(pub i32);
impl GuidanceMode {
    pub const None: Self = Self(0i32);
    pub const Simulation: Self = Self(1i32);
    pub const Navigation: Self = Self(2i32);
    pub const Tracking: Self = Self(3i32);
}
impl ::core::marker::Copy for GuidanceMode {}
impl ::core::clone::Clone for GuidanceMode {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct GuidanceNavigator(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for GuidanceNavigator {}
impl ::core::clone::Clone for GuidanceNavigator {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct GuidanceReroutedEventArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for GuidanceReroutedEventArgs {}
impl ::core::clone::Clone for GuidanceReroutedEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct GuidanceRoadSegment(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for GuidanceRoadSegment {}
impl ::core::clone::Clone for GuidanceRoadSegment {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct GuidanceRoadSignpost(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for GuidanceRoadSignpost {}
impl ::core::clone::Clone for GuidanceRoadSignpost {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct GuidanceRoute(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for GuidanceRoute {}
impl ::core::clone::Clone for GuidanceRoute {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct GuidanceTelemetryCollector(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for GuidanceTelemetryCollector {}
impl ::core::clone::Clone for GuidanceTelemetryCollector {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct GuidanceUpdatedEventArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for GuidanceUpdatedEventArgs {}
impl ::core::clone::Clone for GuidanceUpdatedEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IGuidanceAudioNotificationRequestedEventArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IGuidanceAudioNotificationRequestedEventArgs {}
impl ::core::clone::Clone for IGuidanceAudioNotificationRequestedEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IGuidanceLaneInfo(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IGuidanceLaneInfo {}
impl ::core::clone::Clone for IGuidanceLaneInfo {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IGuidanceManeuver(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IGuidanceManeuver {}
impl ::core::clone::Clone for IGuidanceManeuver {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IGuidanceMapMatchedCoordinate(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IGuidanceMapMatchedCoordinate {}
impl ::core::clone::Clone for IGuidanceMapMatchedCoordinate {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IGuidanceNavigator(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IGuidanceNavigator {}
impl ::core::clone::Clone for IGuidanceNavigator {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IGuidanceNavigator2(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IGuidanceNavigator2 {}
impl ::core::clone::Clone for IGuidanceNavigator2 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IGuidanceNavigatorStatics(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IGuidanceNavigatorStatics {}
impl ::core::clone::Clone for IGuidanceNavigatorStatics {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IGuidanceNavigatorStatics2(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IGuidanceNavigatorStatics2 {}
impl ::core::clone::Clone for IGuidanceNavigatorStatics2 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IGuidanceReroutedEventArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IGuidanceReroutedEventArgs {}
impl ::core::clone::Clone for IGuidanceReroutedEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IGuidanceRoadSegment(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IGuidanceRoadSegment {}
impl ::core::clone::Clone for IGuidanceRoadSegment {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IGuidanceRoadSegment2(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IGuidanceRoadSegment2 {}
impl ::core::clone::Clone for IGuidanceRoadSegment2 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IGuidanceRoadSignpost(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IGuidanceRoadSignpost {}
impl ::core::clone::Clone for IGuidanceRoadSignpost {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IGuidanceRoute(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IGuidanceRoute {}
impl ::core::clone::Clone for IGuidanceRoute {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IGuidanceRouteStatics(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IGuidanceRouteStatics {}
impl ::core::clone::Clone for IGuidanceRouteStatics {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IGuidanceTelemetryCollector(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IGuidanceTelemetryCollector {}
impl ::core::clone::Clone for IGuidanceTelemetryCollector {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IGuidanceTelemetryCollectorStatics(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IGuidanceTelemetryCollectorStatics {}
impl ::core::clone::Clone for IGuidanceTelemetryCollectorStatics {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IGuidanceUpdatedEventArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IGuidanceUpdatedEventArgs {}
impl ::core::clone::Clone for IGuidanceUpdatedEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}

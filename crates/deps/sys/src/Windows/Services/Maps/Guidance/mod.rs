#![allow(non_snake_case, non_camel_case_types)]
#[link(name = "windows")]
extern "system" {}
pub struct GuidanceAudioMeasurementSystem(i32);
pub struct GuidanceAudioNotificationKind(i32);
pub struct GuidanceAudioNotificationRequestedEventArgs(i32);
pub struct GuidanceAudioNotifications(i32);
pub struct GuidanceLaneInfo(i32);
pub struct GuidanceLaneMarkers(i32);
pub struct GuidanceManeuver(i32);
pub struct GuidanceManeuverKind(i32);
pub struct GuidanceMapMatchedCoordinate(i32);
pub struct GuidanceMode(i32);
pub struct GuidanceNavigator(i32);
pub struct GuidanceReroutedEventArgs(i32);
pub struct GuidanceRoadSegment(i32);
pub struct GuidanceRoadSignpost(i32);
pub struct GuidanceRoute(i32);
pub struct GuidanceTelemetryCollector(i32);
pub struct GuidanceUpdatedEventArgs(i32);
pub struct IGuidanceAudioNotificationRequestedEventArgs(pub *mut ::core::ffi::c_void);
pub struct IGuidanceLaneInfo(pub *mut ::core::ffi::c_void);
pub struct IGuidanceManeuver(pub *mut ::core::ffi::c_void);
pub struct IGuidanceMapMatchedCoordinate(pub *mut ::core::ffi::c_void);
pub struct IGuidanceNavigator(pub *mut ::core::ffi::c_void);
pub struct IGuidanceNavigator2(pub *mut ::core::ffi::c_void);
pub struct IGuidanceNavigatorStatics(pub *mut ::core::ffi::c_void);
pub struct IGuidanceNavigatorStatics2(pub *mut ::core::ffi::c_void);
pub struct IGuidanceReroutedEventArgs(pub *mut ::core::ffi::c_void);
pub struct IGuidanceRoadSegment(pub *mut ::core::ffi::c_void);
pub struct IGuidanceRoadSegment2(pub *mut ::core::ffi::c_void);
pub struct IGuidanceRoadSignpost(pub *mut ::core::ffi::c_void);
pub struct IGuidanceRoute(pub *mut ::core::ffi::c_void);
pub struct IGuidanceRouteStatics(pub *mut ::core::ffi::c_void);
pub struct IGuidanceTelemetryCollector(pub *mut ::core::ffi::c_void);
pub struct IGuidanceTelemetryCollectorStatics(pub *mut ::core::ffi::c_void);
pub struct IGuidanceUpdatedEventArgs(pub *mut ::core::ffi::c_void);

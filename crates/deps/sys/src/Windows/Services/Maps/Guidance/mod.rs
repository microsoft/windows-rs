#![allow(non_snake_case, non_camel_case_types)]
#[link(name = "windows")]
extern "system" {}
pub struct GuidanceAudioMeasurementSystem(i32);
pub struct GuidanceAudioNotificationKind(i32);
#[repr(transparent)]
pub struct GuidanceAudioNotificationRequestedEventArgs(pub *mut ::core::ffi::c_void);
pub struct GuidanceAudioNotifications(i32);
#[repr(transparent)]
pub struct GuidanceLaneInfo(pub *mut ::core::ffi::c_void);
pub struct GuidanceLaneMarkers(i32);
#[repr(transparent)]
pub struct GuidanceManeuver(pub *mut ::core::ffi::c_void);
pub struct GuidanceManeuverKind(i32);
#[repr(transparent)]
pub struct GuidanceMapMatchedCoordinate(pub *mut ::core::ffi::c_void);
pub struct GuidanceMode(i32);
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

#![allow(non_snake_case, non_camel_case_types)]
#[link(name = "windows")]
extern "system" {}
pub struct CurrentSessionChangedEventArgs(i32);
pub struct GlobalSystemMediaTransportControlsSession(i32);
pub struct GlobalSystemMediaTransportControlsSessionManager(i32);
pub struct GlobalSystemMediaTransportControlsSessionMediaProperties(i32);
pub struct GlobalSystemMediaTransportControlsSessionPlaybackControls(i32);
pub struct GlobalSystemMediaTransportControlsSessionPlaybackInfo(i32);
pub struct GlobalSystemMediaTransportControlsSessionPlaybackStatus(i32);
pub struct GlobalSystemMediaTransportControlsSessionTimelineProperties(i32);
pub struct ICurrentSessionChangedEventArgs(pub *mut ::core::ffi::c_void);
pub struct IGlobalSystemMediaTransportControlsSession(pub *mut ::core::ffi::c_void);
pub struct IGlobalSystemMediaTransportControlsSessionManager(pub *mut ::core::ffi::c_void);
pub struct IGlobalSystemMediaTransportControlsSessionManagerStatics(pub *mut ::core::ffi::c_void);
pub struct IGlobalSystemMediaTransportControlsSessionMediaProperties(pub *mut ::core::ffi::c_void);
pub struct IGlobalSystemMediaTransportControlsSessionPlaybackControls(pub *mut ::core::ffi::c_void);
pub struct IGlobalSystemMediaTransportControlsSessionPlaybackInfo(pub *mut ::core::ffi::c_void);
pub struct IGlobalSystemMediaTransportControlsSessionTimelineProperties(pub *mut ::core::ffi::c_void);
pub struct IMediaPropertiesChangedEventArgs(pub *mut ::core::ffi::c_void);
pub struct IPlaybackInfoChangedEventArgs(pub *mut ::core::ffi::c_void);
pub struct ISessionsChangedEventArgs(pub *mut ::core::ffi::c_void);
pub struct ITimelinePropertiesChangedEventArgs(pub *mut ::core::ffi::c_void);
pub struct MediaPropertiesChangedEventArgs(i32);
pub struct PlaybackInfoChangedEventArgs(i32);
pub struct SessionsChangedEventArgs(i32);
pub struct TimelinePropertiesChangedEventArgs(i32);

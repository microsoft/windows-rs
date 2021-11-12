#![allow(non_snake_case, non_camel_case_types)]
#[link(name = "windows")]
extern "system" {}
#[repr(transparent)]
pub struct CurrentSessionChangedEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct GlobalSystemMediaTransportControlsSession(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct GlobalSystemMediaTransportControlsSessionManager(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct GlobalSystemMediaTransportControlsSessionMediaProperties(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct GlobalSystemMediaTransportControlsSessionPlaybackControls(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct GlobalSystemMediaTransportControlsSessionPlaybackInfo(pub *mut ::core::ffi::c_void);
#[repr(C)]
pub struct GlobalSystemMediaTransportControlsSessionPlaybackStatus(i32);
#[repr(transparent)]
pub struct GlobalSystemMediaTransportControlsSessionTimelineProperties(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ICurrentSessionChangedEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IGlobalSystemMediaTransportControlsSession(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IGlobalSystemMediaTransportControlsSessionManager(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IGlobalSystemMediaTransportControlsSessionManagerStatics(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IGlobalSystemMediaTransportControlsSessionMediaProperties(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IGlobalSystemMediaTransportControlsSessionPlaybackControls(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IGlobalSystemMediaTransportControlsSessionPlaybackInfo(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IGlobalSystemMediaTransportControlsSessionTimelineProperties(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IMediaPropertiesChangedEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IPlaybackInfoChangedEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ISessionsChangedEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ITimelinePropertiesChangedEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct MediaPropertiesChangedEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct PlaybackInfoChangedEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct SessionsChangedEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct TimelinePropertiesChangedEventArgs(pub *mut ::core::ffi::c_void);

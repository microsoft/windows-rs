#![allow(non_snake_case, non_camel_case_types)]
#[link(name = "windows")]
extern "system" {}
pub struct CurrentTimeChangeRequestedEventArgs(i32);
pub struct ICurrentTimeChangeRequestedEventArgs(pub *mut ::core::ffi::c_void);
pub struct IMuteChangeRequestedEventArgs(pub *mut ::core::ffi::c_void);
pub struct IPlayToConnection(pub *mut ::core::ffi::c_void);
pub struct IPlayToConnectionErrorEventArgs(pub *mut ::core::ffi::c_void);
pub struct IPlayToConnectionStateChangedEventArgs(pub *mut ::core::ffi::c_void);
pub struct IPlayToConnectionTransferredEventArgs(pub *mut ::core::ffi::c_void);
pub struct IPlayToManager(pub *mut ::core::ffi::c_void);
pub struct IPlayToManagerStatics(pub *mut ::core::ffi::c_void);
pub struct IPlayToReceiver(pub *mut ::core::ffi::c_void);
pub struct IPlayToSource(pub *mut ::core::ffi::c_void);
pub struct IPlayToSourceDeferral(pub *mut ::core::ffi::c_void);
pub struct IPlayToSourceRequest(pub *mut ::core::ffi::c_void);
pub struct IPlayToSourceRequestedEventArgs(pub *mut ::core::ffi::c_void);
pub struct IPlayToSourceSelectedEventArgs(pub *mut ::core::ffi::c_void);
pub struct IPlayToSourceWithPreferredSourceUri(pub *mut ::core::ffi::c_void);
pub struct IPlaybackRateChangeRequestedEventArgs(pub *mut ::core::ffi::c_void);
pub struct ISourceChangeRequestedEventArgs(pub *mut ::core::ffi::c_void);
pub struct IVolumeChangeRequestedEventArgs(pub *mut ::core::ffi::c_void);
pub struct MuteChangeRequestedEventArgs(i32);
pub struct PlayToConnection(i32);
pub struct PlayToConnectionError(i32);
pub struct PlayToConnectionErrorEventArgs(i32);
pub struct PlayToConnectionState(i32);
pub struct PlayToConnectionStateChangedEventArgs(i32);
pub struct PlayToConnectionTransferredEventArgs(i32);
pub struct PlayToManager(i32);
pub struct PlayToReceiver(i32);
pub struct PlayToSource(i32);
pub struct PlayToSourceDeferral(i32);
pub struct PlayToSourceRequest(i32);
pub struct PlayToSourceRequestedEventArgs(i32);
pub struct PlayToSourceSelectedEventArgs(i32);
pub struct PlaybackRateChangeRequestedEventArgs(i32);
pub struct SourceChangeRequestedEventArgs(i32);
pub struct VolumeChangeRequestedEventArgs(i32);

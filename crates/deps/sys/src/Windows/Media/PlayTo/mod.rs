#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals)]
#[link(name = "windows")]
extern "system" {}
#[repr(transparent)]
pub struct CurrentTimeChangeRequestedEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ICurrentTimeChangeRequestedEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IMuteChangeRequestedEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IPlayToConnection(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IPlayToConnectionErrorEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IPlayToConnectionStateChangedEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IPlayToConnectionTransferredEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IPlayToManager(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IPlayToManagerStatics(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IPlayToReceiver(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IPlayToSource(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IPlayToSourceDeferral(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IPlayToSourceRequest(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IPlayToSourceRequestedEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IPlayToSourceSelectedEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IPlayToSourceWithPreferredSourceUri(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IPlaybackRateChangeRequestedEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ISourceChangeRequestedEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IVolumeChangeRequestedEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct MuteChangeRequestedEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct PlayToConnection(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct PlayToConnectionError(pub i32);
impl PlayToConnectionError {
    pub const None: PlayToConnectionError = PlayToConnectionError(0i32);
    pub const DeviceNotResponding: PlayToConnectionError = PlayToConnectionError(1i32);
    pub const DeviceError: PlayToConnectionError = PlayToConnectionError(2i32);
    pub const DeviceLocked: PlayToConnectionError = PlayToConnectionError(3i32);
    pub const ProtectedPlaybackFailed: PlayToConnectionError = PlayToConnectionError(4i32);
}
#[repr(transparent)]
pub struct PlayToConnectionErrorEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct PlayToConnectionState(pub i32);
impl PlayToConnectionState {
    pub const Disconnected: PlayToConnectionState = PlayToConnectionState(0i32);
    pub const Connected: PlayToConnectionState = PlayToConnectionState(1i32);
    pub const Rendering: PlayToConnectionState = PlayToConnectionState(2i32);
}
#[repr(transparent)]
pub struct PlayToConnectionStateChangedEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct PlayToConnectionTransferredEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct PlayToManager(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct PlayToReceiver(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct PlayToSource(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct PlayToSourceDeferral(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct PlayToSourceRequest(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct PlayToSourceRequestedEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct PlayToSourceSelectedEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct PlaybackRateChangeRequestedEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct SourceChangeRequestedEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct VolumeChangeRequestedEventArgs(pub *mut ::core::ffi::c_void);

#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, clashing_extern_declarations, clippy::all)]
#[link(name = "windows")]
extern "system" {}
pub type CurrentTimeChangeRequestedEventArgs = *mut ::core::ffi::c_void;
pub type MuteChangeRequestedEventArgs = *mut ::core::ffi::c_void;
pub type PlayToConnection = *mut ::core::ffi::c_void;
#[repr(transparent)]
pub struct PlayToConnectionError(pub i32);
impl PlayToConnectionError {
    pub const None: Self = Self(0i32);
    pub const DeviceNotResponding: Self = Self(1i32);
    pub const DeviceError: Self = Self(2i32);
    pub const DeviceLocked: Self = Self(3i32);
    pub const ProtectedPlaybackFailed: Self = Self(4i32);
}
impl ::core::marker::Copy for PlayToConnectionError {}
impl ::core::clone::Clone for PlayToConnectionError {
    fn clone(&self) -> Self {
        *self
    }
}
pub type PlayToConnectionErrorEventArgs = *mut ::core::ffi::c_void;
#[repr(transparent)]
pub struct PlayToConnectionState(pub i32);
impl PlayToConnectionState {
    pub const Disconnected: Self = Self(0i32);
    pub const Connected: Self = Self(1i32);
    pub const Rendering: Self = Self(2i32);
}
impl ::core::marker::Copy for PlayToConnectionState {}
impl ::core::clone::Clone for PlayToConnectionState {
    fn clone(&self) -> Self {
        *self
    }
}
pub type PlayToConnectionStateChangedEventArgs = *mut ::core::ffi::c_void;
pub type PlayToConnectionTransferredEventArgs = *mut ::core::ffi::c_void;
pub type PlayToManager = *mut ::core::ffi::c_void;
pub type PlayToReceiver = *mut ::core::ffi::c_void;
pub type PlayToSource = *mut ::core::ffi::c_void;
pub type PlayToSourceDeferral = *mut ::core::ffi::c_void;
pub type PlayToSourceRequest = *mut ::core::ffi::c_void;
pub type PlayToSourceRequestedEventArgs = *mut ::core::ffi::c_void;
pub type PlayToSourceSelectedEventArgs = *mut ::core::ffi::c_void;
pub type PlaybackRateChangeRequestedEventArgs = *mut ::core::ffi::c_void;
pub type SourceChangeRequestedEventArgs = *mut ::core::ffi::c_void;
pub type VolumeChangeRequestedEventArgs = *mut ::core::ffi::c_void;

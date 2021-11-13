#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, clashing_extern_declarations, clippy::all)]
#[link(name = "windows")]
extern "system" {}
#[repr(transparent)]
pub struct CurrentTimeChangeRequestedEventArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for CurrentTimeChangeRequestedEventArgs {}
impl ::core::clone::Clone for CurrentTimeChangeRequestedEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ICurrentTimeChangeRequestedEventArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ICurrentTimeChangeRequestedEventArgs {}
impl ::core::clone::Clone for ICurrentTimeChangeRequestedEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IMuteChangeRequestedEventArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IMuteChangeRequestedEventArgs {}
impl ::core::clone::Clone for IMuteChangeRequestedEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IPlayToConnection(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IPlayToConnection {}
impl ::core::clone::Clone for IPlayToConnection {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IPlayToConnectionErrorEventArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IPlayToConnectionErrorEventArgs {}
impl ::core::clone::Clone for IPlayToConnectionErrorEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IPlayToConnectionStateChangedEventArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IPlayToConnectionStateChangedEventArgs {}
impl ::core::clone::Clone for IPlayToConnectionStateChangedEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IPlayToConnectionTransferredEventArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IPlayToConnectionTransferredEventArgs {}
impl ::core::clone::Clone for IPlayToConnectionTransferredEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IPlayToManager(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IPlayToManager {}
impl ::core::clone::Clone for IPlayToManager {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IPlayToManagerStatics(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IPlayToManagerStatics {}
impl ::core::clone::Clone for IPlayToManagerStatics {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IPlayToReceiver(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IPlayToReceiver {}
impl ::core::clone::Clone for IPlayToReceiver {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IPlayToSource(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IPlayToSource {}
impl ::core::clone::Clone for IPlayToSource {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IPlayToSourceDeferral(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IPlayToSourceDeferral {}
impl ::core::clone::Clone for IPlayToSourceDeferral {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IPlayToSourceRequest(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IPlayToSourceRequest {}
impl ::core::clone::Clone for IPlayToSourceRequest {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IPlayToSourceRequestedEventArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IPlayToSourceRequestedEventArgs {}
impl ::core::clone::Clone for IPlayToSourceRequestedEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IPlayToSourceSelectedEventArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IPlayToSourceSelectedEventArgs {}
impl ::core::clone::Clone for IPlayToSourceSelectedEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IPlayToSourceWithPreferredSourceUri(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IPlayToSourceWithPreferredSourceUri {}
impl ::core::clone::Clone for IPlayToSourceWithPreferredSourceUri {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IPlaybackRateChangeRequestedEventArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IPlaybackRateChangeRequestedEventArgs {}
impl ::core::clone::Clone for IPlaybackRateChangeRequestedEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ISourceChangeRequestedEventArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ISourceChangeRequestedEventArgs {}
impl ::core::clone::Clone for ISourceChangeRequestedEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IVolumeChangeRequestedEventArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IVolumeChangeRequestedEventArgs {}
impl ::core::clone::Clone for IVolumeChangeRequestedEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct MuteChangeRequestedEventArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for MuteChangeRequestedEventArgs {}
impl ::core::clone::Clone for MuteChangeRequestedEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct PlayToConnection(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for PlayToConnection {}
impl ::core::clone::Clone for PlayToConnection {
    fn clone(&self) -> Self {
        *self
    }
}
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
#[repr(transparent)]
pub struct PlayToConnectionErrorEventArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for PlayToConnectionErrorEventArgs {}
impl ::core::clone::Clone for PlayToConnectionErrorEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
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
#[repr(transparent)]
pub struct PlayToConnectionStateChangedEventArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for PlayToConnectionStateChangedEventArgs {}
impl ::core::clone::Clone for PlayToConnectionStateChangedEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct PlayToConnectionTransferredEventArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for PlayToConnectionTransferredEventArgs {}
impl ::core::clone::Clone for PlayToConnectionTransferredEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct PlayToManager(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for PlayToManager {}
impl ::core::clone::Clone for PlayToManager {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct PlayToReceiver(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for PlayToReceiver {}
impl ::core::clone::Clone for PlayToReceiver {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct PlayToSource(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for PlayToSource {}
impl ::core::clone::Clone for PlayToSource {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct PlayToSourceDeferral(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for PlayToSourceDeferral {}
impl ::core::clone::Clone for PlayToSourceDeferral {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct PlayToSourceRequest(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for PlayToSourceRequest {}
impl ::core::clone::Clone for PlayToSourceRequest {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct PlayToSourceRequestedEventArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for PlayToSourceRequestedEventArgs {}
impl ::core::clone::Clone for PlayToSourceRequestedEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct PlayToSourceSelectedEventArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for PlayToSourceSelectedEventArgs {}
impl ::core::clone::Clone for PlayToSourceSelectedEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct PlaybackRateChangeRequestedEventArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for PlaybackRateChangeRequestedEventArgs {}
impl ::core::clone::Clone for PlaybackRateChangeRequestedEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct SourceChangeRequestedEventArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for SourceChangeRequestedEventArgs {}
impl ::core::clone::Clone for SourceChangeRequestedEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct VolumeChangeRequestedEventArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for VolumeChangeRequestedEventArgs {}
impl ::core::clone::Clone for VolumeChangeRequestedEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}

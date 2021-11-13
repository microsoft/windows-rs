#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, clashing_extern_declarations, clippy::all)]
#[link(name = "windows")]
extern "system" {}
#[repr(transparent)]
pub struct CurrentSessionChangedEventArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for CurrentSessionChangedEventArgs {}
impl ::core::clone::Clone for CurrentSessionChangedEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct GlobalSystemMediaTransportControlsSession(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for GlobalSystemMediaTransportControlsSession {}
impl ::core::clone::Clone for GlobalSystemMediaTransportControlsSession {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct GlobalSystemMediaTransportControlsSessionManager(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for GlobalSystemMediaTransportControlsSessionManager {}
impl ::core::clone::Clone for GlobalSystemMediaTransportControlsSessionManager {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct GlobalSystemMediaTransportControlsSessionMediaProperties(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for GlobalSystemMediaTransportControlsSessionMediaProperties {}
impl ::core::clone::Clone for GlobalSystemMediaTransportControlsSessionMediaProperties {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct GlobalSystemMediaTransportControlsSessionPlaybackControls(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for GlobalSystemMediaTransportControlsSessionPlaybackControls {}
impl ::core::clone::Clone for GlobalSystemMediaTransportControlsSessionPlaybackControls {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct GlobalSystemMediaTransportControlsSessionPlaybackInfo(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for GlobalSystemMediaTransportControlsSessionPlaybackInfo {}
impl ::core::clone::Clone for GlobalSystemMediaTransportControlsSessionPlaybackInfo {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct GlobalSystemMediaTransportControlsSessionPlaybackStatus(pub i32);
impl GlobalSystemMediaTransportControlsSessionPlaybackStatus {
    pub const Closed: Self = Self(0i32);
    pub const Opened: Self = Self(1i32);
    pub const Changing: Self = Self(2i32);
    pub const Stopped: Self = Self(3i32);
    pub const Playing: Self = Self(4i32);
    pub const Paused: Self = Self(5i32);
}
impl ::core::marker::Copy for GlobalSystemMediaTransportControlsSessionPlaybackStatus {}
impl ::core::clone::Clone for GlobalSystemMediaTransportControlsSessionPlaybackStatus {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct GlobalSystemMediaTransportControlsSessionTimelineProperties(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for GlobalSystemMediaTransportControlsSessionTimelineProperties {}
impl ::core::clone::Clone for GlobalSystemMediaTransportControlsSessionTimelineProperties {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ICurrentSessionChangedEventArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ICurrentSessionChangedEventArgs {}
impl ::core::clone::Clone for ICurrentSessionChangedEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IGlobalSystemMediaTransportControlsSession(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IGlobalSystemMediaTransportControlsSession {}
impl ::core::clone::Clone for IGlobalSystemMediaTransportControlsSession {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IGlobalSystemMediaTransportControlsSessionManager(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IGlobalSystemMediaTransportControlsSessionManager {}
impl ::core::clone::Clone for IGlobalSystemMediaTransportControlsSessionManager {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IGlobalSystemMediaTransportControlsSessionManagerStatics(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IGlobalSystemMediaTransportControlsSessionManagerStatics {}
impl ::core::clone::Clone for IGlobalSystemMediaTransportControlsSessionManagerStatics {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IGlobalSystemMediaTransportControlsSessionMediaProperties(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IGlobalSystemMediaTransportControlsSessionMediaProperties {}
impl ::core::clone::Clone for IGlobalSystemMediaTransportControlsSessionMediaProperties {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IGlobalSystemMediaTransportControlsSessionPlaybackControls(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IGlobalSystemMediaTransportControlsSessionPlaybackControls {}
impl ::core::clone::Clone for IGlobalSystemMediaTransportControlsSessionPlaybackControls {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IGlobalSystemMediaTransportControlsSessionPlaybackInfo(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IGlobalSystemMediaTransportControlsSessionPlaybackInfo {}
impl ::core::clone::Clone for IGlobalSystemMediaTransportControlsSessionPlaybackInfo {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IGlobalSystemMediaTransportControlsSessionTimelineProperties(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IGlobalSystemMediaTransportControlsSessionTimelineProperties {}
impl ::core::clone::Clone for IGlobalSystemMediaTransportControlsSessionTimelineProperties {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IMediaPropertiesChangedEventArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IMediaPropertiesChangedEventArgs {}
impl ::core::clone::Clone for IMediaPropertiesChangedEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IPlaybackInfoChangedEventArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IPlaybackInfoChangedEventArgs {}
impl ::core::clone::Clone for IPlaybackInfoChangedEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ISessionsChangedEventArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ISessionsChangedEventArgs {}
impl ::core::clone::Clone for ISessionsChangedEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ITimelinePropertiesChangedEventArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ITimelinePropertiesChangedEventArgs {}
impl ::core::clone::Clone for ITimelinePropertiesChangedEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct MediaPropertiesChangedEventArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for MediaPropertiesChangedEventArgs {}
impl ::core::clone::Clone for MediaPropertiesChangedEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct PlaybackInfoChangedEventArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for PlaybackInfoChangedEventArgs {}
impl ::core::clone::Clone for PlaybackInfoChangedEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct SessionsChangedEventArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for SessionsChangedEventArgs {}
impl ::core::clone::Clone for SessionsChangedEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct TimelinePropertiesChangedEventArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for TimelinePropertiesChangedEventArgs {}
impl ::core::clone::Clone for TimelinePropertiesChangedEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}

#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals)]
#[link(name = "windows")]
extern "system" {}
#[repr(transparent)]
pub struct CastingConnection(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct CastingConnectionErrorOccurredEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct CastingConnectionErrorStatus(pub i32);
impl CastingConnectionErrorStatus {
    pub const Succeeded: Self = Self(0i32);
    pub const DeviceDidNotRespond: Self = Self(1i32);
    pub const DeviceError: Self = Self(2i32);
    pub const DeviceLocked: Self = Self(3i32);
    pub const ProtectedPlaybackFailed: Self = Self(4i32);
    pub const InvalidCastingSource: Self = Self(5i32);
    pub const Unknown: Self = Self(6i32);
}
impl ::core::marker::Copy for CastingConnectionErrorStatus {}
impl ::core::clone::Clone for CastingConnectionErrorStatus {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct CastingConnectionState(pub i32);
impl CastingConnectionState {
    pub const Disconnected: Self = Self(0i32);
    pub const Connected: Self = Self(1i32);
    pub const Rendering: Self = Self(2i32);
    pub const Disconnecting: Self = Self(3i32);
    pub const Connecting: Self = Self(4i32);
}
impl ::core::marker::Copy for CastingConnectionState {}
impl ::core::clone::Clone for CastingConnectionState {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct CastingDevice(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct CastingDevicePicker(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct CastingDevicePickerFilter(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct CastingDeviceSelectedEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct CastingPlaybackTypes(pub u32);
impl CastingPlaybackTypes {
    pub const None: Self = Self(0u32);
    pub const Audio: Self = Self(1u32);
    pub const Video: Self = Self(2u32);
    pub const Picture: Self = Self(4u32);
}
impl ::core::marker::Copy for CastingPlaybackTypes {}
impl ::core::clone::Clone for CastingPlaybackTypes {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct CastingSource(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ICastingConnection(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ICastingConnectionErrorOccurredEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ICastingDevice(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ICastingDevicePicker(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ICastingDevicePickerFilter(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ICastingDeviceSelectedEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ICastingDeviceStatics(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ICastingSource(pub *mut ::core::ffi::c_void);

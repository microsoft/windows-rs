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
    pub const Succeeded: CastingConnectionErrorStatus = CastingConnectionErrorStatus(0i32);
    pub const DeviceDidNotRespond: CastingConnectionErrorStatus = CastingConnectionErrorStatus(1i32);
    pub const DeviceError: CastingConnectionErrorStatus = CastingConnectionErrorStatus(2i32);
    pub const DeviceLocked: CastingConnectionErrorStatus = CastingConnectionErrorStatus(3i32);
    pub const ProtectedPlaybackFailed: CastingConnectionErrorStatus = CastingConnectionErrorStatus(4i32);
    pub const InvalidCastingSource: CastingConnectionErrorStatus = CastingConnectionErrorStatus(5i32);
    pub const Unknown: CastingConnectionErrorStatus = CastingConnectionErrorStatus(6i32);
}
#[repr(transparent)]
pub struct CastingConnectionState(pub i32);
impl CastingConnectionState {
    pub const Disconnected: CastingConnectionState = CastingConnectionState(0i32);
    pub const Connected: CastingConnectionState = CastingConnectionState(1i32);
    pub const Rendering: CastingConnectionState = CastingConnectionState(2i32);
    pub const Disconnecting: CastingConnectionState = CastingConnectionState(3i32);
    pub const Connecting: CastingConnectionState = CastingConnectionState(4i32);
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
    pub const None: CastingPlaybackTypes = CastingPlaybackTypes(0u32);
    pub const Audio: CastingPlaybackTypes = CastingPlaybackTypes(1u32);
    pub const Video: CastingPlaybackTypes = CastingPlaybackTypes(2u32);
    pub const Picture: CastingPlaybackTypes = CastingPlaybackTypes(4u32);
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

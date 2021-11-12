#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals)]
#[link(name = "windows")]
extern "system" {}
#[repr(transparent)]
pub struct DevicePortalConnection(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct DevicePortalConnectionClosedEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct DevicePortalConnectionClosedReason(pub i32);
impl DevicePortalConnectionClosedReason {
    pub const Unknown: Self = Self(0i32);
    pub const ResourceLimitsExceeded: Self = Self(1i32);
    pub const ProtocolError: Self = Self(2i32);
    pub const NotAuthorized: Self = Self(3i32);
    pub const UserNotPresent: Self = Self(4i32);
    pub const ServiceTerminated: Self = Self(5i32);
}
impl ::core::marker::Copy for DevicePortalConnectionClosedReason {}
impl ::core::clone::Clone for DevicePortalConnectionClosedReason {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct DevicePortalConnectionRequestReceivedEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IDevicePortalConnection(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IDevicePortalConnectionClosedEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IDevicePortalConnectionRequestReceivedEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IDevicePortalConnectionStatics(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IDevicePortalWebSocketConnection(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IDevicePortalWebSocketConnectionRequestReceivedEventArgs(pub *mut ::core::ffi::c_void);

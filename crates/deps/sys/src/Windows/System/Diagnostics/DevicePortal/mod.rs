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
    pub const Unknown: DevicePortalConnectionClosedReason = DevicePortalConnectionClosedReason(0i32);
    pub const ResourceLimitsExceeded: DevicePortalConnectionClosedReason = DevicePortalConnectionClosedReason(1i32);
    pub const ProtocolError: DevicePortalConnectionClosedReason = DevicePortalConnectionClosedReason(2i32);
    pub const NotAuthorized: DevicePortalConnectionClosedReason = DevicePortalConnectionClosedReason(3i32);
    pub const UserNotPresent: DevicePortalConnectionClosedReason = DevicePortalConnectionClosedReason(4i32);
    pub const ServiceTerminated: DevicePortalConnectionClosedReason = DevicePortalConnectionClosedReason(5i32);
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

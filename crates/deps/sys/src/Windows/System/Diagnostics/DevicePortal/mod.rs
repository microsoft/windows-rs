#![allow(non_snake_case, non_camel_case_types)]
#[link(name = "windows")]
extern "system" {}
pub struct DevicePortalConnection(i32);
pub struct DevicePortalConnectionClosedEventArgs(i32);
pub struct DevicePortalConnectionClosedReason(i32);
pub struct DevicePortalConnectionRequestReceivedEventArgs(i32);
pub struct IDevicePortalConnection(pub *mut ::core::ffi::c_void);
pub struct IDevicePortalConnectionClosedEventArgs(pub *mut ::core::ffi::c_void);
pub struct IDevicePortalConnectionRequestReceivedEventArgs(pub *mut ::core::ffi::c_void);
pub struct IDevicePortalConnectionStatics(pub *mut ::core::ffi::c_void);
pub struct IDevicePortalWebSocketConnection(pub *mut ::core::ffi::c_void);
pub struct IDevicePortalWebSocketConnectionRequestReceivedEventArgs(pub *mut ::core::ffi::c_void);

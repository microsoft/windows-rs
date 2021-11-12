#![allow(non_snake_case, non_camel_case_types)]
#[link(name = "windows")]
extern "system" {}
#[repr(transparent)]
pub struct DevicePortalConnection(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct DevicePortalConnectionClosedEventArgs(pub *mut ::core::ffi::c_void);
#[repr(C)]
pub struct DevicePortalConnectionClosedReason(i32);
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

#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals)]
#[link(name = "windows")]
extern "system" {}
#[repr(transparent)]
pub struct DevicePortalConnection(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for DevicePortalConnection {}
impl ::core::clone::Clone for DevicePortalConnection {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct DevicePortalConnectionClosedEventArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for DevicePortalConnectionClosedEventArgs {}
impl ::core::clone::Clone for DevicePortalConnectionClosedEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
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
impl ::core::marker::Copy for DevicePortalConnectionRequestReceivedEventArgs {}
impl ::core::clone::Clone for DevicePortalConnectionRequestReceivedEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IDevicePortalConnection(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IDevicePortalConnection {}
impl ::core::clone::Clone for IDevicePortalConnection {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IDevicePortalConnectionClosedEventArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IDevicePortalConnectionClosedEventArgs {}
impl ::core::clone::Clone for IDevicePortalConnectionClosedEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IDevicePortalConnectionRequestReceivedEventArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IDevicePortalConnectionRequestReceivedEventArgs {}
impl ::core::clone::Clone for IDevicePortalConnectionRequestReceivedEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IDevicePortalConnectionStatics(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IDevicePortalConnectionStatics {}
impl ::core::clone::Clone for IDevicePortalConnectionStatics {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IDevicePortalWebSocketConnection(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IDevicePortalWebSocketConnection {}
impl ::core::clone::Clone for IDevicePortalWebSocketConnection {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IDevicePortalWebSocketConnectionRequestReceivedEventArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IDevicePortalWebSocketConnectionRequestReceivedEventArgs {}
impl ::core::clone::Clone for IDevicePortalWebSocketConnectionRequestReceivedEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}

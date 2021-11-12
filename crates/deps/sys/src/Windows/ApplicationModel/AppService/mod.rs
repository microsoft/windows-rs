#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals)]
#[link(name = "windows")]
extern "system" {}
#[repr(transparent)]
pub struct AppServiceClosedEventArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for AppServiceClosedEventArgs {}
impl ::core::clone::Clone for AppServiceClosedEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct AppServiceClosedStatus(pub i32);
impl AppServiceClosedStatus {
    pub const Completed: Self = Self(0i32);
    pub const Canceled: Self = Self(1i32);
    pub const ResourceLimitsExceeded: Self = Self(2i32);
    pub const Unknown: Self = Self(3i32);
}
impl ::core::marker::Copy for AppServiceClosedStatus {}
impl ::core::clone::Clone for AppServiceClosedStatus {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct AppServiceConnection(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for AppServiceConnection {}
impl ::core::clone::Clone for AppServiceConnection {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct AppServiceConnectionStatus(pub i32);
impl AppServiceConnectionStatus {
    pub const Success: Self = Self(0i32);
    pub const AppNotInstalled: Self = Self(1i32);
    pub const AppUnavailable: Self = Self(2i32);
    pub const AppServiceUnavailable: Self = Self(3i32);
    pub const Unknown: Self = Self(4i32);
    pub const RemoteSystemUnavailable: Self = Self(5i32);
    pub const RemoteSystemNotSupportedByApp: Self = Self(6i32);
    pub const NotAuthorized: Self = Self(7i32);
    pub const AuthenticationError: Self = Self(8i32);
    pub const NetworkNotAvailable: Self = Self(9i32);
    pub const DisabledByPolicy: Self = Self(10i32);
    pub const WebServiceUnavailable: Self = Self(11i32);
}
impl ::core::marker::Copy for AppServiceConnectionStatus {}
impl ::core::clone::Clone for AppServiceConnectionStatus {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct AppServiceDeferral(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for AppServiceDeferral {}
impl ::core::clone::Clone for AppServiceDeferral {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct AppServiceRequest(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for AppServiceRequest {}
impl ::core::clone::Clone for AppServiceRequest {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct AppServiceRequestReceivedEventArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for AppServiceRequestReceivedEventArgs {}
impl ::core::clone::Clone for AppServiceRequestReceivedEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct AppServiceResponse(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for AppServiceResponse {}
impl ::core::clone::Clone for AppServiceResponse {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct AppServiceResponseStatus(pub i32);
impl AppServiceResponseStatus {
    pub const Success: Self = Self(0i32);
    pub const Failure: Self = Self(1i32);
    pub const ResourceLimitsExceeded: Self = Self(2i32);
    pub const Unknown: Self = Self(3i32);
    pub const RemoteSystemUnavailable: Self = Self(4i32);
    pub const MessageSizeTooLarge: Self = Self(5i32);
    pub const AppUnavailable: Self = Self(6i32);
    pub const AuthenticationError: Self = Self(7i32);
    pub const NetworkNotAvailable: Self = Self(8i32);
    pub const DisabledByPolicy: Self = Self(9i32);
    pub const WebServiceUnavailable: Self = Self(10i32);
}
impl ::core::marker::Copy for AppServiceResponseStatus {}
impl ::core::clone::Clone for AppServiceResponseStatus {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct AppServiceTriggerDetails(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for AppServiceTriggerDetails {}
impl ::core::clone::Clone for AppServiceTriggerDetails {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IAppServiceCatalogStatics(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IAppServiceCatalogStatics {}
impl ::core::clone::Clone for IAppServiceCatalogStatics {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IAppServiceClosedEventArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IAppServiceClosedEventArgs {}
impl ::core::clone::Clone for IAppServiceClosedEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IAppServiceConnection(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IAppServiceConnection {}
impl ::core::clone::Clone for IAppServiceConnection {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IAppServiceConnection2(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IAppServiceConnection2 {}
impl ::core::clone::Clone for IAppServiceConnection2 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IAppServiceConnectionStatics(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IAppServiceConnectionStatics {}
impl ::core::clone::Clone for IAppServiceConnectionStatics {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IAppServiceDeferral(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IAppServiceDeferral {}
impl ::core::clone::Clone for IAppServiceDeferral {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IAppServiceRequest(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IAppServiceRequest {}
impl ::core::clone::Clone for IAppServiceRequest {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IAppServiceRequestReceivedEventArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IAppServiceRequestReceivedEventArgs {}
impl ::core::clone::Clone for IAppServiceRequestReceivedEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IAppServiceResponse(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IAppServiceResponse {}
impl ::core::clone::Clone for IAppServiceResponse {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IAppServiceTriggerDetails(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IAppServiceTriggerDetails {}
impl ::core::clone::Clone for IAppServiceTriggerDetails {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IAppServiceTriggerDetails2(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IAppServiceTriggerDetails2 {}
impl ::core::clone::Clone for IAppServiceTriggerDetails2 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IAppServiceTriggerDetails3(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IAppServiceTriggerDetails3 {}
impl ::core::clone::Clone for IAppServiceTriggerDetails3 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IAppServiceTriggerDetails4(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IAppServiceTriggerDetails4 {}
impl ::core::clone::Clone for IAppServiceTriggerDetails4 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IStatelessAppServiceResponse(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IStatelessAppServiceResponse {}
impl ::core::clone::Clone for IStatelessAppServiceResponse {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct StatelessAppServiceResponse(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for StatelessAppServiceResponse {}
impl ::core::clone::Clone for StatelessAppServiceResponse {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct StatelessAppServiceResponseStatus(pub i32);
impl StatelessAppServiceResponseStatus {
    pub const Success: Self = Self(0i32);
    pub const AppNotInstalled: Self = Self(1i32);
    pub const AppUnavailable: Self = Self(2i32);
    pub const AppServiceUnavailable: Self = Self(3i32);
    pub const RemoteSystemUnavailable: Self = Self(4i32);
    pub const RemoteSystemNotSupportedByApp: Self = Self(5i32);
    pub const NotAuthorized: Self = Self(6i32);
    pub const ResourceLimitsExceeded: Self = Self(7i32);
    pub const MessageSizeTooLarge: Self = Self(8i32);
    pub const Failure: Self = Self(9i32);
    pub const Unknown: Self = Self(10i32);
    pub const AuthenticationError: Self = Self(11i32);
    pub const NetworkNotAvailable: Self = Self(12i32);
    pub const DisabledByPolicy: Self = Self(13i32);
    pub const WebServiceUnavailable: Self = Self(14i32);
}
impl ::core::marker::Copy for StatelessAppServiceResponseStatus {}
impl ::core::clone::Clone for StatelessAppServiceResponseStatus {
    fn clone(&self) -> Self {
        *self
    }
}

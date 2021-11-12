#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals)]
#[link(name = "windows")]
extern "system" {}
#[repr(transparent)]
pub struct AppServiceClosedEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct AppServiceClosedStatus(pub i32);
impl AppServiceClosedStatus {
    pub const Completed: AppServiceClosedStatus = AppServiceClosedStatus(0i32);
    pub const Canceled: AppServiceClosedStatus = AppServiceClosedStatus(1i32);
    pub const ResourceLimitsExceeded: AppServiceClosedStatus = AppServiceClosedStatus(2i32);
    pub const Unknown: AppServiceClosedStatus = AppServiceClosedStatus(3i32);
}
#[repr(transparent)]
pub struct AppServiceConnection(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct AppServiceConnectionStatus(pub i32);
impl AppServiceConnectionStatus {
    pub const Success: AppServiceConnectionStatus = AppServiceConnectionStatus(0i32);
    pub const AppNotInstalled: AppServiceConnectionStatus = AppServiceConnectionStatus(1i32);
    pub const AppUnavailable: AppServiceConnectionStatus = AppServiceConnectionStatus(2i32);
    pub const AppServiceUnavailable: AppServiceConnectionStatus = AppServiceConnectionStatus(3i32);
    pub const Unknown: AppServiceConnectionStatus = AppServiceConnectionStatus(4i32);
    pub const RemoteSystemUnavailable: AppServiceConnectionStatus = AppServiceConnectionStatus(5i32);
    pub const RemoteSystemNotSupportedByApp: AppServiceConnectionStatus = AppServiceConnectionStatus(6i32);
    pub const NotAuthorized: AppServiceConnectionStatus = AppServiceConnectionStatus(7i32);
    pub const AuthenticationError: AppServiceConnectionStatus = AppServiceConnectionStatus(8i32);
    pub const NetworkNotAvailable: AppServiceConnectionStatus = AppServiceConnectionStatus(9i32);
    pub const DisabledByPolicy: AppServiceConnectionStatus = AppServiceConnectionStatus(10i32);
    pub const WebServiceUnavailable: AppServiceConnectionStatus = AppServiceConnectionStatus(11i32);
}
#[repr(transparent)]
pub struct AppServiceDeferral(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct AppServiceRequest(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct AppServiceRequestReceivedEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct AppServiceResponse(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct AppServiceResponseStatus(pub i32);
impl AppServiceResponseStatus {
    pub const Success: AppServiceResponseStatus = AppServiceResponseStatus(0i32);
    pub const Failure: AppServiceResponseStatus = AppServiceResponseStatus(1i32);
    pub const ResourceLimitsExceeded: AppServiceResponseStatus = AppServiceResponseStatus(2i32);
    pub const Unknown: AppServiceResponseStatus = AppServiceResponseStatus(3i32);
    pub const RemoteSystemUnavailable: AppServiceResponseStatus = AppServiceResponseStatus(4i32);
    pub const MessageSizeTooLarge: AppServiceResponseStatus = AppServiceResponseStatus(5i32);
    pub const AppUnavailable: AppServiceResponseStatus = AppServiceResponseStatus(6i32);
    pub const AuthenticationError: AppServiceResponseStatus = AppServiceResponseStatus(7i32);
    pub const NetworkNotAvailable: AppServiceResponseStatus = AppServiceResponseStatus(8i32);
    pub const DisabledByPolicy: AppServiceResponseStatus = AppServiceResponseStatus(9i32);
    pub const WebServiceUnavailable: AppServiceResponseStatus = AppServiceResponseStatus(10i32);
}
#[repr(transparent)]
pub struct AppServiceTriggerDetails(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IAppServiceCatalogStatics(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IAppServiceClosedEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IAppServiceConnection(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IAppServiceConnection2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IAppServiceConnectionStatics(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IAppServiceDeferral(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IAppServiceRequest(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IAppServiceRequestReceivedEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IAppServiceResponse(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IAppServiceTriggerDetails(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IAppServiceTriggerDetails2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IAppServiceTriggerDetails3(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IAppServiceTriggerDetails4(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IStatelessAppServiceResponse(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct StatelessAppServiceResponse(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct StatelessAppServiceResponseStatus(pub i32);
impl StatelessAppServiceResponseStatus {
    pub const Success: StatelessAppServiceResponseStatus = StatelessAppServiceResponseStatus(0i32);
    pub const AppNotInstalled: StatelessAppServiceResponseStatus = StatelessAppServiceResponseStatus(1i32);
    pub const AppUnavailable: StatelessAppServiceResponseStatus = StatelessAppServiceResponseStatus(2i32);
    pub const AppServiceUnavailable: StatelessAppServiceResponseStatus = StatelessAppServiceResponseStatus(3i32);
    pub const RemoteSystemUnavailable: StatelessAppServiceResponseStatus = StatelessAppServiceResponseStatus(4i32);
    pub const RemoteSystemNotSupportedByApp: StatelessAppServiceResponseStatus = StatelessAppServiceResponseStatus(5i32);
    pub const NotAuthorized: StatelessAppServiceResponseStatus = StatelessAppServiceResponseStatus(6i32);
    pub const ResourceLimitsExceeded: StatelessAppServiceResponseStatus = StatelessAppServiceResponseStatus(7i32);
    pub const MessageSizeTooLarge: StatelessAppServiceResponseStatus = StatelessAppServiceResponseStatus(8i32);
    pub const Failure: StatelessAppServiceResponseStatus = StatelessAppServiceResponseStatus(9i32);
    pub const Unknown: StatelessAppServiceResponseStatus = StatelessAppServiceResponseStatus(10i32);
    pub const AuthenticationError: StatelessAppServiceResponseStatus = StatelessAppServiceResponseStatus(11i32);
    pub const NetworkNotAvailable: StatelessAppServiceResponseStatus = StatelessAppServiceResponseStatus(12i32);
    pub const DisabledByPolicy: StatelessAppServiceResponseStatus = StatelessAppServiceResponseStatus(13i32);
    pub const WebServiceUnavailable: StatelessAppServiceResponseStatus = StatelessAppServiceResponseStatus(14i32);
}

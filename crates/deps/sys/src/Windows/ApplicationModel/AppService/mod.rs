#![allow(non_snake_case, non_camel_case_types)]
#[link(name = "windows")]
extern "system" {}
pub struct AppServiceCatalog(i32);
pub struct AppServiceClosedEventArgs(i32);
pub struct AppServiceClosedStatus(i32);
pub struct AppServiceConnection(i32);
pub struct AppServiceConnectionStatus(i32);
pub struct AppServiceDeferral(i32);
pub struct AppServiceRequest(i32);
pub struct AppServiceRequestReceivedEventArgs(i32);
pub struct AppServiceResponse(i32);
pub struct AppServiceResponseStatus(i32);
pub struct AppServiceTriggerDetails(i32);
pub struct IAppServiceCatalogStatics(pub *mut ::core::ffi::c_void);
pub struct IAppServiceClosedEventArgs(pub *mut ::core::ffi::c_void);
pub struct IAppServiceConnection(pub *mut ::core::ffi::c_void);
pub struct IAppServiceConnection2(pub *mut ::core::ffi::c_void);
pub struct IAppServiceConnectionStatics(pub *mut ::core::ffi::c_void);
pub struct IAppServiceDeferral(pub *mut ::core::ffi::c_void);
pub struct IAppServiceRequest(pub *mut ::core::ffi::c_void);
pub struct IAppServiceRequestReceivedEventArgs(pub *mut ::core::ffi::c_void);
pub struct IAppServiceResponse(pub *mut ::core::ffi::c_void);
pub struct IAppServiceTriggerDetails(pub *mut ::core::ffi::c_void);
pub struct IAppServiceTriggerDetails2(pub *mut ::core::ffi::c_void);
pub struct IAppServiceTriggerDetails3(pub *mut ::core::ffi::c_void);
pub struct IAppServiceTriggerDetails4(pub *mut ::core::ffi::c_void);
pub struct IStatelessAppServiceResponse(pub *mut ::core::ffi::c_void);
pub struct StatelessAppServiceResponse(i32);
pub struct StatelessAppServiceResponseStatus(i32);

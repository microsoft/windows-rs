#![allow(non_snake_case, non_camel_case_types)]
#[link(name = "windows")]
extern "system" {}
#[repr(transparent)]
pub struct AppServiceCatalog(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct AppServiceClosedEventArgs(pub *mut ::core::ffi::c_void);
pub struct AppServiceClosedStatus(i32);
#[repr(transparent)]
pub struct AppServiceConnection(pub *mut ::core::ffi::c_void);
pub struct AppServiceConnectionStatus(i32);
#[repr(transparent)]
pub struct AppServiceDeferral(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct AppServiceRequest(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct AppServiceRequestReceivedEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct AppServiceResponse(pub *mut ::core::ffi::c_void);
pub struct AppServiceResponseStatus(i32);
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
pub struct StatelessAppServiceResponseStatus(i32);

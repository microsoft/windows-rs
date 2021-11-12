#![allow(non_snake_case, non_camel_case_types)]
#[link(name = "windows")]
extern "system" {}
#[repr(transparent)]
pub struct HttpBaseProtocolFilter(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct HttpCacheControl(pub *mut ::core::ffi::c_void);
pub struct HttpCacheReadBehavior(i32);
pub struct HttpCacheWriteBehavior(i32);
pub struct HttpCookieUsageBehavior(i32);
#[repr(transparent)]
pub struct HttpServerCustomValidationRequestedEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IHttpBaseProtocolFilter(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IHttpBaseProtocolFilter2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IHttpBaseProtocolFilter3(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IHttpBaseProtocolFilter4(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IHttpBaseProtocolFilter5(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IHttpBaseProtocolFilterStatics(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IHttpCacheControl(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IHttpFilter(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IHttpServerCustomValidationRequestedEventArgs(pub *mut ::core::ffi::c_void);

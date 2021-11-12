#![allow(non_snake_case, non_camel_case_types)]
#[link(name = "windows")]
extern "system" {}
pub struct HttpBaseProtocolFilter(i32);
pub struct HttpCacheControl(i32);
pub struct HttpCacheReadBehavior(i32);
pub struct HttpCacheWriteBehavior(i32);
pub struct HttpCookieUsageBehavior(i32);
pub struct HttpServerCustomValidationRequestedEventArgs(i32);
pub struct IHttpBaseProtocolFilter(pub *mut ::core::ffi::c_void);
pub struct IHttpBaseProtocolFilter2(pub *mut ::core::ffi::c_void);
pub struct IHttpBaseProtocolFilter3(pub *mut ::core::ffi::c_void);
pub struct IHttpBaseProtocolFilter4(pub *mut ::core::ffi::c_void);
pub struct IHttpBaseProtocolFilter5(pub *mut ::core::ffi::c_void);
pub struct IHttpBaseProtocolFilterStatics(pub *mut ::core::ffi::c_void);
pub struct IHttpCacheControl(pub *mut ::core::ffi::c_void);
pub struct IHttpFilter(pub *mut ::core::ffi::c_void);
pub struct IHttpServerCustomValidationRequestedEventArgs(pub *mut ::core::ffi::c_void);

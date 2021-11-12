#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals)]
#[link(name = "windows")]
extern "system" {}
#[repr(transparent)]
pub struct HttpBaseProtocolFilter(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct HttpCacheControl(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct HttpCacheReadBehavior(pub i32);
impl HttpCacheReadBehavior {
    pub const Default: HttpCacheReadBehavior = HttpCacheReadBehavior(0i32);
    pub const MostRecent: HttpCacheReadBehavior = HttpCacheReadBehavior(1i32);
    pub const OnlyFromCache: HttpCacheReadBehavior = HttpCacheReadBehavior(2i32);
    pub const NoCache: HttpCacheReadBehavior = HttpCacheReadBehavior(3i32);
}
#[repr(transparent)]
pub struct HttpCacheWriteBehavior(pub i32);
impl HttpCacheWriteBehavior {
    pub const Default: HttpCacheWriteBehavior = HttpCacheWriteBehavior(0i32);
    pub const NoCache: HttpCacheWriteBehavior = HttpCacheWriteBehavior(1i32);
}
#[repr(transparent)]
pub struct HttpCookieUsageBehavior(pub i32);
impl HttpCookieUsageBehavior {
    pub const Default: HttpCookieUsageBehavior = HttpCookieUsageBehavior(0i32);
    pub const NoCookies: HttpCookieUsageBehavior = HttpCookieUsageBehavior(1i32);
}
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

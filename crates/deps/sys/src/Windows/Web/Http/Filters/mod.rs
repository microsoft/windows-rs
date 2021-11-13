#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, clashing_extern_declarations, clippy::all)]
#[link(name = "windows")]
extern "system" {}
#[repr(transparent)]
pub struct HttpBaseProtocolFilter(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for HttpBaseProtocolFilter {}
impl ::core::clone::Clone for HttpBaseProtocolFilter {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct HttpCacheControl(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for HttpCacheControl {}
impl ::core::clone::Clone for HttpCacheControl {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct HttpCacheReadBehavior(pub i32);
impl HttpCacheReadBehavior {
    pub const Default: Self = Self(0i32);
    pub const MostRecent: Self = Self(1i32);
    pub const OnlyFromCache: Self = Self(2i32);
    pub const NoCache: Self = Self(3i32);
}
impl ::core::marker::Copy for HttpCacheReadBehavior {}
impl ::core::clone::Clone for HttpCacheReadBehavior {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct HttpCacheWriteBehavior(pub i32);
impl HttpCacheWriteBehavior {
    pub const Default: Self = Self(0i32);
    pub const NoCache: Self = Self(1i32);
}
impl ::core::marker::Copy for HttpCacheWriteBehavior {}
impl ::core::clone::Clone for HttpCacheWriteBehavior {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct HttpCookieUsageBehavior(pub i32);
impl HttpCookieUsageBehavior {
    pub const Default: Self = Self(0i32);
    pub const NoCookies: Self = Self(1i32);
}
impl ::core::marker::Copy for HttpCookieUsageBehavior {}
impl ::core::clone::Clone for HttpCookieUsageBehavior {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct HttpServerCustomValidationRequestedEventArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for HttpServerCustomValidationRequestedEventArgs {}
impl ::core::clone::Clone for HttpServerCustomValidationRequestedEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IHttpBaseProtocolFilter(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IHttpBaseProtocolFilter {}
impl ::core::clone::Clone for IHttpBaseProtocolFilter {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IHttpBaseProtocolFilter2(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IHttpBaseProtocolFilter2 {}
impl ::core::clone::Clone for IHttpBaseProtocolFilter2 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IHttpBaseProtocolFilter3(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IHttpBaseProtocolFilter3 {}
impl ::core::clone::Clone for IHttpBaseProtocolFilter3 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IHttpBaseProtocolFilter4(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IHttpBaseProtocolFilter4 {}
impl ::core::clone::Clone for IHttpBaseProtocolFilter4 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IHttpBaseProtocolFilter5(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IHttpBaseProtocolFilter5 {}
impl ::core::clone::Clone for IHttpBaseProtocolFilter5 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IHttpBaseProtocolFilterStatics(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IHttpBaseProtocolFilterStatics {}
impl ::core::clone::Clone for IHttpBaseProtocolFilterStatics {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IHttpCacheControl(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IHttpCacheControl {}
impl ::core::clone::Clone for IHttpCacheControl {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IHttpFilter(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IHttpFilter {}
impl ::core::clone::Clone for IHttpFilter {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IHttpServerCustomValidationRequestedEventArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IHttpServerCustomValidationRequestedEventArgs {}
impl ::core::clone::Clone for IHttpServerCustomValidationRequestedEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}

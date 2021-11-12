#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals)]
#[link(name = "windows")]
extern "system" {}
#[repr(transparent)]
pub struct HttpDiagnosticProvider(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for HttpDiagnosticProvider {}
impl ::core::clone::Clone for HttpDiagnosticProvider {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct HttpDiagnosticProviderRequestResponseCompletedEventArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for HttpDiagnosticProviderRequestResponseCompletedEventArgs {}
impl ::core::clone::Clone for HttpDiagnosticProviderRequestResponseCompletedEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct HttpDiagnosticProviderRequestResponseTimestamps(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for HttpDiagnosticProviderRequestResponseTimestamps {}
impl ::core::clone::Clone for HttpDiagnosticProviderRequestResponseTimestamps {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct HttpDiagnosticProviderRequestSentEventArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for HttpDiagnosticProviderRequestSentEventArgs {}
impl ::core::clone::Clone for HttpDiagnosticProviderRequestSentEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct HttpDiagnosticProviderResponseReceivedEventArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for HttpDiagnosticProviderResponseReceivedEventArgs {}
impl ::core::clone::Clone for HttpDiagnosticProviderResponseReceivedEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct HttpDiagnosticRequestInitiator(pub i32);
impl HttpDiagnosticRequestInitiator {
    pub const ParsedElement: Self = Self(0i32);
    pub const Script: Self = Self(1i32);
    pub const Image: Self = Self(2i32);
    pub const Link: Self = Self(3i32);
    pub const Style: Self = Self(4i32);
    pub const XmlHttpRequest: Self = Self(5i32);
    pub const Media: Self = Self(6i32);
    pub const HtmlDownload: Self = Self(7i32);
    pub const Prefetch: Self = Self(8i32);
    pub const Other: Self = Self(9i32);
    pub const CrossOriginPreFlight: Self = Self(10i32);
    pub const Fetch: Self = Self(11i32);
    pub const Beacon: Self = Self(12i32);
}
impl ::core::marker::Copy for HttpDiagnosticRequestInitiator {}
impl ::core::clone::Clone for HttpDiagnosticRequestInitiator {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct HttpDiagnosticSourceLocation(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for HttpDiagnosticSourceLocation {}
impl ::core::clone::Clone for HttpDiagnosticSourceLocation {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IHttpDiagnosticProvider(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IHttpDiagnosticProvider {}
impl ::core::clone::Clone for IHttpDiagnosticProvider {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IHttpDiagnosticProviderRequestResponseCompletedEventArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IHttpDiagnosticProviderRequestResponseCompletedEventArgs {}
impl ::core::clone::Clone for IHttpDiagnosticProviderRequestResponseCompletedEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IHttpDiagnosticProviderRequestResponseTimestamps(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IHttpDiagnosticProviderRequestResponseTimestamps {}
impl ::core::clone::Clone for IHttpDiagnosticProviderRequestResponseTimestamps {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IHttpDiagnosticProviderRequestSentEventArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IHttpDiagnosticProviderRequestSentEventArgs {}
impl ::core::clone::Clone for IHttpDiagnosticProviderRequestSentEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IHttpDiagnosticProviderResponseReceivedEventArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IHttpDiagnosticProviderResponseReceivedEventArgs {}
impl ::core::clone::Clone for IHttpDiagnosticProviderResponseReceivedEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IHttpDiagnosticProviderStatics(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IHttpDiagnosticProviderStatics {}
impl ::core::clone::Clone for IHttpDiagnosticProviderStatics {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IHttpDiagnosticSourceLocation(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IHttpDiagnosticSourceLocation {}
impl ::core::clone::Clone for IHttpDiagnosticSourceLocation {
    fn clone(&self) -> Self {
        *self
    }
}

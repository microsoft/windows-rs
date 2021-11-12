#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals)]
#[link(name = "windows")]
extern "system" {}
#[repr(transparent)]
pub struct HttpDiagnosticProvider(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct HttpDiagnosticProviderRequestResponseCompletedEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct HttpDiagnosticProviderRequestResponseTimestamps(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct HttpDiagnosticProviderRequestSentEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct HttpDiagnosticProviderResponseReceivedEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct HttpDiagnosticRequestInitiator(pub i32);
impl HttpDiagnosticRequestInitiator {
    pub const ParsedElement: HttpDiagnosticRequestInitiator = HttpDiagnosticRequestInitiator(0i32);
    pub const Script: HttpDiagnosticRequestInitiator = HttpDiagnosticRequestInitiator(1i32);
    pub const Image: HttpDiagnosticRequestInitiator = HttpDiagnosticRequestInitiator(2i32);
    pub const Link: HttpDiagnosticRequestInitiator = HttpDiagnosticRequestInitiator(3i32);
    pub const Style: HttpDiagnosticRequestInitiator = HttpDiagnosticRequestInitiator(4i32);
    pub const XmlHttpRequest: HttpDiagnosticRequestInitiator = HttpDiagnosticRequestInitiator(5i32);
    pub const Media: HttpDiagnosticRequestInitiator = HttpDiagnosticRequestInitiator(6i32);
    pub const HtmlDownload: HttpDiagnosticRequestInitiator = HttpDiagnosticRequestInitiator(7i32);
    pub const Prefetch: HttpDiagnosticRequestInitiator = HttpDiagnosticRequestInitiator(8i32);
    pub const Other: HttpDiagnosticRequestInitiator = HttpDiagnosticRequestInitiator(9i32);
    pub const CrossOriginPreFlight: HttpDiagnosticRequestInitiator = HttpDiagnosticRequestInitiator(10i32);
    pub const Fetch: HttpDiagnosticRequestInitiator = HttpDiagnosticRequestInitiator(11i32);
    pub const Beacon: HttpDiagnosticRequestInitiator = HttpDiagnosticRequestInitiator(12i32);
}
#[repr(transparent)]
pub struct HttpDiagnosticSourceLocation(pub *mut ::core::ffi::c_void);
#[repr(C)]
pub struct HttpDiagnosticsContract(i32);
#[repr(transparent)]
pub struct IHttpDiagnosticProvider(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IHttpDiagnosticProviderRequestResponseCompletedEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IHttpDiagnosticProviderRequestResponseTimestamps(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IHttpDiagnosticProviderRequestSentEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IHttpDiagnosticProviderResponseReceivedEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IHttpDiagnosticProviderStatics(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IHttpDiagnosticSourceLocation(pub *mut ::core::ffi::c_void);

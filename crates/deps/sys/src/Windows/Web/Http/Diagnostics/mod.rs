#![allow(non_snake_case, non_camel_case_types)]
#[link(name = "windows")]
extern "system" {}
pub struct HttpDiagnosticProvider(i32);
pub struct HttpDiagnosticProviderRequestResponseCompletedEventArgs(i32);
pub struct HttpDiagnosticProviderRequestResponseTimestamps(i32);
pub struct HttpDiagnosticProviderRequestSentEventArgs(i32);
pub struct HttpDiagnosticProviderResponseReceivedEventArgs(i32);
pub struct HttpDiagnosticRequestInitiator(i32);
pub struct HttpDiagnosticSourceLocation(i32);
pub struct HttpDiagnosticsContract(i32);
pub struct IHttpDiagnosticProvider(pub *mut ::core::ffi::c_void);
pub struct IHttpDiagnosticProviderRequestResponseCompletedEventArgs(pub *mut ::core::ffi::c_void);
pub struct IHttpDiagnosticProviderRequestResponseTimestamps(pub *mut ::core::ffi::c_void);
pub struct IHttpDiagnosticProviderRequestSentEventArgs(pub *mut ::core::ffi::c_void);
pub struct IHttpDiagnosticProviderResponseReceivedEventArgs(pub *mut ::core::ffi::c_void);
pub struct IHttpDiagnosticProviderStatics(pub *mut ::core::ffi::c_void);
pub struct IHttpDiagnosticSourceLocation(pub *mut ::core::ffi::c_void);

#![allow(non_snake_case, non_camel_case_types)]
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
pub struct HttpDiagnosticRequestInitiator(i32);
#[repr(transparent)]
pub struct HttpDiagnosticSourceLocation(pub *mut ::core::ffi::c_void);
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

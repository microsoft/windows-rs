#![allow(non_snake_case, non_camel_case_types)]
#[link(name = "windows")]
extern "system" {}
pub struct ISysStorageProviderEventReceivedEventArgs(pub *mut ::core::ffi::c_void);
pub struct ISysStorageProviderEventReceivedEventArgsFactory(pub *mut ::core::ffi::c_void);
pub struct ISysStorageProviderEventSource(pub *mut ::core::ffi::c_void);
pub struct ISysStorageProviderHandlerFactory(pub *mut ::core::ffi::c_void);
pub struct ISysStorageProviderHttpRequestProvider(pub *mut ::core::ffi::c_void);
pub struct SysStorageProviderEventReceivedEventArgs(i32);

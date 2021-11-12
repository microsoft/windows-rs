#![allow(non_snake_case, non_camel_case_types)]
#[link(name = "windows")]
extern "system" {}
#[repr(transparent)]
pub struct ISysStorageProviderEventReceivedEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ISysStorageProviderEventReceivedEventArgsFactory(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ISysStorageProviderEventSource(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ISysStorageProviderHandlerFactory(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ISysStorageProviderHttpRequestProvider(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct SysStorageProviderEventReceivedEventArgs(pub *mut ::core::ffi::c_void);

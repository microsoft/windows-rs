#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, clashing_extern_declarations, clippy::all)]
#[link(name = "windows")]
extern "system" {}
#[repr(transparent)]
pub struct ISysStorageProviderEventReceivedEventArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ISysStorageProviderEventReceivedEventArgs {}
impl ::core::clone::Clone for ISysStorageProviderEventReceivedEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ISysStorageProviderEventReceivedEventArgsFactory(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ISysStorageProviderEventReceivedEventArgsFactory {}
impl ::core::clone::Clone for ISysStorageProviderEventReceivedEventArgsFactory {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ISysStorageProviderEventSource(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ISysStorageProviderEventSource {}
impl ::core::clone::Clone for ISysStorageProviderEventSource {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ISysStorageProviderHandlerFactory(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ISysStorageProviderHandlerFactory {}
impl ::core::clone::Clone for ISysStorageProviderHandlerFactory {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ISysStorageProviderHttpRequestProvider(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ISysStorageProviderHttpRequestProvider {}
impl ::core::clone::Clone for ISysStorageProviderHttpRequestProvider {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct SysStorageProviderEventReceivedEventArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for SysStorageProviderEventReceivedEventArgs {}
impl ::core::clone::Clone for SysStorageProviderEventReceivedEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}

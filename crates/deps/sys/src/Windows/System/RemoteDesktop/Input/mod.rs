#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, clashing_extern_declarations, clippy::all)]
#[link(name = "windows")]
extern "system" {}
#[repr(transparent)]
pub struct IRemoteTextConnection(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IRemoteTextConnection {}
impl ::core::clone::Clone for IRemoteTextConnection {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IRemoteTextConnectionFactory(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IRemoteTextConnectionFactory {}
impl ::core::clone::Clone for IRemoteTextConnectionFactory {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct RemoteTextConnection(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for RemoteTextConnection {}
impl ::core::clone::Clone for RemoteTextConnection {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct RemoteTextConnectionDataHandler(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for RemoteTextConnectionDataHandler {}
impl ::core::clone::Clone for RemoteTextConnectionDataHandler {
    fn clone(&self) -> Self {
        *self
    }
}

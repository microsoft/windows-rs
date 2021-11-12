#![allow(non_snake_case, non_camel_case_types)]
#[link(name = "windows")]
extern "system" {}
pub struct IRemoteTextConnection(pub *mut ::core::ffi::c_void);
pub struct IRemoteTextConnectionFactory(pub *mut ::core::ffi::c_void);
pub struct RemoteTextConnection(i32);
pub struct RemoteTextConnectionDataHandler(pub *mut ::core::ffi::c_void);

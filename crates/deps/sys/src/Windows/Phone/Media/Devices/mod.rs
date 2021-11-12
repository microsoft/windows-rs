#![allow(non_snake_case, non_camel_case_types)]
#[link(name = "windows")]
extern "system" {}
pub struct AudioRoutingEndpoint(i32);
#[repr(transparent)]
pub struct AudioRoutingManager(pub *mut ::core::ffi::c_void);
pub struct AvailableAudioRoutingEndpoints(i32);
#[repr(transparent)]
pub struct IAudioRoutingManager(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IAudioRoutingManagerStatics(pub *mut ::core::ffi::c_void);

#![allow(non_snake_case, non_camel_case_types)]
#[link(name = "windows")]
extern "system" {}
pub struct AudioRoutingEndpoint(i32);
pub struct AudioRoutingManager(i32);
pub struct AvailableAudioRoutingEndpoints(i32);
pub struct IAudioRoutingManager(pub *mut ::core::ffi::c_void);
pub struct IAudioRoutingManagerStatics(pub *mut ::core::ffi::c_void);

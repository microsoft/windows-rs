#![allow(non_snake_case, non_camel_case_types)]
#[link(name = "windows")]
extern "system" {}
pub struct AudioRoutingEndpoint(i32);
pub struct AudioRoutingManager(i32);
pub struct AvailableAudioRoutingEndpoints(i32);
pub struct IAudioRoutingManager(i32);
pub struct IAudioRoutingManagerStatics(i32);

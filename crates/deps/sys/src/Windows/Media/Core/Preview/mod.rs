#![allow(non_snake_case, non_camel_case_types)]
#[link(name = "windows")]
extern "system" {}
pub struct ISoundLevelBrokerStatics(pub *mut ::core::ffi::c_void);
pub struct SoundLevelBroker(i32);

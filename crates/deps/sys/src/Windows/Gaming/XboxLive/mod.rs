#![allow(non_snake_case, non_camel_case_types)]
#[cfg(feature = "Gaming_XboxLive_Storage")]
pub mod Storage;
#[link(name = "windows")]
extern "system" {}
#[repr(C)]
pub struct StorageApiContract(i32);

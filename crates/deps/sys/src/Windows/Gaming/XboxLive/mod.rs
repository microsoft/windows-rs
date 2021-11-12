#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals)]
#[cfg(feature = "Gaming_XboxLive_Storage")]
pub mod Storage;
#[link(name = "windows")]
extern "system" {}
#[repr(C)]
pub struct StorageApiContract(i32);

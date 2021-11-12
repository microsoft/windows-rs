#![allow(non_snake_case, non_camel_case_types)]
#[cfg(feature = "Gaming_Preview_GamesEnumeration")]
pub mod GamesEnumeration;
#[link(name = "windows")]
extern "system" {}
#[repr(C)]
pub struct GamesEnumerationContract(i32);

#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals)]
#[cfg(feature = "Gaming_Preview_GamesEnumeration")]
pub mod GamesEnumeration;
#[link(name = "windows")]
extern "system" {}
#[repr(C)]
pub struct GamesEnumerationContract(i32);

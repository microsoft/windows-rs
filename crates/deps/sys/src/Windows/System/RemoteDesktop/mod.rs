#![allow(non_snake_case, non_camel_case_types)]
#[cfg(feature = "System_RemoteDesktop_Input")]
pub mod Input;
#[link(name = "windows")]
extern "system" {}

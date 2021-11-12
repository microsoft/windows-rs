#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals)]
#[cfg(feature = "System_RemoteDesktop_Input")]
pub mod Input;
#[link(name = "windows")]
extern "system" {}
#[repr(transparent)]
pub struct IInteractiveSessionStatics(pub *mut ::core::ffi::c_void);

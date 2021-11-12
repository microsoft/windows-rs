#![allow(non_snake_case, non_camel_case_types)]
#[cfg(feature = "System_RemoteDesktop_Input")]
pub mod Input;
#[link(name = "windows")]
extern "system" {}
#[repr(transparent)]
pub struct IInteractiveSessionStatics(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct InteractiveSession(pub *mut ::core::ffi::c_void);

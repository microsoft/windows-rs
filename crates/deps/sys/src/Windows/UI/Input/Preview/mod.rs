#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals)]
#[cfg(feature = "UI_Input_Preview_Injection")]
pub mod Injection;
#[link(name = "windows")]
extern "system" {}
#[repr(transparent)]
pub struct IInputActivationListenerPreviewStatics(pub *mut ::core::ffi::c_void);

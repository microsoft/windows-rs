#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, clashing_extern_declarations, clippy::all)]
#[cfg(feature = "UI_Input_Preview_Injection")]
pub mod Injection;
#[link(name = "windows")]
extern "system" {}
#[repr(transparent)]
pub struct IInputActivationListenerPreviewStatics(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IInputActivationListenerPreviewStatics {}
impl ::core::clone::Clone for IInputActivationListenerPreviewStatics {
    fn clone(&self) -> Self {
        *self
    }
}

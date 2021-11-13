#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, clashing_extern_declarations, clippy::all)]
#[cfg(feature = "System_RemoteDesktop_Input")]
pub mod Input;
#[link(name = "windows")]
extern "system" {}
#[repr(transparent)]
pub struct IInteractiveSessionStatics(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IInteractiveSessionStatics {}
impl ::core::clone::Clone for IInteractiveSessionStatics {
    fn clone(&self) -> Self {
        *self
    }
}

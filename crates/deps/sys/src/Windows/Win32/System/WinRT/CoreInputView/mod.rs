#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, clashing_extern_declarations, clippy::all)]
#[link(name = "windows")]
extern "system" {}
#[repr(transparent)]
pub struct ICoreFrameworkInputViewInterop(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ICoreFrameworkInputViewInterop {}
impl ::core::clone::Clone for ICoreFrameworkInputViewInterop {
    fn clone(&self) -> Self {
        *self
    }
}

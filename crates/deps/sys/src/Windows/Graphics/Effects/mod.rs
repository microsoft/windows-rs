#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, clashing_extern_declarations, clippy::all)]
#[link(name = "windows")]
extern "system" {}
#[repr(transparent)]
pub struct IGraphicsEffect(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IGraphicsEffect {}
impl ::core::clone::Clone for IGraphicsEffect {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IGraphicsEffectSource(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IGraphicsEffectSource {}
impl ::core::clone::Clone for IGraphicsEffectSource {
    fn clone(&self) -> Self {
        *self
    }
}

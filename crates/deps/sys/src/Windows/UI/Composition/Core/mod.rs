#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, clashing_extern_declarations, clippy::all)]
#[link(name = "windows")]
extern "system" {}
#[repr(transparent)]
pub struct CompositorController(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for CompositorController {}
impl ::core::clone::Clone for CompositorController {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ICompositorController(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ICompositorController {}
impl ::core::clone::Clone for ICompositorController {
    fn clone(&self) -> Self {
        *self
    }
}

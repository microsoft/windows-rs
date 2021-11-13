#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, clashing_extern_declarations, clippy::all)]
#[link(name = "windows")]
extern "system" {}
#[repr(transparent)]
pub struct IDummyHICONIncluder(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IDummyHICONIncluder {}
impl ::core::clone::Clone for IDummyHICONIncluder {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IThumbnailExtractor(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IThumbnailExtractor {}
impl ::core::clone::Clone for IThumbnailExtractor {
    fn clone(&self) -> Self {
        *self
    }
}

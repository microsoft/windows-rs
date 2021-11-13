#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, clashing_extern_declarations, clippy::all)]
#[link(name = "windows")]
extern "system" {}
#[repr(transparent)]
pub struct IPwmControllerProvider(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IPwmControllerProvider {}
impl ::core::clone::Clone for IPwmControllerProvider {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IPwmProvider(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IPwmProvider {}
impl ::core::clone::Clone for IPwmProvider {
    fn clone(&self) -> Self {
        *self
    }
}

#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, clashing_extern_declarations, clippy::all)]
#[link(name = "windows")]
extern "system" {}
pub const CpCreateProcess: i32 = 0i32;
pub const CpCreateProcessAsUser: i32 = 1i32;
pub const CpAicLaunchAdminProcess: i32 = 2i32;
#[repr(transparent)]
pub struct IDDEInitializer(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IDDEInitializer {}
impl ::core::clone::Clone for IDDEInitializer {
    fn clone(&self) -> Self {
        *self
    }
}

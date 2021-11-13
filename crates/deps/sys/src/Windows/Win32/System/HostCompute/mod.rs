#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, clashing_extern_declarations, clippy::all)]
#[link(name = "windows")]
extern "system" {}
#[repr(transparent)]
pub struct HCS_CALLBACK(pub isize);
impl ::core::marker::Copy for HCS_CALLBACK {}
impl ::core::clone::Clone for HCS_CALLBACK {
    fn clone(&self) -> Self {
        *self
    }
}

#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals)]
#[link(name = "windows")]
extern "system" {}
#[repr(transparent)]
pub struct IRetailModeStatics(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IRetailModeStatics {}
impl ::core::clone::Clone for IRetailModeStatics {
    fn clone(&self) -> Self {
        *self
    }
}

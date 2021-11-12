#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals)]
#[link(name = "windows")]
extern "system" {}
#[repr(transparent)]
pub struct DesktopWindowTarget(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for DesktopWindowTarget {}
impl ::core::clone::Clone for DesktopWindowTarget {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IDesktopWindowTarget(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IDesktopWindowTarget {}
impl ::core::clone::Clone for IDesktopWindowTarget {
    fn clone(&self) -> Self {
        *self
    }
}

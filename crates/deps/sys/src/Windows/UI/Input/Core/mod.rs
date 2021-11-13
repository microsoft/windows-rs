#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, clashing_extern_declarations, clippy::all)]
#[link(name = "windows")]
extern "system" {}
#[repr(transparent)]
pub struct IRadialControllerIndependentInputSource(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IRadialControllerIndependentInputSource {}
impl ::core::clone::Clone for IRadialControllerIndependentInputSource {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IRadialControllerIndependentInputSource2(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IRadialControllerIndependentInputSource2 {}
impl ::core::clone::Clone for IRadialControllerIndependentInputSource2 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IRadialControllerIndependentInputSourceStatics(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IRadialControllerIndependentInputSourceStatics {}
impl ::core::clone::Clone for IRadialControllerIndependentInputSourceStatics {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct RadialControllerIndependentInputSource(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for RadialControllerIndependentInputSource {}
impl ::core::clone::Clone for RadialControllerIndependentInputSource {
    fn clone(&self) -> Self {
        *self
    }
}

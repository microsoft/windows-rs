#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals)]
#[link(name = "windows")]
extern "system" {}
#[repr(transparent)]
pub struct IRadialControllerConfigurationInterop(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IRadialControllerConfigurationInterop {}
impl ::core::clone::Clone for IRadialControllerConfigurationInterop {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IRadialControllerIndependentInputSourceInterop(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IRadialControllerIndependentInputSourceInterop {}
impl ::core::clone::Clone for IRadialControllerIndependentInputSourceInterop {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IRadialControllerInterop(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IRadialControllerInterop {}
impl ::core::clone::Clone for IRadialControllerInterop {
    fn clone(&self) -> Self {
        *self
    }
}

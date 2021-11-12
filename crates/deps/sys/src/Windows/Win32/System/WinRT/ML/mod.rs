#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals)]
#[link(name = "windows")]
extern "system" {}
#[repr(transparent)]
pub struct ILearningModelDeviceFactoryNative(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ILearningModelDeviceFactoryNative {}
impl ::core::clone::Clone for ILearningModelDeviceFactoryNative {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ILearningModelOperatorProviderNative(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ILearningModelOperatorProviderNative {}
impl ::core::clone::Clone for ILearningModelOperatorProviderNative {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ILearningModelSessionOptionsNative(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ILearningModelSessionOptionsNative {}
impl ::core::clone::Clone for ILearningModelSessionOptionsNative {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ITensorNative(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ITensorNative {}
impl ::core::clone::Clone for ITensorNative {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ITensorStaticsNative(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ITensorStaticsNative {}
impl ::core::clone::Clone for ITensorStaticsNative {
    fn clone(&self) -> Self {
        *self
    }
}

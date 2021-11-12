#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals)]
#[link(name = "windows")]
extern "system" {}
#[repr(transparent)]
pub struct IVariablePhotoCapturedEventArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IVariablePhotoCapturedEventArgs {}
impl ::core::clone::Clone for IVariablePhotoCapturedEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IVariablePhotoSequenceCapture(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IVariablePhotoSequenceCapture {}
impl ::core::clone::Clone for IVariablePhotoSequenceCapture {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IVariablePhotoSequenceCapture2(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IVariablePhotoSequenceCapture2 {}
impl ::core::clone::Clone for IVariablePhotoSequenceCapture2 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct VariablePhotoCapturedEventArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for VariablePhotoCapturedEventArgs {}
impl ::core::clone::Clone for VariablePhotoCapturedEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct VariablePhotoSequenceCapture(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for VariablePhotoSequenceCapture {}
impl ::core::clone::Clone for VariablePhotoSequenceCapture {
    fn clone(&self) -> Self {
        *self
    }
}

#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals)]
#[link(name = "windows")]
extern "system" {}
#[repr(transparent)]
pub struct IVariablePhotoCapturedEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IVariablePhotoSequenceCapture(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IVariablePhotoSequenceCapture2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct VariablePhotoCapturedEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct VariablePhotoSequenceCapture(pub *mut ::core::ffi::c_void);

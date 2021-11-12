#![allow(non_snake_case, non_camel_case_types)]
#[link(name = "windows")]
extern "system" {}
pub struct IVariablePhotoCapturedEventArgs(pub *mut ::core::ffi::c_void);
pub struct IVariablePhotoSequenceCapture(pub *mut ::core::ffi::c_void);
pub struct IVariablePhotoSequenceCapture2(pub *mut ::core::ffi::c_void);
pub struct VariablePhotoCapturedEventArgs(i32);
pub struct VariablePhotoSequenceCapture(i32);

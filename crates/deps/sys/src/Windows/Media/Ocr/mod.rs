#![allow(non_snake_case, non_camel_case_types)]
#[link(name = "windows")]
extern "system" {}
#[repr(transparent)]
pub struct IOcrEngine(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IOcrEngineStatics(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IOcrLine(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IOcrResult(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IOcrWord(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct OcrEngine(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct OcrLine(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct OcrResult(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct OcrWord(pub *mut ::core::ffi::c_void);

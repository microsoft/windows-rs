#![allow(non_snake_case, non_camel_case_types)]
#[link(name = "windows")]
extern "system" {}
pub struct IOcrEngine(pub *mut ::core::ffi::c_void);
pub struct IOcrEngineStatics(pub *mut ::core::ffi::c_void);
pub struct IOcrLine(pub *mut ::core::ffi::c_void);
pub struct IOcrResult(pub *mut ::core::ffi::c_void);
pub struct IOcrWord(pub *mut ::core::ffi::c_void);
pub struct OcrEngine(i32);
pub struct OcrLine(i32);
pub struct OcrResult(i32);
pub struct OcrWord(i32);

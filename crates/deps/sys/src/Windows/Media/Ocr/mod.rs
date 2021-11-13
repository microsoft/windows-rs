#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, clashing_extern_declarations, clippy::all)]
#[link(name = "windows")]
extern "system" {}
#[repr(transparent)]
pub struct IOcrEngine(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IOcrEngine {}
impl ::core::clone::Clone for IOcrEngine {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IOcrEngineStatics(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IOcrEngineStatics {}
impl ::core::clone::Clone for IOcrEngineStatics {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IOcrLine(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IOcrLine {}
impl ::core::clone::Clone for IOcrLine {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IOcrResult(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IOcrResult {}
impl ::core::clone::Clone for IOcrResult {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IOcrWord(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IOcrWord {}
impl ::core::clone::Clone for IOcrWord {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct OcrEngine(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for OcrEngine {}
impl ::core::clone::Clone for OcrEngine {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct OcrLine(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for OcrLine {}
impl ::core::clone::Clone for OcrLine {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct OcrResult(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for OcrResult {}
impl ::core::clone::Clone for OcrResult {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct OcrWord(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for OcrWord {}
impl ::core::clone::Clone for OcrWord {
    fn clone(&self) -> Self {
        *self
    }
}

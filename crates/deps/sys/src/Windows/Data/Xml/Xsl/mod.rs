#![allow(non_snake_case, non_camel_case_types)]
#[link(name = "windows")]
extern "system" {}
pub struct IXsltProcessor(pub *mut ::core::ffi::c_void);
pub struct IXsltProcessor2(pub *mut ::core::ffi::c_void);
pub struct IXsltProcessorFactory(pub *mut ::core::ffi::c_void);
pub struct XsltProcessor(i32);

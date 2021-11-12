#![allow(non_snake_case, non_camel_case_types)]
#[link(name = "windows")]
extern "system" {}
#[repr(transparent)]
pub struct IXsltProcessor(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IXsltProcessor2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IXsltProcessorFactory(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct XsltProcessor(pub *mut ::core::ffi::c_void);

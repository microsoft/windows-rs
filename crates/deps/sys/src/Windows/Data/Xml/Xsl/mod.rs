#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals)]
#[link(name = "windows")]
extern "system" {}
#[repr(transparent)]
pub struct IXsltProcessor(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IXsltProcessor {}
impl ::core::clone::Clone for IXsltProcessor {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IXsltProcessor2(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IXsltProcessor2 {}
impl ::core::clone::Clone for IXsltProcessor2 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IXsltProcessorFactory(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IXsltProcessorFactory {}
impl ::core::clone::Clone for IXsltProcessorFactory {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct XsltProcessor(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for XsltProcessor {}
impl ::core::clone::Clone for XsltProcessor {
    fn clone(&self) -> Self {
        *self
    }
}

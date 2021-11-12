#![allow(non_snake_case, non_camel_case_types)]
#[link(name = "windows")]
extern "system" {}
#[repr(transparent)]
pub struct IPdfDocument(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IPdfDocumentStatics(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IPdfPage(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IPdfPageDimensions(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IPdfPageRenderOptions(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct PdfDocument(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct PdfPage(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct PdfPageDimensions(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct PdfPageRenderOptions(pub *mut ::core::ffi::c_void);
#[repr(C)]
pub struct PdfPageRotation(i32);

#![allow(non_snake_case, non_camel_case_types)]
#[link(name = "windows")]
extern "system" {}
pub struct IPdfDocument(pub *mut ::core::ffi::c_void);
pub struct IPdfDocumentStatics(pub *mut ::core::ffi::c_void);
pub struct IPdfPage(pub *mut ::core::ffi::c_void);
pub struct IPdfPageDimensions(pub *mut ::core::ffi::c_void);
pub struct IPdfPageRenderOptions(pub *mut ::core::ffi::c_void);
pub struct PdfDocument(i32);
pub struct PdfPage(i32);
pub struct PdfPageDimensions(i32);
pub struct PdfPageRenderOptions(i32);
pub struct PdfPageRotation(i32);

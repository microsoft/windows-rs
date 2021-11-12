#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals)]
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
#[repr(transparent)]
pub struct PdfPageRotation(pub i32);
impl PdfPageRotation {
    pub const Normal: Self = Self(0i32);
    pub const Rotate90: Self = Self(1i32);
    pub const Rotate180: Self = Self(2i32);
    pub const Rotate270: Self = Self(3i32);
}
impl ::core::marker::Copy for PdfPageRotation {}
impl ::core::clone::Clone for PdfPageRotation {
    fn clone(&self) -> Self {
        *self
    }
}

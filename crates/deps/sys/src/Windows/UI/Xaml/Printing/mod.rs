#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals)]
#[link(name = "windows")]
extern "system" {}
#[repr(transparent)]
pub struct AddPagesEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct AddPagesEventHandler(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct GetPreviewPageEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct GetPreviewPageEventHandler(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IAddPagesEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IGetPreviewPageEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IPaginateEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IPrintDocument(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IPrintDocumentFactory(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IPrintDocumentStatics(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct PaginateEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct PaginateEventHandler(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct PreviewPageCountType(pub i32);
impl PreviewPageCountType {
    pub const Final: PreviewPageCountType = PreviewPageCountType(0i32);
    pub const Intermediate: PreviewPageCountType = PreviewPageCountType(1i32);
}
#[repr(transparent)]
pub struct PrintDocument(pub *mut ::core::ffi::c_void);

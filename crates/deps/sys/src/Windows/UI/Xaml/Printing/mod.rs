#![allow(non_snake_case, non_camel_case_types)]
#[link(name = "windows")]
extern "system" {}
pub struct AddPagesEventArgs(i32);
pub struct AddPagesEventHandler(pub *mut ::core::ffi::c_void);
pub struct GetPreviewPageEventArgs(i32);
pub struct GetPreviewPageEventHandler(pub *mut ::core::ffi::c_void);
pub struct IAddPagesEventArgs(pub *mut ::core::ffi::c_void);
pub struct IGetPreviewPageEventArgs(pub *mut ::core::ffi::c_void);
pub struct IPaginateEventArgs(pub *mut ::core::ffi::c_void);
pub struct IPrintDocument(pub *mut ::core::ffi::c_void);
pub struct IPrintDocumentFactory(pub *mut ::core::ffi::c_void);
pub struct IPrintDocumentStatics(pub *mut ::core::ffi::c_void);
pub struct PaginateEventArgs(i32);
pub struct PaginateEventHandler(pub *mut ::core::ffi::c_void);
pub struct PreviewPageCountType(i32);
pub struct PrintDocument(i32);

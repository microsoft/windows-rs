#![allow(non_snake_case, non_camel_case_types)]
#[link(name = "windows")]
extern "system" {}
pub struct AddPagesEventArgs(i32);
pub struct AddPagesEventHandler(i32);
pub struct GetPreviewPageEventArgs(i32);
pub struct GetPreviewPageEventHandler(i32);
pub struct IAddPagesEventArgs(i32);
pub struct IGetPreviewPageEventArgs(i32);
pub struct IPaginateEventArgs(i32);
pub struct IPrintDocument(i32);
pub struct IPrintDocumentFactory(i32);
pub struct IPrintDocumentStatics(i32);
pub struct PaginateEventArgs(i32);
pub struct PaginateEventHandler(i32);
pub struct PreviewPageCountType(i32);
pub struct PrintDocument(i32);

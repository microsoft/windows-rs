#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals)]
#[link(name = "windows")]
extern "system" {}
#[repr(transparent)]
pub struct AddPagesEventArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for AddPagesEventArgs {}
impl ::core::clone::Clone for AddPagesEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct AddPagesEventHandler(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for AddPagesEventHandler {}
impl ::core::clone::Clone for AddPagesEventHandler {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct GetPreviewPageEventArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for GetPreviewPageEventArgs {}
impl ::core::clone::Clone for GetPreviewPageEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct GetPreviewPageEventHandler(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for GetPreviewPageEventHandler {}
impl ::core::clone::Clone for GetPreviewPageEventHandler {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IAddPagesEventArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IAddPagesEventArgs {}
impl ::core::clone::Clone for IAddPagesEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IGetPreviewPageEventArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IGetPreviewPageEventArgs {}
impl ::core::clone::Clone for IGetPreviewPageEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IPaginateEventArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IPaginateEventArgs {}
impl ::core::clone::Clone for IPaginateEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IPrintDocument(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IPrintDocument {}
impl ::core::clone::Clone for IPrintDocument {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IPrintDocumentFactory(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IPrintDocumentFactory {}
impl ::core::clone::Clone for IPrintDocumentFactory {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IPrintDocumentStatics(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IPrintDocumentStatics {}
impl ::core::clone::Clone for IPrintDocumentStatics {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct PaginateEventArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for PaginateEventArgs {}
impl ::core::clone::Clone for PaginateEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct PaginateEventHandler(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for PaginateEventHandler {}
impl ::core::clone::Clone for PaginateEventHandler {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct PreviewPageCountType(pub i32);
impl PreviewPageCountType {
    pub const Final: Self = Self(0i32);
    pub const Intermediate: Self = Self(1i32);
}
impl ::core::marker::Copy for PreviewPageCountType {}
impl ::core::clone::Clone for PreviewPageCountType {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct PrintDocument(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for PrintDocument {}
impl ::core::clone::Clone for PrintDocument {
    fn clone(&self) -> Self {
        *self
    }
}

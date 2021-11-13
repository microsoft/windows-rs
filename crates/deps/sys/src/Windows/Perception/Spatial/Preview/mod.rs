#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, clashing_extern_declarations, clippy::all)]
#[link(name = "windows")]
extern "system" {}
#[repr(transparent)]
pub struct ISpatialGraphInteropFrameOfReferencePreview(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ISpatialGraphInteropFrameOfReferencePreview {}
impl ::core::clone::Clone for ISpatialGraphInteropFrameOfReferencePreview {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ISpatialGraphInteropPreviewStatics(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ISpatialGraphInteropPreviewStatics {}
impl ::core::clone::Clone for ISpatialGraphInteropPreviewStatics {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ISpatialGraphInteropPreviewStatics2(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ISpatialGraphInteropPreviewStatics2 {}
impl ::core::clone::Clone for ISpatialGraphInteropPreviewStatics2 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct SpatialGraphInteropFrameOfReferencePreview(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for SpatialGraphInteropFrameOfReferencePreview {}
impl ::core::clone::Clone for SpatialGraphInteropFrameOfReferencePreview {
    fn clone(&self) -> Self {
        *self
    }
}

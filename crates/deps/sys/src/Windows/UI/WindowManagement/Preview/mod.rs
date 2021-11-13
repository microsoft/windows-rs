#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, clashing_extern_declarations, clippy::all)]
#[link(name = "windows")]
extern "system" {}
#[repr(transparent)]
pub struct IWindowManagementPreview(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IWindowManagementPreview {}
impl ::core::clone::Clone for IWindowManagementPreview {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IWindowManagementPreviewStatics(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IWindowManagementPreviewStatics {}
impl ::core::clone::Clone for IWindowManagementPreviewStatics {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct WindowManagementPreview(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for WindowManagementPreview {}
impl ::core::clone::Clone for WindowManagementPreview {
    fn clone(&self) -> Self {
        *self
    }
}

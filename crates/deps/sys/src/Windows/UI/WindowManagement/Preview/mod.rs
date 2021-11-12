#![allow(non_snake_case, non_camel_case_types)]
#[link(name = "windows")]
extern "system" {}
#[repr(transparent)]
pub struct IWindowManagementPreview(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IWindowManagementPreviewStatics(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct WindowManagementPreview(pub *mut ::core::ffi::c_void);

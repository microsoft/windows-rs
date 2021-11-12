#![allow(non_snake_case, non_camel_case_types)]
#[link(name = "windows")]
extern "system" {}
pub struct IWindowManagementPreview(pub *mut ::core::ffi::c_void);
pub struct IWindowManagementPreviewStatics(pub *mut ::core::ffi::c_void);
pub struct WindowManagementPreview(i32);

#![allow(non_snake_case, non_camel_case_types)]
#[link(name = "windows")]
extern "system" {}
pub struct IPreviewBuildsManager(pub *mut ::core::ffi::c_void);
pub struct IPreviewBuildsManagerStatics(pub *mut ::core::ffi::c_void);
pub struct IPreviewBuildsState(pub *mut ::core::ffi::c_void);
pub struct PreviewBuildsManager(i32);
pub struct PreviewBuildsState(i32);

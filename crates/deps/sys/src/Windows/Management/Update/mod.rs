#![allow(non_snake_case, non_camel_case_types)]
#[link(name = "windows")]
extern "system" {}
#[repr(transparent)]
pub struct IPreviewBuildsManager(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IPreviewBuildsManagerStatics(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IPreviewBuildsState(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct PreviewBuildsManager(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct PreviewBuildsState(pub *mut ::core::ffi::c_void);

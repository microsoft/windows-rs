#![allow(non_snake_case, non_camel_case_types)]
#[link(name = "windows")]
extern "system" {}
pub struct CoreAppWindowPreview(i32);
pub struct ICoreAppWindowPreview(pub *mut ::core::ffi::c_void);
pub struct ICoreAppWindowPreviewStatics(pub *mut ::core::ffi::c_void);
pub struct ISystemNavigationCloseRequestedPreviewEventArgs(pub *mut ::core::ffi::c_void);
pub struct ISystemNavigationManagerPreview(pub *mut ::core::ffi::c_void);
pub struct ISystemNavigationManagerPreviewStatics(pub *mut ::core::ffi::c_void);
pub struct SystemNavigationCloseRequestedPreviewEventArgs(i32);
pub struct SystemNavigationManagerPreview(i32);

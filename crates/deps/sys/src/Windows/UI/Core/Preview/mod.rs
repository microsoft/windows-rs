#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals)]
#[link(name = "windows")]
extern "system" {}
#[repr(transparent)]
pub struct CoreAppWindowPreview(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ICoreAppWindowPreview(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ICoreAppWindowPreviewStatics(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ISystemNavigationCloseRequestedPreviewEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ISystemNavigationManagerPreview(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ISystemNavigationManagerPreviewStatics(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct SystemNavigationCloseRequestedPreviewEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct SystemNavigationManagerPreview(pub *mut ::core::ffi::c_void);

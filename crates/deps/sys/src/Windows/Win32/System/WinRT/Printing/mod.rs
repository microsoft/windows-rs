#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals)]
#[link(name = "windows")]
extern "system" {}
#[repr(transparent)]
pub struct IPrintManagerInterop(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IPrintWorkflowConfigurationNative(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IPrintWorkflowObjectModelSourceFileContentNative(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IPrintWorkflowXpsObjectModelTargetPackageNative(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IPrintWorkflowXpsReceiver(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IPrintWorkflowXpsReceiver2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IPrinting3DManagerInterop(pub *mut ::core::ffi::c_void);

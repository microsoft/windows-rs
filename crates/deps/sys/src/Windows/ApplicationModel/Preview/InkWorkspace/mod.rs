#![allow(non_snake_case, non_camel_case_types)]
#[link(name = "windows")]
extern "system" {}
#[repr(transparent)]
pub struct IInkWorkspaceHostedAppManager(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IInkWorkspaceHostedAppManagerStatics(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct InkWorkspaceHostedAppManager(pub *mut ::core::ffi::c_void);
pub struct PreviewInkWorkspaceContract(i32);

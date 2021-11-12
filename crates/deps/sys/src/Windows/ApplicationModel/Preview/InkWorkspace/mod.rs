#![allow(non_snake_case, non_camel_case_types)]
#[link(name = "windows")]
extern "system" {}
pub struct IInkWorkspaceHostedAppManager(pub *mut ::core::ffi::c_void);
pub struct IInkWorkspaceHostedAppManagerStatics(pub *mut ::core::ffi::c_void);
pub struct InkWorkspaceHostedAppManager(i32);
pub struct PreviewInkWorkspaceContract(i32);

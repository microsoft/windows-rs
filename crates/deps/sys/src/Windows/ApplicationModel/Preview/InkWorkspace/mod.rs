#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, clashing_extern_declarations, clippy::all)]
#[link(name = "windows")]
extern "system" {}
#[repr(transparent)]
pub struct IInkWorkspaceHostedAppManager(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IInkWorkspaceHostedAppManager {}
impl ::core::clone::Clone for IInkWorkspaceHostedAppManager {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IInkWorkspaceHostedAppManagerStatics(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IInkWorkspaceHostedAppManagerStatics {}
impl ::core::clone::Clone for IInkWorkspaceHostedAppManagerStatics {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct InkWorkspaceHostedAppManager(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for InkWorkspaceHostedAppManager {}
impl ::core::clone::Clone for InkWorkspaceHostedAppManager {
    fn clone(&self) -> Self {
        *self
    }
}

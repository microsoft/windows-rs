#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals)]
#[link(name = "windows")]
extern "system" {}
#[repr(transparent)]
pub struct CoreAppWindowPreview(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for CoreAppWindowPreview {}
impl ::core::clone::Clone for CoreAppWindowPreview {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ICoreAppWindowPreview(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ICoreAppWindowPreview {}
impl ::core::clone::Clone for ICoreAppWindowPreview {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ICoreAppWindowPreviewStatics(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ICoreAppWindowPreviewStatics {}
impl ::core::clone::Clone for ICoreAppWindowPreviewStatics {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ISystemNavigationCloseRequestedPreviewEventArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ISystemNavigationCloseRequestedPreviewEventArgs {}
impl ::core::clone::Clone for ISystemNavigationCloseRequestedPreviewEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ISystemNavigationManagerPreview(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ISystemNavigationManagerPreview {}
impl ::core::clone::Clone for ISystemNavigationManagerPreview {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ISystemNavigationManagerPreviewStatics(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ISystemNavigationManagerPreviewStatics {}
impl ::core::clone::Clone for ISystemNavigationManagerPreviewStatics {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct SystemNavigationCloseRequestedPreviewEventArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for SystemNavigationCloseRequestedPreviewEventArgs {}
impl ::core::clone::Clone for SystemNavigationCloseRequestedPreviewEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct SystemNavigationManagerPreview(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for SystemNavigationManagerPreview {}
impl ::core::clone::Clone for SystemNavigationManagerPreview {
    fn clone(&self) -> Self {
        *self
    }
}

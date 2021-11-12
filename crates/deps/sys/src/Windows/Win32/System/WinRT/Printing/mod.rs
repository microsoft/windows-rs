#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals)]
#[link(name = "windows")]
extern "system" {}
#[repr(transparent)]
pub struct IPrintManagerInterop(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IPrintManagerInterop {}
impl ::core::clone::Clone for IPrintManagerInterop {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IPrintWorkflowConfigurationNative(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IPrintWorkflowConfigurationNative {}
impl ::core::clone::Clone for IPrintWorkflowConfigurationNative {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IPrintWorkflowObjectModelSourceFileContentNative(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IPrintWorkflowObjectModelSourceFileContentNative {}
impl ::core::clone::Clone for IPrintWorkflowObjectModelSourceFileContentNative {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IPrintWorkflowXpsObjectModelTargetPackageNative(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IPrintWorkflowXpsObjectModelTargetPackageNative {}
impl ::core::clone::Clone for IPrintWorkflowXpsObjectModelTargetPackageNative {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IPrintWorkflowXpsReceiver(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IPrintWorkflowXpsReceiver {}
impl ::core::clone::Clone for IPrintWorkflowXpsReceiver {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IPrintWorkflowXpsReceiver2(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IPrintWorkflowXpsReceiver2 {}
impl ::core::clone::Clone for IPrintWorkflowXpsReceiver2 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IPrinting3DManagerInterop(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IPrinting3DManagerInterop {}
impl ::core::clone::Clone for IPrinting3DManagerInterop {
    fn clone(&self) -> Self {
        *self
    }
}

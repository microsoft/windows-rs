#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals)]
#[link(name = "windows")]
extern "system" {}
pub const E_SURFACE_CONTENTS_LOST: u32 = 2150301728u32;
#[repr(transparent)]
pub struct IDesktopWindowXamlSourceNative(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IDesktopWindowXamlSourceNative2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IFindReferenceTargetsCallback(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IReferenceTracker(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IReferenceTrackerExtension(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IReferenceTrackerHost(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IReferenceTrackerManager(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IReferenceTrackerTarget(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ISurfaceImageSourceManagerNative(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ISurfaceImageSourceNative(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ISurfaceImageSourceNativeWithD2D(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ISwapChainBackgroundPanelNative(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ISwapChainPanelNative(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ISwapChainPanelNative2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ITrackerOwner(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IVirtualSurfaceImageSourceNative(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IVirtualSurfaceUpdatesCallbackNative(pub *mut ::core::ffi::c_void);
#[repr(C)]
pub struct TrackerHandle__ {
    pub unused: i32,
}
impl ::core::marker::Copy for TrackerHandle__ {}
impl ::core::clone::Clone for TrackerHandle__ {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct XAML_REFERENCETRACKER_DISCONNECT(pub i32);
pub const XAML_REFERENCETRACKER_DISCONNECT_DEFAULT: XAML_REFERENCETRACKER_DISCONNECT = XAML_REFERENCETRACKER_DISCONNECT(0i32);
pub const XAML_REFERENCETRACKER_DISCONNECT_SUSPEND: XAML_REFERENCETRACKER_DISCONNECT = XAML_REFERENCETRACKER_DISCONNECT(1i32);
impl ::core::marker::Copy for XAML_REFERENCETRACKER_DISCONNECT {}
impl ::core::clone::Clone for XAML_REFERENCETRACKER_DISCONNECT {
    fn clone(&self) -> Self {
        *self
    }
}

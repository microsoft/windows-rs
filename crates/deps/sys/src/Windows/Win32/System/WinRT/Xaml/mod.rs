#![allow(non_snake_case, non_camel_case_types)]
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
pub struct TrackerHandle__(i32);
pub struct XAML_REFERENCETRACKER_DISCONNECT(i32);

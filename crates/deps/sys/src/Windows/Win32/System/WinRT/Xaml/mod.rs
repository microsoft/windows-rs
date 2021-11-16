#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, clashing_extern_declarations, clippy::all)]
#[link(name = "windows")]
extern "system" {}
pub const E_SURFACE_CONTENTS_LOST: u32 = 2150301728u32;
#[repr(transparent)]
pub struct IDesktopWindowXamlSourceNative(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IDesktopWindowXamlSourceNative {}
impl ::core::clone::Clone for IDesktopWindowXamlSourceNative {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IDesktopWindowXamlSourceNative2(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IDesktopWindowXamlSourceNative2 {}
impl ::core::clone::Clone for IDesktopWindowXamlSourceNative2 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IFindReferenceTargetsCallback(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IFindReferenceTargetsCallback {}
impl ::core::clone::Clone for IFindReferenceTargetsCallback {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IReferenceTracker(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IReferenceTracker {}
impl ::core::clone::Clone for IReferenceTracker {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IReferenceTrackerExtension(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IReferenceTrackerExtension {}
impl ::core::clone::Clone for IReferenceTrackerExtension {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IReferenceTrackerHost(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IReferenceTrackerHost {}
impl ::core::clone::Clone for IReferenceTrackerHost {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IReferenceTrackerManager(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IReferenceTrackerManager {}
impl ::core::clone::Clone for IReferenceTrackerManager {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IReferenceTrackerTarget(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IReferenceTrackerTarget {}
impl ::core::clone::Clone for IReferenceTrackerTarget {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ISurfaceImageSourceManagerNative(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ISurfaceImageSourceManagerNative {}
impl ::core::clone::Clone for ISurfaceImageSourceManagerNative {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ISurfaceImageSourceNative(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ISurfaceImageSourceNative {}
impl ::core::clone::Clone for ISurfaceImageSourceNative {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ISurfaceImageSourceNativeWithD2D(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ISurfaceImageSourceNativeWithD2D {}
impl ::core::clone::Clone for ISurfaceImageSourceNativeWithD2D {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ISwapChainBackgroundPanelNative(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ISwapChainBackgroundPanelNative {}
impl ::core::clone::Clone for ISwapChainBackgroundPanelNative {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ISwapChainPanelNative(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ISwapChainPanelNative {}
impl ::core::clone::Clone for ISwapChainPanelNative {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ISwapChainPanelNative2(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ISwapChainPanelNative2 {}
impl ::core::clone::Clone for ISwapChainPanelNative2 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ITrackerOwner(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ITrackerOwner {}
impl ::core::clone::Clone for ITrackerOwner {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IVirtualSurfaceImageSourceNative(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IVirtualSurfaceImageSourceNative {}
impl ::core::clone::Clone for IVirtualSurfaceImageSourceNative {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IVirtualSurfaceUpdatesCallbackNative(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IVirtualSurfaceUpdatesCallbackNative {}
impl ::core::clone::Clone for IVirtualSurfaceUpdatesCallbackNative {
    fn clone(&self) -> Self {
        *self
    }
}
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
pub const XAML_REFERENCETRACKER_DISCONNECT_DEFAULT: i32 = 0i32;
pub const XAML_REFERENCETRACKER_DISCONNECT_SUSPEND: i32 = 1i32;

#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals)]
#[link(name = "windows")]
extern "system" {
    #[cfg(feature = "Win32_Foundation")]
    pub fn DCompositionAttachMouseDragToHwnd(visual: IDCompositionVisual, hwnd: super::super::Foundation::HWND, enable: super::super::Foundation::BOOL) -> ::windows_sys::core::HRESULT;
    #[cfg(feature = "Win32_Foundation")]
    pub fn DCompositionAttachMouseWheelToHwnd(visual: IDCompositionVisual, hwnd: super::super::Foundation::HWND, enable: super::super::Foundation::BOOL) -> ::windows_sys::core::HRESULT;
    #[cfg(feature = "Win32_Foundation")]
    pub fn DCompositionBoostCompositorClock(enable: super::super::Foundation::BOOL) -> ::windows_sys::core::HRESULT;
    #[cfg(feature = "Win32_Graphics_Dxgi")]
    pub fn DCompositionCreateDevice(dxgidevice: super::Dxgi::IDXGIDevice, iid: *const ::windows_sys::core::GUID, dcompositiondevice: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT;
    pub fn DCompositionCreateDevice2(renderingdevice: ::windows_sys::core::IUnknown, iid: *const ::windows_sys::core::GUID, dcompositiondevice: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT;
    pub fn DCompositionCreateDevice3(renderingdevice: ::windows_sys::core::IUnknown, iid: *const ::windows_sys::core::GUID, dcompositiondevice: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT;
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
    pub fn DCompositionCreateSurfaceHandle(desiredaccess: u32, securityattributes: *const super::super::Security::SECURITY_ATTRIBUTES, surfacehandle: *mut super::super::Foundation::HANDLE) -> ::windows_sys::core::HRESULT;
    pub fn DCompositionGetFrameId(frameidtype: COMPOSITION_FRAME_ID_TYPE, frameid: *mut u64) -> ::windows_sys::core::HRESULT;
    #[cfg(feature = "Win32_Foundation")]
    pub fn DCompositionGetStatistics(frameid: u64, framestats: *mut COMPOSITION_FRAME_STATS, targetidcount: u32, targetids: *mut COMPOSITION_TARGET_ID, actualtargetidcount: *mut u32) -> ::windows_sys::core::HRESULT;
    #[cfg(feature = "Win32_Foundation")]
    pub fn DCompositionGetTargetStatistics(frameid: u64, targetid: *const COMPOSITION_TARGET_ID, targetstats: *mut COMPOSITION_TARGET_STATS) -> ::windows_sys::core::HRESULT;
    #[cfg(feature = "Win32_Foundation")]
    pub fn DCompositionWaitForCompositorClock(count: u32, handles: *const super::super::Foundation::HANDLE, timeoutinms: u32) -> u32;
}
pub const COMPOSITIONOBJECT_READ: i32 = 1i32;
pub const COMPOSITIONOBJECT_WRITE: i32 = 2i32;
#[repr(transparent)]
pub struct COMPOSITION_FRAME_ID_TYPE(pub i32);
pub const COMPOSITION_FRAME_ID_CREATED: COMPOSITION_FRAME_ID_TYPE = COMPOSITION_FRAME_ID_TYPE(0i32);
pub const COMPOSITION_FRAME_ID_CONFIRMED: COMPOSITION_FRAME_ID_TYPE = COMPOSITION_FRAME_ID_TYPE(1i32);
pub const COMPOSITION_FRAME_ID_COMPLETED: COMPOSITION_FRAME_ID_TYPE = COMPOSITION_FRAME_ID_TYPE(2i32);
impl ::core::marker::Copy for COMPOSITION_FRAME_ID_TYPE {}
impl ::core::clone::Clone for COMPOSITION_FRAME_ID_TYPE {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct COMPOSITION_FRAME_STATS {
    pub startTime: u64,
    pub targetTime: u64,
    pub framePeriod: u64,
}
impl ::core::marker::Copy for COMPOSITION_FRAME_STATS {}
impl ::core::clone::Clone for COMPOSITION_FRAME_STATS {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct COMPOSITION_STATS {
    pub presentCount: u32,
    pub refreshCount: u32,
    pub virtualRefreshCount: u32,
    pub time: u64,
}
impl ::core::marker::Copy for COMPOSITION_STATS {}
impl ::core::clone::Clone for COMPOSITION_STATS {
    fn clone(&self) -> Self {
        *self
    }
}
pub const COMPOSITION_STATS_MAX_TARGETS: u32 = 256u32;
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct COMPOSITION_TARGET_ID {
    pub displayAdapterLuid: super::super::Foundation::LUID,
    pub renderAdapterLuid: super::super::Foundation::LUID,
    pub vidPnSourceId: u32,
    pub vidPnTargetId: u32,
    pub uniqueId: u32,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for COMPOSITION_TARGET_ID {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for COMPOSITION_TARGET_ID {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct COMPOSITION_TARGET_STATS {
    pub outstandingPresents: u32,
    pub presentTime: u64,
    pub vblankDuration: u64,
    pub presentedStats: COMPOSITION_STATS,
    pub completedStats: COMPOSITION_STATS,
}
impl ::core::marker::Copy for COMPOSITION_TARGET_STATS {}
impl ::core::clone::Clone for COMPOSITION_TARGET_STATS {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct DCOMPOSITION_BACKFACE_VISIBILITY(pub i32);
pub const DCOMPOSITION_BACKFACE_VISIBILITY_VISIBLE: DCOMPOSITION_BACKFACE_VISIBILITY = DCOMPOSITION_BACKFACE_VISIBILITY(0i32);
pub const DCOMPOSITION_BACKFACE_VISIBILITY_HIDDEN: DCOMPOSITION_BACKFACE_VISIBILITY = DCOMPOSITION_BACKFACE_VISIBILITY(1i32);
pub const DCOMPOSITION_BACKFACE_VISIBILITY_INHERIT: DCOMPOSITION_BACKFACE_VISIBILITY = DCOMPOSITION_BACKFACE_VISIBILITY(-1i32);
impl ::core::marker::Copy for DCOMPOSITION_BACKFACE_VISIBILITY {}
impl ::core::clone::Clone for DCOMPOSITION_BACKFACE_VISIBILITY {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct DCOMPOSITION_BITMAP_INTERPOLATION_MODE(pub i32);
pub const DCOMPOSITION_BITMAP_INTERPOLATION_MODE_NEAREST_NEIGHBOR: DCOMPOSITION_BITMAP_INTERPOLATION_MODE = DCOMPOSITION_BITMAP_INTERPOLATION_MODE(0i32);
pub const DCOMPOSITION_BITMAP_INTERPOLATION_MODE_LINEAR: DCOMPOSITION_BITMAP_INTERPOLATION_MODE = DCOMPOSITION_BITMAP_INTERPOLATION_MODE(1i32);
pub const DCOMPOSITION_BITMAP_INTERPOLATION_MODE_INHERIT: DCOMPOSITION_BITMAP_INTERPOLATION_MODE = DCOMPOSITION_BITMAP_INTERPOLATION_MODE(-1i32);
impl ::core::marker::Copy for DCOMPOSITION_BITMAP_INTERPOLATION_MODE {}
impl ::core::clone::Clone for DCOMPOSITION_BITMAP_INTERPOLATION_MODE {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct DCOMPOSITION_BORDER_MODE(pub i32);
pub const DCOMPOSITION_BORDER_MODE_SOFT: DCOMPOSITION_BORDER_MODE = DCOMPOSITION_BORDER_MODE(0i32);
pub const DCOMPOSITION_BORDER_MODE_HARD: DCOMPOSITION_BORDER_MODE = DCOMPOSITION_BORDER_MODE(1i32);
pub const DCOMPOSITION_BORDER_MODE_INHERIT: DCOMPOSITION_BORDER_MODE = DCOMPOSITION_BORDER_MODE(-1i32);
impl ::core::marker::Copy for DCOMPOSITION_BORDER_MODE {}
impl ::core::clone::Clone for DCOMPOSITION_BORDER_MODE {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct DCOMPOSITION_COMPOSITE_MODE(pub i32);
pub const DCOMPOSITION_COMPOSITE_MODE_SOURCE_OVER: DCOMPOSITION_COMPOSITE_MODE = DCOMPOSITION_COMPOSITE_MODE(0i32);
pub const DCOMPOSITION_COMPOSITE_MODE_DESTINATION_INVERT: DCOMPOSITION_COMPOSITE_MODE = DCOMPOSITION_COMPOSITE_MODE(1i32);
pub const DCOMPOSITION_COMPOSITE_MODE_MIN_BLEND: DCOMPOSITION_COMPOSITE_MODE = DCOMPOSITION_COMPOSITE_MODE(2i32);
pub const DCOMPOSITION_COMPOSITE_MODE_INHERIT: DCOMPOSITION_COMPOSITE_MODE = DCOMPOSITION_COMPOSITE_MODE(-1i32);
impl ::core::marker::Copy for DCOMPOSITION_COMPOSITE_MODE {}
impl ::core::clone::Clone for DCOMPOSITION_COMPOSITE_MODE {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct DCOMPOSITION_DEPTH_MODE(pub i32);
pub const DCOMPOSITION_DEPTH_MODE_TREE: DCOMPOSITION_DEPTH_MODE = DCOMPOSITION_DEPTH_MODE(0i32);
pub const DCOMPOSITION_DEPTH_MODE_SPATIAL: DCOMPOSITION_DEPTH_MODE = DCOMPOSITION_DEPTH_MODE(1i32);
pub const DCOMPOSITION_DEPTH_MODE_SORTED: DCOMPOSITION_DEPTH_MODE = DCOMPOSITION_DEPTH_MODE(3i32);
pub const DCOMPOSITION_DEPTH_MODE_INHERIT: DCOMPOSITION_DEPTH_MODE = DCOMPOSITION_DEPTH_MODE(-1i32);
impl ::core::marker::Copy for DCOMPOSITION_DEPTH_MODE {}
impl ::core::clone::Clone for DCOMPOSITION_DEPTH_MODE {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
pub struct DCOMPOSITION_FRAME_STATISTICS {
    pub lastFrameTime: i64,
    pub currentCompositionRate: super::Dxgi::Common::DXGI_RATIONAL,
    pub currentTime: i64,
    pub timeFrequency: i64,
    pub nextEstimatedFrameTime: i64,
}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
impl ::core::marker::Copy for DCOMPOSITION_FRAME_STATISTICS {}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
impl ::core::clone::Clone for DCOMPOSITION_FRAME_STATISTICS {
    fn clone(&self) -> Self {
        *self
    }
}
pub const DCOMPOSITION_MAX_WAITFORCOMPOSITORCLOCK_OBJECTS: u32 = 32u32;
#[repr(transparent)]
pub struct DCOMPOSITION_OPACITY_MODE(pub i32);
pub const DCOMPOSITION_OPACITY_MODE_LAYER: DCOMPOSITION_OPACITY_MODE = DCOMPOSITION_OPACITY_MODE(0i32);
pub const DCOMPOSITION_OPACITY_MODE_MULTIPLY: DCOMPOSITION_OPACITY_MODE = DCOMPOSITION_OPACITY_MODE(1i32);
pub const DCOMPOSITION_OPACITY_MODE_INHERIT: DCOMPOSITION_OPACITY_MODE = DCOMPOSITION_OPACITY_MODE(-1i32);
impl ::core::marker::Copy for DCOMPOSITION_OPACITY_MODE {}
impl ::core::clone::Clone for DCOMPOSITION_OPACITY_MODE {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct DCompositionInkTrailPoint {
    pub x: f32,
    pub y: f32,
    pub radius: f32,
}
impl ::core::marker::Copy for DCompositionInkTrailPoint {}
impl ::core::clone::Clone for DCompositionInkTrailPoint {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IDCompositionAffineTransform2DEffect(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IDCompositionAnimation(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IDCompositionArithmeticCompositeEffect(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IDCompositionBlendEffect(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IDCompositionBrightnessEffect(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IDCompositionClip(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IDCompositionColorMatrixEffect(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IDCompositionCompositeEffect(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IDCompositionDelegatedInkTrail(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IDCompositionDesktopDevice(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IDCompositionDevice(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IDCompositionDevice2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IDCompositionDevice3(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IDCompositionDeviceDebug(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IDCompositionEffect(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IDCompositionEffectGroup(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IDCompositionFilterEffect(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IDCompositionGaussianBlurEffect(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IDCompositionHueRotationEffect(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IDCompositionInkTrailDevice(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IDCompositionLinearTransferEffect(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IDCompositionMatrixTransform(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IDCompositionMatrixTransform3D(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IDCompositionRectangleClip(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IDCompositionRotateTransform(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IDCompositionRotateTransform3D(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IDCompositionSaturationEffect(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IDCompositionScaleTransform(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IDCompositionScaleTransform3D(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IDCompositionShadowEffect(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IDCompositionSkewTransform(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IDCompositionSurface(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IDCompositionSurfaceFactory(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IDCompositionTableTransferEffect(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IDCompositionTarget(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IDCompositionTransform(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IDCompositionTransform3D(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IDCompositionTranslateTransform(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IDCompositionTranslateTransform3D(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IDCompositionTurbulenceEffect(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IDCompositionVirtualSurface(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IDCompositionVisual(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IDCompositionVisual2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IDCompositionVisual3(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IDCompositionVisualDebug(pub *mut ::core::ffi::c_void);

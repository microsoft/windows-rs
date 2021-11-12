#![allow(non_snake_case, non_camel_case_types)]
#[link(name = "windows")]
extern "system" {
    #[doc = "*Required features: `Win32_Graphics_DirectComposition`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DCompositionAttachMouseDragToHwnd(visual: IDCompositionVisual, hwnd: super::super::Foundation::HWND, enable: super::super::Foundation::BOOL) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `Win32_Graphics_DirectComposition`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DCompositionAttachMouseWheelToHwnd(visual: IDCompositionVisual, hwnd: super::super::Foundation::HWND, enable: super::super::Foundation::BOOL) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `Win32_Graphics_DirectComposition`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DCompositionBoostCompositorClock(enable: super::super::Foundation::BOOL) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `Win32_Graphics_DirectComposition`, `Win32_Graphics_Dxgi`*"]
    #[cfg(feature = "Win32_Graphics_Dxgi")]
    pub fn DCompositionCreateDevice(dxgidevice: super::Dxgi::IDXGIDevice, iid: *const ::windows_sys::core::GUID, dcompositiondevice: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `Win32_Graphics_DirectComposition`*"]
    pub fn DCompositionCreateDevice2(renderingdevice: ::windows_sys::core::IUnknown, iid: *const ::windows_sys::core::GUID, dcompositiondevice: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `Win32_Graphics_DirectComposition`*"]
    pub fn DCompositionCreateDevice3(renderingdevice: ::windows_sys::core::IUnknown, iid: *const ::windows_sys::core::GUID, dcompositiondevice: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `Win32_Graphics_DirectComposition`, `Win32_Foundation`, `Win32_Security`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
    pub fn DCompositionCreateSurfaceHandle(desiredaccess: u32, securityattributes: *const super::super::Security::SECURITY_ATTRIBUTES, surfacehandle: *mut super::super::Foundation::HANDLE) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `Win32_Graphics_DirectComposition`*"]
    pub fn DCompositionGetFrameId(frameidtype: COMPOSITION_FRAME_ID_TYPE, frameid: *mut u64) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `Win32_Graphics_DirectComposition`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DCompositionGetStatistics(frameid: u64, framestats: *mut COMPOSITION_FRAME_STATS, targetidcount: u32, targetids: *mut COMPOSITION_TARGET_ID, actualtargetidcount: *mut u32) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `Win32_Graphics_DirectComposition`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DCompositionGetTargetStatistics(frameid: u64, targetid: *const COMPOSITION_TARGET_ID, targetstats: *mut COMPOSITION_TARGET_STATS) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `Win32_Graphics_DirectComposition`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DCompositionWaitForCompositorClock(count: u32, handles: *const super::super::Foundation::HANDLE, timeoutinms: u32) -> u32;
}
#[doc = "*Required features: `Win32_Graphics_DirectComposition`*"]
pub const COMPOSITIONOBJECT_READ: i32 = 1i32;
#[doc = "*Required features: `Win32_Graphics_DirectComposition`*"]
pub const COMPOSITIONOBJECT_WRITE: i32 = 2i32;
pub struct COMPOSITION_FRAME_ID_TYPE(i32);
pub struct COMPOSITION_FRAME_STATS(i32);
pub struct COMPOSITION_STATS(i32);
#[doc = "*Required features: `Win32_Graphics_DirectComposition`*"]
pub const COMPOSITION_STATS_MAX_TARGETS: u32 = 256u32;
#[cfg(feature = "Win32_Foundation")]
pub struct COMPOSITION_TARGET_ID(i32);
pub struct COMPOSITION_TARGET_STATS(i32);
pub struct DCOMPOSITION_BACKFACE_VISIBILITY(i32);
pub struct DCOMPOSITION_BITMAP_INTERPOLATION_MODE(i32);
pub struct DCOMPOSITION_BORDER_MODE(i32);
pub struct DCOMPOSITION_COMPOSITE_MODE(i32);
pub struct DCOMPOSITION_DEPTH_MODE(i32);
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
pub struct DCOMPOSITION_FRAME_STATISTICS(i32);
#[doc = "*Required features: `Win32_Graphics_DirectComposition`*"]
pub const DCOMPOSITION_MAX_WAITFORCOMPOSITORCLOCK_OBJECTS: u32 = 32u32;
pub struct DCOMPOSITION_OPACITY_MODE(i32);
pub struct DCompositionInkTrailPoint(i32);
pub struct IDCompositionAffineTransform2DEffect(pub *mut ::core::ffi::c_void);
pub struct IDCompositionAnimation(pub *mut ::core::ffi::c_void);
pub struct IDCompositionArithmeticCompositeEffect(pub *mut ::core::ffi::c_void);
pub struct IDCompositionBlendEffect(pub *mut ::core::ffi::c_void);
pub struct IDCompositionBrightnessEffect(pub *mut ::core::ffi::c_void);
pub struct IDCompositionClip(pub *mut ::core::ffi::c_void);
pub struct IDCompositionColorMatrixEffect(pub *mut ::core::ffi::c_void);
pub struct IDCompositionCompositeEffect(pub *mut ::core::ffi::c_void);
pub struct IDCompositionDelegatedInkTrail(pub *mut ::core::ffi::c_void);
pub struct IDCompositionDesktopDevice(pub *mut ::core::ffi::c_void);
pub struct IDCompositionDevice(pub *mut ::core::ffi::c_void);
pub struct IDCompositionDevice2(pub *mut ::core::ffi::c_void);
pub struct IDCompositionDevice3(pub *mut ::core::ffi::c_void);
pub struct IDCompositionDeviceDebug(pub *mut ::core::ffi::c_void);
pub struct IDCompositionEffect(pub *mut ::core::ffi::c_void);
pub struct IDCompositionEffectGroup(pub *mut ::core::ffi::c_void);
pub struct IDCompositionFilterEffect(pub *mut ::core::ffi::c_void);
pub struct IDCompositionGaussianBlurEffect(pub *mut ::core::ffi::c_void);
pub struct IDCompositionHueRotationEffect(pub *mut ::core::ffi::c_void);
pub struct IDCompositionInkTrailDevice(pub *mut ::core::ffi::c_void);
pub struct IDCompositionLinearTransferEffect(pub *mut ::core::ffi::c_void);
pub struct IDCompositionMatrixTransform(pub *mut ::core::ffi::c_void);
pub struct IDCompositionMatrixTransform3D(pub *mut ::core::ffi::c_void);
pub struct IDCompositionRectangleClip(pub *mut ::core::ffi::c_void);
pub struct IDCompositionRotateTransform(pub *mut ::core::ffi::c_void);
pub struct IDCompositionRotateTransform3D(pub *mut ::core::ffi::c_void);
pub struct IDCompositionSaturationEffect(pub *mut ::core::ffi::c_void);
pub struct IDCompositionScaleTransform(pub *mut ::core::ffi::c_void);
pub struct IDCompositionScaleTransform3D(pub *mut ::core::ffi::c_void);
pub struct IDCompositionShadowEffect(pub *mut ::core::ffi::c_void);
pub struct IDCompositionSkewTransform(pub *mut ::core::ffi::c_void);
pub struct IDCompositionSurface(pub *mut ::core::ffi::c_void);
pub struct IDCompositionSurfaceFactory(pub *mut ::core::ffi::c_void);
pub struct IDCompositionTableTransferEffect(pub *mut ::core::ffi::c_void);
pub struct IDCompositionTarget(pub *mut ::core::ffi::c_void);
pub struct IDCompositionTransform(pub *mut ::core::ffi::c_void);
pub struct IDCompositionTransform3D(pub *mut ::core::ffi::c_void);
pub struct IDCompositionTranslateTransform(pub *mut ::core::ffi::c_void);
pub struct IDCompositionTranslateTransform3D(pub *mut ::core::ffi::c_void);
pub struct IDCompositionTurbulenceEffect(pub *mut ::core::ffi::c_void);
pub struct IDCompositionVirtualSurface(pub *mut ::core::ffi::c_void);
pub struct IDCompositionVisual(pub *mut ::core::ffi::c_void);
pub struct IDCompositionVisual2(pub *mut ::core::ffi::c_void);
pub struct IDCompositionVisual3(pub *mut ::core::ffi::c_void);
pub struct IDCompositionVisualDebug(pub *mut ::core::ffi::c_void);

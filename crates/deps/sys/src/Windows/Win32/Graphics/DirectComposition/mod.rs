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
pub struct COMPOSITION_TARGET_ID(i32);
pub struct COMPOSITION_TARGET_STATS(i32);
pub struct DCOMPOSITION_BACKFACE_VISIBILITY(i32);
pub struct DCOMPOSITION_BITMAP_INTERPOLATION_MODE(i32);
pub struct DCOMPOSITION_BORDER_MODE(i32);
pub struct DCOMPOSITION_COMPOSITE_MODE(i32);
pub struct DCOMPOSITION_DEPTH_MODE(i32);
pub struct DCOMPOSITION_FRAME_STATISTICS(i32);
#[doc = "*Required features: `Win32_Graphics_DirectComposition`*"]
pub const DCOMPOSITION_MAX_WAITFORCOMPOSITORCLOCK_OBJECTS: u32 = 32u32;
pub struct DCOMPOSITION_OPACITY_MODE(i32);
pub struct DCompositionInkTrailPoint(i32);
pub struct IDCompositionAffineTransform2DEffect(i32);
pub struct IDCompositionAnimation(i32);
pub struct IDCompositionArithmeticCompositeEffect(i32);
pub struct IDCompositionBlendEffect(i32);
pub struct IDCompositionBrightnessEffect(i32);
pub struct IDCompositionClip(i32);
pub struct IDCompositionColorMatrixEffect(i32);
pub struct IDCompositionCompositeEffect(i32);
pub struct IDCompositionDelegatedInkTrail(i32);
pub struct IDCompositionDesktopDevice(i32);
pub struct IDCompositionDevice(i32);
pub struct IDCompositionDevice2(i32);
pub struct IDCompositionDevice3(i32);
pub struct IDCompositionDeviceDebug(i32);
pub struct IDCompositionEffect(i32);
pub struct IDCompositionEffectGroup(i32);
pub struct IDCompositionFilterEffect(i32);
pub struct IDCompositionGaussianBlurEffect(i32);
pub struct IDCompositionHueRotationEffect(i32);
pub struct IDCompositionInkTrailDevice(i32);
pub struct IDCompositionLinearTransferEffect(i32);
pub struct IDCompositionMatrixTransform(i32);
pub struct IDCompositionMatrixTransform3D(i32);
pub struct IDCompositionRectangleClip(i32);
pub struct IDCompositionRotateTransform(i32);
pub struct IDCompositionRotateTransform3D(i32);
pub struct IDCompositionSaturationEffect(i32);
pub struct IDCompositionScaleTransform(i32);
pub struct IDCompositionScaleTransform3D(i32);
pub struct IDCompositionShadowEffect(i32);
pub struct IDCompositionSkewTransform(i32);
pub struct IDCompositionSurface(i32);
pub struct IDCompositionSurfaceFactory(i32);
pub struct IDCompositionTableTransferEffect(i32);
pub struct IDCompositionTarget(i32);
pub struct IDCompositionTransform(i32);
pub struct IDCompositionTransform3D(i32);
pub struct IDCompositionTranslateTransform(i32);
pub struct IDCompositionTranslateTransform3D(i32);
pub struct IDCompositionTurbulenceEffect(i32);
pub struct IDCompositionVirtualSurface(i32);
pub struct IDCompositionVisual(i32);
pub struct IDCompositionVisual2(i32);
pub struct IDCompositionVisual3(i32);
pub struct IDCompositionVisualDebug(i32);

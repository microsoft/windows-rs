#![allow(non_snake_case, non_camel_case_types)]
#[link(name = "windows")]
extern "system" {
    #[doc = "*Required features: `Win32_Graphics_CompositionSwapchain`*"]
    pub fn CreatePresentationFactory(d3ddevice: ::windows_sys::core::IUnknown, riid: *const ::windows_sys::core::GUID, presentationfactory: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi_Common"))]
pub struct CompositionFrameDisplayInstance(i32);
pub struct CompositionFrameInstanceKind(i32);
pub struct ICompositionFramePresentStatistics(pub *mut ::core::ffi::c_void);
pub struct IIndependentFlipFramePresentStatistics(pub *mut ::core::ffi::c_void);
pub struct IPresentStatistics(pub *mut ::core::ffi::c_void);
pub struct IPresentStatusPresentStatistics(pub *mut ::core::ffi::c_void);
pub struct IPresentationBuffer(pub *mut ::core::ffi::c_void);
pub struct IPresentationContent(pub *mut ::core::ffi::c_void);
pub struct IPresentationFactory(pub *mut ::core::ffi::c_void);
pub struct IPresentationManager(pub *mut ::core::ffi::c_void);
pub struct IPresentationSurface(pub *mut ::core::ffi::c_void);
pub struct PresentStatisticsKind(i32);
pub struct PresentStatus(i32);
pub struct PresentationTransform(i32);
pub struct SystemInterruptTime(i32);

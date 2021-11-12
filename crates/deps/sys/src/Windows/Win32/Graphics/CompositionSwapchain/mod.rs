#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals)]
#[link(name = "windows")]
extern "system" {
    pub fn CreatePresentationFactory(d3ddevice: ::windows_sys::core::IUnknown, riid: *const ::windows_sys::core::GUID, presentationfactory: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT;
}
#[repr(C)]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi_Common"))]
pub struct CompositionFrameDisplayInstance {
    pub displayAdapterLUID: super::super::Foundation::LUID,
    pub displayVidPnSourceId: u32,
    pub displayUniqueId: u32,
    pub renderAdapterLUID: super::super::Foundation::LUID,
    pub instanceKind: CompositionFrameInstanceKind,
    pub finalTransform: PresentationTransform,
    pub requiredCrossAdapterCopy: u8,
    pub colorSpace: super::Dxgi::Common::DXGI_COLOR_SPACE_TYPE,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi_Common"))]
impl ::core::marker::Copy for CompositionFrameDisplayInstance {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi_Common"))]
impl ::core::clone::Clone for CompositionFrameDisplayInstance {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct CompositionFrameInstanceKind(pub i32);
pub const CompositionFrameInstanceKind_ComposedOnScreen: CompositionFrameInstanceKind = CompositionFrameInstanceKind(0i32);
pub const CompositionFrameInstanceKind_ScanoutOnScreen: CompositionFrameInstanceKind = CompositionFrameInstanceKind(1i32);
pub const CompositionFrameInstanceKind_ComposedToIntermediate: CompositionFrameInstanceKind = CompositionFrameInstanceKind(2i32);
#[repr(transparent)]
pub struct ICompositionFramePresentStatistics(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IIndependentFlipFramePresentStatistics(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IPresentStatistics(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IPresentStatusPresentStatistics(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IPresentationBuffer(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IPresentationContent(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IPresentationFactory(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IPresentationManager(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IPresentationSurface(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct PresentStatisticsKind(pub i32);
pub const PresentStatisticsKind_PresentStatus: PresentStatisticsKind = PresentStatisticsKind(1i32);
pub const PresentStatisticsKind_CompositionFrame: PresentStatisticsKind = PresentStatisticsKind(2i32);
pub const PresentStatisticsKind_IndependentFlipFrame: PresentStatisticsKind = PresentStatisticsKind(3i32);
#[repr(transparent)]
pub struct PresentStatus(pub i32);
pub const PresentStatus_Queued: PresentStatus = PresentStatus(0i32);
pub const PresentStatus_Skipped: PresentStatus = PresentStatus(1i32);
pub const PresentStatus_Canceled: PresentStatus = PresentStatus(2i32);
#[repr(C)]
pub struct PresentationTransform {
    pub M11: f32,
    pub M12: f32,
    pub M21: f32,
    pub M22: f32,
    pub M31: f32,
    pub M32: f32,
}
impl ::core::marker::Copy for PresentationTransform {}
impl ::core::clone::Clone for PresentationTransform {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct SystemInterruptTime {
    pub value: u64,
}
impl ::core::marker::Copy for SystemInterruptTime {}
impl ::core::clone::Clone for SystemInterruptTime {
    fn clone(&self) -> Self {
        *self
    }
}

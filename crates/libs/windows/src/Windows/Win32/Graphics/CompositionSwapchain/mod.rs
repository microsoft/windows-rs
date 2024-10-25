pub const CompositionFrameInstanceKind_ComposedOnScreen: CompositionFrameInstanceKind = 0i32;
pub const CompositionFrameInstanceKind_ComposedToIntermediate: CompositionFrameInstanceKind = 2i32;
pub const CompositionFrameInstanceKind_ScanoutOnScreen: CompositionFrameInstanceKind = 1i32;
pub const PresentStatisticsKind_CompositionFrame: PresentStatisticsKind = 2i32;
pub const PresentStatisticsKind_IndependentFlipFrame: PresentStatisticsKind = 3i32;
pub const PresentStatisticsKind_PresentStatus: PresentStatisticsKind = 1i32;
pub const PresentStatus_Canceled: PresentStatus = 2i32;
pub const PresentStatus_Queued: PresentStatus = 0i32;
pub const PresentStatus_Skipped: PresentStatus = 1i32;
#[repr(transparent)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct CompositionFrameInstanceKind(pub i32);
impl windows_core::TypeKind for CompositionFrameInstanceKind {
    type TypeKind = windows_core::CopyType;
}
#[repr(transparent)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct PresentStatisticsKind(pub i32);
impl windows_core::TypeKind for PresentStatisticsKind {
    type TypeKind = windows_core::CopyType;
}
#[repr(transparent)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct PresentStatus(pub i32);
impl windows_core::TypeKind for PresentStatus {
    type TypeKind = windows_core::CopyType;
}
#[repr(C)]
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
#[derive(Clone, Copy, Debug, PartialEq)]
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
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
impl Default for CompositionFrameDisplayInstance {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
impl windows_core::TypeKind for CompositionFrameDisplayInstance {
    type TypeKind = windows_core::CloneType;
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct PresentationTransform {
    pub M11: f32,
    pub M12: f32,
    pub M21: f32,
    pub M22: f32,
    pub M31: f32,
    pub M32: f32,
}
impl Default for PresentationTransform {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
impl windows_core::TypeKind for PresentationTransform {
    type TypeKind = windows_core::CopyType;
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct SystemInterruptTime {
    pub value: u64,
}
impl Default for SystemInterruptTime {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
impl windows_core::TypeKind for SystemInterruptTime {
    type TypeKind = windows_core::CopyType;
}

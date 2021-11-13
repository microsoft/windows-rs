#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, clashing_extern_declarations, clippy::all)]
#[link(name = "windows")]
extern "system" {}
#[repr(C)]
pub struct HolographicAdapterId {
    pub LowPart: u32,
    pub HighPart: i32,
}
impl ::core::marker::Copy for HolographicAdapterId {}
impl ::core::clone::Clone for HolographicAdapterId {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct HolographicCamera(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for HolographicCamera {}
impl ::core::clone::Clone for HolographicCamera {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct HolographicCameraPose(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for HolographicCameraPose {}
impl ::core::clone::Clone for HolographicCameraPose {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct HolographicCameraRenderingParameters(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for HolographicCameraRenderingParameters {}
impl ::core::clone::Clone for HolographicCameraRenderingParameters {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct HolographicCameraViewportParameters(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for HolographicCameraViewportParameters {}
impl ::core::clone::Clone for HolographicCameraViewportParameters {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct HolographicDepthReprojectionMethod(pub i32);
impl HolographicDepthReprojectionMethod {
    pub const DepthReprojection: Self = Self(0i32);
    pub const AutoPlanar: Self = Self(1i32);
}
impl ::core::marker::Copy for HolographicDepthReprojectionMethod {}
impl ::core::clone::Clone for HolographicDepthReprojectionMethod {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct HolographicDisplay(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for HolographicDisplay {}
impl ::core::clone::Clone for HolographicDisplay {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct HolographicFrame(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for HolographicFrame {}
impl ::core::clone::Clone for HolographicFrame {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct HolographicFrameId {
    pub Value: u64,
}
impl ::core::marker::Copy for HolographicFrameId {}
impl ::core::clone::Clone for HolographicFrameId {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct HolographicFramePrediction(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for HolographicFramePrediction {}
impl ::core::clone::Clone for HolographicFramePrediction {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct HolographicFramePresentResult(pub i32);
impl HolographicFramePresentResult {
    pub const Success: Self = Self(0i32);
    pub const DeviceRemoved: Self = Self(1i32);
}
impl ::core::marker::Copy for HolographicFramePresentResult {}
impl ::core::clone::Clone for HolographicFramePresentResult {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct HolographicFramePresentWaitBehavior(pub i32);
impl HolographicFramePresentWaitBehavior {
    pub const WaitForFrameToFinish: Self = Self(0i32);
    pub const DoNotWaitForFrameToFinish: Self = Self(1i32);
}
impl ::core::marker::Copy for HolographicFramePresentWaitBehavior {}
impl ::core::clone::Clone for HolographicFramePresentWaitBehavior {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct HolographicFramePresentationMonitor(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for HolographicFramePresentationMonitor {}
impl ::core::clone::Clone for HolographicFramePresentationMonitor {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct HolographicFramePresentationReport(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for HolographicFramePresentationReport {}
impl ::core::clone::Clone for HolographicFramePresentationReport {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct HolographicFrameRenderingReport(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for HolographicFrameRenderingReport {}
impl ::core::clone::Clone for HolographicFrameRenderingReport {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct HolographicFrameScanoutMonitor(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for HolographicFrameScanoutMonitor {}
impl ::core::clone::Clone for HolographicFrameScanoutMonitor {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct HolographicFrameScanoutReport(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for HolographicFrameScanoutReport {}
impl ::core::clone::Clone for HolographicFrameScanoutReport {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct HolographicQuadLayer(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for HolographicQuadLayer {}
impl ::core::clone::Clone for HolographicQuadLayer {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct HolographicQuadLayerUpdateParameters(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for HolographicQuadLayerUpdateParameters {}
impl ::core::clone::Clone for HolographicQuadLayerUpdateParameters {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct HolographicReprojectionMode(pub i32);
impl HolographicReprojectionMode {
    pub const PositionAndOrientation: Self = Self(0i32);
    pub const OrientationOnly: Self = Self(1i32);
    pub const Disabled: Self = Self(2i32);
}
impl ::core::marker::Copy for HolographicReprojectionMode {}
impl ::core::clone::Clone for HolographicReprojectionMode {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct HolographicSpace(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for HolographicSpace {}
impl ::core::clone::Clone for HolographicSpace {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct HolographicSpaceCameraAddedEventArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for HolographicSpaceCameraAddedEventArgs {}
impl ::core::clone::Clone for HolographicSpaceCameraAddedEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct HolographicSpaceCameraRemovedEventArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for HolographicSpaceCameraRemovedEventArgs {}
impl ::core::clone::Clone for HolographicSpaceCameraRemovedEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct HolographicSpaceUserPresence(pub i32);
impl HolographicSpaceUserPresence {
    pub const Absent: Self = Self(0i32);
    pub const PresentPassive: Self = Self(1i32);
    pub const PresentActive: Self = Self(2i32);
}
impl ::core::marker::Copy for HolographicSpaceUserPresence {}
impl ::core::clone::Clone for HolographicSpaceUserPresence {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[cfg(feature = "Foundation_Numerics")]
pub struct HolographicStereoTransform {
    pub Left: super::super::Foundation::Numerics::Matrix4x4,
    pub Right: super::super::Foundation::Numerics::Matrix4x4,
}
#[cfg(feature = "Foundation_Numerics")]
impl ::core::marker::Copy for HolographicStereoTransform {}
#[cfg(feature = "Foundation_Numerics")]
impl ::core::clone::Clone for HolographicStereoTransform {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct HolographicViewConfiguration(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for HolographicViewConfiguration {}
impl ::core::clone::Clone for HolographicViewConfiguration {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct HolographicViewConfigurationKind(pub i32);
impl HolographicViewConfigurationKind {
    pub const Display: Self = Self(0i32);
    pub const PhotoVideoCamera: Self = Self(1i32);
}
impl ::core::marker::Copy for HolographicViewConfigurationKind {}
impl ::core::clone::Clone for HolographicViewConfigurationKind {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IHolographicCamera(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IHolographicCamera {}
impl ::core::clone::Clone for IHolographicCamera {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IHolographicCamera2(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IHolographicCamera2 {}
impl ::core::clone::Clone for IHolographicCamera2 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IHolographicCamera3(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IHolographicCamera3 {}
impl ::core::clone::Clone for IHolographicCamera3 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IHolographicCamera4(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IHolographicCamera4 {}
impl ::core::clone::Clone for IHolographicCamera4 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IHolographicCamera5(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IHolographicCamera5 {}
impl ::core::clone::Clone for IHolographicCamera5 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IHolographicCamera6(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IHolographicCamera6 {}
impl ::core::clone::Clone for IHolographicCamera6 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IHolographicCameraPose(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IHolographicCameraPose {}
impl ::core::clone::Clone for IHolographicCameraPose {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IHolographicCameraPose2(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IHolographicCameraPose2 {}
impl ::core::clone::Clone for IHolographicCameraPose2 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IHolographicCameraRenderingParameters(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IHolographicCameraRenderingParameters {}
impl ::core::clone::Clone for IHolographicCameraRenderingParameters {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IHolographicCameraRenderingParameters2(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IHolographicCameraRenderingParameters2 {}
impl ::core::clone::Clone for IHolographicCameraRenderingParameters2 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IHolographicCameraRenderingParameters3(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IHolographicCameraRenderingParameters3 {}
impl ::core::clone::Clone for IHolographicCameraRenderingParameters3 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IHolographicCameraRenderingParameters4(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IHolographicCameraRenderingParameters4 {}
impl ::core::clone::Clone for IHolographicCameraRenderingParameters4 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IHolographicCameraViewportParameters(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IHolographicCameraViewportParameters {}
impl ::core::clone::Clone for IHolographicCameraViewportParameters {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IHolographicDisplay(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IHolographicDisplay {}
impl ::core::clone::Clone for IHolographicDisplay {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IHolographicDisplay2(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IHolographicDisplay2 {}
impl ::core::clone::Clone for IHolographicDisplay2 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IHolographicDisplay3(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IHolographicDisplay3 {}
impl ::core::clone::Clone for IHolographicDisplay3 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IHolographicDisplayStatics(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IHolographicDisplayStatics {}
impl ::core::clone::Clone for IHolographicDisplayStatics {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IHolographicFrame(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IHolographicFrame {}
impl ::core::clone::Clone for IHolographicFrame {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IHolographicFrame2(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IHolographicFrame2 {}
impl ::core::clone::Clone for IHolographicFrame2 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IHolographicFrame3(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IHolographicFrame3 {}
impl ::core::clone::Clone for IHolographicFrame3 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IHolographicFramePrediction(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IHolographicFramePrediction {}
impl ::core::clone::Clone for IHolographicFramePrediction {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IHolographicFramePresentationMonitor(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IHolographicFramePresentationMonitor {}
impl ::core::clone::Clone for IHolographicFramePresentationMonitor {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IHolographicFramePresentationReport(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IHolographicFramePresentationReport {}
impl ::core::clone::Clone for IHolographicFramePresentationReport {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IHolographicFrameRenderingReport(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IHolographicFrameRenderingReport {}
impl ::core::clone::Clone for IHolographicFrameRenderingReport {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IHolographicFrameScanoutMonitor(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IHolographicFrameScanoutMonitor {}
impl ::core::clone::Clone for IHolographicFrameScanoutMonitor {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IHolographicFrameScanoutReport(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IHolographicFrameScanoutReport {}
impl ::core::clone::Clone for IHolographicFrameScanoutReport {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IHolographicQuadLayer(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IHolographicQuadLayer {}
impl ::core::clone::Clone for IHolographicQuadLayer {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IHolographicQuadLayerFactory(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IHolographicQuadLayerFactory {}
impl ::core::clone::Clone for IHolographicQuadLayerFactory {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IHolographicQuadLayerUpdateParameters(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IHolographicQuadLayerUpdateParameters {}
impl ::core::clone::Clone for IHolographicQuadLayerUpdateParameters {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IHolographicQuadLayerUpdateParameters2(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IHolographicQuadLayerUpdateParameters2 {}
impl ::core::clone::Clone for IHolographicQuadLayerUpdateParameters2 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IHolographicSpace(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IHolographicSpace {}
impl ::core::clone::Clone for IHolographicSpace {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IHolographicSpace2(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IHolographicSpace2 {}
impl ::core::clone::Clone for IHolographicSpace2 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IHolographicSpace3(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IHolographicSpace3 {}
impl ::core::clone::Clone for IHolographicSpace3 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IHolographicSpaceCameraAddedEventArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IHolographicSpaceCameraAddedEventArgs {}
impl ::core::clone::Clone for IHolographicSpaceCameraAddedEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IHolographicSpaceCameraRemovedEventArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IHolographicSpaceCameraRemovedEventArgs {}
impl ::core::clone::Clone for IHolographicSpaceCameraRemovedEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IHolographicSpaceStatics(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IHolographicSpaceStatics {}
impl ::core::clone::Clone for IHolographicSpaceStatics {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IHolographicSpaceStatics2(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IHolographicSpaceStatics2 {}
impl ::core::clone::Clone for IHolographicSpaceStatics2 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IHolographicSpaceStatics3(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IHolographicSpaceStatics3 {}
impl ::core::clone::Clone for IHolographicSpaceStatics3 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IHolographicViewConfiguration(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IHolographicViewConfiguration {}
impl ::core::clone::Clone for IHolographicViewConfiguration {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IHolographicViewConfiguration2(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IHolographicViewConfiguration2 {}
impl ::core::clone::Clone for IHolographicViewConfiguration2 {
    fn clone(&self) -> Self {
        *self
    }
}

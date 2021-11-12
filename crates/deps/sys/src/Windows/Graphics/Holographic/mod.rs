#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals)]
#[link(name = "windows")]
extern "system" {}
#[repr(C)]
pub struct HolographicAdapterId(i32);
#[repr(transparent)]
pub struct HolographicCamera(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct HolographicCameraPose(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct HolographicCameraRenderingParameters(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct HolographicCameraViewportParameters(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct HolographicDepthReprojectionMethod(pub i32);
impl HolographicDepthReprojectionMethod {
    pub const DepthReprojection: Self = Self(0i32);
    pub const AutoPlanar: Self = Self(1i32);
}
#[repr(transparent)]
pub struct HolographicDisplay(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct HolographicFrame(pub *mut ::core::ffi::c_void);
#[repr(C)]
pub struct HolographicFrameId(i32);
#[repr(transparent)]
pub struct HolographicFramePrediction(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct HolographicFramePresentResult(pub i32);
impl HolographicFramePresentResult {
    pub const Success: Self = Self(0i32);
    pub const DeviceRemoved: Self = Self(1i32);
}
#[repr(transparent)]
pub struct HolographicFramePresentWaitBehavior(pub i32);
impl HolographicFramePresentWaitBehavior {
    pub const WaitForFrameToFinish: Self = Self(0i32);
    pub const DoNotWaitForFrameToFinish: Self = Self(1i32);
}
#[repr(transparent)]
pub struct HolographicFramePresentationMonitor(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct HolographicFramePresentationReport(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct HolographicFrameRenderingReport(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct HolographicFrameScanoutMonitor(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct HolographicFrameScanoutReport(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct HolographicQuadLayer(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct HolographicQuadLayerUpdateParameters(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct HolographicReprojectionMode(pub i32);
impl HolographicReprojectionMode {
    pub const PositionAndOrientation: Self = Self(0i32);
    pub const OrientationOnly: Self = Self(1i32);
    pub const Disabled: Self = Self(2i32);
}
#[repr(transparent)]
pub struct HolographicSpace(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct HolographicSpaceCameraAddedEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct HolographicSpaceCameraRemovedEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct HolographicSpaceUserPresence(pub i32);
impl HolographicSpaceUserPresence {
    pub const Absent: Self = Self(0i32);
    pub const PresentPassive: Self = Self(1i32);
    pub const PresentActive: Self = Self(2i32);
}
#[cfg(feature = "Foundation_Numerics")]
#[repr(C)]
pub struct HolographicStereoTransform(i32);
#[repr(transparent)]
pub struct HolographicViewConfiguration(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct HolographicViewConfigurationKind(pub i32);
impl HolographicViewConfigurationKind {
    pub const Display: Self = Self(0i32);
    pub const PhotoVideoCamera: Self = Self(1i32);
}
#[repr(transparent)]
pub struct IHolographicCamera(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IHolographicCamera2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IHolographicCamera3(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IHolographicCamera4(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IHolographicCamera5(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IHolographicCamera6(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IHolographicCameraPose(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IHolographicCameraPose2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IHolographicCameraRenderingParameters(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IHolographicCameraRenderingParameters2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IHolographicCameraRenderingParameters3(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IHolographicCameraRenderingParameters4(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IHolographicCameraViewportParameters(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IHolographicDisplay(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IHolographicDisplay2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IHolographicDisplay3(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IHolographicDisplayStatics(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IHolographicFrame(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IHolographicFrame2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IHolographicFrame3(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IHolographicFramePrediction(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IHolographicFramePresentationMonitor(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IHolographicFramePresentationReport(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IHolographicFrameRenderingReport(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IHolographicFrameScanoutMonitor(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IHolographicFrameScanoutReport(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IHolographicQuadLayer(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IHolographicQuadLayerFactory(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IHolographicQuadLayerUpdateParameters(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IHolographicQuadLayerUpdateParameters2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IHolographicSpace(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IHolographicSpace2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IHolographicSpace3(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IHolographicSpaceCameraAddedEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IHolographicSpaceCameraRemovedEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IHolographicSpaceStatics(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IHolographicSpaceStatics2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IHolographicSpaceStatics3(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IHolographicViewConfiguration(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IHolographicViewConfiguration2(pub *mut ::core::ffi::c_void);

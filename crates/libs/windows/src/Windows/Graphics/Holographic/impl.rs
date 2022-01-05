#[cfg(feature = "implement_exclusive")]
pub trait IHolographicCameraImpl: Sized {
    fn RenderTargetSize(&self) -> ::windows::core::Result<super::super::Foundation::Size>;
    fn ViewportScaleFactor(&self) -> ::windows::core::Result<f64>;
    fn SetViewportScaleFactor(&self, value: f64) -> ::windows::core::Result<()>;
    fn IsStereo(&self) -> ::windows::core::Result<bool>;
    fn Id(&self) -> ::windows::core::Result<u32>;
    fn SetNearPlaneDistance(&self, value: f64) -> ::windows::core::Result<()>;
    fn SetFarPlaneDistance(&self, value: f64) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IHolographicCamera2Impl: Sized + IHolographicCameraImpl {
    fn LeftViewportParameters(&self) -> ::windows::core::Result<HolographicCameraViewportParameters>;
    fn RightViewportParameters(&self) -> ::windows::core::Result<HolographicCameraViewportParameters>;
    fn Display(&self) -> ::windows::core::Result<HolographicDisplay>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IHolographicCamera3Impl: Sized + IHolographicCameraImpl + IHolographicCamera2Impl {
    fn IsPrimaryLayerEnabled(&self) -> ::windows::core::Result<bool>;
    fn SetIsPrimaryLayerEnabled(&self, value: bool) -> ::windows::core::Result<()>;
    fn MaxQuadLayerCount(&self) -> ::windows::core::Result<u32>;
    fn QuadLayers(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVector<HolographicQuadLayer>>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IHolographicCamera4Impl: Sized {
    fn CanOverrideViewport(&self) -> ::windows::core::Result<bool>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IHolographicCamera5Impl: Sized {
    fn IsHardwareContentProtectionSupported(&self) -> ::windows::core::Result<bool>;
    fn IsHardwareContentProtectionEnabled(&self) -> ::windows::core::Result<bool>;
    fn SetIsHardwareContentProtectionEnabled(&self, value: bool) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IHolographicCamera6Impl: Sized {
    fn ViewConfiguration(&self) -> ::windows::core::Result<HolographicViewConfiguration>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IHolographicCameraPoseImpl: Sized {
    fn HolographicCamera(&self) -> ::windows::core::Result<HolographicCamera>;
    fn Viewport(&self) -> ::windows::core::Result<super::super::Foundation::Rect>;
    fn TryGetViewTransform(&self, coordinatesystem: &::core::option::Option<super::super::Perception::Spatial::SpatialCoordinateSystem>) -> ::windows::core::Result<super::super::Foundation::IReference<HolographicStereoTransform>>;
    fn ProjectionTransform(&self) -> ::windows::core::Result<HolographicStereoTransform>;
    fn TryGetCullingFrustum(&self, coordinatesystem: &::core::option::Option<super::super::Perception::Spatial::SpatialCoordinateSystem>) -> ::windows::core::Result<super::super::Foundation::IReference<super::super::Perception::Spatial::SpatialBoundingFrustum>>;
    fn TryGetVisibleFrustum(&self, coordinatesystem: &::core::option::Option<super::super::Perception::Spatial::SpatialCoordinateSystem>) -> ::windows::core::Result<super::super::Foundation::IReference<super::super::Perception::Spatial::SpatialBoundingFrustum>>;
    fn NearPlaneDistance(&self) -> ::windows::core::Result<f64>;
    fn FarPlaneDistance(&self) -> ::windows::core::Result<f64>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IHolographicCameraPose2Impl: Sized {
    fn OverrideViewTransform(&self, coordinatesystem: &::core::option::Option<super::super::Perception::Spatial::SpatialCoordinateSystem>, coordinatesystemtoviewtransform: &HolographicStereoTransform) -> ::windows::core::Result<()>;
    fn OverrideProjectionTransform(&self, projectiontransform: &HolographicStereoTransform) -> ::windows::core::Result<()>;
    fn OverrideViewport(&self, leftviewport: &super::super::Foundation::Rect, rightviewport: &super::super::Foundation::Rect) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IHolographicCameraRenderingParametersImpl: Sized {
    fn SetFocusPoint(&self, coordinatesystem: &::core::option::Option<super::super::Perception::Spatial::SpatialCoordinateSystem>, position: &super::super::Foundation::Numerics::Vector3) -> ::windows::core::Result<()>;
    fn SetFocusPointWithNormal(&self, coordinatesystem: &::core::option::Option<super::super::Perception::Spatial::SpatialCoordinateSystem>, position: &super::super::Foundation::Numerics::Vector3, normal: &super::super::Foundation::Numerics::Vector3) -> ::windows::core::Result<()>;
    fn SetFocusPointWithNormalLinearVelocity(&self, coordinatesystem: &::core::option::Option<super::super::Perception::Spatial::SpatialCoordinateSystem>, position: &super::super::Foundation::Numerics::Vector3, normal: &super::super::Foundation::Numerics::Vector3, linearvelocity: &super::super::Foundation::Numerics::Vector3) -> ::windows::core::Result<()>;
    fn Direct3D11Device(&self) -> ::windows::core::Result<super::DirectX::Direct3D11::IDirect3DDevice>;
    fn Direct3D11BackBuffer(&self) -> ::windows::core::Result<super::DirectX::Direct3D11::IDirect3DSurface>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IHolographicCameraRenderingParameters2Impl: Sized + IHolographicCameraRenderingParametersImpl {
    fn ReprojectionMode(&self) -> ::windows::core::Result<HolographicReprojectionMode>;
    fn SetReprojectionMode(&self, value: HolographicReprojectionMode) -> ::windows::core::Result<()>;
    fn CommitDirect3D11DepthBuffer(&self, value: &::core::option::Option<super::DirectX::Direct3D11::IDirect3DSurface>) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IHolographicCameraRenderingParameters3Impl: Sized + IHolographicCameraRenderingParametersImpl + IHolographicCameraRenderingParameters2Impl {
    fn IsContentProtectionEnabled(&self) -> ::windows::core::Result<bool>;
    fn SetIsContentProtectionEnabled(&self, value: bool) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IHolographicCameraRenderingParameters4Impl: Sized {
    fn DepthReprojectionMethod(&self) -> ::windows::core::Result<HolographicDepthReprojectionMethod>;
    fn SetDepthReprojectionMethod(&self, value: HolographicDepthReprojectionMethod) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IHolographicCameraViewportParametersImpl: Sized {
    fn HiddenAreaMesh(&self) -> ::windows::core::Result<::windows::core::Array<super::super::Foundation::Numerics::Vector2>>;
    fn VisibleAreaMesh(&self) -> ::windows::core::Result<::windows::core::Array<super::super::Foundation::Numerics::Vector2>>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IHolographicDisplayImpl: Sized {
    fn DisplayName(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn MaxViewportSize(&self) -> ::windows::core::Result<super::super::Foundation::Size>;
    fn IsStereo(&self) -> ::windows::core::Result<bool>;
    fn IsOpaque(&self) -> ::windows::core::Result<bool>;
    fn AdapterId(&self) -> ::windows::core::Result<HolographicAdapterId>;
    fn SpatialLocator(&self) -> ::windows::core::Result<super::super::Perception::Spatial::SpatialLocator>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IHolographicDisplay2Impl: Sized {
    fn RefreshRate(&self) -> ::windows::core::Result<f64>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IHolographicDisplay3Impl: Sized {
    fn TryGetViewConfiguration(&self, kind: HolographicViewConfigurationKind) -> ::windows::core::Result<HolographicViewConfiguration>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IHolographicDisplayStaticsImpl: Sized {
    fn GetDefault(&self) -> ::windows::core::Result<HolographicDisplay>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IHolographicFrameImpl: Sized {
    fn AddedCameras(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<HolographicCamera>>;
    fn RemovedCameras(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<HolographicCamera>>;
    fn GetRenderingParameters(&self, camerapose: &::core::option::Option<HolographicCameraPose>) -> ::windows::core::Result<HolographicCameraRenderingParameters>;
    fn Duration(&self) -> ::windows::core::Result<super::super::Foundation::TimeSpan>;
    fn CurrentPrediction(&self) -> ::windows::core::Result<HolographicFramePrediction>;
    fn UpdateCurrentPrediction(&self) -> ::windows::core::Result<()>;
    fn PresentUsingCurrentPrediction(&self) -> ::windows::core::Result<HolographicFramePresentResult>;
    fn PresentUsingCurrentPredictionWithBehavior(&self, waitbehavior: HolographicFramePresentWaitBehavior) -> ::windows::core::Result<HolographicFramePresentResult>;
    fn WaitForFrameToFinish(&self) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IHolographicFrame2Impl: Sized + IHolographicFrameImpl {
    fn GetQuadLayerUpdateParameters(&self, layer: &::core::option::Option<HolographicQuadLayer>) -> ::windows::core::Result<HolographicQuadLayerUpdateParameters>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IHolographicFrame3Impl: Sized {
    fn Id(&self) -> ::windows::core::Result<HolographicFrameId>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IHolographicFramePredictionImpl: Sized {
    fn CameraPoses(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<HolographicCameraPose>>;
    fn Timestamp(&self) -> ::windows::core::Result<super::super::Perception::PerceptionTimestamp>;
}
#[cfg(all(feature = "Foundation", feature = "deprecated", feature = "implement_exclusive"))]
pub trait IHolographicFramePresentationMonitorImpl: Sized + IClosableImpl {
    fn ReadReports(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<HolographicFramePresentationReport>>;
}
#[cfg(all(feature = "deprecated", feature = "implement_exclusive"))]
pub trait IHolographicFramePresentationReportImpl: Sized {
    fn CompositorGpuDuration(&self) -> ::windows::core::Result<super::super::Foundation::TimeSpan>;
    fn AppGpuDuration(&self) -> ::windows::core::Result<super::super::Foundation::TimeSpan>;
    fn AppGpuOverrun(&self) -> ::windows::core::Result<super::super::Foundation::TimeSpan>;
    fn MissedPresentationOpportunityCount(&self) -> ::windows::core::Result<u32>;
    fn PresentationCount(&self) -> ::windows::core::Result<u32>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IHolographicFrameRenderingReportImpl: Sized {
    fn FrameId(&self) -> ::windows::core::Result<HolographicFrameId>;
    fn MissedLatchCount(&self) -> ::windows::core::Result<u32>;
    fn SystemRelativeFrameReadyTime(&self) -> ::windows::core::Result<super::super::Foundation::TimeSpan>;
    fn SystemRelativeActualGpuFinishTime(&self) -> ::windows::core::Result<super::super::Foundation::TimeSpan>;
    fn SystemRelativeTargetLatchTime(&self) -> ::windows::core::Result<super::super::Foundation::TimeSpan>;
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
pub trait IHolographicFrameScanoutMonitorImpl: Sized + IClosableImpl {
    fn ReadReports(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVector<HolographicFrameScanoutReport>>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IHolographicFrameScanoutReportImpl: Sized {
    fn RenderingReport(&self) -> ::windows::core::Result<HolographicFrameRenderingReport>;
    fn MissedScanoutCount(&self) -> ::windows::core::Result<u32>;
    fn SystemRelativeLatchTime(&self) -> ::windows::core::Result<super::super::Foundation::TimeSpan>;
    fn SystemRelativeScanoutStartTime(&self) -> ::windows::core::Result<super::super::Foundation::TimeSpan>;
    fn SystemRelativePhotonTime(&self) -> ::windows::core::Result<super::super::Foundation::TimeSpan>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IHolographicQuadLayerImpl: Sized {
    fn PixelFormat(&self) -> ::windows::core::Result<super::DirectX::DirectXPixelFormat>;
    fn Size(&self) -> ::windows::core::Result<super::super::Foundation::Size>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IHolographicQuadLayerFactoryImpl: Sized {
    fn Create(&self, size: &super::super::Foundation::Size) -> ::windows::core::Result<HolographicQuadLayer>;
    fn CreateWithPixelFormat(&self, size: &super::super::Foundation::Size, pixelformat: super::DirectX::DirectXPixelFormat) -> ::windows::core::Result<HolographicQuadLayer>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IHolographicQuadLayerUpdateParametersImpl: Sized {
    fn AcquireBufferToUpdateContent(&self) -> ::windows::core::Result<super::DirectX::Direct3D11::IDirect3DSurface>;
    fn UpdateViewport(&self, value: &super::super::Foundation::Rect) -> ::windows::core::Result<()>;
    fn UpdateContentProtectionEnabled(&self, value: bool) -> ::windows::core::Result<()>;
    fn UpdateExtents(&self, value: &super::super::Foundation::Numerics::Vector2) -> ::windows::core::Result<()>;
    fn UpdateLocationWithStationaryMode(&self, coordinatesystem: &::core::option::Option<super::super::Perception::Spatial::SpatialCoordinateSystem>, position: &super::super::Foundation::Numerics::Vector3, orientation: &super::super::Foundation::Numerics::Quaternion) -> ::windows::core::Result<()>;
    fn UpdateLocationWithDisplayRelativeMode(&self, position: &super::super::Foundation::Numerics::Vector3, orientation: &super::super::Foundation::Numerics::Quaternion) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IHolographicQuadLayerUpdateParameters2Impl: Sized {
    fn CanAcquireWithHardwareProtection(&self) -> ::windows::core::Result<bool>;
    fn AcquireBufferToUpdateContentWithHardwareProtection(&self) -> ::windows::core::Result<super::DirectX::Direct3D11::IDirect3DSurface>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IHolographicSpaceImpl: Sized {
    fn PrimaryAdapterId(&self) -> ::windows::core::Result<HolographicAdapterId>;
    fn SetDirect3D11Device(&self, value: &::core::option::Option<super::DirectX::Direct3D11::IDirect3DDevice>) -> ::windows::core::Result<()>;
    fn CameraAdded(&self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<HolographicSpace, HolographicSpaceCameraAddedEventArgs>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveCameraAdded(&self, cookie: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn CameraRemoved(&self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<HolographicSpace, HolographicSpaceCameraRemovedEventArgs>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveCameraRemoved(&self, cookie: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn CreateNextFrame(&self) -> ::windows::core::Result<HolographicFrame>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IHolographicSpace2Impl: Sized {
    fn UserPresence(&self) -> ::windows::core::Result<HolographicSpaceUserPresence>;
    fn UserPresenceChanged(&self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<HolographicSpace, ::windows::core::IInspectable>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveUserPresenceChanged(&self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn WaitForNextFrameReady(&self) -> ::windows::core::Result<()>;
    fn WaitForNextFrameReadyWithHeadStart(&self, requestedheadstartduration: &super::super::Foundation::TimeSpan) -> ::windows::core::Result<()>;
    fn CreateFramePresentationMonitor(&self, maxqueuedreports: u32) -> ::windows::core::Result<HolographicFramePresentationMonitor>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IHolographicSpace3Impl: Sized {
    fn CreateFrameScanoutMonitor(&self, maxqueuedreports: u32) -> ::windows::core::Result<HolographicFrameScanoutMonitor>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IHolographicSpaceCameraAddedEventArgsImpl: Sized {
    fn Camera(&self) -> ::windows::core::Result<HolographicCamera>;
    fn GetDeferral(&self) -> ::windows::core::Result<super::super::Foundation::Deferral>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IHolographicSpaceCameraRemovedEventArgsImpl: Sized {
    fn Camera(&self) -> ::windows::core::Result<HolographicCamera>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IHolographicSpaceStaticsImpl: Sized {
    fn CreateForCoreWindow(&self, window: &::core::option::Option<super::super::UI::Core::CoreWindow>) -> ::windows::core::Result<HolographicSpace>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IHolographicSpaceStatics2Impl: Sized {
    fn IsSupported(&self) -> ::windows::core::Result<bool>;
    fn IsAvailable(&self) -> ::windows::core::Result<bool>;
    fn IsAvailableChanged(&self, handler: &::core::option::Option<super::super::Foundation::EventHandler<::windows::core::IInspectable>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveIsAvailableChanged(&self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IHolographicSpaceStatics3Impl: Sized {
    fn IsConfigured(&self) -> ::windows::core::Result<bool>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IHolographicViewConfigurationImpl: Sized {
    fn NativeRenderTargetSize(&self) -> ::windows::core::Result<super::super::Foundation::Size>;
    fn RenderTargetSize(&self) -> ::windows::core::Result<super::super::Foundation::Size>;
    fn RequestRenderTargetSize(&self, size: &super::super::Foundation::Size) -> ::windows::core::Result<super::super::Foundation::Size>;
    fn SupportedPixelFormats(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<super::DirectX::DirectXPixelFormat>>;
    fn PixelFormat(&self) -> ::windows::core::Result<super::DirectX::DirectXPixelFormat>;
    fn SetPixelFormat(&self, value: super::DirectX::DirectXPixelFormat) -> ::windows::core::Result<()>;
    fn IsStereo(&self) -> ::windows::core::Result<bool>;
    fn RefreshRate(&self) -> ::windows::core::Result<f64>;
    fn Kind(&self) -> ::windows::core::Result<HolographicViewConfigurationKind>;
    fn Display(&self) -> ::windows::core::Result<HolographicDisplay>;
    fn IsEnabled(&self) -> ::windows::core::Result<bool>;
    fn SetIsEnabled(&self, value: bool) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IHolographicViewConfiguration2Impl: Sized {
    fn SupportedDepthReprojectionMethods(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<HolographicDepthReprojectionMethod>>;
}

#[cfg(feature = "implement_exclusive")]
pub trait ICameraIntrinsicsImpl: Sized {
    fn FocalLength(&self) -> ::windows::core::Result<super::super::super::Foundation::Numerics::Vector2>;
    fn PrincipalPoint(&self) -> ::windows::core::Result<super::super::super::Foundation::Numerics::Vector2>;
    fn RadialDistortion(&self) -> ::windows::core::Result<super::super::super::Foundation::Numerics::Vector3>;
    fn TangentialDistortion(&self) -> ::windows::core::Result<super::super::super::Foundation::Numerics::Vector2>;
    fn ImageWidth(&self) -> ::windows::core::Result<u32>;
    fn ImageHeight(&self) -> ::windows::core::Result<u32>;
    fn ProjectOntoFrame(&self, coordinate: &super::super::super::Foundation::Numerics::Vector3) -> ::windows::core::Result<super::super::super::Foundation::Point>;
    fn UnprojectAtUnitDepth(&self, pixelcoordinate: &super::super::super::Foundation::Point) -> ::windows::core::Result<super::super::super::Foundation::Numerics::Vector2>;
    fn ProjectManyOntoFrame(&self, coordinates: &[<super::super::super::Foundation::Numerics::Vector3 as ::windows::core::DefaultType>::DefaultType], results: &mut [<super::super::super::Foundation::Point as ::windows::core::DefaultType>::DefaultType]) -> ::windows::core::Result<()>;
    fn UnprojectPixelsAtUnitDepth(&self, pixelcoordinates: &[<super::super::super::Foundation::Point as ::windows::core::DefaultType>::DefaultType], results: &mut [<super::super::super::Foundation::Numerics::Vector2 as ::windows::core::DefaultType>::DefaultType]) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait ICameraIntrinsics2Impl: Sized {
    fn UndistortedProjectionTransform(&self) -> ::windows::core::Result<super::super::super::Foundation::Numerics::Matrix4x4>;
    fn DistortPoint(&self, input: &super::super::super::Foundation::Point) -> ::windows::core::Result<super::super::super::Foundation::Point>;
    fn DistortPoints(&self, inputs: &[<super::super::super::Foundation::Point as ::windows::core::DefaultType>::DefaultType], results: &mut [<super::super::super::Foundation::Point as ::windows::core::DefaultType>::DefaultType]) -> ::windows::core::Result<()>;
    fn UndistortPoint(&self, input: &super::super::super::Foundation::Point) -> ::windows::core::Result<super::super::super::Foundation::Point>;
    fn UndistortPoints(&self, inputs: &[<super::super::super::Foundation::Point as ::windows::core::DefaultType>::DefaultType], results: &mut [<super::super::super::Foundation::Point as ::windows::core::DefaultType>::DefaultType]) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait ICameraIntrinsicsFactoryImpl: Sized {
    fn Create(&self, focallength: &super::super::super::Foundation::Numerics::Vector2, principalpoint: &super::super::super::Foundation::Numerics::Vector2, radialdistortion: &super::super::super::Foundation::Numerics::Vector3, tangentialdistortion: &super::super::super::Foundation::Numerics::Vector2, imagewidth: u32, imageheight: u32) -> ::windows::core::Result<CameraIntrinsics>;
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
pub trait IDepthCorrelatedCoordinateMapperImpl: Sized + IClosableImpl {
    fn UnprojectPoint(&self, sourcepoint: &super::super::super::Foundation::Point, targetcoordinatesystem: &::core::option::Option<super::super::super::Perception::Spatial::SpatialCoordinateSystem>) -> ::windows::core::Result<super::super::super::Foundation::Numerics::Vector3>;
    fn UnprojectPoints(&self, sourcepoints: &[<super::super::super::Foundation::Point as ::windows::core::DefaultType>::DefaultType], targetcoordinatesystem: &::core::option::Option<super::super::super::Perception::Spatial::SpatialCoordinateSystem>, results: &mut [<super::super::super::Foundation::Numerics::Vector3 as ::windows::core::DefaultType>::DefaultType]) -> ::windows::core::Result<()>;
    fn MapPoint(&self, sourcepoint: &super::super::super::Foundation::Point, targetcoordinatesystem: &::core::option::Option<super::super::super::Perception::Spatial::SpatialCoordinateSystem>, targetcameraintrinsics: &::core::option::Option<CameraIntrinsics>) -> ::windows::core::Result<super::super::super::Foundation::Point>;
    fn MapPoints(&self, sourcepoints: &[<super::super::super::Foundation::Point as ::windows::core::DefaultType>::DefaultType], targetcoordinatesystem: &::core::option::Option<super::super::super::Perception::Spatial::SpatialCoordinateSystem>, targetcameraintrinsics: &::core::option::Option<CameraIntrinsics>, results: &mut [<super::super::super::Foundation::Point as ::windows::core::DefaultType>::DefaultType]) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IFrameControlCapabilitiesImpl: Sized {
    fn Exposure(&self) -> ::windows::core::Result<FrameExposureCapabilities>;
    fn ExposureCompensation(&self) -> ::windows::core::Result<FrameExposureCompensationCapabilities>;
    fn IsoSpeed(&self) -> ::windows::core::Result<FrameIsoSpeedCapabilities>;
    fn Focus(&self) -> ::windows::core::Result<FrameFocusCapabilities>;
    fn PhotoConfirmationSupported(&self) -> ::windows::core::Result<bool>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IFrameControlCapabilities2Impl: Sized {
    fn Flash(&self) -> ::windows::core::Result<FrameFlashCapabilities>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IFrameControllerImpl: Sized {
    fn ExposureControl(&self) -> ::windows::core::Result<FrameExposureControl>;
    fn ExposureCompensationControl(&self) -> ::windows::core::Result<FrameExposureCompensationControl>;
    fn IsoSpeedControl(&self) -> ::windows::core::Result<FrameIsoSpeedControl>;
    fn FocusControl(&self) -> ::windows::core::Result<FrameFocusControl>;
    fn PhotoConfirmationEnabled(&self) -> ::windows::core::Result<super::super::super::Foundation::IReference<bool>>;
    fn SetPhotoConfirmationEnabled(&self, value: &::core::option::Option<super::super::super::Foundation::IReference<bool>>) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IFrameController2Impl: Sized {
    fn FlashControl(&self) -> ::windows::core::Result<FrameFlashControl>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IFrameExposureCapabilitiesImpl: Sized {
    fn Supported(&self) -> ::windows::core::Result<bool>;
    fn Min(&self) -> ::windows::core::Result<super::super::super::Foundation::TimeSpan>;
    fn Max(&self) -> ::windows::core::Result<super::super::super::Foundation::TimeSpan>;
    fn Step(&self) -> ::windows::core::Result<super::super::super::Foundation::TimeSpan>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IFrameExposureCompensationCapabilitiesImpl: Sized {
    fn Supported(&self) -> ::windows::core::Result<bool>;
    fn Min(&self) -> ::windows::core::Result<f32>;
    fn Max(&self) -> ::windows::core::Result<f32>;
    fn Step(&self) -> ::windows::core::Result<f32>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IFrameExposureCompensationControlImpl: Sized {
    fn Value(&self) -> ::windows::core::Result<super::super::super::Foundation::IReference<f32>>;
    fn SetValue(&self, value: &::core::option::Option<super::super::super::Foundation::IReference<f32>>) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IFrameExposureControlImpl: Sized {
    fn Auto(&self) -> ::windows::core::Result<bool>;
    fn SetAuto(&self, value: bool) -> ::windows::core::Result<()>;
    fn Value(&self) -> ::windows::core::Result<super::super::super::Foundation::IReference<super::super::super::Foundation::TimeSpan>>;
    fn SetValue(&self, value: &::core::option::Option<super::super::super::Foundation::IReference<super::super::super::Foundation::TimeSpan>>) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IFrameFlashCapabilitiesImpl: Sized {
    fn Supported(&self) -> ::windows::core::Result<bool>;
    fn RedEyeReductionSupported(&self) -> ::windows::core::Result<bool>;
    fn PowerSupported(&self) -> ::windows::core::Result<bool>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IFrameFlashControlImpl: Sized {
    fn Mode(&self) -> ::windows::core::Result<FrameFlashMode>;
    fn SetMode(&self, value: FrameFlashMode) -> ::windows::core::Result<()>;
    fn Auto(&self) -> ::windows::core::Result<bool>;
    fn SetAuto(&self, value: bool) -> ::windows::core::Result<()>;
    fn RedEyeReduction(&self) -> ::windows::core::Result<bool>;
    fn SetRedEyeReduction(&self, value: bool) -> ::windows::core::Result<()>;
    fn PowerPercent(&self) -> ::windows::core::Result<f32>;
    fn SetPowerPercent(&self, value: f32) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IFrameFocusCapabilitiesImpl: Sized {
    fn Supported(&self) -> ::windows::core::Result<bool>;
    fn Min(&self) -> ::windows::core::Result<u32>;
    fn Max(&self) -> ::windows::core::Result<u32>;
    fn Step(&self) -> ::windows::core::Result<u32>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IFrameFocusControlImpl: Sized {
    fn Value(&self) -> ::windows::core::Result<super::super::super::Foundation::IReference<u32>>;
    fn SetValue(&self, value: &::core::option::Option<super::super::super::Foundation::IReference<u32>>) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IFrameIsoSpeedCapabilitiesImpl: Sized {
    fn Supported(&self) -> ::windows::core::Result<bool>;
    fn Min(&self) -> ::windows::core::Result<u32>;
    fn Max(&self) -> ::windows::core::Result<u32>;
    fn Step(&self) -> ::windows::core::Result<u32>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IFrameIsoSpeedControlImpl: Sized {
    fn Auto(&self) -> ::windows::core::Result<bool>;
    fn SetAuto(&self, value: bool) -> ::windows::core::Result<()>;
    fn Value(&self) -> ::windows::core::Result<super::super::super::Foundation::IReference<u32>>;
    fn SetValue(&self, value: &::core::option::Option<super::super::super::Foundation::IReference<u32>>) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IVariablePhotoSequenceControllerImpl: Sized {
    fn Supported(&self) -> ::windows::core::Result<bool>;
    fn MaxPhotosPerSecond(&self) -> ::windows::core::Result<f32>;
    fn PhotosPerSecondLimit(&self) -> ::windows::core::Result<f32>;
    fn SetPhotosPerSecondLimit(&self, value: f32) -> ::windows::core::Result<()>;
    fn GetHighestConcurrentFrameRate(&self, captureproperties: &::core::option::Option<super::super::MediaProperties::IMediaEncodingProperties>) -> ::windows::core::Result<super::super::MediaProperties::MediaRatio>;
    fn GetCurrentFrameRate(&self) -> ::windows::core::Result<super::super::MediaProperties::MediaRatio>;
    fn FrameCapabilities(&self) -> ::windows::core::Result<FrameControlCapabilities>;
    fn DesiredFrameControllers(&self) -> ::windows::core::Result<super::super::super::Foundation::Collections::IVector<FrameController>>;
}

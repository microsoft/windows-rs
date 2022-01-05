#[cfg(all(feature = "deprecated", feature = "implement_exclusive"))]
pub trait IKnownCameraIntrinsicsPropertiesStaticsImpl: Sized {
    fn FocalLength(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn PrincipalPoint(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn RadialDistortion(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn TangentialDistortion(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
}
#[cfg(all(feature = "deprecated", feature = "implement_exclusive"))]
pub trait IKnownPerceptionColorFrameSourcePropertiesStaticsImpl: Sized {
    fn Exposure(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn AutoExposureEnabled(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn ExposureCompensation(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
}
#[cfg(all(feature = "deprecated", feature = "implement_exclusive"))]
pub trait IKnownPerceptionDepthFrameSourcePropertiesStaticsImpl: Sized {
    fn MinDepth(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn MaxDepth(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
}
#[cfg(all(feature = "deprecated", feature = "implement_exclusive"))]
pub trait IKnownPerceptionFrameSourcePropertiesStaticsImpl: Sized {
    fn Id(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn PhysicalDeviceIds(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn FrameKind(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn DeviceModelVersion(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn EnclosureLocation(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
}
#[cfg(all(feature = "deprecated", feature = "implement_exclusive"))]
pub trait IKnownPerceptionFrameSourcePropertiesStatics2Impl: Sized {
    fn DeviceId(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
}
#[cfg(all(feature = "deprecated", feature = "implement_exclusive"))]
pub trait IKnownPerceptionInfraredFrameSourcePropertiesStaticsImpl: Sized {
    fn Exposure(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn AutoExposureEnabled(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn ExposureCompensation(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn ActiveIlluminationEnabled(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn AmbientSubtractionEnabled(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn StructureLightPatternEnabled(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn InterleavedIlluminationEnabled(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
}
#[cfg(all(feature = "deprecated", feature = "implement_exclusive"))]
pub trait IKnownPerceptionVideoFrameSourcePropertiesStaticsImpl: Sized {
    fn VideoProfile(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SupportedVideoProfiles(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn AvailableVideoProfiles(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn IsMirrored(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn CameraIntrinsics(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
}
#[cfg(all(feature = "deprecated", feature = "implement_exclusive"))]
pub trait IKnownPerceptionVideoProfilePropertiesStaticsImpl: Sized {
    fn BitmapPixelFormat(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn BitmapAlphaMode(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn Width(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn Height(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn FrameDuration(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
}
#[cfg(all(feature = "Foundation", feature = "deprecated", feature = "implement_exclusive"))]
pub trait IPerceptionColorFrameImpl: Sized + IClosableImpl {
    fn VideoFrame(&self) -> ::windows::core::Result<super::super::Media::VideoFrame>;
}
#[cfg(all(feature = "deprecated", feature = "implement_exclusive"))]
pub trait IPerceptionColorFrameArrivedEventArgsImpl: Sized {
    fn RelativeTime(&self) -> ::windows::core::Result<super::super::Foundation::TimeSpan>;
    fn TryOpenFrame(&self) -> ::windows::core::Result<PerceptionColorFrame>;
}
#[cfg(all(feature = "Foundation", feature = "deprecated", feature = "implement_exclusive"))]
pub trait IPerceptionColorFrameReaderImpl: Sized + IClosableImpl {
    fn FrameArrived(&self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<PerceptionColorFrameReader, PerceptionColorFrameArrivedEventArgs>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveFrameArrived(&self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn Source(&self) -> ::windows::core::Result<PerceptionColorFrameSource>;
    fn IsPaused(&self) -> ::windows::core::Result<bool>;
    fn SetIsPaused(&self, value: bool) -> ::windows::core::Result<()>;
    fn TryReadLatestFrame(&self) -> ::windows::core::Result<PerceptionColorFrame>;
}
#[cfg(all(feature = "deprecated", feature = "implement_exclusive"))]
pub trait IPerceptionColorFrameSourceImpl: Sized {
    fn AvailableChanged(&self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<PerceptionColorFrameSource, ::windows::core::IInspectable>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveAvailableChanged(&self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn ActiveChanged(&self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<PerceptionColorFrameSource, ::windows::core::IInspectable>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveActiveChanged(&self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn PropertiesChanged(&self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<PerceptionColorFrameSource, PerceptionFrameSourcePropertiesChangedEventArgs>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemovePropertiesChanged(&self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn VideoProfileChanged(&self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<PerceptionColorFrameSource, ::windows::core::IInspectable>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveVideoProfileChanged(&self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn CameraIntrinsicsChanged(&self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<PerceptionColorFrameSource, ::windows::core::IInspectable>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveCameraIntrinsicsChanged(&self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn Id(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn DisplayName(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn DeviceKind(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn Available(&self) -> ::windows::core::Result<bool>;
    fn Active(&self) -> ::windows::core::Result<bool>;
    fn IsControlled(&self) -> ::windows::core::Result<bool>;
    fn Properties(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IMapView<::windows::core::HSTRING, ::windows::core::IInspectable>>;
    fn SupportedVideoProfiles(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<PerceptionVideoProfile>>;
    fn AvailableVideoProfiles(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<PerceptionVideoProfile>>;
    fn VideoProfile(&self) -> ::windows::core::Result<PerceptionVideoProfile>;
    fn CameraIntrinsics(&self) -> ::windows::core::Result<super::super::Media::Devices::Core::CameraIntrinsics>;
    fn AcquireControlSession(&self) -> ::windows::core::Result<PerceptionControlSession>;
    fn CanControlIndependentlyFrom(&self, targetid: &::windows::core::HSTRING) -> ::windows::core::Result<bool>;
    fn IsCorrelatedWith(&self, targetid: &::windows::core::HSTRING) -> ::windows::core::Result<bool>;
    fn TryGetTransformTo(&self, targetid: &::windows::core::HSTRING, result: &mut super::super::Foundation::Numerics::Matrix4x4) -> ::windows::core::Result<bool>;
    fn TryGetDepthCorrelatedCameraIntrinsicsAsync(&self, correlateddepthframesource: &::core::option::Option<PerceptionDepthFrameSource>) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<PerceptionDepthCorrelatedCameraIntrinsics>>;
    fn TryGetDepthCorrelatedCoordinateMapperAsync(&self, targetsourceid: &::windows::core::HSTRING, correlateddepthframesource: &::core::option::Option<PerceptionDepthFrameSource>) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<PerceptionDepthCorrelatedCoordinateMapper>>;
    fn TrySetVideoProfileAsync(&self, controlsession: &::core::option::Option<PerceptionControlSession>, profile: &::core::option::Option<PerceptionVideoProfile>) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<PerceptionFrameSourcePropertyChangeResult>>;
    fn OpenReader(&self) -> ::windows::core::Result<PerceptionColorFrameReader>;
}
#[cfg(all(feature = "deprecated", feature = "implement_exclusive"))]
pub trait IPerceptionColorFrameSource2Impl: Sized {
    fn DeviceId(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
}
#[cfg(all(feature = "deprecated", feature = "implement_exclusive"))]
pub trait IPerceptionColorFrameSourceAddedEventArgsImpl: Sized {
    fn FrameSource(&self) -> ::windows::core::Result<PerceptionColorFrameSource>;
}
#[cfg(all(feature = "deprecated", feature = "implement_exclusive"))]
pub trait IPerceptionColorFrameSourceRemovedEventArgsImpl: Sized {
    fn FrameSource(&self) -> ::windows::core::Result<PerceptionColorFrameSource>;
}
#[cfg(all(feature = "deprecated", feature = "implement_exclusive"))]
pub trait IPerceptionColorFrameSourceStaticsImpl: Sized {
    fn CreateWatcher(&self) -> ::windows::core::Result<PerceptionColorFrameSourceWatcher>;
    fn FindAllAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IVectorView<PerceptionColorFrameSource>>>;
    fn FromIdAsync(&self, id: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<PerceptionColorFrameSource>>;
    fn RequestAccessAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<PerceptionFrameSourceAccessStatus>>;
}
#[cfg(all(feature = "deprecated", feature = "implement_exclusive"))]
pub trait IPerceptionColorFrameSourceWatcherImpl: Sized {
    fn SourceAdded(&self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<PerceptionColorFrameSourceWatcher, PerceptionColorFrameSourceAddedEventArgs>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveSourceAdded(&self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn SourceRemoved(&self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<PerceptionColorFrameSourceWatcher, PerceptionColorFrameSourceRemovedEventArgs>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveSourceRemoved(&self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn Stopped(&self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<PerceptionColorFrameSourceWatcher, ::windows::core::IInspectable>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveStopped(&self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn EnumerationCompleted(&self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<PerceptionColorFrameSourceWatcher, ::windows::core::IInspectable>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveEnumerationCompleted(&self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn Status(&self) -> ::windows::core::Result<super::Enumeration::DeviceWatcherStatus>;
    fn Start(&self) -> ::windows::core::Result<()>;
    fn Stop(&self) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Foundation", feature = "deprecated", feature = "implement_exclusive"))]
pub trait IPerceptionControlSessionImpl: Sized + IClosableImpl {
    fn ControlLost(&self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<PerceptionControlSession, ::windows::core::IInspectable>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveControlLost(&self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn TrySetPropertyAsync(&self, name: &::windows::core::HSTRING, value: &::core::option::Option<::windows::core::IInspectable>) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<PerceptionFrameSourcePropertyChangeResult>>;
}
#[cfg(all(feature = "deprecated", feature = "implement_exclusive"))]
pub trait IPerceptionDepthCorrelatedCameraIntrinsicsImpl: Sized {
    fn UnprojectPixelAtCorrelatedDepth(&self, pixelcoordinate: &super::super::Foundation::Point, depthframe: &::core::option::Option<PerceptionDepthFrame>) -> ::windows::core::Result<super::super::Foundation::Numerics::Vector3>;
    fn UnprojectPixelsAtCorrelatedDepth(&self, sourcecoordinates: &[<super::super::Foundation::Point as ::windows::core::DefaultType>::DefaultType], depthframe: &::core::option::Option<PerceptionDepthFrame>, results: &mut [<super::super::Foundation::Numerics::Vector3 as ::windows::core::DefaultType>::DefaultType]) -> ::windows::core::Result<()>;
    fn UnprojectRegionPixelsAtCorrelatedDepthAsync(&self, region: &super::super::Foundation::Rect, depthframe: &::core::option::Option<PerceptionDepthFrame>, results: &mut [<super::super::Foundation::Numerics::Vector3 as ::windows::core::DefaultType>::DefaultType]) -> ::windows::core::Result<super::super::Foundation::IAsyncAction>;
    fn UnprojectAllPixelsAtCorrelatedDepthAsync(&self, depthframe: &::core::option::Option<PerceptionDepthFrame>, results: &mut [<super::super::Foundation::Numerics::Vector3 as ::windows::core::DefaultType>::DefaultType]) -> ::windows::core::Result<super::super::Foundation::IAsyncAction>;
}
#[cfg(all(feature = "deprecated", feature = "implement_exclusive"))]
pub trait IPerceptionDepthCorrelatedCoordinateMapperImpl: Sized {
    fn MapPixelToTarget(&self, sourcepixelcoordinate: &super::super::Foundation::Point, depthframe: &::core::option::Option<PerceptionDepthFrame>) -> ::windows::core::Result<super::super::Foundation::Point>;
    fn MapPixelsToTarget(&self, sourcecoordinates: &[<super::super::Foundation::Point as ::windows::core::DefaultType>::DefaultType], depthframe: &::core::option::Option<PerceptionDepthFrame>, results: &mut [<super::super::Foundation::Point as ::windows::core::DefaultType>::DefaultType]) -> ::windows::core::Result<()>;
    fn MapRegionOfPixelsToTargetAsync(&self, region: &super::super::Foundation::Rect, depthframe: &::core::option::Option<PerceptionDepthFrame>, targetcoordinates: &mut [<super::super::Foundation::Point as ::windows::core::DefaultType>::DefaultType]) -> ::windows::core::Result<super::super::Foundation::IAsyncAction>;
    fn MapAllPixelsToTargetAsync(&self, depthframe: &::core::option::Option<PerceptionDepthFrame>, targetcoordinates: &mut [<super::super::Foundation::Point as ::windows::core::DefaultType>::DefaultType]) -> ::windows::core::Result<super::super::Foundation::IAsyncAction>;
}
#[cfg(all(feature = "Foundation", feature = "deprecated", feature = "implement_exclusive"))]
pub trait IPerceptionDepthFrameImpl: Sized + IClosableImpl {
    fn VideoFrame(&self) -> ::windows::core::Result<super::super::Media::VideoFrame>;
}
#[cfg(all(feature = "deprecated", feature = "implement_exclusive"))]
pub trait IPerceptionDepthFrameArrivedEventArgsImpl: Sized {
    fn RelativeTime(&self) -> ::windows::core::Result<super::super::Foundation::TimeSpan>;
    fn TryOpenFrame(&self) -> ::windows::core::Result<PerceptionDepthFrame>;
}
#[cfg(all(feature = "Foundation", feature = "deprecated", feature = "implement_exclusive"))]
pub trait IPerceptionDepthFrameReaderImpl: Sized + IClosableImpl {
    fn FrameArrived(&self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<PerceptionDepthFrameReader, PerceptionDepthFrameArrivedEventArgs>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveFrameArrived(&self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn Source(&self) -> ::windows::core::Result<PerceptionDepthFrameSource>;
    fn IsPaused(&self) -> ::windows::core::Result<bool>;
    fn SetIsPaused(&self, value: bool) -> ::windows::core::Result<()>;
    fn TryReadLatestFrame(&self) -> ::windows::core::Result<PerceptionDepthFrame>;
}
#[cfg(all(feature = "deprecated", feature = "implement_exclusive"))]
pub trait IPerceptionDepthFrameSourceImpl: Sized {
    fn AvailableChanged(&self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<PerceptionDepthFrameSource, ::windows::core::IInspectable>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveAvailableChanged(&self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn ActiveChanged(&self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<PerceptionDepthFrameSource, ::windows::core::IInspectable>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveActiveChanged(&self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn PropertiesChanged(&self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<PerceptionDepthFrameSource, PerceptionFrameSourcePropertiesChangedEventArgs>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemovePropertiesChanged(&self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn VideoProfileChanged(&self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<PerceptionDepthFrameSource, ::windows::core::IInspectable>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveVideoProfileChanged(&self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn CameraIntrinsicsChanged(&self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<PerceptionDepthFrameSource, ::windows::core::IInspectable>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveCameraIntrinsicsChanged(&self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn Id(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn DisplayName(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn DeviceKind(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn Available(&self) -> ::windows::core::Result<bool>;
    fn Active(&self) -> ::windows::core::Result<bool>;
    fn IsControlled(&self) -> ::windows::core::Result<bool>;
    fn Properties(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IMapView<::windows::core::HSTRING, ::windows::core::IInspectable>>;
    fn SupportedVideoProfiles(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<PerceptionVideoProfile>>;
    fn AvailableVideoProfiles(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<PerceptionVideoProfile>>;
    fn VideoProfile(&self) -> ::windows::core::Result<PerceptionVideoProfile>;
    fn CameraIntrinsics(&self) -> ::windows::core::Result<super::super::Media::Devices::Core::CameraIntrinsics>;
    fn AcquireControlSession(&self) -> ::windows::core::Result<PerceptionControlSession>;
    fn CanControlIndependentlyFrom(&self, targetid: &::windows::core::HSTRING) -> ::windows::core::Result<bool>;
    fn IsCorrelatedWith(&self, targetid: &::windows::core::HSTRING) -> ::windows::core::Result<bool>;
    fn TryGetTransformTo(&self, targetid: &::windows::core::HSTRING, result: &mut super::super::Foundation::Numerics::Matrix4x4) -> ::windows::core::Result<bool>;
    fn TryGetDepthCorrelatedCameraIntrinsicsAsync(&self, target: &::core::option::Option<PerceptionDepthFrameSource>) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<PerceptionDepthCorrelatedCameraIntrinsics>>;
    fn TryGetDepthCorrelatedCoordinateMapperAsync(&self, targetid: &::windows::core::HSTRING, depthframesourcetomapwith: &::core::option::Option<PerceptionDepthFrameSource>) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<PerceptionDepthCorrelatedCoordinateMapper>>;
    fn TrySetVideoProfileAsync(&self, controlsession: &::core::option::Option<PerceptionControlSession>, profile: &::core::option::Option<PerceptionVideoProfile>) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<PerceptionFrameSourcePropertyChangeResult>>;
    fn OpenReader(&self) -> ::windows::core::Result<PerceptionDepthFrameReader>;
}
#[cfg(all(feature = "deprecated", feature = "implement_exclusive"))]
pub trait IPerceptionDepthFrameSource2Impl: Sized {
    fn DeviceId(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
}
#[cfg(all(feature = "deprecated", feature = "implement_exclusive"))]
pub trait IPerceptionDepthFrameSourceAddedEventArgsImpl: Sized {
    fn FrameSource(&self) -> ::windows::core::Result<PerceptionDepthFrameSource>;
}
#[cfg(all(feature = "deprecated", feature = "implement_exclusive"))]
pub trait IPerceptionDepthFrameSourceRemovedEventArgsImpl: Sized {
    fn FrameSource(&self) -> ::windows::core::Result<PerceptionDepthFrameSource>;
}
#[cfg(all(feature = "deprecated", feature = "implement_exclusive"))]
pub trait IPerceptionDepthFrameSourceStaticsImpl: Sized {
    fn CreateWatcher(&self) -> ::windows::core::Result<PerceptionDepthFrameSourceWatcher>;
    fn FindAllAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IVectorView<PerceptionDepthFrameSource>>>;
    fn FromIdAsync(&self, id: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<PerceptionDepthFrameSource>>;
    fn RequestAccessAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<PerceptionFrameSourceAccessStatus>>;
}
#[cfg(all(feature = "deprecated", feature = "implement_exclusive"))]
pub trait IPerceptionDepthFrameSourceWatcherImpl: Sized {
    fn SourceAdded(&self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<PerceptionDepthFrameSourceWatcher, PerceptionDepthFrameSourceAddedEventArgs>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveSourceAdded(&self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn SourceRemoved(&self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<PerceptionDepthFrameSourceWatcher, PerceptionDepthFrameSourceRemovedEventArgs>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveSourceRemoved(&self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn Stopped(&self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<PerceptionDepthFrameSourceWatcher, ::windows::core::IInspectable>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveStopped(&self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn EnumerationCompleted(&self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<PerceptionDepthFrameSourceWatcher, ::windows::core::IInspectable>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveEnumerationCompleted(&self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn Status(&self) -> ::windows::core::Result<super::Enumeration::DeviceWatcherStatus>;
    fn Start(&self) -> ::windows::core::Result<()>;
    fn Stop(&self) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "deprecated", feature = "implement_exclusive"))]
pub trait IPerceptionFrameSourcePropertiesChangedEventArgsImpl: Sized {
    fn CollectionChange(&self) -> ::windows::core::Result<super::super::Foundation::Collections::CollectionChange>;
    fn Key(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
}
#[cfg(all(feature = "deprecated", feature = "implement_exclusive"))]
pub trait IPerceptionFrameSourcePropertyChangeResultImpl: Sized {
    fn Status(&self) -> ::windows::core::Result<PerceptionFrameSourcePropertyChangeStatus>;
    fn NewValue(&self) -> ::windows::core::Result<::windows::core::IInspectable>;
}
#[cfg(all(feature = "Foundation", feature = "deprecated", feature = "implement_exclusive"))]
pub trait IPerceptionInfraredFrameImpl: Sized + IClosableImpl {
    fn VideoFrame(&self) -> ::windows::core::Result<super::super::Media::VideoFrame>;
}
#[cfg(all(feature = "deprecated", feature = "implement_exclusive"))]
pub trait IPerceptionInfraredFrameArrivedEventArgsImpl: Sized {
    fn RelativeTime(&self) -> ::windows::core::Result<super::super::Foundation::TimeSpan>;
    fn TryOpenFrame(&self) -> ::windows::core::Result<PerceptionInfraredFrame>;
}
#[cfg(all(feature = "Foundation", feature = "deprecated", feature = "implement_exclusive"))]
pub trait IPerceptionInfraredFrameReaderImpl: Sized + IClosableImpl {
    fn FrameArrived(&self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<PerceptionInfraredFrameReader, PerceptionInfraredFrameArrivedEventArgs>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveFrameArrived(&self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn Source(&self) -> ::windows::core::Result<PerceptionInfraredFrameSource>;
    fn IsPaused(&self) -> ::windows::core::Result<bool>;
    fn SetIsPaused(&self, value: bool) -> ::windows::core::Result<()>;
    fn TryReadLatestFrame(&self) -> ::windows::core::Result<PerceptionInfraredFrame>;
}
#[cfg(all(feature = "deprecated", feature = "implement_exclusive"))]
pub trait IPerceptionInfraredFrameSourceImpl: Sized {
    fn AvailableChanged(&self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<PerceptionInfraredFrameSource, ::windows::core::IInspectable>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveAvailableChanged(&self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn ActiveChanged(&self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<PerceptionInfraredFrameSource, ::windows::core::IInspectable>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveActiveChanged(&self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn PropertiesChanged(&self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<PerceptionInfraredFrameSource, PerceptionFrameSourcePropertiesChangedEventArgs>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemovePropertiesChanged(&self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn VideoProfileChanged(&self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<PerceptionInfraredFrameSource, ::windows::core::IInspectable>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveVideoProfileChanged(&self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn CameraIntrinsicsChanged(&self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<PerceptionInfraredFrameSource, ::windows::core::IInspectable>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveCameraIntrinsicsChanged(&self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn Id(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn DisplayName(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn DeviceKind(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn Available(&self) -> ::windows::core::Result<bool>;
    fn Active(&self) -> ::windows::core::Result<bool>;
    fn IsControlled(&self) -> ::windows::core::Result<bool>;
    fn Properties(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IMapView<::windows::core::HSTRING, ::windows::core::IInspectable>>;
    fn SupportedVideoProfiles(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<PerceptionVideoProfile>>;
    fn AvailableVideoProfiles(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<PerceptionVideoProfile>>;
    fn VideoProfile(&self) -> ::windows::core::Result<PerceptionVideoProfile>;
    fn CameraIntrinsics(&self) -> ::windows::core::Result<super::super::Media::Devices::Core::CameraIntrinsics>;
    fn AcquireControlSession(&self) -> ::windows::core::Result<PerceptionControlSession>;
    fn CanControlIndependentlyFrom(&self, targetid: &::windows::core::HSTRING) -> ::windows::core::Result<bool>;
    fn IsCorrelatedWith(&self, targetid: &::windows::core::HSTRING) -> ::windows::core::Result<bool>;
    fn TryGetTransformTo(&self, targetid: &::windows::core::HSTRING, result: &mut super::super::Foundation::Numerics::Matrix4x4) -> ::windows::core::Result<bool>;
    fn TryGetDepthCorrelatedCameraIntrinsicsAsync(&self, target: &::core::option::Option<PerceptionDepthFrameSource>) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<PerceptionDepthCorrelatedCameraIntrinsics>>;
    fn TryGetDepthCorrelatedCoordinateMapperAsync(&self, targetid: &::windows::core::HSTRING, depthframesourcetomapwith: &::core::option::Option<PerceptionDepthFrameSource>) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<PerceptionDepthCorrelatedCoordinateMapper>>;
    fn TrySetVideoProfileAsync(&self, controlsession: &::core::option::Option<PerceptionControlSession>, profile: &::core::option::Option<PerceptionVideoProfile>) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<PerceptionFrameSourcePropertyChangeResult>>;
    fn OpenReader(&self) -> ::windows::core::Result<PerceptionInfraredFrameReader>;
}
#[cfg(all(feature = "deprecated", feature = "implement_exclusive"))]
pub trait IPerceptionInfraredFrameSource2Impl: Sized {
    fn DeviceId(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
}
#[cfg(all(feature = "deprecated", feature = "implement_exclusive"))]
pub trait IPerceptionInfraredFrameSourceAddedEventArgsImpl: Sized {
    fn FrameSource(&self) -> ::windows::core::Result<PerceptionInfraredFrameSource>;
}
#[cfg(all(feature = "deprecated", feature = "implement_exclusive"))]
pub trait IPerceptionInfraredFrameSourceRemovedEventArgsImpl: Sized {
    fn FrameSource(&self) -> ::windows::core::Result<PerceptionInfraredFrameSource>;
}
#[cfg(all(feature = "deprecated", feature = "implement_exclusive"))]
pub trait IPerceptionInfraredFrameSourceStaticsImpl: Sized {
    fn CreateWatcher(&self) -> ::windows::core::Result<PerceptionInfraredFrameSourceWatcher>;
    fn FindAllAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IVectorView<PerceptionInfraredFrameSource>>>;
    fn FromIdAsync(&self, id: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<PerceptionInfraredFrameSource>>;
    fn RequestAccessAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<PerceptionFrameSourceAccessStatus>>;
}
#[cfg(all(feature = "deprecated", feature = "implement_exclusive"))]
pub trait IPerceptionInfraredFrameSourceWatcherImpl: Sized {
    fn SourceAdded(&self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<PerceptionInfraredFrameSourceWatcher, PerceptionInfraredFrameSourceAddedEventArgs>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveSourceAdded(&self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn SourceRemoved(&self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<PerceptionInfraredFrameSourceWatcher, PerceptionInfraredFrameSourceRemovedEventArgs>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveSourceRemoved(&self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn Stopped(&self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<PerceptionInfraredFrameSourceWatcher, ::windows::core::IInspectable>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveStopped(&self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn EnumerationCompleted(&self, handler: &::core::option::Option<super::super::Foundation::TypedEventHandler<PerceptionInfraredFrameSourceWatcher, ::windows::core::IInspectable>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveEnumerationCompleted(&self, token: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn Status(&self) -> ::windows::core::Result<super::Enumeration::DeviceWatcherStatus>;
    fn Start(&self) -> ::windows::core::Result<()>;
    fn Stop(&self) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "deprecated", feature = "implement_exclusive"))]
pub trait IPerceptionVideoProfileImpl: Sized {
    fn BitmapPixelFormat(&self) -> ::windows::core::Result<super::super::Graphics::Imaging::BitmapPixelFormat>;
    fn BitmapAlphaMode(&self) -> ::windows::core::Result<super::super::Graphics::Imaging::BitmapAlphaMode>;
    fn Width(&self) -> ::windows::core::Result<i32>;
    fn Height(&self) -> ::windows::core::Result<i32>;
    fn FrameDuration(&self) -> ::windows::core::Result<super::super::Foundation::TimeSpan>;
    fn IsEqual(&self, other: &::core::option::Option<PerceptionVideoProfile>) -> ::windows::core::Result<bool>;
}

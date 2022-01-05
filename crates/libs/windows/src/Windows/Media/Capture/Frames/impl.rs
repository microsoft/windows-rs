#[cfg(feature = "implement_exclusive")]
pub trait IAudioMediaFrameImpl: Sized {
    fn FrameReference(&self) -> ::windows::core::Result<MediaFrameReference>;
    fn AudioEncodingProperties(&self) -> ::windows::core::Result<super::super::MediaProperties::AudioEncodingProperties>;
    fn GetAudioFrame(&self) -> ::windows::core::Result<super::super::AudioFrame>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IBufferMediaFrameImpl: Sized {
    fn FrameReference(&self) -> ::windows::core::Result<MediaFrameReference>;
    fn Buffer(&self) -> ::windows::core::Result<super::super::super::Storage::Streams::IBuffer>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IDepthMediaFrameImpl: Sized {
    fn FrameReference(&self) -> ::windows::core::Result<MediaFrameReference>;
    fn VideoMediaFrame(&self) -> ::windows::core::Result<VideoMediaFrame>;
    fn DepthFormat(&self) -> ::windows::core::Result<DepthMediaFrameFormat>;
    fn TryCreateCoordinateMapper(&self, cameraintrinsics: &::core::option::Option<super::super::Devices::Core::CameraIntrinsics>, coordinatesystem: &::core::option::Option<super::super::super::Perception::Spatial::SpatialCoordinateSystem>) -> ::windows::core::Result<super::super::Devices::Core::DepthCorrelatedCoordinateMapper>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IDepthMediaFrame2Impl: Sized {
    fn MaxReliableDepth(&self) -> ::windows::core::Result<u32>;
    fn MinReliableDepth(&self) -> ::windows::core::Result<u32>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IDepthMediaFrameFormatImpl: Sized {
    fn VideoFormat(&self) -> ::windows::core::Result<VideoMediaFrameFormat>;
    fn DepthScaleInMeters(&self) -> ::windows::core::Result<f64>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IInfraredMediaFrameImpl: Sized {
    fn FrameReference(&self) -> ::windows::core::Result<MediaFrameReference>;
    fn VideoMediaFrame(&self) -> ::windows::core::Result<VideoMediaFrame>;
    fn IsIlluminated(&self) -> ::windows::core::Result<bool>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IMediaFrameArrivedEventArgsImpl: Sized {}
#[cfg(feature = "implement_exclusive")]
pub trait IMediaFrameFormatImpl: Sized {
    fn MajorType(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn Subtype(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn FrameRate(&self) -> ::windows::core::Result<super::super::MediaProperties::MediaRatio>;
    fn Properties(&self) -> ::windows::core::Result<super::super::super::Foundation::Collections::IMapView<::windows::core::GUID, ::windows::core::IInspectable>>;
    fn VideoFormat(&self) -> ::windows::core::Result<VideoMediaFrameFormat>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IMediaFrameFormat2Impl: Sized {
    fn AudioEncodingProperties(&self) -> ::windows::core::Result<super::super::MediaProperties::AudioEncodingProperties>;
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
pub trait IMediaFrameReaderImpl: Sized + IClosableImpl {
    fn FrameArrived(&self, handler: &::core::option::Option<super::super::super::Foundation::TypedEventHandler<MediaFrameReader, MediaFrameArrivedEventArgs>>) -> ::windows::core::Result<super::super::super::Foundation::EventRegistrationToken>;
    fn RemoveFrameArrived(&self, token: &super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn TryAcquireLatestFrame(&self) -> ::windows::core::Result<MediaFrameReference>;
    fn StartAsync(&self) -> ::windows::core::Result<super::super::super::Foundation::IAsyncOperation<MediaFrameReaderStartStatus>>;
    fn StopAsync(&self) -> ::windows::core::Result<super::super::super::Foundation::IAsyncAction>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IMediaFrameReader2Impl: Sized {
    fn SetAcquisitionMode(&self, value: MediaFrameReaderAcquisitionMode) -> ::windows::core::Result<()>;
    fn AcquisitionMode(&self) -> ::windows::core::Result<MediaFrameReaderAcquisitionMode>;
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
pub trait IMediaFrameReferenceImpl: Sized + IClosableImpl {
    fn SourceKind(&self) -> ::windows::core::Result<MediaFrameSourceKind>;
    fn Format(&self) -> ::windows::core::Result<MediaFrameFormat>;
    fn SystemRelativeTime(&self) -> ::windows::core::Result<super::super::super::Foundation::IReference<super::super::super::Foundation::TimeSpan>>;
    fn Duration(&self) -> ::windows::core::Result<super::super::super::Foundation::TimeSpan>;
    fn Properties(&self) -> ::windows::core::Result<super::super::super::Foundation::Collections::IMapView<::windows::core::GUID, ::windows::core::IInspectable>>;
    fn BufferMediaFrame(&self) -> ::windows::core::Result<BufferMediaFrame>;
    fn VideoMediaFrame(&self) -> ::windows::core::Result<VideoMediaFrame>;
    fn CoordinateSystem(&self) -> ::windows::core::Result<super::super::super::Perception::Spatial::SpatialCoordinateSystem>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IMediaFrameReference2Impl: Sized {
    fn AudioMediaFrame(&self) -> ::windows::core::Result<AudioMediaFrame>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IMediaFrameSourceImpl: Sized {
    fn Info(&self) -> ::windows::core::Result<MediaFrameSourceInfo>;
    fn Controller(&self) -> ::windows::core::Result<MediaFrameSourceController>;
    fn SupportedFormats(&self) -> ::windows::core::Result<super::super::super::Foundation::Collections::IVectorView<MediaFrameFormat>>;
    fn CurrentFormat(&self) -> ::windows::core::Result<MediaFrameFormat>;
    fn SetFormatAsync(&self, format: &::core::option::Option<MediaFrameFormat>) -> ::windows::core::Result<super::super::super::Foundation::IAsyncAction>;
    fn FormatChanged(&self, handler: &::core::option::Option<super::super::super::Foundation::TypedEventHandler<MediaFrameSource, ::windows::core::IInspectable>>) -> ::windows::core::Result<super::super::super::Foundation::EventRegistrationToken>;
    fn RemoveFormatChanged(&self, token: &super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn TryGetCameraIntrinsics(&self, format: &::core::option::Option<MediaFrameFormat>) -> ::windows::core::Result<super::super::Devices::Core::CameraIntrinsics>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IMediaFrameSourceControllerImpl: Sized {
    fn GetPropertyAsync(&self, propertyid: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::super::Foundation::IAsyncOperation<MediaFrameSourceGetPropertyResult>>;
    fn SetPropertyAsync(&self, propertyid: &::windows::core::HSTRING, propertyvalue: &::core::option::Option<::windows::core::IInspectable>) -> ::windows::core::Result<super::super::super::Foundation::IAsyncOperation<MediaFrameSourceSetPropertyStatus>>;
    fn VideoDeviceController(&self) -> ::windows::core::Result<super::super::Devices::VideoDeviceController>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IMediaFrameSourceController2Impl: Sized {
    fn GetPropertyByExtendedIdAsync(&self, extendedpropertyid: &[<u8 as ::windows::core::DefaultType>::DefaultType], maxpropertyvaluesize: &::core::option::Option<super::super::super::Foundation::IReference<u32>>) -> ::windows::core::Result<super::super::super::Foundation::IAsyncOperation<MediaFrameSourceGetPropertyResult>>;
    fn SetPropertyByExtendedIdAsync(&self, extendedpropertyid: &[<u8 as ::windows::core::DefaultType>::DefaultType], propertyvalue: &[<u8 as ::windows::core::DefaultType>::DefaultType]) -> ::windows::core::Result<super::super::super::Foundation::IAsyncOperation<MediaFrameSourceSetPropertyStatus>>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IMediaFrameSourceController3Impl: Sized {
    fn AudioDeviceController(&self) -> ::windows::core::Result<super::super::Devices::AudioDeviceController>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IMediaFrameSourceGetPropertyResultImpl: Sized {
    fn Status(&self) -> ::windows::core::Result<MediaFrameSourceGetPropertyStatus>;
    fn Value(&self) -> ::windows::core::Result<::windows::core::IInspectable>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IMediaFrameSourceGroupImpl: Sized {
    fn Id(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn DisplayName(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SourceInfos(&self) -> ::windows::core::Result<super::super::super::Foundation::Collections::IVectorView<MediaFrameSourceInfo>>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IMediaFrameSourceGroupStaticsImpl: Sized {
    fn FindAllAsync(&self) -> ::windows::core::Result<super::super::super::Foundation::IAsyncOperation<super::super::super::Foundation::Collections::IVectorView<MediaFrameSourceGroup>>>;
    fn FromIdAsync(&self, id: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::super::Foundation::IAsyncOperation<MediaFrameSourceGroup>>;
    fn GetDeviceSelector(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IMediaFrameSourceInfoImpl: Sized {
    fn Id(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn MediaStreamType(&self) -> ::windows::core::Result<super::MediaStreamType>;
    fn SourceKind(&self) -> ::windows::core::Result<MediaFrameSourceKind>;
    fn SourceGroup(&self) -> ::windows::core::Result<MediaFrameSourceGroup>;
    fn DeviceInformation(&self) -> ::windows::core::Result<super::super::super::Devices::Enumeration::DeviceInformation>;
    fn Properties(&self) -> ::windows::core::Result<super::super::super::Foundation::Collections::IMapView<::windows::core::GUID, ::windows::core::IInspectable>>;
    fn CoordinateSystem(&self) -> ::windows::core::Result<super::super::super::Perception::Spatial::SpatialCoordinateSystem>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IMediaFrameSourceInfo2Impl: Sized {
    fn ProfileId(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn VideoProfileMediaDescription(&self) -> ::windows::core::Result<super::super::super::Foundation::Collections::IVectorView<super::MediaCaptureVideoProfileMediaDescription>>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IMediaFrameSourceInfo3Impl: Sized {
    fn GetRelativePanel(&self, displayregion: &::core::option::Option<super::super::super::UI::WindowManagement::DisplayRegion>) -> ::windows::core::Result<super::super::super::Devices::Enumeration::Panel>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IMultiSourceMediaFrameArrivedEventArgsImpl: Sized {}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
pub trait IMultiSourceMediaFrameReaderImpl: Sized + IClosableImpl {
    fn FrameArrived(&self, handler: &::core::option::Option<super::super::super::Foundation::TypedEventHandler<MultiSourceMediaFrameReader, MultiSourceMediaFrameArrivedEventArgs>>) -> ::windows::core::Result<super::super::super::Foundation::EventRegistrationToken>;
    fn RemoveFrameArrived(&self, token: &super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn TryAcquireLatestFrame(&self) -> ::windows::core::Result<MultiSourceMediaFrameReference>;
    fn StartAsync(&self) -> ::windows::core::Result<super::super::super::Foundation::IAsyncOperation<MultiSourceMediaFrameReaderStartStatus>>;
    fn StopAsync(&self) -> ::windows::core::Result<super::super::super::Foundation::IAsyncAction>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IMultiSourceMediaFrameReader2Impl: Sized {
    fn SetAcquisitionMode(&self, value: MediaFrameReaderAcquisitionMode) -> ::windows::core::Result<()>;
    fn AcquisitionMode(&self) -> ::windows::core::Result<MediaFrameReaderAcquisitionMode>;
}
#[cfg(all(feature = "Foundation", feature = "implement_exclusive"))]
pub trait IMultiSourceMediaFrameReferenceImpl: Sized + IClosableImpl {
    fn TryGetFrameReferenceBySourceId(&self, sourceid: &::windows::core::HSTRING) -> ::windows::core::Result<MediaFrameReference>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IVideoMediaFrameImpl: Sized {
    fn FrameReference(&self) -> ::windows::core::Result<MediaFrameReference>;
    fn VideoFormat(&self) -> ::windows::core::Result<VideoMediaFrameFormat>;
    fn SoftwareBitmap(&self) -> ::windows::core::Result<super::super::super::Graphics::Imaging::SoftwareBitmap>;
    fn Direct3DSurface(&self) -> ::windows::core::Result<super::super::super::Graphics::DirectX::Direct3D11::IDirect3DSurface>;
    fn CameraIntrinsics(&self) -> ::windows::core::Result<super::super::Devices::Core::CameraIntrinsics>;
    fn InfraredMediaFrame(&self) -> ::windows::core::Result<InfraredMediaFrame>;
    fn DepthMediaFrame(&self) -> ::windows::core::Result<DepthMediaFrame>;
    fn GetVideoFrame(&self) -> ::windows::core::Result<super::super::VideoFrame>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IVideoMediaFrameFormatImpl: Sized {
    fn MediaFrameFormat(&self) -> ::windows::core::Result<MediaFrameFormat>;
    fn DepthFormat(&self) -> ::windows::core::Result<DepthMediaFrameFormat>;
    fn Width(&self) -> ::windows::core::Result<u32>;
    fn Height(&self) -> ::windows::core::Result<u32>;
}

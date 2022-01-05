#[cfg(all(feature = "deprecated", feature = "implement_exclusive"))]
pub trait IKnownPerceptionFrameKindStaticsImpl: Sized {
    fn Color(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn Depth(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn Infrared(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
}
#[cfg(all(feature = "deprecated", feature = "implement_exclusive"))]
pub trait IPerceptionControlGroupImpl: Sized {
    fn FrameProviderIds(&self) -> ::windows::core::Result<super::super::super::Foundation::Collections::IVectorView<::windows::core::HSTRING>>;
}
#[cfg(all(feature = "deprecated", feature = "implement_exclusive"))]
pub trait IPerceptionControlGroupFactoryImpl: Sized {
    fn Create(&self, ids: &::core::option::Option<super::super::super::Foundation::Collections::IIterable<::windows::core::HSTRING>>) -> ::windows::core::Result<PerceptionControlGroup>;
}
#[cfg(all(feature = "deprecated", feature = "implement_exclusive"))]
pub trait IPerceptionCorrelationImpl: Sized {
    fn TargetId(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn Position(&self) -> ::windows::core::Result<super::super::super::Foundation::Numerics::Vector3>;
    fn Orientation(&self) -> ::windows::core::Result<super::super::super::Foundation::Numerics::Quaternion>;
}
#[cfg(all(feature = "deprecated", feature = "implement_exclusive"))]
pub trait IPerceptionCorrelationFactoryImpl: Sized {
    fn Create(&self, targetid: &::windows::core::HSTRING, position: &super::super::super::Foundation::Numerics::Vector3, orientation: &super::super::super::Foundation::Numerics::Quaternion) -> ::windows::core::Result<PerceptionCorrelation>;
}
#[cfg(all(feature = "deprecated", feature = "implement_exclusive"))]
pub trait IPerceptionCorrelationGroupImpl: Sized {
    fn RelativeLocations(&self) -> ::windows::core::Result<super::super::super::Foundation::Collections::IVectorView<PerceptionCorrelation>>;
}
#[cfg(all(feature = "deprecated", feature = "implement_exclusive"))]
pub trait IPerceptionCorrelationGroupFactoryImpl: Sized {
    fn Create(&self, relativelocations: &::core::option::Option<super::super::super::Foundation::Collections::IIterable<PerceptionCorrelation>>) -> ::windows::core::Result<PerceptionCorrelationGroup>;
}
#[cfg(all(feature = "deprecated", feature = "implement_exclusive"))]
pub trait IPerceptionFaceAuthenticationGroupImpl: Sized {
    fn FrameProviderIds(&self) -> ::windows::core::Result<super::super::super::Foundation::Collections::IVectorView<::windows::core::HSTRING>>;
}
#[cfg(all(feature = "deprecated", feature = "implement_exclusive"))]
pub trait IPerceptionFaceAuthenticationGroupFactoryImpl: Sized {
    fn Create(&self, ids: &::core::option::Option<super::super::super::Foundation::Collections::IIterable<::windows::core::HSTRING>>, starthandler: &::core::option::Option<PerceptionStartFaceAuthenticationHandler>, stophandler: &::core::option::Option<PerceptionStopFaceAuthenticationHandler>) -> ::windows::core::Result<PerceptionFaceAuthenticationGroup>;
}
#[cfg(all(feature = "deprecated", feature = "implement_exclusive"))]
pub trait IPerceptionFrameImpl: Sized {
    fn RelativeTime(&self) -> ::windows::core::Result<super::super::super::Foundation::TimeSpan>;
    fn SetRelativeTime(&self, value: &super::super::super::Foundation::TimeSpan) -> ::windows::core::Result<()>;
    fn Properties(&self) -> ::windows::core::Result<super::super::super::Foundation::Collections::ValueSet>;
    fn FrameData(&self) -> ::windows::core::Result<super::super::super::Foundation::IMemoryBuffer>;
}
#[cfg(all(feature = "Foundation", feature = "deprecated"))]
pub trait IPerceptionFrameProviderImpl: Sized + IClosableImpl {
    fn FrameProviderInfo(&self) -> ::windows::core::Result<PerceptionFrameProviderInfo>;
    fn Available(&self) -> ::windows::core::Result<bool>;
    fn Properties(&self) -> ::windows::core::Result<super::super::super::Foundation::Collections::IPropertySet>;
    fn Start(&self) -> ::windows::core::Result<()>;
    fn Stop(&self) -> ::windows::core::Result<()>;
    fn SetProperty(&self, value: &::core::option::Option<PerceptionPropertyChangeRequest>) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "deprecated", feature = "implement_exclusive"))]
pub trait IPerceptionFrameProviderInfoImpl: Sized {
    fn Id(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetId(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn DisplayName(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetDisplayName(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn DeviceKind(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetDeviceKind(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn FrameKind(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetFrameKind(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn Hidden(&self) -> ::windows::core::Result<bool>;
    fn SetHidden(&self, value: bool) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Foundation", feature = "deprecated"))]
pub trait IPerceptionFrameProviderManagerImpl: Sized + IClosableImpl {
    fn GetFrameProvider(&self, frameproviderinfo: &::core::option::Option<PerceptionFrameProviderInfo>) -> ::windows::core::Result<IPerceptionFrameProvider>;
}
#[cfg(all(feature = "deprecated", feature = "implement_exclusive"))]
pub trait IPerceptionFrameProviderManagerServiceStaticsImpl: Sized {
    fn RegisterFrameProviderInfo(&self, manager: &::core::option::Option<IPerceptionFrameProviderManager>, frameproviderinfo: &::core::option::Option<PerceptionFrameProviderInfo>) -> ::windows::core::Result<()>;
    fn UnregisterFrameProviderInfo(&self, manager: &::core::option::Option<IPerceptionFrameProviderManager>, frameproviderinfo: &::core::option::Option<PerceptionFrameProviderInfo>) -> ::windows::core::Result<()>;
    fn RegisterFaceAuthenticationGroup(&self, manager: &::core::option::Option<IPerceptionFrameProviderManager>, faceauthenticationgroup: &::core::option::Option<PerceptionFaceAuthenticationGroup>) -> ::windows::core::Result<()>;
    fn UnregisterFaceAuthenticationGroup(&self, manager: &::core::option::Option<IPerceptionFrameProviderManager>, faceauthenticationgroup: &::core::option::Option<PerceptionFaceAuthenticationGroup>) -> ::windows::core::Result<()>;
    fn RegisterControlGroup(&self, manager: &::core::option::Option<IPerceptionFrameProviderManager>, controlgroup: &::core::option::Option<PerceptionControlGroup>) -> ::windows::core::Result<()>;
    fn UnregisterControlGroup(&self, manager: &::core::option::Option<IPerceptionFrameProviderManager>, controlgroup: &::core::option::Option<PerceptionControlGroup>) -> ::windows::core::Result<()>;
    fn RegisterCorrelationGroup(&self, manager: &::core::option::Option<IPerceptionFrameProviderManager>, correlationgroup: &::core::option::Option<PerceptionCorrelationGroup>) -> ::windows::core::Result<()>;
    fn UnregisterCorrelationGroup(&self, manager: &::core::option::Option<IPerceptionFrameProviderManager>, correlationgroup: &::core::option::Option<PerceptionCorrelationGroup>) -> ::windows::core::Result<()>;
    fn UpdateAvailabilityForProvider(&self, provider: &::core::option::Option<IPerceptionFrameProvider>, available: bool) -> ::windows::core::Result<()>;
    fn PublishFrameForProvider(&self, provider: &::core::option::Option<IPerceptionFrameProvider>, frame: &::core::option::Option<PerceptionFrame>) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "deprecated", feature = "implement_exclusive"))]
pub trait IPerceptionPropertyChangeRequestImpl: Sized {
    fn Name(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn Value(&self) -> ::windows::core::Result<::windows::core::IInspectable>;
    fn Status(&self) -> ::windows::core::Result<super::PerceptionFrameSourcePropertyChangeStatus>;
    fn SetStatus(&self, value: super::PerceptionFrameSourcePropertyChangeStatus) -> ::windows::core::Result<()>;
    fn GetDeferral(&self) -> ::windows::core::Result<super::super::super::Foundation::Deferral>;
}
#[cfg(all(feature = "Foundation", feature = "deprecated", feature = "implement_exclusive"))]
pub trait IPerceptionVideoFrameAllocatorImpl: Sized + IClosableImpl {
    fn AllocateFrame(&self) -> ::windows::core::Result<PerceptionFrame>;
    fn CopyFromVideoFrame(&self, frame: &::core::option::Option<super::super::super::Media::VideoFrame>) -> ::windows::core::Result<PerceptionFrame>;
}
#[cfg(all(feature = "deprecated", feature = "implement_exclusive"))]
pub trait IPerceptionVideoFrameAllocatorFactoryImpl: Sized {
    fn Create(&self, maxoutstandingframecountforwrite: u32, format: super::super::super::Graphics::Imaging::BitmapPixelFormat, resolution: &super::super::super::Foundation::Size, alpha: super::super::super::Graphics::Imaging::BitmapAlphaMode) -> ::windows::core::Result<PerceptionVideoFrameAllocator>;
}

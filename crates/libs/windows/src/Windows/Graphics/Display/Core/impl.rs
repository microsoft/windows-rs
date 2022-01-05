#[cfg(feature = "implement_exclusive")]
pub trait IHdmiDisplayInformationImpl: Sized {
    fn GetSupportedDisplayModes(&self) -> ::windows::core::Result<super::super::super::Foundation::Collections::IVectorView<HdmiDisplayMode>>;
    fn GetCurrentDisplayMode(&self) -> ::windows::core::Result<HdmiDisplayMode>;
    fn SetDefaultDisplayModeAsync(&self) -> ::windows::core::Result<super::super::super::Foundation::IAsyncAction>;
    fn RequestSetCurrentDisplayModeAsync(&self, mode: &::core::option::Option<HdmiDisplayMode>) -> ::windows::core::Result<super::super::super::Foundation::IAsyncOperation<bool>>;
    fn RequestSetCurrentDisplayModeWithHdrAsync(&self, mode: &::core::option::Option<HdmiDisplayMode>, hdroption: HdmiDisplayHdrOption) -> ::windows::core::Result<super::super::super::Foundation::IAsyncOperation<bool>>;
    fn RequestSetCurrentDisplayModeWithHdrAndMetadataAsync(&self, mode: &::core::option::Option<HdmiDisplayMode>, hdroption: HdmiDisplayHdrOption, hdrmetadata: &HdmiDisplayHdr2086Metadata) -> ::windows::core::Result<super::super::super::Foundation::IAsyncOperation<bool>>;
    fn DisplayModesChanged(&self, value: &::core::option::Option<super::super::super::Foundation::TypedEventHandler<HdmiDisplayInformation, ::windows::core::IInspectable>>) -> ::windows::core::Result<super::super::super::Foundation::EventRegistrationToken>;
    fn RemoveDisplayModesChanged(&self, token: &super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IHdmiDisplayInformationStaticsImpl: Sized {
    fn GetForCurrentView(&self) -> ::windows::core::Result<HdmiDisplayInformation>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IHdmiDisplayModeImpl: Sized {
    fn ResolutionWidthInRawPixels(&self) -> ::windows::core::Result<u32>;
    fn ResolutionHeightInRawPixels(&self) -> ::windows::core::Result<u32>;
    fn RefreshRate(&self) -> ::windows::core::Result<f64>;
    fn StereoEnabled(&self) -> ::windows::core::Result<bool>;
    fn BitsPerPixel(&self) -> ::windows::core::Result<u16>;
    fn IsEqual(&self, mode: &::core::option::Option<HdmiDisplayMode>) -> ::windows::core::Result<bool>;
    fn ColorSpace(&self) -> ::windows::core::Result<HdmiDisplayColorSpace>;
    fn PixelEncoding(&self) -> ::windows::core::Result<HdmiDisplayPixelEncoding>;
    fn IsSdrLuminanceSupported(&self) -> ::windows::core::Result<bool>;
    fn IsSmpte2084Supported(&self) -> ::windows::core::Result<bool>;
    fn Is2086MetadataSupported(&self) -> ::windows::core::Result<bool>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IHdmiDisplayMode2Impl: Sized {
    fn IsDolbyVisionLowLatencySupported(&self) -> ::windows::core::Result<bool>;
}

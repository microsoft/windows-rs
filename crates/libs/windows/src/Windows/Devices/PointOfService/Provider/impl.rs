#[cfg(feature = "implement_exclusive")]
pub trait IBarcodeScannerDisableScannerRequestImpl: Sized {
    fn ReportCompletedAsync(&self) -> ::windows::core::Result<super::super::super::Foundation::IAsyncAction>;
    fn ReportFailedAsync(&self) -> ::windows::core::Result<super::super::super::Foundation::IAsyncAction>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IBarcodeScannerDisableScannerRequest2Impl: Sized {
    fn ReportFailedWithFailedReasonAsync(&self, reason: i32) -> ::windows::core::Result<super::super::super::Foundation::IAsyncAction>;
    fn ReportFailedWithFailedReasonAndDescriptionAsync(&self, reason: i32, failedreasondescription: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::super::Foundation::IAsyncAction>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IBarcodeScannerDisableScannerRequestEventArgsImpl: Sized {
    fn Request(&self) -> ::windows::core::Result<BarcodeScannerDisableScannerRequest>;
    fn GetDeferral(&self) -> ::windows::core::Result<super::super::super::Foundation::Deferral>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IBarcodeScannerEnableScannerRequestImpl: Sized {
    fn ReportCompletedAsync(&self) -> ::windows::core::Result<super::super::super::Foundation::IAsyncAction>;
    fn ReportFailedAsync(&self) -> ::windows::core::Result<super::super::super::Foundation::IAsyncAction>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IBarcodeScannerEnableScannerRequest2Impl: Sized {
    fn ReportFailedWithFailedReasonAsync(&self, reason: i32) -> ::windows::core::Result<super::super::super::Foundation::IAsyncAction>;
    fn ReportFailedWithFailedReasonAndDescriptionAsync(&self, reason: i32, failedreasondescription: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::super::Foundation::IAsyncAction>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IBarcodeScannerEnableScannerRequestEventArgsImpl: Sized {
    fn Request(&self) -> ::windows::core::Result<BarcodeScannerEnableScannerRequest>;
    fn GetDeferral(&self) -> ::windows::core::Result<super::super::super::Foundation::Deferral>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IBarcodeScannerFrameReaderImpl: Sized {
    fn StartAsync(&self) -> ::windows::core::Result<super::super::super::Foundation::IAsyncOperation<bool>>;
    fn StopAsync(&self) -> ::windows::core::Result<super::super::super::Foundation::IAsyncAction>;
    fn TryAcquireLatestFrameAsync(&self) -> ::windows::core::Result<super::super::super::Foundation::IAsyncOperation<BarcodeScannerVideoFrame>>;
    fn Connection(&self) -> ::windows::core::Result<BarcodeScannerProviderConnection>;
    fn FrameArrived(&self, handler: &::core::option::Option<super::super::super::Foundation::TypedEventHandler<BarcodeScannerFrameReader, BarcodeScannerFrameReaderFrameArrivedEventArgs>>) -> ::windows::core::Result<super::super::super::Foundation::EventRegistrationToken>;
    fn RemoveFrameArrived(&self, token: &super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IBarcodeScannerFrameReaderFrameArrivedEventArgsImpl: Sized {
    fn GetDeferral(&self) -> ::windows::core::Result<super::super::super::Foundation::Deferral>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IBarcodeScannerGetSymbologyAttributesRequestImpl: Sized {
    fn Symbology(&self) -> ::windows::core::Result<u32>;
    fn ReportCompletedAsync(&self, attributes: &::core::option::Option<super::BarcodeSymbologyAttributes>) -> ::windows::core::Result<super::super::super::Foundation::IAsyncAction>;
    fn ReportFailedAsync(&self) -> ::windows::core::Result<super::super::super::Foundation::IAsyncAction>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IBarcodeScannerGetSymbologyAttributesRequest2Impl: Sized {
    fn ReportFailedWithFailedReasonAsync(&self, reason: i32) -> ::windows::core::Result<super::super::super::Foundation::IAsyncAction>;
    fn ReportFailedWithFailedReasonAndDescriptionAsync(&self, reason: i32, failedreasondescription: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::super::Foundation::IAsyncAction>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IBarcodeScannerGetSymbologyAttributesRequestEventArgsImpl: Sized {
    fn Request(&self) -> ::windows::core::Result<BarcodeScannerGetSymbologyAttributesRequest>;
    fn GetDeferral(&self) -> ::windows::core::Result<super::super::super::Foundation::Deferral>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IBarcodeScannerHideVideoPreviewRequestImpl: Sized {
    fn ReportCompletedAsync(&self) -> ::windows::core::Result<super::super::super::Foundation::IAsyncAction>;
    fn ReportFailedAsync(&self) -> ::windows::core::Result<super::super::super::Foundation::IAsyncAction>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IBarcodeScannerHideVideoPreviewRequest2Impl: Sized {
    fn ReportFailedWithFailedReasonAsync(&self, reason: i32) -> ::windows::core::Result<super::super::super::Foundation::IAsyncAction>;
    fn ReportFailedWithFailedReasonAndDescriptionAsync(&self, reason: i32, failedreasondescription: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::super::Foundation::IAsyncAction>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IBarcodeScannerHideVideoPreviewRequestEventArgsImpl: Sized {
    fn Request(&self) -> ::windows::core::Result<BarcodeScannerHideVideoPreviewRequest>;
    fn GetDeferral(&self) -> ::windows::core::Result<super::super::super::Foundation::Deferral>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IBarcodeScannerProviderConnectionImpl: Sized {
    fn Id(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn VideoDeviceId(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SupportedSymbologies(&self) -> ::windows::core::Result<super::super::super::Foundation::Collections::IVector<u32>>;
    fn CompanyName(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetCompanyName(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn Name(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetName(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn Version(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetVersion(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn Start(&self) -> ::windows::core::Result<()>;
    fn ReportScannedDataAsync(&self, report: &::core::option::Option<super::BarcodeScannerReport>) -> ::windows::core::Result<super::super::super::Foundation::IAsyncAction>;
    fn ReportTriggerStateAsync(&self, state: BarcodeScannerTriggerState) -> ::windows::core::Result<super::super::super::Foundation::IAsyncAction>;
    fn ReportErrorAsync(&self, errordata: &::core::option::Option<super::UnifiedPosErrorData>) -> ::windows::core::Result<super::super::super::Foundation::IAsyncAction>;
    fn ReportErrorAsyncWithScanReport(&self, errordata: &::core::option::Option<super::UnifiedPosErrorData>, isretriable: bool, scanreport: &::core::option::Option<super::BarcodeScannerReport>) -> ::windows::core::Result<super::super::super::Foundation::IAsyncAction>;
    fn EnableScannerRequested(&self, handler: &::core::option::Option<super::super::super::Foundation::TypedEventHandler<BarcodeScannerProviderConnection, BarcodeScannerEnableScannerRequestEventArgs>>) -> ::windows::core::Result<super::super::super::Foundation::EventRegistrationToken>;
    fn RemoveEnableScannerRequested(&self, token: &super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn DisableScannerRequested(&self, handler: &::core::option::Option<super::super::super::Foundation::TypedEventHandler<BarcodeScannerProviderConnection, BarcodeScannerDisableScannerRequestEventArgs>>) -> ::windows::core::Result<super::super::super::Foundation::EventRegistrationToken>;
    fn RemoveDisableScannerRequested(&self, token: &super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn SetActiveSymbologiesRequested(&self, handler: &::core::option::Option<super::super::super::Foundation::TypedEventHandler<BarcodeScannerProviderConnection, BarcodeScannerSetActiveSymbologiesRequestEventArgs>>) -> ::windows::core::Result<super::super::super::Foundation::EventRegistrationToken>;
    fn RemoveSetActiveSymbologiesRequested(&self, token: &super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn StartSoftwareTriggerRequested(&self, handler: &::core::option::Option<super::super::super::Foundation::TypedEventHandler<BarcodeScannerProviderConnection, BarcodeScannerStartSoftwareTriggerRequestEventArgs>>) -> ::windows::core::Result<super::super::super::Foundation::EventRegistrationToken>;
    fn RemoveStartSoftwareTriggerRequested(&self, token: &super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn StopSoftwareTriggerRequested(&self, handler: &::core::option::Option<super::super::super::Foundation::TypedEventHandler<BarcodeScannerProviderConnection, BarcodeScannerStopSoftwareTriggerRequestEventArgs>>) -> ::windows::core::Result<super::super::super::Foundation::EventRegistrationToken>;
    fn RemoveStopSoftwareTriggerRequested(&self, token: &super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn GetBarcodeSymbologyAttributesRequested(&self, handler: &::core::option::Option<super::super::super::Foundation::TypedEventHandler<BarcodeScannerProviderConnection, BarcodeScannerGetSymbologyAttributesRequestEventArgs>>) -> ::windows::core::Result<super::super::super::Foundation::EventRegistrationToken>;
    fn RemoveGetBarcodeSymbologyAttributesRequested(&self, token: &super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn SetBarcodeSymbologyAttributesRequested(&self, handler: &::core::option::Option<super::super::super::Foundation::TypedEventHandler<BarcodeScannerProviderConnection, BarcodeScannerSetSymbologyAttributesRequestEventArgs>>) -> ::windows::core::Result<super::super::super::Foundation::EventRegistrationToken>;
    fn RemoveSetBarcodeSymbologyAttributesRequested(&self, token: &super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn HideVideoPreviewRequested(&self, handler: &::core::option::Option<super::super::super::Foundation::TypedEventHandler<BarcodeScannerProviderConnection, BarcodeScannerHideVideoPreviewRequestEventArgs>>) -> ::windows::core::Result<super::super::super::Foundation::EventRegistrationToken>;
    fn RemoveHideVideoPreviewRequested(&self, token: &super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IBarcodeScannerProviderConnection2Impl: Sized {
    fn CreateFrameReaderAsync(&self) -> ::windows::core::Result<super::super::super::Foundation::IAsyncOperation<BarcodeScannerFrameReader>>;
    fn CreateFrameReaderWithFormatAsync(&self, preferredformat: super::super::super::Graphics::Imaging::BitmapPixelFormat) -> ::windows::core::Result<super::super::super::Foundation::IAsyncOperation<BarcodeScannerFrameReader>>;
    fn CreateFrameReaderWithFormatAndSizeAsync(&self, preferredformat: super::super::super::Graphics::Imaging::BitmapPixelFormat, preferredsize: &super::super::super::Graphics::Imaging::BitmapSize) -> ::windows::core::Result<super::super::super::Foundation::IAsyncOperation<BarcodeScannerFrameReader>>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IBarcodeScannerProviderTriggerDetailsImpl: Sized {
    fn Connection(&self) -> ::windows::core::Result<BarcodeScannerProviderConnection>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IBarcodeScannerSetActiveSymbologiesRequestImpl: Sized {
    fn Symbologies(&self) -> ::windows::core::Result<super::super::super::Foundation::Collections::IVectorView<u32>>;
    fn ReportCompletedAsync(&self) -> ::windows::core::Result<super::super::super::Foundation::IAsyncAction>;
    fn ReportFailedAsync(&self) -> ::windows::core::Result<super::super::super::Foundation::IAsyncAction>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IBarcodeScannerSetActiveSymbologiesRequest2Impl: Sized {
    fn ReportFailedWithFailedReasonAsync(&self, reason: i32) -> ::windows::core::Result<super::super::super::Foundation::IAsyncAction>;
    fn ReportFailedWithFailedReasonAndDescriptionAsync(&self, reason: i32, failedreasondescription: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::super::Foundation::IAsyncAction>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IBarcodeScannerSetActiveSymbologiesRequestEventArgsImpl: Sized {
    fn Request(&self) -> ::windows::core::Result<BarcodeScannerSetActiveSymbologiesRequest>;
    fn GetDeferral(&self) -> ::windows::core::Result<super::super::super::Foundation::Deferral>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IBarcodeScannerSetSymbologyAttributesRequestImpl: Sized {
    fn Symbology(&self) -> ::windows::core::Result<u32>;
    fn Attributes(&self) -> ::windows::core::Result<super::BarcodeSymbologyAttributes>;
    fn ReportCompletedAsync(&self) -> ::windows::core::Result<super::super::super::Foundation::IAsyncAction>;
    fn ReportFailedAsync(&self) -> ::windows::core::Result<super::super::super::Foundation::IAsyncAction>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IBarcodeScannerSetSymbologyAttributesRequest2Impl: Sized {
    fn ReportFailedWithFailedReasonAsync(&self, reason: i32) -> ::windows::core::Result<super::super::super::Foundation::IAsyncAction>;
    fn ReportFailedWithFailedReasonAndDescriptionAsync(&self, reason: i32, failedreasondescription: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::super::Foundation::IAsyncAction>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IBarcodeScannerSetSymbologyAttributesRequestEventArgsImpl: Sized {
    fn Request(&self) -> ::windows::core::Result<BarcodeScannerSetSymbologyAttributesRequest>;
    fn GetDeferral(&self) -> ::windows::core::Result<super::super::super::Foundation::Deferral>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IBarcodeScannerStartSoftwareTriggerRequestImpl: Sized {
    fn ReportCompletedAsync(&self) -> ::windows::core::Result<super::super::super::Foundation::IAsyncAction>;
    fn ReportFailedAsync(&self) -> ::windows::core::Result<super::super::super::Foundation::IAsyncAction>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IBarcodeScannerStartSoftwareTriggerRequest2Impl: Sized {
    fn ReportFailedWithFailedReasonAsync(&self, reason: i32) -> ::windows::core::Result<super::super::super::Foundation::IAsyncAction>;
    fn ReportFailedWithFailedReasonAndDescriptionAsync(&self, reason: i32, failedreasondescription: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::super::Foundation::IAsyncAction>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IBarcodeScannerStartSoftwareTriggerRequestEventArgsImpl: Sized {
    fn Request(&self) -> ::windows::core::Result<BarcodeScannerStartSoftwareTriggerRequest>;
    fn GetDeferral(&self) -> ::windows::core::Result<super::super::super::Foundation::Deferral>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IBarcodeScannerStopSoftwareTriggerRequestImpl: Sized {
    fn ReportCompletedAsync(&self) -> ::windows::core::Result<super::super::super::Foundation::IAsyncAction>;
    fn ReportFailedAsync(&self) -> ::windows::core::Result<super::super::super::Foundation::IAsyncAction>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IBarcodeScannerStopSoftwareTriggerRequest2Impl: Sized {
    fn ReportFailedWithFailedReasonAsync(&self, reason: i32) -> ::windows::core::Result<super::super::super::Foundation::IAsyncAction>;
    fn ReportFailedWithFailedReasonAndDescriptionAsync(&self, reason: i32, failedreasondescription: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::super::Foundation::IAsyncAction>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IBarcodeScannerStopSoftwareTriggerRequestEventArgsImpl: Sized {
    fn Request(&self) -> ::windows::core::Result<BarcodeScannerStopSoftwareTriggerRequest>;
    fn GetDeferral(&self) -> ::windows::core::Result<super::super::super::Foundation::Deferral>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IBarcodeScannerVideoFrameImpl: Sized {
    fn Format(&self) -> ::windows::core::Result<super::super::super::Graphics::Imaging::BitmapPixelFormat>;
    fn Width(&self) -> ::windows::core::Result<u32>;
    fn Height(&self) -> ::windows::core::Result<u32>;
    fn PixelData(&self) -> ::windows::core::Result<super::super::super::Storage::Streams::IBuffer>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IBarcodeSymbologyAttributesBuilderImpl: Sized {
    fn IsCheckDigitValidationSupported(&self) -> ::windows::core::Result<bool>;
    fn SetIsCheckDigitValidationSupported(&self, value: bool) -> ::windows::core::Result<()>;
    fn IsCheckDigitTransmissionSupported(&self) -> ::windows::core::Result<bool>;
    fn SetIsCheckDigitTransmissionSupported(&self, value: bool) -> ::windows::core::Result<()>;
    fn IsDecodeLengthSupported(&self) -> ::windows::core::Result<bool>;
    fn SetIsDecodeLengthSupported(&self, value: bool) -> ::windows::core::Result<()>;
    fn CreateAttributes(&self) -> ::windows::core::Result<super::BarcodeSymbologyAttributes>;
}

#[cfg(all(feature = "Media_Core", feature = "implement_exclusive"))]
pub trait IAdaptiveMediaSourceImpl: Sized + IMediaSourceImpl {
    fn IsLive(&self) -> ::windows::core::Result<bool>;
    fn DesiredLiveOffset(&self) -> ::windows::core::Result<super::super::super::Foundation::TimeSpan>;
    fn SetDesiredLiveOffset(&self, value: &super::super::super::Foundation::TimeSpan) -> ::windows::core::Result<()>;
    fn InitialBitrate(&self) -> ::windows::core::Result<u32>;
    fn SetInitialBitrate(&self, value: u32) -> ::windows::core::Result<()>;
    fn CurrentDownloadBitrate(&self) -> ::windows::core::Result<u32>;
    fn CurrentPlaybackBitrate(&self) -> ::windows::core::Result<u32>;
    fn AvailableBitrates(&self) -> ::windows::core::Result<super::super::super::Foundation::Collections::IVectorView<u32>>;
    fn DesiredMinBitrate(&self) -> ::windows::core::Result<super::super::super::Foundation::IReference<u32>>;
    fn SetDesiredMinBitrate(&self, value: &::core::option::Option<super::super::super::Foundation::IReference<u32>>) -> ::windows::core::Result<()>;
    fn DesiredMaxBitrate(&self) -> ::windows::core::Result<super::super::super::Foundation::IReference<u32>>;
    fn SetDesiredMaxBitrate(&self, value: &::core::option::Option<super::super::super::Foundation::IReference<u32>>) -> ::windows::core::Result<()>;
    fn AudioOnlyPlayback(&self) -> ::windows::core::Result<bool>;
    fn InboundBitsPerSecond(&self) -> ::windows::core::Result<u64>;
    fn InboundBitsPerSecondWindow(&self) -> ::windows::core::Result<super::super::super::Foundation::TimeSpan>;
    fn SetInboundBitsPerSecondWindow(&self, value: &super::super::super::Foundation::TimeSpan) -> ::windows::core::Result<()>;
    fn DownloadBitrateChanged(&self, handler: &::core::option::Option<super::super::super::Foundation::TypedEventHandler<AdaptiveMediaSource, AdaptiveMediaSourceDownloadBitrateChangedEventArgs>>) -> ::windows::core::Result<super::super::super::Foundation::EventRegistrationToken>;
    fn RemoveDownloadBitrateChanged(&self, token: &super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn PlaybackBitrateChanged(&self, handler: &::core::option::Option<super::super::super::Foundation::TypedEventHandler<AdaptiveMediaSource, AdaptiveMediaSourcePlaybackBitrateChangedEventArgs>>) -> ::windows::core::Result<super::super::super::Foundation::EventRegistrationToken>;
    fn RemovePlaybackBitrateChanged(&self, token: &super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn DownloadRequested(&self, handler: &::core::option::Option<super::super::super::Foundation::TypedEventHandler<AdaptiveMediaSource, AdaptiveMediaSourceDownloadRequestedEventArgs>>) -> ::windows::core::Result<super::super::super::Foundation::EventRegistrationToken>;
    fn RemoveDownloadRequested(&self, token: &super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn DownloadCompleted(&self, handler: &::core::option::Option<super::super::super::Foundation::TypedEventHandler<AdaptiveMediaSource, AdaptiveMediaSourceDownloadCompletedEventArgs>>) -> ::windows::core::Result<super::super::super::Foundation::EventRegistrationToken>;
    fn RemoveDownloadCompleted(&self, token: &super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn DownloadFailed(&self, handler: &::core::option::Option<super::super::super::Foundation::TypedEventHandler<AdaptiveMediaSource, AdaptiveMediaSourceDownloadFailedEventArgs>>) -> ::windows::core::Result<super::super::super::Foundation::EventRegistrationToken>;
    fn RemoveDownloadFailed(&self, token: &super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IAdaptiveMediaSource2Impl: Sized {
    fn AdvancedSettings(&self) -> ::windows::core::Result<AdaptiveMediaSourceAdvancedSettings>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IAdaptiveMediaSource3Impl: Sized {
    fn MinLiveOffset(&self) -> ::windows::core::Result<super::super::super::Foundation::IReference<super::super::super::Foundation::TimeSpan>>;
    fn MaxSeekableWindowSize(&self) -> ::windows::core::Result<super::super::super::Foundation::IReference<super::super::super::Foundation::TimeSpan>>;
    fn DesiredSeekableWindowSize(&self) -> ::windows::core::Result<super::super::super::Foundation::IReference<super::super::super::Foundation::TimeSpan>>;
    fn SetDesiredSeekableWindowSize(&self, value: &::core::option::Option<super::super::super::Foundation::IReference<super::super::super::Foundation::TimeSpan>>) -> ::windows::core::Result<()>;
    fn Diagnostics(&self) -> ::windows::core::Result<AdaptiveMediaSourceDiagnostics>;
    fn GetCorrelatedTimes(&self) -> ::windows::core::Result<AdaptiveMediaSourceCorrelatedTimes>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IAdaptiveMediaSourceAdvancedSettingsImpl: Sized {
    fn AllSegmentsIndependent(&self) -> ::windows::core::Result<bool>;
    fn SetAllSegmentsIndependent(&self, value: bool) -> ::windows::core::Result<()>;
    fn DesiredBitrateHeadroomRatio(&self) -> ::windows::core::Result<super::super::super::Foundation::IReference<f64>>;
    fn SetDesiredBitrateHeadroomRatio(&self, value: &::core::option::Option<super::super::super::Foundation::IReference<f64>>) -> ::windows::core::Result<()>;
    fn BitrateDowngradeTriggerRatio(&self) -> ::windows::core::Result<super::super::super::Foundation::IReference<f64>>;
    fn SetBitrateDowngradeTriggerRatio(&self, value: &::core::option::Option<super::super::super::Foundation::IReference<f64>>) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IAdaptiveMediaSourceCorrelatedTimesImpl: Sized {
    fn Position(&self) -> ::windows::core::Result<super::super::super::Foundation::IReference<super::super::super::Foundation::TimeSpan>>;
    fn PresentationTimeStamp(&self) -> ::windows::core::Result<super::super::super::Foundation::IReference<super::super::super::Foundation::TimeSpan>>;
    fn ProgramDateTime(&self) -> ::windows::core::Result<super::super::super::Foundation::IReference<super::super::super::Foundation::DateTime>>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IAdaptiveMediaSourceCreationResultImpl: Sized {
    fn Status(&self) -> ::windows::core::Result<AdaptiveMediaSourceCreationStatus>;
    fn MediaSource(&self) -> ::windows::core::Result<AdaptiveMediaSource>;
    fn HttpResponseMessage(&self) -> ::windows::core::Result<super::super::super::Web::Http::HttpResponseMessage>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IAdaptiveMediaSourceCreationResult2Impl: Sized {
    fn ExtendedError(&self) -> ::windows::core::Result<::windows::core::HRESULT>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IAdaptiveMediaSourceDiagnosticAvailableEventArgsImpl: Sized {
    fn DiagnosticType(&self) -> ::windows::core::Result<AdaptiveMediaSourceDiagnosticType>;
    fn RequestId(&self) -> ::windows::core::Result<super::super::super::Foundation::IReference<i32>>;
    fn Position(&self) -> ::windows::core::Result<super::super::super::Foundation::IReference<super::super::super::Foundation::TimeSpan>>;
    fn SegmentId(&self) -> ::windows::core::Result<super::super::super::Foundation::IReference<u64>>;
    fn ResourceType(&self) -> ::windows::core::Result<super::super::super::Foundation::IReference<AdaptiveMediaSourceResourceType>>;
    fn ResourceUri(&self) -> ::windows::core::Result<super::super::super::Foundation::Uri>;
    fn ResourceByteRangeOffset(&self) -> ::windows::core::Result<super::super::super::Foundation::IReference<u64>>;
    fn ResourceByteRangeLength(&self) -> ::windows::core::Result<super::super::super::Foundation::IReference<u64>>;
    fn Bitrate(&self) -> ::windows::core::Result<super::super::super::Foundation::IReference<u32>>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IAdaptiveMediaSourceDiagnosticAvailableEventArgs2Impl: Sized {
    fn ExtendedError(&self) -> ::windows::core::Result<::windows::core::HRESULT>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IAdaptiveMediaSourceDiagnosticAvailableEventArgs3Impl: Sized {
    fn ResourceDuration(&self) -> ::windows::core::Result<super::super::super::Foundation::IReference<super::super::super::Foundation::TimeSpan>>;
    fn ResourceContentType(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IAdaptiveMediaSourceDiagnosticsImpl: Sized {
    fn DiagnosticAvailable(&self, handler: &::core::option::Option<super::super::super::Foundation::TypedEventHandler<AdaptiveMediaSourceDiagnostics, AdaptiveMediaSourceDiagnosticAvailableEventArgs>>) -> ::windows::core::Result<super::super::super::Foundation::EventRegistrationToken>;
    fn RemoveDiagnosticAvailable(&self, token: &super::super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IAdaptiveMediaSourceDownloadBitrateChangedEventArgsImpl: Sized {
    fn OldValue(&self) -> ::windows::core::Result<u32>;
    fn NewValue(&self) -> ::windows::core::Result<u32>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IAdaptiveMediaSourceDownloadBitrateChangedEventArgs2Impl: Sized {
    fn Reason(&self) -> ::windows::core::Result<AdaptiveMediaSourceDownloadBitrateChangedReason>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IAdaptiveMediaSourceDownloadCompletedEventArgsImpl: Sized {
    fn ResourceType(&self) -> ::windows::core::Result<AdaptiveMediaSourceResourceType>;
    fn ResourceUri(&self) -> ::windows::core::Result<super::super::super::Foundation::Uri>;
    fn ResourceByteRangeOffset(&self) -> ::windows::core::Result<super::super::super::Foundation::IReference<u64>>;
    fn ResourceByteRangeLength(&self) -> ::windows::core::Result<super::super::super::Foundation::IReference<u64>>;
    fn HttpResponseMessage(&self) -> ::windows::core::Result<super::super::super::Web::Http::HttpResponseMessage>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IAdaptiveMediaSourceDownloadCompletedEventArgs2Impl: Sized {
    fn RequestId(&self) -> ::windows::core::Result<i32>;
    fn Statistics(&self) -> ::windows::core::Result<AdaptiveMediaSourceDownloadStatistics>;
    fn Position(&self) -> ::windows::core::Result<super::super::super::Foundation::IReference<super::super::super::Foundation::TimeSpan>>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IAdaptiveMediaSourceDownloadCompletedEventArgs3Impl: Sized {
    fn ResourceDuration(&self) -> ::windows::core::Result<super::super::super::Foundation::IReference<super::super::super::Foundation::TimeSpan>>;
    fn ResourceContentType(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IAdaptiveMediaSourceDownloadFailedEventArgsImpl: Sized {
    fn ResourceType(&self) -> ::windows::core::Result<AdaptiveMediaSourceResourceType>;
    fn ResourceUri(&self) -> ::windows::core::Result<super::super::super::Foundation::Uri>;
    fn ResourceByteRangeOffset(&self) -> ::windows::core::Result<super::super::super::Foundation::IReference<u64>>;
    fn ResourceByteRangeLength(&self) -> ::windows::core::Result<super::super::super::Foundation::IReference<u64>>;
    fn HttpResponseMessage(&self) -> ::windows::core::Result<super::super::super::Web::Http::HttpResponseMessage>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IAdaptiveMediaSourceDownloadFailedEventArgs2Impl: Sized {
    fn RequestId(&self) -> ::windows::core::Result<i32>;
    fn ExtendedError(&self) -> ::windows::core::Result<::windows::core::HRESULT>;
    fn Statistics(&self) -> ::windows::core::Result<AdaptiveMediaSourceDownloadStatistics>;
    fn Position(&self) -> ::windows::core::Result<super::super::super::Foundation::IReference<super::super::super::Foundation::TimeSpan>>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IAdaptiveMediaSourceDownloadFailedEventArgs3Impl: Sized {
    fn ResourceDuration(&self) -> ::windows::core::Result<super::super::super::Foundation::IReference<super::super::super::Foundation::TimeSpan>>;
    fn ResourceContentType(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IAdaptiveMediaSourceDownloadRequestedDeferralImpl: Sized {
    fn Complete(&self) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IAdaptiveMediaSourceDownloadRequestedEventArgsImpl: Sized {
    fn ResourceType(&self) -> ::windows::core::Result<AdaptiveMediaSourceResourceType>;
    fn ResourceUri(&self) -> ::windows::core::Result<super::super::super::Foundation::Uri>;
    fn ResourceByteRangeOffset(&self) -> ::windows::core::Result<super::super::super::Foundation::IReference<u64>>;
    fn ResourceByteRangeLength(&self) -> ::windows::core::Result<super::super::super::Foundation::IReference<u64>>;
    fn Result(&self) -> ::windows::core::Result<AdaptiveMediaSourceDownloadResult>;
    fn GetDeferral(&self) -> ::windows::core::Result<AdaptiveMediaSourceDownloadRequestedDeferral>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IAdaptiveMediaSourceDownloadRequestedEventArgs2Impl: Sized {
    fn RequestId(&self) -> ::windows::core::Result<i32>;
    fn Position(&self) -> ::windows::core::Result<super::super::super::Foundation::IReference<super::super::super::Foundation::TimeSpan>>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IAdaptiveMediaSourceDownloadRequestedEventArgs3Impl: Sized {
    fn ResourceDuration(&self) -> ::windows::core::Result<super::super::super::Foundation::IReference<super::super::super::Foundation::TimeSpan>>;
    fn ResourceContentType(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IAdaptiveMediaSourceDownloadResultImpl: Sized {
    fn ResourceUri(&self) -> ::windows::core::Result<super::super::super::Foundation::Uri>;
    fn SetResourceUri(&self, value: &::core::option::Option<super::super::super::Foundation::Uri>) -> ::windows::core::Result<()>;
    fn InputStream(&self) -> ::windows::core::Result<super::super::super::Storage::Streams::IInputStream>;
    fn SetInputStream(&self, value: &::core::option::Option<super::super::super::Storage::Streams::IInputStream>) -> ::windows::core::Result<()>;
    fn Buffer(&self) -> ::windows::core::Result<super::super::super::Storage::Streams::IBuffer>;
    fn SetBuffer(&self, value: &::core::option::Option<super::super::super::Storage::Streams::IBuffer>) -> ::windows::core::Result<()>;
    fn ContentType(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetContentType(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn ExtendedStatus(&self) -> ::windows::core::Result<u32>;
    fn SetExtendedStatus(&self, value: u32) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IAdaptiveMediaSourceDownloadResult2Impl: Sized {
    fn ResourceByteRangeOffset(&self) -> ::windows::core::Result<super::super::super::Foundation::IReference<u64>>;
    fn SetResourceByteRangeOffset(&self, value: &::core::option::Option<super::super::super::Foundation::IReference<u64>>) -> ::windows::core::Result<()>;
    fn ResourceByteRangeLength(&self) -> ::windows::core::Result<super::super::super::Foundation::IReference<u64>>;
    fn SetResourceByteRangeLength(&self, value: &::core::option::Option<super::super::super::Foundation::IReference<u64>>) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IAdaptiveMediaSourceDownloadStatisticsImpl: Sized {
    fn ContentBytesReceivedCount(&self) -> ::windows::core::Result<u64>;
    fn TimeToHeadersReceived(&self) -> ::windows::core::Result<super::super::super::Foundation::IReference<super::super::super::Foundation::TimeSpan>>;
    fn TimeToFirstByteReceived(&self) -> ::windows::core::Result<super::super::super::Foundation::IReference<super::super::super::Foundation::TimeSpan>>;
    fn TimeToLastByteReceived(&self) -> ::windows::core::Result<super::super::super::Foundation::IReference<super::super::super::Foundation::TimeSpan>>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IAdaptiveMediaSourcePlaybackBitrateChangedEventArgsImpl: Sized {
    fn OldValue(&self) -> ::windows::core::Result<u32>;
    fn NewValue(&self) -> ::windows::core::Result<u32>;
    fn AudioOnly(&self) -> ::windows::core::Result<bool>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IAdaptiveMediaSourceStaticsImpl: Sized {
    fn IsContentTypeSupported(&self, contenttype: &::windows::core::HSTRING) -> ::windows::core::Result<bool>;
    fn CreateFromUriAsync(&self, uri: &::core::option::Option<super::super::super::Foundation::Uri>) -> ::windows::core::Result<super::super::super::Foundation::IAsyncOperation<AdaptiveMediaSourceCreationResult>>;
    fn CreateFromUriWithDownloaderAsync(&self, uri: &::core::option::Option<super::super::super::Foundation::Uri>, httpclient: &::core::option::Option<super::super::super::Web::Http::HttpClient>) -> ::windows::core::Result<super::super::super::Foundation::IAsyncOperation<AdaptiveMediaSourceCreationResult>>;
    fn CreateFromStreamAsync(&self, stream: &::core::option::Option<super::super::super::Storage::Streams::IInputStream>, uri: &::core::option::Option<super::super::super::Foundation::Uri>, contenttype: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::super::Foundation::IAsyncOperation<AdaptiveMediaSourceCreationResult>>;
    fn CreateFromStreamWithDownloaderAsync(&self, stream: &::core::option::Option<super::super::super::Storage::Streams::IInputStream>, uri: &::core::option::Option<super::super::super::Foundation::Uri>, contenttype: &::windows::core::HSTRING, httpclient: &::core::option::Option<super::super::super::Web::Http::HttpClient>) -> ::windows::core::Result<super::super::super::Foundation::IAsyncOperation<AdaptiveMediaSourceCreationResult>>;
}

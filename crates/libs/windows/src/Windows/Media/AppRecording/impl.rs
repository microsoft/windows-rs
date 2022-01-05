#[cfg(feature = "implement_exclusive")]
pub trait IAppRecordingManagerImpl: Sized {
    fn GetStatus(&self) -> ::windows::core::Result<AppRecordingStatus>;
    fn StartRecordingToFileAsync(&self, file: &::core::option::Option<super::super::Storage::StorageFile>) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<AppRecordingResult>>;
    fn RecordTimeSpanToFileAsync(&self, starttime: &super::super::Foundation::DateTime, duration: &super::super::Foundation::TimeSpan, file: &::core::option::Option<super::super::Storage::StorageFile>) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<AppRecordingResult>>;
    fn SupportedScreenshotMediaEncodingSubtypes(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<::windows::core::HSTRING>>;
    fn SaveScreenshotToFilesAsync(&self, folder: &::core::option::Option<super::super::Storage::StorageFolder>, filenameprefix: &::windows::core::HSTRING, option: AppRecordingSaveScreenshotOption, requestedformats: &::core::option::Option<super::super::Foundation::Collections::IIterable<::windows::core::HSTRING>>) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<AppRecordingSaveScreenshotResult>>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IAppRecordingManagerStaticsImpl: Sized {
    fn GetDefault(&self) -> ::windows::core::Result<AppRecordingManager>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IAppRecordingResultImpl: Sized {
    fn Succeeded(&self) -> ::windows::core::Result<bool>;
    fn ExtendedError(&self) -> ::windows::core::Result<::windows::core::HRESULT>;
    fn Duration(&self) -> ::windows::core::Result<super::super::Foundation::TimeSpan>;
    fn IsFileTruncated(&self) -> ::windows::core::Result<bool>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IAppRecordingSaveScreenshotResultImpl: Sized {
    fn Succeeded(&self) -> ::windows::core::Result<bool>;
    fn ExtendedError(&self) -> ::windows::core::Result<::windows::core::HRESULT>;
    fn SavedScreenshotInfos(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<AppRecordingSavedScreenshotInfo>>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IAppRecordingSavedScreenshotInfoImpl: Sized {
    fn File(&self) -> ::windows::core::Result<super::super::Storage::StorageFile>;
    fn MediaEncodingSubtype(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IAppRecordingStatusImpl: Sized {
    fn CanRecord(&self) -> ::windows::core::Result<bool>;
    fn CanRecordTimeSpan(&self) -> ::windows::core::Result<bool>;
    fn HistoricalBufferDuration(&self) -> ::windows::core::Result<super::super::Foundation::TimeSpan>;
    fn Details(&self) -> ::windows::core::Result<AppRecordingStatusDetails>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IAppRecordingStatusDetailsImpl: Sized {
    fn IsAnyAppBroadcasting(&self) -> ::windows::core::Result<bool>;
    fn IsCaptureResourceUnavailable(&self) -> ::windows::core::Result<bool>;
    fn IsGameStreamInProgress(&self) -> ::windows::core::Result<bool>;
    fn IsTimeSpanRecordingDisabled(&self) -> ::windows::core::Result<bool>;
    fn IsGpuConstrained(&self) -> ::windows::core::Result<bool>;
    fn IsAppInactive(&self) -> ::windows::core::Result<bool>;
    fn IsBlockedForApp(&self) -> ::windows::core::Result<bool>;
    fn IsDisabledByUser(&self) -> ::windows::core::Result<bool>;
    fn IsDisabledBySystem(&self) -> ::windows::core::Result<bool>;
}

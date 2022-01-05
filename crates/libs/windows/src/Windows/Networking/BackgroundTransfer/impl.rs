#[cfg(feature = "implement_exclusive")]
pub trait IBackgroundDownloaderImpl: Sized + IBackgroundTransferBaseImpl {
    fn CreateDownload(&self, uri: &::core::option::Option<super::super::Foundation::Uri>, resultfile: &::core::option::Option<super::super::Storage::IStorageFile>) -> ::windows::core::Result<DownloadOperation>;
    fn CreateDownloadFromFile(&self, uri: &::core::option::Option<super::super::Foundation::Uri>, resultfile: &::core::option::Option<super::super::Storage::IStorageFile>, requestbodyfile: &::core::option::Option<super::super::Storage::IStorageFile>) -> ::windows::core::Result<DownloadOperation>;
    fn CreateDownloadAsync(&self, uri: &::core::option::Option<super::super::Foundation::Uri>, resultfile: &::core::option::Option<super::super::Storage::IStorageFile>, requestbodystream: &::core::option::Option<super::super::Storage::Streams::IInputStream>) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<DownloadOperation>>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IBackgroundDownloader2Impl: Sized {
    fn TransferGroup(&self) -> ::windows::core::Result<BackgroundTransferGroup>;
    fn SetTransferGroup(&self, value: &::core::option::Option<BackgroundTransferGroup>) -> ::windows::core::Result<()>;
    fn SuccessToastNotification(&self) -> ::windows::core::Result<super::super::UI::Notifications::ToastNotification>;
    fn SetSuccessToastNotification(&self, value: &::core::option::Option<super::super::UI::Notifications::ToastNotification>) -> ::windows::core::Result<()>;
    fn FailureToastNotification(&self) -> ::windows::core::Result<super::super::UI::Notifications::ToastNotification>;
    fn SetFailureToastNotification(&self, value: &::core::option::Option<super::super::UI::Notifications::ToastNotification>) -> ::windows::core::Result<()>;
    fn SuccessTileNotification(&self) -> ::windows::core::Result<super::super::UI::Notifications::TileNotification>;
    fn SetSuccessTileNotification(&self, value: &::core::option::Option<super::super::UI::Notifications::TileNotification>) -> ::windows::core::Result<()>;
    fn FailureTileNotification(&self) -> ::windows::core::Result<super::super::UI::Notifications::TileNotification>;
    fn SetFailureTileNotification(&self, value: &::core::option::Option<super::super::UI::Notifications::TileNotification>) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IBackgroundDownloader3Impl: Sized {
    fn CompletionGroup(&self) -> ::windows::core::Result<BackgroundTransferCompletionGroup>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IBackgroundDownloaderFactoryImpl: Sized {
    fn CreateWithCompletionGroup(&self, completiongroup: &::core::option::Option<BackgroundTransferCompletionGroup>) -> ::windows::core::Result<BackgroundDownloader>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IBackgroundDownloaderStaticMethodsImpl: Sized {
    fn GetCurrentDownloadsAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IVectorView<DownloadOperation>>>;
    fn GetCurrentDownloadsForGroupAsync(&self, group: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IVectorView<DownloadOperation>>>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IBackgroundDownloaderStaticMethods2Impl: Sized {
    fn GetCurrentDownloadsForTransferGroupAsync(&self, group: &::core::option::Option<BackgroundTransferGroup>) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IVectorView<DownloadOperation>>>;
}
#[cfg(all(feature = "deprecated", feature = "implement_exclusive"))]
pub trait IBackgroundDownloaderUserConsentImpl: Sized {
    fn RequestUnconstrainedDownloadsAsync(&self, operations: &::core::option::Option<super::super::Foundation::Collections::IIterable<DownloadOperation>>) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<UnconstrainedTransferRequestResult>>;
}
pub trait IBackgroundTransferBaseImpl: Sized {
    fn SetRequestHeader(&self, headername: &::windows::core::HSTRING, headervalue: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn ServerCredential(&self) -> ::windows::core::Result<super::super::Security::Credentials::PasswordCredential>;
    fn SetServerCredential(&self, credential: &::core::option::Option<super::super::Security::Credentials::PasswordCredential>) -> ::windows::core::Result<()>;
    fn ProxyCredential(&self) -> ::windows::core::Result<super::super::Security::Credentials::PasswordCredential>;
    fn SetProxyCredential(&self, credential: &::core::option::Option<super::super::Security::Credentials::PasswordCredential>) -> ::windows::core::Result<()>;
    fn Method(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetMethod(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn Group(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetGroup(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn CostPolicy(&self) -> ::windows::core::Result<BackgroundTransferCostPolicy>;
    fn SetCostPolicy(&self, value: BackgroundTransferCostPolicy) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IBackgroundTransferCompletionGroupImpl: Sized {
    fn Trigger(&self) -> ::windows::core::Result<super::super::ApplicationModel::Background::IBackgroundTrigger>;
    fn IsEnabled(&self) -> ::windows::core::Result<bool>;
    fn Enable(&self) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IBackgroundTransferCompletionGroupTriggerDetailsImpl: Sized {
    fn Downloads(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<DownloadOperation>>;
    fn Uploads(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVectorView<UploadOperation>>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IBackgroundTransferContentPartImpl: Sized {
    fn SetHeader(&self, headername: &::windows::core::HSTRING, headervalue: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn SetText(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn SetFile(&self, value: &::core::option::Option<super::super::Storage::IStorageFile>) -> ::windows::core::Result<()>;
}
pub trait IBackgroundTransferContentPartFactoryImpl: Sized {
    fn CreateWithName(&self, name: &::windows::core::HSTRING) -> ::windows::core::Result<BackgroundTransferContentPart>;
    fn CreateWithNameAndFileName(&self, name: &::windows::core::HSTRING, filename: &::windows::core::HSTRING) -> ::windows::core::Result<BackgroundTransferContentPart>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IBackgroundTransferErrorStaticMethodsImpl: Sized {
    fn GetStatus(&self, hresult: i32) -> ::windows::core::Result<super::super::Web::WebErrorStatus>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IBackgroundTransferGroupImpl: Sized {
    fn Name(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn TransferBehavior(&self) -> ::windows::core::Result<BackgroundTransferBehavior>;
    fn SetTransferBehavior(&self, value: BackgroundTransferBehavior) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IBackgroundTransferGroupStaticsImpl: Sized {
    fn CreateGroup(&self, name: &::windows::core::HSTRING) -> ::windows::core::Result<BackgroundTransferGroup>;
}
pub trait IBackgroundTransferOperationImpl: Sized {
    fn Guid(&self) -> ::windows::core::Result<::windows::core::GUID>;
    fn RequestedUri(&self) -> ::windows::core::Result<super::super::Foundation::Uri>;
    fn Method(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn Group(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn CostPolicy(&self) -> ::windows::core::Result<BackgroundTransferCostPolicy>;
    fn SetCostPolicy(&self, value: BackgroundTransferCostPolicy) -> ::windows::core::Result<()>;
    fn GetResultStreamAt(&self, position: u64) -> ::windows::core::Result<super::super::Storage::Streams::IInputStream>;
    fn GetResponseInformation(&self) -> ::windows::core::Result<ResponseInformation>;
}
pub trait IBackgroundTransferOperationPriorityImpl: Sized {
    fn Priority(&self) -> ::windows::core::Result<BackgroundTransferPriority>;
    fn SetPriority(&self, value: BackgroundTransferPriority) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IBackgroundTransferRangesDownloadedEventArgsImpl: Sized {
    fn WasDownloadRestarted(&self) -> ::windows::core::Result<bool>;
    fn AddedRanges(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVector<BackgroundTransferFileRange>>;
    fn GetDeferral(&self) -> ::windows::core::Result<super::super::Foundation::Deferral>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IBackgroundUploaderImpl: Sized + IBackgroundTransferBaseImpl {
    fn CreateUpload(&self, uri: &::core::option::Option<super::super::Foundation::Uri>, sourcefile: &::core::option::Option<super::super::Storage::IStorageFile>) -> ::windows::core::Result<UploadOperation>;
    fn CreateUploadFromStreamAsync(&self, uri: &::core::option::Option<super::super::Foundation::Uri>, sourcestream: &::core::option::Option<super::super::Storage::Streams::IInputStream>) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<UploadOperation>>;
    fn CreateUploadWithFormDataAndAutoBoundaryAsync(&self, uri: &::core::option::Option<super::super::Foundation::Uri>, parts: &::core::option::Option<super::super::Foundation::Collections::IIterable<BackgroundTransferContentPart>>) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<UploadOperation>>;
    fn CreateUploadWithSubTypeAsync(&self, uri: &::core::option::Option<super::super::Foundation::Uri>, parts: &::core::option::Option<super::super::Foundation::Collections::IIterable<BackgroundTransferContentPart>>, subtype: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<UploadOperation>>;
    fn CreateUploadWithSubTypeAndBoundaryAsync(&self, uri: &::core::option::Option<super::super::Foundation::Uri>, parts: &::core::option::Option<super::super::Foundation::Collections::IIterable<BackgroundTransferContentPart>>, subtype: &::windows::core::HSTRING, boundary: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<UploadOperation>>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IBackgroundUploader2Impl: Sized {
    fn TransferGroup(&self) -> ::windows::core::Result<BackgroundTransferGroup>;
    fn SetTransferGroup(&self, value: &::core::option::Option<BackgroundTransferGroup>) -> ::windows::core::Result<()>;
    fn SuccessToastNotification(&self) -> ::windows::core::Result<super::super::UI::Notifications::ToastNotification>;
    fn SetSuccessToastNotification(&self, value: &::core::option::Option<super::super::UI::Notifications::ToastNotification>) -> ::windows::core::Result<()>;
    fn FailureToastNotification(&self) -> ::windows::core::Result<super::super::UI::Notifications::ToastNotification>;
    fn SetFailureToastNotification(&self, value: &::core::option::Option<super::super::UI::Notifications::ToastNotification>) -> ::windows::core::Result<()>;
    fn SuccessTileNotification(&self) -> ::windows::core::Result<super::super::UI::Notifications::TileNotification>;
    fn SetSuccessTileNotification(&self, value: &::core::option::Option<super::super::UI::Notifications::TileNotification>) -> ::windows::core::Result<()>;
    fn FailureTileNotification(&self) -> ::windows::core::Result<super::super::UI::Notifications::TileNotification>;
    fn SetFailureTileNotification(&self, value: &::core::option::Option<super::super::UI::Notifications::TileNotification>) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IBackgroundUploader3Impl: Sized {
    fn CompletionGroup(&self) -> ::windows::core::Result<BackgroundTransferCompletionGroup>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IBackgroundUploaderFactoryImpl: Sized {
    fn CreateWithCompletionGroup(&self, completiongroup: &::core::option::Option<BackgroundTransferCompletionGroup>) -> ::windows::core::Result<BackgroundUploader>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IBackgroundUploaderStaticMethodsImpl: Sized {
    fn GetCurrentUploadsAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IVectorView<UploadOperation>>>;
    fn GetCurrentUploadsForGroupAsync(&self, group: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IVectorView<UploadOperation>>>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IBackgroundUploaderStaticMethods2Impl: Sized {
    fn GetCurrentUploadsForTransferGroupAsync(&self, group: &::core::option::Option<BackgroundTransferGroup>) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IVectorView<UploadOperation>>>;
}
#[cfg(all(feature = "deprecated", feature = "implement_exclusive"))]
pub trait IBackgroundUploaderUserConsentImpl: Sized {
    fn RequestUnconstrainedUploadsAsync(&self, operations: &::core::option::Option<super::super::Foundation::Collections::IIterable<UploadOperation>>) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<UnconstrainedTransferRequestResult>>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IContentPrefetcherImpl: Sized {
    fn ContentUris(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVector<super::super::Foundation::Uri>>;
    fn SetIndirectContentUri(&self, value: &::core::option::Option<super::super::Foundation::Uri>) -> ::windows::core::Result<()>;
    fn IndirectContentUri(&self) -> ::windows::core::Result<super::super::Foundation::Uri>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IContentPrefetcherTimeImpl: Sized {
    fn LastSuccessfulPrefetchTime(&self) -> ::windows::core::Result<super::super::Foundation::IReference<super::super::Foundation::DateTime>>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IDownloadOperationImpl: Sized + IBackgroundTransferOperationImpl {
    fn ResultFile(&self) -> ::windows::core::Result<super::super::Storage::IStorageFile>;
    fn Progress(&self) -> ::windows::core::Result<BackgroundDownloadProgress>;
    fn StartAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperationWithProgress<DownloadOperation, DownloadOperation>>;
    fn AttachAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperationWithProgress<DownloadOperation, DownloadOperation>>;
    fn Pause(&self) -> ::windows::core::Result<()>;
    fn Resume(&self) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IDownloadOperation2Impl: Sized {
    fn TransferGroup(&self) -> ::windows::core::Result<BackgroundTransferGroup>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IDownloadOperation3Impl: Sized {
    fn IsRandomAccessRequired(&self) -> ::windows::core::Result<bool>;
    fn SetIsRandomAccessRequired(&self, value: bool) -> ::windows::core::Result<()>;
    fn GetResultRandomAccessStreamReference(&self) -> ::windows::core::Result<super::super::Storage::Streams::IRandomAccessStreamReference>;
    fn GetDownloadedRanges(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVector<BackgroundTransferFileRange>>;
    fn RangesDownloaded(&self, eventhandler: &::core::option::Option<super::super::Foundation::TypedEventHandler<DownloadOperation, BackgroundTransferRangesDownloadedEventArgs>>) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>;
    fn RemoveRangesDownloaded(&self, eventcookie: &super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()>;
    fn SetRequestedUri(&self, value: &::core::option::Option<super::super::Foundation::Uri>) -> ::windows::core::Result<()>;
    fn RecoverableWebErrorStatuses(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVector<super::super::Web::WebErrorStatus>>;
    fn CurrentWebErrorStatus(&self) -> ::windows::core::Result<super::super::Foundation::IReference<super::super::Web::WebErrorStatus>>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IDownloadOperation4Impl: Sized {
    fn MakeCurrentInTransferGroup(&self) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IDownloadOperation5Impl: Sized {
    fn SetRequestHeader(&self, headername: &::windows::core::HSTRING, headervalue: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn RemoveRequestHeader(&self, headername: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IResponseInformationImpl: Sized {
    fn IsResumable(&self) -> ::windows::core::Result<bool>;
    fn ActualUri(&self) -> ::windows::core::Result<super::super::Foundation::Uri>;
    fn StatusCode(&self) -> ::windows::core::Result<u32>;
    fn Headers(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IMapView<::windows::core::HSTRING, ::windows::core::HSTRING>>;
}
#[cfg(all(feature = "deprecated", feature = "implement_exclusive"))]
pub trait IUnconstrainedTransferRequestResultImpl: Sized {
    fn IsUnconstrained(&self) -> ::windows::core::Result<bool>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IUploadOperationImpl: Sized + IBackgroundTransferOperationImpl {
    fn SourceFile(&self) -> ::windows::core::Result<super::super::Storage::IStorageFile>;
    fn Progress(&self) -> ::windows::core::Result<BackgroundUploadProgress>;
    fn StartAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperationWithProgress<UploadOperation, UploadOperation>>;
    fn AttachAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperationWithProgress<UploadOperation, UploadOperation>>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IUploadOperation2Impl: Sized {
    fn TransferGroup(&self) -> ::windows::core::Result<BackgroundTransferGroup>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IUploadOperation3Impl: Sized {
    fn MakeCurrentInTransferGroup(&self) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IUploadOperation4Impl: Sized {
    fn SetRequestHeader(&self, headername: &::windows::core::HSTRING, headervalue: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn RemoveRequestHeader(&self, headername: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
}

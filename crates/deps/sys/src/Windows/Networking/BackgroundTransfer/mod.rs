#![allow(non_snake_case, non_camel_case_types)]
#[link(name = "windows")]
extern "system" {}
#[repr(C)]
pub struct BackgroundDownloadProgress(i32);
#[repr(transparent)]
pub struct BackgroundDownloader(pub *mut ::core::ffi::c_void);
#[repr(C)]
pub struct BackgroundTransferBehavior(i32);
#[repr(transparent)]
pub struct BackgroundTransferCompletionGroup(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct BackgroundTransferCompletionGroupTriggerDetails(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct BackgroundTransferContentPart(pub *mut ::core::ffi::c_void);
#[repr(C)]
pub struct BackgroundTransferCostPolicy(i32);
#[repr(C)]
pub struct BackgroundTransferFileRange(i32);
#[repr(transparent)]
pub struct BackgroundTransferGroup(pub *mut ::core::ffi::c_void);
#[repr(C)]
pub struct BackgroundTransferPriority(i32);
#[repr(transparent)]
pub struct BackgroundTransferRangesDownloadedEventArgs(pub *mut ::core::ffi::c_void);
#[repr(C)]
pub struct BackgroundTransferStatus(i32);
#[repr(C)]
pub struct BackgroundUploadProgress(i32);
#[repr(transparent)]
pub struct BackgroundUploader(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct DownloadOperation(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IBackgroundDownloader(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IBackgroundDownloader2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IBackgroundDownloader3(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IBackgroundDownloaderFactory(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IBackgroundDownloaderStaticMethods(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IBackgroundDownloaderStaticMethods2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IBackgroundDownloaderUserConsent(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IBackgroundTransferBase(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IBackgroundTransferCompletionGroup(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IBackgroundTransferCompletionGroupTriggerDetails(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IBackgroundTransferContentPart(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IBackgroundTransferContentPartFactory(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IBackgroundTransferErrorStaticMethods(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IBackgroundTransferGroup(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IBackgroundTransferGroupStatics(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IBackgroundTransferOperation(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IBackgroundTransferOperationPriority(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IBackgroundTransferRangesDownloadedEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IBackgroundUploader(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IBackgroundUploader2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IBackgroundUploader3(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IBackgroundUploaderFactory(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IBackgroundUploaderStaticMethods(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IBackgroundUploaderStaticMethods2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IBackgroundUploaderUserConsent(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IContentPrefetcher(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IContentPrefetcherTime(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IDownloadOperation(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IDownloadOperation2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IDownloadOperation3(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IDownloadOperation4(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IDownloadOperation5(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IResponseInformation(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IUnconstrainedTransferRequestResult(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IUploadOperation(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IUploadOperation2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IUploadOperation3(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IUploadOperation4(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ResponseInformation(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct UnconstrainedTransferRequestResult(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct UploadOperation(pub *mut ::core::ffi::c_void);

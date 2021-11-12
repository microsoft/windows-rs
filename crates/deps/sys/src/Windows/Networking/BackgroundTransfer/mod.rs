#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals)]
#[link(name = "windows")]
extern "system" {}
#[repr(C)]
pub struct BackgroundDownloadProgress {
    pub BytesReceived: u64,
    pub TotalBytesToReceive: u64,
    pub Status: BackgroundTransferStatus,
    pub HasResponseChanged: bool,
    pub HasRestarted: bool,
}
impl ::core::marker::Copy for BackgroundDownloadProgress {}
impl ::core::clone::Clone for BackgroundDownloadProgress {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct BackgroundDownloader(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct BackgroundTransferBehavior(pub i32);
impl BackgroundTransferBehavior {
    pub const Parallel: Self = Self(0i32);
    pub const Serialized: Self = Self(1i32);
}
impl ::core::marker::Copy for BackgroundTransferBehavior {}
impl ::core::clone::Clone for BackgroundTransferBehavior {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct BackgroundTransferCompletionGroup(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct BackgroundTransferCompletionGroupTriggerDetails(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct BackgroundTransferContentPart(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct BackgroundTransferCostPolicy(pub i32);
impl BackgroundTransferCostPolicy {
    pub const Default: Self = Self(0i32);
    pub const UnrestrictedOnly: Self = Self(1i32);
    pub const Always: Self = Self(2i32);
}
impl ::core::marker::Copy for BackgroundTransferCostPolicy {}
impl ::core::clone::Clone for BackgroundTransferCostPolicy {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct BackgroundTransferFileRange {
    pub Offset: u64,
    pub Length: u64,
}
impl ::core::marker::Copy for BackgroundTransferFileRange {}
impl ::core::clone::Clone for BackgroundTransferFileRange {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct BackgroundTransferGroup(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct BackgroundTransferPriority(pub i32);
impl BackgroundTransferPriority {
    pub const Default: Self = Self(0i32);
    pub const High: Self = Self(1i32);
    pub const Low: Self = Self(2i32);
}
impl ::core::marker::Copy for BackgroundTransferPriority {}
impl ::core::clone::Clone for BackgroundTransferPriority {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct BackgroundTransferRangesDownloadedEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct BackgroundTransferStatus(pub i32);
impl BackgroundTransferStatus {
    pub const Idle: Self = Self(0i32);
    pub const Running: Self = Self(1i32);
    pub const PausedByApplication: Self = Self(2i32);
    pub const PausedCostedNetwork: Self = Self(3i32);
    pub const PausedNoNetwork: Self = Self(4i32);
    pub const Completed: Self = Self(5i32);
    pub const Canceled: Self = Self(6i32);
    pub const Error: Self = Self(7i32);
    pub const PausedRecoverableWebErrorStatus: Self = Self(8i32);
    pub const PausedSystemPolicy: Self = Self(32i32);
}
impl ::core::marker::Copy for BackgroundTransferStatus {}
impl ::core::clone::Clone for BackgroundTransferStatus {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct BackgroundUploadProgress {
    pub BytesReceived: u64,
    pub BytesSent: u64,
    pub TotalBytesToReceive: u64,
    pub TotalBytesToSend: u64,
    pub Status: BackgroundTransferStatus,
    pub HasResponseChanged: bool,
    pub HasRestarted: bool,
}
impl ::core::marker::Copy for BackgroundUploadProgress {}
impl ::core::clone::Clone for BackgroundUploadProgress {
    fn clone(&self) -> Self {
        *self
    }
}
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

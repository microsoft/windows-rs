pub trait AsyncIBackgroundCopyCallbackImpl: Sized {
    fn Begin_JobTransferred();
    fn Finish_JobTransferred();
    fn Begin_JobError();
    fn Finish_JobError();
    fn Begin_JobModification();
    fn Finish_JobModification();
}
#[cfg(feature = "Win32_System_Com")]
pub trait IBITSExtensionSetupImpl: Sized + IDispatchImpl {
    fn EnableBITSUploads();
    fn DisableBITSUploads();
    fn GetCleanupTaskName();
    fn GetCleanupTask();
}
#[cfg(feature = "Win32_System_Com")]
pub trait IBITSExtensionSetupFactoryImpl: Sized + IDispatchImpl {
    fn GetObject();
}
pub trait IBackgroundCopyCallbackImpl: Sized {
    fn JobTransferred();
    fn JobError();
    fn JobModification();
}
pub trait IBackgroundCopyCallback1Impl: Sized {
    fn OnStatus();
    fn OnProgress();
    fn OnProgressEx();
}
pub trait IBackgroundCopyCallback2Impl: Sized + IBackgroundCopyCallbackImpl {
    fn FileTransferred();
}
pub trait IBackgroundCopyCallback3Impl: Sized + IBackgroundCopyCallback2Impl + IBackgroundCopyCallbackImpl {
    fn FileRangesTransferred();
}
pub trait IBackgroundCopyErrorImpl: Sized {
    fn GetError();
    fn GetFile();
    fn GetErrorDescription();
    fn GetErrorContextDescription();
    fn GetProtocol();
}
pub trait IBackgroundCopyFileImpl: Sized {
    fn GetRemoteName();
    fn GetLocalName();
    fn GetProgress();
}
pub trait IBackgroundCopyFile2Impl: Sized + IBackgroundCopyFileImpl {
    fn GetFileRanges();
    fn SetRemoteName();
}
pub trait IBackgroundCopyFile3Impl: Sized + IBackgroundCopyFile2Impl + IBackgroundCopyFileImpl {
    fn GetTemporaryName();
    fn SetValidationState();
    fn GetValidationState();
    fn IsDownloadedFromPeer();
}
pub trait IBackgroundCopyFile4Impl: Sized + IBackgroundCopyFile3Impl + IBackgroundCopyFile2Impl + IBackgroundCopyFileImpl {
    fn GetPeerDownloadStats();
}
pub trait IBackgroundCopyFile5Impl: Sized + IBackgroundCopyFile4Impl + IBackgroundCopyFile3Impl + IBackgroundCopyFile2Impl + IBackgroundCopyFileImpl {
    fn SetProperty();
    fn GetProperty();
}
pub trait IBackgroundCopyFile6Impl: Sized + IBackgroundCopyFile5Impl + IBackgroundCopyFile4Impl + IBackgroundCopyFile3Impl + IBackgroundCopyFile2Impl + IBackgroundCopyFileImpl {
    fn UpdateDownloadPosition();
    fn RequestFileRanges();
    fn GetFilledFileRanges();
}
pub trait IBackgroundCopyGroupImpl: Sized {
    fn GetProp();
    fn SetProp();
    fn GetProgress();
    fn GetStatus();
    fn GetJob();
    fn SuspendGroup();
    fn ResumeGroup();
    fn CancelGroup();
    fn Size();
    fn GroupID();
    fn CreateJob();
    fn EnumJobs();
    fn SwitchToForeground();
    fn QueryNewJobInterface();
    fn SetNotificationPointer();
}
pub trait IBackgroundCopyJobImpl: Sized {
    fn AddFileSet();
    fn AddFile();
    fn EnumFiles();
    fn Suspend();
    fn Resume();
    fn Cancel();
    fn Complete();
    fn GetId();
    fn GetType();
    fn GetProgress();
    fn GetTimes();
    fn GetState();
    fn GetError();
    fn GetOwner();
    fn SetDisplayName();
    fn GetDisplayName();
    fn SetDescription();
    fn GetDescription();
    fn SetPriority();
    fn GetPriority();
    fn SetNotifyFlags();
    fn GetNotifyFlags();
    fn SetNotifyInterface();
    fn GetNotifyInterface();
    fn SetMinimumRetryDelay();
    fn GetMinimumRetryDelay();
    fn SetNoProgressTimeout();
    fn GetNoProgressTimeout();
    fn GetErrorCount();
    fn SetProxySettings();
    fn GetProxySettings();
    fn TakeOwnership();
}
pub trait IBackgroundCopyJob1Impl: Sized {
    fn CancelJob();
    fn GetProgress();
    fn GetStatus();
    fn AddFiles();
    fn GetFile();
    fn GetFileCount();
    fn SwitchToForeground();
    fn JobID();
}
pub trait IBackgroundCopyJob2Impl: Sized + IBackgroundCopyJobImpl {
    fn SetNotifyCmdLine();
    fn GetNotifyCmdLine();
    fn GetReplyProgress();
    fn GetReplyData();
    fn SetReplyFileName();
    fn GetReplyFileName();
    fn SetCredentials();
    fn RemoveCredentials();
}
pub trait IBackgroundCopyJob3Impl: Sized + IBackgroundCopyJob2Impl + IBackgroundCopyJobImpl {
    fn ReplaceRemotePrefix();
    fn AddFileWithRanges();
    fn SetFileACLFlags();
    fn GetFileACLFlags();
}
pub trait IBackgroundCopyJob4Impl: Sized + IBackgroundCopyJob3Impl + IBackgroundCopyJob2Impl + IBackgroundCopyJobImpl {
    fn SetPeerCachingFlags();
    fn GetPeerCachingFlags();
    fn GetOwnerIntegrityLevel();
    fn GetOwnerElevationState();
    fn SetMaximumDownloadTime();
    fn GetMaximumDownloadTime();
}
pub trait IBackgroundCopyJob5Impl: Sized + IBackgroundCopyJob4Impl + IBackgroundCopyJob3Impl + IBackgroundCopyJob2Impl + IBackgroundCopyJobImpl {
    fn SetProperty();
    fn GetProperty();
}
pub trait IBackgroundCopyJobHttpOptionsImpl: Sized {
    fn SetClientCertificateByID();
    fn SetClientCertificateByName();
    fn RemoveClientCertificate();
    fn GetClientCertificate();
    fn SetCustomHeaders();
    fn GetCustomHeaders();
    fn SetSecurityFlags();
    fn GetSecurityFlags();
}
pub trait IBackgroundCopyJobHttpOptions2Impl: Sized + IBackgroundCopyJobHttpOptionsImpl {
    fn SetHttpMethod();
    fn GetHttpMethod();
}
pub trait IBackgroundCopyJobHttpOptions3Impl: Sized + IBackgroundCopyJobHttpOptions2Impl + IBackgroundCopyJobHttpOptionsImpl {
    fn SetServerCertificateValidationInterface();
    fn MakeCustomHeadersWriteOnly();
}
pub trait IBackgroundCopyManagerImpl: Sized {
    fn CreateJob();
    fn GetJob();
    fn EnumJobs();
    fn GetErrorDescription();
}
pub trait IBackgroundCopyQMgrImpl: Sized {
    fn CreateGroup();
    fn GetGroup();
    fn EnumGroups();
}
pub trait IBackgroundCopyServerCertificateValidationCallbackImpl: Sized {
    fn ValidateServerCertificate();
}
pub trait IBitsPeerImpl: Sized {
    fn GetPeerName();
    fn IsAuthenticated();
    fn IsAvailable();
}
pub trait IBitsPeerCacheAdministrationImpl: Sized {
    fn GetMaximumCacheSize();
    fn SetMaximumCacheSize();
    fn GetMaximumContentAge();
    fn SetMaximumContentAge();
    fn GetConfigurationFlags();
    fn SetConfigurationFlags();
    fn EnumRecords();
    fn GetRecord();
    fn ClearRecords();
    fn DeleteRecord();
    fn DeleteUrl();
    fn EnumPeers();
    fn ClearPeers();
    fn DiscoverPeers();
}
pub trait IBitsPeerCacheRecordImpl: Sized {
    fn GetId();
    fn GetOriginUrl();
    fn GetFileSize();
    fn GetFileModificationTime();
    fn GetLastAccessTime();
    fn IsFileValidated();
    fn GetFileRanges();
}
pub trait IBitsTokenOptionsImpl: Sized {
    fn SetHelperTokenFlags();
    fn GetHelperTokenFlags();
    fn SetHelperToken();
    fn ClearHelperToken();
    fn GetHelperTokenSid();
}
pub trait IEnumBackgroundCopyFilesImpl: Sized {
    fn Next();
    fn Skip();
    fn Reset();
    fn Clone();
    fn GetCount();
}
pub trait IEnumBackgroundCopyGroupsImpl: Sized {
    fn Next();
    fn Skip();
    fn Reset();
    fn Clone();
    fn GetCount();
}
pub trait IEnumBackgroundCopyJobsImpl: Sized {
    fn Next();
    fn Skip();
    fn Reset();
    fn Clone();
    fn GetCount();
}
pub trait IEnumBackgroundCopyJobs1Impl: Sized {
    fn Next();
    fn Skip();
    fn Reset();
    fn Clone();
    fn GetCount();
}
pub trait IEnumBitsPeerCacheRecordsImpl: Sized {
    fn Next();
    fn Skip();
    fn Reset();
    fn Clone();
    fn GetCount();
}
pub trait IEnumBitsPeersImpl: Sized {
    fn Next();
    fn Skip();
    fn Reset();
    fn Clone();
    fn GetCount();
}

pub trait IVssAdminImpl: Sized {
    fn RegisterProvider();
    fn UnregisterProvider();
    fn QueryProviders();
    fn AbortAllSnapshotsInProgress();
}
pub trait IVssAdminExImpl: Sized + IVssAdminImpl {
    fn GetProviderCapability();
    fn GetProviderContext();
    fn SetProviderContext();
}
pub trait IVssAsyncImpl: Sized {
    fn Cancel();
    fn Wait();
    fn QueryStatus();
}
pub trait IVssComponentImpl: Sized {
    fn GetLogicalPath();
    fn GetComponentType();
    fn GetComponentName();
    fn GetBackupSucceeded();
    fn GetAlternateLocationMappingCount();
    fn GetAlternateLocationMapping();
    fn SetBackupMetadata();
    fn GetBackupMetadata();
    fn AddPartialFile();
    fn GetPartialFileCount();
    fn GetPartialFile();
    fn IsSelectedForRestore();
    fn GetAdditionalRestores();
    fn GetNewTargetCount();
    fn GetNewTarget();
    fn AddDirectedTarget();
    fn GetDirectedTargetCount();
    fn GetDirectedTarget();
    fn SetRestoreMetadata();
    fn GetRestoreMetadata();
    fn SetRestoreTarget();
    fn GetRestoreTarget();
    fn SetPreRestoreFailureMsg();
    fn GetPreRestoreFailureMsg();
    fn SetPostRestoreFailureMsg();
    fn GetPostRestoreFailureMsg();
    fn SetBackupStamp();
    fn GetBackupStamp();
    fn GetPreviousBackupStamp();
    fn GetBackupOptions();
    fn GetRestoreOptions();
    fn GetRestoreSubcomponentCount();
    fn GetRestoreSubcomponent();
    fn GetFileRestoreStatus();
    fn AddDifferencedFilesByLastModifyTime();
    fn AddDifferencedFilesByLastModifyLSN();
    fn GetDifferencedFilesCount();
    fn GetDifferencedFile();
}
pub trait IVssComponentExImpl: Sized + IVssComponentImpl {
    fn SetPrepareForBackupFailureMsg();
    fn SetPostSnapshotFailureMsg();
    fn GetPrepareForBackupFailureMsg();
    fn GetPostSnapshotFailureMsg();
    fn GetAuthoritativeRestore();
    fn GetRollForward();
    fn GetRestoreName();
}
pub trait IVssComponentEx2Impl: Sized + IVssComponentExImpl + IVssComponentImpl {
    fn SetFailure();
    fn GetFailure();
}
pub trait IVssCreateExpressWriterMetadataImpl: Sized {
    fn AddExcludeFiles();
    fn AddComponent();
    fn AddFilesToFileGroup();
    fn SetRestoreMethod();
    fn AddComponentDependency();
    fn SetBackupSchema();
    fn SaveAsXML();
}
pub trait IVssCreateWriterMetadataImpl: Sized {
    fn AddIncludeFiles();
    fn AddExcludeFiles();
    fn AddComponent();
    fn AddDatabaseFiles();
    fn AddDatabaseLogFiles();
    fn AddFilesToFileGroup();
    fn SetRestoreMethod();
    fn AddAlternateLocationMapping();
    fn AddComponentDependency();
    fn SetBackupSchema();
    fn GetDocument();
    fn SaveAsXML();
}
pub trait IVssDifferentialSoftwareSnapshotMgmtImpl: Sized {
    fn AddDiffArea();
    fn ChangeDiffAreaMaximumSize();
    fn QueryVolumesSupportedForDiffAreas();
    fn QueryDiffAreasForVolume();
    fn QueryDiffAreasOnVolume();
    fn QueryDiffAreasForSnapshot();
}
pub trait IVssDifferentialSoftwareSnapshotMgmt2Impl: Sized + IVssDifferentialSoftwareSnapshotMgmtImpl {
    fn ChangeDiffAreaMaximumSizeEx();
    fn MigrateDiffAreas();
    fn QueryMigrationStatus();
    fn SetSnapshotPriority();
}
pub trait IVssDifferentialSoftwareSnapshotMgmt3Impl: Sized + IVssDifferentialSoftwareSnapshotMgmt2Impl + IVssDifferentialSoftwareSnapshotMgmtImpl {
    fn SetVolumeProtectLevel();
    fn GetVolumeProtectLevel();
    fn ClearVolumeProtectFault();
    fn DeleteUnusedDiffAreas();
    fn QuerySnapshotDeltaBitmap();
}
pub trait IVssEnumMgmtObjectImpl: Sized {
    fn Next();
    fn Skip();
    fn Reset();
    fn Clone();
}
pub trait IVssEnumObjectImpl: Sized {
    fn Next();
    fn Skip();
    fn Reset();
    fn Clone();
}
pub trait IVssExpressWriterImpl: Sized {
    fn CreateMetadata();
    fn LoadMetadata();
    fn Register();
    fn Unregister();
}
pub trait IVssFileShareSnapshotProviderImpl: Sized {
    fn SetContext();
    fn GetSnapshotProperties();
    fn Query();
    fn DeleteSnapshots();
    fn BeginPrepareSnapshot();
    fn IsPathSupported();
    fn IsPathSnapshotted();
    fn SetSnapshotProperty();
}
pub trait IVssHardwareSnapshotProviderImpl: Sized {
    fn AreLunsSupported();
    fn FillInLunInfo();
    fn BeginPrepareSnapshot();
    fn GetTargetLuns();
    fn LocateLuns();
    fn OnLunEmpty();
}
pub trait IVssHardwareSnapshotProviderExImpl: Sized + IVssHardwareSnapshotProviderImpl {
    fn GetProviderCapabilities();
    fn OnLunStateChange();
    fn ResyncLuns();
    fn OnReuseLuns();
}
pub trait IVssProviderCreateSnapshotSetImpl: Sized {
    fn EndPrepareSnapshots();
    fn PreCommitSnapshots();
    fn CommitSnapshots();
    fn PostCommitSnapshots();
    fn PreFinalCommitSnapshots();
    fn PostFinalCommitSnapshots();
    fn AbortSnapshots();
}
pub trait IVssProviderNotificationsImpl: Sized {
    fn OnLoad();
    fn OnUnload();
}
pub trait IVssSnapshotMgmtImpl: Sized {
    fn GetProviderMgmtInterface();
    fn QueryVolumesSupportedForSnapshots();
    fn QuerySnapshotsByVolume();
}
pub trait IVssSnapshotMgmt2Impl: Sized {
    fn GetMinDiffAreaSize();
}
pub trait IVssSoftwareSnapshotProviderImpl: Sized {
    fn SetContext();
    fn GetSnapshotProperties();
    fn Query();
    fn DeleteSnapshots();
    fn BeginPrepareSnapshot();
    fn IsVolumeSupported();
    fn IsVolumeSnapshotted();
    fn SetSnapshotProperty();
    fn RevertToSnapshot();
    fn QueryRevertStatus();
}
pub trait IVssWMDependencyImpl: Sized {
    fn GetWriterId();
    fn GetLogicalPath();
    fn GetComponentName();
}
pub trait IVssWMFiledescImpl: Sized {
    fn GetPath();
    fn GetFilespec();
    fn GetRecursive();
    fn GetAlternateLocation();
    fn GetBackupTypeMask();
}
pub trait IVssWriterComponentsImpl: Sized {
    fn GetComponentCount();
    fn GetWriterInfo();
    fn GetComponent();
}
pub trait IVssWriterImplImpl: Sized {
    fn Initialize();
    fn Subscribe();
    fn Unsubscribe();
    fn Uninitialize();
    fn GetCurrentVolumeArray();
    fn GetCurrentVolumeCount();
    fn GetSnapshotDeviceName();
    fn GetCurrentSnapshotSetId();
    fn GetContext();
    fn GetCurrentLevel();
    fn IsPathAffected();
    fn IsBootableSystemStateBackedUp();
    fn AreComponentsSelected();
    fn GetBackupType();
    fn GetRestoreType();
    fn SetWriterFailure();
    fn IsPartialFileSupportEnabled();
    fn InstallAlternateWriter();
    fn GetIdentityInformation();
    fn SetWriterFailureEx();
    fn GetSessionId();
    fn IsWriterShuttingDown();
}

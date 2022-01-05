pub trait IEnumOfflineFilesItemsImpl: Sized {
    fn Next();
    fn Skip();
    fn Reset();
    fn Clone();
}
pub trait IEnumOfflineFilesSettingsImpl: Sized {
    fn Next();
    fn Skip();
    fn Reset();
    fn Clone();
}
pub trait IOfflineFilesCacheImpl: Sized {
    fn Synchronize();
    fn DeleteItems();
    fn DeleteItemsForUser();
    fn Pin();
    fn Unpin();
    fn GetEncryptionStatus();
    fn Encrypt();
    fn FindItem();
    fn FindItemEx();
    fn RenameItem();
    fn GetLocation();
    fn GetDiskSpaceInformation();
    fn SetDiskSpaceLimits();
    fn ProcessAdminPinPolicy();
    fn GetSettingObject();
    fn EnumSettingObjects();
    fn IsPathCacheable();
}
pub trait IOfflineFilesCache2Impl: Sized + IOfflineFilesCacheImpl {
    fn RenameItemEx();
}
pub trait IOfflineFilesChangeInfoImpl: Sized {
    fn IsDirty();
    fn IsDeletedOffline();
    fn IsCreatedOffline();
    fn IsLocallyModifiedData();
    fn IsLocallyModifiedAttributes();
    fn IsLocallyModifiedTime();
}
pub trait IOfflineFilesConnectionInfoImpl: Sized {
    fn GetConnectState();
    fn SetConnectState();
    fn TransitionOnline();
    fn TransitionOffline();
}
pub trait IOfflineFilesDirectoryItemImpl: Sized + IOfflineFilesItemImpl {}
pub trait IOfflineFilesDirtyInfoImpl: Sized {
    fn LocalDirtyByteCount();
    fn RemoteDirtyByteCount();
}
pub trait IOfflineFilesErrorInfoImpl: Sized {
    fn GetRawData();
    fn GetDescription();
}
pub trait IOfflineFilesEventsImpl: Sized {
    fn CacheMoved();
    fn CacheIsFull();
    fn CacheIsCorrupted();
    fn Enabled();
    fn EncryptionChanged();
    fn SyncBegin();
    fn SyncFileResult();
    fn SyncConflictRecAdded();
    fn SyncConflictRecUpdated();
    fn SyncConflictRecRemoved();
    fn SyncEnd();
    fn NetTransportArrived();
    fn NoNetTransports();
    fn ItemDisconnected();
    fn ItemReconnected();
    fn ItemAvailableOffline();
    fn ItemNotAvailableOffline();
    fn ItemPinned();
    fn ItemNotPinned();
    fn ItemModified();
    fn ItemAddedToCache();
    fn ItemDeletedFromCache();
    fn ItemRenamed();
    fn DataLost();
    fn Ping();
}
pub trait IOfflineFilesEvents2Impl: Sized + IOfflineFilesEventsImpl {
    fn ItemReconnectBegin();
    fn ItemReconnectEnd();
    fn CacheEvictBegin();
    fn CacheEvictEnd();
    fn BackgroundSyncBegin();
    fn BackgroundSyncEnd();
    fn PolicyChangeDetected();
    fn PreferenceChangeDetected();
    fn SettingsChangesApplied();
}
pub trait IOfflineFilesEvents3Impl: Sized + IOfflineFilesEvents2Impl + IOfflineFilesEventsImpl {
    fn TransparentCacheItemNotify();
    fn PrefetchFileBegin();
    fn PrefetchFileEnd();
}
pub trait IOfflineFilesEvents4Impl: Sized + IOfflineFilesEvents3Impl + IOfflineFilesEvents2Impl + IOfflineFilesEventsImpl {
    fn PrefetchCloseHandleBegin();
    fn PrefetchCloseHandleEnd();
}
pub trait IOfflineFilesEventsFilterImpl: Sized {
    fn GetPathFilter();
    fn GetIncludedEvents();
    fn GetExcludedEvents();
}
pub trait IOfflineFilesFileItemImpl: Sized + IOfflineFilesItemImpl {
    fn IsSparse();
    fn IsEncrypted();
}
pub trait IOfflineFilesFileSysInfoImpl: Sized {
    fn GetAttributes();
    fn GetTimes();
    fn GetFileSize();
}
pub trait IOfflineFilesGhostInfoImpl: Sized {
    fn IsGhosted();
}
pub trait IOfflineFilesItemImpl: Sized {
    fn GetItemType();
    fn GetPath();
    fn GetParentItem();
    fn Refresh();
    fn IsMarkedForDeletion();
}
pub trait IOfflineFilesItemContainerImpl: Sized {
    fn EnumItems();
    fn EnumItemsEx();
}
pub trait IOfflineFilesItemFilterImpl: Sized {
    fn GetFilterFlags();
    fn GetTimeFilter();
    fn GetPatternFilter();
}
pub trait IOfflineFilesPinInfoImpl: Sized {
    fn IsPinned();
    fn IsPinnedForUser();
    fn IsPinnedForUserByPolicy();
    fn IsPinnedForComputer();
    fn IsPinnedForFolderRedirection();
}
pub trait IOfflineFilesPinInfo2Impl: Sized + IOfflineFilesPinInfoImpl {
    fn IsPartlyPinned();
}
pub trait IOfflineFilesProgressImpl: Sized {
    fn Begin();
    fn QueryAbort();
    fn End();
}
pub trait IOfflineFilesServerItemImpl: Sized + IOfflineFilesItemImpl {}
pub trait IOfflineFilesSettingImpl: Sized {
    fn GetName();
    fn GetValueType();
    fn GetPreference();
    fn GetPreferenceScope();
    fn SetPreference();
    fn DeletePreference();
    fn GetPolicy();
    fn GetPolicyScope();
    fn GetValue();
}
pub trait IOfflineFilesShareInfoImpl: Sized {
    fn GetShareItem();
    fn GetShareCachingMode();
    fn IsShareDfsJunction();
}
pub trait IOfflineFilesShareItemImpl: Sized + IOfflineFilesItemImpl {}
pub trait IOfflineFilesSimpleProgressImpl: Sized + IOfflineFilesProgressImpl {
    fn ItemBegin();
    fn ItemResult();
}
pub trait IOfflineFilesSuspendImpl: Sized {
    fn SuspendRoot();
}
pub trait IOfflineFilesSuspendInfoImpl: Sized {
    fn IsSuspended();
}
pub trait IOfflineFilesSyncConflictHandlerImpl: Sized {
    fn ResolveConflict();
}
pub trait IOfflineFilesSyncErrorInfoImpl: Sized + IOfflineFilesErrorInfoImpl {
    fn GetSyncOperation();
    fn GetItemChangeFlags();
    fn InfoEnumerated();
    fn InfoAvailable();
    fn GetLocalInfo();
    fn GetRemoteInfo();
    fn GetOriginalInfo();
}
pub trait IOfflineFilesSyncErrorItemInfoImpl: Sized {
    fn GetFileAttributes();
    fn GetFileTimes();
    fn GetFileSize();
}
pub trait IOfflineFilesSyncProgressImpl: Sized + IOfflineFilesProgressImpl {
    fn SyncItemBegin();
    fn SyncItemResult();
}
pub trait IOfflineFilesTransparentCacheInfoImpl: Sized {
    fn IsTransparentlyCached();
}

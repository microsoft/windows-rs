void __stdcall CcAsyncCopyRead(int p0, int p1, int p2, int p3, int p4, int p5, int p6, int p7) {}
void __stdcall CcCanIWrite(int p0, int p1, int p2, int p3) {}
void __stdcall CcCoherencyFlushAndPurgeCache(int p0, int p1, int p2, int p3, int p4) {}
void __stdcall CcCopyRead(int p0, int p1, int p2, int p3, int p4, int p5) {}
void __stdcall CcCopyReadEx(int p0, int p1, int p2, int p3, int p4, int p5, int p6) {}
void __stdcall CcCopyWrite(int p0, int p1, int p2, int p3, int p4) {}
void __stdcall CcCopyWriteEx(int p0, int p1, int p2, int p3, int p4, int p5) {}
void __stdcall CcCopyWriteWontFlush(int p0, int p1, int p2) {}
void __stdcall CcDeferWrite(int p0, int p1, int p2, int p3, int p4, int p5) {}
void __stdcall CcErrorCallbackRoutine(int p0) {}
void __stdcall CcFastCopyRead(int p0, int p1, int p2, int p3, int p4, int p5) {}
void __stdcall CcFastCopyWrite(int p0, int p1, int p2, int p3) {}
void __stdcall CcFlushCache(int p0, int p1, int p2, int p3) {}
void __stdcall CcGetDirtyPages(int p0, int p1, int p2, int p3) {}
void __stdcall CcGetFileObjectFromBcb(int p0) {}
void __stdcall CcGetFileObjectFromSectionPtrs(int p0) {}
void __stdcall CcGetFileObjectFromSectionPtrsRef(int p0) {}
void __stdcall CcGetFlushedValidData(int p0, int p1) {}
void __stdcall CcInitializeCacheMap(int p0, int p1, int p2, int p3, int p4) {}
void __stdcall CcInitializeCacheMapEx(int p0, int p1, int p2, int p3, int p4, int p5) {}
void __stdcall CcIsCacheManagerCallbackNeeded(int p0) {}
void __stdcall CcIsThereDirtyData(int p0) {}
void __stdcall CcIsThereDirtyDataEx(int p0, int p1) {}
void __stdcall CcMapData(int p0, int p1, int p2, int p3, int p4, int p5) {}
void __stdcall CcMdlRead(int p0, int p1, int p2, int p3, int p4) {}
void __stdcall CcMdlReadComplete(int p0, int p1) {}
void __stdcall CcMdlWriteAbort(int p0, int p1) {}
void __stdcall CcMdlWriteComplete(int p0, int p1, int p2) {}
void __stdcall CcPinMappedData(int p0, int p1, int p2, int p3, int p4) {}
void __stdcall CcPinRead(int p0, int p1, int p2, int p3, int p4, int p5) {}
void __stdcall CcPrepareMdlWrite(int p0, int p1, int p2, int p3, int p4) {}
void __stdcall CcPreparePinWrite(int p0, int p1, int p2, int p3, int p4, int p5, int p6) {}
void __stdcall CcPurgeCacheSection(int p0, int p1, int p2, int p3) {}
void __stdcall CcRemapBcb(int p0) {}
void __stdcall CcRepinBcb(int p0) {}
void __stdcall CcScheduleReadAhead(int p0, int p1, int p2) {}
void __stdcall CcScheduleReadAheadEx(int p0, int p1, int p2, int p3) {}
void __stdcall CcSetAdditionalCacheAttributes(int p0, int p1, int p2) {}
void __stdcall CcSetAdditionalCacheAttributesEx(int p0, int p1) {}
void __stdcall CcSetBcbOwnerPointer(int p0, int p1) {}
void __stdcall CcSetDirtyPageThreshold(int p0, int p1) {}
void __stdcall CcSetDirtyPinnedData(int p0, int p1) {}
void __stdcall CcSetFileSizes(int p0, int p1) {}
void __stdcall CcSetFileSizesEx(int p0, int p1) {}
void __stdcall CcSetLogHandleForFile(int p0, int p1, int p2) {}
void __stdcall CcSetParallelFlushFile(int p0, int p1) {}
void __stdcall CcSetReadAheadGranularity(int p0, int p1) {}
void __stdcall CcUninitializeCacheMap(int p0, int p1, int p2) {}
void __stdcall CcUnpinData(int p0) {}
void __stdcall CcUnpinDataForThread(int p0, int p1) {}
void __stdcall CcUnpinRepinnedBcb(int p0, int p1, int p2) {}
void __stdcall CcWaitForCurrentLazyWriterActivity() {}
void __stdcall CcZeroData(int p0, int p1, int p2, int p3) {}
void __stdcall CmCallbackGetKeyObjectID(int p0, int p1, int p2, int p3) {}
void __stdcall CmCallbackGetKeyObjectIDEx(int p0, int p1, int p2, int p3, int p4) {}
void __stdcall CmCallbackReleaseKeyObjectIDEx(int p0) {}
void __stdcall CmGetBoundTransaction(int p0, int p1) {}
void __stdcall CmGetCallbackVersion(int p0, int p1) {}
void __stdcall CmRegisterCallback(int p0, int p1, int p2) {}
void __stdcall CmRegisterCallbackEx(int p0, int p1, int p2, int p3, int p4, int p5) {}
void __stdcall CmSetCallbackObjectContext(int p0, int p1, int p2, int p3) {}
void __stdcall CmUnRegisterCallback(int p0, int p1) {}
void __stdcall DbgBreakPointWithStatus(int p0) {}
void __stdcall DbgSetDebugPrintCallback(int p0, int p1) {}
void __stdcall EtwActivityIdControl(int p0, int p1) {}
void __stdcall EtwProviderEnabled(int p0, int p1, int p2, int p3, int p4) {}
void __stdcall EtwRegister(int p0, int p1, int p2, int p3) {}
void __stdcall EtwSetInformation(int p0, int p1, int p2, int p3, int p4) {}
void __stdcall EtwUnregister(int p0, int p1) {}
void __stdcall EtwWrite(int p0, int p1, int p2, int p3, int p4, int p5) {}
void __stdcall EtwWriteEx(int p0, int p1, int p2, int p3, int p4, int p5, int p6, int p7, int p8, int p9) {}
void __stdcall EtwWriteString(int p0, int p1, int p2, int p3, int p4, int p5, int p6) {}
void __stdcall EtwWriteTransfer(int p0, int p1, int p2, int p3, int p4, int p5, int p6) {}
void __stdcall ExAcquireFastMutex(int p0) {}
void __stdcall ExAcquireFastMutexUnsafe(int p0) {}
void __stdcall ExAcquirePushLockExclusiveEx(int p0, int p1) {}
void __stdcall ExAcquirePushLockSharedEx(int p0, int p1) {}
void __stdcall ExAcquireResourceExclusiveLite(int p0, int p1) {}
void __stdcall ExAcquireResourceSharedLite(int p0, int p1) {}
void __stdcall ExAcquireRundownProtection(int p0) {}
void __stdcall ExAcquireRundownProtectionCacheAware(int p0) {}
void __stdcall ExAcquireRundownProtectionCacheAwareEx(int p0, int p1) {}
void __stdcall ExAcquireRundownProtectionEx(int p0, int p1) {}
void __stdcall ExAcquireSharedStarveExclusive(int p0, int p1) {}
void __stdcall ExAcquireSharedWaitForExclusive(int p0, int p1) {}
void __stdcall ExAcquireSpinLockExclusive(int p0) {}
void __stdcall ExAcquireSpinLockExclusiveAtDpcLevel(int p0) {}
void __stdcall ExAcquireSpinLockShared(int p0) {}
void __stdcall ExAcquireSpinLockSharedAtDpcLevel(int p0) {}
void __stdcall ExAllocateCacheAwareRundownProtection(int p0, int p1) {}
void __stdcall ExAllocatePool(int p0, int p1) {}
void __stdcall ExAllocatePool2(int p0, int p1, int p2, int p3) {}
void __stdcall ExAllocatePool3(int p0, int p1, int p2, int p3, int p4, int p5) {}
void __stdcall ExAllocatePoolWithQuota(int p0, int p1) {}
void __stdcall ExAllocatePoolWithQuotaTag(int p0, int p1, int p2) {}
void __stdcall ExAllocatePoolWithTag(int p0, int p1, int p2) {}
void __stdcall ExAllocatePoolWithTagPriority(int p0, int p1, int p2, int p3) {}
void __stdcall ExAllocateTimer(int p0, int p1, int p2) {}
void __stdcall ExCancelTimer(int p0, int p1) {}
void __stdcall ExCleanupRundownProtectionCacheAware(int p0) {}
void __stdcall ExConvertExclusiveToSharedLite(int p0) {}
void __stdcall ExCreateCallback(int p0, int p1, int p2, int p3) {}
void __stdcall ExCreatePool(int p0, int p1, int p2, int p3) {}
void __stdcall ExDeleteResourceLite(int p0) {}
void __stdcall ExDeleteTimer(int p0, int p1, int p2, int p3) {}
void __stdcall ExDestroyPool(int p0) {}
void __stdcall ExDisableResourceBoostLite(int p0) {}
void __stdcall ExEnterCriticalRegionAndAcquireResourceExclusive(int p0) {}
void __stdcall ExEnterCriticalRegionAndAcquireResourceShared(int p0) {}
void __stdcall ExEnterCriticalRegionAndAcquireSharedWaitForExclusive(int p0) {}
void __stdcall ExEnumerateSystemFirmwareTables(int p0, int p1, int p2, int p3) {}
void __stdcall ExExtendZone(int p0, int p1, int p2) {}
void __stdcall ExFreeCacheAwareRundownProtection(int p0) {}
void __stdcall ExFreePool(int p0) {}
void __stdcall ExFreePool2(int p0, int p1, int p2, int p3) {}
void __stdcall ExFreePoolWithTag(int p0, int p1) {}
void __stdcall ExGetExclusiveWaiterCount(int p0) {}
void __stdcall ExGetFirmwareEnvironmentVariable(int p0, int p1, int p2, int p3, int p4) {}
void __stdcall ExGetFirmwareType() {}
void __stdcall ExGetPreviousMode() {}
void __stdcall ExGetSharedWaiterCount(int p0) {}
void __stdcall ExGetSystemFirmwareTable(int p0, int p1, int p2, int p3, int p4) {}
void __stdcall ExInitializePushLock(int p0) {}
void __stdcall ExInitializeResourceLite(int p0) {}
void __stdcall ExInitializeRundownProtection(int p0) {}
void __stdcall ExInitializeRundownProtectionCacheAware(int p0, int p1) {}
void __stdcall ExInitializeRundownProtectionCacheAwareEx(int p0, int p1) {}
void __stdcall ExInitializeZone(int p0, int p1, int p2, int p3) {}
void __stdcall ExInterlockedAddLargeInteger(int p0, int p1, int p2, int p3) {}
void __stdcall ExInterlockedExtendZone(int p0, int p1, int p2, int p3) {}
void __stdcall ExIsManufacturingModeEnabled() {}
void __stdcall ExIsProcessorFeaturePresent(int p0) {}
void __stdcall ExIsResourceAcquiredExclusiveLite(int p0) {}
void __stdcall ExIsResourceAcquiredSharedLite(int p0) {}
void __stdcall ExIsSoftBoot() {}
void __stdcall ExLocalTimeToSystemTime(int p0, int p1) {}
void __stdcall ExNotifyCallback(int p0, int p1, int p2) {}
void __stdcall ExQueryPoolBlockSize(int p0, int p1) {}
void __stdcall ExQueryTimerResolution(int p0, int p1, int p2) {}
void __stdcall ExQueueWorkItem(int p0, int p1) {}
void __stdcall ExRaiseAccessViolation() {}
void __stdcall ExRaiseDatatypeMisalignment() {}
void __stdcall ExRaiseStatus(int p0) {}
void __stdcall ExReInitializeRundownProtection(int p0) {}
void __stdcall ExReInitializeRundownProtectionCacheAware(int p0) {}
void __stdcall ExRegisterCallback(int p0, int p1, int p2) {}
void __stdcall ExReinitializeResourceLite(int p0) {}
void __stdcall ExReleaseFastMutex(int p0) {}
void __stdcall ExReleaseFastMutexUnsafe(int p0) {}
void __stdcall ExReleasePushLockExclusiveEx(int p0, int p1) {}
void __stdcall ExReleasePushLockSharedEx(int p0, int p1) {}
void __stdcall ExReleaseResourceAndLeaveCriticalRegion(int p0) {}
void __stdcall ExReleaseResourceForThreadLite(int p0, int p1) {}
void __stdcall ExReleaseResourceLite(int p0) {}
void __stdcall ExReleaseRundownProtection(int p0) {}
void __stdcall ExReleaseRundownProtectionCacheAware(int p0) {}
void __stdcall ExReleaseRundownProtectionCacheAwareEx(int p0, int p1) {}
void __stdcall ExReleaseRundownProtectionEx(int p0, int p1) {}
void __stdcall ExReleaseSpinLockExclusive(int p0, int p1) {}
void __stdcall ExReleaseSpinLockExclusiveFromDpcLevel(int p0) {}
void __stdcall ExReleaseSpinLockShared(int p0, int p1) {}
void __stdcall ExReleaseSpinLockSharedFromDpcLevel(int p0) {}
void __stdcall ExRundownCompleted(int p0) {}
void __stdcall ExRundownCompletedCacheAware(int p0) {}
void __stdcall ExSecurePoolUpdate(int p0, int p1, int p2, int p3, int p4, int p5, int p6) {}
void __stdcall ExSecurePoolValidate(int p0, int p1, int p2, int p3) {}
void __stdcall ExSetFirmwareEnvironmentVariable(int p0, int p1, int p2, int p3, int p4) {}
void __stdcall ExSetResourceOwnerPointer(int p0, int p1) {}
void __stdcall ExSetResourceOwnerPointerEx(int p0, int p1, int p2) {}
void __stdcall ExSetTimer(int p0, int p1, int p2, int p3, int p4, int p5) {}
void __stdcall ExSetTimerResolution(int p0, int p1) {}
void __stdcall ExSizeOfRundownProtectionCacheAware() {}
void __stdcall ExSystemTimeToLocalTime(int p0, int p1) {}
void __stdcall ExTryAcquireSpinLockExclusiveAtDpcLevel(int p0) {}
void __stdcall ExTryAcquireSpinLockSharedAtDpcLevel(int p0) {}
void __stdcall ExTryConvertSharedSpinLockExclusive(int p0) {}
void __stdcall ExTryToAcquireFastMutex(int p0) {}
void __stdcall ExUnregisterCallback(int p0) {}
void __stdcall ExUuidCreate(int p0) {}
void __stdcall ExVerifySuite(int p0) {}
void __stdcall ExWaitForRundownProtectionRelease(int p0) {}
void __stdcall ExWaitForRundownProtectionReleaseCacheAware(int p0) {}
void __stdcall FsRtlAcknowledgeEcp(int p0) {}
void __stdcall FsRtlAcquireFileExclusive(int p0) {}
void __stdcall FsRtlAddBaseMcbEntry(int p0, int p1, int p2, int p3, int p4, int p5, int p6) {}
void __stdcall FsRtlAddBaseMcbEntryEx(int p0, int p1, int p2, int p3, int p4, int p5, int p6) {}
void __stdcall FsRtlAddLargeMcbEntry(int p0, int p1, int p2, int p3, int p4, int p5, int p6) {}
void __stdcall FsRtlAddMcbEntry(int p0, int p1, int p2, int p3) {}
void __stdcall FsRtlAddToTunnelCache(int p0, int p1, int p2, int p3, int p4, int p5, int p6, int p7) {}
void __stdcall FsRtlAddToTunnelCacheEx(int p0, int p1, int p2, int p3, int p4, int p5, int p6, int p7) {}
void __stdcall FsRtlAllocateAePushLock(int p0, int p1) {}
void __stdcall FsRtlAllocateExtraCreateParameter(int p0, int p1, int p2, int p3, int p4, int p5) {}
void __stdcall FsRtlAllocateExtraCreateParameterFromLookasideList(int p0, int p1, int p2, int p3, int p4, int p5) {}
void __stdcall FsRtlAllocateExtraCreateParameterList(int p0, int p1) {}
void __stdcall FsRtlAllocateFileLock(int p0, int p1) {}
void __stdcall FsRtlAllocateResource() {}
void __stdcall FsRtlAreNamesEqual(int p0, int p1, int p2, int p3) {}
void __stdcall FsRtlAreThereCurrentOrInProgressFileLocks(int p0) {}
void __stdcall FsRtlAreThereWaitingFileLocks(int p0) {}
void __stdcall FsRtlAreVolumeStartupApplicationsComplete() {}
void __stdcall FsRtlBalanceReads(int p0) {}
void __stdcall FsRtlCancellableWaitForMultipleObjects(int p0, int p1, int p2, int p3, int p4, int p5) {}
void __stdcall FsRtlCancellableWaitForSingleObject(int p0, int p1, int p2) {}
void __stdcall FsRtlChangeBackingFileObject(int p0, int p1, int p2, int p3) {}
void __stdcall FsRtlCheckLockForOplockRequest(int p0, int p1) {}
void __stdcall FsRtlCheckLockForReadAccess(int p0, int p1) {}
void __stdcall FsRtlCheckLockForWriteAccess(int p0, int p1) {}
void __stdcall FsRtlCheckOplock(int p0, int p1, int p2, int p3, int p4) {}
void __stdcall FsRtlCheckOplockEx(int p0, int p1, int p2, int p3, int p4, int p5) {}
void __stdcall FsRtlCheckOplockEx2(int p0, int p1, int p2, int p3, int p4, int p5, int p6, int p7, int p8, int p9, int p10) {}
void __stdcall FsRtlCheckUpperOplock(int p0, int p1, int p2, int p3, int p4, int p5) {}
void __stdcall FsRtlCopyRead(int p0, int p1, int p2, int p3, int p4, int p5, int p6, int p7) {}
void __stdcall FsRtlCopyWrite(int p0, int p1, int p2, int p3, int p4, int p5, int p6, int p7) {}
void __stdcall FsRtlCreateSectionForDataScan(int p0, int p1, int p2, int p3, int p4, int p5, int p6, int p7, int p8, int p9) {}
void __stdcall FsRtlCurrentBatchOplock(int p0) {}
void __stdcall FsRtlCurrentOplock(int p0) {}
void __stdcall FsRtlCurrentOplockH(int p0) {}
void __stdcall FsRtlDeleteExtraCreateParameterLookasideList(int p0, int p1) {}
void __stdcall FsRtlDeleteKeyFromTunnelCache(int p0, int p1, int p2) {}
void __stdcall FsRtlDeleteTunnelCache(int p0) {}
void __stdcall FsRtlDeregisterUncProvider(int p0) {}
void __stdcall FsRtlDismountComplete(int p0, int p1) {}
void __stdcall FsRtlDissectDbcs(int p0, int p1, int p2, int p3) {}
void __stdcall FsRtlDissectName(int p0, int p1, int p2, int p3) {}
void __stdcall FsRtlDoesDbcsContainWildCards(int p0) {}
void __stdcall FsRtlDoesNameContainWildCards(int p0) {}
void __stdcall FsRtlFastCheckLockForRead(int p0, int p1, int p2, int p3, int p4, int p5) {}
void __stdcall FsRtlFastCheckLockForWrite(int p0, int p1, int p2, int p3, int p4, int p5) {}
void __stdcall FsRtlFastUnlockAll(int p0, int p1, int p2, int p3) {}
void __stdcall FsRtlFastUnlockAllByKey(int p0, int p1, int p2, int p3, int p4) {}
void __stdcall FsRtlFastUnlockSingle(int p0, int p1, int p2, int p3, int p4, int p5, int p6, int p7) {}
void __stdcall FsRtlFindExtraCreateParameter(int p0, int p1, int p2, int p3) {}
void __stdcall FsRtlFindInTunnelCache(int p0, int p1, int p2, int p3, int p4, int p5, int p6, int p7) {}
void __stdcall FsRtlFindInTunnelCacheEx(int p0, int p1, int p2, int p3, int p4, int p5, int p6, int p7, int p8) {}
void __stdcall FsRtlFreeAePushLock(int p0) {}
void __stdcall FsRtlFreeExtraCreateParameter(int p0) {}
void __stdcall FsRtlFreeExtraCreateParameterList(int p0) {}
void __stdcall FsRtlFreeFileLock(int p0) {}
void __stdcall FsRtlGetCurrentProcessLoaderList() {}
void __stdcall FsRtlGetEcpListFromIrp(int p0, int p1) {}
void __stdcall FsRtlGetFileSize(int p0, int p1) {}
void __stdcall FsRtlGetNextBaseMcbEntry(int p0, int p1, int p2, int p3, int p4) {}
void __stdcall FsRtlGetNextExtraCreateParameter(int p0, int p1, int p2, int p3, int p4) {}
void __stdcall FsRtlGetNextFileLock(int p0, int p1) {}
void __stdcall FsRtlGetNextLargeMcbEntry(int p0, int p1, int p2, int p3, int p4) {}
void __stdcall FsRtlGetNextMcbEntry(int p0, int p1, int p2, int p3, int p4) {}
void __stdcall FsRtlGetSectorSizeInformation(int p0, int p1) {}
void __stdcall FsRtlGetSupportedFeatures(int p0, int p1) {}
void __stdcall FsRtlGetVirtualDiskNestingLevel(int p0, int p1, int p2) {}
void __stdcall FsRtlIncrementCcFastMdlReadWait() {}
void __stdcall FsRtlIncrementCcFastReadNoWait() {}
void __stdcall FsRtlIncrementCcFastReadNotPossible() {}
void __stdcall FsRtlIncrementCcFastReadResourceMiss() {}
void __stdcall FsRtlIncrementCcFastReadWait() {}
void __stdcall FsRtlInitExtraCreateParameterLookasideList(int p0, int p1, int p2, int p3) {}
void __stdcall FsRtlInitializeBaseMcb(int p0, int p1) {}
void __stdcall FsRtlInitializeBaseMcbEx(int p0, int p1, int p2) {}
void __stdcall FsRtlInitializeExtraCreateParameter(int p0, int p1, int p2, int p3, int p4, int p5) {}
void __stdcall FsRtlInitializeExtraCreateParameterList(int p0) {}
void __stdcall FsRtlInitializeFileLock(int p0, int p1, int p2) {}
void __stdcall FsRtlInitializeLargeMcb(int p0, int p1) {}
void __stdcall FsRtlInitializeMcb(int p0, int p1) {}
void __stdcall FsRtlInitializeOplock(int p0) {}
void __stdcall FsRtlInitializeTunnelCache(int p0) {}
void __stdcall FsRtlInsertExtraCreateParameter(int p0, int p1) {}
void __stdcall FsRtlInsertPerFileContext(int p0, int p1) {}
void __stdcall FsRtlInsertPerFileObjectContext(int p0, int p1) {}
void __stdcall FsRtlInsertPerStreamContext(int p0, int p1) {}
void __stdcall FsRtlIs32BitProcess(int p0) {}
void __stdcall FsRtlIsDaxVolume(int p0) {}
void __stdcall FsRtlIsDbcsInExpression(int p0, int p1) {}
void __stdcall FsRtlIsEcpAcknowledged(int p0) {}
void __stdcall FsRtlIsEcpFromUserMode(int p0) {}
void __stdcall FsRtlIsExtentDangling(int p0, int p1, int p2) {}
void __stdcall FsRtlIsFatDbcsLegal(int p0, int p1, int p2, int p3, int p4) {}
void __stdcall FsRtlIsHpfsDbcsLegal(int p0, int p1, int p2, int p3, int p4) {}
void __stdcall FsRtlIsMobileOS() {}
void __stdcall FsRtlIsNameInExpression(int p0, int p1, int p2, int p3) {}
void __stdcall FsRtlIsNameInUnUpcasedExpression(int p0, int p1, int p2, int p3) {}
void __stdcall FsRtlIsNonEmptyDirectoryReparsePointAllowed(int p0) {}
void __stdcall FsRtlIsNtstatusExpected(int p0) {}
void __stdcall FsRtlIsPagingFile(int p0) {}
void __stdcall FsRtlIsSystemPagingFile(int p0) {}
void __stdcall FsRtlIsTotalDeviceFailure(int p0) {}
void __stdcall FsRtlIssueDeviceIoControl(int p0, int p1, int p2, int p3, int p4, int p5, int p6, int p7) {}
void __stdcall FsRtlKernelFsControlFile(int p0, int p1, int p2, int p3, int p4, int p5, int p6) {}
void __stdcall FsRtlLogCcFlushError(int p0, int p1, int p2, int p3, int p4) {}
void __stdcall FsRtlLookupBaseMcbEntry(int p0, int p1, int p2, int p3, int p4, int p5, int p6, int p7) {}
void __stdcall FsRtlLookupLargeMcbEntry(int p0, int p1, int p2, int p3, int p4, int p5, int p6, int p7) {}
void __stdcall FsRtlLookupLastBaseMcbEntry(int p0, int p1, int p2) {}
void __stdcall FsRtlLookupLastBaseMcbEntryAndIndex(int p0, int p1, int p2, int p3) {}
void __stdcall FsRtlLookupLastLargeMcbEntry(int p0, int p1, int p2) {}
void __stdcall FsRtlLookupLastLargeMcbEntryAndIndex(int p0, int p1, int p2, int p3) {}
void __stdcall FsRtlLookupLastMcbEntry(int p0, int p1, int p2) {}
void __stdcall FsRtlLookupMcbEntry(int p0, int p1, int p2, int p3, int p4) {}
void __stdcall FsRtlLookupPerFileContext(int p0, int p1, int p2) {}
void __stdcall FsRtlLookupPerFileObjectContext(int p0, int p1, int p2) {}
void __stdcall FsRtlLookupPerStreamContextInternal(int p0, int p1, int p2) {}
void __stdcall FsRtlMdlReadCompleteDev(int p0, int p1, int p2) {}
void __stdcall FsRtlMdlReadDev(int p0, int p1, int p2, int p3, int p4, int p5, int p6) {}
void __stdcall FsRtlMdlReadEx(int p0, int p1, int p2, int p3, int p4, int p5) {}
void __stdcall FsRtlMdlWriteCompleteDev(int p0, int p1, int p2, int p3) {}
void __stdcall FsRtlMupGetProviderIdFromName(int p0, int p1) {}
void __stdcall FsRtlMupGetProviderInfoFromFileObject(int p0, int p1, int p2, int p3) {}
void __stdcall FsRtlNormalizeNtstatus(int p0, int p1) {}
void __stdcall FsRtlNotifyCleanup(int p0, int p1, int p2) {}
void __stdcall FsRtlNotifyCleanupAll(int p0, int p1) {}
void __stdcall FsRtlNotifyFilterChangeDirectory(int p0, int p1, int p2, int p3, int p4, int p5, int p6, int p7, int p8, int p9, int p10) {}
void __stdcall FsRtlNotifyFilterReportChange(int p0, int p1, int p2, int p3, int p4, int p5, int p6, int p7, int p8, int p9) {}
void __stdcall FsRtlNotifyFullChangeDirectory(int p0, int p1, int p2, int p3, int p4, int p5, int p6, int p7, int p8, int p9) {}
void __stdcall FsRtlNotifyFullReportChange(int p0, int p1, int p2, int p3, int p4, int p5, int p6, int p7, int p8) {}
void __stdcall FsRtlNotifyInitializeSync(int p0) {}
void __stdcall FsRtlNotifyUninitializeSync(int p0) {}
void __stdcall FsRtlNotifyVolumeEvent(int p0, int p1) {}
void __stdcall FsRtlNotifyVolumeEventEx(int p0, int p1, int p2) {}
void __stdcall FsRtlNumberOfRunsInBaseMcb(int p0) {}
void __stdcall FsRtlNumberOfRunsInLargeMcb(int p0) {}
void __stdcall FsRtlNumberOfRunsInMcb(int p0) {}
void __stdcall FsRtlOplockBreakH(int p0, int p1, int p2, int p3, int p4, int p5) {}
void __stdcall FsRtlOplockBreakH2(int p0, int p1, int p2, int p3, int p4, int p5, int p6, int p7) {}
void __stdcall FsRtlOplockBreakToNone(int p0, int p1, int p2, int p3, int p4, int p5) {}
void __stdcall FsRtlOplockBreakToNoneEx(int p0, int p1, int p2, int p3, int p4, int p5) {}
void __stdcall FsRtlOplockFsctrl(int p0, int p1, int p2) {}
void __stdcall FsRtlOplockFsctrlEx(int p0, int p1, int p2, int p3) {}
void __stdcall FsRtlOplockGetAnyBreakOwnerProcess(int p0) {}
void __stdcall FsRtlOplockIsFastIoPossible(int p0) {}
void __stdcall FsRtlOplockIsSharedRequest(int p0) {}
void __stdcall FsRtlOplockKeysEqual(int p0, int p1) {}
void __stdcall FsRtlPostPagingFileStackOverflow(int p0, int p1, int p2) {}
void __stdcall FsRtlPostStackOverflow(int p0, int p1, int p2) {}
void __stdcall FsRtlPrepareMdlWriteDev(int p0, int p1, int p2, int p3, int p4, int p5, int p6) {}
void __stdcall FsRtlPrepareMdlWriteEx(int p0, int p1, int p2, int p3, int p4, int p5) {}
void __stdcall FsRtlPrepareToReuseEcp(int p0) {}
void __stdcall FsRtlPrivateLock(int p0, int p1, int p2, int p3, int p4, int p5, int p6, int p7, int p8, int p9, int p10, int p11) {}
void __stdcall FsRtlProcessFileLock(int p0, int p1, int p2) {}
void __stdcall FsRtlQueryCachedVdl(int p0, int p1) {}
void __stdcall FsRtlQueryInformationFile(int p0, int p1, int p2, int p3, int p4) {}
void __stdcall FsRtlQueryKernelEaFile(int p0, int p1, int p2, int p3, int p4, int p5, int p6, int p7, int p8) {}
void __stdcall FsRtlQueryMaximumVirtualDiskNestingLevel() {}
void __stdcall FsRtlRegisterFileSystemFilterCallbacks(int p0, int p1) {}
void __stdcall FsRtlRegisterUncProvider(int p0, int p1, int p2) {}
void __stdcall FsRtlRegisterUncProviderEx(int p0, int p1, int p2, int p3) {}
void __stdcall FsRtlRegisterUncProviderEx2(int p0, int p1, int p2, int p3) {}
void __stdcall FsRtlReleaseFile(int p0) {}
void __stdcall FsRtlRemoveBaseMcbEntry(int p0, int p1, int p2, int p3, int p4) {}
void __stdcall FsRtlRemoveDotsFromPath(int p0, int p1, int p2) {}
void __stdcall FsRtlRemoveExtraCreateParameter(int p0, int p1, int p2, int p3) {}
void __stdcall FsRtlRemoveLargeMcbEntry(int p0, int p1, int p2, int p3, int p4) {}
void __stdcall FsRtlRemoveMcbEntry(int p0, int p1, int p2) {}
void __stdcall FsRtlRemovePerFileContext(int p0, int p1, int p2) {}
void __stdcall FsRtlRemovePerFileObjectContext(int p0, int p1, int p2) {}
void __stdcall FsRtlRemovePerStreamContext(int p0, int p1, int p2) {}
void __stdcall FsRtlResetBaseMcb(int p0) {}
void __stdcall FsRtlResetLargeMcb(int p0, int p1) {}
void __stdcall FsRtlSetDriverBacking(int p0, int p1) {}
void __stdcall FsRtlSetEcpListIntoIrp(int p0, int p1) {}
void __stdcall FsRtlSetKernelEaFile(int p0, int p1, int p2) {}
void __stdcall FsRtlSplitBaseMcb(int p0, int p1, int p2, int p3, int p4) {}
void __stdcall FsRtlSplitLargeMcb(int p0, int p1, int p2, int p3, int p4) {}
void __stdcall FsRtlTeardownPerFileContexts(int p0) {}
void __stdcall FsRtlTeardownPerStreamContexts(int p0) {}
void __stdcall FsRtlTruncateBaseMcb(int p0, int p1, int p2) {}
void __stdcall FsRtlTruncateLargeMcb(int p0, int p1, int p2) {}
void __stdcall FsRtlTruncateMcb(int p0, int p1) {}
void __stdcall FsRtlUninitializeBaseMcb(int p0) {}
void __stdcall FsRtlUninitializeFileLock(int p0) {}
void __stdcall FsRtlUninitializeLargeMcb(int p0) {}
void __stdcall FsRtlUninitializeMcb(int p0) {}
void __stdcall FsRtlUninitializeOplock(int p0) {}
void __stdcall FsRtlUpdateDiskCounters(int p0, int p1, int p2, int p3) {}
void __stdcall FsRtlUpperOplockFsctrl(int p0, int p1, int p2, int p3, int p4) {}
void __stdcall FsRtlValidateReparsePointBuffer(int p0, int p1) {}
void __stdcall FsRtlVolumeDeviceToCorrelationId(int p0, int p1) {}
void __stdcall HalExamineMBR(int p0, int p1, int p2, int p3) {}
void __stdcall HvlRegisterWheaErrorNotification(int p0) {}
void __stdcall HvlUnregisterWheaErrorNotification(int p0) {}
void __stdcall IoAcquireCancelSpinLock(int p0) {}
void __stdcall IoAcquireKsrPersistentMemory(int p0, int p1, int p2, int p3) {}
void __stdcall IoAcquireKsrPersistentMemoryEx(int p0, int p1, int p2, int p3, int p4, int p5, int p6) {}
void __stdcall IoAcquireRemoveLockEx(int p0, int p1, int p2, int p3, int p4) {}
void __stdcall IoAcquireVpbSpinLock(int p0) {}
void __stdcall IoAllocateAdapterChannel(int p0, int p1, int p2, int p3, int p4) {}
void __stdcall IoAllocateController(int p0, int p1, int p2, int p3) {}
void __stdcall IoAllocateDriverObjectExtension(int p0, int p1, int p2, int p3) {}
void __stdcall IoAllocateErrorLogEntry(int p0, int p1) {}
void __stdcall IoAllocateIrp(int p0, int p1) {}
void __stdcall IoAllocateIrpEx(int p0, int p1, int p2) {}
void __stdcall IoAllocateMdl(int p0, int p1, int p2, int p3, int p4) {}
void __stdcall IoAllocateSfioStreamIdentifier(int p0, int p1, int p2, int p3) {}
void __stdcall IoAllocateWorkItem(int p0) {}
void __stdcall IoApplyPriorityInfoThread(int p0, int p1, int p2) {}
void __stdcall IoAssignResources(int p0, int p1, int p2, int p3, int p4, int p5) {}
void __stdcall IoAttachDevice(int p0, int p1, int p2) {}
void __stdcall IoAttachDeviceByPointer(int p0, int p1) {}
void __stdcall IoAttachDeviceToDeviceStack(int p0, int p1) {}
void __stdcall IoAttachDeviceToDeviceStackSafe(int p0, int p1, int p2) {}
void __stdcall IoBuildAsynchronousFsdRequest(int p0, int p1, int p2, int p3, int p4, int p5) {}
void __stdcall IoBuildDeviceIoControlRequest(int p0, int p1, int p2, int p3, int p4, int p5, int p6, int p7, int p8) {}
void __stdcall IoBuildPartialMdl(int p0, int p1, int p2, int p3) {}
void __stdcall IoBuildSynchronousFsdRequest(int p0, int p1, int p2, int p3, int p4, int p5, int p6) {}
void __stdcall IoCancelFileOpen(int p0, int p1) {}
void __stdcall IoCancelIrp(int p0) {}
void __stdcall IoCheckDesiredAccess(int p0, int p1) {}
void __stdcall IoCheckEaBufferValidity(int p0, int p1, int p2) {}
void __stdcall IoCheckFunctionAccess(int p0, int p1, int p2, int p3, int p4, int p5) {}
void __stdcall IoCheckLinkShareAccess(int p0, int p1, int p2, int p3, int p4, int p5) {}
void __stdcall IoCheckQuerySetFileInformation(int p0, int p1, int p2) {}
void __stdcall IoCheckQuerySetVolumeInformation(int p0, int p1, int p2) {}
void __stdcall IoCheckQuotaBufferValidity(int p0, int p1, int p2) {}
void __stdcall IoCheckShareAccess(int p0, int p1, int p2, int p3, int p4) {}
void __stdcall IoCheckShareAccessEx(int p0, int p1, int p2, int p3, int p4, int p5) {}
void __stdcall IoCleanupIrp(int p0) {}
void __stdcall IoClearActivityIdThread(int p0) {}
void __stdcall IoClearFsTrackOffsetState(int p0) {}
void __stdcall IoClearIrpExtraCreateParameter(int p0) {}
void __stdcall IoConnectInterrupt(int p0, int p1, int p2, int p3, int p4, int p5, int p6, int p7, int p8, int p9, int p10) {}
void __stdcall IoConnectInterruptEx(int p0) {}
void __stdcall IoCreateController(int p0) {}
void __stdcall IoCreateDevice(int p0, int p1, int p2, int p3, int p4, int p5, int p6) {}
void __stdcall IoCreateDisk(int p0, int p1) {}
void __stdcall IoCreateFile(int p0, int p1, int p2, int p3, int p4, int p5, int p6, int p7, int p8, int p9, int p10, int p11, int p12, int p13) {}
void __stdcall IoCreateFileEx(int p0, int p1, int p2, int p3, int p4, int p5, int p6, int p7, int p8, int p9, int p10, int p11, int p12, int p13, int p14) {}
void __stdcall IoCreateFileSpecifyDeviceObjectHint(int p0, int p1, int p2, int p3, int p4, int p5, int p6, int p7, int p8, int p9, int p10, int p11, int p12, int p13, int p14) {}
void __stdcall IoCreateNotificationEvent(int p0, int p1) {}
void __stdcall IoCreateStreamFileObject(int p0, int p1) {}
void __stdcall IoCreateStreamFileObjectEx(int p0, int p1, int p2) {}
void __stdcall IoCreateStreamFileObjectEx2(int p0, int p1, int p2, int p3, int p4) {}
void __stdcall IoCreateStreamFileObjectLite(int p0, int p1) {}
void __stdcall IoCreateSymbolicLink(int p0, int p1) {}
void __stdcall IoCreateSynchronizationEvent(int p0, int p1) {}
void __stdcall IoCreateSystemThread(int p0, int p1, int p2, int p3, int p4, int p5, int p6, int p7) {}
void __stdcall IoCreateUnprotectedSymbolicLink(int p0, int p1) {}
void __stdcall IoCsqInitialize(int p0, int p1, int p2, int p3, int p4, int p5, int p6) {}
void __stdcall IoCsqInitializeEx(int p0, int p1, int p2, int p3, int p4, int p5, int p6) {}
void __stdcall IoCsqInsertIrp(int p0, int p1, int p2) {}
void __stdcall IoCsqInsertIrpEx(int p0, int p1, int p2, int p3) {}
void __stdcall IoCsqRemoveIrp(int p0, int p1) {}
void __stdcall IoCsqRemoveNextIrp(int p0, int p1) {}
void __stdcall IoDecrementKeepAliveCount(int p0, int p1) {}
void __stdcall IoDeleteController(int p0) {}
void __stdcall IoDeleteDevice(int p0) {}
void __stdcall IoDeleteSymbolicLink(int p0) {}
void __stdcall IoDetachDevice(int p0) {}
void __stdcall IoDisconnectInterrupt(int p0) {}
void __stdcall IoDisconnectInterruptEx(int p0) {}
void __stdcall IoEnumerateDeviceObjectList(int p0, int p1, int p2, int p3) {}
void __stdcall IoEnumerateKsrPersistentMemoryEx(int p0, int p1, int p2, int p3, int p4) {}
void __stdcall IoEnumerateRegisteredFiltersList(int p0, int p1, int p2) {}
void __stdcall IoFastQueryNetworkAttributes(int p0, int p1, int p2, int p3, int p4) {}
void __stdcall IoForwardIrpSynchronously(int p0, int p1) {}
void __stdcall IoFreeController(int p0) {}
void __stdcall IoFreeErrorLogEntry(int p0) {}
void __stdcall IoFreeIrp(int p0) {}
void __stdcall IoFreeKsrPersistentMemory(int p0) {}
void __stdcall IoFreeMdl(int p0) {}
void __stdcall IoFreeSfioStreamIdentifier(int p0, int p1) {}
void __stdcall IoFreeWorkItem(int p0) {}
void __stdcall IoGetActivityIdIrp(int p0, int p1) {}
void __stdcall IoGetActivityIdThread() {}
void __stdcall IoGetAffinityInterrupt(int p0, int p1) {}
void __stdcall IoGetAttachedDevice(int p0) {}
void __stdcall IoGetAttachedDeviceReference(int p0) {}
void __stdcall IoGetBaseFileSystemDeviceObject(int p0) {}
void __stdcall IoGetBootDiskInformation(int p0, int p1) {}
void __stdcall IoGetBootDiskInformationLite(int p0) {}
void __stdcall IoGetConfigurationInformation() {}
void __stdcall IoGetContainerInformation(int p0, int p1, int p2, int p3) {}
void __stdcall IoGetCurrentProcess() {}
void __stdcall IoGetDeviceAttachmentBaseRef(int p0) {}
void __stdcall IoGetDeviceDirectory(int p0, int p1, int p2, int p3, int p4) {}
void __stdcall IoGetDeviceInterfaceAlias(int p0, int p1, int p2) {}
void __stdcall IoGetDeviceInterfacePropertyData(int p0, int p1, int p2, int p3, int p4, int p5, int p6, int p7) {}
void __stdcall IoGetDeviceInterfaces(int p0, int p1, int p2, int p3) {}
void __stdcall IoGetDeviceNumaNode(int p0, int p1) {}
void __stdcall IoGetDeviceObjectPointer(int p0, int p1, int p2, int p3) {}
void __stdcall IoGetDeviceProperty(int p0, int p1, int p2, int p3, int p4) {}
void __stdcall IoGetDevicePropertyData(int p0, int p1, int p2, int p3, int p4, int p5, int p6, int p7) {}
void __stdcall IoGetDeviceToVerify(int p0) {}
void __stdcall IoGetDiskDeviceObject(int p0, int p1) {}
void __stdcall IoGetDmaAdapter(int p0, int p1, int p2) {}
void __stdcall IoGetDriverDirectory(int p0, int p1, int p2, int p3) {}
void __stdcall IoGetDriverObjectExtension(int p0, int p1) {}
void __stdcall IoGetFileObjectGenericMapping() {}
void __stdcall IoGetFsTrackOffsetState(int p0, int p1, int p2) {}
void __stdcall IoGetFsZeroingOffset(int p0, int p1) {}
void __stdcall IoGetInitialStack() {}
void __stdcall IoGetInitiatorProcess(int p0) {}
void __stdcall IoGetIoAttributionHandle(int p0, int p1) {}
void __stdcall IoGetIoPriorityHint(int p0) {}
void __stdcall IoGetIommuInterface(int p0, int p1) {}
void __stdcall IoGetIommuInterfaceEx(int p0, int p1, int p2, int p3) {}
void __stdcall IoGetIrpExtraCreateParameter(int p0, int p1) {}
void __stdcall IoGetLowerDeviceObject(int p0) {}
void __stdcall IoGetOplockKeyContext(int p0) {}
void __stdcall IoGetOplockKeyContextEx(int p0) {}
void __stdcall IoGetPagingIoPriority(int p0) {}
void __stdcall IoGetRelatedDeviceObject(int p0) {}
void __stdcall IoGetRequestorProcess(int p0) {}
void __stdcall IoGetRequestorProcessId(int p0) {}
void __stdcall IoGetRequestorSessionId(int p0, int p1) {}
void __stdcall IoGetSfioStreamIdentifier(int p0, int p1) {}
void __stdcall IoGetSilo(int p0) {}
void __stdcall IoGetSiloParameters(int p0) {}
void __stdcall IoGetStackLimits(int p0, int p1) {}
void __stdcall IoGetTopLevelIrp() {}
void __stdcall IoGetTransactionParameterBlock(int p0) {}
void __stdcall IoIncrementKeepAliveCount(int p0, int p1) {}
void __stdcall IoInitializeIrp(int p0, int p1, int p2) {}
void __stdcall IoInitializeIrpEx(int p0, int p1, int p2, int p3) {}
void __stdcall IoInitializeRemoveLockEx(int p0, int p1, int p2, int p3, int p4) {}
void __stdcall IoInitializeTimer(int p0, int p1, int p2) {}
void __stdcall IoInitializeWorkItem(int p0, int p1) {}
void __stdcall IoInvalidateDeviceRelations(int p0, int p1) {}
void __stdcall IoInvalidateDeviceState(int p0) {}
void __stdcall IoIrpHasFsTrackOffsetExtensionType(int p0) {}
void __stdcall IoIs32bitProcess(int p0) {}
void __stdcall IoIsFileObjectIgnoringSharing(int p0) {}
void __stdcall IoIsFileOriginRemote(int p0) {}
void __stdcall IoIsInitiator32bitProcess(int p0) {}
void __stdcall IoIsOperationSynchronous(int p0) {}
void __stdcall IoIsSystemThread(int p0) {}
void __stdcall IoIsValidIrpStatus(int p0) {}
void __stdcall IoIsValidNameGraftingBuffer(int p0, int p1) {}
void __stdcall IoIsWdmVersionAvailable(int p0, int p1) {}
void __stdcall IoMakeAssociatedIrp(int p0, int p1) {}
void __stdcall IoMakeAssociatedIrpEx(int p0, int p1, int p2) {}
void __stdcall IoOpenDeviceInterfaceRegistryKey(int p0, int p1, int p2) {}
void __stdcall IoOpenDeviceRegistryKey(int p0, int p1, int p2, int p3) {}
void __stdcall IoOpenDriverRegistryKey(int p0, int p1, int p2, int p3, int p4) {}
void __stdcall IoPageRead(int p0, int p1, int p2, int p3, int p4) {}
void __stdcall IoPropagateActivityIdToThread(int p0, int p1, int p2) {}
void __stdcall IoQueryDeviceDescription(int p0, int p1, int p2, int p3, int p4, int p5, int p6, int p7) {}
void __stdcall IoQueryFileDosDeviceName(int p0, int p1) {}
void __stdcall IoQueryFileInformation(int p0, int p1, int p2, int p3, int p4) {}
void __stdcall IoQueryFullDriverPath(int p0, int p1) {}
void __stdcall IoQueryInformationByName(int p0, int p1, int p2, int p3, int p4, int p5, int p6) {}
void __stdcall IoQueryKsrPersistentMemorySize(int p0, int p1, int p2) {}
void __stdcall IoQueryKsrPersistentMemorySizeEx(int p0, int p1, int p2, int p3, int p4, int p5) {}
void __stdcall IoQueryVolumeInformation(int p0, int p1, int p2, int p3, int p4) {}
void __stdcall IoQueueThreadIrp(int p0) {}
void __stdcall IoQueueWorkItem(int p0, int p1, int p2, int p3) {}
void __stdcall IoQueueWorkItemEx(int p0, int p1, int p2, int p3) {}
void __stdcall IoRaiseHardError(int p0, int p1, int p2) {}
void __stdcall IoRaiseInformationalHardError(int p0, int p1, int p2) {}
void __stdcall IoReadDiskSignature(int p0, int p1, int p2) {}
void __stdcall IoReadPartitionTable(int p0, int p1, int p2, int p3) {}
void __stdcall IoReadPartitionTableEx(int p0, int p1) {}
void __stdcall IoRecordIoAttribution(int p0, int p1) {}
void __stdcall IoRegisterBootDriverCallback(int p0, int p1) {}
void __stdcall IoRegisterBootDriverReinitialization(int p0, int p1, int p2) {}
void __stdcall IoRegisterContainerNotification(int p0, int p1, int p2, int p3, int p4) {}
void __stdcall IoRegisterDeviceInterface(int p0, int p1, int p2, int p3) {}
void __stdcall IoRegisterDriverReinitialization(int p0, int p1, int p2) {}
void __stdcall IoRegisterFileSystem(int p0) {}
void __stdcall IoRegisterFsRegistrationChange(int p0, int p1) {}
void __stdcall IoRegisterFsRegistrationChangeMountAware(int p0, int p1, int p2) {}
void __stdcall IoRegisterLastChanceShutdownNotification(int p0) {}
void __stdcall IoRegisterPlugPlayNotification(int p0, int p1, int p2, int p3, int p4, int p5, int p6) {}
void __stdcall IoRegisterShutdownNotification(int p0) {}
void __stdcall IoReleaseCancelSpinLock(int p0) {}
void __stdcall IoReleaseRemoveLockAndWaitEx(int p0, int p1, int p2) {}
void __stdcall IoReleaseRemoveLockEx(int p0, int p1, int p2) {}
void __stdcall IoReleaseVpbSpinLock(int p0) {}
void __stdcall IoRemoveLinkShareAccess(int p0, int p1, int p2) {}
void __stdcall IoRemoveLinkShareAccessEx(int p0, int p1, int p2, int p3) {}
void __stdcall IoRemoveShareAccess(int p0, int p1) {}
void __stdcall IoReplaceFileObjectName(int p0, int p1, int p2) {}
void __stdcall IoReplacePartitionUnit(int p0, int p1, int p2) {}
void __stdcall IoReportDetectedDevice(int p0, int p1, int p2, int p3, int p4, int p5, int p6, int p7) {}
void __stdcall IoReportInterruptActive(int p0) {}
void __stdcall IoReportInterruptInactive(int p0) {}
void __stdcall IoReportResourceForDetection(int p0, int p1, int p2, int p3, int p4, int p5, int p6) {}
void __stdcall IoReportResourceUsage(int p0, int p1, int p2, int p3, int p4, int p5, int p6, int p7, int p8) {}
void __stdcall IoReportRootDevice(int p0) {}
void __stdcall IoReportTargetDeviceChange(int p0, int p1) {}
void __stdcall IoReportTargetDeviceChangeAsynchronous(int p0, int p1, int p2, int p3) {}
void __stdcall IoRequestDeviceEject(int p0) {}
void __stdcall IoRequestDeviceEjectEx(int p0, int p1, int p2, int p3) {}
void __stdcall IoRequestDeviceRemovalForReset(int p0, int p1) {}
void __stdcall IoReserveKsrPersistentMemory(int p0, int p1, int p2, int p3, int p4) {}
void __stdcall IoReserveKsrPersistentMemoryEx(int p0, int p1, int p2, int p3, int p4, int p5, int p6, int p7) {}
void __stdcall IoRetrievePriorityInfo(int p0, int p1, int p2, int p3) {}
void __stdcall IoReuseIrp(int p0, int p1) {}
void __stdcall IoSetActivityIdIrp(int p0, int p1) {}
void __stdcall IoSetActivityIdThread(int p0) {}
void __stdcall IoSetCompletionRoutineEx(int p0, int p1, int p2, int p3, int p4, int p5, int p6) {}
void __stdcall IoSetDeviceInterfacePropertyData(int p0, int p1, int p2, int p3, int p4, int p5, int p6) {}
void __stdcall IoSetDeviceInterfaceState(int p0, int p1) {}
void __stdcall IoSetDevicePropertyData(int p0, int p1, int p2, int p3, int p4, int p5, int p6) {}
void __stdcall IoSetDeviceToVerify(int p0, int p1) {}
void __stdcall IoSetFileObjectIgnoreSharing(int p0) {}
void __stdcall IoSetFileOrigin(int p0, int p1) {}
void __stdcall IoSetFsTrackOffsetState(int p0, int p1, int p2, int p3) {}
void __stdcall IoSetFsZeroingOffset(int p0, int p1) {}
void __stdcall IoSetFsZeroingOffsetRequired(int p0) {}
void __stdcall IoSetHardErrorOrVerifyDevice(int p0, int p1) {}
void __stdcall IoSetInformation(int p0, int p1, int p2, int p3) {}
void __stdcall IoSetIoAttributionIrp(int p0, int p1, int p2) {}
void __stdcall IoSetIoPriorityHint(int p0, int p1) {}
void __stdcall IoSetIrpExtraCreateParameter(int p0, int p1) {}
void __stdcall IoSetLinkShareAccess(int p0, int p1, int p2, int p3, int p4, int p5) {}
void __stdcall IoSetMasterIrpStatus(int p0, int p1) {}
void __stdcall IoSetPartitionInformation(int p0, int p1, int p2, int p3) {}
void __stdcall IoSetPartitionInformationEx(int p0, int p1, int p2) {}
void __stdcall IoSetShareAccess(int p0, int p1, int p2, int p3) {}
void __stdcall IoSetShareAccessEx(int p0, int p1, int p2, int p3, int p4) {}
void __stdcall IoSetStartIoAttributes(int p0, int p1, int p2) {}
void __stdcall IoSetSystemPartition(int p0) {}
void __stdcall IoSetThreadHardErrorMode(int p0) {}
void __stdcall IoSetTopLevelIrp(int p0) {}
void __stdcall IoSizeOfIrpEx(int p0, int p1) {}
void __stdcall IoSizeofWorkItem() {}
void __stdcall IoStartNextPacket(int p0, int p1) {}
void __stdcall IoStartNextPacketByKey(int p0, int p1, int p2) {}
void __stdcall IoStartPacket(int p0, int p1, int p2, int p3) {}
void __stdcall IoStartTimer(int p0) {}
void __stdcall IoStopTimer(int p0) {}
void __stdcall IoSynchronousCallDriver(int p0, int p1) {}
void __stdcall IoSynchronousPageWrite(int p0, int p1, int p2, int p3, int p4) {}
void __stdcall IoThreadToProcess(int p0) {}
void __stdcall IoTransferActivityId(int p0, int p1) {}
void __stdcall IoTranslateBusAddress(int p0, int p1, int p2, int p3, int p4, int p5) {}
void __stdcall IoTryQueueWorkItem(int p0, int p1, int p2, int p3) {}
void __stdcall IoUninitializeWorkItem(int p0) {}
void __stdcall IoUnregisterBootDriverCallback(int p0) {}
void __stdcall IoUnregisterContainerNotification(int p0) {}
void __stdcall IoUnregisterFileSystem(int p0) {}
void __stdcall IoUnregisterFsRegistrationChange(int p0, int p1) {}
void __stdcall IoUnregisterPlugPlayNotification(int p0) {}
void __stdcall IoUnregisterPlugPlayNotificationEx(int p0) {}
void __stdcall IoUnregisterShutdownNotification(int p0) {}
void __stdcall IoUpdateLinkShareAccess(int p0, int p1, int p2) {}
void __stdcall IoUpdateLinkShareAccessEx(int p0, int p1, int p2, int p3) {}
void __stdcall IoUpdateShareAccess(int p0, int p1) {}
void __stdcall IoValidateDeviceIoControlAccess(int p0, int p1) {}
void __stdcall IoVerifyPartitionTable(int p0, int p1) {}
void __stdcall IoVerifyVolume(int p0, int p1) {}
void __stdcall IoVolumeDeviceNameToGuid(int p0, int p1) {}
void __stdcall IoVolumeDeviceNameToGuidPath(int p0, int p1) {}
void __stdcall IoVolumeDeviceToDosName(int p0, int p1) {}
void __stdcall IoVolumeDeviceToGuid(int p0, int p1) {}
void __stdcall IoVolumeDeviceToGuidPath(int p0, int p1) {}
void __stdcall IoWMIAllocateInstanceIds(int p0, int p1, int p2) {}
void __stdcall IoWMIDeviceObjectToInstanceName(int p0, int p1, int p2) {}
void __stdcall IoWMIExecuteMethod(int p0, int p1, int p2, int p3, int p4, int p5) {}
void __stdcall IoWMIHandleToInstanceName(int p0, int p1, int p2) {}
void __stdcall IoWMIOpenBlock(int p0, int p1, int p2) {}
void __stdcall IoWMIQueryAllData(int p0, int p1, int p2) {}
void __stdcall IoWMIQueryAllDataMultiple(int p0, int p1, int p2, int p3) {}
void __stdcall IoWMIQuerySingleInstance(int p0, int p1, int p2, int p3) {}
void __stdcall IoWMIQuerySingleInstanceMultiple(int p0, int p1, int p2, int p3, int p4) {}
void __stdcall IoWMIRegistrationControl(int p0, int p1) {}
void __stdcall IoWMISetNotificationCallback(int p0, int p1, int p2) {}
void __stdcall IoWMISetSingleInstance(int p0, int p1, int p2, int p3, int p4) {}
void __stdcall IoWMISetSingleItem(int p0, int p1, int p2, int p3, int p4, int p5) {}
void __stdcall IoWMISuggestInstanceName(int p0, int p1, int p2, int p3) {}
void __stdcall IoWMIWriteEvent(int p0) {}
void __stdcall IoWithinStackLimits(int p0, int p1) {}
void __stdcall IoWriteErrorLogEntry(int p0) {}
void __stdcall IoWriteKsrPersistentMemory(int p0, int p1, int p2) {}
void __stdcall IoWritePartitionTable(int p0, int p1, int p2, int p3, int p4) {}
void __stdcall IoWritePartitionTableEx(int p0, int p1) {}
void __stdcall IofCallDriver(int p0, int p1) {}
void __stdcall IofCompleteRequest(int p0, int p1) {}
void __stdcall KdChangeOption(int p0, int p1, int p2, int p3, int p4, int p5) {}
void __stdcall KdDisableDebugger() {}
void __stdcall KdEnableDebugger() {}
void __stdcall KdRefreshDebuggerNotPresent() {}
void __stdcall KeAcquireGuardedMutex(int p0) {}
void __stdcall KeAcquireGuardedMutexUnsafe(int p0) {}
void __stdcall KeAcquireInStackQueuedSpinLock(int p0, int p1) {}
void __stdcall KeAcquireInStackQueuedSpinLockAtDpcLevel(int p0, int p1) {}
void __stdcall KeAcquireInStackQueuedSpinLockForDpc(int p0, int p1) {}
void __stdcall KeAcquireInterruptSpinLock(int p0) {}
void __stdcall KeAcquireQueuedSpinLock(int p0) {}
void __stdcall KeAcquireSpinLockForDpc(int p0) {}
void __stdcall KeAcquireSpinLockRaiseToSynch(int p0) {}
void __stdcall KeAddTriageDumpDataBlock(int p0, int p1, int p2) {}
void __stdcall KeAreAllApcsDisabled() {}
void __stdcall KeAreApcsDisabled() {}
void __stdcall KeAttachProcess(int p0) {}
void __stdcall KeBugCheck(int p0) {}
void __stdcall KeBugCheckEx(int p0, int p1, int p2, int p3, int p4) {}
void __stdcall KeCancelTimer(int p0) {}
void __stdcall KeClearEvent(int p0) {}
void __stdcall KeConvertAuxiliaryCounterToPerformanceCounter(int p0, int p1, int p2, int p3) {}
void __stdcall KeConvertPerformanceCounterToAuxiliaryCounter(int p0, int p1, int p2, int p3) {}
void __stdcall KeDelayExecutionThread(int p0, int p1, int p2) {}
void __stdcall KeDeregisterBoundCallback(int p0) {}
void __stdcall KeDeregisterBugCheckCallback(int p0) {}
void __stdcall KeDeregisterBugCheckReasonCallback(int p0) {}
void __stdcall KeDeregisterNmiCallback(int p0) {}
void __stdcall KeDeregisterProcessorChangeCallback(int p0) {}
void __stdcall KeDetachProcess() {}
void __stdcall KeEnterCriticalRegion() {}
void __stdcall KeEnterGuardedRegion() {}
void __stdcall KeExpandKernelStackAndCallout(int p0, int p1, int p2) {}
void __stdcall KeExpandKernelStackAndCalloutEx(int p0, int p1, int p2, int p3, int p4) {}
void __stdcall KeFlushIoBuffers(int p0, int p1, int p2) {}
void __stdcall KeFlushQueuedDpcs() {}
void __stdcall KeGetCurrentIrql() {}
void __stdcall KeGetCurrentNodeNumber() {}
void __stdcall KeGetCurrentProcessorNumberEx(int p0) {}
void __stdcall KeGetProcessorIndexFromNumber(int p0) {}
void __stdcall KeGetProcessorNumberFromIndex(int p0, int p1) {}
void __stdcall KeGetRecommendedSharedDataAlignment() {}
void __stdcall KeInitializeCrashDumpHeader(int p0, int p1, int p2, int p3, int p4) {}
void __stdcall KeInitializeDeviceQueue(int p0) {}
void __stdcall KeInitializeDpc(int p0, int p1, int p2) {}
void __stdcall KeInitializeEvent(int p0, int p1, int p2) {}
void __stdcall KeInitializeGuardedMutex(int p0) {}
void __stdcall KeInitializeMutant(int p0, int p1) {}
void __stdcall KeInitializeMutex(int p0, int p1) {}
void __stdcall KeInitializeQueue(int p0, int p1) {}
void __stdcall KeInitializeSemaphore(int p0, int p1, int p2) {}
void __stdcall KeInitializeSpinLock(int p0) {}
void __stdcall KeInitializeThreadedDpc(int p0, int p1, int p2) {}
void __stdcall KeInitializeTimer(int p0) {}
void __stdcall KeInitializeTimerEx(int p0, int p1) {}
void __stdcall KeInitializeTriageDumpDataArray(int p0, int p1) {}
void __stdcall KeInsertByKeyDeviceQueue(int p0, int p1, int p2) {}
void __stdcall KeInsertDeviceQueue(int p0, int p1) {}
void __stdcall KeInsertHeadQueue(int p0, int p1) {}
void __stdcall KeInsertQueue(int p0, int p1) {}
void __stdcall KeInsertQueueDpc(int p0, int p1, int p2) {}
void __stdcall KeInvalidateAllCaches() {}
void __stdcall KeInvalidateRangeAllCaches(int p0, int p1) {}
void __stdcall KeIpiGenericCall(int p0, int p1) {}
void __stdcall KeIsExecutingDpc() {}
void __stdcall KeLeaveCriticalRegion() {}
void __stdcall KeLeaveGuardedRegion() {}
void __stdcall KePulseEvent(int p0, int p1, int p2) {}
void __stdcall KeQueryActiveGroupCount() {}
void __stdcall KeQueryActiveProcessorCount(int p0) {}
void __stdcall KeQueryActiveProcessorCountEx(int p0) {}
void __stdcall KeQueryActiveProcessors() {}
void __stdcall KeQueryAuxiliaryCounterFrequency(int p0) {}
void __stdcall KeQueryDpcWatchdogInformation(int p0) {}
void __stdcall KeQueryGroupAffinity(int p0) {}
void __stdcall KeQueryHardwareCounterConfiguration(int p0, int p1, int p2) {}
void __stdcall KeQueryHighestNodeNumber() {}
void __stdcall KeQueryInterruptTimePrecise(int p0) {}
void __stdcall KeQueryLogicalProcessorRelationship(int p0, int p1, int p2, int p3) {}
void __stdcall KeQueryMaximumGroupCount() {}
void __stdcall KeQueryMaximumProcessorCount() {}
void __stdcall KeQueryMaximumProcessorCountEx(int p0) {}
void __stdcall KeQueryNodeActiveAffinity(int p0, int p1, int p2) {}
void __stdcall KeQueryNodeActiveAffinity2(int p0, int p1, int p2, int p3) {}
void __stdcall KeQueryNodeActiveProcessorCount(int p0) {}
void __stdcall KeQueryNodeMaximumProcessorCount(int p0) {}
void __stdcall KeQueryPriorityThread(int p0) {}
void __stdcall KeQueryRuntimeThread(int p0, int p1) {}
void __stdcall KeQuerySystemTimePrecise(int p0) {}
void __stdcall KeQueryTimeIncrement() {}
void __stdcall KeQueryTotalCycleTimeThread(int p0, int p1) {}
void __stdcall KeQueryUnbiasedInterruptTime() {}
void __stdcall KeQueryUnbiasedInterruptTimePrecise(int p0) {}
void __stdcall KeReadStateEvent(int p0) {}
void __stdcall KeReadStateMutant(int p0) {}
void __stdcall KeReadStateMutex(int p0) {}
void __stdcall KeReadStateQueue(int p0) {}
void __stdcall KeReadStateSemaphore(int p0) {}
void __stdcall KeReadStateTimer(int p0) {}
void __stdcall KeRegisterBoundCallback(int p0) {}
void __stdcall KeRegisterBugCheckCallback(int p0, int p1, int p2, int p3, int p4) {}
void __stdcall KeRegisterBugCheckReasonCallback(int p0, int p1, int p2, int p3) {}
void __stdcall KeRegisterNmiCallback(int p0, int p1) {}
void __stdcall KeRegisterProcessorChangeCallback(int p0, int p1, int p2) {}
void __stdcall KeReleaseGuardedMutex(int p0) {}
void __stdcall KeReleaseGuardedMutexUnsafe(int p0) {}
void __stdcall KeReleaseInStackQueuedSpinLock(int p0) {}
void __stdcall KeReleaseInStackQueuedSpinLockForDpc(int p0) {}
void __stdcall KeReleaseInStackQueuedSpinLockFromDpcLevel(int p0) {}
void __stdcall KeReleaseInterruptSpinLock(int p0, int p1) {}
void __stdcall KeReleaseMutant(int p0, int p1, int p2, int p3) {}
void __stdcall KeReleaseMutex(int p0, int p1) {}
void __stdcall KeReleaseQueuedSpinLock(int p0, int p1) {}
void __stdcall KeReleaseSemaphore(int p0, int p1, int p2, int p3) {}
void __stdcall KeReleaseSpinLockForDpc(int p0, int p1) {}
void __stdcall KeRemoveByKeyDeviceQueue(int p0, int p1) {}
void __stdcall KeRemoveByKeyDeviceQueueIfBusy(int p0, int p1) {}
void __stdcall KeRemoveDeviceQueue(int p0) {}
void __stdcall KeRemoveEntryDeviceQueue(int p0, int p1) {}
void __stdcall KeRemoveQueue(int p0, int p1, int p2) {}
void __stdcall KeRemoveQueueDpc(int p0) {}
void __stdcall KeRemoveQueueDpcEx(int p0, int p1) {}
void __stdcall KeRemoveQueueEx(int p0, int p1, int p2, int p3, int p4, int p5) {}
void __stdcall KeResetEvent(int p0) {}
void __stdcall KeRestoreExtendedProcessorState(int p0) {}
void __stdcall KeRevertToUserAffinityThread() {}
void __stdcall KeRevertToUserAffinityThreadEx(int p0) {}
void __stdcall KeRevertToUserGroupAffinityThread(int p0) {}
void __stdcall KeRundownQueue(int p0) {}
void __stdcall KeSaveExtendedProcessorState(int p0, int p1, int p2) {}
void __stdcall KeSetBasePriorityThread(int p0, int p1) {}
void __stdcall KeSetCoalescableTimer(int p0, int p1, int p2, int p3, int p4, int p5) {}
void __stdcall KeSetEvent(int p0, int p1, int p2) {}
void __stdcall KeSetHardwareCounterConfiguration(int p0, int p1) {}
void __stdcall KeSetIdealProcessorThread(int p0, int p1) {}
void __stdcall KeSetImportanceDpc(int p0, int p1) {}
void __stdcall KeSetKernelStackSwapEnable(int p0) {}
void __stdcall KeSetPriorityThread(int p0, int p1) {}
void __stdcall KeSetSystemAffinityThread(int p0) {}
void __stdcall KeSetSystemAffinityThreadEx(int p0) {}
void __stdcall KeSetSystemGroupAffinityThread(int p0, int p1) {}
void __stdcall KeSetTargetProcessorDpc(int p0, int p1) {}
void __stdcall KeSetTargetProcessorDpcEx(int p0, int p1) {}
void __stdcall KeSetTimer(int p0, int p1, int p2, int p3) {}
void __stdcall KeSetTimerEx(int p0, int p1, int p2, int p3, int p4) {}
void __stdcall KeShouldYieldProcessor() {}
void __stdcall KeStackAttachProcess(int p0, int p1) {}
void __stdcall KeSynchronizeExecution(int p0, int p1, int p2) {}
void __stdcall KeTestSpinLock(int p0) {}
void __stdcall KeTryToAcquireGuardedMutex(int p0) {}
void __stdcall KeTryToAcquireQueuedSpinLock(int p0, int p1) {}
void __stdcall KeTryToAcquireSpinLockAtDpcLevel(int p0) {}
void __stdcall KeUnstackDetachProcess(int p0) {}
void __stdcall KeWaitForMultipleObjects(int p0, int p1, int p2, int p3, int p4, int p5, int p6, int p7) {}
void __stdcall KeWaitForSingleObject(int p0, int p1, int p2, int p3, int p4) {}
void __stdcall KfRaiseIrql(int p0) {}
void __stdcall MmAddPhysicalMemory(int p0, int p1) {}
void __stdcall MmAddVerifierSpecialThunks(int p0, int p1, int p2) {}
void __stdcall MmAddVerifierThunks(int p0, int p1) {}
void __stdcall MmAdvanceMdl(int p0, int p1) {}
void __stdcall MmAllocateContiguousMemory(int p0, int p1, int p2) {}
void __stdcall MmAllocateContiguousMemoryEx(int p0, int p1, int p2, int p3, int p4, int p5, int p6, int p7, int p8, int p9, int p10, int p11, int p12) {}
void __stdcall MmAllocateContiguousMemorySpecifyCache(int p0, int p1, int p2, int p3, int p4, int p5, int p6, int p7) {}
void __stdcall MmAllocateContiguousMemorySpecifyCacheNode(int p0, int p1, int p2, int p3, int p4, int p5, int p6, int p7, int p8) {}
void __stdcall MmAllocateContiguousNodeMemory(int p0, int p1, int p2, int p3, int p4, int p5, int p6, int p7, int p8) {}
void __stdcall MmAllocateMappingAddress(int p0, int p1) {}
void __stdcall MmAllocateMappingAddressEx(int p0, int p1, int p2) {}
void __stdcall MmAllocateMdlForIoSpace(int p0, int p1, int p2) {}
void __stdcall MmAllocateNodePagesForMdlEx(int p0, int p1, int p2, int p3, int p4, int p5, int p6, int p7, int p8, int p9) {}
void __stdcall MmAllocateNonCachedMemory(int p0) {}
void __stdcall MmAllocatePagesForMdl(int p0, int p1, int p2, int p3, int p4, int p5, int p6) {}
void __stdcall MmAllocatePagesForMdlEx(int p0, int p1, int p2, int p3, int p4, int p5, int p6, int p7, int p8) {}
void __stdcall MmAllocatePartitionNodePagesForMdlEx(int p0, int p1, int p2, int p3, int p4, int p5, int p6, int p7, int p8, int p9, int p10) {}
void __stdcall MmAreMdlPagesCached(int p0) {}
void __stdcall MmBuildMdlForNonPagedPool(int p0) {}
void __stdcall MmCanFileBeTruncated(int p0, int p1) {}
void __stdcall MmCopyMemory(int p0, int p1, int p2, int p3, int p4, int p5) {}
void __stdcall MmCreateMdl(int p0, int p1, int p2) {}
void __stdcall MmCreateMirror() {}
void __stdcall MmDoesFileHaveUserWritableReferences(int p0) {}
void __stdcall MmFlushImageSection(int p0, int p1) {}
void __stdcall MmForceSectionClosed(int p0, int p1) {}
void __stdcall MmForceSectionClosedEx(int p0, int p1) {}
void __stdcall MmFreeContiguousMemory(int p0) {}
void __stdcall MmFreeContiguousMemorySpecifyCache(int p0, int p1, int p2) {}
void __stdcall MmFreeMappingAddress(int p0, int p1) {}
void __stdcall MmFreeNonCachedMemory(int p0, int p1) {}
void __stdcall MmFreePagesFromMdl(int p0) {}
void __stdcall MmFreePagesFromMdlEx(int p0, int p1) {}
void __stdcall MmGetCacheAttribute(int p0, int p1, int p2) {}
void __stdcall MmGetCacheAttributeEx(int p0, int p1, int p2, int p3) {}
void __stdcall MmGetMaximumFileSectionSize() {}
void __stdcall MmGetPhysicalAddress(int p0) {}
void __stdcall MmGetPhysicalMemoryRanges() {}
void __stdcall MmGetPhysicalMemoryRangesEx(int p0) {}
void __stdcall MmGetPhysicalMemoryRangesEx2(int p0, int p1) {}
void __stdcall MmGetSystemRoutineAddress(int p0) {}
void __stdcall MmGetVirtualForPhysical(int p0, int p1) {}
void __stdcall MmIsAddressValid(int p0) {}
void __stdcall MmIsDriverSuspectForVerifier(int p0) {}
void __stdcall MmIsDriverVerifying(int p0) {}
void __stdcall MmIsDriverVerifyingByAddress(int p0) {}
void __stdcall MmIsFileSectionActive(int p0, int p1, int p2) {}
void __stdcall MmIsIoSpaceActive(int p0, int p1, int p2) {}
void __stdcall MmIsNonPagedSystemAddressValid(int p0) {}
void __stdcall MmIsRecursiveIoFault() {}
void __stdcall MmIsThisAnNtAsSystem() {}
void __stdcall MmIsVerifierEnabled(int p0) {}
void __stdcall MmLockPagableDataSection(int p0) {}
void __stdcall MmLockPagableSectionByHandle(int p0) {}
void __stdcall MmMapIoSpace(int p0, int p1, int p2, int p3) {}
void __stdcall MmMapIoSpaceEx(int p0, int p1, int p2, int p3) {}
void __stdcall MmMapLockedPages(int p0, int p1) {}
void __stdcall MmMapLockedPagesSpecifyCache(int p0, int p1, int p2, int p3, int p4, int p5) {}
void __stdcall MmMapLockedPagesWithReservedMapping(int p0, int p1, int p2, int p3) {}
void __stdcall MmMapMdl(int p0, int p1, int p2, int p3) {}
void __stdcall MmMapMemoryDumpMdlEx(int p0, int p1, int p2, int p3) {}
void __stdcall MmMapUserAddressesToPage(int p0, int p1, int p2) {}
void __stdcall MmMapVideoDisplay(int p0, int p1, int p2, int p3) {}
void __stdcall MmMapViewInSessionSpace(int p0, int p1, int p2) {}
void __stdcall MmMapViewInSessionSpaceEx(int p0, int p1, int p2, int p3, int p4) {}
void __stdcall MmMapViewInSystemSpace(int p0, int p1, int p2) {}
void __stdcall MmMapViewInSystemSpaceEx(int p0, int p1, int p2, int p3, int p4) {}
void __stdcall MmMdlPageContentsState(int p0, int p1) {}
void __stdcall MmMdlPagesAreZero(int p0) {}
void __stdcall MmPageEntireDriver(int p0) {}
void __stdcall MmPrefetchPages(int p0, int p1) {}
void __stdcall MmProbeAndLockPages(int p0, int p1, int p2) {}
void __stdcall MmProbeAndLockProcessPages(int p0, int p1, int p2, int p3) {}
void __stdcall MmProbeAndLockSelectedPages(int p0, int p1, int p2, int p3) {}
void __stdcall MmProtectDriverSection(int p0, int p1, int p2) {}
void __stdcall MmProtectMdlSystemAddress(int p0, int p1) {}
void __stdcall MmQuerySystemSize() {}
void __stdcall MmRemovePhysicalMemory(int p0, int p1) {}
void __stdcall MmResetDriverPaging(int p0) {}
void __stdcall MmRotatePhysicalView(int p0, int p1, int p2, int p3, int p4, int p5) {}
void __stdcall MmSecureVirtualMemory(int p0, int p1, int p2) {}
void __stdcall MmSecureVirtualMemoryEx(int p0, int p1, int p2, int p3) {}
void __stdcall MmSetAddressRangeModified(int p0, int p1) {}
void __stdcall MmSetPermanentCacheAttribute(int p0, int p1, int p2, int p3, int p4, int p5) {}
void __stdcall MmSizeOfMdl(int p0, int p1) {}
void __stdcall MmUnlockPagableImageSection(int p0) {}
void __stdcall MmUnlockPages(int p0) {}
void __stdcall MmUnmapIoSpace(int p0, int p1) {}
void __stdcall MmUnmapLockedPages(int p0, int p1) {}
void __stdcall MmUnmapReservedMapping(int p0, int p1, int p2) {}
void __stdcall MmUnmapVideoDisplay(int p0, int p1) {}
void __stdcall MmUnmapViewInSessionSpace(int p0) {}
void __stdcall MmUnmapViewInSystemSpace(int p0) {}
void __stdcall MmUnsecureVirtualMemory(int p0) {}
void __stdcall ObCloseHandle(int p0, int p1) {}
void __stdcall ObDereferenceObjectDeferDelete(int p0) {}
void __stdcall ObDereferenceObjectDeferDeleteWithTag(int p0, int p1) {}
void __stdcall ObGetFilterVersion() {}
void __stdcall ObGetObjectSecurity(int p0, int p1, int p2) {}
void __stdcall ObInsertObject(int p0, int p1, int p2, int p3, int p4, int p5) {}
void __stdcall ObIsKernelHandle(int p0) {}
void __stdcall ObMakeTemporaryObject(int p0) {}
void __stdcall ObOpenObjectByPointer(int p0, int p1, int p2, int p3, int p4, int p5, int p6) {}
void __stdcall ObOpenObjectByPointerWithTag(int p0, int p1, int p2, int p3, int p4, int p5, int p6, int p7) {}
void __stdcall ObQueryNameString(int p0, int p1, int p2, int p3) {}
void __stdcall ObQueryObjectAuditingByHandle(int p0, int p1) {}
void __stdcall ObReferenceObjectByHandle(int p0, int p1, int p2, int p3, int p4, int p5) {}
void __stdcall ObReferenceObjectByHandleWithTag(int p0, int p1, int p2, int p3, int p4, int p5, int p6) {}
void __stdcall ObReferenceObjectByPointer(int p0, int p1, int p2, int p3) {}
void __stdcall ObReferenceObjectByPointerWithTag(int p0, int p1, int p2, int p3, int p4) {}
void __stdcall ObReferenceObjectSafe(int p0) {}
void __stdcall ObReferenceObjectSafeWithTag(int p0, int p1) {}
void __stdcall ObRegisterCallbacks(int p0, int p1) {}
void __stdcall ObReleaseObjectSecurity(int p0, int p1) {}
void __stdcall ObUnRegisterCallbacks(int p0) {}
void __stdcall ObfDereferenceObject(int p0) {}
void __stdcall ObfDereferenceObjectWithTag(int p0, int p1) {}
void __stdcall ObfReferenceObject(int p0) {}
void __stdcall ObfReferenceObjectWithTag(int p0, int p1) {}
void __stdcall PcwAddInstance(int p0, int p1, int p2, int p3, int p4) {}
void __stdcall PcwCloseInstance(int p0) {}
void __stdcall PcwCreateInstance(int p0, int p1, int p2, int p3, int p4) {}
void __stdcall PcwRegister(int p0, int p1) {}
void __stdcall PcwUnregister(int p0) {}
void __stdcall PoCallDriver(int p0, int p1) {}
void __stdcall PoClearPowerRequest(int p0, int p1) {}
void __stdcall PoCreatePowerRequest(int p0, int p1, int p2) {}
void __stdcall PoCreateThermalRequest(int p0, int p1, int p2, int p3, int p4) {}
void __stdcall PoDeletePowerRequest(int p0) {}
void __stdcall PoDeleteThermalRequest(int p0) {}
void __stdcall PoEndDeviceBusy(int p0) {}
void __stdcall PoFxActivateComponent(int p0, int p1, int p2) {}
void __stdcall PoFxCompleteDevicePowerNotRequired(int p0) {}
void __stdcall PoFxCompleteDirectedPowerDown(int p0) {}
void __stdcall PoFxCompleteIdleCondition(int p0, int p1) {}
void __stdcall PoFxCompleteIdleState(int p0, int p1) {}
void __stdcall PoFxIdleComponent(int p0, int p1, int p2) {}
void __stdcall PoFxIssueComponentPerfStateChange(int p0, int p1, int p2, int p3, int p4) {}
void __stdcall PoFxIssueComponentPerfStateChangeMultiple(int p0, int p1, int p2, int p3, int p4, int p5) {}
void __stdcall PoFxNotifySurprisePowerOn(int p0) {}
void __stdcall PoFxPowerControl(int p0, int p1, int p2, int p3, int p4, int p5, int p6) {}
void __stdcall PoFxPowerOnCrashdumpDevice(int p0, int p1) {}
void __stdcall PoFxQueryCurrentComponentPerfState(int p0, int p1, int p2, int p3, int p4) {}
void __stdcall PoFxRegisterComponentPerfStates(int p0, int p1, int p2, int p3, int p4, int p5, int p6) {}
void __stdcall PoFxRegisterCrashdumpDevice(int p0) {}
void __stdcall PoFxRegisterDevice(int p0, int p1, int p2) {}
void __stdcall PoFxRegisterDripsWatchdogCallback(int p0, int p1, int p2, int p3) {}
void __stdcall PoFxReportDevicePoweredOn(int p0) {}
void __stdcall PoFxSetComponentLatency(int p0, int p1, int p2, int p3) {}
void __stdcall PoFxSetComponentResidency(int p0, int p1, int p2, int p3) {}
void __stdcall PoFxSetComponentWake(int p0, int p1, int p2) {}
void __stdcall PoFxSetDeviceIdleTimeout(int p0, int p1, int p2) {}
void __stdcall PoFxSetTargetDripsDevicePowerState(int p0, int p1) {}
void __stdcall PoFxStartDevicePowerManagement(int p0) {}
void __stdcall PoFxUnregisterDevice(int p0) {}
void __stdcall PoGetSystemWake(int p0) {}
void __stdcall PoGetThermalRequestSupport(int p0, int p1) {}
void __stdcall PoQueryWatchdogTime(int p0, int p1) {}
void __stdcall PoQueueShutdownWorkItem(int p0) {}
void __stdcall PoRegisterDeviceForIdleDetection(int p0, int p1, int p2, int p3) {}
void __stdcall PoRegisterPowerSettingCallback(int p0, int p1, int p2, int p3, int p4) {}
void __stdcall PoRegisterSystemState(int p0, int p1) {}
void __stdcall PoRequestPowerIrp(int p0, int p1, int p2, int p3, int p4, int p5) {}
void __stdcall PoSetDeviceBusyEx(int p0) {}
void __stdcall PoSetHiberRange(int p0, int p1, int p2, int p3, int p4) {}
void __stdcall PoSetPowerRequest(int p0, int p1) {}
void __stdcall PoSetPowerState(int p0, int p1, int p2) {}
void __stdcall PoSetSystemState(int p0) {}
void __stdcall PoSetSystemWake(int p0) {}
void __stdcall PoSetSystemWakeDevice(int p0) {}
void __stdcall PoSetThermalActiveCooling(int p0, int p1) {}
void __stdcall PoSetThermalPassiveCooling(int p0, int p1) {}
void __stdcall PoStartDeviceBusy(int p0) {}
void __stdcall PoStartNextPowerIrp(int p0) {}
void __stdcall PoUnregisterPowerSettingCallback(int p0) {}
void __stdcall PoUnregisterSystemState(int p0) {}
void __stdcall ProbeForRead(int p0, int p1, int p2) {}
void __stdcall ProbeForWrite(int p0, int p1, int p2) {}
void __stdcall PsAcquireSiloHardReference(int p0) {}
void __stdcall PsAllocSiloContextSlot(int p0, int p1) {}
void __stdcall PsAllocateAffinityToken(int p0) {}
void __stdcall PsAssignImpersonationToken(int p0, int p1) {}
void __stdcall PsAttachSiloToCurrentThread(int p0) {}
void __stdcall PsChargePoolQuota(int p0, int p1, int p2) {}
void __stdcall PsChargeProcessPoolQuota(int p0, int p1, int p2) {}
void __stdcall PsCreateSiloContext(int p0, int p1, int p2, int p3, int p4) {}
void __stdcall PsCreateSystemThread(int p0, int p1, int p2, int p3, int p4, int p5, int p6) {}
void __stdcall PsDereferenceImpersonationToken(int p0) {}
void __stdcall PsDereferencePrimaryToken(int p0) {}
void __stdcall PsDereferenceSiloContext(int p0) {}
void __stdcall PsDetachSiloFromCurrentThread(int p0) {}
void __stdcall PsDisableImpersonation(int p0, int p1) {}
void __stdcall PsFreeAffinityToken(int p0) {}
void __stdcall PsFreeSiloContextSlot(int p0) {}
void __stdcall PsGetCurrentProcessId() {}
void __stdcall PsGetCurrentServerSilo() {}
void __stdcall PsGetCurrentServerSiloName() {}
void __stdcall PsGetCurrentSilo() {}
void __stdcall PsGetCurrentThreadId() {}
void __stdcall PsGetCurrentThreadTeb() {}
void __stdcall PsGetEffectiveServerSilo(int p0) {}
void __stdcall PsGetHostSilo() {}
void __stdcall PsGetJobServerSilo(int p0, int p1) {}
void __stdcall PsGetJobSilo(int p0, int p1) {}
void __stdcall PsGetParentSilo(int p0) {}
void __stdcall PsGetPermanentSiloContext(int p0, int p1, int p2) {}
void __stdcall PsGetProcessCreateTimeQuadPart(int p0) {}
void __stdcall PsGetProcessExitStatus(int p0) {}
void __stdcall PsGetProcessExitTime() {}
void __stdcall PsGetProcessId(int p0) {}
void __stdcall PsGetProcessStartKey(int p0) {}
void __stdcall PsGetServerSiloServiceSessionId(int p0) {}
void __stdcall PsGetSiloContainerId(int p0) {}
void __stdcall PsGetSiloContext(int p0, int p1, int p2) {}
void __stdcall PsGetSiloMonitorContextSlot(int p0) {}
void __stdcall PsGetThreadCreateTime(int p0) {}
void __stdcall PsGetThreadExitStatus(int p0) {}
void __stdcall PsGetThreadId(int p0) {}
void __stdcall PsGetThreadProcess(int p0) {}
void __stdcall PsGetThreadProcessId(int p0) {}
void __stdcall PsGetThreadProperty(int p0, int p1, int p2) {}
void __stdcall PsGetThreadServerSilo(int p0) {}
void __stdcall PsGetVersion(int p0, int p1, int p2, int p3) {}
void __stdcall PsImpersonateClient(int p0, int p1, int p2, int p3, int p4) {}
void __stdcall PsInsertPermanentSiloContext(int p0, int p1, int p2) {}
void __stdcall PsInsertSiloContext(int p0, int p1, int p2) {}
void __stdcall PsIsCurrentThreadInServerSilo() {}
void __stdcall PsIsCurrentThreadPrefetching() {}
void __stdcall PsIsDiskCountersEnabled() {}
void __stdcall PsIsHostSilo(int p0) {}
void __stdcall PsIsSystemThread(int p0) {}
void __stdcall PsIsThreadTerminating(int p0) {}
void __stdcall PsLookupProcessByProcessId(int p0, int p1) {}
void __stdcall PsLookupThreadByThreadId(int p0, int p1) {}
void __stdcall PsMakeSiloContextPermanent(int p0, int p1) {}
void __stdcall PsQueryTotalCycleTimeProcess(int p0, int p1) {}
void __stdcall PsReferenceImpersonationToken(int p0, int p1, int p2, int p3) {}
void __stdcall PsReferencePrimaryToken(int p0) {}
void __stdcall PsReferenceSiloContext(int p0) {}
void __stdcall PsRegisterSiloMonitor(int p0, int p1) {}
void __stdcall PsReleaseSiloHardReference(int p0) {}
void __stdcall PsRemoveCreateThreadNotifyRoutine(int p0) {}
void __stdcall PsRemoveLoadImageNotifyRoutine(int p0) {}
void __stdcall PsRemoveSiloContext(int p0, int p1, int p2) {}
void __stdcall PsReplaceSiloContext(int p0, int p1, int p2, int p3) {}
void __stdcall PsRestoreImpersonation(int p0, int p1) {}
void __stdcall PsReturnPoolQuota(int p0, int p1, int p2) {}
void __stdcall PsRevertToSelf() {}
void __stdcall PsRevertToUserMultipleGroupAffinityThread(int p0) {}
void __stdcall PsSetCreateProcessNotifyRoutine(int p0, int p1) {}
void __stdcall PsSetCreateProcessNotifyRoutineEx(int p0, int p1) {}
void __stdcall PsSetCreateProcessNotifyRoutineEx2(int p0, int p1, int p2) {}
void __stdcall PsSetCreateThreadNotifyRoutine(int p0) {}
void __stdcall PsSetCreateThreadNotifyRoutineEx(int p0, int p1) {}
void __stdcall PsSetCurrentThreadPrefetching(int p0) {}
void __stdcall PsSetLoadImageNotifyRoutine(int p0) {}
void __stdcall PsSetLoadImageNotifyRoutineEx(int p0, int p1) {}
void __stdcall PsSetSystemMultipleGroupAffinityThread(int p0, int p1, int p2) {}
void __stdcall PsStartSiloMonitor(int p0) {}
void __stdcall PsTerminateServerSilo(int p0, int p1) {}
void __stdcall PsTerminateSystemThread(int p0) {}
void __stdcall PsUnregisterSiloMonitor(int p0) {}
void __stdcall PsUpdateDiskCounters(int p0, int p1, int p2, int p3, int p4, int p5, int p6, int p7) {}
void __stdcall PsWrapApcWow64Thread(int p0, int p1) {}
void __stdcall RtlCompressChunks(int p0, int p1, int p2, int p3, int p4, int p5, int p6) {}
void __stdcall RtlDecompressBufferEx2(int p0, int p1, int p2, int p3, int p4, int p5, int p6, int p7) {}
void __stdcall RtlDecompressChunks(int p0, int p1, int p2, int p3, int p4, int p5, int p6) {}
void __stdcall RtlDecompressFragmentEx(int p0, int p1, int p2, int p3, int p4, int p5, int p6, int p7, int p8) {}
void __stdcall RtlDescribeChunk(int p0, int p1, int p2, int p3, int p4) {}
void __stdcall RtlFindFirstRunClear(int p0, int p1) {}
void __stdcall RtlFindUnicodePrefix(int p0, int p1, int p2) {}
void __stdcall RtlGenerateClass5Guid(int p0, int p1, int p2, int p3) {}
void __stdcall RtlInitializeUnicodePrefix(int p0) {}
void __stdcall RtlInsertUnicodePrefix(int p0, int p1, int p2) {}
void __stdcall RtlIsNtDdiVersionAvailable(int p0) {}
void __stdcall RtlIsSandboxedToken(int p0, int p1) {}
void __stdcall RtlIsServicePackVersionInstalled(int p0) {}
void __stdcall RtlIsValidOemCharacter(int p0) {}
void __stdcall RtlNextUnicodePrefix(int p0, int p1) {}
void __stdcall RtlOemStringToCountedUnicodeString(int p0, int p1, int p2) {}
void __stdcall RtlPrefetchMemoryNonTemporal(int p0, int p1) {}
void __stdcall RtlRemoveUnicodePrefix(int p0, int p1) {}
void __stdcall RtlReserveChunk(int p0, int p1, int p2, int p3, int p4) {}
void __stdcall RtlSetSystemGlobalData(int p0, int p1, int p2) {}
void __stdcall RtlSuffixUnicodeString(int p0, int p1, int p2) {}
void __stdcall RtlUnicodeStringToInt64(int p0, int p1, int p2, int p3) {}
void __stdcall RtlVolumeDeviceToDosName(int p0, int p1) {}
void __stdcall SeAccessCheck(int p0, int p1, int p2, int p3, int p4, int p5, int p6, int p7, int p8, int p9) {}
void __stdcall SeAccessCheckFromState(int p0, int p1, int p2, int p3, int p4, int p5, int p6, int p7, int p8, int p9) {}
void __stdcall SeAccessCheckFromStateEx(int p0, int p1, int p2, int p3, int p4, int p5, int p6, int p7, int p8, int p9) {}
void __stdcall SeAdjustAccessStateForAccessConstraints(int p0, int p1, int p2) {}
void __stdcall SeAdjustAccessStateForTrustLabel(int p0, int p1, int p2) {}
void __stdcall SeAdjustObjectSecurity(int p0, int p1, int p2, int p3, int p4, int p5) {}
void __stdcall SeAppendPrivileges(int p0, int p1) {}
void __stdcall SeAssignSecurity(int p0, int p1, int p2, int p3, int p4, int p5, int p6) {}
void __stdcall SeAssignSecurityEx(int p0, int p1, int p2, int p3, int p4, int p5, int p6, int p7, int p8) {}
void __stdcall SeAuditFipsCryptoSelftests(int p0, int p1) {}
void __stdcall SeAuditHardLinkCreation(int p0, int p1, int p2) {}
void __stdcall SeAuditHardLinkCreationWithTransaction(int p0, int p1, int p2, int p3) {}
void __stdcall SeAuditTransactionStateChange(int p0, int p1, int p2) {}
void __stdcall SeAuditingAnyFileEventsWithContext(int p0, int p1) {}
void __stdcall SeAuditingAnyFileEventsWithContextEx(int p0, int p1, int p2) {}
void __stdcall SeAuditingFileEvents(int p0, int p1) {}
void __stdcall SeAuditingFileEventsWithContext(int p0, int p1, int p2) {}
void __stdcall SeAuditingFileEventsWithContextEx(int p0, int p1, int p2, int p3) {}
void __stdcall SeAuditingFileOrGlobalEvents(int p0, int p1, int p2) {}
void __stdcall SeAuditingHardLinkEvents(int p0, int p1) {}
void __stdcall SeAuditingHardLinkEventsWithContext(int p0, int p1, int p2) {}
void __stdcall SeCaptureSubjectContext(int p0) {}
void __stdcall SeCaptureSubjectContextEx(int p0, int p1, int p2) {}
void __stdcall SeCheckForCriticalAceRemoval(int p0, int p1, int p2, int p3) {}
void __stdcall SeComputeAutoInheritByObjectType(int p0, int p1, int p2) {}
void __stdcall SeCreateClientSecurity(int p0, int p1, int p2, int p3) {}
void __stdcall SeCreateClientSecurityFromSubjectContext(int p0, int p1, int p2, int p3) {}
void __stdcall SeDeassignSecurity(int p0) {}
void __stdcall SeDeleteClientSecurity(int p0) {}
void __stdcall SeDeleteObjectAuditAlarm(int p0, int p1) {}
void __stdcall SeDeleteObjectAuditAlarmWithTransaction(int p0, int p1, int p2) {}
void __stdcall SeEtwWriteKMCveEvent(int p0, int p1) {}
void __stdcall SeExamineSacl(int p0, int p1, int p2, int p3, int p4, int p5, int p6) {}
void __stdcall SeFilterToken(int p0, int p1, int p2, int p3, int p4, int p5) {}
void __stdcall SeFreePrivileges(int p0) {}
void __stdcall SeImpersonateClient(int p0, int p1) {}
void __stdcall SeImpersonateClientEx(int p0, int p1) {}
void __stdcall SeLocateProcessImageName(int p0, int p1) {}
void __stdcall SeLockSubjectContext(int p0) {}
void __stdcall SeMarkLogonSessionForTerminationNotification(int p0) {}
void __stdcall SeMarkLogonSessionForTerminationNotificationEx(int p0, int p1) {}
void __stdcall SeOpenObjectAuditAlarm(int p0, int p1, int p2, int p3, int p4, int p5, int p6, int p7, int p8) {}
void __stdcall SeOpenObjectAuditAlarmWithTransaction(int p0, int p1, int p2, int p3, int p4, int p5, int p6, int p7, int p8, int p9) {}
void __stdcall SeOpenObjectForDeleteAuditAlarm(int p0, int p1, int p2, int p3, int p4, int p5, int p6, int p7, int p8) {}
void __stdcall SeOpenObjectForDeleteAuditAlarmWithTransaction(int p0, int p1, int p2, int p3, int p4, int p5, int p6, int p7, int p8, int p9) {}
void __stdcall SePrivilegeCheck(int p0, int p1, int p2) {}
void __stdcall SeQueryAuthenticationIdToken(int p0, int p1) {}
void __stdcall SeQueryInformationToken(int p0, int p1, int p2) {}
void __stdcall SeQuerySecurityDescriptorInfo(int p0, int p1, int p2, int p3) {}
void __stdcall SeQueryServerSiloToken(int p0, int p1) {}
void __stdcall SeQuerySessionIdToken(int p0, int p1) {}
void __stdcall SeQuerySessionIdTokenEx(int p0, int p1, int p2) {}
void __stdcall SeRegisterImageVerificationCallback(int p0, int p1, int p2, int p3, int p4, int p5) {}
void __stdcall SeRegisterLogonSessionTerminatedRoutine(int p0) {}
void __stdcall SeRegisterLogonSessionTerminatedRoutineEx(int p0, int p1) {}
void __stdcall SeReleaseSubjectContext(int p0) {}
void __stdcall SeReportSecurityEvent(int p0, int p1, int p2, int p3) {}
void __stdcall SeReportSecurityEventWithSubCategory(int p0, int p1, int p2, int p3, int p4) {}
void __stdcall SeSetAccessStateGenericMapping(int p0, int p1) {}
void __stdcall SeSetAuditParameter(int p0, int p1, int p2, int p3) {}
void __stdcall SeSetSecurityDescriptorInfo(int p0, int p1, int p2, int p3, int p4, int p5) {}
void __stdcall SeSetSecurityDescriptorInfoEx(int p0, int p1, int p2, int p3, int p4, int p5, int p6) {}
void __stdcall SeShouldCheckForAccessRightsFromParent(int p0, int p1, int p2) {}
void __stdcall SeSinglePrivilegeCheck(int p0, int p1, int p2) {}
void __stdcall SeTokenFromAccessInformation(int p0, int p1, int p2, int p3) {}
void __stdcall SeTokenIsAdmin(int p0) {}
void __stdcall SeTokenIsRestricted(int p0) {}
void __stdcall SeTokenIsWriteRestricted(int p0) {}
void __stdcall SeTokenType(int p0) {}
void __stdcall SeUnlockSubjectContext(int p0) {}
void __stdcall SeUnregisterImageVerificationCallback(int p0) {}
void __stdcall SeUnregisterLogonSessionTerminatedRoutine(int p0) {}
void __stdcall SeUnregisterLogonSessionTerminatedRoutineEx(int p0, int p1) {}
void __stdcall SeValidSecurityDescriptor(int p0, int p1) {}
void __stdcall TmCommitComplete(int p0, int p1) {}
void __stdcall TmCommitEnlistment(int p0, int p1) {}
void __stdcall TmCommitTransaction(int p0, int p1) {}
void __stdcall TmCreateEnlistment(int p0, int p1, int p2, int p3, int p4, int p5, int p6, int p7, int p8) {}
void __stdcall TmDereferenceEnlistmentKey(int p0, int p1) {}
void __stdcall TmEnableCallbacks(int p0, int p1, int p2) {}
void __stdcall TmGetTransactionId(int p0, int p1) {}
void __stdcall TmInitializeTransactionManager(int p0, int p1, int p2, int p3) {}
void __stdcall TmIsTransactionActive(int p0) {}
void __stdcall TmPrePrepareComplete(int p0, int p1) {}
void __stdcall TmPrePrepareEnlistment(int p0, int p1) {}
void __stdcall TmPrepareComplete(int p0, int p1) {}
void __stdcall TmPrepareEnlistment(int p0, int p1) {}
void __stdcall TmPropagationComplete(int p0, int p1, int p2, int p3) {}
void __stdcall TmPropagationFailed(int p0, int p1, int p2) {}
void __stdcall TmReadOnlyEnlistment(int p0, int p1) {}
void __stdcall TmRecoverEnlistment(int p0, int p1) {}
void __stdcall TmRecoverResourceManager(int p0) {}
void __stdcall TmRecoverTransactionManager(int p0, int p1) {}
void __stdcall TmReferenceEnlistmentKey(int p0, int p1) {}
void __stdcall TmRenameTransactionManager(int p0, int p1) {}
void __stdcall TmRequestOutcomeEnlistment(int p0, int p1) {}
void __stdcall TmRollbackComplete(int p0, int p1) {}
void __stdcall TmRollbackEnlistment(int p0, int p1) {}
void __stdcall TmRollbackTransaction(int p0, int p1) {}
void __stdcall TmSinglePhaseReject(int p0, int p1) {}
void __stdcall VslCreateSecureSection(int p0, int p1, int p2, int p3, int p4) {}
void __stdcall VslDeleteSecureSection(int p0) {}
void __stdcall WheaAddErrorSource(int p0, int p1) {}
void __stdcall WheaAddErrorSourceDeviceDriver(int p0, int p1, int p2) {}
void __stdcall WheaAddErrorSourceDeviceDriverV1(int p0, int p1, int p2, int p3) {}
void __stdcall WheaAddHwErrorReportSectionDeviceDriver(int p0, int p1, int p2) {}
void __stdcall WheaConfigureErrorSource(int p0, int p1) {}
void __stdcall WheaCreateHwErrorReportDeviceDriver(int p0, int p1) {}
void __stdcall WheaErrorSourceGetState(int p0) {}
void __stdcall WheaGetNotifyAllOfflinesPolicy() {}
void __stdcall WheaHighIrqlLogSelEventHandlerRegister(int p0, int p1) {}
void __stdcall WheaHighIrqlLogSelEventHandlerUnregister() {}
void __stdcall WheaHwErrorReportAbandonDeviceDriver(int p0) {}
void __stdcall WheaHwErrorReportSetSectionNameDeviceDriver(int p0, int p1, int p2) {}
void __stdcall WheaHwErrorReportSetSeverityDeviceDriver(int p0, int p1) {}
void __stdcall WheaHwErrorReportSubmitDeviceDriver(int p0) {}
void __stdcall WheaInitializeRecordHeader(int p0) {}
void __stdcall WheaIsCriticalState() {}
void __stdcall WheaLogInternalEvent(int p0) {}
void __stdcall WheaRegisterInUsePageOfflineNotification(int p0, int p1) {}
void __stdcall WheaRemoveErrorSource(int p0) {}
void __stdcall WheaRemoveErrorSourceDeviceDriver(int p0) {}
void __stdcall WheaReportHwError(int p0) {}
void __stdcall WheaReportHwErrorDeviceDriver(int p0, int p1, int p2, int p3, int p4, int p5, int p6) {}
void __stdcall WheaUnconfigureErrorSource(int p0) {}
void __stdcall WheaUnregisterInUsePageOfflineNotification(int p0) {}
void __stdcall WmiQueryTraceInformation(int p0, int p1, int p2, int p3, int p4) {}

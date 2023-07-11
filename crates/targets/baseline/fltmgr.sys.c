void __stdcall FltAcknowledgeEcp(int p0, int p1) {}
void __stdcall FltAcquirePushLockExclusive(int p0) {}
void __stdcall FltAcquirePushLockExclusiveEx(int p0, int p1) {}
void __stdcall FltAcquirePushLockShared(int p0) {}
void __stdcall FltAcquirePushLockSharedEx(int p0, int p1) {}
void __stdcall FltAcquireResourceExclusive(int p0) {}
void __stdcall FltAcquireResourceShared(int p0) {}
void __stdcall FltAddOpenReparseEntry(int p0, int p1, int p2) {}
void __stdcall FltAdjustDeviceStackSizeForIoRedirection(int p0, int p1, int p2) {}
void __stdcall FltAllocateCallbackData(int p0, int p1, int p2) {}
void __stdcall FltAllocateCallbackDataEx(int p0, int p1, int p2, int p3) {}
void __stdcall FltAllocateContext(int p0, int p1, int p2, int p3, int p4) {}
void __stdcall FltAllocateDeferredIoWorkItem() {}
void __stdcall FltAllocateExtraCreateParameter(int p0, int p1, int p2, int p3, int p4, int p5, int p6) {}
void __stdcall FltAllocateExtraCreateParameterFromLookasideList(int p0, int p1, int p2, int p3, int p4, int p5, int p6) {}
void __stdcall FltAllocateExtraCreateParameterList(int p0, int p1, int p2) {}
void __stdcall FltAllocateFileLock(int p0, int p1) {}
void __stdcall FltAllocateGenericWorkItem() {}
void __stdcall FltAllocatePoolAlignedWithTag(int p0, int p1, int p2, int p3) {}
void __stdcall FltApplyPriorityInfoThread(int p0, int p1, int p2) {}
void __stdcall FltAttachVolume(int p0, int p1, int p2, int p3) {}
void __stdcall FltAttachVolumeAtAltitude(int p0, int p1, int p2, int p3, int p4) {}
void __stdcall FltBuildDefaultSecurityDescriptor(int p0, int p1) {}
void __stdcall FltCancelFileOpen(int p0, int p1) {}
void __stdcall FltCancelIo(int p0) {}
void __stdcall FltCancellableWaitForMultipleObjects(int p0, int p1, int p2, int p3, int p4, int p5) {}
void __stdcall FltCancellableWaitForSingleObject(int p0, int p1, int p2) {}
void __stdcall FltCbdqDisable(int p0) {}
void __stdcall FltCbdqEnable(int p0) {}
void __stdcall FltCbdqInitialize(int p0, int p1, int p2, int p3, int p4, int p5, int p6, int p7) {}
void __stdcall FltCbdqInsertIo(int p0, int p1, int p2, int p3) {}
void __stdcall FltCbdqRemoveIo(int p0, int p1) {}
void __stdcall FltCbdqRemoveNextIo(int p0, int p1) {}
void __stdcall FltCheckAndGrowNameControl(int p0, int p1) {}
void __stdcall FltCheckLockForReadAccess(int p0, int p1) {}
void __stdcall FltCheckLockForWriteAccess(int p0, int p1) {}
void __stdcall FltCheckOplock(int p0, int p1, int p2, int p3, int p4) {}
void __stdcall FltCheckOplockEx(int p0, int p1, int p2, int p3, int p4, int p5) {}
void __stdcall FltClearCallbackDataDirty(int p0) {}
void __stdcall FltClearCancelCompletion(int p0) {}
void __stdcall FltClose(int p0) {}
void __stdcall FltCloseClientPort(int p0, int p1) {}
void __stdcall FltCloseCommunicationPort(int p0) {}
void __stdcall FltCloseSectionForDataScan(int p0) {}
void __stdcall FltCommitComplete(int p0, int p1, int p2) {}
void __stdcall FltCommitFinalizeComplete(int p0, int p1, int p2) {}
void __stdcall FltCompareInstanceAltitudes(int p0, int p1) {}
void __stdcall FltCompletePendedPostOperation(int p0) {}
void __stdcall FltCompletePendedPreOperation(int p0, int p1, int p2) {}
void __stdcall FltCopyOpenReparseList(int p0, int p1, int p2) {}
void __stdcall FltCreateCommunicationPort(int p0, int p1, int p2, int p3, int p4, int p5, int p6, int p7) {}
void __stdcall FltCreateFile(int p0, int p1, int p2, int p3, int p4, int p5, int p6, int p7, int p8, int p9, int p10, int p11, int p12, int p13) {}
void __stdcall FltCreateFileEx(int p0, int p1, int p2, int p3, int p4, int p5, int p6, int p7, int p8, int p9, int p10, int p11, int p12, int p13, int p14) {}
void __stdcall FltCreateFileEx2(int p0, int p1, int p2, int p3, int p4, int p5, int p6, int p7, int p8, int p9, int p10, int p11, int p12, int p13, int p14, int p15) {}
void __stdcall FltCreateMailslotFile(int p0, int p1, int p2, int p3, int p4, int p5, int p6, int p7, int p8, int p9, int p10, int p11) {}
void __stdcall FltCreateNamedPipeFile(int p0, int p1, int p2, int p3, int p4, int p5, int p6, int p7, int p8, int p9, int p10, int p11, int p12, int p13, int p14, int p15, int p16, int p17) {}
void __stdcall FltCreateSectionForDataScan(int p0, int p1, int p2, int p3, int p4, int p5, int p6, int p7, int p8, int p9, int p10, int p11) {}
void __stdcall FltCreateSystemVolumeInformationFolder(int p0) {}
void __stdcall FltCurrentBatchOplock(int p0) {}
void __stdcall FltCurrentOplock(int p0) {}
void __stdcall FltCurrentOplockH(int p0) {}
void __stdcall FltDecodeParameters(int p0, int p1, int p2, int p3, int p4) {}
void __stdcall FltDeleteContext(int p0) {}
void __stdcall FltDeleteExtraCreateParameterLookasideList(int p0, int p1, int p2) {}
void __stdcall FltDeleteFileContext(int p0, int p1, int p2) {}
void __stdcall FltDeleteInstanceContext(int p0, int p1) {}
void __stdcall FltDeletePushLock(int p0) {}
void __stdcall FltDeleteStreamContext(int p0, int p1, int p2) {}
void __stdcall FltDeleteStreamHandleContext(int p0, int p1, int p2) {}
void __stdcall FltDeleteTransactionContext(int p0, int p1, int p2) {}
void __stdcall FltDeleteVolumeContext(int p0, int p1, int p2) {}
void __stdcall FltDetachVolume(int p0, int p1, int p2) {}
void __stdcall FltDeviceIoControlFile(int p0, int p1, int p2, int p3, int p4, int p5, int p6, int p7) {}
void __stdcall FltDoCompletionProcessingWhenSafe(int p0, int p1, int p2, int p3, int p4, int p5) {}
void __stdcall FltEnlistInTransaction(int p0, int p1, int p2, int p3) {}
void __stdcall FltEnumerateFilterInformation(int p0, int p1, int p2, int p3, int p4) {}
void __stdcall FltEnumerateFilters(int p0, int p1, int p2) {}
void __stdcall FltEnumerateInstanceInformationByDeviceObject(int p0, int p1, int p2, int p3, int p4, int p5) {}
void __stdcall FltEnumerateInstanceInformationByFilter(int p0, int p1, int p2, int p3, int p4, int p5) {}
void __stdcall FltEnumerateInstanceInformationByVolume(int p0, int p1, int p2, int p3, int p4, int p5) {}
void __stdcall FltEnumerateInstanceInformationByVolumeName(int p0, int p1, int p2, int p3, int p4, int p5) {}
void __stdcall FltEnumerateInstances(int p0, int p1, int p2, int p3, int p4) {}
void __stdcall FltEnumerateVolumeInformation(int p0, int p1, int p2, int p3, int p4, int p5) {}
void __stdcall FltEnumerateVolumes(int p0, int p1, int p2, int p3) {}
void __stdcall FltFastIoMdlRead(int p0, int p1, int p2, int p3, int p4, int p5, int p6) {}
void __stdcall FltFastIoMdlReadComplete(int p0, int p1, int p2) {}
void __stdcall FltFastIoMdlWriteComplete(int p0, int p1, int p2, int p3) {}
void __stdcall FltFastIoPrepareMdlWrite(int p0, int p1, int p2, int p3, int p4, int p5, int p6) {}
void __stdcall FltFindExtraCreateParameter(int p0, int p1, int p2, int p3, int p4) {}
void __stdcall FltFlushBuffers(int p0, int p1) {}
void __stdcall FltFlushBuffers2(int p0, int p1, int p2, int p3) {}
void __stdcall FltFreeCallbackData(int p0) {}
void __stdcall FltFreeDeferredIoWorkItem(int p0) {}
void __stdcall FltFreeExtraCreateParameter(int p0, int p1) {}
void __stdcall FltFreeExtraCreateParameterList(int p0, int p1) {}
void __stdcall FltFreeFileLock(int p0) {}
void __stdcall FltFreeGenericWorkItem(int p0) {}
void __stdcall FltFreeOpenReparseList(int p0, int p1) {}
void __stdcall FltFreePoolAlignedWithTag(int p0, int p1, int p2) {}
void __stdcall FltFreeSecurityDescriptor(int p0) {}
void __stdcall FltFsControlFile(int p0, int p1, int p2, int p3, int p4, int p5, int p6, int p7) {}
void __stdcall FltGetActivityIdCallbackData(int p0, int p1) {}
void __stdcall FltGetBottomInstance(int p0, int p1) {}
void __stdcall FltGetContexts(int p0, int p1, int p2) {}
void __stdcall FltGetContextsEx(int p0, int p1, int p2, int p3) {}
void __stdcall FltGetDestinationFileNameInformation(int p0, int p1, int p2, int p3, int p4, int p5, int p6) {}
void __stdcall FltGetDeviceObject(int p0, int p1) {}
void __stdcall FltGetDiskDeviceObject(int p0, int p1) {}
void __stdcall FltGetEcpListFromCallbackData(int p0, int p1, int p2) {}
void __stdcall FltGetFileContext(int p0, int p1, int p2) {}
void __stdcall FltGetFileNameInformation(int p0, int p1, int p2) {}
void __stdcall FltGetFileNameInformationUnsafe(int p0, int p1, int p2, int p3) {}
void __stdcall FltGetFileSystemType(int p0, int p1) {}
void __stdcall FltGetFilterFromInstance(int p0, int p1) {}
void __stdcall FltGetFilterFromName(int p0, int p1) {}
void __stdcall FltGetFilterInformation(int p0, int p1, int p2, int p3, int p4) {}
void __stdcall FltGetFsZeroingOffset(int p0, int p1) {}
void __stdcall FltGetInstanceContext(int p0, int p1) {}
void __stdcall FltGetInstanceInformation(int p0, int p1, int p2, int p3, int p4) {}
void __stdcall FltGetIoAttributionHandleFromCallbackData(int p0) {}
void __stdcall FltGetIoPriorityHint(int p0) {}
void __stdcall FltGetIoPriorityHintFromCallbackData(int p0) {}
void __stdcall FltGetIoPriorityHintFromFileObject(int p0) {}
void __stdcall FltGetIoPriorityHintFromThread(int p0) {}
void __stdcall FltGetIrpName(int p0) {}
void __stdcall FltGetLowerInstance(int p0, int p1) {}
void __stdcall FltGetNewSystemBufferAddress(int p0) {}
void __stdcall FltGetNextExtraCreateParameter(int p0, int p1, int p2, int p3, int p4, int p5) {}
void __stdcall FltGetRequestorProcess(int p0) {}
void __stdcall FltGetRequestorProcessId(int p0) {}
void __stdcall FltGetRequestorProcessIdEx(int p0) {}
void __stdcall FltGetRequestorSessionId(int p0, int p1) {}
void __stdcall FltGetRoutineAddress(int p0) {}
void __stdcall FltGetSectionContext(int p0, int p1, int p2) {}
void __stdcall FltGetStreamContext(int p0, int p1, int p2) {}
void __stdcall FltGetStreamHandleContext(int p0, int p1, int p2) {}
void __stdcall FltGetSwappedBufferMdlAddress(int p0) {}
void __stdcall FltGetTopInstance(int p0, int p1) {}
void __stdcall FltGetTransactionContext(int p0, int p1, int p2) {}
void __stdcall FltGetTunneledName(int p0, int p1, int p2) {}
void __stdcall FltGetUpperInstance(int p0, int p1) {}
void __stdcall FltGetVolumeContext(int p0, int p1, int p2) {}
void __stdcall FltGetVolumeFromDeviceObject(int p0, int p1, int p2) {}
void __stdcall FltGetVolumeFromFileObject(int p0, int p1, int p2) {}
void __stdcall FltGetVolumeFromInstance(int p0, int p1) {}
void __stdcall FltGetVolumeFromName(int p0, int p1, int p2) {}
void __stdcall FltGetVolumeGuidName(int p0, int p1, int p2) {}
void __stdcall FltGetVolumeInformation(int p0, int p1, int p2, int p3, int p4) {}
void __stdcall FltGetVolumeInstanceFromName(int p0, int p1, int p2, int p3) {}
void __stdcall FltGetVolumeName(int p0, int p1, int p2) {}
void __stdcall FltGetVolumeProperties(int p0, int p1, int p2, int p3) {}
void __stdcall FltInitExtraCreateParameterLookasideList(int p0, int p1, int p2, int p3, int p4) {}
void __stdcall FltInitializeFileLock(int p0) {}
void __stdcall FltInitializeOplock(int p0) {}
void __stdcall FltInitializePushLock(int p0) {}
void __stdcall FltInsertExtraCreateParameter(int p0, int p1, int p2) {}
void __stdcall FltIs32bitProcess(int p0) {}
void __stdcall FltIsCallbackDataDirty(int p0) {}
void __stdcall FltIsDirectory(int p0, int p1, int p2) {}
void __stdcall FltIsEcpAcknowledged(int p0, int p1) {}
void __stdcall FltIsEcpFromUserMode(int p0, int p1) {}
void __stdcall FltIsFltMgrVolumeDeviceObject(int p0) {}
void __stdcall FltIsIoCanceled(int p0) {}
void __stdcall FltIsIoRedirectionAllowed(int p0, int p1, int p2) {}
void __stdcall FltIsIoRedirectionAllowedForOperation(int p0, int p1, int p2, int p3) {}
void __stdcall FltIsOperationSynchronous(int p0) {}
void __stdcall FltIsVolumeSnapshot(int p0, int p1) {}
void __stdcall FltIsVolumeWritable(int p0, int p1) {}
void __stdcall FltLoadFilter(int p0) {}
void __stdcall FltLockUserBuffer(int p0) {}
void __stdcall FltNotifyFilterChangeDirectory(int p0, int p1, int p2, int p3, int p4, int p5, int p6, int p7, int p8, int p9, int p10) {}
void __stdcall FltObjectDereference(int p0) {}
void __stdcall FltObjectReference(int p0) {}
void __stdcall FltOpenVolume(int p0, int p1, int p2) {}
void __stdcall FltOplockBreakH(int p0, int p1, int p2, int p3, int p4, int p5) {}
void __stdcall FltOplockBreakToNone(int p0, int p1, int p2, int p3, int p4) {}
void __stdcall FltOplockBreakToNoneEx(int p0, int p1, int p2, int p3, int p4, int p5) {}
void __stdcall FltOplockFsctrl(int p0, int p1, int p2) {}
void __stdcall FltOplockFsctrlEx(int p0, int p1, int p2, int p3) {}
void __stdcall FltOplockIsFastIoPossible(int p0) {}
void __stdcall FltOplockIsSharedRequest(int p0) {}
void __stdcall FltOplockKeysEqual(int p0, int p1) {}
void __stdcall FltParseFileName(int p0, int p1, int p2, int p3) {}
void __stdcall FltParseFileNameInformation(int p0) {}
void __stdcall FltPerformAsynchronousIo(int p0, int p1, int p2) {}
void __stdcall FltPerformSynchronousIo(int p0) {}
void __stdcall FltPrePrepareComplete(int p0, int p1, int p2) {}
void __stdcall FltPrepareComplete(int p0, int p1, int p2) {}
void __stdcall FltPrepareToReuseEcp(int p0, int p1) {}
void __stdcall FltProcessFileLock(int p0, int p1, int p2) {}
void __stdcall FltPropagateActivityIdToThread(int p0, int p1, int p2) {}
void __stdcall FltPropagateIrpExtension(int p0, int p1, int p2) {}
void __stdcall FltPurgeFileNameInformationCache(int p0, int p1) {}
void __stdcall FltQueryDirectoryFile(int p0, int p1, int p2, int p3, int p4, int p5, int p6, int p7, int p8) {}
void __stdcall FltQueryDirectoryFileEx(int p0, int p1, int p2, int p3, int p4, int p5, int p6, int p7) {}
void __stdcall FltQueryEaFile(int p0, int p1, int p2, int p3, int p4, int p5, int p6, int p7, int p8, int p9) {}
void __stdcall FltQueryInformationByName(int p0, int p1, int p2, int p3, int p4, int p5, int p6, int p7) {}
void __stdcall FltQueryInformationFile(int p0, int p1, int p2, int p3, int p4, int p5) {}
void __stdcall FltQueryQuotaInformationFile(int p0, int p1, int p2, int p3, int p4, int p5, int p6, int p7, int p8, int p9, int p10) {}
void __stdcall FltQuerySecurityObject(int p0, int p1, int p2, int p3, int p4, int p5) {}
void __stdcall FltQueryVolumeInformation(int p0, int p1, int p2, int p3, int p4) {}
void __stdcall FltQueryVolumeInformationFile(int p0, int p1, int p2, int p3, int p4, int p5) {}
void __stdcall FltQueueDeferredIoWorkItem(int p0, int p1, int p2, int p3, int p4) {}
void __stdcall FltQueueGenericWorkItem(int p0, int p1, int p2, int p3, int p4) {}
void __stdcall FltReadFile(int p0, int p1, int p2, int p3, int p4, int p5, int p6, int p7, int p8) {}
void __stdcall FltReadFileEx(int p0, int p1, int p2, int p3, int p4, int p5, int p6, int p7, int p8, int p9, int p10) {}
void __stdcall FltReferenceContext(int p0) {}
void __stdcall FltReferenceFileNameInformation(int p0) {}
void __stdcall FltRegisterFilter(int p0, int p1, int p2) {}
void __stdcall FltRegisterForDataScan(int p0) {}
void __stdcall FltReissueSynchronousIo(int p0, int p1) {}
void __stdcall FltReleaseContext(int p0) {}
void __stdcall FltReleaseContexts(int p0) {}
void __stdcall FltReleaseContextsEx(int p0, int p1) {}
void __stdcall FltReleaseFileNameInformation(int p0) {}
void __stdcall FltReleasePushLock(int p0) {}
void __stdcall FltReleasePushLockEx(int p0, int p1) {}
void __stdcall FltReleaseResource(int p0) {}
void __stdcall FltRemoveExtraCreateParameter(int p0, int p1, int p2, int p3, int p4) {}
void __stdcall FltRemoveOpenReparseEntry(int p0, int p1, int p2) {}
void __stdcall FltRequestFileInfoOnCreateCompletion(int p0, int p1, int p2) {}
void __stdcall FltRequestOperationStatusCallback(int p0, int p1, int p2) {}
void __stdcall FltRetainSwappedBufferMdlAddress(int p0) {}
void __stdcall FltRetrieveFileInfoOnCreateCompletion(int p0, int p1, int p2, int p3) {}
void __stdcall FltRetrieveFileInfoOnCreateCompletionEx(int p0, int p1, int p2, int p3, int p4) {}
void __stdcall FltRetrieveIoPriorityInfo(int p0, int p1, int p2, int p3) {}
void __stdcall FltReuseCallbackData(int p0) {}
void __stdcall FltRollbackComplete(int p0, int p1, int p2) {}
void __stdcall FltRollbackEnlistment(int p0, int p1, int p2) {}
void __stdcall FltSendMessage(int p0, int p1, int p2, int p3, int p4, int p5, int p6) {}
void __stdcall FltSetActivityIdCallbackData(int p0, int p1) {}
void __stdcall FltSetCallbackDataDirty(int p0) {}
void __stdcall FltSetCancelCompletion(int p0, int p1) {}
void __stdcall FltSetEaFile(int p0, int p1, int p2, int p3) {}
void __stdcall FltSetEcpListIntoCallbackData(int p0, int p1, int p2) {}
void __stdcall FltSetFileContext(int p0, int p1, int p2, int p3, int p4) {}
void __stdcall FltSetFsZeroingOffset(int p0, int p1) {}
void __stdcall FltSetFsZeroingOffsetRequired(int p0) {}
void __stdcall FltSetInformationFile(int p0, int p1, int p2, int p3, int p4) {}
void __stdcall FltSetInstanceContext(int p0, int p1, int p2, int p3) {}
void __stdcall FltSetIoPriorityHintIntoCallbackData(int p0, int p1) {}
void __stdcall FltSetIoPriorityHintIntoFileObject(int p0, int p1) {}
void __stdcall FltSetIoPriorityHintIntoThread(int p0, int p1) {}
void __stdcall FltSetQuotaInformationFile(int p0, int p1, int p2, int p3) {}
void __stdcall FltSetSecurityObject(int p0, int p1, int p2, int p3) {}
void __stdcall FltSetStreamContext(int p0, int p1, int p2, int p3, int p4) {}
void __stdcall FltSetStreamHandleContext(int p0, int p1, int p2, int p3, int p4) {}
void __stdcall FltSetTransactionContext(int p0, int p1, int p2, int p3, int p4) {}
void __stdcall FltSetVolumeContext(int p0, int p1, int p2, int p3) {}
void __stdcall FltSetVolumeInformation(int p0, int p1, int p2, int p3, int p4) {}
void __stdcall FltStartFiltering(int p0) {}
void __stdcall FltSupportsFileContexts(int p0) {}
void __stdcall FltSupportsFileContextsEx(int p0, int p1) {}
void __stdcall FltSupportsStreamContexts(int p0) {}
void __stdcall FltSupportsStreamHandleContexts(int p0) {}
void __stdcall FltTagFile(int p0, int p1, int p2, int p3, int p4, int p5) {}
void __stdcall FltTagFileEx(int p0, int p1, int p2, int p3, int p4, int p5, int p6, int p7, int p8) {}
void __stdcall FltUninitializeFileLock(int p0) {}
void __stdcall FltUninitializeOplock(int p0) {}
void __stdcall FltUnloadFilter(int p0) {}
void __stdcall FltUnregisterFilter(int p0) {}
void __stdcall FltUntagFile(int p0, int p1, int p2, int p3) {}
void __stdcall FltVetoBypassIo(int p0, int p1, int p2, int p3) {}
void __stdcall FltWriteFile(int p0, int p1, int p2, int p3, int p4, int p5, int p6, int p7, int p8) {}
void __stdcall FltWriteFileEx(int p0, int p1, int p2, int p3, int p4, int p5, int p6, int p7, int p8, int p9, int p10) {}
void __stdcall FltpTraceRedirectedFileIo(int p0, int p1) {}

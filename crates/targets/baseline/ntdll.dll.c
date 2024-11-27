void __cdecl DbgPrint() {}
void __cdecl DbgPrintEx() {}
void __cdecl DbgPrintReturnControlC() {}
void __stdcall DbgPrompt(int p0, int p1, int p2) {}
void __stdcall DbgQueryDebugFilterState(int p0, int p1) {}
void __stdcall DbgSetDebugFilterState(int p0, int p1, int p2) {}
void __stdcall EtwEventEnabled(int p0, int p1, int p2) {}
void __stdcall NtAccessCheckAndAuditAlarm(int p0, int p1, int p2, int p3, int p4, int p5, int p6, int p7, int p8, int p9, int p10) {}
void __stdcall NtAccessCheckByTypeAndAuditAlarm(int p0, int p1, int p2, int p3, int p4, int p5, int p6, int p7, int p8, int p9, int p10, int p11, int p12, int p13, int p14, int p15) {}
void __stdcall NtAccessCheckByTypeResultListAndAuditAlarm(int p0, int p1, int p2, int p3, int p4, int p5, int p6, int p7, int p8, int p9, int p10, int p11, int p12, int p13, int p14, int p15) {}
void __stdcall NtAccessCheckByTypeResultListAndAuditAlarmByHandle(int p0, int p1, int p2, int p3, int p4, int p5, int p6, int p7, int p8, int p9, int p10, int p11, int p12, int p13, int p14, int p15, int p16) {}
void __stdcall NtAdjustGroupsToken(int p0, int p1, int p2, int p3, int p4, int p5) {}
void __stdcall NtAdjustPrivilegesToken(int p0, int p1, int p2, int p3, int p4, int p5) {}
void __stdcall NtAllocateLocallyUniqueId(int p0) {}
void __stdcall NtAllocateVirtualMemory(int p0, int p1, int p2, int p3, int p4, int p5) {}
void __stdcall NtAllocateVirtualMemoryEx(int p0, int p1, int p2, int p3, int p4, int p5, int p6) {}
void __stdcall NtCancelIoFileEx(int p0, int p1, int p2) {}
void __stdcall NtCancelTimer(int p0, int p1) {}
void __stdcall NtClose(int p0) {}
void __stdcall NtCloseObjectAuditAlarm(int p0, int p1, int p2) {}
void __stdcall NtCommitComplete(int p0, int p1) {}
void __stdcall NtCommitEnlistment(int p0, int p1) {}
void __stdcall NtCommitRegistryTransaction(int p0, int p1) {}
void __stdcall NtCommitTransaction(int p0, int p1) {}
void __stdcall NtCreateDirectoryObject(int p0, int p1, int p2) {}
void __stdcall NtCreateEnlistment(int p0, int p1, int p2, int p3, int p4, int p5, int p6, int p7) {}
void __stdcall NtCreateEvent(int p0, int p1, int p2, int p3, int p4) {}
void __stdcall NtCreateFile(int p0, int p1, int p2, int p3, int p4, int p5, int p6, int p7, int p8, int p9, int p10) {}
void __stdcall NtCreateKey(int p0, int p1, int p2, int p3, int p4, int p5, int p6) {}
void __stdcall NtCreateKeyTransacted(int p0, int p1, int p2, int p3, int p4, int p5, int p6, int p7) {}
void __stdcall NtCreateRegistryTransaction(int p0, int p1, int p2, int p3) {}
void __stdcall NtCreateResourceManager(int p0, int p1, int p2, int p3, int p4, int p5, int p6) {}
void __stdcall NtCreateSection(int p0, int p1, int p2, int p3, int p4, int p5, int p6) {}
void __stdcall NtCreateSectionEx(int p0, int p1, int p2, int p3, int p4, int p5, int p6, int p7, int p8) {}
void __stdcall NtCreateTimer(int p0, int p1, int p2, int p3) {}
void __stdcall NtCreateTransaction(int p0, int p1, int p2, int p3, int p4, int p5, int p6, int p7, int p8, int p9) {}
void __stdcall NtCreateTransactionManager(int p0, int p1, int p2, int p3, int p4, int p5) {}
void __stdcall NtDeleteFile(int p0) {}
void __stdcall NtDeleteKey(int p0) {}
void __stdcall NtDeleteObjectAuditAlarm(int p0, int p1, int p2) {}
void __stdcall NtDeleteValueKey(int p0, int p1) {}
void __stdcall NtDeviceIoControlFile(int p0, int p1, int p2, int p3, int p4, int p5, int p6, int p7, int p8, int p9) {}
void __stdcall NtDisplayString(int p0) {}
void __stdcall NtDuplicateObject(int p0, int p1, int p2, int p3, int p4, int p5, int p6) {}
void __stdcall NtDuplicateToken(int p0, int p1, int p2, int p3, int p4, int p5) {}
void __stdcall NtEnumerateKey(int p0, int p1, int p2, int p3, int p4, int p5) {}
void __stdcall NtEnumerateTransactionObject(int p0, int p1, int p2, int p3, int p4) {}
void __stdcall NtEnumerateValueKey(int p0, int p1, int p2, int p3, int p4, int p5) {}
void __stdcall NtFilterToken(int p0, int p1, int p2, int p3, int p4, int p5) {}
void __stdcall NtFlushBuffersFile(int p0, int p1) {}
void __stdcall NtFlushBuffersFileEx(int p0, int p1, int p2, int p3, int p4) {}
void __stdcall NtFlushKey(int p0) {}
void __stdcall NtFlushVirtualMemory(int p0, int p1, int p2, int p3) {}
void __stdcall NtFreeVirtualMemory(int p0, int p1, int p2, int p3) {}
void __stdcall NtFsControlFile(int p0, int p1, int p2, int p3, int p4, int p5, int p6, int p7, int p8, int p9) {}
void __stdcall NtGetNotificationResourceManager(int p0, int p1, int p2, int p3, int p4, int p5, int p6) {}
void __stdcall NtImpersonateAnonymousToken(int p0) {}
void __stdcall NtLoadDriver(int p0) {}
void __stdcall NtLockFile(int p0, int p1, int p2, int p3, int p4, int p5, int p6, int p7, int p8, int p9) {}
void __stdcall NtMakeTemporaryObject(int p0) {}
void __stdcall NtManagePartition(int p0, int p1, int p2, int p3, int p4) {}
void __stdcall NtMapViewOfSection(int p0, int p1, int p2, int p3, int p4, int p5, int p6, int p7, int p8, int p9) {}
void __stdcall NtNotifyChangeKey(int p0, int p1, int p2, int p3, int p4, int p5, int p6, int p7, int p8, int p9) {}
void __stdcall NtNotifyChangeMultipleKeys(int p0, int p1, int p2, int p3, int p4, int p5, int p6, int p7, int p8, int p9, int p10, int p11) {}
void __stdcall NtOpenDirectoryObject(int p0, int p1, int p2) {}
void __stdcall NtOpenEnlistment(int p0, int p1, int p2, int p3, int p4) {}
void __stdcall NtOpenEvent(int p0, int p1, int p2) {}
void __stdcall NtOpenFile(int p0, int p1, int p2, int p3, int p4, int p5) {}
void __stdcall NtOpenKey(int p0, int p1, int p2) {}
void __stdcall NtOpenKeyEx(int p0, int p1, int p2, int p3) {}
void __stdcall NtOpenKeyTransacted(int p0, int p1, int p2, int p3) {}
void __stdcall NtOpenKeyTransactedEx(int p0, int p1, int p2, int p3, int p4) {}
void __stdcall NtOpenObjectAuditAlarm(int p0, int p1, int p2, int p3, int p4, int p5, int p6, int p7, int p8, int p9, int p10, int p11) {}
void __stdcall NtOpenProcess(int p0, int p1, int p2, int p3) {}
void __stdcall NtOpenProcessToken(int p0, int p1, int p2) {}
void __stdcall NtOpenProcessTokenEx(int p0, int p1, int p2, int p3) {}
void __stdcall NtOpenRegistryTransaction(int p0, int p1, int p2) {}
void __stdcall NtOpenResourceManager(int p0, int p1, int p2, int p3, int p4) {}
void __stdcall NtOpenSection(int p0, int p1, int p2) {}
void __stdcall NtOpenSymbolicLinkObject(int p0, int p1, int p2) {}
void __stdcall NtOpenThreadToken(int p0, int p1, int p2, int p3) {}
void __stdcall NtOpenThreadTokenEx(int p0, int p1, int p2, int p3, int p4) {}
void __stdcall NtOpenTimer(int p0, int p1, int p2) {}
void __stdcall NtOpenTransaction(int p0, int p1, int p2, int p3, int p4) {}
void __stdcall NtOpenTransactionManager(int p0, int p1, int p2, int p3, int p4, int p5) {}
void __stdcall NtPowerInformation(int p0, int p1, int p2, int p3, int p4) {}
void __stdcall NtPrePrepareComplete(int p0, int p1) {}
void __stdcall NtPrePrepareEnlistment(int p0, int p1) {}
void __stdcall NtPrepareComplete(int p0, int p1) {}
void __stdcall NtPrepareEnlistment(int p0, int p1) {}
void __stdcall NtPrivilegeCheck(int p0, int p1, int p2) {}
void __stdcall NtPrivilegeObjectAuditAlarm(int p0, int p1, int p2, int p3, int p4, int p5) {}
void __stdcall NtPrivilegedServiceAuditAlarm(int p0, int p1, int p2, int p3, int p4) {}
void __stdcall NtPropagationComplete(int p0, int p1, int p2, int p3) {}
void __stdcall NtPropagationFailed(int p0, int p1, int p2) {}
void __stdcall NtQueryDirectoryFile(int p0, int p1, int p2, int p3, int p4, int p5, int p6, int p7, int p8, int p9, int p10) {}
void __stdcall NtQueryDirectoryFileEx(int p0, int p1, int p2, int p3, int p4, int p5, int p6, int p7, int p8, int p9) {}
void __stdcall NtQueryDirectoryObject(int p0, int p1, int p2, int p3, int p4, int p5, int p6) {}
void __stdcall NtQueryEaFile(int p0, int p1, int p2, int p3, int p4, int p5, int p6, int p7, int p8) {}
void __stdcall NtQueryFullAttributesFile(int p0, int p1) {}
void __stdcall NtQueryInformationByName(int p0, int p1, int p2, int p3, int p4) {}
void __stdcall NtQueryInformationEnlistment(int p0, int p1, int p2, int p3, int p4) {}
void __stdcall NtQueryInformationFile(int p0, int p1, int p2, int p3, int p4) {}
void __stdcall NtQueryInformationProcess(int p0, int p1, int p2, int p3, int p4) {}
void __stdcall NtQueryInformationResourceManager(int p0, int p1, int p2, int p3, int p4) {}
void __stdcall NtQueryInformationThread(int p0, int p1, int p2, int p3, int p4) {}
void __stdcall NtQueryInformationToken(int p0, int p1, int p2, int p3, int p4) {}
void __stdcall NtQueryInformationTransaction(int p0, int p1, int p2, int p3, int p4) {}
void __stdcall NtQueryInformationTransactionManager(int p0, int p1, int p2, int p3, int p4) {}
void __stdcall NtQueryKey(int p0, int p1, int p2, int p3, int p4) {}
void __stdcall NtQueryMultipleValueKey(int p0, int p1, int p2, int p3, int p4, int p5) {}
void __stdcall NtQueryObject(int p0, int p1, int p2, int p3, int p4) {}
void __stdcall NtQueryQuotaInformationFile(int p0, int p1, int p2, int p3, int p4, int p5, int p6, int p7, int p8) {}
void __stdcall NtQuerySecurityObject(int p0, int p1, int p2, int p3, int p4) {}
void __stdcall NtQuerySymbolicLinkObject(int p0, int p1, int p2) {}
void __stdcall NtQuerySystemInformation(int p0, int p1, int p2, int p3) {}
void __stdcall NtQuerySystemTime(int p0) {}
void __stdcall NtQueryTimerResolution(int p0, int p1, int p2) {}
void __stdcall NtQueryValueKey(int p0, int p1, int p2, int p3, int p4, int p5) {}
void __stdcall NtQueryVirtualMemory(int p0, int p1, int p2, int p3, int p4, int p5) {}
void __stdcall NtQueryVolumeInformationFile(int p0, int p1, int p2, int p3, int p4) {}
void __stdcall NtReadFile(int p0, int p1, int p2, int p3, int p4, int p5, int p6, int p7, int p8) {}
void __stdcall NtReadOnlyEnlistment(int p0, int p1) {}
void __stdcall NtRecoverEnlistment(int p0, int p1) {}
void __stdcall NtRecoverResourceManager(int p0) {}
void __stdcall NtRecoverTransactionManager(int p0) {}
void __stdcall NtRegisterProtocolAddressInformation(int p0, int p1, int p2, int p3, int p4) {}
void __stdcall NtRenameKey(int p0, int p1) {}
void __stdcall NtRenameTransactionManager(int p0, int p1) {}
void __stdcall NtRestoreKey(int p0, int p1, int p2) {}
void __stdcall NtRollbackComplete(int p0, int p1) {}
void __stdcall NtRollbackEnlistment(int p0, int p1) {}
void __stdcall NtRollbackRegistryTransaction(int p0, int p1) {}
void __stdcall NtRollbackTransaction(int p0, int p1) {}
void __stdcall NtRollforwardTransactionManager(int p0, int p1) {}
void __stdcall NtSaveKey(int p0, int p1) {}
void __stdcall NtSaveKeyEx(int p0, int p1, int p2) {}
void __stdcall NtSetEaFile(int p0, int p1, int p2, int p3) {}
void __stdcall NtSetEvent(int p0, int p1) {}
void __stdcall NtSetInformationEnlistment(int p0, int p1, int p2, int p3) {}
void __stdcall NtSetInformationFile(int p0, int p1, int p2, int p3, int p4) {}
void __stdcall NtSetInformationKey(int p0, int p1, int p2, int p3) {}
void __stdcall NtSetInformationResourceManager(int p0, int p1, int p2, int p3) {}
void __stdcall NtSetInformationThread(int p0, int p1, int p2, int p3) {}
void __stdcall NtSetInformationToken(int p0, int p1, int p2, int p3) {}
void __stdcall NtSetInformationTransaction(int p0, int p1, int p2, int p3) {}
void __stdcall NtSetInformationTransactionManager(int p0, int p1, int p2, int p3) {}
void __stdcall NtSetInformationVirtualMemory(int p0, int p1, int p2, int p3, int p4, int p5) {}
void __stdcall NtSetQuotaInformationFile(int p0, int p1, int p2, int p3) {}
void __stdcall NtSetSecurityObject(int p0, int p1, int p2) {}
void __stdcall NtSetTimer(int p0, int p1, int p2, int p3, int p4, int p5, int p6) {}
void __stdcall NtSetTimerEx(int p0, int p1, int p2, int p3) {}
void __stdcall NtSetValueKey(int p0, int p1, int p2, int p3, int p4, int p5) {}
void __stdcall NtSetVolumeInformationFile(int p0, int p1, int p2, int p3, int p4) {}
void __stdcall NtSinglePhaseReject(int p0, int p1) {}
void __stdcall NtTerminateProcess(int p0, int p1) {}
void __stdcall NtUnloadDriver(int p0) {}
void __stdcall NtUnlockFile(int p0, int p1, int p2, int p3, int p4) {}
void __stdcall NtUnmapViewOfSection(int p0, int p1) {}
void __stdcall NtWaitForSingleObject(int p0, int p1, int p2) {}
void __stdcall NtWriteFile(int p0, int p1, int p2, int p3, int p4, int p5, int p6, int p7, int p8) {}
void __stdcall PfxFindPrefix(int p0, int p1) {}
void __stdcall PfxInitialize(int p0) {}
void __stdcall PfxInsertPrefix(int p0, int p1, int p2) {}
void __stdcall PfxRemovePrefix(int p0, int p1) {}
void __stdcall RtlAbsoluteToSelfRelativeSD(int p0, int p1, int p2) {}
void __stdcall RtlAddAccessAllowedAce(int p0, int p1, int p2, int p3) {}
void __stdcall RtlAddAccessAllowedAceEx(int p0, int p1, int p2, int p3, int p4) {}
void __stdcall RtlAddAce(int p0, int p1, int p2, int p3, int p4) {}
void __stdcall RtlAddGrowableFunctionTable() {}
void __stdcall RtlAllocateAndInitializeSid(int p0, int p1, int p2, int p3, int p4, int p5, int p6, int p7, int p8, int p9, int p10) {}
void __stdcall RtlAllocateAndInitializeSidEx(int p0, int p1, int p2, int p3) {}
void __stdcall RtlAllocateHeap(int p0, int p1, int p2) {}
void __stdcall RtlAnsiStringToUnicodeString(int p0, int p1, int p2) {}
void __stdcall RtlAppendStringToString(int p0, int p1) {}
void __stdcall RtlAppendUnicodeStringToString(int p0, int p1) {}
void __stdcall RtlAppendUnicodeToString(int p0, int p1) {}
void __stdcall RtlAreBitsClear(int p0, int p1, int p2) {}
void __stdcall RtlAreBitsSet(int p0, int p1, int p2) {}
void __stdcall RtlAssert(int p0, int p1, int p2, int p3) {}
void __stdcall RtlCharToInteger(int p0, int p1, int p2) {}
void __stdcall RtlCheckRegistryKey(int p0, int p1) {}
void __stdcall RtlClearAllBits(int p0) {}
void __stdcall RtlClearBit(int p0, int p1) {}
void __stdcall RtlClearBits(int p0, int p1, int p2) {}
void __stdcall RtlCmDecodeMemIoResource(int p0, int p1) {}
void __stdcall RtlCmEncodeMemIoResource(int p0, int p1, int p2, int p3, int p4, int p5) {}
void __stdcall RtlCompareAltitudes(int p0, int p1) {}
void __stdcall RtlCompareMemoryUlong(int p0, int p1, int p2) {}
void __stdcall RtlCompareString(int p0, int p1, int p2) {}
void __stdcall RtlCompareUnicodeString(int p0, int p1, int p2) {}
void __stdcall RtlCompareUnicodeStrings(int p0, int p1, int p2, int p3, int p4) {}
void __stdcall RtlCompressBuffer(int p0, int p1, int p2, int p3, int p4, int p5, int p6, int p7) {}
void __stdcall RtlContractHashTable(int p0) {}
void __stdcall RtlConvertDeviceFamilyInfoToString(int p0, int p1, int p2, int p3) {}
void __stdcall RtlConvertSidToUnicodeString(int p0, int p1, int p2) {}
void __stdcall RtlCopyBitMap(int p0, int p1, int p2) {}
void __stdcall RtlCopyLuid(int p0, int p1) {}
void __stdcall RtlCopySid(int p0, int p1, int p2) {}
void __stdcall RtlCopyString(int p0, int p1) {}
void __stdcall RtlCopyUnicodeString(int p0, int p1) {}
void __stdcall RtlCrc32(int p0, int p1, int p2) {}
void __stdcall RtlCrc64(int p0, int p1, int p2, int p3) {}
void __stdcall RtlCreateAcl(int p0, int p1, int p2) {}
void __stdcall RtlCreateHashTable(int p0, int p1, int p2) {}
void __stdcall RtlCreateHashTableEx(int p0, int p1, int p2, int p3) {}
void __stdcall RtlCreateHeap(int p0, int p1, int p2, int p3, int p4, int p5) {}
void __stdcall RtlCreateRegistryKey(int p0, int p1) {}
void __stdcall RtlCreateSecurityDescriptor(int p0, int p1) {}
void __stdcall RtlCreateServiceSid(int p0, int p1, int p2) {}
void __stdcall RtlCreateSystemVolumeInformationFolder(int p0) {}
void __stdcall RtlCreateUnicodeString(int p0, int p1) {}
void __stdcall RtlCreateVirtualAccountSid(int p0, int p1, int p2, int p3) {}
void __stdcall RtlCustomCPToUnicodeN(int p0, int p1, int p2, int p3, int p4, int p5) {}
void __stdcall RtlDecompressBuffer(int p0, int p1, int p2, int p3, int p4, int p5) {}
void __stdcall RtlDecompressBufferEx(int p0, int p1, int p2, int p3, int p4, int p5, int p6) {}
void __stdcall RtlDecompressFragment(int p0, int p1, int p2, int p3, int p4, int p5, int p6, int p7) {}
void __stdcall RtlDelete(int p0) {}
void __stdcall RtlDeleteAce(int p0, int p1) {}
void __stdcall RtlDeleteElementGenericTable(int p0, int p1) {}
void __stdcall RtlDeleteElementGenericTableAvl(int p0, int p1) {}
void __stdcall RtlDeleteElementGenericTableAvlEx(int p0, int p1) {}
void __stdcall RtlDeleteGrowableFunctionTable() {}
void __stdcall RtlDeleteHashTable(int p0) {}
void __stdcall RtlDeleteNoSplay(int p0, int p1) {}
void __stdcall RtlDeleteRegistryValue(int p0, int p1, int p2) {}
void __stdcall RtlDestroyHeap(int p0) {}
void __stdcall RtlDosPathNameToNtPathName_U_WithStatus(int p0, int p1, int p2, int p3) {}
void __stdcall RtlDowncaseUnicodeChar(int p0) {}
void __stdcall RtlDowncaseUnicodeString(int p0, int p1, int p2) {}
void __stdcall RtlDrainNonVolatileFlush() {}
void __stdcall RtlDuplicateUnicodeString(int p0, int p1, int p2) {}
void __stdcall RtlEndEnumerationHashTable(int p0, int p1) {}
void __stdcall RtlEndStrongEnumerationHashTable(int p0, int p1) {}
void __stdcall RtlEndWeakEnumerationHashTable(int p0, int p1) {}
void __stdcall RtlEnumerateEntryHashTable(int p0, int p1) {}
void __stdcall RtlEnumerateGenericTable(int p0, int p1) {}
void __stdcall RtlEnumerateGenericTableAvl(int p0, int p1) {}
void __stdcall RtlEnumerateGenericTableLikeADirectory(int p0, int p1, int p2, int p3, int p4, int p5, int p6) {}
void __stdcall RtlEnumerateGenericTableWithoutSplaying(int p0, int p1) {}
void __stdcall RtlEnumerateGenericTableWithoutSplayingAvl(int p0, int p1) {}
void __stdcall RtlEqualPrefixSid(int p0, int p1) {}
void __stdcall RtlEqualSid(int p0, int p1) {}
void __stdcall RtlEqualString(int p0, int p1, int p2) {}
void __stdcall RtlEqualUnicodeString(int p0, int p1, int p2) {}
void __stdcall RtlEthernetAddressToStringA(int p0, int p1) {}
void __stdcall RtlEthernetAddressToStringW(int p0, int p1) {}
void __stdcall RtlEthernetStringToAddressA(int p0, int p1, int p2) {}
void __stdcall RtlEthernetStringToAddressW(int p0, int p1, int p2) {}
void __stdcall RtlExpandHashTable(int p0) {}
void __stdcall RtlExtendCorrelationVector(int p0) {}
void __stdcall RtlExtractBitMap(int p0, int p1, int p2, int p3) {}
void __stdcall RtlFillNonVolatileMemory() {}
void __stdcall RtlFindClearBits(int p0, int p1, int p2) {}
void __stdcall RtlFindClearBitsAndSet(int p0, int p1, int p2) {}
void __stdcall RtlFindClearRuns(int p0, int p1, int p2, int p3) {}
void __stdcall RtlFindClosestEncodableLength(int p0, int p1, int p2) {}
void __stdcall RtlFindLastBackwardRunClear(int p0, int p1, int p2) {}
void __stdcall RtlFindLeastSignificantBit(int p0, int p1) {}
void __stdcall RtlFindLongestRunClear(int p0, int p1) {}
void __stdcall RtlFindMostSignificantBit(int p0, int p1) {}
void __stdcall RtlFindNextForwardRunClear(int p0, int p1, int p2) {}
void __stdcall RtlFindSetBits(int p0, int p1, int p2) {}
void __stdcall RtlFindSetBitsAndClear(int p0, int p1, int p2) {}
void __stdcall RtlFirstEntrySList(int p0) {}
void __stdcall RtlFlushNonVolatileMemory() {}
void __stdcall RtlFlushNonVolatileMemoryRanges() {}
void __stdcall RtlFreeAnsiString(int p0) {}
void __stdcall RtlFreeHeap(int p0, int p1, int p2) {}
void __stdcall RtlFreeNonVolatileToken() {}
void __stdcall RtlFreeOemString(int p0) {}
void __stdcall RtlFreeSid(int p0) {}
void __stdcall RtlFreeUTF8String(int p0) {}
void __stdcall RtlFreeUnicodeString(int p0) {}
void __stdcall RtlGUIDFromString(int p0, int p1) {}
void __stdcall RtlGenerate8dot3Name(int p0, int p1, int p2, int p3) {}
void __stdcall RtlGetAce(int p0, int p1, int p2) {}
void __stdcall RtlGetActiveConsoleId() {}
void __stdcall RtlGetCallersAddress(int p0, int p1) {}
void __stdcall RtlGetCompressionWorkSpaceSize(int p0, int p1, int p2) {}
void __stdcall RtlGetConsoleSessionForegroundProcessId() {}
void __stdcall RtlGetDaclSecurityDescriptor(int p0, int p1, int p2, int p3) {}
void __stdcall RtlGetDeviceFamilyInfoEnum(int p0, int p1, int p2) {}
void __stdcall RtlGetElementGenericTable(int p0, int p1) {}
void __stdcall RtlGetElementGenericTableAvl(int p0, int p1) {}
void __stdcall RtlGetEnabledExtendedFeatures(int p0, int p1) {}
void __stdcall RtlGetGroupSecurityDescriptor(int p0, int p1, int p2) {}
void __stdcall RtlGetNextEntryHashTable(int p0, int p1) {}
void __stdcall RtlGetNonVolatileToken() {}
void __stdcall RtlGetNtProductType(int p0) {}
void __stdcall RtlGetNtSystemRoot() {}
void __stdcall RtlGetOwnerSecurityDescriptor(int p0, int p1, int p2) {}
void __stdcall RtlGetPersistedStateLocation(int p0, int p1, int p2, int p3, int p4, int p5, int p6) {}
void __stdcall RtlGetProductInfo(int p0, int p1, int p2, int p3, int p4) {}
void __stdcall RtlGetReturnAddressHijackTarget() {}
void __stdcall RtlGetSaclSecurityDescriptor(int p0, int p1, int p2, int p3) {}
void __stdcall RtlGetSuiteMask() {}
void __stdcall RtlGetVersion(int p0) {}
void __stdcall RtlGrowFunctionTable() {}
void __stdcall RtlHashUnicodeString(int p0, int p1, int p2, int p3) {}
void __stdcall RtlIdentifierAuthoritySid(int p0) {}
void __stdcall RtlIdnToAscii(int p0, int p1, int p2, int p3, int p4) {}
void __stdcall RtlIdnToNameprepUnicode(int p0, int p1, int p2, int p3, int p4) {}
void __stdcall RtlIdnToUnicode(int p0, int p1, int p2, int p3, int p4) {}
void __stdcall RtlIncrementCorrelationVector(int p0) {}
void __stdcall RtlInitAnsiString(int p0, int p1) {}
void __stdcall RtlInitAnsiStringEx(int p0, int p1) {}
void __stdcall RtlInitCodePageTable(int p0, int p1) {}
void __stdcall RtlInitEnumerationHashTable(int p0, int p1) {}
void __stdcall RtlInitString(int p0, int p1) {}
void __stdcall RtlInitStringEx(int p0, int p1) {}
void __stdcall RtlInitStrongEnumerationHashTable(int p0, int p1) {}
void __stdcall RtlInitUTF8String(int p0, int p1) {}
void __stdcall RtlInitUTF8StringEx(int p0, int p1) {}
void __stdcall RtlInitUnicodeString(int p0, int p1) {}
void __stdcall RtlInitUnicodeStringEx(int p0, int p1) {}
void __stdcall RtlInitWeakEnumerationHashTable(int p0, int p1) {}
void __stdcall RtlInitializeBitMap(int p0, int p1, int p2) {}
void __stdcall RtlInitializeCorrelationVector(int p0, int p1, int p2) {}
void __stdcall RtlInitializeGenericTable(int p0, int p1, int p2, int p3, int p4) {}
void __stdcall RtlInitializeGenericTableAvl(int p0, int p1, int p2, int p3, int p4) {}
void __stdcall RtlInitializeSListHead(int p0) {}
void __stdcall RtlInitializeSid(int p0, int p1, int p2) {}
void __cdecl RtlInitializeSidEx() {}
void __stdcall RtlInsertElementGenericTable(int p0, int p1, int p2, int p3) {}
void __stdcall RtlInsertElementGenericTableAvl(int p0, int p1, int p2, int p3) {}
void __stdcall RtlInsertElementGenericTableFull(int p0, int p1, int p2, int p3, int p4, int p5) {}
void __stdcall RtlInsertElementGenericTableFullAvl(int p0, int p1, int p2, int p3, int p4, int p5) {}
void __stdcall RtlInsertEntryHashTable(int p0, int p1, int p2, int p3) {}
void __stdcall RtlInt64ToUnicodeString(int p0, int p1, int p2, int p3) {}
void __stdcall RtlIntegerToUnicodeString(int p0, int p1, int p2) {}
void __stdcall RtlInterlockedFlushSList(int p0) {}
void __stdcall RtlInterlockedPopEntrySList(int p0) {}
void __stdcall RtlInterlockedPushEntrySList(int p0, int p1) {}
void __stdcall RtlInterlockedPushListSListEx(int p0, int p1, int p2, int p3) {}
void __stdcall RtlIoDecodeMemIoResource(int p0, int p1, int p2, int p3) {}
void __stdcall RtlIoEncodeMemIoResource(int p0, int p1, int p2, int p3, int p4, int p5, int p6, int p7, int p8, int p9) {}
void __stdcall RtlIpv4AddressToStringA(int p0, int p1) {}
void __stdcall RtlIpv4AddressToStringExA(int p0, int p1, int p2, int p3) {}
void __stdcall RtlIpv4AddressToStringExW(int p0, int p1, int p2, int p3) {}
void __stdcall RtlIpv4AddressToStringW(int p0, int p1) {}
void __stdcall RtlIpv4StringToAddressA(int p0, int p1, int p2, int p3) {}
void __stdcall RtlIpv4StringToAddressExA(int p0, int p1, int p2, int p3) {}
void __stdcall RtlIpv4StringToAddressExW(int p0, int p1, int p2, int p3) {}
void __stdcall RtlIpv4StringToAddressW(int p0, int p1, int p2, int p3) {}
void __stdcall RtlIpv6AddressToStringA(int p0, int p1) {}
void __stdcall RtlIpv6AddressToStringExA(int p0, int p1, int p2, int p3, int p4) {}
void __stdcall RtlIpv6AddressToStringExW(int p0, int p1, int p2, int p3, int p4) {}
void __stdcall RtlIpv6AddressToStringW(int p0, int p1) {}
void __stdcall RtlIpv6StringToAddressA(int p0, int p1, int p2) {}
void __stdcall RtlIpv6StringToAddressExA(int p0, int p1, int p2, int p3) {}
void __stdcall RtlIpv6StringToAddressExW(int p0, int p1, int p2, int p3) {}
void __stdcall RtlIpv6StringToAddressW(int p0, int p1, int p2) {}
void __stdcall RtlIsApiSetImplemented(int p0) {}
void __stdcall RtlIsCloudFilesPlaceholder(int p0, int p1) {}
void __stdcall RtlIsDosDeviceName_U(int p0) {}
void __stdcall RtlIsGenericTableEmpty(int p0) {}
void __stdcall RtlIsGenericTableEmptyAvl(int p0) {}
void __stdcall RtlIsMultiSessionSku() {}
void __stdcall RtlIsMultiUsersInSessionSku() {}
void __stdcall RtlIsNameLegalDOS8Dot3(int p0, int p1, int p2) {}
void __stdcall RtlIsNonEmptyDirectoryReparsePointAllowed(int p0) {}
void __stdcall RtlIsNormalizedString(int p0, int p1, int p2, int p3) {}
void __stdcall RtlIsPartialPlaceholder(int p0, int p1) {}
void __stdcall RtlIsPartialPlaceholderFileHandle(int p0, int p1) {}
void __stdcall RtlIsPartialPlaceholderFileInfo(int p0, int p1, int p2) {}
void __stdcall RtlIsStateSeparationEnabled() {}
void __stdcall RtlIsUntrustedObject(int p0, int p1, int p2) {}
void __stdcall RtlIsZeroMemory(int p0, int p1) {}
void __stdcall RtlLengthRequiredSid(int p0) {}
void __stdcall RtlLengthSecurityDescriptor(int p0) {}
void __stdcall RtlLengthSid(int p0) {}
void __stdcall RtlLocalTimeToSystemTime(int p0, int p1) {}
void __stdcall RtlLookupElementGenericTable(int p0, int p1) {}
void __stdcall RtlLookupElementGenericTableAvl(int p0, int p1) {}
void __stdcall RtlLookupElementGenericTableFull(int p0, int p1, int p2, int p3) {}
void __stdcall RtlLookupElementGenericTableFullAvl(int p0, int p1, int p2, int p3) {}
void __stdcall RtlLookupEntryHashTable(int p0, int p1, int p2) {}
void __stdcall RtlLookupFirstMatchingElementGenericTableAvl(int p0, int p1, int p2) {}
void __stdcall RtlMapGenericMask(int p0, int p1) {}
void __stdcall RtlMultiByteToUnicodeN(int p0, int p1, int p2, int p3, int p4) {}
void __stdcall RtlMultiByteToUnicodeSize(int p0, int p1, int p2) {}
void __stdcall RtlNormalizeSecurityDescriptor(int p0, int p1, int p2, int p3, int p4) {}
void __stdcall RtlNormalizeString(int p0, int p1, int p2, int p3, int p4) {}
void __stdcall RtlNtStatusToDosError(int p0) {}
void __stdcall RtlNtStatusToDosErrorNoTeb(int p0) {}
void __stdcall RtlNumberGenericTableElements(int p0) {}
void __stdcall RtlNumberGenericTableElementsAvl(int p0) {}
void __stdcall RtlNumberOfClearBits(int p0) {}
void __stdcall RtlNumberOfClearBitsInRange(int p0, int p1, int p2) {}
void __stdcall RtlNumberOfSetBits(int p0) {}
void __stdcall RtlNumberOfSetBitsInRange(int p0, int p1, int p2) {}
void __stdcall RtlNumberOfSetBitsUlongPtr(int p0) {}
void __stdcall RtlOemStringToUnicodeString(int p0, int p1, int p2) {}
void __stdcall RtlOemToUnicodeN(int p0, int p1, int p2, int p3, int p4) {}
void __stdcall RtlOsDeploymentState(int p0) {}
void __stdcall RtlPrefixString(int p0, int p1, int p2) {}
void __stdcall RtlPrefixUnicodeString(int p0, int p1, int p2) {}
void __stdcall RtlQueryDepthSList(int p0) {}
void __stdcall RtlQueryPackageIdentity(int p0, int p1, int p2, int p3, int p4, int p5) {}
void __stdcall RtlQueryPackageIdentityEx(int p0, int p1, int p2, int p3, int p4, int p5, int p6) {}
void __stdcall RtlQueryProcessPlaceholderCompatibilityMode() {}
void __stdcall RtlQueryRegistryValueWithFallback(int p0, int p1, int p2, int p3, int p4, int p5, int p6) {}
void __stdcall RtlQueryRegistryValues(int p0, int p1, int p2, int p3, int p4) {}
void __stdcall RtlQueryThreadPlaceholderCompatibilityMode() {}
void __stdcall RtlQueryValidationRunlevel(int p0) {}
void __stdcall RtlRaiseCustomSystemEventTrigger(int p0) {}
void __stdcall RtlRandom(int p0) {}
void __stdcall RtlRandomEx(int p0) {}
void __stdcall RtlRealPredecessor(int p0) {}
void __stdcall RtlRealSuccessor(int p0) {}
void __stdcall RtlRemoveEntryHashTable(int p0, int p1, int p2) {}
void __stdcall RtlReplaceSidInSd(int p0, int p1, int p2, int p3) {}
void __stdcall RtlRunOnceBeginInitialize(int p0, int p1, int p2) {}
void __stdcall RtlRunOnceComplete(int p0, int p1, int p2) {}
void __stdcall RtlRunOnceExecuteOnce(int p0, int p1, int p2, int p3) {}
void __stdcall RtlRunOnceInitialize(int p0) {}
void __stdcall RtlSecondsSince1970ToTime(int p0, int p1) {}
void __stdcall RtlSecondsSince1980ToTime(int p0, int p1) {}
void __stdcall RtlSelfRelativeToAbsoluteSD(int p0, int p1, int p2, int p3, int p4, int p5, int p6, int p7, int p8, int p9, int p10) {}
void __stdcall RtlSetAllBits(int p0) {}
void __stdcall RtlSetBit(int p0, int p1) {}
void __stdcall RtlSetBits(int p0, int p1, int p2) {}
void __stdcall RtlSetDaclSecurityDescriptor(int p0, int p1, int p2, int p3) {}
void __stdcall RtlSetGroupSecurityDescriptor(int p0, int p1, int p2) {}
void __stdcall RtlSetOwnerSecurityDescriptor(int p0, int p1, int p2) {}
void __stdcall RtlSetProcessPlaceholderCompatibilityMode(int p0) {}
void __stdcall RtlSetThreadPlaceholderCompatibilityMode(int p0) {}
void __stdcall RtlSplay(int p0) {}
void __stdcall RtlStringFromGUID(int p0, int p1) {}
void __stdcall RtlStronglyEnumerateEntryHashTable(int p0, int p1) {}
void __stdcall RtlSubAuthorityCountSid(int p0) {}
void __stdcall RtlSubAuthoritySid(int p0, int p1) {}
void __stdcall RtlSubtreePredecessor(int p0) {}
void __stdcall RtlSubtreeSuccessor(int p0) {}
void __stdcall RtlSwitchedVVI(int p0, int p1, int p2, int p3) {}
void __stdcall RtlTestBit(int p0, int p1) {}
void __stdcall RtlTimeFieldsToTime(int p0, int p1) {}
void __stdcall RtlTimeToSecondsSince1970(int p0, int p1) {}
void __stdcall RtlTimeToSecondsSince1980(int p0, int p1) {}
void __stdcall RtlTimeToTimeFields(int p0, int p1) {}
void __stdcall RtlUTF8StringToUnicodeString(int p0, int p1, int p2) {}
void __stdcall RtlUTF8ToUnicodeN(int p0, int p1, int p2, int p3, int p4) {}
void __stdcall RtlUnicodeStringToAnsiString(int p0, int p1, int p2) {}
void __stdcall RtlUnicodeStringToCountedOemString(int p0, int p1, int p2) {}
void __stdcall RtlUnicodeStringToInteger(int p0, int p1, int p2) {}
void __stdcall RtlUnicodeStringToOemString(int p0, int p1, int p2) {}
void __stdcall RtlUnicodeStringToUTF8String(int p0, int p1, int p2) {}
void __stdcall RtlUnicodeToCustomCPN(int p0, int p1, int p2, int p3, int p4, int p5) {}
void __stdcall RtlUnicodeToMultiByteN(int p0, int p1, int p2, int p3, int p4) {}
void __stdcall RtlUnicodeToMultiByteSize(int p0, int p1, int p2) {}
void __stdcall RtlUnicodeToOemN(int p0, int p1, int p2, int p3, int p4) {}
void __stdcall RtlUnicodeToUTF8N(int p0, int p1, int p2, int p3, int p4) {}
void __stdcall RtlUniform(int p0) {}
void __stdcall RtlUpcaseUnicodeChar(int p0) {}
void __stdcall RtlUpcaseUnicodeString(int p0, int p1, int p2) {}
void __stdcall RtlUpcaseUnicodeStringToCountedOemString(int p0, int p1, int p2) {}
void __stdcall RtlUpcaseUnicodeStringToOemString(int p0, int p1, int p2) {}
void __stdcall RtlUpcaseUnicodeToCustomCPN(int p0, int p1, int p2, int p3, int p4, int p5) {}
void __stdcall RtlUpcaseUnicodeToMultiByteN(int p0, int p1, int p2, int p3, int p4) {}
void __stdcall RtlUpcaseUnicodeToOemN(int p0, int p1, int p2, int p3, int p4) {}
void __stdcall RtlUpperChar(int p0) {}
void __stdcall RtlUpperString(int p0, int p1) {}
void __stdcall RtlValidRelativeSecurityDescriptor(int p0, int p1, int p2) {}
void __stdcall RtlValidSecurityDescriptor(int p0) {}
void __stdcall RtlValidSid(int p0) {}
void __stdcall RtlValidateCorrelationVector(int p0) {}
void __stdcall RtlValidateUnicodeString(int p0, int p1) {}
void __stdcall RtlVerifyVersionInfo(int p0, int p1, int p2, int p3) {}
void __stdcall RtlWalkFrameChain(int p0, int p1, int p2) {}
void __stdcall RtlWeaklyEnumerateEntryHashTable(int p0, int p1) {}
void __stdcall RtlWriteNonVolatileMemory() {}
void __stdcall RtlWriteRegistryValue(int p0, int p1, int p2, int p3, int p4, int p5) {}
void __stdcall RtlxAnsiStringToUnicodeSize(int p0) {}
void __stdcall RtlxOemStringToUnicodeSize(int p0) {}
void __stdcall RtlxUnicodeStringToAnsiSize(int p0) {}
void __stdcall RtlxUnicodeStringToOemSize(int p0) {}
void __stdcall ZwAccessCheckAndAuditAlarm(int p0, int p1, int p2, int p3, int p4, int p5, int p6, int p7, int p8, int p9, int p10) {}
void __stdcall ZwAccessCheckByTypeAndAuditAlarm(int p0, int p1, int p2, int p3, int p4, int p5, int p6, int p7, int p8, int p9, int p10, int p11, int p12, int p13, int p14, int p15) {}
void __stdcall ZwAccessCheckByTypeResultListAndAuditAlarm(int p0, int p1, int p2, int p3, int p4, int p5, int p6, int p7, int p8, int p9, int p10, int p11, int p12, int p13, int p14, int p15) {}
void __stdcall ZwAccessCheckByTypeResultListAndAuditAlarmByHandle(int p0, int p1, int p2, int p3, int p4, int p5, int p6, int p7, int p8, int p9, int p10, int p11, int p12, int p13, int p14, int p15, int p16) {}
void __stdcall ZwAdjustGroupsToken(int p0, int p1, int p2, int p3, int p4, int p5) {}
void __stdcall ZwAdjustPrivilegesToken(int p0, int p1, int p2, int p3, int p4, int p5) {}
void __stdcall ZwAllocateLocallyUniqueId(int p0) {}
void __stdcall ZwAllocateVirtualMemory(int p0, int p1, int p2, int p3, int p4, int p5) {}
void __stdcall ZwAllocateVirtualMemoryEx(int p0, int p1, int p2, int p3, int p4, int p5, int p6) {}
void __stdcall ZwCancelIoFileEx(int p0, int p1, int p2) {}
void __stdcall ZwCancelTimer(int p0, int p1) {}
void __stdcall ZwClose(int p0) {}
void __stdcall ZwCloseObjectAuditAlarm(int p0, int p1, int p2) {}
void __stdcall ZwCommitComplete(int p0, int p1) {}
void __stdcall ZwCommitEnlistment(int p0, int p1) {}
void __stdcall ZwCommitRegistryTransaction(int p0, int p1) {}
void __stdcall ZwCommitTransaction(int p0, int p1) {}
void __stdcall ZwCreateDirectoryObject(int p0, int p1, int p2) {}
void __stdcall ZwCreateEnlistment(int p0, int p1, int p2, int p3, int p4, int p5, int p6, int p7) {}
void __stdcall ZwCreateEvent(int p0, int p1, int p2, int p3, int p4) {}
void __stdcall ZwCreateFile(int p0, int p1, int p2, int p3, int p4, int p5, int p6, int p7, int p8, int p9, int p10) {}
void __stdcall ZwCreateKey(int p0, int p1, int p2, int p3, int p4, int p5, int p6) {}
void __stdcall ZwCreateKeyTransacted(int p0, int p1, int p2, int p3, int p4, int p5, int p6, int p7) {}
void __stdcall ZwCreateRegistryTransaction(int p0, int p1, int p2, int p3) {}
void __stdcall ZwCreateResourceManager(int p0, int p1, int p2, int p3, int p4, int p5, int p6) {}
void __stdcall ZwCreateSection(int p0, int p1, int p2, int p3, int p4, int p5, int p6) {}
void __stdcall ZwCreateSectionEx(int p0, int p1, int p2, int p3, int p4, int p5, int p6, int p7, int p8) {}
void __stdcall ZwCreateTimer(int p0, int p1, int p2, int p3) {}
void __stdcall ZwCreateTransaction(int p0, int p1, int p2, int p3, int p4, int p5, int p6, int p7, int p8, int p9) {}
void __stdcall ZwCreateTransactionManager(int p0, int p1, int p2, int p3, int p4, int p5) {}
void __stdcall ZwDeleteFile(int p0) {}
void __stdcall ZwDeleteKey(int p0) {}
void __stdcall ZwDeleteObjectAuditAlarm(int p0, int p1, int p2) {}
void __stdcall ZwDeleteValueKey(int p0, int p1) {}
void __stdcall ZwDeviceIoControlFile(int p0, int p1, int p2, int p3, int p4, int p5, int p6, int p7, int p8, int p9) {}
void __stdcall ZwDisplayString(int p0) {}
void __stdcall ZwDuplicateObject(int p0, int p1, int p2, int p3, int p4, int p5, int p6) {}
void __stdcall ZwDuplicateToken(int p0, int p1, int p2, int p3, int p4, int p5) {}
void __stdcall ZwEnumerateKey(int p0, int p1, int p2, int p3, int p4, int p5) {}
void __stdcall ZwEnumerateTransactionObject(int p0, int p1, int p2, int p3, int p4) {}
void __stdcall ZwEnumerateValueKey(int p0, int p1, int p2, int p3, int p4, int p5) {}
void __stdcall ZwFilterToken(int p0, int p1, int p2, int p3, int p4, int p5) {}
void __stdcall ZwFlushBuffersFile(int p0, int p1) {}
void __stdcall ZwFlushBuffersFileEx(int p0, int p1, int p2, int p3, int p4) {}
void __stdcall ZwFlushKey(int p0) {}
void __stdcall ZwFlushVirtualMemory(int p0, int p1, int p2, int p3) {}
void __stdcall ZwFreeVirtualMemory(int p0, int p1, int p2, int p3) {}
void __stdcall ZwFsControlFile(int p0, int p1, int p2, int p3, int p4, int p5, int p6, int p7, int p8, int p9) {}
void __stdcall ZwGetNotificationResourceManager(int p0, int p1, int p2, int p3, int p4, int p5, int p6) {}
void __stdcall ZwImpersonateAnonymousToken(int p0) {}
void __stdcall ZwLoadDriver(int p0) {}
void __stdcall ZwLockFile(int p0, int p1, int p2, int p3, int p4, int p5, int p6, int p7, int p8, int p9) {}
void __stdcall ZwMakeTemporaryObject(int p0) {}
void __stdcall ZwManagePartition(int p0, int p1, int p2, int p3, int p4) {}
void __stdcall ZwMapViewOfSection(int p0, int p1, int p2, int p3, int p4, int p5, int p6, int p7, int p8, int p9) {}
void __stdcall ZwNotifyChangeKey(int p0, int p1, int p2, int p3, int p4, int p5, int p6, int p7, int p8, int p9) {}
void __stdcall ZwNotifyChangeMultipleKeys(int p0, int p1, int p2, int p3, int p4, int p5, int p6, int p7, int p8, int p9, int p10, int p11) {}
void __stdcall ZwOpenDirectoryObject(int p0, int p1, int p2) {}
void __stdcall ZwOpenEnlistment(int p0, int p1, int p2, int p3, int p4) {}
void __stdcall ZwOpenEvent(int p0, int p1, int p2) {}
void __stdcall ZwOpenFile(int p0, int p1, int p2, int p3, int p4, int p5) {}
void __stdcall ZwOpenKey(int p0, int p1, int p2) {}
void __stdcall ZwOpenKeyEx(int p0, int p1, int p2, int p3) {}
void __stdcall ZwOpenKeyTransacted(int p0, int p1, int p2, int p3) {}
void __stdcall ZwOpenKeyTransactedEx(int p0, int p1, int p2, int p3, int p4) {}
void __stdcall ZwOpenObjectAuditAlarm(int p0, int p1, int p2, int p3, int p4, int p5, int p6, int p7, int p8, int p9, int p10, int p11) {}
void __stdcall ZwOpenProcess(int p0, int p1, int p2, int p3) {}
void __stdcall ZwOpenProcessToken(int p0, int p1, int p2) {}
void __stdcall ZwOpenProcessTokenEx(int p0, int p1, int p2, int p3) {}
void __stdcall ZwOpenRegistryTransaction(int p0, int p1, int p2) {}
void __stdcall ZwOpenResourceManager(int p0, int p1, int p2, int p3, int p4) {}
void __stdcall ZwOpenSection(int p0, int p1, int p2) {}
void __stdcall ZwOpenSymbolicLinkObject(int p0, int p1, int p2) {}
void __stdcall ZwOpenThreadToken(int p0, int p1, int p2, int p3) {}
void __stdcall ZwOpenThreadTokenEx(int p0, int p1, int p2, int p3, int p4) {}
void __stdcall ZwOpenTimer(int p0, int p1, int p2) {}
void __stdcall ZwOpenTransaction(int p0, int p1, int p2, int p3, int p4) {}
void __stdcall ZwOpenTransactionManager(int p0, int p1, int p2, int p3, int p4, int p5) {}
void __stdcall ZwPowerInformation(int p0, int p1, int p2, int p3, int p4) {}
void __stdcall ZwPrePrepareComplete(int p0, int p1) {}
void __stdcall ZwPrePrepareEnlistment(int p0, int p1) {}
void __stdcall ZwPrepareComplete(int p0, int p1) {}
void __stdcall ZwPrepareEnlistment(int p0, int p1) {}
void __stdcall ZwPrivilegeCheck(int p0, int p1, int p2) {}
void __stdcall ZwPrivilegeObjectAuditAlarm(int p0, int p1, int p2, int p3, int p4, int p5) {}
void __stdcall ZwPrivilegedServiceAuditAlarm(int p0, int p1, int p2, int p3, int p4) {}
void __stdcall ZwPropagationComplete(int p0, int p1, int p2, int p3) {}
void __stdcall ZwPropagationFailed(int p0, int p1, int p2) {}
void __stdcall ZwQueryDirectoryFile(int p0, int p1, int p2, int p3, int p4, int p5, int p6, int p7, int p8, int p9, int p10) {}
void __stdcall ZwQueryDirectoryFileEx(int p0, int p1, int p2, int p3, int p4, int p5, int p6, int p7, int p8, int p9) {}
void __stdcall ZwQueryDirectoryObject(int p0, int p1, int p2, int p3, int p4, int p5, int p6) {}
void __stdcall ZwQueryEaFile(int p0, int p1, int p2, int p3, int p4, int p5, int p6, int p7, int p8) {}
void __stdcall ZwQueryFullAttributesFile(int p0, int p1) {}
void __stdcall ZwQueryInformationByName(int p0, int p1, int p2, int p3, int p4) {}
void __stdcall ZwQueryInformationEnlistment(int p0, int p1, int p2, int p3, int p4) {}
void __stdcall ZwQueryInformationFile(int p0, int p1, int p2, int p3, int p4) {}
void __stdcall ZwQueryInformationProcess(int p0, int p1, int p2, int p3, int p4) {}
void __stdcall ZwQueryInformationResourceManager(int p0, int p1, int p2, int p3, int p4) {}
void __stdcall ZwQueryInformationThread(int p0, int p1, int p2, int p3, int p4) {}
void __stdcall ZwQueryInformationToken(int p0, int p1, int p2, int p3, int p4) {}
void __stdcall ZwQueryInformationTransaction(int p0, int p1, int p2, int p3, int p4) {}
void __stdcall ZwQueryInformationTransactionManager(int p0, int p1, int p2, int p3, int p4) {}
void __stdcall ZwQueryKey(int p0, int p1, int p2, int p3, int p4) {}
void __stdcall ZwQueryMultipleValueKey(int p0, int p1, int p2, int p3, int p4, int p5) {}
void __stdcall ZwQueryObject(int p0, int p1, int p2, int p3, int p4) {}
void __stdcall ZwQueryQuotaInformationFile(int p0, int p1, int p2, int p3, int p4, int p5, int p6, int p7, int p8) {}
void __stdcall ZwQuerySecurityObject(int p0, int p1, int p2, int p3, int p4) {}
void __stdcall ZwQuerySymbolicLinkObject(int p0, int p1, int p2) {}
void __stdcall ZwQuerySystemInformation(int p0, int p1, int p2, int p3) {}
void __stdcall ZwQuerySystemTime(int p0) {}
void __stdcall ZwQueryTimerResolution(int p0, int p1, int p2) {}
void __stdcall ZwQueryValueKey(int p0, int p1, int p2, int p3, int p4, int p5) {}
void __stdcall ZwQueryVirtualMemory(int p0, int p1, int p2, int p3, int p4, int p5) {}
void __stdcall ZwQueryVolumeInformationFile(int p0, int p1, int p2, int p3, int p4) {}
void __stdcall ZwReadFile(int p0, int p1, int p2, int p3, int p4, int p5, int p6, int p7, int p8) {}
void __stdcall ZwReadOnlyEnlistment(int p0, int p1) {}
void __stdcall ZwRecoverEnlistment(int p0, int p1) {}
void __stdcall ZwRecoverResourceManager(int p0) {}
void __stdcall ZwRecoverTransactionManager(int p0) {}
void __stdcall ZwRegisterProtocolAddressInformation(int p0, int p1, int p2, int p3, int p4) {}
void __stdcall ZwRenameKey(int p0, int p1) {}
void __stdcall ZwRenameTransactionManager(int p0, int p1) {}
void __stdcall ZwRestoreKey(int p0, int p1, int p2) {}
void __stdcall ZwRollbackComplete(int p0, int p1) {}
void __stdcall ZwRollbackEnlistment(int p0, int p1) {}
void __stdcall ZwRollbackRegistryTransaction(int p0, int p1) {}
void __stdcall ZwRollbackTransaction(int p0, int p1) {}
void __stdcall ZwRollforwardTransactionManager(int p0, int p1) {}
void __stdcall ZwSaveKey(int p0, int p1) {}
void __stdcall ZwSaveKeyEx(int p0, int p1, int p2) {}
void __stdcall ZwSetEaFile(int p0, int p1, int p2, int p3) {}
void __stdcall ZwSetEvent(int p0, int p1) {}
void __stdcall ZwSetInformationEnlistment(int p0, int p1, int p2, int p3) {}
void __stdcall ZwSetInformationFile(int p0, int p1, int p2, int p3, int p4) {}
void __stdcall ZwSetInformationKey(int p0, int p1, int p2, int p3) {}
void __stdcall ZwSetInformationResourceManager(int p0, int p1, int p2, int p3) {}
void __stdcall ZwSetInformationThread(int p0, int p1, int p2, int p3) {}
void __stdcall ZwSetInformationToken(int p0, int p1, int p2, int p3) {}
void __stdcall ZwSetInformationTransaction(int p0, int p1, int p2, int p3) {}
void __stdcall ZwSetInformationTransactionManager(int p0, int p1, int p2, int p3) {}
void __stdcall ZwSetInformationVirtualMemory(int p0, int p1, int p2, int p3, int p4, int p5) {}
void __stdcall ZwSetQuotaInformationFile(int p0, int p1, int p2, int p3) {}
void __stdcall ZwSetSecurityObject(int p0, int p1, int p2) {}
void __stdcall ZwSetTimer(int p0, int p1, int p2, int p3, int p4, int p5, int p6) {}
void __stdcall ZwSetTimerEx(int p0, int p1, int p2, int p3) {}
void __stdcall ZwSetValueKey(int p0, int p1, int p2, int p3, int p4, int p5) {}
void __stdcall ZwSetVolumeInformationFile(int p0, int p1, int p2, int p3, int p4) {}
void __stdcall ZwSinglePhaseReject(int p0, int p1) {}
void __stdcall ZwTerminateProcess(int p0, int p1) {}
void __stdcall ZwUnloadDriver(int p0) {}
void __stdcall ZwUnlockFile(int p0, int p1, int p2, int p3, int p4) {}
void __stdcall ZwUnmapViewOfSection(int p0, int p1) {}
void __stdcall ZwWaitForSingleObject(int p0, int p1, int p2) {}
void __stdcall ZwWriteFile(int p0, int p1, int p2, int p3, int p4, int p5, int p6, int p7, int p8) {}
void __stdcall vDbgPrintEx(int p0, int p1, int p2, int p3) {}
void __stdcall vDbgPrintExWithPrefix(int p0, int p1, int p2, int p3, int p4) {}

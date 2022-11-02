void __stdcall NtClose(int p0) {}
void __stdcall NtCreateFile(int p0, int p1, int p2, int p3, int p4, int p5, int p6, int p7, int p8, int p9, int p10) {}
void __stdcall NtDeviceIoControlFile(int p0, int p1, int p2, int p3, int p4, int p5, int p6, int p7, int p8, int p9) {}
void __stdcall NtNotifyChangeMultipleKeys(int p0, int p1, int p2, int p3, int p4, int p5, int p6, int p7, int p8, int p9, int p10, int p11) {}
void __stdcall NtOpenFile(int p0, int p1, int p2, int p3, int p4, int p5) {}
void __stdcall NtQueryInformationProcess(int p0, int p1, int p2, int p3, int p4) {}
void __stdcall NtQueryInformationThread(int p0, int p1, int p2, int p3, int p4) {}
void __stdcall NtQueryMultipleValueKey(int p0, int p1, int p2, int p3, int p4, int p5) {}
void __stdcall NtQueryObject(int p0, int p1, int p2, int p3, int p4) {}
void __stdcall NtQuerySystemInformation(int p0, int p1, int p2, int p3) {}
void __stdcall NtQuerySystemTime(int p0) {}
void __stdcall NtQueryTimerResolution(int p0, int p1, int p2) {}
void __stdcall NtRenameKey(int p0, int p1) {}
void __stdcall NtSetInformationKey(int p0, int p1, int p2, int p3) {}
void __stdcall NtSetInformationThread(int p0, int p1, int p2, int p3) {}
void __stdcall NtWaitForSingleObject(int p0, int p1, int p2) {}
void __stdcall RtlAddGrowableFunctionTable(int p0, int p1, int p2, int p3, int p4, int p5) {}
void __stdcall RtlAnsiStringToUnicodeString(int p0, int p1, int p2) {}
void __stdcall RtlCharToInteger(int p0, int p1, int p2) {}
void __stdcall RtlConvertDeviceFamilyInfoToString(int p0, int p1, int p2, int p3) {}
void __stdcall RtlConvertSidToUnicodeString(int p0, int p1, int p2) {}
void __stdcall RtlCrc32(int p0, int p1, int p2) {}
void __stdcall RtlCrc64(int p0, int p1, int p2, int p3) {}
void __stdcall RtlDeleteGrowableFunctionTable(int p0) {}
void __stdcall RtlDrainNonVolatileFlush(int p0) {}
void __stdcall RtlEthernetAddressToStringA(int p0, int p1) {}
void __stdcall RtlEthernetAddressToStringW(int p0, int p1) {}
void __stdcall RtlEthernetStringToAddressA(int p0, int p1, int p2) {}
void __stdcall RtlEthernetStringToAddressW(int p0, int p1, int p2) {}
void __stdcall RtlExtendCorrelationVector(int p0) {}
void __stdcall RtlFillNonVolatileMemory(int p0, int p1, int p2, int p3, int p4) {}
void __stdcall RtlFirstEntrySList(int p0) {}
void __stdcall RtlFlushNonVolatileMemory(int p0, int p1, int p2, int p3) {}
void __stdcall RtlFlushNonVolatileMemoryRanges(int p0, int p1, int p2, int p3) {}
void __stdcall RtlFreeAnsiString(int p0) {}
void __stdcall RtlFreeNonVolatileToken(int p0) {}
void __stdcall RtlFreeOemString(int p0) {}
void __stdcall RtlFreeUnicodeString(int p0) {}
void __stdcall RtlGetDeviceFamilyInfoEnum(int p0, int p1, int p2) {}
void __stdcall RtlGetNonVolatileToken(int p0, int p1, int p2) {}
void __stdcall RtlGetProductInfo(int p0, int p1, int p2, int p3, int p4) {}
void __stdcall RtlGetReturnAddressHijackTarget() {}
void __stdcall RtlGrowFunctionTable(int p0, int p1) {}
void __stdcall RtlIncrementCorrelationVector(int p0) {}
void __stdcall RtlInitAnsiString(int p0, int p1) {}
void __stdcall RtlInitAnsiStringEx(int p0, int p1) {}
void __stdcall RtlInitString(int p0, int p1) {}
void __stdcall RtlInitStringEx(int p0, int p1) {}
void __stdcall RtlInitUnicodeString(int p0, int p1) {}
void __stdcall RtlInitializeCorrelationVector(int p0, int p1, int p2) {}
void __stdcall RtlInitializeSListHead(int p0) {}
void __stdcall RtlInterlockedFlushSList(int p0) {}
void __stdcall RtlInterlockedPopEntrySList(int p0) {}
void __stdcall RtlInterlockedPushEntrySList(int p0, int p1) {}
void __stdcall RtlInterlockedPushListSListEx(int p0, int p1, int p2, int p3) {}
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
void __stdcall RtlIsNameLegalDOS8Dot3(int p0, int p1, int p2) {}
void __stdcall RtlIsZeroMemory(int p0, int p1) {}
void __stdcall RtlLocalTimeToSystemTime(int p0, int p1) {}
void __stdcall RtlNormalizeSecurityDescriptor(int p0, int p1, int p2, int p3, int p4) {}
void __stdcall RtlNtStatusToDosError(int p0) {}
void __stdcall RtlOsDeploymentState(int p0) {}
void __stdcall RtlQueryDepthSList(int p0) {}
void __stdcall RtlRaiseCustomSystemEventTrigger(int p0) {}
void __stdcall RtlSwitchedVVI(int p0, int p1, int p2, int p3) {}
void __stdcall RtlTimeToSecondsSince1970(int p0, int p1) {}
void __stdcall RtlUnicodeStringToAnsiString(int p0, int p1, int p2) {}
void __stdcall RtlUnicodeStringToOemString(int p0, int p1, int p2) {}
void __stdcall RtlUnicodeToMultiByteSize(int p0, int p1, int p2) {}
void __stdcall RtlUniform(int p0) {}
void __stdcall RtlValidateCorrelationVector(int p0) {}
void __stdcall RtlWriteNonVolatileMemory(int p0, int p1, int p2, int p3, int p4) {}

void __stdcall NdisAcquireReadWriteLock(int p0, int p1, int p2) {}
void __stdcall NdisAllocateMemoryWithTag(int p0, int p1, int p2) {}
void __stdcall NdisCancelTimer(int p0, int p1) {}
void __stdcall NdisClAddParty(int p0, int p1, int p2, int p3) {}
void __stdcall NdisClCloseAddressFamily(int p0) {}
void __stdcall NdisClCloseCall(int p0, int p1, int p2, int p3) {}
void __stdcall NdisClDeregisterSap(int p0) {}
void __stdcall NdisClDropParty(int p0, int p1, int p2) {}
void __stdcall NdisClGetProtocolVcContextFromTapiCallId(int p0, int p1, int p2) {}
void __stdcall NdisClIncomingCallComplete(int p0, int p1, int p2) {}
void __stdcall NdisClMakeCall(int p0, int p1, int p2, int p3) {}
void __stdcall NdisClModifyCallQoS(int p0, int p1) {}
void __stdcall NdisClRegisterSap(int p0, int p1, int p2, int p3) {}
void __stdcall NdisCloseConfiguration(int p0) {}
void __stdcall NdisCloseFile(int p0) {}
void __stdcall NdisCmActivateVc(int p0, int p1) {}
void __stdcall NdisCmAddPartyComplete(int p0, int p1, int p2, int p3) {}
void __stdcall NdisCmCloseAddressFamilyComplete(int p0, int p1) {}
void __stdcall NdisCmCloseCallComplete(int p0, int p1, int p2) {}
void __stdcall NdisCmDeactivateVc(int p0) {}
void __stdcall NdisCmDeregisterSapComplete(int p0, int p1) {}
void __stdcall NdisCmDispatchCallConnected(int p0) {}
void __stdcall NdisCmDispatchIncomingCall(int p0, int p1, int p2) {}
void __stdcall NdisCmDispatchIncomingCallQoSChange(int p0, int p1) {}
void __stdcall NdisCmDispatchIncomingCloseCall(int p0, int p1, int p2, int p3) {}
void __stdcall NdisCmDispatchIncomingDropParty(int p0, int p1, int p2, int p3) {}
void __stdcall NdisCmDropPartyComplete(int p0, int p1) {}
void __stdcall NdisCmMakeCallComplete(int p0, int p1, int p2, int p3, int p4) {}
void __stdcall NdisCmModifyCallQoSComplete(int p0, int p1, int p2) {}
void __stdcall NdisCmOpenAddressFamilyComplete(int p0, int p1, int p2) {}
void __stdcall NdisCmRegisterSapComplete(int p0, int p1, int p2) {}
void __stdcall NdisCoAssignInstanceName(int p0, int p1, int p2) {}
void __stdcall NdisCoCreateVc(int p0, int p1, int p2, int p3) {}
void __stdcall NdisCoDeleteVc(int p0) {}
void __stdcall NdisCoGetTapiCallId(int p0, int p1) {}
void __stdcall NdisCompleteDmaTransfer(int p0, int p1, int p2, int p3, int p4, int p5) {}
void __stdcall NdisCopyBuffer(int p0, int p1, int p2, int p3, int p4, int p5) {}
void __stdcall NdisDeregisterTdiCallBack() {}
void __stdcall NdisFreeMemory(int p0, int p1, int p2) {}
void __stdcall NdisGeneratePartialCancelId() {}
void __stdcall NdisGetCurrentProcessorCounts(int p0, int p1, int p2) {}
void __stdcall NdisGetCurrentProcessorCpuUsage(int p0) {}
void __stdcall NdisGetRoutineAddress(int p0) {}
void __stdcall NdisGetSharedDataAlignment() {}
void __stdcall NdisGetVersion() {}
void __stdcall NdisIMAssociateMiniport(int p0, int p1) {}
void __stdcall NdisIMCancelInitializeDeviceInstance(int p0, int p1) {}
void __stdcall NdisIMDeInitializeDeviceInstance(int p0) {}
void __stdcall NdisIMGetBindingContext(int p0) {}
void __stdcall NdisIMInitializeDeviceInstanceEx(int p0, int p1, int p2) {}
void __stdcall NdisInitializeEvent(int p0) {}
void __stdcall NdisInitializeReadWriteLock(int p0) {}
void __stdcall NdisInitializeString(int p0, int p1) {}
void __stdcall NdisInitializeTimer(int p0, int p1, int p2) {}
void __stdcall NdisMAllocateSharedMemory(int p0, int p1, int p2, int p3, int p4) {}
void __stdcall NdisMAllocateSharedMemoryAsync(int p0, int p1, int p2, int p3) {}
void __stdcall NdisMCancelTimer(int p0, int p1) {}
void __stdcall NdisMCloseLog(int p0) {}
void __stdcall NdisMCmActivateVc(int p0, int p1) {}
void __stdcall NdisMCmCreateVc(int p0, int p1, int p2, int p3) {}
void __stdcall NdisMCmDeactivateVc(int p0) {}
void __stdcall NdisMCmDeleteVc(int p0) {}
void __stdcall NdisMCmRegisterAddressFamily(int p0, int p1, int p2, int p3) {}
void __stdcall NdisMCoActivateVcComplete(int p0, int p1, int p2) {}
void __stdcall NdisMCoDeactivateVcComplete(int p0, int p1) {}
void __stdcall NdisMCreateLog(int p0, int p1, int p2) {}
void __stdcall NdisMDeregisterDmaChannel(int p0) {}
void __stdcall NdisMDeregisterIoPortRange(int p0, int p1, int p2, int p3) {}
void __stdcall NdisMFlushLog(int p0) {}
void __stdcall NdisMFreeSharedMemory(int p0, int p1, int p2, int p3, int p4, int p5) {}
void __stdcall NdisMGetDeviceProperty(int p0, int p1, int p2, int p3, int p4, int p5) {}
void __stdcall NdisMGetDmaAlignment(int p0) {}
void __stdcall NdisMInitializeTimer(int p0, int p1, int p2, int p3) {}
void __stdcall NdisMMapIoSpace(int p0, int p1, int p2, int p3, int p4) {}
void __stdcall NdisMQueryAdapterInstanceName(int p0, int p1) {}
void __stdcall NdisMReadDmaCounter(int p0) {}
void __stdcall NdisMRegisterDmaChannel(int p0, int p1, int p2, int p3, int p4, int p5) {}
void __stdcall NdisMRegisterIoPortRange(int p0, int p1, int p2, int p3) {}
void __stdcall NdisMRemoveMiniport(int p0) {}
void __stdcall NdisMSetPeriodicTimer(int p0, int p1) {}
void __stdcall NdisMSleep(int p0) {}
void __stdcall NdisMUnmapIoSpace(int p0, int p1, int p2) {}
void __stdcall NdisMWriteLogData(int p0, int p1, int p2) {}
void __stdcall NdisMapFile(int p0, int p1, int p2) {}
void __stdcall NdisOpenConfigurationKeyByIndex(int p0, int p1, int p2, int p3, int p4) {}
void __stdcall NdisOpenConfigurationKeyByName(int p0, int p1, int p2, int p3) {}
void __stdcall NdisOpenFile(int p0, int p1, int p2, int p3, int p4, int p5) {}
void __stdcall NdisQueryAdapterInstanceName(int p0, int p1) {}
void __stdcall NdisQueryBindInstanceName(int p0, int p1) {}
void __stdcall NdisReEnumerateProtocolBindings(int p0) {}
void __stdcall NdisReadConfiguration(int p0, int p1, int p2, int p3, int p4) {}
void __stdcall NdisReadNetworkAddress(int p0, int p1, int p2, int p3) {}
void __stdcall NdisRegisterTdiCallBack(int p0, int p1) {}
void __stdcall NdisReleaseReadWriteLock(int p0, int p1) {}
void __stdcall NdisResetEvent(int p0) {}
void __stdcall NdisSetEvent(int p0) {}
void __stdcall NdisSetPeriodicTimer(int p0, int p1) {}
void __stdcall NdisSetTimer(int p0, int p1) {}
void __stdcall NdisSetTimerEx(int p0, int p1, int p2) {}
void __stdcall NdisSetupDmaTransfer(int p0, int p1, int p2, int p3, int p4, int p5) {}
void __stdcall NdisSystemProcessorCount() {}
void __stdcall NdisUnmapFile(int p0) {}
void __stdcall NdisUpdateSharedMemory(int p0, int p1, int p2, int p3, int p4) {}
void __stdcall NdisWaitEvent(int p0, int p1) {}
void __stdcall NdisWriteConfiguration(int p0, int p1, int p2, int p3) {}
void __cdecl NdisWriteErrorLogEntry() {}
void __stdcall NdisWriteEventLogEntry(int p0, int p1, int p2, int p3, int p4, int p5, int p6) {}

void __stdcall WHvAcceptPartitionMigration(int p0, int p1) {}
void __stdcall WHvAdviseGpaRange(int p0, int p1, int p2, int p3, int p4, int p5) {}
void __stdcall WHvAllocateVpciResource(int p0, int p1, int p2, int p3, int p4) {}
void __stdcall WHvCancelPartitionMigration(int p0) {}
void __stdcall WHvCancelRunVirtualProcessor(int p0, int p1, int p2) {}
void __stdcall WHvCompletePartitionMigration(int p0) {}
void __stdcall WHvCreateNotificationPort(int p0, int p1, int p2, int p3) {}
void __stdcall WHvCreatePartition(int p0) {}
void __stdcall WHvCreateTrigger(int p0, int p1, int p2, int p3) {}
void __stdcall WHvCreateVirtualProcessor(int p0, int p1, int p2) {}
void __stdcall WHvCreateVirtualProcessor2(int p0, int p1, int p2, int p3) {}
void __stdcall WHvCreateVpciDevice(int p0, int p1, int p2, int p3, int p4, int p5) {}
void __stdcall WHvDeleteNotificationPort(int p0, int p1) {}
void __stdcall WHvDeletePartition(int p0) {}
void __stdcall WHvDeleteTrigger(int p0, int p1) {}
void __stdcall WHvDeleteVirtualProcessor(int p0, int p1) {}
void __stdcall WHvDeleteVpciDevice(int p0, int p1, int p2) {}
void __stdcall WHvGetCapability(int p0, int p1, int p2, int p3) {}
void __stdcall WHvGetInterruptTargetVpSet(int p0, int p1, int p2, int p3, int p4, int p5, int p6) {}
void __stdcall WHvGetPartitionCounters(int p0, int p1, int p2, int p3, int p4) {}
void __stdcall WHvGetPartitionProperty(int p0, int p1, int p2, int p3, int p4) {}
void __stdcall WHvGetVirtualProcessorCounters(int p0, int p1, int p2, int p3, int p4, int p5) {}
void __stdcall WHvGetVirtualProcessorCpuidOutput(int p0, int p1, int p2, int p3, int p4) {}
void __stdcall WHvGetVirtualProcessorInterruptControllerState(int p0, int p1, int p2, int p3, int p4) {}
void __stdcall WHvGetVirtualProcessorInterruptControllerState2(int p0, int p1, int p2, int p3, int p4) {}
void __stdcall WHvGetVirtualProcessorRegisters(int p0, int p1, int p2, int p3, int p4) {}
void __stdcall WHvGetVirtualProcessorState(int p0, int p1, int p2, int p3, int p4, int p5) {}
void __stdcall WHvGetVirtualProcessorXsaveState(int p0, int p1, int p2, int p3, int p4) {}
void __stdcall WHvGetVpciDeviceInterruptTarget(int p0, int p1, int p2, int p3, int p4, int p5, int p6, int p7) {}
void __stdcall WHvGetVpciDeviceNotification(int p0, int p1, int p2, int p3, int p4) {}
void __stdcall WHvGetVpciDeviceProperty(int p0, int p1, int p2, int p3, int p4, int p5, int p6) {}
void __stdcall WHvMapGpaRange(int p0, int p1, int p2, int p3, int p4, int p5, int p6) {}
void __stdcall WHvMapGpaRange2(int p0, int p1, int p2, int p3, int p4, int p5, int p6, int p7) {}
void __stdcall WHvMapVpciDeviceInterrupt(int p0, int p1, int p2, int p3, int p4, int p5, int p6, int p7) {}
void __stdcall WHvMapVpciDeviceMmioRanges(int p0, int p1, int p2, int p3, int p4) {}
void __stdcall WHvPostVirtualProcessorSynicMessage(int p0, int p1, int p2, int p3, int p4) {}
void __stdcall WHvQueryGpaRangeDirtyBitmap(int p0, int p1, int p2, int p3, int p4, int p5, int p6) {}
void __stdcall WHvReadGpaRange(int p0, int p1, int p2, int p3, int p4, int p5, int p6, int p7) {}
void __stdcall WHvReadVpciDeviceRegister(int p0, int p1, int p2, int p3, int p4) {}
void __stdcall WHvRegisterPartitionDoorbellEvent(int p0, int p1, int p2) {}
void __stdcall WHvRequestInterrupt(int p0, int p1, int p2) {}
void __stdcall WHvRequestVpciDeviceInterrupt(int p0, int p1, int p2, int p3, int p4, int p5) {}
void __stdcall WHvResetPartition(int p0) {}
void __stdcall WHvResumePartitionTime(int p0) {}
void __stdcall WHvRetargetVpciDeviceInterrupt(int p0, int p1, int p2, int p3, int p4, int p5, int p6) {}
void __stdcall WHvRunVirtualProcessor(int p0, int p1, int p2, int p3) {}
void __stdcall WHvSetNotificationPortProperty(int p0, int p1, int p2, int p3, int p4) {}
void __stdcall WHvSetPartitionProperty(int p0, int p1, int p2, int p3) {}
void __stdcall WHvSetVirtualProcessorInterruptControllerState(int p0, int p1, int p2, int p3) {}
void __stdcall WHvSetVirtualProcessorInterruptControllerState2(int p0, int p1, int p2, int p3) {}
void __stdcall WHvSetVirtualProcessorRegisters(int p0, int p1, int p2, int p3, int p4) {}
void __stdcall WHvSetVirtualProcessorState(int p0, int p1, int p2, int p3, int p4) {}
void __stdcall WHvSetVirtualProcessorXsaveState(int p0, int p1, int p2, int p3) {}
void __stdcall WHvSetVpciDevicePowerState(int p0, int p1, int p2, int p3) {}
void __stdcall WHvSetupPartition(int p0) {}
void __stdcall WHvSignalVirtualProcessorSynicEvent(int p0, int p1, int p2, int p3) {}
void __stdcall WHvStartPartitionMigration(int p0, int p1) {}
void __stdcall WHvSuspendPartitionTime(int p0) {}
void __stdcall WHvTranslateGva(int p0, int p1, int p2, int p3, int p4, int p5, int p6) {}
void __stdcall WHvUnmapGpaRange(int p0, int p1, int p2, int p3, int p4) {}
void __stdcall WHvUnmapVpciDeviceInterrupt(int p0, int p1, int p2, int p3) {}
void __stdcall WHvUnmapVpciDeviceMmioRanges(int p0, int p1, int p2) {}
void __stdcall WHvUnregisterPartitionDoorbellEvent(int p0, int p1) {}
void __stdcall WHvUpdateTriggerParameters(int p0, int p1, int p2) {}
void __stdcall WHvWriteGpaRange(int p0, int p1, int p2, int p3, int p4, int p5, int p6, int p7) {}
void __stdcall WHvWriteVpciDeviceRegister(int p0, int p1, int p2, int p3, int p4) {}

#![allow(non_snake_case, non_camel_case_types)]
#[link(name = "windows")]
extern "system" {
    #[doc = "*Required features: `Win32_System_Hypervisor`*"]
    pub fn ApplyGuestMemoryFix();
    #[doc = "*Required features: `Win32_System_Hypervisor`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ApplyPendingSavedStateFileReplayLog();
    #[doc = "*Required features: `Win32_System_Hypervisor`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CallStackUnwind();
    #[doc = "*Required features: `Win32_System_Hypervisor`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn FindSavedStateSymbolFieldInType();
    #[doc = "*Required features: `Win32_System_Hypervisor`*"]
    pub fn ForceActiveVirtualTrustLevel();
    #[doc = "*Required features: `Win32_System_Hypervisor`*"]
    pub fn ForceArchitecture();
    #[doc = "*Required features: `Win32_System_Hypervisor`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ForceNestedHostMode();
    #[doc = "*Required features: `Win32_System_Hypervisor`*"]
    pub fn ForcePagingMode();
    #[doc = "*Required features: `Win32_System_Hypervisor`*"]
    pub fn GetActiveVirtualTrustLevel();
    #[doc = "*Required features: `Win32_System_Hypervisor`*"]
    pub fn GetArchitecture();
    #[doc = "*Required features: `Win32_System_Hypervisor`*"]
    pub fn GetEnabledVirtualTrustLevels();
    #[doc = "*Required features: `Win32_System_Hypervisor`*"]
    pub fn GetGuestEnabledVirtualTrustLevels();
    #[doc = "*Required features: `Win32_System_Hypervisor`*"]
    pub fn GetGuestOsInfo();
    #[doc = "*Required features: `Win32_System_Hypervisor`*"]
    pub fn GetGuestPhysicalMemoryChunks();
    #[doc = "*Required features: `Win32_System_Hypervisor`*"]
    pub fn GetGuestRawSavedMemorySize();
    #[doc = "*Required features: `Win32_System_Hypervisor`*"]
    pub fn GetMemoryBlockCacheLimit();
    #[doc = "*Required features: `Win32_System_Hypervisor`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetNestedVirtualizationMode();
    #[doc = "*Required features: `Win32_System_Hypervisor`*"]
    pub fn GetPagingMode();
    #[doc = "*Required features: `Win32_System_Hypervisor`*"]
    pub fn GetRegisterValue();
    #[doc = "*Required features: `Win32_System_Hypervisor`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetSavedStateSymbolFieldInfo();
    #[doc = "*Required features: `Win32_System_Hypervisor`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetSavedStateSymbolProviderHandle();
    #[doc = "*Required features: `Win32_System_Hypervisor`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetSavedStateSymbolTypeSize();
    #[doc = "*Required features: `Win32_System_Hypervisor`*"]
    pub fn GetVpCount();
    #[doc = "*Required features: `Win32_System_Hypervisor`*"]
    pub fn GuestPhysicalAddressToRawSavedMemoryOffset();
    #[doc = "*Required features: `Win32_System_Hypervisor`*"]
    pub fn GuestVirtualAddressToPhysicalAddress();
    #[doc = "*Required features: `Win32_System_Hypervisor`*"]
    pub fn HdvCreateDeviceInstance();
    #[doc = "*Required features: `Win32_System_Hypervisor`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn HdvCreateGuestMemoryAperture();
    #[doc = "*Required features: `Win32_System_Hypervisor`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn HdvCreateSectionBackedMmioRange();
    #[doc = "*Required features: `Win32_System_Hypervisor`*"]
    pub fn HdvDeliverGuestInterrupt();
    #[doc = "*Required features: `Win32_System_Hypervisor`*"]
    pub fn HdvDestroyGuestMemoryAperture();
    #[doc = "*Required features: `Win32_System_Hypervisor`*"]
    pub fn HdvDestroySectionBackedMmioRange();
    #[doc = "*Required features: `Win32_System_Hypervisor`, `Win32_System_HostComputeSystem`*"]
    #[cfg(feature = "Win32_System_HostComputeSystem")]
    pub fn HdvInitializeDeviceHost();
    #[doc = "*Required features: `Win32_System_Hypervisor`*"]
    pub fn HdvReadGuestMemory();
    #[doc = "*Required features: `Win32_System_Hypervisor`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn HdvRegisterDoorbell();
    #[doc = "*Required features: `Win32_System_Hypervisor`*"]
    pub fn HdvTeardownDeviceHost();
    #[doc = "*Required features: `Win32_System_Hypervisor`*"]
    pub fn HdvUnregisterDoorbell();
    #[doc = "*Required features: `Win32_System_Hypervisor`*"]
    pub fn HdvWriteGuestMemory();
    #[doc = "*Required features: `Win32_System_Hypervisor`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn InKernelSpace();
    #[doc = "*Required features: `Win32_System_Hypervisor`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn IsActiveVirtualTrustLevelEnabled();
    #[doc = "*Required features: `Win32_System_Hypervisor`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn IsNestedVirtualizationEnabled();
    #[doc = "*Required features: `Win32_System_Hypervisor`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn LoadSavedStateFile();
    #[doc = "*Required features: `Win32_System_Hypervisor`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn LoadSavedStateFiles();
    #[doc = "*Required features: `Win32_System_Hypervisor`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn LoadSavedStateModuleSymbols();
    #[doc = "*Required features: `Win32_System_Hypervisor`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn LoadSavedStateModuleSymbolsEx();
    #[doc = "*Required features: `Win32_System_Hypervisor`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn LoadSavedStateSymbolProvider();
    #[doc = "*Required features: `Win32_System_Hypervisor`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn LocateSavedStateFiles();
    #[doc = "*Required features: `Win32_System_Hypervisor`*"]
    pub fn ReadGuestPhysicalAddress();
    #[doc = "*Required features: `Win32_System_Hypervisor`*"]
    pub fn ReadGuestRawSavedMemory();
    #[doc = "*Required features: `Win32_System_Hypervisor`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ReadSavedStateGlobalVariable();
    #[doc = "*Required features: `Win32_System_Hypervisor`*"]
    pub fn ReleaseSavedStateFiles();
    #[doc = "*Required features: `Win32_System_Hypervisor`*"]
    pub fn ReleaseSavedStateSymbolProvider();
    #[doc = "*Required features: `Win32_System_Hypervisor`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ResolveSavedStateGlobalVariableAddress();
    #[doc = "*Required features: `Win32_System_Hypervisor`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ScanMemoryForDosImages();
    #[doc = "*Required features: `Win32_System_Hypervisor`*"]
    pub fn SetMemoryBlockCacheLimit();
    #[doc = "*Required features: `Win32_System_Hypervisor`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetSavedStateSymbolProviderDebugInfoCallback();
    #[doc = "*Required features: `Win32_System_Hypervisor`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WHvAcceptPartitionMigration();
    #[doc = "*Required features: `Win32_System_Hypervisor`*"]
    pub fn WHvAdviseGpaRange();
    #[doc = "*Required features: `Win32_System_Hypervisor`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WHvAllocateVpciResource();
    #[doc = "*Required features: `Win32_System_Hypervisor`*"]
    pub fn WHvCancelPartitionMigration();
    #[doc = "*Required features: `Win32_System_Hypervisor`*"]
    pub fn WHvCancelRunVirtualProcessor();
    #[doc = "*Required features: `Win32_System_Hypervisor`*"]
    pub fn WHvCompletePartitionMigration();
    #[doc = "*Required features: `Win32_System_Hypervisor`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WHvCreateNotificationPort();
    #[doc = "*Required features: `Win32_System_Hypervisor`*"]
    pub fn WHvCreatePartition();
    #[doc = "*Required features: `Win32_System_Hypervisor`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WHvCreateTrigger();
    #[doc = "*Required features: `Win32_System_Hypervisor`*"]
    pub fn WHvCreateVirtualProcessor();
    #[doc = "*Required features: `Win32_System_Hypervisor`*"]
    pub fn WHvCreateVirtualProcessor2();
    #[doc = "*Required features: `Win32_System_Hypervisor`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WHvCreateVpciDevice();
    #[doc = "*Required features: `Win32_System_Hypervisor`*"]
    pub fn WHvDeleteNotificationPort();
    #[doc = "*Required features: `Win32_System_Hypervisor`*"]
    pub fn WHvDeletePartition();
    #[doc = "*Required features: `Win32_System_Hypervisor`*"]
    pub fn WHvDeleteTrigger();
    #[doc = "*Required features: `Win32_System_Hypervisor`*"]
    pub fn WHvDeleteVirtualProcessor();
    #[doc = "*Required features: `Win32_System_Hypervisor`*"]
    pub fn WHvDeleteVpciDevice();
    #[doc = "*Required features: `Win32_System_Hypervisor`*"]
    pub fn WHvEmulatorCreateEmulator();
    #[doc = "*Required features: `Win32_System_Hypervisor`*"]
    pub fn WHvEmulatorDestroyEmulator();
    #[doc = "*Required features: `Win32_System_Hypervisor`*"]
    pub fn WHvEmulatorTryIoEmulation();
    #[doc = "*Required features: `Win32_System_Hypervisor`*"]
    pub fn WHvEmulatorTryMmioEmulation();
    #[doc = "*Required features: `Win32_System_Hypervisor`*"]
    pub fn WHvGetCapability();
    #[doc = "*Required features: `Win32_System_Hypervisor`*"]
    pub fn WHvGetInterruptTargetVpSet();
    #[doc = "*Required features: `Win32_System_Hypervisor`*"]
    pub fn WHvGetPartitionCounters();
    #[doc = "*Required features: `Win32_System_Hypervisor`*"]
    pub fn WHvGetPartitionProperty();
    #[doc = "*Required features: `Win32_System_Hypervisor`*"]
    pub fn WHvGetVirtualProcessorCounters();
    #[doc = "*Required features: `Win32_System_Hypervisor`*"]
    pub fn WHvGetVirtualProcessorCpuidOutput();
    #[doc = "*Required features: `Win32_System_Hypervisor`*"]
    pub fn WHvGetVirtualProcessorInterruptControllerState();
    #[doc = "*Required features: `Win32_System_Hypervisor`*"]
    pub fn WHvGetVirtualProcessorInterruptControllerState2();
    #[doc = "*Required features: `Win32_System_Hypervisor`*"]
    pub fn WHvGetVirtualProcessorRegisters();
    #[doc = "*Required features: `Win32_System_Hypervisor`*"]
    pub fn WHvGetVirtualProcessorState();
    #[doc = "*Required features: `Win32_System_Hypervisor`*"]
    pub fn WHvGetVirtualProcessorXsaveState();
    #[doc = "*Required features: `Win32_System_Hypervisor`*"]
    pub fn WHvGetVpciDeviceInterruptTarget();
    #[doc = "*Required features: `Win32_System_Hypervisor`*"]
    pub fn WHvGetVpciDeviceNotification();
    #[doc = "*Required features: `Win32_System_Hypervisor`*"]
    pub fn WHvGetVpciDeviceProperty();
    #[doc = "*Required features: `Win32_System_Hypervisor`*"]
    pub fn WHvMapGpaRange();
    #[doc = "*Required features: `Win32_System_Hypervisor`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WHvMapGpaRange2();
    #[doc = "*Required features: `Win32_System_Hypervisor`*"]
    pub fn WHvMapVpciDeviceInterrupt();
    #[doc = "*Required features: `Win32_System_Hypervisor`*"]
    pub fn WHvMapVpciDeviceMmioRanges();
    #[doc = "*Required features: `Win32_System_Hypervisor`*"]
    pub fn WHvPostVirtualProcessorSynicMessage();
    #[doc = "*Required features: `Win32_System_Hypervisor`*"]
    pub fn WHvQueryGpaRangeDirtyBitmap();
    #[doc = "*Required features: `Win32_System_Hypervisor`*"]
    pub fn WHvReadGpaRange();
    #[doc = "*Required features: `Win32_System_Hypervisor`*"]
    pub fn WHvReadVpciDeviceRegister();
    #[doc = "*Required features: `Win32_System_Hypervisor`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WHvRegisterPartitionDoorbellEvent();
    #[doc = "*Required features: `Win32_System_Hypervisor`*"]
    pub fn WHvRequestInterrupt();
    #[doc = "*Required features: `Win32_System_Hypervisor`*"]
    pub fn WHvRequestVpciDeviceInterrupt();
    #[doc = "*Required features: `Win32_System_Hypervisor`*"]
    pub fn WHvResetPartition();
    #[doc = "*Required features: `Win32_System_Hypervisor`*"]
    pub fn WHvResumePartitionTime();
    #[doc = "*Required features: `Win32_System_Hypervisor`*"]
    pub fn WHvRetargetVpciDeviceInterrupt();
    #[doc = "*Required features: `Win32_System_Hypervisor`*"]
    pub fn WHvRunVirtualProcessor();
    #[doc = "*Required features: `Win32_System_Hypervisor`*"]
    pub fn WHvSetNotificationPortProperty();
    #[doc = "*Required features: `Win32_System_Hypervisor`*"]
    pub fn WHvSetPartitionProperty();
    #[doc = "*Required features: `Win32_System_Hypervisor`*"]
    pub fn WHvSetVirtualProcessorInterruptControllerState();
    #[doc = "*Required features: `Win32_System_Hypervisor`*"]
    pub fn WHvSetVirtualProcessorInterruptControllerState2();
    #[doc = "*Required features: `Win32_System_Hypervisor`*"]
    pub fn WHvSetVirtualProcessorRegisters();
    #[doc = "*Required features: `Win32_System_Hypervisor`*"]
    pub fn WHvSetVirtualProcessorState();
    #[doc = "*Required features: `Win32_System_Hypervisor`*"]
    pub fn WHvSetVirtualProcessorXsaveState();
    #[doc = "*Required features: `Win32_System_Hypervisor`, `Win32_System_Power`*"]
    #[cfg(feature = "Win32_System_Power")]
    pub fn WHvSetVpciDevicePowerState();
    #[doc = "*Required features: `Win32_System_Hypervisor`*"]
    pub fn WHvSetupPartition();
    #[doc = "*Required features: `Win32_System_Hypervisor`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WHvSignalVirtualProcessorSynicEvent();
    #[doc = "*Required features: `Win32_System_Hypervisor`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WHvStartPartitionMigration();
    #[doc = "*Required features: `Win32_System_Hypervisor`*"]
    pub fn WHvSuspendPartitionTime();
    #[doc = "*Required features: `Win32_System_Hypervisor`*"]
    pub fn WHvTranslateGva();
    #[doc = "*Required features: `Win32_System_Hypervisor`*"]
    pub fn WHvUnmapGpaRange();
    #[doc = "*Required features: `Win32_System_Hypervisor`*"]
    pub fn WHvUnmapVpciDeviceInterrupt();
    #[doc = "*Required features: `Win32_System_Hypervisor`*"]
    pub fn WHvUnmapVpciDeviceMmioRanges();
    #[doc = "*Required features: `Win32_System_Hypervisor`*"]
    pub fn WHvUnregisterPartitionDoorbellEvent();
    #[doc = "*Required features: `Win32_System_Hypervisor`*"]
    pub fn WHvUpdateTriggerParameters();
    #[doc = "*Required features: `Win32_System_Hypervisor`*"]
    pub fn WHvWriteGpaRange();
    #[doc = "*Required features: `Win32_System_Hypervisor`*"]
    pub fn WHvWriteVpciDeviceRegister();
}

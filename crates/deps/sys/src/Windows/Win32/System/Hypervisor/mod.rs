#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, clashing_extern_declarations, clippy::all)]
#[link(name = "windows")]
extern "system" {
    pub fn ApplyGuestMemoryFix(vmsavedstatedumphandle: *mut ::core::ffi::c_void, vpid: u32, virtualaddress: u64, fixbuffer: *const ::core::ffi::c_void, fixbuffersize: u32) -> ::windows_sys::core::HRESULT;
    #[cfg(feature = "Win32_Foundation")]
    pub fn ApplyPendingSavedStateFileReplayLog(vmrsfile: super::super::Foundation::PWSTR) -> ::windows_sys::core::HRESULT;
    #[cfg(feature = "Win32_Foundation")]
    pub fn CallStackUnwind(vmsavedstatedumphandle: *mut ::core::ffi::c_void, vpid: u32, imageinfo: *const MODULE_INFO, imageinfocount: u32, framecount: u32, callstack: *mut super::super::Foundation::PWSTR) -> ::windows_sys::core::HRESULT;
    #[cfg(feature = "Win32_Foundation")]
    pub fn FindSavedStateSymbolFieldInType(vmsavedstatedumphandle: *mut ::core::ffi::c_void, vpid: u32, typename: super::super::Foundation::PSTR, fieldname: super::super::Foundation::PWSTR, offset: *mut u32, found: *mut super::super::Foundation::BOOL) -> ::windows_sys::core::HRESULT;
    pub fn ForceActiveVirtualTrustLevel(vmsavedstatedumphandle: *mut ::core::ffi::c_void, vpid: u32, virtualtrustlevel: u8) -> ::windows_sys::core::HRESULT;
    pub fn ForceArchitecture(vmsavedstatedumphandle: *mut ::core::ffi::c_void, vpid: u32, architecture: VIRTUAL_PROCESSOR_ARCH) -> ::windows_sys::core::HRESULT;
    #[cfg(feature = "Win32_Foundation")]
    pub fn ForceNestedHostMode(vmsavedstatedumphandle: *mut ::core::ffi::c_void, vpid: u32, hostmode: super::super::Foundation::BOOL, oldmode: *mut super::super::Foundation::BOOL) -> ::windows_sys::core::HRESULT;
    pub fn ForcePagingMode(vmsavedstatedumphandle: *mut ::core::ffi::c_void, vpid: u32, pagingmode: PAGING_MODE) -> ::windows_sys::core::HRESULT;
    pub fn GetActiveVirtualTrustLevel(vmsavedstatedumphandle: *mut ::core::ffi::c_void, vpid: u32, virtualtrustlevel: *mut u8) -> ::windows_sys::core::HRESULT;
    pub fn GetArchitecture(vmsavedstatedumphandle: *mut ::core::ffi::c_void, vpid: u32, architecture: *mut VIRTUAL_PROCESSOR_ARCH) -> ::windows_sys::core::HRESULT;
    pub fn GetEnabledVirtualTrustLevels(vmsavedstatedumphandle: *mut ::core::ffi::c_void, vpid: u32, virtualtrustlevels: *mut u32) -> ::windows_sys::core::HRESULT;
    pub fn GetGuestEnabledVirtualTrustLevels(vmsavedstatedumphandle: *mut ::core::ffi::c_void, virtualtrustlevels: *mut u32) -> ::windows_sys::core::HRESULT;
    pub fn GetGuestOsInfo(vmsavedstatedumphandle: *mut ::core::ffi::c_void, virtualtrustlevel: u8, guestosinfo: *mut GUEST_OS_INFO) -> ::windows_sys::core::HRESULT;
    pub fn GetGuestPhysicalMemoryChunks(vmsavedstatedumphandle: *mut ::core::ffi::c_void, memorychunkpagesize: *mut u64, memorychunks: *mut GPA_MEMORY_CHUNK, memorychunkcount: *mut u64) -> ::windows_sys::core::HRESULT;
    pub fn GetGuestRawSavedMemorySize(vmsavedstatedumphandle: *mut ::core::ffi::c_void, guestrawsavedmemorysize: *mut u64) -> ::windows_sys::core::HRESULT;
    pub fn GetMemoryBlockCacheLimit(vmsavedstatedumphandle: *mut ::core::ffi::c_void, memoryblockcachelimit: *mut u64) -> ::windows_sys::core::HRESULT;
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetNestedVirtualizationMode(vmsavedstatedumphandle: *mut ::core::ffi::c_void, vpid: u32, enabled: *mut super::super::Foundation::BOOL) -> ::windows_sys::core::HRESULT;
    pub fn GetPagingMode(vmsavedstatedumphandle: *mut ::core::ffi::c_void, vpid: u32, pagingmode: *mut PAGING_MODE) -> ::windows_sys::core::HRESULT;
    pub fn GetRegisterValue(vmsavedstatedumphandle: *mut ::core::ffi::c_void, vpid: u32, registerid: u32, registervalue: *mut VIRTUAL_PROCESSOR_REGISTER) -> ::windows_sys::core::HRESULT;
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetSavedStateSymbolFieldInfo(vmsavedstatedumphandle: *mut ::core::ffi::c_void, vpid: u32, typename: super::super::Foundation::PSTR, typefieldinfomap: *mut super::super::Foundation::PWSTR) -> ::windows_sys::core::HRESULT;
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetSavedStateSymbolProviderHandle(vmsavedstatedumphandle: *mut ::core::ffi::c_void) -> super::super::Foundation::HANDLE;
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetSavedStateSymbolTypeSize(vmsavedstatedumphandle: *mut ::core::ffi::c_void, vpid: u32, typename: super::super::Foundation::PSTR, size: *mut u32) -> ::windows_sys::core::HRESULT;
    pub fn GetVpCount(vmsavedstatedumphandle: *mut ::core::ffi::c_void, vpcount: *mut u32) -> ::windows_sys::core::HRESULT;
    pub fn GuestPhysicalAddressToRawSavedMemoryOffset(vmsavedstatedumphandle: *mut ::core::ffi::c_void, physicaladdress: u64, rawsavedmemoryoffset: *mut u64) -> ::windows_sys::core::HRESULT;
    pub fn GuestVirtualAddressToPhysicalAddress(vmsavedstatedumphandle: *mut ::core::ffi::c_void, vpid: u32, virtualaddress: u64, physicaladdress: *mut u64, unmappedregionsize: *mut u64) -> ::windows_sys::core::HRESULT;
    pub fn HdvCreateDeviceInstance(devicehosthandle: *const ::core::ffi::c_void, devicetype: HDV_DEVICE_TYPE, deviceclassid: *const ::windows_sys::core::GUID, deviceinstanceid: *const ::windows_sys::core::GUID, deviceinterface: *const ::core::ffi::c_void, devicecontext: *const ::core::ffi::c_void, devicehandle: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT;
    #[cfg(feature = "Win32_Foundation")]
    pub fn HdvCreateGuestMemoryAperture(requestor: *const ::core::ffi::c_void, guestphysicaladdress: u64, bytecount: u32, writeprotected: super::super::Foundation::BOOL, mappedaddress: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT;
    #[cfg(feature = "Win32_Foundation")]
    pub fn HdvCreateSectionBackedMmioRange(requestor: *const ::core::ffi::c_void, barindex: HDV_PCI_BAR_SELECTOR, offsetinpages: u64, lengthinpages: u64, mappingflags: HDV_MMIO_MAPPING_FLAGS, sectionhandle: super::super::Foundation::HANDLE, sectionoffsetinpages: u64) -> ::windows_sys::core::HRESULT;
    pub fn HdvDeliverGuestInterrupt(requestor: *const ::core::ffi::c_void, msiaddress: u64, msidata: u32) -> ::windows_sys::core::HRESULT;
    pub fn HdvDestroyGuestMemoryAperture(requestor: *const ::core::ffi::c_void, mappedaddress: *const ::core::ffi::c_void) -> ::windows_sys::core::HRESULT;
    pub fn HdvDestroySectionBackedMmioRange(requestor: *const ::core::ffi::c_void, barindex: HDV_PCI_BAR_SELECTOR, offsetinpages: u64) -> ::windows_sys::core::HRESULT;
    #[cfg(feature = "Win32_System_HostComputeSystem")]
    pub fn HdvInitializeDeviceHost(computesystem: super::HostComputeSystem::HCS_SYSTEM, devicehosthandle: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT;
    pub fn HdvReadGuestMemory(requestor: *const ::core::ffi::c_void, guestphysicaladdress: u64, bytecount: u32, buffer: *mut u8) -> ::windows_sys::core::HRESULT;
    #[cfg(feature = "Win32_Foundation")]
    pub fn HdvRegisterDoorbell(requestor: *const ::core::ffi::c_void, barindex: HDV_PCI_BAR_SELECTOR, baroffset: u64, triggervalue: u64, flags: u64, doorbellevent: super::super::Foundation::HANDLE) -> ::windows_sys::core::HRESULT;
    pub fn HdvTeardownDeviceHost(devicehosthandle: *const ::core::ffi::c_void) -> ::windows_sys::core::HRESULT;
    pub fn HdvUnregisterDoorbell(requestor: *const ::core::ffi::c_void, barindex: HDV_PCI_BAR_SELECTOR, baroffset: u64, triggervalue: u64, flags: u64) -> ::windows_sys::core::HRESULT;
    pub fn HdvWriteGuestMemory(requestor: *const ::core::ffi::c_void, guestphysicaladdress: u64, bytecount: u32, buffer: *const u8) -> ::windows_sys::core::HRESULT;
    #[cfg(feature = "Win32_Foundation")]
    pub fn InKernelSpace(vmsavedstatedumphandle: *mut ::core::ffi::c_void, vpid: u32, inkernelspace: *mut super::super::Foundation::BOOL) -> ::windows_sys::core::HRESULT;
    #[cfg(feature = "Win32_Foundation")]
    pub fn IsActiveVirtualTrustLevelEnabled(vmsavedstatedumphandle: *mut ::core::ffi::c_void, vpid: u32, activevirtualtrustlevelenabled: *mut super::super::Foundation::BOOL) -> ::windows_sys::core::HRESULT;
    #[cfg(feature = "Win32_Foundation")]
    pub fn IsNestedVirtualizationEnabled(vmsavedstatedumphandle: *mut ::core::ffi::c_void, enabled: *mut super::super::Foundation::BOOL) -> ::windows_sys::core::HRESULT;
    #[cfg(feature = "Win32_Foundation")]
    pub fn LoadSavedStateFile(vmrsfile: super::super::Foundation::PWSTR, vmsavedstatedumphandle: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT;
    #[cfg(feature = "Win32_Foundation")]
    pub fn LoadSavedStateFiles(binfile: super::super::Foundation::PWSTR, vsvfile: super::super::Foundation::PWSTR, vmsavedstatedumphandle: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT;
    #[cfg(feature = "Win32_Foundation")]
    pub fn LoadSavedStateModuleSymbols(vmsavedstatedumphandle: *mut ::core::ffi::c_void, imagename: super::super::Foundation::PSTR, modulename: super::super::Foundation::PSTR, baseaddress: u64, sizeofbase: u32) -> ::windows_sys::core::HRESULT;
    #[cfg(feature = "Win32_Foundation")]
    pub fn LoadSavedStateModuleSymbolsEx(vmsavedstatedumphandle: *mut ::core::ffi::c_void, imagename: super::super::Foundation::PSTR, imagetimestamp: u32, modulename: super::super::Foundation::PSTR, baseaddress: u64, sizeofbase: u32) -> ::windows_sys::core::HRESULT;
    #[cfg(feature = "Win32_Foundation")]
    pub fn LoadSavedStateSymbolProvider(vmsavedstatedumphandle: *mut ::core::ffi::c_void, usersymbols: super::super::Foundation::PWSTR, force: super::super::Foundation::BOOL) -> ::windows_sys::core::HRESULT;
    #[cfg(feature = "Win32_Foundation")]
    pub fn LocateSavedStateFiles(vmname: super::super::Foundation::PWSTR, snapshotname: super::super::Foundation::PWSTR, binpath: *mut super::super::Foundation::PWSTR, vsvpath: *mut super::super::Foundation::PWSTR, vmrspath: *mut super::super::Foundation::PWSTR) -> ::windows_sys::core::HRESULT;
    pub fn ReadGuestPhysicalAddress(vmsavedstatedumphandle: *mut ::core::ffi::c_void, physicaladdress: u64, buffer: *mut ::core::ffi::c_void, buffersize: u32, bytesread: *mut u32) -> ::windows_sys::core::HRESULT;
    pub fn ReadGuestRawSavedMemory(vmsavedstatedumphandle: *mut ::core::ffi::c_void, rawsavedmemoryoffset: u64, buffer: *mut ::core::ffi::c_void, buffersize: u32, bytesread: *mut u32) -> ::windows_sys::core::HRESULT;
    #[cfg(feature = "Win32_Foundation")]
    pub fn ReadSavedStateGlobalVariable(vmsavedstatedumphandle: *mut ::core::ffi::c_void, vpid: u32, globalname: super::super::Foundation::PSTR, buffer: *mut ::core::ffi::c_void, buffersize: u32) -> ::windows_sys::core::HRESULT;
    pub fn ReleaseSavedStateFiles(vmsavedstatedumphandle: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT;
    pub fn ReleaseSavedStateSymbolProvider(vmsavedstatedumphandle: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT;
    #[cfg(feature = "Win32_Foundation")]
    pub fn ResolveSavedStateGlobalVariableAddress(vmsavedstatedumphandle: *mut ::core::ffi::c_void, vpid: u32, globalname: super::super::Foundation::PSTR, virtualaddress: *mut u64, size: *mut u32) -> ::windows_sys::core::HRESULT;
    #[cfg(feature = "Win32_Foundation")]
    pub fn ScanMemoryForDosImages(vmsavedstatedumphandle: *mut ::core::ffi::c_void, vpid: u32, startaddress: u64, endaddress: u64, callbackcontext: *mut ::core::ffi::c_void, foundimagecallback: FOUND_IMAGE_CALLBACK, standaloneaddress: *const u64, standaloneaddresscount: u32) -> ::windows_sys::core::HRESULT;
    pub fn SetMemoryBlockCacheLimit(vmsavedstatedumphandle: *mut ::core::ffi::c_void, memoryblockcachelimit: u64) -> ::windows_sys::core::HRESULT;
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetSavedStateSymbolProviderDebugInfoCallback(vmsavedstatedumphandle: *mut ::core::ffi::c_void, callback: GUEST_SYMBOLS_PROVIDER_DEBUG_INFO_CALLBACK) -> ::windows_sys::core::HRESULT;
    #[cfg(feature = "Win32_Foundation")]
    pub fn WHvAcceptPartitionMigration(migrationhandle: super::super::Foundation::HANDLE, partition: *mut WHV_PARTITION_HANDLE) -> ::windows_sys::core::HRESULT;
    pub fn WHvAdviseGpaRange(partition: WHV_PARTITION_HANDLE, gparanges: *const WHV_MEMORY_RANGE_ENTRY, gparangescount: u32, advice: WHV_ADVISE_GPA_RANGE_CODE, advicebuffer: *const ::core::ffi::c_void, advicebuffersizeinbytes: u32) -> ::windows_sys::core::HRESULT;
    #[cfg(feature = "Win32_Foundation")]
    pub fn WHvAllocateVpciResource(providerid: *const ::windows_sys::core::GUID, flags: WHV_ALLOCATE_VPCI_RESOURCE_FLAGS, resourcedescriptor: *const ::core::ffi::c_void, resourcedescriptorsizeinbytes: u32, vpciresource: *mut super::super::Foundation::HANDLE) -> ::windows_sys::core::HRESULT;
    pub fn WHvCancelPartitionMigration(partition: WHV_PARTITION_HANDLE) -> ::windows_sys::core::HRESULT;
    pub fn WHvCancelRunVirtualProcessor(partition: WHV_PARTITION_HANDLE, vpindex: u32, flags: u32) -> ::windows_sys::core::HRESULT;
    pub fn WHvCompletePartitionMigration(partition: WHV_PARTITION_HANDLE) -> ::windows_sys::core::HRESULT;
    #[cfg(feature = "Win32_Foundation")]
    pub fn WHvCreateNotificationPort(partition: WHV_PARTITION_HANDLE, parameters: *const WHV_NOTIFICATION_PORT_PARAMETERS, eventhandle: super::super::Foundation::HANDLE, porthandle: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT;
    pub fn WHvCreatePartition(partition: *mut WHV_PARTITION_HANDLE) -> ::windows_sys::core::HRESULT;
    #[cfg(feature = "Win32_Foundation")]
    pub fn WHvCreateTrigger(partition: WHV_PARTITION_HANDLE, parameters: *const WHV_TRIGGER_PARAMETERS, triggerhandle: *mut *mut ::core::ffi::c_void, eventhandle: *mut super::super::Foundation::HANDLE) -> ::windows_sys::core::HRESULT;
    pub fn WHvCreateVirtualProcessor(partition: WHV_PARTITION_HANDLE, vpindex: u32, flags: u32) -> ::windows_sys::core::HRESULT;
    pub fn WHvCreateVirtualProcessor2(partition: WHV_PARTITION_HANDLE, vpindex: u32, properties: *const WHV_VIRTUAL_PROCESSOR_PROPERTY, propertycount: u32) -> ::windows_sys::core::HRESULT;
    #[cfg(feature = "Win32_Foundation")]
    pub fn WHvCreateVpciDevice(partition: WHV_PARTITION_HANDLE, logicaldeviceid: u64, vpciresource: super::super::Foundation::HANDLE, flags: WHV_CREATE_VPCI_DEVICE_FLAGS, notificationeventhandle: super::super::Foundation::HANDLE) -> ::windows_sys::core::HRESULT;
    pub fn WHvDeleteNotificationPort(partition: WHV_PARTITION_HANDLE, porthandle: *const ::core::ffi::c_void) -> ::windows_sys::core::HRESULT;
    pub fn WHvDeletePartition(partition: WHV_PARTITION_HANDLE) -> ::windows_sys::core::HRESULT;
    pub fn WHvDeleteTrigger(partition: WHV_PARTITION_HANDLE, triggerhandle: *const ::core::ffi::c_void) -> ::windows_sys::core::HRESULT;
    pub fn WHvDeleteVirtualProcessor(partition: WHV_PARTITION_HANDLE, vpindex: u32) -> ::windows_sys::core::HRESULT;
    pub fn WHvDeleteVpciDevice(partition: WHV_PARTITION_HANDLE, logicaldeviceid: u64) -> ::windows_sys::core::HRESULT;
    pub fn WHvEmulatorCreateEmulator(callbacks: *const WHV_EMULATOR_CALLBACKS, emulator: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT;
    pub fn WHvEmulatorDestroyEmulator(emulator: *const ::core::ffi::c_void) -> ::windows_sys::core::HRESULT;
    pub fn WHvEmulatorTryIoEmulation(emulator: *const ::core::ffi::c_void, context: *const ::core::ffi::c_void, vpcontext: *const WHV_VP_EXIT_CONTEXT, ioinstructioncontext: *const WHV_X64_IO_PORT_ACCESS_CONTEXT, emulatorreturnstatus: *mut WHV_EMULATOR_STATUS) -> ::windows_sys::core::HRESULT;
    pub fn WHvEmulatorTryMmioEmulation(emulator: *const ::core::ffi::c_void, context: *const ::core::ffi::c_void, vpcontext: *const WHV_VP_EXIT_CONTEXT, mmioinstructioncontext: *const WHV_MEMORY_ACCESS_CONTEXT, emulatorreturnstatus: *mut WHV_EMULATOR_STATUS) -> ::windows_sys::core::HRESULT;
    pub fn WHvGetCapability(capabilitycode: WHV_CAPABILITY_CODE, capabilitybuffer: *mut ::core::ffi::c_void, capabilitybuffersizeinbytes: u32, writtensizeinbytes: *mut u32) -> ::windows_sys::core::HRESULT;
    pub fn WHvGetInterruptTargetVpSet(partition: WHV_PARTITION_HANDLE, destination: u64, destinationmode: WHV_INTERRUPT_DESTINATION_MODE, targetvps: *mut u32, vpcount: u32, targetvpcount: *mut u32) -> ::windows_sys::core::HRESULT;
    pub fn WHvGetPartitionCounters(partition: WHV_PARTITION_HANDLE, counterset: WHV_PARTITION_COUNTER_SET, buffer: *mut ::core::ffi::c_void, buffersizeinbytes: u32, byteswritten: *mut u32) -> ::windows_sys::core::HRESULT;
    pub fn WHvGetPartitionProperty(partition: WHV_PARTITION_HANDLE, propertycode: WHV_PARTITION_PROPERTY_CODE, propertybuffer: *mut ::core::ffi::c_void, propertybuffersizeinbytes: u32, writtensizeinbytes: *mut u32) -> ::windows_sys::core::HRESULT;
    pub fn WHvGetVirtualProcessorCounters(partition: WHV_PARTITION_HANDLE, vpindex: u32, counterset: WHV_PROCESSOR_COUNTER_SET, buffer: *mut ::core::ffi::c_void, buffersizeinbytes: u32, byteswritten: *mut u32) -> ::windows_sys::core::HRESULT;
    pub fn WHvGetVirtualProcessorCpuidOutput(partition: WHV_PARTITION_HANDLE, vpindex: u32, eax: u32, ecx: u32, cpuidoutput: *mut WHV_CPUID_OUTPUT) -> ::windows_sys::core::HRESULT;
    pub fn WHvGetVirtualProcessorInterruptControllerState(partition: WHV_PARTITION_HANDLE, vpindex: u32, state: *mut ::core::ffi::c_void, statesize: u32, writtensize: *mut u32) -> ::windows_sys::core::HRESULT;
    pub fn WHvGetVirtualProcessorInterruptControllerState2(partition: WHV_PARTITION_HANDLE, vpindex: u32, state: *mut ::core::ffi::c_void, statesize: u32, writtensize: *mut u32) -> ::windows_sys::core::HRESULT;
    pub fn WHvGetVirtualProcessorRegisters(partition: WHV_PARTITION_HANDLE, vpindex: u32, registernames: *const WHV_REGISTER_NAME, registercount: u32, registervalues: *mut WHV_REGISTER_VALUE) -> ::windows_sys::core::HRESULT;
    pub fn WHvGetVirtualProcessorState(partition: WHV_PARTITION_HANDLE, vpindex: u32, statetype: WHV_VIRTUAL_PROCESSOR_STATE_TYPE, buffer: *mut ::core::ffi::c_void, buffersizeinbytes: u32, byteswritten: *mut u32) -> ::windows_sys::core::HRESULT;
    pub fn WHvGetVirtualProcessorXsaveState(partition: WHV_PARTITION_HANDLE, vpindex: u32, buffer: *mut ::core::ffi::c_void, buffersizeinbytes: u32, byteswritten: *mut u32) -> ::windows_sys::core::HRESULT;
    pub fn WHvGetVpciDeviceInterruptTarget(partition: WHV_PARTITION_HANDLE, logicaldeviceid: u64, index: u32, multimessagenumber: u32, target: *mut WHV_VPCI_INTERRUPT_TARGET, targetsizeinbytes: u32, byteswritten: *mut u32) -> ::windows_sys::core::HRESULT;
    pub fn WHvGetVpciDeviceNotification(partition: WHV_PARTITION_HANDLE, logicaldeviceid: u64, notification: *mut WHV_VPCI_DEVICE_NOTIFICATION, notificationsizeinbytes: u32) -> ::windows_sys::core::HRESULT;
    pub fn WHvGetVpciDeviceProperty(partition: WHV_PARTITION_HANDLE, logicaldeviceid: u64, propertycode: WHV_VPCI_DEVICE_PROPERTY_CODE, propertybuffer: *mut ::core::ffi::c_void, propertybuffersizeinbytes: u32, writtensizeinbytes: *mut u32) -> ::windows_sys::core::HRESULT;
    pub fn WHvMapGpaRange(partition: WHV_PARTITION_HANDLE, sourceaddress: *const ::core::ffi::c_void, guestaddress: u64, sizeinbytes: u64, flags: WHV_MAP_GPA_RANGE_FLAGS) -> ::windows_sys::core::HRESULT;
    #[cfg(feature = "Win32_Foundation")]
    pub fn WHvMapGpaRange2(partition: WHV_PARTITION_HANDLE, process: super::super::Foundation::HANDLE, sourceaddress: *const ::core::ffi::c_void, guestaddress: u64, sizeinbytes: u64, flags: WHV_MAP_GPA_RANGE_FLAGS) -> ::windows_sys::core::HRESULT;
    pub fn WHvMapVpciDeviceInterrupt(partition: WHV_PARTITION_HANDLE, logicaldeviceid: u64, index: u32, messagecount: u32, target: *const WHV_VPCI_INTERRUPT_TARGET, msiaddress: *mut u64, msidata: *mut u32) -> ::windows_sys::core::HRESULT;
    pub fn WHvMapVpciDeviceMmioRanges(partition: WHV_PARTITION_HANDLE, logicaldeviceid: u64, mappingcount: *mut u32, mappings: *mut *mut WHV_VPCI_MMIO_MAPPING) -> ::windows_sys::core::HRESULT;
    pub fn WHvPostVirtualProcessorSynicMessage(partition: WHV_PARTITION_HANDLE, vpindex: u32, sintindex: u32, message: *const ::core::ffi::c_void, messagesizeinbytes: u32) -> ::windows_sys::core::HRESULT;
    pub fn WHvQueryGpaRangeDirtyBitmap(partition: WHV_PARTITION_HANDLE, guestaddress: u64, rangesizeinbytes: u64, bitmap: *mut u64, bitmapsizeinbytes: u32) -> ::windows_sys::core::HRESULT;
    pub fn WHvReadGpaRange(partition: WHV_PARTITION_HANDLE, vpindex: u32, guestaddress: u64, controls: WHV_ACCESS_GPA_CONTROLS, data: *mut ::core::ffi::c_void, datasizeinbytes: u32) -> ::windows_sys::core::HRESULT;
    pub fn WHvReadVpciDeviceRegister(partition: WHV_PARTITION_HANDLE, logicaldeviceid: u64, register: *const WHV_VPCI_DEVICE_REGISTER, data: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT;
    #[cfg(feature = "Win32_Foundation")]
    pub fn WHvRegisterPartitionDoorbellEvent(partition: WHV_PARTITION_HANDLE, matchdata: *const WHV_DOORBELL_MATCH_DATA, eventhandle: super::super::Foundation::HANDLE) -> ::windows_sys::core::HRESULT;
    pub fn WHvRequestInterrupt(partition: WHV_PARTITION_HANDLE, interrupt: *const WHV_INTERRUPT_CONTROL, interruptcontrolsize: u32) -> ::windows_sys::core::HRESULT;
    pub fn WHvRequestVpciDeviceInterrupt(partition: WHV_PARTITION_HANDLE, logicaldeviceid: u64, msiaddress: u64, msidata: u32) -> ::windows_sys::core::HRESULT;
    pub fn WHvResetPartition(partition: WHV_PARTITION_HANDLE) -> ::windows_sys::core::HRESULT;
    pub fn WHvResumePartitionTime(partition: WHV_PARTITION_HANDLE) -> ::windows_sys::core::HRESULT;
    pub fn WHvRetargetVpciDeviceInterrupt(partition: WHV_PARTITION_HANDLE, logicaldeviceid: u64, msiaddress: u64, msidata: u32, target: *const WHV_VPCI_INTERRUPT_TARGET) -> ::windows_sys::core::HRESULT;
    pub fn WHvRunVirtualProcessor(partition: WHV_PARTITION_HANDLE, vpindex: u32, exitcontext: *mut ::core::ffi::c_void, exitcontextsizeinbytes: u32) -> ::windows_sys::core::HRESULT;
    pub fn WHvSetNotificationPortProperty(partition: WHV_PARTITION_HANDLE, porthandle: *const ::core::ffi::c_void, propertycode: WHV_NOTIFICATION_PORT_PROPERTY_CODE, propertyvalue: u64) -> ::windows_sys::core::HRESULT;
    pub fn WHvSetPartitionProperty(partition: WHV_PARTITION_HANDLE, propertycode: WHV_PARTITION_PROPERTY_CODE, propertybuffer: *const ::core::ffi::c_void, propertybuffersizeinbytes: u32) -> ::windows_sys::core::HRESULT;
    pub fn WHvSetVirtualProcessorInterruptControllerState(partition: WHV_PARTITION_HANDLE, vpindex: u32, state: *const ::core::ffi::c_void, statesize: u32) -> ::windows_sys::core::HRESULT;
    pub fn WHvSetVirtualProcessorInterruptControllerState2(partition: WHV_PARTITION_HANDLE, vpindex: u32, state: *const ::core::ffi::c_void, statesize: u32) -> ::windows_sys::core::HRESULT;
    pub fn WHvSetVirtualProcessorRegisters(partition: WHV_PARTITION_HANDLE, vpindex: u32, registernames: *const WHV_REGISTER_NAME, registercount: u32, registervalues: *const WHV_REGISTER_VALUE) -> ::windows_sys::core::HRESULT;
    pub fn WHvSetVirtualProcessorState(partition: WHV_PARTITION_HANDLE, vpindex: u32, statetype: WHV_VIRTUAL_PROCESSOR_STATE_TYPE, buffer: *const ::core::ffi::c_void, buffersizeinbytes: u32) -> ::windows_sys::core::HRESULT;
    pub fn WHvSetVirtualProcessorXsaveState(partition: WHV_PARTITION_HANDLE, vpindex: u32, buffer: *const ::core::ffi::c_void, buffersizeinbytes: u32) -> ::windows_sys::core::HRESULT;
    #[cfg(feature = "Win32_System_Power")]
    pub fn WHvSetVpciDevicePowerState(partition: WHV_PARTITION_HANDLE, logicaldeviceid: u64, powerstate: super::Power::DEVICE_POWER_STATE) -> ::windows_sys::core::HRESULT;
    pub fn WHvSetupPartition(partition: WHV_PARTITION_HANDLE) -> ::windows_sys::core::HRESULT;
    #[cfg(feature = "Win32_Foundation")]
    pub fn WHvSignalVirtualProcessorSynicEvent(partition: WHV_PARTITION_HANDLE, synicevent: WHV_SYNIC_EVENT_PARAMETERS, newlysignaled: *mut super::super::Foundation::BOOL) -> ::windows_sys::core::HRESULT;
    #[cfg(feature = "Win32_Foundation")]
    pub fn WHvStartPartitionMigration(partition: WHV_PARTITION_HANDLE, migrationhandle: *mut super::super::Foundation::HANDLE) -> ::windows_sys::core::HRESULT;
    pub fn WHvSuspendPartitionTime(partition: WHV_PARTITION_HANDLE) -> ::windows_sys::core::HRESULT;
    pub fn WHvTranslateGva(partition: WHV_PARTITION_HANDLE, vpindex: u32, gva: u64, translateflags: WHV_TRANSLATE_GVA_FLAGS, translationresult: *mut WHV_TRANSLATE_GVA_RESULT, gpa: *mut u64) -> ::windows_sys::core::HRESULT;
    pub fn WHvUnmapGpaRange(partition: WHV_PARTITION_HANDLE, guestaddress: u64, sizeinbytes: u64) -> ::windows_sys::core::HRESULT;
    pub fn WHvUnmapVpciDeviceInterrupt(partition: WHV_PARTITION_HANDLE, logicaldeviceid: u64, index: u32) -> ::windows_sys::core::HRESULT;
    pub fn WHvUnmapVpciDeviceMmioRanges(partition: WHV_PARTITION_HANDLE, logicaldeviceid: u64) -> ::windows_sys::core::HRESULT;
    pub fn WHvUnregisterPartitionDoorbellEvent(partition: WHV_PARTITION_HANDLE, matchdata: *const WHV_DOORBELL_MATCH_DATA) -> ::windows_sys::core::HRESULT;
    pub fn WHvUpdateTriggerParameters(partition: WHV_PARTITION_HANDLE, parameters: *const WHV_TRIGGER_PARAMETERS, triggerhandle: *const ::core::ffi::c_void) -> ::windows_sys::core::HRESULT;
    pub fn WHvWriteGpaRange(partition: WHV_PARTITION_HANDLE, vpindex: u32, guestaddress: u64, controls: WHV_ACCESS_GPA_CONTROLS, data: *const ::core::ffi::c_void, datasizeinbytes: u32) -> ::windows_sys::core::HRESULT;
    pub fn WHvWriteVpciDeviceRegister(partition: WHV_PARTITION_HANDLE, logicaldeviceid: u64, register: *const WHV_VPCI_DEVICE_REGISTER, data: *const ::core::ffi::c_void) -> ::windows_sys::core::HRESULT;
}
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct DOS_IMAGE_INFO {
    pub PdbName: super::super::Foundation::PSTR,
    pub ImageBaseAddress: u64,
    pub ImageSize: u32,
    pub Timestamp: u32,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for DOS_IMAGE_INFO {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for DOS_IMAGE_INFO {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
pub type FOUND_IMAGE_CALLBACK = unsafe extern "system" fn(context: *const ::core::ffi::c_void, imageinfo: *const DOS_IMAGE_INFO) -> super::super::Foundation::BOOL;
#[repr(C)]
pub struct GPA_MEMORY_CHUNK {
    pub GuestPhysicalStartPageIndex: u64,
    pub PageCount: u64,
}
impl ::core::marker::Copy for GPA_MEMORY_CHUNK {}
impl ::core::clone::Clone for GPA_MEMORY_CHUNK {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub union GUEST_OS_INFO {
    pub AsUINT64: u64,
    pub ClosedSource: GUEST_OS_INFO_0,
    pub OpenSource: GUEST_OS_INFO_1,
}
impl ::core::marker::Copy for GUEST_OS_INFO {}
impl ::core::clone::Clone for GUEST_OS_INFO {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct GUEST_OS_INFO_0 {
    pub _bitfield: u64,
}
impl ::core::marker::Copy for GUEST_OS_INFO_0 {}
impl ::core::clone::Clone for GUEST_OS_INFO_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct GUEST_OS_INFO_1 {
    pub _bitfield: u64,
}
impl ::core::marker::Copy for GUEST_OS_INFO_1 {}
impl ::core::clone::Clone for GUEST_OS_INFO_1 {
    fn clone(&self) -> Self {
        *self
    }
}
pub const GuestOsMicrosoftUndefined: i32 = 0i32;
pub const GuestOsMicrosoftMSDOS: i32 = 1i32;
pub const GuestOsMicrosoftWindows3x: i32 = 2i32;
pub const GuestOsMicrosoftWindows9x: i32 = 3i32;
pub const GuestOsMicrosoftWindowsNT: i32 = 4i32;
pub const GuestOsMicrosoftWindowsCE: i32 = 5i32;
pub const GuestOsOpenSourceUndefined: i32 = 0i32;
pub const GuestOsOpenSourceLinux: i32 = 1i32;
pub const GuestOsOpenSourceFreeBSD: i32 = 2i32;
pub const GuestOsOpenSourceXen: i32 = 3i32;
pub const GuestOsOpenSourceIllumos: i32 = 4i32;
pub const GuestOsVendorUndefined: i32 = 0i32;
pub const GuestOsVendorMicrosoft: i32 = 1i32;
pub const GuestOsVendorHPE: i32 = 2i32;
pub const GuestOsVendorLANCOM: i32 = 512i32;
#[cfg(feature = "Win32_Foundation")]
pub type GUEST_SYMBOLS_PROVIDER_DEBUG_INFO_CALLBACK = unsafe extern "system" fn(infomessage: super::super::Foundation::PSTR);
pub const GUID_DEVINTERFACE_VM_GENCOUNTER: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1072875819, data2: 26008, data3: 20064, data4: [142, 28, 12, 207, 73, 39, 227, 25] };
pub const HdvDeviceTypeUndefined: i32 = 0i32;
pub const HdvDeviceTypePCI: i32 = 1i32;
pub const HDV_DOORBELL_FLAG_TRIGGER_SIZE_ANY: i32 = 0i32;
pub const HDV_DOORBELL_FLAG_TRIGGER_SIZE_BYTE: i32 = 1i32;
pub const HDV_DOORBELL_FLAG_TRIGGER_SIZE_WORD: i32 = 2i32;
pub const HDV_DOORBELL_FLAG_TRIGGER_SIZE_DWORD: i32 = 3i32;
pub const HDV_DOORBELL_FLAG_TRIGGER_SIZE_QWORD: i32 = 4i32;
pub const HDV_DOORBELL_FLAG_TRIGGER_ANY_VALUE: i32 = -2147483648i32;
pub const HdvMmioMappingFlagNone: u32 = 0u32;
pub const HdvMmioMappingFlagWriteable: u32 = 1u32;
pub const HdvMmioMappingFlagExecutable: u32 = 2u32;
pub const HDV_PCI_BAR_COUNT: u32 = 6u32;
pub const HDV_PCI_BAR0: i32 = 0i32;
pub const HDV_PCI_BAR1: i32 = 1i32;
pub const HDV_PCI_BAR2: i32 = 2i32;
pub const HDV_PCI_BAR3: i32 = 3i32;
pub const HDV_PCI_BAR4: i32 = 4i32;
pub const HDV_PCI_BAR5: i32 = 5i32;
pub type HDV_PCI_DEVICE_GET_DETAILS = unsafe extern "system" fn(devicecontext: *const ::core::ffi::c_void, pnpid: *mut HDV_PCI_PNP_ID, probedbarscount: u32, probedbars: *mut u32) -> ::windows_sys::core::HRESULT;
pub type HDV_PCI_DEVICE_INITIALIZE = unsafe extern "system" fn(devicecontext: *const ::core::ffi::c_void) -> ::windows_sys::core::HRESULT;
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct HDV_PCI_DEVICE_INTERFACE {
    pub Version: HDV_PCI_INTERFACE_VERSION,
    pub Initialize: HDV_PCI_DEVICE_INITIALIZE,
    pub Teardown: HDV_PCI_DEVICE_TEARDOWN,
    pub SetConfiguration: HDV_PCI_DEVICE_SET_CONFIGURATION,
    pub GetDetails: HDV_PCI_DEVICE_GET_DETAILS,
    pub Start: HDV_PCI_DEVICE_START,
    pub Stop: HDV_PCI_DEVICE_STOP,
    pub ReadConfigSpace: HDV_PCI_READ_CONFIG_SPACE,
    pub WriteConfigSpace: HDV_PCI_WRITE_CONFIG_SPACE,
    pub ReadInterceptedMemory: HDV_PCI_READ_INTERCEPTED_MEMORY,
    pub WriteInterceptedMemory: HDV_PCI_WRITE_INTERCEPTED_MEMORY,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for HDV_PCI_DEVICE_INTERFACE {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for HDV_PCI_DEVICE_INTERFACE {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
pub type HDV_PCI_DEVICE_SET_CONFIGURATION = unsafe extern "system" fn(devicecontext: *const ::core::ffi::c_void, configurationvaluecount: u32, configurationvalues: *const super::super::Foundation::PWSTR) -> ::windows_sys::core::HRESULT;
pub type HDV_PCI_DEVICE_START = unsafe extern "system" fn(devicecontext: *const ::core::ffi::c_void) -> ::windows_sys::core::HRESULT;
pub type HDV_PCI_DEVICE_STOP = unsafe extern "system" fn(devicecontext: *const ::core::ffi::c_void);
pub type HDV_PCI_DEVICE_TEARDOWN = unsafe extern "system" fn(devicecontext: *const ::core::ffi::c_void);
pub const HdvPciDeviceInterfaceVersionInvalid: i32 = 0i32;
pub const HdvPciDeviceInterfaceVersion1: i32 = 1i32;
#[repr(C)]
pub struct HDV_PCI_PNP_ID {
    pub VendorID: u16,
    pub DeviceID: u16,
    pub RevisionID: u8,
    pub ProgIf: u8,
    pub SubClass: u8,
    pub BaseClass: u8,
    pub SubVendorID: u16,
    pub SubSystemID: u16,
}
impl ::core::marker::Copy for HDV_PCI_PNP_ID {}
impl ::core::clone::Clone for HDV_PCI_PNP_ID {
    fn clone(&self) -> Self {
        *self
    }
}
pub type HDV_PCI_READ_CONFIG_SPACE = unsafe extern "system" fn(devicecontext: *const ::core::ffi::c_void, offset: u32, value: *mut u32) -> ::windows_sys::core::HRESULT;
pub type HDV_PCI_READ_INTERCEPTED_MEMORY = unsafe extern "system" fn(devicecontext: *const ::core::ffi::c_void, barindex: HDV_PCI_BAR_SELECTOR, offset: u64, length: u64, value: *mut u8) -> ::windows_sys::core::HRESULT;
pub type HDV_PCI_WRITE_CONFIG_SPACE = unsafe extern "system" fn(devicecontext: *const ::core::ffi::c_void, offset: u32, value: u32) -> ::windows_sys::core::HRESULT;
pub type HDV_PCI_WRITE_INTERCEPTED_MEMORY = unsafe extern "system" fn(devicecontext: *const ::core::ffi::c_void, barindex: HDV_PCI_BAR_SELECTOR, offset: u64, length: u64, value: *const u8) -> ::windows_sys::core::HRESULT;
pub const HVSOCKET_ADDRESS_FLAG_PASSTHRU: u32 = 1u32;
#[repr(C)]
pub struct HVSOCKET_ADDRESS_INFO {
    pub SystemId: ::windows_sys::core::GUID,
    pub VirtualMachineId: ::windows_sys::core::GUID,
    pub SiloId: ::windows_sys::core::GUID,
    pub Flags: u32,
}
impl ::core::marker::Copy for HVSOCKET_ADDRESS_INFO {}
impl ::core::clone::Clone for HVSOCKET_ADDRESS_INFO {
    fn clone(&self) -> Self {
        *self
    }
}
pub const HVSOCKET_CONNECTED_SUSPEND: u32 = 4u32;
pub const HVSOCKET_CONNECT_TIMEOUT: u32 = 1u32;
pub const HVSOCKET_CONNECT_TIMEOUT_MAX: u32 = 300000u32;
pub const HVSOCKET_CONTAINER_PASSTHRU: u32 = 2u32;
pub const HV_GUID_BROADCAST: ::windows_sys::core::GUID = ::windows_sys::core::GUID {
    data1: 4294967295,
    data2: 65535,
    data3: 65535,
    data4: [255, 255, 255, 255, 255, 255, 255, 255],
};
pub const HV_GUID_CHILDREN: ::windows_sys::core::GUID = ::windows_sys::core::GUID {
    data1: 2430307209,
    data2: 3381,
    data3: 20345,
    data4: [140, 233, 73, 234, 10, 200, 183, 205],
};
pub const HV_GUID_LOOPBACK: ::windows_sys::core::GUID = ::windows_sys::core::GUID {
    data1: 3772866967,
    data2: 56662,
    data3: 18960,
    data4: [145, 149, 94, 231, 161, 85, 168, 56],
};
pub const HV_GUID_PARENT: ::windows_sys::core::GUID = ::windows_sys::core::GUID {
    data1: 2754510042,
    data2: 53311,
    data3: 18444,
    data4: [156, 194, 164, 222, 32, 171, 184, 120],
};
pub const HV_GUID_SILOHOST: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 918359132, data2: 29302, data3: 16931, data4: [136, 186, 125, 3, 182, 84, 197, 104] };
pub const HV_GUID_VSOCK_TEMPLATE: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 0, data2: 64203, data3: 4582, data4: [189, 88, 100, 0, 106, 121, 134, 211] };
pub const HV_GUID_ZERO: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 0, data2: 0, data3: 0, data4: [0, 0, 0, 0, 0, 0, 0, 0] };
pub const HV_PROTOCOL_RAW: u32 = 1u32;
pub const IOCTL_VMGENCOUNTER_READ: u32 = 3325956u32;
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct MODULE_INFO {
    pub ProcessImageName: super::super::Foundation::PSTR,
    pub Image: DOS_IMAGE_INFO,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for MODULE_INFO {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for MODULE_INFO {
    fn clone(&self) -> Self {
        *self
    }
}
pub const Paging_Invalid: i32 = 0i32;
pub const Paging_NonPaged: i32 = 1i32;
pub const Paging_32Bit: i32 = 2i32;
pub const Paging_Pae: i32 = 3i32;
pub const Paging_Long: i32 = 4i32;
pub const Paging_Armv8: i32 = 5i32;
pub const X64_RegisterRax: i32 = 0i32;
pub const X64_RegisterRcx: i32 = 1i32;
pub const X64_RegisterRdx: i32 = 2i32;
pub const X64_RegisterRbx: i32 = 3i32;
pub const X64_RegisterRsp: i32 = 4i32;
pub const X64_RegisterRbp: i32 = 5i32;
pub const X64_RegisterRsi: i32 = 6i32;
pub const X64_RegisterRdi: i32 = 7i32;
pub const X64_RegisterR8: i32 = 8i32;
pub const X64_RegisterR9: i32 = 9i32;
pub const X64_RegisterR10: i32 = 10i32;
pub const X64_RegisterR11: i32 = 11i32;
pub const X64_RegisterR12: i32 = 12i32;
pub const X64_RegisterR13: i32 = 13i32;
pub const X64_RegisterR14: i32 = 14i32;
pub const X64_RegisterR15: i32 = 15i32;
pub const X64_RegisterRip: i32 = 16i32;
pub const X64_RegisterRFlags: i32 = 17i32;
pub const X64_RegisterXmm0: i32 = 18i32;
pub const X64_RegisterXmm1: i32 = 19i32;
pub const X64_RegisterXmm2: i32 = 20i32;
pub const X64_RegisterXmm3: i32 = 21i32;
pub const X64_RegisterXmm4: i32 = 22i32;
pub const X64_RegisterXmm5: i32 = 23i32;
pub const X64_RegisterXmm6: i32 = 24i32;
pub const X64_RegisterXmm7: i32 = 25i32;
pub const X64_RegisterXmm8: i32 = 26i32;
pub const X64_RegisterXmm9: i32 = 27i32;
pub const X64_RegisterXmm10: i32 = 28i32;
pub const X64_RegisterXmm11: i32 = 29i32;
pub const X64_RegisterXmm12: i32 = 30i32;
pub const X64_RegisterXmm13: i32 = 31i32;
pub const X64_RegisterXmm14: i32 = 32i32;
pub const X64_RegisterXmm15: i32 = 33i32;
pub const X64_RegisterFpMmx0: i32 = 34i32;
pub const X64_RegisterFpMmx1: i32 = 35i32;
pub const X64_RegisterFpMmx2: i32 = 36i32;
pub const X64_RegisterFpMmx3: i32 = 37i32;
pub const X64_RegisterFpMmx4: i32 = 38i32;
pub const X64_RegisterFpMmx5: i32 = 39i32;
pub const X64_RegisterFpMmx6: i32 = 40i32;
pub const X64_RegisterFpMmx7: i32 = 41i32;
pub const X64_RegisterFpControlStatus: i32 = 42i32;
pub const X64_RegisterXmmControlStatus: i32 = 43i32;
pub const X64_RegisterCr0: i32 = 44i32;
pub const X64_RegisterCr2: i32 = 45i32;
pub const X64_RegisterCr3: i32 = 46i32;
pub const X64_RegisterCr4: i32 = 47i32;
pub const X64_RegisterCr8: i32 = 48i32;
pub const X64_RegisterEfer: i32 = 49i32;
pub const X64_RegisterDr0: i32 = 50i32;
pub const X64_RegisterDr1: i32 = 51i32;
pub const X64_RegisterDr2: i32 = 52i32;
pub const X64_RegisterDr3: i32 = 53i32;
pub const X64_RegisterDr6: i32 = 54i32;
pub const X64_RegisterDr7: i32 = 55i32;
pub const X64_RegisterEs: i32 = 56i32;
pub const X64_RegisterCs: i32 = 57i32;
pub const X64_RegisterSs: i32 = 58i32;
pub const X64_RegisterDs: i32 = 59i32;
pub const X64_RegisterFs: i32 = 60i32;
pub const X64_RegisterGs: i32 = 61i32;
pub const X64_RegisterLdtr: i32 = 62i32;
pub const X64_RegisterTr: i32 = 63i32;
pub const X64_RegisterIdtr: i32 = 64i32;
pub const X64_RegisterGdtr: i32 = 65i32;
pub const X64_RegisterMax: i32 = 66i32;
pub const ARM64_RegisterX0: i32 = 67i32;
pub const ARM64_RegisterX1: i32 = 68i32;
pub const ARM64_RegisterX2: i32 = 69i32;
pub const ARM64_RegisterX3: i32 = 70i32;
pub const ARM64_RegisterX4: i32 = 71i32;
pub const ARM64_RegisterX5: i32 = 72i32;
pub const ARM64_RegisterX6: i32 = 73i32;
pub const ARM64_RegisterX7: i32 = 74i32;
pub const ARM64_RegisterX8: i32 = 75i32;
pub const ARM64_RegisterX9: i32 = 76i32;
pub const ARM64_RegisterX10: i32 = 77i32;
pub const ARM64_RegisterX11: i32 = 78i32;
pub const ARM64_RegisterX12: i32 = 79i32;
pub const ARM64_RegisterX13: i32 = 80i32;
pub const ARM64_RegisterX14: i32 = 81i32;
pub const ARM64_RegisterX15: i32 = 82i32;
pub const ARM64_RegisterX16: i32 = 83i32;
pub const ARM64_RegisterX17: i32 = 84i32;
pub const ARM64_RegisterX18: i32 = 85i32;
pub const ARM64_RegisterX19: i32 = 86i32;
pub const ARM64_RegisterX20: i32 = 87i32;
pub const ARM64_RegisterX21: i32 = 88i32;
pub const ARM64_RegisterX22: i32 = 89i32;
pub const ARM64_RegisterX23: i32 = 90i32;
pub const ARM64_RegisterX24: i32 = 91i32;
pub const ARM64_RegisterX25: i32 = 92i32;
pub const ARM64_RegisterX26: i32 = 93i32;
pub const ARM64_RegisterX27: i32 = 94i32;
pub const ARM64_RegisterX28: i32 = 95i32;
pub const ARM64_RegisterXFp: i32 = 96i32;
pub const ARM64_RegisterXLr: i32 = 97i32;
pub const ARM64_RegisterPc: i32 = 98i32;
pub const ARM64_RegisterSpEl0: i32 = 99i32;
pub const ARM64_RegisterSpEl1: i32 = 100i32;
pub const ARM64_RegisterCpsr: i32 = 101i32;
pub const ARM64_RegisterQ0: i32 = 102i32;
pub const ARM64_RegisterQ1: i32 = 103i32;
pub const ARM64_RegisterQ2: i32 = 104i32;
pub const ARM64_RegisterQ3: i32 = 105i32;
pub const ARM64_RegisterQ4: i32 = 106i32;
pub const ARM64_RegisterQ5: i32 = 107i32;
pub const ARM64_RegisterQ6: i32 = 108i32;
pub const ARM64_RegisterQ7: i32 = 109i32;
pub const ARM64_RegisterQ8: i32 = 110i32;
pub const ARM64_RegisterQ9: i32 = 111i32;
pub const ARM64_RegisterQ10: i32 = 112i32;
pub const ARM64_RegisterQ11: i32 = 113i32;
pub const ARM64_RegisterQ12: i32 = 114i32;
pub const ARM64_RegisterQ13: i32 = 115i32;
pub const ARM64_RegisterQ14: i32 = 116i32;
pub const ARM64_RegisterQ15: i32 = 117i32;
pub const ARM64_RegisterQ16: i32 = 118i32;
pub const ARM64_RegisterQ17: i32 = 119i32;
pub const ARM64_RegisterQ18: i32 = 120i32;
pub const ARM64_RegisterQ19: i32 = 121i32;
pub const ARM64_RegisterQ20: i32 = 122i32;
pub const ARM64_RegisterQ21: i32 = 123i32;
pub const ARM64_RegisterQ22: i32 = 124i32;
pub const ARM64_RegisterQ23: i32 = 125i32;
pub const ARM64_RegisterQ24: i32 = 126i32;
pub const ARM64_RegisterQ25: i32 = 127i32;
pub const ARM64_RegisterQ26: i32 = 128i32;
pub const ARM64_RegisterQ27: i32 = 129i32;
pub const ARM64_RegisterQ28: i32 = 130i32;
pub const ARM64_RegisterQ29: i32 = 131i32;
pub const ARM64_RegisterQ30: i32 = 132i32;
pub const ARM64_RegisterQ31: i32 = 133i32;
pub const ARM64_RegisterFpStatus: i32 = 134i32;
pub const ARM64_RegisterFpControl: i32 = 135i32;
pub const ARM64_RegisterEsrEl1: i32 = 136i32;
pub const ARM64_RegisterSpsrEl1: i32 = 137i32;
pub const ARM64_RegisterFarEl1: i32 = 138i32;
pub const ARM64_RegisterParEl1: i32 = 139i32;
pub const ARM64_RegisterElrEl1: i32 = 140i32;
pub const ARM64_RegisterTtbr0El1: i32 = 141i32;
pub const ARM64_RegisterTtbr1El1: i32 = 142i32;
pub const ARM64_RegisterVbarEl1: i32 = 143i32;
pub const ARM64_RegisterSctlrEl1: i32 = 144i32;
pub const ARM64_RegisterActlrEl1: i32 = 145i32;
pub const ARM64_RegisterTcrEl1: i32 = 146i32;
pub const ARM64_RegisterMairEl1: i32 = 147i32;
pub const ARM64_RegisterAmairEl1: i32 = 148i32;
pub const ARM64_RegisterTpidrEl0: i32 = 149i32;
pub const ARM64_RegisterTpidrroEl0: i32 = 150i32;
pub const ARM64_RegisterTpidrEl1: i32 = 151i32;
pub const ARM64_RegisterContextIdrEl1: i32 = 152i32;
pub const ARM64_RegisterCpacrEl1: i32 = 153i32;
pub const ARM64_RegisterCsselrEl1: i32 = 154i32;
pub const ARM64_RegisterCntkctlEl1: i32 = 155i32;
pub const ARM64_RegisterCntvCvalEl0: i32 = 156i32;
pub const ARM64_RegisterCntvCtlEl0: i32 = 157i32;
pub const ARM64_RegisterMax: i32 = 158i32;
#[repr(C)]
pub struct SOCKADDR_HV {
    pub Family: u16,
    pub Reserved: u16,
    pub VmId: ::windows_sys::core::GUID,
    pub ServiceId: ::windows_sys::core::GUID,
}
impl ::core::marker::Copy for SOCKADDR_HV {}
impl ::core::clone::Clone for SOCKADDR_HV {
    fn clone(&self) -> Self {
        *self
    }
}
pub const Arch_Unknown: i32 = 0i32;
pub const Arch_x86: i32 = 1i32;
pub const Arch_x64: i32 = 2i32;
pub const Arch_Armv8: i32 = 3i32;
#[repr(C)]
pub union VIRTUAL_PROCESSOR_REGISTER {
    pub Reg64: u64,
    pub Reg32: u32,
    pub Reg16: u16,
    pub Reg8: u8,
    pub Reg128: VIRTUAL_PROCESSOR_REGISTER_0,
    pub X64: VIRTUAL_PROCESSOR_REGISTER_1,
}
impl ::core::marker::Copy for VIRTUAL_PROCESSOR_REGISTER {}
impl ::core::clone::Clone for VIRTUAL_PROCESSOR_REGISTER {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct VIRTUAL_PROCESSOR_REGISTER_0 {
    pub Low64: u64,
    pub High64: u64,
}
impl ::core::marker::Copy for VIRTUAL_PROCESSOR_REGISTER_0 {}
impl ::core::clone::Clone for VIRTUAL_PROCESSOR_REGISTER_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub union VIRTUAL_PROCESSOR_REGISTER_1 {
    pub Segment: VIRTUAL_PROCESSOR_REGISTER_1_1,
    pub Table: VIRTUAL_PROCESSOR_REGISTER_1_2,
    pub FpControlStatus: VIRTUAL_PROCESSOR_REGISTER_1_0,
    pub XmmControlStatus: VIRTUAL_PROCESSOR_REGISTER_1_3,
}
impl ::core::marker::Copy for VIRTUAL_PROCESSOR_REGISTER_1 {}
impl ::core::clone::Clone for VIRTUAL_PROCESSOR_REGISTER_1 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct VIRTUAL_PROCESSOR_REGISTER_1_0 {
    pub FpControl: u16,
    pub FpStatus: u16,
    pub FpTag: u8,
    pub Reserved: u8,
    pub LastFpOp: u16,
    pub Anonymous: VIRTUAL_PROCESSOR_REGISTER_1_0_0,
}
impl ::core::marker::Copy for VIRTUAL_PROCESSOR_REGISTER_1_0 {}
impl ::core::clone::Clone for VIRTUAL_PROCESSOR_REGISTER_1_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub union VIRTUAL_PROCESSOR_REGISTER_1_0_0 {
    pub LastFpRip: u64,
    pub Anonymous: VIRTUAL_PROCESSOR_REGISTER_1_0_0_0,
}
impl ::core::marker::Copy for VIRTUAL_PROCESSOR_REGISTER_1_0_0 {}
impl ::core::clone::Clone for VIRTUAL_PROCESSOR_REGISTER_1_0_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct VIRTUAL_PROCESSOR_REGISTER_1_0_0_0 {
    pub LastFpEip: u32,
    pub LastFpCs: u16,
}
impl ::core::marker::Copy for VIRTUAL_PROCESSOR_REGISTER_1_0_0_0 {}
impl ::core::clone::Clone for VIRTUAL_PROCESSOR_REGISTER_1_0_0_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct VIRTUAL_PROCESSOR_REGISTER_1_1 {
    pub Base: u64,
    pub Limit: u32,
    pub Selector: u16,
    pub Anonymous: VIRTUAL_PROCESSOR_REGISTER_1_1_0,
}
impl ::core::marker::Copy for VIRTUAL_PROCESSOR_REGISTER_1_1 {}
impl ::core::clone::Clone for VIRTUAL_PROCESSOR_REGISTER_1_1 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub union VIRTUAL_PROCESSOR_REGISTER_1_1_0 {
    pub Attributes: u16,
    pub Anonymous: VIRTUAL_PROCESSOR_REGISTER_1_1_0_0,
}
impl ::core::marker::Copy for VIRTUAL_PROCESSOR_REGISTER_1_1_0 {}
impl ::core::clone::Clone for VIRTUAL_PROCESSOR_REGISTER_1_1_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct VIRTUAL_PROCESSOR_REGISTER_1_1_0_0 {
    pub _bitfield: u16,
}
impl ::core::marker::Copy for VIRTUAL_PROCESSOR_REGISTER_1_1_0_0 {}
impl ::core::clone::Clone for VIRTUAL_PROCESSOR_REGISTER_1_1_0_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct VIRTUAL_PROCESSOR_REGISTER_1_2 {
    pub Limit: u16,
    pub Base: u64,
}
impl ::core::marker::Copy for VIRTUAL_PROCESSOR_REGISTER_1_2 {}
impl ::core::clone::Clone for VIRTUAL_PROCESSOR_REGISTER_1_2 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct VIRTUAL_PROCESSOR_REGISTER_1_3 {
    pub Anonymous: VIRTUAL_PROCESSOR_REGISTER_1_3_0,
    pub XmmStatusControl: u32,
    pub XmmStatusControlMask: u32,
}
impl ::core::marker::Copy for VIRTUAL_PROCESSOR_REGISTER_1_3 {}
impl ::core::clone::Clone for VIRTUAL_PROCESSOR_REGISTER_1_3 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub union VIRTUAL_PROCESSOR_REGISTER_1_3_0 {
    pub LastFpRdp: u64,
    pub Anonymous: VIRTUAL_PROCESSOR_REGISTER_1_3_0_0,
}
impl ::core::marker::Copy for VIRTUAL_PROCESSOR_REGISTER_1_3_0 {}
impl ::core::clone::Clone for VIRTUAL_PROCESSOR_REGISTER_1_3_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct VIRTUAL_PROCESSOR_REGISTER_1_3_0_0 {
    pub LastFpDp: u32,
    pub LastFpDs: u16,
}
impl ::core::marker::Copy for VIRTUAL_PROCESSOR_REGISTER_1_3_0_0 {}
impl ::core::clone::Clone for VIRTUAL_PROCESSOR_REGISTER_1_3_0_0 {
    fn clone(&self) -> Self {
        *self
    }
}
pub const ProcessorVendor_Unknown: i32 = 0i32;
pub const ProcessorVendor_Amd: i32 = 1i32;
pub const ProcessorVendor_Intel: i32 = 2i32;
pub const ProcessorVendor_Hygon: i32 = 3i32;
pub const ProcessorVendor_Arm: i32 = 4i32;
#[repr(C)]
pub struct VM_GENCOUNTER {
    pub GenerationCount: u64,
    pub GenerationCountHigh: u64,
}
impl ::core::marker::Copy for VM_GENCOUNTER {}
impl ::core::clone::Clone for VM_GENCOUNTER {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub union WHV_ACCESS_GPA_CONTROLS {
    pub AsUINT64: u64,
    pub Anonymous: WHV_ACCESS_GPA_CONTROLS_0,
}
impl ::core::marker::Copy for WHV_ACCESS_GPA_CONTROLS {}
impl ::core::clone::Clone for WHV_ACCESS_GPA_CONTROLS {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct WHV_ACCESS_GPA_CONTROLS_0 {
    pub CacheType: WHV_CACHE_TYPE,
    pub Reserved: u32,
}
impl ::core::marker::Copy for WHV_ACCESS_GPA_CONTROLS_0 {}
impl ::core::clone::Clone for WHV_ACCESS_GPA_CONTROLS_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub union WHV_ADVISE_GPA_RANGE {
    pub Populate: WHV_ADVISE_GPA_RANGE_POPULATE,
}
impl ::core::marker::Copy for WHV_ADVISE_GPA_RANGE {}
impl ::core::clone::Clone for WHV_ADVISE_GPA_RANGE {
    fn clone(&self) -> Self {
        *self
    }
}
pub const WHvAdviseGpaRangeCodePopulate: i32 = 0i32;
pub const WHvAdviseGpaRangeCodePin: i32 = 1i32;
pub const WHvAdviseGpaRangeCodeUnpin: i32 = 2i32;
#[repr(C)]
pub struct WHV_ADVISE_GPA_RANGE_POPULATE {
    pub Flags: WHV_ADVISE_GPA_RANGE_POPULATE_FLAGS,
    pub AccessType: WHV_MEMORY_ACCESS_TYPE,
}
impl ::core::marker::Copy for WHV_ADVISE_GPA_RANGE_POPULATE {}
impl ::core::clone::Clone for WHV_ADVISE_GPA_RANGE_POPULATE {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub union WHV_ADVISE_GPA_RANGE_POPULATE_FLAGS {
    pub AsUINT32: u32,
    pub Anonymous: WHV_ADVISE_GPA_RANGE_POPULATE_FLAGS_0,
}
impl ::core::marker::Copy for WHV_ADVISE_GPA_RANGE_POPULATE_FLAGS {}
impl ::core::clone::Clone for WHV_ADVISE_GPA_RANGE_POPULATE_FLAGS {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct WHV_ADVISE_GPA_RANGE_POPULATE_FLAGS_0 {
    pub _bitfield: u32,
}
impl ::core::marker::Copy for WHV_ADVISE_GPA_RANGE_POPULATE_FLAGS_0 {}
impl ::core::clone::Clone for WHV_ADVISE_GPA_RANGE_POPULATE_FLAGS_0 {
    fn clone(&self) -> Self {
        *self
    }
}
pub const WHvAllocateVpciResourceFlagNone: u32 = 0u32;
pub const WHvAllocateVpciResourceFlagAllowDirectP2P: u32 = 1u32;
pub const WHV_ANY_VP: u32 = 4294967295u32;
pub const WHvCacheTypeUncached: i32 = 0i32;
pub const WHvCacheTypeWriteCombining: i32 = 1i32;
pub const WHvCacheTypeWriteThrough: i32 = 4i32;
pub const WHvCacheTypeWriteProtected: i32 = 5i32;
pub const WHvCacheTypeWriteBack: i32 = 6i32;
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub union WHV_CAPABILITY {
    pub HypervisorPresent: super::super::Foundation::BOOL,
    pub Features: WHV_CAPABILITY_FEATURES,
    pub ExtendedVmExits: WHV_EXTENDED_VM_EXITS,
    pub ProcessorVendor: WHV_PROCESSOR_VENDOR,
    pub ProcessorFeatures: WHV_PROCESSOR_FEATURES,
    pub SyntheticProcessorFeaturesBanks: WHV_SYNTHETIC_PROCESSOR_FEATURES_BANKS,
    pub ProcessorXsaveFeatures: WHV_PROCESSOR_XSAVE_FEATURES,
    pub ProcessorClFlushSize: u8,
    pub ExceptionExitBitmap: u64,
    pub X64MsrExitBitmap: WHV_X64_MSR_EXIT_BITMAP,
    pub ProcessorClockFrequency: u64,
    pub InterruptClockFrequency: u64,
    pub ProcessorFeaturesBanks: WHV_PROCESSOR_FEATURES_BANKS,
    pub GpaRangePopulateFlags: WHV_ADVISE_GPA_RANGE_POPULATE_FLAGS,
    pub ProcessorFrequencyCap: WHV_CAPABILITY_PROCESSOR_FREQUENCY_CAP,
    pub ProcessorPerfmonFeatures: WHV_PROCESSOR_PERFMON_FEATURES,
    pub SchedulerFeatures: WHV_SCHEDULER_FEATURES,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for WHV_CAPABILITY {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for WHV_CAPABILITY {
    fn clone(&self) -> Self {
        *self
    }
}
pub const WHvCapabilityCodeHypervisorPresent: i32 = 0i32;
pub const WHvCapabilityCodeFeatures: i32 = 1i32;
pub const WHvCapabilityCodeExtendedVmExits: i32 = 2i32;
pub const WHvCapabilityCodeExceptionExitBitmap: i32 = 3i32;
pub const WHvCapabilityCodeX64MsrExitBitmap: i32 = 4i32;
pub const WHvCapabilityCodeGpaRangePopulateFlags: i32 = 5i32;
pub const WHvCapabilityCodeSchedulerFeatures: i32 = 6i32;
pub const WHvCapabilityCodeProcessorVendor: i32 = 4096i32;
pub const WHvCapabilityCodeProcessorFeatures: i32 = 4097i32;
pub const WHvCapabilityCodeProcessorClFlushSize: i32 = 4098i32;
pub const WHvCapabilityCodeProcessorXsaveFeatures: i32 = 4099i32;
pub const WHvCapabilityCodeProcessorClockFrequency: i32 = 4100i32;
pub const WHvCapabilityCodeInterruptClockFrequency: i32 = 4101i32;
pub const WHvCapabilityCodeProcessorFeaturesBanks: i32 = 4102i32;
pub const WHvCapabilityCodeProcessorFrequencyCap: i32 = 4103i32;
pub const WHvCapabilityCodeSyntheticProcessorFeaturesBanks: i32 = 4104i32;
pub const WHvCapabilityCodeProcessorPerfmonFeatures: i32 = 4105i32;
#[repr(C)]
pub union WHV_CAPABILITY_FEATURES {
    pub Anonymous: WHV_CAPABILITY_FEATURES_0,
    pub AsUINT64: u64,
}
impl ::core::marker::Copy for WHV_CAPABILITY_FEATURES {}
impl ::core::clone::Clone for WHV_CAPABILITY_FEATURES {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct WHV_CAPABILITY_FEATURES_0 {
    pub _bitfield: u64,
}
impl ::core::marker::Copy for WHV_CAPABILITY_FEATURES_0 {}
impl ::core::clone::Clone for WHV_CAPABILITY_FEATURES_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct WHV_CAPABILITY_PROCESSOR_FREQUENCY_CAP {
    pub _bitfield: u32,
    pub HighestFrequencyMhz: u32,
    pub NominalFrequencyMhz: u32,
    pub LowestFrequencyMhz: u32,
    pub FrequencyStepMhz: u32,
}
impl ::core::marker::Copy for WHV_CAPABILITY_PROCESSOR_FREQUENCY_CAP {}
impl ::core::clone::Clone for WHV_CAPABILITY_PROCESSOR_FREQUENCY_CAP {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct WHV_CPUID_OUTPUT {
    pub Eax: u32,
    pub Ebx: u32,
    pub Ecx: u32,
    pub Edx: u32,
}
impl ::core::marker::Copy for WHV_CPUID_OUTPUT {}
impl ::core::clone::Clone for WHV_CPUID_OUTPUT {
    fn clone(&self) -> Self {
        *self
    }
}
pub const WHvCreateVpciDeviceFlagNone: u32 = 0u32;
pub const WHvCreateVpciDeviceFlagPhysicallyBacked: u32 = 1u32;
pub const WHvCreateVpciDeviceFlagUseLogicalInterrupts: u32 = 2u32;
#[repr(C)]
pub struct WHV_DOORBELL_MATCH_DATA {
    pub GuestAddress: u64,
    pub Value: u64,
    pub Length: u32,
    pub _bitfield: u32,
}
impl ::core::marker::Copy for WHV_DOORBELL_MATCH_DATA {}
impl ::core::clone::Clone for WHV_DOORBELL_MATCH_DATA {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct WHV_EMULATOR_CALLBACKS {
    pub Size: u32,
    pub Reserved: u32,
    pub WHvEmulatorIoPortCallback: WHV_EMULATOR_IO_PORT_CALLBACK,
    pub WHvEmulatorMemoryCallback: WHV_EMULATOR_MEMORY_CALLBACK,
    pub WHvEmulatorGetVirtualProcessorRegisters: WHV_EMULATOR_GET_VIRTUAL_PROCESSOR_REGISTERS_CALLBACK,
    pub WHvEmulatorSetVirtualProcessorRegisters: WHV_EMULATOR_SET_VIRTUAL_PROCESSOR_REGISTERS_CALLBACK,
    pub WHvEmulatorTranslateGvaPage: WHV_EMULATOR_TRANSLATE_GVA_PAGE_CALLBACK,
}
impl ::core::marker::Copy for WHV_EMULATOR_CALLBACKS {}
impl ::core::clone::Clone for WHV_EMULATOR_CALLBACKS {
    fn clone(&self) -> Self {
        *self
    }
}
pub type WHV_EMULATOR_GET_VIRTUAL_PROCESSOR_REGISTERS_CALLBACK = unsafe extern "system" fn(context: *const ::core::ffi::c_void, registernames: *const WHV_REGISTER_NAME, registercount: u32, registervalues: *mut WHV_REGISTER_VALUE) -> ::windows_sys::core::HRESULT;
#[repr(C)]
pub struct WHV_EMULATOR_IO_ACCESS_INFO {
    pub Direction: u8,
    pub Port: u16,
    pub AccessSize: u16,
    pub Data: u32,
}
impl ::core::marker::Copy for WHV_EMULATOR_IO_ACCESS_INFO {}
impl ::core::clone::Clone for WHV_EMULATOR_IO_ACCESS_INFO {
    fn clone(&self) -> Self {
        *self
    }
}
pub type WHV_EMULATOR_IO_PORT_CALLBACK = unsafe extern "system" fn(context: *const ::core::ffi::c_void, ioaccess: *mut WHV_EMULATOR_IO_ACCESS_INFO) -> ::windows_sys::core::HRESULT;
#[repr(C)]
pub struct WHV_EMULATOR_MEMORY_ACCESS_INFO {
    pub GpaAddress: u64,
    pub Direction: u8,
    pub AccessSize: u8,
    pub Data: [u8; 8],
}
impl ::core::marker::Copy for WHV_EMULATOR_MEMORY_ACCESS_INFO {}
impl ::core::clone::Clone for WHV_EMULATOR_MEMORY_ACCESS_INFO {
    fn clone(&self) -> Self {
        *self
    }
}
pub type WHV_EMULATOR_MEMORY_CALLBACK = unsafe extern "system" fn(context: *const ::core::ffi::c_void, memoryaccess: *mut WHV_EMULATOR_MEMORY_ACCESS_INFO) -> ::windows_sys::core::HRESULT;
pub type WHV_EMULATOR_SET_VIRTUAL_PROCESSOR_REGISTERS_CALLBACK = unsafe extern "system" fn(context: *const ::core::ffi::c_void, registernames: *const WHV_REGISTER_NAME, registercount: u32, registervalues: *const WHV_REGISTER_VALUE) -> ::windows_sys::core::HRESULT;
#[repr(C)]
pub union WHV_EMULATOR_STATUS {
    pub Anonymous: WHV_EMULATOR_STATUS_0,
    pub AsUINT32: u32,
}
impl ::core::marker::Copy for WHV_EMULATOR_STATUS {}
impl ::core::clone::Clone for WHV_EMULATOR_STATUS {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct WHV_EMULATOR_STATUS_0 {
    pub _bitfield: u32,
}
impl ::core::marker::Copy for WHV_EMULATOR_STATUS_0 {}
impl ::core::clone::Clone for WHV_EMULATOR_STATUS_0 {
    fn clone(&self) -> Self {
        *self
    }
}
pub type WHV_EMULATOR_TRANSLATE_GVA_PAGE_CALLBACK = unsafe extern "system" fn(context: *const ::core::ffi::c_void, gva: u64, translateflags: WHV_TRANSLATE_GVA_FLAGS, translationresult: *mut WHV_TRANSLATE_GVA_RESULT_CODE, gpa: *mut u64) -> ::windows_sys::core::HRESULT;
pub const WHvX64ExceptionTypeDivideErrorFault: i32 = 0i32;
pub const WHvX64ExceptionTypeDebugTrapOrFault: i32 = 1i32;
pub const WHvX64ExceptionTypeBreakpointTrap: i32 = 3i32;
pub const WHvX64ExceptionTypeOverflowTrap: i32 = 4i32;
pub const WHvX64ExceptionTypeBoundRangeFault: i32 = 5i32;
pub const WHvX64ExceptionTypeInvalidOpcodeFault: i32 = 6i32;
pub const WHvX64ExceptionTypeDeviceNotAvailableFault: i32 = 7i32;
pub const WHvX64ExceptionTypeDoubleFaultAbort: i32 = 8i32;
pub const WHvX64ExceptionTypeInvalidTaskStateSegmentFault: i32 = 10i32;
pub const WHvX64ExceptionTypeSegmentNotPresentFault: i32 = 11i32;
pub const WHvX64ExceptionTypeStackFault: i32 = 12i32;
pub const WHvX64ExceptionTypeGeneralProtectionFault: i32 = 13i32;
pub const WHvX64ExceptionTypePageFault: i32 = 14i32;
pub const WHvX64ExceptionTypeFloatingPointErrorFault: i32 = 16i32;
pub const WHvX64ExceptionTypeAlignmentCheckFault: i32 = 17i32;
pub const WHvX64ExceptionTypeMachineCheckAbort: i32 = 18i32;
pub const WHvX64ExceptionTypeSimdFloatingPointFault: i32 = 19i32;
#[repr(C)]
pub union WHV_EXTENDED_VM_EXITS {
    pub Anonymous: WHV_EXTENDED_VM_EXITS_0,
    pub AsUINT64: u64,
}
impl ::core::marker::Copy for WHV_EXTENDED_VM_EXITS {}
impl ::core::clone::Clone for WHV_EXTENDED_VM_EXITS {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct WHV_EXTENDED_VM_EXITS_0 {
    pub _bitfield: u64,
}
impl ::core::marker::Copy for WHV_EXTENDED_VM_EXITS_0 {}
impl ::core::clone::Clone for WHV_EXTENDED_VM_EXITS_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct WHV_HYPERCALL_CONTEXT {
    pub Rax: u64,
    pub Rbx: u64,
    pub Rcx: u64,
    pub Rdx: u64,
    pub R8: u64,
    pub Rsi: u64,
    pub Rdi: u64,
    pub Reserved0: u64,
    pub XmmRegisters: [WHV_UINT128; 6],
    pub Reserved1: [u64; 2],
}
impl ::core::marker::Copy for WHV_HYPERCALL_CONTEXT {}
impl ::core::clone::Clone for WHV_HYPERCALL_CONTEXT {
    fn clone(&self) -> Self {
        *self
    }
}
pub const WHV_HYPERCALL_CONTEXT_MAX_XMM_REGISTERS: u32 = 6u32;
#[repr(C)]
pub union WHV_INTERNAL_ACTIVITY_REGISTER {
    pub Anonymous: WHV_INTERNAL_ACTIVITY_REGISTER_0,
    pub AsUINT64: u64,
}
impl ::core::marker::Copy for WHV_INTERNAL_ACTIVITY_REGISTER {}
impl ::core::clone::Clone for WHV_INTERNAL_ACTIVITY_REGISTER {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct WHV_INTERNAL_ACTIVITY_REGISTER_0 {
    pub _bitfield: u64,
}
impl ::core::marker::Copy for WHV_INTERNAL_ACTIVITY_REGISTER_0 {}
impl ::core::clone::Clone for WHV_INTERNAL_ACTIVITY_REGISTER_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct WHV_INTERRUPT_CONTROL {
    pub _bitfield: u64,
    pub Destination: u32,
    pub Vector: u32,
}
impl ::core::marker::Copy for WHV_INTERRUPT_CONTROL {}
impl ::core::clone::Clone for WHV_INTERRUPT_CONTROL {
    fn clone(&self) -> Self {
        *self
    }
}
pub const WHvX64InterruptDestinationModePhysical: i32 = 0i32;
pub const WHvX64InterruptDestinationModeLogical: i32 = 1i32;
pub const WHvX64InterruptTriggerModeEdge: i32 = 0i32;
pub const WHvX64InterruptTriggerModeLevel: i32 = 1i32;
pub const WHvX64InterruptTypeFixed: i32 = 0i32;
pub const WHvX64InterruptTypeLowestPriority: i32 = 1i32;
pub const WHvX64InterruptTypeNmi: i32 = 4i32;
pub const WHvX64InterruptTypeInit: i32 = 5i32;
pub const WHvX64InterruptTypeSipi: i32 = 6i32;
pub const WHvX64InterruptTypeLocalInt1: i32 = 9i32;
pub const WHvMapGpaRangeFlagNone: u32 = 0u32;
pub const WHvMapGpaRangeFlagRead: u32 = 1u32;
pub const WHvMapGpaRangeFlagWrite: u32 = 2u32;
pub const WHvMapGpaRangeFlagExecute: u32 = 4u32;
pub const WHvMapGpaRangeFlagTrackDirtyPages: u32 = 8u32;
pub const WHV_MAX_DEVICE_ID_SIZE_IN_CHARS: u32 = 200u32;
#[repr(C)]
pub struct WHV_MEMORY_ACCESS_CONTEXT {
    pub InstructionByteCount: u8,
    pub Reserved: [u8; 3],
    pub InstructionBytes: [u8; 16],
    pub AccessInfo: WHV_MEMORY_ACCESS_INFO,
    pub Gpa: u64,
    pub Gva: u64,
}
impl ::core::marker::Copy for WHV_MEMORY_ACCESS_CONTEXT {}
impl ::core::clone::Clone for WHV_MEMORY_ACCESS_CONTEXT {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub union WHV_MEMORY_ACCESS_INFO {
    pub Anonymous: WHV_MEMORY_ACCESS_INFO_0,
    pub AsUINT32: u32,
}
impl ::core::marker::Copy for WHV_MEMORY_ACCESS_INFO {}
impl ::core::clone::Clone for WHV_MEMORY_ACCESS_INFO {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct WHV_MEMORY_ACCESS_INFO_0 {
    pub _bitfield: u32,
}
impl ::core::marker::Copy for WHV_MEMORY_ACCESS_INFO_0 {}
impl ::core::clone::Clone for WHV_MEMORY_ACCESS_INFO_0 {
    fn clone(&self) -> Self {
        *self
    }
}
pub const WHvMemoryAccessRead: i32 = 0i32;
pub const WHvMemoryAccessWrite: i32 = 1i32;
pub const WHvMemoryAccessExecute: i32 = 2i32;
#[repr(C)]
pub struct WHV_MEMORY_RANGE_ENTRY {
    pub GuestAddress: u64,
    pub SizeInBytes: u64,
}
impl ::core::marker::Copy for WHV_MEMORY_RANGE_ENTRY {}
impl ::core::clone::Clone for WHV_MEMORY_RANGE_ENTRY {
    fn clone(&self) -> Self {
        *self
    }
}
pub const WHvMsrActionArchitectureDefault: i32 = 0i32;
pub const WHvMsrActionIgnoreWriteReadZero: i32 = 1i32;
pub const WHvMsrActionExit: i32 = 2i32;
#[repr(C)]
pub struct WHV_MSR_ACTION_ENTRY {
    pub Index: u32,
    pub ReadAction: u8,
    pub WriteAction: u8,
    pub Reserved: u16,
}
impl ::core::marker::Copy for WHV_MSR_ACTION_ENTRY {}
impl ::core::clone::Clone for WHV_MSR_ACTION_ENTRY {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct WHV_NOTIFICATION_PORT_PARAMETERS {
    pub NotificationPortType: WHV_NOTIFICATION_PORT_TYPE,
    pub Reserved: u32,
    pub Anonymous: WHV_NOTIFICATION_PORT_PARAMETERS_0,
}
impl ::core::marker::Copy for WHV_NOTIFICATION_PORT_PARAMETERS {}
impl ::core::clone::Clone for WHV_NOTIFICATION_PORT_PARAMETERS {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub union WHV_NOTIFICATION_PORT_PARAMETERS_0 {
    pub Doorbell: WHV_DOORBELL_MATCH_DATA,
    pub Event: WHV_NOTIFICATION_PORT_PARAMETERS_0_0,
}
impl ::core::marker::Copy for WHV_NOTIFICATION_PORT_PARAMETERS_0 {}
impl ::core::clone::Clone for WHV_NOTIFICATION_PORT_PARAMETERS_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct WHV_NOTIFICATION_PORT_PARAMETERS_0_0 {
    pub ConnectionId: u32,
}
impl ::core::marker::Copy for WHV_NOTIFICATION_PORT_PARAMETERS_0_0 {}
impl ::core::clone::Clone for WHV_NOTIFICATION_PORT_PARAMETERS_0_0 {
    fn clone(&self) -> Self {
        *self
    }
}
pub const WHvNotificationPortPropertyPreferredTargetVp: i32 = 1i32;
pub const WHvNotificationPortPropertyPreferredTargetDuration: i32 = 5i32;
pub const WHvNotificationPortTypeEvent: i32 = 2i32;
pub const WHvNotificationPortTypeDoorbell: i32 = 4i32;
pub const WHvPartitionCounterSetMemory: i32 = 0i32;
pub type WHV_PARTITION_HANDLE = isize;
#[repr(C)]
pub struct WHV_PARTITION_MEMORY_COUNTERS {
    pub Mapped4KPageCount: u64,
    pub Mapped2MPageCount: u64,
    pub Mapped1GPageCount: u64,
}
impl ::core::marker::Copy for WHV_PARTITION_MEMORY_COUNTERS {}
impl ::core::clone::Clone for WHV_PARTITION_MEMORY_COUNTERS {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub union WHV_PARTITION_PROPERTY {
    pub ExtendedVmExits: WHV_EXTENDED_VM_EXITS,
    pub ProcessorFeatures: WHV_PROCESSOR_FEATURES,
    pub SyntheticProcessorFeaturesBanks: WHV_SYNTHETIC_PROCESSOR_FEATURES_BANKS,
    pub ProcessorXsaveFeatures: WHV_PROCESSOR_XSAVE_FEATURES,
    pub ProcessorClFlushSize: u8,
    pub ProcessorCount: u32,
    pub CpuidExitList: [u32; 1],
    pub CpuidResultList: [WHV_X64_CPUID_RESULT; 1],
    pub CpuidResultList2: [WHV_X64_CPUID_RESULT2; 1],
    pub MsrActionList: [WHV_MSR_ACTION_ENTRY; 1],
    pub UnimplementedMsrAction: WHV_MSR_ACTION,
    pub ExceptionExitBitmap: u64,
    pub LocalApicEmulationMode: WHV_X64_LOCAL_APIC_EMULATION_MODE,
    pub SeparateSecurityDomain: super::super::Foundation::BOOL,
    pub NestedVirtualization: super::super::Foundation::BOOL,
    pub X64MsrExitBitmap: WHV_X64_MSR_EXIT_BITMAP,
    pub ProcessorClockFrequency: u64,
    pub InterruptClockFrequency: u64,
    pub ApicRemoteRead: super::super::Foundation::BOOL,
    pub ProcessorFeaturesBanks: WHV_PROCESSOR_FEATURES_BANKS,
    pub ReferenceTime: u64,
    pub PrimaryNumaNode: u16,
    pub CpuReserve: u32,
    pub CpuCap: u32,
    pub CpuWeight: u32,
    pub CpuGroupId: u64,
    pub ProcessorFrequencyCap: u32,
    pub AllowDeviceAssignment: super::super::Foundation::BOOL,
    pub ProcessorPerfmonFeatures: WHV_PROCESSOR_PERFMON_FEATURES,
    pub DisableSmt: super::super::Foundation::BOOL,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for WHV_PARTITION_PROPERTY {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for WHV_PARTITION_PROPERTY {
    fn clone(&self) -> Self {
        *self
    }
}
pub const WHvPartitionPropertyCodeExtendedVmExits: i32 = 1i32;
pub const WHvPartitionPropertyCodeExceptionExitBitmap: i32 = 2i32;
pub const WHvPartitionPropertyCodeSeparateSecurityDomain: i32 = 3i32;
pub const WHvPartitionPropertyCodeNestedVirtualization: i32 = 4i32;
pub const WHvPartitionPropertyCodeX64MsrExitBitmap: i32 = 5i32;
pub const WHvPartitionPropertyCodePrimaryNumaNode: i32 = 6i32;
pub const WHvPartitionPropertyCodeCpuReserve: i32 = 7i32;
pub const WHvPartitionPropertyCodeCpuCap: i32 = 8i32;
pub const WHvPartitionPropertyCodeCpuWeight: i32 = 9i32;
pub const WHvPartitionPropertyCodeCpuGroupId: i32 = 10i32;
pub const WHvPartitionPropertyCodeProcessorFrequencyCap: i32 = 11i32;
pub const WHvPartitionPropertyCodeAllowDeviceAssignment: i32 = 12i32;
pub const WHvPartitionPropertyCodeDisableSmt: i32 = 13i32;
pub const WHvPartitionPropertyCodeProcessorFeatures: i32 = 4097i32;
pub const WHvPartitionPropertyCodeProcessorClFlushSize: i32 = 4098i32;
pub const WHvPartitionPropertyCodeCpuidExitList: i32 = 4099i32;
pub const WHvPartitionPropertyCodeCpuidResultList: i32 = 4100i32;
pub const WHvPartitionPropertyCodeLocalApicEmulationMode: i32 = 4101i32;
pub const WHvPartitionPropertyCodeProcessorXsaveFeatures: i32 = 4102i32;
pub const WHvPartitionPropertyCodeProcessorClockFrequency: i32 = 4103i32;
pub const WHvPartitionPropertyCodeInterruptClockFrequency: i32 = 4104i32;
pub const WHvPartitionPropertyCodeApicRemoteReadSupport: i32 = 4105i32;
pub const WHvPartitionPropertyCodeProcessorFeaturesBanks: i32 = 4106i32;
pub const WHvPartitionPropertyCodeReferenceTime: i32 = 4107i32;
pub const WHvPartitionPropertyCodeSyntheticProcessorFeaturesBanks: i32 = 4108i32;
pub const WHvPartitionPropertyCodeCpuidResultList2: i32 = 4109i32;
pub const WHvPartitionPropertyCodeProcessorPerfmonFeatures: i32 = 4110i32;
pub const WHvPartitionPropertyCodeMsrActionList: i32 = 4111i32;
pub const WHvPartitionPropertyCodeUnimplementedMsrAction: i32 = 4112i32;
pub const WHvPartitionPropertyCodeProcessorCount: i32 = 8191i32;
#[repr(C)]
pub struct WHV_PROCESSOR_APIC_COUNTERS {
    pub MmioAccessCount: u64,
    pub EoiAccessCount: u64,
    pub TprAccessCount: u64,
    pub SentIpiCount: u64,
    pub SelfIpiCount: u64,
}
impl ::core::marker::Copy for WHV_PROCESSOR_APIC_COUNTERS {}
impl ::core::clone::Clone for WHV_PROCESSOR_APIC_COUNTERS {
    fn clone(&self) -> Self {
        *self
    }
}
pub const WHvProcessorCounterSetRuntime: i32 = 0i32;
pub const WHvProcessorCounterSetIntercepts: i32 = 1i32;
pub const WHvProcessorCounterSetEvents: i32 = 2i32;
pub const WHvProcessorCounterSetApic: i32 = 3i32;
pub const WHvProcessorCounterSetSyntheticFeatures: i32 = 4i32;
#[repr(C)]
pub struct WHV_PROCESSOR_EVENT_COUNTERS {
    pub PageFaultCount: u64,
    pub ExceptionCount: u64,
    pub InterruptCount: u64,
}
impl ::core::marker::Copy for WHV_PROCESSOR_EVENT_COUNTERS {}
impl ::core::clone::Clone for WHV_PROCESSOR_EVENT_COUNTERS {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub union WHV_PROCESSOR_FEATURES {
    pub Anonymous: WHV_PROCESSOR_FEATURES_0,
    pub AsUINT64: u64,
}
impl ::core::marker::Copy for WHV_PROCESSOR_FEATURES {}
impl ::core::clone::Clone for WHV_PROCESSOR_FEATURES {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct WHV_PROCESSOR_FEATURES_0 {
    pub _bitfield: u64,
}
impl ::core::marker::Copy for WHV_PROCESSOR_FEATURES_0 {}
impl ::core::clone::Clone for WHV_PROCESSOR_FEATURES_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub union WHV_PROCESSOR_FEATURES1 {
    pub Anonymous: WHV_PROCESSOR_FEATURES1_0,
    pub AsUINT64: u64,
}
impl ::core::marker::Copy for WHV_PROCESSOR_FEATURES1 {}
impl ::core::clone::Clone for WHV_PROCESSOR_FEATURES1 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct WHV_PROCESSOR_FEATURES1_0 {
    pub _bitfield: u64,
}
impl ::core::marker::Copy for WHV_PROCESSOR_FEATURES1_0 {}
impl ::core::clone::Clone for WHV_PROCESSOR_FEATURES1_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct WHV_PROCESSOR_FEATURES_BANKS {
    pub BanksCount: u32,
    pub Reserved0: u32,
    pub Anonymous: WHV_PROCESSOR_FEATURES_BANKS_0,
}
impl ::core::marker::Copy for WHV_PROCESSOR_FEATURES_BANKS {}
impl ::core::clone::Clone for WHV_PROCESSOR_FEATURES_BANKS {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub union WHV_PROCESSOR_FEATURES_BANKS_0 {
    pub Anonymous: WHV_PROCESSOR_FEATURES_BANKS_0_0,
    pub AsUINT64: [u64; 2],
}
impl ::core::marker::Copy for WHV_PROCESSOR_FEATURES_BANKS_0 {}
impl ::core::clone::Clone for WHV_PROCESSOR_FEATURES_BANKS_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct WHV_PROCESSOR_FEATURES_BANKS_0_0 {
    pub Bank0: WHV_PROCESSOR_FEATURES,
    pub Bank1: WHV_PROCESSOR_FEATURES1,
}
impl ::core::marker::Copy for WHV_PROCESSOR_FEATURES_BANKS_0_0 {}
impl ::core::clone::Clone for WHV_PROCESSOR_FEATURES_BANKS_0_0 {
    fn clone(&self) -> Self {
        *self
    }
}
pub const WHV_PROCESSOR_FEATURES_BANKS_COUNT: u32 = 2u32;
#[repr(C)]
pub struct WHV_PROCESSOR_INTERCEPT_COUNTER {
    pub Count: u64,
    pub Time100ns: u64,
}
impl ::core::marker::Copy for WHV_PROCESSOR_INTERCEPT_COUNTER {}
impl ::core::clone::Clone for WHV_PROCESSOR_INTERCEPT_COUNTER {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct WHV_PROCESSOR_INTERCEPT_COUNTERS {
    pub PageInvalidations: WHV_PROCESSOR_INTERCEPT_COUNTER,
    pub ControlRegisterAccesses: WHV_PROCESSOR_INTERCEPT_COUNTER,
    pub IoInstructions: WHV_PROCESSOR_INTERCEPT_COUNTER,
    pub HaltInstructions: WHV_PROCESSOR_INTERCEPT_COUNTER,
    pub CpuidInstructions: WHV_PROCESSOR_INTERCEPT_COUNTER,
    pub MsrAccesses: WHV_PROCESSOR_INTERCEPT_COUNTER,
    pub OtherIntercepts: WHV_PROCESSOR_INTERCEPT_COUNTER,
    pub PendingInterrupts: WHV_PROCESSOR_INTERCEPT_COUNTER,
    pub EmulatedInstructions: WHV_PROCESSOR_INTERCEPT_COUNTER,
    pub DebugRegisterAccesses: WHV_PROCESSOR_INTERCEPT_COUNTER,
    pub PageFaultIntercepts: WHV_PROCESSOR_INTERCEPT_COUNTER,
    pub NestedPageFaultIntercepts: WHV_PROCESSOR_INTERCEPT_COUNTER,
    pub Hypercalls: WHV_PROCESSOR_INTERCEPT_COUNTER,
    pub RdpmcInstructions: WHV_PROCESSOR_INTERCEPT_COUNTER,
}
impl ::core::marker::Copy for WHV_PROCESSOR_INTERCEPT_COUNTERS {}
impl ::core::clone::Clone for WHV_PROCESSOR_INTERCEPT_COUNTERS {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub union WHV_PROCESSOR_PERFMON_FEATURES {
    pub Anonymous: WHV_PROCESSOR_PERFMON_FEATURES_0,
    pub AsUINT64: u64,
}
impl ::core::marker::Copy for WHV_PROCESSOR_PERFMON_FEATURES {}
impl ::core::clone::Clone for WHV_PROCESSOR_PERFMON_FEATURES {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct WHV_PROCESSOR_PERFMON_FEATURES_0 {
    pub _bitfield: u64,
}
impl ::core::marker::Copy for WHV_PROCESSOR_PERFMON_FEATURES_0 {}
impl ::core::clone::Clone for WHV_PROCESSOR_PERFMON_FEATURES_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct WHV_PROCESSOR_RUNTIME_COUNTERS {
    pub TotalRuntime100ns: u64,
    pub HypervisorRuntime100ns: u64,
}
impl ::core::marker::Copy for WHV_PROCESSOR_RUNTIME_COUNTERS {}
impl ::core::clone::Clone for WHV_PROCESSOR_RUNTIME_COUNTERS {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct WHV_PROCESSOR_SYNTHETIC_FEATURES_COUNTERS {
    pub SyntheticInterruptsCount: u64,
    pub LongSpinWaitHypercallsCount: u64,
    pub OtherHypercallsCount: u64,
    pub SyntheticInterruptHypercallsCount: u64,
    pub VirtualInterruptHypercallsCount: u64,
    pub VirtualMmuHypercallsCount: u64,
}
impl ::core::marker::Copy for WHV_PROCESSOR_SYNTHETIC_FEATURES_COUNTERS {}
impl ::core::clone::Clone for WHV_PROCESSOR_SYNTHETIC_FEATURES_COUNTERS {
    fn clone(&self) -> Self {
        *self
    }
}
pub const WHvProcessorVendorAmd: i32 = 0i32;
pub const WHvProcessorVendorIntel: i32 = 1i32;
pub const WHvProcessorVendorHygon: i32 = 2i32;
#[repr(C)]
pub union WHV_PROCESSOR_XSAVE_FEATURES {
    pub Anonymous: WHV_PROCESSOR_XSAVE_FEATURES_0,
    pub AsUINT64: u64,
}
impl ::core::marker::Copy for WHV_PROCESSOR_XSAVE_FEATURES {}
impl ::core::clone::Clone for WHV_PROCESSOR_XSAVE_FEATURES {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct WHV_PROCESSOR_XSAVE_FEATURES_0 {
    pub _bitfield: u64,
}
impl ::core::marker::Copy for WHV_PROCESSOR_XSAVE_FEATURES_0 {}
impl ::core::clone::Clone for WHV_PROCESSOR_XSAVE_FEATURES_0 {
    fn clone(&self) -> Self {
        *self
    }
}
pub const WHV_READ_WRITE_GPA_RANGE_MAX_SIZE: u32 = 16u32;
pub const WHvX64RegisterRax: i32 = 0i32;
pub const WHvX64RegisterRcx: i32 = 1i32;
pub const WHvX64RegisterRdx: i32 = 2i32;
pub const WHvX64RegisterRbx: i32 = 3i32;
pub const WHvX64RegisterRsp: i32 = 4i32;
pub const WHvX64RegisterRbp: i32 = 5i32;
pub const WHvX64RegisterRsi: i32 = 6i32;
pub const WHvX64RegisterRdi: i32 = 7i32;
pub const WHvX64RegisterR8: i32 = 8i32;
pub const WHvX64RegisterR9: i32 = 9i32;
pub const WHvX64RegisterR10: i32 = 10i32;
pub const WHvX64RegisterR11: i32 = 11i32;
pub const WHvX64RegisterR12: i32 = 12i32;
pub const WHvX64RegisterR13: i32 = 13i32;
pub const WHvX64RegisterR14: i32 = 14i32;
pub const WHvX64RegisterR15: i32 = 15i32;
pub const WHvX64RegisterRip: i32 = 16i32;
pub const WHvX64RegisterRflags: i32 = 17i32;
pub const WHvX64RegisterEs: i32 = 18i32;
pub const WHvX64RegisterCs: i32 = 19i32;
pub const WHvX64RegisterSs: i32 = 20i32;
pub const WHvX64RegisterDs: i32 = 21i32;
pub const WHvX64RegisterFs: i32 = 22i32;
pub const WHvX64RegisterGs: i32 = 23i32;
pub const WHvX64RegisterLdtr: i32 = 24i32;
pub const WHvX64RegisterTr: i32 = 25i32;
pub const WHvX64RegisterIdtr: i32 = 26i32;
pub const WHvX64RegisterGdtr: i32 = 27i32;
pub const WHvX64RegisterCr0: i32 = 28i32;
pub const WHvX64RegisterCr2: i32 = 29i32;
pub const WHvX64RegisterCr3: i32 = 30i32;
pub const WHvX64RegisterCr4: i32 = 31i32;
pub const WHvX64RegisterCr8: i32 = 32i32;
pub const WHvX64RegisterDr0: i32 = 33i32;
pub const WHvX64RegisterDr1: i32 = 34i32;
pub const WHvX64RegisterDr2: i32 = 35i32;
pub const WHvX64RegisterDr3: i32 = 36i32;
pub const WHvX64RegisterDr6: i32 = 37i32;
pub const WHvX64RegisterDr7: i32 = 38i32;
pub const WHvX64RegisterXCr0: i32 = 39i32;
pub const WHvX64RegisterVirtualCr0: i32 = 40i32;
pub const WHvX64RegisterVirtualCr3: i32 = 41i32;
pub const WHvX64RegisterVirtualCr4: i32 = 42i32;
pub const WHvX64RegisterVirtualCr8: i32 = 43i32;
pub const WHvX64RegisterXmm0: i32 = 4096i32;
pub const WHvX64RegisterXmm1: i32 = 4097i32;
pub const WHvX64RegisterXmm2: i32 = 4098i32;
pub const WHvX64RegisterXmm3: i32 = 4099i32;
pub const WHvX64RegisterXmm4: i32 = 4100i32;
pub const WHvX64RegisterXmm5: i32 = 4101i32;
pub const WHvX64RegisterXmm6: i32 = 4102i32;
pub const WHvX64RegisterXmm7: i32 = 4103i32;
pub const WHvX64RegisterXmm8: i32 = 4104i32;
pub const WHvX64RegisterXmm9: i32 = 4105i32;
pub const WHvX64RegisterXmm10: i32 = 4106i32;
pub const WHvX64RegisterXmm11: i32 = 4107i32;
pub const WHvX64RegisterXmm12: i32 = 4108i32;
pub const WHvX64RegisterXmm13: i32 = 4109i32;
pub const WHvX64RegisterXmm14: i32 = 4110i32;
pub const WHvX64RegisterXmm15: i32 = 4111i32;
pub const WHvX64RegisterFpMmx0: i32 = 4112i32;
pub const WHvX64RegisterFpMmx1: i32 = 4113i32;
pub const WHvX64RegisterFpMmx2: i32 = 4114i32;
pub const WHvX64RegisterFpMmx3: i32 = 4115i32;
pub const WHvX64RegisterFpMmx4: i32 = 4116i32;
pub const WHvX64RegisterFpMmx5: i32 = 4117i32;
pub const WHvX64RegisterFpMmx6: i32 = 4118i32;
pub const WHvX64RegisterFpMmx7: i32 = 4119i32;
pub const WHvX64RegisterFpControlStatus: i32 = 4120i32;
pub const WHvX64RegisterXmmControlStatus: i32 = 4121i32;
pub const WHvX64RegisterTsc: i32 = 8192i32;
pub const WHvX64RegisterEfer: i32 = 8193i32;
pub const WHvX64RegisterKernelGsBase: i32 = 8194i32;
pub const WHvX64RegisterApicBase: i32 = 8195i32;
pub const WHvX64RegisterPat: i32 = 8196i32;
pub const WHvX64RegisterSysenterCs: i32 = 8197i32;
pub const WHvX64RegisterSysenterEip: i32 = 8198i32;
pub const WHvX64RegisterSysenterEsp: i32 = 8199i32;
pub const WHvX64RegisterStar: i32 = 8200i32;
pub const WHvX64RegisterLstar: i32 = 8201i32;
pub const WHvX64RegisterCstar: i32 = 8202i32;
pub const WHvX64RegisterSfmask: i32 = 8203i32;
pub const WHvX64RegisterInitialApicId: i32 = 8204i32;
pub const WHvX64RegisterMsrMtrrCap: i32 = 8205i32;
pub const WHvX64RegisterMsrMtrrDefType: i32 = 8206i32;
pub const WHvX64RegisterMsrMtrrPhysBase0: i32 = 8208i32;
pub const WHvX64RegisterMsrMtrrPhysBase1: i32 = 8209i32;
pub const WHvX64RegisterMsrMtrrPhysBase2: i32 = 8210i32;
pub const WHvX64RegisterMsrMtrrPhysBase3: i32 = 8211i32;
pub const WHvX64RegisterMsrMtrrPhysBase4: i32 = 8212i32;
pub const WHvX64RegisterMsrMtrrPhysBase5: i32 = 8213i32;
pub const WHvX64RegisterMsrMtrrPhysBase6: i32 = 8214i32;
pub const WHvX64RegisterMsrMtrrPhysBase7: i32 = 8215i32;
pub const WHvX64RegisterMsrMtrrPhysBase8: i32 = 8216i32;
pub const WHvX64RegisterMsrMtrrPhysBase9: i32 = 8217i32;
pub const WHvX64RegisterMsrMtrrPhysBaseA: i32 = 8218i32;
pub const WHvX64RegisterMsrMtrrPhysBaseB: i32 = 8219i32;
pub const WHvX64RegisterMsrMtrrPhysBaseC: i32 = 8220i32;
pub const WHvX64RegisterMsrMtrrPhysBaseD: i32 = 8221i32;
pub const WHvX64RegisterMsrMtrrPhysBaseE: i32 = 8222i32;
pub const WHvX64RegisterMsrMtrrPhysBaseF: i32 = 8223i32;
pub const WHvX64RegisterMsrMtrrPhysMask0: i32 = 8256i32;
pub const WHvX64RegisterMsrMtrrPhysMask1: i32 = 8257i32;
pub const WHvX64RegisterMsrMtrrPhysMask2: i32 = 8258i32;
pub const WHvX64RegisterMsrMtrrPhysMask3: i32 = 8259i32;
pub const WHvX64RegisterMsrMtrrPhysMask4: i32 = 8260i32;
pub const WHvX64RegisterMsrMtrrPhysMask5: i32 = 8261i32;
pub const WHvX64RegisterMsrMtrrPhysMask6: i32 = 8262i32;
pub const WHvX64RegisterMsrMtrrPhysMask7: i32 = 8263i32;
pub const WHvX64RegisterMsrMtrrPhysMask8: i32 = 8264i32;
pub const WHvX64RegisterMsrMtrrPhysMask9: i32 = 8265i32;
pub const WHvX64RegisterMsrMtrrPhysMaskA: i32 = 8266i32;
pub const WHvX64RegisterMsrMtrrPhysMaskB: i32 = 8267i32;
pub const WHvX64RegisterMsrMtrrPhysMaskC: i32 = 8268i32;
pub const WHvX64RegisterMsrMtrrPhysMaskD: i32 = 8269i32;
pub const WHvX64RegisterMsrMtrrPhysMaskE: i32 = 8270i32;
pub const WHvX64RegisterMsrMtrrPhysMaskF: i32 = 8271i32;
pub const WHvX64RegisterMsrMtrrFix64k00000: i32 = 8304i32;
pub const WHvX64RegisterMsrMtrrFix16k80000: i32 = 8305i32;
pub const WHvX64RegisterMsrMtrrFix16kA0000: i32 = 8306i32;
pub const WHvX64RegisterMsrMtrrFix4kC0000: i32 = 8307i32;
pub const WHvX64RegisterMsrMtrrFix4kC8000: i32 = 8308i32;
pub const WHvX64RegisterMsrMtrrFix4kD0000: i32 = 8309i32;
pub const WHvX64RegisterMsrMtrrFix4kD8000: i32 = 8310i32;
pub const WHvX64RegisterMsrMtrrFix4kE0000: i32 = 8311i32;
pub const WHvX64RegisterMsrMtrrFix4kE8000: i32 = 8312i32;
pub const WHvX64RegisterMsrMtrrFix4kF0000: i32 = 8313i32;
pub const WHvX64RegisterMsrMtrrFix4kF8000: i32 = 8314i32;
pub const WHvX64RegisterTscAux: i32 = 8315i32;
pub const WHvX64RegisterBndcfgs: i32 = 8316i32;
pub const WHvX64RegisterMCount: i32 = 8318i32;
pub const WHvX64RegisterACount: i32 = 8319i32;
pub const WHvX64RegisterSpecCtrl: i32 = 8324i32;
pub const WHvX64RegisterPredCmd: i32 = 8325i32;
pub const WHvX64RegisterTscVirtualOffset: i32 = 8327i32;
pub const WHvX64RegisterTsxCtrl: i32 = 8328i32;
pub const WHvX64RegisterXss: i32 = 8331i32;
pub const WHvX64RegisterUCet: i32 = 8332i32;
pub const WHvX64RegisterSCet: i32 = 8333i32;
pub const WHvX64RegisterSsp: i32 = 8334i32;
pub const WHvX64RegisterPl0Ssp: i32 = 8335i32;
pub const WHvX64RegisterPl1Ssp: i32 = 8336i32;
pub const WHvX64RegisterPl2Ssp: i32 = 8337i32;
pub const WHvX64RegisterPl3Ssp: i32 = 8338i32;
pub const WHvX64RegisterInterruptSspTableAddr: i32 = 8339i32;
pub const WHvX64RegisterTscDeadline: i32 = 8341i32;
pub const WHvX64RegisterTscAdjust: i32 = 8342i32;
pub const WHvX64RegisterUmwaitControl: i32 = 8344i32;
pub const WHvX64RegisterXfd: i32 = 8345i32;
pub const WHvX64RegisterXfdErr: i32 = 8346i32;
pub const WHvX64RegisterApicId: i32 = 12290i32;
pub const WHvX64RegisterApicVersion: i32 = 12291i32;
pub const WHvX64RegisterApicTpr: i32 = 12296i32;
pub const WHvX64RegisterApicPpr: i32 = 12298i32;
pub const WHvX64RegisterApicEoi: i32 = 12299i32;
pub const WHvX64RegisterApicLdr: i32 = 12301i32;
pub const WHvX64RegisterApicSpurious: i32 = 12303i32;
pub const WHvX64RegisterApicIsr0: i32 = 12304i32;
pub const WHvX64RegisterApicIsr1: i32 = 12305i32;
pub const WHvX64RegisterApicIsr2: i32 = 12306i32;
pub const WHvX64RegisterApicIsr3: i32 = 12307i32;
pub const WHvX64RegisterApicIsr4: i32 = 12308i32;
pub const WHvX64RegisterApicIsr5: i32 = 12309i32;
pub const WHvX64RegisterApicIsr6: i32 = 12310i32;
pub const WHvX64RegisterApicIsr7: i32 = 12311i32;
pub const WHvX64RegisterApicTmr0: i32 = 12312i32;
pub const WHvX64RegisterApicTmr1: i32 = 12313i32;
pub const WHvX64RegisterApicTmr2: i32 = 12314i32;
pub const WHvX64RegisterApicTmr3: i32 = 12315i32;
pub const WHvX64RegisterApicTmr4: i32 = 12316i32;
pub const WHvX64RegisterApicTmr5: i32 = 12317i32;
pub const WHvX64RegisterApicTmr6: i32 = 12318i32;
pub const WHvX64RegisterApicTmr7: i32 = 12319i32;
pub const WHvX64RegisterApicIrr0: i32 = 12320i32;
pub const WHvX64RegisterApicIrr1: i32 = 12321i32;
pub const WHvX64RegisterApicIrr2: i32 = 12322i32;
pub const WHvX64RegisterApicIrr3: i32 = 12323i32;
pub const WHvX64RegisterApicIrr4: i32 = 12324i32;
pub const WHvX64RegisterApicIrr5: i32 = 12325i32;
pub const WHvX64RegisterApicIrr6: i32 = 12326i32;
pub const WHvX64RegisterApicIrr7: i32 = 12327i32;
pub const WHvX64RegisterApicEse: i32 = 12328i32;
pub const WHvX64RegisterApicIcr: i32 = 12336i32;
pub const WHvX64RegisterApicLvtTimer: i32 = 12338i32;
pub const WHvX64RegisterApicLvtThermal: i32 = 12339i32;
pub const WHvX64RegisterApicLvtPerfmon: i32 = 12340i32;
pub const WHvX64RegisterApicLvtLint0: i32 = 12341i32;
pub const WHvX64RegisterApicLvtLint1: i32 = 12342i32;
pub const WHvX64RegisterApicLvtError: i32 = 12343i32;
pub const WHvX64RegisterApicInitCount: i32 = 12344i32;
pub const WHvX64RegisterApicCurrentCount: i32 = 12345i32;
pub const WHvX64RegisterApicDivide: i32 = 12350i32;
pub const WHvX64RegisterApicSelfIpi: i32 = 12351i32;
pub const WHvRegisterSint0: i32 = 16384i32;
pub const WHvRegisterSint1: i32 = 16385i32;
pub const WHvRegisterSint2: i32 = 16386i32;
pub const WHvRegisterSint3: i32 = 16387i32;
pub const WHvRegisterSint4: i32 = 16388i32;
pub const WHvRegisterSint5: i32 = 16389i32;
pub const WHvRegisterSint6: i32 = 16390i32;
pub const WHvRegisterSint7: i32 = 16391i32;
pub const WHvRegisterSint8: i32 = 16392i32;
pub const WHvRegisterSint9: i32 = 16393i32;
pub const WHvRegisterSint10: i32 = 16394i32;
pub const WHvRegisterSint11: i32 = 16395i32;
pub const WHvRegisterSint12: i32 = 16396i32;
pub const WHvRegisterSint13: i32 = 16397i32;
pub const WHvRegisterSint14: i32 = 16398i32;
pub const WHvRegisterSint15: i32 = 16399i32;
pub const WHvRegisterScontrol: i32 = 16400i32;
pub const WHvRegisterSversion: i32 = 16401i32;
pub const WHvRegisterSiefp: i32 = 16402i32;
pub const WHvRegisterSimp: i32 = 16403i32;
pub const WHvRegisterEom: i32 = 16404i32;
pub const WHvRegisterVpRuntime: i32 = 20480i32;
pub const WHvX64RegisterHypercall: i32 = 20481i32;
pub const WHvRegisterGuestOsId: i32 = 20482i32;
pub const WHvRegisterVpAssistPage: i32 = 20499i32;
pub const WHvRegisterReferenceTsc: i32 = 20503i32;
pub const WHvRegisterReferenceTscSequence: i32 = 20506i32;
pub const WHvRegisterPendingInterruption: i32 = -2147483648i32;
pub const WHvRegisterInterruptState: i32 = -2147483647i32;
pub const WHvRegisterPendingEvent: i32 = -2147483646i32;
pub const WHvX64RegisterDeliverabilityNotifications: i32 = -2147483644i32;
pub const WHvRegisterInternalActivityState: i32 = -2147483643i32;
pub const WHvX64RegisterPendingDebugException: i32 = -2147483642i32;
#[repr(C)]
pub union WHV_REGISTER_VALUE {
    pub Reg128: WHV_UINT128,
    pub Reg64: u64,
    pub Reg32: u32,
    pub Reg16: u16,
    pub Reg8: u8,
    pub Fp: WHV_X64_FP_REGISTER,
    pub FpControlStatus: WHV_X64_FP_CONTROL_STATUS_REGISTER,
    pub XmmControlStatus: WHV_X64_XMM_CONTROL_STATUS_REGISTER,
    pub Segment: WHV_X64_SEGMENT_REGISTER,
    pub Table: WHV_X64_TABLE_REGISTER,
    pub InterruptState: WHV_X64_INTERRUPT_STATE_REGISTER,
    pub PendingInterruption: WHV_X64_PENDING_INTERRUPTION_REGISTER,
    pub DeliverabilityNotifications: WHV_X64_DELIVERABILITY_NOTIFICATIONS_REGISTER,
    pub ExceptionEvent: WHV_X64_PENDING_EXCEPTION_EVENT,
    pub ExtIntEvent: WHV_X64_PENDING_EXT_INT_EVENT,
    pub InternalActivity: WHV_INTERNAL_ACTIVITY_REGISTER,
    pub PendingDebugException: WHV_X64_PENDING_DEBUG_EXCEPTION,
}
impl ::core::marker::Copy for WHV_REGISTER_VALUE {}
impl ::core::clone::Clone for WHV_REGISTER_VALUE {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct WHV_RUN_VP_CANCELED_CONTEXT {
    pub CancelReason: WHV_RUN_VP_CANCEL_REASON,
}
impl ::core::marker::Copy for WHV_RUN_VP_CANCELED_CONTEXT {}
impl ::core::clone::Clone for WHV_RUN_VP_CANCELED_CONTEXT {
    fn clone(&self) -> Self {
        *self
    }
}
pub const WHvRunVpCancelReasonUser: i32 = 0i32;
#[repr(C)]
pub struct WHV_RUN_VP_EXIT_CONTEXT {
    pub ExitReason: WHV_RUN_VP_EXIT_REASON,
    pub Reserved: u32,
    pub VpContext: WHV_VP_EXIT_CONTEXT,
    pub Anonymous: WHV_RUN_VP_EXIT_CONTEXT_0,
}
impl ::core::marker::Copy for WHV_RUN_VP_EXIT_CONTEXT {}
impl ::core::clone::Clone for WHV_RUN_VP_EXIT_CONTEXT {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub union WHV_RUN_VP_EXIT_CONTEXT_0 {
    pub MemoryAccess: WHV_MEMORY_ACCESS_CONTEXT,
    pub IoPortAccess: WHV_X64_IO_PORT_ACCESS_CONTEXT,
    pub MsrAccess: WHV_X64_MSR_ACCESS_CONTEXT,
    pub CpuidAccess: WHV_X64_CPUID_ACCESS_CONTEXT,
    pub VpException: WHV_VP_EXCEPTION_CONTEXT,
    pub InterruptWindow: WHV_X64_INTERRUPTION_DELIVERABLE_CONTEXT,
    pub UnsupportedFeature: WHV_X64_UNSUPPORTED_FEATURE_CONTEXT,
    pub CancelReason: WHV_RUN_VP_CANCELED_CONTEXT,
    pub ApicEoi: WHV_X64_APIC_EOI_CONTEXT,
    pub ReadTsc: WHV_X64_RDTSC_CONTEXT,
    pub ApicSmi: WHV_X64_APIC_SMI_CONTEXT,
    pub Hypercall: WHV_HYPERCALL_CONTEXT,
    pub ApicInitSipi: WHV_X64_APIC_INIT_SIPI_CONTEXT,
    pub ApicWrite: WHV_X64_APIC_WRITE_CONTEXT,
    pub SynicSintDeliverable: WHV_SYNIC_SINT_DELIVERABLE_CONTEXT,
}
impl ::core::marker::Copy for WHV_RUN_VP_EXIT_CONTEXT_0 {}
impl ::core::clone::Clone for WHV_RUN_VP_EXIT_CONTEXT_0 {
    fn clone(&self) -> Self {
        *self
    }
}
pub const WHvRunVpExitReasonNone: i32 = 0i32;
pub const WHvRunVpExitReasonMemoryAccess: i32 = 1i32;
pub const WHvRunVpExitReasonX64IoPortAccess: i32 = 2i32;
pub const WHvRunVpExitReasonUnrecoverableException: i32 = 4i32;
pub const WHvRunVpExitReasonInvalidVpRegisterValue: i32 = 5i32;
pub const WHvRunVpExitReasonUnsupportedFeature: i32 = 6i32;
pub const WHvRunVpExitReasonX64InterruptWindow: i32 = 7i32;
pub const WHvRunVpExitReasonX64Halt: i32 = 8i32;
pub const WHvRunVpExitReasonX64ApicEoi: i32 = 9i32;
pub const WHvRunVpExitReasonSynicSintDeliverable: i32 = 10i32;
pub const WHvRunVpExitReasonX64MsrAccess: i32 = 4096i32;
pub const WHvRunVpExitReasonX64Cpuid: i32 = 4097i32;
pub const WHvRunVpExitReasonException: i32 = 4098i32;
pub const WHvRunVpExitReasonX64Rdtsc: i32 = 4099i32;
pub const WHvRunVpExitReasonX64ApicSmiTrap: i32 = 4100i32;
pub const WHvRunVpExitReasonHypercall: i32 = 4101i32;
pub const WHvRunVpExitReasonX64ApicInitSipiTrap: i32 = 4102i32;
pub const WHvRunVpExitReasonX64ApicWriteTrap: i32 = 4103i32;
pub const WHvRunVpExitReasonCanceled: i32 = 8193i32;
#[repr(C)]
pub union WHV_SCHEDULER_FEATURES {
    pub Anonymous: WHV_SCHEDULER_FEATURES_0,
    pub AsUINT64: u64,
}
impl ::core::marker::Copy for WHV_SCHEDULER_FEATURES {}
impl ::core::clone::Clone for WHV_SCHEDULER_FEATURES {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct WHV_SCHEDULER_FEATURES_0 {
    pub _bitfield: u64,
}
impl ::core::marker::Copy for WHV_SCHEDULER_FEATURES_0 {}
impl ::core::clone::Clone for WHV_SCHEDULER_FEATURES_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct WHV_SRIOV_RESOURCE_DESCRIPTOR {
    pub PnpInstanceId: [u16; 200],
    pub VirtualFunctionId: super::super::Foundation::LUID,
    pub VirtualFunctionIndex: u16,
    pub Reserved: u16,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for WHV_SRIOV_RESOURCE_DESCRIPTOR {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for WHV_SRIOV_RESOURCE_DESCRIPTOR {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct WHV_SYNIC_EVENT_PARAMETERS {
    pub VpIndex: u32,
    pub TargetSint: u8,
    pub Reserved: u8,
    pub FlagNumber: u16,
}
impl ::core::marker::Copy for WHV_SYNIC_EVENT_PARAMETERS {}
impl ::core::clone::Clone for WHV_SYNIC_EVENT_PARAMETERS {
    fn clone(&self) -> Self {
        *self
    }
}
pub const WHV_SYNIC_MESSAGE_SIZE: u32 = 256u32;
#[repr(C)]
pub struct WHV_SYNIC_SINT_DELIVERABLE_CONTEXT {
    pub DeliverableSints: u16,
    pub Reserved1: u16,
    pub Reserved2: u32,
}
impl ::core::marker::Copy for WHV_SYNIC_SINT_DELIVERABLE_CONTEXT {}
impl ::core::clone::Clone for WHV_SYNIC_SINT_DELIVERABLE_CONTEXT {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub union WHV_SYNTHETIC_PROCESSOR_FEATURES {
    pub Anonymous: WHV_SYNTHETIC_PROCESSOR_FEATURES_0,
    pub AsUINT64: u64,
}
impl ::core::marker::Copy for WHV_SYNTHETIC_PROCESSOR_FEATURES {}
impl ::core::clone::Clone for WHV_SYNTHETIC_PROCESSOR_FEATURES {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct WHV_SYNTHETIC_PROCESSOR_FEATURES_0 {
    pub _bitfield: u64,
}
impl ::core::marker::Copy for WHV_SYNTHETIC_PROCESSOR_FEATURES_0 {}
impl ::core::clone::Clone for WHV_SYNTHETIC_PROCESSOR_FEATURES_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct WHV_SYNTHETIC_PROCESSOR_FEATURES_BANKS {
    pub BanksCount: u32,
    pub Reserved0: u32,
    pub Anonymous: WHV_SYNTHETIC_PROCESSOR_FEATURES_BANKS_0,
}
impl ::core::marker::Copy for WHV_SYNTHETIC_PROCESSOR_FEATURES_BANKS {}
impl ::core::clone::Clone for WHV_SYNTHETIC_PROCESSOR_FEATURES_BANKS {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub union WHV_SYNTHETIC_PROCESSOR_FEATURES_BANKS_0 {
    pub Anonymous: WHV_SYNTHETIC_PROCESSOR_FEATURES_BANKS_0_0,
    pub AsUINT64: [u64; 1],
}
impl ::core::marker::Copy for WHV_SYNTHETIC_PROCESSOR_FEATURES_BANKS_0 {}
impl ::core::clone::Clone for WHV_SYNTHETIC_PROCESSOR_FEATURES_BANKS_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct WHV_SYNTHETIC_PROCESSOR_FEATURES_BANKS_0_0 {
    pub Bank0: WHV_SYNTHETIC_PROCESSOR_FEATURES,
}
impl ::core::marker::Copy for WHV_SYNTHETIC_PROCESSOR_FEATURES_BANKS_0_0 {}
impl ::core::clone::Clone for WHV_SYNTHETIC_PROCESSOR_FEATURES_BANKS_0_0 {
    fn clone(&self) -> Self {
        *self
    }
}
pub const WHV_SYNTHETIC_PROCESSOR_FEATURES_BANKS_COUNT: u32 = 1u32;
pub const WHvTranslateGvaFlagNone: u32 = 0u32;
pub const WHvTranslateGvaFlagValidateRead: u32 = 1u32;
pub const WHvTranslateGvaFlagValidateWrite: u32 = 2u32;
pub const WHvTranslateGvaFlagValidateExecute: u32 = 4u32;
pub const WHvTranslateGvaFlagPrivilegeExempt: u32 = 8u32;
pub const WHvTranslateGvaFlagSetPageTableBits: u32 = 16u32;
pub const WHvTranslateGvaFlagEnforceSmap: u32 = 256u32;
pub const WHvTranslateGvaFlagOverrideSmap: u32 = 512u32;
#[repr(C)]
pub struct WHV_TRANSLATE_GVA_RESULT {
    pub ResultCode: WHV_TRANSLATE_GVA_RESULT_CODE,
    pub Reserved: u32,
}
impl ::core::marker::Copy for WHV_TRANSLATE_GVA_RESULT {}
impl ::core::clone::Clone for WHV_TRANSLATE_GVA_RESULT {
    fn clone(&self) -> Self {
        *self
    }
}
pub const WHvTranslateGvaResultSuccess: i32 = 0i32;
pub const WHvTranslateGvaResultPageNotPresent: i32 = 1i32;
pub const WHvTranslateGvaResultPrivilegeViolation: i32 = 2i32;
pub const WHvTranslateGvaResultInvalidPageTableFlags: i32 = 3i32;
pub const WHvTranslateGvaResultGpaUnmapped: i32 = 4i32;
pub const WHvTranslateGvaResultGpaNoReadAccess: i32 = 5i32;
pub const WHvTranslateGvaResultGpaNoWriteAccess: i32 = 6i32;
pub const WHvTranslateGvaResultGpaIllegalOverlayAccess: i32 = 7i32;
pub const WHvTranslateGvaResultIntercept: i32 = 8i32;
#[repr(C)]
pub struct WHV_TRIGGER_PARAMETERS {
    pub TriggerType: WHV_TRIGGER_TYPE,
    pub Reserved: u32,
    pub Anonymous: WHV_TRIGGER_PARAMETERS_0,
}
impl ::core::marker::Copy for WHV_TRIGGER_PARAMETERS {}
impl ::core::clone::Clone for WHV_TRIGGER_PARAMETERS {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub union WHV_TRIGGER_PARAMETERS_0 {
    pub Interrupt: WHV_INTERRUPT_CONTROL,
    pub SynicEvent: WHV_SYNIC_EVENT_PARAMETERS,
    pub DeviceInterrupt: WHV_TRIGGER_PARAMETERS_0_0,
}
impl ::core::marker::Copy for WHV_TRIGGER_PARAMETERS_0 {}
impl ::core::clone::Clone for WHV_TRIGGER_PARAMETERS_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct WHV_TRIGGER_PARAMETERS_0_0 {
    pub LogicalDeviceId: u64,
    pub MsiAddress: u64,
    pub MsiData: u32,
    pub Reserved: u32,
}
impl ::core::marker::Copy for WHV_TRIGGER_PARAMETERS_0_0 {}
impl ::core::clone::Clone for WHV_TRIGGER_PARAMETERS_0_0 {
    fn clone(&self) -> Self {
        *self
    }
}
pub const WHvTriggerTypeInterrupt: i32 = 0i32;
pub const WHvTriggerTypeSynicEvent: i32 = 1i32;
pub const WHvTriggerTypeDeviceInterrupt: i32 = 2i32;
#[repr(C)]
pub union WHV_UINT128 {
    pub Anonymous: WHV_UINT128_0,
    pub Dword: [u32; 4],
}
impl ::core::marker::Copy for WHV_UINT128 {}
impl ::core::clone::Clone for WHV_UINT128 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct WHV_UINT128_0 {
    pub Low64: u64,
    pub High64: u64,
}
impl ::core::marker::Copy for WHV_UINT128_0 {}
impl ::core::clone::Clone for WHV_UINT128_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct WHV_VIRTUAL_PROCESSOR_PROPERTY {
    pub PropertyCode: WHV_VIRTUAL_PROCESSOR_PROPERTY_CODE,
    pub Reserved: u32,
    pub Anonymous: WHV_VIRTUAL_PROCESSOR_PROPERTY_0,
}
impl ::core::marker::Copy for WHV_VIRTUAL_PROCESSOR_PROPERTY {}
impl ::core::clone::Clone for WHV_VIRTUAL_PROCESSOR_PROPERTY {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub union WHV_VIRTUAL_PROCESSOR_PROPERTY_0 {
    pub NumaNode: u16,
    pub Padding: u64,
}
impl ::core::marker::Copy for WHV_VIRTUAL_PROCESSOR_PROPERTY_0 {}
impl ::core::clone::Clone for WHV_VIRTUAL_PROCESSOR_PROPERTY_0 {
    fn clone(&self) -> Self {
        *self
    }
}
pub const WHvVirtualProcessorPropertyCodeNumaNode: i32 = 0i32;
pub const WHvVirtualProcessorStateTypeSynicMessagePage: i32 = 0i32;
pub const WHvVirtualProcessorStateTypeSynicEventFlagPage: i32 = 1i32;
pub const WHvVirtualProcessorStateTypeSynicTimerState: i32 = 2i32;
pub const WHvVirtualProcessorStateTypeInterruptControllerState2: i32 = 4096i32;
pub const WHvVirtualProcessorStateTypeXsaveState: i32 = 4097i32;
#[repr(C)]
pub struct WHV_VPCI_DEVICE_NOTIFICATION {
    pub NotificationType: WHV_VPCI_DEVICE_NOTIFICATION_TYPE,
    pub Reserved1: u32,
    pub Anonymous: WHV_VPCI_DEVICE_NOTIFICATION_0,
}
impl ::core::marker::Copy for WHV_VPCI_DEVICE_NOTIFICATION {}
impl ::core::clone::Clone for WHV_VPCI_DEVICE_NOTIFICATION {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub union WHV_VPCI_DEVICE_NOTIFICATION_0 {
    pub Reserved2: u64,
}
impl ::core::marker::Copy for WHV_VPCI_DEVICE_NOTIFICATION_0 {}
impl ::core::clone::Clone for WHV_VPCI_DEVICE_NOTIFICATION_0 {
    fn clone(&self) -> Self {
        *self
    }
}
pub const WHvVpciDeviceNotificationUndefined: i32 = 0i32;
pub const WHvVpciDeviceNotificationMmioRemapping: i32 = 1i32;
pub const WHvVpciDeviceNotificationSurpriseRemoval: i32 = 2i32;
pub const WHvVpciDevicePropertyCodeUndefined: i32 = 0i32;
pub const WHvVpciDevicePropertyCodeHardwareIDs: i32 = 1i32;
pub const WHvVpciDevicePropertyCodeProbedBARs: i32 = 2i32;
#[repr(C)]
pub struct WHV_VPCI_DEVICE_REGISTER {
    pub Location: WHV_VPCI_DEVICE_REGISTER_SPACE,
    pub SizeInBytes: u32,
    pub OffsetInBytes: u64,
}
impl ::core::marker::Copy for WHV_VPCI_DEVICE_REGISTER {}
impl ::core::clone::Clone for WHV_VPCI_DEVICE_REGISTER {
    fn clone(&self) -> Self {
        *self
    }
}
pub const WHvVpciConfigSpace: i32 = -1i32;
pub const WHvVpciBar0: i32 = 0i32;
pub const WHvVpciBar1: i32 = 1i32;
pub const WHvVpciBar2: i32 = 2i32;
pub const WHvVpciBar3: i32 = 3i32;
pub const WHvVpciBar4: i32 = 4i32;
pub const WHvVpciBar5: i32 = 5i32;
#[repr(C)]
pub struct WHV_VPCI_HARDWARE_IDS {
    pub VendorID: u16,
    pub DeviceID: u16,
    pub RevisionID: u8,
    pub ProgIf: u8,
    pub SubClass: u8,
    pub BaseClass: u8,
    pub SubVendorID: u16,
    pub SubSystemID: u16,
}
impl ::core::marker::Copy for WHV_VPCI_HARDWARE_IDS {}
impl ::core::clone::Clone for WHV_VPCI_HARDWARE_IDS {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct WHV_VPCI_INTERRUPT_TARGET {
    pub Vector: u32,
    pub Flags: WHV_VPCI_INTERRUPT_TARGET_FLAGS,
    pub ProcessorCount: u32,
    pub Processors: [u32; 1],
}
impl ::core::marker::Copy for WHV_VPCI_INTERRUPT_TARGET {}
impl ::core::clone::Clone for WHV_VPCI_INTERRUPT_TARGET {
    fn clone(&self) -> Self {
        *self
    }
}
pub const WHvVpciInterruptTargetFlagNone: u32 = 0u32;
pub const WHvVpciInterruptTargetFlagMulticast: u32 = 1u32;
#[repr(C)]
pub struct WHV_VPCI_MMIO_MAPPING {
    pub Location: WHV_VPCI_DEVICE_REGISTER_SPACE,
    pub Flags: WHV_VPCI_MMIO_RANGE_FLAGS,
    pub SizeInBytes: u64,
    pub OffsetInBytes: u64,
    pub VirtualAddress: *mut ::core::ffi::c_void,
}
impl ::core::marker::Copy for WHV_VPCI_MMIO_MAPPING {}
impl ::core::clone::Clone for WHV_VPCI_MMIO_MAPPING {
    fn clone(&self) -> Self {
        *self
    }
}
pub const WHvVpciMmioRangeFlagReadAccess: u32 = 1u32;
pub const WHvVpciMmioRangeFlagWriteAccess: u32 = 2u32;
#[repr(C)]
pub struct WHV_VPCI_PROBED_BARS {
    pub Value: [u32; 6],
}
impl ::core::marker::Copy for WHV_VPCI_PROBED_BARS {}
impl ::core::clone::Clone for WHV_VPCI_PROBED_BARS {
    fn clone(&self) -> Self {
        *self
    }
}
pub const WHV_VPCI_TYPE0_BAR_COUNT: u32 = 6u32;
#[repr(C)]
pub struct WHV_VP_EXCEPTION_CONTEXT {
    pub InstructionByteCount: u8,
    pub Reserved: [u8; 3],
    pub InstructionBytes: [u8; 16],
    pub ExceptionInfo: WHV_VP_EXCEPTION_INFO,
    pub ExceptionType: u8,
    pub Reserved2: [u8; 3],
    pub ErrorCode: u32,
    pub ExceptionParameter: u64,
}
impl ::core::marker::Copy for WHV_VP_EXCEPTION_CONTEXT {}
impl ::core::clone::Clone for WHV_VP_EXCEPTION_CONTEXT {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub union WHV_VP_EXCEPTION_INFO {
    pub Anonymous: WHV_VP_EXCEPTION_INFO_0,
    pub AsUINT32: u32,
}
impl ::core::marker::Copy for WHV_VP_EXCEPTION_INFO {}
impl ::core::clone::Clone for WHV_VP_EXCEPTION_INFO {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct WHV_VP_EXCEPTION_INFO_0 {
    pub _bitfield: u32,
}
impl ::core::marker::Copy for WHV_VP_EXCEPTION_INFO_0 {}
impl ::core::clone::Clone for WHV_VP_EXCEPTION_INFO_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct WHV_VP_EXIT_CONTEXT {
    pub ExecutionState: WHV_X64_VP_EXECUTION_STATE,
    pub _bitfield: u8,
    pub Reserved: u8,
    pub Reserved2: u32,
    pub Cs: WHV_X64_SEGMENT_REGISTER,
    pub Rip: u64,
    pub Rflags: u64,
}
impl ::core::marker::Copy for WHV_VP_EXIT_CONTEXT {}
impl ::core::clone::Clone for WHV_VP_EXIT_CONTEXT {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct WHV_X64_APIC_EOI_CONTEXT {
    pub InterruptVector: u32,
}
impl ::core::marker::Copy for WHV_X64_APIC_EOI_CONTEXT {}
impl ::core::clone::Clone for WHV_X64_APIC_EOI_CONTEXT {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct WHV_X64_APIC_INIT_SIPI_CONTEXT {
    pub ApicIcr: u64,
}
impl ::core::marker::Copy for WHV_X64_APIC_INIT_SIPI_CONTEXT {}
impl ::core::clone::Clone for WHV_X64_APIC_INIT_SIPI_CONTEXT {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct WHV_X64_APIC_SMI_CONTEXT {
    pub ApicIcr: u64,
}
impl ::core::marker::Copy for WHV_X64_APIC_SMI_CONTEXT {}
impl ::core::clone::Clone for WHV_X64_APIC_SMI_CONTEXT {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct WHV_X64_APIC_WRITE_CONTEXT {
    pub Type: WHV_X64_APIC_WRITE_TYPE,
    pub Reserved: u32,
    pub WriteValue: u64,
}
impl ::core::marker::Copy for WHV_X64_APIC_WRITE_CONTEXT {}
impl ::core::clone::Clone for WHV_X64_APIC_WRITE_CONTEXT {
    fn clone(&self) -> Self {
        *self
    }
}
pub const WHvX64ApicWriteTypeLdr: i32 = 208i32;
pub const WHvX64ApicWriteTypeDfr: i32 = 224i32;
pub const WHvX64ApicWriteTypeSvr: i32 = 240i32;
pub const WHvX64ApicWriteTypeLint0: i32 = 848i32;
pub const WHvX64ApicWriteTypeLint1: i32 = 864i32;
#[repr(C)]
pub struct WHV_X64_CPUID_ACCESS_CONTEXT {
    pub Rax: u64,
    pub Rcx: u64,
    pub Rdx: u64,
    pub Rbx: u64,
    pub DefaultResultRax: u64,
    pub DefaultResultRcx: u64,
    pub DefaultResultRdx: u64,
    pub DefaultResultRbx: u64,
}
impl ::core::marker::Copy for WHV_X64_CPUID_ACCESS_CONTEXT {}
impl ::core::clone::Clone for WHV_X64_CPUID_ACCESS_CONTEXT {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct WHV_X64_CPUID_RESULT {
    pub Function: u32,
    pub Reserved: [u32; 3],
    pub Eax: u32,
    pub Ebx: u32,
    pub Ecx: u32,
    pub Edx: u32,
}
impl ::core::marker::Copy for WHV_X64_CPUID_RESULT {}
impl ::core::clone::Clone for WHV_X64_CPUID_RESULT {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct WHV_X64_CPUID_RESULT2 {
    pub Function: u32,
    pub Index: u32,
    pub VpIndex: u32,
    pub Flags: WHV_X64_CPUID_RESULT2_FLAGS,
    pub Output: WHV_CPUID_OUTPUT,
    pub Mask: WHV_CPUID_OUTPUT,
}
impl ::core::marker::Copy for WHV_X64_CPUID_RESULT2 {}
impl ::core::clone::Clone for WHV_X64_CPUID_RESULT2 {
    fn clone(&self) -> Self {
        *self
    }
}
pub const WHvX64CpuidResult2FlagSubleafSpecific: u32 = 1u32;
pub const WHvX64CpuidResult2FlagVpSpecific: u32 = 2u32;
#[repr(C)]
pub union WHV_X64_DELIVERABILITY_NOTIFICATIONS_REGISTER {
    pub Anonymous: WHV_X64_DELIVERABILITY_NOTIFICATIONS_REGISTER_0,
    pub AsUINT64: u64,
}
impl ::core::marker::Copy for WHV_X64_DELIVERABILITY_NOTIFICATIONS_REGISTER {}
impl ::core::clone::Clone for WHV_X64_DELIVERABILITY_NOTIFICATIONS_REGISTER {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct WHV_X64_DELIVERABILITY_NOTIFICATIONS_REGISTER_0 {
    pub _bitfield: u64,
}
impl ::core::marker::Copy for WHV_X64_DELIVERABILITY_NOTIFICATIONS_REGISTER_0 {}
impl ::core::clone::Clone for WHV_X64_DELIVERABILITY_NOTIFICATIONS_REGISTER_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub union WHV_X64_FP_CONTROL_STATUS_REGISTER {
    pub Anonymous: WHV_X64_FP_CONTROL_STATUS_REGISTER_0,
    pub AsUINT128: WHV_UINT128,
}
impl ::core::marker::Copy for WHV_X64_FP_CONTROL_STATUS_REGISTER {}
impl ::core::clone::Clone for WHV_X64_FP_CONTROL_STATUS_REGISTER {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct WHV_X64_FP_CONTROL_STATUS_REGISTER_0 {
    pub FpControl: u16,
    pub FpStatus: u16,
    pub FpTag: u8,
    pub Reserved: u8,
    pub LastFpOp: u16,
    pub Anonymous: WHV_X64_FP_CONTROL_STATUS_REGISTER_0_0,
}
impl ::core::marker::Copy for WHV_X64_FP_CONTROL_STATUS_REGISTER_0 {}
impl ::core::clone::Clone for WHV_X64_FP_CONTROL_STATUS_REGISTER_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub union WHV_X64_FP_CONTROL_STATUS_REGISTER_0_0 {
    pub LastFpRip: u64,
    pub Anonymous: WHV_X64_FP_CONTROL_STATUS_REGISTER_0_0_0,
}
impl ::core::marker::Copy for WHV_X64_FP_CONTROL_STATUS_REGISTER_0_0 {}
impl ::core::clone::Clone for WHV_X64_FP_CONTROL_STATUS_REGISTER_0_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct WHV_X64_FP_CONTROL_STATUS_REGISTER_0_0_0 {
    pub LastFpEip: u32,
    pub LastFpCs: u16,
    pub Reserved2: u16,
}
impl ::core::marker::Copy for WHV_X64_FP_CONTROL_STATUS_REGISTER_0_0_0 {}
impl ::core::clone::Clone for WHV_X64_FP_CONTROL_STATUS_REGISTER_0_0_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub union WHV_X64_FP_REGISTER {
    pub Anonymous: WHV_X64_FP_REGISTER_0,
    pub AsUINT128: WHV_UINT128,
}
impl ::core::marker::Copy for WHV_X64_FP_REGISTER {}
impl ::core::clone::Clone for WHV_X64_FP_REGISTER {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct WHV_X64_FP_REGISTER_0 {
    pub Mantissa: u64,
    pub _bitfield: u64,
}
impl ::core::marker::Copy for WHV_X64_FP_REGISTER_0 {}
impl ::core::clone::Clone for WHV_X64_FP_REGISTER_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct WHV_X64_INTERRUPTION_DELIVERABLE_CONTEXT {
    pub DeliverableType: WHV_X64_PENDING_INTERRUPTION_TYPE,
}
impl ::core::marker::Copy for WHV_X64_INTERRUPTION_DELIVERABLE_CONTEXT {}
impl ::core::clone::Clone for WHV_X64_INTERRUPTION_DELIVERABLE_CONTEXT {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub union WHV_X64_INTERRUPT_STATE_REGISTER {
    pub Anonymous: WHV_X64_INTERRUPT_STATE_REGISTER_0,
    pub AsUINT64: u64,
}
impl ::core::marker::Copy for WHV_X64_INTERRUPT_STATE_REGISTER {}
impl ::core::clone::Clone for WHV_X64_INTERRUPT_STATE_REGISTER {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct WHV_X64_INTERRUPT_STATE_REGISTER_0 {
    pub _bitfield: u64,
}
impl ::core::marker::Copy for WHV_X64_INTERRUPT_STATE_REGISTER_0 {}
impl ::core::clone::Clone for WHV_X64_INTERRUPT_STATE_REGISTER_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct WHV_X64_IO_PORT_ACCESS_CONTEXT {
    pub InstructionByteCount: u8,
    pub Reserved: [u8; 3],
    pub InstructionBytes: [u8; 16],
    pub AccessInfo: WHV_X64_IO_PORT_ACCESS_INFO,
    pub PortNumber: u16,
    pub Reserved2: [u16; 3],
    pub Rax: u64,
    pub Rcx: u64,
    pub Rsi: u64,
    pub Rdi: u64,
    pub Ds: WHV_X64_SEGMENT_REGISTER,
    pub Es: WHV_X64_SEGMENT_REGISTER,
}
impl ::core::marker::Copy for WHV_X64_IO_PORT_ACCESS_CONTEXT {}
impl ::core::clone::Clone for WHV_X64_IO_PORT_ACCESS_CONTEXT {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub union WHV_X64_IO_PORT_ACCESS_INFO {
    pub Anonymous: WHV_X64_IO_PORT_ACCESS_INFO_0,
    pub AsUINT32: u32,
}
impl ::core::marker::Copy for WHV_X64_IO_PORT_ACCESS_INFO {}
impl ::core::clone::Clone for WHV_X64_IO_PORT_ACCESS_INFO {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct WHV_X64_IO_PORT_ACCESS_INFO_0 {
    pub _bitfield: u32,
}
impl ::core::marker::Copy for WHV_X64_IO_PORT_ACCESS_INFO_0 {}
impl ::core::clone::Clone for WHV_X64_IO_PORT_ACCESS_INFO_0 {
    fn clone(&self) -> Self {
        *self
    }
}
pub const WHvX64LocalApicEmulationModeNone: i32 = 0i32;
pub const WHvX64LocalApicEmulationModeXApic: i32 = 1i32;
pub const WHvX64LocalApicEmulationModeX2Apic: i32 = 2i32;
#[repr(C)]
pub struct WHV_X64_MSR_ACCESS_CONTEXT {
    pub AccessInfo: WHV_X64_MSR_ACCESS_INFO,
    pub MsrNumber: u32,
    pub Rax: u64,
    pub Rdx: u64,
}
impl ::core::marker::Copy for WHV_X64_MSR_ACCESS_CONTEXT {}
impl ::core::clone::Clone for WHV_X64_MSR_ACCESS_CONTEXT {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub union WHV_X64_MSR_ACCESS_INFO {
    pub Anonymous: WHV_X64_MSR_ACCESS_INFO_0,
    pub AsUINT32: u32,
}
impl ::core::marker::Copy for WHV_X64_MSR_ACCESS_INFO {}
impl ::core::clone::Clone for WHV_X64_MSR_ACCESS_INFO {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct WHV_X64_MSR_ACCESS_INFO_0 {
    pub _bitfield: u32,
}
impl ::core::marker::Copy for WHV_X64_MSR_ACCESS_INFO_0 {}
impl ::core::clone::Clone for WHV_X64_MSR_ACCESS_INFO_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub union WHV_X64_MSR_EXIT_BITMAP {
    pub AsUINT64: u64,
    pub Anonymous: WHV_X64_MSR_EXIT_BITMAP_0,
}
impl ::core::marker::Copy for WHV_X64_MSR_EXIT_BITMAP {}
impl ::core::clone::Clone for WHV_X64_MSR_EXIT_BITMAP {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct WHV_X64_MSR_EXIT_BITMAP_0 {
    pub _bitfield: u64,
}
impl ::core::marker::Copy for WHV_X64_MSR_EXIT_BITMAP_0 {}
impl ::core::clone::Clone for WHV_X64_MSR_EXIT_BITMAP_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub union WHV_X64_PENDING_DEBUG_EXCEPTION {
    pub AsUINT64: u64,
    pub Anonymous: WHV_X64_PENDING_DEBUG_EXCEPTION_0,
}
impl ::core::marker::Copy for WHV_X64_PENDING_DEBUG_EXCEPTION {}
impl ::core::clone::Clone for WHV_X64_PENDING_DEBUG_EXCEPTION {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct WHV_X64_PENDING_DEBUG_EXCEPTION_0 {
    pub _bitfield: u64,
}
impl ::core::marker::Copy for WHV_X64_PENDING_DEBUG_EXCEPTION_0 {}
impl ::core::clone::Clone for WHV_X64_PENDING_DEBUG_EXCEPTION_0 {
    fn clone(&self) -> Self {
        *self
    }
}
pub const WHvX64PendingEventException: i32 = 0i32;
pub const WHvX64PendingEventExtInt: i32 = 5i32;
#[repr(C)]
pub union WHV_X64_PENDING_EXCEPTION_EVENT {
    pub Anonymous: WHV_X64_PENDING_EXCEPTION_EVENT_0,
    pub AsUINT128: WHV_UINT128,
}
impl ::core::marker::Copy for WHV_X64_PENDING_EXCEPTION_EVENT {}
impl ::core::clone::Clone for WHV_X64_PENDING_EXCEPTION_EVENT {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct WHV_X64_PENDING_EXCEPTION_EVENT_0 {
    pub _bitfield: u32,
    pub ErrorCode: u32,
    pub ExceptionParameter: u64,
}
impl ::core::marker::Copy for WHV_X64_PENDING_EXCEPTION_EVENT_0 {}
impl ::core::clone::Clone for WHV_X64_PENDING_EXCEPTION_EVENT_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub union WHV_X64_PENDING_EXT_INT_EVENT {
    pub Anonymous: WHV_X64_PENDING_EXT_INT_EVENT_0,
    pub AsUINT128: WHV_UINT128,
}
impl ::core::marker::Copy for WHV_X64_PENDING_EXT_INT_EVENT {}
impl ::core::clone::Clone for WHV_X64_PENDING_EXT_INT_EVENT {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct WHV_X64_PENDING_EXT_INT_EVENT_0 {
    pub _bitfield: u64,
    pub Reserved2: u64,
}
impl ::core::marker::Copy for WHV_X64_PENDING_EXT_INT_EVENT_0 {}
impl ::core::clone::Clone for WHV_X64_PENDING_EXT_INT_EVENT_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub union WHV_X64_PENDING_INTERRUPTION_REGISTER {
    pub Anonymous: WHV_X64_PENDING_INTERRUPTION_REGISTER_0,
    pub AsUINT64: u64,
}
impl ::core::marker::Copy for WHV_X64_PENDING_INTERRUPTION_REGISTER {}
impl ::core::clone::Clone for WHV_X64_PENDING_INTERRUPTION_REGISTER {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct WHV_X64_PENDING_INTERRUPTION_REGISTER_0 {
    pub _bitfield: u32,
    pub ErrorCode: u32,
}
impl ::core::marker::Copy for WHV_X64_PENDING_INTERRUPTION_REGISTER_0 {}
impl ::core::clone::Clone for WHV_X64_PENDING_INTERRUPTION_REGISTER_0 {
    fn clone(&self) -> Self {
        *self
    }
}
pub const WHvX64PendingInterrupt: i32 = 0i32;
pub const WHvX64PendingNmi: i32 = 2i32;
pub const WHvX64PendingException: i32 = 3i32;
#[repr(C)]
pub struct WHV_X64_RDTSC_CONTEXT {
    pub TscAux: u64,
    pub VirtualOffset: u64,
    pub Tsc: u64,
    pub ReferenceTime: u64,
    pub RdtscInfo: WHV_X64_RDTSC_INFO,
}
impl ::core::marker::Copy for WHV_X64_RDTSC_CONTEXT {}
impl ::core::clone::Clone for WHV_X64_RDTSC_CONTEXT {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub union WHV_X64_RDTSC_INFO {
    pub Anonymous: WHV_X64_RDTSC_INFO_0,
    pub AsUINT64: u64,
}
impl ::core::marker::Copy for WHV_X64_RDTSC_INFO {}
impl ::core::clone::Clone for WHV_X64_RDTSC_INFO {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct WHV_X64_RDTSC_INFO_0 {
    pub _bitfield: u64,
}
impl ::core::marker::Copy for WHV_X64_RDTSC_INFO_0 {}
impl ::core::clone::Clone for WHV_X64_RDTSC_INFO_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct WHV_X64_SEGMENT_REGISTER {
    pub Base: u64,
    pub Limit: u32,
    pub Selector: u16,
    pub Anonymous: WHV_X64_SEGMENT_REGISTER_0,
}
impl ::core::marker::Copy for WHV_X64_SEGMENT_REGISTER {}
impl ::core::clone::Clone for WHV_X64_SEGMENT_REGISTER {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub union WHV_X64_SEGMENT_REGISTER_0 {
    pub Anonymous: WHV_X64_SEGMENT_REGISTER_0_0,
    pub Attributes: u16,
}
impl ::core::marker::Copy for WHV_X64_SEGMENT_REGISTER_0 {}
impl ::core::clone::Clone for WHV_X64_SEGMENT_REGISTER_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct WHV_X64_SEGMENT_REGISTER_0_0 {
    pub _bitfield: u16,
}
impl ::core::marker::Copy for WHV_X64_SEGMENT_REGISTER_0_0 {}
impl ::core::clone::Clone for WHV_X64_SEGMENT_REGISTER_0_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct WHV_X64_TABLE_REGISTER {
    pub Pad: [u16; 3],
    pub Limit: u16,
    pub Base: u64,
}
impl ::core::marker::Copy for WHV_X64_TABLE_REGISTER {}
impl ::core::clone::Clone for WHV_X64_TABLE_REGISTER {
    fn clone(&self) -> Self {
        *self
    }
}
pub const WHvUnsupportedFeatureIntercept: i32 = 1i32;
pub const WHvUnsupportedFeatureTaskSwitchTss: i32 = 2i32;
#[repr(C)]
pub struct WHV_X64_UNSUPPORTED_FEATURE_CONTEXT {
    pub FeatureCode: WHV_X64_UNSUPPORTED_FEATURE_CODE,
    pub Reserved: u32,
    pub FeatureParameter: u64,
}
impl ::core::marker::Copy for WHV_X64_UNSUPPORTED_FEATURE_CONTEXT {}
impl ::core::clone::Clone for WHV_X64_UNSUPPORTED_FEATURE_CONTEXT {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub union WHV_X64_VP_EXECUTION_STATE {
    pub Anonymous: WHV_X64_VP_EXECUTION_STATE_0,
    pub AsUINT16: u16,
}
impl ::core::marker::Copy for WHV_X64_VP_EXECUTION_STATE {}
impl ::core::clone::Clone for WHV_X64_VP_EXECUTION_STATE {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct WHV_X64_VP_EXECUTION_STATE_0 {
    pub _bitfield: u16,
}
impl ::core::marker::Copy for WHV_X64_VP_EXECUTION_STATE_0 {}
impl ::core::clone::Clone for WHV_X64_VP_EXECUTION_STATE_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub union WHV_X64_XMM_CONTROL_STATUS_REGISTER {
    pub Anonymous: WHV_X64_XMM_CONTROL_STATUS_REGISTER_0,
    pub AsUINT128: WHV_UINT128,
}
impl ::core::marker::Copy for WHV_X64_XMM_CONTROL_STATUS_REGISTER {}
impl ::core::clone::Clone for WHV_X64_XMM_CONTROL_STATUS_REGISTER {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct WHV_X64_XMM_CONTROL_STATUS_REGISTER_0 {
    pub Anonymous: WHV_X64_XMM_CONTROL_STATUS_REGISTER_0_0,
    pub XmmStatusControl: u32,
    pub XmmStatusControlMask: u32,
}
impl ::core::marker::Copy for WHV_X64_XMM_CONTROL_STATUS_REGISTER_0 {}
impl ::core::clone::Clone for WHV_X64_XMM_CONTROL_STATUS_REGISTER_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub union WHV_X64_XMM_CONTROL_STATUS_REGISTER_0_0 {
    pub LastFpRdp: u64,
    pub Anonymous: WHV_X64_XMM_CONTROL_STATUS_REGISTER_0_0_0,
}
impl ::core::marker::Copy for WHV_X64_XMM_CONTROL_STATUS_REGISTER_0_0 {}
impl ::core::clone::Clone for WHV_X64_XMM_CONTROL_STATUS_REGISTER_0_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct WHV_X64_XMM_CONTROL_STATUS_REGISTER_0_0_0 {
    pub LastFpDp: u32,
    pub LastFpDs: u16,
    pub Reserved: u16,
}
impl ::core::marker::Copy for WHV_X64_XMM_CONTROL_STATUS_REGISTER_0_0_0 {}
impl ::core::clone::Clone for WHV_X64_XMM_CONTROL_STATUS_REGISTER_0_0_0 {
    fn clone(&self) -> Self {
        *self
    }
}

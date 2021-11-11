#![allow(non_snake_case, non_camel_case_types)]
#[link(name = "windows")]
extern "system" {
    #[doc = "*Required features: `Win32_System_Hypervisor`*"]
    pub fn ApplyGuestMemoryFix(vmsavedstatedumphandle: *mut ::core::ffi::c_void, vpid: u32, virtualaddress: u64, fixbuffer: *const ::core::ffi::c_void, fixbuffersize: u32) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_System_Hypervisor`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ApplyPendingSavedStateFileReplayLog(vmrsfile: super::super::Foundation::PWSTR) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_System_Hypervisor`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CallStackUnwind(vmsavedstatedumphandle: *mut ::core::ffi::c_void, vpid: u32, imageinfo: *const MODULE_INFO, imageinfocount: u32, framecount: u32, callstack: *mut super::super::Foundation::PWSTR) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_System_Hypervisor`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn FindSavedStateSymbolFieldInType(vmsavedstatedumphandle: *mut ::core::ffi::c_void, vpid: u32, typename: super::super::Foundation::PSTR, fieldname: super::super::Foundation::PWSTR, offset: *mut u32, found: *mut super::super::Foundation::BOOL) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_System_Hypervisor`*"]
    pub fn ForceActiveVirtualTrustLevel(vmsavedstatedumphandle: *mut ::core::ffi::c_void, vpid: u32, virtualtrustlevel: u8) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_System_Hypervisor`*"]
    pub fn ForceArchitecture(vmsavedstatedumphandle: *mut ::core::ffi::c_void, vpid: u32, architecture: VIRTUAL_PROCESSOR_ARCH) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_System_Hypervisor`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ForceNestedHostMode(vmsavedstatedumphandle: *mut ::core::ffi::c_void, vpid: u32, hostmode: super::super::Foundation::BOOL, oldmode: *mut super::super::Foundation::BOOL) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_System_Hypervisor`*"]
    pub fn ForcePagingMode(vmsavedstatedumphandle: *mut ::core::ffi::c_void, vpid: u32, pagingmode: PAGING_MODE) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_System_Hypervisor`*"]
    pub fn GetActiveVirtualTrustLevel(vmsavedstatedumphandle: *mut ::core::ffi::c_void, vpid: u32, virtualtrustlevel: *mut u8) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_System_Hypervisor`*"]
    pub fn GetArchitecture(vmsavedstatedumphandle: *mut ::core::ffi::c_void, vpid: u32, architecture: *mut VIRTUAL_PROCESSOR_ARCH) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_System_Hypervisor`*"]
    pub fn GetEnabledVirtualTrustLevels(vmsavedstatedumphandle: *mut ::core::ffi::c_void, vpid: u32, virtualtrustlevels: *mut u32) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_System_Hypervisor`*"]
    pub fn GetGuestEnabledVirtualTrustLevels(vmsavedstatedumphandle: *mut ::core::ffi::c_void, virtualtrustlevels: *mut u32) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_System_Hypervisor`*"]
    pub fn GetGuestOsInfo(vmsavedstatedumphandle: *mut ::core::ffi::c_void, virtualtrustlevel: u8, guestosinfo: *mut GUEST_OS_INFO) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_System_Hypervisor`*"]
    pub fn GetGuestPhysicalMemoryChunks(vmsavedstatedumphandle: *mut ::core::ffi::c_void, memorychunkpagesize: *mut u64, memorychunks: *mut GPA_MEMORY_CHUNK, memorychunkcount: *mut u64) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_System_Hypervisor`*"]
    pub fn GetGuestRawSavedMemorySize(vmsavedstatedumphandle: *mut ::core::ffi::c_void, guestrawsavedmemorysize: *mut u64) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_System_Hypervisor`*"]
    pub fn GetMemoryBlockCacheLimit(vmsavedstatedumphandle: *mut ::core::ffi::c_void, memoryblockcachelimit: *mut u64) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_System_Hypervisor`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetNestedVirtualizationMode(vmsavedstatedumphandle: *mut ::core::ffi::c_void, vpid: u32, enabled: *mut super::super::Foundation::BOOL) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_System_Hypervisor`*"]
    pub fn GetPagingMode(vmsavedstatedumphandle: *mut ::core::ffi::c_void, vpid: u32, pagingmode: *mut PAGING_MODE) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_System_Hypervisor`*"]
    pub fn GetRegisterValue(vmsavedstatedumphandle: *mut ::core::ffi::c_void, vpid: u32, registerid: u32, registervalue: *mut VIRTUAL_PROCESSOR_REGISTER) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_System_Hypervisor`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetSavedStateSymbolFieldInfo(vmsavedstatedumphandle: *mut ::core::ffi::c_void, vpid: u32, typename: super::super::Foundation::PSTR, typefieldinfomap: *mut super::super::Foundation::PWSTR) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_System_Hypervisor`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetSavedStateSymbolProviderHandle(vmsavedstatedumphandle: *mut ::core::ffi::c_void) -> super::super::Foundation::HANDLE;
    #[doc = "*Required features: `Win32_System_Hypervisor`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetSavedStateSymbolTypeSize(vmsavedstatedumphandle: *mut ::core::ffi::c_void, vpid: u32, typename: super::super::Foundation::PSTR, size: *mut u32) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_System_Hypervisor`*"]
    pub fn GetVpCount(vmsavedstatedumphandle: *mut ::core::ffi::c_void, vpcount: *mut u32) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_System_Hypervisor`*"]
    pub fn GuestPhysicalAddressToRawSavedMemoryOffset(vmsavedstatedumphandle: *mut ::core::ffi::c_void, physicaladdress: u64, rawsavedmemoryoffset: *mut u64) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_System_Hypervisor`*"]
    pub fn GuestVirtualAddressToPhysicalAddress(vmsavedstatedumphandle: *mut ::core::ffi::c_void, vpid: u32, virtualaddress: u64, physicaladdress: *mut u64, unmappedregionsize: *mut u64) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_System_Hypervisor`*"]
    pub fn HdvCreateDeviceInstance(devicehosthandle: *const ::core::ffi::c_void, devicetype: HDV_DEVICE_TYPE, deviceclassid: *const ::windows::runtime::GUID, deviceinstanceid: *const ::windows::runtime::GUID, deviceinterface: *const ::core::ffi::c_void, devicecontext: *const ::core::ffi::c_void, devicehandle: *mut *mut ::core::ffi::c_void) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_System_Hypervisor`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn HdvCreateGuestMemoryAperture(requestor: *const ::core::ffi::c_void, guestphysicaladdress: u64, bytecount: u32, writeprotected: super::super::Foundation::BOOL, mappedaddress: *mut *mut ::core::ffi::c_void) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_System_Hypervisor`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn HdvCreateSectionBackedMmioRange(requestor: *const ::core::ffi::c_void, barindex: HDV_PCI_BAR_SELECTOR, offsetinpages: u64, lengthinpages: u64, mappingflags: HDV_MMIO_MAPPING_FLAGS, sectionhandle: super::super::Foundation::HANDLE, sectionoffsetinpages: u64) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_System_Hypervisor`*"]
    pub fn HdvDeliverGuestInterrupt(requestor: *const ::core::ffi::c_void, msiaddress: u64, msidata: u32) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_System_Hypervisor`*"]
    pub fn HdvDestroyGuestMemoryAperture(requestor: *const ::core::ffi::c_void, mappedaddress: *const ::core::ffi::c_void) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_System_Hypervisor`*"]
    pub fn HdvDestroySectionBackedMmioRange(requestor: *const ::core::ffi::c_void, barindex: HDV_PCI_BAR_SELECTOR, offsetinpages: u64) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_System_Hypervisor`, `Win32_System_HostComputeSystem`*"]
    #[cfg(feature = "Win32_System_HostComputeSystem")]
    pub fn HdvInitializeDeviceHost(computesystem: super::HostComputeSystem::HCS_SYSTEM, devicehosthandle: *mut *mut ::core::ffi::c_void) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_System_Hypervisor`*"]
    pub fn HdvReadGuestMemory(requestor: *const ::core::ffi::c_void, guestphysicaladdress: u64, bytecount: u32, buffer: *mut u8) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_System_Hypervisor`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn HdvRegisterDoorbell(requestor: *const ::core::ffi::c_void, barindex: HDV_PCI_BAR_SELECTOR, baroffset: u64, triggervalue: u64, flags: u64, doorbellevent: super::super::Foundation::HANDLE) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_System_Hypervisor`*"]
    pub fn HdvTeardownDeviceHost(devicehosthandle: *const ::core::ffi::c_void) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_System_Hypervisor`*"]
    pub fn HdvUnregisterDoorbell(requestor: *const ::core::ffi::c_void, barindex: HDV_PCI_BAR_SELECTOR, baroffset: u64, triggervalue: u64, flags: u64) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_System_Hypervisor`*"]
    pub fn HdvWriteGuestMemory(requestor: *const ::core::ffi::c_void, guestphysicaladdress: u64, bytecount: u32, buffer: *const u8) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_System_Hypervisor`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn InKernelSpace(vmsavedstatedumphandle: *mut ::core::ffi::c_void, vpid: u32, inkernelspace: *mut super::super::Foundation::BOOL) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_System_Hypervisor`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn IsActiveVirtualTrustLevelEnabled(vmsavedstatedumphandle: *mut ::core::ffi::c_void, vpid: u32, activevirtualtrustlevelenabled: *mut super::super::Foundation::BOOL) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_System_Hypervisor`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn IsNestedVirtualizationEnabled(vmsavedstatedumphandle: *mut ::core::ffi::c_void, enabled: *mut super::super::Foundation::BOOL) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_System_Hypervisor`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn LoadSavedStateFile(vmrsfile: super::super::Foundation::PWSTR, vmsavedstatedumphandle: *mut *mut ::core::ffi::c_void) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_System_Hypervisor`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn LoadSavedStateFiles(binfile: super::super::Foundation::PWSTR, vsvfile: super::super::Foundation::PWSTR, vmsavedstatedumphandle: *mut *mut ::core::ffi::c_void) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_System_Hypervisor`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn LoadSavedStateModuleSymbols(vmsavedstatedumphandle: *mut ::core::ffi::c_void, imagename: super::super::Foundation::PSTR, modulename: super::super::Foundation::PSTR, baseaddress: u64, sizeofbase: u32) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_System_Hypervisor`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn LoadSavedStateModuleSymbolsEx(vmsavedstatedumphandle: *mut ::core::ffi::c_void, imagename: super::super::Foundation::PSTR, imagetimestamp: u32, modulename: super::super::Foundation::PSTR, baseaddress: u64, sizeofbase: u32) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_System_Hypervisor`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn LoadSavedStateSymbolProvider(vmsavedstatedumphandle: *mut ::core::ffi::c_void, usersymbols: super::super::Foundation::PWSTR, force: super::super::Foundation::BOOL) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_System_Hypervisor`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn LocateSavedStateFiles(vmname: super::super::Foundation::PWSTR, snapshotname: super::super::Foundation::PWSTR, binpath: *mut super::super::Foundation::PWSTR, vsvpath: *mut super::super::Foundation::PWSTR, vmrspath: *mut super::super::Foundation::PWSTR) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_System_Hypervisor`*"]
    pub fn ReadGuestPhysicalAddress(vmsavedstatedumphandle: *mut ::core::ffi::c_void, physicaladdress: u64, buffer: *mut ::core::ffi::c_void, buffersize: u32, bytesread: *mut u32) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_System_Hypervisor`*"]
    pub fn ReadGuestRawSavedMemory(vmsavedstatedumphandle: *mut ::core::ffi::c_void, rawsavedmemoryoffset: u64, buffer: *mut ::core::ffi::c_void, buffersize: u32, bytesread: *mut u32) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_System_Hypervisor`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ReadSavedStateGlobalVariable(vmsavedstatedumphandle: *mut ::core::ffi::c_void, vpid: u32, globalname: super::super::Foundation::PSTR, buffer: *mut ::core::ffi::c_void, buffersize: u32) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_System_Hypervisor`*"]
    pub fn ReleaseSavedStateFiles(vmsavedstatedumphandle: *mut ::core::ffi::c_void) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_System_Hypervisor`*"]
    pub fn ReleaseSavedStateSymbolProvider(vmsavedstatedumphandle: *mut ::core::ffi::c_void) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_System_Hypervisor`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ResolveSavedStateGlobalVariableAddress(vmsavedstatedumphandle: *mut ::core::ffi::c_void, vpid: u32, globalname: super::super::Foundation::PSTR, virtualaddress: *mut u64, size: *mut u32) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_System_Hypervisor`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ScanMemoryForDosImages(vmsavedstatedumphandle: *mut ::core::ffi::c_void, vpid: u32, startaddress: u64, endaddress: u64, callbackcontext: *mut ::core::ffi::c_void, foundimagecallback: ::windows::runtime::RawPtr, standaloneaddress: *const u64, standaloneaddresscount: u32) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_System_Hypervisor`*"]
    pub fn SetMemoryBlockCacheLimit(vmsavedstatedumphandle: *mut ::core::ffi::c_void, memoryblockcachelimit: u64) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_System_Hypervisor`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetSavedStateSymbolProviderDebugInfoCallback(vmsavedstatedumphandle: *mut ::core::ffi::c_void, callback: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_System_Hypervisor`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WHvAcceptPartitionMigration(migrationhandle: super::super::Foundation::HANDLE, partition: *mut WHV_PARTITION_HANDLE) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_System_Hypervisor`*"]
    pub fn WHvAdviseGpaRange(partition: WHV_PARTITION_HANDLE, gparanges: *const WHV_MEMORY_RANGE_ENTRY, gparangescount: u32, advice: WHV_ADVISE_GPA_RANGE_CODE, advicebuffer: *const ::core::ffi::c_void, advicebuffersizeinbytes: u32) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_System_Hypervisor`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WHvAllocateVpciResource(providerid: *const ::windows::runtime::GUID, flags: WHV_ALLOCATE_VPCI_RESOURCE_FLAGS, resourcedescriptor: *const ::core::ffi::c_void, resourcedescriptorsizeinbytes: u32, vpciresource: *mut super::super::Foundation::HANDLE) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_System_Hypervisor`*"]
    pub fn WHvCancelPartitionMigration(partition: WHV_PARTITION_HANDLE) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_System_Hypervisor`*"]
    pub fn WHvCancelRunVirtualProcessor(partition: WHV_PARTITION_HANDLE, vpindex: u32, flags: u32) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_System_Hypervisor`*"]
    pub fn WHvCompletePartitionMigration(partition: WHV_PARTITION_HANDLE) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_System_Hypervisor`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WHvCreateNotificationPort(partition: WHV_PARTITION_HANDLE, parameters: *const WHV_NOTIFICATION_PORT_PARAMETERS, eventhandle: super::super::Foundation::HANDLE, porthandle: *mut *mut ::core::ffi::c_void) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_System_Hypervisor`*"]
    pub fn WHvCreatePartition(partition: *mut WHV_PARTITION_HANDLE) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_System_Hypervisor`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WHvCreateTrigger(partition: WHV_PARTITION_HANDLE, parameters: *const WHV_TRIGGER_PARAMETERS, triggerhandle: *mut *mut ::core::ffi::c_void, eventhandle: *mut super::super::Foundation::HANDLE) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_System_Hypervisor`*"]
    pub fn WHvCreateVirtualProcessor(partition: WHV_PARTITION_HANDLE, vpindex: u32, flags: u32) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_System_Hypervisor`*"]
    pub fn WHvCreateVirtualProcessor2(partition: WHV_PARTITION_HANDLE, vpindex: u32, properties: *const WHV_VIRTUAL_PROCESSOR_PROPERTY, propertycount: u32) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_System_Hypervisor`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WHvCreateVpciDevice(partition: WHV_PARTITION_HANDLE, logicaldeviceid: u64, vpciresource: super::super::Foundation::HANDLE, flags: WHV_CREATE_VPCI_DEVICE_FLAGS, notificationeventhandle: super::super::Foundation::HANDLE) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_System_Hypervisor`*"]
    pub fn WHvDeleteNotificationPort(partition: WHV_PARTITION_HANDLE, porthandle: *const ::core::ffi::c_void) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_System_Hypervisor`*"]
    pub fn WHvDeletePartition(partition: WHV_PARTITION_HANDLE) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_System_Hypervisor`*"]
    pub fn WHvDeleteTrigger(partition: WHV_PARTITION_HANDLE, triggerhandle: *const ::core::ffi::c_void) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_System_Hypervisor`*"]
    pub fn WHvDeleteVirtualProcessor(partition: WHV_PARTITION_HANDLE, vpindex: u32) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_System_Hypervisor`*"]
    pub fn WHvDeleteVpciDevice(partition: WHV_PARTITION_HANDLE, logicaldeviceid: u64) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_System_Hypervisor`*"]
    pub fn WHvEmulatorCreateEmulator(callbacks: *const ::core::mem::ManuallyDrop<WHV_EMULATOR_CALLBACKS>, emulator: *mut *mut ::core::ffi::c_void) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_System_Hypervisor`*"]
    pub fn WHvEmulatorDestroyEmulator(emulator: *const ::core::ffi::c_void) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_System_Hypervisor`*"]
    pub fn WHvEmulatorTryIoEmulation(emulator: *const ::core::ffi::c_void, context: *const ::core::ffi::c_void, vpcontext: *const WHV_VP_EXIT_CONTEXT, ioinstructioncontext: *const WHV_X64_IO_PORT_ACCESS_CONTEXT, emulatorreturnstatus: *mut WHV_EMULATOR_STATUS) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_System_Hypervisor`*"]
    pub fn WHvEmulatorTryMmioEmulation(emulator: *const ::core::ffi::c_void, context: *const ::core::ffi::c_void, vpcontext: *const WHV_VP_EXIT_CONTEXT, mmioinstructioncontext: *const WHV_MEMORY_ACCESS_CONTEXT, emulatorreturnstatus: *mut WHV_EMULATOR_STATUS) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_System_Hypervisor`*"]
    pub fn WHvGetCapability(capabilitycode: WHV_CAPABILITY_CODE, capabilitybuffer: *mut ::core::ffi::c_void, capabilitybuffersizeinbytes: u32, writtensizeinbytes: *mut u32) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_System_Hypervisor`*"]
    pub fn WHvGetInterruptTargetVpSet(partition: WHV_PARTITION_HANDLE, destination: u64, destinationmode: WHV_INTERRUPT_DESTINATION_MODE, targetvps: *mut u32, vpcount: u32, targetvpcount: *mut u32) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_System_Hypervisor`*"]
    pub fn WHvGetPartitionCounters(partition: WHV_PARTITION_HANDLE, counterset: WHV_PARTITION_COUNTER_SET, buffer: *mut ::core::ffi::c_void, buffersizeinbytes: u32, byteswritten: *mut u32) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_System_Hypervisor`*"]
    pub fn WHvGetPartitionProperty(partition: WHV_PARTITION_HANDLE, propertycode: WHV_PARTITION_PROPERTY_CODE, propertybuffer: *mut ::core::ffi::c_void, propertybuffersizeinbytes: u32, writtensizeinbytes: *mut u32) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_System_Hypervisor`*"]
    pub fn WHvGetVirtualProcessorCounters(partition: WHV_PARTITION_HANDLE, vpindex: u32, counterset: WHV_PROCESSOR_COUNTER_SET, buffer: *mut ::core::ffi::c_void, buffersizeinbytes: u32, byteswritten: *mut u32) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_System_Hypervisor`*"]
    pub fn WHvGetVirtualProcessorCpuidOutput(partition: WHV_PARTITION_HANDLE, vpindex: u32, eax: u32, ecx: u32, cpuidoutput: *mut WHV_CPUID_OUTPUT) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_System_Hypervisor`*"]
    pub fn WHvGetVirtualProcessorInterruptControllerState(partition: WHV_PARTITION_HANDLE, vpindex: u32, state: *mut ::core::ffi::c_void, statesize: u32, writtensize: *mut u32) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_System_Hypervisor`*"]
    pub fn WHvGetVirtualProcessorInterruptControllerState2(partition: WHV_PARTITION_HANDLE, vpindex: u32, state: *mut ::core::ffi::c_void, statesize: u32, writtensize: *mut u32) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_System_Hypervisor`*"]
    pub fn WHvGetVirtualProcessorRegisters(partition: WHV_PARTITION_HANDLE, vpindex: u32, registernames: *const WHV_REGISTER_NAME, registercount: u32, registervalues: *mut WHV_REGISTER_VALUE) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_System_Hypervisor`*"]
    pub fn WHvGetVirtualProcessorState(partition: WHV_PARTITION_HANDLE, vpindex: u32, statetype: WHV_VIRTUAL_PROCESSOR_STATE_TYPE, buffer: *mut ::core::ffi::c_void, buffersizeinbytes: u32, byteswritten: *mut u32) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_System_Hypervisor`*"]
    pub fn WHvGetVirtualProcessorXsaveState(partition: WHV_PARTITION_HANDLE, vpindex: u32, buffer: *mut ::core::ffi::c_void, buffersizeinbytes: u32, byteswritten: *mut u32) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_System_Hypervisor`*"]
    pub fn WHvGetVpciDeviceInterruptTarget(partition: WHV_PARTITION_HANDLE, logicaldeviceid: u64, index: u32, multimessagenumber: u32, target: *mut WHV_VPCI_INTERRUPT_TARGET, targetsizeinbytes: u32, byteswritten: *mut u32) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_System_Hypervisor`*"]
    pub fn WHvGetVpciDeviceNotification(partition: WHV_PARTITION_HANDLE, logicaldeviceid: u64, notification: *mut WHV_VPCI_DEVICE_NOTIFICATION, notificationsizeinbytes: u32) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_System_Hypervisor`*"]
    pub fn WHvGetVpciDeviceProperty(partition: WHV_PARTITION_HANDLE, logicaldeviceid: u64, propertycode: WHV_VPCI_DEVICE_PROPERTY_CODE, propertybuffer: *mut ::core::ffi::c_void, propertybuffersizeinbytes: u32, writtensizeinbytes: *mut u32) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_System_Hypervisor`*"]
    pub fn WHvMapGpaRange(partition: WHV_PARTITION_HANDLE, sourceaddress: *const ::core::ffi::c_void, guestaddress: u64, sizeinbytes: u64, flags: WHV_MAP_GPA_RANGE_FLAGS) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_System_Hypervisor`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WHvMapGpaRange2(partition: WHV_PARTITION_HANDLE, process: super::super::Foundation::HANDLE, sourceaddress: *const ::core::ffi::c_void, guestaddress: u64, sizeinbytes: u64, flags: WHV_MAP_GPA_RANGE_FLAGS) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_System_Hypervisor`*"]
    pub fn WHvMapVpciDeviceInterrupt(partition: WHV_PARTITION_HANDLE, logicaldeviceid: u64, index: u32, messagecount: u32, target: *const WHV_VPCI_INTERRUPT_TARGET, msiaddress: *mut u64, msidata: *mut u32) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_System_Hypervisor`*"]
    pub fn WHvMapVpciDeviceMmioRanges(partition: WHV_PARTITION_HANDLE, logicaldeviceid: u64, mappingcount: *mut u32, mappings: *mut *mut WHV_VPCI_MMIO_MAPPING) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_System_Hypervisor`*"]
    pub fn WHvPostVirtualProcessorSynicMessage(partition: WHV_PARTITION_HANDLE, vpindex: u32, sintindex: u32, message: *const ::core::ffi::c_void, messagesizeinbytes: u32) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_System_Hypervisor`*"]
    pub fn WHvQueryGpaRangeDirtyBitmap(partition: WHV_PARTITION_HANDLE, guestaddress: u64, rangesizeinbytes: u64, bitmap: *mut u64, bitmapsizeinbytes: u32) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_System_Hypervisor`*"]
    pub fn WHvReadGpaRange(partition: WHV_PARTITION_HANDLE, vpindex: u32, guestaddress: u64, controls: WHV_ACCESS_GPA_CONTROLS, data: *mut ::core::ffi::c_void, datasizeinbytes: u32) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_System_Hypervisor`*"]
    pub fn WHvReadVpciDeviceRegister(partition: WHV_PARTITION_HANDLE, logicaldeviceid: u64, register: *const WHV_VPCI_DEVICE_REGISTER, data: *mut ::core::ffi::c_void) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_System_Hypervisor`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WHvRegisterPartitionDoorbellEvent(partition: WHV_PARTITION_HANDLE, matchdata: *const WHV_DOORBELL_MATCH_DATA, eventhandle: super::super::Foundation::HANDLE) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_System_Hypervisor`*"]
    pub fn WHvRequestInterrupt(partition: WHV_PARTITION_HANDLE, interrupt: *const WHV_INTERRUPT_CONTROL, interruptcontrolsize: u32) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_System_Hypervisor`*"]
    pub fn WHvRequestVpciDeviceInterrupt(partition: WHV_PARTITION_HANDLE, logicaldeviceid: u64, msiaddress: u64, msidata: u32) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_System_Hypervisor`*"]
    pub fn WHvResetPartition(partition: WHV_PARTITION_HANDLE) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_System_Hypervisor`*"]
    pub fn WHvResumePartitionTime(partition: WHV_PARTITION_HANDLE) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_System_Hypervisor`*"]
    pub fn WHvRetargetVpciDeviceInterrupt(partition: WHV_PARTITION_HANDLE, logicaldeviceid: u64, msiaddress: u64, msidata: u32, target: *const WHV_VPCI_INTERRUPT_TARGET) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_System_Hypervisor`*"]
    pub fn WHvRunVirtualProcessor(partition: WHV_PARTITION_HANDLE, vpindex: u32, exitcontext: *mut ::core::ffi::c_void, exitcontextsizeinbytes: u32) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_System_Hypervisor`*"]
    pub fn WHvSetNotificationPortProperty(partition: WHV_PARTITION_HANDLE, porthandle: *const ::core::ffi::c_void, propertycode: WHV_NOTIFICATION_PORT_PROPERTY_CODE, propertyvalue: u64) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_System_Hypervisor`*"]
    pub fn WHvSetPartitionProperty(partition: WHV_PARTITION_HANDLE, propertycode: WHV_PARTITION_PROPERTY_CODE, propertybuffer: *const ::core::ffi::c_void, propertybuffersizeinbytes: u32) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_System_Hypervisor`*"]
    pub fn WHvSetVirtualProcessorInterruptControllerState(partition: WHV_PARTITION_HANDLE, vpindex: u32, state: *const ::core::ffi::c_void, statesize: u32) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_System_Hypervisor`*"]
    pub fn WHvSetVirtualProcessorInterruptControllerState2(partition: WHV_PARTITION_HANDLE, vpindex: u32, state: *const ::core::ffi::c_void, statesize: u32) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_System_Hypervisor`*"]
    pub fn WHvSetVirtualProcessorRegisters(partition: WHV_PARTITION_HANDLE, vpindex: u32, registernames: *const WHV_REGISTER_NAME, registercount: u32, registervalues: *const WHV_REGISTER_VALUE) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_System_Hypervisor`*"]
    pub fn WHvSetVirtualProcessorState(partition: WHV_PARTITION_HANDLE, vpindex: u32, statetype: WHV_VIRTUAL_PROCESSOR_STATE_TYPE, buffer: *const ::core::ffi::c_void, buffersizeinbytes: u32) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_System_Hypervisor`*"]
    pub fn WHvSetVirtualProcessorXsaveState(partition: WHV_PARTITION_HANDLE, vpindex: u32, buffer: *const ::core::ffi::c_void, buffersizeinbytes: u32) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_System_Hypervisor`, `Win32_System_Power`*"]
    #[cfg(feature = "Win32_System_Power")]
    pub fn WHvSetVpciDevicePowerState(partition: WHV_PARTITION_HANDLE, logicaldeviceid: u64, powerstate: super::Power::DEVICE_POWER_STATE) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_System_Hypervisor`*"]
    pub fn WHvSetupPartition(partition: WHV_PARTITION_HANDLE) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_System_Hypervisor`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WHvSignalVirtualProcessorSynicEvent(partition: WHV_PARTITION_HANDLE, synicevent: WHV_SYNIC_EVENT_PARAMETERS, newlysignaled: *mut super::super::Foundation::BOOL) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_System_Hypervisor`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WHvStartPartitionMigration(partition: WHV_PARTITION_HANDLE, migrationhandle: *mut super::super::Foundation::HANDLE) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_System_Hypervisor`*"]
    pub fn WHvSuspendPartitionTime(partition: WHV_PARTITION_HANDLE) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_System_Hypervisor`*"]
    pub fn WHvTranslateGva(partition: WHV_PARTITION_HANDLE, vpindex: u32, gva: u64, translateflags: WHV_TRANSLATE_GVA_FLAGS, translationresult: *mut WHV_TRANSLATE_GVA_RESULT, gpa: *mut u64) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_System_Hypervisor`*"]
    pub fn WHvUnmapGpaRange(partition: WHV_PARTITION_HANDLE, guestaddress: u64, sizeinbytes: u64) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_System_Hypervisor`*"]
    pub fn WHvUnmapVpciDeviceInterrupt(partition: WHV_PARTITION_HANDLE, logicaldeviceid: u64, index: u32) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_System_Hypervisor`*"]
    pub fn WHvUnmapVpciDeviceMmioRanges(partition: WHV_PARTITION_HANDLE, logicaldeviceid: u64) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_System_Hypervisor`*"]
    pub fn WHvUnregisterPartitionDoorbellEvent(partition: WHV_PARTITION_HANDLE, matchdata: *const WHV_DOORBELL_MATCH_DATA) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_System_Hypervisor`*"]
    pub fn WHvUpdateTriggerParameters(partition: WHV_PARTITION_HANDLE, parameters: *const WHV_TRIGGER_PARAMETERS, triggerhandle: *const ::core::ffi::c_void) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_System_Hypervisor`*"]
    pub fn WHvWriteGpaRange(partition: WHV_PARTITION_HANDLE, vpindex: u32, guestaddress: u64, controls: WHV_ACCESS_GPA_CONTROLS, data: *const ::core::ffi::c_void, datasizeinbytes: u32) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_System_Hypervisor`*"]
    pub fn WHvWriteVpciDeviceRegister(partition: WHV_PARTITION_HANDLE, logicaldeviceid: u64, register: *const WHV_VPCI_DEVICE_REGISTER, data: *const ::core::ffi::c_void) -> ::windows::runtime::HRESULT;
}

#![allow(non_snake_case, non_camel_case_types)]
#[link(name = "windows")]
extern "system" {
    #[doc = "*Required features: `Win32_System_Hypervisor`*"]
    pub fn ApplyGuestMemoryFix(vmsavedstatedumphandle: *mut ::core::ffi::c_void, vpid: u32, virtualaddress: u64, fixbuffer: *const ::core::ffi::c_void, fixbuffersize: u32) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `Win32_System_Hypervisor`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ApplyPendingSavedStateFileReplayLog(vmrsfile: super::super::Foundation::PWSTR) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `Win32_System_Hypervisor`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CallStackUnwind(vmsavedstatedumphandle: *mut ::core::ffi::c_void, vpid: u32, imageinfo: *const MODULE_INFO, imageinfocount: u32, framecount: u32, callstack: *mut super::super::Foundation::PWSTR) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `Win32_System_Hypervisor`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn FindSavedStateSymbolFieldInType(vmsavedstatedumphandle: *mut ::core::ffi::c_void, vpid: u32, typename: super::super::Foundation::PSTR, fieldname: super::super::Foundation::PWSTR, offset: *mut u32, found: *mut super::super::Foundation::BOOL) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `Win32_System_Hypervisor`*"]
    pub fn ForceActiveVirtualTrustLevel(vmsavedstatedumphandle: *mut ::core::ffi::c_void, vpid: u32, virtualtrustlevel: u8) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `Win32_System_Hypervisor`*"]
    pub fn ForceArchitecture(vmsavedstatedumphandle: *mut ::core::ffi::c_void, vpid: u32, architecture: VIRTUAL_PROCESSOR_ARCH) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `Win32_System_Hypervisor`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ForceNestedHostMode(vmsavedstatedumphandle: *mut ::core::ffi::c_void, vpid: u32, hostmode: super::super::Foundation::BOOL, oldmode: *mut super::super::Foundation::BOOL) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `Win32_System_Hypervisor`*"]
    pub fn ForcePagingMode(vmsavedstatedumphandle: *mut ::core::ffi::c_void, vpid: u32, pagingmode: PAGING_MODE) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `Win32_System_Hypervisor`*"]
    pub fn GetActiveVirtualTrustLevel(vmsavedstatedumphandle: *mut ::core::ffi::c_void, vpid: u32, virtualtrustlevel: *mut u8) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `Win32_System_Hypervisor`*"]
    pub fn GetArchitecture(vmsavedstatedumphandle: *mut ::core::ffi::c_void, vpid: u32, architecture: *mut VIRTUAL_PROCESSOR_ARCH) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `Win32_System_Hypervisor`*"]
    pub fn GetEnabledVirtualTrustLevels(vmsavedstatedumphandle: *mut ::core::ffi::c_void, vpid: u32, virtualtrustlevels: *mut u32) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `Win32_System_Hypervisor`*"]
    pub fn GetGuestEnabledVirtualTrustLevels(vmsavedstatedumphandle: *mut ::core::ffi::c_void, virtualtrustlevels: *mut u32) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `Win32_System_Hypervisor`*"]
    pub fn GetGuestOsInfo(vmsavedstatedumphandle: *mut ::core::ffi::c_void, virtualtrustlevel: u8, guestosinfo: *mut GUEST_OS_INFO) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `Win32_System_Hypervisor`*"]
    pub fn GetGuestPhysicalMemoryChunks(vmsavedstatedumphandle: *mut ::core::ffi::c_void, memorychunkpagesize: *mut u64, memorychunks: *mut GPA_MEMORY_CHUNK, memorychunkcount: *mut u64) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `Win32_System_Hypervisor`*"]
    pub fn GetGuestRawSavedMemorySize(vmsavedstatedumphandle: *mut ::core::ffi::c_void, guestrawsavedmemorysize: *mut u64) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `Win32_System_Hypervisor`*"]
    pub fn GetMemoryBlockCacheLimit(vmsavedstatedumphandle: *mut ::core::ffi::c_void, memoryblockcachelimit: *mut u64) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `Win32_System_Hypervisor`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetNestedVirtualizationMode(vmsavedstatedumphandle: *mut ::core::ffi::c_void, vpid: u32, enabled: *mut super::super::Foundation::BOOL) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `Win32_System_Hypervisor`*"]
    pub fn GetPagingMode(vmsavedstatedumphandle: *mut ::core::ffi::c_void, vpid: u32, pagingmode: *mut PAGING_MODE) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `Win32_System_Hypervisor`*"]
    pub fn GetRegisterValue(vmsavedstatedumphandle: *mut ::core::ffi::c_void, vpid: u32, registerid: u32, registervalue: *mut VIRTUAL_PROCESSOR_REGISTER) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `Win32_System_Hypervisor`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetSavedStateSymbolFieldInfo(vmsavedstatedumphandle: *mut ::core::ffi::c_void, vpid: u32, typename: super::super::Foundation::PSTR, typefieldinfomap: *mut super::super::Foundation::PWSTR) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `Win32_System_Hypervisor`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetSavedStateSymbolProviderHandle(vmsavedstatedumphandle: *mut ::core::ffi::c_void) -> super::super::Foundation::HANDLE;
    #[doc = "*Required features: `Win32_System_Hypervisor`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetSavedStateSymbolTypeSize(vmsavedstatedumphandle: *mut ::core::ffi::c_void, vpid: u32, typename: super::super::Foundation::PSTR, size: *mut u32) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `Win32_System_Hypervisor`*"]
    pub fn GetVpCount(vmsavedstatedumphandle: *mut ::core::ffi::c_void, vpcount: *mut u32) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `Win32_System_Hypervisor`*"]
    pub fn GuestPhysicalAddressToRawSavedMemoryOffset(vmsavedstatedumphandle: *mut ::core::ffi::c_void, physicaladdress: u64, rawsavedmemoryoffset: *mut u64) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `Win32_System_Hypervisor`*"]
    pub fn GuestVirtualAddressToPhysicalAddress(vmsavedstatedumphandle: *mut ::core::ffi::c_void, vpid: u32, virtualaddress: u64, physicaladdress: *mut u64, unmappedregionsize: *mut u64) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `Win32_System_Hypervisor`*"]
    pub fn HdvCreateDeviceInstance(devicehosthandle: *const ::core::ffi::c_void, devicetype: HDV_DEVICE_TYPE, deviceclassid: *const ::windows_sys::core::GUID, deviceinstanceid: *const ::windows_sys::core::GUID, deviceinterface: *const ::core::ffi::c_void, devicecontext: *const ::core::ffi::c_void, devicehandle: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `Win32_System_Hypervisor`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn HdvCreateGuestMemoryAperture(requestor: *const ::core::ffi::c_void, guestphysicaladdress: u64, bytecount: u32, writeprotected: super::super::Foundation::BOOL, mappedaddress: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `Win32_System_Hypervisor`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn HdvCreateSectionBackedMmioRange(requestor: *const ::core::ffi::c_void, barindex: HDV_PCI_BAR_SELECTOR, offsetinpages: u64, lengthinpages: u64, mappingflags: HDV_MMIO_MAPPING_FLAGS, sectionhandle: super::super::Foundation::HANDLE, sectionoffsetinpages: u64) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `Win32_System_Hypervisor`*"]
    pub fn HdvDeliverGuestInterrupt(requestor: *const ::core::ffi::c_void, msiaddress: u64, msidata: u32) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `Win32_System_Hypervisor`*"]
    pub fn HdvDestroyGuestMemoryAperture(requestor: *const ::core::ffi::c_void, mappedaddress: *const ::core::ffi::c_void) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `Win32_System_Hypervisor`*"]
    pub fn HdvDestroySectionBackedMmioRange(requestor: *const ::core::ffi::c_void, barindex: HDV_PCI_BAR_SELECTOR, offsetinpages: u64) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `Win32_System_Hypervisor`, `Win32_System_HostComputeSystem`*"]
    #[cfg(feature = "Win32_System_HostComputeSystem")]
    pub fn HdvInitializeDeviceHost(computesystem: super::HostComputeSystem::HCS_SYSTEM, devicehosthandle: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `Win32_System_Hypervisor`*"]
    pub fn HdvReadGuestMemory(requestor: *const ::core::ffi::c_void, guestphysicaladdress: u64, bytecount: u32, buffer: *mut u8) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `Win32_System_Hypervisor`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn HdvRegisterDoorbell(requestor: *const ::core::ffi::c_void, barindex: HDV_PCI_BAR_SELECTOR, baroffset: u64, triggervalue: u64, flags: u64, doorbellevent: super::super::Foundation::HANDLE) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `Win32_System_Hypervisor`*"]
    pub fn HdvTeardownDeviceHost(devicehosthandle: *const ::core::ffi::c_void) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `Win32_System_Hypervisor`*"]
    pub fn HdvUnregisterDoorbell(requestor: *const ::core::ffi::c_void, barindex: HDV_PCI_BAR_SELECTOR, baroffset: u64, triggervalue: u64, flags: u64) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `Win32_System_Hypervisor`*"]
    pub fn HdvWriteGuestMemory(requestor: *const ::core::ffi::c_void, guestphysicaladdress: u64, bytecount: u32, buffer: *const u8) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `Win32_System_Hypervisor`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn InKernelSpace(vmsavedstatedumphandle: *mut ::core::ffi::c_void, vpid: u32, inkernelspace: *mut super::super::Foundation::BOOL) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `Win32_System_Hypervisor`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn IsActiveVirtualTrustLevelEnabled(vmsavedstatedumphandle: *mut ::core::ffi::c_void, vpid: u32, activevirtualtrustlevelenabled: *mut super::super::Foundation::BOOL) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `Win32_System_Hypervisor`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn IsNestedVirtualizationEnabled(vmsavedstatedumphandle: *mut ::core::ffi::c_void, enabled: *mut super::super::Foundation::BOOL) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `Win32_System_Hypervisor`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn LoadSavedStateFile(vmrsfile: super::super::Foundation::PWSTR, vmsavedstatedumphandle: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `Win32_System_Hypervisor`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn LoadSavedStateFiles(binfile: super::super::Foundation::PWSTR, vsvfile: super::super::Foundation::PWSTR, vmsavedstatedumphandle: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `Win32_System_Hypervisor`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn LoadSavedStateModuleSymbols(vmsavedstatedumphandle: *mut ::core::ffi::c_void, imagename: super::super::Foundation::PSTR, modulename: super::super::Foundation::PSTR, baseaddress: u64, sizeofbase: u32) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `Win32_System_Hypervisor`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn LoadSavedStateModuleSymbolsEx(vmsavedstatedumphandle: *mut ::core::ffi::c_void, imagename: super::super::Foundation::PSTR, imagetimestamp: u32, modulename: super::super::Foundation::PSTR, baseaddress: u64, sizeofbase: u32) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `Win32_System_Hypervisor`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn LoadSavedStateSymbolProvider(vmsavedstatedumphandle: *mut ::core::ffi::c_void, usersymbols: super::super::Foundation::PWSTR, force: super::super::Foundation::BOOL) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `Win32_System_Hypervisor`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn LocateSavedStateFiles(vmname: super::super::Foundation::PWSTR, snapshotname: super::super::Foundation::PWSTR, binpath: *mut super::super::Foundation::PWSTR, vsvpath: *mut super::super::Foundation::PWSTR, vmrspath: *mut super::super::Foundation::PWSTR) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `Win32_System_Hypervisor`*"]
    pub fn ReadGuestPhysicalAddress(vmsavedstatedumphandle: *mut ::core::ffi::c_void, physicaladdress: u64, buffer: *mut ::core::ffi::c_void, buffersize: u32, bytesread: *mut u32) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `Win32_System_Hypervisor`*"]
    pub fn ReadGuestRawSavedMemory(vmsavedstatedumphandle: *mut ::core::ffi::c_void, rawsavedmemoryoffset: u64, buffer: *mut ::core::ffi::c_void, buffersize: u32, bytesread: *mut u32) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `Win32_System_Hypervisor`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ReadSavedStateGlobalVariable(vmsavedstatedumphandle: *mut ::core::ffi::c_void, vpid: u32, globalname: super::super::Foundation::PSTR, buffer: *mut ::core::ffi::c_void, buffersize: u32) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `Win32_System_Hypervisor`*"]
    pub fn ReleaseSavedStateFiles(vmsavedstatedumphandle: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `Win32_System_Hypervisor`*"]
    pub fn ReleaseSavedStateSymbolProvider(vmsavedstatedumphandle: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `Win32_System_Hypervisor`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ResolveSavedStateGlobalVariableAddress(vmsavedstatedumphandle: *mut ::core::ffi::c_void, vpid: u32, globalname: super::super::Foundation::PSTR, virtualaddress: *mut u64, size: *mut u32) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `Win32_System_Hypervisor`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ScanMemoryForDosImages(vmsavedstatedumphandle: *mut ::core::ffi::c_void, vpid: u32, startaddress: u64, endaddress: u64, callbackcontext: *mut ::core::ffi::c_void, foundimagecallback: FOUND_IMAGE_CALLBACK, standaloneaddress: *const u64, standaloneaddresscount: u32) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `Win32_System_Hypervisor`*"]
    pub fn SetMemoryBlockCacheLimit(vmsavedstatedumphandle: *mut ::core::ffi::c_void, memoryblockcachelimit: u64) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `Win32_System_Hypervisor`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetSavedStateSymbolProviderDebugInfoCallback(vmsavedstatedumphandle: *mut ::core::ffi::c_void, callback: GUEST_SYMBOLS_PROVIDER_DEBUG_INFO_CALLBACK) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `Win32_System_Hypervisor`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WHvAcceptPartitionMigration(migrationhandle: super::super::Foundation::HANDLE, partition: *mut WHV_PARTITION_HANDLE) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `Win32_System_Hypervisor`*"]
    pub fn WHvAdviseGpaRange(partition: WHV_PARTITION_HANDLE, gparanges: *const WHV_MEMORY_RANGE_ENTRY, gparangescount: u32, advice: WHV_ADVISE_GPA_RANGE_CODE, advicebuffer: *const ::core::ffi::c_void, advicebuffersizeinbytes: u32) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `Win32_System_Hypervisor`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WHvAllocateVpciResource(providerid: *const ::windows_sys::core::GUID, flags: WHV_ALLOCATE_VPCI_RESOURCE_FLAGS, resourcedescriptor: *const ::core::ffi::c_void, resourcedescriptorsizeinbytes: u32, vpciresource: *mut super::super::Foundation::HANDLE) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `Win32_System_Hypervisor`*"]
    pub fn WHvCancelPartitionMigration(partition: WHV_PARTITION_HANDLE) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `Win32_System_Hypervisor`*"]
    pub fn WHvCancelRunVirtualProcessor(partition: WHV_PARTITION_HANDLE, vpindex: u32, flags: u32) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `Win32_System_Hypervisor`*"]
    pub fn WHvCompletePartitionMigration(partition: WHV_PARTITION_HANDLE) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `Win32_System_Hypervisor`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WHvCreateNotificationPort(partition: WHV_PARTITION_HANDLE, parameters: *const WHV_NOTIFICATION_PORT_PARAMETERS, eventhandle: super::super::Foundation::HANDLE, porthandle: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `Win32_System_Hypervisor`*"]
    pub fn WHvCreatePartition(partition: *mut WHV_PARTITION_HANDLE) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `Win32_System_Hypervisor`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WHvCreateTrigger(partition: WHV_PARTITION_HANDLE, parameters: *const WHV_TRIGGER_PARAMETERS, triggerhandle: *mut *mut ::core::ffi::c_void, eventhandle: *mut super::super::Foundation::HANDLE) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `Win32_System_Hypervisor`*"]
    pub fn WHvCreateVirtualProcessor(partition: WHV_PARTITION_HANDLE, vpindex: u32, flags: u32) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `Win32_System_Hypervisor`*"]
    pub fn WHvCreateVirtualProcessor2(partition: WHV_PARTITION_HANDLE, vpindex: u32, properties: *const WHV_VIRTUAL_PROCESSOR_PROPERTY, propertycount: u32) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `Win32_System_Hypervisor`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WHvCreateVpciDevice(partition: WHV_PARTITION_HANDLE, logicaldeviceid: u64, vpciresource: super::super::Foundation::HANDLE, flags: WHV_CREATE_VPCI_DEVICE_FLAGS, notificationeventhandle: super::super::Foundation::HANDLE) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `Win32_System_Hypervisor`*"]
    pub fn WHvDeleteNotificationPort(partition: WHV_PARTITION_HANDLE, porthandle: *const ::core::ffi::c_void) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `Win32_System_Hypervisor`*"]
    pub fn WHvDeletePartition(partition: WHV_PARTITION_HANDLE) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `Win32_System_Hypervisor`*"]
    pub fn WHvDeleteTrigger(partition: WHV_PARTITION_HANDLE, triggerhandle: *const ::core::ffi::c_void) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `Win32_System_Hypervisor`*"]
    pub fn WHvDeleteVirtualProcessor(partition: WHV_PARTITION_HANDLE, vpindex: u32) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `Win32_System_Hypervisor`*"]
    pub fn WHvDeleteVpciDevice(partition: WHV_PARTITION_HANDLE, logicaldeviceid: u64) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `Win32_System_Hypervisor`*"]
    pub fn WHvEmulatorCreateEmulator(callbacks: *const WHV_EMULATOR_CALLBACKS, emulator: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `Win32_System_Hypervisor`*"]
    pub fn WHvEmulatorDestroyEmulator(emulator: *const ::core::ffi::c_void) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `Win32_System_Hypervisor`*"]
    pub fn WHvEmulatorTryIoEmulation(emulator: *const ::core::ffi::c_void, context: *const ::core::ffi::c_void, vpcontext: *const WHV_VP_EXIT_CONTEXT, ioinstructioncontext: *const WHV_X64_IO_PORT_ACCESS_CONTEXT, emulatorreturnstatus: *mut WHV_EMULATOR_STATUS) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `Win32_System_Hypervisor`*"]
    pub fn WHvEmulatorTryMmioEmulation(emulator: *const ::core::ffi::c_void, context: *const ::core::ffi::c_void, vpcontext: *const WHV_VP_EXIT_CONTEXT, mmioinstructioncontext: *const WHV_MEMORY_ACCESS_CONTEXT, emulatorreturnstatus: *mut WHV_EMULATOR_STATUS) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `Win32_System_Hypervisor`*"]
    pub fn WHvGetCapability(capabilitycode: WHV_CAPABILITY_CODE, capabilitybuffer: *mut ::core::ffi::c_void, capabilitybuffersizeinbytes: u32, writtensizeinbytes: *mut u32) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `Win32_System_Hypervisor`*"]
    pub fn WHvGetInterruptTargetVpSet(partition: WHV_PARTITION_HANDLE, destination: u64, destinationmode: WHV_INTERRUPT_DESTINATION_MODE, targetvps: *mut u32, vpcount: u32, targetvpcount: *mut u32) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `Win32_System_Hypervisor`*"]
    pub fn WHvGetPartitionCounters(partition: WHV_PARTITION_HANDLE, counterset: WHV_PARTITION_COUNTER_SET, buffer: *mut ::core::ffi::c_void, buffersizeinbytes: u32, byteswritten: *mut u32) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `Win32_System_Hypervisor`*"]
    pub fn WHvGetPartitionProperty(partition: WHV_PARTITION_HANDLE, propertycode: WHV_PARTITION_PROPERTY_CODE, propertybuffer: *mut ::core::ffi::c_void, propertybuffersizeinbytes: u32, writtensizeinbytes: *mut u32) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `Win32_System_Hypervisor`*"]
    pub fn WHvGetVirtualProcessorCounters(partition: WHV_PARTITION_HANDLE, vpindex: u32, counterset: WHV_PROCESSOR_COUNTER_SET, buffer: *mut ::core::ffi::c_void, buffersizeinbytes: u32, byteswritten: *mut u32) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `Win32_System_Hypervisor`*"]
    pub fn WHvGetVirtualProcessorCpuidOutput(partition: WHV_PARTITION_HANDLE, vpindex: u32, eax: u32, ecx: u32, cpuidoutput: *mut WHV_CPUID_OUTPUT) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `Win32_System_Hypervisor`*"]
    pub fn WHvGetVirtualProcessorInterruptControllerState(partition: WHV_PARTITION_HANDLE, vpindex: u32, state: *mut ::core::ffi::c_void, statesize: u32, writtensize: *mut u32) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `Win32_System_Hypervisor`*"]
    pub fn WHvGetVirtualProcessorInterruptControllerState2(partition: WHV_PARTITION_HANDLE, vpindex: u32, state: *mut ::core::ffi::c_void, statesize: u32, writtensize: *mut u32) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `Win32_System_Hypervisor`*"]
    pub fn WHvGetVirtualProcessorRegisters(partition: WHV_PARTITION_HANDLE, vpindex: u32, registernames: *const WHV_REGISTER_NAME, registercount: u32, registervalues: *mut WHV_REGISTER_VALUE) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `Win32_System_Hypervisor`*"]
    pub fn WHvGetVirtualProcessorState(partition: WHV_PARTITION_HANDLE, vpindex: u32, statetype: WHV_VIRTUAL_PROCESSOR_STATE_TYPE, buffer: *mut ::core::ffi::c_void, buffersizeinbytes: u32, byteswritten: *mut u32) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `Win32_System_Hypervisor`*"]
    pub fn WHvGetVirtualProcessorXsaveState(partition: WHV_PARTITION_HANDLE, vpindex: u32, buffer: *mut ::core::ffi::c_void, buffersizeinbytes: u32, byteswritten: *mut u32) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `Win32_System_Hypervisor`*"]
    pub fn WHvGetVpciDeviceInterruptTarget(partition: WHV_PARTITION_HANDLE, logicaldeviceid: u64, index: u32, multimessagenumber: u32, target: *mut WHV_VPCI_INTERRUPT_TARGET, targetsizeinbytes: u32, byteswritten: *mut u32) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `Win32_System_Hypervisor`*"]
    pub fn WHvGetVpciDeviceNotification(partition: WHV_PARTITION_HANDLE, logicaldeviceid: u64, notification: *mut WHV_VPCI_DEVICE_NOTIFICATION, notificationsizeinbytes: u32) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `Win32_System_Hypervisor`*"]
    pub fn WHvGetVpciDeviceProperty(partition: WHV_PARTITION_HANDLE, logicaldeviceid: u64, propertycode: WHV_VPCI_DEVICE_PROPERTY_CODE, propertybuffer: *mut ::core::ffi::c_void, propertybuffersizeinbytes: u32, writtensizeinbytes: *mut u32) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `Win32_System_Hypervisor`*"]
    pub fn WHvMapGpaRange(partition: WHV_PARTITION_HANDLE, sourceaddress: *const ::core::ffi::c_void, guestaddress: u64, sizeinbytes: u64, flags: WHV_MAP_GPA_RANGE_FLAGS) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `Win32_System_Hypervisor`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WHvMapGpaRange2(partition: WHV_PARTITION_HANDLE, process: super::super::Foundation::HANDLE, sourceaddress: *const ::core::ffi::c_void, guestaddress: u64, sizeinbytes: u64, flags: WHV_MAP_GPA_RANGE_FLAGS) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `Win32_System_Hypervisor`*"]
    pub fn WHvMapVpciDeviceInterrupt(partition: WHV_PARTITION_HANDLE, logicaldeviceid: u64, index: u32, messagecount: u32, target: *const WHV_VPCI_INTERRUPT_TARGET, msiaddress: *mut u64, msidata: *mut u32) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `Win32_System_Hypervisor`*"]
    pub fn WHvMapVpciDeviceMmioRanges(partition: WHV_PARTITION_HANDLE, logicaldeviceid: u64, mappingcount: *mut u32, mappings: *mut *mut WHV_VPCI_MMIO_MAPPING) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `Win32_System_Hypervisor`*"]
    pub fn WHvPostVirtualProcessorSynicMessage(partition: WHV_PARTITION_HANDLE, vpindex: u32, sintindex: u32, message: *const ::core::ffi::c_void, messagesizeinbytes: u32) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `Win32_System_Hypervisor`*"]
    pub fn WHvQueryGpaRangeDirtyBitmap(partition: WHV_PARTITION_HANDLE, guestaddress: u64, rangesizeinbytes: u64, bitmap: *mut u64, bitmapsizeinbytes: u32) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `Win32_System_Hypervisor`*"]
    pub fn WHvReadGpaRange(partition: WHV_PARTITION_HANDLE, vpindex: u32, guestaddress: u64, controls: WHV_ACCESS_GPA_CONTROLS, data: *mut ::core::ffi::c_void, datasizeinbytes: u32) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `Win32_System_Hypervisor`*"]
    pub fn WHvReadVpciDeviceRegister(partition: WHV_PARTITION_HANDLE, logicaldeviceid: u64, register: *const WHV_VPCI_DEVICE_REGISTER, data: *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `Win32_System_Hypervisor`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WHvRegisterPartitionDoorbellEvent(partition: WHV_PARTITION_HANDLE, matchdata: *const WHV_DOORBELL_MATCH_DATA, eventhandle: super::super::Foundation::HANDLE) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `Win32_System_Hypervisor`*"]
    pub fn WHvRequestInterrupt(partition: WHV_PARTITION_HANDLE, interrupt: *const WHV_INTERRUPT_CONTROL, interruptcontrolsize: u32) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `Win32_System_Hypervisor`*"]
    pub fn WHvRequestVpciDeviceInterrupt(partition: WHV_PARTITION_HANDLE, logicaldeviceid: u64, msiaddress: u64, msidata: u32) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `Win32_System_Hypervisor`*"]
    pub fn WHvResetPartition(partition: WHV_PARTITION_HANDLE) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `Win32_System_Hypervisor`*"]
    pub fn WHvResumePartitionTime(partition: WHV_PARTITION_HANDLE) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `Win32_System_Hypervisor`*"]
    pub fn WHvRetargetVpciDeviceInterrupt(partition: WHV_PARTITION_HANDLE, logicaldeviceid: u64, msiaddress: u64, msidata: u32, target: *const WHV_VPCI_INTERRUPT_TARGET) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `Win32_System_Hypervisor`*"]
    pub fn WHvRunVirtualProcessor(partition: WHV_PARTITION_HANDLE, vpindex: u32, exitcontext: *mut ::core::ffi::c_void, exitcontextsizeinbytes: u32) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `Win32_System_Hypervisor`*"]
    pub fn WHvSetNotificationPortProperty(partition: WHV_PARTITION_HANDLE, porthandle: *const ::core::ffi::c_void, propertycode: WHV_NOTIFICATION_PORT_PROPERTY_CODE, propertyvalue: u64) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `Win32_System_Hypervisor`*"]
    pub fn WHvSetPartitionProperty(partition: WHV_PARTITION_HANDLE, propertycode: WHV_PARTITION_PROPERTY_CODE, propertybuffer: *const ::core::ffi::c_void, propertybuffersizeinbytes: u32) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `Win32_System_Hypervisor`*"]
    pub fn WHvSetVirtualProcessorInterruptControllerState(partition: WHV_PARTITION_HANDLE, vpindex: u32, state: *const ::core::ffi::c_void, statesize: u32) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `Win32_System_Hypervisor`*"]
    pub fn WHvSetVirtualProcessorInterruptControllerState2(partition: WHV_PARTITION_HANDLE, vpindex: u32, state: *const ::core::ffi::c_void, statesize: u32) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `Win32_System_Hypervisor`*"]
    pub fn WHvSetVirtualProcessorRegisters(partition: WHV_PARTITION_HANDLE, vpindex: u32, registernames: *const WHV_REGISTER_NAME, registercount: u32, registervalues: *const WHV_REGISTER_VALUE) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `Win32_System_Hypervisor`*"]
    pub fn WHvSetVirtualProcessorState(partition: WHV_PARTITION_HANDLE, vpindex: u32, statetype: WHV_VIRTUAL_PROCESSOR_STATE_TYPE, buffer: *const ::core::ffi::c_void, buffersizeinbytes: u32) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `Win32_System_Hypervisor`*"]
    pub fn WHvSetVirtualProcessorXsaveState(partition: WHV_PARTITION_HANDLE, vpindex: u32, buffer: *const ::core::ffi::c_void, buffersizeinbytes: u32) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `Win32_System_Hypervisor`, `Win32_System_Power`*"]
    #[cfg(feature = "Win32_System_Power")]
    pub fn WHvSetVpciDevicePowerState(partition: WHV_PARTITION_HANDLE, logicaldeviceid: u64, powerstate: super::Power::DEVICE_POWER_STATE) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `Win32_System_Hypervisor`*"]
    pub fn WHvSetupPartition(partition: WHV_PARTITION_HANDLE) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `Win32_System_Hypervisor`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WHvSignalVirtualProcessorSynicEvent(partition: WHV_PARTITION_HANDLE, synicevent: WHV_SYNIC_EVENT_PARAMETERS, newlysignaled: *mut super::super::Foundation::BOOL) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `Win32_System_Hypervisor`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WHvStartPartitionMigration(partition: WHV_PARTITION_HANDLE, migrationhandle: *mut super::super::Foundation::HANDLE) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `Win32_System_Hypervisor`*"]
    pub fn WHvSuspendPartitionTime(partition: WHV_PARTITION_HANDLE) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `Win32_System_Hypervisor`*"]
    pub fn WHvTranslateGva(partition: WHV_PARTITION_HANDLE, vpindex: u32, gva: u64, translateflags: WHV_TRANSLATE_GVA_FLAGS, translationresult: *mut WHV_TRANSLATE_GVA_RESULT, gpa: *mut u64) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `Win32_System_Hypervisor`*"]
    pub fn WHvUnmapGpaRange(partition: WHV_PARTITION_HANDLE, guestaddress: u64, sizeinbytes: u64) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `Win32_System_Hypervisor`*"]
    pub fn WHvUnmapVpciDeviceInterrupt(partition: WHV_PARTITION_HANDLE, logicaldeviceid: u64, index: u32) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `Win32_System_Hypervisor`*"]
    pub fn WHvUnmapVpciDeviceMmioRanges(partition: WHV_PARTITION_HANDLE, logicaldeviceid: u64) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `Win32_System_Hypervisor`*"]
    pub fn WHvUnregisterPartitionDoorbellEvent(partition: WHV_PARTITION_HANDLE, matchdata: *const WHV_DOORBELL_MATCH_DATA) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `Win32_System_Hypervisor`*"]
    pub fn WHvUpdateTriggerParameters(partition: WHV_PARTITION_HANDLE, parameters: *const WHV_TRIGGER_PARAMETERS, triggerhandle: *const ::core::ffi::c_void) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `Win32_System_Hypervisor`*"]
    pub fn WHvWriteGpaRange(partition: WHV_PARTITION_HANDLE, vpindex: u32, guestaddress: u64, controls: WHV_ACCESS_GPA_CONTROLS, data: *const ::core::ffi::c_void, datasizeinbytes: u32) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `Win32_System_Hypervisor`*"]
    pub fn WHvWriteVpciDeviceRegister(partition: WHV_PARTITION_HANDLE, logicaldeviceid: u64, register: *const WHV_VPCI_DEVICE_REGISTER, data: *const ::core::ffi::c_void) -> ::windows_sys::core::HRESULT;
}
#[cfg(feature = "Win32_Foundation")]
pub struct DOS_IMAGE_INFO(i32);
pub struct FOUND_IMAGE_CALLBACK(i32);
pub struct GPA_MEMORY_CHUNK(i32);
pub struct GUEST_OS_INFO(i32);
pub struct GUEST_OS_MICROSOFT_IDS(i32);
pub struct GUEST_OS_OPENSOURCE_IDS(i32);
pub struct GUEST_OS_VENDOR(i32);
pub struct GUEST_SYMBOLS_PROVIDER_DEBUG_INFO_CALLBACK(i32);
pub const GUID_DEVINTERFACE_VM_GENCOUNTER: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 1072875819, data2: 26008, data3: 20064, data4: [142, 28, 12, 207, 73, 39, 227, 25] };
pub struct HDV_DEVICE_TYPE(i32);
pub struct HDV_DOORBELL_FLAGS(i32);
pub struct HDV_MMIO_MAPPING_FLAGS(i32);
#[doc = "*Required features: `Win32_System_Hypervisor`*"]
pub const HDV_PCI_BAR_COUNT: u32 = 6u32;
pub struct HDV_PCI_BAR_SELECTOR(i32);
pub struct HDV_PCI_DEVICE_GET_DETAILS(i32);
pub struct HDV_PCI_DEVICE_INITIALIZE(i32);
#[cfg(feature = "Win32_Foundation")]
pub struct HDV_PCI_DEVICE_INTERFACE(i32);
pub struct HDV_PCI_DEVICE_SET_CONFIGURATION(i32);
pub struct HDV_PCI_DEVICE_START(i32);
pub struct HDV_PCI_DEVICE_STOP(i32);
pub struct HDV_PCI_DEVICE_TEARDOWN(i32);
pub struct HDV_PCI_INTERFACE_VERSION(i32);
pub struct HDV_PCI_PNP_ID(i32);
pub struct HDV_PCI_READ_CONFIG_SPACE(i32);
pub struct HDV_PCI_READ_INTERCEPTED_MEMORY(i32);
pub struct HDV_PCI_WRITE_CONFIG_SPACE(i32);
pub struct HDV_PCI_WRITE_INTERCEPTED_MEMORY(i32);
#[doc = "*Required features: `Win32_System_Hypervisor`*"]
pub const HVSOCKET_ADDRESS_FLAG_PASSTHRU: u32 = 1u32;
pub struct HVSOCKET_ADDRESS_INFO(i32);
#[doc = "*Required features: `Win32_System_Hypervisor`*"]
pub const HVSOCKET_CONNECTED_SUSPEND: u32 = 4u32;
#[doc = "*Required features: `Win32_System_Hypervisor`*"]
pub const HVSOCKET_CONNECT_TIMEOUT: u32 = 1u32;
#[doc = "*Required features: `Win32_System_Hypervisor`*"]
pub const HVSOCKET_CONNECT_TIMEOUT_MAX: u32 = 300000u32;
#[doc = "*Required features: `Win32_System_Hypervisor`*"]
pub const HVSOCKET_CONTAINER_PASSTHRU: u32 = 2u32;
pub const HV_GUID_BROADCAST: ::windows_sys::core::GUID = ::windows_sys::GUID {
    data1: 4294967295,
    data2: 65535,
    data3: 65535,
    data4: [255, 255, 255, 255, 255, 255, 255, 255],
};
pub const HV_GUID_CHILDREN: ::windows_sys::core::GUID = ::windows_sys::GUID {
    data1: 2430307209,
    data2: 3381,
    data3: 20345,
    data4: [140, 233, 73, 234, 10, 200, 183, 205],
};
pub const HV_GUID_LOOPBACK: ::windows_sys::core::GUID = ::windows_sys::GUID {
    data1: 3772866967,
    data2: 56662,
    data3: 18960,
    data4: [145, 149, 94, 231, 161, 85, 168, 56],
};
pub const HV_GUID_PARENT: ::windows_sys::core::GUID = ::windows_sys::GUID {
    data1: 2754510042,
    data2: 53311,
    data3: 18444,
    data4: [156, 194, 164, 222, 32, 171, 184, 120],
};
pub const HV_GUID_SILOHOST: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 918359132, data2: 29302, data3: 16931, data4: [136, 186, 125, 3, 182, 84, 197, 104] };
pub const HV_GUID_VSOCK_TEMPLATE: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 0, data2: 64203, data3: 4582, data4: [189, 88, 100, 0, 106, 121, 134, 211] };
pub const HV_GUID_ZERO: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 0, data2: 0, data3: 0, data4: [0, 0, 0, 0, 0, 0, 0, 0] };
#[doc = "*Required features: `Win32_System_Hypervisor`*"]
pub const HV_PROTOCOL_RAW: u32 = 1u32;
#[doc = "*Required features: `Win32_System_Hypervisor`*"]
pub const IOCTL_VMGENCOUNTER_READ: u32 = 3325956u32;
#[cfg(feature = "Win32_Foundation")]
pub struct MODULE_INFO(i32);
pub struct PAGING_MODE(i32);
pub struct REGISTER_ID(i32);
pub struct SOCKADDR_HV(i32);
pub struct VIRTUAL_PROCESSOR_ARCH(i32);
pub struct VIRTUAL_PROCESSOR_REGISTER(i32);
pub struct VIRTUAL_PROCESSOR_VENDOR(i32);
pub struct VM_GENCOUNTER(i32);
pub struct WHV_ACCESS_GPA_CONTROLS(i32);
pub struct WHV_ADVISE_GPA_RANGE(i32);
pub struct WHV_ADVISE_GPA_RANGE_CODE(i32);
pub struct WHV_ADVISE_GPA_RANGE_POPULATE(i32);
pub struct WHV_ADVISE_GPA_RANGE_POPULATE_FLAGS(i32);
pub struct WHV_ALLOCATE_VPCI_RESOURCE_FLAGS(i32);
#[doc = "*Required features: `Win32_System_Hypervisor`*"]
pub const WHV_ANY_VP: u32 = 4294967295u32;
pub struct WHV_CACHE_TYPE(i32);
#[cfg(feature = "Win32_Foundation")]
pub struct WHV_CAPABILITY(i32);
pub struct WHV_CAPABILITY_CODE(i32);
pub struct WHV_CAPABILITY_FEATURES(i32);
pub struct WHV_CAPABILITY_PROCESSOR_FREQUENCY_CAP(i32);
pub struct WHV_CPUID_OUTPUT(i32);
pub struct WHV_CREATE_VPCI_DEVICE_FLAGS(i32);
pub struct WHV_DOORBELL_MATCH_DATA(i32);
pub struct WHV_EMULATOR_CALLBACKS(i32);
pub struct WHV_EMULATOR_GET_VIRTUAL_PROCESSOR_REGISTERS_CALLBACK(i32);
pub struct WHV_EMULATOR_IO_ACCESS_INFO(i32);
pub struct WHV_EMULATOR_IO_PORT_CALLBACK(i32);
pub struct WHV_EMULATOR_MEMORY_ACCESS_INFO(i32);
pub struct WHV_EMULATOR_MEMORY_CALLBACK(i32);
pub struct WHV_EMULATOR_SET_VIRTUAL_PROCESSOR_REGISTERS_CALLBACK(i32);
pub struct WHV_EMULATOR_STATUS(i32);
pub struct WHV_EMULATOR_TRANSLATE_GVA_PAGE_CALLBACK(i32);
pub struct WHV_EXCEPTION_TYPE(i32);
pub struct WHV_EXTENDED_VM_EXITS(i32);
pub struct WHV_HYPERCALL_CONTEXT(i32);
#[doc = "*Required features: `Win32_System_Hypervisor`*"]
pub const WHV_HYPERCALL_CONTEXT_MAX_XMM_REGISTERS: u32 = 6u32;
pub struct WHV_INTERNAL_ACTIVITY_REGISTER(i32);
pub struct WHV_INTERRUPT_CONTROL(i32);
pub struct WHV_INTERRUPT_DESTINATION_MODE(i32);
pub struct WHV_INTERRUPT_TRIGGER_MODE(i32);
pub struct WHV_INTERRUPT_TYPE(i32);
pub struct WHV_MAP_GPA_RANGE_FLAGS(i32);
#[doc = "*Required features: `Win32_System_Hypervisor`*"]
pub const WHV_MAX_DEVICE_ID_SIZE_IN_CHARS: u32 = 200u32;
pub struct WHV_MEMORY_ACCESS_CONTEXT(i32);
pub struct WHV_MEMORY_ACCESS_INFO(i32);
pub struct WHV_MEMORY_ACCESS_TYPE(i32);
pub struct WHV_MEMORY_RANGE_ENTRY(i32);
pub struct WHV_MSR_ACTION(i32);
pub struct WHV_MSR_ACTION_ENTRY(i32);
pub struct WHV_NOTIFICATION_PORT_PARAMETERS(i32);
pub struct WHV_NOTIFICATION_PORT_PROPERTY_CODE(i32);
pub struct WHV_NOTIFICATION_PORT_TYPE(i32);
pub struct WHV_PARTITION_COUNTER_SET(i32);
pub struct WHV_PARTITION_HANDLE(i32);
pub struct WHV_PARTITION_MEMORY_COUNTERS(i32);
#[cfg(feature = "Win32_Foundation")]
pub struct WHV_PARTITION_PROPERTY(i32);
pub struct WHV_PARTITION_PROPERTY_CODE(i32);
pub struct WHV_PROCESSOR_APIC_COUNTERS(i32);
pub struct WHV_PROCESSOR_COUNTER_SET(i32);
pub struct WHV_PROCESSOR_EVENT_COUNTERS(i32);
pub struct WHV_PROCESSOR_FEATURES(i32);
pub struct WHV_PROCESSOR_FEATURES1(i32);
pub struct WHV_PROCESSOR_FEATURES_BANKS(i32);
#[doc = "*Required features: `Win32_System_Hypervisor`*"]
pub const WHV_PROCESSOR_FEATURES_BANKS_COUNT: u32 = 2u32;
pub struct WHV_PROCESSOR_INTERCEPT_COUNTER(i32);
pub struct WHV_PROCESSOR_INTERCEPT_COUNTERS(i32);
pub struct WHV_PROCESSOR_PERFMON_FEATURES(i32);
pub struct WHV_PROCESSOR_RUNTIME_COUNTERS(i32);
pub struct WHV_PROCESSOR_SYNTHETIC_FEATURES_COUNTERS(i32);
pub struct WHV_PROCESSOR_VENDOR(i32);
pub struct WHV_PROCESSOR_XSAVE_FEATURES(i32);
#[doc = "*Required features: `Win32_System_Hypervisor`*"]
pub const WHV_READ_WRITE_GPA_RANGE_MAX_SIZE: u32 = 16u32;
pub struct WHV_REGISTER_NAME(i32);
pub struct WHV_REGISTER_VALUE(i32);
pub struct WHV_RUN_VP_CANCELED_CONTEXT(i32);
pub struct WHV_RUN_VP_CANCEL_REASON(i32);
pub struct WHV_RUN_VP_EXIT_CONTEXT(i32);
pub struct WHV_RUN_VP_EXIT_REASON(i32);
pub struct WHV_SCHEDULER_FEATURES(i32);
#[cfg(feature = "Win32_Foundation")]
pub struct WHV_SRIOV_RESOURCE_DESCRIPTOR(i32);
pub struct WHV_SYNIC_EVENT_PARAMETERS(i32);
#[doc = "*Required features: `Win32_System_Hypervisor`*"]
pub const WHV_SYNIC_MESSAGE_SIZE: u32 = 256u32;
pub struct WHV_SYNIC_SINT_DELIVERABLE_CONTEXT(i32);
pub struct WHV_SYNTHETIC_PROCESSOR_FEATURES(i32);
pub struct WHV_SYNTHETIC_PROCESSOR_FEATURES_BANKS(i32);
#[doc = "*Required features: `Win32_System_Hypervisor`*"]
pub const WHV_SYNTHETIC_PROCESSOR_FEATURES_BANKS_COUNT: u32 = 1u32;
pub struct WHV_TRANSLATE_GVA_FLAGS(i32);
pub struct WHV_TRANSLATE_GVA_RESULT(i32);
pub struct WHV_TRANSLATE_GVA_RESULT_CODE(i32);
pub struct WHV_TRIGGER_PARAMETERS(i32);
pub struct WHV_TRIGGER_TYPE(i32);
pub struct WHV_UINT128(i32);
pub struct WHV_VIRTUAL_PROCESSOR_PROPERTY(i32);
pub struct WHV_VIRTUAL_PROCESSOR_PROPERTY_CODE(i32);
pub struct WHV_VIRTUAL_PROCESSOR_STATE_TYPE(i32);
pub struct WHV_VPCI_DEVICE_NOTIFICATION(i32);
pub struct WHV_VPCI_DEVICE_NOTIFICATION_TYPE(i32);
pub struct WHV_VPCI_DEVICE_PROPERTY_CODE(i32);
pub struct WHV_VPCI_DEVICE_REGISTER(i32);
pub struct WHV_VPCI_DEVICE_REGISTER_SPACE(i32);
pub struct WHV_VPCI_HARDWARE_IDS(i32);
pub struct WHV_VPCI_INTERRUPT_TARGET(i32);
pub struct WHV_VPCI_INTERRUPT_TARGET_FLAGS(i32);
pub struct WHV_VPCI_MMIO_MAPPING(i32);
pub struct WHV_VPCI_MMIO_RANGE_FLAGS(i32);
pub struct WHV_VPCI_PROBED_BARS(i32);
#[doc = "*Required features: `Win32_System_Hypervisor`*"]
pub const WHV_VPCI_TYPE0_BAR_COUNT: u32 = 6u32;
pub struct WHV_VP_EXCEPTION_CONTEXT(i32);
pub struct WHV_VP_EXCEPTION_INFO(i32);
pub struct WHV_VP_EXIT_CONTEXT(i32);
pub struct WHV_X64_APIC_EOI_CONTEXT(i32);
pub struct WHV_X64_APIC_INIT_SIPI_CONTEXT(i32);
pub struct WHV_X64_APIC_SMI_CONTEXT(i32);
pub struct WHV_X64_APIC_WRITE_CONTEXT(i32);
pub struct WHV_X64_APIC_WRITE_TYPE(i32);
pub struct WHV_X64_CPUID_ACCESS_CONTEXT(i32);
pub struct WHV_X64_CPUID_RESULT(i32);
pub struct WHV_X64_CPUID_RESULT2(i32);
pub struct WHV_X64_CPUID_RESULT2_FLAGS(i32);
pub struct WHV_X64_DELIVERABILITY_NOTIFICATIONS_REGISTER(i32);
pub struct WHV_X64_FP_CONTROL_STATUS_REGISTER(i32);
pub struct WHV_X64_FP_REGISTER(i32);
pub struct WHV_X64_INTERRUPTION_DELIVERABLE_CONTEXT(i32);
pub struct WHV_X64_INTERRUPT_STATE_REGISTER(i32);
pub struct WHV_X64_IO_PORT_ACCESS_CONTEXT(i32);
pub struct WHV_X64_IO_PORT_ACCESS_INFO(i32);
pub struct WHV_X64_LOCAL_APIC_EMULATION_MODE(i32);
pub struct WHV_X64_MSR_ACCESS_CONTEXT(i32);
pub struct WHV_X64_MSR_ACCESS_INFO(i32);
pub struct WHV_X64_MSR_EXIT_BITMAP(i32);
pub struct WHV_X64_PENDING_DEBUG_EXCEPTION(i32);
pub struct WHV_X64_PENDING_EVENT_TYPE(i32);
pub struct WHV_X64_PENDING_EXCEPTION_EVENT(i32);
pub struct WHV_X64_PENDING_EXT_INT_EVENT(i32);
pub struct WHV_X64_PENDING_INTERRUPTION_REGISTER(i32);
pub struct WHV_X64_PENDING_INTERRUPTION_TYPE(i32);
pub struct WHV_X64_RDTSC_CONTEXT(i32);
pub struct WHV_X64_RDTSC_INFO(i32);
pub struct WHV_X64_SEGMENT_REGISTER(i32);
pub struct WHV_X64_TABLE_REGISTER(i32);
pub struct WHV_X64_UNSUPPORTED_FEATURE_CODE(i32);
pub struct WHV_X64_UNSUPPORTED_FEATURE_CONTEXT(i32);
pub struct WHV_X64_VP_EXECUTION_STATE(i32);
pub struct WHV_X64_XMM_CONTROL_STATUS_REGISTER(i32);

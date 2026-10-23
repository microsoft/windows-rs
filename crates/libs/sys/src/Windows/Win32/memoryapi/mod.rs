#[cfg(all(feature = "shared", feature = "winnt"))]
windows_link::link!("kernel32.dll" "system" fn AllocateUserPhysicalPages(hprocess : super::HANDLE, numberofpages : super::PULONG_PTR, pagearray : super::PULONG_PTR) -> windows_sys::core::BOOL);
#[cfg(all(feature = "shared", feature = "winnt"))]
windows_link::link!("api-ms-win-core-memory-l1-1-8.dll" "system" fn AllocateUserPhysicalPages2(objecthandle : super::HANDLE, numberofpages : super::PULONG_PTR, pagearray : super::PULONG_PTR, extendedparameters : super::PMEM_EXTENDED_PARAMETER, extendedparametercount : super::ULONG) -> windows_sys::core::BOOL);
#[cfg(all(feature = "shared", feature = "winnt"))]
windows_link::link!("kernel32.dll" "system" fn AllocateUserPhysicalPagesNuma(hprocess : super::HANDLE, numberofpages : super::PULONG_PTR, pagearray : super::PULONG_PTR, nndpreferred : super::DWORD) -> windows_sys::core::BOOL);
#[cfg(all(feature = "minwinbase", feature = "minwindef", feature = "shared", feature = "winnt"))]
windows_link::link!("api-ms-win-core-memory-l1-1-7.dll" "system" fn CreateFileMapping2(file : super::HANDLE, securityattributes : *const super::SECURITY_ATTRIBUTES, desiredaccess : super::ULONG, pageprotection : super::ULONG, allocationattributes : super::ULONG, maximumsize : super::ULONG64, name : windows_sys::core::PCWSTR, extendedparameters : *mut super::MEM_EXTENDED_PARAMETER, parametercount : super::ULONG) -> super::HANDLE);
#[cfg(all(feature = "minwinbase", feature = "minwindef", feature = "shared", feature = "winnt"))]
windows_link::link!("kernel32.dll" "system" fn CreateFileMappingFromApp(hfile : super::HANDLE, securityattributes : super::PSECURITY_ATTRIBUTES, pageprotection : super::ULONG, maximumsize : super::ULONG64, name : windows_sys::core::PCWSTR) -> super::HANDLE);
#[cfg(all(feature = "minwinbase", feature = "minwindef", feature = "shared", feature = "winnt"))]
windows_link::link!("kernel32.dll" "system" fn CreateFileMappingNumaW(hfile : super::HANDLE, lpfilemappingattributes : super::LPSECURITY_ATTRIBUTES, flprotect : super::DWORD, dwmaximumsizehigh : super::DWORD, dwmaximumsizelow : super::DWORD, lpname : windows_sys::core::PCWSTR, nndpreferred : super::DWORD) -> super::HANDLE);
#[cfg(all(feature = "minwinbase", feature = "minwindef", feature = "shared", feature = "winnt"))]
windows_link::link!("kernel32.dll" "system" fn CreateFileMappingW(hfile : super::HANDLE, lpfilemappingattributes : super::LPSECURITY_ATTRIBUTES, flprotect : super::DWORD, dwmaximumsizehigh : super::DWORD, dwmaximumsizelow : super::DWORD, lpname : windows_sys::core::PCWSTR) -> super::HANDLE);
#[cfg(feature = "winnt")]
windows_link::link!("kernel32.dll" "system" fn CreateMemoryResourceNotification(notificationtype : MEMORY_RESOURCE_NOTIFICATION_TYPE) -> super::HANDLE);
#[cfg(all(feature = "shared", feature = "winnt"))]
windows_link::link!("kernel32.dll" "system" fn DiscardVirtualMemory(virtualaddress : super::PVOID, size : super::SIZE_T) -> super::DWORD);
#[cfg(feature = "shared")]
windows_link::link!("kernel32.dll" "system" fn FlushViewOfFile(lpbaseaddress : super::LPCVOID, dwnumberofbytestoflush : super::SIZE_T) -> windows_sys::core::BOOL);
#[cfg(all(feature = "shared", feature = "winnt"))]
windows_link::link!("kernel32.dll" "system" fn FreeUserPhysicalPages(hprocess : super::HANDLE, numberofpages : super::PULONG_PTR, pagearray : super::PULONG_PTR) -> windows_sys::core::BOOL);
#[cfg(feature = "shared")]
windows_link::link!("kernel32.dll" "system" fn GetLargePageMinimum() -> super::SIZE_T);
#[cfg(feature = "minwindef")]
windows_link::link!("kernel32.dll" "system" fn GetMemoryErrorHandlingCapabilities(capabilities : super::PULONG) -> windows_sys::core::BOOL);
#[cfg(feature = "shared")]
windows_link::link!("api-ms-win-core-memory-l1-1-9.dll" "system" fn GetNumaNodeMemoryClosestInitiatorNode(targetnodenumber : super::USHORT, initiatornodenumber : *mut super::USHORT) -> windows_sys::core::BOOL);
#[cfg(feature = "shared")]
windows_link::link!("api-ms-win-core-memory-l1-1-9.dll" "system" fn GetNumaNodeMemoryReadBandwidth(targetnodenumber : super::USHORT, initiatornodenumber : super::USHORT, bandwidth : *mut NUMA_NODE_MEMORY_PERFORMANCE_BANDWIDTH) -> windows_sys::core::BOOL);
#[cfg(feature = "shared")]
windows_link::link!("api-ms-win-core-memory-l1-1-9.dll" "system" fn GetNumaNodeMemoryReadLatency(targetnodenumber : super::USHORT, initiatornodenumber : super::USHORT, latency : *mut NUMA_NODE_MEMORY_PERFORMANCE_LATENCY) -> windows_sys::core::BOOL);
#[cfg(feature = "shared")]
windows_link::link!("api-ms-win-core-memory-l1-1-9.dll" "system" fn GetNumaNodeMemoryWriteBandwidth(targetnodenumber : super::USHORT, initiatornodenumber : super::USHORT, bandwidth : *mut NUMA_NODE_MEMORY_PERFORMANCE_BANDWIDTH) -> windows_sys::core::BOOL);
#[cfg(feature = "shared")]
windows_link::link!("api-ms-win-core-memory-l1-1-9.dll" "system" fn GetNumaNodeMemoryWriteLatency(targetnodenumber : super::USHORT, initiatornodenumber : super::USHORT, latency : *mut NUMA_NODE_MEMORY_PERFORMANCE_LATENCY) -> windows_sys::core::BOOL);
#[cfg(all(feature = "basetsd", feature = "winnt"))]
windows_link::link!("kernel32.dll" "system" fn GetProcessWorkingSetSize(hprocess : super::HANDLE, lpminimumworkingsetsize : super::PSIZE_T, lpmaximumworkingsetsize : super::PSIZE_T) -> windows_sys::core::BOOL);
#[cfg(all(feature = "basetsd", feature = "minwindef", feature = "winnt"))]
windows_link::link!("kernel32.dll" "system" fn GetProcessWorkingSetSizeEx(hprocess : super::HANDLE, lpminimumworkingsetsize : super::PSIZE_T, lpmaximumworkingsetsize : super::PSIZE_T, flags : super::PDWORD) -> windows_sys::core::BOOL);
#[cfg(all(feature = "basetsd", feature = "minwindef"))]
windows_link::link!("kernel32.dll" "system" fn GetSystemFileCacheSize(lpminimumfilecachesize : super::PSIZE_T, lpmaximumfilecachesize : super::PSIZE_T, lpflags : super::PDWORD) -> windows_sys::core::BOOL);
#[cfg(all(feature = "minwindef", feature = "shared", feature = "winnt"))]
windows_link::link!("kernel32.dll" "system" fn GetWriteWatch(dwflags : super::DWORD, lpbaseaddress : super::PVOID, dwregionsize : super::SIZE_T, lpaddresses : *mut super::PVOID, lpdwcount : *mut super::ULONG_PTR, lpdwgranularity : super::LPDWORD) -> super::UINT);
#[cfg(all(feature = "shared", feature = "winnt"))]
windows_link::link!("kernel32.dll" "system" fn MapUserPhysicalPages(virtualaddress : super::PVOID, numberofpages : super::ULONG_PTR, pagearray : super::PULONG_PTR) -> windows_sys::core::BOOL);
#[cfg(all(feature = "minwindef", feature = "shared", feature = "winnt"))]
windows_link::link!("kernel32.dll" "system" fn MapViewOfFile(hfilemappingobject : super::HANDLE, dwdesiredaccess : super::DWORD, dwfileoffsethigh : super::DWORD, dwfileoffsetlow : super::DWORD, dwnumberofbytestomap : super::SIZE_T) -> super::LPVOID);
#[cfg(all(feature = "shared", feature = "winnt"))]
windows_link::link!("api-ms-win-core-memory-l1-1-6.dll" "system" fn MapViewOfFile3(filemapping : super::HANDLE, process : super::HANDLE, baseaddress : super::PVOID, offset : super::ULONG64, viewsize : super::SIZE_T, allocationtype : super::ULONG, pageprotection : super::ULONG, extendedparameters : *mut super::MEM_EXTENDED_PARAMETER, parametercount : super::ULONG) -> super::PVOID);
#[cfg(all(feature = "shared", feature = "winnt"))]
windows_link::link!("api-ms-win-core-memory-l1-1-6.dll" "system" fn MapViewOfFile3FromApp(filemapping : super::HANDLE, process : super::HANDLE, baseaddress : super::PVOID, offset : super::ULONG64, viewsize : super::SIZE_T, allocationtype : super::ULONG, pageprotection : super::ULONG, extendedparameters : *mut super::MEM_EXTENDED_PARAMETER, parametercount : super::ULONG) -> super::PVOID);
#[cfg(all(feature = "minwindef", feature = "shared", feature = "winnt"))]
windows_link::link!("kernel32.dll" "system" fn MapViewOfFileEx(hfilemappingobject : super::HANDLE, dwdesiredaccess : super::DWORD, dwfileoffsethigh : super::DWORD, dwfileoffsetlow : super::DWORD, dwnumberofbytestomap : super::SIZE_T, lpbaseaddress : super::LPVOID) -> super::LPVOID);
#[cfg(all(feature = "shared", feature = "winnt"))]
windows_link::link!("kernel32.dll" "system" fn MapViewOfFileFromApp(hfilemappingobject : super::HANDLE, desiredaccess : super::ULONG, fileoffset : super::ULONG64, numberofbytestomap : super::SIZE_T) -> super::PVOID);
#[cfg(all(feature = "shared", feature = "winnt"))]
windows_link::link!("api-ms-win-core-memory-l1-1-5.dll" "system" fn MapViewOfFileNuma2(filemappinghandle : super::HANDLE, processhandle : super::HANDLE, offset : super::ULONG64, baseaddress : super::PVOID, viewsize : super::SIZE_T, allocationtype : super::ULONG, pageprotection : super::ULONG, preferrednode : super::ULONG) -> super::PVOID);
#[cfg(all(feature = "shared", feature = "winnt"))]
windows_link::link!("kernel32.dll" "system" fn OfferVirtualMemory(virtualaddress : super::PVOID, size : super::SIZE_T, priority : OFFER_PRIORITY) -> super::DWORD);
#[cfg(all(feature = "shared", feature = "winnt"))]
windows_link::link!("api-ms-win-core-memory-l1-1-8.dll" "system" fn OpenDedicatedMemoryPartition(partition : super::HANDLE, dedicatedmemorytypeid : super::ULONG64, desiredaccess : super::ACCESS_MASK, inherithandle : windows_sys::core::BOOL) -> super::HANDLE);
#[cfg(all(feature = "shared", feature = "winnt"))]
windows_link::link!("api-ms-win-core-memory-l1-1-3.dll" "system" fn OpenFileMappingFromApp(desiredaccess : super::ULONG, inherithandle : windows_sys::core::BOOL, name : windows_sys::core::PCWSTR) -> super::HANDLE);
#[cfg(all(feature = "shared", feature = "winnt"))]
windows_link::link!("kernel32.dll" "system" fn OpenFileMappingW(dwdesiredaccess : super::DWORD, binherithandle : windows_sys::core::BOOL, lpname : windows_sys::core::PCWSTR) -> super::HANDLE);
#[cfg(all(feature = "shared", feature = "winnt"))]
windows_link::link!("kernel32.dll" "system" fn PrefetchVirtualMemory(hprocess : super::HANDLE, numberofentries : super::ULONG_PTR, virtualaddresses : PWIN32_MEMORY_RANGE_ENTRY, flags : super::ULONG) -> windows_sys::core::BOOL);
#[cfg(all(feature = "minwindef", feature = "winnt"))]
windows_link::link!("kernel32.dll" "system" fn QueryMemoryResourceNotification(resourcenotificationhandle : super::HANDLE, resourcestate : super::PBOOL) -> windows_sys::core::BOOL);
#[cfg(all(feature = "shared", feature = "winnt"))]
windows_link::link!("api-ms-win-core-memory-l1-1-8.dll" "system" fn QueryPartitionInformation(partition : super::HANDLE, partitioninformationclass : WIN32_MEMORY_PARTITION_INFORMATION_CLASS, partitioninformation : super::PVOID, partitioninformationlength : super::ULONG) -> windows_sys::core::BOOL);
#[cfg(all(feature = "basetsd", feature = "shared", feature = "winnt"))]
windows_link::link!("api-ms-win-core-memory-l1-1-4.dll" "system" fn QueryVirtualMemoryInformation(process : super::HANDLE, virtualaddress : *const core::ffi::c_void, memoryinformationclass : WIN32_MEMORY_INFORMATION_CLASS, memoryinformation : super::PVOID, memoryinformationsize : super::SIZE_T, returnsize : super::PSIZE_T) -> windows_sys::core::BOOL);
#[cfg(all(feature = "minwindef", feature = "shared", feature = "winnt"))]
windows_link::link!("kernel32.dll" "system" fn ReadProcessMemory(hprocess : super::HANDLE, lpbaseaddress : super::LPCVOID, lpbuffer : super::LPVOID, nsize : super::SIZE_T, lpnumberofbytesread : *mut super::SIZE_T) -> windows_sys::core::BOOL);
#[cfg(feature = "shared")]
windows_link::link!("kernel32.dll" "system" fn ReclaimVirtualMemory(virtualaddress : *const core::ffi::c_void, size : super::SIZE_T) -> super::DWORD);
#[cfg(feature = "winnt")]
windows_link::link!("kernel32.dll" "system" fn RegisterBadMemoryNotification(callback : PBAD_MEMORY_CALLBACK_ROUTINE) -> super::PVOID);
#[cfg(all(feature = "minwindef", feature = "shared"))]
windows_link::link!("kernel32.dll" "system" fn ResetWriteWatch(lpbaseaddress : super::LPVOID, dwregionsize : super::SIZE_T) -> super::UINT);
#[cfg(all(feature = "shared", feature = "winnt"))]
windows_link::link!("api-ms-win-core-memory-l1-1-3.dll" "system" fn SetProcessValidCallTargets(hprocess : super::HANDLE, virtualaddress : super::PVOID, regionsize : super::SIZE_T, numberofoffsets : super::ULONG, offsetinformation : super::PCFG_CALL_TARGET_INFO) -> windows_sys::core::BOOL);
#[cfg(all(feature = "shared", feature = "winnt"))]
windows_link::link!("api-ms-win-core-memory-l1-1-7.dll" "system" fn SetProcessValidCallTargetsForMappedView(process : super::HANDLE, virtualaddress : super::PVOID, regionsize : super::SIZE_T, numberofoffsets : super::ULONG, offsetinformation : super::PCFG_CALL_TARGET_INFO, section : super::HANDLE, expectedfileoffset : super::ULONG64) -> windows_sys::core::BOOL);
#[cfg(all(feature = "shared", feature = "winnt"))]
windows_link::link!("kernel32.dll" "system" fn SetProcessWorkingSetSize(hprocess : super::HANDLE, dwminimumworkingsetsize : super::SIZE_T, dwmaximumworkingsetsize : super::SIZE_T) -> windows_sys::core::BOOL);
#[cfg(all(feature = "shared", feature = "winnt"))]
windows_link::link!("kernel32.dll" "system" fn SetProcessWorkingSetSizeEx(hprocess : super::HANDLE, dwminimumworkingsetsize : super::SIZE_T, dwmaximumworkingsetsize : super::SIZE_T, flags : super::DWORD) -> windows_sys::core::BOOL);
#[cfg(feature = "shared")]
windows_link::link!("kernel32.dll" "system" fn SetSystemFileCacheSize(minimumfilecachesize : super::SIZE_T, maximumfilecachesize : super::SIZE_T, flags : super::DWORD) -> windows_sys::core::BOOL);
#[cfg(feature = "shared")]
windows_link::link!("kernel32.dll" "system" fn UnmapViewOfFile(lpbaseaddress : super::LPCVOID) -> windows_sys::core::BOOL);
#[cfg(all(feature = "shared", feature = "winnt"))]
windows_link::link!("api-ms-win-core-memory-l1-1-5.dll" "system" fn UnmapViewOfFile2(process : super::HANDLE, baseaddress : super::PVOID, unmapflags : super::ULONG) -> windows_sys::core::BOOL);
#[cfg(all(feature = "shared", feature = "winnt"))]
windows_link::link!("kernel32.dll" "system" fn UnmapViewOfFileEx(baseaddress : super::PVOID, unmapflags : super::ULONG) -> windows_sys::core::BOOL);
#[cfg(feature = "winnt")]
windows_link::link!("kernel32.dll" "system" fn UnregisterBadMemoryNotification(registrationhandle : super::PVOID) -> windows_sys::core::BOOL);
#[cfg(all(feature = "minwindef", feature = "shared"))]
windows_link::link!("kernel32.dll" "system" fn VirtualAlloc(lpaddress : super::LPVOID, dwsize : super::SIZE_T, flallocationtype : super::DWORD, flprotect : super::DWORD) -> super::LPVOID);
#[cfg(all(feature = "shared", feature = "winnt"))]
windows_link::link!("api-ms-win-core-memory-l1-1-6.dll" "system" fn VirtualAlloc2(process : super::HANDLE, baseaddress : super::PVOID, size : super::SIZE_T, allocationtype : super::ULONG, pageprotection : super::ULONG, extendedparameters : *mut super::MEM_EXTENDED_PARAMETER, parametercount : super::ULONG) -> super::PVOID);
#[cfg(all(feature = "shared", feature = "winnt"))]
windows_link::link!("api-ms-win-core-memory-l1-1-6.dll" "system" fn VirtualAlloc2FromApp(process : super::HANDLE, baseaddress : super::PVOID, size : super::SIZE_T, allocationtype : super::ULONG, pageprotection : super::ULONG, extendedparameters : *mut super::MEM_EXTENDED_PARAMETER, parametercount : super::ULONG) -> super::PVOID);
#[cfg(all(feature = "minwindef", feature = "shared", feature = "winnt"))]
windows_link::link!("kernel32.dll" "system" fn VirtualAllocEx(hprocess : super::HANDLE, lpaddress : super::LPVOID, dwsize : super::SIZE_T, flallocationtype : super::DWORD, flprotect : super::DWORD) -> super::LPVOID);
#[cfg(all(feature = "minwindef", feature = "shared", feature = "winnt"))]
windows_link::link!("kernel32.dll" "system" fn VirtualAllocExNuma(hprocess : super::HANDLE, lpaddress : super::LPVOID, dwsize : super::SIZE_T, flallocationtype : super::DWORD, flprotect : super::DWORD, nndpreferred : super::DWORD) -> super::LPVOID);
#[cfg(all(feature = "shared", feature = "winnt"))]
windows_link::link!("api-ms-win-core-memory-l1-1-3.dll" "system" fn VirtualAllocFromApp(baseaddress : super::PVOID, size : super::SIZE_T, allocationtype : super::ULONG, protection : super::ULONG) -> super::PVOID);
#[cfg(all(feature = "minwindef", feature = "shared"))]
windows_link::link!("kernel32.dll" "system" fn VirtualFree(lpaddress : super::LPVOID, dwsize : super::SIZE_T, dwfreetype : super::DWORD) -> windows_sys::core::BOOL);
#[cfg(all(feature = "minwindef", feature = "shared", feature = "winnt"))]
windows_link::link!("kernel32.dll" "system" fn VirtualFreeEx(hprocess : super::HANDLE, lpaddress : super::LPVOID, dwsize : super::SIZE_T, dwfreetype : super::DWORD) -> windows_sys::core::BOOL);
#[cfg(all(feature = "minwindef", feature = "shared"))]
windows_link::link!("kernel32.dll" "system" fn VirtualLock(lpaddress : super::LPVOID, dwsize : super::SIZE_T) -> windows_sys::core::BOOL);
#[cfg(all(feature = "minwindef", feature = "shared"))]
windows_link::link!("kernel32.dll" "system" fn VirtualProtect(lpaddress : super::LPVOID, dwsize : super::SIZE_T, flnewprotect : super::DWORD, lpfloldprotect : super::PDWORD) -> windows_sys::core::BOOL);
#[cfg(all(feature = "minwindef", feature = "shared", feature = "winnt"))]
windows_link::link!("kernel32.dll" "system" fn VirtualProtectEx(hprocess : super::HANDLE, lpaddress : super::LPVOID, dwsize : super::SIZE_T, flnewprotect : super::DWORD, lpfloldprotect : super::PDWORD) -> windows_sys::core::BOOL);
#[cfg(all(feature = "minwindef", feature = "shared", feature = "winnt"))]
windows_link::link!("api-ms-win-core-memory-l1-1-3.dll" "system" fn VirtualProtectFromApp(address : super::PVOID, size : super::SIZE_T, newprotection : super::ULONG, oldprotection : super::PULONG) -> windows_sys::core::BOOL);
#[cfg(all(feature = "shared", feature = "winnt"))]
windows_link::link!("kernel32.dll" "system" fn VirtualQuery(lpaddress : super::LPCVOID, lpbuffer : super::PMEMORY_BASIC_INFORMATION, dwlength : super::SIZE_T) -> super::SIZE_T);
#[cfg(all(feature = "shared", feature = "winnt"))]
windows_link::link!("kernel32.dll" "system" fn VirtualQueryEx(hprocess : super::HANDLE, lpaddress : super::LPCVOID, lpbuffer : super::PMEMORY_BASIC_INFORMATION, dwlength : super::SIZE_T) -> super::SIZE_T);
#[cfg(all(feature = "minwindef", feature = "shared"))]
windows_link::link!("kernel32.dll" "system" fn VirtualUnlock(lpaddress : super::LPVOID, dwsize : super::SIZE_T) -> windows_sys::core::BOOL);
#[cfg(all(feature = "minwindef", feature = "shared", feature = "winnt"))]
windows_link::link!("api-ms-win-core-memory-l1-1-5.dll" "system" fn VirtualUnlockEx(process : super::HANDLE, address : super::LPVOID, size : super::SIZE_T) -> windows_sys::core::BOOL);
#[cfg(all(feature = "minwindef", feature = "shared", feature = "winnt"))]
windows_link::link!("kernel32.dll" "system" fn WriteProcessMemory(hprocess : super::HANDLE, lpbaseaddress : super::LPVOID, lpbuffer : super::LPCVOID, nsize : super::SIZE_T, lpnumberofbyteswritten : *mut super::SIZE_T) -> windows_sys::core::BOOL);
pub type BAD_MEMORY_CALLBACK_ROUTINE = Option<unsafe extern "system" fn()>;
pub const FILE_CACHE_MAX_HARD_DISABLE: i32 = 2;
pub const FILE_CACHE_MAX_HARD_ENABLE: i32 = 1;
pub const FILE_CACHE_MIN_HARD_DISABLE: i32 = 8;
pub const FILE_CACHE_MIN_HARD_ENABLE: i32 = 4;
pub const FILE_MAP_ALL_ACCESS: i32 = 983071;
pub const FILE_MAP_COPY: i32 = 1;
pub const FILE_MAP_EXECUTE: i32 = 32;
pub const FILE_MAP_LARGE_PAGES: i32 = 536870912;
pub const FILE_MAP_READ: i32 = 4;
pub const FILE_MAP_RESERVE: u32 = 2147483648;
pub const FILE_MAP_TARGETS_INVALID: i32 = 1073741824;
pub const FILE_MAP_WRITE: i32 = 2;
pub const HighMemoryResourceNotification: MEMORY_RESOURCE_NOTIFICATION_TYPE = 1;
pub const LowMemoryResourceNotification: MEMORY_RESOURCE_NOTIFICATION_TYPE = 0;
pub const MEHC_PATROL_SCRUBBER_PRESENT: i32 = 1;
pub type MEMORY_RESOURCE_NOTIFICATION_TYPE = i32;
pub const MemoryPartitionDedicatedMemoryInfo: WIN32_MEMORY_PARTITION_INFORMATION_CLASS = 1;
pub const MemoryPartitionInfo: WIN32_MEMORY_PARTITION_INFORMATION_CLASS = 0;
pub const MemoryRegionInfo: WIN32_MEMORY_INFORMATION_CLASS = 0;
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Default)]
pub struct NUMA_NODE_MEMORY_PERFORMANCE_BANDWIDTH {
    pub Condition: NUMA_NODE_MEMORY_PERFORMANCE_MEASUREMENT_CONDITION,
    pub Bandwidth: super::ULONGLONG,
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Default)]
pub struct NUMA_NODE_MEMORY_PERFORMANCE_LATENCY {
    pub Condition: NUMA_NODE_MEMORY_PERFORMANCE_MEASUREMENT_CONDITION,
    pub Latency: super::ULONGLONG,
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Default)]
pub struct NUMA_NODE_MEMORY_PERFORMANCE_MEASUREMENT_CONDITION {
    pub Flags: NUMA_NODE_MEMORY_PERFORMANCE_MEASUREMENT_CONDITION_0,
    pub MinTransferSizeInBytes: super::ULONGLONG,
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Default)]
pub struct NUMA_NODE_MEMORY_PERFORMANCE_MEASUREMENT_CONDITION_0 {
    pub _bitfield: u8,
}
pub type OFFER_PRIORITY = i32;
pub type PBAD_MEMORY_CALLBACK_ROUTINE = Option<unsafe extern "system" fn()>;
#[cfg(all(feature = "shared", feature = "winnt"))]
pub type PWIN32_MEMORY_RANGE_ENTRY = *mut WIN32_MEMORY_RANGE_ENTRY;
pub const VmOfferPriorityBelowNormal: OFFER_PRIORITY = 3;
pub const VmOfferPriorityLow: OFFER_PRIORITY = 2;
pub const VmOfferPriorityNormal: OFFER_PRIORITY = 4;
pub const VmOfferPriorityVeryLow: OFFER_PRIORITY = 1;
pub type WIN32_MEMORY_INFORMATION_CLASS = i32;
#[repr(C, align(8))]
#[cfg(feature = "shared")]
#[derive(Clone, Copy)]
pub struct WIN32_MEMORY_PARTITION_INFORMATION {
    pub Flags: super::ULONG,
    pub NumaNode: super::ULONG,
    pub Channel: super::ULONG,
    pub NumberOfNumaNodes: super::ULONG,
    pub ResidentAvailablePages: super::ULONG64,
    pub CommittedPages: super::ULONG64,
    pub CommitLimit: super::ULONG64,
    pub PeakCommitment: super::ULONG64,
    pub TotalNumberOfPages: super::ULONG64,
    pub AvailablePages: super::ULONG64,
    pub ZeroPages: super::ULONG64,
    pub FreePages: super::ULONG64,
    pub StandbyPages: super::ULONG64,
    pub Reserved: [super::ULONG64; 16],
    pub MaximumCommitLimit: super::ULONG64,
    pub Reserved2: super::ULONG64,
    pub PartitionId: super::ULONG,
}
#[cfg(feature = "shared")]
impl Default for WIN32_MEMORY_PARTITION_INFORMATION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub type WIN32_MEMORY_PARTITION_INFORMATION_CLASS = i32;
#[repr(C)]
#[cfg(all(feature = "shared", feature = "winnt"))]
#[derive(Clone, Copy, Default)]
pub struct WIN32_MEMORY_RANGE_ENTRY {
    pub VirtualAddress: super::PVOID,
    pub NumberOfBytes: super::SIZE_T,
}
#[repr(C)]
#[cfg(all(feature = "shared", feature = "winnt"))]
#[derive(Clone, Copy)]
pub struct WIN32_MEMORY_REGION_INFORMATION {
    pub AllocationBase: super::PVOID,
    pub AllocationProtect: super::ULONG,
    pub Anonymous: WIN32_MEMORY_REGION_INFORMATION_0,
    pub RegionSize: super::SIZE_T,
    pub CommitSize: super::SIZE_T,
}
#[cfg(all(feature = "shared", feature = "winnt"))]
impl Default for WIN32_MEMORY_REGION_INFORMATION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "shared", feature = "winnt"))]
#[derive(Clone, Copy)]
pub union WIN32_MEMORY_REGION_INFORMATION_0 {
    pub Flags: super::ULONG,
    pub Anonymous: WIN32_MEMORY_REGION_INFORMATION_0_0,
}
#[cfg(all(feature = "shared", feature = "winnt"))]
impl Default for WIN32_MEMORY_REGION_INFORMATION_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "shared", feature = "winnt"))]
#[derive(Clone, Copy, Default)]
pub struct WIN32_MEMORY_REGION_INFORMATION_0_0 {
    pub _bitfield: u32,
}

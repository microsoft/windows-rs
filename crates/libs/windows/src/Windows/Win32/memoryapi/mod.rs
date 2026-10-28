#[cfg(all(feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn AllocateUserPhysicalPages(hprocess: super::HANDLE, numberofpages: super::PULONG_PTR, pagearray: super::PULONG_PTR) -> windows_core::BOOL {
    windows_core::link!("kernel32.dll" "system" fn AllocateUserPhysicalPages(hprocess : super::HANDLE, numberofpages : super::PULONG_PTR, pagearray : super::PULONG_PTR) -> windows_core::BOOL);
    unsafe { AllocateUserPhysicalPages(hprocess, numberofpages as _, pagearray as _) }
}
#[cfg(all(feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn AllocateUserPhysicalPages2(objecthandle: super::HANDLE, numberofpages: super::PULONG_PTR, pagearray: super::PULONG_PTR, mut extendedparameters: Option<&mut [super::MEM_EXTENDED_PARAMETER]>) -> windows_core::BOOL {
    windows_core::link!("api-ms-win-core-memory-l1-1-8.dll" "system" fn AllocateUserPhysicalPages2(objecthandle : super::HANDLE, numberofpages : super::PULONG_PTR, pagearray : super::PULONG_PTR, extendedparameters : super::PMEM_EXTENDED_PARAMETER, extendedparametercount : super::ULONG) -> windows_core::BOOL);
    unsafe { AllocateUserPhysicalPages2(objecthandle, numberofpages as _, pagearray as _, core::mem::transmute(extendedparameters.as_mut().map_or(core::ptr::null_mut(), |slice| slice.as_mut_ptr())), extendedparameters.as_deref().map_or(0, |slice| slice.len().try_into().unwrap())) }
}
#[cfg(all(feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn AllocateUserPhysicalPagesNuma(hprocess: super::HANDLE, numberofpages: super::PULONG_PTR, pagearray: super::PULONG_PTR, nndpreferred: super::DWORD) -> windows_core::BOOL {
    windows_core::link!("kernel32.dll" "system" fn AllocateUserPhysicalPagesNuma(hprocess : super::HANDLE, numberofpages : super::PULONG_PTR, pagearray : super::PULONG_PTR, nndpreferred : super::DWORD) -> windows_core::BOOL);
    unsafe { AllocateUserPhysicalPagesNuma(hprocess, numberofpages as _, pagearray as _, nndpreferred) }
}
#[cfg(all(feature = "minwinbase", feature = "minwindef", feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn CreateFileMapping2<P6>(file: super::HANDLE, securityattributes: Option<*const super::SECURITY_ATTRIBUTES>, desiredaccess: super::ULONG, pageprotection: super::ULONG, allocationattributes: super::ULONG, maximumsize: super::ULONG64, name: P6, mut extendedparameters: Option<&mut [super::MEM_EXTENDED_PARAMETER]>) -> super::HANDLE
where
    P6: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("api-ms-win-core-memory-l1-1-7.dll" "system" fn CreateFileMapping2(file : super::HANDLE, securityattributes : *const super::SECURITY_ATTRIBUTES, desiredaccess : super::ULONG, pageprotection : super::ULONG, allocationattributes : super::ULONG, maximumsize : super::ULONG64, name : windows_core::PCWSTR, extendedparameters : *mut super::MEM_EXTENDED_PARAMETER, parametercount : super::ULONG) -> super::HANDLE);
    unsafe { CreateFileMapping2(file, securityattributes.unwrap_or(core::mem::zeroed()) as _, desiredaccess, pageprotection, allocationattributes, maximumsize, name.param().abi(), extendedparameters.as_mut().map_or(core::ptr::null_mut(), |slice| slice.as_mut_ptr()), extendedparameters.as_deref().map_or(0, |slice| slice.len().try_into().unwrap())) }
}
#[cfg(all(feature = "minwinbase", feature = "minwindef", feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn CreateFileMappingFromApp<P4>(hfile: super::HANDLE, securityattributes: Option<super::PSECURITY_ATTRIBUTES>, pageprotection: super::ULONG, maximumsize: super::ULONG64, name: P4) -> super::HANDLE
where
    P4: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("kernel32.dll" "system" fn CreateFileMappingFromApp(hfile : super::HANDLE, securityattributes : super::PSECURITY_ATTRIBUTES, pageprotection : super::ULONG, maximumsize : super::ULONG64, name : windows_core::PCWSTR) -> super::HANDLE);
    unsafe { CreateFileMappingFromApp(hfile, securityattributes.unwrap_or(core::mem::zeroed()) as _, pageprotection, maximumsize, name.param().abi()) }
}
#[cfg(all(feature = "minwinbase", feature = "minwindef", feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn CreateFileMappingNumaW<P5>(hfile: super::HANDLE, lpfilemappingattributes: Option<super::LPSECURITY_ATTRIBUTES>, flprotect: super::DWORD, dwmaximumsizehigh: super::DWORD, dwmaximumsizelow: super::DWORD, lpname: P5, nndpreferred: super::DWORD) -> super::HANDLE
where
    P5: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("kernel32.dll" "system" fn CreateFileMappingNumaW(hfile : super::HANDLE, lpfilemappingattributes : super::LPSECURITY_ATTRIBUTES, flprotect : super::DWORD, dwmaximumsizehigh : super::DWORD, dwmaximumsizelow : super::DWORD, lpname : windows_core::PCWSTR, nndpreferred : super::DWORD) -> super::HANDLE);
    unsafe { CreateFileMappingNumaW(hfile, lpfilemappingattributes.unwrap_or(core::mem::zeroed()) as _, flprotect, dwmaximumsizehigh, dwmaximumsizelow, lpname.param().abi(), nndpreferred) }
}
#[cfg(all(feature = "minwinbase", feature = "minwindef", feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn CreateFileMappingW<P5>(hfile: super::HANDLE, lpfilemappingattributes: Option<super::LPSECURITY_ATTRIBUTES>, flprotect: super::DWORD, dwmaximumsizehigh: super::DWORD, dwmaximumsizelow: super::DWORD, lpname: P5) -> super::HANDLE
where
    P5: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("kernel32.dll" "system" fn CreateFileMappingW(hfile : super::HANDLE, lpfilemappingattributes : super::LPSECURITY_ATTRIBUTES, flprotect : super::DWORD, dwmaximumsizehigh : super::DWORD, dwmaximumsizelow : super::DWORD, lpname : windows_core::PCWSTR) -> super::HANDLE);
    unsafe { CreateFileMappingW(hfile, lpfilemappingattributes.unwrap_or(core::mem::zeroed()) as _, flprotect, dwmaximumsizehigh, dwmaximumsizelow, lpname.param().abi()) }
}
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn CreateMemoryResourceNotification(notificationtype: MEMORY_RESOURCE_NOTIFICATION_TYPE) -> super::HANDLE {
    windows_core::link!("kernel32.dll" "system" fn CreateMemoryResourceNotification(notificationtype : MEMORY_RESOURCE_NOTIFICATION_TYPE) -> super::HANDLE);
    unsafe { CreateMemoryResourceNotification(notificationtype) }
}
#[cfg(all(feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn DiscardVirtualMemory(virtualaddress: &mut [u8]) -> super::DWORD {
    windows_core::link!("kernel32.dll" "system" fn DiscardVirtualMemory(virtualaddress : super::PVOID, size : super::SIZE_T) -> super::DWORD);
    unsafe { DiscardVirtualMemory(core::mem::transmute(virtualaddress.as_mut_ptr()), virtualaddress.len().try_into().unwrap()) }
}
#[cfg(feature = "shared")]
#[inline]
pub unsafe fn FlushViewOfFile(lpbaseaddress: super::LPCVOID, dwnumberofbytestoflush: super::SIZE_T) -> windows_core::BOOL {
    windows_core::link!("kernel32.dll" "system" fn FlushViewOfFile(lpbaseaddress : super::LPCVOID, dwnumberofbytestoflush : super::SIZE_T) -> windows_core::BOOL);
    unsafe { FlushViewOfFile(lpbaseaddress, dwnumberofbytestoflush) }
}
#[cfg(all(feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn FreeUserPhysicalPages(hprocess: super::HANDLE, numberofpages: super::PULONG_PTR, pagearray: super::PULONG_PTR) -> windows_core::BOOL {
    windows_core::link!("kernel32.dll" "system" fn FreeUserPhysicalPages(hprocess : super::HANDLE, numberofpages : super::PULONG_PTR, pagearray : super::PULONG_PTR) -> windows_core::BOOL);
    unsafe { FreeUserPhysicalPages(hprocess, numberofpages as _, pagearray) }
}
#[cfg(feature = "shared")]
#[inline]
pub unsafe fn GetLargePageMinimum() -> super::SIZE_T {
    windows_core::link!("kernel32.dll" "system" fn GetLargePageMinimum() -> super::SIZE_T);
    unsafe { GetLargePageMinimum() }
}
#[cfg(feature = "minwindef")]
#[inline]
pub unsafe fn GetMemoryErrorHandlingCapabilities(capabilities: super::PULONG) -> windows_core::BOOL {
    windows_core::link!("kernel32.dll" "system" fn GetMemoryErrorHandlingCapabilities(capabilities : super::PULONG) -> windows_core::BOOL);
    unsafe { GetMemoryErrorHandlingCapabilities(capabilities as _) }
}
#[cfg(feature = "shared")]
#[inline]
pub unsafe fn GetNumaNodeMemoryClosestInitiatorNode(targetnodenumber: super::USHORT, initiatornodenumber: *mut super::USHORT) -> windows_core::BOOL {
    windows_core::link!("api-ms-win-core-memory-l1-1-9.dll" "system" fn GetNumaNodeMemoryClosestInitiatorNode(targetnodenumber : super::USHORT, initiatornodenumber : *mut super::USHORT) -> windows_core::BOOL);
    unsafe { GetNumaNodeMemoryClosestInitiatorNode(targetnodenumber, initiatornodenumber as _) }
}
#[cfg(feature = "shared")]
#[inline]
pub unsafe fn GetNumaNodeMemoryReadBandwidth(targetnodenumber: super::USHORT, initiatornodenumber: super::USHORT, bandwidth: *mut NUMA_NODE_MEMORY_PERFORMANCE_BANDWIDTH) -> windows_core::BOOL {
    windows_core::link!("api-ms-win-core-memory-l1-1-9.dll" "system" fn GetNumaNodeMemoryReadBandwidth(targetnodenumber : super::USHORT, initiatornodenumber : super::USHORT, bandwidth : *mut NUMA_NODE_MEMORY_PERFORMANCE_BANDWIDTH) -> windows_core::BOOL);
    unsafe { GetNumaNodeMemoryReadBandwidth(targetnodenumber, initiatornodenumber, bandwidth as _) }
}
#[cfg(feature = "shared")]
#[inline]
pub unsafe fn GetNumaNodeMemoryReadLatency(targetnodenumber: super::USHORT, initiatornodenumber: super::USHORT, latency: *mut NUMA_NODE_MEMORY_PERFORMANCE_LATENCY) -> windows_core::BOOL {
    windows_core::link!("api-ms-win-core-memory-l1-1-9.dll" "system" fn GetNumaNodeMemoryReadLatency(targetnodenumber : super::USHORT, initiatornodenumber : super::USHORT, latency : *mut NUMA_NODE_MEMORY_PERFORMANCE_LATENCY) -> windows_core::BOOL);
    unsafe { GetNumaNodeMemoryReadLatency(targetnodenumber, initiatornodenumber, latency as _) }
}
#[cfg(feature = "shared")]
#[inline]
pub unsafe fn GetNumaNodeMemoryWriteBandwidth(targetnodenumber: super::USHORT, initiatornodenumber: super::USHORT, bandwidth: *mut NUMA_NODE_MEMORY_PERFORMANCE_BANDWIDTH) -> windows_core::BOOL {
    windows_core::link!("api-ms-win-core-memory-l1-1-9.dll" "system" fn GetNumaNodeMemoryWriteBandwidth(targetnodenumber : super::USHORT, initiatornodenumber : super::USHORT, bandwidth : *mut NUMA_NODE_MEMORY_PERFORMANCE_BANDWIDTH) -> windows_core::BOOL);
    unsafe { GetNumaNodeMemoryWriteBandwidth(targetnodenumber, initiatornodenumber, bandwidth as _) }
}
#[cfg(feature = "shared")]
#[inline]
pub unsafe fn GetNumaNodeMemoryWriteLatency(targetnodenumber: super::USHORT, initiatornodenumber: super::USHORT, latency: *mut NUMA_NODE_MEMORY_PERFORMANCE_LATENCY) -> windows_core::BOOL {
    windows_core::link!("api-ms-win-core-memory-l1-1-9.dll" "system" fn GetNumaNodeMemoryWriteLatency(targetnodenumber : super::USHORT, initiatornodenumber : super::USHORT, latency : *mut NUMA_NODE_MEMORY_PERFORMANCE_LATENCY) -> windows_core::BOOL);
    unsafe { GetNumaNodeMemoryWriteLatency(targetnodenumber, initiatornodenumber, latency as _) }
}
#[cfg(all(feature = "basetsd", feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn GetProcessWorkingSetSize(hprocess: super::HANDLE, lpminimumworkingsetsize: super::PSIZE_T, lpmaximumworkingsetsize: super::PSIZE_T) -> windows_core::BOOL {
    windows_core::link!("kernel32.dll" "system" fn GetProcessWorkingSetSize(hprocess : super::HANDLE, lpminimumworkingsetsize : super::PSIZE_T, lpmaximumworkingsetsize : super::PSIZE_T) -> windows_core::BOOL);
    unsafe { GetProcessWorkingSetSize(hprocess, lpminimumworkingsetsize as _, lpmaximumworkingsetsize as _) }
}
#[cfg(all(feature = "basetsd", feature = "minwindef", feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn GetProcessWorkingSetSizeEx(hprocess: super::HANDLE, lpminimumworkingsetsize: super::PSIZE_T, lpmaximumworkingsetsize: super::PSIZE_T, flags: super::PDWORD) -> windows_core::BOOL {
    windows_core::link!("kernel32.dll" "system" fn GetProcessWorkingSetSizeEx(hprocess : super::HANDLE, lpminimumworkingsetsize : super::PSIZE_T, lpmaximumworkingsetsize : super::PSIZE_T, flags : super::PDWORD) -> windows_core::BOOL);
    unsafe { GetProcessWorkingSetSizeEx(hprocess, lpminimumworkingsetsize as _, lpmaximumworkingsetsize as _, flags as _) }
}
#[cfg(all(feature = "basetsd", feature = "minwindef", feature = "shared"))]
#[inline]
pub unsafe fn GetSystemFileCacheSize(lpminimumfilecachesize: super::PSIZE_T, lpmaximumfilecachesize: super::PSIZE_T, lpflags: super::PDWORD) -> windows_core::BOOL {
    windows_core::link!("kernel32.dll" "system" fn GetSystemFileCacheSize(lpminimumfilecachesize : super::PSIZE_T, lpmaximumfilecachesize : super::PSIZE_T, lpflags : super::PDWORD) -> windows_core::BOOL);
    unsafe { GetSystemFileCacheSize(lpminimumfilecachesize as _, lpmaximumfilecachesize as _, lpflags as _) }
}
#[cfg(all(feature = "minwindef", feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn GetWriteWatch(dwflags: super::DWORD, lpbaseaddress: super::PVOID, dwregionsize: super::SIZE_T, lpaddresses: Option<*mut super::PVOID>, lpdwcount: Option<*mut super::ULONG_PTR>, lpdwgranularity: Option<super::LPDWORD>) -> super::UINT {
    windows_core::link!("kernel32.dll" "system" fn GetWriteWatch(dwflags : super::DWORD, lpbaseaddress : super::PVOID, dwregionsize : super::SIZE_T, lpaddresses : *mut super::PVOID, lpdwcount : *mut super::ULONG_PTR, lpdwgranularity : super::LPDWORD) -> super::UINT);
    unsafe { GetWriteWatch(dwflags, lpbaseaddress, dwregionsize, lpaddresses.unwrap_or(core::mem::zeroed()) as _, lpdwcount.unwrap_or(core::mem::zeroed()) as _, lpdwgranularity.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(all(feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn MapUserPhysicalPages(virtualaddress: super::PVOID, pagearray: Option<&[u32]>) -> windows_core::BOOL {
    windows_core::link!("kernel32.dll" "system" fn MapUserPhysicalPages(virtualaddress : super::PVOID, numberofpages : super::ULONG_PTR, pagearray : super::PULONG_PTR) -> windows_core::BOOL);
    unsafe { MapUserPhysicalPages(virtualaddress, pagearray.map_or(0, |slice| slice.len().try_into().unwrap()), core::mem::transmute(pagearray.map_or(core::ptr::null(), |slice| slice.as_ptr()))) }
}
#[cfg(all(feature = "minwindef", feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn MapViewOfFile(hfilemappingobject: super::HANDLE, dwdesiredaccess: super::DWORD, dwfileoffsethigh: super::DWORD, dwfileoffsetlow: super::DWORD, dwnumberofbytestomap: super::SIZE_T) -> super::LPVOID {
    windows_core::link!("kernel32.dll" "system" fn MapViewOfFile(hfilemappingobject : super::HANDLE, dwdesiredaccess : super::DWORD, dwfileoffsethigh : super::DWORD, dwfileoffsetlow : super::DWORD, dwnumberofbytestomap : super::SIZE_T) -> super::LPVOID);
    unsafe { MapViewOfFile(hfilemappingobject, dwdesiredaccess, dwfileoffsethigh, dwfileoffsetlow, dwnumberofbytestomap) }
}
#[cfg(all(feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn MapViewOfFile3(filemapping: super::HANDLE, process: Option<super::HANDLE>, baseaddress: Option<super::PVOID>, offset: super::ULONG64, viewsize: super::SIZE_T, allocationtype: super::ULONG, pageprotection: super::ULONG, mut extendedparameters: Option<&mut [super::MEM_EXTENDED_PARAMETER]>) -> super::PVOID {
    windows_core::link!("api-ms-win-core-memory-l1-1-6.dll" "system" fn MapViewOfFile3(filemapping : super::HANDLE, process : super::HANDLE, baseaddress : super::PVOID, offset : super::ULONG64, viewsize : super::SIZE_T, allocationtype : super::ULONG, pageprotection : super::ULONG, extendedparameters : *mut super::MEM_EXTENDED_PARAMETER, parametercount : super::ULONG) -> super::PVOID);
    unsafe { MapViewOfFile3(filemapping, process.unwrap_or(core::mem::zeroed()) as _, baseaddress.unwrap_or(core::mem::zeroed()) as _, offset, viewsize, allocationtype, pageprotection, extendedparameters.as_mut().map_or(core::ptr::null_mut(), |slice| slice.as_mut_ptr()), extendedparameters.as_deref().map_or(0, |slice| slice.len().try_into().unwrap())) }
}
#[cfg(all(feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn MapViewOfFile3FromApp(filemapping: super::HANDLE, process: Option<super::HANDLE>, baseaddress: Option<super::PVOID>, offset: super::ULONG64, viewsize: super::SIZE_T, allocationtype: super::ULONG, pageprotection: super::ULONG, mut extendedparameters: Option<&mut [super::MEM_EXTENDED_PARAMETER]>) -> super::PVOID {
    windows_core::link!("api-ms-win-core-memory-l1-1-6.dll" "system" fn MapViewOfFile3FromApp(filemapping : super::HANDLE, process : super::HANDLE, baseaddress : super::PVOID, offset : super::ULONG64, viewsize : super::SIZE_T, allocationtype : super::ULONG, pageprotection : super::ULONG, extendedparameters : *mut super::MEM_EXTENDED_PARAMETER, parametercount : super::ULONG) -> super::PVOID);
    unsafe { MapViewOfFile3FromApp(filemapping, process.unwrap_or(core::mem::zeroed()) as _, baseaddress.unwrap_or(core::mem::zeroed()) as _, offset, viewsize, allocationtype, pageprotection, extendedparameters.as_mut().map_or(core::ptr::null_mut(), |slice| slice.as_mut_ptr()), extendedparameters.as_deref().map_or(0, |slice| slice.len().try_into().unwrap())) }
}
#[cfg(all(feature = "minwindef", feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn MapViewOfFileEx(hfilemappingobject: super::HANDLE, dwdesiredaccess: super::DWORD, dwfileoffsethigh: super::DWORD, dwfileoffsetlow: super::DWORD, dwnumberofbytestomap: super::SIZE_T, lpbaseaddress: Option<super::LPVOID>) -> super::LPVOID {
    windows_core::link!("kernel32.dll" "system" fn MapViewOfFileEx(hfilemappingobject : super::HANDLE, dwdesiredaccess : super::DWORD, dwfileoffsethigh : super::DWORD, dwfileoffsetlow : super::DWORD, dwnumberofbytestomap : super::SIZE_T, lpbaseaddress : super::LPVOID) -> super::LPVOID);
    unsafe { MapViewOfFileEx(hfilemappingobject, dwdesiredaccess, dwfileoffsethigh, dwfileoffsetlow, dwnumberofbytestomap, lpbaseaddress.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(all(feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn MapViewOfFileFromApp(hfilemappingobject: super::HANDLE, desiredaccess: super::ULONG, fileoffset: super::ULONG64, numberofbytestomap: super::SIZE_T) -> super::PVOID {
    windows_core::link!("kernel32.dll" "system" fn MapViewOfFileFromApp(hfilemappingobject : super::HANDLE, desiredaccess : super::ULONG, fileoffset : super::ULONG64, numberofbytestomap : super::SIZE_T) -> super::PVOID);
    unsafe { MapViewOfFileFromApp(hfilemappingobject, desiredaccess, fileoffset, numberofbytestomap) }
}
#[cfg(all(feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn MapViewOfFileNuma2(filemappinghandle: super::HANDLE, processhandle: super::HANDLE, offset: super::ULONG64, baseaddress: Option<super::PVOID>, viewsize: super::SIZE_T, allocationtype: super::ULONG, pageprotection: super::ULONG, preferrednode: super::ULONG) -> super::PVOID {
    windows_core::link!("api-ms-win-core-memory-l1-1-5.dll" "system" fn MapViewOfFileNuma2(filemappinghandle : super::HANDLE, processhandle : super::HANDLE, offset : super::ULONG64, baseaddress : super::PVOID, viewsize : super::SIZE_T, allocationtype : super::ULONG, pageprotection : super::ULONG, preferrednode : super::ULONG) -> super::PVOID);
    unsafe { MapViewOfFileNuma2(filemappinghandle, processhandle, offset, baseaddress.unwrap_or(core::mem::zeroed()) as _, viewsize, allocationtype, pageprotection, preferrednode) }
}
#[cfg(all(feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn OfferVirtualMemory(virtualaddress: &mut [u8], priority: OFFER_PRIORITY) -> super::DWORD {
    windows_core::link!("kernel32.dll" "system" fn OfferVirtualMemory(virtualaddress : super::PVOID, size : super::SIZE_T, priority : OFFER_PRIORITY) -> super::DWORD);
    unsafe { OfferVirtualMemory(core::mem::transmute(virtualaddress.as_mut_ptr()), virtualaddress.len().try_into().unwrap(), priority) }
}
#[cfg(all(feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn OpenDedicatedMemoryPartition(partition: super::HANDLE, dedicatedmemorytypeid: super::ULONG64, desiredaccess: super::ACCESS_MASK, inherithandle: bool) -> super::HANDLE {
    windows_core::link!("api-ms-win-core-memory-l1-1-8.dll" "system" fn OpenDedicatedMemoryPartition(partition : super::HANDLE, dedicatedmemorytypeid : super::ULONG64, desiredaccess : super::ACCESS_MASK, inherithandle : windows_core::BOOL) -> super::HANDLE);
    unsafe { OpenDedicatedMemoryPartition(partition, dedicatedmemorytypeid, desiredaccess, inherithandle.into()) }
}
#[cfg(all(feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn OpenFileMappingFromApp<P2>(desiredaccess: super::ULONG, inherithandle: bool, name: P2) -> super::HANDLE
where
    P2: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("api-ms-win-core-memory-l1-1-3.dll" "system" fn OpenFileMappingFromApp(desiredaccess : super::ULONG, inherithandle : windows_core::BOOL, name : windows_core::PCWSTR) -> super::HANDLE);
    unsafe { OpenFileMappingFromApp(desiredaccess, inherithandle.into(), name.param().abi()) }
}
#[cfg(all(feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn OpenFileMappingW<P2>(dwdesiredaccess: super::DWORD, binherithandle: bool, lpname: P2) -> super::HANDLE
where
    P2: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("kernel32.dll" "system" fn OpenFileMappingW(dwdesiredaccess : super::DWORD, binherithandle : windows_core::BOOL, lpname : windows_core::PCWSTR) -> super::HANDLE);
    unsafe { OpenFileMappingW(dwdesiredaccess, binherithandle.into(), lpname.param().abi()) }
}
#[cfg(all(feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn PrefetchVirtualMemory(hprocess: super::HANDLE, virtualaddresses: &[WIN32_MEMORY_RANGE_ENTRY], flags: super::ULONG) -> windows_core::BOOL {
    windows_core::link!("kernel32.dll" "system" fn PrefetchVirtualMemory(hprocess : super::HANDLE, numberofentries : super::ULONG_PTR, virtualaddresses : PWIN32_MEMORY_RANGE_ENTRY, flags : super::ULONG) -> windows_core::BOOL);
    unsafe { PrefetchVirtualMemory(hprocess, virtualaddresses.len().try_into().unwrap(), core::mem::transmute(virtualaddresses.as_ptr()), flags) }
}
#[cfg(all(feature = "minwindef", feature = "winnt"))]
#[inline]
pub unsafe fn QueryMemoryResourceNotification(resourcenotificationhandle: super::HANDLE, resourcestate: super::PBOOL) -> windows_core::BOOL {
    windows_core::link!("kernel32.dll" "system" fn QueryMemoryResourceNotification(resourcenotificationhandle : super::HANDLE, resourcestate : super::PBOOL) -> windows_core::BOOL);
    unsafe { QueryMemoryResourceNotification(resourcenotificationhandle, resourcestate as _) }
}
#[cfg(all(feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn QueryPartitionInformation(partition: super::HANDLE, partitioninformationclass: WIN32_MEMORY_PARTITION_INFORMATION_CLASS, partitioninformation: super::PVOID, partitioninformationlength: super::ULONG) -> windows_core::BOOL {
    windows_core::link!("api-ms-win-core-memory-l1-1-8.dll" "system" fn QueryPartitionInformation(partition : super::HANDLE, partitioninformationclass : WIN32_MEMORY_PARTITION_INFORMATION_CLASS, partitioninformation : super::PVOID, partitioninformationlength : super::ULONG) -> windows_core::BOOL);
    unsafe { QueryPartitionInformation(partition, partitioninformationclass, partitioninformation as _, partitioninformationlength) }
}
#[cfg(all(feature = "basetsd", feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn QueryVirtualMemoryInformation(process: super::HANDLE, virtualaddress: *const core::ffi::c_void, memoryinformationclass: WIN32_MEMORY_INFORMATION_CLASS, memoryinformation: super::PVOID, memoryinformationsize: super::SIZE_T, returnsize: Option<super::PSIZE_T>) -> windows_core::BOOL {
    windows_core::link!("api-ms-win-core-memory-l1-1-4.dll" "system" fn QueryVirtualMemoryInformation(process : super::HANDLE, virtualaddress : *const core::ffi::c_void, memoryinformationclass : WIN32_MEMORY_INFORMATION_CLASS, memoryinformation : super::PVOID, memoryinformationsize : super::SIZE_T, returnsize : super::PSIZE_T) -> windows_core::BOOL);
    unsafe { QueryVirtualMemoryInformation(process, virtualaddress, memoryinformationclass, memoryinformation as _, memoryinformationsize, returnsize.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(all(feature = "minwindef", feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn ReadProcessMemory(hprocess: super::HANDLE, lpbaseaddress: super::LPCVOID, lpbuffer: super::LPVOID, nsize: super::SIZE_T, lpnumberofbytesread: Option<*mut super::SIZE_T>) -> windows_core::BOOL {
    windows_core::link!("kernel32.dll" "system" fn ReadProcessMemory(hprocess : super::HANDLE, lpbaseaddress : super::LPCVOID, lpbuffer : super::LPVOID, nsize : super::SIZE_T, lpnumberofbytesread : *mut super::SIZE_T) -> windows_core::BOOL);
    unsafe { ReadProcessMemory(hprocess, lpbaseaddress, lpbuffer as _, nsize, lpnumberofbytesread.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(feature = "shared")]
#[inline]
pub unsafe fn ReclaimVirtualMemory(virtualaddress: &[u8]) -> super::DWORD {
    windows_core::link!("kernel32.dll" "system" fn ReclaimVirtualMemory(virtualaddress : *const core::ffi::c_void, size : super::SIZE_T) -> super::DWORD);
    unsafe { ReclaimVirtualMemory(core::mem::transmute(virtualaddress.as_ptr()), virtualaddress.len().try_into().unwrap()) }
}
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn RegisterBadMemoryNotification(callback: PBAD_MEMORY_CALLBACK_ROUTINE) -> super::PVOID {
    windows_core::link!("kernel32.dll" "system" fn RegisterBadMemoryNotification(callback : PBAD_MEMORY_CALLBACK_ROUTINE) -> super::PVOID);
    unsafe { RegisterBadMemoryNotification(callback) }
}
#[cfg(all(feature = "minwindef", feature = "shared"))]
#[inline]
pub unsafe fn ResetWriteWatch(lpbaseaddress: super::LPVOID, dwregionsize: super::SIZE_T) -> super::UINT {
    windows_core::link!("kernel32.dll" "system" fn ResetWriteWatch(lpbaseaddress : super::LPVOID, dwregionsize : super::SIZE_T) -> super::UINT);
    unsafe { ResetWriteWatch(lpbaseaddress, dwregionsize) }
}
#[cfg(all(feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn SetProcessValidCallTargets(hprocess: super::HANDLE, virtualaddress: super::PVOID, regionsize: super::SIZE_T, offsetinformation: &mut [super::CFG_CALL_TARGET_INFO]) -> windows_core::BOOL {
    windows_core::link!("api-ms-win-core-memory-l1-1-3.dll" "system" fn SetProcessValidCallTargets(hprocess : super::HANDLE, virtualaddress : super::PVOID, regionsize : super::SIZE_T, numberofoffsets : super::ULONG, offsetinformation : super::PCFG_CALL_TARGET_INFO) -> windows_core::BOOL);
    unsafe { SetProcessValidCallTargets(hprocess, virtualaddress, regionsize, offsetinformation.len().try_into().unwrap(), core::mem::transmute(offsetinformation.as_mut_ptr())) }
}
#[cfg(all(feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn SetProcessValidCallTargetsForMappedView(process: super::HANDLE, virtualaddress: super::PVOID, regionsize: super::SIZE_T, offsetinformation: &mut [super::CFG_CALL_TARGET_INFO], section: super::HANDLE, expectedfileoffset: super::ULONG64) -> windows_core::BOOL {
    windows_core::link!("api-ms-win-core-memory-l1-1-7.dll" "system" fn SetProcessValidCallTargetsForMappedView(process : super::HANDLE, virtualaddress : super::PVOID, regionsize : super::SIZE_T, numberofoffsets : super::ULONG, offsetinformation : super::PCFG_CALL_TARGET_INFO, section : super::HANDLE, expectedfileoffset : super::ULONG64) -> windows_core::BOOL);
    unsafe { SetProcessValidCallTargetsForMappedView(process, virtualaddress, regionsize, offsetinformation.len().try_into().unwrap(), core::mem::transmute(offsetinformation.as_mut_ptr()), section, expectedfileoffset) }
}
#[cfg(all(feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn SetProcessWorkingSetSize(hprocess: super::HANDLE, dwminimumworkingsetsize: super::SIZE_T, dwmaximumworkingsetsize: super::SIZE_T) -> windows_core::BOOL {
    windows_core::link!("kernel32.dll" "system" fn SetProcessWorkingSetSize(hprocess : super::HANDLE, dwminimumworkingsetsize : super::SIZE_T, dwmaximumworkingsetsize : super::SIZE_T) -> windows_core::BOOL);
    unsafe { SetProcessWorkingSetSize(hprocess, dwminimumworkingsetsize, dwmaximumworkingsetsize) }
}
#[cfg(all(feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn SetProcessWorkingSetSizeEx(hprocess: super::HANDLE, dwminimumworkingsetsize: super::SIZE_T, dwmaximumworkingsetsize: super::SIZE_T, flags: super::DWORD) -> windows_core::BOOL {
    windows_core::link!("kernel32.dll" "system" fn SetProcessWorkingSetSizeEx(hprocess : super::HANDLE, dwminimumworkingsetsize : super::SIZE_T, dwmaximumworkingsetsize : super::SIZE_T, flags : super::DWORD) -> windows_core::BOOL);
    unsafe { SetProcessWorkingSetSizeEx(hprocess, dwminimumworkingsetsize, dwmaximumworkingsetsize, flags) }
}
#[cfg(feature = "shared")]
#[inline]
pub unsafe fn SetSystemFileCacheSize(minimumfilecachesize: super::SIZE_T, maximumfilecachesize: super::SIZE_T, flags: super::DWORD) -> windows_core::BOOL {
    windows_core::link!("kernel32.dll" "system" fn SetSystemFileCacheSize(minimumfilecachesize : super::SIZE_T, maximumfilecachesize : super::SIZE_T, flags : super::DWORD) -> windows_core::BOOL);
    unsafe { SetSystemFileCacheSize(minimumfilecachesize, maximumfilecachesize, flags) }
}
#[cfg(feature = "shared")]
#[inline]
pub unsafe fn UnmapViewOfFile(lpbaseaddress: super::LPCVOID) -> windows_core::BOOL {
    windows_core::link!("kernel32.dll" "system" fn UnmapViewOfFile(lpbaseaddress : super::LPCVOID) -> windows_core::BOOL);
    unsafe { UnmapViewOfFile(lpbaseaddress) }
}
#[cfg(all(feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn UnmapViewOfFile2(process: super::HANDLE, baseaddress: super::PVOID, unmapflags: super::ULONG) -> windows_core::BOOL {
    windows_core::link!("api-ms-win-core-memory-l1-1-5.dll" "system" fn UnmapViewOfFile2(process : super::HANDLE, baseaddress : super::PVOID, unmapflags : super::ULONG) -> windows_core::BOOL);
    unsafe { UnmapViewOfFile2(process, baseaddress, unmapflags) }
}
#[cfg(all(feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn UnmapViewOfFileEx(baseaddress: super::PVOID, unmapflags: super::ULONG) -> windows_core::BOOL {
    windows_core::link!("kernel32.dll" "system" fn UnmapViewOfFileEx(baseaddress : super::PVOID, unmapflags : super::ULONG) -> windows_core::BOOL);
    unsafe { UnmapViewOfFileEx(baseaddress, unmapflags) }
}
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn UnregisterBadMemoryNotification(registrationhandle: super::PVOID) -> windows_core::BOOL {
    windows_core::link!("kernel32.dll" "system" fn UnregisterBadMemoryNotification(registrationhandle : super::PVOID) -> windows_core::BOOL);
    unsafe { UnregisterBadMemoryNotification(registrationhandle) }
}
#[cfg(all(feature = "minwindef", feature = "shared"))]
#[inline]
pub unsafe fn VirtualAlloc(lpaddress: Option<super::LPVOID>, dwsize: super::SIZE_T, flallocationtype: super::DWORD, flprotect: super::DWORD) -> super::LPVOID {
    windows_core::link!("kernel32.dll" "system" fn VirtualAlloc(lpaddress : super::LPVOID, dwsize : super::SIZE_T, flallocationtype : super::DWORD, flprotect : super::DWORD) -> super::LPVOID);
    unsafe { VirtualAlloc(lpaddress.unwrap_or(core::mem::zeroed()) as _, dwsize, flallocationtype, flprotect) }
}
#[cfg(all(feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn VirtualAlloc2(process: Option<super::HANDLE>, baseaddress: Option<super::PVOID>, size: super::SIZE_T, allocationtype: super::ULONG, pageprotection: super::ULONG, mut extendedparameters: Option<&mut [super::MEM_EXTENDED_PARAMETER]>) -> super::PVOID {
    windows_core::link!("api-ms-win-core-memory-l1-1-6.dll" "system" fn VirtualAlloc2(process : super::HANDLE, baseaddress : super::PVOID, size : super::SIZE_T, allocationtype : super::ULONG, pageprotection : super::ULONG, extendedparameters : *mut super::MEM_EXTENDED_PARAMETER, parametercount : super::ULONG) -> super::PVOID);
    unsafe { VirtualAlloc2(process.unwrap_or(core::mem::zeroed()) as _, baseaddress.unwrap_or(core::mem::zeroed()) as _, size, allocationtype, pageprotection, extendedparameters.as_mut().map_or(core::ptr::null_mut(), |slice| slice.as_mut_ptr()), extendedparameters.as_deref().map_or(0, |slice| slice.len().try_into().unwrap())) }
}
#[cfg(all(feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn VirtualAlloc2FromApp(process: Option<super::HANDLE>, baseaddress: Option<super::PVOID>, size: super::SIZE_T, allocationtype: super::ULONG, pageprotection: super::ULONG, mut extendedparameters: Option<&mut [super::MEM_EXTENDED_PARAMETER]>) -> super::PVOID {
    windows_core::link!("api-ms-win-core-memory-l1-1-6.dll" "system" fn VirtualAlloc2FromApp(process : super::HANDLE, baseaddress : super::PVOID, size : super::SIZE_T, allocationtype : super::ULONG, pageprotection : super::ULONG, extendedparameters : *mut super::MEM_EXTENDED_PARAMETER, parametercount : super::ULONG) -> super::PVOID);
    unsafe { VirtualAlloc2FromApp(process.unwrap_or(core::mem::zeroed()) as _, baseaddress.unwrap_or(core::mem::zeroed()) as _, size, allocationtype, pageprotection, extendedparameters.as_mut().map_or(core::ptr::null_mut(), |slice| slice.as_mut_ptr()), extendedparameters.as_deref().map_or(0, |slice| slice.len().try_into().unwrap())) }
}
#[cfg(all(feature = "minwindef", feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn VirtualAllocEx(hprocess: super::HANDLE, lpaddress: Option<super::LPVOID>, dwsize: super::SIZE_T, flallocationtype: super::DWORD, flprotect: super::DWORD) -> super::LPVOID {
    windows_core::link!("kernel32.dll" "system" fn VirtualAllocEx(hprocess : super::HANDLE, lpaddress : super::LPVOID, dwsize : super::SIZE_T, flallocationtype : super::DWORD, flprotect : super::DWORD) -> super::LPVOID);
    unsafe { VirtualAllocEx(hprocess, lpaddress.unwrap_or(core::mem::zeroed()) as _, dwsize, flallocationtype, flprotect) }
}
#[cfg(all(feature = "minwindef", feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn VirtualAllocExNuma(hprocess: super::HANDLE, lpaddress: Option<super::LPVOID>, dwsize: super::SIZE_T, flallocationtype: super::DWORD, flprotect: super::DWORD, nndpreferred: super::DWORD) -> super::LPVOID {
    windows_core::link!("kernel32.dll" "system" fn VirtualAllocExNuma(hprocess : super::HANDLE, lpaddress : super::LPVOID, dwsize : super::SIZE_T, flallocationtype : super::DWORD, flprotect : super::DWORD, nndpreferred : super::DWORD) -> super::LPVOID);
    unsafe { VirtualAllocExNuma(hprocess, lpaddress.unwrap_or(core::mem::zeroed()) as _, dwsize, flallocationtype, flprotect, nndpreferred) }
}
#[cfg(all(feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn VirtualAllocFromApp(baseaddress: Option<super::PVOID>, size: super::SIZE_T, allocationtype: super::ULONG, protection: super::ULONG) -> super::PVOID {
    windows_core::link!("api-ms-win-core-memory-l1-1-3.dll" "system" fn VirtualAllocFromApp(baseaddress : super::PVOID, size : super::SIZE_T, allocationtype : super::ULONG, protection : super::ULONG) -> super::PVOID);
    unsafe { VirtualAllocFromApp(baseaddress.unwrap_or(core::mem::zeroed()) as _, size, allocationtype, protection) }
}
#[cfg(all(feature = "minwindef", feature = "shared"))]
#[inline]
pub unsafe fn VirtualFree(lpaddress: super::LPVOID, dwsize: super::SIZE_T, dwfreetype: super::DWORD) -> windows_core::BOOL {
    windows_core::link!("kernel32.dll" "system" fn VirtualFree(lpaddress : super::LPVOID, dwsize : super::SIZE_T, dwfreetype : super::DWORD) -> windows_core::BOOL);
    unsafe { VirtualFree(lpaddress as _, dwsize, dwfreetype) }
}
#[cfg(all(feature = "minwindef", feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn VirtualFreeEx(hprocess: super::HANDLE, lpaddress: super::LPVOID, dwsize: super::SIZE_T, dwfreetype: super::DWORD) -> windows_core::BOOL {
    windows_core::link!("kernel32.dll" "system" fn VirtualFreeEx(hprocess : super::HANDLE, lpaddress : super::LPVOID, dwsize : super::SIZE_T, dwfreetype : super::DWORD) -> windows_core::BOOL);
    unsafe { VirtualFreeEx(hprocess, lpaddress as _, dwsize, dwfreetype) }
}
#[cfg(all(feature = "minwindef", feature = "shared"))]
#[inline]
pub unsafe fn VirtualLock(lpaddress: super::LPVOID, dwsize: super::SIZE_T) -> windows_core::BOOL {
    windows_core::link!("kernel32.dll" "system" fn VirtualLock(lpaddress : super::LPVOID, dwsize : super::SIZE_T) -> windows_core::BOOL);
    unsafe { VirtualLock(lpaddress, dwsize) }
}
#[cfg(all(feature = "minwindef", feature = "shared"))]
#[inline]
pub unsafe fn VirtualProtect(lpaddress: super::LPVOID, dwsize: super::SIZE_T, flnewprotect: super::DWORD, lpfloldprotect: super::PDWORD) -> windows_core::BOOL {
    windows_core::link!("kernel32.dll" "system" fn VirtualProtect(lpaddress : super::LPVOID, dwsize : super::SIZE_T, flnewprotect : super::DWORD, lpfloldprotect : super::PDWORD) -> windows_core::BOOL);
    unsafe { VirtualProtect(lpaddress, dwsize, flnewprotect, lpfloldprotect as _) }
}
#[cfg(all(feature = "minwindef", feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn VirtualProtectEx(hprocess: super::HANDLE, lpaddress: super::LPVOID, dwsize: super::SIZE_T, flnewprotect: super::DWORD, lpfloldprotect: super::PDWORD) -> windows_core::BOOL {
    windows_core::link!("kernel32.dll" "system" fn VirtualProtectEx(hprocess : super::HANDLE, lpaddress : super::LPVOID, dwsize : super::SIZE_T, flnewprotect : super::DWORD, lpfloldprotect : super::PDWORD) -> windows_core::BOOL);
    unsafe { VirtualProtectEx(hprocess, lpaddress, dwsize, flnewprotect, lpfloldprotect as _) }
}
#[cfg(all(feature = "minwindef", feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn VirtualProtectFromApp(address: super::PVOID, size: super::SIZE_T, newprotection: super::ULONG, oldprotection: super::PULONG) -> windows_core::BOOL {
    windows_core::link!("api-ms-win-core-memory-l1-1-3.dll" "system" fn VirtualProtectFromApp(address : super::PVOID, size : super::SIZE_T, newprotection : super::ULONG, oldprotection : super::PULONG) -> windows_core::BOOL);
    unsafe { VirtualProtectFromApp(address, size, newprotection, oldprotection as _) }
}
#[cfg(all(feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn VirtualQuery(lpaddress: Option<super::LPCVOID>, lpbuffer: super::PMEMORY_BASIC_INFORMATION, dwlength: super::SIZE_T) -> super::SIZE_T {
    windows_core::link!("kernel32.dll" "system" fn VirtualQuery(lpaddress : super::LPCVOID, lpbuffer : super::PMEMORY_BASIC_INFORMATION, dwlength : super::SIZE_T) -> super::SIZE_T);
    unsafe { VirtualQuery(lpaddress.unwrap_or(core::mem::zeroed()) as _, lpbuffer as _, dwlength) }
}
#[cfg(all(feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn VirtualQueryEx(hprocess: super::HANDLE, lpaddress: Option<super::LPCVOID>, lpbuffer: super::PMEMORY_BASIC_INFORMATION, dwlength: super::SIZE_T) -> super::SIZE_T {
    windows_core::link!("kernel32.dll" "system" fn VirtualQueryEx(hprocess : super::HANDLE, lpaddress : super::LPCVOID, lpbuffer : super::PMEMORY_BASIC_INFORMATION, dwlength : super::SIZE_T) -> super::SIZE_T);
    unsafe { VirtualQueryEx(hprocess, lpaddress.unwrap_or(core::mem::zeroed()) as _, lpbuffer as _, dwlength) }
}
#[cfg(all(feature = "minwindef", feature = "shared"))]
#[inline]
pub unsafe fn VirtualUnlock(lpaddress: super::LPVOID, dwsize: super::SIZE_T) -> windows_core::BOOL {
    windows_core::link!("kernel32.dll" "system" fn VirtualUnlock(lpaddress : super::LPVOID, dwsize : super::SIZE_T) -> windows_core::BOOL);
    unsafe { VirtualUnlock(lpaddress, dwsize) }
}
#[cfg(all(feature = "minwindef", feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn VirtualUnlockEx(process: Option<super::HANDLE>, address: super::LPVOID, size: super::SIZE_T) -> windows_core::BOOL {
    windows_core::link!("api-ms-win-core-memory-l1-1-5.dll" "system" fn VirtualUnlockEx(process : super::HANDLE, address : super::LPVOID, size : super::SIZE_T) -> windows_core::BOOL);
    unsafe { VirtualUnlockEx(process.unwrap_or(core::mem::zeroed()) as _, address, size) }
}
#[cfg(all(feature = "minwindef", feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn WriteProcessMemory(hprocess: super::HANDLE, lpbaseaddress: super::LPVOID, lpbuffer: super::LPCVOID, nsize: super::SIZE_T, lpnumberofbyteswritten: Option<*mut super::SIZE_T>) -> windows_core::BOOL {
    windows_core::link!("kernel32.dll" "system" fn WriteProcessMemory(hprocess : super::HANDLE, lpbaseaddress : super::LPVOID, lpbuffer : super::LPCVOID, nsize : super::SIZE_T, lpnumberofbyteswritten : *mut super::SIZE_T) -> windows_core::BOOL);
    unsafe { WriteProcessMemory(hprocess, lpbaseaddress, lpbuffer, nsize, lpnumberofbyteswritten.unwrap_or(core::mem::zeroed()) as _) }
}
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
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct NUMA_NODE_MEMORY_PERFORMANCE_BANDWIDTH {
    pub Condition: NUMA_NODE_MEMORY_PERFORMANCE_MEASUREMENT_CONDITION,
    pub Bandwidth: super::ULONGLONG,
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct NUMA_NODE_MEMORY_PERFORMANCE_LATENCY {
    pub Condition: NUMA_NODE_MEMORY_PERFORMANCE_MEASUREMENT_CONDITION,
    pub Latency: super::ULONGLONG,
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct NUMA_NODE_MEMORY_PERFORMANCE_MEASUREMENT_CONDITION {
    pub Flags: NUMA_NODE_MEMORY_PERFORMANCE_MEASUREMENT_CONDITION_0,
    pub MinTransferSizeInBytes: super::ULONGLONG,
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct NUMA_NODE_MEMORY_PERFORMANCE_MEASUREMENT_CONDITION_0 {
    pub _bitfield: u8,
}
#[cfg(feature = "shared")]
impl NUMA_NODE_MEMORY_PERFORMANCE_MEASUREMENT_CONDITION_0 {
    pub fn MinTransferSizeToAchieveValues(&self) -> super::UCHAR {
        (self._bitfield) & 1
    }
    pub fn set_MinTransferSizeToAchieveValues(&mut self, value: super::UCHAR) {
        self._bitfield = (self._bitfield & !1) | (value & 1);
    }
    pub fn NonSequentialTransfers(&self) -> super::UCHAR {
        (self._bitfield >> 1) & 1
    }
    pub fn set_NonSequentialTransfers(&mut self, value: super::UCHAR) {
        self._bitfield = (self._bitfield & !(1 << 1)) | ((value & 1) << 1);
    }
    pub fn Reserved(&self) -> super::UCHAR {
        self._bitfield >> 2
    }
    pub fn set_Reserved(&mut self, value: super::UCHAR) {
        self._bitfield = (self._bitfield & !(63 << 2)) | ((value & 63) << 2);
    }
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
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
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
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
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
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct WIN32_MEMORY_REGION_INFORMATION_0_0 {
    pub _bitfield: u32,
}
#[cfg(all(feature = "shared", feature = "winnt"))]
impl WIN32_MEMORY_REGION_INFORMATION_0_0 {
    pub fn Private(&self) -> super::ULONG {
        (self._bitfield) & 1
    }
    pub fn set_Private(&mut self, value: super::ULONG) {
        self._bitfield = (self._bitfield & !1) | (value & 1);
    }
    pub fn MappedDataFile(&self) -> super::ULONG {
        (self._bitfield >> 1) & 1
    }
    pub fn set_MappedDataFile(&mut self, value: super::ULONG) {
        self._bitfield = (self._bitfield & !(1 << 1)) | ((value & 1) << 1);
    }
    pub fn MappedImage(&self) -> super::ULONG {
        (self._bitfield >> 2) & 1
    }
    pub fn set_MappedImage(&mut self, value: super::ULONG) {
        self._bitfield = (self._bitfield & !(1 << 2)) | ((value & 1) << 2);
    }
    pub fn MappedPageFile(&self) -> super::ULONG {
        (self._bitfield >> 3) & 1
    }
    pub fn set_MappedPageFile(&mut self, value: super::ULONG) {
        self._bitfield = (self._bitfield & !(1 << 3)) | ((value & 1) << 3);
    }
    pub fn MappedPhysical(&self) -> super::ULONG {
        (self._bitfield >> 4) & 1
    }
    pub fn set_MappedPhysical(&mut self, value: super::ULONG) {
        self._bitfield = (self._bitfield & !(1 << 4)) | ((value & 1) << 4);
    }
    pub fn DirectMapped(&self) -> super::ULONG {
        (self._bitfield >> 5) & 1
    }
    pub fn set_DirectMapped(&mut self, value: super::ULONG) {
        self._bitfield = (self._bitfield & !(1 << 5)) | ((value & 1) << 5);
    }
    pub fn Reserved(&self) -> super::ULONG {
        self._bitfield >> 6
    }
    pub fn set_Reserved(&mut self, value: super::ULONG) {
        self._bitfield = (self._bitfield & !(67108863 << 6)) | ((value & 67108863) << 6);
    }
}

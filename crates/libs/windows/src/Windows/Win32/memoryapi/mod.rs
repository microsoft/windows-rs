#[cfg(target_arch = "x86")]
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn AllocateUserPhysicalPages(hprocess: super::winnt::HANDLE, numberofpages: *mut u32, pagearray: *mut u32) -> windows_core::BOOL {
    windows_core::link!("kernel32.dll" "system" fn AllocateUserPhysicalPages(hprocess : super::winnt::HANDLE, numberofpages : *mut u32, pagearray : *mut u32) -> windows_core::BOOL);
    unsafe { AllocateUserPhysicalPages(hprocess, numberofpages as _, pagearray as _) }
}
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn AllocateUserPhysicalPages(hprocess: super::winnt::HANDLE, numberofpages: *mut u64, pagearray: *mut u64) -> windows_core::BOOL {
    windows_core::link!("kernel32.dll" "system" fn AllocateUserPhysicalPages(hprocess : super::winnt::HANDLE, numberofpages : *mut u64, pagearray : *mut u64) -> windows_core::BOOL);
    unsafe { AllocateUserPhysicalPages(hprocess, numberofpages as _, pagearray as _) }
}
#[cfg(target_arch = "x86")]
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn AllocateUserPhysicalPages2(objecthandle: super::winnt::HANDLE, numberofpages: *mut u32, pagearray: *mut u32, extendedparameters: Option<&mut [super::winnt::MEM_EXTENDED_PARAMETER]>) -> windows_core::BOOL {
    windows_core::link!("api-ms-win-core-memory-l1-1-8.dll" "system" fn AllocateUserPhysicalPages2(objecthandle : super::winnt::HANDLE, numberofpages : *mut u32, pagearray : *mut u32, extendedparameters : *mut super::winnt::MEM_EXTENDED_PARAMETER, extendedparametercount : u32) -> windows_core::BOOL);
    unsafe { AllocateUserPhysicalPages2(objecthandle, numberofpages as _, pagearray as _, core::mem::transmute(extendedparameters.as_deref().map_or(core::ptr::null(), |slice| slice.as_ptr())), extendedparameters.as_deref().map_or(0, |slice| slice.len().try_into().unwrap())) }
}
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn AllocateUserPhysicalPages2(objecthandle: super::winnt::HANDLE, numberofpages: *mut u64, pagearray: *mut u64, extendedparameters: Option<&mut [super::winnt::MEM_EXTENDED_PARAMETER]>) -> windows_core::BOOL {
    windows_core::link!("api-ms-win-core-memory-l1-1-8.dll" "system" fn AllocateUserPhysicalPages2(objecthandle : super::winnt::HANDLE, numberofpages : *mut u64, pagearray : *mut u64, extendedparameters : *mut super::winnt::MEM_EXTENDED_PARAMETER, extendedparametercount : u32) -> windows_core::BOOL);
    unsafe { AllocateUserPhysicalPages2(objecthandle, numberofpages as _, pagearray as _, core::mem::transmute(extendedparameters.as_deref().map_or(core::ptr::null(), |slice| slice.as_ptr())), extendedparameters.as_deref().map_or(0, |slice| slice.len().try_into().unwrap())) }
}
#[cfg(target_arch = "x86")]
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn AllocateUserPhysicalPagesNuma(hprocess: super::winnt::HANDLE, numberofpages: *mut u32, pagearray: *mut u32, nndpreferred: u32) -> windows_core::BOOL {
    windows_core::link!("kernel32.dll" "system" fn AllocateUserPhysicalPagesNuma(hprocess : super::winnt::HANDLE, numberofpages : *mut u32, pagearray : *mut u32, nndpreferred : u32) -> windows_core::BOOL);
    unsafe { AllocateUserPhysicalPagesNuma(hprocess, numberofpages as _, pagearray as _, nndpreferred) }
}
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn AllocateUserPhysicalPagesNuma(hprocess: super::winnt::HANDLE, numberofpages: *mut u64, pagearray: *mut u64, nndpreferred: u32) -> windows_core::BOOL {
    windows_core::link!("kernel32.dll" "system" fn AllocateUserPhysicalPagesNuma(hprocess : super::winnt::HANDLE, numberofpages : *mut u64, pagearray : *mut u64, nndpreferred : u32) -> windows_core::BOOL);
    unsafe { AllocateUserPhysicalPagesNuma(hprocess, numberofpages as _, pagearray as _, nndpreferred) }
}
#[cfg(all(feature = "minwinbase", feature = "winnt"))]
#[inline]
pub unsafe fn CreateFileMapping2<P6>(file: super::winnt::HANDLE, securityattributes: Option<*const super::minwinbase::SECURITY_ATTRIBUTES>, desiredaccess: u32, pageprotection: u32, allocationattributes: u32, maximumsize: u64, name: P6, extendedparameters: Option<&mut [super::winnt::MEM_EXTENDED_PARAMETER]>) -> super::winnt::HANDLE
where
    P6: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("api-ms-win-core-memory-l1-1-7.dll" "system" fn CreateFileMapping2(file : super::winnt::HANDLE, securityattributes : *const super::minwinbase::SECURITY_ATTRIBUTES, desiredaccess : u32, pageprotection : u32, allocationattributes : u32, maximumsize : u64, name : windows_core::PCWSTR, extendedparameters : *mut super::winnt::MEM_EXTENDED_PARAMETER, parametercount : u32) -> super::winnt::HANDLE);
    unsafe { CreateFileMapping2(file, securityattributes.unwrap_or(core::mem::zeroed()) as _, desiredaccess, pageprotection, allocationattributes, maximumsize, name.param().abi(), core::mem::transmute(extendedparameters.as_deref().map_or(core::ptr::null(), |slice| slice.as_ptr())), extendedparameters.as_deref().map_or(0, |slice| slice.len().try_into().unwrap())) }
}
#[cfg(all(feature = "minwinbase", feature = "winnt"))]
#[inline]
pub unsafe fn CreateFileMappingFromApp<P4>(hfile: super::winnt::HANDLE, securityattributes: Option<*const super::minwinbase::SECURITY_ATTRIBUTES>, pageprotection: u32, maximumsize: u64, name: P4) -> super::winnt::HANDLE
where
    P4: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("kernel32.dll" "system" fn CreateFileMappingFromApp(hfile : super::winnt::HANDLE, securityattributes : *const super::minwinbase::SECURITY_ATTRIBUTES, pageprotection : u32, maximumsize : u64, name : windows_core::PCWSTR) -> super::winnt::HANDLE);
    unsafe { CreateFileMappingFromApp(hfile, securityattributes.unwrap_or(core::mem::zeroed()) as _, pageprotection, maximumsize, name.param().abi()) }
}
#[cfg(all(feature = "minwinbase", feature = "winnt"))]
#[inline]
pub unsafe fn CreateFileMappingNumaW<P5>(hfile: super::winnt::HANDLE, lpfilemappingattributes: Option<*const super::minwinbase::SECURITY_ATTRIBUTES>, flprotect: u32, dwmaximumsizehigh: u32, dwmaximumsizelow: u32, lpname: P5, nndpreferred: u32) -> super::winnt::HANDLE
where
    P5: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("kernel32.dll" "system" fn CreateFileMappingNumaW(hfile : super::winnt::HANDLE, lpfilemappingattributes : *const super::minwinbase::SECURITY_ATTRIBUTES, flprotect : u32, dwmaximumsizehigh : u32, dwmaximumsizelow : u32, lpname : windows_core::PCWSTR, nndpreferred : u32) -> super::winnt::HANDLE);
    unsafe { CreateFileMappingNumaW(hfile, lpfilemappingattributes.unwrap_or(core::mem::zeroed()) as _, flprotect, dwmaximumsizehigh, dwmaximumsizelow, lpname.param().abi(), nndpreferred) }
}
#[cfg(all(feature = "minwinbase", feature = "winnt"))]
#[inline]
pub unsafe fn CreateFileMappingW<P5>(hfile: super::winnt::HANDLE, lpfilemappingattributes: Option<*const super::minwinbase::SECURITY_ATTRIBUTES>, flprotect: u32, dwmaximumsizehigh: u32, dwmaximumsizelow: u32, lpname: P5) -> super::winnt::HANDLE
where
    P5: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("kernel32.dll" "system" fn CreateFileMappingW(hfile : super::winnt::HANDLE, lpfilemappingattributes : *const super::minwinbase::SECURITY_ATTRIBUTES, flprotect : u32, dwmaximumsizehigh : u32, dwmaximumsizelow : u32, lpname : windows_core::PCWSTR) -> super::winnt::HANDLE);
    unsafe { CreateFileMappingW(hfile, lpfilemappingattributes.unwrap_or(core::mem::zeroed()) as _, flprotect, dwmaximumsizehigh, dwmaximumsizelow, lpname.param().abi()) }
}
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn CreateMemoryResourceNotification(notificationtype: MEMORY_RESOURCE_NOTIFICATION_TYPE) -> super::winnt::HANDLE {
    windows_core::link!("kernel32.dll" "system" fn CreateMemoryResourceNotification(notificationtype : MEMORY_RESOURCE_NOTIFICATION_TYPE) -> super::winnt::HANDLE);
    unsafe { CreateMemoryResourceNotification(notificationtype) }
}
#[inline]
pub unsafe fn DiscardVirtualMemory(virtualaddress: &mut [u8]) -> u32 {
    windows_core::link!("kernel32.dll" "system" fn DiscardVirtualMemory(virtualaddress : *mut core::ffi::c_void, size : usize) -> u32);
    unsafe { DiscardVirtualMemory(core::mem::transmute(virtualaddress.as_ptr()), virtualaddress.len().try_into().unwrap()) }
}
#[inline]
pub unsafe fn FlushViewOfFile(lpbaseaddress: *const core::ffi::c_void, dwnumberofbytestoflush: usize) -> windows_core::BOOL {
    windows_core::link!("kernel32.dll" "system" fn FlushViewOfFile(lpbaseaddress : *const core::ffi::c_void, dwnumberofbytestoflush : usize) -> windows_core::BOOL);
    unsafe { FlushViewOfFile(lpbaseaddress, dwnumberofbytestoflush) }
}
#[cfg(target_arch = "x86")]
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn FreeUserPhysicalPages(hprocess: super::winnt::HANDLE, numberofpages: *mut u32, pagearray: *const u32) -> windows_core::BOOL {
    windows_core::link!("kernel32.dll" "system" fn FreeUserPhysicalPages(hprocess : super::winnt::HANDLE, numberofpages : *mut u32, pagearray : *const u32) -> windows_core::BOOL);
    unsafe { FreeUserPhysicalPages(hprocess, numberofpages as _, pagearray) }
}
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn FreeUserPhysicalPages(hprocess: super::winnt::HANDLE, numberofpages: *mut u64, pagearray: *const u64) -> windows_core::BOOL {
    windows_core::link!("kernel32.dll" "system" fn FreeUserPhysicalPages(hprocess : super::winnt::HANDLE, numberofpages : *mut u64, pagearray : *const u64) -> windows_core::BOOL);
    unsafe { FreeUserPhysicalPages(hprocess, numberofpages as _, pagearray) }
}
#[inline]
pub unsafe fn GetLargePageMinimum() -> usize {
    windows_core::link!("kernel32.dll" "system" fn GetLargePageMinimum() -> usize);
    unsafe { GetLargePageMinimum() }
}
#[inline]
pub unsafe fn GetMemoryErrorHandlingCapabilities(capabilities: *mut u32) -> windows_core::BOOL {
    windows_core::link!("kernel32.dll" "system" fn GetMemoryErrorHandlingCapabilities(capabilities : *mut u32) -> windows_core::BOOL);
    unsafe { GetMemoryErrorHandlingCapabilities(capabilities as _) }
}
#[inline]
pub unsafe fn GetNumaNodeMemoryClosestInitiatorNode(targetnodenumber: u16, initiatornodenumber: *mut u16) -> windows_core::BOOL {
    windows_core::link!("api-ms-win-core-memory-l1-1-9.dll" "system" fn GetNumaNodeMemoryClosestInitiatorNode(targetnodenumber : u16, initiatornodenumber : *mut u16) -> windows_core::BOOL);
    unsafe { GetNumaNodeMemoryClosestInitiatorNode(targetnodenumber, initiatornodenumber as _) }
}
#[inline]
pub unsafe fn GetNumaNodeMemoryReadBandwidth(targetnodenumber: u16, initiatornodenumber: u16, bandwidth: *mut NUMA_NODE_MEMORY_PERFORMANCE_BANDWIDTH) -> windows_core::BOOL {
    windows_core::link!("api-ms-win-core-memory-l1-1-9.dll" "system" fn GetNumaNodeMemoryReadBandwidth(targetnodenumber : u16, initiatornodenumber : u16, bandwidth : *mut NUMA_NODE_MEMORY_PERFORMANCE_BANDWIDTH) -> windows_core::BOOL);
    unsafe { GetNumaNodeMemoryReadBandwidth(targetnodenumber, initiatornodenumber, bandwidth as _) }
}
#[inline]
pub unsafe fn GetNumaNodeMemoryReadLatency(targetnodenumber: u16, initiatornodenumber: u16, latency: *mut NUMA_NODE_MEMORY_PERFORMANCE_LATENCY) -> windows_core::BOOL {
    windows_core::link!("api-ms-win-core-memory-l1-1-9.dll" "system" fn GetNumaNodeMemoryReadLatency(targetnodenumber : u16, initiatornodenumber : u16, latency : *mut NUMA_NODE_MEMORY_PERFORMANCE_LATENCY) -> windows_core::BOOL);
    unsafe { GetNumaNodeMemoryReadLatency(targetnodenumber, initiatornodenumber, latency as _) }
}
#[inline]
pub unsafe fn GetNumaNodeMemoryWriteBandwidth(targetnodenumber: u16, initiatornodenumber: u16, bandwidth: *mut NUMA_NODE_MEMORY_PERFORMANCE_BANDWIDTH) -> windows_core::BOOL {
    windows_core::link!("api-ms-win-core-memory-l1-1-9.dll" "system" fn GetNumaNodeMemoryWriteBandwidth(targetnodenumber : u16, initiatornodenumber : u16, bandwidth : *mut NUMA_NODE_MEMORY_PERFORMANCE_BANDWIDTH) -> windows_core::BOOL);
    unsafe { GetNumaNodeMemoryWriteBandwidth(targetnodenumber, initiatornodenumber, bandwidth as _) }
}
#[inline]
pub unsafe fn GetNumaNodeMemoryWriteLatency(targetnodenumber: u16, initiatornodenumber: u16, latency: *mut NUMA_NODE_MEMORY_PERFORMANCE_LATENCY) -> windows_core::BOOL {
    windows_core::link!("api-ms-win-core-memory-l1-1-9.dll" "system" fn GetNumaNodeMemoryWriteLatency(targetnodenumber : u16, initiatornodenumber : u16, latency : *mut NUMA_NODE_MEMORY_PERFORMANCE_LATENCY) -> windows_core::BOOL);
    unsafe { GetNumaNodeMemoryWriteLatency(targetnodenumber, initiatornodenumber, latency as _) }
}
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn GetProcessWorkingSetSize(hprocess: super::winnt::HANDLE, lpminimumworkingsetsize: *mut usize, lpmaximumworkingsetsize: *mut usize) -> windows_core::BOOL {
    windows_core::link!("kernel32.dll" "system" fn GetProcessWorkingSetSize(hprocess : super::winnt::HANDLE, lpminimumworkingsetsize : *mut usize, lpmaximumworkingsetsize : *mut usize) -> windows_core::BOOL);
    unsafe { GetProcessWorkingSetSize(hprocess, lpminimumworkingsetsize as _, lpmaximumworkingsetsize as _) }
}
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn GetProcessWorkingSetSizeEx(hprocess: super::winnt::HANDLE, lpminimumworkingsetsize: *mut usize, lpmaximumworkingsetsize: *mut usize, flags: *mut u32) -> windows_core::BOOL {
    windows_core::link!("kernel32.dll" "system" fn GetProcessWorkingSetSizeEx(hprocess : super::winnt::HANDLE, lpminimumworkingsetsize : *mut usize, lpmaximumworkingsetsize : *mut usize, flags : *mut u32) -> windows_core::BOOL);
    unsafe { GetProcessWorkingSetSizeEx(hprocess, lpminimumworkingsetsize as _, lpmaximumworkingsetsize as _, flags as _) }
}
#[inline]
pub unsafe fn GetSystemFileCacheSize(lpminimumfilecachesize: *mut usize, lpmaximumfilecachesize: *mut usize, lpflags: *mut u32) -> windows_core::BOOL {
    windows_core::link!("kernel32.dll" "system" fn GetSystemFileCacheSize(lpminimumfilecachesize : *mut usize, lpmaximumfilecachesize : *mut usize, lpflags : *mut u32) -> windows_core::BOOL);
    unsafe { GetSystemFileCacheSize(lpminimumfilecachesize as _, lpmaximumfilecachesize as _, lpflags as _) }
}
#[inline]
pub unsafe fn GetWriteWatch(dwflags: u32, lpbaseaddress: *const core::ffi::c_void, dwregionsize: usize, lpaddresses: Option<*mut *mut core::ffi::c_void>, lpdwcount: Option<*mut usize>, lpdwgranularity: Option<*mut u32>) -> u32 {
    windows_core::link!("kernel32.dll" "system" fn GetWriteWatch(dwflags : u32, lpbaseaddress : *const core::ffi::c_void, dwregionsize : usize, lpaddresses : *mut *mut core::ffi::c_void, lpdwcount : *mut usize, lpdwgranularity : *mut u32) -> u32);
    unsafe { GetWriteWatch(dwflags, lpbaseaddress, dwregionsize, lpaddresses.unwrap_or(core::mem::zeroed()) as _, lpdwcount.unwrap_or(core::mem::zeroed()) as _, lpdwgranularity.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(target_arch = "x86")]
#[inline]
pub unsafe fn MapUserPhysicalPages(virtualaddress: *const core::ffi::c_void, pagearray: Option<&[u32]>) -> windows_core::BOOL {
    windows_core::link!("kernel32.dll" "system" fn MapUserPhysicalPages(virtualaddress : *const core::ffi::c_void, numberofpages : usize, pagearray : *const u32) -> windows_core::BOOL);
    unsafe { MapUserPhysicalPages(virtualaddress, pagearray.map_or(0, |slice| slice.len().try_into().unwrap()), core::mem::transmute(pagearray.map_or(core::ptr::null(), |slice| slice.as_ptr()))) }
}
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[inline]
pub unsafe fn MapUserPhysicalPages(virtualaddress: *const core::ffi::c_void, pagearray: Option<&[u64]>) -> windows_core::BOOL {
    windows_core::link!("kernel32.dll" "system" fn MapUserPhysicalPages(virtualaddress : *const core::ffi::c_void, numberofpages : usize, pagearray : *const u64) -> windows_core::BOOL);
    unsafe { MapUserPhysicalPages(virtualaddress, pagearray.map_or(0, |slice| slice.len().try_into().unwrap()), core::mem::transmute(pagearray.map_or(core::ptr::null(), |slice| slice.as_ptr()))) }
}
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn MapViewOfFile(hfilemappingobject: super::winnt::HANDLE, dwdesiredaccess: u32, dwfileoffsethigh: u32, dwfileoffsetlow: u32, dwnumberofbytestomap: usize) -> *mut core::ffi::c_void {
    windows_core::link!("kernel32.dll" "system" fn MapViewOfFile(hfilemappingobject : super::winnt::HANDLE, dwdesiredaccess : u32, dwfileoffsethigh : u32, dwfileoffsetlow : u32, dwnumberofbytestomap : usize) -> *mut core::ffi::c_void);
    unsafe { MapViewOfFile(hfilemappingobject, dwdesiredaccess, dwfileoffsethigh, dwfileoffsetlow, dwnumberofbytestomap) }
}
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn MapViewOfFile3(filemapping: super::winnt::HANDLE, process: Option<super::winnt::HANDLE>, baseaddress: Option<*const core::ffi::c_void>, offset: u64, viewsize: usize, allocationtype: u32, pageprotection: u32, extendedparameters: Option<&mut [super::winnt::MEM_EXTENDED_PARAMETER]>) -> *mut core::ffi::c_void {
    windows_core::link!("api-ms-win-core-memory-l1-1-6.dll" "system" fn MapViewOfFile3(filemapping : super::winnt::HANDLE, process : super::winnt::HANDLE, baseaddress : *const core::ffi::c_void, offset : u64, viewsize : usize, allocationtype : u32, pageprotection : u32, extendedparameters : *mut super::winnt::MEM_EXTENDED_PARAMETER, parametercount : u32) -> *mut core::ffi::c_void);
    unsafe { MapViewOfFile3(filemapping, process.unwrap_or(core::mem::zeroed()) as _, baseaddress.unwrap_or(core::mem::zeroed()) as _, offset, viewsize, allocationtype, pageprotection, core::mem::transmute(extendedparameters.as_deref().map_or(core::ptr::null(), |slice| slice.as_ptr())), extendedparameters.as_deref().map_or(0, |slice| slice.len().try_into().unwrap())) }
}
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn MapViewOfFile3FromApp(filemapping: super::winnt::HANDLE, process: Option<super::winnt::HANDLE>, baseaddress: Option<*const core::ffi::c_void>, offset: u64, viewsize: usize, allocationtype: u32, pageprotection: u32, extendedparameters: Option<&mut [super::winnt::MEM_EXTENDED_PARAMETER]>) -> *mut core::ffi::c_void {
    windows_core::link!("api-ms-win-core-memory-l1-1-6.dll" "system" fn MapViewOfFile3FromApp(filemapping : super::winnt::HANDLE, process : super::winnt::HANDLE, baseaddress : *const core::ffi::c_void, offset : u64, viewsize : usize, allocationtype : u32, pageprotection : u32, extendedparameters : *mut super::winnt::MEM_EXTENDED_PARAMETER, parametercount : u32) -> *mut core::ffi::c_void);
    unsafe { MapViewOfFile3FromApp(filemapping, process.unwrap_or(core::mem::zeroed()) as _, baseaddress.unwrap_or(core::mem::zeroed()) as _, offset, viewsize, allocationtype, pageprotection, core::mem::transmute(extendedparameters.as_deref().map_or(core::ptr::null(), |slice| slice.as_ptr())), extendedparameters.as_deref().map_or(0, |slice| slice.len().try_into().unwrap())) }
}
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn MapViewOfFileEx(hfilemappingobject: super::winnt::HANDLE, dwdesiredaccess: u32, dwfileoffsethigh: u32, dwfileoffsetlow: u32, dwnumberofbytestomap: usize, lpbaseaddress: Option<*const core::ffi::c_void>) -> *mut core::ffi::c_void {
    windows_core::link!("kernel32.dll" "system" fn MapViewOfFileEx(hfilemappingobject : super::winnt::HANDLE, dwdesiredaccess : u32, dwfileoffsethigh : u32, dwfileoffsetlow : u32, dwnumberofbytestomap : usize, lpbaseaddress : *const core::ffi::c_void) -> *mut core::ffi::c_void);
    unsafe { MapViewOfFileEx(hfilemappingobject, dwdesiredaccess, dwfileoffsethigh, dwfileoffsetlow, dwnumberofbytestomap, lpbaseaddress.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn MapViewOfFileFromApp(hfilemappingobject: super::winnt::HANDLE, desiredaccess: u32, fileoffset: u64, numberofbytestomap: usize) -> *mut core::ffi::c_void {
    windows_core::link!("kernel32.dll" "system" fn MapViewOfFileFromApp(hfilemappingobject : super::winnt::HANDLE, desiredaccess : u32, fileoffset : u64, numberofbytestomap : usize) -> *mut core::ffi::c_void);
    unsafe { MapViewOfFileFromApp(hfilemappingobject, desiredaccess, fileoffset, numberofbytestomap) }
}
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn MapViewOfFileNuma2(filemappinghandle: super::winnt::HANDLE, processhandle: super::winnt::HANDLE, offset: u64, baseaddress: Option<*const core::ffi::c_void>, viewsize: usize, allocationtype: u32, pageprotection: u32, preferrednode: u32) -> *mut core::ffi::c_void {
    windows_core::link!("api-ms-win-core-memory-l1-1-5.dll" "system" fn MapViewOfFileNuma2(filemappinghandle : super::winnt::HANDLE, processhandle : super::winnt::HANDLE, offset : u64, baseaddress : *const core::ffi::c_void, viewsize : usize, allocationtype : u32, pageprotection : u32, preferrednode : u32) -> *mut core::ffi::c_void);
    unsafe { MapViewOfFileNuma2(filemappinghandle, processhandle, offset, baseaddress.unwrap_or(core::mem::zeroed()) as _, viewsize, allocationtype, pageprotection, preferrednode) }
}
#[inline]
pub unsafe fn OfferVirtualMemory(virtualaddress: &mut [u8], priority: OFFER_PRIORITY) -> u32 {
    windows_core::link!("kernel32.dll" "system" fn OfferVirtualMemory(virtualaddress : *mut core::ffi::c_void, size : usize, priority : OFFER_PRIORITY) -> u32);
    unsafe { OfferVirtualMemory(core::mem::transmute(virtualaddress.as_ptr()), virtualaddress.len().try_into().unwrap(), priority) }
}
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn OpenDedicatedMemoryPartition(partition: super::winnt::HANDLE, dedicatedmemorytypeid: u64, desiredaccess: super::winnt::ACCESS_MASK, inherithandle: bool) -> super::winnt::HANDLE {
    windows_core::link!("api-ms-win-core-memory-l1-1-8.dll" "system" fn OpenDedicatedMemoryPartition(partition : super::winnt::HANDLE, dedicatedmemorytypeid : u64, desiredaccess : super::winnt::ACCESS_MASK, inherithandle : windows_core::BOOL) -> super::winnt::HANDLE);
    unsafe { OpenDedicatedMemoryPartition(partition, dedicatedmemorytypeid, desiredaccess, inherithandle.into()) }
}
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn OpenFileMappingFromApp<P2>(desiredaccess: u32, inherithandle: bool, name: P2) -> super::winnt::HANDLE
where
    P2: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("api-ms-win-core-memory-l1-1-3.dll" "system" fn OpenFileMappingFromApp(desiredaccess : u32, inherithandle : windows_core::BOOL, name : windows_core::PCWSTR) -> super::winnt::HANDLE);
    unsafe { OpenFileMappingFromApp(desiredaccess, inherithandle.into(), name.param().abi()) }
}
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn OpenFileMappingW<P2>(dwdesiredaccess: u32, binherithandle: bool, lpname: P2) -> super::winnt::HANDLE
where
    P2: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("kernel32.dll" "system" fn OpenFileMappingW(dwdesiredaccess : u32, binherithandle : windows_core::BOOL, lpname : windows_core::PCWSTR) -> super::winnt::HANDLE);
    unsafe { OpenFileMappingW(dwdesiredaccess, binherithandle.into(), lpname.param().abi()) }
}
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn PrefetchVirtualMemory(hprocess: super::winnt::HANDLE, virtualaddresses: &[WIN32_MEMORY_RANGE_ENTRY], flags: u32) -> windows_core::BOOL {
    windows_core::link!("kernel32.dll" "system" fn PrefetchVirtualMemory(hprocess : super::winnt::HANDLE, numberofentries : usize, virtualaddresses : *const WIN32_MEMORY_RANGE_ENTRY, flags : u32) -> windows_core::BOOL);
    unsafe { PrefetchVirtualMemory(hprocess, virtualaddresses.len().try_into().unwrap(), core::mem::transmute(virtualaddresses.as_ptr()), flags) }
}
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn QueryMemoryResourceNotification(resourcenotificationhandle: super::winnt::HANDLE, resourcestate: *mut windows_core::BOOL) -> windows_core::BOOL {
    windows_core::link!("kernel32.dll" "system" fn QueryMemoryResourceNotification(resourcenotificationhandle : super::winnt::HANDLE, resourcestate : *mut windows_core::BOOL) -> windows_core::BOOL);
    unsafe { QueryMemoryResourceNotification(resourcenotificationhandle, resourcestate as _) }
}
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn QueryPartitionInformation(partition: super::winnt::HANDLE, partitioninformationclass: WIN32_MEMORY_PARTITION_INFORMATION_CLASS, partitioninformation: *mut core::ffi::c_void, partitioninformationlength: u32) -> windows_core::BOOL {
    windows_core::link!("api-ms-win-core-memory-l1-1-8.dll" "system" fn QueryPartitionInformation(partition : super::winnt::HANDLE, partitioninformationclass : WIN32_MEMORY_PARTITION_INFORMATION_CLASS, partitioninformation : *mut core::ffi::c_void, partitioninformationlength : u32) -> windows_core::BOOL);
    unsafe { QueryPartitionInformation(partition, partitioninformationclass, partitioninformation as _, partitioninformationlength) }
}
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn QueryVirtualMemoryInformation(process: super::winnt::HANDLE, virtualaddress: *const core::ffi::c_void, memoryinformationclass: WIN32_MEMORY_INFORMATION_CLASS, memoryinformation: *mut core::ffi::c_void, memoryinformationsize: usize, returnsize: Option<*mut usize>) -> windows_core::BOOL {
    windows_core::link!("api-ms-win-core-memory-l1-1-4.dll" "system" fn QueryVirtualMemoryInformation(process : super::winnt::HANDLE, virtualaddress : *const core::ffi::c_void, memoryinformationclass : WIN32_MEMORY_INFORMATION_CLASS, memoryinformation : *mut core::ffi::c_void, memoryinformationsize : usize, returnsize : *mut usize) -> windows_core::BOOL);
    unsafe { QueryVirtualMemoryInformation(process, virtualaddress, memoryinformationclass, memoryinformation as _, memoryinformationsize, returnsize.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn ReadProcessMemory(hprocess: super::winnt::HANDLE, lpbaseaddress: *const core::ffi::c_void, lpbuffer: *mut core::ffi::c_void, nsize: usize, lpnumberofbytesread: Option<*mut usize>) -> windows_core::BOOL {
    windows_core::link!("kernel32.dll" "system" fn ReadProcessMemory(hprocess : super::winnt::HANDLE, lpbaseaddress : *const core::ffi::c_void, lpbuffer : *mut core::ffi::c_void, nsize : usize, lpnumberofbytesread : *mut usize) -> windows_core::BOOL);
    unsafe { ReadProcessMemory(hprocess, lpbaseaddress, lpbuffer as _, nsize, lpnumberofbytesread.unwrap_or(core::mem::zeroed()) as _) }
}
#[inline]
pub unsafe fn ReclaimVirtualMemory(virtualaddress: &[u8]) -> u32 {
    windows_core::link!("kernel32.dll" "system" fn ReclaimVirtualMemory(virtualaddress : *const core::ffi::c_void, size : usize) -> u32);
    unsafe { ReclaimVirtualMemory(core::mem::transmute(virtualaddress.as_ptr()), virtualaddress.len().try_into().unwrap()) }
}
#[inline]
pub unsafe fn RegisterBadMemoryNotification(callback: PBAD_MEMORY_CALLBACK_ROUTINE) -> *mut core::ffi::c_void {
    windows_core::link!("kernel32.dll" "system" fn RegisterBadMemoryNotification(callback : PBAD_MEMORY_CALLBACK_ROUTINE) -> *mut core::ffi::c_void);
    unsafe { RegisterBadMemoryNotification(callback) }
}
#[inline]
pub unsafe fn ResetWriteWatch(lpbaseaddress: *const core::ffi::c_void, dwregionsize: usize) -> u32 {
    windows_core::link!("kernel32.dll" "system" fn ResetWriteWatch(lpbaseaddress : *const core::ffi::c_void, dwregionsize : usize) -> u32);
    unsafe { ResetWriteWatch(lpbaseaddress, dwregionsize) }
}
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn SetProcessValidCallTargets(hprocess: super::winnt::HANDLE, virtualaddress: *const core::ffi::c_void, regionsize: usize, offsetinformation: &mut [super::winnt::CFG_CALL_TARGET_INFO]) -> windows_core::BOOL {
    windows_core::link!("api-ms-win-core-memory-l1-1-3.dll" "system" fn SetProcessValidCallTargets(hprocess : super::winnt::HANDLE, virtualaddress : *const core::ffi::c_void, regionsize : usize, numberofoffsets : u32, offsetinformation : *mut super::winnt::CFG_CALL_TARGET_INFO) -> windows_core::BOOL);
    unsafe { SetProcessValidCallTargets(hprocess, virtualaddress, regionsize, offsetinformation.len().try_into().unwrap(), core::mem::transmute(offsetinformation.as_ptr())) }
}
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn SetProcessValidCallTargetsForMappedView(process: super::winnt::HANDLE, virtualaddress: *const core::ffi::c_void, regionsize: usize, offsetinformation: &mut [super::winnt::CFG_CALL_TARGET_INFO], section: super::winnt::HANDLE, expectedfileoffset: u64) -> windows_core::BOOL {
    windows_core::link!("api-ms-win-core-memory-l1-1-7.dll" "system" fn SetProcessValidCallTargetsForMappedView(process : super::winnt::HANDLE, virtualaddress : *const core::ffi::c_void, regionsize : usize, numberofoffsets : u32, offsetinformation : *mut super::winnt::CFG_CALL_TARGET_INFO, section : super::winnt::HANDLE, expectedfileoffset : u64) -> windows_core::BOOL);
    unsafe { SetProcessValidCallTargetsForMappedView(process, virtualaddress, regionsize, offsetinformation.len().try_into().unwrap(), core::mem::transmute(offsetinformation.as_ptr()), section, expectedfileoffset) }
}
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn SetProcessWorkingSetSize(hprocess: super::winnt::HANDLE, dwminimumworkingsetsize: usize, dwmaximumworkingsetsize: usize) -> windows_core::BOOL {
    windows_core::link!("kernel32.dll" "system" fn SetProcessWorkingSetSize(hprocess : super::winnt::HANDLE, dwminimumworkingsetsize : usize, dwmaximumworkingsetsize : usize) -> windows_core::BOOL);
    unsafe { SetProcessWorkingSetSize(hprocess, dwminimumworkingsetsize, dwmaximumworkingsetsize) }
}
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn SetProcessWorkingSetSizeEx(hprocess: super::winnt::HANDLE, dwminimumworkingsetsize: usize, dwmaximumworkingsetsize: usize, flags: u32) -> windows_core::BOOL {
    windows_core::link!("kernel32.dll" "system" fn SetProcessWorkingSetSizeEx(hprocess : super::winnt::HANDLE, dwminimumworkingsetsize : usize, dwmaximumworkingsetsize : usize, flags : u32) -> windows_core::BOOL);
    unsafe { SetProcessWorkingSetSizeEx(hprocess, dwminimumworkingsetsize, dwmaximumworkingsetsize, flags) }
}
#[inline]
pub unsafe fn SetSystemFileCacheSize(minimumfilecachesize: usize, maximumfilecachesize: usize, flags: u32) -> windows_core::BOOL {
    windows_core::link!("kernel32.dll" "system" fn SetSystemFileCacheSize(minimumfilecachesize : usize, maximumfilecachesize : usize, flags : u32) -> windows_core::BOOL);
    unsafe { SetSystemFileCacheSize(minimumfilecachesize, maximumfilecachesize, flags) }
}
#[inline]
pub unsafe fn UnmapViewOfFile(lpbaseaddress: *const core::ffi::c_void) -> windows_core::BOOL {
    windows_core::link!("kernel32.dll" "system" fn UnmapViewOfFile(lpbaseaddress : *const core::ffi::c_void) -> windows_core::BOOL);
    unsafe { UnmapViewOfFile(lpbaseaddress) }
}
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn UnmapViewOfFile2(process: super::winnt::HANDLE, baseaddress: *const core::ffi::c_void, unmapflags: u32) -> windows_core::BOOL {
    windows_core::link!("api-ms-win-core-memory-l1-1-5.dll" "system" fn UnmapViewOfFile2(process : super::winnt::HANDLE, baseaddress : *const core::ffi::c_void, unmapflags : u32) -> windows_core::BOOL);
    unsafe { UnmapViewOfFile2(process, baseaddress, unmapflags) }
}
#[inline]
pub unsafe fn UnmapViewOfFileEx(baseaddress: *const core::ffi::c_void, unmapflags: u32) -> windows_core::BOOL {
    windows_core::link!("kernel32.dll" "system" fn UnmapViewOfFileEx(baseaddress : *const core::ffi::c_void, unmapflags : u32) -> windows_core::BOOL);
    unsafe { UnmapViewOfFileEx(baseaddress, unmapflags) }
}
#[inline]
pub unsafe fn UnregisterBadMemoryNotification(registrationhandle: *const core::ffi::c_void) -> windows_core::BOOL {
    windows_core::link!("kernel32.dll" "system" fn UnregisterBadMemoryNotification(registrationhandle : *const core::ffi::c_void) -> windows_core::BOOL);
    unsafe { UnregisterBadMemoryNotification(registrationhandle) }
}
#[inline]
pub unsafe fn VirtualAlloc(lpaddress: Option<*const core::ffi::c_void>, dwsize: usize, flallocationtype: u32, flprotect: u32) -> *mut core::ffi::c_void {
    windows_core::link!("kernel32.dll" "system" fn VirtualAlloc(lpaddress : *const core::ffi::c_void, dwsize : usize, flallocationtype : u32, flprotect : u32) -> *mut core::ffi::c_void);
    unsafe { VirtualAlloc(lpaddress.unwrap_or(core::mem::zeroed()) as _, dwsize, flallocationtype, flprotect) }
}
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn VirtualAlloc2(process: Option<super::winnt::HANDLE>, baseaddress: Option<*const core::ffi::c_void>, size: usize, allocationtype: u32, pageprotection: u32, extendedparameters: Option<&mut [super::winnt::MEM_EXTENDED_PARAMETER]>) -> *mut core::ffi::c_void {
    windows_core::link!("api-ms-win-core-memory-l1-1-6.dll" "system" fn VirtualAlloc2(process : super::winnt::HANDLE, baseaddress : *const core::ffi::c_void, size : usize, allocationtype : u32, pageprotection : u32, extendedparameters : *mut super::winnt::MEM_EXTENDED_PARAMETER, parametercount : u32) -> *mut core::ffi::c_void);
    unsafe { VirtualAlloc2(process.unwrap_or(core::mem::zeroed()) as _, baseaddress.unwrap_or(core::mem::zeroed()) as _, size, allocationtype, pageprotection, core::mem::transmute(extendedparameters.as_deref().map_or(core::ptr::null(), |slice| slice.as_ptr())), extendedparameters.as_deref().map_or(0, |slice| slice.len().try_into().unwrap())) }
}
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn VirtualAlloc2FromApp(process: Option<super::winnt::HANDLE>, baseaddress: Option<*const core::ffi::c_void>, size: usize, allocationtype: u32, pageprotection: u32, extendedparameters: Option<&mut [super::winnt::MEM_EXTENDED_PARAMETER]>) -> *mut core::ffi::c_void {
    windows_core::link!("api-ms-win-core-memory-l1-1-6.dll" "system" fn VirtualAlloc2FromApp(process : super::winnt::HANDLE, baseaddress : *const core::ffi::c_void, size : usize, allocationtype : u32, pageprotection : u32, extendedparameters : *mut super::winnt::MEM_EXTENDED_PARAMETER, parametercount : u32) -> *mut core::ffi::c_void);
    unsafe { VirtualAlloc2FromApp(process.unwrap_or(core::mem::zeroed()) as _, baseaddress.unwrap_or(core::mem::zeroed()) as _, size, allocationtype, pageprotection, core::mem::transmute(extendedparameters.as_deref().map_or(core::ptr::null(), |slice| slice.as_ptr())), extendedparameters.as_deref().map_or(0, |slice| slice.len().try_into().unwrap())) }
}
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn VirtualAllocEx(hprocess: super::winnt::HANDLE, lpaddress: Option<*const core::ffi::c_void>, dwsize: usize, flallocationtype: u32, flprotect: u32) -> *mut core::ffi::c_void {
    windows_core::link!("kernel32.dll" "system" fn VirtualAllocEx(hprocess : super::winnt::HANDLE, lpaddress : *const core::ffi::c_void, dwsize : usize, flallocationtype : u32, flprotect : u32) -> *mut core::ffi::c_void);
    unsafe { VirtualAllocEx(hprocess, lpaddress.unwrap_or(core::mem::zeroed()) as _, dwsize, flallocationtype, flprotect) }
}
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn VirtualAllocExNuma(hprocess: super::winnt::HANDLE, lpaddress: Option<*const core::ffi::c_void>, dwsize: usize, flallocationtype: u32, flprotect: u32, nndpreferred: u32) -> *mut core::ffi::c_void {
    windows_core::link!("kernel32.dll" "system" fn VirtualAllocExNuma(hprocess : super::winnt::HANDLE, lpaddress : *const core::ffi::c_void, dwsize : usize, flallocationtype : u32, flprotect : u32, nndpreferred : u32) -> *mut core::ffi::c_void);
    unsafe { VirtualAllocExNuma(hprocess, lpaddress.unwrap_or(core::mem::zeroed()) as _, dwsize, flallocationtype, flprotect, nndpreferred) }
}
#[inline]
pub unsafe fn VirtualAllocFromApp(baseaddress: Option<*const core::ffi::c_void>, size: usize, allocationtype: u32, protection: u32) -> *mut core::ffi::c_void {
    windows_core::link!("api-ms-win-core-memory-l1-1-3.dll" "system" fn VirtualAllocFromApp(baseaddress : *const core::ffi::c_void, size : usize, allocationtype : u32, protection : u32) -> *mut core::ffi::c_void);
    unsafe { VirtualAllocFromApp(baseaddress.unwrap_or(core::mem::zeroed()) as _, size, allocationtype, protection) }
}
#[inline]
pub unsafe fn VirtualFree(lpaddress: *mut core::ffi::c_void, dwsize: usize, dwfreetype: u32) -> windows_core::BOOL {
    windows_core::link!("kernel32.dll" "system" fn VirtualFree(lpaddress : *mut core::ffi::c_void, dwsize : usize, dwfreetype : u32) -> windows_core::BOOL);
    unsafe { VirtualFree(lpaddress as _, dwsize, dwfreetype) }
}
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn VirtualFreeEx(hprocess: super::winnt::HANDLE, lpaddress: *mut core::ffi::c_void, dwsize: usize, dwfreetype: u32) -> windows_core::BOOL {
    windows_core::link!("kernel32.dll" "system" fn VirtualFreeEx(hprocess : super::winnt::HANDLE, lpaddress : *mut core::ffi::c_void, dwsize : usize, dwfreetype : u32) -> windows_core::BOOL);
    unsafe { VirtualFreeEx(hprocess, lpaddress as _, dwsize, dwfreetype) }
}
#[inline]
pub unsafe fn VirtualLock(lpaddress: *const core::ffi::c_void, dwsize: usize) -> windows_core::BOOL {
    windows_core::link!("kernel32.dll" "system" fn VirtualLock(lpaddress : *const core::ffi::c_void, dwsize : usize) -> windows_core::BOOL);
    unsafe { VirtualLock(lpaddress, dwsize) }
}
#[inline]
pub unsafe fn VirtualProtect(lpaddress: *const core::ffi::c_void, dwsize: usize, flnewprotect: u32, lpfloldprotect: *mut u32) -> windows_core::BOOL {
    windows_core::link!("kernel32.dll" "system" fn VirtualProtect(lpaddress : *const core::ffi::c_void, dwsize : usize, flnewprotect : u32, lpfloldprotect : *mut u32) -> windows_core::BOOL);
    unsafe { VirtualProtect(lpaddress, dwsize, flnewprotect, lpfloldprotect as _) }
}
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn VirtualProtectEx(hprocess: super::winnt::HANDLE, lpaddress: *const core::ffi::c_void, dwsize: usize, flnewprotect: u32, lpfloldprotect: *mut u32) -> windows_core::BOOL {
    windows_core::link!("kernel32.dll" "system" fn VirtualProtectEx(hprocess : super::winnt::HANDLE, lpaddress : *const core::ffi::c_void, dwsize : usize, flnewprotect : u32, lpfloldprotect : *mut u32) -> windows_core::BOOL);
    unsafe { VirtualProtectEx(hprocess, lpaddress, dwsize, flnewprotect, lpfloldprotect as _) }
}
#[inline]
pub unsafe fn VirtualProtectFromApp(address: *const core::ffi::c_void, size: usize, newprotection: u32, oldprotection: *mut u32) -> windows_core::BOOL {
    windows_core::link!("api-ms-win-core-memory-l1-1-3.dll" "system" fn VirtualProtectFromApp(address : *const core::ffi::c_void, size : usize, newprotection : u32, oldprotection : *mut u32) -> windows_core::BOOL);
    unsafe { VirtualProtectFromApp(address, size, newprotection, oldprotection as _) }
}
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn VirtualQuery(lpaddress: Option<*const core::ffi::c_void>, lpbuffer: *mut super::winnt::MEMORY_BASIC_INFORMATION, dwlength: usize) -> usize {
    windows_core::link!("kernel32.dll" "system" fn VirtualQuery(lpaddress : *const core::ffi::c_void, lpbuffer : *mut super::winnt::MEMORY_BASIC_INFORMATION, dwlength : usize) -> usize);
    unsafe { VirtualQuery(lpaddress.unwrap_or(core::mem::zeroed()) as _, lpbuffer as _, dwlength) }
}
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn VirtualQueryEx(hprocess: super::winnt::HANDLE, lpaddress: Option<*const core::ffi::c_void>, lpbuffer: *mut super::winnt::MEMORY_BASIC_INFORMATION, dwlength: usize) -> usize {
    windows_core::link!("kernel32.dll" "system" fn VirtualQueryEx(hprocess : super::winnt::HANDLE, lpaddress : *const core::ffi::c_void, lpbuffer : *mut super::winnt::MEMORY_BASIC_INFORMATION, dwlength : usize) -> usize);
    unsafe { VirtualQueryEx(hprocess, lpaddress.unwrap_or(core::mem::zeroed()) as _, lpbuffer as _, dwlength) }
}
#[inline]
pub unsafe fn VirtualUnlock(lpaddress: *const core::ffi::c_void, dwsize: usize) -> windows_core::BOOL {
    windows_core::link!("kernel32.dll" "system" fn VirtualUnlock(lpaddress : *const core::ffi::c_void, dwsize : usize) -> windows_core::BOOL);
    unsafe { VirtualUnlock(lpaddress, dwsize) }
}
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn VirtualUnlockEx(process: Option<super::winnt::HANDLE>, address: *const core::ffi::c_void, size: usize) -> windows_core::BOOL {
    windows_core::link!("api-ms-win-core-memory-l1-1-5.dll" "system" fn VirtualUnlockEx(process : super::winnt::HANDLE, address : *const core::ffi::c_void, size : usize) -> windows_core::BOOL);
    unsafe { VirtualUnlockEx(process.unwrap_or(core::mem::zeroed()) as _, address, size) }
}
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn WriteProcessMemory(hprocess: super::winnt::HANDLE, lpbaseaddress: *const core::ffi::c_void, lpbuffer: *const core::ffi::c_void, nsize: usize, lpnumberofbyteswritten: Option<*mut usize>) -> windows_core::BOOL {
    windows_core::link!("kernel32.dll" "system" fn WriteProcessMemory(hprocess : super::winnt::HANDLE, lpbaseaddress : *const core::ffi::c_void, lpbuffer : *const core::ffi::c_void, nsize : usize, lpnumberofbyteswritten : *mut usize) -> windows_core::BOOL);
    unsafe { WriteProcessMemory(hprocess, lpbaseaddress, lpbuffer, nsize, lpnumberofbyteswritten.unwrap_or(core::mem::zeroed()) as _) }
}
pub type BAD_MEMORY_CALLBACK_ROUTINE = Option<unsafe extern "system" fn()>;
pub const FILE_CACHE_MAX_HARD_DISABLE: u32 = 2;
pub const FILE_CACHE_MAX_HARD_ENABLE: u32 = 1;
pub const FILE_CACHE_MIN_HARD_DISABLE: u32 = 8;
pub const FILE_CACHE_MIN_HARD_ENABLE: u32 = 4;
pub const FILE_MAP_ALL_ACCESS: u32 = 983071;
pub const FILE_MAP_COPY: u32 = 1;
pub const FILE_MAP_EXECUTE: u32 = 32;
pub const FILE_MAP_LARGE_PAGES: u32 = 536870912;
pub const FILE_MAP_READ: u32 = 4;
pub const FILE_MAP_RESERVE: u32 = 2147483648;
pub const FILE_MAP_TARGETS_INVALID: u32 = 1073741824;
pub const FILE_MAP_WRITE: u32 = 2;
pub const HighMemoryResourceNotification: MEMORY_RESOURCE_NOTIFICATION_TYPE = 1;
pub const LowMemoryResourceNotification: MEMORY_RESOURCE_NOTIFICATION_TYPE = 0;
pub const MEHC_PATROL_SCRUBBER_PRESENT: u32 = 1;
pub type MEMORY_RESOURCE_NOTIFICATION_TYPE = i32;
pub const MemoryPartitionDedicatedMemoryInfo: WIN32_MEMORY_PARTITION_INFORMATION_CLASS = 1;
pub const MemoryPartitionInfo: WIN32_MEMORY_PARTITION_INFORMATION_CLASS = 0;
pub const MemoryRegionInfo: WIN32_MEMORY_INFORMATION_CLASS = 0;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct NUMA_NODE_MEMORY_PERFORMANCE_BANDWIDTH {
    pub Condition: NUMA_NODE_MEMORY_PERFORMANCE_MEASUREMENT_CONDITION,
    pub Bandwidth: u64,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct NUMA_NODE_MEMORY_PERFORMANCE_LATENCY {
    pub Condition: NUMA_NODE_MEMORY_PERFORMANCE_MEASUREMENT_CONDITION,
    pub Latency: u64,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct NUMA_NODE_MEMORY_PERFORMANCE_MEASUREMENT_CONDITION {
    pub Flags: NUMA_NODE_MEMORY_PERFORMANCE_MEASUREMENT_CONDITION_0,
    pub MinTransferSizeInBytes: u64,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct NUMA_NODE_MEMORY_PERFORMANCE_MEASUREMENT_CONDITION_0 {
    pub _bitfield: u8,
}
pub type OFFER_PRIORITY = i32;
pub type PBAD_MEMORY_CALLBACK_ROUTINE = Option<unsafe extern "system" fn()>;
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PWIN32_MEMORY_RANGE_ENTRY(pub *mut WIN32_MEMORY_RANGE_ENTRY);
impl PWIN32_MEMORY_RANGE_ENTRY {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PWIN32_MEMORY_RANGE_ENTRY {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const VmOfferPriorityBelowNormal: OFFER_PRIORITY = 3;
pub const VmOfferPriorityLow: OFFER_PRIORITY = 2;
pub const VmOfferPriorityNormal: OFFER_PRIORITY = 4;
pub const VmOfferPriorityVeryLow: OFFER_PRIORITY = 1;
pub type WIN32_MEMORY_INFORMATION_CLASS = i32;
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct WIN32_MEMORY_PARTITION_INFORMATION {
    pub Flags: u32,
    pub NumaNode: u32,
    pub Channel: u32,
    pub NumberOfNumaNodes: u32,
    pub ResidentAvailablePages: u64,
    pub CommittedPages: u64,
    pub CommitLimit: u64,
    pub PeakCommitment: u64,
    pub TotalNumberOfPages: u64,
    pub AvailablePages: u64,
    pub ZeroPages: u64,
    pub FreePages: u64,
    pub StandbyPages: u64,
    pub Reserved: [u64; 16],
    pub MaximumCommitLimit: u64,
    pub Reserved2: u64,
    pub PartitionId: u32,
}
impl Default for WIN32_MEMORY_PARTITION_INFORMATION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub type WIN32_MEMORY_PARTITION_INFORMATION_CLASS = i32;
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct WIN32_MEMORY_RANGE_ENTRY {
    pub VirtualAddress: *mut core::ffi::c_void,
    pub NumberOfBytes: usize,
}
impl Default for WIN32_MEMORY_RANGE_ENTRY {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct WIN32_MEMORY_REGION_INFORMATION {
    pub AllocationBase: *mut core::ffi::c_void,
    pub AllocationProtect: u32,
    pub Anonymous: WIN32_MEMORY_REGION_INFORMATION_0,
    pub RegionSize: usize,
    pub CommitSize: usize,
}
impl Default for WIN32_MEMORY_REGION_INFORMATION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union WIN32_MEMORY_REGION_INFORMATION_0 {
    pub Flags: u32,
    pub Anonymous: WIN32_MEMORY_REGION_INFORMATION_0_0,
}
impl Default for WIN32_MEMORY_REGION_INFORMATION_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct WIN32_MEMORY_REGION_INFORMATION_0_0 {
    pub _bitfield: u32,
}

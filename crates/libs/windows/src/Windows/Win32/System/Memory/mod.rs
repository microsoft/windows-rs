#[cfg(feature = "Win32_System_Memory_NonVolatile")]
pub mod NonVolatile;
#[inline]
pub unsafe fn AddSecureMemoryCacheCallback(pfncallback: PSECURE_MEMORY_CACHE_CALLBACK) -> windows_core::Result<()> {
    windows_targets::link!("kernel32.dll" "system" fn AddSecureMemoryCacheCallback(pfncallback : PSECURE_MEMORY_CACHE_CALLBACK) -> super::super::Foundation:: BOOL);
    AddSecureMemoryCacheCallback(core::mem::transmute(pfncallback)).ok()
}
#[inline]
pub unsafe fn AllocateUserPhysicalPages(hprocess: super::super::Foundation::HANDLE, numberofpages: *mut usize, pagearray: *mut usize) -> windows_core::Result<()> {
    windows_targets::link!("kernel32.dll" "system" fn AllocateUserPhysicalPages(hprocess : super::super::Foundation:: HANDLE, numberofpages : *mut usize, pagearray : *mut usize) -> super::super::Foundation:: BOOL);
    AllocateUserPhysicalPages(core::mem::transmute(hprocess), core::mem::transmute(numberofpages), core::mem::transmute(pagearray)).ok()
}
#[inline]
pub unsafe fn AllocateUserPhysicalPages2(objecthandle: super::super::Foundation::HANDLE, numberofpages: *mut usize, pagearray: *mut usize, extendedparameters: Option<&mut [MEM_EXTENDED_PARAMETER]>) -> super::super::Foundation::BOOL {
    windows_targets::link!("api-ms-win-core-memory-l1-1-8.dll" "system" fn AllocateUserPhysicalPages2(objecthandle : super::super::Foundation:: HANDLE, numberofpages : *mut usize, pagearray : *mut usize, extendedparameters : *mut MEM_EXTENDED_PARAMETER, extendedparametercount : u32) -> super::super::Foundation:: BOOL);
    AllocateUserPhysicalPages2(core::mem::transmute(objecthandle), core::mem::transmute(numberofpages), core::mem::transmute(pagearray), core::mem::transmute(extendedparameters.as_deref().map_or(core::ptr::null(), |slice| slice.as_ptr())), extendedparameters.as_deref().map_or(0, |slice| slice.len().try_into().unwrap()))
}
#[inline]
pub unsafe fn AllocateUserPhysicalPagesNuma(hprocess: super::super::Foundation::HANDLE, numberofpages: *mut usize, pagearray: *mut usize, nndpreferred: u32) -> windows_core::Result<()> {
    windows_targets::link!("kernel32.dll" "system" fn AllocateUserPhysicalPagesNuma(hprocess : super::super::Foundation:: HANDLE, numberofpages : *mut usize, pagearray : *mut usize, nndpreferred : u32) -> super::super::Foundation:: BOOL);
    AllocateUserPhysicalPagesNuma(core::mem::transmute(hprocess), core::mem::transmute(numberofpages), core::mem::transmute(pagearray), core::mem::transmute(nndpreferred)).ok()
}
#[cfg(feature = "Win32_Security")]
#[inline]
pub unsafe fn CreateFileMapping2<P6>(file: super::super::Foundation::HANDLE, securityattributes: Option<*const super::super::Security::SECURITY_ATTRIBUTES>, desiredaccess: u32, pageprotection: PAGE_PROTECTION_FLAGS, allocationattributes: u32, maximumsize: u64, name: P6, extendedparameters: Option<&mut [MEM_EXTENDED_PARAMETER]>) -> windows_core::Result<super::super::Foundation::HANDLE>
where
    P6: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("api-ms-win-core-memory-l1-1-7.dll" "system" fn CreateFileMapping2(file : super::super::Foundation:: HANDLE, securityattributes : *const super::super::Security:: SECURITY_ATTRIBUTES, desiredaccess : u32, pageprotection : PAGE_PROTECTION_FLAGS, allocationattributes : u32, maximumsize : u64, name : windows_core::PCWSTR, extendedparameters : *mut MEM_EXTENDED_PARAMETER, parametercount : u32) -> super::super::Foundation:: HANDLE);
    let result__ = CreateFileMapping2(core::mem::transmute(file), core::mem::transmute(securityattributes.unwrap_or(core::mem::zeroed())), core::mem::transmute(desiredaccess), core::mem::transmute(pageprotection), core::mem::transmute(allocationattributes), core::mem::transmute(maximumsize), name.param().abi(), core::mem::transmute(extendedparameters.as_deref().map_or(core::ptr::null(), |slice| slice.as_ptr())), extendedparameters.as_deref().map_or(0, |slice| slice.len().try_into().unwrap()));
    (!result__.is_invalid()).then_some(result__).ok_or_else(windows_core::Error::from_win32)
}
#[cfg(feature = "Win32_Security")]
#[inline]
pub unsafe fn CreateFileMappingA<P5>(hfile: super::super::Foundation::HANDLE, lpfilemappingattributes: Option<*const super::super::Security::SECURITY_ATTRIBUTES>, flprotect: PAGE_PROTECTION_FLAGS, dwmaximumsizehigh: u32, dwmaximumsizelow: u32, lpname: P5) -> windows_core::Result<super::super::Foundation::HANDLE>
where
    P5: windows_core::Param<windows_core::PCSTR>,
{
    windows_targets::link!("kernel32.dll" "system" fn CreateFileMappingA(hfile : super::super::Foundation:: HANDLE, lpfilemappingattributes : *const super::super::Security:: SECURITY_ATTRIBUTES, flprotect : PAGE_PROTECTION_FLAGS, dwmaximumsizehigh : u32, dwmaximumsizelow : u32, lpname : windows_core::PCSTR) -> super::super::Foundation:: HANDLE);
    let result__ = CreateFileMappingA(core::mem::transmute(hfile), core::mem::transmute(lpfilemappingattributes.unwrap_or(core::mem::zeroed())), core::mem::transmute(flprotect), core::mem::transmute(dwmaximumsizehigh), core::mem::transmute(dwmaximumsizelow), lpname.param().abi());
    (!result__.is_invalid()).then_some(result__).ok_or_else(windows_core::Error::from_win32)
}
#[cfg(feature = "Win32_Security")]
#[inline]
pub unsafe fn CreateFileMappingFromApp<P4>(hfile: super::super::Foundation::HANDLE, securityattributes: Option<*const super::super::Security::SECURITY_ATTRIBUTES>, pageprotection: PAGE_PROTECTION_FLAGS, maximumsize: u64, name: P4) -> windows_core::Result<super::super::Foundation::HANDLE>
where
    P4: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("kernel32.dll" "system" fn CreateFileMappingFromApp(hfile : super::super::Foundation:: HANDLE, securityattributes : *const super::super::Security:: SECURITY_ATTRIBUTES, pageprotection : PAGE_PROTECTION_FLAGS, maximumsize : u64, name : windows_core::PCWSTR) -> super::super::Foundation:: HANDLE);
    let result__ = CreateFileMappingFromApp(core::mem::transmute(hfile), core::mem::transmute(securityattributes.unwrap_or(core::mem::zeroed())), core::mem::transmute(pageprotection), core::mem::transmute(maximumsize), name.param().abi());
    (!result__.is_invalid()).then_some(result__).ok_or_else(windows_core::Error::from_win32)
}
#[cfg(feature = "Win32_Security")]
#[inline]
pub unsafe fn CreateFileMappingNumaA<P5>(hfile: super::super::Foundation::HANDLE, lpfilemappingattributes: Option<*const super::super::Security::SECURITY_ATTRIBUTES>, flprotect: PAGE_PROTECTION_FLAGS, dwmaximumsizehigh: u32, dwmaximumsizelow: u32, lpname: P5, nndpreferred: u32) -> windows_core::Result<super::super::Foundation::HANDLE>
where
    P5: windows_core::Param<windows_core::PCSTR>,
{
    windows_targets::link!("kernel32.dll" "system" fn CreateFileMappingNumaA(hfile : super::super::Foundation:: HANDLE, lpfilemappingattributes : *const super::super::Security:: SECURITY_ATTRIBUTES, flprotect : PAGE_PROTECTION_FLAGS, dwmaximumsizehigh : u32, dwmaximumsizelow : u32, lpname : windows_core::PCSTR, nndpreferred : u32) -> super::super::Foundation:: HANDLE);
    let result__ = CreateFileMappingNumaA(core::mem::transmute(hfile), core::mem::transmute(lpfilemappingattributes.unwrap_or(core::mem::zeroed())), core::mem::transmute(flprotect), core::mem::transmute(dwmaximumsizehigh), core::mem::transmute(dwmaximumsizelow), lpname.param().abi(), core::mem::transmute(nndpreferred));
    (!result__.is_invalid()).then_some(result__).ok_or_else(windows_core::Error::from_win32)
}
#[cfg(feature = "Win32_Security")]
#[inline]
pub unsafe fn CreateFileMappingNumaW<P5>(hfile: super::super::Foundation::HANDLE, lpfilemappingattributes: Option<*const super::super::Security::SECURITY_ATTRIBUTES>, flprotect: PAGE_PROTECTION_FLAGS, dwmaximumsizehigh: u32, dwmaximumsizelow: u32, lpname: P5, nndpreferred: u32) -> windows_core::Result<super::super::Foundation::HANDLE>
where
    P5: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("kernel32.dll" "system" fn CreateFileMappingNumaW(hfile : super::super::Foundation:: HANDLE, lpfilemappingattributes : *const super::super::Security:: SECURITY_ATTRIBUTES, flprotect : PAGE_PROTECTION_FLAGS, dwmaximumsizehigh : u32, dwmaximumsizelow : u32, lpname : windows_core::PCWSTR, nndpreferred : u32) -> super::super::Foundation:: HANDLE);
    let result__ = CreateFileMappingNumaW(core::mem::transmute(hfile), core::mem::transmute(lpfilemappingattributes.unwrap_or(core::mem::zeroed())), core::mem::transmute(flprotect), core::mem::transmute(dwmaximumsizehigh), core::mem::transmute(dwmaximumsizelow), lpname.param().abi(), core::mem::transmute(nndpreferred));
    (!result__.is_invalid()).then_some(result__).ok_or_else(windows_core::Error::from_win32)
}
#[cfg(feature = "Win32_Security")]
#[inline]
pub unsafe fn CreateFileMappingW<P5>(hfile: super::super::Foundation::HANDLE, lpfilemappingattributes: Option<*const super::super::Security::SECURITY_ATTRIBUTES>, flprotect: PAGE_PROTECTION_FLAGS, dwmaximumsizehigh: u32, dwmaximumsizelow: u32, lpname: P5) -> windows_core::Result<super::super::Foundation::HANDLE>
where
    P5: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("kernel32.dll" "system" fn CreateFileMappingW(hfile : super::super::Foundation:: HANDLE, lpfilemappingattributes : *const super::super::Security:: SECURITY_ATTRIBUTES, flprotect : PAGE_PROTECTION_FLAGS, dwmaximumsizehigh : u32, dwmaximumsizelow : u32, lpname : windows_core::PCWSTR) -> super::super::Foundation:: HANDLE);
    let result__ = CreateFileMappingW(core::mem::transmute(hfile), core::mem::transmute(lpfilemappingattributes.unwrap_or(core::mem::zeroed())), core::mem::transmute(flprotect), core::mem::transmute(dwmaximumsizehigh), core::mem::transmute(dwmaximumsizelow), lpname.param().abi());
    (!result__.is_invalid()).then_some(result__).ok_or_else(windows_core::Error::from_win32)
}
#[inline]
pub unsafe fn CreateMemoryResourceNotification(notificationtype: MEMORY_RESOURCE_NOTIFICATION_TYPE) -> windows_core::Result<super::super::Foundation::HANDLE> {
    windows_targets::link!("kernel32.dll" "system" fn CreateMemoryResourceNotification(notificationtype : MEMORY_RESOURCE_NOTIFICATION_TYPE) -> super::super::Foundation:: HANDLE);
    let result__ = CreateMemoryResourceNotification(core::mem::transmute(notificationtype));
    (!result__.is_invalid()).then_some(result__).ok_or_else(windows_core::Error::from_win32)
}
#[inline]
pub unsafe fn DiscardVirtualMemory(virtualaddress: &mut [u8]) -> u32 {
    windows_targets::link!("kernel32.dll" "system" fn DiscardVirtualMemory(virtualaddress : *mut core::ffi::c_void, size : usize) -> u32);
    DiscardVirtualMemory(core::mem::transmute(virtualaddress.as_ptr()), virtualaddress.len().try_into().unwrap())
}
#[inline]
pub unsafe fn FlushViewOfFile(lpbaseaddress: *const core::ffi::c_void, dwnumberofbytestoflush: usize) -> windows_core::Result<()> {
    windows_targets::link!("kernel32.dll" "system" fn FlushViewOfFile(lpbaseaddress : *const core::ffi::c_void, dwnumberofbytestoflush : usize) -> super::super::Foundation:: BOOL);
    FlushViewOfFile(core::mem::transmute(lpbaseaddress), core::mem::transmute(dwnumberofbytestoflush)).ok()
}
#[inline]
pub unsafe fn FreeUserPhysicalPages(hprocess: super::super::Foundation::HANDLE, numberofpages: *mut usize, pagearray: *const usize) -> windows_core::Result<()> {
    windows_targets::link!("kernel32.dll" "system" fn FreeUserPhysicalPages(hprocess : super::super::Foundation:: HANDLE, numberofpages : *mut usize, pagearray : *const usize) -> super::super::Foundation:: BOOL);
    FreeUserPhysicalPages(core::mem::transmute(hprocess), core::mem::transmute(numberofpages), core::mem::transmute(pagearray)).ok()
}
#[inline]
pub unsafe fn GetLargePageMinimum() -> usize {
    windows_targets::link!("kernel32.dll" "system" fn GetLargePageMinimum() -> usize);
    GetLargePageMinimum()
}
#[inline]
pub unsafe fn GetMemoryErrorHandlingCapabilities(capabilities: *mut u32) -> windows_core::Result<()> {
    windows_targets::link!("kernel32.dll" "system" fn GetMemoryErrorHandlingCapabilities(capabilities : *mut u32) -> super::super::Foundation:: BOOL);
    GetMemoryErrorHandlingCapabilities(core::mem::transmute(capabilities)).ok()
}
#[inline]
pub unsafe fn GetProcessHeap() -> windows_core::Result<super::super::Foundation::HANDLE> {
    windows_targets::link!("kernel32.dll" "system" fn GetProcessHeap() -> super::super::Foundation:: HANDLE);
    let result__ = GetProcessHeap();
    (!result__.is_invalid()).then_some(result__).ok_or_else(windows_core::Error::from_win32)
}
#[inline]
pub unsafe fn GetProcessHeaps(processheaps: &mut [super::super::Foundation::HANDLE]) -> u32 {
    windows_targets::link!("kernel32.dll" "system" fn GetProcessHeaps(numberofheaps : u32, processheaps : *mut super::super::Foundation:: HANDLE) -> u32);
    GetProcessHeaps(processheaps.len().try_into().unwrap(), core::mem::transmute(processheaps.as_ptr()))
}
#[inline]
pub unsafe fn GetProcessWorkingSetSizeEx(hprocess: super::super::Foundation::HANDLE, lpminimumworkingsetsize: *mut usize, lpmaximumworkingsetsize: *mut usize, flags: *mut u32) -> super::super::Foundation::BOOL {
    windows_targets::link!("kernel32.dll" "system" fn GetProcessWorkingSetSizeEx(hprocess : super::super::Foundation:: HANDLE, lpminimumworkingsetsize : *mut usize, lpmaximumworkingsetsize : *mut usize, flags : *mut u32) -> super::super::Foundation:: BOOL);
    GetProcessWorkingSetSizeEx(core::mem::transmute(hprocess), core::mem::transmute(lpminimumworkingsetsize), core::mem::transmute(lpmaximumworkingsetsize), core::mem::transmute(flags))
}
#[inline]
pub unsafe fn GetSystemFileCacheSize(lpminimumfilecachesize: *mut usize, lpmaximumfilecachesize: *mut usize, lpflags: *mut u32) -> windows_core::Result<()> {
    windows_targets::link!("kernel32.dll" "system" fn GetSystemFileCacheSize(lpminimumfilecachesize : *mut usize, lpmaximumfilecachesize : *mut usize, lpflags : *mut u32) -> super::super::Foundation:: BOOL);
    GetSystemFileCacheSize(core::mem::transmute(lpminimumfilecachesize), core::mem::transmute(lpmaximumfilecachesize), core::mem::transmute(lpflags)).ok()
}
#[inline]
pub unsafe fn GetWriteWatch(dwflags: u32, lpbaseaddress: *const core::ffi::c_void, dwregionsize: usize, lpaddresses: Option<*mut *mut core::ffi::c_void>, lpdwcount: Option<*mut usize>, lpdwgranularity: Option<*mut u32>) -> u32 {
    windows_targets::link!("kernel32.dll" "system" fn GetWriteWatch(dwflags : u32, lpbaseaddress : *const core::ffi::c_void, dwregionsize : usize, lpaddresses : *mut *mut core::ffi::c_void, lpdwcount : *mut usize, lpdwgranularity : *mut u32) -> u32);
    GetWriteWatch(core::mem::transmute(dwflags), core::mem::transmute(lpbaseaddress), core::mem::transmute(dwregionsize), core::mem::transmute(lpaddresses.unwrap_or(core::mem::zeroed())), core::mem::transmute(lpdwcount.unwrap_or(core::mem::zeroed())), core::mem::transmute(lpdwgranularity.unwrap_or(core::mem::zeroed())))
}
#[inline]
pub unsafe fn GlobalAlloc(uflags: GLOBAL_ALLOC_FLAGS, dwbytes: usize) -> windows_core::Result<super::super::Foundation::HGLOBAL> {
    windows_targets::link!("kernel32.dll" "system" fn GlobalAlloc(uflags : GLOBAL_ALLOC_FLAGS, dwbytes : usize) -> super::super::Foundation:: HGLOBAL);
    let result__ = GlobalAlloc(core::mem::transmute(uflags), core::mem::transmute(dwbytes));
    (!result__.is_invalid()).then_some(result__).ok_or_else(windows_core::Error::from_win32)
}
#[inline]
pub unsafe fn GlobalFlags(hmem: super::super::Foundation::HGLOBAL) -> u32 {
    windows_targets::link!("kernel32.dll" "system" fn GlobalFlags(hmem : super::super::Foundation:: HGLOBAL) -> u32);
    GlobalFlags(core::mem::transmute(hmem))
}
#[inline]
pub unsafe fn GlobalHandle(pmem: *const core::ffi::c_void) -> windows_core::Result<super::super::Foundation::HGLOBAL> {
    windows_targets::link!("kernel32.dll" "system" fn GlobalHandle(pmem : *const core::ffi::c_void) -> super::super::Foundation:: HGLOBAL);
    let result__ = GlobalHandle(core::mem::transmute(pmem));
    (!result__.is_invalid()).then_some(result__).ok_or_else(windows_core::Error::from_win32)
}
#[inline]
pub unsafe fn GlobalLock(hmem: super::super::Foundation::HGLOBAL) -> *mut core::ffi::c_void {
    windows_targets::link!("kernel32.dll" "system" fn GlobalLock(hmem : super::super::Foundation:: HGLOBAL) -> *mut core::ffi::c_void);
    GlobalLock(core::mem::transmute(hmem))
}
#[inline]
pub unsafe fn GlobalReAlloc(hmem: super::super::Foundation::HGLOBAL, dwbytes: usize, uflags: u32) -> windows_core::Result<super::super::Foundation::HGLOBAL> {
    windows_targets::link!("kernel32.dll" "system" fn GlobalReAlloc(hmem : super::super::Foundation:: HGLOBAL, dwbytes : usize, uflags : u32) -> super::super::Foundation:: HGLOBAL);
    let result__ = GlobalReAlloc(core::mem::transmute(hmem), core::mem::transmute(dwbytes), core::mem::transmute(uflags));
    (!result__.is_invalid()).then_some(result__).ok_or_else(windows_core::Error::from_win32)
}
#[inline]
pub unsafe fn GlobalSize(hmem: super::super::Foundation::HGLOBAL) -> usize {
    windows_targets::link!("kernel32.dll" "system" fn GlobalSize(hmem : super::super::Foundation:: HGLOBAL) -> usize);
    GlobalSize(core::mem::transmute(hmem))
}
#[inline]
pub unsafe fn GlobalUnlock(hmem: super::super::Foundation::HGLOBAL) -> windows_core::Result<()> {
    windows_targets::link!("kernel32.dll" "system" fn GlobalUnlock(hmem : super::super::Foundation:: HGLOBAL) -> super::super::Foundation:: BOOL);
    GlobalUnlock(core::mem::transmute(hmem)).ok()
}
#[inline]
pub unsafe fn HeapAlloc(hheap: super::super::Foundation::HANDLE, dwflags: HEAP_FLAGS, dwbytes: usize) -> *mut core::ffi::c_void {
    windows_targets::link!("kernel32.dll" "system" fn HeapAlloc(hheap : super::super::Foundation:: HANDLE, dwflags : HEAP_FLAGS, dwbytes : usize) -> *mut core::ffi::c_void);
    HeapAlloc(core::mem::transmute(hheap), core::mem::transmute(dwflags), core::mem::transmute(dwbytes))
}
#[inline]
pub unsafe fn HeapCompact(hheap: super::super::Foundation::HANDLE, dwflags: HEAP_FLAGS) -> usize {
    windows_targets::link!("kernel32.dll" "system" fn HeapCompact(hheap : super::super::Foundation:: HANDLE, dwflags : HEAP_FLAGS) -> usize);
    HeapCompact(core::mem::transmute(hheap), core::mem::transmute(dwflags))
}
#[inline]
pub unsafe fn HeapCreate(floptions: HEAP_FLAGS, dwinitialsize: usize, dwmaximumsize: usize) -> windows_core::Result<super::super::Foundation::HANDLE> {
    windows_targets::link!("kernel32.dll" "system" fn HeapCreate(floptions : HEAP_FLAGS, dwinitialsize : usize, dwmaximumsize : usize) -> super::super::Foundation:: HANDLE);
    let result__ = HeapCreate(core::mem::transmute(floptions), core::mem::transmute(dwinitialsize), core::mem::transmute(dwmaximumsize));
    (!result__.is_invalid()).then_some(result__).ok_or_else(windows_core::Error::from_win32)
}
#[inline]
pub unsafe fn HeapDestroy(hheap: super::super::Foundation::HANDLE) -> windows_core::Result<()> {
    windows_targets::link!("kernel32.dll" "system" fn HeapDestroy(hheap : super::super::Foundation:: HANDLE) -> super::super::Foundation:: BOOL);
    HeapDestroy(core::mem::transmute(hheap)).ok()
}
#[inline]
pub unsafe fn HeapFree(hheap: super::super::Foundation::HANDLE, dwflags: HEAP_FLAGS, lpmem: Option<*const core::ffi::c_void>) -> windows_core::Result<()> {
    windows_targets::link!("kernel32.dll" "system" fn HeapFree(hheap : super::super::Foundation:: HANDLE, dwflags : HEAP_FLAGS, lpmem : *const core::ffi::c_void) -> super::super::Foundation:: BOOL);
    HeapFree(core::mem::transmute(hheap), core::mem::transmute(dwflags), core::mem::transmute(lpmem.unwrap_or(core::mem::zeroed()))).ok()
}
#[inline]
pub unsafe fn HeapLock(hheap: super::super::Foundation::HANDLE) -> windows_core::Result<()> {
    windows_targets::link!("kernel32.dll" "system" fn HeapLock(hheap : super::super::Foundation:: HANDLE) -> super::super::Foundation:: BOOL);
    HeapLock(core::mem::transmute(hheap)).ok()
}
#[inline]
pub unsafe fn HeapQueryInformation(heaphandle: Option<super::super::Foundation::HANDLE>, heapinformationclass: HEAP_INFORMATION_CLASS, heapinformation: Option<*mut core::ffi::c_void>, heapinformationlength: usize, returnlength: Option<*mut usize>) -> windows_core::Result<()> {
    windows_targets::link!("kernel32.dll" "system" fn HeapQueryInformation(heaphandle : super::super::Foundation:: HANDLE, heapinformationclass : HEAP_INFORMATION_CLASS, heapinformation : *mut core::ffi::c_void, heapinformationlength : usize, returnlength : *mut usize) -> super::super::Foundation:: BOOL);
    HeapQueryInformation(core::mem::transmute(heaphandle.unwrap_or(core::mem::zeroed())), core::mem::transmute(heapinformationclass), core::mem::transmute(heapinformation.unwrap_or(core::mem::zeroed())), core::mem::transmute(heapinformationlength), core::mem::transmute(returnlength.unwrap_or(core::mem::zeroed()))).ok()
}
#[inline]
pub unsafe fn HeapReAlloc(hheap: super::super::Foundation::HANDLE, dwflags: HEAP_FLAGS, lpmem: Option<*const core::ffi::c_void>, dwbytes: usize) -> *mut core::ffi::c_void {
    windows_targets::link!("kernel32.dll" "system" fn HeapReAlloc(hheap : super::super::Foundation:: HANDLE, dwflags : HEAP_FLAGS, lpmem : *const core::ffi::c_void, dwbytes : usize) -> *mut core::ffi::c_void);
    HeapReAlloc(core::mem::transmute(hheap), core::mem::transmute(dwflags), core::mem::transmute(lpmem.unwrap_or(core::mem::zeroed())), core::mem::transmute(dwbytes))
}
#[inline]
pub unsafe fn HeapSetInformation(heaphandle: Option<super::super::Foundation::HANDLE>, heapinformationclass: HEAP_INFORMATION_CLASS, heapinformation: Option<*const core::ffi::c_void>, heapinformationlength: usize) -> windows_core::Result<()> {
    windows_targets::link!("kernel32.dll" "system" fn HeapSetInformation(heaphandle : super::super::Foundation:: HANDLE, heapinformationclass : HEAP_INFORMATION_CLASS, heapinformation : *const core::ffi::c_void, heapinformationlength : usize) -> super::super::Foundation:: BOOL);
    HeapSetInformation(core::mem::transmute(heaphandle.unwrap_or(core::mem::zeroed())), core::mem::transmute(heapinformationclass), core::mem::transmute(heapinformation.unwrap_or(core::mem::zeroed())), core::mem::transmute(heapinformationlength)).ok()
}
#[inline]
pub unsafe fn HeapSize(hheap: super::super::Foundation::HANDLE, dwflags: HEAP_FLAGS, lpmem: *const core::ffi::c_void) -> usize {
    windows_targets::link!("kernel32.dll" "system" fn HeapSize(hheap : super::super::Foundation:: HANDLE, dwflags : HEAP_FLAGS, lpmem : *const core::ffi::c_void) -> usize);
    HeapSize(core::mem::transmute(hheap), core::mem::transmute(dwflags), core::mem::transmute(lpmem))
}
#[inline]
pub unsafe fn HeapSummary(hheap: super::super::Foundation::HANDLE, dwflags: u32, lpsummary: *mut HEAP_SUMMARY) -> super::super::Foundation::BOOL {
    windows_targets::link!("kernel32.dll" "system" fn HeapSummary(hheap : super::super::Foundation:: HANDLE, dwflags : u32, lpsummary : *mut HEAP_SUMMARY) -> super::super::Foundation:: BOOL);
    HeapSummary(core::mem::transmute(hheap), core::mem::transmute(dwflags), core::mem::transmute(lpsummary))
}
#[inline]
pub unsafe fn HeapUnlock(hheap: super::super::Foundation::HANDLE) -> windows_core::Result<()> {
    windows_targets::link!("kernel32.dll" "system" fn HeapUnlock(hheap : super::super::Foundation:: HANDLE) -> super::super::Foundation:: BOOL);
    HeapUnlock(core::mem::transmute(hheap)).ok()
}
#[inline]
pub unsafe fn HeapValidate(hheap: super::super::Foundation::HANDLE, dwflags: HEAP_FLAGS, lpmem: Option<*const core::ffi::c_void>) -> super::super::Foundation::BOOL {
    windows_targets::link!("kernel32.dll" "system" fn HeapValidate(hheap : super::super::Foundation:: HANDLE, dwflags : HEAP_FLAGS, lpmem : *const core::ffi::c_void) -> super::super::Foundation:: BOOL);
    HeapValidate(core::mem::transmute(hheap), core::mem::transmute(dwflags), core::mem::transmute(lpmem.unwrap_or(core::mem::zeroed())))
}
#[inline]
pub unsafe fn HeapWalk(hheap: super::super::Foundation::HANDLE, lpentry: *mut PROCESS_HEAP_ENTRY) -> windows_core::Result<()> {
    windows_targets::link!("kernel32.dll" "system" fn HeapWalk(hheap : super::super::Foundation:: HANDLE, lpentry : *mut PROCESS_HEAP_ENTRY) -> super::super::Foundation:: BOOL);
    HeapWalk(core::mem::transmute(hheap), core::mem::transmute(lpentry)).ok()
}
#[inline]
pub unsafe fn IsBadCodePtr(lpfn: Option<super::super::Foundation::FARPROC>) -> windows_core::Result<()> {
    windows_targets::link!("kernel32.dll" "system" fn IsBadCodePtr(lpfn : super::super::Foundation:: FARPROC) -> super::super::Foundation:: BOOL);
    IsBadCodePtr(core::mem::transmute(lpfn.unwrap_or(core::mem::zeroed()))).ok()
}
#[inline]
pub unsafe fn IsBadReadPtr(lp: Option<*const core::ffi::c_void>, ucb: usize) -> super::super::Foundation::BOOL {
    windows_targets::link!("kernel32.dll" "system" fn IsBadReadPtr(lp : *const core::ffi::c_void, ucb : usize) -> super::super::Foundation:: BOOL);
    IsBadReadPtr(core::mem::transmute(lp.unwrap_or(core::mem::zeroed())), core::mem::transmute(ucb))
}
#[inline]
pub unsafe fn IsBadStringPtrA<P0>(lpsz: P0, ucchmax: usize) -> super::super::Foundation::BOOL
where
    P0: windows_core::Param<windows_core::PCSTR>,
{
    windows_targets::link!("kernel32.dll" "system" fn IsBadStringPtrA(lpsz : windows_core::PCSTR, ucchmax : usize) -> super::super::Foundation:: BOOL);
    IsBadStringPtrA(lpsz.param().abi(), core::mem::transmute(ucchmax))
}
#[inline]
pub unsafe fn IsBadStringPtrW<P0>(lpsz: P0, ucchmax: usize) -> super::super::Foundation::BOOL
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("kernel32.dll" "system" fn IsBadStringPtrW(lpsz : windows_core::PCWSTR, ucchmax : usize) -> super::super::Foundation:: BOOL);
    IsBadStringPtrW(lpsz.param().abi(), core::mem::transmute(ucchmax))
}
#[inline]
pub unsafe fn IsBadWritePtr(lp: Option<*const core::ffi::c_void>, ucb: usize) -> super::super::Foundation::BOOL {
    windows_targets::link!("kernel32.dll" "system" fn IsBadWritePtr(lp : *const core::ffi::c_void, ucb : usize) -> super::super::Foundation:: BOOL);
    IsBadWritePtr(core::mem::transmute(lp.unwrap_or(core::mem::zeroed())), core::mem::transmute(ucb))
}
#[inline]
pub unsafe fn LocalAlloc(uflags: LOCAL_ALLOC_FLAGS, ubytes: usize) -> windows_core::Result<super::super::Foundation::HLOCAL> {
    windows_targets::link!("kernel32.dll" "system" fn LocalAlloc(uflags : LOCAL_ALLOC_FLAGS, ubytes : usize) -> super::super::Foundation:: HLOCAL);
    let result__ = LocalAlloc(core::mem::transmute(uflags), core::mem::transmute(ubytes));
    (!result__.is_invalid()).then_some(result__).ok_or_else(windows_core::Error::from_win32)
}
#[inline]
pub unsafe fn LocalFlags(hmem: super::super::Foundation::HLOCAL) -> u32 {
    windows_targets::link!("kernel32.dll" "system" fn LocalFlags(hmem : super::super::Foundation:: HLOCAL) -> u32);
    LocalFlags(core::mem::transmute(hmem))
}
#[inline]
pub unsafe fn LocalHandle(pmem: *const core::ffi::c_void) -> windows_core::Result<super::super::Foundation::HLOCAL> {
    windows_targets::link!("kernel32.dll" "system" fn LocalHandle(pmem : *const core::ffi::c_void) -> super::super::Foundation:: HLOCAL);
    let result__ = LocalHandle(core::mem::transmute(pmem));
    (!result__.is_invalid()).then_some(result__).ok_or_else(windows_core::Error::from_win32)
}
#[inline]
pub unsafe fn LocalLock(hmem: super::super::Foundation::HLOCAL) -> *mut core::ffi::c_void {
    windows_targets::link!("kernel32.dll" "system" fn LocalLock(hmem : super::super::Foundation:: HLOCAL) -> *mut core::ffi::c_void);
    LocalLock(core::mem::transmute(hmem))
}
#[inline]
pub unsafe fn LocalReAlloc(hmem: Option<super::super::Foundation::HLOCAL>, ubytes: usize, uflags: u32) -> windows_core::Result<super::super::Foundation::HLOCAL> {
    windows_targets::link!("kernel32.dll" "system" fn LocalReAlloc(hmem : super::super::Foundation:: HLOCAL, ubytes : usize, uflags : u32) -> super::super::Foundation:: HLOCAL);
    let result__ = LocalReAlloc(core::mem::transmute(hmem.unwrap_or(core::mem::zeroed())), core::mem::transmute(ubytes), core::mem::transmute(uflags));
    (!result__.is_invalid()).then_some(result__).ok_or_else(windows_core::Error::from_win32)
}
#[inline]
pub unsafe fn LocalSize(hmem: super::super::Foundation::HLOCAL) -> usize {
    windows_targets::link!("kernel32.dll" "system" fn LocalSize(hmem : super::super::Foundation:: HLOCAL) -> usize);
    LocalSize(core::mem::transmute(hmem))
}
#[inline]
pub unsafe fn LocalUnlock(hmem: super::super::Foundation::HLOCAL) -> windows_core::Result<()> {
    windows_targets::link!("kernel32.dll" "system" fn LocalUnlock(hmem : super::super::Foundation:: HLOCAL) -> super::super::Foundation:: BOOL);
    LocalUnlock(core::mem::transmute(hmem)).ok()
}
#[inline]
pub unsafe fn MapUserPhysicalPages(virtualaddress: *const core::ffi::c_void, pagearray: Option<&[usize]>) -> windows_core::Result<()> {
    windows_targets::link!("kernel32.dll" "system" fn MapUserPhysicalPages(virtualaddress : *const core::ffi::c_void, numberofpages : usize, pagearray : *const usize) -> super::super::Foundation:: BOOL);
    MapUserPhysicalPages(core::mem::transmute(virtualaddress), pagearray.as_deref().map_or(0, |slice| slice.len().try_into().unwrap()), core::mem::transmute(pagearray.as_deref().map_or(core::ptr::null(), |slice| slice.as_ptr()))).ok()
}
#[inline]
pub unsafe fn MapUserPhysicalPagesScatter(virtualaddresses: *const *const core::ffi::c_void, numberofpages: usize, pagearray: Option<*const usize>) -> windows_core::Result<()> {
    windows_targets::link!("kernel32.dll" "system" fn MapUserPhysicalPagesScatter(virtualaddresses : *const *const core::ffi::c_void, numberofpages : usize, pagearray : *const usize) -> super::super::Foundation:: BOOL);
    MapUserPhysicalPagesScatter(core::mem::transmute(virtualaddresses), core::mem::transmute(numberofpages), core::mem::transmute(pagearray.unwrap_or(core::mem::zeroed()))).ok()
}
#[inline]
pub unsafe fn MapViewOfFile(hfilemappingobject: super::super::Foundation::HANDLE, dwdesiredaccess: FILE_MAP, dwfileoffsethigh: u32, dwfileoffsetlow: u32, dwnumberofbytestomap: usize) -> MEMORY_MAPPED_VIEW_ADDRESS {
    windows_targets::link!("kernel32.dll" "system" fn MapViewOfFile(hfilemappingobject : super::super::Foundation:: HANDLE, dwdesiredaccess : FILE_MAP, dwfileoffsethigh : u32, dwfileoffsetlow : u32, dwnumberofbytestomap : usize) -> MEMORY_MAPPED_VIEW_ADDRESS);
    MapViewOfFile(core::mem::transmute(hfilemappingobject), core::mem::transmute(dwdesiredaccess), core::mem::transmute(dwfileoffsethigh), core::mem::transmute(dwfileoffsetlow), core::mem::transmute(dwnumberofbytestomap))
}
#[inline]
pub unsafe fn MapViewOfFile3(filemapping: super::super::Foundation::HANDLE, process: Option<super::super::Foundation::HANDLE>, baseaddress: Option<*const core::ffi::c_void>, offset: u64, viewsize: usize, allocationtype: VIRTUAL_ALLOCATION_TYPE, pageprotection: u32, extendedparameters: Option<&mut [MEM_EXTENDED_PARAMETER]>) -> MEMORY_MAPPED_VIEW_ADDRESS {
    windows_targets::link!("api-ms-win-core-memory-l1-1-6.dll" "system" fn MapViewOfFile3(filemapping : super::super::Foundation:: HANDLE, process : super::super::Foundation:: HANDLE, baseaddress : *const core::ffi::c_void, offset : u64, viewsize : usize, allocationtype : VIRTUAL_ALLOCATION_TYPE, pageprotection : u32, extendedparameters : *mut MEM_EXTENDED_PARAMETER, parametercount : u32) -> MEMORY_MAPPED_VIEW_ADDRESS);
    MapViewOfFile3(
        core::mem::transmute(filemapping),
        core::mem::transmute(process.unwrap_or(core::mem::zeroed())),
        core::mem::transmute(baseaddress.unwrap_or(core::mem::zeroed())),
        core::mem::transmute(offset),
        core::mem::transmute(viewsize),
        core::mem::transmute(allocationtype),
        core::mem::transmute(pageprotection),
        core::mem::transmute(extendedparameters.as_deref().map_or(core::ptr::null(), |slice| slice.as_ptr())),
        extendedparameters.as_deref().map_or(0, |slice| slice.len().try_into().unwrap()),
    )
}
#[inline]
pub unsafe fn MapViewOfFile3FromApp(filemapping: super::super::Foundation::HANDLE, process: Option<super::super::Foundation::HANDLE>, baseaddress: Option<*const core::ffi::c_void>, offset: u64, viewsize: usize, allocationtype: VIRTUAL_ALLOCATION_TYPE, pageprotection: u32, extendedparameters: Option<&mut [MEM_EXTENDED_PARAMETER]>) -> MEMORY_MAPPED_VIEW_ADDRESS {
    windows_targets::link!("api-ms-win-core-memory-l1-1-6.dll" "system" fn MapViewOfFile3FromApp(filemapping : super::super::Foundation:: HANDLE, process : super::super::Foundation:: HANDLE, baseaddress : *const core::ffi::c_void, offset : u64, viewsize : usize, allocationtype : VIRTUAL_ALLOCATION_TYPE, pageprotection : u32, extendedparameters : *mut MEM_EXTENDED_PARAMETER, parametercount : u32) -> MEMORY_MAPPED_VIEW_ADDRESS);
    MapViewOfFile3FromApp(
        core::mem::transmute(filemapping),
        core::mem::transmute(process.unwrap_or(core::mem::zeroed())),
        core::mem::transmute(baseaddress.unwrap_or(core::mem::zeroed())),
        core::mem::transmute(offset),
        core::mem::transmute(viewsize),
        core::mem::transmute(allocationtype),
        core::mem::transmute(pageprotection),
        core::mem::transmute(extendedparameters.as_deref().map_or(core::ptr::null(), |slice| slice.as_ptr())),
        extendedparameters.as_deref().map_or(0, |slice| slice.len().try_into().unwrap()),
    )
}
#[inline]
pub unsafe fn MapViewOfFileEx(hfilemappingobject: super::super::Foundation::HANDLE, dwdesiredaccess: FILE_MAP, dwfileoffsethigh: u32, dwfileoffsetlow: u32, dwnumberofbytestomap: usize, lpbaseaddress: Option<*const core::ffi::c_void>) -> MEMORY_MAPPED_VIEW_ADDRESS {
    windows_targets::link!("kernel32.dll" "system" fn MapViewOfFileEx(hfilemappingobject : super::super::Foundation:: HANDLE, dwdesiredaccess : FILE_MAP, dwfileoffsethigh : u32, dwfileoffsetlow : u32, dwnumberofbytestomap : usize, lpbaseaddress : *const core::ffi::c_void) -> MEMORY_MAPPED_VIEW_ADDRESS);
    MapViewOfFileEx(core::mem::transmute(hfilemappingobject), core::mem::transmute(dwdesiredaccess), core::mem::transmute(dwfileoffsethigh), core::mem::transmute(dwfileoffsetlow), core::mem::transmute(dwnumberofbytestomap), core::mem::transmute(lpbaseaddress.unwrap_or(core::mem::zeroed())))
}
#[inline]
pub unsafe fn MapViewOfFileExNuma(hfilemappingobject: super::super::Foundation::HANDLE, dwdesiredaccess: FILE_MAP, dwfileoffsethigh: u32, dwfileoffsetlow: u32, dwnumberofbytestomap: usize, lpbaseaddress: Option<*const core::ffi::c_void>, nndpreferred: u32) -> MEMORY_MAPPED_VIEW_ADDRESS {
    windows_targets::link!("kernel32.dll" "system" fn MapViewOfFileExNuma(hfilemappingobject : super::super::Foundation:: HANDLE, dwdesiredaccess : FILE_MAP, dwfileoffsethigh : u32, dwfileoffsetlow : u32, dwnumberofbytestomap : usize, lpbaseaddress : *const core::ffi::c_void, nndpreferred : u32) -> MEMORY_MAPPED_VIEW_ADDRESS);
    MapViewOfFileExNuma(core::mem::transmute(hfilemappingobject), core::mem::transmute(dwdesiredaccess), core::mem::transmute(dwfileoffsethigh), core::mem::transmute(dwfileoffsetlow), core::mem::transmute(dwnumberofbytestomap), core::mem::transmute(lpbaseaddress.unwrap_or(core::mem::zeroed())), core::mem::transmute(nndpreferred))
}
#[inline]
pub unsafe fn MapViewOfFileFromApp(hfilemappingobject: super::super::Foundation::HANDLE, desiredaccess: FILE_MAP, fileoffset: u64, numberofbytestomap: usize) -> MEMORY_MAPPED_VIEW_ADDRESS {
    windows_targets::link!("kernel32.dll" "system" fn MapViewOfFileFromApp(hfilemappingobject : super::super::Foundation:: HANDLE, desiredaccess : FILE_MAP, fileoffset : u64, numberofbytestomap : usize) -> MEMORY_MAPPED_VIEW_ADDRESS);
    MapViewOfFileFromApp(core::mem::transmute(hfilemappingobject), core::mem::transmute(desiredaccess), core::mem::transmute(fileoffset), core::mem::transmute(numberofbytestomap))
}
#[inline]
pub unsafe fn MapViewOfFileNuma2(filemappinghandle: super::super::Foundation::HANDLE, processhandle: super::super::Foundation::HANDLE, offset: u64, baseaddress: Option<*const core::ffi::c_void>, viewsize: usize, allocationtype: u32, pageprotection: u32, preferrednode: u32) -> MEMORY_MAPPED_VIEW_ADDRESS {
    windows_targets::link!("api-ms-win-core-memory-l1-1-5.dll" "system" fn MapViewOfFileNuma2(filemappinghandle : super::super::Foundation:: HANDLE, processhandle : super::super::Foundation:: HANDLE, offset : u64, baseaddress : *const core::ffi::c_void, viewsize : usize, allocationtype : u32, pageprotection : u32, preferrednode : u32) -> MEMORY_MAPPED_VIEW_ADDRESS);
    MapViewOfFileNuma2(core::mem::transmute(filemappinghandle), core::mem::transmute(processhandle), core::mem::transmute(offset), core::mem::transmute(baseaddress.unwrap_or(core::mem::zeroed())), core::mem::transmute(viewsize), core::mem::transmute(allocationtype), core::mem::transmute(pageprotection), core::mem::transmute(preferrednode))
}
#[inline]
pub unsafe fn OfferVirtualMemory(virtualaddress: &mut [u8], priority: OFFER_PRIORITY) -> u32 {
    windows_targets::link!("kernel32.dll" "system" fn OfferVirtualMemory(virtualaddress : *mut core::ffi::c_void, size : usize, priority : OFFER_PRIORITY) -> u32);
    OfferVirtualMemory(core::mem::transmute(virtualaddress.as_ptr()), virtualaddress.len().try_into().unwrap(), core::mem::transmute(priority))
}
#[inline]
pub unsafe fn OpenDedicatedMemoryPartition(partition: super::super::Foundation::HANDLE, dedicatedmemorytypeid: u64, desiredaccess: u32, inherithandle: bool) -> super::super::Foundation::HANDLE {
    windows_targets::link!("api-ms-win-core-memory-l1-1-8.dll" "system" fn OpenDedicatedMemoryPartition(partition : super::super::Foundation:: HANDLE, dedicatedmemorytypeid : u64, desiredaccess : u32, inherithandle : super::super::Foundation:: BOOL) -> super::super::Foundation:: HANDLE);
    OpenDedicatedMemoryPartition(core::mem::transmute(partition), core::mem::transmute(dedicatedmemorytypeid), core::mem::transmute(desiredaccess), inherithandle.into())
}
#[inline]
pub unsafe fn OpenFileMappingA<P2>(dwdesiredaccess: u32, binherithandle: bool, lpname: P2) -> windows_core::Result<super::super::Foundation::HANDLE>
where
    P2: windows_core::Param<windows_core::PCSTR>,
{
    windows_targets::link!("kernel32.dll" "system" fn OpenFileMappingA(dwdesiredaccess : u32, binherithandle : super::super::Foundation:: BOOL, lpname : windows_core::PCSTR) -> super::super::Foundation:: HANDLE);
    let result__ = OpenFileMappingA(core::mem::transmute(dwdesiredaccess), binherithandle.into(), lpname.param().abi());
    (!result__.is_invalid()).then_some(result__).ok_or_else(windows_core::Error::from_win32)
}
#[inline]
pub unsafe fn OpenFileMappingFromApp<P2>(desiredaccess: u32, inherithandle: bool, name: P2) -> windows_core::Result<super::super::Foundation::HANDLE>
where
    P2: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("api-ms-win-core-memory-l1-1-3.dll" "system" fn OpenFileMappingFromApp(desiredaccess : u32, inherithandle : super::super::Foundation:: BOOL, name : windows_core::PCWSTR) -> super::super::Foundation:: HANDLE);
    let result__ = OpenFileMappingFromApp(core::mem::transmute(desiredaccess), inherithandle.into(), name.param().abi());
    (!result__.is_invalid()).then_some(result__).ok_or_else(windows_core::Error::from_win32)
}
#[inline]
pub unsafe fn OpenFileMappingW<P2>(dwdesiredaccess: u32, binherithandle: bool, lpname: P2) -> windows_core::Result<super::super::Foundation::HANDLE>
where
    P2: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("kernel32.dll" "system" fn OpenFileMappingW(dwdesiredaccess : u32, binherithandle : super::super::Foundation:: BOOL, lpname : windows_core::PCWSTR) -> super::super::Foundation:: HANDLE);
    let result__ = OpenFileMappingW(core::mem::transmute(dwdesiredaccess), binherithandle.into(), lpname.param().abi());
    (!result__.is_invalid()).then_some(result__).ok_or_else(windows_core::Error::from_win32)
}
#[inline]
pub unsafe fn PrefetchVirtualMemory(hprocess: super::super::Foundation::HANDLE, virtualaddresses: &[WIN32_MEMORY_RANGE_ENTRY], flags: u32) -> windows_core::Result<()> {
    windows_targets::link!("kernel32.dll" "system" fn PrefetchVirtualMemory(hprocess : super::super::Foundation:: HANDLE, numberofentries : usize, virtualaddresses : *const WIN32_MEMORY_RANGE_ENTRY, flags : u32) -> super::super::Foundation:: BOOL);
    PrefetchVirtualMemory(core::mem::transmute(hprocess), virtualaddresses.len().try_into().unwrap(), core::mem::transmute(virtualaddresses.as_ptr()), core::mem::transmute(flags)).ok()
}
#[inline]
pub unsafe fn QueryMemoryResourceNotification(resourcenotificationhandle: super::super::Foundation::HANDLE, resourcestate: *mut super::super::Foundation::BOOL) -> windows_core::Result<()> {
    windows_targets::link!("kernel32.dll" "system" fn QueryMemoryResourceNotification(resourcenotificationhandle : super::super::Foundation:: HANDLE, resourcestate : *mut super::super::Foundation:: BOOL) -> super::super::Foundation:: BOOL);
    QueryMemoryResourceNotification(core::mem::transmute(resourcenotificationhandle), core::mem::transmute(resourcestate)).ok()
}
#[inline]
pub unsafe fn QueryPartitionInformation(partition: super::super::Foundation::HANDLE, partitioninformationclass: WIN32_MEMORY_PARTITION_INFORMATION_CLASS, partitioninformation: *mut core::ffi::c_void, partitioninformationlength: u32) -> super::super::Foundation::BOOL {
    windows_targets::link!("api-ms-win-core-memory-l1-1-8.dll" "system" fn QueryPartitionInformation(partition : super::super::Foundation:: HANDLE, partitioninformationclass : WIN32_MEMORY_PARTITION_INFORMATION_CLASS, partitioninformation : *mut core::ffi::c_void, partitioninformationlength : u32) -> super::super::Foundation:: BOOL);
    QueryPartitionInformation(core::mem::transmute(partition), core::mem::transmute(partitioninformationclass), core::mem::transmute(partitioninformation), core::mem::transmute(partitioninformationlength))
}
#[inline]
pub unsafe fn QueryVirtualMemoryInformation(process: super::super::Foundation::HANDLE, virtualaddress: *const core::ffi::c_void, memoryinformationclass: WIN32_MEMORY_INFORMATION_CLASS, memoryinformation: *mut core::ffi::c_void, memoryinformationsize: usize, returnsize: Option<*mut usize>) -> windows_core::Result<()> {
    windows_targets::link!("api-ms-win-core-memory-l1-1-4.dll" "system" fn QueryVirtualMemoryInformation(process : super::super::Foundation:: HANDLE, virtualaddress : *const core::ffi::c_void, memoryinformationclass : WIN32_MEMORY_INFORMATION_CLASS, memoryinformation : *mut core::ffi::c_void, memoryinformationsize : usize, returnsize : *mut usize) -> super::super::Foundation:: BOOL);
    QueryVirtualMemoryInformation(core::mem::transmute(process), core::mem::transmute(virtualaddress), core::mem::transmute(memoryinformationclass), core::mem::transmute(memoryinformation), core::mem::transmute(memoryinformationsize), core::mem::transmute(returnsize.unwrap_or(core::mem::zeroed()))).ok()
}
#[inline]
pub unsafe fn ReclaimVirtualMemory(virtualaddress: &[u8]) -> u32 {
    windows_targets::link!("kernel32.dll" "system" fn ReclaimVirtualMemory(virtualaddress : *const core::ffi::c_void, size : usize) -> u32);
    ReclaimVirtualMemory(core::mem::transmute(virtualaddress.as_ptr()), virtualaddress.len().try_into().unwrap())
}
#[inline]
pub unsafe fn RegisterBadMemoryNotification(callback: PBAD_MEMORY_CALLBACK_ROUTINE) -> *mut core::ffi::c_void {
    windows_targets::link!("kernel32.dll" "system" fn RegisterBadMemoryNotification(callback : PBAD_MEMORY_CALLBACK_ROUTINE) -> *mut core::ffi::c_void);
    RegisterBadMemoryNotification(core::mem::transmute(callback))
}
#[inline]
pub unsafe fn RemoveSecureMemoryCacheCallback(pfncallback: PSECURE_MEMORY_CACHE_CALLBACK) -> super::super::Foundation::BOOL {
    windows_targets::link!("kernel32.dll" "system" fn RemoveSecureMemoryCacheCallback(pfncallback : PSECURE_MEMORY_CACHE_CALLBACK) -> super::super::Foundation:: BOOL);
    RemoveSecureMemoryCacheCallback(core::mem::transmute(pfncallback))
}
#[inline]
pub unsafe fn ResetWriteWatch(lpbaseaddress: *const core::ffi::c_void, dwregionsize: usize) -> u32 {
    windows_targets::link!("kernel32.dll" "system" fn ResetWriteWatch(lpbaseaddress : *const core::ffi::c_void, dwregionsize : usize) -> u32);
    ResetWriteWatch(core::mem::transmute(lpbaseaddress), core::mem::transmute(dwregionsize))
}
#[inline]
pub unsafe fn RtlCompareMemory(source1: *const core::ffi::c_void, source2: *const core::ffi::c_void, length: usize) -> usize {
    windows_targets::link!("kernel32.dll" "system" fn RtlCompareMemory(source1 : *const core::ffi::c_void, source2 : *const core::ffi::c_void, length : usize) -> usize);
    RtlCompareMemory(core::mem::transmute(source1), core::mem::transmute(source2), core::mem::transmute(length))
}
#[inline]
pub unsafe fn RtlCrc32(buffer: *const core::ffi::c_void, size: usize, initialcrc: u32) -> u32 {
    windows_targets::link!("ntdll.dll" "system" fn RtlCrc32(buffer : *const core::ffi::c_void, size : usize, initialcrc : u32) -> u32);
    RtlCrc32(core::mem::transmute(buffer), core::mem::transmute(size), core::mem::transmute(initialcrc))
}
#[inline]
pub unsafe fn RtlCrc64(buffer: *const core::ffi::c_void, size: usize, initialcrc: u64) -> u64 {
    windows_targets::link!("ntdll.dll" "system" fn RtlCrc64(buffer : *const core::ffi::c_void, size : usize, initialcrc : u64) -> u64);
    RtlCrc64(core::mem::transmute(buffer), core::mem::transmute(size), core::mem::transmute(initialcrc))
}
#[inline]
pub unsafe fn RtlIsZeroMemory(buffer: *const core::ffi::c_void, length: usize) -> super::super::Foundation::BOOLEAN {
    windows_targets::link!("ntdll.dll" "system" fn RtlIsZeroMemory(buffer : *const core::ffi::c_void, length : usize) -> super::super::Foundation:: BOOLEAN);
    RtlIsZeroMemory(core::mem::transmute(buffer), core::mem::transmute(length))
}
#[inline]
pub unsafe fn SetProcessValidCallTargets(hprocess: super::super::Foundation::HANDLE, virtualaddress: *const core::ffi::c_void, regionsize: usize, offsetinformation: &mut [CFG_CALL_TARGET_INFO]) -> windows_core::Result<()> {
    windows_targets::link!("api-ms-win-core-memory-l1-1-3.dll" "system" fn SetProcessValidCallTargets(hprocess : super::super::Foundation:: HANDLE, virtualaddress : *const core::ffi::c_void, regionsize : usize, numberofoffsets : u32, offsetinformation : *mut CFG_CALL_TARGET_INFO) -> super::super::Foundation:: BOOL);
    SetProcessValidCallTargets(core::mem::transmute(hprocess), core::mem::transmute(virtualaddress), core::mem::transmute(regionsize), offsetinformation.len().try_into().unwrap(), core::mem::transmute(offsetinformation.as_ptr())).ok()
}
#[inline]
pub unsafe fn SetProcessValidCallTargetsForMappedView(process: super::super::Foundation::HANDLE, virtualaddress: *const core::ffi::c_void, regionsize: usize, offsetinformation: &mut [CFG_CALL_TARGET_INFO], section: super::super::Foundation::HANDLE, expectedfileoffset: u64) -> super::super::Foundation::BOOL {
    windows_targets::link!("api-ms-win-core-memory-l1-1-7.dll" "system" fn SetProcessValidCallTargetsForMappedView(process : super::super::Foundation:: HANDLE, virtualaddress : *const core::ffi::c_void, regionsize : usize, numberofoffsets : u32, offsetinformation : *mut CFG_CALL_TARGET_INFO, section : super::super::Foundation:: HANDLE, expectedfileoffset : u64) -> super::super::Foundation:: BOOL);
    SetProcessValidCallTargetsForMappedView(core::mem::transmute(process), core::mem::transmute(virtualaddress), core::mem::transmute(regionsize), offsetinformation.len().try_into().unwrap(), core::mem::transmute(offsetinformation.as_ptr()), core::mem::transmute(section), core::mem::transmute(expectedfileoffset))
}
#[inline]
pub unsafe fn SetProcessWorkingSetSizeEx(hprocess: super::super::Foundation::HANDLE, dwminimumworkingsetsize: usize, dwmaximumworkingsetsize: usize, flags: SETPROCESSWORKINGSETSIZEEX_FLAGS) -> windows_core::Result<()> {
    windows_targets::link!("kernel32.dll" "system" fn SetProcessWorkingSetSizeEx(hprocess : super::super::Foundation:: HANDLE, dwminimumworkingsetsize : usize, dwmaximumworkingsetsize : usize, flags : SETPROCESSWORKINGSETSIZEEX_FLAGS) -> super::super::Foundation:: BOOL);
    SetProcessWorkingSetSizeEx(core::mem::transmute(hprocess), core::mem::transmute(dwminimumworkingsetsize), core::mem::transmute(dwmaximumworkingsetsize), core::mem::transmute(flags)).ok()
}
#[inline]
pub unsafe fn SetSystemFileCacheSize(minimumfilecachesize: usize, maximumfilecachesize: usize, flags: u32) -> windows_core::Result<()> {
    windows_targets::link!("kernel32.dll" "system" fn SetSystemFileCacheSize(minimumfilecachesize : usize, maximumfilecachesize : usize, flags : u32) -> super::super::Foundation:: BOOL);
    SetSystemFileCacheSize(core::mem::transmute(minimumfilecachesize), core::mem::transmute(maximumfilecachesize), core::mem::transmute(flags)).ok()
}
#[inline]
pub unsafe fn UnmapViewOfFile(lpbaseaddress: MEMORY_MAPPED_VIEW_ADDRESS) -> windows_core::Result<()> {
    windows_targets::link!("kernel32.dll" "system" fn UnmapViewOfFile(lpbaseaddress : MEMORY_MAPPED_VIEW_ADDRESS) -> super::super::Foundation:: BOOL);
    UnmapViewOfFile(core::mem::transmute(lpbaseaddress)).ok()
}
#[inline]
pub unsafe fn UnmapViewOfFile2(process: super::super::Foundation::HANDLE, baseaddress: MEMORY_MAPPED_VIEW_ADDRESS, unmapflags: UNMAP_VIEW_OF_FILE_FLAGS) -> windows_core::Result<()> {
    windows_targets::link!("api-ms-win-core-memory-l1-1-5.dll" "system" fn UnmapViewOfFile2(process : super::super::Foundation:: HANDLE, baseaddress : MEMORY_MAPPED_VIEW_ADDRESS, unmapflags : UNMAP_VIEW_OF_FILE_FLAGS) -> super::super::Foundation:: BOOL);
    UnmapViewOfFile2(core::mem::transmute(process), core::mem::transmute(baseaddress), core::mem::transmute(unmapflags)).ok()
}
#[inline]
pub unsafe fn UnmapViewOfFileEx(baseaddress: MEMORY_MAPPED_VIEW_ADDRESS, unmapflags: UNMAP_VIEW_OF_FILE_FLAGS) -> windows_core::Result<()> {
    windows_targets::link!("kernel32.dll" "system" fn UnmapViewOfFileEx(baseaddress : MEMORY_MAPPED_VIEW_ADDRESS, unmapflags : UNMAP_VIEW_OF_FILE_FLAGS) -> super::super::Foundation:: BOOL);
    UnmapViewOfFileEx(core::mem::transmute(baseaddress), core::mem::transmute(unmapflags)).ok()
}
#[inline]
pub unsafe fn UnregisterBadMemoryNotification(registrationhandle: *const core::ffi::c_void) -> windows_core::Result<()> {
    windows_targets::link!("kernel32.dll" "system" fn UnregisterBadMemoryNotification(registrationhandle : *const core::ffi::c_void) -> super::super::Foundation:: BOOL);
    UnregisterBadMemoryNotification(core::mem::transmute(registrationhandle)).ok()
}
#[inline]
pub unsafe fn VirtualAlloc(lpaddress: Option<*const core::ffi::c_void>, dwsize: usize, flallocationtype: VIRTUAL_ALLOCATION_TYPE, flprotect: PAGE_PROTECTION_FLAGS) -> *mut core::ffi::c_void {
    windows_targets::link!("kernel32.dll" "system" fn VirtualAlloc(lpaddress : *const core::ffi::c_void, dwsize : usize, flallocationtype : VIRTUAL_ALLOCATION_TYPE, flprotect : PAGE_PROTECTION_FLAGS) -> *mut core::ffi::c_void);
    VirtualAlloc(core::mem::transmute(lpaddress.unwrap_or(core::mem::zeroed())), core::mem::transmute(dwsize), core::mem::transmute(flallocationtype), core::mem::transmute(flprotect))
}
#[inline]
pub unsafe fn VirtualAlloc2(process: Option<super::super::Foundation::HANDLE>, baseaddress: Option<*const core::ffi::c_void>, size: usize, allocationtype: VIRTUAL_ALLOCATION_TYPE, pageprotection: u32, extendedparameters: Option<&mut [MEM_EXTENDED_PARAMETER]>) -> *mut core::ffi::c_void {
    windows_targets::link!("api-ms-win-core-memory-l1-1-6.dll" "system" fn VirtualAlloc2(process : super::super::Foundation:: HANDLE, baseaddress : *const core::ffi::c_void, size : usize, allocationtype : VIRTUAL_ALLOCATION_TYPE, pageprotection : u32, extendedparameters : *mut MEM_EXTENDED_PARAMETER, parametercount : u32) -> *mut core::ffi::c_void);
    VirtualAlloc2(core::mem::transmute(process.unwrap_or(core::mem::zeroed())), core::mem::transmute(baseaddress.unwrap_or(core::mem::zeroed())), core::mem::transmute(size), core::mem::transmute(allocationtype), core::mem::transmute(pageprotection), core::mem::transmute(extendedparameters.as_deref().map_or(core::ptr::null(), |slice| slice.as_ptr())), extendedparameters.as_deref().map_or(0, |slice| slice.len().try_into().unwrap()))
}
#[inline]
pub unsafe fn VirtualAlloc2FromApp(process: Option<super::super::Foundation::HANDLE>, baseaddress: Option<*const core::ffi::c_void>, size: usize, allocationtype: VIRTUAL_ALLOCATION_TYPE, pageprotection: u32, extendedparameters: Option<&mut [MEM_EXTENDED_PARAMETER]>) -> *mut core::ffi::c_void {
    windows_targets::link!("api-ms-win-core-memory-l1-1-6.dll" "system" fn VirtualAlloc2FromApp(process : super::super::Foundation:: HANDLE, baseaddress : *const core::ffi::c_void, size : usize, allocationtype : VIRTUAL_ALLOCATION_TYPE, pageprotection : u32, extendedparameters : *mut MEM_EXTENDED_PARAMETER, parametercount : u32) -> *mut core::ffi::c_void);
    VirtualAlloc2FromApp(core::mem::transmute(process.unwrap_or(core::mem::zeroed())), core::mem::transmute(baseaddress.unwrap_or(core::mem::zeroed())), core::mem::transmute(size), core::mem::transmute(allocationtype), core::mem::transmute(pageprotection), core::mem::transmute(extendedparameters.as_deref().map_or(core::ptr::null(), |slice| slice.as_ptr())), extendedparameters.as_deref().map_or(0, |slice| slice.len().try_into().unwrap()))
}
#[inline]
pub unsafe fn VirtualAllocEx(hprocess: super::super::Foundation::HANDLE, lpaddress: Option<*const core::ffi::c_void>, dwsize: usize, flallocationtype: VIRTUAL_ALLOCATION_TYPE, flprotect: PAGE_PROTECTION_FLAGS) -> *mut core::ffi::c_void {
    windows_targets::link!("kernel32.dll" "system" fn VirtualAllocEx(hprocess : super::super::Foundation:: HANDLE, lpaddress : *const core::ffi::c_void, dwsize : usize, flallocationtype : VIRTUAL_ALLOCATION_TYPE, flprotect : PAGE_PROTECTION_FLAGS) -> *mut core::ffi::c_void);
    VirtualAllocEx(core::mem::transmute(hprocess), core::mem::transmute(lpaddress.unwrap_or(core::mem::zeroed())), core::mem::transmute(dwsize), core::mem::transmute(flallocationtype), core::mem::transmute(flprotect))
}
#[inline]
pub unsafe fn VirtualAllocExNuma(hprocess: super::super::Foundation::HANDLE, lpaddress: Option<*const core::ffi::c_void>, dwsize: usize, flallocationtype: VIRTUAL_ALLOCATION_TYPE, flprotect: u32, nndpreferred: u32) -> *mut core::ffi::c_void {
    windows_targets::link!("kernel32.dll" "system" fn VirtualAllocExNuma(hprocess : super::super::Foundation:: HANDLE, lpaddress : *const core::ffi::c_void, dwsize : usize, flallocationtype : VIRTUAL_ALLOCATION_TYPE, flprotect : u32, nndpreferred : u32) -> *mut core::ffi::c_void);
    VirtualAllocExNuma(core::mem::transmute(hprocess), core::mem::transmute(lpaddress.unwrap_or(core::mem::zeroed())), core::mem::transmute(dwsize), core::mem::transmute(flallocationtype), core::mem::transmute(flprotect), core::mem::transmute(nndpreferred))
}
#[inline]
pub unsafe fn VirtualAllocFromApp(baseaddress: Option<*const core::ffi::c_void>, size: usize, allocationtype: VIRTUAL_ALLOCATION_TYPE, protection: u32) -> *mut core::ffi::c_void {
    windows_targets::link!("api-ms-win-core-memory-l1-1-3.dll" "system" fn VirtualAllocFromApp(baseaddress : *const core::ffi::c_void, size : usize, allocationtype : VIRTUAL_ALLOCATION_TYPE, protection : u32) -> *mut core::ffi::c_void);
    VirtualAllocFromApp(core::mem::transmute(baseaddress.unwrap_or(core::mem::zeroed())), core::mem::transmute(size), core::mem::transmute(allocationtype), core::mem::transmute(protection))
}
#[inline]
pub unsafe fn VirtualFree(lpaddress: *mut core::ffi::c_void, dwsize: usize, dwfreetype: VIRTUAL_FREE_TYPE) -> windows_core::Result<()> {
    windows_targets::link!("kernel32.dll" "system" fn VirtualFree(lpaddress : *mut core::ffi::c_void, dwsize : usize, dwfreetype : VIRTUAL_FREE_TYPE) -> super::super::Foundation:: BOOL);
    VirtualFree(core::mem::transmute(lpaddress), core::mem::transmute(dwsize), core::mem::transmute(dwfreetype)).ok()
}
#[inline]
pub unsafe fn VirtualFreeEx(hprocess: super::super::Foundation::HANDLE, lpaddress: *mut core::ffi::c_void, dwsize: usize, dwfreetype: VIRTUAL_FREE_TYPE) -> windows_core::Result<()> {
    windows_targets::link!("kernel32.dll" "system" fn VirtualFreeEx(hprocess : super::super::Foundation:: HANDLE, lpaddress : *mut core::ffi::c_void, dwsize : usize, dwfreetype : VIRTUAL_FREE_TYPE) -> super::super::Foundation:: BOOL);
    VirtualFreeEx(core::mem::transmute(hprocess), core::mem::transmute(lpaddress), core::mem::transmute(dwsize), core::mem::transmute(dwfreetype)).ok()
}
#[inline]
pub unsafe fn VirtualLock(lpaddress: *const core::ffi::c_void, dwsize: usize) -> windows_core::Result<()> {
    windows_targets::link!("kernel32.dll" "system" fn VirtualLock(lpaddress : *const core::ffi::c_void, dwsize : usize) -> super::super::Foundation:: BOOL);
    VirtualLock(core::mem::transmute(lpaddress), core::mem::transmute(dwsize)).ok()
}
#[inline]
pub unsafe fn VirtualProtect(lpaddress: *const core::ffi::c_void, dwsize: usize, flnewprotect: PAGE_PROTECTION_FLAGS, lpfloldprotect: *mut PAGE_PROTECTION_FLAGS) -> windows_core::Result<()> {
    windows_targets::link!("kernel32.dll" "system" fn VirtualProtect(lpaddress : *const core::ffi::c_void, dwsize : usize, flnewprotect : PAGE_PROTECTION_FLAGS, lpfloldprotect : *mut PAGE_PROTECTION_FLAGS) -> super::super::Foundation:: BOOL);
    VirtualProtect(core::mem::transmute(lpaddress), core::mem::transmute(dwsize), core::mem::transmute(flnewprotect), core::mem::transmute(lpfloldprotect)).ok()
}
#[inline]
pub unsafe fn VirtualProtectEx(hprocess: super::super::Foundation::HANDLE, lpaddress: *const core::ffi::c_void, dwsize: usize, flnewprotect: PAGE_PROTECTION_FLAGS, lpfloldprotect: *mut PAGE_PROTECTION_FLAGS) -> windows_core::Result<()> {
    windows_targets::link!("kernel32.dll" "system" fn VirtualProtectEx(hprocess : super::super::Foundation:: HANDLE, lpaddress : *const core::ffi::c_void, dwsize : usize, flnewprotect : PAGE_PROTECTION_FLAGS, lpfloldprotect : *mut PAGE_PROTECTION_FLAGS) -> super::super::Foundation:: BOOL);
    VirtualProtectEx(core::mem::transmute(hprocess), core::mem::transmute(lpaddress), core::mem::transmute(dwsize), core::mem::transmute(flnewprotect), core::mem::transmute(lpfloldprotect)).ok()
}
#[inline]
pub unsafe fn VirtualProtectFromApp(address: *const core::ffi::c_void, size: usize, newprotection: u32, oldprotection: *mut u32) -> windows_core::Result<()> {
    windows_targets::link!("api-ms-win-core-memory-l1-1-3.dll" "system" fn VirtualProtectFromApp(address : *const core::ffi::c_void, size : usize, newprotection : u32, oldprotection : *mut u32) -> super::super::Foundation:: BOOL);
    VirtualProtectFromApp(core::mem::transmute(address), core::mem::transmute(size), core::mem::transmute(newprotection), core::mem::transmute(oldprotection)).ok()
}
#[inline]
pub unsafe fn VirtualQuery(lpaddress: Option<*const core::ffi::c_void>, lpbuffer: *mut MEMORY_BASIC_INFORMATION, dwlength: usize) -> usize {
    windows_targets::link!("kernel32.dll" "system" fn VirtualQuery(lpaddress : *const core::ffi::c_void, lpbuffer : *mut MEMORY_BASIC_INFORMATION, dwlength : usize) -> usize);
    VirtualQuery(core::mem::transmute(lpaddress.unwrap_or(core::mem::zeroed())), core::mem::transmute(lpbuffer), core::mem::transmute(dwlength))
}
#[inline]
pub unsafe fn VirtualQueryEx(hprocess: super::super::Foundation::HANDLE, lpaddress: Option<*const core::ffi::c_void>, lpbuffer: *mut MEMORY_BASIC_INFORMATION, dwlength: usize) -> usize {
    windows_targets::link!("kernel32.dll" "system" fn VirtualQueryEx(hprocess : super::super::Foundation:: HANDLE, lpaddress : *const core::ffi::c_void, lpbuffer : *mut MEMORY_BASIC_INFORMATION, dwlength : usize) -> usize);
    VirtualQueryEx(core::mem::transmute(hprocess), core::mem::transmute(lpaddress.unwrap_or(core::mem::zeroed())), core::mem::transmute(lpbuffer), core::mem::transmute(dwlength))
}
#[inline]
pub unsafe fn VirtualUnlock(lpaddress: *const core::ffi::c_void, dwsize: usize) -> windows_core::Result<()> {
    windows_targets::link!("kernel32.dll" "system" fn VirtualUnlock(lpaddress : *const core::ffi::c_void, dwsize : usize) -> super::super::Foundation:: BOOL);
    VirtualUnlock(core::mem::transmute(lpaddress), core::mem::transmute(dwsize)).ok()
}
#[inline]
pub unsafe fn VirtualUnlockEx(process: Option<super::super::Foundation::HANDLE>, address: *const core::ffi::c_void, size: usize) -> super::super::Foundation::BOOL {
    windows_targets::link!("api-ms-win-core-memory-l1-1-5.dll" "system" fn VirtualUnlockEx(process : super::super::Foundation:: HANDLE, address : *const core::ffi::c_void, size : usize) -> super::super::Foundation:: BOOL);
    VirtualUnlockEx(core::mem::transmute(process.unwrap_or(core::mem::zeroed())), core::mem::transmute(address), core::mem::transmute(size))
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub struct AtlThunkData_t(pub isize);
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct CFG_CALL_TARGET_INFO {
    pub Offset: usize,
    pub Flags: usize,
}
impl Default for CFG_CALL_TARGET_INFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const FILE_CACHE_MAX_HARD_DISABLE: u32 = 2u32;
pub const FILE_CACHE_MAX_HARD_ENABLE: u32 = 1u32;
pub const FILE_CACHE_MIN_HARD_DISABLE: u32 = 8u32;
pub const FILE_CACHE_MIN_HARD_ENABLE: u32 = 4u32;
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct FILE_MAP(pub u32);
impl FILE_MAP {
    pub const fn contains(&self, other: Self) -> bool {
        self.0 & other.0 == other.0
    }
}
impl core::ops::BitOr for FILE_MAP {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl core::ops::BitAnd for FILE_MAP {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl core::ops::BitOrAssign for FILE_MAP {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl core::ops::BitAndAssign for FILE_MAP {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl core::ops::Not for FILE_MAP {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
pub const FILE_MAP_ALL_ACCESS: FILE_MAP = FILE_MAP(983071u32);
pub const FILE_MAP_COPY: FILE_MAP = FILE_MAP(1u32);
pub const FILE_MAP_EXECUTE: FILE_MAP = FILE_MAP(32u32);
pub const FILE_MAP_LARGE_PAGES: FILE_MAP = FILE_MAP(536870912u32);
pub const FILE_MAP_READ: FILE_MAP = FILE_MAP(4u32);
pub const FILE_MAP_RESERVE: FILE_MAP = FILE_MAP(2147483648u32);
pub const FILE_MAP_TARGETS_INVALID: FILE_MAP = FILE_MAP(1073741824u32);
pub const FILE_MAP_WRITE: FILE_MAP = FILE_MAP(2u32);
pub const GHND: GLOBAL_ALLOC_FLAGS = GLOBAL_ALLOC_FLAGS(66u32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct GLOBAL_ALLOC_FLAGS(pub u32);
impl GLOBAL_ALLOC_FLAGS {
    pub const fn contains(&self, other: Self) -> bool {
        self.0 & other.0 == other.0
    }
}
impl core::ops::BitOr for GLOBAL_ALLOC_FLAGS {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl core::ops::BitAnd for GLOBAL_ALLOC_FLAGS {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl core::ops::BitOrAssign for GLOBAL_ALLOC_FLAGS {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl core::ops::BitAndAssign for GLOBAL_ALLOC_FLAGS {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl core::ops::Not for GLOBAL_ALLOC_FLAGS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
pub const GMEM_FIXED: GLOBAL_ALLOC_FLAGS = GLOBAL_ALLOC_FLAGS(0u32);
pub const GMEM_MOVEABLE: GLOBAL_ALLOC_FLAGS = GLOBAL_ALLOC_FLAGS(2u32);
pub const GMEM_ZEROINIT: GLOBAL_ALLOC_FLAGS = GLOBAL_ALLOC_FLAGS(64u32);
pub const GPTR: GLOBAL_ALLOC_FLAGS = GLOBAL_ALLOC_FLAGS(64u32);
pub const HEAP_CREATE_ALIGN_16: HEAP_FLAGS = HEAP_FLAGS(65536u32);
pub const HEAP_CREATE_ENABLE_EXECUTE: HEAP_FLAGS = HEAP_FLAGS(262144u32);
pub const HEAP_CREATE_ENABLE_TRACING: HEAP_FLAGS = HEAP_FLAGS(131072u32);
pub const HEAP_CREATE_HARDENED: HEAP_FLAGS = HEAP_FLAGS(512u32);
pub const HEAP_CREATE_SEGMENT_HEAP: HEAP_FLAGS = HEAP_FLAGS(256u32);
pub const HEAP_DISABLE_COALESCE_ON_FREE: HEAP_FLAGS = HEAP_FLAGS(128u32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct HEAP_FLAGS(pub u32);
impl HEAP_FLAGS {
    pub const fn contains(&self, other: Self) -> bool {
        self.0 & other.0 == other.0
    }
}
impl core::ops::BitOr for HEAP_FLAGS {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl core::ops::BitAnd for HEAP_FLAGS {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl core::ops::BitOrAssign for HEAP_FLAGS {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl core::ops::BitAndAssign for HEAP_FLAGS {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl core::ops::Not for HEAP_FLAGS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
pub const HEAP_FREE_CHECKING_ENABLED: HEAP_FLAGS = HEAP_FLAGS(64u32);
pub const HEAP_GENERATE_EXCEPTIONS: HEAP_FLAGS = HEAP_FLAGS(4u32);
pub const HEAP_GROWABLE: HEAP_FLAGS = HEAP_FLAGS(2u32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct HEAP_INFORMATION_CLASS(pub i32);
pub const HEAP_MAXIMUM_TAG: HEAP_FLAGS = HEAP_FLAGS(4095u32);
pub const HEAP_NONE: HEAP_FLAGS = HEAP_FLAGS(0u32);
pub const HEAP_NO_SERIALIZE: HEAP_FLAGS = HEAP_FLAGS(1u32);
pub const HEAP_PSEUDO_TAG_FLAG: HEAP_FLAGS = HEAP_FLAGS(32768u32);
pub const HEAP_REALLOC_IN_PLACE_ONLY: HEAP_FLAGS = HEAP_FLAGS(16u32);
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct HEAP_SUMMARY {
    pub cb: u32,
    pub cbAllocated: usize,
    pub cbCommitted: usize,
    pub cbReserved: usize,
    pub cbMaxReserve: usize,
}
impl Default for HEAP_SUMMARY {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const HEAP_TAG_SHIFT: HEAP_FLAGS = HEAP_FLAGS(18u32);
pub const HEAP_TAIL_CHECKING_ENABLED: HEAP_FLAGS = HEAP_FLAGS(32u32);
pub const HEAP_ZERO_MEMORY: HEAP_FLAGS = HEAP_FLAGS(8u32);
pub const HeapCompatibilityInformation: HEAP_INFORMATION_CLASS = HEAP_INFORMATION_CLASS(0i32);
pub const HeapEnableTerminationOnCorruption: HEAP_INFORMATION_CLASS = HEAP_INFORMATION_CLASS(1i32);
pub const HeapOptimizeResources: HEAP_INFORMATION_CLASS = HEAP_INFORMATION_CLASS(3i32);
pub const HeapTag: HEAP_INFORMATION_CLASS = HEAP_INFORMATION_CLASS(7i32);
pub const HighMemoryResourceNotification: MEMORY_RESOURCE_NOTIFICATION_TYPE = MEMORY_RESOURCE_NOTIFICATION_TYPE(1i32);
pub const LHND: LOCAL_ALLOC_FLAGS = LOCAL_ALLOC_FLAGS(66u32);
pub const LMEM_FIXED: LOCAL_ALLOC_FLAGS = LOCAL_ALLOC_FLAGS(0u32);
pub const LMEM_MOVEABLE: LOCAL_ALLOC_FLAGS = LOCAL_ALLOC_FLAGS(2u32);
pub const LMEM_ZEROINIT: LOCAL_ALLOC_FLAGS = LOCAL_ALLOC_FLAGS(64u32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct LOCAL_ALLOC_FLAGS(pub u32);
impl LOCAL_ALLOC_FLAGS {
    pub const fn contains(&self, other: Self) -> bool {
        self.0 & other.0 == other.0
    }
}
impl core::ops::BitOr for LOCAL_ALLOC_FLAGS {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl core::ops::BitAnd for LOCAL_ALLOC_FLAGS {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl core::ops::BitOrAssign for LOCAL_ALLOC_FLAGS {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl core::ops::BitAndAssign for LOCAL_ALLOC_FLAGS {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl core::ops::Not for LOCAL_ALLOC_FLAGS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
pub const LPTR: LOCAL_ALLOC_FLAGS = LOCAL_ALLOC_FLAGS(64u32);
pub const LowMemoryResourceNotification: MEMORY_RESOURCE_NOTIFICATION_TYPE = MEMORY_RESOURCE_NOTIFICATION_TYPE(0i32);
pub const MEHC_PATROL_SCRUBBER_PRESENT: u32 = 1u32;
#[repr(C)]
#[cfg(target_arch = "x86")]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct MEMORY_BASIC_INFORMATION {
    pub BaseAddress: *mut core::ffi::c_void,
    pub AllocationBase: *mut core::ffi::c_void,
    pub AllocationProtect: PAGE_PROTECTION_FLAGS,
    pub RegionSize: usize,
    pub State: VIRTUAL_ALLOCATION_TYPE,
    pub Protect: PAGE_PROTECTION_FLAGS,
    pub Type: PAGE_TYPE,
}
#[cfg(target_arch = "x86")]
impl Default for MEMORY_BASIC_INFORMATION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct MEMORY_BASIC_INFORMATION {
    pub BaseAddress: *mut core::ffi::c_void,
    pub AllocationBase: *mut core::ffi::c_void,
    pub AllocationProtect: PAGE_PROTECTION_FLAGS,
    pub PartitionId: u16,
    pub RegionSize: usize,
    pub State: VIRTUAL_ALLOCATION_TYPE,
    pub Protect: PAGE_PROTECTION_FLAGS,
    pub Type: PAGE_TYPE,
}
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
impl Default for MEMORY_BASIC_INFORMATION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct MEMORY_BASIC_INFORMATION32 {
    pub BaseAddress: u32,
    pub AllocationBase: u32,
    pub AllocationProtect: PAGE_PROTECTION_FLAGS,
    pub RegionSize: u32,
    pub State: VIRTUAL_ALLOCATION_TYPE,
    pub Protect: PAGE_PROTECTION_FLAGS,
    pub Type: PAGE_TYPE,
}
impl Default for MEMORY_BASIC_INFORMATION32 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct MEMORY_BASIC_INFORMATION64 {
    pub BaseAddress: u64,
    pub AllocationBase: u64,
    pub AllocationProtect: PAGE_PROTECTION_FLAGS,
    pub __alignment1: u32,
    pub RegionSize: u64,
    pub State: VIRTUAL_ALLOCATION_TYPE,
    pub Protect: PAGE_PROTECTION_FLAGS,
    pub Type: PAGE_TYPE,
    pub __alignment2: u32,
}
impl Default for MEMORY_BASIC_INFORMATION64 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct MEMORY_MAPPED_VIEW_ADDRESS {
    pub Value: *mut core::ffi::c_void,
}
impl Default for MEMORY_MAPPED_VIEW_ADDRESS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct MEMORY_PARTITION_DEDICATED_MEMORY_ATTRIBUTE {
    pub Type: MEM_DEDICATED_ATTRIBUTE_TYPE,
    pub Reserved: u32,
    pub Value: u64,
}
impl Default for MEMORY_PARTITION_DEDICATED_MEMORY_ATTRIBUTE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct MEMORY_PARTITION_DEDICATED_MEMORY_INFORMATION {
    pub NextEntryOffset: u32,
    pub SizeOfInformation: u32,
    pub Flags: u32,
    pub AttributesOffset: u32,
    pub AttributeCount: u32,
    pub Reserved: u32,
    pub TypeId: u64,
}
impl Default for MEMORY_PARTITION_DEDICATED_MEMORY_INFORMATION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct MEMORY_RESOURCE_NOTIFICATION_TYPE(pub i32);
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct MEM_ADDRESS_REQUIREMENTS {
    pub LowestStartingAddress: *mut core::ffi::c_void,
    pub HighestEndingAddress: *mut core::ffi::c_void,
    pub Alignment: usize,
}
impl Default for MEM_ADDRESS_REQUIREMENTS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const MEM_COMMIT: VIRTUAL_ALLOCATION_TYPE = VIRTUAL_ALLOCATION_TYPE(4096u32);
pub const MEM_DECOMMIT: VIRTUAL_FREE_TYPE = VIRTUAL_FREE_TYPE(16384u32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct MEM_DEDICATED_ATTRIBUTE_TYPE(pub i32);
#[repr(C)]
#[derive(Clone, Copy)]
pub struct MEM_EXTENDED_PARAMETER {
    pub Anonymous1: MEM_EXTENDED_PARAMETER_0,
    pub Anonymous2: MEM_EXTENDED_PARAMETER_1,
}
impl Default for MEM_EXTENDED_PARAMETER {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct MEM_EXTENDED_PARAMETER_0 {
    pub _bitfield: u64,
}
impl Default for MEM_EXTENDED_PARAMETER_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union MEM_EXTENDED_PARAMETER_1 {
    pub ULong64: u64,
    pub Pointer: *mut core::ffi::c_void,
    pub Size: usize,
    pub Handle: super::super::Foundation::HANDLE,
    pub ULong: u32,
}
impl Default for MEM_EXTENDED_PARAMETER_1 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct MEM_EXTENDED_PARAMETER_TYPE(pub i32);
pub const MEM_FREE: VIRTUAL_ALLOCATION_TYPE = VIRTUAL_ALLOCATION_TYPE(65536u32);
pub const MEM_IMAGE: PAGE_TYPE = PAGE_TYPE(16777216u32);
pub const MEM_LARGE_PAGES: VIRTUAL_ALLOCATION_TYPE = VIRTUAL_ALLOCATION_TYPE(536870912u32);
pub const MEM_MAPPED: PAGE_TYPE = PAGE_TYPE(262144u32);
pub const MEM_PRESERVE_PLACEHOLDER: UNMAP_VIEW_OF_FILE_FLAGS = UNMAP_VIEW_OF_FILE_FLAGS(2u32);
pub const MEM_PRIVATE: PAGE_TYPE = PAGE_TYPE(131072u32);
pub const MEM_RELEASE: VIRTUAL_FREE_TYPE = VIRTUAL_FREE_TYPE(32768u32);
pub const MEM_REPLACE_PLACEHOLDER: VIRTUAL_ALLOCATION_TYPE = VIRTUAL_ALLOCATION_TYPE(16384u32);
pub const MEM_RESERVE: VIRTUAL_ALLOCATION_TYPE = VIRTUAL_ALLOCATION_TYPE(8192u32);
pub const MEM_RESERVE_PLACEHOLDER: VIRTUAL_ALLOCATION_TYPE = VIRTUAL_ALLOCATION_TYPE(262144u32);
pub const MEM_RESET: VIRTUAL_ALLOCATION_TYPE = VIRTUAL_ALLOCATION_TYPE(524288u32);
pub const MEM_RESET_UNDO: VIRTUAL_ALLOCATION_TYPE = VIRTUAL_ALLOCATION_TYPE(16777216u32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct MEM_SECTION_EXTENDED_PARAMETER_TYPE(pub i32);
pub const MEM_UNMAP_NONE: UNMAP_VIEW_OF_FILE_FLAGS = UNMAP_VIEW_OF_FILE_FLAGS(0u32);
pub const MEM_UNMAP_WITH_TRANSIENT_BOOST: UNMAP_VIEW_OF_FILE_FLAGS = UNMAP_VIEW_OF_FILE_FLAGS(1u32);
pub const MemDedicatedAttributeMax: MEM_DEDICATED_ATTRIBUTE_TYPE = MEM_DEDICATED_ATTRIBUTE_TYPE(4i32);
pub const MemDedicatedAttributeReadBandwidth: MEM_DEDICATED_ATTRIBUTE_TYPE = MEM_DEDICATED_ATTRIBUTE_TYPE(0i32);
pub const MemDedicatedAttributeReadLatency: MEM_DEDICATED_ATTRIBUTE_TYPE = MEM_DEDICATED_ATTRIBUTE_TYPE(1i32);
pub const MemDedicatedAttributeWriteBandwidth: MEM_DEDICATED_ATTRIBUTE_TYPE = MEM_DEDICATED_ATTRIBUTE_TYPE(2i32);
pub const MemDedicatedAttributeWriteLatency: MEM_DEDICATED_ATTRIBUTE_TYPE = MEM_DEDICATED_ATTRIBUTE_TYPE(3i32);
pub const MemExtendedParameterAddressRequirements: MEM_EXTENDED_PARAMETER_TYPE = MEM_EXTENDED_PARAMETER_TYPE(1i32);
pub const MemExtendedParameterAttributeFlags: MEM_EXTENDED_PARAMETER_TYPE = MEM_EXTENDED_PARAMETER_TYPE(5i32);
pub const MemExtendedParameterImageMachine: MEM_EXTENDED_PARAMETER_TYPE = MEM_EXTENDED_PARAMETER_TYPE(6i32);
pub const MemExtendedParameterInvalidType: MEM_EXTENDED_PARAMETER_TYPE = MEM_EXTENDED_PARAMETER_TYPE(0i32);
pub const MemExtendedParameterMax: MEM_EXTENDED_PARAMETER_TYPE = MEM_EXTENDED_PARAMETER_TYPE(7i32);
pub const MemExtendedParameterNumaNode: MEM_EXTENDED_PARAMETER_TYPE = MEM_EXTENDED_PARAMETER_TYPE(2i32);
pub const MemExtendedParameterPartitionHandle: MEM_EXTENDED_PARAMETER_TYPE = MEM_EXTENDED_PARAMETER_TYPE(3i32);
pub const MemExtendedParameterUserPhysicalHandle: MEM_EXTENDED_PARAMETER_TYPE = MEM_EXTENDED_PARAMETER_TYPE(4i32);
pub const MemSectionExtendedParameterInvalidType: MEM_SECTION_EXTENDED_PARAMETER_TYPE = MEM_SECTION_EXTENDED_PARAMETER_TYPE(0i32);
pub const MemSectionExtendedParameterMax: MEM_SECTION_EXTENDED_PARAMETER_TYPE = MEM_SECTION_EXTENDED_PARAMETER_TYPE(4i32);
pub const MemSectionExtendedParameterNumaNode: MEM_SECTION_EXTENDED_PARAMETER_TYPE = MEM_SECTION_EXTENDED_PARAMETER_TYPE(2i32);
pub const MemSectionExtendedParameterSigningLevel: MEM_SECTION_EXTENDED_PARAMETER_TYPE = MEM_SECTION_EXTENDED_PARAMETER_TYPE(3i32);
pub const MemSectionExtendedParameterUserPhysicalFlags: MEM_SECTION_EXTENDED_PARAMETER_TYPE = MEM_SECTION_EXTENDED_PARAMETER_TYPE(1i32);
pub const MemoryPartitionDedicatedMemoryInfo: WIN32_MEMORY_PARTITION_INFORMATION_CLASS = WIN32_MEMORY_PARTITION_INFORMATION_CLASS(1i32);
pub const MemoryPartitionInfo: WIN32_MEMORY_PARTITION_INFORMATION_CLASS = WIN32_MEMORY_PARTITION_INFORMATION_CLASS(0i32);
pub const MemoryRegionInfo: WIN32_MEMORY_INFORMATION_CLASS = WIN32_MEMORY_INFORMATION_CLASS(0i32);
pub const NONZEROLHND: LOCAL_ALLOC_FLAGS = LOCAL_ALLOC_FLAGS(2u32);
pub const NONZEROLPTR: LOCAL_ALLOC_FLAGS = LOCAL_ALLOC_FLAGS(0u32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct OFFER_PRIORITY(pub i32);
pub const PAGE_ENCLAVE_DECOMMIT: PAGE_PROTECTION_FLAGS = PAGE_PROTECTION_FLAGS(268435456u32);
pub const PAGE_ENCLAVE_MASK: PAGE_PROTECTION_FLAGS = PAGE_PROTECTION_FLAGS(268435456u32);
pub const PAGE_ENCLAVE_SS_FIRST: PAGE_PROTECTION_FLAGS = PAGE_PROTECTION_FLAGS(268435457u32);
pub const PAGE_ENCLAVE_SS_REST: PAGE_PROTECTION_FLAGS = PAGE_PROTECTION_FLAGS(268435458u32);
pub const PAGE_ENCLAVE_THREAD_CONTROL: PAGE_PROTECTION_FLAGS = PAGE_PROTECTION_FLAGS(2147483648u32);
pub const PAGE_ENCLAVE_UNVALIDATED: PAGE_PROTECTION_FLAGS = PAGE_PROTECTION_FLAGS(536870912u32);
pub const PAGE_EXECUTE: PAGE_PROTECTION_FLAGS = PAGE_PROTECTION_FLAGS(16u32);
pub const PAGE_EXECUTE_READ: PAGE_PROTECTION_FLAGS = PAGE_PROTECTION_FLAGS(32u32);
pub const PAGE_EXECUTE_READWRITE: PAGE_PROTECTION_FLAGS = PAGE_PROTECTION_FLAGS(64u32);
pub const PAGE_EXECUTE_WRITECOPY: PAGE_PROTECTION_FLAGS = PAGE_PROTECTION_FLAGS(128u32);
pub const PAGE_GRAPHICS_COHERENT: PAGE_PROTECTION_FLAGS = PAGE_PROTECTION_FLAGS(131072u32);
pub const PAGE_GRAPHICS_EXECUTE: PAGE_PROTECTION_FLAGS = PAGE_PROTECTION_FLAGS(16384u32);
pub const PAGE_GRAPHICS_EXECUTE_READ: PAGE_PROTECTION_FLAGS = PAGE_PROTECTION_FLAGS(32768u32);
pub const PAGE_GRAPHICS_EXECUTE_READWRITE: PAGE_PROTECTION_FLAGS = PAGE_PROTECTION_FLAGS(65536u32);
pub const PAGE_GRAPHICS_NOACCESS: PAGE_PROTECTION_FLAGS = PAGE_PROTECTION_FLAGS(2048u32);
pub const PAGE_GRAPHICS_NOCACHE: PAGE_PROTECTION_FLAGS = PAGE_PROTECTION_FLAGS(262144u32);
pub const PAGE_GRAPHICS_READONLY: PAGE_PROTECTION_FLAGS = PAGE_PROTECTION_FLAGS(4096u32);
pub const PAGE_GRAPHICS_READWRITE: PAGE_PROTECTION_FLAGS = PAGE_PROTECTION_FLAGS(8192u32);
pub const PAGE_GUARD: PAGE_PROTECTION_FLAGS = PAGE_PROTECTION_FLAGS(256u32);
pub const PAGE_NOACCESS: PAGE_PROTECTION_FLAGS = PAGE_PROTECTION_FLAGS(1u32);
pub const PAGE_NOCACHE: PAGE_PROTECTION_FLAGS = PAGE_PROTECTION_FLAGS(512u32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct PAGE_PROTECTION_FLAGS(pub u32);
impl PAGE_PROTECTION_FLAGS {
    pub const fn contains(&self, other: Self) -> bool {
        self.0 & other.0 == other.0
    }
}
impl core::ops::BitOr for PAGE_PROTECTION_FLAGS {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl core::ops::BitAnd for PAGE_PROTECTION_FLAGS {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl core::ops::BitOrAssign for PAGE_PROTECTION_FLAGS {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl core::ops::BitAndAssign for PAGE_PROTECTION_FLAGS {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl core::ops::Not for PAGE_PROTECTION_FLAGS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
pub const PAGE_READONLY: PAGE_PROTECTION_FLAGS = PAGE_PROTECTION_FLAGS(2u32);
pub const PAGE_READWRITE: PAGE_PROTECTION_FLAGS = PAGE_PROTECTION_FLAGS(4u32);
pub const PAGE_REVERT_TO_FILE_MAP: PAGE_PROTECTION_FLAGS = PAGE_PROTECTION_FLAGS(2147483648u32);
pub const PAGE_TARGETS_INVALID: PAGE_PROTECTION_FLAGS = PAGE_PROTECTION_FLAGS(1073741824u32);
pub const PAGE_TARGETS_NO_UPDATE: PAGE_PROTECTION_FLAGS = PAGE_PROTECTION_FLAGS(1073741824u32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct PAGE_TYPE(pub u32);
impl PAGE_TYPE {
    pub const fn contains(&self, other: Self) -> bool {
        self.0 & other.0 == other.0
    }
}
impl core::ops::BitOr for PAGE_TYPE {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl core::ops::BitAnd for PAGE_TYPE {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl core::ops::BitOrAssign for PAGE_TYPE {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl core::ops::BitAndAssign for PAGE_TYPE {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl core::ops::Not for PAGE_TYPE {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
pub const PAGE_WRITECOMBINE: PAGE_PROTECTION_FLAGS = PAGE_PROTECTION_FLAGS(1024u32);
pub const PAGE_WRITECOPY: PAGE_PROTECTION_FLAGS = PAGE_PROTECTION_FLAGS(8u32);
pub type PBAD_MEMORY_CALLBACK_ROUTINE = Option<unsafe extern "system" fn()>;
#[repr(C)]
#[derive(Clone, Copy)]
pub struct PROCESS_HEAP_ENTRY {
    pub lpData: *mut core::ffi::c_void,
    pub cbData: u32,
    pub cbOverhead: u8,
    pub iRegionIndex: u8,
    pub wFlags: u16,
    pub Anonymous: PROCESS_HEAP_ENTRY_0,
}
impl Default for PROCESS_HEAP_ENTRY {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union PROCESS_HEAP_ENTRY_0 {
    pub Block: PROCESS_HEAP_ENTRY_0_0,
    pub Region: PROCESS_HEAP_ENTRY_0_1,
}
impl Default for PROCESS_HEAP_ENTRY_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct PROCESS_HEAP_ENTRY_0_0 {
    pub hMem: super::super::Foundation::HANDLE,
    pub dwReserved: [u32; 3],
}
impl Default for PROCESS_HEAP_ENTRY_0_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct PROCESS_HEAP_ENTRY_0_1 {
    pub dwCommittedSize: u32,
    pub dwUnCommittedSize: u32,
    pub lpFirstBlock: *mut core::ffi::c_void,
    pub lpLastBlock: *mut core::ffi::c_void,
}
impl Default for PROCESS_HEAP_ENTRY_0_1 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub type PSECURE_MEMORY_CACHE_CALLBACK = Option<unsafe extern "system" fn(addr: *const core::ffi::c_void, range: usize) -> super::super::Foundation::BOOLEAN>;
pub const QUOTA_LIMITS_HARDWS_MAX_DISABLE: SETPROCESSWORKINGSETSIZEEX_FLAGS = SETPROCESSWORKINGSETSIZEEX_FLAGS(8u32);
pub const QUOTA_LIMITS_HARDWS_MAX_ENABLE: SETPROCESSWORKINGSETSIZEEX_FLAGS = SETPROCESSWORKINGSETSIZEEX_FLAGS(4u32);
pub const QUOTA_LIMITS_HARDWS_MIN_DISABLE: SETPROCESSWORKINGSETSIZEEX_FLAGS = SETPROCESSWORKINGSETSIZEEX_FLAGS(2u32);
pub const QUOTA_LIMITS_HARDWS_MIN_ENABLE: SETPROCESSWORKINGSETSIZEEX_FLAGS = SETPROCESSWORKINGSETSIZEEX_FLAGS(1u32);
pub const SECTION_ALL_ACCESS: SECTION_FLAGS = SECTION_FLAGS(983071u32);
pub const SECTION_EXTEND_SIZE: SECTION_FLAGS = SECTION_FLAGS(16u32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct SECTION_FLAGS(pub u32);
impl SECTION_FLAGS {
    pub const fn contains(&self, other: Self) -> bool {
        self.0 & other.0 == other.0
    }
}
impl core::ops::BitOr for SECTION_FLAGS {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl core::ops::BitAnd for SECTION_FLAGS {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl core::ops::BitOrAssign for SECTION_FLAGS {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl core::ops::BitAndAssign for SECTION_FLAGS {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl core::ops::Not for SECTION_FLAGS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
pub const SECTION_MAP_EXECUTE: SECTION_FLAGS = SECTION_FLAGS(8u32);
pub const SECTION_MAP_EXECUTE_EXPLICIT: SECTION_FLAGS = SECTION_FLAGS(32u32);
pub const SECTION_MAP_READ: SECTION_FLAGS = SECTION_FLAGS(4u32);
pub const SECTION_MAP_WRITE: SECTION_FLAGS = SECTION_FLAGS(2u32);
pub const SECTION_QUERY: SECTION_FLAGS = SECTION_FLAGS(1u32);
pub const SEC_64K_PAGES: PAGE_PROTECTION_FLAGS = PAGE_PROTECTION_FLAGS(524288u32);
pub const SEC_COMMIT: PAGE_PROTECTION_FLAGS = PAGE_PROTECTION_FLAGS(134217728u32);
pub const SEC_FILE: PAGE_PROTECTION_FLAGS = PAGE_PROTECTION_FLAGS(8388608u32);
pub const SEC_IMAGE: PAGE_PROTECTION_FLAGS = PAGE_PROTECTION_FLAGS(16777216u32);
pub const SEC_IMAGE_NO_EXECUTE: PAGE_PROTECTION_FLAGS = PAGE_PROTECTION_FLAGS(285212672u32);
pub const SEC_LARGE_PAGES: PAGE_PROTECTION_FLAGS = PAGE_PROTECTION_FLAGS(2147483648u32);
pub const SEC_NOCACHE: PAGE_PROTECTION_FLAGS = PAGE_PROTECTION_FLAGS(268435456u32);
pub const SEC_PARTITION_OWNER_HANDLE: PAGE_PROTECTION_FLAGS = PAGE_PROTECTION_FLAGS(262144u32);
pub const SEC_PROTECTED_IMAGE: PAGE_PROTECTION_FLAGS = PAGE_PROTECTION_FLAGS(33554432u32);
pub const SEC_RESERVE: PAGE_PROTECTION_FLAGS = PAGE_PROTECTION_FLAGS(67108864u32);
pub const SEC_WRITECOMBINE: PAGE_PROTECTION_FLAGS = PAGE_PROTECTION_FLAGS(1073741824u32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct SETPROCESSWORKINGSETSIZEEX_FLAGS(pub u32);
impl SETPROCESSWORKINGSETSIZEEX_FLAGS {
    pub const fn contains(&self, other: Self) -> bool {
        self.0 & other.0 == other.0
    }
}
impl core::ops::BitOr for SETPROCESSWORKINGSETSIZEEX_FLAGS {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl core::ops::BitAnd for SETPROCESSWORKINGSETSIZEEX_FLAGS {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl core::ops::BitOrAssign for SETPROCESSWORKINGSETSIZEEX_FLAGS {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl core::ops::BitAndAssign for SETPROCESSWORKINGSETSIZEEX_FLAGS {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl core::ops::Not for SETPROCESSWORKINGSETSIZEEX_FLAGS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct UNMAP_VIEW_OF_FILE_FLAGS(pub u32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct VIRTUAL_ALLOCATION_TYPE(pub u32);
impl VIRTUAL_ALLOCATION_TYPE {
    pub const fn contains(&self, other: Self) -> bool {
        self.0 & other.0 == other.0
    }
}
impl core::ops::BitOr for VIRTUAL_ALLOCATION_TYPE {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl core::ops::BitAnd for VIRTUAL_ALLOCATION_TYPE {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl core::ops::BitOrAssign for VIRTUAL_ALLOCATION_TYPE {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl core::ops::BitAndAssign for VIRTUAL_ALLOCATION_TYPE {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl core::ops::Not for VIRTUAL_ALLOCATION_TYPE {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct VIRTUAL_FREE_TYPE(pub u32);
pub const VmOfferPriorityBelowNormal: OFFER_PRIORITY = OFFER_PRIORITY(3i32);
pub const VmOfferPriorityLow: OFFER_PRIORITY = OFFER_PRIORITY(2i32);
pub const VmOfferPriorityNormal: OFFER_PRIORITY = OFFER_PRIORITY(4i32);
pub const VmOfferPriorityVeryLow: OFFER_PRIORITY = OFFER_PRIORITY(1i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct WIN32_MEMORY_INFORMATION_CLASS(pub i32);
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
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct WIN32_MEMORY_PARTITION_INFORMATION_CLASS(pub i32);
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
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct WIN32_MEMORY_REGION_INFORMATION_0_0 {
    pub _bitfield: u32,
}
impl Default for WIN32_MEMORY_REGION_INFORMATION_0_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

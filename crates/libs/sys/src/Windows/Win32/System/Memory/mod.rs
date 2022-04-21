#[cfg(feature = "Win32_System_Memory_NonVolatile")]
pub mod NonVolatile;
#[link(name = "windows")]
extern "system" {
    #[doc = "*Required features: `\"Win32_System_Memory\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn AddSecureMemoryCacheCallback(pfncallback: PSECURE_MEMORY_CACHE_CALLBACK) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_System_Memory\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn AllocateUserPhysicalPages(hprocess: super::super::Foundation::HANDLE, numberofpages: *mut usize, pagearray: *mut usize) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_System_Memory\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn AllocateUserPhysicalPages2(objecthandle: super::super::Foundation::HANDLE, numberofpages: *mut usize, pagearray: *mut usize, extendedparameters: *mut MEM_EXTENDED_PARAMETER, extendedparametercount: u32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_System_Memory\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn AllocateUserPhysicalPagesNuma(hprocess: super::super::Foundation::HANDLE, numberofpages: *mut usize, pagearray: *mut usize, nndpreferred: u32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_System_Memory\"`, `\"Win32_Foundation\"`, `\"Win32_Security\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
    pub fn CreateFileMapping2(file: super::super::Foundation::HANDLE, securityattributes: *const super::super::Security::SECURITY_ATTRIBUTES, desiredaccess: u32, pageprotection: PAGE_PROTECTION_FLAGS, allocationattributes: u32, maximumsize: u64, name: ::windows_sys::core::PCWSTR, extendedparameters: *mut MEM_EXTENDED_PARAMETER, parametercount: u32) -> super::super::Foundation::HANDLE;
    #[doc = "*Required features: `\"Win32_System_Memory\"`, `\"Win32_Foundation\"`, `\"Win32_Security\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
    pub fn CreateFileMappingA(hfile: super::super::Foundation::HANDLE, lpfilemappingattributes: *const super::super::Security::SECURITY_ATTRIBUTES, flprotect: PAGE_PROTECTION_FLAGS, dwmaximumsizehigh: u32, dwmaximumsizelow: u32, lpname: ::windows_sys::core::PCSTR) -> super::super::Foundation::HANDLE;
    #[doc = "*Required features: `\"Win32_System_Memory\"`, `\"Win32_Foundation\"`, `\"Win32_Security\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
    pub fn CreateFileMappingFromApp(hfile: super::super::Foundation::HANDLE, securityattributes: *const super::super::Security::SECURITY_ATTRIBUTES, pageprotection: PAGE_PROTECTION_FLAGS, maximumsize: u64, name: ::windows_sys::core::PCWSTR) -> super::super::Foundation::HANDLE;
    #[doc = "*Required features: `\"Win32_System_Memory\"`, `\"Win32_Foundation\"`, `\"Win32_Security\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
    pub fn CreateFileMappingNumaA(hfile: super::super::Foundation::HANDLE, lpfilemappingattributes: *const super::super::Security::SECURITY_ATTRIBUTES, flprotect: PAGE_PROTECTION_FLAGS, dwmaximumsizehigh: u32, dwmaximumsizelow: u32, lpname: ::windows_sys::core::PCSTR, nndpreferred: u32) -> super::super::Foundation::HANDLE;
    #[doc = "*Required features: `\"Win32_System_Memory\"`, `\"Win32_Foundation\"`, `\"Win32_Security\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
    pub fn CreateFileMappingNumaW(hfile: super::super::Foundation::HANDLE, lpfilemappingattributes: *const super::super::Security::SECURITY_ATTRIBUTES, flprotect: PAGE_PROTECTION_FLAGS, dwmaximumsizehigh: u32, dwmaximumsizelow: u32, lpname: ::windows_sys::core::PCWSTR, nndpreferred: u32) -> super::super::Foundation::HANDLE;
    #[doc = "*Required features: `\"Win32_System_Memory\"`, `\"Win32_Foundation\"`, `\"Win32_Security\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
    pub fn CreateFileMappingW(hfile: super::super::Foundation::HANDLE, lpfilemappingattributes: *const super::super::Security::SECURITY_ATTRIBUTES, flprotect: PAGE_PROTECTION_FLAGS, dwmaximumsizehigh: u32, dwmaximumsizelow: u32, lpname: ::windows_sys::core::PCWSTR) -> super::super::Foundation::HANDLE;
    #[doc = "*Required features: `\"Win32_System_Memory\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CreateMemoryResourceNotification(notificationtype: MEMORY_RESOURCE_NOTIFICATION_TYPE) -> super::super::Foundation::HANDLE;
    #[doc = "*Required features: `\"Win32_System_Memory\"`*"]
    pub fn DiscardVirtualMemory(virtualaddress: *mut ::core::ffi::c_void, size: usize) -> u32;
    #[doc = "*Required features: `\"Win32_System_Memory\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn FlushViewOfFile(lpbaseaddress: *const ::core::ffi::c_void, dwnumberofbytestoflush: usize) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_System_Memory\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn FreeUserPhysicalPages(hprocess: super::super::Foundation::HANDLE, numberofpages: *mut usize, pagearray: *const usize) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_System_Memory\"`*"]
    pub fn GetLargePageMinimum() -> usize;
    #[doc = "*Required features: `\"Win32_System_Memory\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetMemoryErrorHandlingCapabilities(capabilities: *mut u32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_System_Memory\"`*"]
    pub fn GetProcessHeap() -> HeapHandle;
    #[doc = "*Required features: `\"Win32_System_Memory\"`*"]
    pub fn GetProcessHeaps(numberofheaps: u32, processheaps: *mut HeapHandle) -> u32;
    #[doc = "*Required features: `\"Win32_System_Memory\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetProcessWorkingSetSizeEx(hprocess: super::super::Foundation::HANDLE, lpminimumworkingsetsize: *mut usize, lpmaximumworkingsetsize: *mut usize, flags: *mut u32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_System_Memory\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetSystemFileCacheSize(lpminimumfilecachesize: *mut usize, lpmaximumfilecachesize: *mut usize, lpflags: *mut u32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_System_Memory\"`*"]
    pub fn GetWriteWatch(dwflags: u32, lpbaseaddress: *const ::core::ffi::c_void, dwregionsize: usize, lpaddresses: *mut *mut ::core::ffi::c_void, lpdwcount: *mut usize, lpdwgranularity: *mut u32) -> u32;
    #[doc = "*Required features: `\"Win32_System_Memory\"`*"]
    pub fn GlobalAlloc(uflags: GLOBAL_ALLOC_FLAGS, dwbytes: usize) -> isize;
    #[doc = "*Required features: `\"Win32_System_Memory\"`*"]
    pub fn GlobalFlags(hmem: isize) -> u32;
    #[doc = "*Required features: `\"Win32_System_Memory\"`*"]
    pub fn GlobalFree(hmem: isize) -> isize;
    #[doc = "*Required features: `\"Win32_System_Memory\"`*"]
    pub fn GlobalHandle(pmem: *const ::core::ffi::c_void) -> isize;
    #[doc = "*Required features: `\"Win32_System_Memory\"`*"]
    pub fn GlobalLock(hmem: isize) -> *mut ::core::ffi::c_void;
    #[doc = "*Required features: `\"Win32_System_Memory\"`*"]
    pub fn GlobalReAlloc(hmem: isize, dwbytes: usize, uflags: u32) -> isize;
    #[doc = "*Required features: `\"Win32_System_Memory\"`*"]
    pub fn GlobalSize(hmem: isize) -> usize;
    #[doc = "*Required features: `\"Win32_System_Memory\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GlobalUnlock(hmem: isize) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_System_Memory\"`*"]
    pub fn HeapAlloc(hheap: HeapHandle, dwflags: HEAP_FLAGS, dwbytes: usize) -> *mut ::core::ffi::c_void;
    #[doc = "*Required features: `\"Win32_System_Memory\"`*"]
    pub fn HeapCompact(hheap: HeapHandle, dwflags: HEAP_FLAGS) -> usize;
    #[doc = "*Required features: `\"Win32_System_Memory\"`*"]
    pub fn HeapCreate(floptions: HEAP_FLAGS, dwinitialsize: usize, dwmaximumsize: usize) -> HeapHandle;
    #[doc = "*Required features: `\"Win32_System_Memory\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn HeapDestroy(hheap: HeapHandle) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_System_Memory\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn HeapFree(hheap: HeapHandle, dwflags: HEAP_FLAGS, lpmem: *const ::core::ffi::c_void) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_System_Memory\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn HeapLock(hheap: HeapHandle) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_System_Memory\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn HeapQueryInformation(heaphandle: HeapHandle, heapinformationclass: HEAP_INFORMATION_CLASS, heapinformation: *mut ::core::ffi::c_void, heapinformationlength: usize, returnlength: *mut usize) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_System_Memory\"`*"]
    pub fn HeapReAlloc(hheap: HeapHandle, dwflags: HEAP_FLAGS, lpmem: *const ::core::ffi::c_void, dwbytes: usize) -> *mut ::core::ffi::c_void;
    #[doc = "*Required features: `\"Win32_System_Memory\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn HeapSetInformation(heaphandle: HeapHandle, heapinformationclass: HEAP_INFORMATION_CLASS, heapinformation: *const ::core::ffi::c_void, heapinformationlength: usize) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_System_Memory\"`*"]
    pub fn HeapSize(hheap: HeapHandle, dwflags: HEAP_FLAGS, lpmem: *const ::core::ffi::c_void) -> usize;
    #[doc = "*Required features: `\"Win32_System_Memory\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn HeapSummary(hheap: super::super::Foundation::HANDLE, dwflags: u32, lpsummary: *mut HEAP_SUMMARY) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_System_Memory\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn HeapUnlock(hheap: HeapHandle) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_System_Memory\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn HeapValidate(hheap: HeapHandle, dwflags: HEAP_FLAGS, lpmem: *const ::core::ffi::c_void) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_System_Memory\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn HeapWalk(hheap: HeapHandle, lpentry: *mut PROCESS_HEAP_ENTRY) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_System_Memory\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn IsBadCodePtr(lpfn: super::super::Foundation::FARPROC) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_System_Memory\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn IsBadReadPtr(lp: *const ::core::ffi::c_void, ucb: usize) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_System_Memory\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn IsBadStringPtrA(lpsz: ::windows_sys::core::PCSTR, ucchmax: usize) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_System_Memory\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn IsBadStringPtrW(lpsz: ::windows_sys::core::PCWSTR, ucchmax: usize) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_System_Memory\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn IsBadWritePtr(lp: *const ::core::ffi::c_void, ucb: usize) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_System_Memory\"`*"]
    pub fn LocalAlloc(uflags: LOCAL_ALLOC_FLAGS, ubytes: usize) -> isize;
    #[doc = "*Required features: `\"Win32_System_Memory\"`*"]
    pub fn LocalFlags(hmem: isize) -> u32;
    #[doc = "*Required features: `\"Win32_System_Memory\"`*"]
    pub fn LocalFree(hmem: isize) -> isize;
    #[doc = "*Required features: `\"Win32_System_Memory\"`*"]
    pub fn LocalHandle(pmem: *const ::core::ffi::c_void) -> isize;
    #[doc = "*Required features: `\"Win32_System_Memory\"`*"]
    pub fn LocalLock(hmem: isize) -> *mut ::core::ffi::c_void;
    #[doc = "*Required features: `\"Win32_System_Memory\"`*"]
    pub fn LocalReAlloc(hmem: isize, ubytes: usize, uflags: u32) -> isize;
    #[doc = "*Required features: `\"Win32_System_Memory\"`*"]
    pub fn LocalSize(hmem: isize) -> usize;
    #[doc = "*Required features: `\"Win32_System_Memory\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn LocalUnlock(hmem: isize) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_System_Memory\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn MapUserPhysicalPages(virtualaddress: *const ::core::ffi::c_void, numberofpages: usize, pagearray: *const usize) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_System_Memory\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn MapUserPhysicalPagesScatter(virtualaddresses: *const *const ::core::ffi::c_void, numberofpages: usize, pagearray: *const usize) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_System_Memory\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn MapViewOfFile(hfilemappingobject: super::super::Foundation::HANDLE, dwdesiredaccess: FILE_MAP, dwfileoffsethigh: u32, dwfileoffsetlow: u32, dwnumberofbytestomap: usize) -> *mut ::core::ffi::c_void;
    #[doc = "*Required features: `\"Win32_System_Memory\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn MapViewOfFile3(filemapping: super::super::Foundation::HANDLE, process: super::super::Foundation::HANDLE, baseaddress: *const ::core::ffi::c_void, offset: u64, viewsize: usize, allocationtype: VIRTUAL_ALLOCATION_TYPE, pageprotection: u32, extendedparameters: *mut MEM_EXTENDED_PARAMETER, parametercount: u32) -> *mut ::core::ffi::c_void;
    #[doc = "*Required features: `\"Win32_System_Memory\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn MapViewOfFile3FromApp(filemapping: super::super::Foundation::HANDLE, process: super::super::Foundation::HANDLE, baseaddress: *const ::core::ffi::c_void, offset: u64, viewsize: usize, allocationtype: VIRTUAL_ALLOCATION_TYPE, pageprotection: u32, extendedparameters: *mut MEM_EXTENDED_PARAMETER, parametercount: u32) -> *mut ::core::ffi::c_void;
    #[doc = "*Required features: `\"Win32_System_Memory\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn MapViewOfFileEx(hfilemappingobject: super::super::Foundation::HANDLE, dwdesiredaccess: FILE_MAP, dwfileoffsethigh: u32, dwfileoffsetlow: u32, dwnumberofbytestomap: usize, lpbaseaddress: *const ::core::ffi::c_void) -> *mut ::core::ffi::c_void;
    #[doc = "*Required features: `\"Win32_System_Memory\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn MapViewOfFileExNuma(hfilemappingobject: super::super::Foundation::HANDLE, dwdesiredaccess: FILE_MAP, dwfileoffsethigh: u32, dwfileoffsetlow: u32, dwnumberofbytestomap: usize, lpbaseaddress: *const ::core::ffi::c_void, nndpreferred: u32) -> *mut ::core::ffi::c_void;
    #[doc = "*Required features: `\"Win32_System_Memory\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn MapViewOfFileFromApp(hfilemappingobject: super::super::Foundation::HANDLE, desiredaccess: FILE_MAP, fileoffset: u64, numberofbytestomap: usize) -> *mut ::core::ffi::c_void;
    #[doc = "*Required features: `\"Win32_System_Memory\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn MapViewOfFileNuma2(filemappinghandle: super::super::Foundation::HANDLE, processhandle: super::super::Foundation::HANDLE, offset: u64, baseaddress: *const ::core::ffi::c_void, viewsize: usize, allocationtype: u32, pageprotection: u32, preferrednode: u32) -> *mut ::core::ffi::c_void;
    #[doc = "*Required features: `\"Win32_System_Memory\"`*"]
    pub fn OfferVirtualMemory(virtualaddress: *mut ::core::ffi::c_void, size: usize, priority: OFFER_PRIORITY) -> u32;
    #[doc = "*Required features: `\"Win32_System_Memory\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn OpenDedicatedMemoryPartition(partition: super::super::Foundation::HANDLE, dedicatedmemorytypeid: u64, desiredaccess: u32, inherithandle: super::super::Foundation::BOOL) -> super::super::Foundation::HANDLE;
    #[doc = "*Required features: `\"Win32_System_Memory\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn OpenFileMappingA(dwdesiredaccess: u32, binherithandle: super::super::Foundation::BOOL, lpname: ::windows_sys::core::PCSTR) -> super::super::Foundation::HANDLE;
    #[doc = "*Required features: `\"Win32_System_Memory\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn OpenFileMappingFromApp(desiredaccess: u32, inherithandle: super::super::Foundation::BOOL, name: ::windows_sys::core::PCWSTR) -> super::super::Foundation::HANDLE;
    #[doc = "*Required features: `\"Win32_System_Memory\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn OpenFileMappingW(dwdesiredaccess: u32, binherithandle: super::super::Foundation::BOOL, lpname: ::windows_sys::core::PCWSTR) -> super::super::Foundation::HANDLE;
    #[doc = "*Required features: `\"Win32_System_Memory\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn PrefetchVirtualMemory(hprocess: super::super::Foundation::HANDLE, numberofentries: usize, virtualaddresses: *const WIN32_MEMORY_RANGE_ENTRY, flags: u32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_System_Memory\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn QueryMemoryResourceNotification(resourcenotificationhandle: super::super::Foundation::HANDLE, resourcestate: *mut super::super::Foundation::BOOL) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_System_Memory\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn QueryPartitionInformation(partition: super::super::Foundation::HANDLE, partitioninformationclass: WIN32_MEMORY_PARTITION_INFORMATION_CLASS, partitioninformation: *mut ::core::ffi::c_void, partitioninformationlength: u32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_System_Memory\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn QueryVirtualMemoryInformation(process: super::super::Foundation::HANDLE, virtualaddress: *const ::core::ffi::c_void, memoryinformationclass: WIN32_MEMORY_INFORMATION_CLASS, memoryinformation: *mut ::core::ffi::c_void, memoryinformationsize: usize, returnsize: *mut usize) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_System_Memory\"`*"]
    pub fn ReclaimVirtualMemory(virtualaddress: *const ::core::ffi::c_void, size: usize) -> u32;
    #[doc = "*Required features: `\"Win32_System_Memory\"`*"]
    pub fn RegisterBadMemoryNotification(callback: PBAD_MEMORY_CALLBACK_ROUTINE) -> *mut ::core::ffi::c_void;
    #[doc = "*Required features: `\"Win32_System_Memory\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn RemoveSecureMemoryCacheCallback(pfncallback: PSECURE_MEMORY_CACHE_CALLBACK) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_System_Memory\"`*"]
    pub fn ResetWriteWatch(lpbaseaddress: *const ::core::ffi::c_void, dwregionsize: usize) -> u32;
    #[doc = "*Required features: `\"Win32_System_Memory\"`*"]
    pub fn RtlCompareMemory(source1: *const ::core::ffi::c_void, source2: *const ::core::ffi::c_void, length: usize) -> usize;
    #[doc = "*Required features: `\"Win32_System_Memory\"`*"]
    pub fn RtlCrc32(buffer: *const ::core::ffi::c_void, size: usize, initialcrc: u32) -> u32;
    #[doc = "*Required features: `\"Win32_System_Memory\"`*"]
    pub fn RtlCrc64(buffer: *const ::core::ffi::c_void, size: usize, initialcrc: u64) -> u64;
    #[doc = "*Required features: `\"Win32_System_Memory\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn RtlIsZeroMemory(buffer: *const ::core::ffi::c_void, length: usize) -> super::super::Foundation::BOOLEAN;
    #[doc = "*Required features: `\"Win32_System_Memory\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetProcessValidCallTargets(hprocess: super::super::Foundation::HANDLE, virtualaddress: *const ::core::ffi::c_void, regionsize: usize, numberofoffsets: u32, offsetinformation: *mut CFG_CALL_TARGET_INFO) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_System_Memory\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetProcessValidCallTargetsForMappedView(process: super::super::Foundation::HANDLE, virtualaddress: *const ::core::ffi::c_void, regionsize: usize, numberofoffsets: u32, offsetinformation: *mut CFG_CALL_TARGET_INFO, section: super::super::Foundation::HANDLE, expectedfileoffset: u64) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_System_Memory\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetProcessWorkingSetSizeEx(hprocess: super::super::Foundation::HANDLE, dwminimumworkingsetsize: usize, dwmaximumworkingsetsize: usize, flags: u32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_System_Memory\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetSystemFileCacheSize(minimumfilecachesize: usize, maximumfilecachesize: usize, flags: u32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_System_Memory\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn UnmapViewOfFile(lpbaseaddress: *const ::core::ffi::c_void) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_System_Memory\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn UnmapViewOfFile2(process: super::super::Foundation::HANDLE, baseaddress: *const ::core::ffi::c_void, unmapflags: UNMAP_VIEW_OF_FILE_FLAGS) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_System_Memory\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn UnmapViewOfFileEx(baseaddress: *const ::core::ffi::c_void, unmapflags: UNMAP_VIEW_OF_FILE_FLAGS) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_System_Memory\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn UnregisterBadMemoryNotification(registrationhandle: *const ::core::ffi::c_void) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_System_Memory\"`*"]
    pub fn VirtualAlloc(lpaddress: *const ::core::ffi::c_void, dwsize: usize, flallocationtype: VIRTUAL_ALLOCATION_TYPE, flprotect: PAGE_PROTECTION_FLAGS) -> *mut ::core::ffi::c_void;
    #[doc = "*Required features: `\"Win32_System_Memory\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn VirtualAlloc2(process: super::super::Foundation::HANDLE, baseaddress: *const ::core::ffi::c_void, size: usize, allocationtype: VIRTUAL_ALLOCATION_TYPE, pageprotection: u32, extendedparameters: *mut MEM_EXTENDED_PARAMETER, parametercount: u32) -> *mut ::core::ffi::c_void;
    #[doc = "*Required features: `\"Win32_System_Memory\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn VirtualAlloc2FromApp(process: super::super::Foundation::HANDLE, baseaddress: *const ::core::ffi::c_void, size: usize, allocationtype: VIRTUAL_ALLOCATION_TYPE, pageprotection: u32, extendedparameters: *mut MEM_EXTENDED_PARAMETER, parametercount: u32) -> *mut ::core::ffi::c_void;
    #[doc = "*Required features: `\"Win32_System_Memory\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn VirtualAllocEx(hprocess: super::super::Foundation::HANDLE, lpaddress: *const ::core::ffi::c_void, dwsize: usize, flallocationtype: VIRTUAL_ALLOCATION_TYPE, flprotect: PAGE_PROTECTION_FLAGS) -> *mut ::core::ffi::c_void;
    #[doc = "*Required features: `\"Win32_System_Memory\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn VirtualAllocExNuma(hprocess: super::super::Foundation::HANDLE, lpaddress: *const ::core::ffi::c_void, dwsize: usize, flallocationtype: VIRTUAL_ALLOCATION_TYPE, flprotect: u32, nndpreferred: u32) -> *mut ::core::ffi::c_void;
    #[doc = "*Required features: `\"Win32_System_Memory\"`*"]
    pub fn VirtualAllocFromApp(baseaddress: *const ::core::ffi::c_void, size: usize, allocationtype: VIRTUAL_ALLOCATION_TYPE, protection: u32) -> *mut ::core::ffi::c_void;
    #[doc = "*Required features: `\"Win32_System_Memory\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn VirtualFree(lpaddress: *mut ::core::ffi::c_void, dwsize: usize, dwfreetype: VIRTUAL_FREE_TYPE) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_System_Memory\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn VirtualFreeEx(hprocess: super::super::Foundation::HANDLE, lpaddress: *mut ::core::ffi::c_void, dwsize: usize, dwfreetype: VIRTUAL_FREE_TYPE) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_System_Memory\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn VirtualLock(lpaddress: *const ::core::ffi::c_void, dwsize: usize) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_System_Memory\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn VirtualProtect(lpaddress: *const ::core::ffi::c_void, dwsize: usize, flnewprotect: PAGE_PROTECTION_FLAGS, lpfloldprotect: *mut PAGE_PROTECTION_FLAGS) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_System_Memory\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn VirtualProtectEx(hprocess: super::super::Foundation::HANDLE, lpaddress: *const ::core::ffi::c_void, dwsize: usize, flnewprotect: PAGE_PROTECTION_FLAGS, lpfloldprotect: *mut PAGE_PROTECTION_FLAGS) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_System_Memory\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn VirtualProtectFromApp(address: *const ::core::ffi::c_void, size: usize, newprotection: u32, oldprotection: *mut u32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_System_Memory\"`*"]
    pub fn VirtualQuery(lpaddress: *const ::core::ffi::c_void, lpbuffer: *mut MEMORY_BASIC_INFORMATION, dwlength: usize) -> usize;
    #[doc = "*Required features: `\"Win32_System_Memory\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn VirtualQueryEx(hprocess: super::super::Foundation::HANDLE, lpaddress: *const ::core::ffi::c_void, lpbuffer: *mut MEMORY_BASIC_INFORMATION, dwlength: usize) -> usize;
    #[doc = "*Required features: `\"Win32_System_Memory\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn VirtualUnlock(lpaddress: *const ::core::ffi::c_void, dwsize: usize) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_System_Memory\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn VirtualUnlockEx(process: super::super::Foundation::HANDLE, address: *const ::core::ffi::c_void, size: usize) -> super::super::Foundation::BOOL;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Memory\"`*"]
pub struct CFG_CALL_TARGET_INFO {
    pub Offset: usize,
    pub Flags: usize,
}
impl ::core::marker::Copy for CFG_CALL_TARGET_INFO {}
impl ::core::clone::Clone for CFG_CALL_TARGET_INFO {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_System_Memory\"`*"]
pub const FILE_CACHE_MAX_HARD_DISABLE: u32 = 2u32;
#[doc = "*Required features: `\"Win32_System_Memory\"`*"]
pub const FILE_CACHE_MAX_HARD_ENABLE: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_Memory\"`*"]
pub const FILE_CACHE_MIN_HARD_DISABLE: u32 = 8u32;
#[doc = "*Required features: `\"Win32_System_Memory\"`*"]
pub const FILE_CACHE_MIN_HARD_ENABLE: u32 = 4u32;
#[doc = "*Required features: `\"Win32_System_Memory\"`*"]
pub type FILE_MAP = u32;
#[doc = "*Required features: `\"Win32_System_Memory\"`*"]
pub const FILE_MAP_WRITE: FILE_MAP = 2u32;
#[doc = "*Required features: `\"Win32_System_Memory\"`*"]
pub const FILE_MAP_READ: FILE_MAP = 4u32;
#[doc = "*Required features: `\"Win32_System_Memory\"`*"]
pub const FILE_MAP_ALL_ACCESS: FILE_MAP = 983071u32;
#[doc = "*Required features: `\"Win32_System_Memory\"`*"]
pub const FILE_MAP_EXECUTE: FILE_MAP = 32u32;
#[doc = "*Required features: `\"Win32_System_Memory\"`*"]
pub const FILE_MAP_COPY: FILE_MAP = 1u32;
#[doc = "*Required features: `\"Win32_System_Memory\"`*"]
pub const FILE_MAP_RESERVE: FILE_MAP = 2147483648u32;
#[doc = "*Required features: `\"Win32_System_Memory\"`*"]
pub const FILE_MAP_TARGETS_INVALID: FILE_MAP = 1073741824u32;
#[doc = "*Required features: `\"Win32_System_Memory\"`*"]
pub const FILE_MAP_LARGE_PAGES: FILE_MAP = 536870912u32;
#[doc = "*Required features: `\"Win32_System_Memory\"`*"]
pub type GLOBAL_ALLOC_FLAGS = u32;
#[doc = "*Required features: `\"Win32_System_Memory\"`*"]
pub const GHND: GLOBAL_ALLOC_FLAGS = 66u32;
#[doc = "*Required features: `\"Win32_System_Memory\"`*"]
pub const GMEM_FIXED: GLOBAL_ALLOC_FLAGS = 0u32;
#[doc = "*Required features: `\"Win32_System_Memory\"`*"]
pub const GMEM_MOVEABLE: GLOBAL_ALLOC_FLAGS = 2u32;
#[doc = "*Required features: `\"Win32_System_Memory\"`*"]
pub const GMEM_ZEROINIT: GLOBAL_ALLOC_FLAGS = 64u32;
#[doc = "*Required features: `\"Win32_System_Memory\"`*"]
pub const GPTR: GLOBAL_ALLOC_FLAGS = 64u32;
#[doc = "*Required features: `\"Win32_System_Memory\"`*"]
pub type HEAP_FLAGS = u32;
#[doc = "*Required features: `\"Win32_System_Memory\"`*"]
pub const HEAP_NONE: HEAP_FLAGS = 0u32;
#[doc = "*Required features: `\"Win32_System_Memory\"`*"]
pub const HEAP_NO_SERIALIZE: HEAP_FLAGS = 1u32;
#[doc = "*Required features: `\"Win32_System_Memory\"`*"]
pub const HEAP_GROWABLE: HEAP_FLAGS = 2u32;
#[doc = "*Required features: `\"Win32_System_Memory\"`*"]
pub const HEAP_GENERATE_EXCEPTIONS: HEAP_FLAGS = 4u32;
#[doc = "*Required features: `\"Win32_System_Memory\"`*"]
pub const HEAP_ZERO_MEMORY: HEAP_FLAGS = 8u32;
#[doc = "*Required features: `\"Win32_System_Memory\"`*"]
pub const HEAP_REALLOC_IN_PLACE_ONLY: HEAP_FLAGS = 16u32;
#[doc = "*Required features: `\"Win32_System_Memory\"`*"]
pub const HEAP_TAIL_CHECKING_ENABLED: HEAP_FLAGS = 32u32;
#[doc = "*Required features: `\"Win32_System_Memory\"`*"]
pub const HEAP_FREE_CHECKING_ENABLED: HEAP_FLAGS = 64u32;
#[doc = "*Required features: `\"Win32_System_Memory\"`*"]
pub const HEAP_DISABLE_COALESCE_ON_FREE: HEAP_FLAGS = 128u32;
#[doc = "*Required features: `\"Win32_System_Memory\"`*"]
pub const HEAP_CREATE_ALIGN_16: HEAP_FLAGS = 65536u32;
#[doc = "*Required features: `\"Win32_System_Memory\"`*"]
pub const HEAP_CREATE_ENABLE_TRACING: HEAP_FLAGS = 131072u32;
#[doc = "*Required features: `\"Win32_System_Memory\"`*"]
pub const HEAP_CREATE_ENABLE_EXECUTE: HEAP_FLAGS = 262144u32;
#[doc = "*Required features: `\"Win32_System_Memory\"`*"]
pub const HEAP_MAXIMUM_TAG: HEAP_FLAGS = 4095u32;
#[doc = "*Required features: `\"Win32_System_Memory\"`*"]
pub const HEAP_PSEUDO_TAG_FLAG: HEAP_FLAGS = 32768u32;
#[doc = "*Required features: `\"Win32_System_Memory\"`*"]
pub const HEAP_TAG_SHIFT: HEAP_FLAGS = 18u32;
#[doc = "*Required features: `\"Win32_System_Memory\"`*"]
pub const HEAP_CREATE_SEGMENT_HEAP: HEAP_FLAGS = 256u32;
#[doc = "*Required features: `\"Win32_System_Memory\"`*"]
pub const HEAP_CREATE_HARDENED: HEAP_FLAGS = 512u32;
#[doc = "*Required features: `\"Win32_System_Memory\"`*"]
pub type HEAP_INFORMATION_CLASS = i32;
#[doc = "*Required features: `\"Win32_System_Memory\"`*"]
pub const HeapCompatibilityInformation: HEAP_INFORMATION_CLASS = 0i32;
#[doc = "*Required features: `\"Win32_System_Memory\"`*"]
pub const HeapEnableTerminationOnCorruption: HEAP_INFORMATION_CLASS = 1i32;
#[doc = "*Required features: `\"Win32_System_Memory\"`*"]
pub const HeapOptimizeResources: HEAP_INFORMATION_CLASS = 3i32;
#[doc = "*Required features: `\"Win32_System_Memory\"`*"]
pub const HeapTag: HEAP_INFORMATION_CLASS = 7i32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Memory\"`*"]
pub struct HEAP_SUMMARY {
    pub cb: u32,
    pub cbAllocated: usize,
    pub cbCommitted: usize,
    pub cbReserved: usize,
    pub cbMaxReserve: usize,
}
impl ::core::marker::Copy for HEAP_SUMMARY {}
impl ::core::clone::Clone for HEAP_SUMMARY {
    fn clone(&self) -> Self {
        *self
    }
}
pub type HeapHandle = isize;
#[doc = "*Required features: `\"Win32_System_Memory\"`*"]
pub type LOCAL_ALLOC_FLAGS = u32;
#[doc = "*Required features: `\"Win32_System_Memory\"`*"]
pub const LHND: LOCAL_ALLOC_FLAGS = 66u32;
#[doc = "*Required features: `\"Win32_System_Memory\"`*"]
pub const LMEM_FIXED: LOCAL_ALLOC_FLAGS = 0u32;
#[doc = "*Required features: `\"Win32_System_Memory\"`*"]
pub const LMEM_MOVEABLE: LOCAL_ALLOC_FLAGS = 2u32;
#[doc = "*Required features: `\"Win32_System_Memory\"`*"]
pub const LMEM_ZEROINIT: LOCAL_ALLOC_FLAGS = 64u32;
#[doc = "*Required features: `\"Win32_System_Memory\"`*"]
pub const LPTR: LOCAL_ALLOC_FLAGS = 64u32;
#[doc = "*Required features: `\"Win32_System_Memory\"`*"]
pub const NONZEROLHND: LOCAL_ALLOC_FLAGS = 2u32;
#[doc = "*Required features: `\"Win32_System_Memory\"`*"]
pub const NONZEROLPTR: LOCAL_ALLOC_FLAGS = 0u32;
#[doc = "*Required features: `\"Win32_System_Memory\"`*"]
pub const MEHC_PATROL_SCRUBBER_PRESENT: u32 = 1u32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Memory\"`*"]
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
pub struct MEMORY_BASIC_INFORMATION {
    pub BaseAddress: *mut ::core::ffi::c_void,
    pub AllocationBase: *mut ::core::ffi::c_void,
    pub AllocationProtect: PAGE_PROTECTION_FLAGS,
    pub PartitionId: u16,
    pub RegionSize: usize,
    pub State: VIRTUAL_ALLOCATION_TYPE,
    pub Protect: PAGE_PROTECTION_FLAGS,
    pub Type: PAGE_TYPE,
}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
impl ::core::marker::Copy for MEMORY_BASIC_INFORMATION {}
#[cfg(any(target_arch = "aarch64", target_arch = "x86_64"))]
impl ::core::clone::Clone for MEMORY_BASIC_INFORMATION {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Memory\"`*"]
#[cfg(target_arch = "x86")]
pub struct MEMORY_BASIC_INFORMATION {
    pub BaseAddress: *mut ::core::ffi::c_void,
    pub AllocationBase: *mut ::core::ffi::c_void,
    pub AllocationProtect: PAGE_PROTECTION_FLAGS,
    pub RegionSize: usize,
    pub State: VIRTUAL_ALLOCATION_TYPE,
    pub Protect: PAGE_PROTECTION_FLAGS,
    pub Type: PAGE_TYPE,
}
#[cfg(target_arch = "x86")]
impl ::core::marker::Copy for MEMORY_BASIC_INFORMATION {}
#[cfg(target_arch = "x86")]
impl ::core::clone::Clone for MEMORY_BASIC_INFORMATION {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Memory\"`*"]
pub struct MEMORY_BASIC_INFORMATION32 {
    pub BaseAddress: u32,
    pub AllocationBase: u32,
    pub AllocationProtect: PAGE_PROTECTION_FLAGS,
    pub RegionSize: u32,
    pub State: VIRTUAL_ALLOCATION_TYPE,
    pub Protect: PAGE_PROTECTION_FLAGS,
    pub Type: PAGE_TYPE,
}
impl ::core::marker::Copy for MEMORY_BASIC_INFORMATION32 {}
impl ::core::clone::Clone for MEMORY_BASIC_INFORMATION32 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Memory\"`*"]
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
impl ::core::marker::Copy for MEMORY_BASIC_INFORMATION64 {}
impl ::core::clone::Clone for MEMORY_BASIC_INFORMATION64 {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_System_Memory\"`*"]
pub type MEMORY_RESOURCE_NOTIFICATION_TYPE = i32;
#[doc = "*Required features: `\"Win32_System_Memory\"`*"]
pub const LowMemoryResourceNotification: MEMORY_RESOURCE_NOTIFICATION_TYPE = 0i32;
#[doc = "*Required features: `\"Win32_System_Memory\"`*"]
pub const HighMemoryResourceNotification: MEMORY_RESOURCE_NOTIFICATION_TYPE = 1i32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Memory\"`*"]
pub struct MEM_ADDRESS_REQUIREMENTS {
    pub LowestStartingAddress: *mut ::core::ffi::c_void,
    pub HighestEndingAddress: *mut ::core::ffi::c_void,
    pub Alignment: usize,
}
impl ::core::marker::Copy for MEM_ADDRESS_REQUIREMENTS {}
impl ::core::clone::Clone for MEM_ADDRESS_REQUIREMENTS {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Memory\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct MEM_EXTENDED_PARAMETER {
    pub Anonymous1: MEM_EXTENDED_PARAMETER_0,
    pub Anonymous2: MEM_EXTENDED_PARAMETER_1,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for MEM_EXTENDED_PARAMETER {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for MEM_EXTENDED_PARAMETER {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Memory\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct MEM_EXTENDED_PARAMETER_0 {
    pub _bitfield: u64,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for MEM_EXTENDED_PARAMETER_0 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for MEM_EXTENDED_PARAMETER_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Memory\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub union MEM_EXTENDED_PARAMETER_1 {
    pub ULong64: u64,
    pub Pointer: *mut ::core::ffi::c_void,
    pub Size: usize,
    pub Handle: super::super::Foundation::HANDLE,
    pub ULong: u32,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for MEM_EXTENDED_PARAMETER_1 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for MEM_EXTENDED_PARAMETER_1 {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_System_Memory\"`*"]
pub type MEM_EXTENDED_PARAMETER_TYPE = i32;
#[doc = "*Required features: `\"Win32_System_Memory\"`*"]
pub const MemExtendedParameterInvalidType: MEM_EXTENDED_PARAMETER_TYPE = 0i32;
#[doc = "*Required features: `\"Win32_System_Memory\"`*"]
pub const MemExtendedParameterAddressRequirements: MEM_EXTENDED_PARAMETER_TYPE = 1i32;
#[doc = "*Required features: `\"Win32_System_Memory\"`*"]
pub const MemExtendedParameterNumaNode: MEM_EXTENDED_PARAMETER_TYPE = 2i32;
#[doc = "*Required features: `\"Win32_System_Memory\"`*"]
pub const MemExtendedParameterPartitionHandle: MEM_EXTENDED_PARAMETER_TYPE = 3i32;
#[doc = "*Required features: `\"Win32_System_Memory\"`*"]
pub const MemExtendedParameterUserPhysicalHandle: MEM_EXTENDED_PARAMETER_TYPE = 4i32;
#[doc = "*Required features: `\"Win32_System_Memory\"`*"]
pub const MemExtendedParameterAttributeFlags: MEM_EXTENDED_PARAMETER_TYPE = 5i32;
#[doc = "*Required features: `\"Win32_System_Memory\"`*"]
pub const MemExtendedParameterImageMachine: MEM_EXTENDED_PARAMETER_TYPE = 6i32;
#[doc = "*Required features: `\"Win32_System_Memory\"`*"]
pub const MemExtendedParameterMax: MEM_EXTENDED_PARAMETER_TYPE = 7i32;
#[doc = "*Required features: `\"Win32_System_Memory\"`*"]
pub type OFFER_PRIORITY = i32;
#[doc = "*Required features: `\"Win32_System_Memory\"`*"]
pub const VmOfferPriorityVeryLow: OFFER_PRIORITY = 1i32;
#[doc = "*Required features: `\"Win32_System_Memory\"`*"]
pub const VmOfferPriorityLow: OFFER_PRIORITY = 2i32;
#[doc = "*Required features: `\"Win32_System_Memory\"`*"]
pub const VmOfferPriorityBelowNormal: OFFER_PRIORITY = 3i32;
#[doc = "*Required features: `\"Win32_System_Memory\"`*"]
pub const VmOfferPriorityNormal: OFFER_PRIORITY = 4i32;
#[doc = "*Required features: `\"Win32_System_Memory\"`*"]
pub type PAGE_PROTECTION_FLAGS = u32;
#[doc = "*Required features: `\"Win32_System_Memory\"`*"]
pub const PAGE_NOACCESS: PAGE_PROTECTION_FLAGS = 1u32;
#[doc = "*Required features: `\"Win32_System_Memory\"`*"]
pub const PAGE_READONLY: PAGE_PROTECTION_FLAGS = 2u32;
#[doc = "*Required features: `\"Win32_System_Memory\"`*"]
pub const PAGE_READWRITE: PAGE_PROTECTION_FLAGS = 4u32;
#[doc = "*Required features: `\"Win32_System_Memory\"`*"]
pub const PAGE_WRITECOPY: PAGE_PROTECTION_FLAGS = 8u32;
#[doc = "*Required features: `\"Win32_System_Memory\"`*"]
pub const PAGE_EXECUTE: PAGE_PROTECTION_FLAGS = 16u32;
#[doc = "*Required features: `\"Win32_System_Memory\"`*"]
pub const PAGE_EXECUTE_READ: PAGE_PROTECTION_FLAGS = 32u32;
#[doc = "*Required features: `\"Win32_System_Memory\"`*"]
pub const PAGE_EXECUTE_READWRITE: PAGE_PROTECTION_FLAGS = 64u32;
#[doc = "*Required features: `\"Win32_System_Memory\"`*"]
pub const PAGE_EXECUTE_WRITECOPY: PAGE_PROTECTION_FLAGS = 128u32;
#[doc = "*Required features: `\"Win32_System_Memory\"`*"]
pub const PAGE_GUARD: PAGE_PROTECTION_FLAGS = 256u32;
#[doc = "*Required features: `\"Win32_System_Memory\"`*"]
pub const PAGE_NOCACHE: PAGE_PROTECTION_FLAGS = 512u32;
#[doc = "*Required features: `\"Win32_System_Memory\"`*"]
pub const PAGE_WRITECOMBINE: PAGE_PROTECTION_FLAGS = 1024u32;
#[doc = "*Required features: `\"Win32_System_Memory\"`*"]
pub const PAGE_GRAPHICS_NOACCESS: PAGE_PROTECTION_FLAGS = 2048u32;
#[doc = "*Required features: `\"Win32_System_Memory\"`*"]
pub const PAGE_GRAPHICS_READONLY: PAGE_PROTECTION_FLAGS = 4096u32;
#[doc = "*Required features: `\"Win32_System_Memory\"`*"]
pub const PAGE_GRAPHICS_READWRITE: PAGE_PROTECTION_FLAGS = 8192u32;
#[doc = "*Required features: `\"Win32_System_Memory\"`*"]
pub const PAGE_GRAPHICS_EXECUTE: PAGE_PROTECTION_FLAGS = 16384u32;
#[doc = "*Required features: `\"Win32_System_Memory\"`*"]
pub const PAGE_GRAPHICS_EXECUTE_READ: PAGE_PROTECTION_FLAGS = 32768u32;
#[doc = "*Required features: `\"Win32_System_Memory\"`*"]
pub const PAGE_GRAPHICS_EXECUTE_READWRITE: PAGE_PROTECTION_FLAGS = 65536u32;
#[doc = "*Required features: `\"Win32_System_Memory\"`*"]
pub const PAGE_GRAPHICS_COHERENT: PAGE_PROTECTION_FLAGS = 131072u32;
#[doc = "*Required features: `\"Win32_System_Memory\"`*"]
pub const PAGE_GRAPHICS_NOCACHE: PAGE_PROTECTION_FLAGS = 262144u32;
#[doc = "*Required features: `\"Win32_System_Memory\"`*"]
pub const PAGE_ENCLAVE_THREAD_CONTROL: PAGE_PROTECTION_FLAGS = 2147483648u32;
#[doc = "*Required features: `\"Win32_System_Memory\"`*"]
pub const PAGE_REVERT_TO_FILE_MAP: PAGE_PROTECTION_FLAGS = 2147483648u32;
#[doc = "*Required features: `\"Win32_System_Memory\"`*"]
pub const PAGE_TARGETS_NO_UPDATE: PAGE_PROTECTION_FLAGS = 1073741824u32;
#[doc = "*Required features: `\"Win32_System_Memory\"`*"]
pub const PAGE_TARGETS_INVALID: PAGE_PROTECTION_FLAGS = 1073741824u32;
#[doc = "*Required features: `\"Win32_System_Memory\"`*"]
pub const PAGE_ENCLAVE_UNVALIDATED: PAGE_PROTECTION_FLAGS = 536870912u32;
#[doc = "*Required features: `\"Win32_System_Memory\"`*"]
pub const PAGE_ENCLAVE_MASK: PAGE_PROTECTION_FLAGS = 268435456u32;
#[doc = "*Required features: `\"Win32_System_Memory\"`*"]
pub const PAGE_ENCLAVE_DECOMMIT: PAGE_PROTECTION_FLAGS = 268435456u32;
#[doc = "*Required features: `\"Win32_System_Memory\"`*"]
pub const PAGE_ENCLAVE_SS_FIRST: PAGE_PROTECTION_FLAGS = 268435457u32;
#[doc = "*Required features: `\"Win32_System_Memory\"`*"]
pub const PAGE_ENCLAVE_SS_REST: PAGE_PROTECTION_FLAGS = 268435458u32;
#[doc = "*Required features: `\"Win32_System_Memory\"`*"]
pub const SEC_PARTITION_OWNER_HANDLE: PAGE_PROTECTION_FLAGS = 262144u32;
#[doc = "*Required features: `\"Win32_System_Memory\"`*"]
pub const SEC_64K_PAGES: PAGE_PROTECTION_FLAGS = 524288u32;
#[doc = "*Required features: `\"Win32_System_Memory\"`*"]
pub const SEC_FILE: PAGE_PROTECTION_FLAGS = 8388608u32;
#[doc = "*Required features: `\"Win32_System_Memory\"`*"]
pub const SEC_IMAGE: PAGE_PROTECTION_FLAGS = 16777216u32;
#[doc = "*Required features: `\"Win32_System_Memory\"`*"]
pub const SEC_PROTECTED_IMAGE: PAGE_PROTECTION_FLAGS = 33554432u32;
#[doc = "*Required features: `\"Win32_System_Memory\"`*"]
pub const SEC_RESERVE: PAGE_PROTECTION_FLAGS = 67108864u32;
#[doc = "*Required features: `\"Win32_System_Memory\"`*"]
pub const SEC_COMMIT: PAGE_PROTECTION_FLAGS = 134217728u32;
#[doc = "*Required features: `\"Win32_System_Memory\"`*"]
pub const SEC_NOCACHE: PAGE_PROTECTION_FLAGS = 268435456u32;
#[doc = "*Required features: `\"Win32_System_Memory\"`*"]
pub const SEC_WRITECOMBINE: PAGE_PROTECTION_FLAGS = 1073741824u32;
#[doc = "*Required features: `\"Win32_System_Memory\"`*"]
pub const SEC_LARGE_PAGES: PAGE_PROTECTION_FLAGS = 2147483648u32;
#[doc = "*Required features: `\"Win32_System_Memory\"`*"]
pub const SEC_IMAGE_NO_EXECUTE: PAGE_PROTECTION_FLAGS = 285212672u32;
#[doc = "*Required features: `\"Win32_System_Memory\"`*"]
pub type PAGE_TYPE = u32;
#[doc = "*Required features: `\"Win32_System_Memory\"`*"]
pub const MEM_PRIVATE: PAGE_TYPE = 131072u32;
#[doc = "*Required features: `\"Win32_System_Memory\"`*"]
pub const MEM_MAPPED: PAGE_TYPE = 262144u32;
#[doc = "*Required features: `\"Win32_System_Memory\"`*"]
pub const MEM_IMAGE: PAGE_TYPE = 16777216u32;
#[doc = "*Required features: `\"Win32_System_Memory\"`*"]
pub type PBAD_MEMORY_CALLBACK_ROUTINE = ::core::option::Option<unsafe extern "system" fn()>;
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Memory\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct PROCESS_HEAP_ENTRY {
    pub lpData: *mut ::core::ffi::c_void,
    pub cbData: u32,
    pub cbOverhead: u8,
    pub iRegionIndex: u8,
    pub wFlags: u16,
    pub Anonymous: PROCESS_HEAP_ENTRY_0,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for PROCESS_HEAP_ENTRY {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for PROCESS_HEAP_ENTRY {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Memory\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub union PROCESS_HEAP_ENTRY_0 {
    pub Block: PROCESS_HEAP_ENTRY_0_0,
    pub Region: PROCESS_HEAP_ENTRY_0_1,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for PROCESS_HEAP_ENTRY_0 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for PROCESS_HEAP_ENTRY_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Memory\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct PROCESS_HEAP_ENTRY_0_0 {
    pub hMem: super::super::Foundation::HANDLE,
    pub dwReserved: [u32; 3],
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for PROCESS_HEAP_ENTRY_0_0 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for PROCESS_HEAP_ENTRY_0_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Memory\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct PROCESS_HEAP_ENTRY_0_1 {
    pub dwCommittedSize: u32,
    pub dwUnCommittedSize: u32,
    pub lpFirstBlock: *mut ::core::ffi::c_void,
    pub lpLastBlock: *mut ::core::ffi::c_void,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for PROCESS_HEAP_ENTRY_0_1 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for PROCESS_HEAP_ENTRY_0_1 {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_System_Memory\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub type PSECURE_MEMORY_CACHE_CALLBACK = ::core::option::Option<unsafe extern "system" fn(addr: *const ::core::ffi::c_void, range: usize) -> super::super::Foundation::BOOLEAN>;
#[doc = "*Required features: `\"Win32_System_Memory\"`*"]
pub type UNMAP_VIEW_OF_FILE_FLAGS = u32;
#[doc = "*Required features: `\"Win32_System_Memory\"`*"]
pub const MEM_UNMAP_NONE: UNMAP_VIEW_OF_FILE_FLAGS = 0u32;
#[doc = "*Required features: `\"Win32_System_Memory\"`*"]
pub const MEM_UNMAP_WITH_TRANSIENT_BOOST: UNMAP_VIEW_OF_FILE_FLAGS = 1u32;
#[doc = "*Required features: `\"Win32_System_Memory\"`*"]
pub const MEM_PRESERVE_PLACEHOLDER: UNMAP_VIEW_OF_FILE_FLAGS = 2u32;
#[doc = "*Required features: `\"Win32_System_Memory\"`*"]
pub type VIRTUAL_ALLOCATION_TYPE = u32;
#[doc = "*Required features: `\"Win32_System_Memory\"`*"]
pub const MEM_COMMIT: VIRTUAL_ALLOCATION_TYPE = 4096u32;
#[doc = "*Required features: `\"Win32_System_Memory\"`*"]
pub const MEM_RESERVE: VIRTUAL_ALLOCATION_TYPE = 8192u32;
#[doc = "*Required features: `\"Win32_System_Memory\"`*"]
pub const MEM_RESET: VIRTUAL_ALLOCATION_TYPE = 524288u32;
#[doc = "*Required features: `\"Win32_System_Memory\"`*"]
pub const MEM_RESET_UNDO: VIRTUAL_ALLOCATION_TYPE = 16777216u32;
#[doc = "*Required features: `\"Win32_System_Memory\"`*"]
pub const MEM_REPLACE_PLACEHOLDER: VIRTUAL_ALLOCATION_TYPE = 16384u32;
#[doc = "*Required features: `\"Win32_System_Memory\"`*"]
pub const MEM_LARGE_PAGES: VIRTUAL_ALLOCATION_TYPE = 536870912u32;
#[doc = "*Required features: `\"Win32_System_Memory\"`*"]
pub const MEM_RESERVE_PLACEHOLDER: VIRTUAL_ALLOCATION_TYPE = 262144u32;
#[doc = "*Required features: `\"Win32_System_Memory\"`*"]
pub const MEM_FREE: VIRTUAL_ALLOCATION_TYPE = 65536u32;
#[doc = "*Required features: `\"Win32_System_Memory\"`*"]
pub type VIRTUAL_FREE_TYPE = u32;
#[doc = "*Required features: `\"Win32_System_Memory\"`*"]
pub const MEM_DECOMMIT: VIRTUAL_FREE_TYPE = 16384u32;
#[doc = "*Required features: `\"Win32_System_Memory\"`*"]
pub const MEM_RELEASE: VIRTUAL_FREE_TYPE = 32768u32;
#[doc = "*Required features: `\"Win32_System_Memory\"`*"]
pub type WIN32_MEMORY_INFORMATION_CLASS = i32;
#[doc = "*Required features: `\"Win32_System_Memory\"`*"]
pub const MemoryRegionInfo: WIN32_MEMORY_INFORMATION_CLASS = 0i32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Memory\"`*"]
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
impl ::core::marker::Copy for WIN32_MEMORY_PARTITION_INFORMATION {}
impl ::core::clone::Clone for WIN32_MEMORY_PARTITION_INFORMATION {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_System_Memory\"`*"]
pub type WIN32_MEMORY_PARTITION_INFORMATION_CLASS = i32;
#[doc = "*Required features: `\"Win32_System_Memory\"`*"]
pub const MemoryPartitionInfo: WIN32_MEMORY_PARTITION_INFORMATION_CLASS = 0i32;
#[doc = "*Required features: `\"Win32_System_Memory\"`*"]
pub const MemoryPartitionDedicatedMemoryInfo: WIN32_MEMORY_PARTITION_INFORMATION_CLASS = 1i32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Memory\"`*"]
pub struct WIN32_MEMORY_RANGE_ENTRY {
    pub VirtualAddress: *mut ::core::ffi::c_void,
    pub NumberOfBytes: usize,
}
impl ::core::marker::Copy for WIN32_MEMORY_RANGE_ENTRY {}
impl ::core::clone::Clone for WIN32_MEMORY_RANGE_ENTRY {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Memory\"`*"]
pub struct WIN32_MEMORY_REGION_INFORMATION {
    pub AllocationBase: *mut ::core::ffi::c_void,
    pub AllocationProtect: u32,
    pub Anonymous: WIN32_MEMORY_REGION_INFORMATION_0,
    pub RegionSize: usize,
    pub CommitSize: usize,
}
impl ::core::marker::Copy for WIN32_MEMORY_REGION_INFORMATION {}
impl ::core::clone::Clone for WIN32_MEMORY_REGION_INFORMATION {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Memory\"`*"]
pub union WIN32_MEMORY_REGION_INFORMATION_0 {
    pub Flags: u32,
    pub Anonymous: WIN32_MEMORY_REGION_INFORMATION_0_0,
}
impl ::core::marker::Copy for WIN32_MEMORY_REGION_INFORMATION_0 {}
impl ::core::clone::Clone for WIN32_MEMORY_REGION_INFORMATION_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Memory\"`*"]
pub struct WIN32_MEMORY_REGION_INFORMATION_0_0 {
    pub _bitfield: u32,
}
impl ::core::marker::Copy for WIN32_MEMORY_REGION_INFORMATION_0_0 {}
impl ::core::clone::Clone for WIN32_MEMORY_REGION_INFORMATION_0_0 {
    fn clone(&self) -> Self {
        *self
    }
}

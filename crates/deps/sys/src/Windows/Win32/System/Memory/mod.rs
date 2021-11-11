#![allow(non_snake_case, non_camel_case_types)]
#[cfg(feature = "Win32_System_Memory_NonVolatile")]
pub mod NonVolatile;
#[link(name = "windows")]
extern "system" {
    #[doc = "*Required features: `Win32_System_Memory`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn AddSecureMemoryCacheCallback(pfncallback: ::windows::runtime::RawPtr) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_System_Memory`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn AllocateUserPhysicalPages(hprocess: super::super::Foundation::HANDLE, numberofpages: *mut usize, pagearray: *mut usize) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_System_Memory`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn AllocateUserPhysicalPages2(objecthandle: super::super::Foundation::HANDLE, numberofpages: *mut usize, pagearray: *mut usize, extendedparameters: *mut MEM_EXTENDED_PARAMETER, extendedparametercount: u32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_System_Memory`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn AllocateUserPhysicalPagesNuma(hprocess: super::super::Foundation::HANDLE, numberofpages: *mut usize, pagearray: *mut usize, nndpreferred: u32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_System_Memory`, `Win32_Foundation`, `Win32_Security`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
    pub fn CreateFileMapping2(file: super::super::Foundation::HANDLE, securityattributes: *const super::super::Security::SECURITY_ATTRIBUTES, desiredaccess: u32, pageprotection: PAGE_PROTECTION_FLAGS, allocationattributes: u32, maximumsize: u64, name: super::super::Foundation::PWSTR, extendedparameters: *mut MEM_EXTENDED_PARAMETER, parametercount: u32) -> super::super::Foundation::HANDLE;
    #[doc = "*Required features: `Win32_System_Memory`, `Win32_Foundation`, `Win32_Security`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
    pub fn CreateFileMappingA(hfile: super::super::Foundation::HANDLE, lpfilemappingattributes: *const super::super::Security::SECURITY_ATTRIBUTES, flprotect: PAGE_PROTECTION_FLAGS, dwmaximumsizehigh: u32, dwmaximumsizelow: u32, lpname: super::super::Foundation::PSTR) -> super::super::Foundation::HANDLE;
    #[doc = "*Required features: `Win32_System_Memory`, `Win32_Foundation`, `Win32_Security`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
    pub fn CreateFileMappingFromApp(hfile: super::super::Foundation::HANDLE, securityattributes: *const super::super::Security::SECURITY_ATTRIBUTES, pageprotection: PAGE_PROTECTION_FLAGS, maximumsize: u64, name: super::super::Foundation::PWSTR) -> super::super::Foundation::HANDLE;
    #[doc = "*Required features: `Win32_System_Memory`, `Win32_Foundation`, `Win32_Security`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
    pub fn CreateFileMappingNumaA(hfile: super::super::Foundation::HANDLE, lpfilemappingattributes: *const super::super::Security::SECURITY_ATTRIBUTES, flprotect: PAGE_PROTECTION_FLAGS, dwmaximumsizehigh: u32, dwmaximumsizelow: u32, lpname: super::super::Foundation::PSTR, nndpreferred: u32) -> super::super::Foundation::HANDLE;
    #[doc = "*Required features: `Win32_System_Memory`, `Win32_Foundation`, `Win32_Security`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
    pub fn CreateFileMappingNumaW(hfile: super::super::Foundation::HANDLE, lpfilemappingattributes: *const super::super::Security::SECURITY_ATTRIBUTES, flprotect: PAGE_PROTECTION_FLAGS, dwmaximumsizehigh: u32, dwmaximumsizelow: u32, lpname: super::super::Foundation::PWSTR, nndpreferred: u32) -> super::super::Foundation::HANDLE;
    #[doc = "*Required features: `Win32_System_Memory`, `Win32_Foundation`, `Win32_Security`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
    pub fn CreateFileMappingW(hfile: super::super::Foundation::HANDLE, lpfilemappingattributes: *const super::super::Security::SECURITY_ATTRIBUTES, flprotect: PAGE_PROTECTION_FLAGS, dwmaximumsizehigh: u32, dwmaximumsizelow: u32, lpname: super::super::Foundation::PWSTR) -> super::super::Foundation::HANDLE;
    #[doc = "*Required features: `Win32_System_Memory`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CreateMemoryResourceNotification(notificationtype: MEMORY_RESOURCE_NOTIFICATION_TYPE) -> super::super::Foundation::HANDLE;
    #[doc = "*Required features: `Win32_System_Memory`*"]
    pub fn DiscardVirtualMemory(virtualaddress: *mut ::core::ffi::c_void, size: usize) -> u32;
    #[doc = "*Required features: `Win32_System_Memory`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn FlushViewOfFile(lpbaseaddress: *const ::core::ffi::c_void, dwnumberofbytestoflush: usize) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_System_Memory`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn FreeUserPhysicalPages(hprocess: super::super::Foundation::HANDLE, numberofpages: *mut usize, pagearray: *const usize) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_System_Memory`*"]
    pub fn GetLargePageMinimum() -> usize;
    #[doc = "*Required features: `Win32_System_Memory`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetMemoryErrorHandlingCapabilities(capabilities: *mut u32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_System_Memory`*"]
    pub fn GetProcessHeap() -> HeapHandle;
    #[doc = "*Required features: `Win32_System_Memory`*"]
    pub fn GetProcessHeaps(numberofheaps: u32, processheaps: *mut HeapHandle) -> u32;
    #[doc = "*Required features: `Win32_System_Memory`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetProcessWorkingSetSizeEx(hprocess: super::super::Foundation::HANDLE, lpminimumworkingsetsize: *mut usize, lpmaximumworkingsetsize: *mut usize, flags: *mut u32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_System_Memory`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetSystemFileCacheSize(lpminimumfilecachesize: *mut usize, lpmaximumfilecachesize: *mut usize, lpflags: *mut u32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_System_Memory`*"]
    pub fn GetWriteWatch(dwflags: u32, lpbaseaddress: *const ::core::ffi::c_void, dwregionsize: usize, lpaddresses: *mut *mut ::core::ffi::c_void, lpdwcount: *mut usize, lpdwgranularity: *mut u32) -> u32;
    #[doc = "*Required features: `Win32_System_Memory`*"]
    pub fn GlobalAlloc(uflags: GLOBAL_ALLOC_FLAGS, dwbytes: usize) -> isize;
    #[doc = "*Required features: `Win32_System_Memory`*"]
    pub fn GlobalFlags(hmem: isize) -> u32;
    #[doc = "*Required features: `Win32_System_Memory`*"]
    pub fn GlobalFree(hmem: isize) -> isize;
    #[doc = "*Required features: `Win32_System_Memory`*"]
    pub fn GlobalHandle(pmem: *const ::core::ffi::c_void) -> isize;
    #[doc = "*Required features: `Win32_System_Memory`*"]
    pub fn GlobalLock(hmem: isize) -> *mut ::core::ffi::c_void;
    #[doc = "*Required features: `Win32_System_Memory`*"]
    pub fn GlobalReAlloc(hmem: isize, dwbytes: usize, uflags: u32) -> isize;
    #[doc = "*Required features: `Win32_System_Memory`*"]
    pub fn GlobalSize(hmem: isize) -> usize;
    #[doc = "*Required features: `Win32_System_Memory`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GlobalUnlock(hmem: isize) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_System_Memory`*"]
    pub fn HeapAlloc(hheap: HeapHandle, dwflags: HEAP_FLAGS, dwbytes: usize) -> *mut ::core::ffi::c_void;
    #[doc = "*Required features: `Win32_System_Memory`*"]
    pub fn HeapCompact(hheap: HeapHandle, dwflags: HEAP_FLAGS) -> usize;
    #[doc = "*Required features: `Win32_System_Memory`*"]
    pub fn HeapCreate(floptions: HEAP_FLAGS, dwinitialsize: usize, dwmaximumsize: usize) -> HeapHandle;
    #[doc = "*Required features: `Win32_System_Memory`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn HeapDestroy(hheap: HeapHandle) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_System_Memory`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn HeapFree(hheap: HeapHandle, dwflags: HEAP_FLAGS, lpmem: *const ::core::ffi::c_void) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_System_Memory`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn HeapLock(hheap: HeapHandle) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_System_Memory`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn HeapQueryInformation(heaphandle: HeapHandle, heapinformationclass: HEAP_INFORMATION_CLASS, heapinformation: *mut ::core::ffi::c_void, heapinformationlength: usize, returnlength: *mut usize) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_System_Memory`*"]
    pub fn HeapReAlloc(hheap: HeapHandle, dwflags: HEAP_FLAGS, lpmem: *const ::core::ffi::c_void, dwbytes: usize) -> *mut ::core::ffi::c_void;
    #[doc = "*Required features: `Win32_System_Memory`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn HeapSetInformation(heaphandle: HeapHandle, heapinformationclass: HEAP_INFORMATION_CLASS, heapinformation: *const ::core::ffi::c_void, heapinformationlength: usize) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_System_Memory`*"]
    pub fn HeapSize(hheap: HeapHandle, dwflags: HEAP_FLAGS, lpmem: *const ::core::ffi::c_void) -> usize;
    #[doc = "*Required features: `Win32_System_Memory`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn HeapSummary(hheap: super::super::Foundation::HANDLE, dwflags: u32, lpsummary: *mut HEAP_SUMMARY) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_System_Memory`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn HeapUnlock(hheap: HeapHandle) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_System_Memory`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn HeapValidate(hheap: HeapHandle, dwflags: HEAP_FLAGS, lpmem: *const ::core::ffi::c_void) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_System_Memory`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn HeapWalk(hheap: HeapHandle, lpentry: *mut PROCESS_HEAP_ENTRY) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_System_Memory`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn IsBadCodePtr(lpfn: ::windows::runtime::RawPtr) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_System_Memory`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn IsBadReadPtr(lp: *const ::core::ffi::c_void, ucb: usize) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_System_Memory`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn IsBadStringPtrA(lpsz: super::super::Foundation::PSTR, ucchmax: usize) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_System_Memory`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn IsBadStringPtrW(lpsz: super::super::Foundation::PWSTR, ucchmax: usize) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_System_Memory`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn IsBadWritePtr(lp: *const ::core::ffi::c_void, ucb: usize) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_System_Memory`*"]
    pub fn LocalAlloc(uflags: LOCAL_ALLOC_FLAGS, ubytes: usize) -> isize;
    #[doc = "*Required features: `Win32_System_Memory`*"]
    pub fn LocalFlags(hmem: isize) -> u32;
    #[doc = "*Required features: `Win32_System_Memory`*"]
    pub fn LocalFree(hmem: isize) -> isize;
    #[doc = "*Required features: `Win32_System_Memory`*"]
    pub fn LocalHandle(pmem: *const ::core::ffi::c_void) -> isize;
    #[doc = "*Required features: `Win32_System_Memory`*"]
    pub fn LocalLock(hmem: isize) -> *mut ::core::ffi::c_void;
    #[doc = "*Required features: `Win32_System_Memory`*"]
    pub fn LocalReAlloc(hmem: isize, ubytes: usize, uflags: u32) -> isize;
    #[doc = "*Required features: `Win32_System_Memory`*"]
    pub fn LocalSize(hmem: isize) -> usize;
    #[doc = "*Required features: `Win32_System_Memory`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn LocalUnlock(hmem: isize) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_System_Memory`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn MapUserPhysicalPages(virtualaddress: *const ::core::ffi::c_void, numberofpages: usize, pagearray: *const usize) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_System_Memory`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn MapUserPhysicalPagesScatter(virtualaddresses: *const *const ::core::ffi::c_void, numberofpages: usize, pagearray: *const usize) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_System_Memory`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn MapViewOfFile(hfilemappingobject: super::super::Foundation::HANDLE, dwdesiredaccess: FILE_MAP, dwfileoffsethigh: u32, dwfileoffsetlow: u32, dwnumberofbytestomap: usize) -> *mut ::core::ffi::c_void;
    #[doc = "*Required features: `Win32_System_Memory`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn MapViewOfFile3(filemapping: super::super::Foundation::HANDLE, process: super::super::Foundation::HANDLE, baseaddress: *const ::core::ffi::c_void, offset: u64, viewsize: usize, allocationtype: VIRTUAL_ALLOCATION_TYPE, pageprotection: u32, extendedparameters: *mut MEM_EXTENDED_PARAMETER, parametercount: u32) -> *mut ::core::ffi::c_void;
    #[doc = "*Required features: `Win32_System_Memory`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn MapViewOfFile3FromApp(filemapping: super::super::Foundation::HANDLE, process: super::super::Foundation::HANDLE, baseaddress: *const ::core::ffi::c_void, offset: u64, viewsize: usize, allocationtype: VIRTUAL_ALLOCATION_TYPE, pageprotection: u32, extendedparameters: *mut MEM_EXTENDED_PARAMETER, parametercount: u32) -> *mut ::core::ffi::c_void;
    #[doc = "*Required features: `Win32_System_Memory`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn MapViewOfFileEx(hfilemappingobject: super::super::Foundation::HANDLE, dwdesiredaccess: FILE_MAP, dwfileoffsethigh: u32, dwfileoffsetlow: u32, dwnumberofbytestomap: usize, lpbaseaddress: *const ::core::ffi::c_void) -> *mut ::core::ffi::c_void;
    #[doc = "*Required features: `Win32_System_Memory`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn MapViewOfFileExNuma(hfilemappingobject: super::super::Foundation::HANDLE, dwdesiredaccess: FILE_MAP, dwfileoffsethigh: u32, dwfileoffsetlow: u32, dwnumberofbytestomap: usize, lpbaseaddress: *const ::core::ffi::c_void, nndpreferred: u32) -> *mut ::core::ffi::c_void;
    #[doc = "*Required features: `Win32_System_Memory`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn MapViewOfFileFromApp(hfilemappingobject: super::super::Foundation::HANDLE, desiredaccess: FILE_MAP, fileoffset: u64, numberofbytestomap: usize) -> *mut ::core::ffi::c_void;
    #[doc = "*Required features: `Win32_System_Memory`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn MapViewOfFileNuma2(filemappinghandle: super::super::Foundation::HANDLE, processhandle: super::super::Foundation::HANDLE, offset: u64, baseaddress: *const ::core::ffi::c_void, viewsize: usize, allocationtype: u32, pageprotection: u32, preferrednode: u32) -> *mut ::core::ffi::c_void;
    #[doc = "*Required features: `Win32_System_Memory`*"]
    pub fn OfferVirtualMemory(virtualaddress: *mut ::core::ffi::c_void, size: usize, priority: OFFER_PRIORITY) -> u32;
    #[doc = "*Required features: `Win32_System_Memory`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn OpenDedicatedMemoryPartition(partition: super::super::Foundation::HANDLE, dedicatedmemorytypeid: u64, desiredaccess: u32, inherithandle: super::super::Foundation::BOOL) -> super::super::Foundation::HANDLE;
    #[doc = "*Required features: `Win32_System_Memory`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn OpenFileMappingA(dwdesiredaccess: u32, binherithandle: super::super::Foundation::BOOL, lpname: super::super::Foundation::PSTR) -> super::super::Foundation::HANDLE;
    #[doc = "*Required features: `Win32_System_Memory`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn OpenFileMappingFromApp(desiredaccess: u32, inherithandle: super::super::Foundation::BOOL, name: super::super::Foundation::PWSTR) -> super::super::Foundation::HANDLE;
    #[doc = "*Required features: `Win32_System_Memory`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn OpenFileMappingW(dwdesiredaccess: u32, binherithandle: super::super::Foundation::BOOL, lpname: super::super::Foundation::PWSTR) -> super::super::Foundation::HANDLE;
    #[doc = "*Required features: `Win32_System_Memory`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn PrefetchVirtualMemory(hprocess: super::super::Foundation::HANDLE, numberofentries: usize, virtualaddresses: *const WIN32_MEMORY_RANGE_ENTRY, flags: u32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_System_Memory`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn QueryMemoryResourceNotification(resourcenotificationhandle: super::super::Foundation::HANDLE, resourcestate: *mut super::super::Foundation::BOOL) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_System_Memory`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn QueryPartitionInformation(partition: super::super::Foundation::HANDLE, partitioninformationclass: WIN32_MEMORY_PARTITION_INFORMATION_CLASS, partitioninformation: *mut ::core::ffi::c_void, partitioninformationlength: u32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_System_Memory`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn QueryVirtualMemoryInformation(process: super::super::Foundation::HANDLE, virtualaddress: *const ::core::ffi::c_void, memoryinformationclass: WIN32_MEMORY_INFORMATION_CLASS, memoryinformation: *mut ::core::ffi::c_void, memoryinformationsize: usize, returnsize: *mut usize) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_System_Memory`*"]
    pub fn ReclaimVirtualMemory(virtualaddress: *const ::core::ffi::c_void, size: usize) -> u32;
    #[doc = "*Required features: `Win32_System_Memory`*"]
    pub fn RegisterBadMemoryNotification(callback: ::windows::runtime::RawPtr) -> *mut ::core::ffi::c_void;
    #[doc = "*Required features: `Win32_System_Memory`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn RemoveSecureMemoryCacheCallback(pfncallback: ::windows::runtime::RawPtr) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_System_Memory`*"]
    pub fn ResetWriteWatch(lpbaseaddress: *const ::core::ffi::c_void, dwregionsize: usize) -> u32;
    #[doc = "*Required features: `Win32_System_Memory`*"]
    pub fn RtlCompareMemory(source1: *const ::core::ffi::c_void, source2: *const ::core::ffi::c_void, length: usize) -> usize;
    #[doc = "*Required features: `Win32_System_Memory`*"]
    pub fn RtlCrc32(buffer: *const ::core::ffi::c_void, size: usize, initialcrc: u32) -> u32;
    #[doc = "*Required features: `Win32_System_Memory`*"]
    pub fn RtlCrc64(buffer: *const ::core::ffi::c_void, size: usize, initialcrc: u64) -> u64;
    #[doc = "*Required features: `Win32_System_Memory`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn RtlIsZeroMemory(buffer: *const ::core::ffi::c_void, length: usize) -> super::super::Foundation::BOOLEAN;
    #[doc = "*Required features: `Win32_System_Memory`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetProcessValidCallTargets(hprocess: super::super::Foundation::HANDLE, virtualaddress: *const ::core::ffi::c_void, regionsize: usize, numberofoffsets: u32, offsetinformation: *mut CFG_CALL_TARGET_INFO) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_System_Memory`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetProcessValidCallTargetsForMappedView(process: super::super::Foundation::HANDLE, virtualaddress: *const ::core::ffi::c_void, regionsize: usize, numberofoffsets: u32, offsetinformation: *mut CFG_CALL_TARGET_INFO, section: super::super::Foundation::HANDLE, expectedfileoffset: u64) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_System_Memory`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetProcessWorkingSetSizeEx(hprocess: super::super::Foundation::HANDLE, dwminimumworkingsetsize: usize, dwmaximumworkingsetsize: usize, flags: u32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_System_Memory`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetSystemFileCacheSize(minimumfilecachesize: usize, maximumfilecachesize: usize, flags: u32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_System_Memory`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn UnmapViewOfFile(lpbaseaddress: *const ::core::ffi::c_void) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_System_Memory`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn UnmapViewOfFile2(process: super::super::Foundation::HANDLE, baseaddress: *const ::core::ffi::c_void, unmapflags: UNMAP_VIEW_OF_FILE_FLAGS) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_System_Memory`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn UnmapViewOfFileEx(baseaddress: *const ::core::ffi::c_void, unmapflags: UNMAP_VIEW_OF_FILE_FLAGS) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_System_Memory`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn UnregisterBadMemoryNotification(registrationhandle: *const ::core::ffi::c_void) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_System_Memory`*"]
    pub fn VirtualAlloc(lpaddress: *const ::core::ffi::c_void, dwsize: usize, flallocationtype: VIRTUAL_ALLOCATION_TYPE, flprotect: PAGE_PROTECTION_FLAGS) -> *mut ::core::ffi::c_void;
    #[doc = "*Required features: `Win32_System_Memory`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn VirtualAlloc2(process: super::super::Foundation::HANDLE, baseaddress: *const ::core::ffi::c_void, size: usize, allocationtype: VIRTUAL_ALLOCATION_TYPE, pageprotection: u32, extendedparameters: *mut MEM_EXTENDED_PARAMETER, parametercount: u32) -> *mut ::core::ffi::c_void;
    #[doc = "*Required features: `Win32_System_Memory`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn VirtualAlloc2FromApp(process: super::super::Foundation::HANDLE, baseaddress: *const ::core::ffi::c_void, size: usize, allocationtype: VIRTUAL_ALLOCATION_TYPE, pageprotection: u32, extendedparameters: *mut MEM_EXTENDED_PARAMETER, parametercount: u32) -> *mut ::core::ffi::c_void;
    #[doc = "*Required features: `Win32_System_Memory`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn VirtualAllocEx(hprocess: super::super::Foundation::HANDLE, lpaddress: *const ::core::ffi::c_void, dwsize: usize, flallocationtype: VIRTUAL_ALLOCATION_TYPE, flprotect: PAGE_PROTECTION_FLAGS) -> *mut ::core::ffi::c_void;
    #[doc = "*Required features: `Win32_System_Memory`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn VirtualAllocExNuma(hprocess: super::super::Foundation::HANDLE, lpaddress: *const ::core::ffi::c_void, dwsize: usize, flallocationtype: VIRTUAL_ALLOCATION_TYPE, flprotect: u32, nndpreferred: u32) -> *mut ::core::ffi::c_void;
    #[doc = "*Required features: `Win32_System_Memory`*"]
    pub fn VirtualAllocFromApp(baseaddress: *const ::core::ffi::c_void, size: usize, allocationtype: VIRTUAL_ALLOCATION_TYPE, protection: u32) -> *mut ::core::ffi::c_void;
    #[doc = "*Required features: `Win32_System_Memory`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn VirtualFree(lpaddress: *mut ::core::ffi::c_void, dwsize: usize, dwfreetype: VIRTUAL_FREE_TYPE) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_System_Memory`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn VirtualFreeEx(hprocess: super::super::Foundation::HANDLE, lpaddress: *mut ::core::ffi::c_void, dwsize: usize, dwfreetype: VIRTUAL_FREE_TYPE) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_System_Memory`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn VirtualLock(lpaddress: *const ::core::ffi::c_void, dwsize: usize) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_System_Memory`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn VirtualProtect(lpaddress: *const ::core::ffi::c_void, dwsize: usize, flnewprotect: PAGE_PROTECTION_FLAGS, lpfloldprotect: *mut PAGE_PROTECTION_FLAGS) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_System_Memory`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn VirtualProtectEx(hprocess: super::super::Foundation::HANDLE, lpaddress: *const ::core::ffi::c_void, dwsize: usize, flnewprotect: PAGE_PROTECTION_FLAGS, lpfloldprotect: *mut PAGE_PROTECTION_FLAGS) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_System_Memory`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn VirtualProtectFromApp(address: *const ::core::ffi::c_void, size: usize, newprotection: u32, oldprotection: *mut u32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_System_Memory`*"]
    pub fn VirtualQuery(lpaddress: *const ::core::ffi::c_void, lpbuffer: *mut MEMORY_BASIC_INFORMATION, dwlength: usize) -> usize;
    #[doc = "*Required features: `Win32_System_Memory`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn VirtualQueryEx(hprocess: super::super::Foundation::HANDLE, lpaddress: *const ::core::ffi::c_void, lpbuffer: *mut MEMORY_BASIC_INFORMATION, dwlength: usize) -> usize;
    #[doc = "*Required features: `Win32_System_Memory`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn VirtualUnlock(lpaddress: *const ::core::ffi::c_void, dwsize: usize) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_System_Memory`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn VirtualUnlockEx(process: super::super::Foundation::HANDLE, address: *const ::core::ffi::c_void, size: usize) -> super::super::Foundation::BOOL;
}

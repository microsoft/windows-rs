#![allow(non_snake_case, non_camel_case_types)]
#[cfg(feature = "Win32_System_Memory_NonVolatile")]
pub mod NonVolatile;
#[link(name = "windows")]
extern "system" {
    #[doc = "*Required features: `Win32_System_Memory`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn AddSecureMemoryCacheCallback();
    #[doc = "*Required features: `Win32_System_Memory`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn AllocateUserPhysicalPages();
    #[doc = "*Required features: `Win32_System_Memory`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn AllocateUserPhysicalPages2();
    #[doc = "*Required features: `Win32_System_Memory`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn AllocateUserPhysicalPagesNuma();
    #[doc = "*Required features: `Win32_System_Memory`, `Win32_Foundation`, `Win32_Security`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
    pub fn CreateFileMapping2();
    #[doc = "*Required features: `Win32_System_Memory`, `Win32_Foundation`, `Win32_Security`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
    pub fn CreateFileMappingA();
    #[doc = "*Required features: `Win32_System_Memory`, `Win32_Foundation`, `Win32_Security`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
    pub fn CreateFileMappingFromApp();
    #[doc = "*Required features: `Win32_System_Memory`, `Win32_Foundation`, `Win32_Security`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
    pub fn CreateFileMappingNumaA();
    #[doc = "*Required features: `Win32_System_Memory`, `Win32_Foundation`, `Win32_Security`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
    pub fn CreateFileMappingNumaW();
    #[doc = "*Required features: `Win32_System_Memory`, `Win32_Foundation`, `Win32_Security`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
    pub fn CreateFileMappingW();
    #[doc = "*Required features: `Win32_System_Memory`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CreateMemoryResourceNotification();
    #[doc = "*Required features: `Win32_System_Memory`*"]
    pub fn DiscardVirtualMemory();
    #[doc = "*Required features: `Win32_System_Memory`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn FlushViewOfFile();
    #[doc = "*Required features: `Win32_System_Memory`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn FreeUserPhysicalPages();
    #[doc = "*Required features: `Win32_System_Memory`*"]
    pub fn GetLargePageMinimum();
    #[doc = "*Required features: `Win32_System_Memory`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetMemoryErrorHandlingCapabilities();
    #[doc = "*Required features: `Win32_System_Memory`*"]
    pub fn GetProcessHeap();
    #[doc = "*Required features: `Win32_System_Memory`*"]
    pub fn GetProcessHeaps();
    #[doc = "*Required features: `Win32_System_Memory`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetProcessWorkingSetSizeEx();
    #[doc = "*Required features: `Win32_System_Memory`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetSystemFileCacheSize();
    #[doc = "*Required features: `Win32_System_Memory`*"]
    pub fn GetWriteWatch();
    #[doc = "*Required features: `Win32_System_Memory`*"]
    pub fn GlobalAlloc();
    #[doc = "*Required features: `Win32_System_Memory`*"]
    pub fn GlobalFlags();
    #[doc = "*Required features: `Win32_System_Memory`*"]
    pub fn GlobalFree();
    #[doc = "*Required features: `Win32_System_Memory`*"]
    pub fn GlobalHandle();
    #[doc = "*Required features: `Win32_System_Memory`*"]
    pub fn GlobalLock();
    #[doc = "*Required features: `Win32_System_Memory`*"]
    pub fn GlobalReAlloc();
    #[doc = "*Required features: `Win32_System_Memory`*"]
    pub fn GlobalSize();
    #[doc = "*Required features: `Win32_System_Memory`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GlobalUnlock();
    #[doc = "*Required features: `Win32_System_Memory`*"]
    pub fn HeapAlloc();
    #[doc = "*Required features: `Win32_System_Memory`*"]
    pub fn HeapCompact();
    #[doc = "*Required features: `Win32_System_Memory`*"]
    pub fn HeapCreate();
    #[doc = "*Required features: `Win32_System_Memory`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn HeapDestroy();
    #[doc = "*Required features: `Win32_System_Memory`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn HeapFree();
    #[doc = "*Required features: `Win32_System_Memory`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn HeapLock();
    #[doc = "*Required features: `Win32_System_Memory`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn HeapQueryInformation();
    #[doc = "*Required features: `Win32_System_Memory`*"]
    pub fn HeapReAlloc();
    #[doc = "*Required features: `Win32_System_Memory`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn HeapSetInformation();
    #[doc = "*Required features: `Win32_System_Memory`*"]
    pub fn HeapSize();
    #[doc = "*Required features: `Win32_System_Memory`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn HeapSummary();
    #[doc = "*Required features: `Win32_System_Memory`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn HeapUnlock();
    #[doc = "*Required features: `Win32_System_Memory`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn HeapValidate();
    #[doc = "*Required features: `Win32_System_Memory`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn HeapWalk();
    #[doc = "*Required features: `Win32_System_Memory`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn IsBadCodePtr();
    #[doc = "*Required features: `Win32_System_Memory`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn IsBadReadPtr();
    #[doc = "*Required features: `Win32_System_Memory`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn IsBadStringPtrA();
    #[doc = "*Required features: `Win32_System_Memory`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn IsBadStringPtrW();
    #[doc = "*Required features: `Win32_System_Memory`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn IsBadWritePtr();
    #[doc = "*Required features: `Win32_System_Memory`*"]
    pub fn LocalAlloc();
    #[doc = "*Required features: `Win32_System_Memory`*"]
    pub fn LocalFlags();
    #[doc = "*Required features: `Win32_System_Memory`*"]
    pub fn LocalFree();
    #[doc = "*Required features: `Win32_System_Memory`*"]
    pub fn LocalHandle();
    #[doc = "*Required features: `Win32_System_Memory`*"]
    pub fn LocalLock();
    #[doc = "*Required features: `Win32_System_Memory`*"]
    pub fn LocalReAlloc();
    #[doc = "*Required features: `Win32_System_Memory`*"]
    pub fn LocalSize();
    #[doc = "*Required features: `Win32_System_Memory`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn LocalUnlock();
    #[doc = "*Required features: `Win32_System_Memory`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn MapUserPhysicalPages();
    #[doc = "*Required features: `Win32_System_Memory`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn MapUserPhysicalPagesScatter();
    #[doc = "*Required features: `Win32_System_Memory`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn MapViewOfFile();
    #[doc = "*Required features: `Win32_System_Memory`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn MapViewOfFile3();
    #[doc = "*Required features: `Win32_System_Memory`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn MapViewOfFile3FromApp();
    #[doc = "*Required features: `Win32_System_Memory`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn MapViewOfFileEx();
    #[doc = "*Required features: `Win32_System_Memory`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn MapViewOfFileExNuma();
    #[doc = "*Required features: `Win32_System_Memory`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn MapViewOfFileFromApp();
    #[doc = "*Required features: `Win32_System_Memory`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn MapViewOfFileNuma2();
    #[doc = "*Required features: `Win32_System_Memory`*"]
    pub fn OfferVirtualMemory();
    #[doc = "*Required features: `Win32_System_Memory`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn OpenDedicatedMemoryPartition();
    #[doc = "*Required features: `Win32_System_Memory`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn OpenFileMappingA();
    #[doc = "*Required features: `Win32_System_Memory`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn OpenFileMappingFromApp();
    #[doc = "*Required features: `Win32_System_Memory`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn OpenFileMappingW();
    #[doc = "*Required features: `Win32_System_Memory`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn PrefetchVirtualMemory();
    #[doc = "*Required features: `Win32_System_Memory`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn QueryMemoryResourceNotification();
    #[doc = "*Required features: `Win32_System_Memory`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn QueryPartitionInformation();
    #[doc = "*Required features: `Win32_System_Memory`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn QueryVirtualMemoryInformation();
    #[doc = "*Required features: `Win32_System_Memory`*"]
    pub fn ReclaimVirtualMemory();
    #[doc = "*Required features: `Win32_System_Memory`*"]
    pub fn RegisterBadMemoryNotification();
    #[doc = "*Required features: `Win32_System_Memory`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn RemoveSecureMemoryCacheCallback();
    #[doc = "*Required features: `Win32_System_Memory`*"]
    pub fn ResetWriteWatch();
    #[doc = "*Required features: `Win32_System_Memory`*"]
    pub fn RtlCompareMemory();
    #[doc = "*Required features: `Win32_System_Memory`*"]
    pub fn RtlCrc32();
    #[doc = "*Required features: `Win32_System_Memory`*"]
    pub fn RtlCrc64();
    #[doc = "*Required features: `Win32_System_Memory`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn RtlIsZeroMemory();
    #[doc = "*Required features: `Win32_System_Memory`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetProcessValidCallTargets();
    #[doc = "*Required features: `Win32_System_Memory`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetProcessValidCallTargetsForMappedView();
    #[doc = "*Required features: `Win32_System_Memory`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetProcessWorkingSetSizeEx();
    #[doc = "*Required features: `Win32_System_Memory`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetSystemFileCacheSize();
    #[doc = "*Required features: `Win32_System_Memory`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn UnmapViewOfFile();
    #[doc = "*Required features: `Win32_System_Memory`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn UnmapViewOfFile2();
    #[doc = "*Required features: `Win32_System_Memory`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn UnmapViewOfFileEx();
    #[doc = "*Required features: `Win32_System_Memory`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn UnregisterBadMemoryNotification();
    #[doc = "*Required features: `Win32_System_Memory`*"]
    pub fn VirtualAlloc();
    #[doc = "*Required features: `Win32_System_Memory`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn VirtualAlloc2();
    #[doc = "*Required features: `Win32_System_Memory`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn VirtualAlloc2FromApp();
    #[doc = "*Required features: `Win32_System_Memory`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn VirtualAllocEx();
    #[doc = "*Required features: `Win32_System_Memory`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn VirtualAllocExNuma();
    #[doc = "*Required features: `Win32_System_Memory`*"]
    pub fn VirtualAllocFromApp();
    #[doc = "*Required features: `Win32_System_Memory`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn VirtualFree();
    #[doc = "*Required features: `Win32_System_Memory`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn VirtualFreeEx();
    #[doc = "*Required features: `Win32_System_Memory`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn VirtualLock();
    #[doc = "*Required features: `Win32_System_Memory`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn VirtualProtect();
    #[doc = "*Required features: `Win32_System_Memory`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn VirtualProtectEx();
    #[doc = "*Required features: `Win32_System_Memory`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn VirtualProtectFromApp();
    #[doc = "*Required features: `Win32_System_Memory`*"]
    pub fn VirtualQuery();
    #[doc = "*Required features: `Win32_System_Memory`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn VirtualQueryEx();
    #[doc = "*Required features: `Win32_System_Memory`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn VirtualUnlock();
    #[doc = "*Required features: `Win32_System_Memory`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn VirtualUnlockEx();
}

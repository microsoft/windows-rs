#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn GetProcessHeap() -> super::HANDLE {
    windows_core::link!("kernel32.dll" "system" fn GetProcessHeap() -> super::HANDLE);
    unsafe { GetProcessHeap() }
}
#[cfg(all(feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn GetProcessHeaps(numberofheaps: super::DWORD, processheaps: super::PHANDLE) -> super::DWORD {
    windows_core::link!("kernel32.dll" "system" fn GetProcessHeaps(numberofheaps : super::DWORD, processheaps : super::PHANDLE) -> super::DWORD);
    unsafe { GetProcessHeaps(numberofheaps, processheaps as _) }
}
#[cfg(all(feature = "minwindef", feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn HeapAlloc(hheap: super::HANDLE, dwflags: super::DWORD, dwbytes: super::SIZE_T) -> super::LPVOID {
    windows_core::link!("kernel32.dll" "system" fn HeapAlloc(hheap : super::HANDLE, dwflags : super::DWORD, dwbytes : super::SIZE_T) -> super::LPVOID);
    unsafe { HeapAlloc(hheap, dwflags, dwbytes) }
}
#[cfg(all(feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn HeapCompact(hheap: super::HANDLE, dwflags: super::DWORD) -> super::SIZE_T {
    windows_core::link!("kernel32.dll" "system" fn HeapCompact(hheap : super::HANDLE, dwflags : super::DWORD) -> super::SIZE_T);
    unsafe { HeapCompact(hheap, dwflags) }
}
#[cfg(all(feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn HeapCreate(floptions: super::DWORD, dwinitialsize: super::SIZE_T, dwmaximumsize: super::SIZE_T) -> super::HANDLE {
    windows_core::link!("kernel32.dll" "system" fn HeapCreate(floptions : super::DWORD, dwinitialsize : super::SIZE_T, dwmaximumsize : super::SIZE_T) -> super::HANDLE);
    unsafe { HeapCreate(floptions, dwinitialsize, dwmaximumsize) }
}
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn HeapDestroy(hheap: super::HANDLE) -> windows_core::BOOL {
    windows_core::link!("kernel32.dll" "system" fn HeapDestroy(hheap : super::HANDLE) -> windows_core::BOOL);
    unsafe { HeapDestroy(hheap) }
}
#[cfg(all(feature = "minwindef", feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn HeapFree(hheap: super::HANDLE, dwflags: super::DWORD, lpmem: super::LPVOID) -> windows_core::BOOL {
    windows_core::link!("kernel32.dll" "system" fn HeapFree(hheap : super::HANDLE, dwflags : super::DWORD, lpmem : super::LPVOID) -> windows_core::BOOL);
    unsafe { HeapFree(hheap as _, dwflags, lpmem as _) }
}
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn HeapLock(hheap: super::HANDLE) -> windows_core::BOOL {
    windows_core::link!("kernel32.dll" "system" fn HeapLock(hheap : super::HANDLE) -> windows_core::BOOL);
    unsafe { HeapLock(hheap) }
}
#[cfg(all(feature = "basetsd", feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn HeapQueryInformation(heaphandle: Option<super::HANDLE>, heapinformationclass: super::HEAP_INFORMATION_CLASS, heapinformation: Option<super::PVOID>, heapinformationlength: super::SIZE_T, returnlength: Option<super::PSIZE_T>) -> windows_core::BOOL {
    windows_core::link!("kernel32.dll" "system" fn HeapQueryInformation(heaphandle : super::HANDLE, heapinformationclass : super::HEAP_INFORMATION_CLASS, heapinformation : super::PVOID, heapinformationlength : super::SIZE_T, returnlength : super::PSIZE_T) -> windows_core::BOOL);
    unsafe { HeapQueryInformation(heaphandle.unwrap_or(core::mem::zeroed()) as _, heapinformationclass, heapinformation.unwrap_or(core::mem::zeroed()) as _, heapinformationlength, returnlength.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(all(feature = "minwindef", feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn HeapReAlloc(hheap: super::HANDLE, dwflags: super::DWORD, lpmem: super::LPVOID, dwbytes: super::SIZE_T) -> super::LPVOID {
    windows_core::link!("kernel32.dll" "system" fn HeapReAlloc(hheap : super::HANDLE, dwflags : super::DWORD, lpmem : super::LPVOID, dwbytes : super::SIZE_T) -> super::LPVOID);
    unsafe { HeapReAlloc(hheap as _, dwflags, lpmem as _, dwbytes) }
}
#[cfg(all(feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn HeapSetInformation(heaphandle: Option<super::HANDLE>, heapinformationclass: super::HEAP_INFORMATION_CLASS, heapinformation: Option<super::PVOID>, heapinformationlength: super::SIZE_T) -> windows_core::BOOL {
    windows_core::link!("kernel32.dll" "system" fn HeapSetInformation(heaphandle : super::HANDLE, heapinformationclass : super::HEAP_INFORMATION_CLASS, heapinformation : super::PVOID, heapinformationlength : super::SIZE_T) -> windows_core::BOOL);
    unsafe { HeapSetInformation(heaphandle.unwrap_or(core::mem::zeroed()) as _, heapinformationclass, heapinformation.unwrap_or(core::mem::zeroed()) as _, heapinformationlength) }
}
#[cfg(all(feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn HeapSize(hheap: super::HANDLE, dwflags: super::DWORD, lpmem: super::LPCVOID) -> super::SIZE_T {
    windows_core::link!("kernel32.dll" "system" fn HeapSize(hheap : super::HANDLE, dwflags : super::DWORD, lpmem : super::LPCVOID) -> super::SIZE_T);
    unsafe { HeapSize(hheap, dwflags, lpmem) }
}
#[cfg(all(feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn HeapSummary(hheap: super::HANDLE, dwflags: super::DWORD, lpsummary: LPHEAP_SUMMARY) -> windows_core::BOOL {
    windows_core::link!("kernel32.dll" "system" fn HeapSummary(hheap : super::HANDLE, dwflags : super::DWORD, lpsummary : LPHEAP_SUMMARY) -> windows_core::BOOL);
    unsafe { HeapSummary(hheap, dwflags, lpsummary as _) }
}
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn HeapUnlock(hheap: super::HANDLE) -> windows_core::BOOL {
    windows_core::link!("kernel32.dll" "system" fn HeapUnlock(hheap : super::HANDLE) -> windows_core::BOOL);
    unsafe { HeapUnlock(hheap) }
}
#[cfg(all(feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn HeapValidate(hheap: super::HANDLE, dwflags: super::DWORD, lpmem: Option<super::LPCVOID>) -> windows_core::BOOL {
    windows_core::link!("kernel32.dll" "system" fn HeapValidate(hheap : super::HANDLE, dwflags : super::DWORD, lpmem : super::LPCVOID) -> windows_core::BOOL);
    unsafe { HeapValidate(hheap, dwflags, lpmem.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(all(feature = "minwinbase", feature = "minwindef", feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn HeapWalk(hheap: super::HANDLE, lpentry: super::LPPROCESS_HEAP_ENTRY) -> windows_core::BOOL {
    windows_core::link!("kernel32.dll" "system" fn HeapWalk(hheap : super::HANDLE, lpentry : super::LPPROCESS_HEAP_ENTRY) -> windows_core::BOOL);
    unsafe { HeapWalk(hheap, lpentry as _) }
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct HEAP_SUMMARY {
    pub cb: super::DWORD,
    pub cbAllocated: super::SIZE_T,
    pub cbCommitted: super::SIZE_T,
    pub cbReserved: super::SIZE_T,
    pub cbMaxReserve: super::SIZE_T,
}
#[cfg(feature = "shared")]
pub type LPHEAP_SUMMARY = *mut HEAP_SUMMARY;
#[cfg(feature = "shared")]
pub type PHEAP_SUMMARY = *mut HEAP_SUMMARY;

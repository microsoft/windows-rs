#[cfg(feature = "winnt")]
windows_link::link!("kernel32.dll" "system" fn GetProcessHeap() -> super::HANDLE);
#[cfg(all(feature = "shared", feature = "winnt"))]
windows_link::link!("kernel32.dll" "system" fn GetProcessHeaps(numberofheaps : super::DWORD, processheaps : super::PHANDLE) -> super::DWORD);
#[cfg(all(feature = "minwindef", feature = "shared", feature = "winnt"))]
windows_link::link!("kernel32.dll" "system" fn HeapAlloc(hheap : super::HANDLE, dwflags : super::DWORD, dwbytes : super::SIZE_T) -> super::LPVOID);
#[cfg(all(feature = "shared", feature = "winnt"))]
windows_link::link!("kernel32.dll" "system" fn HeapCompact(hheap : super::HANDLE, dwflags : super::DWORD) -> super::SIZE_T);
#[cfg(all(feature = "shared", feature = "winnt"))]
windows_link::link!("kernel32.dll" "system" fn HeapCreate(floptions : super::DWORD, dwinitialsize : super::SIZE_T, dwmaximumsize : super::SIZE_T) -> super::HANDLE);
#[cfg(feature = "winnt")]
windows_link::link!("kernel32.dll" "system" fn HeapDestroy(hheap : super::HANDLE) -> windows_sys::core::BOOL);
#[cfg(all(feature = "minwindef", feature = "shared", feature = "winnt"))]
windows_link::link!("kernel32.dll" "system" fn HeapFree(hheap : super::HANDLE, dwflags : super::DWORD, lpmem : super::LPVOID) -> windows_sys::core::BOOL);
#[cfg(feature = "winnt")]
windows_link::link!("kernel32.dll" "system" fn HeapLock(hheap : super::HANDLE) -> windows_sys::core::BOOL);
#[cfg(all(feature = "basetsd", feature = "shared", feature = "winnt"))]
windows_link::link!("kernel32.dll" "system" fn HeapQueryInformation(heaphandle : super::HANDLE, heapinformationclass : super::HEAP_INFORMATION_CLASS, heapinformation : super::PVOID, heapinformationlength : super::SIZE_T, returnlength : super::PSIZE_T) -> windows_sys::core::BOOL);
#[cfg(all(feature = "minwindef", feature = "shared", feature = "winnt"))]
windows_link::link!("kernel32.dll" "system" fn HeapReAlloc(hheap : super::HANDLE, dwflags : super::DWORD, lpmem : super::LPVOID, dwbytes : super::SIZE_T) -> super::LPVOID);
#[cfg(all(feature = "shared", feature = "winnt"))]
windows_link::link!("kernel32.dll" "system" fn HeapSetInformation(heaphandle : super::HANDLE, heapinformationclass : super::HEAP_INFORMATION_CLASS, heapinformation : super::PVOID, heapinformationlength : super::SIZE_T) -> windows_sys::core::BOOL);
#[cfg(all(feature = "shared", feature = "winnt"))]
windows_link::link!("kernel32.dll" "system" fn HeapSize(hheap : super::HANDLE, dwflags : super::DWORD, lpmem : super::LPCVOID) -> super::SIZE_T);
#[cfg(all(feature = "shared", feature = "winnt"))]
windows_link::link!("kernel32.dll" "system" fn HeapSummary(hheap : super::HANDLE, dwflags : super::DWORD, lpsummary : LPHEAP_SUMMARY) -> windows_sys::core::BOOL);
#[cfg(feature = "winnt")]
windows_link::link!("kernel32.dll" "system" fn HeapUnlock(hheap : super::HANDLE) -> windows_sys::core::BOOL);
#[cfg(all(feature = "shared", feature = "winnt"))]
windows_link::link!("kernel32.dll" "system" fn HeapValidate(hheap : super::HANDLE, dwflags : super::DWORD, lpmem : super::LPCVOID) -> windows_sys::core::BOOL);
#[cfg(all(feature = "minwinbase", feature = "minwindef", feature = "shared", feature = "winnt"))]
windows_link::link!("kernel32.dll" "system" fn HeapWalk(hheap : super::HANDLE, lpentry : super::LPPROCESS_HEAP_ENTRY) -> windows_sys::core::BOOL);
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Default)]
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

#[cfg(feature = "Win32_winnt")]
windows_link::link!("kernel32.dll" "system" fn GetProcessHeap() -> super::winnt::HANDLE);
#[cfg(feature = "Win32_winnt")]
windows_link::link!("kernel32.dll" "system" fn GetProcessHeaps(numberofheaps : u32, processheaps : *mut super::winnt::HANDLE) -> u32);
#[cfg(feature = "Win32_winnt")]
windows_link::link!("kernel32.dll" "system" fn HeapAlloc(hheap : super::winnt::HANDLE, dwflags : u32, dwbytes : usize) -> *mut core::ffi::c_void);
#[cfg(feature = "Win32_winnt")]
windows_link::link!("kernel32.dll" "system" fn HeapCompact(hheap : super::winnt::HANDLE, dwflags : u32) -> usize);
#[cfg(feature = "Win32_winnt")]
windows_link::link!("kernel32.dll" "system" fn HeapCreate(floptions : u32, dwinitialsize : usize, dwmaximumsize : usize) -> super::winnt::HANDLE);
#[cfg(feature = "Win32_winnt")]
windows_link::link!("kernel32.dll" "system" fn HeapDestroy(hheap : super::winnt::HANDLE) -> windows_sys::core::BOOL);
#[cfg(feature = "Win32_winnt")]
windows_link::link!("kernel32.dll" "system" fn HeapFree(hheap : super::winnt::HANDLE, dwflags : u32, lpmem : *mut core::ffi::c_void) -> windows_sys::core::BOOL);
#[cfg(feature = "Win32_winnt")]
windows_link::link!("kernel32.dll" "system" fn HeapLock(hheap : super::winnt::HANDLE) -> windows_sys::core::BOOL);
#[cfg(feature = "Win32_winnt")]
windows_link::link!("kernel32.dll" "system" fn HeapQueryInformation(heaphandle : super::winnt::HANDLE, heapinformationclass : super::winnt::HEAP_INFORMATION_CLASS, heapinformation : *mut core::ffi::c_void, heapinformationlength : usize, returnlength : *mut usize) -> windows_sys::core::BOOL);
#[cfg(feature = "Win32_winnt")]
windows_link::link!("kernel32.dll" "system" fn HeapReAlloc(hheap : super::winnt::HANDLE, dwflags : u32, lpmem : *mut core::ffi::c_void, dwbytes : usize) -> *mut core::ffi::c_void);
#[cfg(feature = "Win32_winnt")]
windows_link::link!("kernel32.dll" "system" fn HeapSetInformation(heaphandle : super::winnt::HANDLE, heapinformationclass : super::winnt::HEAP_INFORMATION_CLASS, heapinformation : *const core::ffi::c_void, heapinformationlength : usize) -> windows_sys::core::BOOL);
#[cfg(feature = "Win32_winnt")]
windows_link::link!("kernel32.dll" "system" fn HeapSize(hheap : super::winnt::HANDLE, dwflags : u32, lpmem : *const core::ffi::c_void) -> usize);
#[cfg(feature = "Win32_winnt")]
windows_link::link!("kernel32.dll" "system" fn HeapSummary(hheap : super::winnt::HANDLE, dwflags : u32, lpsummary : LPHEAP_SUMMARY) -> windows_sys::core::BOOL);
#[cfg(feature = "Win32_winnt")]
windows_link::link!("kernel32.dll" "system" fn HeapUnlock(hheap : super::winnt::HANDLE) -> windows_sys::core::BOOL);
#[cfg(feature = "Win32_winnt")]
windows_link::link!("kernel32.dll" "system" fn HeapValidate(hheap : super::winnt::HANDLE, dwflags : u32, lpmem : *const core::ffi::c_void) -> windows_sys::core::BOOL);
#[cfg(all(feature = "Win32_minwinbase", feature = "Win32_winnt"))]
windows_link::link!("kernel32.dll" "system" fn HeapWalk(hheap : super::winnt::HANDLE, lpentry : *mut super::minwinbase::PROCESS_HEAP_ENTRY) -> windows_sys::core::BOOL);
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct HEAP_SUMMARY {
    pub cb: u32,
    pub cbAllocated: usize,
    pub cbCommitted: usize,
    pub cbReserved: usize,
    pub cbMaxReserve: usize,
}
pub type LPHEAP_SUMMARY = PHEAP_SUMMARY;
pub type PHEAP_SUMMARY = *mut HEAP_SUMMARY;

#[cfg(feature = "Win32_winnt")]
#[inline]
pub unsafe fn GetProcessHeap() -> super::winnt::HANDLE {
    windows_core::link!("kernel32.dll" "system" fn GetProcessHeap() -> super::winnt::HANDLE);
    unsafe { GetProcessHeap() }
}
#[cfg(feature = "Win32_winnt")]
#[inline]
pub unsafe fn GetProcessHeaps(processheaps: &mut [super::winnt::HANDLE]) -> u32 {
    windows_core::link!("kernel32.dll" "system" fn GetProcessHeaps(numberofheaps : u32, processheaps : *mut super::winnt::HANDLE) -> u32);
    unsafe { GetProcessHeaps(processheaps.len().try_into().unwrap(), core::mem::transmute(processheaps.as_ptr())) }
}
#[cfg(feature = "Win32_winnt")]
#[inline]
pub unsafe fn HeapAlloc(hheap: super::winnt::HANDLE, dwflags: u32, dwbytes: usize) -> *mut core::ffi::c_void {
    windows_core::link!("kernel32.dll" "system" fn HeapAlloc(hheap : super::winnt::HANDLE, dwflags : u32, dwbytes : usize) -> *mut core::ffi::c_void);
    unsafe { HeapAlloc(hheap, dwflags, dwbytes) }
}
#[cfg(feature = "Win32_winnt")]
#[inline]
pub unsafe fn HeapCompact(hheap: super::winnt::HANDLE, dwflags: u32) -> usize {
    windows_core::link!("kernel32.dll" "system" fn HeapCompact(hheap : super::winnt::HANDLE, dwflags : u32) -> usize);
    unsafe { HeapCompact(hheap, dwflags) }
}
#[cfg(feature = "Win32_winnt")]
#[inline]
pub unsafe fn HeapCreate(floptions: u32, dwinitialsize: usize, dwmaximumsize: usize) -> super::winnt::HANDLE {
    windows_core::link!("kernel32.dll" "system" fn HeapCreate(floptions : u32, dwinitialsize : usize, dwmaximumsize : usize) -> super::winnt::HANDLE);
    unsafe { HeapCreate(floptions, dwinitialsize, dwmaximumsize) }
}
#[cfg(feature = "Win32_winnt")]
#[inline]
pub unsafe fn HeapDestroy(hheap: super::winnt::HANDLE) -> windows_core::BOOL {
    windows_core::link!("kernel32.dll" "system" fn HeapDestroy(hheap : super::winnt::HANDLE) -> windows_core::BOOL);
    unsafe { HeapDestroy(hheap) }
}
#[cfg(feature = "Win32_winnt")]
#[inline]
pub unsafe fn HeapFree(hheap: super::winnt::HANDLE, dwflags: u32, lpmem: *mut core::ffi::c_void) -> windows_core::BOOL {
    windows_core::link!("kernel32.dll" "system" fn HeapFree(hheap : super::winnt::HANDLE, dwflags : u32, lpmem : *mut core::ffi::c_void) -> windows_core::BOOL);
    unsafe { HeapFree(hheap as _, dwflags, lpmem as _) }
}
#[cfg(feature = "Win32_winnt")]
#[inline]
pub unsafe fn HeapLock(hheap: super::winnt::HANDLE) -> windows_core::BOOL {
    windows_core::link!("kernel32.dll" "system" fn HeapLock(hheap : super::winnt::HANDLE) -> windows_core::BOOL);
    unsafe { HeapLock(hheap) }
}
#[cfg(feature = "Win32_winnt")]
#[inline]
pub unsafe fn HeapQueryInformation(heaphandle: Option<super::winnt::HANDLE>, heapinformationclass: super::winnt::HEAP_INFORMATION_CLASS, heapinformation: Option<*mut core::ffi::c_void>, heapinformationlength: usize, returnlength: Option<*mut usize>) -> windows_core::BOOL {
    windows_core::link!("kernel32.dll" "system" fn HeapQueryInformation(heaphandle : super::winnt::HANDLE, heapinformationclass : super::winnt::HEAP_INFORMATION_CLASS, heapinformation : *mut core::ffi::c_void, heapinformationlength : usize, returnlength : *mut usize) -> windows_core::BOOL);
    unsafe { HeapQueryInformation(heaphandle.unwrap_or(core::mem::zeroed()) as _, heapinformationclass, heapinformation.unwrap_or(core::mem::zeroed()) as _, heapinformationlength, returnlength.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(feature = "Win32_winnt")]
#[inline]
pub unsafe fn HeapReAlloc(hheap: super::winnt::HANDLE, dwflags: u32, lpmem: *mut core::ffi::c_void, dwbytes: usize) -> *mut core::ffi::c_void {
    windows_core::link!("kernel32.dll" "system" fn HeapReAlloc(hheap : super::winnt::HANDLE, dwflags : u32, lpmem : *mut core::ffi::c_void, dwbytes : usize) -> *mut core::ffi::c_void);
    unsafe { HeapReAlloc(hheap as _, dwflags, lpmem as _, dwbytes) }
}
#[cfg(feature = "Win32_winnt")]
#[inline]
pub unsafe fn HeapSetInformation(heaphandle: Option<super::winnt::HANDLE>, heapinformationclass: super::winnt::HEAP_INFORMATION_CLASS, heapinformation: Option<*const core::ffi::c_void>, heapinformationlength: usize) -> windows_core::BOOL {
    windows_core::link!("kernel32.dll" "system" fn HeapSetInformation(heaphandle : super::winnt::HANDLE, heapinformationclass : super::winnt::HEAP_INFORMATION_CLASS, heapinformation : *const core::ffi::c_void, heapinformationlength : usize) -> windows_core::BOOL);
    unsafe { HeapSetInformation(heaphandle.unwrap_or(core::mem::zeroed()) as _, heapinformationclass, heapinformation.unwrap_or(core::mem::zeroed()) as _, heapinformationlength) }
}
#[cfg(feature = "Win32_winnt")]
#[inline]
pub unsafe fn HeapSize(hheap: super::winnt::HANDLE, dwflags: u32, lpmem: *const core::ffi::c_void) -> usize {
    windows_core::link!("kernel32.dll" "system" fn HeapSize(hheap : super::winnt::HANDLE, dwflags : u32, lpmem : *const core::ffi::c_void) -> usize);
    unsafe { HeapSize(hheap, dwflags, lpmem) }
}
#[cfg(feature = "Win32_winnt")]
#[inline]
pub unsafe fn HeapSummary(hheap: super::winnt::HANDLE, dwflags: u32, lpsummary: LPHEAP_SUMMARY) -> windows_core::BOOL {
    windows_core::link!("kernel32.dll" "system" fn HeapSummary(hheap : super::winnt::HANDLE, dwflags : u32, lpsummary : LPHEAP_SUMMARY) -> windows_core::BOOL);
    unsafe { HeapSummary(hheap, dwflags, lpsummary as _) }
}
#[cfg(feature = "Win32_winnt")]
#[inline]
pub unsafe fn HeapUnlock(hheap: super::winnt::HANDLE) -> windows_core::BOOL {
    windows_core::link!("kernel32.dll" "system" fn HeapUnlock(hheap : super::winnt::HANDLE) -> windows_core::BOOL);
    unsafe { HeapUnlock(hheap) }
}
#[cfg(feature = "Win32_winnt")]
#[inline]
pub unsafe fn HeapValidate(hheap: super::winnt::HANDLE, dwflags: u32, lpmem: Option<*const core::ffi::c_void>) -> windows_core::BOOL {
    windows_core::link!("kernel32.dll" "system" fn HeapValidate(hheap : super::winnt::HANDLE, dwflags : u32, lpmem : *const core::ffi::c_void) -> windows_core::BOOL);
    unsafe { HeapValidate(hheap, dwflags, lpmem.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(all(feature = "Win32_minwinbase", feature = "Win32_winnt"))]
#[inline]
pub unsafe fn HeapWalk(hheap: super::winnt::HANDLE, lpentry: *mut super::minwinbase::PROCESS_HEAP_ENTRY) -> windows_core::BOOL {
    windows_core::link!("kernel32.dll" "system" fn HeapWalk(hheap : super::winnt::HANDLE, lpentry : *mut super::minwinbase::PROCESS_HEAP_ENTRY) -> windows_core::BOOL);
    unsafe { HeapWalk(hheap, lpentry as _) }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct HEAP_SUMMARY {
    pub cb: u32,
    pub cbAllocated: usize,
    pub cbCommitted: usize,
    pub cbReserved: usize,
    pub cbMaxReserve: usize,
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub struct LPHEAP_SUMMARY(pub PHEAP_SUMMARY);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PHEAP_SUMMARY(pub *mut HEAP_SUMMARY);
impl PHEAP_SUMMARY {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PHEAP_SUMMARY {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

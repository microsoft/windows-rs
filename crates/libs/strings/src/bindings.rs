pub type GetProcessHeap = unsafe extern "system" fn() -> HANDLE;
windows_link::link!("kernel32.dll" "system" fn GetProcessHeap() -> HANDLE);
pub type HeapAlloc = unsafe extern "system" fn(
    hheap: HANDLE,
    dwflags: HEAP_FLAGS,
    dwbytes: usize,
) -> *mut core::ffi::c_void;
windows_link::link!("kernel32.dll" "system" fn HeapAlloc(hheap : HANDLE, dwflags : HEAP_FLAGS, dwbytes : usize) -> *mut core::ffi::c_void);
pub type HeapFree = unsafe extern "system" fn(
    hheap: HANDLE,
    dwflags: HEAP_FLAGS,
    lpmem: *const core::ffi::c_void,
) -> BOOL;
windows_link::link!("kernel32.dll" "system" fn HeapFree(hheap : HANDLE, dwflags : HEAP_FLAGS, lpmem : *const core::ffi::c_void) -> BOOL);
pub type SysAllocStringLen = unsafe extern "system" fn(strin: PCWSTR, ui: u32) -> BSTR;
windows_link::link!("oleaut32.dll" "system" fn SysAllocStringLen(strin : PCWSTR, ui : u32) -> BSTR);
pub type SysFreeString = unsafe extern "system" fn(bstrstring: BSTR);
windows_link::link!("oleaut32.dll" "system" fn SysFreeString(bstrstring : BSTR));
pub type SysStringLen = unsafe extern "system" fn(pbstr: BSTR) -> u32;
windows_link::link!("oleaut32.dll" "system" fn SysStringLen(pbstr : BSTR) -> u32);
pub type BOOL = i32;
pub type BSTR = *const u16;
pub type HANDLE = *mut core::ffi::c_void;
pub type HEAP_FLAGS = u32;
pub type PCWSTR = *const u16;

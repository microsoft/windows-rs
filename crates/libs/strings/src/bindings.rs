#![allow(
    non_snake_case,
    non_upper_case_globals,
    non_camel_case_types,
    dead_code,
    clippy::all
)]
windows_targets::link!("kernel32.dll" "system" fn GetProcessHeap() -> HANDLE);
windows_targets::link!("kernel32.dll" "system" fn HeapAlloc(hheap : HANDLE, dwflags : HEAP_FLAGS, dwbytes : usize) -> *mut core::ffi::c_void);
windows_targets::link!("kernel32.dll" "system" fn HeapFree(hheap : HANDLE, dwflags : HEAP_FLAGS, lpmem : *const core::ffi::c_void) -> BOOL);
windows_targets::link!("oleaut32.dll" "system" fn SysAllocStringLen(strin : PCWSTR, ui : u32) -> BSTR);
windows_targets::link!("oleaut32.dll" "system" fn SysFreeString(bstrstring : BSTR));
windows_targets::link!("oleaut32.dll" "system" fn SysStringLen(pbstr : BSTR) -> u32);
pub type BOOL = i32;
pub type BSTR = *const u16;
pub const E_OUTOFMEMORY: HRESULT = 0x8007000E_u32 as _;
pub type HANDLE = *mut core::ffi::c_void;
pub type HEAP_FLAGS = u32;
pub type HRESULT = i32;
pub type PCWSTR = *const u16;

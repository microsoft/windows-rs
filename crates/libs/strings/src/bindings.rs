windows_link::link!("kernel32.dll" "system" fn GetProcessHeap() -> HANDLE);
windows_link::link!("kernel32.dll" "system" fn HeapAlloc(hheap : HANDLE, dwflags : DWORD, dwbytes : SIZE_T) -> LPVOID);
windows_link::link!("kernel32.dll" "system" fn HeapFree(hheap : HANDLE, dwflags : DWORD, lpmem : LPVOID) -> BOOL);
windows_link::link!("oleaut32.dll" "system" fn SysAllocStringLen(strin : *const OLECHAR, ui : UINT) -> BSTR);
windows_link::link!("oleaut32.dll" "system" fn SysFreeString(bstrstring : BSTR));
windows_link::link!("oleaut32.dll" "system" fn SysStringLen(pbstr : BSTR) -> UINT);
pub type BOOL = i32;
pub type BSTR = *const u16;
pub type DWORD = u32;
pub type HANDLE = *mut core::ffi::c_void;
pub type LPVOID = *mut core::ffi::c_void;
pub type OLECHAR = u16;
#[cfg(target_arch = "x86")]
pub type SIZE_T = u32;
#[cfg(any(
    target_arch = "aarch64",
    target_arch = "arm64ec",
    target_arch = "x86_64"
))]
pub type SIZE_T = u64;
pub type UINT = u32;

#![allow(
    non_snake_case,
    non_upper_case_globals,
    non_camel_case_types,
    dead_code,
    clippy::all
)]

windows_link::link!("kernel32.dll" "system" fn CloseHandle(hobject : HANDLE) -> BOOL);
windows_link::link!("ole32.dll" "system" fn CoIncrementMTAUsage(pcookie : *mut CO_MTA_USAGE_COOKIE) -> HRESULT);
windows_link::link!("ole32.dll" "system" fn CoTaskMemAlloc(cb : usize) -> *mut core::ffi::c_void);
windows_link::link!("ole32.dll" "system" fn CoTaskMemFree(pv : *const core::ffi::c_void));
windows_link::link!("kernel32.dll" "system" fn CreateEventW(lpeventattributes : *const SECURITY_ATTRIBUTES, bmanualreset : BOOL, binitialstate : BOOL, lpname : PCWSTR) -> HANDLE);
windows_link::link!("kernel32.dll" "system" fn EncodePointer(ptr : *const core::ffi::c_void) -> *mut core::ffi::c_void);
windows_link::link!("kernel32.dll" "system" fn FreeLibrary(hlibmodule : HMODULE) -> BOOL);
windows_link::link!("kernel32.dll" "system" fn GetProcAddress(hmodule : HMODULE, lpprocname : PCSTR) -> FARPROC);
windows_link::link!("kernel32.dll" "system" fn LoadLibraryExA(lplibfilename : PCSTR, hfile : HANDLE, dwflags : LOAD_LIBRARY_FLAGS) -> HMODULE);
windows_link::link!("api-ms-win-core-winrt-l1-1-0.dll" "system" fn RoGetActivationFactory(activatableclassid : HSTRING, iid : *const GUID, factory : *mut *mut core::ffi::c_void) -> HRESULT);
windows_link::link!("kernel32.dll" "system" fn SetEvent(hevent : HANDLE) -> BOOL);
windows_link::link!("kernel32.dll" "system" fn WaitForSingleObject(hhandle : HANDLE, dwmilliseconds : u32) -> WAIT_EVENT);
pub type BOOL = i32;
pub type CO_MTA_USAGE_COOKIE = *mut core::ffi::c_void;
pub type FARPROC = Option<unsafe extern "system" fn() -> isize>;
#[repr(C)]
#[derive(Clone, Copy)]
pub struct GUID {
    pub data1: u32,
    pub data2: u16,
    pub data3: u16,
    pub data4: [u8; 8],
}
impl GUID {
    pub const fn from_u128(uuid: u128) -> Self {
        Self {
            data1: (uuid >> 96) as u32,
            data2: (uuid >> 80 & 0xffff) as u16,
            data3: (uuid >> 64 & 0xffff) as u16,
            data4: (uuid as u64).to_be_bytes(),
        }
    }
}
pub type HANDLE = *mut core::ffi::c_void;
pub type HINSTANCE = *mut core::ffi::c_void;
pub type HMODULE = *mut core::ffi::c_void;
pub type HRESULT = i32;
pub type LOAD_LIBRARY_FLAGS = u32;
pub const LOAD_LIBRARY_SEARCH_DEFAULT_DIRS: LOAD_LIBRARY_FLAGS = 4096u32;
pub type PCSTR = *const u8;
pub type PCWSTR = *const u16;
#[repr(C)]
#[derive(Clone, Copy)]
pub struct SECURITY_ATTRIBUTES {
    pub nLength: u32,
    pub lpSecurityDescriptor: *mut core::ffi::c_void,
    pub bInheritHandle: BOOL,
}
impl Default for SECURITY_ATTRIBUTES {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub type HSTRING = *mut core::ffi::c_void;
pub type WAIT_EVENT = u32;

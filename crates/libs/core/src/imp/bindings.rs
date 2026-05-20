pub type CoIncrementMTAUsage =
    unsafe extern "system" fn(pcookie: *mut CO_MTA_USAGE_COOKIE) -> HRESULT;
windows_link::link!("combase.dll" "system" fn CoIncrementMTAUsage(pcookie : *mut CO_MTA_USAGE_COOKIE) -> HRESULT);
pub type CoTaskMemAlloc = unsafe extern "system" fn(cb: usize) -> *mut core::ffi::c_void;
windows_link::link!("combase.dll" "system" fn CoTaskMemAlloc(cb : usize) -> *mut core::ffi::c_void);
pub type CoTaskMemFree = unsafe extern "system" fn(pv: *const core::ffi::c_void);
windows_link::link!("combase.dll" "system" fn CoTaskMemFree(pv : *const core::ffi::c_void));
pub type EncodePointer =
    unsafe extern "system" fn(ptr: *const core::ffi::c_void) -> *mut core::ffi::c_void;
windows_link::link!("kernel32.dll" "system" fn EncodePointer(ptr : *const core::ffi::c_void) -> *mut core::ffi::c_void);
pub type FreeLibrary = unsafe extern "system" fn(hlibmodule: HMODULE) -> BOOL;
windows_link::link!("kernel32.dll" "system" fn FreeLibrary(hlibmodule : HMODULE) -> BOOL);
pub type GetProcAddress = unsafe extern "system" fn(hmodule: HMODULE, lpprocname: PCSTR) -> FARPROC;
windows_link::link!("kernel32.dll" "system" fn GetProcAddress(hmodule : HMODULE, lpprocname : PCSTR) -> FARPROC);
pub type LoadLibraryExA = unsafe extern "system" fn(
    lplibfilename: PCSTR,
    hfile: HANDLE,
    dwflags: LOAD_LIBRARY_FLAGS,
) -> HMODULE;
windows_link::link!("kernel32.dll" "system" fn LoadLibraryExA(lplibfilename : PCSTR, hfile : HANDLE, dwflags : LOAD_LIBRARY_FLAGS) -> HMODULE);
pub type RoGetActivationFactory = unsafe extern "system" fn(
    activatableclassid: HSTRING,
    iid: *const GUID,
    factory: *mut *mut core::ffi::c_void,
) -> HRESULT;
windows_link::link!("api-ms-win-core-winrt-l1-1-0.dll" "system" fn RoGetActivationFactory(activatableclassid : HSTRING, iid : *const GUID, factory : *mut *mut core::ffi::c_void) -> HRESULT);
pub type UuidCreate = unsafe extern "system" fn(uuid: *mut GUID) -> RPC_STATUS;
windows_link::link!("rpcrt4.dll" "system" fn UuidCreate(uuid : *mut GUID) -> RPC_STATUS);
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
pub type RPC_STATUS = i32;
pub const RPC_S_UUID_LOCAL_ONLY: RPC_STATUS = 1824i32;
pub type HSTRING = *mut core::ffi::c_void;

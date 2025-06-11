#![allow(
    non_snake_case,
    non_upper_case_globals,
    non_camel_case_types,
    dead_code,
    clippy::all
)]

windows_link::link!("kernel32.dll" "system" fn CloseHandle(hobject : HANDLE) -> BOOL);
windows_link::link!("ole32.dll" "system" fn CoCreateInstance(rclsid : *const GUID, punkouter : * mut core::ffi::c_void, dwclscontext : CLSCTX, riid : *const GUID, ppv : *mut *mut core::ffi::c_void) -> HRESULT);
windows_link::link!("kernel32.dll" "system" fn CreateEventW(lpeventattributes : *const SECURITY_ATTRIBUTES, bmanualreset : BOOL, binitialstate : BOOL, lpname : PCWSTR) -> HANDLE);
windows_link::link!("kernel32.dll" "system" fn SetEvent(hevent : HANDLE) -> BOOL);
windows_link::link!("kernel32.dll" "system" fn WaitForSingleObject(hhandle : HANDLE, dwmilliseconds : u32) -> WAIT_EVENT);
windows_link::link!("user32.dll" "C" fn wsprintfA(param0 : PSTR, param1 : PCSTR, ...) -> i32);
pub type BOOL = i32;
pub type CLSCTX = u32;
pub const CLSCTX_ALL: CLSCTX = 23u32;
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
pub type HRESULT = i32;
pub const IID_IUnknown: GUID = GUID::from_u128(0x00000000_0000_0000_c000_000000000046);
#[repr(C)]
pub struct IUnknown_Vtbl {
    pub QueryInterface: unsafe extern "system" fn(
        this: *mut core::ffi::c_void,
        iid: *const GUID,
        interface: *mut *mut core::ffi::c_void,
    ) -> HRESULT,
    pub AddRef: unsafe extern "system" fn(this: *mut core::ffi::c_void) -> u32,
    pub Release: unsafe extern "system" fn(this: *mut core::ffi::c_void) -> u32,
}
pub type PCSTR = *const u8;
pub type PCWSTR = *const u16;
pub type PSTR = *mut u8;
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
pub const STGTY_REPEAT: i32 = 256i32;
pub const UIAnimationManager: GUID = GUID::from_u128(0x4c1fc63a_695c_47e8_a339_1a194be3d0b8);
pub type WAIT_EVENT = u32;

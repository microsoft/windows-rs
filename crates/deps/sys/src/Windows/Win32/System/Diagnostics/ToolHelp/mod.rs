#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals)]
#[link(name = "windows")]
extern "system" {
    #[cfg(feature = "Win32_Foundation")]
    pub fn CreateToolhelp32Snapshot(dwflags: CREATE_TOOLHELP_SNAPSHOT_FLAGS, th32processid: u32) -> super::super::super::Foundation::HANDLE;
    #[cfg(feature = "Win32_Foundation")]
    pub fn Heap32First(lphe: *mut HEAPENTRY32, th32processid: u32, th32heapid: usize) -> super::super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn Heap32ListFirst(hsnapshot: super::super::super::Foundation::HANDLE, lphl: *mut HEAPLIST32) -> super::super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn Heap32ListNext(hsnapshot: super::super::super::Foundation::HANDLE, lphl: *mut HEAPLIST32) -> super::super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn Heap32Next(lphe: *mut HEAPENTRY32) -> super::super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn Module32First(hsnapshot: super::super::super::Foundation::HANDLE, lpme: *mut MODULEENTRY32) -> super::super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn Module32FirstW(hsnapshot: super::super::super::Foundation::HANDLE, lpme: *mut MODULEENTRY32W) -> super::super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn Module32Next(hsnapshot: super::super::super::Foundation::HANDLE, lpme: *mut MODULEENTRY32) -> super::super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn Module32NextW(hsnapshot: super::super::super::Foundation::HANDLE, lpme: *mut MODULEENTRY32W) -> super::super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn Process32First(hsnapshot: super::super::super::Foundation::HANDLE, lppe: *mut PROCESSENTRY32) -> super::super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn Process32FirstW(hsnapshot: super::super::super::Foundation::HANDLE, lppe: *mut PROCESSENTRY32W) -> super::super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn Process32Next(hsnapshot: super::super::super::Foundation::HANDLE, lppe: *mut PROCESSENTRY32) -> super::super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn Process32NextW(hsnapshot: super::super::super::Foundation::HANDLE, lppe: *mut PROCESSENTRY32W) -> super::super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn Thread32First(hsnapshot: super::super::super::Foundation::HANDLE, lpte: *mut THREADENTRY32) -> super::super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn Thread32Next(hsnapshot: super::super::super::Foundation::HANDLE, lpte: *mut THREADENTRY32) -> super::super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn Toolhelp32ReadProcessMemory(th32processid: u32, lpbaseaddress: *const ::core::ffi::c_void, lpbuffer: *mut ::core::ffi::c_void, cbread: usize, lpnumberofbytesread: *mut usize) -> super::super::super::Foundation::BOOL;
}
#[repr(transparent)]
pub struct CREATE_TOOLHELP_SNAPSHOT_FLAGS(pub u32);
pub const TH32CS_INHERIT: CREATE_TOOLHELP_SNAPSHOT_FLAGS = CREATE_TOOLHELP_SNAPSHOT_FLAGS(2147483648u32);
pub const TH32CS_SNAPALL: CREATE_TOOLHELP_SNAPSHOT_FLAGS = CREATE_TOOLHELP_SNAPSHOT_FLAGS(15u32);
pub const TH32CS_SNAPHEAPLIST: CREATE_TOOLHELP_SNAPSHOT_FLAGS = CREATE_TOOLHELP_SNAPSHOT_FLAGS(1u32);
pub const TH32CS_SNAPMODULE: CREATE_TOOLHELP_SNAPSHOT_FLAGS = CREATE_TOOLHELP_SNAPSHOT_FLAGS(8u32);
pub const TH32CS_SNAPMODULE32: CREATE_TOOLHELP_SNAPSHOT_FLAGS = CREATE_TOOLHELP_SNAPSHOT_FLAGS(16u32);
pub const TH32CS_SNAPPROCESS: CREATE_TOOLHELP_SNAPSHOT_FLAGS = CREATE_TOOLHELP_SNAPSHOT_FLAGS(2u32);
pub const TH32CS_SNAPTHREAD: CREATE_TOOLHELP_SNAPSHOT_FLAGS = CREATE_TOOLHELP_SNAPSHOT_FLAGS(4u32);
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct HEAPENTRY32(i32);
#[repr(transparent)]
pub struct HEAPENTRY32_FLAGS(pub u32);
pub const LF32_FIXED: HEAPENTRY32_FLAGS = HEAPENTRY32_FLAGS(1u32);
pub const LF32_FREE: HEAPENTRY32_FLAGS = HEAPENTRY32_FLAGS(2u32);
pub const LF32_MOVEABLE: HEAPENTRY32_FLAGS = HEAPENTRY32_FLAGS(4u32);
#[repr(C)]
pub struct HEAPLIST32(i32);
pub const HF32_DEFAULT: u32 = 1u32;
pub const HF32_SHARED: u32 = 2u32;
pub const MAX_MODULE_NAME32: u32 = 255u32;
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct MODULEENTRY32(i32);
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct MODULEENTRY32W(i32);
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct PROCESSENTRY32(i32);
#[repr(C)]
pub struct PROCESSENTRY32W(i32);
#[repr(C)]
pub struct THREADENTRY32(i32);

#![allow(non_snake_case, non_camel_case_types)]
#[link(name = "windows")]
extern "system" {
    #[doc = "*Required features: `Win32_System_Diagnostics_ToolHelp`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CreateToolhelp32Snapshot(dwflags: CREATE_TOOLHELP_SNAPSHOT_FLAGS, th32processid: u32) -> super::super::super::Foundation::HANDLE;
    #[doc = "*Required features: `Win32_System_Diagnostics_ToolHelp`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn Heap32First(lphe: *mut HEAPENTRY32, th32processid: u32, th32heapid: usize) -> super::super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_System_Diagnostics_ToolHelp`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn Heap32ListFirst(hsnapshot: super::super::super::Foundation::HANDLE, lphl: *mut HEAPLIST32) -> super::super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_System_Diagnostics_ToolHelp`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn Heap32ListNext(hsnapshot: super::super::super::Foundation::HANDLE, lphl: *mut HEAPLIST32) -> super::super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_System_Diagnostics_ToolHelp`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn Heap32Next(lphe: *mut HEAPENTRY32) -> super::super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_System_Diagnostics_ToolHelp`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn Module32First(hsnapshot: super::super::super::Foundation::HANDLE, lpme: *mut MODULEENTRY32) -> super::super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_System_Diagnostics_ToolHelp`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn Module32FirstW(hsnapshot: super::super::super::Foundation::HANDLE, lpme: *mut MODULEENTRY32W) -> super::super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_System_Diagnostics_ToolHelp`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn Module32Next(hsnapshot: super::super::super::Foundation::HANDLE, lpme: *mut MODULEENTRY32) -> super::super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_System_Diagnostics_ToolHelp`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn Module32NextW(hsnapshot: super::super::super::Foundation::HANDLE, lpme: *mut MODULEENTRY32W) -> super::super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_System_Diagnostics_ToolHelp`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn Process32First(hsnapshot: super::super::super::Foundation::HANDLE, lppe: *mut PROCESSENTRY32) -> super::super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_System_Diagnostics_ToolHelp`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn Process32FirstW(hsnapshot: super::super::super::Foundation::HANDLE, lppe: *mut PROCESSENTRY32W) -> super::super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_System_Diagnostics_ToolHelp`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn Process32Next(hsnapshot: super::super::super::Foundation::HANDLE, lppe: *mut PROCESSENTRY32) -> super::super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_System_Diagnostics_ToolHelp`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn Process32NextW(hsnapshot: super::super::super::Foundation::HANDLE, lppe: *mut PROCESSENTRY32W) -> super::super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_System_Diagnostics_ToolHelp`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn Thread32First(hsnapshot: super::super::super::Foundation::HANDLE, lpte: *mut THREADENTRY32) -> super::super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_System_Diagnostics_ToolHelp`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn Thread32Next(hsnapshot: super::super::super::Foundation::HANDLE, lpte: *mut THREADENTRY32) -> super::super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_System_Diagnostics_ToolHelp`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn Toolhelp32ReadProcessMemory(th32processid: u32, lpbaseaddress: *const ::core::ffi::c_void, lpbuffer: *mut ::core::ffi::c_void, cbread: usize, lpnumberofbytesread: *mut usize) -> super::super::super::Foundation::BOOL;
}
pub struct CREATE_TOOLHELP_SNAPSHOT_FLAGS(i32);
pub struct HEAPENTRY32(i32);
pub struct HEAPENTRY32_FLAGS(i32);
pub struct HEAPLIST32(i32);
#[doc = "*Required features: `Win32_System_Diagnostics_ToolHelp`*"]
pub const HF32_DEFAULT: u32 = 1u32;
#[doc = "*Required features: `Win32_System_Diagnostics_ToolHelp`*"]
pub const HF32_SHARED: u32 = 2u32;
#[doc = "*Required features: `Win32_System_Diagnostics_ToolHelp`*"]
pub const MAX_MODULE_NAME32: u32 = 255u32;
pub struct MODULEENTRY32(i32);
pub struct MODULEENTRY32W(i32);
pub struct PROCESSENTRY32(i32);
pub struct PROCESSENTRY32W(i32);
pub struct THREADENTRY32(i32);

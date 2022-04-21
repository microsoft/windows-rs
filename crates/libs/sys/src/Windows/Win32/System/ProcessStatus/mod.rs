#[link(name = "windows")]
extern "system" {
    #[doc = "*Required features: `\"Win32_System_ProcessStatus\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn K32EmptyWorkingSet(hprocess: super::super::Foundation::HANDLE) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_System_ProcessStatus\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn K32EnumDeviceDrivers(lpimagebase: *mut *mut ::core::ffi::c_void, cb: u32, lpcbneeded: *mut u32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_System_ProcessStatus\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn K32EnumPageFilesA(pcallbackroutine: PENUM_PAGE_FILE_CALLBACKA, pcontext: *mut ::core::ffi::c_void) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_System_ProcessStatus\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn K32EnumPageFilesW(pcallbackroutine: PENUM_PAGE_FILE_CALLBACKW, pcontext: *mut ::core::ffi::c_void) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_System_ProcessStatus\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn K32EnumProcessModules(hprocess: super::super::Foundation::HANDLE, lphmodule: *mut super::super::Foundation::HINSTANCE, cb: u32, lpcbneeded: *mut u32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_System_ProcessStatus\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn K32EnumProcessModulesEx(hprocess: super::super::Foundation::HANDLE, lphmodule: *mut super::super::Foundation::HINSTANCE, cb: u32, lpcbneeded: *mut u32, dwfilterflag: ENUM_PROCESS_MODULES_EX_FLAGS) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_System_ProcessStatus\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn K32EnumProcesses(lpidprocess: *mut u32, cb: u32, lpcbneeded: *mut u32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_System_ProcessStatus\"`*"]
    pub fn K32GetDeviceDriverBaseNameA(imagebase: *const ::core::ffi::c_void, lpfilename: ::windows_sys::core::PSTR, nsize: u32) -> u32;
    #[doc = "*Required features: `\"Win32_System_ProcessStatus\"`*"]
    pub fn K32GetDeviceDriverBaseNameW(imagebase: *const ::core::ffi::c_void, lpbasename: ::windows_sys::core::PWSTR, nsize: u32) -> u32;
    #[doc = "*Required features: `\"Win32_System_ProcessStatus\"`*"]
    pub fn K32GetDeviceDriverFileNameA(imagebase: *const ::core::ffi::c_void, lpfilename: ::windows_sys::core::PSTR, nsize: u32) -> u32;
    #[doc = "*Required features: `\"Win32_System_ProcessStatus\"`*"]
    pub fn K32GetDeviceDriverFileNameW(imagebase: *const ::core::ffi::c_void, lpfilename: ::windows_sys::core::PWSTR, nsize: u32) -> u32;
    #[doc = "*Required features: `\"Win32_System_ProcessStatus\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn K32GetMappedFileNameA(hprocess: super::super::Foundation::HANDLE, lpv: *const ::core::ffi::c_void, lpfilename: ::windows_sys::core::PSTR, nsize: u32) -> u32;
    #[doc = "*Required features: `\"Win32_System_ProcessStatus\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn K32GetMappedFileNameW(hprocess: super::super::Foundation::HANDLE, lpv: *const ::core::ffi::c_void, lpfilename: ::windows_sys::core::PWSTR, nsize: u32) -> u32;
    #[doc = "*Required features: `\"Win32_System_ProcessStatus\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn K32GetModuleBaseNameA(hprocess: super::super::Foundation::HANDLE, hmodule: super::super::Foundation::HINSTANCE, lpbasename: ::windows_sys::core::PSTR, nsize: u32) -> u32;
    #[doc = "*Required features: `\"Win32_System_ProcessStatus\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn K32GetModuleBaseNameW(hprocess: super::super::Foundation::HANDLE, hmodule: super::super::Foundation::HINSTANCE, lpbasename: ::windows_sys::core::PWSTR, nsize: u32) -> u32;
    #[doc = "*Required features: `\"Win32_System_ProcessStatus\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn K32GetModuleFileNameExA(hprocess: super::super::Foundation::HANDLE, hmodule: super::super::Foundation::HINSTANCE, lpfilename: ::windows_sys::core::PSTR, nsize: u32) -> u32;
    #[doc = "*Required features: `\"Win32_System_ProcessStatus\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn K32GetModuleFileNameExW(hprocess: super::super::Foundation::HANDLE, hmodule: super::super::Foundation::HINSTANCE, lpfilename: ::windows_sys::core::PWSTR, nsize: u32) -> u32;
    #[doc = "*Required features: `\"Win32_System_ProcessStatus\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn K32GetModuleInformation(hprocess: super::super::Foundation::HANDLE, hmodule: super::super::Foundation::HINSTANCE, lpmodinfo: *mut MODULEINFO, cb: u32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_System_ProcessStatus\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn K32GetPerformanceInfo(pperformanceinformation: *mut PERFORMANCE_INFORMATION, cb: u32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_System_ProcessStatus\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn K32GetProcessImageFileNameA(hprocess: super::super::Foundation::HANDLE, lpimagefilename: ::windows_sys::core::PSTR, nsize: u32) -> u32;
    #[doc = "*Required features: `\"Win32_System_ProcessStatus\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn K32GetProcessImageFileNameW(hprocess: super::super::Foundation::HANDLE, lpimagefilename: ::windows_sys::core::PWSTR, nsize: u32) -> u32;
    #[doc = "*Required features: `\"Win32_System_ProcessStatus\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn K32GetProcessMemoryInfo(process: super::super::Foundation::HANDLE, ppsmemcounters: *mut PROCESS_MEMORY_COUNTERS, cb: u32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_System_ProcessStatus\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn K32GetWsChanges(hprocess: super::super::Foundation::HANDLE, lpwatchinfo: *mut PSAPI_WS_WATCH_INFORMATION, cb: u32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_System_ProcessStatus\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn K32GetWsChangesEx(hprocess: super::super::Foundation::HANDLE, lpwatchinfoex: *mut PSAPI_WS_WATCH_INFORMATION_EX, cb: *mut u32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_System_ProcessStatus\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn K32InitializeProcessForWsWatch(hprocess: super::super::Foundation::HANDLE) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_System_ProcessStatus\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn K32QueryWorkingSet(hprocess: super::super::Foundation::HANDLE, pv: *mut ::core::ffi::c_void, cb: u32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `\"Win32_System_ProcessStatus\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn K32QueryWorkingSetEx(hprocess: super::super::Foundation::HANDLE, pv: *mut ::core::ffi::c_void, cb: u32) -> super::super::Foundation::BOOL;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_ProcessStatus\"`*"]
pub struct ENUM_PAGE_FILE_INFORMATION {
    pub cb: u32,
    pub Reserved: u32,
    pub TotalSize: usize,
    pub TotalInUse: usize,
    pub PeakUsage: usize,
}
impl ::core::marker::Copy for ENUM_PAGE_FILE_INFORMATION {}
impl ::core::clone::Clone for ENUM_PAGE_FILE_INFORMATION {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_System_ProcessStatus\"`*"]
pub type ENUM_PROCESS_MODULES_EX_FLAGS = u32;
#[doc = "*Required features: `\"Win32_System_ProcessStatus\"`*"]
pub const LIST_MODULES_ALL: ENUM_PROCESS_MODULES_EX_FLAGS = 3u32;
#[doc = "*Required features: `\"Win32_System_ProcessStatus\"`*"]
pub const LIST_MODULES_DEFAULT: ENUM_PROCESS_MODULES_EX_FLAGS = 0u32;
#[doc = "*Required features: `\"Win32_System_ProcessStatus\"`*"]
pub const LIST_MODULES_32BIT: ENUM_PROCESS_MODULES_EX_FLAGS = 1u32;
#[doc = "*Required features: `\"Win32_System_ProcessStatus\"`*"]
pub const LIST_MODULES_64BIT: ENUM_PROCESS_MODULES_EX_FLAGS = 2u32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_ProcessStatus\"`*"]
pub struct MODULEINFO {
    pub lpBaseOfDll: *mut ::core::ffi::c_void,
    pub SizeOfImage: u32,
    pub EntryPoint: *mut ::core::ffi::c_void,
}
impl ::core::marker::Copy for MODULEINFO {}
impl ::core::clone::Clone for MODULEINFO {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_System_ProcessStatus\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub type PENUM_PAGE_FILE_CALLBACKA = ::core::option::Option<unsafe extern "system" fn(pcontext: *mut ::core::ffi::c_void, ppagefileinfo: *mut ENUM_PAGE_FILE_INFORMATION, lpfilename: ::windows_sys::core::PCSTR) -> super::super::Foundation::BOOL>;
#[doc = "*Required features: `\"Win32_System_ProcessStatus\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub type PENUM_PAGE_FILE_CALLBACKW = ::core::option::Option<unsafe extern "system" fn(pcontext: *mut ::core::ffi::c_void, ppagefileinfo: *mut ENUM_PAGE_FILE_INFORMATION, lpfilename: ::windows_sys::core::PCWSTR) -> super::super::Foundation::BOOL>;
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_ProcessStatus\"`*"]
pub struct PERFORMANCE_INFORMATION {
    pub cb: u32,
    pub CommitTotal: usize,
    pub CommitLimit: usize,
    pub CommitPeak: usize,
    pub PhysicalTotal: usize,
    pub PhysicalAvailable: usize,
    pub SystemCache: usize,
    pub KernelTotal: usize,
    pub KernelPaged: usize,
    pub KernelNonpaged: usize,
    pub PageSize: usize,
    pub HandleCount: u32,
    pub ProcessCount: u32,
    pub ThreadCount: u32,
}
impl ::core::marker::Copy for PERFORMANCE_INFORMATION {}
impl ::core::clone::Clone for PERFORMANCE_INFORMATION {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_ProcessStatus\"`*"]
pub struct PROCESS_MEMORY_COUNTERS {
    pub cb: u32,
    pub PageFaultCount: u32,
    pub PeakWorkingSetSize: usize,
    pub WorkingSetSize: usize,
    pub QuotaPeakPagedPoolUsage: usize,
    pub QuotaPagedPoolUsage: usize,
    pub QuotaPeakNonPagedPoolUsage: usize,
    pub QuotaNonPagedPoolUsage: usize,
    pub PagefileUsage: usize,
    pub PeakPagefileUsage: usize,
}
impl ::core::marker::Copy for PROCESS_MEMORY_COUNTERS {}
impl ::core::clone::Clone for PROCESS_MEMORY_COUNTERS {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_ProcessStatus\"`*"]
pub struct PROCESS_MEMORY_COUNTERS_EX {
    pub cb: u32,
    pub PageFaultCount: u32,
    pub PeakWorkingSetSize: usize,
    pub WorkingSetSize: usize,
    pub QuotaPeakPagedPoolUsage: usize,
    pub QuotaPagedPoolUsage: usize,
    pub QuotaPeakNonPagedPoolUsage: usize,
    pub QuotaNonPagedPoolUsage: usize,
    pub PagefileUsage: usize,
    pub PeakPagefileUsage: usize,
    pub PrivateUsage: usize,
}
impl ::core::marker::Copy for PROCESS_MEMORY_COUNTERS_EX {}
impl ::core::clone::Clone for PROCESS_MEMORY_COUNTERS_EX {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_System_ProcessStatus\"`*"]
pub const PSAPI_VERSION: u32 = 2u32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_ProcessStatus\"`*"]
pub union PSAPI_WORKING_SET_BLOCK {
    pub Flags: usize,
    pub Anonymous: PSAPI_WORKING_SET_BLOCK_0,
}
impl ::core::marker::Copy for PSAPI_WORKING_SET_BLOCK {}
impl ::core::clone::Clone for PSAPI_WORKING_SET_BLOCK {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_ProcessStatus\"`*"]
pub struct PSAPI_WORKING_SET_BLOCK_0 {
    pub _bitfield: usize,
}
impl ::core::marker::Copy for PSAPI_WORKING_SET_BLOCK_0 {}
impl ::core::clone::Clone for PSAPI_WORKING_SET_BLOCK_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_ProcessStatus\"`*"]
pub union PSAPI_WORKING_SET_EX_BLOCK {
    pub Flags: usize,
    pub Anonymous: PSAPI_WORKING_SET_EX_BLOCK_0,
}
impl ::core::marker::Copy for PSAPI_WORKING_SET_EX_BLOCK {}
impl ::core::clone::Clone for PSAPI_WORKING_SET_EX_BLOCK {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_ProcessStatus\"`*"]
pub union PSAPI_WORKING_SET_EX_BLOCK_0 {
    pub Anonymous: PSAPI_WORKING_SET_EX_BLOCK_0_0,
    pub Invalid: PSAPI_WORKING_SET_EX_BLOCK_0_1,
}
impl ::core::marker::Copy for PSAPI_WORKING_SET_EX_BLOCK_0 {}
impl ::core::clone::Clone for PSAPI_WORKING_SET_EX_BLOCK_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_ProcessStatus\"`*"]
pub struct PSAPI_WORKING_SET_EX_BLOCK_0_0 {
    pub _bitfield: usize,
}
impl ::core::marker::Copy for PSAPI_WORKING_SET_EX_BLOCK_0_0 {}
impl ::core::clone::Clone for PSAPI_WORKING_SET_EX_BLOCK_0_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_ProcessStatus\"`*"]
pub struct PSAPI_WORKING_SET_EX_BLOCK_0_1 {
    pub _bitfield: usize,
}
impl ::core::marker::Copy for PSAPI_WORKING_SET_EX_BLOCK_0_1 {}
impl ::core::clone::Clone for PSAPI_WORKING_SET_EX_BLOCK_0_1 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_ProcessStatus\"`*"]
pub struct PSAPI_WORKING_SET_EX_INFORMATION {
    pub VirtualAddress: *mut ::core::ffi::c_void,
    pub VirtualAttributes: PSAPI_WORKING_SET_EX_BLOCK,
}
impl ::core::marker::Copy for PSAPI_WORKING_SET_EX_INFORMATION {}
impl ::core::clone::Clone for PSAPI_WORKING_SET_EX_INFORMATION {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_ProcessStatus\"`*"]
pub struct PSAPI_WORKING_SET_INFORMATION {
    pub NumberOfEntries: usize,
    pub WorkingSetInfo: [PSAPI_WORKING_SET_BLOCK; 1],
}
impl ::core::marker::Copy for PSAPI_WORKING_SET_INFORMATION {}
impl ::core::clone::Clone for PSAPI_WORKING_SET_INFORMATION {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_ProcessStatus\"`*"]
pub struct PSAPI_WS_WATCH_INFORMATION {
    pub FaultingPc: *mut ::core::ffi::c_void,
    pub FaultingVa: *mut ::core::ffi::c_void,
}
impl ::core::marker::Copy for PSAPI_WS_WATCH_INFORMATION {}
impl ::core::clone::Clone for PSAPI_WS_WATCH_INFORMATION {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_ProcessStatus\"`*"]
pub struct PSAPI_WS_WATCH_INFORMATION_EX {
    pub BasicInfo: PSAPI_WS_WATCH_INFORMATION,
    pub FaultingThreadId: usize,
    pub Flags: usize,
}
impl ::core::marker::Copy for PSAPI_WS_WATCH_INFORMATION_EX {}
impl ::core::clone::Clone for PSAPI_WS_WATCH_INFORMATION_EX {
    fn clone(&self) -> Self {
        *self
    }
}

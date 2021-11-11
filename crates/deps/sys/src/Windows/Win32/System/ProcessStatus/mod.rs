#![allow(non_snake_case, non_camel_case_types)]
#[link(name = "windows")]
extern "system" {
    #[doc = "*Required features: `Win32_System_ProcessStatus`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn K32EmptyWorkingSet(hprocess: super::super::Foundation::HANDLE) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_System_ProcessStatus`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn K32EnumDeviceDrivers(lpimagebase: *mut *mut ::core::ffi::c_void, cb: u32, lpcbneeded: *mut u32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_System_ProcessStatus`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn K32EnumPageFilesA(pcallbackroutine: ::windows::runtime::RawPtr, pcontext: *mut ::core::ffi::c_void) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_System_ProcessStatus`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn K32EnumPageFilesW(pcallbackroutine: ::windows::runtime::RawPtr, pcontext: *mut ::core::ffi::c_void) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_System_ProcessStatus`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn K32EnumProcessModules(hprocess: super::super::Foundation::HANDLE, lphmodule: *mut super::super::Foundation::HINSTANCE, cb: u32, lpcbneeded: *mut u32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_System_ProcessStatus`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn K32EnumProcessModulesEx(hprocess: super::super::Foundation::HANDLE, lphmodule: *mut super::super::Foundation::HINSTANCE, cb: u32, lpcbneeded: *mut u32, dwfilterflag: ENUM_PROCESS_MODULES_EX_FLAGS) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_System_ProcessStatus`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn K32EnumProcesses(lpidprocess: *mut u32, cb: u32, lpcbneeded: *mut u32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_System_ProcessStatus`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn K32GetDeviceDriverBaseNameA(imagebase: *const ::core::ffi::c_void, lpfilename: super::super::Foundation::PSTR, nsize: u32) -> u32;
    #[doc = "*Required features: `Win32_System_ProcessStatus`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn K32GetDeviceDriverBaseNameW(imagebase: *const ::core::ffi::c_void, lpbasename: super::super::Foundation::PWSTR, nsize: u32) -> u32;
    #[doc = "*Required features: `Win32_System_ProcessStatus`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn K32GetDeviceDriverFileNameA(imagebase: *const ::core::ffi::c_void, lpfilename: super::super::Foundation::PSTR, nsize: u32) -> u32;
    #[doc = "*Required features: `Win32_System_ProcessStatus`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn K32GetDeviceDriverFileNameW(imagebase: *const ::core::ffi::c_void, lpfilename: super::super::Foundation::PWSTR, nsize: u32) -> u32;
    #[doc = "*Required features: `Win32_System_ProcessStatus`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn K32GetMappedFileNameA(hprocess: super::super::Foundation::HANDLE, lpv: *const ::core::ffi::c_void, lpfilename: super::super::Foundation::PSTR, nsize: u32) -> u32;
    #[doc = "*Required features: `Win32_System_ProcessStatus`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn K32GetMappedFileNameW(hprocess: super::super::Foundation::HANDLE, lpv: *const ::core::ffi::c_void, lpfilename: super::super::Foundation::PWSTR, nsize: u32) -> u32;
    #[doc = "*Required features: `Win32_System_ProcessStatus`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn K32GetModuleBaseNameA(hprocess: super::super::Foundation::HANDLE, hmodule: super::super::Foundation::HINSTANCE, lpbasename: super::super::Foundation::PSTR, nsize: u32) -> u32;
    #[doc = "*Required features: `Win32_System_ProcessStatus`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn K32GetModuleBaseNameW(hprocess: super::super::Foundation::HANDLE, hmodule: super::super::Foundation::HINSTANCE, lpbasename: super::super::Foundation::PWSTR, nsize: u32) -> u32;
    #[doc = "*Required features: `Win32_System_ProcessStatus`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn K32GetModuleFileNameExA(hprocess: super::super::Foundation::HANDLE, hmodule: super::super::Foundation::HINSTANCE, lpfilename: super::super::Foundation::PSTR, nsize: u32) -> u32;
    #[doc = "*Required features: `Win32_System_ProcessStatus`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn K32GetModuleFileNameExW(hprocess: super::super::Foundation::HANDLE, hmodule: super::super::Foundation::HINSTANCE, lpfilename: super::super::Foundation::PWSTR, nsize: u32) -> u32;
    #[doc = "*Required features: `Win32_System_ProcessStatus`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn K32GetModuleInformation(hprocess: super::super::Foundation::HANDLE, hmodule: super::super::Foundation::HINSTANCE, lpmodinfo: *mut MODULEINFO, cb: u32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_System_ProcessStatus`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn K32GetPerformanceInfo(pperformanceinformation: *mut PERFORMANCE_INFORMATION, cb: u32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_System_ProcessStatus`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn K32GetProcessImageFileNameA(hprocess: super::super::Foundation::HANDLE, lpimagefilename: super::super::Foundation::PSTR, nsize: u32) -> u32;
    #[doc = "*Required features: `Win32_System_ProcessStatus`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn K32GetProcessImageFileNameW(hprocess: super::super::Foundation::HANDLE, lpimagefilename: super::super::Foundation::PWSTR, nsize: u32) -> u32;
    #[doc = "*Required features: `Win32_System_ProcessStatus`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn K32GetProcessMemoryInfo(process: super::super::Foundation::HANDLE, ppsmemcounters: *mut PROCESS_MEMORY_COUNTERS, cb: u32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_System_ProcessStatus`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn K32GetWsChanges(hprocess: super::super::Foundation::HANDLE, lpwatchinfo: *mut PSAPI_WS_WATCH_INFORMATION, cb: u32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_System_ProcessStatus`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn K32GetWsChangesEx(hprocess: super::super::Foundation::HANDLE, lpwatchinfoex: *mut PSAPI_WS_WATCH_INFORMATION_EX, cb: *mut u32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_System_ProcessStatus`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn K32InitializeProcessForWsWatch(hprocess: super::super::Foundation::HANDLE) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_System_ProcessStatus`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn K32QueryWorkingSet(hprocess: super::super::Foundation::HANDLE, pv: *mut ::core::ffi::c_void, cb: u32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_System_ProcessStatus`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn K32QueryWorkingSetEx(hprocess: super::super::Foundation::HANDLE, pv: *mut ::core::ffi::c_void, cb: u32) -> super::super::Foundation::BOOL;
}

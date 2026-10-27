#[cfg(feature = "winnt")]
windows_link::link!("kernel32.dll" "system" "K32EmptyWorkingSet" fn EmptyWorkingSet(hprocess : super::HANDLE) -> windows_sys::core::BOOL);
#[cfg(all(feature = "minwindef", feature = "shared"))]
windows_link::link!("kernel32.dll" "system" "K32EnumDeviceDrivers" fn EnumDeviceDrivers(lpimagebase : *mut super::LPVOID, cb : super::DWORD, lpcbneeded : super::LPDWORD) -> windows_sys::core::BOOL);
#[cfg(all(feature = "minwindef", feature = "shared"))]
windows_link::link!("kernel32.dll" "system" "K32EnumPageFilesA" fn EnumPageFilesA(pcallbackroutine : PENUM_PAGE_FILE_CALLBACKA, pcontext : super::LPVOID) -> windows_sys::core::BOOL);
#[cfg(all(feature = "minwindef", feature = "shared"))]
windows_link::link!("kernel32.dll" "system" "K32EnumPageFilesW" fn EnumPageFilesW(pcallbackroutine : PENUM_PAGE_FILE_CALLBACKW, pcontext : super::LPVOID) -> windows_sys::core::BOOL);
#[cfg(all(feature = "minwindef", feature = "shared", feature = "winnt"))]
windows_link::link!("kernel32.dll" "system" "K32EnumProcessModules" fn EnumProcessModules(hprocess : super::HANDLE, lphmodule : *mut super::HMODULE, cb : super::DWORD, lpcbneeded : super::LPDWORD) -> windows_sys::core::BOOL);
#[cfg(all(feature = "minwindef", feature = "shared", feature = "winnt"))]
windows_link::link!("kernel32.dll" "system" "K32EnumProcessModulesEx" fn EnumProcessModulesEx(hprocess : super::HANDLE, lphmodule : *mut super::HMODULE, cb : super::DWORD, lpcbneeded : super::LPDWORD, dwfilterflag : super::DWORD) -> windows_sys::core::BOOL);
#[cfg(all(feature = "minwindef", feature = "shared"))]
windows_link::link!("kernel32.dll" "system" "K32EnumProcesses" fn EnumProcesses(lpidprocess : *mut super::DWORD, cb : super::DWORD, lpcbneeded : super::LPDWORD) -> windows_sys::core::BOOL);
#[cfg(all(feature = "minwindef", feature = "shared"))]
windows_link::link!("kernel32.dll" "system" "K32GetDeviceDriverBaseNameA" fn GetDeviceDriverBaseNameA(imagebase : super::LPVOID, lpfilename : windows_sys::core::PSTR, nsize : super::DWORD) -> super::DWORD);
#[cfg(all(feature = "minwindef", feature = "shared"))]
windows_link::link!("kernel32.dll" "system" "K32GetDeviceDriverBaseNameW" fn GetDeviceDriverBaseNameW(imagebase : super::LPVOID, lpbasename : windows_sys::core::PWSTR, nsize : super::DWORD) -> super::DWORD);
#[cfg(all(feature = "minwindef", feature = "shared"))]
windows_link::link!("kernel32.dll" "system" "K32GetDeviceDriverFileNameA" fn GetDeviceDriverFileNameA(imagebase : super::LPVOID, lpfilename : windows_sys::core::PSTR, nsize : super::DWORD) -> super::DWORD);
#[cfg(all(feature = "minwindef", feature = "shared"))]
windows_link::link!("kernel32.dll" "system" "K32GetDeviceDriverFileNameW" fn GetDeviceDriverFileNameW(imagebase : super::LPVOID, lpfilename : windows_sys::core::PWSTR, nsize : super::DWORD) -> super::DWORD);
#[cfg(all(feature = "minwindef", feature = "shared", feature = "winnt"))]
windows_link::link!("kernel32.dll" "system" "K32GetMappedFileNameA" fn GetMappedFileNameA(hprocess : super::HANDLE, lpv : super::LPVOID, lpfilename : windows_sys::core::PSTR, nsize : super::DWORD) -> super::DWORD);
#[cfg(all(feature = "minwindef", feature = "shared", feature = "winnt"))]
windows_link::link!("kernel32.dll" "system" "K32GetMappedFileNameW" fn GetMappedFileNameW(hprocess : super::HANDLE, lpv : super::LPVOID, lpfilename : windows_sys::core::PWSTR, nsize : super::DWORD) -> super::DWORD);
#[cfg(all(feature = "minwindef", feature = "shared", feature = "winnt"))]
windows_link::link!("kernel32.dll" "system" "K32GetModuleBaseNameA" fn GetModuleBaseNameA(hprocess : super::HANDLE, hmodule : super::HMODULE, lpbasename : windows_sys::core::PSTR, nsize : super::DWORD) -> super::DWORD);
#[cfg(all(feature = "minwindef", feature = "shared", feature = "winnt"))]
windows_link::link!("kernel32.dll" "system" "K32GetModuleBaseNameW" fn GetModuleBaseNameW(hprocess : super::HANDLE, hmodule : super::HMODULE, lpbasename : windows_sys::core::PWSTR, nsize : super::DWORD) -> super::DWORD);
#[cfg(all(feature = "minwindef", feature = "shared", feature = "winnt"))]
windows_link::link!("kernel32.dll" "system" "K32GetModuleFileNameExA" fn GetModuleFileNameExA(hprocess : super::HANDLE, hmodule : super::HMODULE, lpfilename : windows_sys::core::PSTR, nsize : super::DWORD) -> super::DWORD);
#[cfg(all(feature = "minwindef", feature = "shared", feature = "winnt"))]
windows_link::link!("kernel32.dll" "system" "K32GetModuleFileNameExW" fn GetModuleFileNameExW(hprocess : super::HANDLE, hmodule : super::HMODULE, lpfilename : windows_sys::core::PWSTR, nsize : super::DWORD) -> super::DWORD);
#[cfg(all(feature = "minwindef", feature = "shared", feature = "winnt"))]
windows_link::link!("kernel32.dll" "system" "K32GetModuleInformation" fn GetModuleInformation(hprocess : super::HANDLE, hmodule : super::HMODULE, lpmodinfo : LPMODULEINFO, cb : super::DWORD) -> windows_sys::core::BOOL);
#[cfg(feature = "shared")]
windows_link::link!("kernel32.dll" "system" "K32GetPerformanceInfo" fn GetPerformanceInfo(pperformanceinformation : PPERFORMANCE_INFORMATION, cb : super::DWORD) -> windows_sys::core::BOOL);
#[cfg(all(feature = "shared", feature = "winnt"))]
windows_link::link!("kernel32.dll" "system" "K32GetProcessImageFileNameA" fn GetProcessImageFileNameA(hprocess : super::HANDLE, lpimagefilename : windows_sys::core::PSTR, nsize : super::DWORD) -> super::DWORD);
#[cfg(all(feature = "shared", feature = "winnt"))]
windows_link::link!("kernel32.dll" "system" "K32GetProcessImageFileNameW" fn GetProcessImageFileNameW(hprocess : super::HANDLE, lpimagefilename : windows_sys::core::PWSTR, nsize : super::DWORD) -> super::DWORD);
#[cfg(all(feature = "shared", feature = "winnt"))]
windows_link::link!("kernel32.dll" "system" "K32GetProcessMemoryInfo" fn GetProcessMemoryInfo(process : super::HANDLE, ppsmemcounters : PPROCESS_MEMORY_COUNTERS, cb : super::DWORD) -> windows_sys::core::BOOL);
#[cfg(all(feature = "minwindef", feature = "shared", feature = "winnt"))]
windows_link::link!("kernel32.dll" "system" "K32GetWsChanges" fn GetWsChanges(hprocess : super::HANDLE, lpwatchinfo : PPSAPI_WS_WATCH_INFORMATION, cb : super::DWORD) -> windows_sys::core::BOOL);
#[cfg(all(feature = "minwindef", feature = "shared", feature = "winnt"))]
windows_link::link!("kernel32.dll" "system" "K32GetWsChangesEx" fn GetWsChangesEx(hprocess : super::HANDLE, lpwatchinfoex : PPSAPI_WS_WATCH_INFORMATION_EX, cb : super::PDWORD) -> windows_sys::core::BOOL);
#[cfg(feature = "winnt")]
windows_link::link!("kernel32.dll" "system" "K32InitializeProcessForWsWatch" fn InitializeProcessForWsWatch(hprocess : super::HANDLE) -> windows_sys::core::BOOL);
#[cfg(feature = "winnt")]
windows_link::link!("kernel32.dll" "system" fn K32EmptyWorkingSet(hprocess : super::HANDLE) -> windows_sys::core::BOOL);
#[cfg(all(feature = "minwindef", feature = "shared"))]
windows_link::link!("kernel32.dll" "system" fn K32EnumDeviceDrivers(lpimagebase : *mut super::LPVOID, cb : super::DWORD, lpcbneeded : super::LPDWORD) -> windows_sys::core::BOOL);
#[cfg(all(feature = "minwindef", feature = "shared"))]
windows_link::link!("kernel32.dll" "system" fn K32EnumPageFilesA(pcallbackroutine : PENUM_PAGE_FILE_CALLBACKA, pcontext : super::LPVOID) -> windows_sys::core::BOOL);
#[cfg(all(feature = "minwindef", feature = "shared"))]
windows_link::link!("kernel32.dll" "system" fn K32EnumPageFilesW(pcallbackroutine : PENUM_PAGE_FILE_CALLBACKW, pcontext : super::LPVOID) -> windows_sys::core::BOOL);
#[cfg(all(feature = "minwindef", feature = "shared", feature = "winnt"))]
windows_link::link!("kernel32.dll" "system" fn K32EnumProcessModules(hprocess : super::HANDLE, lphmodule : *mut super::HMODULE, cb : super::DWORD, lpcbneeded : super::LPDWORD) -> windows_sys::core::BOOL);
#[cfg(all(feature = "minwindef", feature = "shared", feature = "winnt"))]
windows_link::link!("kernel32.dll" "system" fn K32EnumProcessModulesEx(hprocess : super::HANDLE, lphmodule : *mut super::HMODULE, cb : super::DWORD, lpcbneeded : super::LPDWORD, dwfilterflag : super::DWORD) -> windows_sys::core::BOOL);
#[cfg(all(feature = "minwindef", feature = "shared"))]
windows_link::link!("kernel32.dll" "system" fn K32EnumProcesses(lpidprocess : *mut super::DWORD, cb : super::DWORD, lpcbneeded : super::LPDWORD) -> windows_sys::core::BOOL);
#[cfg(all(feature = "minwindef", feature = "shared"))]
windows_link::link!("kernel32.dll" "system" fn K32GetDeviceDriverBaseNameA(imagebase : super::LPVOID, lpfilename : windows_sys::core::PSTR, nsize : super::DWORD) -> super::DWORD);
#[cfg(all(feature = "minwindef", feature = "shared"))]
windows_link::link!("kernel32.dll" "system" fn K32GetDeviceDriverBaseNameW(imagebase : super::LPVOID, lpbasename : windows_sys::core::PWSTR, nsize : super::DWORD) -> super::DWORD);
#[cfg(all(feature = "minwindef", feature = "shared"))]
windows_link::link!("kernel32.dll" "system" fn K32GetDeviceDriverFileNameA(imagebase : super::LPVOID, lpfilename : windows_sys::core::PSTR, nsize : super::DWORD) -> super::DWORD);
#[cfg(all(feature = "minwindef", feature = "shared"))]
windows_link::link!("kernel32.dll" "system" fn K32GetDeviceDriverFileNameW(imagebase : super::LPVOID, lpfilename : windows_sys::core::PWSTR, nsize : super::DWORD) -> super::DWORD);
#[cfg(all(feature = "minwindef", feature = "shared", feature = "winnt"))]
windows_link::link!("kernel32.dll" "system" fn K32GetMappedFileNameA(hprocess : super::HANDLE, lpv : super::LPVOID, lpfilename : windows_sys::core::PSTR, nsize : super::DWORD) -> super::DWORD);
#[cfg(all(feature = "minwindef", feature = "shared", feature = "winnt"))]
windows_link::link!("kernel32.dll" "system" fn K32GetMappedFileNameW(hprocess : super::HANDLE, lpv : super::LPVOID, lpfilename : windows_sys::core::PWSTR, nsize : super::DWORD) -> super::DWORD);
#[cfg(all(feature = "minwindef", feature = "shared", feature = "winnt"))]
windows_link::link!("kernel32.dll" "system" fn K32GetModuleBaseNameA(hprocess : super::HANDLE, hmodule : super::HMODULE, lpbasename : windows_sys::core::PSTR, nsize : super::DWORD) -> super::DWORD);
#[cfg(all(feature = "minwindef", feature = "shared", feature = "winnt"))]
windows_link::link!("kernel32.dll" "system" fn K32GetModuleBaseNameW(hprocess : super::HANDLE, hmodule : super::HMODULE, lpbasename : windows_sys::core::PWSTR, nsize : super::DWORD) -> super::DWORD);
#[cfg(all(feature = "minwindef", feature = "shared", feature = "winnt"))]
windows_link::link!("kernel32.dll" "system" fn K32GetModuleFileNameExA(hprocess : super::HANDLE, hmodule : super::HMODULE, lpfilename : windows_sys::core::PSTR, nsize : super::DWORD) -> super::DWORD);
#[cfg(all(feature = "minwindef", feature = "shared", feature = "winnt"))]
windows_link::link!("kernel32.dll" "system" fn K32GetModuleFileNameExW(hprocess : super::HANDLE, hmodule : super::HMODULE, lpfilename : windows_sys::core::PWSTR, nsize : super::DWORD) -> super::DWORD);
#[cfg(all(feature = "minwindef", feature = "shared", feature = "winnt"))]
windows_link::link!("kernel32.dll" "system" fn K32GetModuleInformation(hprocess : super::HANDLE, hmodule : super::HMODULE, lpmodinfo : LPMODULEINFO, cb : super::DWORD) -> windows_sys::core::BOOL);
#[cfg(feature = "shared")]
windows_link::link!("kernel32.dll" "system" fn K32GetPerformanceInfo(pperformanceinformation : PPERFORMANCE_INFORMATION, cb : super::DWORD) -> windows_sys::core::BOOL);
#[cfg(all(feature = "shared", feature = "winnt"))]
windows_link::link!("kernel32.dll" "system" fn K32GetProcessImageFileNameA(hprocess : super::HANDLE, lpimagefilename : windows_sys::core::PSTR, nsize : super::DWORD) -> super::DWORD);
#[cfg(all(feature = "shared", feature = "winnt"))]
windows_link::link!("kernel32.dll" "system" fn K32GetProcessImageFileNameW(hprocess : super::HANDLE, lpimagefilename : windows_sys::core::PWSTR, nsize : super::DWORD) -> super::DWORD);
#[cfg(all(feature = "shared", feature = "winnt"))]
windows_link::link!("kernel32.dll" "system" fn K32GetProcessMemoryInfo(process : super::HANDLE, ppsmemcounters : PPROCESS_MEMORY_COUNTERS, cb : super::DWORD) -> windows_sys::core::BOOL);
#[cfg(all(feature = "minwindef", feature = "shared", feature = "winnt"))]
windows_link::link!("kernel32.dll" "system" fn K32GetWsChanges(hprocess : super::HANDLE, lpwatchinfo : PPSAPI_WS_WATCH_INFORMATION, cb : super::DWORD) -> windows_sys::core::BOOL);
#[cfg(all(feature = "minwindef", feature = "shared", feature = "winnt"))]
windows_link::link!("kernel32.dll" "system" fn K32GetWsChangesEx(hprocess : super::HANDLE, lpwatchinfoex : PPSAPI_WS_WATCH_INFORMATION_EX, cb : super::PDWORD) -> windows_sys::core::BOOL);
#[cfg(feature = "winnt")]
windows_link::link!("kernel32.dll" "system" fn K32InitializeProcessForWsWatch(hprocess : super::HANDLE) -> windows_sys::core::BOOL);
#[cfg(all(feature = "shared", feature = "winnt"))]
windows_link::link!("kernel32.dll" "system" fn K32QueryWorkingSet(hprocess : super::HANDLE, pv : super::PVOID, cb : super::DWORD) -> windows_sys::core::BOOL);
#[cfg(all(feature = "shared", feature = "winnt"))]
windows_link::link!("kernel32.dll" "system" fn K32QueryWorkingSetEx(hprocess : super::HANDLE, pv : super::PVOID, cb : super::DWORD) -> windows_sys::core::BOOL);
#[cfg(all(feature = "shared", feature = "winnt"))]
windows_link::link!("kernel32.dll" "system" "K32QueryWorkingSet" fn QueryWorkingSet(hprocess : super::HANDLE, pv : super::PVOID, cb : super::DWORD) -> windows_sys::core::BOOL);
#[cfg(all(feature = "shared", feature = "winnt"))]
windows_link::link!("kernel32.dll" "system" "K32QueryWorkingSetEx" fn QueryWorkingSetEx(hprocess : super::HANDLE, pv : super::PVOID, cb : super::DWORD) -> windows_sys::core::BOOL);
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Default)]
pub struct ENUM_PAGE_FILE_INFORMATION {
    pub cb: super::DWORD,
    pub Reserved: super::DWORD,
    pub TotalSize: super::SIZE_T,
    pub TotalInUse: super::SIZE_T,
    pub PeakUsage: super::SIZE_T,
}
pub const LIST_MODULES_32BIT: i32 = 1;
pub const LIST_MODULES_64BIT: i32 = 2;
pub const LIST_MODULES_ALL: i32 = 3;
pub const LIST_MODULES_DEFAULT: i32 = 0;
#[cfg(all(feature = "minwindef", feature = "shared"))]
pub type LPMODULEINFO = *mut MODULEINFO;
#[repr(C)]
#[cfg(all(feature = "minwindef", feature = "shared"))]
#[derive(Clone, Copy, Default)]
pub struct MODULEINFO {
    pub lpBaseOfDll: super::LPVOID,
    pub SizeOfImage: super::DWORD,
    pub EntryPoint: super::LPVOID,
}
#[cfg(all(feature = "minwindef", feature = "shared"))]
pub type PENUM_PAGE_FILE_CALLBACKA = Option<unsafe extern "system" fn(pcontext: super::LPVOID, ppagefileinfo: PENUM_PAGE_FILE_INFORMATION, lpfilename: windows_sys::core::PCSTR) -> windows_sys::core::BOOL>;
#[cfg(all(feature = "minwindef", feature = "shared"))]
pub type PENUM_PAGE_FILE_CALLBACKW = Option<unsafe extern "system" fn(pcontext: super::LPVOID, ppagefileinfo: PENUM_PAGE_FILE_INFORMATION, lpfilename: windows_sys::core::PCWSTR) -> windows_sys::core::BOOL>;
#[cfg(feature = "shared")]
pub type PENUM_PAGE_FILE_INFORMATION = *mut ENUM_PAGE_FILE_INFORMATION;
#[cfg(feature = "shared")]
pub type PERFORMACE_INFORMATION = _PERFORMANCE_INFORMATION;
#[cfg(feature = "shared")]
pub type PERFORMANCE_INFORMATION = _PERFORMANCE_INFORMATION;
#[cfg(feature = "shared")]
pub type PPERFORMACE_INFORMATION = *mut _PERFORMANCE_INFORMATION;
#[cfg(feature = "shared")]
pub type PPERFORMANCE_INFORMATION = *mut _PERFORMANCE_INFORMATION;
#[cfg(feature = "shared")]
pub type PPROCESS_MEMORY_COUNTERS = *mut PROCESS_MEMORY_COUNTERS;
#[cfg(feature = "shared")]
pub type PPROCESS_MEMORY_COUNTERS_EX = *mut PROCESS_MEMORY_COUNTERS_EX;
#[cfg(feature = "shared")]
pub type PPROCESS_MEMORY_COUNTERS_EX2 = *mut PROCESS_MEMORY_COUNTERS_EX2;
#[cfg(feature = "shared")]
pub type PPSAPI_WORKING_SET_BLOCK = *mut PSAPI_WORKING_SET_BLOCK;
#[cfg(feature = "shared")]
pub type PPSAPI_WORKING_SET_EX_BLOCK = *mut PSAPI_WORKING_SET_EX_BLOCK;
#[cfg(all(feature = "shared", feature = "winnt"))]
pub type PPSAPI_WORKING_SET_EX_INFORMATION = *mut PSAPI_WORKING_SET_EX_INFORMATION;
#[cfg(feature = "shared")]
pub type PPSAPI_WORKING_SET_INFORMATION = *mut PSAPI_WORKING_SET_INFORMATION;
#[cfg(feature = "minwindef")]
pub type PPSAPI_WS_WATCH_INFORMATION = *mut PSAPI_WS_WATCH_INFORMATION;
#[cfg(all(feature = "minwindef", feature = "shared"))]
pub type PPSAPI_WS_WATCH_INFORMATION_EX = *mut PSAPI_WS_WATCH_INFORMATION_EX;
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Default)]
pub struct PROCESS_MEMORY_COUNTERS {
    pub cb: super::DWORD,
    pub PageFaultCount: super::DWORD,
    pub PeakWorkingSetSize: super::SIZE_T,
    pub WorkingSetSize: super::SIZE_T,
    pub QuotaPeakPagedPoolUsage: super::SIZE_T,
    pub QuotaPagedPoolUsage: super::SIZE_T,
    pub QuotaPeakNonPagedPoolUsage: super::SIZE_T,
    pub QuotaNonPagedPoolUsage: super::SIZE_T,
    pub PagefileUsage: super::SIZE_T,
    pub PeakPagefileUsage: super::SIZE_T,
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Default)]
pub struct PROCESS_MEMORY_COUNTERS_EX {
    pub cb: super::DWORD,
    pub PageFaultCount: super::DWORD,
    pub PeakWorkingSetSize: super::SIZE_T,
    pub WorkingSetSize: super::SIZE_T,
    pub QuotaPeakPagedPoolUsage: super::SIZE_T,
    pub QuotaPagedPoolUsage: super::SIZE_T,
    pub QuotaPeakNonPagedPoolUsage: super::SIZE_T,
    pub QuotaNonPagedPoolUsage: super::SIZE_T,
    pub PagefileUsage: super::SIZE_T,
    pub PeakPagefileUsage: super::SIZE_T,
    pub PrivateUsage: super::SIZE_T,
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Default)]
pub struct PROCESS_MEMORY_COUNTERS_EX2 {
    pub cb: super::DWORD,
    pub PageFaultCount: super::DWORD,
    pub PeakWorkingSetSize: super::SIZE_T,
    pub WorkingSetSize: super::SIZE_T,
    pub QuotaPeakPagedPoolUsage: super::SIZE_T,
    pub QuotaPagedPoolUsage: super::SIZE_T,
    pub QuotaPeakNonPagedPoolUsage: super::SIZE_T,
    pub QuotaNonPagedPoolUsage: super::SIZE_T,
    pub PagefileUsage: super::SIZE_T,
    pub PeakPagefileUsage: super::SIZE_T,
    pub PrivateUsage: super::SIZE_T,
    pub PrivateWorkingSetSize: super::SIZE_T,
    pub SharedCommitUsage: super::ULONG64,
}
pub const PSAPI_VERSION: i32 = 2;
#[repr(C)]
#[cfg(target_arch = "x86")]
#[cfg(feature = "shared")]
#[derive(Clone, Copy)]
pub union PSAPI_WORKING_SET_BLOCK {
    pub Flags: super::ULONG_PTR,
    pub Anonymous: PSAPI_WORKING_SET_BLOCK_0,
}
#[cfg(target_arch = "x86")]
#[cfg(feature = "shared")]
impl Default for PSAPI_WORKING_SET_BLOCK {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(target_arch = "x86")]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Default)]
pub struct PSAPI_WORKING_SET_BLOCK_0 {
    pub _bitfield: u32,
}
#[repr(C)]
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[cfg(feature = "shared")]
#[derive(Clone, Copy)]
pub union PSAPI_WORKING_SET_BLOCK {
    pub Flags: super::ULONG_PTR,
    pub Anonymous: PSAPI_WORKING_SET_BLOCK_0,
}
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[cfg(feature = "shared")]
impl Default for PSAPI_WORKING_SET_BLOCK {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Default)]
pub struct PSAPI_WORKING_SET_BLOCK_0 {
    pub _bitfield: u64,
}
#[repr(C)]
#[cfg(target_arch = "x86")]
#[cfg(feature = "shared")]
#[derive(Clone, Copy)]
pub union PSAPI_WORKING_SET_EX_BLOCK {
    pub Flags: super::ULONG_PTR,
    pub Anonymous: PSAPI_WORKING_SET_EX_BLOCK_0,
}
#[cfg(target_arch = "x86")]
#[cfg(feature = "shared")]
impl Default for PSAPI_WORKING_SET_EX_BLOCK {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(target_arch = "x86")]
#[cfg(feature = "shared")]
#[derive(Clone, Copy)]
pub union PSAPI_WORKING_SET_EX_BLOCK_0 {
    pub Anonymous: PSAPI_WORKING_SET_EX_BLOCK_0_0,
    pub Invalid: PSAPI_WORKING_SET_EX_BLOCK_0_1,
}
#[cfg(target_arch = "x86")]
#[cfg(feature = "shared")]
impl Default for PSAPI_WORKING_SET_EX_BLOCK_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(target_arch = "x86")]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Default)]
pub struct PSAPI_WORKING_SET_EX_BLOCK_0_0 {
    pub _bitfield: u32,
}
#[repr(C)]
#[cfg(target_arch = "x86")]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Default)]
pub struct PSAPI_WORKING_SET_EX_BLOCK_0_1 {
    pub _bitfield: u32,
}
#[repr(C)]
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[cfg(feature = "shared")]
#[derive(Clone, Copy)]
pub union PSAPI_WORKING_SET_EX_BLOCK {
    pub Flags: super::ULONG_PTR,
    pub Anonymous: PSAPI_WORKING_SET_EX_BLOCK_0,
}
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[cfg(feature = "shared")]
impl Default for PSAPI_WORKING_SET_EX_BLOCK {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[cfg(feature = "shared")]
#[derive(Clone, Copy)]
pub union PSAPI_WORKING_SET_EX_BLOCK_0 {
    pub Anonymous: PSAPI_WORKING_SET_EX_BLOCK_0_0,
    pub Invalid: PSAPI_WORKING_SET_EX_BLOCK_0_1,
}
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[cfg(feature = "shared")]
impl Default for PSAPI_WORKING_SET_EX_BLOCK_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Default)]
pub struct PSAPI_WORKING_SET_EX_BLOCK_0_0 {
    pub _bitfield: u64,
}
#[repr(C)]
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Default)]
pub struct PSAPI_WORKING_SET_EX_BLOCK_0_1 {
    pub _bitfield: u64,
}
#[repr(C)]
#[cfg(all(feature = "shared", feature = "winnt"))]
#[derive(Clone, Copy)]
pub struct PSAPI_WORKING_SET_EX_INFORMATION {
    pub VirtualAddress: super::PVOID,
    pub VirtualAttributes: PSAPI_WORKING_SET_EX_BLOCK,
}
#[cfg(all(feature = "shared", feature = "winnt"))]
impl Default for PSAPI_WORKING_SET_EX_INFORMATION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy)]
pub struct PSAPI_WORKING_SET_INFORMATION {
    pub NumberOfEntries: super::ULONG_PTR,
    pub WorkingSetInfo: [PSAPI_WORKING_SET_BLOCK; 1],
}
#[cfg(feature = "shared")]
impl Default for PSAPI_WORKING_SET_INFORMATION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "minwindef")]
#[derive(Clone, Copy, Default)]
pub struct PSAPI_WS_WATCH_INFORMATION {
    pub FaultingPc: super::LPVOID,
    pub FaultingVa: super::LPVOID,
}
#[repr(C)]
#[cfg(all(feature = "minwindef", feature = "shared"))]
#[derive(Clone, Copy, Default)]
pub struct PSAPI_WS_WATCH_INFORMATION_EX {
    pub BasicInfo: PSAPI_WS_WATCH_INFORMATION,
    pub FaultingThreadId: super::ULONG_PTR,
    pub Flags: super::ULONG_PTR,
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Default)]
pub struct _PERFORMANCE_INFORMATION {
    pub cb: super::DWORD,
    pub CommitTotal: super::SIZE_T,
    pub CommitLimit: super::SIZE_T,
    pub CommitPeak: super::SIZE_T,
    pub PhysicalTotal: super::SIZE_T,
    pub PhysicalAvailable: super::SIZE_T,
    pub SystemCache: super::SIZE_T,
    pub KernelTotal: super::SIZE_T,
    pub KernelPaged: super::SIZE_T,
    pub KernelNonpaged: super::SIZE_T,
    pub PageSize: super::SIZE_T,
    pub HandleCount: super::DWORD,
    pub ProcessCount: super::DWORD,
    pub ThreadCount: super::DWORD,
}

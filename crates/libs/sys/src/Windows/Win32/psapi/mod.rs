#[cfg(feature = "winnt")]
windows_link::link!("kernel32.dll" "system" "K32EmptyWorkingSet" fn EmptyWorkingSet(hprocess : super::HANDLE) -> windows_sys::core::BOOL);
windows_link::link!("kernel32.dll" "system" "K32EnumDeviceDrivers" fn EnumDeviceDrivers(lpimagebase : *mut *mut core::ffi::c_void, cb : u32, lpcbneeded : *mut u32) -> windows_sys::core::BOOL);
windows_link::link!("kernel32.dll" "system" "K32EnumPageFilesA" fn EnumPageFilesA(pcallbackroutine : PENUM_PAGE_FILE_CALLBACKA, pcontext : *mut core::ffi::c_void) -> windows_sys::core::BOOL);
windows_link::link!("kernel32.dll" "system" "K32EnumPageFilesW" fn EnumPageFilesW(pcallbackroutine : PENUM_PAGE_FILE_CALLBACKW, pcontext : *mut core::ffi::c_void) -> windows_sys::core::BOOL);
#[cfg(all(feature = "minwindef", feature = "winnt"))]
windows_link::link!("kernel32.dll" "system" "K32EnumProcessModules" fn EnumProcessModules(hprocess : super::HANDLE, lphmodule : *mut super::HMODULE, cb : u32, lpcbneeded : *mut u32) -> windows_sys::core::BOOL);
#[cfg(all(feature = "minwindef", feature = "winnt"))]
windows_link::link!("kernel32.dll" "system" "K32EnumProcessModulesEx" fn EnumProcessModulesEx(hprocess : super::HANDLE, lphmodule : *mut super::HMODULE, cb : u32, lpcbneeded : *mut u32, dwfilterflag : u32) -> windows_sys::core::BOOL);
windows_link::link!("kernel32.dll" "system" "K32EnumProcesses" fn EnumProcesses(lpidprocess : *mut u32, cb : u32, lpcbneeded : *mut u32) -> windows_sys::core::BOOL);
windows_link::link!("kernel32.dll" "system" "K32GetDeviceDriverBaseNameA" fn GetDeviceDriverBaseNameA(imagebase : *const core::ffi::c_void, lpfilename : windows_sys::core::PSTR, nsize : u32) -> u32);
windows_link::link!("kernel32.dll" "system" "K32GetDeviceDriverBaseNameW" fn GetDeviceDriverBaseNameW(imagebase : *const core::ffi::c_void, lpbasename : windows_sys::core::PWSTR, nsize : u32) -> u32);
windows_link::link!("kernel32.dll" "system" "K32GetDeviceDriverFileNameA" fn GetDeviceDriverFileNameA(imagebase : *const core::ffi::c_void, lpfilename : windows_sys::core::PSTR, nsize : u32) -> u32);
windows_link::link!("kernel32.dll" "system" "K32GetDeviceDriverFileNameW" fn GetDeviceDriverFileNameW(imagebase : *const core::ffi::c_void, lpfilename : windows_sys::core::PWSTR, nsize : u32) -> u32);
#[cfg(feature = "winnt")]
windows_link::link!("kernel32.dll" "system" "K32GetMappedFileNameA" fn GetMappedFileNameA(hprocess : super::HANDLE, lpv : *const core::ffi::c_void, lpfilename : windows_sys::core::PSTR, nsize : u32) -> u32);
#[cfg(feature = "winnt")]
windows_link::link!("kernel32.dll" "system" "K32GetMappedFileNameW" fn GetMappedFileNameW(hprocess : super::HANDLE, lpv : *const core::ffi::c_void, lpfilename : windows_sys::core::PWSTR, nsize : u32) -> u32);
#[cfg(all(feature = "minwindef", feature = "winnt"))]
windows_link::link!("kernel32.dll" "system" "K32GetModuleBaseNameA" fn GetModuleBaseNameA(hprocess : super::HANDLE, hmodule : super::HMODULE, lpbasename : windows_sys::core::PSTR, nsize : u32) -> u32);
#[cfg(all(feature = "minwindef", feature = "winnt"))]
windows_link::link!("kernel32.dll" "system" "K32GetModuleBaseNameW" fn GetModuleBaseNameW(hprocess : super::HANDLE, hmodule : super::HMODULE, lpbasename : windows_sys::core::PWSTR, nsize : u32) -> u32);
#[cfg(all(feature = "minwindef", feature = "winnt"))]
windows_link::link!("kernel32.dll" "system" "K32GetModuleFileNameExA" fn GetModuleFileNameExA(hprocess : super::HANDLE, hmodule : super::HMODULE, lpfilename : windows_sys::core::PCSTR, nsize : u32) -> u32);
#[cfg(all(feature = "minwindef", feature = "winnt"))]
windows_link::link!("kernel32.dll" "system" "K32GetModuleFileNameExW" fn GetModuleFileNameExW(hprocess : super::HANDLE, hmodule : super::HMODULE, lpfilename : windows_sys::core::PCWSTR, nsize : u32) -> u32);
#[cfg(all(feature = "minwindef", feature = "winnt"))]
windows_link::link!("kernel32.dll" "system" "K32GetModuleInformation" fn GetModuleInformation(hprocess : super::HANDLE, hmodule : super::HMODULE, lpmodinfo : *mut MODULEINFO, cb : u32) -> windows_sys::core::BOOL);
windows_link::link!("kernel32.dll" "system" "K32GetPerformanceInfo" fn GetPerformanceInfo(pperformanceinformation : *mut PERFORMANCE_INFORMATION, cb : u32) -> windows_sys::core::BOOL);
#[cfg(feature = "winnt")]
windows_link::link!("kernel32.dll" "system" "K32GetProcessImageFileNameA" fn GetProcessImageFileNameA(hprocess : super::HANDLE, lpimagefilename : windows_sys::core::PSTR, nsize : u32) -> u32);
#[cfg(feature = "winnt")]
windows_link::link!("kernel32.dll" "system" "K32GetProcessImageFileNameW" fn GetProcessImageFileNameW(hprocess : super::HANDLE, lpimagefilename : windows_sys::core::PWSTR, nsize : u32) -> u32);
#[cfg(feature = "winnt")]
windows_link::link!("kernel32.dll" "system" "K32GetProcessMemoryInfo" fn GetProcessMemoryInfo(process : super::HANDLE, ppsmemcounters : *mut PROCESS_MEMORY_COUNTERS, cb : u32) -> windows_sys::core::BOOL);
#[cfg(feature = "winnt")]
windows_link::link!("kernel32.dll" "system" "K32GetWsChanges" fn GetWsChanges(hprocess : super::HANDLE, lpwatchinfo : *mut PSAPI_WS_WATCH_INFORMATION, cb : u32) -> windows_sys::core::BOOL);
#[cfg(feature = "winnt")]
windows_link::link!("kernel32.dll" "system" "K32GetWsChangesEx" fn GetWsChangesEx(hprocess : super::HANDLE, lpwatchinfoex : *mut PSAPI_WS_WATCH_INFORMATION_EX, cb : *mut u32) -> windows_sys::core::BOOL);
#[cfg(feature = "winnt")]
windows_link::link!("kernel32.dll" "system" "K32InitializeProcessForWsWatch" fn InitializeProcessForWsWatch(hprocess : super::HANDLE) -> windows_sys::core::BOOL);
#[cfg(feature = "winnt")]
windows_link::link!("kernel32.dll" "system" "K32QueryWorkingSet" fn QueryWorkingSet(hprocess : super::HANDLE, pv : *mut core::ffi::c_void, cb : u32) -> windows_sys::core::BOOL);
#[cfg(feature = "winnt")]
windows_link::link!("kernel32.dll" "system" "K32QueryWorkingSetEx" fn QueryWorkingSetEx(hprocess : super::HANDLE, pv : *mut core::ffi::c_void, cb : u32) -> windows_sys::core::BOOL);
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct ENUM_PAGE_FILE_INFORMATION {
    pub cb: u32,
    pub Reserved: u32,
    pub TotalSize: usize,
    pub TotalInUse: usize,
    pub PeakUsage: usize,
}
pub const LIST_MODULES_32BIT: u32 = 1;
pub const LIST_MODULES_64BIT: u32 = 2;
pub const LIST_MODULES_ALL: u32 = 3;
pub const LIST_MODULES_DEFAULT: u32 = 0;
pub type LPMODULEINFO = *mut MODULEINFO;
#[repr(C)]
#[derive(Clone, Copy)]
pub struct MODULEINFO {
    pub lpBaseOfDll: *mut core::ffi::c_void,
    pub SizeOfImage: u32,
    pub EntryPoint: *mut core::ffi::c_void,
}
impl Default for MODULEINFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub type PENUM_PAGE_FILE_CALLBACKA = Option<unsafe extern "system" fn(pcontext: *mut core::ffi::c_void, ppagefileinfo: *mut ENUM_PAGE_FILE_INFORMATION, lpfilename: windows_sys::core::PCSTR) -> windows_sys::core::BOOL>;
pub type PENUM_PAGE_FILE_CALLBACKW = Option<unsafe extern "system" fn(pcontext: *mut core::ffi::c_void, ppagefileinfo: *mut ENUM_PAGE_FILE_INFORMATION, lpfilename: windows_sys::core::PCWSTR) -> windows_sys::core::BOOL>;
pub type PENUM_PAGE_FILE_INFORMATION = *mut ENUM_PAGE_FILE_INFORMATION;
pub type PERFORMACE_INFORMATION = PERFORMANCE_INFORMATION;
#[repr(C)]
#[derive(Clone, Copy, Default)]
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
pub type PPERFORMACE_INFORMATION = *mut PERFORMANCE_INFORMATION;
pub type PPERFORMANCE_INFORMATION = *mut PERFORMANCE_INFORMATION;
pub type PPROCESS_MEMORY_COUNTERS = *mut PROCESS_MEMORY_COUNTERS;
pub type PPROCESS_MEMORY_COUNTERS_EX = *mut PROCESS_MEMORY_COUNTERS_EX;
pub type PPROCESS_MEMORY_COUNTERS_EX2 = *mut PROCESS_MEMORY_COUNTERS_EX2;
pub type PPSAPI_WORKING_SET_BLOCK = *mut PSAPI_WORKING_SET_BLOCK;
pub type PPSAPI_WORKING_SET_EX_BLOCK = *mut PSAPI_WORKING_SET_EX_BLOCK;
pub type PPSAPI_WORKING_SET_EX_INFORMATION = *mut PSAPI_WORKING_SET_EX_INFORMATION;
pub type PPSAPI_WORKING_SET_INFORMATION = *mut PSAPI_WORKING_SET_INFORMATION;
pub type PPSAPI_WS_WATCH_INFORMATION = *mut PSAPI_WS_WATCH_INFORMATION;
pub type PPSAPI_WS_WATCH_INFORMATION_EX = *mut PSAPI_WS_WATCH_INFORMATION_EX;
#[repr(C)]
#[derive(Clone, Copy, Default)]
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
#[repr(C)]
#[derive(Clone, Copy, Default)]
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
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct PROCESS_MEMORY_COUNTERS_EX2 {
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
    pub PrivateWorkingSetSize: usize,
    pub SharedCommitUsage: u64,
}
pub const PSAPI_VERSION: u32 = 2;
#[repr(C)]
#[derive(Clone, Copy)]
pub union PSAPI_WORKING_SET_BLOCK {
    pub Flags: usize,
    pub Anonymous: PSAPI_WORKING_SET_BLOCK_0,
}
impl Default for PSAPI_WORKING_SET_BLOCK {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct PSAPI_WORKING_SET_BLOCK_0 {
    pub _bitfield: usize,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union PSAPI_WORKING_SET_EX_BLOCK {
    pub Flags: usize,
    pub Anonymous: PSAPI_WORKING_SET_EX_BLOCK_0,
}
impl Default for PSAPI_WORKING_SET_EX_BLOCK {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union PSAPI_WORKING_SET_EX_BLOCK_0 {
    pub Anonymous: PSAPI_WORKING_SET_EX_BLOCK_0_0,
    pub Invalid: PSAPI_WORKING_SET_EX_BLOCK_0_1,
}
impl Default for PSAPI_WORKING_SET_EX_BLOCK_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct PSAPI_WORKING_SET_EX_BLOCK_0_0 {
    pub _bitfield: usize,
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct PSAPI_WORKING_SET_EX_BLOCK_0_1 {
    pub _bitfield: usize,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct PSAPI_WORKING_SET_EX_INFORMATION {
    pub VirtualAddress: *mut core::ffi::c_void,
    pub VirtualAttributes: PSAPI_WORKING_SET_EX_BLOCK,
}
impl Default for PSAPI_WORKING_SET_EX_INFORMATION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct PSAPI_WORKING_SET_INFORMATION {
    pub NumberOfEntries: usize,
    pub WorkingSetInfo: [PSAPI_WORKING_SET_BLOCK; 1],
}
impl Default for PSAPI_WORKING_SET_INFORMATION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct PSAPI_WS_WATCH_INFORMATION {
    pub FaultingPc: *mut core::ffi::c_void,
    pub FaultingVa: *mut core::ffi::c_void,
}
impl Default for PSAPI_WS_WATCH_INFORMATION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct PSAPI_WS_WATCH_INFORMATION_EX {
    pub BasicInfo: PSAPI_WS_WATCH_INFORMATION,
    pub FaultingThreadId: usize,
    pub Flags: usize,
}

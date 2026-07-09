#[cfg(feature = "Win32_winnt")]
windows_link::link!("kernel32.dll" "C" fn K32EmptyWorkingSet(hprocess : super::winnt::HANDLE) -> windows_sys::core::BOOL);
windows_link::link!("kernel32.dll" "C" fn K32EnumDeviceDrivers(lpimagebase : *mut *mut core::ffi::c_void, cb : u32, lpcbneeded : *mut u32) -> windows_sys::core::BOOL);
windows_link::link!("kernel32.dll" "C" fn K32EnumPageFilesA(pcallbackroutine : PENUM_PAGE_FILE_CALLBACKA, pcontext : *mut core::ffi::c_void) -> windows_sys::core::BOOL);
windows_link::link!("kernel32.dll" "C" fn K32EnumPageFilesW(pcallbackroutine : PENUM_PAGE_FILE_CALLBACKW, pcontext : *mut core::ffi::c_void) -> windows_sys::core::BOOL);
#[cfg(all(feature = "Win32_minwindef", feature = "Win32_winnt"))]
windows_link::link!("kernel32.dll" "C" fn K32EnumProcessModules(hprocess : super::winnt::HANDLE, lphmodule : *mut super::minwindef::HMODULE, cb : u32, lpcbneeded : *mut u32) -> windows_sys::core::BOOL);
#[cfg(all(feature = "Win32_minwindef", feature = "Win32_winnt"))]
windows_link::link!("kernel32.dll" "C" fn K32EnumProcessModulesEx(hprocess : super::winnt::HANDLE, lphmodule : *mut super::minwindef::HMODULE, cb : u32, lpcbneeded : *mut u32, dwfilterflag : u32) -> windows_sys::core::BOOL);
windows_link::link!("kernel32.dll" "C" fn K32EnumProcesses(lpidprocess : *mut u32, cb : u32, lpcbneeded : *mut u32) -> windows_sys::core::BOOL);
windows_link::link!("kernel32.dll" "C" fn K32GetDeviceDriverBaseNameA(imagebase : *const core::ffi::c_void, lpfilename : windows_sys::core::PSTR, nsize : u32) -> u32);
windows_link::link!("kernel32.dll" "C" fn K32GetDeviceDriverBaseNameW(imagebase : *const core::ffi::c_void, lpbasename : windows_sys::core::PWSTR, nsize : u32) -> u32);
windows_link::link!("kernel32.dll" "C" fn K32GetDeviceDriverFileNameA(imagebase : *const core::ffi::c_void, lpfilename : windows_sys::core::PSTR, nsize : u32) -> u32);
windows_link::link!("kernel32.dll" "C" fn K32GetDeviceDriverFileNameW(imagebase : *const core::ffi::c_void, lpfilename : windows_sys::core::PWSTR, nsize : u32) -> u32);
#[cfg(feature = "Win32_winnt")]
windows_link::link!("kernel32.dll" "C" fn K32GetMappedFileNameA(hprocess : super::winnt::HANDLE, lpv : *const core::ffi::c_void, lpfilename : windows_sys::core::PSTR, nsize : u32) -> u32);
#[cfg(feature = "Win32_winnt")]
windows_link::link!("kernel32.dll" "C" fn K32GetMappedFileNameW(hprocess : super::winnt::HANDLE, lpv : *const core::ffi::c_void, lpfilename : windows_sys::core::PWSTR, nsize : u32) -> u32);
#[cfg(all(feature = "Win32_minwindef", feature = "Win32_winnt"))]
windows_link::link!("kernel32.dll" "C" fn K32GetModuleBaseNameA(hprocess : super::winnt::HANDLE, hmodule : super::minwindef::HMODULE, lpbasename : windows_sys::core::PSTR, nsize : u32) -> u32);
#[cfg(all(feature = "Win32_minwindef", feature = "Win32_winnt"))]
windows_link::link!("kernel32.dll" "C" fn K32GetModuleBaseNameW(hprocess : super::winnt::HANDLE, hmodule : super::minwindef::HMODULE, lpbasename : windows_sys::core::PWSTR, nsize : u32) -> u32);
#[cfg(all(feature = "Win32_minwindef", feature = "Win32_winnt"))]
windows_link::link!("kernel32.dll" "C" fn K32GetModuleFileNameExA(hprocess : super::winnt::HANDLE, hmodule : super::minwindef::HMODULE, lpfilename : windows_sys::core::PCSTR, nsize : u32) -> u32);
#[cfg(all(feature = "Win32_minwindef", feature = "Win32_winnt"))]
windows_link::link!("kernel32.dll" "C" fn K32GetModuleFileNameExW(hprocess : super::winnt::HANDLE, hmodule : super::minwindef::HMODULE, lpfilename : windows_sys::core::PCWSTR, nsize : u32) -> u32);
#[cfg(all(feature = "Win32_minwindef", feature = "Win32_winnt"))]
windows_link::link!("kernel32.dll" "C" fn K32GetModuleInformation(hprocess : super::winnt::HANDLE, hmodule : super::minwindef::HMODULE, lpmodinfo : *mut MODULEINFO, cb : u32) -> windows_sys::core::BOOL);
windows_link::link!("kernel32.dll" "C" fn K32GetPerformanceInfo(pperformanceinformation : *mut PERFORMANCE_INFORMATION, cb : u32) -> windows_sys::core::BOOL);
#[cfg(feature = "Win32_winnt")]
windows_link::link!("kernel32.dll" "C" fn K32GetProcessImageFileNameA(hprocess : super::winnt::HANDLE, lpimagefilename : windows_sys::core::PSTR, nsize : u32) -> u32);
#[cfg(feature = "Win32_winnt")]
windows_link::link!("kernel32.dll" "C" fn K32GetProcessImageFileNameW(hprocess : super::winnt::HANDLE, lpimagefilename : windows_sys::core::PWSTR, nsize : u32) -> u32);
#[cfg(feature = "Win32_winnt")]
windows_link::link!("kernel32.dll" "C" fn K32GetProcessMemoryInfo(process : super::winnt::HANDLE, ppsmemcounters : *mut PROCESS_MEMORY_COUNTERS, cb : u32) -> windows_sys::core::BOOL);
#[cfg(feature = "Win32_winnt")]
windows_link::link!("kernel32.dll" "C" fn K32GetWsChanges(hprocess : super::winnt::HANDLE, lpwatchinfo : *mut PSAPI_WS_WATCH_INFORMATION, cb : u32) -> windows_sys::core::BOOL);
#[cfg(feature = "Win32_winnt")]
windows_link::link!("kernel32.dll" "C" fn K32GetWsChangesEx(hprocess : super::winnt::HANDLE, lpwatchinfoex : *mut PSAPI_WS_WATCH_INFORMATION_EX, cb : *mut u32) -> windows_sys::core::BOOL);
#[cfg(feature = "Win32_winnt")]
windows_link::link!("kernel32.dll" "C" fn K32InitializeProcessForWsWatch(hprocess : super::winnt::HANDLE) -> windows_sys::core::BOOL);
#[cfg(feature = "Win32_winnt")]
windows_link::link!("kernel32.dll" "C" fn K32QueryWorkingSet(hprocess : super::winnt::HANDLE, pv : *mut core::ffi::c_void, cb : u32) -> windows_sys::core::BOOL);
#[cfg(feature = "Win32_winnt")]
windows_link::link!("kernel32.dll" "C" fn K32QueryWorkingSetEx(hprocess : super::winnt::HANDLE, pv : *mut core::ffi::c_void, cb : u32) -> windows_sys::core::BOOL);
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

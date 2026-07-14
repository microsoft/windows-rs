#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn EmptyWorkingSet(hprocess: super::winnt::HANDLE) -> windows_core::BOOL {
    windows_core::link!("kernel32.dll" "system" "K32EmptyWorkingSet" fn EmptyWorkingSet(hprocess : super::winnt::HANDLE) -> windows_core::BOOL);
    unsafe { EmptyWorkingSet(hprocess) }
}
#[inline]
pub unsafe fn EnumDeviceDrivers(lpimagebase: *mut *mut core::ffi::c_void, cb: u32, lpcbneeded: *mut u32) -> windows_core::BOOL {
    windows_core::link!("kernel32.dll" "system" "K32EnumDeviceDrivers" fn EnumDeviceDrivers(lpimagebase : *mut *mut core::ffi::c_void, cb : u32, lpcbneeded : *mut u32) -> windows_core::BOOL);
    unsafe { EnumDeviceDrivers(lpimagebase as _, cb, lpcbneeded as _) }
}
#[inline]
pub unsafe fn EnumPageFilesA(pcallbackroutine: PENUM_PAGE_FILE_CALLBACKA, pcontext: *mut core::ffi::c_void) -> windows_core::BOOL {
    windows_core::link!("kernel32.dll" "system" "K32EnumPageFilesA" fn EnumPageFilesA(pcallbackroutine : PENUM_PAGE_FILE_CALLBACKA, pcontext : *mut core::ffi::c_void) -> windows_core::BOOL);
    unsafe { EnumPageFilesA(pcallbackroutine, pcontext as _) }
}
#[inline]
pub unsafe fn EnumPageFilesW(pcallbackroutine: PENUM_PAGE_FILE_CALLBACKW, pcontext: *mut core::ffi::c_void) -> windows_core::BOOL {
    windows_core::link!("kernel32.dll" "system" "K32EnumPageFilesW" fn EnumPageFilesW(pcallbackroutine : PENUM_PAGE_FILE_CALLBACKW, pcontext : *mut core::ffi::c_void) -> windows_core::BOOL);
    unsafe { EnumPageFilesW(pcallbackroutine, pcontext as _) }
}
#[cfg(all(feature = "minwindef", feature = "winnt"))]
#[inline]
pub unsafe fn EnumProcessModules(hprocess: super::winnt::HANDLE, lphmodule: *mut super::minwindef::HMODULE, cb: u32, lpcbneeded: *mut u32) -> windows_core::BOOL {
    windows_core::link!("kernel32.dll" "system" "K32EnumProcessModules" fn EnumProcessModules(hprocess : super::winnt::HANDLE, lphmodule : *mut super::minwindef::HMODULE, cb : u32, lpcbneeded : *mut u32) -> windows_core::BOOL);
    unsafe { EnumProcessModules(hprocess, lphmodule as _, cb, lpcbneeded as _) }
}
#[cfg(all(feature = "minwindef", feature = "winnt"))]
#[inline]
pub unsafe fn EnumProcessModulesEx(hprocess: super::winnt::HANDLE, lphmodule: *mut super::minwindef::HMODULE, cb: u32, lpcbneeded: *mut u32, dwfilterflag: u32) -> windows_core::BOOL {
    windows_core::link!("kernel32.dll" "system" "K32EnumProcessModulesEx" fn EnumProcessModulesEx(hprocess : super::winnt::HANDLE, lphmodule : *mut super::minwindef::HMODULE, cb : u32, lpcbneeded : *mut u32, dwfilterflag : u32) -> windows_core::BOOL);
    unsafe { EnumProcessModulesEx(hprocess, lphmodule as _, cb, lpcbneeded as _, dwfilterflag) }
}
#[inline]
pub unsafe fn EnumProcesses(lpidprocess: *mut u32, cb: u32, lpcbneeded: *mut u32) -> windows_core::BOOL {
    windows_core::link!("kernel32.dll" "system" "K32EnumProcesses" fn EnumProcesses(lpidprocess : *mut u32, cb : u32, lpcbneeded : *mut u32) -> windows_core::BOOL);
    unsafe { EnumProcesses(lpidprocess as _, cb, lpcbneeded as _) }
}
#[inline]
pub unsafe fn GetDeviceDriverBaseNameA(imagebase: *const core::ffi::c_void, lpfilename: &mut [u8]) -> u32 {
    windows_core::link!("kernel32.dll" "system" "K32GetDeviceDriverBaseNameA" fn GetDeviceDriverBaseNameA(imagebase : *const core::ffi::c_void, lpfilename : windows_core::PSTR, nsize : u32) -> u32);
    unsafe { GetDeviceDriverBaseNameA(imagebase, core::mem::transmute(lpfilename.as_mut_ptr()), lpfilename.len().try_into().unwrap()) }
}
#[inline]
pub unsafe fn GetDeviceDriverBaseNameW(imagebase: *const core::ffi::c_void, lpbasename: &mut [u16]) -> u32 {
    windows_core::link!("kernel32.dll" "system" "K32GetDeviceDriverBaseNameW" fn GetDeviceDriverBaseNameW(imagebase : *const core::ffi::c_void, lpbasename : windows_core::PWSTR, nsize : u32) -> u32);
    unsafe { GetDeviceDriverBaseNameW(imagebase, core::mem::transmute(lpbasename.as_mut_ptr()), lpbasename.len().try_into().unwrap()) }
}
#[inline]
pub unsafe fn GetDeviceDriverFileNameA(imagebase: *const core::ffi::c_void, lpfilename: &mut [u8]) -> u32 {
    windows_core::link!("kernel32.dll" "system" "K32GetDeviceDriverFileNameA" fn GetDeviceDriverFileNameA(imagebase : *const core::ffi::c_void, lpfilename : windows_core::PSTR, nsize : u32) -> u32);
    unsafe { GetDeviceDriverFileNameA(imagebase, core::mem::transmute(lpfilename.as_mut_ptr()), lpfilename.len().try_into().unwrap()) }
}
#[inline]
pub unsafe fn GetDeviceDriverFileNameW(imagebase: *const core::ffi::c_void, lpfilename: &mut [u16]) -> u32 {
    windows_core::link!("kernel32.dll" "system" "K32GetDeviceDriverFileNameW" fn GetDeviceDriverFileNameW(imagebase : *const core::ffi::c_void, lpfilename : windows_core::PWSTR, nsize : u32) -> u32);
    unsafe { GetDeviceDriverFileNameW(imagebase, core::mem::transmute(lpfilename.as_mut_ptr()), lpfilename.len().try_into().unwrap()) }
}
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn GetMappedFileNameA(hprocess: super::winnt::HANDLE, lpv: *const core::ffi::c_void, lpfilename: &mut [u8]) -> u32 {
    windows_core::link!("kernel32.dll" "system" "K32GetMappedFileNameA" fn GetMappedFileNameA(hprocess : super::winnt::HANDLE, lpv : *const core::ffi::c_void, lpfilename : windows_core::PSTR, nsize : u32) -> u32);
    unsafe { GetMappedFileNameA(hprocess, lpv, core::mem::transmute(lpfilename.as_mut_ptr()), lpfilename.len().try_into().unwrap()) }
}
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn GetMappedFileNameW(hprocess: super::winnt::HANDLE, lpv: *const core::ffi::c_void, lpfilename: &mut [u16]) -> u32 {
    windows_core::link!("kernel32.dll" "system" "K32GetMappedFileNameW" fn GetMappedFileNameW(hprocess : super::winnt::HANDLE, lpv : *const core::ffi::c_void, lpfilename : windows_core::PWSTR, nsize : u32) -> u32);
    unsafe { GetMappedFileNameW(hprocess, lpv, core::mem::transmute(lpfilename.as_mut_ptr()), lpfilename.len().try_into().unwrap()) }
}
#[cfg(all(feature = "minwindef", feature = "winnt"))]
#[inline]
pub unsafe fn GetModuleBaseNameA(hprocess: super::winnt::HANDLE, hmodule: Option<super::minwindef::HMODULE>, lpbasename: &mut [u8]) -> u32 {
    windows_core::link!("kernel32.dll" "system" "K32GetModuleBaseNameA" fn GetModuleBaseNameA(hprocess : super::winnt::HANDLE, hmodule : super::minwindef::HMODULE, lpbasename : windows_core::PSTR, nsize : u32) -> u32);
    unsafe { GetModuleBaseNameA(hprocess, hmodule.unwrap_or(core::mem::zeroed()) as _, core::mem::transmute(lpbasename.as_mut_ptr()), lpbasename.len().try_into().unwrap()) }
}
#[cfg(all(feature = "minwindef", feature = "winnt"))]
#[inline]
pub unsafe fn GetModuleBaseNameW(hprocess: super::winnt::HANDLE, hmodule: Option<super::minwindef::HMODULE>, lpbasename: &mut [u16]) -> u32 {
    windows_core::link!("kernel32.dll" "system" "K32GetModuleBaseNameW" fn GetModuleBaseNameW(hprocess : super::winnt::HANDLE, hmodule : super::minwindef::HMODULE, lpbasename : windows_core::PWSTR, nsize : u32) -> u32);
    unsafe { GetModuleBaseNameW(hprocess, hmodule.unwrap_or(core::mem::zeroed()) as _, core::mem::transmute(lpbasename.as_mut_ptr()), lpbasename.len().try_into().unwrap()) }
}
#[cfg(all(feature = "minwindef", feature = "winnt"))]
#[inline]
pub unsafe fn GetModuleFileNameExA<P2>(hprocess: Option<super::winnt::HANDLE>, hmodule: Option<super::minwindef::HMODULE>, lpfilename: P2, nsize: u32) -> u32
where
    P2: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("kernel32.dll" "system" "K32GetModuleFileNameExA" fn GetModuleFileNameExA(hprocess : super::winnt::HANDLE, hmodule : super::minwindef::HMODULE, lpfilename : windows_core::PCSTR, nsize : u32) -> u32);
    unsafe { GetModuleFileNameExA(hprocess.unwrap_or(core::mem::zeroed()) as _, hmodule.unwrap_or(core::mem::zeroed()) as _, lpfilename.param().abi(), nsize) }
}
#[cfg(all(feature = "minwindef", feature = "winnt"))]
#[inline]
pub unsafe fn GetModuleFileNameExW<P2>(hprocess: Option<super::winnt::HANDLE>, hmodule: Option<super::minwindef::HMODULE>, lpfilename: P2, nsize: u32) -> u32
where
    P2: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("kernel32.dll" "system" "K32GetModuleFileNameExW" fn GetModuleFileNameExW(hprocess : super::winnt::HANDLE, hmodule : super::minwindef::HMODULE, lpfilename : windows_core::PCWSTR, nsize : u32) -> u32);
    unsafe { GetModuleFileNameExW(hprocess.unwrap_or(core::mem::zeroed()) as _, hmodule.unwrap_or(core::mem::zeroed()) as _, lpfilename.param().abi(), nsize) }
}
#[cfg(all(feature = "minwindef", feature = "winnt"))]
#[inline]
pub unsafe fn GetModuleInformation(hprocess: super::winnt::HANDLE, hmodule: super::minwindef::HMODULE, lpmodinfo: *mut MODULEINFO, cb: u32) -> windows_core::BOOL {
    windows_core::link!("kernel32.dll" "system" "K32GetModuleInformation" fn GetModuleInformation(hprocess : super::winnt::HANDLE, hmodule : super::minwindef::HMODULE, lpmodinfo : *mut MODULEINFO, cb : u32) -> windows_core::BOOL);
    unsafe { GetModuleInformation(hprocess, hmodule, lpmodinfo as _, cb) }
}
#[inline]
pub unsafe fn GetPerformanceInfo(pperformanceinformation: *mut PERFORMANCE_INFORMATION, cb: u32) -> windows_core::BOOL {
    windows_core::link!("kernel32.dll" "system" "K32GetPerformanceInfo" fn GetPerformanceInfo(pperformanceinformation : *mut PERFORMANCE_INFORMATION, cb : u32) -> windows_core::BOOL);
    unsafe { GetPerformanceInfo(pperformanceinformation as _, cb) }
}
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn GetProcessImageFileNameA(hprocess: super::winnt::HANDLE, lpimagefilename: &mut [u8]) -> u32 {
    windows_core::link!("kernel32.dll" "system" "K32GetProcessImageFileNameA" fn GetProcessImageFileNameA(hprocess : super::winnt::HANDLE, lpimagefilename : windows_core::PSTR, nsize : u32) -> u32);
    unsafe { GetProcessImageFileNameA(hprocess, core::mem::transmute(lpimagefilename.as_mut_ptr()), lpimagefilename.len().try_into().unwrap()) }
}
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn GetProcessImageFileNameW(hprocess: super::winnt::HANDLE, lpimagefilename: &mut [u16]) -> u32 {
    windows_core::link!("kernel32.dll" "system" "K32GetProcessImageFileNameW" fn GetProcessImageFileNameW(hprocess : super::winnt::HANDLE, lpimagefilename : windows_core::PWSTR, nsize : u32) -> u32);
    unsafe { GetProcessImageFileNameW(hprocess, core::mem::transmute(lpimagefilename.as_mut_ptr()), lpimagefilename.len().try_into().unwrap()) }
}
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn GetProcessMemoryInfo(process: super::winnt::HANDLE, ppsmemcounters: *mut PROCESS_MEMORY_COUNTERS, cb: u32) -> windows_core::BOOL {
    windows_core::link!("kernel32.dll" "system" "K32GetProcessMemoryInfo" fn GetProcessMemoryInfo(process : super::winnt::HANDLE, ppsmemcounters : *mut PROCESS_MEMORY_COUNTERS, cb : u32) -> windows_core::BOOL);
    unsafe { GetProcessMemoryInfo(process, ppsmemcounters as _, cb) }
}
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn GetWsChanges(hprocess: super::winnt::HANDLE, lpwatchinfo: *mut PSAPI_WS_WATCH_INFORMATION, cb: u32) -> windows_core::BOOL {
    windows_core::link!("kernel32.dll" "system" "K32GetWsChanges" fn GetWsChanges(hprocess : super::winnt::HANDLE, lpwatchinfo : *mut PSAPI_WS_WATCH_INFORMATION, cb : u32) -> windows_core::BOOL);
    unsafe { GetWsChanges(hprocess, lpwatchinfo as _, cb) }
}
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn GetWsChangesEx(hprocess: super::winnt::HANDLE, lpwatchinfoex: *mut PSAPI_WS_WATCH_INFORMATION_EX, cb: *mut u32) -> windows_core::BOOL {
    windows_core::link!("kernel32.dll" "system" "K32GetWsChangesEx" fn GetWsChangesEx(hprocess : super::winnt::HANDLE, lpwatchinfoex : *mut PSAPI_WS_WATCH_INFORMATION_EX, cb : *mut u32) -> windows_core::BOOL);
    unsafe { GetWsChangesEx(hprocess, lpwatchinfoex as _, cb as _) }
}
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn InitializeProcessForWsWatch(hprocess: super::winnt::HANDLE) -> windows_core::BOOL {
    windows_core::link!("kernel32.dll" "system" "K32InitializeProcessForWsWatch" fn InitializeProcessForWsWatch(hprocess : super::winnt::HANDLE) -> windows_core::BOOL);
    unsafe { InitializeProcessForWsWatch(hprocess) }
}
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn QueryWorkingSet(hprocess: super::winnt::HANDLE, pv: *mut core::ffi::c_void, cb: u32) -> windows_core::BOOL {
    windows_core::link!("kernel32.dll" "system" "K32QueryWorkingSet" fn QueryWorkingSet(hprocess : super::winnt::HANDLE, pv : *mut core::ffi::c_void, cb : u32) -> windows_core::BOOL);
    unsafe { QueryWorkingSet(hprocess, pv as _, cb) }
}
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn QueryWorkingSetEx(hprocess: super::winnt::HANDLE, pv: *mut core::ffi::c_void, cb: u32) -> windows_core::BOOL {
    windows_core::link!("kernel32.dll" "system" "K32QueryWorkingSetEx" fn QueryWorkingSetEx(hprocess : super::winnt::HANDLE, pv : *mut core::ffi::c_void, cb : u32) -> windows_core::BOOL);
    unsafe { QueryWorkingSetEx(hprocess, pv as _, cb) }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
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
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
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
pub type PENUM_PAGE_FILE_CALLBACKA = Option<unsafe extern "system" fn(pcontext: *mut core::ffi::c_void, ppagefileinfo: *mut ENUM_PAGE_FILE_INFORMATION, lpfilename: windows_core::PCSTR) -> windows_core::BOOL>;
pub type PENUM_PAGE_FILE_CALLBACKW = Option<unsafe extern "system" fn(pcontext: *mut core::ffi::c_void, ppagefileinfo: *mut ENUM_PAGE_FILE_INFORMATION, lpfilename: windows_core::PCWSTR) -> windows_core::BOOL>;
pub type PENUM_PAGE_FILE_INFORMATION = *mut ENUM_PAGE_FILE_INFORMATION;
pub type PERFORMACE_INFORMATION = PERFORMANCE_INFORMATION;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
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
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
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
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
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
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
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
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
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
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct PSAPI_WORKING_SET_EX_BLOCK_0_0 {
    pub _bitfield: usize,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
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
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
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
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct PSAPI_WS_WATCH_INFORMATION_EX {
    pub BasicInfo: PSAPI_WS_WATCH_INFORMATION,
    pub FaultingThreadId: usize,
    pub Flags: usize,
}

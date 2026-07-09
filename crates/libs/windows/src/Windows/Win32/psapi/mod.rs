#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn K32EmptyWorkingSet(hprocess: super::winnt::HANDLE) -> windows_core::BOOL {
    windows_core::link!("kernel32.dll" "C" fn K32EmptyWorkingSet(hprocess : super::winnt::HANDLE) -> windows_core::BOOL);
    unsafe { K32EmptyWorkingSet(hprocess) }
}
#[inline]
pub unsafe fn K32EnumDeviceDrivers(lpimagebase: *mut *mut core::ffi::c_void, cb: u32, lpcbneeded: *mut u32) -> windows_core::BOOL {
    windows_core::link!("kernel32.dll" "C" fn K32EnumDeviceDrivers(lpimagebase : *mut *mut core::ffi::c_void, cb : u32, lpcbneeded : *mut u32) -> windows_core::BOOL);
    unsafe { K32EnumDeviceDrivers(lpimagebase as _, cb, lpcbneeded as _) }
}
#[inline]
pub unsafe fn K32EnumPageFilesA(pcallbackroutine: PENUM_PAGE_FILE_CALLBACKA, pcontext: *mut core::ffi::c_void) -> windows_core::BOOL {
    windows_core::link!("kernel32.dll" "C" fn K32EnumPageFilesA(pcallbackroutine : PENUM_PAGE_FILE_CALLBACKA, pcontext : *mut core::ffi::c_void) -> windows_core::BOOL);
    unsafe { K32EnumPageFilesA(pcallbackroutine, pcontext as _) }
}
#[inline]
pub unsafe fn K32EnumPageFilesW(pcallbackroutine: PENUM_PAGE_FILE_CALLBACKW, pcontext: *mut core::ffi::c_void) -> windows_core::BOOL {
    windows_core::link!("kernel32.dll" "C" fn K32EnumPageFilesW(pcallbackroutine : PENUM_PAGE_FILE_CALLBACKW, pcontext : *mut core::ffi::c_void) -> windows_core::BOOL);
    unsafe { K32EnumPageFilesW(pcallbackroutine, pcontext as _) }
}
#[cfg(all(feature = "minwindef", feature = "winnt"))]
#[inline]
pub unsafe fn K32EnumProcessModules(hprocess: super::winnt::HANDLE, lphmodule: *mut super::minwindef::HMODULE, cb: u32, lpcbneeded: *mut u32) -> windows_core::BOOL {
    windows_core::link!("kernel32.dll" "C" fn K32EnumProcessModules(hprocess : super::winnt::HANDLE, lphmodule : *mut super::minwindef::HMODULE, cb : u32, lpcbneeded : *mut u32) -> windows_core::BOOL);
    unsafe { K32EnumProcessModules(hprocess, lphmodule as _, cb, lpcbneeded as _) }
}
#[cfg(all(feature = "minwindef", feature = "winnt"))]
#[inline]
pub unsafe fn K32EnumProcessModulesEx(hprocess: super::winnt::HANDLE, lphmodule: *mut super::minwindef::HMODULE, cb: u32, lpcbneeded: *mut u32, dwfilterflag: u32) -> windows_core::BOOL {
    windows_core::link!("kernel32.dll" "C" fn K32EnumProcessModulesEx(hprocess : super::winnt::HANDLE, lphmodule : *mut super::minwindef::HMODULE, cb : u32, lpcbneeded : *mut u32, dwfilterflag : u32) -> windows_core::BOOL);
    unsafe { K32EnumProcessModulesEx(hprocess, lphmodule as _, cb, lpcbneeded as _, dwfilterflag) }
}
#[inline]
pub unsafe fn K32EnumProcesses(lpidprocess: *mut u32, cb: u32, lpcbneeded: *mut u32) -> windows_core::BOOL {
    windows_core::link!("kernel32.dll" "C" fn K32EnumProcesses(lpidprocess : *mut u32, cb : u32, lpcbneeded : *mut u32) -> windows_core::BOOL);
    unsafe { K32EnumProcesses(lpidprocess as _, cb, lpcbneeded as _) }
}
#[inline]
pub unsafe fn K32GetDeviceDriverBaseNameA(imagebase: *const core::ffi::c_void, lpfilename: &mut [u8]) -> u32 {
    windows_core::link!("kernel32.dll" "C" fn K32GetDeviceDriverBaseNameA(imagebase : *const core::ffi::c_void, lpfilename : windows_core::PSTR, nsize : u32) -> u32);
    unsafe { K32GetDeviceDriverBaseNameA(imagebase, core::mem::transmute(lpfilename.as_ptr()), lpfilename.len().try_into().unwrap()) }
}
#[inline]
pub unsafe fn K32GetDeviceDriverBaseNameW(imagebase: *const core::ffi::c_void, lpbasename: &mut [u16]) -> u32 {
    windows_core::link!("kernel32.dll" "C" fn K32GetDeviceDriverBaseNameW(imagebase : *const core::ffi::c_void, lpbasename : windows_core::PWSTR, nsize : u32) -> u32);
    unsafe { K32GetDeviceDriverBaseNameW(imagebase, core::mem::transmute(lpbasename.as_ptr()), lpbasename.len().try_into().unwrap()) }
}
#[inline]
pub unsafe fn K32GetDeviceDriverFileNameA(imagebase: *const core::ffi::c_void, lpfilename: &mut [u8]) -> u32 {
    windows_core::link!("kernel32.dll" "C" fn K32GetDeviceDriverFileNameA(imagebase : *const core::ffi::c_void, lpfilename : windows_core::PSTR, nsize : u32) -> u32);
    unsafe { K32GetDeviceDriverFileNameA(imagebase, core::mem::transmute(lpfilename.as_ptr()), lpfilename.len().try_into().unwrap()) }
}
#[inline]
pub unsafe fn K32GetDeviceDriverFileNameW(imagebase: *const core::ffi::c_void, lpfilename: &mut [u16]) -> u32 {
    windows_core::link!("kernel32.dll" "C" fn K32GetDeviceDriverFileNameW(imagebase : *const core::ffi::c_void, lpfilename : windows_core::PWSTR, nsize : u32) -> u32);
    unsafe { K32GetDeviceDriverFileNameW(imagebase, core::mem::transmute(lpfilename.as_ptr()), lpfilename.len().try_into().unwrap()) }
}
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn K32GetMappedFileNameA(hprocess: super::winnt::HANDLE, lpv: *const core::ffi::c_void, lpfilename: &mut [u8]) -> u32 {
    windows_core::link!("kernel32.dll" "C" fn K32GetMappedFileNameA(hprocess : super::winnt::HANDLE, lpv : *const core::ffi::c_void, lpfilename : windows_core::PSTR, nsize : u32) -> u32);
    unsafe { K32GetMappedFileNameA(hprocess, lpv, core::mem::transmute(lpfilename.as_ptr()), lpfilename.len().try_into().unwrap()) }
}
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn K32GetMappedFileNameW(hprocess: super::winnt::HANDLE, lpv: *const core::ffi::c_void, lpfilename: &mut [u16]) -> u32 {
    windows_core::link!("kernel32.dll" "C" fn K32GetMappedFileNameW(hprocess : super::winnt::HANDLE, lpv : *const core::ffi::c_void, lpfilename : windows_core::PWSTR, nsize : u32) -> u32);
    unsafe { K32GetMappedFileNameW(hprocess, lpv, core::mem::transmute(lpfilename.as_ptr()), lpfilename.len().try_into().unwrap()) }
}
#[cfg(all(feature = "minwindef", feature = "winnt"))]
#[inline]
pub unsafe fn K32GetModuleBaseNameA(hprocess: super::winnt::HANDLE, hmodule: Option<super::minwindef::HMODULE>, lpbasename: &mut [u8]) -> u32 {
    windows_core::link!("kernel32.dll" "C" fn K32GetModuleBaseNameA(hprocess : super::winnt::HANDLE, hmodule : super::minwindef::HMODULE, lpbasename : windows_core::PSTR, nsize : u32) -> u32);
    unsafe { K32GetModuleBaseNameA(hprocess, hmodule.unwrap_or(core::mem::zeroed()) as _, core::mem::transmute(lpbasename.as_ptr()), lpbasename.len().try_into().unwrap()) }
}
#[cfg(all(feature = "minwindef", feature = "winnt"))]
#[inline]
pub unsafe fn K32GetModuleBaseNameW(hprocess: super::winnt::HANDLE, hmodule: Option<super::minwindef::HMODULE>, lpbasename: &mut [u16]) -> u32 {
    windows_core::link!("kernel32.dll" "C" fn K32GetModuleBaseNameW(hprocess : super::winnt::HANDLE, hmodule : super::minwindef::HMODULE, lpbasename : windows_core::PWSTR, nsize : u32) -> u32);
    unsafe { K32GetModuleBaseNameW(hprocess, hmodule.unwrap_or(core::mem::zeroed()) as _, core::mem::transmute(lpbasename.as_ptr()), lpbasename.len().try_into().unwrap()) }
}
#[cfg(all(feature = "minwindef", feature = "winnt"))]
#[inline]
pub unsafe fn K32GetModuleFileNameExA<P2>(hprocess: Option<super::winnt::HANDLE>, hmodule: Option<super::minwindef::HMODULE>, lpfilename: P2, nsize: u32) -> u32
where
    P2: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("kernel32.dll" "C" fn K32GetModuleFileNameExA(hprocess : super::winnt::HANDLE, hmodule : super::minwindef::HMODULE, lpfilename : windows_core::PCSTR, nsize : u32) -> u32);
    unsafe { K32GetModuleFileNameExA(hprocess.unwrap_or(core::mem::zeroed()) as _, hmodule.unwrap_or(core::mem::zeroed()) as _, lpfilename.param().abi(), nsize) }
}
#[cfg(all(feature = "minwindef", feature = "winnt"))]
#[inline]
pub unsafe fn K32GetModuleFileNameExW<P2>(hprocess: Option<super::winnt::HANDLE>, hmodule: Option<super::minwindef::HMODULE>, lpfilename: P2, nsize: u32) -> u32
where
    P2: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("kernel32.dll" "C" fn K32GetModuleFileNameExW(hprocess : super::winnt::HANDLE, hmodule : super::minwindef::HMODULE, lpfilename : windows_core::PCWSTR, nsize : u32) -> u32);
    unsafe { K32GetModuleFileNameExW(hprocess.unwrap_or(core::mem::zeroed()) as _, hmodule.unwrap_or(core::mem::zeroed()) as _, lpfilename.param().abi(), nsize) }
}
#[cfg(all(feature = "minwindef", feature = "winnt"))]
#[inline]
pub unsafe fn K32GetModuleInformation(hprocess: super::winnt::HANDLE, hmodule: super::minwindef::HMODULE, lpmodinfo: *mut MODULEINFO, cb: u32) -> windows_core::BOOL {
    windows_core::link!("kernel32.dll" "C" fn K32GetModuleInformation(hprocess : super::winnt::HANDLE, hmodule : super::minwindef::HMODULE, lpmodinfo : *mut MODULEINFO, cb : u32) -> windows_core::BOOL);
    unsafe { K32GetModuleInformation(hprocess, hmodule, lpmodinfo as _, cb) }
}
#[inline]
pub unsafe fn K32GetPerformanceInfo(pperformanceinformation: *mut PERFORMANCE_INFORMATION, cb: u32) -> windows_core::BOOL {
    windows_core::link!("kernel32.dll" "C" fn K32GetPerformanceInfo(pperformanceinformation : *mut PERFORMANCE_INFORMATION, cb : u32) -> windows_core::BOOL);
    unsafe { K32GetPerformanceInfo(pperformanceinformation as _, cb) }
}
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn K32GetProcessImageFileNameA(hprocess: super::winnt::HANDLE, lpimagefilename: &mut [u8]) -> u32 {
    windows_core::link!("kernel32.dll" "C" fn K32GetProcessImageFileNameA(hprocess : super::winnt::HANDLE, lpimagefilename : windows_core::PSTR, nsize : u32) -> u32);
    unsafe { K32GetProcessImageFileNameA(hprocess, core::mem::transmute(lpimagefilename.as_ptr()), lpimagefilename.len().try_into().unwrap()) }
}
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn K32GetProcessImageFileNameW(hprocess: super::winnt::HANDLE, lpimagefilename: &mut [u16]) -> u32 {
    windows_core::link!("kernel32.dll" "C" fn K32GetProcessImageFileNameW(hprocess : super::winnt::HANDLE, lpimagefilename : windows_core::PWSTR, nsize : u32) -> u32);
    unsafe { K32GetProcessImageFileNameW(hprocess, core::mem::transmute(lpimagefilename.as_ptr()), lpimagefilename.len().try_into().unwrap()) }
}
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn K32GetProcessMemoryInfo(process: super::winnt::HANDLE, ppsmemcounters: *mut PROCESS_MEMORY_COUNTERS, cb: u32) -> windows_core::BOOL {
    windows_core::link!("kernel32.dll" "C" fn K32GetProcessMemoryInfo(process : super::winnt::HANDLE, ppsmemcounters : *mut PROCESS_MEMORY_COUNTERS, cb : u32) -> windows_core::BOOL);
    unsafe { K32GetProcessMemoryInfo(process, ppsmemcounters as _, cb) }
}
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn K32GetWsChanges(hprocess: super::winnt::HANDLE, lpwatchinfo: *mut PSAPI_WS_WATCH_INFORMATION, cb: u32) -> windows_core::BOOL {
    windows_core::link!("kernel32.dll" "C" fn K32GetWsChanges(hprocess : super::winnt::HANDLE, lpwatchinfo : *mut PSAPI_WS_WATCH_INFORMATION, cb : u32) -> windows_core::BOOL);
    unsafe { K32GetWsChanges(hprocess, lpwatchinfo as _, cb) }
}
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn K32GetWsChangesEx(hprocess: super::winnt::HANDLE, lpwatchinfoex: *mut PSAPI_WS_WATCH_INFORMATION_EX, cb: *mut u32) -> windows_core::BOOL {
    windows_core::link!("kernel32.dll" "C" fn K32GetWsChangesEx(hprocess : super::winnt::HANDLE, lpwatchinfoex : *mut PSAPI_WS_WATCH_INFORMATION_EX, cb : *mut u32) -> windows_core::BOOL);
    unsafe { K32GetWsChangesEx(hprocess, lpwatchinfoex as _, cb as _) }
}
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn K32InitializeProcessForWsWatch(hprocess: super::winnt::HANDLE) -> windows_core::BOOL {
    windows_core::link!("kernel32.dll" "C" fn K32InitializeProcessForWsWatch(hprocess : super::winnt::HANDLE) -> windows_core::BOOL);
    unsafe { K32InitializeProcessForWsWatch(hprocess) }
}
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn K32QueryWorkingSet(hprocess: super::winnt::HANDLE, pv: *mut core::ffi::c_void, cb: u32) -> windows_core::BOOL {
    windows_core::link!("kernel32.dll" "C" fn K32QueryWorkingSet(hprocess : super::winnt::HANDLE, pv : *mut core::ffi::c_void, cb : u32) -> windows_core::BOOL);
    unsafe { K32QueryWorkingSet(hprocess, pv as _, cb) }
}
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn K32QueryWorkingSetEx(hprocess: super::winnt::HANDLE, pv: *mut core::ffi::c_void, cb: u32) -> windows_core::BOOL {
    windows_core::link!("kernel32.dll" "C" fn K32QueryWorkingSetEx(hprocess : super::winnt::HANDLE, pv : *mut core::ffi::c_void, cb : u32) -> windows_core::BOOL);
    unsafe { K32QueryWorkingSetEx(hprocess, pv as _, cb) }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
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
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct LPMODULEINFO(pub *mut MODULEINFO);
impl LPMODULEINFO {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for LPMODULEINFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
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
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PENUM_PAGE_FILE_INFORMATION(pub *mut ENUM_PAGE_FILE_INFORMATION);
impl PENUM_PAGE_FILE_INFORMATION {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PENUM_PAGE_FILE_INFORMATION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub type PERFORMACE_INFORMATION = PERFORMANCE_INFORMATION;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
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
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PPERFORMACE_INFORMATION(pub *mut PERFORMANCE_INFORMATION);
impl PPERFORMACE_INFORMATION {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PPERFORMACE_INFORMATION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PPERFORMANCE_INFORMATION(pub *mut PERFORMANCE_INFORMATION);
impl PPERFORMANCE_INFORMATION {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PPERFORMANCE_INFORMATION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PPROCESS_MEMORY_COUNTERS(pub *mut PROCESS_MEMORY_COUNTERS);
impl PPROCESS_MEMORY_COUNTERS {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PPROCESS_MEMORY_COUNTERS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PPROCESS_MEMORY_COUNTERS_EX(pub *mut PROCESS_MEMORY_COUNTERS_EX);
impl PPROCESS_MEMORY_COUNTERS_EX {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PPROCESS_MEMORY_COUNTERS_EX {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PPROCESS_MEMORY_COUNTERS_EX2(pub *mut PROCESS_MEMORY_COUNTERS_EX2);
impl PPROCESS_MEMORY_COUNTERS_EX2 {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PPROCESS_MEMORY_COUNTERS_EX2 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PPSAPI_WORKING_SET_BLOCK(pub *mut PSAPI_WORKING_SET_BLOCK);
impl PPSAPI_WORKING_SET_BLOCK {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PPSAPI_WORKING_SET_BLOCK {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PPSAPI_WORKING_SET_EX_BLOCK(pub *mut PSAPI_WORKING_SET_EX_BLOCK);
impl PPSAPI_WORKING_SET_EX_BLOCK {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PPSAPI_WORKING_SET_EX_BLOCK {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PPSAPI_WORKING_SET_EX_INFORMATION(pub *mut PSAPI_WORKING_SET_EX_INFORMATION);
impl PPSAPI_WORKING_SET_EX_INFORMATION {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PPSAPI_WORKING_SET_EX_INFORMATION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PPSAPI_WORKING_SET_INFORMATION(pub *mut PSAPI_WORKING_SET_INFORMATION);
impl PPSAPI_WORKING_SET_INFORMATION {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PPSAPI_WORKING_SET_INFORMATION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PPSAPI_WS_WATCH_INFORMATION(pub *mut PSAPI_WS_WATCH_INFORMATION);
impl PPSAPI_WS_WATCH_INFORMATION {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PPSAPI_WS_WATCH_INFORMATION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PPSAPI_WS_WATCH_INFORMATION_EX(pub *mut PSAPI_WS_WATCH_INFORMATION_EX);
impl PPSAPI_WS_WATCH_INFORMATION_EX {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PPSAPI_WS_WATCH_INFORMATION_EX {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
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
#[derive(Clone, Copy, Debug, Default, PartialEq)]
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
#[derive(Clone, Copy, Debug, Default, PartialEq)]
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
#[derive(Clone, Copy, Debug, Default, PartialEq)]
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
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct PSAPI_WORKING_SET_EX_BLOCK_0_0 {
    pub _bitfield: usize,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
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
#[derive(Clone, Copy, Debug, PartialEq)]
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
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct PSAPI_WS_WATCH_INFORMATION_EX {
    pub BasicInfo: PSAPI_WS_WATCH_INFORMATION,
    pub FaultingThreadId: usize,
    pub Flags: usize,
}

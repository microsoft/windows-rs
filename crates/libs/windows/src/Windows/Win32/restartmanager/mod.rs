#[cfg(all(feature = "minwindef", feature = "shared"))]
#[inline]
pub unsafe fn RmAddFilter<P1, P3>(dwsessionhandle: super::DWORD, strmodulename: P1, pprocess: Option<*const RM_UNIQUE_PROCESS>, strserviceshortname: P3, filteraction: RM_FILTER_ACTION) -> super::DWORD
where
    P1: windows_core::Param<windows_core::PCWSTR>,
    P3: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("rstrtmgr.dll" "system" fn RmAddFilter(dwsessionhandle : super::DWORD, strmodulename : windows_core::PCWSTR, pprocess : *const RM_UNIQUE_PROCESS, strserviceshortname : windows_core::PCWSTR, filteraction : RM_FILTER_ACTION) -> super::DWORD);
    unsafe { RmAddFilter(dwsessionhandle, strmodulename.param().abi(), pprocess.unwrap_or(core::mem::zeroed()) as _, strserviceshortname.param().abi(), filteraction) }
}
#[cfg(feature = "shared")]
#[inline]
pub unsafe fn RmCancelCurrentTask(dwsessionhandle: super::DWORD) -> super::DWORD {
    windows_core::link!("rstrtmgr.dll" "system" fn RmCancelCurrentTask(dwsessionhandle : super::DWORD) -> super::DWORD);
    unsafe { RmCancelCurrentTask(dwsessionhandle) }
}
#[cfg(feature = "shared")]
#[inline]
pub unsafe fn RmEndSession(dwsessionhandle: super::DWORD) -> super::DWORD {
    windows_core::link!("rstrtmgr.dll" "system" fn RmEndSession(dwsessionhandle : super::DWORD) -> super::DWORD);
    unsafe { RmEndSession(dwsessionhandle) }
}
#[cfg(all(feature = "minwindef", feature = "shared"))]
#[inline]
pub unsafe fn RmGetFilterList(dwsessionhandle: super::DWORD, pbfilterbuf: Option<super::PBYTE>, cbfilterbuf: super::DWORD, cbfilterbufneeded: super::LPDWORD) -> super::DWORD {
    windows_core::link!("rstrtmgr.dll" "system" fn RmGetFilterList(dwsessionhandle : super::DWORD, pbfilterbuf : super::PBYTE, cbfilterbuf : super::DWORD, cbfilterbufneeded : super::LPDWORD) -> super::DWORD);
    unsafe { RmGetFilterList(dwsessionhandle, pbfilterbuf.unwrap_or(core::mem::zeroed()) as _, cbfilterbuf, cbfilterbufneeded as _) }
}
#[cfg(all(feature = "minwindef", feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn RmGetList(dwsessionhandle: super::DWORD, pnprocinfoneeded: *mut super::UINT, pnprocinfo: *mut super::UINT, rgaffectedapps: Option<*mut RM_PROCESS_INFO>, lpdwrebootreasons: super::LPDWORD) -> super::DWORD {
    windows_core::link!("rstrtmgr.dll" "system" fn RmGetList(dwsessionhandle : super::DWORD, pnprocinfoneeded : *mut super::UINT, pnprocinfo : *mut super::UINT, rgaffectedapps : *mut RM_PROCESS_INFO, lpdwrebootreasons : super::LPDWORD) -> super::DWORD);
    unsafe { RmGetList(dwsessionhandle, pnprocinfoneeded as _, pnprocinfo as _, rgaffectedapps.unwrap_or(core::mem::zeroed()) as _, lpdwrebootreasons as _) }
}
#[cfg(all(feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn RmJoinSession(psessionhandle: *mut super::DWORD, strsessionkey: *const super::WCHAR) -> super::DWORD {
    windows_core::link!("rstrtmgr.dll" "system" fn RmJoinSession(psessionhandle : *mut super::DWORD, strsessionkey : *const super::WCHAR) -> super::DWORD);
    unsafe { RmJoinSession(psessionhandle as _, strsessionkey) }
}
#[cfg(all(feature = "minwindef", feature = "shared"))]
#[inline]
pub unsafe fn RmRegisterResources(dwsessionhandle: super::DWORD, rgsfilenames: Option<&[windows_core::PCWSTR]>, rgapplications: Option<&[RM_UNIQUE_PROCESS]>, rgsservicenames: Option<&[windows_core::PCWSTR]>) -> super::DWORD {
    windows_core::link!("rstrtmgr.dll" "system" fn RmRegisterResources(dwsessionhandle : super::DWORD, nfiles : super::UINT, rgsfilenames : *const windows_core::PCWSTR, napplications : super::UINT, rgapplications : *const RM_UNIQUE_PROCESS, nservices : super::UINT, rgsservicenames : *const windows_core::PCWSTR) -> super::DWORD);
    unsafe { RmRegisterResources(dwsessionhandle, rgsfilenames.map_or(0, |slice| slice.len().try_into().unwrap()), rgsfilenames.map_or(core::ptr::null(), |slice| slice.as_ptr()), rgapplications.map_or(0, |slice| slice.len().try_into().unwrap()), rgapplications.map_or(core::ptr::null(), |slice| slice.as_ptr()), rgsservicenames.map_or(0, |slice| slice.len().try_into().unwrap()), rgsservicenames.map_or(core::ptr::null(), |slice| slice.as_ptr())) }
}
#[cfg(all(feature = "minwindef", feature = "shared"))]
#[inline]
pub unsafe fn RmRemoveFilter<P1, P3>(dwsessionhandle: super::DWORD, strmodulename: P1, pprocess: Option<*const RM_UNIQUE_PROCESS>, strserviceshortname: P3) -> super::DWORD
where
    P1: windows_core::Param<windows_core::PCWSTR>,
    P3: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("rstrtmgr.dll" "system" fn RmRemoveFilter(dwsessionhandle : super::DWORD, strmodulename : windows_core::PCWSTR, pprocess : *const RM_UNIQUE_PROCESS, strserviceshortname : windows_core::PCWSTR) -> super::DWORD);
    unsafe { RmRemoveFilter(dwsessionhandle, strmodulename.param().abi(), pprocess.unwrap_or(core::mem::zeroed()) as _, strserviceshortname.param().abi()) }
}
#[cfg(feature = "shared")]
#[inline]
pub unsafe fn RmRestart(dwsessionhandle: super::DWORD, dwrestartflags: Option<super::DWORD>, fnstatus: RM_WRITE_STATUS_CALLBACK) -> super::DWORD {
    windows_core::link!("rstrtmgr.dll" "system" fn RmRestart(dwsessionhandle : super::DWORD, dwrestartflags : super::DWORD, fnstatus : RM_WRITE_STATUS_CALLBACK) -> super::DWORD);
    unsafe { RmRestart(dwsessionhandle, dwrestartflags.unwrap_or(core::mem::zeroed()) as _, fnstatus) }
}
#[cfg(feature = "shared")]
#[inline]
pub unsafe fn RmShutdown(dwsessionhandle: super::DWORD, lactionflags: super::ULONG, fnstatus: RM_WRITE_STATUS_CALLBACK) -> super::DWORD {
    windows_core::link!("rstrtmgr.dll" "system" fn RmShutdown(dwsessionhandle : super::DWORD, lactionflags : super::ULONG, fnstatus : RM_WRITE_STATUS_CALLBACK) -> super::DWORD);
    unsafe { RmShutdown(dwsessionhandle, lactionflags, fnstatus) }
}
#[cfg(all(feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn RmStartSession(psessionhandle: *mut super::DWORD, dwsessionflags: Option<super::DWORD>, strsessionkey: *mut super::WCHAR) -> super::DWORD {
    windows_core::link!("rstrtmgr.dll" "system" fn RmStartSession(psessionhandle : *mut super::DWORD, dwsessionflags : super::DWORD, strsessionkey : *mut super::WCHAR) -> super::DWORD);
    unsafe { RmStartSession(psessionhandle as _, dwsessionflags.unwrap_or(core::mem::zeroed()) as _, strsessionkey as _) }
}
pub const CCH_RM_MAX_APP_NAME: i32 = 255;
pub const CCH_RM_MAX_SVC_NAME: i32 = 63;
#[cfg(target_arch = "x86")]
pub const CCH_RM_SESSION_KEY: u32 = 32;
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
pub const CCH_RM_SESSION_KEY: u64 = 32;
#[cfg(all(feature = "minwindef", feature = "shared"))]
pub type PRM_FILTER_INFO = *mut RM_FILTER_INFO;
#[cfg(all(feature = "minwindef", feature = "shared", feature = "winnt"))]
pub type PRM_PROCESS_INFO = *mut RM_PROCESS_INFO;
#[cfg(all(feature = "minwindef", feature = "shared"))]
pub type PRM_UNIQUE_PROCESS = *mut RM_UNIQUE_PROCESS;
pub type RM_APP_STATUS = i32;
pub type RM_APP_TYPE = i32;
pub type RM_FILTER_ACTION = i32;
#[repr(C)]
#[cfg(all(feature = "minwindef", feature = "shared"))]
#[derive(Clone, Copy)]
pub struct RM_FILTER_INFO {
    pub FilterAction: RM_FILTER_ACTION,
    pub FilterTrigger: RM_FILTER_TRIGGER,
    pub cbNextOffset: super::DWORD,
    pub Anonymous: RM_FILTER_INFO_0,
}
#[cfg(all(feature = "minwindef", feature = "shared"))]
impl Default for RM_FILTER_INFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "minwindef", feature = "shared"))]
#[derive(Clone, Copy)]
pub union RM_FILTER_INFO_0 {
    pub strFilename: windows_core::PWSTR,
    pub Process: RM_UNIQUE_PROCESS,
    pub strServiceShortName: windows_core::PWSTR,
}
#[cfg(all(feature = "minwindef", feature = "shared"))]
impl Default for RM_FILTER_INFO_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub type RM_FILTER_TRIGGER = i32;
pub const RM_INVALID_PROCESS: i32 = -1;
pub const RM_INVALID_TS_SESSION: i32 = -1;
#[repr(C)]
#[cfg(all(feature = "minwindef", feature = "shared", feature = "winnt"))]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct RM_PROCESS_INFO {
    pub Process: RM_UNIQUE_PROCESS,
    pub strAppName: [super::WCHAR; 256],
    pub strServiceShortName: [super::WCHAR; 64],
    pub ApplicationType: RM_APP_TYPE,
    pub AppStatus: super::ULONG,
    pub TSSessionId: super::DWORD,
    pub bRestartable: windows_core::BOOL,
}
#[cfg(all(feature = "minwindef", feature = "shared", feature = "winnt"))]
impl Default for RM_PROCESS_INFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub type RM_REBOOT_REASON = i32;
#[cfg(target_arch = "x86")]
pub const RM_SESSION_KEY_LEN: u32 = 16;
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
pub const RM_SESSION_KEY_LEN: u64 = 16;
pub type RM_SHUTDOWN_TYPE = i32;
#[repr(C)]
#[cfg(all(feature = "minwindef", feature = "shared"))]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct RM_UNIQUE_PROCESS {
    pub dwProcessId: super::DWORD,
    pub ProcessStartTime: super::FILETIME,
}
#[cfg(feature = "shared")]
pub type RM_WRITE_STATUS_CALLBACK = Option<unsafe extern "C" fn(npercentcomplete: super::UINT)>;
pub const RmConsole: RM_APP_TYPE = 5;
pub const RmCritical: RM_APP_TYPE = 1000;
pub const RmExplorer: RM_APP_TYPE = 4;
pub const RmFilterTriggerFile: RM_FILTER_TRIGGER = 1;
pub const RmFilterTriggerInvalid: RM_FILTER_TRIGGER = 0;
pub const RmFilterTriggerProcess: RM_FILTER_TRIGGER = 2;
pub const RmFilterTriggerService: RM_FILTER_TRIGGER = 3;
pub const RmForceShutdown: RM_SHUTDOWN_TYPE = 1;
pub const RmInvalidFilterAction: RM_FILTER_ACTION = 0;
pub const RmMainWindow: RM_APP_TYPE = 1;
pub const RmNoRestart: RM_FILTER_ACTION = 1;
pub const RmNoShutdown: RM_FILTER_ACTION = 2;
pub const RmOtherWindow: RM_APP_TYPE = 2;
pub const RmRebootReasonCriticalProcess: RM_REBOOT_REASON = 4;
pub const RmRebootReasonCriticalService: RM_REBOOT_REASON = 8;
pub const RmRebootReasonDetectedSelf: RM_REBOOT_REASON = 16;
pub const RmRebootReasonNone: RM_REBOOT_REASON = 0;
pub const RmRebootReasonPermissionDenied: RM_REBOOT_REASON = 1;
pub const RmRebootReasonSessionMismatch: RM_REBOOT_REASON = 2;
pub const RmService: RM_APP_TYPE = 3;
pub const RmShutdownOnlyRegistered: RM_SHUTDOWN_TYPE = 16;
pub const RmStatusErrorOnRestart: RM_APP_STATUS = 32;
pub const RmStatusErrorOnStop: RM_APP_STATUS = 16;
pub const RmStatusRestartMasked: RM_APP_STATUS = 128;
pub const RmStatusRestarted: RM_APP_STATUS = 8;
pub const RmStatusRunning: RM_APP_STATUS = 1;
pub const RmStatusShutdownMasked: RM_APP_STATUS = 64;
pub const RmStatusStopped: RM_APP_STATUS = 2;
pub const RmStatusStoppedOther: RM_APP_STATUS = 4;
pub const RmStatusUnknown: RM_APP_STATUS = 0;
pub const RmUnknownApp: RM_APP_TYPE = 0;

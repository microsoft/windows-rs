#[cfg(all(feature = "minwindef", feature = "shared"))]
windows_link::link!("rstrtmgr.dll" "system" fn RmAddFilter(dwsessionhandle : super::DWORD, strmodulename : windows_sys::core::PCWSTR, pprocess : *const RM_UNIQUE_PROCESS, strserviceshortname : windows_sys::core::PCWSTR, filteraction : RM_FILTER_ACTION) -> super::DWORD);
#[cfg(feature = "shared")]
windows_link::link!("rstrtmgr.dll" "system" fn RmCancelCurrentTask(dwsessionhandle : super::DWORD) -> super::DWORD);
#[cfg(feature = "shared")]
windows_link::link!("rstrtmgr.dll" "system" fn RmEndSession(dwsessionhandle : super::DWORD) -> super::DWORD);
#[cfg(all(feature = "minwindef", feature = "shared"))]
windows_link::link!("rstrtmgr.dll" "system" fn RmGetFilterList(dwsessionhandle : super::DWORD, pbfilterbuf : super::PBYTE, cbfilterbuf : super::DWORD, cbfilterbufneeded : super::LPDWORD) -> super::DWORD);
#[cfg(all(feature = "minwindef", feature = "shared", feature = "winnt"))]
windows_link::link!("rstrtmgr.dll" "system" fn RmGetList(dwsessionhandle : super::DWORD, pnprocinfoneeded : *mut super::UINT, pnprocinfo : *mut super::UINT, rgaffectedapps : *mut RM_PROCESS_INFO, lpdwrebootreasons : super::LPDWORD) -> super::DWORD);
#[cfg(all(feature = "shared", feature = "winnt"))]
windows_link::link!("rstrtmgr.dll" "system" fn RmJoinSession(psessionhandle : *mut super::DWORD, strsessionkey : *const super::WCHAR) -> super::DWORD);
#[cfg(all(feature = "minwindef", feature = "shared"))]
windows_link::link!("rstrtmgr.dll" "system" fn RmRegisterResources(dwsessionhandle : super::DWORD, nfiles : super::UINT, rgsfilenames : *const windows_sys::core::PCWSTR, napplications : super::UINT, rgapplications : *const RM_UNIQUE_PROCESS, nservices : super::UINT, rgsservicenames : *const windows_sys::core::PCWSTR) -> super::DWORD);
#[cfg(all(feature = "minwindef", feature = "shared"))]
windows_link::link!("rstrtmgr.dll" "system" fn RmRemoveFilter(dwsessionhandle : super::DWORD, strmodulename : windows_sys::core::PCWSTR, pprocess : *const RM_UNIQUE_PROCESS, strserviceshortname : windows_sys::core::PCWSTR) -> super::DWORD);
#[cfg(feature = "shared")]
windows_link::link!("rstrtmgr.dll" "system" fn RmRestart(dwsessionhandle : super::DWORD, dwrestartflags : super::DWORD, fnstatus : RM_WRITE_STATUS_CALLBACK) -> super::DWORD);
#[cfg(feature = "shared")]
windows_link::link!("rstrtmgr.dll" "system" fn RmShutdown(dwsessionhandle : super::DWORD, lactionflags : super::ULONG, fnstatus : RM_WRITE_STATUS_CALLBACK) -> super::DWORD);
#[cfg(all(feature = "shared", feature = "winnt"))]
windows_link::link!("rstrtmgr.dll" "system" fn RmStartSession(psessionhandle : *mut super::DWORD, dwsessionflags : super::DWORD, strsessionkey : *mut super::WCHAR) -> super::DWORD);
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
    pub strFilename: windows_sys::core::PWSTR,
    pub Process: RM_UNIQUE_PROCESS,
    pub strServiceShortName: windows_sys::core::PWSTR,
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
#[derive(Clone, Copy)]
pub struct RM_PROCESS_INFO {
    pub Process: RM_UNIQUE_PROCESS,
    pub strAppName: [super::WCHAR; 256],
    pub strServiceShortName: [super::WCHAR; 64],
    pub ApplicationType: RM_APP_TYPE,
    pub AppStatus: super::ULONG,
    pub TSSessionId: super::DWORD,
    pub bRestartable: windows_sys::core::BOOL,
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
#[derive(Clone, Copy, Default)]
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

#[cfg(feature = "Win32_minwindef")]
windows_link::link!("rstrtmgr.dll" "system" fn RmAddFilter(dwsessionhandle : u32, strmodulename : windows_sys::core::PCWSTR, pprocess : *const RM_UNIQUE_PROCESS, strserviceshortname : windows_sys::core::PCWSTR, filteraction : RM_FILTER_ACTION) -> u32);
windows_link::link!("rstrtmgr.dll" "system" fn RmCancelCurrentTask(dwsessionhandle : u32) -> u32);
windows_link::link!("rstrtmgr.dll" "system" fn RmEndSession(dwsessionhandle : u32) -> u32);
windows_link::link!("rstrtmgr.dll" "system" fn RmGetFilterList(dwsessionhandle : u32, pbfilterbuf : *mut u8, cbfilterbuf : u32, cbfilterbufneeded : *mut u32) -> u32);
#[cfg(feature = "Win32_minwindef")]
windows_link::link!("rstrtmgr.dll" "system" fn RmGetList(dwsessionhandle : u32, pnprocinfoneeded : *mut u32, pnprocinfo : *mut u32, rgaffectedapps : *mut RM_PROCESS_INFO, lpdwrebootreasons : *mut u32) -> u32);
windows_link::link!("rstrtmgr.dll" "system" fn RmJoinSession(psessionhandle : *mut u32, strsessionkey : *const u16) -> u32);
#[cfg(feature = "Win32_minwindef")]
windows_link::link!("rstrtmgr.dll" "system" fn RmRegisterResources(dwsessionhandle : u32, nfiles : u32, rgsfilenames : *const windows_sys::core::PCWSTR, napplications : u32, rgapplications : *const RM_UNIQUE_PROCESS, nservices : u32, rgsservicenames : *const windows_sys::core::PCWSTR) -> u32);
#[cfg(feature = "Win32_minwindef")]
windows_link::link!("rstrtmgr.dll" "system" fn RmRemoveFilter(dwsessionhandle : u32, strmodulename : windows_sys::core::PCWSTR, pprocess : *const RM_UNIQUE_PROCESS, strserviceshortname : windows_sys::core::PCWSTR) -> u32);
windows_link::link!("rstrtmgr.dll" "system" fn RmRestart(dwsessionhandle : u32, dwrestartflags : u32, fnstatus : RM_WRITE_STATUS_CALLBACK) -> u32);
windows_link::link!("rstrtmgr.dll" "system" fn RmShutdown(dwsessionhandle : u32, lactionflags : u32, fnstatus : RM_WRITE_STATUS_CALLBACK) -> u32);
windows_link::link!("rstrtmgr.dll" "system" fn RmStartSession(psessionhandle : *mut u32, dwsessionflags : u32, strsessionkey : *mut u16) -> u32);
pub const CCH_RM_MAX_APP_NAME: u32 = 255;
pub const CCH_RM_MAX_SVC_NAME: u32 = 63;
pub const CCH_RM_SESSION_KEY: u32 = 32;
#[cfg(feature = "Win32_minwindef")]
pub type PRM_FILTER_INFO = *mut RM_FILTER_INFO;
#[cfg(feature = "Win32_minwindef")]
pub type PRM_PROCESS_INFO = *mut RM_PROCESS_INFO;
#[cfg(feature = "Win32_minwindef")]
pub type PRM_UNIQUE_PROCESS = *mut RM_UNIQUE_PROCESS;
pub type RM_APP_STATUS = i32;
pub type RM_APP_TYPE = i32;
pub type RM_FILTER_ACTION = i32;
#[repr(C)]
#[cfg(feature = "Win32_minwindef")]
#[derive(Clone, Copy)]
pub struct RM_FILTER_INFO {
    pub FilterAction: RM_FILTER_ACTION,
    pub FilterTrigger: RM_FILTER_TRIGGER,
    pub cbNextOffset: u32,
    pub Anonymous: RM_FILTER_INFO_0,
}
#[cfg(feature = "Win32_minwindef")]
impl Default for RM_FILTER_INFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "Win32_minwindef")]
#[derive(Clone, Copy)]
pub union RM_FILTER_INFO_0 {
    pub strFilename: windows_sys::core::PWSTR,
    pub Process: RM_UNIQUE_PROCESS,
    pub strServiceShortName: windows_sys::core::PWSTR,
}
#[cfg(feature = "Win32_minwindef")]
impl Default for RM_FILTER_INFO_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub type RM_FILTER_TRIGGER = i32;
pub const RM_INVALID_PROCESS: i32 = -1;
pub const RM_INVALID_TS_SESSION: i32 = -1;
#[repr(C)]
#[cfg(feature = "Win32_minwindef")]
#[derive(Clone, Copy)]
pub struct RM_PROCESS_INFO {
    pub Process: RM_UNIQUE_PROCESS,
    pub strAppName: [u16; 256],
    pub strServiceShortName: [u16; 64],
    pub ApplicationType: RM_APP_TYPE,
    pub AppStatus: u32,
    pub TSSessionId: u32,
    pub bRestartable: windows_sys::core::BOOL,
}
#[cfg(feature = "Win32_minwindef")]
impl Default for RM_PROCESS_INFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub type RM_REBOOT_REASON = i32;
pub type RM_SHUTDOWN_TYPE = i32;
#[repr(C)]
#[cfg(feature = "Win32_minwindef")]
#[derive(Clone, Copy, Default)]
pub struct RM_UNIQUE_PROCESS {
    pub dwProcessId: u32,
    pub ProcessStartTime: super::minwindef::FILETIME,
}
pub type RM_WRITE_STATUS_CALLBACK = Option<unsafe extern "system" fn(npercentcomplete: u32)>;
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

#[cfg(feature = "Win32_minwindef")]
#[inline]
pub unsafe fn RmAddFilter<P1, P3>(dwsessionhandle: u32, strmodulename: P1, pprocess: Option<*const RM_UNIQUE_PROCESS>, strserviceshortname: P3, filteraction: RM_FILTER_ACTION) -> u32
where
    P1: windows_core::Param<windows_core::PCWSTR>,
    P3: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("rstrtmgr.dll" "system" fn RmAddFilter(dwsessionhandle : u32, strmodulename : windows_core::PCWSTR, pprocess : *const RM_UNIQUE_PROCESS, strserviceshortname : windows_core::PCWSTR, filteraction : RM_FILTER_ACTION) -> u32);
    unsafe { RmAddFilter(dwsessionhandle, strmodulename.param().abi(), pprocess.unwrap_or(core::mem::zeroed()) as _, strserviceshortname.param().abi(), filteraction) }
}
#[inline]
pub unsafe fn RmCancelCurrentTask(dwsessionhandle: u32) -> u32 {
    windows_core::link!("rstrtmgr.dll" "system" fn RmCancelCurrentTask(dwsessionhandle : u32) -> u32);
    unsafe { RmCancelCurrentTask(dwsessionhandle) }
}
#[inline]
pub unsafe fn RmEndSession(dwsessionhandle: u32) -> u32 {
    windows_core::link!("rstrtmgr.dll" "system" fn RmEndSession(dwsessionhandle : u32) -> u32);
    unsafe { RmEndSession(dwsessionhandle) }
}
#[inline]
pub unsafe fn RmGetFilterList(dwsessionhandle: u32, pbfilterbuf: Option<&mut [u8]>, cbfilterbufneeded: *mut u32) -> u32 {
    windows_core::link!("rstrtmgr.dll" "system" fn RmGetFilterList(dwsessionhandle : u32, pbfilterbuf : *mut u8, cbfilterbuf : u32, cbfilterbufneeded : *mut u32) -> u32);
    unsafe { RmGetFilterList(dwsessionhandle, core::mem::transmute(pbfilterbuf.as_deref().map_or(core::ptr::null(), |slice| slice.as_ptr())), pbfilterbuf.as_deref().map_or(0, |slice| slice.len().try_into().unwrap()), cbfilterbufneeded as _) }
}
#[cfg(feature = "Win32_minwindef")]
#[inline]
pub unsafe fn RmGetList(dwsessionhandle: u32, pnprocinfoneeded: *mut u32, pnprocinfo: *mut u32, rgaffectedapps: Option<*mut RM_PROCESS_INFO>, lpdwrebootreasons: *mut u32) -> u32 {
    windows_core::link!("rstrtmgr.dll" "system" fn RmGetList(dwsessionhandle : u32, pnprocinfoneeded : *mut u32, pnprocinfo : *mut u32, rgaffectedapps : *mut RM_PROCESS_INFO, lpdwrebootreasons : *mut u32) -> u32);
    unsafe { RmGetList(dwsessionhandle, pnprocinfoneeded as _, pnprocinfo as _, rgaffectedapps.unwrap_or(core::mem::zeroed()) as _, lpdwrebootreasons as _) }
}
#[inline]
pub unsafe fn RmJoinSession(psessionhandle: *mut u32, strsessionkey: *const u16) -> u32 {
    windows_core::link!("rstrtmgr.dll" "system" fn RmJoinSession(psessionhandle : *mut u32, strsessionkey : *const u16) -> u32);
    unsafe { RmJoinSession(psessionhandle as _, strsessionkey) }
}
#[cfg(feature = "Win32_minwindef")]
#[inline]
pub unsafe fn RmRegisterResources(dwsessionhandle: u32, rgsfilenames: Option<&[windows_core::PCWSTR]>, rgapplications: Option<&[RM_UNIQUE_PROCESS]>, rgsservicenames: Option<&[windows_core::PCWSTR]>) -> u32 {
    windows_core::link!("rstrtmgr.dll" "system" fn RmRegisterResources(dwsessionhandle : u32, nfiles : u32, rgsfilenames : *const windows_core::PCWSTR, napplications : u32, rgapplications : *const RM_UNIQUE_PROCESS, nservices : u32, rgsservicenames : *const windows_core::PCWSTR) -> u32);
    unsafe { RmRegisterResources(dwsessionhandle, rgsfilenames.map_or(0, |slice| slice.len().try_into().unwrap()), core::mem::transmute(rgsfilenames.map_or(core::ptr::null(), |slice| slice.as_ptr())), rgapplications.map_or(0, |slice| slice.len().try_into().unwrap()), core::mem::transmute(rgapplications.map_or(core::ptr::null(), |slice| slice.as_ptr())), rgsservicenames.map_or(0, |slice| slice.len().try_into().unwrap()), core::mem::transmute(rgsservicenames.map_or(core::ptr::null(), |slice| slice.as_ptr()))) }
}
#[cfg(feature = "Win32_minwindef")]
#[inline]
pub unsafe fn RmRemoveFilter<P1, P3>(dwsessionhandle: u32, strmodulename: P1, pprocess: Option<*const RM_UNIQUE_PROCESS>, strserviceshortname: P3) -> u32
where
    P1: windows_core::Param<windows_core::PCWSTR>,
    P3: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("rstrtmgr.dll" "system" fn RmRemoveFilter(dwsessionhandle : u32, strmodulename : windows_core::PCWSTR, pprocess : *const RM_UNIQUE_PROCESS, strserviceshortname : windows_core::PCWSTR) -> u32);
    unsafe { RmRemoveFilter(dwsessionhandle, strmodulename.param().abi(), pprocess.unwrap_or(core::mem::zeroed()) as _, strserviceshortname.param().abi()) }
}
#[inline]
pub unsafe fn RmRestart(dwsessionhandle: u32, dwrestartflags: Option<u32>, fnstatus: RM_WRITE_STATUS_CALLBACK) -> u32 {
    windows_core::link!("rstrtmgr.dll" "system" fn RmRestart(dwsessionhandle : u32, dwrestartflags : u32, fnstatus : RM_WRITE_STATUS_CALLBACK) -> u32);
    unsafe { RmRestart(dwsessionhandle, dwrestartflags.unwrap_or(core::mem::zeroed()) as _, fnstatus) }
}
#[inline]
pub unsafe fn RmShutdown(dwsessionhandle: u32, lactionflags: u32, fnstatus: RM_WRITE_STATUS_CALLBACK) -> u32 {
    windows_core::link!("rstrtmgr.dll" "system" fn RmShutdown(dwsessionhandle : u32, lactionflags : u32, fnstatus : RM_WRITE_STATUS_CALLBACK) -> u32);
    unsafe { RmShutdown(dwsessionhandle, lactionflags, fnstatus) }
}
#[inline]
pub unsafe fn RmStartSession(psessionhandle: *mut u32, dwsessionflags: Option<u32>, strsessionkey: *mut u16) -> u32 {
    windows_core::link!("rstrtmgr.dll" "system" fn RmStartSession(psessionhandle : *mut u32, dwsessionflags : u32, strsessionkey : *mut u16) -> u32);
    unsafe { RmStartSession(psessionhandle as _, dwsessionflags.unwrap_or(core::mem::zeroed()) as _, strsessionkey as _) }
}
pub const CCH_RM_MAX_APP_NAME: u32 = 255;
pub const CCH_RM_MAX_SVC_NAME: u32 = 63;
pub const CCH_RM_SESSION_KEY: u32 = 32;
#[cfg(feature = "Win32_minwindef")]
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PRM_FILTER_INFO(pub *mut RM_FILTER_INFO);
#[cfg(feature = "Win32_minwindef")]
impl PRM_FILTER_INFO {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
#[cfg(feature = "Win32_minwindef")]
impl Default for PRM_FILTER_INFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_minwindef")]
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PRM_PROCESS_INFO(pub *mut RM_PROCESS_INFO);
#[cfg(feature = "Win32_minwindef")]
impl PRM_PROCESS_INFO {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
#[cfg(feature = "Win32_minwindef")]
impl Default for PRM_PROCESS_INFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_minwindef")]
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PRM_UNIQUE_PROCESS(pub *mut RM_UNIQUE_PROCESS);
#[cfg(feature = "Win32_minwindef")]
impl PRM_UNIQUE_PROCESS {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
#[cfg(feature = "Win32_minwindef")]
impl Default for PRM_UNIQUE_PROCESS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
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
    pub strFilename: windows_core::PWSTR,
    pub Process: RM_UNIQUE_PROCESS,
    pub strServiceShortName: windows_core::PWSTR,
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
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct RM_PROCESS_INFO {
    pub Process: RM_UNIQUE_PROCESS,
    pub strAppName: [u16; 256],
    pub strServiceShortName: [u16; 64],
    pub ApplicationType: RM_APP_TYPE,
    pub AppStatus: u32,
    pub TSSessionId: u32,
    pub bRestartable: windows_core::BOOL,
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
#[derive(Clone, Copy, Debug, Default, PartialEq)]
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

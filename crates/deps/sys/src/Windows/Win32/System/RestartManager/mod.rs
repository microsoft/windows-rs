#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, clashing_extern_declarations, clippy::all)]
#[link(name = "windows")]
extern "system" {
    #[cfg(feature = "Win32_Foundation")]
    pub fn RmAddFilter(dwsessionhandle: u32, strmodulename: super::super::Foundation::PWSTR, pprocess: *const RM_UNIQUE_PROCESS, strserviceshortname: super::super::Foundation::PWSTR, filteraction: RM_FILTER_ACTION) -> u32;
    pub fn RmCancelCurrentTask(dwsessionhandle: u32) -> u32;
    pub fn RmEndSession(dwsessionhandle: u32) -> u32;
    pub fn RmGetFilterList(dwsessionhandle: u32, pbfilterbuf: *mut u8, cbfilterbuf: u32, cbfilterbufneeded: *mut u32) -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn RmGetList(dwsessionhandle: u32, pnprocinfoneeded: *mut u32, pnprocinfo: *mut u32, rgaffectedapps: *mut RM_PROCESS_INFO, lpdwrebootreasons: *mut u32) -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn RmJoinSession(psessionhandle: *mut u32, strsessionkey: super::super::Foundation::PWSTR) -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn RmRegisterResources(dwsessionhandle: u32, nfiles: u32, rgsfilenames: *const super::super::Foundation::PWSTR, napplications: u32, rgapplications: *const RM_UNIQUE_PROCESS, nservices: u32, rgsservicenames: *const super::super::Foundation::PWSTR) -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn RmRemoveFilter(dwsessionhandle: u32, strmodulename: super::super::Foundation::PWSTR, pprocess: *const RM_UNIQUE_PROCESS, strserviceshortname: super::super::Foundation::PWSTR) -> u32;
    pub fn RmRestart(dwsessionhandle: u32, dwrestartflags: u32, fnstatus: RM_WRITE_STATUS_CALLBACK) -> u32;
    pub fn RmShutdown(dwsessionhandle: u32, lactionflags: u32, fnstatus: RM_WRITE_STATUS_CALLBACK) -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn RmStartSession(psessionhandle: *mut u32, dwsessionflags: u32, strsessionkey: super::super::Foundation::PWSTR) -> u32;
}
pub const CCH_RM_MAX_APP_NAME: u32 = 255u32;
pub const CCH_RM_MAX_SVC_NAME: u32 = 63u32;
pub const CCH_RM_SESSION_KEY: u32 = 32u32;
pub const RmStatusUnknown: i32 = 0i32;
pub const RmStatusRunning: i32 = 1i32;
pub const RmStatusStopped: i32 = 2i32;
pub const RmStatusStoppedOther: i32 = 4i32;
pub const RmStatusRestarted: i32 = 8i32;
pub const RmStatusErrorOnStop: i32 = 16i32;
pub const RmStatusErrorOnRestart: i32 = 32i32;
pub const RmStatusShutdownMasked: i32 = 64i32;
pub const RmStatusRestartMasked: i32 = 128i32;
pub const RmUnknownApp: i32 = 0i32;
pub const RmMainWindow: i32 = 1i32;
pub const RmOtherWindow: i32 = 2i32;
pub const RmService: i32 = 3i32;
pub const RmExplorer: i32 = 4i32;
pub const RmConsole: i32 = 5i32;
pub const RmCritical: i32 = 1000i32;
pub const RmInvalidFilterAction: i32 = 0i32;
pub const RmNoRestart: i32 = 1i32;
pub const RmNoShutdown: i32 = 2i32;
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct RM_FILTER_INFO {
    pub FilterAction: RM_FILTER_ACTION,
    pub FilterTrigger: RM_FILTER_TRIGGER,
    pub cbNextOffset: u32,
    pub Anonymous: RM_FILTER_INFO_0,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for RM_FILTER_INFO {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for RM_FILTER_INFO {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub union RM_FILTER_INFO_0 {
    pub strFilename: super::super::Foundation::PWSTR,
    pub Process: RM_UNIQUE_PROCESS,
    pub strServiceShortName: super::super::Foundation::PWSTR,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for RM_FILTER_INFO_0 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for RM_FILTER_INFO_0 {
    fn clone(&self) -> Self {
        *self
    }
}
pub const RmFilterTriggerInvalid: i32 = 0i32;
pub const RmFilterTriggerFile: i32 = 1i32;
pub const RmFilterTriggerProcess: i32 = 2i32;
pub const RmFilterTriggerService: i32 = 3i32;
pub const RM_INVALID_PROCESS: i32 = -1i32;
pub const RM_INVALID_TS_SESSION: i32 = -1i32;
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct RM_PROCESS_INFO {
    pub Process: RM_UNIQUE_PROCESS,
    pub strAppName: [u16; 256],
    pub strServiceShortName: [u16; 64],
    pub ApplicationType: RM_APP_TYPE,
    pub AppStatus: u32,
    pub TSSessionId: u32,
    pub bRestartable: super::super::Foundation::BOOL,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for RM_PROCESS_INFO {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for RM_PROCESS_INFO {
    fn clone(&self) -> Self {
        *self
    }
}
pub const RmRebootReasonNone: i32 = 0i32;
pub const RmRebootReasonPermissionDenied: i32 = 1i32;
pub const RmRebootReasonSessionMismatch: i32 = 2i32;
pub const RmRebootReasonCriticalProcess: i32 = 4i32;
pub const RmRebootReasonCriticalService: i32 = 8i32;
pub const RmRebootReasonDetectedSelf: i32 = 16i32;
pub const RmForceShutdown: i32 = 1i32;
pub const RmShutdownOnlyRegistered: i32 = 16i32;
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct RM_UNIQUE_PROCESS {
    pub dwProcessId: u32,
    pub ProcessStartTime: super::super::Foundation::FILETIME,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for RM_UNIQUE_PROCESS {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for RM_UNIQUE_PROCESS {
    fn clone(&self) -> Self {
        *self
    }
}
pub type RM_WRITE_STATUS_CALLBACK = unsafe extern "system" fn(npercentcomplete: u32);

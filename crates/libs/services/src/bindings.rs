windows_link::link!("advapi32.dll" "system" fn RegisterServiceCtrlHandlerExW(lpservicename : PCWSTR, lphandlerproc : LPHANDLER_FUNCTION_EX, lpcontext : *const core::ffi::c_void) -> SERVICE_STATUS_HANDLE);
windows_link::link!("advapi32.dll" "system" fn SetServiceStatus(hservicestatus : SERVICE_STATUS_HANDLE, lpservicestatus : *const SERVICE_STATUS) -> BOOL);
windows_link::link!("advapi32.dll" "system" fn StartServiceCtrlDispatcherW(lpservicestarttable : *const SERVICE_TABLE_ENTRYW) -> BOOL);
pub type BOOL = i32;
pub type LPHANDLER_FUNCTION_EX = Option<
    unsafe extern "system" fn(
        dwcontrol: u32,
        dweventtype: u32,
        lpeventdata: *mut core::ffi::c_void,
        lpcontext: *mut core::ffi::c_void,
    ) -> u32,
>;
pub type LPSERVICE_MAIN_FUNCTIONW =
    Option<unsafe extern "system" fn(dwnumservicesargs: u32, lpserviceargvectors: *mut PWSTR)>;
pub const NO_ERROR: u32 = 0;
pub type PCWSTR = *const u16;
pub type PWSTR = *mut u16;
pub const SERVICE_ACCEPT_PAUSE_CONTINUE: u32 = 2;
pub const SERVICE_ACCEPT_SHUTDOWN: u32 = 4;
pub const SERVICE_ACCEPT_STOP: u32 = 1;
pub const SERVICE_CONTINUE_PENDING: u32 = 5;
pub const SERVICE_CONTROL_CONTINUE: u32 = 3;
pub const SERVICE_CONTROL_PAUSE: u32 = 2;
pub const SERVICE_CONTROL_SHUTDOWN: u32 = 5;
pub const SERVICE_CONTROL_STOP: u32 = 1;
pub const SERVICE_PAUSED: u32 = 7;
pub const SERVICE_PAUSE_PENDING: u32 = 6;
pub const SERVICE_RUNNING: u32 = 4;
pub const SERVICE_START_PENDING: u32 = 2;
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct SERVICE_STATUS {
    pub dwServiceType: u32,
    pub dwCurrentState: u32,
    pub dwControlsAccepted: u32,
    pub dwWin32ExitCode: u32,
    pub dwServiceSpecificExitCode: u32,
    pub dwCheckPoint: u32,
    pub dwWaitHint: u32,
}
pub type SERVICE_STATUS_HANDLE = *mut core::ffi::c_void;
pub const SERVICE_STOPPED: u32 = 1;
pub const SERVICE_STOP_PENDING: u32 = 3;
#[repr(C)]
#[derive(Clone, Copy)]
pub struct SERVICE_TABLE_ENTRYW {
    pub lpServiceName: PWSTR,
    pub lpServiceProc: LPSERVICE_MAIN_FUNCTIONW,
}
impl Default for SERVICE_TABLE_ENTRYW {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const SERVICE_WIN32_OWN_PROCESS: u32 = 16;

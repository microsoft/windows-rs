windows_link::link!("netapi32.dll" "system" fn NetAlertRaise(alerttype : windows_sys::core::PCWSTR, buffer : *const core::ffi::c_void, buffersize : u32) -> u32);
windows_link::link!("netapi32.dll" "system" fn NetAlertRaiseEx(alerttype : windows_sys::core::PCWSTR, variableinfo : *const core::ffi::c_void, variableinfosize : u32, servicename : windows_sys::core::PCWSTR) -> u32);
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct ADMIN_OTHER_INFO {
    pub alrtad_errcode: u32,
    pub alrtad_numstrings: u32,
}
pub const ALERTER_MAILSLOT: windows_sys::core::PCWSTR = windows_sys::core::w!("\\\\.\\MAILSLOT\\Alerter");
pub const ALERT_ADMIN_EVENT: windows_sys::core::PCWSTR = windows_sys::core::w!("ADMIN");
pub const ALERT_ERRORLOG_EVENT: windows_sys::core::PCWSTR = windows_sys::core::w!("ERRORLOG");
pub const ALERT_MESSAGE_EVENT: windows_sys::core::PCWSTR = windows_sys::core::w!("MESSAGE");
pub const ALERT_PRINT_EVENT: windows_sys::core::PCWSTR = windows_sys::core::w!("PRINTING");
pub const ALERT_USER_EVENT: windows_sys::core::PCWSTR = windows_sys::core::w!("USER");
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct ERRLOG_OTHER_INFO {
    pub alrter_errcode: u32,
    pub alrter_offset: u32,
}
pub type LPADMIN_OTHER_INFO = *mut ADMIN_OTHER_INFO;
pub type LPERRLOG_OTHER_INFO = *mut ERRLOG_OTHER_INFO;
pub type LPPRINT_OTHER_INFO = *mut PRINT_OTHER_INFO;
pub type LPSTD_ALERT = *mut STD_ALERT;
pub type LPUSER_OTHER_INFO = *mut USER_OTHER_INFO;
pub type PADMIN_OTHER_INFO = *mut ADMIN_OTHER_INFO;
pub type PERRLOG_OTHER_INFO = *mut ERRLOG_OTHER_INFO;
pub type PPRINT_OTHER_INFO = *mut PRINT_OTHER_INFO;
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct PRINT_OTHER_INFO {
    pub alrtpr_jobid: u32,
    pub alrtpr_status: u32,
    pub alrtpr_submitted: u32,
    pub alrtpr_size: u32,
}
pub const PRJOB_COMPLETE: i32 = 4;
pub const PRJOB_DELETED: i32 = 32768;
pub const PRJOB_DESTNOPAPER: i32 = 256;
pub const PRJOB_DESTOFFLINE: i32 = 32;
pub const PRJOB_DESTPAUSED: i32 = 64;
pub const PRJOB_DEVSTATUS: i32 = 508;
pub const PRJOB_ERROR: i32 = 16;
pub const PRJOB_INTERV: i32 = 8;
pub const PRJOB_NOTIFY: i32 = 128;
pub const PRJOB_QSTATUS: i32 = 3;
pub const PRJOB_QS_PAUSED: i32 = 1;
pub const PRJOB_QS_PRINTING: i32 = 3;
pub const PRJOB_QS_QUEUED: i32 = 0;
pub const PRJOB_QS_SPOOLING: i32 = 2;
pub type PSTD_ALERT = *mut STD_ALERT;
pub type PUSER_OTHER_INFO = *mut USER_OTHER_INFO;
#[repr(C)]
#[derive(Clone, Copy)]
pub struct STD_ALERT {
    pub alrt_timestamp: u32,
    pub alrt_eventname: [u16; 17],
    pub alrt_servicename: [u16; 81],
}
impl Default for STD_ALERT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct USER_OTHER_INFO {
    pub alrtus_errcode: u32,
    pub alrtus_numstrings: u32,
}

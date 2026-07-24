#[inline]
pub unsafe fn NetAlertRaise<P0>(alerttype: P0, buffer: *const core::ffi::c_void, buffersize: u32) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("netapi32.dll" "system" fn NetAlertRaise(alerttype : windows_core::PCWSTR, buffer : *const core::ffi::c_void, buffersize : u32) -> u32);
    unsafe { NetAlertRaise(alerttype.param().abi(), buffer, buffersize) }
}
#[inline]
pub unsafe fn NetAlertRaiseEx<P0, P3>(alerttype: P0, variableinfo: *const core::ffi::c_void, variableinfosize: u32, servicename: P3) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P3: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("netapi32.dll" "system" fn NetAlertRaiseEx(alerttype : windows_core::PCWSTR, variableinfo : *const core::ffi::c_void, variableinfosize : u32, servicename : windows_core::PCWSTR) -> u32);
    unsafe { NetAlertRaiseEx(alerttype.param().abi(), variableinfo, variableinfosize, servicename.param().abi()) }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct ADMIN_OTHER_INFO {
    pub alrtad_errcode: u32,
    pub alrtad_numstrings: u32,
}
pub const ALERTER_MAILSLOT: windows_core::PCWSTR = windows_core::w!("\\\\.\\MAILSLOT\\Alerter");
pub const ALERT_ADMIN_EVENT: windows_core::PCWSTR = windows_core::w!("ADMIN");
pub const ALERT_ERRORLOG_EVENT: windows_core::PCWSTR = windows_core::w!("ERRORLOG");
pub const ALERT_MESSAGE_EVENT: windows_core::PCWSTR = windows_core::w!("MESSAGE");
pub const ALERT_PRINT_EVENT: windows_core::PCWSTR = windows_core::w!("PRINTING");
pub const ALERT_USER_EVENT: windows_core::PCWSTR = windows_core::w!("USER");
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
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
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
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
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
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
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct USER_OTHER_INFO {
    pub alrtus_errcode: u32,
    pub alrtus_numstrings: u32,
}

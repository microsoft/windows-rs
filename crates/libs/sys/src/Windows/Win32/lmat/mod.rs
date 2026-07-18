windows_link::link!("netapi32.dll" "system" fn NetScheduleJobAdd(servername : windows_sys::core::PCWSTR, buffer : *mut u8, jobid : *mut u32) -> u32);
windows_link::link!("netapi32.dll" "system" fn NetScheduleJobDel(servername : windows_sys::core::PCWSTR, minjobid : u32, maxjobid : u32) -> u32);
#[cfg(feature = "minwindef")]
windows_link::link!("netapi32.dll" "system" fn NetScheduleJobEnum(servername : windows_sys::core::PCWSTR, pointertobuffer : *mut super::LPBYTE, prefferedmaximumlength : u32, entriesread : *mut u32, totalentries : *mut u32, resumehandle : *mut u32) -> u32);
#[cfg(feature = "minwindef")]
windows_link::link!("netapi32.dll" "system" fn NetScheduleJobGetInfo(servername : windows_sys::core::PCWSTR, jobid : u32, pointertobuffer : *mut super::LPBYTE) -> u32);
#[repr(C)]
#[derive(Clone, Copy)]
pub struct AT_ENUM {
    pub JobId: u32,
    pub JobTime: usize,
    pub DaysOfMonth: u32,
    pub DaysOfWeek: u8,
    pub Flags: u8,
    pub Command: windows_sys::core::PWSTR,
}
impl Default for AT_ENUM {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct AT_INFO {
    pub JobTime: usize,
    pub DaysOfMonth: u32,
    pub DaysOfWeek: u8,
    pub Flags: u8,
    pub Command: windows_sys::core::PWSTR,
}
impl Default for AT_INFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const JOB_ADD_CURRENT_DATE: u32 = 8;
pub const JOB_EXEC_ERROR: u32 = 2;
pub const JOB_INPUT_FLAGS: u32 = 25;
pub const JOB_NONINTERACTIVE: u32 = 16;
pub const JOB_OUTPUT_FLAGS: u32 = 23;
pub const JOB_RUNS_TODAY: u32 = 4;
pub const JOB_RUN_PERIODICALLY: u32 = 1;
pub type LPAT_ENUM = *mut AT_ENUM;
pub type LPAT_INFO = *mut AT_INFO;
pub type PAT_ENUM = *mut AT_ENUM;
pub type PAT_INFO = *mut AT_INFO;

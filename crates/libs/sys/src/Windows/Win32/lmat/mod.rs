#[cfg(all(feature = "minwindef", feature = "shared"))]
windows_link::link!("netapi32.dll" "system" fn NetScheduleJobAdd(servername : windows_sys::core::PCWSTR, buffer : super::LPBYTE, jobid : super::LPDWORD) -> super::DWORD);
#[cfg(feature = "shared")]
windows_link::link!("netapi32.dll" "system" fn NetScheduleJobDel(servername : windows_sys::core::PCWSTR, minjobid : super::DWORD, maxjobid : super::DWORD) -> super::DWORD);
#[cfg(all(feature = "minwindef", feature = "shared"))]
windows_link::link!("netapi32.dll" "system" fn NetScheduleJobEnum(servername : windows_sys::core::PCWSTR, pointertobuffer : *mut super::LPBYTE, prefferedmaximumlength : super::DWORD, entriesread : super::LPDWORD, totalentries : super::LPDWORD, resumehandle : super::LPDWORD) -> super::DWORD);
#[cfg(feature = "shared")]
windows_link::link!("netapi32.dll" "system" fn NetScheduleJobGetInfo(servername : windows_sys::core::PCWSTR, jobid : super::DWORD, pointertobuffer : *mut super::LPBYTE) -> super::DWORD);
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Default)]
pub struct AT_ENUM {
    pub JobId: super::DWORD,
    pub JobTime: super::DWORD_PTR,
    pub DaysOfMonth: super::DWORD,
    pub DaysOfWeek: super::UCHAR,
    pub Flags: super::UCHAR,
    pub Command: windows_sys::core::PWSTR,
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Default)]
pub struct AT_INFO {
    pub JobTime: super::DWORD_PTR,
    pub DaysOfMonth: super::DWORD,
    pub DaysOfWeek: super::UCHAR,
    pub Flags: super::UCHAR,
    pub Command: windows_sys::core::PWSTR,
}
pub const JOB_ADD_CURRENT_DATE: i32 = 8;
pub const JOB_EXEC_ERROR: i32 = 2;
pub const JOB_INPUT_FLAGS: i32 = 25;
pub const JOB_NONINTERACTIVE: i32 = 16;
pub const JOB_OUTPUT_FLAGS: i32 = 23;
pub const JOB_RUNS_TODAY: i32 = 4;
pub const JOB_RUN_PERIODICALLY: i32 = 1;
#[cfg(feature = "shared")]
pub type LPAT_ENUM = *mut AT_ENUM;
#[cfg(feature = "shared")]
pub type LPAT_INFO = *mut AT_INFO;
#[cfg(feature = "shared")]
pub type PAT_ENUM = *mut AT_ENUM;
#[cfg(feature = "shared")]
pub type PAT_INFO = *mut AT_INFO;

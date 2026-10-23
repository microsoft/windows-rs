#[cfg(all(feature = "minwindef", feature = "shared"))]
#[inline]
pub unsafe fn NetScheduleJobAdd(servername: &windows_core::PCWSTR, buffer: super::LPBYTE, jobid: super::LPDWORD) -> super::DWORD {
    windows_core::link!("netapi32.dll" "system" fn NetScheduleJobAdd(servername : windows_core::PCWSTR, buffer : super::LPBYTE, jobid : super::LPDWORD) -> super::DWORD);
    unsafe { NetScheduleJobAdd(core::mem::transmute_copy(servername), buffer as _, jobid as _) }
}
#[cfg(feature = "shared")]
#[inline]
pub unsafe fn NetScheduleJobDel(servername: &windows_core::PCWSTR, minjobid: super::DWORD, maxjobid: super::DWORD) -> super::DWORD {
    windows_core::link!("netapi32.dll" "system" fn NetScheduleJobDel(servername : windows_core::PCWSTR, minjobid : super::DWORD, maxjobid : super::DWORD) -> super::DWORD);
    unsafe { NetScheduleJobDel(core::mem::transmute_copy(servername), minjobid, maxjobid) }
}
#[cfg(all(feature = "minwindef", feature = "shared"))]
#[inline]
pub unsafe fn NetScheduleJobEnum(servername: &windows_core::PCWSTR, pointertobuffer: *mut super::LPBYTE, prefferedmaximumlength: super::DWORD, entriesread: super::LPDWORD, totalentries: super::LPDWORD, resumehandle: super::LPDWORD) -> super::DWORD {
    windows_core::link!("netapi32.dll" "system" fn NetScheduleJobEnum(servername : windows_core::PCWSTR, pointertobuffer : *mut super::LPBYTE, prefferedmaximumlength : super::DWORD, entriesread : super::LPDWORD, totalentries : super::LPDWORD, resumehandle : super::LPDWORD) -> super::DWORD);
    unsafe { NetScheduleJobEnum(core::mem::transmute_copy(servername), pointertobuffer as _, prefferedmaximumlength, entriesread as _, totalentries as _, resumehandle as _) }
}
#[cfg(feature = "shared")]
#[inline]
pub unsafe fn NetScheduleJobGetInfo(servername: &windows_core::PCWSTR, jobid: super::DWORD, pointertobuffer: *mut super::LPBYTE) -> super::DWORD {
    windows_core::link!("netapi32.dll" "system" fn NetScheduleJobGetInfo(servername : windows_core::PCWSTR, jobid : super::DWORD, pointertobuffer : *mut super::LPBYTE) -> super::DWORD);
    unsafe { NetScheduleJobGetInfo(core::mem::transmute_copy(servername), jobid, pointertobuffer as _) }
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct AT_ENUM {
    pub JobId: super::DWORD,
    pub JobTime: super::DWORD_PTR,
    pub DaysOfMonth: super::DWORD,
    pub DaysOfWeek: super::UCHAR,
    pub Flags: super::UCHAR,
    pub Command: windows_core::PWSTR,
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct AT_INFO {
    pub JobTime: super::DWORD_PTR,
    pub DaysOfMonth: super::DWORD,
    pub DaysOfWeek: super::UCHAR,
    pub Flags: super::UCHAR,
    pub Command: windows_core::PWSTR,
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

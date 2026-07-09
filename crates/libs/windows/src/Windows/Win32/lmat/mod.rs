#[inline]
pub unsafe fn NetScheduleJobAdd<P0>(servername: P0, buffer: *mut u8, jobid: *mut u32) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("netapi32.dll" "system" fn NetScheduleJobAdd(servername : windows_core::PCWSTR, buffer : *mut u8, jobid : *mut u32) -> u32);
    unsafe { NetScheduleJobAdd(servername.param().abi(), buffer as _, jobid as _) }
}
#[inline]
pub unsafe fn NetScheduleJobDel<P0>(servername: P0, minjobid: u32, maxjobid: u32) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("netapi32.dll" "system" fn NetScheduleJobDel(servername : windows_core::PCWSTR, minjobid : u32, maxjobid : u32) -> u32);
    unsafe { NetScheduleJobDel(servername.param().abi(), minjobid, maxjobid) }
}
#[cfg(feature = "Win32_minwindef")]
#[inline]
pub unsafe fn NetScheduleJobEnum<P0>(servername: P0, pointertobuffer: *mut super::minwindef::LPBYTE, prefferedmaximumlength: u32, entriesread: *mut u32, totalentries: *mut u32, resumehandle: *mut u32) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("netapi32.dll" "system" fn NetScheduleJobEnum(servername : windows_core::PCWSTR, pointertobuffer : *mut super::minwindef::LPBYTE, prefferedmaximumlength : u32, entriesread : *mut u32, totalentries : *mut u32, resumehandle : *mut u32) -> u32);
    unsafe { NetScheduleJobEnum(servername.param().abi(), pointertobuffer as _, prefferedmaximumlength, entriesread as _, totalentries as _, resumehandle as _) }
}
#[cfg(feature = "Win32_minwindef")]
#[inline]
pub unsafe fn NetScheduleJobGetInfo<P0>(servername: P0, jobid: u32, pointertobuffer: *mut super::minwindef::LPBYTE) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("netapi32.dll" "system" fn NetScheduleJobGetInfo(servername : windows_core::PCWSTR, jobid : u32, pointertobuffer : *mut super::minwindef::LPBYTE) -> u32);
    unsafe { NetScheduleJobGetInfo(servername.param().abi(), jobid, pointertobuffer as _) }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct AT_ENUM {
    pub JobId: u32,
    pub JobTime: usize,
    pub DaysOfMonth: u32,
    pub DaysOfWeek: u8,
    pub Flags: u8,
    pub Command: windows_core::PWSTR,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct AT_INFO {
    pub JobTime: usize,
    pub DaysOfMonth: u32,
    pub DaysOfWeek: u8,
    pub Flags: u8,
    pub Command: windows_core::PWSTR,
}
pub const JOB_ADD_CURRENT_DATE: u32 = 8;
pub const JOB_EXEC_ERROR: u32 = 2;
pub const JOB_INPUT_FLAGS: u32 = 25;
pub const JOB_NONINTERACTIVE: u32 = 16;
pub const JOB_OUTPUT_FLAGS: u32 = 23;
pub const JOB_RUNS_TODAY: u32 = 4;
pub const JOB_RUN_PERIODICALLY: u32 = 1;
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct LPAT_ENUM(pub *mut AT_ENUM);
impl LPAT_ENUM {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for LPAT_ENUM {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct LPAT_INFO(pub *mut AT_INFO);
impl LPAT_INFO {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for LPAT_INFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PAT_ENUM(pub *mut AT_ENUM);
impl PAT_ENUM {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PAT_ENUM {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PAT_INFO(pub *mut AT_INFO);
impl PAT_INFO {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PAT_INFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

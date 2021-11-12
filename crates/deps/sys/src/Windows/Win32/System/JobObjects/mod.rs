#![allow(non_snake_case, non_camel_case_types)]
#[link(name = "windows")]
extern "system" {
    #[cfg(feature = "Win32_Foundation")]
    pub fn AssignProcessToJobObject(hjob: super::super::Foundation::HANDLE, hprocess: super::super::Foundation::HANDLE) -> super::super::Foundation::BOOL;
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
    pub fn CreateJobObjectA(lpjobattributes: *const super::super::Security::SECURITY_ATTRIBUTES, lpname: super::super::Foundation::PSTR) -> super::super::Foundation::HANDLE;
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
    pub fn CreateJobObjectW(lpjobattributes: *const super::super::Security::SECURITY_ATTRIBUTES, lpname: super::super::Foundation::PWSTR) -> super::super::Foundation::HANDLE;
    #[cfg(feature = "Win32_Foundation")]
    pub fn CreateJobSet(numjob: u32, userjobset: *const JOB_SET_ARRAY, flags: u32) -> super::super::Foundation::BOOL;
    pub fn FreeMemoryJobObject(buffer: *const ::core::ffi::c_void);
    #[cfg(feature = "Win32_Foundation")]
    pub fn IsProcessInJob(processhandle: super::super::Foundation::HANDLE, jobhandle: super::super::Foundation::HANDLE, result: *mut super::super::Foundation::BOOL) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn OpenJobObjectA(dwdesiredaccess: u32, binherithandle: super::super::Foundation::BOOL, lpname: super::super::Foundation::PSTR) -> super::super::Foundation::HANDLE;
    #[cfg(feature = "Win32_Foundation")]
    pub fn OpenJobObjectW(dwdesiredaccess: u32, binherithandle: super::super::Foundation::BOOL, lpname: super::super::Foundation::PWSTR) -> super::super::Foundation::HANDLE;
    #[cfg(feature = "Win32_Foundation")]
    pub fn QueryInformationJobObject(hjob: super::super::Foundation::HANDLE, jobobjectinformationclass: JOBOBJECTINFOCLASS, lpjobobjectinformation: *mut ::core::ffi::c_void, cbjobobjectinformationlength: u32, lpreturnlength: *mut u32) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn QueryIoRateControlInformationJobObject(hjob: super::super::Foundation::HANDLE, volumename: super::super::Foundation::PWSTR, infoblocks: *mut *mut JOBOBJECT_IO_RATE_CONTROL_INFORMATION, infoblockcount: *mut u32) -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetInformationJobObject(hjob: super::super::Foundation::HANDLE, jobobjectinformationclass: JOBOBJECTINFOCLASS, lpjobobjectinformation: *const ::core::ffi::c_void, cbjobobjectinformationlength: u32) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetIoRateControlInformationJobObject(hjob: super::super::Foundation::HANDLE, ioratecontrolinfo: *const JOBOBJECT_IO_RATE_CONTROL_INFORMATION) -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn TerminateJobObject(hjob: super::super::Foundation::HANDLE, uexitcode: u32) -> super::super::Foundation::BOOL;
    #[cfg(feature = "Win32_Foundation")]
    pub fn UserHandleGrantAccess(huserhandle: super::super::Foundation::HANDLE, hjob: super::super::Foundation::HANDLE, bgrant: super::super::Foundation::BOOL) -> super::super::Foundation::BOOL;
}
#[repr(C)]
pub struct JOBOBJECTINFOCLASS(i32);
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct JOBOBJECT_ASSOCIATE_COMPLETION_PORT(i32);
#[repr(C)]
pub struct JOBOBJECT_BASIC_ACCOUNTING_INFORMATION(i32);
#[cfg(feature = "Win32_System_Threading")]
#[repr(C)]
pub struct JOBOBJECT_BASIC_AND_IO_ACCOUNTING_INFORMATION(i32);
#[repr(C)]
pub struct JOBOBJECT_BASIC_LIMIT_INFORMATION(i32);
#[repr(C)]
pub struct JOBOBJECT_BASIC_PROCESS_ID_LIST(i32);
#[repr(C)]
pub struct JOBOBJECT_BASIC_UI_RESTRICTIONS(i32);
#[repr(C)]
pub struct JOBOBJECT_CPU_RATE_CONTROL_INFORMATION(i32);
#[repr(C)]
pub struct JOBOBJECT_END_OF_JOB_TIME_INFORMATION(i32);
#[cfg(feature = "Win32_System_Threading")]
#[repr(C)]
pub struct JOBOBJECT_EXTENDED_LIMIT_INFORMATION(i32);
#[repr(C)]
pub struct JOBOBJECT_IO_ATTRIBUTION_CONTROL_FLAGS(i32);
#[repr(C)]
pub struct JOBOBJECT_IO_ATTRIBUTION_INFORMATION(i32);
#[repr(C)]
pub struct JOBOBJECT_IO_ATTRIBUTION_STATS(i32);
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct JOBOBJECT_IO_RATE_CONTROL_INFORMATION(i32);
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct JOBOBJECT_IO_RATE_CONTROL_INFORMATION_NATIVE(i32);
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct JOBOBJECT_IO_RATE_CONTROL_INFORMATION_NATIVE_V2(i32);
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct JOBOBJECT_IO_RATE_CONTROL_INFORMATION_NATIVE_V3(i32);
#[repr(C)]
pub struct JOBOBJECT_JOBSET_INFORMATION(i32);
#[repr(C)]
pub struct JOBOBJECT_LIMIT_VIOLATION_INFORMATION(i32);
#[repr(C)]
pub struct JOBOBJECT_LIMIT_VIOLATION_INFORMATION_2(i32);
#[repr(C)]
pub struct JOBOBJECT_NET_RATE_CONTROL_INFORMATION(i32);
#[repr(C)]
pub struct JOBOBJECT_NOTIFICATION_LIMIT_INFORMATION(i32);
#[repr(C)]
pub struct JOBOBJECT_NOTIFICATION_LIMIT_INFORMATION_2(i32);
#[repr(C)]
pub struct JOBOBJECT_RATE_CONTROL_TOLERANCE(i32);
#[repr(C)]
pub struct JOBOBJECT_RATE_CONTROL_TOLERANCE_INTERVAL(i32);
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
#[repr(C)]
pub struct JOBOBJECT_SECURITY_LIMIT_INFORMATION(i32);
#[repr(C)]
pub struct JOB_OBJECT_CPU_RATE_CONTROL(i32);
#[repr(C)]
pub struct JOB_OBJECT_IO_RATE_CONTROL_FLAGS(i32);
#[repr(C)]
pub struct JOB_OBJECT_LIMIT(i32);
#[repr(C)]
pub struct JOB_OBJECT_NET_RATE_CONTROL_FLAGS(i32);
#[repr(C)]
pub struct JOB_OBJECT_SECURITY(i32);
#[repr(C)]
pub struct JOB_OBJECT_TERMINATE_AT_END_ACTION(i32);
#[repr(C)]
pub struct JOB_OBJECT_UILIMIT(i32);
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct JOB_SET_ARRAY(i32);

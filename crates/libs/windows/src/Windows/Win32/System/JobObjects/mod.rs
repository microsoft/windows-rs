#[inline]
pub unsafe fn AssignProcessToJobObject<P0, P1>(hjob: P0, hprocess: P1) -> windows_core::Result<()>
where
    P0: windows_core::Param<super::super::Foundation::HANDLE>,
    P1: windows_core::Param<super::super::Foundation::HANDLE>,
{
    windows_targets::link!("kernel32.dll" "system" fn AssignProcessToJobObject(hjob : super::super::Foundation:: HANDLE, hprocess : super::super::Foundation:: HANDLE) -> super::super::Foundation:: BOOL);
    AssignProcessToJobObject(hjob.param().abi(), hprocess.param().abi()).ok()
}
#[cfg(feature = "Win32_Security")]
#[inline]
pub unsafe fn CreateJobObjectA<P0>(lpjobattributes: Option<*const super::super::Security::SECURITY_ATTRIBUTES>, lpname: P0) -> windows_core::Result<super::super::Foundation::HANDLE>
where
    P0: windows_core::Param<windows_core::PCSTR>,
{
    windows_targets::link!("kernel32.dll" "system" fn CreateJobObjectA(lpjobattributes : *const super::super::Security:: SECURITY_ATTRIBUTES, lpname : windows_core::PCSTR) -> super::super::Foundation:: HANDLE);
    let result__ = CreateJobObjectA(core::mem::transmute(lpjobattributes.unwrap_or(std::ptr::null())), lpname.param().abi());
    (!result__.is_invalid()).then(|| result__).ok_or_else(windows_core::Error::from_win32)
}
#[cfg(feature = "Win32_Security")]
#[inline]
pub unsafe fn CreateJobObjectW<P0>(lpjobattributes: Option<*const super::super::Security::SECURITY_ATTRIBUTES>, lpname: P0) -> windows_core::Result<super::super::Foundation::HANDLE>
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("kernel32.dll" "system" fn CreateJobObjectW(lpjobattributes : *const super::super::Security:: SECURITY_ATTRIBUTES, lpname : windows_core::PCWSTR) -> super::super::Foundation:: HANDLE);
    let result__ = CreateJobObjectW(core::mem::transmute(lpjobattributes.unwrap_or(std::ptr::null())), lpname.param().abi());
    (!result__.is_invalid()).then(|| result__).ok_or_else(windows_core::Error::from_win32)
}
#[inline]
pub unsafe fn CreateJobSet(userjobset: &[JOB_SET_ARRAY], flags: u32) -> super::super::Foundation::BOOL {
    windows_targets::link!("kernel32.dll" "system" fn CreateJobSet(numjob : u32, userjobset : *const JOB_SET_ARRAY, flags : u32) -> super::super::Foundation:: BOOL);
    CreateJobSet(userjobset.len().try_into().unwrap(), core::mem::transmute(userjobset.as_ptr()), flags)
}
#[inline]
pub unsafe fn FreeMemoryJobObject(buffer: *const core::ffi::c_void) {
    windows_targets::link!("kernel32.dll" "system" fn FreeMemoryJobObject(buffer : *const core::ffi::c_void));
    FreeMemoryJobObject(buffer)
}
#[inline]
pub unsafe fn IsProcessInJob<P0, P1>(processhandle: P0, jobhandle: P1, result: *mut super::super::Foundation::BOOL) -> windows_core::Result<()>
where
    P0: windows_core::Param<super::super::Foundation::HANDLE>,
    P1: windows_core::Param<super::super::Foundation::HANDLE>,
{
    windows_targets::link!("kernel32.dll" "system" fn IsProcessInJob(processhandle : super::super::Foundation:: HANDLE, jobhandle : super::super::Foundation:: HANDLE, result : *mut super::super::Foundation:: BOOL) -> super::super::Foundation:: BOOL);
    IsProcessInJob(processhandle.param().abi(), jobhandle.param().abi(), result).ok()
}
#[inline]
pub unsafe fn OpenJobObjectA<P0, P1>(dwdesiredaccess: u32, binherithandle: P0, lpname: P1) -> windows_core::Result<super::super::Foundation::HANDLE>
where
    P0: windows_core::Param<super::super::Foundation::BOOL>,
    P1: windows_core::Param<windows_core::PCSTR>,
{
    windows_targets::link!("kernel32.dll" "system" fn OpenJobObjectA(dwdesiredaccess : u32, binherithandle : super::super::Foundation:: BOOL, lpname : windows_core::PCSTR) -> super::super::Foundation:: HANDLE);
    let result__ = OpenJobObjectA(dwdesiredaccess, binherithandle.param().abi(), lpname.param().abi());
    (!result__.is_invalid()).then(|| result__).ok_or_else(windows_core::Error::from_win32)
}
#[inline]
pub unsafe fn OpenJobObjectW<P0, P1>(dwdesiredaccess: u32, binherithandle: P0, lpname: P1) -> windows_core::Result<super::super::Foundation::HANDLE>
where
    P0: windows_core::Param<super::super::Foundation::BOOL>,
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("kernel32.dll" "system" fn OpenJobObjectW(dwdesiredaccess : u32, binherithandle : super::super::Foundation:: BOOL, lpname : windows_core::PCWSTR) -> super::super::Foundation:: HANDLE);
    let result__ = OpenJobObjectW(dwdesiredaccess, binherithandle.param().abi(), lpname.param().abi());
    (!result__.is_invalid()).then(|| result__).ok_or_else(windows_core::Error::from_win32)
}
#[inline]
pub unsafe fn QueryInformationJobObject<P0>(hjob: P0, jobobjectinformationclass: JOBOBJECTINFOCLASS, lpjobobjectinformation: *mut core::ffi::c_void, cbjobobjectinformationlength: u32, lpreturnlength: Option<*mut u32>) -> windows_core::Result<()>
where
    P0: windows_core::Param<super::super::Foundation::HANDLE>,
{
    windows_targets::link!("kernel32.dll" "system" fn QueryInformationJobObject(hjob : super::super::Foundation:: HANDLE, jobobjectinformationclass : JOBOBJECTINFOCLASS, lpjobobjectinformation : *mut core::ffi::c_void, cbjobobjectinformationlength : u32, lpreturnlength : *mut u32) -> super::super::Foundation:: BOOL);
    QueryInformationJobObject(hjob.param().abi(), jobobjectinformationclass, lpjobobjectinformation, cbjobobjectinformationlength, core::mem::transmute(lpreturnlength.unwrap_or(std::ptr::null_mut()))).ok()
}
#[inline]
pub unsafe fn QueryIoRateControlInformationJobObject<P0, P1>(hjob: P0, volumename: P1, infoblocks: *mut *mut JOBOBJECT_IO_RATE_CONTROL_INFORMATION, infoblockcount: *mut u32) -> u32
where
    P0: windows_core::Param<super::super::Foundation::HANDLE>,
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_targets::link!("kernel32.dll" "system" fn QueryIoRateControlInformationJobObject(hjob : super::super::Foundation:: HANDLE, volumename : windows_core::PCWSTR, infoblocks : *mut *mut JOBOBJECT_IO_RATE_CONTROL_INFORMATION, infoblockcount : *mut u32) -> u32);
    QueryIoRateControlInformationJobObject(hjob.param().abi(), volumename.param().abi(), infoblocks, infoblockcount)
}
#[inline]
pub unsafe fn SetInformationJobObject<P0>(hjob: P0, jobobjectinformationclass: JOBOBJECTINFOCLASS, lpjobobjectinformation: *const core::ffi::c_void, cbjobobjectinformationlength: u32) -> windows_core::Result<()>
where
    P0: windows_core::Param<super::super::Foundation::HANDLE>,
{
    windows_targets::link!("kernel32.dll" "system" fn SetInformationJobObject(hjob : super::super::Foundation:: HANDLE, jobobjectinformationclass : JOBOBJECTINFOCLASS, lpjobobjectinformation : *const core::ffi::c_void, cbjobobjectinformationlength : u32) -> super::super::Foundation:: BOOL);
    SetInformationJobObject(hjob.param().abi(), jobobjectinformationclass, lpjobobjectinformation, cbjobobjectinformationlength).ok()
}
#[inline]
pub unsafe fn SetIoRateControlInformationJobObject<P0>(hjob: P0, ioratecontrolinfo: *const JOBOBJECT_IO_RATE_CONTROL_INFORMATION) -> u32
where
    P0: windows_core::Param<super::super::Foundation::HANDLE>,
{
    windows_targets::link!("kernel32.dll" "system" fn SetIoRateControlInformationJobObject(hjob : super::super::Foundation:: HANDLE, ioratecontrolinfo : *const JOBOBJECT_IO_RATE_CONTROL_INFORMATION) -> u32);
    SetIoRateControlInformationJobObject(hjob.param().abi(), ioratecontrolinfo)
}
#[inline]
pub unsafe fn TerminateJobObject<P0>(hjob: P0, uexitcode: u32) -> windows_core::Result<()>
where
    P0: windows_core::Param<super::super::Foundation::HANDLE>,
{
    windows_targets::link!("kernel32.dll" "system" fn TerminateJobObject(hjob : super::super::Foundation:: HANDLE, uexitcode : u32) -> super::super::Foundation:: BOOL);
    TerminateJobObject(hjob.param().abi(), uexitcode).ok()
}
#[inline]
pub unsafe fn UserHandleGrantAccess<P0, P1, P2>(huserhandle: P0, hjob: P1, bgrant: P2) -> windows_core::Result<()>
where
    P0: windows_core::Param<super::super::Foundation::HANDLE>,
    P1: windows_core::Param<super::super::Foundation::HANDLE>,
    P2: windows_core::Param<super::super::Foundation::BOOL>,
{
    windows_targets::link!("user32.dll" "system" fn UserHandleGrantAccess(huserhandle : super::super::Foundation:: HANDLE, hjob : super::super::Foundation:: HANDLE, bgrant : super::super::Foundation:: BOOL) -> super::super::Foundation:: BOOL);
    UserHandleGrantAccess(huserhandle.param().abi(), hjob.param().abi(), bgrant.param().abi()).ok()
}
pub const JOBOBJECT_IO_ATTRIBUTION_CONTROL_DISABLE: JOBOBJECT_IO_ATTRIBUTION_CONTROL_FLAGS = JOBOBJECT_IO_ATTRIBUTION_CONTROL_FLAGS(2i32);
pub const JOBOBJECT_IO_ATTRIBUTION_CONTROL_ENABLE: JOBOBJECT_IO_ATTRIBUTION_CONTROL_FLAGS = JOBOBJECT_IO_ATTRIBUTION_CONTROL_FLAGS(1i32);
pub const JOBOBJECT_IO_ATTRIBUTION_CONTROL_VALID_FLAGS: JOBOBJECT_IO_ATTRIBUTION_CONTROL_FLAGS = JOBOBJECT_IO_ATTRIBUTION_CONTROL_FLAGS(3i32);
pub const JOB_OBJECT_BASIC_LIMIT_VALID_FLAGS: JOB_OBJECT_LIMIT = JOB_OBJECT_LIMIT(255u32);
pub const JOB_OBJECT_CPU_RATE_CONTROL_ENABLE: JOB_OBJECT_CPU_RATE_CONTROL = JOB_OBJECT_CPU_RATE_CONTROL(1u32);
pub const JOB_OBJECT_CPU_RATE_CONTROL_HARD_CAP: JOB_OBJECT_CPU_RATE_CONTROL = JOB_OBJECT_CPU_RATE_CONTROL(4u32);
pub const JOB_OBJECT_CPU_RATE_CONTROL_MIN_MAX_RATE: JOB_OBJECT_CPU_RATE_CONTROL = JOB_OBJECT_CPU_RATE_CONTROL(16u32);
pub const JOB_OBJECT_CPU_RATE_CONTROL_NOTIFY: JOB_OBJECT_CPU_RATE_CONTROL = JOB_OBJECT_CPU_RATE_CONTROL(8u32);
pub const JOB_OBJECT_CPU_RATE_CONTROL_VALID_FLAGS: JOB_OBJECT_CPU_RATE_CONTROL = JOB_OBJECT_CPU_RATE_CONTROL(31u32);
pub const JOB_OBJECT_CPU_RATE_CONTROL_WEIGHT_BASED: JOB_OBJECT_CPU_RATE_CONTROL = JOB_OBJECT_CPU_RATE_CONTROL(2u32);
pub const JOB_OBJECT_EXTENDED_LIMIT_VALID_FLAGS: JOB_OBJECT_LIMIT = JOB_OBJECT_LIMIT(32767u32);
pub const JOB_OBJECT_IO_RATE_CONTROL_ENABLE: JOB_OBJECT_IO_RATE_CONTROL_FLAGS = JOB_OBJECT_IO_RATE_CONTROL_FLAGS(1i32);
pub const JOB_OBJECT_IO_RATE_CONTROL_FORCE_UNIT_ACCESS_ALL: JOB_OBJECT_IO_RATE_CONTROL_FLAGS = JOB_OBJECT_IO_RATE_CONTROL_FLAGS(4i32);
pub const JOB_OBJECT_IO_RATE_CONTROL_FORCE_UNIT_ACCESS_ON_SOFT_CAP: JOB_OBJECT_IO_RATE_CONTROL_FLAGS = JOB_OBJECT_IO_RATE_CONTROL_FLAGS(8i32);
pub const JOB_OBJECT_IO_RATE_CONTROL_STANDALONE_VOLUME: JOB_OBJECT_IO_RATE_CONTROL_FLAGS = JOB_OBJECT_IO_RATE_CONTROL_FLAGS(2i32);
pub const JOB_OBJECT_IO_RATE_CONTROL_VALID_FLAGS: JOB_OBJECT_IO_RATE_CONTROL_FLAGS = JOB_OBJECT_IO_RATE_CONTROL_FLAGS(15i32);
pub const JOB_OBJECT_LIMIT_ACTIVE_PROCESS: JOB_OBJECT_LIMIT = JOB_OBJECT_LIMIT(8u32);
pub const JOB_OBJECT_LIMIT_AFFINITY: JOB_OBJECT_LIMIT = JOB_OBJECT_LIMIT(16u32);
pub const JOB_OBJECT_LIMIT_BREAKAWAY_OK: JOB_OBJECT_LIMIT = JOB_OBJECT_LIMIT(2048u32);
pub const JOB_OBJECT_LIMIT_CPU_RATE_CONTROL: JOB_OBJECT_LIMIT = JOB_OBJECT_LIMIT(262144u32);
pub const JOB_OBJECT_LIMIT_DIE_ON_UNHANDLED_EXCEPTION: JOB_OBJECT_LIMIT = JOB_OBJECT_LIMIT(1024u32);
pub const JOB_OBJECT_LIMIT_IO_RATE_CONTROL: JOB_OBJECT_LIMIT = JOB_OBJECT_LIMIT(524288u32);
pub const JOB_OBJECT_LIMIT_JOB_MEMORY: JOB_OBJECT_LIMIT = JOB_OBJECT_LIMIT(512u32);
pub const JOB_OBJECT_LIMIT_JOB_MEMORY_HIGH: JOB_OBJECT_LIMIT = JOB_OBJECT_LIMIT(512u32);
pub const JOB_OBJECT_LIMIT_JOB_MEMORY_LOW: JOB_OBJECT_LIMIT = JOB_OBJECT_LIMIT(32768u32);
pub const JOB_OBJECT_LIMIT_JOB_READ_BYTES: JOB_OBJECT_LIMIT = JOB_OBJECT_LIMIT(65536u32);
pub const JOB_OBJECT_LIMIT_JOB_TIME: JOB_OBJECT_LIMIT = JOB_OBJECT_LIMIT(4u32);
pub const JOB_OBJECT_LIMIT_JOB_WRITE_BYTES: JOB_OBJECT_LIMIT = JOB_OBJECT_LIMIT(131072u32);
pub const JOB_OBJECT_LIMIT_KILL_ON_JOB_CLOSE: JOB_OBJECT_LIMIT = JOB_OBJECT_LIMIT(8192u32);
pub const JOB_OBJECT_LIMIT_NET_RATE_CONTROL: JOB_OBJECT_LIMIT = JOB_OBJECT_LIMIT(1048576u32);
pub const JOB_OBJECT_LIMIT_PRESERVE_JOB_TIME: JOB_OBJECT_LIMIT = JOB_OBJECT_LIMIT(64u32);
pub const JOB_OBJECT_LIMIT_PRIORITY_CLASS: JOB_OBJECT_LIMIT = JOB_OBJECT_LIMIT(32u32);
pub const JOB_OBJECT_LIMIT_PROCESS_MEMORY: JOB_OBJECT_LIMIT = JOB_OBJECT_LIMIT(256u32);
pub const JOB_OBJECT_LIMIT_PROCESS_TIME: JOB_OBJECT_LIMIT = JOB_OBJECT_LIMIT(2u32);
pub const JOB_OBJECT_LIMIT_RATE_CONTROL: JOB_OBJECT_LIMIT = JOB_OBJECT_LIMIT(262144u32);
pub const JOB_OBJECT_LIMIT_SCHEDULING_CLASS: JOB_OBJECT_LIMIT = JOB_OBJECT_LIMIT(128u32);
pub const JOB_OBJECT_LIMIT_SILENT_BREAKAWAY_OK: JOB_OBJECT_LIMIT = JOB_OBJECT_LIMIT(4096u32);
pub const JOB_OBJECT_LIMIT_SUBSET_AFFINITY: JOB_OBJECT_LIMIT = JOB_OBJECT_LIMIT(16384u32);
pub const JOB_OBJECT_LIMIT_VALID_FLAGS: JOB_OBJECT_LIMIT = JOB_OBJECT_LIMIT(524287u32);
pub const JOB_OBJECT_LIMIT_WORKINGSET: JOB_OBJECT_LIMIT = JOB_OBJECT_LIMIT(1u32);
pub const JOB_OBJECT_NET_RATE_CONTROL_DSCP_TAG: JOB_OBJECT_NET_RATE_CONTROL_FLAGS = JOB_OBJECT_NET_RATE_CONTROL_FLAGS(4i32);
pub const JOB_OBJECT_NET_RATE_CONTROL_ENABLE: JOB_OBJECT_NET_RATE_CONTROL_FLAGS = JOB_OBJECT_NET_RATE_CONTROL_FLAGS(1i32);
pub const JOB_OBJECT_NET_RATE_CONTROL_MAX_BANDWIDTH: JOB_OBJECT_NET_RATE_CONTROL_FLAGS = JOB_OBJECT_NET_RATE_CONTROL_FLAGS(2i32);
pub const JOB_OBJECT_NET_RATE_CONTROL_VALID_FLAGS: JOB_OBJECT_NET_RATE_CONTROL_FLAGS = JOB_OBJECT_NET_RATE_CONTROL_FLAGS(7i32);
pub const JOB_OBJECT_NOTIFICATION_LIMIT_VALID_FLAGS: JOB_OBJECT_LIMIT = JOB_OBJECT_LIMIT(2064900u32);
pub const JOB_OBJECT_POST_AT_END_OF_JOB: JOB_OBJECT_TERMINATE_AT_END_ACTION = JOB_OBJECT_TERMINATE_AT_END_ACTION(1u32);
pub const JOB_OBJECT_SECURITY_FILTER_TOKENS: JOB_OBJECT_SECURITY = JOB_OBJECT_SECURITY(8u32);
pub const JOB_OBJECT_SECURITY_NO_ADMIN: JOB_OBJECT_SECURITY = JOB_OBJECT_SECURITY(1u32);
pub const JOB_OBJECT_SECURITY_ONLY_TOKEN: JOB_OBJECT_SECURITY = JOB_OBJECT_SECURITY(4u32);
pub const JOB_OBJECT_SECURITY_RESTRICTED_TOKEN: JOB_OBJECT_SECURITY = JOB_OBJECT_SECURITY(2u32);
pub const JOB_OBJECT_SECURITY_VALID_FLAGS: JOB_OBJECT_SECURITY = JOB_OBJECT_SECURITY(15u32);
pub const JOB_OBJECT_TERMINATE_AT_END_OF_JOB: JOB_OBJECT_TERMINATE_AT_END_ACTION = JOB_OBJECT_TERMINATE_AT_END_ACTION(0u32);
pub const JOB_OBJECT_UILIMIT_DESKTOP: JOB_OBJECT_UILIMIT = JOB_OBJECT_UILIMIT(64u32);
pub const JOB_OBJECT_UILIMIT_DISPLAYSETTINGS: JOB_OBJECT_UILIMIT = JOB_OBJECT_UILIMIT(16u32);
pub const JOB_OBJECT_UILIMIT_EXITWINDOWS: JOB_OBJECT_UILIMIT = JOB_OBJECT_UILIMIT(128u32);
pub const JOB_OBJECT_UILIMIT_GLOBALATOMS: JOB_OBJECT_UILIMIT = JOB_OBJECT_UILIMIT(32u32);
pub const JOB_OBJECT_UILIMIT_HANDLES: JOB_OBJECT_UILIMIT = JOB_OBJECT_UILIMIT(1u32);
pub const JOB_OBJECT_UILIMIT_NONE: JOB_OBJECT_UILIMIT = JOB_OBJECT_UILIMIT(0u32);
pub const JOB_OBJECT_UILIMIT_READCLIPBOARD: JOB_OBJECT_UILIMIT = JOB_OBJECT_UILIMIT(2u32);
pub const JOB_OBJECT_UILIMIT_SYSTEMPARAMETERS: JOB_OBJECT_UILIMIT = JOB_OBJECT_UILIMIT(8u32);
pub const JOB_OBJECT_UILIMIT_WRITECLIPBOARD: JOB_OBJECT_UILIMIT = JOB_OBJECT_UILIMIT(4u32);
pub const JobObjectAssociateCompletionPortInformation: JOBOBJECTINFOCLASS = JOBOBJECTINFOCLASS(7i32);
pub const JobObjectBasicAccountingInformation: JOBOBJECTINFOCLASS = JOBOBJECTINFOCLASS(1i32);
pub const JobObjectBasicAndIoAccountingInformation: JOBOBJECTINFOCLASS = JOBOBJECTINFOCLASS(8i32);
pub const JobObjectBasicLimitInformation: JOBOBJECTINFOCLASS = JOBOBJECTINFOCLASS(2i32);
pub const JobObjectBasicProcessIdList: JOBOBJECTINFOCLASS = JOBOBJECTINFOCLASS(3i32);
pub const JobObjectBasicUIRestrictions: JOBOBJECTINFOCLASS = JOBOBJECTINFOCLASS(4i32);
pub const JobObjectCompletionCounter: JOBOBJECTINFOCLASS = JOBOBJECTINFOCLASS(17i32);
pub const JobObjectCompletionFilter: JOBOBJECTINFOCLASS = JOBOBJECTINFOCLASS(16i32);
pub const JobObjectCpuRateControlInformation: JOBOBJECTINFOCLASS = JOBOBJECTINFOCLASS(15i32);
pub const JobObjectCreateSilo: JOBOBJECTINFOCLASS = JOBOBJECTINFOCLASS(35i32);
pub const JobObjectEndOfJobTimeInformation: JOBOBJECTINFOCLASS = JOBOBJECTINFOCLASS(6i32);
pub const JobObjectExtendedLimitInformation: JOBOBJECTINFOCLASS = JOBOBJECTINFOCLASS(9i32);
pub const JobObjectGroupInformation: JOBOBJECTINFOCLASS = JOBOBJECTINFOCLASS(11i32);
pub const JobObjectGroupInformationEx: JOBOBJECTINFOCLASS = JOBOBJECTINFOCLASS(14i32);
pub const JobObjectJobSetInformation: JOBOBJECTINFOCLASS = JOBOBJECTINFOCLASS(10i32);
pub const JobObjectLimitViolationInformation: JOBOBJECTINFOCLASS = JOBOBJECTINFOCLASS(13i32);
pub const JobObjectLimitViolationInformation2: JOBOBJECTINFOCLASS = JOBOBJECTINFOCLASS(34i32);
pub const JobObjectNetRateControlInformation: JOBOBJECTINFOCLASS = JOBOBJECTINFOCLASS(32i32);
pub const JobObjectNotificationLimitInformation: JOBOBJECTINFOCLASS = JOBOBJECTINFOCLASS(12i32);
pub const JobObjectNotificationLimitInformation2: JOBOBJECTINFOCLASS = JOBOBJECTINFOCLASS(33i32);
pub const JobObjectReserved10Information: JOBOBJECTINFOCLASS = JOBOBJECTINFOCLASS(27i32);
pub const JobObjectReserved11Information: JOBOBJECTINFOCLASS = JOBOBJECTINFOCLASS(28i32);
pub const JobObjectReserved12Information: JOBOBJECTINFOCLASS = JOBOBJECTINFOCLASS(29i32);
pub const JobObjectReserved13Information: JOBOBJECTINFOCLASS = JOBOBJECTINFOCLASS(30i32);
pub const JobObjectReserved14Information: JOBOBJECTINFOCLASS = JOBOBJECTINFOCLASS(31i32);
pub const JobObjectReserved15Information: JOBOBJECTINFOCLASS = JOBOBJECTINFOCLASS(37i32);
pub const JobObjectReserved16Information: JOBOBJECTINFOCLASS = JOBOBJECTINFOCLASS(38i32);
pub const JobObjectReserved17Information: JOBOBJECTINFOCLASS = JOBOBJECTINFOCLASS(39i32);
pub const JobObjectReserved18Information: JOBOBJECTINFOCLASS = JOBOBJECTINFOCLASS(40i32);
pub const JobObjectReserved19Information: JOBOBJECTINFOCLASS = JOBOBJECTINFOCLASS(41i32);
pub const JobObjectReserved1Information: JOBOBJECTINFOCLASS = JOBOBJECTINFOCLASS(18i32);
pub const JobObjectReserved20Information: JOBOBJECTINFOCLASS = JOBOBJECTINFOCLASS(42i32);
pub const JobObjectReserved21Information: JOBOBJECTINFOCLASS = JOBOBJECTINFOCLASS(43i32);
pub const JobObjectReserved22Information: JOBOBJECTINFOCLASS = JOBOBJECTINFOCLASS(44i32);
pub const JobObjectReserved23Information: JOBOBJECTINFOCLASS = JOBOBJECTINFOCLASS(45i32);
pub const JobObjectReserved24Information: JOBOBJECTINFOCLASS = JOBOBJECTINFOCLASS(46i32);
pub const JobObjectReserved25Information: JOBOBJECTINFOCLASS = JOBOBJECTINFOCLASS(47i32);
pub const JobObjectReserved26Information: JOBOBJECTINFOCLASS = JOBOBJECTINFOCLASS(48i32);
pub const JobObjectReserved27Information: JOBOBJECTINFOCLASS = JOBOBJECTINFOCLASS(49i32);
pub const JobObjectReserved2Information: JOBOBJECTINFOCLASS = JOBOBJECTINFOCLASS(19i32);
pub const JobObjectReserved3Information: JOBOBJECTINFOCLASS = JOBOBJECTINFOCLASS(20i32);
pub const JobObjectReserved4Information: JOBOBJECTINFOCLASS = JOBOBJECTINFOCLASS(21i32);
pub const JobObjectReserved5Information: JOBOBJECTINFOCLASS = JOBOBJECTINFOCLASS(22i32);
pub const JobObjectReserved6Information: JOBOBJECTINFOCLASS = JOBOBJECTINFOCLASS(23i32);
pub const JobObjectReserved7Information: JOBOBJECTINFOCLASS = JOBOBJECTINFOCLASS(24i32);
pub const JobObjectReserved8Information: JOBOBJECTINFOCLASS = JOBOBJECTINFOCLASS(25i32);
pub const JobObjectReserved9Information: JOBOBJECTINFOCLASS = JOBOBJECTINFOCLASS(26i32);
pub const JobObjectSecurityLimitInformation: JOBOBJECTINFOCLASS = JOBOBJECTINFOCLASS(5i32);
pub const JobObjectSiloBasicInformation: JOBOBJECTINFOCLASS = JOBOBJECTINFOCLASS(36i32);
pub const MaxJobObjectInfoClass: JOBOBJECTINFOCLASS = JOBOBJECTINFOCLASS(50i32);
pub const ToleranceHigh: JOBOBJECT_RATE_CONTROL_TOLERANCE = JOBOBJECT_RATE_CONTROL_TOLERANCE(3i32);
pub const ToleranceIntervalLong: JOBOBJECT_RATE_CONTROL_TOLERANCE_INTERVAL = JOBOBJECT_RATE_CONTROL_TOLERANCE_INTERVAL(3i32);
pub const ToleranceIntervalMedium: JOBOBJECT_RATE_CONTROL_TOLERANCE_INTERVAL = JOBOBJECT_RATE_CONTROL_TOLERANCE_INTERVAL(2i32);
pub const ToleranceIntervalShort: JOBOBJECT_RATE_CONTROL_TOLERANCE_INTERVAL = JOBOBJECT_RATE_CONTROL_TOLERANCE_INTERVAL(1i32);
pub const ToleranceLow: JOBOBJECT_RATE_CONTROL_TOLERANCE = JOBOBJECT_RATE_CONTROL_TOLERANCE(1i32);
pub const ToleranceMedium: JOBOBJECT_RATE_CONTROL_TOLERANCE = JOBOBJECT_RATE_CONTROL_TOLERANCE(2i32);
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct JOBOBJECTINFOCLASS(pub i32);
impl windows_core::TypeKind for JOBOBJECTINFOCLASS {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for JOBOBJECTINFOCLASS {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("JOBOBJECTINFOCLASS").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct JOBOBJECT_IO_ATTRIBUTION_CONTROL_FLAGS(pub i32);
impl windows_core::TypeKind for JOBOBJECT_IO_ATTRIBUTION_CONTROL_FLAGS {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for JOBOBJECT_IO_ATTRIBUTION_CONTROL_FLAGS {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("JOBOBJECT_IO_ATTRIBUTION_CONTROL_FLAGS").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct JOBOBJECT_RATE_CONTROL_TOLERANCE(pub i32);
impl windows_core::TypeKind for JOBOBJECT_RATE_CONTROL_TOLERANCE {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for JOBOBJECT_RATE_CONTROL_TOLERANCE {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("JOBOBJECT_RATE_CONTROL_TOLERANCE").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct JOBOBJECT_RATE_CONTROL_TOLERANCE_INTERVAL(pub i32);
impl windows_core::TypeKind for JOBOBJECT_RATE_CONTROL_TOLERANCE_INTERVAL {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for JOBOBJECT_RATE_CONTROL_TOLERANCE_INTERVAL {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("JOBOBJECT_RATE_CONTROL_TOLERANCE_INTERVAL").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct JOB_OBJECT_CPU_RATE_CONTROL(pub u32);
impl windows_core::TypeKind for JOB_OBJECT_CPU_RATE_CONTROL {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for JOB_OBJECT_CPU_RATE_CONTROL {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("JOB_OBJECT_CPU_RATE_CONTROL").field(&self.0).finish()
    }
}
impl JOB_OBJECT_CPU_RATE_CONTROL {
    pub const fn contains(&self, other: Self) -> bool {
        self.0 & other.0 == other.0
    }
}
impl core::ops::BitOr for JOB_OBJECT_CPU_RATE_CONTROL {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl core::ops::BitAnd for JOB_OBJECT_CPU_RATE_CONTROL {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl core::ops::BitOrAssign for JOB_OBJECT_CPU_RATE_CONTROL {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl core::ops::BitAndAssign for JOB_OBJECT_CPU_RATE_CONTROL {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl core::ops::Not for JOB_OBJECT_CPU_RATE_CONTROL {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct JOB_OBJECT_IO_RATE_CONTROL_FLAGS(pub i32);
impl windows_core::TypeKind for JOB_OBJECT_IO_RATE_CONTROL_FLAGS {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for JOB_OBJECT_IO_RATE_CONTROL_FLAGS {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("JOB_OBJECT_IO_RATE_CONTROL_FLAGS").field(&self.0).finish()
    }
}
impl JOB_OBJECT_IO_RATE_CONTROL_FLAGS {
    pub const fn contains(&self, other: Self) -> bool {
        self.0 & other.0 == other.0
    }
}
impl core::ops::BitOr for JOB_OBJECT_IO_RATE_CONTROL_FLAGS {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl core::ops::BitAnd for JOB_OBJECT_IO_RATE_CONTROL_FLAGS {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl core::ops::BitOrAssign for JOB_OBJECT_IO_RATE_CONTROL_FLAGS {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl core::ops::BitAndAssign for JOB_OBJECT_IO_RATE_CONTROL_FLAGS {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl core::ops::Not for JOB_OBJECT_IO_RATE_CONTROL_FLAGS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct JOB_OBJECT_LIMIT(pub u32);
impl windows_core::TypeKind for JOB_OBJECT_LIMIT {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for JOB_OBJECT_LIMIT {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("JOB_OBJECT_LIMIT").field(&self.0).finish()
    }
}
impl JOB_OBJECT_LIMIT {
    pub const fn contains(&self, other: Self) -> bool {
        self.0 & other.0 == other.0
    }
}
impl core::ops::BitOr for JOB_OBJECT_LIMIT {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl core::ops::BitAnd for JOB_OBJECT_LIMIT {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl core::ops::BitOrAssign for JOB_OBJECT_LIMIT {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl core::ops::BitAndAssign for JOB_OBJECT_LIMIT {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl core::ops::Not for JOB_OBJECT_LIMIT {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct JOB_OBJECT_NET_RATE_CONTROL_FLAGS(pub i32);
impl windows_core::TypeKind for JOB_OBJECT_NET_RATE_CONTROL_FLAGS {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for JOB_OBJECT_NET_RATE_CONTROL_FLAGS {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("JOB_OBJECT_NET_RATE_CONTROL_FLAGS").field(&self.0).finish()
    }
}
impl JOB_OBJECT_NET_RATE_CONTROL_FLAGS {
    pub const fn contains(&self, other: Self) -> bool {
        self.0 & other.0 == other.0
    }
}
impl core::ops::BitOr for JOB_OBJECT_NET_RATE_CONTROL_FLAGS {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl core::ops::BitAnd for JOB_OBJECT_NET_RATE_CONTROL_FLAGS {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl core::ops::BitOrAssign for JOB_OBJECT_NET_RATE_CONTROL_FLAGS {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl core::ops::BitAndAssign for JOB_OBJECT_NET_RATE_CONTROL_FLAGS {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl core::ops::Not for JOB_OBJECT_NET_RATE_CONTROL_FLAGS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct JOB_OBJECT_SECURITY(pub u32);
impl windows_core::TypeKind for JOB_OBJECT_SECURITY {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for JOB_OBJECT_SECURITY {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("JOB_OBJECT_SECURITY").field(&self.0).finish()
    }
}
impl JOB_OBJECT_SECURITY {
    pub const fn contains(&self, other: Self) -> bool {
        self.0 & other.0 == other.0
    }
}
impl core::ops::BitOr for JOB_OBJECT_SECURITY {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl core::ops::BitAnd for JOB_OBJECT_SECURITY {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl core::ops::BitOrAssign for JOB_OBJECT_SECURITY {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl core::ops::BitAndAssign for JOB_OBJECT_SECURITY {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl core::ops::Not for JOB_OBJECT_SECURITY {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct JOB_OBJECT_TERMINATE_AT_END_ACTION(pub u32);
impl windows_core::TypeKind for JOB_OBJECT_TERMINATE_AT_END_ACTION {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for JOB_OBJECT_TERMINATE_AT_END_ACTION {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("JOB_OBJECT_TERMINATE_AT_END_ACTION").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub struct JOB_OBJECT_UILIMIT(pub u32);
impl windows_core::TypeKind for JOB_OBJECT_UILIMIT {
    type TypeKind = windows_core::CopyType;
}
impl core::fmt::Debug for JOB_OBJECT_UILIMIT {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("JOB_OBJECT_UILIMIT").field(&self.0).finish()
    }
}
impl JOB_OBJECT_UILIMIT {
    pub const fn contains(&self, other: Self) -> bool {
        self.0 & other.0 == other.0
    }
}
impl core::ops::BitOr for JOB_OBJECT_UILIMIT {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl core::ops::BitAnd for JOB_OBJECT_UILIMIT {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl core::ops::BitOrAssign for JOB_OBJECT_UILIMIT {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl core::ops::BitAndAssign for JOB_OBJECT_UILIMIT {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl core::ops::Not for JOB_OBJECT_UILIMIT {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct JOBOBJECT_ASSOCIATE_COMPLETION_PORT {
    pub CompletionKey: *mut core::ffi::c_void,
    pub CompletionPort: super::super::Foundation::HANDLE,
}
impl windows_core::TypeKind for JOBOBJECT_ASSOCIATE_COMPLETION_PORT {
    type TypeKind = windows_core::CopyType;
}
impl Default for JOBOBJECT_ASSOCIATE_COMPLETION_PORT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct JOBOBJECT_BASIC_ACCOUNTING_INFORMATION {
    pub TotalUserTime: i64,
    pub TotalKernelTime: i64,
    pub ThisPeriodTotalUserTime: i64,
    pub ThisPeriodTotalKernelTime: i64,
    pub TotalPageFaultCount: u32,
    pub TotalProcesses: u32,
    pub ActiveProcesses: u32,
    pub TotalTerminatedProcesses: u32,
}
impl windows_core::TypeKind for JOBOBJECT_BASIC_ACCOUNTING_INFORMATION {
    type TypeKind = windows_core::CopyType;
}
impl Default for JOBOBJECT_BASIC_ACCOUNTING_INFORMATION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "Win32_System_Threading")]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct JOBOBJECT_BASIC_AND_IO_ACCOUNTING_INFORMATION {
    pub BasicInfo: JOBOBJECT_BASIC_ACCOUNTING_INFORMATION,
    pub IoInfo: super::Threading::IO_COUNTERS,
}
#[cfg(feature = "Win32_System_Threading")]
impl windows_core::TypeKind for JOBOBJECT_BASIC_AND_IO_ACCOUNTING_INFORMATION {
    type TypeKind = windows_core::CopyType;
}
#[cfg(feature = "Win32_System_Threading")]
impl Default for JOBOBJECT_BASIC_AND_IO_ACCOUNTING_INFORMATION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct JOBOBJECT_BASIC_LIMIT_INFORMATION {
    pub PerProcessUserTimeLimit: i64,
    pub PerJobUserTimeLimit: i64,
    pub LimitFlags: JOB_OBJECT_LIMIT,
    pub MinimumWorkingSetSize: usize,
    pub MaximumWorkingSetSize: usize,
    pub ActiveProcessLimit: u32,
    pub Affinity: usize,
    pub PriorityClass: u32,
    pub SchedulingClass: u32,
}
impl windows_core::TypeKind for JOBOBJECT_BASIC_LIMIT_INFORMATION {
    type TypeKind = windows_core::CopyType;
}
impl Default for JOBOBJECT_BASIC_LIMIT_INFORMATION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct JOBOBJECT_BASIC_PROCESS_ID_LIST {
    pub NumberOfAssignedProcesses: u32,
    pub NumberOfProcessIdsInList: u32,
    pub ProcessIdList: [usize; 1],
}
impl windows_core::TypeKind for JOBOBJECT_BASIC_PROCESS_ID_LIST {
    type TypeKind = windows_core::CopyType;
}
impl Default for JOBOBJECT_BASIC_PROCESS_ID_LIST {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct JOBOBJECT_BASIC_UI_RESTRICTIONS {
    pub UIRestrictionsClass: JOB_OBJECT_UILIMIT,
}
impl windows_core::TypeKind for JOBOBJECT_BASIC_UI_RESTRICTIONS {
    type TypeKind = windows_core::CopyType;
}
impl Default for JOBOBJECT_BASIC_UI_RESTRICTIONS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct JOBOBJECT_CPU_RATE_CONTROL_INFORMATION {
    pub ControlFlags: JOB_OBJECT_CPU_RATE_CONTROL,
    pub Anonymous: JOBOBJECT_CPU_RATE_CONTROL_INFORMATION_0,
}
impl windows_core::TypeKind for JOBOBJECT_CPU_RATE_CONTROL_INFORMATION {
    type TypeKind = windows_core::CopyType;
}
impl Default for JOBOBJECT_CPU_RATE_CONTROL_INFORMATION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union JOBOBJECT_CPU_RATE_CONTROL_INFORMATION_0 {
    pub CpuRate: u32,
    pub Weight: u32,
    pub Anonymous: JOBOBJECT_CPU_RATE_CONTROL_INFORMATION_0_0,
}
impl windows_core::TypeKind for JOBOBJECT_CPU_RATE_CONTROL_INFORMATION_0 {
    type TypeKind = windows_core::CopyType;
}
impl Default for JOBOBJECT_CPU_RATE_CONTROL_INFORMATION_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct JOBOBJECT_CPU_RATE_CONTROL_INFORMATION_0_0 {
    pub MinRate: u16,
    pub MaxRate: u16,
}
impl windows_core::TypeKind for JOBOBJECT_CPU_RATE_CONTROL_INFORMATION_0_0 {
    type TypeKind = windows_core::CopyType;
}
impl Default for JOBOBJECT_CPU_RATE_CONTROL_INFORMATION_0_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct JOBOBJECT_END_OF_JOB_TIME_INFORMATION {
    pub EndOfJobTimeAction: JOB_OBJECT_TERMINATE_AT_END_ACTION,
}
impl windows_core::TypeKind for JOBOBJECT_END_OF_JOB_TIME_INFORMATION {
    type TypeKind = windows_core::CopyType;
}
impl Default for JOBOBJECT_END_OF_JOB_TIME_INFORMATION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "Win32_System_Threading")]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct JOBOBJECT_EXTENDED_LIMIT_INFORMATION {
    pub BasicLimitInformation: JOBOBJECT_BASIC_LIMIT_INFORMATION,
    pub IoInfo: super::Threading::IO_COUNTERS,
    pub ProcessMemoryLimit: usize,
    pub JobMemoryLimit: usize,
    pub PeakProcessMemoryUsed: usize,
    pub PeakJobMemoryUsed: usize,
}
#[cfg(feature = "Win32_System_Threading")]
impl windows_core::TypeKind for JOBOBJECT_EXTENDED_LIMIT_INFORMATION {
    type TypeKind = windows_core::CopyType;
}
#[cfg(feature = "Win32_System_Threading")]
impl Default for JOBOBJECT_EXTENDED_LIMIT_INFORMATION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct JOBOBJECT_IO_ATTRIBUTION_INFORMATION {
    pub ControlFlags: u32,
    pub ReadStats: JOBOBJECT_IO_ATTRIBUTION_STATS,
    pub WriteStats: JOBOBJECT_IO_ATTRIBUTION_STATS,
}
impl windows_core::TypeKind for JOBOBJECT_IO_ATTRIBUTION_INFORMATION {
    type TypeKind = windows_core::CopyType;
}
impl Default for JOBOBJECT_IO_ATTRIBUTION_INFORMATION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct JOBOBJECT_IO_ATTRIBUTION_STATS {
    pub IoCount: usize,
    pub TotalNonOverlappedQueueTime: u64,
    pub TotalNonOverlappedServiceTime: u64,
    pub TotalSize: u64,
}
impl windows_core::TypeKind for JOBOBJECT_IO_ATTRIBUTION_STATS {
    type TypeKind = windows_core::CopyType;
}
impl Default for JOBOBJECT_IO_ATTRIBUTION_STATS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct JOBOBJECT_IO_RATE_CONTROL_INFORMATION {
    pub MaxIops: i64,
    pub MaxBandwidth: i64,
    pub ReservationIops: i64,
    pub VolumeName: windows_core::PCWSTR,
    pub BaseIoSize: u32,
    pub ControlFlags: u32,
}
impl windows_core::TypeKind for JOBOBJECT_IO_RATE_CONTROL_INFORMATION {
    type TypeKind = windows_core::CopyType;
}
impl Default for JOBOBJECT_IO_RATE_CONTROL_INFORMATION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct JOBOBJECT_IO_RATE_CONTROL_INFORMATION_NATIVE_V1 {
    pub MaxIops: i64,
    pub MaxBandwidth: i64,
    pub ReservationIops: i64,
    pub VolumeName: windows_core::PWSTR,
    pub BaseIoSize: u32,
    pub ControlFlags: JOB_OBJECT_IO_RATE_CONTROL_FLAGS,
    pub VolumeNameLength: u16,
}
impl windows_core::TypeKind for JOBOBJECT_IO_RATE_CONTROL_INFORMATION_NATIVE_V1 {
    type TypeKind = windows_core::CopyType;
}
impl Default for JOBOBJECT_IO_RATE_CONTROL_INFORMATION_NATIVE_V1 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct JOBOBJECT_IO_RATE_CONTROL_INFORMATION_NATIVE_V2 {
    pub MaxIops: i64,
    pub MaxBandwidth: i64,
    pub ReservationIops: i64,
    pub VolumeName: windows_core::PWSTR,
    pub BaseIoSize: u32,
    pub ControlFlags: JOB_OBJECT_IO_RATE_CONTROL_FLAGS,
    pub VolumeNameLength: u16,
    pub CriticalReservationIops: i64,
    pub ReservationBandwidth: i64,
    pub CriticalReservationBandwidth: i64,
    pub MaxTimePercent: i64,
    pub ReservationTimePercent: i64,
    pub CriticalReservationTimePercent: i64,
}
impl windows_core::TypeKind for JOBOBJECT_IO_RATE_CONTROL_INFORMATION_NATIVE_V2 {
    type TypeKind = windows_core::CopyType;
}
impl Default for JOBOBJECT_IO_RATE_CONTROL_INFORMATION_NATIVE_V2 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct JOBOBJECT_IO_RATE_CONTROL_INFORMATION_NATIVE_V3 {
    pub MaxIops: i64,
    pub MaxBandwidth: i64,
    pub ReservationIops: i64,
    pub VolumeName: windows_core::PWSTR,
    pub BaseIoSize: u32,
    pub ControlFlags: JOB_OBJECT_IO_RATE_CONTROL_FLAGS,
    pub VolumeNameLength: u16,
    pub CriticalReservationIops: i64,
    pub ReservationBandwidth: i64,
    pub CriticalReservationBandwidth: i64,
    pub MaxTimePercent: i64,
    pub ReservationTimePercent: i64,
    pub CriticalReservationTimePercent: i64,
    pub SoftMaxIops: i64,
    pub SoftMaxBandwidth: i64,
    pub SoftMaxTimePercent: i64,
    pub LimitExcessNotifyIops: i64,
    pub LimitExcessNotifyBandwidth: i64,
    pub LimitExcessNotifyTimePercent: i64,
}
impl windows_core::TypeKind for JOBOBJECT_IO_RATE_CONTROL_INFORMATION_NATIVE_V3 {
    type TypeKind = windows_core::CopyType;
}
impl Default for JOBOBJECT_IO_RATE_CONTROL_INFORMATION_NATIVE_V3 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct JOBOBJECT_JOBSET_INFORMATION {
    pub MemberLevel: u32,
}
impl windows_core::TypeKind for JOBOBJECT_JOBSET_INFORMATION {
    type TypeKind = windows_core::CopyType;
}
impl Default for JOBOBJECT_JOBSET_INFORMATION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct JOBOBJECT_LIMIT_VIOLATION_INFORMATION {
    pub LimitFlags: JOB_OBJECT_LIMIT,
    pub ViolationLimitFlags: JOB_OBJECT_LIMIT,
    pub IoReadBytes: u64,
    pub IoReadBytesLimit: u64,
    pub IoWriteBytes: u64,
    pub IoWriteBytesLimit: u64,
    pub PerJobUserTime: i64,
    pub PerJobUserTimeLimit: i64,
    pub JobMemory: u64,
    pub JobMemoryLimit: u64,
    pub RateControlTolerance: JOBOBJECT_RATE_CONTROL_TOLERANCE,
    pub RateControlToleranceLimit: JOBOBJECT_RATE_CONTROL_TOLERANCE,
}
impl windows_core::TypeKind for JOBOBJECT_LIMIT_VIOLATION_INFORMATION {
    type TypeKind = windows_core::CopyType;
}
impl Default for JOBOBJECT_LIMIT_VIOLATION_INFORMATION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct JOBOBJECT_LIMIT_VIOLATION_INFORMATION_2 {
    pub LimitFlags: JOB_OBJECT_LIMIT,
    pub ViolationLimitFlags: JOB_OBJECT_LIMIT,
    pub IoReadBytes: u64,
    pub IoReadBytesLimit: u64,
    pub IoWriteBytes: u64,
    pub IoWriteBytesLimit: u64,
    pub PerJobUserTime: i64,
    pub PerJobUserTimeLimit: i64,
    pub JobMemory: u64,
    pub Anonymous1: JOBOBJECT_LIMIT_VIOLATION_INFORMATION_2_0,
    pub Anonymous2: JOBOBJECT_LIMIT_VIOLATION_INFORMATION_2_1,
    pub Anonymous3: JOBOBJECT_LIMIT_VIOLATION_INFORMATION_2_2,
    pub JobLowMemoryLimit: u64,
    pub IoRateControlTolerance: JOBOBJECT_RATE_CONTROL_TOLERANCE,
    pub IoRateControlToleranceLimit: JOBOBJECT_RATE_CONTROL_TOLERANCE,
    pub NetRateControlTolerance: JOBOBJECT_RATE_CONTROL_TOLERANCE,
    pub NetRateControlToleranceLimit: JOBOBJECT_RATE_CONTROL_TOLERANCE,
}
impl windows_core::TypeKind for JOBOBJECT_LIMIT_VIOLATION_INFORMATION_2 {
    type TypeKind = windows_core::CopyType;
}
impl Default for JOBOBJECT_LIMIT_VIOLATION_INFORMATION_2 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union JOBOBJECT_LIMIT_VIOLATION_INFORMATION_2_0 {
    pub JobHighMemoryLimit: u64,
    pub JobMemoryLimit: u64,
}
impl windows_core::TypeKind for JOBOBJECT_LIMIT_VIOLATION_INFORMATION_2_0 {
    type TypeKind = windows_core::CopyType;
}
impl Default for JOBOBJECT_LIMIT_VIOLATION_INFORMATION_2_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union JOBOBJECT_LIMIT_VIOLATION_INFORMATION_2_1 {
    pub RateControlTolerance: JOBOBJECT_RATE_CONTROL_TOLERANCE,
    pub CpuRateControlTolerance: JOBOBJECT_RATE_CONTROL_TOLERANCE,
}
impl windows_core::TypeKind for JOBOBJECT_LIMIT_VIOLATION_INFORMATION_2_1 {
    type TypeKind = windows_core::CopyType;
}
impl Default for JOBOBJECT_LIMIT_VIOLATION_INFORMATION_2_1 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union JOBOBJECT_LIMIT_VIOLATION_INFORMATION_2_2 {
    pub RateControlToleranceLimit: JOBOBJECT_RATE_CONTROL_TOLERANCE,
    pub CpuRateControlToleranceLimit: JOBOBJECT_RATE_CONTROL_TOLERANCE,
}
impl windows_core::TypeKind for JOBOBJECT_LIMIT_VIOLATION_INFORMATION_2_2 {
    type TypeKind = windows_core::CopyType;
}
impl Default for JOBOBJECT_LIMIT_VIOLATION_INFORMATION_2_2 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct JOBOBJECT_NET_RATE_CONTROL_INFORMATION {
    pub MaxBandwidth: u64,
    pub ControlFlags: JOB_OBJECT_NET_RATE_CONTROL_FLAGS,
    pub DscpTag: u8,
}
impl windows_core::TypeKind for JOBOBJECT_NET_RATE_CONTROL_INFORMATION {
    type TypeKind = windows_core::CopyType;
}
impl Default for JOBOBJECT_NET_RATE_CONTROL_INFORMATION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct JOBOBJECT_NOTIFICATION_LIMIT_INFORMATION {
    pub IoReadBytesLimit: u64,
    pub IoWriteBytesLimit: u64,
    pub PerJobUserTimeLimit: i64,
    pub JobMemoryLimit: u64,
    pub RateControlTolerance: JOBOBJECT_RATE_CONTROL_TOLERANCE,
    pub RateControlToleranceInterval: JOBOBJECT_RATE_CONTROL_TOLERANCE_INTERVAL,
    pub LimitFlags: JOB_OBJECT_LIMIT,
}
impl windows_core::TypeKind for JOBOBJECT_NOTIFICATION_LIMIT_INFORMATION {
    type TypeKind = windows_core::CopyType;
}
impl Default for JOBOBJECT_NOTIFICATION_LIMIT_INFORMATION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct JOBOBJECT_NOTIFICATION_LIMIT_INFORMATION_2 {
    pub IoReadBytesLimit: u64,
    pub IoWriteBytesLimit: u64,
    pub PerJobUserTimeLimit: i64,
    pub Anonymous1: JOBOBJECT_NOTIFICATION_LIMIT_INFORMATION_2_0,
    pub Anonymous2: JOBOBJECT_NOTIFICATION_LIMIT_INFORMATION_2_1,
    pub Anonymous3: JOBOBJECT_NOTIFICATION_LIMIT_INFORMATION_2_2,
    pub LimitFlags: JOB_OBJECT_LIMIT,
    pub IoRateControlTolerance: JOBOBJECT_RATE_CONTROL_TOLERANCE,
    pub JobLowMemoryLimit: u64,
    pub IoRateControlToleranceInterval: JOBOBJECT_RATE_CONTROL_TOLERANCE_INTERVAL,
    pub NetRateControlTolerance: JOBOBJECT_RATE_CONTROL_TOLERANCE,
    pub NetRateControlToleranceInterval: JOBOBJECT_RATE_CONTROL_TOLERANCE_INTERVAL,
}
impl windows_core::TypeKind for JOBOBJECT_NOTIFICATION_LIMIT_INFORMATION_2 {
    type TypeKind = windows_core::CopyType;
}
impl Default for JOBOBJECT_NOTIFICATION_LIMIT_INFORMATION_2 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union JOBOBJECT_NOTIFICATION_LIMIT_INFORMATION_2_0 {
    pub JobHighMemoryLimit: u64,
    pub JobMemoryLimit: u64,
}
impl windows_core::TypeKind for JOBOBJECT_NOTIFICATION_LIMIT_INFORMATION_2_0 {
    type TypeKind = windows_core::CopyType;
}
impl Default for JOBOBJECT_NOTIFICATION_LIMIT_INFORMATION_2_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union JOBOBJECT_NOTIFICATION_LIMIT_INFORMATION_2_1 {
    pub RateControlTolerance: JOBOBJECT_RATE_CONTROL_TOLERANCE,
    pub CpuRateControlTolerance: JOBOBJECT_RATE_CONTROL_TOLERANCE,
}
impl windows_core::TypeKind for JOBOBJECT_NOTIFICATION_LIMIT_INFORMATION_2_1 {
    type TypeKind = windows_core::CopyType;
}
impl Default for JOBOBJECT_NOTIFICATION_LIMIT_INFORMATION_2_1 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union JOBOBJECT_NOTIFICATION_LIMIT_INFORMATION_2_2 {
    pub RateControlToleranceInterval: JOBOBJECT_RATE_CONTROL_TOLERANCE_INTERVAL,
    pub CpuRateControlToleranceInterval: JOBOBJECT_RATE_CONTROL_TOLERANCE_INTERVAL,
}
impl windows_core::TypeKind for JOBOBJECT_NOTIFICATION_LIMIT_INFORMATION_2_2 {
    type TypeKind = windows_core::CopyType;
}
impl Default for JOBOBJECT_NOTIFICATION_LIMIT_INFORMATION_2_2 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "Win32_Security")]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct JOBOBJECT_SECURITY_LIMIT_INFORMATION {
    pub SecurityLimitFlags: JOB_OBJECT_SECURITY,
    pub JobToken: super::super::Foundation::HANDLE,
    pub SidsToDisable: *mut super::super::Security::TOKEN_GROUPS,
    pub PrivilegesToDelete: *mut super::super::Security::TOKEN_PRIVILEGES,
    pub RestrictedSids: *mut super::super::Security::TOKEN_GROUPS,
}
#[cfg(feature = "Win32_Security")]
impl windows_core::TypeKind for JOBOBJECT_SECURITY_LIMIT_INFORMATION {
    type TypeKind = windows_core::CopyType;
}
#[cfg(feature = "Win32_Security")]
impl Default for JOBOBJECT_SECURITY_LIMIT_INFORMATION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct JOB_SET_ARRAY {
    pub JobHandle: super::super::Foundation::HANDLE,
    pub MemberLevel: u32,
    pub Flags: u32,
}
impl windows_core::TypeKind for JOB_SET_ARRAY {
    type TypeKind = windows_core::CopyType;
}
impl Default for JOB_SET_ARRAY {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

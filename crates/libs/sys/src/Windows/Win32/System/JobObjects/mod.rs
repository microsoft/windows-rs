windows_link::link!("kernel32.dll" "system" fn AssignProcessToJobObject(hjob : super::super::Foundation::HANDLE, hprocess : super::super::Foundation::HANDLE) -> windows_sys::core::BOOL);
#[cfg(feature = "Win32_Security")]
windows_link::link!("kernel32.dll" "system" fn CreateJobObjectA(lpjobattributes : *const super::super::Security::SECURITY_ATTRIBUTES, lpname : windows_sys::core::PCSTR) -> super::super::Foundation::HANDLE);
#[cfg(feature = "Win32_Security")]
windows_link::link!("kernel32.dll" "system" fn CreateJobObjectW(lpjobattributes : *const super::super::Security::SECURITY_ATTRIBUTES, lpname : windows_sys::core::PCWSTR) -> super::super::Foundation::HANDLE);
windows_link::link!("kernel32.dll" "system" fn CreateJobSet(numjob : u32, userjobset : *const JOB_SET_ARRAY, flags : u32) -> windows_sys::core::BOOL);
windows_link::link!("kernel32.dll" "system" fn FreeMemoryJobObject(buffer : *const core::ffi::c_void));
windows_link::link!("kernel32.dll" "system" fn IsProcessInJob(processhandle : super::super::Foundation::HANDLE, jobhandle : super::super::Foundation::HANDLE, result : *mut windows_sys::core::BOOL) -> windows_sys::core::BOOL);
windows_link::link!("kernel32.dll" "system" fn OpenJobObjectA(dwdesiredaccess : u32, binherithandle : windows_sys::core::BOOL, lpname : windows_sys::core::PCSTR) -> super::super::Foundation::HANDLE);
windows_link::link!("kernel32.dll" "system" fn OpenJobObjectW(dwdesiredaccess : u32, binherithandle : windows_sys::core::BOOL, lpname : windows_sys::core::PCWSTR) -> super::super::Foundation::HANDLE);
windows_link::link!("kernel32.dll" "system" fn QueryInformationJobObject(hjob : super::super::Foundation::HANDLE, jobobjectinformationclass : JOBOBJECTINFOCLASS, lpjobobjectinformation : *mut core::ffi::c_void, cbjobobjectinformationlength : u32, lpreturnlength : *mut u32) -> windows_sys::core::BOOL);
windows_link::link!("kernel32.dll" "system" fn QueryIoRateControlInformationJobObject(hjob : super::super::Foundation::HANDLE, volumename : windows_sys::core::PCWSTR, infoblocks : *mut *mut JOBOBJECT_IO_RATE_CONTROL_INFORMATION, infoblockcount : *mut u32) -> u32);
windows_link::link!("kernel32.dll" "system" fn SetInformationJobObject(hjob : super::super::Foundation::HANDLE, jobobjectinformationclass : JOBOBJECTINFOCLASS, lpjobobjectinformation : *const core::ffi::c_void, cbjobobjectinformationlength : u32) -> windows_sys::core::BOOL);
windows_link::link!("kernel32.dll" "system" fn SetIoRateControlInformationJobObject(hjob : super::super::Foundation::HANDLE, ioratecontrolinfo : *const JOBOBJECT_IO_RATE_CONTROL_INFORMATION) -> u32);
windows_link::link!("kernel32.dll" "system" fn TerminateJobObject(hjob : super::super::Foundation::HANDLE, uexitcode : u32) -> windows_sys::core::BOOL);
windows_link::link!("user32.dll" "system" fn UserHandleGrantAccess(huserhandle : super::super::Foundation::HANDLE, hjob : super::super::Foundation::HANDLE, bgrant : windows_sys::core::BOOL) -> windows_sys::core::BOOL);
pub type JOBOBJECTINFOCLASS = i32;
#[repr(C)]
#[derive(Clone, Copy)]
pub struct JOBOBJECT_ASSOCIATE_COMPLETION_PORT {
    pub CompletionKey: *mut core::ffi::c_void,
    pub CompletionPort: super::super::Foundation::HANDLE,
}
impl Default for JOBOBJECT_ASSOCIATE_COMPLETION_PORT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
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
#[repr(C)]
#[cfg(feature = "Win32_System_Threading")]
#[derive(Clone, Copy, Default)]
pub struct JOBOBJECT_BASIC_AND_IO_ACCOUNTING_INFORMATION {
    pub BasicInfo: JOBOBJECT_BASIC_ACCOUNTING_INFORMATION,
    pub IoInfo: super::Threading::IO_COUNTERS,
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
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
#[repr(C)]
#[derive(Clone, Copy)]
pub struct JOBOBJECT_BASIC_PROCESS_ID_LIST {
    pub NumberOfAssignedProcesses: u32,
    pub NumberOfProcessIdsInList: u32,
    pub ProcessIdList: [usize; 1],
}
impl Default for JOBOBJECT_BASIC_PROCESS_ID_LIST {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct JOBOBJECT_BASIC_UI_RESTRICTIONS {
    pub UIRestrictionsClass: JOB_OBJECT_UILIMIT,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct JOBOBJECT_CPU_RATE_CONTROL_INFORMATION {
    pub ControlFlags: JOB_OBJECT_CPU_RATE_CONTROL,
    pub Anonymous: JOBOBJECT_CPU_RATE_CONTROL_INFORMATION_0,
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
impl Default for JOBOBJECT_CPU_RATE_CONTROL_INFORMATION_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct JOBOBJECT_CPU_RATE_CONTROL_INFORMATION_0_0 {
    pub MinRate: u16,
    pub MaxRate: u16,
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct JOBOBJECT_END_OF_JOB_TIME_INFORMATION {
    pub EndOfJobTimeAction: JOB_OBJECT_TERMINATE_AT_END_ACTION,
}
#[repr(C)]
#[cfg(feature = "Win32_System_Threading")]
#[derive(Clone, Copy, Default)]
pub struct JOBOBJECT_EXTENDED_LIMIT_INFORMATION {
    pub BasicLimitInformation: JOBOBJECT_BASIC_LIMIT_INFORMATION,
    pub IoInfo: super::Threading::IO_COUNTERS,
    pub ProcessMemoryLimit: usize,
    pub JobMemoryLimit: usize,
    pub PeakProcessMemoryUsed: usize,
    pub PeakJobMemoryUsed: usize,
}
pub const JOBOBJECT_IO_ATTRIBUTION_CONTROL_DISABLE: JOBOBJECT_IO_ATTRIBUTION_CONTROL_FLAGS = 2;
pub const JOBOBJECT_IO_ATTRIBUTION_CONTROL_ENABLE: JOBOBJECT_IO_ATTRIBUTION_CONTROL_FLAGS = 1;
pub type JOBOBJECT_IO_ATTRIBUTION_CONTROL_FLAGS = i32;
pub const JOBOBJECT_IO_ATTRIBUTION_CONTROL_VALID_FLAGS: JOBOBJECT_IO_ATTRIBUTION_CONTROL_FLAGS = 3;
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct JOBOBJECT_IO_ATTRIBUTION_INFORMATION {
    pub ControlFlags: u32,
    pub ReadStats: JOBOBJECT_IO_ATTRIBUTION_STATS,
    pub WriteStats: JOBOBJECT_IO_ATTRIBUTION_STATS,
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct JOBOBJECT_IO_ATTRIBUTION_STATS {
    pub IoCount: usize,
    pub TotalNonOverlappedQueueTime: u64,
    pub TotalNonOverlappedServiceTime: u64,
    pub TotalSize: u64,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct JOBOBJECT_IO_RATE_CONTROL_INFORMATION {
    pub MaxIops: i64,
    pub MaxBandwidth: i64,
    pub ReservationIops: i64,
    pub VolumeName: windows_sys::core::PCWSTR,
    pub BaseIoSize: u32,
    pub ControlFlags: u32,
}
impl Default for JOBOBJECT_IO_RATE_CONTROL_INFORMATION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct JOBOBJECT_IO_RATE_CONTROL_INFORMATION_NATIVE_V1 {
    pub MaxIops: i64,
    pub MaxBandwidth: i64,
    pub ReservationIops: i64,
    pub VolumeName: windows_sys::core::PWSTR,
    pub BaseIoSize: u32,
    pub ControlFlags: JOB_OBJECT_IO_RATE_CONTROL_FLAGS,
    pub VolumeNameLength: u16,
}
impl Default for JOBOBJECT_IO_RATE_CONTROL_INFORMATION_NATIVE_V1 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct JOBOBJECT_IO_RATE_CONTROL_INFORMATION_NATIVE_V2 {
    pub MaxIops: i64,
    pub MaxBandwidth: i64,
    pub ReservationIops: i64,
    pub VolumeName: windows_sys::core::PWSTR,
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
impl Default for JOBOBJECT_IO_RATE_CONTROL_INFORMATION_NATIVE_V2 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct JOBOBJECT_IO_RATE_CONTROL_INFORMATION_NATIVE_V3 {
    pub MaxIops: i64,
    pub MaxBandwidth: i64,
    pub ReservationIops: i64,
    pub VolumeName: windows_sys::core::PWSTR,
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
impl Default for JOBOBJECT_IO_RATE_CONTROL_INFORMATION_NATIVE_V3 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct JOBOBJECT_JOBSET_INFORMATION {
    pub MemberLevel: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
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
impl Default for JOBOBJECT_LIMIT_VIOLATION_INFORMATION_2_2 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct JOBOBJECT_NET_RATE_CONTROL_INFORMATION {
    pub MaxBandwidth: u64,
    pub ControlFlags: JOB_OBJECT_NET_RATE_CONTROL_FLAGS,
    pub DscpTag: u8,
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct JOBOBJECT_NOTIFICATION_LIMIT_INFORMATION {
    pub IoReadBytesLimit: u64,
    pub IoWriteBytesLimit: u64,
    pub PerJobUserTimeLimit: i64,
    pub JobMemoryLimit: u64,
    pub RateControlTolerance: JOBOBJECT_RATE_CONTROL_TOLERANCE,
    pub RateControlToleranceInterval: JOBOBJECT_RATE_CONTROL_TOLERANCE_INTERVAL,
    pub LimitFlags: JOB_OBJECT_LIMIT,
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
impl Default for JOBOBJECT_NOTIFICATION_LIMIT_INFORMATION_2_2 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub type JOBOBJECT_RATE_CONTROL_TOLERANCE = i32;
pub type JOBOBJECT_RATE_CONTROL_TOLERANCE_INTERVAL = i32;
#[repr(C)]
#[cfg(feature = "Win32_Security")]
#[derive(Clone, Copy)]
pub struct JOBOBJECT_SECURITY_LIMIT_INFORMATION {
    pub SecurityLimitFlags: JOB_OBJECT_SECURITY,
    pub JobToken: super::super::Foundation::HANDLE,
    pub SidsToDisable: *mut super::super::Security::TOKEN_GROUPS,
    pub PrivilegesToDelete: *mut super::super::Security::TOKEN_PRIVILEGES,
    pub RestrictedSids: *mut super::super::Security::TOKEN_GROUPS,
}
#[cfg(feature = "Win32_Security")]
impl Default for JOBOBJECT_SECURITY_LIMIT_INFORMATION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const JOB_OBJECT_BASIC_LIMIT_VALID_FLAGS: JOB_OBJECT_LIMIT = 255;
pub type JOB_OBJECT_CPU_RATE_CONTROL = u32;
pub const JOB_OBJECT_CPU_RATE_CONTROL_ENABLE: JOB_OBJECT_CPU_RATE_CONTROL = 1;
pub const JOB_OBJECT_CPU_RATE_CONTROL_HARD_CAP: JOB_OBJECT_CPU_RATE_CONTROL = 4;
pub const JOB_OBJECT_CPU_RATE_CONTROL_MIN_MAX_RATE: JOB_OBJECT_CPU_RATE_CONTROL = 16;
pub const JOB_OBJECT_CPU_RATE_CONTROL_NOTIFY: JOB_OBJECT_CPU_RATE_CONTROL = 8;
pub const JOB_OBJECT_CPU_RATE_CONTROL_VALID_FLAGS: JOB_OBJECT_CPU_RATE_CONTROL = 63;
pub const JOB_OBJECT_CPU_RATE_CONTROL_WEIGHT_BASED: JOB_OBJECT_CPU_RATE_CONTROL = 2;
pub const JOB_OBJECT_EXTENDED_LIMIT_VALID_FLAGS: JOB_OBJECT_LIMIT = 32767;
pub const JOB_OBJECT_IO_RATE_CONTROL_ENABLE: JOB_OBJECT_IO_RATE_CONTROL_FLAGS = 1;
pub type JOB_OBJECT_IO_RATE_CONTROL_FLAGS = i32;
pub const JOB_OBJECT_IO_RATE_CONTROL_FORCE_UNIT_ACCESS_ALL: JOB_OBJECT_IO_RATE_CONTROL_FLAGS = 4;
pub const JOB_OBJECT_IO_RATE_CONTROL_FORCE_UNIT_ACCESS_ON_SOFT_CAP: JOB_OBJECT_IO_RATE_CONTROL_FLAGS = 8;
pub const JOB_OBJECT_IO_RATE_CONTROL_STANDALONE_VOLUME: JOB_OBJECT_IO_RATE_CONTROL_FLAGS = 2;
pub const JOB_OBJECT_IO_RATE_CONTROL_VALID_FLAGS: JOB_OBJECT_IO_RATE_CONTROL_FLAGS = 15;
pub type JOB_OBJECT_LIMIT = u32;
pub const JOB_OBJECT_LIMIT_ACTIVE_PROCESS: JOB_OBJECT_LIMIT = 8;
pub const JOB_OBJECT_LIMIT_AFFINITY: JOB_OBJECT_LIMIT = 16;
pub const JOB_OBJECT_LIMIT_BREAKAWAY_OK: JOB_OBJECT_LIMIT = 2048;
pub const JOB_OBJECT_LIMIT_CPU_RATE_CONTROL: JOB_OBJECT_LIMIT = 262144;
pub const JOB_OBJECT_LIMIT_DIE_ON_UNHANDLED_EXCEPTION: JOB_OBJECT_LIMIT = 1024;
pub const JOB_OBJECT_LIMIT_IO_RATE_CONTROL: JOB_OBJECT_LIMIT = 524288;
pub const JOB_OBJECT_LIMIT_JOB_MEMORY: JOB_OBJECT_LIMIT = 512;
pub const JOB_OBJECT_LIMIT_JOB_MEMORY_HIGH: JOB_OBJECT_LIMIT = 512;
pub const JOB_OBJECT_LIMIT_JOB_MEMORY_LOW: JOB_OBJECT_LIMIT = 32768;
pub const JOB_OBJECT_LIMIT_JOB_READ_BYTES: JOB_OBJECT_LIMIT = 65536;
pub const JOB_OBJECT_LIMIT_JOB_TIME: JOB_OBJECT_LIMIT = 4;
pub const JOB_OBJECT_LIMIT_JOB_WRITE_BYTES: JOB_OBJECT_LIMIT = 131072;
pub const JOB_OBJECT_LIMIT_KILL_ON_JOB_CLOSE: JOB_OBJECT_LIMIT = 8192;
pub const JOB_OBJECT_LIMIT_NET_RATE_CONTROL: JOB_OBJECT_LIMIT = 1048576;
pub const JOB_OBJECT_LIMIT_PRESERVE_JOB_TIME: JOB_OBJECT_LIMIT = 64;
pub const JOB_OBJECT_LIMIT_PRIORITY_CLASS: JOB_OBJECT_LIMIT = 32;
pub const JOB_OBJECT_LIMIT_PROCESS_MEMORY: JOB_OBJECT_LIMIT = 256;
pub const JOB_OBJECT_LIMIT_PROCESS_TIME: JOB_OBJECT_LIMIT = 2;
pub const JOB_OBJECT_LIMIT_RATE_CONTROL: JOB_OBJECT_LIMIT = 262144;
pub const JOB_OBJECT_LIMIT_SCHEDULING_CLASS: JOB_OBJECT_LIMIT = 128;
pub const JOB_OBJECT_LIMIT_SILENT_BREAKAWAY_OK: JOB_OBJECT_LIMIT = 4096;
pub const JOB_OBJECT_LIMIT_SUBSET_AFFINITY: JOB_OBJECT_LIMIT = 16384;
pub const JOB_OBJECT_LIMIT_VALID_FLAGS: JOB_OBJECT_LIMIT = 524287;
pub const JOB_OBJECT_LIMIT_WORKINGSET: JOB_OBJECT_LIMIT = 1;
pub const JOB_OBJECT_NET_RATE_CONTROL_DSCP_TAG: JOB_OBJECT_NET_RATE_CONTROL_FLAGS = 4;
pub const JOB_OBJECT_NET_RATE_CONTROL_ENABLE: JOB_OBJECT_NET_RATE_CONTROL_FLAGS = 1;
pub type JOB_OBJECT_NET_RATE_CONTROL_FLAGS = i32;
pub const JOB_OBJECT_NET_RATE_CONTROL_MAX_BANDWIDTH: JOB_OBJECT_NET_RATE_CONTROL_FLAGS = 2;
pub const JOB_OBJECT_NET_RATE_CONTROL_VALID_FLAGS: JOB_OBJECT_NET_RATE_CONTROL_FLAGS = 7;
pub const JOB_OBJECT_NOTIFICATION_LIMIT_VALID_FLAGS: JOB_OBJECT_LIMIT = 2064900;
pub const JOB_OBJECT_POST_AT_END_OF_JOB: JOB_OBJECT_TERMINATE_AT_END_ACTION = 1;
pub type JOB_OBJECT_SECURITY = u32;
pub const JOB_OBJECT_SECURITY_FILTER_TOKENS: JOB_OBJECT_SECURITY = 8;
pub const JOB_OBJECT_SECURITY_NO_ADMIN: JOB_OBJECT_SECURITY = 1;
pub const JOB_OBJECT_SECURITY_ONLY_TOKEN: JOB_OBJECT_SECURITY = 4;
pub const JOB_OBJECT_SECURITY_RESTRICTED_TOKEN: JOB_OBJECT_SECURITY = 2;
pub const JOB_OBJECT_SECURITY_VALID_FLAGS: JOB_OBJECT_SECURITY = 15;
pub type JOB_OBJECT_TERMINATE_AT_END_ACTION = u32;
pub const JOB_OBJECT_TERMINATE_AT_END_OF_JOB: JOB_OBJECT_TERMINATE_AT_END_ACTION = 0;
pub type JOB_OBJECT_UILIMIT = u32;
pub const JOB_OBJECT_UILIMIT_DESKTOP: JOB_OBJECT_UILIMIT = 64;
pub const JOB_OBJECT_UILIMIT_DISPLAYSETTINGS: JOB_OBJECT_UILIMIT = 16;
pub const JOB_OBJECT_UILIMIT_EXITWINDOWS: JOB_OBJECT_UILIMIT = 128;
pub const JOB_OBJECT_UILIMIT_GLOBALATOMS: JOB_OBJECT_UILIMIT = 32;
pub const JOB_OBJECT_UILIMIT_HANDLES: JOB_OBJECT_UILIMIT = 1;
pub const JOB_OBJECT_UILIMIT_NONE: JOB_OBJECT_UILIMIT = 0;
pub const JOB_OBJECT_UILIMIT_READCLIPBOARD: JOB_OBJECT_UILIMIT = 2;
pub const JOB_OBJECT_UILIMIT_SYSTEMPARAMETERS: JOB_OBJECT_UILIMIT = 8;
pub const JOB_OBJECT_UILIMIT_WRITECLIPBOARD: JOB_OBJECT_UILIMIT = 4;
#[repr(C)]
#[derive(Clone, Copy)]
pub struct JOB_SET_ARRAY {
    pub JobHandle: super::super::Foundation::HANDLE,
    pub MemberLevel: u32,
    pub Flags: u32,
}
impl Default for JOB_SET_ARRAY {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const JobObjectAssociateCompletionPortInformation: JOBOBJECTINFOCLASS = 7;
pub const JobObjectBasicAccountingInformation: JOBOBJECTINFOCLASS = 1;
pub const JobObjectBasicAndIoAccountingInformation: JOBOBJECTINFOCLASS = 8;
pub const JobObjectBasicLimitInformation: JOBOBJECTINFOCLASS = 2;
pub const JobObjectBasicProcessIdList: JOBOBJECTINFOCLASS = 3;
pub const JobObjectBasicUIRestrictions: JOBOBJECTINFOCLASS = 4;
pub const JobObjectCompletionCounter: JOBOBJECTINFOCLASS = 17;
pub const JobObjectCompletionFilter: JOBOBJECTINFOCLASS = 16;
pub const JobObjectCpuRateControlInformation: JOBOBJECTINFOCLASS = 15;
pub const JobObjectCreateSilo: JOBOBJECTINFOCLASS = 35;
pub const JobObjectEndOfJobTimeInformation: JOBOBJECTINFOCLASS = 6;
pub const JobObjectExtendedLimitInformation: JOBOBJECTINFOCLASS = 9;
pub const JobObjectGroupInformation: JOBOBJECTINFOCLASS = 11;
pub const JobObjectGroupInformationEx: JOBOBJECTINFOCLASS = 14;
pub const JobObjectJobSetInformation: JOBOBJECTINFOCLASS = 10;
pub const JobObjectLimitViolationInformation: JOBOBJECTINFOCLASS = 13;
pub const JobObjectLimitViolationInformation2: JOBOBJECTINFOCLASS = 34;
pub const JobObjectNetRateControlInformation: JOBOBJECTINFOCLASS = 32;
pub const JobObjectNetworkAccountingInformation: JOBOBJECTINFOCLASS = 51;
pub const JobObjectNotificationLimitInformation: JOBOBJECTINFOCLASS = 12;
pub const JobObjectNotificationLimitInformation2: JOBOBJECTINFOCLASS = 33;
pub const JobObjectReserved10Information: JOBOBJECTINFOCLASS = 27;
pub const JobObjectReserved11Information: JOBOBJECTINFOCLASS = 28;
pub const JobObjectReserved12Information: JOBOBJECTINFOCLASS = 29;
pub const JobObjectReserved13Information: JOBOBJECTINFOCLASS = 30;
pub const JobObjectReserved14Information: JOBOBJECTINFOCLASS = 31;
pub const JobObjectReserved15Information: JOBOBJECTINFOCLASS = 37;
pub const JobObjectReserved16Information: JOBOBJECTINFOCLASS = 38;
pub const JobObjectReserved17Information: JOBOBJECTINFOCLASS = 39;
pub const JobObjectReserved18Information: JOBOBJECTINFOCLASS = 40;
pub const JobObjectReserved19Information: JOBOBJECTINFOCLASS = 41;
pub const JobObjectReserved1Information: JOBOBJECTINFOCLASS = 18;
pub const JobObjectReserved20Information: JOBOBJECTINFOCLASS = 42;
pub const JobObjectReserved21Information: JOBOBJECTINFOCLASS = 43;
pub const JobObjectReserved22Information: JOBOBJECTINFOCLASS = 44;
pub const JobObjectReserved23Information: JOBOBJECTINFOCLASS = 45;
pub const JobObjectReserved24Information: JOBOBJECTINFOCLASS = 46;
pub const JobObjectReserved25Information: JOBOBJECTINFOCLASS = 47;
pub const JobObjectReserved26Information: JOBOBJECTINFOCLASS = 48;
pub const JobObjectReserved27Information: JOBOBJECTINFOCLASS = 49;
pub const JobObjectReserved28Information: JOBOBJECTINFOCLASS = 50;
pub const JobObjectReserved2Information: JOBOBJECTINFOCLASS = 19;
pub const JobObjectReserved3Information: JOBOBJECTINFOCLASS = 20;
pub const JobObjectReserved4Information: JOBOBJECTINFOCLASS = 21;
pub const JobObjectReserved5Information: JOBOBJECTINFOCLASS = 22;
pub const JobObjectReserved6Information: JOBOBJECTINFOCLASS = 23;
pub const JobObjectReserved7Information: JOBOBJECTINFOCLASS = 24;
pub const JobObjectReserved8Information: JOBOBJECTINFOCLASS = 25;
pub const JobObjectReserved9Information: JOBOBJECTINFOCLASS = 26;
pub const JobObjectSecurityLimitInformation: JOBOBJECTINFOCLASS = 5;
pub const JobObjectSiloBasicInformation: JOBOBJECTINFOCLASS = 36;
pub const MaxJobObjectInfoClass: JOBOBJECTINFOCLASS = 52;
pub const ToleranceHigh: JOBOBJECT_RATE_CONTROL_TOLERANCE = 3;
pub const ToleranceIntervalLong: JOBOBJECT_RATE_CONTROL_TOLERANCE_INTERVAL = 3;
pub const ToleranceIntervalMedium: JOBOBJECT_RATE_CONTROL_TOLERANCE_INTERVAL = 2;
pub const ToleranceIntervalShort: JOBOBJECT_RATE_CONTROL_TOLERANCE_INTERVAL = 1;
pub const ToleranceLow: JOBOBJECT_RATE_CONTROL_TOLERANCE = 1;
pub const ToleranceMedium: JOBOBJECT_RATE_CONTROL_TOLERANCE = 2;

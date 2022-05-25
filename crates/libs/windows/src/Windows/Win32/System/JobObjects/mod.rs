#[doc = "*Required features: `\"Win32_System_JobObjects\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn AssignProcessToJobObject<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::HANDLE>, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::HANDLE>>(hjob: Param0, hprocess: Param1) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn AssignProcessToJobObject(hjob: super::super::Foundation::HANDLE, hprocess: super::super::Foundation::HANDLE) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(AssignProcessToJobObject(hjob.into_param().abi(), hprocess.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_JobObjects\"`, `\"Win32_Foundation\"`, `\"Win32_Security\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
#[inline]
pub unsafe fn CreateJobObjectA<'a, Param1: ::windows::core::IntoParam<'a, ::windows::core::PCSTR>>(lpjobattributes: *const super::super::Security::SECURITY_ATTRIBUTES, lpname: Param1) -> ::windows::core::Result<super::super::Foundation::HANDLE> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CreateJobObjectA(lpjobattributes: *const super::super::Security::SECURITY_ATTRIBUTES, lpname: ::windows::core::PCSTR) -> super::super::Foundation::HANDLE;
        }
        let result__ = CreateJobObjectA(::core::mem::transmute(lpjobattributes), lpname.into_param().abi());
        (!result__.is_invalid()).then(|| result__).ok_or_else(::windows::core::Error::from_win32)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_JobObjects\"`, `\"Win32_Foundation\"`, `\"Win32_Security\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
#[inline]
pub unsafe fn CreateJobObjectW<'a, Param1: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>>(lpjobattributes: *const super::super::Security::SECURITY_ATTRIBUTES, lpname: Param1) -> ::windows::core::Result<super::super::Foundation::HANDLE> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CreateJobObjectW(lpjobattributes: *const super::super::Security::SECURITY_ATTRIBUTES, lpname: ::windows::core::PCWSTR) -> super::super::Foundation::HANDLE;
        }
        let result__ = CreateJobObjectW(::core::mem::transmute(lpjobattributes), lpname.into_param().abi());
        (!result__.is_invalid()).then(|| result__).ok_or_else(::windows::core::Error::from_win32)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_JobObjects\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn CreateJobSet(userjobset: &[JOB_SET_ARRAY], flags: u32) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CreateJobSet(numjob: u32, userjobset: *const JOB_SET_ARRAY, flags: u32) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(CreateJobSet(userjobset.len() as _, ::core::mem::transmute(::windows::core::as_ptr_or_null(userjobset)), ::core::mem::transmute(flags)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_JobObjects\"`*"]
#[inline]
pub unsafe fn FreeMemoryJobObject(buffer: *const ::core::ffi::c_void) {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn FreeMemoryJobObject(buffer: *const ::core::ffi::c_void);
        }
        FreeMemoryJobObject(::core::mem::transmute(buffer))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_JobObjects\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn IsProcessInJob<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::HANDLE>, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::HANDLE>>(processhandle: Param0, jobhandle: Param1, result: *mut super::super::Foundation::BOOL) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn IsProcessInJob(processhandle: super::super::Foundation::HANDLE, jobhandle: super::super::Foundation::HANDLE, result: *mut super::super::Foundation::BOOL) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(IsProcessInJob(processhandle.into_param().abi(), jobhandle.into_param().abi(), ::core::mem::transmute(result)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_JobObjects\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct JOBOBJECTINFOCLASS(pub i32);
#[doc = "*Required features: `\"Win32_System_JobObjects\"`*"]
pub const JobObjectBasicAccountingInformation: JOBOBJECTINFOCLASS = JOBOBJECTINFOCLASS(1i32);
#[doc = "*Required features: `\"Win32_System_JobObjects\"`*"]
pub const JobObjectBasicLimitInformation: JOBOBJECTINFOCLASS = JOBOBJECTINFOCLASS(2i32);
#[doc = "*Required features: `\"Win32_System_JobObjects\"`*"]
pub const JobObjectBasicProcessIdList: JOBOBJECTINFOCLASS = JOBOBJECTINFOCLASS(3i32);
#[doc = "*Required features: `\"Win32_System_JobObjects\"`*"]
pub const JobObjectBasicUIRestrictions: JOBOBJECTINFOCLASS = JOBOBJECTINFOCLASS(4i32);
#[doc = "*Required features: `\"Win32_System_JobObjects\"`*"]
pub const JobObjectSecurityLimitInformation: JOBOBJECTINFOCLASS = JOBOBJECTINFOCLASS(5i32);
#[doc = "*Required features: `\"Win32_System_JobObjects\"`*"]
pub const JobObjectEndOfJobTimeInformation: JOBOBJECTINFOCLASS = JOBOBJECTINFOCLASS(6i32);
#[doc = "*Required features: `\"Win32_System_JobObjects\"`*"]
pub const JobObjectAssociateCompletionPortInformation: JOBOBJECTINFOCLASS = JOBOBJECTINFOCLASS(7i32);
#[doc = "*Required features: `\"Win32_System_JobObjects\"`*"]
pub const JobObjectBasicAndIoAccountingInformation: JOBOBJECTINFOCLASS = JOBOBJECTINFOCLASS(8i32);
#[doc = "*Required features: `\"Win32_System_JobObjects\"`*"]
pub const JobObjectExtendedLimitInformation: JOBOBJECTINFOCLASS = JOBOBJECTINFOCLASS(9i32);
#[doc = "*Required features: `\"Win32_System_JobObjects\"`*"]
pub const JobObjectJobSetInformation: JOBOBJECTINFOCLASS = JOBOBJECTINFOCLASS(10i32);
#[doc = "*Required features: `\"Win32_System_JobObjects\"`*"]
pub const JobObjectGroupInformation: JOBOBJECTINFOCLASS = JOBOBJECTINFOCLASS(11i32);
#[doc = "*Required features: `\"Win32_System_JobObjects\"`*"]
pub const JobObjectNotificationLimitInformation: JOBOBJECTINFOCLASS = JOBOBJECTINFOCLASS(12i32);
#[doc = "*Required features: `\"Win32_System_JobObjects\"`*"]
pub const JobObjectLimitViolationInformation: JOBOBJECTINFOCLASS = JOBOBJECTINFOCLASS(13i32);
#[doc = "*Required features: `\"Win32_System_JobObjects\"`*"]
pub const JobObjectGroupInformationEx: JOBOBJECTINFOCLASS = JOBOBJECTINFOCLASS(14i32);
#[doc = "*Required features: `\"Win32_System_JobObjects\"`*"]
pub const JobObjectCpuRateControlInformation: JOBOBJECTINFOCLASS = JOBOBJECTINFOCLASS(15i32);
#[doc = "*Required features: `\"Win32_System_JobObjects\"`*"]
pub const JobObjectCompletionFilter: JOBOBJECTINFOCLASS = JOBOBJECTINFOCLASS(16i32);
#[doc = "*Required features: `\"Win32_System_JobObjects\"`*"]
pub const JobObjectCompletionCounter: JOBOBJECTINFOCLASS = JOBOBJECTINFOCLASS(17i32);
#[doc = "*Required features: `\"Win32_System_JobObjects\"`*"]
pub const JobObjectReserved1Information: JOBOBJECTINFOCLASS = JOBOBJECTINFOCLASS(18i32);
#[doc = "*Required features: `\"Win32_System_JobObjects\"`*"]
pub const JobObjectReserved2Information: JOBOBJECTINFOCLASS = JOBOBJECTINFOCLASS(19i32);
#[doc = "*Required features: `\"Win32_System_JobObjects\"`*"]
pub const JobObjectReserved3Information: JOBOBJECTINFOCLASS = JOBOBJECTINFOCLASS(20i32);
#[doc = "*Required features: `\"Win32_System_JobObjects\"`*"]
pub const JobObjectReserved4Information: JOBOBJECTINFOCLASS = JOBOBJECTINFOCLASS(21i32);
#[doc = "*Required features: `\"Win32_System_JobObjects\"`*"]
pub const JobObjectReserved5Information: JOBOBJECTINFOCLASS = JOBOBJECTINFOCLASS(22i32);
#[doc = "*Required features: `\"Win32_System_JobObjects\"`*"]
pub const JobObjectReserved6Information: JOBOBJECTINFOCLASS = JOBOBJECTINFOCLASS(23i32);
#[doc = "*Required features: `\"Win32_System_JobObjects\"`*"]
pub const JobObjectReserved7Information: JOBOBJECTINFOCLASS = JOBOBJECTINFOCLASS(24i32);
#[doc = "*Required features: `\"Win32_System_JobObjects\"`*"]
pub const JobObjectReserved8Information: JOBOBJECTINFOCLASS = JOBOBJECTINFOCLASS(25i32);
#[doc = "*Required features: `\"Win32_System_JobObjects\"`*"]
pub const JobObjectReserved9Information: JOBOBJECTINFOCLASS = JOBOBJECTINFOCLASS(26i32);
#[doc = "*Required features: `\"Win32_System_JobObjects\"`*"]
pub const JobObjectReserved10Information: JOBOBJECTINFOCLASS = JOBOBJECTINFOCLASS(27i32);
#[doc = "*Required features: `\"Win32_System_JobObjects\"`*"]
pub const JobObjectReserved11Information: JOBOBJECTINFOCLASS = JOBOBJECTINFOCLASS(28i32);
#[doc = "*Required features: `\"Win32_System_JobObjects\"`*"]
pub const JobObjectReserved12Information: JOBOBJECTINFOCLASS = JOBOBJECTINFOCLASS(29i32);
#[doc = "*Required features: `\"Win32_System_JobObjects\"`*"]
pub const JobObjectReserved13Information: JOBOBJECTINFOCLASS = JOBOBJECTINFOCLASS(30i32);
#[doc = "*Required features: `\"Win32_System_JobObjects\"`*"]
pub const JobObjectReserved14Information: JOBOBJECTINFOCLASS = JOBOBJECTINFOCLASS(31i32);
#[doc = "*Required features: `\"Win32_System_JobObjects\"`*"]
pub const JobObjectNetRateControlInformation: JOBOBJECTINFOCLASS = JOBOBJECTINFOCLASS(32i32);
#[doc = "*Required features: `\"Win32_System_JobObjects\"`*"]
pub const JobObjectNotificationLimitInformation2: JOBOBJECTINFOCLASS = JOBOBJECTINFOCLASS(33i32);
#[doc = "*Required features: `\"Win32_System_JobObjects\"`*"]
pub const JobObjectLimitViolationInformation2: JOBOBJECTINFOCLASS = JOBOBJECTINFOCLASS(34i32);
#[doc = "*Required features: `\"Win32_System_JobObjects\"`*"]
pub const JobObjectCreateSilo: JOBOBJECTINFOCLASS = JOBOBJECTINFOCLASS(35i32);
#[doc = "*Required features: `\"Win32_System_JobObjects\"`*"]
pub const JobObjectSiloBasicInformation: JOBOBJECTINFOCLASS = JOBOBJECTINFOCLASS(36i32);
#[doc = "*Required features: `\"Win32_System_JobObjects\"`*"]
pub const JobObjectReserved15Information: JOBOBJECTINFOCLASS = JOBOBJECTINFOCLASS(37i32);
#[doc = "*Required features: `\"Win32_System_JobObjects\"`*"]
pub const JobObjectReserved16Information: JOBOBJECTINFOCLASS = JOBOBJECTINFOCLASS(38i32);
#[doc = "*Required features: `\"Win32_System_JobObjects\"`*"]
pub const JobObjectReserved17Information: JOBOBJECTINFOCLASS = JOBOBJECTINFOCLASS(39i32);
#[doc = "*Required features: `\"Win32_System_JobObjects\"`*"]
pub const JobObjectReserved18Information: JOBOBJECTINFOCLASS = JOBOBJECTINFOCLASS(40i32);
#[doc = "*Required features: `\"Win32_System_JobObjects\"`*"]
pub const JobObjectReserved19Information: JOBOBJECTINFOCLASS = JOBOBJECTINFOCLASS(41i32);
#[doc = "*Required features: `\"Win32_System_JobObjects\"`*"]
pub const JobObjectReserved20Information: JOBOBJECTINFOCLASS = JOBOBJECTINFOCLASS(42i32);
#[doc = "*Required features: `\"Win32_System_JobObjects\"`*"]
pub const JobObjectReserved21Information: JOBOBJECTINFOCLASS = JOBOBJECTINFOCLASS(43i32);
#[doc = "*Required features: `\"Win32_System_JobObjects\"`*"]
pub const JobObjectReserved22Information: JOBOBJECTINFOCLASS = JOBOBJECTINFOCLASS(44i32);
#[doc = "*Required features: `\"Win32_System_JobObjects\"`*"]
pub const JobObjectReserved23Information: JOBOBJECTINFOCLASS = JOBOBJECTINFOCLASS(45i32);
#[doc = "*Required features: `\"Win32_System_JobObjects\"`*"]
pub const JobObjectReserved24Information: JOBOBJECTINFOCLASS = JOBOBJECTINFOCLASS(46i32);
#[doc = "*Required features: `\"Win32_System_JobObjects\"`*"]
pub const JobObjectReserved25Information: JOBOBJECTINFOCLASS = JOBOBJECTINFOCLASS(47i32);
#[doc = "*Required features: `\"Win32_System_JobObjects\"`*"]
pub const MaxJobObjectInfoClass: JOBOBJECTINFOCLASS = JOBOBJECTINFOCLASS(48i32);
impl ::core::marker::Copy for JOBOBJECTINFOCLASS {}
impl ::core::clone::Clone for JOBOBJECTINFOCLASS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for JOBOBJECTINFOCLASS {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for JOBOBJECTINFOCLASS {
    type Abi = Self;
}
impl ::core::fmt::Debug for JOBOBJECTINFOCLASS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("JOBOBJECTINFOCLASS").field(&self.0).finish()
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_JobObjects\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct JOBOBJECT_ASSOCIATE_COMPLETION_PORT {
    pub CompletionKey: *mut ::core::ffi::c_void,
    pub CompletionPort: super::super::Foundation::HANDLE,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for JOBOBJECT_ASSOCIATE_COMPLETION_PORT {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for JOBOBJECT_ASSOCIATE_COMPLETION_PORT {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for JOBOBJECT_ASSOCIATE_COMPLETION_PORT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("JOBOBJECT_ASSOCIATE_COMPLETION_PORT").field("CompletionKey", &self.CompletionKey).field("CompletionPort", &self.CompletionPort).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for JOBOBJECT_ASSOCIATE_COMPLETION_PORT {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for JOBOBJECT_ASSOCIATE_COMPLETION_PORT {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<JOBOBJECT_ASSOCIATE_COMPLETION_PORT>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for JOBOBJECT_ASSOCIATE_COMPLETION_PORT {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for JOBOBJECT_ASSOCIATE_COMPLETION_PORT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_JobObjects\"`*"]
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
impl ::core::marker::Copy for JOBOBJECT_BASIC_ACCOUNTING_INFORMATION {}
impl ::core::clone::Clone for JOBOBJECT_BASIC_ACCOUNTING_INFORMATION {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for JOBOBJECT_BASIC_ACCOUNTING_INFORMATION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("JOBOBJECT_BASIC_ACCOUNTING_INFORMATION")
            .field("TotalUserTime", &self.TotalUserTime)
            .field("TotalKernelTime", &self.TotalKernelTime)
            .field("ThisPeriodTotalUserTime", &self.ThisPeriodTotalUserTime)
            .field("ThisPeriodTotalKernelTime", &self.ThisPeriodTotalKernelTime)
            .field("TotalPageFaultCount", &self.TotalPageFaultCount)
            .field("TotalProcesses", &self.TotalProcesses)
            .field("ActiveProcesses", &self.ActiveProcesses)
            .field("TotalTerminatedProcesses", &self.TotalTerminatedProcesses)
            .finish()
    }
}
unsafe impl ::windows::core::Abi for JOBOBJECT_BASIC_ACCOUNTING_INFORMATION {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for JOBOBJECT_BASIC_ACCOUNTING_INFORMATION {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<JOBOBJECT_BASIC_ACCOUNTING_INFORMATION>()) == 0 }
    }
}
impl ::core::cmp::Eq for JOBOBJECT_BASIC_ACCOUNTING_INFORMATION {}
impl ::core::default::Default for JOBOBJECT_BASIC_ACCOUNTING_INFORMATION {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_JobObjects\"`, `\"Win32_System_Threading\"`*"]
#[cfg(feature = "Win32_System_Threading")]
pub struct JOBOBJECT_BASIC_AND_IO_ACCOUNTING_INFORMATION {
    pub BasicInfo: JOBOBJECT_BASIC_ACCOUNTING_INFORMATION,
    pub IoInfo: super::Threading::IO_COUNTERS,
}
#[cfg(feature = "Win32_System_Threading")]
impl ::core::marker::Copy for JOBOBJECT_BASIC_AND_IO_ACCOUNTING_INFORMATION {}
#[cfg(feature = "Win32_System_Threading")]
impl ::core::clone::Clone for JOBOBJECT_BASIC_AND_IO_ACCOUNTING_INFORMATION {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_System_Threading")]
impl ::core::fmt::Debug for JOBOBJECT_BASIC_AND_IO_ACCOUNTING_INFORMATION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("JOBOBJECT_BASIC_AND_IO_ACCOUNTING_INFORMATION").field("BasicInfo", &self.BasicInfo).field("IoInfo", &self.IoInfo).finish()
    }
}
#[cfg(feature = "Win32_System_Threading")]
unsafe impl ::windows::core::Abi for JOBOBJECT_BASIC_AND_IO_ACCOUNTING_INFORMATION {
    type Abi = Self;
}
#[cfg(feature = "Win32_System_Threading")]
impl ::core::cmp::PartialEq for JOBOBJECT_BASIC_AND_IO_ACCOUNTING_INFORMATION {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<JOBOBJECT_BASIC_AND_IO_ACCOUNTING_INFORMATION>()) == 0 }
    }
}
#[cfg(feature = "Win32_System_Threading")]
impl ::core::cmp::Eq for JOBOBJECT_BASIC_AND_IO_ACCOUNTING_INFORMATION {}
#[cfg(feature = "Win32_System_Threading")]
impl ::core::default::Default for JOBOBJECT_BASIC_AND_IO_ACCOUNTING_INFORMATION {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_JobObjects\"`*"]
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
impl ::core::marker::Copy for JOBOBJECT_BASIC_LIMIT_INFORMATION {}
impl ::core::clone::Clone for JOBOBJECT_BASIC_LIMIT_INFORMATION {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for JOBOBJECT_BASIC_LIMIT_INFORMATION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("JOBOBJECT_BASIC_LIMIT_INFORMATION")
            .field("PerProcessUserTimeLimit", &self.PerProcessUserTimeLimit)
            .field("PerJobUserTimeLimit", &self.PerJobUserTimeLimit)
            .field("LimitFlags", &self.LimitFlags)
            .field("MinimumWorkingSetSize", &self.MinimumWorkingSetSize)
            .field("MaximumWorkingSetSize", &self.MaximumWorkingSetSize)
            .field("ActiveProcessLimit", &self.ActiveProcessLimit)
            .field("Affinity", &self.Affinity)
            .field("PriorityClass", &self.PriorityClass)
            .field("SchedulingClass", &self.SchedulingClass)
            .finish()
    }
}
unsafe impl ::windows::core::Abi for JOBOBJECT_BASIC_LIMIT_INFORMATION {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for JOBOBJECT_BASIC_LIMIT_INFORMATION {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<JOBOBJECT_BASIC_LIMIT_INFORMATION>()) == 0 }
    }
}
impl ::core::cmp::Eq for JOBOBJECT_BASIC_LIMIT_INFORMATION {}
impl ::core::default::Default for JOBOBJECT_BASIC_LIMIT_INFORMATION {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_JobObjects\"`*"]
pub struct JOBOBJECT_BASIC_PROCESS_ID_LIST {
    pub NumberOfAssignedProcesses: u32,
    pub NumberOfProcessIdsInList: u32,
    pub ProcessIdList: [usize; 1],
}
impl ::core::marker::Copy for JOBOBJECT_BASIC_PROCESS_ID_LIST {}
impl ::core::clone::Clone for JOBOBJECT_BASIC_PROCESS_ID_LIST {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for JOBOBJECT_BASIC_PROCESS_ID_LIST {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("JOBOBJECT_BASIC_PROCESS_ID_LIST").field("NumberOfAssignedProcesses", &self.NumberOfAssignedProcesses).field("NumberOfProcessIdsInList", &self.NumberOfProcessIdsInList).field("ProcessIdList", &self.ProcessIdList).finish()
    }
}
unsafe impl ::windows::core::Abi for JOBOBJECT_BASIC_PROCESS_ID_LIST {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for JOBOBJECT_BASIC_PROCESS_ID_LIST {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<JOBOBJECT_BASIC_PROCESS_ID_LIST>()) == 0 }
    }
}
impl ::core::cmp::Eq for JOBOBJECT_BASIC_PROCESS_ID_LIST {}
impl ::core::default::Default for JOBOBJECT_BASIC_PROCESS_ID_LIST {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_JobObjects\"`*"]
pub struct JOBOBJECT_BASIC_UI_RESTRICTIONS {
    pub UIRestrictionsClass: JOB_OBJECT_UILIMIT,
}
impl ::core::marker::Copy for JOBOBJECT_BASIC_UI_RESTRICTIONS {}
impl ::core::clone::Clone for JOBOBJECT_BASIC_UI_RESTRICTIONS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for JOBOBJECT_BASIC_UI_RESTRICTIONS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("JOBOBJECT_BASIC_UI_RESTRICTIONS").field("UIRestrictionsClass", &self.UIRestrictionsClass).finish()
    }
}
unsafe impl ::windows::core::Abi for JOBOBJECT_BASIC_UI_RESTRICTIONS {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for JOBOBJECT_BASIC_UI_RESTRICTIONS {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<JOBOBJECT_BASIC_UI_RESTRICTIONS>()) == 0 }
    }
}
impl ::core::cmp::Eq for JOBOBJECT_BASIC_UI_RESTRICTIONS {}
impl ::core::default::Default for JOBOBJECT_BASIC_UI_RESTRICTIONS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_JobObjects\"`*"]
pub struct JOBOBJECT_CPU_RATE_CONTROL_INFORMATION {
    pub ControlFlags: JOB_OBJECT_CPU_RATE_CONTROL,
    pub Anonymous: JOBOBJECT_CPU_RATE_CONTROL_INFORMATION_0,
}
impl ::core::marker::Copy for JOBOBJECT_CPU_RATE_CONTROL_INFORMATION {}
impl ::core::clone::Clone for JOBOBJECT_CPU_RATE_CONTROL_INFORMATION {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for JOBOBJECT_CPU_RATE_CONTROL_INFORMATION {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for JOBOBJECT_CPU_RATE_CONTROL_INFORMATION {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<JOBOBJECT_CPU_RATE_CONTROL_INFORMATION>()) == 0 }
    }
}
impl ::core::cmp::Eq for JOBOBJECT_CPU_RATE_CONTROL_INFORMATION {}
impl ::core::default::Default for JOBOBJECT_CPU_RATE_CONTROL_INFORMATION {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_JobObjects\"`*"]
pub union JOBOBJECT_CPU_RATE_CONTROL_INFORMATION_0 {
    pub CpuRate: u32,
    pub Weight: u32,
    pub Anonymous: JOBOBJECT_CPU_RATE_CONTROL_INFORMATION_0_0,
}
impl ::core::marker::Copy for JOBOBJECT_CPU_RATE_CONTROL_INFORMATION_0 {}
impl ::core::clone::Clone for JOBOBJECT_CPU_RATE_CONTROL_INFORMATION_0 {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for JOBOBJECT_CPU_RATE_CONTROL_INFORMATION_0 {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for JOBOBJECT_CPU_RATE_CONTROL_INFORMATION_0 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<JOBOBJECT_CPU_RATE_CONTROL_INFORMATION_0>()) == 0 }
    }
}
impl ::core::cmp::Eq for JOBOBJECT_CPU_RATE_CONTROL_INFORMATION_0 {}
impl ::core::default::Default for JOBOBJECT_CPU_RATE_CONTROL_INFORMATION_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_JobObjects\"`*"]
pub struct JOBOBJECT_CPU_RATE_CONTROL_INFORMATION_0_0 {
    pub MinRate: u16,
    pub MaxRate: u16,
}
impl ::core::marker::Copy for JOBOBJECT_CPU_RATE_CONTROL_INFORMATION_0_0 {}
impl ::core::clone::Clone for JOBOBJECT_CPU_RATE_CONTROL_INFORMATION_0_0 {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for JOBOBJECT_CPU_RATE_CONTROL_INFORMATION_0_0 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("JOBOBJECT_CPU_RATE_CONTROL_INFORMATION_0_0").field("MinRate", &self.MinRate).field("MaxRate", &self.MaxRate).finish()
    }
}
unsafe impl ::windows::core::Abi for JOBOBJECT_CPU_RATE_CONTROL_INFORMATION_0_0 {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for JOBOBJECT_CPU_RATE_CONTROL_INFORMATION_0_0 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<JOBOBJECT_CPU_RATE_CONTROL_INFORMATION_0_0>()) == 0 }
    }
}
impl ::core::cmp::Eq for JOBOBJECT_CPU_RATE_CONTROL_INFORMATION_0_0 {}
impl ::core::default::Default for JOBOBJECT_CPU_RATE_CONTROL_INFORMATION_0_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_JobObjects\"`*"]
pub struct JOBOBJECT_END_OF_JOB_TIME_INFORMATION {
    pub EndOfJobTimeAction: JOB_OBJECT_TERMINATE_AT_END_ACTION,
}
impl ::core::marker::Copy for JOBOBJECT_END_OF_JOB_TIME_INFORMATION {}
impl ::core::clone::Clone for JOBOBJECT_END_OF_JOB_TIME_INFORMATION {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for JOBOBJECT_END_OF_JOB_TIME_INFORMATION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("JOBOBJECT_END_OF_JOB_TIME_INFORMATION").field("EndOfJobTimeAction", &self.EndOfJobTimeAction).finish()
    }
}
unsafe impl ::windows::core::Abi for JOBOBJECT_END_OF_JOB_TIME_INFORMATION {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for JOBOBJECT_END_OF_JOB_TIME_INFORMATION {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<JOBOBJECT_END_OF_JOB_TIME_INFORMATION>()) == 0 }
    }
}
impl ::core::cmp::Eq for JOBOBJECT_END_OF_JOB_TIME_INFORMATION {}
impl ::core::default::Default for JOBOBJECT_END_OF_JOB_TIME_INFORMATION {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_JobObjects\"`, `\"Win32_System_Threading\"`*"]
#[cfg(feature = "Win32_System_Threading")]
pub struct JOBOBJECT_EXTENDED_LIMIT_INFORMATION {
    pub BasicLimitInformation: JOBOBJECT_BASIC_LIMIT_INFORMATION,
    pub IoInfo: super::Threading::IO_COUNTERS,
    pub ProcessMemoryLimit: usize,
    pub JobMemoryLimit: usize,
    pub PeakProcessMemoryUsed: usize,
    pub PeakJobMemoryUsed: usize,
}
#[cfg(feature = "Win32_System_Threading")]
impl ::core::marker::Copy for JOBOBJECT_EXTENDED_LIMIT_INFORMATION {}
#[cfg(feature = "Win32_System_Threading")]
impl ::core::clone::Clone for JOBOBJECT_EXTENDED_LIMIT_INFORMATION {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_System_Threading")]
impl ::core::fmt::Debug for JOBOBJECT_EXTENDED_LIMIT_INFORMATION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("JOBOBJECT_EXTENDED_LIMIT_INFORMATION").field("BasicLimitInformation", &self.BasicLimitInformation).field("IoInfo", &self.IoInfo).field("ProcessMemoryLimit", &self.ProcessMemoryLimit).field("JobMemoryLimit", &self.JobMemoryLimit).field("PeakProcessMemoryUsed", &self.PeakProcessMemoryUsed).field("PeakJobMemoryUsed", &self.PeakJobMemoryUsed).finish()
    }
}
#[cfg(feature = "Win32_System_Threading")]
unsafe impl ::windows::core::Abi for JOBOBJECT_EXTENDED_LIMIT_INFORMATION {
    type Abi = Self;
}
#[cfg(feature = "Win32_System_Threading")]
impl ::core::cmp::PartialEq for JOBOBJECT_EXTENDED_LIMIT_INFORMATION {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<JOBOBJECT_EXTENDED_LIMIT_INFORMATION>()) == 0 }
    }
}
#[cfg(feature = "Win32_System_Threading")]
impl ::core::cmp::Eq for JOBOBJECT_EXTENDED_LIMIT_INFORMATION {}
#[cfg(feature = "Win32_System_Threading")]
impl ::core::default::Default for JOBOBJECT_EXTENDED_LIMIT_INFORMATION {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_System_JobObjects\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct JOBOBJECT_IO_ATTRIBUTION_CONTROL_FLAGS(pub i32);
#[doc = "*Required features: `\"Win32_System_JobObjects\"`*"]
pub const JOBOBJECT_IO_ATTRIBUTION_CONTROL_ENABLE: JOBOBJECT_IO_ATTRIBUTION_CONTROL_FLAGS = JOBOBJECT_IO_ATTRIBUTION_CONTROL_FLAGS(1i32);
#[doc = "*Required features: `\"Win32_System_JobObjects\"`*"]
pub const JOBOBJECT_IO_ATTRIBUTION_CONTROL_DISABLE: JOBOBJECT_IO_ATTRIBUTION_CONTROL_FLAGS = JOBOBJECT_IO_ATTRIBUTION_CONTROL_FLAGS(2i32);
#[doc = "*Required features: `\"Win32_System_JobObjects\"`*"]
pub const JOBOBJECT_IO_ATTRIBUTION_CONTROL_VALID_FLAGS: JOBOBJECT_IO_ATTRIBUTION_CONTROL_FLAGS = JOBOBJECT_IO_ATTRIBUTION_CONTROL_FLAGS(3i32);
impl ::core::marker::Copy for JOBOBJECT_IO_ATTRIBUTION_CONTROL_FLAGS {}
impl ::core::clone::Clone for JOBOBJECT_IO_ATTRIBUTION_CONTROL_FLAGS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for JOBOBJECT_IO_ATTRIBUTION_CONTROL_FLAGS {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for JOBOBJECT_IO_ATTRIBUTION_CONTROL_FLAGS {
    type Abi = Self;
}
impl ::core::fmt::Debug for JOBOBJECT_IO_ATTRIBUTION_CONTROL_FLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("JOBOBJECT_IO_ATTRIBUTION_CONTROL_FLAGS").field(&self.0).finish()
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_JobObjects\"`*"]
pub struct JOBOBJECT_IO_ATTRIBUTION_INFORMATION {
    pub ControlFlags: u32,
    pub ReadStats: JOBOBJECT_IO_ATTRIBUTION_STATS,
    pub WriteStats: JOBOBJECT_IO_ATTRIBUTION_STATS,
}
impl ::core::marker::Copy for JOBOBJECT_IO_ATTRIBUTION_INFORMATION {}
impl ::core::clone::Clone for JOBOBJECT_IO_ATTRIBUTION_INFORMATION {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for JOBOBJECT_IO_ATTRIBUTION_INFORMATION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("JOBOBJECT_IO_ATTRIBUTION_INFORMATION").field("ControlFlags", &self.ControlFlags).field("ReadStats", &self.ReadStats).field("WriteStats", &self.WriteStats).finish()
    }
}
unsafe impl ::windows::core::Abi for JOBOBJECT_IO_ATTRIBUTION_INFORMATION {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for JOBOBJECT_IO_ATTRIBUTION_INFORMATION {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<JOBOBJECT_IO_ATTRIBUTION_INFORMATION>()) == 0 }
    }
}
impl ::core::cmp::Eq for JOBOBJECT_IO_ATTRIBUTION_INFORMATION {}
impl ::core::default::Default for JOBOBJECT_IO_ATTRIBUTION_INFORMATION {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_JobObjects\"`*"]
pub struct JOBOBJECT_IO_ATTRIBUTION_STATS {
    pub IoCount: usize,
    pub TotalNonOverlappedQueueTime: u64,
    pub TotalNonOverlappedServiceTime: u64,
    pub TotalSize: u64,
}
impl ::core::marker::Copy for JOBOBJECT_IO_ATTRIBUTION_STATS {}
impl ::core::clone::Clone for JOBOBJECT_IO_ATTRIBUTION_STATS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for JOBOBJECT_IO_ATTRIBUTION_STATS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("JOBOBJECT_IO_ATTRIBUTION_STATS").field("IoCount", &self.IoCount).field("TotalNonOverlappedQueueTime", &self.TotalNonOverlappedQueueTime).field("TotalNonOverlappedServiceTime", &self.TotalNonOverlappedServiceTime).field("TotalSize", &self.TotalSize).finish()
    }
}
unsafe impl ::windows::core::Abi for JOBOBJECT_IO_ATTRIBUTION_STATS {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for JOBOBJECT_IO_ATTRIBUTION_STATS {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<JOBOBJECT_IO_ATTRIBUTION_STATS>()) == 0 }
    }
}
impl ::core::cmp::Eq for JOBOBJECT_IO_ATTRIBUTION_STATS {}
impl ::core::default::Default for JOBOBJECT_IO_ATTRIBUTION_STATS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_JobObjects\"`*"]
pub struct JOBOBJECT_IO_RATE_CONTROL_INFORMATION {
    pub MaxIops: i64,
    pub MaxBandwidth: i64,
    pub ReservationIops: i64,
    pub VolumeName: ::windows::core::PCWSTR,
    pub BaseIoSize: u32,
    pub ControlFlags: JOB_OBJECT_IO_RATE_CONTROL_FLAGS,
}
impl ::core::marker::Copy for JOBOBJECT_IO_RATE_CONTROL_INFORMATION {}
impl ::core::clone::Clone for JOBOBJECT_IO_RATE_CONTROL_INFORMATION {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for JOBOBJECT_IO_RATE_CONTROL_INFORMATION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("JOBOBJECT_IO_RATE_CONTROL_INFORMATION").field("MaxIops", &self.MaxIops).field("MaxBandwidth", &self.MaxBandwidth).field("ReservationIops", &self.ReservationIops).field("VolumeName", &self.VolumeName).field("BaseIoSize", &self.BaseIoSize).field("ControlFlags", &self.ControlFlags).finish()
    }
}
unsafe impl ::windows::core::Abi for JOBOBJECT_IO_RATE_CONTROL_INFORMATION {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for JOBOBJECT_IO_RATE_CONTROL_INFORMATION {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<JOBOBJECT_IO_RATE_CONTROL_INFORMATION>()) == 0 }
    }
}
impl ::core::cmp::Eq for JOBOBJECT_IO_RATE_CONTROL_INFORMATION {}
impl ::core::default::Default for JOBOBJECT_IO_RATE_CONTROL_INFORMATION {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_JobObjects\"`*"]
pub struct JOBOBJECT_IO_RATE_CONTROL_INFORMATION_NATIVE {
    pub MaxIops: i64,
    pub MaxBandwidth: i64,
    pub ReservationIops: i64,
    pub VolumeName: ::windows::core::PWSTR,
    pub BaseIoSize: u32,
    pub ControlFlags: JOB_OBJECT_IO_RATE_CONTROL_FLAGS,
    pub VolumeNameLength: u16,
}
impl ::core::marker::Copy for JOBOBJECT_IO_RATE_CONTROL_INFORMATION_NATIVE {}
impl ::core::clone::Clone for JOBOBJECT_IO_RATE_CONTROL_INFORMATION_NATIVE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for JOBOBJECT_IO_RATE_CONTROL_INFORMATION_NATIVE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("JOBOBJECT_IO_RATE_CONTROL_INFORMATION_NATIVE").field("MaxIops", &self.MaxIops).field("MaxBandwidth", &self.MaxBandwidth).field("ReservationIops", &self.ReservationIops).field("VolumeName", &self.VolumeName).field("BaseIoSize", &self.BaseIoSize).field("ControlFlags", &self.ControlFlags).field("VolumeNameLength", &self.VolumeNameLength).finish()
    }
}
unsafe impl ::windows::core::Abi for JOBOBJECT_IO_RATE_CONTROL_INFORMATION_NATIVE {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for JOBOBJECT_IO_RATE_CONTROL_INFORMATION_NATIVE {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<JOBOBJECT_IO_RATE_CONTROL_INFORMATION_NATIVE>()) == 0 }
    }
}
impl ::core::cmp::Eq for JOBOBJECT_IO_RATE_CONTROL_INFORMATION_NATIVE {}
impl ::core::default::Default for JOBOBJECT_IO_RATE_CONTROL_INFORMATION_NATIVE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_JobObjects\"`*"]
pub struct JOBOBJECT_IO_RATE_CONTROL_INFORMATION_NATIVE_V2 {
    pub MaxIops: i64,
    pub MaxBandwidth: i64,
    pub ReservationIops: i64,
    pub VolumeName: ::windows::core::PWSTR,
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
impl ::core::marker::Copy for JOBOBJECT_IO_RATE_CONTROL_INFORMATION_NATIVE_V2 {}
impl ::core::clone::Clone for JOBOBJECT_IO_RATE_CONTROL_INFORMATION_NATIVE_V2 {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for JOBOBJECT_IO_RATE_CONTROL_INFORMATION_NATIVE_V2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("JOBOBJECT_IO_RATE_CONTROL_INFORMATION_NATIVE_V2")
            .field("MaxIops", &self.MaxIops)
            .field("MaxBandwidth", &self.MaxBandwidth)
            .field("ReservationIops", &self.ReservationIops)
            .field("VolumeName", &self.VolumeName)
            .field("BaseIoSize", &self.BaseIoSize)
            .field("ControlFlags", &self.ControlFlags)
            .field("VolumeNameLength", &self.VolumeNameLength)
            .field("CriticalReservationIops", &self.CriticalReservationIops)
            .field("ReservationBandwidth", &self.ReservationBandwidth)
            .field("CriticalReservationBandwidth", &self.CriticalReservationBandwidth)
            .field("MaxTimePercent", &self.MaxTimePercent)
            .field("ReservationTimePercent", &self.ReservationTimePercent)
            .field("CriticalReservationTimePercent", &self.CriticalReservationTimePercent)
            .finish()
    }
}
unsafe impl ::windows::core::Abi for JOBOBJECT_IO_RATE_CONTROL_INFORMATION_NATIVE_V2 {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for JOBOBJECT_IO_RATE_CONTROL_INFORMATION_NATIVE_V2 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<JOBOBJECT_IO_RATE_CONTROL_INFORMATION_NATIVE_V2>()) == 0 }
    }
}
impl ::core::cmp::Eq for JOBOBJECT_IO_RATE_CONTROL_INFORMATION_NATIVE_V2 {}
impl ::core::default::Default for JOBOBJECT_IO_RATE_CONTROL_INFORMATION_NATIVE_V2 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_JobObjects\"`*"]
pub struct JOBOBJECT_IO_RATE_CONTROL_INFORMATION_NATIVE_V3 {
    pub MaxIops: i64,
    pub MaxBandwidth: i64,
    pub ReservationIops: i64,
    pub VolumeName: ::windows::core::PWSTR,
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
impl ::core::marker::Copy for JOBOBJECT_IO_RATE_CONTROL_INFORMATION_NATIVE_V3 {}
impl ::core::clone::Clone for JOBOBJECT_IO_RATE_CONTROL_INFORMATION_NATIVE_V3 {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for JOBOBJECT_IO_RATE_CONTROL_INFORMATION_NATIVE_V3 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("JOBOBJECT_IO_RATE_CONTROL_INFORMATION_NATIVE_V3")
            .field("MaxIops", &self.MaxIops)
            .field("MaxBandwidth", &self.MaxBandwidth)
            .field("ReservationIops", &self.ReservationIops)
            .field("VolumeName", &self.VolumeName)
            .field("BaseIoSize", &self.BaseIoSize)
            .field("ControlFlags", &self.ControlFlags)
            .field("VolumeNameLength", &self.VolumeNameLength)
            .field("CriticalReservationIops", &self.CriticalReservationIops)
            .field("ReservationBandwidth", &self.ReservationBandwidth)
            .field("CriticalReservationBandwidth", &self.CriticalReservationBandwidth)
            .field("MaxTimePercent", &self.MaxTimePercent)
            .field("ReservationTimePercent", &self.ReservationTimePercent)
            .field("CriticalReservationTimePercent", &self.CriticalReservationTimePercent)
            .field("SoftMaxIops", &self.SoftMaxIops)
            .field("SoftMaxBandwidth", &self.SoftMaxBandwidth)
            .field("SoftMaxTimePercent", &self.SoftMaxTimePercent)
            .field("LimitExcessNotifyIops", &self.LimitExcessNotifyIops)
            .field("LimitExcessNotifyBandwidth", &self.LimitExcessNotifyBandwidth)
            .field("LimitExcessNotifyTimePercent", &self.LimitExcessNotifyTimePercent)
            .finish()
    }
}
unsafe impl ::windows::core::Abi for JOBOBJECT_IO_RATE_CONTROL_INFORMATION_NATIVE_V3 {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for JOBOBJECT_IO_RATE_CONTROL_INFORMATION_NATIVE_V3 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<JOBOBJECT_IO_RATE_CONTROL_INFORMATION_NATIVE_V3>()) == 0 }
    }
}
impl ::core::cmp::Eq for JOBOBJECT_IO_RATE_CONTROL_INFORMATION_NATIVE_V3 {}
impl ::core::default::Default for JOBOBJECT_IO_RATE_CONTROL_INFORMATION_NATIVE_V3 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_JobObjects\"`*"]
pub struct JOBOBJECT_JOBSET_INFORMATION {
    pub MemberLevel: u32,
}
impl ::core::marker::Copy for JOBOBJECT_JOBSET_INFORMATION {}
impl ::core::clone::Clone for JOBOBJECT_JOBSET_INFORMATION {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for JOBOBJECT_JOBSET_INFORMATION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("JOBOBJECT_JOBSET_INFORMATION").field("MemberLevel", &self.MemberLevel).finish()
    }
}
unsafe impl ::windows::core::Abi for JOBOBJECT_JOBSET_INFORMATION {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for JOBOBJECT_JOBSET_INFORMATION {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<JOBOBJECT_JOBSET_INFORMATION>()) == 0 }
    }
}
impl ::core::cmp::Eq for JOBOBJECT_JOBSET_INFORMATION {}
impl ::core::default::Default for JOBOBJECT_JOBSET_INFORMATION {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_JobObjects\"`*"]
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
impl ::core::marker::Copy for JOBOBJECT_LIMIT_VIOLATION_INFORMATION {}
impl ::core::clone::Clone for JOBOBJECT_LIMIT_VIOLATION_INFORMATION {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for JOBOBJECT_LIMIT_VIOLATION_INFORMATION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("JOBOBJECT_LIMIT_VIOLATION_INFORMATION")
            .field("LimitFlags", &self.LimitFlags)
            .field("ViolationLimitFlags", &self.ViolationLimitFlags)
            .field("IoReadBytes", &self.IoReadBytes)
            .field("IoReadBytesLimit", &self.IoReadBytesLimit)
            .field("IoWriteBytes", &self.IoWriteBytes)
            .field("IoWriteBytesLimit", &self.IoWriteBytesLimit)
            .field("PerJobUserTime", &self.PerJobUserTime)
            .field("PerJobUserTimeLimit", &self.PerJobUserTimeLimit)
            .field("JobMemory", &self.JobMemory)
            .field("JobMemoryLimit", &self.JobMemoryLimit)
            .field("RateControlTolerance", &self.RateControlTolerance)
            .field("RateControlToleranceLimit", &self.RateControlToleranceLimit)
            .finish()
    }
}
unsafe impl ::windows::core::Abi for JOBOBJECT_LIMIT_VIOLATION_INFORMATION {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for JOBOBJECT_LIMIT_VIOLATION_INFORMATION {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<JOBOBJECT_LIMIT_VIOLATION_INFORMATION>()) == 0 }
    }
}
impl ::core::cmp::Eq for JOBOBJECT_LIMIT_VIOLATION_INFORMATION {}
impl ::core::default::Default for JOBOBJECT_LIMIT_VIOLATION_INFORMATION {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_JobObjects\"`*"]
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
impl ::core::marker::Copy for JOBOBJECT_LIMIT_VIOLATION_INFORMATION_2 {}
impl ::core::clone::Clone for JOBOBJECT_LIMIT_VIOLATION_INFORMATION_2 {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for JOBOBJECT_LIMIT_VIOLATION_INFORMATION_2 {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for JOBOBJECT_LIMIT_VIOLATION_INFORMATION_2 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<JOBOBJECT_LIMIT_VIOLATION_INFORMATION_2>()) == 0 }
    }
}
impl ::core::cmp::Eq for JOBOBJECT_LIMIT_VIOLATION_INFORMATION_2 {}
impl ::core::default::Default for JOBOBJECT_LIMIT_VIOLATION_INFORMATION_2 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_JobObjects\"`*"]
pub union JOBOBJECT_LIMIT_VIOLATION_INFORMATION_2_0 {
    pub JobHighMemoryLimit: u64,
    pub JobMemoryLimit: u64,
}
impl ::core::marker::Copy for JOBOBJECT_LIMIT_VIOLATION_INFORMATION_2_0 {}
impl ::core::clone::Clone for JOBOBJECT_LIMIT_VIOLATION_INFORMATION_2_0 {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for JOBOBJECT_LIMIT_VIOLATION_INFORMATION_2_0 {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for JOBOBJECT_LIMIT_VIOLATION_INFORMATION_2_0 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<JOBOBJECT_LIMIT_VIOLATION_INFORMATION_2_0>()) == 0 }
    }
}
impl ::core::cmp::Eq for JOBOBJECT_LIMIT_VIOLATION_INFORMATION_2_0 {}
impl ::core::default::Default for JOBOBJECT_LIMIT_VIOLATION_INFORMATION_2_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_JobObjects\"`*"]
pub union JOBOBJECT_LIMIT_VIOLATION_INFORMATION_2_1 {
    pub RateControlTolerance: JOBOBJECT_RATE_CONTROL_TOLERANCE,
    pub CpuRateControlTolerance: JOBOBJECT_RATE_CONTROL_TOLERANCE,
}
impl ::core::marker::Copy for JOBOBJECT_LIMIT_VIOLATION_INFORMATION_2_1 {}
impl ::core::clone::Clone for JOBOBJECT_LIMIT_VIOLATION_INFORMATION_2_1 {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for JOBOBJECT_LIMIT_VIOLATION_INFORMATION_2_1 {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for JOBOBJECT_LIMIT_VIOLATION_INFORMATION_2_1 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<JOBOBJECT_LIMIT_VIOLATION_INFORMATION_2_1>()) == 0 }
    }
}
impl ::core::cmp::Eq for JOBOBJECT_LIMIT_VIOLATION_INFORMATION_2_1 {}
impl ::core::default::Default for JOBOBJECT_LIMIT_VIOLATION_INFORMATION_2_1 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_JobObjects\"`*"]
pub union JOBOBJECT_LIMIT_VIOLATION_INFORMATION_2_2 {
    pub RateControlToleranceLimit: JOBOBJECT_RATE_CONTROL_TOLERANCE,
    pub CpuRateControlToleranceLimit: JOBOBJECT_RATE_CONTROL_TOLERANCE,
}
impl ::core::marker::Copy for JOBOBJECT_LIMIT_VIOLATION_INFORMATION_2_2 {}
impl ::core::clone::Clone for JOBOBJECT_LIMIT_VIOLATION_INFORMATION_2_2 {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for JOBOBJECT_LIMIT_VIOLATION_INFORMATION_2_2 {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for JOBOBJECT_LIMIT_VIOLATION_INFORMATION_2_2 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<JOBOBJECT_LIMIT_VIOLATION_INFORMATION_2_2>()) == 0 }
    }
}
impl ::core::cmp::Eq for JOBOBJECT_LIMIT_VIOLATION_INFORMATION_2_2 {}
impl ::core::default::Default for JOBOBJECT_LIMIT_VIOLATION_INFORMATION_2_2 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_JobObjects\"`*"]
pub struct JOBOBJECT_NET_RATE_CONTROL_INFORMATION {
    pub MaxBandwidth: u64,
    pub ControlFlags: JOB_OBJECT_NET_RATE_CONTROL_FLAGS,
    pub DscpTag: u8,
}
impl ::core::marker::Copy for JOBOBJECT_NET_RATE_CONTROL_INFORMATION {}
impl ::core::clone::Clone for JOBOBJECT_NET_RATE_CONTROL_INFORMATION {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for JOBOBJECT_NET_RATE_CONTROL_INFORMATION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("JOBOBJECT_NET_RATE_CONTROL_INFORMATION").field("MaxBandwidth", &self.MaxBandwidth).field("ControlFlags", &self.ControlFlags).field("DscpTag", &self.DscpTag).finish()
    }
}
unsafe impl ::windows::core::Abi for JOBOBJECT_NET_RATE_CONTROL_INFORMATION {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for JOBOBJECT_NET_RATE_CONTROL_INFORMATION {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<JOBOBJECT_NET_RATE_CONTROL_INFORMATION>()) == 0 }
    }
}
impl ::core::cmp::Eq for JOBOBJECT_NET_RATE_CONTROL_INFORMATION {}
impl ::core::default::Default for JOBOBJECT_NET_RATE_CONTROL_INFORMATION {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_JobObjects\"`*"]
pub struct JOBOBJECT_NOTIFICATION_LIMIT_INFORMATION {
    pub IoReadBytesLimit: u64,
    pub IoWriteBytesLimit: u64,
    pub PerJobUserTimeLimit: i64,
    pub JobMemoryLimit: u64,
    pub RateControlTolerance: JOBOBJECT_RATE_CONTROL_TOLERANCE,
    pub RateControlToleranceInterval: JOBOBJECT_RATE_CONTROL_TOLERANCE_INTERVAL,
    pub LimitFlags: JOB_OBJECT_LIMIT,
}
impl ::core::marker::Copy for JOBOBJECT_NOTIFICATION_LIMIT_INFORMATION {}
impl ::core::clone::Clone for JOBOBJECT_NOTIFICATION_LIMIT_INFORMATION {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for JOBOBJECT_NOTIFICATION_LIMIT_INFORMATION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("JOBOBJECT_NOTIFICATION_LIMIT_INFORMATION").field("IoReadBytesLimit", &self.IoReadBytesLimit).field("IoWriteBytesLimit", &self.IoWriteBytesLimit).field("PerJobUserTimeLimit", &self.PerJobUserTimeLimit).field("JobMemoryLimit", &self.JobMemoryLimit).field("RateControlTolerance", &self.RateControlTolerance).field("RateControlToleranceInterval", &self.RateControlToleranceInterval).field("LimitFlags", &self.LimitFlags).finish()
    }
}
unsafe impl ::windows::core::Abi for JOBOBJECT_NOTIFICATION_LIMIT_INFORMATION {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for JOBOBJECT_NOTIFICATION_LIMIT_INFORMATION {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<JOBOBJECT_NOTIFICATION_LIMIT_INFORMATION>()) == 0 }
    }
}
impl ::core::cmp::Eq for JOBOBJECT_NOTIFICATION_LIMIT_INFORMATION {}
impl ::core::default::Default for JOBOBJECT_NOTIFICATION_LIMIT_INFORMATION {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_JobObjects\"`*"]
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
impl ::core::marker::Copy for JOBOBJECT_NOTIFICATION_LIMIT_INFORMATION_2 {}
impl ::core::clone::Clone for JOBOBJECT_NOTIFICATION_LIMIT_INFORMATION_2 {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for JOBOBJECT_NOTIFICATION_LIMIT_INFORMATION_2 {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for JOBOBJECT_NOTIFICATION_LIMIT_INFORMATION_2 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<JOBOBJECT_NOTIFICATION_LIMIT_INFORMATION_2>()) == 0 }
    }
}
impl ::core::cmp::Eq for JOBOBJECT_NOTIFICATION_LIMIT_INFORMATION_2 {}
impl ::core::default::Default for JOBOBJECT_NOTIFICATION_LIMIT_INFORMATION_2 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_JobObjects\"`*"]
pub union JOBOBJECT_NOTIFICATION_LIMIT_INFORMATION_2_0 {
    pub JobHighMemoryLimit: u64,
    pub JobMemoryLimit: u64,
}
impl ::core::marker::Copy for JOBOBJECT_NOTIFICATION_LIMIT_INFORMATION_2_0 {}
impl ::core::clone::Clone for JOBOBJECT_NOTIFICATION_LIMIT_INFORMATION_2_0 {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for JOBOBJECT_NOTIFICATION_LIMIT_INFORMATION_2_0 {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for JOBOBJECT_NOTIFICATION_LIMIT_INFORMATION_2_0 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<JOBOBJECT_NOTIFICATION_LIMIT_INFORMATION_2_0>()) == 0 }
    }
}
impl ::core::cmp::Eq for JOBOBJECT_NOTIFICATION_LIMIT_INFORMATION_2_0 {}
impl ::core::default::Default for JOBOBJECT_NOTIFICATION_LIMIT_INFORMATION_2_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_JobObjects\"`*"]
pub union JOBOBJECT_NOTIFICATION_LIMIT_INFORMATION_2_1 {
    pub RateControlTolerance: JOBOBJECT_RATE_CONTROL_TOLERANCE,
    pub CpuRateControlTolerance: JOBOBJECT_RATE_CONTROL_TOLERANCE,
}
impl ::core::marker::Copy for JOBOBJECT_NOTIFICATION_LIMIT_INFORMATION_2_1 {}
impl ::core::clone::Clone for JOBOBJECT_NOTIFICATION_LIMIT_INFORMATION_2_1 {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for JOBOBJECT_NOTIFICATION_LIMIT_INFORMATION_2_1 {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for JOBOBJECT_NOTIFICATION_LIMIT_INFORMATION_2_1 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<JOBOBJECT_NOTIFICATION_LIMIT_INFORMATION_2_1>()) == 0 }
    }
}
impl ::core::cmp::Eq for JOBOBJECT_NOTIFICATION_LIMIT_INFORMATION_2_1 {}
impl ::core::default::Default for JOBOBJECT_NOTIFICATION_LIMIT_INFORMATION_2_1 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_JobObjects\"`*"]
pub union JOBOBJECT_NOTIFICATION_LIMIT_INFORMATION_2_2 {
    pub RateControlToleranceInterval: JOBOBJECT_RATE_CONTROL_TOLERANCE_INTERVAL,
    pub CpuRateControlToleranceInterval: JOBOBJECT_RATE_CONTROL_TOLERANCE_INTERVAL,
}
impl ::core::marker::Copy for JOBOBJECT_NOTIFICATION_LIMIT_INFORMATION_2_2 {}
impl ::core::clone::Clone for JOBOBJECT_NOTIFICATION_LIMIT_INFORMATION_2_2 {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for JOBOBJECT_NOTIFICATION_LIMIT_INFORMATION_2_2 {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for JOBOBJECT_NOTIFICATION_LIMIT_INFORMATION_2_2 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<JOBOBJECT_NOTIFICATION_LIMIT_INFORMATION_2_2>()) == 0 }
    }
}
impl ::core::cmp::Eq for JOBOBJECT_NOTIFICATION_LIMIT_INFORMATION_2_2 {}
impl ::core::default::Default for JOBOBJECT_NOTIFICATION_LIMIT_INFORMATION_2_2 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_System_JobObjects\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct JOBOBJECT_RATE_CONTROL_TOLERANCE(pub i32);
#[doc = "*Required features: `\"Win32_System_JobObjects\"`*"]
pub const ToleranceLow: JOBOBJECT_RATE_CONTROL_TOLERANCE = JOBOBJECT_RATE_CONTROL_TOLERANCE(1i32);
#[doc = "*Required features: `\"Win32_System_JobObjects\"`*"]
pub const ToleranceMedium: JOBOBJECT_RATE_CONTROL_TOLERANCE = JOBOBJECT_RATE_CONTROL_TOLERANCE(2i32);
#[doc = "*Required features: `\"Win32_System_JobObjects\"`*"]
pub const ToleranceHigh: JOBOBJECT_RATE_CONTROL_TOLERANCE = JOBOBJECT_RATE_CONTROL_TOLERANCE(3i32);
impl ::core::marker::Copy for JOBOBJECT_RATE_CONTROL_TOLERANCE {}
impl ::core::clone::Clone for JOBOBJECT_RATE_CONTROL_TOLERANCE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for JOBOBJECT_RATE_CONTROL_TOLERANCE {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for JOBOBJECT_RATE_CONTROL_TOLERANCE {
    type Abi = Self;
}
impl ::core::fmt::Debug for JOBOBJECT_RATE_CONTROL_TOLERANCE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("JOBOBJECT_RATE_CONTROL_TOLERANCE").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_System_JobObjects\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct JOBOBJECT_RATE_CONTROL_TOLERANCE_INTERVAL(pub i32);
#[doc = "*Required features: `\"Win32_System_JobObjects\"`*"]
pub const ToleranceIntervalShort: JOBOBJECT_RATE_CONTROL_TOLERANCE_INTERVAL = JOBOBJECT_RATE_CONTROL_TOLERANCE_INTERVAL(1i32);
#[doc = "*Required features: `\"Win32_System_JobObjects\"`*"]
pub const ToleranceIntervalMedium: JOBOBJECT_RATE_CONTROL_TOLERANCE_INTERVAL = JOBOBJECT_RATE_CONTROL_TOLERANCE_INTERVAL(2i32);
#[doc = "*Required features: `\"Win32_System_JobObjects\"`*"]
pub const ToleranceIntervalLong: JOBOBJECT_RATE_CONTROL_TOLERANCE_INTERVAL = JOBOBJECT_RATE_CONTROL_TOLERANCE_INTERVAL(3i32);
impl ::core::marker::Copy for JOBOBJECT_RATE_CONTROL_TOLERANCE_INTERVAL {}
impl ::core::clone::Clone for JOBOBJECT_RATE_CONTROL_TOLERANCE_INTERVAL {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for JOBOBJECT_RATE_CONTROL_TOLERANCE_INTERVAL {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for JOBOBJECT_RATE_CONTROL_TOLERANCE_INTERVAL {
    type Abi = Self;
}
impl ::core::fmt::Debug for JOBOBJECT_RATE_CONTROL_TOLERANCE_INTERVAL {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("JOBOBJECT_RATE_CONTROL_TOLERANCE_INTERVAL").field(&self.0).finish()
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_JobObjects\"`, `\"Win32_Foundation\"`, `\"Win32_Security\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
pub struct JOBOBJECT_SECURITY_LIMIT_INFORMATION {
    pub SecurityLimitFlags: JOB_OBJECT_SECURITY,
    pub JobToken: super::super::Foundation::HANDLE,
    pub SidsToDisable: *mut super::super::Security::TOKEN_GROUPS,
    pub PrivilegesToDelete: *mut super::super::Security::TOKEN_PRIVILEGES,
    pub RestrictedSids: *mut super::super::Security::TOKEN_GROUPS,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
impl ::core::marker::Copy for JOBOBJECT_SECURITY_LIMIT_INFORMATION {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
impl ::core::clone::Clone for JOBOBJECT_SECURITY_LIMIT_INFORMATION {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
impl ::core::fmt::Debug for JOBOBJECT_SECURITY_LIMIT_INFORMATION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("JOBOBJECT_SECURITY_LIMIT_INFORMATION").field("SecurityLimitFlags", &self.SecurityLimitFlags).field("JobToken", &self.JobToken).field("SidsToDisable", &self.SidsToDisable).field("PrivilegesToDelete", &self.PrivilegesToDelete).field("RestrictedSids", &self.RestrictedSids).finish()
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
unsafe impl ::windows::core::Abi for JOBOBJECT_SECURITY_LIMIT_INFORMATION {
    type Abi = Self;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
impl ::core::cmp::PartialEq for JOBOBJECT_SECURITY_LIMIT_INFORMATION {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<JOBOBJECT_SECURITY_LIMIT_INFORMATION>()) == 0 }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
impl ::core::cmp::Eq for JOBOBJECT_SECURITY_LIMIT_INFORMATION {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
impl ::core::default::Default for JOBOBJECT_SECURITY_LIMIT_INFORMATION {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_System_JobObjects\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct JOB_OBJECT_CPU_RATE_CONTROL(pub u32);
#[doc = "*Required features: `\"Win32_System_JobObjects\"`*"]
pub const JOB_OBJECT_CPU_RATE_CONTROL_ENABLE: JOB_OBJECT_CPU_RATE_CONTROL = JOB_OBJECT_CPU_RATE_CONTROL(1u32);
#[doc = "*Required features: `\"Win32_System_JobObjects\"`*"]
pub const JOB_OBJECT_CPU_RATE_CONTROL_WEIGHT_BASED: JOB_OBJECT_CPU_RATE_CONTROL = JOB_OBJECT_CPU_RATE_CONTROL(2u32);
#[doc = "*Required features: `\"Win32_System_JobObjects\"`*"]
pub const JOB_OBJECT_CPU_RATE_CONTROL_HARD_CAP: JOB_OBJECT_CPU_RATE_CONTROL = JOB_OBJECT_CPU_RATE_CONTROL(4u32);
#[doc = "*Required features: `\"Win32_System_JobObjects\"`*"]
pub const JOB_OBJECT_CPU_RATE_CONTROL_NOTIFY: JOB_OBJECT_CPU_RATE_CONTROL = JOB_OBJECT_CPU_RATE_CONTROL(8u32);
#[doc = "*Required features: `\"Win32_System_JobObjects\"`*"]
pub const JOB_OBJECT_CPU_RATE_CONTROL_MIN_MAX_RATE: JOB_OBJECT_CPU_RATE_CONTROL = JOB_OBJECT_CPU_RATE_CONTROL(16u32);
#[doc = "*Required features: `\"Win32_System_JobObjects\"`*"]
pub const JOB_OBJECT_CPU_RATE_CONTROL_VALID_FLAGS: JOB_OBJECT_CPU_RATE_CONTROL = JOB_OBJECT_CPU_RATE_CONTROL(31u32);
impl ::core::marker::Copy for JOB_OBJECT_CPU_RATE_CONTROL {}
impl ::core::clone::Clone for JOB_OBJECT_CPU_RATE_CONTROL {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for JOB_OBJECT_CPU_RATE_CONTROL {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for JOB_OBJECT_CPU_RATE_CONTROL {
    type Abi = Self;
}
impl ::core::fmt::Debug for JOB_OBJECT_CPU_RATE_CONTROL {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("JOB_OBJECT_CPU_RATE_CONTROL").field(&self.0).finish()
    }
}
impl ::core::ops::BitOr for JOB_OBJECT_CPU_RATE_CONTROL {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for JOB_OBJECT_CPU_RATE_CONTROL {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for JOB_OBJECT_CPU_RATE_CONTROL {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for JOB_OBJECT_CPU_RATE_CONTROL {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for JOB_OBJECT_CPU_RATE_CONTROL {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[doc = "*Required features: `\"Win32_System_JobObjects\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct JOB_OBJECT_IO_RATE_CONTROL_FLAGS(pub i32);
#[doc = "*Required features: `\"Win32_System_JobObjects\"`*"]
pub const JOB_OBJECT_IO_RATE_CONTROL_ENABLE: JOB_OBJECT_IO_RATE_CONTROL_FLAGS = JOB_OBJECT_IO_RATE_CONTROL_FLAGS(1i32);
#[doc = "*Required features: `\"Win32_System_JobObjects\"`*"]
pub const JOB_OBJECT_IO_RATE_CONTROL_STANDALONE_VOLUME: JOB_OBJECT_IO_RATE_CONTROL_FLAGS = JOB_OBJECT_IO_RATE_CONTROL_FLAGS(2i32);
#[doc = "*Required features: `\"Win32_System_JobObjects\"`*"]
pub const JOB_OBJECT_IO_RATE_CONTROL_FORCE_UNIT_ACCESS_ALL: JOB_OBJECT_IO_RATE_CONTROL_FLAGS = JOB_OBJECT_IO_RATE_CONTROL_FLAGS(4i32);
#[doc = "*Required features: `\"Win32_System_JobObjects\"`*"]
pub const JOB_OBJECT_IO_RATE_CONTROL_FORCE_UNIT_ACCESS_ON_SOFT_CAP: JOB_OBJECT_IO_RATE_CONTROL_FLAGS = JOB_OBJECT_IO_RATE_CONTROL_FLAGS(8i32);
#[doc = "*Required features: `\"Win32_System_JobObjects\"`*"]
pub const JOB_OBJECT_IO_RATE_CONTROL_VALID_FLAGS: JOB_OBJECT_IO_RATE_CONTROL_FLAGS = JOB_OBJECT_IO_RATE_CONTROL_FLAGS(15i32);
impl ::core::marker::Copy for JOB_OBJECT_IO_RATE_CONTROL_FLAGS {}
impl ::core::clone::Clone for JOB_OBJECT_IO_RATE_CONTROL_FLAGS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for JOB_OBJECT_IO_RATE_CONTROL_FLAGS {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for JOB_OBJECT_IO_RATE_CONTROL_FLAGS {
    type Abi = Self;
}
impl ::core::fmt::Debug for JOB_OBJECT_IO_RATE_CONTROL_FLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("JOB_OBJECT_IO_RATE_CONTROL_FLAGS").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_System_JobObjects\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct JOB_OBJECT_LIMIT(pub u32);
#[doc = "*Required features: `\"Win32_System_JobObjects\"`*"]
pub const JOB_OBJECT_LIMIT_WORKINGSET: JOB_OBJECT_LIMIT = JOB_OBJECT_LIMIT(1u32);
#[doc = "*Required features: `\"Win32_System_JobObjects\"`*"]
pub const JOB_OBJECT_LIMIT_PROCESS_TIME: JOB_OBJECT_LIMIT = JOB_OBJECT_LIMIT(2u32);
#[doc = "*Required features: `\"Win32_System_JobObjects\"`*"]
pub const JOB_OBJECT_LIMIT_JOB_TIME: JOB_OBJECT_LIMIT = JOB_OBJECT_LIMIT(4u32);
#[doc = "*Required features: `\"Win32_System_JobObjects\"`*"]
pub const JOB_OBJECT_LIMIT_ACTIVE_PROCESS: JOB_OBJECT_LIMIT = JOB_OBJECT_LIMIT(8u32);
#[doc = "*Required features: `\"Win32_System_JobObjects\"`*"]
pub const JOB_OBJECT_LIMIT_AFFINITY: JOB_OBJECT_LIMIT = JOB_OBJECT_LIMIT(16u32);
#[doc = "*Required features: `\"Win32_System_JobObjects\"`*"]
pub const JOB_OBJECT_LIMIT_PRIORITY_CLASS: JOB_OBJECT_LIMIT = JOB_OBJECT_LIMIT(32u32);
#[doc = "*Required features: `\"Win32_System_JobObjects\"`*"]
pub const JOB_OBJECT_LIMIT_PRESERVE_JOB_TIME: JOB_OBJECT_LIMIT = JOB_OBJECT_LIMIT(64u32);
#[doc = "*Required features: `\"Win32_System_JobObjects\"`*"]
pub const JOB_OBJECT_LIMIT_SCHEDULING_CLASS: JOB_OBJECT_LIMIT = JOB_OBJECT_LIMIT(128u32);
#[doc = "*Required features: `\"Win32_System_JobObjects\"`*"]
pub const JOB_OBJECT_LIMIT_PROCESS_MEMORY: JOB_OBJECT_LIMIT = JOB_OBJECT_LIMIT(256u32);
#[doc = "*Required features: `\"Win32_System_JobObjects\"`*"]
pub const JOB_OBJECT_LIMIT_JOB_MEMORY: JOB_OBJECT_LIMIT = JOB_OBJECT_LIMIT(512u32);
#[doc = "*Required features: `\"Win32_System_JobObjects\"`*"]
pub const JOB_OBJECT_LIMIT_JOB_MEMORY_HIGH: JOB_OBJECT_LIMIT = JOB_OBJECT_LIMIT(512u32);
#[doc = "*Required features: `\"Win32_System_JobObjects\"`*"]
pub const JOB_OBJECT_LIMIT_DIE_ON_UNHANDLED_EXCEPTION: JOB_OBJECT_LIMIT = JOB_OBJECT_LIMIT(1024u32);
#[doc = "*Required features: `\"Win32_System_JobObjects\"`*"]
pub const JOB_OBJECT_LIMIT_BREAKAWAY_OK: JOB_OBJECT_LIMIT = JOB_OBJECT_LIMIT(2048u32);
#[doc = "*Required features: `\"Win32_System_JobObjects\"`*"]
pub const JOB_OBJECT_LIMIT_SILENT_BREAKAWAY_OK: JOB_OBJECT_LIMIT = JOB_OBJECT_LIMIT(4096u32);
#[doc = "*Required features: `\"Win32_System_JobObjects\"`*"]
pub const JOB_OBJECT_LIMIT_KILL_ON_JOB_CLOSE: JOB_OBJECT_LIMIT = JOB_OBJECT_LIMIT(8192u32);
#[doc = "*Required features: `\"Win32_System_JobObjects\"`*"]
pub const JOB_OBJECT_LIMIT_SUBSET_AFFINITY: JOB_OBJECT_LIMIT = JOB_OBJECT_LIMIT(16384u32);
#[doc = "*Required features: `\"Win32_System_JobObjects\"`*"]
pub const JOB_OBJECT_LIMIT_JOB_MEMORY_LOW: JOB_OBJECT_LIMIT = JOB_OBJECT_LIMIT(32768u32);
#[doc = "*Required features: `\"Win32_System_JobObjects\"`*"]
pub const JOB_OBJECT_LIMIT_JOB_READ_BYTES: JOB_OBJECT_LIMIT = JOB_OBJECT_LIMIT(65536u32);
#[doc = "*Required features: `\"Win32_System_JobObjects\"`*"]
pub const JOB_OBJECT_LIMIT_JOB_WRITE_BYTES: JOB_OBJECT_LIMIT = JOB_OBJECT_LIMIT(131072u32);
#[doc = "*Required features: `\"Win32_System_JobObjects\"`*"]
pub const JOB_OBJECT_LIMIT_RATE_CONTROL: JOB_OBJECT_LIMIT = JOB_OBJECT_LIMIT(262144u32);
#[doc = "*Required features: `\"Win32_System_JobObjects\"`*"]
pub const JOB_OBJECT_LIMIT_CPU_RATE_CONTROL: JOB_OBJECT_LIMIT = JOB_OBJECT_LIMIT(262144u32);
#[doc = "*Required features: `\"Win32_System_JobObjects\"`*"]
pub const JOB_OBJECT_LIMIT_IO_RATE_CONTROL: JOB_OBJECT_LIMIT = JOB_OBJECT_LIMIT(524288u32);
#[doc = "*Required features: `\"Win32_System_JobObjects\"`*"]
pub const JOB_OBJECT_LIMIT_NET_RATE_CONTROL: JOB_OBJECT_LIMIT = JOB_OBJECT_LIMIT(1048576u32);
#[doc = "*Required features: `\"Win32_System_JobObjects\"`*"]
pub const JOB_OBJECT_LIMIT_VALID_FLAGS: JOB_OBJECT_LIMIT = JOB_OBJECT_LIMIT(524287u32);
#[doc = "*Required features: `\"Win32_System_JobObjects\"`*"]
pub const JOB_OBJECT_BASIC_LIMIT_VALID_FLAGS: JOB_OBJECT_LIMIT = JOB_OBJECT_LIMIT(255u32);
#[doc = "*Required features: `\"Win32_System_JobObjects\"`*"]
pub const JOB_OBJECT_EXTENDED_LIMIT_VALID_FLAGS: JOB_OBJECT_LIMIT = JOB_OBJECT_LIMIT(32767u32);
#[doc = "*Required features: `\"Win32_System_JobObjects\"`*"]
pub const JOB_OBJECT_NOTIFICATION_LIMIT_VALID_FLAGS: JOB_OBJECT_LIMIT = JOB_OBJECT_LIMIT(2064900u32);
impl ::core::marker::Copy for JOB_OBJECT_LIMIT {}
impl ::core::clone::Clone for JOB_OBJECT_LIMIT {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for JOB_OBJECT_LIMIT {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for JOB_OBJECT_LIMIT {
    type Abi = Self;
}
impl ::core::fmt::Debug for JOB_OBJECT_LIMIT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("JOB_OBJECT_LIMIT").field(&self.0).finish()
    }
}
impl ::core::ops::BitOr for JOB_OBJECT_LIMIT {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for JOB_OBJECT_LIMIT {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for JOB_OBJECT_LIMIT {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for JOB_OBJECT_LIMIT {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for JOB_OBJECT_LIMIT {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[doc = "*Required features: `\"Win32_System_JobObjects\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct JOB_OBJECT_NET_RATE_CONTROL_FLAGS(pub u32);
#[doc = "*Required features: `\"Win32_System_JobObjects\"`*"]
pub const JOB_OBJECT_NET_RATE_CONTROL_ENABLE: JOB_OBJECT_NET_RATE_CONTROL_FLAGS = JOB_OBJECT_NET_RATE_CONTROL_FLAGS(1u32);
#[doc = "*Required features: `\"Win32_System_JobObjects\"`*"]
pub const JOB_OBJECT_NET_RATE_CONTROL_MAX_BANDWIDTH: JOB_OBJECT_NET_RATE_CONTROL_FLAGS = JOB_OBJECT_NET_RATE_CONTROL_FLAGS(2u32);
#[doc = "*Required features: `\"Win32_System_JobObjects\"`*"]
pub const JOB_OBJECT_NET_RATE_CONTROL_DSCP_TAG: JOB_OBJECT_NET_RATE_CONTROL_FLAGS = JOB_OBJECT_NET_RATE_CONTROL_FLAGS(4u32);
#[doc = "*Required features: `\"Win32_System_JobObjects\"`*"]
pub const JOB_OBJECT_NET_RATE_CONTROL_VALID_FLAGS: JOB_OBJECT_NET_RATE_CONTROL_FLAGS = JOB_OBJECT_NET_RATE_CONTROL_FLAGS(7u32);
impl ::core::marker::Copy for JOB_OBJECT_NET_RATE_CONTROL_FLAGS {}
impl ::core::clone::Clone for JOB_OBJECT_NET_RATE_CONTROL_FLAGS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for JOB_OBJECT_NET_RATE_CONTROL_FLAGS {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for JOB_OBJECT_NET_RATE_CONTROL_FLAGS {
    type Abi = Self;
}
impl ::core::fmt::Debug for JOB_OBJECT_NET_RATE_CONTROL_FLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("JOB_OBJECT_NET_RATE_CONTROL_FLAGS").field(&self.0).finish()
    }
}
impl ::core::ops::BitOr for JOB_OBJECT_NET_RATE_CONTROL_FLAGS {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for JOB_OBJECT_NET_RATE_CONTROL_FLAGS {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for JOB_OBJECT_NET_RATE_CONTROL_FLAGS {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for JOB_OBJECT_NET_RATE_CONTROL_FLAGS {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for JOB_OBJECT_NET_RATE_CONTROL_FLAGS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[doc = "*Required features: `\"Win32_System_JobObjects\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct JOB_OBJECT_SECURITY(pub u32);
#[doc = "*Required features: `\"Win32_System_JobObjects\"`*"]
pub const JOB_OBJECT_SECURITY_NO_ADMIN: JOB_OBJECT_SECURITY = JOB_OBJECT_SECURITY(1u32);
#[doc = "*Required features: `\"Win32_System_JobObjects\"`*"]
pub const JOB_OBJECT_SECURITY_RESTRICTED_TOKEN: JOB_OBJECT_SECURITY = JOB_OBJECT_SECURITY(2u32);
#[doc = "*Required features: `\"Win32_System_JobObjects\"`*"]
pub const JOB_OBJECT_SECURITY_ONLY_TOKEN: JOB_OBJECT_SECURITY = JOB_OBJECT_SECURITY(4u32);
#[doc = "*Required features: `\"Win32_System_JobObjects\"`*"]
pub const JOB_OBJECT_SECURITY_FILTER_TOKENS: JOB_OBJECT_SECURITY = JOB_OBJECT_SECURITY(8u32);
#[doc = "*Required features: `\"Win32_System_JobObjects\"`*"]
pub const JOB_OBJECT_SECURITY_VALID_FLAGS: JOB_OBJECT_SECURITY = JOB_OBJECT_SECURITY(15u32);
impl ::core::marker::Copy for JOB_OBJECT_SECURITY {}
impl ::core::clone::Clone for JOB_OBJECT_SECURITY {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for JOB_OBJECT_SECURITY {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for JOB_OBJECT_SECURITY {
    type Abi = Self;
}
impl ::core::fmt::Debug for JOB_OBJECT_SECURITY {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("JOB_OBJECT_SECURITY").field(&self.0).finish()
    }
}
impl ::core::ops::BitOr for JOB_OBJECT_SECURITY {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for JOB_OBJECT_SECURITY {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for JOB_OBJECT_SECURITY {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for JOB_OBJECT_SECURITY {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for JOB_OBJECT_SECURITY {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[doc = "*Required features: `\"Win32_System_JobObjects\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct JOB_OBJECT_TERMINATE_AT_END_ACTION(pub u32);
#[doc = "*Required features: `\"Win32_System_JobObjects\"`*"]
pub const JOB_OBJECT_TERMINATE_AT_END_OF_JOB: JOB_OBJECT_TERMINATE_AT_END_ACTION = JOB_OBJECT_TERMINATE_AT_END_ACTION(0u32);
#[doc = "*Required features: `\"Win32_System_JobObjects\"`*"]
pub const JOB_OBJECT_POST_AT_END_OF_JOB: JOB_OBJECT_TERMINATE_AT_END_ACTION = JOB_OBJECT_TERMINATE_AT_END_ACTION(1u32);
impl ::core::marker::Copy for JOB_OBJECT_TERMINATE_AT_END_ACTION {}
impl ::core::clone::Clone for JOB_OBJECT_TERMINATE_AT_END_ACTION {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for JOB_OBJECT_TERMINATE_AT_END_ACTION {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for JOB_OBJECT_TERMINATE_AT_END_ACTION {
    type Abi = Self;
}
impl ::core::fmt::Debug for JOB_OBJECT_TERMINATE_AT_END_ACTION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("JOB_OBJECT_TERMINATE_AT_END_ACTION").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_System_JobObjects\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct JOB_OBJECT_UILIMIT(pub u32);
#[doc = "*Required features: `\"Win32_System_JobObjects\"`*"]
pub const JOB_OBJECT_UILIMIT_NONE: JOB_OBJECT_UILIMIT = JOB_OBJECT_UILIMIT(0u32);
#[doc = "*Required features: `\"Win32_System_JobObjects\"`*"]
pub const JOB_OBJECT_UILIMIT_HANDLES: JOB_OBJECT_UILIMIT = JOB_OBJECT_UILIMIT(1u32);
#[doc = "*Required features: `\"Win32_System_JobObjects\"`*"]
pub const JOB_OBJECT_UILIMIT_READCLIPBOARD: JOB_OBJECT_UILIMIT = JOB_OBJECT_UILIMIT(2u32);
#[doc = "*Required features: `\"Win32_System_JobObjects\"`*"]
pub const JOB_OBJECT_UILIMIT_WRITECLIPBOARD: JOB_OBJECT_UILIMIT = JOB_OBJECT_UILIMIT(4u32);
#[doc = "*Required features: `\"Win32_System_JobObjects\"`*"]
pub const JOB_OBJECT_UILIMIT_SYSTEMPARAMETERS: JOB_OBJECT_UILIMIT = JOB_OBJECT_UILIMIT(8u32);
#[doc = "*Required features: `\"Win32_System_JobObjects\"`*"]
pub const JOB_OBJECT_UILIMIT_DISPLAYSETTINGS: JOB_OBJECT_UILIMIT = JOB_OBJECT_UILIMIT(16u32);
#[doc = "*Required features: `\"Win32_System_JobObjects\"`*"]
pub const JOB_OBJECT_UILIMIT_GLOBALATOMS: JOB_OBJECT_UILIMIT = JOB_OBJECT_UILIMIT(32u32);
#[doc = "*Required features: `\"Win32_System_JobObjects\"`*"]
pub const JOB_OBJECT_UILIMIT_DESKTOP: JOB_OBJECT_UILIMIT = JOB_OBJECT_UILIMIT(64u32);
#[doc = "*Required features: `\"Win32_System_JobObjects\"`*"]
pub const JOB_OBJECT_UILIMIT_EXITWINDOWS: JOB_OBJECT_UILIMIT = JOB_OBJECT_UILIMIT(128u32);
impl ::core::marker::Copy for JOB_OBJECT_UILIMIT {}
impl ::core::clone::Clone for JOB_OBJECT_UILIMIT {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for JOB_OBJECT_UILIMIT {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for JOB_OBJECT_UILIMIT {
    type Abi = Self;
}
impl ::core::fmt::Debug for JOB_OBJECT_UILIMIT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("JOB_OBJECT_UILIMIT").field(&self.0).finish()
    }
}
impl ::core::ops::BitOr for JOB_OBJECT_UILIMIT {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for JOB_OBJECT_UILIMIT {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for JOB_OBJECT_UILIMIT {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for JOB_OBJECT_UILIMIT {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for JOB_OBJECT_UILIMIT {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_JobObjects\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct JOB_SET_ARRAY {
    pub JobHandle: super::super::Foundation::HANDLE,
    pub MemberLevel: u32,
    pub Flags: u32,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for JOB_SET_ARRAY {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for JOB_SET_ARRAY {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for JOB_SET_ARRAY {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("JOB_SET_ARRAY").field("JobHandle", &self.JobHandle).field("MemberLevel", &self.MemberLevel).field("Flags", &self.Flags).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for JOB_SET_ARRAY {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for JOB_SET_ARRAY {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<JOB_SET_ARRAY>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for JOB_SET_ARRAY {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for JOB_SET_ARRAY {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_System_JobObjects\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn OpenJobObjectA<'a, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::BOOL>, Param2: ::windows::core::IntoParam<'a, ::windows::core::PCSTR>>(dwdesiredaccess: u32, binherithandle: Param1, lpname: Param2) -> ::windows::core::Result<super::super::Foundation::HANDLE> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn OpenJobObjectA(dwdesiredaccess: u32, binherithandle: super::super::Foundation::BOOL, lpname: ::windows::core::PCSTR) -> super::super::Foundation::HANDLE;
        }
        let result__ = OpenJobObjectA(::core::mem::transmute(dwdesiredaccess), binherithandle.into_param().abi(), lpname.into_param().abi());
        (!result__.is_invalid()).then(|| result__).ok_or_else(::windows::core::Error::from_win32)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_JobObjects\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn OpenJobObjectW<'a, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::BOOL>, Param2: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>>(dwdesiredaccess: u32, binherithandle: Param1, lpname: Param2) -> ::windows::core::Result<super::super::Foundation::HANDLE> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn OpenJobObjectW(dwdesiredaccess: u32, binherithandle: super::super::Foundation::BOOL, lpname: ::windows::core::PCWSTR) -> super::super::Foundation::HANDLE;
        }
        let result__ = OpenJobObjectW(::core::mem::transmute(dwdesiredaccess), binherithandle.into_param().abi(), lpname.into_param().abi());
        (!result__.is_invalid()).then(|| result__).ok_or_else(::windows::core::Error::from_win32)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_JobObjects\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn QueryInformationJobObject<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::HANDLE>>(hjob: Param0, jobobjectinformationclass: JOBOBJECTINFOCLASS, lpjobobjectinformation: *mut ::core::ffi::c_void, cbjobobjectinformationlength: u32, lpreturnlength: *mut u32) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn QueryInformationJobObject(hjob: super::super::Foundation::HANDLE, jobobjectinformationclass: JOBOBJECTINFOCLASS, lpjobobjectinformation: *mut ::core::ffi::c_void, cbjobobjectinformationlength: u32, lpreturnlength: *mut u32) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(QueryInformationJobObject(hjob.into_param().abi(), ::core::mem::transmute(jobobjectinformationclass), ::core::mem::transmute(lpjobobjectinformation), ::core::mem::transmute(cbjobobjectinformationlength), ::core::mem::transmute(lpreturnlength)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_JobObjects\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn QueryIoRateControlInformationJobObject<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::HANDLE>, Param1: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>>(hjob: Param0, volumename: Param1, infoblocks: *mut *mut JOBOBJECT_IO_RATE_CONTROL_INFORMATION, infoblockcount: *mut u32) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn QueryIoRateControlInformationJobObject(hjob: super::super::Foundation::HANDLE, volumename: ::windows::core::PCWSTR, infoblocks: *mut *mut JOBOBJECT_IO_RATE_CONTROL_INFORMATION, infoblockcount: *mut u32) -> u32;
        }
        ::core::mem::transmute(QueryIoRateControlInformationJobObject(hjob.into_param().abi(), volumename.into_param().abi(), ::core::mem::transmute(infoblocks), ::core::mem::transmute(infoblockcount)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_JobObjects\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn SetInformationJobObject<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::HANDLE>>(hjob: Param0, jobobjectinformationclass: JOBOBJECTINFOCLASS, lpjobobjectinformation: *const ::core::ffi::c_void, cbjobobjectinformationlength: u32) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn SetInformationJobObject(hjob: super::super::Foundation::HANDLE, jobobjectinformationclass: JOBOBJECTINFOCLASS, lpjobobjectinformation: *const ::core::ffi::c_void, cbjobobjectinformationlength: u32) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(SetInformationJobObject(hjob.into_param().abi(), ::core::mem::transmute(jobobjectinformationclass), ::core::mem::transmute(lpjobobjectinformation), ::core::mem::transmute(cbjobobjectinformationlength)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_JobObjects\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn SetIoRateControlInformationJobObject<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::HANDLE>>(hjob: Param0, ioratecontrolinfo: *const JOBOBJECT_IO_RATE_CONTROL_INFORMATION) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn SetIoRateControlInformationJobObject(hjob: super::super::Foundation::HANDLE, ioratecontrolinfo: *const JOBOBJECT_IO_RATE_CONTROL_INFORMATION) -> u32;
        }
        ::core::mem::transmute(SetIoRateControlInformationJobObject(hjob.into_param().abi(), ::core::mem::transmute(ioratecontrolinfo)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_JobObjects\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn TerminateJobObject<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::HANDLE>>(hjob: Param0, uexitcode: u32) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn TerminateJobObject(hjob: super::super::Foundation::HANDLE, uexitcode: u32) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(TerminateJobObject(hjob.into_param().abi(), ::core::mem::transmute(uexitcode)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_JobObjects\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn UserHandleGrantAccess<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::HANDLE>, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::HANDLE>, Param2: ::windows::core::IntoParam<'a, super::super::Foundation::BOOL>>(huserhandle: Param0, hjob: Param1, bgrant: Param2) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn UserHandleGrantAccess(huserhandle: super::super::Foundation::HANDLE, hjob: super::super::Foundation::HANDLE, bgrant: super::super::Foundation::BOOL) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(UserHandleGrantAccess(huserhandle.into_param().abi(), hjob.into_param().abi(), bgrant.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "implement")]
::core::include!("impl.rs");

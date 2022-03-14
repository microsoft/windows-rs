#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, clashing_extern_declarations, clippy::all)]
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
pub struct APP_MEMORY_INFORMATION {
    pub AvailableCommit: u64,
    pub PrivateCommitUsage: u64,
    pub PeakPrivateCommitUsage: u64,
    pub TotalCommitUsage: u64,
}
impl ::core::marker::Copy for APP_MEMORY_INFORMATION {}
impl ::core::clone::Clone for APP_MEMORY_INFORMATION {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for APP_MEMORY_INFORMATION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("APP_MEMORY_INFORMATION").field("AvailableCommit", &self.AvailableCommit).field("PrivateCommitUsage", &self.PrivateCommitUsage).field("PeakPrivateCommitUsage", &self.PeakPrivateCommitUsage).field("TotalCommitUsage", &self.TotalCommitUsage).finish()
    }
}
unsafe impl ::windows::core::Abi for APP_MEMORY_INFORMATION {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for APP_MEMORY_INFORMATION {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<APP_MEMORY_INFORMATION>()) == 0 }
    }
}
impl ::core::cmp::Eq for APP_MEMORY_INFORMATION {}
impl ::core::default::Default for APP_MEMORY_INFORMATION {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct AVRT_PRIORITY(pub i32);
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
pub const AVRT_PRIORITY_VERYLOW: AVRT_PRIORITY = AVRT_PRIORITY(-2i32);
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
pub const AVRT_PRIORITY_LOW: AVRT_PRIORITY = AVRT_PRIORITY(-1i32);
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
pub const AVRT_PRIORITY_NORMAL: AVRT_PRIORITY = AVRT_PRIORITY(0i32);
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
pub const AVRT_PRIORITY_HIGH: AVRT_PRIORITY = AVRT_PRIORITY(1i32);
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
pub const AVRT_PRIORITY_CRITICAL: AVRT_PRIORITY = AVRT_PRIORITY(2i32);
impl ::core::marker::Copy for AVRT_PRIORITY {}
impl ::core::clone::Clone for AVRT_PRIORITY {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for AVRT_PRIORITY {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for AVRT_PRIORITY {
    type Abi = Self;
}
impl ::core::fmt::Debug for AVRT_PRIORITY {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("AVRT_PRIORITY").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
#[inline]
pub unsafe fn AcquireSRWLockExclusive(srwlock: *mut RTL_SRWLOCK) {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn AcquireSRWLockExclusive(srwlock: *mut RTL_SRWLOCK);
        }
        AcquireSRWLockExclusive(::core::mem::transmute(srwlock))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
#[inline]
pub unsafe fn AcquireSRWLockShared(srwlock: *mut RTL_SRWLOCK) {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn AcquireSRWLockShared(srwlock: *mut RTL_SRWLOCK);
        }
        AcquireSRWLockShared(::core::mem::transmute(srwlock))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_Threading\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn AddIntegrityLabelToBoundaryDescriptor<'a, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::PSID>>(boundarydescriptor: *mut super::super::Foundation::HANDLE, integritylabel: Param1) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn AddIntegrityLabelToBoundaryDescriptor(boundarydescriptor: *mut super::super::Foundation::HANDLE, integritylabel: super::super::Foundation::PSID) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(AddIntegrityLabelToBoundaryDescriptor(::core::mem::transmute(boundarydescriptor), integritylabel.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_Threading\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn AddSIDToBoundaryDescriptor<'a, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::PSID>>(boundarydescriptor: *mut super::super::Foundation::HANDLE, requiredsid: Param1) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn AddSIDToBoundaryDescriptor(boundarydescriptor: *mut super::super::Foundation::HANDLE, requiredsid: super::super::Foundation::PSID) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(AddSIDToBoundaryDescriptor(::core::mem::transmute(boundarydescriptor), requiredsid.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_Threading\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn AttachThreadInput<'a, Param2: ::windows::core::IntoParam<'a, super::super::Foundation::BOOL>>(idattach: u32, idattachto: u32, fattach: Param2) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn AttachThreadInput(idattach: u32, idattachto: u32, fattach: super::super::Foundation::BOOL) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(AttachThreadInput(::core::mem::transmute(idattach), ::core::mem::transmute(idattachto), fattach.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_Threading\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn AvQuerySystemResponsiveness<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::HANDLE>>(avrthandle: Param0, systemresponsivenessvalue: *mut u32) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn AvQuerySystemResponsiveness(avrthandle: super::super::Foundation::HANDLE, systemresponsivenessvalue: *mut u32) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(AvQuerySystemResponsiveness(avrthandle.into_param().abi(), ::core::mem::transmute(systemresponsivenessvalue)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_Threading\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn AvRevertMmThreadCharacteristics<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::HANDLE>>(avrthandle: Param0) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn AvRevertMmThreadCharacteristics(avrthandle: super::super::Foundation::HANDLE) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(AvRevertMmThreadCharacteristics(avrthandle.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_Threading\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn AvRtCreateThreadOrderingGroup(context: *mut super::super::Foundation::HANDLE, period: *const i64, threadorderingguid: *mut ::windows::core::GUID, timeout: *const i64) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn AvRtCreateThreadOrderingGroup(context: *mut super::super::Foundation::HANDLE, period: *const i64, threadorderingguid: *mut ::windows::core::GUID, timeout: *const i64) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(AvRtCreateThreadOrderingGroup(::core::mem::transmute(context), ::core::mem::transmute(period), ::core::mem::transmute(threadorderingguid), ::core::mem::transmute(timeout)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_Threading\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn AvRtCreateThreadOrderingGroupExA<'a, Param4: ::windows::core::IntoParam<'a, ::windows::core::PCSTR>>(context: *mut super::super::Foundation::HANDLE, period: *const i64, threadorderingguid: *mut ::windows::core::GUID, timeout: *const i64, taskname: Param4) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn AvRtCreateThreadOrderingGroupExA(context: *mut super::super::Foundation::HANDLE, period: *const i64, threadorderingguid: *mut ::windows::core::GUID, timeout: *const i64, taskname: ::windows::core::PCSTR) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(AvRtCreateThreadOrderingGroupExA(::core::mem::transmute(context), ::core::mem::transmute(period), ::core::mem::transmute(threadorderingguid), ::core::mem::transmute(timeout), taskname.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_Threading\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn AvRtCreateThreadOrderingGroupExW<'a, Param4: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>>(context: *mut super::super::Foundation::HANDLE, period: *const i64, threadorderingguid: *mut ::windows::core::GUID, timeout: *const i64, taskname: Param4) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn AvRtCreateThreadOrderingGroupExW(context: *mut super::super::Foundation::HANDLE, period: *const i64, threadorderingguid: *mut ::windows::core::GUID, timeout: *const i64, taskname: ::windows::core::PCWSTR) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(AvRtCreateThreadOrderingGroupExW(::core::mem::transmute(context), ::core::mem::transmute(period), ::core::mem::transmute(threadorderingguid), ::core::mem::transmute(timeout), taskname.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_Threading\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn AvRtDeleteThreadOrderingGroup<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::HANDLE>>(context: Param0) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn AvRtDeleteThreadOrderingGroup(context: super::super::Foundation::HANDLE) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(AvRtDeleteThreadOrderingGroup(context.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_Threading\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn AvRtJoinThreadOrderingGroup<'a, Param2: ::windows::core::IntoParam<'a, super::super::Foundation::BOOL>>(context: *mut super::super::Foundation::HANDLE, threadorderingguid: *const ::windows::core::GUID, before: Param2) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn AvRtJoinThreadOrderingGroup(context: *mut super::super::Foundation::HANDLE, threadorderingguid: *const ::windows::core::GUID, before: super::super::Foundation::BOOL) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(AvRtJoinThreadOrderingGroup(::core::mem::transmute(context), ::core::mem::transmute(threadorderingguid), before.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_Threading\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn AvRtLeaveThreadOrderingGroup<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::HANDLE>>(context: Param0) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn AvRtLeaveThreadOrderingGroup(context: super::super::Foundation::HANDLE) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(AvRtLeaveThreadOrderingGroup(context.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_Threading\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn AvRtWaitOnThreadOrderingGroup<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::HANDLE>>(context: Param0) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn AvRtWaitOnThreadOrderingGroup(context: super::super::Foundation::HANDLE) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(AvRtWaitOnThreadOrderingGroup(context.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_Threading\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn AvSetMmMaxThreadCharacteristicsA<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::PCSTR>, Param1: ::windows::core::IntoParam<'a, ::windows::core::PCSTR>>(firsttask: Param0, secondtask: Param1, taskindex: *mut u32) -> super::super::Foundation::HANDLE {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn AvSetMmMaxThreadCharacteristicsA(firsttask: ::windows::core::PCSTR, secondtask: ::windows::core::PCSTR, taskindex: *mut u32) -> super::super::Foundation::HANDLE;
        }
        ::core::mem::transmute(AvSetMmMaxThreadCharacteristicsA(firsttask.into_param().abi(), secondtask.into_param().abi(), ::core::mem::transmute(taskindex)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_Threading\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn AvSetMmMaxThreadCharacteristicsW<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>, Param1: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>>(firsttask: Param0, secondtask: Param1, taskindex: *mut u32) -> super::super::Foundation::HANDLE {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn AvSetMmMaxThreadCharacteristicsW(firsttask: ::windows::core::PCWSTR, secondtask: ::windows::core::PCWSTR, taskindex: *mut u32) -> super::super::Foundation::HANDLE;
        }
        ::core::mem::transmute(AvSetMmMaxThreadCharacteristicsW(firsttask.into_param().abi(), secondtask.into_param().abi(), ::core::mem::transmute(taskindex)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_Threading\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn AvSetMmThreadCharacteristicsA<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::PCSTR>>(taskname: Param0, taskindex: *mut u32) -> super::super::Foundation::HANDLE {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn AvSetMmThreadCharacteristicsA(taskname: ::windows::core::PCSTR, taskindex: *mut u32) -> super::super::Foundation::HANDLE;
        }
        ::core::mem::transmute(AvSetMmThreadCharacteristicsA(taskname.into_param().abi(), ::core::mem::transmute(taskindex)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_Threading\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn AvSetMmThreadCharacteristicsW<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>>(taskname: Param0, taskindex: *mut u32) -> super::super::Foundation::HANDLE {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn AvSetMmThreadCharacteristicsW(taskname: ::windows::core::PCWSTR, taskindex: *mut u32) -> super::super::Foundation::HANDLE;
        }
        ::core::mem::transmute(AvSetMmThreadCharacteristicsW(taskname.into_param().abi(), ::core::mem::transmute(taskindex)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_Threading\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn AvSetMmThreadPriority<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::HANDLE>>(avrthandle: Param0, priority: AVRT_PRIORITY) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn AvSetMmThreadPriority(avrthandle: super::super::Foundation::HANDLE, priority: AVRT_PRIORITY) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(AvSetMmThreadPriority(avrthandle.into_param().abi(), ::core::mem::transmute(priority)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct BoundaryDescriptorHandle(pub isize);
impl BoundaryDescriptorHandle {
    pub fn is_invalid(&self) -> bool {
        *self == unsafe { ::core::mem::zeroed() }
    }
    pub fn ok(self) -> ::windows::core::Result<Self> {
        if !self.is_invalid() {
            Ok(self)
        } else {
            Err(::windows::core::Error::from_win32())
        }
    }
}
impl ::core::default::Default for BoundaryDescriptorHandle {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::clone::Clone for BoundaryDescriptorHandle {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::marker::Copy for BoundaryDescriptorHandle {}
impl ::core::fmt::Debug for BoundaryDescriptorHandle {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("BoundaryDescriptorHandle").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Abi for BoundaryDescriptorHandle {
    type Abi = Self;
}
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
pub const CONDITION_VARIABLE_LOCKMODE_SHARED: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct CREATE_EVENT(pub u32);
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
pub const CREATE_EVENT_INITIAL_SET: CREATE_EVENT = CREATE_EVENT(2u32);
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
pub const CREATE_EVENT_MANUAL_RESET: CREATE_EVENT = CREATE_EVENT(1u32);
impl ::core::marker::Copy for CREATE_EVENT {}
impl ::core::clone::Clone for CREATE_EVENT {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for CREATE_EVENT {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for CREATE_EVENT {
    type Abi = Self;
}
impl ::core::fmt::Debug for CREATE_EVENT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CREATE_EVENT").field(&self.0).finish()
    }
}
impl ::core::ops::BitOr for CREATE_EVENT {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for CREATE_EVENT {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for CREATE_EVENT {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for CREATE_EVENT {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for CREATE_EVENT {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
pub const CREATE_MUTEX_INITIAL_OWNER: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct CREATE_PROCESS_LOGON_FLAGS(pub u32);
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
pub const LOGON_WITH_PROFILE: CREATE_PROCESS_LOGON_FLAGS = CREATE_PROCESS_LOGON_FLAGS(1u32);
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
pub const LOGON_NETCREDENTIALS_ONLY: CREATE_PROCESS_LOGON_FLAGS = CREATE_PROCESS_LOGON_FLAGS(2u32);
impl ::core::marker::Copy for CREATE_PROCESS_LOGON_FLAGS {}
impl ::core::clone::Clone for CREATE_PROCESS_LOGON_FLAGS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for CREATE_PROCESS_LOGON_FLAGS {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for CREATE_PROCESS_LOGON_FLAGS {
    type Abi = Self;
}
impl ::core::fmt::Debug for CREATE_PROCESS_LOGON_FLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CREATE_PROCESS_LOGON_FLAGS").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
pub const CREATE_WAITABLE_TIMER_HIGH_RESOLUTION: u32 = 2u32;
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
pub const CREATE_WAITABLE_TIMER_MANUAL_RESET: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_Threading\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn CallbackMayRunLong(pci: *mut TP_CALLBACK_INSTANCE) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CallbackMayRunLong(pci: *mut TP_CALLBACK_INSTANCE) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(CallbackMayRunLong(::core::mem::transmute(pci)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
#[inline]
pub unsafe fn CancelThreadpoolIo(pio: *mut TP_IO) {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CancelThreadpoolIo(pio: *mut TP_IO);
        }
        CancelThreadpoolIo(::core::mem::transmute(pio))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_Threading\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn CancelWaitableTimer<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::HANDLE>>(htimer: Param0) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CancelWaitableTimer(htimer: super::super::Foundation::HANDLE) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(CancelWaitableTimer(htimer.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_Threading\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn ChangeTimerQueueTimer<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::HANDLE>, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::HANDLE>>(timerqueue: Param0, timer: Param1, duetime: u32, period: u32) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn ChangeTimerQueueTimer(timerqueue: super::super::Foundation::HANDLE, timer: super::super::Foundation::HANDLE, duetime: u32, period: u32) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(ChangeTimerQueueTimer(timerqueue.into_param().abi(), timer.into_param().abi(), ::core::mem::transmute(duetime), ::core::mem::transmute(period)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_Threading\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn ClosePrivateNamespace<'a, Param0: ::windows::core::IntoParam<'a, NamespaceHandle>>(handle: Param0, flags: u32) -> super::super::Foundation::BOOLEAN {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn ClosePrivateNamespace(handle: NamespaceHandle, flags: u32) -> super::super::Foundation::BOOLEAN;
        }
        ::core::mem::transmute(ClosePrivateNamespace(handle.into_param().abi(), ::core::mem::transmute(flags)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
#[inline]
pub unsafe fn CloseThreadpool<'a, Param0: ::windows::core::IntoParam<'a, PTP_POOL>>(ptpp: Param0) {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CloseThreadpool(ptpp: PTP_POOL);
        }
        CloseThreadpool(ptpp.into_param().abi())
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
#[inline]
pub unsafe fn CloseThreadpoolCleanupGroup(ptpcg: isize) {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CloseThreadpoolCleanupGroup(ptpcg: isize);
        }
        CloseThreadpoolCleanupGroup(::core::mem::transmute(ptpcg))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_Threading\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn CloseThreadpoolCleanupGroupMembers<'a, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::BOOL>>(ptpcg: isize, fcancelpendingcallbacks: Param1, pvcleanupcontext: *mut ::core::ffi::c_void) {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CloseThreadpoolCleanupGroupMembers(ptpcg: isize, fcancelpendingcallbacks: super::super::Foundation::BOOL, pvcleanupcontext: *mut ::core::ffi::c_void);
        }
        CloseThreadpoolCleanupGroupMembers(::core::mem::transmute(ptpcg), fcancelpendingcallbacks.into_param().abi(), ::core::mem::transmute(pvcleanupcontext))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
#[inline]
pub unsafe fn CloseThreadpoolIo(pio: *mut TP_IO) {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CloseThreadpoolIo(pio: *mut TP_IO);
        }
        CloseThreadpoolIo(::core::mem::transmute(pio))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
#[inline]
pub unsafe fn CloseThreadpoolTimer(pti: *mut TP_TIMER) {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CloseThreadpoolTimer(pti: *mut TP_TIMER);
        }
        CloseThreadpoolTimer(::core::mem::transmute(pti))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
#[inline]
pub unsafe fn CloseThreadpoolWait(pwa: *mut TP_WAIT) {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CloseThreadpoolWait(pwa: *mut TP_WAIT);
        }
        CloseThreadpoolWait(::core::mem::transmute(pwa))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
#[inline]
pub unsafe fn CloseThreadpoolWork(pwk: *mut TP_WORK) {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CloseThreadpoolWork(pwk: *mut TP_WORK);
        }
        CloseThreadpoolWork(::core::mem::transmute(pwk))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_Threading\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn ConvertFiberToThread() -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn ConvertFiberToThread() -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(ConvertFiberToThread())
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
#[inline]
pub unsafe fn ConvertThreadToFiber(lpparameter: *const ::core::ffi::c_void) -> *mut ::core::ffi::c_void {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn ConvertThreadToFiber(lpparameter: *const ::core::ffi::c_void) -> *mut ::core::ffi::c_void;
        }
        ::core::mem::transmute(ConvertThreadToFiber(::core::mem::transmute(lpparameter)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
#[inline]
pub unsafe fn ConvertThreadToFiberEx(lpparameter: *const ::core::ffi::c_void, dwflags: u32) -> *mut ::core::ffi::c_void {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn ConvertThreadToFiberEx(lpparameter: *const ::core::ffi::c_void, dwflags: u32) -> *mut ::core::ffi::c_void;
        }
        ::core::mem::transmute(ConvertThreadToFiberEx(::core::mem::transmute(lpparameter), ::core::mem::transmute(dwflags)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
#[inline]
pub unsafe fn CreateBoundaryDescriptorA<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::PCSTR>>(name: Param0, flags: u32) -> BoundaryDescriptorHandle {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CreateBoundaryDescriptorA(name: ::windows::core::PCSTR, flags: u32) -> BoundaryDescriptorHandle;
        }
        ::core::mem::transmute(CreateBoundaryDescriptorA(name.into_param().abi(), ::core::mem::transmute(flags)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
#[inline]
pub unsafe fn CreateBoundaryDescriptorW<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>>(name: Param0, flags: u32) -> BoundaryDescriptorHandle {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CreateBoundaryDescriptorW(name: ::windows::core::PCWSTR, flags: u32) -> BoundaryDescriptorHandle;
        }
        ::core::mem::transmute(CreateBoundaryDescriptorW(name.into_param().abi(), ::core::mem::transmute(flags)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_Threading\"`, `\"Win32_Foundation\"`, `\"Win32_Security\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
#[inline]
pub unsafe fn CreateEventA<'a, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::BOOL>, Param2: ::windows::core::IntoParam<'a, super::super::Foundation::BOOL>, Param3: ::windows::core::IntoParam<'a, ::windows::core::PCSTR>>(lpeventattributes: *const super::super::Security::SECURITY_ATTRIBUTES, bmanualreset: Param1, binitialstate: Param2, lpname: Param3) -> super::super::Foundation::HANDLE {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CreateEventA(lpeventattributes: *const super::super::Security::SECURITY_ATTRIBUTES, bmanualreset: super::super::Foundation::BOOL, binitialstate: super::super::Foundation::BOOL, lpname: ::windows::core::PCSTR) -> super::super::Foundation::HANDLE;
        }
        ::core::mem::transmute(CreateEventA(::core::mem::transmute(lpeventattributes), bmanualreset.into_param().abi(), binitialstate.into_param().abi(), lpname.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_Threading\"`, `\"Win32_Foundation\"`, `\"Win32_Security\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
#[inline]
pub unsafe fn CreateEventExA<'a, Param1: ::windows::core::IntoParam<'a, ::windows::core::PCSTR>>(lpeventattributes: *const super::super::Security::SECURITY_ATTRIBUTES, lpname: Param1, dwflags: CREATE_EVENT, dwdesiredaccess: u32) -> super::super::Foundation::HANDLE {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CreateEventExA(lpeventattributes: *const super::super::Security::SECURITY_ATTRIBUTES, lpname: ::windows::core::PCSTR, dwflags: CREATE_EVENT, dwdesiredaccess: u32) -> super::super::Foundation::HANDLE;
        }
        ::core::mem::transmute(CreateEventExA(::core::mem::transmute(lpeventattributes), lpname.into_param().abi(), ::core::mem::transmute(dwflags), ::core::mem::transmute(dwdesiredaccess)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_Threading\"`, `\"Win32_Foundation\"`, `\"Win32_Security\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
#[inline]
pub unsafe fn CreateEventExW<'a, Param1: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>>(lpeventattributes: *const super::super::Security::SECURITY_ATTRIBUTES, lpname: Param1, dwflags: CREATE_EVENT, dwdesiredaccess: u32) -> super::super::Foundation::HANDLE {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CreateEventExW(lpeventattributes: *const super::super::Security::SECURITY_ATTRIBUTES, lpname: ::windows::core::PCWSTR, dwflags: CREATE_EVENT, dwdesiredaccess: u32) -> super::super::Foundation::HANDLE;
        }
        ::core::mem::transmute(CreateEventExW(::core::mem::transmute(lpeventattributes), lpname.into_param().abi(), ::core::mem::transmute(dwflags), ::core::mem::transmute(dwdesiredaccess)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_Threading\"`, `\"Win32_Foundation\"`, `\"Win32_Security\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
#[inline]
pub unsafe fn CreateEventW<'a, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::BOOL>, Param2: ::windows::core::IntoParam<'a, super::super::Foundation::BOOL>, Param3: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>>(lpeventattributes: *const super::super::Security::SECURITY_ATTRIBUTES, bmanualreset: Param1, binitialstate: Param2, lpname: Param3) -> super::super::Foundation::HANDLE {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CreateEventW(lpeventattributes: *const super::super::Security::SECURITY_ATTRIBUTES, bmanualreset: super::super::Foundation::BOOL, binitialstate: super::super::Foundation::BOOL, lpname: ::windows::core::PCWSTR) -> super::super::Foundation::HANDLE;
        }
        ::core::mem::transmute(CreateEventW(::core::mem::transmute(lpeventattributes), bmanualreset.into_param().abi(), binitialstate.into_param().abi(), lpname.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
#[inline]
pub unsafe fn CreateFiber(dwstacksize: usize, lpstartaddress: LPFIBER_START_ROUTINE, lpparameter: *const ::core::ffi::c_void) -> *mut ::core::ffi::c_void {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CreateFiber(dwstacksize: usize, lpstartaddress: ::windows::core::RawPtr, lpparameter: *const ::core::ffi::c_void) -> *mut ::core::ffi::c_void;
        }
        ::core::mem::transmute(CreateFiber(::core::mem::transmute(dwstacksize), ::core::mem::transmute(lpstartaddress), ::core::mem::transmute(lpparameter)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
#[inline]
pub unsafe fn CreateFiberEx(dwstackcommitsize: usize, dwstackreservesize: usize, dwflags: u32, lpstartaddress: LPFIBER_START_ROUTINE, lpparameter: *const ::core::ffi::c_void) -> *mut ::core::ffi::c_void {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CreateFiberEx(dwstackcommitsize: usize, dwstackreservesize: usize, dwflags: u32, lpstartaddress: ::windows::core::RawPtr, lpparameter: *const ::core::ffi::c_void) -> *mut ::core::ffi::c_void;
        }
        ::core::mem::transmute(CreateFiberEx(::core::mem::transmute(dwstackcommitsize), ::core::mem::transmute(dwstackreservesize), ::core::mem::transmute(dwflags), ::core::mem::transmute(lpstartaddress), ::core::mem::transmute(lpparameter)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_Threading\"`, `\"Win32_Foundation\"`, `\"Win32_Security\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
#[inline]
pub unsafe fn CreateMutexA<'a, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::BOOL>, Param2: ::windows::core::IntoParam<'a, ::windows::core::PCSTR>>(lpmutexattributes: *const super::super::Security::SECURITY_ATTRIBUTES, binitialowner: Param1, lpname: Param2) -> super::super::Foundation::HANDLE {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CreateMutexA(lpmutexattributes: *const super::super::Security::SECURITY_ATTRIBUTES, binitialowner: super::super::Foundation::BOOL, lpname: ::windows::core::PCSTR) -> super::super::Foundation::HANDLE;
        }
        ::core::mem::transmute(CreateMutexA(::core::mem::transmute(lpmutexattributes), binitialowner.into_param().abi(), lpname.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_Threading\"`, `\"Win32_Foundation\"`, `\"Win32_Security\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
#[inline]
pub unsafe fn CreateMutexExA<'a, Param1: ::windows::core::IntoParam<'a, ::windows::core::PCSTR>>(lpmutexattributes: *const super::super::Security::SECURITY_ATTRIBUTES, lpname: Param1, dwflags: u32, dwdesiredaccess: u32) -> super::super::Foundation::HANDLE {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CreateMutexExA(lpmutexattributes: *const super::super::Security::SECURITY_ATTRIBUTES, lpname: ::windows::core::PCSTR, dwflags: u32, dwdesiredaccess: u32) -> super::super::Foundation::HANDLE;
        }
        ::core::mem::transmute(CreateMutexExA(::core::mem::transmute(lpmutexattributes), lpname.into_param().abi(), ::core::mem::transmute(dwflags), ::core::mem::transmute(dwdesiredaccess)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_Threading\"`, `\"Win32_Foundation\"`, `\"Win32_Security\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
#[inline]
pub unsafe fn CreateMutexExW<'a, Param1: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>>(lpmutexattributes: *const super::super::Security::SECURITY_ATTRIBUTES, lpname: Param1, dwflags: u32, dwdesiredaccess: u32) -> super::super::Foundation::HANDLE {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CreateMutexExW(lpmutexattributes: *const super::super::Security::SECURITY_ATTRIBUTES, lpname: ::windows::core::PCWSTR, dwflags: u32, dwdesiredaccess: u32) -> super::super::Foundation::HANDLE;
        }
        ::core::mem::transmute(CreateMutexExW(::core::mem::transmute(lpmutexattributes), lpname.into_param().abi(), ::core::mem::transmute(dwflags), ::core::mem::transmute(dwdesiredaccess)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_Threading\"`, `\"Win32_Foundation\"`, `\"Win32_Security\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
#[inline]
pub unsafe fn CreateMutexW<'a, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::BOOL>, Param2: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>>(lpmutexattributes: *const super::super::Security::SECURITY_ATTRIBUTES, binitialowner: Param1, lpname: Param2) -> super::super::Foundation::HANDLE {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CreateMutexW(lpmutexattributes: *const super::super::Security::SECURITY_ATTRIBUTES, binitialowner: super::super::Foundation::BOOL, lpname: ::windows::core::PCWSTR) -> super::super::Foundation::HANDLE;
        }
        ::core::mem::transmute(CreateMutexW(::core::mem::transmute(lpmutexattributes), binitialowner.into_param().abi(), lpname.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_Threading\"`, `\"Win32_Foundation\"`, `\"Win32_Security\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
#[inline]
pub unsafe fn CreatePrivateNamespaceA<'a, Param2: ::windows::core::IntoParam<'a, ::windows::core::PCSTR>>(lpprivatenamespaceattributes: *const super::super::Security::SECURITY_ATTRIBUTES, lpboundarydescriptor: *const ::core::ffi::c_void, lpaliasprefix: Param2) -> NamespaceHandle {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CreatePrivateNamespaceA(lpprivatenamespaceattributes: *const super::super::Security::SECURITY_ATTRIBUTES, lpboundarydescriptor: *const ::core::ffi::c_void, lpaliasprefix: ::windows::core::PCSTR) -> NamespaceHandle;
        }
        ::core::mem::transmute(CreatePrivateNamespaceA(::core::mem::transmute(lpprivatenamespaceattributes), ::core::mem::transmute(lpboundarydescriptor), lpaliasprefix.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_Threading\"`, `\"Win32_Foundation\"`, `\"Win32_Security\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
#[inline]
pub unsafe fn CreatePrivateNamespaceW<'a, Param2: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>>(lpprivatenamespaceattributes: *const super::super::Security::SECURITY_ATTRIBUTES, lpboundarydescriptor: *const ::core::ffi::c_void, lpaliasprefix: Param2) -> NamespaceHandle {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CreatePrivateNamespaceW(lpprivatenamespaceattributes: *const super::super::Security::SECURITY_ATTRIBUTES, lpboundarydescriptor: *const ::core::ffi::c_void, lpaliasprefix: ::windows::core::PCWSTR) -> NamespaceHandle;
        }
        ::core::mem::transmute(CreatePrivateNamespaceW(::core::mem::transmute(lpprivatenamespaceattributes), ::core::mem::transmute(lpboundarydescriptor), lpaliasprefix.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_Threading\"`, `\"Win32_Foundation\"`, `\"Win32_Security\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
#[inline]
pub unsafe fn CreateProcessA<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::PCSTR>, Param4: ::windows::core::IntoParam<'a, super::super::Foundation::BOOL>, Param7: ::windows::core::IntoParam<'a, ::windows::core::PCSTR>>(lpapplicationname: Param0, lpcommandline: ::windows::core::PSTR, lpprocessattributes: *const super::super::Security::SECURITY_ATTRIBUTES, lpthreadattributes: *const super::super::Security::SECURITY_ATTRIBUTES, binherithandles: Param4, dwcreationflags: PROCESS_CREATION_FLAGS, lpenvironment: *const ::core::ffi::c_void, lpcurrentdirectory: Param7, lpstartupinfo: *const STARTUPINFOA, lpprocessinformation: *mut PROCESS_INFORMATION) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CreateProcessA(lpapplicationname: ::windows::core::PCSTR, lpcommandline: ::windows::core::PSTR, lpprocessattributes: *const super::super::Security::SECURITY_ATTRIBUTES, lpthreadattributes: *const super::super::Security::SECURITY_ATTRIBUTES, binherithandles: super::super::Foundation::BOOL, dwcreationflags: PROCESS_CREATION_FLAGS, lpenvironment: *const ::core::ffi::c_void, lpcurrentdirectory: ::windows::core::PCSTR, lpstartupinfo: *const STARTUPINFOA, lpprocessinformation: *mut PROCESS_INFORMATION) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(CreateProcessA(lpapplicationname.into_param().abi(), ::core::mem::transmute(lpcommandline), ::core::mem::transmute(lpprocessattributes), ::core::mem::transmute(lpthreadattributes), binherithandles.into_param().abi(), ::core::mem::transmute(dwcreationflags), ::core::mem::transmute(lpenvironment), lpcurrentdirectory.into_param().abi(), ::core::mem::transmute(lpstartupinfo), ::core::mem::transmute(lpprocessinformation)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_Threading\"`, `\"Win32_Foundation\"`, `\"Win32_Security\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
#[inline]
pub unsafe fn CreateProcessAsUserA<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::HANDLE>, Param1: ::windows::core::IntoParam<'a, ::windows::core::PCSTR>, Param5: ::windows::core::IntoParam<'a, super::super::Foundation::BOOL>, Param8: ::windows::core::IntoParam<'a, ::windows::core::PCSTR>>(htoken: Param0, lpapplicationname: Param1, lpcommandline: ::windows::core::PSTR, lpprocessattributes: *const super::super::Security::SECURITY_ATTRIBUTES, lpthreadattributes: *const super::super::Security::SECURITY_ATTRIBUTES, binherithandles: Param5, dwcreationflags: u32, lpenvironment: *const ::core::ffi::c_void, lpcurrentdirectory: Param8, lpstartupinfo: *const STARTUPINFOA, lpprocessinformation: *mut PROCESS_INFORMATION) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CreateProcessAsUserA(htoken: super::super::Foundation::HANDLE, lpapplicationname: ::windows::core::PCSTR, lpcommandline: ::windows::core::PSTR, lpprocessattributes: *const super::super::Security::SECURITY_ATTRIBUTES, lpthreadattributes: *const super::super::Security::SECURITY_ATTRIBUTES, binherithandles: super::super::Foundation::BOOL, dwcreationflags: u32, lpenvironment: *const ::core::ffi::c_void, lpcurrentdirectory: ::windows::core::PCSTR, lpstartupinfo: *const STARTUPINFOA, lpprocessinformation: *mut PROCESS_INFORMATION) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(CreateProcessAsUserA(htoken.into_param().abi(), lpapplicationname.into_param().abi(), ::core::mem::transmute(lpcommandline), ::core::mem::transmute(lpprocessattributes), ::core::mem::transmute(lpthreadattributes), binherithandles.into_param().abi(), ::core::mem::transmute(dwcreationflags), ::core::mem::transmute(lpenvironment), lpcurrentdirectory.into_param().abi(), ::core::mem::transmute(lpstartupinfo), ::core::mem::transmute(lpprocessinformation)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_Threading\"`, `\"Win32_Foundation\"`, `\"Win32_Security\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
#[inline]
pub unsafe fn CreateProcessAsUserW<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::HANDLE>, Param1: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>, Param5: ::windows::core::IntoParam<'a, super::super::Foundation::BOOL>, Param8: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>>(htoken: Param0, lpapplicationname: Param1, lpcommandline: ::windows::core::PWSTR, lpprocessattributes: *const super::super::Security::SECURITY_ATTRIBUTES, lpthreadattributes: *const super::super::Security::SECURITY_ATTRIBUTES, binherithandles: Param5, dwcreationflags: u32, lpenvironment: *const ::core::ffi::c_void, lpcurrentdirectory: Param8, lpstartupinfo: *const STARTUPINFOW, lpprocessinformation: *mut PROCESS_INFORMATION) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CreateProcessAsUserW(htoken: super::super::Foundation::HANDLE, lpapplicationname: ::windows::core::PCWSTR, lpcommandline: ::windows::core::PWSTR, lpprocessattributes: *const super::super::Security::SECURITY_ATTRIBUTES, lpthreadattributes: *const super::super::Security::SECURITY_ATTRIBUTES, binherithandles: super::super::Foundation::BOOL, dwcreationflags: u32, lpenvironment: *const ::core::ffi::c_void, lpcurrentdirectory: ::windows::core::PCWSTR, lpstartupinfo: *const STARTUPINFOW, lpprocessinformation: *mut PROCESS_INFORMATION) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(CreateProcessAsUserW(htoken.into_param().abi(), lpapplicationname.into_param().abi(), ::core::mem::transmute(lpcommandline), ::core::mem::transmute(lpprocessattributes), ::core::mem::transmute(lpthreadattributes), binherithandles.into_param().abi(), ::core::mem::transmute(dwcreationflags), ::core::mem::transmute(lpenvironment), lpcurrentdirectory.into_param().abi(), ::core::mem::transmute(lpstartupinfo), ::core::mem::transmute(lpprocessinformation)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_Threading\"`, `\"Win32_Foundation\"`, `\"Win32_Security\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
#[inline]
pub unsafe fn CreateProcessW<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>, Param4: ::windows::core::IntoParam<'a, super::super::Foundation::BOOL>, Param7: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>>(lpapplicationname: Param0, lpcommandline: ::windows::core::PWSTR, lpprocessattributes: *const super::super::Security::SECURITY_ATTRIBUTES, lpthreadattributes: *const super::super::Security::SECURITY_ATTRIBUTES, binherithandles: Param4, dwcreationflags: PROCESS_CREATION_FLAGS, lpenvironment: *const ::core::ffi::c_void, lpcurrentdirectory: Param7, lpstartupinfo: *const STARTUPINFOW, lpprocessinformation: *mut PROCESS_INFORMATION) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CreateProcessW(lpapplicationname: ::windows::core::PCWSTR, lpcommandline: ::windows::core::PWSTR, lpprocessattributes: *const super::super::Security::SECURITY_ATTRIBUTES, lpthreadattributes: *const super::super::Security::SECURITY_ATTRIBUTES, binherithandles: super::super::Foundation::BOOL, dwcreationflags: PROCESS_CREATION_FLAGS, lpenvironment: *const ::core::ffi::c_void, lpcurrentdirectory: ::windows::core::PCWSTR, lpstartupinfo: *const STARTUPINFOW, lpprocessinformation: *mut PROCESS_INFORMATION) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(CreateProcessW(lpapplicationname.into_param().abi(), ::core::mem::transmute(lpcommandline), ::core::mem::transmute(lpprocessattributes), ::core::mem::transmute(lpthreadattributes), binherithandles.into_param().abi(), ::core::mem::transmute(dwcreationflags), ::core::mem::transmute(lpenvironment), lpcurrentdirectory.into_param().abi(), ::core::mem::transmute(lpstartupinfo), ::core::mem::transmute(lpprocessinformation)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_Threading\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn CreateProcessWithLogonW<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>, Param1: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>, Param2: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>, Param4: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>, Param8: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>>(lpusername: Param0, lpdomain: Param1, lppassword: Param2, dwlogonflags: CREATE_PROCESS_LOGON_FLAGS, lpapplicationname: Param4, lpcommandline: ::windows::core::PWSTR, dwcreationflags: u32, lpenvironment: *const ::core::ffi::c_void, lpcurrentdirectory: Param8, lpstartupinfo: *const STARTUPINFOW, lpprocessinformation: *mut PROCESS_INFORMATION) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CreateProcessWithLogonW(lpusername: ::windows::core::PCWSTR, lpdomain: ::windows::core::PCWSTR, lppassword: ::windows::core::PCWSTR, dwlogonflags: CREATE_PROCESS_LOGON_FLAGS, lpapplicationname: ::windows::core::PCWSTR, lpcommandline: ::windows::core::PWSTR, dwcreationflags: u32, lpenvironment: *const ::core::ffi::c_void, lpcurrentdirectory: ::windows::core::PCWSTR, lpstartupinfo: *const STARTUPINFOW, lpprocessinformation: *mut PROCESS_INFORMATION) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(CreateProcessWithLogonW(lpusername.into_param().abi(), lpdomain.into_param().abi(), lppassword.into_param().abi(), ::core::mem::transmute(dwlogonflags), lpapplicationname.into_param().abi(), ::core::mem::transmute(lpcommandline), ::core::mem::transmute(dwcreationflags), ::core::mem::transmute(lpenvironment), lpcurrentdirectory.into_param().abi(), ::core::mem::transmute(lpstartupinfo), ::core::mem::transmute(lpprocessinformation)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_Threading\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn CreateProcessWithTokenW<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::HANDLE>, Param2: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>, Param6: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>>(htoken: Param0, dwlogonflags: CREATE_PROCESS_LOGON_FLAGS, lpapplicationname: Param2, lpcommandline: ::windows::core::PWSTR, dwcreationflags: u32, lpenvironment: *const ::core::ffi::c_void, lpcurrentdirectory: Param6, lpstartupinfo: *const STARTUPINFOW, lpprocessinformation: *mut PROCESS_INFORMATION) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CreateProcessWithTokenW(htoken: super::super::Foundation::HANDLE, dwlogonflags: CREATE_PROCESS_LOGON_FLAGS, lpapplicationname: ::windows::core::PCWSTR, lpcommandline: ::windows::core::PWSTR, dwcreationflags: u32, lpenvironment: *const ::core::ffi::c_void, lpcurrentdirectory: ::windows::core::PCWSTR, lpstartupinfo: *const STARTUPINFOW, lpprocessinformation: *mut PROCESS_INFORMATION) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(CreateProcessWithTokenW(htoken.into_param().abi(), ::core::mem::transmute(dwlogonflags), lpapplicationname.into_param().abi(), ::core::mem::transmute(lpcommandline), ::core::mem::transmute(dwcreationflags), ::core::mem::transmute(lpenvironment), lpcurrentdirectory.into_param().abi(), ::core::mem::transmute(lpstartupinfo), ::core::mem::transmute(lpprocessinformation)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_Threading\"`, `\"Win32_Foundation\"`, `\"Win32_Security\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
#[inline]
pub unsafe fn CreateRemoteThread<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::HANDLE>>(hprocess: Param0, lpthreadattributes: *const super::super::Security::SECURITY_ATTRIBUTES, dwstacksize: usize, lpstartaddress: LPTHREAD_START_ROUTINE, lpparameter: *const ::core::ffi::c_void, dwcreationflags: u32, lpthreadid: *mut u32) -> super::super::Foundation::HANDLE {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CreateRemoteThread(hprocess: super::super::Foundation::HANDLE, lpthreadattributes: *const super::super::Security::SECURITY_ATTRIBUTES, dwstacksize: usize, lpstartaddress: ::windows::core::RawPtr, lpparameter: *const ::core::ffi::c_void, dwcreationflags: u32, lpthreadid: *mut u32) -> super::super::Foundation::HANDLE;
        }
        ::core::mem::transmute(CreateRemoteThread(hprocess.into_param().abi(), ::core::mem::transmute(lpthreadattributes), ::core::mem::transmute(dwstacksize), ::core::mem::transmute(lpstartaddress), ::core::mem::transmute(lpparameter), ::core::mem::transmute(dwcreationflags), ::core::mem::transmute(lpthreadid)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_Threading\"`, `\"Win32_Foundation\"`, `\"Win32_Security\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
#[inline]
pub unsafe fn CreateRemoteThreadEx<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::HANDLE>, Param6: ::windows::core::IntoParam<'a, LPPROC_THREAD_ATTRIBUTE_LIST>>(hprocess: Param0, lpthreadattributes: *const super::super::Security::SECURITY_ATTRIBUTES, dwstacksize: usize, lpstartaddress: LPTHREAD_START_ROUTINE, lpparameter: *const ::core::ffi::c_void, dwcreationflags: u32, lpattributelist: Param6, lpthreadid: *mut u32) -> super::super::Foundation::HANDLE {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CreateRemoteThreadEx(hprocess: super::super::Foundation::HANDLE, lpthreadattributes: *const super::super::Security::SECURITY_ATTRIBUTES, dwstacksize: usize, lpstartaddress: ::windows::core::RawPtr, lpparameter: *const ::core::ffi::c_void, dwcreationflags: u32, lpattributelist: LPPROC_THREAD_ATTRIBUTE_LIST, lpthreadid: *mut u32) -> super::super::Foundation::HANDLE;
        }
        ::core::mem::transmute(CreateRemoteThreadEx(hprocess.into_param().abi(), ::core::mem::transmute(lpthreadattributes), ::core::mem::transmute(dwstacksize), ::core::mem::transmute(lpstartaddress), ::core::mem::transmute(lpparameter), ::core::mem::transmute(dwcreationflags), lpattributelist.into_param().abi(), ::core::mem::transmute(lpthreadid)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_Threading\"`, `\"Win32_Foundation\"`, `\"Win32_Security\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
#[inline]
pub unsafe fn CreateSemaphoreA<'a, Param3: ::windows::core::IntoParam<'a, ::windows::core::PCSTR>>(lpsemaphoreattributes: *const super::super::Security::SECURITY_ATTRIBUTES, linitialcount: i32, lmaximumcount: i32, lpname: Param3) -> super::super::Foundation::HANDLE {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CreateSemaphoreA(lpsemaphoreattributes: *const super::super::Security::SECURITY_ATTRIBUTES, linitialcount: i32, lmaximumcount: i32, lpname: ::windows::core::PCSTR) -> super::super::Foundation::HANDLE;
        }
        ::core::mem::transmute(CreateSemaphoreA(::core::mem::transmute(lpsemaphoreattributes), ::core::mem::transmute(linitialcount), ::core::mem::transmute(lmaximumcount), lpname.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_Threading\"`, `\"Win32_Foundation\"`, `\"Win32_Security\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
#[inline]
pub unsafe fn CreateSemaphoreExA<'a, Param3: ::windows::core::IntoParam<'a, ::windows::core::PCSTR>>(lpsemaphoreattributes: *const super::super::Security::SECURITY_ATTRIBUTES, linitialcount: i32, lmaximumcount: i32, lpname: Param3, dwflags: u32, dwdesiredaccess: u32) -> super::super::Foundation::HANDLE {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CreateSemaphoreExA(lpsemaphoreattributes: *const super::super::Security::SECURITY_ATTRIBUTES, linitialcount: i32, lmaximumcount: i32, lpname: ::windows::core::PCSTR, dwflags: u32, dwdesiredaccess: u32) -> super::super::Foundation::HANDLE;
        }
        ::core::mem::transmute(CreateSemaphoreExA(::core::mem::transmute(lpsemaphoreattributes), ::core::mem::transmute(linitialcount), ::core::mem::transmute(lmaximumcount), lpname.into_param().abi(), ::core::mem::transmute(dwflags), ::core::mem::transmute(dwdesiredaccess)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_Threading\"`, `\"Win32_Foundation\"`, `\"Win32_Security\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
#[inline]
pub unsafe fn CreateSemaphoreExW<'a, Param3: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>>(lpsemaphoreattributes: *const super::super::Security::SECURITY_ATTRIBUTES, linitialcount: i32, lmaximumcount: i32, lpname: Param3, dwflags: u32, dwdesiredaccess: u32) -> super::super::Foundation::HANDLE {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CreateSemaphoreExW(lpsemaphoreattributes: *const super::super::Security::SECURITY_ATTRIBUTES, linitialcount: i32, lmaximumcount: i32, lpname: ::windows::core::PCWSTR, dwflags: u32, dwdesiredaccess: u32) -> super::super::Foundation::HANDLE;
        }
        ::core::mem::transmute(CreateSemaphoreExW(::core::mem::transmute(lpsemaphoreattributes), ::core::mem::transmute(linitialcount), ::core::mem::transmute(lmaximumcount), lpname.into_param().abi(), ::core::mem::transmute(dwflags), ::core::mem::transmute(dwdesiredaccess)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_Threading\"`, `\"Win32_Foundation\"`, `\"Win32_Security\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
#[inline]
pub unsafe fn CreateSemaphoreW<'a, Param3: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>>(lpsemaphoreattributes: *const super::super::Security::SECURITY_ATTRIBUTES, linitialcount: i32, lmaximumcount: i32, lpname: Param3) -> super::super::Foundation::HANDLE {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CreateSemaphoreW(lpsemaphoreattributes: *const super::super::Security::SECURITY_ATTRIBUTES, linitialcount: i32, lmaximumcount: i32, lpname: ::windows::core::PCWSTR) -> super::super::Foundation::HANDLE;
        }
        ::core::mem::transmute(CreateSemaphoreW(::core::mem::transmute(lpsemaphoreattributes), ::core::mem::transmute(linitialcount), ::core::mem::transmute(lmaximumcount), lpname.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_Threading\"`, `\"Win32_Foundation\"`, `\"Win32_Security\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
#[inline]
pub unsafe fn CreateThread(lpthreadattributes: *const super::super::Security::SECURITY_ATTRIBUTES, dwstacksize: usize, lpstartaddress: LPTHREAD_START_ROUTINE, lpparameter: *const ::core::ffi::c_void, dwcreationflags: THREAD_CREATION_FLAGS, lpthreadid: *mut u32) -> super::super::Foundation::HANDLE {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CreateThread(lpthreadattributes: *const super::super::Security::SECURITY_ATTRIBUTES, dwstacksize: usize, lpstartaddress: ::windows::core::RawPtr, lpparameter: *const ::core::ffi::c_void, dwcreationflags: THREAD_CREATION_FLAGS, lpthreadid: *mut u32) -> super::super::Foundation::HANDLE;
        }
        ::core::mem::transmute(CreateThread(::core::mem::transmute(lpthreadattributes), ::core::mem::transmute(dwstacksize), ::core::mem::transmute(lpstartaddress), ::core::mem::transmute(lpparameter), ::core::mem::transmute(dwcreationflags), ::core::mem::transmute(lpthreadid)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
#[inline]
pub unsafe fn CreateThreadpool(reserved: *mut ::core::ffi::c_void) -> PTP_POOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CreateThreadpool(reserved: *mut ::core::ffi::c_void) -> PTP_POOL;
        }
        ::core::mem::transmute(CreateThreadpool(::core::mem::transmute(reserved)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
#[inline]
pub unsafe fn CreateThreadpoolCleanupGroup() -> isize {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CreateThreadpoolCleanupGroup() -> isize;
        }
        ::core::mem::transmute(CreateThreadpoolCleanupGroup())
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_Threading\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn CreateThreadpoolIo<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::HANDLE>>(fl: Param0, pfnio: PTP_WIN32_IO_CALLBACK, pv: *mut ::core::ffi::c_void, pcbe: *const TP_CALLBACK_ENVIRON_V3) -> *mut TP_IO {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CreateThreadpoolIo(fl: super::super::Foundation::HANDLE, pfnio: ::windows::core::RawPtr, pv: *mut ::core::ffi::c_void, pcbe: *const TP_CALLBACK_ENVIRON_V3) -> *mut TP_IO;
        }
        ::core::mem::transmute(CreateThreadpoolIo(fl.into_param().abi(), ::core::mem::transmute(pfnio), ::core::mem::transmute(pv), ::core::mem::transmute(pcbe)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
#[inline]
pub unsafe fn CreateThreadpoolTimer(pfnti: PTP_TIMER_CALLBACK, pv: *mut ::core::ffi::c_void, pcbe: *const TP_CALLBACK_ENVIRON_V3) -> *mut TP_TIMER {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CreateThreadpoolTimer(pfnti: ::windows::core::RawPtr, pv: *mut ::core::ffi::c_void, pcbe: *const TP_CALLBACK_ENVIRON_V3) -> *mut TP_TIMER;
        }
        ::core::mem::transmute(CreateThreadpoolTimer(::core::mem::transmute(pfnti), ::core::mem::transmute(pv), ::core::mem::transmute(pcbe)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
#[inline]
pub unsafe fn CreateThreadpoolWait(pfnwa: PTP_WAIT_CALLBACK, pv: *mut ::core::ffi::c_void, pcbe: *const TP_CALLBACK_ENVIRON_V3) -> *mut TP_WAIT {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CreateThreadpoolWait(pfnwa: ::windows::core::RawPtr, pv: *mut ::core::ffi::c_void, pcbe: *const TP_CALLBACK_ENVIRON_V3) -> *mut TP_WAIT;
        }
        ::core::mem::transmute(CreateThreadpoolWait(::core::mem::transmute(pfnwa), ::core::mem::transmute(pv), ::core::mem::transmute(pcbe)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
#[inline]
pub unsafe fn CreateThreadpoolWork(pfnwk: PTP_WORK_CALLBACK, pv: *mut ::core::ffi::c_void, pcbe: *const TP_CALLBACK_ENVIRON_V3) -> *mut TP_WORK {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CreateThreadpoolWork(pfnwk: ::windows::core::RawPtr, pv: *mut ::core::ffi::c_void, pcbe: *const TP_CALLBACK_ENVIRON_V3) -> *mut TP_WORK;
        }
        ::core::mem::transmute(CreateThreadpoolWork(::core::mem::transmute(pfnwk), ::core::mem::transmute(pv), ::core::mem::transmute(pcbe)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_Threading\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn CreateTimerQueue() -> super::super::Foundation::HANDLE {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CreateTimerQueue() -> super::super::Foundation::HANDLE;
        }
        ::core::mem::transmute(CreateTimerQueue())
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_Threading\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn CreateTimerQueueTimer<'a, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::HANDLE>>(phnewtimer: *mut super::super::Foundation::HANDLE, timerqueue: Param1, callback: WAITORTIMERCALLBACK, parameter: *const ::core::ffi::c_void, duetime: u32, period: u32, flags: WORKER_THREAD_FLAGS) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CreateTimerQueueTimer(phnewtimer: *mut super::super::Foundation::HANDLE, timerqueue: super::super::Foundation::HANDLE, callback: ::windows::core::RawPtr, parameter: *const ::core::ffi::c_void, duetime: u32, period: u32, flags: WORKER_THREAD_FLAGS) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(CreateTimerQueueTimer(::core::mem::transmute(phnewtimer), timerqueue.into_param().abi(), ::core::mem::transmute(callback), ::core::mem::transmute(parameter), ::core::mem::transmute(duetime), ::core::mem::transmute(period), ::core::mem::transmute(flags)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_Threading\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn CreateUmsCompletionList(umscompletionlist: *mut *mut ::core::ffi::c_void) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CreateUmsCompletionList(umscompletionlist: *mut *mut ::core::ffi::c_void) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(CreateUmsCompletionList(::core::mem::transmute(umscompletionlist)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_Threading\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn CreateUmsThreadContext(lpumsthread: *mut *mut ::core::ffi::c_void) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CreateUmsThreadContext(lpumsthread: *mut *mut ::core::ffi::c_void) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(CreateUmsThreadContext(::core::mem::transmute(lpumsthread)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_Threading\"`, `\"Win32_Foundation\"`, `\"Win32_Security\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
#[inline]
pub unsafe fn CreateWaitableTimerExW<'a, Param1: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>>(lptimerattributes: *const super::super::Security::SECURITY_ATTRIBUTES, lptimername: Param1, dwflags: u32, dwdesiredaccess: u32) -> super::super::Foundation::HANDLE {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CreateWaitableTimerExW(lptimerattributes: *const super::super::Security::SECURITY_ATTRIBUTES, lptimername: ::windows::core::PCWSTR, dwflags: u32, dwdesiredaccess: u32) -> super::super::Foundation::HANDLE;
        }
        ::core::mem::transmute(CreateWaitableTimerExW(::core::mem::transmute(lptimerattributes), lptimername.into_param().abi(), ::core::mem::transmute(dwflags), ::core::mem::transmute(dwdesiredaccess)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_Threading\"`, `\"Win32_Foundation\"`, `\"Win32_Security\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
#[inline]
pub unsafe fn CreateWaitableTimerW<'a, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::BOOL>, Param2: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>>(lptimerattributes: *const super::super::Security::SECURITY_ATTRIBUTES, bmanualreset: Param1, lptimername: Param2) -> super::super::Foundation::HANDLE {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CreateWaitableTimerW(lptimerattributes: *const super::super::Security::SECURITY_ATTRIBUTES, bmanualreset: super::super::Foundation::BOOL, lptimername: ::windows::core::PCWSTR) -> super::super::Foundation::HANDLE;
        }
        ::core::mem::transmute(CreateWaitableTimerW(::core::mem::transmute(lptimerattributes), bmanualreset.into_param().abi(), lptimername.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
#[inline]
pub unsafe fn DeleteBoundaryDescriptor<'a, Param0: ::windows::core::IntoParam<'a, BoundaryDescriptorHandle>>(boundarydescriptor: Param0) {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn DeleteBoundaryDescriptor(boundarydescriptor: BoundaryDescriptorHandle);
        }
        DeleteBoundaryDescriptor(boundarydescriptor.into_param().abi())
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_Threading\"`, `\"Win32_Foundation\"`, `\"Win32_System_Kernel\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Kernel"))]
#[inline]
pub unsafe fn DeleteCriticalSection(lpcriticalsection: *mut RTL_CRITICAL_SECTION) {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn DeleteCriticalSection(lpcriticalsection: *mut RTL_CRITICAL_SECTION);
        }
        DeleteCriticalSection(::core::mem::transmute(lpcriticalsection))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
#[inline]
pub unsafe fn DeleteFiber(lpfiber: *const ::core::ffi::c_void) {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn DeleteFiber(lpfiber: *const ::core::ffi::c_void);
        }
        DeleteFiber(::core::mem::transmute(lpfiber))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
#[inline]
pub unsafe fn DeleteProcThreadAttributeList<'a, Param0: ::windows::core::IntoParam<'a, LPPROC_THREAD_ATTRIBUTE_LIST>>(lpattributelist: Param0) {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn DeleteProcThreadAttributeList(lpattributelist: LPPROC_THREAD_ATTRIBUTE_LIST);
        }
        DeleteProcThreadAttributeList(lpattributelist.into_param().abi())
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_Threading\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn DeleteSynchronizationBarrier(lpbarrier: *mut RTL_BARRIER) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn DeleteSynchronizationBarrier(lpbarrier: *mut RTL_BARRIER) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(DeleteSynchronizationBarrier(::core::mem::transmute(lpbarrier)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_Threading\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn DeleteTimerQueue<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::HANDLE>>(timerqueue: Param0) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn DeleteTimerQueue(timerqueue: super::super::Foundation::HANDLE) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(DeleteTimerQueue(timerqueue.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_Threading\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn DeleteTimerQueueEx<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::HANDLE>, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::HANDLE>>(timerqueue: Param0, completionevent: Param1) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn DeleteTimerQueueEx(timerqueue: super::super::Foundation::HANDLE, completionevent: super::super::Foundation::HANDLE) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(DeleteTimerQueueEx(timerqueue.into_param().abi(), completionevent.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_Threading\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn DeleteTimerQueueTimer<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::HANDLE>, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::HANDLE>, Param2: ::windows::core::IntoParam<'a, super::super::Foundation::HANDLE>>(timerqueue: Param0, timer: Param1, completionevent: Param2) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn DeleteTimerQueueTimer(timerqueue: super::super::Foundation::HANDLE, timer: super::super::Foundation::HANDLE, completionevent: super::super::Foundation::HANDLE) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(DeleteTimerQueueTimer(timerqueue.into_param().abi(), timer.into_param().abi(), completionevent.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_Threading\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn DeleteUmsCompletionList(umscompletionlist: *const ::core::ffi::c_void) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn DeleteUmsCompletionList(umscompletionlist: *const ::core::ffi::c_void) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(DeleteUmsCompletionList(::core::mem::transmute(umscompletionlist)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_Threading\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn DeleteUmsThreadContext(umsthread: *const ::core::ffi::c_void) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn DeleteUmsThreadContext(umsthread: *const ::core::ffi::c_void) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(DeleteUmsThreadContext(::core::mem::transmute(umsthread)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_Threading\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn DequeueUmsCompletionListItems(umscompletionlist: *const ::core::ffi::c_void, waittimeout: u32, umsthreadlist: *mut *mut ::core::ffi::c_void) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn DequeueUmsCompletionListItems(umscompletionlist: *const ::core::ffi::c_void, waittimeout: u32, umsthreadlist: *mut *mut ::core::ffi::c_void) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(DequeueUmsCompletionListItems(::core::mem::transmute(umscompletionlist), ::core::mem::transmute(waittimeout), ::core::mem::transmute(umsthreadlist)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
#[inline]
pub unsafe fn DisassociateCurrentThreadFromCallback(pci: *mut TP_CALLBACK_INSTANCE) {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn DisassociateCurrentThreadFromCallback(pci: *mut TP_CALLBACK_INSTANCE);
        }
        DisassociateCurrentThreadFromCallback(::core::mem::transmute(pci))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_Threading\"`, `\"Win32_Foundation\"`, `\"Win32_System_Kernel\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Kernel"))]
#[inline]
pub unsafe fn EnterCriticalSection(lpcriticalsection: *mut RTL_CRITICAL_SECTION) {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn EnterCriticalSection(lpcriticalsection: *mut RTL_CRITICAL_SECTION);
        }
        EnterCriticalSection(::core::mem::transmute(lpcriticalsection))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_Threading\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn EnterSynchronizationBarrier(lpbarrier: *mut RTL_BARRIER, dwflags: u32) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn EnterSynchronizationBarrier(lpbarrier: *mut RTL_BARRIER, dwflags: u32) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(EnterSynchronizationBarrier(::core::mem::transmute(lpbarrier), ::core::mem::transmute(dwflags)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_Threading\"`, `\"Win32_Foundation\"`, `\"Win32_System_SystemServices\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_SystemServices"))]
#[inline]
pub unsafe fn EnterUmsSchedulingMode(schedulerstartupinfo: *const UMS_SCHEDULER_STARTUP_INFO) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn EnterUmsSchedulingMode(schedulerstartupinfo: *const UMS_SCHEDULER_STARTUP_INFO) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(EnterUmsSchedulingMode(::core::mem::transmute(schedulerstartupinfo)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_Threading\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn ExecuteUmsThread(umsthread: *mut ::core::ffi::c_void) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn ExecuteUmsThread(umsthread: *mut ::core::ffi::c_void) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(ExecuteUmsThread(::core::mem::transmute(umsthread)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
#[inline]
pub unsafe fn ExitProcess(uexitcode: u32) -> ! {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn ExitProcess(uexitcode: u32) -> !;
        }
        ExitProcess(::core::mem::transmute(uexitcode))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
#[inline]
pub unsafe fn ExitThread(dwexitcode: u32) -> ! {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn ExitThread(dwexitcode: u32) -> !;
        }
        ExitThread(::core::mem::transmute(dwexitcode))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
#[inline]
pub unsafe fn FlsAlloc(lpcallback: PFLS_CALLBACK_FUNCTION) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn FlsAlloc(lpcallback: ::windows::core::RawPtr) -> u32;
        }
        ::core::mem::transmute(FlsAlloc(::core::mem::transmute(lpcallback)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_Threading\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn FlsFree(dwflsindex: u32) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn FlsFree(dwflsindex: u32) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(FlsFree(::core::mem::transmute(dwflsindex)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
#[inline]
pub unsafe fn FlsGetValue(dwflsindex: u32) -> *mut ::core::ffi::c_void {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn FlsGetValue(dwflsindex: u32) -> *mut ::core::ffi::c_void;
        }
        ::core::mem::transmute(FlsGetValue(::core::mem::transmute(dwflsindex)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_Threading\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn FlsSetValue(dwflsindex: u32, lpflsdata: *const ::core::ffi::c_void) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn FlsSetValue(dwflsindex: u32, lpflsdata: *const ::core::ffi::c_void) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(FlsSetValue(::core::mem::transmute(dwflsindex), ::core::mem::transmute(lpflsdata)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
#[inline]
pub unsafe fn FlushProcessWriteBuffers() {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn FlushProcessWriteBuffers();
        }
        FlushProcessWriteBuffers()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_Threading\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn FreeLibraryWhenCallbackReturns<'a, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::HINSTANCE>>(pci: *mut TP_CALLBACK_INSTANCE, r#mod: Param1) {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn FreeLibraryWhenCallbackReturns(pci: *mut TP_CALLBACK_INSTANCE, r#mod: super::super::Foundation::HINSTANCE);
        }
        FreeLibraryWhenCallbackReturns(::core::mem::transmute(pci), r#mod.into_param().abi())
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct GET_GUI_RESOURCES_FLAGS(pub u32);
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
pub const GR_GDIOBJECTS: GET_GUI_RESOURCES_FLAGS = GET_GUI_RESOURCES_FLAGS(0u32);
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
pub const GR_GDIOBJECTS_PEAK: GET_GUI_RESOURCES_FLAGS = GET_GUI_RESOURCES_FLAGS(2u32);
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
pub const GR_USEROBJECTS: GET_GUI_RESOURCES_FLAGS = GET_GUI_RESOURCES_FLAGS(1u32);
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
pub const GR_USEROBJECTS_PEAK: GET_GUI_RESOURCES_FLAGS = GET_GUI_RESOURCES_FLAGS(4u32);
impl ::core::marker::Copy for GET_GUI_RESOURCES_FLAGS {}
impl ::core::clone::Clone for GET_GUI_RESOURCES_FLAGS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for GET_GUI_RESOURCES_FLAGS {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for GET_GUI_RESOURCES_FLAGS {
    type Abi = Self;
}
impl ::core::fmt::Debug for GET_GUI_RESOURCES_FLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("GET_GUI_RESOURCES_FLAGS").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
#[inline]
pub unsafe fn GetActiveProcessorCount(groupnumber: u16) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn GetActiveProcessorCount(groupnumber: u16) -> u32;
        }
        ::core::mem::transmute(GetActiveProcessorCount(::core::mem::transmute(groupnumber)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
#[inline]
pub unsafe fn GetActiveProcessorGroupCount() -> u16 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn GetActiveProcessorGroupCount() -> u16;
        }
        ::core::mem::transmute(GetActiveProcessorGroupCount())
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_Threading\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn GetCurrentProcess() -> super::super::Foundation::HANDLE {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn GetCurrentProcess() -> super::super::Foundation::HANDLE;
        }
        ::core::mem::transmute(GetCurrentProcess())
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
#[inline]
pub unsafe fn GetCurrentProcessId() -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn GetCurrentProcessId() -> u32;
        }
        ::core::mem::transmute(GetCurrentProcessId())
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
#[inline]
pub unsafe fn GetCurrentProcessorNumber() -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn GetCurrentProcessorNumber() -> u32;
        }
        ::core::mem::transmute(GetCurrentProcessorNumber())
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_Threading\"`, `\"Win32_System_Kernel\"`*"]
#[cfg(feature = "Win32_System_Kernel")]
#[inline]
pub unsafe fn GetCurrentProcessorNumberEx(procnumber: *mut super::Kernel::PROCESSOR_NUMBER) {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn GetCurrentProcessorNumberEx(procnumber: *mut super::Kernel::PROCESSOR_NUMBER);
        }
        GetCurrentProcessorNumberEx(::core::mem::transmute(procnumber))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_Threading\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn GetCurrentThread() -> super::super::Foundation::HANDLE {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn GetCurrentThread() -> super::super::Foundation::HANDLE;
        }
        ::core::mem::transmute(GetCurrentThread())
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
#[inline]
pub unsafe fn GetCurrentThreadId() -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn GetCurrentThreadId() -> u32;
        }
        ::core::mem::transmute(GetCurrentThreadId())
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
#[inline]
pub unsafe fn GetCurrentThreadStackLimits(lowlimit: *mut usize, highlimit: *mut usize) {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn GetCurrentThreadStackLimits(lowlimit: *mut usize, highlimit: *mut usize);
        }
        GetCurrentThreadStackLimits(::core::mem::transmute(lowlimit), ::core::mem::transmute(highlimit))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
#[inline]
pub unsafe fn GetCurrentUmsThread() -> *mut ::core::ffi::c_void {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn GetCurrentUmsThread() -> *mut ::core::ffi::c_void;
        }
        ::core::mem::transmute(GetCurrentUmsThread())
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_Threading\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn GetExitCodeProcess<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::HANDLE>>(hprocess: Param0, lpexitcode: *mut u32) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn GetExitCodeProcess(hprocess: super::super::Foundation::HANDLE, lpexitcode: *mut u32) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(GetExitCodeProcess(hprocess.into_param().abi(), ::core::mem::transmute(lpexitcode)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_Threading\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn GetExitCodeThread<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::HANDLE>>(hthread: Param0, lpexitcode: *mut u32) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn GetExitCodeThread(hthread: super::super::Foundation::HANDLE, lpexitcode: *mut u32) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(GetExitCodeThread(hthread.into_param().abi(), ::core::mem::transmute(lpexitcode)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_Threading\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn GetGuiResources<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::HANDLE>>(hprocess: Param0, uiflags: GET_GUI_RESOURCES_FLAGS) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn GetGuiResources(hprocess: super::super::Foundation::HANDLE, uiflags: GET_GUI_RESOURCES_FLAGS) -> u32;
        }
        ::core::mem::transmute(GetGuiResources(hprocess.into_param().abi(), ::core::mem::transmute(uiflags)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
#[inline]
pub unsafe fn GetMachineTypeAttributes(machine: u16) -> ::windows::core::Result<MACHINE_ATTRIBUTES> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn GetMachineTypeAttributes(machine: u16, machinetypeattributes: *mut MACHINE_ATTRIBUTES) -> ::windows::core::HRESULT;
        }
        let mut result__: MACHINE_ATTRIBUTES = ::core::mem::zeroed();
        GetMachineTypeAttributes(::core::mem::transmute(machine), ::core::mem::transmute(&mut result__)).from_abi::<MACHINE_ATTRIBUTES>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
#[inline]
pub unsafe fn GetMaximumProcessorCount(groupnumber: u16) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn GetMaximumProcessorCount(groupnumber: u16) -> u32;
        }
        ::core::mem::transmute(GetMaximumProcessorCount(::core::mem::transmute(groupnumber)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
#[inline]
pub unsafe fn GetMaximumProcessorGroupCount() -> u16 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn GetMaximumProcessorGroupCount() -> u16;
        }
        ::core::mem::transmute(GetMaximumProcessorGroupCount())
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
#[inline]
pub unsafe fn GetNextUmsListItem(umscontext: *mut ::core::ffi::c_void) -> *mut ::core::ffi::c_void {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn GetNextUmsListItem(umscontext: *mut ::core::ffi::c_void) -> *mut ::core::ffi::c_void;
        }
        ::core::mem::transmute(GetNextUmsListItem(::core::mem::transmute(umscontext)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_Threading\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn GetNumaAvailableMemoryNode(node: u8, availablebytes: *mut u64) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn GetNumaAvailableMemoryNode(node: u8, availablebytes: *mut u64) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(GetNumaAvailableMemoryNode(::core::mem::transmute(node), ::core::mem::transmute(availablebytes)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_Threading\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn GetNumaAvailableMemoryNodeEx(node: u16, availablebytes: *mut u64) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn GetNumaAvailableMemoryNodeEx(node: u16, availablebytes: *mut u64) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(GetNumaAvailableMemoryNodeEx(::core::mem::transmute(node), ::core::mem::transmute(availablebytes)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_Threading\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn GetNumaHighestNodeNumber(highestnodenumber: *mut u32) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn GetNumaHighestNodeNumber(highestnodenumber: *mut u32) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(GetNumaHighestNodeNumber(::core::mem::transmute(highestnodenumber)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_Threading\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn GetNumaNodeNumberFromHandle<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::HANDLE>>(hfile: Param0, nodenumber: *mut u16) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn GetNumaNodeNumberFromHandle(hfile: super::super::Foundation::HANDLE, nodenumber: *mut u16) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(GetNumaNodeNumberFromHandle(hfile.into_param().abi(), ::core::mem::transmute(nodenumber)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_Threading\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn GetNumaNodeProcessorMask(node: u8, processormask: *mut u64) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn GetNumaNodeProcessorMask(node: u8, processormask: *mut u64) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(GetNumaNodeProcessorMask(::core::mem::transmute(node), ::core::mem::transmute(processormask)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_Threading\"`, `\"Win32_Foundation\"`, `\"Win32_System_SystemInformation\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_SystemInformation"))]
#[inline]
pub unsafe fn GetNumaNodeProcessorMask2(nodenumber: u16, processormasks: &mut [super::SystemInformation::GROUP_AFFINITY], requiredmaskcount: *mut u16) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn GetNumaNodeProcessorMask2(nodenumber: u16, processormasks: *mut super::SystemInformation::GROUP_AFFINITY, processormaskcount: u16, requiredmaskcount: *mut u16) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(GetNumaNodeProcessorMask2(::core::mem::transmute(nodenumber), ::core::mem::transmute(::windows::core::as_mut_ptr_or_null(processormasks)), processormasks.len() as _, ::core::mem::transmute(requiredmaskcount)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_Threading\"`, `\"Win32_Foundation\"`, `\"Win32_System_SystemInformation\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_SystemInformation"))]
#[inline]
pub unsafe fn GetNumaNodeProcessorMaskEx(node: u16, processormask: *mut super::SystemInformation::GROUP_AFFINITY) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn GetNumaNodeProcessorMaskEx(node: u16, processormask: *mut super::SystemInformation::GROUP_AFFINITY) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(GetNumaNodeProcessorMaskEx(::core::mem::transmute(node), ::core::mem::transmute(processormask)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_Threading\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn GetNumaProcessorNode(processor: u8, nodenumber: *mut u8) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn GetNumaProcessorNode(processor: u8, nodenumber: *mut u8) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(GetNumaProcessorNode(::core::mem::transmute(processor), ::core::mem::transmute(nodenumber)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_Threading\"`, `\"Win32_Foundation\"`, `\"Win32_System_Kernel\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Kernel"))]
#[inline]
pub unsafe fn GetNumaProcessorNodeEx(processor: *const super::Kernel::PROCESSOR_NUMBER, nodenumber: *mut u16) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn GetNumaProcessorNodeEx(processor: *const super::Kernel::PROCESSOR_NUMBER, nodenumber: *mut u16) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(GetNumaProcessorNodeEx(::core::mem::transmute(processor), ::core::mem::transmute(nodenumber)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_Threading\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn GetNumaProximityNode(proximityid: u32, nodenumber: *mut u8) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn GetNumaProximityNode(proximityid: u32, nodenumber: *mut u8) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(GetNumaProximityNode(::core::mem::transmute(proximityid), ::core::mem::transmute(nodenumber)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_Threading\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn GetNumaProximityNodeEx(proximityid: u32, nodenumber: *mut u16) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn GetNumaProximityNodeEx(proximityid: u32, nodenumber: *mut u16) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(GetNumaProximityNodeEx(::core::mem::transmute(proximityid), ::core::mem::transmute(nodenumber)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_Threading\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn GetPriorityClass<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::HANDLE>>(hprocess: Param0) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn GetPriorityClass(hprocess: super::super::Foundation::HANDLE) -> u32;
        }
        ::core::mem::transmute(GetPriorityClass(hprocess.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_Threading\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn GetProcessAffinityMask<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::HANDLE>>(hprocess: Param0, lpprocessaffinitymask: *mut usize, lpsystemaffinitymask: *mut usize) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn GetProcessAffinityMask(hprocess: super::super::Foundation::HANDLE, lpprocessaffinitymask: *mut usize, lpsystemaffinitymask: *mut usize) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(GetProcessAffinityMask(hprocess.into_param().abi(), ::core::mem::transmute(lpprocessaffinitymask), ::core::mem::transmute(lpsystemaffinitymask)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_Threading\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn GetProcessDEPPolicy<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::HANDLE>>(hprocess: Param0, lpflags: *mut u32, lppermanent: *mut super::super::Foundation::BOOL) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn GetProcessDEPPolicy(hprocess: super::super::Foundation::HANDLE, lpflags: *mut u32, lppermanent: *mut super::super::Foundation::BOOL) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(GetProcessDEPPolicy(hprocess.into_param().abi(), ::core::mem::transmute(lpflags), ::core::mem::transmute(lppermanent)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_Threading\"`, `\"Win32_Foundation\"`, `\"Win32_System_SystemInformation\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_SystemInformation"))]
#[inline]
pub unsafe fn GetProcessDefaultCpuSetMasks<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::HANDLE>>(process: Param0, cpusetmasks: &mut [super::SystemInformation::GROUP_AFFINITY], requiredmaskcount: *mut u16) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn GetProcessDefaultCpuSetMasks(process: super::super::Foundation::HANDLE, cpusetmasks: *mut super::SystemInformation::GROUP_AFFINITY, cpusetmaskcount: u16, requiredmaskcount: *mut u16) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(GetProcessDefaultCpuSetMasks(process.into_param().abi(), ::core::mem::transmute(::windows::core::as_mut_ptr_or_null(cpusetmasks)), cpusetmasks.len() as _, ::core::mem::transmute(requiredmaskcount)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_Threading\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn GetProcessDefaultCpuSets<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::HANDLE>>(process: Param0, cpusetids: &mut [u32], requiredidcount: *mut u32) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn GetProcessDefaultCpuSets(process: super::super::Foundation::HANDLE, cpusetids: *mut u32, cpusetidcount: u32, requiredidcount: *mut u32) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(GetProcessDefaultCpuSets(process.into_param().abi(), ::core::mem::transmute(::windows::core::as_mut_ptr_or_null(cpusetids)), cpusetids.len() as _, ::core::mem::transmute(requiredidcount)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_Threading\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn GetProcessGroupAffinity<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::HANDLE>>(hprocess: Param0, groupcount: *mut u16, grouparray: *mut u16) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn GetProcessGroupAffinity(hprocess: super::super::Foundation::HANDLE, groupcount: *mut u16, grouparray: *mut u16) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(GetProcessGroupAffinity(hprocess.into_param().abi(), ::core::mem::transmute(groupcount), ::core::mem::transmute(grouparray)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_Threading\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn GetProcessHandleCount<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::HANDLE>>(hprocess: Param0, pdwhandlecount: *mut u32) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn GetProcessHandleCount(hprocess: super::super::Foundation::HANDLE, pdwhandlecount: *mut u32) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(GetProcessHandleCount(hprocess.into_param().abi(), ::core::mem::transmute(pdwhandlecount)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_Threading\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn GetProcessId<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::HANDLE>>(process: Param0) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn GetProcessId(process: super::super::Foundation::HANDLE) -> u32;
        }
        ::core::mem::transmute(GetProcessId(process.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_Threading\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn GetProcessIdOfThread<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::HANDLE>>(thread: Param0) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn GetProcessIdOfThread(thread: super::super::Foundation::HANDLE) -> u32;
        }
        ::core::mem::transmute(GetProcessIdOfThread(thread.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_Threading\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn GetProcessInformation<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::HANDLE>>(hprocess: Param0, processinformationclass: PROCESS_INFORMATION_CLASS, processinformation: *mut ::core::ffi::c_void, processinformationsize: u32) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn GetProcessInformation(hprocess: super::super::Foundation::HANDLE, processinformationclass: PROCESS_INFORMATION_CLASS, processinformation: *mut ::core::ffi::c_void, processinformationsize: u32) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(GetProcessInformation(hprocess.into_param().abi(), ::core::mem::transmute(processinformationclass), ::core::mem::transmute(processinformation), ::core::mem::transmute(processinformationsize)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_Threading\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn GetProcessIoCounters<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::HANDLE>>(hprocess: Param0, lpiocounters: *mut IO_COUNTERS) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn GetProcessIoCounters(hprocess: super::super::Foundation::HANDLE, lpiocounters: *mut IO_COUNTERS) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(GetProcessIoCounters(hprocess.into_param().abi(), ::core::mem::transmute(lpiocounters)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_Threading\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn GetProcessMitigationPolicy<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::HANDLE>>(hprocess: Param0, mitigationpolicy: PROCESS_MITIGATION_POLICY, lpbuffer: *mut ::core::ffi::c_void, dwlength: usize) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn GetProcessMitigationPolicy(hprocess: super::super::Foundation::HANDLE, mitigationpolicy: PROCESS_MITIGATION_POLICY, lpbuffer: *mut ::core::ffi::c_void, dwlength: usize) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(GetProcessMitigationPolicy(hprocess.into_param().abi(), ::core::mem::transmute(mitigationpolicy), ::core::mem::transmute(lpbuffer), ::core::mem::transmute(dwlength)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_Threading\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn GetProcessPriorityBoost<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::HANDLE>>(hprocess: Param0, pdisablepriorityboost: *mut super::super::Foundation::BOOL) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn GetProcessPriorityBoost(hprocess: super::super::Foundation::HANDLE, pdisablepriorityboost: *mut super::super::Foundation::BOOL) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(GetProcessPriorityBoost(hprocess.into_param().abi(), ::core::mem::transmute(pdisablepriorityboost)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_Threading\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn GetProcessShutdownParameters(lpdwlevel: *mut u32, lpdwflags: *mut u32) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn GetProcessShutdownParameters(lpdwlevel: *mut u32, lpdwflags: *mut u32) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(GetProcessShutdownParameters(::core::mem::transmute(lpdwlevel), ::core::mem::transmute(lpdwflags)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_Threading\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn GetProcessTimes<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::HANDLE>>(hprocess: Param0, lpcreationtime: *mut super::super::Foundation::FILETIME, lpexittime: *mut super::super::Foundation::FILETIME, lpkerneltime: *mut super::super::Foundation::FILETIME, lpusertime: *mut super::super::Foundation::FILETIME) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn GetProcessTimes(hprocess: super::super::Foundation::HANDLE, lpcreationtime: *mut super::super::Foundation::FILETIME, lpexittime: *mut super::super::Foundation::FILETIME, lpkerneltime: *mut super::super::Foundation::FILETIME, lpusertime: *mut super::super::Foundation::FILETIME) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(GetProcessTimes(hprocess.into_param().abi(), ::core::mem::transmute(lpcreationtime), ::core::mem::transmute(lpexittime), ::core::mem::transmute(lpkerneltime), ::core::mem::transmute(lpusertime)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
#[inline]
pub unsafe fn GetProcessVersion(processid: u32) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn GetProcessVersion(processid: u32) -> u32;
        }
        ::core::mem::transmute(GetProcessVersion(::core::mem::transmute(processid)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_Threading\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn GetProcessWorkingSetSize<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::HANDLE>>(hprocess: Param0, lpminimumworkingsetsize: *mut usize, lpmaximumworkingsetsize: *mut usize) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn GetProcessWorkingSetSize(hprocess: super::super::Foundation::HANDLE, lpminimumworkingsetsize: *mut usize, lpmaximumworkingsetsize: *mut usize) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(GetProcessWorkingSetSize(hprocess.into_param().abi(), ::core::mem::transmute(lpminimumworkingsetsize), ::core::mem::transmute(lpmaximumworkingsetsize)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_Threading\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn GetStartupInfoA(lpstartupinfo: *mut STARTUPINFOA) {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn GetStartupInfoA(lpstartupinfo: *mut STARTUPINFOA);
        }
        GetStartupInfoA(::core::mem::transmute(lpstartupinfo))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_Threading\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn GetStartupInfoW(lpstartupinfo: *mut STARTUPINFOW) {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn GetStartupInfoW(lpstartupinfo: *mut STARTUPINFOW);
        }
        GetStartupInfoW(::core::mem::transmute(lpstartupinfo))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_Threading\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn GetSystemTimes(lpidletime: *mut super::super::Foundation::FILETIME, lpkerneltime: *mut super::super::Foundation::FILETIME, lpusertime: *mut super::super::Foundation::FILETIME) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn GetSystemTimes(lpidletime: *mut super::super::Foundation::FILETIME, lpkerneltime: *mut super::super::Foundation::FILETIME, lpusertime: *mut super::super::Foundation::FILETIME) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(GetSystemTimes(::core::mem::transmute(lpidletime), ::core::mem::transmute(lpkerneltime), ::core::mem::transmute(lpusertime)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_Threading\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn GetThreadDescription<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::HANDLE>>(hthread: Param0) -> ::windows::core::Result<::windows::core::PWSTR> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn GetThreadDescription(hthread: super::super::Foundation::HANDLE, ppszthreaddescription: *mut ::windows::core::PWSTR) -> ::windows::core::HRESULT;
        }
        let mut result__: ::windows::core::PWSTR = ::core::mem::zeroed();
        GetThreadDescription(hthread.into_param().abi(), ::core::mem::transmute(&mut result__)).from_abi::<::windows::core::PWSTR>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_Threading\"`, `\"Win32_Foundation\"`, `\"Win32_System_SystemInformation\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_SystemInformation"))]
#[inline]
pub unsafe fn GetThreadGroupAffinity<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::HANDLE>>(hthread: Param0, groupaffinity: *mut super::SystemInformation::GROUP_AFFINITY) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn GetThreadGroupAffinity(hthread: super::super::Foundation::HANDLE, groupaffinity: *mut super::SystemInformation::GROUP_AFFINITY) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(GetThreadGroupAffinity(hthread.into_param().abi(), ::core::mem::transmute(groupaffinity)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_Threading\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn GetThreadIOPendingFlag<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::HANDLE>>(hthread: Param0, lpioispending: *mut super::super::Foundation::BOOL) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn GetThreadIOPendingFlag(hthread: super::super::Foundation::HANDLE, lpioispending: *mut super::super::Foundation::BOOL) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(GetThreadIOPendingFlag(hthread.into_param().abi(), ::core::mem::transmute(lpioispending)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_Threading\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn GetThreadId<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::HANDLE>>(thread: Param0) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn GetThreadId(thread: super::super::Foundation::HANDLE) -> u32;
        }
        ::core::mem::transmute(GetThreadId(thread.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_Threading\"`, `\"Win32_Foundation\"`, `\"Win32_System_Kernel\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Kernel"))]
#[inline]
pub unsafe fn GetThreadIdealProcessorEx<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::HANDLE>>(hthread: Param0, lpidealprocessor: *mut super::Kernel::PROCESSOR_NUMBER) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn GetThreadIdealProcessorEx(hthread: super::super::Foundation::HANDLE, lpidealprocessor: *mut super::Kernel::PROCESSOR_NUMBER) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(GetThreadIdealProcessorEx(hthread.into_param().abi(), ::core::mem::transmute(lpidealprocessor)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_Threading\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn GetThreadInformation<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::HANDLE>>(hthread: Param0, threadinformationclass: THREAD_INFORMATION_CLASS, threadinformation: *mut ::core::ffi::c_void, threadinformationsize: u32) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn GetThreadInformation(hthread: super::super::Foundation::HANDLE, threadinformationclass: THREAD_INFORMATION_CLASS, threadinformation: *mut ::core::ffi::c_void, threadinformationsize: u32) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(GetThreadInformation(hthread.into_param().abi(), ::core::mem::transmute(threadinformationclass), ::core::mem::transmute(threadinformation), ::core::mem::transmute(threadinformationsize)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_Threading\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn GetThreadPriority<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::HANDLE>>(hthread: Param0) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn GetThreadPriority(hthread: super::super::Foundation::HANDLE) -> i32;
        }
        ::core::mem::transmute(GetThreadPriority(hthread.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_Threading\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn GetThreadPriorityBoost<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::HANDLE>>(hthread: Param0, pdisablepriorityboost: *mut super::super::Foundation::BOOL) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn GetThreadPriorityBoost(hthread: super::super::Foundation::HANDLE, pdisablepriorityboost: *mut super::super::Foundation::BOOL) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(GetThreadPriorityBoost(hthread.into_param().abi(), ::core::mem::transmute(pdisablepriorityboost)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_Threading\"`, `\"Win32_Foundation\"`, `\"Win32_System_SystemInformation\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_SystemInformation"))]
#[inline]
pub unsafe fn GetThreadSelectedCpuSetMasks<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::HANDLE>>(thread: Param0, cpusetmasks: &mut [super::SystemInformation::GROUP_AFFINITY], requiredmaskcount: *mut u16) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn GetThreadSelectedCpuSetMasks(thread: super::super::Foundation::HANDLE, cpusetmasks: *mut super::SystemInformation::GROUP_AFFINITY, cpusetmaskcount: u16, requiredmaskcount: *mut u16) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(GetThreadSelectedCpuSetMasks(thread.into_param().abi(), ::core::mem::transmute(::windows::core::as_mut_ptr_or_null(cpusetmasks)), cpusetmasks.len() as _, ::core::mem::transmute(requiredmaskcount)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_Threading\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn GetThreadSelectedCpuSets<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::HANDLE>>(thread: Param0, cpusetids: &mut [u32], requiredidcount: *mut u32) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn GetThreadSelectedCpuSets(thread: super::super::Foundation::HANDLE, cpusetids: *mut u32, cpusetidcount: u32, requiredidcount: *mut u32) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(GetThreadSelectedCpuSets(thread.into_param().abi(), ::core::mem::transmute(::windows::core::as_mut_ptr_or_null(cpusetids)), cpusetids.len() as _, ::core::mem::transmute(requiredidcount)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_Threading\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn GetThreadTimes<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::HANDLE>>(hthread: Param0, lpcreationtime: *mut super::super::Foundation::FILETIME, lpexittime: *mut super::super::Foundation::FILETIME, lpkerneltime: *mut super::super::Foundation::FILETIME, lpusertime: *mut super::super::Foundation::FILETIME) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn GetThreadTimes(hthread: super::super::Foundation::HANDLE, lpcreationtime: *mut super::super::Foundation::FILETIME, lpexittime: *mut super::super::Foundation::FILETIME, lpkerneltime: *mut super::super::Foundation::FILETIME, lpusertime: *mut super::super::Foundation::FILETIME) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(GetThreadTimes(hthread.into_param().abi(), ::core::mem::transmute(lpcreationtime), ::core::mem::transmute(lpexittime), ::core::mem::transmute(lpkerneltime), ::core::mem::transmute(lpusertime)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_Threading\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn GetUmsCompletionListEvent(umscompletionlist: *const ::core::ffi::c_void, umscompletionevent: *mut super::super::Foundation::HANDLE) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn GetUmsCompletionListEvent(umscompletionlist: *const ::core::ffi::c_void, umscompletionevent: *mut super::super::Foundation::HANDLE) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(GetUmsCompletionListEvent(::core::mem::transmute(umscompletionlist), ::core::mem::transmute(umscompletionevent)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_Threading\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn GetUmsSystemThreadInformation<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::HANDLE>>(threadhandle: Param0, systemthreadinfo: *mut UMS_SYSTEM_THREAD_INFORMATION) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn GetUmsSystemThreadInformation(threadhandle: super::super::Foundation::HANDLE, systemthreadinfo: *mut UMS_SYSTEM_THREAD_INFORMATION) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(GetUmsSystemThreadInformation(threadhandle.into_param().abi(), ::core::mem::transmute(systemthreadinfo)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
pub const INIT_ONCE_ASYNC: u32 = 2u32;
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
pub const INIT_ONCE_CHECK_ONLY: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
pub const INIT_ONCE_CTX_RESERVED_BITS: u32 = 2u32;
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
pub const INIT_ONCE_INIT_FAILED: u32 = 4u32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
pub struct IO_COUNTERS {
    pub ReadOperationCount: u64,
    pub WriteOperationCount: u64,
    pub OtherOperationCount: u64,
    pub ReadTransferCount: u64,
    pub WriteTransferCount: u64,
    pub OtherTransferCount: u64,
}
impl ::core::marker::Copy for IO_COUNTERS {}
impl ::core::clone::Clone for IO_COUNTERS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for IO_COUNTERS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("IO_COUNTERS").field("ReadOperationCount", &self.ReadOperationCount).field("WriteOperationCount", &self.WriteOperationCount).field("OtherOperationCount", &self.OtherOperationCount).field("ReadTransferCount", &self.ReadTransferCount).field("WriteTransferCount", &self.WriteTransferCount).field("OtherTransferCount", &self.OtherTransferCount).finish()
    }
}
unsafe impl ::windows::core::Abi for IO_COUNTERS {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for IO_COUNTERS {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<IO_COUNTERS>()) == 0 }
    }
}
impl ::core::cmp::Eq for IO_COUNTERS {}
impl ::core::default::Default for IO_COUNTERS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_System_Threading\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn InitOnceBeginInitialize(lpinitonce: *mut RTL_RUN_ONCE, dwflags: u32, fpending: *mut super::super::Foundation::BOOL, lpcontext: *mut *mut ::core::ffi::c_void) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn InitOnceBeginInitialize(lpinitonce: *mut RTL_RUN_ONCE, dwflags: u32, fpending: *mut super::super::Foundation::BOOL, lpcontext: *mut *mut ::core::ffi::c_void) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(InitOnceBeginInitialize(::core::mem::transmute(lpinitonce), ::core::mem::transmute(dwflags), ::core::mem::transmute(fpending), ::core::mem::transmute(lpcontext)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_Threading\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn InitOnceComplete(lpinitonce: *mut RTL_RUN_ONCE, dwflags: u32, lpcontext: *const ::core::ffi::c_void) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn InitOnceComplete(lpinitonce: *mut RTL_RUN_ONCE, dwflags: u32, lpcontext: *const ::core::ffi::c_void) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(InitOnceComplete(::core::mem::transmute(lpinitonce), ::core::mem::transmute(dwflags), ::core::mem::transmute(lpcontext)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_Threading\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn InitOnceExecuteOnce(initonce: *mut RTL_RUN_ONCE, initfn: PINIT_ONCE_FN, parameter: *mut ::core::ffi::c_void, context: *mut *mut ::core::ffi::c_void) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn InitOnceExecuteOnce(initonce: *mut RTL_RUN_ONCE, initfn: ::windows::core::RawPtr, parameter: *mut ::core::ffi::c_void, context: *mut *mut ::core::ffi::c_void) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(InitOnceExecuteOnce(::core::mem::transmute(initonce), ::core::mem::transmute(initfn), ::core::mem::transmute(parameter), ::core::mem::transmute(context)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
#[inline]
pub unsafe fn InitOnceInitialize(initonce: *mut RTL_RUN_ONCE) {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn InitOnceInitialize(initonce: *mut RTL_RUN_ONCE);
        }
        InitOnceInitialize(::core::mem::transmute(initonce))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
#[inline]
pub unsafe fn InitializeConditionVariable(conditionvariable: *mut RTL_CONDITION_VARIABLE) {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn InitializeConditionVariable(conditionvariable: *mut RTL_CONDITION_VARIABLE);
        }
        InitializeConditionVariable(::core::mem::transmute(conditionvariable))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_Threading\"`, `\"Win32_Foundation\"`, `\"Win32_System_Kernel\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Kernel"))]
#[inline]
pub unsafe fn InitializeCriticalSection(lpcriticalsection: *mut RTL_CRITICAL_SECTION) {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn InitializeCriticalSection(lpcriticalsection: *mut RTL_CRITICAL_SECTION);
        }
        InitializeCriticalSection(::core::mem::transmute(lpcriticalsection))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_Threading\"`, `\"Win32_Foundation\"`, `\"Win32_System_Kernel\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Kernel"))]
#[inline]
pub unsafe fn InitializeCriticalSectionAndSpinCount(lpcriticalsection: *mut RTL_CRITICAL_SECTION, dwspincount: u32) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn InitializeCriticalSectionAndSpinCount(lpcriticalsection: *mut RTL_CRITICAL_SECTION, dwspincount: u32) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(InitializeCriticalSectionAndSpinCount(::core::mem::transmute(lpcriticalsection), ::core::mem::transmute(dwspincount)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_Threading\"`, `\"Win32_Foundation\"`, `\"Win32_System_Kernel\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Kernel"))]
#[inline]
pub unsafe fn InitializeCriticalSectionEx(lpcriticalsection: *mut RTL_CRITICAL_SECTION, dwspincount: u32, flags: u32) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn InitializeCriticalSectionEx(lpcriticalsection: *mut RTL_CRITICAL_SECTION, dwspincount: u32, flags: u32) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(InitializeCriticalSectionEx(::core::mem::transmute(lpcriticalsection), ::core::mem::transmute(dwspincount), ::core::mem::transmute(flags)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_Threading\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn InitializeProcThreadAttributeList(lpattributelist: LPPROC_THREAD_ATTRIBUTE_LIST, dwattributecount: u32, dwflags: u32, lpsize: *mut usize) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn InitializeProcThreadAttributeList(lpattributelist: LPPROC_THREAD_ATTRIBUTE_LIST, dwattributecount: u32, dwflags: u32, lpsize: *mut usize) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(InitializeProcThreadAttributeList(::core::mem::transmute(lpattributelist), ::core::mem::transmute(dwattributecount), ::core::mem::transmute(dwflags), ::core::mem::transmute(lpsize)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_Threading\"`, `\"Win32_System_Kernel\"`*"]
#[cfg(feature = "Win32_System_Kernel")]
#[inline]
pub unsafe fn InitializeSListHead(listhead: *mut super::Kernel::SLIST_HEADER) {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn InitializeSListHead(listhead: *mut super::Kernel::SLIST_HEADER);
        }
        InitializeSListHead(::core::mem::transmute(listhead))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
#[inline]
pub unsafe fn InitializeSRWLock(srwlock: *mut RTL_SRWLOCK) {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn InitializeSRWLock(srwlock: *mut RTL_SRWLOCK);
        }
        InitializeSRWLock(::core::mem::transmute(srwlock))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_Threading\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn InitializeSynchronizationBarrier(lpbarrier: *mut RTL_BARRIER, ltotalthreads: i32, lspincount: i32) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn InitializeSynchronizationBarrier(lpbarrier: *mut RTL_BARRIER, ltotalthreads: i32, lspincount: i32) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(InitializeSynchronizationBarrier(::core::mem::transmute(lpbarrier), ::core::mem::transmute(ltotalthreads), ::core::mem::transmute(lspincount)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_Threading\"`, `\"Win32_System_Kernel\"`*"]
#[cfg(feature = "Win32_System_Kernel")]
#[inline]
pub unsafe fn InterlockedFlushSList(listhead: *mut super::Kernel::SLIST_HEADER) -> *mut super::Kernel::SLIST_ENTRY {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn InterlockedFlushSList(listhead: *mut super::Kernel::SLIST_HEADER) -> *mut super::Kernel::SLIST_ENTRY;
        }
        ::core::mem::transmute(InterlockedFlushSList(::core::mem::transmute(listhead)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_Threading\"`, `\"Win32_System_Kernel\"`*"]
#[cfg(feature = "Win32_System_Kernel")]
#[inline]
pub unsafe fn InterlockedPopEntrySList(listhead: *mut super::Kernel::SLIST_HEADER) -> *mut super::Kernel::SLIST_ENTRY {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn InterlockedPopEntrySList(listhead: *mut super::Kernel::SLIST_HEADER) -> *mut super::Kernel::SLIST_ENTRY;
        }
        ::core::mem::transmute(InterlockedPopEntrySList(::core::mem::transmute(listhead)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_Threading\"`, `\"Win32_System_Kernel\"`*"]
#[cfg(feature = "Win32_System_Kernel")]
#[inline]
pub unsafe fn InterlockedPushEntrySList(listhead: *mut super::Kernel::SLIST_HEADER, listentry: *mut super::Kernel::SLIST_ENTRY) -> *mut super::Kernel::SLIST_ENTRY {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn InterlockedPushEntrySList(listhead: *mut super::Kernel::SLIST_HEADER, listentry: *mut super::Kernel::SLIST_ENTRY) -> *mut super::Kernel::SLIST_ENTRY;
        }
        ::core::mem::transmute(InterlockedPushEntrySList(::core::mem::transmute(listhead), ::core::mem::transmute(listentry)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_Threading\"`, `\"Win32_System_Kernel\"`*"]
#[cfg(feature = "Win32_System_Kernel")]
#[inline]
pub unsafe fn InterlockedPushListSListEx(listhead: *mut super::Kernel::SLIST_HEADER, list: *mut super::Kernel::SLIST_ENTRY, listend: *mut super::Kernel::SLIST_ENTRY, count: u32) -> *mut super::Kernel::SLIST_ENTRY {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn InterlockedPushListSListEx(listhead: *mut super::Kernel::SLIST_HEADER, list: *mut super::Kernel::SLIST_ENTRY, listend: *mut super::Kernel::SLIST_ENTRY, count: u32) -> *mut super::Kernel::SLIST_ENTRY;
        }
        ::core::mem::transmute(InterlockedPushListSListEx(::core::mem::transmute(listhead), ::core::mem::transmute(list), ::core::mem::transmute(listend), ::core::mem::transmute(count)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_Threading\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn IsImmersiveProcess<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::HANDLE>>(hprocess: Param0) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn IsImmersiveProcess(hprocess: super::super::Foundation::HANDLE) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(IsImmersiveProcess(hprocess.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_Threading\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn IsProcessCritical<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::HANDLE>>(hprocess: Param0, critical: *mut super::super::Foundation::BOOL) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn IsProcessCritical(hprocess: super::super::Foundation::HANDLE, critical: *mut super::super::Foundation::BOOL) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(IsProcessCritical(hprocess.into_param().abi(), ::core::mem::transmute(critical)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_Threading\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn IsProcessorFeaturePresent(processorfeature: PROCESSOR_FEATURE_ID) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn IsProcessorFeaturePresent(processorfeature: PROCESSOR_FEATURE_ID) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(IsProcessorFeaturePresent(::core::mem::transmute(processorfeature)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_Threading\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn IsThreadAFiber() -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn IsThreadAFiber() -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(IsThreadAFiber())
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_Threading\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn IsThreadpoolTimerSet(pti: *mut TP_TIMER) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn IsThreadpoolTimerSet(pti: *mut TP_TIMER) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(IsThreadpoolTimerSet(::core::mem::transmute(pti)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_Threading\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn IsWow64Process<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::HANDLE>>(hprocess: Param0, wow64process: *mut super::super::Foundation::BOOL) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn IsWow64Process(hprocess: super::super::Foundation::HANDLE, wow64process: *mut super::super::Foundation::BOOL) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(IsWow64Process(hprocess.into_param().abi(), ::core::mem::transmute(wow64process)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_Threading\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn IsWow64Process2<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::HANDLE>>(hprocess: Param0, pprocessmachine: *mut u16, pnativemachine: *mut u16) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn IsWow64Process2(hprocess: super::super::Foundation::HANDLE, pprocessmachine: *mut u16, pnativemachine: *mut u16) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(IsWow64Process2(hprocess.into_param().abi(), ::core::mem::transmute(pprocessmachine), ::core::mem::transmute(pnativemachine)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
pub type LPFIBER_START_ROUTINE = ::core::option::Option<unsafe extern "system" fn(lpfiberparameter: *mut ::core::ffi::c_void)>;
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct LPPROC_THREAD_ATTRIBUTE_LIST(pub *mut ::core::ffi::c_void);
impl LPPROC_THREAD_ATTRIBUTE_LIST {
    pub fn is_invalid(&self) -> bool {
        *self == unsafe { ::core::mem::zeroed() }
    }
    pub fn ok(self) -> ::windows::core::Result<Self> {
        if !self.is_invalid() {
            Ok(self)
        } else {
            Err(::windows::core::Error::from_win32())
        }
    }
}
impl ::core::default::Default for LPPROC_THREAD_ATTRIBUTE_LIST {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::clone::Clone for LPPROC_THREAD_ATTRIBUTE_LIST {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::marker::Copy for LPPROC_THREAD_ATTRIBUTE_LIST {}
impl ::core::fmt::Debug for LPPROC_THREAD_ATTRIBUTE_LIST {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("LPPROC_THREAD_ATTRIBUTE_LIST").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Abi for LPPROC_THREAD_ATTRIBUTE_LIST {
    type Abi = Self;
}
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
pub type LPTHREAD_START_ROUTINE = ::core::option::Option<unsafe extern "system" fn(lpthreadparameter: *mut ::core::ffi::c_void) -> u32>;
#[doc = "*Required features: `\"Win32_System_Threading\"`, `\"Win32_Foundation\"`, `\"Win32_System_Kernel\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Kernel"))]
#[inline]
pub unsafe fn LeaveCriticalSection(lpcriticalsection: *mut RTL_CRITICAL_SECTION) {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn LeaveCriticalSection(lpcriticalsection: *mut RTL_CRITICAL_SECTION);
        }
        LeaveCriticalSection(::core::mem::transmute(lpcriticalsection))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_Threading\"`, `\"Win32_Foundation\"`, `\"Win32_System_Kernel\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Kernel"))]
#[inline]
pub unsafe fn LeaveCriticalSectionWhenCallbackReturns(pci: *mut TP_CALLBACK_INSTANCE, pcs: *mut RTL_CRITICAL_SECTION) {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn LeaveCriticalSectionWhenCallbackReturns(pci: *mut TP_CALLBACK_INSTANCE, pcs: *mut RTL_CRITICAL_SECTION);
        }
        LeaveCriticalSectionWhenCallbackReturns(::core::mem::transmute(pci), ::core::mem::transmute(pcs))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct MACHINE_ATTRIBUTES(pub u32);
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
pub const UserEnabled: MACHINE_ATTRIBUTES = MACHINE_ATTRIBUTES(1u32);
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
pub const KernelEnabled: MACHINE_ATTRIBUTES = MACHINE_ATTRIBUTES(2u32);
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
pub const Wow64Container: MACHINE_ATTRIBUTES = MACHINE_ATTRIBUTES(4u32);
impl ::core::marker::Copy for MACHINE_ATTRIBUTES {}
impl ::core::clone::Clone for MACHINE_ATTRIBUTES {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for MACHINE_ATTRIBUTES {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for MACHINE_ATTRIBUTES {
    type Abi = Self;
}
impl ::core::fmt::Debug for MACHINE_ATTRIBUTES {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MACHINE_ATTRIBUTES").field(&self.0).finish()
    }
}
impl ::core::ops::BitOr for MACHINE_ATTRIBUTES {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for MACHINE_ATTRIBUTES {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for MACHINE_ATTRIBUTES {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for MACHINE_ATTRIBUTES {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for MACHINE_ATTRIBUTES {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct MEMORY_PRIORITY(pub u32);
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
pub const MEMORY_PRIORITY_VERY_LOW: MEMORY_PRIORITY = MEMORY_PRIORITY(1u32);
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
pub const MEMORY_PRIORITY_LOW: MEMORY_PRIORITY = MEMORY_PRIORITY(2u32);
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
pub const MEMORY_PRIORITY_MEDIUM: MEMORY_PRIORITY = MEMORY_PRIORITY(3u32);
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
pub const MEMORY_PRIORITY_BELOW_NORMAL: MEMORY_PRIORITY = MEMORY_PRIORITY(4u32);
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
pub const MEMORY_PRIORITY_NORMAL: MEMORY_PRIORITY = MEMORY_PRIORITY(5u32);
impl ::core::marker::Copy for MEMORY_PRIORITY {}
impl ::core::clone::Clone for MEMORY_PRIORITY {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for MEMORY_PRIORITY {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for MEMORY_PRIORITY {
    type Abi = Self;
}
impl ::core::fmt::Debug for MEMORY_PRIORITY {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MEMORY_PRIORITY").field(&self.0).finish()
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
pub struct MEMORY_PRIORITY_INFORMATION {
    pub MemoryPriority: MEMORY_PRIORITY,
}
impl ::core::marker::Copy for MEMORY_PRIORITY_INFORMATION {}
impl ::core::clone::Clone for MEMORY_PRIORITY_INFORMATION {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for MEMORY_PRIORITY_INFORMATION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("MEMORY_PRIORITY_INFORMATION").field("MemoryPriority", &self.MemoryPriority).finish()
    }
}
unsafe impl ::windows::core::Abi for MEMORY_PRIORITY_INFORMATION {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for MEMORY_PRIORITY_INFORMATION {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<MEMORY_PRIORITY_INFORMATION>()) == 0 }
    }
}
impl ::core::cmp::Eq for MEMORY_PRIORITY_INFORMATION {}
impl ::core::default::Default for MEMORY_PRIORITY_INFORMATION {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
pub const MUTEX_MODIFY_STATE: u32 = 1u32;
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct NamespaceHandle(pub isize);
impl NamespaceHandle {
    pub fn is_invalid(&self) -> bool {
        *self == unsafe { ::core::mem::zeroed() }
    }
    pub fn ok(self) -> ::windows::core::Result<Self> {
        if !self.is_invalid() {
            Ok(self)
        } else {
            Err(::windows::core::Error::from_win32())
        }
    }
}
impl ::core::default::Default for NamespaceHandle {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::clone::Clone for NamespaceHandle {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::marker::Copy for NamespaceHandle {}
impl ::core::fmt::Debug for NamespaceHandle {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("NamespaceHandle").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Abi for NamespaceHandle {
    type Abi = Self;
}
#[doc = "*Required features: `\"Win32_System_Threading\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn NtQueryInformationProcess<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::HANDLE>>(processhandle: Param0, processinformationclass: PROCESSINFOCLASS, processinformation: *mut ::core::ffi::c_void, processinformationlength: u32, returnlength: *mut u32) -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn NtQueryInformationProcess(processhandle: super::super::Foundation::HANDLE, processinformationclass: PROCESSINFOCLASS, processinformation: *mut ::core::ffi::c_void, processinformationlength: u32, returnlength: *mut u32) -> super::super::Foundation::NTSTATUS;
        }
        NtQueryInformationProcess(processhandle.into_param().abi(), ::core::mem::transmute(processinformationclass), ::core::mem::transmute(processinformation), ::core::mem::transmute(processinformationlength), ::core::mem::transmute(returnlength)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_Threading\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn NtQueryInformationThread<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::HANDLE>>(threadhandle: Param0, threadinformationclass: THREADINFOCLASS, threadinformation: *mut ::core::ffi::c_void, threadinformationlength: u32, returnlength: *mut u32) -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn NtQueryInformationThread(threadhandle: super::super::Foundation::HANDLE, threadinformationclass: THREADINFOCLASS, threadinformation: *mut ::core::ffi::c_void, threadinformationlength: u32, returnlength: *mut u32) -> super::super::Foundation::NTSTATUS;
        }
        NtQueryInformationThread(threadhandle.into_param().abi(), ::core::mem::transmute(threadinformationclass), ::core::mem::transmute(threadinformation), ::core::mem::transmute(threadinformationlength), ::core::mem::transmute(returnlength)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_Threading\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn NtSetInformationThread<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::HANDLE>>(threadhandle: Param0, threadinformationclass: THREADINFOCLASS, threadinformation: *const ::core::ffi::c_void, threadinformationlength: u32) -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn NtSetInformationThread(threadhandle: super::super::Foundation::HANDLE, threadinformationclass: THREADINFOCLASS, threadinformation: *const ::core::ffi::c_void, threadinformationlength: u32) -> super::super::Foundation::NTSTATUS;
        }
        NtSetInformationThread(threadhandle.into_param().abi(), ::core::mem::transmute(threadinformationclass), ::core::mem::transmute(threadinformation), ::core::mem::transmute(threadinformationlength)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_Threading\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn OpenEventA<'a, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::BOOL>, Param2: ::windows::core::IntoParam<'a, ::windows::core::PCSTR>>(dwdesiredaccess: u32, binherithandle: Param1, lpname: Param2) -> super::super::Foundation::HANDLE {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn OpenEventA(dwdesiredaccess: u32, binherithandle: super::super::Foundation::BOOL, lpname: ::windows::core::PCSTR) -> super::super::Foundation::HANDLE;
        }
        ::core::mem::transmute(OpenEventA(::core::mem::transmute(dwdesiredaccess), binherithandle.into_param().abi(), lpname.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_Threading\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn OpenEventW<'a, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::BOOL>, Param2: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>>(dwdesiredaccess: u32, binherithandle: Param1, lpname: Param2) -> super::super::Foundation::HANDLE {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn OpenEventW(dwdesiredaccess: u32, binherithandle: super::super::Foundation::BOOL, lpname: ::windows::core::PCWSTR) -> super::super::Foundation::HANDLE;
        }
        ::core::mem::transmute(OpenEventW(::core::mem::transmute(dwdesiredaccess), binherithandle.into_param().abi(), lpname.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_Threading\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn OpenMutexW<'a, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::BOOL>, Param2: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>>(dwdesiredaccess: u32, binherithandle: Param1, lpname: Param2) -> super::super::Foundation::HANDLE {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn OpenMutexW(dwdesiredaccess: u32, binherithandle: super::super::Foundation::BOOL, lpname: ::windows::core::PCWSTR) -> super::super::Foundation::HANDLE;
        }
        ::core::mem::transmute(OpenMutexW(::core::mem::transmute(dwdesiredaccess), binherithandle.into_param().abi(), lpname.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
#[inline]
pub unsafe fn OpenPrivateNamespaceA<'a, Param1: ::windows::core::IntoParam<'a, ::windows::core::PCSTR>>(lpboundarydescriptor: *const ::core::ffi::c_void, lpaliasprefix: Param1) -> NamespaceHandle {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn OpenPrivateNamespaceA(lpboundarydescriptor: *const ::core::ffi::c_void, lpaliasprefix: ::windows::core::PCSTR) -> NamespaceHandle;
        }
        ::core::mem::transmute(OpenPrivateNamespaceA(::core::mem::transmute(lpboundarydescriptor), lpaliasprefix.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
#[inline]
pub unsafe fn OpenPrivateNamespaceW<'a, Param1: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>>(lpboundarydescriptor: *const ::core::ffi::c_void, lpaliasprefix: Param1) -> NamespaceHandle {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn OpenPrivateNamespaceW(lpboundarydescriptor: *const ::core::ffi::c_void, lpaliasprefix: ::windows::core::PCWSTR) -> NamespaceHandle;
        }
        ::core::mem::transmute(OpenPrivateNamespaceW(::core::mem::transmute(lpboundarydescriptor), lpaliasprefix.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_Threading\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn OpenProcess<'a, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::BOOL>>(dwdesiredaccess: PROCESS_ACCESS_RIGHTS, binherithandle: Param1, dwprocessid: u32) -> super::super::Foundation::HANDLE {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn OpenProcess(dwdesiredaccess: PROCESS_ACCESS_RIGHTS, binherithandle: super::super::Foundation::BOOL, dwprocessid: u32) -> super::super::Foundation::HANDLE;
        }
        ::core::mem::transmute(OpenProcess(::core::mem::transmute(dwdesiredaccess), binherithandle.into_param().abi(), ::core::mem::transmute(dwprocessid)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_Threading\"`, `\"Win32_Foundation\"`, `\"Win32_Security\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
#[inline]
pub unsafe fn OpenProcessToken<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::HANDLE>>(processhandle: Param0, desiredaccess: super::super::Security::TOKEN_ACCESS_MASK, tokenhandle: *mut super::super::Foundation::HANDLE) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn OpenProcessToken(processhandle: super::super::Foundation::HANDLE, desiredaccess: super::super::Security::TOKEN_ACCESS_MASK, tokenhandle: *mut super::super::Foundation::HANDLE) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(OpenProcessToken(processhandle.into_param().abi(), ::core::mem::transmute(desiredaccess), ::core::mem::transmute(tokenhandle)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_Threading\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn OpenSemaphoreW<'a, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::BOOL>, Param2: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>>(dwdesiredaccess: u32, binherithandle: Param1, lpname: Param2) -> super::super::Foundation::HANDLE {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn OpenSemaphoreW(dwdesiredaccess: u32, binherithandle: super::super::Foundation::BOOL, lpname: ::windows::core::PCWSTR) -> super::super::Foundation::HANDLE;
        }
        ::core::mem::transmute(OpenSemaphoreW(::core::mem::transmute(dwdesiredaccess), binherithandle.into_param().abi(), lpname.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_Threading\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn OpenThread<'a, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::BOOL>>(dwdesiredaccess: THREAD_ACCESS_RIGHTS, binherithandle: Param1, dwthreadid: u32) -> super::super::Foundation::HANDLE {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn OpenThread(dwdesiredaccess: THREAD_ACCESS_RIGHTS, binherithandle: super::super::Foundation::BOOL, dwthreadid: u32) -> super::super::Foundation::HANDLE;
        }
        ::core::mem::transmute(OpenThread(::core::mem::transmute(dwdesiredaccess), binherithandle.into_param().abi(), ::core::mem::transmute(dwthreadid)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_Threading\"`, `\"Win32_Foundation\"`, `\"Win32_Security\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
#[inline]
pub unsafe fn OpenThreadToken<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::HANDLE>, Param2: ::windows::core::IntoParam<'a, super::super::Foundation::BOOL>>(threadhandle: Param0, desiredaccess: super::super::Security::TOKEN_ACCESS_MASK, openasself: Param2, tokenhandle: *mut super::super::Foundation::HANDLE) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn OpenThreadToken(threadhandle: super::super::Foundation::HANDLE, desiredaccess: super::super::Security::TOKEN_ACCESS_MASK, openasself: super::super::Foundation::BOOL, tokenhandle: *mut super::super::Foundation::HANDLE) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(OpenThreadToken(threadhandle.into_param().abi(), ::core::mem::transmute(desiredaccess), openasself.into_param().abi(), ::core::mem::transmute(tokenhandle)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_Threading\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn OpenWaitableTimerW<'a, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::BOOL>, Param2: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>>(dwdesiredaccess: u32, binherithandle: Param1, lptimername: Param2) -> super::super::Foundation::HANDLE {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn OpenWaitableTimerW(dwdesiredaccess: u32, binherithandle: super::super::Foundation::BOOL, lptimername: ::windows::core::PCWSTR) -> super::super::Foundation::HANDLE;
        }
        ::core::mem::transmute(OpenWaitableTimerW(::core::mem::transmute(dwdesiredaccess), binherithandle.into_param().abi(), lptimername.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Threading\"`, `\"Win32_Foundation\"`, `\"Win32_System_Kernel\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Kernel"))]
pub struct PEB {
    pub Reserved1: [u8; 2],
    pub BeingDebugged: u8,
    pub Reserved2: [u8; 1],
    pub Reserved3: [*mut ::core::ffi::c_void; 2],
    pub Ldr: *mut PEB_LDR_DATA,
    pub ProcessParameters: *mut RTL_USER_PROCESS_PARAMETERS,
    pub Reserved4: [*mut ::core::ffi::c_void; 3],
    pub AtlThunkSListPtr: *mut ::core::ffi::c_void,
    pub Reserved5: *mut ::core::ffi::c_void,
    pub Reserved6: u32,
    pub Reserved7: *mut ::core::ffi::c_void,
    pub Reserved8: u32,
    pub AtlThunkSListPtr32: u32,
    pub Reserved9: [*mut ::core::ffi::c_void; 45],
    pub Reserved10: [u8; 96],
    pub PostProcessInitRoutine: PPS_POST_PROCESS_INIT_ROUTINE,
    pub Reserved11: [u8; 128],
    pub Reserved12: [*mut ::core::ffi::c_void; 1],
    pub SessionId: u32,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Kernel"))]
impl ::core::marker::Copy for PEB {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Kernel"))]
impl ::core::clone::Clone for PEB {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Kernel"))]
impl ::core::fmt::Debug for PEB {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("PEB")
            .field("Reserved1", &self.Reserved1)
            .field("BeingDebugged", &self.BeingDebugged)
            .field("Reserved2", &self.Reserved2)
            .field("Reserved3", &self.Reserved3)
            .field("Ldr", &self.Ldr)
            .field("ProcessParameters", &self.ProcessParameters)
            .field("Reserved4", &self.Reserved4)
            .field("AtlThunkSListPtr", &self.AtlThunkSListPtr)
            .field("Reserved5", &self.Reserved5)
            .field("Reserved6", &self.Reserved6)
            .field("Reserved7", &self.Reserved7)
            .field("Reserved8", &self.Reserved8)
            .field("AtlThunkSListPtr32", &self.AtlThunkSListPtr32)
            .field("Reserved9", &self.Reserved9)
            .field("Reserved10", &self.Reserved10)
            .field("PostProcessInitRoutine", &self.PostProcessInitRoutine.map(|f| f as usize))
            .field("Reserved11", &self.Reserved11)
            .field("Reserved12", &self.Reserved12)
            .field("SessionId", &self.SessionId)
            .finish()
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Kernel"))]
unsafe impl ::windows::core::Abi for PEB {
    type Abi = Self;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Kernel"))]
impl ::core::cmp::PartialEq for PEB {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<PEB>()) == 0 }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Kernel"))]
impl ::core::cmp::Eq for PEB {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Kernel"))]
impl ::core::default::Default for PEB {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Threading\"`, `\"Win32_System_Kernel\"`*"]
#[cfg(feature = "Win32_System_Kernel")]
pub struct PEB_LDR_DATA {
    pub Reserved1: [u8; 8],
    pub Reserved2: [*mut ::core::ffi::c_void; 3],
    pub InMemoryOrderModuleList: super::Kernel::LIST_ENTRY,
}
#[cfg(feature = "Win32_System_Kernel")]
impl ::core::marker::Copy for PEB_LDR_DATA {}
#[cfg(feature = "Win32_System_Kernel")]
impl ::core::clone::Clone for PEB_LDR_DATA {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_System_Kernel")]
impl ::core::fmt::Debug for PEB_LDR_DATA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("PEB_LDR_DATA").field("Reserved1", &self.Reserved1).field("Reserved2", &self.Reserved2).field("InMemoryOrderModuleList", &self.InMemoryOrderModuleList).finish()
    }
}
#[cfg(feature = "Win32_System_Kernel")]
unsafe impl ::windows::core::Abi for PEB_LDR_DATA {
    type Abi = Self;
}
#[cfg(feature = "Win32_System_Kernel")]
impl ::core::cmp::PartialEq for PEB_LDR_DATA {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<PEB_LDR_DATA>()) == 0 }
    }
}
#[cfg(feature = "Win32_System_Kernel")]
impl ::core::cmp::Eq for PEB_LDR_DATA {}
#[cfg(feature = "Win32_System_Kernel")]
impl ::core::default::Default for PEB_LDR_DATA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
pub type PFLS_CALLBACK_FUNCTION = ::core::option::Option<unsafe extern "system" fn(lpflsdata: *const ::core::ffi::c_void)>;
#[doc = "*Required features: `\"Win32_System_Threading\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub type PINIT_ONCE_FN = ::core::option::Option<unsafe extern "system" fn(initonce: *mut RTL_RUN_ONCE, parameter: *mut ::core::ffi::c_void, context: *mut *mut ::core::ffi::c_void) -> super::super::Foundation::BOOL>;
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
pub const PME_CURRENT_VERSION: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
pub const PME_FAILFAST_ON_COMMIT_FAIL_DISABLE: u32 = 0u32;
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
pub const PME_FAILFAST_ON_COMMIT_FAIL_ENABLE: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct POWER_REQUEST_CONTEXT_FLAGS(pub u32);
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
pub const POWER_REQUEST_CONTEXT_DETAILED_STRING: POWER_REQUEST_CONTEXT_FLAGS = POWER_REQUEST_CONTEXT_FLAGS(2u32);
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
pub const POWER_REQUEST_CONTEXT_SIMPLE_STRING: POWER_REQUEST_CONTEXT_FLAGS = POWER_REQUEST_CONTEXT_FLAGS(1u32);
impl ::core::marker::Copy for POWER_REQUEST_CONTEXT_FLAGS {}
impl ::core::clone::Clone for POWER_REQUEST_CONTEXT_FLAGS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for POWER_REQUEST_CONTEXT_FLAGS {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for POWER_REQUEST_CONTEXT_FLAGS {
    type Abi = Self;
}
impl ::core::fmt::Debug for POWER_REQUEST_CONTEXT_FLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("POWER_REQUEST_CONTEXT_FLAGS").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
pub type PPS_POST_PROCESS_INIT_ROUTINE = ::core::option::Option<unsafe extern "system" fn()>;
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
pub const PRIVATE_NAMESPACE_FLAG_DESTROY: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct PROCESSINFOCLASS(pub i32);
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
pub const ProcessBasicInformation: PROCESSINFOCLASS = PROCESSINFOCLASS(0i32);
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
pub const ProcessDebugPort: PROCESSINFOCLASS = PROCESSINFOCLASS(7i32);
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
pub const ProcessWow64Information: PROCESSINFOCLASS = PROCESSINFOCLASS(26i32);
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
pub const ProcessImageFileName: PROCESSINFOCLASS = PROCESSINFOCLASS(27i32);
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
pub const ProcessBreakOnTermination: PROCESSINFOCLASS = PROCESSINFOCLASS(29i32);
impl ::core::marker::Copy for PROCESSINFOCLASS {}
impl ::core::clone::Clone for PROCESSINFOCLASS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for PROCESSINFOCLASS {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for PROCESSINFOCLASS {
    type Abi = Self;
}
impl ::core::fmt::Debug for PROCESSINFOCLASS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PROCESSINFOCLASS").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct PROCESSOR_FEATURE_ID(pub u32);
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
pub const PF_ARM_64BIT_LOADSTORE_ATOMIC: PROCESSOR_FEATURE_ID = PROCESSOR_FEATURE_ID(25u32);
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
pub const PF_ARM_DIVIDE_INSTRUCTION_AVAILABLE: PROCESSOR_FEATURE_ID = PROCESSOR_FEATURE_ID(24u32);
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
pub const PF_ARM_EXTERNAL_CACHE_AVAILABLE: PROCESSOR_FEATURE_ID = PROCESSOR_FEATURE_ID(26u32);
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
pub const PF_ARM_FMAC_INSTRUCTIONS_AVAILABLE: PROCESSOR_FEATURE_ID = PROCESSOR_FEATURE_ID(27u32);
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
pub const PF_ARM_VFP_32_REGISTERS_AVAILABLE: PROCESSOR_FEATURE_ID = PROCESSOR_FEATURE_ID(18u32);
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
pub const PF_3DNOW_INSTRUCTIONS_AVAILABLE: PROCESSOR_FEATURE_ID = PROCESSOR_FEATURE_ID(7u32);
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
pub const PF_CHANNELS_ENABLED: PROCESSOR_FEATURE_ID = PROCESSOR_FEATURE_ID(16u32);
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
pub const PF_COMPARE_EXCHANGE_DOUBLE: PROCESSOR_FEATURE_ID = PROCESSOR_FEATURE_ID(2u32);
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
pub const PF_COMPARE_EXCHANGE128: PROCESSOR_FEATURE_ID = PROCESSOR_FEATURE_ID(14u32);
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
pub const PF_COMPARE64_EXCHANGE128: PROCESSOR_FEATURE_ID = PROCESSOR_FEATURE_ID(15u32);
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
pub const PF_FASTFAIL_AVAILABLE: PROCESSOR_FEATURE_ID = PROCESSOR_FEATURE_ID(23u32);
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
pub const PF_FLOATING_POINT_EMULATED: PROCESSOR_FEATURE_ID = PROCESSOR_FEATURE_ID(1u32);
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
pub const PF_FLOATING_POINT_PRECISION_ERRATA: PROCESSOR_FEATURE_ID = PROCESSOR_FEATURE_ID(0u32);
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
pub const PF_MMX_INSTRUCTIONS_AVAILABLE: PROCESSOR_FEATURE_ID = PROCESSOR_FEATURE_ID(3u32);
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
pub const PF_NX_ENABLED: PROCESSOR_FEATURE_ID = PROCESSOR_FEATURE_ID(12u32);
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
pub const PF_PAE_ENABLED: PROCESSOR_FEATURE_ID = PROCESSOR_FEATURE_ID(9u32);
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
pub const PF_RDTSC_INSTRUCTION_AVAILABLE: PROCESSOR_FEATURE_ID = PROCESSOR_FEATURE_ID(8u32);
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
pub const PF_RDWRFSGSBASE_AVAILABLE: PROCESSOR_FEATURE_ID = PROCESSOR_FEATURE_ID(22u32);
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
pub const PF_SECOND_LEVEL_ADDRESS_TRANSLATION: PROCESSOR_FEATURE_ID = PROCESSOR_FEATURE_ID(20u32);
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
pub const PF_SSE3_INSTRUCTIONS_AVAILABLE: PROCESSOR_FEATURE_ID = PROCESSOR_FEATURE_ID(13u32);
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
pub const PF_VIRT_FIRMWARE_ENABLED: PROCESSOR_FEATURE_ID = PROCESSOR_FEATURE_ID(21u32);
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
pub const PF_XMMI_INSTRUCTIONS_AVAILABLE: PROCESSOR_FEATURE_ID = PROCESSOR_FEATURE_ID(6u32);
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
pub const PF_XMMI64_INSTRUCTIONS_AVAILABLE: PROCESSOR_FEATURE_ID = PROCESSOR_FEATURE_ID(10u32);
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
pub const PF_XSAVE_ENABLED: PROCESSOR_FEATURE_ID = PROCESSOR_FEATURE_ID(17u32);
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
pub const PF_ARM_V8_INSTRUCTIONS_AVAILABLE: PROCESSOR_FEATURE_ID = PROCESSOR_FEATURE_ID(29u32);
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
pub const PF_ARM_V8_CRYPTO_INSTRUCTIONS_AVAILABLE: PROCESSOR_FEATURE_ID = PROCESSOR_FEATURE_ID(30u32);
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
pub const PF_ARM_V8_CRC32_INSTRUCTIONS_AVAILABLE: PROCESSOR_FEATURE_ID = PROCESSOR_FEATURE_ID(31u32);
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
pub const PF_ARM_V81_ATOMIC_INSTRUCTIONS_AVAILABLE: PROCESSOR_FEATURE_ID = PROCESSOR_FEATURE_ID(34u32);
impl ::core::marker::Copy for PROCESSOR_FEATURE_ID {}
impl ::core::clone::Clone for PROCESSOR_FEATURE_ID {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for PROCESSOR_FEATURE_ID {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for PROCESSOR_FEATURE_ID {
    type Abi = Self;
}
impl ::core::fmt::Debug for PROCESSOR_FEATURE_ID {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PROCESSOR_FEATURE_ID").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct PROCESS_ACCESS_RIGHTS(pub u32);
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
pub const PROCESS_TERMINATE: PROCESS_ACCESS_RIGHTS = PROCESS_ACCESS_RIGHTS(1u32);
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
pub const PROCESS_CREATE_THREAD: PROCESS_ACCESS_RIGHTS = PROCESS_ACCESS_RIGHTS(2u32);
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
pub const PROCESS_SET_SESSIONID: PROCESS_ACCESS_RIGHTS = PROCESS_ACCESS_RIGHTS(4u32);
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
pub const PROCESS_VM_OPERATION: PROCESS_ACCESS_RIGHTS = PROCESS_ACCESS_RIGHTS(8u32);
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
pub const PROCESS_VM_READ: PROCESS_ACCESS_RIGHTS = PROCESS_ACCESS_RIGHTS(16u32);
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
pub const PROCESS_VM_WRITE: PROCESS_ACCESS_RIGHTS = PROCESS_ACCESS_RIGHTS(32u32);
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
pub const PROCESS_DUP_HANDLE: PROCESS_ACCESS_RIGHTS = PROCESS_ACCESS_RIGHTS(64u32);
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
pub const PROCESS_CREATE_PROCESS: PROCESS_ACCESS_RIGHTS = PROCESS_ACCESS_RIGHTS(128u32);
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
pub const PROCESS_SET_QUOTA: PROCESS_ACCESS_RIGHTS = PROCESS_ACCESS_RIGHTS(256u32);
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
pub const PROCESS_SET_INFORMATION: PROCESS_ACCESS_RIGHTS = PROCESS_ACCESS_RIGHTS(512u32);
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
pub const PROCESS_QUERY_INFORMATION: PROCESS_ACCESS_RIGHTS = PROCESS_ACCESS_RIGHTS(1024u32);
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
pub const PROCESS_SUSPEND_RESUME: PROCESS_ACCESS_RIGHTS = PROCESS_ACCESS_RIGHTS(2048u32);
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
pub const PROCESS_QUERY_LIMITED_INFORMATION: PROCESS_ACCESS_RIGHTS = PROCESS_ACCESS_RIGHTS(4096u32);
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
pub const PROCESS_SET_LIMITED_INFORMATION: PROCESS_ACCESS_RIGHTS = PROCESS_ACCESS_RIGHTS(8192u32);
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
pub const PROCESS_ALL_ACCESS: PROCESS_ACCESS_RIGHTS = PROCESS_ACCESS_RIGHTS(2097151u32);
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
pub const PROCESS_DELETE: PROCESS_ACCESS_RIGHTS = PROCESS_ACCESS_RIGHTS(65536u32);
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
pub const PROCESS_READ_CONTROL: PROCESS_ACCESS_RIGHTS = PROCESS_ACCESS_RIGHTS(131072u32);
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
pub const PROCESS_WRITE_DAC: PROCESS_ACCESS_RIGHTS = PROCESS_ACCESS_RIGHTS(262144u32);
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
pub const PROCESS_WRITE_OWNER: PROCESS_ACCESS_RIGHTS = PROCESS_ACCESS_RIGHTS(524288u32);
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
pub const PROCESS_SYNCHRONIZE: PROCESS_ACCESS_RIGHTS = PROCESS_ACCESS_RIGHTS(1048576u32);
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
pub const PROCESS_STANDARD_RIGHTS_REQUIRED: PROCESS_ACCESS_RIGHTS = PROCESS_ACCESS_RIGHTS(983040u32);
impl ::core::marker::Copy for PROCESS_ACCESS_RIGHTS {}
impl ::core::clone::Clone for PROCESS_ACCESS_RIGHTS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for PROCESS_ACCESS_RIGHTS {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for PROCESS_ACCESS_RIGHTS {
    type Abi = Self;
}
impl ::core::fmt::Debug for PROCESS_ACCESS_RIGHTS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PROCESS_ACCESS_RIGHTS").field(&self.0).finish()
    }
}
impl ::core::ops::BitOr for PROCESS_ACCESS_RIGHTS {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for PROCESS_ACCESS_RIGHTS {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for PROCESS_ACCESS_RIGHTS {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for PROCESS_ACCESS_RIGHTS {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for PROCESS_ACCESS_RIGHTS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct PROCESS_AFFINITY_AUTO_UPDATE_FLAGS(pub u32);
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
pub const PROCESS_AFFINITY_DISABLE_AUTO_UPDATE: PROCESS_AFFINITY_AUTO_UPDATE_FLAGS = PROCESS_AFFINITY_AUTO_UPDATE_FLAGS(0u32);
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
pub const PROCESS_AFFINITY_ENABLE_AUTO_UPDATE: PROCESS_AFFINITY_AUTO_UPDATE_FLAGS = PROCESS_AFFINITY_AUTO_UPDATE_FLAGS(1u32);
impl ::core::marker::Copy for PROCESS_AFFINITY_AUTO_UPDATE_FLAGS {}
impl ::core::clone::Clone for PROCESS_AFFINITY_AUTO_UPDATE_FLAGS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for PROCESS_AFFINITY_AUTO_UPDATE_FLAGS {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for PROCESS_AFFINITY_AUTO_UPDATE_FLAGS {
    type Abi = Self;
}
impl ::core::fmt::Debug for PROCESS_AFFINITY_AUTO_UPDATE_FLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PROCESS_AFFINITY_AUTO_UPDATE_FLAGS").field(&self.0).finish()
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Threading\"`, `\"Win32_Foundation\"`, `\"Win32_System_Kernel\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Kernel"))]
pub struct PROCESS_BASIC_INFORMATION {
    pub Reserved1: *mut ::core::ffi::c_void,
    pub PebBaseAddress: *mut PEB,
    pub Reserved2: [*mut ::core::ffi::c_void; 2],
    pub UniqueProcessId: usize,
    pub Reserved3: *mut ::core::ffi::c_void,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Kernel"))]
impl ::core::marker::Copy for PROCESS_BASIC_INFORMATION {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Kernel"))]
impl ::core::clone::Clone for PROCESS_BASIC_INFORMATION {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Kernel"))]
impl ::core::fmt::Debug for PROCESS_BASIC_INFORMATION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("PROCESS_BASIC_INFORMATION").field("Reserved1", &self.Reserved1).field("PebBaseAddress", &self.PebBaseAddress).field("Reserved2", &self.Reserved2).field("UniqueProcessId", &self.UniqueProcessId).field("Reserved3", &self.Reserved3).finish()
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Kernel"))]
unsafe impl ::windows::core::Abi for PROCESS_BASIC_INFORMATION {
    type Abi = Self;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Kernel"))]
impl ::core::cmp::PartialEq for PROCESS_BASIC_INFORMATION {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<PROCESS_BASIC_INFORMATION>()) == 0 }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Kernel"))]
impl ::core::cmp::Eq for PROCESS_BASIC_INFORMATION {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Kernel"))]
impl ::core::default::Default for PROCESS_BASIC_INFORMATION {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct PROCESS_CREATION_FLAGS(pub u32);
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
pub const DEBUG_PROCESS: PROCESS_CREATION_FLAGS = PROCESS_CREATION_FLAGS(1u32);
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
pub const DEBUG_ONLY_THIS_PROCESS: PROCESS_CREATION_FLAGS = PROCESS_CREATION_FLAGS(2u32);
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
pub const CREATE_SUSPENDED: PROCESS_CREATION_FLAGS = PROCESS_CREATION_FLAGS(4u32);
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
pub const DETACHED_PROCESS: PROCESS_CREATION_FLAGS = PROCESS_CREATION_FLAGS(8u32);
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
pub const CREATE_NEW_CONSOLE: PROCESS_CREATION_FLAGS = PROCESS_CREATION_FLAGS(16u32);
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
pub const NORMAL_PRIORITY_CLASS: PROCESS_CREATION_FLAGS = PROCESS_CREATION_FLAGS(32u32);
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
pub const IDLE_PRIORITY_CLASS: PROCESS_CREATION_FLAGS = PROCESS_CREATION_FLAGS(64u32);
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
pub const HIGH_PRIORITY_CLASS: PROCESS_CREATION_FLAGS = PROCESS_CREATION_FLAGS(128u32);
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
pub const REALTIME_PRIORITY_CLASS: PROCESS_CREATION_FLAGS = PROCESS_CREATION_FLAGS(256u32);
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
pub const CREATE_NEW_PROCESS_GROUP: PROCESS_CREATION_FLAGS = PROCESS_CREATION_FLAGS(512u32);
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
pub const CREATE_UNICODE_ENVIRONMENT: PROCESS_CREATION_FLAGS = PROCESS_CREATION_FLAGS(1024u32);
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
pub const CREATE_SEPARATE_WOW_VDM: PROCESS_CREATION_FLAGS = PROCESS_CREATION_FLAGS(2048u32);
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
pub const CREATE_SHARED_WOW_VDM: PROCESS_CREATION_FLAGS = PROCESS_CREATION_FLAGS(4096u32);
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
pub const CREATE_FORCEDOS: PROCESS_CREATION_FLAGS = PROCESS_CREATION_FLAGS(8192u32);
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
pub const BELOW_NORMAL_PRIORITY_CLASS: PROCESS_CREATION_FLAGS = PROCESS_CREATION_FLAGS(16384u32);
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
pub const ABOVE_NORMAL_PRIORITY_CLASS: PROCESS_CREATION_FLAGS = PROCESS_CREATION_FLAGS(32768u32);
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
pub const INHERIT_PARENT_AFFINITY: PROCESS_CREATION_FLAGS = PROCESS_CREATION_FLAGS(65536u32);
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
pub const INHERIT_CALLER_PRIORITY: PROCESS_CREATION_FLAGS = PROCESS_CREATION_FLAGS(131072u32);
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
pub const CREATE_PROTECTED_PROCESS: PROCESS_CREATION_FLAGS = PROCESS_CREATION_FLAGS(262144u32);
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
pub const EXTENDED_STARTUPINFO_PRESENT: PROCESS_CREATION_FLAGS = PROCESS_CREATION_FLAGS(524288u32);
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
pub const PROCESS_MODE_BACKGROUND_BEGIN: PROCESS_CREATION_FLAGS = PROCESS_CREATION_FLAGS(1048576u32);
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
pub const PROCESS_MODE_BACKGROUND_END: PROCESS_CREATION_FLAGS = PROCESS_CREATION_FLAGS(2097152u32);
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
pub const CREATE_SECURE_PROCESS: PROCESS_CREATION_FLAGS = PROCESS_CREATION_FLAGS(4194304u32);
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
pub const CREATE_BREAKAWAY_FROM_JOB: PROCESS_CREATION_FLAGS = PROCESS_CREATION_FLAGS(16777216u32);
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
pub const CREATE_PRESERVE_CODE_AUTHZ_LEVEL: PROCESS_CREATION_FLAGS = PROCESS_CREATION_FLAGS(33554432u32);
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
pub const CREATE_DEFAULT_ERROR_MODE: PROCESS_CREATION_FLAGS = PROCESS_CREATION_FLAGS(67108864u32);
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
pub const CREATE_NO_WINDOW: PROCESS_CREATION_FLAGS = PROCESS_CREATION_FLAGS(134217728u32);
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
pub const PROFILE_USER: PROCESS_CREATION_FLAGS = PROCESS_CREATION_FLAGS(268435456u32);
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
pub const PROFILE_KERNEL: PROCESS_CREATION_FLAGS = PROCESS_CREATION_FLAGS(536870912u32);
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
pub const PROFILE_SERVER: PROCESS_CREATION_FLAGS = PROCESS_CREATION_FLAGS(1073741824u32);
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
pub const CREATE_IGNORE_SYSTEM_DEFAULT: PROCESS_CREATION_FLAGS = PROCESS_CREATION_FLAGS(2147483648u32);
impl ::core::marker::Copy for PROCESS_CREATION_FLAGS {}
impl ::core::clone::Clone for PROCESS_CREATION_FLAGS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for PROCESS_CREATION_FLAGS {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for PROCESS_CREATION_FLAGS {
    type Abi = Self;
}
impl ::core::fmt::Debug for PROCESS_CREATION_FLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PROCESS_CREATION_FLAGS").field(&self.0).finish()
    }
}
impl ::core::ops::BitOr for PROCESS_CREATION_FLAGS {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for PROCESS_CREATION_FLAGS {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for PROCESS_CREATION_FLAGS {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for PROCESS_CREATION_FLAGS {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for PROCESS_CREATION_FLAGS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct PROCESS_DEP_FLAGS(pub u32);
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
pub const PROCESS_DEP_ENABLE: PROCESS_DEP_FLAGS = PROCESS_DEP_FLAGS(1u32);
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
pub const PROCESS_DEP_DISABLE_ATL_THUNK_EMULATION: PROCESS_DEP_FLAGS = PROCESS_DEP_FLAGS(2u32);
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
pub const PROCESS_DEP_NONE: PROCESS_DEP_FLAGS = PROCESS_DEP_FLAGS(0u32);
impl ::core::marker::Copy for PROCESS_DEP_FLAGS {}
impl ::core::clone::Clone for PROCESS_DEP_FLAGS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for PROCESS_DEP_FLAGS {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for PROCESS_DEP_FLAGS {
    type Abi = Self;
}
impl ::core::fmt::Debug for PROCESS_DEP_FLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PROCESS_DEP_FLAGS").field(&self.0).finish()
    }
}
impl ::core::ops::BitOr for PROCESS_DEP_FLAGS {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for PROCESS_DEP_FLAGS {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for PROCESS_DEP_FLAGS {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for PROCESS_DEP_FLAGS {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for PROCESS_DEP_FLAGS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
pub struct PROCESS_DYNAMIC_EH_CONTINUATION_TARGET {
    pub TargetAddress: usize,
    pub Flags: usize,
}
impl ::core::marker::Copy for PROCESS_DYNAMIC_EH_CONTINUATION_TARGET {}
impl ::core::clone::Clone for PROCESS_DYNAMIC_EH_CONTINUATION_TARGET {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for PROCESS_DYNAMIC_EH_CONTINUATION_TARGET {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("PROCESS_DYNAMIC_EH_CONTINUATION_TARGET").field("TargetAddress", &self.TargetAddress).field("Flags", &self.Flags).finish()
    }
}
unsafe impl ::windows::core::Abi for PROCESS_DYNAMIC_EH_CONTINUATION_TARGET {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for PROCESS_DYNAMIC_EH_CONTINUATION_TARGET {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<PROCESS_DYNAMIC_EH_CONTINUATION_TARGET>()) == 0 }
    }
}
impl ::core::cmp::Eq for PROCESS_DYNAMIC_EH_CONTINUATION_TARGET {}
impl ::core::default::Default for PROCESS_DYNAMIC_EH_CONTINUATION_TARGET {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
pub struct PROCESS_DYNAMIC_EH_CONTINUATION_TARGETS_INFORMATION {
    pub NumberOfTargets: u16,
    pub Reserved: u16,
    pub Reserved2: u32,
    pub Targets: *mut PROCESS_DYNAMIC_EH_CONTINUATION_TARGET,
}
impl ::core::marker::Copy for PROCESS_DYNAMIC_EH_CONTINUATION_TARGETS_INFORMATION {}
impl ::core::clone::Clone for PROCESS_DYNAMIC_EH_CONTINUATION_TARGETS_INFORMATION {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for PROCESS_DYNAMIC_EH_CONTINUATION_TARGETS_INFORMATION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("PROCESS_DYNAMIC_EH_CONTINUATION_TARGETS_INFORMATION").field("NumberOfTargets", &self.NumberOfTargets).field("Reserved", &self.Reserved).field("Reserved2", &self.Reserved2).field("Targets", &self.Targets).finish()
    }
}
unsafe impl ::windows::core::Abi for PROCESS_DYNAMIC_EH_CONTINUATION_TARGETS_INFORMATION {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for PROCESS_DYNAMIC_EH_CONTINUATION_TARGETS_INFORMATION {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<PROCESS_DYNAMIC_EH_CONTINUATION_TARGETS_INFORMATION>()) == 0 }
    }
}
impl ::core::cmp::Eq for PROCESS_DYNAMIC_EH_CONTINUATION_TARGETS_INFORMATION {}
impl ::core::default::Default for PROCESS_DYNAMIC_EH_CONTINUATION_TARGETS_INFORMATION {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
pub struct PROCESS_DYNAMIC_ENFORCED_ADDRESS_RANGE {
    pub BaseAddress: usize,
    pub Size: usize,
    pub Flags: u32,
}
impl ::core::marker::Copy for PROCESS_DYNAMIC_ENFORCED_ADDRESS_RANGE {}
impl ::core::clone::Clone for PROCESS_DYNAMIC_ENFORCED_ADDRESS_RANGE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for PROCESS_DYNAMIC_ENFORCED_ADDRESS_RANGE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("PROCESS_DYNAMIC_ENFORCED_ADDRESS_RANGE").field("BaseAddress", &self.BaseAddress).field("Size", &self.Size).field("Flags", &self.Flags).finish()
    }
}
unsafe impl ::windows::core::Abi for PROCESS_DYNAMIC_ENFORCED_ADDRESS_RANGE {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for PROCESS_DYNAMIC_ENFORCED_ADDRESS_RANGE {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<PROCESS_DYNAMIC_ENFORCED_ADDRESS_RANGE>()) == 0 }
    }
}
impl ::core::cmp::Eq for PROCESS_DYNAMIC_ENFORCED_ADDRESS_RANGE {}
impl ::core::default::Default for PROCESS_DYNAMIC_ENFORCED_ADDRESS_RANGE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
pub struct PROCESS_DYNAMIC_ENFORCED_ADDRESS_RANGES_INFORMATION {
    pub NumberOfRanges: u16,
    pub Reserved: u16,
    pub Reserved2: u32,
    pub Ranges: *mut PROCESS_DYNAMIC_ENFORCED_ADDRESS_RANGE,
}
impl ::core::marker::Copy for PROCESS_DYNAMIC_ENFORCED_ADDRESS_RANGES_INFORMATION {}
impl ::core::clone::Clone for PROCESS_DYNAMIC_ENFORCED_ADDRESS_RANGES_INFORMATION {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for PROCESS_DYNAMIC_ENFORCED_ADDRESS_RANGES_INFORMATION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("PROCESS_DYNAMIC_ENFORCED_ADDRESS_RANGES_INFORMATION").field("NumberOfRanges", &self.NumberOfRanges).field("Reserved", &self.Reserved).field("Reserved2", &self.Reserved2).field("Ranges", &self.Ranges).finish()
    }
}
unsafe impl ::windows::core::Abi for PROCESS_DYNAMIC_ENFORCED_ADDRESS_RANGES_INFORMATION {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for PROCESS_DYNAMIC_ENFORCED_ADDRESS_RANGES_INFORMATION {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<PROCESS_DYNAMIC_ENFORCED_ADDRESS_RANGES_INFORMATION>()) == 0 }
    }
}
impl ::core::cmp::Eq for PROCESS_DYNAMIC_ENFORCED_ADDRESS_RANGES_INFORMATION {}
impl ::core::default::Default for PROCESS_DYNAMIC_ENFORCED_ADDRESS_RANGES_INFORMATION {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Threading\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct PROCESS_INFORMATION {
    pub hProcess: super::super::Foundation::HANDLE,
    pub hThread: super::super::Foundation::HANDLE,
    pub dwProcessId: u32,
    pub dwThreadId: u32,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for PROCESS_INFORMATION {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for PROCESS_INFORMATION {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for PROCESS_INFORMATION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("PROCESS_INFORMATION").field("hProcess", &self.hProcess).field("hThread", &self.hThread).field("dwProcessId", &self.dwProcessId).field("dwThreadId", &self.dwThreadId).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for PROCESS_INFORMATION {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for PROCESS_INFORMATION {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<PROCESS_INFORMATION>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for PROCESS_INFORMATION {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for PROCESS_INFORMATION {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct PROCESS_INFORMATION_CLASS(pub i32);
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
pub const ProcessMemoryPriority: PROCESS_INFORMATION_CLASS = PROCESS_INFORMATION_CLASS(0i32);
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
pub const ProcessMemoryExhaustionInfo: PROCESS_INFORMATION_CLASS = PROCESS_INFORMATION_CLASS(1i32);
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
pub const ProcessAppMemoryInfo: PROCESS_INFORMATION_CLASS = PROCESS_INFORMATION_CLASS(2i32);
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
pub const ProcessInPrivateInfo: PROCESS_INFORMATION_CLASS = PROCESS_INFORMATION_CLASS(3i32);
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
pub const ProcessPowerThrottling: PROCESS_INFORMATION_CLASS = PROCESS_INFORMATION_CLASS(4i32);
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
pub const ProcessReservedValue1: PROCESS_INFORMATION_CLASS = PROCESS_INFORMATION_CLASS(5i32);
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
pub const ProcessTelemetryCoverageInfo: PROCESS_INFORMATION_CLASS = PROCESS_INFORMATION_CLASS(6i32);
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
pub const ProcessProtectionLevelInfo: PROCESS_INFORMATION_CLASS = PROCESS_INFORMATION_CLASS(7i32);
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
pub const ProcessLeapSecondInfo: PROCESS_INFORMATION_CLASS = PROCESS_INFORMATION_CLASS(8i32);
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
pub const ProcessMachineTypeInfo: PROCESS_INFORMATION_CLASS = PROCESS_INFORMATION_CLASS(9i32);
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
pub const ProcessInformationClassMax: PROCESS_INFORMATION_CLASS = PROCESS_INFORMATION_CLASS(10i32);
impl ::core::marker::Copy for PROCESS_INFORMATION_CLASS {}
impl ::core::clone::Clone for PROCESS_INFORMATION_CLASS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for PROCESS_INFORMATION_CLASS {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for PROCESS_INFORMATION_CLASS {
    type Abi = Self;
}
impl ::core::fmt::Debug for PROCESS_INFORMATION_CLASS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PROCESS_INFORMATION_CLASS").field(&self.0).finish()
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
pub struct PROCESS_LEAP_SECOND_INFO {
    pub Flags: u32,
    pub Reserved: u32,
}
impl ::core::marker::Copy for PROCESS_LEAP_SECOND_INFO {}
impl ::core::clone::Clone for PROCESS_LEAP_SECOND_INFO {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for PROCESS_LEAP_SECOND_INFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("PROCESS_LEAP_SECOND_INFO").field("Flags", &self.Flags).field("Reserved", &self.Reserved).finish()
    }
}
unsafe impl ::windows::core::Abi for PROCESS_LEAP_SECOND_INFO {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for PROCESS_LEAP_SECOND_INFO {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<PROCESS_LEAP_SECOND_INFO>()) == 0 }
    }
}
impl ::core::cmp::Eq for PROCESS_LEAP_SECOND_INFO {}
impl ::core::default::Default for PROCESS_LEAP_SECOND_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
pub const PROCESS_LEAP_SECOND_INFO_FLAG_ENABLE_SIXTY_SECOND: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
pub const PROCESS_LEAP_SECOND_INFO_VALID_FLAGS: u32 = 1u32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
pub struct PROCESS_MACHINE_INFORMATION {
    pub ProcessMachine: u16,
    pub Res0: u16,
    pub MachineAttributes: MACHINE_ATTRIBUTES,
}
impl ::core::marker::Copy for PROCESS_MACHINE_INFORMATION {}
impl ::core::clone::Clone for PROCESS_MACHINE_INFORMATION {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for PROCESS_MACHINE_INFORMATION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("PROCESS_MACHINE_INFORMATION").field("ProcessMachine", &self.ProcessMachine).field("Res0", &self.Res0).field("MachineAttributes", &self.MachineAttributes).finish()
    }
}
unsafe impl ::windows::core::Abi for PROCESS_MACHINE_INFORMATION {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for PROCESS_MACHINE_INFORMATION {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<PROCESS_MACHINE_INFORMATION>()) == 0 }
    }
}
impl ::core::cmp::Eq for PROCESS_MACHINE_INFORMATION {}
impl ::core::default::Default for PROCESS_MACHINE_INFORMATION {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
pub struct PROCESS_MEMORY_EXHAUSTION_INFO {
    pub Version: u16,
    pub Reserved: u16,
    pub Type: PROCESS_MEMORY_EXHAUSTION_TYPE,
    pub Value: usize,
}
impl ::core::marker::Copy for PROCESS_MEMORY_EXHAUSTION_INFO {}
impl ::core::clone::Clone for PROCESS_MEMORY_EXHAUSTION_INFO {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for PROCESS_MEMORY_EXHAUSTION_INFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("PROCESS_MEMORY_EXHAUSTION_INFO").field("Version", &self.Version).field("Reserved", &self.Reserved).field("Type", &self.Type).field("Value", &self.Value).finish()
    }
}
unsafe impl ::windows::core::Abi for PROCESS_MEMORY_EXHAUSTION_INFO {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for PROCESS_MEMORY_EXHAUSTION_INFO {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<PROCESS_MEMORY_EXHAUSTION_INFO>()) == 0 }
    }
}
impl ::core::cmp::Eq for PROCESS_MEMORY_EXHAUSTION_INFO {}
impl ::core::default::Default for PROCESS_MEMORY_EXHAUSTION_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct PROCESS_MEMORY_EXHAUSTION_TYPE(pub i32);
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
pub const PMETypeFailFastOnCommitFailure: PROCESS_MEMORY_EXHAUSTION_TYPE = PROCESS_MEMORY_EXHAUSTION_TYPE(0i32);
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
pub const PMETypeMax: PROCESS_MEMORY_EXHAUSTION_TYPE = PROCESS_MEMORY_EXHAUSTION_TYPE(1i32);
impl ::core::marker::Copy for PROCESS_MEMORY_EXHAUSTION_TYPE {}
impl ::core::clone::Clone for PROCESS_MEMORY_EXHAUSTION_TYPE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for PROCESS_MEMORY_EXHAUSTION_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for PROCESS_MEMORY_EXHAUSTION_TYPE {
    type Abi = Self;
}
impl ::core::fmt::Debug for PROCESS_MEMORY_EXHAUSTION_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PROCESS_MEMORY_EXHAUSTION_TYPE").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct PROCESS_MITIGATION_POLICY(pub i32);
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
pub const ProcessDEPPolicy: PROCESS_MITIGATION_POLICY = PROCESS_MITIGATION_POLICY(0i32);
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
pub const ProcessASLRPolicy: PROCESS_MITIGATION_POLICY = PROCESS_MITIGATION_POLICY(1i32);
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
pub const ProcessDynamicCodePolicy: PROCESS_MITIGATION_POLICY = PROCESS_MITIGATION_POLICY(2i32);
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
pub const ProcessStrictHandleCheckPolicy: PROCESS_MITIGATION_POLICY = PROCESS_MITIGATION_POLICY(3i32);
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
pub const ProcessSystemCallDisablePolicy: PROCESS_MITIGATION_POLICY = PROCESS_MITIGATION_POLICY(4i32);
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
pub const ProcessMitigationOptionsMask: PROCESS_MITIGATION_POLICY = PROCESS_MITIGATION_POLICY(5i32);
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
pub const ProcessExtensionPointDisablePolicy: PROCESS_MITIGATION_POLICY = PROCESS_MITIGATION_POLICY(6i32);
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
pub const ProcessControlFlowGuardPolicy: PROCESS_MITIGATION_POLICY = PROCESS_MITIGATION_POLICY(7i32);
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
pub const ProcessSignaturePolicy: PROCESS_MITIGATION_POLICY = PROCESS_MITIGATION_POLICY(8i32);
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
pub const ProcessFontDisablePolicy: PROCESS_MITIGATION_POLICY = PROCESS_MITIGATION_POLICY(9i32);
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
pub const ProcessImageLoadPolicy: PROCESS_MITIGATION_POLICY = PROCESS_MITIGATION_POLICY(10i32);
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
pub const ProcessSystemCallFilterPolicy: PROCESS_MITIGATION_POLICY = PROCESS_MITIGATION_POLICY(11i32);
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
pub const ProcessPayloadRestrictionPolicy: PROCESS_MITIGATION_POLICY = PROCESS_MITIGATION_POLICY(12i32);
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
pub const ProcessChildProcessPolicy: PROCESS_MITIGATION_POLICY = PROCESS_MITIGATION_POLICY(13i32);
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
pub const ProcessSideChannelIsolationPolicy: PROCESS_MITIGATION_POLICY = PROCESS_MITIGATION_POLICY(14i32);
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
pub const ProcessUserShadowStackPolicy: PROCESS_MITIGATION_POLICY = PROCESS_MITIGATION_POLICY(15i32);
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
pub const ProcessRedirectionTrustPolicy: PROCESS_MITIGATION_POLICY = PROCESS_MITIGATION_POLICY(16i32);
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
pub const MaxProcessMitigationPolicy: PROCESS_MITIGATION_POLICY = PROCESS_MITIGATION_POLICY(17i32);
impl ::core::marker::Copy for PROCESS_MITIGATION_POLICY {}
impl ::core::clone::Clone for PROCESS_MITIGATION_POLICY {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for PROCESS_MITIGATION_POLICY {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for PROCESS_MITIGATION_POLICY {
    type Abi = Self;
}
impl ::core::fmt::Debug for PROCESS_MITIGATION_POLICY {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PROCESS_MITIGATION_POLICY").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct PROCESS_NAME_FORMAT(pub u32);
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
pub const PROCESS_NAME_WIN32: PROCESS_NAME_FORMAT = PROCESS_NAME_FORMAT(0u32);
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
pub const PROCESS_NAME_NATIVE: PROCESS_NAME_FORMAT = PROCESS_NAME_FORMAT(1u32);
impl ::core::marker::Copy for PROCESS_NAME_FORMAT {}
impl ::core::clone::Clone for PROCESS_NAME_FORMAT {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for PROCESS_NAME_FORMAT {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for PROCESS_NAME_FORMAT {
    type Abi = Self;
}
impl ::core::fmt::Debug for PROCESS_NAME_FORMAT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PROCESS_NAME_FORMAT").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
pub const PROCESS_POWER_THROTTLING_CURRENT_VERSION: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
pub const PROCESS_POWER_THROTTLING_EXECUTION_SPEED: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
pub const PROCESS_POWER_THROTTLING_IGNORE_TIMER_RESOLUTION: u32 = 4u32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
pub struct PROCESS_POWER_THROTTLING_STATE {
    pub Version: u32,
    pub ControlMask: u32,
    pub StateMask: u32,
}
impl ::core::marker::Copy for PROCESS_POWER_THROTTLING_STATE {}
impl ::core::clone::Clone for PROCESS_POWER_THROTTLING_STATE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for PROCESS_POWER_THROTTLING_STATE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("PROCESS_POWER_THROTTLING_STATE").field("Version", &self.Version).field("ControlMask", &self.ControlMask).field("StateMask", &self.StateMask).finish()
    }
}
unsafe impl ::windows::core::Abi for PROCESS_POWER_THROTTLING_STATE {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for PROCESS_POWER_THROTTLING_STATE {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<PROCESS_POWER_THROTTLING_STATE>()) == 0 }
    }
}
impl ::core::cmp::Eq for PROCESS_POWER_THROTTLING_STATE {}
impl ::core::default::Default for PROCESS_POWER_THROTTLING_STATE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct PROCESS_PROTECTION_LEVEL(pub u32);
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
pub const PROTECTION_LEVEL_WINTCB_LIGHT: PROCESS_PROTECTION_LEVEL = PROCESS_PROTECTION_LEVEL(0u32);
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
pub const PROTECTION_LEVEL_WINDOWS: PROCESS_PROTECTION_LEVEL = PROCESS_PROTECTION_LEVEL(1u32);
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
pub const PROTECTION_LEVEL_WINDOWS_LIGHT: PROCESS_PROTECTION_LEVEL = PROCESS_PROTECTION_LEVEL(2u32);
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
pub const PROTECTION_LEVEL_ANTIMALWARE_LIGHT: PROCESS_PROTECTION_LEVEL = PROCESS_PROTECTION_LEVEL(3u32);
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
pub const PROTECTION_LEVEL_LSA_LIGHT: PROCESS_PROTECTION_LEVEL = PROCESS_PROTECTION_LEVEL(4u32);
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
pub const PROTECTION_LEVEL_WINTCB: PROCESS_PROTECTION_LEVEL = PROCESS_PROTECTION_LEVEL(5u32);
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
pub const PROTECTION_LEVEL_CODEGEN_LIGHT: PROCESS_PROTECTION_LEVEL = PROCESS_PROTECTION_LEVEL(6u32);
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
pub const PROTECTION_LEVEL_AUTHENTICODE: PROCESS_PROTECTION_LEVEL = PROCESS_PROTECTION_LEVEL(7u32);
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
pub const PROTECTION_LEVEL_PPL_APP: PROCESS_PROTECTION_LEVEL = PROCESS_PROTECTION_LEVEL(8u32);
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
pub const PROTECTION_LEVEL_NONE: PROCESS_PROTECTION_LEVEL = PROCESS_PROTECTION_LEVEL(4294967294u32);
impl ::core::marker::Copy for PROCESS_PROTECTION_LEVEL {}
impl ::core::clone::Clone for PROCESS_PROTECTION_LEVEL {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for PROCESS_PROTECTION_LEVEL {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for PROCESS_PROTECTION_LEVEL {
    type Abi = Self;
}
impl ::core::fmt::Debug for PROCESS_PROTECTION_LEVEL {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PROCESS_PROTECTION_LEVEL").field(&self.0).finish()
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
pub struct PROCESS_PROTECTION_LEVEL_INFORMATION {
    pub ProtectionLevel: PROCESS_PROTECTION_LEVEL,
}
impl ::core::marker::Copy for PROCESS_PROTECTION_LEVEL_INFORMATION {}
impl ::core::clone::Clone for PROCESS_PROTECTION_LEVEL_INFORMATION {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for PROCESS_PROTECTION_LEVEL_INFORMATION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("PROCESS_PROTECTION_LEVEL_INFORMATION").field("ProtectionLevel", &self.ProtectionLevel).finish()
    }
}
unsafe impl ::windows::core::Abi for PROCESS_PROTECTION_LEVEL_INFORMATION {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for PROCESS_PROTECTION_LEVEL_INFORMATION {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<PROCESS_PROTECTION_LEVEL_INFORMATION>()) == 0 }
    }
}
impl ::core::cmp::Eq for PROCESS_PROTECTION_LEVEL_INFORMATION {}
impl ::core::default::Default for PROCESS_PROTECTION_LEVEL_INFORMATION {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
pub const PROC_THREAD_ATTRIBUTE_ALL_APPLICATION_PACKAGES_POLICY: u32 = 131087u32;
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
pub const PROC_THREAD_ATTRIBUTE_CHILD_PROCESS_POLICY: u32 = 131086u32;
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
pub const PROC_THREAD_ATTRIBUTE_COMPONENT_FILTER: u32 = 131098u32;
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
pub const PROC_THREAD_ATTRIBUTE_DESKTOP_APP_POLICY: u32 = 131090u32;
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
pub const PROC_THREAD_ATTRIBUTE_ENABLE_OPTIONAL_XSTATE_FEATURES: u32 = 196635u32;
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
pub const PROC_THREAD_ATTRIBUTE_GROUP_AFFINITY: u32 = 196611u32;
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
pub const PROC_THREAD_ATTRIBUTE_HANDLE_LIST: u32 = 131074u32;
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
pub const PROC_THREAD_ATTRIBUTE_IDEAL_PROCESSOR: u32 = 196613u32;
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
pub const PROC_THREAD_ATTRIBUTE_JOB_LIST: u32 = 131085u32;
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
pub const PROC_THREAD_ATTRIBUTE_MACHINE_TYPE: u32 = 131097u32;
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
pub const PROC_THREAD_ATTRIBUTE_MITIGATION_AUDIT_POLICY: u32 = 131096u32;
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
pub const PROC_THREAD_ATTRIBUTE_MITIGATION_POLICY: u32 = 131079u32;
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct PROC_THREAD_ATTRIBUTE_NUM(pub u32);
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
pub const ProcThreadAttributeParentProcess: PROC_THREAD_ATTRIBUTE_NUM = PROC_THREAD_ATTRIBUTE_NUM(0u32);
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
pub const ProcThreadAttributeHandleList: PROC_THREAD_ATTRIBUTE_NUM = PROC_THREAD_ATTRIBUTE_NUM(2u32);
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
pub const ProcThreadAttributeGroupAffinity: PROC_THREAD_ATTRIBUTE_NUM = PROC_THREAD_ATTRIBUTE_NUM(3u32);
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
pub const ProcThreadAttributePreferredNode: PROC_THREAD_ATTRIBUTE_NUM = PROC_THREAD_ATTRIBUTE_NUM(4u32);
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
pub const ProcThreadAttributeIdealProcessor: PROC_THREAD_ATTRIBUTE_NUM = PROC_THREAD_ATTRIBUTE_NUM(5u32);
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
pub const ProcThreadAttributeUmsThread: PROC_THREAD_ATTRIBUTE_NUM = PROC_THREAD_ATTRIBUTE_NUM(6u32);
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
pub const ProcThreadAttributeMitigationPolicy: PROC_THREAD_ATTRIBUTE_NUM = PROC_THREAD_ATTRIBUTE_NUM(7u32);
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
pub const ProcThreadAttributeSecurityCapabilities: PROC_THREAD_ATTRIBUTE_NUM = PROC_THREAD_ATTRIBUTE_NUM(9u32);
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
pub const ProcThreadAttributeProtectionLevel: PROC_THREAD_ATTRIBUTE_NUM = PROC_THREAD_ATTRIBUTE_NUM(11u32);
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
pub const ProcThreadAttributeJobList: PROC_THREAD_ATTRIBUTE_NUM = PROC_THREAD_ATTRIBUTE_NUM(13u32);
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
pub const ProcThreadAttributeChildProcessPolicy: PROC_THREAD_ATTRIBUTE_NUM = PROC_THREAD_ATTRIBUTE_NUM(14u32);
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
pub const ProcThreadAttributeAllApplicationPackagesPolicy: PROC_THREAD_ATTRIBUTE_NUM = PROC_THREAD_ATTRIBUTE_NUM(15u32);
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
pub const ProcThreadAttributeWin32kFilter: PROC_THREAD_ATTRIBUTE_NUM = PROC_THREAD_ATTRIBUTE_NUM(16u32);
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
pub const ProcThreadAttributeSafeOpenPromptOriginClaim: PROC_THREAD_ATTRIBUTE_NUM = PROC_THREAD_ATTRIBUTE_NUM(17u32);
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
pub const ProcThreadAttributeDesktopAppPolicy: PROC_THREAD_ATTRIBUTE_NUM = PROC_THREAD_ATTRIBUTE_NUM(18u32);
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
pub const ProcThreadAttributePseudoConsole: PROC_THREAD_ATTRIBUTE_NUM = PROC_THREAD_ATTRIBUTE_NUM(22u32);
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
pub const ProcThreadAttributeMitigationAuditPolicy: PROC_THREAD_ATTRIBUTE_NUM = PROC_THREAD_ATTRIBUTE_NUM(24u32);
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
pub const ProcThreadAttributeMachineType: PROC_THREAD_ATTRIBUTE_NUM = PROC_THREAD_ATTRIBUTE_NUM(25u32);
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
pub const ProcThreadAttributeComponentFilter: PROC_THREAD_ATTRIBUTE_NUM = PROC_THREAD_ATTRIBUTE_NUM(26u32);
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
pub const ProcThreadAttributeEnableOptionalXStateFeatures: PROC_THREAD_ATTRIBUTE_NUM = PROC_THREAD_ATTRIBUTE_NUM(27u32);
impl ::core::marker::Copy for PROC_THREAD_ATTRIBUTE_NUM {}
impl ::core::clone::Clone for PROC_THREAD_ATTRIBUTE_NUM {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for PROC_THREAD_ATTRIBUTE_NUM {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for PROC_THREAD_ATTRIBUTE_NUM {
    type Abi = Self;
}
impl ::core::fmt::Debug for PROC_THREAD_ATTRIBUTE_NUM {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PROC_THREAD_ATTRIBUTE_NUM").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
pub const PROC_THREAD_ATTRIBUTE_PARENT_PROCESS: u32 = 131072u32;
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
pub const PROC_THREAD_ATTRIBUTE_PREFERRED_NODE: u32 = 131076u32;
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
pub const PROC_THREAD_ATTRIBUTE_PROTECTION_LEVEL: u32 = 131083u32;
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
pub const PROC_THREAD_ATTRIBUTE_PSEUDOCONSOLE: u32 = 131094u32;
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
pub const PROC_THREAD_ATTRIBUTE_REPLACE_VALUE: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
pub const PROC_THREAD_ATTRIBUTE_SECURITY_CAPABILITIES: u32 = 131081u32;
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
pub const PROC_THREAD_ATTRIBUTE_UMS_THREAD: u32 = 196614u32;
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
pub const PROC_THREAD_ATTRIBUTE_WIN32K_FILTER: u32 = 131088u32;
#[doc = "*Required features: `\"Win32_System_Threading\"`, `\"Win32_System_SystemServices\"`*"]
#[cfg(feature = "Win32_System_SystemServices")]
pub type PRTL_UMS_SCHEDULER_ENTRY_POINT = ::core::option::Option<unsafe extern "system" fn(reason: super::SystemServices::RTL_UMS_SCHEDULER_REASON, activationpayload: usize, schedulerparam: *const ::core::ffi::c_void)>;
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
pub type PTIMERAPCROUTINE = ::core::option::Option<unsafe extern "system" fn(lpargtocompletionroutine: *const ::core::ffi::c_void, dwtimerlowvalue: u32, dwtimerhighvalue: u32)>;
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
pub type PTP_CLEANUP_GROUP_CANCEL_CALLBACK = ::core::option::Option<unsafe extern "system" fn(objectcontext: *mut ::core::ffi::c_void, cleanupcontext: *mut ::core::ffi::c_void)>;
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct PTP_POOL(pub isize);
impl PTP_POOL {
    pub fn is_invalid(&self) -> bool {
        *self == unsafe { ::core::mem::zeroed() }
    }
    pub fn ok(self) -> ::windows::core::Result<Self> {
        if !self.is_invalid() {
            Ok(self)
        } else {
            Err(::windows::core::Error::from_win32())
        }
    }
}
impl ::core::default::Default for PTP_POOL {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::clone::Clone for PTP_POOL {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::marker::Copy for PTP_POOL {}
impl ::core::fmt::Debug for PTP_POOL {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PTP_POOL").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Abi for PTP_POOL {
    type Abi = Self;
}
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
pub type PTP_SIMPLE_CALLBACK = ::core::option::Option<unsafe extern "system" fn(instance: *mut TP_CALLBACK_INSTANCE, context: *mut ::core::ffi::c_void)>;
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
pub type PTP_TIMER_CALLBACK = ::core::option::Option<unsafe extern "system" fn(instance: *mut TP_CALLBACK_INSTANCE, context: *mut ::core::ffi::c_void, timer: *mut TP_TIMER)>;
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
pub type PTP_WAIT_CALLBACK = ::core::option::Option<unsafe extern "system" fn(instance: *mut TP_CALLBACK_INSTANCE, context: *mut ::core::ffi::c_void, wait: *mut TP_WAIT, waitresult: u32)>;
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
pub type PTP_WIN32_IO_CALLBACK = ::core::option::Option<unsafe extern "system" fn(instance: *mut TP_CALLBACK_INSTANCE, context: *mut ::core::ffi::c_void, overlapped: *mut ::core::ffi::c_void, ioresult: u32, numberofbytestransferred: usize, io: *mut TP_IO)>;
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
pub type PTP_WORK_CALLBACK = ::core::option::Option<unsafe extern "system" fn(instance: *mut TP_CALLBACK_INSTANCE, context: *mut ::core::ffi::c_void, work: *mut TP_WORK)>;
#[doc = "*Required features: `\"Win32_System_Threading\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn PulseEvent<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::HANDLE>>(hevent: Param0) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn PulseEvent(hevent: super::super::Foundation::HANDLE) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(PulseEvent(hevent.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct QUEUE_USER_APC_FLAGS(pub i32);
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
pub const QUEUE_USER_APC_FLAGS_NONE: QUEUE_USER_APC_FLAGS = QUEUE_USER_APC_FLAGS(0i32);
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
pub const QUEUE_USER_APC_FLAGS_SPECIAL_USER_APC: QUEUE_USER_APC_FLAGS = QUEUE_USER_APC_FLAGS(1i32);
impl ::core::marker::Copy for QUEUE_USER_APC_FLAGS {}
impl ::core::clone::Clone for QUEUE_USER_APC_FLAGS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for QUEUE_USER_APC_FLAGS {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for QUEUE_USER_APC_FLAGS {
    type Abi = Self;
}
impl ::core::fmt::Debug for QUEUE_USER_APC_FLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("QUEUE_USER_APC_FLAGS").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_System_Threading\"`, `\"Win32_System_Kernel\"`*"]
#[cfg(feature = "Win32_System_Kernel")]
#[inline]
pub unsafe fn QueryDepthSList(listhead: *const super::Kernel::SLIST_HEADER) -> u16 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn QueryDepthSList(listhead: *const super::Kernel::SLIST_HEADER) -> u16;
        }
        ::core::mem::transmute(QueryDepthSList(::core::mem::transmute(listhead)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_Threading\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn QueryFullProcessImageNameA<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::HANDLE>>(hprocess: Param0, dwflags: PROCESS_NAME_FORMAT, lpexename: ::windows::core::PSTR, lpdwsize: *mut u32) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn QueryFullProcessImageNameA(hprocess: super::super::Foundation::HANDLE, dwflags: PROCESS_NAME_FORMAT, lpexename: ::windows::core::PSTR, lpdwsize: *mut u32) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(QueryFullProcessImageNameA(hprocess.into_param().abi(), ::core::mem::transmute(dwflags), ::core::mem::transmute(lpexename), ::core::mem::transmute(lpdwsize)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_Threading\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn QueryFullProcessImageNameW<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::HANDLE>>(hprocess: Param0, dwflags: PROCESS_NAME_FORMAT, lpexename: ::windows::core::PWSTR, lpdwsize: *mut u32) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn QueryFullProcessImageNameW(hprocess: super::super::Foundation::HANDLE, dwflags: PROCESS_NAME_FORMAT, lpexename: ::windows::core::PWSTR, lpdwsize: *mut u32) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(QueryFullProcessImageNameW(hprocess.into_param().abi(), ::core::mem::transmute(dwflags), ::core::mem::transmute(lpexename), ::core::mem::transmute(lpdwsize)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_Threading\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn QueryProcessAffinityUpdateMode<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::HANDLE>>(hprocess: Param0, lpdwflags: *mut PROCESS_AFFINITY_AUTO_UPDATE_FLAGS) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn QueryProcessAffinityUpdateMode(hprocess: super::super::Foundation::HANDLE, lpdwflags: *mut PROCESS_AFFINITY_AUTO_UPDATE_FLAGS) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(QueryProcessAffinityUpdateMode(hprocess.into_param().abi(), ::core::mem::transmute(lpdwflags)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_Threading\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn QueryProtectedPolicy(policyguid: *const ::windows::core::GUID, policyvalue: *mut usize) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn QueryProtectedPolicy(policyguid: *const ::windows::core::GUID, policyvalue: *mut usize) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(QueryProtectedPolicy(::core::mem::transmute(policyguid), ::core::mem::transmute(policyvalue)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_Threading\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn QueryThreadpoolStackInformation<'a, Param0: ::windows::core::IntoParam<'a, PTP_POOL>>(ptpp: Param0, ptpsi: *mut TP_POOL_STACK_INFORMATION) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn QueryThreadpoolStackInformation(ptpp: PTP_POOL, ptpsi: *mut TP_POOL_STACK_INFORMATION) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(QueryThreadpoolStackInformation(ptpp.into_param().abi(), ::core::mem::transmute(ptpsi)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_Threading\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn QueryUmsThreadInformation(umsthread: *const ::core::ffi::c_void, umsthreadinfoclass: RTL_UMS_THREAD_INFO_CLASS, umsthreadinformation: *mut ::core::ffi::c_void, umsthreadinformationlength: u32, returnlength: *mut u32) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn QueryUmsThreadInformation(umsthread: *const ::core::ffi::c_void, umsthreadinfoclass: RTL_UMS_THREAD_INFO_CLASS, umsthreadinformation: *mut ::core::ffi::c_void, umsthreadinformationlength: u32, returnlength: *mut u32) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(QueryUmsThreadInformation(::core::mem::transmute(umsthread), ::core::mem::transmute(umsthreadinfoclass), ::core::mem::transmute(umsthreadinformation), ::core::mem::transmute(umsthreadinformationlength), ::core::mem::transmute(returnlength)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_Threading\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn QueueUserAPC<'a, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::HANDLE>>(pfnapc: super::super::Foundation::PAPCFUNC, hthread: Param1, dwdata: usize) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn QueueUserAPC(pfnapc: ::windows::core::RawPtr, hthread: super::super::Foundation::HANDLE, dwdata: usize) -> u32;
        }
        ::core::mem::transmute(QueueUserAPC(::core::mem::transmute(pfnapc), hthread.into_param().abi(), ::core::mem::transmute(dwdata)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_Threading\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn QueueUserAPC2<'a, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::HANDLE>>(apcroutine: super::super::Foundation::PAPCFUNC, thread: Param1, data: usize, flags: QUEUE_USER_APC_FLAGS) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn QueueUserAPC2(apcroutine: ::windows::core::RawPtr, thread: super::super::Foundation::HANDLE, data: usize, flags: QUEUE_USER_APC_FLAGS) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(QueueUserAPC2(::core::mem::transmute(apcroutine), thread.into_param().abi(), ::core::mem::transmute(data), ::core::mem::transmute(flags)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_Threading\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn QueueUserWorkItem(function: LPTHREAD_START_ROUTINE, context: *const ::core::ffi::c_void, flags: WORKER_THREAD_FLAGS) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn QueueUserWorkItem(function: ::windows::core::RawPtr, context: *const ::core::ffi::c_void, flags: WORKER_THREAD_FLAGS) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(QueueUserWorkItem(::core::mem::transmute(function), ::core::mem::transmute(context), ::core::mem::transmute(flags)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Threading\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct REASON_CONTEXT {
    pub Version: u32,
    pub Flags: POWER_REQUEST_CONTEXT_FLAGS,
    pub Reason: REASON_CONTEXT_0,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for REASON_CONTEXT {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for REASON_CONTEXT {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for REASON_CONTEXT {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for REASON_CONTEXT {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<REASON_CONTEXT>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for REASON_CONTEXT {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for REASON_CONTEXT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Threading\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub union REASON_CONTEXT_0 {
    pub Detailed: REASON_CONTEXT_0_0,
    pub SimpleReasonString: ::windows::core::PWSTR,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for REASON_CONTEXT_0 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for REASON_CONTEXT_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for REASON_CONTEXT_0 {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for REASON_CONTEXT_0 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<REASON_CONTEXT_0>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for REASON_CONTEXT_0 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for REASON_CONTEXT_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Threading\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct REASON_CONTEXT_0_0 {
    pub LocalizedReasonModule: super::super::Foundation::HINSTANCE,
    pub LocalizedReasonId: u32,
    pub ReasonStringCount: u32,
    pub ReasonStrings: *mut ::windows::core::PWSTR,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for REASON_CONTEXT_0_0 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for REASON_CONTEXT_0_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for REASON_CONTEXT_0_0 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("REASON_CONTEXT_0_0").field("LocalizedReasonModule", &self.LocalizedReasonModule).field("LocalizedReasonId", &self.LocalizedReasonId).field("ReasonStringCount", &self.ReasonStringCount).field("ReasonStrings", &self.ReasonStrings).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for REASON_CONTEXT_0_0 {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for REASON_CONTEXT_0_0 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<REASON_CONTEXT_0_0>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for REASON_CONTEXT_0_0 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for REASON_CONTEXT_0_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
pub struct RTL_BARRIER {
    pub Reserved1: u32,
    pub Reserved2: u32,
    pub Reserved3: [usize; 2],
    pub Reserved4: u32,
    pub Reserved5: u32,
}
impl ::core::marker::Copy for RTL_BARRIER {}
impl ::core::clone::Clone for RTL_BARRIER {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for RTL_BARRIER {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("RTL_BARRIER").field("Reserved1", &self.Reserved1).field("Reserved2", &self.Reserved2).field("Reserved3", &self.Reserved3).field("Reserved4", &self.Reserved4).field("Reserved5", &self.Reserved5).finish()
    }
}
unsafe impl ::windows::core::Abi for RTL_BARRIER {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for RTL_BARRIER {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<RTL_BARRIER>()) == 0 }
    }
}
impl ::core::cmp::Eq for RTL_BARRIER {}
impl ::core::default::Default for RTL_BARRIER {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
pub struct RTL_CONDITION_VARIABLE {
    pub Ptr: *mut ::core::ffi::c_void,
}
impl ::core::marker::Copy for RTL_CONDITION_VARIABLE {}
impl ::core::clone::Clone for RTL_CONDITION_VARIABLE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for RTL_CONDITION_VARIABLE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("RTL_CONDITION_VARIABLE").field("Ptr", &self.Ptr).finish()
    }
}
unsafe impl ::windows::core::Abi for RTL_CONDITION_VARIABLE {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for RTL_CONDITION_VARIABLE {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<RTL_CONDITION_VARIABLE>()) == 0 }
    }
}
impl ::core::cmp::Eq for RTL_CONDITION_VARIABLE {}
impl ::core::default::Default for RTL_CONDITION_VARIABLE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Threading\"`, `\"Win32_Foundation\"`, `\"Win32_System_Kernel\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Kernel"))]
pub struct RTL_CRITICAL_SECTION {
    pub DebugInfo: *mut RTL_CRITICAL_SECTION_DEBUG,
    pub LockCount: i32,
    pub RecursionCount: i32,
    pub OwningThread: super::super::Foundation::HANDLE,
    pub LockSemaphore: super::super::Foundation::HANDLE,
    pub SpinCount: usize,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Kernel"))]
impl ::core::marker::Copy for RTL_CRITICAL_SECTION {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Kernel"))]
impl ::core::clone::Clone for RTL_CRITICAL_SECTION {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Kernel"))]
impl ::core::fmt::Debug for RTL_CRITICAL_SECTION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("RTL_CRITICAL_SECTION").field("DebugInfo", &self.DebugInfo).field("LockCount", &self.LockCount).field("RecursionCount", &self.RecursionCount).field("OwningThread", &self.OwningThread).field("LockSemaphore", &self.LockSemaphore).field("SpinCount", &self.SpinCount).finish()
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Kernel"))]
unsafe impl ::windows::core::Abi for RTL_CRITICAL_SECTION {
    type Abi = Self;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Kernel"))]
impl ::core::cmp::PartialEq for RTL_CRITICAL_SECTION {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<RTL_CRITICAL_SECTION>()) == 0 }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Kernel"))]
impl ::core::cmp::Eq for RTL_CRITICAL_SECTION {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Kernel"))]
impl ::core::default::Default for RTL_CRITICAL_SECTION {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Threading\"`, `\"Win32_Foundation\"`, `\"Win32_System_Kernel\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Kernel"))]
pub struct RTL_CRITICAL_SECTION_DEBUG {
    pub Type: u16,
    pub CreatorBackTraceIndex: u16,
    pub CriticalSection: *mut RTL_CRITICAL_SECTION,
    pub ProcessLocksList: super::Kernel::LIST_ENTRY,
    pub EntryCount: u32,
    pub ContentionCount: u32,
    pub Flags: u32,
    pub CreatorBackTraceIndexHigh: u16,
    pub SpareWORD: u16,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Kernel"))]
impl ::core::marker::Copy for RTL_CRITICAL_SECTION_DEBUG {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Kernel"))]
impl ::core::clone::Clone for RTL_CRITICAL_SECTION_DEBUG {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Kernel"))]
impl ::core::fmt::Debug for RTL_CRITICAL_SECTION_DEBUG {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("RTL_CRITICAL_SECTION_DEBUG").field("Type", &self.Type).field("CreatorBackTraceIndex", &self.CreatorBackTraceIndex).field("CriticalSection", &self.CriticalSection).field("ProcessLocksList", &self.ProcessLocksList).field("EntryCount", &self.EntryCount).field("ContentionCount", &self.ContentionCount).field("Flags", &self.Flags).field("CreatorBackTraceIndexHigh", &self.CreatorBackTraceIndexHigh).field("SpareWORD", &self.SpareWORD).finish()
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Kernel"))]
unsafe impl ::windows::core::Abi for RTL_CRITICAL_SECTION_DEBUG {
    type Abi = Self;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Kernel"))]
impl ::core::cmp::PartialEq for RTL_CRITICAL_SECTION_DEBUG {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<RTL_CRITICAL_SECTION_DEBUG>()) == 0 }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Kernel"))]
impl ::core::cmp::Eq for RTL_CRITICAL_SECTION_DEBUG {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Kernel"))]
impl ::core::default::Default for RTL_CRITICAL_SECTION_DEBUG {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
pub union RTL_RUN_ONCE {
    pub Ptr: *mut ::core::ffi::c_void,
}
impl ::core::marker::Copy for RTL_RUN_ONCE {}
impl ::core::clone::Clone for RTL_RUN_ONCE {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for RTL_RUN_ONCE {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for RTL_RUN_ONCE {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<RTL_RUN_ONCE>()) == 0 }
    }
}
impl ::core::cmp::Eq for RTL_RUN_ONCE {}
impl ::core::default::Default for RTL_RUN_ONCE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
pub struct RTL_SRWLOCK {
    pub Ptr: *mut ::core::ffi::c_void,
}
impl ::core::marker::Copy for RTL_SRWLOCK {}
impl ::core::clone::Clone for RTL_SRWLOCK {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for RTL_SRWLOCK {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("RTL_SRWLOCK").field("Ptr", &self.Ptr).finish()
    }
}
unsafe impl ::windows::core::Abi for RTL_SRWLOCK {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for RTL_SRWLOCK {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<RTL_SRWLOCK>()) == 0 }
    }
}
impl ::core::cmp::Eq for RTL_SRWLOCK {}
impl ::core::default::Default for RTL_SRWLOCK {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct RTL_UMS_THREAD_INFO_CLASS(pub i32);
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
pub const UmsThreadInvalidInfoClass: RTL_UMS_THREAD_INFO_CLASS = RTL_UMS_THREAD_INFO_CLASS(0i32);
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
pub const UmsThreadUserContext: RTL_UMS_THREAD_INFO_CLASS = RTL_UMS_THREAD_INFO_CLASS(1i32);
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
pub const UmsThreadPriority: RTL_UMS_THREAD_INFO_CLASS = RTL_UMS_THREAD_INFO_CLASS(2i32);
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
pub const UmsThreadAffinity: RTL_UMS_THREAD_INFO_CLASS = RTL_UMS_THREAD_INFO_CLASS(3i32);
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
pub const UmsThreadTeb: RTL_UMS_THREAD_INFO_CLASS = RTL_UMS_THREAD_INFO_CLASS(4i32);
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
pub const UmsThreadIsSuspended: RTL_UMS_THREAD_INFO_CLASS = RTL_UMS_THREAD_INFO_CLASS(5i32);
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
pub const UmsThreadIsTerminated: RTL_UMS_THREAD_INFO_CLASS = RTL_UMS_THREAD_INFO_CLASS(6i32);
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
pub const UmsThreadMaxInfoClass: RTL_UMS_THREAD_INFO_CLASS = RTL_UMS_THREAD_INFO_CLASS(7i32);
impl ::core::marker::Copy for RTL_UMS_THREAD_INFO_CLASS {}
impl ::core::clone::Clone for RTL_UMS_THREAD_INFO_CLASS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for RTL_UMS_THREAD_INFO_CLASS {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for RTL_UMS_THREAD_INFO_CLASS {
    type Abi = Self;
}
impl ::core::fmt::Debug for RTL_UMS_THREAD_INFO_CLASS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("RTL_UMS_THREAD_INFO_CLASS").field(&self.0).finish()
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Threading\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct RTL_USER_PROCESS_PARAMETERS {
    pub Reserved1: [u8; 16],
    pub Reserved2: [*mut ::core::ffi::c_void; 10],
    pub ImagePathName: super::super::Foundation::UNICODE_STRING,
    pub CommandLine: super::super::Foundation::UNICODE_STRING,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for RTL_USER_PROCESS_PARAMETERS {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for RTL_USER_PROCESS_PARAMETERS {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for RTL_USER_PROCESS_PARAMETERS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("RTL_USER_PROCESS_PARAMETERS").field("Reserved1", &self.Reserved1).field("Reserved2", &self.Reserved2).field("ImagePathName", &self.ImagePathName).field("CommandLine", &self.CommandLine).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for RTL_USER_PROCESS_PARAMETERS {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for RTL_USER_PROCESS_PARAMETERS {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<RTL_USER_PROCESS_PARAMETERS>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for RTL_USER_PROCESS_PARAMETERS {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for RTL_USER_PROCESS_PARAMETERS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_System_Threading\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn RegisterWaitForSingleObject<'a, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::HANDLE>>(phnewwaitobject: *mut super::super::Foundation::HANDLE, hobject: Param1, callback: WAITORTIMERCALLBACK, context: *const ::core::ffi::c_void, dwmilliseconds: u32, dwflags: WORKER_THREAD_FLAGS) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn RegisterWaitForSingleObject(phnewwaitobject: *mut super::super::Foundation::HANDLE, hobject: super::super::Foundation::HANDLE, callback: ::windows::core::RawPtr, context: *const ::core::ffi::c_void, dwmilliseconds: u32, dwflags: WORKER_THREAD_FLAGS) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(RegisterWaitForSingleObject(::core::mem::transmute(phnewwaitobject), hobject.into_param().abi(), ::core::mem::transmute(callback), ::core::mem::transmute(context), ::core::mem::transmute(dwmilliseconds), ::core::mem::transmute(dwflags)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_Threading\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn ReleaseMutex<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::HANDLE>>(hmutex: Param0) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn ReleaseMutex(hmutex: super::super::Foundation::HANDLE) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(ReleaseMutex(hmutex.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_Threading\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn ReleaseMutexWhenCallbackReturns<'a, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::HANDLE>>(pci: *mut TP_CALLBACK_INSTANCE, r#mut: Param1) {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn ReleaseMutexWhenCallbackReturns(pci: *mut TP_CALLBACK_INSTANCE, r#mut: super::super::Foundation::HANDLE);
        }
        ReleaseMutexWhenCallbackReturns(::core::mem::transmute(pci), r#mut.into_param().abi())
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
#[inline]
pub unsafe fn ReleaseSRWLockExclusive(srwlock: *mut RTL_SRWLOCK) {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn ReleaseSRWLockExclusive(srwlock: *mut RTL_SRWLOCK);
        }
        ReleaseSRWLockExclusive(::core::mem::transmute(srwlock))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
#[inline]
pub unsafe fn ReleaseSRWLockShared(srwlock: *mut RTL_SRWLOCK) {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn ReleaseSRWLockShared(srwlock: *mut RTL_SRWLOCK);
        }
        ReleaseSRWLockShared(::core::mem::transmute(srwlock))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_Threading\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn ReleaseSemaphore<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::HANDLE>>(hsemaphore: Param0, lreleasecount: i32, lppreviouscount: *mut i32) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn ReleaseSemaphore(hsemaphore: super::super::Foundation::HANDLE, lreleasecount: i32, lppreviouscount: *mut i32) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(ReleaseSemaphore(hsemaphore.into_param().abi(), ::core::mem::transmute(lreleasecount), ::core::mem::transmute(lppreviouscount)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_Threading\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn ReleaseSemaphoreWhenCallbackReturns<'a, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::HANDLE>>(pci: *mut TP_CALLBACK_INSTANCE, sem: Param1, crel: u32) {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn ReleaseSemaphoreWhenCallbackReturns(pci: *mut TP_CALLBACK_INSTANCE, sem: super::super::Foundation::HANDLE, crel: u32);
        }
        ReleaseSemaphoreWhenCallbackReturns(::core::mem::transmute(pci), sem.into_param().abi(), ::core::mem::transmute(crel))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_Threading\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn ResetEvent<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::HANDLE>>(hevent: Param0) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn ResetEvent(hevent: super::super::Foundation::HANDLE) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(ResetEvent(hevent.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_Threading\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn ResumeThread<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::HANDLE>>(hthread: Param0) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn ResumeThread(hthread: super::super::Foundation::HANDLE) -> u32;
        }
        ::core::mem::transmute(ResumeThread(hthread.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Threading\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct STARTUPINFOA {
    pub cb: u32,
    pub lpReserved: ::windows::core::PSTR,
    pub lpDesktop: ::windows::core::PSTR,
    pub lpTitle: ::windows::core::PSTR,
    pub dwX: u32,
    pub dwY: u32,
    pub dwXSize: u32,
    pub dwYSize: u32,
    pub dwXCountChars: u32,
    pub dwYCountChars: u32,
    pub dwFillAttribute: u32,
    pub dwFlags: STARTUPINFOW_FLAGS,
    pub wShowWindow: u16,
    pub cbReserved2: u16,
    pub lpReserved2: *mut u8,
    pub hStdInput: super::super::Foundation::HANDLE,
    pub hStdOutput: super::super::Foundation::HANDLE,
    pub hStdError: super::super::Foundation::HANDLE,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for STARTUPINFOA {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for STARTUPINFOA {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for STARTUPINFOA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("STARTUPINFOA")
            .field("cb", &self.cb)
            .field("lpReserved", &self.lpReserved)
            .field("lpDesktop", &self.lpDesktop)
            .field("lpTitle", &self.lpTitle)
            .field("dwX", &self.dwX)
            .field("dwY", &self.dwY)
            .field("dwXSize", &self.dwXSize)
            .field("dwYSize", &self.dwYSize)
            .field("dwXCountChars", &self.dwXCountChars)
            .field("dwYCountChars", &self.dwYCountChars)
            .field("dwFillAttribute", &self.dwFillAttribute)
            .field("dwFlags", &self.dwFlags)
            .field("wShowWindow", &self.wShowWindow)
            .field("cbReserved2", &self.cbReserved2)
            .field("lpReserved2", &self.lpReserved2)
            .field("hStdInput", &self.hStdInput)
            .field("hStdOutput", &self.hStdOutput)
            .field("hStdError", &self.hStdError)
            .finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for STARTUPINFOA {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for STARTUPINFOA {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<STARTUPINFOA>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for STARTUPINFOA {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for STARTUPINFOA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Threading\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct STARTUPINFOEXA {
    pub StartupInfo: STARTUPINFOA,
    pub lpAttributeList: LPPROC_THREAD_ATTRIBUTE_LIST,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for STARTUPINFOEXA {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for STARTUPINFOEXA {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for STARTUPINFOEXA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("STARTUPINFOEXA").field("StartupInfo", &self.StartupInfo).field("lpAttributeList", &self.lpAttributeList).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for STARTUPINFOEXA {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for STARTUPINFOEXA {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<STARTUPINFOEXA>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for STARTUPINFOEXA {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for STARTUPINFOEXA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Threading\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct STARTUPINFOEXW {
    pub StartupInfo: STARTUPINFOW,
    pub lpAttributeList: LPPROC_THREAD_ATTRIBUTE_LIST,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for STARTUPINFOEXW {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for STARTUPINFOEXW {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for STARTUPINFOEXW {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("STARTUPINFOEXW").field("StartupInfo", &self.StartupInfo).field("lpAttributeList", &self.lpAttributeList).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for STARTUPINFOEXW {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for STARTUPINFOEXW {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<STARTUPINFOEXW>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for STARTUPINFOEXW {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for STARTUPINFOEXW {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Threading\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct STARTUPINFOW {
    pub cb: u32,
    pub lpReserved: ::windows::core::PWSTR,
    pub lpDesktop: ::windows::core::PWSTR,
    pub lpTitle: ::windows::core::PWSTR,
    pub dwX: u32,
    pub dwY: u32,
    pub dwXSize: u32,
    pub dwYSize: u32,
    pub dwXCountChars: u32,
    pub dwYCountChars: u32,
    pub dwFillAttribute: u32,
    pub dwFlags: STARTUPINFOW_FLAGS,
    pub wShowWindow: u16,
    pub cbReserved2: u16,
    pub lpReserved2: *mut u8,
    pub hStdInput: super::super::Foundation::HANDLE,
    pub hStdOutput: super::super::Foundation::HANDLE,
    pub hStdError: super::super::Foundation::HANDLE,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for STARTUPINFOW {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for STARTUPINFOW {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for STARTUPINFOW {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("STARTUPINFOW")
            .field("cb", &self.cb)
            .field("lpReserved", &self.lpReserved)
            .field("lpDesktop", &self.lpDesktop)
            .field("lpTitle", &self.lpTitle)
            .field("dwX", &self.dwX)
            .field("dwY", &self.dwY)
            .field("dwXSize", &self.dwXSize)
            .field("dwYSize", &self.dwYSize)
            .field("dwXCountChars", &self.dwXCountChars)
            .field("dwYCountChars", &self.dwYCountChars)
            .field("dwFillAttribute", &self.dwFillAttribute)
            .field("dwFlags", &self.dwFlags)
            .field("wShowWindow", &self.wShowWindow)
            .field("cbReserved2", &self.cbReserved2)
            .field("lpReserved2", &self.lpReserved2)
            .field("hStdInput", &self.hStdInput)
            .field("hStdOutput", &self.hStdOutput)
            .field("hStdError", &self.hStdError)
            .finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for STARTUPINFOW {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for STARTUPINFOW {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<STARTUPINFOW>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for STARTUPINFOW {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for STARTUPINFOW {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct STARTUPINFOW_FLAGS(pub u32);
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
pub const STARTF_FORCEONFEEDBACK: STARTUPINFOW_FLAGS = STARTUPINFOW_FLAGS(64u32);
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
pub const STARTF_FORCEOFFFEEDBACK: STARTUPINFOW_FLAGS = STARTUPINFOW_FLAGS(128u32);
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
pub const STARTF_PREVENTPINNING: STARTUPINFOW_FLAGS = STARTUPINFOW_FLAGS(8192u32);
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
pub const STARTF_RUNFULLSCREEN: STARTUPINFOW_FLAGS = STARTUPINFOW_FLAGS(32u32);
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
pub const STARTF_TITLEISAPPID: STARTUPINFOW_FLAGS = STARTUPINFOW_FLAGS(4096u32);
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
pub const STARTF_TITLEISLINKNAME: STARTUPINFOW_FLAGS = STARTUPINFOW_FLAGS(2048u32);
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
pub const STARTF_UNTRUSTEDSOURCE: STARTUPINFOW_FLAGS = STARTUPINFOW_FLAGS(32768u32);
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
pub const STARTF_USECOUNTCHARS: STARTUPINFOW_FLAGS = STARTUPINFOW_FLAGS(8u32);
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
pub const STARTF_USEFILLATTRIBUTE: STARTUPINFOW_FLAGS = STARTUPINFOW_FLAGS(16u32);
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
pub const STARTF_USEHOTKEY: STARTUPINFOW_FLAGS = STARTUPINFOW_FLAGS(512u32);
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
pub const STARTF_USEPOSITION: STARTUPINFOW_FLAGS = STARTUPINFOW_FLAGS(4u32);
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
pub const STARTF_USESHOWWINDOW: STARTUPINFOW_FLAGS = STARTUPINFOW_FLAGS(1u32);
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
pub const STARTF_USESIZE: STARTUPINFOW_FLAGS = STARTUPINFOW_FLAGS(2u32);
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
pub const STARTF_USESTDHANDLES: STARTUPINFOW_FLAGS = STARTUPINFOW_FLAGS(256u32);
impl ::core::marker::Copy for STARTUPINFOW_FLAGS {}
impl ::core::clone::Clone for STARTUPINFOW_FLAGS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for STARTUPINFOW_FLAGS {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for STARTUPINFOW_FLAGS {
    type Abi = Self;
}
impl ::core::fmt::Debug for STARTUPINFOW_FLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("STARTUPINFOW_FLAGS").field(&self.0).finish()
    }
}
impl ::core::ops::BitOr for STARTUPINFOW_FLAGS {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for STARTUPINFOW_FLAGS {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for STARTUPINFOW_FLAGS {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for STARTUPINFOW_FLAGS {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for STARTUPINFOW_FLAGS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
pub const SYNCHRONIZATION_BARRIER_FLAGS_BLOCK_ONLY: u32 = 2u32;
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
pub const SYNCHRONIZATION_BARRIER_FLAGS_NO_DELETE: u32 = 4u32;
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
pub const SYNCHRONIZATION_BARRIER_FLAGS_SPIN_ONLY: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_Threading\"`, `\"Win32_Foundation\"`, `\"Win32_System_Kernel\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Kernel"))]
#[inline]
pub unsafe fn SetCriticalSectionSpinCount(lpcriticalsection: *mut RTL_CRITICAL_SECTION, dwspincount: u32) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn SetCriticalSectionSpinCount(lpcriticalsection: *mut RTL_CRITICAL_SECTION, dwspincount: u32) -> u32;
        }
        ::core::mem::transmute(SetCriticalSectionSpinCount(::core::mem::transmute(lpcriticalsection), ::core::mem::transmute(dwspincount)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_Threading\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn SetEvent<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::HANDLE>>(hevent: Param0) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn SetEvent(hevent: super::super::Foundation::HANDLE) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(SetEvent(hevent.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_Threading\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn SetEventWhenCallbackReturns<'a, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::HANDLE>>(pci: *mut TP_CALLBACK_INSTANCE, evt: Param1) {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn SetEventWhenCallbackReturns(pci: *mut TP_CALLBACK_INSTANCE, evt: super::super::Foundation::HANDLE);
        }
        SetEventWhenCallbackReturns(::core::mem::transmute(pci), evt.into_param().abi())
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_Threading\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn SetPriorityClass<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::HANDLE>>(hprocess: Param0, dwpriorityclass: PROCESS_CREATION_FLAGS) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn SetPriorityClass(hprocess: super::super::Foundation::HANDLE, dwpriorityclass: PROCESS_CREATION_FLAGS) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(SetPriorityClass(hprocess.into_param().abi(), ::core::mem::transmute(dwpriorityclass)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_Threading\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn SetProcessAffinityMask<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::HANDLE>>(hprocess: Param0, dwprocessaffinitymask: usize) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn SetProcessAffinityMask(hprocess: super::super::Foundation::HANDLE, dwprocessaffinitymask: usize) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(SetProcessAffinityMask(hprocess.into_param().abi(), ::core::mem::transmute(dwprocessaffinitymask)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_Threading\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn SetProcessAffinityUpdateMode<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::HANDLE>>(hprocess: Param0, dwflags: PROCESS_AFFINITY_AUTO_UPDATE_FLAGS) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn SetProcessAffinityUpdateMode(hprocess: super::super::Foundation::HANDLE, dwflags: PROCESS_AFFINITY_AUTO_UPDATE_FLAGS) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(SetProcessAffinityUpdateMode(hprocess.into_param().abi(), ::core::mem::transmute(dwflags)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_Threading\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn SetProcessDEPPolicy(dwflags: PROCESS_DEP_FLAGS) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn SetProcessDEPPolicy(dwflags: PROCESS_DEP_FLAGS) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(SetProcessDEPPolicy(::core::mem::transmute(dwflags)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_Threading\"`, `\"Win32_Foundation\"`, `\"Win32_System_SystemInformation\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_SystemInformation"))]
#[inline]
pub unsafe fn SetProcessDefaultCpuSetMasks<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::HANDLE>>(process: Param0, cpusetmasks: &[super::SystemInformation::GROUP_AFFINITY]) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn SetProcessDefaultCpuSetMasks(process: super::super::Foundation::HANDLE, cpusetmasks: *const super::SystemInformation::GROUP_AFFINITY, cpusetmaskcount: u16) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(SetProcessDefaultCpuSetMasks(process.into_param().abi(), ::core::mem::transmute(::windows::core::as_ptr_or_null(cpusetmasks)), cpusetmasks.len() as _))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_Threading\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn SetProcessDefaultCpuSets<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::HANDLE>>(process: Param0, cpusetids: &[u32]) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn SetProcessDefaultCpuSets(process: super::super::Foundation::HANDLE, cpusetids: *const u32, cpusetidcount: u32) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(SetProcessDefaultCpuSets(process.into_param().abi(), ::core::mem::transmute(::windows::core::as_ptr_or_null(cpusetids)), cpusetids.len() as _))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_Threading\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn SetProcessDynamicEHContinuationTargets<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::HANDLE>>(process: Param0, targets: &mut [PROCESS_DYNAMIC_EH_CONTINUATION_TARGET]) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn SetProcessDynamicEHContinuationTargets(process: super::super::Foundation::HANDLE, numberoftargets: u16, targets: *mut PROCESS_DYNAMIC_EH_CONTINUATION_TARGET) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(SetProcessDynamicEHContinuationTargets(process.into_param().abi(), targets.len() as _, ::core::mem::transmute(::windows::core::as_mut_ptr_or_null(targets))))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_Threading\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn SetProcessDynamicEnforcedCetCompatibleRanges<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::HANDLE>>(process: Param0, ranges: &mut [PROCESS_DYNAMIC_ENFORCED_ADDRESS_RANGE]) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn SetProcessDynamicEnforcedCetCompatibleRanges(process: super::super::Foundation::HANDLE, numberofranges: u16, ranges: *mut PROCESS_DYNAMIC_ENFORCED_ADDRESS_RANGE) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(SetProcessDynamicEnforcedCetCompatibleRanges(process.into_param().abi(), ranges.len() as _, ::core::mem::transmute(::windows::core::as_mut_ptr_or_null(ranges))))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_Threading\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn SetProcessInformation<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::HANDLE>>(hprocess: Param0, processinformationclass: PROCESS_INFORMATION_CLASS, processinformation: *const ::core::ffi::c_void, processinformationsize: u32) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn SetProcessInformation(hprocess: super::super::Foundation::HANDLE, processinformationclass: PROCESS_INFORMATION_CLASS, processinformation: *const ::core::ffi::c_void, processinformationsize: u32) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(SetProcessInformation(hprocess.into_param().abi(), ::core::mem::transmute(processinformationclass), ::core::mem::transmute(processinformation), ::core::mem::transmute(processinformationsize)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_Threading\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn SetProcessMitigationPolicy(mitigationpolicy: PROCESS_MITIGATION_POLICY, lpbuffer: *const ::core::ffi::c_void, dwlength: usize) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn SetProcessMitigationPolicy(mitigationpolicy: PROCESS_MITIGATION_POLICY, lpbuffer: *const ::core::ffi::c_void, dwlength: usize) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(SetProcessMitigationPolicy(::core::mem::transmute(mitigationpolicy), ::core::mem::transmute(lpbuffer), ::core::mem::transmute(dwlength)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_Threading\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn SetProcessPriorityBoost<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::HANDLE>, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::BOOL>>(hprocess: Param0, bdisablepriorityboost: Param1) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn SetProcessPriorityBoost(hprocess: super::super::Foundation::HANDLE, bdisablepriorityboost: super::super::Foundation::BOOL) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(SetProcessPriorityBoost(hprocess.into_param().abi(), bdisablepriorityboost.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_Threading\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn SetProcessRestrictionExemption<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BOOL>>(fenableexemption: Param0) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn SetProcessRestrictionExemption(fenableexemption: super::super::Foundation::BOOL) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(SetProcessRestrictionExemption(fenableexemption.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_Threading\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn SetProcessShutdownParameters(dwlevel: u32, dwflags: u32) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn SetProcessShutdownParameters(dwlevel: u32, dwflags: u32) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(SetProcessShutdownParameters(::core::mem::transmute(dwlevel), ::core::mem::transmute(dwflags)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_Threading\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn SetProcessWorkingSetSize<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::HANDLE>>(hprocess: Param0, dwminimumworkingsetsize: usize, dwmaximumworkingsetsize: usize) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn SetProcessWorkingSetSize(hprocess: super::super::Foundation::HANDLE, dwminimumworkingsetsize: usize, dwmaximumworkingsetsize: usize) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(SetProcessWorkingSetSize(hprocess.into_param().abi(), ::core::mem::transmute(dwminimumworkingsetsize), ::core::mem::transmute(dwmaximumworkingsetsize)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_Threading\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn SetProtectedPolicy(policyguid: *const ::windows::core::GUID, policyvalue: usize, oldpolicyvalue: *mut usize) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn SetProtectedPolicy(policyguid: *const ::windows::core::GUID, policyvalue: usize, oldpolicyvalue: *mut usize) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(SetProtectedPolicy(::core::mem::transmute(policyguid), ::core::mem::transmute(policyvalue), ::core::mem::transmute(oldpolicyvalue)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_Threading\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn SetThreadAffinityMask<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::HANDLE>>(hthread: Param0, dwthreadaffinitymask: usize) -> usize {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn SetThreadAffinityMask(hthread: super::super::Foundation::HANDLE, dwthreadaffinitymask: usize) -> usize;
        }
        ::core::mem::transmute(SetThreadAffinityMask(hthread.into_param().abi(), ::core::mem::transmute(dwthreadaffinitymask)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_Threading\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn SetThreadDescription<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::HANDLE>, Param1: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>>(hthread: Param0, lpthreaddescription: Param1) -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn SetThreadDescription(hthread: super::super::Foundation::HANDLE, lpthreaddescription: ::windows::core::PCWSTR) -> ::windows::core::HRESULT;
        }
        SetThreadDescription(hthread.into_param().abi(), lpthreaddescription.into_param().abi()).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_Threading\"`, `\"Win32_Foundation\"`, `\"Win32_System_SystemInformation\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_SystemInformation"))]
#[inline]
pub unsafe fn SetThreadGroupAffinity<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::HANDLE>>(hthread: Param0, groupaffinity: *const super::SystemInformation::GROUP_AFFINITY, previousgroupaffinity: *mut super::SystemInformation::GROUP_AFFINITY) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn SetThreadGroupAffinity(hthread: super::super::Foundation::HANDLE, groupaffinity: *const super::SystemInformation::GROUP_AFFINITY, previousgroupaffinity: *mut super::SystemInformation::GROUP_AFFINITY) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(SetThreadGroupAffinity(hthread.into_param().abi(), ::core::mem::transmute(groupaffinity), ::core::mem::transmute(previousgroupaffinity)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_Threading\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn SetThreadIdealProcessor<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::HANDLE>>(hthread: Param0, dwidealprocessor: u32) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn SetThreadIdealProcessor(hthread: super::super::Foundation::HANDLE, dwidealprocessor: u32) -> u32;
        }
        ::core::mem::transmute(SetThreadIdealProcessor(hthread.into_param().abi(), ::core::mem::transmute(dwidealprocessor)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_Threading\"`, `\"Win32_Foundation\"`, `\"Win32_System_Kernel\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Kernel"))]
#[inline]
pub unsafe fn SetThreadIdealProcessorEx<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::HANDLE>>(hthread: Param0, lpidealprocessor: *const super::Kernel::PROCESSOR_NUMBER, lppreviousidealprocessor: *mut super::Kernel::PROCESSOR_NUMBER) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn SetThreadIdealProcessorEx(hthread: super::super::Foundation::HANDLE, lpidealprocessor: *const super::Kernel::PROCESSOR_NUMBER, lppreviousidealprocessor: *mut super::Kernel::PROCESSOR_NUMBER) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(SetThreadIdealProcessorEx(hthread.into_param().abi(), ::core::mem::transmute(lpidealprocessor), ::core::mem::transmute(lppreviousidealprocessor)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_Threading\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn SetThreadInformation<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::HANDLE>>(hthread: Param0, threadinformationclass: THREAD_INFORMATION_CLASS, threadinformation: *const ::core::ffi::c_void, threadinformationsize: u32) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn SetThreadInformation(hthread: super::super::Foundation::HANDLE, threadinformationclass: THREAD_INFORMATION_CLASS, threadinformation: *const ::core::ffi::c_void, threadinformationsize: u32) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(SetThreadInformation(hthread.into_param().abi(), ::core::mem::transmute(threadinformationclass), ::core::mem::transmute(threadinformation), ::core::mem::transmute(threadinformationsize)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_Threading\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn SetThreadPriority<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::HANDLE>>(hthread: Param0, npriority: THREAD_PRIORITY) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn SetThreadPriority(hthread: super::super::Foundation::HANDLE, npriority: THREAD_PRIORITY) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(SetThreadPriority(hthread.into_param().abi(), ::core::mem::transmute(npriority)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_Threading\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn SetThreadPriorityBoost<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::HANDLE>, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::BOOL>>(hthread: Param0, bdisablepriorityboost: Param1) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn SetThreadPriorityBoost(hthread: super::super::Foundation::HANDLE, bdisablepriorityboost: super::super::Foundation::BOOL) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(SetThreadPriorityBoost(hthread.into_param().abi(), bdisablepriorityboost.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_Threading\"`, `\"Win32_Foundation\"`, `\"Win32_System_SystemInformation\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_SystemInformation"))]
#[inline]
pub unsafe fn SetThreadSelectedCpuSetMasks<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::HANDLE>>(thread: Param0, cpusetmasks: &[super::SystemInformation::GROUP_AFFINITY]) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn SetThreadSelectedCpuSetMasks(thread: super::super::Foundation::HANDLE, cpusetmasks: *const super::SystemInformation::GROUP_AFFINITY, cpusetmaskcount: u16) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(SetThreadSelectedCpuSetMasks(thread.into_param().abi(), ::core::mem::transmute(::windows::core::as_ptr_or_null(cpusetmasks)), cpusetmasks.len() as _))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_Threading\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn SetThreadSelectedCpuSets<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::HANDLE>>(thread: Param0, cpusetids: &[u32]) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn SetThreadSelectedCpuSets(thread: super::super::Foundation::HANDLE, cpusetids: *const u32, cpusetidcount: u32) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(SetThreadSelectedCpuSets(thread.into_param().abi(), ::core::mem::transmute(::windows::core::as_ptr_or_null(cpusetids)), cpusetids.len() as _))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_Threading\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn SetThreadStackGuarantee(stacksizeinbytes: *mut u32) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn SetThreadStackGuarantee(stacksizeinbytes: *mut u32) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(SetThreadStackGuarantee(::core::mem::transmute(stacksizeinbytes)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_Threading\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn SetThreadToken<'a, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::HANDLE>>(thread: *const super::super::Foundation::HANDLE, token: Param1) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn SetThreadToken(thread: *const super::super::Foundation::HANDLE, token: super::super::Foundation::HANDLE) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(SetThreadToken(::core::mem::transmute(thread), token.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_Threading\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn SetThreadpoolStackInformation<'a, Param0: ::windows::core::IntoParam<'a, PTP_POOL>>(ptpp: Param0, ptpsi: *const TP_POOL_STACK_INFORMATION) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn SetThreadpoolStackInformation(ptpp: PTP_POOL, ptpsi: *const TP_POOL_STACK_INFORMATION) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(SetThreadpoolStackInformation(ptpp.into_param().abi(), ::core::mem::transmute(ptpsi)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
#[inline]
pub unsafe fn SetThreadpoolThreadMaximum<'a, Param0: ::windows::core::IntoParam<'a, PTP_POOL>>(ptpp: Param0, cthrdmost: u32) {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn SetThreadpoolThreadMaximum(ptpp: PTP_POOL, cthrdmost: u32);
        }
        SetThreadpoolThreadMaximum(ptpp.into_param().abi(), ::core::mem::transmute(cthrdmost))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_Threading\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn SetThreadpoolThreadMinimum<'a, Param0: ::windows::core::IntoParam<'a, PTP_POOL>>(ptpp: Param0, cthrdmic: u32) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn SetThreadpoolThreadMinimum(ptpp: PTP_POOL, cthrdmic: u32) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(SetThreadpoolThreadMinimum(ptpp.into_param().abi(), ::core::mem::transmute(cthrdmic)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_Threading\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn SetThreadpoolTimer(pti: *mut TP_TIMER, pftduetime: *const super::super::Foundation::FILETIME, msperiod: u32, mswindowlength: u32) {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn SetThreadpoolTimer(pti: *mut TP_TIMER, pftduetime: *const super::super::Foundation::FILETIME, msperiod: u32, mswindowlength: u32);
        }
        SetThreadpoolTimer(::core::mem::transmute(pti), ::core::mem::transmute(pftduetime), ::core::mem::transmute(msperiod), ::core::mem::transmute(mswindowlength))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_Threading\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn SetThreadpoolTimerEx(pti: *mut TP_TIMER, pftduetime: *const super::super::Foundation::FILETIME, msperiod: u32, mswindowlength: u32) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn SetThreadpoolTimerEx(pti: *mut TP_TIMER, pftduetime: *const super::super::Foundation::FILETIME, msperiod: u32, mswindowlength: u32) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(SetThreadpoolTimerEx(::core::mem::transmute(pti), ::core::mem::transmute(pftduetime), ::core::mem::transmute(msperiod), ::core::mem::transmute(mswindowlength)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_Threading\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn SetThreadpoolWait<'a, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::HANDLE>>(pwa: *mut TP_WAIT, h: Param1, pfttimeout: *const super::super::Foundation::FILETIME) {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn SetThreadpoolWait(pwa: *mut TP_WAIT, h: super::super::Foundation::HANDLE, pfttimeout: *const super::super::Foundation::FILETIME);
        }
        SetThreadpoolWait(::core::mem::transmute(pwa), h.into_param().abi(), ::core::mem::transmute(pfttimeout))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_Threading\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn SetThreadpoolWaitEx<'a, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::HANDLE>>(pwa: *mut TP_WAIT, h: Param1, pfttimeout: *const super::super::Foundation::FILETIME, reserved: *mut ::core::ffi::c_void) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn SetThreadpoolWaitEx(pwa: *mut TP_WAIT, h: super::super::Foundation::HANDLE, pfttimeout: *const super::super::Foundation::FILETIME, reserved: *mut ::core::ffi::c_void) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(SetThreadpoolWaitEx(::core::mem::transmute(pwa), h.into_param().abi(), ::core::mem::transmute(pfttimeout), ::core::mem::transmute(reserved)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_Threading\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn SetTimerQueueTimer<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::HANDLE>, Param5: ::windows::core::IntoParam<'a, super::super::Foundation::BOOL>>(timerqueue: Param0, callback: WAITORTIMERCALLBACK, parameter: *const ::core::ffi::c_void, duetime: u32, period: u32, preferio: Param5) -> super::super::Foundation::HANDLE {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn SetTimerQueueTimer(timerqueue: super::super::Foundation::HANDLE, callback: ::windows::core::RawPtr, parameter: *const ::core::ffi::c_void, duetime: u32, period: u32, preferio: super::super::Foundation::BOOL) -> super::super::Foundation::HANDLE;
        }
        ::core::mem::transmute(SetTimerQueueTimer(timerqueue.into_param().abi(), ::core::mem::transmute(callback), ::core::mem::transmute(parameter), ::core::mem::transmute(duetime), ::core::mem::transmute(period), preferio.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_Threading\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn SetUmsThreadInformation(umsthread: *const ::core::ffi::c_void, umsthreadinfoclass: RTL_UMS_THREAD_INFO_CLASS, umsthreadinformation: *const ::core::ffi::c_void, umsthreadinformationlength: u32) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn SetUmsThreadInformation(umsthread: *const ::core::ffi::c_void, umsthreadinfoclass: RTL_UMS_THREAD_INFO_CLASS, umsthreadinformation: *const ::core::ffi::c_void, umsthreadinformationlength: u32) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(SetUmsThreadInformation(::core::mem::transmute(umsthread), ::core::mem::transmute(umsthreadinfoclass), ::core::mem::transmute(umsthreadinformation), ::core::mem::transmute(umsthreadinformationlength)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_Threading\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn SetWaitableTimer<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::HANDLE>, Param5: ::windows::core::IntoParam<'a, super::super::Foundation::BOOL>>(htimer: Param0, lpduetime: *const i64, lperiod: i32, pfncompletionroutine: PTIMERAPCROUTINE, lpargtocompletionroutine: *const ::core::ffi::c_void, fresume: Param5) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn SetWaitableTimer(htimer: super::super::Foundation::HANDLE, lpduetime: *const i64, lperiod: i32, pfncompletionroutine: ::windows::core::RawPtr, lpargtocompletionroutine: *const ::core::ffi::c_void, fresume: super::super::Foundation::BOOL) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(SetWaitableTimer(htimer.into_param().abi(), ::core::mem::transmute(lpduetime), ::core::mem::transmute(lperiod), ::core::mem::transmute(pfncompletionroutine), ::core::mem::transmute(lpargtocompletionroutine), fresume.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_Threading\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn SetWaitableTimerEx<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::HANDLE>>(htimer: Param0, lpduetime: *const i64, lperiod: i32, pfncompletionroutine: PTIMERAPCROUTINE, lpargtocompletionroutine: *const ::core::ffi::c_void, wakecontext: *const REASON_CONTEXT, tolerabledelay: u32) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn SetWaitableTimerEx(htimer: super::super::Foundation::HANDLE, lpduetime: *const i64, lperiod: i32, pfncompletionroutine: ::windows::core::RawPtr, lpargtocompletionroutine: *const ::core::ffi::c_void, wakecontext: *const REASON_CONTEXT, tolerabledelay: u32) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(SetWaitableTimerEx(htimer.into_param().abi(), ::core::mem::transmute(lpduetime), ::core::mem::transmute(lperiod), ::core::mem::transmute(pfncompletionroutine), ::core::mem::transmute(lpargtocompletionroutine), ::core::mem::transmute(wakecontext), ::core::mem::transmute(tolerabledelay)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
#[inline]
pub unsafe fn Sleep(dwmilliseconds: u32) {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn Sleep(dwmilliseconds: u32);
        }
        Sleep(::core::mem::transmute(dwmilliseconds))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_Threading\"`, `\"Win32_Foundation\"`, `\"Win32_System_Kernel\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Kernel"))]
#[inline]
pub unsafe fn SleepConditionVariableCS(conditionvariable: *mut RTL_CONDITION_VARIABLE, criticalsection: *mut RTL_CRITICAL_SECTION, dwmilliseconds: u32) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn SleepConditionVariableCS(conditionvariable: *mut RTL_CONDITION_VARIABLE, criticalsection: *mut RTL_CRITICAL_SECTION, dwmilliseconds: u32) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(SleepConditionVariableCS(::core::mem::transmute(conditionvariable), ::core::mem::transmute(criticalsection), ::core::mem::transmute(dwmilliseconds)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_Threading\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn SleepConditionVariableSRW(conditionvariable: *mut RTL_CONDITION_VARIABLE, srwlock: *mut RTL_SRWLOCK, dwmilliseconds: u32, flags: u32) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn SleepConditionVariableSRW(conditionvariable: *mut RTL_CONDITION_VARIABLE, srwlock: *mut RTL_SRWLOCK, dwmilliseconds: u32, flags: u32) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(SleepConditionVariableSRW(::core::mem::transmute(conditionvariable), ::core::mem::transmute(srwlock), ::core::mem::transmute(dwmilliseconds), ::core::mem::transmute(flags)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_Threading\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn SleepEx<'a, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::BOOL>>(dwmilliseconds: u32, balertable: Param1) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn SleepEx(dwmilliseconds: u32, balertable: super::super::Foundation::BOOL) -> u32;
        }
        ::core::mem::transmute(SleepEx(::core::mem::transmute(dwmilliseconds), balertable.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
#[inline]
pub unsafe fn StartThreadpoolIo(pio: *mut TP_IO) {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn StartThreadpoolIo(pio: *mut TP_IO);
        }
        StartThreadpoolIo(::core::mem::transmute(pio))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
#[inline]
pub unsafe fn SubmitThreadpoolWork(pwk: *mut TP_WORK) {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn SubmitThreadpoolWork(pwk: *mut TP_WORK);
        }
        SubmitThreadpoolWork(::core::mem::transmute(pwk))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_Threading\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn SuspendThread<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::HANDLE>>(hthread: Param0) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn SuspendThread(hthread: super::super::Foundation::HANDLE) -> u32;
        }
        ::core::mem::transmute(SuspendThread(hthread.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
#[inline]
pub unsafe fn SwitchToFiber(lpfiber: *const ::core::ffi::c_void) {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn SwitchToFiber(lpfiber: *const ::core::ffi::c_void);
        }
        SwitchToFiber(::core::mem::transmute(lpfiber))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_Threading\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn SwitchToThread() -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn SwitchToThread() -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(SwitchToThread())
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct THREADINFOCLASS(pub i32);
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
pub const ThreadIsIoPending: THREADINFOCLASS = THREADINFOCLASS(16i32);
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
pub const ThreadNameInformation: THREADINFOCLASS = THREADINFOCLASS(38i32);
impl ::core::marker::Copy for THREADINFOCLASS {}
impl ::core::clone::Clone for THREADINFOCLASS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for THREADINFOCLASS {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for THREADINFOCLASS {
    type Abi = Self;
}
impl ::core::fmt::Debug for THREADINFOCLASS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("THREADINFOCLASS").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct THREAD_ACCESS_RIGHTS(pub u32);
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
pub const THREAD_TERMINATE: THREAD_ACCESS_RIGHTS = THREAD_ACCESS_RIGHTS(1u32);
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
pub const THREAD_SUSPEND_RESUME: THREAD_ACCESS_RIGHTS = THREAD_ACCESS_RIGHTS(2u32);
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
pub const THREAD_GET_CONTEXT: THREAD_ACCESS_RIGHTS = THREAD_ACCESS_RIGHTS(8u32);
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
pub const THREAD_SET_CONTEXT: THREAD_ACCESS_RIGHTS = THREAD_ACCESS_RIGHTS(16u32);
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
pub const THREAD_SET_INFORMATION: THREAD_ACCESS_RIGHTS = THREAD_ACCESS_RIGHTS(32u32);
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
pub const THREAD_QUERY_INFORMATION: THREAD_ACCESS_RIGHTS = THREAD_ACCESS_RIGHTS(64u32);
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
pub const THREAD_SET_THREAD_TOKEN: THREAD_ACCESS_RIGHTS = THREAD_ACCESS_RIGHTS(128u32);
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
pub const THREAD_IMPERSONATE: THREAD_ACCESS_RIGHTS = THREAD_ACCESS_RIGHTS(256u32);
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
pub const THREAD_DIRECT_IMPERSONATION: THREAD_ACCESS_RIGHTS = THREAD_ACCESS_RIGHTS(512u32);
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
pub const THREAD_SET_LIMITED_INFORMATION: THREAD_ACCESS_RIGHTS = THREAD_ACCESS_RIGHTS(1024u32);
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
pub const THREAD_QUERY_LIMITED_INFORMATION: THREAD_ACCESS_RIGHTS = THREAD_ACCESS_RIGHTS(2048u32);
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
pub const THREAD_RESUME: THREAD_ACCESS_RIGHTS = THREAD_ACCESS_RIGHTS(4096u32);
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
pub const THREAD_ALL_ACCESS: THREAD_ACCESS_RIGHTS = THREAD_ACCESS_RIGHTS(2097151u32);
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
pub const THREAD_DELETE: THREAD_ACCESS_RIGHTS = THREAD_ACCESS_RIGHTS(65536u32);
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
pub const THREAD_READ_CONTROL: THREAD_ACCESS_RIGHTS = THREAD_ACCESS_RIGHTS(131072u32);
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
pub const THREAD_WRITE_DAC: THREAD_ACCESS_RIGHTS = THREAD_ACCESS_RIGHTS(262144u32);
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
pub const THREAD_WRITE_OWNER: THREAD_ACCESS_RIGHTS = THREAD_ACCESS_RIGHTS(524288u32);
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
pub const THREAD_SYNCHRONIZE: THREAD_ACCESS_RIGHTS = THREAD_ACCESS_RIGHTS(1048576u32);
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
pub const THREAD_STANDARD_RIGHTS_REQUIRED: THREAD_ACCESS_RIGHTS = THREAD_ACCESS_RIGHTS(983040u32);
impl ::core::marker::Copy for THREAD_ACCESS_RIGHTS {}
impl ::core::clone::Clone for THREAD_ACCESS_RIGHTS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for THREAD_ACCESS_RIGHTS {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for THREAD_ACCESS_RIGHTS {
    type Abi = Self;
}
impl ::core::fmt::Debug for THREAD_ACCESS_RIGHTS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("THREAD_ACCESS_RIGHTS").field(&self.0).finish()
    }
}
impl ::core::ops::BitOr for THREAD_ACCESS_RIGHTS {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for THREAD_ACCESS_RIGHTS {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for THREAD_ACCESS_RIGHTS {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for THREAD_ACCESS_RIGHTS {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for THREAD_ACCESS_RIGHTS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct THREAD_CREATION_FLAGS(pub u32);
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
pub const THREAD_CREATE_RUN_IMMEDIATELY: THREAD_CREATION_FLAGS = THREAD_CREATION_FLAGS(0u32);
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
pub const THREAD_CREATE_SUSPENDED: THREAD_CREATION_FLAGS = THREAD_CREATION_FLAGS(4u32);
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
pub const STACK_SIZE_PARAM_IS_A_RESERVATION: THREAD_CREATION_FLAGS = THREAD_CREATION_FLAGS(65536u32);
impl ::core::marker::Copy for THREAD_CREATION_FLAGS {}
impl ::core::clone::Clone for THREAD_CREATION_FLAGS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for THREAD_CREATION_FLAGS {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for THREAD_CREATION_FLAGS {
    type Abi = Self;
}
impl ::core::fmt::Debug for THREAD_CREATION_FLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("THREAD_CREATION_FLAGS").field(&self.0).finish()
    }
}
impl ::core::ops::BitOr for THREAD_CREATION_FLAGS {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for THREAD_CREATION_FLAGS {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for THREAD_CREATION_FLAGS {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for THREAD_CREATION_FLAGS {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for THREAD_CREATION_FLAGS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct THREAD_INFORMATION_CLASS(pub i32);
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
pub const ThreadMemoryPriority: THREAD_INFORMATION_CLASS = THREAD_INFORMATION_CLASS(0i32);
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
pub const ThreadAbsoluteCpuPriority: THREAD_INFORMATION_CLASS = THREAD_INFORMATION_CLASS(1i32);
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
pub const ThreadDynamicCodePolicy: THREAD_INFORMATION_CLASS = THREAD_INFORMATION_CLASS(2i32);
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
pub const ThreadPowerThrottling: THREAD_INFORMATION_CLASS = THREAD_INFORMATION_CLASS(3i32);
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
pub const ThreadInformationClassMax: THREAD_INFORMATION_CLASS = THREAD_INFORMATION_CLASS(4i32);
impl ::core::marker::Copy for THREAD_INFORMATION_CLASS {}
impl ::core::clone::Clone for THREAD_INFORMATION_CLASS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for THREAD_INFORMATION_CLASS {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for THREAD_INFORMATION_CLASS {
    type Abi = Self;
}
impl ::core::fmt::Debug for THREAD_INFORMATION_CLASS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("THREAD_INFORMATION_CLASS").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
pub const THREAD_POWER_THROTTLING_CURRENT_VERSION: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
pub const THREAD_POWER_THROTTLING_EXECUTION_SPEED: u32 = 1u32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
pub struct THREAD_POWER_THROTTLING_STATE {
    pub Version: u32,
    pub ControlMask: u32,
    pub StateMask: u32,
}
impl ::core::marker::Copy for THREAD_POWER_THROTTLING_STATE {}
impl ::core::clone::Clone for THREAD_POWER_THROTTLING_STATE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for THREAD_POWER_THROTTLING_STATE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("THREAD_POWER_THROTTLING_STATE").field("Version", &self.Version).field("ControlMask", &self.ControlMask).field("StateMask", &self.StateMask).finish()
    }
}
unsafe impl ::windows::core::Abi for THREAD_POWER_THROTTLING_STATE {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for THREAD_POWER_THROTTLING_STATE {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<THREAD_POWER_THROTTLING_STATE>()) == 0 }
    }
}
impl ::core::cmp::Eq for THREAD_POWER_THROTTLING_STATE {}
impl ::core::default::Default for THREAD_POWER_THROTTLING_STATE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
pub const THREAD_POWER_THROTTLING_VALID_FLAGS: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct THREAD_PRIORITY(pub i32);
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
pub const THREAD_MODE_BACKGROUND_BEGIN: THREAD_PRIORITY = THREAD_PRIORITY(65536i32);
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
pub const THREAD_MODE_BACKGROUND_END: THREAD_PRIORITY = THREAD_PRIORITY(131072i32);
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
pub const THREAD_PRIORITY_ABOVE_NORMAL: THREAD_PRIORITY = THREAD_PRIORITY(1i32);
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
pub const THREAD_PRIORITY_BELOW_NORMAL: THREAD_PRIORITY = THREAD_PRIORITY(-1i32);
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
pub const THREAD_PRIORITY_HIGHEST: THREAD_PRIORITY = THREAD_PRIORITY(2i32);
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
pub const THREAD_PRIORITY_IDLE: THREAD_PRIORITY = THREAD_PRIORITY(-15i32);
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
pub const THREAD_PRIORITY_MIN: THREAD_PRIORITY = THREAD_PRIORITY(-2i32);
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
pub const THREAD_PRIORITY_LOWEST: THREAD_PRIORITY = THREAD_PRIORITY(-2i32);
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
pub const THREAD_PRIORITY_NORMAL: THREAD_PRIORITY = THREAD_PRIORITY(0i32);
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
pub const THREAD_PRIORITY_TIME_CRITICAL: THREAD_PRIORITY = THREAD_PRIORITY(15i32);
impl ::core::marker::Copy for THREAD_PRIORITY {}
impl ::core::clone::Clone for THREAD_PRIORITY {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for THREAD_PRIORITY {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for THREAD_PRIORITY {
    type Abi = Self;
}
impl ::core::fmt::Debug for THREAD_PRIORITY {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("THREAD_PRIORITY").field(&self.0).finish()
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
pub struct TP_CALLBACK_ENVIRON_V3 {
    pub Version: u32,
    pub Pool: PTP_POOL,
    pub CleanupGroup: isize,
    pub CleanupGroupCancelCallback: PTP_CLEANUP_GROUP_CANCEL_CALLBACK,
    pub RaceDll: *mut ::core::ffi::c_void,
    pub ActivationContext: isize,
    pub FinalizationCallback: PTP_SIMPLE_CALLBACK,
    pub u: TP_CALLBACK_ENVIRON_V3_1,
    pub CallbackPriority: TP_CALLBACK_PRIORITY,
    pub Size: u32,
}
impl ::core::marker::Copy for TP_CALLBACK_ENVIRON_V3 {}
impl ::core::clone::Clone for TP_CALLBACK_ENVIRON_V3 {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for TP_CALLBACK_ENVIRON_V3 {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for TP_CALLBACK_ENVIRON_V3 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<TP_CALLBACK_ENVIRON_V3>()) == 0 }
    }
}
impl ::core::cmp::Eq for TP_CALLBACK_ENVIRON_V3 {}
impl ::core::default::Default for TP_CALLBACK_ENVIRON_V3 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct TP_CALLBACK_ENVIRON_V3_0(pub u8);
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
pub union TP_CALLBACK_ENVIRON_V3_1 {
    pub Flags: u32,
    pub s: TP_CALLBACK_ENVIRON_V3_1_0,
}
impl ::core::marker::Copy for TP_CALLBACK_ENVIRON_V3_1 {}
impl ::core::clone::Clone for TP_CALLBACK_ENVIRON_V3_1 {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for TP_CALLBACK_ENVIRON_V3_1 {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for TP_CALLBACK_ENVIRON_V3_1 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<TP_CALLBACK_ENVIRON_V3_1>()) == 0 }
    }
}
impl ::core::cmp::Eq for TP_CALLBACK_ENVIRON_V3_1 {}
impl ::core::default::Default for TP_CALLBACK_ENVIRON_V3_1 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
pub struct TP_CALLBACK_ENVIRON_V3_1_0 {
    pub _bitfield: u32,
}
impl ::core::marker::Copy for TP_CALLBACK_ENVIRON_V3_1_0 {}
impl ::core::clone::Clone for TP_CALLBACK_ENVIRON_V3_1_0 {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for TP_CALLBACK_ENVIRON_V3_1_0 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("TP_CALLBACK_ENVIRON_V3_1_0").field("_bitfield", &self._bitfield).finish()
    }
}
unsafe impl ::windows::core::Abi for TP_CALLBACK_ENVIRON_V3_1_0 {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for TP_CALLBACK_ENVIRON_V3_1_0 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<TP_CALLBACK_ENVIRON_V3_1_0>()) == 0 }
    }
}
impl ::core::cmp::Eq for TP_CALLBACK_ENVIRON_V3_1_0 {}
impl ::core::default::Default for TP_CALLBACK_ENVIRON_V3_1_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct TP_CALLBACK_INSTANCE(pub u8);
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct TP_CALLBACK_PRIORITY(pub i32);
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
pub const TP_CALLBACK_PRIORITY_HIGH: TP_CALLBACK_PRIORITY = TP_CALLBACK_PRIORITY(0i32);
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
pub const TP_CALLBACK_PRIORITY_NORMAL: TP_CALLBACK_PRIORITY = TP_CALLBACK_PRIORITY(1i32);
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
pub const TP_CALLBACK_PRIORITY_LOW: TP_CALLBACK_PRIORITY = TP_CALLBACK_PRIORITY(2i32);
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
pub const TP_CALLBACK_PRIORITY_INVALID: TP_CALLBACK_PRIORITY = TP_CALLBACK_PRIORITY(3i32);
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
pub const TP_CALLBACK_PRIORITY_COUNT: TP_CALLBACK_PRIORITY = TP_CALLBACK_PRIORITY(3i32);
impl ::core::marker::Copy for TP_CALLBACK_PRIORITY {}
impl ::core::clone::Clone for TP_CALLBACK_PRIORITY {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for TP_CALLBACK_PRIORITY {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for TP_CALLBACK_PRIORITY {
    type Abi = Self;
}
impl ::core::fmt::Debug for TP_CALLBACK_PRIORITY {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("TP_CALLBACK_PRIORITY").field(&self.0).finish()
    }
}
#[repr(C)]
pub struct TP_IO(pub u8);
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
pub struct TP_POOL_STACK_INFORMATION {
    pub StackReserve: usize,
    pub StackCommit: usize,
}
impl ::core::marker::Copy for TP_POOL_STACK_INFORMATION {}
impl ::core::clone::Clone for TP_POOL_STACK_INFORMATION {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for TP_POOL_STACK_INFORMATION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("TP_POOL_STACK_INFORMATION").field("StackReserve", &self.StackReserve).field("StackCommit", &self.StackCommit).finish()
    }
}
unsafe impl ::windows::core::Abi for TP_POOL_STACK_INFORMATION {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for TP_POOL_STACK_INFORMATION {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<TP_POOL_STACK_INFORMATION>()) == 0 }
    }
}
impl ::core::cmp::Eq for TP_POOL_STACK_INFORMATION {}
impl ::core::default::Default for TP_POOL_STACK_INFORMATION {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct TP_TIMER(pub u8);
#[repr(C)]
pub struct TP_WAIT(pub u8);
#[repr(C)]
pub struct TP_WORK(pub u8);
#[doc = "*Required features: `\"Win32_System_Threading\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn TerminateProcess<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::HANDLE>>(hprocess: Param0, uexitcode: u32) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn TerminateProcess(hprocess: super::super::Foundation::HANDLE, uexitcode: u32) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(TerminateProcess(hprocess.into_param().abi(), ::core::mem::transmute(uexitcode)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_Threading\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn TerminateThread<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::HANDLE>>(hthread: Param0, dwexitcode: u32) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn TerminateThread(hthread: super::super::Foundation::HANDLE, dwexitcode: u32) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(TerminateThread(hthread.into_param().abi(), ::core::mem::transmute(dwexitcode)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct TimerQueueHandle(pub isize);
impl TimerQueueHandle {
    pub fn is_invalid(&self) -> bool {
        *self == unsafe { ::core::mem::zeroed() }
    }
    pub fn ok(self) -> ::windows::core::Result<Self> {
        if !self.is_invalid() {
            Ok(self)
        } else {
            Err(::windows::core::Error::from_win32())
        }
    }
}
impl ::core::default::Default for TimerQueueHandle {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::clone::Clone for TimerQueueHandle {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::marker::Copy for TimerQueueHandle {}
impl ::core::fmt::Debug for TimerQueueHandle {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("TimerQueueHandle").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Abi for TimerQueueHandle {
    type Abi = Self;
}
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
#[inline]
pub unsafe fn TlsAlloc() -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn TlsAlloc() -> u32;
        }
        ::core::mem::transmute(TlsAlloc())
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_Threading\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn TlsFree(dwtlsindex: u32) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn TlsFree(dwtlsindex: u32) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(TlsFree(::core::mem::transmute(dwtlsindex)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
#[inline]
pub unsafe fn TlsGetValue(dwtlsindex: u32) -> *mut ::core::ffi::c_void {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn TlsGetValue(dwtlsindex: u32) -> *mut ::core::ffi::c_void;
        }
        ::core::mem::transmute(TlsGetValue(::core::mem::transmute(dwtlsindex)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_Threading\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn TlsSetValue(dwtlsindex: u32, lptlsvalue: *const ::core::ffi::c_void) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn TlsSetValue(dwtlsindex: u32, lptlsvalue: *const ::core::ffi::c_void) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(TlsSetValue(::core::mem::transmute(dwtlsindex), ::core::mem::transmute(lptlsvalue)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_Threading\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn TryAcquireSRWLockExclusive(srwlock: *mut RTL_SRWLOCK) -> super::super::Foundation::BOOLEAN {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn TryAcquireSRWLockExclusive(srwlock: *mut RTL_SRWLOCK) -> super::super::Foundation::BOOLEAN;
        }
        ::core::mem::transmute(TryAcquireSRWLockExclusive(::core::mem::transmute(srwlock)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_Threading\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn TryAcquireSRWLockShared(srwlock: *mut RTL_SRWLOCK) -> super::super::Foundation::BOOLEAN {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn TryAcquireSRWLockShared(srwlock: *mut RTL_SRWLOCK) -> super::super::Foundation::BOOLEAN;
        }
        ::core::mem::transmute(TryAcquireSRWLockShared(::core::mem::transmute(srwlock)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_Threading\"`, `\"Win32_Foundation\"`, `\"Win32_System_Kernel\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Kernel"))]
#[inline]
pub unsafe fn TryEnterCriticalSection(lpcriticalsection: *mut RTL_CRITICAL_SECTION) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn TryEnterCriticalSection(lpcriticalsection: *mut RTL_CRITICAL_SECTION) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(TryEnterCriticalSection(::core::mem::transmute(lpcriticalsection)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_Threading\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn TrySubmitThreadpoolCallback(pfns: PTP_SIMPLE_CALLBACK, pv: *mut ::core::ffi::c_void, pcbe: *const TP_CALLBACK_ENVIRON_V3) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn TrySubmitThreadpoolCallback(pfns: ::windows::core::RawPtr, pv: *mut ::core::ffi::c_void, pcbe: *const TP_CALLBACK_ENVIRON_V3) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(TrySubmitThreadpoolCallback(::core::mem::transmute(pfns), ::core::mem::transmute(pv), ::core::mem::transmute(pcbe)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Threading\"`, `\"Win32_System_SystemServices\"`*"]
#[cfg(feature = "Win32_System_SystemServices")]
pub struct UMS_SCHEDULER_STARTUP_INFO {
    pub UmsVersion: u32,
    pub CompletionList: *mut ::core::ffi::c_void,
    pub SchedulerProc: PRTL_UMS_SCHEDULER_ENTRY_POINT,
    pub SchedulerParam: *mut ::core::ffi::c_void,
}
#[cfg(feature = "Win32_System_SystemServices")]
impl ::core::marker::Copy for UMS_SCHEDULER_STARTUP_INFO {}
#[cfg(feature = "Win32_System_SystemServices")]
impl ::core::clone::Clone for UMS_SCHEDULER_STARTUP_INFO {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_System_SystemServices")]
impl ::core::fmt::Debug for UMS_SCHEDULER_STARTUP_INFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("UMS_SCHEDULER_STARTUP_INFO").field("UmsVersion", &self.UmsVersion).field("CompletionList", &self.CompletionList).field("SchedulerProc", &self.SchedulerProc.map(|f| f as usize)).field("SchedulerParam", &self.SchedulerParam).finish()
    }
}
#[cfg(feature = "Win32_System_SystemServices")]
unsafe impl ::windows::core::Abi for UMS_SCHEDULER_STARTUP_INFO {
    type Abi = Self;
}
#[cfg(feature = "Win32_System_SystemServices")]
impl ::core::cmp::PartialEq for UMS_SCHEDULER_STARTUP_INFO {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<UMS_SCHEDULER_STARTUP_INFO>()) == 0 }
    }
}
#[cfg(feature = "Win32_System_SystemServices")]
impl ::core::cmp::Eq for UMS_SCHEDULER_STARTUP_INFO {}
#[cfg(feature = "Win32_System_SystemServices")]
impl ::core::default::Default for UMS_SCHEDULER_STARTUP_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
pub struct UMS_SYSTEM_THREAD_INFORMATION {
    pub UmsVersion: u32,
    pub Anonymous: UMS_SYSTEM_THREAD_INFORMATION_0,
}
impl ::core::marker::Copy for UMS_SYSTEM_THREAD_INFORMATION {}
impl ::core::clone::Clone for UMS_SYSTEM_THREAD_INFORMATION {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for UMS_SYSTEM_THREAD_INFORMATION {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for UMS_SYSTEM_THREAD_INFORMATION {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<UMS_SYSTEM_THREAD_INFORMATION>()) == 0 }
    }
}
impl ::core::cmp::Eq for UMS_SYSTEM_THREAD_INFORMATION {}
impl ::core::default::Default for UMS_SYSTEM_THREAD_INFORMATION {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
pub union UMS_SYSTEM_THREAD_INFORMATION_0 {
    pub Anonymous: UMS_SYSTEM_THREAD_INFORMATION_0_0,
    pub ThreadUmsFlags: u32,
}
impl ::core::marker::Copy for UMS_SYSTEM_THREAD_INFORMATION_0 {}
impl ::core::clone::Clone for UMS_SYSTEM_THREAD_INFORMATION_0 {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for UMS_SYSTEM_THREAD_INFORMATION_0 {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for UMS_SYSTEM_THREAD_INFORMATION_0 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<UMS_SYSTEM_THREAD_INFORMATION_0>()) == 0 }
    }
}
impl ::core::cmp::Eq for UMS_SYSTEM_THREAD_INFORMATION_0 {}
impl ::core::default::Default for UMS_SYSTEM_THREAD_INFORMATION_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
pub struct UMS_SYSTEM_THREAD_INFORMATION_0_0 {
    pub _bitfield: u32,
}
impl ::core::marker::Copy for UMS_SYSTEM_THREAD_INFORMATION_0_0 {}
impl ::core::clone::Clone for UMS_SYSTEM_THREAD_INFORMATION_0_0 {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for UMS_SYSTEM_THREAD_INFORMATION_0_0 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("UMS_SYSTEM_THREAD_INFORMATION_0_0").field("_bitfield", &self._bitfield).finish()
    }
}
unsafe impl ::windows::core::Abi for UMS_SYSTEM_THREAD_INFORMATION_0_0 {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for UMS_SYSTEM_THREAD_INFORMATION_0_0 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<UMS_SYSTEM_THREAD_INFORMATION_0_0>()) == 0 }
    }
}
impl ::core::cmp::Eq for UMS_SYSTEM_THREAD_INFORMATION_0_0 {}
impl ::core::default::Default for UMS_SYSTEM_THREAD_INFORMATION_0_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_System_Threading\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn UmsThreadYield(schedulerparam: *const ::core::ffi::c_void) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn UmsThreadYield(schedulerparam: *const ::core::ffi::c_void) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(UmsThreadYield(::core::mem::transmute(schedulerparam)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_Threading\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn UnregisterWait<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::HANDLE>>(waithandle: Param0) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn UnregisterWait(waithandle: super::super::Foundation::HANDLE) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(UnregisterWait(waithandle.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_Threading\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn UnregisterWaitEx<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::HANDLE>, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::HANDLE>>(waithandle: Param0, completionevent: Param1) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn UnregisterWaitEx(waithandle: super::super::Foundation::HANDLE, completionevent: super::super::Foundation::HANDLE) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(UnregisterWaitEx(waithandle.into_param().abi(), completionevent.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_Threading\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn UpdateProcThreadAttribute<'a, Param0: ::windows::core::IntoParam<'a, LPPROC_THREAD_ATTRIBUTE_LIST>>(lpattributelist: Param0, dwflags: u32, attribute: usize, lpvalue: *const ::core::ffi::c_void, cbsize: usize, lppreviousvalue: *mut ::core::ffi::c_void, lpreturnsize: *const usize) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn UpdateProcThreadAttribute(lpattributelist: LPPROC_THREAD_ATTRIBUTE_LIST, dwflags: u32, attribute: usize, lpvalue: *const ::core::ffi::c_void, cbsize: usize, lppreviousvalue: *mut ::core::ffi::c_void, lpreturnsize: *const usize) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(UpdateProcThreadAttribute(lpattributelist.into_param().abi(), ::core::mem::transmute(dwflags), ::core::mem::transmute(attribute), ::core::mem::transmute(lpvalue), ::core::mem::transmute(cbsize), ::core::mem::transmute(lppreviousvalue), ::core::mem::transmute(lpreturnsize)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_Threading\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub type WAITORTIMERCALLBACK = ::core::option::Option<unsafe extern "system" fn(param0: *mut ::core::ffi::c_void, param1: super::super::Foundation::BOOLEAN)>;
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
pub const WAIT_ABANDONED: u32 = 128u32;
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
pub const WAIT_ABANDONED_0: u32 = 128u32;
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
pub const WAIT_IO_COMPLETION: u32 = 192u32;
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
pub const WAIT_OBJECT_0: u32 = 0u32;
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct WORKER_THREAD_FLAGS(pub u32);
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
pub const WT_EXECUTEDEFAULT: WORKER_THREAD_FLAGS = WORKER_THREAD_FLAGS(0u32);
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
pub const WT_EXECUTEINIOTHREAD: WORKER_THREAD_FLAGS = WORKER_THREAD_FLAGS(1u32);
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
pub const WT_EXECUTEINPERSISTENTTHREAD: WORKER_THREAD_FLAGS = WORKER_THREAD_FLAGS(128u32);
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
pub const WT_EXECUTEINWAITTHREAD: WORKER_THREAD_FLAGS = WORKER_THREAD_FLAGS(4u32);
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
pub const WT_EXECUTELONGFUNCTION: WORKER_THREAD_FLAGS = WORKER_THREAD_FLAGS(16u32);
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
pub const WT_EXECUTEONLYONCE: WORKER_THREAD_FLAGS = WORKER_THREAD_FLAGS(8u32);
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
pub const WT_TRANSFER_IMPERSONATION: WORKER_THREAD_FLAGS = WORKER_THREAD_FLAGS(256u32);
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
pub const WT_EXECUTEINTIMERTHREAD: WORKER_THREAD_FLAGS = WORKER_THREAD_FLAGS(32u32);
impl ::core::marker::Copy for WORKER_THREAD_FLAGS {}
impl ::core::clone::Clone for WORKER_THREAD_FLAGS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for WORKER_THREAD_FLAGS {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for WORKER_THREAD_FLAGS {
    type Abi = Self;
}
impl ::core::fmt::Debug for WORKER_THREAD_FLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WORKER_THREAD_FLAGS").field(&self.0).finish()
    }
}
impl ::core::ops::BitOr for WORKER_THREAD_FLAGS {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for WORKER_THREAD_FLAGS {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for WORKER_THREAD_FLAGS {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for WORKER_THREAD_FLAGS {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for WORKER_THREAD_FLAGS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[doc = "*Required features: `\"Win32_System_Threading\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn WaitForInputIdle<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::HANDLE>>(hprocess: Param0, dwmilliseconds: u32) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn WaitForInputIdle(hprocess: super::super::Foundation::HANDLE, dwmilliseconds: u32) -> u32;
        }
        ::core::mem::transmute(WaitForInputIdle(hprocess.into_param().abi(), ::core::mem::transmute(dwmilliseconds)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_Threading\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn WaitForMultipleObjects<'a, Param2: ::windows::core::IntoParam<'a, super::super::Foundation::BOOL>>(lphandles: &[super::super::Foundation::HANDLE], bwaitall: Param2, dwmilliseconds: u32) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn WaitForMultipleObjects(ncount: u32, lphandles: *const super::super::Foundation::HANDLE, bwaitall: super::super::Foundation::BOOL, dwmilliseconds: u32) -> u32;
        }
        ::core::mem::transmute(WaitForMultipleObjects(lphandles.len() as _, ::core::mem::transmute(::windows::core::as_ptr_or_null(lphandles)), bwaitall.into_param().abi(), ::core::mem::transmute(dwmilliseconds)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_Threading\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn WaitForMultipleObjectsEx<'a, Param2: ::windows::core::IntoParam<'a, super::super::Foundation::BOOL>, Param4: ::windows::core::IntoParam<'a, super::super::Foundation::BOOL>>(lphandles: &[super::super::Foundation::HANDLE], bwaitall: Param2, dwmilliseconds: u32, balertable: Param4) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn WaitForMultipleObjectsEx(ncount: u32, lphandles: *const super::super::Foundation::HANDLE, bwaitall: super::super::Foundation::BOOL, dwmilliseconds: u32, balertable: super::super::Foundation::BOOL) -> u32;
        }
        ::core::mem::transmute(WaitForMultipleObjectsEx(lphandles.len() as _, ::core::mem::transmute(::windows::core::as_ptr_or_null(lphandles)), bwaitall.into_param().abi(), ::core::mem::transmute(dwmilliseconds), balertable.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_Threading\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn WaitForSingleObject<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::HANDLE>>(hhandle: Param0, dwmilliseconds: u32) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn WaitForSingleObject(hhandle: super::super::Foundation::HANDLE, dwmilliseconds: u32) -> u32;
        }
        ::core::mem::transmute(WaitForSingleObject(hhandle.into_param().abi(), ::core::mem::transmute(dwmilliseconds)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_Threading\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn WaitForSingleObjectEx<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::HANDLE>, Param2: ::windows::core::IntoParam<'a, super::super::Foundation::BOOL>>(hhandle: Param0, dwmilliseconds: u32, balertable: Param2) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn WaitForSingleObjectEx(hhandle: super::super::Foundation::HANDLE, dwmilliseconds: u32, balertable: super::super::Foundation::BOOL) -> u32;
        }
        ::core::mem::transmute(WaitForSingleObjectEx(hhandle.into_param().abi(), ::core::mem::transmute(dwmilliseconds), balertable.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_Threading\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn WaitForThreadpoolIoCallbacks<'a, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::BOOL>>(pio: *mut TP_IO, fcancelpendingcallbacks: Param1) {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn WaitForThreadpoolIoCallbacks(pio: *mut TP_IO, fcancelpendingcallbacks: super::super::Foundation::BOOL);
        }
        WaitForThreadpoolIoCallbacks(::core::mem::transmute(pio), fcancelpendingcallbacks.into_param().abi())
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_Threading\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn WaitForThreadpoolTimerCallbacks<'a, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::BOOL>>(pti: *mut TP_TIMER, fcancelpendingcallbacks: Param1) {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn WaitForThreadpoolTimerCallbacks(pti: *mut TP_TIMER, fcancelpendingcallbacks: super::super::Foundation::BOOL);
        }
        WaitForThreadpoolTimerCallbacks(::core::mem::transmute(pti), fcancelpendingcallbacks.into_param().abi())
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_Threading\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn WaitForThreadpoolWaitCallbacks<'a, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::BOOL>>(pwa: *mut TP_WAIT, fcancelpendingcallbacks: Param1) {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn WaitForThreadpoolWaitCallbacks(pwa: *mut TP_WAIT, fcancelpendingcallbacks: super::super::Foundation::BOOL);
        }
        WaitForThreadpoolWaitCallbacks(::core::mem::transmute(pwa), fcancelpendingcallbacks.into_param().abi())
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_Threading\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn WaitForThreadpoolWorkCallbacks<'a, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::BOOL>>(pwk: *mut TP_WORK, fcancelpendingcallbacks: Param1) {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn WaitForThreadpoolWorkCallbacks(pwk: *mut TP_WORK, fcancelpendingcallbacks: super::super::Foundation::BOOL);
        }
        WaitForThreadpoolWorkCallbacks(::core::mem::transmute(pwk), fcancelpendingcallbacks.into_param().abi())
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_Threading\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn WaitOnAddress(address: *const ::core::ffi::c_void, compareaddress: *const ::core::ffi::c_void, addresssize: usize, dwmilliseconds: u32) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn WaitOnAddress(address: *const ::core::ffi::c_void, compareaddress: *const ::core::ffi::c_void, addresssize: usize, dwmilliseconds: u32) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(WaitOnAddress(::core::mem::transmute(address), ::core::mem::transmute(compareaddress), ::core::mem::transmute(addresssize), ::core::mem::transmute(dwmilliseconds)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
#[inline]
pub unsafe fn WakeAllConditionVariable(conditionvariable: *mut RTL_CONDITION_VARIABLE) {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn WakeAllConditionVariable(conditionvariable: *mut RTL_CONDITION_VARIABLE);
        }
        WakeAllConditionVariable(::core::mem::transmute(conditionvariable))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
#[inline]
pub unsafe fn WakeByAddressAll(address: *const ::core::ffi::c_void) {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn WakeByAddressAll(address: *const ::core::ffi::c_void);
        }
        WakeByAddressAll(::core::mem::transmute(address))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
#[inline]
pub unsafe fn WakeByAddressSingle(address: *const ::core::ffi::c_void) {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn WakeByAddressSingle(address: *const ::core::ffi::c_void);
        }
        WakeByAddressSingle(::core::mem::transmute(address))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
#[inline]
pub unsafe fn WakeConditionVariable(conditionvariable: *mut RTL_CONDITION_VARIABLE) {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn WakeConditionVariable(conditionvariable: *mut RTL_CONDITION_VARIABLE);
        }
        WakeConditionVariable(::core::mem::transmute(conditionvariable))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
#[inline]
pub unsafe fn WinExec<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::PCSTR>>(lpcmdline: Param0, ucmdshow: u32) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn WinExec(lpcmdline: ::windows::core::PCSTR, ucmdshow: u32) -> u32;
        }
        ::core::mem::transmute(WinExec(lpcmdline.into_param().abi(), ::core::mem::transmute(ucmdshow)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_Threading\"`*"]
#[inline]
pub unsafe fn Wow64SetThreadDefaultGuestMachine(machine: u16) -> u16 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn Wow64SetThreadDefaultGuestMachine(machine: u16) -> u16;
        }
        ::core::mem::transmute(Wow64SetThreadDefaultGuestMachine(::core::mem::transmute(machine)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_Threading\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn Wow64SuspendThread<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::HANDLE>>(hthread: Param0) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn Wow64SuspendThread(hthread: super::super::Foundation::HANDLE) -> u32;
        }
        ::core::mem::transmute(Wow64SuspendThread(hthread.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "implement")]
::core::include!("impl.rs");

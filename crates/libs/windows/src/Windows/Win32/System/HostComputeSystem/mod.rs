#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, clashing_extern_declarations, clippy::all)]
#[doc = "*Required features: `\"Win32_System_HostComputeSystem\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct HCS_CREATE_OPTIONS(pub i32);
#[doc = "*Required features: `\"Win32_System_HostComputeSystem\"`*"]
pub const HcsCreateOptions_1: HCS_CREATE_OPTIONS = HCS_CREATE_OPTIONS(65536i32);
impl ::core::marker::Copy for HCS_CREATE_OPTIONS {}
impl ::core::clone::Clone for HCS_CREATE_OPTIONS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for HCS_CREATE_OPTIONS {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for HCS_CREATE_OPTIONS {
    type Abi = Self;
}
impl ::core::fmt::Debug for HCS_CREATE_OPTIONS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("HCS_CREATE_OPTIONS").field(&self.0).finish()
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_HostComputeSystem\"`, `\"Win32_Foundation\"`, `\"Win32_Security\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
pub struct HCS_CREATE_OPTIONS_1 {
    pub Version: HCS_CREATE_OPTIONS,
    pub UserToken: super::super::Foundation::HANDLE,
    pub SecurityDescriptor: *mut super::super::Security::SECURITY_DESCRIPTOR,
    pub CallbackOptions: HCS_EVENT_OPTIONS,
    pub CallbackContext: *mut ::core::ffi::c_void,
    pub Callback: HCS_EVENT_CALLBACK,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
impl ::core::marker::Copy for HCS_CREATE_OPTIONS_1 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
impl ::core::clone::Clone for HCS_CREATE_OPTIONS_1 {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
impl ::core::fmt::Debug for HCS_CREATE_OPTIONS_1 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("HCS_CREATE_OPTIONS_1").field("Version", &self.Version).field("UserToken", &self.UserToken).field("SecurityDescriptor", &self.SecurityDescriptor).field("CallbackOptions", &self.CallbackOptions).field("CallbackContext", &self.CallbackContext).field("Callback", &self.Callback.map(|f| f as usize)).finish()
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
unsafe impl ::windows::core::Abi for HCS_CREATE_OPTIONS_1 {
    type Abi = Self;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
impl ::core::cmp::PartialEq for HCS_CREATE_OPTIONS_1 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<HCS_CREATE_OPTIONS_1>()) == 0 }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
impl ::core::cmp::Eq for HCS_CREATE_OPTIONS_1 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
impl ::core::default::Default for HCS_CREATE_OPTIONS_1 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_HostComputeSystem\"`*"]
pub struct HCS_EVENT {
    pub Type: HCS_EVENT_TYPE,
    pub EventData: ::windows::core::PCWSTR,
    pub Operation: HCS_OPERATION,
}
impl ::core::marker::Copy for HCS_EVENT {}
impl ::core::clone::Clone for HCS_EVENT {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for HCS_EVENT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("HCS_EVENT").field("Type", &self.Type).field("EventData", &self.EventData).field("Operation", &self.Operation).finish()
    }
}
unsafe impl ::windows::core::Abi for HCS_EVENT {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for HCS_EVENT {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<HCS_EVENT>()) == 0 }
    }
}
impl ::core::cmp::Eq for HCS_EVENT {}
impl ::core::default::Default for HCS_EVENT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_System_HostComputeSystem\"`*"]
pub type HCS_EVENT_CALLBACK = ::core::option::Option<unsafe extern "system" fn(event: *const HCS_EVENT, context: *const ::core::ffi::c_void)>;
#[doc = "*Required features: `\"Win32_System_HostComputeSystem\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct HCS_EVENT_OPTIONS(pub u32);
#[doc = "*Required features: `\"Win32_System_HostComputeSystem\"`*"]
pub const HcsEventOptionNone: HCS_EVENT_OPTIONS = HCS_EVENT_OPTIONS(0u32);
#[doc = "*Required features: `\"Win32_System_HostComputeSystem\"`*"]
pub const HcsEventOptionEnableOperationCallbacks: HCS_EVENT_OPTIONS = HCS_EVENT_OPTIONS(1u32);
impl ::core::marker::Copy for HCS_EVENT_OPTIONS {}
impl ::core::clone::Clone for HCS_EVENT_OPTIONS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for HCS_EVENT_OPTIONS {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for HCS_EVENT_OPTIONS {
    type Abi = Self;
}
impl ::core::fmt::Debug for HCS_EVENT_OPTIONS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("HCS_EVENT_OPTIONS").field(&self.0).finish()
    }
}
impl ::core::ops::BitOr for HCS_EVENT_OPTIONS {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for HCS_EVENT_OPTIONS {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for HCS_EVENT_OPTIONS {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for HCS_EVENT_OPTIONS {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for HCS_EVENT_OPTIONS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[doc = "*Required features: `\"Win32_System_HostComputeSystem\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct HCS_EVENT_TYPE(pub i32);
#[doc = "*Required features: `\"Win32_System_HostComputeSystem\"`*"]
pub const HcsEventInvalid: HCS_EVENT_TYPE = HCS_EVENT_TYPE(0i32);
#[doc = "*Required features: `\"Win32_System_HostComputeSystem\"`*"]
pub const HcsEventSystemExited: HCS_EVENT_TYPE = HCS_EVENT_TYPE(1i32);
#[doc = "*Required features: `\"Win32_System_HostComputeSystem\"`*"]
pub const HcsEventSystemCrashInitiated: HCS_EVENT_TYPE = HCS_EVENT_TYPE(2i32);
#[doc = "*Required features: `\"Win32_System_HostComputeSystem\"`*"]
pub const HcsEventSystemCrashReport: HCS_EVENT_TYPE = HCS_EVENT_TYPE(3i32);
#[doc = "*Required features: `\"Win32_System_HostComputeSystem\"`*"]
pub const HcsEventSystemRdpEnhancedModeStateChanged: HCS_EVENT_TYPE = HCS_EVENT_TYPE(4i32);
#[doc = "*Required features: `\"Win32_System_HostComputeSystem\"`*"]
pub const HcsEventSystemSiloJobCreated: HCS_EVENT_TYPE = HCS_EVENT_TYPE(5i32);
#[doc = "*Required features: `\"Win32_System_HostComputeSystem\"`*"]
pub const HcsEventSystemGuestConnectionClosed: HCS_EVENT_TYPE = HCS_EVENT_TYPE(6i32);
#[doc = "*Required features: `\"Win32_System_HostComputeSystem\"`*"]
pub const HcsEventProcessExited: HCS_EVENT_TYPE = HCS_EVENT_TYPE(65536i32);
#[doc = "*Required features: `\"Win32_System_HostComputeSystem\"`*"]
pub const HcsEventOperationCallback: HCS_EVENT_TYPE = HCS_EVENT_TYPE(16777216i32);
#[doc = "*Required features: `\"Win32_System_HostComputeSystem\"`*"]
pub const HcsEventServiceDisconnect: HCS_EVENT_TYPE = HCS_EVENT_TYPE(33554432i32);
impl ::core::marker::Copy for HCS_EVENT_TYPE {}
impl ::core::clone::Clone for HCS_EVENT_TYPE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for HCS_EVENT_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for HCS_EVENT_TYPE {
    type Abi = Self;
}
impl ::core::fmt::Debug for HCS_EVENT_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("HCS_EVENT_TYPE").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_System_HostComputeSystem\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct HCS_NOTIFICATIONS(pub i32);
#[doc = "*Required features: `\"Win32_System_HostComputeSystem\"`*"]
pub const HcsNotificationInvalid: HCS_NOTIFICATIONS = HCS_NOTIFICATIONS(0i32);
#[doc = "*Required features: `\"Win32_System_HostComputeSystem\"`*"]
pub const HcsNotificationSystemExited: HCS_NOTIFICATIONS = HCS_NOTIFICATIONS(1i32);
#[doc = "*Required features: `\"Win32_System_HostComputeSystem\"`*"]
pub const HcsNotificationSystemCreateCompleted: HCS_NOTIFICATIONS = HCS_NOTIFICATIONS(2i32);
#[doc = "*Required features: `\"Win32_System_HostComputeSystem\"`*"]
pub const HcsNotificationSystemStartCompleted: HCS_NOTIFICATIONS = HCS_NOTIFICATIONS(3i32);
#[doc = "*Required features: `\"Win32_System_HostComputeSystem\"`*"]
pub const HcsNotificationSystemPauseCompleted: HCS_NOTIFICATIONS = HCS_NOTIFICATIONS(4i32);
#[doc = "*Required features: `\"Win32_System_HostComputeSystem\"`*"]
pub const HcsNotificationSystemResumeCompleted: HCS_NOTIFICATIONS = HCS_NOTIFICATIONS(5i32);
#[doc = "*Required features: `\"Win32_System_HostComputeSystem\"`*"]
pub const HcsNotificationSystemCrashReport: HCS_NOTIFICATIONS = HCS_NOTIFICATIONS(6i32);
#[doc = "*Required features: `\"Win32_System_HostComputeSystem\"`*"]
pub const HcsNotificationSystemSiloJobCreated: HCS_NOTIFICATIONS = HCS_NOTIFICATIONS(7i32);
#[doc = "*Required features: `\"Win32_System_HostComputeSystem\"`*"]
pub const HcsNotificationSystemSaveCompleted: HCS_NOTIFICATIONS = HCS_NOTIFICATIONS(8i32);
#[doc = "*Required features: `\"Win32_System_HostComputeSystem\"`*"]
pub const HcsNotificationSystemRdpEnhancedModeStateChanged: HCS_NOTIFICATIONS = HCS_NOTIFICATIONS(9i32);
#[doc = "*Required features: `\"Win32_System_HostComputeSystem\"`*"]
pub const HcsNotificationSystemShutdownFailed: HCS_NOTIFICATIONS = HCS_NOTIFICATIONS(10i32);
#[doc = "*Required features: `\"Win32_System_HostComputeSystem\"`*"]
pub const HcsNotificationSystemShutdownCompleted: HCS_NOTIFICATIONS = HCS_NOTIFICATIONS(10i32);
#[doc = "*Required features: `\"Win32_System_HostComputeSystem\"`*"]
pub const HcsNotificationSystemGetPropertiesCompleted: HCS_NOTIFICATIONS = HCS_NOTIFICATIONS(11i32);
#[doc = "*Required features: `\"Win32_System_HostComputeSystem\"`*"]
pub const HcsNotificationSystemModifyCompleted: HCS_NOTIFICATIONS = HCS_NOTIFICATIONS(12i32);
#[doc = "*Required features: `\"Win32_System_HostComputeSystem\"`*"]
pub const HcsNotificationSystemCrashInitiated: HCS_NOTIFICATIONS = HCS_NOTIFICATIONS(13i32);
#[doc = "*Required features: `\"Win32_System_HostComputeSystem\"`*"]
pub const HcsNotificationSystemGuestConnectionClosed: HCS_NOTIFICATIONS = HCS_NOTIFICATIONS(14i32);
#[doc = "*Required features: `\"Win32_System_HostComputeSystem\"`*"]
pub const HcsNotificationSystemOperationCompletion: HCS_NOTIFICATIONS = HCS_NOTIFICATIONS(15i32);
#[doc = "*Required features: `\"Win32_System_HostComputeSystem\"`*"]
pub const HcsNotificationSystemPassThru: HCS_NOTIFICATIONS = HCS_NOTIFICATIONS(16i32);
#[doc = "*Required features: `\"Win32_System_HostComputeSystem\"`*"]
pub const HcsNotificationProcessExited: HCS_NOTIFICATIONS = HCS_NOTIFICATIONS(65536i32);
#[doc = "*Required features: `\"Win32_System_HostComputeSystem\"`*"]
pub const HcsNotificationServiceDisconnect: HCS_NOTIFICATIONS = HCS_NOTIFICATIONS(16777216i32);
#[doc = "*Required features: `\"Win32_System_HostComputeSystem\"`*"]
pub const HcsNotificationFlagsReserved: HCS_NOTIFICATIONS = HCS_NOTIFICATIONS(-268435456i32);
impl ::core::marker::Copy for HCS_NOTIFICATIONS {}
impl ::core::clone::Clone for HCS_NOTIFICATIONS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for HCS_NOTIFICATIONS {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for HCS_NOTIFICATIONS {
    type Abi = Self;
}
impl ::core::fmt::Debug for HCS_NOTIFICATIONS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("HCS_NOTIFICATIONS").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_System_HostComputeSystem\"`*"]
pub type HCS_NOTIFICATION_CALLBACK = ::core::option::Option<unsafe extern "system" fn(notificationtype: u32, context: *const ::core::ffi::c_void, notificationstatus: ::windows::core::HRESULT, notificationdata: ::windows::core::PCWSTR)>;
#[doc = "*Required features: `\"Win32_System_HostComputeSystem\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct HCS_NOTIFICATION_FLAGS(pub i32);
#[doc = "*Required features: `\"Win32_System_HostComputeSystem\"`*"]
pub const HcsNotificationFlagSuccess: HCS_NOTIFICATION_FLAGS = HCS_NOTIFICATION_FLAGS(0i32);
#[doc = "*Required features: `\"Win32_System_HostComputeSystem\"`*"]
pub const HcsNotificationFlagFailure: HCS_NOTIFICATION_FLAGS = HCS_NOTIFICATION_FLAGS(-2147483648i32);
impl ::core::marker::Copy for HCS_NOTIFICATION_FLAGS {}
impl ::core::clone::Clone for HCS_NOTIFICATION_FLAGS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for HCS_NOTIFICATION_FLAGS {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for HCS_NOTIFICATION_FLAGS {
    type Abi = Self;
}
impl ::core::fmt::Debug for HCS_NOTIFICATION_FLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("HCS_NOTIFICATION_FLAGS").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct HCS_OPERATION(pub isize);
impl HCS_OPERATION {
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
impl ::core::default::Default for HCS_OPERATION {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::clone::Clone for HCS_OPERATION {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::marker::Copy for HCS_OPERATION {}
impl ::core::fmt::Debug for HCS_OPERATION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("HCS_OPERATION").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Abi for HCS_OPERATION {
    type Abi = Self;
}
#[doc = "*Required features: `\"Win32_System_HostComputeSystem\"`*"]
pub type HCS_OPERATION_COMPLETION = ::core::option::Option<unsafe extern "system" fn(operation: HCS_OPERATION, context: *const ::core::ffi::c_void)>;
#[doc = "*Required features: `\"Win32_System_HostComputeSystem\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct HCS_OPERATION_TYPE(pub i32);
#[doc = "*Required features: `\"Win32_System_HostComputeSystem\"`*"]
pub const HcsOperationTypeNone: HCS_OPERATION_TYPE = HCS_OPERATION_TYPE(-1i32);
#[doc = "*Required features: `\"Win32_System_HostComputeSystem\"`*"]
pub const HcsOperationTypeEnumerate: HCS_OPERATION_TYPE = HCS_OPERATION_TYPE(0i32);
#[doc = "*Required features: `\"Win32_System_HostComputeSystem\"`*"]
pub const HcsOperationTypeCreate: HCS_OPERATION_TYPE = HCS_OPERATION_TYPE(1i32);
#[doc = "*Required features: `\"Win32_System_HostComputeSystem\"`*"]
pub const HcsOperationTypeStart: HCS_OPERATION_TYPE = HCS_OPERATION_TYPE(2i32);
#[doc = "*Required features: `\"Win32_System_HostComputeSystem\"`*"]
pub const HcsOperationTypeShutdown: HCS_OPERATION_TYPE = HCS_OPERATION_TYPE(3i32);
#[doc = "*Required features: `\"Win32_System_HostComputeSystem\"`*"]
pub const HcsOperationTypePause: HCS_OPERATION_TYPE = HCS_OPERATION_TYPE(4i32);
#[doc = "*Required features: `\"Win32_System_HostComputeSystem\"`*"]
pub const HcsOperationTypeResume: HCS_OPERATION_TYPE = HCS_OPERATION_TYPE(5i32);
#[doc = "*Required features: `\"Win32_System_HostComputeSystem\"`*"]
pub const HcsOperationTypeSave: HCS_OPERATION_TYPE = HCS_OPERATION_TYPE(6i32);
#[doc = "*Required features: `\"Win32_System_HostComputeSystem\"`*"]
pub const HcsOperationTypeTerminate: HCS_OPERATION_TYPE = HCS_OPERATION_TYPE(7i32);
#[doc = "*Required features: `\"Win32_System_HostComputeSystem\"`*"]
pub const HcsOperationTypeModify: HCS_OPERATION_TYPE = HCS_OPERATION_TYPE(8i32);
#[doc = "*Required features: `\"Win32_System_HostComputeSystem\"`*"]
pub const HcsOperationTypeGetProperties: HCS_OPERATION_TYPE = HCS_OPERATION_TYPE(9i32);
#[doc = "*Required features: `\"Win32_System_HostComputeSystem\"`*"]
pub const HcsOperationTypeCreateProcess: HCS_OPERATION_TYPE = HCS_OPERATION_TYPE(10i32);
#[doc = "*Required features: `\"Win32_System_HostComputeSystem\"`*"]
pub const HcsOperationTypeSignalProcess: HCS_OPERATION_TYPE = HCS_OPERATION_TYPE(11i32);
#[doc = "*Required features: `\"Win32_System_HostComputeSystem\"`*"]
pub const HcsOperationTypeGetProcessInfo: HCS_OPERATION_TYPE = HCS_OPERATION_TYPE(12i32);
#[doc = "*Required features: `\"Win32_System_HostComputeSystem\"`*"]
pub const HcsOperationTypeGetProcessProperties: HCS_OPERATION_TYPE = HCS_OPERATION_TYPE(13i32);
#[doc = "*Required features: `\"Win32_System_HostComputeSystem\"`*"]
pub const HcsOperationTypeModifyProcess: HCS_OPERATION_TYPE = HCS_OPERATION_TYPE(14i32);
#[doc = "*Required features: `\"Win32_System_HostComputeSystem\"`*"]
pub const HcsOperationTypeCrash: HCS_OPERATION_TYPE = HCS_OPERATION_TYPE(15i32);
impl ::core::marker::Copy for HCS_OPERATION_TYPE {}
impl ::core::clone::Clone for HCS_OPERATION_TYPE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for HCS_OPERATION_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for HCS_OPERATION_TYPE {
    type Abi = Self;
}
impl ::core::fmt::Debug for HCS_OPERATION_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("HCS_OPERATION_TYPE").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct HCS_PROCESS(pub isize);
impl HCS_PROCESS {
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
impl ::core::default::Default for HCS_PROCESS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::clone::Clone for HCS_PROCESS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::marker::Copy for HCS_PROCESS {}
impl ::core::fmt::Debug for HCS_PROCESS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("HCS_PROCESS").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Abi for HCS_PROCESS {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_HostComputeSystem\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct HCS_PROCESS_INFORMATION {
    pub ProcessId: u32,
    pub Reserved: u32,
    pub StdInput: super::super::Foundation::HANDLE,
    pub StdOutput: super::super::Foundation::HANDLE,
    pub StdError: super::super::Foundation::HANDLE,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for HCS_PROCESS_INFORMATION {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for HCS_PROCESS_INFORMATION {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for HCS_PROCESS_INFORMATION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("HCS_PROCESS_INFORMATION").field("ProcessId", &self.ProcessId).field("Reserved", &self.Reserved).field("StdInput", &self.StdInput).field("StdOutput", &self.StdOutput).field("StdError", &self.StdError).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for HCS_PROCESS_INFORMATION {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for HCS_PROCESS_INFORMATION {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<HCS_PROCESS_INFORMATION>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for HCS_PROCESS_INFORMATION {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for HCS_PROCESS_INFORMATION {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct HCS_SYSTEM(pub isize);
impl HCS_SYSTEM {
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
impl ::core::default::Default for HCS_SYSTEM {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::clone::Clone for HCS_SYSTEM {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::marker::Copy for HCS_SYSTEM {}
impl ::core::fmt::Debug for HCS_SYSTEM {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("HCS_SYSTEM").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Abi for HCS_SYSTEM {
    type Abi = Self;
}
#[doc = "*Required features: `\"Win32_System_HostComputeSystem\"`*"]
#[inline]
pub unsafe fn HcsAttachLayerStorageFilter<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>, Param1: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>>(layerpath: Param0, layerdata: Param1) -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn HcsAttachLayerStorageFilter(layerpath: ::windows::core::PCWSTR, layerdata: ::windows::core::PCWSTR) -> ::windows::core::HRESULT;
        }
        HcsAttachLayerStorageFilter(layerpath.into_param().abi(), layerdata.into_param().abi()).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_HostComputeSystem\"`*"]
#[inline]
pub unsafe fn HcsCancelOperation<'a, Param0: ::windows::core::IntoParam<'a, HCS_OPERATION>>(operation: Param0) -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn HcsCancelOperation(operation: HCS_OPERATION) -> ::windows::core::HRESULT;
        }
        HcsCancelOperation(operation.into_param().abi()).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_HostComputeSystem\"`*"]
#[inline]
pub unsafe fn HcsCloseComputeSystem<'a, Param0: ::windows::core::IntoParam<'a, HCS_SYSTEM>>(computesystem: Param0) {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn HcsCloseComputeSystem(computesystem: HCS_SYSTEM);
        }
        HcsCloseComputeSystem(computesystem.into_param().abi())
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_HostComputeSystem\"`*"]
#[inline]
pub unsafe fn HcsCloseOperation<'a, Param0: ::windows::core::IntoParam<'a, HCS_OPERATION>>(operation: Param0) {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn HcsCloseOperation(operation: HCS_OPERATION);
        }
        HcsCloseOperation(operation.into_param().abi())
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_HostComputeSystem\"`*"]
#[inline]
pub unsafe fn HcsCloseProcess<'a, Param0: ::windows::core::IntoParam<'a, HCS_PROCESS>>(process: Param0) {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn HcsCloseProcess(process: HCS_PROCESS);
        }
        HcsCloseProcess(process.into_param().abi())
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_HostComputeSystem\"`*"]
#[inline]
pub unsafe fn HcsCrashComputeSystem<'a, Param0: ::windows::core::IntoParam<'a, HCS_SYSTEM>, Param1: ::windows::core::IntoParam<'a, HCS_OPERATION>, Param2: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>>(computesystem: Param0, operation: Param1, options: Param2) -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn HcsCrashComputeSystem(computesystem: HCS_SYSTEM, operation: HCS_OPERATION, options: ::windows::core::PCWSTR) -> ::windows::core::HRESULT;
        }
        HcsCrashComputeSystem(computesystem.into_param().abi(), operation.into_param().abi(), options.into_param().abi()).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_HostComputeSystem\"`, `\"Win32_Foundation\"`, `\"Win32_Security\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
#[inline]
pub unsafe fn HcsCreateComputeSystem<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>, Param1: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>, Param2: ::windows::core::IntoParam<'a, HCS_OPERATION>>(id: Param0, configuration: Param1, operation: Param2, securitydescriptor: *const super::super::Security::SECURITY_DESCRIPTOR) -> ::windows::core::Result<HCS_SYSTEM> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn HcsCreateComputeSystem(id: ::windows::core::PCWSTR, configuration: ::windows::core::PCWSTR, operation: HCS_OPERATION, securitydescriptor: *const super::super::Security::SECURITY_DESCRIPTOR, computesystem: *mut HCS_SYSTEM) -> ::windows::core::HRESULT;
        }
        let mut result__: HCS_SYSTEM = ::core::mem::zeroed();
        HcsCreateComputeSystem(id.into_param().abi(), configuration.into_param().abi(), operation.into_param().abi(), ::core::mem::transmute(securitydescriptor), ::core::mem::transmute(&mut result__)).from_abi::<HCS_SYSTEM>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_HostComputeSystem\"`*"]
#[inline]
pub unsafe fn HcsCreateComputeSystemInNamespace<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>, Param1: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>, Param2: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>, Param3: ::windows::core::IntoParam<'a, HCS_OPERATION>>(idnamespace: Param0, id: Param1, configuration: Param2, operation: Param3, options: *const HCS_CREATE_OPTIONS) -> ::windows::core::Result<HCS_SYSTEM> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn HcsCreateComputeSystemInNamespace(idnamespace: ::windows::core::PCWSTR, id: ::windows::core::PCWSTR, configuration: ::windows::core::PCWSTR, operation: HCS_OPERATION, options: *const HCS_CREATE_OPTIONS, computesystem: *mut HCS_SYSTEM) -> ::windows::core::HRESULT;
        }
        let mut result__: HCS_SYSTEM = ::core::mem::zeroed();
        HcsCreateComputeSystemInNamespace(idnamespace.into_param().abi(), id.into_param().abi(), configuration.into_param().abi(), operation.into_param().abi(), ::core::mem::transmute(options), ::core::mem::transmute(&mut result__)).from_abi::<HCS_SYSTEM>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_HostComputeSystem\"`*"]
#[inline]
pub unsafe fn HcsCreateEmptyGuestStateFile<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>>(gueststatefilepath: Param0) -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn HcsCreateEmptyGuestStateFile(gueststatefilepath: ::windows::core::PCWSTR) -> ::windows::core::HRESULT;
        }
        HcsCreateEmptyGuestStateFile(gueststatefilepath.into_param().abi()).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_HostComputeSystem\"`*"]
#[inline]
pub unsafe fn HcsCreateEmptyRuntimeStateFile<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>>(runtimestatefilepath: Param0) -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn HcsCreateEmptyRuntimeStateFile(runtimestatefilepath: ::windows::core::PCWSTR) -> ::windows::core::HRESULT;
        }
        HcsCreateEmptyRuntimeStateFile(runtimestatefilepath.into_param().abi()).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_HostComputeSystem\"`*"]
#[inline]
pub unsafe fn HcsCreateOperation(context: *const ::core::ffi::c_void, callback: HCS_OPERATION_COMPLETION) -> HCS_OPERATION {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn HcsCreateOperation(context: *const ::core::ffi::c_void, callback: ::windows::core::RawPtr) -> HCS_OPERATION;
        }
        ::core::mem::transmute(HcsCreateOperation(::core::mem::transmute(context), ::core::mem::transmute(callback)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_HostComputeSystem\"`, `\"Win32_Foundation\"`, `\"Win32_Security\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
#[inline]
pub unsafe fn HcsCreateProcess<'a, Param0: ::windows::core::IntoParam<'a, HCS_SYSTEM>, Param1: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>, Param2: ::windows::core::IntoParam<'a, HCS_OPERATION>>(computesystem: Param0, processparameters: Param1, operation: Param2, securitydescriptor: *const super::super::Security::SECURITY_DESCRIPTOR) -> ::windows::core::Result<HCS_PROCESS> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn HcsCreateProcess(computesystem: HCS_SYSTEM, processparameters: ::windows::core::PCWSTR, operation: HCS_OPERATION, securitydescriptor: *const super::super::Security::SECURITY_DESCRIPTOR, process: *mut HCS_PROCESS) -> ::windows::core::HRESULT;
        }
        let mut result__: HCS_PROCESS = ::core::mem::zeroed();
        HcsCreateProcess(computesystem.into_param().abi(), processparameters.into_param().abi(), operation.into_param().abi(), ::core::mem::transmute(securitydescriptor), ::core::mem::transmute(&mut result__)).from_abi::<HCS_PROCESS>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_HostComputeSystem\"`*"]
#[inline]
pub unsafe fn HcsDestroyLayer<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>>(layerpath: Param0) -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn HcsDestroyLayer(layerpath: ::windows::core::PCWSTR) -> ::windows::core::HRESULT;
        }
        HcsDestroyLayer(layerpath.into_param().abi()).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_HostComputeSystem\"`*"]
#[inline]
pub unsafe fn HcsDetachLayerStorageFilter<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>>(layerpath: Param0) -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn HcsDetachLayerStorageFilter(layerpath: ::windows::core::PCWSTR) -> ::windows::core::HRESULT;
        }
        HcsDetachLayerStorageFilter(layerpath.into_param().abi()).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_HostComputeSystem\"`*"]
#[inline]
pub unsafe fn HcsEnumerateComputeSystems<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>, Param1: ::windows::core::IntoParam<'a, HCS_OPERATION>>(query: Param0, operation: Param1) -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn HcsEnumerateComputeSystems(query: ::windows::core::PCWSTR, operation: HCS_OPERATION) -> ::windows::core::HRESULT;
        }
        HcsEnumerateComputeSystems(query.into_param().abi(), operation.into_param().abi()).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_HostComputeSystem\"`*"]
#[inline]
pub unsafe fn HcsEnumerateComputeSystemsInNamespace<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>, Param1: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>, Param2: ::windows::core::IntoParam<'a, HCS_OPERATION>>(idnamespace: Param0, query: Param1, operation: Param2) -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn HcsEnumerateComputeSystemsInNamespace(idnamespace: ::windows::core::PCWSTR, query: ::windows::core::PCWSTR, operation: HCS_OPERATION) -> ::windows::core::HRESULT;
        }
        HcsEnumerateComputeSystemsInNamespace(idnamespace.into_param().abi(), query.into_param().abi(), operation.into_param().abi()).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_HostComputeSystem\"`*"]
#[inline]
pub unsafe fn HcsExportLayer<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>, Param1: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>, Param2: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>, Param3: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>>(layerpath: Param0, exportfolderpath: Param1, layerdata: Param2, options: Param3) -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn HcsExportLayer(layerpath: ::windows::core::PCWSTR, exportfolderpath: ::windows::core::PCWSTR, layerdata: ::windows::core::PCWSTR, options: ::windows::core::PCWSTR) -> ::windows::core::HRESULT;
        }
        HcsExportLayer(layerpath.into_param().abi(), exportfolderpath.into_param().abi(), layerdata.into_param().abi(), options.into_param().abi()).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_HostComputeSystem\"`*"]
#[inline]
pub unsafe fn HcsExportLegacyWritableLayer<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>, Param1: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>, Param2: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>, Param3: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>>(writablelayermountpath: Param0, writablelayerfolderpath: Param1, exportfolderpath: Param2, layerdata: Param3) -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn HcsExportLegacyWritableLayer(writablelayermountpath: ::windows::core::PCWSTR, writablelayerfolderpath: ::windows::core::PCWSTR, exportfolderpath: ::windows::core::PCWSTR, layerdata: ::windows::core::PCWSTR) -> ::windows::core::HRESULT;
        }
        HcsExportLegacyWritableLayer(writablelayermountpath.into_param().abi(), writablelayerfolderpath.into_param().abi(), exportfolderpath.into_param().abi(), layerdata.into_param().abi()).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_HostComputeSystem\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn HcsFormatWritableLayerVhd<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::HANDLE>>(vhdhandle: Param0) -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn HcsFormatWritableLayerVhd(vhdhandle: super::super::Foundation::HANDLE) -> ::windows::core::HRESULT;
        }
        HcsFormatWritableLayerVhd(vhdhandle.into_param().abi()).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_HostComputeSystem\"`*"]
#[inline]
pub unsafe fn HcsGetComputeSystemFromOperation<'a, Param0: ::windows::core::IntoParam<'a, HCS_OPERATION>>(operation: Param0) -> HCS_SYSTEM {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn HcsGetComputeSystemFromOperation(operation: HCS_OPERATION) -> HCS_SYSTEM;
        }
        ::core::mem::transmute(HcsGetComputeSystemFromOperation(operation.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_HostComputeSystem\"`*"]
#[inline]
pub unsafe fn HcsGetComputeSystemProperties<'a, Param0: ::windows::core::IntoParam<'a, HCS_SYSTEM>, Param1: ::windows::core::IntoParam<'a, HCS_OPERATION>, Param2: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>>(computesystem: Param0, operation: Param1, propertyquery: Param2) -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn HcsGetComputeSystemProperties(computesystem: HCS_SYSTEM, operation: HCS_OPERATION, propertyquery: ::windows::core::PCWSTR) -> ::windows::core::HRESULT;
        }
        HcsGetComputeSystemProperties(computesystem.into_param().abi(), operation.into_param().abi(), propertyquery.into_param().abi()).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_HostComputeSystem\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn HcsGetLayerVhdMountPath<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::HANDLE>>(vhdhandle: Param0) -> ::windows::core::Result<::windows::core::PWSTR> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn HcsGetLayerVhdMountPath(vhdhandle: super::super::Foundation::HANDLE, mountpath: *mut ::windows::core::PWSTR) -> ::windows::core::HRESULT;
        }
        let mut result__: ::windows::core::PWSTR = ::core::mem::zeroed();
        HcsGetLayerVhdMountPath(vhdhandle.into_param().abi(), ::core::mem::transmute(&mut result__)).from_abi::<::windows::core::PWSTR>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_HostComputeSystem\"`*"]
#[inline]
pub unsafe fn HcsGetOperationContext<'a, Param0: ::windows::core::IntoParam<'a, HCS_OPERATION>>(operation: Param0) -> *mut ::core::ffi::c_void {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn HcsGetOperationContext(operation: HCS_OPERATION) -> *mut ::core::ffi::c_void;
        }
        ::core::mem::transmute(HcsGetOperationContext(operation.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_HostComputeSystem\"`*"]
#[inline]
pub unsafe fn HcsGetOperationId<'a, Param0: ::windows::core::IntoParam<'a, HCS_OPERATION>>(operation: Param0) -> u64 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn HcsGetOperationId(operation: HCS_OPERATION) -> u64;
        }
        ::core::mem::transmute(HcsGetOperationId(operation.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_HostComputeSystem\"`*"]
#[inline]
pub unsafe fn HcsGetOperationResult<'a, Param0: ::windows::core::IntoParam<'a, HCS_OPERATION>>(operation: Param0) -> ::windows::core::Result<::windows::core::PWSTR> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn HcsGetOperationResult(operation: HCS_OPERATION, resultdocument: *mut ::windows::core::PWSTR) -> ::windows::core::HRESULT;
        }
        let mut result__: ::windows::core::PWSTR = ::core::mem::zeroed();
        HcsGetOperationResult(operation.into_param().abi(), ::core::mem::transmute(&mut result__)).from_abi::<::windows::core::PWSTR>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_HostComputeSystem\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn HcsGetOperationResultAndProcessInfo<'a, Param0: ::windows::core::IntoParam<'a, HCS_OPERATION>>(operation: Param0, processinformation: *mut HCS_PROCESS_INFORMATION, resultdocument: *mut ::windows::core::PWSTR) -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn HcsGetOperationResultAndProcessInfo(operation: HCS_OPERATION, processinformation: *mut HCS_PROCESS_INFORMATION, resultdocument: *mut ::windows::core::PWSTR) -> ::windows::core::HRESULT;
        }
        HcsGetOperationResultAndProcessInfo(operation.into_param().abi(), ::core::mem::transmute(processinformation), ::core::mem::transmute(resultdocument)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_HostComputeSystem\"`*"]
#[inline]
pub unsafe fn HcsGetOperationType<'a, Param0: ::windows::core::IntoParam<'a, HCS_OPERATION>>(operation: Param0) -> HCS_OPERATION_TYPE {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn HcsGetOperationType(operation: HCS_OPERATION) -> HCS_OPERATION_TYPE;
        }
        ::core::mem::transmute(HcsGetOperationType(operation.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_HostComputeSystem\"`*"]
#[inline]
pub unsafe fn HcsGetProcessFromOperation<'a, Param0: ::windows::core::IntoParam<'a, HCS_OPERATION>>(operation: Param0) -> HCS_PROCESS {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn HcsGetProcessFromOperation(operation: HCS_OPERATION) -> HCS_PROCESS;
        }
        ::core::mem::transmute(HcsGetProcessFromOperation(operation.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_HostComputeSystem\"`*"]
#[inline]
pub unsafe fn HcsGetProcessInfo<'a, Param0: ::windows::core::IntoParam<'a, HCS_PROCESS>, Param1: ::windows::core::IntoParam<'a, HCS_OPERATION>>(process: Param0, operation: Param1) -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn HcsGetProcessInfo(process: HCS_PROCESS, operation: HCS_OPERATION) -> ::windows::core::HRESULT;
        }
        HcsGetProcessInfo(process.into_param().abi(), operation.into_param().abi()).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_HostComputeSystem\"`*"]
#[inline]
pub unsafe fn HcsGetProcessProperties<'a, Param0: ::windows::core::IntoParam<'a, HCS_PROCESS>, Param1: ::windows::core::IntoParam<'a, HCS_OPERATION>, Param2: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>>(process: Param0, operation: Param1, propertyquery: Param2) -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn HcsGetProcessProperties(process: HCS_PROCESS, operation: HCS_OPERATION, propertyquery: ::windows::core::PCWSTR) -> ::windows::core::HRESULT;
        }
        HcsGetProcessProperties(process.into_param().abi(), operation.into_param().abi(), propertyquery.into_param().abi()).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_HostComputeSystem\"`*"]
#[inline]
pub unsafe fn HcsGetProcessorCompatibilityFromSavedState<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>>(runtimefilename: Param0) -> ::windows::core::Result<::windows::core::PWSTR> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn HcsGetProcessorCompatibilityFromSavedState(runtimefilename: ::windows::core::PCWSTR, processorfeaturesstring: *mut ::windows::core::PWSTR) -> ::windows::core::HRESULT;
        }
        let mut result__: ::windows::core::PWSTR = ::core::mem::zeroed();
        HcsGetProcessorCompatibilityFromSavedState(runtimefilename.into_param().abi(), ::core::mem::transmute(&mut result__)).from_abi::<::windows::core::PWSTR>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_HostComputeSystem\"`*"]
#[inline]
pub unsafe fn HcsGetServiceProperties<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>>(propertyquery: Param0) -> ::windows::core::Result<::windows::core::PWSTR> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn HcsGetServiceProperties(propertyquery: ::windows::core::PCWSTR, result: *mut ::windows::core::PWSTR) -> ::windows::core::HRESULT;
        }
        let mut result__: ::windows::core::PWSTR = ::core::mem::zeroed();
        HcsGetServiceProperties(propertyquery.into_param().abi(), ::core::mem::transmute(&mut result__)).from_abi::<::windows::core::PWSTR>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_HostComputeSystem\"`*"]
#[inline]
pub unsafe fn HcsGrantVmAccess<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>, Param1: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>>(vmid: Param0, filepath: Param1) -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn HcsGrantVmAccess(vmid: ::windows::core::PCWSTR, filepath: ::windows::core::PCWSTR) -> ::windows::core::HRESULT;
        }
        HcsGrantVmAccess(vmid.into_param().abi(), filepath.into_param().abi()).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_HostComputeSystem\"`*"]
#[inline]
pub unsafe fn HcsGrantVmGroupAccess<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>>(filepath: Param0) -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn HcsGrantVmGroupAccess(filepath: ::windows::core::PCWSTR) -> ::windows::core::HRESULT;
        }
        HcsGrantVmGroupAccess(filepath.into_param().abi()).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_HostComputeSystem\"`*"]
#[inline]
pub unsafe fn HcsImportLayer<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>, Param1: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>, Param2: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>>(layerpath: Param0, sourcefolderpath: Param1, layerdata: Param2) -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn HcsImportLayer(layerpath: ::windows::core::PCWSTR, sourcefolderpath: ::windows::core::PCWSTR, layerdata: ::windows::core::PCWSTR) -> ::windows::core::HRESULT;
        }
        HcsImportLayer(layerpath.into_param().abi(), sourcefolderpath.into_param().abi(), layerdata.into_param().abi()).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_HostComputeSystem\"`*"]
#[inline]
pub unsafe fn HcsInitializeLegacyWritableLayer<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>, Param1: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>, Param2: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>, Param3: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>>(writablelayermountpath: Param0, writablelayerfolderpath: Param1, layerdata: Param2, options: Param3) -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn HcsInitializeLegacyWritableLayer(writablelayermountpath: ::windows::core::PCWSTR, writablelayerfolderpath: ::windows::core::PCWSTR, layerdata: ::windows::core::PCWSTR, options: ::windows::core::PCWSTR) -> ::windows::core::HRESULT;
        }
        HcsInitializeLegacyWritableLayer(writablelayermountpath.into_param().abi(), writablelayerfolderpath.into_param().abi(), layerdata.into_param().abi(), options.into_param().abi()).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_HostComputeSystem\"`*"]
#[inline]
pub unsafe fn HcsInitializeWritableLayer<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>, Param1: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>, Param2: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>>(writablelayerpath: Param0, layerdata: Param1, options: Param2) -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn HcsInitializeWritableLayer(writablelayerpath: ::windows::core::PCWSTR, layerdata: ::windows::core::PCWSTR, options: ::windows::core::PCWSTR) -> ::windows::core::HRESULT;
        }
        HcsInitializeWritableLayer(writablelayerpath.into_param().abi(), layerdata.into_param().abi(), options.into_param().abi()).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_HostComputeSystem\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn HcsModifyComputeSystem<'a, Param0: ::windows::core::IntoParam<'a, HCS_SYSTEM>, Param1: ::windows::core::IntoParam<'a, HCS_OPERATION>, Param2: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>, Param3: ::windows::core::IntoParam<'a, super::super::Foundation::HANDLE>>(computesystem: Param0, operation: Param1, configuration: Param2, identity: Param3) -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn HcsModifyComputeSystem(computesystem: HCS_SYSTEM, operation: HCS_OPERATION, configuration: ::windows::core::PCWSTR, identity: super::super::Foundation::HANDLE) -> ::windows::core::HRESULT;
        }
        HcsModifyComputeSystem(computesystem.into_param().abi(), operation.into_param().abi(), configuration.into_param().abi(), identity.into_param().abi()).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_HostComputeSystem\"`*"]
#[inline]
pub unsafe fn HcsModifyProcess<'a, Param0: ::windows::core::IntoParam<'a, HCS_PROCESS>, Param1: ::windows::core::IntoParam<'a, HCS_OPERATION>, Param2: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>>(process: Param0, operation: Param1, settings: Param2) -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn HcsModifyProcess(process: HCS_PROCESS, operation: HCS_OPERATION, settings: ::windows::core::PCWSTR) -> ::windows::core::HRESULT;
        }
        HcsModifyProcess(process.into_param().abi(), operation.into_param().abi(), settings.into_param().abi()).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_HostComputeSystem\"`*"]
#[inline]
pub unsafe fn HcsModifyServiceSettings<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>>(settings: Param0) -> ::windows::core::Result<::windows::core::PWSTR> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn HcsModifyServiceSettings(settings: ::windows::core::PCWSTR, result: *mut ::windows::core::PWSTR) -> ::windows::core::HRESULT;
        }
        let mut result__: ::windows::core::PWSTR = ::core::mem::zeroed();
        HcsModifyServiceSettings(settings.into_param().abi(), ::core::mem::transmute(&mut result__)).from_abi::<::windows::core::PWSTR>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_HostComputeSystem\"`*"]
#[inline]
pub unsafe fn HcsOpenComputeSystem<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>>(id: Param0, requestedaccess: u32) -> ::windows::core::Result<HCS_SYSTEM> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn HcsOpenComputeSystem(id: ::windows::core::PCWSTR, requestedaccess: u32, computesystem: *mut HCS_SYSTEM) -> ::windows::core::HRESULT;
        }
        let mut result__: HCS_SYSTEM = ::core::mem::zeroed();
        HcsOpenComputeSystem(id.into_param().abi(), ::core::mem::transmute(requestedaccess), ::core::mem::transmute(&mut result__)).from_abi::<HCS_SYSTEM>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_HostComputeSystem\"`*"]
#[inline]
pub unsafe fn HcsOpenComputeSystemInNamespace<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>, Param1: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>>(idnamespace: Param0, id: Param1, requestedaccess: u32) -> ::windows::core::Result<HCS_SYSTEM> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn HcsOpenComputeSystemInNamespace(idnamespace: ::windows::core::PCWSTR, id: ::windows::core::PCWSTR, requestedaccess: u32, computesystem: *mut HCS_SYSTEM) -> ::windows::core::HRESULT;
        }
        let mut result__: HCS_SYSTEM = ::core::mem::zeroed();
        HcsOpenComputeSystemInNamespace(idnamespace.into_param().abi(), id.into_param().abi(), ::core::mem::transmute(requestedaccess), ::core::mem::transmute(&mut result__)).from_abi::<HCS_SYSTEM>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_HostComputeSystem\"`*"]
#[inline]
pub unsafe fn HcsOpenProcess<'a, Param0: ::windows::core::IntoParam<'a, HCS_SYSTEM>>(computesystem: Param0, processid: u32, requestedaccess: u32) -> ::windows::core::Result<HCS_PROCESS> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn HcsOpenProcess(computesystem: HCS_SYSTEM, processid: u32, requestedaccess: u32, process: *mut HCS_PROCESS) -> ::windows::core::HRESULT;
        }
        let mut result__: HCS_PROCESS = ::core::mem::zeroed();
        HcsOpenProcess(computesystem.into_param().abi(), ::core::mem::transmute(processid), ::core::mem::transmute(requestedaccess), ::core::mem::transmute(&mut result__)).from_abi::<HCS_PROCESS>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_HostComputeSystem\"`*"]
#[inline]
pub unsafe fn HcsPauseComputeSystem<'a, Param0: ::windows::core::IntoParam<'a, HCS_SYSTEM>, Param1: ::windows::core::IntoParam<'a, HCS_OPERATION>, Param2: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>>(computesystem: Param0, operation: Param1, options: Param2) -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn HcsPauseComputeSystem(computesystem: HCS_SYSTEM, operation: HCS_OPERATION, options: ::windows::core::PCWSTR) -> ::windows::core::HRESULT;
        }
        HcsPauseComputeSystem(computesystem.into_param().abi(), operation.into_param().abi(), options.into_param().abi()).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_HostComputeSystem\"`*"]
#[inline]
pub unsafe fn HcsResumeComputeSystem<'a, Param0: ::windows::core::IntoParam<'a, HCS_SYSTEM>, Param1: ::windows::core::IntoParam<'a, HCS_OPERATION>, Param2: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>>(computesystem: Param0, operation: Param1, options: Param2) -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn HcsResumeComputeSystem(computesystem: HCS_SYSTEM, operation: HCS_OPERATION, options: ::windows::core::PCWSTR) -> ::windows::core::HRESULT;
        }
        HcsResumeComputeSystem(computesystem.into_param().abi(), operation.into_param().abi(), options.into_param().abi()).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_HostComputeSystem\"`*"]
#[inline]
pub unsafe fn HcsRevokeVmAccess<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>, Param1: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>>(vmid: Param0, filepath: Param1) -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn HcsRevokeVmAccess(vmid: ::windows::core::PCWSTR, filepath: ::windows::core::PCWSTR) -> ::windows::core::HRESULT;
        }
        HcsRevokeVmAccess(vmid.into_param().abi(), filepath.into_param().abi()).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_HostComputeSystem\"`*"]
#[inline]
pub unsafe fn HcsRevokeVmGroupAccess<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>>(filepath: Param0) -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn HcsRevokeVmGroupAccess(filepath: ::windows::core::PCWSTR) -> ::windows::core::HRESULT;
        }
        HcsRevokeVmGroupAccess(filepath.into_param().abi()).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_HostComputeSystem\"`*"]
#[inline]
pub unsafe fn HcsSaveComputeSystem<'a, Param0: ::windows::core::IntoParam<'a, HCS_SYSTEM>, Param1: ::windows::core::IntoParam<'a, HCS_OPERATION>, Param2: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>>(computesystem: Param0, operation: Param1, options: Param2) -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn HcsSaveComputeSystem(computesystem: HCS_SYSTEM, operation: HCS_OPERATION, options: ::windows::core::PCWSTR) -> ::windows::core::HRESULT;
        }
        HcsSaveComputeSystem(computesystem.into_param().abi(), operation.into_param().abi(), options.into_param().abi()).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_HostComputeSystem\"`*"]
#[inline]
pub unsafe fn HcsSetComputeSystemCallback<'a, Param0: ::windows::core::IntoParam<'a, HCS_SYSTEM>>(computesystem: Param0, callbackoptions: HCS_EVENT_OPTIONS, context: *const ::core::ffi::c_void, callback: HCS_EVENT_CALLBACK) -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn HcsSetComputeSystemCallback(computesystem: HCS_SYSTEM, callbackoptions: HCS_EVENT_OPTIONS, context: *const ::core::ffi::c_void, callback: ::windows::core::RawPtr) -> ::windows::core::HRESULT;
        }
        HcsSetComputeSystemCallback(computesystem.into_param().abi(), ::core::mem::transmute(callbackoptions), ::core::mem::transmute(context), ::core::mem::transmute(callback)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_HostComputeSystem\"`*"]
#[inline]
pub unsafe fn HcsSetOperationCallback<'a, Param0: ::windows::core::IntoParam<'a, HCS_OPERATION>>(operation: Param0, context: *const ::core::ffi::c_void, callback: HCS_OPERATION_COMPLETION) -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn HcsSetOperationCallback(operation: HCS_OPERATION, context: *const ::core::ffi::c_void, callback: ::windows::core::RawPtr) -> ::windows::core::HRESULT;
        }
        HcsSetOperationCallback(operation.into_param().abi(), ::core::mem::transmute(context), ::core::mem::transmute(callback)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_HostComputeSystem\"`*"]
#[inline]
pub unsafe fn HcsSetOperationContext<'a, Param0: ::windows::core::IntoParam<'a, HCS_OPERATION>>(operation: Param0, context: *const ::core::ffi::c_void) -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn HcsSetOperationContext(operation: HCS_OPERATION, context: *const ::core::ffi::c_void) -> ::windows::core::HRESULT;
        }
        HcsSetOperationContext(operation.into_param().abi(), ::core::mem::transmute(context)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_HostComputeSystem\"`*"]
#[inline]
pub unsafe fn HcsSetProcessCallback<'a, Param0: ::windows::core::IntoParam<'a, HCS_PROCESS>>(process: Param0, callbackoptions: HCS_EVENT_OPTIONS, context: *const ::core::ffi::c_void, callback: HCS_EVENT_CALLBACK) -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn HcsSetProcessCallback(process: HCS_PROCESS, callbackoptions: HCS_EVENT_OPTIONS, context: *const ::core::ffi::c_void, callback: ::windows::core::RawPtr) -> ::windows::core::HRESULT;
        }
        HcsSetProcessCallback(process.into_param().abi(), ::core::mem::transmute(callbackoptions), ::core::mem::transmute(context), ::core::mem::transmute(callback)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_HostComputeSystem\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn HcsSetupBaseOSLayer<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::HANDLE>, Param2: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>>(layerpath: Param0, vhdhandle: Param1, options: Param2) -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn HcsSetupBaseOSLayer(layerpath: ::windows::core::PCWSTR, vhdhandle: super::super::Foundation::HANDLE, options: ::windows::core::PCWSTR) -> ::windows::core::HRESULT;
        }
        HcsSetupBaseOSLayer(layerpath.into_param().abi(), vhdhandle.into_param().abi(), options.into_param().abi()).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_HostComputeSystem\"`*"]
#[inline]
pub unsafe fn HcsSetupBaseOSVolume<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>, Param1: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>, Param2: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>>(layerpath: Param0, volumepath: Param1, options: Param2) -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn HcsSetupBaseOSVolume(layerpath: ::windows::core::PCWSTR, volumepath: ::windows::core::PCWSTR, options: ::windows::core::PCWSTR) -> ::windows::core::HRESULT;
        }
        HcsSetupBaseOSVolume(layerpath.into_param().abi(), volumepath.into_param().abi(), options.into_param().abi()).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_HostComputeSystem\"`*"]
#[inline]
pub unsafe fn HcsShutDownComputeSystem<'a, Param0: ::windows::core::IntoParam<'a, HCS_SYSTEM>, Param1: ::windows::core::IntoParam<'a, HCS_OPERATION>, Param2: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>>(computesystem: Param0, operation: Param1, options: Param2) -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn HcsShutDownComputeSystem(computesystem: HCS_SYSTEM, operation: HCS_OPERATION, options: ::windows::core::PCWSTR) -> ::windows::core::HRESULT;
        }
        HcsShutDownComputeSystem(computesystem.into_param().abi(), operation.into_param().abi(), options.into_param().abi()).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_HostComputeSystem\"`*"]
#[inline]
pub unsafe fn HcsSignalProcess<'a, Param0: ::windows::core::IntoParam<'a, HCS_PROCESS>, Param1: ::windows::core::IntoParam<'a, HCS_OPERATION>, Param2: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>>(process: Param0, operation: Param1, options: Param2) -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn HcsSignalProcess(process: HCS_PROCESS, operation: HCS_OPERATION, options: ::windows::core::PCWSTR) -> ::windows::core::HRESULT;
        }
        HcsSignalProcess(process.into_param().abi(), operation.into_param().abi(), options.into_param().abi()).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_HostComputeSystem\"`*"]
#[inline]
pub unsafe fn HcsStartComputeSystem<'a, Param0: ::windows::core::IntoParam<'a, HCS_SYSTEM>, Param1: ::windows::core::IntoParam<'a, HCS_OPERATION>, Param2: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>>(computesystem: Param0, operation: Param1, options: Param2) -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn HcsStartComputeSystem(computesystem: HCS_SYSTEM, operation: HCS_OPERATION, options: ::windows::core::PCWSTR) -> ::windows::core::HRESULT;
        }
        HcsStartComputeSystem(computesystem.into_param().abi(), operation.into_param().abi(), options.into_param().abi()).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_HostComputeSystem\"`*"]
#[inline]
pub unsafe fn HcsSubmitWerReport<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>>(settings: Param0) -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn HcsSubmitWerReport(settings: ::windows::core::PCWSTR) -> ::windows::core::HRESULT;
        }
        HcsSubmitWerReport(settings.into_param().abi()).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_HostComputeSystem\"`*"]
#[inline]
pub unsafe fn HcsTerminateComputeSystem<'a, Param0: ::windows::core::IntoParam<'a, HCS_SYSTEM>, Param1: ::windows::core::IntoParam<'a, HCS_OPERATION>, Param2: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>>(computesystem: Param0, operation: Param1, options: Param2) -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn HcsTerminateComputeSystem(computesystem: HCS_SYSTEM, operation: HCS_OPERATION, options: ::windows::core::PCWSTR) -> ::windows::core::HRESULT;
        }
        HcsTerminateComputeSystem(computesystem.into_param().abi(), operation.into_param().abi(), options.into_param().abi()).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_HostComputeSystem\"`*"]
#[inline]
pub unsafe fn HcsTerminateProcess<'a, Param0: ::windows::core::IntoParam<'a, HCS_PROCESS>, Param1: ::windows::core::IntoParam<'a, HCS_OPERATION>, Param2: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>>(process: Param0, operation: Param1, options: Param2) -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn HcsTerminateProcess(process: HCS_PROCESS, operation: HCS_OPERATION, options: ::windows::core::PCWSTR) -> ::windows::core::HRESULT;
        }
        HcsTerminateProcess(process.into_param().abi(), operation.into_param().abi(), options.into_param().abi()).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_HostComputeSystem\"`*"]
#[inline]
pub unsafe fn HcsWaitForComputeSystemExit<'a, Param0: ::windows::core::IntoParam<'a, HCS_SYSTEM>>(computesystem: Param0, timeoutms: u32) -> ::windows::core::Result<::windows::core::PWSTR> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn HcsWaitForComputeSystemExit(computesystem: HCS_SYSTEM, timeoutms: u32, result: *mut ::windows::core::PWSTR) -> ::windows::core::HRESULT;
        }
        let mut result__: ::windows::core::PWSTR = ::core::mem::zeroed();
        HcsWaitForComputeSystemExit(computesystem.into_param().abi(), ::core::mem::transmute(timeoutms), ::core::mem::transmute(&mut result__)).from_abi::<::windows::core::PWSTR>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_HostComputeSystem\"`*"]
#[inline]
pub unsafe fn HcsWaitForOperationResult<'a, Param0: ::windows::core::IntoParam<'a, HCS_OPERATION>>(operation: Param0, timeoutms: u32) -> ::windows::core::Result<::windows::core::PWSTR> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn HcsWaitForOperationResult(operation: HCS_OPERATION, timeoutms: u32, resultdocument: *mut ::windows::core::PWSTR) -> ::windows::core::HRESULT;
        }
        let mut result__: ::windows::core::PWSTR = ::core::mem::zeroed();
        HcsWaitForOperationResult(operation.into_param().abi(), ::core::mem::transmute(timeoutms), ::core::mem::transmute(&mut result__)).from_abi::<::windows::core::PWSTR>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_HostComputeSystem\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn HcsWaitForOperationResultAndProcessInfo<'a, Param0: ::windows::core::IntoParam<'a, HCS_OPERATION>>(operation: Param0, timeoutms: u32, processinformation: *mut HCS_PROCESS_INFORMATION, resultdocument: *mut ::windows::core::PWSTR) -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn HcsWaitForOperationResultAndProcessInfo(operation: HCS_OPERATION, timeoutms: u32, processinformation: *mut HCS_PROCESS_INFORMATION, resultdocument: *mut ::windows::core::PWSTR) -> ::windows::core::HRESULT;
        }
        HcsWaitForOperationResultAndProcessInfo(operation.into_param().abi(), ::core::mem::transmute(timeoutms), ::core::mem::transmute(processinformation), ::core::mem::transmute(resultdocument)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_HostComputeSystem\"`*"]
#[inline]
pub unsafe fn HcsWaitForProcessExit<'a, Param0: ::windows::core::IntoParam<'a, HCS_PROCESS>>(computesystem: Param0, timeoutms: u32) -> ::windows::core::Result<::windows::core::PWSTR> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn HcsWaitForProcessExit(computesystem: HCS_PROCESS, timeoutms: u32, result: *mut ::windows::core::PWSTR) -> ::windows::core::HRESULT;
        }
        let mut result__: ::windows::core::PWSTR = ::core::mem::zeroed();
        HcsWaitForProcessExit(computesystem.into_param().abi(), ::core::mem::transmute(timeoutms), ::core::mem::transmute(&mut result__)).from_abi::<::windows::core::PWSTR>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "implement")]
::core::include!("impl.rs");

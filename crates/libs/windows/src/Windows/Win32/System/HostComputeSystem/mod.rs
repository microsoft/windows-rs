#[doc = "*Required features: `\"Win32_System_HostComputeSystem\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
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
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
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
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
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
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
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
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
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
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct HCS_OPERATION(pub isize);
impl HCS_OPERATION {
    pub fn is_invalid(&self) -> bool {
        self.0 == -1 || self.0 == 0
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
impl ::core::convert::From<::core::option::Option<HCS_OPERATION>> for HCS_OPERATION {
    fn from(optional: ::core::option::Option<HCS_OPERATION>) -> HCS_OPERATION {
        optional.unwrap_or_default()
    }
}
unsafe impl ::windows::core::Abi for HCS_OPERATION {
    type Abi = Self;
}
#[doc = "*Required features: `\"Win32_System_HostComputeSystem\"`*"]
pub type HCS_OPERATION_COMPLETION = ::core::option::Option<unsafe extern "system" fn(operation: HCS_OPERATION, context: *const ::core::ffi::c_void)>;
#[doc = "*Required features: `\"Win32_System_HostComputeSystem\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
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
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct HCS_PROCESS(pub isize);
impl HCS_PROCESS {
    pub fn is_invalid(&self) -> bool {
        self.0 == -1 || self.0 == 0
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
impl ::core::convert::From<::core::option::Option<HCS_PROCESS>> for HCS_PROCESS {
    fn from(optional: ::core::option::Option<HCS_PROCESS>) -> HCS_PROCESS {
        optional.unwrap_or_default()
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
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct HCS_SYSTEM(pub isize);
impl HCS_SYSTEM {
    pub fn is_invalid(&self) -> bool {
        self.0 == -1 || self.0 == 0
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
impl ::core::convert::From<::core::option::Option<HCS_SYSTEM>> for HCS_SYSTEM {
    fn from(optional: ::core::option::Option<HCS_SYSTEM>) -> HCS_SYSTEM {
        optional.unwrap_or_default()
    }
}
unsafe impl ::windows::core::Abi for HCS_SYSTEM {
    type Abi = Self;
}
#[doc = "*Required features: `\"Win32_System_HostComputeSystem\"`*"]
#[inline]
pub unsafe fn HcsAttachLayerStorageFilter<'a, P0, P1>(layerpath: P0, layerdata: P1) -> ::windows::core::Result<()>
where
    P0: ::std::convert::Into<::windows::core::PCWSTR>,
    P1: ::std::convert::Into<::windows::core::PCWSTR>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn HcsAttachLayerStorageFilter(layerpath: ::windows::core::PCWSTR, layerdata: ::windows::core::PCWSTR) -> ::windows::core::HRESULT;
    }
    HcsAttachLayerStorageFilter(layerpath.into(), layerdata.into()).ok()
}
#[doc = "*Required features: `\"Win32_System_HostComputeSystem\"`*"]
#[inline]
pub unsafe fn HcsCancelOperation<'a, P0>(operation: P0) -> ::windows::core::Result<()>
where
    P0: ::std::convert::Into<HCS_OPERATION>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn HcsCancelOperation(operation: HCS_OPERATION) -> ::windows::core::HRESULT;
    }
    HcsCancelOperation(operation.into()).ok()
}
#[doc = "*Required features: `\"Win32_System_HostComputeSystem\"`*"]
#[inline]
pub unsafe fn HcsCloseComputeSystem<'a, P0>(computesystem: P0)
where
    P0: ::std::convert::Into<HCS_SYSTEM>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn HcsCloseComputeSystem(computesystem: HCS_SYSTEM);
    }
    HcsCloseComputeSystem(computesystem.into())
}
#[doc = "*Required features: `\"Win32_System_HostComputeSystem\"`*"]
#[inline]
pub unsafe fn HcsCloseOperation<'a, P0>(operation: P0)
where
    P0: ::std::convert::Into<HCS_OPERATION>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn HcsCloseOperation(operation: HCS_OPERATION);
    }
    HcsCloseOperation(operation.into())
}
#[doc = "*Required features: `\"Win32_System_HostComputeSystem\"`*"]
#[inline]
pub unsafe fn HcsCloseProcess<'a, P0>(process: P0)
where
    P0: ::std::convert::Into<HCS_PROCESS>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn HcsCloseProcess(process: HCS_PROCESS);
    }
    HcsCloseProcess(process.into())
}
#[doc = "*Required features: `\"Win32_System_HostComputeSystem\"`*"]
#[inline]
pub unsafe fn HcsCrashComputeSystem<'a, P0, P1, P2>(computesystem: P0, operation: P1, options: P2) -> ::windows::core::Result<()>
where
    P0: ::std::convert::Into<HCS_SYSTEM>,
    P1: ::std::convert::Into<HCS_OPERATION>,
    P2: ::std::convert::Into<::windows::core::PCWSTR>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn HcsCrashComputeSystem(computesystem: HCS_SYSTEM, operation: HCS_OPERATION, options: ::windows::core::PCWSTR) -> ::windows::core::HRESULT;
    }
    HcsCrashComputeSystem(computesystem.into(), operation.into(), options.into()).ok()
}
#[doc = "*Required features: `\"Win32_System_HostComputeSystem\"`, `\"Win32_Foundation\"`, `\"Win32_Security\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
#[inline]
pub unsafe fn HcsCreateComputeSystem<'a, P0, P1, P2>(id: P0, configuration: P1, operation: P2, securitydescriptor: *const super::super::Security::SECURITY_DESCRIPTOR) -> ::windows::core::Result<HCS_SYSTEM>
where
    P0: ::std::convert::Into<::windows::core::PCWSTR>,
    P1: ::std::convert::Into<::windows::core::PCWSTR>,
    P2: ::std::convert::Into<HCS_OPERATION>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn HcsCreateComputeSystem(id: ::windows::core::PCWSTR, configuration: ::windows::core::PCWSTR, operation: HCS_OPERATION, securitydescriptor: *const super::super::Security::SECURITY_DESCRIPTOR, computesystem: *mut HCS_SYSTEM) -> ::windows::core::HRESULT;
    }
    let mut result__ = ::core::mem::MaybeUninit::zeroed();
    HcsCreateComputeSystem(id.into(), configuration.into(), operation.into(), ::core::mem::transmute(securitydescriptor), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<HCS_SYSTEM>(result__)
}
#[doc = "*Required features: `\"Win32_System_HostComputeSystem\"`*"]
#[inline]
pub unsafe fn HcsCreateComputeSystemInNamespace<'a, P0, P1, P2, P3>(idnamespace: P0, id: P1, configuration: P2, operation: P3, options: *const HCS_CREATE_OPTIONS) -> ::windows::core::Result<HCS_SYSTEM>
where
    P0: ::std::convert::Into<::windows::core::PCWSTR>,
    P1: ::std::convert::Into<::windows::core::PCWSTR>,
    P2: ::std::convert::Into<::windows::core::PCWSTR>,
    P3: ::std::convert::Into<HCS_OPERATION>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn HcsCreateComputeSystemInNamespace(idnamespace: ::windows::core::PCWSTR, id: ::windows::core::PCWSTR, configuration: ::windows::core::PCWSTR, operation: HCS_OPERATION, options: *const HCS_CREATE_OPTIONS, computesystem: *mut HCS_SYSTEM) -> ::windows::core::HRESULT;
    }
    let mut result__ = ::core::mem::MaybeUninit::zeroed();
    HcsCreateComputeSystemInNamespace(idnamespace.into(), id.into(), configuration.into(), operation.into(), ::core::mem::transmute(options), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<HCS_SYSTEM>(result__)
}
#[doc = "*Required features: `\"Win32_System_HostComputeSystem\"`*"]
#[inline]
pub unsafe fn HcsCreateEmptyGuestStateFile<'a, P0>(gueststatefilepath: P0) -> ::windows::core::Result<()>
where
    P0: ::std::convert::Into<::windows::core::PCWSTR>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn HcsCreateEmptyGuestStateFile(gueststatefilepath: ::windows::core::PCWSTR) -> ::windows::core::HRESULT;
    }
    HcsCreateEmptyGuestStateFile(gueststatefilepath.into()).ok()
}
#[doc = "*Required features: `\"Win32_System_HostComputeSystem\"`*"]
#[inline]
pub unsafe fn HcsCreateEmptyRuntimeStateFile<'a, P0>(runtimestatefilepath: P0) -> ::windows::core::Result<()>
where
    P0: ::std::convert::Into<::windows::core::PCWSTR>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn HcsCreateEmptyRuntimeStateFile(runtimestatefilepath: ::windows::core::PCWSTR) -> ::windows::core::HRESULT;
    }
    HcsCreateEmptyRuntimeStateFile(runtimestatefilepath.into()).ok()
}
#[doc = "*Required features: `\"Win32_System_HostComputeSystem\"`*"]
#[inline]
pub unsafe fn HcsCreateOperation(context: *const ::core::ffi::c_void, callback: HCS_OPERATION_COMPLETION) -> HCS_OPERATION {
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn HcsCreateOperation(context: *const ::core::ffi::c_void, callback: *mut ::core::ffi::c_void) -> HCS_OPERATION;
    }
    HcsCreateOperation(::core::mem::transmute(context), ::core::mem::transmute(callback))
}
#[doc = "*Required features: `\"Win32_System_HostComputeSystem\"`, `\"Win32_Foundation\"`, `\"Win32_Security\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
#[inline]
pub unsafe fn HcsCreateProcess<'a, P0, P1, P2>(computesystem: P0, processparameters: P1, operation: P2, securitydescriptor: *const super::super::Security::SECURITY_DESCRIPTOR) -> ::windows::core::Result<HCS_PROCESS>
where
    P0: ::std::convert::Into<HCS_SYSTEM>,
    P1: ::std::convert::Into<::windows::core::PCWSTR>,
    P2: ::std::convert::Into<HCS_OPERATION>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn HcsCreateProcess(computesystem: HCS_SYSTEM, processparameters: ::windows::core::PCWSTR, operation: HCS_OPERATION, securitydescriptor: *const super::super::Security::SECURITY_DESCRIPTOR, process: *mut HCS_PROCESS) -> ::windows::core::HRESULT;
    }
    let mut result__ = ::core::mem::MaybeUninit::zeroed();
    HcsCreateProcess(computesystem.into(), processparameters.into(), operation.into(), ::core::mem::transmute(securitydescriptor), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<HCS_PROCESS>(result__)
}
#[doc = "*Required features: `\"Win32_System_HostComputeSystem\"`*"]
#[inline]
pub unsafe fn HcsDestroyLayer<'a, P0>(layerpath: P0) -> ::windows::core::Result<()>
where
    P0: ::std::convert::Into<::windows::core::PCWSTR>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn HcsDestroyLayer(layerpath: ::windows::core::PCWSTR) -> ::windows::core::HRESULT;
    }
    HcsDestroyLayer(layerpath.into()).ok()
}
#[doc = "*Required features: `\"Win32_System_HostComputeSystem\"`*"]
#[inline]
pub unsafe fn HcsDetachLayerStorageFilter<'a, P0>(layerpath: P0) -> ::windows::core::Result<()>
where
    P0: ::std::convert::Into<::windows::core::PCWSTR>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn HcsDetachLayerStorageFilter(layerpath: ::windows::core::PCWSTR) -> ::windows::core::HRESULT;
    }
    HcsDetachLayerStorageFilter(layerpath.into()).ok()
}
#[doc = "*Required features: `\"Win32_System_HostComputeSystem\"`*"]
#[inline]
pub unsafe fn HcsEnumerateComputeSystems<'a, P0, P1>(query: P0, operation: P1) -> ::windows::core::Result<()>
where
    P0: ::std::convert::Into<::windows::core::PCWSTR>,
    P1: ::std::convert::Into<HCS_OPERATION>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn HcsEnumerateComputeSystems(query: ::windows::core::PCWSTR, operation: HCS_OPERATION) -> ::windows::core::HRESULT;
    }
    HcsEnumerateComputeSystems(query.into(), operation.into()).ok()
}
#[doc = "*Required features: `\"Win32_System_HostComputeSystem\"`*"]
#[inline]
pub unsafe fn HcsEnumerateComputeSystemsInNamespace<'a, P0, P1, P2>(idnamespace: P0, query: P1, operation: P2) -> ::windows::core::Result<()>
where
    P0: ::std::convert::Into<::windows::core::PCWSTR>,
    P1: ::std::convert::Into<::windows::core::PCWSTR>,
    P2: ::std::convert::Into<HCS_OPERATION>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn HcsEnumerateComputeSystemsInNamespace(idnamespace: ::windows::core::PCWSTR, query: ::windows::core::PCWSTR, operation: HCS_OPERATION) -> ::windows::core::HRESULT;
    }
    HcsEnumerateComputeSystemsInNamespace(idnamespace.into(), query.into(), operation.into()).ok()
}
#[doc = "*Required features: `\"Win32_System_HostComputeSystem\"`*"]
#[inline]
pub unsafe fn HcsExportLayer<'a, P0, P1, P2, P3>(layerpath: P0, exportfolderpath: P1, layerdata: P2, options: P3) -> ::windows::core::Result<()>
where
    P0: ::std::convert::Into<::windows::core::PCWSTR>,
    P1: ::std::convert::Into<::windows::core::PCWSTR>,
    P2: ::std::convert::Into<::windows::core::PCWSTR>,
    P3: ::std::convert::Into<::windows::core::PCWSTR>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn HcsExportLayer(layerpath: ::windows::core::PCWSTR, exportfolderpath: ::windows::core::PCWSTR, layerdata: ::windows::core::PCWSTR, options: ::windows::core::PCWSTR) -> ::windows::core::HRESULT;
    }
    HcsExportLayer(layerpath.into(), exportfolderpath.into(), layerdata.into(), options.into()).ok()
}
#[doc = "*Required features: `\"Win32_System_HostComputeSystem\"`*"]
#[inline]
pub unsafe fn HcsExportLegacyWritableLayer<'a, P0, P1, P2, P3>(writablelayermountpath: P0, writablelayerfolderpath: P1, exportfolderpath: P2, layerdata: P3) -> ::windows::core::Result<()>
where
    P0: ::std::convert::Into<::windows::core::PCWSTR>,
    P1: ::std::convert::Into<::windows::core::PCWSTR>,
    P2: ::std::convert::Into<::windows::core::PCWSTR>,
    P3: ::std::convert::Into<::windows::core::PCWSTR>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn HcsExportLegacyWritableLayer(writablelayermountpath: ::windows::core::PCWSTR, writablelayerfolderpath: ::windows::core::PCWSTR, exportfolderpath: ::windows::core::PCWSTR, layerdata: ::windows::core::PCWSTR) -> ::windows::core::HRESULT;
    }
    HcsExportLegacyWritableLayer(writablelayermountpath.into(), writablelayerfolderpath.into(), exportfolderpath.into(), layerdata.into()).ok()
}
#[doc = "*Required features: `\"Win32_System_HostComputeSystem\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn HcsFormatWritableLayerVhd<'a, P0>(vhdhandle: P0) -> ::windows::core::Result<()>
where
    P0: ::std::convert::Into<super::super::Foundation::HANDLE>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn HcsFormatWritableLayerVhd(vhdhandle: super::super::Foundation::HANDLE) -> ::windows::core::HRESULT;
    }
    HcsFormatWritableLayerVhd(vhdhandle.into()).ok()
}
#[doc = "*Required features: `\"Win32_System_HostComputeSystem\"`*"]
#[inline]
pub unsafe fn HcsGetComputeSystemFromOperation<'a, P0>(operation: P0) -> HCS_SYSTEM
where
    P0: ::std::convert::Into<HCS_OPERATION>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn HcsGetComputeSystemFromOperation(operation: HCS_OPERATION) -> HCS_SYSTEM;
    }
    HcsGetComputeSystemFromOperation(operation.into())
}
#[doc = "*Required features: `\"Win32_System_HostComputeSystem\"`*"]
#[inline]
pub unsafe fn HcsGetComputeSystemProperties<'a, P0, P1, P2>(computesystem: P0, operation: P1, propertyquery: P2) -> ::windows::core::Result<()>
where
    P0: ::std::convert::Into<HCS_SYSTEM>,
    P1: ::std::convert::Into<HCS_OPERATION>,
    P2: ::std::convert::Into<::windows::core::PCWSTR>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn HcsGetComputeSystemProperties(computesystem: HCS_SYSTEM, operation: HCS_OPERATION, propertyquery: ::windows::core::PCWSTR) -> ::windows::core::HRESULT;
    }
    HcsGetComputeSystemProperties(computesystem.into(), operation.into(), propertyquery.into()).ok()
}
#[doc = "*Required features: `\"Win32_System_HostComputeSystem\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn HcsGetLayerVhdMountPath<'a, P0>(vhdhandle: P0) -> ::windows::core::Result<::windows::core::PWSTR>
where
    P0: ::std::convert::Into<super::super::Foundation::HANDLE>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn HcsGetLayerVhdMountPath(vhdhandle: super::super::Foundation::HANDLE, mountpath: *mut ::windows::core::PWSTR) -> ::windows::core::HRESULT;
    }
    let mut result__ = ::core::mem::MaybeUninit::zeroed();
    HcsGetLayerVhdMountPath(vhdhandle.into(), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::windows::core::PWSTR>(result__)
}
#[doc = "*Required features: `\"Win32_System_HostComputeSystem\"`*"]
#[inline]
pub unsafe fn HcsGetOperationContext<'a, P0>(operation: P0) -> *mut ::core::ffi::c_void
where
    P0: ::std::convert::Into<HCS_OPERATION>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn HcsGetOperationContext(operation: HCS_OPERATION) -> *mut ::core::ffi::c_void;
    }
    HcsGetOperationContext(operation.into())
}
#[doc = "*Required features: `\"Win32_System_HostComputeSystem\"`*"]
#[inline]
pub unsafe fn HcsGetOperationId<'a, P0>(operation: P0) -> u64
where
    P0: ::std::convert::Into<HCS_OPERATION>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn HcsGetOperationId(operation: HCS_OPERATION) -> u64;
    }
    HcsGetOperationId(operation.into())
}
#[doc = "*Required features: `\"Win32_System_HostComputeSystem\"`*"]
#[inline]
pub unsafe fn HcsGetOperationResult<'a, P0>(operation: P0) -> ::windows::core::Result<::windows::core::PWSTR>
where
    P0: ::std::convert::Into<HCS_OPERATION>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn HcsGetOperationResult(operation: HCS_OPERATION, resultdocument: *mut ::windows::core::PWSTR) -> ::windows::core::HRESULT;
    }
    let mut result__ = ::core::mem::MaybeUninit::zeroed();
    HcsGetOperationResult(operation.into(), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::windows::core::PWSTR>(result__)
}
#[doc = "*Required features: `\"Win32_System_HostComputeSystem\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn HcsGetOperationResultAndProcessInfo<'a, P0>(operation: P0, processinformation: *mut HCS_PROCESS_INFORMATION, resultdocument: *mut ::windows::core::PWSTR) -> ::windows::core::Result<()>
where
    P0: ::std::convert::Into<HCS_OPERATION>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn HcsGetOperationResultAndProcessInfo(operation: HCS_OPERATION, processinformation: *mut HCS_PROCESS_INFORMATION, resultdocument: *mut ::windows::core::PWSTR) -> ::windows::core::HRESULT;
    }
    HcsGetOperationResultAndProcessInfo(operation.into(), ::core::mem::transmute(processinformation), ::core::mem::transmute(resultdocument)).ok()
}
#[doc = "*Required features: `\"Win32_System_HostComputeSystem\"`*"]
#[inline]
pub unsafe fn HcsGetOperationType<'a, P0>(operation: P0) -> HCS_OPERATION_TYPE
where
    P0: ::std::convert::Into<HCS_OPERATION>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn HcsGetOperationType(operation: HCS_OPERATION) -> HCS_OPERATION_TYPE;
    }
    HcsGetOperationType(operation.into())
}
#[doc = "*Required features: `\"Win32_System_HostComputeSystem\"`*"]
#[inline]
pub unsafe fn HcsGetProcessFromOperation<'a, P0>(operation: P0) -> HCS_PROCESS
where
    P0: ::std::convert::Into<HCS_OPERATION>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn HcsGetProcessFromOperation(operation: HCS_OPERATION) -> HCS_PROCESS;
    }
    HcsGetProcessFromOperation(operation.into())
}
#[doc = "*Required features: `\"Win32_System_HostComputeSystem\"`*"]
#[inline]
pub unsafe fn HcsGetProcessInfo<'a, P0, P1>(process: P0, operation: P1) -> ::windows::core::Result<()>
where
    P0: ::std::convert::Into<HCS_PROCESS>,
    P1: ::std::convert::Into<HCS_OPERATION>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn HcsGetProcessInfo(process: HCS_PROCESS, operation: HCS_OPERATION) -> ::windows::core::HRESULT;
    }
    HcsGetProcessInfo(process.into(), operation.into()).ok()
}
#[doc = "*Required features: `\"Win32_System_HostComputeSystem\"`*"]
#[inline]
pub unsafe fn HcsGetProcessProperties<'a, P0, P1, P2>(process: P0, operation: P1, propertyquery: P2) -> ::windows::core::Result<()>
where
    P0: ::std::convert::Into<HCS_PROCESS>,
    P1: ::std::convert::Into<HCS_OPERATION>,
    P2: ::std::convert::Into<::windows::core::PCWSTR>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn HcsGetProcessProperties(process: HCS_PROCESS, operation: HCS_OPERATION, propertyquery: ::windows::core::PCWSTR) -> ::windows::core::HRESULT;
    }
    HcsGetProcessProperties(process.into(), operation.into(), propertyquery.into()).ok()
}
#[doc = "*Required features: `\"Win32_System_HostComputeSystem\"`*"]
#[inline]
pub unsafe fn HcsGetProcessorCompatibilityFromSavedState<'a, P0>(runtimefilename: P0) -> ::windows::core::Result<::windows::core::PWSTR>
where
    P0: ::std::convert::Into<::windows::core::PCWSTR>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn HcsGetProcessorCompatibilityFromSavedState(runtimefilename: ::windows::core::PCWSTR, processorfeaturesstring: *mut ::windows::core::PWSTR) -> ::windows::core::HRESULT;
    }
    let mut result__ = ::core::mem::MaybeUninit::zeroed();
    HcsGetProcessorCompatibilityFromSavedState(runtimefilename.into(), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::windows::core::PWSTR>(result__)
}
#[doc = "*Required features: `\"Win32_System_HostComputeSystem\"`*"]
#[inline]
pub unsafe fn HcsGetServiceProperties<'a, P0>(propertyquery: P0) -> ::windows::core::Result<::windows::core::PWSTR>
where
    P0: ::std::convert::Into<::windows::core::PCWSTR>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn HcsGetServiceProperties(propertyquery: ::windows::core::PCWSTR, result: *mut ::windows::core::PWSTR) -> ::windows::core::HRESULT;
    }
    let mut result__ = ::core::mem::MaybeUninit::zeroed();
    HcsGetServiceProperties(propertyquery.into(), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::windows::core::PWSTR>(result__)
}
#[doc = "*Required features: `\"Win32_System_HostComputeSystem\"`*"]
#[inline]
pub unsafe fn HcsGrantVmAccess<'a, P0, P1>(vmid: P0, filepath: P1) -> ::windows::core::Result<()>
where
    P0: ::std::convert::Into<::windows::core::PCWSTR>,
    P1: ::std::convert::Into<::windows::core::PCWSTR>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn HcsGrantVmAccess(vmid: ::windows::core::PCWSTR, filepath: ::windows::core::PCWSTR) -> ::windows::core::HRESULT;
    }
    HcsGrantVmAccess(vmid.into(), filepath.into()).ok()
}
#[doc = "*Required features: `\"Win32_System_HostComputeSystem\"`*"]
#[inline]
pub unsafe fn HcsGrantVmGroupAccess<'a, P0>(filepath: P0) -> ::windows::core::Result<()>
where
    P0: ::std::convert::Into<::windows::core::PCWSTR>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn HcsGrantVmGroupAccess(filepath: ::windows::core::PCWSTR) -> ::windows::core::HRESULT;
    }
    HcsGrantVmGroupAccess(filepath.into()).ok()
}
#[doc = "*Required features: `\"Win32_System_HostComputeSystem\"`*"]
#[inline]
pub unsafe fn HcsImportLayer<'a, P0, P1, P2>(layerpath: P0, sourcefolderpath: P1, layerdata: P2) -> ::windows::core::Result<()>
where
    P0: ::std::convert::Into<::windows::core::PCWSTR>,
    P1: ::std::convert::Into<::windows::core::PCWSTR>,
    P2: ::std::convert::Into<::windows::core::PCWSTR>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn HcsImportLayer(layerpath: ::windows::core::PCWSTR, sourcefolderpath: ::windows::core::PCWSTR, layerdata: ::windows::core::PCWSTR) -> ::windows::core::HRESULT;
    }
    HcsImportLayer(layerpath.into(), sourcefolderpath.into(), layerdata.into()).ok()
}
#[doc = "*Required features: `\"Win32_System_HostComputeSystem\"`*"]
#[inline]
pub unsafe fn HcsInitializeLegacyWritableLayer<'a, P0, P1, P2, P3>(writablelayermountpath: P0, writablelayerfolderpath: P1, layerdata: P2, options: P3) -> ::windows::core::Result<()>
where
    P0: ::std::convert::Into<::windows::core::PCWSTR>,
    P1: ::std::convert::Into<::windows::core::PCWSTR>,
    P2: ::std::convert::Into<::windows::core::PCWSTR>,
    P3: ::std::convert::Into<::windows::core::PCWSTR>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn HcsInitializeLegacyWritableLayer(writablelayermountpath: ::windows::core::PCWSTR, writablelayerfolderpath: ::windows::core::PCWSTR, layerdata: ::windows::core::PCWSTR, options: ::windows::core::PCWSTR) -> ::windows::core::HRESULT;
    }
    HcsInitializeLegacyWritableLayer(writablelayermountpath.into(), writablelayerfolderpath.into(), layerdata.into(), options.into()).ok()
}
#[doc = "*Required features: `\"Win32_System_HostComputeSystem\"`*"]
#[inline]
pub unsafe fn HcsInitializeWritableLayer<'a, P0, P1, P2>(writablelayerpath: P0, layerdata: P1, options: P2) -> ::windows::core::Result<()>
where
    P0: ::std::convert::Into<::windows::core::PCWSTR>,
    P1: ::std::convert::Into<::windows::core::PCWSTR>,
    P2: ::std::convert::Into<::windows::core::PCWSTR>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn HcsInitializeWritableLayer(writablelayerpath: ::windows::core::PCWSTR, layerdata: ::windows::core::PCWSTR, options: ::windows::core::PCWSTR) -> ::windows::core::HRESULT;
    }
    HcsInitializeWritableLayer(writablelayerpath.into(), layerdata.into(), options.into()).ok()
}
#[doc = "*Required features: `\"Win32_System_HostComputeSystem\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn HcsModifyComputeSystem<'a, P0, P1, P2, P3>(computesystem: P0, operation: P1, configuration: P2, identity: P3) -> ::windows::core::Result<()>
where
    P0: ::std::convert::Into<HCS_SYSTEM>,
    P1: ::std::convert::Into<HCS_OPERATION>,
    P2: ::std::convert::Into<::windows::core::PCWSTR>,
    P3: ::std::convert::Into<super::super::Foundation::HANDLE>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn HcsModifyComputeSystem(computesystem: HCS_SYSTEM, operation: HCS_OPERATION, configuration: ::windows::core::PCWSTR, identity: super::super::Foundation::HANDLE) -> ::windows::core::HRESULT;
    }
    HcsModifyComputeSystem(computesystem.into(), operation.into(), configuration.into(), identity.into()).ok()
}
#[doc = "*Required features: `\"Win32_System_HostComputeSystem\"`*"]
#[inline]
pub unsafe fn HcsModifyProcess<'a, P0, P1, P2>(process: P0, operation: P1, settings: P2) -> ::windows::core::Result<()>
where
    P0: ::std::convert::Into<HCS_PROCESS>,
    P1: ::std::convert::Into<HCS_OPERATION>,
    P2: ::std::convert::Into<::windows::core::PCWSTR>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn HcsModifyProcess(process: HCS_PROCESS, operation: HCS_OPERATION, settings: ::windows::core::PCWSTR) -> ::windows::core::HRESULT;
    }
    HcsModifyProcess(process.into(), operation.into(), settings.into()).ok()
}
#[doc = "*Required features: `\"Win32_System_HostComputeSystem\"`*"]
#[inline]
pub unsafe fn HcsModifyServiceSettings<'a, P0>(settings: P0) -> ::windows::core::Result<::windows::core::PWSTR>
where
    P0: ::std::convert::Into<::windows::core::PCWSTR>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn HcsModifyServiceSettings(settings: ::windows::core::PCWSTR, result: *mut ::windows::core::PWSTR) -> ::windows::core::HRESULT;
    }
    let mut result__ = ::core::mem::MaybeUninit::zeroed();
    HcsModifyServiceSettings(settings.into(), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::windows::core::PWSTR>(result__)
}
#[doc = "*Required features: `\"Win32_System_HostComputeSystem\"`*"]
#[inline]
pub unsafe fn HcsOpenComputeSystem<'a, P0>(id: P0, requestedaccess: u32) -> ::windows::core::Result<HCS_SYSTEM>
where
    P0: ::std::convert::Into<::windows::core::PCWSTR>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn HcsOpenComputeSystem(id: ::windows::core::PCWSTR, requestedaccess: u32, computesystem: *mut HCS_SYSTEM) -> ::windows::core::HRESULT;
    }
    let mut result__ = ::core::mem::MaybeUninit::zeroed();
    HcsOpenComputeSystem(id.into(), requestedaccess, ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<HCS_SYSTEM>(result__)
}
#[doc = "*Required features: `\"Win32_System_HostComputeSystem\"`*"]
#[inline]
pub unsafe fn HcsOpenComputeSystemInNamespace<'a, P0, P1>(idnamespace: P0, id: P1, requestedaccess: u32) -> ::windows::core::Result<HCS_SYSTEM>
where
    P0: ::std::convert::Into<::windows::core::PCWSTR>,
    P1: ::std::convert::Into<::windows::core::PCWSTR>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn HcsOpenComputeSystemInNamespace(idnamespace: ::windows::core::PCWSTR, id: ::windows::core::PCWSTR, requestedaccess: u32, computesystem: *mut HCS_SYSTEM) -> ::windows::core::HRESULT;
    }
    let mut result__ = ::core::mem::MaybeUninit::zeroed();
    HcsOpenComputeSystemInNamespace(idnamespace.into(), id.into(), requestedaccess, ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<HCS_SYSTEM>(result__)
}
#[doc = "*Required features: `\"Win32_System_HostComputeSystem\"`*"]
#[inline]
pub unsafe fn HcsOpenProcess<'a, P0>(computesystem: P0, processid: u32, requestedaccess: u32) -> ::windows::core::Result<HCS_PROCESS>
where
    P0: ::std::convert::Into<HCS_SYSTEM>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn HcsOpenProcess(computesystem: HCS_SYSTEM, processid: u32, requestedaccess: u32, process: *mut HCS_PROCESS) -> ::windows::core::HRESULT;
    }
    let mut result__ = ::core::mem::MaybeUninit::zeroed();
    HcsOpenProcess(computesystem.into(), processid, requestedaccess, ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<HCS_PROCESS>(result__)
}
#[doc = "*Required features: `\"Win32_System_HostComputeSystem\"`*"]
#[inline]
pub unsafe fn HcsPauseComputeSystem<'a, P0, P1, P2>(computesystem: P0, operation: P1, options: P2) -> ::windows::core::Result<()>
where
    P0: ::std::convert::Into<HCS_SYSTEM>,
    P1: ::std::convert::Into<HCS_OPERATION>,
    P2: ::std::convert::Into<::windows::core::PCWSTR>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn HcsPauseComputeSystem(computesystem: HCS_SYSTEM, operation: HCS_OPERATION, options: ::windows::core::PCWSTR) -> ::windows::core::HRESULT;
    }
    HcsPauseComputeSystem(computesystem.into(), operation.into(), options.into()).ok()
}
#[doc = "*Required features: `\"Win32_System_HostComputeSystem\"`*"]
#[inline]
pub unsafe fn HcsResumeComputeSystem<'a, P0, P1, P2>(computesystem: P0, operation: P1, options: P2) -> ::windows::core::Result<()>
where
    P0: ::std::convert::Into<HCS_SYSTEM>,
    P1: ::std::convert::Into<HCS_OPERATION>,
    P2: ::std::convert::Into<::windows::core::PCWSTR>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn HcsResumeComputeSystem(computesystem: HCS_SYSTEM, operation: HCS_OPERATION, options: ::windows::core::PCWSTR) -> ::windows::core::HRESULT;
    }
    HcsResumeComputeSystem(computesystem.into(), operation.into(), options.into()).ok()
}
#[doc = "*Required features: `\"Win32_System_HostComputeSystem\"`*"]
#[inline]
pub unsafe fn HcsRevokeVmAccess<'a, P0, P1>(vmid: P0, filepath: P1) -> ::windows::core::Result<()>
where
    P0: ::std::convert::Into<::windows::core::PCWSTR>,
    P1: ::std::convert::Into<::windows::core::PCWSTR>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn HcsRevokeVmAccess(vmid: ::windows::core::PCWSTR, filepath: ::windows::core::PCWSTR) -> ::windows::core::HRESULT;
    }
    HcsRevokeVmAccess(vmid.into(), filepath.into()).ok()
}
#[doc = "*Required features: `\"Win32_System_HostComputeSystem\"`*"]
#[inline]
pub unsafe fn HcsRevokeVmGroupAccess<'a, P0>(filepath: P0) -> ::windows::core::Result<()>
where
    P0: ::std::convert::Into<::windows::core::PCWSTR>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn HcsRevokeVmGroupAccess(filepath: ::windows::core::PCWSTR) -> ::windows::core::HRESULT;
    }
    HcsRevokeVmGroupAccess(filepath.into()).ok()
}
#[doc = "*Required features: `\"Win32_System_HostComputeSystem\"`*"]
#[inline]
pub unsafe fn HcsSaveComputeSystem<'a, P0, P1, P2>(computesystem: P0, operation: P1, options: P2) -> ::windows::core::Result<()>
where
    P0: ::std::convert::Into<HCS_SYSTEM>,
    P1: ::std::convert::Into<HCS_OPERATION>,
    P2: ::std::convert::Into<::windows::core::PCWSTR>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn HcsSaveComputeSystem(computesystem: HCS_SYSTEM, operation: HCS_OPERATION, options: ::windows::core::PCWSTR) -> ::windows::core::HRESULT;
    }
    HcsSaveComputeSystem(computesystem.into(), operation.into(), options.into()).ok()
}
#[doc = "*Required features: `\"Win32_System_HostComputeSystem\"`*"]
#[inline]
pub unsafe fn HcsSetComputeSystemCallback<'a, P0>(computesystem: P0, callbackoptions: HCS_EVENT_OPTIONS, context: *const ::core::ffi::c_void, callback: HCS_EVENT_CALLBACK) -> ::windows::core::Result<()>
where
    P0: ::std::convert::Into<HCS_SYSTEM>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn HcsSetComputeSystemCallback(computesystem: HCS_SYSTEM, callbackoptions: HCS_EVENT_OPTIONS, context: *const ::core::ffi::c_void, callback: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT;
    }
    HcsSetComputeSystemCallback(computesystem.into(), callbackoptions, ::core::mem::transmute(context), ::core::mem::transmute(callback)).ok()
}
#[doc = "*Required features: `\"Win32_System_HostComputeSystem\"`*"]
#[inline]
pub unsafe fn HcsSetOperationCallback<'a, P0>(operation: P0, context: *const ::core::ffi::c_void, callback: HCS_OPERATION_COMPLETION) -> ::windows::core::Result<()>
where
    P0: ::std::convert::Into<HCS_OPERATION>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn HcsSetOperationCallback(operation: HCS_OPERATION, context: *const ::core::ffi::c_void, callback: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT;
    }
    HcsSetOperationCallback(operation.into(), ::core::mem::transmute(context), ::core::mem::transmute(callback)).ok()
}
#[doc = "*Required features: `\"Win32_System_HostComputeSystem\"`*"]
#[inline]
pub unsafe fn HcsSetOperationContext<'a, P0>(operation: P0, context: *const ::core::ffi::c_void) -> ::windows::core::Result<()>
where
    P0: ::std::convert::Into<HCS_OPERATION>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn HcsSetOperationContext(operation: HCS_OPERATION, context: *const ::core::ffi::c_void) -> ::windows::core::HRESULT;
    }
    HcsSetOperationContext(operation.into(), ::core::mem::transmute(context)).ok()
}
#[doc = "*Required features: `\"Win32_System_HostComputeSystem\"`*"]
#[inline]
pub unsafe fn HcsSetProcessCallback<'a, P0>(process: P0, callbackoptions: HCS_EVENT_OPTIONS, context: *const ::core::ffi::c_void, callback: HCS_EVENT_CALLBACK) -> ::windows::core::Result<()>
where
    P0: ::std::convert::Into<HCS_PROCESS>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn HcsSetProcessCallback(process: HCS_PROCESS, callbackoptions: HCS_EVENT_OPTIONS, context: *const ::core::ffi::c_void, callback: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT;
    }
    HcsSetProcessCallback(process.into(), callbackoptions, ::core::mem::transmute(context), ::core::mem::transmute(callback)).ok()
}
#[doc = "*Required features: `\"Win32_System_HostComputeSystem\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn HcsSetupBaseOSLayer<'a, P0, P1, P2>(layerpath: P0, vhdhandle: P1, options: P2) -> ::windows::core::Result<()>
where
    P0: ::std::convert::Into<::windows::core::PCWSTR>,
    P1: ::std::convert::Into<super::super::Foundation::HANDLE>,
    P2: ::std::convert::Into<::windows::core::PCWSTR>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn HcsSetupBaseOSLayer(layerpath: ::windows::core::PCWSTR, vhdhandle: super::super::Foundation::HANDLE, options: ::windows::core::PCWSTR) -> ::windows::core::HRESULT;
    }
    HcsSetupBaseOSLayer(layerpath.into(), vhdhandle.into(), options.into()).ok()
}
#[doc = "*Required features: `\"Win32_System_HostComputeSystem\"`*"]
#[inline]
pub unsafe fn HcsSetupBaseOSVolume<'a, P0, P1, P2>(layerpath: P0, volumepath: P1, options: P2) -> ::windows::core::Result<()>
where
    P0: ::std::convert::Into<::windows::core::PCWSTR>,
    P1: ::std::convert::Into<::windows::core::PCWSTR>,
    P2: ::std::convert::Into<::windows::core::PCWSTR>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn HcsSetupBaseOSVolume(layerpath: ::windows::core::PCWSTR, volumepath: ::windows::core::PCWSTR, options: ::windows::core::PCWSTR) -> ::windows::core::HRESULT;
    }
    HcsSetupBaseOSVolume(layerpath.into(), volumepath.into(), options.into()).ok()
}
#[doc = "*Required features: `\"Win32_System_HostComputeSystem\"`*"]
#[inline]
pub unsafe fn HcsShutDownComputeSystem<'a, P0, P1, P2>(computesystem: P0, operation: P1, options: P2) -> ::windows::core::Result<()>
where
    P0: ::std::convert::Into<HCS_SYSTEM>,
    P1: ::std::convert::Into<HCS_OPERATION>,
    P2: ::std::convert::Into<::windows::core::PCWSTR>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn HcsShutDownComputeSystem(computesystem: HCS_SYSTEM, operation: HCS_OPERATION, options: ::windows::core::PCWSTR) -> ::windows::core::HRESULT;
    }
    HcsShutDownComputeSystem(computesystem.into(), operation.into(), options.into()).ok()
}
#[doc = "*Required features: `\"Win32_System_HostComputeSystem\"`*"]
#[inline]
pub unsafe fn HcsSignalProcess<'a, P0, P1, P2>(process: P0, operation: P1, options: P2) -> ::windows::core::Result<()>
where
    P0: ::std::convert::Into<HCS_PROCESS>,
    P1: ::std::convert::Into<HCS_OPERATION>,
    P2: ::std::convert::Into<::windows::core::PCWSTR>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn HcsSignalProcess(process: HCS_PROCESS, operation: HCS_OPERATION, options: ::windows::core::PCWSTR) -> ::windows::core::HRESULT;
    }
    HcsSignalProcess(process.into(), operation.into(), options.into()).ok()
}
#[doc = "*Required features: `\"Win32_System_HostComputeSystem\"`*"]
#[inline]
pub unsafe fn HcsStartComputeSystem<'a, P0, P1, P2>(computesystem: P0, operation: P1, options: P2) -> ::windows::core::Result<()>
where
    P0: ::std::convert::Into<HCS_SYSTEM>,
    P1: ::std::convert::Into<HCS_OPERATION>,
    P2: ::std::convert::Into<::windows::core::PCWSTR>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn HcsStartComputeSystem(computesystem: HCS_SYSTEM, operation: HCS_OPERATION, options: ::windows::core::PCWSTR) -> ::windows::core::HRESULT;
    }
    HcsStartComputeSystem(computesystem.into(), operation.into(), options.into()).ok()
}
#[doc = "*Required features: `\"Win32_System_HostComputeSystem\"`*"]
#[inline]
pub unsafe fn HcsSubmitWerReport<'a, P0>(settings: P0) -> ::windows::core::Result<()>
where
    P0: ::std::convert::Into<::windows::core::PCWSTR>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn HcsSubmitWerReport(settings: ::windows::core::PCWSTR) -> ::windows::core::HRESULT;
    }
    HcsSubmitWerReport(settings.into()).ok()
}
#[doc = "*Required features: `\"Win32_System_HostComputeSystem\"`*"]
#[inline]
pub unsafe fn HcsTerminateComputeSystem<'a, P0, P1, P2>(computesystem: P0, operation: P1, options: P2) -> ::windows::core::Result<()>
where
    P0: ::std::convert::Into<HCS_SYSTEM>,
    P1: ::std::convert::Into<HCS_OPERATION>,
    P2: ::std::convert::Into<::windows::core::PCWSTR>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn HcsTerminateComputeSystem(computesystem: HCS_SYSTEM, operation: HCS_OPERATION, options: ::windows::core::PCWSTR) -> ::windows::core::HRESULT;
    }
    HcsTerminateComputeSystem(computesystem.into(), operation.into(), options.into()).ok()
}
#[doc = "*Required features: `\"Win32_System_HostComputeSystem\"`*"]
#[inline]
pub unsafe fn HcsTerminateProcess<'a, P0, P1, P2>(process: P0, operation: P1, options: P2) -> ::windows::core::Result<()>
where
    P0: ::std::convert::Into<HCS_PROCESS>,
    P1: ::std::convert::Into<HCS_OPERATION>,
    P2: ::std::convert::Into<::windows::core::PCWSTR>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn HcsTerminateProcess(process: HCS_PROCESS, operation: HCS_OPERATION, options: ::windows::core::PCWSTR) -> ::windows::core::HRESULT;
    }
    HcsTerminateProcess(process.into(), operation.into(), options.into()).ok()
}
#[doc = "*Required features: `\"Win32_System_HostComputeSystem\"`*"]
#[inline]
pub unsafe fn HcsWaitForComputeSystemExit<'a, P0>(computesystem: P0, timeoutms: u32) -> ::windows::core::Result<::windows::core::PWSTR>
where
    P0: ::std::convert::Into<HCS_SYSTEM>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn HcsWaitForComputeSystemExit(computesystem: HCS_SYSTEM, timeoutms: u32, result: *mut ::windows::core::PWSTR) -> ::windows::core::HRESULT;
    }
    let mut result__ = ::core::mem::MaybeUninit::zeroed();
    HcsWaitForComputeSystemExit(computesystem.into(), timeoutms, ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::windows::core::PWSTR>(result__)
}
#[doc = "*Required features: `\"Win32_System_HostComputeSystem\"`*"]
#[inline]
pub unsafe fn HcsWaitForOperationResult<'a, P0>(operation: P0, timeoutms: u32) -> ::windows::core::Result<::windows::core::PWSTR>
where
    P0: ::std::convert::Into<HCS_OPERATION>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn HcsWaitForOperationResult(operation: HCS_OPERATION, timeoutms: u32, resultdocument: *mut ::windows::core::PWSTR) -> ::windows::core::HRESULT;
    }
    let mut result__ = ::core::mem::MaybeUninit::zeroed();
    HcsWaitForOperationResult(operation.into(), timeoutms, ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::windows::core::PWSTR>(result__)
}
#[doc = "*Required features: `\"Win32_System_HostComputeSystem\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn HcsWaitForOperationResultAndProcessInfo<'a, P0>(operation: P0, timeoutms: u32, processinformation: *mut HCS_PROCESS_INFORMATION, resultdocument: *mut ::windows::core::PWSTR) -> ::windows::core::Result<()>
where
    P0: ::std::convert::Into<HCS_OPERATION>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn HcsWaitForOperationResultAndProcessInfo(operation: HCS_OPERATION, timeoutms: u32, processinformation: *mut HCS_PROCESS_INFORMATION, resultdocument: *mut ::windows::core::PWSTR) -> ::windows::core::HRESULT;
    }
    HcsWaitForOperationResultAndProcessInfo(operation.into(), timeoutms, ::core::mem::transmute(processinformation), ::core::mem::transmute(resultdocument)).ok()
}
#[doc = "*Required features: `\"Win32_System_HostComputeSystem\"`*"]
#[inline]
pub unsafe fn HcsWaitForProcessExit<'a, P0>(computesystem: P0, timeoutms: u32) -> ::windows::core::Result<::windows::core::PWSTR>
where
    P0: ::std::convert::Into<HCS_PROCESS>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn HcsWaitForProcessExit(computesystem: HCS_PROCESS, timeoutms: u32, result: *mut ::windows::core::PWSTR) -> ::windows::core::HRESULT;
    }
    let mut result__ = ::core::mem::MaybeUninit::zeroed();
    HcsWaitForProcessExit(computesystem.into(), timeoutms, ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::windows::core::PWSTR>(result__)
}
#[cfg(feature = "implement")]
::core::include!("impl.rs");

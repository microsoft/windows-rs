#![allow(unused_variables, non_upper_case_globals, non_snake_case, unused_unsafe, non_camel_case_types, dead_code, clippy::all)]
#[doc = "*Required features: `Win32_System_HostComputeSystem`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct HCS_CREATE_OPTIONS(pub i32);
pub const HcsCreateOptions_1: HCS_CREATE_OPTIONS = HCS_CREATE_OPTIONS(65536i32);
impl ::std::convert::From<i32> for HCS_CREATE_OPTIONS {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for HCS_CREATE_OPTIONS {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone)]
#[repr(C)]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
#[doc = "*Required features: `Win32_System_HostComputeSystem`, `Win32_Foundation`, `Win32_Security`*"]
pub struct HCS_CREATE_OPTIONS_1 {
    pub Version: HCS_CREATE_OPTIONS,
    pub UserToken: super::super::Foundation::HANDLE,
    pub SecurityDescriptor: *mut super::super::Security::SECURITY_DESCRIPTOR,
    pub CallbackOptions: HCS_EVENT_OPTIONS,
    pub CallbackContext: *mut ::std::ffi::c_void,
    pub Callback: ::std::option::Option<HCS_EVENT_CALLBACK>,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
impl HCS_CREATE_OPTIONS_1 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
impl ::std::default::Default for HCS_CREATE_OPTIONS_1 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
impl ::std::fmt::Debug for HCS_CREATE_OPTIONS_1 {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("HCS_CREATE_OPTIONS_1").field("Version", &self.Version).field("UserToken", &self.UserToken).field("SecurityDescriptor", &self.SecurityDescriptor).field("CallbackOptions", &self.CallbackOptions).field("CallbackContext", &self.CallbackContext).finish()
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
impl ::std::cmp::PartialEq for HCS_CREATE_OPTIONS_1 {
    fn eq(&self, other: &Self) -> bool {
        self.Version == other.Version && self.UserToken == other.UserToken && self.SecurityDescriptor == other.SecurityDescriptor && self.CallbackOptions == other.CallbackOptions && self.CallbackContext == other.CallbackContext && self.Callback.map(|f| f as usize) == other.Callback.map(|f| f as usize)
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
impl ::std::cmp::Eq for HCS_CREATE_OPTIONS_1 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
unsafe impl ::windows::runtime::Abi for HCS_CREATE_OPTIONS_1 {
    type Abi = ::std::mem::ManuallyDrop<Self>;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
#[doc = "*Required features: `Win32_System_HostComputeSystem`, `Win32_Foundation`*"]
pub struct HCS_EVENT {
    pub Type: HCS_EVENT_TYPE,
    pub EventData: super::super::Foundation::PWSTR,
    pub Operation: HCS_OPERATION,
}
#[cfg(feature = "Win32_Foundation")]
impl HCS_EVENT {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for HCS_EVENT {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::fmt::Debug for HCS_EVENT {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("HCS_EVENT").field("Type", &self.Type).field("EventData", &self.EventData).field("Operation", &self.Operation).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for HCS_EVENT {
    fn eq(&self, other: &Self) -> bool {
        self.Type == other.Type && self.EventData == other.EventData && self.Operation == other.Operation
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for HCS_EVENT {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for HCS_EVENT {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_System_HostComputeSystem`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
pub type HCS_EVENT_CALLBACK = unsafe extern "system" fn(event: *const HCS_EVENT, context: *const ::std::ffi::c_void);
#[doc = "*Required features: `Win32_System_HostComputeSystem`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct HCS_EVENT_OPTIONS(pub u32);
pub const HcsEventOptionNone: HCS_EVENT_OPTIONS = HCS_EVENT_OPTIONS(0u32);
pub const HcsEventOptionEnableOperationCallbacks: HCS_EVENT_OPTIONS = HCS_EVENT_OPTIONS(1u32);
impl ::std::convert::From<u32> for HCS_EVENT_OPTIONS {
    fn from(value: u32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for HCS_EVENT_OPTIONS {
    type Abi = Self;
}
impl ::std::ops::BitOr for HCS_EVENT_OPTIONS {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self {
        Self(self.0 | rhs.0)
    }
}
impl ::std::ops::BitAnd for HCS_EVENT_OPTIONS {
    type Output = Self;
    fn bitand(self, rhs: Self) -> Self {
        Self(self.0 & rhs.0)
    }
}
impl ::std::ops::BitOrAssign for HCS_EVENT_OPTIONS {
    fn bitor_assign(&mut self, rhs: Self) {
        self.0.bitor_assign(rhs.0)
    }
}
impl ::std::ops::BitAndAssign for HCS_EVENT_OPTIONS {
    fn bitand_assign(&mut self, rhs: Self) {
        self.0.bitand_assign(rhs.0)
    }
}
impl ::std::ops::Not for HCS_EVENT_OPTIONS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[doc = "*Required features: `Win32_System_HostComputeSystem`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct HCS_EVENT_TYPE(pub i32);
pub const HcsEventInvalid: HCS_EVENT_TYPE = HCS_EVENT_TYPE(0i32);
pub const HcsEventSystemExited: HCS_EVENT_TYPE = HCS_EVENT_TYPE(1i32);
pub const HcsEventSystemCrashInitiated: HCS_EVENT_TYPE = HCS_EVENT_TYPE(2i32);
pub const HcsEventSystemCrashReport: HCS_EVENT_TYPE = HCS_EVENT_TYPE(3i32);
pub const HcsEventSystemRdpEnhancedModeStateChanged: HCS_EVENT_TYPE = HCS_EVENT_TYPE(4i32);
pub const HcsEventSystemSiloJobCreated: HCS_EVENT_TYPE = HCS_EVENT_TYPE(5i32);
pub const HcsEventSystemGuestConnectionClosed: HCS_EVENT_TYPE = HCS_EVENT_TYPE(6i32);
pub const HcsEventProcessExited: HCS_EVENT_TYPE = HCS_EVENT_TYPE(65536i32);
pub const HcsEventOperationCallback: HCS_EVENT_TYPE = HCS_EVENT_TYPE(16777216i32);
pub const HcsEventServiceDisconnect: HCS_EVENT_TYPE = HCS_EVENT_TYPE(33554432i32);
impl ::std::convert::From<i32> for HCS_EVENT_TYPE {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for HCS_EVENT_TYPE {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_System_HostComputeSystem`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct HCS_NOTIFICATIONS(pub i32);
pub const HcsNotificationInvalid: HCS_NOTIFICATIONS = HCS_NOTIFICATIONS(0i32);
pub const HcsNotificationSystemExited: HCS_NOTIFICATIONS = HCS_NOTIFICATIONS(1i32);
pub const HcsNotificationSystemCreateCompleted: HCS_NOTIFICATIONS = HCS_NOTIFICATIONS(2i32);
pub const HcsNotificationSystemStartCompleted: HCS_NOTIFICATIONS = HCS_NOTIFICATIONS(3i32);
pub const HcsNotificationSystemPauseCompleted: HCS_NOTIFICATIONS = HCS_NOTIFICATIONS(4i32);
pub const HcsNotificationSystemResumeCompleted: HCS_NOTIFICATIONS = HCS_NOTIFICATIONS(5i32);
pub const HcsNotificationSystemCrashReport: HCS_NOTIFICATIONS = HCS_NOTIFICATIONS(6i32);
pub const HcsNotificationSystemSiloJobCreated: HCS_NOTIFICATIONS = HCS_NOTIFICATIONS(7i32);
pub const HcsNotificationSystemSaveCompleted: HCS_NOTIFICATIONS = HCS_NOTIFICATIONS(8i32);
pub const HcsNotificationSystemRdpEnhancedModeStateChanged: HCS_NOTIFICATIONS = HCS_NOTIFICATIONS(9i32);
pub const HcsNotificationSystemShutdownFailed: HCS_NOTIFICATIONS = HCS_NOTIFICATIONS(10i32);
pub const HcsNotificationSystemShutdownCompleted: HCS_NOTIFICATIONS = HCS_NOTIFICATIONS(10i32);
pub const HcsNotificationSystemGetPropertiesCompleted: HCS_NOTIFICATIONS = HCS_NOTIFICATIONS(11i32);
pub const HcsNotificationSystemModifyCompleted: HCS_NOTIFICATIONS = HCS_NOTIFICATIONS(12i32);
pub const HcsNotificationSystemCrashInitiated: HCS_NOTIFICATIONS = HCS_NOTIFICATIONS(13i32);
pub const HcsNotificationSystemGuestConnectionClosed: HCS_NOTIFICATIONS = HCS_NOTIFICATIONS(14i32);
pub const HcsNotificationSystemOperationCompletion: HCS_NOTIFICATIONS = HCS_NOTIFICATIONS(15i32);
pub const HcsNotificationSystemPassThru: HCS_NOTIFICATIONS = HCS_NOTIFICATIONS(16i32);
pub const HcsNotificationProcessExited: HCS_NOTIFICATIONS = HCS_NOTIFICATIONS(65536i32);
pub const HcsNotificationServiceDisconnect: HCS_NOTIFICATIONS = HCS_NOTIFICATIONS(16777216i32);
pub const HcsNotificationFlagsReserved: HCS_NOTIFICATIONS = HCS_NOTIFICATIONS(-268435456i32);
impl ::std::convert::From<i32> for HCS_NOTIFICATIONS {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for HCS_NOTIFICATIONS {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_System_HostComputeSystem`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
pub type HCS_NOTIFICATION_CALLBACK = unsafe extern "system" fn(notificationtype: u32, context: *const ::std::ffi::c_void, notificationstatus: ::windows::runtime::HRESULT, notificationdata: super::super::Foundation::PWSTR);
#[doc = "*Required features: `Win32_System_HostComputeSystem`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct HCS_NOTIFICATION_FLAGS(pub i32);
pub const HcsNotificationFlagSuccess: HCS_NOTIFICATION_FLAGS = HCS_NOTIFICATION_FLAGS(0i32);
pub const HcsNotificationFlagFailure: HCS_NOTIFICATION_FLAGS = HCS_NOTIFICATION_FLAGS(-2147483648i32);
impl ::std::convert::From<i32> for HCS_NOTIFICATION_FLAGS {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for HCS_NOTIFICATION_FLAGS {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy, :: std :: fmt :: Debug, :: std :: cmp :: PartialEq, :: std :: cmp :: Eq)]
#[repr(transparent)]
pub struct HCS_OPERATION(pub isize);
impl ::std::default::Default for HCS_OPERATION {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
unsafe impl ::windows::runtime::Handle for HCS_OPERATION {}
unsafe impl ::windows::runtime::Abi for HCS_OPERATION {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_System_HostComputeSystem`*"]
pub type HCS_OPERATION_COMPLETION = unsafe extern "system" fn(operation: HCS_OPERATION, context: *const ::std::ffi::c_void);
#[doc = "*Required features: `Win32_System_HostComputeSystem`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct HCS_OPERATION_TYPE(pub i32);
pub const HcsOperationTypeNone: HCS_OPERATION_TYPE = HCS_OPERATION_TYPE(-1i32);
pub const HcsOperationTypeEnumerate: HCS_OPERATION_TYPE = HCS_OPERATION_TYPE(0i32);
pub const HcsOperationTypeCreate: HCS_OPERATION_TYPE = HCS_OPERATION_TYPE(1i32);
pub const HcsOperationTypeStart: HCS_OPERATION_TYPE = HCS_OPERATION_TYPE(2i32);
pub const HcsOperationTypeShutdown: HCS_OPERATION_TYPE = HCS_OPERATION_TYPE(3i32);
pub const HcsOperationTypePause: HCS_OPERATION_TYPE = HCS_OPERATION_TYPE(4i32);
pub const HcsOperationTypeResume: HCS_OPERATION_TYPE = HCS_OPERATION_TYPE(5i32);
pub const HcsOperationTypeSave: HCS_OPERATION_TYPE = HCS_OPERATION_TYPE(6i32);
pub const HcsOperationTypeTerminate: HCS_OPERATION_TYPE = HCS_OPERATION_TYPE(7i32);
pub const HcsOperationTypeModify: HCS_OPERATION_TYPE = HCS_OPERATION_TYPE(8i32);
pub const HcsOperationTypeGetProperties: HCS_OPERATION_TYPE = HCS_OPERATION_TYPE(9i32);
pub const HcsOperationTypeCreateProcess: HCS_OPERATION_TYPE = HCS_OPERATION_TYPE(10i32);
pub const HcsOperationTypeSignalProcess: HCS_OPERATION_TYPE = HCS_OPERATION_TYPE(11i32);
pub const HcsOperationTypeGetProcessInfo: HCS_OPERATION_TYPE = HCS_OPERATION_TYPE(12i32);
pub const HcsOperationTypeGetProcessProperties: HCS_OPERATION_TYPE = HCS_OPERATION_TYPE(13i32);
pub const HcsOperationTypeModifyProcess: HCS_OPERATION_TYPE = HCS_OPERATION_TYPE(14i32);
pub const HcsOperationTypeCrash: HCS_OPERATION_TYPE = HCS_OPERATION_TYPE(15i32);
impl ::std::convert::From<i32> for HCS_OPERATION_TYPE {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for HCS_OPERATION_TYPE {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy, :: std :: fmt :: Debug, :: std :: cmp :: PartialEq, :: std :: cmp :: Eq)]
#[repr(transparent)]
pub struct HCS_PROCESS(pub isize);
impl ::std::default::Default for HCS_PROCESS {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
unsafe impl ::windows::runtime::Handle for HCS_PROCESS {}
unsafe impl ::windows::runtime::Abi for HCS_PROCESS {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
#[doc = "*Required features: `Win32_System_HostComputeSystem`, `Win32_Foundation`*"]
pub struct HCS_PROCESS_INFORMATION {
    pub ProcessId: u32,
    pub Reserved: u32,
    pub StdInput: super::super::Foundation::HANDLE,
    pub StdOutput: super::super::Foundation::HANDLE,
    pub StdError: super::super::Foundation::HANDLE,
}
#[cfg(feature = "Win32_Foundation")]
impl HCS_PROCESS_INFORMATION {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for HCS_PROCESS_INFORMATION {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::fmt::Debug for HCS_PROCESS_INFORMATION {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("HCS_PROCESS_INFORMATION").field("ProcessId", &self.ProcessId).field("Reserved", &self.Reserved).field("StdInput", &self.StdInput).field("StdOutput", &self.StdOutput).field("StdError", &self.StdError).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for HCS_PROCESS_INFORMATION {
    fn eq(&self, other: &Self) -> bool {
        self.ProcessId == other.ProcessId && self.Reserved == other.Reserved && self.StdInput == other.StdInput && self.StdOutput == other.StdOutput && self.StdError == other.StdError
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for HCS_PROCESS_INFORMATION {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for HCS_PROCESS_INFORMATION {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy, :: std :: fmt :: Debug, :: std :: cmp :: PartialEq, :: std :: cmp :: Eq)]
#[repr(transparent)]
pub struct HCS_SYSTEM(pub isize);
impl ::std::default::Default for HCS_SYSTEM {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
unsafe impl ::windows::runtime::Handle for HCS_SYSTEM {}
unsafe impl ::windows::runtime::Abi for HCS_SYSTEM {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_System_HostComputeSystem`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn HcsAttachLayerStorageFilter<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>, Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>>(layerpath: Param0, layerdata: Param1) -> ::windows::runtime::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn HcsAttachLayerStorageFilter(layerpath: super::super::Foundation::PWSTR, layerdata: super::super::Foundation::PWSTR) -> ::windows::runtime::HRESULT;
        }
        HcsAttachLayerStorageFilter(layerpath.into_param().abi(), layerdata.into_param().abi()).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_System_HostComputeSystem`*"]
#[inline]
pub unsafe fn HcsCancelOperation<'a, Param0: ::windows::runtime::IntoParam<'a, HCS_OPERATION>>(operation: Param0) -> ::windows::runtime::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn HcsCancelOperation(operation: HCS_OPERATION) -> ::windows::runtime::HRESULT;
        }
        HcsCancelOperation(operation.into_param().abi()).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_System_HostComputeSystem`*"]
#[inline]
pub unsafe fn HcsCloseComputeSystem<'a, Param0: ::windows::runtime::IntoParam<'a, HCS_SYSTEM>>(computesystem: Param0) {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn HcsCloseComputeSystem(computesystem: HCS_SYSTEM);
        }
        ::std::mem::transmute(HcsCloseComputeSystem(computesystem.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_System_HostComputeSystem`*"]
#[inline]
pub unsafe fn HcsCloseOperation<'a, Param0: ::windows::runtime::IntoParam<'a, HCS_OPERATION>>(operation: Param0) {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn HcsCloseOperation(operation: HCS_OPERATION);
        }
        ::std::mem::transmute(HcsCloseOperation(operation.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_System_HostComputeSystem`*"]
#[inline]
pub unsafe fn HcsCloseProcess<'a, Param0: ::windows::runtime::IntoParam<'a, HCS_PROCESS>>(process: Param0) {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn HcsCloseProcess(process: HCS_PROCESS);
        }
        ::std::mem::transmute(HcsCloseProcess(process.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_System_HostComputeSystem`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn HcsCrashComputeSystem<'a, Param0: ::windows::runtime::IntoParam<'a, HCS_SYSTEM>, Param1: ::windows::runtime::IntoParam<'a, HCS_OPERATION>, Param2: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>>(computesystem: Param0, operation: Param1, options: Param2) -> ::windows::runtime::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn HcsCrashComputeSystem(computesystem: HCS_SYSTEM, operation: HCS_OPERATION, options: super::super::Foundation::PWSTR) -> ::windows::runtime::HRESULT;
        }
        HcsCrashComputeSystem(computesystem.into_param().abi(), operation.into_param().abi(), options.into_param().abi()).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_System_HostComputeSystem`, `Win32_Foundation`, `Win32_Security`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
#[inline]
pub unsafe fn HcsCreateComputeSystem<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>, Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>, Param2: ::windows::runtime::IntoParam<'a, HCS_OPERATION>>(id: Param0, configuration: Param1, operation: Param2, securitydescriptor: *const super::super::Security::SECURITY_DESCRIPTOR) -> ::windows::runtime::Result<HCS_SYSTEM> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn HcsCreateComputeSystem(id: super::super::Foundation::PWSTR, configuration: super::super::Foundation::PWSTR, operation: HCS_OPERATION, securitydescriptor: *const super::super::Security::SECURITY_DESCRIPTOR, computesystem: *mut HCS_SYSTEM) -> ::windows::runtime::HRESULT;
        }
        let mut result__: <HCS_SYSTEM as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        HcsCreateComputeSystem(id.into_param().abi(), configuration.into_param().abi(), operation.into_param().abi(), ::std::mem::transmute(securitydescriptor), &mut result__).from_abi::<HCS_SYSTEM>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_System_HostComputeSystem`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn HcsCreateComputeSystemInNamespace<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>, Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>, Param2: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>, Param3: ::windows::runtime::IntoParam<'a, HCS_OPERATION>>(idnamespace: Param0, id: Param1, configuration: Param2, operation: Param3, options: *const HCS_CREATE_OPTIONS) -> ::windows::runtime::Result<HCS_SYSTEM> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn HcsCreateComputeSystemInNamespace(idnamespace: super::super::Foundation::PWSTR, id: super::super::Foundation::PWSTR, configuration: super::super::Foundation::PWSTR, operation: HCS_OPERATION, options: *const HCS_CREATE_OPTIONS, computesystem: *mut HCS_SYSTEM) -> ::windows::runtime::HRESULT;
        }
        let mut result__: <HCS_SYSTEM as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        HcsCreateComputeSystemInNamespace(idnamespace.into_param().abi(), id.into_param().abi(), configuration.into_param().abi(), operation.into_param().abi(), ::std::mem::transmute(options), &mut result__).from_abi::<HCS_SYSTEM>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_System_HostComputeSystem`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn HcsCreateEmptyGuestStateFile<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>>(gueststatefilepath: Param0) -> ::windows::runtime::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn HcsCreateEmptyGuestStateFile(gueststatefilepath: super::super::Foundation::PWSTR) -> ::windows::runtime::HRESULT;
        }
        HcsCreateEmptyGuestStateFile(gueststatefilepath.into_param().abi()).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_System_HostComputeSystem`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn HcsCreateEmptyRuntimeStateFile<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>>(runtimestatefilepath: Param0) -> ::windows::runtime::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn HcsCreateEmptyRuntimeStateFile(runtimestatefilepath: super::super::Foundation::PWSTR) -> ::windows::runtime::HRESULT;
        }
        HcsCreateEmptyRuntimeStateFile(runtimestatefilepath.into_param().abi()).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_System_HostComputeSystem`*"]
#[inline]
pub unsafe fn HcsCreateOperation(context: *const ::std::ffi::c_void, callback: ::std::option::Option<HCS_OPERATION_COMPLETION>) -> HCS_OPERATION {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn HcsCreateOperation(context: *const ::std::ffi::c_void, callback: ::windows::runtime::RawPtr) -> HCS_OPERATION;
        }
        ::std::mem::transmute(HcsCreateOperation(::std::mem::transmute(context), ::std::mem::transmute(callback)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_System_HostComputeSystem`, `Win32_Foundation`, `Win32_Security`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
#[inline]
pub unsafe fn HcsCreateProcess<'a, Param0: ::windows::runtime::IntoParam<'a, HCS_SYSTEM>, Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>, Param2: ::windows::runtime::IntoParam<'a, HCS_OPERATION>>(computesystem: Param0, processparameters: Param1, operation: Param2, securitydescriptor: *const super::super::Security::SECURITY_DESCRIPTOR) -> ::windows::runtime::Result<HCS_PROCESS> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn HcsCreateProcess(computesystem: HCS_SYSTEM, processparameters: super::super::Foundation::PWSTR, operation: HCS_OPERATION, securitydescriptor: *const super::super::Security::SECURITY_DESCRIPTOR, process: *mut HCS_PROCESS) -> ::windows::runtime::HRESULT;
        }
        let mut result__: <HCS_PROCESS as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        HcsCreateProcess(computesystem.into_param().abi(), processparameters.into_param().abi(), operation.into_param().abi(), ::std::mem::transmute(securitydescriptor), &mut result__).from_abi::<HCS_PROCESS>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_System_HostComputeSystem`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn HcsDestroyLayer<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>>(layerpath: Param0) -> ::windows::runtime::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn HcsDestroyLayer(layerpath: super::super::Foundation::PWSTR) -> ::windows::runtime::HRESULT;
        }
        HcsDestroyLayer(layerpath.into_param().abi()).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_System_HostComputeSystem`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn HcsDetachLayerStorageFilter<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>>(layerpath: Param0) -> ::windows::runtime::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn HcsDetachLayerStorageFilter(layerpath: super::super::Foundation::PWSTR) -> ::windows::runtime::HRESULT;
        }
        HcsDetachLayerStorageFilter(layerpath.into_param().abi()).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_System_HostComputeSystem`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn HcsEnumerateComputeSystems<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>, Param1: ::windows::runtime::IntoParam<'a, HCS_OPERATION>>(query: Param0, operation: Param1) -> ::windows::runtime::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn HcsEnumerateComputeSystems(query: super::super::Foundation::PWSTR, operation: HCS_OPERATION) -> ::windows::runtime::HRESULT;
        }
        HcsEnumerateComputeSystems(query.into_param().abi(), operation.into_param().abi()).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_System_HostComputeSystem`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn HcsEnumerateComputeSystemsInNamespace<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>, Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>, Param2: ::windows::runtime::IntoParam<'a, HCS_OPERATION>>(idnamespace: Param0, query: Param1, operation: Param2) -> ::windows::runtime::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn HcsEnumerateComputeSystemsInNamespace(idnamespace: super::super::Foundation::PWSTR, query: super::super::Foundation::PWSTR, operation: HCS_OPERATION) -> ::windows::runtime::HRESULT;
        }
        HcsEnumerateComputeSystemsInNamespace(idnamespace.into_param().abi(), query.into_param().abi(), operation.into_param().abi()).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_System_HostComputeSystem`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn HcsExportLayer<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>, Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>, Param2: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>, Param3: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>>(layerpath: Param0, exportfolderpath: Param1, layerdata: Param2, options: Param3) -> ::windows::runtime::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn HcsExportLayer(layerpath: super::super::Foundation::PWSTR, exportfolderpath: super::super::Foundation::PWSTR, layerdata: super::super::Foundation::PWSTR, options: super::super::Foundation::PWSTR) -> ::windows::runtime::HRESULT;
        }
        HcsExportLayer(layerpath.into_param().abi(), exportfolderpath.into_param().abi(), layerdata.into_param().abi(), options.into_param().abi()).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_System_HostComputeSystem`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn HcsExportLegacyWritableLayer<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>, Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>, Param2: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>, Param3: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>>(writablelayermountpath: Param0, writablelayerfolderpath: Param1, exportfolderpath: Param2, layerdata: Param3) -> ::windows::runtime::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn HcsExportLegacyWritableLayer(writablelayermountpath: super::super::Foundation::PWSTR, writablelayerfolderpath: super::super::Foundation::PWSTR, exportfolderpath: super::super::Foundation::PWSTR, layerdata: super::super::Foundation::PWSTR) -> ::windows::runtime::HRESULT;
        }
        HcsExportLegacyWritableLayer(writablelayermountpath.into_param().abi(), writablelayerfolderpath.into_param().abi(), exportfolderpath.into_param().abi(), layerdata.into_param().abi()).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_System_HostComputeSystem`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn HcsFormatWritableLayerVhd<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::HANDLE>>(vhdhandle: Param0) -> ::windows::runtime::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn HcsFormatWritableLayerVhd(vhdhandle: super::super::Foundation::HANDLE) -> ::windows::runtime::HRESULT;
        }
        HcsFormatWritableLayerVhd(vhdhandle.into_param().abi()).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_System_HostComputeSystem`*"]
#[inline]
pub unsafe fn HcsGetComputeSystemFromOperation<'a, Param0: ::windows::runtime::IntoParam<'a, HCS_OPERATION>>(operation: Param0) -> HCS_SYSTEM {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn HcsGetComputeSystemFromOperation(operation: HCS_OPERATION) -> HCS_SYSTEM;
        }
        ::std::mem::transmute(HcsGetComputeSystemFromOperation(operation.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_System_HostComputeSystem`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn HcsGetComputeSystemProperties<'a, Param0: ::windows::runtime::IntoParam<'a, HCS_SYSTEM>, Param1: ::windows::runtime::IntoParam<'a, HCS_OPERATION>, Param2: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>>(computesystem: Param0, operation: Param1, propertyquery: Param2) -> ::windows::runtime::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn HcsGetComputeSystemProperties(computesystem: HCS_SYSTEM, operation: HCS_OPERATION, propertyquery: super::super::Foundation::PWSTR) -> ::windows::runtime::HRESULT;
        }
        HcsGetComputeSystemProperties(computesystem.into_param().abi(), operation.into_param().abi(), propertyquery.into_param().abi()).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_System_HostComputeSystem`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn HcsGetLayerVhdMountPath<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::HANDLE>>(vhdhandle: Param0) -> ::windows::runtime::Result<super::super::Foundation::PWSTR> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn HcsGetLayerVhdMountPath(vhdhandle: super::super::Foundation::HANDLE, mountpath: *mut super::super::Foundation::PWSTR) -> ::windows::runtime::HRESULT;
        }
        let mut result__: <super::super::Foundation::PWSTR as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        HcsGetLayerVhdMountPath(vhdhandle.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::PWSTR>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_System_HostComputeSystem`*"]
#[inline]
pub unsafe fn HcsGetOperationContext<'a, Param0: ::windows::runtime::IntoParam<'a, HCS_OPERATION>>(operation: Param0) -> *mut ::std::ffi::c_void {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn HcsGetOperationContext(operation: HCS_OPERATION) -> *mut ::std::ffi::c_void;
        }
        ::std::mem::transmute(HcsGetOperationContext(operation.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_System_HostComputeSystem`*"]
#[inline]
pub unsafe fn HcsGetOperationId<'a, Param0: ::windows::runtime::IntoParam<'a, HCS_OPERATION>>(operation: Param0) -> u64 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn HcsGetOperationId(operation: HCS_OPERATION) -> u64;
        }
        ::std::mem::transmute(HcsGetOperationId(operation.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_System_HostComputeSystem`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn HcsGetOperationResult<'a, Param0: ::windows::runtime::IntoParam<'a, HCS_OPERATION>>(operation: Param0) -> ::windows::runtime::Result<super::super::Foundation::PWSTR> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn HcsGetOperationResult(operation: HCS_OPERATION, resultdocument: *mut super::super::Foundation::PWSTR) -> ::windows::runtime::HRESULT;
        }
        let mut result__: <super::super::Foundation::PWSTR as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        HcsGetOperationResult(operation.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::PWSTR>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_System_HostComputeSystem`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn HcsGetOperationResultAndProcessInfo<'a, Param0: ::windows::runtime::IntoParam<'a, HCS_OPERATION>>(operation: Param0, processinformation: *mut HCS_PROCESS_INFORMATION, resultdocument: *mut super::super::Foundation::PWSTR) -> ::windows::runtime::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn HcsGetOperationResultAndProcessInfo(operation: HCS_OPERATION, processinformation: *mut HCS_PROCESS_INFORMATION, resultdocument: *mut super::super::Foundation::PWSTR) -> ::windows::runtime::HRESULT;
        }
        HcsGetOperationResultAndProcessInfo(operation.into_param().abi(), ::std::mem::transmute(processinformation), ::std::mem::transmute(resultdocument)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_System_HostComputeSystem`*"]
#[inline]
pub unsafe fn HcsGetOperationType<'a, Param0: ::windows::runtime::IntoParam<'a, HCS_OPERATION>>(operation: Param0) -> HCS_OPERATION_TYPE {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn HcsGetOperationType(operation: HCS_OPERATION) -> HCS_OPERATION_TYPE;
        }
        ::std::mem::transmute(HcsGetOperationType(operation.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_System_HostComputeSystem`*"]
#[inline]
pub unsafe fn HcsGetProcessFromOperation<'a, Param0: ::windows::runtime::IntoParam<'a, HCS_OPERATION>>(operation: Param0) -> HCS_PROCESS {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn HcsGetProcessFromOperation(operation: HCS_OPERATION) -> HCS_PROCESS;
        }
        ::std::mem::transmute(HcsGetProcessFromOperation(operation.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_System_HostComputeSystem`*"]
#[inline]
pub unsafe fn HcsGetProcessInfo<'a, Param0: ::windows::runtime::IntoParam<'a, HCS_PROCESS>, Param1: ::windows::runtime::IntoParam<'a, HCS_OPERATION>>(process: Param0, operation: Param1) -> ::windows::runtime::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn HcsGetProcessInfo(process: HCS_PROCESS, operation: HCS_OPERATION) -> ::windows::runtime::HRESULT;
        }
        HcsGetProcessInfo(process.into_param().abi(), operation.into_param().abi()).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_System_HostComputeSystem`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn HcsGetProcessProperties<'a, Param0: ::windows::runtime::IntoParam<'a, HCS_PROCESS>, Param1: ::windows::runtime::IntoParam<'a, HCS_OPERATION>, Param2: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>>(process: Param0, operation: Param1, propertyquery: Param2) -> ::windows::runtime::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn HcsGetProcessProperties(process: HCS_PROCESS, operation: HCS_OPERATION, propertyquery: super::super::Foundation::PWSTR) -> ::windows::runtime::HRESULT;
        }
        HcsGetProcessProperties(process.into_param().abi(), operation.into_param().abi(), propertyquery.into_param().abi()).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_System_HostComputeSystem`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn HcsGetProcessorCompatibilityFromSavedState<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>>(runtimefilename: Param0) -> ::windows::runtime::Result<super::super::Foundation::PWSTR> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn HcsGetProcessorCompatibilityFromSavedState(runtimefilename: super::super::Foundation::PWSTR, processorfeaturesstring: *mut super::super::Foundation::PWSTR) -> ::windows::runtime::HRESULT;
        }
        let mut result__: <super::super::Foundation::PWSTR as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        HcsGetProcessorCompatibilityFromSavedState(runtimefilename.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::PWSTR>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_System_HostComputeSystem`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn HcsGetServiceProperties<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>>(propertyquery: Param0) -> ::windows::runtime::Result<super::super::Foundation::PWSTR> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn HcsGetServiceProperties(propertyquery: super::super::Foundation::PWSTR, result: *mut super::super::Foundation::PWSTR) -> ::windows::runtime::HRESULT;
        }
        let mut result__: <super::super::Foundation::PWSTR as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        HcsGetServiceProperties(propertyquery.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::PWSTR>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_System_HostComputeSystem`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn HcsGrantVmAccess<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>, Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>>(vmid: Param0, filepath: Param1) -> ::windows::runtime::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn HcsGrantVmAccess(vmid: super::super::Foundation::PWSTR, filepath: super::super::Foundation::PWSTR) -> ::windows::runtime::HRESULT;
        }
        HcsGrantVmAccess(vmid.into_param().abi(), filepath.into_param().abi()).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_System_HostComputeSystem`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn HcsGrantVmGroupAccess<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>>(filepath: Param0) -> ::windows::runtime::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn HcsGrantVmGroupAccess(filepath: super::super::Foundation::PWSTR) -> ::windows::runtime::HRESULT;
        }
        HcsGrantVmGroupAccess(filepath.into_param().abi()).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_System_HostComputeSystem`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn HcsImportLayer<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>, Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>, Param2: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>>(layerpath: Param0, sourcefolderpath: Param1, layerdata: Param2) -> ::windows::runtime::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn HcsImportLayer(layerpath: super::super::Foundation::PWSTR, sourcefolderpath: super::super::Foundation::PWSTR, layerdata: super::super::Foundation::PWSTR) -> ::windows::runtime::HRESULT;
        }
        HcsImportLayer(layerpath.into_param().abi(), sourcefolderpath.into_param().abi(), layerdata.into_param().abi()).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_System_HostComputeSystem`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn HcsInitializeLegacyWritableLayer<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>, Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>, Param2: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>, Param3: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>>(writablelayermountpath: Param0, writablelayerfolderpath: Param1, layerdata: Param2, options: Param3) -> ::windows::runtime::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn HcsInitializeLegacyWritableLayer(writablelayermountpath: super::super::Foundation::PWSTR, writablelayerfolderpath: super::super::Foundation::PWSTR, layerdata: super::super::Foundation::PWSTR, options: super::super::Foundation::PWSTR) -> ::windows::runtime::HRESULT;
        }
        HcsInitializeLegacyWritableLayer(writablelayermountpath.into_param().abi(), writablelayerfolderpath.into_param().abi(), layerdata.into_param().abi(), options.into_param().abi()).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_System_HostComputeSystem`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn HcsInitializeWritableLayer<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>, Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>, Param2: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>>(writablelayerpath: Param0, layerdata: Param1, options: Param2) -> ::windows::runtime::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn HcsInitializeWritableLayer(writablelayerpath: super::super::Foundation::PWSTR, layerdata: super::super::Foundation::PWSTR, options: super::super::Foundation::PWSTR) -> ::windows::runtime::HRESULT;
        }
        HcsInitializeWritableLayer(writablelayerpath.into_param().abi(), layerdata.into_param().abi(), options.into_param().abi()).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_System_HostComputeSystem`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn HcsModifyComputeSystem<'a, Param0: ::windows::runtime::IntoParam<'a, HCS_SYSTEM>, Param1: ::windows::runtime::IntoParam<'a, HCS_OPERATION>, Param2: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>, Param3: ::windows::runtime::IntoParam<'a, super::super::Foundation::HANDLE>>(computesystem: Param0, operation: Param1, configuration: Param2, identity: Param3) -> ::windows::runtime::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn HcsModifyComputeSystem(computesystem: HCS_SYSTEM, operation: HCS_OPERATION, configuration: super::super::Foundation::PWSTR, identity: super::super::Foundation::HANDLE) -> ::windows::runtime::HRESULT;
        }
        HcsModifyComputeSystem(computesystem.into_param().abi(), operation.into_param().abi(), configuration.into_param().abi(), identity.into_param().abi()).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_System_HostComputeSystem`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn HcsModifyProcess<'a, Param0: ::windows::runtime::IntoParam<'a, HCS_PROCESS>, Param1: ::windows::runtime::IntoParam<'a, HCS_OPERATION>, Param2: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>>(process: Param0, operation: Param1, settings: Param2) -> ::windows::runtime::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn HcsModifyProcess(process: HCS_PROCESS, operation: HCS_OPERATION, settings: super::super::Foundation::PWSTR) -> ::windows::runtime::HRESULT;
        }
        HcsModifyProcess(process.into_param().abi(), operation.into_param().abi(), settings.into_param().abi()).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_System_HostComputeSystem`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn HcsModifyServiceSettings<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>>(settings: Param0) -> ::windows::runtime::Result<super::super::Foundation::PWSTR> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn HcsModifyServiceSettings(settings: super::super::Foundation::PWSTR, result: *mut super::super::Foundation::PWSTR) -> ::windows::runtime::HRESULT;
        }
        let mut result__: <super::super::Foundation::PWSTR as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        HcsModifyServiceSettings(settings.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::PWSTR>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_System_HostComputeSystem`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn HcsOpenComputeSystem<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>>(id: Param0, requestedaccess: u32) -> ::windows::runtime::Result<HCS_SYSTEM> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn HcsOpenComputeSystem(id: super::super::Foundation::PWSTR, requestedaccess: u32, computesystem: *mut HCS_SYSTEM) -> ::windows::runtime::HRESULT;
        }
        let mut result__: <HCS_SYSTEM as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        HcsOpenComputeSystem(id.into_param().abi(), ::std::mem::transmute(requestedaccess), &mut result__).from_abi::<HCS_SYSTEM>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_System_HostComputeSystem`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn HcsOpenComputeSystemInNamespace<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>, Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>>(idnamespace: Param0, id: Param1, requestedaccess: u32) -> ::windows::runtime::Result<HCS_SYSTEM> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn HcsOpenComputeSystemInNamespace(idnamespace: super::super::Foundation::PWSTR, id: super::super::Foundation::PWSTR, requestedaccess: u32, computesystem: *mut HCS_SYSTEM) -> ::windows::runtime::HRESULT;
        }
        let mut result__: <HCS_SYSTEM as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        HcsOpenComputeSystemInNamespace(idnamespace.into_param().abi(), id.into_param().abi(), ::std::mem::transmute(requestedaccess), &mut result__).from_abi::<HCS_SYSTEM>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_System_HostComputeSystem`*"]
#[inline]
pub unsafe fn HcsOpenProcess<'a, Param0: ::windows::runtime::IntoParam<'a, HCS_SYSTEM>>(computesystem: Param0, processid: u32, requestedaccess: u32) -> ::windows::runtime::Result<HCS_PROCESS> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn HcsOpenProcess(computesystem: HCS_SYSTEM, processid: u32, requestedaccess: u32, process: *mut HCS_PROCESS) -> ::windows::runtime::HRESULT;
        }
        let mut result__: <HCS_PROCESS as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        HcsOpenProcess(computesystem.into_param().abi(), ::std::mem::transmute(processid), ::std::mem::transmute(requestedaccess), &mut result__).from_abi::<HCS_PROCESS>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_System_HostComputeSystem`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn HcsPauseComputeSystem<'a, Param0: ::windows::runtime::IntoParam<'a, HCS_SYSTEM>, Param1: ::windows::runtime::IntoParam<'a, HCS_OPERATION>, Param2: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>>(computesystem: Param0, operation: Param1, options: Param2) -> ::windows::runtime::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn HcsPauseComputeSystem(computesystem: HCS_SYSTEM, operation: HCS_OPERATION, options: super::super::Foundation::PWSTR) -> ::windows::runtime::HRESULT;
        }
        HcsPauseComputeSystem(computesystem.into_param().abi(), operation.into_param().abi(), options.into_param().abi()).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_System_HostComputeSystem`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn HcsResumeComputeSystem<'a, Param0: ::windows::runtime::IntoParam<'a, HCS_SYSTEM>, Param1: ::windows::runtime::IntoParam<'a, HCS_OPERATION>, Param2: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>>(computesystem: Param0, operation: Param1, options: Param2) -> ::windows::runtime::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn HcsResumeComputeSystem(computesystem: HCS_SYSTEM, operation: HCS_OPERATION, options: super::super::Foundation::PWSTR) -> ::windows::runtime::HRESULT;
        }
        HcsResumeComputeSystem(computesystem.into_param().abi(), operation.into_param().abi(), options.into_param().abi()).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_System_HostComputeSystem`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn HcsRevokeVmAccess<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>, Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>>(vmid: Param0, filepath: Param1) -> ::windows::runtime::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn HcsRevokeVmAccess(vmid: super::super::Foundation::PWSTR, filepath: super::super::Foundation::PWSTR) -> ::windows::runtime::HRESULT;
        }
        HcsRevokeVmAccess(vmid.into_param().abi(), filepath.into_param().abi()).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_System_HostComputeSystem`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn HcsRevokeVmGroupAccess<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>>(filepath: Param0) -> ::windows::runtime::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn HcsRevokeVmGroupAccess(filepath: super::super::Foundation::PWSTR) -> ::windows::runtime::HRESULT;
        }
        HcsRevokeVmGroupAccess(filepath.into_param().abi()).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_System_HostComputeSystem`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn HcsSaveComputeSystem<'a, Param0: ::windows::runtime::IntoParam<'a, HCS_SYSTEM>, Param1: ::windows::runtime::IntoParam<'a, HCS_OPERATION>, Param2: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>>(computesystem: Param0, operation: Param1, options: Param2) -> ::windows::runtime::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn HcsSaveComputeSystem(computesystem: HCS_SYSTEM, operation: HCS_OPERATION, options: super::super::Foundation::PWSTR) -> ::windows::runtime::HRESULT;
        }
        HcsSaveComputeSystem(computesystem.into_param().abi(), operation.into_param().abi(), options.into_param().abi()).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_System_HostComputeSystem`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn HcsSetComputeSystemCallback<'a, Param0: ::windows::runtime::IntoParam<'a, HCS_SYSTEM>>(computesystem: Param0, callbackoptions: HCS_EVENT_OPTIONS, context: *const ::std::ffi::c_void, callback: ::std::option::Option<HCS_EVENT_CALLBACK>) -> ::windows::runtime::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn HcsSetComputeSystemCallback(computesystem: HCS_SYSTEM, callbackoptions: HCS_EVENT_OPTIONS, context: *const ::std::ffi::c_void, callback: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT;
        }
        HcsSetComputeSystemCallback(computesystem.into_param().abi(), ::std::mem::transmute(callbackoptions), ::std::mem::transmute(context), ::std::mem::transmute(callback)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_System_HostComputeSystem`*"]
#[inline]
pub unsafe fn HcsSetOperationCallback<'a, Param0: ::windows::runtime::IntoParam<'a, HCS_OPERATION>>(operation: Param0, context: *const ::std::ffi::c_void, callback: ::std::option::Option<HCS_OPERATION_COMPLETION>) -> ::windows::runtime::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn HcsSetOperationCallback(operation: HCS_OPERATION, context: *const ::std::ffi::c_void, callback: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT;
        }
        HcsSetOperationCallback(operation.into_param().abi(), ::std::mem::transmute(context), ::std::mem::transmute(callback)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_System_HostComputeSystem`*"]
#[inline]
pub unsafe fn HcsSetOperationContext<'a, Param0: ::windows::runtime::IntoParam<'a, HCS_OPERATION>>(operation: Param0, context: *const ::std::ffi::c_void) -> ::windows::runtime::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn HcsSetOperationContext(operation: HCS_OPERATION, context: *const ::std::ffi::c_void) -> ::windows::runtime::HRESULT;
        }
        HcsSetOperationContext(operation.into_param().abi(), ::std::mem::transmute(context)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_System_HostComputeSystem`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn HcsSetProcessCallback<'a, Param0: ::windows::runtime::IntoParam<'a, HCS_PROCESS>>(process: Param0, callbackoptions: HCS_EVENT_OPTIONS, context: *const ::std::ffi::c_void, callback: ::std::option::Option<HCS_EVENT_CALLBACK>) -> ::windows::runtime::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn HcsSetProcessCallback(process: HCS_PROCESS, callbackoptions: HCS_EVENT_OPTIONS, context: *const ::std::ffi::c_void, callback: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT;
        }
        HcsSetProcessCallback(process.into_param().abi(), ::std::mem::transmute(callbackoptions), ::std::mem::transmute(context), ::std::mem::transmute(callback)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_System_HostComputeSystem`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn HcsSetupBaseOSLayer<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>, Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::HANDLE>, Param2: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>>(layerpath: Param0, vhdhandle: Param1, options: Param2) -> ::windows::runtime::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn HcsSetupBaseOSLayer(layerpath: super::super::Foundation::PWSTR, vhdhandle: super::super::Foundation::HANDLE, options: super::super::Foundation::PWSTR) -> ::windows::runtime::HRESULT;
        }
        HcsSetupBaseOSLayer(layerpath.into_param().abi(), vhdhandle.into_param().abi(), options.into_param().abi()).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_System_HostComputeSystem`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn HcsSetupBaseOSVolume<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>, Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>, Param2: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>>(layerpath: Param0, volumepath: Param1, options: Param2) -> ::windows::runtime::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn HcsSetupBaseOSVolume(layerpath: super::super::Foundation::PWSTR, volumepath: super::super::Foundation::PWSTR, options: super::super::Foundation::PWSTR) -> ::windows::runtime::HRESULT;
        }
        HcsSetupBaseOSVolume(layerpath.into_param().abi(), volumepath.into_param().abi(), options.into_param().abi()).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_System_HostComputeSystem`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn HcsShutDownComputeSystem<'a, Param0: ::windows::runtime::IntoParam<'a, HCS_SYSTEM>, Param1: ::windows::runtime::IntoParam<'a, HCS_OPERATION>, Param2: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>>(computesystem: Param0, operation: Param1, options: Param2) -> ::windows::runtime::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn HcsShutDownComputeSystem(computesystem: HCS_SYSTEM, operation: HCS_OPERATION, options: super::super::Foundation::PWSTR) -> ::windows::runtime::HRESULT;
        }
        HcsShutDownComputeSystem(computesystem.into_param().abi(), operation.into_param().abi(), options.into_param().abi()).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_System_HostComputeSystem`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn HcsSignalProcess<'a, Param0: ::windows::runtime::IntoParam<'a, HCS_PROCESS>, Param1: ::windows::runtime::IntoParam<'a, HCS_OPERATION>, Param2: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>>(process: Param0, operation: Param1, options: Param2) -> ::windows::runtime::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn HcsSignalProcess(process: HCS_PROCESS, operation: HCS_OPERATION, options: super::super::Foundation::PWSTR) -> ::windows::runtime::HRESULT;
        }
        HcsSignalProcess(process.into_param().abi(), operation.into_param().abi(), options.into_param().abi()).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_System_HostComputeSystem`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn HcsStartComputeSystem<'a, Param0: ::windows::runtime::IntoParam<'a, HCS_SYSTEM>, Param1: ::windows::runtime::IntoParam<'a, HCS_OPERATION>, Param2: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>>(computesystem: Param0, operation: Param1, options: Param2) -> ::windows::runtime::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn HcsStartComputeSystem(computesystem: HCS_SYSTEM, operation: HCS_OPERATION, options: super::super::Foundation::PWSTR) -> ::windows::runtime::HRESULT;
        }
        HcsStartComputeSystem(computesystem.into_param().abi(), operation.into_param().abi(), options.into_param().abi()).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_System_HostComputeSystem`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn HcsSubmitWerReport<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>>(settings: Param0) -> ::windows::runtime::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn HcsSubmitWerReport(settings: super::super::Foundation::PWSTR) -> ::windows::runtime::HRESULT;
        }
        HcsSubmitWerReport(settings.into_param().abi()).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_System_HostComputeSystem`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn HcsTerminateComputeSystem<'a, Param0: ::windows::runtime::IntoParam<'a, HCS_SYSTEM>, Param1: ::windows::runtime::IntoParam<'a, HCS_OPERATION>, Param2: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>>(computesystem: Param0, operation: Param1, options: Param2) -> ::windows::runtime::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn HcsTerminateComputeSystem(computesystem: HCS_SYSTEM, operation: HCS_OPERATION, options: super::super::Foundation::PWSTR) -> ::windows::runtime::HRESULT;
        }
        HcsTerminateComputeSystem(computesystem.into_param().abi(), operation.into_param().abi(), options.into_param().abi()).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_System_HostComputeSystem`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn HcsTerminateProcess<'a, Param0: ::windows::runtime::IntoParam<'a, HCS_PROCESS>, Param1: ::windows::runtime::IntoParam<'a, HCS_OPERATION>, Param2: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>>(process: Param0, operation: Param1, options: Param2) -> ::windows::runtime::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn HcsTerminateProcess(process: HCS_PROCESS, operation: HCS_OPERATION, options: super::super::Foundation::PWSTR) -> ::windows::runtime::HRESULT;
        }
        HcsTerminateProcess(process.into_param().abi(), operation.into_param().abi(), options.into_param().abi()).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_System_HostComputeSystem`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn HcsWaitForComputeSystemExit<'a, Param0: ::windows::runtime::IntoParam<'a, HCS_SYSTEM>>(computesystem: Param0, timeoutms: u32) -> ::windows::runtime::Result<super::super::Foundation::PWSTR> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn HcsWaitForComputeSystemExit(computesystem: HCS_SYSTEM, timeoutms: u32, result: *mut super::super::Foundation::PWSTR) -> ::windows::runtime::HRESULT;
        }
        let mut result__: <super::super::Foundation::PWSTR as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        HcsWaitForComputeSystemExit(computesystem.into_param().abi(), ::std::mem::transmute(timeoutms), &mut result__).from_abi::<super::super::Foundation::PWSTR>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_System_HostComputeSystem`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn HcsWaitForOperationResult<'a, Param0: ::windows::runtime::IntoParam<'a, HCS_OPERATION>>(operation: Param0, timeoutms: u32) -> ::windows::runtime::Result<super::super::Foundation::PWSTR> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn HcsWaitForOperationResult(operation: HCS_OPERATION, timeoutms: u32, resultdocument: *mut super::super::Foundation::PWSTR) -> ::windows::runtime::HRESULT;
        }
        let mut result__: <super::super::Foundation::PWSTR as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        HcsWaitForOperationResult(operation.into_param().abi(), ::std::mem::transmute(timeoutms), &mut result__).from_abi::<super::super::Foundation::PWSTR>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_System_HostComputeSystem`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn HcsWaitForOperationResultAndProcessInfo<'a, Param0: ::windows::runtime::IntoParam<'a, HCS_OPERATION>>(operation: Param0, timeoutms: u32, processinformation: *mut HCS_PROCESS_INFORMATION, resultdocument: *mut super::super::Foundation::PWSTR) -> ::windows::runtime::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn HcsWaitForOperationResultAndProcessInfo(operation: HCS_OPERATION, timeoutms: u32, processinformation: *mut HCS_PROCESS_INFORMATION, resultdocument: *mut super::super::Foundation::PWSTR) -> ::windows::runtime::HRESULT;
        }
        HcsWaitForOperationResultAndProcessInfo(operation.into_param().abi(), ::std::mem::transmute(timeoutms), ::std::mem::transmute(processinformation), ::std::mem::transmute(resultdocument)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_System_HostComputeSystem`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn HcsWaitForProcessExit<'a, Param0: ::windows::runtime::IntoParam<'a, HCS_PROCESS>>(computesystem: Param0, timeoutms: u32) -> ::windows::runtime::Result<super::super::Foundation::PWSTR> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn HcsWaitForProcessExit(computesystem: HCS_PROCESS, timeoutms: u32, result: *mut super::super::Foundation::PWSTR) -> ::windows::runtime::HRESULT;
        }
        let mut result__: <super::super::Foundation::PWSTR as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        HcsWaitForProcessExit(computesystem.into_param().abi(), ::std::mem::transmute(timeoutms), &mut result__).from_abi::<super::super::Foundation::PWSTR>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}

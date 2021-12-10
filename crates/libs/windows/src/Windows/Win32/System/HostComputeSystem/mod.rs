#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, clashing_extern_declarations, clippy::all)]
#[doc = "*Required features: 'Win32_System_HostComputeSystem'*"]
pub type HCS_CREATE_OPTIONS = i32;
#[doc = "*Required features: 'Win32_System_HostComputeSystem'*"]
pub const HcsCreateOptions_1: HCS_CREATE_OPTIONS = 65536i32;
#[repr(C)]
#[doc = "*Required features: 'Win32_System_HostComputeSystem', 'Win32_Foundation', 'Win32_Security'*"]
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
#[doc = "*Required features: 'Win32_System_HostComputeSystem', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
pub struct HCS_EVENT {
    pub Type: HCS_EVENT_TYPE,
    pub EventData: super::super::Foundation::PWSTR,
    pub Operation: HCS_OPERATION,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for HCS_EVENT {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for HCS_EVENT {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for HCS_EVENT {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for HCS_EVENT {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<HCS_EVENT>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for HCS_EVENT {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for HCS_EVENT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: 'Win32_System_HostComputeSystem', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
pub type HCS_EVENT_CALLBACK = ::core::option::Option<unsafe extern "system" fn(event: *const HCS_EVENT, context: *const ::core::ffi::c_void)>;
#[doc = "*Required features: 'Win32_System_HostComputeSystem'*"]
pub type HCS_EVENT_OPTIONS = u32;
#[doc = "*Required features: 'Win32_System_HostComputeSystem'*"]
pub const HcsEventOptionNone: HCS_EVENT_OPTIONS = 0u32;
#[doc = "*Required features: 'Win32_System_HostComputeSystem'*"]
pub const HcsEventOptionEnableOperationCallbacks: HCS_EVENT_OPTIONS = 1u32;
#[doc = "*Required features: 'Win32_System_HostComputeSystem'*"]
pub type HCS_EVENT_TYPE = i32;
#[doc = "*Required features: 'Win32_System_HostComputeSystem'*"]
pub const HcsEventInvalid: HCS_EVENT_TYPE = 0i32;
#[doc = "*Required features: 'Win32_System_HostComputeSystem'*"]
pub const HcsEventSystemExited: HCS_EVENT_TYPE = 1i32;
#[doc = "*Required features: 'Win32_System_HostComputeSystem'*"]
pub const HcsEventSystemCrashInitiated: HCS_EVENT_TYPE = 2i32;
#[doc = "*Required features: 'Win32_System_HostComputeSystem'*"]
pub const HcsEventSystemCrashReport: HCS_EVENT_TYPE = 3i32;
#[doc = "*Required features: 'Win32_System_HostComputeSystem'*"]
pub const HcsEventSystemRdpEnhancedModeStateChanged: HCS_EVENT_TYPE = 4i32;
#[doc = "*Required features: 'Win32_System_HostComputeSystem'*"]
pub const HcsEventSystemSiloJobCreated: HCS_EVENT_TYPE = 5i32;
#[doc = "*Required features: 'Win32_System_HostComputeSystem'*"]
pub const HcsEventSystemGuestConnectionClosed: HCS_EVENT_TYPE = 6i32;
#[doc = "*Required features: 'Win32_System_HostComputeSystem'*"]
pub const HcsEventProcessExited: HCS_EVENT_TYPE = 65536i32;
#[doc = "*Required features: 'Win32_System_HostComputeSystem'*"]
pub const HcsEventOperationCallback: HCS_EVENT_TYPE = 16777216i32;
#[doc = "*Required features: 'Win32_System_HostComputeSystem'*"]
pub const HcsEventServiceDisconnect: HCS_EVENT_TYPE = 33554432i32;
#[doc = "*Required features: 'Win32_System_HostComputeSystem'*"]
pub type HCS_NOTIFICATIONS = i32;
#[doc = "*Required features: 'Win32_System_HostComputeSystem'*"]
pub const HcsNotificationInvalid: HCS_NOTIFICATIONS = 0i32;
#[doc = "*Required features: 'Win32_System_HostComputeSystem'*"]
pub const HcsNotificationSystemExited: HCS_NOTIFICATIONS = 1i32;
#[doc = "*Required features: 'Win32_System_HostComputeSystem'*"]
pub const HcsNotificationSystemCreateCompleted: HCS_NOTIFICATIONS = 2i32;
#[doc = "*Required features: 'Win32_System_HostComputeSystem'*"]
pub const HcsNotificationSystemStartCompleted: HCS_NOTIFICATIONS = 3i32;
#[doc = "*Required features: 'Win32_System_HostComputeSystem'*"]
pub const HcsNotificationSystemPauseCompleted: HCS_NOTIFICATIONS = 4i32;
#[doc = "*Required features: 'Win32_System_HostComputeSystem'*"]
pub const HcsNotificationSystemResumeCompleted: HCS_NOTIFICATIONS = 5i32;
#[doc = "*Required features: 'Win32_System_HostComputeSystem'*"]
pub const HcsNotificationSystemCrashReport: HCS_NOTIFICATIONS = 6i32;
#[doc = "*Required features: 'Win32_System_HostComputeSystem'*"]
pub const HcsNotificationSystemSiloJobCreated: HCS_NOTIFICATIONS = 7i32;
#[doc = "*Required features: 'Win32_System_HostComputeSystem'*"]
pub const HcsNotificationSystemSaveCompleted: HCS_NOTIFICATIONS = 8i32;
#[doc = "*Required features: 'Win32_System_HostComputeSystem'*"]
pub const HcsNotificationSystemRdpEnhancedModeStateChanged: HCS_NOTIFICATIONS = 9i32;
#[doc = "*Required features: 'Win32_System_HostComputeSystem'*"]
pub const HcsNotificationSystemShutdownFailed: HCS_NOTIFICATIONS = 10i32;
#[doc = "*Required features: 'Win32_System_HostComputeSystem'*"]
pub const HcsNotificationSystemShutdownCompleted: HCS_NOTIFICATIONS = 10i32;
#[doc = "*Required features: 'Win32_System_HostComputeSystem'*"]
pub const HcsNotificationSystemGetPropertiesCompleted: HCS_NOTIFICATIONS = 11i32;
#[doc = "*Required features: 'Win32_System_HostComputeSystem'*"]
pub const HcsNotificationSystemModifyCompleted: HCS_NOTIFICATIONS = 12i32;
#[doc = "*Required features: 'Win32_System_HostComputeSystem'*"]
pub const HcsNotificationSystemCrashInitiated: HCS_NOTIFICATIONS = 13i32;
#[doc = "*Required features: 'Win32_System_HostComputeSystem'*"]
pub const HcsNotificationSystemGuestConnectionClosed: HCS_NOTIFICATIONS = 14i32;
#[doc = "*Required features: 'Win32_System_HostComputeSystem'*"]
pub const HcsNotificationSystemOperationCompletion: HCS_NOTIFICATIONS = 15i32;
#[doc = "*Required features: 'Win32_System_HostComputeSystem'*"]
pub const HcsNotificationSystemPassThru: HCS_NOTIFICATIONS = 16i32;
#[doc = "*Required features: 'Win32_System_HostComputeSystem'*"]
pub const HcsNotificationProcessExited: HCS_NOTIFICATIONS = 65536i32;
#[doc = "*Required features: 'Win32_System_HostComputeSystem'*"]
pub const HcsNotificationServiceDisconnect: HCS_NOTIFICATIONS = 16777216i32;
#[doc = "*Required features: 'Win32_System_HostComputeSystem'*"]
pub const HcsNotificationFlagsReserved: HCS_NOTIFICATIONS = -268435456i32;
#[doc = "*Required features: 'Win32_System_HostComputeSystem', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
pub type HCS_NOTIFICATION_CALLBACK = ::core::option::Option<unsafe extern "system" fn(notificationtype: u32, context: *const ::core::ffi::c_void, notificationstatus: ::windows::core::HRESULT, notificationdata: super::super::Foundation::PWSTR)>;
#[doc = "*Required features: 'Win32_System_HostComputeSystem'*"]
pub type HCS_NOTIFICATION_FLAGS = i32;
#[doc = "*Required features: 'Win32_System_HostComputeSystem'*"]
pub const HcsNotificationFlagSuccess: HCS_NOTIFICATION_FLAGS = 0i32;
#[doc = "*Required features: 'Win32_System_HostComputeSystem'*"]
pub const HcsNotificationFlagFailure: HCS_NOTIFICATION_FLAGS = -2147483648i32;
pub type HCS_OPERATION = isize;
#[doc = "*Required features: 'Win32_System_HostComputeSystem'*"]
pub type HCS_OPERATION_COMPLETION = ::core::option::Option<unsafe extern "system" fn(operation: HCS_OPERATION, context: *const ::core::ffi::c_void)>;
#[doc = "*Required features: 'Win32_System_HostComputeSystem'*"]
pub type HCS_OPERATION_TYPE = i32;
#[doc = "*Required features: 'Win32_System_HostComputeSystem'*"]
pub const HcsOperationTypeNone: HCS_OPERATION_TYPE = -1i32;
#[doc = "*Required features: 'Win32_System_HostComputeSystem'*"]
pub const HcsOperationTypeEnumerate: HCS_OPERATION_TYPE = 0i32;
#[doc = "*Required features: 'Win32_System_HostComputeSystem'*"]
pub const HcsOperationTypeCreate: HCS_OPERATION_TYPE = 1i32;
#[doc = "*Required features: 'Win32_System_HostComputeSystem'*"]
pub const HcsOperationTypeStart: HCS_OPERATION_TYPE = 2i32;
#[doc = "*Required features: 'Win32_System_HostComputeSystem'*"]
pub const HcsOperationTypeShutdown: HCS_OPERATION_TYPE = 3i32;
#[doc = "*Required features: 'Win32_System_HostComputeSystem'*"]
pub const HcsOperationTypePause: HCS_OPERATION_TYPE = 4i32;
#[doc = "*Required features: 'Win32_System_HostComputeSystem'*"]
pub const HcsOperationTypeResume: HCS_OPERATION_TYPE = 5i32;
#[doc = "*Required features: 'Win32_System_HostComputeSystem'*"]
pub const HcsOperationTypeSave: HCS_OPERATION_TYPE = 6i32;
#[doc = "*Required features: 'Win32_System_HostComputeSystem'*"]
pub const HcsOperationTypeTerminate: HCS_OPERATION_TYPE = 7i32;
#[doc = "*Required features: 'Win32_System_HostComputeSystem'*"]
pub const HcsOperationTypeModify: HCS_OPERATION_TYPE = 8i32;
#[doc = "*Required features: 'Win32_System_HostComputeSystem'*"]
pub const HcsOperationTypeGetProperties: HCS_OPERATION_TYPE = 9i32;
#[doc = "*Required features: 'Win32_System_HostComputeSystem'*"]
pub const HcsOperationTypeCreateProcess: HCS_OPERATION_TYPE = 10i32;
#[doc = "*Required features: 'Win32_System_HostComputeSystem'*"]
pub const HcsOperationTypeSignalProcess: HCS_OPERATION_TYPE = 11i32;
#[doc = "*Required features: 'Win32_System_HostComputeSystem'*"]
pub const HcsOperationTypeGetProcessInfo: HCS_OPERATION_TYPE = 12i32;
#[doc = "*Required features: 'Win32_System_HostComputeSystem'*"]
pub const HcsOperationTypeGetProcessProperties: HCS_OPERATION_TYPE = 13i32;
#[doc = "*Required features: 'Win32_System_HostComputeSystem'*"]
pub const HcsOperationTypeModifyProcess: HCS_OPERATION_TYPE = 14i32;
#[doc = "*Required features: 'Win32_System_HostComputeSystem'*"]
pub const HcsOperationTypeCrash: HCS_OPERATION_TYPE = 15i32;
pub type HCS_PROCESS = isize;
#[repr(C)]
#[doc = "*Required features: 'Win32_System_HostComputeSystem', 'Win32_Foundation'*"]
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
pub type HCS_SYSTEM = isize;
#[doc = "*Required features: 'Win32_System_HostComputeSystem', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn HcsAttachLayerStorageFilter<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(layerpath: Param0, layerdata: Param1) -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn HcsAttachLayerStorageFilter(layerpath: super::super::Foundation::PWSTR, layerdata: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT;
        }
        HcsAttachLayerStorageFilter(layerpath.into_param().abi(), layerdata.into_param().abi()).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_System_HostComputeSystem'*"]
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
#[doc = "*Required features: 'Win32_System_HostComputeSystem'*"]
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
#[doc = "*Required features: 'Win32_System_HostComputeSystem'*"]
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
#[doc = "*Required features: 'Win32_System_HostComputeSystem'*"]
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
#[doc = "*Required features: 'Win32_System_HostComputeSystem', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn HcsCrashComputeSystem<'a, Param0: ::windows::core::IntoParam<'a, HCS_SYSTEM>, Param1: ::windows::core::IntoParam<'a, HCS_OPERATION>, Param2: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(computesystem: Param0, operation: Param1, options: Param2) -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn HcsCrashComputeSystem(computesystem: HCS_SYSTEM, operation: HCS_OPERATION, options: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT;
        }
        HcsCrashComputeSystem(computesystem.into_param().abi(), operation.into_param().abi(), options.into_param().abi()).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_System_HostComputeSystem', 'Win32_Foundation', 'Win32_Security'*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
#[inline]
pub unsafe fn HcsCreateComputeSystem<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>, Param2: ::windows::core::IntoParam<'a, HCS_OPERATION>>(id: Param0, configuration: Param1, operation: Param2, securitydescriptor: *const super::super::Security::SECURITY_DESCRIPTOR) -> ::windows::core::Result<HCS_SYSTEM> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn HcsCreateComputeSystem(id: super::super::Foundation::PWSTR, configuration: super::super::Foundation::PWSTR, operation: HCS_OPERATION, securitydescriptor: *const super::super::Security::SECURITY_DESCRIPTOR, computesystem: *mut HCS_SYSTEM) -> ::windows::core::HRESULT;
        }
        let mut result__: HCS_SYSTEM = ::core::mem::zeroed();
        HcsCreateComputeSystem(id.into_param().abi(), configuration.into_param().abi(), operation.into_param().abi(), ::core::mem::transmute(securitydescriptor), ::core::mem::transmute(&mut result__)).from_abi::<HCS_SYSTEM>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_System_HostComputeSystem', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn HcsCreateComputeSystemInNamespace<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>, Param2: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>, Param3: ::windows::core::IntoParam<'a, HCS_OPERATION>>(idnamespace: Param0, id: Param1, configuration: Param2, operation: Param3, options: *const HCS_CREATE_OPTIONS) -> ::windows::core::Result<HCS_SYSTEM> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn HcsCreateComputeSystemInNamespace(idnamespace: super::super::Foundation::PWSTR, id: super::super::Foundation::PWSTR, configuration: super::super::Foundation::PWSTR, operation: HCS_OPERATION, options: *const HCS_CREATE_OPTIONS, computesystem: *mut HCS_SYSTEM) -> ::windows::core::HRESULT;
        }
        let mut result__: HCS_SYSTEM = ::core::mem::zeroed();
        HcsCreateComputeSystemInNamespace(idnamespace.into_param().abi(), id.into_param().abi(), configuration.into_param().abi(), operation.into_param().abi(), ::core::mem::transmute(options), ::core::mem::transmute(&mut result__)).from_abi::<HCS_SYSTEM>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_System_HostComputeSystem', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn HcsCreateEmptyGuestStateFile<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(gueststatefilepath: Param0) -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn HcsCreateEmptyGuestStateFile(gueststatefilepath: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT;
        }
        HcsCreateEmptyGuestStateFile(gueststatefilepath.into_param().abi()).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_System_HostComputeSystem', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn HcsCreateEmptyRuntimeStateFile<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(runtimestatefilepath: Param0) -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn HcsCreateEmptyRuntimeStateFile(runtimestatefilepath: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT;
        }
        HcsCreateEmptyRuntimeStateFile(runtimestatefilepath.into_param().abi()).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_System_HostComputeSystem'*"]
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
#[doc = "*Required features: 'Win32_System_HostComputeSystem', 'Win32_Foundation', 'Win32_Security'*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
#[inline]
pub unsafe fn HcsCreateProcess<'a, Param0: ::windows::core::IntoParam<'a, HCS_SYSTEM>, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>, Param2: ::windows::core::IntoParam<'a, HCS_OPERATION>>(computesystem: Param0, processparameters: Param1, operation: Param2, securitydescriptor: *const super::super::Security::SECURITY_DESCRIPTOR) -> ::windows::core::Result<HCS_PROCESS> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn HcsCreateProcess(computesystem: HCS_SYSTEM, processparameters: super::super::Foundation::PWSTR, operation: HCS_OPERATION, securitydescriptor: *const super::super::Security::SECURITY_DESCRIPTOR, process: *mut HCS_PROCESS) -> ::windows::core::HRESULT;
        }
        let mut result__: HCS_PROCESS = ::core::mem::zeroed();
        HcsCreateProcess(computesystem.into_param().abi(), processparameters.into_param().abi(), operation.into_param().abi(), ::core::mem::transmute(securitydescriptor), ::core::mem::transmute(&mut result__)).from_abi::<HCS_PROCESS>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_System_HostComputeSystem', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn HcsDestroyLayer<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(layerpath: Param0) -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn HcsDestroyLayer(layerpath: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT;
        }
        HcsDestroyLayer(layerpath.into_param().abi()).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_System_HostComputeSystem', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn HcsDetachLayerStorageFilter<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(layerpath: Param0) -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn HcsDetachLayerStorageFilter(layerpath: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT;
        }
        HcsDetachLayerStorageFilter(layerpath.into_param().abi()).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_System_HostComputeSystem', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn HcsEnumerateComputeSystems<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>, Param1: ::windows::core::IntoParam<'a, HCS_OPERATION>>(query: Param0, operation: Param1) -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn HcsEnumerateComputeSystems(query: super::super::Foundation::PWSTR, operation: HCS_OPERATION) -> ::windows::core::HRESULT;
        }
        HcsEnumerateComputeSystems(query.into_param().abi(), operation.into_param().abi()).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_System_HostComputeSystem', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn HcsEnumerateComputeSystemsInNamespace<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>, Param2: ::windows::core::IntoParam<'a, HCS_OPERATION>>(idnamespace: Param0, query: Param1, operation: Param2) -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn HcsEnumerateComputeSystemsInNamespace(idnamespace: super::super::Foundation::PWSTR, query: super::super::Foundation::PWSTR, operation: HCS_OPERATION) -> ::windows::core::HRESULT;
        }
        HcsEnumerateComputeSystemsInNamespace(idnamespace.into_param().abi(), query.into_param().abi(), operation.into_param().abi()).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_System_HostComputeSystem', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn HcsExportLayer<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>, Param2: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>, Param3: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(layerpath: Param0, exportfolderpath: Param1, layerdata: Param2, options: Param3) -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn HcsExportLayer(layerpath: super::super::Foundation::PWSTR, exportfolderpath: super::super::Foundation::PWSTR, layerdata: super::super::Foundation::PWSTR, options: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT;
        }
        HcsExportLayer(layerpath.into_param().abi(), exportfolderpath.into_param().abi(), layerdata.into_param().abi(), options.into_param().abi()).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_System_HostComputeSystem', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn HcsExportLegacyWritableLayer<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>, Param2: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>, Param3: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(writablelayermountpath: Param0, writablelayerfolderpath: Param1, exportfolderpath: Param2, layerdata: Param3) -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn HcsExportLegacyWritableLayer(writablelayermountpath: super::super::Foundation::PWSTR, writablelayerfolderpath: super::super::Foundation::PWSTR, exportfolderpath: super::super::Foundation::PWSTR, layerdata: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT;
        }
        HcsExportLegacyWritableLayer(writablelayermountpath.into_param().abi(), writablelayerfolderpath.into_param().abi(), exportfolderpath.into_param().abi(), layerdata.into_param().abi()).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_System_HostComputeSystem', 'Win32_Foundation'*"]
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
#[doc = "*Required features: 'Win32_System_HostComputeSystem'*"]
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
#[doc = "*Required features: 'Win32_System_HostComputeSystem', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn HcsGetComputeSystemProperties<'a, Param0: ::windows::core::IntoParam<'a, HCS_SYSTEM>, Param1: ::windows::core::IntoParam<'a, HCS_OPERATION>, Param2: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(computesystem: Param0, operation: Param1, propertyquery: Param2) -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn HcsGetComputeSystemProperties(computesystem: HCS_SYSTEM, operation: HCS_OPERATION, propertyquery: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT;
        }
        HcsGetComputeSystemProperties(computesystem.into_param().abi(), operation.into_param().abi(), propertyquery.into_param().abi()).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_System_HostComputeSystem', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn HcsGetLayerVhdMountPath<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::HANDLE>>(vhdhandle: Param0) -> ::windows::core::Result<super::super::Foundation::PWSTR> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn HcsGetLayerVhdMountPath(vhdhandle: super::super::Foundation::HANDLE, mountpath: *mut super::super::Foundation::PWSTR) -> ::windows::core::HRESULT;
        }
        let mut result__: super::super::Foundation::PWSTR = ::core::mem::zeroed();
        HcsGetLayerVhdMountPath(vhdhandle.into_param().abi(), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::PWSTR>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_System_HostComputeSystem'*"]
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
#[doc = "*Required features: 'Win32_System_HostComputeSystem'*"]
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
#[doc = "*Required features: 'Win32_System_HostComputeSystem', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn HcsGetOperationResult<'a, Param0: ::windows::core::IntoParam<'a, HCS_OPERATION>>(operation: Param0) -> ::windows::core::Result<super::super::Foundation::PWSTR> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn HcsGetOperationResult(operation: HCS_OPERATION, resultdocument: *mut super::super::Foundation::PWSTR) -> ::windows::core::HRESULT;
        }
        let mut result__: super::super::Foundation::PWSTR = ::core::mem::zeroed();
        HcsGetOperationResult(operation.into_param().abi(), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::PWSTR>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_System_HostComputeSystem', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn HcsGetOperationResultAndProcessInfo<'a, Param0: ::windows::core::IntoParam<'a, HCS_OPERATION>>(operation: Param0, processinformation: *mut HCS_PROCESS_INFORMATION, resultdocument: *mut super::super::Foundation::PWSTR) -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn HcsGetOperationResultAndProcessInfo(operation: HCS_OPERATION, processinformation: *mut HCS_PROCESS_INFORMATION, resultdocument: *mut super::super::Foundation::PWSTR) -> ::windows::core::HRESULT;
        }
        HcsGetOperationResultAndProcessInfo(operation.into_param().abi(), ::core::mem::transmute(processinformation), ::core::mem::transmute(resultdocument)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_System_HostComputeSystem'*"]
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
#[doc = "*Required features: 'Win32_System_HostComputeSystem'*"]
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
#[doc = "*Required features: 'Win32_System_HostComputeSystem'*"]
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
#[doc = "*Required features: 'Win32_System_HostComputeSystem', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn HcsGetProcessProperties<'a, Param0: ::windows::core::IntoParam<'a, HCS_PROCESS>, Param1: ::windows::core::IntoParam<'a, HCS_OPERATION>, Param2: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(process: Param0, operation: Param1, propertyquery: Param2) -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn HcsGetProcessProperties(process: HCS_PROCESS, operation: HCS_OPERATION, propertyquery: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT;
        }
        HcsGetProcessProperties(process.into_param().abi(), operation.into_param().abi(), propertyquery.into_param().abi()).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_System_HostComputeSystem', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn HcsGetProcessorCompatibilityFromSavedState<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(runtimefilename: Param0) -> ::windows::core::Result<super::super::Foundation::PWSTR> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn HcsGetProcessorCompatibilityFromSavedState(runtimefilename: super::super::Foundation::PWSTR, processorfeaturesstring: *mut super::super::Foundation::PWSTR) -> ::windows::core::HRESULT;
        }
        let mut result__: super::super::Foundation::PWSTR = ::core::mem::zeroed();
        HcsGetProcessorCompatibilityFromSavedState(runtimefilename.into_param().abi(), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::PWSTR>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_System_HostComputeSystem', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn HcsGetServiceProperties<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(propertyquery: Param0) -> ::windows::core::Result<super::super::Foundation::PWSTR> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn HcsGetServiceProperties(propertyquery: super::super::Foundation::PWSTR, result: *mut super::super::Foundation::PWSTR) -> ::windows::core::HRESULT;
        }
        let mut result__: super::super::Foundation::PWSTR = ::core::mem::zeroed();
        HcsGetServiceProperties(propertyquery.into_param().abi(), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::PWSTR>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_System_HostComputeSystem', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn HcsGrantVmAccess<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(vmid: Param0, filepath: Param1) -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn HcsGrantVmAccess(vmid: super::super::Foundation::PWSTR, filepath: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT;
        }
        HcsGrantVmAccess(vmid.into_param().abi(), filepath.into_param().abi()).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_System_HostComputeSystem', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn HcsGrantVmGroupAccess<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(filepath: Param0) -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn HcsGrantVmGroupAccess(filepath: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT;
        }
        HcsGrantVmGroupAccess(filepath.into_param().abi()).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_System_HostComputeSystem', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn HcsImportLayer<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>, Param2: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(layerpath: Param0, sourcefolderpath: Param1, layerdata: Param2) -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn HcsImportLayer(layerpath: super::super::Foundation::PWSTR, sourcefolderpath: super::super::Foundation::PWSTR, layerdata: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT;
        }
        HcsImportLayer(layerpath.into_param().abi(), sourcefolderpath.into_param().abi(), layerdata.into_param().abi()).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_System_HostComputeSystem', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn HcsInitializeLegacyWritableLayer<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>, Param2: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>, Param3: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(writablelayermountpath: Param0, writablelayerfolderpath: Param1, layerdata: Param2, options: Param3) -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn HcsInitializeLegacyWritableLayer(writablelayermountpath: super::super::Foundation::PWSTR, writablelayerfolderpath: super::super::Foundation::PWSTR, layerdata: super::super::Foundation::PWSTR, options: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT;
        }
        HcsInitializeLegacyWritableLayer(writablelayermountpath.into_param().abi(), writablelayerfolderpath.into_param().abi(), layerdata.into_param().abi(), options.into_param().abi()).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_System_HostComputeSystem', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn HcsInitializeWritableLayer<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>, Param2: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(writablelayerpath: Param0, layerdata: Param1, options: Param2) -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn HcsInitializeWritableLayer(writablelayerpath: super::super::Foundation::PWSTR, layerdata: super::super::Foundation::PWSTR, options: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT;
        }
        HcsInitializeWritableLayer(writablelayerpath.into_param().abi(), layerdata.into_param().abi(), options.into_param().abi()).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_System_HostComputeSystem', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn HcsModifyComputeSystem<'a, Param0: ::windows::core::IntoParam<'a, HCS_SYSTEM>, Param1: ::windows::core::IntoParam<'a, HCS_OPERATION>, Param2: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>, Param3: ::windows::core::IntoParam<'a, super::super::Foundation::HANDLE>>(computesystem: Param0, operation: Param1, configuration: Param2, identity: Param3) -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn HcsModifyComputeSystem(computesystem: HCS_SYSTEM, operation: HCS_OPERATION, configuration: super::super::Foundation::PWSTR, identity: super::super::Foundation::HANDLE) -> ::windows::core::HRESULT;
        }
        HcsModifyComputeSystem(computesystem.into_param().abi(), operation.into_param().abi(), configuration.into_param().abi(), identity.into_param().abi()).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_System_HostComputeSystem', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn HcsModifyProcess<'a, Param0: ::windows::core::IntoParam<'a, HCS_PROCESS>, Param1: ::windows::core::IntoParam<'a, HCS_OPERATION>, Param2: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(process: Param0, operation: Param1, settings: Param2) -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn HcsModifyProcess(process: HCS_PROCESS, operation: HCS_OPERATION, settings: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT;
        }
        HcsModifyProcess(process.into_param().abi(), operation.into_param().abi(), settings.into_param().abi()).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_System_HostComputeSystem', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn HcsModifyServiceSettings<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(settings: Param0) -> ::windows::core::Result<super::super::Foundation::PWSTR> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn HcsModifyServiceSettings(settings: super::super::Foundation::PWSTR, result: *mut super::super::Foundation::PWSTR) -> ::windows::core::HRESULT;
        }
        let mut result__: super::super::Foundation::PWSTR = ::core::mem::zeroed();
        HcsModifyServiceSettings(settings.into_param().abi(), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::PWSTR>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_System_HostComputeSystem', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn HcsOpenComputeSystem<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(id: Param0, requestedaccess: u32) -> ::windows::core::Result<HCS_SYSTEM> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn HcsOpenComputeSystem(id: super::super::Foundation::PWSTR, requestedaccess: u32, computesystem: *mut HCS_SYSTEM) -> ::windows::core::HRESULT;
        }
        let mut result__: HCS_SYSTEM = ::core::mem::zeroed();
        HcsOpenComputeSystem(id.into_param().abi(), ::core::mem::transmute(requestedaccess), ::core::mem::transmute(&mut result__)).from_abi::<HCS_SYSTEM>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_System_HostComputeSystem', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn HcsOpenComputeSystemInNamespace<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(idnamespace: Param0, id: Param1, requestedaccess: u32) -> ::windows::core::Result<HCS_SYSTEM> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn HcsOpenComputeSystemInNamespace(idnamespace: super::super::Foundation::PWSTR, id: super::super::Foundation::PWSTR, requestedaccess: u32, computesystem: *mut HCS_SYSTEM) -> ::windows::core::HRESULT;
        }
        let mut result__: HCS_SYSTEM = ::core::mem::zeroed();
        HcsOpenComputeSystemInNamespace(idnamespace.into_param().abi(), id.into_param().abi(), ::core::mem::transmute(requestedaccess), ::core::mem::transmute(&mut result__)).from_abi::<HCS_SYSTEM>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_System_HostComputeSystem'*"]
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
#[doc = "*Required features: 'Win32_System_HostComputeSystem', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn HcsPauseComputeSystem<'a, Param0: ::windows::core::IntoParam<'a, HCS_SYSTEM>, Param1: ::windows::core::IntoParam<'a, HCS_OPERATION>, Param2: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(computesystem: Param0, operation: Param1, options: Param2) -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn HcsPauseComputeSystem(computesystem: HCS_SYSTEM, operation: HCS_OPERATION, options: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT;
        }
        HcsPauseComputeSystem(computesystem.into_param().abi(), operation.into_param().abi(), options.into_param().abi()).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_System_HostComputeSystem', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn HcsResumeComputeSystem<'a, Param0: ::windows::core::IntoParam<'a, HCS_SYSTEM>, Param1: ::windows::core::IntoParam<'a, HCS_OPERATION>, Param2: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(computesystem: Param0, operation: Param1, options: Param2) -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn HcsResumeComputeSystem(computesystem: HCS_SYSTEM, operation: HCS_OPERATION, options: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT;
        }
        HcsResumeComputeSystem(computesystem.into_param().abi(), operation.into_param().abi(), options.into_param().abi()).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_System_HostComputeSystem', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn HcsRevokeVmAccess<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(vmid: Param0, filepath: Param1) -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn HcsRevokeVmAccess(vmid: super::super::Foundation::PWSTR, filepath: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT;
        }
        HcsRevokeVmAccess(vmid.into_param().abi(), filepath.into_param().abi()).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_System_HostComputeSystem', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn HcsRevokeVmGroupAccess<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(filepath: Param0) -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn HcsRevokeVmGroupAccess(filepath: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT;
        }
        HcsRevokeVmGroupAccess(filepath.into_param().abi()).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_System_HostComputeSystem', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn HcsSaveComputeSystem<'a, Param0: ::windows::core::IntoParam<'a, HCS_SYSTEM>, Param1: ::windows::core::IntoParam<'a, HCS_OPERATION>, Param2: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(computesystem: Param0, operation: Param1, options: Param2) -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn HcsSaveComputeSystem(computesystem: HCS_SYSTEM, operation: HCS_OPERATION, options: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT;
        }
        HcsSaveComputeSystem(computesystem.into_param().abi(), operation.into_param().abi(), options.into_param().abi()).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_System_HostComputeSystem', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
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
#[doc = "*Required features: 'Win32_System_HostComputeSystem'*"]
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
#[doc = "*Required features: 'Win32_System_HostComputeSystem'*"]
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
#[doc = "*Required features: 'Win32_System_HostComputeSystem', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
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
#[doc = "*Required features: 'Win32_System_HostComputeSystem', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn HcsSetupBaseOSLayer<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::HANDLE>, Param2: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(layerpath: Param0, vhdhandle: Param1, options: Param2) -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn HcsSetupBaseOSLayer(layerpath: super::super::Foundation::PWSTR, vhdhandle: super::super::Foundation::HANDLE, options: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT;
        }
        HcsSetupBaseOSLayer(layerpath.into_param().abi(), vhdhandle.into_param().abi(), options.into_param().abi()).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_System_HostComputeSystem', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn HcsSetupBaseOSVolume<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>, Param2: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(layerpath: Param0, volumepath: Param1, options: Param2) -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn HcsSetupBaseOSVolume(layerpath: super::super::Foundation::PWSTR, volumepath: super::super::Foundation::PWSTR, options: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT;
        }
        HcsSetupBaseOSVolume(layerpath.into_param().abi(), volumepath.into_param().abi(), options.into_param().abi()).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_System_HostComputeSystem', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn HcsShutDownComputeSystem<'a, Param0: ::windows::core::IntoParam<'a, HCS_SYSTEM>, Param1: ::windows::core::IntoParam<'a, HCS_OPERATION>, Param2: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(computesystem: Param0, operation: Param1, options: Param2) -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn HcsShutDownComputeSystem(computesystem: HCS_SYSTEM, operation: HCS_OPERATION, options: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT;
        }
        HcsShutDownComputeSystem(computesystem.into_param().abi(), operation.into_param().abi(), options.into_param().abi()).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_System_HostComputeSystem', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn HcsSignalProcess<'a, Param0: ::windows::core::IntoParam<'a, HCS_PROCESS>, Param1: ::windows::core::IntoParam<'a, HCS_OPERATION>, Param2: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(process: Param0, operation: Param1, options: Param2) -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn HcsSignalProcess(process: HCS_PROCESS, operation: HCS_OPERATION, options: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT;
        }
        HcsSignalProcess(process.into_param().abi(), operation.into_param().abi(), options.into_param().abi()).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_System_HostComputeSystem', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn HcsStartComputeSystem<'a, Param0: ::windows::core::IntoParam<'a, HCS_SYSTEM>, Param1: ::windows::core::IntoParam<'a, HCS_OPERATION>, Param2: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(computesystem: Param0, operation: Param1, options: Param2) -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn HcsStartComputeSystem(computesystem: HCS_SYSTEM, operation: HCS_OPERATION, options: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT;
        }
        HcsStartComputeSystem(computesystem.into_param().abi(), operation.into_param().abi(), options.into_param().abi()).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_System_HostComputeSystem', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn HcsSubmitWerReport<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(settings: Param0) -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn HcsSubmitWerReport(settings: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT;
        }
        HcsSubmitWerReport(settings.into_param().abi()).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_System_HostComputeSystem', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn HcsTerminateComputeSystem<'a, Param0: ::windows::core::IntoParam<'a, HCS_SYSTEM>, Param1: ::windows::core::IntoParam<'a, HCS_OPERATION>, Param2: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(computesystem: Param0, operation: Param1, options: Param2) -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn HcsTerminateComputeSystem(computesystem: HCS_SYSTEM, operation: HCS_OPERATION, options: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT;
        }
        HcsTerminateComputeSystem(computesystem.into_param().abi(), operation.into_param().abi(), options.into_param().abi()).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_System_HostComputeSystem', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn HcsTerminateProcess<'a, Param0: ::windows::core::IntoParam<'a, HCS_PROCESS>, Param1: ::windows::core::IntoParam<'a, HCS_OPERATION>, Param2: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(process: Param0, operation: Param1, options: Param2) -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn HcsTerminateProcess(process: HCS_PROCESS, operation: HCS_OPERATION, options: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT;
        }
        HcsTerminateProcess(process.into_param().abi(), operation.into_param().abi(), options.into_param().abi()).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_System_HostComputeSystem', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn HcsWaitForComputeSystemExit<'a, Param0: ::windows::core::IntoParam<'a, HCS_SYSTEM>>(computesystem: Param0, timeoutms: u32) -> ::windows::core::Result<super::super::Foundation::PWSTR> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn HcsWaitForComputeSystemExit(computesystem: HCS_SYSTEM, timeoutms: u32, result: *mut super::super::Foundation::PWSTR) -> ::windows::core::HRESULT;
        }
        let mut result__: super::super::Foundation::PWSTR = ::core::mem::zeroed();
        HcsWaitForComputeSystemExit(computesystem.into_param().abi(), ::core::mem::transmute(timeoutms), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::PWSTR>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_System_HostComputeSystem', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn HcsWaitForOperationResult<'a, Param0: ::windows::core::IntoParam<'a, HCS_OPERATION>>(operation: Param0, timeoutms: u32) -> ::windows::core::Result<super::super::Foundation::PWSTR> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn HcsWaitForOperationResult(operation: HCS_OPERATION, timeoutms: u32, resultdocument: *mut super::super::Foundation::PWSTR) -> ::windows::core::HRESULT;
        }
        let mut result__: super::super::Foundation::PWSTR = ::core::mem::zeroed();
        HcsWaitForOperationResult(operation.into_param().abi(), ::core::mem::transmute(timeoutms), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::PWSTR>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_System_HostComputeSystem', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn HcsWaitForOperationResultAndProcessInfo<'a, Param0: ::windows::core::IntoParam<'a, HCS_OPERATION>>(operation: Param0, timeoutms: u32, processinformation: *mut HCS_PROCESS_INFORMATION, resultdocument: *mut super::super::Foundation::PWSTR) -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn HcsWaitForOperationResultAndProcessInfo(operation: HCS_OPERATION, timeoutms: u32, processinformation: *mut HCS_PROCESS_INFORMATION, resultdocument: *mut super::super::Foundation::PWSTR) -> ::windows::core::HRESULT;
        }
        HcsWaitForOperationResultAndProcessInfo(operation.into_param().abi(), ::core::mem::transmute(timeoutms), ::core::mem::transmute(processinformation), ::core::mem::transmute(resultdocument)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_System_HostComputeSystem', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn HcsWaitForProcessExit<'a, Param0: ::windows::core::IntoParam<'a, HCS_PROCESS>>(computesystem: Param0, timeoutms: u32) -> ::windows::core::Result<super::super::Foundation::PWSTR> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn HcsWaitForProcessExit(computesystem: HCS_PROCESS, timeoutms: u32, result: *mut super::super::Foundation::PWSTR) -> ::windows::core::HRESULT;
        }
        let mut result__: super::super::Foundation::PWSTR = ::core::mem::zeroed();
        HcsWaitForProcessExit(computesystem.into_param().abi(), ::core::mem::transmute(timeoutms), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::PWSTR>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}

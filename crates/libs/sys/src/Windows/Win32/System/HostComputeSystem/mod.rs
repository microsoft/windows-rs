#[link(name = "windows")]
extern "system" {
    #[doc = "*Required features: `\"Win32_System_HostComputeSystem\"`*"]
    pub fn HcsAttachLayerStorageFilter(layerpath: ::windows_sys::core::PCWSTR, layerdata: ::windows_sys::core::PCWSTR) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `\"Win32_System_HostComputeSystem\"`*"]
    pub fn HcsCancelOperation(operation: HCS_OPERATION) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `\"Win32_System_HostComputeSystem\"`*"]
    pub fn HcsCloseComputeSystem(computesystem: HCS_SYSTEM);
    #[doc = "*Required features: `\"Win32_System_HostComputeSystem\"`*"]
    pub fn HcsCloseOperation(operation: HCS_OPERATION);
    #[doc = "*Required features: `\"Win32_System_HostComputeSystem\"`*"]
    pub fn HcsCloseProcess(process: HCS_PROCESS);
    #[doc = "*Required features: `\"Win32_System_HostComputeSystem\"`*"]
    pub fn HcsCrashComputeSystem(computesystem: HCS_SYSTEM, operation: HCS_OPERATION, options: ::windows_sys::core::PCWSTR) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `\"Win32_System_HostComputeSystem\"`, `\"Win32_Foundation\"`, `\"Win32_Security\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
    pub fn HcsCreateComputeSystem(id: ::windows_sys::core::PCWSTR, configuration: ::windows_sys::core::PCWSTR, operation: HCS_OPERATION, securitydescriptor: *const super::super::Security::SECURITY_DESCRIPTOR, computesystem: *mut HCS_SYSTEM) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `\"Win32_System_HostComputeSystem\"`*"]
    pub fn HcsCreateComputeSystemInNamespace(idnamespace: ::windows_sys::core::PCWSTR, id: ::windows_sys::core::PCWSTR, configuration: ::windows_sys::core::PCWSTR, operation: HCS_OPERATION, options: *const HCS_CREATE_OPTIONS, computesystem: *mut HCS_SYSTEM) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `\"Win32_System_HostComputeSystem\"`*"]
    pub fn HcsCreateEmptyGuestStateFile(gueststatefilepath: ::windows_sys::core::PCWSTR) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `\"Win32_System_HostComputeSystem\"`*"]
    pub fn HcsCreateEmptyRuntimeStateFile(runtimestatefilepath: ::windows_sys::core::PCWSTR) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `\"Win32_System_HostComputeSystem\"`*"]
    pub fn HcsCreateOperation(context: *const ::core::ffi::c_void, callback: HCS_OPERATION_COMPLETION) -> HCS_OPERATION;
    #[doc = "*Required features: `\"Win32_System_HostComputeSystem\"`, `\"Win32_Foundation\"`, `\"Win32_Security\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
    pub fn HcsCreateProcess(computesystem: HCS_SYSTEM, processparameters: ::windows_sys::core::PCWSTR, operation: HCS_OPERATION, securitydescriptor: *const super::super::Security::SECURITY_DESCRIPTOR, process: *mut HCS_PROCESS) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `\"Win32_System_HostComputeSystem\"`*"]
    pub fn HcsDestroyLayer(layerpath: ::windows_sys::core::PCWSTR) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `\"Win32_System_HostComputeSystem\"`*"]
    pub fn HcsDetachLayerStorageFilter(layerpath: ::windows_sys::core::PCWSTR) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `\"Win32_System_HostComputeSystem\"`*"]
    pub fn HcsEnumerateComputeSystems(query: ::windows_sys::core::PCWSTR, operation: HCS_OPERATION) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `\"Win32_System_HostComputeSystem\"`*"]
    pub fn HcsEnumerateComputeSystemsInNamespace(idnamespace: ::windows_sys::core::PCWSTR, query: ::windows_sys::core::PCWSTR, operation: HCS_OPERATION) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `\"Win32_System_HostComputeSystem\"`*"]
    pub fn HcsExportLayer(layerpath: ::windows_sys::core::PCWSTR, exportfolderpath: ::windows_sys::core::PCWSTR, layerdata: ::windows_sys::core::PCWSTR, options: ::windows_sys::core::PCWSTR) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `\"Win32_System_HostComputeSystem\"`*"]
    pub fn HcsExportLegacyWritableLayer(writablelayermountpath: ::windows_sys::core::PCWSTR, writablelayerfolderpath: ::windows_sys::core::PCWSTR, exportfolderpath: ::windows_sys::core::PCWSTR, layerdata: ::windows_sys::core::PCWSTR) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `\"Win32_System_HostComputeSystem\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn HcsFormatWritableLayerVhd(vhdhandle: super::super::Foundation::HANDLE) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `\"Win32_System_HostComputeSystem\"`*"]
    pub fn HcsGetComputeSystemFromOperation(operation: HCS_OPERATION) -> HCS_SYSTEM;
    #[doc = "*Required features: `\"Win32_System_HostComputeSystem\"`*"]
    pub fn HcsGetComputeSystemProperties(computesystem: HCS_SYSTEM, operation: HCS_OPERATION, propertyquery: ::windows_sys::core::PCWSTR) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `\"Win32_System_HostComputeSystem\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn HcsGetLayerVhdMountPath(vhdhandle: super::super::Foundation::HANDLE, mountpath: *mut ::windows_sys::core::PWSTR) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `\"Win32_System_HostComputeSystem\"`*"]
    pub fn HcsGetOperationContext(operation: HCS_OPERATION) -> *mut ::core::ffi::c_void;
    #[doc = "*Required features: `\"Win32_System_HostComputeSystem\"`*"]
    pub fn HcsGetOperationId(operation: HCS_OPERATION) -> u64;
    #[doc = "*Required features: `\"Win32_System_HostComputeSystem\"`*"]
    pub fn HcsGetOperationResult(operation: HCS_OPERATION, resultdocument: *mut ::windows_sys::core::PWSTR) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `\"Win32_System_HostComputeSystem\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn HcsGetOperationResultAndProcessInfo(operation: HCS_OPERATION, processinformation: *mut HCS_PROCESS_INFORMATION, resultdocument: *mut ::windows_sys::core::PWSTR) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `\"Win32_System_HostComputeSystem\"`*"]
    pub fn HcsGetOperationType(operation: HCS_OPERATION) -> HCS_OPERATION_TYPE;
    #[doc = "*Required features: `\"Win32_System_HostComputeSystem\"`*"]
    pub fn HcsGetProcessFromOperation(operation: HCS_OPERATION) -> HCS_PROCESS;
    #[doc = "*Required features: `\"Win32_System_HostComputeSystem\"`*"]
    pub fn HcsGetProcessInfo(process: HCS_PROCESS, operation: HCS_OPERATION) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `\"Win32_System_HostComputeSystem\"`*"]
    pub fn HcsGetProcessProperties(process: HCS_PROCESS, operation: HCS_OPERATION, propertyquery: ::windows_sys::core::PCWSTR) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `\"Win32_System_HostComputeSystem\"`*"]
    pub fn HcsGetProcessorCompatibilityFromSavedState(runtimefilename: ::windows_sys::core::PCWSTR, processorfeaturesstring: *mut ::windows_sys::core::PWSTR) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `\"Win32_System_HostComputeSystem\"`*"]
    pub fn HcsGetServiceProperties(propertyquery: ::windows_sys::core::PCWSTR, result: *mut ::windows_sys::core::PWSTR) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `\"Win32_System_HostComputeSystem\"`*"]
    pub fn HcsGrantVmAccess(vmid: ::windows_sys::core::PCWSTR, filepath: ::windows_sys::core::PCWSTR) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `\"Win32_System_HostComputeSystem\"`*"]
    pub fn HcsGrantVmGroupAccess(filepath: ::windows_sys::core::PCWSTR) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `\"Win32_System_HostComputeSystem\"`*"]
    pub fn HcsImportLayer(layerpath: ::windows_sys::core::PCWSTR, sourcefolderpath: ::windows_sys::core::PCWSTR, layerdata: ::windows_sys::core::PCWSTR) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `\"Win32_System_HostComputeSystem\"`*"]
    pub fn HcsInitializeLegacyWritableLayer(writablelayermountpath: ::windows_sys::core::PCWSTR, writablelayerfolderpath: ::windows_sys::core::PCWSTR, layerdata: ::windows_sys::core::PCWSTR, options: ::windows_sys::core::PCWSTR) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `\"Win32_System_HostComputeSystem\"`*"]
    pub fn HcsInitializeWritableLayer(writablelayerpath: ::windows_sys::core::PCWSTR, layerdata: ::windows_sys::core::PCWSTR, options: ::windows_sys::core::PCWSTR) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `\"Win32_System_HostComputeSystem\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn HcsModifyComputeSystem(computesystem: HCS_SYSTEM, operation: HCS_OPERATION, configuration: ::windows_sys::core::PCWSTR, identity: super::super::Foundation::HANDLE) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `\"Win32_System_HostComputeSystem\"`*"]
    pub fn HcsModifyProcess(process: HCS_PROCESS, operation: HCS_OPERATION, settings: ::windows_sys::core::PCWSTR) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `\"Win32_System_HostComputeSystem\"`*"]
    pub fn HcsModifyServiceSettings(settings: ::windows_sys::core::PCWSTR, result: *mut ::windows_sys::core::PWSTR) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `\"Win32_System_HostComputeSystem\"`*"]
    pub fn HcsOpenComputeSystem(id: ::windows_sys::core::PCWSTR, requestedaccess: u32, computesystem: *mut HCS_SYSTEM) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `\"Win32_System_HostComputeSystem\"`*"]
    pub fn HcsOpenComputeSystemInNamespace(idnamespace: ::windows_sys::core::PCWSTR, id: ::windows_sys::core::PCWSTR, requestedaccess: u32, computesystem: *mut HCS_SYSTEM) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `\"Win32_System_HostComputeSystem\"`*"]
    pub fn HcsOpenProcess(computesystem: HCS_SYSTEM, processid: u32, requestedaccess: u32, process: *mut HCS_PROCESS) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `\"Win32_System_HostComputeSystem\"`*"]
    pub fn HcsPauseComputeSystem(computesystem: HCS_SYSTEM, operation: HCS_OPERATION, options: ::windows_sys::core::PCWSTR) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `\"Win32_System_HostComputeSystem\"`*"]
    pub fn HcsResumeComputeSystem(computesystem: HCS_SYSTEM, operation: HCS_OPERATION, options: ::windows_sys::core::PCWSTR) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `\"Win32_System_HostComputeSystem\"`*"]
    pub fn HcsRevokeVmAccess(vmid: ::windows_sys::core::PCWSTR, filepath: ::windows_sys::core::PCWSTR) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `\"Win32_System_HostComputeSystem\"`*"]
    pub fn HcsRevokeVmGroupAccess(filepath: ::windows_sys::core::PCWSTR) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `\"Win32_System_HostComputeSystem\"`*"]
    pub fn HcsSaveComputeSystem(computesystem: HCS_SYSTEM, operation: HCS_OPERATION, options: ::windows_sys::core::PCWSTR) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `\"Win32_System_HostComputeSystem\"`*"]
    pub fn HcsSetComputeSystemCallback(computesystem: HCS_SYSTEM, callbackoptions: HCS_EVENT_OPTIONS, context: *const ::core::ffi::c_void, callback: HCS_EVENT_CALLBACK) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `\"Win32_System_HostComputeSystem\"`*"]
    pub fn HcsSetOperationCallback(operation: HCS_OPERATION, context: *const ::core::ffi::c_void, callback: HCS_OPERATION_COMPLETION) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `\"Win32_System_HostComputeSystem\"`*"]
    pub fn HcsSetOperationContext(operation: HCS_OPERATION, context: *const ::core::ffi::c_void) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `\"Win32_System_HostComputeSystem\"`*"]
    pub fn HcsSetProcessCallback(process: HCS_PROCESS, callbackoptions: HCS_EVENT_OPTIONS, context: *const ::core::ffi::c_void, callback: HCS_EVENT_CALLBACK) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `\"Win32_System_HostComputeSystem\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn HcsSetupBaseOSLayer(layerpath: ::windows_sys::core::PCWSTR, vhdhandle: super::super::Foundation::HANDLE, options: ::windows_sys::core::PCWSTR) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `\"Win32_System_HostComputeSystem\"`*"]
    pub fn HcsSetupBaseOSVolume(layerpath: ::windows_sys::core::PCWSTR, volumepath: ::windows_sys::core::PCWSTR, options: ::windows_sys::core::PCWSTR) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `\"Win32_System_HostComputeSystem\"`*"]
    pub fn HcsShutDownComputeSystem(computesystem: HCS_SYSTEM, operation: HCS_OPERATION, options: ::windows_sys::core::PCWSTR) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `\"Win32_System_HostComputeSystem\"`*"]
    pub fn HcsSignalProcess(process: HCS_PROCESS, operation: HCS_OPERATION, options: ::windows_sys::core::PCWSTR) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `\"Win32_System_HostComputeSystem\"`*"]
    pub fn HcsStartComputeSystem(computesystem: HCS_SYSTEM, operation: HCS_OPERATION, options: ::windows_sys::core::PCWSTR) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `\"Win32_System_HostComputeSystem\"`*"]
    pub fn HcsSubmitWerReport(settings: ::windows_sys::core::PCWSTR) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `\"Win32_System_HostComputeSystem\"`*"]
    pub fn HcsTerminateComputeSystem(computesystem: HCS_SYSTEM, operation: HCS_OPERATION, options: ::windows_sys::core::PCWSTR) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `\"Win32_System_HostComputeSystem\"`*"]
    pub fn HcsTerminateProcess(process: HCS_PROCESS, operation: HCS_OPERATION, options: ::windows_sys::core::PCWSTR) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `\"Win32_System_HostComputeSystem\"`*"]
    pub fn HcsWaitForComputeSystemExit(computesystem: HCS_SYSTEM, timeoutms: u32, result: *mut ::windows_sys::core::PWSTR) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `\"Win32_System_HostComputeSystem\"`*"]
    pub fn HcsWaitForOperationResult(operation: HCS_OPERATION, timeoutms: u32, resultdocument: *mut ::windows_sys::core::PWSTR) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `\"Win32_System_HostComputeSystem\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn HcsWaitForOperationResultAndProcessInfo(operation: HCS_OPERATION, timeoutms: u32, processinformation: *mut HCS_PROCESS_INFORMATION, resultdocument: *mut ::windows_sys::core::PWSTR) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `\"Win32_System_HostComputeSystem\"`*"]
    pub fn HcsWaitForProcessExit(computesystem: HCS_PROCESS, timeoutms: u32, result: *mut ::windows_sys::core::PWSTR) -> ::windows_sys::core::HRESULT;
}
#[doc = "*Required features: `\"Win32_System_HostComputeSystem\"`*"]
pub type HCS_CREATE_OPTIONS = i32;
#[doc = "*Required features: `\"Win32_System_HostComputeSystem\"`*"]
pub const HcsCreateOptions_1: HCS_CREATE_OPTIONS = 65536i32;
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
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_HostComputeSystem\"`*"]
pub struct HCS_EVENT {
    pub Type: HCS_EVENT_TYPE,
    pub EventData: ::windows_sys::core::PCWSTR,
    pub Operation: HCS_OPERATION,
}
impl ::core::marker::Copy for HCS_EVENT {}
impl ::core::clone::Clone for HCS_EVENT {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: `\"Win32_System_HostComputeSystem\"`*"]
pub type HCS_EVENT_CALLBACK = ::core::option::Option<unsafe extern "system" fn(event: *const HCS_EVENT, context: *const ::core::ffi::c_void)>;
#[doc = "*Required features: `\"Win32_System_HostComputeSystem\"`*"]
pub type HCS_EVENT_OPTIONS = u32;
#[doc = "*Required features: `\"Win32_System_HostComputeSystem\"`*"]
pub const HcsEventOptionNone: HCS_EVENT_OPTIONS = 0u32;
#[doc = "*Required features: `\"Win32_System_HostComputeSystem\"`*"]
pub const HcsEventOptionEnableOperationCallbacks: HCS_EVENT_OPTIONS = 1u32;
#[doc = "*Required features: `\"Win32_System_HostComputeSystem\"`*"]
pub type HCS_EVENT_TYPE = i32;
#[doc = "*Required features: `\"Win32_System_HostComputeSystem\"`*"]
pub const HcsEventInvalid: HCS_EVENT_TYPE = 0i32;
#[doc = "*Required features: `\"Win32_System_HostComputeSystem\"`*"]
pub const HcsEventSystemExited: HCS_EVENT_TYPE = 1i32;
#[doc = "*Required features: `\"Win32_System_HostComputeSystem\"`*"]
pub const HcsEventSystemCrashInitiated: HCS_EVENT_TYPE = 2i32;
#[doc = "*Required features: `\"Win32_System_HostComputeSystem\"`*"]
pub const HcsEventSystemCrashReport: HCS_EVENT_TYPE = 3i32;
#[doc = "*Required features: `\"Win32_System_HostComputeSystem\"`*"]
pub const HcsEventSystemRdpEnhancedModeStateChanged: HCS_EVENT_TYPE = 4i32;
#[doc = "*Required features: `\"Win32_System_HostComputeSystem\"`*"]
pub const HcsEventSystemSiloJobCreated: HCS_EVENT_TYPE = 5i32;
#[doc = "*Required features: `\"Win32_System_HostComputeSystem\"`*"]
pub const HcsEventSystemGuestConnectionClosed: HCS_EVENT_TYPE = 6i32;
#[doc = "*Required features: `\"Win32_System_HostComputeSystem\"`*"]
pub const HcsEventProcessExited: HCS_EVENT_TYPE = 65536i32;
#[doc = "*Required features: `\"Win32_System_HostComputeSystem\"`*"]
pub const HcsEventOperationCallback: HCS_EVENT_TYPE = 16777216i32;
#[doc = "*Required features: `\"Win32_System_HostComputeSystem\"`*"]
pub const HcsEventServiceDisconnect: HCS_EVENT_TYPE = 33554432i32;
#[doc = "*Required features: `\"Win32_System_HostComputeSystem\"`*"]
pub type HCS_NOTIFICATIONS = i32;
#[doc = "*Required features: `\"Win32_System_HostComputeSystem\"`*"]
pub const HcsNotificationInvalid: HCS_NOTIFICATIONS = 0i32;
#[doc = "*Required features: `\"Win32_System_HostComputeSystem\"`*"]
pub const HcsNotificationSystemExited: HCS_NOTIFICATIONS = 1i32;
#[doc = "*Required features: `\"Win32_System_HostComputeSystem\"`*"]
pub const HcsNotificationSystemCreateCompleted: HCS_NOTIFICATIONS = 2i32;
#[doc = "*Required features: `\"Win32_System_HostComputeSystem\"`*"]
pub const HcsNotificationSystemStartCompleted: HCS_NOTIFICATIONS = 3i32;
#[doc = "*Required features: `\"Win32_System_HostComputeSystem\"`*"]
pub const HcsNotificationSystemPauseCompleted: HCS_NOTIFICATIONS = 4i32;
#[doc = "*Required features: `\"Win32_System_HostComputeSystem\"`*"]
pub const HcsNotificationSystemResumeCompleted: HCS_NOTIFICATIONS = 5i32;
#[doc = "*Required features: `\"Win32_System_HostComputeSystem\"`*"]
pub const HcsNotificationSystemCrashReport: HCS_NOTIFICATIONS = 6i32;
#[doc = "*Required features: `\"Win32_System_HostComputeSystem\"`*"]
pub const HcsNotificationSystemSiloJobCreated: HCS_NOTIFICATIONS = 7i32;
#[doc = "*Required features: `\"Win32_System_HostComputeSystem\"`*"]
pub const HcsNotificationSystemSaveCompleted: HCS_NOTIFICATIONS = 8i32;
#[doc = "*Required features: `\"Win32_System_HostComputeSystem\"`*"]
pub const HcsNotificationSystemRdpEnhancedModeStateChanged: HCS_NOTIFICATIONS = 9i32;
#[doc = "*Required features: `\"Win32_System_HostComputeSystem\"`*"]
pub const HcsNotificationSystemShutdownFailed: HCS_NOTIFICATIONS = 10i32;
#[doc = "*Required features: `\"Win32_System_HostComputeSystem\"`*"]
pub const HcsNotificationSystemShutdownCompleted: HCS_NOTIFICATIONS = 10i32;
#[doc = "*Required features: `\"Win32_System_HostComputeSystem\"`*"]
pub const HcsNotificationSystemGetPropertiesCompleted: HCS_NOTIFICATIONS = 11i32;
#[doc = "*Required features: `\"Win32_System_HostComputeSystem\"`*"]
pub const HcsNotificationSystemModifyCompleted: HCS_NOTIFICATIONS = 12i32;
#[doc = "*Required features: `\"Win32_System_HostComputeSystem\"`*"]
pub const HcsNotificationSystemCrashInitiated: HCS_NOTIFICATIONS = 13i32;
#[doc = "*Required features: `\"Win32_System_HostComputeSystem\"`*"]
pub const HcsNotificationSystemGuestConnectionClosed: HCS_NOTIFICATIONS = 14i32;
#[doc = "*Required features: `\"Win32_System_HostComputeSystem\"`*"]
pub const HcsNotificationSystemOperationCompletion: HCS_NOTIFICATIONS = 15i32;
#[doc = "*Required features: `\"Win32_System_HostComputeSystem\"`*"]
pub const HcsNotificationSystemPassThru: HCS_NOTIFICATIONS = 16i32;
#[doc = "*Required features: `\"Win32_System_HostComputeSystem\"`*"]
pub const HcsNotificationProcessExited: HCS_NOTIFICATIONS = 65536i32;
#[doc = "*Required features: `\"Win32_System_HostComputeSystem\"`*"]
pub const HcsNotificationServiceDisconnect: HCS_NOTIFICATIONS = 16777216i32;
#[doc = "*Required features: `\"Win32_System_HostComputeSystem\"`*"]
pub const HcsNotificationFlagsReserved: HCS_NOTIFICATIONS = -268435456i32;
#[doc = "*Required features: `\"Win32_System_HostComputeSystem\"`*"]
pub type HCS_NOTIFICATION_CALLBACK = ::core::option::Option<unsafe extern "system" fn(notificationtype: u32, context: *const ::core::ffi::c_void, notificationstatus: ::windows_sys::core::HRESULT, notificationdata: ::windows_sys::core::PCWSTR)>;
#[doc = "*Required features: `\"Win32_System_HostComputeSystem\"`*"]
pub type HCS_NOTIFICATION_FLAGS = i32;
#[doc = "*Required features: `\"Win32_System_HostComputeSystem\"`*"]
pub const HcsNotificationFlagSuccess: HCS_NOTIFICATION_FLAGS = 0i32;
#[doc = "*Required features: `\"Win32_System_HostComputeSystem\"`*"]
pub const HcsNotificationFlagFailure: HCS_NOTIFICATION_FLAGS = -2147483648i32;
pub type HCS_OPERATION = isize;
#[doc = "*Required features: `\"Win32_System_HostComputeSystem\"`*"]
pub type HCS_OPERATION_COMPLETION = ::core::option::Option<unsafe extern "system" fn(operation: HCS_OPERATION, context: *const ::core::ffi::c_void)>;
#[doc = "*Required features: `\"Win32_System_HostComputeSystem\"`*"]
pub type HCS_OPERATION_TYPE = i32;
#[doc = "*Required features: `\"Win32_System_HostComputeSystem\"`*"]
pub const HcsOperationTypeNone: HCS_OPERATION_TYPE = -1i32;
#[doc = "*Required features: `\"Win32_System_HostComputeSystem\"`*"]
pub const HcsOperationTypeEnumerate: HCS_OPERATION_TYPE = 0i32;
#[doc = "*Required features: `\"Win32_System_HostComputeSystem\"`*"]
pub const HcsOperationTypeCreate: HCS_OPERATION_TYPE = 1i32;
#[doc = "*Required features: `\"Win32_System_HostComputeSystem\"`*"]
pub const HcsOperationTypeStart: HCS_OPERATION_TYPE = 2i32;
#[doc = "*Required features: `\"Win32_System_HostComputeSystem\"`*"]
pub const HcsOperationTypeShutdown: HCS_OPERATION_TYPE = 3i32;
#[doc = "*Required features: `\"Win32_System_HostComputeSystem\"`*"]
pub const HcsOperationTypePause: HCS_OPERATION_TYPE = 4i32;
#[doc = "*Required features: `\"Win32_System_HostComputeSystem\"`*"]
pub const HcsOperationTypeResume: HCS_OPERATION_TYPE = 5i32;
#[doc = "*Required features: `\"Win32_System_HostComputeSystem\"`*"]
pub const HcsOperationTypeSave: HCS_OPERATION_TYPE = 6i32;
#[doc = "*Required features: `\"Win32_System_HostComputeSystem\"`*"]
pub const HcsOperationTypeTerminate: HCS_OPERATION_TYPE = 7i32;
#[doc = "*Required features: `\"Win32_System_HostComputeSystem\"`*"]
pub const HcsOperationTypeModify: HCS_OPERATION_TYPE = 8i32;
#[doc = "*Required features: `\"Win32_System_HostComputeSystem\"`*"]
pub const HcsOperationTypeGetProperties: HCS_OPERATION_TYPE = 9i32;
#[doc = "*Required features: `\"Win32_System_HostComputeSystem\"`*"]
pub const HcsOperationTypeCreateProcess: HCS_OPERATION_TYPE = 10i32;
#[doc = "*Required features: `\"Win32_System_HostComputeSystem\"`*"]
pub const HcsOperationTypeSignalProcess: HCS_OPERATION_TYPE = 11i32;
#[doc = "*Required features: `\"Win32_System_HostComputeSystem\"`*"]
pub const HcsOperationTypeGetProcessInfo: HCS_OPERATION_TYPE = 12i32;
#[doc = "*Required features: `\"Win32_System_HostComputeSystem\"`*"]
pub const HcsOperationTypeGetProcessProperties: HCS_OPERATION_TYPE = 13i32;
#[doc = "*Required features: `\"Win32_System_HostComputeSystem\"`*"]
pub const HcsOperationTypeModifyProcess: HCS_OPERATION_TYPE = 14i32;
#[doc = "*Required features: `\"Win32_System_HostComputeSystem\"`*"]
pub const HcsOperationTypeCrash: HCS_OPERATION_TYPE = 15i32;
pub type HCS_PROCESS = isize;
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
pub type HCS_SYSTEM = isize;

#![allow(non_snake_case, non_camel_case_types)]
#[link(name = "windows")]
extern "system" {
    #[doc = "*Required features: `Win32_System_HostComputeSystem`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn HcsAttachLayerStorageFilter(layerpath: super::super::Foundation::PWSTR, layerdata: super::super::Foundation::PWSTR) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `Win32_System_HostComputeSystem`*"]
    pub fn HcsCancelOperation(operation: HCS_OPERATION) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `Win32_System_HostComputeSystem`*"]
    pub fn HcsCloseComputeSystem(computesystem: HCS_SYSTEM);
    #[doc = "*Required features: `Win32_System_HostComputeSystem`*"]
    pub fn HcsCloseOperation(operation: HCS_OPERATION);
    #[doc = "*Required features: `Win32_System_HostComputeSystem`*"]
    pub fn HcsCloseProcess(process: HCS_PROCESS);
    #[doc = "*Required features: `Win32_System_HostComputeSystem`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn HcsCrashComputeSystem(computesystem: HCS_SYSTEM, operation: HCS_OPERATION, options: super::super::Foundation::PWSTR) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `Win32_System_HostComputeSystem`, `Win32_Foundation`, `Win32_Security`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
    pub fn HcsCreateComputeSystem(id: super::super::Foundation::PWSTR, configuration: super::super::Foundation::PWSTR, operation: HCS_OPERATION, securitydescriptor: *const super::super::Security::SECURITY_DESCRIPTOR, computesystem: *mut HCS_SYSTEM) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `Win32_System_HostComputeSystem`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn HcsCreateComputeSystemInNamespace(idnamespace: super::super::Foundation::PWSTR, id: super::super::Foundation::PWSTR, configuration: super::super::Foundation::PWSTR, operation: HCS_OPERATION, options: *const HCS_CREATE_OPTIONS, computesystem: *mut HCS_SYSTEM) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `Win32_System_HostComputeSystem`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn HcsCreateEmptyGuestStateFile(gueststatefilepath: super::super::Foundation::PWSTR) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `Win32_System_HostComputeSystem`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn HcsCreateEmptyRuntimeStateFile(runtimestatefilepath: super::super::Foundation::PWSTR) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `Win32_System_HostComputeSystem`*"]
    pub fn HcsCreateOperation(context: *const ::core::ffi::c_void, callback: HCS_OPERATION_COMPLETION) -> HCS_OPERATION;
    #[doc = "*Required features: `Win32_System_HostComputeSystem`, `Win32_Foundation`, `Win32_Security`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
    pub fn HcsCreateProcess(computesystem: HCS_SYSTEM, processparameters: super::super::Foundation::PWSTR, operation: HCS_OPERATION, securitydescriptor: *const super::super::Security::SECURITY_DESCRIPTOR, process: *mut HCS_PROCESS) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `Win32_System_HostComputeSystem`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn HcsDestroyLayer(layerpath: super::super::Foundation::PWSTR) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `Win32_System_HostComputeSystem`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn HcsDetachLayerStorageFilter(layerpath: super::super::Foundation::PWSTR) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `Win32_System_HostComputeSystem`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn HcsEnumerateComputeSystems(query: super::super::Foundation::PWSTR, operation: HCS_OPERATION) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `Win32_System_HostComputeSystem`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn HcsEnumerateComputeSystemsInNamespace(idnamespace: super::super::Foundation::PWSTR, query: super::super::Foundation::PWSTR, operation: HCS_OPERATION) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `Win32_System_HostComputeSystem`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn HcsExportLayer(layerpath: super::super::Foundation::PWSTR, exportfolderpath: super::super::Foundation::PWSTR, layerdata: super::super::Foundation::PWSTR, options: super::super::Foundation::PWSTR) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `Win32_System_HostComputeSystem`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn HcsExportLegacyWritableLayer(writablelayermountpath: super::super::Foundation::PWSTR, writablelayerfolderpath: super::super::Foundation::PWSTR, exportfolderpath: super::super::Foundation::PWSTR, layerdata: super::super::Foundation::PWSTR) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `Win32_System_HostComputeSystem`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn HcsFormatWritableLayerVhd(vhdhandle: super::super::Foundation::HANDLE) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `Win32_System_HostComputeSystem`*"]
    pub fn HcsGetComputeSystemFromOperation(operation: HCS_OPERATION) -> HCS_SYSTEM;
    #[doc = "*Required features: `Win32_System_HostComputeSystem`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn HcsGetComputeSystemProperties(computesystem: HCS_SYSTEM, operation: HCS_OPERATION, propertyquery: super::super::Foundation::PWSTR) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `Win32_System_HostComputeSystem`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn HcsGetLayerVhdMountPath(vhdhandle: super::super::Foundation::HANDLE, mountpath: *mut super::super::Foundation::PWSTR) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `Win32_System_HostComputeSystem`*"]
    pub fn HcsGetOperationContext(operation: HCS_OPERATION) -> *mut ::core::ffi::c_void;
    #[doc = "*Required features: `Win32_System_HostComputeSystem`*"]
    pub fn HcsGetOperationId(operation: HCS_OPERATION) -> u64;
    #[doc = "*Required features: `Win32_System_HostComputeSystem`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn HcsGetOperationResult(operation: HCS_OPERATION, resultdocument: *mut super::super::Foundation::PWSTR) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `Win32_System_HostComputeSystem`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn HcsGetOperationResultAndProcessInfo(operation: HCS_OPERATION, processinformation: *mut HCS_PROCESS_INFORMATION, resultdocument: *mut super::super::Foundation::PWSTR) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `Win32_System_HostComputeSystem`*"]
    pub fn HcsGetOperationType(operation: HCS_OPERATION) -> HCS_OPERATION_TYPE;
    #[doc = "*Required features: `Win32_System_HostComputeSystem`*"]
    pub fn HcsGetProcessFromOperation(operation: HCS_OPERATION) -> HCS_PROCESS;
    #[doc = "*Required features: `Win32_System_HostComputeSystem`*"]
    pub fn HcsGetProcessInfo(process: HCS_PROCESS, operation: HCS_OPERATION) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `Win32_System_HostComputeSystem`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn HcsGetProcessProperties(process: HCS_PROCESS, operation: HCS_OPERATION, propertyquery: super::super::Foundation::PWSTR) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `Win32_System_HostComputeSystem`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn HcsGetProcessorCompatibilityFromSavedState(runtimefilename: super::super::Foundation::PWSTR, processorfeaturesstring: *mut super::super::Foundation::PWSTR) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `Win32_System_HostComputeSystem`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn HcsGetServiceProperties(propertyquery: super::super::Foundation::PWSTR, result: *mut super::super::Foundation::PWSTR) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `Win32_System_HostComputeSystem`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn HcsGrantVmAccess(vmid: super::super::Foundation::PWSTR, filepath: super::super::Foundation::PWSTR) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `Win32_System_HostComputeSystem`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn HcsGrantVmGroupAccess(filepath: super::super::Foundation::PWSTR) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `Win32_System_HostComputeSystem`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn HcsImportLayer(layerpath: super::super::Foundation::PWSTR, sourcefolderpath: super::super::Foundation::PWSTR, layerdata: super::super::Foundation::PWSTR) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `Win32_System_HostComputeSystem`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn HcsInitializeLegacyWritableLayer(writablelayermountpath: super::super::Foundation::PWSTR, writablelayerfolderpath: super::super::Foundation::PWSTR, layerdata: super::super::Foundation::PWSTR, options: super::super::Foundation::PWSTR) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `Win32_System_HostComputeSystem`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn HcsInitializeWritableLayer(writablelayerpath: super::super::Foundation::PWSTR, layerdata: super::super::Foundation::PWSTR, options: super::super::Foundation::PWSTR) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `Win32_System_HostComputeSystem`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn HcsModifyComputeSystem(computesystem: HCS_SYSTEM, operation: HCS_OPERATION, configuration: super::super::Foundation::PWSTR, identity: super::super::Foundation::HANDLE) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `Win32_System_HostComputeSystem`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn HcsModifyProcess(process: HCS_PROCESS, operation: HCS_OPERATION, settings: super::super::Foundation::PWSTR) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `Win32_System_HostComputeSystem`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn HcsModifyServiceSettings(settings: super::super::Foundation::PWSTR, result: *mut super::super::Foundation::PWSTR) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `Win32_System_HostComputeSystem`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn HcsOpenComputeSystem(id: super::super::Foundation::PWSTR, requestedaccess: u32, computesystem: *mut HCS_SYSTEM) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `Win32_System_HostComputeSystem`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn HcsOpenComputeSystemInNamespace(idnamespace: super::super::Foundation::PWSTR, id: super::super::Foundation::PWSTR, requestedaccess: u32, computesystem: *mut HCS_SYSTEM) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `Win32_System_HostComputeSystem`*"]
    pub fn HcsOpenProcess(computesystem: HCS_SYSTEM, processid: u32, requestedaccess: u32, process: *mut HCS_PROCESS) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `Win32_System_HostComputeSystem`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn HcsPauseComputeSystem(computesystem: HCS_SYSTEM, operation: HCS_OPERATION, options: super::super::Foundation::PWSTR) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `Win32_System_HostComputeSystem`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn HcsResumeComputeSystem(computesystem: HCS_SYSTEM, operation: HCS_OPERATION, options: super::super::Foundation::PWSTR) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `Win32_System_HostComputeSystem`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn HcsRevokeVmAccess(vmid: super::super::Foundation::PWSTR, filepath: super::super::Foundation::PWSTR) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `Win32_System_HostComputeSystem`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn HcsRevokeVmGroupAccess(filepath: super::super::Foundation::PWSTR) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `Win32_System_HostComputeSystem`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn HcsSaveComputeSystem(computesystem: HCS_SYSTEM, operation: HCS_OPERATION, options: super::super::Foundation::PWSTR) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `Win32_System_HostComputeSystem`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn HcsSetComputeSystemCallback(computesystem: HCS_SYSTEM, callbackoptions: HCS_EVENT_OPTIONS, context: *const ::core::ffi::c_void, callback: HCS_EVENT_CALLBACK) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `Win32_System_HostComputeSystem`*"]
    pub fn HcsSetOperationCallback(operation: HCS_OPERATION, context: *const ::core::ffi::c_void, callback: HCS_OPERATION_COMPLETION) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `Win32_System_HostComputeSystem`*"]
    pub fn HcsSetOperationContext(operation: HCS_OPERATION, context: *const ::core::ffi::c_void) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `Win32_System_HostComputeSystem`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn HcsSetProcessCallback(process: HCS_PROCESS, callbackoptions: HCS_EVENT_OPTIONS, context: *const ::core::ffi::c_void, callback: HCS_EVENT_CALLBACK) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `Win32_System_HostComputeSystem`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn HcsSetupBaseOSLayer(layerpath: super::super::Foundation::PWSTR, vhdhandle: super::super::Foundation::HANDLE, options: super::super::Foundation::PWSTR) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `Win32_System_HostComputeSystem`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn HcsSetupBaseOSVolume(layerpath: super::super::Foundation::PWSTR, volumepath: super::super::Foundation::PWSTR, options: super::super::Foundation::PWSTR) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `Win32_System_HostComputeSystem`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn HcsShutDownComputeSystem(computesystem: HCS_SYSTEM, operation: HCS_OPERATION, options: super::super::Foundation::PWSTR) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `Win32_System_HostComputeSystem`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn HcsSignalProcess(process: HCS_PROCESS, operation: HCS_OPERATION, options: super::super::Foundation::PWSTR) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `Win32_System_HostComputeSystem`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn HcsStartComputeSystem(computesystem: HCS_SYSTEM, operation: HCS_OPERATION, options: super::super::Foundation::PWSTR) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `Win32_System_HostComputeSystem`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn HcsSubmitWerReport(settings: super::super::Foundation::PWSTR) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `Win32_System_HostComputeSystem`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn HcsTerminateComputeSystem(computesystem: HCS_SYSTEM, operation: HCS_OPERATION, options: super::super::Foundation::PWSTR) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `Win32_System_HostComputeSystem`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn HcsTerminateProcess(process: HCS_PROCESS, operation: HCS_OPERATION, options: super::super::Foundation::PWSTR) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `Win32_System_HostComputeSystem`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn HcsWaitForComputeSystemExit(computesystem: HCS_SYSTEM, timeoutms: u32, result: *mut super::super::Foundation::PWSTR) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `Win32_System_HostComputeSystem`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn HcsWaitForOperationResult(operation: HCS_OPERATION, timeoutms: u32, resultdocument: *mut super::super::Foundation::PWSTR) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `Win32_System_HostComputeSystem`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn HcsWaitForOperationResultAndProcessInfo(operation: HCS_OPERATION, timeoutms: u32, processinformation: *mut HCS_PROCESS_INFORMATION, resultdocument: *mut super::super::Foundation::PWSTR) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `Win32_System_HostComputeSystem`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn HcsWaitForProcessExit(computesystem: HCS_PROCESS, timeoutms: u32, result: *mut super::super::Foundation::PWSTR) -> ::windows_sys::core::HRESULT;
}
pub struct HCS_CREATE_OPTIONS(i32);
pub struct HCS_CREATE_OPTIONS_1(i32);
pub struct HCS_EVENT(i32);
pub struct HCS_EVENT_CALLBACK(i32);
pub struct HCS_EVENT_OPTIONS(i32);
pub struct HCS_EVENT_TYPE(i32);
pub struct HCS_NOTIFICATIONS(i32);
pub struct HCS_NOTIFICATION_CALLBACK(i32);
pub struct HCS_NOTIFICATION_FLAGS(i32);
pub struct HCS_OPERATION(i32);
pub struct HCS_OPERATION_COMPLETION(i32);
pub struct HCS_OPERATION_TYPE(i32);
pub struct HCS_PROCESS(i32);
pub struct HCS_PROCESS_INFORMATION(i32);
pub struct HCS_SYSTEM(i32);

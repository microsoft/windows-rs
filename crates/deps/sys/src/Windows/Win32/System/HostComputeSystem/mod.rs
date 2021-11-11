#![allow(non_snake_case, non_camel_case_types)]
#[link(name = "windows")]
extern "system" {
    #[doc = "*Required features: `Win32_System_HostComputeSystem`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn HcsAttachLayerStorageFilter();
    #[doc = "*Required features: `Win32_System_HostComputeSystem`*"]
    pub fn HcsCancelOperation();
    #[doc = "*Required features: `Win32_System_HostComputeSystem`*"]
    pub fn HcsCloseComputeSystem();
    #[doc = "*Required features: `Win32_System_HostComputeSystem`*"]
    pub fn HcsCloseOperation();
    #[doc = "*Required features: `Win32_System_HostComputeSystem`*"]
    pub fn HcsCloseProcess();
    #[doc = "*Required features: `Win32_System_HostComputeSystem`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn HcsCrashComputeSystem();
    #[doc = "*Required features: `Win32_System_HostComputeSystem`, `Win32_Foundation`, `Win32_Security`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
    pub fn HcsCreateComputeSystem();
    #[doc = "*Required features: `Win32_System_HostComputeSystem`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn HcsCreateComputeSystemInNamespace();
    #[doc = "*Required features: `Win32_System_HostComputeSystem`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn HcsCreateEmptyGuestStateFile();
    #[doc = "*Required features: `Win32_System_HostComputeSystem`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn HcsCreateEmptyRuntimeStateFile();
    #[doc = "*Required features: `Win32_System_HostComputeSystem`*"]
    pub fn HcsCreateOperation();
    #[doc = "*Required features: `Win32_System_HostComputeSystem`, `Win32_Foundation`, `Win32_Security`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
    pub fn HcsCreateProcess();
    #[doc = "*Required features: `Win32_System_HostComputeSystem`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn HcsDestroyLayer();
    #[doc = "*Required features: `Win32_System_HostComputeSystem`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn HcsDetachLayerStorageFilter();
    #[doc = "*Required features: `Win32_System_HostComputeSystem`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn HcsEnumerateComputeSystems();
    #[doc = "*Required features: `Win32_System_HostComputeSystem`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn HcsEnumerateComputeSystemsInNamespace();
    #[doc = "*Required features: `Win32_System_HostComputeSystem`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn HcsExportLayer();
    #[doc = "*Required features: `Win32_System_HostComputeSystem`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn HcsExportLegacyWritableLayer();
    #[doc = "*Required features: `Win32_System_HostComputeSystem`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn HcsFormatWritableLayerVhd();
    #[doc = "*Required features: `Win32_System_HostComputeSystem`*"]
    pub fn HcsGetComputeSystemFromOperation();
    #[doc = "*Required features: `Win32_System_HostComputeSystem`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn HcsGetComputeSystemProperties();
    #[doc = "*Required features: `Win32_System_HostComputeSystem`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn HcsGetLayerVhdMountPath();
    #[doc = "*Required features: `Win32_System_HostComputeSystem`*"]
    pub fn HcsGetOperationContext();
    #[doc = "*Required features: `Win32_System_HostComputeSystem`*"]
    pub fn HcsGetOperationId();
    #[doc = "*Required features: `Win32_System_HostComputeSystem`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn HcsGetOperationResult();
    #[doc = "*Required features: `Win32_System_HostComputeSystem`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn HcsGetOperationResultAndProcessInfo();
    #[doc = "*Required features: `Win32_System_HostComputeSystem`*"]
    pub fn HcsGetOperationType();
    #[doc = "*Required features: `Win32_System_HostComputeSystem`*"]
    pub fn HcsGetProcessFromOperation();
    #[doc = "*Required features: `Win32_System_HostComputeSystem`*"]
    pub fn HcsGetProcessInfo();
    #[doc = "*Required features: `Win32_System_HostComputeSystem`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn HcsGetProcessProperties();
    #[doc = "*Required features: `Win32_System_HostComputeSystem`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn HcsGetProcessorCompatibilityFromSavedState();
    #[doc = "*Required features: `Win32_System_HostComputeSystem`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn HcsGetServiceProperties();
    #[doc = "*Required features: `Win32_System_HostComputeSystem`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn HcsGrantVmAccess();
    #[doc = "*Required features: `Win32_System_HostComputeSystem`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn HcsGrantVmGroupAccess();
    #[doc = "*Required features: `Win32_System_HostComputeSystem`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn HcsImportLayer();
    #[doc = "*Required features: `Win32_System_HostComputeSystem`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn HcsInitializeLegacyWritableLayer();
    #[doc = "*Required features: `Win32_System_HostComputeSystem`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn HcsInitializeWritableLayer();
    #[doc = "*Required features: `Win32_System_HostComputeSystem`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn HcsModifyComputeSystem();
    #[doc = "*Required features: `Win32_System_HostComputeSystem`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn HcsModifyProcess();
    #[doc = "*Required features: `Win32_System_HostComputeSystem`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn HcsModifyServiceSettings();
    #[doc = "*Required features: `Win32_System_HostComputeSystem`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn HcsOpenComputeSystem();
    #[doc = "*Required features: `Win32_System_HostComputeSystem`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn HcsOpenComputeSystemInNamespace();
    #[doc = "*Required features: `Win32_System_HostComputeSystem`*"]
    pub fn HcsOpenProcess();
    #[doc = "*Required features: `Win32_System_HostComputeSystem`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn HcsPauseComputeSystem();
    #[doc = "*Required features: `Win32_System_HostComputeSystem`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn HcsResumeComputeSystem();
    #[doc = "*Required features: `Win32_System_HostComputeSystem`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn HcsRevokeVmAccess();
    #[doc = "*Required features: `Win32_System_HostComputeSystem`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn HcsRevokeVmGroupAccess();
    #[doc = "*Required features: `Win32_System_HostComputeSystem`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn HcsSaveComputeSystem();
    #[doc = "*Required features: `Win32_System_HostComputeSystem`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn HcsSetComputeSystemCallback();
    #[doc = "*Required features: `Win32_System_HostComputeSystem`*"]
    pub fn HcsSetOperationCallback();
    #[doc = "*Required features: `Win32_System_HostComputeSystem`*"]
    pub fn HcsSetOperationContext();
    #[doc = "*Required features: `Win32_System_HostComputeSystem`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn HcsSetProcessCallback();
    #[doc = "*Required features: `Win32_System_HostComputeSystem`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn HcsSetupBaseOSLayer();
    #[doc = "*Required features: `Win32_System_HostComputeSystem`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn HcsSetupBaseOSVolume();
    #[doc = "*Required features: `Win32_System_HostComputeSystem`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn HcsShutDownComputeSystem();
    #[doc = "*Required features: `Win32_System_HostComputeSystem`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn HcsSignalProcess();
    #[doc = "*Required features: `Win32_System_HostComputeSystem`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn HcsStartComputeSystem();
    #[doc = "*Required features: `Win32_System_HostComputeSystem`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn HcsSubmitWerReport();
    #[doc = "*Required features: `Win32_System_HostComputeSystem`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn HcsTerminateComputeSystem();
    #[doc = "*Required features: `Win32_System_HostComputeSystem`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn HcsTerminateProcess();
    #[doc = "*Required features: `Win32_System_HostComputeSystem`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn HcsWaitForComputeSystemExit();
    #[doc = "*Required features: `Win32_System_HostComputeSystem`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn HcsWaitForOperationResult();
    #[doc = "*Required features: `Win32_System_HostComputeSystem`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn HcsWaitForOperationResultAndProcessInfo();
    #[doc = "*Required features: `Win32_System_HostComputeSystem`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn HcsWaitForProcessExit();
}

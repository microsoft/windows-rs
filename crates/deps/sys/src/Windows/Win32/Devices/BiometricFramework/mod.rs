#![allow(non_snake_case, non_camel_case_types)]
#[link(name = "windows")]
extern "system" {
    #[doc = "*Required features: `Win32_Devices_BiometricFramework`*"]
    pub fn WinBioAcquireFocus();
    #[doc = "*Required features: `Win32_Devices_BiometricFramework`*"]
    pub fn WinBioAsyncEnumBiometricUnits();
    #[doc = "*Required features: `Win32_Devices_BiometricFramework`*"]
    pub fn WinBioAsyncEnumDatabases();
    #[doc = "*Required features: `Win32_Devices_BiometricFramework`*"]
    pub fn WinBioAsyncEnumServiceProviders();
    #[doc = "*Required features: `Win32_Devices_BiometricFramework`*"]
    pub fn WinBioAsyncMonitorFrameworkChanges();
    #[doc = "*Required features: `Win32_Devices_BiometricFramework`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WinBioAsyncOpenFramework();
    #[doc = "*Required features: `Win32_Devices_BiometricFramework`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WinBioAsyncOpenSession();
    #[doc = "*Required features: `Win32_Devices_BiometricFramework`*"]
    pub fn WinBioCancel();
    #[doc = "*Required features: `Win32_Devices_BiometricFramework`*"]
    pub fn WinBioCaptureSample();
    #[doc = "*Required features: `Win32_Devices_BiometricFramework`*"]
    pub fn WinBioCaptureSampleWithCallback();
    #[doc = "*Required features: `Win32_Devices_BiometricFramework`*"]
    pub fn WinBioCloseFramework();
    #[doc = "*Required features: `Win32_Devices_BiometricFramework`*"]
    pub fn WinBioCloseSession();
    #[doc = "*Required features: `Win32_Devices_BiometricFramework`*"]
    pub fn WinBioControlUnit();
    #[doc = "*Required features: `Win32_Devices_BiometricFramework`*"]
    pub fn WinBioControlUnitPrivileged();
    #[doc = "*Required features: `Win32_Devices_BiometricFramework`*"]
    pub fn WinBioDeleteTemplate();
    #[doc = "*Required features: `Win32_Devices_BiometricFramework`*"]
    pub fn WinBioEnrollBegin();
    #[doc = "*Required features: `Win32_Devices_BiometricFramework`*"]
    pub fn WinBioEnrollCapture();
    #[doc = "*Required features: `Win32_Devices_BiometricFramework`*"]
    pub fn WinBioEnrollCaptureWithCallback();
    #[doc = "*Required features: `Win32_Devices_BiometricFramework`*"]
    pub fn WinBioEnrollCommit();
    #[doc = "*Required features: `Win32_Devices_BiometricFramework`*"]
    pub fn WinBioEnrollDiscard();
    #[doc = "*Required features: `Win32_Devices_BiometricFramework`*"]
    pub fn WinBioEnrollSelect();
    #[doc = "*Required features: `Win32_Devices_BiometricFramework`*"]
    pub fn WinBioEnumBiometricUnits();
    #[doc = "*Required features: `Win32_Devices_BiometricFramework`*"]
    pub fn WinBioEnumDatabases();
    #[doc = "*Required features: `Win32_Devices_BiometricFramework`*"]
    pub fn WinBioEnumEnrollments();
    #[doc = "*Required features: `Win32_Devices_BiometricFramework`*"]
    pub fn WinBioEnumServiceProviders();
    #[doc = "*Required features: `Win32_Devices_BiometricFramework`*"]
    pub fn WinBioFree();
    #[doc = "*Required features: `Win32_Devices_BiometricFramework`*"]
    pub fn WinBioGetCredentialState();
    #[doc = "*Required features: `Win32_Devices_BiometricFramework`*"]
    pub fn WinBioGetDomainLogonSetting();
    #[doc = "*Required features: `Win32_Devices_BiometricFramework`*"]
    pub fn WinBioGetEnabledSetting();
    #[doc = "*Required features: `Win32_Devices_BiometricFramework`*"]
    pub fn WinBioGetEnrolledFactors();
    #[doc = "*Required features: `Win32_Devices_BiometricFramework`*"]
    pub fn WinBioGetLogonSetting();
    #[doc = "*Required features: `Win32_Devices_BiometricFramework`*"]
    pub fn WinBioGetProperty();
    #[doc = "*Required features: `Win32_Devices_BiometricFramework`*"]
    pub fn WinBioIdentify();
    #[doc = "*Required features: `Win32_Devices_BiometricFramework`*"]
    pub fn WinBioIdentifyWithCallback();
    #[doc = "*Required features: `Win32_Devices_BiometricFramework`*"]
    pub fn WinBioImproveBegin();
    #[doc = "*Required features: `Win32_Devices_BiometricFramework`*"]
    pub fn WinBioImproveEnd();
    #[doc = "*Required features: `Win32_Devices_BiometricFramework`*"]
    pub fn WinBioLocateSensor();
    #[doc = "*Required features: `Win32_Devices_BiometricFramework`*"]
    pub fn WinBioLocateSensorWithCallback();
    #[doc = "*Required features: `Win32_Devices_BiometricFramework`*"]
    pub fn WinBioLockUnit();
    #[doc = "*Required features: `Win32_Devices_BiometricFramework`*"]
    pub fn WinBioLogonIdentifiedUser();
    #[doc = "*Required features: `Win32_Devices_BiometricFramework`*"]
    pub fn WinBioMonitorPresence();
    #[doc = "*Required features: `Win32_Devices_BiometricFramework`*"]
    pub fn WinBioOpenSession();
    #[doc = "*Required features: `Win32_Devices_BiometricFramework`*"]
    pub fn WinBioRegisterEventMonitor();
    #[doc = "*Required features: `Win32_Devices_BiometricFramework`*"]
    pub fn WinBioReleaseFocus();
    #[doc = "*Required features: `Win32_Devices_BiometricFramework`*"]
    pub fn WinBioRemoveAllCredentials();
    #[doc = "*Required features: `Win32_Devices_BiometricFramework`*"]
    pub fn WinBioRemoveAllDomainCredentials();
    #[doc = "*Required features: `Win32_Devices_BiometricFramework`*"]
    pub fn WinBioRemoveCredential();
    #[doc = "*Required features: `Win32_Devices_BiometricFramework`*"]
    pub fn WinBioSetCredential();
    #[doc = "*Required features: `Win32_Devices_BiometricFramework`*"]
    pub fn WinBioSetProperty();
    #[doc = "*Required features: `Win32_Devices_BiometricFramework`*"]
    pub fn WinBioUnlockUnit();
    #[doc = "*Required features: `Win32_Devices_BiometricFramework`*"]
    pub fn WinBioUnregisterEventMonitor();
    #[doc = "*Required features: `Win32_Devices_BiometricFramework`*"]
    pub fn WinBioVerify();
    #[doc = "*Required features: `Win32_Devices_BiometricFramework`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WinBioVerifyWithCallback();
    #[doc = "*Required features: `Win32_Devices_BiometricFramework`*"]
    pub fn WinBioWait();
}

#![allow(non_snake_case, non_camel_case_types)]
#[link(name = "windows")]
extern "system" {
    #[doc = "*Required features: `Win32_System_Power`*"]
    pub fn CallNtPowerInformation();
    #[doc = "*Required features: `Win32_System_Power`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CanUserWritePwrScheme();
    #[doc = "*Required features: `Win32_System_Power`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DeletePwrScheme();
    #[doc = "*Required features: `Win32_System_Power`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DevicePowerClose();
    #[doc = "*Required features: `Win32_System_Power`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DevicePowerEnumDevices();
    #[doc = "*Required features: `Win32_System_Power`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DevicePowerOpen();
    #[doc = "*Required features: `Win32_System_Power`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DevicePowerSetDeviceState();
    #[doc = "*Required features: `Win32_System_Power`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn EnumPwrSchemes();
    #[doc = "*Required features: `Win32_System_Power`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetActivePwrScheme();
    #[doc = "*Required features: `Win32_System_Power`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetCurrentPowerPolicies();
    #[doc = "*Required features: `Win32_System_Power`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetDevicePowerState();
    #[doc = "*Required features: `Win32_System_Power`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetPwrCapabilities();
    #[doc = "*Required features: `Win32_System_Power`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetPwrDiskSpindownRange();
    #[doc = "*Required features: `Win32_System_Power`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetSystemPowerStatus();
    #[doc = "*Required features: `Win32_System_Power`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn IsAdminOverrideActive();
    #[doc = "*Required features: `Win32_System_Power`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn IsPwrHibernateAllowed();
    #[doc = "*Required features: `Win32_System_Power`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn IsPwrShutdownAllowed();
    #[doc = "*Required features: `Win32_System_Power`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn IsPwrSuspendAllowed();
    #[doc = "*Required features: `Win32_System_Power`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn IsSystemResumeAutomatic();
    #[doc = "*Required features: `Win32_System_Power`*"]
    pub fn PowerCanRestoreIndividualDefaultPowerScheme();
    #[doc = "*Required features: `Win32_System_Power`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn PowerClearRequest();
    #[doc = "*Required features: `Win32_System_Power`, `Win32_System_Registry`*"]
    #[cfg(feature = "Win32_System_Registry")]
    pub fn PowerCreatePossibleSetting();
    #[doc = "*Required features: `Win32_System_Power`, `Win32_Foundation`, `Win32_System_Threading`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Threading"))]
    pub fn PowerCreateRequest();
    #[doc = "*Required features: `Win32_System_Power`, `Win32_System_Registry`*"]
    #[cfg(feature = "Win32_System_Registry")]
    pub fn PowerCreateSetting();
    #[doc = "*Required features: `Win32_System_Power`, `Win32_System_Registry`*"]
    #[cfg(feature = "Win32_System_Registry")]
    pub fn PowerDeleteScheme();
    #[doc = "*Required features: `Win32_System_Power`*"]
    pub fn PowerDeterminePlatformRole();
    #[doc = "*Required features: `Win32_System_Power`*"]
    pub fn PowerDeterminePlatformRoleEx();
    #[doc = "*Required features: `Win32_System_Power`, `Win32_System_Registry`*"]
    #[cfg(feature = "Win32_System_Registry")]
    pub fn PowerDuplicateScheme();
    #[doc = "*Required features: `Win32_System_Power`, `Win32_System_Registry`*"]
    #[cfg(feature = "Win32_System_Registry")]
    pub fn PowerEnumerate();
    #[doc = "*Required features: `Win32_System_Power`, `Win32_System_Registry`*"]
    #[cfg(feature = "Win32_System_Registry")]
    pub fn PowerGetActiveScheme();
    #[doc = "*Required features: `Win32_System_Power`, `Win32_Foundation`, `Win32_System_Registry`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Registry"))]
    pub fn PowerImportPowerScheme();
    #[doc = "*Required features: `Win32_System_Power`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn PowerIsSettingRangeDefined();
    #[doc = "*Required features: `Win32_System_Power`, `Win32_Foundation`, `Win32_System_Registry`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Registry"))]
    pub fn PowerOpenSystemPowerKey();
    #[doc = "*Required features: `Win32_System_Power`, `Win32_Foundation`, `Win32_System_Registry`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Registry"))]
    pub fn PowerOpenUserPowerKey();
    #[doc = "*Required features: `Win32_System_Power`, `Win32_System_Registry`*"]
    #[cfg(feature = "Win32_System_Registry")]
    pub fn PowerReadACDefaultIndex();
    #[doc = "*Required features: `Win32_System_Power`, `Win32_System_Registry`*"]
    #[cfg(feature = "Win32_System_Registry")]
    pub fn PowerReadACValue();
    #[doc = "*Required features: `Win32_System_Power`, `Win32_System_Registry`*"]
    #[cfg(feature = "Win32_System_Registry")]
    pub fn PowerReadACValueIndex();
    #[doc = "*Required features: `Win32_System_Power`, `Win32_System_Registry`*"]
    #[cfg(feature = "Win32_System_Registry")]
    pub fn PowerReadDCDefaultIndex();
    #[doc = "*Required features: `Win32_System_Power`, `Win32_System_Registry`*"]
    #[cfg(feature = "Win32_System_Registry")]
    pub fn PowerReadDCValue();
    #[doc = "*Required features: `Win32_System_Power`, `Win32_System_Registry`*"]
    #[cfg(feature = "Win32_System_Registry")]
    pub fn PowerReadDCValueIndex();
    #[doc = "*Required features: `Win32_System_Power`, `Win32_System_Registry`*"]
    #[cfg(feature = "Win32_System_Registry")]
    pub fn PowerReadDescription();
    #[doc = "*Required features: `Win32_System_Power`, `Win32_System_Registry`*"]
    #[cfg(feature = "Win32_System_Registry")]
    pub fn PowerReadFriendlyName();
    #[doc = "*Required features: `Win32_System_Power`, `Win32_System_Registry`*"]
    #[cfg(feature = "Win32_System_Registry")]
    pub fn PowerReadIconResourceSpecifier();
    #[doc = "*Required features: `Win32_System_Power`, `Win32_System_Registry`*"]
    #[cfg(feature = "Win32_System_Registry")]
    pub fn PowerReadPossibleDescription();
    #[doc = "*Required features: `Win32_System_Power`, `Win32_System_Registry`*"]
    #[cfg(feature = "Win32_System_Registry")]
    pub fn PowerReadPossibleFriendlyName();
    #[doc = "*Required features: `Win32_System_Power`, `Win32_System_Registry`*"]
    #[cfg(feature = "Win32_System_Registry")]
    pub fn PowerReadPossibleValue();
    #[doc = "*Required features: `Win32_System_Power`*"]
    pub fn PowerReadSettingAttributes();
    #[doc = "*Required features: `Win32_System_Power`, `Win32_System_Registry`*"]
    #[cfg(feature = "Win32_System_Registry")]
    pub fn PowerReadValueIncrement();
    #[doc = "*Required features: `Win32_System_Power`, `Win32_System_Registry`*"]
    #[cfg(feature = "Win32_System_Registry")]
    pub fn PowerReadValueMax();
    #[doc = "*Required features: `Win32_System_Power`, `Win32_System_Registry`*"]
    #[cfg(feature = "Win32_System_Registry")]
    pub fn PowerReadValueMin();
    #[doc = "*Required features: `Win32_System_Power`, `Win32_System_Registry`*"]
    #[cfg(feature = "Win32_System_Registry")]
    pub fn PowerReadValueUnitsSpecifier();
    #[doc = "*Required features: `Win32_System_Power`*"]
    pub fn PowerRegisterForEffectivePowerModeNotifications();
    #[doc = "*Required features: `Win32_System_Power`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn PowerRegisterSuspendResumeNotification();
    #[doc = "*Required features: `Win32_System_Power`*"]
    pub fn PowerRemovePowerSetting();
    #[doc = "*Required features: `Win32_System_Power`*"]
    pub fn PowerReplaceDefaultPowerSchemes();
    #[doc = "*Required features: `Win32_System_Power`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn PowerReportThermalEvent();
    #[doc = "*Required features: `Win32_System_Power`*"]
    pub fn PowerRestoreDefaultPowerSchemes();
    #[doc = "*Required features: `Win32_System_Power`*"]
    pub fn PowerRestoreIndividualDefaultPowerScheme();
    #[doc = "*Required features: `Win32_System_Power`, `Win32_System_Registry`*"]
    #[cfg(feature = "Win32_System_Registry")]
    pub fn PowerSetActiveScheme();
    #[doc = "*Required features: `Win32_System_Power`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn PowerSetRequest();
    #[doc = "*Required features: `Win32_System_Power`*"]
    pub fn PowerSettingAccessCheck();
    #[doc = "*Required features: `Win32_System_Power`, `Win32_System_Registry`*"]
    #[cfg(feature = "Win32_System_Registry")]
    pub fn PowerSettingAccessCheckEx();
    #[doc = "*Required features: `Win32_System_Power`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn PowerSettingRegisterNotification();
    #[doc = "*Required features: `Win32_System_Power`*"]
    pub fn PowerSettingUnregisterNotification();
    #[doc = "*Required features: `Win32_System_Power`*"]
    pub fn PowerUnregisterFromEffectivePowerModeNotifications();
    #[doc = "*Required features: `Win32_System_Power`*"]
    pub fn PowerUnregisterSuspendResumeNotification();
    #[doc = "*Required features: `Win32_System_Power`, `Win32_System_Registry`*"]
    #[cfg(feature = "Win32_System_Registry")]
    pub fn PowerWriteACDefaultIndex();
    #[doc = "*Required features: `Win32_System_Power`, `Win32_System_Registry`*"]
    #[cfg(feature = "Win32_System_Registry")]
    pub fn PowerWriteACValueIndex();
    #[doc = "*Required features: `Win32_System_Power`, `Win32_System_Registry`*"]
    #[cfg(feature = "Win32_System_Registry")]
    pub fn PowerWriteDCDefaultIndex();
    #[doc = "*Required features: `Win32_System_Power`, `Win32_System_Registry`*"]
    #[cfg(feature = "Win32_System_Registry")]
    pub fn PowerWriteDCValueIndex();
    #[doc = "*Required features: `Win32_System_Power`, `Win32_System_Registry`*"]
    #[cfg(feature = "Win32_System_Registry")]
    pub fn PowerWriteDescription();
    #[doc = "*Required features: `Win32_System_Power`, `Win32_System_Registry`*"]
    #[cfg(feature = "Win32_System_Registry")]
    pub fn PowerWriteFriendlyName();
    #[doc = "*Required features: `Win32_System_Power`, `Win32_System_Registry`*"]
    #[cfg(feature = "Win32_System_Registry")]
    pub fn PowerWriteIconResourceSpecifier();
    #[doc = "*Required features: `Win32_System_Power`, `Win32_System_Registry`*"]
    #[cfg(feature = "Win32_System_Registry")]
    pub fn PowerWritePossibleDescription();
    #[doc = "*Required features: `Win32_System_Power`, `Win32_System_Registry`*"]
    #[cfg(feature = "Win32_System_Registry")]
    pub fn PowerWritePossibleFriendlyName();
    #[doc = "*Required features: `Win32_System_Power`, `Win32_System_Registry`*"]
    #[cfg(feature = "Win32_System_Registry")]
    pub fn PowerWritePossibleValue();
    #[doc = "*Required features: `Win32_System_Power`*"]
    pub fn PowerWriteSettingAttributes();
    #[doc = "*Required features: `Win32_System_Power`, `Win32_System_Registry`*"]
    #[cfg(feature = "Win32_System_Registry")]
    pub fn PowerWriteValueIncrement();
    #[doc = "*Required features: `Win32_System_Power`, `Win32_System_Registry`*"]
    #[cfg(feature = "Win32_System_Registry")]
    pub fn PowerWriteValueMax();
    #[doc = "*Required features: `Win32_System_Power`, `Win32_System_Registry`*"]
    #[cfg(feature = "Win32_System_Registry")]
    pub fn PowerWriteValueMin();
    #[doc = "*Required features: `Win32_System_Power`, `Win32_System_Registry`*"]
    #[cfg(feature = "Win32_System_Registry")]
    pub fn PowerWriteValueUnitsSpecifier();
    #[doc = "*Required features: `Win32_System_Power`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ReadGlobalPwrPolicy();
    #[doc = "*Required features: `Win32_System_Power`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ReadProcessorPwrScheme();
    #[doc = "*Required features: `Win32_System_Power`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ReadPwrScheme();
    #[doc = "*Required features: `Win32_System_Power`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn RegisterPowerSettingNotification();
    #[doc = "*Required features: `Win32_System_Power`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn RegisterSuspendResumeNotification();
    #[doc = "*Required features: `Win32_System_Power`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn RequestWakeupLatency();
    #[doc = "*Required features: `Win32_System_Power`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetActivePwrScheme();
    #[doc = "*Required features: `Win32_System_Power`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetSuspendState();
    #[doc = "*Required features: `Win32_System_Power`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetSystemPowerState();
    #[doc = "*Required features: `Win32_System_Power`*"]
    pub fn SetThreadExecutionState();
    #[doc = "*Required features: `Win32_System_Power`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn UnregisterPowerSettingNotification();
    #[doc = "*Required features: `Win32_System_Power`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn UnregisterSuspendResumeNotification();
    #[doc = "*Required features: `Win32_System_Power`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ValidatePowerPolicies();
    #[doc = "*Required features: `Win32_System_Power`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WriteGlobalPwrPolicy();
    #[doc = "*Required features: `Win32_System_Power`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WriteProcessorPwrScheme();
    #[doc = "*Required features: `Win32_System_Power`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WritePwrScheme();
}

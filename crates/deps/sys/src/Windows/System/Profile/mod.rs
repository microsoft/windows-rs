#![allow(non_snake_case, non_camel_case_types)]
#[cfg(feature = "System_Profile_SystemManufacturers")]
pub mod SystemManufacturers;
#[link(name = "windows")]
extern "system" {
    fn AnalyticsInfo();
    fn AnalyticsVersionInfo();
    fn AppApplicability();
    fn EducationSettings();
    fn HardwareIdentification();
    fn HardwareToken();
    fn IAnalyticsInfoStatics();
    fn IAnalyticsInfoStatics2();
    fn IAnalyticsVersionInfo();
    fn IAnalyticsVersionInfo2();
    fn IAppApplicabilityStatics();
    fn IEducationSettingsStatics();
    fn IHardwareIdentificationStatics();
    fn IHardwareToken();
    fn IKnownRetailInfoPropertiesStatics();
    fn IPlatformDiagnosticsAndUsageDataSettingsStatics();
    fn IRetailInfoStatics();
    fn ISharedModeSettingsStatics();
    fn ISharedModeSettingsStatics2();
    fn ISystemIdentificationInfo();
    fn ISystemIdentificationStatics();
    fn ISystemSetupInfoStatics();
    fn IUnsupportedAppRequirement();
    fn IWindowsIntegrityPolicyStatics();
    fn KnownRetailInfoProperties();
    fn PlatformDataCollectionLevel();
    fn PlatformDiagnosticsAndUsageDataSettings();
    fn ProfileHardwareTokenContract();
    fn ProfileRetailInfoContract();
    fn ProfileSharedModeContract();
    fn RetailInfo();
    fn SharedModeSettings();
    fn SystemIdentification();
    fn SystemIdentificationInfo();
    fn SystemIdentificationSource();
    fn SystemOutOfBoxExperienceState();
    fn SystemSetupInfo();
    fn UnsupportedAppRequirement();
    fn UnsupportedAppRequirementReasons();
    fn WindowsIntegrityPolicy();
}

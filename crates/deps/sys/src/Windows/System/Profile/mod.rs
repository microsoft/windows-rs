#![allow(non_snake_case, non_camel_case_types)]
#[cfg(feature = "System_Profile_SystemManufacturers")]
pub mod SystemManufacturers;
#[link(name = "windows")]
extern "system" {}
pub struct AnalyticsInfo(i32);
pub struct AnalyticsVersionInfo(i32);
pub struct AppApplicability(i32);
pub struct EducationSettings(i32);
pub struct HardwareIdentification(i32);
pub struct HardwareToken(i32);
pub struct IAnalyticsInfoStatics(pub *mut ::core::ffi::c_void);
pub struct IAnalyticsInfoStatics2(pub *mut ::core::ffi::c_void);
pub struct IAnalyticsVersionInfo(pub *mut ::core::ffi::c_void);
pub struct IAnalyticsVersionInfo2(pub *mut ::core::ffi::c_void);
pub struct IAppApplicabilityStatics(pub *mut ::core::ffi::c_void);
pub struct IEducationSettingsStatics(pub *mut ::core::ffi::c_void);
pub struct IHardwareIdentificationStatics(pub *mut ::core::ffi::c_void);
pub struct IHardwareToken(pub *mut ::core::ffi::c_void);
pub struct IKnownRetailInfoPropertiesStatics(pub *mut ::core::ffi::c_void);
pub struct IPlatformDiagnosticsAndUsageDataSettingsStatics(pub *mut ::core::ffi::c_void);
pub struct IRetailInfoStatics(pub *mut ::core::ffi::c_void);
pub struct ISharedModeSettingsStatics(pub *mut ::core::ffi::c_void);
pub struct ISharedModeSettingsStatics2(pub *mut ::core::ffi::c_void);
pub struct ISystemIdentificationInfo(pub *mut ::core::ffi::c_void);
pub struct ISystemIdentificationStatics(pub *mut ::core::ffi::c_void);
pub struct ISystemSetupInfoStatics(pub *mut ::core::ffi::c_void);
pub struct IUnsupportedAppRequirement(pub *mut ::core::ffi::c_void);
pub struct IWindowsIntegrityPolicyStatics(pub *mut ::core::ffi::c_void);
pub struct KnownRetailInfoProperties(i32);
pub struct PlatformDataCollectionLevel(i32);
pub struct PlatformDiagnosticsAndUsageDataSettings(i32);
pub struct ProfileHardwareTokenContract(i32);
pub struct ProfileRetailInfoContract(i32);
pub struct ProfileSharedModeContract(i32);
pub struct RetailInfo(i32);
pub struct SharedModeSettings(i32);
pub struct SystemIdentification(i32);
pub struct SystemIdentificationInfo(i32);
pub struct SystemIdentificationSource(i32);
pub struct SystemOutOfBoxExperienceState(i32);
pub struct SystemSetupInfo(i32);
pub struct UnsupportedAppRequirement(i32);
pub struct UnsupportedAppRequirementReasons(i32);
pub struct WindowsIntegrityPolicy(i32);

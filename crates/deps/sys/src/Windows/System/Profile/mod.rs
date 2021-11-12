#![allow(non_snake_case, non_camel_case_types)]
#[cfg(feature = "System_Profile_SystemManufacturers")]
pub mod SystemManufacturers;
#[link(name = "windows")]
extern "system" {}
#[repr(transparent)]
pub struct AnalyticsInfo(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct AnalyticsVersionInfo(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct AppApplicability(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct EducationSettings(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct HardwareIdentification(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct HardwareToken(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IAnalyticsInfoStatics(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IAnalyticsInfoStatics2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IAnalyticsVersionInfo(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IAnalyticsVersionInfo2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IAppApplicabilityStatics(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IEducationSettingsStatics(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IHardwareIdentificationStatics(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IHardwareToken(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IKnownRetailInfoPropertiesStatics(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IPlatformDiagnosticsAndUsageDataSettingsStatics(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IRetailInfoStatics(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ISharedModeSettingsStatics(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ISharedModeSettingsStatics2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ISystemIdentificationInfo(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ISystemIdentificationStatics(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ISystemSetupInfoStatics(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IUnsupportedAppRequirement(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IWindowsIntegrityPolicyStatics(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct KnownRetailInfoProperties(pub *mut ::core::ffi::c_void);
pub struct PlatformDataCollectionLevel(i32);
#[repr(transparent)]
pub struct PlatformDiagnosticsAndUsageDataSettings(pub *mut ::core::ffi::c_void);
pub struct ProfileHardwareTokenContract(i32);
pub struct ProfileRetailInfoContract(i32);
pub struct ProfileSharedModeContract(i32);
#[repr(transparent)]
pub struct RetailInfo(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct SharedModeSettings(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct SystemIdentification(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct SystemIdentificationInfo(pub *mut ::core::ffi::c_void);
pub struct SystemIdentificationSource(i32);
pub struct SystemOutOfBoxExperienceState(i32);
#[repr(transparent)]
pub struct SystemSetupInfo(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct UnsupportedAppRequirement(pub *mut ::core::ffi::c_void);
pub struct UnsupportedAppRequirementReasons(i32);
#[repr(transparent)]
pub struct WindowsIntegrityPolicy(pub *mut ::core::ffi::c_void);

#![allow(non_snake_case, non_camel_case_types)]
#[cfg(feature = "System_Profile_SystemManufacturers")]
pub mod SystemManufacturers;
#[link(name = "windows")]
extern "system" {}
#[repr(transparent)]
pub struct AnalyticsVersionInfo(pub *mut ::core::ffi::c_void);
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
#[repr(C)]
pub struct PlatformDataCollectionLevel(i32);
#[repr(C)]
pub struct ProfileHardwareTokenContract(i32);
#[repr(C)]
pub struct ProfileRetailInfoContract(i32);
#[repr(C)]
pub struct ProfileSharedModeContract(i32);
#[repr(transparent)]
pub struct SystemIdentificationInfo(pub *mut ::core::ffi::c_void);
#[repr(C)]
pub struct SystemIdentificationSource(i32);
#[repr(C)]
pub struct SystemOutOfBoxExperienceState(i32);
#[repr(transparent)]
pub struct UnsupportedAppRequirement(pub *mut ::core::ffi::c_void);
#[repr(C)]
pub struct UnsupportedAppRequirementReasons(i32);

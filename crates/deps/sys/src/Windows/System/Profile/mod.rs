#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals)]
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
#[repr(transparent)]
pub struct PlatformDataCollectionLevel(pub i32);
impl PlatformDataCollectionLevel {
    pub const Security: Self = Self(0i32);
    pub const Basic: Self = Self(1i32);
    pub const Enhanced: Self = Self(2i32);
    pub const Full: Self = Self(3i32);
}
#[repr(transparent)]
pub struct SystemIdentificationInfo(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct SystemIdentificationSource(pub i32);
impl SystemIdentificationSource {
    pub const None: Self = Self(0i32);
    pub const Tpm: Self = Self(1i32);
    pub const Uefi: Self = Self(2i32);
    pub const Registry: Self = Self(3i32);
}
#[repr(transparent)]
pub struct SystemOutOfBoxExperienceState(pub i32);
impl SystemOutOfBoxExperienceState {
    pub const NotStarted: Self = Self(0i32);
    pub const InProgress: Self = Self(1i32);
    pub const Completed: Self = Self(2i32);
}
#[repr(transparent)]
pub struct UnsupportedAppRequirement(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct UnsupportedAppRequirementReasons(pub u32);
impl UnsupportedAppRequirementReasons {
    pub const Unknown: Self = Self(0u32);
    pub const DeniedBySystem: Self = Self(1u32);
}

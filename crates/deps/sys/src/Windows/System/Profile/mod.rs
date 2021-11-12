#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals)]
#[cfg(feature = "System_Profile_SystemManufacturers")]
pub mod SystemManufacturers;
#[link(name = "windows")]
extern "system" {}
#[repr(transparent)]
pub struct AnalyticsVersionInfo(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for AnalyticsVersionInfo {}
impl ::core::clone::Clone for AnalyticsVersionInfo {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct HardwareToken(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for HardwareToken {}
impl ::core::clone::Clone for HardwareToken {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IAnalyticsInfoStatics(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IAnalyticsInfoStatics {}
impl ::core::clone::Clone for IAnalyticsInfoStatics {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IAnalyticsInfoStatics2(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IAnalyticsInfoStatics2 {}
impl ::core::clone::Clone for IAnalyticsInfoStatics2 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IAnalyticsVersionInfo(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IAnalyticsVersionInfo {}
impl ::core::clone::Clone for IAnalyticsVersionInfo {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IAnalyticsVersionInfo2(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IAnalyticsVersionInfo2 {}
impl ::core::clone::Clone for IAnalyticsVersionInfo2 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IAppApplicabilityStatics(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IAppApplicabilityStatics {}
impl ::core::clone::Clone for IAppApplicabilityStatics {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IEducationSettingsStatics(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IEducationSettingsStatics {}
impl ::core::clone::Clone for IEducationSettingsStatics {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IHardwareIdentificationStatics(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IHardwareIdentificationStatics {}
impl ::core::clone::Clone for IHardwareIdentificationStatics {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IHardwareToken(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IHardwareToken {}
impl ::core::clone::Clone for IHardwareToken {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IKnownRetailInfoPropertiesStatics(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IKnownRetailInfoPropertiesStatics {}
impl ::core::clone::Clone for IKnownRetailInfoPropertiesStatics {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IPlatformDiagnosticsAndUsageDataSettingsStatics(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IPlatformDiagnosticsAndUsageDataSettingsStatics {}
impl ::core::clone::Clone for IPlatformDiagnosticsAndUsageDataSettingsStatics {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IRetailInfoStatics(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IRetailInfoStatics {}
impl ::core::clone::Clone for IRetailInfoStatics {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ISharedModeSettingsStatics(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ISharedModeSettingsStatics {}
impl ::core::clone::Clone for ISharedModeSettingsStatics {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ISharedModeSettingsStatics2(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ISharedModeSettingsStatics2 {}
impl ::core::clone::Clone for ISharedModeSettingsStatics2 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ISystemIdentificationInfo(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ISystemIdentificationInfo {}
impl ::core::clone::Clone for ISystemIdentificationInfo {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ISystemIdentificationStatics(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ISystemIdentificationStatics {}
impl ::core::clone::Clone for ISystemIdentificationStatics {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ISystemSetupInfoStatics(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ISystemSetupInfoStatics {}
impl ::core::clone::Clone for ISystemSetupInfoStatics {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IUnsupportedAppRequirement(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IUnsupportedAppRequirement {}
impl ::core::clone::Clone for IUnsupportedAppRequirement {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IWindowsIntegrityPolicyStatics(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IWindowsIntegrityPolicyStatics {}
impl ::core::clone::Clone for IWindowsIntegrityPolicyStatics {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct PlatformDataCollectionLevel(pub i32);
impl PlatformDataCollectionLevel {
    pub const Security: Self = Self(0i32);
    pub const Basic: Self = Self(1i32);
    pub const Enhanced: Self = Self(2i32);
    pub const Full: Self = Self(3i32);
}
impl ::core::marker::Copy for PlatformDataCollectionLevel {}
impl ::core::clone::Clone for PlatformDataCollectionLevel {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct SystemIdentificationInfo(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for SystemIdentificationInfo {}
impl ::core::clone::Clone for SystemIdentificationInfo {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct SystemIdentificationSource(pub i32);
impl SystemIdentificationSource {
    pub const None: Self = Self(0i32);
    pub const Tpm: Self = Self(1i32);
    pub const Uefi: Self = Self(2i32);
    pub const Registry: Self = Self(3i32);
}
impl ::core::marker::Copy for SystemIdentificationSource {}
impl ::core::clone::Clone for SystemIdentificationSource {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct SystemOutOfBoxExperienceState(pub i32);
impl SystemOutOfBoxExperienceState {
    pub const NotStarted: Self = Self(0i32);
    pub const InProgress: Self = Self(1i32);
    pub const Completed: Self = Self(2i32);
}
impl ::core::marker::Copy for SystemOutOfBoxExperienceState {}
impl ::core::clone::Clone for SystemOutOfBoxExperienceState {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct UnsupportedAppRequirement(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for UnsupportedAppRequirement {}
impl ::core::clone::Clone for UnsupportedAppRequirement {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct UnsupportedAppRequirementReasons(pub u32);
impl UnsupportedAppRequirementReasons {
    pub const Unknown: Self = Self(0u32);
    pub const DeniedBySystem: Self = Self(1u32);
}
impl ::core::marker::Copy for UnsupportedAppRequirementReasons {}
impl ::core::clone::Clone for UnsupportedAppRequirementReasons {
    fn clone(&self) -> Self {
        *self
    }
}

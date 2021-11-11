#![allow(non_snake_case, non_camel_case_types)]
#[link(name = "windows")]
extern "system" {
    #[doc = "*Required features: `Win32_Storage_Packaging_Appx`*"]
    pub fn ActivatePackageVirtualizationContext();
    #[doc = "*Required features: `Win32_Storage_Packaging_Appx`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn AddPackageDependency();
    #[doc = "*Required features: `Win32_Storage_Packaging_Appx`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn AppPolicyGetClrCompat();
    #[doc = "*Required features: `Win32_Storage_Packaging_Appx`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn AppPolicyGetCreateFileAccess();
    #[doc = "*Required features: `Win32_Storage_Packaging_Appx`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn AppPolicyGetLifecycleManagement();
    #[doc = "*Required features: `Win32_Storage_Packaging_Appx`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn AppPolicyGetMediaFoundationCodecLoading();
    #[doc = "*Required features: `Win32_Storage_Packaging_Appx`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn AppPolicyGetProcessTerminationMethod();
    #[doc = "*Required features: `Win32_Storage_Packaging_Appx`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn AppPolicyGetShowDeveloperDiagnostic();
    #[doc = "*Required features: `Win32_Storage_Packaging_Appx`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn AppPolicyGetThreadInitializationType();
    #[doc = "*Required features: `Win32_Storage_Packaging_Appx`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn AppPolicyGetWindowingModel();
    #[doc = "*Required features: `Win32_Storage_Packaging_Appx`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CheckIsMSIXPackage();
    #[doc = "*Required features: `Win32_Storage_Packaging_Appx`*"]
    pub fn ClosePackageInfo();
    #[doc = "*Required features: `Win32_Storage_Packaging_Appx`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CreatePackageVirtualizationContext();
    #[doc = "*Required features: `Win32_Storage_Packaging_Appx`*"]
    pub fn DeactivatePackageVirtualizationContext();
    #[doc = "*Required features: `Win32_Storage_Packaging_Appx`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DeletePackageDependency();
    #[doc = "*Required features: `Win32_Storage_Packaging_Appx`*"]
    pub fn DuplicatePackageVirtualizationContext();
    #[doc = "*Required features: `Win32_Storage_Packaging_Appx`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn FindPackagesByPackageFamily();
    #[doc = "*Required features: `Win32_Storage_Packaging_Appx`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn FormatApplicationUserModelId();
    #[doc = "*Required features: `Win32_Storage_Packaging_Appx`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetApplicationUserModelId();
    #[doc = "*Required features: `Win32_Storage_Packaging_Appx`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetApplicationUserModelIdFromToken();
    #[doc = "*Required features: `Win32_Storage_Packaging_Appx`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetCurrentApplicationUserModelId();
    #[doc = "*Required features: `Win32_Storage_Packaging_Appx`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetCurrentPackageFamilyName();
    #[doc = "*Required features: `Win32_Storage_Packaging_Appx`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetCurrentPackageFullName();
    #[doc = "*Required features: `Win32_Storage_Packaging_Appx`*"]
    pub fn GetCurrentPackageId();
    #[doc = "*Required features: `Win32_Storage_Packaging_Appx`*"]
    pub fn GetCurrentPackageInfo();
    #[doc = "*Required features: `Win32_Storage_Packaging_Appx`*"]
    pub fn GetCurrentPackageInfo2();
    #[doc = "*Required features: `Win32_Storage_Packaging_Appx`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetCurrentPackagePath();
    #[doc = "*Required features: `Win32_Storage_Packaging_Appx`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetCurrentPackagePath2();
    #[doc = "*Required features: `Win32_Storage_Packaging_Appx`*"]
    pub fn GetCurrentPackageVirtualizationContext();
    #[doc = "*Required features: `Win32_Storage_Packaging_Appx`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetIdForPackageDependencyContext();
    #[doc = "*Required features: `Win32_Storage_Packaging_Appx`*"]
    pub fn GetPackageApplicationIds();
    #[doc = "*Required features: `Win32_Storage_Packaging_Appx`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetPackageFamilyName();
    #[doc = "*Required features: `Win32_Storage_Packaging_Appx`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetPackageFamilyNameFromToken();
    #[doc = "*Required features: `Win32_Storage_Packaging_Appx`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetPackageFullName();
    #[doc = "*Required features: `Win32_Storage_Packaging_Appx`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetPackageFullNameFromToken();
    #[doc = "*Required features: `Win32_Storage_Packaging_Appx`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetPackageId();
    #[doc = "*Required features: `Win32_Storage_Packaging_Appx`*"]
    pub fn GetPackageInfo();
    #[doc = "*Required features: `Win32_Storage_Packaging_Appx`*"]
    pub fn GetPackageInfo2();
    #[doc = "*Required features: `Win32_Storage_Packaging_Appx`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetPackagePath();
    #[doc = "*Required features: `Win32_Storage_Packaging_Appx`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetPackagePathByFullName();
    #[doc = "*Required features: `Win32_Storage_Packaging_Appx`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetPackagePathByFullName2();
    #[doc = "*Required features: `Win32_Storage_Packaging_Appx`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetPackagesByPackageFamily();
    #[doc = "*Required features: `Win32_Storage_Packaging_Appx`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetProcessesInVirtualizationContext();
    #[doc = "*Required features: `Win32_Storage_Packaging_Appx`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetResolvedPackageFullNameForPackageDependency();
    #[doc = "*Required features: `Win32_Storage_Packaging_Appx`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetStagedPackageOrigin();
    #[doc = "*Required features: `Win32_Storage_Packaging_Appx`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetStagedPackagePathByFullName();
    #[doc = "*Required features: `Win32_Storage_Packaging_Appx`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetStagedPackagePathByFullName2();
    #[doc = "*Required features: `Win32_Storage_Packaging_Appx`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn OpenPackageInfoByFullName();
    #[doc = "*Required features: `Win32_Storage_Packaging_Appx`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn OpenPackageInfoByFullNameForUser();
    #[doc = "*Required features: `Win32_Storage_Packaging_Appx`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn PackageFamilyNameFromFullName();
    #[doc = "*Required features: `Win32_Storage_Packaging_Appx`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn PackageFamilyNameFromId();
    #[doc = "*Required features: `Win32_Storage_Packaging_Appx`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn PackageFullNameFromId();
    #[doc = "*Required features: `Win32_Storage_Packaging_Appx`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn PackageIdFromFullName();
    #[doc = "*Required features: `Win32_Storage_Packaging_Appx`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn PackageNameAndPublisherIdFromFamilyName();
    #[doc = "*Required features: `Win32_Storage_Packaging_Appx`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ParseApplicationUserModelId();
    #[doc = "*Required features: `Win32_Storage_Packaging_Appx`*"]
    pub fn ReleasePackageVirtualizationContext();
    #[doc = "*Required features: `Win32_Storage_Packaging_Appx`*"]
    pub fn RemovePackageDependency();
    #[doc = "*Required features: `Win32_Storage_Packaging_Appx`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn TryCreatePackageDependency();
    #[doc = "*Required features: `Win32_Storage_Packaging_Appx`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn VerifyApplicationUserModelId();
    #[doc = "*Required features: `Win32_Storage_Packaging_Appx`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn VerifyPackageFamilyName();
    #[doc = "*Required features: `Win32_Storage_Packaging_Appx`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn VerifyPackageFullName();
    #[doc = "*Required features: `Win32_Storage_Packaging_Appx`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn VerifyPackageId();
    #[doc = "*Required features: `Win32_Storage_Packaging_Appx`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn VerifyPackageRelativeApplicationId();
}

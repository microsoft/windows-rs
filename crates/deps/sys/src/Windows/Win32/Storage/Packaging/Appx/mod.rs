#![allow(non_snake_case, non_camel_case_types)]
#[link(name = "windows")]
extern "system" {
    #[doc = "*Required features: `Win32_Storage_Packaging_Appx`*"]
    pub fn ActivatePackageVirtualizationContext(context: *const PACKAGE_VIRTUALIZATION_CONTEXT_HANDLE__, cookie: *mut usize) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `Win32_Storage_Packaging_Appx`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn AddPackageDependency(packagedependencyid: super::super::super::Foundation::PWSTR, rank: i32, options: AddPackageDependencyOptions, packagedependencycontext: *mut *mut PACKAGEDEPENDENCY_CONTEXT__, packagefullname: *mut super::super::super::Foundation::PWSTR) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `Win32_Storage_Packaging_Appx`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn AppPolicyGetClrCompat(processtoken: super::super::super::Foundation::HANDLE, policy: *mut AppPolicyClrCompat) -> i32;
    #[doc = "*Required features: `Win32_Storage_Packaging_Appx`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn AppPolicyGetCreateFileAccess(processtoken: super::super::super::Foundation::HANDLE, policy: *mut AppPolicyCreateFileAccess) -> i32;
    #[doc = "*Required features: `Win32_Storage_Packaging_Appx`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn AppPolicyGetLifecycleManagement(processtoken: super::super::super::Foundation::HANDLE, policy: *mut AppPolicyLifecycleManagement) -> i32;
    #[doc = "*Required features: `Win32_Storage_Packaging_Appx`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn AppPolicyGetMediaFoundationCodecLoading(processtoken: super::super::super::Foundation::HANDLE, policy: *mut AppPolicyMediaFoundationCodecLoading) -> i32;
    #[doc = "*Required features: `Win32_Storage_Packaging_Appx`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn AppPolicyGetProcessTerminationMethod(processtoken: super::super::super::Foundation::HANDLE, policy: *mut AppPolicyProcessTerminationMethod) -> i32;
    #[doc = "*Required features: `Win32_Storage_Packaging_Appx`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn AppPolicyGetShowDeveloperDiagnostic(processtoken: super::super::super::Foundation::HANDLE, policy: *mut AppPolicyShowDeveloperDiagnostic) -> i32;
    #[doc = "*Required features: `Win32_Storage_Packaging_Appx`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn AppPolicyGetThreadInitializationType(processtoken: super::super::super::Foundation::HANDLE, policy: *mut AppPolicyThreadInitializationType) -> i32;
    #[doc = "*Required features: `Win32_Storage_Packaging_Appx`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn AppPolicyGetWindowingModel(processtoken: super::super::super::Foundation::HANDLE, policy: *mut AppPolicyWindowingModel) -> i32;
    #[doc = "*Required features: `Win32_Storage_Packaging_Appx`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CheckIsMSIXPackage(packagefullname: super::super::super::Foundation::PWSTR, ismsixpackage: *mut super::super::super::Foundation::BOOL) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `Win32_Storage_Packaging_Appx`*"]
    pub fn ClosePackageInfo(packageinforeference: *const _PACKAGE_INFO_REFERENCE) -> i32;
    #[doc = "*Required features: `Win32_Storage_Packaging_Appx`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CreatePackageVirtualizationContext(packagefamilyname: super::super::super::Foundation::PWSTR, context: *mut *mut PACKAGE_VIRTUALIZATION_CONTEXT_HANDLE__) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `Win32_Storage_Packaging_Appx`*"]
    pub fn DeactivatePackageVirtualizationContext(cookie: usize);
    #[doc = "*Required features: `Win32_Storage_Packaging_Appx`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DeletePackageDependency(packagedependencyid: super::super::super::Foundation::PWSTR) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `Win32_Storage_Packaging_Appx`*"]
    pub fn DuplicatePackageVirtualizationContext(sourcecontext: *const PACKAGE_VIRTUALIZATION_CONTEXT_HANDLE__, destcontext: *mut *mut PACKAGE_VIRTUALIZATION_CONTEXT_HANDLE__) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `Win32_Storage_Packaging_Appx`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn FindPackagesByPackageFamily(packagefamilyname: super::super::super::Foundation::PWSTR, packagefilters: u32, count: *mut u32, packagefullnames: *mut super::super::super::Foundation::PWSTR, bufferlength: *mut u32, buffer: super::super::super::Foundation::PWSTR, packageproperties: *mut u32) -> i32;
    #[doc = "*Required features: `Win32_Storage_Packaging_Appx`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn FormatApplicationUserModelId(packagefamilyname: super::super::super::Foundation::PWSTR, packagerelativeapplicationid: super::super::super::Foundation::PWSTR, applicationusermodelidlength: *mut u32, applicationusermodelid: super::super::super::Foundation::PWSTR) -> i32;
    #[doc = "*Required features: `Win32_Storage_Packaging_Appx`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetApplicationUserModelId(hprocess: super::super::super::Foundation::HANDLE, applicationusermodelidlength: *mut u32, applicationusermodelid: super::super::super::Foundation::PWSTR) -> i32;
    #[doc = "*Required features: `Win32_Storage_Packaging_Appx`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetApplicationUserModelIdFromToken(token: super::super::super::Foundation::HANDLE, applicationusermodelidlength: *mut u32, applicationusermodelid: super::super::super::Foundation::PWSTR) -> i32;
    #[doc = "*Required features: `Win32_Storage_Packaging_Appx`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetCurrentApplicationUserModelId(applicationusermodelidlength: *mut u32, applicationusermodelid: super::super::super::Foundation::PWSTR) -> i32;
    #[doc = "*Required features: `Win32_Storage_Packaging_Appx`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetCurrentPackageFamilyName(packagefamilynamelength: *mut u32, packagefamilyname: super::super::super::Foundation::PWSTR) -> i32;
    #[doc = "*Required features: `Win32_Storage_Packaging_Appx`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetCurrentPackageFullName(packagefullnamelength: *mut u32, packagefullname: super::super::super::Foundation::PWSTR) -> i32;
    #[doc = "*Required features: `Win32_Storage_Packaging_Appx`*"]
    pub fn GetCurrentPackageId(bufferlength: *mut u32, buffer: *mut u8) -> i32;
    #[doc = "*Required features: `Win32_Storage_Packaging_Appx`*"]
    pub fn GetCurrentPackageInfo(flags: u32, bufferlength: *mut u32, buffer: *mut u8, count: *mut u32) -> i32;
    #[doc = "*Required features: `Win32_Storage_Packaging_Appx`*"]
    pub fn GetCurrentPackageInfo2(flags: u32, packagepathtype: PackagePathType, bufferlength: *mut u32, buffer: *mut u8, count: *mut u32) -> i32;
    #[doc = "*Required features: `Win32_Storage_Packaging_Appx`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetCurrentPackagePath(pathlength: *mut u32, path: super::super::super::Foundation::PWSTR) -> i32;
    #[doc = "*Required features: `Win32_Storage_Packaging_Appx`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetCurrentPackagePath2(packagepathtype: PackagePathType, pathlength: *mut u32, path: super::super::super::Foundation::PWSTR) -> i32;
    #[doc = "*Required features: `Win32_Storage_Packaging_Appx`*"]
    pub fn GetCurrentPackageVirtualizationContext() -> *mut PACKAGE_VIRTUALIZATION_CONTEXT_HANDLE__;
    #[doc = "*Required features: `Win32_Storage_Packaging_Appx`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetIdForPackageDependencyContext(packagedependencycontext: *const PACKAGEDEPENDENCY_CONTEXT__, packagedependencyid: *mut super::super::super::Foundation::PWSTR) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `Win32_Storage_Packaging_Appx`*"]
    pub fn GetPackageApplicationIds(packageinforeference: *const _PACKAGE_INFO_REFERENCE, bufferlength: *mut u32, buffer: *mut u8, count: *mut u32) -> i32;
    #[doc = "*Required features: `Win32_Storage_Packaging_Appx`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetPackageFamilyName(hprocess: super::super::super::Foundation::HANDLE, packagefamilynamelength: *mut u32, packagefamilyname: super::super::super::Foundation::PWSTR) -> i32;
    #[doc = "*Required features: `Win32_Storage_Packaging_Appx`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetPackageFamilyNameFromToken(token: super::super::super::Foundation::HANDLE, packagefamilynamelength: *mut u32, packagefamilyname: super::super::super::Foundation::PWSTR) -> i32;
    #[doc = "*Required features: `Win32_Storage_Packaging_Appx`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetPackageFullName(hprocess: super::super::super::Foundation::HANDLE, packagefullnamelength: *mut u32, packagefullname: super::super::super::Foundation::PWSTR) -> i32;
    #[doc = "*Required features: `Win32_Storage_Packaging_Appx`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetPackageFullNameFromToken(token: super::super::super::Foundation::HANDLE, packagefullnamelength: *mut u32, packagefullname: super::super::super::Foundation::PWSTR) -> i32;
    #[doc = "*Required features: `Win32_Storage_Packaging_Appx`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetPackageId(hprocess: super::super::super::Foundation::HANDLE, bufferlength: *mut u32, buffer: *mut u8) -> i32;
    #[doc = "*Required features: `Win32_Storage_Packaging_Appx`*"]
    pub fn GetPackageInfo(packageinforeference: *const _PACKAGE_INFO_REFERENCE, flags: u32, bufferlength: *mut u32, buffer: *mut u8, count: *mut u32) -> i32;
    #[doc = "*Required features: `Win32_Storage_Packaging_Appx`*"]
    pub fn GetPackageInfo2(packageinforeference: *const _PACKAGE_INFO_REFERENCE, flags: u32, packagepathtype: PackagePathType, bufferlength: *mut u32, buffer: *mut u8, count: *mut u32) -> i32;
    #[doc = "*Required features: `Win32_Storage_Packaging_Appx`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetPackagePath(packageid: *const PACKAGE_ID, reserved: u32, pathlength: *mut u32, path: super::super::super::Foundation::PWSTR) -> i32;
    #[doc = "*Required features: `Win32_Storage_Packaging_Appx`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetPackagePathByFullName(packagefullname: super::super::super::Foundation::PWSTR, pathlength: *mut u32, path: super::super::super::Foundation::PWSTR) -> i32;
    #[doc = "*Required features: `Win32_Storage_Packaging_Appx`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetPackagePathByFullName2(packagefullname: super::super::super::Foundation::PWSTR, packagepathtype: PackagePathType, pathlength: *mut u32, path: super::super::super::Foundation::PWSTR) -> i32;
    #[doc = "*Required features: `Win32_Storage_Packaging_Appx`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetPackagesByPackageFamily(packagefamilyname: super::super::super::Foundation::PWSTR, count: *mut u32, packagefullnames: *mut super::super::super::Foundation::PWSTR, bufferlength: *mut u32, buffer: super::super::super::Foundation::PWSTR) -> i32;
    #[doc = "*Required features: `Win32_Storage_Packaging_Appx`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetProcessesInVirtualizationContext(packagefamilyname: super::super::super::Foundation::PWSTR, count: *mut u32, processes: *mut *mut super::super::super::Foundation::HANDLE) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `Win32_Storage_Packaging_Appx`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetResolvedPackageFullNameForPackageDependency(packagedependencyid: super::super::super::Foundation::PWSTR, packagefullname: *mut super::super::super::Foundation::PWSTR) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `Win32_Storage_Packaging_Appx`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetStagedPackageOrigin(packagefullname: super::super::super::Foundation::PWSTR, origin: *mut PackageOrigin) -> i32;
    #[doc = "*Required features: `Win32_Storage_Packaging_Appx`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetStagedPackagePathByFullName(packagefullname: super::super::super::Foundation::PWSTR, pathlength: *mut u32, path: super::super::super::Foundation::PWSTR) -> i32;
    #[doc = "*Required features: `Win32_Storage_Packaging_Appx`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetStagedPackagePathByFullName2(packagefullname: super::super::super::Foundation::PWSTR, packagepathtype: PackagePathType, pathlength: *mut u32, path: super::super::super::Foundation::PWSTR) -> i32;
    #[doc = "*Required features: `Win32_Storage_Packaging_Appx`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn OpenPackageInfoByFullName(packagefullname: super::super::super::Foundation::PWSTR, reserved: u32, packageinforeference: *mut *mut _PACKAGE_INFO_REFERENCE) -> i32;
    #[doc = "*Required features: `Win32_Storage_Packaging_Appx`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn OpenPackageInfoByFullNameForUser(usersid: super::super::super::Foundation::PSID, packagefullname: super::super::super::Foundation::PWSTR, reserved: u32, packageinforeference: *mut *mut _PACKAGE_INFO_REFERENCE) -> i32;
    #[doc = "*Required features: `Win32_Storage_Packaging_Appx`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn PackageFamilyNameFromFullName(packagefullname: super::super::super::Foundation::PWSTR, packagefamilynamelength: *mut u32, packagefamilyname: super::super::super::Foundation::PWSTR) -> i32;
    #[doc = "*Required features: `Win32_Storage_Packaging_Appx`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn PackageFamilyNameFromId(packageid: *const PACKAGE_ID, packagefamilynamelength: *mut u32, packagefamilyname: super::super::super::Foundation::PWSTR) -> i32;
    #[doc = "*Required features: `Win32_Storage_Packaging_Appx`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn PackageFullNameFromId(packageid: *const PACKAGE_ID, packagefullnamelength: *mut u32, packagefullname: super::super::super::Foundation::PWSTR) -> i32;
    #[doc = "*Required features: `Win32_Storage_Packaging_Appx`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn PackageIdFromFullName(packagefullname: super::super::super::Foundation::PWSTR, flags: u32, bufferlength: *mut u32, buffer: *mut u8) -> i32;
    #[doc = "*Required features: `Win32_Storage_Packaging_Appx`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn PackageNameAndPublisherIdFromFamilyName(packagefamilyname: super::super::super::Foundation::PWSTR, packagenamelength: *mut u32, packagename: super::super::super::Foundation::PWSTR, packagepublisheridlength: *mut u32, packagepublisherid: super::super::super::Foundation::PWSTR) -> i32;
    #[doc = "*Required features: `Win32_Storage_Packaging_Appx`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ParseApplicationUserModelId(applicationusermodelid: super::super::super::Foundation::PWSTR, packagefamilynamelength: *mut u32, packagefamilyname: super::super::super::Foundation::PWSTR, packagerelativeapplicationidlength: *mut u32, packagerelativeapplicationid: super::super::super::Foundation::PWSTR) -> i32;
    #[doc = "*Required features: `Win32_Storage_Packaging_Appx`*"]
    pub fn ReleasePackageVirtualizationContext(context: *const PACKAGE_VIRTUALIZATION_CONTEXT_HANDLE__);
    #[doc = "*Required features: `Win32_Storage_Packaging_Appx`*"]
    pub fn RemovePackageDependency(packagedependencycontext: *const PACKAGEDEPENDENCY_CONTEXT__) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `Win32_Storage_Packaging_Appx`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn TryCreatePackageDependency(user: super::super::super::Foundation::PSID, packagefamilyname: super::super::super::Foundation::PWSTR, minversion: PACKAGE_VERSION, packagedependencyprocessorarchitectures: PackageDependencyProcessorArchitectures, lifetimekind: PackageDependencyLifetimeKind, lifetimeartifact: super::super::super::Foundation::PWSTR, options: CreatePackageDependencyOptions, packagedependencyid: *mut super::super::super::Foundation::PWSTR) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `Win32_Storage_Packaging_Appx`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn VerifyApplicationUserModelId(applicationusermodelid: super::super::super::Foundation::PWSTR) -> i32;
    #[doc = "*Required features: `Win32_Storage_Packaging_Appx`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn VerifyPackageFamilyName(packagefamilyname: super::super::super::Foundation::PWSTR) -> i32;
    #[doc = "*Required features: `Win32_Storage_Packaging_Appx`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn VerifyPackageFullName(packagefullname: super::super::super::Foundation::PWSTR) -> i32;
    #[doc = "*Required features: `Win32_Storage_Packaging_Appx`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn VerifyPackageId(packageid: *const PACKAGE_ID) -> i32;
    #[doc = "*Required features: `Win32_Storage_Packaging_Appx`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn VerifyPackageRelativeApplicationId(packagerelativeapplicationid: super::super::super::Foundation::PWSTR) -> i32;
}
pub struct APPX_BUNDLE_FOOTPRINT_FILE_TYPE(i32);
pub struct APPX_BUNDLE_PAYLOAD_PACKAGE_TYPE(i32);
pub struct APPX_CAPABILITIES(i32);
pub struct APPX_CAPABILITY_CLASS_TYPE(i32);
pub struct APPX_COMPRESSION_OPTION(i32);
pub struct APPX_ENCRYPTED_EXEMPTIONS(i32);
pub struct APPX_ENCRYPTED_PACKAGE_OPTIONS(i32);
pub struct APPX_ENCRYPTED_PACKAGE_SETTINGS(i32);
pub struct APPX_ENCRYPTED_PACKAGE_SETTINGS2(i32);
pub struct APPX_FOOTPRINT_FILE_TYPE(i32);
pub struct APPX_KEY_INFO(i32);
pub struct APPX_PACKAGE_ARCHITECTURE(i32);
pub struct APPX_PACKAGE_ARCHITECTURE2(i32);
pub struct APPX_PACKAGE_EDITOR_UPDATE_PACKAGE_MANIFEST_OPTIONS(i32);
pub struct APPX_PACKAGE_EDITOR_UPDATE_PACKAGE_OPTION(i32);
pub struct APPX_PACKAGE_SETTINGS(i32);
pub struct APPX_PACKAGE_WRITER_PAYLOAD_STREAM(i32);
pub struct APPX_PACKAGING_CONTEXT_CHANGE_TYPE(i32);
pub struct AddPackageDependencyOptions(i32);
pub struct AppPolicyClrCompat(i32);
pub struct AppPolicyCreateFileAccess(i32);
pub struct AppPolicyLifecycleManagement(i32);
pub struct AppPolicyMediaFoundationCodecLoading(i32);
pub struct AppPolicyProcessTerminationMethod(i32);
pub struct AppPolicyShowDeveloperDiagnostic(i32);
pub struct AppPolicyThreadInitializationType(i32);
pub struct AppPolicyWindowingModel(i32);
pub struct AppxBundleFactory(i32);
pub struct AppxEncryptionFactory(i32);
pub struct AppxFactory(i32);
pub struct AppxPackageEditor(i32);
pub struct AppxPackagingDiagnosticEventSinkManager(i32);
pub struct CreatePackageDependencyOptions(i32);
pub struct DX_FEATURE_LEVEL(i32);
pub struct IAppxBlockMapBlock(i32);
pub struct IAppxBlockMapBlocksEnumerator(i32);
pub struct IAppxBlockMapFile(i32);
pub struct IAppxBlockMapFilesEnumerator(i32);
pub struct IAppxBlockMapReader(i32);
pub struct IAppxBundleFactory(i32);
pub struct IAppxBundleManifestOptionalBundleInfo(i32);
pub struct IAppxBundleManifestOptionalBundleInfoEnumerator(i32);
pub struct IAppxBundleManifestPackageInfo(i32);
pub struct IAppxBundleManifestPackageInfo2(i32);
pub struct IAppxBundleManifestPackageInfo3(i32);
pub struct IAppxBundleManifestPackageInfo4(i32);
pub struct IAppxBundleManifestPackageInfoEnumerator(i32);
pub struct IAppxBundleManifestReader(i32);
pub struct IAppxBundleManifestReader2(i32);
pub struct IAppxBundleReader(i32);
pub struct IAppxBundleWriter(i32);
pub struct IAppxBundleWriter2(i32);
pub struct IAppxBundleWriter3(i32);
pub struct IAppxBundleWriter4(i32);
pub struct IAppxContentGroup(i32);
pub struct IAppxContentGroupFilesEnumerator(i32);
pub struct IAppxContentGroupMapReader(i32);
pub struct IAppxContentGroupMapWriter(i32);
pub struct IAppxContentGroupsEnumerator(i32);
pub struct IAppxEncryptedBundleWriter(i32);
pub struct IAppxEncryptedBundleWriter2(i32);
pub struct IAppxEncryptedBundleWriter3(i32);
pub struct IAppxEncryptedPackageWriter(i32);
pub struct IAppxEncryptedPackageWriter2(i32);
pub struct IAppxEncryptionFactory(i32);
pub struct IAppxEncryptionFactory2(i32);
pub struct IAppxEncryptionFactory3(i32);
pub struct IAppxEncryptionFactory4(i32);
pub struct IAppxFactory(i32);
pub struct IAppxFactory2(i32);
pub struct IAppxFile(i32);
pub struct IAppxFilesEnumerator(i32);
pub struct IAppxManifestApplication(i32);
pub struct IAppxManifestApplicationsEnumerator(i32);
pub struct IAppxManifestCapabilitiesEnumerator(i32);
pub struct IAppxManifestDeviceCapabilitiesEnumerator(i32);
pub struct IAppxManifestDriverConstraint(i32);
pub struct IAppxManifestDriverConstraintsEnumerator(i32);
pub struct IAppxManifestDriverDependenciesEnumerator(i32);
pub struct IAppxManifestDriverDependency(i32);
pub struct IAppxManifestHostRuntimeDependenciesEnumerator(i32);
pub struct IAppxManifestHostRuntimeDependency(i32);
pub struct IAppxManifestHostRuntimeDependency2(i32);
pub struct IAppxManifestMainPackageDependenciesEnumerator(i32);
pub struct IAppxManifestMainPackageDependency(i32);
pub struct IAppxManifestOSPackageDependenciesEnumerator(i32);
pub struct IAppxManifestOSPackageDependency(i32);
pub struct IAppxManifestOptionalPackageInfo(i32);
pub struct IAppxManifestPackageDependenciesEnumerator(i32);
pub struct IAppxManifestPackageDependency(i32);
pub struct IAppxManifestPackageDependency2(i32);
pub struct IAppxManifestPackageDependency3(i32);
pub struct IAppxManifestPackageId(i32);
pub struct IAppxManifestPackageId2(i32);
pub struct IAppxManifestProperties(i32);
pub struct IAppxManifestQualifiedResource(i32);
pub struct IAppxManifestQualifiedResourcesEnumerator(i32);
pub struct IAppxManifestReader(i32);
pub struct IAppxManifestReader2(i32);
pub struct IAppxManifestReader3(i32);
pub struct IAppxManifestReader4(i32);
pub struct IAppxManifestReader5(i32);
pub struct IAppxManifestReader6(i32);
pub struct IAppxManifestReader7(i32);
pub struct IAppxManifestResourcesEnumerator(i32);
pub struct IAppxManifestTargetDeviceFamiliesEnumerator(i32);
pub struct IAppxManifestTargetDeviceFamily(i32);
pub struct IAppxPackageEditor(i32);
pub struct IAppxPackageReader(i32);
pub struct IAppxPackageWriter(i32);
pub struct IAppxPackageWriter2(i32);
pub struct IAppxPackageWriter3(i32);
pub struct IAppxPackagingDiagnosticEventSink(i32);
pub struct IAppxPackagingDiagnosticEventSinkManager(i32);
pub struct IAppxSourceContentGroupMapReader(i32);
pub struct PACKAGEDEPENDENCY_CONTEXT__(i32);
#[doc = "*Required features: `Win32_Storage_Packaging_Appx`*"]
pub const PACKAGE_DEPENDENCY_RANK_DEFAULT: u32 = 0u32;
#[doc = "*Required features: `Win32_Storage_Packaging_Appx`*"]
pub const PACKAGE_FILTER_ALL_LOADED: u32 = 0u32;
#[doc = "*Required features: `Win32_Storage_Packaging_Appx`*"]
pub const PACKAGE_FILTER_BUNDLE: u32 = 128u32;
#[doc = "*Required features: `Win32_Storage_Packaging_Appx`*"]
pub const PACKAGE_FILTER_DIRECT: u32 = 32u32;
#[doc = "*Required features: `Win32_Storage_Packaging_Appx`*"]
pub const PACKAGE_FILTER_DYNAMIC: u32 = 1048576u32;
#[doc = "*Required features: `Win32_Storage_Packaging_Appx`*"]
pub const PACKAGE_FILTER_HEAD: u32 = 16u32;
#[doc = "*Required features: `Win32_Storage_Packaging_Appx`*"]
pub const PACKAGE_FILTER_HOSTRUNTIME: u32 = 2097152u32;
#[doc = "*Required features: `Win32_Storage_Packaging_Appx`*"]
pub const PACKAGE_FILTER_IS_IN_RELATED_SET: u32 = 262144u32;
#[doc = "*Required features: `Win32_Storage_Packaging_Appx`*"]
pub const PACKAGE_FILTER_OPTIONAL: u32 = 131072u32;
#[doc = "*Required features: `Win32_Storage_Packaging_Appx`*"]
pub const PACKAGE_FILTER_RESOURCE: u32 = 64u32;
#[doc = "*Required features: `Win32_Storage_Packaging_Appx`*"]
pub const PACKAGE_FILTER_STATIC: u32 = 524288u32;
pub struct PACKAGE_ID(i32);
pub struct PACKAGE_INFO(i32);
#[doc = "*Required features: `Win32_Storage_Packaging_Appx`*"]
pub const PACKAGE_INFORMATION_BASIC: u32 = 0u32;
#[doc = "*Required features: `Win32_Storage_Packaging_Appx`*"]
pub const PACKAGE_INFORMATION_FULL: u32 = 256u32;
#[doc = "*Required features: `Win32_Storage_Packaging_Appx`*"]
pub const PACKAGE_PROPERTY_BUNDLE: u32 = 4u32;
#[doc = "*Required features: `Win32_Storage_Packaging_Appx`*"]
pub const PACKAGE_PROPERTY_DEVELOPMENT_MODE: u32 = 65536u32;
#[doc = "*Required features: `Win32_Storage_Packaging_Appx`*"]
pub const PACKAGE_PROPERTY_DYNAMIC: u32 = 1048576u32;
#[doc = "*Required features: `Win32_Storage_Packaging_Appx`*"]
pub const PACKAGE_PROPERTY_FRAMEWORK: u32 = 1u32;
#[doc = "*Required features: `Win32_Storage_Packaging_Appx`*"]
pub const PACKAGE_PROPERTY_HOSTRUNTIME: u32 = 2097152u32;
#[doc = "*Required features: `Win32_Storage_Packaging_Appx`*"]
pub const PACKAGE_PROPERTY_IS_IN_RELATED_SET: u32 = 262144u32;
#[doc = "*Required features: `Win32_Storage_Packaging_Appx`*"]
pub const PACKAGE_PROPERTY_OPTIONAL: u32 = 8u32;
#[doc = "*Required features: `Win32_Storage_Packaging_Appx`*"]
pub const PACKAGE_PROPERTY_RESOURCE: u32 = 2u32;
#[doc = "*Required features: `Win32_Storage_Packaging_Appx`*"]
pub const PACKAGE_PROPERTY_STATIC: u32 = 524288u32;
pub struct PACKAGE_VERSION(i32);
pub struct PACKAGE_VIRTUALIZATION_CONTEXT_HANDLE__(i32);
pub struct PackageDependencyLifetimeKind(i32);
pub struct PackageDependencyProcessorArchitectures(i32);
pub struct PackageOrigin(i32);
pub struct PackagePathType(i32);
pub struct _PACKAGE_INFO_REFERENCE(i32);

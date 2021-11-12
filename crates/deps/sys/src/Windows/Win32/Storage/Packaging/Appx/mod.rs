#![allow(non_snake_case, non_camel_case_types)]
#[link(name = "windows")]
extern "system" {
    pub fn ActivatePackageVirtualizationContext(context: *const PACKAGE_VIRTUALIZATION_CONTEXT_HANDLE__, cookie: *mut usize) -> ::windows_sys::core::HRESULT;
    #[cfg(feature = "Win32_Foundation")]
    pub fn AddPackageDependency(packagedependencyid: super::super::super::Foundation::PWSTR, rank: i32, options: AddPackageDependencyOptions, packagedependencycontext: *mut *mut PACKAGEDEPENDENCY_CONTEXT__, packagefullname: *mut super::super::super::Foundation::PWSTR) -> ::windows_sys::core::HRESULT;
    #[cfg(feature = "Win32_Foundation")]
    pub fn AppPolicyGetClrCompat(processtoken: super::super::super::Foundation::HANDLE, policy: *mut AppPolicyClrCompat) -> i32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn AppPolicyGetCreateFileAccess(processtoken: super::super::super::Foundation::HANDLE, policy: *mut AppPolicyCreateFileAccess) -> i32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn AppPolicyGetLifecycleManagement(processtoken: super::super::super::Foundation::HANDLE, policy: *mut AppPolicyLifecycleManagement) -> i32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn AppPolicyGetMediaFoundationCodecLoading(processtoken: super::super::super::Foundation::HANDLE, policy: *mut AppPolicyMediaFoundationCodecLoading) -> i32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn AppPolicyGetProcessTerminationMethod(processtoken: super::super::super::Foundation::HANDLE, policy: *mut AppPolicyProcessTerminationMethod) -> i32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn AppPolicyGetShowDeveloperDiagnostic(processtoken: super::super::super::Foundation::HANDLE, policy: *mut AppPolicyShowDeveloperDiagnostic) -> i32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn AppPolicyGetThreadInitializationType(processtoken: super::super::super::Foundation::HANDLE, policy: *mut AppPolicyThreadInitializationType) -> i32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn AppPolicyGetWindowingModel(processtoken: super::super::super::Foundation::HANDLE, policy: *mut AppPolicyWindowingModel) -> i32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn CheckIsMSIXPackage(packagefullname: super::super::super::Foundation::PWSTR, ismsixpackage: *mut super::super::super::Foundation::BOOL) -> ::windows_sys::core::HRESULT;
    pub fn ClosePackageInfo(packageinforeference: *const _PACKAGE_INFO_REFERENCE) -> i32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn CreatePackageVirtualizationContext(packagefamilyname: super::super::super::Foundation::PWSTR, context: *mut *mut PACKAGE_VIRTUALIZATION_CONTEXT_HANDLE__) -> ::windows_sys::core::HRESULT;
    pub fn DeactivatePackageVirtualizationContext(cookie: usize);
    #[cfg(feature = "Win32_Foundation")]
    pub fn DeletePackageDependency(packagedependencyid: super::super::super::Foundation::PWSTR) -> ::windows_sys::core::HRESULT;
    pub fn DuplicatePackageVirtualizationContext(sourcecontext: *const PACKAGE_VIRTUALIZATION_CONTEXT_HANDLE__, destcontext: *mut *mut PACKAGE_VIRTUALIZATION_CONTEXT_HANDLE__) -> ::windows_sys::core::HRESULT;
    #[cfg(feature = "Win32_Foundation")]
    pub fn FindPackagesByPackageFamily(packagefamilyname: super::super::super::Foundation::PWSTR, packagefilters: u32, count: *mut u32, packagefullnames: *mut super::super::super::Foundation::PWSTR, bufferlength: *mut u32, buffer: super::super::super::Foundation::PWSTR, packageproperties: *mut u32) -> i32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn FormatApplicationUserModelId(packagefamilyname: super::super::super::Foundation::PWSTR, packagerelativeapplicationid: super::super::super::Foundation::PWSTR, applicationusermodelidlength: *mut u32, applicationusermodelid: super::super::super::Foundation::PWSTR) -> i32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetApplicationUserModelId(hprocess: super::super::super::Foundation::HANDLE, applicationusermodelidlength: *mut u32, applicationusermodelid: super::super::super::Foundation::PWSTR) -> i32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetApplicationUserModelIdFromToken(token: super::super::super::Foundation::HANDLE, applicationusermodelidlength: *mut u32, applicationusermodelid: super::super::super::Foundation::PWSTR) -> i32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetCurrentApplicationUserModelId(applicationusermodelidlength: *mut u32, applicationusermodelid: super::super::super::Foundation::PWSTR) -> i32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetCurrentPackageFamilyName(packagefamilynamelength: *mut u32, packagefamilyname: super::super::super::Foundation::PWSTR) -> i32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetCurrentPackageFullName(packagefullnamelength: *mut u32, packagefullname: super::super::super::Foundation::PWSTR) -> i32;
    pub fn GetCurrentPackageId(bufferlength: *mut u32, buffer: *mut u8) -> i32;
    pub fn GetCurrentPackageInfo(flags: u32, bufferlength: *mut u32, buffer: *mut u8, count: *mut u32) -> i32;
    pub fn GetCurrentPackageInfo2(flags: u32, packagepathtype: PackagePathType, bufferlength: *mut u32, buffer: *mut u8, count: *mut u32) -> i32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetCurrentPackagePath(pathlength: *mut u32, path: super::super::super::Foundation::PWSTR) -> i32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetCurrentPackagePath2(packagepathtype: PackagePathType, pathlength: *mut u32, path: super::super::super::Foundation::PWSTR) -> i32;
    pub fn GetCurrentPackageVirtualizationContext() -> *mut PACKAGE_VIRTUALIZATION_CONTEXT_HANDLE__;
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetIdForPackageDependencyContext(packagedependencycontext: *const PACKAGEDEPENDENCY_CONTEXT__, packagedependencyid: *mut super::super::super::Foundation::PWSTR) -> ::windows_sys::core::HRESULT;
    pub fn GetPackageApplicationIds(packageinforeference: *const _PACKAGE_INFO_REFERENCE, bufferlength: *mut u32, buffer: *mut u8, count: *mut u32) -> i32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetPackageFamilyName(hprocess: super::super::super::Foundation::HANDLE, packagefamilynamelength: *mut u32, packagefamilyname: super::super::super::Foundation::PWSTR) -> i32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetPackageFamilyNameFromToken(token: super::super::super::Foundation::HANDLE, packagefamilynamelength: *mut u32, packagefamilyname: super::super::super::Foundation::PWSTR) -> i32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetPackageFullName(hprocess: super::super::super::Foundation::HANDLE, packagefullnamelength: *mut u32, packagefullname: super::super::super::Foundation::PWSTR) -> i32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetPackageFullNameFromToken(token: super::super::super::Foundation::HANDLE, packagefullnamelength: *mut u32, packagefullname: super::super::super::Foundation::PWSTR) -> i32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetPackageId(hprocess: super::super::super::Foundation::HANDLE, bufferlength: *mut u32, buffer: *mut u8) -> i32;
    pub fn GetPackageInfo(packageinforeference: *const _PACKAGE_INFO_REFERENCE, flags: u32, bufferlength: *mut u32, buffer: *mut u8, count: *mut u32) -> i32;
    pub fn GetPackageInfo2(packageinforeference: *const _PACKAGE_INFO_REFERENCE, flags: u32, packagepathtype: PackagePathType, bufferlength: *mut u32, buffer: *mut u8, count: *mut u32) -> i32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetPackagePath(packageid: *const PACKAGE_ID, reserved: u32, pathlength: *mut u32, path: super::super::super::Foundation::PWSTR) -> i32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetPackagePathByFullName(packagefullname: super::super::super::Foundation::PWSTR, pathlength: *mut u32, path: super::super::super::Foundation::PWSTR) -> i32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetPackagePathByFullName2(packagefullname: super::super::super::Foundation::PWSTR, packagepathtype: PackagePathType, pathlength: *mut u32, path: super::super::super::Foundation::PWSTR) -> i32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetPackagesByPackageFamily(packagefamilyname: super::super::super::Foundation::PWSTR, count: *mut u32, packagefullnames: *mut super::super::super::Foundation::PWSTR, bufferlength: *mut u32, buffer: super::super::super::Foundation::PWSTR) -> i32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetProcessesInVirtualizationContext(packagefamilyname: super::super::super::Foundation::PWSTR, count: *mut u32, processes: *mut *mut super::super::super::Foundation::HANDLE) -> ::windows_sys::core::HRESULT;
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetResolvedPackageFullNameForPackageDependency(packagedependencyid: super::super::super::Foundation::PWSTR, packagefullname: *mut super::super::super::Foundation::PWSTR) -> ::windows_sys::core::HRESULT;
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetStagedPackageOrigin(packagefullname: super::super::super::Foundation::PWSTR, origin: *mut PackageOrigin) -> i32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetStagedPackagePathByFullName(packagefullname: super::super::super::Foundation::PWSTR, pathlength: *mut u32, path: super::super::super::Foundation::PWSTR) -> i32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetStagedPackagePathByFullName2(packagefullname: super::super::super::Foundation::PWSTR, packagepathtype: PackagePathType, pathlength: *mut u32, path: super::super::super::Foundation::PWSTR) -> i32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn OpenPackageInfoByFullName(packagefullname: super::super::super::Foundation::PWSTR, reserved: u32, packageinforeference: *mut *mut _PACKAGE_INFO_REFERENCE) -> i32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn OpenPackageInfoByFullNameForUser(usersid: super::super::super::Foundation::PSID, packagefullname: super::super::super::Foundation::PWSTR, reserved: u32, packageinforeference: *mut *mut _PACKAGE_INFO_REFERENCE) -> i32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn PackageFamilyNameFromFullName(packagefullname: super::super::super::Foundation::PWSTR, packagefamilynamelength: *mut u32, packagefamilyname: super::super::super::Foundation::PWSTR) -> i32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn PackageFamilyNameFromId(packageid: *const PACKAGE_ID, packagefamilynamelength: *mut u32, packagefamilyname: super::super::super::Foundation::PWSTR) -> i32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn PackageFullNameFromId(packageid: *const PACKAGE_ID, packagefullnamelength: *mut u32, packagefullname: super::super::super::Foundation::PWSTR) -> i32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn PackageIdFromFullName(packagefullname: super::super::super::Foundation::PWSTR, flags: u32, bufferlength: *mut u32, buffer: *mut u8) -> i32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn PackageNameAndPublisherIdFromFamilyName(packagefamilyname: super::super::super::Foundation::PWSTR, packagenamelength: *mut u32, packagename: super::super::super::Foundation::PWSTR, packagepublisheridlength: *mut u32, packagepublisherid: super::super::super::Foundation::PWSTR) -> i32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn ParseApplicationUserModelId(applicationusermodelid: super::super::super::Foundation::PWSTR, packagefamilynamelength: *mut u32, packagefamilyname: super::super::super::Foundation::PWSTR, packagerelativeapplicationidlength: *mut u32, packagerelativeapplicationid: super::super::super::Foundation::PWSTR) -> i32;
    pub fn ReleasePackageVirtualizationContext(context: *const PACKAGE_VIRTUALIZATION_CONTEXT_HANDLE__);
    pub fn RemovePackageDependency(packagedependencycontext: *const PACKAGEDEPENDENCY_CONTEXT__) -> ::windows_sys::core::HRESULT;
    #[cfg(feature = "Win32_Foundation")]
    pub fn TryCreatePackageDependency(user: super::super::super::Foundation::PSID, packagefamilyname: super::super::super::Foundation::PWSTR, minversion: PACKAGE_VERSION, packagedependencyprocessorarchitectures: PackageDependencyProcessorArchitectures, lifetimekind: PackageDependencyLifetimeKind, lifetimeartifact: super::super::super::Foundation::PWSTR, options: CreatePackageDependencyOptions, packagedependencyid: *mut super::super::super::Foundation::PWSTR) -> ::windows_sys::core::HRESULT;
    #[cfg(feature = "Win32_Foundation")]
    pub fn VerifyApplicationUserModelId(applicationusermodelid: super::super::super::Foundation::PWSTR) -> i32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn VerifyPackageFamilyName(packagefamilyname: super::super::super::Foundation::PWSTR) -> i32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn VerifyPackageFullName(packagefullname: super::super::super::Foundation::PWSTR) -> i32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn VerifyPackageId(packageid: *const PACKAGE_ID) -> i32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn VerifyPackageRelativeApplicationId(packagerelativeapplicationid: super::super::super::Foundation::PWSTR) -> i32;
}
#[repr(C)]
pub struct APPX_BUNDLE_FOOTPRINT_FILE_TYPE(i32);
#[repr(C)]
pub struct APPX_BUNDLE_PAYLOAD_PACKAGE_TYPE(i32);
#[repr(C)]
pub struct APPX_CAPABILITIES(i32);
#[repr(C)]
pub struct APPX_CAPABILITY_CLASS_TYPE(i32);
#[repr(C)]
pub struct APPX_COMPRESSION_OPTION(i32);
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct APPX_ENCRYPTED_EXEMPTIONS(i32);
#[repr(C)]
pub struct APPX_ENCRYPTED_PACKAGE_OPTIONS(i32);
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
#[repr(C)]
pub struct APPX_ENCRYPTED_PACKAGE_SETTINGS(i32);
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
#[repr(C)]
pub struct APPX_ENCRYPTED_PACKAGE_SETTINGS2(i32);
#[repr(C)]
pub struct APPX_FOOTPRINT_FILE_TYPE(i32);
#[repr(C)]
pub struct APPX_KEY_INFO(i32);
#[repr(C)]
pub struct APPX_PACKAGE_ARCHITECTURE(i32);
#[repr(C)]
pub struct APPX_PACKAGE_ARCHITECTURE2(i32);
#[repr(C)]
pub struct APPX_PACKAGE_EDITOR_UPDATE_PACKAGE_MANIFEST_OPTIONS(i32);
#[repr(C)]
pub struct APPX_PACKAGE_EDITOR_UPDATE_PACKAGE_OPTION(i32);
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
#[repr(C)]
pub struct APPX_PACKAGE_SETTINGS(i32);
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
#[repr(C)]
pub struct APPX_PACKAGE_WRITER_PAYLOAD_STREAM(i32);
#[repr(C)]
pub struct APPX_PACKAGING_CONTEXT_CHANGE_TYPE(i32);
#[repr(C)]
pub struct AddPackageDependencyOptions(i32);
#[repr(C)]
pub struct AppPolicyClrCompat(i32);
#[repr(C)]
pub struct AppPolicyCreateFileAccess(i32);
#[repr(C)]
pub struct AppPolicyLifecycleManagement(i32);
#[repr(C)]
pub struct AppPolicyMediaFoundationCodecLoading(i32);
#[repr(C)]
pub struct AppPolicyProcessTerminationMethod(i32);
#[repr(C)]
pub struct AppPolicyShowDeveloperDiagnostic(i32);
#[repr(C)]
pub struct AppPolicyThreadInitializationType(i32);
#[repr(C)]
pub struct AppPolicyWindowingModel(i32);
#[repr(C)]
pub struct AppxBundleFactory(i32);
#[repr(C)]
pub struct AppxEncryptionFactory(i32);
#[repr(C)]
pub struct AppxFactory(i32);
#[repr(C)]
pub struct AppxPackageEditor(i32);
#[repr(C)]
pub struct AppxPackagingDiagnosticEventSinkManager(i32);
#[repr(C)]
pub struct CreatePackageDependencyOptions(i32);
#[repr(C)]
pub struct DX_FEATURE_LEVEL(i32);
#[repr(transparent)]
pub struct IAppxBlockMapBlock(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IAppxBlockMapBlocksEnumerator(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IAppxBlockMapFile(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IAppxBlockMapFilesEnumerator(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IAppxBlockMapReader(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IAppxBundleFactory(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IAppxBundleManifestOptionalBundleInfo(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IAppxBundleManifestOptionalBundleInfoEnumerator(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IAppxBundleManifestPackageInfo(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IAppxBundleManifestPackageInfo2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IAppxBundleManifestPackageInfo3(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IAppxBundleManifestPackageInfo4(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IAppxBundleManifestPackageInfoEnumerator(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IAppxBundleManifestReader(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IAppxBundleManifestReader2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IAppxBundleReader(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IAppxBundleWriter(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IAppxBundleWriter2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IAppxBundleWriter3(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IAppxBundleWriter4(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IAppxContentGroup(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IAppxContentGroupFilesEnumerator(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IAppxContentGroupMapReader(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IAppxContentGroupMapWriter(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IAppxContentGroupsEnumerator(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IAppxEncryptedBundleWriter(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IAppxEncryptedBundleWriter2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IAppxEncryptedBundleWriter3(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IAppxEncryptedPackageWriter(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IAppxEncryptedPackageWriter2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IAppxEncryptionFactory(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IAppxEncryptionFactory2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IAppxEncryptionFactory3(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IAppxEncryptionFactory4(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IAppxFactory(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IAppxFactory2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IAppxFile(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IAppxFilesEnumerator(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IAppxManifestApplication(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IAppxManifestApplicationsEnumerator(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IAppxManifestCapabilitiesEnumerator(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IAppxManifestDeviceCapabilitiesEnumerator(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IAppxManifestDriverConstraint(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IAppxManifestDriverConstraintsEnumerator(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IAppxManifestDriverDependenciesEnumerator(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IAppxManifestDriverDependency(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IAppxManifestHostRuntimeDependenciesEnumerator(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IAppxManifestHostRuntimeDependency(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IAppxManifestHostRuntimeDependency2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IAppxManifestMainPackageDependenciesEnumerator(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IAppxManifestMainPackageDependency(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IAppxManifestOSPackageDependenciesEnumerator(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IAppxManifestOSPackageDependency(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IAppxManifestOptionalPackageInfo(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IAppxManifestPackageDependenciesEnumerator(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IAppxManifestPackageDependency(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IAppxManifestPackageDependency2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IAppxManifestPackageDependency3(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IAppxManifestPackageId(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IAppxManifestPackageId2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IAppxManifestProperties(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IAppxManifestQualifiedResource(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IAppxManifestQualifiedResourcesEnumerator(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IAppxManifestReader(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IAppxManifestReader2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IAppxManifestReader3(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IAppxManifestReader4(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IAppxManifestReader5(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IAppxManifestReader6(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IAppxManifestReader7(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IAppxManifestResourcesEnumerator(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IAppxManifestTargetDeviceFamiliesEnumerator(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IAppxManifestTargetDeviceFamily(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IAppxPackageEditor(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IAppxPackageReader(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IAppxPackageWriter(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IAppxPackageWriter2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IAppxPackageWriter3(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IAppxPackagingDiagnosticEventSink(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IAppxPackagingDiagnosticEventSinkManager(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IAppxSourceContentGroupMapReader(pub *mut ::core::ffi::c_void);
#[repr(C)]
pub struct PACKAGEDEPENDENCY_CONTEXT__(i32);
pub const PACKAGE_DEPENDENCY_RANK_DEFAULT: u32 = 0u32;
pub const PACKAGE_FILTER_ALL_LOADED: u32 = 0u32;
pub const PACKAGE_FILTER_BUNDLE: u32 = 128u32;
pub const PACKAGE_FILTER_DIRECT: u32 = 32u32;
pub const PACKAGE_FILTER_DYNAMIC: u32 = 1048576u32;
pub const PACKAGE_FILTER_HEAD: u32 = 16u32;
pub const PACKAGE_FILTER_HOSTRUNTIME: u32 = 2097152u32;
pub const PACKAGE_FILTER_IS_IN_RELATED_SET: u32 = 262144u32;
pub const PACKAGE_FILTER_OPTIONAL: u32 = 131072u32;
pub const PACKAGE_FILTER_RESOURCE: u32 = 64u32;
pub const PACKAGE_FILTER_STATIC: u32 = 524288u32;
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct PACKAGE_ID(i32);
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct PACKAGE_INFO(i32);
pub const PACKAGE_INFORMATION_BASIC: u32 = 0u32;
pub const PACKAGE_INFORMATION_FULL: u32 = 256u32;
pub const PACKAGE_PROPERTY_BUNDLE: u32 = 4u32;
pub const PACKAGE_PROPERTY_DEVELOPMENT_MODE: u32 = 65536u32;
pub const PACKAGE_PROPERTY_DYNAMIC: u32 = 1048576u32;
pub const PACKAGE_PROPERTY_FRAMEWORK: u32 = 1u32;
pub const PACKAGE_PROPERTY_HOSTRUNTIME: u32 = 2097152u32;
pub const PACKAGE_PROPERTY_IS_IN_RELATED_SET: u32 = 262144u32;
pub const PACKAGE_PROPERTY_OPTIONAL: u32 = 8u32;
pub const PACKAGE_PROPERTY_RESOURCE: u32 = 2u32;
pub const PACKAGE_PROPERTY_STATIC: u32 = 524288u32;
#[repr(C)]
pub struct PACKAGE_VERSION(i32);
#[repr(C)]
pub struct PACKAGE_VIRTUALIZATION_CONTEXT_HANDLE__(i32);
#[repr(C)]
pub struct PackageDependencyLifetimeKind(i32);
#[repr(C)]
pub struct PackageDependencyProcessorArchitectures(i32);
#[repr(C)]
pub struct PackageOrigin(i32);
#[repr(C)]
pub struct PackagePathType(i32);
#[repr(C)]
pub struct _PACKAGE_INFO_REFERENCE(i32);

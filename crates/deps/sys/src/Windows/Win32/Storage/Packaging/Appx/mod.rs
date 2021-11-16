#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, clashing_extern_declarations, clippy::all)]
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
pub const APPX_BUNDLE_FOOTPRINT_FILE_TYPE_FIRST: i32 = 0i32;
pub const APPX_BUNDLE_FOOTPRINT_FILE_TYPE_MANIFEST: i32 = 0i32;
pub const APPX_BUNDLE_FOOTPRINT_FILE_TYPE_BLOCKMAP: i32 = 1i32;
pub const APPX_BUNDLE_FOOTPRINT_FILE_TYPE_SIGNATURE: i32 = 2i32;
pub const APPX_BUNDLE_FOOTPRINT_FILE_TYPE_LAST: i32 = 2i32;
pub const APPX_BUNDLE_PAYLOAD_PACKAGE_TYPE_APPLICATION: i32 = 0i32;
pub const APPX_BUNDLE_PAYLOAD_PACKAGE_TYPE_RESOURCE: i32 = 1i32;
pub const APPX_CAPABILITY_INTERNET_CLIENT: u32 = 1u32;
pub const APPX_CAPABILITY_INTERNET_CLIENT_SERVER: u32 = 2u32;
pub const APPX_CAPABILITY_PRIVATE_NETWORK_CLIENT_SERVER: u32 = 4u32;
pub const APPX_CAPABILITY_DOCUMENTS_LIBRARY: u32 = 8u32;
pub const APPX_CAPABILITY_PICTURES_LIBRARY: u32 = 16u32;
pub const APPX_CAPABILITY_VIDEOS_LIBRARY: u32 = 32u32;
pub const APPX_CAPABILITY_MUSIC_LIBRARY: u32 = 64u32;
pub const APPX_CAPABILITY_ENTERPRISE_AUTHENTICATION: u32 = 128u32;
pub const APPX_CAPABILITY_SHARED_USER_CERTIFICATES: u32 = 256u32;
pub const APPX_CAPABILITY_REMOVABLE_STORAGE: u32 = 512u32;
pub const APPX_CAPABILITY_APPOINTMENTS: u32 = 1024u32;
pub const APPX_CAPABILITY_CONTACTS: u32 = 2048u32;
pub const APPX_CAPABILITY_CLASS_DEFAULT: i32 = 0i32;
pub const APPX_CAPABILITY_CLASS_GENERAL: i32 = 1i32;
pub const APPX_CAPABILITY_CLASS_RESTRICTED: i32 = 2i32;
pub const APPX_CAPABILITY_CLASS_WINDOWS: i32 = 4i32;
pub const APPX_CAPABILITY_CLASS_ALL: i32 = 7i32;
pub const APPX_CAPABILITY_CLASS_CUSTOM: i32 = 8i32;
pub const APPX_COMPRESSION_OPTION_NONE: i32 = 0i32;
pub const APPX_COMPRESSION_OPTION_NORMAL: i32 = 1i32;
pub const APPX_COMPRESSION_OPTION_MAXIMUM: i32 = 2i32;
pub const APPX_COMPRESSION_OPTION_FAST: i32 = 3i32;
pub const APPX_COMPRESSION_OPTION_SUPERFAST: i32 = 4i32;
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct APPX_ENCRYPTED_EXEMPTIONS {
    pub count: u32,
    pub plainTextFiles: *mut super::super::super::Foundation::PWSTR,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for APPX_ENCRYPTED_EXEMPTIONS {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for APPX_ENCRYPTED_EXEMPTIONS {
    fn clone(&self) -> Self {
        *self
    }
}
pub const APPX_ENCRYPTED_PACKAGE_OPTION_NONE: u32 = 0u32;
pub const APPX_ENCRYPTED_PACKAGE_OPTION_DIFFUSION: u32 = 1u32;
pub const APPX_ENCRYPTED_PACKAGE_OPTION_PAGE_HASHING: u32 = 2u32;
#[repr(C)]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
pub struct APPX_ENCRYPTED_PACKAGE_SETTINGS {
    pub keyLength: u32,
    pub encryptionAlgorithm: super::super::super::Foundation::PWSTR,
    pub useDiffusion: super::super::super::Foundation::BOOL,
    pub blockMapHashAlgorithm: super::super::super::System::Com::IUri,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl ::core::marker::Copy for APPX_ENCRYPTED_PACKAGE_SETTINGS {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl ::core::clone::Clone for APPX_ENCRYPTED_PACKAGE_SETTINGS {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
pub struct APPX_ENCRYPTED_PACKAGE_SETTINGS2 {
    pub keyLength: u32,
    pub encryptionAlgorithm: super::super::super::Foundation::PWSTR,
    pub blockMapHashAlgorithm: super::super::super::System::Com::IUri,
    pub options: u32,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl ::core::marker::Copy for APPX_ENCRYPTED_PACKAGE_SETTINGS2 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl ::core::clone::Clone for APPX_ENCRYPTED_PACKAGE_SETTINGS2 {
    fn clone(&self) -> Self {
        *self
    }
}
pub const APPX_FOOTPRINT_FILE_TYPE_MANIFEST: i32 = 0i32;
pub const APPX_FOOTPRINT_FILE_TYPE_BLOCKMAP: i32 = 1i32;
pub const APPX_FOOTPRINT_FILE_TYPE_SIGNATURE: i32 = 2i32;
pub const APPX_FOOTPRINT_FILE_TYPE_CODEINTEGRITY: i32 = 3i32;
pub const APPX_FOOTPRINT_FILE_TYPE_CONTENTGROUPMAP: i32 = 4i32;
#[repr(C)]
pub struct APPX_KEY_INFO {
    pub keyLength: u32,
    pub keyIdLength: u32,
    pub key: *mut u8,
    pub keyId: *mut u8,
}
impl ::core::marker::Copy for APPX_KEY_INFO {}
impl ::core::clone::Clone for APPX_KEY_INFO {
    fn clone(&self) -> Self {
        *self
    }
}
pub const APPX_PACKAGE_ARCHITECTURE_X86: i32 = 0i32;
pub const APPX_PACKAGE_ARCHITECTURE_ARM: i32 = 5i32;
pub const APPX_PACKAGE_ARCHITECTURE_X64: i32 = 9i32;
pub const APPX_PACKAGE_ARCHITECTURE_NEUTRAL: i32 = 11i32;
pub const APPX_PACKAGE_ARCHITECTURE_ARM64: i32 = 12i32;
pub const APPX_PACKAGE_ARCHITECTURE2_X86: i32 = 0i32;
pub const APPX_PACKAGE_ARCHITECTURE2_ARM: i32 = 5i32;
pub const APPX_PACKAGE_ARCHITECTURE2_X64: i32 = 9i32;
pub const APPX_PACKAGE_ARCHITECTURE2_NEUTRAL: i32 = 11i32;
pub const APPX_PACKAGE_ARCHITECTURE2_ARM64: i32 = 12i32;
pub const APPX_PACKAGE_ARCHITECTURE2_X86_ON_ARM64: i32 = 14i32;
pub const APPX_PACKAGE_ARCHITECTURE2_UNKNOWN: i32 = 65535i32;
pub const APPX_PACKAGE_EDITOR_UPDATE_PACKAGE_MANIFEST_OPTION_NONE: u32 = 0u32;
pub const APPX_PACKAGE_EDITOR_UPDATE_PACKAGE_MANIFEST_OPTION_SKIP_VALIDATION: u32 = 1u32;
pub const APPX_PACKAGE_EDITOR_UPDATE_PACKAGE_MANIFEST_OPTION_LOCALIZED: u32 = 2u32;
pub const APPX_PACKAGE_EDITOR_UPDATE_PACKAGE_OPTION_APPEND_DELTA: i32 = 0i32;
#[repr(C)]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
pub struct APPX_PACKAGE_SETTINGS {
    pub forceZip32: super::super::super::Foundation::BOOL,
    pub hashMethod: super::super::super::System::Com::IUri,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl ::core::marker::Copy for APPX_PACKAGE_SETTINGS {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl ::core::clone::Clone for APPX_PACKAGE_SETTINGS {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
pub struct APPX_PACKAGE_WRITER_PAYLOAD_STREAM {
    pub inputStream: super::super::super::System::Com::IStream,
    pub fileName: super::super::super::Foundation::PWSTR,
    pub contentType: super::super::super::Foundation::PWSTR,
    pub compressionOption: APPX_COMPRESSION_OPTION,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl ::core::marker::Copy for APPX_PACKAGE_WRITER_PAYLOAD_STREAM {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl ::core::clone::Clone for APPX_PACKAGE_WRITER_PAYLOAD_STREAM {
    fn clone(&self) -> Self {
        *self
    }
}
pub const APPX_PACKAGING_CONTEXT_CHANGE_TYPE_START: i32 = 0i32;
pub const APPX_PACKAGING_CONTEXT_CHANGE_TYPE_CHANGE: i32 = 1i32;
pub const APPX_PACKAGING_CONTEXT_CHANGE_TYPE_DETAILS: i32 = 2i32;
pub const APPX_PACKAGING_CONTEXT_CHANGE_TYPE_END: i32 = 3i32;
pub const AddPackageDependencyOptions_None: i32 = 0i32;
pub const AddPackageDependencyOptions_PrependIfRankCollision: i32 = 1i32;
pub const AppPolicyClrCompat_Other: i32 = 0i32;
pub const AppPolicyClrCompat_ClassicDesktop: i32 = 1i32;
pub const AppPolicyClrCompat_Universal: i32 = 2i32;
pub const AppPolicyClrCompat_PackagedDesktop: i32 = 3i32;
pub const AppPolicyCreateFileAccess_Full: i32 = 0i32;
pub const AppPolicyCreateFileAccess_Limited: i32 = 1i32;
pub const AppPolicyLifecycleManagement_Unmanaged: i32 = 0i32;
pub const AppPolicyLifecycleManagement_Managed: i32 = 1i32;
pub const AppPolicyMediaFoundationCodecLoading_All: i32 = 0i32;
pub const AppPolicyMediaFoundationCodecLoading_InboxOnly: i32 = 1i32;
pub const AppPolicyProcessTerminationMethod_ExitProcess: i32 = 0i32;
pub const AppPolicyProcessTerminationMethod_TerminateProcess: i32 = 1i32;
pub const AppPolicyShowDeveloperDiagnostic_None: i32 = 0i32;
pub const AppPolicyShowDeveloperDiagnostic_ShowUI: i32 = 1i32;
pub const AppPolicyThreadInitializationType_None: i32 = 0i32;
pub const AppPolicyThreadInitializationType_InitializeWinRT: i32 = 1i32;
pub const AppPolicyWindowingModel_None: i32 = 0i32;
pub const AppPolicyWindowingModel_Universal: i32 = 1i32;
pub const AppPolicyWindowingModel_ClassicDesktop: i32 = 2i32;
pub const AppPolicyWindowingModel_ClassicPhone: i32 = 3i32;
pub const AppxBundleFactory: ::windows_sys::core::GUID = ::windows_sys::core::GUID {
    data1: 932054086,
    data2: 21380,
    data3: 17335,
    data4: [136, 119, 231, 219, 221, 136, 52, 70],
};
pub const AppxEncryptionFactory: ::windows_sys::core::GUID = ::windows_sys::core::GUID {
    data1: 3697692637,
    data2: 55400,
    data3: 18158,
    data4: [135, 128, 141, 25, 108, 183, 57, 247],
};
pub const AppxFactory: ::windows_sys::core::GUID = ::windows_sys::core::GUID {
    data1: 1480761664,
    data2: 65439,
    data3: 16742,
    data4: [143, 92, 98, 245, 183, 176, 199, 129],
};
pub const AppxPackageEditor: ::windows_sys::core::GUID = ::windows_sys::core::GUID {
    data1: 4026856138,
    data2: 44732,
    data3: 19213,
    data4: [191, 88, 229, 22, 213, 188, 192, 171],
};
pub const AppxPackagingDiagnosticEventSinkManager: ::windows_sys::core::GUID = ::windows_sys::core::GUID {
    data1: 1355418182,
    data2: 5512,
    data3: 16737,
    data4: [142, 210, 239, 158, 70, 156, 237, 93],
};
pub const CreatePackageDependencyOptions_None: i32 = 0i32;
pub const CreatePackageDependencyOptions_DoNotVerifyDependencyResolution: i32 = 1i32;
pub const CreatePackageDependencyOptions_ScopeIsSystem: i32 = 2i32;
pub const DX_FEATURE_LEVEL_UNSPECIFIED: i32 = 0i32;
pub const DX_FEATURE_LEVEL_9: i32 = 1i32;
pub const DX_FEATURE_LEVEL_10: i32 = 2i32;
pub const DX_FEATURE_LEVEL_11: i32 = 3i32;
#[repr(transparent)]
pub struct IAppxBlockMapBlock(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IAppxBlockMapBlock {}
impl ::core::clone::Clone for IAppxBlockMapBlock {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IAppxBlockMapBlocksEnumerator(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IAppxBlockMapBlocksEnumerator {}
impl ::core::clone::Clone for IAppxBlockMapBlocksEnumerator {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IAppxBlockMapFile(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IAppxBlockMapFile {}
impl ::core::clone::Clone for IAppxBlockMapFile {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IAppxBlockMapFilesEnumerator(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IAppxBlockMapFilesEnumerator {}
impl ::core::clone::Clone for IAppxBlockMapFilesEnumerator {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IAppxBlockMapReader(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IAppxBlockMapReader {}
impl ::core::clone::Clone for IAppxBlockMapReader {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IAppxBundleFactory(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IAppxBundleFactory {}
impl ::core::clone::Clone for IAppxBundleFactory {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IAppxBundleManifestOptionalBundleInfo(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IAppxBundleManifestOptionalBundleInfo {}
impl ::core::clone::Clone for IAppxBundleManifestOptionalBundleInfo {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IAppxBundleManifestOptionalBundleInfoEnumerator(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IAppxBundleManifestOptionalBundleInfoEnumerator {}
impl ::core::clone::Clone for IAppxBundleManifestOptionalBundleInfoEnumerator {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IAppxBundleManifestPackageInfo(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IAppxBundleManifestPackageInfo {}
impl ::core::clone::Clone for IAppxBundleManifestPackageInfo {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IAppxBundleManifestPackageInfo2(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IAppxBundleManifestPackageInfo2 {}
impl ::core::clone::Clone for IAppxBundleManifestPackageInfo2 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IAppxBundleManifestPackageInfo3(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IAppxBundleManifestPackageInfo3 {}
impl ::core::clone::Clone for IAppxBundleManifestPackageInfo3 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IAppxBundleManifestPackageInfo4(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IAppxBundleManifestPackageInfo4 {}
impl ::core::clone::Clone for IAppxBundleManifestPackageInfo4 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IAppxBundleManifestPackageInfoEnumerator(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IAppxBundleManifestPackageInfoEnumerator {}
impl ::core::clone::Clone for IAppxBundleManifestPackageInfoEnumerator {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IAppxBundleManifestReader(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IAppxBundleManifestReader {}
impl ::core::clone::Clone for IAppxBundleManifestReader {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IAppxBundleManifestReader2(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IAppxBundleManifestReader2 {}
impl ::core::clone::Clone for IAppxBundleManifestReader2 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IAppxBundleReader(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IAppxBundleReader {}
impl ::core::clone::Clone for IAppxBundleReader {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IAppxBundleWriter(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IAppxBundleWriter {}
impl ::core::clone::Clone for IAppxBundleWriter {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IAppxBundleWriter2(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IAppxBundleWriter2 {}
impl ::core::clone::Clone for IAppxBundleWriter2 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IAppxBundleWriter3(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IAppxBundleWriter3 {}
impl ::core::clone::Clone for IAppxBundleWriter3 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IAppxBundleWriter4(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IAppxBundleWriter4 {}
impl ::core::clone::Clone for IAppxBundleWriter4 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IAppxContentGroup(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IAppxContentGroup {}
impl ::core::clone::Clone for IAppxContentGroup {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IAppxContentGroupFilesEnumerator(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IAppxContentGroupFilesEnumerator {}
impl ::core::clone::Clone for IAppxContentGroupFilesEnumerator {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IAppxContentGroupMapReader(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IAppxContentGroupMapReader {}
impl ::core::clone::Clone for IAppxContentGroupMapReader {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IAppxContentGroupMapWriter(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IAppxContentGroupMapWriter {}
impl ::core::clone::Clone for IAppxContentGroupMapWriter {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IAppxContentGroupsEnumerator(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IAppxContentGroupsEnumerator {}
impl ::core::clone::Clone for IAppxContentGroupsEnumerator {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IAppxEncryptedBundleWriter(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IAppxEncryptedBundleWriter {}
impl ::core::clone::Clone for IAppxEncryptedBundleWriter {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IAppxEncryptedBundleWriter2(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IAppxEncryptedBundleWriter2 {}
impl ::core::clone::Clone for IAppxEncryptedBundleWriter2 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IAppxEncryptedBundleWriter3(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IAppxEncryptedBundleWriter3 {}
impl ::core::clone::Clone for IAppxEncryptedBundleWriter3 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IAppxEncryptedPackageWriter(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IAppxEncryptedPackageWriter {}
impl ::core::clone::Clone for IAppxEncryptedPackageWriter {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IAppxEncryptedPackageWriter2(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IAppxEncryptedPackageWriter2 {}
impl ::core::clone::Clone for IAppxEncryptedPackageWriter2 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IAppxEncryptionFactory(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IAppxEncryptionFactory {}
impl ::core::clone::Clone for IAppxEncryptionFactory {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IAppxEncryptionFactory2(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IAppxEncryptionFactory2 {}
impl ::core::clone::Clone for IAppxEncryptionFactory2 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IAppxEncryptionFactory3(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IAppxEncryptionFactory3 {}
impl ::core::clone::Clone for IAppxEncryptionFactory3 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IAppxEncryptionFactory4(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IAppxEncryptionFactory4 {}
impl ::core::clone::Clone for IAppxEncryptionFactory4 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IAppxFactory(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IAppxFactory {}
impl ::core::clone::Clone for IAppxFactory {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IAppxFactory2(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IAppxFactory2 {}
impl ::core::clone::Clone for IAppxFactory2 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IAppxFile(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IAppxFile {}
impl ::core::clone::Clone for IAppxFile {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IAppxFilesEnumerator(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IAppxFilesEnumerator {}
impl ::core::clone::Clone for IAppxFilesEnumerator {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IAppxManifestApplication(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IAppxManifestApplication {}
impl ::core::clone::Clone for IAppxManifestApplication {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IAppxManifestApplicationsEnumerator(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IAppxManifestApplicationsEnumerator {}
impl ::core::clone::Clone for IAppxManifestApplicationsEnumerator {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IAppxManifestCapabilitiesEnumerator(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IAppxManifestCapabilitiesEnumerator {}
impl ::core::clone::Clone for IAppxManifestCapabilitiesEnumerator {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IAppxManifestDeviceCapabilitiesEnumerator(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IAppxManifestDeviceCapabilitiesEnumerator {}
impl ::core::clone::Clone for IAppxManifestDeviceCapabilitiesEnumerator {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IAppxManifestDriverConstraint(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IAppxManifestDriverConstraint {}
impl ::core::clone::Clone for IAppxManifestDriverConstraint {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IAppxManifestDriverConstraintsEnumerator(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IAppxManifestDriverConstraintsEnumerator {}
impl ::core::clone::Clone for IAppxManifestDriverConstraintsEnumerator {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IAppxManifestDriverDependenciesEnumerator(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IAppxManifestDriverDependenciesEnumerator {}
impl ::core::clone::Clone for IAppxManifestDriverDependenciesEnumerator {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IAppxManifestDriverDependency(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IAppxManifestDriverDependency {}
impl ::core::clone::Clone for IAppxManifestDriverDependency {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IAppxManifestHostRuntimeDependenciesEnumerator(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IAppxManifestHostRuntimeDependenciesEnumerator {}
impl ::core::clone::Clone for IAppxManifestHostRuntimeDependenciesEnumerator {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IAppxManifestHostRuntimeDependency(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IAppxManifestHostRuntimeDependency {}
impl ::core::clone::Clone for IAppxManifestHostRuntimeDependency {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IAppxManifestHostRuntimeDependency2(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IAppxManifestHostRuntimeDependency2 {}
impl ::core::clone::Clone for IAppxManifestHostRuntimeDependency2 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IAppxManifestMainPackageDependenciesEnumerator(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IAppxManifestMainPackageDependenciesEnumerator {}
impl ::core::clone::Clone for IAppxManifestMainPackageDependenciesEnumerator {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IAppxManifestMainPackageDependency(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IAppxManifestMainPackageDependency {}
impl ::core::clone::Clone for IAppxManifestMainPackageDependency {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IAppxManifestOSPackageDependenciesEnumerator(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IAppxManifestOSPackageDependenciesEnumerator {}
impl ::core::clone::Clone for IAppxManifestOSPackageDependenciesEnumerator {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IAppxManifestOSPackageDependency(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IAppxManifestOSPackageDependency {}
impl ::core::clone::Clone for IAppxManifestOSPackageDependency {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IAppxManifestOptionalPackageInfo(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IAppxManifestOptionalPackageInfo {}
impl ::core::clone::Clone for IAppxManifestOptionalPackageInfo {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IAppxManifestPackageDependenciesEnumerator(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IAppxManifestPackageDependenciesEnumerator {}
impl ::core::clone::Clone for IAppxManifestPackageDependenciesEnumerator {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IAppxManifestPackageDependency(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IAppxManifestPackageDependency {}
impl ::core::clone::Clone for IAppxManifestPackageDependency {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IAppxManifestPackageDependency2(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IAppxManifestPackageDependency2 {}
impl ::core::clone::Clone for IAppxManifestPackageDependency2 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IAppxManifestPackageDependency3(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IAppxManifestPackageDependency3 {}
impl ::core::clone::Clone for IAppxManifestPackageDependency3 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IAppxManifestPackageId(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IAppxManifestPackageId {}
impl ::core::clone::Clone for IAppxManifestPackageId {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IAppxManifestPackageId2(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IAppxManifestPackageId2 {}
impl ::core::clone::Clone for IAppxManifestPackageId2 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IAppxManifestProperties(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IAppxManifestProperties {}
impl ::core::clone::Clone for IAppxManifestProperties {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IAppxManifestQualifiedResource(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IAppxManifestQualifiedResource {}
impl ::core::clone::Clone for IAppxManifestQualifiedResource {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IAppxManifestQualifiedResourcesEnumerator(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IAppxManifestQualifiedResourcesEnumerator {}
impl ::core::clone::Clone for IAppxManifestQualifiedResourcesEnumerator {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IAppxManifestReader(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IAppxManifestReader {}
impl ::core::clone::Clone for IAppxManifestReader {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IAppxManifestReader2(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IAppxManifestReader2 {}
impl ::core::clone::Clone for IAppxManifestReader2 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IAppxManifestReader3(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IAppxManifestReader3 {}
impl ::core::clone::Clone for IAppxManifestReader3 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IAppxManifestReader4(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IAppxManifestReader4 {}
impl ::core::clone::Clone for IAppxManifestReader4 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IAppxManifestReader5(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IAppxManifestReader5 {}
impl ::core::clone::Clone for IAppxManifestReader5 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IAppxManifestReader6(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IAppxManifestReader6 {}
impl ::core::clone::Clone for IAppxManifestReader6 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IAppxManifestReader7(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IAppxManifestReader7 {}
impl ::core::clone::Clone for IAppxManifestReader7 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IAppxManifestResourcesEnumerator(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IAppxManifestResourcesEnumerator {}
impl ::core::clone::Clone for IAppxManifestResourcesEnumerator {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IAppxManifestTargetDeviceFamiliesEnumerator(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IAppxManifestTargetDeviceFamiliesEnumerator {}
impl ::core::clone::Clone for IAppxManifestTargetDeviceFamiliesEnumerator {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IAppxManifestTargetDeviceFamily(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IAppxManifestTargetDeviceFamily {}
impl ::core::clone::Clone for IAppxManifestTargetDeviceFamily {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IAppxPackageEditor(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IAppxPackageEditor {}
impl ::core::clone::Clone for IAppxPackageEditor {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IAppxPackageReader(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IAppxPackageReader {}
impl ::core::clone::Clone for IAppxPackageReader {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IAppxPackageWriter(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IAppxPackageWriter {}
impl ::core::clone::Clone for IAppxPackageWriter {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IAppxPackageWriter2(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IAppxPackageWriter2 {}
impl ::core::clone::Clone for IAppxPackageWriter2 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IAppxPackageWriter3(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IAppxPackageWriter3 {}
impl ::core::clone::Clone for IAppxPackageWriter3 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IAppxPackagingDiagnosticEventSink(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IAppxPackagingDiagnosticEventSink {}
impl ::core::clone::Clone for IAppxPackagingDiagnosticEventSink {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IAppxPackagingDiagnosticEventSinkManager(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IAppxPackagingDiagnosticEventSinkManager {}
impl ::core::clone::Clone for IAppxPackagingDiagnosticEventSinkManager {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IAppxSourceContentGroupMapReader(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IAppxSourceContentGroupMapReader {}
impl ::core::clone::Clone for IAppxSourceContentGroupMapReader {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct PACKAGEDEPENDENCY_CONTEXT__ {
    pub unused: i32,
}
impl ::core::marker::Copy for PACKAGEDEPENDENCY_CONTEXT__ {}
impl ::core::clone::Clone for PACKAGEDEPENDENCY_CONTEXT__ {
    fn clone(&self) -> Self {
        *self
    }
}
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
#[repr(C, packed(4))]
#[cfg(feature = "Win32_Foundation")]
pub struct PACKAGE_ID {
    pub reserved: u32,
    pub processorArchitecture: u32,
    pub version: PACKAGE_VERSION,
    pub name: super::super::super::Foundation::PWSTR,
    pub publisher: super::super::super::Foundation::PWSTR,
    pub resourceId: super::super::super::Foundation::PWSTR,
    pub publisherId: super::super::super::Foundation::PWSTR,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for PACKAGE_ID {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for PACKAGE_ID {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C, packed(4))]
#[cfg(feature = "Win32_Foundation")]
pub struct PACKAGE_INFO {
    pub reserved: u32,
    pub flags: u32,
    pub path: super::super::super::Foundation::PWSTR,
    pub packageFullName: super::super::super::Foundation::PWSTR,
    pub packageFamilyName: super::super::super::Foundation::PWSTR,
    pub packageId: PACKAGE_ID,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for PACKAGE_INFO {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for PACKAGE_INFO {
    fn clone(&self) -> Self {
        *self
    }
}
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
pub struct PACKAGE_VERSION {
    pub Anonymous: PACKAGE_VERSION_0,
}
impl ::core::marker::Copy for PACKAGE_VERSION {}
impl ::core::clone::Clone for PACKAGE_VERSION {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C, packed(4))]
pub union PACKAGE_VERSION_0 {
    pub Version: u64,
    pub Anonymous: PACKAGE_VERSION_0_0,
}
impl ::core::marker::Copy for PACKAGE_VERSION_0 {}
impl ::core::clone::Clone for PACKAGE_VERSION_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct PACKAGE_VERSION_0_0 {
    pub Revision: u16,
    pub Build: u16,
    pub Minor: u16,
    pub Major: u16,
}
impl ::core::marker::Copy for PACKAGE_VERSION_0_0 {}
impl ::core::clone::Clone for PACKAGE_VERSION_0_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
pub struct PACKAGE_VIRTUALIZATION_CONTEXT_HANDLE__ {
    pub unused: i32,
}
impl ::core::marker::Copy for PACKAGE_VIRTUALIZATION_CONTEXT_HANDLE__ {}
impl ::core::clone::Clone for PACKAGE_VIRTUALIZATION_CONTEXT_HANDLE__ {
    fn clone(&self) -> Self {
        *self
    }
}
pub const PackageDependencyLifetimeKind_Process: i32 = 0i32;
pub const PackageDependencyLifetimeKind_FilePath: i32 = 1i32;
pub const PackageDependencyLifetimeKind_RegistryKey: i32 = 2i32;
pub const PackageDependencyProcessorArchitectures_None: i32 = 0i32;
pub const PackageDependencyProcessorArchitectures_Neutral: i32 = 1i32;
pub const PackageDependencyProcessorArchitectures_X86: i32 = 2i32;
pub const PackageDependencyProcessorArchitectures_X64: i32 = 4i32;
pub const PackageDependencyProcessorArchitectures_Arm: i32 = 8i32;
pub const PackageDependencyProcessorArchitectures_Arm64: i32 = 16i32;
pub const PackageDependencyProcessorArchitectures_X86A64: i32 = 32i32;
pub const PackageOrigin_Unknown: i32 = 0i32;
pub const PackageOrigin_Unsigned: i32 = 1i32;
pub const PackageOrigin_Inbox: i32 = 2i32;
pub const PackageOrigin_Store: i32 = 3i32;
pub const PackageOrigin_DeveloperUnsigned: i32 = 4i32;
pub const PackageOrigin_DeveloperSigned: i32 = 5i32;
pub const PackageOrigin_LineOfBusiness: i32 = 6i32;
pub const PackagePathType_Install: i32 = 0i32;
pub const PackagePathType_Mutable: i32 = 1i32;
pub const PackagePathType_Effective: i32 = 2i32;
pub const PackagePathType_MachineExternal: i32 = 3i32;
pub const PackagePathType_UserExternal: i32 = 4i32;
pub const PackagePathType_EffectiveExternal: i32 = 5i32;
#[repr(C)]
pub struct _PACKAGE_INFO_REFERENCE {
    pub reserved: *mut ::core::ffi::c_void,
}
impl ::core::marker::Copy for _PACKAGE_INFO_REFERENCE {}
impl ::core::clone::Clone for _PACKAGE_INFO_REFERENCE {
    fn clone(&self) -> Self {
        *self
    }
}

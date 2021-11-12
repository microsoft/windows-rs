#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals)]
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
#[repr(transparent)]
pub struct APPX_BUNDLE_FOOTPRINT_FILE_TYPE(pub i32);
pub const APPX_BUNDLE_FOOTPRINT_FILE_TYPE_FIRST: APPX_BUNDLE_FOOTPRINT_FILE_TYPE = APPX_BUNDLE_FOOTPRINT_FILE_TYPE(0i32);
pub const APPX_BUNDLE_FOOTPRINT_FILE_TYPE_MANIFEST: APPX_BUNDLE_FOOTPRINT_FILE_TYPE = APPX_BUNDLE_FOOTPRINT_FILE_TYPE(0i32);
pub const APPX_BUNDLE_FOOTPRINT_FILE_TYPE_BLOCKMAP: APPX_BUNDLE_FOOTPRINT_FILE_TYPE = APPX_BUNDLE_FOOTPRINT_FILE_TYPE(1i32);
pub const APPX_BUNDLE_FOOTPRINT_FILE_TYPE_SIGNATURE: APPX_BUNDLE_FOOTPRINT_FILE_TYPE = APPX_BUNDLE_FOOTPRINT_FILE_TYPE(2i32);
pub const APPX_BUNDLE_FOOTPRINT_FILE_TYPE_LAST: APPX_BUNDLE_FOOTPRINT_FILE_TYPE = APPX_BUNDLE_FOOTPRINT_FILE_TYPE(2i32);
impl ::core::marker::Copy for APPX_BUNDLE_FOOTPRINT_FILE_TYPE {}
impl ::core::clone::Clone for APPX_BUNDLE_FOOTPRINT_FILE_TYPE {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct APPX_BUNDLE_PAYLOAD_PACKAGE_TYPE(pub i32);
pub const APPX_BUNDLE_PAYLOAD_PACKAGE_TYPE_APPLICATION: APPX_BUNDLE_PAYLOAD_PACKAGE_TYPE = APPX_BUNDLE_PAYLOAD_PACKAGE_TYPE(0i32);
pub const APPX_BUNDLE_PAYLOAD_PACKAGE_TYPE_RESOURCE: APPX_BUNDLE_PAYLOAD_PACKAGE_TYPE = APPX_BUNDLE_PAYLOAD_PACKAGE_TYPE(1i32);
impl ::core::marker::Copy for APPX_BUNDLE_PAYLOAD_PACKAGE_TYPE {}
impl ::core::clone::Clone for APPX_BUNDLE_PAYLOAD_PACKAGE_TYPE {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct APPX_CAPABILITIES(pub u32);
pub const APPX_CAPABILITY_INTERNET_CLIENT: APPX_CAPABILITIES = APPX_CAPABILITIES(1u32);
pub const APPX_CAPABILITY_INTERNET_CLIENT_SERVER: APPX_CAPABILITIES = APPX_CAPABILITIES(2u32);
pub const APPX_CAPABILITY_PRIVATE_NETWORK_CLIENT_SERVER: APPX_CAPABILITIES = APPX_CAPABILITIES(4u32);
pub const APPX_CAPABILITY_DOCUMENTS_LIBRARY: APPX_CAPABILITIES = APPX_CAPABILITIES(8u32);
pub const APPX_CAPABILITY_PICTURES_LIBRARY: APPX_CAPABILITIES = APPX_CAPABILITIES(16u32);
pub const APPX_CAPABILITY_VIDEOS_LIBRARY: APPX_CAPABILITIES = APPX_CAPABILITIES(32u32);
pub const APPX_CAPABILITY_MUSIC_LIBRARY: APPX_CAPABILITIES = APPX_CAPABILITIES(64u32);
pub const APPX_CAPABILITY_ENTERPRISE_AUTHENTICATION: APPX_CAPABILITIES = APPX_CAPABILITIES(128u32);
pub const APPX_CAPABILITY_SHARED_USER_CERTIFICATES: APPX_CAPABILITIES = APPX_CAPABILITIES(256u32);
pub const APPX_CAPABILITY_REMOVABLE_STORAGE: APPX_CAPABILITIES = APPX_CAPABILITIES(512u32);
pub const APPX_CAPABILITY_APPOINTMENTS: APPX_CAPABILITIES = APPX_CAPABILITIES(1024u32);
pub const APPX_CAPABILITY_CONTACTS: APPX_CAPABILITIES = APPX_CAPABILITIES(2048u32);
impl ::core::marker::Copy for APPX_CAPABILITIES {}
impl ::core::clone::Clone for APPX_CAPABILITIES {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct APPX_CAPABILITY_CLASS_TYPE(pub i32);
pub const APPX_CAPABILITY_CLASS_DEFAULT: APPX_CAPABILITY_CLASS_TYPE = APPX_CAPABILITY_CLASS_TYPE(0i32);
pub const APPX_CAPABILITY_CLASS_GENERAL: APPX_CAPABILITY_CLASS_TYPE = APPX_CAPABILITY_CLASS_TYPE(1i32);
pub const APPX_CAPABILITY_CLASS_RESTRICTED: APPX_CAPABILITY_CLASS_TYPE = APPX_CAPABILITY_CLASS_TYPE(2i32);
pub const APPX_CAPABILITY_CLASS_WINDOWS: APPX_CAPABILITY_CLASS_TYPE = APPX_CAPABILITY_CLASS_TYPE(4i32);
pub const APPX_CAPABILITY_CLASS_ALL: APPX_CAPABILITY_CLASS_TYPE = APPX_CAPABILITY_CLASS_TYPE(7i32);
pub const APPX_CAPABILITY_CLASS_CUSTOM: APPX_CAPABILITY_CLASS_TYPE = APPX_CAPABILITY_CLASS_TYPE(8i32);
impl ::core::marker::Copy for APPX_CAPABILITY_CLASS_TYPE {}
impl ::core::clone::Clone for APPX_CAPABILITY_CLASS_TYPE {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct APPX_COMPRESSION_OPTION(pub i32);
pub const APPX_COMPRESSION_OPTION_NONE: APPX_COMPRESSION_OPTION = APPX_COMPRESSION_OPTION(0i32);
pub const APPX_COMPRESSION_OPTION_NORMAL: APPX_COMPRESSION_OPTION = APPX_COMPRESSION_OPTION(1i32);
pub const APPX_COMPRESSION_OPTION_MAXIMUM: APPX_COMPRESSION_OPTION = APPX_COMPRESSION_OPTION(2i32);
pub const APPX_COMPRESSION_OPTION_FAST: APPX_COMPRESSION_OPTION = APPX_COMPRESSION_OPTION(3i32);
pub const APPX_COMPRESSION_OPTION_SUPERFAST: APPX_COMPRESSION_OPTION = APPX_COMPRESSION_OPTION(4i32);
impl ::core::marker::Copy for APPX_COMPRESSION_OPTION {}
impl ::core::clone::Clone for APPX_COMPRESSION_OPTION {
    fn clone(&self) -> Self {
        *self
    }
}
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
#[repr(transparent)]
pub struct APPX_ENCRYPTED_PACKAGE_OPTIONS(pub u32);
pub const APPX_ENCRYPTED_PACKAGE_OPTION_NONE: APPX_ENCRYPTED_PACKAGE_OPTIONS = APPX_ENCRYPTED_PACKAGE_OPTIONS(0u32);
pub const APPX_ENCRYPTED_PACKAGE_OPTION_DIFFUSION: APPX_ENCRYPTED_PACKAGE_OPTIONS = APPX_ENCRYPTED_PACKAGE_OPTIONS(1u32);
pub const APPX_ENCRYPTED_PACKAGE_OPTION_PAGE_HASHING: APPX_ENCRYPTED_PACKAGE_OPTIONS = APPX_ENCRYPTED_PACKAGE_OPTIONS(2u32);
impl ::core::marker::Copy for APPX_ENCRYPTED_PACKAGE_OPTIONS {}
impl ::core::clone::Clone for APPX_ENCRYPTED_PACKAGE_OPTIONS {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
pub struct APPX_ENCRYPTED_PACKAGE_SETTINGS {
    pub keyLength: u32,
    pub encryptionAlgorithm: super::super::super::Foundation::PWSTR,
    pub useDiffusion: super::super::super::Foundation::BOOL,
    pub blockMapHashAlgorithm: ::core::option::Option<super::super::super::System::Com::IUri>,
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
    pub blockMapHashAlgorithm: ::core::option::Option<super::super::super::System::Com::IUri>,
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
#[repr(transparent)]
pub struct APPX_FOOTPRINT_FILE_TYPE(pub i32);
pub const APPX_FOOTPRINT_FILE_TYPE_MANIFEST: APPX_FOOTPRINT_FILE_TYPE = APPX_FOOTPRINT_FILE_TYPE(0i32);
pub const APPX_FOOTPRINT_FILE_TYPE_BLOCKMAP: APPX_FOOTPRINT_FILE_TYPE = APPX_FOOTPRINT_FILE_TYPE(1i32);
pub const APPX_FOOTPRINT_FILE_TYPE_SIGNATURE: APPX_FOOTPRINT_FILE_TYPE = APPX_FOOTPRINT_FILE_TYPE(2i32);
pub const APPX_FOOTPRINT_FILE_TYPE_CODEINTEGRITY: APPX_FOOTPRINT_FILE_TYPE = APPX_FOOTPRINT_FILE_TYPE(3i32);
pub const APPX_FOOTPRINT_FILE_TYPE_CONTENTGROUPMAP: APPX_FOOTPRINT_FILE_TYPE = APPX_FOOTPRINT_FILE_TYPE(4i32);
impl ::core::marker::Copy for APPX_FOOTPRINT_FILE_TYPE {}
impl ::core::clone::Clone for APPX_FOOTPRINT_FILE_TYPE {
    fn clone(&self) -> Self {
        *self
    }
}
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
#[repr(transparent)]
pub struct APPX_PACKAGE_ARCHITECTURE(pub i32);
pub const APPX_PACKAGE_ARCHITECTURE_X86: APPX_PACKAGE_ARCHITECTURE = APPX_PACKAGE_ARCHITECTURE(0i32);
pub const APPX_PACKAGE_ARCHITECTURE_ARM: APPX_PACKAGE_ARCHITECTURE = APPX_PACKAGE_ARCHITECTURE(5i32);
pub const APPX_PACKAGE_ARCHITECTURE_X64: APPX_PACKAGE_ARCHITECTURE = APPX_PACKAGE_ARCHITECTURE(9i32);
pub const APPX_PACKAGE_ARCHITECTURE_NEUTRAL: APPX_PACKAGE_ARCHITECTURE = APPX_PACKAGE_ARCHITECTURE(11i32);
pub const APPX_PACKAGE_ARCHITECTURE_ARM64: APPX_PACKAGE_ARCHITECTURE = APPX_PACKAGE_ARCHITECTURE(12i32);
impl ::core::marker::Copy for APPX_PACKAGE_ARCHITECTURE {}
impl ::core::clone::Clone for APPX_PACKAGE_ARCHITECTURE {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct APPX_PACKAGE_ARCHITECTURE2(pub i32);
pub const APPX_PACKAGE_ARCHITECTURE2_X86: APPX_PACKAGE_ARCHITECTURE2 = APPX_PACKAGE_ARCHITECTURE2(0i32);
pub const APPX_PACKAGE_ARCHITECTURE2_ARM: APPX_PACKAGE_ARCHITECTURE2 = APPX_PACKAGE_ARCHITECTURE2(5i32);
pub const APPX_PACKAGE_ARCHITECTURE2_X64: APPX_PACKAGE_ARCHITECTURE2 = APPX_PACKAGE_ARCHITECTURE2(9i32);
pub const APPX_PACKAGE_ARCHITECTURE2_NEUTRAL: APPX_PACKAGE_ARCHITECTURE2 = APPX_PACKAGE_ARCHITECTURE2(11i32);
pub const APPX_PACKAGE_ARCHITECTURE2_ARM64: APPX_PACKAGE_ARCHITECTURE2 = APPX_PACKAGE_ARCHITECTURE2(12i32);
pub const APPX_PACKAGE_ARCHITECTURE2_X86_ON_ARM64: APPX_PACKAGE_ARCHITECTURE2 = APPX_PACKAGE_ARCHITECTURE2(14i32);
pub const APPX_PACKAGE_ARCHITECTURE2_UNKNOWN: APPX_PACKAGE_ARCHITECTURE2 = APPX_PACKAGE_ARCHITECTURE2(65535i32);
impl ::core::marker::Copy for APPX_PACKAGE_ARCHITECTURE2 {}
impl ::core::clone::Clone for APPX_PACKAGE_ARCHITECTURE2 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct APPX_PACKAGE_EDITOR_UPDATE_PACKAGE_MANIFEST_OPTIONS(pub u32);
pub const APPX_PACKAGE_EDITOR_UPDATE_PACKAGE_MANIFEST_OPTION_NONE: APPX_PACKAGE_EDITOR_UPDATE_PACKAGE_MANIFEST_OPTIONS = APPX_PACKAGE_EDITOR_UPDATE_PACKAGE_MANIFEST_OPTIONS(0u32);
pub const APPX_PACKAGE_EDITOR_UPDATE_PACKAGE_MANIFEST_OPTION_SKIP_VALIDATION: APPX_PACKAGE_EDITOR_UPDATE_PACKAGE_MANIFEST_OPTIONS = APPX_PACKAGE_EDITOR_UPDATE_PACKAGE_MANIFEST_OPTIONS(1u32);
pub const APPX_PACKAGE_EDITOR_UPDATE_PACKAGE_MANIFEST_OPTION_LOCALIZED: APPX_PACKAGE_EDITOR_UPDATE_PACKAGE_MANIFEST_OPTIONS = APPX_PACKAGE_EDITOR_UPDATE_PACKAGE_MANIFEST_OPTIONS(2u32);
impl ::core::marker::Copy for APPX_PACKAGE_EDITOR_UPDATE_PACKAGE_MANIFEST_OPTIONS {}
impl ::core::clone::Clone for APPX_PACKAGE_EDITOR_UPDATE_PACKAGE_MANIFEST_OPTIONS {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct APPX_PACKAGE_EDITOR_UPDATE_PACKAGE_OPTION(pub i32);
pub const APPX_PACKAGE_EDITOR_UPDATE_PACKAGE_OPTION_APPEND_DELTA: APPX_PACKAGE_EDITOR_UPDATE_PACKAGE_OPTION = APPX_PACKAGE_EDITOR_UPDATE_PACKAGE_OPTION(0i32);
impl ::core::marker::Copy for APPX_PACKAGE_EDITOR_UPDATE_PACKAGE_OPTION {}
impl ::core::clone::Clone for APPX_PACKAGE_EDITOR_UPDATE_PACKAGE_OPTION {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
pub struct APPX_PACKAGE_SETTINGS {
    pub forceZip32: super::super::super::Foundation::BOOL,
    pub hashMethod: ::core::option::Option<super::super::super::System::Com::IUri>,
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
    pub inputStream: ::core::option::Option<super::super::super::System::Com::IStream>,
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
#[repr(transparent)]
pub struct APPX_PACKAGING_CONTEXT_CHANGE_TYPE(pub i32);
pub const APPX_PACKAGING_CONTEXT_CHANGE_TYPE_START: APPX_PACKAGING_CONTEXT_CHANGE_TYPE = APPX_PACKAGING_CONTEXT_CHANGE_TYPE(0i32);
pub const APPX_PACKAGING_CONTEXT_CHANGE_TYPE_CHANGE: APPX_PACKAGING_CONTEXT_CHANGE_TYPE = APPX_PACKAGING_CONTEXT_CHANGE_TYPE(1i32);
pub const APPX_PACKAGING_CONTEXT_CHANGE_TYPE_DETAILS: APPX_PACKAGING_CONTEXT_CHANGE_TYPE = APPX_PACKAGING_CONTEXT_CHANGE_TYPE(2i32);
pub const APPX_PACKAGING_CONTEXT_CHANGE_TYPE_END: APPX_PACKAGING_CONTEXT_CHANGE_TYPE = APPX_PACKAGING_CONTEXT_CHANGE_TYPE(3i32);
impl ::core::marker::Copy for APPX_PACKAGING_CONTEXT_CHANGE_TYPE {}
impl ::core::clone::Clone for APPX_PACKAGING_CONTEXT_CHANGE_TYPE {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct AddPackageDependencyOptions(pub i32);
pub const AddPackageDependencyOptions_None: AddPackageDependencyOptions = AddPackageDependencyOptions(0i32);
pub const AddPackageDependencyOptions_PrependIfRankCollision: AddPackageDependencyOptions = AddPackageDependencyOptions(1i32);
impl ::core::marker::Copy for AddPackageDependencyOptions {}
impl ::core::clone::Clone for AddPackageDependencyOptions {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct AppPolicyClrCompat(pub i32);
pub const AppPolicyClrCompat_Other: AppPolicyClrCompat = AppPolicyClrCompat(0i32);
pub const AppPolicyClrCompat_ClassicDesktop: AppPolicyClrCompat = AppPolicyClrCompat(1i32);
pub const AppPolicyClrCompat_Universal: AppPolicyClrCompat = AppPolicyClrCompat(2i32);
pub const AppPolicyClrCompat_PackagedDesktop: AppPolicyClrCompat = AppPolicyClrCompat(3i32);
impl ::core::marker::Copy for AppPolicyClrCompat {}
impl ::core::clone::Clone for AppPolicyClrCompat {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct AppPolicyCreateFileAccess(pub i32);
pub const AppPolicyCreateFileAccess_Full: AppPolicyCreateFileAccess = AppPolicyCreateFileAccess(0i32);
pub const AppPolicyCreateFileAccess_Limited: AppPolicyCreateFileAccess = AppPolicyCreateFileAccess(1i32);
impl ::core::marker::Copy for AppPolicyCreateFileAccess {}
impl ::core::clone::Clone for AppPolicyCreateFileAccess {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct AppPolicyLifecycleManagement(pub i32);
pub const AppPolicyLifecycleManagement_Unmanaged: AppPolicyLifecycleManagement = AppPolicyLifecycleManagement(0i32);
pub const AppPolicyLifecycleManagement_Managed: AppPolicyLifecycleManagement = AppPolicyLifecycleManagement(1i32);
impl ::core::marker::Copy for AppPolicyLifecycleManagement {}
impl ::core::clone::Clone for AppPolicyLifecycleManagement {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct AppPolicyMediaFoundationCodecLoading(pub i32);
pub const AppPolicyMediaFoundationCodecLoading_All: AppPolicyMediaFoundationCodecLoading = AppPolicyMediaFoundationCodecLoading(0i32);
pub const AppPolicyMediaFoundationCodecLoading_InboxOnly: AppPolicyMediaFoundationCodecLoading = AppPolicyMediaFoundationCodecLoading(1i32);
impl ::core::marker::Copy for AppPolicyMediaFoundationCodecLoading {}
impl ::core::clone::Clone for AppPolicyMediaFoundationCodecLoading {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct AppPolicyProcessTerminationMethod(pub i32);
pub const AppPolicyProcessTerminationMethod_ExitProcess: AppPolicyProcessTerminationMethod = AppPolicyProcessTerminationMethod(0i32);
pub const AppPolicyProcessTerminationMethod_TerminateProcess: AppPolicyProcessTerminationMethod = AppPolicyProcessTerminationMethod(1i32);
impl ::core::marker::Copy for AppPolicyProcessTerminationMethod {}
impl ::core::clone::Clone for AppPolicyProcessTerminationMethod {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct AppPolicyShowDeveloperDiagnostic(pub i32);
pub const AppPolicyShowDeveloperDiagnostic_None: AppPolicyShowDeveloperDiagnostic = AppPolicyShowDeveloperDiagnostic(0i32);
pub const AppPolicyShowDeveloperDiagnostic_ShowUI: AppPolicyShowDeveloperDiagnostic = AppPolicyShowDeveloperDiagnostic(1i32);
impl ::core::marker::Copy for AppPolicyShowDeveloperDiagnostic {}
impl ::core::clone::Clone for AppPolicyShowDeveloperDiagnostic {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct AppPolicyThreadInitializationType(pub i32);
pub const AppPolicyThreadInitializationType_None: AppPolicyThreadInitializationType = AppPolicyThreadInitializationType(0i32);
pub const AppPolicyThreadInitializationType_InitializeWinRT: AppPolicyThreadInitializationType = AppPolicyThreadInitializationType(1i32);
impl ::core::marker::Copy for AppPolicyThreadInitializationType {}
impl ::core::clone::Clone for AppPolicyThreadInitializationType {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct AppPolicyWindowingModel(pub i32);
pub const AppPolicyWindowingModel_None: AppPolicyWindowingModel = AppPolicyWindowingModel(0i32);
pub const AppPolicyWindowingModel_Universal: AppPolicyWindowingModel = AppPolicyWindowingModel(1i32);
pub const AppPolicyWindowingModel_ClassicDesktop: AppPolicyWindowingModel = AppPolicyWindowingModel(2i32);
pub const AppPolicyWindowingModel_ClassicPhone: AppPolicyWindowingModel = AppPolicyWindowingModel(3i32);
impl ::core::marker::Copy for AppPolicyWindowingModel {}
impl ::core::clone::Clone for AppPolicyWindowingModel {
    fn clone(&self) -> Self {
        *self
    }
}
pub const AppxBundleFactory: ::windows_sys::core::GUID = ::windows_sys::GUID {
    data1: 932054086,
    data2: 21380,
    data3: 17335,
    data4: [136, 119, 231, 219, 221, 136, 52, 70],
};
pub const AppxEncryptionFactory: ::windows_sys::core::GUID = ::windows_sys::GUID {
    data1: 3697692637,
    data2: 55400,
    data3: 18158,
    data4: [135, 128, 141, 25, 108, 183, 57, 247],
};
pub const AppxFactory: ::windows_sys::core::GUID = ::windows_sys::GUID {
    data1: 1480761664,
    data2: 65439,
    data3: 16742,
    data4: [143, 92, 98, 245, 183, 176, 199, 129],
};
pub const AppxPackageEditor: ::windows_sys::core::GUID = ::windows_sys::GUID {
    data1: 4026856138,
    data2: 44732,
    data3: 19213,
    data4: [191, 88, 229, 22, 213, 188, 192, 171],
};
pub const AppxPackagingDiagnosticEventSinkManager: ::windows_sys::core::GUID = ::windows_sys::GUID {
    data1: 1355418182,
    data2: 5512,
    data3: 16737,
    data4: [142, 210, 239, 158, 70, 156, 237, 93],
};
#[repr(transparent)]
pub struct CreatePackageDependencyOptions(pub i32);
pub const CreatePackageDependencyOptions_None: CreatePackageDependencyOptions = CreatePackageDependencyOptions(0i32);
pub const CreatePackageDependencyOptions_DoNotVerifyDependencyResolution: CreatePackageDependencyOptions = CreatePackageDependencyOptions(1i32);
pub const CreatePackageDependencyOptions_ScopeIsSystem: CreatePackageDependencyOptions = CreatePackageDependencyOptions(2i32);
impl ::core::marker::Copy for CreatePackageDependencyOptions {}
impl ::core::clone::Clone for CreatePackageDependencyOptions {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct DX_FEATURE_LEVEL(pub i32);
pub const DX_FEATURE_LEVEL_UNSPECIFIED: DX_FEATURE_LEVEL = DX_FEATURE_LEVEL(0i32);
pub const DX_FEATURE_LEVEL_9: DX_FEATURE_LEVEL = DX_FEATURE_LEVEL(1i32);
pub const DX_FEATURE_LEVEL_10: DX_FEATURE_LEVEL = DX_FEATURE_LEVEL(2i32);
pub const DX_FEATURE_LEVEL_11: DX_FEATURE_LEVEL = DX_FEATURE_LEVEL(3i32);
impl ::core::marker::Copy for DX_FEATURE_LEVEL {}
impl ::core::clone::Clone for DX_FEATURE_LEVEL {
    fn clone(&self) -> Self {
        *self
    }
}
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
#[repr(transparent)]
pub struct PackageDependencyLifetimeKind(pub i32);
pub const PackageDependencyLifetimeKind_Process: PackageDependencyLifetimeKind = PackageDependencyLifetimeKind(0i32);
pub const PackageDependencyLifetimeKind_FilePath: PackageDependencyLifetimeKind = PackageDependencyLifetimeKind(1i32);
pub const PackageDependencyLifetimeKind_RegistryKey: PackageDependencyLifetimeKind = PackageDependencyLifetimeKind(2i32);
impl ::core::marker::Copy for PackageDependencyLifetimeKind {}
impl ::core::clone::Clone for PackageDependencyLifetimeKind {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct PackageDependencyProcessorArchitectures(pub i32);
pub const PackageDependencyProcessorArchitectures_None: PackageDependencyProcessorArchitectures = PackageDependencyProcessorArchitectures(0i32);
pub const PackageDependencyProcessorArchitectures_Neutral: PackageDependencyProcessorArchitectures = PackageDependencyProcessorArchitectures(1i32);
pub const PackageDependencyProcessorArchitectures_X86: PackageDependencyProcessorArchitectures = PackageDependencyProcessorArchitectures(2i32);
pub const PackageDependencyProcessorArchitectures_X64: PackageDependencyProcessorArchitectures = PackageDependencyProcessorArchitectures(4i32);
pub const PackageDependencyProcessorArchitectures_Arm: PackageDependencyProcessorArchitectures = PackageDependencyProcessorArchitectures(8i32);
pub const PackageDependencyProcessorArchitectures_Arm64: PackageDependencyProcessorArchitectures = PackageDependencyProcessorArchitectures(16i32);
pub const PackageDependencyProcessorArchitectures_X86A64: PackageDependencyProcessorArchitectures = PackageDependencyProcessorArchitectures(32i32);
impl ::core::marker::Copy for PackageDependencyProcessorArchitectures {}
impl ::core::clone::Clone for PackageDependencyProcessorArchitectures {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct PackageOrigin(pub i32);
pub const PackageOrigin_Unknown: PackageOrigin = PackageOrigin(0i32);
pub const PackageOrigin_Unsigned: PackageOrigin = PackageOrigin(1i32);
pub const PackageOrigin_Inbox: PackageOrigin = PackageOrigin(2i32);
pub const PackageOrigin_Store: PackageOrigin = PackageOrigin(3i32);
pub const PackageOrigin_DeveloperUnsigned: PackageOrigin = PackageOrigin(4i32);
pub const PackageOrigin_DeveloperSigned: PackageOrigin = PackageOrigin(5i32);
pub const PackageOrigin_LineOfBusiness: PackageOrigin = PackageOrigin(6i32);
impl ::core::marker::Copy for PackageOrigin {}
impl ::core::clone::Clone for PackageOrigin {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct PackagePathType(pub i32);
pub const PackagePathType_Install: PackagePathType = PackagePathType(0i32);
pub const PackagePathType_Mutable: PackagePathType = PackagePathType(1i32);
pub const PackagePathType_Effective: PackagePathType = PackagePathType(2i32);
pub const PackagePathType_MachineExternal: PackagePathType = PackagePathType(3i32);
pub const PackagePathType_UserExternal: PackagePathType = PackagePathType(4i32);
pub const PackagePathType_EffectiveExternal: PackagePathType = PackagePathType(5i32);
impl ::core::marker::Copy for PackagePathType {}
impl ::core::clone::Clone for PackagePathType {
    fn clone(&self) -> Self {
        *self
    }
}
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

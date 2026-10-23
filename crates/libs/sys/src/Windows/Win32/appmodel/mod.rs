#[cfg(feature = "shared")]
windows_link::link!("api-ms-win-appmodel-runtime-l1-1-5.dll" "system" fn AddPackageDependency(packagedependencyid : windows_sys::core::PCWSTR, rank : super::INT32, options : AddPackageDependencyOptions, packagedependencycontext : *mut PACKAGEDEPENDENCY_CONTEXT, packagefullname : *mut windows_sys::core::PWSTR) -> windows_sys::core::HRESULT);
#[cfg(feature = "shared")]
windows_link::link!("api-ms-win-appmodel-runtime-l1-1-7.dll" "system" fn AddPackageDependency2(packagedependencyid : windows_sys::core::PCWSTR, rank : super::INT32, options : AddPackageDependencyOptions2, packagedependencycontext : *mut PACKAGEDEPENDENCY_CONTEXT, packagefullname : *mut windows_sys::core::PWSTR) -> windows_sys::core::HRESULT);
#[cfg(all(feature = "shared", feature = "winnt"))]
windows_link::link!("kernel32.dll" "system" fn AppPolicyGetClrCompat(processtoken : super::HANDLE, policy : *mut AppPolicyClrCompat) -> super::LONG);
#[cfg(all(feature = "shared", feature = "winnt"))]
windows_link::link!("kernel32.dll" "system" fn AppPolicyGetCreateFileAccess(processtoken : super::HANDLE, policy : *mut AppPolicyCreateFileAccess) -> super::LONG);
#[cfg(all(feature = "shared", feature = "winnt"))]
windows_link::link!("kernel32.dll" "system" fn AppPolicyGetLifecycleManagement(processtoken : super::HANDLE, policy : *mut AppPolicyLifecycleManagement) -> super::LONG);
#[cfg(all(feature = "shared", feature = "winnt"))]
windows_link::link!("kernel32.dll" "system" fn AppPolicyGetMediaFoundationCodecLoading(processtoken : super::HANDLE, policy : *mut AppPolicyMediaFoundationCodecLoading) -> super::LONG);
#[cfg(all(feature = "shared", feature = "winnt"))]
windows_link::link!("kernel32.dll" "system" fn AppPolicyGetProcessTerminationMethod(processtoken : super::HANDLE, policy : *mut AppPolicyProcessTerminationMethod) -> super::LONG);
#[cfg(all(feature = "shared", feature = "winnt"))]
windows_link::link!("kernel32.dll" "system" fn AppPolicyGetShowDeveloperDiagnostic(processtoken : super::HANDLE, policy : *mut AppPolicyShowDeveloperDiagnostic) -> super::LONG);
#[cfg(all(feature = "shared", feature = "winnt"))]
windows_link::link!("kernel32.dll" "system" fn AppPolicyGetThreadInitializationType(processtoken : super::HANDLE, policy : *mut AppPolicyThreadInitializationType) -> super::LONG);
#[cfg(all(feature = "shared", feature = "winnt"))]
windows_link::link!("kernel32.dll" "system" fn AppPolicyGetWindowingModel(processtoken : super::HANDLE, policy : *mut AppPolicyWindowingModel) -> super::LONG);
windows_link::link!("api-ms-win-appmodel-runtime-l1-1-4.dll" "system" fn CheckIsMSIXPackage(packagefullname : windows_sys::core::PCWSTR, ismsixpackage : *mut windows_sys::core::BOOL) -> windows_sys::core::HRESULT);
#[cfg(feature = "shared")]
windows_link::link!("kernel32.dll" "system" fn ClosePackageInfo(packageinforeference : PACKAGE_INFO_REFERENCE) -> super::LONG);
windows_link::link!("api-ms-win-appmodel-runtime-l1-1-5.dll" "system" fn DeletePackageDependency(packagedependencyid : windows_sys::core::PCWSTR) -> windows_sys::core::HRESULT);
#[cfg(all(feature = "shared", feature = "winnt"))]
windows_link::link!("api-ms-win-appmodel-runtime-l1-1-7.dll" "system" fn FindPackageDependency(findpackagedependencycriteria : *const FindPackageDependencyCriteria, packagedependencyidscount : *mut super::UINT32, packagedependencyids : *mut *mut windows_sys::core::PWSTR) -> windows_sys::core::HRESULT);
#[cfg(all(feature = "shared", feature = "winnt"))]
windows_link::link!("kernel32.dll" "system" fn FindPackagesByPackageFamily(packagefamilyname : windows_sys::core::PCWSTR, packagefilters : super::UINT32, count : *mut super::UINT32, packagefullnames : *mut windows_sys::core::PWSTR, bufferlength : *mut super::UINT32, buffer : *mut super::WCHAR, packageproperties : *mut super::UINT32) -> super::LONG);
#[cfg(feature = "shared")]
windows_link::link!("kernel32.dll" "system" fn FormatApplicationUserModelId(packagefamilyname : windows_sys::core::PCWSTR, packagerelativeapplicationid : windows_sys::core::PCWSTR, applicationusermodelidlength : *mut super::UINT32, applicationusermodelid : windows_sys::core::PWSTR) -> super::LONG);
#[cfg(all(feature = "shared", feature = "winnt"))]
windows_link::link!("kernel32.dll" "system" fn GetApplicationUserModelId(hprocess : super::HANDLE, applicationusermodelidlength : *mut super::UINT32, applicationusermodelid : windows_sys::core::PWSTR) -> super::LONG);
#[cfg(all(feature = "shared", feature = "winnt"))]
windows_link::link!("api-ms-win-appmodel-runtime-l1-1-1.dll" "system" fn GetApplicationUserModelIdFromToken(token : super::HANDLE, applicationusermodelidlength : *mut super::UINT32, applicationusermodelid : windows_sys::core::PWSTR) -> super::LONG);
#[cfg(feature = "shared")]
windows_link::link!("kernel32.dll" "system" fn GetCurrentApplicationUserModelId(applicationusermodelidlength : *mut super::UINT32, applicationusermodelid : windows_sys::core::PWSTR) -> super::LONG);
#[cfg(feature = "shared")]
windows_link::link!("kernel32.dll" "system" fn GetCurrentPackageFamilyName(packagefamilynamelength : *mut super::UINT32, packagefamilyname : windows_sys::core::PWSTR) -> super::LONG);
#[cfg(feature = "shared")]
windows_link::link!("kernel32.dll" "system" fn GetCurrentPackageFullName(packagefullnamelength : *mut super::UINT32, packagefullname : windows_sys::core::PWSTR) -> super::LONG);
#[cfg(feature = "shared")]
windows_link::link!("kernel32.dll" "system" fn GetCurrentPackageId(bufferlength : *mut super::UINT32, buffer : *mut super::BYTE) -> super::LONG);
#[cfg(feature = "shared")]
windows_link::link!("kernel32.dll" "system" fn GetCurrentPackageInfo(flags : super::UINT32, bufferlength : *mut super::UINT32, buffer : *mut super::BYTE, count : *mut super::UINT32) -> super::LONG);
#[cfg(feature = "shared")]
windows_link::link!("api-ms-win-appmodel-runtime-l1-1-3.dll" "system" fn GetCurrentPackageInfo2(flags : super::UINT32, packagepathtype : PackagePathType, bufferlength : *mut super::UINT32, buffer : *mut super::BYTE, count : *mut super::UINT32) -> super::LONG);
#[cfg(feature = "shared")]
windows_link::link!("kernel32.dll" "system" fn GetCurrentPackagePath(pathlength : *mut super::UINT32, path : windows_sys::core::PWSTR) -> super::LONG);
#[cfg(feature = "shared")]
windows_link::link!("api-ms-win-appmodel-runtime-l1-1-3.dll" "system" fn GetCurrentPackagePath2(packagepathtype : PackagePathType, pathlength : *mut super::UINT32, path : windows_sys::core::PWSTR) -> super::LONG);
windows_link::link!("api-ms-win-appmodel-runtime-l1-1-5.dll" "system" fn GetIdForPackageDependencyContext(packagedependencycontext : PACKAGEDEPENDENCY_CONTEXT, packagedependencyid : *mut windows_sys::core::PWSTR) -> windows_sys::core::HRESULT);
#[cfg(feature = "shared")]
windows_link::link!("kernel32.dll" "system" fn GetPackageApplicationIds(packageinforeference : PACKAGE_INFO_REFERENCE, bufferlength : *mut super::UINT32, buffer : *mut super::BYTE, count : *mut super::UINT32) -> super::LONG);
#[cfg(all(feature = "minwindef", feature = "shared", feature = "winnt"))]
windows_link::link!("api-ms-win-appmodel-runtime-l1-1-7.dll" "system" fn GetPackageDependencyInformation(packagedependencyid : windows_sys::core::PCWSTR, user : *mut super::PSID, packagefamilyname : *mut windows_sys::core::PWSTR, minversion : *mut PACKAGE_VERSION, packagedependencyprocessorarchitectures : *mut PackageDependencyProcessorArchitectures, lifetimekind : *mut PackageDependencyLifetimeKind, lifetimeartifact : *mut windows_sys::core::PWSTR, options : *mut CreatePackageDependencyOptions, lifetimeexpiration : *mut super::FILETIME) -> windows_sys::core::HRESULT);
#[cfg(all(feature = "shared", feature = "winnt"))]
windows_link::link!("kernel32.dll" "system" fn GetPackageFamilyName(hprocess : super::HANDLE, packagefamilynamelength : *mut super::UINT32, packagefamilyname : windows_sys::core::PWSTR) -> super::LONG);
#[cfg(all(feature = "shared", feature = "winnt"))]
windows_link::link!("api-ms-win-appmodel-runtime-l1-1-1.dll" "system" fn GetPackageFamilyNameFromToken(token : super::HANDLE, packagefamilynamelength : *mut super::UINT32, packagefamilyname : windows_sys::core::PWSTR) -> super::LONG);
#[cfg(all(feature = "shared", feature = "winnt"))]
windows_link::link!("kernel32.dll" "system" fn GetPackageFullName(hprocess : super::HANDLE, packagefullnamelength : *mut super::UINT32, packagefullname : windows_sys::core::PWSTR) -> super::LONG);
#[cfg(all(feature = "shared", feature = "winnt"))]
windows_link::link!("api-ms-win-appmodel-runtime-l1-1-1.dll" "system" fn GetPackageFullNameFromToken(token : super::HANDLE, packagefullnamelength : *mut super::UINT32, packagefullname : windows_sys::core::PWSTR) -> super::LONG);
#[cfg(feature = "shared")]
windows_link::link!("api-ms-win-appmodel-runtime-l1-1-6.dll" "system" fn GetPackageGraphRevisionId() -> super::UINT32);
#[cfg(all(feature = "shared", feature = "winnt"))]
windows_link::link!("kernel32.dll" "system" fn GetPackageId(hprocess : super::HANDLE, bufferlength : *mut super::UINT32, buffer : *mut super::BYTE) -> super::LONG);
#[cfg(feature = "shared")]
windows_link::link!("kernel32.dll" "system" fn GetPackageInfo(packageinforeference : PACKAGE_INFO_REFERENCE, flags : super::UINT32, bufferlength : *mut super::UINT32, buffer : *mut super::BYTE, count : *mut super::UINT32) -> super::LONG);
#[cfg(feature = "shared")]
windows_link::link!("api-ms-win-appmodel-runtime-l1-1-3.dll" "system" fn GetPackageInfo2(packageinforeference : PACKAGE_INFO_REFERENCE, flags : super::UINT32, packagepathtype : PackagePathType, bufferlength : *mut super::UINT32, buffer : *mut super::BYTE, count : *mut super::UINT32) -> super::LONG);
#[cfg(feature = "shared")]
windows_link::link!("kernel32.dll" "system" fn GetPackagePath(packageid : *const PACKAGE_ID, reserved : super::UINT32, pathlength : *mut super::UINT32, path : windows_sys::core::PWSTR) -> super::LONG);
#[cfg(feature = "shared")]
windows_link::link!("kernel32.dll" "system" fn GetPackagePathByFullName(packagefullname : windows_sys::core::PCWSTR, pathlength : *mut super::UINT32, path : windows_sys::core::PWSTR) -> super::LONG);
#[cfg(feature = "shared")]
windows_link::link!("api-ms-win-appmodel-runtime-l1-1-3.dll" "system" fn GetPackagePathByFullName2(packagefullname : windows_sys::core::PCWSTR, packagepathtype : PackagePathType, pathlength : *mut super::UINT32, path : windows_sys::core::PWSTR) -> super::LONG);
#[cfg(all(feature = "shared", feature = "winnt"))]
windows_link::link!("kernel32.dll" "system" fn GetPackagesByPackageFamily(packagefamilyname : windows_sys::core::PCWSTR, count : *mut super::UINT32, packagefullnames : *mut windows_sys::core::PWSTR, bufferlength : *mut super::UINT32, buffer : *mut super::WCHAR) -> super::LONG);
#[cfg(all(feature = "shared", feature = "winnt"))]
windows_link::link!("api-ms-win-appmodel-runtime-l1-1-7.dll" "system" fn GetProcessesUsingPackageDependency(packagedependencyid : windows_sys::core::PCWSTR, user : super::PSID, scopeissystem : windows_sys::core::BOOL, processidscount : *mut super::UINT32, processids : *mut *mut super::DWORD) -> windows_sys::core::HRESULT);
windows_link::link!("api-ms-win-appmodel-runtime-l1-1-5.dll" "system" fn GetResolvedPackageFullNameForPackageDependency(packagedependencyid : windows_sys::core::PCWSTR, packagefullname : *mut windows_sys::core::PWSTR) -> windows_sys::core::HRESULT);
windows_link::link!("api-ms-win-appmodel-runtime-l1-1-7.dll" "system" fn GetResolvedPackageFullNameForPackageDependency2(packagedependencyid : windows_sys::core::PCWSTR, packagefullname : *mut windows_sys::core::PWSTR) -> windows_sys::core::HRESULT);
#[cfg(feature = "shared")]
windows_link::link!("api-ms-win-appmodel-runtime-l1-1-1.dll" "system" fn GetStagedPackageOrigin(packagefullname : windows_sys::core::PCWSTR, origin : *mut PackageOrigin) -> super::LONG);
windows_link::link!("api-ms-win-appmodel-runtime-l1-1-7.dll" "system" fn GetStagedPackageOrigin2(packagefullname : windows_sys::core::PCWSTR, origin : *mut PackageOrigin2) -> windows_sys::core::HRESULT);
#[cfg(feature = "shared")]
windows_link::link!("kernel32.dll" "system" fn GetStagedPackagePathByFullName(packagefullname : windows_sys::core::PCWSTR, pathlength : *mut super::UINT32, path : windows_sys::core::PWSTR) -> super::LONG);
#[cfg(feature = "shared")]
windows_link::link!("api-ms-win-appmodel-runtime-l1-1-3.dll" "system" fn GetStagedPackagePathByFullName2(packagefullname : windows_sys::core::PCWSTR, packagepathtype : PackagePathType, pathlength : *mut super::UINT32, path : windows_sys::core::PWSTR) -> super::LONG);
windows_link::link!("api-ms-win-appmodel-runtime-l1-1-7.dll" "system" fn IsPackageFeatureSupported(feature : AppModelPackageFeature) -> windows_sys::core::BOOL);
#[cfg(feature = "shared")]
windows_link::link!("kernel32.dll" "system" fn OpenPackageInfoByFullName(packagefullname : windows_sys::core::PCWSTR, reserved : super::UINT32, packageinforeference : *mut PACKAGE_INFO_REFERENCE) -> super::LONG);
#[cfg(all(feature = "shared", feature = "winnt"))]
windows_link::link!("api-ms-win-appmodel-runtime-l1-1-1.dll" "system" fn OpenPackageInfoByFullNameForUser(usersid : super::PSID, packagefullname : windows_sys::core::PCWSTR, reserved : super::UINT32, packageinforeference : *mut PACKAGE_INFO_REFERENCE) -> super::LONG);
#[cfg(feature = "shared")]
windows_link::link!("kernel32.dll" "system" fn PackageFamilyNameFromFullName(packagefullname : windows_sys::core::PCWSTR, packagefamilynamelength : *mut super::UINT32, packagefamilyname : windows_sys::core::PWSTR) -> super::LONG);
#[cfg(feature = "shared")]
windows_link::link!("kernel32.dll" "system" fn PackageFamilyNameFromId(packageid : *const PACKAGE_ID, packagefamilynamelength : *mut super::UINT32, packagefamilyname : windows_sys::core::PWSTR) -> super::LONG);
#[cfg(feature = "shared")]
windows_link::link!("kernel32.dll" "system" fn PackageFullNameFromId(packageid : *const PACKAGE_ID, packagefullnamelength : *mut super::UINT32, packagefullname : windows_sys::core::PWSTR) -> super::LONG);
#[cfg(feature = "shared")]
windows_link::link!("kernel32.dll" "system" fn PackageIdFromFullName(packagefullname : windows_sys::core::PCWSTR, flags : super::UINT32, bufferlength : *mut super::UINT32, buffer : *mut super::BYTE) -> super::LONG);
#[cfg(feature = "shared")]
windows_link::link!("kernel32.dll" "system" fn PackageNameAndPublisherIdFromFamilyName(packagefamilyname : windows_sys::core::PCWSTR, packagenamelength : *mut super::UINT32, packagename : windows_sys::core::PWSTR, packagepublisheridlength : *mut super::UINT32, packagepublisherid : windows_sys::core::PWSTR) -> super::LONG);
#[cfg(feature = "shared")]
windows_link::link!("kernel32.dll" "system" fn ParseApplicationUserModelId(applicationusermodelid : windows_sys::core::PCWSTR, packagefamilynamelength : *mut super::UINT32, packagefamilyname : windows_sys::core::PWSTR, packagerelativeapplicationidlength : *mut super::UINT32, packagerelativeapplicationid : windows_sys::core::PWSTR) -> super::LONG);
windows_link::link!("api-ms-win-appmodel-runtime-l1-1-5.dll" "system" fn RemovePackageDependency(packagedependencycontext : PACKAGEDEPENDENCY_CONTEXT) -> windows_sys::core::HRESULT);
#[cfg(all(feature = "shared", feature = "winnt"))]
windows_link::link!("api-ms-win-appmodel-runtime-l1-1-5.dll" "system" fn TryCreatePackageDependency(user : super::PSID, packagefamilyname : windows_sys::core::PCWSTR, minversion : PACKAGE_VERSION, packagedependencyprocessorarchitectures : PackageDependencyProcessorArchitectures, lifetimekind : PackageDependencyLifetimeKind, lifetimeartifact : windows_sys::core::PCWSTR, options : CreatePackageDependencyOptions, packagedependencyid : *mut windows_sys::core::PWSTR) -> windows_sys::core::HRESULT);
#[cfg(all(feature = "minwindef", feature = "shared", feature = "winnt"))]
windows_link::link!("api-ms-win-appmodel-runtime-l1-1-7.dll" "system" fn TryCreatePackageDependency2(user : super::PSID, packagefamilyname : windows_sys::core::PCWSTR, minversion : PACKAGE_VERSION, packagedependencyprocessorarchitectures : PackageDependencyProcessorArchitectures, lifetimekind : PackageDependencyLifetimeKind, lifetimeartifact : windows_sys::core::PCWSTR, options : CreatePackageDependencyOptions, lifetimeexpiration : *const super::FILETIME, packagedependencyid : *mut windows_sys::core::PWSTR) -> windows_sys::core::HRESULT);
#[cfg(feature = "shared")]
windows_link::link!("api-ms-win-appmodel-runtime-l1-1-1.dll" "system" fn VerifyApplicationUserModelId(applicationusermodelid : windows_sys::core::PCWSTR) -> super::LONG);
#[cfg(feature = "shared")]
windows_link::link!("api-ms-win-appmodel-runtime-l1-1-1.dll" "system" fn VerifyPackageFamilyName(packagefamilyname : windows_sys::core::PCWSTR) -> super::LONG);
#[cfg(feature = "shared")]
windows_link::link!("api-ms-win-appmodel-runtime-l1-1-1.dll" "system" fn VerifyPackageFullName(packagefullname : windows_sys::core::PCWSTR) -> super::LONG);
#[cfg(feature = "shared")]
windows_link::link!("api-ms-win-appmodel-runtime-l1-1-1.dll" "system" fn VerifyPackageId(packageid : *const PACKAGE_ID) -> super::LONG);
#[cfg(feature = "shared")]
windows_link::link!("api-ms-win-appmodel-runtime-l1-1-1.dll" "system" fn VerifyPackageRelativeApplicationId(packagerelativeapplicationid : windows_sys::core::PCWSTR) -> super::LONG);
pub type AddPackageDependencyOptions = i32;
pub type AddPackageDependencyOptions2 = i32;
pub const AddPackageDependencyOptions2_None: AddPackageDependencyOptions2 = 0;
pub const AddPackageDependencyOptions2_PrependIfRankCollision: AddPackageDependencyOptions2 = 1;
pub const AddPackageDependencyOptions2_SpecifiedPackageFamilyOnly: AddPackageDependencyOptions2 = 2;
pub const AddPackageDependencyOptions_None: AddPackageDependencyOptions = 0;
pub const AddPackageDependencyOptions_PrependIfRankCollision: AddPackageDependencyOptions = 1;
pub type AppModelPackageFeature = i32;
pub const AppModelPackageFeature_SignedSBOM: AppModelPackageFeature = 1;
pub type AppPolicyClrCompat = i32;
pub const AppPolicyClrCompat_ClassicDesktop: AppPolicyClrCompat = 1;
pub const AppPolicyClrCompat_Other: AppPolicyClrCompat = 0;
pub const AppPolicyClrCompat_PackagedDesktop: AppPolicyClrCompat = 3;
pub const AppPolicyClrCompat_Universal: AppPolicyClrCompat = 2;
pub type AppPolicyCreateFileAccess = i32;
pub const AppPolicyCreateFileAccess_Full: AppPolicyCreateFileAccess = 0;
pub const AppPolicyCreateFileAccess_Limited: AppPolicyCreateFileAccess = 1;
pub type AppPolicyLifecycleManagement = i32;
pub const AppPolicyLifecycleManagement_Managed: AppPolicyLifecycleManagement = 1;
pub const AppPolicyLifecycleManagement_Unmanaged: AppPolicyLifecycleManagement = 0;
pub type AppPolicyMediaFoundationCodecLoading = i32;
pub const AppPolicyMediaFoundationCodecLoading_All: AppPolicyMediaFoundationCodecLoading = 0;
pub const AppPolicyMediaFoundationCodecLoading_InboxOnly: AppPolicyMediaFoundationCodecLoading = 1;
pub type AppPolicyProcessTerminationMethod = i32;
pub const AppPolicyProcessTerminationMethod_ExitProcess: AppPolicyProcessTerminationMethod = 0;
pub const AppPolicyProcessTerminationMethod_TerminateProcess: AppPolicyProcessTerminationMethod = 1;
pub type AppPolicyShowDeveloperDiagnostic = i32;
pub const AppPolicyShowDeveloperDiagnostic_None: AppPolicyShowDeveloperDiagnostic = 0;
pub const AppPolicyShowDeveloperDiagnostic_ShowUI: AppPolicyShowDeveloperDiagnostic = 1;
pub type AppPolicyThreadInitializationType = i32;
pub const AppPolicyThreadInitializationType_InitializeWinRT: AppPolicyThreadInitializationType = 1;
pub const AppPolicyThreadInitializationType_None: AppPolicyThreadInitializationType = 0;
pub type AppPolicyWindowingModel = i32;
pub const AppPolicyWindowingModel_ClassicDesktop: AppPolicyWindowingModel = 2;
pub const AppPolicyWindowingModel_ClassicPhone: AppPolicyWindowingModel = 3;
pub const AppPolicyWindowingModel_None: AppPolicyWindowingModel = 0;
pub const AppPolicyWindowingModel_Universal: AppPolicyWindowingModel = 1;
pub type CreatePackageDependencyOptions = i32;
pub const CreatePackageDependencyOptions_DoNotVerifyDependencyResolution: CreatePackageDependencyOptions = 1;
pub const CreatePackageDependencyOptions_None: CreatePackageDependencyOptions = 0;
pub const CreatePackageDependencyOptions_ScopeIsSystem: CreatePackageDependencyOptions = 2;
#[repr(C)]
#[cfg(feature = "winnt")]
#[derive(Clone, Copy, Default)]
pub struct FindPackageDependencyCriteria {
    pub User: super::PSID,
    pub ScopeIsSystem: windows_sys::core::BOOL,
    pub PackageFamilyName: windows_sys::core::PCWSTR,
}
pub type PACKAGEDEPENDENCY_CONTEXT = *mut PACKAGEDEPENDENCY_CONTEXT__;
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct PACKAGEDEPENDENCY_CONTEXT__ {
    pub unused: i32,
}
pub const PACKAGE_DEPENDENCY_RANK_DEFAULT: i32 = 0;
pub const PACKAGE_FILTER_ALL_LOADED: i32 = 0;
pub const PACKAGE_FILTER_BUNDLE: i32 = 128;
pub const PACKAGE_FILTER_DIRECT: i32 = 32;
pub const PACKAGE_FILTER_DYNAMIC: i32 = 1048576;
pub const PACKAGE_FILTER_HEAD: i32 = 16;
pub const PACKAGE_FILTER_HOSTRUNTIME: i32 = 2097152;
pub const PACKAGE_FILTER_IS_IN_RELATED_SET: i32 = 262144;
pub const PACKAGE_FILTER_OPTIONAL: i32 = 131072;
pub const PACKAGE_FILTER_RESOURCE: i32 = 64;
pub const PACKAGE_FILTER_STATIC: i32 = 524288;
#[repr(C, packed(4))]
#[cfg(feature = "shared")]
#[derive(Clone, Copy)]
pub struct PACKAGE_ID {
    pub reserved: super::UINT32,
    pub processorArchitecture: super::UINT32,
    pub version: PACKAGE_VERSION,
    pub name: windows_sys::core::PWSTR,
    pub publisher: windows_sys::core::PWSTR,
    pub resourceId: windows_sys::core::PWSTR,
    pub publisherId: windows_sys::core::PWSTR,
}
#[cfg(feature = "shared")]
impl Default for PACKAGE_ID {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(4))]
#[cfg(feature = "shared")]
#[derive(Clone, Copy)]
pub struct PACKAGE_INFO {
    pub reserved: super::UINT32,
    pub flags: super::UINT32,
    pub path: windows_sys::core::PWSTR,
    pub packageFullName: windows_sys::core::PWSTR,
    pub packageFamilyName: windows_sys::core::PWSTR,
    pub packageId: PACKAGE_ID,
}
#[cfg(feature = "shared")]
impl Default for PACKAGE_INFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const PACKAGE_INFORMATION_BASIC: i32 = 0;
pub const PACKAGE_INFORMATION_FULL: i32 = 256;
pub type PACKAGE_INFO_REFERENCE = *mut _PACKAGE_INFO_REFERENCE;
pub const PACKAGE_PROPERTY_BUNDLE: i32 = 4;
pub const PACKAGE_PROPERTY_DEVELOPMENT_MODE: i32 = 65536;
pub const PACKAGE_PROPERTY_DYNAMIC: i32 = 1048576;
pub const PACKAGE_PROPERTY_FRAMEWORK: i32 = 1;
pub const PACKAGE_PROPERTY_HOSTRUNTIME: i32 = 2097152;
pub const PACKAGE_PROPERTY_IS_IN_RELATED_SET: i32 = 262144;
pub const PACKAGE_PROPERTY_OPTIONAL: i32 = 8;
pub const PACKAGE_PROPERTY_RESOURCE: i32 = 2;
pub const PACKAGE_PROPERTY_STATIC: i32 = 524288;
#[repr(C, packed(4))]
#[cfg(feature = "shared")]
#[derive(Clone, Copy)]
pub struct PACKAGE_VERSION {
    pub Anonymous: PACKAGE_VERSION_0,
}
#[cfg(feature = "shared")]
impl Default for PACKAGE_VERSION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(4))]
#[cfg(feature = "shared")]
#[derive(Clone, Copy)]
pub union PACKAGE_VERSION_0 {
    pub Version: super::UINT64,
    pub Anonymous: PACKAGE_VERSION_0_0,
}
#[cfg(feature = "shared")]
impl Default for PACKAGE_VERSION_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(4))]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Default)]
pub struct PACKAGE_VERSION_0_0 {
    pub Revision: super::USHORT,
    pub Build: super::USHORT,
    pub Minor: super::USHORT,
    pub Major: super::USHORT,
}
pub type PackageDependencyLifetimeKind = i32;
pub const PackageDependencyLifetimeKind_FilePath: PackageDependencyLifetimeKind = 1;
pub const PackageDependencyLifetimeKind_Process: PackageDependencyLifetimeKind = 0;
pub const PackageDependencyLifetimeKind_RegistryKey: PackageDependencyLifetimeKind = 2;
pub type PackageDependencyProcessorArchitectures = i32;
pub const PackageDependencyProcessorArchitectures_Arm: PackageDependencyProcessorArchitectures = 8;
pub const PackageDependencyProcessorArchitectures_Arm64: PackageDependencyProcessorArchitectures = 16;
pub const PackageDependencyProcessorArchitectures_Neutral: PackageDependencyProcessorArchitectures = 1;
pub const PackageDependencyProcessorArchitectures_None: PackageDependencyProcessorArchitectures = 0;
pub const PackageDependencyProcessorArchitectures_X64: PackageDependencyProcessorArchitectures = 4;
pub const PackageDependencyProcessorArchitectures_X86: PackageDependencyProcessorArchitectures = 2;
pub const PackageDependencyProcessorArchitectures_X86A64: PackageDependencyProcessorArchitectures = 32;
pub type PackageOrigin = i32;
pub type PackageOrigin2 = i32;
pub const PackageOrigin2_DeveloperSigned: PackageOrigin2 = 5;
pub const PackageOrigin2_DeveloperUnsigned: PackageOrigin2 = 4;
pub const PackageOrigin2_Inbox: PackageOrigin2 = 2;
pub const PackageOrigin2_LineOfBusiness: PackageOrigin2 = 6;
pub const PackageOrigin2_SignedSBOM: PackageOrigin2 = 7;
pub const PackageOrigin2_Store: PackageOrigin2 = 3;
pub const PackageOrigin2_Unknown: PackageOrigin2 = 0;
pub const PackageOrigin2_Unsigned: PackageOrigin2 = 1;
pub const PackageOrigin_DeveloperSigned: PackageOrigin = 5;
pub const PackageOrigin_DeveloperUnsigned: PackageOrigin = 4;
pub const PackageOrigin_Inbox: PackageOrigin = 2;
pub const PackageOrigin_LineOfBusiness: PackageOrigin = 6;
pub const PackageOrigin_SignedSBOM: PackageOrigin = 7;
pub const PackageOrigin_Store: PackageOrigin = 3;
pub const PackageOrigin_Unknown: PackageOrigin = 0;
pub const PackageOrigin_Unsigned: PackageOrigin = 1;
pub type PackagePathType = i32;
pub const PackagePathType_Effective: PackagePathType = 2;
pub const PackagePathType_EffectiveExternal: PackagePathType = 5;
pub const PackagePathType_Install: PackagePathType = 0;
pub const PackagePathType_MachineExternal: PackagePathType = 3;
pub const PackagePathType_Mutable: PackagePathType = 1;
pub const PackagePathType_UserExternal: PackagePathType = 4;
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct _PACKAGE_INFO_REFERENCE {
    pub reserved: *mut core::ffi::c_void,
}

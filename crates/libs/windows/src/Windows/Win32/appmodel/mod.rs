#[cfg(feature = "shared")]
#[inline]
pub unsafe fn AddPackageDependency<P0>(packagedependencyid: P0, rank: super::INT32, options: AddPackageDependencyOptions, packagedependencycontext: *mut PACKAGEDEPENDENCY_CONTEXT, packagefullname: Option<*mut windows_core::PWSTR>) -> windows_core::HRESULT
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("api-ms-win-appmodel-runtime-l1-1-5.dll" "system" fn AddPackageDependency(packagedependencyid : windows_core::PCWSTR, rank : super::INT32, options : AddPackageDependencyOptions, packagedependencycontext : *mut PACKAGEDEPENDENCY_CONTEXT, packagefullname : *mut windows_core::PWSTR) -> windows_core::HRESULT);
    unsafe { AddPackageDependency(packagedependencyid.param().abi(), rank, options, packagedependencycontext as _, packagefullname.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(feature = "shared")]
#[inline]
pub unsafe fn AddPackageDependency2<P0>(packagedependencyid: P0, rank: super::INT32, options: AddPackageDependencyOptions2, packagedependencycontext: *mut PACKAGEDEPENDENCY_CONTEXT, packagefullname: Option<*mut windows_core::PWSTR>) -> windows_core::HRESULT
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("api-ms-win-appmodel-runtime-l1-1-7.dll" "system" fn AddPackageDependency2(packagedependencyid : windows_core::PCWSTR, rank : super::INT32, options : AddPackageDependencyOptions2, packagedependencycontext : *mut PACKAGEDEPENDENCY_CONTEXT, packagefullname : *mut windows_core::PWSTR) -> windows_core::HRESULT);
    unsafe { AddPackageDependency2(packagedependencyid.param().abi(), rank, options, packagedependencycontext as _, packagefullname.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(all(feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn AppPolicyGetClrCompat(processtoken: super::HANDLE, policy: *mut AppPolicyClrCompat) -> super::LONG {
    windows_core::link!("kernel32.dll" "system" fn AppPolicyGetClrCompat(processtoken : super::HANDLE, policy : *mut AppPolicyClrCompat) -> super::LONG);
    unsafe { AppPolicyGetClrCompat(processtoken, policy as _) }
}
#[cfg(all(feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn AppPolicyGetCreateFileAccess(processtoken: super::HANDLE, policy: *mut AppPolicyCreateFileAccess) -> super::LONG {
    windows_core::link!("kernel32.dll" "system" fn AppPolicyGetCreateFileAccess(processtoken : super::HANDLE, policy : *mut AppPolicyCreateFileAccess) -> super::LONG);
    unsafe { AppPolicyGetCreateFileAccess(processtoken, policy as _) }
}
#[cfg(all(feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn AppPolicyGetLifecycleManagement(processtoken: super::HANDLE, policy: *mut AppPolicyLifecycleManagement) -> super::LONG {
    windows_core::link!("kernel32.dll" "system" fn AppPolicyGetLifecycleManagement(processtoken : super::HANDLE, policy : *mut AppPolicyLifecycleManagement) -> super::LONG);
    unsafe { AppPolicyGetLifecycleManagement(processtoken, policy as _) }
}
#[cfg(all(feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn AppPolicyGetMediaFoundationCodecLoading(processtoken: super::HANDLE, policy: *mut AppPolicyMediaFoundationCodecLoading) -> super::LONG {
    windows_core::link!("kernel32.dll" "system" fn AppPolicyGetMediaFoundationCodecLoading(processtoken : super::HANDLE, policy : *mut AppPolicyMediaFoundationCodecLoading) -> super::LONG);
    unsafe { AppPolicyGetMediaFoundationCodecLoading(processtoken, policy as _) }
}
#[cfg(all(feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn AppPolicyGetProcessTerminationMethod(processtoken: super::HANDLE, policy: *mut AppPolicyProcessTerminationMethod) -> super::LONG {
    windows_core::link!("kernel32.dll" "system" fn AppPolicyGetProcessTerminationMethod(processtoken : super::HANDLE, policy : *mut AppPolicyProcessTerminationMethod) -> super::LONG);
    unsafe { AppPolicyGetProcessTerminationMethod(processtoken, policy as _) }
}
#[cfg(all(feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn AppPolicyGetShowDeveloperDiagnostic(processtoken: super::HANDLE, policy: *mut AppPolicyShowDeveloperDiagnostic) -> super::LONG {
    windows_core::link!("kernel32.dll" "system" fn AppPolicyGetShowDeveloperDiagnostic(processtoken : super::HANDLE, policy : *mut AppPolicyShowDeveloperDiagnostic) -> super::LONG);
    unsafe { AppPolicyGetShowDeveloperDiagnostic(processtoken, policy as _) }
}
#[cfg(all(feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn AppPolicyGetThreadInitializationType(processtoken: super::HANDLE, policy: *mut AppPolicyThreadInitializationType) -> super::LONG {
    windows_core::link!("kernel32.dll" "system" fn AppPolicyGetThreadInitializationType(processtoken : super::HANDLE, policy : *mut AppPolicyThreadInitializationType) -> super::LONG);
    unsafe { AppPolicyGetThreadInitializationType(processtoken, policy as _) }
}
#[cfg(all(feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn AppPolicyGetWindowingModel(processtoken: super::HANDLE, policy: *mut AppPolicyWindowingModel) -> super::LONG {
    windows_core::link!("kernel32.dll" "system" fn AppPolicyGetWindowingModel(processtoken : super::HANDLE, policy : *mut AppPolicyWindowingModel) -> super::LONG);
    unsafe { AppPolicyGetWindowingModel(processtoken, policy as _) }
}
#[inline]
pub unsafe fn CheckIsMSIXPackage<P0>(packagefullname: P0) -> windows_core::Result<windows_core::BOOL>
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("api-ms-win-appmodel-runtime-l1-1-4.dll" "system" fn CheckIsMSIXPackage(packagefullname : windows_core::PCWSTR, ismsixpackage : *mut windows_core::BOOL) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        CheckIsMSIXPackage(packagefullname.param().abi(), &mut result__).map(|| result__)
    }
}
#[cfg(feature = "shared")]
#[inline]
pub unsafe fn ClosePackageInfo(packageinforeference: PACKAGE_INFO_REFERENCE) -> super::LONG {
    windows_core::link!("kernel32.dll" "system" fn ClosePackageInfo(packageinforeference : PACKAGE_INFO_REFERENCE) -> super::LONG);
    unsafe { ClosePackageInfo(packageinforeference) }
}
#[inline]
pub unsafe fn DeletePackageDependency<P0>(packagedependencyid: P0) -> windows_core::HRESULT
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("api-ms-win-appmodel-runtime-l1-1-5.dll" "system" fn DeletePackageDependency(packagedependencyid : windows_core::PCWSTR) -> windows_core::HRESULT);
    unsafe { DeletePackageDependency(packagedependencyid.param().abi()) }
}
#[cfg(all(feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn FindPackageDependency(findpackagedependencycriteria: *const FindPackageDependencyCriteria, packagedependencyidscount: *mut super::UINT32, packagedependencyids: *mut *mut windows_core::PWSTR) -> windows_core::HRESULT {
    windows_core::link!("api-ms-win-appmodel-runtime-l1-1-7.dll" "system" fn FindPackageDependency(findpackagedependencycriteria : *const FindPackageDependencyCriteria, packagedependencyidscount : *mut super::UINT32, packagedependencyids : *mut *mut windows_core::PWSTR) -> windows_core::HRESULT);
    unsafe { FindPackageDependency(findpackagedependencycriteria, packagedependencyidscount as _, packagedependencyids as _) }
}
#[cfg(all(feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn FindPackagesByPackageFamily<P0>(packagefamilyname: P0, packagefilters: super::UINT32, count: *mut super::UINT32, packagefullnames: Option<*mut windows_core::PWSTR>, bufferlength: *mut super::UINT32, buffer: Option<*mut super::WCHAR>, packageproperties: Option<*mut super::UINT32>) -> super::LONG
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("kernel32.dll" "system" fn FindPackagesByPackageFamily(packagefamilyname : windows_core::PCWSTR, packagefilters : super::UINT32, count : *mut super::UINT32, packagefullnames : *mut windows_core::PWSTR, bufferlength : *mut super::UINT32, buffer : *mut super::WCHAR, packageproperties : *mut super::UINT32) -> super::LONG);
    unsafe { FindPackagesByPackageFamily(packagefamilyname.param().abi(), packagefilters, count as _, packagefullnames.unwrap_or(core::mem::zeroed()) as _, bufferlength as _, buffer.unwrap_or(core::mem::zeroed()) as _, packageproperties.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(feature = "shared")]
#[inline]
pub unsafe fn FormatApplicationUserModelId<P0, P1>(packagefamilyname: P0, packagerelativeapplicationid: P1, applicationusermodelidlength: *mut super::UINT32, applicationusermodelid: Option<windows_core::PWSTR>) -> super::LONG
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("kernel32.dll" "system" fn FormatApplicationUserModelId(packagefamilyname : windows_core::PCWSTR, packagerelativeapplicationid : windows_core::PCWSTR, applicationusermodelidlength : *mut super::UINT32, applicationusermodelid : windows_core::PWSTR) -> super::LONG);
    unsafe { FormatApplicationUserModelId(packagefamilyname.param().abi(), packagerelativeapplicationid.param().abi(), applicationusermodelidlength as _, applicationusermodelid.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(all(feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn GetApplicationUserModelId(hprocess: super::HANDLE, applicationusermodelidlength: *mut super::UINT32, applicationusermodelid: Option<windows_core::PWSTR>) -> super::LONG {
    windows_core::link!("kernel32.dll" "system" fn GetApplicationUserModelId(hprocess : super::HANDLE, applicationusermodelidlength : *mut super::UINT32, applicationusermodelid : windows_core::PWSTR) -> super::LONG);
    unsafe { GetApplicationUserModelId(hprocess, applicationusermodelidlength as _, applicationusermodelid.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(all(feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn GetApplicationUserModelIdFromToken(token: super::HANDLE, applicationusermodelidlength: *mut super::UINT32, applicationusermodelid: Option<windows_core::PWSTR>) -> super::LONG {
    windows_core::link!("api-ms-win-appmodel-runtime-l1-1-1.dll" "system" fn GetApplicationUserModelIdFromToken(token : super::HANDLE, applicationusermodelidlength : *mut super::UINT32, applicationusermodelid : windows_core::PWSTR) -> super::LONG);
    unsafe { GetApplicationUserModelIdFromToken(token, applicationusermodelidlength as _, applicationusermodelid.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(feature = "shared")]
#[inline]
pub unsafe fn GetCurrentApplicationUserModelId(applicationusermodelidlength: *mut super::UINT32, applicationusermodelid: Option<windows_core::PWSTR>) -> super::LONG {
    windows_core::link!("kernel32.dll" "system" fn GetCurrentApplicationUserModelId(applicationusermodelidlength : *mut super::UINT32, applicationusermodelid : windows_core::PWSTR) -> super::LONG);
    unsafe { GetCurrentApplicationUserModelId(applicationusermodelidlength as _, applicationusermodelid.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(feature = "shared")]
#[inline]
pub unsafe fn GetCurrentPackageFamilyName(packagefamilynamelength: *mut super::UINT32, packagefamilyname: Option<windows_core::PWSTR>) -> super::LONG {
    windows_core::link!("kernel32.dll" "system" fn GetCurrentPackageFamilyName(packagefamilynamelength : *mut super::UINT32, packagefamilyname : windows_core::PWSTR) -> super::LONG);
    unsafe { GetCurrentPackageFamilyName(packagefamilynamelength as _, packagefamilyname.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(feature = "shared")]
#[inline]
pub unsafe fn GetCurrentPackageFullName(packagefullnamelength: *mut super::UINT32, packagefullname: Option<windows_core::PWSTR>) -> super::LONG {
    windows_core::link!("kernel32.dll" "system" fn GetCurrentPackageFullName(packagefullnamelength : *mut super::UINT32, packagefullname : windows_core::PWSTR) -> super::LONG);
    unsafe { GetCurrentPackageFullName(packagefullnamelength as _, packagefullname.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(feature = "shared")]
#[inline]
pub unsafe fn GetCurrentPackageId(bufferlength: *mut super::UINT32, buffer: Option<*mut super::BYTE>) -> super::LONG {
    windows_core::link!("kernel32.dll" "system" fn GetCurrentPackageId(bufferlength : *mut super::UINT32, buffer : *mut super::BYTE) -> super::LONG);
    unsafe { GetCurrentPackageId(bufferlength as _, buffer.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(feature = "shared")]
#[inline]
pub unsafe fn GetCurrentPackageInfo(flags: super::UINT32, bufferlength: *mut super::UINT32, buffer: Option<*mut super::BYTE>, count: Option<*mut super::UINT32>) -> super::LONG {
    windows_core::link!("kernel32.dll" "system" fn GetCurrentPackageInfo(flags : super::UINT32, bufferlength : *mut super::UINT32, buffer : *mut super::BYTE, count : *mut super::UINT32) -> super::LONG);
    unsafe { GetCurrentPackageInfo(flags, bufferlength as _, buffer.unwrap_or(core::mem::zeroed()) as _, count.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(feature = "shared")]
#[inline]
pub unsafe fn GetCurrentPackageInfo2(flags: super::UINT32, packagepathtype: PackagePathType, bufferlength: *mut super::UINT32, buffer: Option<*mut super::BYTE>, count: Option<*mut super::UINT32>) -> super::LONG {
    windows_core::link!("api-ms-win-appmodel-runtime-l1-1-3.dll" "system" fn GetCurrentPackageInfo2(flags : super::UINT32, packagepathtype : PackagePathType, bufferlength : *mut super::UINT32, buffer : *mut super::BYTE, count : *mut super::UINT32) -> super::LONG);
    unsafe { GetCurrentPackageInfo2(flags, packagepathtype, bufferlength as _, buffer.unwrap_or(core::mem::zeroed()) as _, count.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(feature = "shared")]
#[inline]
pub unsafe fn GetCurrentPackagePath(pathlength: *mut super::UINT32, path: Option<windows_core::PWSTR>) -> super::LONG {
    windows_core::link!("kernel32.dll" "system" fn GetCurrentPackagePath(pathlength : *mut super::UINT32, path : windows_core::PWSTR) -> super::LONG);
    unsafe { GetCurrentPackagePath(pathlength as _, path.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(feature = "shared")]
#[inline]
pub unsafe fn GetCurrentPackagePath2(packagepathtype: PackagePathType, pathlength: *mut super::UINT32, path: Option<windows_core::PWSTR>) -> super::LONG {
    windows_core::link!("api-ms-win-appmodel-runtime-l1-1-3.dll" "system" fn GetCurrentPackagePath2(packagepathtype : PackagePathType, pathlength : *mut super::UINT32, path : windows_core::PWSTR) -> super::LONG);
    unsafe { GetCurrentPackagePath2(packagepathtype, pathlength as _, path.unwrap_or(core::mem::zeroed()) as _) }
}
#[inline]
pub unsafe fn GetIdForPackageDependencyContext(packagedependencycontext: PACKAGEDEPENDENCY_CONTEXT) -> windows_core::Result<windows_core::PWSTR> {
    windows_core::link!("api-ms-win-appmodel-runtime-l1-1-5.dll" "system" fn GetIdForPackageDependencyContext(packagedependencycontext : PACKAGEDEPENDENCY_CONTEXT, packagedependencyid : *mut windows_core::PWSTR) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        GetIdForPackageDependencyContext(packagedependencycontext, &mut result__).map(|| result__)
    }
}
#[cfg(feature = "shared")]
#[inline]
pub unsafe fn GetPackageApplicationIds(packageinforeference: PACKAGE_INFO_REFERENCE, bufferlength: *mut super::UINT32, buffer: Option<*mut super::BYTE>, count: Option<*mut super::UINT32>) -> super::LONG {
    windows_core::link!("kernel32.dll" "system" fn GetPackageApplicationIds(packageinforeference : PACKAGE_INFO_REFERENCE, bufferlength : *mut super::UINT32, buffer : *mut super::BYTE, count : *mut super::UINT32) -> super::LONG);
    unsafe { GetPackageApplicationIds(packageinforeference, bufferlength as _, buffer.unwrap_or(core::mem::zeroed()) as _, count.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(all(feature = "minwindef", feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn GetPackageDependencyInformation<P0>(packagedependencyid: P0, user: Option<*mut super::PSID>, packagefamilyname: Option<*mut windows_core::PWSTR>, minversion: Option<*mut PACKAGE_VERSION>, packagedependencyprocessorarchitectures: Option<*mut PackageDependencyProcessorArchitectures>, lifetimekind: Option<*mut PackageDependencyLifetimeKind>, lifetimeartifact: Option<*mut windows_core::PWSTR>, options: Option<*mut CreatePackageDependencyOptions>, lifetimeexpiration: Option<*mut super::FILETIME>) -> windows_core::HRESULT
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("api-ms-win-appmodel-runtime-l1-1-7.dll" "system" fn GetPackageDependencyInformation(packagedependencyid : windows_core::PCWSTR, user : *mut super::PSID, packagefamilyname : *mut windows_core::PWSTR, minversion : *mut PACKAGE_VERSION, packagedependencyprocessorarchitectures : *mut PackageDependencyProcessorArchitectures, lifetimekind : *mut PackageDependencyLifetimeKind, lifetimeartifact : *mut windows_core::PWSTR, options : *mut CreatePackageDependencyOptions, lifetimeexpiration : *mut super::FILETIME) -> windows_core::HRESULT);
    unsafe { GetPackageDependencyInformation(packagedependencyid.param().abi(), user.unwrap_or(core::mem::zeroed()) as _, packagefamilyname.unwrap_or(core::mem::zeroed()) as _, minversion.unwrap_or(core::mem::zeroed()) as _, packagedependencyprocessorarchitectures.unwrap_or(core::mem::zeroed()) as _, lifetimekind.unwrap_or(core::mem::zeroed()) as _, lifetimeartifact.unwrap_or(core::mem::zeroed()) as _, options.unwrap_or(core::mem::zeroed()) as _, lifetimeexpiration.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(all(feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn GetPackageFamilyName(hprocess: super::HANDLE, packagefamilynamelength: *mut super::UINT32, packagefamilyname: Option<windows_core::PWSTR>) -> super::LONG {
    windows_core::link!("kernel32.dll" "system" fn GetPackageFamilyName(hprocess : super::HANDLE, packagefamilynamelength : *mut super::UINT32, packagefamilyname : windows_core::PWSTR) -> super::LONG);
    unsafe { GetPackageFamilyName(hprocess, packagefamilynamelength as _, packagefamilyname.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(all(feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn GetPackageFamilyNameFromToken(token: super::HANDLE, packagefamilynamelength: *mut super::UINT32, packagefamilyname: Option<windows_core::PWSTR>) -> super::LONG {
    windows_core::link!("api-ms-win-appmodel-runtime-l1-1-1.dll" "system" fn GetPackageFamilyNameFromToken(token : super::HANDLE, packagefamilynamelength : *mut super::UINT32, packagefamilyname : windows_core::PWSTR) -> super::LONG);
    unsafe { GetPackageFamilyNameFromToken(token, packagefamilynamelength as _, packagefamilyname.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(all(feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn GetPackageFullName(hprocess: super::HANDLE, packagefullnamelength: *mut super::UINT32, packagefullname: Option<windows_core::PWSTR>) -> super::LONG {
    windows_core::link!("kernel32.dll" "system" fn GetPackageFullName(hprocess : super::HANDLE, packagefullnamelength : *mut super::UINT32, packagefullname : windows_core::PWSTR) -> super::LONG);
    unsafe { GetPackageFullName(hprocess, packagefullnamelength as _, packagefullname.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(all(feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn GetPackageFullNameFromToken(token: super::HANDLE, packagefullnamelength: *mut super::UINT32, packagefullname: Option<windows_core::PWSTR>) -> super::LONG {
    windows_core::link!("api-ms-win-appmodel-runtime-l1-1-1.dll" "system" fn GetPackageFullNameFromToken(token : super::HANDLE, packagefullnamelength : *mut super::UINT32, packagefullname : windows_core::PWSTR) -> super::LONG);
    unsafe { GetPackageFullNameFromToken(token, packagefullnamelength as _, packagefullname.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(feature = "shared")]
#[inline]
pub unsafe fn GetPackageGraphRevisionId() -> super::UINT32 {
    windows_core::link!("api-ms-win-appmodel-runtime-l1-1-6.dll" "system" fn GetPackageGraphRevisionId() -> super::UINT32);
    unsafe { GetPackageGraphRevisionId() }
}
#[cfg(all(feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn GetPackageId(hprocess: super::HANDLE, bufferlength: *mut super::UINT32, buffer: Option<*mut super::BYTE>) -> super::LONG {
    windows_core::link!("kernel32.dll" "system" fn GetPackageId(hprocess : super::HANDLE, bufferlength : *mut super::UINT32, buffer : *mut super::BYTE) -> super::LONG);
    unsafe { GetPackageId(hprocess, bufferlength as _, buffer.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(feature = "shared")]
#[inline]
pub unsafe fn GetPackageInfo(packageinforeference: PACKAGE_INFO_REFERENCE, flags: super::UINT32, bufferlength: *mut super::UINT32, buffer: Option<*mut super::BYTE>, count: Option<*mut super::UINT32>) -> super::LONG {
    windows_core::link!("kernel32.dll" "system" fn GetPackageInfo(packageinforeference : PACKAGE_INFO_REFERENCE, flags : super::UINT32, bufferlength : *mut super::UINT32, buffer : *mut super::BYTE, count : *mut super::UINT32) -> super::LONG);
    unsafe { GetPackageInfo(packageinforeference, flags, bufferlength as _, buffer.unwrap_or(core::mem::zeroed()) as _, count.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(feature = "shared")]
#[inline]
pub unsafe fn GetPackageInfo2(packageinforeference: PACKAGE_INFO_REFERENCE, flags: super::UINT32, packagepathtype: PackagePathType, bufferlength: *mut super::UINT32, buffer: Option<*mut super::BYTE>, count: Option<*mut super::UINT32>) -> super::LONG {
    windows_core::link!("api-ms-win-appmodel-runtime-l1-1-3.dll" "system" fn GetPackageInfo2(packageinforeference : PACKAGE_INFO_REFERENCE, flags : super::UINT32, packagepathtype : PackagePathType, bufferlength : *mut super::UINT32, buffer : *mut super::BYTE, count : *mut super::UINT32) -> super::LONG);
    unsafe { GetPackageInfo2(packageinforeference, flags, packagepathtype, bufferlength as _, buffer.unwrap_or(core::mem::zeroed()) as _, count.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(feature = "shared")]
#[inline]
pub unsafe fn GetPackagePath(packageid: *const PACKAGE_ID, reserved: Option<super::UINT32>, pathlength: *mut super::UINT32, path: Option<windows_core::PWSTR>) -> super::LONG {
    windows_core::link!("kernel32.dll" "system" fn GetPackagePath(packageid : *const PACKAGE_ID, reserved : super::UINT32, pathlength : *mut super::UINT32, path : windows_core::PWSTR) -> super::LONG);
    unsafe { GetPackagePath(packageid, reserved.unwrap_or(core::mem::zeroed()) as _, pathlength as _, path.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(feature = "shared")]
#[inline]
pub unsafe fn GetPackagePathByFullName<P0>(packagefullname: P0, pathlength: *mut super::UINT32, path: Option<windows_core::PWSTR>) -> super::LONG
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("kernel32.dll" "system" fn GetPackagePathByFullName(packagefullname : windows_core::PCWSTR, pathlength : *mut super::UINT32, path : windows_core::PWSTR) -> super::LONG);
    unsafe { GetPackagePathByFullName(packagefullname.param().abi(), pathlength as _, path.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(feature = "shared")]
#[inline]
pub unsafe fn GetPackagePathByFullName2<P0>(packagefullname: P0, packagepathtype: PackagePathType, pathlength: *mut super::UINT32, path: Option<windows_core::PWSTR>) -> super::LONG
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("api-ms-win-appmodel-runtime-l1-1-3.dll" "system" fn GetPackagePathByFullName2(packagefullname : windows_core::PCWSTR, packagepathtype : PackagePathType, pathlength : *mut super::UINT32, path : windows_core::PWSTR) -> super::LONG);
    unsafe { GetPackagePathByFullName2(packagefullname.param().abi(), packagepathtype, pathlength as _, path.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(all(feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn GetPackagesByPackageFamily<P0>(packagefamilyname: P0, count: *mut super::UINT32, packagefullnames: Option<*mut windows_core::PWSTR>, bufferlength: *mut super::UINT32, buffer: Option<*mut super::WCHAR>) -> super::LONG
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("kernel32.dll" "system" fn GetPackagesByPackageFamily(packagefamilyname : windows_core::PCWSTR, count : *mut super::UINT32, packagefullnames : *mut windows_core::PWSTR, bufferlength : *mut super::UINT32, buffer : *mut super::WCHAR) -> super::LONG);
    unsafe { GetPackagesByPackageFamily(packagefamilyname.param().abi(), count as _, packagefullnames.unwrap_or(core::mem::zeroed()) as _, bufferlength as _, buffer.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(all(feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn GetProcessesUsingPackageDependency<P0>(packagedependencyid: P0, user: Option<super::PSID>, scopeissystem: bool, processidscount: *mut super::UINT32, processids: *mut *mut super::DWORD) -> windows_core::HRESULT
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("api-ms-win-appmodel-runtime-l1-1-7.dll" "system" fn GetProcessesUsingPackageDependency(packagedependencyid : windows_core::PCWSTR, user : super::PSID, scopeissystem : windows_core::BOOL, processidscount : *mut super::UINT32, processids : *mut *mut super::DWORD) -> windows_core::HRESULT);
    unsafe { GetProcessesUsingPackageDependency(packagedependencyid.param().abi(), user.unwrap_or(core::mem::zeroed()) as _, scopeissystem.into(), processidscount as _, processids as _) }
}
#[inline]
pub unsafe fn GetResolvedPackageFullNameForPackageDependency<P0>(packagedependencyid: P0) -> windows_core::Result<windows_core::PWSTR>
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("api-ms-win-appmodel-runtime-l1-1-5.dll" "system" fn GetResolvedPackageFullNameForPackageDependency(packagedependencyid : windows_core::PCWSTR, packagefullname : *mut windows_core::PWSTR) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        GetResolvedPackageFullNameForPackageDependency(packagedependencyid.param().abi(), &mut result__).map(|| result__)
    }
}
#[inline]
pub unsafe fn GetResolvedPackageFullNameForPackageDependency2<P0>(packagedependencyid: P0) -> windows_core::Result<windows_core::PWSTR>
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("api-ms-win-appmodel-runtime-l1-1-7.dll" "system" fn GetResolvedPackageFullNameForPackageDependency2(packagedependencyid : windows_core::PCWSTR, packagefullname : *mut windows_core::PWSTR) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        GetResolvedPackageFullNameForPackageDependency2(packagedependencyid.param().abi(), &mut result__).map(|| result__)
    }
}
#[cfg(feature = "shared")]
#[inline]
pub unsafe fn GetStagedPackageOrigin<P0>(packagefullname: P0, origin: *mut PackageOrigin) -> super::LONG
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("api-ms-win-appmodel-runtime-l1-1-1.dll" "system" fn GetStagedPackageOrigin(packagefullname : windows_core::PCWSTR, origin : *mut PackageOrigin) -> super::LONG);
    unsafe { GetStagedPackageOrigin(packagefullname.param().abi(), origin as _) }
}
#[inline]
pub unsafe fn GetStagedPackageOrigin2<P0>(packagefullname: P0) -> windows_core::Result<PackageOrigin2>
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("api-ms-win-appmodel-runtime-l1-1-7.dll" "system" fn GetStagedPackageOrigin2(packagefullname : windows_core::PCWSTR, origin : *mut PackageOrigin2) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        GetStagedPackageOrigin2(packagefullname.param().abi(), &mut result__).map(|| result__)
    }
}
#[cfg(feature = "shared")]
#[inline]
pub unsafe fn GetStagedPackagePathByFullName<P0>(packagefullname: P0, pathlength: *mut super::UINT32, path: Option<windows_core::PWSTR>) -> super::LONG
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("kernel32.dll" "system" fn GetStagedPackagePathByFullName(packagefullname : windows_core::PCWSTR, pathlength : *mut super::UINT32, path : windows_core::PWSTR) -> super::LONG);
    unsafe { GetStagedPackagePathByFullName(packagefullname.param().abi(), pathlength as _, path.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(feature = "shared")]
#[inline]
pub unsafe fn GetStagedPackagePathByFullName2<P0>(packagefullname: P0, packagepathtype: PackagePathType, pathlength: *mut super::UINT32, path: Option<windows_core::PWSTR>) -> super::LONG
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("api-ms-win-appmodel-runtime-l1-1-3.dll" "system" fn GetStagedPackagePathByFullName2(packagefullname : windows_core::PCWSTR, packagepathtype : PackagePathType, pathlength : *mut super::UINT32, path : windows_core::PWSTR) -> super::LONG);
    unsafe { GetStagedPackagePathByFullName2(packagefullname.param().abi(), packagepathtype, pathlength as _, path.unwrap_or(core::mem::zeroed()) as _) }
}
#[inline]
pub unsafe fn IsPackageFeatureSupported(feature: AppModelPackageFeature) -> windows_core::BOOL {
    windows_core::link!("api-ms-win-appmodel-runtime-l1-1-7.dll" "system" fn IsPackageFeatureSupported(feature : AppModelPackageFeature) -> windows_core::BOOL);
    unsafe { IsPackageFeatureSupported(feature) }
}
#[cfg(feature = "shared")]
#[inline]
pub unsafe fn OpenPackageInfoByFullName<P0>(packagefullname: P0, reserved: Option<super::UINT32>, packageinforeference: *mut PACKAGE_INFO_REFERENCE) -> super::LONG
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("kernel32.dll" "system" fn OpenPackageInfoByFullName(packagefullname : windows_core::PCWSTR, reserved : super::UINT32, packageinforeference : *mut PACKAGE_INFO_REFERENCE) -> super::LONG);
    unsafe { OpenPackageInfoByFullName(packagefullname.param().abi(), reserved.unwrap_or(core::mem::zeroed()) as _, packageinforeference as _) }
}
#[cfg(all(feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn OpenPackageInfoByFullNameForUser<P1>(usersid: Option<super::PSID>, packagefullname: P1, reserved: Option<super::UINT32>, packageinforeference: *mut PACKAGE_INFO_REFERENCE) -> super::LONG
where
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("api-ms-win-appmodel-runtime-l1-1-1.dll" "system" fn OpenPackageInfoByFullNameForUser(usersid : super::PSID, packagefullname : windows_core::PCWSTR, reserved : super::UINT32, packageinforeference : *mut PACKAGE_INFO_REFERENCE) -> super::LONG);
    unsafe { OpenPackageInfoByFullNameForUser(usersid.unwrap_or(core::mem::zeroed()) as _, packagefullname.param().abi(), reserved.unwrap_or(core::mem::zeroed()) as _, packageinforeference as _) }
}
#[cfg(feature = "shared")]
#[inline]
pub unsafe fn PackageFamilyNameFromFullName<P0>(packagefullname: P0, packagefamilynamelength: *mut super::UINT32, packagefamilyname: Option<windows_core::PWSTR>) -> super::LONG
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("kernel32.dll" "system" fn PackageFamilyNameFromFullName(packagefullname : windows_core::PCWSTR, packagefamilynamelength : *mut super::UINT32, packagefamilyname : windows_core::PWSTR) -> super::LONG);
    unsafe { PackageFamilyNameFromFullName(packagefullname.param().abi(), packagefamilynamelength as _, packagefamilyname.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(feature = "shared")]
#[inline]
pub unsafe fn PackageFamilyNameFromId(packageid: *const PACKAGE_ID, packagefamilynamelength: *mut super::UINT32, packagefamilyname: Option<windows_core::PWSTR>) -> super::LONG {
    windows_core::link!("kernel32.dll" "system" fn PackageFamilyNameFromId(packageid : *const PACKAGE_ID, packagefamilynamelength : *mut super::UINT32, packagefamilyname : windows_core::PWSTR) -> super::LONG);
    unsafe { PackageFamilyNameFromId(packageid, packagefamilynamelength as _, packagefamilyname.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(feature = "shared")]
#[inline]
pub unsafe fn PackageFullNameFromId(packageid: *const PACKAGE_ID, packagefullnamelength: *mut super::UINT32, packagefullname: Option<windows_core::PWSTR>) -> super::LONG {
    windows_core::link!("kernel32.dll" "system" fn PackageFullNameFromId(packageid : *const PACKAGE_ID, packagefullnamelength : *mut super::UINT32, packagefullname : windows_core::PWSTR) -> super::LONG);
    unsafe { PackageFullNameFromId(packageid, packagefullnamelength as _, packagefullname.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(feature = "shared")]
#[inline]
pub unsafe fn PackageIdFromFullName<P0>(packagefullname: P0, flags: super::UINT32, bufferlength: *mut super::UINT32, buffer: Option<*mut super::BYTE>) -> super::LONG
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("kernel32.dll" "system" fn PackageIdFromFullName(packagefullname : windows_core::PCWSTR, flags : super::UINT32, bufferlength : *mut super::UINT32, buffer : *mut super::BYTE) -> super::LONG);
    unsafe { PackageIdFromFullName(packagefullname.param().abi(), flags, bufferlength as _, buffer.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(feature = "shared")]
#[inline]
pub unsafe fn PackageNameAndPublisherIdFromFamilyName<P0>(packagefamilyname: P0, packagenamelength: *mut super::UINT32, packagename: Option<windows_core::PWSTR>, packagepublisheridlength: *mut super::UINT32, packagepublisherid: Option<windows_core::PWSTR>) -> super::LONG
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("kernel32.dll" "system" fn PackageNameAndPublisherIdFromFamilyName(packagefamilyname : windows_core::PCWSTR, packagenamelength : *mut super::UINT32, packagename : windows_core::PWSTR, packagepublisheridlength : *mut super::UINT32, packagepublisherid : windows_core::PWSTR) -> super::LONG);
    unsafe { PackageNameAndPublisherIdFromFamilyName(packagefamilyname.param().abi(), packagenamelength as _, packagename.unwrap_or(core::mem::zeroed()) as _, packagepublisheridlength as _, packagepublisherid.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(feature = "shared")]
#[inline]
pub unsafe fn ParseApplicationUserModelId<P0>(applicationusermodelid: P0, packagefamilynamelength: *mut super::UINT32, packagefamilyname: Option<windows_core::PWSTR>, packagerelativeapplicationidlength: *mut super::UINT32, packagerelativeapplicationid: Option<windows_core::PWSTR>) -> super::LONG
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("kernel32.dll" "system" fn ParseApplicationUserModelId(applicationusermodelid : windows_core::PCWSTR, packagefamilynamelength : *mut super::UINT32, packagefamilyname : windows_core::PWSTR, packagerelativeapplicationidlength : *mut super::UINT32, packagerelativeapplicationid : windows_core::PWSTR) -> super::LONG);
    unsafe { ParseApplicationUserModelId(applicationusermodelid.param().abi(), packagefamilynamelength as _, packagefamilyname.unwrap_or(core::mem::zeroed()) as _, packagerelativeapplicationidlength as _, packagerelativeapplicationid.unwrap_or(core::mem::zeroed()) as _) }
}
#[inline]
pub unsafe fn RemovePackageDependency(packagedependencycontext: PACKAGEDEPENDENCY_CONTEXT) -> windows_core::HRESULT {
    windows_core::link!("api-ms-win-appmodel-runtime-l1-1-5.dll" "system" fn RemovePackageDependency(packagedependencycontext : PACKAGEDEPENDENCY_CONTEXT) -> windows_core::HRESULT);
    unsafe { RemovePackageDependency(packagedependencycontext) }
}
#[cfg(all(feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn TryCreatePackageDependency<P1, P5>(user: super::PSID, packagefamilyname: P1, minversion: PACKAGE_VERSION, packagedependencyprocessorarchitectures: PackageDependencyProcessorArchitectures, lifetimekind: PackageDependencyLifetimeKind, lifetimeartifact: P5, options: CreatePackageDependencyOptions) -> windows_core::Result<windows_core::PWSTR>
where
    P1: windows_core::Param<windows_core::PCWSTR>,
    P5: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("api-ms-win-appmodel-runtime-l1-1-5.dll" "system" fn TryCreatePackageDependency(user : super::PSID, packagefamilyname : windows_core::PCWSTR, minversion : PACKAGE_VERSION, packagedependencyprocessorarchitectures : PackageDependencyProcessorArchitectures, lifetimekind : PackageDependencyLifetimeKind, lifetimeartifact : windows_core::PCWSTR, options : CreatePackageDependencyOptions, packagedependencyid : *mut windows_core::PWSTR) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        TryCreatePackageDependency(user as _, packagefamilyname.param().abi(), minversion, packagedependencyprocessorarchitectures, lifetimekind, lifetimeartifact.param().abi(), options, &mut result__).map(|| result__)
    }
}
#[cfg(all(feature = "minwindef", feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn TryCreatePackageDependency2<P1, P5>(user: super::PSID, packagefamilyname: P1, minversion: PACKAGE_VERSION, packagedependencyprocessorarchitectures: PackageDependencyProcessorArchitectures, lifetimekind: PackageDependencyLifetimeKind, lifetimeartifact: P5, options: CreatePackageDependencyOptions, lifetimeexpiration: *const super::FILETIME) -> windows_core::Result<windows_core::PWSTR>
where
    P1: windows_core::Param<windows_core::PCWSTR>,
    P5: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("api-ms-win-appmodel-runtime-l1-1-7.dll" "system" fn TryCreatePackageDependency2(user : super::PSID, packagefamilyname : windows_core::PCWSTR, minversion : PACKAGE_VERSION, packagedependencyprocessorarchitectures : PackageDependencyProcessorArchitectures, lifetimekind : PackageDependencyLifetimeKind, lifetimeartifact : windows_core::PCWSTR, options : CreatePackageDependencyOptions, lifetimeexpiration : *const super::FILETIME, packagedependencyid : *mut windows_core::PWSTR) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        TryCreatePackageDependency2(user as _, packagefamilyname.param().abi(), minversion, packagedependencyprocessorarchitectures, lifetimekind, lifetimeartifact.param().abi(), options, lifetimeexpiration, &mut result__).map(|| result__)
    }
}
#[cfg(feature = "shared")]
#[inline]
pub unsafe fn VerifyApplicationUserModelId<P0>(applicationusermodelid: P0) -> super::LONG
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("api-ms-win-appmodel-runtime-l1-1-1.dll" "system" fn VerifyApplicationUserModelId(applicationusermodelid : windows_core::PCWSTR) -> super::LONG);
    unsafe { VerifyApplicationUserModelId(applicationusermodelid.param().abi()) }
}
#[cfg(feature = "shared")]
#[inline]
pub unsafe fn VerifyPackageFamilyName<P0>(packagefamilyname: P0) -> super::LONG
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("api-ms-win-appmodel-runtime-l1-1-1.dll" "system" fn VerifyPackageFamilyName(packagefamilyname : windows_core::PCWSTR) -> super::LONG);
    unsafe { VerifyPackageFamilyName(packagefamilyname.param().abi()) }
}
#[cfg(feature = "shared")]
#[inline]
pub unsafe fn VerifyPackageFullName<P0>(packagefullname: P0) -> super::LONG
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("api-ms-win-appmodel-runtime-l1-1-1.dll" "system" fn VerifyPackageFullName(packagefullname : windows_core::PCWSTR) -> super::LONG);
    unsafe { VerifyPackageFullName(packagefullname.param().abi()) }
}
#[cfg(feature = "shared")]
#[inline]
pub unsafe fn VerifyPackageId(packageid: *const PACKAGE_ID) -> super::LONG {
    windows_core::link!("api-ms-win-appmodel-runtime-l1-1-1.dll" "system" fn VerifyPackageId(packageid : *const PACKAGE_ID) -> super::LONG);
    unsafe { VerifyPackageId(packageid) }
}
#[cfg(feature = "shared")]
#[inline]
pub unsafe fn VerifyPackageRelativeApplicationId<P0>(packagerelativeapplicationid: P0) -> super::LONG
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("api-ms-win-appmodel-runtime-l1-1-1.dll" "system" fn VerifyPackageRelativeApplicationId(packagerelativeapplicationid : windows_core::PCWSTR) -> super::LONG);
    unsafe { VerifyPackageRelativeApplicationId(packagerelativeapplicationid.param().abi()) }
}
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
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct FindPackageDependencyCriteria {
    pub User: super::PSID,
    pub ScopeIsSystem: windows_core::BOOL,
    pub PackageFamilyName: windows_core::PCWSTR,
}
pub type PACKAGEDEPENDENCY_CONTEXT = *mut PACKAGEDEPENDENCY_CONTEXT__;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
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
    pub name: windows_core::PWSTR,
    pub publisher: windows_core::PWSTR,
    pub resourceId: windows_core::PWSTR,
    pub publisherId: windows_core::PWSTR,
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
    pub path: windows_core::PWSTR,
    pub packageFullName: windows_core::PWSTR,
    pub packageFamilyName: windows_core::PWSTR,
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
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct _PACKAGE_INFO_REFERENCE {
    pub reserved: *mut core::ffi::c_void,
}

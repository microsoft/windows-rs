#[doc = "*Required features: `\"Win32_Storage_Packaging_Appx\"`*"]
#[inline]
pub unsafe fn ActivatePackageVirtualizationContext(context: *const PACKAGE_VIRTUALIZATION_CONTEXT_HANDLE__) -> ::windows::core::Result<usize> {
    ::windows::core::link ! ( "kernel32.dll""system" fn ActivatePackageVirtualizationContext ( context : *const PACKAGE_VIRTUALIZATION_CONTEXT_HANDLE__ , cookie : *mut usize ) -> :: windows::core::HRESULT );
    let mut result__ = ::core::mem::MaybeUninit::zeroed();
    ActivatePackageVirtualizationContext(context, result__.as_mut_ptr()).from_abi(result__)
}
#[doc = "*Required features: `\"Win32_Storage_Packaging_Appx\"`*"]
#[inline]
pub unsafe fn AddPackageDependency<P0>(packagedependencyid: P0, rank: i32, options: AddPackageDependencyOptions, packagedependencycontext: *mut *mut PACKAGEDEPENDENCY_CONTEXT__, packagefullname: ::core::option::Option<*mut ::windows::core::PWSTR>) -> ::windows::core::Result<()>
where
    P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
{
    ::windows::core::link ! ( "kernelbase.dll""system" fn AddPackageDependency ( packagedependencyid : :: windows::core::PCWSTR , rank : i32 , options : AddPackageDependencyOptions , packagedependencycontext : *mut *mut PACKAGEDEPENDENCY_CONTEXT__ , packagefullname : *mut :: windows::core::PWSTR ) -> :: windows::core::HRESULT );
    AddPackageDependency(packagedependencyid.into().abi(), rank, options, packagedependencycontext, ::core::mem::transmute(packagefullname.unwrap_or(::std::ptr::null_mut()))).ok()
}
#[doc = "*Required features: `\"Win32_Storage_Packaging_Appx\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn AppPolicyGetClrCompat<P0>(processtoken: P0, policy: *mut AppPolicyClrCompat) -> super::super::super::Foundation::WIN32_ERROR
where
    P0: ::std::convert::Into<super::super::super::Foundation::HANDLE>,
{
    ::windows::core::link ! ( "kernel32.dll""system" fn AppPolicyGetClrCompat ( processtoken : super::super::super::Foundation:: HANDLE , policy : *mut AppPolicyClrCompat ) -> super::super::super::Foundation:: WIN32_ERROR );
    AppPolicyGetClrCompat(processtoken.into(), policy)
}
#[doc = "*Required features: `\"Win32_Storage_Packaging_Appx\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn AppPolicyGetCreateFileAccess<P0>(processtoken: P0, policy: *mut AppPolicyCreateFileAccess) -> super::super::super::Foundation::WIN32_ERROR
where
    P0: ::std::convert::Into<super::super::super::Foundation::HANDLE>,
{
    ::windows::core::link ! ( "kernel32.dll""system" fn AppPolicyGetCreateFileAccess ( processtoken : super::super::super::Foundation:: HANDLE , policy : *mut AppPolicyCreateFileAccess ) -> super::super::super::Foundation:: WIN32_ERROR );
    AppPolicyGetCreateFileAccess(processtoken.into(), policy)
}
#[doc = "*Required features: `\"Win32_Storage_Packaging_Appx\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn AppPolicyGetLifecycleManagement<P0>(processtoken: P0, policy: *mut AppPolicyLifecycleManagement) -> super::super::super::Foundation::WIN32_ERROR
where
    P0: ::std::convert::Into<super::super::super::Foundation::HANDLE>,
{
    ::windows::core::link ! ( "kernel32.dll""system" fn AppPolicyGetLifecycleManagement ( processtoken : super::super::super::Foundation:: HANDLE , policy : *mut AppPolicyLifecycleManagement ) -> super::super::super::Foundation:: WIN32_ERROR );
    AppPolicyGetLifecycleManagement(processtoken.into(), policy)
}
#[doc = "*Required features: `\"Win32_Storage_Packaging_Appx\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn AppPolicyGetMediaFoundationCodecLoading<P0>(processtoken: P0, policy: *mut AppPolicyMediaFoundationCodecLoading) -> super::super::super::Foundation::WIN32_ERROR
where
    P0: ::std::convert::Into<super::super::super::Foundation::HANDLE>,
{
    ::windows::core::link ! ( "kernel32.dll""system" fn AppPolicyGetMediaFoundationCodecLoading ( processtoken : super::super::super::Foundation:: HANDLE , policy : *mut AppPolicyMediaFoundationCodecLoading ) -> super::super::super::Foundation:: WIN32_ERROR );
    AppPolicyGetMediaFoundationCodecLoading(processtoken.into(), policy)
}
#[doc = "*Required features: `\"Win32_Storage_Packaging_Appx\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn AppPolicyGetProcessTerminationMethod<P0>(processtoken: P0, policy: *mut AppPolicyProcessTerminationMethod) -> super::super::super::Foundation::WIN32_ERROR
where
    P0: ::std::convert::Into<super::super::super::Foundation::HANDLE>,
{
    ::windows::core::link ! ( "kernel32.dll""system" fn AppPolicyGetProcessTerminationMethod ( processtoken : super::super::super::Foundation:: HANDLE , policy : *mut AppPolicyProcessTerminationMethod ) -> super::super::super::Foundation:: WIN32_ERROR );
    AppPolicyGetProcessTerminationMethod(processtoken.into(), policy)
}
#[doc = "*Required features: `\"Win32_Storage_Packaging_Appx\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn AppPolicyGetShowDeveloperDiagnostic<P0>(processtoken: P0, policy: *mut AppPolicyShowDeveloperDiagnostic) -> super::super::super::Foundation::WIN32_ERROR
where
    P0: ::std::convert::Into<super::super::super::Foundation::HANDLE>,
{
    ::windows::core::link ! ( "kernel32.dll""system" fn AppPolicyGetShowDeveloperDiagnostic ( processtoken : super::super::super::Foundation:: HANDLE , policy : *mut AppPolicyShowDeveloperDiagnostic ) -> super::super::super::Foundation:: WIN32_ERROR );
    AppPolicyGetShowDeveloperDiagnostic(processtoken.into(), policy)
}
#[doc = "*Required features: `\"Win32_Storage_Packaging_Appx\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn AppPolicyGetThreadInitializationType<P0>(processtoken: P0, policy: *mut AppPolicyThreadInitializationType) -> super::super::super::Foundation::WIN32_ERROR
where
    P0: ::std::convert::Into<super::super::super::Foundation::HANDLE>,
{
    ::windows::core::link ! ( "kernel32.dll""system" fn AppPolicyGetThreadInitializationType ( processtoken : super::super::super::Foundation:: HANDLE , policy : *mut AppPolicyThreadInitializationType ) -> super::super::super::Foundation:: WIN32_ERROR );
    AppPolicyGetThreadInitializationType(processtoken.into(), policy)
}
#[doc = "*Required features: `\"Win32_Storage_Packaging_Appx\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn AppPolicyGetWindowingModel<P0>(processtoken: P0, policy: *mut AppPolicyWindowingModel) -> super::super::super::Foundation::WIN32_ERROR
where
    P0: ::std::convert::Into<super::super::super::Foundation::HANDLE>,
{
    ::windows::core::link ! ( "kernel32.dll""system" fn AppPolicyGetWindowingModel ( processtoken : super::super::super::Foundation:: HANDLE , policy : *mut AppPolicyWindowingModel ) -> super::super::super::Foundation:: WIN32_ERROR );
    AppPolicyGetWindowingModel(processtoken.into(), policy)
}
#[doc = "*Required features: `\"Win32_Storage_Packaging_Appx\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn CheckIsMSIXPackage<P0>(packagefullname: P0) -> ::windows::core::Result<super::super::super::Foundation::BOOL>
where
    P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
{
    ::windows::core::link ! ( "kernel32.dll""system" fn CheckIsMSIXPackage ( packagefullname : :: windows::core::PCWSTR , ismsixpackage : *mut super::super::super::Foundation:: BOOL ) -> :: windows::core::HRESULT );
    let mut result__ = ::core::mem::MaybeUninit::zeroed();
    CheckIsMSIXPackage(packagefullname.into().abi(), result__.as_mut_ptr()).from_abi(result__)
}
#[doc = "*Required features: `\"Win32_Storage_Packaging_Appx\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn ClosePackageInfo(packageinforeference: *const _PACKAGE_INFO_REFERENCE) -> super::super::super::Foundation::WIN32_ERROR {
    ::windows::core::link ! ( "kernel32.dll""system" fn ClosePackageInfo ( packageinforeference : *const _PACKAGE_INFO_REFERENCE ) -> super::super::super::Foundation:: WIN32_ERROR );
    ClosePackageInfo(packageinforeference)
}
#[doc = "*Required features: `\"Win32_Storage_Packaging_Appx\"`*"]
#[inline]
pub unsafe fn CreatePackageVirtualizationContext<P0>(packagefamilyname: P0) -> ::windows::core::Result<*mut PACKAGE_VIRTUALIZATION_CONTEXT_HANDLE__>
where
    P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
{
    ::windows::core::link ! ( "kernel32.dll""system" fn CreatePackageVirtualizationContext ( packagefamilyname : :: windows::core::PCWSTR , context : *mut *mut PACKAGE_VIRTUALIZATION_CONTEXT_HANDLE__ ) -> :: windows::core::HRESULT );
    let mut result__ = ::core::mem::MaybeUninit::zeroed();
    CreatePackageVirtualizationContext(packagefamilyname.into().abi(), result__.as_mut_ptr()).from_abi(result__)
}
#[doc = "*Required features: `\"Win32_Storage_Packaging_Appx\"`*"]
#[inline]
pub unsafe fn DeactivatePackageVirtualizationContext(cookie: usize) {
    ::windows::core::link ! ( "kernel32.dll""system" fn DeactivatePackageVirtualizationContext ( cookie : usize ) -> ( ) );
    DeactivatePackageVirtualizationContext(cookie)
}
#[doc = "*Required features: `\"Win32_Storage_Packaging_Appx\"`*"]
#[inline]
pub unsafe fn DeletePackageDependency<P0>(packagedependencyid: P0) -> ::windows::core::Result<()>
where
    P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
{
    ::windows::core::link ! ( "kernelbase.dll""system" fn DeletePackageDependency ( packagedependencyid : :: windows::core::PCWSTR ) -> :: windows::core::HRESULT );
    DeletePackageDependency(packagedependencyid.into().abi()).ok()
}
#[doc = "*Required features: `\"Win32_Storage_Packaging_Appx\"`*"]
#[inline]
pub unsafe fn DuplicatePackageVirtualizationContext(sourcecontext: *const PACKAGE_VIRTUALIZATION_CONTEXT_HANDLE__) -> ::windows::core::Result<*mut PACKAGE_VIRTUALIZATION_CONTEXT_HANDLE__> {
    ::windows::core::link ! ( "kernel32.dll""system" fn DuplicatePackageVirtualizationContext ( sourcecontext : *const PACKAGE_VIRTUALIZATION_CONTEXT_HANDLE__ , destcontext : *mut *mut PACKAGE_VIRTUALIZATION_CONTEXT_HANDLE__ ) -> :: windows::core::HRESULT );
    let mut result__ = ::core::mem::MaybeUninit::zeroed();
    DuplicatePackageVirtualizationContext(sourcecontext, result__.as_mut_ptr()).from_abi(result__)
}
#[doc = "*Required features: `\"Win32_Storage_Packaging_Appx\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn FindPackagesByPackageFamily<P0>(packagefamilyname: P0, packagefilters: u32, count: *mut u32, packagefullnames: ::core::option::Option<*mut ::windows::core::PWSTR>, bufferlength: *mut u32, buffer: ::windows::core::PWSTR, packageproperties: ::core::option::Option<*mut u32>) -> super::super::super::Foundation::WIN32_ERROR
where
    P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
{
    ::windows::core::link ! ( "kernel32.dll""system" fn FindPackagesByPackageFamily ( packagefamilyname : :: windows::core::PCWSTR , packagefilters : u32 , count : *mut u32 , packagefullnames : *mut :: windows::core::PWSTR , bufferlength : *mut u32 , buffer : :: windows::core::PWSTR , packageproperties : *mut u32 ) -> super::super::super::Foundation:: WIN32_ERROR );
    FindPackagesByPackageFamily(packagefamilyname.into().abi(), packagefilters, count, ::core::mem::transmute(packagefullnames.unwrap_or(::std::ptr::null_mut())), bufferlength, ::core::mem::transmute(buffer), ::core::mem::transmute(packageproperties.unwrap_or(::std::ptr::null_mut())))
}
#[doc = "*Required features: `\"Win32_Storage_Packaging_Appx\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn FormatApplicationUserModelId<P0, P1>(packagefamilyname: P0, packagerelativeapplicationid: P1, applicationusermodelidlength: *mut u32, applicationusermodelid: ::windows::core::PWSTR) -> super::super::super::Foundation::WIN32_ERROR
where
    P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
    P1: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
{
    ::windows::core::link ! ( "kernel32.dll""system" fn FormatApplicationUserModelId ( packagefamilyname : :: windows::core::PCWSTR , packagerelativeapplicationid : :: windows::core::PCWSTR , applicationusermodelidlength : *mut u32 , applicationusermodelid : :: windows::core::PWSTR ) -> super::super::super::Foundation:: WIN32_ERROR );
    FormatApplicationUserModelId(packagefamilyname.into().abi(), packagerelativeapplicationid.into().abi(), applicationusermodelidlength, ::core::mem::transmute(applicationusermodelid))
}
#[doc = "*Required features: `\"Win32_Storage_Packaging_Appx\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn GetApplicationUserModelId<P0>(hprocess: P0, applicationusermodelidlength: *mut u32, applicationusermodelid: ::windows::core::PWSTR) -> super::super::super::Foundation::WIN32_ERROR
where
    P0: ::std::convert::Into<super::super::super::Foundation::HANDLE>,
{
    ::windows::core::link ! ( "kernel32.dll""system" fn GetApplicationUserModelId ( hprocess : super::super::super::Foundation:: HANDLE , applicationusermodelidlength : *mut u32 , applicationusermodelid : :: windows::core::PWSTR ) -> super::super::super::Foundation:: WIN32_ERROR );
    GetApplicationUserModelId(hprocess.into(), applicationusermodelidlength, ::core::mem::transmute(applicationusermodelid))
}
#[doc = "*Required features: `\"Win32_Storage_Packaging_Appx\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn GetApplicationUserModelIdFromToken<P0>(token: P0, applicationusermodelidlength: *mut u32, applicationusermodelid: ::windows::core::PWSTR) -> super::super::super::Foundation::WIN32_ERROR
where
    P0: ::std::convert::Into<super::super::super::Foundation::HANDLE>,
{
    ::windows::core::link ! ( "api-ms-win-appmodel-runtime-l1-1-1.dll""system" fn GetApplicationUserModelIdFromToken ( token : super::super::super::Foundation:: HANDLE , applicationusermodelidlength : *mut u32 , applicationusermodelid : :: windows::core::PWSTR ) -> super::super::super::Foundation:: WIN32_ERROR );
    GetApplicationUserModelIdFromToken(token.into(), applicationusermodelidlength, ::core::mem::transmute(applicationusermodelid))
}
#[doc = "*Required features: `\"Win32_Storage_Packaging_Appx\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn GetCurrentApplicationUserModelId(applicationusermodelidlength: *mut u32, applicationusermodelid: ::windows::core::PWSTR) -> super::super::super::Foundation::WIN32_ERROR {
    ::windows::core::link ! ( "kernel32.dll""system" fn GetCurrentApplicationUserModelId ( applicationusermodelidlength : *mut u32 , applicationusermodelid : :: windows::core::PWSTR ) -> super::super::super::Foundation:: WIN32_ERROR );
    GetCurrentApplicationUserModelId(applicationusermodelidlength, ::core::mem::transmute(applicationusermodelid))
}
#[doc = "*Required features: `\"Win32_Storage_Packaging_Appx\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn GetCurrentPackageFamilyName(packagefamilynamelength: *mut u32, packagefamilyname: ::windows::core::PWSTR) -> super::super::super::Foundation::WIN32_ERROR {
    ::windows::core::link ! ( "kernel32.dll""system" fn GetCurrentPackageFamilyName ( packagefamilynamelength : *mut u32 , packagefamilyname : :: windows::core::PWSTR ) -> super::super::super::Foundation:: WIN32_ERROR );
    GetCurrentPackageFamilyName(packagefamilynamelength, ::core::mem::transmute(packagefamilyname))
}
#[doc = "*Required features: `\"Win32_Storage_Packaging_Appx\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn GetCurrentPackageFullName(packagefullnamelength: *mut u32, packagefullname: ::windows::core::PWSTR) -> super::super::super::Foundation::WIN32_ERROR {
    ::windows::core::link ! ( "kernel32.dll""system" fn GetCurrentPackageFullName ( packagefullnamelength : *mut u32 , packagefullname : :: windows::core::PWSTR ) -> super::super::super::Foundation:: WIN32_ERROR );
    GetCurrentPackageFullName(packagefullnamelength, ::core::mem::transmute(packagefullname))
}
#[doc = "*Required features: `\"Win32_Storage_Packaging_Appx\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn GetCurrentPackageId(bufferlength: *mut u32, buffer: ::core::option::Option<*mut u8>) -> super::super::super::Foundation::WIN32_ERROR {
    ::windows::core::link ! ( "kernel32.dll""system" fn GetCurrentPackageId ( bufferlength : *mut u32 , buffer : *mut u8 ) -> super::super::super::Foundation:: WIN32_ERROR );
    GetCurrentPackageId(bufferlength, ::core::mem::transmute(buffer.unwrap_or(::std::ptr::null_mut())))
}
#[doc = "*Required features: `\"Win32_Storage_Packaging_Appx\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn GetCurrentPackageInfo(flags: u32, bufferlength: *mut u32, buffer: ::core::option::Option<*mut u8>, count: ::core::option::Option<*mut u32>) -> super::super::super::Foundation::WIN32_ERROR {
    ::windows::core::link ! ( "kernel32.dll""system" fn GetCurrentPackageInfo ( flags : u32 , bufferlength : *mut u32 , buffer : *mut u8 , count : *mut u32 ) -> super::super::super::Foundation:: WIN32_ERROR );
    GetCurrentPackageInfo(flags, bufferlength, ::core::mem::transmute(buffer.unwrap_or(::std::ptr::null_mut())), ::core::mem::transmute(count.unwrap_or(::std::ptr::null_mut())))
}
#[doc = "*Required features: `\"Win32_Storage_Packaging_Appx\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn GetCurrentPackageInfo2(flags: u32, packagepathtype: PackagePathType, bufferlength: *mut u32, buffer: ::core::option::Option<*mut u8>, count: ::core::option::Option<*mut u32>) -> super::super::super::Foundation::WIN32_ERROR {
    ::windows::core::link ! ( "api-ms-win-appmodel-runtime-l1-1-3.dll""system" fn GetCurrentPackageInfo2 ( flags : u32 , packagepathtype : PackagePathType , bufferlength : *mut u32 , buffer : *mut u8 , count : *mut u32 ) -> super::super::super::Foundation:: WIN32_ERROR );
    GetCurrentPackageInfo2(flags, packagepathtype, bufferlength, ::core::mem::transmute(buffer.unwrap_or(::std::ptr::null_mut())), ::core::mem::transmute(count.unwrap_or(::std::ptr::null_mut())))
}
#[doc = "*Required features: `\"Win32_Storage_Packaging_Appx\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn GetCurrentPackagePath(pathlength: *mut u32, path: ::windows::core::PWSTR) -> super::super::super::Foundation::WIN32_ERROR {
    ::windows::core::link ! ( "kernel32.dll""system" fn GetCurrentPackagePath ( pathlength : *mut u32 , path : :: windows::core::PWSTR ) -> super::super::super::Foundation:: WIN32_ERROR );
    GetCurrentPackagePath(pathlength, ::core::mem::transmute(path))
}
#[doc = "*Required features: `\"Win32_Storage_Packaging_Appx\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn GetCurrentPackagePath2(packagepathtype: PackagePathType, pathlength: *mut u32, path: ::windows::core::PWSTR) -> super::super::super::Foundation::WIN32_ERROR {
    ::windows::core::link ! ( "api-ms-win-appmodel-runtime-l1-1-3.dll""system" fn GetCurrentPackagePath2 ( packagepathtype : PackagePathType , pathlength : *mut u32 , path : :: windows::core::PWSTR ) -> super::super::super::Foundation:: WIN32_ERROR );
    GetCurrentPackagePath2(packagepathtype, pathlength, ::core::mem::transmute(path))
}
#[doc = "*Required features: `\"Win32_Storage_Packaging_Appx\"`*"]
#[inline]
pub unsafe fn GetCurrentPackageVirtualizationContext() -> *mut PACKAGE_VIRTUALIZATION_CONTEXT_HANDLE__ {
    ::windows::core::link ! ( "kernel32.dll""system" fn GetCurrentPackageVirtualizationContext ( ) -> *mut PACKAGE_VIRTUALIZATION_CONTEXT_HANDLE__ );
    GetCurrentPackageVirtualizationContext()
}
#[doc = "*Required features: `\"Win32_Storage_Packaging_Appx\"`*"]
#[inline]
pub unsafe fn GetIdForPackageDependencyContext(packagedependencycontext: *const PACKAGEDEPENDENCY_CONTEXT__) -> ::windows::core::Result<::windows::core::PWSTR> {
    ::windows::core::link ! ( "kernelbase.dll""system" fn GetIdForPackageDependencyContext ( packagedependencycontext : *const PACKAGEDEPENDENCY_CONTEXT__ , packagedependencyid : *mut :: windows::core::PWSTR ) -> :: windows::core::HRESULT );
    let mut result__ = ::core::mem::MaybeUninit::zeroed();
    GetIdForPackageDependencyContext(packagedependencycontext, result__.as_mut_ptr()).from_abi(result__)
}
#[doc = "*Required features: `\"Win32_Storage_Packaging_Appx\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn GetPackageApplicationIds(packageinforeference: *const _PACKAGE_INFO_REFERENCE, bufferlength: *mut u32, buffer: ::core::option::Option<*mut u8>, count: ::core::option::Option<*mut u32>) -> super::super::super::Foundation::WIN32_ERROR {
    ::windows::core::link ! ( "kernel32.dll""system" fn GetPackageApplicationIds ( packageinforeference : *const _PACKAGE_INFO_REFERENCE , bufferlength : *mut u32 , buffer : *mut u8 , count : *mut u32 ) -> super::super::super::Foundation:: WIN32_ERROR );
    GetPackageApplicationIds(packageinforeference, bufferlength, ::core::mem::transmute(buffer.unwrap_or(::std::ptr::null_mut())), ::core::mem::transmute(count.unwrap_or(::std::ptr::null_mut())))
}
#[doc = "*Required features: `\"Win32_Storage_Packaging_Appx\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn GetPackageFamilyName<P0>(hprocess: P0, packagefamilynamelength: *mut u32, packagefamilyname: ::windows::core::PWSTR) -> super::super::super::Foundation::WIN32_ERROR
where
    P0: ::std::convert::Into<super::super::super::Foundation::HANDLE>,
{
    ::windows::core::link ! ( "kernel32.dll""system" fn GetPackageFamilyName ( hprocess : super::super::super::Foundation:: HANDLE , packagefamilynamelength : *mut u32 , packagefamilyname : :: windows::core::PWSTR ) -> super::super::super::Foundation:: WIN32_ERROR );
    GetPackageFamilyName(hprocess.into(), packagefamilynamelength, ::core::mem::transmute(packagefamilyname))
}
#[doc = "*Required features: `\"Win32_Storage_Packaging_Appx\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn GetPackageFamilyNameFromToken<P0>(token: P0, packagefamilynamelength: *mut u32, packagefamilyname: ::windows::core::PWSTR) -> super::super::super::Foundation::WIN32_ERROR
where
    P0: ::std::convert::Into<super::super::super::Foundation::HANDLE>,
{
    ::windows::core::link ! ( "api-ms-win-appmodel-runtime-l1-1-1.dll""system" fn GetPackageFamilyNameFromToken ( token : super::super::super::Foundation:: HANDLE , packagefamilynamelength : *mut u32 , packagefamilyname : :: windows::core::PWSTR ) -> super::super::super::Foundation:: WIN32_ERROR );
    GetPackageFamilyNameFromToken(token.into(), packagefamilynamelength, ::core::mem::transmute(packagefamilyname))
}
#[doc = "*Required features: `\"Win32_Storage_Packaging_Appx\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn GetPackageFullName<P0>(hprocess: P0, packagefullnamelength: *mut u32, packagefullname: ::windows::core::PWSTR) -> super::super::super::Foundation::WIN32_ERROR
where
    P0: ::std::convert::Into<super::super::super::Foundation::HANDLE>,
{
    ::windows::core::link ! ( "kernel32.dll""system" fn GetPackageFullName ( hprocess : super::super::super::Foundation:: HANDLE , packagefullnamelength : *mut u32 , packagefullname : :: windows::core::PWSTR ) -> super::super::super::Foundation:: WIN32_ERROR );
    GetPackageFullName(hprocess.into(), packagefullnamelength, ::core::mem::transmute(packagefullname))
}
#[doc = "*Required features: `\"Win32_Storage_Packaging_Appx\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn GetPackageFullNameFromToken<P0>(token: P0, packagefullnamelength: *mut u32, packagefullname: ::windows::core::PWSTR) -> super::super::super::Foundation::WIN32_ERROR
where
    P0: ::std::convert::Into<super::super::super::Foundation::HANDLE>,
{
    ::windows::core::link ! ( "api-ms-win-appmodel-runtime-l1-1-1.dll""system" fn GetPackageFullNameFromToken ( token : super::super::super::Foundation:: HANDLE , packagefullnamelength : *mut u32 , packagefullname : :: windows::core::PWSTR ) -> super::super::super::Foundation:: WIN32_ERROR );
    GetPackageFullNameFromToken(token.into(), packagefullnamelength, ::core::mem::transmute(packagefullname))
}
#[doc = "*Required features: `\"Win32_Storage_Packaging_Appx\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn GetPackageId<P0>(hprocess: P0, bufferlength: *mut u32, buffer: ::core::option::Option<*mut u8>) -> super::super::super::Foundation::WIN32_ERROR
where
    P0: ::std::convert::Into<super::super::super::Foundation::HANDLE>,
{
    ::windows::core::link ! ( "kernel32.dll""system" fn GetPackageId ( hprocess : super::super::super::Foundation:: HANDLE , bufferlength : *mut u32 , buffer : *mut u8 ) -> super::super::super::Foundation:: WIN32_ERROR );
    GetPackageId(hprocess.into(), bufferlength, ::core::mem::transmute(buffer.unwrap_or(::std::ptr::null_mut())))
}
#[doc = "*Required features: `\"Win32_Storage_Packaging_Appx\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn GetPackageInfo(packageinforeference: *const _PACKAGE_INFO_REFERENCE, flags: u32, bufferlength: *mut u32, buffer: ::core::option::Option<*mut u8>, count: ::core::option::Option<*mut u32>) -> super::super::super::Foundation::WIN32_ERROR {
    ::windows::core::link ! ( "kernel32.dll""system" fn GetPackageInfo ( packageinforeference : *const _PACKAGE_INFO_REFERENCE , flags : u32 , bufferlength : *mut u32 , buffer : *mut u8 , count : *mut u32 ) -> super::super::super::Foundation:: WIN32_ERROR );
    GetPackageInfo(packageinforeference, flags, bufferlength, ::core::mem::transmute(buffer.unwrap_or(::std::ptr::null_mut())), ::core::mem::transmute(count.unwrap_or(::std::ptr::null_mut())))
}
#[doc = "*Required features: `\"Win32_Storage_Packaging_Appx\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn GetPackageInfo2(packageinforeference: *const _PACKAGE_INFO_REFERENCE, flags: u32, packagepathtype: PackagePathType, bufferlength: *mut u32, buffer: ::core::option::Option<*mut u8>, count: ::core::option::Option<*mut u32>) -> super::super::super::Foundation::WIN32_ERROR {
    ::windows::core::link ! ( "api-ms-win-appmodel-runtime-l1-1-3.dll""system" fn GetPackageInfo2 ( packageinforeference : *const _PACKAGE_INFO_REFERENCE , flags : u32 , packagepathtype : PackagePathType , bufferlength : *mut u32 , buffer : *mut u8 , count : *mut u32 ) -> super::super::super::Foundation:: WIN32_ERROR );
    GetPackageInfo2(packageinforeference, flags, packagepathtype, bufferlength, ::core::mem::transmute(buffer.unwrap_or(::std::ptr::null_mut())), ::core::mem::transmute(count.unwrap_or(::std::ptr::null_mut())))
}
#[doc = "*Required features: `\"Win32_Storage_Packaging_Appx\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn GetPackagePath(packageid: *const PACKAGE_ID, reserved: u32, pathlength: *mut u32, path: ::windows::core::PWSTR) -> super::super::super::Foundation::WIN32_ERROR {
    ::windows::core::link ! ( "kernel32.dll""system" fn GetPackagePath ( packageid : *const PACKAGE_ID , reserved : u32 , pathlength : *mut u32 , path : :: windows::core::PWSTR ) -> super::super::super::Foundation:: WIN32_ERROR );
    GetPackagePath(packageid, reserved, pathlength, ::core::mem::transmute(path))
}
#[doc = "*Required features: `\"Win32_Storage_Packaging_Appx\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn GetPackagePathByFullName<P0>(packagefullname: P0, pathlength: *mut u32, path: ::windows::core::PWSTR) -> super::super::super::Foundation::WIN32_ERROR
where
    P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
{
    ::windows::core::link ! ( "kernel32.dll""system" fn GetPackagePathByFullName ( packagefullname : :: windows::core::PCWSTR , pathlength : *mut u32 , path : :: windows::core::PWSTR ) -> super::super::super::Foundation:: WIN32_ERROR );
    GetPackagePathByFullName(packagefullname.into().abi(), pathlength, ::core::mem::transmute(path))
}
#[doc = "*Required features: `\"Win32_Storage_Packaging_Appx\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn GetPackagePathByFullName2<P0>(packagefullname: P0, packagepathtype: PackagePathType, pathlength: *mut u32, path: ::windows::core::PWSTR) -> super::super::super::Foundation::WIN32_ERROR
where
    P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
{
    ::windows::core::link ! ( "api-ms-win-appmodel-runtime-l1-1-3.dll""system" fn GetPackagePathByFullName2 ( packagefullname : :: windows::core::PCWSTR , packagepathtype : PackagePathType , pathlength : *mut u32 , path : :: windows::core::PWSTR ) -> super::super::super::Foundation:: WIN32_ERROR );
    GetPackagePathByFullName2(packagefullname.into().abi(), packagepathtype, pathlength, ::core::mem::transmute(path))
}
#[doc = "*Required features: `\"Win32_Storage_Packaging_Appx\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn GetPackagesByPackageFamily<P0>(packagefamilyname: P0, count: *mut u32, packagefullnames: ::core::option::Option<*mut ::windows::core::PWSTR>, bufferlength: *mut u32, buffer: ::windows::core::PWSTR) -> super::super::super::Foundation::WIN32_ERROR
where
    P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
{
    ::windows::core::link ! ( "kernel32.dll""system" fn GetPackagesByPackageFamily ( packagefamilyname : :: windows::core::PCWSTR , count : *mut u32 , packagefullnames : *mut :: windows::core::PWSTR , bufferlength : *mut u32 , buffer : :: windows::core::PWSTR ) -> super::super::super::Foundation:: WIN32_ERROR );
    GetPackagesByPackageFamily(packagefamilyname.into().abi(), count, ::core::mem::transmute(packagefullnames.unwrap_or(::std::ptr::null_mut())), bufferlength, ::core::mem::transmute(buffer))
}
#[doc = "*Required features: `\"Win32_Storage_Packaging_Appx\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn GetProcessesInVirtualizationContext<P0>(packagefamilyname: P0, count: *mut u32, processes: *mut *mut super::super::super::Foundation::HANDLE) -> ::windows::core::Result<()>
where
    P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
{
    ::windows::core::link ! ( "kernel32.dll""system" fn GetProcessesInVirtualizationContext ( packagefamilyname : :: windows::core::PCWSTR , count : *mut u32 , processes : *mut *mut super::super::super::Foundation:: HANDLE ) -> :: windows::core::HRESULT );
    GetProcessesInVirtualizationContext(packagefamilyname.into().abi(), count, processes).ok()
}
#[doc = "*Required features: `\"Win32_Storage_Packaging_Appx\"`*"]
#[inline]
pub unsafe fn GetResolvedPackageFullNameForPackageDependency<P0>(packagedependencyid: P0) -> ::windows::core::Result<::windows::core::PWSTR>
where
    P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
{
    ::windows::core::link ! ( "kernelbase.dll""system" fn GetResolvedPackageFullNameForPackageDependency ( packagedependencyid : :: windows::core::PCWSTR , packagefullname : *mut :: windows::core::PWSTR ) -> :: windows::core::HRESULT );
    let mut result__ = ::core::mem::MaybeUninit::zeroed();
    GetResolvedPackageFullNameForPackageDependency(packagedependencyid.into().abi(), result__.as_mut_ptr()).from_abi(result__)
}
#[doc = "*Required features: `\"Win32_Storage_Packaging_Appx\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn GetStagedPackageOrigin<P0>(packagefullname: P0, origin: *mut PackageOrigin) -> super::super::super::Foundation::WIN32_ERROR
where
    P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
{
    ::windows::core::link ! ( "api-ms-win-appmodel-runtime-l1-1-1.dll""system" fn GetStagedPackageOrigin ( packagefullname : :: windows::core::PCWSTR , origin : *mut PackageOrigin ) -> super::super::super::Foundation:: WIN32_ERROR );
    GetStagedPackageOrigin(packagefullname.into().abi(), origin)
}
#[doc = "*Required features: `\"Win32_Storage_Packaging_Appx\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn GetStagedPackagePathByFullName<P0>(packagefullname: P0, pathlength: *mut u32, path: ::windows::core::PWSTR) -> super::super::super::Foundation::WIN32_ERROR
where
    P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
{
    ::windows::core::link ! ( "kernel32.dll""system" fn GetStagedPackagePathByFullName ( packagefullname : :: windows::core::PCWSTR , pathlength : *mut u32 , path : :: windows::core::PWSTR ) -> super::super::super::Foundation:: WIN32_ERROR );
    GetStagedPackagePathByFullName(packagefullname.into().abi(), pathlength, ::core::mem::transmute(path))
}
#[doc = "*Required features: `\"Win32_Storage_Packaging_Appx\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn GetStagedPackagePathByFullName2<P0>(packagefullname: P0, packagepathtype: PackagePathType, pathlength: *mut u32, path: ::windows::core::PWSTR) -> super::super::super::Foundation::WIN32_ERROR
where
    P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
{
    ::windows::core::link ! ( "api-ms-win-appmodel-runtime-l1-1-3.dll""system" fn GetStagedPackagePathByFullName2 ( packagefullname : :: windows::core::PCWSTR , packagepathtype : PackagePathType , pathlength : *mut u32 , path : :: windows::core::PWSTR ) -> super::super::super::Foundation:: WIN32_ERROR );
    GetStagedPackagePathByFullName2(packagefullname.into().abi(), packagepathtype, pathlength, ::core::mem::transmute(path))
}
#[doc = "*Required features: `\"Win32_Storage_Packaging_Appx\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn OpenPackageInfoByFullName<P0>(packagefullname: P0, reserved: u32, packageinforeference: *mut *mut _PACKAGE_INFO_REFERENCE) -> super::super::super::Foundation::WIN32_ERROR
where
    P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
{
    ::windows::core::link ! ( "kernel32.dll""system" fn OpenPackageInfoByFullName ( packagefullname : :: windows::core::PCWSTR , reserved : u32 , packageinforeference : *mut *mut _PACKAGE_INFO_REFERENCE ) -> super::super::super::Foundation:: WIN32_ERROR );
    OpenPackageInfoByFullName(packagefullname.into().abi(), reserved, packageinforeference)
}
#[doc = "*Required features: `\"Win32_Storage_Packaging_Appx\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn OpenPackageInfoByFullNameForUser<P0, P1>(usersid: P0, packagefullname: P1, reserved: u32, packageinforeference: *mut *mut _PACKAGE_INFO_REFERENCE) -> super::super::super::Foundation::WIN32_ERROR
where
    P0: ::std::convert::Into<super::super::super::Foundation::PSID>,
    P1: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
{
    ::windows::core::link ! ( "api-ms-win-appmodel-runtime-l1-1-1.dll""system" fn OpenPackageInfoByFullNameForUser ( usersid : super::super::super::Foundation:: PSID , packagefullname : :: windows::core::PCWSTR , reserved : u32 , packageinforeference : *mut *mut _PACKAGE_INFO_REFERENCE ) -> super::super::super::Foundation:: WIN32_ERROR );
    OpenPackageInfoByFullNameForUser(usersid.into(), packagefullname.into().abi(), reserved, packageinforeference)
}
#[doc = "*Required features: `\"Win32_Storage_Packaging_Appx\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn PackageFamilyNameFromFullName<P0>(packagefullname: P0, packagefamilynamelength: *mut u32, packagefamilyname: ::windows::core::PWSTR) -> super::super::super::Foundation::WIN32_ERROR
where
    P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
{
    ::windows::core::link ! ( "kernel32.dll""system" fn PackageFamilyNameFromFullName ( packagefullname : :: windows::core::PCWSTR , packagefamilynamelength : *mut u32 , packagefamilyname : :: windows::core::PWSTR ) -> super::super::super::Foundation:: WIN32_ERROR );
    PackageFamilyNameFromFullName(packagefullname.into().abi(), packagefamilynamelength, ::core::mem::transmute(packagefamilyname))
}
#[doc = "*Required features: `\"Win32_Storage_Packaging_Appx\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn PackageFamilyNameFromId(packageid: *const PACKAGE_ID, packagefamilynamelength: *mut u32, packagefamilyname: ::windows::core::PWSTR) -> super::super::super::Foundation::WIN32_ERROR {
    ::windows::core::link ! ( "kernel32.dll""system" fn PackageFamilyNameFromId ( packageid : *const PACKAGE_ID , packagefamilynamelength : *mut u32 , packagefamilyname : :: windows::core::PWSTR ) -> super::super::super::Foundation:: WIN32_ERROR );
    PackageFamilyNameFromId(packageid, packagefamilynamelength, ::core::mem::transmute(packagefamilyname))
}
#[doc = "*Required features: `\"Win32_Storage_Packaging_Appx\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn PackageFullNameFromId(packageid: *const PACKAGE_ID, packagefullnamelength: *mut u32, packagefullname: ::windows::core::PWSTR) -> super::super::super::Foundation::WIN32_ERROR {
    ::windows::core::link ! ( "kernel32.dll""system" fn PackageFullNameFromId ( packageid : *const PACKAGE_ID , packagefullnamelength : *mut u32 , packagefullname : :: windows::core::PWSTR ) -> super::super::super::Foundation:: WIN32_ERROR );
    PackageFullNameFromId(packageid, packagefullnamelength, ::core::mem::transmute(packagefullname))
}
#[doc = "*Required features: `\"Win32_Storage_Packaging_Appx\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn PackageIdFromFullName<P0>(packagefullname: P0, flags: u32, bufferlength: *mut u32, buffer: ::core::option::Option<*mut u8>) -> super::super::super::Foundation::WIN32_ERROR
where
    P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
{
    ::windows::core::link ! ( "kernel32.dll""system" fn PackageIdFromFullName ( packagefullname : :: windows::core::PCWSTR , flags : u32 , bufferlength : *mut u32 , buffer : *mut u8 ) -> super::super::super::Foundation:: WIN32_ERROR );
    PackageIdFromFullName(packagefullname.into().abi(), flags, bufferlength, ::core::mem::transmute(buffer.unwrap_or(::std::ptr::null_mut())))
}
#[doc = "*Required features: `\"Win32_Storage_Packaging_Appx\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn PackageNameAndPublisherIdFromFamilyName<P0>(packagefamilyname: P0, packagenamelength: *mut u32, packagename: ::windows::core::PWSTR, packagepublisheridlength: *mut u32, packagepublisherid: ::windows::core::PWSTR) -> super::super::super::Foundation::WIN32_ERROR
where
    P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
{
    ::windows::core::link ! ( "kernel32.dll""system" fn PackageNameAndPublisherIdFromFamilyName ( packagefamilyname : :: windows::core::PCWSTR , packagenamelength : *mut u32 , packagename : :: windows::core::PWSTR , packagepublisheridlength : *mut u32 , packagepublisherid : :: windows::core::PWSTR ) -> super::super::super::Foundation:: WIN32_ERROR );
    PackageNameAndPublisherIdFromFamilyName(packagefamilyname.into().abi(), packagenamelength, ::core::mem::transmute(packagename), packagepublisheridlength, ::core::mem::transmute(packagepublisherid))
}
#[doc = "*Required features: `\"Win32_Storage_Packaging_Appx\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn ParseApplicationUserModelId<P0>(applicationusermodelid: P0, packagefamilynamelength: *mut u32, packagefamilyname: ::windows::core::PWSTR, packagerelativeapplicationidlength: *mut u32, packagerelativeapplicationid: ::windows::core::PWSTR) -> super::super::super::Foundation::WIN32_ERROR
where
    P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
{
    ::windows::core::link ! ( "kernel32.dll""system" fn ParseApplicationUserModelId ( applicationusermodelid : :: windows::core::PCWSTR , packagefamilynamelength : *mut u32 , packagefamilyname : :: windows::core::PWSTR , packagerelativeapplicationidlength : *mut u32 , packagerelativeapplicationid : :: windows::core::PWSTR ) -> super::super::super::Foundation:: WIN32_ERROR );
    ParseApplicationUserModelId(applicationusermodelid.into().abi(), packagefamilynamelength, ::core::mem::transmute(packagefamilyname), packagerelativeapplicationidlength, ::core::mem::transmute(packagerelativeapplicationid))
}
#[doc = "*Required features: `\"Win32_Storage_Packaging_Appx\"`*"]
#[inline]
pub unsafe fn ReleasePackageVirtualizationContext(context: *const PACKAGE_VIRTUALIZATION_CONTEXT_HANDLE__) {
    ::windows::core::link ! ( "kernel32.dll""system" fn ReleasePackageVirtualizationContext ( context : *const PACKAGE_VIRTUALIZATION_CONTEXT_HANDLE__ ) -> ( ) );
    ReleasePackageVirtualizationContext(context)
}
#[doc = "*Required features: `\"Win32_Storage_Packaging_Appx\"`*"]
#[inline]
pub unsafe fn RemovePackageDependency(packagedependencycontext: *const PACKAGEDEPENDENCY_CONTEXT__) -> ::windows::core::Result<()> {
    ::windows::core::link ! ( "kernelbase.dll""system" fn RemovePackageDependency ( packagedependencycontext : *const PACKAGEDEPENDENCY_CONTEXT__ ) -> :: windows::core::HRESULT );
    RemovePackageDependency(packagedependencycontext).ok()
}
#[doc = "*Required features: `\"Win32_Storage_Packaging_Appx\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn TryCreatePackageDependency<P0, P1, P2>(user: P0, packagefamilyname: P1, minversion: PACKAGE_VERSION, packagedependencyprocessorarchitectures: PackageDependencyProcessorArchitectures, lifetimekind: PackageDependencyLifetimeKind, lifetimeartifact: P2, options: CreatePackageDependencyOptions) -> ::windows::core::Result<::windows::core::PWSTR>
where
    P0: ::std::convert::Into<super::super::super::Foundation::PSID>,
    P1: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
    P2: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
{
    ::windows::core::link ! ( "kernelbase.dll""system" fn TryCreatePackageDependency ( user : super::super::super::Foundation:: PSID , packagefamilyname : :: windows::core::PCWSTR , minversion : PACKAGE_VERSION , packagedependencyprocessorarchitectures : PackageDependencyProcessorArchitectures , lifetimekind : PackageDependencyLifetimeKind , lifetimeartifact : :: windows::core::PCWSTR , options : CreatePackageDependencyOptions , packagedependencyid : *mut :: windows::core::PWSTR ) -> :: windows::core::HRESULT );
    let mut result__ = ::core::mem::MaybeUninit::zeroed();
    TryCreatePackageDependency(user.into(), packagefamilyname.into().abi(), ::core::mem::transmute(minversion), packagedependencyprocessorarchitectures, lifetimekind, lifetimeartifact.into().abi(), options, result__.as_mut_ptr()).from_abi(result__)
}
#[doc = "*Required features: `\"Win32_Storage_Packaging_Appx\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn VerifyApplicationUserModelId<P0>(applicationusermodelid: P0) -> super::super::super::Foundation::WIN32_ERROR
where
    P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
{
    ::windows::core::link ! ( "api-ms-win-appmodel-runtime-l1-1-1.dll""system" fn VerifyApplicationUserModelId ( applicationusermodelid : :: windows::core::PCWSTR ) -> super::super::super::Foundation:: WIN32_ERROR );
    VerifyApplicationUserModelId(applicationusermodelid.into().abi())
}
#[doc = "*Required features: `\"Win32_Storage_Packaging_Appx\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn VerifyPackageFamilyName<P0>(packagefamilyname: P0) -> super::super::super::Foundation::WIN32_ERROR
where
    P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
{
    ::windows::core::link ! ( "api-ms-win-appmodel-runtime-l1-1-1.dll""system" fn VerifyPackageFamilyName ( packagefamilyname : :: windows::core::PCWSTR ) -> super::super::super::Foundation:: WIN32_ERROR );
    VerifyPackageFamilyName(packagefamilyname.into().abi())
}
#[doc = "*Required features: `\"Win32_Storage_Packaging_Appx\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn VerifyPackageFullName<P0>(packagefullname: P0) -> super::super::super::Foundation::WIN32_ERROR
where
    P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
{
    ::windows::core::link ! ( "api-ms-win-appmodel-runtime-l1-1-1.dll""system" fn VerifyPackageFullName ( packagefullname : :: windows::core::PCWSTR ) -> super::super::super::Foundation:: WIN32_ERROR );
    VerifyPackageFullName(packagefullname.into().abi())
}
#[doc = "*Required features: `\"Win32_Storage_Packaging_Appx\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn VerifyPackageId(packageid: *const PACKAGE_ID) -> super::super::super::Foundation::WIN32_ERROR {
    ::windows::core::link ! ( "api-ms-win-appmodel-runtime-l1-1-1.dll""system" fn VerifyPackageId ( packageid : *const PACKAGE_ID ) -> super::super::super::Foundation:: WIN32_ERROR );
    VerifyPackageId(packageid)
}
#[doc = "*Required features: `\"Win32_Storage_Packaging_Appx\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn VerifyPackageRelativeApplicationId<P0>(packagerelativeapplicationid: P0) -> super::super::super::Foundation::WIN32_ERROR
where
    P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
{
    ::windows::core::link ! ( "api-ms-win-appmodel-runtime-l1-1-1.dll""system" fn VerifyPackageRelativeApplicationId ( packagerelativeapplicationid : :: windows::core::PCWSTR ) -> super::super::super::Foundation:: WIN32_ERROR );
    VerifyPackageRelativeApplicationId(packagerelativeapplicationid.into().abi())
}
#[doc = "*Required features: `\"Win32_Storage_Packaging_Appx\"`*"]
#[repr(transparent)]
pub struct IAppxBlockMapBlock(::windows::core::IUnknown);
impl IAppxBlockMapBlock {
    pub unsafe fn GetHash(&self, buffersize: *mut u32, buffer: *mut *mut u8) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).GetHash)(::windows::core::Vtable::as_raw(self), buffersize, buffer).ok()
    }
    pub unsafe fn GetCompressedSize(&self) -> ::windows::core::Result<u32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).GetCompressedSize)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
}
::windows::core::interface_hierarchy!(IAppxBlockMapBlock, ::windows::core::IUnknown);
impl ::core::clone::Clone for IAppxBlockMapBlock {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IAppxBlockMapBlock {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IAppxBlockMapBlock {}
impl ::core::fmt::Debug for IAppxBlockMapBlock {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IAppxBlockMapBlock").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Vtable for IAppxBlockMapBlock {
    type Vtable = IAppxBlockMapBlock_Vtbl;
}
unsafe impl ::windows::core::Interface for IAppxBlockMapBlock {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x75cf3930_3244_4fe0_a8c8_e0bcb270b889);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppxBlockMapBlock_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub GetHash: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, buffersize: *mut u32, buffer: *mut *mut u8) -> ::windows::core::HRESULT,
    pub GetCompressedSize: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, size: *mut u32) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Storage_Packaging_Appx\"`*"]
#[repr(transparent)]
pub struct IAppxBlockMapBlocksEnumerator(::windows::core::IUnknown);
impl IAppxBlockMapBlocksEnumerator {
    pub unsafe fn GetCurrent(&self) -> ::windows::core::Result<IAppxBlockMapBlock> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).GetCurrent)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetHasCurrent(&self) -> ::windows::core::Result<super::super::super::Foundation::BOOL> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).GetHasCurrent)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn MoveNext(&self) -> ::windows::core::Result<super::super::super::Foundation::BOOL> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).MoveNext)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
}
::windows::core::interface_hierarchy!(IAppxBlockMapBlocksEnumerator, ::windows::core::IUnknown);
impl ::core::clone::Clone for IAppxBlockMapBlocksEnumerator {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IAppxBlockMapBlocksEnumerator {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IAppxBlockMapBlocksEnumerator {}
impl ::core::fmt::Debug for IAppxBlockMapBlocksEnumerator {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IAppxBlockMapBlocksEnumerator").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Vtable for IAppxBlockMapBlocksEnumerator {
    type Vtable = IAppxBlockMapBlocksEnumerator_Vtbl;
}
unsafe impl ::windows::core::Interface for IAppxBlockMapBlocksEnumerator {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x6b429b5b_36ef_479e_b9eb_0c1482b49e16);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppxBlockMapBlocksEnumerator_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub GetCurrent: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, block: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub GetHasCurrent: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, hascurrent: *mut super::super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetHasCurrent: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub MoveNext: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, hasnext: *mut super::super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    MoveNext: usize,
}
#[doc = "*Required features: `\"Win32_Storage_Packaging_Appx\"`*"]
#[repr(transparent)]
pub struct IAppxBlockMapFile(::windows::core::IUnknown);
impl IAppxBlockMapFile {
    pub unsafe fn GetBlocks(&self) -> ::windows::core::Result<IAppxBlockMapBlocksEnumerator> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).GetBlocks)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetLocalFileHeaderSize(&self) -> ::windows::core::Result<u32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).GetLocalFileHeaderSize)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetName(&self) -> ::windows::core::Result<::windows::core::PWSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).GetName)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetUncompressedSize(&self) -> ::windows::core::Result<u64> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).GetUncompressedSize)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub unsafe fn ValidateFileHash<P0>(&self, filestream: P0) -> ::windows::core::Result<super::super::super::Foundation::BOOL>
    where
        P0: ::std::convert::Into<::windows::core::InParam<super::super::super::System::Com::IStream>>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).ValidateFileHash)(::windows::core::Vtable::as_raw(self), filestream.into().abi(), result__.as_mut_ptr()).from_abi(result__)
    }
}
::windows::core::interface_hierarchy!(IAppxBlockMapFile, ::windows::core::IUnknown);
impl ::core::clone::Clone for IAppxBlockMapFile {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IAppxBlockMapFile {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IAppxBlockMapFile {}
impl ::core::fmt::Debug for IAppxBlockMapFile {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IAppxBlockMapFile").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Vtable for IAppxBlockMapFile {
    type Vtable = IAppxBlockMapFile_Vtbl;
}
unsafe impl ::windows::core::Interface for IAppxBlockMapFile {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x277672ac_4f63_42c1_8abc_beae3600eb59);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppxBlockMapFile_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub GetBlocks: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, blocks: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub GetLocalFileHeaderSize: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, lfhsize: *mut u32) -> ::windows::core::HRESULT,
    pub GetName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, name: *mut ::windows::core::PWSTR) -> ::windows::core::HRESULT,
    pub GetUncompressedSize: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, size: *mut u64) -> ::windows::core::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub ValidateFileHash: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, filestream: *mut ::core::ffi::c_void, isvalid: *mut super::super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com")))]
    ValidateFileHash: usize,
}
#[doc = "*Required features: `\"Win32_Storage_Packaging_Appx\"`*"]
#[repr(transparent)]
pub struct IAppxBlockMapFilesEnumerator(::windows::core::IUnknown);
impl IAppxBlockMapFilesEnumerator {
    pub unsafe fn GetCurrent(&self) -> ::windows::core::Result<IAppxBlockMapFile> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).GetCurrent)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetHasCurrent(&self) -> ::windows::core::Result<super::super::super::Foundation::BOOL> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).GetHasCurrent)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn MoveNext(&self) -> ::windows::core::Result<super::super::super::Foundation::BOOL> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).MoveNext)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
}
::windows::core::interface_hierarchy!(IAppxBlockMapFilesEnumerator, ::windows::core::IUnknown);
impl ::core::clone::Clone for IAppxBlockMapFilesEnumerator {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IAppxBlockMapFilesEnumerator {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IAppxBlockMapFilesEnumerator {}
impl ::core::fmt::Debug for IAppxBlockMapFilesEnumerator {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IAppxBlockMapFilesEnumerator").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Vtable for IAppxBlockMapFilesEnumerator {
    type Vtable = IAppxBlockMapFilesEnumerator_Vtbl;
}
unsafe impl ::windows::core::Interface for IAppxBlockMapFilesEnumerator {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x02b856a2_4262_4070_bacb_1a8cbbc42305);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppxBlockMapFilesEnumerator_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub GetCurrent: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, file: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub GetHasCurrent: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, hascurrent: *mut super::super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetHasCurrent: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub MoveNext: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, hascurrent: *mut super::super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    MoveNext: usize,
}
#[doc = "*Required features: `\"Win32_Storage_Packaging_Appx\"`*"]
#[repr(transparent)]
pub struct IAppxBlockMapReader(::windows::core::IUnknown);
impl IAppxBlockMapReader {
    pub unsafe fn GetFile<P0>(&self, filename: P0) -> ::windows::core::Result<IAppxBlockMapFile>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).GetFile)(::windows::core::Vtable::as_raw(self), filename.into().abi(), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetFiles(&self) -> ::windows::core::Result<IAppxBlockMapFilesEnumerator> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).GetFiles)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetHashMethod(&self) -> ::windows::core::Result<super::super::super::System::Com::IUri> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).GetHashMethod)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetStream(&self) -> ::windows::core::Result<super::super::super::System::Com::IStream> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).GetStream)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
}
::windows::core::interface_hierarchy!(IAppxBlockMapReader, ::windows::core::IUnknown);
impl ::core::clone::Clone for IAppxBlockMapReader {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IAppxBlockMapReader {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IAppxBlockMapReader {}
impl ::core::fmt::Debug for IAppxBlockMapReader {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IAppxBlockMapReader").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Vtable for IAppxBlockMapReader {
    type Vtable = IAppxBlockMapReader_Vtbl;
}
unsafe impl ::windows::core::Interface for IAppxBlockMapReader {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x5efec991_bca3_42d1_9ec2_e92d609ec22a);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppxBlockMapReader_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub GetFile: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, filename: ::windows::core::PCWSTR, file: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub GetFiles: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, enumerator: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub GetHashMethod: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, hashmethod: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    GetHashMethod: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub GetStream: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, blockmapstream: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    GetStream: usize,
}
#[doc = "*Required features: `\"Win32_Storage_Packaging_Appx\"`*"]
#[repr(transparent)]
pub struct IAppxBundleFactory(::windows::core::IUnknown);
impl IAppxBundleFactory {
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn CreateBundleWriter<P0>(&self, outputstream: P0, bundleversion: u64) -> ::windows::core::Result<IAppxBundleWriter>
    where
        P0: ::std::convert::Into<::windows::core::InParam<super::super::super::System::Com::IStream>>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).CreateBundleWriter)(::windows::core::Vtable::as_raw(self), outputstream.into().abi(), bundleversion, result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn CreateBundleReader<P0>(&self, inputstream: P0) -> ::windows::core::Result<IAppxBundleReader>
    where
        P0: ::std::convert::Into<::windows::core::InParam<super::super::super::System::Com::IStream>>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).CreateBundleReader)(::windows::core::Vtable::as_raw(self), inputstream.into().abi(), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn CreateBundleManifestReader<P0>(&self, inputstream: P0) -> ::windows::core::Result<IAppxBundleManifestReader>
    where
        P0: ::std::convert::Into<::windows::core::InParam<super::super::super::System::Com::IStream>>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).CreateBundleManifestReader)(::windows::core::Vtable::as_raw(self), inputstream.into().abi(), result__.as_mut_ptr()).from_abi(result__)
    }
}
::windows::core::interface_hierarchy!(IAppxBundleFactory, ::windows::core::IUnknown);
impl ::core::clone::Clone for IAppxBundleFactory {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IAppxBundleFactory {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IAppxBundleFactory {}
impl ::core::fmt::Debug for IAppxBundleFactory {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IAppxBundleFactory").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Vtable for IAppxBundleFactory {
    type Vtable = IAppxBundleFactory_Vtbl;
}
unsafe impl ::windows::core::Interface for IAppxBundleFactory {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xbba65864_965f_4a5f_855f_f074bdbf3a7b);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppxBundleFactory_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    #[cfg(feature = "Win32_System_Com")]
    pub CreateBundleWriter: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, outputstream: *mut ::core::ffi::c_void, bundleversion: u64, bundlewriter: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    CreateBundleWriter: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub CreateBundleReader: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, inputstream: *mut ::core::ffi::c_void, bundlereader: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    CreateBundleReader: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub CreateBundleManifestReader: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, inputstream: *mut ::core::ffi::c_void, manifestreader: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    CreateBundleManifestReader: usize,
}
#[doc = "*Required features: `\"Win32_Storage_Packaging_Appx\"`*"]
#[repr(transparent)]
pub struct IAppxBundleManifestOptionalBundleInfo(::windows::core::IUnknown);
impl IAppxBundleManifestOptionalBundleInfo {
    pub unsafe fn GetPackageId(&self) -> ::windows::core::Result<IAppxManifestPackageId> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).GetPackageId)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetFileName(&self) -> ::windows::core::Result<::windows::core::PWSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).GetFileName)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetPackageInfoItems(&self) -> ::windows::core::Result<IAppxBundleManifestPackageInfoEnumerator> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).GetPackageInfoItems)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
}
::windows::core::interface_hierarchy!(IAppxBundleManifestOptionalBundleInfo, ::windows::core::IUnknown);
impl ::core::clone::Clone for IAppxBundleManifestOptionalBundleInfo {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IAppxBundleManifestOptionalBundleInfo {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IAppxBundleManifestOptionalBundleInfo {}
impl ::core::fmt::Debug for IAppxBundleManifestOptionalBundleInfo {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IAppxBundleManifestOptionalBundleInfo").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Vtable for IAppxBundleManifestOptionalBundleInfo {
    type Vtable = IAppxBundleManifestOptionalBundleInfo_Vtbl;
}
unsafe impl ::windows::core::Interface for IAppxBundleManifestOptionalBundleInfo {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x515bf2e8_bcb0_4d69_8c48_e383147b6e12);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppxBundleManifestOptionalBundleInfo_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub GetPackageId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, packageid: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub GetFileName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, filename: *mut ::windows::core::PWSTR) -> ::windows::core::HRESULT,
    pub GetPackageInfoItems: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, packageinfoitems: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Storage_Packaging_Appx\"`*"]
#[repr(transparent)]
pub struct IAppxBundleManifestOptionalBundleInfoEnumerator(::windows::core::IUnknown);
impl IAppxBundleManifestOptionalBundleInfoEnumerator {
    pub unsafe fn GetCurrent(&self) -> ::windows::core::Result<IAppxBundleManifestOptionalBundleInfo> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).GetCurrent)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetHasCurrent(&self) -> ::windows::core::Result<super::super::super::Foundation::BOOL> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).GetHasCurrent)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn MoveNext(&self) -> ::windows::core::Result<super::super::super::Foundation::BOOL> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).MoveNext)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
}
::windows::core::interface_hierarchy!(IAppxBundleManifestOptionalBundleInfoEnumerator, ::windows::core::IUnknown);
impl ::core::clone::Clone for IAppxBundleManifestOptionalBundleInfoEnumerator {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IAppxBundleManifestOptionalBundleInfoEnumerator {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IAppxBundleManifestOptionalBundleInfoEnumerator {}
impl ::core::fmt::Debug for IAppxBundleManifestOptionalBundleInfoEnumerator {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IAppxBundleManifestOptionalBundleInfoEnumerator").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Vtable for IAppxBundleManifestOptionalBundleInfoEnumerator {
    type Vtable = IAppxBundleManifestOptionalBundleInfoEnumerator_Vtbl;
}
unsafe impl ::windows::core::Interface for IAppxBundleManifestOptionalBundleInfoEnumerator {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x9a178793_f97e_46ac_aaca_dd5ba4c177c8);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppxBundleManifestOptionalBundleInfoEnumerator_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub GetCurrent: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, optionalbundle: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub GetHasCurrent: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, hascurrent: *mut super::super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetHasCurrent: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub MoveNext: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, hasnext: *mut super::super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    MoveNext: usize,
}
#[doc = "*Required features: `\"Win32_Storage_Packaging_Appx\"`*"]
#[repr(transparent)]
pub struct IAppxBundleManifestPackageInfo(::windows::core::IUnknown);
impl IAppxBundleManifestPackageInfo {
    pub unsafe fn GetPackageType(&self) -> ::windows::core::Result<APPX_BUNDLE_PAYLOAD_PACKAGE_TYPE> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).GetPackageType)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetPackageId(&self) -> ::windows::core::Result<IAppxManifestPackageId> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).GetPackageId)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetFileName(&self) -> ::windows::core::Result<::windows::core::PWSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).GetFileName)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetOffset(&self) -> ::windows::core::Result<u64> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).GetOffset)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetSize(&self) -> ::windows::core::Result<u64> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).GetSize)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetResources(&self) -> ::windows::core::Result<IAppxManifestQualifiedResourcesEnumerator> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).GetResources)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
}
::windows::core::interface_hierarchy!(IAppxBundleManifestPackageInfo, ::windows::core::IUnknown);
impl ::core::clone::Clone for IAppxBundleManifestPackageInfo {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IAppxBundleManifestPackageInfo {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IAppxBundleManifestPackageInfo {}
impl ::core::fmt::Debug for IAppxBundleManifestPackageInfo {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IAppxBundleManifestPackageInfo").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Vtable for IAppxBundleManifestPackageInfo {
    type Vtable = IAppxBundleManifestPackageInfo_Vtbl;
}
unsafe impl ::windows::core::Interface for IAppxBundleManifestPackageInfo {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x54cd06c1_268f_40bb_8ed2_757a9ebaec8d);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppxBundleManifestPackageInfo_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub GetPackageType: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, packagetype: *mut APPX_BUNDLE_PAYLOAD_PACKAGE_TYPE) -> ::windows::core::HRESULT,
    pub GetPackageId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, packageid: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub GetFileName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, filename: *mut ::windows::core::PWSTR) -> ::windows::core::HRESULT,
    pub GetOffset: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, offset: *mut u64) -> ::windows::core::HRESULT,
    pub GetSize: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, size: *mut u64) -> ::windows::core::HRESULT,
    pub GetResources: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, resources: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Storage_Packaging_Appx\"`*"]
#[repr(transparent)]
pub struct IAppxBundleManifestPackageInfo2(::windows::core::IUnknown);
impl IAppxBundleManifestPackageInfo2 {
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetIsPackageReference(&self) -> ::windows::core::Result<super::super::super::Foundation::BOOL> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).GetIsPackageReference)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetIsNonQualifiedResourcePackage(&self) -> ::windows::core::Result<super::super::super::Foundation::BOOL> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).GetIsNonQualifiedResourcePackage)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetIsDefaultApplicablePackage(&self) -> ::windows::core::Result<super::super::super::Foundation::BOOL> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).GetIsDefaultApplicablePackage)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
}
::windows::core::interface_hierarchy!(IAppxBundleManifestPackageInfo2, ::windows::core::IUnknown);
impl ::core::clone::Clone for IAppxBundleManifestPackageInfo2 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IAppxBundleManifestPackageInfo2 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IAppxBundleManifestPackageInfo2 {}
impl ::core::fmt::Debug for IAppxBundleManifestPackageInfo2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IAppxBundleManifestPackageInfo2").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Vtable for IAppxBundleManifestPackageInfo2 {
    type Vtable = IAppxBundleManifestPackageInfo2_Vtbl;
}
unsafe impl ::windows::core::Interface for IAppxBundleManifestPackageInfo2 {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x44c2acbc_b2cf_4ccb_bbdb_9c6da8c3bc9e);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppxBundleManifestPackageInfo2_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    #[cfg(feature = "Win32_Foundation")]
    pub GetIsPackageReference: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ispackagereference: *mut super::super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetIsPackageReference: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub GetIsNonQualifiedResourcePackage: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, isnonqualifiedresourcepackage: *mut super::super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetIsNonQualifiedResourcePackage: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub GetIsDefaultApplicablePackage: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, isdefaultapplicablepackage: *mut super::super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetIsDefaultApplicablePackage: usize,
}
#[doc = "*Required features: `\"Win32_Storage_Packaging_Appx\"`*"]
#[repr(transparent)]
pub struct IAppxBundleManifestPackageInfo3(::windows::core::IUnknown);
impl IAppxBundleManifestPackageInfo3 {
    pub unsafe fn GetTargetDeviceFamilies(&self) -> ::windows::core::Result<IAppxManifestTargetDeviceFamiliesEnumerator> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).GetTargetDeviceFamilies)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
}
::windows::core::interface_hierarchy!(IAppxBundleManifestPackageInfo3, ::windows::core::IUnknown);
impl ::core::clone::Clone for IAppxBundleManifestPackageInfo3 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IAppxBundleManifestPackageInfo3 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IAppxBundleManifestPackageInfo3 {}
impl ::core::fmt::Debug for IAppxBundleManifestPackageInfo3 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IAppxBundleManifestPackageInfo3").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Vtable for IAppxBundleManifestPackageInfo3 {
    type Vtable = IAppxBundleManifestPackageInfo3_Vtbl;
}
unsafe impl ::windows::core::Interface for IAppxBundleManifestPackageInfo3 {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x6ba74b98_bb74_4296_80d0_5f4256a99675);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppxBundleManifestPackageInfo3_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub GetTargetDeviceFamilies: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, targetdevicefamilies: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Storage_Packaging_Appx\"`*"]
#[repr(transparent)]
pub struct IAppxBundleManifestPackageInfo4(::windows::core::IUnknown);
impl IAppxBundleManifestPackageInfo4 {
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetIsStub(&self) -> ::windows::core::Result<super::super::super::Foundation::BOOL> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).GetIsStub)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
}
::windows::core::interface_hierarchy!(IAppxBundleManifestPackageInfo4, ::windows::core::IUnknown);
impl ::core::clone::Clone for IAppxBundleManifestPackageInfo4 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IAppxBundleManifestPackageInfo4 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IAppxBundleManifestPackageInfo4 {}
impl ::core::fmt::Debug for IAppxBundleManifestPackageInfo4 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IAppxBundleManifestPackageInfo4").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Vtable for IAppxBundleManifestPackageInfo4 {
    type Vtable = IAppxBundleManifestPackageInfo4_Vtbl;
}
unsafe impl ::windows::core::Interface for IAppxBundleManifestPackageInfo4 {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x5da6f13d_a8a7_4532_857c_1393d659371d);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppxBundleManifestPackageInfo4_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    #[cfg(feature = "Win32_Foundation")]
    pub GetIsStub: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, isstub: *mut super::super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetIsStub: usize,
}
#[doc = "*Required features: `\"Win32_Storage_Packaging_Appx\"`*"]
#[repr(transparent)]
pub struct IAppxBundleManifestPackageInfoEnumerator(::windows::core::IUnknown);
impl IAppxBundleManifestPackageInfoEnumerator {
    pub unsafe fn GetCurrent(&self) -> ::windows::core::Result<IAppxBundleManifestPackageInfo> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).GetCurrent)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetHasCurrent(&self) -> ::windows::core::Result<super::super::super::Foundation::BOOL> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).GetHasCurrent)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn MoveNext(&self) -> ::windows::core::Result<super::super::super::Foundation::BOOL> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).MoveNext)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
}
::windows::core::interface_hierarchy!(IAppxBundleManifestPackageInfoEnumerator, ::windows::core::IUnknown);
impl ::core::clone::Clone for IAppxBundleManifestPackageInfoEnumerator {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IAppxBundleManifestPackageInfoEnumerator {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IAppxBundleManifestPackageInfoEnumerator {}
impl ::core::fmt::Debug for IAppxBundleManifestPackageInfoEnumerator {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IAppxBundleManifestPackageInfoEnumerator").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Vtable for IAppxBundleManifestPackageInfoEnumerator {
    type Vtable = IAppxBundleManifestPackageInfoEnumerator_Vtbl;
}
unsafe impl ::windows::core::Interface for IAppxBundleManifestPackageInfoEnumerator {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xf9b856ee_49a6_4e19_b2b0_6a2406d63a32);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppxBundleManifestPackageInfoEnumerator_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub GetCurrent: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, packageinfo: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub GetHasCurrent: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, hascurrent: *mut super::super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetHasCurrent: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub MoveNext: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, hasnext: *mut super::super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    MoveNext: usize,
}
#[doc = "*Required features: `\"Win32_Storage_Packaging_Appx\"`*"]
#[repr(transparent)]
pub struct IAppxBundleManifestReader(::windows::core::IUnknown);
impl IAppxBundleManifestReader {
    pub unsafe fn GetPackageId(&self) -> ::windows::core::Result<IAppxManifestPackageId> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).GetPackageId)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetPackageInfoItems(&self) -> ::windows::core::Result<IAppxBundleManifestPackageInfoEnumerator> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).GetPackageInfoItems)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetStream(&self) -> ::windows::core::Result<super::super::super::System::Com::IStream> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).GetStream)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
}
::windows::core::interface_hierarchy!(IAppxBundleManifestReader, ::windows::core::IUnknown);
impl ::core::clone::Clone for IAppxBundleManifestReader {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IAppxBundleManifestReader {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IAppxBundleManifestReader {}
impl ::core::fmt::Debug for IAppxBundleManifestReader {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IAppxBundleManifestReader").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Vtable for IAppxBundleManifestReader {
    type Vtable = IAppxBundleManifestReader_Vtbl;
}
unsafe impl ::windows::core::Interface for IAppxBundleManifestReader {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xcf0ebbc1_cc99_4106_91eb_e67462e04fb0);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppxBundleManifestReader_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub GetPackageId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, packageid: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub GetPackageInfoItems: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, packageinfoitems: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub GetStream: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, manifeststream: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    GetStream: usize,
}
#[doc = "*Required features: `\"Win32_Storage_Packaging_Appx\"`*"]
#[repr(transparent)]
pub struct IAppxBundleManifestReader2(::windows::core::IUnknown);
impl IAppxBundleManifestReader2 {
    pub unsafe fn GetOptionalBundles(&self) -> ::windows::core::Result<IAppxBundleManifestOptionalBundleInfoEnumerator> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).GetOptionalBundles)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
}
::windows::core::interface_hierarchy!(IAppxBundleManifestReader2, ::windows::core::IUnknown);
impl ::core::clone::Clone for IAppxBundleManifestReader2 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IAppxBundleManifestReader2 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IAppxBundleManifestReader2 {}
impl ::core::fmt::Debug for IAppxBundleManifestReader2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IAppxBundleManifestReader2").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Vtable for IAppxBundleManifestReader2 {
    type Vtable = IAppxBundleManifestReader2_Vtbl;
}
unsafe impl ::windows::core::Interface for IAppxBundleManifestReader2 {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x5517df70_033f_4af2_8213_87d766805c02);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppxBundleManifestReader2_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub GetOptionalBundles: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, optionalbundles: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Storage_Packaging_Appx\"`*"]
#[repr(transparent)]
pub struct IAppxBundleReader(::windows::core::IUnknown);
impl IAppxBundleReader {
    pub unsafe fn GetFootprintFile(&self, filetype: APPX_BUNDLE_FOOTPRINT_FILE_TYPE) -> ::windows::core::Result<IAppxFile> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).GetFootprintFile)(::windows::core::Vtable::as_raw(self), filetype, result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetBlockMap(&self) -> ::windows::core::Result<IAppxBlockMapReader> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).GetBlockMap)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetManifest(&self) -> ::windows::core::Result<IAppxBundleManifestReader> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).GetManifest)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetPayloadPackages(&self) -> ::windows::core::Result<IAppxFilesEnumerator> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).GetPayloadPackages)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetPayloadPackage<P0>(&self, filename: P0) -> ::windows::core::Result<IAppxFile>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).GetPayloadPackage)(::windows::core::Vtable::as_raw(self), filename.into().abi(), result__.as_mut_ptr()).from_abi(result__)
    }
}
::windows::core::interface_hierarchy!(IAppxBundleReader, ::windows::core::IUnknown);
impl ::core::clone::Clone for IAppxBundleReader {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IAppxBundleReader {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IAppxBundleReader {}
impl ::core::fmt::Debug for IAppxBundleReader {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IAppxBundleReader").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Vtable for IAppxBundleReader {
    type Vtable = IAppxBundleReader_Vtbl;
}
unsafe impl ::windows::core::Interface for IAppxBundleReader {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xdd75b8c0_ba76_43b0_ae0f_68656a1dc5c8);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppxBundleReader_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub GetFootprintFile: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, filetype: APPX_BUNDLE_FOOTPRINT_FILE_TYPE, footprintfile: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub GetBlockMap: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, blockmapreader: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub GetManifest: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, manifestreader: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub GetPayloadPackages: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, payloadpackages: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub GetPayloadPackage: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, filename: ::windows::core::PCWSTR, payloadpackage: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Storage_Packaging_Appx\"`*"]
#[repr(transparent)]
pub struct IAppxBundleWriter(::windows::core::IUnknown);
impl IAppxBundleWriter {
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn AddPayloadPackage<P0, P1>(&self, filename: P0, packagestream: P1) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
        P1: ::std::convert::Into<::windows::core::InParam<super::super::super::System::Com::IStream>>,
    {
        (::windows::core::Vtable::vtable(self).AddPayloadPackage)(::windows::core::Vtable::as_raw(self), filename.into().abi(), packagestream.into().abi()).ok()
    }
    pub unsafe fn Close(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).Close)(::windows::core::Vtable::as_raw(self)).ok()
    }
}
::windows::core::interface_hierarchy!(IAppxBundleWriter, ::windows::core::IUnknown);
impl ::core::clone::Clone for IAppxBundleWriter {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IAppxBundleWriter {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IAppxBundleWriter {}
impl ::core::fmt::Debug for IAppxBundleWriter {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IAppxBundleWriter").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Vtable for IAppxBundleWriter {
    type Vtable = IAppxBundleWriter_Vtbl;
}
unsafe impl ::windows::core::Interface for IAppxBundleWriter {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xec446fe8_bfec_4c64_ab4f_49f038f0c6d2);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppxBundleWriter_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    #[cfg(feature = "Win32_System_Com")]
    pub AddPayloadPackage: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, filename: ::windows::core::PCWSTR, packagestream: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    AddPayloadPackage: usize,
    pub Close: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Storage_Packaging_Appx\"`*"]
#[repr(transparent)]
pub struct IAppxBundleWriter2(::windows::core::IUnknown);
impl IAppxBundleWriter2 {
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn AddExternalPackageReference<P0, P1>(&self, filename: P0, inputstream: P1) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
        P1: ::std::convert::Into<::windows::core::InParam<super::super::super::System::Com::IStream>>,
    {
        (::windows::core::Vtable::vtable(self).AddExternalPackageReference)(::windows::core::Vtable::as_raw(self), filename.into().abi(), inputstream.into().abi()).ok()
    }
}
::windows::core::interface_hierarchy!(IAppxBundleWriter2, ::windows::core::IUnknown);
impl ::core::clone::Clone for IAppxBundleWriter2 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IAppxBundleWriter2 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IAppxBundleWriter2 {}
impl ::core::fmt::Debug for IAppxBundleWriter2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IAppxBundleWriter2").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Vtable for IAppxBundleWriter2 {
    type Vtable = IAppxBundleWriter2_Vtbl;
}
unsafe impl ::windows::core::Interface for IAppxBundleWriter2 {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x6d8fe971_01cc_49a0_b685_233851279962);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppxBundleWriter2_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    #[cfg(feature = "Win32_System_Com")]
    pub AddExternalPackageReference: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, filename: ::windows::core::PCWSTR, inputstream: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    AddExternalPackageReference: usize,
}
#[doc = "*Required features: `\"Win32_Storage_Packaging_Appx\"`*"]
#[repr(transparent)]
pub struct IAppxBundleWriter3(::windows::core::IUnknown);
impl IAppxBundleWriter3 {
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn AddPackageReference<P0, P1>(&self, filename: P0, inputstream: P1) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
        P1: ::std::convert::Into<::windows::core::InParam<super::super::super::System::Com::IStream>>,
    {
        (::windows::core::Vtable::vtable(self).AddPackageReference)(::windows::core::Vtable::as_raw(self), filename.into().abi(), inputstream.into().abi()).ok()
    }
    pub unsafe fn Close<P0>(&self, hashmethodstring: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
    {
        (::windows::core::Vtable::vtable(self).Close)(::windows::core::Vtable::as_raw(self), hashmethodstring.into().abi()).ok()
    }
}
::windows::core::interface_hierarchy!(IAppxBundleWriter3, ::windows::core::IUnknown);
impl ::core::clone::Clone for IAppxBundleWriter3 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IAppxBundleWriter3 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IAppxBundleWriter3 {}
impl ::core::fmt::Debug for IAppxBundleWriter3 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IAppxBundleWriter3").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Vtable for IAppxBundleWriter3 {
    type Vtable = IAppxBundleWriter3_Vtbl;
}
unsafe impl ::windows::core::Interface for IAppxBundleWriter3 {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xad711152_f969_4193_82d5_9ddf2786d21a);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppxBundleWriter3_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    #[cfg(feature = "Win32_System_Com")]
    pub AddPackageReference: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, filename: ::windows::core::PCWSTR, inputstream: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    AddPackageReference: usize,
    pub Close: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, hashmethodstring: ::windows::core::PCWSTR) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Storage_Packaging_Appx\"`*"]
#[repr(transparent)]
pub struct IAppxBundleWriter4(::windows::core::IUnknown);
impl IAppxBundleWriter4 {
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub unsafe fn AddPayloadPackage<P0, P1, P2>(&self, filename: P0, packagestream: P1, isdefaultapplicablepackage: P2) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
        P1: ::std::convert::Into<::windows::core::InParam<super::super::super::System::Com::IStream>>,
        P2: ::std::convert::Into<super::super::super::Foundation::BOOL>,
    {
        (::windows::core::Vtable::vtable(self).AddPayloadPackage)(::windows::core::Vtable::as_raw(self), filename.into().abi(), packagestream.into().abi(), isdefaultapplicablepackage.into()).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub unsafe fn AddPackageReference<P0, P1, P2>(&self, filename: P0, inputstream: P1, isdefaultapplicablepackage: P2) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
        P1: ::std::convert::Into<::windows::core::InParam<super::super::super::System::Com::IStream>>,
        P2: ::std::convert::Into<super::super::super::Foundation::BOOL>,
    {
        (::windows::core::Vtable::vtable(self).AddPackageReference)(::windows::core::Vtable::as_raw(self), filename.into().abi(), inputstream.into().abi(), isdefaultapplicablepackage.into()).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub unsafe fn AddExternalPackageReference<P0, P1, P2>(&self, filename: P0, inputstream: P1, isdefaultapplicablepackage: P2) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
        P1: ::std::convert::Into<::windows::core::InParam<super::super::super::System::Com::IStream>>,
        P2: ::std::convert::Into<super::super::super::Foundation::BOOL>,
    {
        (::windows::core::Vtable::vtable(self).AddExternalPackageReference)(::windows::core::Vtable::as_raw(self), filename.into().abi(), inputstream.into().abi(), isdefaultapplicablepackage.into()).ok()
    }
}
::windows::core::interface_hierarchy!(IAppxBundleWriter4, ::windows::core::IUnknown);
impl ::core::clone::Clone for IAppxBundleWriter4 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IAppxBundleWriter4 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IAppxBundleWriter4 {}
impl ::core::fmt::Debug for IAppxBundleWriter4 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IAppxBundleWriter4").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Vtable for IAppxBundleWriter4 {
    type Vtable = IAppxBundleWriter4_Vtbl;
}
unsafe impl ::windows::core::Interface for IAppxBundleWriter4 {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x9cd9d523_5009_4c01_9882_dc029fbd47a3);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppxBundleWriter4_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub AddPayloadPackage: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, filename: ::windows::core::PCWSTR, packagestream: *mut ::core::ffi::c_void, isdefaultapplicablepackage: super::super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com")))]
    AddPayloadPackage: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub AddPackageReference: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, filename: ::windows::core::PCWSTR, inputstream: *mut ::core::ffi::c_void, isdefaultapplicablepackage: super::super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com")))]
    AddPackageReference: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub AddExternalPackageReference: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, filename: ::windows::core::PCWSTR, inputstream: *mut ::core::ffi::c_void, isdefaultapplicablepackage: super::super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com")))]
    AddExternalPackageReference: usize,
}
#[doc = "*Required features: `\"Win32_Storage_Packaging_Appx\"`*"]
#[repr(transparent)]
pub struct IAppxContentGroup(::windows::core::IUnknown);
impl IAppxContentGroup {
    pub unsafe fn GetName(&self) -> ::windows::core::Result<::windows::core::PWSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).GetName)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetFiles(&self) -> ::windows::core::Result<IAppxContentGroupFilesEnumerator> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).GetFiles)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
}
::windows::core::interface_hierarchy!(IAppxContentGroup, ::windows::core::IUnknown);
impl ::core::clone::Clone for IAppxContentGroup {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IAppxContentGroup {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IAppxContentGroup {}
impl ::core::fmt::Debug for IAppxContentGroup {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IAppxContentGroup").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Vtable for IAppxContentGroup {
    type Vtable = IAppxContentGroup_Vtbl;
}
unsafe impl ::windows::core::Interface for IAppxContentGroup {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x328f6468_c04f_4e3c_b6fa_6b8d27f3003a);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppxContentGroup_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub GetName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, groupname: *mut ::windows::core::PWSTR) -> ::windows::core::HRESULT,
    pub GetFiles: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, enumerator: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Storage_Packaging_Appx\"`*"]
#[repr(transparent)]
pub struct IAppxContentGroupFilesEnumerator(::windows::core::IUnknown);
impl IAppxContentGroupFilesEnumerator {
    pub unsafe fn GetCurrent(&self) -> ::windows::core::Result<::windows::core::PWSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).GetCurrent)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetHasCurrent(&self) -> ::windows::core::Result<super::super::super::Foundation::BOOL> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).GetHasCurrent)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn MoveNext(&self) -> ::windows::core::Result<super::super::super::Foundation::BOOL> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).MoveNext)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
}
::windows::core::interface_hierarchy!(IAppxContentGroupFilesEnumerator, ::windows::core::IUnknown);
impl ::core::clone::Clone for IAppxContentGroupFilesEnumerator {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IAppxContentGroupFilesEnumerator {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IAppxContentGroupFilesEnumerator {}
impl ::core::fmt::Debug for IAppxContentGroupFilesEnumerator {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IAppxContentGroupFilesEnumerator").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Vtable for IAppxContentGroupFilesEnumerator {
    type Vtable = IAppxContentGroupFilesEnumerator_Vtbl;
}
unsafe impl ::windows::core::Interface for IAppxContentGroupFilesEnumerator {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x1a09a2fd_7440_44eb_8c84_848205a6a1cc);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppxContentGroupFilesEnumerator_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub GetCurrent: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, file: *mut ::windows::core::PWSTR) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub GetHasCurrent: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, hascurrent: *mut super::super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetHasCurrent: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub MoveNext: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, hasnext: *mut super::super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    MoveNext: usize,
}
#[doc = "*Required features: `\"Win32_Storage_Packaging_Appx\"`*"]
#[repr(transparent)]
pub struct IAppxContentGroupMapReader(::windows::core::IUnknown);
impl IAppxContentGroupMapReader {
    pub unsafe fn GetRequiredGroup(&self) -> ::windows::core::Result<IAppxContentGroup> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).GetRequiredGroup)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetAutomaticGroups(&self) -> ::windows::core::Result<IAppxContentGroupsEnumerator> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).GetAutomaticGroups)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
}
::windows::core::interface_hierarchy!(IAppxContentGroupMapReader, ::windows::core::IUnknown);
impl ::core::clone::Clone for IAppxContentGroupMapReader {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IAppxContentGroupMapReader {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IAppxContentGroupMapReader {}
impl ::core::fmt::Debug for IAppxContentGroupMapReader {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IAppxContentGroupMapReader").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Vtable for IAppxContentGroupMapReader {
    type Vtable = IAppxContentGroupMapReader_Vtbl;
}
unsafe impl ::windows::core::Interface for IAppxContentGroupMapReader {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x418726d8_dd99_4f5d_9886_157add20de01);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppxContentGroupMapReader_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub GetRequiredGroup: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, requiredgroup: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub GetAutomaticGroups: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, automaticgroupsenumerator: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Storage_Packaging_Appx\"`*"]
#[repr(transparent)]
pub struct IAppxContentGroupMapWriter(::windows::core::IUnknown);
impl IAppxContentGroupMapWriter {
    pub unsafe fn AddAutomaticGroup<P0>(&self, groupname: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
    {
        (::windows::core::Vtable::vtable(self).AddAutomaticGroup)(::windows::core::Vtable::as_raw(self), groupname.into().abi()).ok()
    }
    pub unsafe fn AddAutomaticFile<P0>(&self, filename: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
    {
        (::windows::core::Vtable::vtable(self).AddAutomaticFile)(::windows::core::Vtable::as_raw(self), filename.into().abi()).ok()
    }
    pub unsafe fn Close(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).Close)(::windows::core::Vtable::as_raw(self)).ok()
    }
}
::windows::core::interface_hierarchy!(IAppxContentGroupMapWriter, ::windows::core::IUnknown);
impl ::core::clone::Clone for IAppxContentGroupMapWriter {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IAppxContentGroupMapWriter {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IAppxContentGroupMapWriter {}
impl ::core::fmt::Debug for IAppxContentGroupMapWriter {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IAppxContentGroupMapWriter").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Vtable for IAppxContentGroupMapWriter {
    type Vtable = IAppxContentGroupMapWriter_Vtbl;
}
unsafe impl ::windows::core::Interface for IAppxContentGroupMapWriter {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xd07ab776_a9de_4798_8c14_3db31e687c78);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppxContentGroupMapWriter_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub AddAutomaticGroup: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, groupname: ::windows::core::PCWSTR) -> ::windows::core::HRESULT,
    pub AddAutomaticFile: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, filename: ::windows::core::PCWSTR) -> ::windows::core::HRESULT,
    pub Close: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Storage_Packaging_Appx\"`*"]
#[repr(transparent)]
pub struct IAppxContentGroupsEnumerator(::windows::core::IUnknown);
impl IAppxContentGroupsEnumerator {
    pub unsafe fn GetCurrent(&self) -> ::windows::core::Result<IAppxContentGroup> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).GetCurrent)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetHasCurrent(&self) -> ::windows::core::Result<super::super::super::Foundation::BOOL> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).GetHasCurrent)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn MoveNext(&self) -> ::windows::core::Result<super::super::super::Foundation::BOOL> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).MoveNext)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
}
::windows::core::interface_hierarchy!(IAppxContentGroupsEnumerator, ::windows::core::IUnknown);
impl ::core::clone::Clone for IAppxContentGroupsEnumerator {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IAppxContentGroupsEnumerator {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IAppxContentGroupsEnumerator {}
impl ::core::fmt::Debug for IAppxContentGroupsEnumerator {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IAppxContentGroupsEnumerator").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Vtable for IAppxContentGroupsEnumerator {
    type Vtable = IAppxContentGroupsEnumerator_Vtbl;
}
unsafe impl ::windows::core::Interface for IAppxContentGroupsEnumerator {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x3264e477_16d1_4d63_823e_7d2984696634);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppxContentGroupsEnumerator_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub GetCurrent: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, stream: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub GetHasCurrent: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, hascurrent: *mut super::super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetHasCurrent: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub MoveNext: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, hasnext: *mut super::super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    MoveNext: usize,
}
#[doc = "*Required features: `\"Win32_Storage_Packaging_Appx\"`*"]
#[repr(transparent)]
pub struct IAppxEncryptedBundleWriter(::windows::core::IUnknown);
impl IAppxEncryptedBundleWriter {
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn AddPayloadPackageEncrypted<P0, P1>(&self, filename: P0, packagestream: P1) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
        P1: ::std::convert::Into<::windows::core::InParam<super::super::super::System::Com::IStream>>,
    {
        (::windows::core::Vtable::vtable(self).AddPayloadPackageEncrypted)(::windows::core::Vtable::as_raw(self), filename.into().abi(), packagestream.into().abi()).ok()
    }
    pub unsafe fn Close(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).Close)(::windows::core::Vtable::as_raw(self)).ok()
    }
}
::windows::core::interface_hierarchy!(IAppxEncryptedBundleWriter, ::windows::core::IUnknown);
impl ::core::clone::Clone for IAppxEncryptedBundleWriter {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IAppxEncryptedBundleWriter {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IAppxEncryptedBundleWriter {}
impl ::core::fmt::Debug for IAppxEncryptedBundleWriter {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IAppxEncryptedBundleWriter").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Vtable for IAppxEncryptedBundleWriter {
    type Vtable = IAppxEncryptedBundleWriter_Vtbl;
}
unsafe impl ::windows::core::Interface for IAppxEncryptedBundleWriter {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x80b0902f_7bf0_4117_b8c6_4279ef81ee77);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppxEncryptedBundleWriter_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    #[cfg(feature = "Win32_System_Com")]
    pub AddPayloadPackageEncrypted: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, filename: ::windows::core::PCWSTR, packagestream: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    AddPayloadPackageEncrypted: usize,
    pub Close: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Storage_Packaging_Appx\"`*"]
#[repr(transparent)]
pub struct IAppxEncryptedBundleWriter2(::windows::core::IUnknown);
impl IAppxEncryptedBundleWriter2 {
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn AddExternalPackageReference<P0, P1>(&self, filename: P0, inputstream: P1) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
        P1: ::std::convert::Into<::windows::core::InParam<super::super::super::System::Com::IStream>>,
    {
        (::windows::core::Vtable::vtable(self).AddExternalPackageReference)(::windows::core::Vtable::as_raw(self), filename.into().abi(), inputstream.into().abi()).ok()
    }
}
::windows::core::interface_hierarchy!(IAppxEncryptedBundleWriter2, ::windows::core::IUnknown);
impl ::core::clone::Clone for IAppxEncryptedBundleWriter2 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IAppxEncryptedBundleWriter2 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IAppxEncryptedBundleWriter2 {}
impl ::core::fmt::Debug for IAppxEncryptedBundleWriter2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IAppxEncryptedBundleWriter2").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Vtable for IAppxEncryptedBundleWriter2 {
    type Vtable = IAppxEncryptedBundleWriter2_Vtbl;
}
unsafe impl ::windows::core::Interface for IAppxEncryptedBundleWriter2 {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xe644be82_f0fa_42b8_a956_8d1cb48ee379);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppxEncryptedBundleWriter2_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    #[cfg(feature = "Win32_System_Com")]
    pub AddExternalPackageReference: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, filename: ::windows::core::PCWSTR, inputstream: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    AddExternalPackageReference: usize,
}
#[doc = "*Required features: `\"Win32_Storage_Packaging_Appx\"`*"]
#[repr(transparent)]
pub struct IAppxEncryptedBundleWriter3(::windows::core::IUnknown);
impl IAppxEncryptedBundleWriter3 {
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub unsafe fn AddPayloadPackageEncrypted<P0, P1, P2>(&self, filename: P0, packagestream: P1, isdefaultapplicablepackage: P2) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
        P1: ::std::convert::Into<::windows::core::InParam<super::super::super::System::Com::IStream>>,
        P2: ::std::convert::Into<super::super::super::Foundation::BOOL>,
    {
        (::windows::core::Vtable::vtable(self).AddPayloadPackageEncrypted)(::windows::core::Vtable::as_raw(self), filename.into().abi(), packagestream.into().abi(), isdefaultapplicablepackage.into()).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub unsafe fn AddExternalPackageReference<P0, P1, P2>(&self, filename: P0, inputstream: P1, isdefaultapplicablepackage: P2) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
        P1: ::std::convert::Into<::windows::core::InParam<super::super::super::System::Com::IStream>>,
        P2: ::std::convert::Into<super::super::super::Foundation::BOOL>,
    {
        (::windows::core::Vtable::vtable(self).AddExternalPackageReference)(::windows::core::Vtable::as_raw(self), filename.into().abi(), inputstream.into().abi(), isdefaultapplicablepackage.into()).ok()
    }
}
::windows::core::interface_hierarchy!(IAppxEncryptedBundleWriter3, ::windows::core::IUnknown);
impl ::core::clone::Clone for IAppxEncryptedBundleWriter3 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IAppxEncryptedBundleWriter3 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IAppxEncryptedBundleWriter3 {}
impl ::core::fmt::Debug for IAppxEncryptedBundleWriter3 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IAppxEncryptedBundleWriter3").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Vtable for IAppxEncryptedBundleWriter3 {
    type Vtable = IAppxEncryptedBundleWriter3_Vtbl;
}
unsafe impl ::windows::core::Interface for IAppxEncryptedBundleWriter3 {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x0d34deb3_5cae_4dd3_977c_504932a51d31);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppxEncryptedBundleWriter3_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub AddPayloadPackageEncrypted: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, filename: ::windows::core::PCWSTR, packagestream: *mut ::core::ffi::c_void, isdefaultapplicablepackage: super::super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com")))]
    AddPayloadPackageEncrypted: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub AddExternalPackageReference: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, filename: ::windows::core::PCWSTR, inputstream: *mut ::core::ffi::c_void, isdefaultapplicablepackage: super::super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com")))]
    AddExternalPackageReference: usize,
}
#[doc = "*Required features: `\"Win32_Storage_Packaging_Appx\"`*"]
#[repr(transparent)]
pub struct IAppxEncryptedPackageWriter(::windows::core::IUnknown);
impl IAppxEncryptedPackageWriter {
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn AddPayloadFileEncrypted<P0, P1>(&self, filename: P0, compressionoption: APPX_COMPRESSION_OPTION, inputstream: P1) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
        P1: ::std::convert::Into<::windows::core::InParam<super::super::super::System::Com::IStream>>,
    {
        (::windows::core::Vtable::vtable(self).AddPayloadFileEncrypted)(::windows::core::Vtable::as_raw(self), filename.into().abi(), compressionoption, inputstream.into().abi()).ok()
    }
    pub unsafe fn Close(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).Close)(::windows::core::Vtable::as_raw(self)).ok()
    }
}
::windows::core::interface_hierarchy!(IAppxEncryptedPackageWriter, ::windows::core::IUnknown);
impl ::core::clone::Clone for IAppxEncryptedPackageWriter {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IAppxEncryptedPackageWriter {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IAppxEncryptedPackageWriter {}
impl ::core::fmt::Debug for IAppxEncryptedPackageWriter {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IAppxEncryptedPackageWriter").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Vtable for IAppxEncryptedPackageWriter {
    type Vtable = IAppxEncryptedPackageWriter_Vtbl;
}
unsafe impl ::windows::core::Interface for IAppxEncryptedPackageWriter {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xf43d0b0b_1379_40e2_9b29_682ea2bf42af);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppxEncryptedPackageWriter_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    #[cfg(feature = "Win32_System_Com")]
    pub AddPayloadFileEncrypted: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, filename: ::windows::core::PCWSTR, compressionoption: APPX_COMPRESSION_OPTION, inputstream: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    AddPayloadFileEncrypted: usize,
    pub Close: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Storage_Packaging_Appx\"`*"]
#[repr(transparent)]
pub struct IAppxEncryptedPackageWriter2(::windows::core::IUnknown);
impl IAppxEncryptedPackageWriter2 {
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn AddPayloadFilesEncrypted(&self, payloadfiles: &[APPX_PACKAGE_WRITER_PAYLOAD_STREAM], memorylimit: u64) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).AddPayloadFilesEncrypted)(::windows::core::Vtable::as_raw(self), payloadfiles.len() as _, ::core::mem::transmute(payloadfiles.as_ptr()), memorylimit).ok()
    }
}
::windows::core::interface_hierarchy!(IAppxEncryptedPackageWriter2, ::windows::core::IUnknown);
impl ::core::clone::Clone for IAppxEncryptedPackageWriter2 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IAppxEncryptedPackageWriter2 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IAppxEncryptedPackageWriter2 {}
impl ::core::fmt::Debug for IAppxEncryptedPackageWriter2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IAppxEncryptedPackageWriter2").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Vtable for IAppxEncryptedPackageWriter2 {
    type Vtable = IAppxEncryptedPackageWriter2_Vtbl;
}
unsafe impl ::windows::core::Interface for IAppxEncryptedPackageWriter2 {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x3e475447_3a25_40b5_8ad2_f953ae50c92d);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppxEncryptedPackageWriter2_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    #[cfg(feature = "Win32_System_Com")]
    pub AddPayloadFilesEncrypted: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, filecount: u32, payloadfiles: *const APPX_PACKAGE_WRITER_PAYLOAD_STREAM, memorylimit: u64) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    AddPayloadFilesEncrypted: usize,
}
#[doc = "*Required features: `\"Win32_Storage_Packaging_Appx\"`*"]
#[repr(transparent)]
pub struct IAppxEncryptionFactory(::windows::core::IUnknown);
impl IAppxEncryptionFactory {
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub unsafe fn EncryptPackage<P0, P1>(&self, inputstream: P0, outputstream: P1, settings: *const APPX_ENCRYPTED_PACKAGE_SETTINGS, keyinfo: *const APPX_KEY_INFO, exemptedfiles: *const APPX_ENCRYPTED_EXEMPTIONS) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<super::super::super::System::Com::IStream>>,
        P1: ::std::convert::Into<::windows::core::InParam<super::super::super::System::Com::IStream>>,
    {
        (::windows::core::Vtable::vtable(self).EncryptPackage)(::windows::core::Vtable::as_raw(self), inputstream.into().abi(), outputstream.into().abi(), settings, keyinfo, exemptedfiles).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn DecryptPackage<P0, P1>(&self, inputstream: P0, outputstream: P1, keyinfo: *const APPX_KEY_INFO) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<super::super::super::System::Com::IStream>>,
        P1: ::std::convert::Into<::windows::core::InParam<super::super::super::System::Com::IStream>>,
    {
        (::windows::core::Vtable::vtable(self).DecryptPackage)(::windows::core::Vtable::as_raw(self), inputstream.into().abi(), outputstream.into().abi(), keyinfo).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub unsafe fn CreateEncryptedPackageWriter<P0, P1>(&self, outputstream: P0, manifeststream: P1, settings: *const APPX_ENCRYPTED_PACKAGE_SETTINGS, keyinfo: *const APPX_KEY_INFO, exemptedfiles: *const APPX_ENCRYPTED_EXEMPTIONS) -> ::windows::core::Result<IAppxEncryptedPackageWriter>
    where
        P0: ::std::convert::Into<::windows::core::InParam<super::super::super::System::Com::IStream>>,
        P1: ::std::convert::Into<::windows::core::InParam<super::super::super::System::Com::IStream>>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).CreateEncryptedPackageWriter)(::windows::core::Vtable::as_raw(self), outputstream.into().abi(), manifeststream.into().abi(), settings, keyinfo, exemptedfiles, result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn CreateEncryptedPackageReader<P0>(&self, inputstream: P0, keyinfo: *const APPX_KEY_INFO) -> ::windows::core::Result<IAppxPackageReader>
    where
        P0: ::std::convert::Into<::windows::core::InParam<super::super::super::System::Com::IStream>>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).CreateEncryptedPackageReader)(::windows::core::Vtable::as_raw(self), inputstream.into().abi(), keyinfo, result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub unsafe fn EncryptBundle<P0, P1>(&self, inputstream: P0, outputstream: P1, settings: *const APPX_ENCRYPTED_PACKAGE_SETTINGS, keyinfo: *const APPX_KEY_INFO, exemptedfiles: *const APPX_ENCRYPTED_EXEMPTIONS) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<super::super::super::System::Com::IStream>>,
        P1: ::std::convert::Into<::windows::core::InParam<super::super::super::System::Com::IStream>>,
    {
        (::windows::core::Vtable::vtable(self).EncryptBundle)(::windows::core::Vtable::as_raw(self), inputstream.into().abi(), outputstream.into().abi(), settings, keyinfo, exemptedfiles).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn DecryptBundle<P0, P1>(&self, inputstream: P0, outputstream: P1, keyinfo: *const APPX_KEY_INFO) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<super::super::super::System::Com::IStream>>,
        P1: ::std::convert::Into<::windows::core::InParam<super::super::super::System::Com::IStream>>,
    {
        (::windows::core::Vtable::vtable(self).DecryptBundle)(::windows::core::Vtable::as_raw(self), inputstream.into().abi(), outputstream.into().abi(), keyinfo).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub unsafe fn CreateEncryptedBundleWriter<P0>(&self, outputstream: P0, bundleversion: u64, settings: *const APPX_ENCRYPTED_PACKAGE_SETTINGS, keyinfo: *const APPX_KEY_INFO, exemptedfiles: *const APPX_ENCRYPTED_EXEMPTIONS) -> ::windows::core::Result<IAppxEncryptedBundleWriter>
    where
        P0: ::std::convert::Into<::windows::core::InParam<super::super::super::System::Com::IStream>>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).CreateEncryptedBundleWriter)(::windows::core::Vtable::as_raw(self), outputstream.into().abi(), bundleversion, settings, keyinfo, exemptedfiles, result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn CreateEncryptedBundleReader<P0>(&self, inputstream: P0, keyinfo: *const APPX_KEY_INFO) -> ::windows::core::Result<IAppxBundleReader>
    where
        P0: ::std::convert::Into<::windows::core::InParam<super::super::super::System::Com::IStream>>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).CreateEncryptedBundleReader)(::windows::core::Vtable::as_raw(self), inputstream.into().abi(), keyinfo, result__.as_mut_ptr()).from_abi(result__)
    }
}
::windows::core::interface_hierarchy!(IAppxEncryptionFactory, ::windows::core::IUnknown);
impl ::core::clone::Clone for IAppxEncryptionFactory {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IAppxEncryptionFactory {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IAppxEncryptionFactory {}
impl ::core::fmt::Debug for IAppxEncryptionFactory {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IAppxEncryptionFactory").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Vtable for IAppxEncryptionFactory {
    type Vtable = IAppxEncryptionFactory_Vtbl;
}
unsafe impl ::windows::core::Interface for IAppxEncryptionFactory {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x80e8e04d_8c88_44ae_a011_7cadf6fb2e72);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppxEncryptionFactory_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub EncryptPackage: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, inputstream: *mut ::core::ffi::c_void, outputstream: *mut ::core::ffi::c_void, settings: *const APPX_ENCRYPTED_PACKAGE_SETTINGS, keyinfo: *const APPX_KEY_INFO, exemptedfiles: *const APPX_ENCRYPTED_EXEMPTIONS) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com")))]
    EncryptPackage: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub DecryptPackage: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, inputstream: *mut ::core::ffi::c_void, outputstream: *mut ::core::ffi::c_void, keyinfo: *const APPX_KEY_INFO) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    DecryptPackage: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub CreateEncryptedPackageWriter: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, outputstream: *mut ::core::ffi::c_void, manifeststream: *mut ::core::ffi::c_void, settings: *const APPX_ENCRYPTED_PACKAGE_SETTINGS, keyinfo: *const APPX_KEY_INFO, exemptedfiles: *const APPX_ENCRYPTED_EXEMPTIONS, packagewriter: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com")))]
    CreateEncryptedPackageWriter: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub CreateEncryptedPackageReader: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, inputstream: *mut ::core::ffi::c_void, keyinfo: *const APPX_KEY_INFO, packagereader: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    CreateEncryptedPackageReader: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub EncryptBundle: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, inputstream: *mut ::core::ffi::c_void, outputstream: *mut ::core::ffi::c_void, settings: *const APPX_ENCRYPTED_PACKAGE_SETTINGS, keyinfo: *const APPX_KEY_INFO, exemptedfiles: *const APPX_ENCRYPTED_EXEMPTIONS) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com")))]
    EncryptBundle: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub DecryptBundle: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, inputstream: *mut ::core::ffi::c_void, outputstream: *mut ::core::ffi::c_void, keyinfo: *const APPX_KEY_INFO) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    DecryptBundle: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub CreateEncryptedBundleWriter: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, outputstream: *mut ::core::ffi::c_void, bundleversion: u64, settings: *const APPX_ENCRYPTED_PACKAGE_SETTINGS, keyinfo: *const APPX_KEY_INFO, exemptedfiles: *const APPX_ENCRYPTED_EXEMPTIONS, bundlewriter: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com")))]
    CreateEncryptedBundleWriter: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub CreateEncryptedBundleReader: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, inputstream: *mut ::core::ffi::c_void, keyinfo: *const APPX_KEY_INFO, bundlereader: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    CreateEncryptedBundleReader: usize,
}
#[doc = "*Required features: `\"Win32_Storage_Packaging_Appx\"`*"]
#[repr(transparent)]
pub struct IAppxEncryptionFactory2(::windows::core::IUnknown);
impl IAppxEncryptionFactory2 {
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub unsafe fn CreateEncryptedPackageWriter<P0, P1, P2>(&self, outputstream: P0, manifeststream: P1, contentgroupmapstream: P2, settings: *const APPX_ENCRYPTED_PACKAGE_SETTINGS, keyinfo: *const APPX_KEY_INFO, exemptedfiles: *const APPX_ENCRYPTED_EXEMPTIONS) -> ::windows::core::Result<IAppxEncryptedPackageWriter>
    where
        P0: ::std::convert::Into<::windows::core::InParam<super::super::super::System::Com::IStream>>,
        P1: ::std::convert::Into<::windows::core::InParam<super::super::super::System::Com::IStream>>,
        P2: ::std::convert::Into<::windows::core::InParam<super::super::super::System::Com::IStream>>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).CreateEncryptedPackageWriter)(::windows::core::Vtable::as_raw(self), outputstream.into().abi(), manifeststream.into().abi(), contentgroupmapstream.into().abi(), settings, keyinfo, exemptedfiles, result__.as_mut_ptr()).from_abi(result__)
    }
}
::windows::core::interface_hierarchy!(IAppxEncryptionFactory2, ::windows::core::IUnknown);
impl ::core::clone::Clone for IAppxEncryptionFactory2 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IAppxEncryptionFactory2 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IAppxEncryptionFactory2 {}
impl ::core::fmt::Debug for IAppxEncryptionFactory2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IAppxEncryptionFactory2").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Vtable for IAppxEncryptionFactory2 {
    type Vtable = IAppxEncryptionFactory2_Vtbl;
}
unsafe impl ::windows::core::Interface for IAppxEncryptionFactory2 {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xc1b11eee_c4ba_4ab2_a55d_d015fe8ff64f);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppxEncryptionFactory2_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub CreateEncryptedPackageWriter: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, outputstream: *mut ::core::ffi::c_void, manifeststream: *mut ::core::ffi::c_void, contentgroupmapstream: *mut ::core::ffi::c_void, settings: *const APPX_ENCRYPTED_PACKAGE_SETTINGS, keyinfo: *const APPX_KEY_INFO, exemptedfiles: *const APPX_ENCRYPTED_EXEMPTIONS, packagewriter: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com")))]
    CreateEncryptedPackageWriter: usize,
}
#[doc = "*Required features: `\"Win32_Storage_Packaging_Appx\"`*"]
#[repr(transparent)]
pub struct IAppxEncryptionFactory3(::windows::core::IUnknown);
impl IAppxEncryptionFactory3 {
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn EncryptPackage<P0, P1>(&self, inputstream: P0, outputstream: P1, settings: *const APPX_ENCRYPTED_PACKAGE_SETTINGS2, keyinfo: *const APPX_KEY_INFO, exemptedfiles: *const APPX_ENCRYPTED_EXEMPTIONS) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<super::super::super::System::Com::IStream>>,
        P1: ::std::convert::Into<::windows::core::InParam<super::super::super::System::Com::IStream>>,
    {
        (::windows::core::Vtable::vtable(self).EncryptPackage)(::windows::core::Vtable::as_raw(self), inputstream.into().abi(), outputstream.into().abi(), settings, keyinfo, exemptedfiles).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn CreateEncryptedPackageWriter<P0, P1, P2>(&self, outputstream: P0, manifeststream: P1, contentgroupmapstream: P2, settings: *const APPX_ENCRYPTED_PACKAGE_SETTINGS2, keyinfo: *const APPX_KEY_INFO, exemptedfiles: *const APPX_ENCRYPTED_EXEMPTIONS) -> ::windows::core::Result<IAppxEncryptedPackageWriter>
    where
        P0: ::std::convert::Into<::windows::core::InParam<super::super::super::System::Com::IStream>>,
        P1: ::std::convert::Into<::windows::core::InParam<super::super::super::System::Com::IStream>>,
        P2: ::std::convert::Into<::windows::core::InParam<super::super::super::System::Com::IStream>>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).CreateEncryptedPackageWriter)(::windows::core::Vtable::as_raw(self), outputstream.into().abi(), manifeststream.into().abi(), contentgroupmapstream.into().abi(), settings, keyinfo, exemptedfiles, result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn EncryptBundle<P0, P1>(&self, inputstream: P0, outputstream: P1, settings: *const APPX_ENCRYPTED_PACKAGE_SETTINGS2, keyinfo: *const APPX_KEY_INFO, exemptedfiles: *const APPX_ENCRYPTED_EXEMPTIONS) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<super::super::super::System::Com::IStream>>,
        P1: ::std::convert::Into<::windows::core::InParam<super::super::super::System::Com::IStream>>,
    {
        (::windows::core::Vtable::vtable(self).EncryptBundle)(::windows::core::Vtable::as_raw(self), inputstream.into().abi(), outputstream.into().abi(), settings, keyinfo, exemptedfiles).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn CreateEncryptedBundleWriter<P0>(&self, outputstream: P0, bundleversion: u64, settings: *const APPX_ENCRYPTED_PACKAGE_SETTINGS2, keyinfo: *const APPX_KEY_INFO, exemptedfiles: *const APPX_ENCRYPTED_EXEMPTIONS) -> ::windows::core::Result<IAppxEncryptedBundleWriter>
    where
        P0: ::std::convert::Into<::windows::core::InParam<super::super::super::System::Com::IStream>>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).CreateEncryptedBundleWriter)(::windows::core::Vtable::as_raw(self), outputstream.into().abi(), bundleversion, settings, keyinfo, exemptedfiles, result__.as_mut_ptr()).from_abi(result__)
    }
}
::windows::core::interface_hierarchy!(IAppxEncryptionFactory3, ::windows::core::IUnknown);
impl ::core::clone::Clone for IAppxEncryptionFactory3 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IAppxEncryptionFactory3 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IAppxEncryptionFactory3 {}
impl ::core::fmt::Debug for IAppxEncryptionFactory3 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IAppxEncryptionFactory3").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Vtable for IAppxEncryptionFactory3 {
    type Vtable = IAppxEncryptionFactory3_Vtbl;
}
unsafe impl ::windows::core::Interface for IAppxEncryptionFactory3 {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x09edca37_cd64_47d6_b7e8_1cb11d4f7e05);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppxEncryptionFactory3_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    #[cfg(feature = "Win32_System_Com")]
    pub EncryptPackage: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, inputstream: *mut ::core::ffi::c_void, outputstream: *mut ::core::ffi::c_void, settings: *const APPX_ENCRYPTED_PACKAGE_SETTINGS2, keyinfo: *const APPX_KEY_INFO, exemptedfiles: *const APPX_ENCRYPTED_EXEMPTIONS) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    EncryptPackage: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub CreateEncryptedPackageWriter: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, outputstream: *mut ::core::ffi::c_void, manifeststream: *mut ::core::ffi::c_void, contentgroupmapstream: *mut ::core::ffi::c_void, settings: *const APPX_ENCRYPTED_PACKAGE_SETTINGS2, keyinfo: *const APPX_KEY_INFO, exemptedfiles: *const APPX_ENCRYPTED_EXEMPTIONS, packagewriter: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    CreateEncryptedPackageWriter: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub EncryptBundle: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, inputstream: *mut ::core::ffi::c_void, outputstream: *mut ::core::ffi::c_void, settings: *const APPX_ENCRYPTED_PACKAGE_SETTINGS2, keyinfo: *const APPX_KEY_INFO, exemptedfiles: *const APPX_ENCRYPTED_EXEMPTIONS) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    EncryptBundle: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub CreateEncryptedBundleWriter: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, outputstream: *mut ::core::ffi::c_void, bundleversion: u64, settings: *const APPX_ENCRYPTED_PACKAGE_SETTINGS2, keyinfo: *const APPX_KEY_INFO, exemptedfiles: *const APPX_ENCRYPTED_EXEMPTIONS, bundlewriter: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    CreateEncryptedBundleWriter: usize,
}
#[doc = "*Required features: `\"Win32_Storage_Packaging_Appx\"`*"]
#[repr(transparent)]
pub struct IAppxEncryptionFactory4(::windows::core::IUnknown);
impl IAppxEncryptionFactory4 {
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn EncryptPackage<P0, P1>(&self, inputstream: P0, outputstream: P1, settings: *const APPX_ENCRYPTED_PACKAGE_SETTINGS2, keyinfo: *const APPX_KEY_INFO, exemptedfiles: *const APPX_ENCRYPTED_EXEMPTIONS, memorylimit: u64) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<super::super::super::System::Com::IStream>>,
        P1: ::std::convert::Into<::windows::core::InParam<super::super::super::System::Com::IStream>>,
    {
        (::windows::core::Vtable::vtable(self).EncryptPackage)(::windows::core::Vtable::as_raw(self), inputstream.into().abi(), outputstream.into().abi(), settings, keyinfo, exemptedfiles, memorylimit).ok()
    }
}
::windows::core::interface_hierarchy!(IAppxEncryptionFactory4, ::windows::core::IUnknown);
impl ::core::clone::Clone for IAppxEncryptionFactory4 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IAppxEncryptionFactory4 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IAppxEncryptionFactory4 {}
impl ::core::fmt::Debug for IAppxEncryptionFactory4 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IAppxEncryptionFactory4").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Vtable for IAppxEncryptionFactory4 {
    type Vtable = IAppxEncryptionFactory4_Vtbl;
}
unsafe impl ::windows::core::Interface for IAppxEncryptionFactory4 {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xa879611f_12fd_41fe_85d5_06ae779bbaf5);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppxEncryptionFactory4_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    #[cfg(feature = "Win32_System_Com")]
    pub EncryptPackage: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, inputstream: *mut ::core::ffi::c_void, outputstream: *mut ::core::ffi::c_void, settings: *const APPX_ENCRYPTED_PACKAGE_SETTINGS2, keyinfo: *const APPX_KEY_INFO, exemptedfiles: *const APPX_ENCRYPTED_EXEMPTIONS, memorylimit: u64) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    EncryptPackage: usize,
}
#[doc = "*Required features: `\"Win32_Storage_Packaging_Appx\"`*"]
#[repr(transparent)]
pub struct IAppxFactory(::windows::core::IUnknown);
impl IAppxFactory {
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub unsafe fn CreatePackageWriter<P0>(&self, outputstream: P0, settings: *const APPX_PACKAGE_SETTINGS) -> ::windows::core::Result<IAppxPackageWriter>
    where
        P0: ::std::convert::Into<::windows::core::InParam<super::super::super::System::Com::IStream>>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).CreatePackageWriter)(::windows::core::Vtable::as_raw(self), outputstream.into().abi(), settings, result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn CreatePackageReader<P0>(&self, inputstream: P0) -> ::windows::core::Result<IAppxPackageReader>
    where
        P0: ::std::convert::Into<::windows::core::InParam<super::super::super::System::Com::IStream>>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).CreatePackageReader)(::windows::core::Vtable::as_raw(self), inputstream.into().abi(), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn CreateManifestReader<P0>(&self, inputstream: P0) -> ::windows::core::Result<IAppxManifestReader>
    where
        P0: ::std::convert::Into<::windows::core::InParam<super::super::super::System::Com::IStream>>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).CreateManifestReader)(::windows::core::Vtable::as_raw(self), inputstream.into().abi(), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn CreateBlockMapReader<P0>(&self, inputstream: P0) -> ::windows::core::Result<IAppxBlockMapReader>
    where
        P0: ::std::convert::Into<::windows::core::InParam<super::super::super::System::Com::IStream>>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).CreateBlockMapReader)(::windows::core::Vtable::as_raw(self), inputstream.into().abi(), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn CreateValidatedBlockMapReader<P0, P1>(&self, blockmapstream: P0, signaturefilename: P1) -> ::windows::core::Result<IAppxBlockMapReader>
    where
        P0: ::std::convert::Into<::windows::core::InParam<super::super::super::System::Com::IStream>>,
        P1: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).CreateValidatedBlockMapReader)(::windows::core::Vtable::as_raw(self), blockmapstream.into().abi(), signaturefilename.into().abi(), result__.as_mut_ptr()).from_abi(result__)
    }
}
::windows::core::interface_hierarchy!(IAppxFactory, ::windows::core::IUnknown);
impl ::core::clone::Clone for IAppxFactory {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IAppxFactory {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IAppxFactory {}
impl ::core::fmt::Debug for IAppxFactory {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IAppxFactory").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Vtable for IAppxFactory {
    type Vtable = IAppxFactory_Vtbl;
}
unsafe impl ::windows::core::Interface for IAppxFactory {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xbeb94909_e451_438b_b5a7_d79e767b75d8);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppxFactory_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub CreatePackageWriter: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, outputstream: *mut ::core::ffi::c_void, settings: *const APPX_PACKAGE_SETTINGS, packagewriter: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com")))]
    CreatePackageWriter: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub CreatePackageReader: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, inputstream: *mut ::core::ffi::c_void, packagereader: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    CreatePackageReader: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub CreateManifestReader: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, inputstream: *mut ::core::ffi::c_void, manifestreader: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    CreateManifestReader: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub CreateBlockMapReader: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, inputstream: *mut ::core::ffi::c_void, blockmapreader: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    CreateBlockMapReader: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub CreateValidatedBlockMapReader: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, blockmapstream: *mut ::core::ffi::c_void, signaturefilename: ::windows::core::PCWSTR, blockmapreader: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    CreateValidatedBlockMapReader: usize,
}
#[doc = "*Required features: `\"Win32_Storage_Packaging_Appx\"`*"]
#[repr(transparent)]
pub struct IAppxFactory2(::windows::core::IUnknown);
impl IAppxFactory2 {
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn CreateContentGroupMapReader<P0>(&self, inputstream: P0) -> ::windows::core::Result<IAppxContentGroupMapReader>
    where
        P0: ::std::convert::Into<::windows::core::InParam<super::super::super::System::Com::IStream>>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).CreateContentGroupMapReader)(::windows::core::Vtable::as_raw(self), inputstream.into().abi(), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn CreateSourceContentGroupMapReader<P0>(&self, inputstream: P0) -> ::windows::core::Result<IAppxSourceContentGroupMapReader>
    where
        P0: ::std::convert::Into<::windows::core::InParam<super::super::super::System::Com::IStream>>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).CreateSourceContentGroupMapReader)(::windows::core::Vtable::as_raw(self), inputstream.into().abi(), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn CreateContentGroupMapWriter<P0>(&self, stream: P0) -> ::windows::core::Result<IAppxContentGroupMapWriter>
    where
        P0: ::std::convert::Into<::windows::core::InParam<super::super::super::System::Com::IStream>>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).CreateContentGroupMapWriter)(::windows::core::Vtable::as_raw(self), stream.into().abi(), result__.as_mut_ptr()).from_abi(result__)
    }
}
::windows::core::interface_hierarchy!(IAppxFactory2, ::windows::core::IUnknown);
impl ::core::clone::Clone for IAppxFactory2 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IAppxFactory2 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IAppxFactory2 {}
impl ::core::fmt::Debug for IAppxFactory2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IAppxFactory2").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Vtable for IAppxFactory2 {
    type Vtable = IAppxFactory2_Vtbl;
}
unsafe impl ::windows::core::Interface for IAppxFactory2 {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xf1346df2_c282_4e22_b918_743a929a8d55);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppxFactory2_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    #[cfg(feature = "Win32_System_Com")]
    pub CreateContentGroupMapReader: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, inputstream: *mut ::core::ffi::c_void, contentgroupmapreader: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    CreateContentGroupMapReader: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub CreateSourceContentGroupMapReader: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, inputstream: *mut ::core::ffi::c_void, reader: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    CreateSourceContentGroupMapReader: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub CreateContentGroupMapWriter: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, stream: *mut ::core::ffi::c_void, contentgroupmapwriter: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    CreateContentGroupMapWriter: usize,
}
#[doc = "*Required features: `\"Win32_Storage_Packaging_Appx\"`*"]
#[repr(transparent)]
pub struct IAppxFile(::windows::core::IUnknown);
impl IAppxFile {
    pub unsafe fn GetCompressionOption(&self) -> ::windows::core::Result<APPX_COMPRESSION_OPTION> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).GetCompressionOption)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetContentType(&self) -> ::windows::core::Result<::windows::core::PWSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).GetContentType)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetName(&self) -> ::windows::core::Result<::windows::core::PWSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).GetName)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetSize(&self) -> ::windows::core::Result<u64> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).GetSize)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetStream(&self) -> ::windows::core::Result<super::super::super::System::Com::IStream> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).GetStream)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
}
::windows::core::interface_hierarchy!(IAppxFile, ::windows::core::IUnknown);
impl ::core::clone::Clone for IAppxFile {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IAppxFile {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IAppxFile {}
impl ::core::fmt::Debug for IAppxFile {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IAppxFile").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Vtable for IAppxFile {
    type Vtable = IAppxFile_Vtbl;
}
unsafe impl ::windows::core::Interface for IAppxFile {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x91df827b_94fd_468f_827b_57f41b2f6f2e);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppxFile_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub GetCompressionOption: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, compressionoption: *mut APPX_COMPRESSION_OPTION) -> ::windows::core::HRESULT,
    pub GetContentType: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, contenttype: *mut ::windows::core::PWSTR) -> ::windows::core::HRESULT,
    pub GetName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, filename: *mut ::windows::core::PWSTR) -> ::windows::core::HRESULT,
    pub GetSize: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, size: *mut u64) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub GetStream: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, stream: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    GetStream: usize,
}
#[doc = "*Required features: `\"Win32_Storage_Packaging_Appx\"`*"]
#[repr(transparent)]
pub struct IAppxFilesEnumerator(::windows::core::IUnknown);
impl IAppxFilesEnumerator {
    pub unsafe fn GetCurrent(&self) -> ::windows::core::Result<IAppxFile> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).GetCurrent)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetHasCurrent(&self) -> ::windows::core::Result<super::super::super::Foundation::BOOL> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).GetHasCurrent)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn MoveNext(&self) -> ::windows::core::Result<super::super::super::Foundation::BOOL> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).MoveNext)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
}
::windows::core::interface_hierarchy!(IAppxFilesEnumerator, ::windows::core::IUnknown);
impl ::core::clone::Clone for IAppxFilesEnumerator {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IAppxFilesEnumerator {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IAppxFilesEnumerator {}
impl ::core::fmt::Debug for IAppxFilesEnumerator {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IAppxFilesEnumerator").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Vtable for IAppxFilesEnumerator {
    type Vtable = IAppxFilesEnumerator_Vtbl;
}
unsafe impl ::windows::core::Interface for IAppxFilesEnumerator {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xf007eeaf_9831_411c_9847_917cdc62d1fe);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppxFilesEnumerator_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub GetCurrent: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, file: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub GetHasCurrent: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, hascurrent: *mut super::super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetHasCurrent: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub MoveNext: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, hasnext: *mut super::super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    MoveNext: usize,
}
#[doc = "*Required features: `\"Win32_Storage_Packaging_Appx\"`*"]
#[repr(transparent)]
pub struct IAppxManifestApplication(::windows::core::IUnknown);
impl IAppxManifestApplication {
    pub unsafe fn GetStringValue<P0>(&self, name: P0) -> ::windows::core::Result<::windows::core::PWSTR>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).GetStringValue)(::windows::core::Vtable::as_raw(self), name.into().abi(), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetAppUserModelId(&self) -> ::windows::core::Result<::windows::core::PWSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).GetAppUserModelId)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
}
::windows::core::interface_hierarchy!(IAppxManifestApplication, ::windows::core::IUnknown);
impl ::core::clone::Clone for IAppxManifestApplication {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IAppxManifestApplication {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IAppxManifestApplication {}
impl ::core::fmt::Debug for IAppxManifestApplication {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IAppxManifestApplication").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Vtable for IAppxManifestApplication {
    type Vtable = IAppxManifestApplication_Vtbl;
}
unsafe impl ::windows::core::Interface for IAppxManifestApplication {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x5da89bf4_3773_46be_b650_7e744863b7e8);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppxManifestApplication_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub GetStringValue: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, name: ::windows::core::PCWSTR, value: *mut ::windows::core::PWSTR) -> ::windows::core::HRESULT,
    pub GetAppUserModelId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, appusermodelid: *mut ::windows::core::PWSTR) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Storage_Packaging_Appx\"`*"]
#[repr(transparent)]
pub struct IAppxManifestApplicationsEnumerator(::windows::core::IUnknown);
impl IAppxManifestApplicationsEnumerator {
    pub unsafe fn GetCurrent(&self) -> ::windows::core::Result<IAppxManifestApplication> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).GetCurrent)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetHasCurrent(&self) -> ::windows::core::Result<super::super::super::Foundation::BOOL> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).GetHasCurrent)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn MoveNext(&self) -> ::windows::core::Result<super::super::super::Foundation::BOOL> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).MoveNext)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
}
::windows::core::interface_hierarchy!(IAppxManifestApplicationsEnumerator, ::windows::core::IUnknown);
impl ::core::clone::Clone for IAppxManifestApplicationsEnumerator {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IAppxManifestApplicationsEnumerator {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IAppxManifestApplicationsEnumerator {}
impl ::core::fmt::Debug for IAppxManifestApplicationsEnumerator {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IAppxManifestApplicationsEnumerator").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Vtable for IAppxManifestApplicationsEnumerator {
    type Vtable = IAppxManifestApplicationsEnumerator_Vtbl;
}
unsafe impl ::windows::core::Interface for IAppxManifestApplicationsEnumerator {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x9eb8a55a_f04b_4d0d_808d_686185d4847a);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppxManifestApplicationsEnumerator_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub GetCurrent: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, application: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub GetHasCurrent: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, hascurrent: *mut super::super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetHasCurrent: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub MoveNext: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, hasnext: *mut super::super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    MoveNext: usize,
}
#[doc = "*Required features: `\"Win32_Storage_Packaging_Appx\"`*"]
#[repr(transparent)]
pub struct IAppxManifestCapabilitiesEnumerator(::windows::core::IUnknown);
impl IAppxManifestCapabilitiesEnumerator {
    pub unsafe fn GetCurrent(&self) -> ::windows::core::Result<::windows::core::PWSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).GetCurrent)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetHasCurrent(&self) -> ::windows::core::Result<super::super::super::Foundation::BOOL> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).GetHasCurrent)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn MoveNext(&self) -> ::windows::core::Result<super::super::super::Foundation::BOOL> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).MoveNext)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
}
::windows::core::interface_hierarchy!(IAppxManifestCapabilitiesEnumerator, ::windows::core::IUnknown);
impl ::core::clone::Clone for IAppxManifestCapabilitiesEnumerator {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IAppxManifestCapabilitiesEnumerator {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IAppxManifestCapabilitiesEnumerator {}
impl ::core::fmt::Debug for IAppxManifestCapabilitiesEnumerator {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IAppxManifestCapabilitiesEnumerator").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Vtable for IAppxManifestCapabilitiesEnumerator {
    type Vtable = IAppxManifestCapabilitiesEnumerator_Vtbl;
}
unsafe impl ::windows::core::Interface for IAppxManifestCapabilitiesEnumerator {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x11d22258_f470_42c1_b291_8361c5437e41);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppxManifestCapabilitiesEnumerator_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub GetCurrent: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, capability: *mut ::windows::core::PWSTR) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub GetHasCurrent: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, hascurrent: *mut super::super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetHasCurrent: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub MoveNext: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, hasnext: *mut super::super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    MoveNext: usize,
}
#[doc = "*Required features: `\"Win32_Storage_Packaging_Appx\"`*"]
#[repr(transparent)]
pub struct IAppxManifestDeviceCapabilitiesEnumerator(::windows::core::IUnknown);
impl IAppxManifestDeviceCapabilitiesEnumerator {
    pub unsafe fn GetCurrent(&self) -> ::windows::core::Result<::windows::core::PWSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).GetCurrent)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetHasCurrent(&self) -> ::windows::core::Result<super::super::super::Foundation::BOOL> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).GetHasCurrent)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn MoveNext(&self) -> ::windows::core::Result<super::super::super::Foundation::BOOL> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).MoveNext)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
}
::windows::core::interface_hierarchy!(IAppxManifestDeviceCapabilitiesEnumerator, ::windows::core::IUnknown);
impl ::core::clone::Clone for IAppxManifestDeviceCapabilitiesEnumerator {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IAppxManifestDeviceCapabilitiesEnumerator {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IAppxManifestDeviceCapabilitiesEnumerator {}
impl ::core::fmt::Debug for IAppxManifestDeviceCapabilitiesEnumerator {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IAppxManifestDeviceCapabilitiesEnumerator").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Vtable for IAppxManifestDeviceCapabilitiesEnumerator {
    type Vtable = IAppxManifestDeviceCapabilitiesEnumerator_Vtbl;
}
unsafe impl ::windows::core::Interface for IAppxManifestDeviceCapabilitiesEnumerator {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x30204541_427b_4a1c_bacf_655bf463a540);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppxManifestDeviceCapabilitiesEnumerator_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub GetCurrent: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, devicecapability: *mut ::windows::core::PWSTR) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub GetHasCurrent: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, hascurrent: *mut super::super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetHasCurrent: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub MoveNext: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, hasnext: *mut super::super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    MoveNext: usize,
}
#[doc = "*Required features: `\"Win32_Storage_Packaging_Appx\"`*"]
#[repr(transparent)]
pub struct IAppxManifestDriverConstraint(::windows::core::IUnknown);
impl IAppxManifestDriverConstraint {
    pub unsafe fn GetName(&self) -> ::windows::core::Result<::windows::core::PWSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).GetName)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetMinVersion(&self) -> ::windows::core::Result<u64> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).GetMinVersion)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetMinDate(&self) -> ::windows::core::Result<::windows::core::PWSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).GetMinDate)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
}
::windows::core::interface_hierarchy!(IAppxManifestDriverConstraint, ::windows::core::IUnknown);
impl ::core::clone::Clone for IAppxManifestDriverConstraint {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IAppxManifestDriverConstraint {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IAppxManifestDriverConstraint {}
impl ::core::fmt::Debug for IAppxManifestDriverConstraint {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IAppxManifestDriverConstraint").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Vtable for IAppxManifestDriverConstraint {
    type Vtable = IAppxManifestDriverConstraint_Vtbl;
}
unsafe impl ::windows::core::Interface for IAppxManifestDriverConstraint {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xc031bee4_bbcc_48ea_a237_c34045c80a07);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppxManifestDriverConstraint_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub GetName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, name: *mut ::windows::core::PWSTR) -> ::windows::core::HRESULT,
    pub GetMinVersion: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, minversion: *mut u64) -> ::windows::core::HRESULT,
    pub GetMinDate: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, mindate: *mut ::windows::core::PWSTR) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Storage_Packaging_Appx\"`*"]
#[repr(transparent)]
pub struct IAppxManifestDriverConstraintsEnumerator(::windows::core::IUnknown);
impl IAppxManifestDriverConstraintsEnumerator {
    pub unsafe fn GetCurrent(&self) -> ::windows::core::Result<IAppxManifestDriverConstraint> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).GetCurrent)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetHasCurrent(&self) -> ::windows::core::Result<super::super::super::Foundation::BOOL> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).GetHasCurrent)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn MoveNext(&self) -> ::windows::core::Result<super::super::super::Foundation::BOOL> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).MoveNext)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
}
::windows::core::interface_hierarchy!(IAppxManifestDriverConstraintsEnumerator, ::windows::core::IUnknown);
impl ::core::clone::Clone for IAppxManifestDriverConstraintsEnumerator {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IAppxManifestDriverConstraintsEnumerator {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IAppxManifestDriverConstraintsEnumerator {}
impl ::core::fmt::Debug for IAppxManifestDriverConstraintsEnumerator {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IAppxManifestDriverConstraintsEnumerator").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Vtable for IAppxManifestDriverConstraintsEnumerator {
    type Vtable = IAppxManifestDriverConstraintsEnumerator_Vtbl;
}
unsafe impl ::windows::core::Interface for IAppxManifestDriverConstraintsEnumerator {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xd402b2d1_f600_49e0_95e6_975d8da13d89);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppxManifestDriverConstraintsEnumerator_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub GetCurrent: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, driverconstraint: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub GetHasCurrent: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, hascurrent: *mut super::super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetHasCurrent: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub MoveNext: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, hasnext: *mut super::super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    MoveNext: usize,
}
#[doc = "*Required features: `\"Win32_Storage_Packaging_Appx\"`*"]
#[repr(transparent)]
pub struct IAppxManifestDriverDependenciesEnumerator(::windows::core::IUnknown);
impl IAppxManifestDriverDependenciesEnumerator {
    pub unsafe fn GetCurrent(&self) -> ::windows::core::Result<IAppxManifestDriverDependency> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).GetCurrent)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetHasCurrent(&self) -> ::windows::core::Result<super::super::super::Foundation::BOOL> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).GetHasCurrent)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn MoveNext(&self) -> ::windows::core::Result<super::super::super::Foundation::BOOL> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).MoveNext)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
}
::windows::core::interface_hierarchy!(IAppxManifestDriverDependenciesEnumerator, ::windows::core::IUnknown);
impl ::core::clone::Clone for IAppxManifestDriverDependenciesEnumerator {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IAppxManifestDriverDependenciesEnumerator {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IAppxManifestDriverDependenciesEnumerator {}
impl ::core::fmt::Debug for IAppxManifestDriverDependenciesEnumerator {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IAppxManifestDriverDependenciesEnumerator").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Vtable for IAppxManifestDriverDependenciesEnumerator {
    type Vtable = IAppxManifestDriverDependenciesEnumerator_Vtbl;
}
unsafe impl ::windows::core::Interface for IAppxManifestDriverDependenciesEnumerator {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xfe039db2_467f_4755_8404_8f5eb6865b33);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppxManifestDriverDependenciesEnumerator_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub GetCurrent: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, driverdependency: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub GetHasCurrent: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, hascurrent: *mut super::super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetHasCurrent: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub MoveNext: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, hasnext: *mut super::super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    MoveNext: usize,
}
#[doc = "*Required features: `\"Win32_Storage_Packaging_Appx\"`*"]
#[repr(transparent)]
pub struct IAppxManifestDriverDependency(::windows::core::IUnknown);
impl IAppxManifestDriverDependency {
    pub unsafe fn GetDriverConstraints(&self) -> ::windows::core::Result<IAppxManifestDriverConstraintsEnumerator> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).GetDriverConstraints)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
}
::windows::core::interface_hierarchy!(IAppxManifestDriverDependency, ::windows::core::IUnknown);
impl ::core::clone::Clone for IAppxManifestDriverDependency {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IAppxManifestDriverDependency {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IAppxManifestDriverDependency {}
impl ::core::fmt::Debug for IAppxManifestDriverDependency {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IAppxManifestDriverDependency").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Vtable for IAppxManifestDriverDependency {
    type Vtable = IAppxManifestDriverDependency_Vtbl;
}
unsafe impl ::windows::core::Interface for IAppxManifestDriverDependency {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x1210cb94_5a92_4602_be24_79f318af4af9);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppxManifestDriverDependency_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub GetDriverConstraints: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, driverconstraints: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Storage_Packaging_Appx\"`*"]
#[repr(transparent)]
pub struct IAppxManifestHostRuntimeDependenciesEnumerator(::windows::core::IUnknown);
impl IAppxManifestHostRuntimeDependenciesEnumerator {
    pub unsafe fn GetCurrent(&self) -> ::windows::core::Result<IAppxManifestHostRuntimeDependency> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).GetCurrent)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetHasCurrent(&self) -> ::windows::core::Result<super::super::super::Foundation::BOOL> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).GetHasCurrent)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn MoveNext(&self) -> ::windows::core::Result<super::super::super::Foundation::BOOL> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).MoveNext)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
}
::windows::core::interface_hierarchy!(IAppxManifestHostRuntimeDependenciesEnumerator, ::windows::core::IUnknown);
impl ::core::clone::Clone for IAppxManifestHostRuntimeDependenciesEnumerator {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IAppxManifestHostRuntimeDependenciesEnumerator {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IAppxManifestHostRuntimeDependenciesEnumerator {}
impl ::core::fmt::Debug for IAppxManifestHostRuntimeDependenciesEnumerator {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IAppxManifestHostRuntimeDependenciesEnumerator").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Vtable for IAppxManifestHostRuntimeDependenciesEnumerator {
    type Vtable = IAppxManifestHostRuntimeDependenciesEnumerator_Vtbl;
}
unsafe impl ::windows::core::Interface for IAppxManifestHostRuntimeDependenciesEnumerator {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x6427a646_7f49_433e_b1a6_0da309f6885a);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppxManifestHostRuntimeDependenciesEnumerator_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub GetCurrent: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, hostruntimedependency: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub GetHasCurrent: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, hascurrent: *mut super::super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetHasCurrent: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub MoveNext: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, hasnext: *mut super::super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    MoveNext: usize,
}
#[doc = "*Required features: `\"Win32_Storage_Packaging_Appx\"`*"]
#[repr(transparent)]
pub struct IAppxManifestHostRuntimeDependency(::windows::core::IUnknown);
impl IAppxManifestHostRuntimeDependency {
    pub unsafe fn GetName(&self) -> ::windows::core::Result<::windows::core::PWSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).GetName)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetPublisher(&self) -> ::windows::core::Result<::windows::core::PWSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).GetPublisher)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetMinVersion(&self) -> ::windows::core::Result<u64> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).GetMinVersion)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
}
::windows::core::interface_hierarchy!(IAppxManifestHostRuntimeDependency, ::windows::core::IUnknown);
impl ::core::clone::Clone for IAppxManifestHostRuntimeDependency {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IAppxManifestHostRuntimeDependency {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IAppxManifestHostRuntimeDependency {}
impl ::core::fmt::Debug for IAppxManifestHostRuntimeDependency {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IAppxManifestHostRuntimeDependency").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Vtable for IAppxManifestHostRuntimeDependency {
    type Vtable = IAppxManifestHostRuntimeDependency_Vtbl;
}
unsafe impl ::windows::core::Interface for IAppxManifestHostRuntimeDependency {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x3455d234_8414_410d_95c7_7b35255b8391);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppxManifestHostRuntimeDependency_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub GetName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, name: *mut ::windows::core::PWSTR) -> ::windows::core::HRESULT,
    pub GetPublisher: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, publisher: *mut ::windows::core::PWSTR) -> ::windows::core::HRESULT,
    pub GetMinVersion: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, minversion: *mut u64) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Storage_Packaging_Appx\"`*"]
#[repr(transparent)]
pub struct IAppxManifestHostRuntimeDependency2(::windows::core::IUnknown);
impl IAppxManifestHostRuntimeDependency2 {
    pub unsafe fn GetPackageFamilyName(&self) -> ::windows::core::Result<::windows::core::PWSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).GetPackageFamilyName)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
}
::windows::core::interface_hierarchy!(IAppxManifestHostRuntimeDependency2, ::windows::core::IUnknown);
impl ::core::clone::Clone for IAppxManifestHostRuntimeDependency2 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IAppxManifestHostRuntimeDependency2 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IAppxManifestHostRuntimeDependency2 {}
impl ::core::fmt::Debug for IAppxManifestHostRuntimeDependency2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IAppxManifestHostRuntimeDependency2").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Vtable for IAppxManifestHostRuntimeDependency2 {
    type Vtable = IAppxManifestHostRuntimeDependency2_Vtbl;
}
unsafe impl ::windows::core::Interface for IAppxManifestHostRuntimeDependency2 {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xc26f23a8_ee10_4ad6_b898_2b4d7aebfe6a);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppxManifestHostRuntimeDependency2_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub GetPackageFamilyName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, packagefamilyname: *mut ::windows::core::PWSTR) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Storage_Packaging_Appx\"`*"]
#[repr(transparent)]
pub struct IAppxManifestMainPackageDependenciesEnumerator(::windows::core::IUnknown);
impl IAppxManifestMainPackageDependenciesEnumerator {
    pub unsafe fn GetCurrent(&self) -> ::windows::core::Result<IAppxManifestMainPackageDependency> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).GetCurrent)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetHasCurrent(&self) -> ::windows::core::Result<super::super::super::Foundation::BOOL> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).GetHasCurrent)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn MoveNext(&self) -> ::windows::core::Result<super::super::super::Foundation::BOOL> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).MoveNext)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
}
::windows::core::interface_hierarchy!(IAppxManifestMainPackageDependenciesEnumerator, ::windows::core::IUnknown);
impl ::core::clone::Clone for IAppxManifestMainPackageDependenciesEnumerator {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IAppxManifestMainPackageDependenciesEnumerator {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IAppxManifestMainPackageDependenciesEnumerator {}
impl ::core::fmt::Debug for IAppxManifestMainPackageDependenciesEnumerator {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IAppxManifestMainPackageDependenciesEnumerator").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Vtable for IAppxManifestMainPackageDependenciesEnumerator {
    type Vtable = IAppxManifestMainPackageDependenciesEnumerator_Vtbl;
}
unsafe impl ::windows::core::Interface for IAppxManifestMainPackageDependenciesEnumerator {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xa99c4f00_51d2_4f0f_ba46_7ed5255ebdff);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppxManifestMainPackageDependenciesEnumerator_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub GetCurrent: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, mainpackagedependency: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub GetHasCurrent: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, hascurrent: *mut super::super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetHasCurrent: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub MoveNext: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, hasnext: *mut super::super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    MoveNext: usize,
}
#[doc = "*Required features: `\"Win32_Storage_Packaging_Appx\"`*"]
#[repr(transparent)]
pub struct IAppxManifestMainPackageDependency(::windows::core::IUnknown);
impl IAppxManifestMainPackageDependency {
    pub unsafe fn GetName(&self) -> ::windows::core::Result<::windows::core::PWSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).GetName)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetPublisher(&self) -> ::windows::core::Result<::windows::core::PWSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).GetPublisher)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetPackageFamilyName(&self) -> ::windows::core::Result<::windows::core::PWSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).GetPackageFamilyName)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
}
::windows::core::interface_hierarchy!(IAppxManifestMainPackageDependency, ::windows::core::IUnknown);
impl ::core::clone::Clone for IAppxManifestMainPackageDependency {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IAppxManifestMainPackageDependency {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IAppxManifestMainPackageDependency {}
impl ::core::fmt::Debug for IAppxManifestMainPackageDependency {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IAppxManifestMainPackageDependency").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Vtable for IAppxManifestMainPackageDependency {
    type Vtable = IAppxManifestMainPackageDependency_Vtbl;
}
unsafe impl ::windows::core::Interface for IAppxManifestMainPackageDependency {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x05d0611c_bc29_46d5_97e2_84b9c79bd8ae);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppxManifestMainPackageDependency_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub GetName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, name: *mut ::windows::core::PWSTR) -> ::windows::core::HRESULT,
    pub GetPublisher: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, publisher: *mut ::windows::core::PWSTR) -> ::windows::core::HRESULT,
    pub GetPackageFamilyName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, packagefamilyname: *mut ::windows::core::PWSTR) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Storage_Packaging_Appx\"`*"]
#[repr(transparent)]
pub struct IAppxManifestOSPackageDependenciesEnumerator(::windows::core::IUnknown);
impl IAppxManifestOSPackageDependenciesEnumerator {
    pub unsafe fn GetCurrent(&self) -> ::windows::core::Result<IAppxManifestOSPackageDependency> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).GetCurrent)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetHasCurrent(&self) -> ::windows::core::Result<super::super::super::Foundation::BOOL> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).GetHasCurrent)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn MoveNext(&self) -> ::windows::core::Result<super::super::super::Foundation::BOOL> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).MoveNext)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
}
::windows::core::interface_hierarchy!(IAppxManifestOSPackageDependenciesEnumerator, ::windows::core::IUnknown);
impl ::core::clone::Clone for IAppxManifestOSPackageDependenciesEnumerator {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IAppxManifestOSPackageDependenciesEnumerator {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IAppxManifestOSPackageDependenciesEnumerator {}
impl ::core::fmt::Debug for IAppxManifestOSPackageDependenciesEnumerator {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IAppxManifestOSPackageDependenciesEnumerator").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Vtable for IAppxManifestOSPackageDependenciesEnumerator {
    type Vtable = IAppxManifestOSPackageDependenciesEnumerator_Vtbl;
}
unsafe impl ::windows::core::Interface for IAppxManifestOSPackageDependenciesEnumerator {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xb84e2fc3_f8ec_4bc1_8ae2_156346f5ffea);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppxManifestOSPackageDependenciesEnumerator_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub GetCurrent: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ospackagedependency: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub GetHasCurrent: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, hascurrent: *mut super::super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetHasCurrent: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub MoveNext: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, hasnext: *mut super::super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    MoveNext: usize,
}
#[doc = "*Required features: `\"Win32_Storage_Packaging_Appx\"`*"]
#[repr(transparent)]
pub struct IAppxManifestOSPackageDependency(::windows::core::IUnknown);
impl IAppxManifestOSPackageDependency {
    pub unsafe fn GetName(&self) -> ::windows::core::Result<::windows::core::PWSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).GetName)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetVersion(&self) -> ::windows::core::Result<u64> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).GetVersion)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
}
::windows::core::interface_hierarchy!(IAppxManifestOSPackageDependency, ::windows::core::IUnknown);
impl ::core::clone::Clone for IAppxManifestOSPackageDependency {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IAppxManifestOSPackageDependency {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IAppxManifestOSPackageDependency {}
impl ::core::fmt::Debug for IAppxManifestOSPackageDependency {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IAppxManifestOSPackageDependency").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Vtable for IAppxManifestOSPackageDependency {
    type Vtable = IAppxManifestOSPackageDependency_Vtbl;
}
unsafe impl ::windows::core::Interface for IAppxManifestOSPackageDependency {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x154995ee_54a6_4f14_ac97_d8cf0519644b);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppxManifestOSPackageDependency_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub GetName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, name: *mut ::windows::core::PWSTR) -> ::windows::core::HRESULT,
    pub GetVersion: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, version: *mut u64) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Storage_Packaging_Appx\"`*"]
#[repr(transparent)]
pub struct IAppxManifestOptionalPackageInfo(::windows::core::IUnknown);
impl IAppxManifestOptionalPackageInfo {
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetIsOptionalPackage(&self) -> ::windows::core::Result<super::super::super::Foundation::BOOL> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).GetIsOptionalPackage)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetMainPackageName(&self) -> ::windows::core::Result<::windows::core::PWSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).GetMainPackageName)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
}
::windows::core::interface_hierarchy!(IAppxManifestOptionalPackageInfo, ::windows::core::IUnknown);
impl ::core::clone::Clone for IAppxManifestOptionalPackageInfo {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IAppxManifestOptionalPackageInfo {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IAppxManifestOptionalPackageInfo {}
impl ::core::fmt::Debug for IAppxManifestOptionalPackageInfo {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IAppxManifestOptionalPackageInfo").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Vtable for IAppxManifestOptionalPackageInfo {
    type Vtable = IAppxManifestOptionalPackageInfo_Vtbl;
}
unsafe impl ::windows::core::Interface for IAppxManifestOptionalPackageInfo {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x2634847d_5b5d_4fe5_a243_002ff95edc7e);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppxManifestOptionalPackageInfo_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    #[cfg(feature = "Win32_Foundation")]
    pub GetIsOptionalPackage: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, isoptionalpackage: *mut super::super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetIsOptionalPackage: usize,
    pub GetMainPackageName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, mainpackagename: *mut ::windows::core::PWSTR) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Storage_Packaging_Appx\"`*"]
#[repr(transparent)]
pub struct IAppxManifestPackageDependenciesEnumerator(::windows::core::IUnknown);
impl IAppxManifestPackageDependenciesEnumerator {
    pub unsafe fn GetCurrent(&self) -> ::windows::core::Result<IAppxManifestPackageDependency> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).GetCurrent)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetHasCurrent(&self) -> ::windows::core::Result<super::super::super::Foundation::BOOL> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).GetHasCurrent)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn MoveNext(&self) -> ::windows::core::Result<super::super::super::Foundation::BOOL> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).MoveNext)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
}
::windows::core::interface_hierarchy!(IAppxManifestPackageDependenciesEnumerator, ::windows::core::IUnknown);
impl ::core::clone::Clone for IAppxManifestPackageDependenciesEnumerator {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IAppxManifestPackageDependenciesEnumerator {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IAppxManifestPackageDependenciesEnumerator {}
impl ::core::fmt::Debug for IAppxManifestPackageDependenciesEnumerator {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IAppxManifestPackageDependenciesEnumerator").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Vtable for IAppxManifestPackageDependenciesEnumerator {
    type Vtable = IAppxManifestPackageDependenciesEnumerator_Vtbl;
}
unsafe impl ::windows::core::Interface for IAppxManifestPackageDependenciesEnumerator {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xb43bbcf9_65a6_42dd_bac0_8c6741e7f5a4);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppxManifestPackageDependenciesEnumerator_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub GetCurrent: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dependency: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub GetHasCurrent: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, hascurrent: *mut super::super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetHasCurrent: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub MoveNext: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, hasnext: *mut super::super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    MoveNext: usize,
}
#[doc = "*Required features: `\"Win32_Storage_Packaging_Appx\"`*"]
#[repr(transparent)]
pub struct IAppxManifestPackageDependency(::windows::core::IUnknown);
impl IAppxManifestPackageDependency {
    pub unsafe fn GetName(&self) -> ::windows::core::Result<::windows::core::PWSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).GetName)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetPublisher(&self) -> ::windows::core::Result<::windows::core::PWSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).GetPublisher)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetMinVersion(&self) -> ::windows::core::Result<u64> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).GetMinVersion)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
}
::windows::core::interface_hierarchy!(IAppxManifestPackageDependency, ::windows::core::IUnknown);
impl ::core::clone::Clone for IAppxManifestPackageDependency {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IAppxManifestPackageDependency {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IAppxManifestPackageDependency {}
impl ::core::fmt::Debug for IAppxManifestPackageDependency {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IAppxManifestPackageDependency").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Vtable for IAppxManifestPackageDependency {
    type Vtable = IAppxManifestPackageDependency_Vtbl;
}
unsafe impl ::windows::core::Interface for IAppxManifestPackageDependency {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xe4946b59_733e_43f0_a724_3bde4c1285a0);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppxManifestPackageDependency_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub GetName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, name: *mut ::windows::core::PWSTR) -> ::windows::core::HRESULT,
    pub GetPublisher: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, publisher: *mut ::windows::core::PWSTR) -> ::windows::core::HRESULT,
    pub GetMinVersion: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, minversion: *mut u64) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Storage_Packaging_Appx\"`*"]
#[repr(transparent)]
pub struct IAppxManifestPackageDependency2(::windows::core::IUnknown);
impl IAppxManifestPackageDependency2 {
    pub unsafe fn GetName(&self) -> ::windows::core::Result<::windows::core::PWSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetName)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetPublisher(&self) -> ::windows::core::Result<::windows::core::PWSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetPublisher)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetMinVersion(&self) -> ::windows::core::Result<u64> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetMinVersion)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetMaxMajorVersionTested(&self) -> ::windows::core::Result<u16> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).GetMaxMajorVersionTested)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
}
::windows::core::interface_hierarchy!(IAppxManifestPackageDependency2, ::windows::core::IUnknown, IAppxManifestPackageDependency);
impl ::core::clone::Clone for IAppxManifestPackageDependency2 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IAppxManifestPackageDependency2 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IAppxManifestPackageDependency2 {}
impl ::core::fmt::Debug for IAppxManifestPackageDependency2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IAppxManifestPackageDependency2").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Vtable for IAppxManifestPackageDependency2 {
    type Vtable = IAppxManifestPackageDependency2_Vtbl;
}
unsafe impl ::windows::core::Interface for IAppxManifestPackageDependency2 {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xdda0b713_f3ff_49d3_898a_2786780c5d98);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppxManifestPackageDependency2_Vtbl {
    pub base__: IAppxManifestPackageDependency_Vtbl,
    pub GetMaxMajorVersionTested: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, maxmajorversiontested: *mut u16) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Storage_Packaging_Appx\"`*"]
#[repr(transparent)]
pub struct IAppxManifestPackageDependency3(::windows::core::IUnknown);
impl IAppxManifestPackageDependency3 {
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetIsOptional(&self) -> ::windows::core::Result<super::super::super::Foundation::BOOL> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).GetIsOptional)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
}
::windows::core::interface_hierarchy!(IAppxManifestPackageDependency3, ::windows::core::IUnknown);
impl ::core::clone::Clone for IAppxManifestPackageDependency3 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IAppxManifestPackageDependency3 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IAppxManifestPackageDependency3 {}
impl ::core::fmt::Debug for IAppxManifestPackageDependency3 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IAppxManifestPackageDependency3").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Vtable for IAppxManifestPackageDependency3 {
    type Vtable = IAppxManifestPackageDependency3_Vtbl;
}
unsafe impl ::windows::core::Interface for IAppxManifestPackageDependency3 {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x1ac56374_6198_4d6b_92e4_749d5ab8a895);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppxManifestPackageDependency3_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    #[cfg(feature = "Win32_Foundation")]
    pub GetIsOptional: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, isoptional: *mut super::super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetIsOptional: usize,
}
#[doc = "*Required features: `\"Win32_Storage_Packaging_Appx\"`*"]
#[repr(transparent)]
pub struct IAppxManifestPackageId(::windows::core::IUnknown);
impl IAppxManifestPackageId {
    pub unsafe fn GetName(&self) -> ::windows::core::Result<::windows::core::PWSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).GetName)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetArchitecture(&self) -> ::windows::core::Result<APPX_PACKAGE_ARCHITECTURE> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).GetArchitecture)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetPublisher(&self) -> ::windows::core::Result<::windows::core::PWSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).GetPublisher)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetVersion(&self) -> ::windows::core::Result<u64> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).GetVersion)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetResourceId(&self) -> ::windows::core::Result<::windows::core::PWSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).GetResourceId)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn ComparePublisher<P0>(&self, other: P0) -> ::windows::core::Result<super::super::super::Foundation::BOOL>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).ComparePublisher)(::windows::core::Vtable::as_raw(self), other.into().abi(), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetPackageFullName(&self) -> ::windows::core::Result<::windows::core::PWSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).GetPackageFullName)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetPackageFamilyName(&self) -> ::windows::core::Result<::windows::core::PWSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).GetPackageFamilyName)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
}
::windows::core::interface_hierarchy!(IAppxManifestPackageId, ::windows::core::IUnknown);
impl ::core::clone::Clone for IAppxManifestPackageId {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IAppxManifestPackageId {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IAppxManifestPackageId {}
impl ::core::fmt::Debug for IAppxManifestPackageId {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IAppxManifestPackageId").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Vtable for IAppxManifestPackageId {
    type Vtable = IAppxManifestPackageId_Vtbl;
}
unsafe impl ::windows::core::Interface for IAppxManifestPackageId {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x283ce2d7_7153_4a91_9649_7a0f7240945f);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppxManifestPackageId_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub GetName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, name: *mut ::windows::core::PWSTR) -> ::windows::core::HRESULT,
    pub GetArchitecture: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, architecture: *mut APPX_PACKAGE_ARCHITECTURE) -> ::windows::core::HRESULT,
    pub GetPublisher: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, publisher: *mut ::windows::core::PWSTR) -> ::windows::core::HRESULT,
    pub GetVersion: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, packageversion: *mut u64) -> ::windows::core::HRESULT,
    pub GetResourceId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, resourceid: *mut ::windows::core::PWSTR) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub ComparePublisher: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, other: ::windows::core::PCWSTR, issame: *mut super::super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    ComparePublisher: usize,
    pub GetPackageFullName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, packagefullname: *mut ::windows::core::PWSTR) -> ::windows::core::HRESULT,
    pub GetPackageFamilyName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, packagefamilyname: *mut ::windows::core::PWSTR) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Storage_Packaging_Appx\"`*"]
#[repr(transparent)]
pub struct IAppxManifestPackageId2(::windows::core::IUnknown);
impl IAppxManifestPackageId2 {
    pub unsafe fn GetName(&self) -> ::windows::core::Result<::windows::core::PWSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetName)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetArchitecture(&self) -> ::windows::core::Result<APPX_PACKAGE_ARCHITECTURE> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetArchitecture)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetPublisher(&self) -> ::windows::core::Result<::windows::core::PWSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetPublisher)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetVersion(&self) -> ::windows::core::Result<u64> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetVersion)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetResourceId(&self) -> ::windows::core::Result<::windows::core::PWSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetResourceId)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn ComparePublisher<P0>(&self, other: P0) -> ::windows::core::Result<super::super::super::Foundation::BOOL>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.ComparePublisher)(::windows::core::Vtable::as_raw(self), other.into().abi(), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetPackageFullName(&self) -> ::windows::core::Result<::windows::core::PWSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetPackageFullName)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetPackageFamilyName(&self) -> ::windows::core::Result<::windows::core::PWSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetPackageFamilyName)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetArchitecture2(&self) -> ::windows::core::Result<APPX_PACKAGE_ARCHITECTURE2> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).GetArchitecture2)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
}
::windows::core::interface_hierarchy!(IAppxManifestPackageId2, ::windows::core::IUnknown, IAppxManifestPackageId);
impl ::core::clone::Clone for IAppxManifestPackageId2 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IAppxManifestPackageId2 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IAppxManifestPackageId2 {}
impl ::core::fmt::Debug for IAppxManifestPackageId2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IAppxManifestPackageId2").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Vtable for IAppxManifestPackageId2 {
    type Vtable = IAppxManifestPackageId2_Vtbl;
}
unsafe impl ::windows::core::Interface for IAppxManifestPackageId2 {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x2256999d_d617_42f1_880e_0ba4542319d5);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppxManifestPackageId2_Vtbl {
    pub base__: IAppxManifestPackageId_Vtbl,
    pub GetArchitecture2: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, architecture: *mut APPX_PACKAGE_ARCHITECTURE2) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Storage_Packaging_Appx\"`*"]
#[repr(transparent)]
pub struct IAppxManifestProperties(::windows::core::IUnknown);
impl IAppxManifestProperties {
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetBoolValue<P0>(&self, name: P0) -> ::windows::core::Result<super::super::super::Foundation::BOOL>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).GetBoolValue)(::windows::core::Vtable::as_raw(self), name.into().abi(), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetStringValue<P0>(&self, name: P0) -> ::windows::core::Result<::windows::core::PWSTR>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).GetStringValue)(::windows::core::Vtable::as_raw(self), name.into().abi(), result__.as_mut_ptr()).from_abi(result__)
    }
}
::windows::core::interface_hierarchy!(IAppxManifestProperties, ::windows::core::IUnknown);
impl ::core::clone::Clone for IAppxManifestProperties {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IAppxManifestProperties {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IAppxManifestProperties {}
impl ::core::fmt::Debug for IAppxManifestProperties {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IAppxManifestProperties").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Vtable for IAppxManifestProperties {
    type Vtable = IAppxManifestProperties_Vtbl;
}
unsafe impl ::windows::core::Interface for IAppxManifestProperties {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x03faf64d_f26f_4b2c_aaf7_8fe7789b8bca);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppxManifestProperties_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    #[cfg(feature = "Win32_Foundation")]
    pub GetBoolValue: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, name: ::windows::core::PCWSTR, value: *mut super::super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetBoolValue: usize,
    pub GetStringValue: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, name: ::windows::core::PCWSTR, value: *mut ::windows::core::PWSTR) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Storage_Packaging_Appx\"`*"]
#[repr(transparent)]
pub struct IAppxManifestQualifiedResource(::windows::core::IUnknown);
impl IAppxManifestQualifiedResource {
    pub unsafe fn GetLanguage(&self) -> ::windows::core::Result<::windows::core::PWSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).GetLanguage)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetScale(&self) -> ::windows::core::Result<u32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).GetScale)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetDXFeatureLevel(&self) -> ::windows::core::Result<DX_FEATURE_LEVEL> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).GetDXFeatureLevel)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
}
::windows::core::interface_hierarchy!(IAppxManifestQualifiedResource, ::windows::core::IUnknown);
impl ::core::clone::Clone for IAppxManifestQualifiedResource {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IAppxManifestQualifiedResource {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IAppxManifestQualifiedResource {}
impl ::core::fmt::Debug for IAppxManifestQualifiedResource {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IAppxManifestQualifiedResource").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Vtable for IAppxManifestQualifiedResource {
    type Vtable = IAppxManifestQualifiedResource_Vtbl;
}
unsafe impl ::windows::core::Interface for IAppxManifestQualifiedResource {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x3b53a497_3c5c_48d1_9ea3_bb7eac8cd7d4);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppxManifestQualifiedResource_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub GetLanguage: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, language: *mut ::windows::core::PWSTR) -> ::windows::core::HRESULT,
    pub GetScale: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, scale: *mut u32) -> ::windows::core::HRESULT,
    pub GetDXFeatureLevel: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dxfeaturelevel: *mut DX_FEATURE_LEVEL) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Storage_Packaging_Appx\"`*"]
#[repr(transparent)]
pub struct IAppxManifestQualifiedResourcesEnumerator(::windows::core::IUnknown);
impl IAppxManifestQualifiedResourcesEnumerator {
    pub unsafe fn GetCurrent(&self) -> ::windows::core::Result<IAppxManifestQualifiedResource> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).GetCurrent)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetHasCurrent(&self) -> ::windows::core::Result<super::super::super::Foundation::BOOL> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).GetHasCurrent)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn MoveNext(&self) -> ::windows::core::Result<super::super::super::Foundation::BOOL> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).MoveNext)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
}
::windows::core::interface_hierarchy!(IAppxManifestQualifiedResourcesEnumerator, ::windows::core::IUnknown);
impl ::core::clone::Clone for IAppxManifestQualifiedResourcesEnumerator {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IAppxManifestQualifiedResourcesEnumerator {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IAppxManifestQualifiedResourcesEnumerator {}
impl ::core::fmt::Debug for IAppxManifestQualifiedResourcesEnumerator {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IAppxManifestQualifiedResourcesEnumerator").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Vtable for IAppxManifestQualifiedResourcesEnumerator {
    type Vtable = IAppxManifestQualifiedResourcesEnumerator_Vtbl;
}
unsafe impl ::windows::core::Interface for IAppxManifestQualifiedResourcesEnumerator {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x8ef6adfe_3762_4a8f_9373_2fc5d444c8d2);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppxManifestQualifiedResourcesEnumerator_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub GetCurrent: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, resource: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub GetHasCurrent: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, hascurrent: *mut super::super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetHasCurrent: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub MoveNext: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, hasnext: *mut super::super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    MoveNext: usize,
}
#[doc = "*Required features: `\"Win32_Storage_Packaging_Appx\"`*"]
#[repr(transparent)]
pub struct IAppxManifestReader(::windows::core::IUnknown);
impl IAppxManifestReader {
    pub unsafe fn GetPackageId(&self) -> ::windows::core::Result<IAppxManifestPackageId> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).GetPackageId)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetProperties(&self) -> ::windows::core::Result<IAppxManifestProperties> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).GetProperties)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetPackageDependencies(&self) -> ::windows::core::Result<IAppxManifestPackageDependenciesEnumerator> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).GetPackageDependencies)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetCapabilities(&self) -> ::windows::core::Result<APPX_CAPABILITIES> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).GetCapabilities)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetResources(&self) -> ::windows::core::Result<IAppxManifestResourcesEnumerator> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).GetResources)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetDeviceCapabilities(&self) -> ::windows::core::Result<IAppxManifestDeviceCapabilitiesEnumerator> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).GetDeviceCapabilities)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetPrerequisite<P0>(&self, name: P0) -> ::windows::core::Result<u64>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).GetPrerequisite)(::windows::core::Vtable::as_raw(self), name.into().abi(), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetApplications(&self) -> ::windows::core::Result<IAppxManifestApplicationsEnumerator> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).GetApplications)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetStream(&self) -> ::windows::core::Result<super::super::super::System::Com::IStream> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).GetStream)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
}
::windows::core::interface_hierarchy!(IAppxManifestReader, ::windows::core::IUnknown);
impl ::core::clone::Clone for IAppxManifestReader {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IAppxManifestReader {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IAppxManifestReader {}
impl ::core::fmt::Debug for IAppxManifestReader {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IAppxManifestReader").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Vtable for IAppxManifestReader {
    type Vtable = IAppxManifestReader_Vtbl;
}
unsafe impl ::windows::core::Interface for IAppxManifestReader {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x4e1bd148_55a0_4480_a3d1_15544710637c);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppxManifestReader_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub GetPackageId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, packageid: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub GetProperties: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, packageproperties: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub GetPackageDependencies: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dependencies: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub GetCapabilities: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, capabilities: *mut APPX_CAPABILITIES) -> ::windows::core::HRESULT,
    pub GetResources: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, resources: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub GetDeviceCapabilities: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, devicecapabilities: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub GetPrerequisite: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, name: ::windows::core::PCWSTR, value: *mut u64) -> ::windows::core::HRESULT,
    pub GetApplications: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, applications: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub GetStream: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, manifeststream: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    GetStream: usize,
}
#[doc = "*Required features: `\"Win32_Storage_Packaging_Appx\"`*"]
#[repr(transparent)]
pub struct IAppxManifestReader2(::windows::core::IUnknown);
impl IAppxManifestReader2 {
    pub unsafe fn GetPackageId(&self) -> ::windows::core::Result<IAppxManifestPackageId> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetPackageId)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetProperties(&self) -> ::windows::core::Result<IAppxManifestProperties> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetProperties)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetPackageDependencies(&self) -> ::windows::core::Result<IAppxManifestPackageDependenciesEnumerator> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetPackageDependencies)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetCapabilities(&self) -> ::windows::core::Result<APPX_CAPABILITIES> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetCapabilities)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetResources(&self) -> ::windows::core::Result<IAppxManifestResourcesEnumerator> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetResources)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetDeviceCapabilities(&self) -> ::windows::core::Result<IAppxManifestDeviceCapabilitiesEnumerator> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetDeviceCapabilities)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetPrerequisite<P0>(&self, name: P0) -> ::windows::core::Result<u64>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetPrerequisite)(::windows::core::Vtable::as_raw(self), name.into().abi(), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetApplications(&self) -> ::windows::core::Result<IAppxManifestApplicationsEnumerator> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetApplications)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetStream(&self) -> ::windows::core::Result<super::super::super::System::Com::IStream> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetStream)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetQualifiedResources(&self) -> ::windows::core::Result<IAppxManifestQualifiedResourcesEnumerator> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).GetQualifiedResources)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
}
::windows::core::interface_hierarchy!(IAppxManifestReader2, ::windows::core::IUnknown, IAppxManifestReader);
impl ::core::clone::Clone for IAppxManifestReader2 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IAppxManifestReader2 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IAppxManifestReader2 {}
impl ::core::fmt::Debug for IAppxManifestReader2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IAppxManifestReader2").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Vtable for IAppxManifestReader2 {
    type Vtable = IAppxManifestReader2_Vtbl;
}
unsafe impl ::windows::core::Interface for IAppxManifestReader2 {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xd06f67bc_b31d_4eba_a8af_638e73e77b4d);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppxManifestReader2_Vtbl {
    pub base__: IAppxManifestReader_Vtbl,
    pub GetQualifiedResources: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, resources: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Storage_Packaging_Appx\"`*"]
#[repr(transparent)]
pub struct IAppxManifestReader3(::windows::core::IUnknown);
impl IAppxManifestReader3 {
    pub unsafe fn GetPackageId(&self) -> ::windows::core::Result<IAppxManifestPackageId> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.GetPackageId)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetProperties(&self) -> ::windows::core::Result<IAppxManifestProperties> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.GetProperties)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetPackageDependencies(&self) -> ::windows::core::Result<IAppxManifestPackageDependenciesEnumerator> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.GetPackageDependencies)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetCapabilities(&self) -> ::windows::core::Result<APPX_CAPABILITIES> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.GetCapabilities)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetResources(&self) -> ::windows::core::Result<IAppxManifestResourcesEnumerator> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.GetResources)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetDeviceCapabilities(&self) -> ::windows::core::Result<IAppxManifestDeviceCapabilitiesEnumerator> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.GetDeviceCapabilities)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetPrerequisite<P0>(&self, name: P0) -> ::windows::core::Result<u64>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.GetPrerequisite)(::windows::core::Vtable::as_raw(self), name.into().abi(), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetApplications(&self) -> ::windows::core::Result<IAppxManifestApplicationsEnumerator> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.GetApplications)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetStream(&self) -> ::windows::core::Result<super::super::super::System::Com::IStream> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.GetStream)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetQualifiedResources(&self) -> ::windows::core::Result<IAppxManifestQualifiedResourcesEnumerator> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetQualifiedResources)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetCapabilitiesByCapabilityClass(&self, capabilityclass: APPX_CAPABILITY_CLASS_TYPE) -> ::windows::core::Result<IAppxManifestCapabilitiesEnumerator> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).GetCapabilitiesByCapabilityClass)(::windows::core::Vtable::as_raw(self), capabilityclass, result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetTargetDeviceFamilies(&self) -> ::windows::core::Result<IAppxManifestTargetDeviceFamiliesEnumerator> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).GetTargetDeviceFamilies)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
}
::windows::core::interface_hierarchy!(IAppxManifestReader3, ::windows::core::IUnknown, IAppxManifestReader, IAppxManifestReader2);
impl ::core::clone::Clone for IAppxManifestReader3 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IAppxManifestReader3 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IAppxManifestReader3 {}
impl ::core::fmt::Debug for IAppxManifestReader3 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IAppxManifestReader3").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Vtable for IAppxManifestReader3 {
    type Vtable = IAppxManifestReader3_Vtbl;
}
unsafe impl ::windows::core::Interface for IAppxManifestReader3 {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xc43825ab_69b7_400a_9709_cc37f5a72d24);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppxManifestReader3_Vtbl {
    pub base__: IAppxManifestReader2_Vtbl,
    pub GetCapabilitiesByCapabilityClass: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, capabilityclass: APPX_CAPABILITY_CLASS_TYPE, capabilities: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub GetTargetDeviceFamilies: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, targetdevicefamilies: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Storage_Packaging_Appx\"`*"]
#[repr(transparent)]
pub struct IAppxManifestReader4(::windows::core::IUnknown);
impl IAppxManifestReader4 {
    pub unsafe fn GetPackageId(&self) -> ::windows::core::Result<IAppxManifestPackageId> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.GetPackageId)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetProperties(&self) -> ::windows::core::Result<IAppxManifestProperties> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.GetProperties)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetPackageDependencies(&self) -> ::windows::core::Result<IAppxManifestPackageDependenciesEnumerator> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.GetPackageDependencies)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetCapabilities(&self) -> ::windows::core::Result<APPX_CAPABILITIES> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.GetCapabilities)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetResources(&self) -> ::windows::core::Result<IAppxManifestResourcesEnumerator> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.GetResources)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetDeviceCapabilities(&self) -> ::windows::core::Result<IAppxManifestDeviceCapabilitiesEnumerator> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.GetDeviceCapabilities)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetPrerequisite<P0>(&self, name: P0) -> ::windows::core::Result<u64>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.GetPrerequisite)(::windows::core::Vtable::as_raw(self), name.into().abi(), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetApplications(&self) -> ::windows::core::Result<IAppxManifestApplicationsEnumerator> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.GetApplications)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetStream(&self) -> ::windows::core::Result<super::super::super::System::Com::IStream> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.GetStream)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetQualifiedResources(&self) -> ::windows::core::Result<IAppxManifestQualifiedResourcesEnumerator> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.GetQualifiedResources)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetCapabilitiesByCapabilityClass(&self, capabilityclass: APPX_CAPABILITY_CLASS_TYPE) -> ::windows::core::Result<IAppxManifestCapabilitiesEnumerator> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetCapabilitiesByCapabilityClass)(::windows::core::Vtable::as_raw(self), capabilityclass, result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetTargetDeviceFamilies(&self) -> ::windows::core::Result<IAppxManifestTargetDeviceFamiliesEnumerator> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetTargetDeviceFamilies)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetOptionalPackageInfo(&self) -> ::windows::core::Result<IAppxManifestOptionalPackageInfo> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).GetOptionalPackageInfo)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
}
::windows::core::interface_hierarchy!(IAppxManifestReader4, ::windows::core::IUnknown, IAppxManifestReader, IAppxManifestReader2, IAppxManifestReader3);
impl ::core::clone::Clone for IAppxManifestReader4 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IAppxManifestReader4 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IAppxManifestReader4 {}
impl ::core::fmt::Debug for IAppxManifestReader4 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IAppxManifestReader4").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Vtable for IAppxManifestReader4 {
    type Vtable = IAppxManifestReader4_Vtbl;
}
unsafe impl ::windows::core::Interface for IAppxManifestReader4 {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x4579bb7c_741d_4161_b5a1_47bd3b78ad9b);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppxManifestReader4_Vtbl {
    pub base__: IAppxManifestReader3_Vtbl,
    pub GetOptionalPackageInfo: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, optionalpackageinfo: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Storage_Packaging_Appx\"`*"]
#[repr(transparent)]
pub struct IAppxManifestReader5(::windows::core::IUnknown);
impl IAppxManifestReader5 {
    pub unsafe fn GetMainPackageDependencies(&self) -> ::windows::core::Result<IAppxManifestMainPackageDependenciesEnumerator> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).GetMainPackageDependencies)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
}
::windows::core::interface_hierarchy!(IAppxManifestReader5, ::windows::core::IUnknown);
impl ::core::clone::Clone for IAppxManifestReader5 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IAppxManifestReader5 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IAppxManifestReader5 {}
impl ::core::fmt::Debug for IAppxManifestReader5 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IAppxManifestReader5").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Vtable for IAppxManifestReader5 {
    type Vtable = IAppxManifestReader5_Vtbl;
}
unsafe impl ::windows::core::Interface for IAppxManifestReader5 {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x8d7ae132_a690_4c00_b75a_6aae1feaac80);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppxManifestReader5_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub GetMainPackageDependencies: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, mainpackagedependencies: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Storage_Packaging_Appx\"`*"]
#[repr(transparent)]
pub struct IAppxManifestReader6(::windows::core::IUnknown);
impl IAppxManifestReader6 {
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetIsNonQualifiedResourcePackage(&self) -> ::windows::core::Result<super::super::super::Foundation::BOOL> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).GetIsNonQualifiedResourcePackage)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
}
::windows::core::interface_hierarchy!(IAppxManifestReader6, ::windows::core::IUnknown);
impl ::core::clone::Clone for IAppxManifestReader6 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IAppxManifestReader6 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IAppxManifestReader6 {}
impl ::core::fmt::Debug for IAppxManifestReader6 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IAppxManifestReader6").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Vtable for IAppxManifestReader6 {
    type Vtable = IAppxManifestReader6_Vtbl;
}
unsafe impl ::windows::core::Interface for IAppxManifestReader6 {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x34deaca4_d3c0_4e3e_b312_e42625e3807e);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppxManifestReader6_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    #[cfg(feature = "Win32_Foundation")]
    pub GetIsNonQualifiedResourcePackage: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, isnonqualifiedresourcepackage: *mut super::super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetIsNonQualifiedResourcePackage: usize,
}
#[doc = "*Required features: `\"Win32_Storage_Packaging_Appx\"`*"]
#[repr(transparent)]
pub struct IAppxManifestReader7(::windows::core::IUnknown);
impl IAppxManifestReader7 {
    pub unsafe fn GetDriverDependencies(&self) -> ::windows::core::Result<IAppxManifestDriverDependenciesEnumerator> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).GetDriverDependencies)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetOSPackageDependencies(&self) -> ::windows::core::Result<IAppxManifestOSPackageDependenciesEnumerator> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).GetOSPackageDependencies)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetHostRuntimeDependencies(&self) -> ::windows::core::Result<IAppxManifestHostRuntimeDependenciesEnumerator> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).GetHostRuntimeDependencies)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
}
::windows::core::interface_hierarchy!(IAppxManifestReader7, ::windows::core::IUnknown);
impl ::core::clone::Clone for IAppxManifestReader7 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IAppxManifestReader7 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IAppxManifestReader7 {}
impl ::core::fmt::Debug for IAppxManifestReader7 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IAppxManifestReader7").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Vtable for IAppxManifestReader7 {
    type Vtable = IAppxManifestReader7_Vtbl;
}
unsafe impl ::windows::core::Interface for IAppxManifestReader7 {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x8efe6f27_0ce0_4988_b32d_738eb63db3b7);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppxManifestReader7_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub GetDriverDependencies: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, driverdependencies: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub GetOSPackageDependencies: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ospackagedependencies: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub GetHostRuntimeDependencies: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, hostruntimedependencies: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Storage_Packaging_Appx\"`*"]
#[repr(transparent)]
pub struct IAppxManifestResourcesEnumerator(::windows::core::IUnknown);
impl IAppxManifestResourcesEnumerator {
    pub unsafe fn GetCurrent(&self) -> ::windows::core::Result<::windows::core::PWSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).GetCurrent)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetHasCurrent(&self) -> ::windows::core::Result<super::super::super::Foundation::BOOL> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).GetHasCurrent)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn MoveNext(&self) -> ::windows::core::Result<super::super::super::Foundation::BOOL> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).MoveNext)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
}
::windows::core::interface_hierarchy!(IAppxManifestResourcesEnumerator, ::windows::core::IUnknown);
impl ::core::clone::Clone for IAppxManifestResourcesEnumerator {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IAppxManifestResourcesEnumerator {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IAppxManifestResourcesEnumerator {}
impl ::core::fmt::Debug for IAppxManifestResourcesEnumerator {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IAppxManifestResourcesEnumerator").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Vtable for IAppxManifestResourcesEnumerator {
    type Vtable = IAppxManifestResourcesEnumerator_Vtbl;
}
unsafe impl ::windows::core::Interface for IAppxManifestResourcesEnumerator {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xde4dfbbd_881a_48bb_858c_d6f2baeae6ed);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppxManifestResourcesEnumerator_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub GetCurrent: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, resource: *mut ::windows::core::PWSTR) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub GetHasCurrent: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, hascurrent: *mut super::super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetHasCurrent: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub MoveNext: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, hasnext: *mut super::super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    MoveNext: usize,
}
#[doc = "*Required features: `\"Win32_Storage_Packaging_Appx\"`*"]
#[repr(transparent)]
pub struct IAppxManifestTargetDeviceFamiliesEnumerator(::windows::core::IUnknown);
impl IAppxManifestTargetDeviceFamiliesEnumerator {
    pub unsafe fn GetCurrent(&self) -> ::windows::core::Result<IAppxManifestTargetDeviceFamily> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).GetCurrent)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetHasCurrent(&self) -> ::windows::core::Result<super::super::super::Foundation::BOOL> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).GetHasCurrent)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn MoveNext(&self) -> ::windows::core::Result<super::super::super::Foundation::BOOL> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).MoveNext)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
}
::windows::core::interface_hierarchy!(IAppxManifestTargetDeviceFamiliesEnumerator, ::windows::core::IUnknown);
impl ::core::clone::Clone for IAppxManifestTargetDeviceFamiliesEnumerator {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IAppxManifestTargetDeviceFamiliesEnumerator {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IAppxManifestTargetDeviceFamiliesEnumerator {}
impl ::core::fmt::Debug for IAppxManifestTargetDeviceFamiliesEnumerator {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IAppxManifestTargetDeviceFamiliesEnumerator").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Vtable for IAppxManifestTargetDeviceFamiliesEnumerator {
    type Vtable = IAppxManifestTargetDeviceFamiliesEnumerator_Vtbl;
}
unsafe impl ::windows::core::Interface for IAppxManifestTargetDeviceFamiliesEnumerator {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x36537f36_27a4_4788_88c0_733819575017);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppxManifestTargetDeviceFamiliesEnumerator_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub GetCurrent: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, targetdevicefamily: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub GetHasCurrent: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, hascurrent: *mut super::super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetHasCurrent: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub MoveNext: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, hasnext: *mut super::super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    MoveNext: usize,
}
#[doc = "*Required features: `\"Win32_Storage_Packaging_Appx\"`*"]
#[repr(transparent)]
pub struct IAppxManifestTargetDeviceFamily(::windows::core::IUnknown);
impl IAppxManifestTargetDeviceFamily {
    pub unsafe fn GetName(&self) -> ::windows::core::Result<::windows::core::PWSTR> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).GetName)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetMinVersion(&self) -> ::windows::core::Result<u64> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).GetMinVersion)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetMaxVersionTested(&self) -> ::windows::core::Result<u64> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).GetMaxVersionTested)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
}
::windows::core::interface_hierarchy!(IAppxManifestTargetDeviceFamily, ::windows::core::IUnknown);
impl ::core::clone::Clone for IAppxManifestTargetDeviceFamily {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IAppxManifestTargetDeviceFamily {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IAppxManifestTargetDeviceFamily {}
impl ::core::fmt::Debug for IAppxManifestTargetDeviceFamily {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IAppxManifestTargetDeviceFamily").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Vtable for IAppxManifestTargetDeviceFamily {
    type Vtable = IAppxManifestTargetDeviceFamily_Vtbl;
}
unsafe impl ::windows::core::Interface for IAppxManifestTargetDeviceFamily {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x9091b09b_c8d5_4f31_8687_a338259faefb);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppxManifestTargetDeviceFamily_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub GetName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, name: *mut ::windows::core::PWSTR) -> ::windows::core::HRESULT,
    pub GetMinVersion: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, minversion: *mut u64) -> ::windows::core::HRESULT,
    pub GetMaxVersionTested: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, maxversiontested: *mut u64) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Storage_Packaging_Appx\"`*"]
#[repr(transparent)]
pub struct IAppxPackageEditor(::windows::core::IUnknown);
impl IAppxPackageEditor {
    pub unsafe fn SetWorkingDirectory<P0>(&self, workingdirectory: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
    {
        (::windows::core::Vtable::vtable(self).SetWorkingDirectory)(::windows::core::Vtable::as_raw(self), workingdirectory.into().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn CreateDeltaPackage<P0, P1, P2>(&self, updatedpackagestream: P0, baselinepackagestream: P1, deltapackagestream: P2) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<super::super::super::System::Com::IStream>>,
        P1: ::std::convert::Into<::windows::core::InParam<super::super::super::System::Com::IStream>>,
        P2: ::std::convert::Into<::windows::core::InParam<super::super::super::System::Com::IStream>>,
    {
        (::windows::core::Vtable::vtable(self).CreateDeltaPackage)(::windows::core::Vtable::as_raw(self), updatedpackagestream.into().abi(), baselinepackagestream.into().abi(), deltapackagestream.into().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn CreateDeltaPackageUsingBaselineBlockMap<P0, P1, P2, P3>(&self, updatedpackagestream: P0, baselineblockmapstream: P1, baselinepackagefullname: P2, deltapackagestream: P3) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<super::super::super::System::Com::IStream>>,
        P1: ::std::convert::Into<::windows::core::InParam<super::super::super::System::Com::IStream>>,
        P2: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
        P3: ::std::convert::Into<::windows::core::InParam<super::super::super::System::Com::IStream>>,
    {
        (::windows::core::Vtable::vtable(self).CreateDeltaPackageUsingBaselineBlockMap)(::windows::core::Vtable::as_raw(self), updatedpackagestream.into().abi(), baselineblockmapstream.into().abi(), baselinepackagefullname.into().abi(), deltapackagestream.into().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn UpdatePackage<P0, P1>(&self, baselinepackagestream: P0, deltapackagestream: P1, updateoption: APPX_PACKAGE_EDITOR_UPDATE_PACKAGE_OPTION) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<super::super::super::System::Com::IStream>>,
        P1: ::std::convert::Into<::windows::core::InParam<super::super::super::System::Com::IStream>>,
    {
        (::windows::core::Vtable::vtable(self).UpdatePackage)(::windows::core::Vtable::as_raw(self), baselinepackagestream.into().abi(), deltapackagestream.into().abi(), updateoption).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn UpdateEncryptedPackage<P0, P1>(&self, baselineencryptedpackagestream: P0, deltapackagestream: P1, updateoption: APPX_PACKAGE_EDITOR_UPDATE_PACKAGE_OPTION, settings: *const APPX_ENCRYPTED_PACKAGE_SETTINGS2, keyinfo: *const APPX_KEY_INFO) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<super::super::super::System::Com::IStream>>,
        P1: ::std::convert::Into<::windows::core::InParam<super::super::super::System::Com::IStream>>,
    {
        (::windows::core::Vtable::vtable(self).UpdateEncryptedPackage)(::windows::core::Vtable::as_raw(self), baselineencryptedpackagestream.into().abi(), deltapackagestream.into().abi(), updateoption, settings, keyinfo).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub unsafe fn UpdatePackageManifest<P0, P1, P2>(&self, packagestream: P0, updatedmanifeststream: P1, ispackageencrypted: P2, options: APPX_PACKAGE_EDITOR_UPDATE_PACKAGE_MANIFEST_OPTIONS) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<super::super::super::System::Com::IStream>>,
        P1: ::std::convert::Into<::windows::core::InParam<super::super::super::System::Com::IStream>>,
        P2: ::std::convert::Into<super::super::super::Foundation::BOOL>,
    {
        (::windows::core::Vtable::vtable(self).UpdatePackageManifest)(::windows::core::Vtable::as_raw(self), packagestream.into().abi(), updatedmanifeststream.into().abi(), ispackageencrypted.into(), options).ok()
    }
}
::windows::core::interface_hierarchy!(IAppxPackageEditor, ::windows::core::IUnknown);
impl ::core::clone::Clone for IAppxPackageEditor {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IAppxPackageEditor {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IAppxPackageEditor {}
impl ::core::fmt::Debug for IAppxPackageEditor {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IAppxPackageEditor").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Vtable for IAppxPackageEditor {
    type Vtable = IAppxPackageEditor_Vtbl;
}
unsafe impl ::windows::core::Interface for IAppxPackageEditor {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xe2adb6dc_5e71_4416_86b6_86e5f5291a6b);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppxPackageEditor_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub SetWorkingDirectory: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, workingdirectory: ::windows::core::PCWSTR) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub CreateDeltaPackage: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, updatedpackagestream: *mut ::core::ffi::c_void, baselinepackagestream: *mut ::core::ffi::c_void, deltapackagestream: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    CreateDeltaPackage: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub CreateDeltaPackageUsingBaselineBlockMap: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, updatedpackagestream: *mut ::core::ffi::c_void, baselineblockmapstream: *mut ::core::ffi::c_void, baselinepackagefullname: ::windows::core::PCWSTR, deltapackagestream: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    CreateDeltaPackageUsingBaselineBlockMap: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub UpdatePackage: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, baselinepackagestream: *mut ::core::ffi::c_void, deltapackagestream: *mut ::core::ffi::c_void, updateoption: APPX_PACKAGE_EDITOR_UPDATE_PACKAGE_OPTION) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    UpdatePackage: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub UpdateEncryptedPackage: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, baselineencryptedpackagestream: *mut ::core::ffi::c_void, deltapackagestream: *mut ::core::ffi::c_void, updateoption: APPX_PACKAGE_EDITOR_UPDATE_PACKAGE_OPTION, settings: *const APPX_ENCRYPTED_PACKAGE_SETTINGS2, keyinfo: *const APPX_KEY_INFO) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    UpdateEncryptedPackage: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub UpdatePackageManifest: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, packagestream: *mut ::core::ffi::c_void, updatedmanifeststream: *mut ::core::ffi::c_void, ispackageencrypted: super::super::super::Foundation::BOOL, options: APPX_PACKAGE_EDITOR_UPDATE_PACKAGE_MANIFEST_OPTIONS) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com")))]
    UpdatePackageManifest: usize,
}
#[doc = "*Required features: `\"Win32_Storage_Packaging_Appx\"`*"]
#[repr(transparent)]
pub struct IAppxPackageReader(::windows::core::IUnknown);
impl IAppxPackageReader {
    pub unsafe fn GetBlockMap(&self) -> ::windows::core::Result<IAppxBlockMapReader> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).GetBlockMap)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetFootprintFile(&self, r#type: APPX_FOOTPRINT_FILE_TYPE) -> ::windows::core::Result<IAppxFile> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).GetFootprintFile)(::windows::core::Vtable::as_raw(self), r#type, result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetPayloadFile<P0>(&self, filename: P0) -> ::windows::core::Result<IAppxFile>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).GetPayloadFile)(::windows::core::Vtable::as_raw(self), filename.into().abi(), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetPayloadFiles(&self) -> ::windows::core::Result<IAppxFilesEnumerator> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).GetPayloadFiles)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetManifest(&self) -> ::windows::core::Result<IAppxManifestReader> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).GetManifest)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
}
::windows::core::interface_hierarchy!(IAppxPackageReader, ::windows::core::IUnknown);
impl ::core::clone::Clone for IAppxPackageReader {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IAppxPackageReader {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IAppxPackageReader {}
impl ::core::fmt::Debug for IAppxPackageReader {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IAppxPackageReader").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Vtable for IAppxPackageReader {
    type Vtable = IAppxPackageReader_Vtbl;
}
unsafe impl ::windows::core::Interface for IAppxPackageReader {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xb5c49650_99bc_481c_9a34_3d53a4106708);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppxPackageReader_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub GetBlockMap: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, blockmapreader: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub GetFootprintFile: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, r#type: APPX_FOOTPRINT_FILE_TYPE, file: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub GetPayloadFile: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, filename: ::windows::core::PCWSTR, file: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub GetPayloadFiles: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, filesenumerator: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub GetManifest: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, manifestreader: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Storage_Packaging_Appx\"`*"]
#[repr(transparent)]
pub struct IAppxPackageWriter(::windows::core::IUnknown);
impl IAppxPackageWriter {
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn AddPayloadFile<P0, P1, P2>(&self, filename: P0, contenttype: P1, compressionoption: APPX_COMPRESSION_OPTION, inputstream: P2) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
        P1: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
        P2: ::std::convert::Into<::windows::core::InParam<super::super::super::System::Com::IStream>>,
    {
        (::windows::core::Vtable::vtable(self).AddPayloadFile)(::windows::core::Vtable::as_raw(self), filename.into().abi(), contenttype.into().abi(), compressionoption, inputstream.into().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Close<P0>(&self, manifest: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<super::super::super::System::Com::IStream>>,
    {
        (::windows::core::Vtable::vtable(self).Close)(::windows::core::Vtable::as_raw(self), manifest.into().abi()).ok()
    }
}
::windows::core::interface_hierarchy!(IAppxPackageWriter, ::windows::core::IUnknown);
impl ::core::clone::Clone for IAppxPackageWriter {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IAppxPackageWriter {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IAppxPackageWriter {}
impl ::core::fmt::Debug for IAppxPackageWriter {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IAppxPackageWriter").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Vtable for IAppxPackageWriter {
    type Vtable = IAppxPackageWriter_Vtbl;
}
unsafe impl ::windows::core::Interface for IAppxPackageWriter {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x9099e33b_246f_41e4_881a_008eb613f858);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppxPackageWriter_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    #[cfg(feature = "Win32_System_Com")]
    pub AddPayloadFile: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, filename: ::windows::core::PCWSTR, contenttype: ::windows::core::PCWSTR, compressionoption: APPX_COMPRESSION_OPTION, inputstream: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    AddPayloadFile: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub Close: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, manifest: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Close: usize,
}
#[doc = "*Required features: `\"Win32_Storage_Packaging_Appx\"`*"]
#[repr(transparent)]
pub struct IAppxPackageWriter2(::windows::core::IUnknown);
impl IAppxPackageWriter2 {
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Close<P0, P1>(&self, manifest: P0, contentgroupmap: P1) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<super::super::super::System::Com::IStream>>,
        P1: ::std::convert::Into<::windows::core::InParam<super::super::super::System::Com::IStream>>,
    {
        (::windows::core::Vtable::vtable(self).Close)(::windows::core::Vtable::as_raw(self), manifest.into().abi(), contentgroupmap.into().abi()).ok()
    }
}
::windows::core::interface_hierarchy!(IAppxPackageWriter2, ::windows::core::IUnknown);
impl ::core::clone::Clone for IAppxPackageWriter2 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IAppxPackageWriter2 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IAppxPackageWriter2 {}
impl ::core::fmt::Debug for IAppxPackageWriter2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IAppxPackageWriter2").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Vtable for IAppxPackageWriter2 {
    type Vtable = IAppxPackageWriter2_Vtbl;
}
unsafe impl ::windows::core::Interface for IAppxPackageWriter2 {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x2cf5c4fd_e54c_4ea5_ba4e_f8c4b105a8c8);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppxPackageWriter2_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    #[cfg(feature = "Win32_System_Com")]
    pub Close: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, manifest: *mut ::core::ffi::c_void, contentgroupmap: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Close: usize,
}
#[doc = "*Required features: `\"Win32_Storage_Packaging_Appx\"`*"]
#[repr(transparent)]
pub struct IAppxPackageWriter3(::windows::core::IUnknown);
impl IAppxPackageWriter3 {
    #[doc = "*Required features: `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn AddPayloadFiles(&self, payloadfiles: &[APPX_PACKAGE_WRITER_PAYLOAD_STREAM], memorylimit: u64) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).AddPayloadFiles)(::windows::core::Vtable::as_raw(self), payloadfiles.len() as _, ::core::mem::transmute(payloadfiles.as_ptr()), memorylimit).ok()
    }
}
::windows::core::interface_hierarchy!(IAppxPackageWriter3, ::windows::core::IUnknown);
impl ::core::clone::Clone for IAppxPackageWriter3 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IAppxPackageWriter3 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IAppxPackageWriter3 {}
impl ::core::fmt::Debug for IAppxPackageWriter3 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IAppxPackageWriter3").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Vtable for IAppxPackageWriter3 {
    type Vtable = IAppxPackageWriter3_Vtbl;
}
unsafe impl ::windows::core::Interface for IAppxPackageWriter3 {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xa83aacd3_41c0_4501_b8a3_74164f50b2fd);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppxPackageWriter3_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    #[cfg(feature = "Win32_System_Com")]
    pub AddPayloadFiles: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, filecount: u32, payloadfiles: *const APPX_PACKAGE_WRITER_PAYLOAD_STREAM, memorylimit: u64) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    AddPayloadFiles: usize,
}
#[doc = "*Required features: `\"Win32_Storage_Packaging_Appx\"`*"]
#[repr(transparent)]
pub struct IAppxPackagingDiagnosticEventSink(::windows::core::IUnknown);
impl IAppxPackagingDiagnosticEventSink {
    pub unsafe fn ReportContextChange<P0, P1, P2>(&self, changetype: APPX_PACKAGING_CONTEXT_CHANGE_TYPE, contextid: i32, contextname: P0, contextmessage: P1, detailsmessage: P2) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCSTR>>,
        P1: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
        P2: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
    {
        (::windows::core::Vtable::vtable(self).ReportContextChange)(::windows::core::Vtable::as_raw(self), changetype, contextid, contextname.into().abi(), contextmessage.into().abi(), detailsmessage.into().abi()).ok()
    }
    pub unsafe fn ReportError<P0>(&self, errormessage: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
    {
        (::windows::core::Vtable::vtable(self).ReportError)(::windows::core::Vtable::as_raw(self), errormessage.into().abi()).ok()
    }
}
::windows::core::interface_hierarchy!(IAppxPackagingDiagnosticEventSink, ::windows::core::IUnknown);
impl ::core::clone::Clone for IAppxPackagingDiagnosticEventSink {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IAppxPackagingDiagnosticEventSink {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IAppxPackagingDiagnosticEventSink {}
impl ::core::fmt::Debug for IAppxPackagingDiagnosticEventSink {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IAppxPackagingDiagnosticEventSink").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Vtable for IAppxPackagingDiagnosticEventSink {
    type Vtable = IAppxPackagingDiagnosticEventSink_Vtbl;
}
unsafe impl ::windows::core::Interface for IAppxPackagingDiagnosticEventSink {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x17239d47_6adb_45d2_80f6_f9cbc3bf059d);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppxPackagingDiagnosticEventSink_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub ReportContextChange: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, changetype: APPX_PACKAGING_CONTEXT_CHANGE_TYPE, contextid: i32, contextname: ::windows::core::PCSTR, contextmessage: ::windows::core::PCWSTR, detailsmessage: ::windows::core::PCWSTR) -> ::windows::core::HRESULT,
    pub ReportError: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, errormessage: ::windows::core::PCWSTR) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Storage_Packaging_Appx\"`*"]
#[repr(transparent)]
pub struct IAppxPackagingDiagnosticEventSinkManager(::windows::core::IUnknown);
impl IAppxPackagingDiagnosticEventSinkManager {
    pub unsafe fn SetSinkForProcess<P0>(&self, sink: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<IAppxPackagingDiagnosticEventSink>>,
    {
        (::windows::core::Vtable::vtable(self).SetSinkForProcess)(::windows::core::Vtable::as_raw(self), sink.into().abi()).ok()
    }
}
::windows::core::interface_hierarchy!(IAppxPackagingDiagnosticEventSinkManager, ::windows::core::IUnknown);
impl ::core::clone::Clone for IAppxPackagingDiagnosticEventSinkManager {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IAppxPackagingDiagnosticEventSinkManager {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IAppxPackagingDiagnosticEventSinkManager {}
impl ::core::fmt::Debug for IAppxPackagingDiagnosticEventSinkManager {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IAppxPackagingDiagnosticEventSinkManager").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Vtable for IAppxPackagingDiagnosticEventSinkManager {
    type Vtable = IAppxPackagingDiagnosticEventSinkManager_Vtbl;
}
unsafe impl ::windows::core::Interface for IAppxPackagingDiagnosticEventSinkManager {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x369648fa_a7eb_4909_a15d_6954a078f18a);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppxPackagingDiagnosticEventSinkManager_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub SetSinkForProcess: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, sink: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Storage_Packaging_Appx\"`*"]
#[repr(transparent)]
pub struct IAppxSourceContentGroupMapReader(::windows::core::IUnknown);
impl IAppxSourceContentGroupMapReader {
    pub unsafe fn GetRequiredGroup(&self) -> ::windows::core::Result<IAppxContentGroup> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).GetRequiredGroup)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetAutomaticGroups(&self) -> ::windows::core::Result<IAppxContentGroupsEnumerator> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).GetAutomaticGroups)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
}
::windows::core::interface_hierarchy!(IAppxSourceContentGroupMapReader, ::windows::core::IUnknown);
impl ::core::clone::Clone for IAppxSourceContentGroupMapReader {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IAppxSourceContentGroupMapReader {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IAppxSourceContentGroupMapReader {}
impl ::core::fmt::Debug for IAppxSourceContentGroupMapReader {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IAppxSourceContentGroupMapReader").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Vtable for IAppxSourceContentGroupMapReader {
    type Vtable = IAppxSourceContentGroupMapReader_Vtbl;
}
unsafe impl ::windows::core::Interface for IAppxSourceContentGroupMapReader {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xf329791d_540b_4a9f_bc75_3282b7d73193);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppxSourceContentGroupMapReader_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub GetRequiredGroup: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, requiredgroup: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub GetAutomaticGroups: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, automaticgroupsenumerator: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_Storage_Packaging_Appx\"`*"]
pub const AppxBundleFactory: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x378e0446_5384_43b7_8877_e7dbdd883446);
#[doc = "*Required features: `\"Win32_Storage_Packaging_Appx\"`*"]
pub const AppxEncryptionFactory: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xdc664fdd_d868_46ee_8780_8d196cb739f7);
#[doc = "*Required features: `\"Win32_Storage_Packaging_Appx\"`*"]
pub const AppxFactory: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x5842a140_ff9f_4166_8f5c_62f5b7b0c781);
#[doc = "*Required features: `\"Win32_Storage_Packaging_Appx\"`*"]
pub const AppxPackageEditor: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xf004f2ca_aebc_4b0d_bf58_e516d5bcc0ab);
#[doc = "*Required features: `\"Win32_Storage_Packaging_Appx\"`*"]
pub const AppxPackagingDiagnosticEventSinkManager: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x50ca0a46_1588_4161_8ed2_ef9e469ced5d);
#[doc = "*Required features: `\"Win32_Storage_Packaging_Appx\"`*"]
pub const PACKAGE_DEPENDENCY_RANK_DEFAULT: u32 = 0u32;
#[doc = "*Required features: `\"Win32_Storage_Packaging_Appx\"`*"]
pub const PACKAGE_FILTER_ALL_LOADED: u32 = 0u32;
#[doc = "*Required features: `\"Win32_Storage_Packaging_Appx\"`*"]
pub const PACKAGE_FILTER_BUNDLE: u32 = 128u32;
#[doc = "*Required features: `\"Win32_Storage_Packaging_Appx\"`*"]
pub const PACKAGE_FILTER_DIRECT: u32 = 32u32;
#[doc = "*Required features: `\"Win32_Storage_Packaging_Appx\"`*"]
pub const PACKAGE_FILTER_DYNAMIC: u32 = 1048576u32;
#[doc = "*Required features: `\"Win32_Storage_Packaging_Appx\"`*"]
pub const PACKAGE_FILTER_HEAD: u32 = 16u32;
#[doc = "*Required features: `\"Win32_Storage_Packaging_Appx\"`*"]
pub const PACKAGE_FILTER_HOSTRUNTIME: u32 = 2097152u32;
#[doc = "*Required features: `\"Win32_Storage_Packaging_Appx\"`*"]
pub const PACKAGE_FILTER_IS_IN_RELATED_SET: u32 = 262144u32;
#[doc = "*Required features: `\"Win32_Storage_Packaging_Appx\"`*"]
pub const PACKAGE_FILTER_OPTIONAL: u32 = 131072u32;
#[doc = "*Required features: `\"Win32_Storage_Packaging_Appx\"`*"]
pub const PACKAGE_FILTER_RESOURCE: u32 = 64u32;
#[doc = "*Required features: `\"Win32_Storage_Packaging_Appx\"`*"]
pub const PACKAGE_FILTER_STATIC: u32 = 524288u32;
#[doc = "*Required features: `\"Win32_Storage_Packaging_Appx\"`*"]
pub const PACKAGE_INFORMATION_BASIC: u32 = 0u32;
#[doc = "*Required features: `\"Win32_Storage_Packaging_Appx\"`*"]
pub const PACKAGE_INFORMATION_FULL: u32 = 256u32;
#[doc = "*Required features: `\"Win32_Storage_Packaging_Appx\"`*"]
pub const PACKAGE_PROPERTY_BUNDLE: u32 = 4u32;
#[doc = "*Required features: `\"Win32_Storage_Packaging_Appx\"`*"]
pub const PACKAGE_PROPERTY_DEVELOPMENT_MODE: u32 = 65536u32;
#[doc = "*Required features: `\"Win32_Storage_Packaging_Appx\"`*"]
pub const PACKAGE_PROPERTY_DYNAMIC: u32 = 1048576u32;
#[doc = "*Required features: `\"Win32_Storage_Packaging_Appx\"`*"]
pub const PACKAGE_PROPERTY_FRAMEWORK: u32 = 1u32;
#[doc = "*Required features: `\"Win32_Storage_Packaging_Appx\"`*"]
pub const PACKAGE_PROPERTY_HOSTRUNTIME: u32 = 2097152u32;
#[doc = "*Required features: `\"Win32_Storage_Packaging_Appx\"`*"]
pub const PACKAGE_PROPERTY_IS_IN_RELATED_SET: u32 = 262144u32;
#[doc = "*Required features: `\"Win32_Storage_Packaging_Appx\"`*"]
pub const PACKAGE_PROPERTY_OPTIONAL: u32 = 8u32;
#[doc = "*Required features: `\"Win32_Storage_Packaging_Appx\"`*"]
pub const PACKAGE_PROPERTY_RESOURCE: u32 = 2u32;
#[doc = "*Required features: `\"Win32_Storage_Packaging_Appx\"`*"]
pub const PACKAGE_PROPERTY_STATIC: u32 = 524288u32;
#[doc = "*Required features: `\"Win32_Storage_Packaging_Appx\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct APPX_BUNDLE_FOOTPRINT_FILE_TYPE(pub i32);
#[doc = "*Required features: `\"Win32_Storage_Packaging_Appx\"`*"]
pub const APPX_BUNDLE_FOOTPRINT_FILE_TYPE_FIRST: APPX_BUNDLE_FOOTPRINT_FILE_TYPE = APPX_BUNDLE_FOOTPRINT_FILE_TYPE(0i32);
#[doc = "*Required features: `\"Win32_Storage_Packaging_Appx\"`*"]
pub const APPX_BUNDLE_FOOTPRINT_FILE_TYPE_MANIFEST: APPX_BUNDLE_FOOTPRINT_FILE_TYPE = APPX_BUNDLE_FOOTPRINT_FILE_TYPE(0i32);
#[doc = "*Required features: `\"Win32_Storage_Packaging_Appx\"`*"]
pub const APPX_BUNDLE_FOOTPRINT_FILE_TYPE_BLOCKMAP: APPX_BUNDLE_FOOTPRINT_FILE_TYPE = APPX_BUNDLE_FOOTPRINT_FILE_TYPE(1i32);
#[doc = "*Required features: `\"Win32_Storage_Packaging_Appx\"`*"]
pub const APPX_BUNDLE_FOOTPRINT_FILE_TYPE_SIGNATURE: APPX_BUNDLE_FOOTPRINT_FILE_TYPE = APPX_BUNDLE_FOOTPRINT_FILE_TYPE(2i32);
#[doc = "*Required features: `\"Win32_Storage_Packaging_Appx\"`*"]
pub const APPX_BUNDLE_FOOTPRINT_FILE_TYPE_LAST: APPX_BUNDLE_FOOTPRINT_FILE_TYPE = APPX_BUNDLE_FOOTPRINT_FILE_TYPE(2i32);
impl ::core::marker::Copy for APPX_BUNDLE_FOOTPRINT_FILE_TYPE {}
impl ::core::clone::Clone for APPX_BUNDLE_FOOTPRINT_FILE_TYPE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for APPX_BUNDLE_FOOTPRINT_FILE_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for APPX_BUNDLE_FOOTPRINT_FILE_TYPE {
    type Abi = Self;
}
impl ::core::fmt::Debug for APPX_BUNDLE_FOOTPRINT_FILE_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("APPX_BUNDLE_FOOTPRINT_FILE_TYPE").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Storage_Packaging_Appx\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct APPX_BUNDLE_PAYLOAD_PACKAGE_TYPE(pub i32);
#[doc = "*Required features: `\"Win32_Storage_Packaging_Appx\"`*"]
pub const APPX_BUNDLE_PAYLOAD_PACKAGE_TYPE_APPLICATION: APPX_BUNDLE_PAYLOAD_PACKAGE_TYPE = APPX_BUNDLE_PAYLOAD_PACKAGE_TYPE(0i32);
#[doc = "*Required features: `\"Win32_Storage_Packaging_Appx\"`*"]
pub const APPX_BUNDLE_PAYLOAD_PACKAGE_TYPE_RESOURCE: APPX_BUNDLE_PAYLOAD_PACKAGE_TYPE = APPX_BUNDLE_PAYLOAD_PACKAGE_TYPE(1i32);
impl ::core::marker::Copy for APPX_BUNDLE_PAYLOAD_PACKAGE_TYPE {}
impl ::core::clone::Clone for APPX_BUNDLE_PAYLOAD_PACKAGE_TYPE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for APPX_BUNDLE_PAYLOAD_PACKAGE_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for APPX_BUNDLE_PAYLOAD_PACKAGE_TYPE {
    type Abi = Self;
}
impl ::core::fmt::Debug for APPX_BUNDLE_PAYLOAD_PACKAGE_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("APPX_BUNDLE_PAYLOAD_PACKAGE_TYPE").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Storage_Packaging_Appx\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct APPX_CAPABILITIES(pub u32);
#[doc = "*Required features: `\"Win32_Storage_Packaging_Appx\"`*"]
pub const APPX_CAPABILITY_INTERNET_CLIENT: APPX_CAPABILITIES = APPX_CAPABILITIES(1u32);
#[doc = "*Required features: `\"Win32_Storage_Packaging_Appx\"`*"]
pub const APPX_CAPABILITY_INTERNET_CLIENT_SERVER: APPX_CAPABILITIES = APPX_CAPABILITIES(2u32);
#[doc = "*Required features: `\"Win32_Storage_Packaging_Appx\"`*"]
pub const APPX_CAPABILITY_PRIVATE_NETWORK_CLIENT_SERVER: APPX_CAPABILITIES = APPX_CAPABILITIES(4u32);
#[doc = "*Required features: `\"Win32_Storage_Packaging_Appx\"`*"]
pub const APPX_CAPABILITY_DOCUMENTS_LIBRARY: APPX_CAPABILITIES = APPX_CAPABILITIES(8u32);
#[doc = "*Required features: `\"Win32_Storage_Packaging_Appx\"`*"]
pub const APPX_CAPABILITY_PICTURES_LIBRARY: APPX_CAPABILITIES = APPX_CAPABILITIES(16u32);
#[doc = "*Required features: `\"Win32_Storage_Packaging_Appx\"`*"]
pub const APPX_CAPABILITY_VIDEOS_LIBRARY: APPX_CAPABILITIES = APPX_CAPABILITIES(32u32);
#[doc = "*Required features: `\"Win32_Storage_Packaging_Appx\"`*"]
pub const APPX_CAPABILITY_MUSIC_LIBRARY: APPX_CAPABILITIES = APPX_CAPABILITIES(64u32);
#[doc = "*Required features: `\"Win32_Storage_Packaging_Appx\"`*"]
pub const APPX_CAPABILITY_ENTERPRISE_AUTHENTICATION: APPX_CAPABILITIES = APPX_CAPABILITIES(128u32);
#[doc = "*Required features: `\"Win32_Storage_Packaging_Appx\"`*"]
pub const APPX_CAPABILITY_SHARED_USER_CERTIFICATES: APPX_CAPABILITIES = APPX_CAPABILITIES(256u32);
#[doc = "*Required features: `\"Win32_Storage_Packaging_Appx\"`*"]
pub const APPX_CAPABILITY_REMOVABLE_STORAGE: APPX_CAPABILITIES = APPX_CAPABILITIES(512u32);
#[doc = "*Required features: `\"Win32_Storage_Packaging_Appx\"`*"]
pub const APPX_CAPABILITY_APPOINTMENTS: APPX_CAPABILITIES = APPX_CAPABILITIES(1024u32);
#[doc = "*Required features: `\"Win32_Storage_Packaging_Appx\"`*"]
pub const APPX_CAPABILITY_CONTACTS: APPX_CAPABILITIES = APPX_CAPABILITIES(2048u32);
impl ::core::marker::Copy for APPX_CAPABILITIES {}
impl ::core::clone::Clone for APPX_CAPABILITIES {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for APPX_CAPABILITIES {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for APPX_CAPABILITIES {
    type Abi = Self;
}
impl ::core::fmt::Debug for APPX_CAPABILITIES {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("APPX_CAPABILITIES").field(&self.0).finish()
    }
}
impl ::core::ops::BitOr for APPX_CAPABILITIES {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for APPX_CAPABILITIES {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for APPX_CAPABILITIES {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for APPX_CAPABILITIES {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for APPX_CAPABILITIES {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[doc = "*Required features: `\"Win32_Storage_Packaging_Appx\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct APPX_CAPABILITY_CLASS_TYPE(pub i32);
#[doc = "*Required features: `\"Win32_Storage_Packaging_Appx\"`*"]
pub const APPX_CAPABILITY_CLASS_DEFAULT: APPX_CAPABILITY_CLASS_TYPE = APPX_CAPABILITY_CLASS_TYPE(0i32);
#[doc = "*Required features: `\"Win32_Storage_Packaging_Appx\"`*"]
pub const APPX_CAPABILITY_CLASS_GENERAL: APPX_CAPABILITY_CLASS_TYPE = APPX_CAPABILITY_CLASS_TYPE(1i32);
#[doc = "*Required features: `\"Win32_Storage_Packaging_Appx\"`*"]
pub const APPX_CAPABILITY_CLASS_RESTRICTED: APPX_CAPABILITY_CLASS_TYPE = APPX_CAPABILITY_CLASS_TYPE(2i32);
#[doc = "*Required features: `\"Win32_Storage_Packaging_Appx\"`*"]
pub const APPX_CAPABILITY_CLASS_WINDOWS: APPX_CAPABILITY_CLASS_TYPE = APPX_CAPABILITY_CLASS_TYPE(4i32);
#[doc = "*Required features: `\"Win32_Storage_Packaging_Appx\"`*"]
pub const APPX_CAPABILITY_CLASS_ALL: APPX_CAPABILITY_CLASS_TYPE = APPX_CAPABILITY_CLASS_TYPE(7i32);
#[doc = "*Required features: `\"Win32_Storage_Packaging_Appx\"`*"]
pub const APPX_CAPABILITY_CLASS_CUSTOM: APPX_CAPABILITY_CLASS_TYPE = APPX_CAPABILITY_CLASS_TYPE(8i32);
impl ::core::marker::Copy for APPX_CAPABILITY_CLASS_TYPE {}
impl ::core::clone::Clone for APPX_CAPABILITY_CLASS_TYPE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for APPX_CAPABILITY_CLASS_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for APPX_CAPABILITY_CLASS_TYPE {
    type Abi = Self;
}
impl ::core::fmt::Debug for APPX_CAPABILITY_CLASS_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("APPX_CAPABILITY_CLASS_TYPE").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Storage_Packaging_Appx\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct APPX_COMPRESSION_OPTION(pub i32);
#[doc = "*Required features: `\"Win32_Storage_Packaging_Appx\"`*"]
pub const APPX_COMPRESSION_OPTION_NONE: APPX_COMPRESSION_OPTION = APPX_COMPRESSION_OPTION(0i32);
#[doc = "*Required features: `\"Win32_Storage_Packaging_Appx\"`*"]
pub const APPX_COMPRESSION_OPTION_NORMAL: APPX_COMPRESSION_OPTION = APPX_COMPRESSION_OPTION(1i32);
#[doc = "*Required features: `\"Win32_Storage_Packaging_Appx\"`*"]
pub const APPX_COMPRESSION_OPTION_MAXIMUM: APPX_COMPRESSION_OPTION = APPX_COMPRESSION_OPTION(2i32);
#[doc = "*Required features: `\"Win32_Storage_Packaging_Appx\"`*"]
pub const APPX_COMPRESSION_OPTION_FAST: APPX_COMPRESSION_OPTION = APPX_COMPRESSION_OPTION(3i32);
#[doc = "*Required features: `\"Win32_Storage_Packaging_Appx\"`*"]
pub const APPX_COMPRESSION_OPTION_SUPERFAST: APPX_COMPRESSION_OPTION = APPX_COMPRESSION_OPTION(4i32);
impl ::core::marker::Copy for APPX_COMPRESSION_OPTION {}
impl ::core::clone::Clone for APPX_COMPRESSION_OPTION {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for APPX_COMPRESSION_OPTION {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for APPX_COMPRESSION_OPTION {
    type Abi = Self;
}
impl ::core::fmt::Debug for APPX_COMPRESSION_OPTION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("APPX_COMPRESSION_OPTION").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Storage_Packaging_Appx\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct APPX_ENCRYPTED_PACKAGE_OPTIONS(pub u32);
#[doc = "*Required features: `\"Win32_Storage_Packaging_Appx\"`*"]
pub const APPX_ENCRYPTED_PACKAGE_OPTION_NONE: APPX_ENCRYPTED_PACKAGE_OPTIONS = APPX_ENCRYPTED_PACKAGE_OPTIONS(0u32);
#[doc = "*Required features: `\"Win32_Storage_Packaging_Appx\"`*"]
pub const APPX_ENCRYPTED_PACKAGE_OPTION_DIFFUSION: APPX_ENCRYPTED_PACKAGE_OPTIONS = APPX_ENCRYPTED_PACKAGE_OPTIONS(1u32);
#[doc = "*Required features: `\"Win32_Storage_Packaging_Appx\"`*"]
pub const APPX_ENCRYPTED_PACKAGE_OPTION_PAGE_HASHING: APPX_ENCRYPTED_PACKAGE_OPTIONS = APPX_ENCRYPTED_PACKAGE_OPTIONS(2u32);
impl ::core::marker::Copy for APPX_ENCRYPTED_PACKAGE_OPTIONS {}
impl ::core::clone::Clone for APPX_ENCRYPTED_PACKAGE_OPTIONS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for APPX_ENCRYPTED_PACKAGE_OPTIONS {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for APPX_ENCRYPTED_PACKAGE_OPTIONS {
    type Abi = Self;
}
impl ::core::fmt::Debug for APPX_ENCRYPTED_PACKAGE_OPTIONS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("APPX_ENCRYPTED_PACKAGE_OPTIONS").field(&self.0).finish()
    }
}
impl ::core::ops::BitOr for APPX_ENCRYPTED_PACKAGE_OPTIONS {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for APPX_ENCRYPTED_PACKAGE_OPTIONS {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for APPX_ENCRYPTED_PACKAGE_OPTIONS {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for APPX_ENCRYPTED_PACKAGE_OPTIONS {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for APPX_ENCRYPTED_PACKAGE_OPTIONS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[doc = "*Required features: `\"Win32_Storage_Packaging_Appx\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct APPX_FOOTPRINT_FILE_TYPE(pub i32);
#[doc = "*Required features: `\"Win32_Storage_Packaging_Appx\"`*"]
pub const APPX_FOOTPRINT_FILE_TYPE_MANIFEST: APPX_FOOTPRINT_FILE_TYPE = APPX_FOOTPRINT_FILE_TYPE(0i32);
#[doc = "*Required features: `\"Win32_Storage_Packaging_Appx\"`*"]
pub const APPX_FOOTPRINT_FILE_TYPE_BLOCKMAP: APPX_FOOTPRINT_FILE_TYPE = APPX_FOOTPRINT_FILE_TYPE(1i32);
#[doc = "*Required features: `\"Win32_Storage_Packaging_Appx\"`*"]
pub const APPX_FOOTPRINT_FILE_TYPE_SIGNATURE: APPX_FOOTPRINT_FILE_TYPE = APPX_FOOTPRINT_FILE_TYPE(2i32);
#[doc = "*Required features: `\"Win32_Storage_Packaging_Appx\"`*"]
pub const APPX_FOOTPRINT_FILE_TYPE_CODEINTEGRITY: APPX_FOOTPRINT_FILE_TYPE = APPX_FOOTPRINT_FILE_TYPE(3i32);
#[doc = "*Required features: `\"Win32_Storage_Packaging_Appx\"`*"]
pub const APPX_FOOTPRINT_FILE_TYPE_CONTENTGROUPMAP: APPX_FOOTPRINT_FILE_TYPE = APPX_FOOTPRINT_FILE_TYPE(4i32);
impl ::core::marker::Copy for APPX_FOOTPRINT_FILE_TYPE {}
impl ::core::clone::Clone for APPX_FOOTPRINT_FILE_TYPE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for APPX_FOOTPRINT_FILE_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for APPX_FOOTPRINT_FILE_TYPE {
    type Abi = Self;
}
impl ::core::fmt::Debug for APPX_FOOTPRINT_FILE_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("APPX_FOOTPRINT_FILE_TYPE").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Storage_Packaging_Appx\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct APPX_PACKAGE_ARCHITECTURE(pub i32);
#[doc = "*Required features: `\"Win32_Storage_Packaging_Appx\"`*"]
pub const APPX_PACKAGE_ARCHITECTURE_X86: APPX_PACKAGE_ARCHITECTURE = APPX_PACKAGE_ARCHITECTURE(0i32);
#[doc = "*Required features: `\"Win32_Storage_Packaging_Appx\"`*"]
pub const APPX_PACKAGE_ARCHITECTURE_ARM: APPX_PACKAGE_ARCHITECTURE = APPX_PACKAGE_ARCHITECTURE(5i32);
#[doc = "*Required features: `\"Win32_Storage_Packaging_Appx\"`*"]
pub const APPX_PACKAGE_ARCHITECTURE_X64: APPX_PACKAGE_ARCHITECTURE = APPX_PACKAGE_ARCHITECTURE(9i32);
#[doc = "*Required features: `\"Win32_Storage_Packaging_Appx\"`*"]
pub const APPX_PACKAGE_ARCHITECTURE_NEUTRAL: APPX_PACKAGE_ARCHITECTURE = APPX_PACKAGE_ARCHITECTURE(11i32);
#[doc = "*Required features: `\"Win32_Storage_Packaging_Appx\"`*"]
pub const APPX_PACKAGE_ARCHITECTURE_ARM64: APPX_PACKAGE_ARCHITECTURE = APPX_PACKAGE_ARCHITECTURE(12i32);
impl ::core::marker::Copy for APPX_PACKAGE_ARCHITECTURE {}
impl ::core::clone::Clone for APPX_PACKAGE_ARCHITECTURE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for APPX_PACKAGE_ARCHITECTURE {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for APPX_PACKAGE_ARCHITECTURE {
    type Abi = Self;
}
impl ::core::fmt::Debug for APPX_PACKAGE_ARCHITECTURE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("APPX_PACKAGE_ARCHITECTURE").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Storage_Packaging_Appx\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct APPX_PACKAGE_ARCHITECTURE2(pub i32);
#[doc = "*Required features: `\"Win32_Storage_Packaging_Appx\"`*"]
pub const APPX_PACKAGE_ARCHITECTURE2_X86: APPX_PACKAGE_ARCHITECTURE2 = APPX_PACKAGE_ARCHITECTURE2(0i32);
#[doc = "*Required features: `\"Win32_Storage_Packaging_Appx\"`*"]
pub const APPX_PACKAGE_ARCHITECTURE2_ARM: APPX_PACKAGE_ARCHITECTURE2 = APPX_PACKAGE_ARCHITECTURE2(5i32);
#[doc = "*Required features: `\"Win32_Storage_Packaging_Appx\"`*"]
pub const APPX_PACKAGE_ARCHITECTURE2_X64: APPX_PACKAGE_ARCHITECTURE2 = APPX_PACKAGE_ARCHITECTURE2(9i32);
#[doc = "*Required features: `\"Win32_Storage_Packaging_Appx\"`*"]
pub const APPX_PACKAGE_ARCHITECTURE2_NEUTRAL: APPX_PACKAGE_ARCHITECTURE2 = APPX_PACKAGE_ARCHITECTURE2(11i32);
#[doc = "*Required features: `\"Win32_Storage_Packaging_Appx\"`*"]
pub const APPX_PACKAGE_ARCHITECTURE2_ARM64: APPX_PACKAGE_ARCHITECTURE2 = APPX_PACKAGE_ARCHITECTURE2(12i32);
#[doc = "*Required features: `\"Win32_Storage_Packaging_Appx\"`*"]
pub const APPX_PACKAGE_ARCHITECTURE2_X86_ON_ARM64: APPX_PACKAGE_ARCHITECTURE2 = APPX_PACKAGE_ARCHITECTURE2(14i32);
#[doc = "*Required features: `\"Win32_Storage_Packaging_Appx\"`*"]
pub const APPX_PACKAGE_ARCHITECTURE2_UNKNOWN: APPX_PACKAGE_ARCHITECTURE2 = APPX_PACKAGE_ARCHITECTURE2(65535i32);
impl ::core::marker::Copy for APPX_PACKAGE_ARCHITECTURE2 {}
impl ::core::clone::Clone for APPX_PACKAGE_ARCHITECTURE2 {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for APPX_PACKAGE_ARCHITECTURE2 {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for APPX_PACKAGE_ARCHITECTURE2 {
    type Abi = Self;
}
impl ::core::fmt::Debug for APPX_PACKAGE_ARCHITECTURE2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("APPX_PACKAGE_ARCHITECTURE2").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Storage_Packaging_Appx\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct APPX_PACKAGE_EDITOR_UPDATE_PACKAGE_MANIFEST_OPTIONS(pub u32);
#[doc = "*Required features: `\"Win32_Storage_Packaging_Appx\"`*"]
pub const APPX_PACKAGE_EDITOR_UPDATE_PACKAGE_MANIFEST_OPTION_NONE: APPX_PACKAGE_EDITOR_UPDATE_PACKAGE_MANIFEST_OPTIONS = APPX_PACKAGE_EDITOR_UPDATE_PACKAGE_MANIFEST_OPTIONS(0u32);
#[doc = "*Required features: `\"Win32_Storage_Packaging_Appx\"`*"]
pub const APPX_PACKAGE_EDITOR_UPDATE_PACKAGE_MANIFEST_OPTION_SKIP_VALIDATION: APPX_PACKAGE_EDITOR_UPDATE_PACKAGE_MANIFEST_OPTIONS = APPX_PACKAGE_EDITOR_UPDATE_PACKAGE_MANIFEST_OPTIONS(1u32);
#[doc = "*Required features: `\"Win32_Storage_Packaging_Appx\"`*"]
pub const APPX_PACKAGE_EDITOR_UPDATE_PACKAGE_MANIFEST_OPTION_LOCALIZED: APPX_PACKAGE_EDITOR_UPDATE_PACKAGE_MANIFEST_OPTIONS = APPX_PACKAGE_EDITOR_UPDATE_PACKAGE_MANIFEST_OPTIONS(2u32);
impl ::core::marker::Copy for APPX_PACKAGE_EDITOR_UPDATE_PACKAGE_MANIFEST_OPTIONS {}
impl ::core::clone::Clone for APPX_PACKAGE_EDITOR_UPDATE_PACKAGE_MANIFEST_OPTIONS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for APPX_PACKAGE_EDITOR_UPDATE_PACKAGE_MANIFEST_OPTIONS {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for APPX_PACKAGE_EDITOR_UPDATE_PACKAGE_MANIFEST_OPTIONS {
    type Abi = Self;
}
impl ::core::fmt::Debug for APPX_PACKAGE_EDITOR_UPDATE_PACKAGE_MANIFEST_OPTIONS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("APPX_PACKAGE_EDITOR_UPDATE_PACKAGE_MANIFEST_OPTIONS").field(&self.0).finish()
    }
}
impl ::core::ops::BitOr for APPX_PACKAGE_EDITOR_UPDATE_PACKAGE_MANIFEST_OPTIONS {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for APPX_PACKAGE_EDITOR_UPDATE_PACKAGE_MANIFEST_OPTIONS {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for APPX_PACKAGE_EDITOR_UPDATE_PACKAGE_MANIFEST_OPTIONS {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for APPX_PACKAGE_EDITOR_UPDATE_PACKAGE_MANIFEST_OPTIONS {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for APPX_PACKAGE_EDITOR_UPDATE_PACKAGE_MANIFEST_OPTIONS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[doc = "*Required features: `\"Win32_Storage_Packaging_Appx\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct APPX_PACKAGE_EDITOR_UPDATE_PACKAGE_OPTION(pub i32);
#[doc = "*Required features: `\"Win32_Storage_Packaging_Appx\"`*"]
pub const APPX_PACKAGE_EDITOR_UPDATE_PACKAGE_OPTION_APPEND_DELTA: APPX_PACKAGE_EDITOR_UPDATE_PACKAGE_OPTION = APPX_PACKAGE_EDITOR_UPDATE_PACKAGE_OPTION(0i32);
impl ::core::marker::Copy for APPX_PACKAGE_EDITOR_UPDATE_PACKAGE_OPTION {}
impl ::core::clone::Clone for APPX_PACKAGE_EDITOR_UPDATE_PACKAGE_OPTION {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for APPX_PACKAGE_EDITOR_UPDATE_PACKAGE_OPTION {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for APPX_PACKAGE_EDITOR_UPDATE_PACKAGE_OPTION {
    type Abi = Self;
}
impl ::core::fmt::Debug for APPX_PACKAGE_EDITOR_UPDATE_PACKAGE_OPTION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("APPX_PACKAGE_EDITOR_UPDATE_PACKAGE_OPTION").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Storage_Packaging_Appx\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct APPX_PACKAGING_CONTEXT_CHANGE_TYPE(pub i32);
#[doc = "*Required features: `\"Win32_Storage_Packaging_Appx\"`*"]
pub const APPX_PACKAGING_CONTEXT_CHANGE_TYPE_START: APPX_PACKAGING_CONTEXT_CHANGE_TYPE = APPX_PACKAGING_CONTEXT_CHANGE_TYPE(0i32);
#[doc = "*Required features: `\"Win32_Storage_Packaging_Appx\"`*"]
pub const APPX_PACKAGING_CONTEXT_CHANGE_TYPE_CHANGE: APPX_PACKAGING_CONTEXT_CHANGE_TYPE = APPX_PACKAGING_CONTEXT_CHANGE_TYPE(1i32);
#[doc = "*Required features: `\"Win32_Storage_Packaging_Appx\"`*"]
pub const APPX_PACKAGING_CONTEXT_CHANGE_TYPE_DETAILS: APPX_PACKAGING_CONTEXT_CHANGE_TYPE = APPX_PACKAGING_CONTEXT_CHANGE_TYPE(2i32);
#[doc = "*Required features: `\"Win32_Storage_Packaging_Appx\"`*"]
pub const APPX_PACKAGING_CONTEXT_CHANGE_TYPE_END: APPX_PACKAGING_CONTEXT_CHANGE_TYPE = APPX_PACKAGING_CONTEXT_CHANGE_TYPE(3i32);
impl ::core::marker::Copy for APPX_PACKAGING_CONTEXT_CHANGE_TYPE {}
impl ::core::clone::Clone for APPX_PACKAGING_CONTEXT_CHANGE_TYPE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for APPX_PACKAGING_CONTEXT_CHANGE_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for APPX_PACKAGING_CONTEXT_CHANGE_TYPE {
    type Abi = Self;
}
impl ::core::fmt::Debug for APPX_PACKAGING_CONTEXT_CHANGE_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("APPX_PACKAGING_CONTEXT_CHANGE_TYPE").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Storage_Packaging_Appx\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct AddPackageDependencyOptions(pub i32);
#[doc = "*Required features: `\"Win32_Storage_Packaging_Appx\"`*"]
pub const AddPackageDependencyOptions_None: AddPackageDependencyOptions = AddPackageDependencyOptions(0i32);
#[doc = "*Required features: `\"Win32_Storage_Packaging_Appx\"`*"]
pub const AddPackageDependencyOptions_PrependIfRankCollision: AddPackageDependencyOptions = AddPackageDependencyOptions(1i32);
impl ::core::marker::Copy for AddPackageDependencyOptions {}
impl ::core::clone::Clone for AddPackageDependencyOptions {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for AddPackageDependencyOptions {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for AddPackageDependencyOptions {
    type Abi = Self;
}
impl ::core::fmt::Debug for AddPackageDependencyOptions {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("AddPackageDependencyOptions").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Storage_Packaging_Appx\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct AppPolicyClrCompat(pub i32);
#[doc = "*Required features: `\"Win32_Storage_Packaging_Appx\"`*"]
pub const AppPolicyClrCompat_Other: AppPolicyClrCompat = AppPolicyClrCompat(0i32);
#[doc = "*Required features: `\"Win32_Storage_Packaging_Appx\"`*"]
pub const AppPolicyClrCompat_ClassicDesktop: AppPolicyClrCompat = AppPolicyClrCompat(1i32);
#[doc = "*Required features: `\"Win32_Storage_Packaging_Appx\"`*"]
pub const AppPolicyClrCompat_Universal: AppPolicyClrCompat = AppPolicyClrCompat(2i32);
#[doc = "*Required features: `\"Win32_Storage_Packaging_Appx\"`*"]
pub const AppPolicyClrCompat_PackagedDesktop: AppPolicyClrCompat = AppPolicyClrCompat(3i32);
impl ::core::marker::Copy for AppPolicyClrCompat {}
impl ::core::clone::Clone for AppPolicyClrCompat {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for AppPolicyClrCompat {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for AppPolicyClrCompat {
    type Abi = Self;
}
impl ::core::fmt::Debug for AppPolicyClrCompat {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("AppPolicyClrCompat").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Storage_Packaging_Appx\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct AppPolicyCreateFileAccess(pub i32);
#[doc = "*Required features: `\"Win32_Storage_Packaging_Appx\"`*"]
pub const AppPolicyCreateFileAccess_Full: AppPolicyCreateFileAccess = AppPolicyCreateFileAccess(0i32);
#[doc = "*Required features: `\"Win32_Storage_Packaging_Appx\"`*"]
pub const AppPolicyCreateFileAccess_Limited: AppPolicyCreateFileAccess = AppPolicyCreateFileAccess(1i32);
impl ::core::marker::Copy for AppPolicyCreateFileAccess {}
impl ::core::clone::Clone for AppPolicyCreateFileAccess {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for AppPolicyCreateFileAccess {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for AppPolicyCreateFileAccess {
    type Abi = Self;
}
impl ::core::fmt::Debug for AppPolicyCreateFileAccess {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("AppPolicyCreateFileAccess").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Storage_Packaging_Appx\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct AppPolicyLifecycleManagement(pub i32);
#[doc = "*Required features: `\"Win32_Storage_Packaging_Appx\"`*"]
pub const AppPolicyLifecycleManagement_Unmanaged: AppPolicyLifecycleManagement = AppPolicyLifecycleManagement(0i32);
#[doc = "*Required features: `\"Win32_Storage_Packaging_Appx\"`*"]
pub const AppPolicyLifecycleManagement_Managed: AppPolicyLifecycleManagement = AppPolicyLifecycleManagement(1i32);
impl ::core::marker::Copy for AppPolicyLifecycleManagement {}
impl ::core::clone::Clone for AppPolicyLifecycleManagement {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for AppPolicyLifecycleManagement {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for AppPolicyLifecycleManagement {
    type Abi = Self;
}
impl ::core::fmt::Debug for AppPolicyLifecycleManagement {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("AppPolicyLifecycleManagement").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Storage_Packaging_Appx\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct AppPolicyMediaFoundationCodecLoading(pub i32);
#[doc = "*Required features: `\"Win32_Storage_Packaging_Appx\"`*"]
pub const AppPolicyMediaFoundationCodecLoading_All: AppPolicyMediaFoundationCodecLoading = AppPolicyMediaFoundationCodecLoading(0i32);
#[doc = "*Required features: `\"Win32_Storage_Packaging_Appx\"`*"]
pub const AppPolicyMediaFoundationCodecLoading_InboxOnly: AppPolicyMediaFoundationCodecLoading = AppPolicyMediaFoundationCodecLoading(1i32);
impl ::core::marker::Copy for AppPolicyMediaFoundationCodecLoading {}
impl ::core::clone::Clone for AppPolicyMediaFoundationCodecLoading {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for AppPolicyMediaFoundationCodecLoading {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for AppPolicyMediaFoundationCodecLoading {
    type Abi = Self;
}
impl ::core::fmt::Debug for AppPolicyMediaFoundationCodecLoading {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("AppPolicyMediaFoundationCodecLoading").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Storage_Packaging_Appx\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct AppPolicyProcessTerminationMethod(pub i32);
#[doc = "*Required features: `\"Win32_Storage_Packaging_Appx\"`*"]
pub const AppPolicyProcessTerminationMethod_ExitProcess: AppPolicyProcessTerminationMethod = AppPolicyProcessTerminationMethod(0i32);
#[doc = "*Required features: `\"Win32_Storage_Packaging_Appx\"`*"]
pub const AppPolicyProcessTerminationMethod_TerminateProcess: AppPolicyProcessTerminationMethod = AppPolicyProcessTerminationMethod(1i32);
impl ::core::marker::Copy for AppPolicyProcessTerminationMethod {}
impl ::core::clone::Clone for AppPolicyProcessTerminationMethod {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for AppPolicyProcessTerminationMethod {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for AppPolicyProcessTerminationMethod {
    type Abi = Self;
}
impl ::core::fmt::Debug for AppPolicyProcessTerminationMethod {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("AppPolicyProcessTerminationMethod").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Storage_Packaging_Appx\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct AppPolicyShowDeveloperDiagnostic(pub i32);
#[doc = "*Required features: `\"Win32_Storage_Packaging_Appx\"`*"]
pub const AppPolicyShowDeveloperDiagnostic_None: AppPolicyShowDeveloperDiagnostic = AppPolicyShowDeveloperDiagnostic(0i32);
#[doc = "*Required features: `\"Win32_Storage_Packaging_Appx\"`*"]
pub const AppPolicyShowDeveloperDiagnostic_ShowUI: AppPolicyShowDeveloperDiagnostic = AppPolicyShowDeveloperDiagnostic(1i32);
impl ::core::marker::Copy for AppPolicyShowDeveloperDiagnostic {}
impl ::core::clone::Clone for AppPolicyShowDeveloperDiagnostic {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for AppPolicyShowDeveloperDiagnostic {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for AppPolicyShowDeveloperDiagnostic {
    type Abi = Self;
}
impl ::core::fmt::Debug for AppPolicyShowDeveloperDiagnostic {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("AppPolicyShowDeveloperDiagnostic").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Storage_Packaging_Appx\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct AppPolicyThreadInitializationType(pub i32);
#[doc = "*Required features: `\"Win32_Storage_Packaging_Appx\"`*"]
pub const AppPolicyThreadInitializationType_None: AppPolicyThreadInitializationType = AppPolicyThreadInitializationType(0i32);
#[doc = "*Required features: `\"Win32_Storage_Packaging_Appx\"`*"]
pub const AppPolicyThreadInitializationType_InitializeWinRT: AppPolicyThreadInitializationType = AppPolicyThreadInitializationType(1i32);
impl ::core::marker::Copy for AppPolicyThreadInitializationType {}
impl ::core::clone::Clone for AppPolicyThreadInitializationType {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for AppPolicyThreadInitializationType {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for AppPolicyThreadInitializationType {
    type Abi = Self;
}
impl ::core::fmt::Debug for AppPolicyThreadInitializationType {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("AppPolicyThreadInitializationType").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Storage_Packaging_Appx\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct AppPolicyWindowingModel(pub i32);
#[doc = "*Required features: `\"Win32_Storage_Packaging_Appx\"`*"]
pub const AppPolicyWindowingModel_None: AppPolicyWindowingModel = AppPolicyWindowingModel(0i32);
#[doc = "*Required features: `\"Win32_Storage_Packaging_Appx\"`*"]
pub const AppPolicyWindowingModel_Universal: AppPolicyWindowingModel = AppPolicyWindowingModel(1i32);
#[doc = "*Required features: `\"Win32_Storage_Packaging_Appx\"`*"]
pub const AppPolicyWindowingModel_ClassicDesktop: AppPolicyWindowingModel = AppPolicyWindowingModel(2i32);
#[doc = "*Required features: `\"Win32_Storage_Packaging_Appx\"`*"]
pub const AppPolicyWindowingModel_ClassicPhone: AppPolicyWindowingModel = AppPolicyWindowingModel(3i32);
impl ::core::marker::Copy for AppPolicyWindowingModel {}
impl ::core::clone::Clone for AppPolicyWindowingModel {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for AppPolicyWindowingModel {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for AppPolicyWindowingModel {
    type Abi = Self;
}
impl ::core::fmt::Debug for AppPolicyWindowingModel {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("AppPolicyWindowingModel").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Storage_Packaging_Appx\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct CreatePackageDependencyOptions(pub i32);
#[doc = "*Required features: `\"Win32_Storage_Packaging_Appx\"`*"]
pub const CreatePackageDependencyOptions_None: CreatePackageDependencyOptions = CreatePackageDependencyOptions(0i32);
#[doc = "*Required features: `\"Win32_Storage_Packaging_Appx\"`*"]
pub const CreatePackageDependencyOptions_DoNotVerifyDependencyResolution: CreatePackageDependencyOptions = CreatePackageDependencyOptions(1i32);
#[doc = "*Required features: `\"Win32_Storage_Packaging_Appx\"`*"]
pub const CreatePackageDependencyOptions_ScopeIsSystem: CreatePackageDependencyOptions = CreatePackageDependencyOptions(2i32);
impl ::core::marker::Copy for CreatePackageDependencyOptions {}
impl ::core::clone::Clone for CreatePackageDependencyOptions {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for CreatePackageDependencyOptions {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for CreatePackageDependencyOptions {
    type Abi = Self;
}
impl ::core::fmt::Debug for CreatePackageDependencyOptions {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CreatePackageDependencyOptions").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Storage_Packaging_Appx\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct DX_FEATURE_LEVEL(pub i32);
#[doc = "*Required features: `\"Win32_Storage_Packaging_Appx\"`*"]
pub const DX_FEATURE_LEVEL_UNSPECIFIED: DX_FEATURE_LEVEL = DX_FEATURE_LEVEL(0i32);
#[doc = "*Required features: `\"Win32_Storage_Packaging_Appx\"`*"]
pub const DX_FEATURE_LEVEL_9: DX_FEATURE_LEVEL = DX_FEATURE_LEVEL(1i32);
#[doc = "*Required features: `\"Win32_Storage_Packaging_Appx\"`*"]
pub const DX_FEATURE_LEVEL_10: DX_FEATURE_LEVEL = DX_FEATURE_LEVEL(2i32);
#[doc = "*Required features: `\"Win32_Storage_Packaging_Appx\"`*"]
pub const DX_FEATURE_LEVEL_11: DX_FEATURE_LEVEL = DX_FEATURE_LEVEL(3i32);
impl ::core::marker::Copy for DX_FEATURE_LEVEL {}
impl ::core::clone::Clone for DX_FEATURE_LEVEL {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for DX_FEATURE_LEVEL {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for DX_FEATURE_LEVEL {
    type Abi = Self;
}
impl ::core::fmt::Debug for DX_FEATURE_LEVEL {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DX_FEATURE_LEVEL").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Storage_Packaging_Appx\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct PackageDependencyLifetimeKind(pub i32);
#[doc = "*Required features: `\"Win32_Storage_Packaging_Appx\"`*"]
pub const PackageDependencyLifetimeKind_Process: PackageDependencyLifetimeKind = PackageDependencyLifetimeKind(0i32);
#[doc = "*Required features: `\"Win32_Storage_Packaging_Appx\"`*"]
pub const PackageDependencyLifetimeKind_FilePath: PackageDependencyLifetimeKind = PackageDependencyLifetimeKind(1i32);
#[doc = "*Required features: `\"Win32_Storage_Packaging_Appx\"`*"]
pub const PackageDependencyLifetimeKind_RegistryKey: PackageDependencyLifetimeKind = PackageDependencyLifetimeKind(2i32);
impl ::core::marker::Copy for PackageDependencyLifetimeKind {}
impl ::core::clone::Clone for PackageDependencyLifetimeKind {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for PackageDependencyLifetimeKind {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for PackageDependencyLifetimeKind {
    type Abi = Self;
}
impl ::core::fmt::Debug for PackageDependencyLifetimeKind {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PackageDependencyLifetimeKind").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Storage_Packaging_Appx\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct PackageDependencyProcessorArchitectures(pub i32);
#[doc = "*Required features: `\"Win32_Storage_Packaging_Appx\"`*"]
pub const PackageDependencyProcessorArchitectures_None: PackageDependencyProcessorArchitectures = PackageDependencyProcessorArchitectures(0i32);
#[doc = "*Required features: `\"Win32_Storage_Packaging_Appx\"`*"]
pub const PackageDependencyProcessorArchitectures_Neutral: PackageDependencyProcessorArchitectures = PackageDependencyProcessorArchitectures(1i32);
#[doc = "*Required features: `\"Win32_Storage_Packaging_Appx\"`*"]
pub const PackageDependencyProcessorArchitectures_X86: PackageDependencyProcessorArchitectures = PackageDependencyProcessorArchitectures(2i32);
#[doc = "*Required features: `\"Win32_Storage_Packaging_Appx\"`*"]
pub const PackageDependencyProcessorArchitectures_X64: PackageDependencyProcessorArchitectures = PackageDependencyProcessorArchitectures(4i32);
#[doc = "*Required features: `\"Win32_Storage_Packaging_Appx\"`*"]
pub const PackageDependencyProcessorArchitectures_Arm: PackageDependencyProcessorArchitectures = PackageDependencyProcessorArchitectures(8i32);
#[doc = "*Required features: `\"Win32_Storage_Packaging_Appx\"`*"]
pub const PackageDependencyProcessorArchitectures_Arm64: PackageDependencyProcessorArchitectures = PackageDependencyProcessorArchitectures(16i32);
#[doc = "*Required features: `\"Win32_Storage_Packaging_Appx\"`*"]
pub const PackageDependencyProcessorArchitectures_X86A64: PackageDependencyProcessorArchitectures = PackageDependencyProcessorArchitectures(32i32);
impl ::core::marker::Copy for PackageDependencyProcessorArchitectures {}
impl ::core::clone::Clone for PackageDependencyProcessorArchitectures {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for PackageDependencyProcessorArchitectures {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for PackageDependencyProcessorArchitectures {
    type Abi = Self;
}
impl ::core::fmt::Debug for PackageDependencyProcessorArchitectures {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PackageDependencyProcessorArchitectures").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Storage_Packaging_Appx\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct PackageOrigin(pub i32);
#[doc = "*Required features: `\"Win32_Storage_Packaging_Appx\"`*"]
pub const PackageOrigin_Unknown: PackageOrigin = PackageOrigin(0i32);
#[doc = "*Required features: `\"Win32_Storage_Packaging_Appx\"`*"]
pub const PackageOrigin_Unsigned: PackageOrigin = PackageOrigin(1i32);
#[doc = "*Required features: `\"Win32_Storage_Packaging_Appx\"`*"]
pub const PackageOrigin_Inbox: PackageOrigin = PackageOrigin(2i32);
#[doc = "*Required features: `\"Win32_Storage_Packaging_Appx\"`*"]
pub const PackageOrigin_Store: PackageOrigin = PackageOrigin(3i32);
#[doc = "*Required features: `\"Win32_Storage_Packaging_Appx\"`*"]
pub const PackageOrigin_DeveloperUnsigned: PackageOrigin = PackageOrigin(4i32);
#[doc = "*Required features: `\"Win32_Storage_Packaging_Appx\"`*"]
pub const PackageOrigin_DeveloperSigned: PackageOrigin = PackageOrigin(5i32);
#[doc = "*Required features: `\"Win32_Storage_Packaging_Appx\"`*"]
pub const PackageOrigin_LineOfBusiness: PackageOrigin = PackageOrigin(6i32);
impl ::core::marker::Copy for PackageOrigin {}
impl ::core::clone::Clone for PackageOrigin {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for PackageOrigin {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for PackageOrigin {
    type Abi = Self;
}
impl ::core::fmt::Debug for PackageOrigin {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PackageOrigin").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_Storage_Packaging_Appx\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct PackagePathType(pub i32);
#[doc = "*Required features: `\"Win32_Storage_Packaging_Appx\"`*"]
pub const PackagePathType_Install: PackagePathType = PackagePathType(0i32);
#[doc = "*Required features: `\"Win32_Storage_Packaging_Appx\"`*"]
pub const PackagePathType_Mutable: PackagePathType = PackagePathType(1i32);
#[doc = "*Required features: `\"Win32_Storage_Packaging_Appx\"`*"]
pub const PackagePathType_Effective: PackagePathType = PackagePathType(2i32);
#[doc = "*Required features: `\"Win32_Storage_Packaging_Appx\"`*"]
pub const PackagePathType_MachineExternal: PackagePathType = PackagePathType(3i32);
#[doc = "*Required features: `\"Win32_Storage_Packaging_Appx\"`*"]
pub const PackagePathType_UserExternal: PackagePathType = PackagePathType(4i32);
#[doc = "*Required features: `\"Win32_Storage_Packaging_Appx\"`*"]
pub const PackagePathType_EffectiveExternal: PackagePathType = PackagePathType(5i32);
impl ::core::marker::Copy for PackagePathType {}
impl ::core::clone::Clone for PackagePathType {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for PackagePathType {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for PackagePathType {
    type Abi = Self;
}
impl ::core::fmt::Debug for PackagePathType {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PackagePathType").field(&self.0).finish()
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Storage_Packaging_Appx\"`*"]
pub struct APPX_ENCRYPTED_EXEMPTIONS {
    pub count: u32,
    pub plainTextFiles: *mut ::windows::core::PWSTR,
}
impl ::core::marker::Copy for APPX_ENCRYPTED_EXEMPTIONS {}
impl ::core::clone::Clone for APPX_ENCRYPTED_EXEMPTIONS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for APPX_ENCRYPTED_EXEMPTIONS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("APPX_ENCRYPTED_EXEMPTIONS").field("count", &self.count).field("plainTextFiles", &self.plainTextFiles).finish()
    }
}
unsafe impl ::windows::core::Abi for APPX_ENCRYPTED_EXEMPTIONS {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for APPX_ENCRYPTED_EXEMPTIONS {
    fn eq(&self, other: &Self) -> bool {
        self.count == other.count && self.plainTextFiles == other.plainTextFiles
    }
}
impl ::core::cmp::Eq for APPX_ENCRYPTED_EXEMPTIONS {}
impl ::core::default::Default for APPX_ENCRYPTED_EXEMPTIONS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Storage_Packaging_Appx\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
pub struct APPX_ENCRYPTED_PACKAGE_SETTINGS {
    pub keyLength: u32,
    pub encryptionAlgorithm: ::windows::core::PCWSTR,
    pub useDiffusion: super::super::super::Foundation::BOOL,
    pub blockMapHashAlgorithm: ::windows::core::ManuallyDrop<super::super::super::System::Com::IUri>,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl ::core::clone::Clone for APPX_ENCRYPTED_PACKAGE_SETTINGS {
    fn clone(&self) -> Self {
        unsafe { ::core::mem::transmute_copy(self) }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl ::core::fmt::Debug for APPX_ENCRYPTED_PACKAGE_SETTINGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("APPX_ENCRYPTED_PACKAGE_SETTINGS").field("keyLength", &self.keyLength).field("encryptionAlgorithm", &self.encryptionAlgorithm).field("useDiffusion", &self.useDiffusion).field("blockMapHashAlgorithm", &self.blockMapHashAlgorithm).finish()
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
unsafe impl ::windows::core::Abi for APPX_ENCRYPTED_PACKAGE_SETTINGS {
    type Abi = Self;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl ::core::cmp::PartialEq for APPX_ENCRYPTED_PACKAGE_SETTINGS {
    fn eq(&self, other: &Self) -> bool {
        self.keyLength == other.keyLength && self.encryptionAlgorithm == other.encryptionAlgorithm && self.useDiffusion == other.useDiffusion && self.blockMapHashAlgorithm == other.blockMapHashAlgorithm
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl ::core::cmp::Eq for APPX_ENCRYPTED_PACKAGE_SETTINGS {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl ::core::default::Default for APPX_ENCRYPTED_PACKAGE_SETTINGS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Storage_Packaging_Appx\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
pub struct APPX_ENCRYPTED_PACKAGE_SETTINGS2 {
    pub keyLength: u32,
    pub encryptionAlgorithm: ::windows::core::PCWSTR,
    pub blockMapHashAlgorithm: ::windows::core::ManuallyDrop<super::super::super::System::Com::IUri>,
    pub options: u32,
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for APPX_ENCRYPTED_PACKAGE_SETTINGS2 {
    fn clone(&self) -> Self {
        unsafe { ::core::mem::transmute_copy(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for APPX_ENCRYPTED_PACKAGE_SETTINGS2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("APPX_ENCRYPTED_PACKAGE_SETTINGS2").field("keyLength", &self.keyLength).field("encryptionAlgorithm", &self.encryptionAlgorithm).field("blockMapHashAlgorithm", &self.blockMapHashAlgorithm).field("options", &self.options).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Abi for APPX_ENCRYPTED_PACKAGE_SETTINGS2 {
    type Abi = Self;
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for APPX_ENCRYPTED_PACKAGE_SETTINGS2 {
    fn eq(&self, other: &Self) -> bool {
        self.keyLength == other.keyLength && self.encryptionAlgorithm == other.encryptionAlgorithm && self.blockMapHashAlgorithm == other.blockMapHashAlgorithm && self.options == other.options
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for APPX_ENCRYPTED_PACKAGE_SETTINGS2 {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::default::Default for APPX_ENCRYPTED_PACKAGE_SETTINGS2 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Storage_Packaging_Appx\"`*"]
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
impl ::core::fmt::Debug for APPX_KEY_INFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("APPX_KEY_INFO").field("keyLength", &self.keyLength).field("keyIdLength", &self.keyIdLength).field("key", &self.key).field("keyId", &self.keyId).finish()
    }
}
unsafe impl ::windows::core::Abi for APPX_KEY_INFO {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for APPX_KEY_INFO {
    fn eq(&self, other: &Self) -> bool {
        self.keyLength == other.keyLength && self.keyIdLength == other.keyIdLength && self.key == other.key && self.keyId == other.keyId
    }
}
impl ::core::cmp::Eq for APPX_KEY_INFO {}
impl ::core::default::Default for APPX_KEY_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Storage_Packaging_Appx\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
pub struct APPX_PACKAGE_SETTINGS {
    pub forceZip32: super::super::super::Foundation::BOOL,
    pub hashMethod: ::windows::core::ManuallyDrop<super::super::super::System::Com::IUri>,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl ::core::clone::Clone for APPX_PACKAGE_SETTINGS {
    fn clone(&self) -> Self {
        unsafe { ::core::mem::transmute_copy(self) }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl ::core::fmt::Debug for APPX_PACKAGE_SETTINGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("APPX_PACKAGE_SETTINGS").field("forceZip32", &self.forceZip32).field("hashMethod", &self.hashMethod).finish()
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
unsafe impl ::windows::core::Abi for APPX_PACKAGE_SETTINGS {
    type Abi = Self;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl ::core::cmp::PartialEq for APPX_PACKAGE_SETTINGS {
    fn eq(&self, other: &Self) -> bool {
        self.forceZip32 == other.forceZip32 && self.hashMethod == other.hashMethod
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl ::core::cmp::Eq for APPX_PACKAGE_SETTINGS {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl ::core::default::Default for APPX_PACKAGE_SETTINGS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Storage_Packaging_Appx\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
pub struct APPX_PACKAGE_WRITER_PAYLOAD_STREAM {
    pub inputStream: ::windows::core::ManuallyDrop<super::super::super::System::Com::IStream>,
    pub fileName: ::windows::core::PCWSTR,
    pub contentType: ::windows::core::PCWSTR,
    pub compressionOption: APPX_COMPRESSION_OPTION,
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for APPX_PACKAGE_WRITER_PAYLOAD_STREAM {
    fn clone(&self) -> Self {
        unsafe { ::core::mem::transmute_copy(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for APPX_PACKAGE_WRITER_PAYLOAD_STREAM {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("APPX_PACKAGE_WRITER_PAYLOAD_STREAM").field("inputStream", &self.inputStream).field("fileName", &self.fileName).field("contentType", &self.contentType).field("compressionOption", &self.compressionOption).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Abi for APPX_PACKAGE_WRITER_PAYLOAD_STREAM {
    type Abi = Self;
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for APPX_PACKAGE_WRITER_PAYLOAD_STREAM {
    fn eq(&self, other: &Self) -> bool {
        self.inputStream == other.inputStream && self.fileName == other.fileName && self.contentType == other.contentType && self.compressionOption == other.compressionOption
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for APPX_PACKAGE_WRITER_PAYLOAD_STREAM {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::default::Default for APPX_PACKAGE_WRITER_PAYLOAD_STREAM {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Storage_Packaging_Appx\"`*"]
pub struct PACKAGEDEPENDENCY_CONTEXT__ {
    pub unused: i32,
}
impl ::core::marker::Copy for PACKAGEDEPENDENCY_CONTEXT__ {}
impl ::core::clone::Clone for PACKAGEDEPENDENCY_CONTEXT__ {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for PACKAGEDEPENDENCY_CONTEXT__ {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("PACKAGEDEPENDENCY_CONTEXT__").field("unused", &self.unused).finish()
    }
}
unsafe impl ::windows::core::Abi for PACKAGEDEPENDENCY_CONTEXT__ {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for PACKAGEDEPENDENCY_CONTEXT__ {
    fn eq(&self, other: &Self) -> bool {
        self.unused == other.unused
    }
}
impl ::core::cmp::Eq for PACKAGEDEPENDENCY_CONTEXT__ {}
impl ::core::default::Default for PACKAGEDEPENDENCY_CONTEXT__ {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C, packed(4))]
#[doc = "*Required features: `\"Win32_Storage_Packaging_Appx\"`*"]
pub struct PACKAGE_ID {
    pub reserved: u32,
    pub processorArchitecture: u32,
    pub version: PACKAGE_VERSION,
    pub name: ::windows::core::PWSTR,
    pub publisher: ::windows::core::PWSTR,
    pub resourceId: ::windows::core::PWSTR,
    pub publisherId: ::windows::core::PWSTR,
}
impl ::core::marker::Copy for PACKAGE_ID {}
impl ::core::clone::Clone for PACKAGE_ID {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for PACKAGE_ID {
    type Abi = Self;
}
impl ::core::default::Default for PACKAGE_ID {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C, packed(4))]
#[doc = "*Required features: `\"Win32_Storage_Packaging_Appx\"`*"]
pub struct PACKAGE_INFO {
    pub reserved: u32,
    pub flags: u32,
    pub path: ::windows::core::PWSTR,
    pub packageFullName: ::windows::core::PWSTR,
    pub packageFamilyName: ::windows::core::PWSTR,
    pub packageId: PACKAGE_ID,
}
impl ::core::marker::Copy for PACKAGE_INFO {}
impl ::core::clone::Clone for PACKAGE_INFO {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for PACKAGE_INFO {
    type Abi = Self;
}
impl ::core::default::Default for PACKAGE_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Storage_Packaging_Appx\"`*"]
pub struct PACKAGE_VERSION {
    pub Anonymous: PACKAGE_VERSION_0,
}
impl ::core::marker::Copy for PACKAGE_VERSION {}
impl ::core::clone::Clone for PACKAGE_VERSION {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for PACKAGE_VERSION {
    type Abi = Self;
}
impl ::core::default::Default for PACKAGE_VERSION {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C, packed(4))]
#[doc = "*Required features: `\"Win32_Storage_Packaging_Appx\"`*"]
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
unsafe impl ::windows::core::Abi for PACKAGE_VERSION_0 {
    type Abi = Self;
}
impl ::core::default::Default for PACKAGE_VERSION_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Storage_Packaging_Appx\"`*"]
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
impl ::core::fmt::Debug for PACKAGE_VERSION_0_0 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("PACKAGE_VERSION_0_0").field("Revision", &self.Revision).field("Build", &self.Build).field("Minor", &self.Minor).field("Major", &self.Major).finish()
    }
}
unsafe impl ::windows::core::Abi for PACKAGE_VERSION_0_0 {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for PACKAGE_VERSION_0_0 {
    fn eq(&self, other: &Self) -> bool {
        self.Revision == other.Revision && self.Build == other.Build && self.Minor == other.Minor && self.Major == other.Major
    }
}
impl ::core::cmp::Eq for PACKAGE_VERSION_0_0 {}
impl ::core::default::Default for PACKAGE_VERSION_0_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Storage_Packaging_Appx\"`*"]
pub struct PACKAGE_VIRTUALIZATION_CONTEXT_HANDLE__ {
    pub unused: i32,
}
impl ::core::marker::Copy for PACKAGE_VIRTUALIZATION_CONTEXT_HANDLE__ {}
impl ::core::clone::Clone for PACKAGE_VIRTUALIZATION_CONTEXT_HANDLE__ {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for PACKAGE_VIRTUALIZATION_CONTEXT_HANDLE__ {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("PACKAGE_VIRTUALIZATION_CONTEXT_HANDLE__").field("unused", &self.unused).finish()
    }
}
unsafe impl ::windows::core::Abi for PACKAGE_VIRTUALIZATION_CONTEXT_HANDLE__ {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for PACKAGE_VIRTUALIZATION_CONTEXT_HANDLE__ {
    fn eq(&self, other: &Self) -> bool {
        self.unused == other.unused
    }
}
impl ::core::cmp::Eq for PACKAGE_VIRTUALIZATION_CONTEXT_HANDLE__ {}
impl ::core::default::Default for PACKAGE_VIRTUALIZATION_CONTEXT_HANDLE__ {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_Storage_Packaging_Appx\"`*"]
pub struct _PACKAGE_INFO_REFERENCE {
    pub reserved: *mut ::core::ffi::c_void,
}
impl ::core::marker::Copy for _PACKAGE_INFO_REFERENCE {}
impl ::core::clone::Clone for _PACKAGE_INFO_REFERENCE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for _PACKAGE_INFO_REFERENCE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("_PACKAGE_INFO_REFERENCE").field("reserved", &self.reserved).finish()
    }
}
unsafe impl ::windows::core::Abi for _PACKAGE_INFO_REFERENCE {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for _PACKAGE_INFO_REFERENCE {
    fn eq(&self, other: &Self) -> bool {
        self.reserved == other.reserved
    }
}
impl ::core::cmp::Eq for _PACKAGE_INFO_REFERENCE {}
impl ::core::default::Default for _PACKAGE_INFO_REFERENCE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "implement")]
::core::include!("impl.rs");

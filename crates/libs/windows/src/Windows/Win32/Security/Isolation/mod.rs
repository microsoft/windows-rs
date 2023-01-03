#[doc = "*Required features: `\"Win32_Security_Isolation\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn CreateAppContainerProfile<P0, P1, P2>(pszappcontainername: P0, pszdisplayname: P1, pszdescription: P2, pcapabilities: ::core::option::Option<&[super::SID_AND_ATTRIBUTES]>) -> ::windows::core::Result<super::super::Foundation::PSID>
where
    P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
    P1: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
    P2: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
{
    ::windows::core::link ! ( "userenv.dll""system" fn CreateAppContainerProfile ( pszappcontainername : :: windows::core::PCWSTR , pszdisplayname : :: windows::core::PCWSTR , pszdescription : :: windows::core::PCWSTR , pcapabilities : *const super:: SID_AND_ATTRIBUTES , dwcapabilitycount : u32 , ppsidappcontainersid : *mut super::super::Foundation:: PSID ) -> :: windows::core::HRESULT );
    let mut result__ = ::core::mem::MaybeUninit::zeroed();
    CreateAppContainerProfile(pszappcontainername.into().abi(), pszdisplayname.into().abi(), pszdescription.into().abi(), ::core::mem::transmute(pcapabilities.as_deref().map_or(::core::ptr::null(), |slice| slice.as_ptr())), pcapabilities.as_deref().map_or(0, |slice| slice.len() as _), result__.as_mut_ptr()).from_abi(result__)
}
#[doc = "*Required features: `\"Win32_Security_Isolation\"`*"]
#[inline]
pub unsafe fn DeleteAppContainerProfile<P0>(pszappcontainername: P0) -> ::windows::core::Result<()>
where
    P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
{
    ::windows::core::link ! ( "userenv.dll""system" fn DeleteAppContainerProfile ( pszappcontainername : :: windows::core::PCWSTR ) -> :: windows::core::HRESULT );
    DeleteAppContainerProfile(pszappcontainername.into().abi()).ok()
}
#[doc = "*Required features: `\"Win32_Security_Isolation\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn DeriveAppContainerSidFromAppContainerName<P0>(pszappcontainername: P0) -> ::windows::core::Result<super::super::Foundation::PSID>
where
    P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
{
    ::windows::core::link ! ( "userenv.dll""system" fn DeriveAppContainerSidFromAppContainerName ( pszappcontainername : :: windows::core::PCWSTR , ppsidappcontainersid : *mut super::super::Foundation:: PSID ) -> :: windows::core::HRESULT );
    let mut result__ = ::core::mem::MaybeUninit::zeroed();
    DeriveAppContainerSidFromAppContainerName(pszappcontainername.into().abi(), result__.as_mut_ptr()).from_abi(result__)
}
#[doc = "*Required features: `\"Win32_Security_Isolation\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn DeriveRestrictedAppContainerSidFromAppContainerSidAndRestrictedName<P0, P1>(psidappcontainersid: P0, pszrestrictedappcontainername: P1) -> ::windows::core::Result<super::super::Foundation::PSID>
where
    P0: ::std::convert::Into<super::super::Foundation::PSID>,
    P1: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
{
    ::windows::core::link ! ( "userenv.dll""system" fn DeriveRestrictedAppContainerSidFromAppContainerSidAndRestrictedName ( psidappcontainersid : super::super::Foundation:: PSID , pszrestrictedappcontainername : :: windows::core::PCWSTR , ppsidrestrictedappcontainersid : *mut super::super::Foundation:: PSID ) -> :: windows::core::HRESULT );
    let mut result__ = ::core::mem::MaybeUninit::zeroed();
    DeriveRestrictedAppContainerSidFromAppContainerSidAndRestrictedName(psidappcontainersid.into(), pszrestrictedappcontainername.into().abi(), result__.as_mut_ptr()).from_abi(result__)
}
#[doc = "*Required features: `\"Win32_Security_Isolation\"`*"]
#[inline]
pub unsafe fn GetAppContainerFolderPath<P0>(pszappcontainersid: P0) -> ::windows::core::Result<::windows::core::PWSTR>
where
    P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
{
    ::windows::core::link ! ( "userenv.dll""system" fn GetAppContainerFolderPath ( pszappcontainersid : :: windows::core::PCWSTR , ppszpath : *mut :: windows::core::PWSTR ) -> :: windows::core::HRESULT );
    let mut result__ = ::core::mem::MaybeUninit::zeroed();
    GetAppContainerFolderPath(pszappcontainersid.into().abi(), result__.as_mut_ptr()).from_abi(result__)
}
#[doc = "*Required features: `\"Win32_Security_Isolation\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn GetAppContainerNamedObjectPath<P0, P1>(token: P0, appcontainersid: P1, objectpath: ::core::option::Option<&mut [u16]>, returnlength: *mut u32) -> super::super::Foundation::BOOL
where
    P0: ::std::convert::Into<super::super::Foundation::HANDLE>,
    P1: ::std::convert::Into<super::super::Foundation::PSID>,
{
    ::windows::core::link ! ( "kernel32.dll""system" fn GetAppContainerNamedObjectPath ( token : super::super::Foundation:: HANDLE , appcontainersid : super::super::Foundation:: PSID , objectpathlength : u32 , objectpath : :: windows::core::PWSTR , returnlength : *mut u32 ) -> super::super::Foundation:: BOOL );
    GetAppContainerNamedObjectPath(token.into(), appcontainersid.into(), objectpath.as_deref().map_or(0, |slice| slice.len() as _), ::core::mem::transmute(objectpath.as_deref().map_or(::core::ptr::null(), |slice| slice.as_ptr())), returnlength)
}
#[doc = "*Required features: `\"Win32_Security_Isolation\"`, `\"Win32_System_Registry\"`*"]
#[cfg(feature = "Win32_System_Registry")]
#[inline]
pub unsafe fn GetAppContainerRegistryLocation(desiredaccess: u32) -> ::windows::core::Result<super::super::System::Registry::HKEY> {
    ::windows::core::link ! ( "userenv.dll""system" fn GetAppContainerRegistryLocation ( desiredaccess : u32 , phappcontainerkey : *mut super::super::System::Registry:: HKEY ) -> :: windows::core::HRESULT );
    let mut result__ = ::core::mem::MaybeUninit::zeroed();
    GetAppContainerRegistryLocation(desiredaccess, result__.as_mut_ptr()).from_abi(result__)
}
#[doc = "*Required features: `\"Win32_Security_Isolation\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn IsProcessInIsolatedContainer() -> ::windows::core::Result<super::super::Foundation::BOOL> {
    ::windows::core::link ! ( "api-ms-win-security-isolatedcontainer-l1-1-0.dll""system" fn IsProcessInIsolatedContainer ( isprocessinisolatedcontainer : *mut super::super::Foundation:: BOOL ) -> :: windows::core::HRESULT );
    let mut result__ = ::core::mem::MaybeUninit::zeroed();
    IsProcessInIsolatedContainer(result__.as_mut_ptr()).from_abi(result__)
}
#[doc = "*Required features: `\"Win32_Security_Isolation\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn IsProcessInIsolatedWindowsEnvironment() -> ::windows::core::Result<super::super::Foundation::BOOL> {
    ::windows::core::link ! ( "isolatedwindowsenvironmentutils.dll""system" fn IsProcessInIsolatedWindowsEnvironment ( isprocessinisolatedwindowsenvironment : *mut super::super::Foundation:: BOOL ) -> :: windows::core::HRESULT );
    let mut result__ = ::core::mem::MaybeUninit::zeroed();
    IsProcessInIsolatedWindowsEnvironment(result__.as_mut_ptr()).from_abi(result__)
}
#[doc = "*Required features: `\"Win32_Security_Isolation\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn IsProcessInWDAGContainer(reserved: *const ::core::ffi::c_void) -> ::windows::core::Result<super::super::Foundation::BOOL> {
    ::windows::core::link ! ( "api-ms-win-security-isolatedcontainer-l1-1-1.dll""system" fn IsProcessInWDAGContainer ( reserved : *const ::core::ffi::c_void , isprocessinwdagcontainer : *mut super::super::Foundation:: BOOL ) -> :: windows::core::HRESULT );
    let mut result__ = ::core::mem::MaybeUninit::zeroed();
    IsProcessInWDAGContainer(reserved, result__.as_mut_ptr()).from_abi(result__)
}
#[doc = "*Required features: `\"Win32_Security_Isolation\"`*"]
#[repr(transparent)]
pub struct IIsolatedAppLauncher(::windows::core::IUnknown);
impl IIsolatedAppLauncher {
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Launch<P0, P1>(&self, appusermodelid: P0, arguments: P1, telemetryparameters: *const IsolatedAppLauncherTelemetryParameters) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
        P1: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
    {
        (::windows::core::Vtable::vtable(self).Launch)(::windows::core::Vtable::as_raw(self), appusermodelid.into().abi(), arguments.into().abi(), telemetryparameters).ok()
    }
}
::windows::core::interface_hierarchy!(IIsolatedAppLauncher, ::windows::core::IUnknown);
impl ::core::clone::Clone for IIsolatedAppLauncher {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IIsolatedAppLauncher {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IIsolatedAppLauncher {}
impl ::core::fmt::Debug for IIsolatedAppLauncher {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IIsolatedAppLauncher").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Vtable for IIsolatedAppLauncher {
    type Vtable = IIsolatedAppLauncher_Vtbl;
}
unsafe impl ::windows::core::Interface for IIsolatedAppLauncher {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xf686878f_7b42_4cc4_96fb_f4f3b6e3d24d);
}
#[repr(C)]
#[doc(hidden)]
pub struct IIsolatedAppLauncher_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    #[cfg(feature = "Win32_Foundation")]
    pub Launch: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, appusermodelid: ::windows::core::PCWSTR, arguments: ::windows::core::PCWSTR, telemetryparameters: *const IsolatedAppLauncherTelemetryParameters) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    Launch: usize,
}
#[doc = "*Required features: `\"Win32_Security_Isolation\"`*"]
pub const IsolatedAppLauncher: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xbc812430_e75e_4fd1_9641_1f9f1e2d9a1f);
#[repr(C)]
#[doc = "*Required features: `\"Win32_Security_Isolation\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct IsolatedAppLauncherTelemetryParameters {
    pub EnableForLaunch: super::super::Foundation::BOOL,
    pub CorrelationGUID: ::windows::core::GUID,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for IsolatedAppLauncherTelemetryParameters {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for IsolatedAppLauncherTelemetryParameters {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for IsolatedAppLauncherTelemetryParameters {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("IsolatedAppLauncherTelemetryParameters").field("EnableForLaunch", &self.EnableForLaunch).field("CorrelationGUID", &self.CorrelationGUID).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for IsolatedAppLauncherTelemetryParameters {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for IsolatedAppLauncherTelemetryParameters {
    fn eq(&self, other: &Self) -> bool {
        self.EnableForLaunch == other.EnableForLaunch && self.CorrelationGUID == other.CorrelationGUID
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for IsolatedAppLauncherTelemetryParameters {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for IsolatedAppLauncherTelemetryParameters {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "implement")]
::core::include!("impl.rs");

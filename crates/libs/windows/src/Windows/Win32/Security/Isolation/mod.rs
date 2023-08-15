#[doc = "*Required features: `\"Win32_Security_Isolation\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn CreateAppContainerProfile<P0, P1, P2>(pszappcontainername: P0, pszdisplayname: P1, pszdescription: P2, pcapabilities: ::core::option::Option<&[super::SID_AND_ATTRIBUTES]>) -> ::windows_core::Result<super::super::Foundation::PSID>
where
    P0: ::windows_core::IntoParam<::windows_core::PCWSTR>,
    P1: ::windows_core::IntoParam<::windows_core::PCWSTR>,
    P2: ::windows_core::IntoParam<::windows_core::PCWSTR>,
{
    ::windows_targets::link!("userenv.dll" "system" fn CreateAppContainerProfile(pszappcontainername : ::windows_core::PCWSTR, pszdisplayname : ::windows_core::PCWSTR, pszdescription : ::windows_core::PCWSTR, pcapabilities : *const super:: SID_AND_ATTRIBUTES, dwcapabilitycount : u32, ppsidappcontainersid : *mut super::super::Foundation:: PSID) -> ::windows_core::HRESULT);
    let mut result__ = ::std::mem::zeroed();
    CreateAppContainerProfile(pszappcontainername.into_param().abi(), pszdisplayname.into_param().abi(), pszdescription.into_param().abi(), ::core::mem::transmute(pcapabilities.as_deref().map_or(::core::ptr::null(), |slice| slice.as_ptr())), pcapabilities.as_deref().map_or(0, |slice| slice.len() as _), &mut result__).from_abi(result__)
}
#[doc = "*Required features: `\"Win32_Security_Isolation\"`*"]
#[inline]
pub unsafe fn DeleteAppContainerProfile<P0>(pszappcontainername: P0) -> ::windows_core::Result<()>
where
    P0: ::windows_core::IntoParam<::windows_core::PCWSTR>,
{
    ::windows_targets::link!("userenv.dll" "system" fn DeleteAppContainerProfile(pszappcontainername : ::windows_core::PCWSTR) -> ::windows_core::HRESULT);
    DeleteAppContainerProfile(pszappcontainername.into_param().abi()).ok()
}
#[doc = "*Required features: `\"Win32_Security_Isolation\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn DeriveAppContainerSidFromAppContainerName<P0>(pszappcontainername: P0) -> ::windows_core::Result<super::super::Foundation::PSID>
where
    P0: ::windows_core::IntoParam<::windows_core::PCWSTR>,
{
    ::windows_targets::link!("userenv.dll" "system" fn DeriveAppContainerSidFromAppContainerName(pszappcontainername : ::windows_core::PCWSTR, ppsidappcontainersid : *mut super::super::Foundation:: PSID) -> ::windows_core::HRESULT);
    let mut result__ = ::std::mem::zeroed();
    DeriveAppContainerSidFromAppContainerName(pszappcontainername.into_param().abi(), &mut result__).from_abi(result__)
}
#[doc = "*Required features: `\"Win32_Security_Isolation\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn DeriveRestrictedAppContainerSidFromAppContainerSidAndRestrictedName<P0, P1>(psidappcontainersid: P0, pszrestrictedappcontainername: P1) -> ::windows_core::Result<super::super::Foundation::PSID>
where
    P0: ::windows_core::IntoParam<super::super::Foundation::PSID>,
    P1: ::windows_core::IntoParam<::windows_core::PCWSTR>,
{
    ::windows_targets::link!("userenv.dll" "system" fn DeriveRestrictedAppContainerSidFromAppContainerSidAndRestrictedName(psidappcontainersid : super::super::Foundation:: PSID, pszrestrictedappcontainername : ::windows_core::PCWSTR, ppsidrestrictedappcontainersid : *mut super::super::Foundation:: PSID) -> ::windows_core::HRESULT);
    let mut result__ = ::std::mem::zeroed();
    DeriveRestrictedAppContainerSidFromAppContainerSidAndRestrictedName(psidappcontainersid.into_param().abi(), pszrestrictedappcontainername.into_param().abi(), &mut result__).from_abi(result__)
}
#[doc = "*Required features: `\"Win32_Security_Isolation\"`*"]
#[inline]
pub unsafe fn GetAppContainerFolderPath<P0>(pszappcontainersid: P0) -> ::windows_core::Result<::windows_core::PWSTR>
where
    P0: ::windows_core::IntoParam<::windows_core::PCWSTR>,
{
    ::windows_targets::link!("userenv.dll" "system" fn GetAppContainerFolderPath(pszappcontainersid : ::windows_core::PCWSTR, ppszpath : *mut ::windows_core::PWSTR) -> ::windows_core::HRESULT);
    let mut result__ = ::std::mem::zeroed();
    GetAppContainerFolderPath(pszappcontainersid.into_param().abi(), &mut result__).from_abi(result__)
}
#[doc = "*Required features: `\"Win32_Security_Isolation\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn GetAppContainerNamedObjectPath<P0, P1>(token: P0, appcontainersid: P1, objectpath: ::core::option::Option<&mut [u16]>, returnlength: *mut u32) -> ::windows_core::Result<()>
where
    P0: ::windows_core::IntoParam<super::super::Foundation::HANDLE>,
    P1: ::windows_core::IntoParam<super::super::Foundation::PSID>,
{
    ::windows_targets::link!("kernel32.dll" "system" fn GetAppContainerNamedObjectPath(token : super::super::Foundation:: HANDLE, appcontainersid : super::super::Foundation:: PSID, objectpathlength : u32, objectpath : ::windows_core::PWSTR, returnlength : *mut u32) -> super::super::Foundation:: BOOL);
    GetAppContainerNamedObjectPath(token.into_param().abi(), appcontainersid.into_param().abi(), objectpath.as_deref().map_or(0, |slice| slice.len() as _), ::core::mem::transmute(objectpath.as_deref().map_or(::core::ptr::null(), |slice| slice.as_ptr())), returnlength).ok()
}
#[doc = "*Required features: `\"Win32_Security_Isolation\"`, `\"Win32_System_Registry\"`*"]
#[cfg(feature = "Win32_System_Registry")]
#[inline]
pub unsafe fn GetAppContainerRegistryLocation(desiredaccess: u32) -> ::windows_core::Result<super::super::System::Registry::HKEY> {
    ::windows_targets::link!("userenv.dll" "system" fn GetAppContainerRegistryLocation(desiredaccess : u32, phappcontainerkey : *mut super::super::System::Registry:: HKEY) -> ::windows_core::HRESULT);
    let mut result__ = ::std::mem::zeroed();
    GetAppContainerRegistryLocation(desiredaccess, &mut result__).from_abi(result__)
}
#[doc = "*Required features: `\"Win32_Security_Isolation\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn IsCrossIsolatedEnvironmentClipboardContent() -> ::windows_core::Result<super::super::Foundation::BOOL> {
    ::windows_targets::link!("isolatedwindowsenvironmentutils.dll" "system" fn IsCrossIsolatedEnvironmentClipboardContent(iscrossisolatedenvironmentclipboardcontent : *mut super::super::Foundation:: BOOL) -> ::windows_core::HRESULT);
    let mut result__ = ::std::mem::zeroed();
    IsCrossIsolatedEnvironmentClipboardContent(&mut result__).from_abi(result__)
}
#[doc = "*Required features: `\"Win32_Security_Isolation\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn IsProcessInIsolatedContainer() -> ::windows_core::Result<super::super::Foundation::BOOL> {
    ::windows_targets::link!("api-ms-win-security-isolatedcontainer-l1-1-0.dll" "system" fn IsProcessInIsolatedContainer(isprocessinisolatedcontainer : *mut super::super::Foundation:: BOOL) -> ::windows_core::HRESULT);
    let mut result__ = ::std::mem::zeroed();
    IsProcessInIsolatedContainer(&mut result__).from_abi(result__)
}
#[doc = "*Required features: `\"Win32_Security_Isolation\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn IsProcessInIsolatedWindowsEnvironment() -> ::windows_core::Result<super::super::Foundation::BOOL> {
    ::windows_targets::link!("isolatedwindowsenvironmentutils.dll" "system" fn IsProcessInIsolatedWindowsEnvironment(isprocessinisolatedwindowsenvironment : *mut super::super::Foundation:: BOOL) -> ::windows_core::HRESULT);
    let mut result__ = ::std::mem::zeroed();
    IsProcessInIsolatedWindowsEnvironment(&mut result__).from_abi(result__)
}
#[doc = "*Required features: `\"Win32_Security_Isolation\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn IsProcessInWDAGContainer(reserved: *const ::core::ffi::c_void) -> ::windows_core::Result<super::super::Foundation::BOOL> {
    ::windows_targets::link!("api-ms-win-security-isolatedcontainer-l1-1-1.dll" "system" fn IsProcessInWDAGContainer(reserved : *const ::core::ffi::c_void, isprocessinwdagcontainer : *mut super::super::Foundation:: BOOL) -> ::windows_core::HRESULT);
    let mut result__ = ::std::mem::zeroed();
    IsProcessInWDAGContainer(reserved, &mut result__).from_abi(result__)
}
#[doc = "*Required features: `\"Win32_Security_Isolation\"`*"]
#[repr(transparent)]
pub struct IIsolatedAppLauncher(::windows_core::IUnknown);
impl IIsolatedAppLauncher {
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Launch<P0, P1>(&self, appusermodelid: P0, arguments: P1, telemetryparameters: *const IsolatedAppLauncherTelemetryParameters) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::PCWSTR>,
        P1: ::windows_core::IntoParam<::windows_core::PCWSTR>,
    {
        (::windows_core::Interface::vtable(self).Launch)(::windows_core::Interface::as_raw(self), appusermodelid.into_param().abi(), arguments.into_param().abi(), telemetryparameters).ok()
    }
}
::windows_core::imp::interface_hierarchy!(IIsolatedAppLauncher, ::windows_core::IUnknown);
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
unsafe impl ::windows_core::Interface for IIsolatedAppLauncher {
    type Vtable = IIsolatedAppLauncher_Vtbl;
}
impl ::core::clone::Clone for IIsolatedAppLauncher {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for IIsolatedAppLauncher {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xf686878f_7b42_4cc4_96fb_f4f3b6e3d24d);
}
#[repr(C)]
#[doc(hidden)]
pub struct IIsolatedAppLauncher_Vtbl {
    pub base__: ::windows_core::IUnknown_Vtbl,
    #[cfg(feature = "Win32_Foundation")]
    pub Launch: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, appusermodelid: ::windows_core::PCWSTR, arguments: ::windows_core::PCWSTR, telemetryparameters: *const IsolatedAppLauncherTelemetryParameters) -> ::windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    Launch: usize,
}
#[doc = "*Required features: `\"Win32_Security_Isolation\"`*"]
pub const IsolatedAppLauncher: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xbc812430_e75e_4fd1_9641_1f9f1e2d9a1f);
#[doc = "*Required features: `\"Win32_Security_Isolation\"`*"]
pub const WDAG_CLIPBOARD_TAG: ::windows_core::PCWSTR = ::windows_core::w!("CrossIsolatedEnvironmentContent");
#[repr(C)]
#[doc = "*Required features: `\"Win32_Security_Isolation\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct IsolatedAppLauncherTelemetryParameters {
    pub EnableForLaunch: super::super::Foundation::BOOL,
    pub CorrelationGUID: ::windows_core::GUID,
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
impl ::windows_core::TypeKind for IsolatedAppLauncherTelemetryParameters {
    type TypeKind = ::windows_core::CopyType;
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

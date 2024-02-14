#[inline]
pub unsafe fn CreateAppContainerProfile<P0, P1, P2>(pszappcontainername: P0, pszdisplayname: P1, pszdescription: P2, pcapabilities: ::core::option::Option<&[super::SID_AND_ATTRIBUTES]>) -> ::windows_core::Result<super::super::Foundation::PSID>
where
    P0: ::windows_core::IntoParam<::windows_core::PCWSTR>,
    P1: ::windows_core::IntoParam<::windows_core::PCWSTR>,
    P2: ::windows_core::IntoParam<::windows_core::PCWSTR>,
{
    ::windows_targets::link!("userenv.dll" "system" fn CreateAppContainerProfile(pszappcontainername : ::windows_core::PCWSTR, pszdisplayname : ::windows_core::PCWSTR, pszdescription : ::windows_core::PCWSTR, pcapabilities : *const super:: SID_AND_ATTRIBUTES, dwcapabilitycount : u32, ppsidappcontainersid : *mut super::super::Foundation:: PSID) -> ::windows_core::HRESULT);
    let mut result__ = ::std::mem::zeroed();
    CreateAppContainerProfile(pszappcontainername.into_param().abi(), pszdisplayname.into_param().abi(), pszdescription.into_param().abi(), ::core::mem::transmute(pcapabilities.as_deref().map_or(::core::ptr::null(), |slice| slice.as_ptr())), pcapabilities.as_deref().map_or(0, |slice| slice.len().try_into().unwrap()), &mut result__).map(|| result__)
}
#[inline]
pub unsafe fn DeleteAppContainerProfile<P0>(pszappcontainername: P0) -> ::windows_core::Result<()>
where
    P0: ::windows_core::IntoParam<::windows_core::PCWSTR>,
{
    ::windows_targets::link!("userenv.dll" "system" fn DeleteAppContainerProfile(pszappcontainername : ::windows_core::PCWSTR) -> ::windows_core::HRESULT);
    DeleteAppContainerProfile(pszappcontainername.into_param().abi()).ok()
}
#[inline]
pub unsafe fn DeriveAppContainerSidFromAppContainerName<P0>(pszappcontainername: P0) -> ::windows_core::Result<super::super::Foundation::PSID>
where
    P0: ::windows_core::IntoParam<::windows_core::PCWSTR>,
{
    ::windows_targets::link!("userenv.dll" "system" fn DeriveAppContainerSidFromAppContainerName(pszappcontainername : ::windows_core::PCWSTR, ppsidappcontainersid : *mut super::super::Foundation:: PSID) -> ::windows_core::HRESULT);
    let mut result__ = ::std::mem::zeroed();
    DeriveAppContainerSidFromAppContainerName(pszappcontainername.into_param().abi(), &mut result__).map(|| result__)
}
#[inline]
pub unsafe fn DeriveRestrictedAppContainerSidFromAppContainerSidAndRestrictedName<P0, P1>(psidappcontainersid: P0, pszrestrictedappcontainername: P1) -> ::windows_core::Result<super::super::Foundation::PSID>
where
    P0: ::windows_core::IntoParam<super::super::Foundation::PSID>,
    P1: ::windows_core::IntoParam<::windows_core::PCWSTR>,
{
    ::windows_targets::link!("userenv.dll" "system" fn DeriveRestrictedAppContainerSidFromAppContainerSidAndRestrictedName(psidappcontainersid : super::super::Foundation:: PSID, pszrestrictedappcontainername : ::windows_core::PCWSTR, ppsidrestrictedappcontainersid : *mut super::super::Foundation:: PSID) -> ::windows_core::HRESULT);
    let mut result__ = ::std::mem::zeroed();
    DeriveRestrictedAppContainerSidFromAppContainerSidAndRestrictedName(psidappcontainersid.into_param().abi(), pszrestrictedappcontainername.into_param().abi(), &mut result__).map(|| result__)
}
#[inline]
pub unsafe fn GetAppContainerFolderPath<P0>(pszappcontainersid: P0) -> ::windows_core::Result<::windows_core::PWSTR>
where
    P0: ::windows_core::IntoParam<::windows_core::PCWSTR>,
{
    ::windows_targets::link!("userenv.dll" "system" fn GetAppContainerFolderPath(pszappcontainersid : ::windows_core::PCWSTR, ppszpath : *mut ::windows_core::PWSTR) -> ::windows_core::HRESULT);
    let mut result__ = ::std::mem::zeroed();
    GetAppContainerFolderPath(pszappcontainersid.into_param().abi(), &mut result__).map(|| result__)
}
#[inline]
pub unsafe fn GetAppContainerNamedObjectPath<P0, P1>(token: P0, appcontainersid: P1, objectpath: ::core::option::Option<&mut [u16]>, returnlength: *mut u32) -> ::windows_core::Result<()>
where
    P0: ::windows_core::IntoParam<super::super::Foundation::HANDLE>,
    P1: ::windows_core::IntoParam<super::super::Foundation::PSID>,
{
    ::windows_targets::link!("kernel32.dll" "system" fn GetAppContainerNamedObjectPath(token : super::super::Foundation:: HANDLE, appcontainersid : super::super::Foundation:: PSID, objectpathlength : u32, objectpath : ::windows_core::PWSTR, returnlength : *mut u32) -> super::super::Foundation:: BOOL);
    GetAppContainerNamedObjectPath(token.into_param().abi(), appcontainersid.into_param().abi(), objectpath.as_deref().map_or(0, |slice| slice.len().try_into().unwrap()), ::core::mem::transmute(objectpath.as_deref().map_or(::core::ptr::null(), |slice| slice.as_ptr())), returnlength).ok()
}
#[cfg(feature = "Win32_System_Registry")]
#[inline]
pub unsafe fn GetAppContainerRegistryLocation(desiredaccess: u32) -> ::windows_core::Result<super::super::System::Registry::HKEY> {
    ::windows_targets::link!("userenv.dll" "system" fn GetAppContainerRegistryLocation(desiredaccess : u32, phappcontainerkey : *mut super::super::System::Registry:: HKEY) -> ::windows_core::HRESULT);
    let mut result__ = ::std::mem::zeroed();
    GetAppContainerRegistryLocation(desiredaccess, &mut result__).map(|| result__)
}
#[inline]
pub unsafe fn IsCrossIsolatedEnvironmentClipboardContent() -> ::windows_core::Result<super::super::Foundation::BOOL> {
    ::windows_targets::link!("isolatedwindowsenvironmentutils.dll" "system" fn IsCrossIsolatedEnvironmentClipboardContent(iscrossisolatedenvironmentclipboardcontent : *mut super::super::Foundation:: BOOL) -> ::windows_core::HRESULT);
    let mut result__ = ::std::mem::zeroed();
    IsCrossIsolatedEnvironmentClipboardContent(&mut result__).map(|| result__)
}
#[inline]
pub unsafe fn IsProcessInIsolatedContainer() -> ::windows_core::Result<super::super::Foundation::BOOL> {
    ::windows_targets::link!("api-ms-win-security-isolatedcontainer-l1-1-0.dll" "system" fn IsProcessInIsolatedContainer(isprocessinisolatedcontainer : *mut super::super::Foundation:: BOOL) -> ::windows_core::HRESULT);
    let mut result__ = ::std::mem::zeroed();
    IsProcessInIsolatedContainer(&mut result__).map(|| result__)
}
#[inline]
pub unsafe fn IsProcessInIsolatedWindowsEnvironment() -> ::windows_core::Result<super::super::Foundation::BOOL> {
    ::windows_targets::link!("isolatedwindowsenvironmentutils.dll" "system" fn IsProcessInIsolatedWindowsEnvironment(isprocessinisolatedwindowsenvironment : *mut super::super::Foundation:: BOOL) -> ::windows_core::HRESULT);
    let mut result__ = ::std::mem::zeroed();
    IsProcessInIsolatedWindowsEnvironment(&mut result__).map(|| result__)
}
#[inline]
pub unsafe fn IsProcessInWDAGContainer(reserved: *const ::core::ffi::c_void) -> ::windows_core::Result<super::super::Foundation::BOOL> {
    ::windows_targets::link!("api-ms-win-security-isolatedcontainer-l1-1-1.dll" "system" fn IsProcessInWDAGContainer(reserved : *const ::core::ffi::c_void, isprocessinwdagcontainer : *mut super::super::Foundation:: BOOL) -> ::windows_core::HRESULT);
    let mut result__ = ::std::mem::zeroed();
    IsProcessInWDAGContainer(reserved, &mut result__).map(|| result__)
}
::windows_core::imp::com_interface!(IIsolatedAppLauncher, IIsolatedAppLauncher_Vtbl, 0xf686878f_7b42_4cc4_96fb_f4f3b6e3d24d);
::windows_core::imp::interface_hierarchy!(IIsolatedAppLauncher, ::windows_core::IUnknown);
impl IIsolatedAppLauncher {
    pub unsafe fn Launch<P0, P1>(&self, appusermodelid: P0, arguments: P1, telemetryparameters: *const IsolatedAppLauncherTelemetryParameters) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::PCWSTR>,
        P1: ::windows_core::IntoParam<::windows_core::PCWSTR>,
    {
        (::windows_core::Interface::vtable(self).Launch)(::windows_core::Interface::as_raw(self), appusermodelid.into_param().abi(), arguments.into_param().abi(), telemetryparameters).ok()
    }
}
#[repr(C)]
pub struct IIsolatedAppLauncher_Vtbl {
    pub base__: ::windows_core::IUnknown_Vtbl,
    pub Launch: unsafe extern "system" fn(*mut ::core::ffi::c_void, ::windows_core::PCWSTR, ::windows_core::PCWSTR, *const IsolatedAppLauncherTelemetryParameters) -> ::windows_core::HRESULT,
}
::windows_core::imp::com_interface!(IIsolatedProcessLauncher, IIsolatedProcessLauncher_Vtbl, 0x1aa24232_9a91_4201_88cb_122f9d6522e0);
::windows_core::imp::interface_hierarchy!(IIsolatedProcessLauncher, ::windows_core::IUnknown);
impl IIsolatedProcessLauncher {
    pub unsafe fn LaunchProcess<P0, P1, P2>(&self, process: P0, arguments: P1, workingdirectory: P2) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::PCWSTR>,
        P1: ::windows_core::IntoParam<::windows_core::PCWSTR>,
        P2: ::windows_core::IntoParam<::windows_core::PCWSTR>,
    {
        (::windows_core::Interface::vtable(self).LaunchProcess)(::windows_core::Interface::as_raw(self), process.into_param().abi(), arguments.into_param().abi(), workingdirectory.into_param().abi()).ok()
    }
    pub unsafe fn ShareDirectory<P0, P1, P2>(&self, hostpath: P0, containerpath: P1, readonly: P2) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::PCWSTR>,
        P1: ::windows_core::IntoParam<::windows_core::PCWSTR>,
        P2: ::windows_core::IntoParam<super::super::Foundation::BOOL>,
    {
        (::windows_core::Interface::vtable(self).ShareDirectory)(::windows_core::Interface::as_raw(self), hostpath.into_param().abi(), containerpath.into_param().abi(), readonly.into_param().abi()).ok()
    }
    pub unsafe fn GetContainerGuid(&self) -> ::windows_core::Result<::windows_core::GUID> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).GetContainerGuid)(::windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn AllowSetForegroundAccess(&self, pid: u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).AllowSetForegroundAccess)(::windows_core::Interface::as_raw(self), pid).ok()
    }
    pub unsafe fn IsContainerRunning(&self) -> ::windows_core::Result<super::super::Foundation::BOOL> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).IsContainerRunning)(::windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
}
#[repr(C)]
pub struct IIsolatedProcessLauncher_Vtbl {
    pub base__: ::windows_core::IUnknown_Vtbl,
    pub LaunchProcess: unsafe extern "system" fn(*mut ::core::ffi::c_void, ::windows_core::PCWSTR, ::windows_core::PCWSTR, ::windows_core::PCWSTR) -> ::windows_core::HRESULT,
    pub ShareDirectory: unsafe extern "system" fn(*mut ::core::ffi::c_void, ::windows_core::PCWSTR, ::windows_core::PCWSTR, super::super::Foundation::BOOL) -> ::windows_core::HRESULT,
    pub GetContainerGuid: unsafe extern "system" fn(*mut ::core::ffi::c_void, *mut ::windows_core::GUID) -> ::windows_core::HRESULT,
    pub AllowSetForegroundAccess: unsafe extern "system" fn(*mut ::core::ffi::c_void, u32) -> ::windows_core::HRESULT,
    pub IsContainerRunning: unsafe extern "system" fn(*mut ::core::ffi::c_void, *mut super::super::Foundation::BOOL) -> ::windows_core::HRESULT,
}
::windows_core::imp::com_interface!(IIsolatedProcessLauncher2, IIsolatedProcessLauncher2_Vtbl, 0x780e4416_5e72_4123_808e_66dc6479feef);
::windows_core::imp::interface_hierarchy!(IIsolatedProcessLauncher2, ::windows_core::IUnknown, IIsolatedProcessLauncher);
impl IIsolatedProcessLauncher2 {
    pub unsafe fn LaunchProcess<P0, P1, P2>(&self, process: P0, arguments: P1, workingdirectory: P2) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::PCWSTR>,
        P1: ::windows_core::IntoParam<::windows_core::PCWSTR>,
        P2: ::windows_core::IntoParam<::windows_core::PCWSTR>,
    {
        (::windows_core::Interface::vtable(self).base__.LaunchProcess)(::windows_core::Interface::as_raw(self), process.into_param().abi(), arguments.into_param().abi(), workingdirectory.into_param().abi()).ok()
    }
    pub unsafe fn ShareDirectory<P0, P1, P2>(&self, hostpath: P0, containerpath: P1, readonly: P2) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::PCWSTR>,
        P1: ::windows_core::IntoParam<::windows_core::PCWSTR>,
        P2: ::windows_core::IntoParam<super::super::Foundation::BOOL>,
    {
        (::windows_core::Interface::vtable(self).base__.ShareDirectory)(::windows_core::Interface::as_raw(self), hostpath.into_param().abi(), containerpath.into_param().abi(), readonly.into_param().abi()).ok()
    }
    pub unsafe fn GetContainerGuid(&self) -> ::windows_core::Result<::windows_core::GUID> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.GetContainerGuid)(::windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn AllowSetForegroundAccess(&self, pid: u32) -> ::windows_core::Result<()> {
        (::windows_core::Interface::vtable(self).base__.AllowSetForegroundAccess)(::windows_core::Interface::as_raw(self), pid).ok()
    }
    pub unsafe fn IsContainerRunning(&self) -> ::windows_core::Result<super::super::Foundation::BOOL> {
        let mut result__ = ::std::mem::zeroed();
        (::windows_core::Interface::vtable(self).base__.IsContainerRunning)(::windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
    }
    pub unsafe fn LaunchProcess2<P0, P1, P2>(&self, process: P0, arguments: P1, workingdirectory: P2, correlationguid: *const ::windows_core::GUID) -> ::windows_core::Result<()>
    where
        P0: ::windows_core::IntoParam<::windows_core::PCWSTR>,
        P1: ::windows_core::IntoParam<::windows_core::PCWSTR>,
        P2: ::windows_core::IntoParam<::windows_core::PCWSTR>,
    {
        (::windows_core::Interface::vtable(self).LaunchProcess2)(::windows_core::Interface::as_raw(self), process.into_param().abi(), arguments.into_param().abi(), workingdirectory.into_param().abi(), correlationguid).ok()
    }
}
#[repr(C)]
pub struct IIsolatedProcessLauncher2_Vtbl {
    pub base__: IIsolatedProcessLauncher_Vtbl,
    pub LaunchProcess2: unsafe extern "system" fn(*mut ::core::ffi::c_void, ::windows_core::PCWSTR, ::windows_core::PCWSTR, ::windows_core::PCWSTR, *const ::windows_core::GUID) -> ::windows_core::HRESULT,
}
pub const IsolatedAppLauncher: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xbc812430_e75e_4fd1_9641_1f9f1e2d9a1f);
pub const WDAG_CLIPBOARD_TAG: ::windows_core::PCWSTR = ::windows_core::w!("CrossIsolatedEnvironmentContent");
#[repr(C)]
pub struct IsolatedAppLauncherTelemetryParameters {
    pub EnableForLaunch: super::super::Foundation::BOOL,
    pub CorrelationGUID: ::windows_core::GUID,
}
impl ::core::marker::Copy for IsolatedAppLauncherTelemetryParameters {}
impl ::core::clone::Clone for IsolatedAppLauncherTelemetryParameters {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for IsolatedAppLauncherTelemetryParameters {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("IsolatedAppLauncherTelemetryParameters").field("EnableForLaunch", &self.EnableForLaunch).field("CorrelationGUID", &self.CorrelationGUID).finish()
    }
}
impl ::windows_core::TypeKind for IsolatedAppLauncherTelemetryParameters {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::cmp::PartialEq for IsolatedAppLauncherTelemetryParameters {
    fn eq(&self, other: &Self) -> bool {
        self.EnableForLaunch == other.EnableForLaunch && self.CorrelationGUID == other.CorrelationGUID
    }
}
impl ::core::cmp::Eq for IsolatedAppLauncherTelemetryParameters {}
impl ::core::default::Default for IsolatedAppLauncherTelemetryParameters {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "implement")]
::core::include!("impl.rs");

#![allow(unused_variables, non_upper_case_globals, non_snake_case, unused_unsafe, non_camel_case_types, dead_code, clippy::all)]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn CreateAppContainerProfile<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>, Param2: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(pszappcontainername: Param0, pszdisplayname: Param1, pszdescription: Param2, pcapabilities: *const super::SID_AND_ATTRIBUTES, dwcapabilitycount: u32) -> ::windows::core::Result<super::super::Foundation::PSID> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CreateAppContainerProfile(pszappcontainername: super::super::Foundation::PWSTR, pszdisplayname: super::super::Foundation::PWSTR, pszdescription: super::super::Foundation::PWSTR, pcapabilities: *const super::SID_AND_ATTRIBUTES, dwcapabilitycount: u32, ppsidappcontainersid: *mut super::super::Foundation::PSID) -> ::windows::core::HRESULT;
        }
        let mut result__: <super::super::Foundation::PSID as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        CreateAppContainerProfile(pszappcontainername.into_param().abi(), pszdisplayname.into_param().abi(), pszdescription.into_param().abi(), ::core::mem::transmute(pcapabilities), ::core::mem::transmute(dwcapabilitycount), &mut result__).from_abi::<super::super::Foundation::PSID>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn DeleteAppContainerProfile<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(pszappcontainername: Param0) -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn DeleteAppContainerProfile(pszappcontainername: super::super::Foundation::PWSTR) -> ::windows::core::HRESULT;
        }
        DeleteAppContainerProfile(pszappcontainername.into_param().abi()).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn DeriveAppContainerSidFromAppContainerName<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(pszappcontainername: Param0) -> ::windows::core::Result<super::super::Foundation::PSID> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn DeriveAppContainerSidFromAppContainerName(pszappcontainername: super::super::Foundation::PWSTR, ppsidappcontainersid: *mut super::super::Foundation::PSID) -> ::windows::core::HRESULT;
        }
        let mut result__: <super::super::Foundation::PSID as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        DeriveAppContainerSidFromAppContainerName(pszappcontainername.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::PSID>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn DeriveRestrictedAppContainerSidFromAppContainerSidAndRestrictedName<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PSID>, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(psidappcontainersid: Param0, pszrestrictedappcontainername: Param1) -> ::windows::core::Result<super::super::Foundation::PSID> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn DeriveRestrictedAppContainerSidFromAppContainerSidAndRestrictedName(psidappcontainersid: super::super::Foundation::PSID, pszrestrictedappcontainername: super::super::Foundation::PWSTR, ppsidrestrictedappcontainersid: *mut super::super::Foundation::PSID) -> ::windows::core::HRESULT;
        }
        let mut result__: <super::super::Foundation::PSID as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        DeriveRestrictedAppContainerSidFromAppContainerSidAndRestrictedName(psidappcontainersid.into_param().abi(), pszrestrictedappcontainername.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::PSID>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn GetAppContainerFolderPath<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(pszappcontainersid: Param0) -> ::windows::core::Result<super::super::Foundation::PWSTR> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn GetAppContainerFolderPath(pszappcontainersid: super::super::Foundation::PWSTR, ppszpath: *mut super::super::Foundation::PWSTR) -> ::windows::core::HRESULT;
        }
        let mut result__: <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        GetAppContainerFolderPath(pszappcontainersid.into_param().abi(), &mut result__).from_abi::<super::super::Foundation::PWSTR>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn GetAppContainerNamedObjectPath<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::HANDLE>, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::PSID>>(token: Param0, appcontainersid: Param1, objectpathlength: u32, objectpath: super::super::Foundation::PWSTR, returnlength: *mut u32) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn GetAppContainerNamedObjectPath(token: super::super::Foundation::HANDLE, appcontainersid: super::super::Foundation::PSID, objectpathlength: u32, objectpath: super::super::Foundation::PWSTR, returnlength: *mut u32) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(GetAppContainerNamedObjectPath(token.into_param().abi(), appcontainersid.into_param().abi(), ::core::mem::transmute(objectpathlength), ::core::mem::transmute(objectpath), ::core::mem::transmute(returnlength)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_System_Registry")]
#[inline]
pub unsafe fn GetAppContainerRegistryLocation(desiredaccess: u32) -> ::windows::core::Result<super::super::System::Registry::HKEY> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn GetAppContainerRegistryLocation(desiredaccess: u32, phappcontainerkey: *mut super::super::System::Registry::HKEY) -> ::windows::core::HRESULT;
        }
        let mut result__: <super::super::System::Registry::HKEY as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        GetAppContainerRegistryLocation(::core::mem::transmute(desiredaccess), &mut result__).from_abi::<super::super::System::Registry::HKEY>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IIsolatedAppLauncher(pub ::windows::core::IUnknown);
impl IIsolatedAppLauncher {
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Launch<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(&self, appusermodelid: Param0, arguments: Param1, telemetryparameters: *const IsolatedAppLauncherTelemetryParameters) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), appusermodelid.into_param().abi(), arguments.into_param().abi(), ::core::mem::transmute(telemetryparameters)).ok()
    }
}
unsafe impl ::windows::core::Interface for IIsolatedAppLauncher {
    type Vtable = IIsolatedAppLauncher_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xf686878f_7b42_4cc4_96fb_f4f3b6e3d24d);
}
impl ::core::convert::From<IIsolatedAppLauncher> for ::windows::core::IUnknown {
    fn from(value: IIsolatedAppLauncher) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IIsolatedAppLauncher> for ::windows::core::IUnknown {
    fn from(value: &IIsolatedAppLauncher) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IIsolatedAppLauncher {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IIsolatedAppLauncher {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IIsolatedAppLauncher_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, appusermodelid: super::super::Foundation::PWSTR, arguments: super::super::Foundation::PWSTR, telemetryparameters: *const IsolatedAppLauncherTelemetryParameters) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
);
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn IsProcessInIsolatedContainer() -> ::windows::core::Result<super::super::Foundation::BOOL> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn IsProcessInIsolatedContainer(isprocessinisolatedcontainer: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT;
        }
        let mut result__: <super::super::Foundation::BOOL as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        IsProcessInIsolatedContainer(&mut result__).from_abi::<super::super::Foundation::BOOL>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn IsProcessInIsolatedWindowsEnvironment() -> ::windows::core::Result<super::super::Foundation::BOOL> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn IsProcessInIsolatedWindowsEnvironment(isprocessinisolatedwindowsenvironment: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT;
        }
        let mut result__: <super::super::Foundation::BOOL as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        IsProcessInIsolatedWindowsEnvironment(&mut result__).from_abi::<super::super::Foundation::BOOL>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn IsProcessInWDAGContainer(reserved: *const ::core::ffi::c_void) -> ::windows::core::Result<super::super::Foundation::BOOL> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn IsProcessInWDAGContainer(reserved: *const ::core::ffi::c_void, isprocessinwdagcontainer: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT;
        }
        let mut result__: <super::super::Foundation::BOOL as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        IsProcessInWDAGContainer(::core::mem::transmute(reserved), &mut result__).from_abi::<super::super::Foundation::BOOL>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
pub const IsolatedAppLauncher: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xbc812430_e75e_4fd1_9641_1f9f1e2d9a1f);
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct IsolatedAppLauncherTelemetryParameters {
    pub EnableForLaunch: super::super::Foundation::BOOL,
    pub CorrelationGUID: ::windows::core::GUID,
}
#[cfg(feature = "Win32_Foundation")]
impl IsolatedAppLauncherTelemetryParameters {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for IsolatedAppLauncherTelemetryParameters {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for IsolatedAppLauncherTelemetryParameters {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("IsolatedAppLauncherTelemetryParameters").field("EnableForLaunch", &self.EnableForLaunch).field("CorrelationGUID", &self.CorrelationGUID).finish()
    }
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
unsafe impl ::windows::core::Abi for IsolatedAppLauncherTelemetryParameters {
    type Abi = Self;
}

#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, clashing_extern_declarations, clippy::all)]
#[doc = "*Required features: `\"Win32_Security_Isolation\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn CreateAppContainerProfile<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>, Param1: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>, Param2: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>>(pszappcontainername: Param0, pszdisplayname: Param1, pszdescription: Param2, pcapabilities: &[super::SID_AND_ATTRIBUTES]) -> ::windows::core::Result<super::super::Foundation::PSID> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CreateAppContainerProfile(pszappcontainername: ::windows::core::PCWSTR, pszdisplayname: ::windows::core::PCWSTR, pszdescription: ::windows::core::PCWSTR, pcapabilities: *const super::SID_AND_ATTRIBUTES, dwcapabilitycount: u32, ppsidappcontainersid: *mut super::super::Foundation::PSID) -> ::windows::core::HRESULT;
        }
        let mut result__: super::super::Foundation::PSID = ::core::mem::zeroed();
        CreateAppContainerProfile(pszappcontainername.into_param().abi(), pszdisplayname.into_param().abi(), pszdescription.into_param().abi(), ::core::mem::transmute(::windows::core::as_ptr_or_null(pcapabilities)), pcapabilities.len() as _, ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::PSID>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Security_Isolation\"`*"]
#[inline]
pub unsafe fn DeleteAppContainerProfile<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>>(pszappcontainername: Param0) -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn DeleteAppContainerProfile(pszappcontainername: ::windows::core::PCWSTR) -> ::windows::core::HRESULT;
        }
        DeleteAppContainerProfile(pszappcontainername.into_param().abi()).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Security_Isolation\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn DeriveAppContainerSidFromAppContainerName<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>>(pszappcontainername: Param0) -> ::windows::core::Result<super::super::Foundation::PSID> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn DeriveAppContainerSidFromAppContainerName(pszappcontainername: ::windows::core::PCWSTR, ppsidappcontainersid: *mut super::super::Foundation::PSID) -> ::windows::core::HRESULT;
        }
        let mut result__: super::super::Foundation::PSID = ::core::mem::zeroed();
        DeriveAppContainerSidFromAppContainerName(pszappcontainername.into_param().abi(), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::PSID>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Security_Isolation\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn DeriveRestrictedAppContainerSidFromAppContainerSidAndRestrictedName<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PSID>, Param1: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>>(psidappcontainersid: Param0, pszrestrictedappcontainername: Param1) -> ::windows::core::Result<super::super::Foundation::PSID> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn DeriveRestrictedAppContainerSidFromAppContainerSidAndRestrictedName(psidappcontainersid: super::super::Foundation::PSID, pszrestrictedappcontainername: ::windows::core::PCWSTR, ppsidrestrictedappcontainersid: *mut super::super::Foundation::PSID) -> ::windows::core::HRESULT;
        }
        let mut result__: super::super::Foundation::PSID = ::core::mem::zeroed();
        DeriveRestrictedAppContainerSidFromAppContainerSidAndRestrictedName(psidappcontainersid.into_param().abi(), pszrestrictedappcontainername.into_param().abi(), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::PSID>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Security_Isolation\"`*"]
#[inline]
pub unsafe fn GetAppContainerFolderPath<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>>(pszappcontainersid: Param0) -> ::windows::core::Result<::windows::core::PWSTR> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn GetAppContainerFolderPath(pszappcontainersid: ::windows::core::PCWSTR, ppszpath: *mut ::windows::core::PWSTR) -> ::windows::core::HRESULT;
        }
        let mut result__: ::windows::core::PWSTR = ::core::mem::zeroed();
        GetAppContainerFolderPath(pszappcontainersid.into_param().abi(), ::core::mem::transmute(&mut result__)).from_abi::<::windows::core::PWSTR>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Security_Isolation\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn GetAppContainerNamedObjectPath<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::HANDLE>, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::PSID>>(token: Param0, appcontainersid: Param1, objectpath: &mut [u16], returnlength: *mut u32) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn GetAppContainerNamedObjectPath(token: super::super::Foundation::HANDLE, appcontainersid: super::super::Foundation::PSID, objectpathlength: u32, objectpath: ::windows::core::PWSTR, returnlength: *mut u32) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(GetAppContainerNamedObjectPath(token.into_param().abi(), appcontainersid.into_param().abi(), objectpath.len() as _, ::core::mem::transmute(::windows::core::as_mut_ptr_or_null(objectpath)), ::core::mem::transmute(returnlength)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Security_Isolation\"`, `\"Win32_System_Registry\"`*"]
#[cfg(feature = "Win32_System_Registry")]
#[inline]
pub unsafe fn GetAppContainerRegistryLocation(desiredaccess: u32) -> ::windows::core::Result<super::super::System::Registry::HKEY> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn GetAppContainerRegistryLocation(desiredaccess: u32, phappcontainerkey: *mut super::super::System::Registry::HKEY) -> ::windows::core::HRESULT;
        }
        let mut result__: super::super::System::Registry::HKEY = ::core::mem::zeroed();
        GetAppContainerRegistryLocation(::core::mem::transmute(desiredaccess), ::core::mem::transmute(&mut result__)).from_abi::<super::super::System::Registry::HKEY>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Security_Isolation\"`*"]
#[repr(transparent)]
pub struct IIsolatedAppLauncher(::windows::core::IUnknown);
impl IIsolatedAppLauncher {
    #[doc = "*Required features: `\"Win32_Security_Isolation\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Launch<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>, Param1: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>>(&self, appusermodelid: Param0, arguments: Param1, telemetryparameters: *const IsolatedAppLauncherTelemetryParameters) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Launch)(::core::mem::transmute_copy(self), appusermodelid.into_param().abi(), arguments.into_param().abi(), ::core::mem::transmute(telemetryparameters)).ok()
    }
}
impl ::core::convert::From<IIsolatedAppLauncher> for ::windows::core::IUnknown {
    fn from(value: IIsolatedAppLauncher) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IIsolatedAppLauncher> for ::windows::core::IUnknown {
    fn from(value: &IIsolatedAppLauncher) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IIsolatedAppLauncher {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IIsolatedAppLauncher {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
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
unsafe impl ::windows::core::Interface for IIsolatedAppLauncher {
    type Vtable = IIsolatedAppLauncher_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xf686878f_7b42_4cc4_96fb_f4f3b6e3d24d);
}
#[repr(C)]
#[doc(hidden)]
pub struct IIsolatedAppLauncher_Vtbl {
    pub base: ::windows::core::IUnknownVtbl,
    #[cfg(feature = "Win32_Foundation")]
    pub Launch: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, appusermodelid: ::windows::core::PCWSTR, arguments: ::windows::core::PCWSTR, telemetryparameters: *const IsolatedAppLauncherTelemetryParameters) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    Launch: usize,
}
#[doc = "*Required features: `\"Win32_Security_Isolation\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn IsProcessInIsolatedContainer() -> ::windows::core::Result<super::super::Foundation::BOOL> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn IsProcessInIsolatedContainer(isprocessinisolatedcontainer: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT;
        }
        let mut result__: super::super::Foundation::BOOL = ::core::mem::zeroed();
        IsProcessInIsolatedContainer(::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BOOL>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Security_Isolation\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn IsProcessInIsolatedWindowsEnvironment() -> ::windows::core::Result<super::super::Foundation::BOOL> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn IsProcessInIsolatedWindowsEnvironment(isprocessinisolatedwindowsenvironment: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT;
        }
        let mut result__: super::super::Foundation::BOOL = ::core::mem::zeroed();
        IsProcessInIsolatedWindowsEnvironment(::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BOOL>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_Security_Isolation\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn IsProcessInWDAGContainer(reserved: *const ::core::ffi::c_void) -> ::windows::core::Result<super::super::Foundation::BOOL> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn IsProcessInWDAGContainer(reserved: *const ::core::ffi::c_void, isprocessinwdagcontainer: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT;
        }
        let mut result__: super::super::Foundation::BOOL = ::core::mem::zeroed();
        IsProcessInWDAGContainer(::core::mem::transmute(reserved), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BOOL>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
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
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<IsolatedAppLauncherTelemetryParameters>()) == 0 }
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

#![allow(
    unused_variables,
    non_upper_case_globals,
    non_snake_case,
    unused_unsafe,
    non_camel_case_types,
    dead_code,
    clippy::all
)]
#[cfg(feature = "Win32_Foundation")]
pub unsafe fn CreateAppContainerProfile<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>,
    Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>,
    Param2: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>,
>(
    pszappcontainername: Param0,
    pszdisplayname: Param1,
    pszdescription: Param2,
    pcapabilities: *const super::SID_AND_ATTRIBUTES,
    dwcapabilitycount: u32,
) -> ::windows::runtime::Result<super::super::Foundation::PSID> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CreateAppContainerProfile(
                pszappcontainername: super::super::Foundation::PWSTR,
                pszdisplayname: super::super::Foundation::PWSTR,
                pszdescription: super::super::Foundation::PWSTR,
                pcapabilities: *const super::SID_AND_ATTRIBUTES,
                dwcapabilitycount: u32,
                ppsidappcontainersid: *mut super::super::Foundation::PSID,
            ) -> ::windows::runtime::HRESULT;
        }
        let mut result__: <super::super::Foundation::PSID as ::windows::runtime::Abi>::Abi =
            ::std::mem::zeroed();
        CreateAppContainerProfile(
            pszappcontainername.into_param().abi(),
            pszdisplayname.into_param().abi(),
            pszdescription.into_param().abi(),
            ::std::mem::transmute(pcapabilities),
            ::std::mem::transmute(dwcapabilitycount),
            &mut result__,
        )
        .from_abi::<super::super::Foundation::PSID>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
pub unsafe fn DeleteAppContainerProfile<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>,
>(
    pszappcontainername: Param0,
) -> ::windows::runtime::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn DeleteAppContainerProfile(
                pszappcontainername: super::super::Foundation::PWSTR,
            ) -> ::windows::runtime::HRESULT;
        }
        DeleteAppContainerProfile(pszappcontainername.into_param().abi()).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
pub unsafe fn DeriveAppContainerSidFromAppContainerName<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>,
>(
    pszappcontainername: Param0,
) -> ::windows::runtime::Result<super::super::Foundation::PSID> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn DeriveAppContainerSidFromAppContainerName(
                pszappcontainername: super::super::Foundation::PWSTR,
                ppsidappcontainersid: *mut super::super::Foundation::PSID,
            ) -> ::windows::runtime::HRESULT;
        }
        let mut result__: <super::super::Foundation::PSID as ::windows::runtime::Abi>::Abi =
            ::std::mem::zeroed();
        DeriveAppContainerSidFromAppContainerName(
            pszappcontainername.into_param().abi(),
            &mut result__,
        )
        .from_abi::<super::super::Foundation::PSID>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
pub unsafe fn DeriveRestrictedAppContainerSidFromAppContainerSidAndRestrictedName<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PSID>,
    Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>,
>(
    psidappcontainersid: Param0,
    pszrestrictedappcontainername: Param1,
) -> ::windows::runtime::Result<super::super::Foundation::PSID> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn DeriveRestrictedAppContainerSidFromAppContainerSidAndRestrictedName(
                psidappcontainersid: super::super::Foundation::PSID,
                pszrestrictedappcontainername: super::super::Foundation::PWSTR,
                ppsidrestrictedappcontainersid: *mut super::super::Foundation::PSID,
            ) -> ::windows::runtime::HRESULT;
        }
        let mut result__: <super::super::Foundation::PSID as ::windows::runtime::Abi>::Abi =
            ::std::mem::zeroed();
        DeriveRestrictedAppContainerSidFromAppContainerSidAndRestrictedName(
            psidappcontainersid.into_param().abi(),
            pszrestrictedappcontainername.into_param().abi(),
            &mut result__,
        )
        .from_abi::<super::super::Foundation::PSID>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
pub unsafe fn GetAppContainerFolderPath<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>,
>(
    pszappcontainersid: Param0,
) -> ::windows::runtime::Result<super::super::Foundation::PWSTR> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn GetAppContainerFolderPath(
                pszappcontainersid: super::super::Foundation::PWSTR,
                ppszpath: *mut super::super::Foundation::PWSTR,
            ) -> ::windows::runtime::HRESULT;
        }
        let mut result__: <super::super::Foundation::PWSTR as ::windows::runtime::Abi>::Abi =
            ::std::mem::zeroed();
        GetAppContainerFolderPath(pszappcontainersid.into_param().abi(), &mut result__)
            .from_abi::<super::super::Foundation::PWSTR>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn GetAppContainerNamedObjectPath<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::HANDLE>,
    Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::PSID>,
>(
    token: Param0,
    appcontainersid: Param1,
    objectpathlength: u32,
    objectpath: super::super::Foundation::PWSTR,
    returnlength: *mut u32,
) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn GetAppContainerNamedObjectPath(
                token: super::super::Foundation::HANDLE,
                appcontainersid: super::super::Foundation::PSID,
                objectpathlength: u32,
                objectpath: super::super::Foundation::PWSTR,
                returnlength: *mut u32,
            ) -> super::super::Foundation::BOOL;
        }
        ::std::mem::transmute(GetAppContainerNamedObjectPath(
            token.into_param().abi(),
            appcontainersid.into_param().abi(),
            ::std::mem::transmute(objectpathlength),
            ::std::mem::transmute(objectpath),
            ::std::mem::transmute(returnlength),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_System_Registry")]
pub unsafe fn GetAppContainerRegistryLocation(
    desiredaccess: u32,
) -> ::windows::runtime::Result<super::super::System::Registry::HKEY> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn GetAppContainerRegistryLocation(
                desiredaccess: u32,
                phappcontainerkey: *mut super::super::System::Registry::HKEY,
            ) -> ::windows::runtime::HRESULT;
        }
        let mut result__: <super::super::System::Registry::HKEY as ::windows::runtime::Abi>::Abi =
            ::std::mem::zeroed();
        GetAppContainerRegistryLocation(::std::mem::transmute(desiredaccess), &mut result__)
            .from_abi::<super::super::System::Registry::HKEY>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[repr(transparent)]
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: clone :: Clone,
    :: std :: fmt :: Debug,
)]
pub struct IIsolatedAppLauncher(::windows::runtime::IUnknown);
impl IIsolatedAppLauncher {
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Launch<
        'a,
        Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>,
        Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>,
    >(
        &self,
        appusermodelid: Param0,
        arguments: Param1,
        telemetryparameters: *const IsolatedAppLauncherTelemetryParameters,
    ) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(
            ::std::mem::transmute_copy(self),
            appusermodelid.into_param().abi(),
            arguments.into_param().abi(),
            ::std::mem::transmute(telemetryparameters),
        )
        .ok()
    }
}
unsafe impl ::windows::runtime::Interface for IIsolatedAppLauncher {
    type Vtable = IIsolatedAppLauncher_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
        4136011663,
        31554,
        19652,
        [150, 251, 244, 243, 182, 227, 210, 77],
    );
}
impl ::std::convert::From<IIsolatedAppLauncher> for ::windows::runtime::IUnknown {
    fn from(value: IIsolatedAppLauncher) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IIsolatedAppLauncher> for ::windows::runtime::IUnknown {
    fn from(value: &IIsolatedAppLauncher) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IIsolatedAppLauncher {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(self),
        )
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &IIsolatedAppLauncher {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(
            ::std::convert::Into::<::windows::runtime::IUnknown>::into(::std::clone::Clone::clone(
                self,
            )),
        )
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IIsolatedAppLauncher_abi(
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        iid: &::windows::runtime::GUID,
        interface: *mut ::windows::runtime::RawPtr,
    ) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    #[cfg(feature = "Win32_Foundation")]
    pub  unsafe extern "system" fn(
        this: ::windows::runtime::RawPtr,
        appusermodelid: super::super::Foundation::PWSTR,
        arguments: super::super::Foundation::PWSTR,
        telemetryparameters: *const IsolatedAppLauncherTelemetryParameters,
    ) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
);
#[cfg(feature = "Win32_Foundation")]
pub unsafe fn IsProcessInIsolatedContainer(
) -> ::windows::runtime::Result<super::super::Foundation::BOOL> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn IsProcessInIsolatedContainer(
                isprocessinisolatedcontainer: *mut super::super::Foundation::BOOL,
            ) -> ::windows::runtime::HRESULT;
        }
        let mut result__: <super::super::Foundation::BOOL as ::windows::runtime::Abi>::Abi =
            ::std::mem::zeroed();
        IsProcessInIsolatedContainer(&mut result__)
            .from_abi::<super::super::Foundation::BOOL>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
pub unsafe fn IsProcessInIsolatedWindowsEnvironment(
) -> ::windows::runtime::Result<super::super::Foundation::BOOL> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn IsProcessInIsolatedWindowsEnvironment(
                isprocessinisolatedwindowsenvironment: *mut super::super::Foundation::BOOL,
            ) -> ::windows::runtime::HRESULT;
        }
        let mut result__: <super::super::Foundation::BOOL as ::windows::runtime::Abi>::Abi =
            ::std::mem::zeroed();
        IsProcessInIsolatedWindowsEnvironment(&mut result__)
            .from_abi::<super::super::Foundation::BOOL>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
pub unsafe fn IsProcessInWDAGContainer(
    reserved: *const ::std::ffi::c_void,
) -> ::windows::runtime::Result<super::super::Foundation::BOOL> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn IsProcessInWDAGContainer(
                reserved: *const ::std::ffi::c_void,
                isprocessinwdagcontainer: *mut super::super::Foundation::BOOL,
            ) -> ::windows::runtime::HRESULT;
        }
        let mut result__: <super::super::Foundation::BOOL as ::windows::runtime::Abi>::Abi =
            ::std::mem::zeroed();
        IsProcessInWDAGContainer(::std::mem::transmute(reserved), &mut result__)
            .from_abi::<super::super::Foundation::BOOL>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
pub const IsolatedAppLauncher: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(
    3162580016,
    59230,
    20433,
    [150, 65, 31, 159, 30, 45, 154, 31],
);
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct IsolatedAppLauncherTelemetryParameters {
    pub EnableForLaunch: super::super::Foundation::BOOL,
    pub CorrelationGUID: ::windows::runtime::GUID,
}
#[cfg(feature = "Win32_Foundation")]
impl IsolatedAppLauncherTelemetryParameters {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for IsolatedAppLauncherTelemetryParameters {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::fmt::Debug for IsolatedAppLauncherTelemetryParameters {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("IsolatedAppLauncherTelemetryParameters")
            .field("EnableForLaunch", &self.EnableForLaunch)
            .field("CorrelationGUID", &self.CorrelationGUID)
            .finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for IsolatedAppLauncherTelemetryParameters {
    fn eq(&self, other: &Self) -> bool {
        self.EnableForLaunch == other.EnableForLaunch
            && self.CorrelationGUID == other.CorrelationGUID
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for IsolatedAppLauncherTelemetryParameters {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for IsolatedAppLauncherTelemetryParameters {
    type Abi = Self;
    type DefaultType = Self;
}

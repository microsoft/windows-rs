#![allow(unused_variables, non_upper_case_globals, non_snake_case, unused_unsafe, non_camel_case_types, dead_code, clippy::all)]
pub const CUSTOM_SYSTEM_STATE_CHANGE_EVENT_GUID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(762980374, 3166, 17916, [156, 231, 87, 14, 94, 205, 233, 201]);
#[doc = "*Required features: `Win32_System_Services`, `Win32_Foundation`, `Win32_Security`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
#[inline]
pub unsafe fn ChangeServiceConfig2A<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Security::SC_HANDLE>>(hservice: Param0, dwinfolevel: SERVICE_CONFIG, lpinfo: *const ::std::ffi::c_void) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn ChangeServiceConfig2A(hservice: super::super::Security::SC_HANDLE, dwinfolevel: SERVICE_CONFIG, lpinfo: *const ::std::ffi::c_void) -> super::super::Foundation::BOOL;
        }
        ::std::mem::transmute(ChangeServiceConfig2A(hservice.into_param().abi(), ::std::mem::transmute(dwinfolevel), ::std::mem::transmute(lpinfo)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_System_Services`, `Win32_Foundation`, `Win32_Security`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
#[inline]
pub unsafe fn ChangeServiceConfig2W<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Security::SC_HANDLE>>(hservice: Param0, dwinfolevel: SERVICE_CONFIG, lpinfo: *const ::std::ffi::c_void) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn ChangeServiceConfig2W(hservice: super::super::Security::SC_HANDLE, dwinfolevel: SERVICE_CONFIG, lpinfo: *const ::std::ffi::c_void) -> super::super::Foundation::BOOL;
        }
        ::std::mem::transmute(ChangeServiceConfig2W(hservice.into_param().abi(), ::std::mem::transmute(dwinfolevel), ::std::mem::transmute(lpinfo)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_System_Services`, `Win32_Foundation`, `Win32_Security`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
#[inline]
pub unsafe fn ChangeServiceConfigA<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::super::Security::SC_HANDLE>,
    Param4: ::windows::runtime::IntoParam<'a, super::super::Foundation::PSTR>,
    Param5: ::windows::runtime::IntoParam<'a, super::super::Foundation::PSTR>,
    Param7: ::windows::runtime::IntoParam<'a, super::super::Foundation::PSTR>,
    Param8: ::windows::runtime::IntoParam<'a, super::super::Foundation::PSTR>,
    Param9: ::windows::runtime::IntoParam<'a, super::super::Foundation::PSTR>,
    Param10: ::windows::runtime::IntoParam<'a, super::super::Foundation::PSTR>,
>(
    hservice: Param0,
    dwservicetype: u32,
    dwstarttype: SERVICE_START_TYPE,
    dwerrorcontrol: SERVICE_ERROR,
    lpbinarypathname: Param4,
    lploadordergroup: Param5,
    lpdwtagid: *mut u32,
    lpdependencies: Param7,
    lpservicestartname: Param8,
    lppassword: Param9,
    lpdisplayname: Param10,
) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn ChangeServiceConfigA(
                hservice: super::super::Security::SC_HANDLE,
                dwservicetype: u32,
                dwstarttype: SERVICE_START_TYPE,
                dwerrorcontrol: SERVICE_ERROR,
                lpbinarypathname: super::super::Foundation::PSTR,
                lploadordergroup: super::super::Foundation::PSTR,
                lpdwtagid: *mut u32,
                lpdependencies: super::super::Foundation::PSTR,
                lpservicestartname: super::super::Foundation::PSTR,
                lppassword: super::super::Foundation::PSTR,
                lpdisplayname: super::super::Foundation::PSTR,
            ) -> super::super::Foundation::BOOL;
        }
        ::std::mem::transmute(ChangeServiceConfigA(
            hservice.into_param().abi(),
            ::std::mem::transmute(dwservicetype),
            ::std::mem::transmute(dwstarttype),
            ::std::mem::transmute(dwerrorcontrol),
            lpbinarypathname.into_param().abi(),
            lploadordergroup.into_param().abi(),
            ::std::mem::transmute(lpdwtagid),
            lpdependencies.into_param().abi(),
            lpservicestartname.into_param().abi(),
            lppassword.into_param().abi(),
            lpdisplayname.into_param().abi(),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_System_Services`, `Win32_Foundation`, `Win32_Security`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
#[inline]
pub unsafe fn ChangeServiceConfigW<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::super::Security::SC_HANDLE>,
    Param4: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>,
    Param5: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>,
    Param7: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>,
    Param8: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>,
    Param9: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>,
    Param10: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>,
>(
    hservice: Param0,
    dwservicetype: u32,
    dwstarttype: SERVICE_START_TYPE,
    dwerrorcontrol: SERVICE_ERROR,
    lpbinarypathname: Param4,
    lploadordergroup: Param5,
    lpdwtagid: *mut u32,
    lpdependencies: Param7,
    lpservicestartname: Param8,
    lppassword: Param9,
    lpdisplayname: Param10,
) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn ChangeServiceConfigW(
                hservice: super::super::Security::SC_HANDLE,
                dwservicetype: u32,
                dwstarttype: SERVICE_START_TYPE,
                dwerrorcontrol: SERVICE_ERROR,
                lpbinarypathname: super::super::Foundation::PWSTR,
                lploadordergroup: super::super::Foundation::PWSTR,
                lpdwtagid: *mut u32,
                lpdependencies: super::super::Foundation::PWSTR,
                lpservicestartname: super::super::Foundation::PWSTR,
                lppassword: super::super::Foundation::PWSTR,
                lpdisplayname: super::super::Foundation::PWSTR,
            ) -> super::super::Foundation::BOOL;
        }
        ::std::mem::transmute(ChangeServiceConfigW(
            hservice.into_param().abi(),
            ::std::mem::transmute(dwservicetype),
            ::std::mem::transmute(dwstarttype),
            ::std::mem::transmute(dwerrorcontrol),
            lpbinarypathname.into_param().abi(),
            lploadordergroup.into_param().abi(),
            ::std::mem::transmute(lpdwtagid),
            lpdependencies.into_param().abi(),
            lpservicestartname.into_param().abi(),
            lppassword.into_param().abi(),
            lpdisplayname.into_param().abi(),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_System_Services`, `Win32_Foundation`, `Win32_Security`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
#[inline]
pub unsafe fn CloseServiceHandle<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Security::SC_HANDLE>>(hscobject: Param0) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CloseServiceHandle(hscobject: super::super::Security::SC_HANDLE) -> super::super::Foundation::BOOL;
        }
        ::std::mem::transmute(CloseServiceHandle(hscobject.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_System_Services`, `Win32_Foundation`, `Win32_Security`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
#[inline]
pub unsafe fn ControlService<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Security::SC_HANDLE>>(hservice: Param0, dwcontrol: u32, lpservicestatus: *mut SERVICE_STATUS) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn ControlService(hservice: super::super::Security::SC_HANDLE, dwcontrol: u32, lpservicestatus: *mut SERVICE_STATUS) -> super::super::Foundation::BOOL;
        }
        ::std::mem::transmute(ControlService(hservice.into_param().abi(), ::std::mem::transmute(dwcontrol), ::std::mem::transmute(lpservicestatus)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_System_Services`, `Win32_Foundation`, `Win32_Security`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
#[inline]
pub unsafe fn ControlServiceExA<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Security::SC_HANDLE>>(hservice: Param0, dwcontrol: u32, dwinfolevel: u32, pcontrolparams: *mut ::std::ffi::c_void) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn ControlServiceExA(hservice: super::super::Security::SC_HANDLE, dwcontrol: u32, dwinfolevel: u32, pcontrolparams: *mut ::std::ffi::c_void) -> super::super::Foundation::BOOL;
        }
        ::std::mem::transmute(ControlServiceExA(hservice.into_param().abi(), ::std::mem::transmute(dwcontrol), ::std::mem::transmute(dwinfolevel), ::std::mem::transmute(pcontrolparams)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_System_Services`, `Win32_Foundation`, `Win32_Security`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
#[inline]
pub unsafe fn ControlServiceExW<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Security::SC_HANDLE>>(hservice: Param0, dwcontrol: u32, dwinfolevel: u32, pcontrolparams: *mut ::std::ffi::c_void) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn ControlServiceExW(hservice: super::super::Security::SC_HANDLE, dwcontrol: u32, dwinfolevel: u32, pcontrolparams: *mut ::std::ffi::c_void) -> super::super::Foundation::BOOL;
        }
        ::std::mem::transmute(ControlServiceExW(hservice.into_param().abi(), ::std::mem::transmute(dwcontrol), ::std::mem::transmute(dwinfolevel), ::std::mem::transmute(pcontrolparams)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_System_Services`, `Win32_Foundation`, `Win32_Security`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
#[inline]
pub unsafe fn CreateServiceA<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::super::Security::SC_HANDLE>,
    Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::PSTR>,
    Param2: ::windows::runtime::IntoParam<'a, super::super::Foundation::PSTR>,
    Param7: ::windows::runtime::IntoParam<'a, super::super::Foundation::PSTR>,
    Param8: ::windows::runtime::IntoParam<'a, super::super::Foundation::PSTR>,
    Param10: ::windows::runtime::IntoParam<'a, super::super::Foundation::PSTR>,
    Param11: ::windows::runtime::IntoParam<'a, super::super::Foundation::PSTR>,
    Param12: ::windows::runtime::IntoParam<'a, super::super::Foundation::PSTR>,
>(
    hscmanager: Param0,
    lpservicename: Param1,
    lpdisplayname: Param2,
    dwdesiredaccess: u32,
    dwservicetype: ENUM_SERVICE_TYPE,
    dwstarttype: SERVICE_START_TYPE,
    dwerrorcontrol: SERVICE_ERROR,
    lpbinarypathname: Param7,
    lploadordergroup: Param8,
    lpdwtagid: *mut u32,
    lpdependencies: Param10,
    lpservicestartname: Param11,
    lppassword: Param12,
) -> super::super::Security::SC_HANDLE {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CreateServiceA(
                hscmanager: super::super::Security::SC_HANDLE,
                lpservicename: super::super::Foundation::PSTR,
                lpdisplayname: super::super::Foundation::PSTR,
                dwdesiredaccess: u32,
                dwservicetype: ENUM_SERVICE_TYPE,
                dwstarttype: SERVICE_START_TYPE,
                dwerrorcontrol: SERVICE_ERROR,
                lpbinarypathname: super::super::Foundation::PSTR,
                lploadordergroup: super::super::Foundation::PSTR,
                lpdwtagid: *mut u32,
                lpdependencies: super::super::Foundation::PSTR,
                lpservicestartname: super::super::Foundation::PSTR,
                lppassword: super::super::Foundation::PSTR,
            ) -> super::super::Security::SC_HANDLE;
        }
        ::std::mem::transmute(CreateServiceA(
            hscmanager.into_param().abi(),
            lpservicename.into_param().abi(),
            lpdisplayname.into_param().abi(),
            ::std::mem::transmute(dwdesiredaccess),
            ::std::mem::transmute(dwservicetype),
            ::std::mem::transmute(dwstarttype),
            ::std::mem::transmute(dwerrorcontrol),
            lpbinarypathname.into_param().abi(),
            lploadordergroup.into_param().abi(),
            ::std::mem::transmute(lpdwtagid),
            lpdependencies.into_param().abi(),
            lpservicestartname.into_param().abi(),
            lppassword.into_param().abi(),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_System_Services`, `Win32_Foundation`, `Win32_Security`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
#[inline]
pub unsafe fn CreateServiceW<
    'a,
    Param0: ::windows::runtime::IntoParam<'a, super::super::Security::SC_HANDLE>,
    Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>,
    Param2: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>,
    Param7: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>,
    Param8: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>,
    Param10: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>,
    Param11: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>,
    Param12: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>,
>(
    hscmanager: Param0,
    lpservicename: Param1,
    lpdisplayname: Param2,
    dwdesiredaccess: u32,
    dwservicetype: ENUM_SERVICE_TYPE,
    dwstarttype: SERVICE_START_TYPE,
    dwerrorcontrol: SERVICE_ERROR,
    lpbinarypathname: Param7,
    lploadordergroup: Param8,
    lpdwtagid: *mut u32,
    lpdependencies: Param10,
    lpservicestartname: Param11,
    lppassword: Param12,
) -> super::super::Security::SC_HANDLE {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CreateServiceW(
                hscmanager: super::super::Security::SC_HANDLE,
                lpservicename: super::super::Foundation::PWSTR,
                lpdisplayname: super::super::Foundation::PWSTR,
                dwdesiredaccess: u32,
                dwservicetype: ENUM_SERVICE_TYPE,
                dwstarttype: SERVICE_START_TYPE,
                dwerrorcontrol: SERVICE_ERROR,
                lpbinarypathname: super::super::Foundation::PWSTR,
                lploadordergroup: super::super::Foundation::PWSTR,
                lpdwtagid: *mut u32,
                lpdependencies: super::super::Foundation::PWSTR,
                lpservicestartname: super::super::Foundation::PWSTR,
                lppassword: super::super::Foundation::PWSTR,
            ) -> super::super::Security::SC_HANDLE;
        }
        ::std::mem::transmute(CreateServiceW(
            hscmanager.into_param().abi(),
            lpservicename.into_param().abi(),
            lpdisplayname.into_param().abi(),
            ::std::mem::transmute(dwdesiredaccess),
            ::std::mem::transmute(dwservicetype),
            ::std::mem::transmute(dwstarttype),
            ::std::mem::transmute(dwerrorcontrol),
            lpbinarypathname.into_param().abi(),
            lploadordergroup.into_param().abi(),
            ::std::mem::transmute(lpdwtagid),
            lpdependencies.into_param().abi(),
            lpservicestartname.into_param().abi(),
            lppassword.into_param().abi(),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
pub const DOMAIN_JOIN_GUID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(484575930, 38993, 17441, [148, 48, 29, 222, 183, 102, 232, 9]);
pub const DOMAIN_LEAVE_GUID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3719254382, 22722, 18534, [149, 116, 195, 182, 21, 212, 46, 161]);
#[doc = "*Required features: `Win32_System_Services`, `Win32_Foundation`, `Win32_Security`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
#[inline]
pub unsafe fn DeleteService<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Security::SC_HANDLE>>(hservice: Param0) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn DeleteService(hservice: super::super::Security::SC_HANDLE) -> super::super::Foundation::BOOL;
        }
        ::std::mem::transmute(DeleteService(hservice.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_System_Services`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct ENUM_SERVICE_STATE(pub u32);
pub const SERVICE_ACTIVE: ENUM_SERVICE_STATE = ENUM_SERVICE_STATE(1u32);
pub const SERVICE_INACTIVE: ENUM_SERVICE_STATE = ENUM_SERVICE_STATE(2u32);
pub const SERVICE_STATE_ALL: ENUM_SERVICE_STATE = ENUM_SERVICE_STATE(3u32);
impl ::std::convert::From<u32> for ENUM_SERVICE_STATE {
    fn from(value: u32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for ENUM_SERVICE_STATE {
    type Abi = Self;
}
impl ::std::ops::BitOr for ENUM_SERVICE_STATE {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self {
        Self(self.0 | rhs.0)
    }
}
impl ::std::ops::BitAnd for ENUM_SERVICE_STATE {
    type Output = Self;
    fn bitand(self, rhs: Self) -> Self {
        Self(self.0 & rhs.0)
    }
}
impl ::std::ops::BitOrAssign for ENUM_SERVICE_STATE {
    fn bitor_assign(&mut self, rhs: Self) {
        self.0.bitor_assign(rhs.0)
    }
}
impl ::std::ops::BitAndAssign for ENUM_SERVICE_STATE {
    fn bitand_assign(&mut self, rhs: Self) {
        self.0.bitand_assign(rhs.0)
    }
}
impl ::std::ops::Not for ENUM_SERVICE_STATE {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
#[doc = "*Required features: `Win32_System_Services`, `Win32_Foundation`*"]
pub struct ENUM_SERVICE_STATUSA {
    pub lpServiceName: super::super::Foundation::PSTR,
    pub lpDisplayName: super::super::Foundation::PSTR,
    pub ServiceStatus: SERVICE_STATUS,
}
#[cfg(feature = "Win32_Foundation")]
impl ENUM_SERVICE_STATUSA {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for ENUM_SERVICE_STATUSA {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::fmt::Debug for ENUM_SERVICE_STATUSA {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("ENUM_SERVICE_STATUSA").field("lpServiceName", &self.lpServiceName).field("lpDisplayName", &self.lpDisplayName).field("ServiceStatus", &self.ServiceStatus).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for ENUM_SERVICE_STATUSA {
    fn eq(&self, other: &Self) -> bool {
        self.lpServiceName == other.lpServiceName && self.lpDisplayName == other.lpDisplayName && self.ServiceStatus == other.ServiceStatus
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for ENUM_SERVICE_STATUSA {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for ENUM_SERVICE_STATUSA {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
#[doc = "*Required features: `Win32_System_Services`, `Win32_Foundation`*"]
pub struct ENUM_SERVICE_STATUSW {
    pub lpServiceName: super::super::Foundation::PWSTR,
    pub lpDisplayName: super::super::Foundation::PWSTR,
    pub ServiceStatus: SERVICE_STATUS,
}
#[cfg(feature = "Win32_Foundation")]
impl ENUM_SERVICE_STATUSW {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for ENUM_SERVICE_STATUSW {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::fmt::Debug for ENUM_SERVICE_STATUSW {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("ENUM_SERVICE_STATUSW").field("lpServiceName", &self.lpServiceName).field("lpDisplayName", &self.lpDisplayName).field("ServiceStatus", &self.ServiceStatus).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for ENUM_SERVICE_STATUSW {
    fn eq(&self, other: &Self) -> bool {
        self.lpServiceName == other.lpServiceName && self.lpDisplayName == other.lpDisplayName && self.ServiceStatus == other.ServiceStatus
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for ENUM_SERVICE_STATUSW {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for ENUM_SERVICE_STATUSW {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
#[doc = "*Required features: `Win32_System_Services`, `Win32_Foundation`*"]
pub struct ENUM_SERVICE_STATUS_PROCESSA {
    pub lpServiceName: super::super::Foundation::PSTR,
    pub lpDisplayName: super::super::Foundation::PSTR,
    pub ServiceStatusProcess: SERVICE_STATUS_PROCESS,
}
#[cfg(feature = "Win32_Foundation")]
impl ENUM_SERVICE_STATUS_PROCESSA {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for ENUM_SERVICE_STATUS_PROCESSA {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::fmt::Debug for ENUM_SERVICE_STATUS_PROCESSA {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("ENUM_SERVICE_STATUS_PROCESSA").field("lpServiceName", &self.lpServiceName).field("lpDisplayName", &self.lpDisplayName).field("ServiceStatusProcess", &self.ServiceStatusProcess).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for ENUM_SERVICE_STATUS_PROCESSA {
    fn eq(&self, other: &Self) -> bool {
        self.lpServiceName == other.lpServiceName && self.lpDisplayName == other.lpDisplayName && self.ServiceStatusProcess == other.ServiceStatusProcess
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for ENUM_SERVICE_STATUS_PROCESSA {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for ENUM_SERVICE_STATUS_PROCESSA {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
#[doc = "*Required features: `Win32_System_Services`, `Win32_Foundation`*"]
pub struct ENUM_SERVICE_STATUS_PROCESSW {
    pub lpServiceName: super::super::Foundation::PWSTR,
    pub lpDisplayName: super::super::Foundation::PWSTR,
    pub ServiceStatusProcess: SERVICE_STATUS_PROCESS,
}
#[cfg(feature = "Win32_Foundation")]
impl ENUM_SERVICE_STATUS_PROCESSW {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for ENUM_SERVICE_STATUS_PROCESSW {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::fmt::Debug for ENUM_SERVICE_STATUS_PROCESSW {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("ENUM_SERVICE_STATUS_PROCESSW").field("lpServiceName", &self.lpServiceName).field("lpDisplayName", &self.lpDisplayName).field("ServiceStatusProcess", &self.ServiceStatusProcess).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for ENUM_SERVICE_STATUS_PROCESSW {
    fn eq(&self, other: &Self) -> bool {
        self.lpServiceName == other.lpServiceName && self.lpDisplayName == other.lpDisplayName && self.ServiceStatusProcess == other.ServiceStatusProcess
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for ENUM_SERVICE_STATUS_PROCESSW {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for ENUM_SERVICE_STATUS_PROCESSW {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_System_Services`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct ENUM_SERVICE_TYPE(pub u32);
pub const SERVICE_DRIVER: ENUM_SERVICE_TYPE = ENUM_SERVICE_TYPE(11u32);
pub const SERVICE_FILE_SYSTEM_DRIVER_: ENUM_SERVICE_TYPE = ENUM_SERVICE_TYPE(2u32);
pub const SERVICE_KERNEL_DRIVER: ENUM_SERVICE_TYPE = ENUM_SERVICE_TYPE(1u32);
pub const SERVICE_WIN32: ENUM_SERVICE_TYPE = ENUM_SERVICE_TYPE(48u32);
pub const SERVICE_WIN32_OWN_PROCESS_: ENUM_SERVICE_TYPE = ENUM_SERVICE_TYPE(16u32);
pub const SERVICE_WIN32_SHARE_PROCESS: ENUM_SERVICE_TYPE = ENUM_SERVICE_TYPE(32u32);
pub const SERVICE_ADAPTER: ENUM_SERVICE_TYPE = ENUM_SERVICE_TYPE(4u32);
pub const SERVICE_FILE_SYSTEM_DRIVER: ENUM_SERVICE_TYPE = ENUM_SERVICE_TYPE(2u32);
pub const SERVICE_RECOGNIZER_DRIVER: ENUM_SERVICE_TYPE = ENUM_SERVICE_TYPE(8u32);
pub const SERVICE_WIN32_OWN_PROCESS: ENUM_SERVICE_TYPE = ENUM_SERVICE_TYPE(16u32);
pub const SERVICE_USER_OWN_PROCESS: ENUM_SERVICE_TYPE = ENUM_SERVICE_TYPE(80u32);
pub const SERVICE_USER_SHARE_PROCESS: ENUM_SERVICE_TYPE = ENUM_SERVICE_TYPE(96u32);
impl ::std::convert::From<u32> for ENUM_SERVICE_TYPE {
    fn from(value: u32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for ENUM_SERVICE_TYPE {
    type Abi = Self;
}
impl ::std::ops::BitOr for ENUM_SERVICE_TYPE {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self {
        Self(self.0 | rhs.0)
    }
}
impl ::std::ops::BitAnd for ENUM_SERVICE_TYPE {
    type Output = Self;
    fn bitand(self, rhs: Self) -> Self {
        Self(self.0 & rhs.0)
    }
}
impl ::std::ops::BitOrAssign for ENUM_SERVICE_TYPE {
    fn bitor_assign(&mut self, rhs: Self) {
        self.0.bitor_assign(rhs.0)
    }
}
impl ::std::ops::BitAndAssign for ENUM_SERVICE_TYPE {
    fn bitand_assign(&mut self, rhs: Self) {
        self.0.bitand_assign(rhs.0)
    }
}
impl ::std::ops::Not for ENUM_SERVICE_TYPE {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[doc = "*Required features: `Win32_System_Services`, `Win32_Foundation`, `Win32_Security`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
#[inline]
pub unsafe fn EnumDependentServicesA<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Security::SC_HANDLE>>(hservice: Param0, dwservicestate: ENUM_SERVICE_STATE, lpservices: *mut ENUM_SERVICE_STATUSA, cbbufsize: u32, pcbbytesneeded: *mut u32, lpservicesreturned: *mut u32) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn EnumDependentServicesA(hservice: super::super::Security::SC_HANDLE, dwservicestate: ENUM_SERVICE_STATE, lpservices: *mut ENUM_SERVICE_STATUSA, cbbufsize: u32, pcbbytesneeded: *mut u32, lpservicesreturned: *mut u32) -> super::super::Foundation::BOOL;
        }
        ::std::mem::transmute(EnumDependentServicesA(hservice.into_param().abi(), ::std::mem::transmute(dwservicestate), ::std::mem::transmute(lpservices), ::std::mem::transmute(cbbufsize), ::std::mem::transmute(pcbbytesneeded), ::std::mem::transmute(lpservicesreturned)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_System_Services`, `Win32_Foundation`, `Win32_Security`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
#[inline]
pub unsafe fn EnumDependentServicesW<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Security::SC_HANDLE>>(hservice: Param0, dwservicestate: ENUM_SERVICE_STATE, lpservices: *mut ENUM_SERVICE_STATUSW, cbbufsize: u32, pcbbytesneeded: *mut u32, lpservicesreturned: *mut u32) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn EnumDependentServicesW(hservice: super::super::Security::SC_HANDLE, dwservicestate: ENUM_SERVICE_STATE, lpservices: *mut ENUM_SERVICE_STATUSW, cbbufsize: u32, pcbbytesneeded: *mut u32, lpservicesreturned: *mut u32) -> super::super::Foundation::BOOL;
        }
        ::std::mem::transmute(EnumDependentServicesW(hservice.into_param().abi(), ::std::mem::transmute(dwservicestate), ::std::mem::transmute(lpservices), ::std::mem::transmute(cbbufsize), ::std::mem::transmute(pcbbytesneeded), ::std::mem::transmute(lpservicesreturned)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_System_Services`, `Win32_Foundation`, `Win32_Security`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
#[inline]
pub unsafe fn EnumServicesStatusA<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Security::SC_HANDLE>>(hscmanager: Param0, dwservicetype: ENUM_SERVICE_TYPE, dwservicestate: ENUM_SERVICE_STATE, lpservices: *mut ENUM_SERVICE_STATUSA, cbbufsize: u32, pcbbytesneeded: *mut u32, lpservicesreturned: *mut u32, lpresumehandle: *mut u32) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn EnumServicesStatusA(hscmanager: super::super::Security::SC_HANDLE, dwservicetype: ENUM_SERVICE_TYPE, dwservicestate: ENUM_SERVICE_STATE, lpservices: *mut ENUM_SERVICE_STATUSA, cbbufsize: u32, pcbbytesneeded: *mut u32, lpservicesreturned: *mut u32, lpresumehandle: *mut u32) -> super::super::Foundation::BOOL;
        }
        ::std::mem::transmute(EnumServicesStatusA(
            hscmanager.into_param().abi(),
            ::std::mem::transmute(dwservicetype),
            ::std::mem::transmute(dwservicestate),
            ::std::mem::transmute(lpservices),
            ::std::mem::transmute(cbbufsize),
            ::std::mem::transmute(pcbbytesneeded),
            ::std::mem::transmute(lpservicesreturned),
            ::std::mem::transmute(lpresumehandle),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_System_Services`, `Win32_Foundation`, `Win32_Security`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
#[inline]
pub unsafe fn EnumServicesStatusExA<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Security::SC_HANDLE>, Param9: ::windows::runtime::IntoParam<'a, super::super::Foundation::PSTR>>(hscmanager: Param0, infolevel: SC_ENUM_TYPE, dwservicetype: ENUM_SERVICE_TYPE, dwservicestate: ENUM_SERVICE_STATE, lpservices: *mut u8, cbbufsize: u32, pcbbytesneeded: *mut u32, lpservicesreturned: *mut u32, lpresumehandle: *mut u32, pszgroupname: Param9) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn EnumServicesStatusExA(hscmanager: super::super::Security::SC_HANDLE, infolevel: SC_ENUM_TYPE, dwservicetype: ENUM_SERVICE_TYPE, dwservicestate: ENUM_SERVICE_STATE, lpservices: *mut u8, cbbufsize: u32, pcbbytesneeded: *mut u32, lpservicesreturned: *mut u32, lpresumehandle: *mut u32, pszgroupname: super::super::Foundation::PSTR) -> super::super::Foundation::BOOL;
        }
        ::std::mem::transmute(EnumServicesStatusExA(
            hscmanager.into_param().abi(),
            ::std::mem::transmute(infolevel),
            ::std::mem::transmute(dwservicetype),
            ::std::mem::transmute(dwservicestate),
            ::std::mem::transmute(lpservices),
            ::std::mem::transmute(cbbufsize),
            ::std::mem::transmute(pcbbytesneeded),
            ::std::mem::transmute(lpservicesreturned),
            ::std::mem::transmute(lpresumehandle),
            pszgroupname.into_param().abi(),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_System_Services`, `Win32_Foundation`, `Win32_Security`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
#[inline]
pub unsafe fn EnumServicesStatusExW<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Security::SC_HANDLE>, Param9: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>>(hscmanager: Param0, infolevel: SC_ENUM_TYPE, dwservicetype: ENUM_SERVICE_TYPE, dwservicestate: ENUM_SERVICE_STATE, lpservices: *mut u8, cbbufsize: u32, pcbbytesneeded: *mut u32, lpservicesreturned: *mut u32, lpresumehandle: *mut u32, pszgroupname: Param9) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn EnumServicesStatusExW(hscmanager: super::super::Security::SC_HANDLE, infolevel: SC_ENUM_TYPE, dwservicetype: ENUM_SERVICE_TYPE, dwservicestate: ENUM_SERVICE_STATE, lpservices: *mut u8, cbbufsize: u32, pcbbytesneeded: *mut u32, lpservicesreturned: *mut u32, lpresumehandle: *mut u32, pszgroupname: super::super::Foundation::PWSTR) -> super::super::Foundation::BOOL;
        }
        ::std::mem::transmute(EnumServicesStatusExW(
            hscmanager.into_param().abi(),
            ::std::mem::transmute(infolevel),
            ::std::mem::transmute(dwservicetype),
            ::std::mem::transmute(dwservicestate),
            ::std::mem::transmute(lpservices),
            ::std::mem::transmute(cbbufsize),
            ::std::mem::transmute(pcbbytesneeded),
            ::std::mem::transmute(lpservicesreturned),
            ::std::mem::transmute(lpresumehandle),
            pszgroupname.into_param().abi(),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_System_Services`, `Win32_Foundation`, `Win32_Security`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
#[inline]
pub unsafe fn EnumServicesStatusW<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Security::SC_HANDLE>>(hscmanager: Param0, dwservicetype: ENUM_SERVICE_TYPE, dwservicestate: ENUM_SERVICE_STATE, lpservices: *mut ENUM_SERVICE_STATUSW, cbbufsize: u32, pcbbytesneeded: *mut u32, lpservicesreturned: *mut u32, lpresumehandle: *mut u32) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn EnumServicesStatusW(hscmanager: super::super::Security::SC_HANDLE, dwservicetype: ENUM_SERVICE_TYPE, dwservicestate: ENUM_SERVICE_STATE, lpservices: *mut ENUM_SERVICE_STATUSW, cbbufsize: u32, pcbbytesneeded: *mut u32, lpservicesreturned: *mut u32, lpresumehandle: *mut u32) -> super::super::Foundation::BOOL;
        }
        ::std::mem::transmute(EnumServicesStatusW(
            hscmanager.into_param().abi(),
            ::std::mem::transmute(dwservicetype),
            ::std::mem::transmute(dwservicestate),
            ::std::mem::transmute(lpservices),
            ::std::mem::transmute(cbbufsize),
            ::std::mem::transmute(pcbbytesneeded),
            ::std::mem::transmute(lpservicesreturned),
            ::std::mem::transmute(lpresumehandle),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
pub const FIREWALL_PORT_CLOSE_GUID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2705648952, 36370, 19940, [157, 150, 230, 71, 64, 177, 165, 36]);
pub const FIREWALL_PORT_OPEN_GUID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3075907079, 33825, 20192, [173, 16, 134, 145, 90, 253, 173, 9]);
#[doc = "*Required features: `Win32_System_Services`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn GetServiceDirectory<'a, Param0: ::windows::runtime::IntoParam<'a, SERVICE_STATUS_HANDLE>>(hservicestatus: Param0, edirectorytype: SERVICE_DIRECTORY_TYPE, lppathbuffer: super::super::Foundation::PWSTR, cchpathbufferlength: u32, lpcchrequiredbufferlength: *mut u32) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn GetServiceDirectory(hservicestatus: SERVICE_STATUS_HANDLE, edirectorytype: SERVICE_DIRECTORY_TYPE, lppathbuffer: super::super::Foundation::PWSTR, cchpathbufferlength: u32, lpcchrequiredbufferlength: *mut u32) -> u32;
        }
        ::std::mem::transmute(GetServiceDirectory(hservicestatus.into_param().abi(), ::std::mem::transmute(edirectorytype), ::std::mem::transmute(lppathbuffer), ::std::mem::transmute(cchpathbufferlength), ::std::mem::transmute(lpcchrequiredbufferlength)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_System_Services`, `Win32_Foundation`, `Win32_Security`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
#[inline]
pub unsafe fn GetServiceDisplayNameA<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Security::SC_HANDLE>, Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::PSTR>>(hscmanager: Param0, lpservicename: Param1, lpdisplayname: super::super::Foundation::PSTR, lpcchbuffer: *mut u32) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn GetServiceDisplayNameA(hscmanager: super::super::Security::SC_HANDLE, lpservicename: super::super::Foundation::PSTR, lpdisplayname: super::super::Foundation::PSTR, lpcchbuffer: *mut u32) -> super::super::Foundation::BOOL;
        }
        ::std::mem::transmute(GetServiceDisplayNameA(hscmanager.into_param().abi(), lpservicename.into_param().abi(), ::std::mem::transmute(lpdisplayname), ::std::mem::transmute(lpcchbuffer)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_System_Services`, `Win32_Foundation`, `Win32_Security`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
#[inline]
pub unsafe fn GetServiceDisplayNameW<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Security::SC_HANDLE>, Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>>(hscmanager: Param0, lpservicename: Param1, lpdisplayname: super::super::Foundation::PWSTR, lpcchbuffer: *mut u32) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn GetServiceDisplayNameW(hscmanager: super::super::Security::SC_HANDLE, lpservicename: super::super::Foundation::PWSTR, lpdisplayname: super::super::Foundation::PWSTR, lpcchbuffer: *mut u32) -> super::super::Foundation::BOOL;
        }
        ::std::mem::transmute(GetServiceDisplayNameW(hscmanager.into_param().abi(), lpservicename.into_param().abi(), ::std::mem::transmute(lpdisplayname), ::std::mem::transmute(lpcchbuffer)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_System_Services`, `Win32_Foundation`, `Win32_Security`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
#[inline]
pub unsafe fn GetServiceKeyNameA<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Security::SC_HANDLE>, Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::PSTR>>(hscmanager: Param0, lpdisplayname: Param1, lpservicename: super::super::Foundation::PSTR, lpcchbuffer: *mut u32) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn GetServiceKeyNameA(hscmanager: super::super::Security::SC_HANDLE, lpdisplayname: super::super::Foundation::PSTR, lpservicename: super::super::Foundation::PSTR, lpcchbuffer: *mut u32) -> super::super::Foundation::BOOL;
        }
        ::std::mem::transmute(GetServiceKeyNameA(hscmanager.into_param().abi(), lpdisplayname.into_param().abi(), ::std::mem::transmute(lpservicename), ::std::mem::transmute(lpcchbuffer)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_System_Services`, `Win32_Foundation`, `Win32_Security`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
#[inline]
pub unsafe fn GetServiceKeyNameW<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Security::SC_HANDLE>, Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>>(hscmanager: Param0, lpdisplayname: Param1, lpservicename: super::super::Foundation::PWSTR, lpcchbuffer: *mut u32) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn GetServiceKeyNameW(hscmanager: super::super::Security::SC_HANDLE, lpdisplayname: super::super::Foundation::PWSTR, lpservicename: super::super::Foundation::PWSTR, lpcchbuffer: *mut u32) -> super::super::Foundation::BOOL;
        }
        ::std::mem::transmute(GetServiceKeyNameW(hscmanager.into_param().abi(), lpdisplayname.into_param().abi(), ::std::mem::transmute(lpservicename), ::std::mem::transmute(lpcchbuffer)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_System_Services`, `Win32_System_Registry`*"]
#[cfg(feature = "Win32_System_Registry")]
#[inline]
pub unsafe fn GetServiceRegistryStateKey<'a, Param0: ::windows::runtime::IntoParam<'a, SERVICE_STATUS_HANDLE>>(servicestatushandle: Param0, statetype: SERVICE_REGISTRY_STATE_TYPE, accessmask: u32, servicestatekey: *mut super::Registry::HKEY) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn GetServiceRegistryStateKey(servicestatushandle: SERVICE_STATUS_HANDLE, statetype: SERVICE_REGISTRY_STATE_TYPE, accessmask: u32, servicestatekey: *mut super::Registry::HKEY) -> u32;
        }
        ::std::mem::transmute(GetServiceRegistryStateKey(servicestatushandle.into_param().abi(), ::std::mem::transmute(statetype), ::std::mem::transmute(accessmask), ::std::mem::transmute(servicestatekey)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_System_Services`, `Win32_Foundation`, `Win32_Security`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
#[inline]
pub unsafe fn GetSharedServiceDirectory<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Security::SC_HANDLE>>(servicehandle: Param0, directorytype: SERVICE_SHARED_DIRECTORY_TYPE, pathbuffer: super::super::Foundation::PWSTR, pathbufferlength: u32, requiredbufferlength: *mut u32) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn GetSharedServiceDirectory(servicehandle: super::super::Security::SC_HANDLE, directorytype: SERVICE_SHARED_DIRECTORY_TYPE, pathbuffer: super::super::Foundation::PWSTR, pathbufferlength: u32, requiredbufferlength: *mut u32) -> u32;
        }
        ::std::mem::transmute(GetSharedServiceDirectory(servicehandle.into_param().abi(), ::std::mem::transmute(directorytype), ::std::mem::transmute(pathbuffer), ::std::mem::transmute(pathbufferlength), ::std::mem::transmute(requiredbufferlength)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_System_Services`, `Win32_Security`, `Win32_System_Registry`*"]
#[cfg(all(feature = "Win32_Security", feature = "Win32_System_Registry"))]
#[inline]
pub unsafe fn GetSharedServiceRegistryStateKey<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Security::SC_HANDLE>>(servicehandle: Param0, statetype: SERVICE_SHARED_REGISTRY_STATE_TYPE, accessmask: u32, servicestatekey: *mut super::Registry::HKEY) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn GetSharedServiceRegistryStateKey(servicehandle: super::super::Security::SC_HANDLE, statetype: SERVICE_SHARED_REGISTRY_STATE_TYPE, accessmask: u32, servicestatekey: *mut super::Registry::HKEY) -> u32;
        }
        ::std::mem::transmute(GetSharedServiceRegistryStateKey(servicehandle.into_param().abi(), ::std::mem::transmute(statetype), ::std::mem::transmute(accessmask), ::std::mem::transmute(servicestatekey)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
pub type HANDLER_FUNCTION = unsafe extern "system" fn(dwcontrol: u32);
pub type HANDLER_FUNCTION_EX = unsafe extern "system" fn(dwcontrol: u32, dweventtype: u32, lpeventdata: *mut ::std::ffi::c_void, lpcontext: *mut ::std::ffi::c_void) -> u32;
pub type LPHANDLER_FUNCTION = unsafe extern "system" fn(dwcontrol: u32);
pub type LPHANDLER_FUNCTION_EX = unsafe extern "system" fn(dwcontrol: u32, dweventtype: u32, lpeventdata: *mut ::std::ffi::c_void, lpcontext: *mut ::std::ffi::c_void) -> u32;
#[cfg(feature = "Win32_Foundation")]
pub type LPSERVICE_MAIN_FUNCTIONA = unsafe extern "system" fn(dwnumservicesargs: u32, lpserviceargvectors: *mut super::super::Foundation::PSTR);
#[cfg(feature = "Win32_Foundation")]
pub type LPSERVICE_MAIN_FUNCTIONW = unsafe extern "system" fn(dwnumservicesargs: u32, lpserviceargvectors: *mut super::super::Foundation::PWSTR);
#[doc = "*Required features: `Win32_System_Services`, `Win32_Security`*"]
#[cfg(feature = "Win32_Security")]
#[inline]
pub unsafe fn LockServiceDatabase<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Security::SC_HANDLE>>(hscmanager: Param0) -> *mut ::std::ffi::c_void {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn LockServiceDatabase(hscmanager: super::super::Security::SC_HANDLE) -> *mut ::std::ffi::c_void;
        }
        ::std::mem::transmute(LockServiceDatabase(hscmanager.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
pub const MACHINE_POLICY_PRESENT_GUID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1704970982, 23515, 19881, [177, 255, 202, 42, 23, 141, 70, 224]);
pub const NAMED_PIPE_EVENT_GUID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(528601393, 16300, 17719, [158, 12, 126, 123, 12, 47, 75, 85]);
pub const NETWORK_MANAGER_FIRST_IP_ADDRESS_ARRIVAL_GUID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1328018142, 5346, 17163, [165, 73, 124, 212, 140, 188, 130, 69]);
pub const NETWORK_MANAGER_LAST_IP_ADDRESS_REMOVAL_GUID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3427509802, 5678, 17992, [132, 122, 182, 189, 249, 147, 227, 53]);
#[doc = "*Required features: `Win32_System_Services`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn NotifyBootConfigStatus<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::BOOL>>(bootacceptable: Param0) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn NotifyBootConfigStatus(bootacceptable: super::super::Foundation::BOOL) -> super::super::Foundation::BOOL;
        }
        ::std::mem::transmute(NotifyBootConfigStatus(bootacceptable.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_System_Services`, `Win32_Foundation`, `Win32_Security`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
#[inline]
pub unsafe fn NotifyServiceStatusChangeA<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Security::SC_HANDLE>>(hservice: Param0, dwnotifymask: SERVICE_NOTIFY, pnotifybuffer: *const SERVICE_NOTIFY_2A) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn NotifyServiceStatusChangeA(hservice: super::super::Security::SC_HANDLE, dwnotifymask: SERVICE_NOTIFY, pnotifybuffer: *const ::std::mem::ManuallyDrop<SERVICE_NOTIFY_2A>) -> u32;
        }
        ::std::mem::transmute(NotifyServiceStatusChangeA(hservice.into_param().abi(), ::std::mem::transmute(dwnotifymask), ::std::mem::transmute(pnotifybuffer)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_System_Services`, `Win32_Foundation`, `Win32_Security`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
#[inline]
pub unsafe fn NotifyServiceStatusChangeW<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Security::SC_HANDLE>>(hservice: Param0, dwnotifymask: SERVICE_NOTIFY, pnotifybuffer: *const SERVICE_NOTIFY_2W) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn NotifyServiceStatusChangeW(hservice: super::super::Security::SC_HANDLE, dwnotifymask: SERVICE_NOTIFY, pnotifybuffer: *const ::std::mem::ManuallyDrop<SERVICE_NOTIFY_2W>) -> u32;
        }
        ::std::mem::transmute(NotifyServiceStatusChangeW(hservice.into_param().abi(), ::std::mem::transmute(dwnotifymask), ::std::mem::transmute(pnotifybuffer)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_System_Services`, `Win32_Foundation`, `Win32_Security`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
#[inline]
pub unsafe fn OpenSCManagerA<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PSTR>, Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::PSTR>>(lpmachinename: Param0, lpdatabasename: Param1, dwdesiredaccess: u32) -> super::super::Security::SC_HANDLE {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn OpenSCManagerA(lpmachinename: super::super::Foundation::PSTR, lpdatabasename: super::super::Foundation::PSTR, dwdesiredaccess: u32) -> super::super::Security::SC_HANDLE;
        }
        ::std::mem::transmute(OpenSCManagerA(lpmachinename.into_param().abi(), lpdatabasename.into_param().abi(), ::std::mem::transmute(dwdesiredaccess)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_System_Services`, `Win32_Foundation`, `Win32_Security`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
#[inline]
pub unsafe fn OpenSCManagerW<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>, Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>>(lpmachinename: Param0, lpdatabasename: Param1, dwdesiredaccess: u32) -> super::super::Security::SC_HANDLE {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn OpenSCManagerW(lpmachinename: super::super::Foundation::PWSTR, lpdatabasename: super::super::Foundation::PWSTR, dwdesiredaccess: u32) -> super::super::Security::SC_HANDLE;
        }
        ::std::mem::transmute(OpenSCManagerW(lpmachinename.into_param().abi(), lpdatabasename.into_param().abi(), ::std::mem::transmute(dwdesiredaccess)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_System_Services`, `Win32_Foundation`, `Win32_Security`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
#[inline]
pub unsafe fn OpenServiceA<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Security::SC_HANDLE>, Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::PSTR>>(hscmanager: Param0, lpservicename: Param1, dwdesiredaccess: u32) -> super::super::Security::SC_HANDLE {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn OpenServiceA(hscmanager: super::super::Security::SC_HANDLE, lpservicename: super::super::Foundation::PSTR, dwdesiredaccess: u32) -> super::super::Security::SC_HANDLE;
        }
        ::std::mem::transmute(OpenServiceA(hscmanager.into_param().abi(), lpservicename.into_param().abi(), ::std::mem::transmute(dwdesiredaccess)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_System_Services`, `Win32_Foundation`, `Win32_Security`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
#[inline]
pub unsafe fn OpenServiceW<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Security::SC_HANDLE>, Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>>(hscmanager: Param0, lpservicename: Param1, dwdesiredaccess: u32) -> super::super::Security::SC_HANDLE {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn OpenServiceW(hscmanager: super::super::Security::SC_HANDLE, lpservicename: super::super::Foundation::PWSTR, dwdesiredaccess: u32) -> super::super::Security::SC_HANDLE;
        }
        ::std::mem::transmute(OpenServiceW(hscmanager.into_param().abi(), lpservicename.into_param().abi(), ::std::mem::transmute(dwdesiredaccess)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
pub type PFN_SC_NOTIFY_CALLBACK = unsafe extern "system" fn(pparameter: *const ::std::ffi::c_void);
pub type PSC_NOTIFICATION_CALLBACK = unsafe extern "system" fn(dwnotify: u32, pcallbackcontext: *const ::std::ffi::c_void);
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
#[doc = "*Required features: `Win32_System_Services`, `Win32_Foundation`*"]
pub struct QUERY_SERVICE_CONFIGA {
    pub dwServiceType: ENUM_SERVICE_TYPE,
    pub dwStartType: SERVICE_START_TYPE,
    pub dwErrorControl: SERVICE_ERROR,
    pub lpBinaryPathName: super::super::Foundation::PSTR,
    pub lpLoadOrderGroup: super::super::Foundation::PSTR,
    pub dwTagId: u32,
    pub lpDependencies: super::super::Foundation::PSTR,
    pub lpServiceStartName: super::super::Foundation::PSTR,
    pub lpDisplayName: super::super::Foundation::PSTR,
}
#[cfg(feature = "Win32_Foundation")]
impl QUERY_SERVICE_CONFIGA {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for QUERY_SERVICE_CONFIGA {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::fmt::Debug for QUERY_SERVICE_CONFIGA {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("QUERY_SERVICE_CONFIGA")
            .field("dwServiceType", &self.dwServiceType)
            .field("dwStartType", &self.dwStartType)
            .field("dwErrorControl", &self.dwErrorControl)
            .field("lpBinaryPathName", &self.lpBinaryPathName)
            .field("lpLoadOrderGroup", &self.lpLoadOrderGroup)
            .field("dwTagId", &self.dwTagId)
            .field("lpDependencies", &self.lpDependencies)
            .field("lpServiceStartName", &self.lpServiceStartName)
            .field("lpDisplayName", &self.lpDisplayName)
            .finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for QUERY_SERVICE_CONFIGA {
    fn eq(&self, other: &Self) -> bool {
        self.dwServiceType == other.dwServiceType && self.dwStartType == other.dwStartType && self.dwErrorControl == other.dwErrorControl && self.lpBinaryPathName == other.lpBinaryPathName && self.lpLoadOrderGroup == other.lpLoadOrderGroup && self.dwTagId == other.dwTagId && self.lpDependencies == other.lpDependencies && self.lpServiceStartName == other.lpServiceStartName && self.lpDisplayName == other.lpDisplayName
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for QUERY_SERVICE_CONFIGA {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for QUERY_SERVICE_CONFIGA {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
#[doc = "*Required features: `Win32_System_Services`, `Win32_Foundation`*"]
pub struct QUERY_SERVICE_CONFIGW {
    pub dwServiceType: ENUM_SERVICE_TYPE,
    pub dwStartType: SERVICE_START_TYPE,
    pub dwErrorControl: SERVICE_ERROR,
    pub lpBinaryPathName: super::super::Foundation::PWSTR,
    pub lpLoadOrderGroup: super::super::Foundation::PWSTR,
    pub dwTagId: u32,
    pub lpDependencies: super::super::Foundation::PWSTR,
    pub lpServiceStartName: super::super::Foundation::PWSTR,
    pub lpDisplayName: super::super::Foundation::PWSTR,
}
#[cfg(feature = "Win32_Foundation")]
impl QUERY_SERVICE_CONFIGW {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for QUERY_SERVICE_CONFIGW {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::fmt::Debug for QUERY_SERVICE_CONFIGW {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("QUERY_SERVICE_CONFIGW")
            .field("dwServiceType", &self.dwServiceType)
            .field("dwStartType", &self.dwStartType)
            .field("dwErrorControl", &self.dwErrorControl)
            .field("lpBinaryPathName", &self.lpBinaryPathName)
            .field("lpLoadOrderGroup", &self.lpLoadOrderGroup)
            .field("dwTagId", &self.dwTagId)
            .field("lpDependencies", &self.lpDependencies)
            .field("lpServiceStartName", &self.lpServiceStartName)
            .field("lpDisplayName", &self.lpDisplayName)
            .finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for QUERY_SERVICE_CONFIGW {
    fn eq(&self, other: &Self) -> bool {
        self.dwServiceType == other.dwServiceType && self.dwStartType == other.dwStartType && self.dwErrorControl == other.dwErrorControl && self.lpBinaryPathName == other.lpBinaryPathName && self.lpLoadOrderGroup == other.lpLoadOrderGroup && self.dwTagId == other.dwTagId && self.lpDependencies == other.lpDependencies && self.lpServiceStartName == other.lpServiceStartName && self.lpDisplayName == other.lpDisplayName
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for QUERY_SERVICE_CONFIGW {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for QUERY_SERVICE_CONFIGW {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
#[doc = "*Required features: `Win32_System_Services`, `Win32_Foundation`*"]
pub struct QUERY_SERVICE_LOCK_STATUSA {
    pub fIsLocked: u32,
    pub lpLockOwner: super::super::Foundation::PSTR,
    pub dwLockDuration: u32,
}
#[cfg(feature = "Win32_Foundation")]
impl QUERY_SERVICE_LOCK_STATUSA {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for QUERY_SERVICE_LOCK_STATUSA {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::fmt::Debug for QUERY_SERVICE_LOCK_STATUSA {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("QUERY_SERVICE_LOCK_STATUSA").field("fIsLocked", &self.fIsLocked).field("lpLockOwner", &self.lpLockOwner).field("dwLockDuration", &self.dwLockDuration).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for QUERY_SERVICE_LOCK_STATUSA {
    fn eq(&self, other: &Self) -> bool {
        self.fIsLocked == other.fIsLocked && self.lpLockOwner == other.lpLockOwner && self.dwLockDuration == other.dwLockDuration
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for QUERY_SERVICE_LOCK_STATUSA {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for QUERY_SERVICE_LOCK_STATUSA {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
#[doc = "*Required features: `Win32_System_Services`, `Win32_Foundation`*"]
pub struct QUERY_SERVICE_LOCK_STATUSW {
    pub fIsLocked: u32,
    pub lpLockOwner: super::super::Foundation::PWSTR,
    pub dwLockDuration: u32,
}
#[cfg(feature = "Win32_Foundation")]
impl QUERY_SERVICE_LOCK_STATUSW {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for QUERY_SERVICE_LOCK_STATUSW {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::fmt::Debug for QUERY_SERVICE_LOCK_STATUSW {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("QUERY_SERVICE_LOCK_STATUSW").field("fIsLocked", &self.fIsLocked).field("lpLockOwner", &self.lpLockOwner).field("dwLockDuration", &self.dwLockDuration).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for QUERY_SERVICE_LOCK_STATUSW {
    fn eq(&self, other: &Self) -> bool {
        self.fIsLocked == other.fIsLocked && self.lpLockOwner == other.lpLockOwner && self.dwLockDuration == other.dwLockDuration
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for QUERY_SERVICE_LOCK_STATUSW {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for QUERY_SERVICE_LOCK_STATUSW {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_System_Services`, `Win32_Foundation`, `Win32_Security`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
#[inline]
pub unsafe fn QueryServiceConfig2A<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Security::SC_HANDLE>>(hservice: Param0, dwinfolevel: SERVICE_CONFIG, lpbuffer: *mut u8, cbbufsize: u32, pcbbytesneeded: *mut u32) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn QueryServiceConfig2A(hservice: super::super::Security::SC_HANDLE, dwinfolevel: SERVICE_CONFIG, lpbuffer: *mut u8, cbbufsize: u32, pcbbytesneeded: *mut u32) -> super::super::Foundation::BOOL;
        }
        ::std::mem::transmute(QueryServiceConfig2A(hservice.into_param().abi(), ::std::mem::transmute(dwinfolevel), ::std::mem::transmute(lpbuffer), ::std::mem::transmute(cbbufsize), ::std::mem::transmute(pcbbytesneeded)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_System_Services`, `Win32_Foundation`, `Win32_Security`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
#[inline]
pub unsafe fn QueryServiceConfig2W<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Security::SC_HANDLE>>(hservice: Param0, dwinfolevel: SERVICE_CONFIG, lpbuffer: *mut u8, cbbufsize: u32, pcbbytesneeded: *mut u32) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn QueryServiceConfig2W(hservice: super::super::Security::SC_HANDLE, dwinfolevel: SERVICE_CONFIG, lpbuffer: *mut u8, cbbufsize: u32, pcbbytesneeded: *mut u32) -> super::super::Foundation::BOOL;
        }
        ::std::mem::transmute(QueryServiceConfig2W(hservice.into_param().abi(), ::std::mem::transmute(dwinfolevel), ::std::mem::transmute(lpbuffer), ::std::mem::transmute(cbbufsize), ::std::mem::transmute(pcbbytesneeded)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_System_Services`, `Win32_Foundation`, `Win32_Security`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
#[inline]
pub unsafe fn QueryServiceConfigA<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Security::SC_HANDLE>>(hservice: Param0, lpserviceconfig: *mut QUERY_SERVICE_CONFIGA, cbbufsize: u32, pcbbytesneeded: *mut u32) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn QueryServiceConfigA(hservice: super::super::Security::SC_HANDLE, lpserviceconfig: *mut QUERY_SERVICE_CONFIGA, cbbufsize: u32, pcbbytesneeded: *mut u32) -> super::super::Foundation::BOOL;
        }
        ::std::mem::transmute(QueryServiceConfigA(hservice.into_param().abi(), ::std::mem::transmute(lpserviceconfig), ::std::mem::transmute(cbbufsize), ::std::mem::transmute(pcbbytesneeded)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_System_Services`, `Win32_Foundation`, `Win32_Security`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
#[inline]
pub unsafe fn QueryServiceConfigW<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Security::SC_HANDLE>>(hservice: Param0, lpserviceconfig: *mut QUERY_SERVICE_CONFIGW, cbbufsize: u32, pcbbytesneeded: *mut u32) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn QueryServiceConfigW(hservice: super::super::Security::SC_HANDLE, lpserviceconfig: *mut QUERY_SERVICE_CONFIGW, cbbufsize: u32, pcbbytesneeded: *mut u32) -> super::super::Foundation::BOOL;
        }
        ::std::mem::transmute(QueryServiceConfigW(hservice.into_param().abi(), ::std::mem::transmute(lpserviceconfig), ::std::mem::transmute(cbbufsize), ::std::mem::transmute(pcbbytesneeded)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_System_Services`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn QueryServiceDynamicInformation<'a, Param0: ::windows::runtime::IntoParam<'a, SERVICE_STATUS_HANDLE>>(hservicestatus: Param0, dwinfolevel: u32, ppdynamicinfo: *mut *mut ::std::ffi::c_void) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn QueryServiceDynamicInformation(hservicestatus: SERVICE_STATUS_HANDLE, dwinfolevel: u32, ppdynamicinfo: *mut *mut ::std::ffi::c_void) -> super::super::Foundation::BOOL;
        }
        ::std::mem::transmute(QueryServiceDynamicInformation(hservicestatus.into_param().abi(), ::std::mem::transmute(dwinfolevel), ::std::mem::transmute(ppdynamicinfo)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_System_Services`, `Win32_Foundation`, `Win32_Security`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
#[inline]
pub unsafe fn QueryServiceLockStatusA<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Security::SC_HANDLE>>(hscmanager: Param0, lplockstatus: *mut QUERY_SERVICE_LOCK_STATUSA, cbbufsize: u32, pcbbytesneeded: *mut u32) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn QueryServiceLockStatusA(hscmanager: super::super::Security::SC_HANDLE, lplockstatus: *mut QUERY_SERVICE_LOCK_STATUSA, cbbufsize: u32, pcbbytesneeded: *mut u32) -> super::super::Foundation::BOOL;
        }
        ::std::mem::transmute(QueryServiceLockStatusA(hscmanager.into_param().abi(), ::std::mem::transmute(lplockstatus), ::std::mem::transmute(cbbufsize), ::std::mem::transmute(pcbbytesneeded)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_System_Services`, `Win32_Foundation`, `Win32_Security`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
#[inline]
pub unsafe fn QueryServiceLockStatusW<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Security::SC_HANDLE>>(hscmanager: Param0, lplockstatus: *mut QUERY_SERVICE_LOCK_STATUSW, cbbufsize: u32, pcbbytesneeded: *mut u32) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn QueryServiceLockStatusW(hscmanager: super::super::Security::SC_HANDLE, lplockstatus: *mut QUERY_SERVICE_LOCK_STATUSW, cbbufsize: u32, pcbbytesneeded: *mut u32) -> super::super::Foundation::BOOL;
        }
        ::std::mem::transmute(QueryServiceLockStatusW(hscmanager.into_param().abi(), ::std::mem::transmute(lplockstatus), ::std::mem::transmute(cbbufsize), ::std::mem::transmute(pcbbytesneeded)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_System_Services`, `Win32_Foundation`, `Win32_Security`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
#[inline]
pub unsafe fn QueryServiceObjectSecurity<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Security::SC_HANDLE>>(hservice: Param0, dwsecurityinformation: u32, lpsecuritydescriptor: *mut super::super::Security::SECURITY_DESCRIPTOR, cbbufsize: u32, pcbbytesneeded: *mut u32) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn QueryServiceObjectSecurity(hservice: super::super::Security::SC_HANDLE, dwsecurityinformation: u32, lpsecuritydescriptor: *mut super::super::Security::SECURITY_DESCRIPTOR, cbbufsize: u32, pcbbytesneeded: *mut u32) -> super::super::Foundation::BOOL;
        }
        ::std::mem::transmute(QueryServiceObjectSecurity(hservice.into_param().abi(), ::std::mem::transmute(dwsecurityinformation), ::std::mem::transmute(lpsecuritydescriptor), ::std::mem::transmute(cbbufsize), ::std::mem::transmute(pcbbytesneeded)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_System_Services`, `Win32_Foundation`, `Win32_Security`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
#[inline]
pub unsafe fn QueryServiceStatus<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Security::SC_HANDLE>>(hservice: Param0, lpservicestatus: *mut SERVICE_STATUS) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn QueryServiceStatus(hservice: super::super::Security::SC_HANDLE, lpservicestatus: *mut SERVICE_STATUS) -> super::super::Foundation::BOOL;
        }
        ::std::mem::transmute(QueryServiceStatus(hservice.into_param().abi(), ::std::mem::transmute(lpservicestatus)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_System_Services`, `Win32_Foundation`, `Win32_Security`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
#[inline]
pub unsafe fn QueryServiceStatusEx<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Security::SC_HANDLE>>(hservice: Param0, infolevel: SC_STATUS_TYPE, lpbuffer: *mut u8, cbbufsize: u32, pcbbytesneeded: *mut u32) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn QueryServiceStatusEx(hservice: super::super::Security::SC_HANDLE, infolevel: SC_STATUS_TYPE, lpbuffer: *mut u8, cbbufsize: u32, pcbbytesneeded: *mut u32) -> super::super::Foundation::BOOL;
        }
        ::std::mem::transmute(QueryServiceStatusEx(hservice.into_param().abi(), ::std::mem::transmute(infolevel), ::std::mem::transmute(lpbuffer), ::std::mem::transmute(cbbufsize), ::std::mem::transmute(pcbbytesneeded)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
pub const RPC_INTERFACE_EVENT_GUID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3163607399, 38000, 16697, [169, 186, 190, 11, 187, 245, 183, 77]);
#[doc = "*Required features: `Win32_System_Services`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn RegisterServiceCtrlHandlerA<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PSTR>>(lpservicename: Param0, lphandlerproc: ::std::option::Option<LPHANDLER_FUNCTION>) -> SERVICE_STATUS_HANDLE {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn RegisterServiceCtrlHandlerA(lpservicename: super::super::Foundation::PSTR, lphandlerproc: ::windows::runtime::RawPtr) -> SERVICE_STATUS_HANDLE;
        }
        ::std::mem::transmute(RegisterServiceCtrlHandlerA(lpservicename.into_param().abi(), ::std::mem::transmute(lphandlerproc)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_System_Services`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn RegisterServiceCtrlHandlerExA<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PSTR>>(lpservicename: Param0, lphandlerproc: ::std::option::Option<LPHANDLER_FUNCTION_EX>, lpcontext: *const ::std::ffi::c_void) -> SERVICE_STATUS_HANDLE {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn RegisterServiceCtrlHandlerExA(lpservicename: super::super::Foundation::PSTR, lphandlerproc: ::windows::runtime::RawPtr, lpcontext: *const ::std::ffi::c_void) -> SERVICE_STATUS_HANDLE;
        }
        ::std::mem::transmute(RegisterServiceCtrlHandlerExA(lpservicename.into_param().abi(), ::std::mem::transmute(lphandlerproc), ::std::mem::transmute(lpcontext)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_System_Services`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn RegisterServiceCtrlHandlerExW<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>>(lpservicename: Param0, lphandlerproc: ::std::option::Option<LPHANDLER_FUNCTION_EX>, lpcontext: *const ::std::ffi::c_void) -> SERVICE_STATUS_HANDLE {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn RegisterServiceCtrlHandlerExW(lpservicename: super::super::Foundation::PWSTR, lphandlerproc: ::windows::runtime::RawPtr, lpcontext: *const ::std::ffi::c_void) -> SERVICE_STATUS_HANDLE;
        }
        ::std::mem::transmute(RegisterServiceCtrlHandlerExW(lpservicename.into_param().abi(), ::std::mem::transmute(lphandlerproc), ::std::mem::transmute(lpcontext)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_System_Services`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn RegisterServiceCtrlHandlerW<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>>(lpservicename: Param0, lphandlerproc: ::std::option::Option<LPHANDLER_FUNCTION>) -> SERVICE_STATUS_HANDLE {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn RegisterServiceCtrlHandlerW(lpservicename: super::super::Foundation::PWSTR, lphandlerproc: ::windows::runtime::RawPtr) -> SERVICE_STATUS_HANDLE;
        }
        ::std::mem::transmute(RegisterServiceCtrlHandlerW(lpservicename.into_param().abi(), ::std::mem::transmute(lphandlerproc)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_System_Services`*"]
pub struct SC_ACTION {
    pub Type: SC_ACTION_TYPE,
    pub Delay: u32,
}
impl SC_ACTION {}
impl ::std::default::Default for SC_ACTION {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for SC_ACTION {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("SC_ACTION").field("Type", &self.Type).field("Delay", &self.Delay).finish()
    }
}
impl ::std::cmp::PartialEq for SC_ACTION {
    fn eq(&self, other: &Self) -> bool {
        self.Type == other.Type && self.Delay == other.Delay
    }
}
impl ::std::cmp::Eq for SC_ACTION {}
unsafe impl ::windows::runtime::Abi for SC_ACTION {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_System_Services`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct SC_ACTION_TYPE(pub i32);
pub const SC_ACTION_NONE: SC_ACTION_TYPE = SC_ACTION_TYPE(0i32);
pub const SC_ACTION_RESTART: SC_ACTION_TYPE = SC_ACTION_TYPE(1i32);
pub const SC_ACTION_REBOOT: SC_ACTION_TYPE = SC_ACTION_TYPE(2i32);
pub const SC_ACTION_RUN_COMMAND: SC_ACTION_TYPE = SC_ACTION_TYPE(3i32);
pub const SC_ACTION_OWN_RESTART: SC_ACTION_TYPE = SC_ACTION_TYPE(4i32);
impl ::std::convert::From<i32> for SC_ACTION_TYPE {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for SC_ACTION_TYPE {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_System_Services`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct SC_ENUM_TYPE(pub i32);
pub const SC_ENUM_PROCESS_INFO: SC_ENUM_TYPE = SC_ENUM_TYPE(0i32);
impl ::std::convert::From<i32> for SC_ENUM_TYPE {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for SC_ENUM_TYPE {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_System_Services`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct SC_EVENT_TYPE(pub i32);
pub const SC_EVENT_DATABASE_CHANGE: SC_EVENT_TYPE = SC_EVENT_TYPE(0i32);
pub const SC_EVENT_PROPERTY_CHANGE: SC_EVENT_TYPE = SC_EVENT_TYPE(1i32);
pub const SC_EVENT_STATUS_CHANGE: SC_EVENT_TYPE = SC_EVENT_TYPE(2i32);
impl ::std::convert::From<i32> for SC_EVENT_TYPE {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for SC_EVENT_TYPE {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_System_Services`*"]
pub const SC_MANAGER_ALL_ACCESS: u32 = 983103u32;
#[doc = "*Required features: `Win32_System_Services`*"]
pub const SC_MANAGER_CONNECT: u32 = 1u32;
#[doc = "*Required features: `Win32_System_Services`*"]
pub const SC_MANAGER_CREATE_SERVICE: u32 = 2u32;
#[doc = "*Required features: `Win32_System_Services`*"]
pub const SC_MANAGER_ENUMERATE_SERVICE: u32 = 4u32;
#[doc = "*Required features: `Win32_System_Services`*"]
pub const SC_MANAGER_LOCK: u32 = 8u32;
#[doc = "*Required features: `Win32_System_Services`*"]
pub const SC_MANAGER_MODIFY_BOOT_CONFIG: u32 = 32u32;
#[doc = "*Required features: `Win32_System_Services`*"]
pub const SC_MANAGER_QUERY_LOCK_STATUS: u32 = 16u32;
#[doc = "*Required features: `Win32_System_Services`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct SC_STATUS_TYPE(pub i32);
pub const SC_STATUS_PROCESS_INFO: SC_STATUS_TYPE = SC_STATUS_TYPE(0i32);
impl ::std::convert::From<i32> for SC_STATUS_TYPE {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for SC_STATUS_TYPE {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_System_Services`*"]
pub const SERVICE_ACCEPT_HARDWAREPROFILECHANGE: u32 = 32u32;
#[doc = "*Required features: `Win32_System_Services`*"]
pub const SERVICE_ACCEPT_LOWRESOURCES: u32 = 8192u32;
#[doc = "*Required features: `Win32_System_Services`*"]
pub const SERVICE_ACCEPT_NETBINDCHANGE: u32 = 16u32;
#[doc = "*Required features: `Win32_System_Services`*"]
pub const SERVICE_ACCEPT_PARAMCHANGE: u32 = 8u32;
#[doc = "*Required features: `Win32_System_Services`*"]
pub const SERVICE_ACCEPT_PAUSE_CONTINUE: u32 = 2u32;
#[doc = "*Required features: `Win32_System_Services`*"]
pub const SERVICE_ACCEPT_POWEREVENT: u32 = 64u32;
#[doc = "*Required features: `Win32_System_Services`*"]
pub const SERVICE_ACCEPT_PRESHUTDOWN: u32 = 256u32;
#[doc = "*Required features: `Win32_System_Services`*"]
pub const SERVICE_ACCEPT_SESSIONCHANGE: u32 = 128u32;
#[doc = "*Required features: `Win32_System_Services`*"]
pub const SERVICE_ACCEPT_SHUTDOWN: u32 = 4u32;
#[doc = "*Required features: `Win32_System_Services`*"]
pub const SERVICE_ACCEPT_STOP: u32 = 1u32;
#[doc = "*Required features: `Win32_System_Services`*"]
pub const SERVICE_ACCEPT_SYSTEMLOWRESOURCES: u32 = 16384u32;
#[doc = "*Required features: `Win32_System_Services`*"]
pub const SERVICE_ACCEPT_TIMECHANGE: u32 = 512u32;
#[doc = "*Required features: `Win32_System_Services`*"]
pub const SERVICE_ACCEPT_TRIGGEREVENT: u32 = 1024u32;
#[doc = "*Required features: `Win32_System_Services`*"]
pub const SERVICE_ACCEPT_USER_LOGOFF: u32 = 2048u32;
#[doc = "*Required features: `Win32_System_Services`*"]
pub const SERVICE_ALL_ACCESS: u32 = 983551u32;
#[doc = "*Required features: `Win32_System_Services`*"]
pub const SERVICE_CHANGE_CONFIG: u32 = 2u32;
#[doc = "*Required features: `Win32_System_Services`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct SERVICE_CONFIG(pub u32);
pub const SERVICE_CONFIG_DELAYED_AUTO_START_INFO: SERVICE_CONFIG = SERVICE_CONFIG(3u32);
pub const SERVICE_CONFIG_DESCRIPTION: SERVICE_CONFIG = SERVICE_CONFIG(1u32);
pub const SERVICE_CONFIG_FAILURE_ACTIONS: SERVICE_CONFIG = SERVICE_CONFIG(2u32);
pub const SERVICE_CONFIG_FAILURE_ACTIONS_FLAG: SERVICE_CONFIG = SERVICE_CONFIG(4u32);
pub const SERVICE_CONFIG_PREFERRED_NODE: SERVICE_CONFIG = SERVICE_CONFIG(9u32);
pub const SERVICE_CONFIG_PRESHUTDOWN_INFO: SERVICE_CONFIG = SERVICE_CONFIG(7u32);
pub const SERVICE_CONFIG_REQUIRED_PRIVILEGES_INFO: SERVICE_CONFIG = SERVICE_CONFIG(6u32);
pub const SERVICE_CONFIG_SERVICE_SID_INFO: SERVICE_CONFIG = SERVICE_CONFIG(5u32);
pub const SERVICE_CONFIG_TRIGGER_INFO: SERVICE_CONFIG = SERVICE_CONFIG(8u32);
pub const SERVICE_CONFIG_LAUNCH_PROTECTED: SERVICE_CONFIG = SERVICE_CONFIG(12u32);
impl ::std::convert::From<u32> for SERVICE_CONFIG {
    fn from(value: u32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for SERVICE_CONFIG {
    type Abi = Self;
}
impl ::std::ops::BitOr for SERVICE_CONFIG {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self {
        Self(self.0 | rhs.0)
    }
}
impl ::std::ops::BitAnd for SERVICE_CONFIG {
    type Output = Self;
    fn bitand(self, rhs: Self) -> Self {
        Self(self.0 & rhs.0)
    }
}
impl ::std::ops::BitOrAssign for SERVICE_CONFIG {
    fn bitor_assign(&mut self, rhs: Self) {
        self.0.bitor_assign(rhs.0)
    }
}
impl ::std::ops::BitAndAssign for SERVICE_CONFIG {
    fn bitand_assign(&mut self, rhs: Self) {
        self.0.bitand_assign(rhs.0)
    }
}
impl ::std::ops::Not for SERVICE_CONFIG {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[doc = "*Required features: `Win32_System_Services`*"]
pub const SERVICE_CONTROL_CONTINUE: u32 = 3u32;
#[doc = "*Required features: `Win32_System_Services`*"]
pub const SERVICE_CONTROL_DEVICEEVENT: u32 = 11u32;
#[doc = "*Required features: `Win32_System_Services`*"]
pub const SERVICE_CONTROL_HARDWAREPROFILECHANGE: u32 = 12u32;
#[doc = "*Required features: `Win32_System_Services`*"]
pub const SERVICE_CONTROL_INTERROGATE: u32 = 4u32;
#[doc = "*Required features: `Win32_System_Services`*"]
pub const SERVICE_CONTROL_LOWRESOURCES: u32 = 96u32;
#[doc = "*Required features: `Win32_System_Services`*"]
pub const SERVICE_CONTROL_NETBINDADD: u32 = 7u32;
#[doc = "*Required features: `Win32_System_Services`*"]
pub const SERVICE_CONTROL_NETBINDDISABLE: u32 = 10u32;
#[doc = "*Required features: `Win32_System_Services`*"]
pub const SERVICE_CONTROL_NETBINDENABLE: u32 = 9u32;
#[doc = "*Required features: `Win32_System_Services`*"]
pub const SERVICE_CONTROL_NETBINDREMOVE: u32 = 8u32;
#[doc = "*Required features: `Win32_System_Services`*"]
pub const SERVICE_CONTROL_PARAMCHANGE: u32 = 6u32;
#[doc = "*Required features: `Win32_System_Services`*"]
pub const SERVICE_CONTROL_PAUSE: u32 = 2u32;
#[doc = "*Required features: `Win32_System_Services`*"]
pub const SERVICE_CONTROL_POWEREVENT: u32 = 13u32;
#[doc = "*Required features: `Win32_System_Services`*"]
pub const SERVICE_CONTROL_PRESHUTDOWN: u32 = 15u32;
#[doc = "*Required features: `Win32_System_Services`*"]
pub const SERVICE_CONTROL_SESSIONCHANGE: u32 = 14u32;
#[doc = "*Required features: `Win32_System_Services`*"]
pub const SERVICE_CONTROL_SHUTDOWN: u32 = 5u32;
#[doc = "*Required features: `Win32_System_Services`*"]
pub const SERVICE_CONTROL_STATUS_REASON_INFO: u32 = 1u32;
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
#[doc = "*Required features: `Win32_System_Services`, `Win32_Foundation`*"]
pub struct SERVICE_CONTROL_STATUS_REASON_PARAMSA {
    pub dwReason: u32,
    pub pszComment: super::super::Foundation::PSTR,
    pub ServiceStatus: SERVICE_STATUS_PROCESS,
}
#[cfg(feature = "Win32_Foundation")]
impl SERVICE_CONTROL_STATUS_REASON_PARAMSA {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for SERVICE_CONTROL_STATUS_REASON_PARAMSA {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::fmt::Debug for SERVICE_CONTROL_STATUS_REASON_PARAMSA {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("SERVICE_CONTROL_STATUS_REASON_PARAMSA").field("dwReason", &self.dwReason).field("pszComment", &self.pszComment).field("ServiceStatus", &self.ServiceStatus).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for SERVICE_CONTROL_STATUS_REASON_PARAMSA {
    fn eq(&self, other: &Self) -> bool {
        self.dwReason == other.dwReason && self.pszComment == other.pszComment && self.ServiceStatus == other.ServiceStatus
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for SERVICE_CONTROL_STATUS_REASON_PARAMSA {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for SERVICE_CONTROL_STATUS_REASON_PARAMSA {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
#[doc = "*Required features: `Win32_System_Services`, `Win32_Foundation`*"]
pub struct SERVICE_CONTROL_STATUS_REASON_PARAMSW {
    pub dwReason: u32,
    pub pszComment: super::super::Foundation::PWSTR,
    pub ServiceStatus: SERVICE_STATUS_PROCESS,
}
#[cfg(feature = "Win32_Foundation")]
impl SERVICE_CONTROL_STATUS_REASON_PARAMSW {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for SERVICE_CONTROL_STATUS_REASON_PARAMSW {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::fmt::Debug for SERVICE_CONTROL_STATUS_REASON_PARAMSW {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("SERVICE_CONTROL_STATUS_REASON_PARAMSW").field("dwReason", &self.dwReason).field("pszComment", &self.pszComment).field("ServiceStatus", &self.ServiceStatus).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for SERVICE_CONTROL_STATUS_REASON_PARAMSW {
    fn eq(&self, other: &Self) -> bool {
        self.dwReason == other.dwReason && self.pszComment == other.pszComment && self.ServiceStatus == other.ServiceStatus
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for SERVICE_CONTROL_STATUS_REASON_PARAMSW {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for SERVICE_CONTROL_STATUS_REASON_PARAMSW {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_System_Services`*"]
pub const SERVICE_CONTROL_STOP: u32 = 1u32;
#[doc = "*Required features: `Win32_System_Services`*"]
pub const SERVICE_CONTROL_SYSTEMLOWRESOURCES: u32 = 97u32;
#[doc = "*Required features: `Win32_System_Services`*"]
pub const SERVICE_CONTROL_TIMECHANGE: u32 = 16u32;
#[doc = "*Required features: `Win32_System_Services`*"]
pub const SERVICE_CONTROL_TRIGGEREVENT: u32 = 32u32;
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_System_Services`*"]
pub struct SERVICE_CUSTOM_SYSTEM_STATE_CHANGE_DATA_ITEM {
    pub u: SERVICE_CUSTOM_SYSTEM_STATE_CHANGE_DATA_ITEM_0,
}
impl SERVICE_CUSTOM_SYSTEM_STATE_CHANGE_DATA_ITEM {}
impl ::std::default::Default for SERVICE_CUSTOM_SYSTEM_STATE_CHANGE_DATA_ITEM {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::cmp::PartialEq for SERVICE_CUSTOM_SYSTEM_STATE_CHANGE_DATA_ITEM {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
impl ::std::cmp::Eq for SERVICE_CUSTOM_SYSTEM_STATE_CHANGE_DATA_ITEM {}
unsafe impl ::windows::runtime::Abi for SERVICE_CUSTOM_SYSTEM_STATE_CHANGE_DATA_ITEM {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_System_Services`*"]
pub union SERVICE_CUSTOM_SYSTEM_STATE_CHANGE_DATA_ITEM_0 {
    pub CustomStateId: SERVICE_TRIGGER_CUSTOM_STATE_ID,
    pub s: SERVICE_CUSTOM_SYSTEM_STATE_CHANGE_DATA_ITEM_0_0,
}
impl SERVICE_CUSTOM_SYSTEM_STATE_CHANGE_DATA_ITEM_0 {}
impl ::std::default::Default for SERVICE_CUSTOM_SYSTEM_STATE_CHANGE_DATA_ITEM_0 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::cmp::PartialEq for SERVICE_CUSTOM_SYSTEM_STATE_CHANGE_DATA_ITEM_0 {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
impl ::std::cmp::Eq for SERVICE_CUSTOM_SYSTEM_STATE_CHANGE_DATA_ITEM_0 {}
unsafe impl ::windows::runtime::Abi for SERVICE_CUSTOM_SYSTEM_STATE_CHANGE_DATA_ITEM_0 {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_System_Services`*"]
pub struct SERVICE_CUSTOM_SYSTEM_STATE_CHANGE_DATA_ITEM_0_0 {
    pub DataOffset: u32,
    pub Data: [u8; 1],
}
impl SERVICE_CUSTOM_SYSTEM_STATE_CHANGE_DATA_ITEM_0_0 {}
impl ::std::default::Default for SERVICE_CUSTOM_SYSTEM_STATE_CHANGE_DATA_ITEM_0_0 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for SERVICE_CUSTOM_SYSTEM_STATE_CHANGE_DATA_ITEM_0_0 {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("_s_e__Struct").field("DataOffset", &self.DataOffset).field("Data", &self.Data).finish()
    }
}
impl ::std::cmp::PartialEq for SERVICE_CUSTOM_SYSTEM_STATE_CHANGE_DATA_ITEM_0_0 {
    fn eq(&self, other: &Self) -> bool {
        self.DataOffset == other.DataOffset && self.Data == other.Data
    }
}
impl ::std::cmp::Eq for SERVICE_CUSTOM_SYSTEM_STATE_CHANGE_DATA_ITEM_0_0 {}
unsafe impl ::windows::runtime::Abi for SERVICE_CUSTOM_SYSTEM_STATE_CHANGE_DATA_ITEM_0_0 {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
#[doc = "*Required features: `Win32_System_Services`, `Win32_Foundation`*"]
pub struct SERVICE_DELAYED_AUTO_START_INFO {
    pub fDelayedAutostart: super::super::Foundation::BOOL,
}
#[cfg(feature = "Win32_Foundation")]
impl SERVICE_DELAYED_AUTO_START_INFO {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for SERVICE_DELAYED_AUTO_START_INFO {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::fmt::Debug for SERVICE_DELAYED_AUTO_START_INFO {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("SERVICE_DELAYED_AUTO_START_INFO").field("fDelayedAutostart", &self.fDelayedAutostart).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for SERVICE_DELAYED_AUTO_START_INFO {
    fn eq(&self, other: &Self) -> bool {
        self.fDelayedAutostart == other.fDelayedAutostart
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for SERVICE_DELAYED_AUTO_START_INFO {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for SERVICE_DELAYED_AUTO_START_INFO {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
#[doc = "*Required features: `Win32_System_Services`, `Win32_Foundation`*"]
pub struct SERVICE_DESCRIPTIONA {
    pub lpDescription: super::super::Foundation::PSTR,
}
#[cfg(feature = "Win32_Foundation")]
impl SERVICE_DESCRIPTIONA {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for SERVICE_DESCRIPTIONA {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::fmt::Debug for SERVICE_DESCRIPTIONA {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("SERVICE_DESCRIPTIONA").field("lpDescription", &self.lpDescription).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for SERVICE_DESCRIPTIONA {
    fn eq(&self, other: &Self) -> bool {
        self.lpDescription == other.lpDescription
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for SERVICE_DESCRIPTIONA {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for SERVICE_DESCRIPTIONA {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
#[doc = "*Required features: `Win32_System_Services`, `Win32_Foundation`*"]
pub struct SERVICE_DESCRIPTIONW {
    pub lpDescription: super::super::Foundation::PWSTR,
}
#[cfg(feature = "Win32_Foundation")]
impl SERVICE_DESCRIPTIONW {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for SERVICE_DESCRIPTIONW {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::fmt::Debug for SERVICE_DESCRIPTIONW {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("SERVICE_DESCRIPTIONW").field("lpDescription", &self.lpDescription).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for SERVICE_DESCRIPTIONW {
    fn eq(&self, other: &Self) -> bool {
        self.lpDescription == other.lpDescription
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for SERVICE_DESCRIPTIONW {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for SERVICE_DESCRIPTIONW {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_System_Services`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct SERVICE_DIRECTORY_TYPE(pub i32);
pub const ServiceDirectoryPersistentState: SERVICE_DIRECTORY_TYPE = SERVICE_DIRECTORY_TYPE(0i32);
pub const ServiceDirectoryTypeMax: SERVICE_DIRECTORY_TYPE = SERVICE_DIRECTORY_TYPE(1i32);
impl ::std::convert::From<i32> for SERVICE_DIRECTORY_TYPE {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for SERVICE_DIRECTORY_TYPE {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_System_Services`*"]
pub const SERVICE_DYNAMIC_INFORMATION_LEVEL_START_REASON: u32 = 1u32;
#[doc = "*Required features: `Win32_System_Services`*"]
pub const SERVICE_ENUMERATE_DEPENDENTS: u32 = 8u32;
#[doc = "*Required features: `Win32_System_Services`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct SERVICE_ERROR(pub u32);
pub const SERVICE_ERROR_CRITICAL: SERVICE_ERROR = SERVICE_ERROR(3u32);
pub const SERVICE_ERROR_IGNORE: SERVICE_ERROR = SERVICE_ERROR(0u32);
pub const SERVICE_ERROR_NORMAL: SERVICE_ERROR = SERVICE_ERROR(1u32);
pub const SERVICE_ERROR_SEVERE: SERVICE_ERROR = SERVICE_ERROR(2u32);
impl ::std::convert::From<u32> for SERVICE_ERROR {
    fn from(value: u32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for SERVICE_ERROR {
    type Abi = Self;
}
impl ::std::ops::BitOr for SERVICE_ERROR {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self {
        Self(self.0 | rhs.0)
    }
}
impl ::std::ops::BitAnd for SERVICE_ERROR {
    type Output = Self;
    fn bitand(self, rhs: Self) -> Self {
        Self(self.0 & rhs.0)
    }
}
impl ::std::ops::BitOrAssign for SERVICE_ERROR {
    fn bitor_assign(&mut self, rhs: Self) {
        self.0.bitor_assign(rhs.0)
    }
}
impl ::std::ops::BitAndAssign for SERVICE_ERROR {
    fn bitand_assign(&mut self, rhs: Self) {
        self.0.bitand_assign(rhs.0)
    }
}
impl ::std::ops::Not for SERVICE_ERROR {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
#[doc = "*Required features: `Win32_System_Services`, `Win32_Foundation`*"]
pub struct SERVICE_FAILURE_ACTIONSA {
    pub dwResetPeriod: u32,
    pub lpRebootMsg: super::super::Foundation::PSTR,
    pub lpCommand: super::super::Foundation::PSTR,
    pub cActions: u32,
    pub lpsaActions: *mut SC_ACTION,
}
#[cfg(feature = "Win32_Foundation")]
impl SERVICE_FAILURE_ACTIONSA {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for SERVICE_FAILURE_ACTIONSA {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::fmt::Debug for SERVICE_FAILURE_ACTIONSA {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("SERVICE_FAILURE_ACTIONSA").field("dwResetPeriod", &self.dwResetPeriod).field("lpRebootMsg", &self.lpRebootMsg).field("lpCommand", &self.lpCommand).field("cActions", &self.cActions).field("lpsaActions", &self.lpsaActions).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for SERVICE_FAILURE_ACTIONSA {
    fn eq(&self, other: &Self) -> bool {
        self.dwResetPeriod == other.dwResetPeriod && self.lpRebootMsg == other.lpRebootMsg && self.lpCommand == other.lpCommand && self.cActions == other.cActions && self.lpsaActions == other.lpsaActions
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for SERVICE_FAILURE_ACTIONSA {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for SERVICE_FAILURE_ACTIONSA {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
#[doc = "*Required features: `Win32_System_Services`, `Win32_Foundation`*"]
pub struct SERVICE_FAILURE_ACTIONSW {
    pub dwResetPeriod: u32,
    pub lpRebootMsg: super::super::Foundation::PWSTR,
    pub lpCommand: super::super::Foundation::PWSTR,
    pub cActions: u32,
    pub lpsaActions: *mut SC_ACTION,
}
#[cfg(feature = "Win32_Foundation")]
impl SERVICE_FAILURE_ACTIONSW {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for SERVICE_FAILURE_ACTIONSW {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::fmt::Debug for SERVICE_FAILURE_ACTIONSW {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("SERVICE_FAILURE_ACTIONSW").field("dwResetPeriod", &self.dwResetPeriod).field("lpRebootMsg", &self.lpRebootMsg).field("lpCommand", &self.lpCommand).field("cActions", &self.cActions).field("lpsaActions", &self.lpsaActions).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for SERVICE_FAILURE_ACTIONSW {
    fn eq(&self, other: &Self) -> bool {
        self.dwResetPeriod == other.dwResetPeriod && self.lpRebootMsg == other.lpRebootMsg && self.lpCommand == other.lpCommand && self.cActions == other.cActions && self.lpsaActions == other.lpsaActions
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for SERVICE_FAILURE_ACTIONSW {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for SERVICE_FAILURE_ACTIONSW {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
#[doc = "*Required features: `Win32_System_Services`, `Win32_Foundation`*"]
pub struct SERVICE_FAILURE_ACTIONS_FLAG {
    pub fFailureActionsOnNonCrashFailures: super::super::Foundation::BOOL,
}
#[cfg(feature = "Win32_Foundation")]
impl SERVICE_FAILURE_ACTIONS_FLAG {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for SERVICE_FAILURE_ACTIONS_FLAG {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::fmt::Debug for SERVICE_FAILURE_ACTIONS_FLAG {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("SERVICE_FAILURE_ACTIONS_FLAG").field("fFailureActionsOnNonCrashFailures", &self.fFailureActionsOnNonCrashFailures).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for SERVICE_FAILURE_ACTIONS_FLAG {
    fn eq(&self, other: &Self) -> bool {
        self.fFailureActionsOnNonCrashFailures == other.fFailureActionsOnNonCrashFailures
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for SERVICE_FAILURE_ACTIONS_FLAG {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for SERVICE_FAILURE_ACTIONS_FLAG {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_System_Services`*"]
pub const SERVICE_INTERROGATE: u32 = 128u32;
#[doc = "*Required features: `Win32_System_Services`*"]
pub const SERVICE_LAUNCH_PROTECTED_ANTIMALWARE_LIGHT: u32 = 3u32;
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_System_Services`*"]
pub struct SERVICE_LAUNCH_PROTECTED_INFO {
    pub dwLaunchProtected: u32,
}
impl SERVICE_LAUNCH_PROTECTED_INFO {}
impl ::std::default::Default for SERVICE_LAUNCH_PROTECTED_INFO {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for SERVICE_LAUNCH_PROTECTED_INFO {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("SERVICE_LAUNCH_PROTECTED_INFO").field("dwLaunchProtected", &self.dwLaunchProtected).finish()
    }
}
impl ::std::cmp::PartialEq for SERVICE_LAUNCH_PROTECTED_INFO {
    fn eq(&self, other: &Self) -> bool {
        self.dwLaunchProtected == other.dwLaunchProtected
    }
}
impl ::std::cmp::Eq for SERVICE_LAUNCH_PROTECTED_INFO {}
unsafe impl ::windows::runtime::Abi for SERVICE_LAUNCH_PROTECTED_INFO {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_System_Services`*"]
pub const SERVICE_LAUNCH_PROTECTED_NONE: u32 = 0u32;
#[doc = "*Required features: `Win32_System_Services`*"]
pub const SERVICE_LAUNCH_PROTECTED_WINDOWS: u32 = 1u32;
#[doc = "*Required features: `Win32_System_Services`*"]
pub const SERVICE_LAUNCH_PROTECTED_WINDOWS_LIGHT: u32 = 2u32;
pub type SERVICE_MAIN_FUNCTIONA = unsafe extern "system" fn(dwnumservicesargs: u32, lpserviceargvectors: *mut *mut i8);
#[cfg(feature = "Win32_Foundation")]
pub type SERVICE_MAIN_FUNCTIONW = unsafe extern "system" fn(dwnumservicesargs: u32, lpserviceargvectors: *mut super::super::Foundation::PWSTR);
#[doc = "*Required features: `Win32_System_Services`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct SERVICE_NOTIFY(pub u32);
pub const SERVICE_NOTIFY_CREATED: SERVICE_NOTIFY = SERVICE_NOTIFY(128u32);
pub const SERVICE_NOTIFY_CONTINUE_PENDING: SERVICE_NOTIFY = SERVICE_NOTIFY(16u32);
pub const SERVICE_NOTIFY_DELETE_PENDING: SERVICE_NOTIFY = SERVICE_NOTIFY(512u32);
pub const SERVICE_NOTIFY_DELETED: SERVICE_NOTIFY = SERVICE_NOTIFY(256u32);
pub const SERVICE_NOTIFY_PAUSE_PENDING: SERVICE_NOTIFY = SERVICE_NOTIFY(32u32);
pub const SERVICE_NOTIFY_PAUSED: SERVICE_NOTIFY = SERVICE_NOTIFY(64u32);
pub const SERVICE_NOTIFY_RUNNING: SERVICE_NOTIFY = SERVICE_NOTIFY(8u32);
pub const SERVICE_NOTIFY_START_PENDING: SERVICE_NOTIFY = SERVICE_NOTIFY(2u32);
pub const SERVICE_NOTIFY_STOP_PENDING: SERVICE_NOTIFY = SERVICE_NOTIFY(4u32);
pub const SERVICE_NOTIFY_STOPPED: SERVICE_NOTIFY = SERVICE_NOTIFY(1u32);
impl ::std::convert::From<u32> for SERVICE_NOTIFY {
    fn from(value: u32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for SERVICE_NOTIFY {
    type Abi = Self;
}
impl ::std::ops::BitOr for SERVICE_NOTIFY {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self {
        Self(self.0 | rhs.0)
    }
}
impl ::std::ops::BitAnd for SERVICE_NOTIFY {
    type Output = Self;
    fn bitand(self, rhs: Self) -> Self {
        Self(self.0 & rhs.0)
    }
}
impl ::std::ops::BitOrAssign for SERVICE_NOTIFY {
    fn bitor_assign(&mut self, rhs: Self) {
        self.0.bitor_assign(rhs.0)
    }
}
impl ::std::ops::BitAndAssign for SERVICE_NOTIFY {
    fn bitand_assign(&mut self, rhs: Self) {
        self.0.bitand_assign(rhs.0)
    }
}
impl ::std::ops::Not for SERVICE_NOTIFY {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[derive(:: std :: clone :: Clone)]
#[repr(C)]
#[doc = "*Required features: `Win32_System_Services`*"]
pub struct SERVICE_NOTIFY_1 {
    pub dwVersion: u32,
    pub pfnNotifyCallback: ::std::option::Option<PFN_SC_NOTIFY_CALLBACK>,
    pub pContext: *mut ::std::ffi::c_void,
    pub dwNotificationStatus: u32,
    pub ServiceStatus: SERVICE_STATUS_PROCESS,
}
impl SERVICE_NOTIFY_1 {}
impl ::std::default::Default for SERVICE_NOTIFY_1 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for SERVICE_NOTIFY_1 {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("SERVICE_NOTIFY_1").field("dwVersion", &self.dwVersion).field("pContext", &self.pContext).field("dwNotificationStatus", &self.dwNotificationStatus).field("ServiceStatus", &self.ServiceStatus).finish()
    }
}
impl ::std::cmp::PartialEq for SERVICE_NOTIFY_1 {
    fn eq(&self, other: &Self) -> bool {
        self.dwVersion == other.dwVersion && self.pfnNotifyCallback.map(|f| f as usize) == other.pfnNotifyCallback.map(|f| f as usize) && self.pContext == other.pContext && self.dwNotificationStatus == other.dwNotificationStatus && self.ServiceStatus == other.ServiceStatus
    }
}
impl ::std::cmp::Eq for SERVICE_NOTIFY_1 {}
unsafe impl ::windows::runtime::Abi for SERVICE_NOTIFY_1 {
    type Abi = ::std::mem::ManuallyDrop<Self>;
}
#[derive(:: std :: clone :: Clone)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
#[doc = "*Required features: `Win32_System_Services`, `Win32_Foundation`*"]
pub struct SERVICE_NOTIFY_2A {
    pub dwVersion: u32,
    pub pfnNotifyCallback: ::std::option::Option<PFN_SC_NOTIFY_CALLBACK>,
    pub pContext: *mut ::std::ffi::c_void,
    pub dwNotificationStatus: u32,
    pub ServiceStatus: SERVICE_STATUS_PROCESS,
    pub dwNotificationTriggered: u32,
    pub pszServiceNames: super::super::Foundation::PSTR,
}
#[cfg(feature = "Win32_Foundation")]
impl SERVICE_NOTIFY_2A {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for SERVICE_NOTIFY_2A {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::fmt::Debug for SERVICE_NOTIFY_2A {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("SERVICE_NOTIFY_2A")
            .field("dwVersion", &self.dwVersion)
            .field("pContext", &self.pContext)
            .field("dwNotificationStatus", &self.dwNotificationStatus)
            .field("ServiceStatus", &self.ServiceStatus)
            .field("dwNotificationTriggered", &self.dwNotificationTriggered)
            .field("pszServiceNames", &self.pszServiceNames)
            .finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for SERVICE_NOTIFY_2A {
    fn eq(&self, other: &Self) -> bool {
        self.dwVersion == other.dwVersion && self.pfnNotifyCallback.map(|f| f as usize) == other.pfnNotifyCallback.map(|f| f as usize) && self.pContext == other.pContext && self.dwNotificationStatus == other.dwNotificationStatus && self.ServiceStatus == other.ServiceStatus && self.dwNotificationTriggered == other.dwNotificationTriggered && self.pszServiceNames == other.pszServiceNames
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for SERVICE_NOTIFY_2A {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for SERVICE_NOTIFY_2A {
    type Abi = ::std::mem::ManuallyDrop<Self>;
}
#[derive(:: std :: clone :: Clone)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
#[doc = "*Required features: `Win32_System_Services`, `Win32_Foundation`*"]
pub struct SERVICE_NOTIFY_2W {
    pub dwVersion: u32,
    pub pfnNotifyCallback: ::std::option::Option<PFN_SC_NOTIFY_CALLBACK>,
    pub pContext: *mut ::std::ffi::c_void,
    pub dwNotificationStatus: u32,
    pub ServiceStatus: SERVICE_STATUS_PROCESS,
    pub dwNotificationTriggered: u32,
    pub pszServiceNames: super::super::Foundation::PWSTR,
}
#[cfg(feature = "Win32_Foundation")]
impl SERVICE_NOTIFY_2W {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for SERVICE_NOTIFY_2W {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::fmt::Debug for SERVICE_NOTIFY_2W {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("SERVICE_NOTIFY_2W")
            .field("dwVersion", &self.dwVersion)
            .field("pContext", &self.pContext)
            .field("dwNotificationStatus", &self.dwNotificationStatus)
            .field("ServiceStatus", &self.ServiceStatus)
            .field("dwNotificationTriggered", &self.dwNotificationTriggered)
            .field("pszServiceNames", &self.pszServiceNames)
            .finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for SERVICE_NOTIFY_2W {
    fn eq(&self, other: &Self) -> bool {
        self.dwVersion == other.dwVersion && self.pfnNotifyCallback.map(|f| f as usize) == other.pfnNotifyCallback.map(|f| f as usize) && self.pContext == other.pContext && self.dwNotificationStatus == other.dwNotificationStatus && self.ServiceStatus == other.ServiceStatus && self.dwNotificationTriggered == other.dwNotificationTriggered && self.pszServiceNames == other.pszServiceNames
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for SERVICE_NOTIFY_2W {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for SERVICE_NOTIFY_2W {
    type Abi = ::std::mem::ManuallyDrop<Self>;
}
#[doc = "*Required features: `Win32_System_Services`*"]
pub const SERVICE_NOTIFY_STATUS_CHANGE: u32 = 2u32;
#[doc = "*Required features: `Win32_System_Services`*"]
pub const SERVICE_NOTIFY_STATUS_CHANGE_1: u32 = 1u32;
#[doc = "*Required features: `Win32_System_Services`*"]
pub const SERVICE_NOTIFY_STATUS_CHANGE_2: u32 = 2u32;
#[doc = "*Required features: `Win32_System_Services`*"]
pub const SERVICE_NO_CHANGE: u32 = 4294967295u32;
#[doc = "*Required features: `Win32_System_Services`*"]
pub const SERVICE_PAUSE_CONTINUE: u32 = 64u32;
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
#[doc = "*Required features: `Win32_System_Services`, `Win32_Foundation`*"]
pub struct SERVICE_PREFERRED_NODE_INFO {
    pub usPreferredNode: u16,
    pub fDelete: super::super::Foundation::BOOLEAN,
}
#[cfg(feature = "Win32_Foundation")]
impl SERVICE_PREFERRED_NODE_INFO {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for SERVICE_PREFERRED_NODE_INFO {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::fmt::Debug for SERVICE_PREFERRED_NODE_INFO {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("SERVICE_PREFERRED_NODE_INFO").field("usPreferredNode", &self.usPreferredNode).field("fDelete", &self.fDelete).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for SERVICE_PREFERRED_NODE_INFO {
    fn eq(&self, other: &Self) -> bool {
        self.usPreferredNode == other.usPreferredNode && self.fDelete == other.fDelete
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for SERVICE_PREFERRED_NODE_INFO {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for SERVICE_PREFERRED_NODE_INFO {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_System_Services`*"]
pub struct SERVICE_PRESHUTDOWN_INFO {
    pub dwPreshutdownTimeout: u32,
}
impl SERVICE_PRESHUTDOWN_INFO {}
impl ::std::default::Default for SERVICE_PRESHUTDOWN_INFO {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for SERVICE_PRESHUTDOWN_INFO {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("SERVICE_PRESHUTDOWN_INFO").field("dwPreshutdownTimeout", &self.dwPreshutdownTimeout).finish()
    }
}
impl ::std::cmp::PartialEq for SERVICE_PRESHUTDOWN_INFO {
    fn eq(&self, other: &Self) -> bool {
        self.dwPreshutdownTimeout == other.dwPreshutdownTimeout
    }
}
impl ::std::cmp::Eq for SERVICE_PRESHUTDOWN_INFO {}
unsafe impl ::windows::runtime::Abi for SERVICE_PRESHUTDOWN_INFO {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_System_Services`*"]
pub const SERVICE_QUERY_CONFIG: u32 = 1u32;
#[doc = "*Required features: `Win32_System_Services`*"]
pub const SERVICE_QUERY_STATUS: u32 = 4u32;
#[doc = "*Required features: `Win32_System_Services`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct SERVICE_REGISTRY_STATE_TYPE(pub i32);
pub const ServiceRegistryStateParameters: SERVICE_REGISTRY_STATE_TYPE = SERVICE_REGISTRY_STATE_TYPE(0i32);
pub const ServiceRegistryStatePersistent: SERVICE_REGISTRY_STATE_TYPE = SERVICE_REGISTRY_STATE_TYPE(1i32);
pub const MaxServiceRegistryStateType: SERVICE_REGISTRY_STATE_TYPE = SERVICE_REGISTRY_STATE_TYPE(2i32);
impl ::std::convert::From<i32> for SERVICE_REGISTRY_STATE_TYPE {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for SERVICE_REGISTRY_STATE_TYPE {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
#[doc = "*Required features: `Win32_System_Services`, `Win32_Foundation`*"]
pub struct SERVICE_REQUIRED_PRIVILEGES_INFOA {
    pub pmszRequiredPrivileges: super::super::Foundation::PSTR,
}
#[cfg(feature = "Win32_Foundation")]
impl SERVICE_REQUIRED_PRIVILEGES_INFOA {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for SERVICE_REQUIRED_PRIVILEGES_INFOA {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::fmt::Debug for SERVICE_REQUIRED_PRIVILEGES_INFOA {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("SERVICE_REQUIRED_PRIVILEGES_INFOA").field("pmszRequiredPrivileges", &self.pmszRequiredPrivileges).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for SERVICE_REQUIRED_PRIVILEGES_INFOA {
    fn eq(&self, other: &Self) -> bool {
        self.pmszRequiredPrivileges == other.pmszRequiredPrivileges
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for SERVICE_REQUIRED_PRIVILEGES_INFOA {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for SERVICE_REQUIRED_PRIVILEGES_INFOA {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
#[doc = "*Required features: `Win32_System_Services`, `Win32_Foundation`*"]
pub struct SERVICE_REQUIRED_PRIVILEGES_INFOW {
    pub pmszRequiredPrivileges: super::super::Foundation::PWSTR,
}
#[cfg(feature = "Win32_Foundation")]
impl SERVICE_REQUIRED_PRIVILEGES_INFOW {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for SERVICE_REQUIRED_PRIVILEGES_INFOW {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::fmt::Debug for SERVICE_REQUIRED_PRIVILEGES_INFOW {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("SERVICE_REQUIRED_PRIVILEGES_INFOW").field("pmszRequiredPrivileges", &self.pmszRequiredPrivileges).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for SERVICE_REQUIRED_PRIVILEGES_INFOW {
    fn eq(&self, other: &Self) -> bool {
        self.pmszRequiredPrivileges == other.pmszRequiredPrivileges
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for SERVICE_REQUIRED_PRIVILEGES_INFOW {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for SERVICE_REQUIRED_PRIVILEGES_INFOW {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_System_Services`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct SERVICE_RUNS_IN_PROCESS(pub u32);
pub const SERVICE_RUNS_IN_NON_SYSTEM_OR_NOT_RUNNING: SERVICE_RUNS_IN_PROCESS = SERVICE_RUNS_IN_PROCESS(0u32);
pub const SERVICE_RUNS_IN_SYSTEM_PROCESS: SERVICE_RUNS_IN_PROCESS = SERVICE_RUNS_IN_PROCESS(1u32);
impl ::std::convert::From<u32> for SERVICE_RUNS_IN_PROCESS {
    fn from(value: u32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for SERVICE_RUNS_IN_PROCESS {
    type Abi = Self;
}
impl ::std::ops::BitOr for SERVICE_RUNS_IN_PROCESS {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self {
        Self(self.0 | rhs.0)
    }
}
impl ::std::ops::BitAnd for SERVICE_RUNS_IN_PROCESS {
    type Output = Self;
    fn bitand(self, rhs: Self) -> Self {
        Self(self.0 & rhs.0)
    }
}
impl ::std::ops::BitOrAssign for SERVICE_RUNS_IN_PROCESS {
    fn bitor_assign(&mut self, rhs: Self) {
        self.0.bitor_assign(rhs.0)
    }
}
impl ::std::ops::BitAndAssign for SERVICE_RUNS_IN_PROCESS {
    fn bitand_assign(&mut self, rhs: Self) {
        self.0.bitand_assign(rhs.0)
    }
}
impl ::std::ops::Not for SERVICE_RUNS_IN_PROCESS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[doc = "*Required features: `Win32_System_Services`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct SERVICE_SHARED_DIRECTORY_TYPE(pub i32);
pub const ServiceSharedDirectoryPersistentState: SERVICE_SHARED_DIRECTORY_TYPE = SERVICE_SHARED_DIRECTORY_TYPE(0i32);
impl ::std::convert::From<i32> for SERVICE_SHARED_DIRECTORY_TYPE {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for SERVICE_SHARED_DIRECTORY_TYPE {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_System_Services`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct SERVICE_SHARED_REGISTRY_STATE_TYPE(pub i32);
pub const ServiceSharedRegistryPersistentState: SERVICE_SHARED_REGISTRY_STATE_TYPE = SERVICE_SHARED_REGISTRY_STATE_TYPE(0i32);
impl ::std::convert::From<i32> for SERVICE_SHARED_REGISTRY_STATE_TYPE {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for SERVICE_SHARED_REGISTRY_STATE_TYPE {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_System_Services`*"]
pub struct SERVICE_SID_INFO {
    pub dwServiceSidType: u32,
}
impl SERVICE_SID_INFO {}
impl ::std::default::Default for SERVICE_SID_INFO {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for SERVICE_SID_INFO {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("SERVICE_SID_INFO").field("dwServiceSidType", &self.dwServiceSidType).finish()
    }
}
impl ::std::cmp::PartialEq for SERVICE_SID_INFO {
    fn eq(&self, other: &Self) -> bool {
        self.dwServiceSidType == other.dwServiceSidType
    }
}
impl ::std::cmp::Eq for SERVICE_SID_INFO {}
unsafe impl ::windows::runtime::Abi for SERVICE_SID_INFO {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_System_Services`*"]
pub const SERVICE_SID_TYPE_NONE: u32 = 0u32;
#[doc = "*Required features: `Win32_System_Services`*"]
pub const SERVICE_SID_TYPE_UNRESTRICTED: u32 = 1u32;
#[doc = "*Required features: `Win32_System_Services`*"]
pub const SERVICE_START: u32 = 16u32;
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_System_Services`*"]
pub struct SERVICE_START_REASON {
    pub dwReason: u32,
}
impl SERVICE_START_REASON {}
impl ::std::default::Default for SERVICE_START_REASON {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for SERVICE_START_REASON {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("SERVICE_START_REASON").field("dwReason", &self.dwReason).finish()
    }
}
impl ::std::cmp::PartialEq for SERVICE_START_REASON {
    fn eq(&self, other: &Self) -> bool {
        self.dwReason == other.dwReason
    }
}
impl ::std::cmp::Eq for SERVICE_START_REASON {}
unsafe impl ::windows::runtime::Abi for SERVICE_START_REASON {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_System_Services`*"]
pub const SERVICE_START_REASON_AUTO: u32 = 2u32;
#[doc = "*Required features: `Win32_System_Services`*"]
pub const SERVICE_START_REASON_DELAYEDAUTO: u32 = 16u32;
#[doc = "*Required features: `Win32_System_Services`*"]
pub const SERVICE_START_REASON_DEMAND: u32 = 1u32;
#[doc = "*Required features: `Win32_System_Services`*"]
pub const SERVICE_START_REASON_RESTART_ON_FAILURE: u32 = 8u32;
#[doc = "*Required features: `Win32_System_Services`*"]
pub const SERVICE_START_REASON_TRIGGER: u32 = 4u32;
#[doc = "*Required features: `Win32_System_Services`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct SERVICE_START_TYPE(pub u32);
pub const SERVICE_AUTO_START: SERVICE_START_TYPE = SERVICE_START_TYPE(2u32);
pub const SERVICE_BOOT_START: SERVICE_START_TYPE = SERVICE_START_TYPE(0u32);
pub const SERVICE_DEMAND_START: SERVICE_START_TYPE = SERVICE_START_TYPE(3u32);
pub const SERVICE_DISABLED: SERVICE_START_TYPE = SERVICE_START_TYPE(4u32);
pub const SERVICE_SYSTEM_START: SERVICE_START_TYPE = SERVICE_START_TYPE(1u32);
impl ::std::convert::From<u32> for SERVICE_START_TYPE {
    fn from(value: u32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for SERVICE_START_TYPE {
    type Abi = Self;
}
impl ::std::ops::BitOr for SERVICE_START_TYPE {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self {
        Self(self.0 | rhs.0)
    }
}
impl ::std::ops::BitAnd for SERVICE_START_TYPE {
    type Output = Self;
    fn bitand(self, rhs: Self) -> Self {
        Self(self.0 & rhs.0)
    }
}
impl ::std::ops::BitOrAssign for SERVICE_START_TYPE {
    fn bitor_assign(&mut self, rhs: Self) {
        self.0.bitor_assign(rhs.0)
    }
}
impl ::std::ops::BitAndAssign for SERVICE_START_TYPE {
    fn bitand_assign(&mut self, rhs: Self) {
        self.0.bitand_assign(rhs.0)
    }
}
impl ::std::ops::Not for SERVICE_START_TYPE {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_System_Services`*"]
pub struct SERVICE_STATUS {
    pub dwServiceType: ENUM_SERVICE_TYPE,
    pub dwCurrentState: SERVICE_STATUS_CURRENT_STATE,
    pub dwControlsAccepted: u32,
    pub dwWin32ExitCode: u32,
    pub dwServiceSpecificExitCode: u32,
    pub dwCheckPoint: u32,
    pub dwWaitHint: u32,
}
impl SERVICE_STATUS {}
impl ::std::default::Default for SERVICE_STATUS {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for SERVICE_STATUS {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("SERVICE_STATUS")
            .field("dwServiceType", &self.dwServiceType)
            .field("dwCurrentState", &self.dwCurrentState)
            .field("dwControlsAccepted", &self.dwControlsAccepted)
            .field("dwWin32ExitCode", &self.dwWin32ExitCode)
            .field("dwServiceSpecificExitCode", &self.dwServiceSpecificExitCode)
            .field("dwCheckPoint", &self.dwCheckPoint)
            .field("dwWaitHint", &self.dwWaitHint)
            .finish()
    }
}
impl ::std::cmp::PartialEq for SERVICE_STATUS {
    fn eq(&self, other: &Self) -> bool {
        self.dwServiceType == other.dwServiceType && self.dwCurrentState == other.dwCurrentState && self.dwControlsAccepted == other.dwControlsAccepted && self.dwWin32ExitCode == other.dwWin32ExitCode && self.dwServiceSpecificExitCode == other.dwServiceSpecificExitCode && self.dwCheckPoint == other.dwCheckPoint && self.dwWaitHint == other.dwWaitHint
    }
}
impl ::std::cmp::Eq for SERVICE_STATUS {}
unsafe impl ::windows::runtime::Abi for SERVICE_STATUS {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_System_Services`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct SERVICE_STATUS_CURRENT_STATE(pub u32);
pub const SERVICE_CONTINUE_PENDING: SERVICE_STATUS_CURRENT_STATE = SERVICE_STATUS_CURRENT_STATE(5u32);
pub const SERVICE_PAUSE_PENDING: SERVICE_STATUS_CURRENT_STATE = SERVICE_STATUS_CURRENT_STATE(6u32);
pub const SERVICE_PAUSED: SERVICE_STATUS_CURRENT_STATE = SERVICE_STATUS_CURRENT_STATE(7u32);
pub const SERVICE_RUNNING: SERVICE_STATUS_CURRENT_STATE = SERVICE_STATUS_CURRENT_STATE(4u32);
pub const SERVICE_START_PENDING: SERVICE_STATUS_CURRENT_STATE = SERVICE_STATUS_CURRENT_STATE(2u32);
pub const SERVICE_STOP_PENDING: SERVICE_STATUS_CURRENT_STATE = SERVICE_STATUS_CURRENT_STATE(3u32);
pub const SERVICE_STOPPED: SERVICE_STATUS_CURRENT_STATE = SERVICE_STATUS_CURRENT_STATE(1u32);
impl ::std::convert::From<u32> for SERVICE_STATUS_CURRENT_STATE {
    fn from(value: u32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for SERVICE_STATUS_CURRENT_STATE {
    type Abi = Self;
}
impl ::std::ops::BitOr for SERVICE_STATUS_CURRENT_STATE {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self {
        Self(self.0 | rhs.0)
    }
}
impl ::std::ops::BitAnd for SERVICE_STATUS_CURRENT_STATE {
    type Output = Self;
    fn bitand(self, rhs: Self) -> Self {
        Self(self.0 & rhs.0)
    }
}
impl ::std::ops::BitOrAssign for SERVICE_STATUS_CURRENT_STATE {
    fn bitor_assign(&mut self, rhs: Self) {
        self.0.bitor_assign(rhs.0)
    }
}
impl ::std::ops::BitAndAssign for SERVICE_STATUS_CURRENT_STATE {
    fn bitand_assign(&mut self, rhs: Self) {
        self.0.bitand_assign(rhs.0)
    }
}
impl ::std::ops::Not for SERVICE_STATUS_CURRENT_STATE {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy, :: std :: fmt :: Debug, :: std :: cmp :: PartialEq, :: std :: cmp :: Eq)]
#[repr(transparent)]
pub struct SERVICE_STATUS_HANDLE(pub isize);
impl ::std::default::Default for SERVICE_STATUS_HANDLE {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
unsafe impl ::windows::runtime::Handle for SERVICE_STATUS_HANDLE {}
unsafe impl ::windows::runtime::Abi for SERVICE_STATUS_HANDLE {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_System_Services`*"]
pub struct SERVICE_STATUS_PROCESS {
    pub dwServiceType: ENUM_SERVICE_TYPE,
    pub dwCurrentState: SERVICE_STATUS_CURRENT_STATE,
    pub dwControlsAccepted: u32,
    pub dwWin32ExitCode: u32,
    pub dwServiceSpecificExitCode: u32,
    pub dwCheckPoint: u32,
    pub dwWaitHint: u32,
    pub dwProcessId: u32,
    pub dwServiceFlags: SERVICE_RUNS_IN_PROCESS,
}
impl SERVICE_STATUS_PROCESS {}
impl ::std::default::Default for SERVICE_STATUS_PROCESS {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for SERVICE_STATUS_PROCESS {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("SERVICE_STATUS_PROCESS")
            .field("dwServiceType", &self.dwServiceType)
            .field("dwCurrentState", &self.dwCurrentState)
            .field("dwControlsAccepted", &self.dwControlsAccepted)
            .field("dwWin32ExitCode", &self.dwWin32ExitCode)
            .field("dwServiceSpecificExitCode", &self.dwServiceSpecificExitCode)
            .field("dwCheckPoint", &self.dwCheckPoint)
            .field("dwWaitHint", &self.dwWaitHint)
            .field("dwProcessId", &self.dwProcessId)
            .field("dwServiceFlags", &self.dwServiceFlags)
            .finish()
    }
}
impl ::std::cmp::PartialEq for SERVICE_STATUS_PROCESS {
    fn eq(&self, other: &Self) -> bool {
        self.dwServiceType == other.dwServiceType && self.dwCurrentState == other.dwCurrentState && self.dwControlsAccepted == other.dwControlsAccepted && self.dwWin32ExitCode == other.dwWin32ExitCode && self.dwServiceSpecificExitCode == other.dwServiceSpecificExitCode && self.dwCheckPoint == other.dwCheckPoint && self.dwWaitHint == other.dwWaitHint && self.dwProcessId == other.dwProcessId && self.dwServiceFlags == other.dwServiceFlags
    }
}
impl ::std::cmp::Eq for SERVICE_STATUS_PROCESS {}
unsafe impl ::windows::runtime::Abi for SERVICE_STATUS_PROCESS {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_System_Services`*"]
pub const SERVICE_STOP: u32 = 32u32;
#[doc = "*Required features: `Win32_System_Services`*"]
pub const SERVICE_STOP_REASON_FLAG_CUSTOM: u32 = 536870912u32;
#[doc = "*Required features: `Win32_System_Services`*"]
pub const SERVICE_STOP_REASON_FLAG_MAX: u32 = 2147483648u32;
#[doc = "*Required features: `Win32_System_Services`*"]
pub const SERVICE_STOP_REASON_FLAG_MIN: u32 = 0u32;
#[doc = "*Required features: `Win32_System_Services`*"]
pub const SERVICE_STOP_REASON_FLAG_PLANNED: u32 = 1073741824u32;
#[doc = "*Required features: `Win32_System_Services`*"]
pub const SERVICE_STOP_REASON_FLAG_UNPLANNED: u32 = 268435456u32;
#[doc = "*Required features: `Win32_System_Services`*"]
pub const SERVICE_STOP_REASON_MAJOR_APPLICATION: u32 = 327680u32;
#[doc = "*Required features: `Win32_System_Services`*"]
pub const SERVICE_STOP_REASON_MAJOR_HARDWARE: u32 = 131072u32;
#[doc = "*Required features: `Win32_System_Services`*"]
pub const SERVICE_STOP_REASON_MAJOR_MAX: u32 = 458752u32;
#[doc = "*Required features: `Win32_System_Services`*"]
pub const SERVICE_STOP_REASON_MAJOR_MAX_CUSTOM: u32 = 16711680u32;
#[doc = "*Required features: `Win32_System_Services`*"]
pub const SERVICE_STOP_REASON_MAJOR_MIN: u32 = 0u32;
#[doc = "*Required features: `Win32_System_Services`*"]
pub const SERVICE_STOP_REASON_MAJOR_MIN_CUSTOM: u32 = 4194304u32;
#[doc = "*Required features: `Win32_System_Services`*"]
pub const SERVICE_STOP_REASON_MAJOR_NONE: u32 = 393216u32;
#[doc = "*Required features: `Win32_System_Services`*"]
pub const SERVICE_STOP_REASON_MAJOR_OPERATINGSYSTEM: u32 = 196608u32;
#[doc = "*Required features: `Win32_System_Services`*"]
pub const SERVICE_STOP_REASON_MAJOR_OTHER: u32 = 65536u32;
#[doc = "*Required features: `Win32_System_Services`*"]
pub const SERVICE_STOP_REASON_MAJOR_SOFTWARE: u32 = 262144u32;
#[doc = "*Required features: `Win32_System_Services`*"]
pub const SERVICE_STOP_REASON_MINOR_DISK: u32 = 8u32;
#[doc = "*Required features: `Win32_System_Services`*"]
pub const SERVICE_STOP_REASON_MINOR_ENVIRONMENT: u32 = 10u32;
#[doc = "*Required features: `Win32_System_Services`*"]
pub const SERVICE_STOP_REASON_MINOR_HARDWARE_DRIVER: u32 = 11u32;
#[doc = "*Required features: `Win32_System_Services`*"]
pub const SERVICE_STOP_REASON_MINOR_HUNG: u32 = 6u32;
#[doc = "*Required features: `Win32_System_Services`*"]
pub const SERVICE_STOP_REASON_MINOR_INSTALLATION: u32 = 3u32;
#[doc = "*Required features: `Win32_System_Services`*"]
pub const SERVICE_STOP_REASON_MINOR_MAINTENANCE: u32 = 2u32;
#[doc = "*Required features: `Win32_System_Services`*"]
pub const SERVICE_STOP_REASON_MINOR_MAX: u32 = 25u32;
#[doc = "*Required features: `Win32_System_Services`*"]
pub const SERVICE_STOP_REASON_MINOR_MAX_CUSTOM: u32 = 65535u32;
#[doc = "*Required features: `Win32_System_Services`*"]
pub const SERVICE_STOP_REASON_MINOR_MEMOTYLIMIT: u32 = 24u32;
#[doc = "*Required features: `Win32_System_Services`*"]
pub const SERVICE_STOP_REASON_MINOR_MIN: u32 = 0u32;
#[doc = "*Required features: `Win32_System_Services`*"]
pub const SERVICE_STOP_REASON_MINOR_MIN_CUSTOM: u32 = 256u32;
#[doc = "*Required features: `Win32_System_Services`*"]
pub const SERVICE_STOP_REASON_MINOR_MMC: u32 = 22u32;
#[doc = "*Required features: `Win32_System_Services`*"]
pub const SERVICE_STOP_REASON_MINOR_NETWORKCARD: u32 = 9u32;
#[doc = "*Required features: `Win32_System_Services`*"]
pub const SERVICE_STOP_REASON_MINOR_NETWORK_CONNECTIVITY: u32 = 17u32;
#[doc = "*Required features: `Win32_System_Services`*"]
pub const SERVICE_STOP_REASON_MINOR_NONE: u32 = 23u32;
#[doc = "*Required features: `Win32_System_Services`*"]
pub const SERVICE_STOP_REASON_MINOR_OTHER: u32 = 1u32;
#[doc = "*Required features: `Win32_System_Services`*"]
pub const SERVICE_STOP_REASON_MINOR_OTHERDRIVER: u32 = 12u32;
#[doc = "*Required features: `Win32_System_Services`*"]
pub const SERVICE_STOP_REASON_MINOR_RECONFIG: u32 = 5u32;
#[doc = "*Required features: `Win32_System_Services`*"]
pub const SERVICE_STOP_REASON_MINOR_SECURITY: u32 = 16u32;
#[doc = "*Required features: `Win32_System_Services`*"]
pub const SERVICE_STOP_REASON_MINOR_SECURITYFIX: u32 = 15u32;
#[doc = "*Required features: `Win32_System_Services`*"]
pub const SERVICE_STOP_REASON_MINOR_SECURITYFIX_UNINSTALL: u32 = 21u32;
#[doc = "*Required features: `Win32_System_Services`*"]
pub const SERVICE_STOP_REASON_MINOR_SERVICEPACK: u32 = 13u32;
#[doc = "*Required features: `Win32_System_Services`*"]
pub const SERVICE_STOP_REASON_MINOR_SERVICEPACK_UNINSTALL: u32 = 19u32;
#[doc = "*Required features: `Win32_System_Services`*"]
pub const SERVICE_STOP_REASON_MINOR_SOFTWARE_UPDATE: u32 = 14u32;
#[doc = "*Required features: `Win32_System_Services`*"]
pub const SERVICE_STOP_REASON_MINOR_SOFTWARE_UPDATE_UNINSTALL: u32 = 20u32;
#[doc = "*Required features: `Win32_System_Services`*"]
pub const SERVICE_STOP_REASON_MINOR_UNSTABLE: u32 = 7u32;
#[doc = "*Required features: `Win32_System_Services`*"]
pub const SERVICE_STOP_REASON_MINOR_UPGRADE: u32 = 4u32;
#[doc = "*Required features: `Win32_System_Services`*"]
pub const SERVICE_STOP_REASON_MINOR_WMI: u32 = 18u32;
#[derive(:: std :: clone :: Clone)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
#[doc = "*Required features: `Win32_System_Services`, `Win32_Foundation`*"]
pub struct SERVICE_TABLE_ENTRYA {
    pub lpServiceName: super::super::Foundation::PSTR,
    pub lpServiceProc: ::std::option::Option<LPSERVICE_MAIN_FUNCTIONA>,
}
#[cfg(feature = "Win32_Foundation")]
impl SERVICE_TABLE_ENTRYA {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for SERVICE_TABLE_ENTRYA {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::fmt::Debug for SERVICE_TABLE_ENTRYA {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("SERVICE_TABLE_ENTRYA").field("lpServiceName", &self.lpServiceName).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for SERVICE_TABLE_ENTRYA {
    fn eq(&self, other: &Self) -> bool {
        self.lpServiceName == other.lpServiceName && self.lpServiceProc.map(|f| f as usize) == other.lpServiceProc.map(|f| f as usize)
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for SERVICE_TABLE_ENTRYA {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for SERVICE_TABLE_ENTRYA {
    type Abi = ::std::mem::ManuallyDrop<Self>;
}
#[derive(:: std :: clone :: Clone)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
#[doc = "*Required features: `Win32_System_Services`, `Win32_Foundation`*"]
pub struct SERVICE_TABLE_ENTRYW {
    pub lpServiceName: super::super::Foundation::PWSTR,
    pub lpServiceProc: ::std::option::Option<LPSERVICE_MAIN_FUNCTIONW>,
}
#[cfg(feature = "Win32_Foundation")]
impl SERVICE_TABLE_ENTRYW {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for SERVICE_TABLE_ENTRYW {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::fmt::Debug for SERVICE_TABLE_ENTRYW {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("SERVICE_TABLE_ENTRYW").field("lpServiceName", &self.lpServiceName).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for SERVICE_TABLE_ENTRYW {
    fn eq(&self, other: &Self) -> bool {
        self.lpServiceName == other.lpServiceName && self.lpServiceProc.map(|f| f as usize) == other.lpServiceProc.map(|f| f as usize)
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for SERVICE_TABLE_ENTRYW {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for SERVICE_TABLE_ENTRYW {
    type Abi = ::std::mem::ManuallyDrop<Self>;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_System_Services`*"]
pub struct SERVICE_TIMECHANGE_INFO {
    pub liNewTime: i64,
    pub liOldTime: i64,
}
impl SERVICE_TIMECHANGE_INFO {}
impl ::std::default::Default for SERVICE_TIMECHANGE_INFO {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for SERVICE_TIMECHANGE_INFO {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("SERVICE_TIMECHANGE_INFO").field("liNewTime", &self.liNewTime).field("liOldTime", &self.liOldTime).finish()
    }
}
impl ::std::cmp::PartialEq for SERVICE_TIMECHANGE_INFO {
    fn eq(&self, other: &Self) -> bool {
        self.liNewTime == other.liNewTime && self.liOldTime == other.liOldTime
    }
}
impl ::std::cmp::Eq for SERVICE_TIMECHANGE_INFO {}
unsafe impl ::windows::runtime::Abi for SERVICE_TIMECHANGE_INFO {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_System_Services`*"]
pub struct SERVICE_TRIGGER {
    pub dwTriggerType: SERVICE_TRIGGER_TYPE,
    pub dwAction: SERVICE_TRIGGER_ACTION,
    pub pTriggerSubtype: *mut ::windows::runtime::GUID,
    pub cDataItems: u32,
    pub pDataItems: *mut SERVICE_TRIGGER_SPECIFIC_DATA_ITEM,
}
impl SERVICE_TRIGGER {}
impl ::std::default::Default for SERVICE_TRIGGER {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for SERVICE_TRIGGER {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("SERVICE_TRIGGER").field("dwTriggerType", &self.dwTriggerType).field("dwAction", &self.dwAction).field("pTriggerSubtype", &self.pTriggerSubtype).field("cDataItems", &self.cDataItems).field("pDataItems", &self.pDataItems).finish()
    }
}
impl ::std::cmp::PartialEq for SERVICE_TRIGGER {
    fn eq(&self, other: &Self) -> bool {
        self.dwTriggerType == other.dwTriggerType && self.dwAction == other.dwAction && self.pTriggerSubtype == other.pTriggerSubtype && self.cDataItems == other.cDataItems && self.pDataItems == other.pDataItems
    }
}
impl ::std::cmp::Eq for SERVICE_TRIGGER {}
unsafe impl ::windows::runtime::Abi for SERVICE_TRIGGER {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_System_Services`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct SERVICE_TRIGGER_ACTION(pub u32);
pub const SERVICE_TRIGGER_ACTION_SERVICE_START: SERVICE_TRIGGER_ACTION = SERVICE_TRIGGER_ACTION(1u32);
pub const SERVICE_TRIGGER_ACTION_SERVICE_STOP: SERVICE_TRIGGER_ACTION = SERVICE_TRIGGER_ACTION(2u32);
impl ::std::convert::From<u32> for SERVICE_TRIGGER_ACTION {
    fn from(value: u32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for SERVICE_TRIGGER_ACTION {
    type Abi = Self;
}
impl ::std::ops::BitOr for SERVICE_TRIGGER_ACTION {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self {
        Self(self.0 | rhs.0)
    }
}
impl ::std::ops::BitAnd for SERVICE_TRIGGER_ACTION {
    type Output = Self;
    fn bitand(self, rhs: Self) -> Self {
        Self(self.0 & rhs.0)
    }
}
impl ::std::ops::BitOrAssign for SERVICE_TRIGGER_ACTION {
    fn bitor_assign(&mut self, rhs: Self) {
        self.0.bitor_assign(rhs.0)
    }
}
impl ::std::ops::BitAndAssign for SERVICE_TRIGGER_ACTION {
    fn bitand_assign(&mut self, rhs: Self) {
        self.0.bitand_assign(rhs.0)
    }
}
impl ::std::ops::Not for SERVICE_TRIGGER_ACTION {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_System_Services`*"]
pub struct SERVICE_TRIGGER_CUSTOM_STATE_ID {
    pub Data: [u32; 2],
}
impl SERVICE_TRIGGER_CUSTOM_STATE_ID {}
impl ::std::default::Default for SERVICE_TRIGGER_CUSTOM_STATE_ID {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for SERVICE_TRIGGER_CUSTOM_STATE_ID {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("SERVICE_TRIGGER_CUSTOM_STATE_ID").field("Data", &self.Data).finish()
    }
}
impl ::std::cmp::PartialEq for SERVICE_TRIGGER_CUSTOM_STATE_ID {
    fn eq(&self, other: &Self) -> bool {
        self.Data == other.Data
    }
}
impl ::std::cmp::Eq for SERVICE_TRIGGER_CUSTOM_STATE_ID {}
unsafe impl ::windows::runtime::Abi for SERVICE_TRIGGER_CUSTOM_STATE_ID {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_System_Services`*"]
pub struct SERVICE_TRIGGER_INFO {
    pub cTriggers: u32,
    pub pTriggers: *mut SERVICE_TRIGGER,
    pub pReserved: *mut u8,
}
impl SERVICE_TRIGGER_INFO {}
impl ::std::default::Default for SERVICE_TRIGGER_INFO {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for SERVICE_TRIGGER_INFO {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("SERVICE_TRIGGER_INFO").field("cTriggers", &self.cTriggers).field("pTriggers", &self.pTriggers).field("pReserved", &self.pReserved).finish()
    }
}
impl ::std::cmp::PartialEq for SERVICE_TRIGGER_INFO {
    fn eq(&self, other: &Self) -> bool {
        self.cTriggers == other.cTriggers && self.pTriggers == other.pTriggers && self.pReserved == other.pReserved
    }
}
impl ::std::cmp::Eq for SERVICE_TRIGGER_INFO {}
unsafe impl ::windows::runtime::Abi for SERVICE_TRIGGER_INFO {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_System_Services`*"]
pub struct SERVICE_TRIGGER_SPECIFIC_DATA_ITEM {
    pub dwDataType: SERVICE_TRIGGER_SPECIFIC_DATA_ITEM_DATA_TYPE,
    pub cbData: u32,
    pub pData: *mut u8,
}
impl SERVICE_TRIGGER_SPECIFIC_DATA_ITEM {}
impl ::std::default::Default for SERVICE_TRIGGER_SPECIFIC_DATA_ITEM {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for SERVICE_TRIGGER_SPECIFIC_DATA_ITEM {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("SERVICE_TRIGGER_SPECIFIC_DATA_ITEM").field("dwDataType", &self.dwDataType).field("cbData", &self.cbData).field("pData", &self.pData).finish()
    }
}
impl ::std::cmp::PartialEq for SERVICE_TRIGGER_SPECIFIC_DATA_ITEM {
    fn eq(&self, other: &Self) -> bool {
        self.dwDataType == other.dwDataType && self.cbData == other.cbData && self.pData == other.pData
    }
}
impl ::std::cmp::Eq for SERVICE_TRIGGER_SPECIFIC_DATA_ITEM {}
unsafe impl ::windows::runtime::Abi for SERVICE_TRIGGER_SPECIFIC_DATA_ITEM {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_System_Services`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct SERVICE_TRIGGER_SPECIFIC_DATA_ITEM_DATA_TYPE(pub u32);
pub const SERVICE_TRIGGER_DATA_TYPE_BINARY: SERVICE_TRIGGER_SPECIFIC_DATA_ITEM_DATA_TYPE = SERVICE_TRIGGER_SPECIFIC_DATA_ITEM_DATA_TYPE(1u32);
pub const SERVICE_TRIGGER_DATA_TYPE_STRING: SERVICE_TRIGGER_SPECIFIC_DATA_ITEM_DATA_TYPE = SERVICE_TRIGGER_SPECIFIC_DATA_ITEM_DATA_TYPE(2u32);
pub const SERVICE_TRIGGER_DATA_TYPE_LEVEL: SERVICE_TRIGGER_SPECIFIC_DATA_ITEM_DATA_TYPE = SERVICE_TRIGGER_SPECIFIC_DATA_ITEM_DATA_TYPE(3u32);
pub const SERVICE_TRIGGER_DATA_TYPE_KEYWORD_ANY: SERVICE_TRIGGER_SPECIFIC_DATA_ITEM_DATA_TYPE = SERVICE_TRIGGER_SPECIFIC_DATA_ITEM_DATA_TYPE(4u32);
pub const SERVICE_TRIGGER_DATA_TYPE_KEYWORD_ALL: SERVICE_TRIGGER_SPECIFIC_DATA_ITEM_DATA_TYPE = SERVICE_TRIGGER_SPECIFIC_DATA_ITEM_DATA_TYPE(5u32);
impl ::std::convert::From<u32> for SERVICE_TRIGGER_SPECIFIC_DATA_ITEM_DATA_TYPE {
    fn from(value: u32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for SERVICE_TRIGGER_SPECIFIC_DATA_ITEM_DATA_TYPE {
    type Abi = Self;
}
impl ::std::ops::BitOr for SERVICE_TRIGGER_SPECIFIC_DATA_ITEM_DATA_TYPE {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self {
        Self(self.0 | rhs.0)
    }
}
impl ::std::ops::BitAnd for SERVICE_TRIGGER_SPECIFIC_DATA_ITEM_DATA_TYPE {
    type Output = Self;
    fn bitand(self, rhs: Self) -> Self {
        Self(self.0 & rhs.0)
    }
}
impl ::std::ops::BitOrAssign for SERVICE_TRIGGER_SPECIFIC_DATA_ITEM_DATA_TYPE {
    fn bitor_assign(&mut self, rhs: Self) {
        self.0.bitor_assign(rhs.0)
    }
}
impl ::std::ops::BitAndAssign for SERVICE_TRIGGER_SPECIFIC_DATA_ITEM_DATA_TYPE {
    fn bitand_assign(&mut self, rhs: Self) {
        self.0.bitand_assign(rhs.0)
    }
}
impl ::std::ops::Not for SERVICE_TRIGGER_SPECIFIC_DATA_ITEM_DATA_TYPE {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[doc = "*Required features: `Win32_System_Services`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct SERVICE_TRIGGER_TYPE(pub u32);
pub const SERVICE_TRIGGER_TYPE_CUSTOM: SERVICE_TRIGGER_TYPE = SERVICE_TRIGGER_TYPE(20u32);
pub const SERVICE_TRIGGER_TYPE_DEVICE_INTERFACE_ARRIVAL: SERVICE_TRIGGER_TYPE = SERVICE_TRIGGER_TYPE(1u32);
pub const SERVICE_TRIGGER_TYPE_DOMAIN_JOIN: SERVICE_TRIGGER_TYPE = SERVICE_TRIGGER_TYPE(3u32);
pub const SERVICE_TRIGGER_TYPE_FIREWALL_PORT_EVENT: SERVICE_TRIGGER_TYPE = SERVICE_TRIGGER_TYPE(4u32);
pub const SERVICE_TRIGGER_TYPE_GROUP_POLICY: SERVICE_TRIGGER_TYPE = SERVICE_TRIGGER_TYPE(5u32);
pub const SERVICE_TRIGGER_TYPE_IP_ADDRESS_AVAILABILITY: SERVICE_TRIGGER_TYPE = SERVICE_TRIGGER_TYPE(2u32);
pub const SERVICE_TRIGGER_TYPE_NETWORK_ENDPOINT: SERVICE_TRIGGER_TYPE = SERVICE_TRIGGER_TYPE(6u32);
impl ::std::convert::From<u32> for SERVICE_TRIGGER_TYPE {
    fn from(value: u32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for SERVICE_TRIGGER_TYPE {
    type Abi = Self;
}
impl ::std::ops::BitOr for SERVICE_TRIGGER_TYPE {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self {
        Self(self.0 | rhs.0)
    }
}
impl ::std::ops::BitAnd for SERVICE_TRIGGER_TYPE {
    type Output = Self;
    fn bitand(self, rhs: Self) -> Self {
        Self(self.0 & rhs.0)
    }
}
impl ::std::ops::BitOrAssign for SERVICE_TRIGGER_TYPE {
    fn bitor_assign(&mut self, rhs: Self) {
        self.0.bitor_assign(rhs.0)
    }
}
impl ::std::ops::BitAndAssign for SERVICE_TRIGGER_TYPE {
    fn bitand_assign(&mut self, rhs: Self) {
        self.0.bitand_assign(rhs.0)
    }
}
impl ::std::ops::Not for SERVICE_TRIGGER_TYPE {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[doc = "*Required features: `Win32_System_Services`*"]
pub const SERVICE_TRIGGER_TYPE_AGGREGATE: u32 = 30u32;
#[doc = "*Required features: `Win32_System_Services`*"]
pub const SERVICE_TRIGGER_TYPE_CUSTOM_SYSTEM_STATE_CHANGE: u32 = 7u32;
#[doc = "*Required features: `Win32_System_Services`*"]
pub const SERVICE_USER_DEFINED_CONTROL: u32 = 256u32;
#[doc = "*Required features: `Win32_System_Services`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn SetServiceBits<'a, Param0: ::windows::runtime::IntoParam<'a, SERVICE_STATUS_HANDLE>, Param2: ::windows::runtime::IntoParam<'a, super::super::Foundation::BOOL>, Param3: ::windows::runtime::IntoParam<'a, super::super::Foundation::BOOL>>(hservicestatus: Param0, dwservicebits: u32, bsetbitson: Param2, bupdateimmediately: Param3) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn SetServiceBits(hservicestatus: SERVICE_STATUS_HANDLE, dwservicebits: u32, bsetbitson: super::super::Foundation::BOOL, bupdateimmediately: super::super::Foundation::BOOL) -> super::super::Foundation::BOOL;
        }
        ::std::mem::transmute(SetServiceBits(hservicestatus.into_param().abi(), ::std::mem::transmute(dwservicebits), bsetbitson.into_param().abi(), bupdateimmediately.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_System_Services`, `Win32_Foundation`, `Win32_Security`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
#[inline]
pub unsafe fn SetServiceObjectSecurity<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Security::SC_HANDLE>>(hservice: Param0, dwsecurityinformation: super::super::Security::OBJECT_SECURITY_INFORMATION, lpsecuritydescriptor: *const super::super::Security::SECURITY_DESCRIPTOR) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn SetServiceObjectSecurity(hservice: super::super::Security::SC_HANDLE, dwsecurityinformation: super::super::Security::OBJECT_SECURITY_INFORMATION, lpsecuritydescriptor: *const super::super::Security::SECURITY_DESCRIPTOR) -> super::super::Foundation::BOOL;
        }
        ::std::mem::transmute(SetServiceObjectSecurity(hservice.into_param().abi(), ::std::mem::transmute(dwsecurityinformation), ::std::mem::transmute(lpsecuritydescriptor)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_System_Services`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn SetServiceStatus<'a, Param0: ::windows::runtime::IntoParam<'a, SERVICE_STATUS_HANDLE>>(hservicestatus: Param0, lpservicestatus: *const SERVICE_STATUS) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn SetServiceStatus(hservicestatus: SERVICE_STATUS_HANDLE, lpservicestatus: *const SERVICE_STATUS) -> super::super::Foundation::BOOL;
        }
        ::std::mem::transmute(SetServiceStatus(hservicestatus.into_param().abi(), ::std::mem::transmute(lpservicestatus)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_System_Services`, `Win32_Foundation`, `Win32_Security`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
#[inline]
pub unsafe fn StartServiceA<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Security::SC_HANDLE>>(hservice: Param0, dwnumserviceargs: u32, lpserviceargvectors: *const super::super::Foundation::PSTR) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn StartServiceA(hservice: super::super::Security::SC_HANDLE, dwnumserviceargs: u32, lpserviceargvectors: *const super::super::Foundation::PSTR) -> super::super::Foundation::BOOL;
        }
        ::std::mem::transmute(StartServiceA(hservice.into_param().abi(), ::std::mem::transmute(dwnumserviceargs), ::std::mem::transmute(lpserviceargvectors)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_System_Services`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn StartServiceCtrlDispatcherA(lpservicestarttable: *const SERVICE_TABLE_ENTRYA) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn StartServiceCtrlDispatcherA(lpservicestarttable: *const ::std::mem::ManuallyDrop<SERVICE_TABLE_ENTRYA>) -> super::super::Foundation::BOOL;
        }
        ::std::mem::transmute(StartServiceCtrlDispatcherA(::std::mem::transmute(lpservicestarttable)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_System_Services`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn StartServiceCtrlDispatcherW(lpservicestarttable: *const SERVICE_TABLE_ENTRYW) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn StartServiceCtrlDispatcherW(lpservicestarttable: *const ::std::mem::ManuallyDrop<SERVICE_TABLE_ENTRYW>) -> super::super::Foundation::BOOL;
        }
        ::std::mem::transmute(StartServiceCtrlDispatcherW(::std::mem::transmute(lpservicestarttable)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_System_Services`, `Win32_Foundation`, `Win32_Security`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
#[inline]
pub unsafe fn StartServiceW<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Security::SC_HANDLE>>(hservice: Param0, dwnumserviceargs: u32, lpserviceargvectors: *const super::super::Foundation::PWSTR) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn StartServiceW(hservice: super::super::Security::SC_HANDLE, dwnumserviceargs: u32, lpserviceargvectors: *const super::super::Foundation::PWSTR) -> super::super::Foundation::BOOL;
        }
        ::std::mem::transmute(StartServiceW(hservice.into_param().abi(), ::std::mem::transmute(dwnumserviceargs), ::std::mem::transmute(lpserviceargvectors)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
pub const USER_POLICY_PRESENT_GUID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1425753800, 61577, 17996, [177, 253, 89, 209, 182, 44, 59, 80]);
#[doc = "*Required features: `Win32_System_Services`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn UnlockServiceDatabase(sclock: *const ::std::ffi::c_void) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn UnlockServiceDatabase(sclock: *const ::std::ffi::c_void) -> super::super::Foundation::BOOL;
        }
        ::std::mem::transmute(UnlockServiceDatabase(::std::mem::transmute(sclock)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_System_Services`, `Win32_Foundation`, `Win32_Security`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
#[inline]
pub unsafe fn WaitServiceState<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Security::SC_HANDLE>, Param3: ::windows::runtime::IntoParam<'a, super::super::Foundation::HANDLE>>(hservice: Param0, dwnotify: u32, dwtimeout: u32, hcancelevent: Param3) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn WaitServiceState(hservice: super::super::Security::SC_HANDLE, dwnotify: u32, dwtimeout: u32, hcancelevent: super::super::Foundation::HANDLE) -> u32;
        }
        ::std::mem::transmute(WaitServiceState(hservice.into_param().abi(), ::std::mem::transmute(dwnotify), ::std::mem::transmute(dwtimeout), hcancelevent.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[repr(C)]
#[derive(:: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug, :: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy)]
pub struct _SC_NOTIFICATION_REGISTRATION(pub u8);

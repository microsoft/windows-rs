#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, clashing_extern_declarations, clippy::all)]
pub const CUSTOM_SYSTEM_STATE_CHANGE_EVENT_GUID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x2d7a2816_0c5e_45fc_9ce7_570e5ecde9c9);
#[doc = "*Required features: 'Win32_System_Services', 'Win32_Foundation', 'Win32_Security'*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
#[inline]
pub unsafe fn ChangeServiceConfig2A<'a, Param0: ::windows::core::IntoParam<'a, super::super::Security::SC_HANDLE>>(hservice: Param0, dwinfolevel: SERVICE_CONFIG, lpinfo: *const ::core::ffi::c_void) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn ChangeServiceConfig2A(hservice: super::super::Security::SC_HANDLE, dwinfolevel: SERVICE_CONFIG, lpinfo: *const ::core::ffi::c_void) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(ChangeServiceConfig2A(hservice.into_param().abi(), ::core::mem::transmute(dwinfolevel), ::core::mem::transmute(lpinfo)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_System_Services', 'Win32_Foundation', 'Win32_Security'*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
#[inline]
pub unsafe fn ChangeServiceConfig2W<'a, Param0: ::windows::core::IntoParam<'a, super::super::Security::SC_HANDLE>>(hservice: Param0, dwinfolevel: SERVICE_CONFIG, lpinfo: *const ::core::ffi::c_void) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn ChangeServiceConfig2W(hservice: super::super::Security::SC_HANDLE, dwinfolevel: SERVICE_CONFIG, lpinfo: *const ::core::ffi::c_void) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(ChangeServiceConfig2W(hservice.into_param().abi(), ::core::mem::transmute(dwinfolevel), ::core::mem::transmute(lpinfo)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_System_Services', 'Win32_Foundation', 'Win32_Security'*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
#[inline]
pub unsafe fn ChangeServiceConfigA<'a, Param0: ::windows::core::IntoParam<'a, super::super::Security::SC_HANDLE>, Param4: ::windows::core::IntoParam<'a, super::super::Foundation::PSTR>, Param5: ::windows::core::IntoParam<'a, super::super::Foundation::PSTR>, Param7: ::windows::core::IntoParam<'a, super::super::Foundation::PSTR>, Param8: ::windows::core::IntoParam<'a, super::super::Foundation::PSTR>, Param9: ::windows::core::IntoParam<'a, super::super::Foundation::PSTR>, Param10: ::windows::core::IntoParam<'a, super::super::Foundation::PSTR>>(
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
            fn ChangeServiceConfigA(hservice: super::super::Security::SC_HANDLE, dwservicetype: u32, dwstarttype: SERVICE_START_TYPE, dwerrorcontrol: SERVICE_ERROR, lpbinarypathname: super::super::Foundation::PSTR, lploadordergroup: super::super::Foundation::PSTR, lpdwtagid: *mut u32, lpdependencies: super::super::Foundation::PSTR, lpservicestartname: super::super::Foundation::PSTR, lppassword: super::super::Foundation::PSTR, lpdisplayname: super::super::Foundation::PSTR) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(ChangeServiceConfigA(hservice.into_param().abi(), ::core::mem::transmute(dwservicetype), ::core::mem::transmute(dwstarttype), ::core::mem::transmute(dwerrorcontrol), lpbinarypathname.into_param().abi(), lploadordergroup.into_param().abi(), ::core::mem::transmute(lpdwtagid), lpdependencies.into_param().abi(), lpservicestartname.into_param().abi(), lppassword.into_param().abi(), lpdisplayname.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_System_Services', 'Win32_Foundation', 'Win32_Security'*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
#[inline]
pub unsafe fn ChangeServiceConfigW<'a, Param0: ::windows::core::IntoParam<'a, super::super::Security::SC_HANDLE>, Param4: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>, Param5: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>, Param7: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>, Param8: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>, Param9: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>, Param10: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(
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
            fn ChangeServiceConfigW(hservice: super::super::Security::SC_HANDLE, dwservicetype: u32, dwstarttype: SERVICE_START_TYPE, dwerrorcontrol: SERVICE_ERROR, lpbinarypathname: super::super::Foundation::PWSTR, lploadordergroup: super::super::Foundation::PWSTR, lpdwtagid: *mut u32, lpdependencies: super::super::Foundation::PWSTR, lpservicestartname: super::super::Foundation::PWSTR, lppassword: super::super::Foundation::PWSTR, lpdisplayname: super::super::Foundation::PWSTR) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(ChangeServiceConfigW(hservice.into_param().abi(), ::core::mem::transmute(dwservicetype), ::core::mem::transmute(dwstarttype), ::core::mem::transmute(dwerrorcontrol), lpbinarypathname.into_param().abi(), lploadordergroup.into_param().abi(), ::core::mem::transmute(lpdwtagid), lpdependencies.into_param().abi(), lpservicestartname.into_param().abi(), lppassword.into_param().abi(), lpdisplayname.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_System_Services', 'Win32_Foundation', 'Win32_Security'*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
#[inline]
pub unsafe fn CloseServiceHandle<'a, Param0: ::windows::core::IntoParam<'a, super::super::Security::SC_HANDLE>>(hscobject: Param0) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn CloseServiceHandle(hscobject: super::super::Security::SC_HANDLE) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(CloseServiceHandle(hscobject.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_System_Services', 'Win32_Foundation', 'Win32_Security'*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
#[inline]
pub unsafe fn ControlService<'a, Param0: ::windows::core::IntoParam<'a, super::super::Security::SC_HANDLE>>(hservice: Param0, dwcontrol: u32, lpservicestatus: *mut SERVICE_STATUS) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn ControlService(hservice: super::super::Security::SC_HANDLE, dwcontrol: u32, lpservicestatus: *mut SERVICE_STATUS) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(ControlService(hservice.into_param().abi(), ::core::mem::transmute(dwcontrol), ::core::mem::transmute(lpservicestatus)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_System_Services', 'Win32_Foundation', 'Win32_Security'*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
#[inline]
pub unsafe fn ControlServiceExA<'a, Param0: ::windows::core::IntoParam<'a, super::super::Security::SC_HANDLE>>(hservice: Param0, dwcontrol: u32, dwinfolevel: u32, pcontrolparams: *mut ::core::ffi::c_void) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn ControlServiceExA(hservice: super::super::Security::SC_HANDLE, dwcontrol: u32, dwinfolevel: u32, pcontrolparams: *mut ::core::ffi::c_void) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(ControlServiceExA(hservice.into_param().abi(), ::core::mem::transmute(dwcontrol), ::core::mem::transmute(dwinfolevel), ::core::mem::transmute(pcontrolparams)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_System_Services', 'Win32_Foundation', 'Win32_Security'*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
#[inline]
pub unsafe fn ControlServiceExW<'a, Param0: ::windows::core::IntoParam<'a, super::super::Security::SC_HANDLE>>(hservice: Param0, dwcontrol: u32, dwinfolevel: u32, pcontrolparams: *mut ::core::ffi::c_void) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn ControlServiceExW(hservice: super::super::Security::SC_HANDLE, dwcontrol: u32, dwinfolevel: u32, pcontrolparams: *mut ::core::ffi::c_void) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(ControlServiceExW(hservice.into_param().abi(), ::core::mem::transmute(dwcontrol), ::core::mem::transmute(dwinfolevel), ::core::mem::transmute(pcontrolparams)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_System_Services', 'Win32_Foundation', 'Win32_Security'*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
#[inline]
pub unsafe fn CreateServiceA<'a, Param0: ::windows::core::IntoParam<'a, super::super::Security::SC_HANDLE>, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::PSTR>, Param2: ::windows::core::IntoParam<'a, super::super::Foundation::PSTR>, Param7: ::windows::core::IntoParam<'a, super::super::Foundation::PSTR>, Param8: ::windows::core::IntoParam<'a, super::super::Foundation::PSTR>, Param10: ::windows::core::IntoParam<'a, super::super::Foundation::PSTR>, Param11: ::windows::core::IntoParam<'a, super::super::Foundation::PSTR>, Param12: ::windows::core::IntoParam<'a, super::super::Foundation::PSTR>>(
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
            fn CreateServiceA(hscmanager: super::super::Security::SC_HANDLE, lpservicename: super::super::Foundation::PSTR, lpdisplayname: super::super::Foundation::PSTR, dwdesiredaccess: u32, dwservicetype: ENUM_SERVICE_TYPE, dwstarttype: SERVICE_START_TYPE, dwerrorcontrol: SERVICE_ERROR, lpbinarypathname: super::super::Foundation::PSTR, lploadordergroup: super::super::Foundation::PSTR, lpdwtagid: *mut u32, lpdependencies: super::super::Foundation::PSTR, lpservicestartname: super::super::Foundation::PSTR, lppassword: super::super::Foundation::PSTR) -> super::super::Security::SC_HANDLE;
        }
        ::core::mem::transmute(CreateServiceA(
            hscmanager.into_param().abi(),
            lpservicename.into_param().abi(),
            lpdisplayname.into_param().abi(),
            ::core::mem::transmute(dwdesiredaccess),
            ::core::mem::transmute(dwservicetype),
            ::core::mem::transmute(dwstarttype),
            ::core::mem::transmute(dwerrorcontrol),
            lpbinarypathname.into_param().abi(),
            lploadordergroup.into_param().abi(),
            ::core::mem::transmute(lpdwtagid),
            lpdependencies.into_param().abi(),
            lpservicestartname.into_param().abi(),
            lppassword.into_param().abi(),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_System_Services', 'Win32_Foundation', 'Win32_Security'*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
#[inline]
pub unsafe fn CreateServiceW<'a, Param0: ::windows::core::IntoParam<'a, super::super::Security::SC_HANDLE>, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>, Param2: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>, Param7: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>, Param8: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>, Param10: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>, Param11: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>, Param12: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(
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
            fn CreateServiceW(hscmanager: super::super::Security::SC_HANDLE, lpservicename: super::super::Foundation::PWSTR, lpdisplayname: super::super::Foundation::PWSTR, dwdesiredaccess: u32, dwservicetype: ENUM_SERVICE_TYPE, dwstarttype: SERVICE_START_TYPE, dwerrorcontrol: SERVICE_ERROR, lpbinarypathname: super::super::Foundation::PWSTR, lploadordergroup: super::super::Foundation::PWSTR, lpdwtagid: *mut u32, lpdependencies: super::super::Foundation::PWSTR, lpservicestartname: super::super::Foundation::PWSTR, lppassword: super::super::Foundation::PWSTR) -> super::super::Security::SC_HANDLE;
        }
        ::core::mem::transmute(CreateServiceW(
            hscmanager.into_param().abi(),
            lpservicename.into_param().abi(),
            lpdisplayname.into_param().abi(),
            ::core::mem::transmute(dwdesiredaccess),
            ::core::mem::transmute(dwservicetype),
            ::core::mem::transmute(dwstarttype),
            ::core::mem::transmute(dwerrorcontrol),
            lpbinarypathname.into_param().abi(),
            lploadordergroup.into_param().abi(),
            ::core::mem::transmute(lpdwtagid),
            lpdependencies.into_param().abi(),
            lpservicestartname.into_param().abi(),
            lppassword.into_param().abi(),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
pub const DOMAIN_JOIN_GUID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x1ce20aba_9851_4421_9430_1ddeb766e809);
pub const DOMAIN_LEAVE_GUID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xddaf516e_58c2_4866_9574_c3b615d42ea1);
#[doc = "*Required features: 'Win32_System_Services', 'Win32_Foundation', 'Win32_Security'*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
#[inline]
pub unsafe fn DeleteService<'a, Param0: ::windows::core::IntoParam<'a, super::super::Security::SC_HANDLE>>(hservice: Param0) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn DeleteService(hservice: super::super::Security::SC_HANDLE) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(DeleteService(hservice.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_System_Services'*"]
pub type ENUM_SERVICE_STATE = u32;
#[doc = "*Required features: 'Win32_System_Services'*"]
pub const SERVICE_ACTIVE: ENUM_SERVICE_STATE = 1u32;
#[doc = "*Required features: 'Win32_System_Services'*"]
pub const SERVICE_INACTIVE: ENUM_SERVICE_STATE = 2u32;
#[doc = "*Required features: 'Win32_System_Services'*"]
pub const SERVICE_STATE_ALL: ENUM_SERVICE_STATE = 3u32;
#[repr(C)]
#[doc = "*Required features: 'Win32_System_Services', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
pub struct ENUM_SERVICE_STATUSA {
    pub lpServiceName: super::super::Foundation::PSTR,
    pub lpDisplayName: super::super::Foundation::PSTR,
    pub ServiceStatus: SERVICE_STATUS,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for ENUM_SERVICE_STATUSA {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for ENUM_SERVICE_STATUSA {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for ENUM_SERVICE_STATUSA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("ENUM_SERVICE_STATUSA").field("lpServiceName", &self.lpServiceName).field("lpDisplayName", &self.lpDisplayName).field("ServiceStatus", &self.ServiceStatus).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for ENUM_SERVICE_STATUSA {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for ENUM_SERVICE_STATUSA {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<ENUM_SERVICE_STATUSA>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for ENUM_SERVICE_STATUSA {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for ENUM_SERVICE_STATUSA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: 'Win32_System_Services', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
pub struct ENUM_SERVICE_STATUSW {
    pub lpServiceName: super::super::Foundation::PWSTR,
    pub lpDisplayName: super::super::Foundation::PWSTR,
    pub ServiceStatus: SERVICE_STATUS,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for ENUM_SERVICE_STATUSW {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for ENUM_SERVICE_STATUSW {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for ENUM_SERVICE_STATUSW {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("ENUM_SERVICE_STATUSW").field("lpServiceName", &self.lpServiceName).field("lpDisplayName", &self.lpDisplayName).field("ServiceStatus", &self.ServiceStatus).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for ENUM_SERVICE_STATUSW {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for ENUM_SERVICE_STATUSW {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<ENUM_SERVICE_STATUSW>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for ENUM_SERVICE_STATUSW {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for ENUM_SERVICE_STATUSW {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: 'Win32_System_Services', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
pub struct ENUM_SERVICE_STATUS_PROCESSA {
    pub lpServiceName: super::super::Foundation::PSTR,
    pub lpDisplayName: super::super::Foundation::PSTR,
    pub ServiceStatusProcess: SERVICE_STATUS_PROCESS,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for ENUM_SERVICE_STATUS_PROCESSA {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for ENUM_SERVICE_STATUS_PROCESSA {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for ENUM_SERVICE_STATUS_PROCESSA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("ENUM_SERVICE_STATUS_PROCESSA").field("lpServiceName", &self.lpServiceName).field("lpDisplayName", &self.lpDisplayName).field("ServiceStatusProcess", &self.ServiceStatusProcess).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for ENUM_SERVICE_STATUS_PROCESSA {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for ENUM_SERVICE_STATUS_PROCESSA {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<ENUM_SERVICE_STATUS_PROCESSA>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for ENUM_SERVICE_STATUS_PROCESSA {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for ENUM_SERVICE_STATUS_PROCESSA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: 'Win32_System_Services', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
pub struct ENUM_SERVICE_STATUS_PROCESSW {
    pub lpServiceName: super::super::Foundation::PWSTR,
    pub lpDisplayName: super::super::Foundation::PWSTR,
    pub ServiceStatusProcess: SERVICE_STATUS_PROCESS,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for ENUM_SERVICE_STATUS_PROCESSW {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for ENUM_SERVICE_STATUS_PROCESSW {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for ENUM_SERVICE_STATUS_PROCESSW {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("ENUM_SERVICE_STATUS_PROCESSW").field("lpServiceName", &self.lpServiceName).field("lpDisplayName", &self.lpDisplayName).field("ServiceStatusProcess", &self.ServiceStatusProcess).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for ENUM_SERVICE_STATUS_PROCESSW {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for ENUM_SERVICE_STATUS_PROCESSW {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<ENUM_SERVICE_STATUS_PROCESSW>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for ENUM_SERVICE_STATUS_PROCESSW {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for ENUM_SERVICE_STATUS_PROCESSW {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: 'Win32_System_Services'*"]
pub type ENUM_SERVICE_TYPE = u32;
#[doc = "*Required features: 'Win32_System_Services'*"]
pub const SERVICE_DRIVER: ENUM_SERVICE_TYPE = 11u32;
#[doc = "*Required features: 'Win32_System_Services'*"]
pub const SERVICE_FILE_SYSTEM_DRIVER_: ENUM_SERVICE_TYPE = 2u32;
#[doc = "*Required features: 'Win32_System_Services'*"]
pub const SERVICE_KERNEL_DRIVER: ENUM_SERVICE_TYPE = 1u32;
#[doc = "*Required features: 'Win32_System_Services'*"]
pub const SERVICE_WIN32: ENUM_SERVICE_TYPE = 48u32;
#[doc = "*Required features: 'Win32_System_Services'*"]
pub const SERVICE_WIN32_OWN_PROCESS_: ENUM_SERVICE_TYPE = 16u32;
#[doc = "*Required features: 'Win32_System_Services'*"]
pub const SERVICE_WIN32_SHARE_PROCESS: ENUM_SERVICE_TYPE = 32u32;
#[doc = "*Required features: 'Win32_System_Services'*"]
pub const SERVICE_ADAPTER: ENUM_SERVICE_TYPE = 4u32;
#[doc = "*Required features: 'Win32_System_Services'*"]
pub const SERVICE_FILE_SYSTEM_DRIVER: ENUM_SERVICE_TYPE = 2u32;
#[doc = "*Required features: 'Win32_System_Services'*"]
pub const SERVICE_RECOGNIZER_DRIVER: ENUM_SERVICE_TYPE = 8u32;
#[doc = "*Required features: 'Win32_System_Services'*"]
pub const SERVICE_WIN32_OWN_PROCESS: ENUM_SERVICE_TYPE = 16u32;
#[doc = "*Required features: 'Win32_System_Services'*"]
pub const SERVICE_USER_OWN_PROCESS: ENUM_SERVICE_TYPE = 80u32;
#[doc = "*Required features: 'Win32_System_Services'*"]
pub const SERVICE_USER_SHARE_PROCESS: ENUM_SERVICE_TYPE = 96u32;
#[doc = "*Required features: 'Win32_System_Services', 'Win32_Foundation', 'Win32_Security'*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
#[inline]
pub unsafe fn EnumDependentServicesA<'a, Param0: ::windows::core::IntoParam<'a, super::super::Security::SC_HANDLE>>(hservice: Param0, dwservicestate: ENUM_SERVICE_STATE, lpservices: *mut ENUM_SERVICE_STATUSA, cbbufsize: u32, pcbbytesneeded: *mut u32, lpservicesreturned: *mut u32) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn EnumDependentServicesA(hservice: super::super::Security::SC_HANDLE, dwservicestate: ENUM_SERVICE_STATE, lpservices: *mut ENUM_SERVICE_STATUSA, cbbufsize: u32, pcbbytesneeded: *mut u32, lpservicesreturned: *mut u32) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(EnumDependentServicesA(hservice.into_param().abi(), ::core::mem::transmute(dwservicestate), ::core::mem::transmute(lpservices), ::core::mem::transmute(cbbufsize), ::core::mem::transmute(pcbbytesneeded), ::core::mem::transmute(lpservicesreturned)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_System_Services', 'Win32_Foundation', 'Win32_Security'*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
#[inline]
pub unsafe fn EnumDependentServicesW<'a, Param0: ::windows::core::IntoParam<'a, super::super::Security::SC_HANDLE>>(hservice: Param0, dwservicestate: ENUM_SERVICE_STATE, lpservices: *mut ENUM_SERVICE_STATUSW, cbbufsize: u32, pcbbytesneeded: *mut u32, lpservicesreturned: *mut u32) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn EnumDependentServicesW(hservice: super::super::Security::SC_HANDLE, dwservicestate: ENUM_SERVICE_STATE, lpservices: *mut ENUM_SERVICE_STATUSW, cbbufsize: u32, pcbbytesneeded: *mut u32, lpservicesreturned: *mut u32) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(EnumDependentServicesW(hservice.into_param().abi(), ::core::mem::transmute(dwservicestate), ::core::mem::transmute(lpservices), ::core::mem::transmute(cbbufsize), ::core::mem::transmute(pcbbytesneeded), ::core::mem::transmute(lpservicesreturned)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_System_Services', 'Win32_Foundation', 'Win32_Security'*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
#[inline]
pub unsafe fn EnumServicesStatusA<'a, Param0: ::windows::core::IntoParam<'a, super::super::Security::SC_HANDLE>>(hscmanager: Param0, dwservicetype: ENUM_SERVICE_TYPE, dwservicestate: ENUM_SERVICE_STATE, lpservices: *mut ENUM_SERVICE_STATUSA, cbbufsize: u32, pcbbytesneeded: *mut u32, lpservicesreturned: *mut u32, lpresumehandle: *mut u32) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn EnumServicesStatusA(hscmanager: super::super::Security::SC_HANDLE, dwservicetype: ENUM_SERVICE_TYPE, dwservicestate: ENUM_SERVICE_STATE, lpservices: *mut ENUM_SERVICE_STATUSA, cbbufsize: u32, pcbbytesneeded: *mut u32, lpservicesreturned: *mut u32, lpresumehandle: *mut u32) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(EnumServicesStatusA(hscmanager.into_param().abi(), ::core::mem::transmute(dwservicetype), ::core::mem::transmute(dwservicestate), ::core::mem::transmute(lpservices), ::core::mem::transmute(cbbufsize), ::core::mem::transmute(pcbbytesneeded), ::core::mem::transmute(lpservicesreturned), ::core::mem::transmute(lpresumehandle)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_System_Services', 'Win32_Foundation', 'Win32_Security'*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
#[inline]
pub unsafe fn EnumServicesStatusExA<'a, Param0: ::windows::core::IntoParam<'a, super::super::Security::SC_HANDLE>, Param9: ::windows::core::IntoParam<'a, super::super::Foundation::PSTR>>(hscmanager: Param0, infolevel: SC_ENUM_TYPE, dwservicetype: ENUM_SERVICE_TYPE, dwservicestate: ENUM_SERVICE_STATE, lpservices: *mut u8, cbbufsize: u32, pcbbytesneeded: *mut u32, lpservicesreturned: *mut u32, lpresumehandle: *mut u32, pszgroupname: Param9) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn EnumServicesStatusExA(hscmanager: super::super::Security::SC_HANDLE, infolevel: SC_ENUM_TYPE, dwservicetype: ENUM_SERVICE_TYPE, dwservicestate: ENUM_SERVICE_STATE, lpservices: *mut u8, cbbufsize: u32, pcbbytesneeded: *mut u32, lpservicesreturned: *mut u32, lpresumehandle: *mut u32, pszgroupname: super::super::Foundation::PSTR) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(EnumServicesStatusExA(hscmanager.into_param().abi(), ::core::mem::transmute(infolevel), ::core::mem::transmute(dwservicetype), ::core::mem::transmute(dwservicestate), ::core::mem::transmute(lpservices), ::core::mem::transmute(cbbufsize), ::core::mem::transmute(pcbbytesneeded), ::core::mem::transmute(lpservicesreturned), ::core::mem::transmute(lpresumehandle), pszgroupname.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_System_Services', 'Win32_Foundation', 'Win32_Security'*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
#[inline]
pub unsafe fn EnumServicesStatusExW<'a, Param0: ::windows::core::IntoParam<'a, super::super::Security::SC_HANDLE>, Param9: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(hscmanager: Param0, infolevel: SC_ENUM_TYPE, dwservicetype: ENUM_SERVICE_TYPE, dwservicestate: ENUM_SERVICE_STATE, lpservices: *mut u8, cbbufsize: u32, pcbbytesneeded: *mut u32, lpservicesreturned: *mut u32, lpresumehandle: *mut u32, pszgroupname: Param9) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn EnumServicesStatusExW(hscmanager: super::super::Security::SC_HANDLE, infolevel: SC_ENUM_TYPE, dwservicetype: ENUM_SERVICE_TYPE, dwservicestate: ENUM_SERVICE_STATE, lpservices: *mut u8, cbbufsize: u32, pcbbytesneeded: *mut u32, lpservicesreturned: *mut u32, lpresumehandle: *mut u32, pszgroupname: super::super::Foundation::PWSTR) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(EnumServicesStatusExW(hscmanager.into_param().abi(), ::core::mem::transmute(infolevel), ::core::mem::transmute(dwservicetype), ::core::mem::transmute(dwservicestate), ::core::mem::transmute(lpservices), ::core::mem::transmute(cbbufsize), ::core::mem::transmute(pcbbytesneeded), ::core::mem::transmute(lpservicesreturned), ::core::mem::transmute(lpresumehandle), pszgroupname.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_System_Services', 'Win32_Foundation', 'Win32_Security'*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
#[inline]
pub unsafe fn EnumServicesStatusW<'a, Param0: ::windows::core::IntoParam<'a, super::super::Security::SC_HANDLE>>(hscmanager: Param0, dwservicetype: ENUM_SERVICE_TYPE, dwservicestate: ENUM_SERVICE_STATE, lpservices: *mut ENUM_SERVICE_STATUSW, cbbufsize: u32, pcbbytesneeded: *mut u32, lpservicesreturned: *mut u32, lpresumehandle: *mut u32) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn EnumServicesStatusW(hscmanager: super::super::Security::SC_HANDLE, dwservicetype: ENUM_SERVICE_TYPE, dwservicestate: ENUM_SERVICE_STATE, lpservices: *mut ENUM_SERVICE_STATUSW, cbbufsize: u32, pcbbytesneeded: *mut u32, lpservicesreturned: *mut u32, lpresumehandle: *mut u32) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(EnumServicesStatusW(hscmanager.into_param().abi(), ::core::mem::transmute(dwservicetype), ::core::mem::transmute(dwservicestate), ::core::mem::transmute(lpservices), ::core::mem::transmute(cbbufsize), ::core::mem::transmute(pcbbytesneeded), ::core::mem::transmute(lpservicesreturned), ::core::mem::transmute(lpresumehandle)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
pub const FIREWALL_PORT_CLOSE_GUID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xa144ed38_8e12_4de4_9d96_e64740b1a524);
pub const FIREWALL_PORT_OPEN_GUID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xb7569e07_8421_4ee0_ad10_86915afdad09);
#[doc = "*Required features: 'Win32_System_Services', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn GetServiceDirectory<'a, Param0: ::windows::core::IntoParam<'a, SERVICE_STATUS_HANDLE>>(hservicestatus: Param0, edirectorytype: SERVICE_DIRECTORY_TYPE, lppathbuffer: super::super::Foundation::PWSTR, cchpathbufferlength: u32, lpcchrequiredbufferlength: *mut u32) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn GetServiceDirectory(hservicestatus: SERVICE_STATUS_HANDLE, edirectorytype: SERVICE_DIRECTORY_TYPE, lppathbuffer: super::super::Foundation::PWSTR, cchpathbufferlength: u32, lpcchrequiredbufferlength: *mut u32) -> u32;
        }
        ::core::mem::transmute(GetServiceDirectory(hservicestatus.into_param().abi(), ::core::mem::transmute(edirectorytype), ::core::mem::transmute(lppathbuffer), ::core::mem::transmute(cchpathbufferlength), ::core::mem::transmute(lpcchrequiredbufferlength)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_System_Services', 'Win32_Foundation', 'Win32_Security'*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
#[inline]
pub unsafe fn GetServiceDisplayNameA<'a, Param0: ::windows::core::IntoParam<'a, super::super::Security::SC_HANDLE>, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::PSTR>>(hscmanager: Param0, lpservicename: Param1, lpdisplayname: super::super::Foundation::PSTR, lpcchbuffer: *mut u32) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn GetServiceDisplayNameA(hscmanager: super::super::Security::SC_HANDLE, lpservicename: super::super::Foundation::PSTR, lpdisplayname: super::super::Foundation::PSTR, lpcchbuffer: *mut u32) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(GetServiceDisplayNameA(hscmanager.into_param().abi(), lpservicename.into_param().abi(), ::core::mem::transmute(lpdisplayname), ::core::mem::transmute(lpcchbuffer)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_System_Services', 'Win32_Foundation', 'Win32_Security'*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
#[inline]
pub unsafe fn GetServiceDisplayNameW<'a, Param0: ::windows::core::IntoParam<'a, super::super::Security::SC_HANDLE>, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(hscmanager: Param0, lpservicename: Param1, lpdisplayname: super::super::Foundation::PWSTR, lpcchbuffer: *mut u32) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn GetServiceDisplayNameW(hscmanager: super::super::Security::SC_HANDLE, lpservicename: super::super::Foundation::PWSTR, lpdisplayname: super::super::Foundation::PWSTR, lpcchbuffer: *mut u32) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(GetServiceDisplayNameW(hscmanager.into_param().abi(), lpservicename.into_param().abi(), ::core::mem::transmute(lpdisplayname), ::core::mem::transmute(lpcchbuffer)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_System_Services', 'Win32_Foundation', 'Win32_Security'*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
#[inline]
pub unsafe fn GetServiceKeyNameA<'a, Param0: ::windows::core::IntoParam<'a, super::super::Security::SC_HANDLE>, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::PSTR>>(hscmanager: Param0, lpdisplayname: Param1, lpservicename: super::super::Foundation::PSTR, lpcchbuffer: *mut u32) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn GetServiceKeyNameA(hscmanager: super::super::Security::SC_HANDLE, lpdisplayname: super::super::Foundation::PSTR, lpservicename: super::super::Foundation::PSTR, lpcchbuffer: *mut u32) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(GetServiceKeyNameA(hscmanager.into_param().abi(), lpdisplayname.into_param().abi(), ::core::mem::transmute(lpservicename), ::core::mem::transmute(lpcchbuffer)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_System_Services', 'Win32_Foundation', 'Win32_Security'*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
#[inline]
pub unsafe fn GetServiceKeyNameW<'a, Param0: ::windows::core::IntoParam<'a, super::super::Security::SC_HANDLE>, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(hscmanager: Param0, lpdisplayname: Param1, lpservicename: super::super::Foundation::PWSTR, lpcchbuffer: *mut u32) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn GetServiceKeyNameW(hscmanager: super::super::Security::SC_HANDLE, lpdisplayname: super::super::Foundation::PWSTR, lpservicename: super::super::Foundation::PWSTR, lpcchbuffer: *mut u32) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(GetServiceKeyNameW(hscmanager.into_param().abi(), lpdisplayname.into_param().abi(), ::core::mem::transmute(lpservicename), ::core::mem::transmute(lpcchbuffer)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_System_Services', 'Win32_System_Registry'*"]
#[cfg(feature = "Win32_System_Registry")]
#[inline]
pub unsafe fn GetServiceRegistryStateKey<'a, Param0: ::windows::core::IntoParam<'a, SERVICE_STATUS_HANDLE>>(servicestatushandle: Param0, statetype: SERVICE_REGISTRY_STATE_TYPE, accessmask: u32, servicestatekey: *mut super::Registry::HKEY) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn GetServiceRegistryStateKey(servicestatushandle: SERVICE_STATUS_HANDLE, statetype: SERVICE_REGISTRY_STATE_TYPE, accessmask: u32, servicestatekey: *mut super::Registry::HKEY) -> u32;
        }
        ::core::mem::transmute(GetServiceRegistryStateKey(servicestatushandle.into_param().abi(), ::core::mem::transmute(statetype), ::core::mem::transmute(accessmask), ::core::mem::transmute(servicestatekey)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_System_Services', 'Win32_Foundation', 'Win32_Security'*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
#[inline]
pub unsafe fn GetSharedServiceDirectory<'a, Param0: ::windows::core::IntoParam<'a, super::super::Security::SC_HANDLE>>(servicehandle: Param0, directorytype: SERVICE_SHARED_DIRECTORY_TYPE, pathbuffer: super::super::Foundation::PWSTR, pathbufferlength: u32, requiredbufferlength: *mut u32) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn GetSharedServiceDirectory(servicehandle: super::super::Security::SC_HANDLE, directorytype: SERVICE_SHARED_DIRECTORY_TYPE, pathbuffer: super::super::Foundation::PWSTR, pathbufferlength: u32, requiredbufferlength: *mut u32) -> u32;
        }
        ::core::mem::transmute(GetSharedServiceDirectory(servicehandle.into_param().abi(), ::core::mem::transmute(directorytype), ::core::mem::transmute(pathbuffer), ::core::mem::transmute(pathbufferlength), ::core::mem::transmute(requiredbufferlength)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_System_Services', 'Win32_Security', 'Win32_System_Registry'*"]
#[cfg(all(feature = "Win32_Security", feature = "Win32_System_Registry"))]
#[inline]
pub unsafe fn GetSharedServiceRegistryStateKey<'a, Param0: ::windows::core::IntoParam<'a, super::super::Security::SC_HANDLE>>(servicehandle: Param0, statetype: SERVICE_SHARED_REGISTRY_STATE_TYPE, accessmask: u32, servicestatekey: *mut super::Registry::HKEY) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn GetSharedServiceRegistryStateKey(servicehandle: super::super::Security::SC_HANDLE, statetype: SERVICE_SHARED_REGISTRY_STATE_TYPE, accessmask: u32, servicestatekey: *mut super::Registry::HKEY) -> u32;
        }
        ::core::mem::transmute(GetSharedServiceRegistryStateKey(servicehandle.into_param().abi(), ::core::mem::transmute(statetype), ::core::mem::transmute(accessmask), ::core::mem::transmute(servicestatekey)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_System_Services'*"]
pub type HANDLER_FUNCTION = ::core::option::Option<unsafe extern "system" fn(dwcontrol: u32)>;
#[doc = "*Required features: 'Win32_System_Services'*"]
pub type HANDLER_FUNCTION_EX = ::core::option::Option<unsafe extern "system" fn(dwcontrol: u32, dweventtype: u32, lpeventdata: *mut ::core::ffi::c_void, lpcontext: *mut ::core::ffi::c_void) -> u32>;
#[doc = "*Required features: 'Win32_System_Services'*"]
pub type LPHANDLER_FUNCTION = ::core::option::Option<unsafe extern "system" fn(dwcontrol: u32)>;
#[doc = "*Required features: 'Win32_System_Services'*"]
pub type LPHANDLER_FUNCTION_EX = ::core::option::Option<unsafe extern "system" fn(dwcontrol: u32, dweventtype: u32, lpeventdata: *mut ::core::ffi::c_void, lpcontext: *mut ::core::ffi::c_void) -> u32>;
#[doc = "*Required features: 'Win32_System_Services', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
pub type LPSERVICE_MAIN_FUNCTIONA = ::core::option::Option<unsafe extern "system" fn(dwnumservicesargs: u32, lpserviceargvectors: *mut super::super::Foundation::PSTR)>;
#[doc = "*Required features: 'Win32_System_Services', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
pub type LPSERVICE_MAIN_FUNCTIONW = ::core::option::Option<unsafe extern "system" fn(dwnumservicesargs: u32, lpserviceargvectors: *mut super::super::Foundation::PWSTR)>;
#[doc = "*Required features: 'Win32_System_Services', 'Win32_Security'*"]
#[cfg(feature = "Win32_Security")]
#[inline]
pub unsafe fn LockServiceDatabase<'a, Param0: ::windows::core::IntoParam<'a, super::super::Security::SC_HANDLE>>(hscmanager: Param0) -> *mut ::core::ffi::c_void {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn LockServiceDatabase(hscmanager: super::super::Security::SC_HANDLE) -> *mut ::core::ffi::c_void;
        }
        ::core::mem::transmute(LockServiceDatabase(hscmanager.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
pub const MACHINE_POLICY_PRESENT_GUID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x659fcae6_5bdb_4da9_b1ff_ca2a178d46e0);
pub const NAMED_PIPE_EVENT_GUID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x1f81d131_3fac_4537_9e0c_7e7b0c2f4b55);
pub const NETWORK_MANAGER_FIRST_IP_ADDRESS_ARRIVAL_GUID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x4f27f2de_14e2_430b_a549_7cd48cbc8245);
pub const NETWORK_MANAGER_LAST_IP_ADDRESS_REMOVAL_GUID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xcc4ba62a_162e_4648_847a_b6bdf993e335);
#[doc = "*Required features: 'Win32_System_Services', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn NotifyBootConfigStatus<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BOOL>>(bootacceptable: Param0) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn NotifyBootConfigStatus(bootacceptable: super::super::Foundation::BOOL) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(NotifyBootConfigStatus(bootacceptable.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_System_Services', 'Win32_Foundation', 'Win32_Security'*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
#[inline]
pub unsafe fn NotifyServiceStatusChangeA<'a, Param0: ::windows::core::IntoParam<'a, super::super::Security::SC_HANDLE>>(hservice: Param0, dwnotifymask: SERVICE_NOTIFY, pnotifybuffer: *const SERVICE_NOTIFY_2A) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn NotifyServiceStatusChangeA(hservice: super::super::Security::SC_HANDLE, dwnotifymask: SERVICE_NOTIFY, pnotifybuffer: *const SERVICE_NOTIFY_2A) -> u32;
        }
        ::core::mem::transmute(NotifyServiceStatusChangeA(hservice.into_param().abi(), ::core::mem::transmute(dwnotifymask), ::core::mem::transmute(pnotifybuffer)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_System_Services', 'Win32_Foundation', 'Win32_Security'*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
#[inline]
pub unsafe fn NotifyServiceStatusChangeW<'a, Param0: ::windows::core::IntoParam<'a, super::super::Security::SC_HANDLE>>(hservice: Param0, dwnotifymask: SERVICE_NOTIFY, pnotifybuffer: *const SERVICE_NOTIFY_2W) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn NotifyServiceStatusChangeW(hservice: super::super::Security::SC_HANDLE, dwnotifymask: SERVICE_NOTIFY, pnotifybuffer: *const SERVICE_NOTIFY_2W) -> u32;
        }
        ::core::mem::transmute(NotifyServiceStatusChangeW(hservice.into_param().abi(), ::core::mem::transmute(dwnotifymask), ::core::mem::transmute(pnotifybuffer)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_System_Services', 'Win32_Foundation', 'Win32_Security'*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
#[inline]
pub unsafe fn OpenSCManagerA<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PSTR>, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::PSTR>>(lpmachinename: Param0, lpdatabasename: Param1, dwdesiredaccess: u32) -> super::super::Security::SC_HANDLE {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn OpenSCManagerA(lpmachinename: super::super::Foundation::PSTR, lpdatabasename: super::super::Foundation::PSTR, dwdesiredaccess: u32) -> super::super::Security::SC_HANDLE;
        }
        ::core::mem::transmute(OpenSCManagerA(lpmachinename.into_param().abi(), lpdatabasename.into_param().abi(), ::core::mem::transmute(dwdesiredaccess)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_System_Services', 'Win32_Foundation', 'Win32_Security'*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
#[inline]
pub unsafe fn OpenSCManagerW<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(lpmachinename: Param0, lpdatabasename: Param1, dwdesiredaccess: u32) -> super::super::Security::SC_HANDLE {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn OpenSCManagerW(lpmachinename: super::super::Foundation::PWSTR, lpdatabasename: super::super::Foundation::PWSTR, dwdesiredaccess: u32) -> super::super::Security::SC_HANDLE;
        }
        ::core::mem::transmute(OpenSCManagerW(lpmachinename.into_param().abi(), lpdatabasename.into_param().abi(), ::core::mem::transmute(dwdesiredaccess)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_System_Services', 'Win32_Foundation', 'Win32_Security'*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
#[inline]
pub unsafe fn OpenServiceA<'a, Param0: ::windows::core::IntoParam<'a, super::super::Security::SC_HANDLE>, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::PSTR>>(hscmanager: Param0, lpservicename: Param1, dwdesiredaccess: u32) -> super::super::Security::SC_HANDLE {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn OpenServiceA(hscmanager: super::super::Security::SC_HANDLE, lpservicename: super::super::Foundation::PSTR, dwdesiredaccess: u32) -> super::super::Security::SC_HANDLE;
        }
        ::core::mem::transmute(OpenServiceA(hscmanager.into_param().abi(), lpservicename.into_param().abi(), ::core::mem::transmute(dwdesiredaccess)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_System_Services', 'Win32_Foundation', 'Win32_Security'*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
#[inline]
pub unsafe fn OpenServiceW<'a, Param0: ::windows::core::IntoParam<'a, super::super::Security::SC_HANDLE>, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(hscmanager: Param0, lpservicename: Param1, dwdesiredaccess: u32) -> super::super::Security::SC_HANDLE {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn OpenServiceW(hscmanager: super::super::Security::SC_HANDLE, lpservicename: super::super::Foundation::PWSTR, dwdesiredaccess: u32) -> super::super::Security::SC_HANDLE;
        }
        ::core::mem::transmute(OpenServiceW(hscmanager.into_param().abi(), lpservicename.into_param().abi(), ::core::mem::transmute(dwdesiredaccess)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_System_Services'*"]
pub type PFN_SC_NOTIFY_CALLBACK = ::core::option::Option<unsafe extern "system" fn(pparameter: *const ::core::ffi::c_void)>;
#[doc = "*Required features: 'Win32_System_Services'*"]
pub type PSC_NOTIFICATION_CALLBACK = ::core::option::Option<unsafe extern "system" fn(dwnotify: u32, pcallbackcontext: *const ::core::ffi::c_void)>;
#[repr(C)]
#[doc = "*Required features: 'Win32_System_Services', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
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
impl ::core::marker::Copy for QUERY_SERVICE_CONFIGA {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for QUERY_SERVICE_CONFIGA {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for QUERY_SERVICE_CONFIGA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("QUERY_SERVICE_CONFIGA").field("dwServiceType", &self.dwServiceType).field("dwStartType", &self.dwStartType).field("dwErrorControl", &self.dwErrorControl).field("lpBinaryPathName", &self.lpBinaryPathName).field("lpLoadOrderGroup", &self.lpLoadOrderGroup).field("dwTagId", &self.dwTagId).field("lpDependencies", &self.lpDependencies).field("lpServiceStartName", &self.lpServiceStartName).field("lpDisplayName", &self.lpDisplayName).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for QUERY_SERVICE_CONFIGA {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for QUERY_SERVICE_CONFIGA {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<QUERY_SERVICE_CONFIGA>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for QUERY_SERVICE_CONFIGA {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for QUERY_SERVICE_CONFIGA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: 'Win32_System_Services', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
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
impl ::core::marker::Copy for QUERY_SERVICE_CONFIGW {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for QUERY_SERVICE_CONFIGW {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for QUERY_SERVICE_CONFIGW {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("QUERY_SERVICE_CONFIGW").field("dwServiceType", &self.dwServiceType).field("dwStartType", &self.dwStartType).field("dwErrorControl", &self.dwErrorControl).field("lpBinaryPathName", &self.lpBinaryPathName).field("lpLoadOrderGroup", &self.lpLoadOrderGroup).field("dwTagId", &self.dwTagId).field("lpDependencies", &self.lpDependencies).field("lpServiceStartName", &self.lpServiceStartName).field("lpDisplayName", &self.lpDisplayName).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for QUERY_SERVICE_CONFIGW {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for QUERY_SERVICE_CONFIGW {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<QUERY_SERVICE_CONFIGW>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for QUERY_SERVICE_CONFIGW {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for QUERY_SERVICE_CONFIGW {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: 'Win32_System_Services', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
pub struct QUERY_SERVICE_LOCK_STATUSA {
    pub fIsLocked: u32,
    pub lpLockOwner: super::super::Foundation::PSTR,
    pub dwLockDuration: u32,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for QUERY_SERVICE_LOCK_STATUSA {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for QUERY_SERVICE_LOCK_STATUSA {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for QUERY_SERVICE_LOCK_STATUSA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("QUERY_SERVICE_LOCK_STATUSA").field("fIsLocked", &self.fIsLocked).field("lpLockOwner", &self.lpLockOwner).field("dwLockDuration", &self.dwLockDuration).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for QUERY_SERVICE_LOCK_STATUSA {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for QUERY_SERVICE_LOCK_STATUSA {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<QUERY_SERVICE_LOCK_STATUSA>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for QUERY_SERVICE_LOCK_STATUSA {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for QUERY_SERVICE_LOCK_STATUSA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: 'Win32_System_Services', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
pub struct QUERY_SERVICE_LOCK_STATUSW {
    pub fIsLocked: u32,
    pub lpLockOwner: super::super::Foundation::PWSTR,
    pub dwLockDuration: u32,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for QUERY_SERVICE_LOCK_STATUSW {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for QUERY_SERVICE_LOCK_STATUSW {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for QUERY_SERVICE_LOCK_STATUSW {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("QUERY_SERVICE_LOCK_STATUSW").field("fIsLocked", &self.fIsLocked).field("lpLockOwner", &self.lpLockOwner).field("dwLockDuration", &self.dwLockDuration).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for QUERY_SERVICE_LOCK_STATUSW {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for QUERY_SERVICE_LOCK_STATUSW {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<QUERY_SERVICE_LOCK_STATUSW>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for QUERY_SERVICE_LOCK_STATUSW {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for QUERY_SERVICE_LOCK_STATUSW {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: 'Win32_System_Services', 'Win32_Foundation', 'Win32_Security'*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
#[inline]
pub unsafe fn QueryServiceConfig2A<'a, Param0: ::windows::core::IntoParam<'a, super::super::Security::SC_HANDLE>>(hservice: Param0, dwinfolevel: SERVICE_CONFIG, lpbuffer: *mut u8, cbbufsize: u32, pcbbytesneeded: *mut u32) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn QueryServiceConfig2A(hservice: super::super::Security::SC_HANDLE, dwinfolevel: SERVICE_CONFIG, lpbuffer: *mut u8, cbbufsize: u32, pcbbytesneeded: *mut u32) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(QueryServiceConfig2A(hservice.into_param().abi(), ::core::mem::transmute(dwinfolevel), ::core::mem::transmute(lpbuffer), ::core::mem::transmute(cbbufsize), ::core::mem::transmute(pcbbytesneeded)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_System_Services', 'Win32_Foundation', 'Win32_Security'*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
#[inline]
pub unsafe fn QueryServiceConfig2W<'a, Param0: ::windows::core::IntoParam<'a, super::super::Security::SC_HANDLE>>(hservice: Param0, dwinfolevel: SERVICE_CONFIG, lpbuffer: *mut u8, cbbufsize: u32, pcbbytesneeded: *mut u32) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn QueryServiceConfig2W(hservice: super::super::Security::SC_HANDLE, dwinfolevel: SERVICE_CONFIG, lpbuffer: *mut u8, cbbufsize: u32, pcbbytesneeded: *mut u32) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(QueryServiceConfig2W(hservice.into_param().abi(), ::core::mem::transmute(dwinfolevel), ::core::mem::transmute(lpbuffer), ::core::mem::transmute(cbbufsize), ::core::mem::transmute(pcbbytesneeded)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_System_Services', 'Win32_Foundation', 'Win32_Security'*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
#[inline]
pub unsafe fn QueryServiceConfigA<'a, Param0: ::windows::core::IntoParam<'a, super::super::Security::SC_HANDLE>>(hservice: Param0, lpserviceconfig: *mut QUERY_SERVICE_CONFIGA, cbbufsize: u32, pcbbytesneeded: *mut u32) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn QueryServiceConfigA(hservice: super::super::Security::SC_HANDLE, lpserviceconfig: *mut QUERY_SERVICE_CONFIGA, cbbufsize: u32, pcbbytesneeded: *mut u32) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(QueryServiceConfigA(hservice.into_param().abi(), ::core::mem::transmute(lpserviceconfig), ::core::mem::transmute(cbbufsize), ::core::mem::transmute(pcbbytesneeded)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_System_Services', 'Win32_Foundation', 'Win32_Security'*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
#[inline]
pub unsafe fn QueryServiceConfigW<'a, Param0: ::windows::core::IntoParam<'a, super::super::Security::SC_HANDLE>>(hservice: Param0, lpserviceconfig: *mut QUERY_SERVICE_CONFIGW, cbbufsize: u32, pcbbytesneeded: *mut u32) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn QueryServiceConfigW(hservice: super::super::Security::SC_HANDLE, lpserviceconfig: *mut QUERY_SERVICE_CONFIGW, cbbufsize: u32, pcbbytesneeded: *mut u32) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(QueryServiceConfigW(hservice.into_param().abi(), ::core::mem::transmute(lpserviceconfig), ::core::mem::transmute(cbbufsize), ::core::mem::transmute(pcbbytesneeded)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_System_Services', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn QueryServiceDynamicInformation<'a, Param0: ::windows::core::IntoParam<'a, SERVICE_STATUS_HANDLE>>(hservicestatus: Param0, dwinfolevel: u32, ppdynamicinfo: *mut *mut ::core::ffi::c_void) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn QueryServiceDynamicInformation(hservicestatus: SERVICE_STATUS_HANDLE, dwinfolevel: u32, ppdynamicinfo: *mut *mut ::core::ffi::c_void) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(QueryServiceDynamicInformation(hservicestatus.into_param().abi(), ::core::mem::transmute(dwinfolevel), ::core::mem::transmute(ppdynamicinfo)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_System_Services', 'Win32_Foundation', 'Win32_Security'*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
#[inline]
pub unsafe fn QueryServiceLockStatusA<'a, Param0: ::windows::core::IntoParam<'a, super::super::Security::SC_HANDLE>>(hscmanager: Param0, lplockstatus: *mut QUERY_SERVICE_LOCK_STATUSA, cbbufsize: u32, pcbbytesneeded: *mut u32) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn QueryServiceLockStatusA(hscmanager: super::super::Security::SC_HANDLE, lplockstatus: *mut QUERY_SERVICE_LOCK_STATUSA, cbbufsize: u32, pcbbytesneeded: *mut u32) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(QueryServiceLockStatusA(hscmanager.into_param().abi(), ::core::mem::transmute(lplockstatus), ::core::mem::transmute(cbbufsize), ::core::mem::transmute(pcbbytesneeded)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_System_Services', 'Win32_Foundation', 'Win32_Security'*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
#[inline]
pub unsafe fn QueryServiceLockStatusW<'a, Param0: ::windows::core::IntoParam<'a, super::super::Security::SC_HANDLE>>(hscmanager: Param0, lplockstatus: *mut QUERY_SERVICE_LOCK_STATUSW, cbbufsize: u32, pcbbytesneeded: *mut u32) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn QueryServiceLockStatusW(hscmanager: super::super::Security::SC_HANDLE, lplockstatus: *mut QUERY_SERVICE_LOCK_STATUSW, cbbufsize: u32, pcbbytesneeded: *mut u32) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(QueryServiceLockStatusW(hscmanager.into_param().abi(), ::core::mem::transmute(lplockstatus), ::core::mem::transmute(cbbufsize), ::core::mem::transmute(pcbbytesneeded)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_System_Services', 'Win32_Foundation', 'Win32_Security'*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
#[inline]
pub unsafe fn QueryServiceObjectSecurity<'a, Param0: ::windows::core::IntoParam<'a, super::super::Security::SC_HANDLE>>(hservice: Param0, dwsecurityinformation: u32, lpsecuritydescriptor: *mut super::super::Security::SECURITY_DESCRIPTOR, cbbufsize: u32, pcbbytesneeded: *mut u32) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn QueryServiceObjectSecurity(hservice: super::super::Security::SC_HANDLE, dwsecurityinformation: u32, lpsecuritydescriptor: *mut super::super::Security::SECURITY_DESCRIPTOR, cbbufsize: u32, pcbbytesneeded: *mut u32) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(QueryServiceObjectSecurity(hservice.into_param().abi(), ::core::mem::transmute(dwsecurityinformation), ::core::mem::transmute(lpsecuritydescriptor), ::core::mem::transmute(cbbufsize), ::core::mem::transmute(pcbbytesneeded)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_System_Services', 'Win32_Foundation', 'Win32_Security'*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
#[inline]
pub unsafe fn QueryServiceStatus<'a, Param0: ::windows::core::IntoParam<'a, super::super::Security::SC_HANDLE>>(hservice: Param0, lpservicestatus: *mut SERVICE_STATUS) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn QueryServiceStatus(hservice: super::super::Security::SC_HANDLE, lpservicestatus: *mut SERVICE_STATUS) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(QueryServiceStatus(hservice.into_param().abi(), ::core::mem::transmute(lpservicestatus)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_System_Services', 'Win32_Foundation', 'Win32_Security'*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
#[inline]
pub unsafe fn QueryServiceStatusEx<'a, Param0: ::windows::core::IntoParam<'a, super::super::Security::SC_HANDLE>>(hservice: Param0, infolevel: SC_STATUS_TYPE, lpbuffer: *mut u8, cbbufsize: u32, pcbbytesneeded: *mut u32) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn QueryServiceStatusEx(hservice: super::super::Security::SC_HANDLE, infolevel: SC_STATUS_TYPE, lpbuffer: *mut u8, cbbufsize: u32, pcbbytesneeded: *mut u32) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(QueryServiceStatusEx(hservice.into_param().abi(), ::core::mem::transmute(infolevel), ::core::mem::transmute(lpbuffer), ::core::mem::transmute(cbbufsize), ::core::mem::transmute(pcbbytesneeded)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
pub const RPC_INTERFACE_EVENT_GUID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xbc90d167_9470_4139_a9ba_be0bbbf5b74d);
#[doc = "*Required features: 'Win32_System_Services', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn RegisterServiceCtrlHandlerA<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PSTR>>(lpservicename: Param0, lphandlerproc: LPHANDLER_FUNCTION) -> SERVICE_STATUS_HANDLE {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn RegisterServiceCtrlHandlerA(lpservicename: super::super::Foundation::PSTR, lphandlerproc: ::windows::core::RawPtr) -> SERVICE_STATUS_HANDLE;
        }
        ::core::mem::transmute(RegisterServiceCtrlHandlerA(lpservicename.into_param().abi(), ::core::mem::transmute(lphandlerproc)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_System_Services', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn RegisterServiceCtrlHandlerExA<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PSTR>>(lpservicename: Param0, lphandlerproc: LPHANDLER_FUNCTION_EX, lpcontext: *const ::core::ffi::c_void) -> SERVICE_STATUS_HANDLE {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn RegisterServiceCtrlHandlerExA(lpservicename: super::super::Foundation::PSTR, lphandlerproc: ::windows::core::RawPtr, lpcontext: *const ::core::ffi::c_void) -> SERVICE_STATUS_HANDLE;
        }
        ::core::mem::transmute(RegisterServiceCtrlHandlerExA(lpservicename.into_param().abi(), ::core::mem::transmute(lphandlerproc), ::core::mem::transmute(lpcontext)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_System_Services', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn RegisterServiceCtrlHandlerExW<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(lpservicename: Param0, lphandlerproc: LPHANDLER_FUNCTION_EX, lpcontext: *const ::core::ffi::c_void) -> SERVICE_STATUS_HANDLE {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn RegisterServiceCtrlHandlerExW(lpservicename: super::super::Foundation::PWSTR, lphandlerproc: ::windows::core::RawPtr, lpcontext: *const ::core::ffi::c_void) -> SERVICE_STATUS_HANDLE;
        }
        ::core::mem::transmute(RegisterServiceCtrlHandlerExW(lpservicename.into_param().abi(), ::core::mem::transmute(lphandlerproc), ::core::mem::transmute(lpcontext)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_System_Services', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn RegisterServiceCtrlHandlerW<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(lpservicename: Param0, lphandlerproc: LPHANDLER_FUNCTION) -> SERVICE_STATUS_HANDLE {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn RegisterServiceCtrlHandlerW(lpservicename: super::super::Foundation::PWSTR, lphandlerproc: ::windows::core::RawPtr) -> SERVICE_STATUS_HANDLE;
        }
        ::core::mem::transmute(RegisterServiceCtrlHandlerW(lpservicename.into_param().abi(), ::core::mem::transmute(lphandlerproc)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[repr(C)]
#[doc = "*Required features: 'Win32_System_Services'*"]
pub struct SC_ACTION {
    pub Type: SC_ACTION_TYPE,
    pub Delay: u32,
}
impl ::core::marker::Copy for SC_ACTION {}
impl ::core::clone::Clone for SC_ACTION {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for SC_ACTION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SC_ACTION").field("Type", &self.Type).field("Delay", &self.Delay).finish()
    }
}
unsafe impl ::windows::core::Abi for SC_ACTION {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for SC_ACTION {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<SC_ACTION>()) == 0 }
    }
}
impl ::core::cmp::Eq for SC_ACTION {}
impl ::core::default::Default for SC_ACTION {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: 'Win32_System_Services'*"]
pub type SC_ACTION_TYPE = i32;
#[doc = "*Required features: 'Win32_System_Services'*"]
pub const SC_ACTION_NONE: SC_ACTION_TYPE = 0i32;
#[doc = "*Required features: 'Win32_System_Services'*"]
pub const SC_ACTION_RESTART: SC_ACTION_TYPE = 1i32;
#[doc = "*Required features: 'Win32_System_Services'*"]
pub const SC_ACTION_REBOOT: SC_ACTION_TYPE = 2i32;
#[doc = "*Required features: 'Win32_System_Services'*"]
pub const SC_ACTION_RUN_COMMAND: SC_ACTION_TYPE = 3i32;
#[doc = "*Required features: 'Win32_System_Services'*"]
pub const SC_ACTION_OWN_RESTART: SC_ACTION_TYPE = 4i32;
#[doc = "*Required features: 'Win32_System_Services'*"]
pub type SC_ENUM_TYPE = i32;
#[doc = "*Required features: 'Win32_System_Services'*"]
pub const SC_ENUM_PROCESS_INFO: SC_ENUM_TYPE = 0i32;
#[doc = "*Required features: 'Win32_System_Services'*"]
pub type SC_EVENT_TYPE = i32;
#[doc = "*Required features: 'Win32_System_Services'*"]
pub const SC_EVENT_DATABASE_CHANGE: SC_EVENT_TYPE = 0i32;
#[doc = "*Required features: 'Win32_System_Services'*"]
pub const SC_EVENT_PROPERTY_CHANGE: SC_EVENT_TYPE = 1i32;
#[doc = "*Required features: 'Win32_System_Services'*"]
pub const SC_EVENT_STATUS_CHANGE: SC_EVENT_TYPE = 2i32;
#[doc = "*Required features: 'Win32_System_Services'*"]
pub const SC_MANAGER_ALL_ACCESS: u32 = 983103u32;
#[doc = "*Required features: 'Win32_System_Services'*"]
pub const SC_MANAGER_CONNECT: u32 = 1u32;
#[doc = "*Required features: 'Win32_System_Services'*"]
pub const SC_MANAGER_CREATE_SERVICE: u32 = 2u32;
#[doc = "*Required features: 'Win32_System_Services'*"]
pub const SC_MANAGER_ENUMERATE_SERVICE: u32 = 4u32;
#[doc = "*Required features: 'Win32_System_Services'*"]
pub const SC_MANAGER_LOCK: u32 = 8u32;
#[doc = "*Required features: 'Win32_System_Services'*"]
pub const SC_MANAGER_MODIFY_BOOT_CONFIG: u32 = 32u32;
#[doc = "*Required features: 'Win32_System_Services'*"]
pub const SC_MANAGER_QUERY_LOCK_STATUS: u32 = 16u32;
#[doc = "*Required features: 'Win32_System_Services'*"]
pub type SC_STATUS_TYPE = i32;
#[doc = "*Required features: 'Win32_System_Services'*"]
pub const SC_STATUS_PROCESS_INFO: SC_STATUS_TYPE = 0i32;
#[doc = "*Required features: 'Win32_System_Services'*"]
pub const SERVICE_ACCEPT_HARDWAREPROFILECHANGE: u32 = 32u32;
#[doc = "*Required features: 'Win32_System_Services'*"]
pub const SERVICE_ACCEPT_LOWRESOURCES: u32 = 8192u32;
#[doc = "*Required features: 'Win32_System_Services'*"]
pub const SERVICE_ACCEPT_NETBINDCHANGE: u32 = 16u32;
#[doc = "*Required features: 'Win32_System_Services'*"]
pub const SERVICE_ACCEPT_PARAMCHANGE: u32 = 8u32;
#[doc = "*Required features: 'Win32_System_Services'*"]
pub const SERVICE_ACCEPT_PAUSE_CONTINUE: u32 = 2u32;
#[doc = "*Required features: 'Win32_System_Services'*"]
pub const SERVICE_ACCEPT_POWEREVENT: u32 = 64u32;
#[doc = "*Required features: 'Win32_System_Services'*"]
pub const SERVICE_ACCEPT_PRESHUTDOWN: u32 = 256u32;
#[doc = "*Required features: 'Win32_System_Services'*"]
pub const SERVICE_ACCEPT_SESSIONCHANGE: u32 = 128u32;
#[doc = "*Required features: 'Win32_System_Services'*"]
pub const SERVICE_ACCEPT_SHUTDOWN: u32 = 4u32;
#[doc = "*Required features: 'Win32_System_Services'*"]
pub const SERVICE_ACCEPT_STOP: u32 = 1u32;
#[doc = "*Required features: 'Win32_System_Services'*"]
pub const SERVICE_ACCEPT_SYSTEMLOWRESOURCES: u32 = 16384u32;
#[doc = "*Required features: 'Win32_System_Services'*"]
pub const SERVICE_ACCEPT_TIMECHANGE: u32 = 512u32;
#[doc = "*Required features: 'Win32_System_Services'*"]
pub const SERVICE_ACCEPT_TRIGGEREVENT: u32 = 1024u32;
#[doc = "*Required features: 'Win32_System_Services'*"]
pub const SERVICE_ACCEPT_USER_LOGOFF: u32 = 2048u32;
#[doc = "*Required features: 'Win32_System_Services'*"]
pub const SERVICE_ALL_ACCESS: u32 = 983551u32;
#[doc = "*Required features: 'Win32_System_Services'*"]
pub const SERVICE_CHANGE_CONFIG: u32 = 2u32;
#[doc = "*Required features: 'Win32_System_Services'*"]
pub type SERVICE_CONFIG = u32;
#[doc = "*Required features: 'Win32_System_Services'*"]
pub const SERVICE_CONFIG_DELAYED_AUTO_START_INFO: SERVICE_CONFIG = 3u32;
#[doc = "*Required features: 'Win32_System_Services'*"]
pub const SERVICE_CONFIG_DESCRIPTION: SERVICE_CONFIG = 1u32;
#[doc = "*Required features: 'Win32_System_Services'*"]
pub const SERVICE_CONFIG_FAILURE_ACTIONS: SERVICE_CONFIG = 2u32;
#[doc = "*Required features: 'Win32_System_Services'*"]
pub const SERVICE_CONFIG_FAILURE_ACTIONS_FLAG: SERVICE_CONFIG = 4u32;
#[doc = "*Required features: 'Win32_System_Services'*"]
pub const SERVICE_CONFIG_PREFERRED_NODE: SERVICE_CONFIG = 9u32;
#[doc = "*Required features: 'Win32_System_Services'*"]
pub const SERVICE_CONFIG_PRESHUTDOWN_INFO: SERVICE_CONFIG = 7u32;
#[doc = "*Required features: 'Win32_System_Services'*"]
pub const SERVICE_CONFIG_REQUIRED_PRIVILEGES_INFO: SERVICE_CONFIG = 6u32;
#[doc = "*Required features: 'Win32_System_Services'*"]
pub const SERVICE_CONFIG_SERVICE_SID_INFO: SERVICE_CONFIG = 5u32;
#[doc = "*Required features: 'Win32_System_Services'*"]
pub const SERVICE_CONFIG_TRIGGER_INFO: SERVICE_CONFIG = 8u32;
#[doc = "*Required features: 'Win32_System_Services'*"]
pub const SERVICE_CONFIG_LAUNCH_PROTECTED: SERVICE_CONFIG = 12u32;
#[doc = "*Required features: 'Win32_System_Services'*"]
pub const SERVICE_CONTROL_CONTINUE: u32 = 3u32;
#[doc = "*Required features: 'Win32_System_Services'*"]
pub const SERVICE_CONTROL_DEVICEEVENT: u32 = 11u32;
#[doc = "*Required features: 'Win32_System_Services'*"]
pub const SERVICE_CONTROL_HARDWAREPROFILECHANGE: u32 = 12u32;
#[doc = "*Required features: 'Win32_System_Services'*"]
pub const SERVICE_CONTROL_INTERROGATE: u32 = 4u32;
#[doc = "*Required features: 'Win32_System_Services'*"]
pub const SERVICE_CONTROL_LOWRESOURCES: u32 = 96u32;
#[doc = "*Required features: 'Win32_System_Services'*"]
pub const SERVICE_CONTROL_NETBINDADD: u32 = 7u32;
#[doc = "*Required features: 'Win32_System_Services'*"]
pub const SERVICE_CONTROL_NETBINDDISABLE: u32 = 10u32;
#[doc = "*Required features: 'Win32_System_Services'*"]
pub const SERVICE_CONTROL_NETBINDENABLE: u32 = 9u32;
#[doc = "*Required features: 'Win32_System_Services'*"]
pub const SERVICE_CONTROL_NETBINDREMOVE: u32 = 8u32;
#[doc = "*Required features: 'Win32_System_Services'*"]
pub const SERVICE_CONTROL_PARAMCHANGE: u32 = 6u32;
#[doc = "*Required features: 'Win32_System_Services'*"]
pub const SERVICE_CONTROL_PAUSE: u32 = 2u32;
#[doc = "*Required features: 'Win32_System_Services'*"]
pub const SERVICE_CONTROL_POWEREVENT: u32 = 13u32;
#[doc = "*Required features: 'Win32_System_Services'*"]
pub const SERVICE_CONTROL_PRESHUTDOWN: u32 = 15u32;
#[doc = "*Required features: 'Win32_System_Services'*"]
pub const SERVICE_CONTROL_SESSIONCHANGE: u32 = 14u32;
#[doc = "*Required features: 'Win32_System_Services'*"]
pub const SERVICE_CONTROL_SHUTDOWN: u32 = 5u32;
#[doc = "*Required features: 'Win32_System_Services'*"]
pub const SERVICE_CONTROL_STATUS_REASON_INFO: u32 = 1u32;
#[repr(C)]
#[doc = "*Required features: 'Win32_System_Services', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
pub struct SERVICE_CONTROL_STATUS_REASON_PARAMSA {
    pub dwReason: u32,
    pub pszComment: super::super::Foundation::PSTR,
    pub ServiceStatus: SERVICE_STATUS_PROCESS,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for SERVICE_CONTROL_STATUS_REASON_PARAMSA {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for SERVICE_CONTROL_STATUS_REASON_PARAMSA {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for SERVICE_CONTROL_STATUS_REASON_PARAMSA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SERVICE_CONTROL_STATUS_REASON_PARAMSA").field("dwReason", &self.dwReason).field("pszComment", &self.pszComment).field("ServiceStatus", &self.ServiceStatus).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for SERVICE_CONTROL_STATUS_REASON_PARAMSA {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for SERVICE_CONTROL_STATUS_REASON_PARAMSA {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<SERVICE_CONTROL_STATUS_REASON_PARAMSA>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for SERVICE_CONTROL_STATUS_REASON_PARAMSA {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for SERVICE_CONTROL_STATUS_REASON_PARAMSA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: 'Win32_System_Services', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
pub struct SERVICE_CONTROL_STATUS_REASON_PARAMSW {
    pub dwReason: u32,
    pub pszComment: super::super::Foundation::PWSTR,
    pub ServiceStatus: SERVICE_STATUS_PROCESS,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for SERVICE_CONTROL_STATUS_REASON_PARAMSW {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for SERVICE_CONTROL_STATUS_REASON_PARAMSW {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for SERVICE_CONTROL_STATUS_REASON_PARAMSW {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SERVICE_CONTROL_STATUS_REASON_PARAMSW").field("dwReason", &self.dwReason).field("pszComment", &self.pszComment).field("ServiceStatus", &self.ServiceStatus).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for SERVICE_CONTROL_STATUS_REASON_PARAMSW {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for SERVICE_CONTROL_STATUS_REASON_PARAMSW {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<SERVICE_CONTROL_STATUS_REASON_PARAMSW>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for SERVICE_CONTROL_STATUS_REASON_PARAMSW {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for SERVICE_CONTROL_STATUS_REASON_PARAMSW {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: 'Win32_System_Services'*"]
pub const SERVICE_CONTROL_STOP: u32 = 1u32;
#[doc = "*Required features: 'Win32_System_Services'*"]
pub const SERVICE_CONTROL_SYSTEMLOWRESOURCES: u32 = 97u32;
#[doc = "*Required features: 'Win32_System_Services'*"]
pub const SERVICE_CONTROL_TIMECHANGE: u32 = 16u32;
#[doc = "*Required features: 'Win32_System_Services'*"]
pub const SERVICE_CONTROL_TRIGGEREVENT: u32 = 32u32;
#[repr(C)]
#[doc = "*Required features: 'Win32_System_Services'*"]
pub struct SERVICE_CUSTOM_SYSTEM_STATE_CHANGE_DATA_ITEM {
    pub u: SERVICE_CUSTOM_SYSTEM_STATE_CHANGE_DATA_ITEM_0,
}
impl ::core::marker::Copy for SERVICE_CUSTOM_SYSTEM_STATE_CHANGE_DATA_ITEM {}
impl ::core::clone::Clone for SERVICE_CUSTOM_SYSTEM_STATE_CHANGE_DATA_ITEM {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for SERVICE_CUSTOM_SYSTEM_STATE_CHANGE_DATA_ITEM {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for SERVICE_CUSTOM_SYSTEM_STATE_CHANGE_DATA_ITEM {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<SERVICE_CUSTOM_SYSTEM_STATE_CHANGE_DATA_ITEM>()) == 0 }
    }
}
impl ::core::cmp::Eq for SERVICE_CUSTOM_SYSTEM_STATE_CHANGE_DATA_ITEM {}
impl ::core::default::Default for SERVICE_CUSTOM_SYSTEM_STATE_CHANGE_DATA_ITEM {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: 'Win32_System_Services'*"]
pub union SERVICE_CUSTOM_SYSTEM_STATE_CHANGE_DATA_ITEM_0 {
    pub CustomStateId: SERVICE_TRIGGER_CUSTOM_STATE_ID,
    pub s: SERVICE_CUSTOM_SYSTEM_STATE_CHANGE_DATA_ITEM_0_0,
}
impl ::core::marker::Copy for SERVICE_CUSTOM_SYSTEM_STATE_CHANGE_DATA_ITEM_0 {}
impl ::core::clone::Clone for SERVICE_CUSTOM_SYSTEM_STATE_CHANGE_DATA_ITEM_0 {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for SERVICE_CUSTOM_SYSTEM_STATE_CHANGE_DATA_ITEM_0 {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for SERVICE_CUSTOM_SYSTEM_STATE_CHANGE_DATA_ITEM_0 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<SERVICE_CUSTOM_SYSTEM_STATE_CHANGE_DATA_ITEM_0>()) == 0 }
    }
}
impl ::core::cmp::Eq for SERVICE_CUSTOM_SYSTEM_STATE_CHANGE_DATA_ITEM_0 {}
impl ::core::default::Default for SERVICE_CUSTOM_SYSTEM_STATE_CHANGE_DATA_ITEM_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: 'Win32_System_Services'*"]
pub struct SERVICE_CUSTOM_SYSTEM_STATE_CHANGE_DATA_ITEM_0_0 {
    pub DataOffset: u32,
    pub Data: [u8; 1],
}
impl ::core::marker::Copy for SERVICE_CUSTOM_SYSTEM_STATE_CHANGE_DATA_ITEM_0_0 {}
impl ::core::clone::Clone for SERVICE_CUSTOM_SYSTEM_STATE_CHANGE_DATA_ITEM_0_0 {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for SERVICE_CUSTOM_SYSTEM_STATE_CHANGE_DATA_ITEM_0_0 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SERVICE_CUSTOM_SYSTEM_STATE_CHANGE_DATA_ITEM_0_0").field("DataOffset", &self.DataOffset).field("Data", &self.Data).finish()
    }
}
unsafe impl ::windows::core::Abi for SERVICE_CUSTOM_SYSTEM_STATE_CHANGE_DATA_ITEM_0_0 {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for SERVICE_CUSTOM_SYSTEM_STATE_CHANGE_DATA_ITEM_0_0 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<SERVICE_CUSTOM_SYSTEM_STATE_CHANGE_DATA_ITEM_0_0>()) == 0 }
    }
}
impl ::core::cmp::Eq for SERVICE_CUSTOM_SYSTEM_STATE_CHANGE_DATA_ITEM_0_0 {}
impl ::core::default::Default for SERVICE_CUSTOM_SYSTEM_STATE_CHANGE_DATA_ITEM_0_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: 'Win32_System_Services', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
pub struct SERVICE_DELAYED_AUTO_START_INFO {
    pub fDelayedAutostart: super::super::Foundation::BOOL,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for SERVICE_DELAYED_AUTO_START_INFO {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for SERVICE_DELAYED_AUTO_START_INFO {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for SERVICE_DELAYED_AUTO_START_INFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SERVICE_DELAYED_AUTO_START_INFO").field("fDelayedAutostart", &self.fDelayedAutostart).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for SERVICE_DELAYED_AUTO_START_INFO {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for SERVICE_DELAYED_AUTO_START_INFO {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<SERVICE_DELAYED_AUTO_START_INFO>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for SERVICE_DELAYED_AUTO_START_INFO {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for SERVICE_DELAYED_AUTO_START_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: 'Win32_System_Services', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
pub struct SERVICE_DESCRIPTIONA {
    pub lpDescription: super::super::Foundation::PSTR,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for SERVICE_DESCRIPTIONA {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for SERVICE_DESCRIPTIONA {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for SERVICE_DESCRIPTIONA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SERVICE_DESCRIPTIONA").field("lpDescription", &self.lpDescription).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for SERVICE_DESCRIPTIONA {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for SERVICE_DESCRIPTIONA {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<SERVICE_DESCRIPTIONA>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for SERVICE_DESCRIPTIONA {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for SERVICE_DESCRIPTIONA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: 'Win32_System_Services', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
pub struct SERVICE_DESCRIPTIONW {
    pub lpDescription: super::super::Foundation::PWSTR,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for SERVICE_DESCRIPTIONW {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for SERVICE_DESCRIPTIONW {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for SERVICE_DESCRIPTIONW {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SERVICE_DESCRIPTIONW").field("lpDescription", &self.lpDescription).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for SERVICE_DESCRIPTIONW {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for SERVICE_DESCRIPTIONW {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<SERVICE_DESCRIPTIONW>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for SERVICE_DESCRIPTIONW {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for SERVICE_DESCRIPTIONW {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: 'Win32_System_Services'*"]
pub type SERVICE_DIRECTORY_TYPE = i32;
#[doc = "*Required features: 'Win32_System_Services'*"]
pub const ServiceDirectoryPersistentState: SERVICE_DIRECTORY_TYPE = 0i32;
#[doc = "*Required features: 'Win32_System_Services'*"]
pub const ServiceDirectoryTypeMax: SERVICE_DIRECTORY_TYPE = 1i32;
#[doc = "*Required features: 'Win32_System_Services'*"]
pub const SERVICE_DYNAMIC_INFORMATION_LEVEL_START_REASON: u32 = 1u32;
#[doc = "*Required features: 'Win32_System_Services'*"]
pub const SERVICE_ENUMERATE_DEPENDENTS: u32 = 8u32;
#[doc = "*Required features: 'Win32_System_Services'*"]
pub type SERVICE_ERROR = u32;
#[doc = "*Required features: 'Win32_System_Services'*"]
pub const SERVICE_ERROR_CRITICAL: SERVICE_ERROR = 3u32;
#[doc = "*Required features: 'Win32_System_Services'*"]
pub const SERVICE_ERROR_IGNORE: SERVICE_ERROR = 0u32;
#[doc = "*Required features: 'Win32_System_Services'*"]
pub const SERVICE_ERROR_NORMAL: SERVICE_ERROR = 1u32;
#[doc = "*Required features: 'Win32_System_Services'*"]
pub const SERVICE_ERROR_SEVERE: SERVICE_ERROR = 2u32;
#[repr(C)]
#[doc = "*Required features: 'Win32_System_Services', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
pub struct SERVICE_FAILURE_ACTIONSA {
    pub dwResetPeriod: u32,
    pub lpRebootMsg: super::super::Foundation::PSTR,
    pub lpCommand: super::super::Foundation::PSTR,
    pub cActions: u32,
    pub lpsaActions: *mut SC_ACTION,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for SERVICE_FAILURE_ACTIONSA {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for SERVICE_FAILURE_ACTIONSA {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for SERVICE_FAILURE_ACTIONSA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SERVICE_FAILURE_ACTIONSA").field("dwResetPeriod", &self.dwResetPeriod).field("lpRebootMsg", &self.lpRebootMsg).field("lpCommand", &self.lpCommand).field("cActions", &self.cActions).field("lpsaActions", &self.lpsaActions).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for SERVICE_FAILURE_ACTIONSA {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for SERVICE_FAILURE_ACTIONSA {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<SERVICE_FAILURE_ACTIONSA>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for SERVICE_FAILURE_ACTIONSA {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for SERVICE_FAILURE_ACTIONSA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: 'Win32_System_Services', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
pub struct SERVICE_FAILURE_ACTIONSW {
    pub dwResetPeriod: u32,
    pub lpRebootMsg: super::super::Foundation::PWSTR,
    pub lpCommand: super::super::Foundation::PWSTR,
    pub cActions: u32,
    pub lpsaActions: *mut SC_ACTION,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for SERVICE_FAILURE_ACTIONSW {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for SERVICE_FAILURE_ACTIONSW {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for SERVICE_FAILURE_ACTIONSW {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SERVICE_FAILURE_ACTIONSW").field("dwResetPeriod", &self.dwResetPeriod).field("lpRebootMsg", &self.lpRebootMsg).field("lpCommand", &self.lpCommand).field("cActions", &self.cActions).field("lpsaActions", &self.lpsaActions).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for SERVICE_FAILURE_ACTIONSW {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for SERVICE_FAILURE_ACTIONSW {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<SERVICE_FAILURE_ACTIONSW>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for SERVICE_FAILURE_ACTIONSW {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for SERVICE_FAILURE_ACTIONSW {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: 'Win32_System_Services', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
pub struct SERVICE_FAILURE_ACTIONS_FLAG {
    pub fFailureActionsOnNonCrashFailures: super::super::Foundation::BOOL,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for SERVICE_FAILURE_ACTIONS_FLAG {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for SERVICE_FAILURE_ACTIONS_FLAG {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for SERVICE_FAILURE_ACTIONS_FLAG {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SERVICE_FAILURE_ACTIONS_FLAG").field("fFailureActionsOnNonCrashFailures", &self.fFailureActionsOnNonCrashFailures).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for SERVICE_FAILURE_ACTIONS_FLAG {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for SERVICE_FAILURE_ACTIONS_FLAG {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<SERVICE_FAILURE_ACTIONS_FLAG>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for SERVICE_FAILURE_ACTIONS_FLAG {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for SERVICE_FAILURE_ACTIONS_FLAG {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: 'Win32_System_Services'*"]
pub const SERVICE_INTERROGATE: u32 = 128u32;
#[doc = "*Required features: 'Win32_System_Services'*"]
pub const SERVICE_LAUNCH_PROTECTED_ANTIMALWARE_LIGHT: u32 = 3u32;
#[repr(C)]
#[doc = "*Required features: 'Win32_System_Services'*"]
pub struct SERVICE_LAUNCH_PROTECTED_INFO {
    pub dwLaunchProtected: u32,
}
impl ::core::marker::Copy for SERVICE_LAUNCH_PROTECTED_INFO {}
impl ::core::clone::Clone for SERVICE_LAUNCH_PROTECTED_INFO {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for SERVICE_LAUNCH_PROTECTED_INFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SERVICE_LAUNCH_PROTECTED_INFO").field("dwLaunchProtected", &self.dwLaunchProtected).finish()
    }
}
unsafe impl ::windows::core::Abi for SERVICE_LAUNCH_PROTECTED_INFO {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for SERVICE_LAUNCH_PROTECTED_INFO {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<SERVICE_LAUNCH_PROTECTED_INFO>()) == 0 }
    }
}
impl ::core::cmp::Eq for SERVICE_LAUNCH_PROTECTED_INFO {}
impl ::core::default::Default for SERVICE_LAUNCH_PROTECTED_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: 'Win32_System_Services'*"]
pub const SERVICE_LAUNCH_PROTECTED_NONE: u32 = 0u32;
#[doc = "*Required features: 'Win32_System_Services'*"]
pub const SERVICE_LAUNCH_PROTECTED_WINDOWS: u32 = 1u32;
#[doc = "*Required features: 'Win32_System_Services'*"]
pub const SERVICE_LAUNCH_PROTECTED_WINDOWS_LIGHT: u32 = 2u32;
#[doc = "*Required features: 'Win32_System_Services'*"]
pub type SERVICE_MAIN_FUNCTIONA = ::core::option::Option<unsafe extern "system" fn(dwnumservicesargs: u32, lpserviceargvectors: *mut *mut i8)>;
#[doc = "*Required features: 'Win32_System_Services', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
pub type SERVICE_MAIN_FUNCTIONW = ::core::option::Option<unsafe extern "system" fn(dwnumservicesargs: u32, lpserviceargvectors: *mut super::super::Foundation::PWSTR)>;
#[doc = "*Required features: 'Win32_System_Services'*"]
pub type SERVICE_NOTIFY = u32;
#[doc = "*Required features: 'Win32_System_Services'*"]
pub const SERVICE_NOTIFY_CREATED: SERVICE_NOTIFY = 128u32;
#[doc = "*Required features: 'Win32_System_Services'*"]
pub const SERVICE_NOTIFY_CONTINUE_PENDING: SERVICE_NOTIFY = 16u32;
#[doc = "*Required features: 'Win32_System_Services'*"]
pub const SERVICE_NOTIFY_DELETE_PENDING: SERVICE_NOTIFY = 512u32;
#[doc = "*Required features: 'Win32_System_Services'*"]
pub const SERVICE_NOTIFY_DELETED: SERVICE_NOTIFY = 256u32;
#[doc = "*Required features: 'Win32_System_Services'*"]
pub const SERVICE_NOTIFY_PAUSE_PENDING: SERVICE_NOTIFY = 32u32;
#[doc = "*Required features: 'Win32_System_Services'*"]
pub const SERVICE_NOTIFY_PAUSED: SERVICE_NOTIFY = 64u32;
#[doc = "*Required features: 'Win32_System_Services'*"]
pub const SERVICE_NOTIFY_RUNNING: SERVICE_NOTIFY = 8u32;
#[doc = "*Required features: 'Win32_System_Services'*"]
pub const SERVICE_NOTIFY_START_PENDING: SERVICE_NOTIFY = 2u32;
#[doc = "*Required features: 'Win32_System_Services'*"]
pub const SERVICE_NOTIFY_STOP_PENDING: SERVICE_NOTIFY = 4u32;
#[doc = "*Required features: 'Win32_System_Services'*"]
pub const SERVICE_NOTIFY_STOPPED: SERVICE_NOTIFY = 1u32;
#[repr(C)]
#[doc = "*Required features: 'Win32_System_Services'*"]
pub struct SERVICE_NOTIFY_1 {
    pub dwVersion: u32,
    pub pfnNotifyCallback: PFN_SC_NOTIFY_CALLBACK,
    pub pContext: *mut ::core::ffi::c_void,
    pub dwNotificationStatus: u32,
    pub ServiceStatus: SERVICE_STATUS_PROCESS,
}
impl ::core::marker::Copy for SERVICE_NOTIFY_1 {}
impl ::core::clone::Clone for SERVICE_NOTIFY_1 {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for SERVICE_NOTIFY_1 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SERVICE_NOTIFY_1").field("dwVersion", &self.dwVersion).field("pfnNotifyCallback", &self.pfnNotifyCallback.map(|f| f as usize)).field("pContext", &self.pContext).field("dwNotificationStatus", &self.dwNotificationStatus).field("ServiceStatus", &self.ServiceStatus).finish()
    }
}
unsafe impl ::windows::core::Abi for SERVICE_NOTIFY_1 {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for SERVICE_NOTIFY_1 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<SERVICE_NOTIFY_1>()) == 0 }
    }
}
impl ::core::cmp::Eq for SERVICE_NOTIFY_1 {}
impl ::core::default::Default for SERVICE_NOTIFY_1 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: 'Win32_System_Services', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
pub struct SERVICE_NOTIFY_2A {
    pub dwVersion: u32,
    pub pfnNotifyCallback: PFN_SC_NOTIFY_CALLBACK,
    pub pContext: *mut ::core::ffi::c_void,
    pub dwNotificationStatus: u32,
    pub ServiceStatus: SERVICE_STATUS_PROCESS,
    pub dwNotificationTriggered: u32,
    pub pszServiceNames: super::super::Foundation::PSTR,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for SERVICE_NOTIFY_2A {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for SERVICE_NOTIFY_2A {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for SERVICE_NOTIFY_2A {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SERVICE_NOTIFY_2A").field("dwVersion", &self.dwVersion).field("pfnNotifyCallback", &self.pfnNotifyCallback.map(|f| f as usize)).field("pContext", &self.pContext).field("dwNotificationStatus", &self.dwNotificationStatus).field("ServiceStatus", &self.ServiceStatus).field("dwNotificationTriggered", &self.dwNotificationTriggered).field("pszServiceNames", &self.pszServiceNames).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for SERVICE_NOTIFY_2A {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for SERVICE_NOTIFY_2A {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<SERVICE_NOTIFY_2A>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for SERVICE_NOTIFY_2A {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for SERVICE_NOTIFY_2A {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: 'Win32_System_Services', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
pub struct SERVICE_NOTIFY_2W {
    pub dwVersion: u32,
    pub pfnNotifyCallback: PFN_SC_NOTIFY_CALLBACK,
    pub pContext: *mut ::core::ffi::c_void,
    pub dwNotificationStatus: u32,
    pub ServiceStatus: SERVICE_STATUS_PROCESS,
    pub dwNotificationTriggered: u32,
    pub pszServiceNames: super::super::Foundation::PWSTR,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for SERVICE_NOTIFY_2W {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for SERVICE_NOTIFY_2W {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for SERVICE_NOTIFY_2W {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SERVICE_NOTIFY_2W").field("dwVersion", &self.dwVersion).field("pfnNotifyCallback", &self.pfnNotifyCallback.map(|f| f as usize)).field("pContext", &self.pContext).field("dwNotificationStatus", &self.dwNotificationStatus).field("ServiceStatus", &self.ServiceStatus).field("dwNotificationTriggered", &self.dwNotificationTriggered).field("pszServiceNames", &self.pszServiceNames).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for SERVICE_NOTIFY_2W {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for SERVICE_NOTIFY_2W {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<SERVICE_NOTIFY_2W>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for SERVICE_NOTIFY_2W {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for SERVICE_NOTIFY_2W {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: 'Win32_System_Services'*"]
pub const SERVICE_NOTIFY_STATUS_CHANGE: u32 = 2u32;
#[doc = "*Required features: 'Win32_System_Services'*"]
pub const SERVICE_NOTIFY_STATUS_CHANGE_1: u32 = 1u32;
#[doc = "*Required features: 'Win32_System_Services'*"]
pub const SERVICE_NOTIFY_STATUS_CHANGE_2: u32 = 2u32;
#[doc = "*Required features: 'Win32_System_Services'*"]
pub const SERVICE_NO_CHANGE: u32 = 4294967295u32;
#[doc = "*Required features: 'Win32_System_Services'*"]
pub const SERVICE_PAUSE_CONTINUE: u32 = 64u32;
#[repr(C)]
#[doc = "*Required features: 'Win32_System_Services', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
pub struct SERVICE_PREFERRED_NODE_INFO {
    pub usPreferredNode: u16,
    pub fDelete: super::super::Foundation::BOOLEAN,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for SERVICE_PREFERRED_NODE_INFO {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for SERVICE_PREFERRED_NODE_INFO {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for SERVICE_PREFERRED_NODE_INFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SERVICE_PREFERRED_NODE_INFO").field("usPreferredNode", &self.usPreferredNode).field("fDelete", &self.fDelete).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for SERVICE_PREFERRED_NODE_INFO {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for SERVICE_PREFERRED_NODE_INFO {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<SERVICE_PREFERRED_NODE_INFO>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for SERVICE_PREFERRED_NODE_INFO {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for SERVICE_PREFERRED_NODE_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: 'Win32_System_Services'*"]
pub struct SERVICE_PRESHUTDOWN_INFO {
    pub dwPreshutdownTimeout: u32,
}
impl ::core::marker::Copy for SERVICE_PRESHUTDOWN_INFO {}
impl ::core::clone::Clone for SERVICE_PRESHUTDOWN_INFO {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for SERVICE_PRESHUTDOWN_INFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SERVICE_PRESHUTDOWN_INFO").field("dwPreshutdownTimeout", &self.dwPreshutdownTimeout).finish()
    }
}
unsafe impl ::windows::core::Abi for SERVICE_PRESHUTDOWN_INFO {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for SERVICE_PRESHUTDOWN_INFO {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<SERVICE_PRESHUTDOWN_INFO>()) == 0 }
    }
}
impl ::core::cmp::Eq for SERVICE_PRESHUTDOWN_INFO {}
impl ::core::default::Default for SERVICE_PRESHUTDOWN_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: 'Win32_System_Services'*"]
pub const SERVICE_QUERY_CONFIG: u32 = 1u32;
#[doc = "*Required features: 'Win32_System_Services'*"]
pub const SERVICE_QUERY_STATUS: u32 = 4u32;
#[doc = "*Required features: 'Win32_System_Services'*"]
pub type SERVICE_REGISTRY_STATE_TYPE = i32;
#[doc = "*Required features: 'Win32_System_Services'*"]
pub const ServiceRegistryStateParameters: SERVICE_REGISTRY_STATE_TYPE = 0i32;
#[doc = "*Required features: 'Win32_System_Services'*"]
pub const ServiceRegistryStatePersistent: SERVICE_REGISTRY_STATE_TYPE = 1i32;
#[doc = "*Required features: 'Win32_System_Services'*"]
pub const MaxServiceRegistryStateType: SERVICE_REGISTRY_STATE_TYPE = 2i32;
#[repr(C)]
#[doc = "*Required features: 'Win32_System_Services', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
pub struct SERVICE_REQUIRED_PRIVILEGES_INFOA {
    pub pmszRequiredPrivileges: super::super::Foundation::PSTR,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for SERVICE_REQUIRED_PRIVILEGES_INFOA {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for SERVICE_REQUIRED_PRIVILEGES_INFOA {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for SERVICE_REQUIRED_PRIVILEGES_INFOA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SERVICE_REQUIRED_PRIVILEGES_INFOA").field("pmszRequiredPrivileges", &self.pmszRequiredPrivileges).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for SERVICE_REQUIRED_PRIVILEGES_INFOA {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for SERVICE_REQUIRED_PRIVILEGES_INFOA {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<SERVICE_REQUIRED_PRIVILEGES_INFOA>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for SERVICE_REQUIRED_PRIVILEGES_INFOA {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for SERVICE_REQUIRED_PRIVILEGES_INFOA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: 'Win32_System_Services', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
pub struct SERVICE_REQUIRED_PRIVILEGES_INFOW {
    pub pmszRequiredPrivileges: super::super::Foundation::PWSTR,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for SERVICE_REQUIRED_PRIVILEGES_INFOW {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for SERVICE_REQUIRED_PRIVILEGES_INFOW {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for SERVICE_REQUIRED_PRIVILEGES_INFOW {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SERVICE_REQUIRED_PRIVILEGES_INFOW").field("pmszRequiredPrivileges", &self.pmszRequiredPrivileges).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for SERVICE_REQUIRED_PRIVILEGES_INFOW {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for SERVICE_REQUIRED_PRIVILEGES_INFOW {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<SERVICE_REQUIRED_PRIVILEGES_INFOW>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for SERVICE_REQUIRED_PRIVILEGES_INFOW {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for SERVICE_REQUIRED_PRIVILEGES_INFOW {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: 'Win32_System_Services'*"]
pub type SERVICE_RUNS_IN_PROCESS = u32;
#[doc = "*Required features: 'Win32_System_Services'*"]
pub const SERVICE_RUNS_IN_NON_SYSTEM_OR_NOT_RUNNING: SERVICE_RUNS_IN_PROCESS = 0u32;
#[doc = "*Required features: 'Win32_System_Services'*"]
pub const SERVICE_RUNS_IN_SYSTEM_PROCESS: SERVICE_RUNS_IN_PROCESS = 1u32;
#[doc = "*Required features: 'Win32_System_Services'*"]
pub type SERVICE_SHARED_DIRECTORY_TYPE = i32;
#[doc = "*Required features: 'Win32_System_Services'*"]
pub const ServiceSharedDirectoryPersistentState: SERVICE_SHARED_DIRECTORY_TYPE = 0i32;
#[doc = "*Required features: 'Win32_System_Services'*"]
pub type SERVICE_SHARED_REGISTRY_STATE_TYPE = i32;
#[doc = "*Required features: 'Win32_System_Services'*"]
pub const ServiceSharedRegistryPersistentState: SERVICE_SHARED_REGISTRY_STATE_TYPE = 0i32;
#[repr(C)]
#[doc = "*Required features: 'Win32_System_Services'*"]
pub struct SERVICE_SID_INFO {
    pub dwServiceSidType: u32,
}
impl ::core::marker::Copy for SERVICE_SID_INFO {}
impl ::core::clone::Clone for SERVICE_SID_INFO {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for SERVICE_SID_INFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SERVICE_SID_INFO").field("dwServiceSidType", &self.dwServiceSidType).finish()
    }
}
unsafe impl ::windows::core::Abi for SERVICE_SID_INFO {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for SERVICE_SID_INFO {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<SERVICE_SID_INFO>()) == 0 }
    }
}
impl ::core::cmp::Eq for SERVICE_SID_INFO {}
impl ::core::default::Default for SERVICE_SID_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: 'Win32_System_Services'*"]
pub const SERVICE_SID_TYPE_NONE: u32 = 0u32;
#[doc = "*Required features: 'Win32_System_Services'*"]
pub const SERVICE_SID_TYPE_UNRESTRICTED: u32 = 1u32;
#[doc = "*Required features: 'Win32_System_Services'*"]
pub const SERVICE_START: u32 = 16u32;
#[repr(C)]
#[doc = "*Required features: 'Win32_System_Services'*"]
pub struct SERVICE_START_REASON {
    pub dwReason: u32,
}
impl ::core::marker::Copy for SERVICE_START_REASON {}
impl ::core::clone::Clone for SERVICE_START_REASON {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for SERVICE_START_REASON {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SERVICE_START_REASON").field("dwReason", &self.dwReason).finish()
    }
}
unsafe impl ::windows::core::Abi for SERVICE_START_REASON {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for SERVICE_START_REASON {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<SERVICE_START_REASON>()) == 0 }
    }
}
impl ::core::cmp::Eq for SERVICE_START_REASON {}
impl ::core::default::Default for SERVICE_START_REASON {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: 'Win32_System_Services'*"]
pub const SERVICE_START_REASON_AUTO: u32 = 2u32;
#[doc = "*Required features: 'Win32_System_Services'*"]
pub const SERVICE_START_REASON_DELAYEDAUTO: u32 = 16u32;
#[doc = "*Required features: 'Win32_System_Services'*"]
pub const SERVICE_START_REASON_DEMAND: u32 = 1u32;
#[doc = "*Required features: 'Win32_System_Services'*"]
pub const SERVICE_START_REASON_RESTART_ON_FAILURE: u32 = 8u32;
#[doc = "*Required features: 'Win32_System_Services'*"]
pub const SERVICE_START_REASON_TRIGGER: u32 = 4u32;
#[doc = "*Required features: 'Win32_System_Services'*"]
pub type SERVICE_START_TYPE = u32;
#[doc = "*Required features: 'Win32_System_Services'*"]
pub const SERVICE_AUTO_START: SERVICE_START_TYPE = 2u32;
#[doc = "*Required features: 'Win32_System_Services'*"]
pub const SERVICE_BOOT_START: SERVICE_START_TYPE = 0u32;
#[doc = "*Required features: 'Win32_System_Services'*"]
pub const SERVICE_DEMAND_START: SERVICE_START_TYPE = 3u32;
#[doc = "*Required features: 'Win32_System_Services'*"]
pub const SERVICE_DISABLED: SERVICE_START_TYPE = 4u32;
#[doc = "*Required features: 'Win32_System_Services'*"]
pub const SERVICE_SYSTEM_START: SERVICE_START_TYPE = 1u32;
#[repr(C)]
#[doc = "*Required features: 'Win32_System_Services'*"]
pub struct SERVICE_STATUS {
    pub dwServiceType: ENUM_SERVICE_TYPE,
    pub dwCurrentState: SERVICE_STATUS_CURRENT_STATE,
    pub dwControlsAccepted: u32,
    pub dwWin32ExitCode: u32,
    pub dwServiceSpecificExitCode: u32,
    pub dwCheckPoint: u32,
    pub dwWaitHint: u32,
}
impl ::core::marker::Copy for SERVICE_STATUS {}
impl ::core::clone::Clone for SERVICE_STATUS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for SERVICE_STATUS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SERVICE_STATUS").field("dwServiceType", &self.dwServiceType).field("dwCurrentState", &self.dwCurrentState).field("dwControlsAccepted", &self.dwControlsAccepted).field("dwWin32ExitCode", &self.dwWin32ExitCode).field("dwServiceSpecificExitCode", &self.dwServiceSpecificExitCode).field("dwCheckPoint", &self.dwCheckPoint).field("dwWaitHint", &self.dwWaitHint).finish()
    }
}
unsafe impl ::windows::core::Abi for SERVICE_STATUS {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for SERVICE_STATUS {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<SERVICE_STATUS>()) == 0 }
    }
}
impl ::core::cmp::Eq for SERVICE_STATUS {}
impl ::core::default::Default for SERVICE_STATUS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: 'Win32_System_Services'*"]
pub type SERVICE_STATUS_CURRENT_STATE = u32;
#[doc = "*Required features: 'Win32_System_Services'*"]
pub const SERVICE_CONTINUE_PENDING: SERVICE_STATUS_CURRENT_STATE = 5u32;
#[doc = "*Required features: 'Win32_System_Services'*"]
pub const SERVICE_PAUSE_PENDING: SERVICE_STATUS_CURRENT_STATE = 6u32;
#[doc = "*Required features: 'Win32_System_Services'*"]
pub const SERVICE_PAUSED: SERVICE_STATUS_CURRENT_STATE = 7u32;
#[doc = "*Required features: 'Win32_System_Services'*"]
pub const SERVICE_RUNNING: SERVICE_STATUS_CURRENT_STATE = 4u32;
#[doc = "*Required features: 'Win32_System_Services'*"]
pub const SERVICE_START_PENDING: SERVICE_STATUS_CURRENT_STATE = 2u32;
#[doc = "*Required features: 'Win32_System_Services'*"]
pub const SERVICE_STOP_PENDING: SERVICE_STATUS_CURRENT_STATE = 3u32;
#[doc = "*Required features: 'Win32_System_Services'*"]
pub const SERVICE_STOPPED: SERVICE_STATUS_CURRENT_STATE = 1u32;
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct SERVICE_STATUS_HANDLE(pub isize);
impl SERVICE_STATUS_HANDLE {
    pub fn is_invalid(&self) -> bool {
        *self == unsafe { ::core::mem::zeroed() }
    }
    pub fn ok(self) -> ::windows::core::Result<Self> {
        if !self.is_invalid() {
            Ok(self)
        } else {
            Err(::windows::core::Error::from_win32())
        }
    }
}
impl ::core::default::Default for SERVICE_STATUS_HANDLE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::clone::Clone for SERVICE_STATUS_HANDLE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::marker::Copy for SERVICE_STATUS_HANDLE {}
impl ::core::fmt::Debug for SERVICE_STATUS_HANDLE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SERVICE_STATUS_HANDLE").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Abi for SERVICE_STATUS_HANDLE {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: 'Win32_System_Services'*"]
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
impl ::core::marker::Copy for SERVICE_STATUS_PROCESS {}
impl ::core::clone::Clone for SERVICE_STATUS_PROCESS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for SERVICE_STATUS_PROCESS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SERVICE_STATUS_PROCESS").field("dwServiceType", &self.dwServiceType).field("dwCurrentState", &self.dwCurrentState).field("dwControlsAccepted", &self.dwControlsAccepted).field("dwWin32ExitCode", &self.dwWin32ExitCode).field("dwServiceSpecificExitCode", &self.dwServiceSpecificExitCode).field("dwCheckPoint", &self.dwCheckPoint).field("dwWaitHint", &self.dwWaitHint).field("dwProcessId", &self.dwProcessId).field("dwServiceFlags", &self.dwServiceFlags).finish()
    }
}
unsafe impl ::windows::core::Abi for SERVICE_STATUS_PROCESS {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for SERVICE_STATUS_PROCESS {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<SERVICE_STATUS_PROCESS>()) == 0 }
    }
}
impl ::core::cmp::Eq for SERVICE_STATUS_PROCESS {}
impl ::core::default::Default for SERVICE_STATUS_PROCESS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: 'Win32_System_Services'*"]
pub const SERVICE_STOP: u32 = 32u32;
#[doc = "*Required features: 'Win32_System_Services'*"]
pub const SERVICE_STOP_REASON_FLAG_CUSTOM: u32 = 536870912u32;
#[doc = "*Required features: 'Win32_System_Services'*"]
pub const SERVICE_STOP_REASON_FLAG_MAX: u32 = 2147483648u32;
#[doc = "*Required features: 'Win32_System_Services'*"]
pub const SERVICE_STOP_REASON_FLAG_MIN: u32 = 0u32;
#[doc = "*Required features: 'Win32_System_Services'*"]
pub const SERVICE_STOP_REASON_FLAG_PLANNED: u32 = 1073741824u32;
#[doc = "*Required features: 'Win32_System_Services'*"]
pub const SERVICE_STOP_REASON_FLAG_UNPLANNED: u32 = 268435456u32;
#[doc = "*Required features: 'Win32_System_Services'*"]
pub const SERVICE_STOP_REASON_MAJOR_APPLICATION: u32 = 327680u32;
#[doc = "*Required features: 'Win32_System_Services'*"]
pub const SERVICE_STOP_REASON_MAJOR_HARDWARE: u32 = 131072u32;
#[doc = "*Required features: 'Win32_System_Services'*"]
pub const SERVICE_STOP_REASON_MAJOR_MAX: u32 = 458752u32;
#[doc = "*Required features: 'Win32_System_Services'*"]
pub const SERVICE_STOP_REASON_MAJOR_MAX_CUSTOM: u32 = 16711680u32;
#[doc = "*Required features: 'Win32_System_Services'*"]
pub const SERVICE_STOP_REASON_MAJOR_MIN: u32 = 0u32;
#[doc = "*Required features: 'Win32_System_Services'*"]
pub const SERVICE_STOP_REASON_MAJOR_MIN_CUSTOM: u32 = 4194304u32;
#[doc = "*Required features: 'Win32_System_Services'*"]
pub const SERVICE_STOP_REASON_MAJOR_NONE: u32 = 393216u32;
#[doc = "*Required features: 'Win32_System_Services'*"]
pub const SERVICE_STOP_REASON_MAJOR_OPERATINGSYSTEM: u32 = 196608u32;
#[doc = "*Required features: 'Win32_System_Services'*"]
pub const SERVICE_STOP_REASON_MAJOR_OTHER: u32 = 65536u32;
#[doc = "*Required features: 'Win32_System_Services'*"]
pub const SERVICE_STOP_REASON_MAJOR_SOFTWARE: u32 = 262144u32;
#[doc = "*Required features: 'Win32_System_Services'*"]
pub const SERVICE_STOP_REASON_MINOR_DISK: u32 = 8u32;
#[doc = "*Required features: 'Win32_System_Services'*"]
pub const SERVICE_STOP_REASON_MINOR_ENVIRONMENT: u32 = 10u32;
#[doc = "*Required features: 'Win32_System_Services'*"]
pub const SERVICE_STOP_REASON_MINOR_HARDWARE_DRIVER: u32 = 11u32;
#[doc = "*Required features: 'Win32_System_Services'*"]
pub const SERVICE_STOP_REASON_MINOR_HUNG: u32 = 6u32;
#[doc = "*Required features: 'Win32_System_Services'*"]
pub const SERVICE_STOP_REASON_MINOR_INSTALLATION: u32 = 3u32;
#[doc = "*Required features: 'Win32_System_Services'*"]
pub const SERVICE_STOP_REASON_MINOR_MAINTENANCE: u32 = 2u32;
#[doc = "*Required features: 'Win32_System_Services'*"]
pub const SERVICE_STOP_REASON_MINOR_MAX: u32 = 25u32;
#[doc = "*Required features: 'Win32_System_Services'*"]
pub const SERVICE_STOP_REASON_MINOR_MAX_CUSTOM: u32 = 65535u32;
#[doc = "*Required features: 'Win32_System_Services'*"]
pub const SERVICE_STOP_REASON_MINOR_MEMOTYLIMIT: u32 = 24u32;
#[doc = "*Required features: 'Win32_System_Services'*"]
pub const SERVICE_STOP_REASON_MINOR_MIN: u32 = 0u32;
#[doc = "*Required features: 'Win32_System_Services'*"]
pub const SERVICE_STOP_REASON_MINOR_MIN_CUSTOM: u32 = 256u32;
#[doc = "*Required features: 'Win32_System_Services'*"]
pub const SERVICE_STOP_REASON_MINOR_MMC: u32 = 22u32;
#[doc = "*Required features: 'Win32_System_Services'*"]
pub const SERVICE_STOP_REASON_MINOR_NETWORKCARD: u32 = 9u32;
#[doc = "*Required features: 'Win32_System_Services'*"]
pub const SERVICE_STOP_REASON_MINOR_NETWORK_CONNECTIVITY: u32 = 17u32;
#[doc = "*Required features: 'Win32_System_Services'*"]
pub const SERVICE_STOP_REASON_MINOR_NONE: u32 = 23u32;
#[doc = "*Required features: 'Win32_System_Services'*"]
pub const SERVICE_STOP_REASON_MINOR_OTHER: u32 = 1u32;
#[doc = "*Required features: 'Win32_System_Services'*"]
pub const SERVICE_STOP_REASON_MINOR_OTHERDRIVER: u32 = 12u32;
#[doc = "*Required features: 'Win32_System_Services'*"]
pub const SERVICE_STOP_REASON_MINOR_RECONFIG: u32 = 5u32;
#[doc = "*Required features: 'Win32_System_Services'*"]
pub const SERVICE_STOP_REASON_MINOR_SECURITY: u32 = 16u32;
#[doc = "*Required features: 'Win32_System_Services'*"]
pub const SERVICE_STOP_REASON_MINOR_SECURITYFIX: u32 = 15u32;
#[doc = "*Required features: 'Win32_System_Services'*"]
pub const SERVICE_STOP_REASON_MINOR_SECURITYFIX_UNINSTALL: u32 = 21u32;
#[doc = "*Required features: 'Win32_System_Services'*"]
pub const SERVICE_STOP_REASON_MINOR_SERVICEPACK: u32 = 13u32;
#[doc = "*Required features: 'Win32_System_Services'*"]
pub const SERVICE_STOP_REASON_MINOR_SERVICEPACK_UNINSTALL: u32 = 19u32;
#[doc = "*Required features: 'Win32_System_Services'*"]
pub const SERVICE_STOP_REASON_MINOR_SOFTWARE_UPDATE: u32 = 14u32;
#[doc = "*Required features: 'Win32_System_Services'*"]
pub const SERVICE_STOP_REASON_MINOR_SOFTWARE_UPDATE_UNINSTALL: u32 = 20u32;
#[doc = "*Required features: 'Win32_System_Services'*"]
pub const SERVICE_STOP_REASON_MINOR_UNSTABLE: u32 = 7u32;
#[doc = "*Required features: 'Win32_System_Services'*"]
pub const SERVICE_STOP_REASON_MINOR_UPGRADE: u32 = 4u32;
#[doc = "*Required features: 'Win32_System_Services'*"]
pub const SERVICE_STOP_REASON_MINOR_WMI: u32 = 18u32;
#[repr(C)]
#[doc = "*Required features: 'Win32_System_Services', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
pub struct SERVICE_TABLE_ENTRYA {
    pub lpServiceName: super::super::Foundation::PSTR,
    pub lpServiceProc: LPSERVICE_MAIN_FUNCTIONA,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for SERVICE_TABLE_ENTRYA {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for SERVICE_TABLE_ENTRYA {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for SERVICE_TABLE_ENTRYA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SERVICE_TABLE_ENTRYA").field("lpServiceName", &self.lpServiceName).field("lpServiceProc", &self.lpServiceProc.map(|f| f as usize)).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for SERVICE_TABLE_ENTRYA {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for SERVICE_TABLE_ENTRYA {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<SERVICE_TABLE_ENTRYA>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for SERVICE_TABLE_ENTRYA {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for SERVICE_TABLE_ENTRYA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: 'Win32_System_Services', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
pub struct SERVICE_TABLE_ENTRYW {
    pub lpServiceName: super::super::Foundation::PWSTR,
    pub lpServiceProc: LPSERVICE_MAIN_FUNCTIONW,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for SERVICE_TABLE_ENTRYW {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for SERVICE_TABLE_ENTRYW {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for SERVICE_TABLE_ENTRYW {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SERVICE_TABLE_ENTRYW").field("lpServiceName", &self.lpServiceName).field("lpServiceProc", &self.lpServiceProc.map(|f| f as usize)).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for SERVICE_TABLE_ENTRYW {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for SERVICE_TABLE_ENTRYW {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<SERVICE_TABLE_ENTRYW>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for SERVICE_TABLE_ENTRYW {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for SERVICE_TABLE_ENTRYW {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: 'Win32_System_Services'*"]
pub struct SERVICE_TIMECHANGE_INFO {
    pub liNewTime: i64,
    pub liOldTime: i64,
}
impl ::core::marker::Copy for SERVICE_TIMECHANGE_INFO {}
impl ::core::clone::Clone for SERVICE_TIMECHANGE_INFO {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for SERVICE_TIMECHANGE_INFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SERVICE_TIMECHANGE_INFO").field("liNewTime", &self.liNewTime).field("liOldTime", &self.liOldTime).finish()
    }
}
unsafe impl ::windows::core::Abi for SERVICE_TIMECHANGE_INFO {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for SERVICE_TIMECHANGE_INFO {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<SERVICE_TIMECHANGE_INFO>()) == 0 }
    }
}
impl ::core::cmp::Eq for SERVICE_TIMECHANGE_INFO {}
impl ::core::default::Default for SERVICE_TIMECHANGE_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: 'Win32_System_Services'*"]
pub struct SERVICE_TRIGGER {
    pub dwTriggerType: SERVICE_TRIGGER_TYPE,
    pub dwAction: SERVICE_TRIGGER_ACTION,
    pub pTriggerSubtype: *mut ::windows::core::GUID,
    pub cDataItems: u32,
    pub pDataItems: *mut SERVICE_TRIGGER_SPECIFIC_DATA_ITEM,
}
impl ::core::marker::Copy for SERVICE_TRIGGER {}
impl ::core::clone::Clone for SERVICE_TRIGGER {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for SERVICE_TRIGGER {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SERVICE_TRIGGER").field("dwTriggerType", &self.dwTriggerType).field("dwAction", &self.dwAction).field("pTriggerSubtype", &self.pTriggerSubtype).field("cDataItems", &self.cDataItems).field("pDataItems", &self.pDataItems).finish()
    }
}
unsafe impl ::windows::core::Abi for SERVICE_TRIGGER {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for SERVICE_TRIGGER {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<SERVICE_TRIGGER>()) == 0 }
    }
}
impl ::core::cmp::Eq for SERVICE_TRIGGER {}
impl ::core::default::Default for SERVICE_TRIGGER {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: 'Win32_System_Services'*"]
pub type SERVICE_TRIGGER_ACTION = u32;
#[doc = "*Required features: 'Win32_System_Services'*"]
pub const SERVICE_TRIGGER_ACTION_SERVICE_START: SERVICE_TRIGGER_ACTION = 1u32;
#[doc = "*Required features: 'Win32_System_Services'*"]
pub const SERVICE_TRIGGER_ACTION_SERVICE_STOP: SERVICE_TRIGGER_ACTION = 2u32;
#[repr(C)]
#[doc = "*Required features: 'Win32_System_Services'*"]
pub struct SERVICE_TRIGGER_CUSTOM_STATE_ID {
    pub Data: [u32; 2],
}
impl ::core::marker::Copy for SERVICE_TRIGGER_CUSTOM_STATE_ID {}
impl ::core::clone::Clone for SERVICE_TRIGGER_CUSTOM_STATE_ID {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for SERVICE_TRIGGER_CUSTOM_STATE_ID {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SERVICE_TRIGGER_CUSTOM_STATE_ID").field("Data", &self.Data).finish()
    }
}
unsafe impl ::windows::core::Abi for SERVICE_TRIGGER_CUSTOM_STATE_ID {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for SERVICE_TRIGGER_CUSTOM_STATE_ID {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<SERVICE_TRIGGER_CUSTOM_STATE_ID>()) == 0 }
    }
}
impl ::core::cmp::Eq for SERVICE_TRIGGER_CUSTOM_STATE_ID {}
impl ::core::default::Default for SERVICE_TRIGGER_CUSTOM_STATE_ID {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: 'Win32_System_Services'*"]
pub struct SERVICE_TRIGGER_INFO {
    pub cTriggers: u32,
    pub pTriggers: *mut SERVICE_TRIGGER,
    pub pReserved: *mut u8,
}
impl ::core::marker::Copy for SERVICE_TRIGGER_INFO {}
impl ::core::clone::Clone for SERVICE_TRIGGER_INFO {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for SERVICE_TRIGGER_INFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SERVICE_TRIGGER_INFO").field("cTriggers", &self.cTriggers).field("pTriggers", &self.pTriggers).field("pReserved", &self.pReserved).finish()
    }
}
unsafe impl ::windows::core::Abi for SERVICE_TRIGGER_INFO {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for SERVICE_TRIGGER_INFO {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<SERVICE_TRIGGER_INFO>()) == 0 }
    }
}
impl ::core::cmp::Eq for SERVICE_TRIGGER_INFO {}
impl ::core::default::Default for SERVICE_TRIGGER_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: 'Win32_System_Services'*"]
pub struct SERVICE_TRIGGER_SPECIFIC_DATA_ITEM {
    pub dwDataType: SERVICE_TRIGGER_SPECIFIC_DATA_ITEM_DATA_TYPE,
    pub cbData: u32,
    pub pData: *mut u8,
}
impl ::core::marker::Copy for SERVICE_TRIGGER_SPECIFIC_DATA_ITEM {}
impl ::core::clone::Clone for SERVICE_TRIGGER_SPECIFIC_DATA_ITEM {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for SERVICE_TRIGGER_SPECIFIC_DATA_ITEM {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SERVICE_TRIGGER_SPECIFIC_DATA_ITEM").field("dwDataType", &self.dwDataType).field("cbData", &self.cbData).field("pData", &self.pData).finish()
    }
}
unsafe impl ::windows::core::Abi for SERVICE_TRIGGER_SPECIFIC_DATA_ITEM {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for SERVICE_TRIGGER_SPECIFIC_DATA_ITEM {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<SERVICE_TRIGGER_SPECIFIC_DATA_ITEM>()) == 0 }
    }
}
impl ::core::cmp::Eq for SERVICE_TRIGGER_SPECIFIC_DATA_ITEM {}
impl ::core::default::Default for SERVICE_TRIGGER_SPECIFIC_DATA_ITEM {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: 'Win32_System_Services'*"]
pub type SERVICE_TRIGGER_SPECIFIC_DATA_ITEM_DATA_TYPE = u32;
#[doc = "*Required features: 'Win32_System_Services'*"]
pub const SERVICE_TRIGGER_DATA_TYPE_BINARY: SERVICE_TRIGGER_SPECIFIC_DATA_ITEM_DATA_TYPE = 1u32;
#[doc = "*Required features: 'Win32_System_Services'*"]
pub const SERVICE_TRIGGER_DATA_TYPE_STRING: SERVICE_TRIGGER_SPECIFIC_DATA_ITEM_DATA_TYPE = 2u32;
#[doc = "*Required features: 'Win32_System_Services'*"]
pub const SERVICE_TRIGGER_DATA_TYPE_LEVEL: SERVICE_TRIGGER_SPECIFIC_DATA_ITEM_DATA_TYPE = 3u32;
#[doc = "*Required features: 'Win32_System_Services'*"]
pub const SERVICE_TRIGGER_DATA_TYPE_KEYWORD_ANY: SERVICE_TRIGGER_SPECIFIC_DATA_ITEM_DATA_TYPE = 4u32;
#[doc = "*Required features: 'Win32_System_Services'*"]
pub const SERVICE_TRIGGER_DATA_TYPE_KEYWORD_ALL: SERVICE_TRIGGER_SPECIFIC_DATA_ITEM_DATA_TYPE = 5u32;
#[doc = "*Required features: 'Win32_System_Services'*"]
pub type SERVICE_TRIGGER_TYPE = u32;
#[doc = "*Required features: 'Win32_System_Services'*"]
pub const SERVICE_TRIGGER_TYPE_CUSTOM: SERVICE_TRIGGER_TYPE = 20u32;
#[doc = "*Required features: 'Win32_System_Services'*"]
pub const SERVICE_TRIGGER_TYPE_DEVICE_INTERFACE_ARRIVAL: SERVICE_TRIGGER_TYPE = 1u32;
#[doc = "*Required features: 'Win32_System_Services'*"]
pub const SERVICE_TRIGGER_TYPE_DOMAIN_JOIN: SERVICE_TRIGGER_TYPE = 3u32;
#[doc = "*Required features: 'Win32_System_Services'*"]
pub const SERVICE_TRIGGER_TYPE_FIREWALL_PORT_EVENT: SERVICE_TRIGGER_TYPE = 4u32;
#[doc = "*Required features: 'Win32_System_Services'*"]
pub const SERVICE_TRIGGER_TYPE_GROUP_POLICY: SERVICE_TRIGGER_TYPE = 5u32;
#[doc = "*Required features: 'Win32_System_Services'*"]
pub const SERVICE_TRIGGER_TYPE_IP_ADDRESS_AVAILABILITY: SERVICE_TRIGGER_TYPE = 2u32;
#[doc = "*Required features: 'Win32_System_Services'*"]
pub const SERVICE_TRIGGER_TYPE_NETWORK_ENDPOINT: SERVICE_TRIGGER_TYPE = 6u32;
#[doc = "*Required features: 'Win32_System_Services'*"]
pub const SERVICE_TRIGGER_TYPE_AGGREGATE: u32 = 30u32;
#[doc = "*Required features: 'Win32_System_Services'*"]
pub const SERVICE_TRIGGER_TYPE_CUSTOM_SYSTEM_STATE_CHANGE: u32 = 7u32;
#[doc = "*Required features: 'Win32_System_Services'*"]
pub const SERVICE_USER_DEFINED_CONTROL: u32 = 256u32;
#[doc = "*Required features: 'Win32_System_Services', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn SetServiceBits<'a, Param0: ::windows::core::IntoParam<'a, SERVICE_STATUS_HANDLE>, Param2: ::windows::core::IntoParam<'a, super::super::Foundation::BOOL>, Param3: ::windows::core::IntoParam<'a, super::super::Foundation::BOOL>>(hservicestatus: Param0, dwservicebits: u32, bsetbitson: Param2, bupdateimmediately: Param3) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn SetServiceBits(hservicestatus: SERVICE_STATUS_HANDLE, dwservicebits: u32, bsetbitson: super::super::Foundation::BOOL, bupdateimmediately: super::super::Foundation::BOOL) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(SetServiceBits(hservicestatus.into_param().abi(), ::core::mem::transmute(dwservicebits), bsetbitson.into_param().abi(), bupdateimmediately.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_System_Services', 'Win32_Foundation', 'Win32_Security'*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
#[inline]
pub unsafe fn SetServiceObjectSecurity<'a, Param0: ::windows::core::IntoParam<'a, super::super::Security::SC_HANDLE>>(hservice: Param0, dwsecurityinformation: super::super::Security::OBJECT_SECURITY_INFORMATION, lpsecuritydescriptor: *const super::super::Security::SECURITY_DESCRIPTOR) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn SetServiceObjectSecurity(hservice: super::super::Security::SC_HANDLE, dwsecurityinformation: super::super::Security::OBJECT_SECURITY_INFORMATION, lpsecuritydescriptor: *const super::super::Security::SECURITY_DESCRIPTOR) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(SetServiceObjectSecurity(hservice.into_param().abi(), ::core::mem::transmute(dwsecurityinformation), ::core::mem::transmute(lpsecuritydescriptor)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_System_Services', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn SetServiceStatus<'a, Param0: ::windows::core::IntoParam<'a, SERVICE_STATUS_HANDLE>>(hservicestatus: Param0, lpservicestatus: *const SERVICE_STATUS) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn SetServiceStatus(hservicestatus: SERVICE_STATUS_HANDLE, lpservicestatus: *const SERVICE_STATUS) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(SetServiceStatus(hservicestatus.into_param().abi(), ::core::mem::transmute(lpservicestatus)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_System_Services', 'Win32_Foundation', 'Win32_Security'*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
#[inline]
pub unsafe fn StartServiceA<'a, Param0: ::windows::core::IntoParam<'a, super::super::Security::SC_HANDLE>>(hservice: Param0, dwnumserviceargs: u32, lpserviceargvectors: *const super::super::Foundation::PSTR) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn StartServiceA(hservice: super::super::Security::SC_HANDLE, dwnumserviceargs: u32, lpserviceargvectors: *const super::super::Foundation::PSTR) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(StartServiceA(hservice.into_param().abi(), ::core::mem::transmute(dwnumserviceargs), ::core::mem::transmute(lpserviceargvectors)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_System_Services', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn StartServiceCtrlDispatcherA(lpservicestarttable: *const SERVICE_TABLE_ENTRYA) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn StartServiceCtrlDispatcherA(lpservicestarttable: *const SERVICE_TABLE_ENTRYA) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(StartServiceCtrlDispatcherA(::core::mem::transmute(lpservicestarttable)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_System_Services', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn StartServiceCtrlDispatcherW(lpservicestarttable: *const SERVICE_TABLE_ENTRYW) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn StartServiceCtrlDispatcherW(lpservicestarttable: *const SERVICE_TABLE_ENTRYW) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(StartServiceCtrlDispatcherW(::core::mem::transmute(lpservicestarttable)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_System_Services', 'Win32_Foundation', 'Win32_Security'*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
#[inline]
pub unsafe fn StartServiceW<'a, Param0: ::windows::core::IntoParam<'a, super::super::Security::SC_HANDLE>>(hservice: Param0, dwnumserviceargs: u32, lpserviceargvectors: *const super::super::Foundation::PWSTR) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn StartServiceW(hservice: super::super::Security::SC_HANDLE, dwnumserviceargs: u32, lpserviceargvectors: *const super::super::Foundation::PWSTR) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(StartServiceW(hservice.into_param().abi(), ::core::mem::transmute(dwnumserviceargs), ::core::mem::transmute(lpserviceargvectors)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
pub const USER_POLICY_PRESENT_GUID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x54fb46c8_f089_464c_b1fd_59d1b62c3b50);
#[doc = "*Required features: 'Win32_System_Services', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn UnlockServiceDatabase(sclock: *const ::core::ffi::c_void) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn UnlockServiceDatabase(sclock: *const ::core::ffi::c_void) -> super::super::Foundation::BOOL;
        }
        ::core::mem::transmute(UnlockServiceDatabase(::core::mem::transmute(sclock)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_System_Services', 'Win32_Foundation', 'Win32_Security'*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
#[inline]
pub unsafe fn WaitServiceState<'a, Param0: ::windows::core::IntoParam<'a, super::super::Security::SC_HANDLE>, Param3: ::windows::core::IntoParam<'a, super::super::Foundation::HANDLE>>(hservice: Param0, dwnotify: u32, dwtimeout: u32, hcancelevent: Param3) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn WaitServiceState(hservice: super::super::Security::SC_HANDLE, dwnotify: u32, dwtimeout: u32, hcancelevent: super::super::Foundation::HANDLE) -> u32;
        }
        ::core::mem::transmute(WaitServiceState(hservice.into_param().abi(), ::core::mem::transmute(dwnotify), ::core::mem::transmute(dwtimeout), hcancelevent.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[repr(C)]
pub struct _SC_NOTIFICATION_REGISTRATION(pub u8);

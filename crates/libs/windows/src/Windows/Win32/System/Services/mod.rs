pub const CUSTOM_SYSTEM_STATE_CHANGE_EVENT_GUID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x2d7a2816_0c5e_45fc_9ce7_570e5ecde9c9);
#[doc = "*Required features: `\"Win32_System_Services\"`, `\"Win32_Foundation\"`, `\"Win32_Security\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
#[inline]
pub unsafe fn ChangeServiceConfig2A<'a, P0>(hservice: P0, dwinfolevel: SERVICE_CONFIG, lpinfo: *const ::core::ffi::c_void) -> super::super::Foundation::BOOL
where
    P0: ::std::convert::Into<super::super::Security::SC_HANDLE>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn ChangeServiceConfig2A(hservice: super::super::Security::SC_HANDLE, dwinfolevel: SERVICE_CONFIG, lpinfo: *const ::core::ffi::c_void) -> super::super::Foundation::BOOL;
    }
    ChangeServiceConfig2A(hservice.into(), dwinfolevel, ::core::mem::transmute(lpinfo))
}
#[doc = "*Required features: `\"Win32_System_Services\"`, `\"Win32_Foundation\"`, `\"Win32_Security\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
#[inline]
pub unsafe fn ChangeServiceConfig2W<'a, P0>(hservice: P0, dwinfolevel: SERVICE_CONFIG, lpinfo: *const ::core::ffi::c_void) -> super::super::Foundation::BOOL
where
    P0: ::std::convert::Into<super::super::Security::SC_HANDLE>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn ChangeServiceConfig2W(hservice: super::super::Security::SC_HANDLE, dwinfolevel: SERVICE_CONFIG, lpinfo: *const ::core::ffi::c_void) -> super::super::Foundation::BOOL;
    }
    ChangeServiceConfig2W(hservice.into(), dwinfolevel, ::core::mem::transmute(lpinfo))
}
#[doc = "*Required features: `\"Win32_System_Services\"`, `\"Win32_Foundation\"`, `\"Win32_Security\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
#[inline]
pub unsafe fn ChangeServiceConfigA<'a, P0, P1, P2, P3, P4, P5, P6>(hservice: P0, dwservicetype: u32, dwstarttype: SERVICE_START_TYPE, dwerrorcontrol: SERVICE_ERROR, lpbinarypathname: P1, lploadordergroup: P2, lpdwtagid: *mut u32, lpdependencies: P3, lpservicestartname: P4, lppassword: P5, lpdisplayname: P6) -> super::super::Foundation::BOOL
where
    P0: ::std::convert::Into<super::super::Security::SC_HANDLE>,
    P1: ::std::convert::Into<::windows::core::PCSTR>,
    P2: ::std::convert::Into<::windows::core::PCSTR>,
    P3: ::std::convert::Into<::windows::core::PCSTR>,
    P4: ::std::convert::Into<::windows::core::PCSTR>,
    P5: ::std::convert::Into<::windows::core::PCSTR>,
    P6: ::std::convert::Into<::windows::core::PCSTR>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn ChangeServiceConfigA(hservice: super::super::Security::SC_HANDLE, dwservicetype: u32, dwstarttype: SERVICE_START_TYPE, dwerrorcontrol: SERVICE_ERROR, lpbinarypathname: ::windows::core::PCSTR, lploadordergroup: ::windows::core::PCSTR, lpdwtagid: *mut u32, lpdependencies: ::windows::core::PCSTR, lpservicestartname: ::windows::core::PCSTR, lppassword: ::windows::core::PCSTR, lpdisplayname: ::windows::core::PCSTR) -> super::super::Foundation::BOOL;
    }
    ChangeServiceConfigA(hservice.into(), dwservicetype, dwstarttype, dwerrorcontrol, lpbinarypathname.into(), lploadordergroup.into(), ::core::mem::transmute(lpdwtagid), lpdependencies.into(), lpservicestartname.into(), lppassword.into(), lpdisplayname.into())
}
#[doc = "*Required features: `\"Win32_System_Services\"`, `\"Win32_Foundation\"`, `\"Win32_Security\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
#[inline]
pub unsafe fn ChangeServiceConfigW<'a, P0, P1, P2, P3, P4, P5, P6>(hservice: P0, dwservicetype: u32, dwstarttype: SERVICE_START_TYPE, dwerrorcontrol: SERVICE_ERROR, lpbinarypathname: P1, lploadordergroup: P2, lpdwtagid: *mut u32, lpdependencies: P3, lpservicestartname: P4, lppassword: P5, lpdisplayname: P6) -> super::super::Foundation::BOOL
where
    P0: ::std::convert::Into<super::super::Security::SC_HANDLE>,
    P1: ::std::convert::Into<::windows::core::PCWSTR>,
    P2: ::std::convert::Into<::windows::core::PCWSTR>,
    P3: ::std::convert::Into<::windows::core::PCWSTR>,
    P4: ::std::convert::Into<::windows::core::PCWSTR>,
    P5: ::std::convert::Into<::windows::core::PCWSTR>,
    P6: ::std::convert::Into<::windows::core::PCWSTR>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn ChangeServiceConfigW(hservice: super::super::Security::SC_HANDLE, dwservicetype: u32, dwstarttype: SERVICE_START_TYPE, dwerrorcontrol: SERVICE_ERROR, lpbinarypathname: ::windows::core::PCWSTR, lploadordergroup: ::windows::core::PCWSTR, lpdwtagid: *mut u32, lpdependencies: ::windows::core::PCWSTR, lpservicestartname: ::windows::core::PCWSTR, lppassword: ::windows::core::PCWSTR, lpdisplayname: ::windows::core::PCWSTR) -> super::super::Foundation::BOOL;
    }
    ChangeServiceConfigW(hservice.into(), dwservicetype, dwstarttype, dwerrorcontrol, lpbinarypathname.into(), lploadordergroup.into(), ::core::mem::transmute(lpdwtagid), lpdependencies.into(), lpservicestartname.into(), lppassword.into(), lpdisplayname.into())
}
#[doc = "*Required features: `\"Win32_System_Services\"`, `\"Win32_Foundation\"`, `\"Win32_Security\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
#[inline]
pub unsafe fn CloseServiceHandle<'a, P0>(hscobject: P0) -> super::super::Foundation::BOOL
where
    P0: ::std::convert::Into<super::super::Security::SC_HANDLE>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn CloseServiceHandle(hscobject: super::super::Security::SC_HANDLE) -> super::super::Foundation::BOOL;
    }
    CloseServiceHandle(hscobject.into())
}
#[doc = "*Required features: `\"Win32_System_Services\"`, `\"Win32_Foundation\"`, `\"Win32_Security\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
#[inline]
pub unsafe fn ControlService<'a, P0>(hservice: P0, dwcontrol: u32, lpservicestatus: *mut SERVICE_STATUS) -> super::super::Foundation::BOOL
where
    P0: ::std::convert::Into<super::super::Security::SC_HANDLE>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn ControlService(hservice: super::super::Security::SC_HANDLE, dwcontrol: u32, lpservicestatus: *mut SERVICE_STATUS) -> super::super::Foundation::BOOL;
    }
    ControlService(hservice.into(), dwcontrol, ::core::mem::transmute(lpservicestatus))
}
#[doc = "*Required features: `\"Win32_System_Services\"`, `\"Win32_Foundation\"`, `\"Win32_Security\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
#[inline]
pub unsafe fn ControlServiceExA<'a, P0>(hservice: P0, dwcontrol: u32, dwinfolevel: u32, pcontrolparams: *mut ::core::ffi::c_void) -> super::super::Foundation::BOOL
where
    P0: ::std::convert::Into<super::super::Security::SC_HANDLE>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn ControlServiceExA(hservice: super::super::Security::SC_HANDLE, dwcontrol: u32, dwinfolevel: u32, pcontrolparams: *mut ::core::ffi::c_void) -> super::super::Foundation::BOOL;
    }
    ControlServiceExA(hservice.into(), dwcontrol, dwinfolevel, ::core::mem::transmute(pcontrolparams))
}
#[doc = "*Required features: `\"Win32_System_Services\"`, `\"Win32_Foundation\"`, `\"Win32_Security\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
#[inline]
pub unsafe fn ControlServiceExW<'a, P0>(hservice: P0, dwcontrol: u32, dwinfolevel: u32, pcontrolparams: *mut ::core::ffi::c_void) -> super::super::Foundation::BOOL
where
    P0: ::std::convert::Into<super::super::Security::SC_HANDLE>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn ControlServiceExW(hservice: super::super::Security::SC_HANDLE, dwcontrol: u32, dwinfolevel: u32, pcontrolparams: *mut ::core::ffi::c_void) -> super::super::Foundation::BOOL;
    }
    ControlServiceExW(hservice.into(), dwcontrol, dwinfolevel, ::core::mem::transmute(pcontrolparams))
}
#[doc = "*Required features: `\"Win32_System_Services\"`, `\"Win32_Security\"`*"]
#[cfg(feature = "Win32_Security")]
#[inline]
pub unsafe fn CreateServiceA<'a, P0, P1, P2, P3, P4, P5, P6, P7>(hscmanager: P0, lpservicename: P1, lpdisplayname: P2, dwdesiredaccess: u32, dwservicetype: ENUM_SERVICE_TYPE, dwstarttype: SERVICE_START_TYPE, dwerrorcontrol: SERVICE_ERROR, lpbinarypathname: P3, lploadordergroup: P4, lpdwtagid: *mut u32, lpdependencies: P5, lpservicestartname: P6, lppassword: P7) -> ::windows::core::Result<super::super::Security::SC_HANDLE>
where
    P0: ::std::convert::Into<super::super::Security::SC_HANDLE>,
    P1: ::std::convert::Into<::windows::core::PCSTR>,
    P2: ::std::convert::Into<::windows::core::PCSTR>,
    P3: ::std::convert::Into<::windows::core::PCSTR>,
    P4: ::std::convert::Into<::windows::core::PCSTR>,
    P5: ::std::convert::Into<::windows::core::PCSTR>,
    P6: ::std::convert::Into<::windows::core::PCSTR>,
    P7: ::std::convert::Into<::windows::core::PCSTR>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn CreateServiceA(hscmanager: super::super::Security::SC_HANDLE, lpservicename: ::windows::core::PCSTR, lpdisplayname: ::windows::core::PCSTR, dwdesiredaccess: u32, dwservicetype: ENUM_SERVICE_TYPE, dwstarttype: SERVICE_START_TYPE, dwerrorcontrol: SERVICE_ERROR, lpbinarypathname: ::windows::core::PCSTR, lploadordergroup: ::windows::core::PCSTR, lpdwtagid: *mut u32, lpdependencies: ::windows::core::PCSTR, lpservicestartname: ::windows::core::PCSTR, lppassword: ::windows::core::PCSTR) -> super::super::Security::SC_HANDLE;
    }
    let result__ = CreateServiceA(hscmanager.into(), lpservicename.into(), lpdisplayname.into(), dwdesiredaccess, dwservicetype, dwstarttype, dwerrorcontrol, lpbinarypathname.into(), lploadordergroup.into(), ::core::mem::transmute(lpdwtagid), lpdependencies.into(), lpservicestartname.into(), lppassword.into());
    (!result__.is_invalid()).then(|| result__).ok_or_else(::windows::core::Error::from_win32)
}
#[doc = "*Required features: `\"Win32_System_Services\"`, `\"Win32_Security\"`*"]
#[cfg(feature = "Win32_Security")]
#[inline]
pub unsafe fn CreateServiceW<'a, P0, P1, P2, P3, P4, P5, P6, P7>(hscmanager: P0, lpservicename: P1, lpdisplayname: P2, dwdesiredaccess: u32, dwservicetype: ENUM_SERVICE_TYPE, dwstarttype: SERVICE_START_TYPE, dwerrorcontrol: SERVICE_ERROR, lpbinarypathname: P3, lploadordergroup: P4, lpdwtagid: *mut u32, lpdependencies: P5, lpservicestartname: P6, lppassword: P7) -> ::windows::core::Result<super::super::Security::SC_HANDLE>
where
    P0: ::std::convert::Into<super::super::Security::SC_HANDLE>,
    P1: ::std::convert::Into<::windows::core::PCWSTR>,
    P2: ::std::convert::Into<::windows::core::PCWSTR>,
    P3: ::std::convert::Into<::windows::core::PCWSTR>,
    P4: ::std::convert::Into<::windows::core::PCWSTR>,
    P5: ::std::convert::Into<::windows::core::PCWSTR>,
    P6: ::std::convert::Into<::windows::core::PCWSTR>,
    P7: ::std::convert::Into<::windows::core::PCWSTR>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn CreateServiceW(hscmanager: super::super::Security::SC_HANDLE, lpservicename: ::windows::core::PCWSTR, lpdisplayname: ::windows::core::PCWSTR, dwdesiredaccess: u32, dwservicetype: ENUM_SERVICE_TYPE, dwstarttype: SERVICE_START_TYPE, dwerrorcontrol: SERVICE_ERROR, lpbinarypathname: ::windows::core::PCWSTR, lploadordergroup: ::windows::core::PCWSTR, lpdwtagid: *mut u32, lpdependencies: ::windows::core::PCWSTR, lpservicestartname: ::windows::core::PCWSTR, lppassword: ::windows::core::PCWSTR) -> super::super::Security::SC_HANDLE;
    }
    let result__ = CreateServiceW(hscmanager.into(), lpservicename.into(), lpdisplayname.into(), dwdesiredaccess, dwservicetype, dwstarttype, dwerrorcontrol, lpbinarypathname.into(), lploadordergroup.into(), ::core::mem::transmute(lpdwtagid), lpdependencies.into(), lpservicestartname.into(), lppassword.into());
    (!result__.is_invalid()).then(|| result__).ok_or_else(::windows::core::Error::from_win32)
}
pub const DOMAIN_JOIN_GUID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x1ce20aba_9851_4421_9430_1ddeb766e809);
pub const DOMAIN_LEAVE_GUID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xddaf516e_58c2_4866_9574_c3b615d42ea1);
#[doc = "*Required features: `\"Win32_System_Services\"`, `\"Win32_Foundation\"`, `\"Win32_Security\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
#[inline]
pub unsafe fn DeleteService<'a, P0>(hservice: P0) -> super::super::Foundation::BOOL
where
    P0: ::std::convert::Into<super::super::Security::SC_HANDLE>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn DeleteService(hservice: super::super::Security::SC_HANDLE) -> super::super::Foundation::BOOL;
    }
    DeleteService(hservice.into())
}
#[doc = "*Required features: `\"Win32_System_Services\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct ENUM_SERVICE_STATE(pub u32);
#[doc = "*Required features: `\"Win32_System_Services\"`*"]
pub const SERVICE_ACTIVE: ENUM_SERVICE_STATE = ENUM_SERVICE_STATE(1u32);
#[doc = "*Required features: `\"Win32_System_Services\"`*"]
pub const SERVICE_INACTIVE: ENUM_SERVICE_STATE = ENUM_SERVICE_STATE(2u32);
#[doc = "*Required features: `\"Win32_System_Services\"`*"]
pub const SERVICE_STATE_ALL: ENUM_SERVICE_STATE = ENUM_SERVICE_STATE(3u32);
impl ::core::marker::Copy for ENUM_SERVICE_STATE {}
impl ::core::clone::Clone for ENUM_SERVICE_STATE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for ENUM_SERVICE_STATE {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for ENUM_SERVICE_STATE {
    type Abi = Self;
}
impl ::core::fmt::Debug for ENUM_SERVICE_STATE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ENUM_SERVICE_STATE").field(&self.0).finish()
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Services\"`*"]
pub struct ENUM_SERVICE_STATUSA {
    pub lpServiceName: ::windows::core::PSTR,
    pub lpDisplayName: ::windows::core::PSTR,
    pub ServiceStatus: SERVICE_STATUS,
}
impl ::core::marker::Copy for ENUM_SERVICE_STATUSA {}
impl ::core::clone::Clone for ENUM_SERVICE_STATUSA {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for ENUM_SERVICE_STATUSA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("ENUM_SERVICE_STATUSA").field("lpServiceName", &self.lpServiceName).field("lpDisplayName", &self.lpDisplayName).field("ServiceStatus", &self.ServiceStatus).finish()
    }
}
unsafe impl ::windows::core::Abi for ENUM_SERVICE_STATUSA {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for ENUM_SERVICE_STATUSA {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<ENUM_SERVICE_STATUSA>()) == 0 }
    }
}
impl ::core::cmp::Eq for ENUM_SERVICE_STATUSA {}
impl ::core::default::Default for ENUM_SERVICE_STATUSA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Services\"`*"]
pub struct ENUM_SERVICE_STATUSW {
    pub lpServiceName: ::windows::core::PWSTR,
    pub lpDisplayName: ::windows::core::PWSTR,
    pub ServiceStatus: SERVICE_STATUS,
}
impl ::core::marker::Copy for ENUM_SERVICE_STATUSW {}
impl ::core::clone::Clone for ENUM_SERVICE_STATUSW {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for ENUM_SERVICE_STATUSW {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("ENUM_SERVICE_STATUSW").field("lpServiceName", &self.lpServiceName).field("lpDisplayName", &self.lpDisplayName).field("ServiceStatus", &self.ServiceStatus).finish()
    }
}
unsafe impl ::windows::core::Abi for ENUM_SERVICE_STATUSW {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for ENUM_SERVICE_STATUSW {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<ENUM_SERVICE_STATUSW>()) == 0 }
    }
}
impl ::core::cmp::Eq for ENUM_SERVICE_STATUSW {}
impl ::core::default::Default for ENUM_SERVICE_STATUSW {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Services\"`*"]
pub struct ENUM_SERVICE_STATUS_PROCESSA {
    pub lpServiceName: ::windows::core::PSTR,
    pub lpDisplayName: ::windows::core::PSTR,
    pub ServiceStatusProcess: SERVICE_STATUS_PROCESS,
}
impl ::core::marker::Copy for ENUM_SERVICE_STATUS_PROCESSA {}
impl ::core::clone::Clone for ENUM_SERVICE_STATUS_PROCESSA {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for ENUM_SERVICE_STATUS_PROCESSA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("ENUM_SERVICE_STATUS_PROCESSA").field("lpServiceName", &self.lpServiceName).field("lpDisplayName", &self.lpDisplayName).field("ServiceStatusProcess", &self.ServiceStatusProcess).finish()
    }
}
unsafe impl ::windows::core::Abi for ENUM_SERVICE_STATUS_PROCESSA {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for ENUM_SERVICE_STATUS_PROCESSA {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<ENUM_SERVICE_STATUS_PROCESSA>()) == 0 }
    }
}
impl ::core::cmp::Eq for ENUM_SERVICE_STATUS_PROCESSA {}
impl ::core::default::Default for ENUM_SERVICE_STATUS_PROCESSA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Services\"`*"]
pub struct ENUM_SERVICE_STATUS_PROCESSW {
    pub lpServiceName: ::windows::core::PWSTR,
    pub lpDisplayName: ::windows::core::PWSTR,
    pub ServiceStatusProcess: SERVICE_STATUS_PROCESS,
}
impl ::core::marker::Copy for ENUM_SERVICE_STATUS_PROCESSW {}
impl ::core::clone::Clone for ENUM_SERVICE_STATUS_PROCESSW {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for ENUM_SERVICE_STATUS_PROCESSW {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("ENUM_SERVICE_STATUS_PROCESSW").field("lpServiceName", &self.lpServiceName).field("lpDisplayName", &self.lpDisplayName).field("ServiceStatusProcess", &self.ServiceStatusProcess).finish()
    }
}
unsafe impl ::windows::core::Abi for ENUM_SERVICE_STATUS_PROCESSW {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for ENUM_SERVICE_STATUS_PROCESSW {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<ENUM_SERVICE_STATUS_PROCESSW>()) == 0 }
    }
}
impl ::core::cmp::Eq for ENUM_SERVICE_STATUS_PROCESSW {}
impl ::core::default::Default for ENUM_SERVICE_STATUS_PROCESSW {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_System_Services\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct ENUM_SERVICE_TYPE(pub u32);
#[doc = "*Required features: `\"Win32_System_Services\"`*"]
pub const SERVICE_DRIVER: ENUM_SERVICE_TYPE = ENUM_SERVICE_TYPE(11u32);
#[doc = "*Required features: `\"Win32_System_Services\"`*"]
pub const SERVICE_KERNEL_DRIVER: ENUM_SERVICE_TYPE = ENUM_SERVICE_TYPE(1u32);
#[doc = "*Required features: `\"Win32_System_Services\"`*"]
pub const SERVICE_WIN32: ENUM_SERVICE_TYPE = ENUM_SERVICE_TYPE(48u32);
#[doc = "*Required features: `\"Win32_System_Services\"`*"]
pub const SERVICE_WIN32_SHARE_PROCESS: ENUM_SERVICE_TYPE = ENUM_SERVICE_TYPE(32u32);
#[doc = "*Required features: `\"Win32_System_Services\"`*"]
pub const SERVICE_ADAPTER: ENUM_SERVICE_TYPE = ENUM_SERVICE_TYPE(4u32);
#[doc = "*Required features: `\"Win32_System_Services\"`*"]
pub const SERVICE_FILE_SYSTEM_DRIVER: ENUM_SERVICE_TYPE = ENUM_SERVICE_TYPE(2u32);
#[doc = "*Required features: `\"Win32_System_Services\"`*"]
pub const SERVICE_RECOGNIZER_DRIVER: ENUM_SERVICE_TYPE = ENUM_SERVICE_TYPE(8u32);
#[doc = "*Required features: `\"Win32_System_Services\"`*"]
pub const SERVICE_WIN32_OWN_PROCESS: ENUM_SERVICE_TYPE = ENUM_SERVICE_TYPE(16u32);
#[doc = "*Required features: `\"Win32_System_Services\"`*"]
pub const SERVICE_USER_OWN_PROCESS: ENUM_SERVICE_TYPE = ENUM_SERVICE_TYPE(80u32);
#[doc = "*Required features: `\"Win32_System_Services\"`*"]
pub const SERVICE_USER_SHARE_PROCESS: ENUM_SERVICE_TYPE = ENUM_SERVICE_TYPE(96u32);
impl ::core::marker::Copy for ENUM_SERVICE_TYPE {}
impl ::core::clone::Clone for ENUM_SERVICE_TYPE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for ENUM_SERVICE_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for ENUM_SERVICE_TYPE {
    type Abi = Self;
}
impl ::core::fmt::Debug for ENUM_SERVICE_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ENUM_SERVICE_TYPE").field(&self.0).finish()
    }
}
impl ::core::ops::BitOr for ENUM_SERVICE_TYPE {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for ENUM_SERVICE_TYPE {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for ENUM_SERVICE_TYPE {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for ENUM_SERVICE_TYPE {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for ENUM_SERVICE_TYPE {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[doc = "*Required features: `\"Win32_System_Services\"`, `\"Win32_Foundation\"`, `\"Win32_Security\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
#[inline]
pub unsafe fn EnumDependentServicesA<'a, P0>(hservice: P0, dwservicestate: ENUM_SERVICE_STATE, lpservices: *mut ENUM_SERVICE_STATUSA, cbbufsize: u32, pcbbytesneeded: *mut u32, lpservicesreturned: *mut u32) -> super::super::Foundation::BOOL
where
    P0: ::std::convert::Into<super::super::Security::SC_HANDLE>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn EnumDependentServicesA(hservice: super::super::Security::SC_HANDLE, dwservicestate: ENUM_SERVICE_STATE, lpservices: *mut ENUM_SERVICE_STATUSA, cbbufsize: u32, pcbbytesneeded: *mut u32, lpservicesreturned: *mut u32) -> super::super::Foundation::BOOL;
    }
    EnumDependentServicesA(hservice.into(), dwservicestate, ::core::mem::transmute(lpservices), cbbufsize, ::core::mem::transmute(pcbbytesneeded), ::core::mem::transmute(lpservicesreturned))
}
#[doc = "*Required features: `\"Win32_System_Services\"`, `\"Win32_Foundation\"`, `\"Win32_Security\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
#[inline]
pub unsafe fn EnumDependentServicesW<'a, P0>(hservice: P0, dwservicestate: ENUM_SERVICE_STATE, lpservices: *mut ENUM_SERVICE_STATUSW, cbbufsize: u32, pcbbytesneeded: *mut u32, lpservicesreturned: *mut u32) -> super::super::Foundation::BOOL
where
    P0: ::std::convert::Into<super::super::Security::SC_HANDLE>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn EnumDependentServicesW(hservice: super::super::Security::SC_HANDLE, dwservicestate: ENUM_SERVICE_STATE, lpservices: *mut ENUM_SERVICE_STATUSW, cbbufsize: u32, pcbbytesneeded: *mut u32, lpservicesreturned: *mut u32) -> super::super::Foundation::BOOL;
    }
    EnumDependentServicesW(hservice.into(), dwservicestate, ::core::mem::transmute(lpservices), cbbufsize, ::core::mem::transmute(pcbbytesneeded), ::core::mem::transmute(lpservicesreturned))
}
#[doc = "*Required features: `\"Win32_System_Services\"`, `\"Win32_Foundation\"`, `\"Win32_Security\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
#[inline]
pub unsafe fn EnumServicesStatusA<'a, P0>(hscmanager: P0, dwservicetype: ENUM_SERVICE_TYPE, dwservicestate: ENUM_SERVICE_STATE, lpservices: *mut ENUM_SERVICE_STATUSA, cbbufsize: u32, pcbbytesneeded: *mut u32, lpservicesreturned: *mut u32, lpresumehandle: *mut u32) -> super::super::Foundation::BOOL
where
    P0: ::std::convert::Into<super::super::Security::SC_HANDLE>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn EnumServicesStatusA(hscmanager: super::super::Security::SC_HANDLE, dwservicetype: ENUM_SERVICE_TYPE, dwservicestate: ENUM_SERVICE_STATE, lpservices: *mut ENUM_SERVICE_STATUSA, cbbufsize: u32, pcbbytesneeded: *mut u32, lpservicesreturned: *mut u32, lpresumehandle: *mut u32) -> super::super::Foundation::BOOL;
    }
    EnumServicesStatusA(hscmanager.into(), dwservicetype, dwservicestate, ::core::mem::transmute(lpservices), cbbufsize, ::core::mem::transmute(pcbbytesneeded), ::core::mem::transmute(lpservicesreturned), ::core::mem::transmute(lpresumehandle))
}
#[doc = "*Required features: `\"Win32_System_Services\"`, `\"Win32_Foundation\"`, `\"Win32_Security\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
#[inline]
pub unsafe fn EnumServicesStatusExA<'a, P0, P1>(hscmanager: P0, infolevel: SC_ENUM_TYPE, dwservicetype: ENUM_SERVICE_TYPE, dwservicestate: ENUM_SERVICE_STATE, lpservices: *mut u8, cbbufsize: u32, pcbbytesneeded: *mut u32, lpservicesreturned: *mut u32, lpresumehandle: *mut u32, pszgroupname: P1) -> super::super::Foundation::BOOL
where
    P0: ::std::convert::Into<super::super::Security::SC_HANDLE>,
    P1: ::std::convert::Into<::windows::core::PCSTR>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn EnumServicesStatusExA(hscmanager: super::super::Security::SC_HANDLE, infolevel: SC_ENUM_TYPE, dwservicetype: ENUM_SERVICE_TYPE, dwservicestate: ENUM_SERVICE_STATE, lpservices: *mut u8, cbbufsize: u32, pcbbytesneeded: *mut u32, lpservicesreturned: *mut u32, lpresumehandle: *mut u32, pszgroupname: ::windows::core::PCSTR) -> super::super::Foundation::BOOL;
    }
    EnumServicesStatusExA(hscmanager.into(), infolevel, dwservicetype, dwservicestate, ::core::mem::transmute(lpservices), cbbufsize, ::core::mem::transmute(pcbbytesneeded), ::core::mem::transmute(lpservicesreturned), ::core::mem::transmute(lpresumehandle), pszgroupname.into())
}
#[doc = "*Required features: `\"Win32_System_Services\"`, `\"Win32_Foundation\"`, `\"Win32_Security\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
#[inline]
pub unsafe fn EnumServicesStatusExW<'a, P0, P1>(hscmanager: P0, infolevel: SC_ENUM_TYPE, dwservicetype: ENUM_SERVICE_TYPE, dwservicestate: ENUM_SERVICE_STATE, lpservices: *mut u8, cbbufsize: u32, pcbbytesneeded: *mut u32, lpservicesreturned: *mut u32, lpresumehandle: *mut u32, pszgroupname: P1) -> super::super::Foundation::BOOL
where
    P0: ::std::convert::Into<super::super::Security::SC_HANDLE>,
    P1: ::std::convert::Into<::windows::core::PCWSTR>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn EnumServicesStatusExW(hscmanager: super::super::Security::SC_HANDLE, infolevel: SC_ENUM_TYPE, dwservicetype: ENUM_SERVICE_TYPE, dwservicestate: ENUM_SERVICE_STATE, lpservices: *mut u8, cbbufsize: u32, pcbbytesneeded: *mut u32, lpservicesreturned: *mut u32, lpresumehandle: *mut u32, pszgroupname: ::windows::core::PCWSTR) -> super::super::Foundation::BOOL;
    }
    EnumServicesStatusExW(hscmanager.into(), infolevel, dwservicetype, dwservicestate, ::core::mem::transmute(lpservices), cbbufsize, ::core::mem::transmute(pcbbytesneeded), ::core::mem::transmute(lpservicesreturned), ::core::mem::transmute(lpresumehandle), pszgroupname.into())
}
#[doc = "*Required features: `\"Win32_System_Services\"`, `\"Win32_Foundation\"`, `\"Win32_Security\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
#[inline]
pub unsafe fn EnumServicesStatusW<'a, P0>(hscmanager: P0, dwservicetype: ENUM_SERVICE_TYPE, dwservicestate: ENUM_SERVICE_STATE, lpservices: *mut ENUM_SERVICE_STATUSW, cbbufsize: u32, pcbbytesneeded: *mut u32, lpservicesreturned: *mut u32, lpresumehandle: *mut u32) -> super::super::Foundation::BOOL
where
    P0: ::std::convert::Into<super::super::Security::SC_HANDLE>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn EnumServicesStatusW(hscmanager: super::super::Security::SC_HANDLE, dwservicetype: ENUM_SERVICE_TYPE, dwservicestate: ENUM_SERVICE_STATE, lpservices: *mut ENUM_SERVICE_STATUSW, cbbufsize: u32, pcbbytesneeded: *mut u32, lpservicesreturned: *mut u32, lpresumehandle: *mut u32) -> super::super::Foundation::BOOL;
    }
    EnumServicesStatusW(hscmanager.into(), dwservicetype, dwservicestate, ::core::mem::transmute(lpservices), cbbufsize, ::core::mem::transmute(pcbbytesneeded), ::core::mem::transmute(lpservicesreturned), ::core::mem::transmute(lpresumehandle))
}
pub const FIREWALL_PORT_CLOSE_GUID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xa144ed38_8e12_4de4_9d96_e64740b1a524);
pub const FIREWALL_PORT_OPEN_GUID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xb7569e07_8421_4ee0_ad10_86915afdad09);
#[doc = "*Required features: `\"Win32_System_Services\"`*"]
#[inline]
pub unsafe fn GetServiceDirectory<'a, P0>(hservicestatus: P0, edirectorytype: SERVICE_DIRECTORY_TYPE, lppathbuffer: &mut [u16], lpcchrequiredbufferlength: *mut u32) -> u32
where
    P0: ::std::convert::Into<SERVICE_STATUS_HANDLE>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn GetServiceDirectory(hservicestatus: SERVICE_STATUS_HANDLE, edirectorytype: SERVICE_DIRECTORY_TYPE, lppathbuffer: ::windows::core::PWSTR, cchpathbufferlength: u32, lpcchrequiredbufferlength: *mut u32) -> u32;
    }
    GetServiceDirectory(hservicestatus.into(), edirectorytype, ::core::mem::transmute(::windows::core::as_mut_ptr_or_null(lppathbuffer)), lppathbuffer.len() as _, ::core::mem::transmute(lpcchrequiredbufferlength))
}
#[doc = "*Required features: `\"Win32_System_Services\"`, `\"Win32_Foundation\"`, `\"Win32_Security\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
#[inline]
pub unsafe fn GetServiceDisplayNameA<'a, P0, P1>(hscmanager: P0, lpservicename: P1, lpdisplayname: ::windows::core::PSTR, lpcchbuffer: *mut u32) -> super::super::Foundation::BOOL
where
    P0: ::std::convert::Into<super::super::Security::SC_HANDLE>,
    P1: ::std::convert::Into<::windows::core::PCSTR>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn GetServiceDisplayNameA(hscmanager: super::super::Security::SC_HANDLE, lpservicename: ::windows::core::PCSTR, lpdisplayname: ::windows::core::PSTR, lpcchbuffer: *mut u32) -> super::super::Foundation::BOOL;
    }
    GetServiceDisplayNameA(hscmanager.into(), lpservicename.into(), ::core::mem::transmute(lpdisplayname), ::core::mem::transmute(lpcchbuffer))
}
#[doc = "*Required features: `\"Win32_System_Services\"`, `\"Win32_Foundation\"`, `\"Win32_Security\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
#[inline]
pub unsafe fn GetServiceDisplayNameW<'a, P0, P1>(hscmanager: P0, lpservicename: P1, lpdisplayname: ::windows::core::PWSTR, lpcchbuffer: *mut u32) -> super::super::Foundation::BOOL
where
    P0: ::std::convert::Into<super::super::Security::SC_HANDLE>,
    P1: ::std::convert::Into<::windows::core::PCWSTR>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn GetServiceDisplayNameW(hscmanager: super::super::Security::SC_HANDLE, lpservicename: ::windows::core::PCWSTR, lpdisplayname: ::windows::core::PWSTR, lpcchbuffer: *mut u32) -> super::super::Foundation::BOOL;
    }
    GetServiceDisplayNameW(hscmanager.into(), lpservicename.into(), ::core::mem::transmute(lpdisplayname), ::core::mem::transmute(lpcchbuffer))
}
#[doc = "*Required features: `\"Win32_System_Services\"`, `\"Win32_Foundation\"`, `\"Win32_Security\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
#[inline]
pub unsafe fn GetServiceKeyNameA<'a, P0, P1>(hscmanager: P0, lpdisplayname: P1, lpservicename: ::windows::core::PSTR, lpcchbuffer: *mut u32) -> super::super::Foundation::BOOL
where
    P0: ::std::convert::Into<super::super::Security::SC_HANDLE>,
    P1: ::std::convert::Into<::windows::core::PCSTR>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn GetServiceKeyNameA(hscmanager: super::super::Security::SC_HANDLE, lpdisplayname: ::windows::core::PCSTR, lpservicename: ::windows::core::PSTR, lpcchbuffer: *mut u32) -> super::super::Foundation::BOOL;
    }
    GetServiceKeyNameA(hscmanager.into(), lpdisplayname.into(), ::core::mem::transmute(lpservicename), ::core::mem::transmute(lpcchbuffer))
}
#[doc = "*Required features: `\"Win32_System_Services\"`, `\"Win32_Foundation\"`, `\"Win32_Security\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
#[inline]
pub unsafe fn GetServiceKeyNameW<'a, P0, P1>(hscmanager: P0, lpdisplayname: P1, lpservicename: ::windows::core::PWSTR, lpcchbuffer: *mut u32) -> super::super::Foundation::BOOL
where
    P0: ::std::convert::Into<super::super::Security::SC_HANDLE>,
    P1: ::std::convert::Into<::windows::core::PCWSTR>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn GetServiceKeyNameW(hscmanager: super::super::Security::SC_HANDLE, lpdisplayname: ::windows::core::PCWSTR, lpservicename: ::windows::core::PWSTR, lpcchbuffer: *mut u32) -> super::super::Foundation::BOOL;
    }
    GetServiceKeyNameW(hscmanager.into(), lpdisplayname.into(), ::core::mem::transmute(lpservicename), ::core::mem::transmute(lpcchbuffer))
}
#[doc = "*Required features: `\"Win32_System_Services\"`, `\"Win32_System_Registry\"`*"]
#[cfg(feature = "Win32_System_Registry")]
#[inline]
pub unsafe fn GetServiceRegistryStateKey<'a, P0>(servicestatushandle: P0, statetype: SERVICE_REGISTRY_STATE_TYPE, accessmask: u32, servicestatekey: *mut super::Registry::HKEY) -> u32
where
    P0: ::std::convert::Into<SERVICE_STATUS_HANDLE>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn GetServiceRegistryStateKey(servicestatushandle: SERVICE_STATUS_HANDLE, statetype: SERVICE_REGISTRY_STATE_TYPE, accessmask: u32, servicestatekey: *mut super::Registry::HKEY) -> u32;
    }
    GetServiceRegistryStateKey(servicestatushandle.into(), statetype, accessmask, ::core::mem::transmute(servicestatekey))
}
#[doc = "*Required features: `\"Win32_System_Services\"`, `\"Win32_Security\"`*"]
#[cfg(feature = "Win32_Security")]
#[inline]
pub unsafe fn GetSharedServiceDirectory<'a, P0>(servicehandle: P0, directorytype: SERVICE_SHARED_DIRECTORY_TYPE, pathbuffer: &mut [u16], requiredbufferlength: *mut u32) -> u32
where
    P0: ::std::convert::Into<super::super::Security::SC_HANDLE>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn GetSharedServiceDirectory(servicehandle: super::super::Security::SC_HANDLE, directorytype: SERVICE_SHARED_DIRECTORY_TYPE, pathbuffer: ::windows::core::PWSTR, pathbufferlength: u32, requiredbufferlength: *mut u32) -> u32;
    }
    GetSharedServiceDirectory(servicehandle.into(), directorytype, ::core::mem::transmute(::windows::core::as_mut_ptr_or_null(pathbuffer)), pathbuffer.len() as _, ::core::mem::transmute(requiredbufferlength))
}
#[doc = "*Required features: `\"Win32_System_Services\"`, `\"Win32_Security\"`, `\"Win32_System_Registry\"`*"]
#[cfg(all(feature = "Win32_Security", feature = "Win32_System_Registry"))]
#[inline]
pub unsafe fn GetSharedServiceRegistryStateKey<'a, P0>(servicehandle: P0, statetype: SERVICE_SHARED_REGISTRY_STATE_TYPE, accessmask: u32, servicestatekey: *mut super::Registry::HKEY) -> u32
where
    P0: ::std::convert::Into<super::super::Security::SC_HANDLE>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn GetSharedServiceRegistryStateKey(servicehandle: super::super::Security::SC_HANDLE, statetype: SERVICE_SHARED_REGISTRY_STATE_TYPE, accessmask: u32, servicestatekey: *mut super::Registry::HKEY) -> u32;
    }
    GetSharedServiceRegistryStateKey(servicehandle.into(), statetype, accessmask, ::core::mem::transmute(servicestatekey))
}
#[doc = "*Required features: `\"Win32_System_Services\"`*"]
pub type HANDLER_FUNCTION = ::core::option::Option<unsafe extern "system" fn(dwcontrol: u32)>;
#[doc = "*Required features: `\"Win32_System_Services\"`*"]
pub type HANDLER_FUNCTION_EX = ::core::option::Option<unsafe extern "system" fn(dwcontrol: u32, dweventtype: u32, lpeventdata: *mut ::core::ffi::c_void, lpcontext: *mut ::core::ffi::c_void) -> u32>;
#[doc = "*Required features: `\"Win32_System_Services\"`*"]
pub type LPHANDLER_FUNCTION = ::core::option::Option<unsafe extern "system" fn(dwcontrol: u32)>;
#[doc = "*Required features: `\"Win32_System_Services\"`*"]
pub type LPHANDLER_FUNCTION_EX = ::core::option::Option<unsafe extern "system" fn(dwcontrol: u32, dweventtype: u32, lpeventdata: *mut ::core::ffi::c_void, lpcontext: *mut ::core::ffi::c_void) -> u32>;
#[doc = "*Required features: `\"Win32_System_Services\"`*"]
pub type LPSERVICE_MAIN_FUNCTIONA = ::core::option::Option<unsafe extern "system" fn(dwnumservicesargs: u32, lpserviceargvectors: *mut ::windows::core::PSTR)>;
#[doc = "*Required features: `\"Win32_System_Services\"`*"]
pub type LPSERVICE_MAIN_FUNCTIONW = ::core::option::Option<unsafe extern "system" fn(dwnumservicesargs: u32, lpserviceargvectors: *mut ::windows::core::PWSTR)>;
#[doc = "*Required features: `\"Win32_System_Services\"`, `\"Win32_Security\"`*"]
#[cfg(feature = "Win32_Security")]
#[inline]
pub unsafe fn LockServiceDatabase<'a, P0>(hscmanager: P0) -> *mut ::core::ffi::c_void
where
    P0: ::std::convert::Into<super::super::Security::SC_HANDLE>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn LockServiceDatabase(hscmanager: super::super::Security::SC_HANDLE) -> *mut ::core::ffi::c_void;
    }
    LockServiceDatabase(hscmanager.into())
}
pub const MACHINE_POLICY_PRESENT_GUID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x659fcae6_5bdb_4da9_b1ff_ca2a178d46e0);
pub const NAMED_PIPE_EVENT_GUID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x1f81d131_3fac_4537_9e0c_7e7b0c2f4b55);
pub const NETWORK_MANAGER_FIRST_IP_ADDRESS_ARRIVAL_GUID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x4f27f2de_14e2_430b_a549_7cd48cbc8245);
pub const NETWORK_MANAGER_LAST_IP_ADDRESS_REMOVAL_GUID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xcc4ba62a_162e_4648_847a_b6bdf993e335);
#[doc = "*Required features: `\"Win32_System_Services\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn NotifyBootConfigStatus<'a, P0>(bootacceptable: P0) -> super::super::Foundation::BOOL
where
    P0: ::std::convert::Into<super::super::Foundation::BOOL>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn NotifyBootConfigStatus(bootacceptable: super::super::Foundation::BOOL) -> super::super::Foundation::BOOL;
    }
    NotifyBootConfigStatus(bootacceptable.into())
}
#[doc = "*Required features: `\"Win32_System_Services\"`, `\"Win32_Security\"`*"]
#[cfg(feature = "Win32_Security")]
#[inline]
pub unsafe fn NotifyServiceStatusChangeA<'a, P0>(hservice: P0, dwnotifymask: SERVICE_NOTIFY, pnotifybuffer: *const SERVICE_NOTIFY_2A) -> u32
where
    P0: ::std::convert::Into<super::super::Security::SC_HANDLE>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn NotifyServiceStatusChangeA(hservice: super::super::Security::SC_HANDLE, dwnotifymask: SERVICE_NOTIFY, pnotifybuffer: *const SERVICE_NOTIFY_2A) -> u32;
    }
    NotifyServiceStatusChangeA(hservice.into(), dwnotifymask, ::core::mem::transmute(pnotifybuffer))
}
#[doc = "*Required features: `\"Win32_System_Services\"`, `\"Win32_Security\"`*"]
#[cfg(feature = "Win32_Security")]
#[inline]
pub unsafe fn NotifyServiceStatusChangeW<'a, P0>(hservice: P0, dwnotifymask: SERVICE_NOTIFY, pnotifybuffer: *const SERVICE_NOTIFY_2W) -> u32
where
    P0: ::std::convert::Into<super::super::Security::SC_HANDLE>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn NotifyServiceStatusChangeW(hservice: super::super::Security::SC_HANDLE, dwnotifymask: SERVICE_NOTIFY, pnotifybuffer: *const SERVICE_NOTIFY_2W) -> u32;
    }
    NotifyServiceStatusChangeW(hservice.into(), dwnotifymask, ::core::mem::transmute(pnotifybuffer))
}
#[doc = "*Required features: `\"Win32_System_Services\"`, `\"Win32_Security\"`*"]
#[cfg(feature = "Win32_Security")]
#[inline]
pub unsafe fn OpenSCManagerA<'a, P0, P1>(lpmachinename: P0, lpdatabasename: P1, dwdesiredaccess: u32) -> ::windows::core::Result<super::super::Security::SC_HANDLE>
where
    P0: ::std::convert::Into<::windows::core::PCSTR>,
    P1: ::std::convert::Into<::windows::core::PCSTR>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn OpenSCManagerA(lpmachinename: ::windows::core::PCSTR, lpdatabasename: ::windows::core::PCSTR, dwdesiredaccess: u32) -> super::super::Security::SC_HANDLE;
    }
    let result__ = OpenSCManagerA(lpmachinename.into(), lpdatabasename.into(), dwdesiredaccess);
    (!result__.is_invalid()).then(|| result__).ok_or_else(::windows::core::Error::from_win32)
}
#[doc = "*Required features: `\"Win32_System_Services\"`, `\"Win32_Security\"`*"]
#[cfg(feature = "Win32_Security")]
#[inline]
pub unsafe fn OpenSCManagerW<'a, P0, P1>(lpmachinename: P0, lpdatabasename: P1, dwdesiredaccess: u32) -> ::windows::core::Result<super::super::Security::SC_HANDLE>
where
    P0: ::std::convert::Into<::windows::core::PCWSTR>,
    P1: ::std::convert::Into<::windows::core::PCWSTR>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn OpenSCManagerW(lpmachinename: ::windows::core::PCWSTR, lpdatabasename: ::windows::core::PCWSTR, dwdesiredaccess: u32) -> super::super::Security::SC_HANDLE;
    }
    let result__ = OpenSCManagerW(lpmachinename.into(), lpdatabasename.into(), dwdesiredaccess);
    (!result__.is_invalid()).then(|| result__).ok_or_else(::windows::core::Error::from_win32)
}
#[doc = "*Required features: `\"Win32_System_Services\"`, `\"Win32_Security\"`*"]
#[cfg(feature = "Win32_Security")]
#[inline]
pub unsafe fn OpenServiceA<'a, P0, P1>(hscmanager: P0, lpservicename: P1, dwdesiredaccess: u32) -> ::windows::core::Result<super::super::Security::SC_HANDLE>
where
    P0: ::std::convert::Into<super::super::Security::SC_HANDLE>,
    P1: ::std::convert::Into<::windows::core::PCSTR>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn OpenServiceA(hscmanager: super::super::Security::SC_HANDLE, lpservicename: ::windows::core::PCSTR, dwdesiredaccess: u32) -> super::super::Security::SC_HANDLE;
    }
    let result__ = OpenServiceA(hscmanager.into(), lpservicename.into(), dwdesiredaccess);
    (!result__.is_invalid()).then(|| result__).ok_or_else(::windows::core::Error::from_win32)
}
#[doc = "*Required features: `\"Win32_System_Services\"`, `\"Win32_Security\"`*"]
#[cfg(feature = "Win32_Security")]
#[inline]
pub unsafe fn OpenServiceW<'a, P0, P1>(hscmanager: P0, lpservicename: P1, dwdesiredaccess: u32) -> ::windows::core::Result<super::super::Security::SC_HANDLE>
where
    P0: ::std::convert::Into<super::super::Security::SC_HANDLE>,
    P1: ::std::convert::Into<::windows::core::PCWSTR>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn OpenServiceW(hscmanager: super::super::Security::SC_HANDLE, lpservicename: ::windows::core::PCWSTR, dwdesiredaccess: u32) -> super::super::Security::SC_HANDLE;
    }
    let result__ = OpenServiceW(hscmanager.into(), lpservicename.into(), dwdesiredaccess);
    (!result__.is_invalid()).then(|| result__).ok_or_else(::windows::core::Error::from_win32)
}
#[doc = "*Required features: `\"Win32_System_Services\"`*"]
pub type PFN_SC_NOTIFY_CALLBACK = ::core::option::Option<unsafe extern "system" fn(pparameter: *const ::core::ffi::c_void)>;
#[doc = "*Required features: `\"Win32_System_Services\"`*"]
pub type PSC_NOTIFICATION_CALLBACK = ::core::option::Option<unsafe extern "system" fn(dwnotify: u32, pcallbackcontext: *const ::core::ffi::c_void)>;
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Services\"`*"]
pub struct QUERY_SERVICE_CONFIGA {
    pub dwServiceType: ENUM_SERVICE_TYPE,
    pub dwStartType: SERVICE_START_TYPE,
    pub dwErrorControl: SERVICE_ERROR,
    pub lpBinaryPathName: ::windows::core::PSTR,
    pub lpLoadOrderGroup: ::windows::core::PSTR,
    pub dwTagId: u32,
    pub lpDependencies: ::windows::core::PSTR,
    pub lpServiceStartName: ::windows::core::PSTR,
    pub lpDisplayName: ::windows::core::PSTR,
}
impl ::core::marker::Copy for QUERY_SERVICE_CONFIGA {}
impl ::core::clone::Clone for QUERY_SERVICE_CONFIGA {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for QUERY_SERVICE_CONFIGA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("QUERY_SERVICE_CONFIGA").field("dwServiceType", &self.dwServiceType).field("dwStartType", &self.dwStartType).field("dwErrorControl", &self.dwErrorControl).field("lpBinaryPathName", &self.lpBinaryPathName).field("lpLoadOrderGroup", &self.lpLoadOrderGroup).field("dwTagId", &self.dwTagId).field("lpDependencies", &self.lpDependencies).field("lpServiceStartName", &self.lpServiceStartName).field("lpDisplayName", &self.lpDisplayName).finish()
    }
}
unsafe impl ::windows::core::Abi for QUERY_SERVICE_CONFIGA {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for QUERY_SERVICE_CONFIGA {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<QUERY_SERVICE_CONFIGA>()) == 0 }
    }
}
impl ::core::cmp::Eq for QUERY_SERVICE_CONFIGA {}
impl ::core::default::Default for QUERY_SERVICE_CONFIGA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Services\"`*"]
pub struct QUERY_SERVICE_CONFIGW {
    pub dwServiceType: ENUM_SERVICE_TYPE,
    pub dwStartType: SERVICE_START_TYPE,
    pub dwErrorControl: SERVICE_ERROR,
    pub lpBinaryPathName: ::windows::core::PWSTR,
    pub lpLoadOrderGroup: ::windows::core::PWSTR,
    pub dwTagId: u32,
    pub lpDependencies: ::windows::core::PWSTR,
    pub lpServiceStartName: ::windows::core::PWSTR,
    pub lpDisplayName: ::windows::core::PWSTR,
}
impl ::core::marker::Copy for QUERY_SERVICE_CONFIGW {}
impl ::core::clone::Clone for QUERY_SERVICE_CONFIGW {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for QUERY_SERVICE_CONFIGW {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("QUERY_SERVICE_CONFIGW").field("dwServiceType", &self.dwServiceType).field("dwStartType", &self.dwStartType).field("dwErrorControl", &self.dwErrorControl).field("lpBinaryPathName", &self.lpBinaryPathName).field("lpLoadOrderGroup", &self.lpLoadOrderGroup).field("dwTagId", &self.dwTagId).field("lpDependencies", &self.lpDependencies).field("lpServiceStartName", &self.lpServiceStartName).field("lpDisplayName", &self.lpDisplayName).finish()
    }
}
unsafe impl ::windows::core::Abi for QUERY_SERVICE_CONFIGW {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for QUERY_SERVICE_CONFIGW {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<QUERY_SERVICE_CONFIGW>()) == 0 }
    }
}
impl ::core::cmp::Eq for QUERY_SERVICE_CONFIGW {}
impl ::core::default::Default for QUERY_SERVICE_CONFIGW {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Services\"`*"]
pub struct QUERY_SERVICE_LOCK_STATUSA {
    pub fIsLocked: u32,
    pub lpLockOwner: ::windows::core::PSTR,
    pub dwLockDuration: u32,
}
impl ::core::marker::Copy for QUERY_SERVICE_LOCK_STATUSA {}
impl ::core::clone::Clone for QUERY_SERVICE_LOCK_STATUSA {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for QUERY_SERVICE_LOCK_STATUSA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("QUERY_SERVICE_LOCK_STATUSA").field("fIsLocked", &self.fIsLocked).field("lpLockOwner", &self.lpLockOwner).field("dwLockDuration", &self.dwLockDuration).finish()
    }
}
unsafe impl ::windows::core::Abi for QUERY_SERVICE_LOCK_STATUSA {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for QUERY_SERVICE_LOCK_STATUSA {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<QUERY_SERVICE_LOCK_STATUSA>()) == 0 }
    }
}
impl ::core::cmp::Eq for QUERY_SERVICE_LOCK_STATUSA {}
impl ::core::default::Default for QUERY_SERVICE_LOCK_STATUSA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Services\"`*"]
pub struct QUERY_SERVICE_LOCK_STATUSW {
    pub fIsLocked: u32,
    pub lpLockOwner: ::windows::core::PWSTR,
    pub dwLockDuration: u32,
}
impl ::core::marker::Copy for QUERY_SERVICE_LOCK_STATUSW {}
impl ::core::clone::Clone for QUERY_SERVICE_LOCK_STATUSW {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for QUERY_SERVICE_LOCK_STATUSW {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("QUERY_SERVICE_LOCK_STATUSW").field("fIsLocked", &self.fIsLocked).field("lpLockOwner", &self.lpLockOwner).field("dwLockDuration", &self.dwLockDuration).finish()
    }
}
unsafe impl ::windows::core::Abi for QUERY_SERVICE_LOCK_STATUSW {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for QUERY_SERVICE_LOCK_STATUSW {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<QUERY_SERVICE_LOCK_STATUSW>()) == 0 }
    }
}
impl ::core::cmp::Eq for QUERY_SERVICE_LOCK_STATUSW {}
impl ::core::default::Default for QUERY_SERVICE_LOCK_STATUSW {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_System_Services\"`, `\"Win32_Foundation\"`, `\"Win32_Security\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
#[inline]
pub unsafe fn QueryServiceConfig2A<'a, P0>(hservice: P0, dwinfolevel: SERVICE_CONFIG, lpbuffer: *mut u8, cbbufsize: u32, pcbbytesneeded: *mut u32) -> super::super::Foundation::BOOL
where
    P0: ::std::convert::Into<super::super::Security::SC_HANDLE>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn QueryServiceConfig2A(hservice: super::super::Security::SC_HANDLE, dwinfolevel: SERVICE_CONFIG, lpbuffer: *mut u8, cbbufsize: u32, pcbbytesneeded: *mut u32) -> super::super::Foundation::BOOL;
    }
    QueryServiceConfig2A(hservice.into(), dwinfolevel, ::core::mem::transmute(lpbuffer), cbbufsize, ::core::mem::transmute(pcbbytesneeded))
}
#[doc = "*Required features: `\"Win32_System_Services\"`, `\"Win32_Foundation\"`, `\"Win32_Security\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
#[inline]
pub unsafe fn QueryServiceConfig2W<'a, P0>(hservice: P0, dwinfolevel: SERVICE_CONFIG, lpbuffer: *mut u8, cbbufsize: u32, pcbbytesneeded: *mut u32) -> super::super::Foundation::BOOL
where
    P0: ::std::convert::Into<super::super::Security::SC_HANDLE>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn QueryServiceConfig2W(hservice: super::super::Security::SC_HANDLE, dwinfolevel: SERVICE_CONFIG, lpbuffer: *mut u8, cbbufsize: u32, pcbbytesneeded: *mut u32) -> super::super::Foundation::BOOL;
    }
    QueryServiceConfig2W(hservice.into(), dwinfolevel, ::core::mem::transmute(lpbuffer), cbbufsize, ::core::mem::transmute(pcbbytesneeded))
}
#[doc = "*Required features: `\"Win32_System_Services\"`, `\"Win32_Foundation\"`, `\"Win32_Security\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
#[inline]
pub unsafe fn QueryServiceConfigA<'a, P0>(hservice: P0, lpserviceconfig: *mut QUERY_SERVICE_CONFIGA, cbbufsize: u32, pcbbytesneeded: *mut u32) -> super::super::Foundation::BOOL
where
    P0: ::std::convert::Into<super::super::Security::SC_HANDLE>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn QueryServiceConfigA(hservice: super::super::Security::SC_HANDLE, lpserviceconfig: *mut QUERY_SERVICE_CONFIGA, cbbufsize: u32, pcbbytesneeded: *mut u32) -> super::super::Foundation::BOOL;
    }
    QueryServiceConfigA(hservice.into(), ::core::mem::transmute(lpserviceconfig), cbbufsize, ::core::mem::transmute(pcbbytesneeded))
}
#[doc = "*Required features: `\"Win32_System_Services\"`, `\"Win32_Foundation\"`, `\"Win32_Security\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
#[inline]
pub unsafe fn QueryServiceConfigW<'a, P0>(hservice: P0, lpserviceconfig: *mut QUERY_SERVICE_CONFIGW, cbbufsize: u32, pcbbytesneeded: *mut u32) -> super::super::Foundation::BOOL
where
    P0: ::std::convert::Into<super::super::Security::SC_HANDLE>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn QueryServiceConfigW(hservice: super::super::Security::SC_HANDLE, lpserviceconfig: *mut QUERY_SERVICE_CONFIGW, cbbufsize: u32, pcbbytesneeded: *mut u32) -> super::super::Foundation::BOOL;
    }
    QueryServiceConfigW(hservice.into(), ::core::mem::transmute(lpserviceconfig), cbbufsize, ::core::mem::transmute(pcbbytesneeded))
}
#[doc = "*Required features: `\"Win32_System_Services\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn QueryServiceDynamicInformation<'a, P0>(hservicestatus: P0, dwinfolevel: u32, ppdynamicinfo: *mut *mut ::core::ffi::c_void) -> super::super::Foundation::BOOL
where
    P0: ::std::convert::Into<SERVICE_STATUS_HANDLE>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn QueryServiceDynamicInformation(hservicestatus: SERVICE_STATUS_HANDLE, dwinfolevel: u32, ppdynamicinfo: *mut *mut ::core::ffi::c_void) -> super::super::Foundation::BOOL;
    }
    QueryServiceDynamicInformation(hservicestatus.into(), dwinfolevel, ::core::mem::transmute(ppdynamicinfo))
}
#[doc = "*Required features: `\"Win32_System_Services\"`, `\"Win32_Foundation\"`, `\"Win32_Security\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
#[inline]
pub unsafe fn QueryServiceLockStatusA<'a, P0>(hscmanager: P0, lplockstatus: *mut QUERY_SERVICE_LOCK_STATUSA, cbbufsize: u32, pcbbytesneeded: *mut u32) -> super::super::Foundation::BOOL
where
    P0: ::std::convert::Into<super::super::Security::SC_HANDLE>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn QueryServiceLockStatusA(hscmanager: super::super::Security::SC_HANDLE, lplockstatus: *mut QUERY_SERVICE_LOCK_STATUSA, cbbufsize: u32, pcbbytesneeded: *mut u32) -> super::super::Foundation::BOOL;
    }
    QueryServiceLockStatusA(hscmanager.into(), ::core::mem::transmute(lplockstatus), cbbufsize, ::core::mem::transmute(pcbbytesneeded))
}
#[doc = "*Required features: `\"Win32_System_Services\"`, `\"Win32_Foundation\"`, `\"Win32_Security\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
#[inline]
pub unsafe fn QueryServiceLockStatusW<'a, P0>(hscmanager: P0, lplockstatus: *mut QUERY_SERVICE_LOCK_STATUSW, cbbufsize: u32, pcbbytesneeded: *mut u32) -> super::super::Foundation::BOOL
where
    P0: ::std::convert::Into<super::super::Security::SC_HANDLE>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn QueryServiceLockStatusW(hscmanager: super::super::Security::SC_HANDLE, lplockstatus: *mut QUERY_SERVICE_LOCK_STATUSW, cbbufsize: u32, pcbbytesneeded: *mut u32) -> super::super::Foundation::BOOL;
    }
    QueryServiceLockStatusW(hscmanager.into(), ::core::mem::transmute(lplockstatus), cbbufsize, ::core::mem::transmute(pcbbytesneeded))
}
#[doc = "*Required features: `\"Win32_System_Services\"`, `\"Win32_Foundation\"`, `\"Win32_Security\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
#[inline]
pub unsafe fn QueryServiceObjectSecurity<'a, P0>(hservice: P0, dwsecurityinformation: u32, lpsecuritydescriptor: super::super::Security::PSECURITY_DESCRIPTOR, cbbufsize: u32, pcbbytesneeded: *mut u32) -> super::super::Foundation::BOOL
where
    P0: ::std::convert::Into<super::super::Security::SC_HANDLE>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn QueryServiceObjectSecurity(hservice: super::super::Security::SC_HANDLE, dwsecurityinformation: u32, lpsecuritydescriptor: super::super::Security::PSECURITY_DESCRIPTOR, cbbufsize: u32, pcbbytesneeded: *mut u32) -> super::super::Foundation::BOOL;
    }
    QueryServiceObjectSecurity(hservice.into(), dwsecurityinformation, ::core::mem::transmute(lpsecuritydescriptor), cbbufsize, ::core::mem::transmute(pcbbytesneeded))
}
#[doc = "*Required features: `\"Win32_System_Services\"`, `\"Win32_Foundation\"`, `\"Win32_Security\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
#[inline]
pub unsafe fn QueryServiceStatus<'a, P0>(hservice: P0, lpservicestatus: *mut SERVICE_STATUS) -> super::super::Foundation::BOOL
where
    P0: ::std::convert::Into<super::super::Security::SC_HANDLE>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn QueryServiceStatus(hservice: super::super::Security::SC_HANDLE, lpservicestatus: *mut SERVICE_STATUS) -> super::super::Foundation::BOOL;
    }
    QueryServiceStatus(hservice.into(), ::core::mem::transmute(lpservicestatus))
}
#[doc = "*Required features: `\"Win32_System_Services\"`, `\"Win32_Foundation\"`, `\"Win32_Security\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
#[inline]
pub unsafe fn QueryServiceStatusEx<'a, P0>(hservice: P0, infolevel: SC_STATUS_TYPE, lpbuffer: *mut u8, cbbufsize: u32, pcbbytesneeded: *mut u32) -> super::super::Foundation::BOOL
where
    P0: ::std::convert::Into<super::super::Security::SC_HANDLE>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn QueryServiceStatusEx(hservice: super::super::Security::SC_HANDLE, infolevel: SC_STATUS_TYPE, lpbuffer: *mut u8, cbbufsize: u32, pcbbytesneeded: *mut u32) -> super::super::Foundation::BOOL;
    }
    QueryServiceStatusEx(hservice.into(), infolevel, ::core::mem::transmute(lpbuffer), cbbufsize, ::core::mem::transmute(pcbbytesneeded))
}
pub const RPC_INTERFACE_EVENT_GUID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xbc90d167_9470_4139_a9ba_be0bbbf5b74d);
#[doc = "*Required features: `\"Win32_System_Services\"`*"]
#[inline]
pub unsafe fn RegisterServiceCtrlHandlerA<'a, P0>(lpservicename: P0, lphandlerproc: LPHANDLER_FUNCTION) -> ::windows::core::Result<SERVICE_STATUS_HANDLE>
where
    P0: ::std::convert::Into<::windows::core::PCSTR>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn RegisterServiceCtrlHandlerA(lpservicename: ::windows::core::PCSTR, lphandlerproc: *mut ::core::ffi::c_void) -> SERVICE_STATUS_HANDLE;
    }
    let result__ = RegisterServiceCtrlHandlerA(lpservicename.into(), ::core::mem::transmute(lphandlerproc));
    (!result__.is_invalid()).then(|| result__).ok_or_else(::windows::core::Error::from_win32)
}
#[doc = "*Required features: `\"Win32_System_Services\"`*"]
#[inline]
pub unsafe fn RegisterServiceCtrlHandlerExA<'a, P0>(lpservicename: P0, lphandlerproc: LPHANDLER_FUNCTION_EX, lpcontext: *const ::core::ffi::c_void) -> ::windows::core::Result<SERVICE_STATUS_HANDLE>
where
    P0: ::std::convert::Into<::windows::core::PCSTR>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn RegisterServiceCtrlHandlerExA(lpservicename: ::windows::core::PCSTR, lphandlerproc: *mut ::core::ffi::c_void, lpcontext: *const ::core::ffi::c_void) -> SERVICE_STATUS_HANDLE;
    }
    let result__ = RegisterServiceCtrlHandlerExA(lpservicename.into(), ::core::mem::transmute(lphandlerproc), ::core::mem::transmute(lpcontext));
    (!result__.is_invalid()).then(|| result__).ok_or_else(::windows::core::Error::from_win32)
}
#[doc = "*Required features: `\"Win32_System_Services\"`*"]
#[inline]
pub unsafe fn RegisterServiceCtrlHandlerExW<'a, P0>(lpservicename: P0, lphandlerproc: LPHANDLER_FUNCTION_EX, lpcontext: *const ::core::ffi::c_void) -> ::windows::core::Result<SERVICE_STATUS_HANDLE>
where
    P0: ::std::convert::Into<::windows::core::PCWSTR>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn RegisterServiceCtrlHandlerExW(lpservicename: ::windows::core::PCWSTR, lphandlerproc: *mut ::core::ffi::c_void, lpcontext: *const ::core::ffi::c_void) -> SERVICE_STATUS_HANDLE;
    }
    let result__ = RegisterServiceCtrlHandlerExW(lpservicename.into(), ::core::mem::transmute(lphandlerproc), ::core::mem::transmute(lpcontext));
    (!result__.is_invalid()).then(|| result__).ok_or_else(::windows::core::Error::from_win32)
}
#[doc = "*Required features: `\"Win32_System_Services\"`*"]
#[inline]
pub unsafe fn RegisterServiceCtrlHandlerW<'a, P0>(lpservicename: P0, lphandlerproc: LPHANDLER_FUNCTION) -> ::windows::core::Result<SERVICE_STATUS_HANDLE>
where
    P0: ::std::convert::Into<::windows::core::PCWSTR>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn RegisterServiceCtrlHandlerW(lpservicename: ::windows::core::PCWSTR, lphandlerproc: *mut ::core::ffi::c_void) -> SERVICE_STATUS_HANDLE;
    }
    let result__ = RegisterServiceCtrlHandlerW(lpservicename.into(), ::core::mem::transmute(lphandlerproc));
    (!result__.is_invalid()).then(|| result__).ok_or_else(::windows::core::Error::from_win32)
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Services\"`*"]
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
#[doc = "*Required features: `\"Win32_System_Services\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct SC_ACTION_TYPE(pub i32);
#[doc = "*Required features: `\"Win32_System_Services\"`*"]
pub const SC_ACTION_NONE: SC_ACTION_TYPE = SC_ACTION_TYPE(0i32);
#[doc = "*Required features: `\"Win32_System_Services\"`*"]
pub const SC_ACTION_RESTART: SC_ACTION_TYPE = SC_ACTION_TYPE(1i32);
#[doc = "*Required features: `\"Win32_System_Services\"`*"]
pub const SC_ACTION_REBOOT: SC_ACTION_TYPE = SC_ACTION_TYPE(2i32);
#[doc = "*Required features: `\"Win32_System_Services\"`*"]
pub const SC_ACTION_RUN_COMMAND: SC_ACTION_TYPE = SC_ACTION_TYPE(3i32);
#[doc = "*Required features: `\"Win32_System_Services\"`*"]
pub const SC_ACTION_OWN_RESTART: SC_ACTION_TYPE = SC_ACTION_TYPE(4i32);
impl ::core::marker::Copy for SC_ACTION_TYPE {}
impl ::core::clone::Clone for SC_ACTION_TYPE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for SC_ACTION_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for SC_ACTION_TYPE {
    type Abi = Self;
}
impl ::core::fmt::Debug for SC_ACTION_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SC_ACTION_TYPE").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_System_Services\"`*"]
pub const SC_AGGREGATE_STORAGE_KEY: &str = "System\\CurrentControlSet\\Control\\ServiceAggregatedEvents";
#[doc = "*Required features: `\"Win32_System_Services\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct SC_ENUM_TYPE(pub i32);
#[doc = "*Required features: `\"Win32_System_Services\"`*"]
pub const SC_ENUM_PROCESS_INFO: SC_ENUM_TYPE = SC_ENUM_TYPE(0i32);
impl ::core::marker::Copy for SC_ENUM_TYPE {}
impl ::core::clone::Clone for SC_ENUM_TYPE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for SC_ENUM_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for SC_ENUM_TYPE {
    type Abi = Self;
}
impl ::core::fmt::Debug for SC_ENUM_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SC_ENUM_TYPE").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_System_Services\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct SC_EVENT_TYPE(pub i32);
#[doc = "*Required features: `\"Win32_System_Services\"`*"]
pub const SC_EVENT_DATABASE_CHANGE: SC_EVENT_TYPE = SC_EVENT_TYPE(0i32);
#[doc = "*Required features: `\"Win32_System_Services\"`*"]
pub const SC_EVENT_PROPERTY_CHANGE: SC_EVENT_TYPE = SC_EVENT_TYPE(1i32);
#[doc = "*Required features: `\"Win32_System_Services\"`*"]
pub const SC_EVENT_STATUS_CHANGE: SC_EVENT_TYPE = SC_EVENT_TYPE(2i32);
impl ::core::marker::Copy for SC_EVENT_TYPE {}
impl ::core::clone::Clone for SC_EVENT_TYPE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for SC_EVENT_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for SC_EVENT_TYPE {
    type Abi = Self;
}
impl ::core::fmt::Debug for SC_EVENT_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SC_EVENT_TYPE").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_System_Services\"`*"]
pub const SC_MANAGER_ALL_ACCESS: u32 = 983103u32;
#[doc = "*Required features: `\"Win32_System_Services\"`*"]
pub const SC_MANAGER_CONNECT: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_Services\"`*"]
pub const SC_MANAGER_CREATE_SERVICE: u32 = 2u32;
#[doc = "*Required features: `\"Win32_System_Services\"`*"]
pub const SC_MANAGER_ENUMERATE_SERVICE: u32 = 4u32;
#[doc = "*Required features: `\"Win32_System_Services\"`*"]
pub const SC_MANAGER_LOCK: u32 = 8u32;
#[doc = "*Required features: `\"Win32_System_Services\"`*"]
pub const SC_MANAGER_MODIFY_BOOT_CONFIG: u32 = 32u32;
#[doc = "*Required features: `\"Win32_System_Services\"`*"]
pub const SC_MANAGER_QUERY_LOCK_STATUS: u32 = 16u32;
#[doc = "*Required features: `\"Win32_System_Services\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct SC_STATUS_TYPE(pub i32);
#[doc = "*Required features: `\"Win32_System_Services\"`*"]
pub const SC_STATUS_PROCESS_INFO: SC_STATUS_TYPE = SC_STATUS_TYPE(0i32);
impl ::core::marker::Copy for SC_STATUS_TYPE {}
impl ::core::clone::Clone for SC_STATUS_TYPE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for SC_STATUS_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for SC_STATUS_TYPE {
    type Abi = Self;
}
impl ::core::fmt::Debug for SC_STATUS_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SC_STATUS_TYPE").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_System_Services\"`*"]
pub const SERVICES_ACTIVE_DATABASE: &str = "ServicesActive";
#[doc = "*Required features: `\"Win32_System_Services\"`*"]
pub const SERVICES_ACTIVE_DATABASEA: &str = "ServicesActive";
#[doc = "*Required features: `\"Win32_System_Services\"`*"]
pub const SERVICES_ACTIVE_DATABASEW: &str = "ServicesActive";
#[doc = "*Required features: `\"Win32_System_Services\"`*"]
pub const SERVICES_FAILED_DATABASE: &str = "ServicesFailed";
#[doc = "*Required features: `\"Win32_System_Services\"`*"]
pub const SERVICES_FAILED_DATABASEA: &str = "ServicesFailed";
#[doc = "*Required features: `\"Win32_System_Services\"`*"]
pub const SERVICES_FAILED_DATABASEW: &str = "ServicesFailed";
#[doc = "*Required features: `\"Win32_System_Services\"`*"]
pub const SERVICE_ACCEPT_HARDWAREPROFILECHANGE: u32 = 32u32;
#[doc = "*Required features: `\"Win32_System_Services\"`*"]
pub const SERVICE_ACCEPT_LOWRESOURCES: u32 = 8192u32;
#[doc = "*Required features: `\"Win32_System_Services\"`*"]
pub const SERVICE_ACCEPT_NETBINDCHANGE: u32 = 16u32;
#[doc = "*Required features: `\"Win32_System_Services\"`*"]
pub const SERVICE_ACCEPT_PARAMCHANGE: u32 = 8u32;
#[doc = "*Required features: `\"Win32_System_Services\"`*"]
pub const SERVICE_ACCEPT_PAUSE_CONTINUE: u32 = 2u32;
#[doc = "*Required features: `\"Win32_System_Services\"`*"]
pub const SERVICE_ACCEPT_POWEREVENT: u32 = 64u32;
#[doc = "*Required features: `\"Win32_System_Services\"`*"]
pub const SERVICE_ACCEPT_PRESHUTDOWN: u32 = 256u32;
#[doc = "*Required features: `\"Win32_System_Services\"`*"]
pub const SERVICE_ACCEPT_SESSIONCHANGE: u32 = 128u32;
#[doc = "*Required features: `\"Win32_System_Services\"`*"]
pub const SERVICE_ACCEPT_SHUTDOWN: u32 = 4u32;
#[doc = "*Required features: `\"Win32_System_Services\"`*"]
pub const SERVICE_ACCEPT_STOP: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_Services\"`*"]
pub const SERVICE_ACCEPT_SYSTEMLOWRESOURCES: u32 = 16384u32;
#[doc = "*Required features: `\"Win32_System_Services\"`*"]
pub const SERVICE_ACCEPT_TIMECHANGE: u32 = 512u32;
#[doc = "*Required features: `\"Win32_System_Services\"`*"]
pub const SERVICE_ACCEPT_TRIGGEREVENT: u32 = 1024u32;
#[doc = "*Required features: `\"Win32_System_Services\"`*"]
pub const SERVICE_ACCEPT_USER_LOGOFF: u32 = 2048u32;
#[doc = "*Required features: `\"Win32_System_Services\"`*"]
pub const SERVICE_ALL_ACCESS: u32 = 983551u32;
#[doc = "*Required features: `\"Win32_System_Services\"`*"]
pub const SERVICE_CHANGE_CONFIG: u32 = 2u32;
#[doc = "*Required features: `\"Win32_System_Services\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct SERVICE_CONFIG(pub u32);
#[doc = "*Required features: `\"Win32_System_Services\"`*"]
pub const SERVICE_CONFIG_DELAYED_AUTO_START_INFO: SERVICE_CONFIG = SERVICE_CONFIG(3u32);
#[doc = "*Required features: `\"Win32_System_Services\"`*"]
pub const SERVICE_CONFIG_DESCRIPTION: SERVICE_CONFIG = SERVICE_CONFIG(1u32);
#[doc = "*Required features: `\"Win32_System_Services\"`*"]
pub const SERVICE_CONFIG_FAILURE_ACTIONS: SERVICE_CONFIG = SERVICE_CONFIG(2u32);
#[doc = "*Required features: `\"Win32_System_Services\"`*"]
pub const SERVICE_CONFIG_FAILURE_ACTIONS_FLAG: SERVICE_CONFIG = SERVICE_CONFIG(4u32);
#[doc = "*Required features: `\"Win32_System_Services\"`*"]
pub const SERVICE_CONFIG_PREFERRED_NODE: SERVICE_CONFIG = SERVICE_CONFIG(9u32);
#[doc = "*Required features: `\"Win32_System_Services\"`*"]
pub const SERVICE_CONFIG_PRESHUTDOWN_INFO: SERVICE_CONFIG = SERVICE_CONFIG(7u32);
#[doc = "*Required features: `\"Win32_System_Services\"`*"]
pub const SERVICE_CONFIG_REQUIRED_PRIVILEGES_INFO: SERVICE_CONFIG = SERVICE_CONFIG(6u32);
#[doc = "*Required features: `\"Win32_System_Services\"`*"]
pub const SERVICE_CONFIG_SERVICE_SID_INFO: SERVICE_CONFIG = SERVICE_CONFIG(5u32);
#[doc = "*Required features: `\"Win32_System_Services\"`*"]
pub const SERVICE_CONFIG_TRIGGER_INFO: SERVICE_CONFIG = SERVICE_CONFIG(8u32);
#[doc = "*Required features: `\"Win32_System_Services\"`*"]
pub const SERVICE_CONFIG_LAUNCH_PROTECTED: SERVICE_CONFIG = SERVICE_CONFIG(12u32);
impl ::core::marker::Copy for SERVICE_CONFIG {}
impl ::core::clone::Clone for SERVICE_CONFIG {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for SERVICE_CONFIG {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for SERVICE_CONFIG {
    type Abi = Self;
}
impl ::core::fmt::Debug for SERVICE_CONFIG {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SERVICE_CONFIG").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_System_Services\"`*"]
pub const SERVICE_CONTROL_CONTINUE: u32 = 3u32;
#[doc = "*Required features: `\"Win32_System_Services\"`*"]
pub const SERVICE_CONTROL_DEVICEEVENT: u32 = 11u32;
#[doc = "*Required features: `\"Win32_System_Services\"`*"]
pub const SERVICE_CONTROL_HARDWAREPROFILECHANGE: u32 = 12u32;
#[doc = "*Required features: `\"Win32_System_Services\"`*"]
pub const SERVICE_CONTROL_INTERROGATE: u32 = 4u32;
#[doc = "*Required features: `\"Win32_System_Services\"`*"]
pub const SERVICE_CONTROL_LOWRESOURCES: u32 = 96u32;
#[doc = "*Required features: `\"Win32_System_Services\"`*"]
pub const SERVICE_CONTROL_NETBINDADD: u32 = 7u32;
#[doc = "*Required features: `\"Win32_System_Services\"`*"]
pub const SERVICE_CONTROL_NETBINDDISABLE: u32 = 10u32;
#[doc = "*Required features: `\"Win32_System_Services\"`*"]
pub const SERVICE_CONTROL_NETBINDENABLE: u32 = 9u32;
#[doc = "*Required features: `\"Win32_System_Services\"`*"]
pub const SERVICE_CONTROL_NETBINDREMOVE: u32 = 8u32;
#[doc = "*Required features: `\"Win32_System_Services\"`*"]
pub const SERVICE_CONTROL_PARAMCHANGE: u32 = 6u32;
#[doc = "*Required features: `\"Win32_System_Services\"`*"]
pub const SERVICE_CONTROL_PAUSE: u32 = 2u32;
#[doc = "*Required features: `\"Win32_System_Services\"`*"]
pub const SERVICE_CONTROL_POWEREVENT: u32 = 13u32;
#[doc = "*Required features: `\"Win32_System_Services\"`*"]
pub const SERVICE_CONTROL_PRESHUTDOWN: u32 = 15u32;
#[doc = "*Required features: `\"Win32_System_Services\"`*"]
pub const SERVICE_CONTROL_SESSIONCHANGE: u32 = 14u32;
#[doc = "*Required features: `\"Win32_System_Services\"`*"]
pub const SERVICE_CONTROL_SHUTDOWN: u32 = 5u32;
#[doc = "*Required features: `\"Win32_System_Services\"`*"]
pub const SERVICE_CONTROL_STATUS_REASON_INFO: u32 = 1u32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Services\"`*"]
pub struct SERVICE_CONTROL_STATUS_REASON_PARAMSA {
    pub dwReason: u32,
    pub pszComment: ::windows::core::PSTR,
    pub ServiceStatus: SERVICE_STATUS_PROCESS,
}
impl ::core::marker::Copy for SERVICE_CONTROL_STATUS_REASON_PARAMSA {}
impl ::core::clone::Clone for SERVICE_CONTROL_STATUS_REASON_PARAMSA {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for SERVICE_CONTROL_STATUS_REASON_PARAMSA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SERVICE_CONTROL_STATUS_REASON_PARAMSA").field("dwReason", &self.dwReason).field("pszComment", &self.pszComment).field("ServiceStatus", &self.ServiceStatus).finish()
    }
}
unsafe impl ::windows::core::Abi for SERVICE_CONTROL_STATUS_REASON_PARAMSA {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for SERVICE_CONTROL_STATUS_REASON_PARAMSA {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<SERVICE_CONTROL_STATUS_REASON_PARAMSA>()) == 0 }
    }
}
impl ::core::cmp::Eq for SERVICE_CONTROL_STATUS_REASON_PARAMSA {}
impl ::core::default::Default for SERVICE_CONTROL_STATUS_REASON_PARAMSA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Services\"`*"]
pub struct SERVICE_CONTROL_STATUS_REASON_PARAMSW {
    pub dwReason: u32,
    pub pszComment: ::windows::core::PWSTR,
    pub ServiceStatus: SERVICE_STATUS_PROCESS,
}
impl ::core::marker::Copy for SERVICE_CONTROL_STATUS_REASON_PARAMSW {}
impl ::core::clone::Clone for SERVICE_CONTROL_STATUS_REASON_PARAMSW {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for SERVICE_CONTROL_STATUS_REASON_PARAMSW {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SERVICE_CONTROL_STATUS_REASON_PARAMSW").field("dwReason", &self.dwReason).field("pszComment", &self.pszComment).field("ServiceStatus", &self.ServiceStatus).finish()
    }
}
unsafe impl ::windows::core::Abi for SERVICE_CONTROL_STATUS_REASON_PARAMSW {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for SERVICE_CONTROL_STATUS_REASON_PARAMSW {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<SERVICE_CONTROL_STATUS_REASON_PARAMSW>()) == 0 }
    }
}
impl ::core::cmp::Eq for SERVICE_CONTROL_STATUS_REASON_PARAMSW {}
impl ::core::default::Default for SERVICE_CONTROL_STATUS_REASON_PARAMSW {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_System_Services\"`*"]
pub const SERVICE_CONTROL_STOP: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_Services\"`*"]
pub const SERVICE_CONTROL_SYSTEMLOWRESOURCES: u32 = 97u32;
#[doc = "*Required features: `\"Win32_System_Services\"`*"]
pub const SERVICE_CONTROL_TIMECHANGE: u32 = 16u32;
#[doc = "*Required features: `\"Win32_System_Services\"`*"]
pub const SERVICE_CONTROL_TRIGGEREVENT: u32 = 32u32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Services\"`*"]
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
#[doc = "*Required features: `\"Win32_System_Services\"`*"]
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
#[doc = "*Required features: `\"Win32_System_Services\"`*"]
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
#[doc = "*Required features: `\"Win32_System_Services\"`, `\"Win32_Foundation\"`*"]
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
#[doc = "*Required features: `\"Win32_System_Services\"`*"]
pub struct SERVICE_DESCRIPTIONA {
    pub lpDescription: ::windows::core::PSTR,
}
impl ::core::marker::Copy for SERVICE_DESCRIPTIONA {}
impl ::core::clone::Clone for SERVICE_DESCRIPTIONA {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for SERVICE_DESCRIPTIONA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SERVICE_DESCRIPTIONA").field("lpDescription", &self.lpDescription).finish()
    }
}
unsafe impl ::windows::core::Abi for SERVICE_DESCRIPTIONA {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for SERVICE_DESCRIPTIONA {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<SERVICE_DESCRIPTIONA>()) == 0 }
    }
}
impl ::core::cmp::Eq for SERVICE_DESCRIPTIONA {}
impl ::core::default::Default for SERVICE_DESCRIPTIONA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Services\"`*"]
pub struct SERVICE_DESCRIPTIONW {
    pub lpDescription: ::windows::core::PWSTR,
}
impl ::core::marker::Copy for SERVICE_DESCRIPTIONW {}
impl ::core::clone::Clone for SERVICE_DESCRIPTIONW {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for SERVICE_DESCRIPTIONW {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SERVICE_DESCRIPTIONW").field("lpDescription", &self.lpDescription).finish()
    }
}
unsafe impl ::windows::core::Abi for SERVICE_DESCRIPTIONW {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for SERVICE_DESCRIPTIONW {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<SERVICE_DESCRIPTIONW>()) == 0 }
    }
}
impl ::core::cmp::Eq for SERVICE_DESCRIPTIONW {}
impl ::core::default::Default for SERVICE_DESCRIPTIONW {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_System_Services\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct SERVICE_DIRECTORY_TYPE(pub i32);
#[doc = "*Required features: `\"Win32_System_Services\"`*"]
pub const ServiceDirectoryPersistentState: SERVICE_DIRECTORY_TYPE = SERVICE_DIRECTORY_TYPE(0i32);
#[doc = "*Required features: `\"Win32_System_Services\"`*"]
pub const ServiceDirectoryTypeMax: SERVICE_DIRECTORY_TYPE = SERVICE_DIRECTORY_TYPE(1i32);
impl ::core::marker::Copy for SERVICE_DIRECTORY_TYPE {}
impl ::core::clone::Clone for SERVICE_DIRECTORY_TYPE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for SERVICE_DIRECTORY_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for SERVICE_DIRECTORY_TYPE {
    type Abi = Self;
}
impl ::core::fmt::Debug for SERVICE_DIRECTORY_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SERVICE_DIRECTORY_TYPE").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_System_Services\"`*"]
pub const SERVICE_DYNAMIC_INFORMATION_LEVEL_START_REASON: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_Services\"`*"]
pub const SERVICE_ENUMERATE_DEPENDENTS: u32 = 8u32;
#[doc = "*Required features: `\"Win32_System_Services\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct SERVICE_ERROR(pub u32);
#[doc = "*Required features: `\"Win32_System_Services\"`*"]
pub const SERVICE_ERROR_CRITICAL: SERVICE_ERROR = SERVICE_ERROR(3u32);
#[doc = "*Required features: `\"Win32_System_Services\"`*"]
pub const SERVICE_ERROR_IGNORE: SERVICE_ERROR = SERVICE_ERROR(0u32);
#[doc = "*Required features: `\"Win32_System_Services\"`*"]
pub const SERVICE_ERROR_NORMAL: SERVICE_ERROR = SERVICE_ERROR(1u32);
#[doc = "*Required features: `\"Win32_System_Services\"`*"]
pub const SERVICE_ERROR_SEVERE: SERVICE_ERROR = SERVICE_ERROR(2u32);
impl ::core::marker::Copy for SERVICE_ERROR {}
impl ::core::clone::Clone for SERVICE_ERROR {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for SERVICE_ERROR {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for SERVICE_ERROR {
    type Abi = Self;
}
impl ::core::fmt::Debug for SERVICE_ERROR {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SERVICE_ERROR").field(&self.0).finish()
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Services\"`*"]
pub struct SERVICE_FAILURE_ACTIONSA {
    pub dwResetPeriod: u32,
    pub lpRebootMsg: ::windows::core::PSTR,
    pub lpCommand: ::windows::core::PSTR,
    pub cActions: u32,
    pub lpsaActions: *mut SC_ACTION,
}
impl ::core::marker::Copy for SERVICE_FAILURE_ACTIONSA {}
impl ::core::clone::Clone for SERVICE_FAILURE_ACTIONSA {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for SERVICE_FAILURE_ACTIONSA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SERVICE_FAILURE_ACTIONSA").field("dwResetPeriod", &self.dwResetPeriod).field("lpRebootMsg", &self.lpRebootMsg).field("lpCommand", &self.lpCommand).field("cActions", &self.cActions).field("lpsaActions", &self.lpsaActions).finish()
    }
}
unsafe impl ::windows::core::Abi for SERVICE_FAILURE_ACTIONSA {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for SERVICE_FAILURE_ACTIONSA {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<SERVICE_FAILURE_ACTIONSA>()) == 0 }
    }
}
impl ::core::cmp::Eq for SERVICE_FAILURE_ACTIONSA {}
impl ::core::default::Default for SERVICE_FAILURE_ACTIONSA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Services\"`*"]
pub struct SERVICE_FAILURE_ACTIONSW {
    pub dwResetPeriod: u32,
    pub lpRebootMsg: ::windows::core::PWSTR,
    pub lpCommand: ::windows::core::PWSTR,
    pub cActions: u32,
    pub lpsaActions: *mut SC_ACTION,
}
impl ::core::marker::Copy for SERVICE_FAILURE_ACTIONSW {}
impl ::core::clone::Clone for SERVICE_FAILURE_ACTIONSW {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for SERVICE_FAILURE_ACTIONSW {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SERVICE_FAILURE_ACTIONSW").field("dwResetPeriod", &self.dwResetPeriod).field("lpRebootMsg", &self.lpRebootMsg).field("lpCommand", &self.lpCommand).field("cActions", &self.cActions).field("lpsaActions", &self.lpsaActions).finish()
    }
}
unsafe impl ::windows::core::Abi for SERVICE_FAILURE_ACTIONSW {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for SERVICE_FAILURE_ACTIONSW {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<SERVICE_FAILURE_ACTIONSW>()) == 0 }
    }
}
impl ::core::cmp::Eq for SERVICE_FAILURE_ACTIONSW {}
impl ::core::default::Default for SERVICE_FAILURE_ACTIONSW {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Services\"`, `\"Win32_Foundation\"`*"]
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
#[doc = "*Required features: `\"Win32_System_Services\"`*"]
pub const SERVICE_INTERROGATE: u32 = 128u32;
#[doc = "*Required features: `\"Win32_System_Services\"`*"]
pub const SERVICE_LAUNCH_PROTECTED_ANTIMALWARE_LIGHT: u32 = 3u32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Services\"`*"]
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
#[doc = "*Required features: `\"Win32_System_Services\"`*"]
pub const SERVICE_LAUNCH_PROTECTED_NONE: u32 = 0u32;
#[doc = "*Required features: `\"Win32_System_Services\"`*"]
pub const SERVICE_LAUNCH_PROTECTED_WINDOWS: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_Services\"`*"]
pub const SERVICE_LAUNCH_PROTECTED_WINDOWS_LIGHT: u32 = 2u32;
#[doc = "*Required features: `\"Win32_System_Services\"`*"]
pub type SERVICE_MAIN_FUNCTIONA = ::core::option::Option<unsafe extern "system" fn(dwnumservicesargs: u32, lpserviceargvectors: *mut *mut i8)>;
#[doc = "*Required features: `\"Win32_System_Services\"`*"]
pub type SERVICE_MAIN_FUNCTIONW = ::core::option::Option<unsafe extern "system" fn(dwnumservicesargs: u32, lpserviceargvectors: *mut ::windows::core::PWSTR)>;
#[doc = "*Required features: `\"Win32_System_Services\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct SERVICE_NOTIFY(pub u32);
#[doc = "*Required features: `\"Win32_System_Services\"`*"]
pub const SERVICE_NOTIFY_CREATED: SERVICE_NOTIFY = SERVICE_NOTIFY(128u32);
#[doc = "*Required features: `\"Win32_System_Services\"`*"]
pub const SERVICE_NOTIFY_CONTINUE_PENDING: SERVICE_NOTIFY = SERVICE_NOTIFY(16u32);
#[doc = "*Required features: `\"Win32_System_Services\"`*"]
pub const SERVICE_NOTIFY_DELETE_PENDING: SERVICE_NOTIFY = SERVICE_NOTIFY(512u32);
#[doc = "*Required features: `\"Win32_System_Services\"`*"]
pub const SERVICE_NOTIFY_DELETED: SERVICE_NOTIFY = SERVICE_NOTIFY(256u32);
#[doc = "*Required features: `\"Win32_System_Services\"`*"]
pub const SERVICE_NOTIFY_PAUSE_PENDING: SERVICE_NOTIFY = SERVICE_NOTIFY(32u32);
#[doc = "*Required features: `\"Win32_System_Services\"`*"]
pub const SERVICE_NOTIFY_PAUSED: SERVICE_NOTIFY = SERVICE_NOTIFY(64u32);
#[doc = "*Required features: `\"Win32_System_Services\"`*"]
pub const SERVICE_NOTIFY_RUNNING: SERVICE_NOTIFY = SERVICE_NOTIFY(8u32);
#[doc = "*Required features: `\"Win32_System_Services\"`*"]
pub const SERVICE_NOTIFY_START_PENDING: SERVICE_NOTIFY = SERVICE_NOTIFY(2u32);
#[doc = "*Required features: `\"Win32_System_Services\"`*"]
pub const SERVICE_NOTIFY_STOP_PENDING: SERVICE_NOTIFY = SERVICE_NOTIFY(4u32);
#[doc = "*Required features: `\"Win32_System_Services\"`*"]
pub const SERVICE_NOTIFY_STOPPED: SERVICE_NOTIFY = SERVICE_NOTIFY(1u32);
impl ::core::marker::Copy for SERVICE_NOTIFY {}
impl ::core::clone::Clone for SERVICE_NOTIFY {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for SERVICE_NOTIFY {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for SERVICE_NOTIFY {
    type Abi = Self;
}
impl ::core::fmt::Debug for SERVICE_NOTIFY {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SERVICE_NOTIFY").field(&self.0).finish()
    }
}
impl ::core::ops::BitOr for SERVICE_NOTIFY {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for SERVICE_NOTIFY {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for SERVICE_NOTIFY {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for SERVICE_NOTIFY {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for SERVICE_NOTIFY {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Services\"`*"]
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
#[doc = "*Required features: `\"Win32_System_Services\"`*"]
pub struct SERVICE_NOTIFY_2A {
    pub dwVersion: u32,
    pub pfnNotifyCallback: PFN_SC_NOTIFY_CALLBACK,
    pub pContext: *mut ::core::ffi::c_void,
    pub dwNotificationStatus: u32,
    pub ServiceStatus: SERVICE_STATUS_PROCESS,
    pub dwNotificationTriggered: u32,
    pub pszServiceNames: ::windows::core::PSTR,
}
impl ::core::marker::Copy for SERVICE_NOTIFY_2A {}
impl ::core::clone::Clone for SERVICE_NOTIFY_2A {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for SERVICE_NOTIFY_2A {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SERVICE_NOTIFY_2A").field("dwVersion", &self.dwVersion).field("pfnNotifyCallback", &self.pfnNotifyCallback.map(|f| f as usize)).field("pContext", &self.pContext).field("dwNotificationStatus", &self.dwNotificationStatus).field("ServiceStatus", &self.ServiceStatus).field("dwNotificationTriggered", &self.dwNotificationTriggered).field("pszServiceNames", &self.pszServiceNames).finish()
    }
}
unsafe impl ::windows::core::Abi for SERVICE_NOTIFY_2A {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for SERVICE_NOTIFY_2A {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<SERVICE_NOTIFY_2A>()) == 0 }
    }
}
impl ::core::cmp::Eq for SERVICE_NOTIFY_2A {}
impl ::core::default::Default for SERVICE_NOTIFY_2A {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Services\"`*"]
pub struct SERVICE_NOTIFY_2W {
    pub dwVersion: u32,
    pub pfnNotifyCallback: PFN_SC_NOTIFY_CALLBACK,
    pub pContext: *mut ::core::ffi::c_void,
    pub dwNotificationStatus: u32,
    pub ServiceStatus: SERVICE_STATUS_PROCESS,
    pub dwNotificationTriggered: u32,
    pub pszServiceNames: ::windows::core::PWSTR,
}
impl ::core::marker::Copy for SERVICE_NOTIFY_2W {}
impl ::core::clone::Clone for SERVICE_NOTIFY_2W {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for SERVICE_NOTIFY_2W {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SERVICE_NOTIFY_2W").field("dwVersion", &self.dwVersion).field("pfnNotifyCallback", &self.pfnNotifyCallback.map(|f| f as usize)).field("pContext", &self.pContext).field("dwNotificationStatus", &self.dwNotificationStatus).field("ServiceStatus", &self.ServiceStatus).field("dwNotificationTriggered", &self.dwNotificationTriggered).field("pszServiceNames", &self.pszServiceNames).finish()
    }
}
unsafe impl ::windows::core::Abi for SERVICE_NOTIFY_2W {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for SERVICE_NOTIFY_2W {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<SERVICE_NOTIFY_2W>()) == 0 }
    }
}
impl ::core::cmp::Eq for SERVICE_NOTIFY_2W {}
impl ::core::default::Default for SERVICE_NOTIFY_2W {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_System_Services\"`*"]
pub const SERVICE_NOTIFY_STATUS_CHANGE: u32 = 2u32;
#[doc = "*Required features: `\"Win32_System_Services\"`*"]
pub const SERVICE_NOTIFY_STATUS_CHANGE_1: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_Services\"`*"]
pub const SERVICE_NOTIFY_STATUS_CHANGE_2: u32 = 2u32;
#[doc = "*Required features: `\"Win32_System_Services\"`*"]
pub const SERVICE_NO_CHANGE: u32 = 4294967295u32;
#[doc = "*Required features: `\"Win32_System_Services\"`*"]
pub const SERVICE_PAUSE_CONTINUE: u32 = 64u32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Services\"`, `\"Win32_Foundation\"`*"]
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
#[doc = "*Required features: `\"Win32_System_Services\"`*"]
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
#[doc = "*Required features: `\"Win32_System_Services\"`*"]
pub const SERVICE_QUERY_CONFIG: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_Services\"`*"]
pub const SERVICE_QUERY_STATUS: u32 = 4u32;
#[doc = "*Required features: `\"Win32_System_Services\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct SERVICE_REGISTRY_STATE_TYPE(pub i32);
#[doc = "*Required features: `\"Win32_System_Services\"`*"]
pub const ServiceRegistryStateParameters: SERVICE_REGISTRY_STATE_TYPE = SERVICE_REGISTRY_STATE_TYPE(0i32);
#[doc = "*Required features: `\"Win32_System_Services\"`*"]
pub const ServiceRegistryStatePersistent: SERVICE_REGISTRY_STATE_TYPE = SERVICE_REGISTRY_STATE_TYPE(1i32);
#[doc = "*Required features: `\"Win32_System_Services\"`*"]
pub const MaxServiceRegistryStateType: SERVICE_REGISTRY_STATE_TYPE = SERVICE_REGISTRY_STATE_TYPE(2i32);
impl ::core::marker::Copy for SERVICE_REGISTRY_STATE_TYPE {}
impl ::core::clone::Clone for SERVICE_REGISTRY_STATE_TYPE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for SERVICE_REGISTRY_STATE_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for SERVICE_REGISTRY_STATE_TYPE {
    type Abi = Self;
}
impl ::core::fmt::Debug for SERVICE_REGISTRY_STATE_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SERVICE_REGISTRY_STATE_TYPE").field(&self.0).finish()
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Services\"`*"]
pub struct SERVICE_REQUIRED_PRIVILEGES_INFOA {
    pub pmszRequiredPrivileges: ::windows::core::PSTR,
}
impl ::core::marker::Copy for SERVICE_REQUIRED_PRIVILEGES_INFOA {}
impl ::core::clone::Clone for SERVICE_REQUIRED_PRIVILEGES_INFOA {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for SERVICE_REQUIRED_PRIVILEGES_INFOA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SERVICE_REQUIRED_PRIVILEGES_INFOA").field("pmszRequiredPrivileges", &self.pmszRequiredPrivileges).finish()
    }
}
unsafe impl ::windows::core::Abi for SERVICE_REQUIRED_PRIVILEGES_INFOA {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for SERVICE_REQUIRED_PRIVILEGES_INFOA {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<SERVICE_REQUIRED_PRIVILEGES_INFOA>()) == 0 }
    }
}
impl ::core::cmp::Eq for SERVICE_REQUIRED_PRIVILEGES_INFOA {}
impl ::core::default::Default for SERVICE_REQUIRED_PRIVILEGES_INFOA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Services\"`*"]
pub struct SERVICE_REQUIRED_PRIVILEGES_INFOW {
    pub pmszRequiredPrivileges: ::windows::core::PWSTR,
}
impl ::core::marker::Copy for SERVICE_REQUIRED_PRIVILEGES_INFOW {}
impl ::core::clone::Clone for SERVICE_REQUIRED_PRIVILEGES_INFOW {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for SERVICE_REQUIRED_PRIVILEGES_INFOW {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SERVICE_REQUIRED_PRIVILEGES_INFOW").field("pmszRequiredPrivileges", &self.pmszRequiredPrivileges).finish()
    }
}
unsafe impl ::windows::core::Abi for SERVICE_REQUIRED_PRIVILEGES_INFOW {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for SERVICE_REQUIRED_PRIVILEGES_INFOW {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<SERVICE_REQUIRED_PRIVILEGES_INFOW>()) == 0 }
    }
}
impl ::core::cmp::Eq for SERVICE_REQUIRED_PRIVILEGES_INFOW {}
impl ::core::default::Default for SERVICE_REQUIRED_PRIVILEGES_INFOW {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_System_Services\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct SERVICE_RUNS_IN_PROCESS(pub u32);
#[doc = "*Required features: `\"Win32_System_Services\"`*"]
pub const SERVICE_RUNS_IN_NON_SYSTEM_OR_NOT_RUNNING: SERVICE_RUNS_IN_PROCESS = SERVICE_RUNS_IN_PROCESS(0u32);
#[doc = "*Required features: `\"Win32_System_Services\"`*"]
pub const SERVICE_RUNS_IN_SYSTEM_PROCESS: SERVICE_RUNS_IN_PROCESS = SERVICE_RUNS_IN_PROCESS(1u32);
impl ::core::marker::Copy for SERVICE_RUNS_IN_PROCESS {}
impl ::core::clone::Clone for SERVICE_RUNS_IN_PROCESS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for SERVICE_RUNS_IN_PROCESS {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for SERVICE_RUNS_IN_PROCESS {
    type Abi = Self;
}
impl ::core::fmt::Debug for SERVICE_RUNS_IN_PROCESS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SERVICE_RUNS_IN_PROCESS").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_System_Services\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct SERVICE_SHARED_DIRECTORY_TYPE(pub i32);
#[doc = "*Required features: `\"Win32_System_Services\"`*"]
pub const ServiceSharedDirectoryPersistentState: SERVICE_SHARED_DIRECTORY_TYPE = SERVICE_SHARED_DIRECTORY_TYPE(0i32);
impl ::core::marker::Copy for SERVICE_SHARED_DIRECTORY_TYPE {}
impl ::core::clone::Clone for SERVICE_SHARED_DIRECTORY_TYPE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for SERVICE_SHARED_DIRECTORY_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for SERVICE_SHARED_DIRECTORY_TYPE {
    type Abi = Self;
}
impl ::core::fmt::Debug for SERVICE_SHARED_DIRECTORY_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SERVICE_SHARED_DIRECTORY_TYPE").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_System_Services\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct SERVICE_SHARED_REGISTRY_STATE_TYPE(pub i32);
#[doc = "*Required features: `\"Win32_System_Services\"`*"]
pub const ServiceSharedRegistryPersistentState: SERVICE_SHARED_REGISTRY_STATE_TYPE = SERVICE_SHARED_REGISTRY_STATE_TYPE(0i32);
impl ::core::marker::Copy for SERVICE_SHARED_REGISTRY_STATE_TYPE {}
impl ::core::clone::Clone for SERVICE_SHARED_REGISTRY_STATE_TYPE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for SERVICE_SHARED_REGISTRY_STATE_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for SERVICE_SHARED_REGISTRY_STATE_TYPE {
    type Abi = Self;
}
impl ::core::fmt::Debug for SERVICE_SHARED_REGISTRY_STATE_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SERVICE_SHARED_REGISTRY_STATE_TYPE").field(&self.0).finish()
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Services\"`*"]
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
#[doc = "*Required features: `\"Win32_System_Services\"`*"]
pub const SERVICE_SID_TYPE_NONE: u32 = 0u32;
#[doc = "*Required features: `\"Win32_System_Services\"`*"]
pub const SERVICE_SID_TYPE_UNRESTRICTED: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_Services\"`*"]
pub const SERVICE_START: u32 = 16u32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Services\"`*"]
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
#[doc = "*Required features: `\"Win32_System_Services\"`*"]
pub const SERVICE_START_REASON_AUTO: u32 = 2u32;
#[doc = "*Required features: `\"Win32_System_Services\"`*"]
pub const SERVICE_START_REASON_DELAYEDAUTO: u32 = 16u32;
#[doc = "*Required features: `\"Win32_System_Services\"`*"]
pub const SERVICE_START_REASON_DEMAND: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_Services\"`*"]
pub const SERVICE_START_REASON_RESTART_ON_FAILURE: u32 = 8u32;
#[doc = "*Required features: `\"Win32_System_Services\"`*"]
pub const SERVICE_START_REASON_TRIGGER: u32 = 4u32;
#[doc = "*Required features: `\"Win32_System_Services\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct SERVICE_START_TYPE(pub u32);
#[doc = "*Required features: `\"Win32_System_Services\"`*"]
pub const SERVICE_AUTO_START: SERVICE_START_TYPE = SERVICE_START_TYPE(2u32);
#[doc = "*Required features: `\"Win32_System_Services\"`*"]
pub const SERVICE_BOOT_START: SERVICE_START_TYPE = SERVICE_START_TYPE(0u32);
#[doc = "*Required features: `\"Win32_System_Services\"`*"]
pub const SERVICE_DEMAND_START: SERVICE_START_TYPE = SERVICE_START_TYPE(3u32);
#[doc = "*Required features: `\"Win32_System_Services\"`*"]
pub const SERVICE_DISABLED: SERVICE_START_TYPE = SERVICE_START_TYPE(4u32);
#[doc = "*Required features: `\"Win32_System_Services\"`*"]
pub const SERVICE_SYSTEM_START: SERVICE_START_TYPE = SERVICE_START_TYPE(1u32);
impl ::core::marker::Copy for SERVICE_START_TYPE {}
impl ::core::clone::Clone for SERVICE_START_TYPE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for SERVICE_START_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for SERVICE_START_TYPE {
    type Abi = Self;
}
impl ::core::fmt::Debug for SERVICE_START_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SERVICE_START_TYPE").field(&self.0).finish()
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Services\"`*"]
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
#[doc = "*Required features: `\"Win32_System_Services\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct SERVICE_STATUS_CURRENT_STATE(pub u32);
#[doc = "*Required features: `\"Win32_System_Services\"`*"]
pub const SERVICE_CONTINUE_PENDING: SERVICE_STATUS_CURRENT_STATE = SERVICE_STATUS_CURRENT_STATE(5u32);
#[doc = "*Required features: `\"Win32_System_Services\"`*"]
pub const SERVICE_PAUSE_PENDING: SERVICE_STATUS_CURRENT_STATE = SERVICE_STATUS_CURRENT_STATE(6u32);
#[doc = "*Required features: `\"Win32_System_Services\"`*"]
pub const SERVICE_PAUSED: SERVICE_STATUS_CURRENT_STATE = SERVICE_STATUS_CURRENT_STATE(7u32);
#[doc = "*Required features: `\"Win32_System_Services\"`*"]
pub const SERVICE_RUNNING: SERVICE_STATUS_CURRENT_STATE = SERVICE_STATUS_CURRENT_STATE(4u32);
#[doc = "*Required features: `\"Win32_System_Services\"`*"]
pub const SERVICE_START_PENDING: SERVICE_STATUS_CURRENT_STATE = SERVICE_STATUS_CURRENT_STATE(2u32);
#[doc = "*Required features: `\"Win32_System_Services\"`*"]
pub const SERVICE_STOP_PENDING: SERVICE_STATUS_CURRENT_STATE = SERVICE_STATUS_CURRENT_STATE(3u32);
#[doc = "*Required features: `\"Win32_System_Services\"`*"]
pub const SERVICE_STOPPED: SERVICE_STATUS_CURRENT_STATE = SERVICE_STATUS_CURRENT_STATE(1u32);
impl ::core::marker::Copy for SERVICE_STATUS_CURRENT_STATE {}
impl ::core::clone::Clone for SERVICE_STATUS_CURRENT_STATE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for SERVICE_STATUS_CURRENT_STATE {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for SERVICE_STATUS_CURRENT_STATE {
    type Abi = Self;
}
impl ::core::fmt::Debug for SERVICE_STATUS_CURRENT_STATE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SERVICE_STATUS_CURRENT_STATE").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct SERVICE_STATUS_HANDLE(pub isize);
impl SERVICE_STATUS_HANDLE {
    pub fn is_invalid(&self) -> bool {
        self.0 == -1 || self.0 == 0
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
impl ::core::convert::From<::core::option::Option<SERVICE_STATUS_HANDLE>> for SERVICE_STATUS_HANDLE {
    fn from(optional: ::core::option::Option<SERVICE_STATUS_HANDLE>) -> SERVICE_STATUS_HANDLE {
        optional.unwrap_or_default()
    }
}
unsafe impl ::windows::core::Abi for SERVICE_STATUS_HANDLE {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Services\"`*"]
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
#[doc = "*Required features: `\"Win32_System_Services\"`*"]
pub const SERVICE_STOP: u32 = 32u32;
#[doc = "*Required features: `\"Win32_System_Services\"`*"]
pub const SERVICE_STOP_REASON_FLAG_CUSTOM: u32 = 536870912u32;
#[doc = "*Required features: `\"Win32_System_Services\"`*"]
pub const SERVICE_STOP_REASON_FLAG_MAX: u32 = 2147483648u32;
#[doc = "*Required features: `\"Win32_System_Services\"`*"]
pub const SERVICE_STOP_REASON_FLAG_MIN: u32 = 0u32;
#[doc = "*Required features: `\"Win32_System_Services\"`*"]
pub const SERVICE_STOP_REASON_FLAG_PLANNED: u32 = 1073741824u32;
#[doc = "*Required features: `\"Win32_System_Services\"`*"]
pub const SERVICE_STOP_REASON_FLAG_UNPLANNED: u32 = 268435456u32;
#[doc = "*Required features: `\"Win32_System_Services\"`*"]
pub const SERVICE_STOP_REASON_MAJOR_APPLICATION: u32 = 327680u32;
#[doc = "*Required features: `\"Win32_System_Services\"`*"]
pub const SERVICE_STOP_REASON_MAJOR_HARDWARE: u32 = 131072u32;
#[doc = "*Required features: `\"Win32_System_Services\"`*"]
pub const SERVICE_STOP_REASON_MAJOR_MAX: u32 = 458752u32;
#[doc = "*Required features: `\"Win32_System_Services\"`*"]
pub const SERVICE_STOP_REASON_MAJOR_MAX_CUSTOM: u32 = 16711680u32;
#[doc = "*Required features: `\"Win32_System_Services\"`*"]
pub const SERVICE_STOP_REASON_MAJOR_MIN: u32 = 0u32;
#[doc = "*Required features: `\"Win32_System_Services\"`*"]
pub const SERVICE_STOP_REASON_MAJOR_MIN_CUSTOM: u32 = 4194304u32;
#[doc = "*Required features: `\"Win32_System_Services\"`*"]
pub const SERVICE_STOP_REASON_MAJOR_NONE: u32 = 393216u32;
#[doc = "*Required features: `\"Win32_System_Services\"`*"]
pub const SERVICE_STOP_REASON_MAJOR_OPERATINGSYSTEM: u32 = 196608u32;
#[doc = "*Required features: `\"Win32_System_Services\"`*"]
pub const SERVICE_STOP_REASON_MAJOR_OTHER: u32 = 65536u32;
#[doc = "*Required features: `\"Win32_System_Services\"`*"]
pub const SERVICE_STOP_REASON_MAJOR_SOFTWARE: u32 = 262144u32;
#[doc = "*Required features: `\"Win32_System_Services\"`*"]
pub const SERVICE_STOP_REASON_MINOR_DISK: u32 = 8u32;
#[doc = "*Required features: `\"Win32_System_Services\"`*"]
pub const SERVICE_STOP_REASON_MINOR_ENVIRONMENT: u32 = 10u32;
#[doc = "*Required features: `\"Win32_System_Services\"`*"]
pub const SERVICE_STOP_REASON_MINOR_HARDWARE_DRIVER: u32 = 11u32;
#[doc = "*Required features: `\"Win32_System_Services\"`*"]
pub const SERVICE_STOP_REASON_MINOR_HUNG: u32 = 6u32;
#[doc = "*Required features: `\"Win32_System_Services\"`*"]
pub const SERVICE_STOP_REASON_MINOR_INSTALLATION: u32 = 3u32;
#[doc = "*Required features: `\"Win32_System_Services\"`*"]
pub const SERVICE_STOP_REASON_MINOR_MAINTENANCE: u32 = 2u32;
#[doc = "*Required features: `\"Win32_System_Services\"`*"]
pub const SERVICE_STOP_REASON_MINOR_MAX: u32 = 25u32;
#[doc = "*Required features: `\"Win32_System_Services\"`*"]
pub const SERVICE_STOP_REASON_MINOR_MAX_CUSTOM: u32 = 65535u32;
#[doc = "*Required features: `\"Win32_System_Services\"`*"]
pub const SERVICE_STOP_REASON_MINOR_MEMOTYLIMIT: u32 = 24u32;
#[doc = "*Required features: `\"Win32_System_Services\"`*"]
pub const SERVICE_STOP_REASON_MINOR_MIN: u32 = 0u32;
#[doc = "*Required features: `\"Win32_System_Services\"`*"]
pub const SERVICE_STOP_REASON_MINOR_MIN_CUSTOM: u32 = 256u32;
#[doc = "*Required features: `\"Win32_System_Services\"`*"]
pub const SERVICE_STOP_REASON_MINOR_MMC: u32 = 22u32;
#[doc = "*Required features: `\"Win32_System_Services\"`*"]
pub const SERVICE_STOP_REASON_MINOR_NETWORKCARD: u32 = 9u32;
#[doc = "*Required features: `\"Win32_System_Services\"`*"]
pub const SERVICE_STOP_REASON_MINOR_NETWORK_CONNECTIVITY: u32 = 17u32;
#[doc = "*Required features: `\"Win32_System_Services\"`*"]
pub const SERVICE_STOP_REASON_MINOR_NONE: u32 = 23u32;
#[doc = "*Required features: `\"Win32_System_Services\"`*"]
pub const SERVICE_STOP_REASON_MINOR_OTHER: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_Services\"`*"]
pub const SERVICE_STOP_REASON_MINOR_OTHERDRIVER: u32 = 12u32;
#[doc = "*Required features: `\"Win32_System_Services\"`*"]
pub const SERVICE_STOP_REASON_MINOR_RECONFIG: u32 = 5u32;
#[doc = "*Required features: `\"Win32_System_Services\"`*"]
pub const SERVICE_STOP_REASON_MINOR_SECURITY: u32 = 16u32;
#[doc = "*Required features: `\"Win32_System_Services\"`*"]
pub const SERVICE_STOP_REASON_MINOR_SECURITYFIX: u32 = 15u32;
#[doc = "*Required features: `\"Win32_System_Services\"`*"]
pub const SERVICE_STOP_REASON_MINOR_SECURITYFIX_UNINSTALL: u32 = 21u32;
#[doc = "*Required features: `\"Win32_System_Services\"`*"]
pub const SERVICE_STOP_REASON_MINOR_SERVICEPACK: u32 = 13u32;
#[doc = "*Required features: `\"Win32_System_Services\"`*"]
pub const SERVICE_STOP_REASON_MINOR_SERVICEPACK_UNINSTALL: u32 = 19u32;
#[doc = "*Required features: `\"Win32_System_Services\"`*"]
pub const SERVICE_STOP_REASON_MINOR_SOFTWARE_UPDATE: u32 = 14u32;
#[doc = "*Required features: `\"Win32_System_Services\"`*"]
pub const SERVICE_STOP_REASON_MINOR_SOFTWARE_UPDATE_UNINSTALL: u32 = 20u32;
#[doc = "*Required features: `\"Win32_System_Services\"`*"]
pub const SERVICE_STOP_REASON_MINOR_UNSTABLE: u32 = 7u32;
#[doc = "*Required features: `\"Win32_System_Services\"`*"]
pub const SERVICE_STOP_REASON_MINOR_UPGRADE: u32 = 4u32;
#[doc = "*Required features: `\"Win32_System_Services\"`*"]
pub const SERVICE_STOP_REASON_MINOR_WMI: u32 = 18u32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Services\"`*"]
pub struct SERVICE_TABLE_ENTRYA {
    pub lpServiceName: ::windows::core::PSTR,
    pub lpServiceProc: LPSERVICE_MAIN_FUNCTIONA,
}
impl ::core::marker::Copy for SERVICE_TABLE_ENTRYA {}
impl ::core::clone::Clone for SERVICE_TABLE_ENTRYA {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for SERVICE_TABLE_ENTRYA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SERVICE_TABLE_ENTRYA").field("lpServiceName", &self.lpServiceName).field("lpServiceProc", &self.lpServiceProc.map(|f| f as usize)).finish()
    }
}
unsafe impl ::windows::core::Abi for SERVICE_TABLE_ENTRYA {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for SERVICE_TABLE_ENTRYA {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<SERVICE_TABLE_ENTRYA>()) == 0 }
    }
}
impl ::core::cmp::Eq for SERVICE_TABLE_ENTRYA {}
impl ::core::default::Default for SERVICE_TABLE_ENTRYA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Services\"`*"]
pub struct SERVICE_TABLE_ENTRYW {
    pub lpServiceName: ::windows::core::PWSTR,
    pub lpServiceProc: LPSERVICE_MAIN_FUNCTIONW,
}
impl ::core::marker::Copy for SERVICE_TABLE_ENTRYW {}
impl ::core::clone::Clone for SERVICE_TABLE_ENTRYW {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for SERVICE_TABLE_ENTRYW {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SERVICE_TABLE_ENTRYW").field("lpServiceName", &self.lpServiceName).field("lpServiceProc", &self.lpServiceProc.map(|f| f as usize)).finish()
    }
}
unsafe impl ::windows::core::Abi for SERVICE_TABLE_ENTRYW {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for SERVICE_TABLE_ENTRYW {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<SERVICE_TABLE_ENTRYW>()) == 0 }
    }
}
impl ::core::cmp::Eq for SERVICE_TABLE_ENTRYW {}
impl ::core::default::Default for SERVICE_TABLE_ENTRYW {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Services\"`*"]
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
#[doc = "*Required features: `\"Win32_System_Services\"`*"]
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
#[doc = "*Required features: `\"Win32_System_Services\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct SERVICE_TRIGGER_ACTION(pub u32);
#[doc = "*Required features: `\"Win32_System_Services\"`*"]
pub const SERVICE_TRIGGER_ACTION_SERVICE_START: SERVICE_TRIGGER_ACTION = SERVICE_TRIGGER_ACTION(1u32);
#[doc = "*Required features: `\"Win32_System_Services\"`*"]
pub const SERVICE_TRIGGER_ACTION_SERVICE_STOP: SERVICE_TRIGGER_ACTION = SERVICE_TRIGGER_ACTION(2u32);
impl ::core::marker::Copy for SERVICE_TRIGGER_ACTION {}
impl ::core::clone::Clone for SERVICE_TRIGGER_ACTION {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for SERVICE_TRIGGER_ACTION {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for SERVICE_TRIGGER_ACTION {
    type Abi = Self;
}
impl ::core::fmt::Debug for SERVICE_TRIGGER_ACTION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SERVICE_TRIGGER_ACTION").field(&self.0).finish()
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Services\"`*"]
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
#[doc = "*Required features: `\"Win32_System_Services\"`*"]
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
#[doc = "*Required features: `\"Win32_System_Services\"`*"]
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
#[doc = "*Required features: `\"Win32_System_Services\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct SERVICE_TRIGGER_SPECIFIC_DATA_ITEM_DATA_TYPE(pub u32);
#[doc = "*Required features: `\"Win32_System_Services\"`*"]
pub const SERVICE_TRIGGER_DATA_TYPE_BINARY: SERVICE_TRIGGER_SPECIFIC_DATA_ITEM_DATA_TYPE = SERVICE_TRIGGER_SPECIFIC_DATA_ITEM_DATA_TYPE(1u32);
#[doc = "*Required features: `\"Win32_System_Services\"`*"]
pub const SERVICE_TRIGGER_DATA_TYPE_STRING: SERVICE_TRIGGER_SPECIFIC_DATA_ITEM_DATA_TYPE = SERVICE_TRIGGER_SPECIFIC_DATA_ITEM_DATA_TYPE(2u32);
#[doc = "*Required features: `\"Win32_System_Services\"`*"]
pub const SERVICE_TRIGGER_DATA_TYPE_LEVEL: SERVICE_TRIGGER_SPECIFIC_DATA_ITEM_DATA_TYPE = SERVICE_TRIGGER_SPECIFIC_DATA_ITEM_DATA_TYPE(3u32);
#[doc = "*Required features: `\"Win32_System_Services\"`*"]
pub const SERVICE_TRIGGER_DATA_TYPE_KEYWORD_ANY: SERVICE_TRIGGER_SPECIFIC_DATA_ITEM_DATA_TYPE = SERVICE_TRIGGER_SPECIFIC_DATA_ITEM_DATA_TYPE(4u32);
#[doc = "*Required features: `\"Win32_System_Services\"`*"]
pub const SERVICE_TRIGGER_DATA_TYPE_KEYWORD_ALL: SERVICE_TRIGGER_SPECIFIC_DATA_ITEM_DATA_TYPE = SERVICE_TRIGGER_SPECIFIC_DATA_ITEM_DATA_TYPE(5u32);
impl ::core::marker::Copy for SERVICE_TRIGGER_SPECIFIC_DATA_ITEM_DATA_TYPE {}
impl ::core::clone::Clone for SERVICE_TRIGGER_SPECIFIC_DATA_ITEM_DATA_TYPE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for SERVICE_TRIGGER_SPECIFIC_DATA_ITEM_DATA_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for SERVICE_TRIGGER_SPECIFIC_DATA_ITEM_DATA_TYPE {
    type Abi = Self;
}
impl ::core::fmt::Debug for SERVICE_TRIGGER_SPECIFIC_DATA_ITEM_DATA_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SERVICE_TRIGGER_SPECIFIC_DATA_ITEM_DATA_TYPE").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_System_Services\"`*"]
pub const SERVICE_TRIGGER_STARTED_ARGUMENT: &str = "TriggerStarted";
#[doc = "*Required features: `\"Win32_System_Services\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct SERVICE_TRIGGER_TYPE(pub u32);
#[doc = "*Required features: `\"Win32_System_Services\"`*"]
pub const SERVICE_TRIGGER_TYPE_CUSTOM: SERVICE_TRIGGER_TYPE = SERVICE_TRIGGER_TYPE(20u32);
#[doc = "*Required features: `\"Win32_System_Services\"`*"]
pub const SERVICE_TRIGGER_TYPE_DEVICE_INTERFACE_ARRIVAL: SERVICE_TRIGGER_TYPE = SERVICE_TRIGGER_TYPE(1u32);
#[doc = "*Required features: `\"Win32_System_Services\"`*"]
pub const SERVICE_TRIGGER_TYPE_DOMAIN_JOIN: SERVICE_TRIGGER_TYPE = SERVICE_TRIGGER_TYPE(3u32);
#[doc = "*Required features: `\"Win32_System_Services\"`*"]
pub const SERVICE_TRIGGER_TYPE_FIREWALL_PORT_EVENT: SERVICE_TRIGGER_TYPE = SERVICE_TRIGGER_TYPE(4u32);
#[doc = "*Required features: `\"Win32_System_Services\"`*"]
pub const SERVICE_TRIGGER_TYPE_GROUP_POLICY: SERVICE_TRIGGER_TYPE = SERVICE_TRIGGER_TYPE(5u32);
#[doc = "*Required features: `\"Win32_System_Services\"`*"]
pub const SERVICE_TRIGGER_TYPE_IP_ADDRESS_AVAILABILITY: SERVICE_TRIGGER_TYPE = SERVICE_TRIGGER_TYPE(2u32);
#[doc = "*Required features: `\"Win32_System_Services\"`*"]
pub const SERVICE_TRIGGER_TYPE_NETWORK_ENDPOINT: SERVICE_TRIGGER_TYPE = SERVICE_TRIGGER_TYPE(6u32);
impl ::core::marker::Copy for SERVICE_TRIGGER_TYPE {}
impl ::core::clone::Clone for SERVICE_TRIGGER_TYPE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for SERVICE_TRIGGER_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for SERVICE_TRIGGER_TYPE {
    type Abi = Self;
}
impl ::core::fmt::Debug for SERVICE_TRIGGER_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SERVICE_TRIGGER_TYPE").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_System_Services\"`*"]
pub const SERVICE_TRIGGER_TYPE_AGGREGATE: u32 = 30u32;
#[doc = "*Required features: `\"Win32_System_Services\"`*"]
pub const SERVICE_TRIGGER_TYPE_CUSTOM_SYSTEM_STATE_CHANGE: u32 = 7u32;
#[doc = "*Required features: `\"Win32_System_Services\"`*"]
pub const SERVICE_USER_DEFINED_CONTROL: u32 = 256u32;
#[doc = "*Required features: `\"Win32_System_Services\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn SetServiceBits<'a, P0, P1, P2>(hservicestatus: P0, dwservicebits: u32, bsetbitson: P1, bupdateimmediately: P2) -> super::super::Foundation::BOOL
where
    P0: ::std::convert::Into<SERVICE_STATUS_HANDLE>,
    P1: ::std::convert::Into<super::super::Foundation::BOOL>,
    P2: ::std::convert::Into<super::super::Foundation::BOOL>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn SetServiceBits(hservicestatus: SERVICE_STATUS_HANDLE, dwservicebits: u32, bsetbitson: super::super::Foundation::BOOL, bupdateimmediately: super::super::Foundation::BOOL) -> super::super::Foundation::BOOL;
    }
    SetServiceBits(hservicestatus.into(), dwservicebits, bsetbitson.into(), bupdateimmediately.into())
}
#[doc = "*Required features: `\"Win32_System_Services\"`, `\"Win32_Foundation\"`, `\"Win32_Security\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
#[inline]
pub unsafe fn SetServiceObjectSecurity<'a, P0, P1>(hservice: P0, dwsecurityinformation: super::super::Security::OBJECT_SECURITY_INFORMATION, lpsecuritydescriptor: P1) -> super::super::Foundation::BOOL
where
    P0: ::std::convert::Into<super::super::Security::SC_HANDLE>,
    P1: ::std::convert::Into<super::super::Security::PSECURITY_DESCRIPTOR>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn SetServiceObjectSecurity(hservice: super::super::Security::SC_HANDLE, dwsecurityinformation: super::super::Security::OBJECT_SECURITY_INFORMATION, lpsecuritydescriptor: super::super::Security::PSECURITY_DESCRIPTOR) -> super::super::Foundation::BOOL;
    }
    SetServiceObjectSecurity(hservice.into(), dwsecurityinformation, lpsecuritydescriptor.into())
}
#[doc = "*Required features: `\"Win32_System_Services\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn SetServiceStatus<'a, P0>(hservicestatus: P0, lpservicestatus: *const SERVICE_STATUS) -> super::super::Foundation::BOOL
where
    P0: ::std::convert::Into<SERVICE_STATUS_HANDLE>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn SetServiceStatus(hservicestatus: SERVICE_STATUS_HANDLE, lpservicestatus: *const SERVICE_STATUS) -> super::super::Foundation::BOOL;
    }
    SetServiceStatus(hservicestatus.into(), ::core::mem::transmute(lpservicestatus))
}
#[doc = "*Required features: `\"Win32_System_Services\"`, `\"Win32_Foundation\"`, `\"Win32_Security\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
#[inline]
pub unsafe fn StartServiceA<'a, P0>(hservice: P0, lpserviceargvectors: &[::windows::core::PSTR]) -> super::super::Foundation::BOOL
where
    P0: ::std::convert::Into<super::super::Security::SC_HANDLE>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn StartServiceA(hservice: super::super::Security::SC_HANDLE, dwnumserviceargs: u32, lpserviceargvectors: *const ::windows::core::PSTR) -> super::super::Foundation::BOOL;
    }
    StartServiceA(hservice.into(), lpserviceargvectors.len() as _, ::core::mem::transmute(::windows::core::as_ptr_or_null(lpserviceargvectors)))
}
#[doc = "*Required features: `\"Win32_System_Services\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn StartServiceCtrlDispatcherA(lpservicestarttable: *const SERVICE_TABLE_ENTRYA) -> super::super::Foundation::BOOL {
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn StartServiceCtrlDispatcherA(lpservicestarttable: *const SERVICE_TABLE_ENTRYA) -> super::super::Foundation::BOOL;
    }
    StartServiceCtrlDispatcherA(::core::mem::transmute(lpservicestarttable))
}
#[doc = "*Required features: `\"Win32_System_Services\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn StartServiceCtrlDispatcherW(lpservicestarttable: *const SERVICE_TABLE_ENTRYW) -> super::super::Foundation::BOOL {
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn StartServiceCtrlDispatcherW(lpservicestarttable: *const SERVICE_TABLE_ENTRYW) -> super::super::Foundation::BOOL;
    }
    StartServiceCtrlDispatcherW(::core::mem::transmute(lpservicestarttable))
}
#[doc = "*Required features: `\"Win32_System_Services\"`, `\"Win32_Foundation\"`, `\"Win32_Security\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
#[inline]
pub unsafe fn StartServiceW<'a, P0>(hservice: P0, lpserviceargvectors: &[::windows::core::PWSTR]) -> super::super::Foundation::BOOL
where
    P0: ::std::convert::Into<super::super::Security::SC_HANDLE>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn StartServiceW(hservice: super::super::Security::SC_HANDLE, dwnumserviceargs: u32, lpserviceargvectors: *const ::windows::core::PWSTR) -> super::super::Foundation::BOOL;
    }
    StartServiceW(hservice.into(), lpserviceargvectors.len() as _, ::core::mem::transmute(::windows::core::as_ptr_or_null(lpserviceargvectors)))
}
pub const USER_POLICY_PRESENT_GUID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x54fb46c8_f089_464c_b1fd_59d1b62c3b50);
#[doc = "*Required features: `\"Win32_System_Services\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn UnlockServiceDatabase(sclock: *const ::core::ffi::c_void) -> super::super::Foundation::BOOL {
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn UnlockServiceDatabase(sclock: *const ::core::ffi::c_void) -> super::super::Foundation::BOOL;
    }
    UnlockServiceDatabase(::core::mem::transmute(sclock))
}
#[doc = "*Required features: `\"Win32_System_Services\"`, `\"Win32_Foundation\"`, `\"Win32_Security\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
#[inline]
pub unsafe fn WaitServiceState<'a, P0, P1>(hservice: P0, dwnotify: u32, dwtimeout: u32, hcancelevent: P1) -> u32
where
    P0: ::std::convert::Into<super::super::Security::SC_HANDLE>,
    P1: ::std::convert::Into<super::super::Foundation::HANDLE>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn WaitServiceState(hservice: super::super::Security::SC_HANDLE, dwnotify: u32, dwtimeout: u32, hcancelevent: super::super::Foundation::HANDLE) -> u32;
    }
    WaitServiceState(hservice.into(), dwnotify, dwtimeout, hcancelevent.into())
}
#[repr(C)]
pub struct _SC_NOTIFICATION_REGISTRATION(pub u8);
#[cfg(feature = "implement")]
::core::include!("impl.rs");

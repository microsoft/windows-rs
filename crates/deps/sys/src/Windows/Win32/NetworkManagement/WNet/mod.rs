#![allow(non_snake_case, non_camel_case_types)]
#[link(name = "windows")]
extern "system" {
    #[doc = "*Required features: `Win32_NetworkManagement_WNet`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn MultinetGetConnectionPerformanceA(lpnetresource: *const NETRESOURCEA, lpnetconnectinfostruct: *mut NETCONNECTINFOSTRUCT) -> u32;
    #[doc = "*Required features: `Win32_NetworkManagement_WNet`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn MultinetGetConnectionPerformanceW(lpnetresource: *const NETRESOURCEW, lpnetconnectinfostruct: *mut NETCONNECTINFOSTRUCT) -> u32;
    #[doc = "*Required features: `Win32_NetworkManagement_WNet`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn NPAddConnection(lpnetresource: *const NETRESOURCEW, lppassword: super::super::Foundation::PWSTR, lpusername: super::super::Foundation::PWSTR) -> u32;
    #[doc = "*Required features: `Win32_NetworkManagement_WNet`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn NPAddConnection3(hwndowner: super::super::Foundation::HWND, lpnetresource: *const NETRESOURCEW, lppassword: super::super::Foundation::PWSTR, lpusername: super::super::Foundation::PWSTR, dwflags: NET_USE_CONNECT_FLAGS) -> u32;
    #[doc = "*Required features: `Win32_NetworkManagement_WNet`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn NPAddConnection4(hwndowner: super::super::Foundation::HWND, lpnetresource: *const NETRESOURCEW, lpauthbuffer: *const ::core::ffi::c_void, cbauthbuffer: u32, dwflags: u32, lpuseoptions: *const u8, cbuseoptions: u32) -> u32;
    #[doc = "*Required features: `Win32_NetworkManagement_WNet`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn NPCancelConnection(lpname: super::super::Foundation::PWSTR, fforce: super::super::Foundation::BOOL) -> u32;
    #[doc = "*Required features: `Win32_NetworkManagement_WNet`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn NPCancelConnection2(lpname: super::super::Foundation::PWSTR, fforce: super::super::Foundation::BOOL, dwflags: u32) -> u32;
    #[doc = "*Required features: `Win32_NetworkManagement_WNet`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn NPCloseEnum(henum: super::super::Foundation::HANDLE) -> u32;
    #[doc = "*Required features: `Win32_NetworkManagement_WNet`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn NPEnumResource(henum: super::super::Foundation::HANDLE, lpccount: *mut u32, lpbuffer: *mut ::core::ffi::c_void, lpbuffersize: *mut u32) -> u32;
    #[doc = "*Required features: `Win32_NetworkManagement_WNet`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn NPFormatNetworkName(lpremotename: super::super::Foundation::PWSTR, lpformattedname: super::super::Foundation::PWSTR, lpnlength: *mut u32, dwflags: NETWORK_NAME_FORMAT_FLAGS, dwavecharperline: u32) -> u32;
    #[doc = "*Required features: `Win32_NetworkManagement_WNet`*"]
    pub fn NPGetCaps(ndex: u32) -> u32;
    #[doc = "*Required features: `Win32_NetworkManagement_WNet`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn NPGetConnection(lplocalname: super::super::Foundation::PWSTR, lpremotename: super::super::Foundation::PWSTR, lpnbufferlen: *mut u32) -> u32;
    #[doc = "*Required features: `Win32_NetworkManagement_WNet`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn NPGetConnection3(lplocalname: super::super::Foundation::PWSTR, dwlevel: u32, lpbuffer: *mut ::core::ffi::c_void, lpbuffersize: *mut u32) -> u32;
    #[doc = "*Required features: `Win32_NetworkManagement_WNet`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn NPGetConnectionPerformance(lpremotename: super::super::Foundation::PWSTR, lpnetconnectinfo: *mut NETCONNECTINFOSTRUCT) -> u32;
    #[doc = "*Required features: `Win32_NetworkManagement_WNet`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn NPGetPersistentUseOptionsForConnection(lpremotepath: super::super::Foundation::PWSTR, lpreaduseoptions: *const u8, cbreaduseoptions: u32, lpwriteuseoptions: *mut u8, lpsizewriteuseoptions: *mut u32) -> u32;
    #[doc = "*Required features: `Win32_NetworkManagement_WNet`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn NPGetResourceInformation(lpnetresource: *const NETRESOURCEW, lpbuffer: *mut ::core::ffi::c_void, lpbuffersize: *mut u32, lplpsystem: *mut super::super::Foundation::PWSTR) -> u32;
    #[doc = "*Required features: `Win32_NetworkManagement_WNet`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn NPGetResourceParent(lpnetresource: *const NETRESOURCEW, lpbuffer: *mut ::core::ffi::c_void, lpbuffersize: *mut u32) -> u32;
    #[doc = "*Required features: `Win32_NetworkManagement_WNet`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn NPGetUniversalName(lplocalpath: super::super::Foundation::PWSTR, dwinfolevel: UNC_INFO_LEVEL, lpbuffer: *mut ::core::ffi::c_void, lpbuffersize: *mut u32) -> u32;
    #[doc = "*Required features: `Win32_NetworkManagement_WNet`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn NPGetUser(lpname: super::super::Foundation::PWSTR, lpusername: super::super::Foundation::PWSTR, lpnbufferlen: *mut u32) -> u32;
    #[doc = "*Required features: `Win32_NetworkManagement_WNet`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn NPOpenEnum(dwscope: u32, dwtype: u32, dwusage: u32, lpnetresource: *const NETRESOURCEW, lphenum: *mut super::super::Foundation::HANDLE) -> u32;
    #[doc = "*Required features: `Win32_NetworkManagement_WNet`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WNetAddConnection2A(lpnetresource: *const NETRESOURCEA, lppassword: super::super::Foundation::PSTR, lpusername: super::super::Foundation::PSTR, dwflags: u32) -> u32;
    #[doc = "*Required features: `Win32_NetworkManagement_WNet`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WNetAddConnection2W(lpnetresource: *const NETRESOURCEW, lppassword: super::super::Foundation::PWSTR, lpusername: super::super::Foundation::PWSTR, dwflags: u32) -> u32;
    #[doc = "*Required features: `Win32_NetworkManagement_WNet`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WNetAddConnection3A(hwndowner: super::super::Foundation::HWND, lpnetresource: *const NETRESOURCEA, lppassword: super::super::Foundation::PSTR, lpusername: super::super::Foundation::PSTR, dwflags: u32) -> u32;
    #[doc = "*Required features: `Win32_NetworkManagement_WNet`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WNetAddConnection3W(hwndowner: super::super::Foundation::HWND, lpnetresource: *const NETRESOURCEW, lppassword: super::super::Foundation::PWSTR, lpusername: super::super::Foundation::PWSTR, dwflags: u32) -> u32;
    #[doc = "*Required features: `Win32_NetworkManagement_WNet`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WNetAddConnection4A(hwndowner: super::super::Foundation::HWND, lpnetresource: *const NETRESOURCEA, pauthbuffer: *const ::core::ffi::c_void, cbauthbuffer: u32, dwflags: u32, lpuseoptions: *const u8, cbuseoptions: u32) -> u32;
    #[doc = "*Required features: `Win32_NetworkManagement_WNet`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WNetAddConnection4W(hwndowner: super::super::Foundation::HWND, lpnetresource: *const NETRESOURCEW, pauthbuffer: *const ::core::ffi::c_void, cbauthbuffer: u32, dwflags: u32, lpuseoptions: *const u8, cbuseoptions: u32) -> u32;
    #[doc = "*Required features: `Win32_NetworkManagement_WNet`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WNetAddConnectionA(lpremotename: super::super::Foundation::PSTR, lppassword: super::super::Foundation::PSTR, lplocalname: super::super::Foundation::PSTR) -> u32;
    #[doc = "*Required features: `Win32_NetworkManagement_WNet`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WNetAddConnectionW(lpremotename: super::super::Foundation::PWSTR, lppassword: super::super::Foundation::PWSTR, lplocalname: super::super::Foundation::PWSTR) -> u32;
    #[doc = "*Required features: `Win32_NetworkManagement_WNet`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WNetCancelConnection2A(lpname: super::super::Foundation::PSTR, dwflags: u32, fforce: super::super::Foundation::BOOL) -> u32;
    #[doc = "*Required features: `Win32_NetworkManagement_WNet`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WNetCancelConnection2W(lpname: super::super::Foundation::PWSTR, dwflags: u32, fforce: super::super::Foundation::BOOL) -> u32;
    #[doc = "*Required features: `Win32_NetworkManagement_WNet`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WNetCancelConnectionA(lpname: super::super::Foundation::PSTR, fforce: super::super::Foundation::BOOL) -> u32;
    #[doc = "*Required features: `Win32_NetworkManagement_WNet`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WNetCancelConnectionW(lpname: super::super::Foundation::PWSTR, fforce: super::super::Foundation::BOOL) -> u32;
    #[doc = "*Required features: `Win32_NetworkManagement_WNet`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WNetCloseEnum(henum: super::super::Foundation::HANDLE) -> u32;
    #[doc = "*Required features: `Win32_NetworkManagement_WNet`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WNetConnectionDialog(hwnd: super::super::Foundation::HWND, dwtype: u32) -> u32;
    #[doc = "*Required features: `Win32_NetworkManagement_WNet`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WNetConnectionDialog1A(lpconndlgstruct: *mut CONNECTDLGSTRUCTA) -> u32;
    #[doc = "*Required features: `Win32_NetworkManagement_WNet`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WNetConnectionDialog1W(lpconndlgstruct: *mut CONNECTDLGSTRUCTW) -> u32;
    #[doc = "*Required features: `Win32_NetworkManagement_WNet`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WNetDisconnectDialog(hwnd: super::super::Foundation::HWND, dwtype: u32) -> u32;
    #[doc = "*Required features: `Win32_NetworkManagement_WNet`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WNetDisconnectDialog1A(lpconndlgstruct: *const DISCDLGSTRUCTA) -> u32;
    #[doc = "*Required features: `Win32_NetworkManagement_WNet`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WNetDisconnectDialog1W(lpconndlgstruct: *const DISCDLGSTRUCTW) -> u32;
    #[doc = "*Required features: `Win32_NetworkManagement_WNet`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WNetEnumResourceA(henum: super::super::Foundation::HANDLE, lpccount: *mut u32, lpbuffer: *mut ::core::ffi::c_void, lpbuffersize: *mut u32) -> u32;
    #[doc = "*Required features: `Win32_NetworkManagement_WNet`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WNetEnumResourceW(henum: super::super::Foundation::HANDLE, lpccount: *mut u32, lpbuffer: *mut ::core::ffi::c_void, lpbuffersize: *mut u32) -> u32;
    #[doc = "*Required features: `Win32_NetworkManagement_WNet`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WNetGetConnectionA(lplocalname: super::super::Foundation::PSTR, lpremotename: super::super::Foundation::PSTR, lpnlength: *mut u32) -> u32;
    #[doc = "*Required features: `Win32_NetworkManagement_WNet`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WNetGetConnectionW(lplocalname: super::super::Foundation::PWSTR, lpremotename: super::super::Foundation::PWSTR, lpnlength: *mut u32) -> u32;
    #[doc = "*Required features: `Win32_NetworkManagement_WNet`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WNetGetLastErrorA(lperror: *mut u32, lperrorbuf: super::super::Foundation::PSTR, nerrorbufsize: u32, lpnamebuf: super::super::Foundation::PSTR, nnamebufsize: u32) -> u32;
    #[doc = "*Required features: `Win32_NetworkManagement_WNet`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WNetGetLastErrorW(lperror: *mut u32, lperrorbuf: super::super::Foundation::PWSTR, nerrorbufsize: u32, lpnamebuf: super::super::Foundation::PWSTR, nnamebufsize: u32) -> u32;
    #[doc = "*Required features: `Win32_NetworkManagement_WNet`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WNetGetNetworkInformationA(lpprovider: super::super::Foundation::PSTR, lpnetinfostruct: *mut NETINFOSTRUCT) -> u32;
    #[doc = "*Required features: `Win32_NetworkManagement_WNet`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WNetGetNetworkInformationW(lpprovider: super::super::Foundation::PWSTR, lpnetinfostruct: *mut NETINFOSTRUCT) -> u32;
    #[doc = "*Required features: `Win32_NetworkManagement_WNet`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WNetGetProviderNameA(dwnettype: u32, lpprovidername: super::super::Foundation::PSTR, lpbuffersize: *mut u32) -> u32;
    #[doc = "*Required features: `Win32_NetworkManagement_WNet`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WNetGetProviderNameW(dwnettype: u32, lpprovidername: super::super::Foundation::PWSTR, lpbuffersize: *mut u32) -> u32;
    #[doc = "*Required features: `Win32_NetworkManagement_WNet`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WNetGetResourceInformationA(lpnetresource: *const NETRESOURCEA, lpbuffer: *mut ::core::ffi::c_void, lpcbbuffer: *mut u32, lplpsystem: *mut super::super::Foundation::PSTR) -> u32;
    #[doc = "*Required features: `Win32_NetworkManagement_WNet`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WNetGetResourceInformationW(lpnetresource: *const NETRESOURCEW, lpbuffer: *mut ::core::ffi::c_void, lpcbbuffer: *mut u32, lplpsystem: *mut super::super::Foundation::PWSTR) -> u32;
    #[doc = "*Required features: `Win32_NetworkManagement_WNet`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WNetGetResourceParentA(lpnetresource: *const NETRESOURCEA, lpbuffer: *mut ::core::ffi::c_void, lpcbbuffer: *mut u32) -> u32;
    #[doc = "*Required features: `Win32_NetworkManagement_WNet`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WNetGetResourceParentW(lpnetresource: *const NETRESOURCEW, lpbuffer: *mut ::core::ffi::c_void, lpcbbuffer: *mut u32) -> u32;
    #[doc = "*Required features: `Win32_NetworkManagement_WNet`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WNetGetUniversalNameA(lplocalpath: super::super::Foundation::PSTR, dwinfolevel: UNC_INFO_LEVEL, lpbuffer: *mut ::core::ffi::c_void, lpbuffersize: *mut u32) -> u32;
    #[doc = "*Required features: `Win32_NetworkManagement_WNet`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WNetGetUniversalNameW(lplocalpath: super::super::Foundation::PWSTR, dwinfolevel: UNC_INFO_LEVEL, lpbuffer: *mut ::core::ffi::c_void, lpbuffersize: *mut u32) -> u32;
    #[doc = "*Required features: `Win32_NetworkManagement_WNet`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WNetGetUserA(lpname: super::super::Foundation::PSTR, lpusername: super::super::Foundation::PSTR, lpnlength: *mut u32) -> u32;
    #[doc = "*Required features: `Win32_NetworkManagement_WNet`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WNetGetUserW(lpname: super::super::Foundation::PWSTR, lpusername: super::super::Foundation::PWSTR, lpnlength: *mut u32) -> u32;
    #[doc = "*Required features: `Win32_NetworkManagement_WNet`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WNetOpenEnumA(dwscope: NET_RESOURCE_SCOPE, dwtype: NET_RESOURCE_TYPE, dwusage: WNET_OPEN_ENUM_USAGE, lpnetresource: *const NETRESOURCEA, lphenum: *mut NetEnumHandle) -> u32;
    #[doc = "*Required features: `Win32_NetworkManagement_WNet`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WNetOpenEnumW(dwscope: NET_RESOURCE_SCOPE, dwtype: NET_RESOURCE_TYPE, dwusage: WNET_OPEN_ENUM_USAGE, lpnetresource: *const NETRESOURCEW, lphenum: *mut NetEnumHandle) -> u32;
    #[doc = "*Required features: `Win32_NetworkManagement_WNet`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WNetSetLastErrorA(err: u32, lperror: super::super::Foundation::PSTR, lpproviders: super::super::Foundation::PSTR);
    #[doc = "*Required features: `Win32_NetworkManagement_WNet`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WNetSetLastErrorW(err: u32, lperror: super::super::Foundation::PWSTR, lpproviders: super::super::Foundation::PWSTR);
    #[doc = "*Required features: `Win32_NetworkManagement_WNet`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WNetUseConnection4A(hwndowner: super::super::Foundation::HWND, lpnetresource: *const NETRESOURCEA, pauthbuffer: *const ::core::ffi::c_void, cbauthbuffer: u32, dwflags: u32, lpuseoptions: *const u8, cbuseoptions: u32, lpaccessname: super::super::Foundation::PSTR, lpbuffersize: *mut u32, lpresult: *mut u32) -> u32;
    #[doc = "*Required features: `Win32_NetworkManagement_WNet`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WNetUseConnection4W(hwndowner: super::super::Foundation::HWND, lpnetresource: *const NETRESOURCEW, pauthbuffer: *const ::core::ffi::c_void, cbauthbuffer: u32, dwflags: u32, lpuseoptions: *const u8, cbuseoptions: u32, lpaccessname: super::super::Foundation::PWSTR, lpbuffersize: *mut u32, lpresult: *mut u32) -> u32;
    #[doc = "*Required features: `Win32_NetworkManagement_WNet`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WNetUseConnectionA(hwndowner: super::super::Foundation::HWND, lpnetresource: *const NETRESOURCEA, lppassword: super::super::Foundation::PSTR, lpuserid: super::super::Foundation::PSTR, dwflags: NET_USE_CONNECT_FLAGS, lpaccessname: super::super::Foundation::PSTR, lpbuffersize: *mut u32, lpresult: *mut u32) -> u32;
    #[doc = "*Required features: `Win32_NetworkManagement_WNet`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WNetUseConnectionW(hwndowner: super::super::Foundation::HWND, lpnetresource: *const NETRESOURCEW, lppassword: super::super::Foundation::PWSTR, lpuserid: super::super::Foundation::PWSTR, dwflags: NET_USE_CONNECT_FLAGS, lpaccessname: super::super::Foundation::PWSTR, lpbuffersize: *mut u32, lpresult: *mut u32) -> u32;
}

#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, clashing_extern_declarations, clippy::all)]
#[link(name = "windows")]
extern "system" {
    #[doc = "*Required features: 'Win32_NetworkManagement_WNet', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn MultinetGetConnectionPerformanceA(lpnetresource: *const NETRESOURCEA, lpnetconnectinfostruct: *mut NETCONNECTINFOSTRUCT) -> u32;
    #[doc = "*Required features: 'Win32_NetworkManagement_WNet', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn MultinetGetConnectionPerformanceW(lpnetresource: *const NETRESOURCEW, lpnetconnectinfostruct: *mut NETCONNECTINFOSTRUCT) -> u32;
    #[doc = "*Required features: 'Win32_NetworkManagement_WNet', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn NPAddConnection(lpnetresource: *const NETRESOURCEW, lppassword: super::super::Foundation::PWSTR, lpusername: super::super::Foundation::PWSTR) -> u32;
    #[doc = "*Required features: 'Win32_NetworkManagement_WNet', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn NPAddConnection3(hwndowner: super::super::Foundation::HWND, lpnetresource: *const NETRESOURCEW, lppassword: super::super::Foundation::PWSTR, lpusername: super::super::Foundation::PWSTR, dwflags: NET_USE_CONNECT_FLAGS) -> u32;
    #[doc = "*Required features: 'Win32_NetworkManagement_WNet', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn NPAddConnection4(hwndowner: super::super::Foundation::HWND, lpnetresource: *const NETRESOURCEW, lpauthbuffer: *const ::core::ffi::c_void, cbauthbuffer: u32, dwflags: u32, lpuseoptions: *const u8, cbuseoptions: u32) -> u32;
    #[doc = "*Required features: 'Win32_NetworkManagement_WNet', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn NPCancelConnection(lpname: super::super::Foundation::PWSTR, fforce: super::super::Foundation::BOOL) -> u32;
    #[doc = "*Required features: 'Win32_NetworkManagement_WNet', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn NPCancelConnection2(lpname: super::super::Foundation::PWSTR, fforce: super::super::Foundation::BOOL, dwflags: u32) -> u32;
    #[doc = "*Required features: 'Win32_NetworkManagement_WNet', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn NPCloseEnum(henum: super::super::Foundation::HANDLE) -> u32;
    #[doc = "*Required features: 'Win32_NetworkManagement_WNet', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn NPEnumResource(henum: super::super::Foundation::HANDLE, lpccount: *mut u32, lpbuffer: *mut ::core::ffi::c_void, lpbuffersize: *mut u32) -> u32;
    #[doc = "*Required features: 'Win32_NetworkManagement_WNet', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn NPFormatNetworkName(lpremotename: super::super::Foundation::PWSTR, lpformattedname: super::super::Foundation::PWSTR, lpnlength: *mut u32, dwflags: NETWORK_NAME_FORMAT_FLAGS, dwavecharperline: u32) -> u32;
    #[doc = "*Required features: 'Win32_NetworkManagement_WNet'*"]
    pub fn NPGetCaps(ndex: u32) -> u32;
    #[doc = "*Required features: 'Win32_NetworkManagement_WNet', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn NPGetConnection(lplocalname: super::super::Foundation::PWSTR, lpremotename: super::super::Foundation::PWSTR, lpnbufferlen: *mut u32) -> u32;
    #[doc = "*Required features: 'Win32_NetworkManagement_WNet', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn NPGetConnection3(lplocalname: super::super::Foundation::PWSTR, dwlevel: u32, lpbuffer: *mut ::core::ffi::c_void, lpbuffersize: *mut u32) -> u32;
    #[doc = "*Required features: 'Win32_NetworkManagement_WNet', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn NPGetConnectionPerformance(lpremotename: super::super::Foundation::PWSTR, lpnetconnectinfo: *mut NETCONNECTINFOSTRUCT) -> u32;
    #[doc = "*Required features: 'Win32_NetworkManagement_WNet', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn NPGetPersistentUseOptionsForConnection(lpremotepath: super::super::Foundation::PWSTR, lpreaduseoptions: *const u8, cbreaduseoptions: u32, lpwriteuseoptions: *mut u8, lpsizewriteuseoptions: *mut u32) -> u32;
    #[doc = "*Required features: 'Win32_NetworkManagement_WNet', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn NPGetResourceInformation(lpnetresource: *const NETRESOURCEW, lpbuffer: *mut ::core::ffi::c_void, lpbuffersize: *mut u32, lplpsystem: *mut super::super::Foundation::PWSTR) -> u32;
    #[doc = "*Required features: 'Win32_NetworkManagement_WNet', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn NPGetResourceParent(lpnetresource: *const NETRESOURCEW, lpbuffer: *mut ::core::ffi::c_void, lpbuffersize: *mut u32) -> u32;
    #[doc = "*Required features: 'Win32_NetworkManagement_WNet', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn NPGetUniversalName(lplocalpath: super::super::Foundation::PWSTR, dwinfolevel: UNC_INFO_LEVEL, lpbuffer: *mut ::core::ffi::c_void, lpbuffersize: *mut u32) -> u32;
    #[doc = "*Required features: 'Win32_NetworkManagement_WNet', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn NPGetUser(lpname: super::super::Foundation::PWSTR, lpusername: super::super::Foundation::PWSTR, lpnbufferlen: *mut u32) -> u32;
    #[doc = "*Required features: 'Win32_NetworkManagement_WNet', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn NPOpenEnum(dwscope: u32, dwtype: u32, dwusage: u32, lpnetresource: *const NETRESOURCEW, lphenum: *mut super::super::Foundation::HANDLE) -> u32;
    #[doc = "*Required features: 'Win32_NetworkManagement_WNet', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WNetAddConnection2A(lpnetresource: *const NETRESOURCEA, lppassword: super::super::Foundation::PSTR, lpusername: super::super::Foundation::PSTR, dwflags: u32) -> u32;
    #[doc = "*Required features: 'Win32_NetworkManagement_WNet', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WNetAddConnection2W(lpnetresource: *const NETRESOURCEW, lppassword: super::super::Foundation::PWSTR, lpusername: super::super::Foundation::PWSTR, dwflags: u32) -> u32;
    #[doc = "*Required features: 'Win32_NetworkManagement_WNet', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WNetAddConnection3A(hwndowner: super::super::Foundation::HWND, lpnetresource: *const NETRESOURCEA, lppassword: super::super::Foundation::PSTR, lpusername: super::super::Foundation::PSTR, dwflags: u32) -> u32;
    #[doc = "*Required features: 'Win32_NetworkManagement_WNet', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WNetAddConnection3W(hwndowner: super::super::Foundation::HWND, lpnetresource: *const NETRESOURCEW, lppassword: super::super::Foundation::PWSTR, lpusername: super::super::Foundation::PWSTR, dwflags: u32) -> u32;
    #[doc = "*Required features: 'Win32_NetworkManagement_WNet', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WNetAddConnection4A(hwndowner: super::super::Foundation::HWND, lpnetresource: *const NETRESOURCEA, pauthbuffer: *const ::core::ffi::c_void, cbauthbuffer: u32, dwflags: u32, lpuseoptions: *const u8, cbuseoptions: u32) -> u32;
    #[doc = "*Required features: 'Win32_NetworkManagement_WNet', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WNetAddConnection4W(hwndowner: super::super::Foundation::HWND, lpnetresource: *const NETRESOURCEW, pauthbuffer: *const ::core::ffi::c_void, cbauthbuffer: u32, dwflags: u32, lpuseoptions: *const u8, cbuseoptions: u32) -> u32;
    #[doc = "*Required features: 'Win32_NetworkManagement_WNet', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WNetAddConnectionA(lpremotename: super::super::Foundation::PSTR, lppassword: super::super::Foundation::PSTR, lplocalname: super::super::Foundation::PSTR) -> u32;
    #[doc = "*Required features: 'Win32_NetworkManagement_WNet', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WNetAddConnectionW(lpremotename: super::super::Foundation::PWSTR, lppassword: super::super::Foundation::PWSTR, lplocalname: super::super::Foundation::PWSTR) -> u32;
    #[doc = "*Required features: 'Win32_NetworkManagement_WNet', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WNetCancelConnection2A(lpname: super::super::Foundation::PSTR, dwflags: u32, fforce: super::super::Foundation::BOOL) -> u32;
    #[doc = "*Required features: 'Win32_NetworkManagement_WNet', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WNetCancelConnection2W(lpname: super::super::Foundation::PWSTR, dwflags: u32, fforce: super::super::Foundation::BOOL) -> u32;
    #[doc = "*Required features: 'Win32_NetworkManagement_WNet', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WNetCancelConnectionA(lpname: super::super::Foundation::PSTR, fforce: super::super::Foundation::BOOL) -> u32;
    #[doc = "*Required features: 'Win32_NetworkManagement_WNet', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WNetCancelConnectionW(lpname: super::super::Foundation::PWSTR, fforce: super::super::Foundation::BOOL) -> u32;
    #[doc = "*Required features: 'Win32_NetworkManagement_WNet', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WNetCloseEnum(henum: super::super::Foundation::HANDLE) -> u32;
    #[doc = "*Required features: 'Win32_NetworkManagement_WNet', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WNetConnectionDialog(hwnd: super::super::Foundation::HWND, dwtype: u32) -> u32;
    #[doc = "*Required features: 'Win32_NetworkManagement_WNet', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WNetConnectionDialog1A(lpconndlgstruct: *mut CONNECTDLGSTRUCTA) -> u32;
    #[doc = "*Required features: 'Win32_NetworkManagement_WNet', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WNetConnectionDialog1W(lpconndlgstruct: *mut CONNECTDLGSTRUCTW) -> u32;
    #[doc = "*Required features: 'Win32_NetworkManagement_WNet', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WNetDisconnectDialog(hwnd: super::super::Foundation::HWND, dwtype: u32) -> u32;
    #[doc = "*Required features: 'Win32_NetworkManagement_WNet', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WNetDisconnectDialog1A(lpconndlgstruct: *const DISCDLGSTRUCTA) -> u32;
    #[doc = "*Required features: 'Win32_NetworkManagement_WNet', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WNetDisconnectDialog1W(lpconndlgstruct: *const DISCDLGSTRUCTW) -> u32;
    #[doc = "*Required features: 'Win32_NetworkManagement_WNet', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WNetEnumResourceA(henum: super::super::Foundation::HANDLE, lpccount: *mut u32, lpbuffer: *mut ::core::ffi::c_void, lpbuffersize: *mut u32) -> u32;
    #[doc = "*Required features: 'Win32_NetworkManagement_WNet', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WNetEnumResourceW(henum: super::super::Foundation::HANDLE, lpccount: *mut u32, lpbuffer: *mut ::core::ffi::c_void, lpbuffersize: *mut u32) -> u32;
    #[doc = "*Required features: 'Win32_NetworkManagement_WNet', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WNetGetConnectionA(lplocalname: super::super::Foundation::PSTR, lpremotename: super::super::Foundation::PSTR, lpnlength: *mut u32) -> u32;
    #[doc = "*Required features: 'Win32_NetworkManagement_WNet', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WNetGetConnectionW(lplocalname: super::super::Foundation::PWSTR, lpremotename: super::super::Foundation::PWSTR, lpnlength: *mut u32) -> u32;
    #[doc = "*Required features: 'Win32_NetworkManagement_WNet', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WNetGetLastErrorA(lperror: *mut u32, lperrorbuf: super::super::Foundation::PSTR, nerrorbufsize: u32, lpnamebuf: super::super::Foundation::PSTR, nnamebufsize: u32) -> u32;
    #[doc = "*Required features: 'Win32_NetworkManagement_WNet', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WNetGetLastErrorW(lperror: *mut u32, lperrorbuf: super::super::Foundation::PWSTR, nerrorbufsize: u32, lpnamebuf: super::super::Foundation::PWSTR, nnamebufsize: u32) -> u32;
    #[doc = "*Required features: 'Win32_NetworkManagement_WNet', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WNetGetNetworkInformationA(lpprovider: super::super::Foundation::PSTR, lpnetinfostruct: *mut NETINFOSTRUCT) -> u32;
    #[doc = "*Required features: 'Win32_NetworkManagement_WNet', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WNetGetNetworkInformationW(lpprovider: super::super::Foundation::PWSTR, lpnetinfostruct: *mut NETINFOSTRUCT) -> u32;
    #[doc = "*Required features: 'Win32_NetworkManagement_WNet', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WNetGetProviderNameA(dwnettype: u32, lpprovidername: super::super::Foundation::PSTR, lpbuffersize: *mut u32) -> u32;
    #[doc = "*Required features: 'Win32_NetworkManagement_WNet', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WNetGetProviderNameW(dwnettype: u32, lpprovidername: super::super::Foundation::PWSTR, lpbuffersize: *mut u32) -> u32;
    #[doc = "*Required features: 'Win32_NetworkManagement_WNet', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WNetGetResourceInformationA(lpnetresource: *const NETRESOURCEA, lpbuffer: *mut ::core::ffi::c_void, lpcbbuffer: *mut u32, lplpsystem: *mut super::super::Foundation::PSTR) -> u32;
    #[doc = "*Required features: 'Win32_NetworkManagement_WNet', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WNetGetResourceInformationW(lpnetresource: *const NETRESOURCEW, lpbuffer: *mut ::core::ffi::c_void, lpcbbuffer: *mut u32, lplpsystem: *mut super::super::Foundation::PWSTR) -> u32;
    #[doc = "*Required features: 'Win32_NetworkManagement_WNet', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WNetGetResourceParentA(lpnetresource: *const NETRESOURCEA, lpbuffer: *mut ::core::ffi::c_void, lpcbbuffer: *mut u32) -> u32;
    #[doc = "*Required features: 'Win32_NetworkManagement_WNet', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WNetGetResourceParentW(lpnetresource: *const NETRESOURCEW, lpbuffer: *mut ::core::ffi::c_void, lpcbbuffer: *mut u32) -> u32;
    #[doc = "*Required features: 'Win32_NetworkManagement_WNet', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WNetGetUniversalNameA(lplocalpath: super::super::Foundation::PSTR, dwinfolevel: UNC_INFO_LEVEL, lpbuffer: *mut ::core::ffi::c_void, lpbuffersize: *mut u32) -> u32;
    #[doc = "*Required features: 'Win32_NetworkManagement_WNet', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WNetGetUniversalNameW(lplocalpath: super::super::Foundation::PWSTR, dwinfolevel: UNC_INFO_LEVEL, lpbuffer: *mut ::core::ffi::c_void, lpbuffersize: *mut u32) -> u32;
    #[doc = "*Required features: 'Win32_NetworkManagement_WNet', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WNetGetUserA(lpname: super::super::Foundation::PSTR, lpusername: super::super::Foundation::PSTR, lpnlength: *mut u32) -> u32;
    #[doc = "*Required features: 'Win32_NetworkManagement_WNet', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WNetGetUserW(lpname: super::super::Foundation::PWSTR, lpusername: super::super::Foundation::PWSTR, lpnlength: *mut u32) -> u32;
    #[doc = "*Required features: 'Win32_NetworkManagement_WNet', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WNetOpenEnumA(dwscope: NET_RESOURCE_SCOPE, dwtype: NET_RESOURCE_TYPE, dwusage: WNET_OPEN_ENUM_USAGE, lpnetresource: *const NETRESOURCEA, lphenum: *mut NetEnumHandle) -> u32;
    #[doc = "*Required features: 'Win32_NetworkManagement_WNet', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WNetOpenEnumW(dwscope: NET_RESOURCE_SCOPE, dwtype: NET_RESOURCE_TYPE, dwusage: WNET_OPEN_ENUM_USAGE, lpnetresource: *const NETRESOURCEW, lphenum: *mut NetEnumHandle) -> u32;
    #[doc = "*Required features: 'Win32_NetworkManagement_WNet', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WNetSetLastErrorA(err: u32, lperror: super::super::Foundation::PSTR, lpproviders: super::super::Foundation::PSTR);
    #[doc = "*Required features: 'Win32_NetworkManagement_WNet', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WNetSetLastErrorW(err: u32, lperror: super::super::Foundation::PWSTR, lpproviders: super::super::Foundation::PWSTR);
    #[doc = "*Required features: 'Win32_NetworkManagement_WNet', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WNetUseConnection4A(hwndowner: super::super::Foundation::HWND, lpnetresource: *const NETRESOURCEA, pauthbuffer: *const ::core::ffi::c_void, cbauthbuffer: u32, dwflags: u32, lpuseoptions: *const u8, cbuseoptions: u32, lpaccessname: super::super::Foundation::PSTR, lpbuffersize: *mut u32, lpresult: *mut u32) -> u32;
    #[doc = "*Required features: 'Win32_NetworkManagement_WNet', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WNetUseConnection4W(hwndowner: super::super::Foundation::HWND, lpnetresource: *const NETRESOURCEW, pauthbuffer: *const ::core::ffi::c_void, cbauthbuffer: u32, dwflags: u32, lpuseoptions: *const u8, cbuseoptions: u32, lpaccessname: super::super::Foundation::PWSTR, lpbuffersize: *mut u32, lpresult: *mut u32) -> u32;
    #[doc = "*Required features: 'Win32_NetworkManagement_WNet', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WNetUseConnectionA(hwndowner: super::super::Foundation::HWND, lpnetresource: *const NETRESOURCEA, lppassword: super::super::Foundation::PSTR, lpuserid: super::super::Foundation::PSTR, dwflags: NET_USE_CONNECT_FLAGS, lpaccessname: super::super::Foundation::PSTR, lpbuffersize: *mut u32, lpresult: *mut u32) -> u32;
    #[doc = "*Required features: 'Win32_NetworkManagement_WNet', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WNetUseConnectionW(hwndowner: super::super::Foundation::HWND, lpnetresource: *const NETRESOURCEW, lppassword: super::super::Foundation::PWSTR, lpuserid: super::super::Foundation::PWSTR, dwflags: NET_USE_CONNECT_FLAGS, lpaccessname: super::super::Foundation::PWSTR, lpbuffersize: *mut u32, lpresult: *mut u32) -> u32;
}
#[repr(C)]
#[doc = "*Required features: 'Win32_NetworkManagement_WNet', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
pub struct CONNECTDLGSTRUCTA {
    pub cbStructure: u32,
    pub hwndOwner: super::super::Foundation::HWND,
    pub lpConnRes: *mut NETRESOURCEA,
    pub dwFlags: CONNECTDLGSTRUCT_FLAGS,
    pub dwDevNum: u32,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for CONNECTDLGSTRUCTA {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for CONNECTDLGSTRUCTA {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: 'Win32_NetworkManagement_WNet', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
pub struct CONNECTDLGSTRUCTW {
    pub cbStructure: u32,
    pub hwndOwner: super::super::Foundation::HWND,
    pub lpConnRes: *mut NETRESOURCEW,
    pub dwFlags: CONNECTDLGSTRUCT_FLAGS,
    pub dwDevNum: u32,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for CONNECTDLGSTRUCTW {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for CONNECTDLGSTRUCTW {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: 'Win32_NetworkManagement_WNet'*"]
pub type CONNECTDLGSTRUCT_FLAGS = u32;
#[doc = "*Required features: 'Win32_NetworkManagement_WNet'*"]
pub const CONNDLG_RO_PATH: CONNECTDLGSTRUCT_FLAGS = 1u32;
#[doc = "*Required features: 'Win32_NetworkManagement_WNet'*"]
pub const CONNDLG_CONN_POINT: CONNECTDLGSTRUCT_FLAGS = 2u32;
#[doc = "*Required features: 'Win32_NetworkManagement_WNet'*"]
pub const CONNDLG_USE_MRU: CONNECTDLGSTRUCT_FLAGS = 4u32;
#[doc = "*Required features: 'Win32_NetworkManagement_WNet'*"]
pub const CONNDLG_HIDE_BOX: CONNECTDLGSTRUCT_FLAGS = 8u32;
#[doc = "*Required features: 'Win32_NetworkManagement_WNet'*"]
pub const CONNDLG_PERSIST: CONNECTDLGSTRUCT_FLAGS = 16u32;
#[doc = "*Required features: 'Win32_NetworkManagement_WNet'*"]
pub const CONNDLG_NOT_PERSIST: CONNECTDLGSTRUCT_FLAGS = 32u32;
#[doc = "*Required features: 'Win32_NetworkManagement_WNet'*"]
pub const CONNECT_CRED_RESET: u32 = 8192u32;
#[doc = "*Required features: 'Win32_NetworkManagement_WNet'*"]
pub const CONNECT_CURRENT_MEDIA: u32 = 512u32;
#[doc = "*Required features: 'Win32_NetworkManagement_WNet'*"]
pub const CONNECT_GLOBAL_MAPPING: u32 = 262144u32;
#[doc = "*Required features: 'Win32_NetworkManagement_WNet'*"]
pub const CONNECT_LOCALDRIVE: u32 = 256u32;
#[doc = "*Required features: 'Win32_NetworkManagement_WNet'*"]
pub const CONNECT_NEED_DRIVE: u32 = 32u32;
#[doc = "*Required features: 'Win32_NetworkManagement_WNet'*"]
pub const CONNECT_REFCOUNT: u32 = 64u32;
#[doc = "*Required features: 'Win32_NetworkManagement_WNet'*"]
pub const CONNECT_REQUIRE_INTEGRITY: u32 = 16384u32;
#[doc = "*Required features: 'Win32_NetworkManagement_WNet'*"]
pub const CONNECT_REQUIRE_PRIVACY: u32 = 32768u32;
#[doc = "*Required features: 'Win32_NetworkManagement_WNet'*"]
pub const CONNECT_RESERVED: u32 = 4278190080u32;
#[doc = "*Required features: 'Win32_NetworkManagement_WNet'*"]
pub const CONNECT_WRITE_THROUGH_SEMANTICS: u32 = 65536u32;
#[repr(C)]
#[doc = "*Required features: 'Win32_NetworkManagement_WNet', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
pub struct DISCDLGSTRUCTA {
    pub cbStructure: u32,
    pub hwndOwner: super::super::Foundation::HWND,
    pub lpLocalName: super::super::Foundation::PSTR,
    pub lpRemoteName: super::super::Foundation::PSTR,
    pub dwFlags: DISCDLGSTRUCT_FLAGS,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for DISCDLGSTRUCTA {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for DISCDLGSTRUCTA {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: 'Win32_NetworkManagement_WNet', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
pub struct DISCDLGSTRUCTW {
    pub cbStructure: u32,
    pub hwndOwner: super::super::Foundation::HWND,
    pub lpLocalName: super::super::Foundation::PWSTR,
    pub lpRemoteName: super::super::Foundation::PWSTR,
    pub dwFlags: DISCDLGSTRUCT_FLAGS,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for DISCDLGSTRUCTW {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for DISCDLGSTRUCTW {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: 'Win32_NetworkManagement_WNet'*"]
pub type DISCDLGSTRUCT_FLAGS = u32;
#[doc = "*Required features: 'Win32_NetworkManagement_WNet'*"]
pub const DISC_UPDATE_PROFILE: DISCDLGSTRUCT_FLAGS = 1u32;
#[doc = "*Required features: 'Win32_NetworkManagement_WNet'*"]
pub const DISC_NO_FORCE: DISCDLGSTRUCT_FLAGS = 64u32;
#[repr(C)]
#[doc = "*Required features: 'Win32_NetworkManagement_WNet'*"]
pub struct NETCONNECTINFOSTRUCT {
    pub cbStructure: u32,
    pub dwFlags: u32,
    pub dwSpeed: u32,
    pub dwDelay: u32,
    pub dwOptDataSize: u32,
}
impl ::core::marker::Copy for NETCONNECTINFOSTRUCT {}
impl ::core::clone::Clone for NETCONNECTINFOSTRUCT {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: 'Win32_NetworkManagement_WNet', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
pub struct NETINFOSTRUCT {
    pub cbStructure: u32,
    pub dwProviderVersion: u32,
    pub dwStatus: super::super::Foundation::WIN32_ERROR,
    pub dwCharacteristics: NETINFOSTRUCT_CHARACTERISTICS,
    pub dwHandle: usize,
    pub wNetType: u16,
    pub dwPrinters: u32,
    pub dwDrives: u32,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for NETINFOSTRUCT {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for NETINFOSTRUCT {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: 'Win32_NetworkManagement_WNet'*"]
pub type NETINFOSTRUCT_CHARACTERISTICS = u32;
#[doc = "*Required features: 'Win32_NetworkManagement_WNet'*"]
pub const NETINFO_DLL16: NETINFOSTRUCT_CHARACTERISTICS = 1u32;
#[doc = "*Required features: 'Win32_NetworkManagement_WNet'*"]
pub const NETINFO_DISKRED: NETINFOSTRUCT_CHARACTERISTICS = 4u32;
#[doc = "*Required features: 'Win32_NetworkManagement_WNet'*"]
pub const NETINFO_PRINTERRED: NETINFOSTRUCT_CHARACTERISTICS = 8u32;
#[doc = "*Required features: 'Win32_NetworkManagement_WNet'*"]
pub const NETPROPERTY_PERSISTENT: u32 = 1u32;
#[repr(C)]
#[doc = "*Required features: 'Win32_NetworkManagement_WNet', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
pub struct NETRESOURCEA {
    pub dwScope: NET_RESOURCE_SCOPE,
    pub dwType: NET_RESOURCE_TYPE,
    pub dwDisplayType: u32,
    pub dwUsage: u32,
    pub lpLocalName: super::super::Foundation::PSTR,
    pub lpRemoteName: super::super::Foundation::PSTR,
    pub lpComment: super::super::Foundation::PSTR,
    pub lpProvider: super::super::Foundation::PSTR,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for NETRESOURCEA {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for NETRESOURCEA {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: 'Win32_NetworkManagement_WNet', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
pub struct NETRESOURCEW {
    pub dwScope: NET_RESOURCE_SCOPE,
    pub dwType: NET_RESOURCE_TYPE,
    pub dwDisplayType: u32,
    pub dwUsage: u32,
    pub lpLocalName: super::super::Foundation::PWSTR,
    pub lpRemoteName: super::super::Foundation::PWSTR,
    pub lpComment: super::super::Foundation::PWSTR,
    pub lpProvider: super::super::Foundation::PWSTR,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for NETRESOURCEW {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for NETRESOURCEW {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: 'Win32_NetworkManagement_WNet'*"]
pub type NETWORK_NAME_FORMAT_FLAGS = u32;
#[doc = "*Required features: 'Win32_NetworkManagement_WNet'*"]
pub const WNFMT_MULTILINE: NETWORK_NAME_FORMAT_FLAGS = 1u32;
#[doc = "*Required features: 'Win32_NetworkManagement_WNet'*"]
pub const WNFMT_ABBREVIATED: NETWORK_NAME_FORMAT_FLAGS = 2u32;
#[doc = "*Required features: 'Win32_NetworkManagement_WNet'*"]
pub type NET_RESOURCE_SCOPE = u32;
#[doc = "*Required features: 'Win32_NetworkManagement_WNet'*"]
pub const RESOURCE_CONNECTED: NET_RESOURCE_SCOPE = 1u32;
#[doc = "*Required features: 'Win32_NetworkManagement_WNet'*"]
pub const RESOURCE_CONTEXT: NET_RESOURCE_SCOPE = 5u32;
#[doc = "*Required features: 'Win32_NetworkManagement_WNet'*"]
pub const RESOURCE_GLOBALNET: NET_RESOURCE_SCOPE = 2u32;
#[doc = "*Required features: 'Win32_NetworkManagement_WNet'*"]
pub const RESOURCE_REMEMBERED: NET_RESOURCE_SCOPE = 3u32;
#[doc = "*Required features: 'Win32_NetworkManagement_WNet'*"]
pub type NET_RESOURCE_TYPE = u32;
#[doc = "*Required features: 'Win32_NetworkManagement_WNet'*"]
pub const RESOURCETYPE_ANY: NET_RESOURCE_TYPE = 0u32;
#[doc = "*Required features: 'Win32_NetworkManagement_WNet'*"]
pub const RESOURCETYPE_DISK: NET_RESOURCE_TYPE = 1u32;
#[doc = "*Required features: 'Win32_NetworkManagement_WNet'*"]
pub const RESOURCETYPE_PRINT: NET_RESOURCE_TYPE = 2u32;
#[doc = "*Required features: 'Win32_NetworkManagement_WNet'*"]
pub type NET_USE_CONNECT_FLAGS = u32;
#[doc = "*Required features: 'Win32_NetworkManagement_WNet'*"]
pub const CONNECT_INTERACTIVE: NET_USE_CONNECT_FLAGS = 8u32;
#[doc = "*Required features: 'Win32_NetworkManagement_WNet'*"]
pub const CONNECT_PROMPT: NET_USE_CONNECT_FLAGS = 16u32;
#[doc = "*Required features: 'Win32_NetworkManagement_WNet'*"]
pub const CONNECT_REDIRECT: NET_USE_CONNECT_FLAGS = 128u32;
#[doc = "*Required features: 'Win32_NetworkManagement_WNet'*"]
pub const CONNECT_UPDATE_PROFILE: NET_USE_CONNECT_FLAGS = 1u32;
#[doc = "*Required features: 'Win32_NetworkManagement_WNet'*"]
pub const CONNECT_COMMANDLINE: NET_USE_CONNECT_FLAGS = 2048u32;
#[doc = "*Required features: 'Win32_NetworkManagement_WNet'*"]
pub const CONNECT_CMD_SAVECRED: NET_USE_CONNECT_FLAGS = 4096u32;
#[doc = "*Required features: 'Win32_NetworkManagement_WNet'*"]
pub const CONNECT_TEMPORARY: NET_USE_CONNECT_FLAGS = 4u32;
#[doc = "*Required features: 'Win32_NetworkManagement_WNet'*"]
pub const CONNECT_DEFERRED: NET_USE_CONNECT_FLAGS = 1024u32;
#[doc = "*Required features: 'Win32_NetworkManagement_WNet'*"]
pub const CONNECT_UPDATE_RECENT: NET_USE_CONNECT_FLAGS = 2u32;
#[repr(C)]
#[doc = "*Required features: 'Win32_NetworkManagement_WNet', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
pub struct NOTIFYADD {
    pub hwndOwner: super::super::Foundation::HWND,
    pub NetResource: NETRESOURCEA,
    pub dwAddFlags: NET_USE_CONNECT_FLAGS,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for NOTIFYADD {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for NOTIFYADD {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: 'Win32_NetworkManagement_WNet', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
pub struct NOTIFYCANCEL {
    pub lpName: super::super::Foundation::PWSTR,
    pub lpProvider: super::super::Foundation::PWSTR,
    pub dwFlags: u32,
    pub fForce: super::super::Foundation::BOOL,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for NOTIFYCANCEL {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for NOTIFYCANCEL {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: 'Win32_NetworkManagement_WNet'*"]
pub struct NOTIFYINFO {
    pub dwNotifyStatus: u32,
    pub dwOperationStatus: u32,
    pub lpContext: *mut ::core::ffi::c_void,
}
impl ::core::marker::Copy for NOTIFYINFO {}
impl ::core::clone::Clone for NOTIFYINFO {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: 'Win32_NetworkManagement_WNet'*"]
pub const NOTIFY_POST: u32 = 2u32;
#[doc = "*Required features: 'Win32_NetworkManagement_WNet'*"]
pub const NOTIFY_PRE: u32 = 1u32;
#[doc = "*Required features: 'Win32_NetworkManagement_WNet'*"]
pub type NPDIRECTORY_NOTIFY_OPERATION = u32;
#[doc = "*Required features: 'Win32_NetworkManagement_WNet'*"]
pub const WNDN_MKDIR: NPDIRECTORY_NOTIFY_OPERATION = 1u32;
#[doc = "*Required features: 'Win32_NetworkManagement_WNet'*"]
pub const WNDN_RMDIR: NPDIRECTORY_NOTIFY_OPERATION = 2u32;
#[doc = "*Required features: 'Win32_NetworkManagement_WNet'*"]
pub const WNDN_MVDIR: NPDIRECTORY_NOTIFY_OPERATION = 3u32;
#[doc = "*Required features: 'Win32_NetworkManagement_WNet'*"]
pub type NP_PROPERTY_DIALOG_SELECTION = u32;
#[doc = "*Required features: 'Win32_NetworkManagement_WNet'*"]
pub const WNPS_FILE: NP_PROPERTY_DIALOG_SELECTION = 0u32;
#[doc = "*Required features: 'Win32_NetworkManagement_WNet'*"]
pub const WNPS_DIR: NP_PROPERTY_DIALOG_SELECTION = 1u32;
#[doc = "*Required features: 'Win32_NetworkManagement_WNet'*"]
pub const WNPS_MULT: NP_PROPERTY_DIALOG_SELECTION = 2u32;
pub type NetEnumHandle = isize;
#[doc = "*Required features: 'Win32_NetworkManagement_WNet', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
pub type PF_AddConnectNotify = ::core::option::Option<unsafe extern "system" fn(lpnotifyinfo: *mut NOTIFYINFO, lpaddinfo: *const NOTIFYADD) -> u32>;
#[doc = "*Required features: 'Win32_NetworkManagement_WNet', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
pub type PF_CancelConnectNotify = ::core::option::Option<unsafe extern "system" fn(lpnotifyinfo: *mut NOTIFYINFO, lpcancelinfo: *const NOTIFYCANCEL) -> u32>;
#[doc = "*Required features: 'Win32_NetworkManagement_WNet', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
pub type PF_NPAddConnection = ::core::option::Option<unsafe extern "system" fn(lpnetresource: *const NETRESOURCEW, lppassword: super::super::Foundation::PWSTR, lpusername: super::super::Foundation::PWSTR) -> u32>;
#[doc = "*Required features: 'Win32_NetworkManagement_WNet', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
pub type PF_NPAddConnection3 = ::core::option::Option<unsafe extern "system" fn(hwndowner: super::super::Foundation::HWND, lpnetresource: *const NETRESOURCEW, lppassword: super::super::Foundation::PWSTR, lpusername: super::super::Foundation::PWSTR, dwflags: u32) -> u32>;
#[doc = "*Required features: 'Win32_NetworkManagement_WNet', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
pub type PF_NPAddConnection4 = ::core::option::Option<unsafe extern "system" fn(hwndowner: super::super::Foundation::HWND, lpnetresource: *const NETRESOURCEW, lpauthbuffer: *const ::core::ffi::c_void, cbauthbuffer: u32, dwflags: u32, lpuseoptions: *const u8, cbuseoptions: u32) -> u32>;
#[doc = "*Required features: 'Win32_NetworkManagement_WNet', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
pub type PF_NPCancelConnection = ::core::option::Option<unsafe extern "system" fn(lpname: super::super::Foundation::PWSTR, fforce: super::super::Foundation::BOOL) -> u32>;
#[doc = "*Required features: 'Win32_NetworkManagement_WNet', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
pub type PF_NPCancelConnection2 = ::core::option::Option<unsafe extern "system" fn(lpname: super::super::Foundation::PWSTR, fforce: super::super::Foundation::BOOL, dwflags: u32) -> u32>;
#[doc = "*Required features: 'Win32_NetworkManagement_WNet', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
pub type PF_NPCloseEnum = ::core::option::Option<unsafe extern "system" fn(henum: super::super::Foundation::HANDLE) -> u32>;
#[doc = "*Required features: 'Win32_NetworkManagement_WNet', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
pub type PF_NPDeviceMode = ::core::option::Option<unsafe extern "system" fn(hparent: super::super::Foundation::HWND) -> u32>;
#[doc = "*Required features: 'Win32_NetworkManagement_WNet', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
pub type PF_NPDirectoryNotify = ::core::option::Option<unsafe extern "system" fn(hwnd: super::super::Foundation::HWND, lpdir: super::super::Foundation::PWSTR, dwoper: u32) -> u32>;
#[doc = "*Required features: 'Win32_NetworkManagement_WNet', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
pub type PF_NPEnumResource = ::core::option::Option<unsafe extern "system" fn(henum: super::super::Foundation::HANDLE, lpccount: *mut u32, lpbuffer: *mut ::core::ffi::c_void, lpbuffersize: *mut u32) -> u32>;
#[doc = "*Required features: 'Win32_NetworkManagement_WNet', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
pub type PF_NPFMXEditPerm = ::core::option::Option<unsafe extern "system" fn(lpdrivename: super::super::Foundation::PWSTR, hwndfmx: super::super::Foundation::HWND, ndialogtype: u32) -> u32>;
#[doc = "*Required features: 'Win32_NetworkManagement_WNet', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
pub type PF_NPFMXGetPermCaps = ::core::option::Option<unsafe extern "system" fn(lpdrivename: super::super::Foundation::PWSTR) -> u32>;
#[doc = "*Required features: 'Win32_NetworkManagement_WNet', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
pub type PF_NPFMXGetPermHelp = ::core::option::Option<unsafe extern "system" fn(lpdrivename: super::super::Foundation::PWSTR, ndialogtype: u32, fdirectory: super::super::Foundation::BOOL, lpfilenamebuffer: *mut ::core::ffi::c_void, lpbuffersize: *mut u32, lpnhelpcontext: *mut u32) -> u32>;
#[doc = "*Required features: 'Win32_NetworkManagement_WNet', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
pub type PF_NPFormatNetworkName = ::core::option::Option<unsafe extern "system" fn(lpremotename: super::super::Foundation::PWSTR, lpformattedname: super::super::Foundation::PWSTR, lpnlength: *mut u32, dwflags: u32, dwavecharperline: u32) -> u32>;
#[doc = "*Required features: 'Win32_NetworkManagement_WNet'*"]
pub type PF_NPGetCaps = ::core::option::Option<unsafe extern "system" fn(ndex: u32) -> u32>;
#[doc = "*Required features: 'Win32_NetworkManagement_WNet', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
pub type PF_NPGetConnection = ::core::option::Option<unsafe extern "system" fn(lplocalname: super::super::Foundation::PWSTR, lpremotename: super::super::Foundation::PWSTR, lpnbufferlen: *mut u32) -> u32>;
#[doc = "*Required features: 'Win32_NetworkManagement_WNet', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
pub type PF_NPGetConnection3 = ::core::option::Option<unsafe extern "system" fn(lplocalname: super::super::Foundation::PWSTR, dwlevel: u32, lpbuffer: *mut ::core::ffi::c_void, lpbuffersize: *mut u32) -> u32>;
#[doc = "*Required features: 'Win32_NetworkManagement_WNet', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
pub type PF_NPGetConnectionPerformance = ::core::option::Option<unsafe extern "system" fn(lpremotename: super::super::Foundation::PWSTR, lpnetconnectinfo: *mut NETCONNECTINFOSTRUCT) -> u32>;
#[doc = "*Required features: 'Win32_NetworkManagement_WNet', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
pub type PF_NPGetDirectoryType = ::core::option::Option<unsafe extern "system" fn(lpname: super::super::Foundation::PWSTR, lptype: *const i32, bflushcache: super::super::Foundation::BOOL) -> u32>;
#[doc = "*Required features: 'Win32_NetworkManagement_WNet', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
pub type PF_NPGetPersistentUseOptionsForConnection = ::core::option::Option<unsafe extern "system" fn(lpremotepath: super::super::Foundation::PWSTR, lpreaduseoptions: *const u8, cbreaduseoptions: u32, lpwriteuseoptions: *mut u8, lpsizewriteuseoptions: *mut u32) -> u32>;
#[doc = "*Required features: 'Win32_NetworkManagement_WNet', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
pub type PF_NPGetPropertyText = ::core::option::Option<unsafe extern "system" fn(ibutton: u32, npropsel: u32, lpname: super::super::Foundation::PWSTR, lpbuttonname: super::super::Foundation::PWSTR, nbuttonnamelen: u32, ntype: u32) -> u32>;
#[doc = "*Required features: 'Win32_NetworkManagement_WNet', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
pub type PF_NPGetResourceInformation = ::core::option::Option<unsafe extern "system" fn(lpnetresource: *const NETRESOURCEW, lpbuffer: *mut ::core::ffi::c_void, lpbuffersize: *mut u32, lplpsystem: *mut super::super::Foundation::PWSTR) -> u32>;
#[doc = "*Required features: 'Win32_NetworkManagement_WNet', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
pub type PF_NPGetResourceParent = ::core::option::Option<unsafe extern "system" fn(lpnetresource: *const NETRESOURCEW, lpbuffer: *mut ::core::ffi::c_void, lpbuffersize: *mut u32) -> u32>;
#[doc = "*Required features: 'Win32_NetworkManagement_WNet', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
pub type PF_NPGetUniversalName = ::core::option::Option<unsafe extern "system" fn(lplocalpath: super::super::Foundation::PWSTR, dwinfolevel: u32, lpbuffer: *mut ::core::ffi::c_void, lpnbuffersize: *mut u32) -> u32>;
#[doc = "*Required features: 'Win32_NetworkManagement_WNet', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
pub type PF_NPGetUser = ::core::option::Option<unsafe extern "system" fn(lpname: super::super::Foundation::PWSTR, lpusername: super::super::Foundation::PWSTR, lpnbufferlen: *mut u32) -> u32>;
#[doc = "*Required features: 'Win32_NetworkManagement_WNet', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
pub type PF_NPLogonNotify = ::core::option::Option<unsafe extern "system" fn(lplogonid: *const super::super::Foundation::LUID, lpauthentinfotype: super::super::Foundation::PWSTR, lpauthentinfo: *const ::core::ffi::c_void, lppreviousauthentinfotype: super::super::Foundation::PWSTR, lppreviousauthentinfo: *const ::core::ffi::c_void, lpstationname: super::super::Foundation::PWSTR, stationhandle: *const ::core::ffi::c_void, lplogonscript: *mut super::super::Foundation::PWSTR) -> u32>;
#[doc = "*Required features: 'Win32_NetworkManagement_WNet', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
pub type PF_NPOpenEnum = ::core::option::Option<unsafe extern "system" fn(dwscope: u32, dwtype: u32, dwusage: u32, lpnetresource: *const NETRESOURCEW, lphenum: *mut super::super::Foundation::HANDLE) -> u32>;
#[doc = "*Required features: 'Win32_NetworkManagement_WNet', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
pub type PF_NPPasswordChangeNotify = ::core::option::Option<unsafe extern "system" fn(lpauthentinfotype: super::super::Foundation::PWSTR, lpauthentinfo: *const ::core::ffi::c_void, lppreviousauthentinfotype: super::super::Foundation::PWSTR, lppreviousauthentinfo: *const ::core::ffi::c_void, lpstationname: super::super::Foundation::PWSTR, stationhandle: *const ::core::ffi::c_void, dwchangeinfo: u32) -> u32>;
#[doc = "*Required features: 'Win32_NetworkManagement_WNet', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
pub type PF_NPPropertyDialog = ::core::option::Option<unsafe extern "system" fn(hwndparent: super::super::Foundation::HWND, ibuttondlg: u32, npropsel: u32, lpfilename: super::super::Foundation::PWSTR, ntype: u32) -> u32>;
#[doc = "*Required features: 'Win32_NetworkManagement_WNet', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
pub type PF_NPSearchDialog = ::core::option::Option<unsafe extern "system" fn(hwndparent: super::super::Foundation::HWND, lpnetresource: *const NETRESOURCEW, lpbuffer: *mut ::core::ffi::c_void, cbbuffer: u32, lpnflags: *mut u32) -> u32>;
#[repr(C)]
#[doc = "*Required features: 'Win32_NetworkManagement_WNet', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
pub struct REMOTE_NAME_INFOA {
    pub lpUniversalName: super::super::Foundation::PSTR,
    pub lpConnectionName: super::super::Foundation::PSTR,
    pub lpRemainingPath: super::super::Foundation::PSTR,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for REMOTE_NAME_INFOA {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for REMOTE_NAME_INFOA {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: 'Win32_NetworkManagement_WNet', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
pub struct REMOTE_NAME_INFOW {
    pub lpUniversalName: super::super::Foundation::PWSTR,
    pub lpConnectionName: super::super::Foundation::PWSTR,
    pub lpRemainingPath: super::super::Foundation::PWSTR,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for REMOTE_NAME_INFOW {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for REMOTE_NAME_INFOW {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: 'Win32_NetworkManagement_WNet'*"]
pub const RESOURCEDISPLAYTYPE_DIRECTORY: u32 = 9u32;
#[doc = "*Required features: 'Win32_NetworkManagement_WNet'*"]
pub const RESOURCEDISPLAYTYPE_NDSCONTAINER: u32 = 11u32;
#[doc = "*Required features: 'Win32_NetworkManagement_WNet'*"]
pub const RESOURCEDISPLAYTYPE_NETWORK: u32 = 6u32;
#[doc = "*Required features: 'Win32_NetworkManagement_WNet'*"]
pub const RESOURCEDISPLAYTYPE_ROOT: u32 = 7u32;
#[doc = "*Required features: 'Win32_NetworkManagement_WNet'*"]
pub const RESOURCEDISPLAYTYPE_SHAREADMIN: u32 = 8u32;
#[doc = "*Required features: 'Win32_NetworkManagement_WNet'*"]
pub const RESOURCETYPE_RESERVED: u32 = 8u32;
#[doc = "*Required features: 'Win32_NetworkManagement_WNet'*"]
pub const RESOURCETYPE_UNKNOWN: u32 = 4294967295u32;
#[doc = "*Required features: 'Win32_NetworkManagement_WNet'*"]
pub const RESOURCEUSAGE_NOLOCALDEVICE: u32 = 4u32;
#[doc = "*Required features: 'Win32_NetworkManagement_WNet'*"]
pub const RESOURCEUSAGE_RESERVED: u32 = 2147483648u32;
#[doc = "*Required features: 'Win32_NetworkManagement_WNet'*"]
pub const RESOURCEUSAGE_SIBLING: u32 = 8u32;
#[doc = "*Required features: 'Win32_NetworkManagement_WNet'*"]
pub const RESOURCE_RECENT: u32 = 4u32;
#[doc = "*Required features: 'Win32_NetworkManagement_WNet'*"]
pub type UNC_INFO_LEVEL = u32;
#[doc = "*Required features: 'Win32_NetworkManagement_WNet'*"]
pub const UNIVERSAL_NAME_INFO_LEVEL: UNC_INFO_LEVEL = 1u32;
#[doc = "*Required features: 'Win32_NetworkManagement_WNet'*"]
pub const REMOTE_NAME_INFO_LEVEL: UNC_INFO_LEVEL = 2u32;
#[repr(C)]
#[doc = "*Required features: 'Win32_NetworkManagement_WNet', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
pub struct UNIVERSAL_NAME_INFOA {
    pub lpUniversalName: super::super::Foundation::PSTR,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for UNIVERSAL_NAME_INFOA {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for UNIVERSAL_NAME_INFOA {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: 'Win32_NetworkManagement_WNet', 'Win32_Foundation'*"]
#[cfg(feature = "Win32_Foundation")]
pub struct UNIVERSAL_NAME_INFOW {
    pub lpUniversalName: super::super::Foundation::PWSTR,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for UNIVERSAL_NAME_INFOW {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for UNIVERSAL_NAME_INFOW {
    fn clone(&self) -> Self {
        *self
    }
}
#[doc = "*Required features: 'Win32_NetworkManagement_WNet'*"]
pub const WNCON_DYNAMIC: u32 = 8u32;
#[doc = "*Required features: 'Win32_NetworkManagement_WNet'*"]
pub const WNCON_FORNETCARD: u32 = 1u32;
#[doc = "*Required features: 'Win32_NetworkManagement_WNet'*"]
pub const WNCON_NOTROUTED: u32 = 2u32;
#[doc = "*Required features: 'Win32_NetworkManagement_WNet'*"]
pub const WNCON_SLOWLINK: u32 = 4u32;
#[doc = "*Required features: 'Win32_NetworkManagement_WNet'*"]
pub const WNDT_NETWORK: u32 = 1u32;
#[doc = "*Required features: 'Win32_NetworkManagement_WNet'*"]
pub const WNDT_NORMAL: u32 = 0u32;
#[doc = "*Required features: 'Win32_NetworkManagement_WNet'*"]
pub type WNET_OPEN_ENUM_USAGE = u32;
#[doc = "*Required features: 'Win32_NetworkManagement_WNet'*"]
pub const RESOURCEUSAGE_NONE: WNET_OPEN_ENUM_USAGE = 0u32;
#[doc = "*Required features: 'Win32_NetworkManagement_WNet'*"]
pub const RESOURCEUSAGE_CONNECTABLE: WNET_OPEN_ENUM_USAGE = 1u32;
#[doc = "*Required features: 'Win32_NetworkManagement_WNet'*"]
pub const RESOURCEUSAGE_CONTAINER: WNET_OPEN_ENUM_USAGE = 2u32;
#[doc = "*Required features: 'Win32_NetworkManagement_WNet'*"]
pub const RESOURCEUSAGE_ATTACHED: WNET_OPEN_ENUM_USAGE = 16u32;
#[doc = "*Required features: 'Win32_NetworkManagement_WNet'*"]
pub const RESOURCEUSAGE_ALL: WNET_OPEN_ENUM_USAGE = 19u32;
#[doc = "*Required features: 'Win32_NetworkManagement_WNet'*"]
pub const WNFMT_CONNECTION: u32 = 32u32;
#[doc = "*Required features: 'Win32_NetworkManagement_WNet'*"]
pub const WNFMT_INENUM: u32 = 16u32;
#[doc = "*Required features: 'Win32_NetworkManagement_WNet'*"]
pub const WNGETCON_CONNECTED: u32 = 0u32;
#[doc = "*Required features: 'Win32_NetworkManagement_WNet'*"]
pub const WNGETCON_DISCONNECTED: u32 = 1u32;
#[doc = "*Required features: 'Win32_NetworkManagement_WNet'*"]
pub const WNNC_ADMIN: u32 = 9u32;
#[doc = "*Required features: 'Win32_NetworkManagement_WNet'*"]
pub const WNNC_ADM_DIRECTORYNOTIFY: u32 = 2u32;
#[doc = "*Required features: 'Win32_NetworkManagement_WNet'*"]
pub const WNNC_ADM_GETDIRECTORYTYPE: u32 = 1u32;
#[doc = "*Required features: 'Win32_NetworkManagement_WNet'*"]
pub const WNNC_CONNECTION: u32 = 6u32;
#[doc = "*Required features: 'Win32_NetworkManagement_WNet'*"]
pub const WNNC_CONNECTION_FLAGS: u32 = 13u32;
#[doc = "*Required features: 'Win32_NetworkManagement_WNet'*"]
pub const WNNC_CON_ADDCONNECTION: u32 = 1u32;
#[doc = "*Required features: 'Win32_NetworkManagement_WNet'*"]
pub const WNNC_CON_ADDCONNECTION3: u32 = 8u32;
#[doc = "*Required features: 'Win32_NetworkManagement_WNet'*"]
pub const WNNC_CON_ADDCONNECTION4: u32 = 16u32;
#[doc = "*Required features: 'Win32_NetworkManagement_WNet'*"]
pub const WNNC_CON_CANCELCONNECTION: u32 = 2u32;
#[doc = "*Required features: 'Win32_NetworkManagement_WNet'*"]
pub const WNNC_CON_CANCELCONNECTION2: u32 = 32u32;
#[doc = "*Required features: 'Win32_NetworkManagement_WNet'*"]
pub const WNNC_CON_DEFER: u32 = 128u32;
#[doc = "*Required features: 'Win32_NetworkManagement_WNet'*"]
pub const WNNC_CON_GETCONNECTIONS: u32 = 4u32;
#[doc = "*Required features: 'Win32_NetworkManagement_WNet'*"]
pub const WNNC_CON_GETPERFORMANCE: u32 = 64u32;
#[doc = "*Required features: 'Win32_NetworkManagement_WNet'*"]
pub const WNNC_DIALOG: u32 = 8u32;
#[doc = "*Required features: 'Win32_NetworkManagement_WNet'*"]
pub const WNNC_DLG_DEVICEMODE: u32 = 1u32;
#[doc = "*Required features: 'Win32_NetworkManagement_WNet'*"]
pub const WNNC_DLG_FORMATNETWORKNAME: u32 = 128u32;
#[doc = "*Required features: 'Win32_NetworkManagement_WNet'*"]
pub const WNNC_DLG_GETRESOURCEINFORMATION: u32 = 2048u32;
#[doc = "*Required features: 'Win32_NetworkManagement_WNet'*"]
pub const WNNC_DLG_GETRESOURCEPARENT: u32 = 512u32;
#[doc = "*Required features: 'Win32_NetworkManagement_WNet'*"]
pub const WNNC_DLG_PERMISSIONEDITOR: u32 = 256u32;
#[doc = "*Required features: 'Win32_NetworkManagement_WNet'*"]
pub const WNNC_DLG_PROPERTYDIALOG: u32 = 32u32;
#[doc = "*Required features: 'Win32_NetworkManagement_WNet'*"]
pub const WNNC_DLG_SEARCHDIALOG: u32 = 64u32;
#[doc = "*Required features: 'Win32_NetworkManagement_WNet'*"]
pub const WNNC_DRIVER_VERSION: u32 = 3u32;
#[doc = "*Required features: 'Win32_NetworkManagement_WNet'*"]
pub const WNNC_ENUMERATION: u32 = 11u32;
#[doc = "*Required features: 'Win32_NetworkManagement_WNet'*"]
pub const WNNC_ENUM_CONTEXT: u32 = 4u32;
#[doc = "*Required features: 'Win32_NetworkManagement_WNet'*"]
pub const WNNC_ENUM_GLOBAL: u32 = 1u32;
#[doc = "*Required features: 'Win32_NetworkManagement_WNet'*"]
pub const WNNC_ENUM_LOCAL: u32 = 2u32;
#[doc = "*Required features: 'Win32_NetworkManagement_WNet'*"]
pub const WNNC_ENUM_SHAREABLE: u32 = 8u32;
#[doc = "*Required features: 'Win32_NetworkManagement_WNet'*"]
pub const WNNC_NET_NONE: u32 = 0u32;
#[doc = "*Required features: 'Win32_NetworkManagement_WNet'*"]
pub const WNNC_NET_TYPE: u32 = 2u32;
#[doc = "*Required features: 'Win32_NetworkManagement_WNet'*"]
pub const WNNC_SPEC_VERSION: u32 = 1u32;
#[doc = "*Required features: 'Win32_NetworkManagement_WNet'*"]
pub const WNNC_SPEC_VERSION51: u32 = 327681u32;
#[doc = "*Required features: 'Win32_NetworkManagement_WNet'*"]
pub const WNNC_START: u32 = 12u32;
#[doc = "*Required features: 'Win32_NetworkManagement_WNet'*"]
pub const WNNC_USER: u32 = 4u32;
#[doc = "*Required features: 'Win32_NetworkManagement_WNet'*"]
pub const WNNC_USR_GETUSER: u32 = 1u32;
#[doc = "*Required features: 'Win32_NetworkManagement_WNet'*"]
pub const WNNC_WAIT_FOR_START: u32 = 1u32;
#[doc = "*Required features: 'Win32_NetworkManagement_WNet'*"]
pub const WNPERMC_AUDIT: u32 = 2u32;
#[doc = "*Required features: 'Win32_NetworkManagement_WNet'*"]
pub const WNPERMC_OWNER: u32 = 4u32;
#[doc = "*Required features: 'Win32_NetworkManagement_WNet'*"]
pub const WNPERMC_PERM: u32 = 1u32;
#[doc = "*Required features: 'Win32_NetworkManagement_WNet'*"]
pub type WNPERM_DLG = u32;
#[doc = "*Required features: 'Win32_NetworkManagement_WNet'*"]
pub const WNPERM_DLG_PERM: WNPERM_DLG = 0u32;
#[doc = "*Required features: 'Win32_NetworkManagement_WNet'*"]
pub const WNPERM_DLG_AUDIT: WNPERM_DLG = 1u32;
#[doc = "*Required features: 'Win32_NetworkManagement_WNet'*"]
pub const WNPERM_DLG_OWNER: WNPERM_DLG = 2u32;
#[doc = "*Required features: 'Win32_NetworkManagement_WNet'*"]
pub const WNSRCH_REFRESH_FIRST_LEVEL: u32 = 1u32;
#[doc = "*Required features: 'Win32_NetworkManagement_WNet'*"]
pub const WNTYPE_COMM: u32 = 4u32;
#[doc = "*Required features: 'Win32_NetworkManagement_WNet'*"]
pub const WNTYPE_DRIVE: u32 = 1u32;
#[doc = "*Required features: 'Win32_NetworkManagement_WNet'*"]
pub const WNTYPE_FILE: u32 = 2u32;
#[doc = "*Required features: 'Win32_NetworkManagement_WNet'*"]
pub const WNTYPE_PRINTER: u32 = 3u32;
#[doc = "*Required features: 'Win32_NetworkManagement_WNet'*"]
pub const WN_CREDENTIAL_CLASS: u32 = 2u32;
#[doc = "*Required features: 'Win32_NetworkManagement_WNet'*"]
pub const WN_NETWORK_CLASS: u32 = 1u32;
#[doc = "*Required features: 'Win32_NetworkManagement_WNet'*"]
pub const WN_NT_PASSWORD_CHANGED: u32 = 2u32;
#[doc = "*Required features: 'Win32_NetworkManagement_WNet'*"]
pub const WN_PRIMARY_AUTHENT_CLASS: u32 = 4u32;
#[doc = "*Required features: 'Win32_NetworkManagement_WNet'*"]
pub const WN_SERVICE_CLASS: u32 = 8u32;
#[doc = "*Required features: 'Win32_NetworkManagement_WNet'*"]
pub const WN_VALID_LOGON_ACCOUNT: u32 = 1u32;

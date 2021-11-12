#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals)]
#[link(name = "windows")]
extern "system" {
    #[cfg(feature = "Win32_Foundation")]
    pub fn MultinetGetConnectionPerformanceA(lpnetresource: *const NETRESOURCEA, lpnetconnectinfostruct: *mut NETCONNECTINFOSTRUCT) -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn MultinetGetConnectionPerformanceW(lpnetresource: *const NETRESOURCEW, lpnetconnectinfostruct: *mut NETCONNECTINFOSTRUCT) -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn NPAddConnection(lpnetresource: *const NETRESOURCEW, lppassword: super::super::Foundation::PWSTR, lpusername: super::super::Foundation::PWSTR) -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn NPAddConnection3(hwndowner: super::super::Foundation::HWND, lpnetresource: *const NETRESOURCEW, lppassword: super::super::Foundation::PWSTR, lpusername: super::super::Foundation::PWSTR, dwflags: NET_USE_CONNECT_FLAGS) -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn NPAddConnection4(hwndowner: super::super::Foundation::HWND, lpnetresource: *const NETRESOURCEW, lpauthbuffer: *const ::core::ffi::c_void, cbauthbuffer: u32, dwflags: u32, lpuseoptions: *const u8, cbuseoptions: u32) -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn NPCancelConnection(lpname: super::super::Foundation::PWSTR, fforce: super::super::Foundation::BOOL) -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn NPCancelConnection2(lpname: super::super::Foundation::PWSTR, fforce: super::super::Foundation::BOOL, dwflags: u32) -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn NPCloseEnum(henum: super::super::Foundation::HANDLE) -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn NPEnumResource(henum: super::super::Foundation::HANDLE, lpccount: *mut u32, lpbuffer: *mut ::core::ffi::c_void, lpbuffersize: *mut u32) -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn NPFormatNetworkName(lpremotename: super::super::Foundation::PWSTR, lpformattedname: super::super::Foundation::PWSTR, lpnlength: *mut u32, dwflags: NETWORK_NAME_FORMAT_FLAGS, dwavecharperline: u32) -> u32;
    pub fn NPGetCaps(ndex: u32) -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn NPGetConnection(lplocalname: super::super::Foundation::PWSTR, lpremotename: super::super::Foundation::PWSTR, lpnbufferlen: *mut u32) -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn NPGetConnection3(lplocalname: super::super::Foundation::PWSTR, dwlevel: u32, lpbuffer: *mut ::core::ffi::c_void, lpbuffersize: *mut u32) -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn NPGetConnectionPerformance(lpremotename: super::super::Foundation::PWSTR, lpnetconnectinfo: *mut NETCONNECTINFOSTRUCT) -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn NPGetPersistentUseOptionsForConnection(lpremotepath: super::super::Foundation::PWSTR, lpreaduseoptions: *const u8, cbreaduseoptions: u32, lpwriteuseoptions: *mut u8, lpsizewriteuseoptions: *mut u32) -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn NPGetResourceInformation(lpnetresource: *const NETRESOURCEW, lpbuffer: *mut ::core::ffi::c_void, lpbuffersize: *mut u32, lplpsystem: *mut super::super::Foundation::PWSTR) -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn NPGetResourceParent(lpnetresource: *const NETRESOURCEW, lpbuffer: *mut ::core::ffi::c_void, lpbuffersize: *mut u32) -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn NPGetUniversalName(lplocalpath: super::super::Foundation::PWSTR, dwinfolevel: UNC_INFO_LEVEL, lpbuffer: *mut ::core::ffi::c_void, lpbuffersize: *mut u32) -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn NPGetUser(lpname: super::super::Foundation::PWSTR, lpusername: super::super::Foundation::PWSTR, lpnbufferlen: *mut u32) -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn NPOpenEnum(dwscope: u32, dwtype: u32, dwusage: u32, lpnetresource: *const NETRESOURCEW, lphenum: *mut super::super::Foundation::HANDLE) -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn WNetAddConnection2A(lpnetresource: *const NETRESOURCEA, lppassword: super::super::Foundation::PSTR, lpusername: super::super::Foundation::PSTR, dwflags: u32) -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn WNetAddConnection2W(lpnetresource: *const NETRESOURCEW, lppassword: super::super::Foundation::PWSTR, lpusername: super::super::Foundation::PWSTR, dwflags: u32) -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn WNetAddConnection3A(hwndowner: super::super::Foundation::HWND, lpnetresource: *const NETRESOURCEA, lppassword: super::super::Foundation::PSTR, lpusername: super::super::Foundation::PSTR, dwflags: u32) -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn WNetAddConnection3W(hwndowner: super::super::Foundation::HWND, lpnetresource: *const NETRESOURCEW, lppassword: super::super::Foundation::PWSTR, lpusername: super::super::Foundation::PWSTR, dwflags: u32) -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn WNetAddConnection4A(hwndowner: super::super::Foundation::HWND, lpnetresource: *const NETRESOURCEA, pauthbuffer: *const ::core::ffi::c_void, cbauthbuffer: u32, dwflags: u32, lpuseoptions: *const u8, cbuseoptions: u32) -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn WNetAddConnection4W(hwndowner: super::super::Foundation::HWND, lpnetresource: *const NETRESOURCEW, pauthbuffer: *const ::core::ffi::c_void, cbauthbuffer: u32, dwflags: u32, lpuseoptions: *const u8, cbuseoptions: u32) -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn WNetAddConnectionA(lpremotename: super::super::Foundation::PSTR, lppassword: super::super::Foundation::PSTR, lplocalname: super::super::Foundation::PSTR) -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn WNetAddConnectionW(lpremotename: super::super::Foundation::PWSTR, lppassword: super::super::Foundation::PWSTR, lplocalname: super::super::Foundation::PWSTR) -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn WNetCancelConnection2A(lpname: super::super::Foundation::PSTR, dwflags: u32, fforce: super::super::Foundation::BOOL) -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn WNetCancelConnection2W(lpname: super::super::Foundation::PWSTR, dwflags: u32, fforce: super::super::Foundation::BOOL) -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn WNetCancelConnectionA(lpname: super::super::Foundation::PSTR, fforce: super::super::Foundation::BOOL) -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn WNetCancelConnectionW(lpname: super::super::Foundation::PWSTR, fforce: super::super::Foundation::BOOL) -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn WNetCloseEnum(henum: super::super::Foundation::HANDLE) -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn WNetConnectionDialog(hwnd: super::super::Foundation::HWND, dwtype: u32) -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn WNetConnectionDialog1A(lpconndlgstruct: *mut CONNECTDLGSTRUCTA) -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn WNetConnectionDialog1W(lpconndlgstruct: *mut CONNECTDLGSTRUCTW) -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn WNetDisconnectDialog(hwnd: super::super::Foundation::HWND, dwtype: u32) -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn WNetDisconnectDialog1A(lpconndlgstruct: *const DISCDLGSTRUCTA) -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn WNetDisconnectDialog1W(lpconndlgstruct: *const DISCDLGSTRUCTW) -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn WNetEnumResourceA(henum: super::super::Foundation::HANDLE, lpccount: *mut u32, lpbuffer: *mut ::core::ffi::c_void, lpbuffersize: *mut u32) -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn WNetEnumResourceW(henum: super::super::Foundation::HANDLE, lpccount: *mut u32, lpbuffer: *mut ::core::ffi::c_void, lpbuffersize: *mut u32) -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn WNetGetConnectionA(lplocalname: super::super::Foundation::PSTR, lpremotename: super::super::Foundation::PSTR, lpnlength: *mut u32) -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn WNetGetConnectionW(lplocalname: super::super::Foundation::PWSTR, lpremotename: super::super::Foundation::PWSTR, lpnlength: *mut u32) -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn WNetGetLastErrorA(lperror: *mut u32, lperrorbuf: super::super::Foundation::PSTR, nerrorbufsize: u32, lpnamebuf: super::super::Foundation::PSTR, nnamebufsize: u32) -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn WNetGetLastErrorW(lperror: *mut u32, lperrorbuf: super::super::Foundation::PWSTR, nerrorbufsize: u32, lpnamebuf: super::super::Foundation::PWSTR, nnamebufsize: u32) -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn WNetGetNetworkInformationA(lpprovider: super::super::Foundation::PSTR, lpnetinfostruct: *mut NETINFOSTRUCT) -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn WNetGetNetworkInformationW(lpprovider: super::super::Foundation::PWSTR, lpnetinfostruct: *mut NETINFOSTRUCT) -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn WNetGetProviderNameA(dwnettype: u32, lpprovidername: super::super::Foundation::PSTR, lpbuffersize: *mut u32) -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn WNetGetProviderNameW(dwnettype: u32, lpprovidername: super::super::Foundation::PWSTR, lpbuffersize: *mut u32) -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn WNetGetResourceInformationA(lpnetresource: *const NETRESOURCEA, lpbuffer: *mut ::core::ffi::c_void, lpcbbuffer: *mut u32, lplpsystem: *mut super::super::Foundation::PSTR) -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn WNetGetResourceInformationW(lpnetresource: *const NETRESOURCEW, lpbuffer: *mut ::core::ffi::c_void, lpcbbuffer: *mut u32, lplpsystem: *mut super::super::Foundation::PWSTR) -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn WNetGetResourceParentA(lpnetresource: *const NETRESOURCEA, lpbuffer: *mut ::core::ffi::c_void, lpcbbuffer: *mut u32) -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn WNetGetResourceParentW(lpnetresource: *const NETRESOURCEW, lpbuffer: *mut ::core::ffi::c_void, lpcbbuffer: *mut u32) -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn WNetGetUniversalNameA(lplocalpath: super::super::Foundation::PSTR, dwinfolevel: UNC_INFO_LEVEL, lpbuffer: *mut ::core::ffi::c_void, lpbuffersize: *mut u32) -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn WNetGetUniversalNameW(lplocalpath: super::super::Foundation::PWSTR, dwinfolevel: UNC_INFO_LEVEL, lpbuffer: *mut ::core::ffi::c_void, lpbuffersize: *mut u32) -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn WNetGetUserA(lpname: super::super::Foundation::PSTR, lpusername: super::super::Foundation::PSTR, lpnlength: *mut u32) -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn WNetGetUserW(lpname: super::super::Foundation::PWSTR, lpusername: super::super::Foundation::PWSTR, lpnlength: *mut u32) -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn WNetOpenEnumA(dwscope: NET_RESOURCE_SCOPE, dwtype: NET_RESOURCE_TYPE, dwusage: WNET_OPEN_ENUM_USAGE, lpnetresource: *const NETRESOURCEA, lphenum: *mut NetEnumHandle) -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn WNetOpenEnumW(dwscope: NET_RESOURCE_SCOPE, dwtype: NET_RESOURCE_TYPE, dwusage: WNET_OPEN_ENUM_USAGE, lpnetresource: *const NETRESOURCEW, lphenum: *mut NetEnumHandle) -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn WNetSetLastErrorA(err: u32, lperror: super::super::Foundation::PSTR, lpproviders: super::super::Foundation::PSTR);
    #[cfg(feature = "Win32_Foundation")]
    pub fn WNetSetLastErrorW(err: u32, lperror: super::super::Foundation::PWSTR, lpproviders: super::super::Foundation::PWSTR);
    #[cfg(feature = "Win32_Foundation")]
    pub fn WNetUseConnection4A(hwndowner: super::super::Foundation::HWND, lpnetresource: *const NETRESOURCEA, pauthbuffer: *const ::core::ffi::c_void, cbauthbuffer: u32, dwflags: u32, lpuseoptions: *const u8, cbuseoptions: u32, lpaccessname: super::super::Foundation::PSTR, lpbuffersize: *mut u32, lpresult: *mut u32) -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn WNetUseConnection4W(hwndowner: super::super::Foundation::HWND, lpnetresource: *const NETRESOURCEW, pauthbuffer: *const ::core::ffi::c_void, cbauthbuffer: u32, dwflags: u32, lpuseoptions: *const u8, cbuseoptions: u32, lpaccessname: super::super::Foundation::PWSTR, lpbuffersize: *mut u32, lpresult: *mut u32) -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn WNetUseConnectionA(hwndowner: super::super::Foundation::HWND, lpnetresource: *const NETRESOURCEA, lppassword: super::super::Foundation::PSTR, lpuserid: super::super::Foundation::PSTR, dwflags: NET_USE_CONNECT_FLAGS, lpaccessname: super::super::Foundation::PSTR, lpbuffersize: *mut u32, lpresult: *mut u32) -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn WNetUseConnectionW(hwndowner: super::super::Foundation::HWND, lpnetresource: *const NETRESOURCEW, lppassword: super::super::Foundation::PWSTR, lpuserid: super::super::Foundation::PWSTR, dwflags: NET_USE_CONNECT_FLAGS, lpaccessname: super::super::Foundation::PWSTR, lpbuffersize: *mut u32, lpresult: *mut u32) -> u32;
}
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct CONNECTDLGSTRUCTA(i32);
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct CONNECTDLGSTRUCTW(i32);
#[repr(transparent)]
pub struct CONNECTDLGSTRUCT_FLAGS(pub u32);
pub const CONNDLG_RO_PATH: CONNECTDLGSTRUCT_FLAGS = CONNECTDLGSTRUCT_FLAGS(1u32);
pub const CONNDLG_CONN_POINT: CONNECTDLGSTRUCT_FLAGS = CONNECTDLGSTRUCT_FLAGS(2u32);
pub const CONNDLG_USE_MRU: CONNECTDLGSTRUCT_FLAGS = CONNECTDLGSTRUCT_FLAGS(4u32);
pub const CONNDLG_HIDE_BOX: CONNECTDLGSTRUCT_FLAGS = CONNECTDLGSTRUCT_FLAGS(8u32);
pub const CONNDLG_PERSIST: CONNECTDLGSTRUCT_FLAGS = CONNECTDLGSTRUCT_FLAGS(16u32);
pub const CONNDLG_NOT_PERSIST: CONNECTDLGSTRUCT_FLAGS = CONNECTDLGSTRUCT_FLAGS(32u32);
pub const CONNECT_CRED_RESET: u32 = 8192u32;
pub const CONNECT_CURRENT_MEDIA: u32 = 512u32;
pub const CONNECT_GLOBAL_MAPPING: u32 = 262144u32;
pub const CONNECT_LOCALDRIVE: u32 = 256u32;
pub const CONNECT_NEED_DRIVE: u32 = 32u32;
pub const CONNECT_REFCOUNT: u32 = 64u32;
pub const CONNECT_REQUIRE_INTEGRITY: u32 = 16384u32;
pub const CONNECT_REQUIRE_PRIVACY: u32 = 32768u32;
pub const CONNECT_RESERVED: u32 = 4278190080u32;
pub const CONNECT_WRITE_THROUGH_SEMANTICS: u32 = 65536u32;
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct DISCDLGSTRUCTA(i32);
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct DISCDLGSTRUCTW(i32);
#[repr(transparent)]
pub struct DISCDLGSTRUCT_FLAGS(pub u32);
pub const DISC_UPDATE_PROFILE: DISCDLGSTRUCT_FLAGS = DISCDLGSTRUCT_FLAGS(1u32);
pub const DISC_NO_FORCE: DISCDLGSTRUCT_FLAGS = DISCDLGSTRUCT_FLAGS(64u32);
#[repr(C)]
pub struct NETCONNECTINFOSTRUCT(i32);
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct NETINFOSTRUCT(i32);
#[repr(transparent)]
pub struct NETINFOSTRUCT_CHARACTERISTICS(pub u32);
pub const NETINFO_DLL16: NETINFOSTRUCT_CHARACTERISTICS = NETINFOSTRUCT_CHARACTERISTICS(1u32);
pub const NETINFO_DISKRED: NETINFOSTRUCT_CHARACTERISTICS = NETINFOSTRUCT_CHARACTERISTICS(4u32);
pub const NETINFO_PRINTERRED: NETINFOSTRUCT_CHARACTERISTICS = NETINFOSTRUCT_CHARACTERISTICS(8u32);
pub const NETPROPERTY_PERSISTENT: u32 = 1u32;
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct NETRESOURCEA(i32);
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct NETRESOURCEW(i32);
#[repr(transparent)]
pub struct NETWORK_NAME_FORMAT_FLAGS(pub u32);
pub const WNFMT_MULTILINE: NETWORK_NAME_FORMAT_FLAGS = NETWORK_NAME_FORMAT_FLAGS(1u32);
pub const WNFMT_ABBREVIATED: NETWORK_NAME_FORMAT_FLAGS = NETWORK_NAME_FORMAT_FLAGS(2u32);
#[repr(transparent)]
pub struct NET_RESOURCE_SCOPE(pub u32);
pub const RESOURCE_CONNECTED: NET_RESOURCE_SCOPE = NET_RESOURCE_SCOPE(1u32);
pub const RESOURCE_CONTEXT: NET_RESOURCE_SCOPE = NET_RESOURCE_SCOPE(5u32);
pub const RESOURCE_GLOBALNET: NET_RESOURCE_SCOPE = NET_RESOURCE_SCOPE(2u32);
pub const RESOURCE_REMEMBERED: NET_RESOURCE_SCOPE = NET_RESOURCE_SCOPE(3u32);
#[repr(transparent)]
pub struct NET_RESOURCE_TYPE(pub u32);
pub const RESOURCETYPE_ANY: NET_RESOURCE_TYPE = NET_RESOURCE_TYPE(0u32);
pub const RESOURCETYPE_DISK: NET_RESOURCE_TYPE = NET_RESOURCE_TYPE(1u32);
pub const RESOURCETYPE_PRINT: NET_RESOURCE_TYPE = NET_RESOURCE_TYPE(2u32);
#[repr(transparent)]
pub struct NET_USE_CONNECT_FLAGS(pub u32);
pub const CONNECT_INTERACTIVE: NET_USE_CONNECT_FLAGS = NET_USE_CONNECT_FLAGS(8u32);
pub const CONNECT_PROMPT: NET_USE_CONNECT_FLAGS = NET_USE_CONNECT_FLAGS(16u32);
pub const CONNECT_REDIRECT: NET_USE_CONNECT_FLAGS = NET_USE_CONNECT_FLAGS(128u32);
pub const CONNECT_UPDATE_PROFILE: NET_USE_CONNECT_FLAGS = NET_USE_CONNECT_FLAGS(1u32);
pub const CONNECT_COMMANDLINE: NET_USE_CONNECT_FLAGS = NET_USE_CONNECT_FLAGS(2048u32);
pub const CONNECT_CMD_SAVECRED: NET_USE_CONNECT_FLAGS = NET_USE_CONNECT_FLAGS(4096u32);
pub const CONNECT_TEMPORARY: NET_USE_CONNECT_FLAGS = NET_USE_CONNECT_FLAGS(4u32);
pub const CONNECT_DEFERRED: NET_USE_CONNECT_FLAGS = NET_USE_CONNECT_FLAGS(1024u32);
pub const CONNECT_UPDATE_RECENT: NET_USE_CONNECT_FLAGS = NET_USE_CONNECT_FLAGS(2u32);
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct NOTIFYADD(i32);
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct NOTIFYCANCEL(i32);
#[repr(C)]
pub struct NOTIFYINFO(i32);
pub const NOTIFY_POST: u32 = 2u32;
pub const NOTIFY_PRE: u32 = 1u32;
#[repr(transparent)]
pub struct NPDIRECTORY_NOTIFY_OPERATION(pub u32);
pub const WNDN_MKDIR: NPDIRECTORY_NOTIFY_OPERATION = NPDIRECTORY_NOTIFY_OPERATION(1u32);
pub const WNDN_RMDIR: NPDIRECTORY_NOTIFY_OPERATION = NPDIRECTORY_NOTIFY_OPERATION(2u32);
pub const WNDN_MVDIR: NPDIRECTORY_NOTIFY_OPERATION = NPDIRECTORY_NOTIFY_OPERATION(3u32);
#[repr(transparent)]
pub struct NP_PROPERTY_DIALOG_SELECTION(pub u32);
pub const WNPS_FILE: NP_PROPERTY_DIALOG_SELECTION = NP_PROPERTY_DIALOG_SELECTION(0u32);
pub const WNPS_DIR: NP_PROPERTY_DIALOG_SELECTION = NP_PROPERTY_DIALOG_SELECTION(1u32);
pub const WNPS_MULT: NP_PROPERTY_DIALOG_SELECTION = NP_PROPERTY_DIALOG_SELECTION(2u32);
#[repr(C)]
pub struct NetEnumHandle(i32);
#[repr(C)]
pub struct PF_AddConnectNotify(i32);
#[repr(C)]
pub struct PF_CancelConnectNotify(i32);
#[repr(C)]
pub struct PF_NPAddConnection(i32);
#[repr(C)]
pub struct PF_NPAddConnection3(i32);
#[repr(C)]
pub struct PF_NPAddConnection4(i32);
#[repr(C)]
pub struct PF_NPCancelConnection(i32);
#[repr(C)]
pub struct PF_NPCancelConnection2(i32);
#[repr(C)]
pub struct PF_NPCloseEnum(i32);
#[repr(C)]
pub struct PF_NPDeviceMode(i32);
#[repr(C)]
pub struct PF_NPDirectoryNotify(i32);
#[repr(C)]
pub struct PF_NPEnumResource(i32);
#[repr(C)]
pub struct PF_NPFMXEditPerm(i32);
#[repr(C)]
pub struct PF_NPFMXGetPermCaps(i32);
#[repr(C)]
pub struct PF_NPFMXGetPermHelp(i32);
#[repr(C)]
pub struct PF_NPFormatNetworkName(i32);
#[repr(C)]
pub struct PF_NPGetCaps(i32);
#[repr(C)]
pub struct PF_NPGetConnection(i32);
#[repr(C)]
pub struct PF_NPGetConnection3(i32);
#[repr(C)]
pub struct PF_NPGetConnectionPerformance(i32);
#[repr(C)]
pub struct PF_NPGetDirectoryType(i32);
#[repr(C)]
pub struct PF_NPGetPersistentUseOptionsForConnection(i32);
#[repr(C)]
pub struct PF_NPGetPropertyText(i32);
#[repr(C)]
pub struct PF_NPGetResourceInformation(i32);
#[repr(C)]
pub struct PF_NPGetResourceParent(i32);
#[repr(C)]
pub struct PF_NPGetUniversalName(i32);
#[repr(C)]
pub struct PF_NPGetUser(i32);
#[repr(C)]
pub struct PF_NPLogonNotify(i32);
#[repr(C)]
pub struct PF_NPOpenEnum(i32);
#[repr(C)]
pub struct PF_NPPasswordChangeNotify(i32);
#[repr(C)]
pub struct PF_NPPropertyDialog(i32);
#[repr(C)]
pub struct PF_NPSearchDialog(i32);
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct REMOTE_NAME_INFOA(i32);
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct REMOTE_NAME_INFOW(i32);
pub const RESOURCEDISPLAYTYPE_DIRECTORY: u32 = 9u32;
pub const RESOURCEDISPLAYTYPE_NDSCONTAINER: u32 = 11u32;
pub const RESOURCEDISPLAYTYPE_NETWORK: u32 = 6u32;
pub const RESOURCEDISPLAYTYPE_ROOT: u32 = 7u32;
pub const RESOURCEDISPLAYTYPE_SHAREADMIN: u32 = 8u32;
pub const RESOURCETYPE_RESERVED: u32 = 8u32;
pub const RESOURCETYPE_UNKNOWN: u32 = 4294967295u32;
pub const RESOURCEUSAGE_NOLOCALDEVICE: u32 = 4u32;
pub const RESOURCEUSAGE_RESERVED: u32 = 2147483648u32;
pub const RESOURCEUSAGE_SIBLING: u32 = 8u32;
pub const RESOURCE_RECENT: u32 = 4u32;
#[repr(transparent)]
pub struct UNC_INFO_LEVEL(pub u32);
pub const UNIVERSAL_NAME_INFO_LEVEL: UNC_INFO_LEVEL = UNC_INFO_LEVEL(1u32);
pub const REMOTE_NAME_INFO_LEVEL: UNC_INFO_LEVEL = UNC_INFO_LEVEL(2u32);
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct UNIVERSAL_NAME_INFOA(i32);
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct UNIVERSAL_NAME_INFOW(i32);
pub const WNCON_DYNAMIC: u32 = 8u32;
pub const WNCON_FORNETCARD: u32 = 1u32;
pub const WNCON_NOTROUTED: u32 = 2u32;
pub const WNCON_SLOWLINK: u32 = 4u32;
pub const WNDT_NETWORK: u32 = 1u32;
pub const WNDT_NORMAL: u32 = 0u32;
#[repr(transparent)]
pub struct WNET_OPEN_ENUM_USAGE(pub u32);
pub const RESOURCEUSAGE_NONE: WNET_OPEN_ENUM_USAGE = WNET_OPEN_ENUM_USAGE(0u32);
pub const RESOURCEUSAGE_CONNECTABLE: WNET_OPEN_ENUM_USAGE = WNET_OPEN_ENUM_USAGE(1u32);
pub const RESOURCEUSAGE_CONTAINER: WNET_OPEN_ENUM_USAGE = WNET_OPEN_ENUM_USAGE(2u32);
pub const RESOURCEUSAGE_ATTACHED: WNET_OPEN_ENUM_USAGE = WNET_OPEN_ENUM_USAGE(16u32);
pub const RESOURCEUSAGE_ALL: WNET_OPEN_ENUM_USAGE = WNET_OPEN_ENUM_USAGE(19u32);
pub const WNFMT_CONNECTION: u32 = 32u32;
pub const WNFMT_INENUM: u32 = 16u32;
pub const WNGETCON_CONNECTED: u32 = 0u32;
pub const WNGETCON_DISCONNECTED: u32 = 1u32;
pub const WNNC_ADMIN: u32 = 9u32;
pub const WNNC_ADM_DIRECTORYNOTIFY: u32 = 2u32;
pub const WNNC_ADM_GETDIRECTORYTYPE: u32 = 1u32;
pub const WNNC_CONNECTION: u32 = 6u32;
pub const WNNC_CONNECTION_FLAGS: u32 = 13u32;
pub const WNNC_CON_ADDCONNECTION: u32 = 1u32;
pub const WNNC_CON_ADDCONNECTION3: u32 = 8u32;
pub const WNNC_CON_ADDCONNECTION4: u32 = 16u32;
pub const WNNC_CON_CANCELCONNECTION: u32 = 2u32;
pub const WNNC_CON_CANCELCONNECTION2: u32 = 32u32;
pub const WNNC_CON_DEFER: u32 = 128u32;
pub const WNNC_CON_GETCONNECTIONS: u32 = 4u32;
pub const WNNC_CON_GETPERFORMANCE: u32 = 64u32;
pub const WNNC_DIALOG: u32 = 8u32;
pub const WNNC_DLG_DEVICEMODE: u32 = 1u32;
pub const WNNC_DLG_FORMATNETWORKNAME: u32 = 128u32;
pub const WNNC_DLG_GETRESOURCEINFORMATION: u32 = 2048u32;
pub const WNNC_DLG_GETRESOURCEPARENT: u32 = 512u32;
pub const WNNC_DLG_PERMISSIONEDITOR: u32 = 256u32;
pub const WNNC_DLG_PROPERTYDIALOG: u32 = 32u32;
pub const WNNC_DLG_SEARCHDIALOG: u32 = 64u32;
pub const WNNC_DRIVER_VERSION: u32 = 3u32;
pub const WNNC_ENUMERATION: u32 = 11u32;
pub const WNNC_ENUM_CONTEXT: u32 = 4u32;
pub const WNNC_ENUM_GLOBAL: u32 = 1u32;
pub const WNNC_ENUM_LOCAL: u32 = 2u32;
pub const WNNC_ENUM_SHAREABLE: u32 = 8u32;
pub const WNNC_NET_NONE: u32 = 0u32;
pub const WNNC_NET_TYPE: u32 = 2u32;
pub const WNNC_SPEC_VERSION: u32 = 1u32;
pub const WNNC_SPEC_VERSION51: u32 = 327681u32;
pub const WNNC_START: u32 = 12u32;
pub const WNNC_USER: u32 = 4u32;
pub const WNNC_USR_GETUSER: u32 = 1u32;
pub const WNNC_WAIT_FOR_START: u32 = 1u32;
pub const WNPERMC_AUDIT: u32 = 2u32;
pub const WNPERMC_OWNER: u32 = 4u32;
pub const WNPERMC_PERM: u32 = 1u32;
#[repr(transparent)]
pub struct WNPERM_DLG(pub u32);
pub const WNPERM_DLG_PERM: WNPERM_DLG = WNPERM_DLG(0u32);
pub const WNPERM_DLG_AUDIT: WNPERM_DLG = WNPERM_DLG(1u32);
pub const WNPERM_DLG_OWNER: WNPERM_DLG = WNPERM_DLG(2u32);
pub const WNSRCH_REFRESH_FIRST_LEVEL: u32 = 1u32;
pub const WNTYPE_COMM: u32 = 4u32;
pub const WNTYPE_DRIVE: u32 = 1u32;
pub const WNTYPE_FILE: u32 = 2u32;
pub const WNTYPE_PRINTER: u32 = 3u32;
pub const WN_CREDENTIAL_CLASS: u32 = 2u32;
pub const WN_NETWORK_CLASS: u32 = 1u32;
pub const WN_NT_PASSWORD_CHANGED: u32 = 2u32;
pub const WN_PRIMARY_AUTHENT_CLASS: u32 = 4u32;
pub const WN_SERVICE_CLASS: u32 = 8u32;
pub const WN_VALID_LOGON_ACCOUNT: u32 = 1u32;

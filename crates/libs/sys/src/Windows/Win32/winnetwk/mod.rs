windows_link::link!("mpr.dll" "system" fn MultinetGetConnectionPerformanceA(lpnetresource : *const NETRESOURCEA, lpnetconnectinfostruct : *mut NETCONNECTINFOSTRUCT) -> u32);
windows_link::link!("mpr.dll" "system" fn MultinetGetConnectionPerformanceW(lpnetresource : *const NETRESOURCEW, lpnetconnectinfostruct : *mut NETCONNECTINFOSTRUCT) -> u32);
windows_link::link!("mpr.dll" "system" fn WNetAddConnection2A(lpnetresource : *const NETRESOURCEA, lppassword : windows_sys::core::PCSTR, lpusername : windows_sys::core::PCSTR, dwflags : u32) -> u32);
windows_link::link!("mpr.dll" "system" fn WNetAddConnection2W(lpnetresource : *const NETRESOURCEW, lppassword : windows_sys::core::PCWSTR, lpusername : windows_sys::core::PCWSTR, dwflags : u32) -> u32);
#[cfg(feature = "windef")]
windows_link::link!("mpr.dll" "system" fn WNetAddConnection3A(hwndowner : super::HWND, lpnetresource : *const NETRESOURCEA, lppassword : windows_sys::core::PCSTR, lpusername : windows_sys::core::PCSTR, dwflags : u32) -> u32);
#[cfg(feature = "windef")]
windows_link::link!("mpr.dll" "system" fn WNetAddConnection3W(hwndowner : super::HWND, lpnetresource : *const NETRESOURCEW, lppassword : windows_sys::core::PCWSTR, lpusername : windows_sys::core::PCWSTR, dwflags : u32) -> u32);
#[cfg(feature = "windef")]
windows_link::link!("mpr.dll" "system" fn WNetAddConnection4A(hwndowner : super::HWND, lpnetresource : *const NETRESOURCEA, pauthbuffer : *const core::ffi::c_void, cbauthbuffer : u32, dwflags : u32, lpuseoptions : *const u8, cbuseoptions : u32) -> u32);
#[cfg(feature = "windef")]
windows_link::link!("mpr.dll" "system" fn WNetAddConnection4W(hwndowner : super::HWND, lpnetresource : *const NETRESOURCEW, pauthbuffer : *const core::ffi::c_void, cbauthbuffer : u32, dwflags : u32, lpuseoptions : *const u8, cbuseoptions : u32) -> u32);
windows_link::link!("mpr.dll" "system" fn WNetAddConnectionA(lpremotename : windows_sys::core::PCSTR, lppassword : windows_sys::core::PCSTR, lplocalname : windows_sys::core::PCSTR) -> u32);
windows_link::link!("mpr.dll" "system" fn WNetAddConnectionW(lpremotename : windows_sys::core::PCWSTR, lppassword : windows_sys::core::PCWSTR, lplocalname : windows_sys::core::PCWSTR) -> u32);
windows_link::link!("mpr.dll" "system" fn WNetCancelConnection2A(lpname : windows_sys::core::PCSTR, dwflags : u32, fforce : windows_sys::core::BOOL) -> u32);
windows_link::link!("mpr.dll" "system" fn WNetCancelConnection2W(lpname : windows_sys::core::PCWSTR, dwflags : u32, fforce : windows_sys::core::BOOL) -> u32);
windows_link::link!("mpr.dll" "system" fn WNetCancelConnectionA(lpname : windows_sys::core::PCSTR, fforce : windows_sys::core::BOOL) -> u32);
windows_link::link!("mpr.dll" "system" fn WNetCancelConnectionW(lpname : windows_sys::core::PCWSTR, fforce : windows_sys::core::BOOL) -> u32);
#[cfg(feature = "winnt")]
windows_link::link!("mpr.dll" "system" fn WNetCloseEnum(henum : super::HANDLE) -> u32);
#[cfg(feature = "windef")]
windows_link::link!("mpr.dll" "system" fn WNetConnectionDialog(hwnd : super::HWND, dwtype : u32) -> u32);
#[cfg(feature = "windef")]
windows_link::link!("mpr.dll" "system" fn WNetConnectionDialog1A(lpconndlgstruct : *mut CONNECTDLGSTRUCTA) -> u32);
#[cfg(feature = "windef")]
windows_link::link!("mpr.dll" "system" fn WNetConnectionDialog1W(lpconndlgstruct : *mut CONNECTDLGSTRUCTW) -> u32);
#[cfg(feature = "windef")]
windows_link::link!("mpr.dll" "system" fn WNetDisconnectDialog(hwnd : super::HWND, dwtype : u32) -> u32);
#[cfg(feature = "windef")]
windows_link::link!("mpr.dll" "system" fn WNetDisconnectDialog1A(lpconndlgstruct : *const DISCDLGSTRUCTA) -> u32);
#[cfg(feature = "windef")]
windows_link::link!("mpr.dll" "system" fn WNetDisconnectDialog1W(lpconndlgstruct : *const DISCDLGSTRUCTW) -> u32);
#[cfg(feature = "winnt")]
windows_link::link!("mpr.dll" "system" fn WNetEnumResourceA(henum : super::HANDLE, lpccount : *mut u32, lpbuffer : *mut core::ffi::c_void, lpbuffersize : *mut u32) -> u32);
#[cfg(feature = "winnt")]
windows_link::link!("mpr.dll" "system" fn WNetEnumResourceW(henum : super::HANDLE, lpccount : *mut u32, lpbuffer : *mut core::ffi::c_void, lpbuffersize : *mut u32) -> u32);
windows_link::link!("mpr.dll" "system" fn WNetGetConnectionA(lplocalname : windows_sys::core::PCSTR, lpremotename : windows_sys::core::PSTR, lpnlength : *mut u32) -> u32);
windows_link::link!("mpr.dll" "system" fn WNetGetConnectionW(lplocalname : windows_sys::core::PCWSTR, lpremotename : windows_sys::core::PWSTR, lpnlength : *mut u32) -> u32);
windows_link::link!("mpr.dll" "system" fn WNetGetLastErrorA(lperror : *mut u32, lperrorbuf : windows_sys::core::PSTR, nerrorbufsize : u32, lpnamebuf : windows_sys::core::PSTR, nnamebufsize : u32) -> u32);
windows_link::link!("mpr.dll" "system" fn WNetGetLastErrorW(lperror : *mut u32, lperrorbuf : windows_sys::core::PWSTR, nerrorbufsize : u32, lpnamebuf : windows_sys::core::PWSTR, nnamebufsize : u32) -> u32);
windows_link::link!("mpr.dll" "system" fn WNetGetNetworkInformationA(lpprovider : windows_sys::core::PCSTR, lpnetinfostruct : *mut NETINFOSTRUCT) -> u32);
windows_link::link!("mpr.dll" "system" fn WNetGetNetworkInformationW(lpprovider : windows_sys::core::PCWSTR, lpnetinfostruct : *mut NETINFOSTRUCT) -> u32);
windows_link::link!("mpr.dll" "system" fn WNetGetProviderNameA(dwnettype : u32, lpprovidername : windows_sys::core::PSTR, lpbuffersize : *mut u32) -> u32);
windows_link::link!("mpr.dll" "system" fn WNetGetProviderNameW(dwnettype : u32, lpprovidername : windows_sys::core::PWSTR, lpbuffersize : *mut u32) -> u32);
windows_link::link!("mpr.dll" "system" fn WNetGetResourceInformationA(lpnetresource : *const NETRESOURCEA, lpbuffer : *mut core::ffi::c_void, lpcbbuffer : *mut u32, lplpsystem : *mut windows_sys::core::PSTR) -> u32);
windows_link::link!("mpr.dll" "system" fn WNetGetResourceInformationW(lpnetresource : *const NETRESOURCEW, lpbuffer : *mut core::ffi::c_void, lpcbbuffer : *mut u32, lplpsystem : *mut windows_sys::core::PWSTR) -> u32);
windows_link::link!("mpr.dll" "system" fn WNetGetResourceParentA(lpnetresource : *const NETRESOURCEA, lpbuffer : *mut core::ffi::c_void, lpcbbuffer : *mut u32) -> u32);
windows_link::link!("mpr.dll" "system" fn WNetGetResourceParentW(lpnetresource : *const NETRESOURCEW, lpbuffer : *mut core::ffi::c_void, lpcbbuffer : *mut u32) -> u32);
windows_link::link!("mpr.dll" "system" fn WNetGetUniversalNameA(lplocalpath : windows_sys::core::PCSTR, dwinfolevel : u32, lpbuffer : *mut core::ffi::c_void, lpbuffersize : *mut u32) -> u32);
windows_link::link!("mpr.dll" "system" fn WNetGetUniversalNameW(lplocalpath : windows_sys::core::PCWSTR, dwinfolevel : u32, lpbuffer : *mut core::ffi::c_void, lpbuffersize : *mut u32) -> u32);
windows_link::link!("mpr.dll" "system" fn WNetGetUserA(lpname : windows_sys::core::PCSTR, lpusername : windows_sys::core::PSTR, lpnlength : *mut u32) -> u32);
windows_link::link!("mpr.dll" "system" fn WNetGetUserW(lpname : windows_sys::core::PCWSTR, lpusername : windows_sys::core::PWSTR, lpnlength : *mut u32) -> u32);
#[cfg(feature = "winnt")]
windows_link::link!("mpr.dll" "system" fn WNetOpenEnumA(dwscope : u32, dwtype : u32, dwusage : u32, lpnetresource : *const NETRESOURCEA, lphenum : *mut super::HANDLE) -> u32);
#[cfg(feature = "winnt")]
windows_link::link!("mpr.dll" "system" fn WNetOpenEnumW(dwscope : u32, dwtype : u32, dwusage : u32, lpnetresource : *const NETRESOURCEW, lphenum : *mut super::HANDLE) -> u32);
#[cfg(feature = "windef")]
windows_link::link!("mpr.dll" "system" fn WNetUseConnection4A(hwndowner : super::HWND, lpnetresource : *const NETRESOURCEA, pauthbuffer : *const core::ffi::c_void, cbauthbuffer : u32, dwflags : u32, lpuseoptions : *const u8, cbuseoptions : u32, lpaccessname : windows_sys::core::PSTR, lpbuffersize : *mut u32, lpresult : *mut u32) -> u32);
#[cfg(feature = "windef")]
windows_link::link!("mpr.dll" "system" fn WNetUseConnection4W(hwndowner : super::HWND, lpnetresource : *const NETRESOURCEW, pauthbuffer : *const core::ffi::c_void, cbauthbuffer : u32, dwflags : u32, lpuseoptions : *const u8, cbuseoptions : u32, lpaccessname : windows_sys::core::PWSTR, lpbuffersize : *mut u32, lpresult : *mut u32) -> u32);
#[cfg(feature = "windef")]
windows_link::link!("mpr.dll" "system" fn WNetUseConnectionA(hwndowner : super::HWND, lpnetresource : *const NETRESOURCEA, lppassword : windows_sys::core::PCSTR, lpuserid : windows_sys::core::PCSTR, dwflags : u32, lpaccessname : windows_sys::core::PSTR, lpbuffersize : *mut u32, lpresult : *mut u32) -> u32);
#[cfg(feature = "windef")]
windows_link::link!("mpr.dll" "system" fn WNetUseConnectionW(hwndowner : super::HWND, lpnetresource : *const NETRESOURCEW, lppassword : windows_sys::core::PCWSTR, lpuserid : windows_sys::core::PCWSTR, dwflags : u32, lpaccessname : windows_sys::core::PWSTR, lpbuffersize : *mut u32, lpresult : *mut u32) -> u32);
pub const CONNDLG_CONN_POINT: i32 = 2;
pub const CONNDLG_HIDE_BOX: i32 = 8;
pub const CONNDLG_NOT_PERSIST: i32 = 32;
pub const CONNDLG_PERSIST: i32 = 16;
pub const CONNDLG_RO_PATH: i32 = 1;
pub const CONNDLG_USE_MRU: i32 = 4;
#[cfg(feature = "windef")]
pub type CONNECTDLGSTRUCT = CONNECTDLGSTRUCTA;
#[repr(C)]
#[cfg(feature = "windef")]
#[derive(Clone, Copy, Default)]
pub struct CONNECTDLGSTRUCTA {
    pub cbStructure: u32,
    pub hwndOwner: super::HWND,
    pub lpConnRes: LPNETRESOURCEA,
    pub dwFlags: u32,
    pub dwDevNum: u32,
}
#[repr(C)]
#[cfg(feature = "windef")]
#[derive(Clone, Copy, Default)]
pub struct CONNECTDLGSTRUCTW {
    pub cbStructure: u32,
    pub hwndOwner: super::HWND,
    pub lpConnRes: LPNETRESOURCEW,
    pub dwFlags: u32,
    pub dwDevNum: u32,
}
pub const CONNECT_CMD_SAVECRED: i32 = 4096;
pub const CONNECT_COMMANDLINE: i32 = 2048;
pub const CONNECT_CRED_RESET: i32 = 8192;
pub const CONNECT_CURRENT_MEDIA: i32 = 512;
pub const CONNECT_DEFERRED: i32 = 1024;
pub const CONNECT_GLOBAL_MAPPING: i32 = 262144;
pub const CONNECT_INTERACTIVE: i32 = 8;
pub const CONNECT_LOCALDRIVE: i32 = 256;
pub const CONNECT_NEED_DRIVE: i32 = 32;
pub const CONNECT_PROMPT: i32 = 16;
pub const CONNECT_REDIRECT: i32 = 128;
pub const CONNECT_REFCOUNT: i32 = 64;
pub const CONNECT_REQUIRE_INTEGRITY: i32 = 16384;
pub const CONNECT_REQUIRE_PRIVACY: i32 = 32768;
pub const CONNECT_RESERVED: u32 = 4278190080;
pub const CONNECT_TEMPORARY: i32 = 4;
pub const CONNECT_UPDATE_PROFILE: i32 = 1;
pub const CONNECT_UPDATE_RECENT: i32 = 2;
pub const CONNECT_WRITE_THROUGH_SEMANTICS: i32 = 65536;
#[cfg(feature = "windef")]
pub type DISCDLGSTRUCT = DISCDLGSTRUCTA;
#[repr(C)]
#[cfg(feature = "windef")]
#[derive(Clone, Copy, Default)]
pub struct DISCDLGSTRUCTA {
    pub cbStructure: u32,
    pub hwndOwner: super::HWND,
    pub lpLocalName: windows_sys::core::PSTR,
    pub lpRemoteName: windows_sys::core::PSTR,
    pub dwFlags: u32,
}
#[repr(C)]
#[cfg(feature = "windef")]
#[derive(Clone, Copy, Default)]
pub struct DISCDLGSTRUCTW {
    pub cbStructure: u32,
    pub hwndOwner: super::HWND,
    pub lpLocalName: windows_sys::core::PWSTR,
    pub lpRemoteName: windows_sys::core::PWSTR,
    pub dwFlags: u32,
}
pub const DISC_NO_FORCE: i32 = 64;
pub const DISC_UPDATE_PROFILE: i32 = 1;
#[cfg(feature = "windef")]
pub type LPCONNECTDLGSTRUCT = LPCONNECTDLGSTRUCTA;
#[cfg(feature = "windef")]
pub type LPCONNECTDLGSTRUCTA = *mut CONNECTDLGSTRUCTA;
#[cfg(feature = "windef")]
pub type LPCONNECTDLGSTRUCTW = *mut CONNECTDLGSTRUCTW;
#[cfg(feature = "windef")]
pub type LPDISCDLGSTRUCT = LPDISCDLGSTRUCTA;
#[cfg(feature = "windef")]
pub type LPDISCDLGSTRUCTA = *mut DISCDLGSTRUCTA;
#[cfg(feature = "windef")]
pub type LPDISCDLGSTRUCTW = *mut DISCDLGSTRUCTW;
pub type LPNETCONNECTINFOSTRUCT = *mut NETCONNECTINFOSTRUCT;
pub type LPNETINFOSTRUCT = *mut NETINFOSTRUCT;
pub type LPNETRESOURCE = LPNETRESOURCEA;
pub type LPNETRESOURCEA = *mut NETRESOURCEA;
pub type LPNETRESOURCEW = *mut NETRESOURCEW;
pub type LPREMOTE_NAME_INFO = LPREMOTE_NAME_INFOA;
pub type LPREMOTE_NAME_INFOA = *mut REMOTE_NAME_INFOA;
pub type LPREMOTE_NAME_INFOW = *mut REMOTE_NAME_INFOW;
pub type LPUNIVERSAL_NAME_INFO = LPUNIVERSAL_NAME_INFOA;
pub type LPUNIVERSAL_NAME_INFOA = *mut UNIVERSAL_NAME_INFOA;
pub type LPUNIVERSAL_NAME_INFOW = *mut UNIVERSAL_NAME_INFOW;
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct NETCONNECTINFOSTRUCT {
    pub cbStructure: u32,
    pub dwFlags: u32,
    pub dwSpeed: u32,
    pub dwDelay: u32,
    pub dwOptDataSize: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct NETINFOSTRUCT {
    pub cbStructure: u32,
    pub dwProviderVersion: u32,
    pub dwStatus: u32,
    pub dwCharacteristics: u32,
    pub dwHandle: usize,
    pub wNetType: u16,
    pub dwPrinters: u32,
    pub dwDrives: u32,
}
pub const NETINFO_DISKRED: i32 = 4;
pub const NETINFO_DLL16: i32 = 1;
pub const NETINFO_PRINTERRED: i32 = 8;
pub const NETPROPERTY_PERSISTENT: i32 = 1;
pub type NETRESOURCE = NETRESOURCEA;
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct NETRESOURCEA {
    pub dwScope: u32,
    pub dwType: u32,
    pub dwDisplayType: u32,
    pub dwUsage: u32,
    pub lpLocalName: windows_sys::core::PSTR,
    pub lpRemoteName: windows_sys::core::PSTR,
    pub lpComment: windows_sys::core::PSTR,
    pub lpProvider: windows_sys::core::PSTR,
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct NETRESOURCEW {
    pub dwScope: u32,
    pub dwType: u32,
    pub dwDisplayType: u32,
    pub dwUsage: u32,
    pub lpLocalName: windows_sys::core::PWSTR,
    pub lpRemoteName: windows_sys::core::PWSTR,
    pub lpComment: windows_sys::core::PWSTR,
    pub lpProvider: windows_sys::core::PWSTR,
}
pub type REMOTE_NAME_INFO = REMOTE_NAME_INFOA;
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct REMOTE_NAME_INFOA {
    pub lpUniversalName: windows_sys::core::PSTR,
    pub lpConnectionName: windows_sys::core::PSTR,
    pub lpRemainingPath: windows_sys::core::PSTR,
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct REMOTE_NAME_INFOW {
    pub lpUniversalName: windows_sys::core::PWSTR,
    pub lpConnectionName: windows_sys::core::PWSTR,
    pub lpRemainingPath: windows_sys::core::PWSTR,
}
pub const REMOTE_NAME_INFO_LEVEL: i32 = 2;
pub const RESOURCEDISPLAYTYPE_DIRECTORY: i32 = 9;
pub const RESOURCEDISPLAYTYPE_DOMAIN: i32 = 1;
pub const RESOURCEDISPLAYTYPE_FILE: i32 = 4;
pub const RESOURCEDISPLAYTYPE_GENERIC: i32 = 0;
pub const RESOURCEDISPLAYTYPE_GROUP: i32 = 5;
pub const RESOURCEDISPLAYTYPE_NDSCONTAINER: i32 = 11;
pub const RESOURCEDISPLAYTYPE_NETWORK: i32 = 6;
pub const RESOURCEDISPLAYTYPE_ROOT: i32 = 7;
pub const RESOURCEDISPLAYTYPE_SERVER: i32 = 2;
pub const RESOURCEDISPLAYTYPE_SHARE: i32 = 3;
pub const RESOURCEDISPLAYTYPE_SHAREADMIN: i32 = 8;
pub const RESOURCEDISPLAYTYPE_TREE: i32 = 10;
pub const RESOURCETYPE_ANY: i32 = 0;
pub const RESOURCETYPE_DISK: i32 = 1;
pub const RESOURCETYPE_PRINT: i32 = 2;
pub const RESOURCETYPE_RESERVED: i32 = 8;
pub const RESOURCETYPE_UNKNOWN: u32 = 4294967295;
pub const RESOURCEUSAGE_ALL: i32 = 19;
pub const RESOURCEUSAGE_ATTACHED: i32 = 16;
pub const RESOURCEUSAGE_CONNECTABLE: i32 = 1;
pub const RESOURCEUSAGE_CONTAINER: i32 = 2;
pub const RESOURCEUSAGE_NOLOCALDEVICE: i32 = 4;
pub const RESOURCEUSAGE_RESERVED: u32 = 2147483648;
pub const RESOURCEUSAGE_SIBLING: i32 = 8;
pub const RESOURCE_CONNECTED: i32 = 1;
pub const RESOURCE_CONTEXT: i32 = 5;
pub const RESOURCE_GLOBALNET: i32 = 2;
pub const RESOURCE_RECENT: i32 = 4;
pub const RESOURCE_REMEMBERED: i32 = 3;
pub type UNIVERSAL_NAME_INFO = UNIVERSAL_NAME_INFOA;
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct UNIVERSAL_NAME_INFOA {
    pub lpUniversalName: windows_sys::core::PSTR,
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct UNIVERSAL_NAME_INFOW {
    pub lpUniversalName: windows_sys::core::PWSTR,
}
pub const UNIVERSAL_NAME_INFO_LEVEL: i32 = 1;
pub const WNCON_DYNAMIC: i32 = 8;
pub const WNCON_FORNETCARD: i32 = 1;
pub const WNCON_NOTROUTED: i32 = 2;
pub const WNCON_SLOWLINK: i32 = 4;
pub const WNFMT_ABBREVIATED: i32 = 2;
pub const WNFMT_CONNECTION: i32 = 32;
pub const WNFMT_INENUM: i32 = 16;
pub const WNFMT_MULTILINE: i32 = 1;
pub const WN_ACCESS_DENIED: i32 = 5;
pub const WN_ALREADY_CONNECTED: i32 = 85;
pub const WN_BAD_DEV_TYPE: i32 = 66;
pub const WN_BAD_HANDLE: i32 = 6;
pub const WN_BAD_LEVEL: i32 = 124;
pub const WN_BAD_LOCALNAME: i32 = 1200;
pub const WN_BAD_NETNAME: i32 = 67;
pub const WN_BAD_PASSWORD: i32 = 86;
pub const WN_BAD_POINTER: i32 = 487;
pub const WN_BAD_PROFILE: i32 = 1206;
pub const WN_BAD_PROVIDER: i32 = 1204;
pub const WN_BAD_USER: i32 = 2202;
pub const WN_BAD_VALUE: i32 = 87;
pub const WN_CANCEL: i32 = 1223;
pub const WN_CANNOT_OPEN_PROFILE: i32 = 1205;
pub const WN_CONNECTED_OTHER_PASSWORD: i32 = 2108;
pub const WN_CONNECTED_OTHER_PASSWORD_DEFAULT: i32 = 2109;
pub const WN_CONNECTION_CLOSED: i32 = 1201;
pub const WN_DEVICE_ALREADY_REMEMBERED: i32 = 1202;
pub const WN_DEVICE_ERROR: i32 = 31;
pub const WN_DEVICE_IN_USE: i32 = 2404;
pub const WN_EXTENDED_ERROR: i32 = 1208;
pub const WN_FUNCTION_BUSY: i32 = 170;
pub const WN_MORE_DATA: i32 = 234;
pub const WN_NET_ERROR: i32 = 59;
pub const WN_NOT_AUTHENTICATED: i32 = 1244;
pub const WN_NOT_CONNECTED: i32 = 2250;
pub const WN_NOT_CONTAINER: i32 = 1207;
pub const WN_NOT_INITIALIZING: i32 = 1247;
pub const WN_NOT_LOGGED_ON: i32 = 1245;
pub const WN_NOT_SUPPORTED: i32 = 50;
pub const WN_NOT_VALIDATED: i32 = 1311;
pub const WN_NO_ERROR: i32 = 0;
pub const WN_NO_MORE_DEVICES: i32 = 1248;
pub const WN_NO_MORE_ENTRIES: i32 = 259;
pub const WN_NO_NETWORK: i32 = 1222;
pub const WN_NO_NET_OR_BAD_PATH: i32 = 1203;
pub const WN_OPEN_FILES: i32 = 2401;
pub const WN_OUT_OF_MEMORY: i32 = 8;
pub const WN_RETRY: i32 = 1237;
pub const WN_SUCCESS: i32 = 0;
pub const WN_WINDOWS_ERROR: i32 = 59;

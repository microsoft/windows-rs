#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals)]
#[link(name = "windows")]
extern "system" {}
#[cfg(feature = "Win32_Foundation")]
pub type PFNMSGECALLBACK = unsafe extern "system" fn(bverbose: super::super::Foundation::BOOL, lpmessage: super::super::Foundation::PWSTR) -> u32;
#[cfg(feature = "Win32_Foundation")]
pub type PWLX_ASSIGN_SHELL_PROTECTION = unsafe extern "system" fn(hwlx: super::super::Foundation::HANDLE, htoken: super::super::Foundation::HANDLE, hprocess: super::super::Foundation::HANDLE, hthread: super::super::Foundation::HANDLE) -> i32;
#[cfg(feature = "Win32_Foundation")]
pub type PWLX_CHANGE_PASSWORD_NOTIFY = unsafe extern "system" fn(hwlx: super::super::Foundation::HANDLE, pmprinfo: *mut WLX_MPR_NOTIFY_INFO, dwchangeinfo: u32) -> i32;
#[cfg(feature = "Win32_Foundation")]
pub type PWLX_CHANGE_PASSWORD_NOTIFY_EX = unsafe extern "system" fn(hwlx: super::super::Foundation::HANDLE, pmprinfo: *mut WLX_MPR_NOTIFY_INFO, dwchangeinfo: u32, providername: super::super::Foundation::PWSTR, reserved: *mut ::core::ffi::c_void) -> i32;
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_StationsAndDesktops"))]
pub type PWLX_CLOSE_USER_DESKTOP = unsafe extern "system" fn(hwlx: super::super::Foundation::HANDLE, pdesktop: *mut WLX_DESKTOP, htoken: super::super::Foundation::HANDLE) -> super::super::Foundation::BOOL;
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_StationsAndDesktops"))]
pub type PWLX_CREATE_USER_DESKTOP = unsafe extern "system" fn(hwlx: super::super::Foundation::HANDLE, htoken: super::super::Foundation::HANDLE, flags: u32, pszdesktopname: super::super::Foundation::PWSTR, ppdesktop: *mut *mut WLX_DESKTOP) -> super::super::Foundation::BOOL;
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_WindowsAndMessaging"))]
pub type PWLX_DIALOG_BOX = unsafe extern "system" fn(hwlx: super::super::Foundation::HANDLE, hinst: super::super::Foundation::HANDLE, lpsztemplate: super::super::Foundation::PWSTR, hwndowner: super::super::Foundation::HWND, dlgprc: super::super::UI::WindowsAndMessaging::DLGPROC) -> i32;
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_WindowsAndMessaging"))]
pub type PWLX_DIALOG_BOX_INDIRECT = unsafe extern "system" fn(hwlx: super::super::Foundation::HANDLE, hinst: super::super::Foundation::HANDLE, hdialogtemplate: *mut super::super::UI::WindowsAndMessaging::DLGTEMPLATE, hwndowner: super::super::Foundation::HWND, dlgprc: super::super::UI::WindowsAndMessaging::DLGPROC) -> i32;
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_WindowsAndMessaging"))]
pub type PWLX_DIALOG_BOX_INDIRECT_PARAM = unsafe extern "system" fn(hwlx: super::super::Foundation::HANDLE, hinst: super::super::Foundation::HANDLE, hdialogtemplate: *mut super::super::UI::WindowsAndMessaging::DLGTEMPLATE, hwndowner: super::super::Foundation::HWND, dlgprc: super::super::UI::WindowsAndMessaging::DLGPROC, dwinitparam: super::super::Foundation::LPARAM) -> i32;
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_WindowsAndMessaging"))]
pub type PWLX_DIALOG_BOX_PARAM = unsafe extern "system" fn(hwlx: super::super::Foundation::HANDLE, hinst: super::super::Foundation::HANDLE, lpsztemplate: super::super::Foundation::PWSTR, hwndowner: super::super::Foundation::HWND, dlgprc: super::super::UI::WindowsAndMessaging::DLGPROC, dwinitparam: super::super::Foundation::LPARAM) -> i32;
#[cfg(feature = "Win32_Foundation")]
pub type PWLX_DISCONNECT = unsafe extern "system" fn() -> super::super::Foundation::BOOL;
#[cfg(feature = "Win32_Foundation")]
pub type PWLX_GET_OPTION = unsafe extern "system" fn(hwlx: super::super::Foundation::HANDLE, option: u32, value: *mut usize) -> super::super::Foundation::BOOL;
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_StationsAndDesktops"))]
pub type PWLX_GET_SOURCE_DESKTOP = unsafe extern "system" fn(hwlx: super::super::Foundation::HANDLE, ppdesktop: *mut *mut WLX_DESKTOP) -> super::super::Foundation::BOOL;
#[cfg(feature = "Win32_Foundation")]
pub type PWLX_MESSAGE_BOX = unsafe extern "system" fn(hwlx: super::super::Foundation::HANDLE, hwndowner: super::super::Foundation::HWND, lpsztext: super::super::Foundation::PWSTR, lpsztitle: super::super::Foundation::PWSTR, fustyle: u32) -> i32;
#[cfg(feature = "Win32_Foundation")]
pub type PWLX_QUERY_CLIENT_CREDENTIALS = unsafe extern "system" fn(pcred: *mut WLX_CLIENT_CREDENTIALS_INFO_V1_0) -> super::super::Foundation::BOOL;
#[cfg(feature = "Win32_Foundation")]
pub type PWLX_QUERY_CONSOLESWITCH_CREDENTIALS = unsafe extern "system" fn(pcred: *mut WLX_CONSOLESWITCH_CREDENTIALS_INFO_V1_0) -> u32;
#[cfg(feature = "Win32_Foundation")]
pub type PWLX_QUERY_IC_CREDENTIALS = unsafe extern "system" fn(pcred: *mut WLX_CLIENT_CREDENTIALS_INFO_V1_0) -> super::super::Foundation::BOOL;
#[cfg(feature = "Win32_Foundation")]
pub type PWLX_QUERY_TERMINAL_SERVICES_DATA = unsafe extern "system" fn(hwlx: super::super::Foundation::HANDLE, ptsdata: *mut WLX_TERMINAL_SERVICES_DATA, username: super::super::Foundation::PWSTR, domain: super::super::Foundation::PWSTR) -> u32;
#[cfg(feature = "Win32_Foundation")]
pub type PWLX_QUERY_TS_LOGON_CREDENTIALS = unsafe extern "system" fn(pcred: *mut WLX_CLIENT_CREDENTIALS_INFO_V2_0) -> super::super::Foundation::BOOL;
#[cfg(feature = "Win32_Foundation")]
pub type PWLX_SAS_NOTIFY = unsafe extern "system" fn(hwlx: super::super::Foundation::HANDLE, dwsastype: u32);
#[cfg(feature = "Win32_Foundation")]
pub type PWLX_SET_CONTEXT_POINTER = unsafe extern "system" fn(hwlx: super::super::Foundation::HANDLE, pwlxcontext: *mut ::core::ffi::c_void);
#[cfg(feature = "Win32_Foundation")]
pub type PWLX_SET_OPTION = unsafe extern "system" fn(hwlx: super::super::Foundation::HANDLE, option: u32, value: usize, oldvalue: *mut usize) -> super::super::Foundation::BOOL;
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_StationsAndDesktops"))]
pub type PWLX_SET_RETURN_DESKTOP = unsafe extern "system" fn(hwlx: super::super::Foundation::HANDLE, pdesktop: *mut WLX_DESKTOP) -> super::super::Foundation::BOOL;
#[cfg(feature = "Win32_Foundation")]
pub type PWLX_SET_TIMEOUT = unsafe extern "system" fn(hwlx: super::super::Foundation::HANDLE, timeout: u32) -> super::super::Foundation::BOOL;
#[cfg(feature = "Win32_Foundation")]
pub type PWLX_SWITCH_DESKTOP_TO_USER = unsafe extern "system" fn(hwlx: super::super::Foundation::HANDLE) -> i32;
#[cfg(feature = "Win32_Foundation")]
pub type PWLX_SWITCH_DESKTOP_TO_WINLOGON = unsafe extern "system" fn(hwlx: super::super::Foundation::HANDLE) -> i32;
#[cfg(feature = "Win32_Foundation")]
pub type PWLX_USE_CTRL_ALT_DEL = unsafe extern "system" fn(hwlx: super::super::Foundation::HANDLE);
#[cfg(feature = "Win32_Foundation")]
pub type PWLX_WIN31_MIGRATE = unsafe extern "system" fn(hwlx: super::super::Foundation::HANDLE);
pub const STATUSMSG_OPTION_NOANIMATION: u32 = 1u32;
pub const STATUSMSG_OPTION_SETFOREGROUND: u32 = 2u32;
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct WLX_CLIENT_CREDENTIALS_INFO_V1_0(i32);
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct WLX_CLIENT_CREDENTIALS_INFO_V2_0(i32);
pub const WLX_CONSOLESWITCHCREDENTIAL_TYPE_V1_0: u32 = 1u32;
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct WLX_CONSOLESWITCH_CREDENTIALS_INFO_V1_0(i32);
pub const WLX_CREATE_INSTANCE_ONLY: u32 = 1u32;
pub const WLX_CREATE_USER: u32 = 2u32;
pub const WLX_CREDENTIAL_TYPE_V1_0: u32 = 1u32;
pub const WLX_CREDENTIAL_TYPE_V2_0: u32 = 2u32;
pub const WLX_CURRENT_VERSION: u32 = 65540u32;
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_StationsAndDesktops"))]
#[repr(C)]
pub struct WLX_DESKTOP(i32);
pub const WLX_DESKTOP_HANDLE: u32 = 2u32;
pub const WLX_DESKTOP_NAME: u32 = 1u32;
pub const WLX_DIRECTORY_LENGTH: u32 = 256u32;
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_WindowsAndMessaging"))]
#[repr(C)]
pub struct WLX_DISPATCH_VERSION_1_0(i32);
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_StationsAndDesktops", feature = "Win32_UI_WindowsAndMessaging"))]
#[repr(C)]
pub struct WLX_DISPATCH_VERSION_1_1(i32);
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_StationsAndDesktops", feature = "Win32_UI_WindowsAndMessaging"))]
#[repr(C)]
pub struct WLX_DISPATCH_VERSION_1_2(i32);
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_StationsAndDesktops", feature = "Win32_UI_WindowsAndMessaging"))]
#[repr(C)]
pub struct WLX_DISPATCH_VERSION_1_3(i32);
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_StationsAndDesktops", feature = "Win32_UI_WindowsAndMessaging"))]
#[repr(C)]
pub struct WLX_DISPATCH_VERSION_1_4(i32);
pub const WLX_DLG_INPUT_TIMEOUT: u32 = 102u32;
pub const WLX_DLG_SAS: u32 = 101u32;
pub const WLX_DLG_SCREEN_SAVER_TIMEOUT: u32 = 103u32;
pub const WLX_DLG_USER_LOGOFF: u32 = 104u32;
pub const WLX_LOGON_OPT_NO_PROFILE: u32 = 1u32;
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct WLX_MPR_NOTIFY_INFO(i32);
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_StationsAndDesktops"))]
#[repr(C)]
pub struct WLX_NOTIFICATION_INFO(i32);
pub const WLX_OPTION_CONTEXT_POINTER: u32 = 2u32;
pub const WLX_OPTION_DISPATCH_TABLE_SIZE: u32 = 65539u32;
pub const WLX_OPTION_FORCE_LOGOFF_TIME: u32 = 4u32;
pub const WLX_OPTION_IGNORE_AUTO_LOGON: u32 = 8u32;
pub const WLX_OPTION_NO_SWITCH_ON_SAS: u32 = 9u32;
pub const WLX_OPTION_SMART_CARD_INFO: u32 = 65538u32;
pub const WLX_OPTION_SMART_CARD_PRESENT: u32 = 65537u32;
pub const WLX_OPTION_USE_CTRL_ALT_DEL: u32 = 1u32;
pub const WLX_OPTION_USE_SMART_CARD: u32 = 3u32;
pub const WLX_PROFILE_TYPE_V1_0: u32 = 1u32;
pub const WLX_PROFILE_TYPE_V2_0: u32 = 2u32;
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct WLX_PROFILE_V1_0(i32);
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct WLX_PROFILE_V2_0(i32);
pub const WLX_SAS_ACTION_DELAYED_FORCE_LOGOFF: u32 = 16u32;
pub const WLX_SAS_ACTION_FORCE_LOGOFF: u32 = 9u32;
pub const WLX_SAS_ACTION_LOCK_WKSTA: u32 = 3u32;
pub const WLX_SAS_ACTION_LOGOFF: u32 = 4u32;
pub const WLX_SAS_ACTION_LOGON: u32 = 1u32;
pub const WLX_SAS_ACTION_NONE: u32 = 2u32;
pub const WLX_SAS_ACTION_PWD_CHANGED: u32 = 6u32;
pub const WLX_SAS_ACTION_RECONNECTED: u32 = 15u32;
pub const WLX_SAS_ACTION_SHUTDOWN_HIBERNATE: u32 = 14u32;
pub const WLX_SAS_ACTION_SHUTDOWN_SLEEP: u32 = 12u32;
pub const WLX_SAS_ACTION_SHUTDOWN_SLEEP2: u32 = 13u32;
pub const WLX_SAS_ACTION_SWITCH_CONSOLE: u32 = 17u32;
pub const WLX_SAS_ACTION_TASKLIST: u32 = 7u32;
pub const WLX_SAS_ACTION_UNLOCK_WKSTA: u32 = 8u32;
pub const WLX_SAS_TYPE_AUTHENTICATED: u32 = 7u32;
pub const WLX_SAS_TYPE_CTRL_ALT_DEL: u32 = 1u32;
pub const WLX_SAS_TYPE_MAX_MSFT_VALUE: u32 = 127u32;
pub const WLX_SAS_TYPE_SCRNSVR_ACTIVITY: u32 = 3u32;
pub const WLX_SAS_TYPE_SCRNSVR_TIMEOUT: u32 = 2u32;
pub const WLX_SAS_TYPE_SC_FIRST_READER_ARRIVED: u32 = 8u32;
pub const WLX_SAS_TYPE_SC_INSERT: u32 = 5u32;
pub const WLX_SAS_TYPE_SC_LAST_READER_REMOVED: u32 = 9u32;
pub const WLX_SAS_TYPE_SC_REMOVE: u32 = 6u32;
pub const WLX_SAS_TYPE_SWITCHUSER: u32 = 10u32;
pub const WLX_SAS_TYPE_TIMEOUT: u32 = 0u32;
pub const WLX_SAS_TYPE_USER_LOGOFF: u32 = 4u32;
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct WLX_SC_NOTIFICATION_INFO(i32);
#[repr(transparent)]
pub struct WLX_SHUTDOWN_TYPE(pub u32);
pub const WLX_SAS_ACTION_SHUTDOWN: WLX_SHUTDOWN_TYPE = WLX_SHUTDOWN_TYPE(5u32);
pub const WLX_SAS_ACTION_SHUTDOWN_REBOOT: WLX_SHUTDOWN_TYPE = WLX_SHUTDOWN_TYPE(11u32);
pub const WLX_SAS_ACTION_SHUTDOWN_POWER_OFF: WLX_SHUTDOWN_TYPE = WLX_SHUTDOWN_TYPE(10u32);
#[repr(C)]
pub struct WLX_TERMINAL_SERVICES_DATA(i32);
pub const WLX_VERSION_1_0: u32 = 65536u32;
pub const WLX_VERSION_1_1: u32 = 65537u32;
pub const WLX_VERSION_1_2: u32 = 65538u32;
pub const WLX_VERSION_1_3: u32 = 65539u32;
pub const WLX_VERSION_1_4: u32 = 65540u32;
pub const WLX_WM_SAS: u32 = 1625u32;

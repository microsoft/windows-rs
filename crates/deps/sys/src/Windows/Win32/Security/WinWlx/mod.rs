#![allow(non_snake_case, non_camel_case_types)]
#[link(name = "windows")]
extern "system" {}
pub struct PFNMSGECALLBACK(i32);
pub struct PWLX_ASSIGN_SHELL_PROTECTION(i32);
pub struct PWLX_CHANGE_PASSWORD_NOTIFY(i32);
pub struct PWLX_CHANGE_PASSWORD_NOTIFY_EX(i32);
pub struct PWLX_CLOSE_USER_DESKTOP(i32);
pub struct PWLX_CREATE_USER_DESKTOP(i32);
pub struct PWLX_DIALOG_BOX(i32);
pub struct PWLX_DIALOG_BOX_INDIRECT(i32);
pub struct PWLX_DIALOG_BOX_INDIRECT_PARAM(i32);
pub struct PWLX_DIALOG_BOX_PARAM(i32);
pub struct PWLX_DISCONNECT(i32);
pub struct PWLX_GET_OPTION(i32);
pub struct PWLX_GET_SOURCE_DESKTOP(i32);
pub struct PWLX_MESSAGE_BOX(i32);
pub struct PWLX_QUERY_CLIENT_CREDENTIALS(i32);
pub struct PWLX_QUERY_CONSOLESWITCH_CREDENTIALS(i32);
pub struct PWLX_QUERY_IC_CREDENTIALS(i32);
pub struct PWLX_QUERY_TERMINAL_SERVICES_DATA(i32);
pub struct PWLX_QUERY_TS_LOGON_CREDENTIALS(i32);
pub struct PWLX_SAS_NOTIFY(i32);
pub struct PWLX_SET_CONTEXT_POINTER(i32);
pub struct PWLX_SET_OPTION(i32);
pub struct PWLX_SET_RETURN_DESKTOP(i32);
pub struct PWLX_SET_TIMEOUT(i32);
pub struct PWLX_SWITCH_DESKTOP_TO_USER(i32);
pub struct PWLX_SWITCH_DESKTOP_TO_WINLOGON(i32);
pub struct PWLX_USE_CTRL_ALT_DEL(i32);
pub struct PWLX_WIN31_MIGRATE(i32);
#[doc = "*Required features: `Win32_Security_WinWlx`*"]
pub const STATUSMSG_OPTION_NOANIMATION: u32 = 1u32;
#[doc = "*Required features: `Win32_Security_WinWlx`*"]
pub const STATUSMSG_OPTION_SETFOREGROUND: u32 = 2u32;
#[cfg(feature = "Win32_Foundation")]
pub struct WLX_CLIENT_CREDENTIALS_INFO_V1_0(i32);
#[cfg(feature = "Win32_Foundation")]
pub struct WLX_CLIENT_CREDENTIALS_INFO_V2_0(i32);
#[doc = "*Required features: `Win32_Security_WinWlx`*"]
pub const WLX_CONSOLESWITCHCREDENTIAL_TYPE_V1_0: u32 = 1u32;
#[cfg(feature = "Win32_Foundation")]
pub struct WLX_CONSOLESWITCH_CREDENTIALS_INFO_V1_0(i32);
#[doc = "*Required features: `Win32_Security_WinWlx`*"]
pub const WLX_CREATE_INSTANCE_ONLY: u32 = 1u32;
#[doc = "*Required features: `Win32_Security_WinWlx`*"]
pub const WLX_CREATE_USER: u32 = 2u32;
#[doc = "*Required features: `Win32_Security_WinWlx`*"]
pub const WLX_CREDENTIAL_TYPE_V1_0: u32 = 1u32;
#[doc = "*Required features: `Win32_Security_WinWlx`*"]
pub const WLX_CREDENTIAL_TYPE_V2_0: u32 = 2u32;
#[doc = "*Required features: `Win32_Security_WinWlx`*"]
pub const WLX_CURRENT_VERSION: u32 = 65540u32;
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_StationsAndDesktops"))]
pub struct WLX_DESKTOP(i32);
#[doc = "*Required features: `Win32_Security_WinWlx`*"]
pub const WLX_DESKTOP_HANDLE: u32 = 2u32;
#[doc = "*Required features: `Win32_Security_WinWlx`*"]
pub const WLX_DESKTOP_NAME: u32 = 1u32;
#[doc = "*Required features: `Win32_Security_WinWlx`*"]
pub const WLX_DIRECTORY_LENGTH: u32 = 256u32;
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_WindowsAndMessaging"))]
pub struct WLX_DISPATCH_VERSION_1_0(i32);
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_StationsAndDesktops", feature = "Win32_UI_WindowsAndMessaging"))]
pub struct WLX_DISPATCH_VERSION_1_1(i32);
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_StationsAndDesktops", feature = "Win32_UI_WindowsAndMessaging"))]
pub struct WLX_DISPATCH_VERSION_1_2(i32);
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_StationsAndDesktops", feature = "Win32_UI_WindowsAndMessaging"))]
pub struct WLX_DISPATCH_VERSION_1_3(i32);
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_StationsAndDesktops", feature = "Win32_UI_WindowsAndMessaging"))]
pub struct WLX_DISPATCH_VERSION_1_4(i32);
#[doc = "*Required features: `Win32_Security_WinWlx`*"]
pub const WLX_DLG_INPUT_TIMEOUT: u32 = 102u32;
#[doc = "*Required features: `Win32_Security_WinWlx`*"]
pub const WLX_DLG_SAS: u32 = 101u32;
#[doc = "*Required features: `Win32_Security_WinWlx`*"]
pub const WLX_DLG_SCREEN_SAVER_TIMEOUT: u32 = 103u32;
#[doc = "*Required features: `Win32_Security_WinWlx`*"]
pub const WLX_DLG_USER_LOGOFF: u32 = 104u32;
#[doc = "*Required features: `Win32_Security_WinWlx`*"]
pub const WLX_LOGON_OPT_NO_PROFILE: u32 = 1u32;
#[cfg(feature = "Win32_Foundation")]
pub struct WLX_MPR_NOTIFY_INFO(i32);
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_StationsAndDesktops"))]
pub struct WLX_NOTIFICATION_INFO(i32);
#[doc = "*Required features: `Win32_Security_WinWlx`*"]
pub const WLX_OPTION_CONTEXT_POINTER: u32 = 2u32;
#[doc = "*Required features: `Win32_Security_WinWlx`*"]
pub const WLX_OPTION_DISPATCH_TABLE_SIZE: u32 = 65539u32;
#[doc = "*Required features: `Win32_Security_WinWlx`*"]
pub const WLX_OPTION_FORCE_LOGOFF_TIME: u32 = 4u32;
#[doc = "*Required features: `Win32_Security_WinWlx`*"]
pub const WLX_OPTION_IGNORE_AUTO_LOGON: u32 = 8u32;
#[doc = "*Required features: `Win32_Security_WinWlx`*"]
pub const WLX_OPTION_NO_SWITCH_ON_SAS: u32 = 9u32;
#[doc = "*Required features: `Win32_Security_WinWlx`*"]
pub const WLX_OPTION_SMART_CARD_INFO: u32 = 65538u32;
#[doc = "*Required features: `Win32_Security_WinWlx`*"]
pub const WLX_OPTION_SMART_CARD_PRESENT: u32 = 65537u32;
#[doc = "*Required features: `Win32_Security_WinWlx`*"]
pub const WLX_OPTION_USE_CTRL_ALT_DEL: u32 = 1u32;
#[doc = "*Required features: `Win32_Security_WinWlx`*"]
pub const WLX_OPTION_USE_SMART_CARD: u32 = 3u32;
#[doc = "*Required features: `Win32_Security_WinWlx`*"]
pub const WLX_PROFILE_TYPE_V1_0: u32 = 1u32;
#[doc = "*Required features: `Win32_Security_WinWlx`*"]
pub const WLX_PROFILE_TYPE_V2_0: u32 = 2u32;
#[cfg(feature = "Win32_Foundation")]
pub struct WLX_PROFILE_V1_0(i32);
#[cfg(feature = "Win32_Foundation")]
pub struct WLX_PROFILE_V2_0(i32);
#[doc = "*Required features: `Win32_Security_WinWlx`*"]
pub const WLX_SAS_ACTION_DELAYED_FORCE_LOGOFF: u32 = 16u32;
#[doc = "*Required features: `Win32_Security_WinWlx`*"]
pub const WLX_SAS_ACTION_FORCE_LOGOFF: u32 = 9u32;
#[doc = "*Required features: `Win32_Security_WinWlx`*"]
pub const WLX_SAS_ACTION_LOCK_WKSTA: u32 = 3u32;
#[doc = "*Required features: `Win32_Security_WinWlx`*"]
pub const WLX_SAS_ACTION_LOGOFF: u32 = 4u32;
#[doc = "*Required features: `Win32_Security_WinWlx`*"]
pub const WLX_SAS_ACTION_LOGON: u32 = 1u32;
#[doc = "*Required features: `Win32_Security_WinWlx`*"]
pub const WLX_SAS_ACTION_NONE: u32 = 2u32;
#[doc = "*Required features: `Win32_Security_WinWlx`*"]
pub const WLX_SAS_ACTION_PWD_CHANGED: u32 = 6u32;
#[doc = "*Required features: `Win32_Security_WinWlx`*"]
pub const WLX_SAS_ACTION_RECONNECTED: u32 = 15u32;
#[doc = "*Required features: `Win32_Security_WinWlx`*"]
pub const WLX_SAS_ACTION_SHUTDOWN_HIBERNATE: u32 = 14u32;
#[doc = "*Required features: `Win32_Security_WinWlx`*"]
pub const WLX_SAS_ACTION_SHUTDOWN_SLEEP: u32 = 12u32;
#[doc = "*Required features: `Win32_Security_WinWlx`*"]
pub const WLX_SAS_ACTION_SHUTDOWN_SLEEP2: u32 = 13u32;
#[doc = "*Required features: `Win32_Security_WinWlx`*"]
pub const WLX_SAS_ACTION_SWITCH_CONSOLE: u32 = 17u32;
#[doc = "*Required features: `Win32_Security_WinWlx`*"]
pub const WLX_SAS_ACTION_TASKLIST: u32 = 7u32;
#[doc = "*Required features: `Win32_Security_WinWlx`*"]
pub const WLX_SAS_ACTION_UNLOCK_WKSTA: u32 = 8u32;
#[doc = "*Required features: `Win32_Security_WinWlx`*"]
pub const WLX_SAS_TYPE_AUTHENTICATED: u32 = 7u32;
#[doc = "*Required features: `Win32_Security_WinWlx`*"]
pub const WLX_SAS_TYPE_CTRL_ALT_DEL: u32 = 1u32;
#[doc = "*Required features: `Win32_Security_WinWlx`*"]
pub const WLX_SAS_TYPE_MAX_MSFT_VALUE: u32 = 127u32;
#[doc = "*Required features: `Win32_Security_WinWlx`*"]
pub const WLX_SAS_TYPE_SCRNSVR_ACTIVITY: u32 = 3u32;
#[doc = "*Required features: `Win32_Security_WinWlx`*"]
pub const WLX_SAS_TYPE_SCRNSVR_TIMEOUT: u32 = 2u32;
#[doc = "*Required features: `Win32_Security_WinWlx`*"]
pub const WLX_SAS_TYPE_SC_FIRST_READER_ARRIVED: u32 = 8u32;
#[doc = "*Required features: `Win32_Security_WinWlx`*"]
pub const WLX_SAS_TYPE_SC_INSERT: u32 = 5u32;
#[doc = "*Required features: `Win32_Security_WinWlx`*"]
pub const WLX_SAS_TYPE_SC_LAST_READER_REMOVED: u32 = 9u32;
#[doc = "*Required features: `Win32_Security_WinWlx`*"]
pub const WLX_SAS_TYPE_SC_REMOVE: u32 = 6u32;
#[doc = "*Required features: `Win32_Security_WinWlx`*"]
pub const WLX_SAS_TYPE_SWITCHUSER: u32 = 10u32;
#[doc = "*Required features: `Win32_Security_WinWlx`*"]
pub const WLX_SAS_TYPE_TIMEOUT: u32 = 0u32;
#[doc = "*Required features: `Win32_Security_WinWlx`*"]
pub const WLX_SAS_TYPE_USER_LOGOFF: u32 = 4u32;
#[cfg(feature = "Win32_Foundation")]
pub struct WLX_SC_NOTIFICATION_INFO(i32);
pub struct WLX_SHUTDOWN_TYPE(i32);
pub struct WLX_TERMINAL_SERVICES_DATA(i32);
#[doc = "*Required features: `Win32_Security_WinWlx`*"]
pub const WLX_VERSION_1_0: u32 = 65536u32;
#[doc = "*Required features: `Win32_Security_WinWlx`*"]
pub const WLX_VERSION_1_1: u32 = 65537u32;
#[doc = "*Required features: `Win32_Security_WinWlx`*"]
pub const WLX_VERSION_1_2: u32 = 65538u32;
#[doc = "*Required features: `Win32_Security_WinWlx`*"]
pub const WLX_VERSION_1_3: u32 = 65539u32;
#[doc = "*Required features: `Win32_Security_WinWlx`*"]
pub const WLX_VERSION_1_4: u32 = 65540u32;
#[doc = "*Required features: `Win32_Security_WinWlx`*"]
pub const WLX_WM_SAS: u32 = 1625u32;

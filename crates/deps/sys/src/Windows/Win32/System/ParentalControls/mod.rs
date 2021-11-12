#![allow(non_snake_case, non_camel_case_types)]
#[link(name = "windows")]
extern "system" {}
#[doc = "*Required features: `Win32_System_ParentalControls`*"]
pub const ARRAY_SEP_CHAR: u32 = 9u32;
#[doc = "*Required features: `Win32_System_ParentalControls`*"]
pub const FACILITY_WPC: u32 = 2457u32;
#[repr(transparent)]
pub struct IWPCGamesSettings(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IWPCProviderConfig(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IWPCProviderState(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IWPCProviderSupport(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IWPCSettings(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IWPCWebSettings(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IWindowsParentalControls(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IWindowsParentalControlsCore(pub *mut ::core::ffi::c_void);
#[doc = "*Required features: `Win32_System_ParentalControls`*"]
pub const MSG_Event_AppBlocked: i32 = -1342177264i32;
#[doc = "*Required features: `Win32_System_ParentalControls`*"]
pub const MSG_Event_AppOverride: i32 = -1342177263i32;
#[doc = "*Required features: `Win32_System_ParentalControls`*"]
pub const MSG_Event_Application: i32 = -1342177260i32;
#[doc = "*Required features: `Win32_System_ParentalControls`*"]
pub const MSG_Event_ComputerUsage: i32 = -1342177259i32;
#[doc = "*Required features: `Win32_System_ParentalControls`*"]
pub const MSG_Event_ContentUsage: i32 = -1342177258i32;
#[doc = "*Required features: `Win32_System_ParentalControls`*"]
pub const MSG_Event_Custom: i32 = -1342177267i32;
#[doc = "*Required features: `Win32_System_ParentalControls`*"]
pub const MSG_Event_EmailContact: i32 = -1342177266i32;
#[doc = "*Required features: `Win32_System_ParentalControls`*"]
pub const MSG_Event_EmailReceived: i32 = -1342177276i32;
#[doc = "*Required features: `Win32_System_ParentalControls`*"]
pub const MSG_Event_EmailSent: i32 = -1342177275i32;
#[doc = "*Required features: `Win32_System_ParentalControls`*"]
pub const MSG_Event_FileDownload: i32 = -1342177270i32;
#[doc = "*Required features: `Win32_System_ParentalControls`*"]
pub const MSG_Event_GameStart: i32 = -1342177278i32;
#[doc = "*Required features: `Win32_System_ParentalControls`*"]
pub const MSG_Event_IMContact: i32 = -1342177265i32;
#[doc = "*Required features: `Win32_System_ParentalControls`*"]
pub const MSG_Event_IMFeature: i32 = -1342177269i32;
#[doc = "*Required features: `Win32_System_ParentalControls`*"]
pub const MSG_Event_IMInvitation: i32 = -1342177273i32;
#[doc = "*Required features: `Win32_System_ParentalControls`*"]
pub const MSG_Event_IMJoin: i32 = -1342177272i32;
#[doc = "*Required features: `Win32_System_ParentalControls`*"]
pub const MSG_Event_IMLeave: i32 = -1342177271i32;
#[doc = "*Required features: `Win32_System_ParentalControls`*"]
pub const MSG_Event_MediaPlayback: i32 = -1342177274i32;
#[doc = "*Required features: `Win32_System_ParentalControls`*"]
pub const MSG_Event_SettingChange: i32 = -1342177279i32;
#[doc = "*Required features: `Win32_System_ParentalControls`*"]
pub const MSG_Event_UrlVisit: i32 = -1342177277i32;
#[doc = "*Required features: `Win32_System_ParentalControls`*"]
pub const MSG_Event_WebOverride: i32 = -1342177262i32;
#[doc = "*Required features: `Win32_System_ParentalControls`*"]
pub const MSG_Event_WebsiteVisit: i32 = -1342177261i32;
#[doc = "*Required features: `Win32_System_ParentalControls`*"]
pub const MSG_Keyword_ThirdParty: i32 = 268435462i32;
#[doc = "*Required features: `Win32_System_ParentalControls`*"]
pub const MSG_Keyword_WPC: i32 = 268435461i32;
#[doc = "*Required features: `Win32_System_ParentalControls`*"]
pub const MSG_Opcode_Launch: i32 = 805306390i32;
#[doc = "*Required features: `Win32_System_ParentalControls`*"]
pub const MSG_Opcode_Locate: i32 = 805306388i32;
#[doc = "*Required features: `Win32_System_ParentalControls`*"]
pub const MSG_Opcode_Modify: i32 = 805306389i32;
#[doc = "*Required features: `Win32_System_ParentalControls`*"]
pub const MSG_Opcode_System: i32 = 805306391i32;
#[doc = "*Required features: `Win32_System_ParentalControls`*"]
pub const MSG_Opcode_Web: i32 = 805306392i32;
#[doc = "*Required features: `Win32_System_ParentalControls`*"]
pub const MSG_Publisher_Name: i32 = -1879048191i32;
#[doc = "*Required features: `Win32_System_ParentalControls`*"]
pub const MSG_Task_AppBlocked: i32 = 1879048208i32;
#[doc = "*Required features: `Win32_System_ParentalControls`*"]
pub const MSG_Task_AppOverride: i32 = 1879048209i32;
#[doc = "*Required features: `Win32_System_ParentalControls`*"]
pub const MSG_Task_Application: i32 = 1879048212i32;
#[doc = "*Required features: `Win32_System_ParentalControls`*"]
pub const MSG_Task_ComputerUsage: i32 = 1879048213i32;
#[doc = "*Required features: `Win32_System_ParentalControls`*"]
pub const MSG_Task_ContentUsage: i32 = 1879048214i32;
#[doc = "*Required features: `Win32_System_ParentalControls`*"]
pub const MSG_Task_Custom: i32 = 1879048205i32;
#[doc = "*Required features: `Win32_System_ParentalControls`*"]
pub const MSG_Task_EmailContact: i32 = 1879048206i32;
#[doc = "*Required features: `Win32_System_ParentalControls`*"]
pub const MSG_Task_EmailReceived: i32 = 1879048196i32;
#[doc = "*Required features: `Win32_System_ParentalControls`*"]
pub const MSG_Task_EmailSent: i32 = 1879048197i32;
#[doc = "*Required features: `Win32_System_ParentalControls`*"]
pub const MSG_Task_FileDownload: i32 = 1879048202i32;
#[doc = "*Required features: `Win32_System_ParentalControls`*"]
pub const MSG_Task_GameStart: i32 = 1879048194i32;
#[doc = "*Required features: `Win32_System_ParentalControls`*"]
pub const MSG_Task_IMContact: i32 = 1879048207i32;
#[doc = "*Required features: `Win32_System_ParentalControls`*"]
pub const MSG_Task_IMFeature: i32 = 1879048203i32;
#[doc = "*Required features: `Win32_System_ParentalControls`*"]
pub const MSG_Task_IMInvitation: i32 = 1879048199i32;
#[doc = "*Required features: `Win32_System_ParentalControls`*"]
pub const MSG_Task_IMJoin: i32 = 1879048200i32;
#[doc = "*Required features: `Win32_System_ParentalControls`*"]
pub const MSG_Task_IMLeave: i32 = 1879048201i32;
#[doc = "*Required features: `Win32_System_ParentalControls`*"]
pub const MSG_Task_MediaPlayback: i32 = 1879048198i32;
#[doc = "*Required features: `Win32_System_ParentalControls`*"]
pub const MSG_Task_SettingChange: i32 = 1879048193i32;
#[doc = "*Required features: `Win32_System_ParentalControls`*"]
pub const MSG_Task_UrlVisit: i32 = 1879048195i32;
#[doc = "*Required features: `Win32_System_ParentalControls`*"]
pub const MSG_Task_WebOverride: i32 = 1879048210i32;
#[doc = "*Required features: `Win32_System_ParentalControls`*"]
pub const MSG_Task_WebsiteVisit: i32 = 1879048211i32;
#[doc = "*Required features: `Win32_System_ParentalControls`*"]
pub const WPCCHANNEL: u32 = 16u32;
#[doc = "*Required features: `Win32_System_ParentalControls`*"]
pub const WPCEVENT_APPLICATION_value: u32 = 20u32;
#[doc = "*Required features: `Win32_System_ParentalControls`*"]
pub const WPCEVENT_APPOVERRIDE_value: u32 = 17u32;
#[doc = "*Required features: `Win32_System_ParentalControls`*"]
pub const WPCEVENT_COMPUTERUSAGE_value: u32 = 21u32;
#[doc = "*Required features: `Win32_System_ParentalControls`*"]
pub const WPCEVENT_CONTENTUSAGE_value: u32 = 22u32;
#[doc = "*Required features: `Win32_System_ParentalControls`*"]
pub const WPCEVENT_CUSTOM_value: u32 = 13u32;
#[doc = "*Required features: `Win32_System_ParentalControls`*"]
pub const WPCEVENT_EMAIL_CONTACT_value: u32 = 14u32;
#[doc = "*Required features: `Win32_System_ParentalControls`*"]
pub const WPCEVENT_EMAIL_RECEIVED_value: u32 = 4u32;
#[doc = "*Required features: `Win32_System_ParentalControls`*"]
pub const WPCEVENT_EMAIL_SENT_value: u32 = 5u32;
#[doc = "*Required features: `Win32_System_ParentalControls`*"]
pub const WPCEVENT_GAME_START_value: u32 = 2u32;
#[doc = "*Required features: `Win32_System_ParentalControls`*"]
pub const WPCEVENT_IM_CONTACT_value: u32 = 15u32;
#[doc = "*Required features: `Win32_System_ParentalControls`*"]
pub const WPCEVENT_IM_FEATURE_value: u32 = 11u32;
#[doc = "*Required features: `Win32_System_ParentalControls`*"]
pub const WPCEVENT_IM_INVITATION_value: u32 = 7u32;
#[doc = "*Required features: `Win32_System_ParentalControls`*"]
pub const WPCEVENT_IM_JOIN_value: u32 = 8u32;
#[doc = "*Required features: `Win32_System_ParentalControls`*"]
pub const WPCEVENT_IM_LEAVE_value: u32 = 9u32;
#[doc = "*Required features: `Win32_System_ParentalControls`*"]
pub const WPCEVENT_MEDIA_PLAYBACK_value: u32 = 6u32;
#[doc = "*Required features: `Win32_System_ParentalControls`*"]
pub const WPCEVENT_SYSTEM_APPBLOCKED_value: u32 = 16u32;
#[doc = "*Required features: `Win32_System_ParentalControls`*"]
pub const WPCEVENT_SYS_SETTINGCHANGE_value: u32 = 1u32;
#[doc = "*Required features: `Win32_System_ParentalControls`*"]
pub const WPCEVENT_WEBOVERRIDE_value: u32 = 18u32;
#[doc = "*Required features: `Win32_System_ParentalControls`*"]
pub const WPCEVENT_WEB_FILEDOWNLOAD_value: u32 = 10u32;
#[doc = "*Required features: `Win32_System_ParentalControls`*"]
pub const WPCEVENT_WEB_URLVISIT_value: u32 = 3u32;
#[doc = "*Required features: `Win32_System_ParentalControls`*"]
pub const WPCEVENT_WEB_WEBSITEVISIT_value: u32 = 19u32;
pub struct WPCFLAG_IM_FEATURE(i32);
pub struct WPCFLAG_IM_LEAVE(i32);
pub struct WPCFLAG_ISBLOCKED(i32);
pub struct WPCFLAG_LOGOFF_TYPE(i32);
pub struct WPCFLAG_OVERRIDE(i32);
pub struct WPCFLAG_RESTRICTION(i32);
pub struct WPCFLAG_VISIBILITY(i32);
pub struct WPCFLAG_WEB_SETTING(i32);
pub const WPCPROV: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 17367141, data2: 46183, data3: 17667, data4: [155, 40, 83, 55, 102, 118, 16, 135] };
#[doc = "*Required features: `Win32_System_ParentalControls`*"]
pub const WPCPROV_KEYWORD_ThirdParty: u32 = 32u32;
#[doc = "*Required features: `Win32_System_ParentalControls`*"]
pub const WPCPROV_KEYWORD_WPC: u32 = 16u32;
#[doc = "*Required features: `Win32_System_ParentalControls`*"]
pub const WPCPROV_TASK_AppBlocked: u32 = 16u32;
#[doc = "*Required features: `Win32_System_ParentalControls`*"]
pub const WPCPROV_TASK_AppOverride: u32 = 17u32;
#[doc = "*Required features: `Win32_System_ParentalControls`*"]
pub const WPCPROV_TASK_Application: u32 = 20u32;
#[doc = "*Required features: `Win32_System_ParentalControls`*"]
pub const WPCPROV_TASK_ComputerUsage: u32 = 21u32;
#[doc = "*Required features: `Win32_System_ParentalControls`*"]
pub const WPCPROV_TASK_ContentUsage: u32 = 22u32;
#[doc = "*Required features: `Win32_System_ParentalControls`*"]
pub const WPCPROV_TASK_Custom: u32 = 13u32;
#[doc = "*Required features: `Win32_System_ParentalControls`*"]
pub const WPCPROV_TASK_EmailContact: u32 = 14u32;
#[doc = "*Required features: `Win32_System_ParentalControls`*"]
pub const WPCPROV_TASK_EmailReceived: u32 = 4u32;
#[doc = "*Required features: `Win32_System_ParentalControls`*"]
pub const WPCPROV_TASK_EmailSent: u32 = 5u32;
#[doc = "*Required features: `Win32_System_ParentalControls`*"]
pub const WPCPROV_TASK_FileDownload: u32 = 10u32;
#[doc = "*Required features: `Win32_System_ParentalControls`*"]
pub const WPCPROV_TASK_GameStart: u32 = 2u32;
#[doc = "*Required features: `Win32_System_ParentalControls`*"]
pub const WPCPROV_TASK_IMContact: u32 = 15u32;
#[doc = "*Required features: `Win32_System_ParentalControls`*"]
pub const WPCPROV_TASK_IMFeature: u32 = 11u32;
#[doc = "*Required features: `Win32_System_ParentalControls`*"]
pub const WPCPROV_TASK_IMInvitation: u32 = 7u32;
#[doc = "*Required features: `Win32_System_ParentalControls`*"]
pub const WPCPROV_TASK_IMJoin: u32 = 8u32;
#[doc = "*Required features: `Win32_System_ParentalControls`*"]
pub const WPCPROV_TASK_IMLeave: u32 = 9u32;
#[doc = "*Required features: `Win32_System_ParentalControls`*"]
pub const WPCPROV_TASK_MediaPlayback: u32 = 6u32;
#[doc = "*Required features: `Win32_System_ParentalControls`*"]
pub const WPCPROV_TASK_SettingChange: u32 = 1u32;
#[doc = "*Required features: `Win32_System_ParentalControls`*"]
pub const WPCPROV_TASK_UrlVisit: u32 = 3u32;
#[doc = "*Required features: `Win32_System_ParentalControls`*"]
pub const WPCPROV_TASK_WebOverride: u32 = 18u32;
#[doc = "*Required features: `Win32_System_ParentalControls`*"]
pub const WPCPROV_TASK_WebsiteVisit: u32 = 19u32;
#[doc = "*Required features: `Win32_System_ParentalControls`*"]
pub const WPC_APP_LAUNCH: u32 = 22u32;
pub struct WPC_ARGS_APPLICATIONEVENT(i32);
pub struct WPC_ARGS_APPOVERRIDEEVENT(i32);
pub struct WPC_ARGS_COMPUTERUSAGEEVENT(i32);
pub struct WPC_ARGS_CONTENTUSAGEEVENT(i32);
pub struct WPC_ARGS_CONVERSATIONINITEVENT(i32);
pub struct WPC_ARGS_CONVERSATIONJOINEVENT(i32);
pub struct WPC_ARGS_CONVERSATIONLEAVEEVENT(i32);
pub struct WPC_ARGS_CUSTOMEVENT(i32);
pub struct WPC_ARGS_EMAILCONTACTEVENT(i32);
pub struct WPC_ARGS_EMAILRECEIEVEDEVENT(i32);
pub struct WPC_ARGS_EMAILSENTEVENT(i32);
pub struct WPC_ARGS_FILEDOWNLOADEVENT(i32);
pub struct WPC_ARGS_GAMESTARTEVENT(i32);
pub struct WPC_ARGS_IMCONTACTEVENT(i32);
pub struct WPC_ARGS_IMFEATUREEVENT(i32);
pub struct WPC_ARGS_MEDIADOWNLOADEVENT(i32);
pub struct WPC_ARGS_MEDIAPLAYBACKEVENT(i32);
pub struct WPC_ARGS_SAFERAPPBLOCKED(i32);
pub struct WPC_ARGS_SETTINGSCHANGEEVENT(i32);
pub struct WPC_ARGS_URLVISITEVENT(i32);
pub struct WPC_ARGS_WEBOVERRIDEEVENT(i32);
pub struct WPC_ARGS_WEBSITEVISITEVENT(i32);
pub struct WPC_MEDIA_EXPLICIT(i32);
pub struct WPC_MEDIA_TYPE(i32);
pub struct WPC_SETTINGS(i32);
#[doc = "*Required features: `Win32_System_ParentalControls`*"]
pub const WPC_SETTINGS_LOCATE: u32 = 20u32;
#[doc = "*Required features: `Win32_System_ParentalControls`*"]
pub const WPC_SETTINGS_MODIFY: u32 = 21u32;
#[doc = "*Required features: `Win32_System_ParentalControls`*"]
pub const WPC_SYSTEM: u32 = 23u32;
#[doc = "*Required features: `Win32_System_ParentalControls`*"]
pub const WPC_WEB: u32 = 24u32;
pub struct WindowsParentalControls(i32);
pub struct WpcProviderSupport(i32);
pub struct WpcSettingsProvider(i32);

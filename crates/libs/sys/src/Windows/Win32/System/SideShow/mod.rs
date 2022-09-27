pub type ISideShowBulkCapabilities = *mut ::core::ffi::c_void;
pub type ISideShowCapabilities = *mut ::core::ffi::c_void;
pub type ISideShowCapabilitiesCollection = *mut ::core::ffi::c_void;
pub type ISideShowContent = *mut ::core::ffi::c_void;
pub type ISideShowContentManager = *mut ::core::ffi::c_void;
pub type ISideShowEvents = *mut ::core::ffi::c_void;
pub type ISideShowKeyCollection = *mut ::core::ffi::c_void;
pub type ISideShowNotification = *mut ::core::ffi::c_void;
pub type ISideShowNotificationManager = *mut ::core::ffi::c_void;
pub type ISideShowPropVariantCollection = *mut ::core::ffi::c_void;
pub type ISideShowSession = *mut ::core::ffi::c_void;
#[doc = "*Required features: `\"Win32_System_SideShow\"`*"]
pub const CONTENT_ID_GLANCE: u32 = 0u32;
#[doc = "*Required features: `\"Win32_System_SideShow\"`*"]
pub const CONTENT_ID_HOME: u32 = 1u32;
pub const GUID_DEVINTERFACE_SIDESHOW: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x152e5811_feb9_4b00_90f4_d32947ae1681);
pub const SIDESHOW_APPLICATION_EVENT: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x4cb572fa_1d3b_49b3_a17a_2e6bff052854);
#[doc = "*Required features: `\"Win32_System_SideShow\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const SIDESHOW_CAPABILITY_CLIENT_AREA_HEIGHT: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_sys::core::GUID::from_u128(0x8abc88a8_857b_4ad7_a35a_b5942f492b99), pid: 16u32 };
#[doc = "*Required features: `\"Win32_System_SideShow\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const SIDESHOW_CAPABILITY_CLIENT_AREA_WIDTH: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_sys::core::GUID::from_u128(0x8abc88a8_857b_4ad7_a35a_b5942f492b99), pid: 15u32 };
#[doc = "*Required features: `\"Win32_System_SideShow\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const SIDESHOW_CAPABILITY_COLOR_DEPTH: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_sys::core::GUID::from_u128(0x8abc88a8_857b_4ad7_a35a_b5942f492b99), pid: 5u32 };
#[doc = "*Required features: `\"Win32_System_SideShow\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const SIDESHOW_CAPABILITY_COLOR_TYPE: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_sys::core::GUID::from_u128(0x8abc88a8_857b_4ad7_a35a_b5942f492b99), pid: 6u32 };
#[doc = "*Required features: `\"Win32_System_SideShow\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const SIDESHOW_CAPABILITY_CURRENT_LANGUAGE: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_sys::core::GUID::from_u128(0x8abc88a8_857b_4ad7_a35a_b5942f492b99), pid: 9u32 };
#[doc = "*Required features: `\"Win32_System_SideShow\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const SIDESHOW_CAPABILITY_DATA_CACHE: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_sys::core::GUID::from_u128(0x8abc88a8_857b_4ad7_a35a_b5942f492b99), pid: 7u32 };
#[doc = "*Required features: `\"Win32_System_SideShow\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const SIDESHOW_CAPABILITY_DEVICE_ID: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_sys::core::GUID::from_u128(0x8abc88a8_857b_4ad7_a35a_b5942f492b99), pid: 1u32 };
pub const SIDESHOW_CAPABILITY_DEVICE_PROPERTIES: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x8abc88a8_857b_4ad7_a35a_b5942f492b99);
#[doc = "*Required features: `\"Win32_System_SideShow\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const SIDESHOW_CAPABILITY_SCREEN_HEIGHT: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_sys::core::GUID::from_u128(0x8abc88a8_857b_4ad7_a35a_b5942f492b99), pid: 4u32 };
#[doc = "*Required features: `\"Win32_System_SideShow\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const SIDESHOW_CAPABILITY_SCREEN_TYPE: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_sys::core::GUID::from_u128(0x8abc88a8_857b_4ad7_a35a_b5942f492b99), pid: 2u32 };
#[doc = "*Required features: `\"Win32_System_SideShow\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const SIDESHOW_CAPABILITY_SCREEN_WIDTH: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_sys::core::GUID::from_u128(0x8abc88a8_857b_4ad7_a35a_b5942f492b99), pid: 3u32 };
#[doc = "*Required features: `\"Win32_System_SideShow\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const SIDESHOW_CAPABILITY_SUPPORTED_IMAGE_FORMATS: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_sys::core::GUID::from_u128(0x8abc88a8_857b_4ad7_a35a_b5942f492b99), pid: 14u32 };
#[doc = "*Required features: `\"Win32_System_SideShow\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const SIDESHOW_CAPABILITY_SUPPORTED_LANGUAGES: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_sys::core::GUID::from_u128(0x8abc88a8_857b_4ad7_a35a_b5942f492b99), pid: 8u32 };
#[doc = "*Required features: `\"Win32_System_SideShow\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const SIDESHOW_CAPABILITY_SUPPORTED_THEMES: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows_sys::core::GUID::from_u128(0x8abc88a8_857b_4ad7_a35a_b5942f492b99), pid: 10u32 };
pub const SIDESHOW_CONTENT_MISSING_EVENT: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x5007fba8_d313_439f_bea2_a50201d3e9a8);
pub const SIDESHOW_ENDPOINT_ICAL: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x4dff36b5_9dde_4f76_9a2a_96435047063d);
pub const SIDESHOW_ENDPOINT_SIMPLE_CONTENT_FORMAT: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0xa9a5353f_2d4b_47ce_93ee_759f3a7dda4f);
#[doc = "*Required features: `\"Win32_System_SideShow\"`*"]
pub const SIDESHOW_EVENTID_APPLICATION_ENTER: u32 = 4294901760u32;
#[doc = "*Required features: `\"Win32_System_SideShow\"`*"]
pub const SIDESHOW_EVENTID_APPLICATION_EXIT: u32 = 4294901761u32;
pub const SIDESHOW_NEW_EVENT_DATA_AVAILABLE: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x57813854_2fc1_411c_a59f_f24927608804);
pub const SIDESHOW_USER_CHANGE_REQUEST_EVENT: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x5009673c_3f7d_4c7e_9971_eaa2e91f1575);
pub const SideShowKeyCollection: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0xdfbbdbf8_18de_49b8_83dc_ebc727c62d94);
pub const SideShowNotification: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0x0ce3e86f_d5cd_4525_a766_1abab1a752f5);
pub const SideShowPropVariantCollection: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0xe640f415_539e_4923_96cd_5f093bc250cd);
pub const SideShowSession: ::windows_sys::core::GUID = ::windows_sys::core::GUID::from_u128(0xe20543b9_f785_4ea2_981e_c4ffa76bbc7c);
#[doc = "*Required features: `\"Win32_System_SideShow\"`*"]
pub const VERSION_1_WINDOWS_7: u32 = 0u32;
#[doc = "*Required features: `\"Win32_System_SideShow\"`*"]
pub type SCF_BUTTON_IDS = i32;
#[doc = "*Required features: `\"Win32_System_SideShow\"`*"]
pub const SCF_BUTTON_MENU: SCF_BUTTON_IDS = 1i32;
#[doc = "*Required features: `\"Win32_System_SideShow\"`*"]
pub const SCF_BUTTON_SELECT: SCF_BUTTON_IDS = 2i32;
#[doc = "*Required features: `\"Win32_System_SideShow\"`*"]
pub const SCF_BUTTON_UP: SCF_BUTTON_IDS = 3i32;
#[doc = "*Required features: `\"Win32_System_SideShow\"`*"]
pub const SCF_BUTTON_DOWN: SCF_BUTTON_IDS = 4i32;
#[doc = "*Required features: `\"Win32_System_SideShow\"`*"]
pub const SCF_BUTTON_LEFT: SCF_BUTTON_IDS = 5i32;
#[doc = "*Required features: `\"Win32_System_SideShow\"`*"]
pub const SCF_BUTTON_RIGHT: SCF_BUTTON_IDS = 6i32;
#[doc = "*Required features: `\"Win32_System_SideShow\"`*"]
pub const SCF_BUTTON_PLAY: SCF_BUTTON_IDS = 7i32;
#[doc = "*Required features: `\"Win32_System_SideShow\"`*"]
pub const SCF_BUTTON_PAUSE: SCF_BUTTON_IDS = 8i32;
#[doc = "*Required features: `\"Win32_System_SideShow\"`*"]
pub const SCF_BUTTON_FASTFORWARD: SCF_BUTTON_IDS = 9i32;
#[doc = "*Required features: `\"Win32_System_SideShow\"`*"]
pub const SCF_BUTTON_REWIND: SCF_BUTTON_IDS = 10i32;
#[doc = "*Required features: `\"Win32_System_SideShow\"`*"]
pub const SCF_BUTTON_STOP: SCF_BUTTON_IDS = 11i32;
#[doc = "*Required features: `\"Win32_System_SideShow\"`*"]
pub const SCF_BUTTON_BACK: SCF_BUTTON_IDS = 65280i32;
#[doc = "*Required features: `\"Win32_System_SideShow\"`*"]
pub type SCF_EVENT_IDS = i32;
#[doc = "*Required features: `\"Win32_System_SideShow\"`*"]
pub const SCF_EVENT_NAVIGATION: SCF_EVENT_IDS = 1i32;
#[doc = "*Required features: `\"Win32_System_SideShow\"`*"]
pub const SCF_EVENT_MENUACTION: SCF_EVENT_IDS = 2i32;
#[doc = "*Required features: `\"Win32_System_SideShow\"`*"]
pub const SCF_EVENT_CONTEXTMENU: SCF_EVENT_IDS = 3i32;
#[doc = "*Required features: `\"Win32_System_SideShow\"`*"]
pub type SIDESHOW_COLOR_TYPE = i32;
#[doc = "*Required features: `\"Win32_System_SideShow\"`*"]
pub const SIDESHOW_COLOR_TYPE_COLOR: SIDESHOW_COLOR_TYPE = 0i32;
#[doc = "*Required features: `\"Win32_System_SideShow\"`*"]
pub const SIDESHOW_COLOR_TYPE_GREYSCALE: SIDESHOW_COLOR_TYPE = 1i32;
#[doc = "*Required features: `\"Win32_System_SideShow\"`*"]
pub const SIDESHOW_COLOR_TYPE_BLACK_AND_WHITE: SIDESHOW_COLOR_TYPE = 2i32;
#[doc = "*Required features: `\"Win32_System_SideShow\"`*"]
pub type SIDESHOW_SCREEN_TYPE = i32;
#[doc = "*Required features: `\"Win32_System_SideShow\"`*"]
pub const SIDESHOW_SCREEN_TYPE_BITMAP: SIDESHOW_SCREEN_TYPE = 0i32;
#[doc = "*Required features: `\"Win32_System_SideShow\"`*"]
pub const SIDESHOW_SCREEN_TYPE_TEXT: SIDESHOW_SCREEN_TYPE = 1i32;
#[repr(C, packed(1))]
#[doc = "*Required features: `\"Win32_System_SideShow\"`*"]
pub struct APPLICATION_EVENT_DATA {
    pub cbApplicationEventData: u32,
    pub ApplicationId: ::windows_sys::core::GUID,
    pub EndpointId: ::windows_sys::core::GUID,
    pub dwEventId: u32,
    pub cbEventData: u32,
    pub bEventData: [u8; 1],
}
impl ::core::marker::Copy for APPLICATION_EVENT_DATA {}
impl ::core::clone::Clone for APPLICATION_EVENT_DATA {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C, packed(1))]
#[doc = "*Required features: `\"Win32_System_SideShow\"`*"]
pub struct CONTENT_MISSING_EVENT_DATA {
    pub cbContentMissingEventData: u32,
    pub ApplicationId: ::windows_sys::core::GUID,
    pub EndpointId: ::windows_sys::core::GUID,
    pub ContentId: u32,
}
impl ::core::marker::Copy for CONTENT_MISSING_EVENT_DATA {}
impl ::core::clone::Clone for CONTENT_MISSING_EVENT_DATA {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C, packed(1))]
#[doc = "*Required features: `\"Win32_System_SideShow\"`*"]
pub struct DEVICE_USER_CHANGE_EVENT_DATA {
    pub cbDeviceUserChangeEventData: u32,
    pub wszUser: u16,
}
impl ::core::marker::Copy for DEVICE_USER_CHANGE_EVENT_DATA {}
impl ::core::clone::Clone for DEVICE_USER_CHANGE_EVENT_DATA {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C, packed(1))]
#[doc = "*Required features: `\"Win32_System_SideShow\"`*"]
pub struct EVENT_DATA_HEADER {
    pub cbEventDataHeader: u32,
    pub guidEventType: ::windows_sys::core::GUID,
    pub dwVersion: u32,
    pub cbEventDataSid: u32,
}
impl ::core::marker::Copy for EVENT_DATA_HEADER {}
impl ::core::clone::Clone for EVENT_DATA_HEADER {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C, packed(1))]
#[doc = "*Required features: `\"Win32_System_SideShow\"`*"]
pub struct NEW_EVENT_DATA_AVAILABLE {
    pub cbNewEventDataAvailable: u32,
    pub dwVersion: u32,
}
impl ::core::marker::Copy for NEW_EVENT_DATA_AVAILABLE {}
impl ::core::clone::Clone for NEW_EVENT_DATA_AVAILABLE {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_SideShow\"`*"]
pub struct SCF_CONTEXTMENU_EVENT {
    pub PreviousPage: u32,
    pub TargetPage: u32,
    pub PreviousItemId: u32,
    pub MenuPage: u32,
    pub MenuItemId: u32,
}
impl ::core::marker::Copy for SCF_CONTEXTMENU_EVENT {}
impl ::core::clone::Clone for SCF_CONTEXTMENU_EVENT {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_SideShow\"`*"]
pub struct SCF_EVENT_HEADER {
    pub PreviousPage: u32,
    pub TargetPage: u32,
}
impl ::core::marker::Copy for SCF_EVENT_HEADER {}
impl ::core::clone::Clone for SCF_EVENT_HEADER {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_SideShow\"`*"]
pub struct SCF_MENUACTION_EVENT {
    pub PreviousPage: u32,
    pub TargetPage: u32,
    pub Button: u32,
    pub ItemId: u32,
}
impl ::core::marker::Copy for SCF_MENUACTION_EVENT {}
impl ::core::clone::Clone for SCF_MENUACTION_EVENT {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_SideShow\"`*"]
pub struct SCF_NAVIGATION_EVENT {
    pub PreviousPage: u32,
    pub TargetPage: u32,
    pub Button: u32,
}
impl ::core::marker::Copy for SCF_NAVIGATION_EVENT {}
impl ::core::clone::Clone for SCF_NAVIGATION_EVENT {
    fn clone(&self) -> Self {
        *self
    }
}

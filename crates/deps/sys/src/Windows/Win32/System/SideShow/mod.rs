#![allow(non_snake_case, non_camel_case_types)]
#[link(name = "windows")]
extern "system" {}
pub struct APPLICATION_EVENT_DATA(i32);
#[doc = "*Required features: `Win32_System_SideShow`*"]
pub const CONTENT_ID_GLANCE: u32 = 0u32;
#[doc = "*Required features: `Win32_System_SideShow`*"]
pub const CONTENT_ID_HOME: u32 = 1u32;
pub struct CONTENT_MISSING_EVENT_DATA(i32);
pub struct DEVICE_USER_CHANGE_EVENT_DATA(i32);
pub struct EVENT_DATA_HEADER(i32);
pub const GUID_DEVINTERFACE_SIDESHOW: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 355358737, data2: 65209, data3: 19200, data4: [144, 244, 211, 41, 71, 174, 22, 129] };
pub struct ISideShowBulkCapabilities(i32);
pub struct ISideShowCapabilities(i32);
pub struct ISideShowCapabilitiesCollection(i32);
pub struct ISideShowContent(i32);
pub struct ISideShowContentManager(i32);
pub struct ISideShowEvents(i32);
pub struct ISideShowKeyCollection(i32);
pub struct ISideShowNotification(i32);
pub struct ISideShowNotificationManager(i32);
pub struct ISideShowPropVariantCollection(i32);
pub struct ISideShowSession(i32);
pub struct NEW_EVENT_DATA_AVAILABLE(i32);
pub struct SCF_BUTTON_IDS(i32);
pub struct SCF_CONTEXTMENU_EVENT(i32);
pub struct SCF_EVENT_HEADER(i32);
pub struct SCF_EVENT_IDS(i32);
pub struct SCF_MENUACTION_EVENT(i32);
pub struct SCF_NAVIGATION_EVENT(i32);
pub const SIDESHOW_APPLICATION_EVENT: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 1286959866, data2: 7483, data3: 18867, data4: [161, 122, 46, 107, 255, 5, 40, 84] };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_System_SideShow`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const SIDESHOW_CAPABILITY_CLIENT_AREA_HEIGHT: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::GUID { data1: 2327611560, data2: 34171, data3: 19159, data4: [163, 90, 181, 148, 47, 73, 43, 153] },
    pid: 16u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_System_SideShow`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const SIDESHOW_CAPABILITY_CLIENT_AREA_WIDTH: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::GUID { data1: 2327611560, data2: 34171, data3: 19159, data4: [163, 90, 181, 148, 47, 73, 43, 153] },
    pid: 15u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_System_SideShow`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const SIDESHOW_CAPABILITY_COLOR_DEPTH: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::GUID { data1: 2327611560, data2: 34171, data3: 19159, data4: [163, 90, 181, 148, 47, 73, 43, 153] },
    pid: 5u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_System_SideShow`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const SIDESHOW_CAPABILITY_COLOR_TYPE: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::GUID { data1: 2327611560, data2: 34171, data3: 19159, data4: [163, 90, 181, 148, 47, 73, 43, 153] },
    pid: 6u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_System_SideShow`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const SIDESHOW_CAPABILITY_CURRENT_LANGUAGE: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::GUID { data1: 2327611560, data2: 34171, data3: 19159, data4: [163, 90, 181, 148, 47, 73, 43, 153] },
    pid: 9u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_System_SideShow`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const SIDESHOW_CAPABILITY_DATA_CACHE: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::GUID { data1: 2327611560, data2: 34171, data3: 19159, data4: [163, 90, 181, 148, 47, 73, 43, 153] },
    pid: 7u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_System_SideShow`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const SIDESHOW_CAPABILITY_DEVICE_ID: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::GUID { data1: 2327611560, data2: 34171, data3: 19159, data4: [163, 90, 181, 148, 47, 73, 43, 153] },
    pid: 1u32,
};
pub const SIDESHOW_CAPABILITY_DEVICE_PROPERTIES: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 2327611560, data2: 34171, data3: 19159, data4: [163, 90, 181, 148, 47, 73, 43, 153] };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_System_SideShow`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const SIDESHOW_CAPABILITY_SCREEN_HEIGHT: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::GUID { data1: 2327611560, data2: 34171, data3: 19159, data4: [163, 90, 181, 148, 47, 73, 43, 153] },
    pid: 4u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_System_SideShow`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const SIDESHOW_CAPABILITY_SCREEN_TYPE: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::GUID { data1: 2327611560, data2: 34171, data3: 19159, data4: [163, 90, 181, 148, 47, 73, 43, 153] },
    pid: 2u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_System_SideShow`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const SIDESHOW_CAPABILITY_SCREEN_WIDTH: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::GUID { data1: 2327611560, data2: 34171, data3: 19159, data4: [163, 90, 181, 148, 47, 73, 43, 153] },
    pid: 3u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_System_SideShow`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const SIDESHOW_CAPABILITY_SUPPORTED_IMAGE_FORMATS: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::GUID { data1: 2327611560, data2: 34171, data3: 19159, data4: [163, 90, 181, 148, 47, 73, 43, 153] },
    pid: 14u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_System_SideShow`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const SIDESHOW_CAPABILITY_SUPPORTED_LANGUAGES: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::GUID { data1: 2327611560, data2: 34171, data3: 19159, data4: [163, 90, 181, 148, 47, 73, 43, 153] },
    pid: 8u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_System_SideShow`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const SIDESHOW_CAPABILITY_SUPPORTED_THEMES: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::GUID { data1: 2327611560, data2: 34171, data3: 19159, data4: [163, 90, 181, 148, 47, 73, 43, 153] },
    pid: 10u32,
};
pub struct SIDESHOW_COLOR_TYPE(i32);
pub const SIDESHOW_CONTENT_MISSING_EVENT: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 1342700456, data2: 54035, data3: 17311, data4: [190, 162, 165, 2, 1, 211, 233, 168] };
pub const SIDESHOW_ENDPOINT_ICAL: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 1308571317, data2: 40414, data3: 20342, data4: [154, 42, 150, 67, 80, 71, 6, 61] };
pub const SIDESHOW_ENDPOINT_SIMPLE_CONTENT_FORMAT: ::windows_sys::core::GUID = ::windows_sys::GUID {
    data1: 2846176575,
    data2: 11595,
    data3: 18382,
    data4: [147, 238, 117, 159, 58, 125, 218, 79],
};
#[doc = "*Required features: `Win32_System_SideShow`*"]
pub const SIDESHOW_EVENTID_APPLICATION_ENTER: u32 = 4294901760u32;
#[doc = "*Required features: `Win32_System_SideShow`*"]
pub const SIDESHOW_EVENTID_APPLICATION_EXIT: u32 = 4294901761u32;
pub const SIDESHOW_NEW_EVENT_DATA_AVAILABLE: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 1468086356, data2: 12225, data3: 16668, data4: [165, 159, 242, 73, 39, 96, 136, 4] };
pub struct SIDESHOW_SCREEN_TYPE(i32);
pub const SIDESHOW_USER_CHANGE_REQUEST_EVENT: ::windows_sys::core::GUID = ::windows_sys::GUID {
    data1: 1342793532,
    data2: 16253,
    data3: 19582,
    data4: [153, 113, 234, 162, 233, 31, 21, 117],
};
pub struct SideShowKeyCollection(i32);
pub struct SideShowNotification(i32);
pub struct SideShowPropVariantCollection(i32);
pub struct SideShowSession(i32);
#[doc = "*Required features: `Win32_System_SideShow`*"]
pub const VERSION_1_WINDOWS_7: u32 = 0u32;

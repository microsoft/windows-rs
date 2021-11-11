#![allow(non_snake_case, non_camel_case_types)]
#[link(name = "windows")]
extern "system" {
    #[doc = "*Required features: `Win32_System_StationsAndDesktops`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn BroadcastSystemMessageA();
    #[doc = "*Required features: `Win32_System_StationsAndDesktops`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn BroadcastSystemMessageExA();
    #[doc = "*Required features: `Win32_System_StationsAndDesktops`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn BroadcastSystemMessageExW();
    #[doc = "*Required features: `Win32_System_StationsAndDesktops`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn BroadcastSystemMessageW();
    #[doc = "*Required features: `Win32_System_StationsAndDesktops`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CloseDesktop();
    #[doc = "*Required features: `Win32_System_StationsAndDesktops`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CloseWindowStation();
    #[doc = "*Required features: `Win32_System_StationsAndDesktops`, `Win32_Foundation`, `Win32_Graphics_Gdi`, `Win32_Security`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_Security"))]
    pub fn CreateDesktopA();
    #[doc = "*Required features: `Win32_System_StationsAndDesktops`, `Win32_Foundation`, `Win32_Graphics_Gdi`, `Win32_Security`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_Security"))]
    pub fn CreateDesktopExA();
    #[doc = "*Required features: `Win32_System_StationsAndDesktops`, `Win32_Foundation`, `Win32_Graphics_Gdi`, `Win32_Security`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_Security"))]
    pub fn CreateDesktopExW();
    #[doc = "*Required features: `Win32_System_StationsAndDesktops`, `Win32_Foundation`, `Win32_Graphics_Gdi`, `Win32_Security`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_Security"))]
    pub fn CreateDesktopW();
    #[doc = "*Required features: `Win32_System_StationsAndDesktops`, `Win32_Foundation`, `Win32_Security`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
    pub fn CreateWindowStationA();
    #[doc = "*Required features: `Win32_System_StationsAndDesktops`, `Win32_Foundation`, `Win32_Security`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
    pub fn CreateWindowStationW();
    #[doc = "*Required features: `Win32_System_StationsAndDesktops`, `Win32_Foundation`, `Win32_UI_WindowsAndMessaging`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_WindowsAndMessaging"))]
    pub fn EnumDesktopWindows();
    #[doc = "*Required features: `Win32_System_StationsAndDesktops`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn EnumDesktopsA();
    #[doc = "*Required features: `Win32_System_StationsAndDesktops`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn EnumDesktopsW();
    #[doc = "*Required features: `Win32_System_StationsAndDesktops`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn EnumWindowStationsA();
    #[doc = "*Required features: `Win32_System_StationsAndDesktops`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn EnumWindowStationsW();
    #[doc = "*Required features: `Win32_System_StationsAndDesktops`*"]
    pub fn GetProcessWindowStation();
    #[doc = "*Required features: `Win32_System_StationsAndDesktops`*"]
    pub fn GetThreadDesktop();
    #[doc = "*Required features: `Win32_System_StationsAndDesktops`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetUserObjectInformationA();
    #[doc = "*Required features: `Win32_System_StationsAndDesktops`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetUserObjectInformationW();
    #[doc = "*Required features: `Win32_System_StationsAndDesktops`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn OpenDesktopA();
    #[doc = "*Required features: `Win32_System_StationsAndDesktops`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn OpenDesktopW();
    #[doc = "*Required features: `Win32_System_StationsAndDesktops`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn OpenInputDesktop();
    #[doc = "*Required features: `Win32_System_StationsAndDesktops`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn OpenWindowStationA();
    #[doc = "*Required features: `Win32_System_StationsAndDesktops`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn OpenWindowStationW();
    #[doc = "*Required features: `Win32_System_StationsAndDesktops`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetProcessWindowStation();
    #[doc = "*Required features: `Win32_System_StationsAndDesktops`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetThreadDesktop();
    #[doc = "*Required features: `Win32_System_StationsAndDesktops`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetUserObjectInformationA();
    #[doc = "*Required features: `Win32_System_StationsAndDesktops`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetUserObjectInformationW();
    #[doc = "*Required features: `Win32_System_StationsAndDesktops`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SwitchDesktop();
}

#![allow(non_snake_case, non_camel_case_types)]
#[link(name = "windows")]
extern "system" {
    #[doc = "*Required features: `Win32_System_StationsAndDesktops`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn BroadcastSystemMessageA(flags: u32, lpinfo: *mut u32, msg: u32, wparam: super::super::Foundation::WPARAM, lparam: super::super::Foundation::LPARAM) -> i32;
    #[doc = "*Required features: `Win32_System_StationsAndDesktops`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn BroadcastSystemMessageExA(flags: BROADCAST_SYSTEM_MESSAGE_FLAGS, lpinfo: *mut BROADCAST_SYSTEM_MESSAGE_INFO, msg: u32, wparam: super::super::Foundation::WPARAM, lparam: super::super::Foundation::LPARAM, pbsminfo: *mut BSMINFO) -> i32;
    #[doc = "*Required features: `Win32_System_StationsAndDesktops`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn BroadcastSystemMessageExW(flags: BROADCAST_SYSTEM_MESSAGE_FLAGS, lpinfo: *mut BROADCAST_SYSTEM_MESSAGE_INFO, msg: u32, wparam: super::super::Foundation::WPARAM, lparam: super::super::Foundation::LPARAM, pbsminfo: *mut BSMINFO) -> i32;
    #[doc = "*Required features: `Win32_System_StationsAndDesktops`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn BroadcastSystemMessageW(flags: BROADCAST_SYSTEM_MESSAGE_FLAGS, lpinfo: *mut BROADCAST_SYSTEM_MESSAGE_INFO, msg: u32, wparam: super::super::Foundation::WPARAM, lparam: super::super::Foundation::LPARAM) -> i32;
    #[doc = "*Required features: `Win32_System_StationsAndDesktops`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CloseDesktop(hdesktop: HDESK) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_System_StationsAndDesktops`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CloseWindowStation(hwinsta: HWINSTA) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_System_StationsAndDesktops`, `Win32_Foundation`, `Win32_Graphics_Gdi`, `Win32_Security`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_Security"))]
    pub fn CreateDesktopA(lpszdesktop: super::super::Foundation::PSTR, lpszdevice: super::super::Foundation::PSTR, pdevmode: *mut super::super::Graphics::Gdi::DEVMODEA, dwflags: u32, dwdesiredaccess: u32, lpsa: *const super::super::Security::SECURITY_ATTRIBUTES) -> HDESK;
    #[doc = "*Required features: `Win32_System_StationsAndDesktops`, `Win32_Foundation`, `Win32_Graphics_Gdi`, `Win32_Security`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_Security"))]
    pub fn CreateDesktopExA(lpszdesktop: super::super::Foundation::PSTR, lpszdevice: super::super::Foundation::PSTR, pdevmode: *mut super::super::Graphics::Gdi::DEVMODEA, dwflags: u32, dwdesiredaccess: u32, lpsa: *const super::super::Security::SECURITY_ATTRIBUTES, ulheapsize: u32, pvoid: *mut ::core::ffi::c_void) -> HDESK;
    #[doc = "*Required features: `Win32_System_StationsAndDesktops`, `Win32_Foundation`, `Win32_Graphics_Gdi`, `Win32_Security`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_Security"))]
    pub fn CreateDesktopExW(lpszdesktop: super::super::Foundation::PWSTR, lpszdevice: super::super::Foundation::PWSTR, pdevmode: *mut super::super::Graphics::Gdi::DEVMODEW, dwflags: u32, dwdesiredaccess: u32, lpsa: *const super::super::Security::SECURITY_ATTRIBUTES, ulheapsize: u32, pvoid: *mut ::core::ffi::c_void) -> HDESK;
    #[doc = "*Required features: `Win32_System_StationsAndDesktops`, `Win32_Foundation`, `Win32_Graphics_Gdi`, `Win32_Security`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_Security"))]
    pub fn CreateDesktopW(lpszdesktop: super::super::Foundation::PWSTR, lpszdevice: super::super::Foundation::PWSTR, pdevmode: *mut super::super::Graphics::Gdi::DEVMODEW, dwflags: u32, dwdesiredaccess: u32, lpsa: *const super::super::Security::SECURITY_ATTRIBUTES) -> HDESK;
    #[doc = "*Required features: `Win32_System_StationsAndDesktops`, `Win32_Foundation`, `Win32_Security`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
    pub fn CreateWindowStationA(lpwinsta: super::super::Foundation::PSTR, dwflags: u32, dwdesiredaccess: u32, lpsa: *const super::super::Security::SECURITY_ATTRIBUTES) -> HWINSTA;
    #[doc = "*Required features: `Win32_System_StationsAndDesktops`, `Win32_Foundation`, `Win32_Security`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
    pub fn CreateWindowStationW(lpwinsta: super::super::Foundation::PWSTR, dwflags: u32, dwdesiredaccess: u32, lpsa: *const super::super::Security::SECURITY_ATTRIBUTES) -> HWINSTA;
    #[doc = "*Required features: `Win32_System_StationsAndDesktops`, `Win32_Foundation`, `Win32_UI_WindowsAndMessaging`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_WindowsAndMessaging"))]
    pub fn EnumDesktopWindows(hdesktop: HDESK, lpfn: super::super::UI::WindowsAndMessaging::WNDENUMPROC, lparam: super::super::Foundation::LPARAM) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_System_StationsAndDesktops`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn EnumDesktopsA(hwinsta: HWINSTA, lpenumfunc: DESKTOPENUMPROCA, lparam: super::super::Foundation::LPARAM) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_System_StationsAndDesktops`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn EnumDesktopsW(hwinsta: HWINSTA, lpenumfunc: DESKTOPENUMPROCW, lparam: super::super::Foundation::LPARAM) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_System_StationsAndDesktops`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn EnumWindowStationsA(lpenumfunc: WINSTAENUMPROCA, lparam: super::super::Foundation::LPARAM) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_System_StationsAndDesktops`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn EnumWindowStationsW(lpenumfunc: WINSTAENUMPROCW, lparam: super::super::Foundation::LPARAM) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_System_StationsAndDesktops`*"]
    pub fn GetProcessWindowStation() -> HWINSTA;
    #[doc = "*Required features: `Win32_System_StationsAndDesktops`*"]
    pub fn GetThreadDesktop(dwthreadid: u32) -> HDESK;
    #[doc = "*Required features: `Win32_System_StationsAndDesktops`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetUserObjectInformationA(hobj: super::super::Foundation::HANDLE, nindex: USER_OBJECT_INFORMATION_INDEX, pvinfo: *mut ::core::ffi::c_void, nlength: u32, lpnlengthneeded: *mut u32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_System_StationsAndDesktops`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetUserObjectInformationW(hobj: super::super::Foundation::HANDLE, nindex: USER_OBJECT_INFORMATION_INDEX, pvinfo: *mut ::core::ffi::c_void, nlength: u32, lpnlengthneeded: *mut u32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_System_StationsAndDesktops`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn OpenDesktopA(lpszdesktop: super::super::Foundation::PSTR, dwflags: u32, finherit: super::super::Foundation::BOOL, dwdesiredaccess: u32) -> HDESK;
    #[doc = "*Required features: `Win32_System_StationsAndDesktops`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn OpenDesktopW(lpszdesktop: super::super::Foundation::PWSTR, dwflags: u32, finherit: super::super::Foundation::BOOL, dwdesiredaccess: u32) -> HDESK;
    #[doc = "*Required features: `Win32_System_StationsAndDesktops`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn OpenInputDesktop(dwflags: u32, finherit: super::super::Foundation::BOOL, dwdesiredaccess: u32) -> HDESK;
    #[doc = "*Required features: `Win32_System_StationsAndDesktops`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn OpenWindowStationA(lpszwinsta: super::super::Foundation::PSTR, finherit: super::super::Foundation::BOOL, dwdesiredaccess: u32) -> HWINSTA;
    #[doc = "*Required features: `Win32_System_StationsAndDesktops`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn OpenWindowStationW(lpszwinsta: super::super::Foundation::PWSTR, finherit: super::super::Foundation::BOOL, dwdesiredaccess: u32) -> HWINSTA;
    #[doc = "*Required features: `Win32_System_StationsAndDesktops`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetProcessWindowStation(hwinsta: HWINSTA) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_System_StationsAndDesktops`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetThreadDesktop(hdesktop: HDESK) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_System_StationsAndDesktops`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetUserObjectInformationA(hobj: super::super::Foundation::HANDLE, nindex: i32, pvinfo: *const ::core::ffi::c_void, nlength: u32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_System_StationsAndDesktops`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetUserObjectInformationW(hobj: super::super::Foundation::HANDLE, nindex: i32, pvinfo: *const ::core::ffi::c_void, nlength: u32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_System_StationsAndDesktops`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SwitchDesktop(hdesktop: HDESK) -> super::super::Foundation::BOOL;
}
pub struct BROADCAST_SYSTEM_MESSAGE_FLAGS(i32);
pub struct BROADCAST_SYSTEM_MESSAGE_INFO(i32);
pub struct BSMINFO(i32);
pub struct DESKTOPENUMPROCA(i32);
pub struct DESKTOPENUMPROCW(i32);
pub struct HDESK(i32);
pub struct HWINSTA(i32);
pub struct USEROBJECTFLAGS(i32);
pub struct USER_OBJECT_INFORMATION_INDEX(i32);
pub struct WINSTAENUMPROCA(i32);
pub struct WINSTAENUMPROCW(i32);

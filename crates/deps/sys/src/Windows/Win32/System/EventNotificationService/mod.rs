#![allow(non_snake_case, non_camel_case_types)]
#[link(name = "windows")]
extern "system" {
    #[doc = "*Required features: `Win32_System_EventNotificationService`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn IsDestinationReachableA(lpszdestination: super::super::Foundation::PSTR, lpqocinfo: *mut QOCINFO) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_System_EventNotificationService`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn IsDestinationReachableW(lpszdestination: super::super::Foundation::PWSTR, lpqocinfo: *mut QOCINFO) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_System_EventNotificationService`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn IsNetworkAlive(lpdwflags: *mut u32) -> super::super::Foundation::BOOL;
}

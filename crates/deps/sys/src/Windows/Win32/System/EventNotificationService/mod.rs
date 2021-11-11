#![allow(non_snake_case, non_camel_case_types)]
#[link(name = "windows")]
extern "system" {
    #[doc = "*Required features: `Win32_System_EventNotificationService`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn IsDestinationReachableA();
    #[doc = "*Required features: `Win32_System_EventNotificationService`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn IsDestinationReachableW();
    #[doc = "*Required features: `Win32_System_EventNotificationService`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn IsNetworkAlive();
}

#![allow(non_snake_case, non_camel_case_types)]
#[link(name = "windows")]
extern "system" {
    #[doc = "*Required features: `Win32_System_SecurityCenter`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WscGetAntiMalwareUri();
    #[doc = "*Required features: `Win32_System_SecurityCenter`*"]
    pub fn WscGetSecurityProviderHealth();
    #[doc = "*Required features: `Win32_System_SecurityCenter`*"]
    pub fn WscQueryAntiMalwareUri();
    #[doc = "*Required features: `Win32_System_SecurityCenter`, `Win32_Foundation`, `Win32_System_Threading`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Threading"))]
    pub fn WscRegisterForChanges();
    #[doc = "*Required features: `Win32_System_SecurityCenter`*"]
    pub fn WscRegisterForUserNotifications();
    #[doc = "*Required features: `Win32_System_SecurityCenter`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WscUnRegisterChanges();
}

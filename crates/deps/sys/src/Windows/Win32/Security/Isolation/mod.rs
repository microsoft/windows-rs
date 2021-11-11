#![allow(non_snake_case, non_camel_case_types)]
#[link(name = "windows")]
extern "system" {
    #[doc = "*Required features: `Win32_Security_Isolation`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CreateAppContainerProfile();
    #[doc = "*Required features: `Win32_Security_Isolation`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DeleteAppContainerProfile();
    #[doc = "*Required features: `Win32_Security_Isolation`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DeriveAppContainerSidFromAppContainerName();
    #[doc = "*Required features: `Win32_Security_Isolation`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DeriveRestrictedAppContainerSidFromAppContainerSidAndRestrictedName();
    #[doc = "*Required features: `Win32_Security_Isolation`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetAppContainerFolderPath();
    #[doc = "*Required features: `Win32_Security_Isolation`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetAppContainerNamedObjectPath();
    #[doc = "*Required features: `Win32_Security_Isolation`, `Win32_System_Registry`*"]
    #[cfg(feature = "Win32_System_Registry")]
    pub fn GetAppContainerRegistryLocation();
    #[doc = "*Required features: `Win32_Security_Isolation`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn IsProcessInIsolatedContainer();
    #[doc = "*Required features: `Win32_Security_Isolation`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn IsProcessInIsolatedWindowsEnvironment();
    #[doc = "*Required features: `Win32_Security_Isolation`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn IsProcessInWDAGContainer();
}

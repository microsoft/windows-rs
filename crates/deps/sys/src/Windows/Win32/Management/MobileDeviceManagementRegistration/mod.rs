#![allow(non_snake_case, non_camel_case_types)]
#[link(name = "windows")]
extern "system" {
    #[doc = "*Required features: `Win32_Management_MobileDeviceManagementRegistration`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ApplyLocalManagementSyncML();
    #[doc = "*Required features: `Win32_Management_MobileDeviceManagementRegistration`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DiscoverManagementService();
    #[doc = "*Required features: `Win32_Management_MobileDeviceManagementRegistration`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DiscoverManagementServiceEx();
    #[doc = "*Required features: `Win32_Management_MobileDeviceManagementRegistration`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetDeviceManagementConfigInfo();
    #[doc = "*Required features: `Win32_Management_MobileDeviceManagementRegistration`*"]
    pub fn GetDeviceRegistrationInfo();
    #[doc = "*Required features: `Win32_Management_MobileDeviceManagementRegistration`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetManagementAppHyperlink();
    #[doc = "*Required features: `Win32_Management_MobileDeviceManagementRegistration`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn IsDeviceRegisteredWithManagement();
    #[doc = "*Required features: `Win32_Management_MobileDeviceManagementRegistration`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn IsManagementRegistrationAllowed();
    #[doc = "*Required features: `Win32_Management_MobileDeviceManagementRegistration`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn IsMdmUxWithoutAadAllowed();
    #[doc = "*Required features: `Win32_Management_MobileDeviceManagementRegistration`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn RegisterDeviceWithLocalManagement();
    #[doc = "*Required features: `Win32_Management_MobileDeviceManagementRegistration`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn RegisterDeviceWithManagement();
    #[doc = "*Required features: `Win32_Management_MobileDeviceManagementRegistration`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn RegisterDeviceWithManagementUsingAADCredentials();
    #[doc = "*Required features: `Win32_Management_MobileDeviceManagementRegistration`*"]
    pub fn RegisterDeviceWithManagementUsingAADDeviceCredentials();
    #[doc = "*Required features: `Win32_Management_MobileDeviceManagementRegistration`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn RegisterDeviceWithManagementUsingAADDeviceCredentials2();
    #[doc = "*Required features: `Win32_Management_MobileDeviceManagementRegistration`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetDeviceManagementConfigInfo();
    #[doc = "*Required features: `Win32_Management_MobileDeviceManagementRegistration`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetManagedExternally();
    #[doc = "*Required features: `Win32_Management_MobileDeviceManagementRegistration`*"]
    pub fn UnregisterDeviceWithLocalManagement();
    #[doc = "*Required features: `Win32_Management_MobileDeviceManagementRegistration`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn UnregisterDeviceWithManagement();
}

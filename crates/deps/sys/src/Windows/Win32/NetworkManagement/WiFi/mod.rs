#![allow(non_snake_case, non_camel_case_types)]
#[link(name = "windows")]
extern "system" {
    #[doc = "*Required features: `Win32_NetworkManagement_WiFi`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WFDCancelOpenSession();
    #[doc = "*Required features: `Win32_NetworkManagement_WiFi`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WFDCloseHandle();
    #[doc = "*Required features: `Win32_NetworkManagement_WiFi`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WFDCloseSession();
    #[doc = "*Required features: `Win32_NetworkManagement_WiFi`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WFDOpenHandle();
    #[doc = "*Required features: `Win32_NetworkManagement_WiFi`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WFDOpenLegacySession();
    #[doc = "*Required features: `Win32_NetworkManagement_WiFi`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WFDStartOpenSession();
    #[doc = "*Required features: `Win32_NetworkManagement_WiFi`*"]
    pub fn WFDUpdateDeviceVisibility();
    #[doc = "*Required features: `Win32_NetworkManagement_WiFi`*"]
    pub fn WlanAllocateMemory();
    #[doc = "*Required features: `Win32_NetworkManagement_WiFi`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WlanCloseHandle();
    #[doc = "*Required features: `Win32_NetworkManagement_WiFi`, `Win32_Foundation`, `Win32_NetworkManagement_Ndis`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_NetworkManagement_Ndis"))]
    pub fn WlanConnect();
    #[doc = "*Required features: `Win32_NetworkManagement_WiFi`, `Win32_Foundation`, `Win32_NetworkManagement_Ndis`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_NetworkManagement_Ndis"))]
    pub fn WlanConnect2();
    #[doc = "*Required features: `Win32_NetworkManagement_WiFi`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WlanDeleteProfile();
    #[doc = "*Required features: `Win32_NetworkManagement_WiFi`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WlanDeviceServiceCommand();
    #[doc = "*Required features: `Win32_NetworkManagement_WiFi`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WlanDisconnect();
    #[doc = "*Required features: `Win32_NetworkManagement_WiFi`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WlanEnumInterfaces();
    #[doc = "*Required features: `Win32_NetworkManagement_WiFi`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WlanExtractPsdIEDataList();
    #[doc = "*Required features: `Win32_NetworkManagement_WiFi`*"]
    pub fn WlanFreeMemory();
    #[doc = "*Required features: `Win32_NetworkManagement_WiFi`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WlanGetAvailableNetworkList();
    #[doc = "*Required features: `Win32_NetworkManagement_WiFi`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WlanGetAvailableNetworkList2();
    #[doc = "*Required features: `Win32_NetworkManagement_WiFi`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WlanGetFilterList();
    #[doc = "*Required features: `Win32_NetworkManagement_WiFi`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WlanGetInterfaceCapability();
    #[doc = "*Required features: `Win32_NetworkManagement_WiFi`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WlanGetNetworkBssList();
    #[doc = "*Required features: `Win32_NetworkManagement_WiFi`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WlanGetProfile();
    #[doc = "*Required features: `Win32_NetworkManagement_WiFi`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WlanGetProfileCustomUserData();
    #[doc = "*Required features: `Win32_NetworkManagement_WiFi`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WlanGetProfileList();
    #[doc = "*Required features: `Win32_NetworkManagement_WiFi`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WlanGetSecuritySettings();
    #[doc = "*Required features: `Win32_NetworkManagement_WiFi`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WlanGetSupportedDeviceServices();
    #[doc = "*Required features: `Win32_NetworkManagement_WiFi`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WlanHostedNetworkForceStart();
    #[doc = "*Required features: `Win32_NetworkManagement_WiFi`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WlanHostedNetworkForceStop();
    #[doc = "*Required features: `Win32_NetworkManagement_WiFi`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WlanHostedNetworkInitSettings();
    #[doc = "*Required features: `Win32_NetworkManagement_WiFi`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WlanHostedNetworkQueryProperty();
    #[doc = "*Required features: `Win32_NetworkManagement_WiFi`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WlanHostedNetworkQuerySecondaryKey();
    #[doc = "*Required features: `Win32_NetworkManagement_WiFi`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WlanHostedNetworkQueryStatus();
    #[doc = "*Required features: `Win32_NetworkManagement_WiFi`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WlanHostedNetworkRefreshSecuritySettings();
    #[doc = "*Required features: `Win32_NetworkManagement_WiFi`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WlanHostedNetworkSetProperty();
    #[doc = "*Required features: `Win32_NetworkManagement_WiFi`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WlanHostedNetworkSetSecondaryKey();
    #[doc = "*Required features: `Win32_NetworkManagement_WiFi`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WlanHostedNetworkStartUsing();
    #[doc = "*Required features: `Win32_NetworkManagement_WiFi`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WlanHostedNetworkStopUsing();
    #[doc = "*Required features: `Win32_NetworkManagement_WiFi`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WlanIhvControl();
    #[doc = "*Required features: `Win32_NetworkManagement_WiFi`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WlanOpenHandle();
    #[doc = "*Required features: `Win32_NetworkManagement_WiFi`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WlanQueryAutoConfigParameter();
    #[doc = "*Required features: `Win32_NetworkManagement_WiFi`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WlanQueryInterface();
    #[doc = "*Required features: `Win32_NetworkManagement_WiFi`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WlanReasonCodeToString();
    #[doc = "*Required features: `Win32_NetworkManagement_WiFi`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WlanRegisterDeviceServiceNotification();
    #[doc = "*Required features: `Win32_NetworkManagement_WiFi`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WlanRegisterNotification();
    #[doc = "*Required features: `Win32_NetworkManagement_WiFi`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WlanRegisterVirtualStationNotification();
    #[doc = "*Required features: `Win32_NetworkManagement_WiFi`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WlanRenameProfile();
    #[doc = "*Required features: `Win32_NetworkManagement_WiFi`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WlanSaveTemporaryProfile();
    #[doc = "*Required features: `Win32_NetworkManagement_WiFi`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WlanScan();
    #[doc = "*Required features: `Win32_NetworkManagement_WiFi`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WlanSetAutoConfigParameter();
    #[doc = "*Required features: `Win32_NetworkManagement_WiFi`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WlanSetFilterList();
    #[doc = "*Required features: `Win32_NetworkManagement_WiFi`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WlanSetInterface();
    #[doc = "*Required features: `Win32_NetworkManagement_WiFi`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WlanSetProfile();
    #[doc = "*Required features: `Win32_NetworkManagement_WiFi`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WlanSetProfileCustomUserData();
    #[doc = "*Required features: `Win32_NetworkManagement_WiFi`, `Win32_Foundation`, `Win32_Security_ExtensibleAuthenticationProtocol`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_ExtensibleAuthenticationProtocol"))]
    pub fn WlanSetProfileEapUserData();
    #[doc = "*Required features: `Win32_NetworkManagement_WiFi`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WlanSetProfileEapXmlUserData();
    #[doc = "*Required features: `Win32_NetworkManagement_WiFi`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WlanSetProfileList();
    #[doc = "*Required features: `Win32_NetworkManagement_WiFi`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WlanSetProfilePosition();
    #[doc = "*Required features: `Win32_NetworkManagement_WiFi`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WlanSetPsdIEDataList();
    #[doc = "*Required features: `Win32_NetworkManagement_WiFi`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WlanSetSecuritySettings();
    #[doc = "*Required features: `Win32_NetworkManagement_WiFi`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WlanUIEditProfile();
}

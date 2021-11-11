#![allow(non_snake_case, non_camel_case_types)]
#[link(name = "windows")]
extern "system" {
    #[doc = "*Required features: `Win32_NetworkManagement_WNet`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn MultinetGetConnectionPerformanceA();
    #[doc = "*Required features: `Win32_NetworkManagement_WNet`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn MultinetGetConnectionPerformanceW();
    #[doc = "*Required features: `Win32_NetworkManagement_WNet`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn NPAddConnection();
    #[doc = "*Required features: `Win32_NetworkManagement_WNet`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn NPAddConnection3();
    #[doc = "*Required features: `Win32_NetworkManagement_WNet`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn NPAddConnection4();
    #[doc = "*Required features: `Win32_NetworkManagement_WNet`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn NPCancelConnection();
    #[doc = "*Required features: `Win32_NetworkManagement_WNet`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn NPCancelConnection2();
    #[doc = "*Required features: `Win32_NetworkManagement_WNet`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn NPCloseEnum();
    #[doc = "*Required features: `Win32_NetworkManagement_WNet`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn NPEnumResource();
    #[doc = "*Required features: `Win32_NetworkManagement_WNet`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn NPFormatNetworkName();
    #[doc = "*Required features: `Win32_NetworkManagement_WNet`*"]
    pub fn NPGetCaps();
    #[doc = "*Required features: `Win32_NetworkManagement_WNet`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn NPGetConnection();
    #[doc = "*Required features: `Win32_NetworkManagement_WNet`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn NPGetConnection3();
    #[doc = "*Required features: `Win32_NetworkManagement_WNet`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn NPGetConnectionPerformance();
    #[doc = "*Required features: `Win32_NetworkManagement_WNet`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn NPGetPersistentUseOptionsForConnection();
    #[doc = "*Required features: `Win32_NetworkManagement_WNet`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn NPGetResourceInformation();
    #[doc = "*Required features: `Win32_NetworkManagement_WNet`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn NPGetResourceParent();
    #[doc = "*Required features: `Win32_NetworkManagement_WNet`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn NPGetUniversalName();
    #[doc = "*Required features: `Win32_NetworkManagement_WNet`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn NPGetUser();
    #[doc = "*Required features: `Win32_NetworkManagement_WNet`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn NPOpenEnum();
    #[doc = "*Required features: `Win32_NetworkManagement_WNet`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WNetAddConnection2A();
    #[doc = "*Required features: `Win32_NetworkManagement_WNet`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WNetAddConnection2W();
    #[doc = "*Required features: `Win32_NetworkManagement_WNet`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WNetAddConnection3A();
    #[doc = "*Required features: `Win32_NetworkManagement_WNet`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WNetAddConnection3W();
    #[doc = "*Required features: `Win32_NetworkManagement_WNet`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WNetAddConnection4A();
    #[doc = "*Required features: `Win32_NetworkManagement_WNet`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WNetAddConnection4W();
    #[doc = "*Required features: `Win32_NetworkManagement_WNet`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WNetAddConnectionA();
    #[doc = "*Required features: `Win32_NetworkManagement_WNet`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WNetAddConnectionW();
    #[doc = "*Required features: `Win32_NetworkManagement_WNet`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WNetCancelConnection2A();
    #[doc = "*Required features: `Win32_NetworkManagement_WNet`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WNetCancelConnection2W();
    #[doc = "*Required features: `Win32_NetworkManagement_WNet`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WNetCancelConnectionA();
    #[doc = "*Required features: `Win32_NetworkManagement_WNet`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WNetCancelConnectionW();
    #[doc = "*Required features: `Win32_NetworkManagement_WNet`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WNetCloseEnum();
    #[doc = "*Required features: `Win32_NetworkManagement_WNet`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WNetConnectionDialog();
    #[doc = "*Required features: `Win32_NetworkManagement_WNet`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WNetConnectionDialog1A();
    #[doc = "*Required features: `Win32_NetworkManagement_WNet`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WNetConnectionDialog1W();
    #[doc = "*Required features: `Win32_NetworkManagement_WNet`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WNetDisconnectDialog();
    #[doc = "*Required features: `Win32_NetworkManagement_WNet`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WNetDisconnectDialog1A();
    #[doc = "*Required features: `Win32_NetworkManagement_WNet`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WNetDisconnectDialog1W();
    #[doc = "*Required features: `Win32_NetworkManagement_WNet`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WNetEnumResourceA();
    #[doc = "*Required features: `Win32_NetworkManagement_WNet`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WNetEnumResourceW();
    #[doc = "*Required features: `Win32_NetworkManagement_WNet`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WNetGetConnectionA();
    #[doc = "*Required features: `Win32_NetworkManagement_WNet`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WNetGetConnectionW();
    #[doc = "*Required features: `Win32_NetworkManagement_WNet`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WNetGetLastErrorA();
    #[doc = "*Required features: `Win32_NetworkManagement_WNet`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WNetGetLastErrorW();
    #[doc = "*Required features: `Win32_NetworkManagement_WNet`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WNetGetNetworkInformationA();
    #[doc = "*Required features: `Win32_NetworkManagement_WNet`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WNetGetNetworkInformationW();
    #[doc = "*Required features: `Win32_NetworkManagement_WNet`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WNetGetProviderNameA();
    #[doc = "*Required features: `Win32_NetworkManagement_WNet`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WNetGetProviderNameW();
    #[doc = "*Required features: `Win32_NetworkManagement_WNet`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WNetGetResourceInformationA();
    #[doc = "*Required features: `Win32_NetworkManagement_WNet`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WNetGetResourceInformationW();
    #[doc = "*Required features: `Win32_NetworkManagement_WNet`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WNetGetResourceParentA();
    #[doc = "*Required features: `Win32_NetworkManagement_WNet`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WNetGetResourceParentW();
    #[doc = "*Required features: `Win32_NetworkManagement_WNet`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WNetGetUniversalNameA();
    #[doc = "*Required features: `Win32_NetworkManagement_WNet`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WNetGetUniversalNameW();
    #[doc = "*Required features: `Win32_NetworkManagement_WNet`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WNetGetUserA();
    #[doc = "*Required features: `Win32_NetworkManagement_WNet`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WNetGetUserW();
    #[doc = "*Required features: `Win32_NetworkManagement_WNet`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WNetOpenEnumA();
    #[doc = "*Required features: `Win32_NetworkManagement_WNet`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WNetOpenEnumW();
    #[doc = "*Required features: `Win32_NetworkManagement_WNet`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WNetSetLastErrorA();
    #[doc = "*Required features: `Win32_NetworkManagement_WNet`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WNetSetLastErrorW();
    #[doc = "*Required features: `Win32_NetworkManagement_WNet`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WNetUseConnection4A();
    #[doc = "*Required features: `Win32_NetworkManagement_WNet`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WNetUseConnection4W();
    #[doc = "*Required features: `Win32_NetworkManagement_WNet`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WNetUseConnectionA();
    #[doc = "*Required features: `Win32_NetworkManagement_WNet`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WNetUseConnectionW();
}

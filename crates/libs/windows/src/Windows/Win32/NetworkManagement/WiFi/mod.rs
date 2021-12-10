#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, clashing_extern_declarations, clippy::all)]
pub type CH_DESCRIPTION_TYPE = i32;
pub const ch_description_type_logical: CH_DESCRIPTION_TYPE = 1i32;
pub const ch_description_type_center_frequency: CH_DESCRIPTION_TYPE = 2i32;
pub const ch_description_type_phy_specific: CH_DESCRIPTION_TYPE = 3i32;
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const DEVPKEY_InfraCast_AccessPointBssid: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0x1506935d_e3e7_450f_8637_82233ebe5f6e), pid: 19u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const DEVPKEY_InfraCast_ChallengeAep: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0x1506935d_e3e7_450f_8637_82233ebe5f6e), pid: 21u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const DEVPKEY_InfraCast_DevnodeAep: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0x1506935d_e3e7_450f_8637_82233ebe5f6e), pid: 23u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const DEVPKEY_InfraCast_HostName_ResolutionMode: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0x1506935d_e3e7_450f_8637_82233ebe5f6e), pid: 25u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const DEVPKEY_InfraCast_PinSupported: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0x1506935d_e3e7_450f_8637_82233ebe5f6e), pid: 29u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const DEVPKEY_InfraCast_RtspTcpConnectionParametersSupported: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0x1506935d_e3e7_450f_8637_82233ebe5f6e), pid: 30u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const DEVPKEY_InfraCast_SinkHostName: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0x1506935d_e3e7_450f_8637_82233ebe5f6e), pid: 20u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const DEVPKEY_InfraCast_SinkIpAddress: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0x1506935d_e3e7_450f_8637_82233ebe5f6e), pid: 26u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const DEVPKEY_InfraCast_StreamSecuritySupported: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0x1506935d_e3e7_450f_8637_82233ebe5f6e), pid: 18u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const DEVPKEY_InfraCast_Supported: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0x1506935d_e3e7_450f_8637_82233ebe5f6e), pid: 17u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const DEVPKEY_WiFiDirectServices_AdvertisementId: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0x31b37743_7c5e_4005_93e6_e953f92b82e9), pid: 5u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const DEVPKEY_WiFiDirectServices_RequestServiceInformation: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0x31b37743_7c5e_4005_93e6_e953f92b82e9), pid: 7u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const DEVPKEY_WiFiDirectServices_ServiceAddress: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0x31b37743_7c5e_4005_93e6_e953f92b82e9), pid: 2u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const DEVPKEY_WiFiDirectServices_ServiceConfigMethods: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0x31b37743_7c5e_4005_93e6_e953f92b82e9), pid: 6u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const DEVPKEY_WiFiDirectServices_ServiceInformation: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0x31b37743_7c5e_4005_93e6_e953f92b82e9), pid: 4u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const DEVPKEY_WiFiDirectServices_ServiceName: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0x31b37743_7c5e_4005_93e6_e953f92b82e9), pid: 3u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const DEVPKEY_WiFiDirect_DeviceAddress: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0x1506935d_e3e7_450f_8637_82233ebe5f6e), pid: 1u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const DEVPKEY_WiFiDirect_DeviceAddressCopy: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0x1506935d_e3e7_450f_8637_82233ebe5f6e), pid: 13u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const DEVPKEY_WiFiDirect_FoundWsbService: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0x1506935d_e3e7_450f_8637_82233ebe5f6e), pid: 24u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const DEVPKEY_WiFiDirect_GroupId: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0x1506935d_e3e7_450f_8637_82233ebe5f6e), pid: 4u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const DEVPKEY_WiFiDirect_InformationElements: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0x1506935d_e3e7_450f_8637_82233ebe5f6e), pid: 12u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const DEVPKEY_WiFiDirect_InterfaceAddress: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0x1506935d_e3e7_450f_8637_82233ebe5f6e), pid: 2u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const DEVPKEY_WiFiDirect_InterfaceGuid: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0x1506935d_e3e7_450f_8637_82233ebe5f6e), pid: 3u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const DEVPKEY_WiFiDirect_IsConnected: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0x1506935d_e3e7_450f_8637_82233ebe5f6e), pid: 5u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const DEVPKEY_WiFiDirect_IsDMGCapable: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0x1506935d_e3e7_450f_8637_82233ebe5f6e), pid: 22u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const DEVPKEY_WiFiDirect_IsLegacyDevice: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0x1506935d_e3e7_450f_8637_82233ebe5f6e), pid: 7u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const DEVPKEY_WiFiDirect_IsMiracastLCPSupported: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0x1506935d_e3e7_450f_8637_82233ebe5f6e), pid: 9u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const DEVPKEY_WiFiDirect_IsRecentlyAssociated: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0x1506935d_e3e7_450f_8637_82233ebe5f6e), pid: 14u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const DEVPKEY_WiFiDirect_IsVisible: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0x1506935d_e3e7_450f_8637_82233ebe5f6e), pid: 6u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const DEVPKEY_WiFiDirect_LinkQuality: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0x1506935d_e3e7_450f_8637_82233ebe5f6e), pid: 28u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const DEVPKEY_WiFiDirect_MiracastVersion: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0x1506935d_e3e7_450f_8637_82233ebe5f6e), pid: 8u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const DEVPKEY_WiFiDirect_Miracast_SessionMgmtControlPort: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0x1506935d_e3e7_450f_8637_82233ebe5f6e), pid: 31u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const DEVPKEY_WiFiDirect_NoMiracastAutoProject: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0x1506935d_e3e7_450f_8637_82233ebe5f6e), pid: 16u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const DEVPKEY_WiFiDirect_RtspTcpConnectionParametersSupported: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0x1506935d_e3e7_450f_8637_82233ebe5f6e), pid: 32u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const DEVPKEY_WiFiDirect_Service_Aeps: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0x1506935d_e3e7_450f_8637_82233ebe5f6e), pid: 15u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const DEVPKEY_WiFiDirect_Services: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0x1506935d_e3e7_450f_8637_82233ebe5f6e), pid: 10u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const DEVPKEY_WiFiDirect_SupportedChannelList: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0x1506935d_e3e7_450f_8637_82233ebe5f6e), pid: 11u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const DEVPKEY_WiFiDirect_TransientAssociation: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0x1506935d_e3e7_450f_8637_82233ebe5f6e), pid: 27u32 };
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const DEVPKEY_WiFi_InterfaceGuid: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0xef1167eb_cbfc_4341_a568_a7c91a68982c), pid: 2u32 };
pub const DISCOVERY_FILTER_BITMASK_ANY: u32 = 15u32;
pub const DISCOVERY_FILTER_BITMASK_DEVICE: u32 = 1u32;
pub const DISCOVERY_FILTER_BITMASK_GO: u32 = 2u32;
#[repr(C)]
pub struct DOT11_ACCESSNETWORKOPTIONS {
    pub AccessNetworkType: u8,
    pub Internet: u8,
    pub ASRA: u8,
    pub ESR: u8,
    pub UESA: u8,
}
impl ::core::marker::Copy for DOT11_ACCESSNETWORKOPTIONS {}
impl ::core::clone::Clone for DOT11_ACCESSNETWORKOPTIONS {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for DOT11_ACCESSNETWORKOPTIONS {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for DOT11_ACCESSNETWORKOPTIONS {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<DOT11_ACCESSNETWORKOPTIONS>()) == 0 }
    }
}
impl ::core::cmp::Eq for DOT11_ACCESSNETWORKOPTIONS {}
impl ::core::default::Default for DOT11_ACCESSNETWORKOPTIONS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
pub type DOT11_AC_PARAM = i32;
pub const dot11_AC_param_BE: DOT11_AC_PARAM = 0i32;
pub const dot11_AC_param_BK: DOT11_AC_PARAM = 1i32;
pub const dot11_AC_param_VI: DOT11_AC_PARAM = 2i32;
pub const dot11_AC_param_VO: DOT11_AC_PARAM = 3i32;
pub const dot11_AC_param_max: DOT11_AC_PARAM = 4i32;
#[repr(C)]
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
pub struct DOT11_ADDITIONAL_IE {
    pub Header: super::Ndis::NDIS_OBJECT_HEADER,
    pub uBeaconIEsOffset: u32,
    pub uBeaconIEsLength: u32,
    pub uResponseIEsOffset: u32,
    pub uResponseIEsLength: u32,
}
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
impl ::core::marker::Copy for DOT11_ADDITIONAL_IE {}
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
impl ::core::clone::Clone for DOT11_ADDITIONAL_IE {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
unsafe impl ::windows::core::Abi for DOT11_ADDITIONAL_IE {
    type Abi = Self;
}
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
impl ::core::cmp::PartialEq for DOT11_ADDITIONAL_IE {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<DOT11_ADDITIONAL_IE>()) == 0 }
    }
}
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
impl ::core::cmp::Eq for DOT11_ADDITIONAL_IE {}
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
impl ::core::default::Default for DOT11_ADDITIONAL_IE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
pub const DOT11_ADDITIONAL_IE_REVISION_1: u32 = 1u32;
pub type DOT11_ADHOC_AUTH_ALGORITHM = i32;
pub const DOT11_ADHOC_AUTH_ALGO_INVALID: DOT11_ADHOC_AUTH_ALGORITHM = -1i32;
pub const DOT11_ADHOC_AUTH_ALGO_80211_OPEN: DOT11_ADHOC_AUTH_ALGORITHM = 1i32;
pub const DOT11_ADHOC_AUTH_ALGO_RSNA_PSK: DOT11_ADHOC_AUTH_ALGORITHM = 7i32;
pub type DOT11_ADHOC_CIPHER_ALGORITHM = i32;
pub const DOT11_ADHOC_CIPHER_ALGO_INVALID: DOT11_ADHOC_CIPHER_ALGORITHM = -1i32;
pub const DOT11_ADHOC_CIPHER_ALGO_NONE: DOT11_ADHOC_CIPHER_ALGORITHM = 0i32;
pub const DOT11_ADHOC_CIPHER_ALGO_CCMP: DOT11_ADHOC_CIPHER_ALGORITHM = 4i32;
pub const DOT11_ADHOC_CIPHER_ALGO_WEP: DOT11_ADHOC_CIPHER_ALGORITHM = 257i32;
pub type DOT11_ADHOC_CONNECT_FAIL_REASON = i32;
pub const DOT11_ADHOC_CONNECT_FAIL_DOMAIN_MISMATCH: DOT11_ADHOC_CONNECT_FAIL_REASON = 0i32;
pub const DOT11_ADHOC_CONNECT_FAIL_PASSPHRASE_MISMATCH: DOT11_ADHOC_CONNECT_FAIL_REASON = 1i32;
pub const DOT11_ADHOC_CONNECT_FAIL_OTHER: DOT11_ADHOC_CONNECT_FAIL_REASON = 2i32;
pub type DOT11_ADHOC_NETWORK_CONNECTION_STATUS = i32;
pub const DOT11_ADHOC_NETWORK_CONNECTION_STATUS_INVALID: DOT11_ADHOC_NETWORK_CONNECTION_STATUS = 0i32;
pub const DOT11_ADHOC_NETWORK_CONNECTION_STATUS_DISCONNECTED: DOT11_ADHOC_NETWORK_CONNECTION_STATUS = 11i32;
pub const DOT11_ADHOC_NETWORK_CONNECTION_STATUS_CONNECTING: DOT11_ADHOC_NETWORK_CONNECTION_STATUS = 12i32;
pub const DOT11_ADHOC_NETWORK_CONNECTION_STATUS_CONNECTED: DOT11_ADHOC_NETWORK_CONNECTION_STATUS = 13i32;
pub const DOT11_ADHOC_NETWORK_CONNECTION_STATUS_FORMED: DOT11_ADHOC_NETWORK_CONNECTION_STATUS = 14i32;
#[repr(C)]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_NetworkManagement_Ndis"))]
pub struct DOT11_ANQP_QUERY_COMPLETE_PARAMETERS {
    pub Header: super::Ndis::NDIS_OBJECT_HEADER,
    pub Status: DOT11_ANQP_QUERY_RESULT,
    pub hContext: super::super::Foundation::HANDLE,
    pub uResponseLength: u32,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_NetworkManagement_Ndis"))]
impl ::core::marker::Copy for DOT11_ANQP_QUERY_COMPLETE_PARAMETERS {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_NetworkManagement_Ndis"))]
impl ::core::clone::Clone for DOT11_ANQP_QUERY_COMPLETE_PARAMETERS {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_NetworkManagement_Ndis"))]
unsafe impl ::windows::core::Abi for DOT11_ANQP_QUERY_COMPLETE_PARAMETERS {
    type Abi = Self;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_NetworkManagement_Ndis"))]
impl ::core::cmp::PartialEq for DOT11_ANQP_QUERY_COMPLETE_PARAMETERS {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<DOT11_ANQP_QUERY_COMPLETE_PARAMETERS>()) == 0 }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_NetworkManagement_Ndis"))]
impl ::core::cmp::Eq for DOT11_ANQP_QUERY_COMPLETE_PARAMETERS {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_NetworkManagement_Ndis"))]
impl ::core::default::Default for DOT11_ANQP_QUERY_COMPLETE_PARAMETERS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
pub const DOT11_ANQP_QUERY_COMPLETE_PARAMETERS_REVISION_1: u32 = 1u32;
pub type DOT11_ANQP_QUERY_RESULT = i32;
pub const dot11_ANQP_query_result_success: DOT11_ANQP_QUERY_RESULT = 0i32;
pub const dot11_ANQP_query_result_failure: DOT11_ANQP_QUERY_RESULT = 1i32;
pub const dot11_ANQP_query_result_timed_out: DOT11_ANQP_QUERY_RESULT = 2i32;
pub const dot11_ANQP_query_result_resources: DOT11_ANQP_QUERY_RESULT = 3i32;
pub const dot11_ANQP_query_result_advertisement_protocol_not_supported_on_remote: DOT11_ANQP_QUERY_RESULT = 4i32;
pub const dot11_ANQP_query_result_gas_protocol_failure: DOT11_ANQP_QUERY_RESULT = 5i32;
pub const dot11_ANQP_query_result_advertisement_server_not_responding: DOT11_ANQP_QUERY_RESULT = 6i32;
pub const dot11_ANQP_query_result_access_issues: DOT11_ANQP_QUERY_RESULT = 7i32;
#[repr(C)]
pub struct DOT11_AP_JOIN_REQUEST {
    pub uJoinFailureTimeout: u32,
    pub OperationalRateSet: DOT11_RATE_SET,
    pub uChCenterFrequency: u32,
    pub dot11BSSDescription: DOT11_BSS_DESCRIPTION,
}
impl ::core::marker::Copy for DOT11_AP_JOIN_REQUEST {}
impl ::core::clone::Clone for DOT11_AP_JOIN_REQUEST {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for DOT11_AP_JOIN_REQUEST {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for DOT11_AP_JOIN_REQUEST {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<DOT11_AP_JOIN_REQUEST>()) == 0 }
    }
}
impl ::core::cmp::Eq for DOT11_AP_JOIN_REQUEST {}
impl ::core::default::Default for DOT11_AP_JOIN_REQUEST {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_NetworkManagement_Ndis"))]
pub struct DOT11_ASSOCIATION_COMPLETION_PARAMETERS {
    pub Header: super::Ndis::NDIS_OBJECT_HEADER,
    pub MacAddr: [u8; 6],
    pub uStatus: u32,
    pub bReAssocReq: super::super::Foundation::BOOLEAN,
    pub bReAssocResp: super::super::Foundation::BOOLEAN,
    pub uAssocReqOffset: u32,
    pub uAssocReqSize: u32,
    pub uAssocRespOffset: u32,
    pub uAssocRespSize: u32,
    pub uBeaconOffset: u32,
    pub uBeaconSize: u32,
    pub uIHVDataOffset: u32,
    pub uIHVDataSize: u32,
    pub AuthAlgo: DOT11_AUTH_ALGORITHM,
    pub UnicastCipher: DOT11_CIPHER_ALGORITHM,
    pub MulticastCipher: DOT11_CIPHER_ALGORITHM,
    pub uActivePhyListOffset: u32,
    pub uActivePhyListSize: u32,
    pub bFourAddressSupported: super::super::Foundation::BOOLEAN,
    pub bPortAuthorized: super::super::Foundation::BOOLEAN,
    pub ucActiveQoSProtocol: u8,
    pub DSInfo: DOT11_DS_INFO,
    pub uEncapTableOffset: u32,
    pub uEncapTableSize: u32,
    pub MulticastMgmtCipher: DOT11_CIPHER_ALGORITHM,
    pub uAssocComebackTime: u32,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_NetworkManagement_Ndis"))]
impl ::core::marker::Copy for DOT11_ASSOCIATION_COMPLETION_PARAMETERS {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_NetworkManagement_Ndis"))]
impl ::core::clone::Clone for DOT11_ASSOCIATION_COMPLETION_PARAMETERS {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_NetworkManagement_Ndis"))]
unsafe impl ::windows::core::Abi for DOT11_ASSOCIATION_COMPLETION_PARAMETERS {
    type Abi = Self;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_NetworkManagement_Ndis"))]
impl ::core::cmp::PartialEq for DOT11_ASSOCIATION_COMPLETION_PARAMETERS {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<DOT11_ASSOCIATION_COMPLETION_PARAMETERS>()) == 0 }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_NetworkManagement_Ndis"))]
impl ::core::cmp::Eq for DOT11_ASSOCIATION_COMPLETION_PARAMETERS {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_NetworkManagement_Ndis"))]
impl ::core::default::Default for DOT11_ASSOCIATION_COMPLETION_PARAMETERS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
pub const DOT11_ASSOCIATION_COMPLETION_PARAMETERS_REVISION_1: u32 = 1u32;
pub const DOT11_ASSOCIATION_COMPLETION_PARAMETERS_REVISION_2: u32 = 2u32;
#[repr(C)]
pub struct DOT11_ASSOCIATION_INFO_EX {
    pub PeerMacAddress: [u8; 6],
    pub BSSID: [u8; 6],
    pub usCapabilityInformation: u16,
    pub usListenInterval: u16,
    pub ucPeerSupportedRates: [u8; 255],
    pub usAssociationID: u16,
    pub dot11AssociationState: DOT11_ASSOCIATION_STATE,
    pub dot11PowerMode: DOT11_POWER_MODE,
    pub liAssociationUpTime: i64,
    pub ullNumOfTxPacketSuccesses: u64,
    pub ullNumOfTxPacketFailures: u64,
    pub ullNumOfRxPacketSuccesses: u64,
    pub ullNumOfRxPacketFailures: u64,
}
impl ::core::marker::Copy for DOT11_ASSOCIATION_INFO_EX {}
impl ::core::clone::Clone for DOT11_ASSOCIATION_INFO_EX {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for DOT11_ASSOCIATION_INFO_EX {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for DOT11_ASSOCIATION_INFO_EX {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<DOT11_ASSOCIATION_INFO_EX>()) == 0 }
    }
}
impl ::core::cmp::Eq for DOT11_ASSOCIATION_INFO_EX {}
impl ::core::default::Default for DOT11_ASSOCIATION_INFO_EX {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
pub struct DOT11_ASSOCIATION_INFO_LIST {
    pub Header: super::Ndis::NDIS_OBJECT_HEADER,
    pub uNumOfEntries: u32,
    pub uTotalNumOfEntries: u32,
    pub dot11AssocInfo: [DOT11_ASSOCIATION_INFO_EX; 1],
}
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
impl ::core::marker::Copy for DOT11_ASSOCIATION_INFO_LIST {}
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
impl ::core::clone::Clone for DOT11_ASSOCIATION_INFO_LIST {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
unsafe impl ::windows::core::Abi for DOT11_ASSOCIATION_INFO_LIST {
    type Abi = Self;
}
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
impl ::core::cmp::PartialEq for DOT11_ASSOCIATION_INFO_LIST {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<DOT11_ASSOCIATION_INFO_LIST>()) == 0 }
    }
}
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
impl ::core::cmp::Eq for DOT11_ASSOCIATION_INFO_LIST {}
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
impl ::core::default::Default for DOT11_ASSOCIATION_INFO_LIST {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
pub const DOT11_ASSOCIATION_INFO_LIST_REVISION_1: u32 = 1u32;
#[repr(C)]
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
pub struct DOT11_ASSOCIATION_PARAMS {
    pub Header: super::Ndis::NDIS_OBJECT_HEADER,
    pub BSSID: [u8; 6],
    pub uAssocRequestIEsOffset: u32,
    pub uAssocRequestIEsLength: u32,
}
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
impl ::core::marker::Copy for DOT11_ASSOCIATION_PARAMS {}
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
impl ::core::clone::Clone for DOT11_ASSOCIATION_PARAMS {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
unsafe impl ::windows::core::Abi for DOT11_ASSOCIATION_PARAMS {
    type Abi = Self;
}
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
impl ::core::cmp::PartialEq for DOT11_ASSOCIATION_PARAMS {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<DOT11_ASSOCIATION_PARAMS>()) == 0 }
    }
}
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
impl ::core::cmp::Eq for DOT11_ASSOCIATION_PARAMS {}
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
impl ::core::default::Default for DOT11_ASSOCIATION_PARAMS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
pub const DOT11_ASSOCIATION_PARAMS_REVISION_1: u32 = 1u32;
#[repr(C)]
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
pub struct DOT11_ASSOCIATION_START_PARAMETERS {
    pub Header: super::Ndis::NDIS_OBJECT_HEADER,
    pub MacAddr: [u8; 6],
    pub SSID: DOT11_SSID,
    pub uIHVDataOffset: u32,
    pub uIHVDataSize: u32,
}
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
impl ::core::marker::Copy for DOT11_ASSOCIATION_START_PARAMETERS {}
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
impl ::core::clone::Clone for DOT11_ASSOCIATION_START_PARAMETERS {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
unsafe impl ::windows::core::Abi for DOT11_ASSOCIATION_START_PARAMETERS {
    type Abi = Self;
}
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
impl ::core::cmp::PartialEq for DOT11_ASSOCIATION_START_PARAMETERS {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<DOT11_ASSOCIATION_START_PARAMETERS>()) == 0 }
    }
}
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
impl ::core::cmp::Eq for DOT11_ASSOCIATION_START_PARAMETERS {}
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
impl ::core::default::Default for DOT11_ASSOCIATION_START_PARAMETERS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
pub const DOT11_ASSOCIATION_START_PARAMETERS_REVISION_1: u32 = 1u32;
pub type DOT11_ASSOCIATION_STATE = i32;
pub const dot11_assoc_state_zero: DOT11_ASSOCIATION_STATE = 0i32;
pub const dot11_assoc_state_unauth_unassoc: DOT11_ASSOCIATION_STATE = 1i32;
pub const dot11_assoc_state_auth_unassoc: DOT11_ASSOCIATION_STATE = 2i32;
pub const dot11_assoc_state_auth_assoc: DOT11_ASSOCIATION_STATE = 3i32;
pub const DOT11_ASSOC_ERROR_SOURCE_OS: u32 = 0u32;
pub const DOT11_ASSOC_ERROR_SOURCE_OTHER: u32 = 255u32;
pub const DOT11_ASSOC_ERROR_SOURCE_REMOTE: u32 = 1u32;
pub const DOT11_ASSOC_STATUS_SUCCESS: u32 = 0u32;
pub type DOT11_AUTH_ALGORITHM = i32;
pub const DOT11_AUTH_ALGO_80211_OPEN: DOT11_AUTH_ALGORITHM = 1i32;
pub const DOT11_AUTH_ALGO_80211_SHARED_KEY: DOT11_AUTH_ALGORITHM = 2i32;
pub const DOT11_AUTH_ALGO_WPA: DOT11_AUTH_ALGORITHM = 3i32;
pub const DOT11_AUTH_ALGO_WPA_PSK: DOT11_AUTH_ALGORITHM = 4i32;
pub const DOT11_AUTH_ALGO_WPA_NONE: DOT11_AUTH_ALGORITHM = 5i32;
pub const DOT11_AUTH_ALGO_RSNA: DOT11_AUTH_ALGORITHM = 6i32;
pub const DOT11_AUTH_ALGO_RSNA_PSK: DOT11_AUTH_ALGORITHM = 7i32;
pub const DOT11_AUTH_ALGO_WPA3: DOT11_AUTH_ALGORITHM = 8i32;
pub const DOT11_AUTH_ALGO_WPA3_ENT_192: DOT11_AUTH_ALGORITHM = 8i32;
pub const DOT11_AUTH_ALGO_WPA3_SAE: DOT11_AUTH_ALGORITHM = 9i32;
pub const DOT11_AUTH_ALGO_OWE: DOT11_AUTH_ALGORITHM = 10i32;
pub const DOT11_AUTH_ALGO_WPA3_ENT: DOT11_AUTH_ALGORITHM = 11i32;
pub const DOT11_AUTH_ALGO_IHV_START: DOT11_AUTH_ALGORITHM = -2147483648i32;
pub const DOT11_AUTH_ALGO_IHV_END: DOT11_AUTH_ALGORITHM = -1i32;
#[repr(C)]
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
pub struct DOT11_AUTH_ALGORITHM_LIST {
    pub Header: super::Ndis::NDIS_OBJECT_HEADER,
    pub uNumOfEntries: u32,
    pub uTotalNumOfEntries: u32,
    pub AlgorithmIds: [DOT11_AUTH_ALGORITHM; 1],
}
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
impl ::core::marker::Copy for DOT11_AUTH_ALGORITHM_LIST {}
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
impl ::core::clone::Clone for DOT11_AUTH_ALGORITHM_LIST {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
unsafe impl ::windows::core::Abi for DOT11_AUTH_ALGORITHM_LIST {
    type Abi = Self;
}
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
impl ::core::cmp::PartialEq for DOT11_AUTH_ALGORITHM_LIST {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<DOT11_AUTH_ALGORITHM_LIST>()) == 0 }
    }
}
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
impl ::core::cmp::Eq for DOT11_AUTH_ALGORITHM_LIST {}
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
impl ::core::default::Default for DOT11_AUTH_ALGORITHM_LIST {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
pub const DOT11_AUTH_ALGORITHM_LIST_REVISION_1: u32 = 1u32;
pub const DOT11_AUTH_ALGO_MICHAEL: u32 = 1u32;
#[repr(C)]
pub struct DOT11_AUTH_CIPHER_PAIR {
    pub AuthAlgoId: DOT11_AUTH_ALGORITHM,
    pub CipherAlgoId: DOT11_CIPHER_ALGORITHM,
}
impl ::core::marker::Copy for DOT11_AUTH_CIPHER_PAIR {}
impl ::core::clone::Clone for DOT11_AUTH_CIPHER_PAIR {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for DOT11_AUTH_CIPHER_PAIR {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for DOT11_AUTH_CIPHER_PAIR {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<DOT11_AUTH_CIPHER_PAIR>()) == 0 }
    }
}
impl ::core::cmp::Eq for DOT11_AUTH_CIPHER_PAIR {}
impl ::core::default::Default for DOT11_AUTH_CIPHER_PAIR {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
pub struct DOT11_AUTH_CIPHER_PAIR_LIST {
    pub Header: super::Ndis::NDIS_OBJECT_HEADER,
    pub uNumOfEntries: u32,
    pub uTotalNumOfEntries: u32,
    pub AuthCipherPairs: [DOT11_AUTH_CIPHER_PAIR; 1],
}
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
impl ::core::marker::Copy for DOT11_AUTH_CIPHER_PAIR_LIST {}
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
impl ::core::clone::Clone for DOT11_AUTH_CIPHER_PAIR_LIST {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
unsafe impl ::windows::core::Abi for DOT11_AUTH_CIPHER_PAIR_LIST {
    type Abi = Self;
}
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
impl ::core::cmp::PartialEq for DOT11_AUTH_CIPHER_PAIR_LIST {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<DOT11_AUTH_CIPHER_PAIR_LIST>()) == 0 }
    }
}
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
impl ::core::cmp::Eq for DOT11_AUTH_CIPHER_PAIR_LIST {}
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
impl ::core::default::Default for DOT11_AUTH_CIPHER_PAIR_LIST {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
pub const DOT11_AUTH_CIPHER_PAIR_LIST_REVISION_1: u32 = 1u32;
#[repr(C)]
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
pub struct DOT11_AVAILABLE_CHANNEL_LIST {
    pub Header: super::Ndis::NDIS_OBJECT_HEADER,
    pub uNumOfEntries: u32,
    pub uTotalNumOfEntries: u32,
    pub uChannelNumber: [u32; 1],
}
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
impl ::core::marker::Copy for DOT11_AVAILABLE_CHANNEL_LIST {}
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
impl ::core::clone::Clone for DOT11_AVAILABLE_CHANNEL_LIST {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
unsafe impl ::windows::core::Abi for DOT11_AVAILABLE_CHANNEL_LIST {
    type Abi = Self;
}
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
impl ::core::cmp::PartialEq for DOT11_AVAILABLE_CHANNEL_LIST {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<DOT11_AVAILABLE_CHANNEL_LIST>()) == 0 }
    }
}
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
impl ::core::cmp::Eq for DOT11_AVAILABLE_CHANNEL_LIST {}
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
impl ::core::default::Default for DOT11_AVAILABLE_CHANNEL_LIST {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
pub const DOT11_AVAILABLE_CHANNEL_LIST_REVISION_1: u32 = 1u32;
#[repr(C)]
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
pub struct DOT11_AVAILABLE_FREQUENCY_LIST {
    pub Header: super::Ndis::NDIS_OBJECT_HEADER,
    pub uNumOfEntries: u32,
    pub uTotalNumOfEntries: u32,
    pub uFrequencyValue: [u32; 1],
}
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
impl ::core::marker::Copy for DOT11_AVAILABLE_FREQUENCY_LIST {}
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
impl ::core::clone::Clone for DOT11_AVAILABLE_FREQUENCY_LIST {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
unsafe impl ::windows::core::Abi for DOT11_AVAILABLE_FREQUENCY_LIST {
    type Abi = Self;
}
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
impl ::core::cmp::PartialEq for DOT11_AVAILABLE_FREQUENCY_LIST {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<DOT11_AVAILABLE_FREQUENCY_LIST>()) == 0 }
    }
}
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
impl ::core::cmp::Eq for DOT11_AVAILABLE_FREQUENCY_LIST {}
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
impl ::core::default::Default for DOT11_AVAILABLE_FREQUENCY_LIST {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
pub const DOT11_AVAILABLE_FREQUENCY_LIST_REVISION_1: u32 = 1u32;
pub type DOT11_BAND = i32;
pub const dot11_band_2p4g: DOT11_BAND = 1i32;
pub const dot11_band_4p9g: DOT11_BAND = 2i32;
pub const dot11_band_5g: DOT11_BAND = 3i32;
#[repr(C)]
pub struct DOT11_BSSID_CANDIDATE {
    pub BSSID: [u8; 6],
    pub uFlags: u32,
}
impl ::core::marker::Copy for DOT11_BSSID_CANDIDATE {}
impl ::core::clone::Clone for DOT11_BSSID_CANDIDATE {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for DOT11_BSSID_CANDIDATE {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for DOT11_BSSID_CANDIDATE {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<DOT11_BSSID_CANDIDATE>()) == 0 }
    }
}
impl ::core::cmp::Eq for DOT11_BSSID_CANDIDATE {}
impl ::core::default::Default for DOT11_BSSID_CANDIDATE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
pub struct DOT11_BSSID_LIST {
    pub Header: super::Ndis::NDIS_OBJECT_HEADER,
    pub uNumOfEntries: u32,
    pub uTotalNumOfEntries: u32,
    pub BSSIDs: [u8; 6],
}
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
impl ::core::marker::Copy for DOT11_BSSID_LIST {}
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
impl ::core::clone::Clone for DOT11_BSSID_LIST {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
unsafe impl ::windows::core::Abi for DOT11_BSSID_LIST {
    type Abi = Self;
}
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
impl ::core::cmp::PartialEq for DOT11_BSSID_LIST {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<DOT11_BSSID_LIST>()) == 0 }
    }
}
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
impl ::core::cmp::Eq for DOT11_BSSID_LIST {}
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
impl ::core::default::Default for DOT11_BSSID_LIST {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
pub const DOT11_BSSID_LIST_REVISION_1: u32 = 1u32;
#[repr(C)]
pub struct DOT11_BSS_DESCRIPTION {
    pub uReserved: u32,
    pub dot11BSSID: [u8; 6],
    pub dot11BSSType: DOT11_BSS_TYPE,
    pub usBeaconPeriod: u16,
    pub ullTimestamp: u64,
    pub usCapabilityInformation: u16,
    pub uBufferLength: u32,
    pub ucBuffer: [u8; 1],
}
impl ::core::marker::Copy for DOT11_BSS_DESCRIPTION {}
impl ::core::clone::Clone for DOT11_BSS_DESCRIPTION {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for DOT11_BSS_DESCRIPTION {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for DOT11_BSS_DESCRIPTION {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<DOT11_BSS_DESCRIPTION>()) == 0 }
    }
}
impl ::core::cmp::Eq for DOT11_BSS_DESCRIPTION {}
impl ::core::default::Default for DOT11_BSS_DESCRIPTION {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct DOT11_BSS_ENTRY {
    pub uPhyId: u32,
    pub PhySpecificInfo: DOT11_BSS_ENTRY_PHY_SPECIFIC_INFO,
    pub dot11BSSID: [u8; 6],
    pub dot11BSSType: DOT11_BSS_TYPE,
    pub lRSSI: i32,
    pub uLinkQuality: u32,
    pub bInRegDomain: super::super::Foundation::BOOLEAN,
    pub usBeaconPeriod: u16,
    pub ullTimestamp: u64,
    pub ullHostTimestamp: u64,
    pub usCapabilityInformation: u16,
    pub uBufferLength: u32,
    pub ucBuffer: [u8; 1],
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for DOT11_BSS_ENTRY {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for DOT11_BSS_ENTRY {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for DOT11_BSS_ENTRY {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for DOT11_BSS_ENTRY {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<DOT11_BSS_ENTRY>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for DOT11_BSS_ENTRY {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for DOT11_BSS_ENTRY {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
pub const DOT11_BSS_ENTRY_BYTE_ARRAY_REVISION_1: u32 = 1u32;
#[repr(C)]
pub union DOT11_BSS_ENTRY_PHY_SPECIFIC_INFO {
    pub uChCenterFrequency: u32,
    pub FHSS: DOT11_BSS_ENTRY_PHY_SPECIFIC_INFO_0,
}
impl ::core::marker::Copy for DOT11_BSS_ENTRY_PHY_SPECIFIC_INFO {}
impl ::core::clone::Clone for DOT11_BSS_ENTRY_PHY_SPECIFIC_INFO {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for DOT11_BSS_ENTRY_PHY_SPECIFIC_INFO {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for DOT11_BSS_ENTRY_PHY_SPECIFIC_INFO {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<DOT11_BSS_ENTRY_PHY_SPECIFIC_INFO>()) == 0 }
    }
}
impl ::core::cmp::Eq for DOT11_BSS_ENTRY_PHY_SPECIFIC_INFO {}
impl ::core::default::Default for DOT11_BSS_ENTRY_PHY_SPECIFIC_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct DOT11_BSS_ENTRY_PHY_SPECIFIC_INFO_0 {
    pub uHopPattern: u32,
    pub uHopSet: u32,
    pub uDwellTime: u32,
}
impl ::core::marker::Copy for DOT11_BSS_ENTRY_PHY_SPECIFIC_INFO_0 {}
impl ::core::clone::Clone for DOT11_BSS_ENTRY_PHY_SPECIFIC_INFO_0 {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for DOT11_BSS_ENTRY_PHY_SPECIFIC_INFO_0 {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for DOT11_BSS_ENTRY_PHY_SPECIFIC_INFO_0 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<DOT11_BSS_ENTRY_PHY_SPECIFIC_INFO_0>()) == 0 }
    }
}
impl ::core::cmp::Eq for DOT11_BSS_ENTRY_PHY_SPECIFIC_INFO_0 {}
impl ::core::default::Default for DOT11_BSS_ENTRY_PHY_SPECIFIC_INFO_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
pub type DOT11_BSS_TYPE = i32;
pub const dot11_BSS_type_infrastructure: DOT11_BSS_TYPE = 1i32;
pub const dot11_BSS_type_independent: DOT11_BSS_TYPE = 2i32;
pub const dot11_BSS_type_any: DOT11_BSS_TYPE = 3i32;
#[repr(C)]
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
pub struct DOT11_BYTE_ARRAY {
    pub Header: super::Ndis::NDIS_OBJECT_HEADER,
    pub uNumOfBytes: u32,
    pub uTotalNumOfBytes: u32,
    pub ucBuffer: [u8; 1],
}
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
impl ::core::marker::Copy for DOT11_BYTE_ARRAY {}
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
impl ::core::clone::Clone for DOT11_BYTE_ARRAY {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
unsafe impl ::windows::core::Abi for DOT11_BYTE_ARRAY {
    type Abi = Self;
}
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
impl ::core::cmp::PartialEq for DOT11_BYTE_ARRAY {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<DOT11_BYTE_ARRAY>()) == 0 }
    }
}
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
impl ::core::cmp::Eq for DOT11_BYTE_ARRAY {}
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
impl ::core::default::Default for DOT11_BYTE_ARRAY {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
pub struct DOT11_CAN_SUSTAIN_AP_PARAMETERS {
    pub Header: super::Ndis::NDIS_OBJECT_HEADER,
    pub ulReason: u32,
}
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
impl ::core::marker::Copy for DOT11_CAN_SUSTAIN_AP_PARAMETERS {}
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
impl ::core::clone::Clone for DOT11_CAN_SUSTAIN_AP_PARAMETERS {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
unsafe impl ::windows::core::Abi for DOT11_CAN_SUSTAIN_AP_PARAMETERS {
    type Abi = Self;
}
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
impl ::core::cmp::PartialEq for DOT11_CAN_SUSTAIN_AP_PARAMETERS {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<DOT11_CAN_SUSTAIN_AP_PARAMETERS>()) == 0 }
    }
}
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
impl ::core::cmp::Eq for DOT11_CAN_SUSTAIN_AP_PARAMETERS {}
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
impl ::core::default::Default for DOT11_CAN_SUSTAIN_AP_PARAMETERS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
pub const DOT11_CAN_SUSTAIN_AP_PARAMETERS_REVISION_1: u32 = 1u32;
pub const DOT11_CAN_SUSTAIN_AP_REASON_IHV_END: u32 = 4294967295u32;
pub const DOT11_CAN_SUSTAIN_AP_REASON_IHV_START: u32 = 4278190080u32;
pub const DOT11_CAPABILITY_CHANNEL_AGILITY: u32 = 128u32;
pub const DOT11_CAPABILITY_DSSSOFDM: u32 = 8192u32;
pub const DOT11_CAPABILITY_INFO_CF_POLLABLE: u32 = 4u32;
pub const DOT11_CAPABILITY_INFO_CF_POLL_REQ: u32 = 8u32;
pub const DOT11_CAPABILITY_INFO_ESS: u32 = 1u32;
pub const DOT11_CAPABILITY_INFO_IBSS: u32 = 2u32;
pub const DOT11_CAPABILITY_INFO_PRIVACY: u32 = 16u32;
pub const DOT11_CAPABILITY_PBCC: u32 = 64u32;
pub const DOT11_CAPABILITY_SHORT_PREAMBLE: u32 = 32u32;
pub const DOT11_CAPABILITY_SHORT_SLOT_TIME: u32 = 1024u32;
pub const DOT11_CCA_MODE_CS_ONLY: u32 = 2u32;
pub const DOT11_CCA_MODE_CS_WITH_TIMER: u32 = 8u32;
pub const DOT11_CCA_MODE_ED_ONLY: u32 = 1u32;
pub const DOT11_CCA_MODE_ED_and_CS: u32 = 4u32;
pub const DOT11_CCA_MODE_HRCS_AND_ED: u32 = 16u32;
#[repr(C)]
pub struct DOT11_CHANNEL_HINT {
    pub Dot11PhyType: DOT11_PHY_TYPE,
    pub uChannelNumber: u32,
}
impl ::core::marker::Copy for DOT11_CHANNEL_HINT {}
impl ::core::clone::Clone for DOT11_CHANNEL_HINT {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for DOT11_CHANNEL_HINT {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for DOT11_CHANNEL_HINT {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<DOT11_CHANNEL_HINT>()) == 0 }
    }
}
impl ::core::cmp::Eq for DOT11_CHANNEL_HINT {}
impl ::core::default::Default for DOT11_CHANNEL_HINT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
pub type DOT11_CIPHER_ALGORITHM = i32;
pub const DOT11_CIPHER_ALGO_NONE: DOT11_CIPHER_ALGORITHM = 0i32;
pub const DOT11_CIPHER_ALGO_WEP40: DOT11_CIPHER_ALGORITHM = 1i32;
pub const DOT11_CIPHER_ALGO_TKIP: DOT11_CIPHER_ALGORITHM = 2i32;
pub const DOT11_CIPHER_ALGO_CCMP: DOT11_CIPHER_ALGORITHM = 4i32;
pub const DOT11_CIPHER_ALGO_WEP104: DOT11_CIPHER_ALGORITHM = 5i32;
pub const DOT11_CIPHER_ALGO_BIP: DOT11_CIPHER_ALGORITHM = 6i32;
pub const DOT11_CIPHER_ALGO_GCMP: DOT11_CIPHER_ALGORITHM = 8i32;
pub const DOT11_CIPHER_ALGO_GCMP_256: DOT11_CIPHER_ALGORITHM = 9i32;
pub const DOT11_CIPHER_ALGO_CCMP_256: DOT11_CIPHER_ALGORITHM = 10i32;
pub const DOT11_CIPHER_ALGO_BIP_GMAC_128: DOT11_CIPHER_ALGORITHM = 11i32;
pub const DOT11_CIPHER_ALGO_BIP_GMAC_256: DOT11_CIPHER_ALGORITHM = 12i32;
pub const DOT11_CIPHER_ALGO_BIP_CMAC_256: DOT11_CIPHER_ALGORITHM = 13i32;
pub const DOT11_CIPHER_ALGO_WPA_USE_GROUP: DOT11_CIPHER_ALGORITHM = 256i32;
pub const DOT11_CIPHER_ALGO_RSN_USE_GROUP: DOT11_CIPHER_ALGORITHM = 256i32;
pub const DOT11_CIPHER_ALGO_WEP: DOT11_CIPHER_ALGORITHM = 257i32;
pub const DOT11_CIPHER_ALGO_IHV_START: DOT11_CIPHER_ALGORITHM = -2147483648i32;
pub const DOT11_CIPHER_ALGO_IHV_END: DOT11_CIPHER_ALGORITHM = -1i32;
#[repr(C)]
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
pub struct DOT11_CIPHER_ALGORITHM_LIST {
    pub Header: super::Ndis::NDIS_OBJECT_HEADER,
    pub uNumOfEntries: u32,
    pub uTotalNumOfEntries: u32,
    pub AlgorithmIds: [DOT11_CIPHER_ALGORITHM; 1],
}
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
impl ::core::marker::Copy for DOT11_CIPHER_ALGORITHM_LIST {}
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
impl ::core::clone::Clone for DOT11_CIPHER_ALGORITHM_LIST {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
unsafe impl ::windows::core::Abi for DOT11_CIPHER_ALGORITHM_LIST {
    type Abi = Self;
}
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
impl ::core::cmp::PartialEq for DOT11_CIPHER_ALGORITHM_LIST {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<DOT11_CIPHER_ALGORITHM_LIST>()) == 0 }
    }
}
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
impl ::core::cmp::Eq for DOT11_CIPHER_ALGORITHM_LIST {}
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
impl ::core::default::Default for DOT11_CIPHER_ALGORITHM_LIST {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
pub const DOT11_CIPHER_ALGORITHM_LIST_REVISION_1: u32 = 1u32;
#[repr(C)]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_NetworkManagement_Ndis"))]
pub struct DOT11_CIPHER_DEFAULT_KEY_VALUE {
    pub Header: super::Ndis::NDIS_OBJECT_HEADER,
    pub uKeyIndex: u32,
    pub AlgorithmId: DOT11_CIPHER_ALGORITHM,
    pub MacAddr: [u8; 6],
    pub bDelete: super::super::Foundation::BOOLEAN,
    pub bStatic: super::super::Foundation::BOOLEAN,
    pub usKeyLength: u16,
    pub ucKey: [u8; 1],
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_NetworkManagement_Ndis"))]
impl ::core::marker::Copy for DOT11_CIPHER_DEFAULT_KEY_VALUE {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_NetworkManagement_Ndis"))]
impl ::core::clone::Clone for DOT11_CIPHER_DEFAULT_KEY_VALUE {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_NetworkManagement_Ndis"))]
unsafe impl ::windows::core::Abi for DOT11_CIPHER_DEFAULT_KEY_VALUE {
    type Abi = Self;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_NetworkManagement_Ndis"))]
impl ::core::cmp::PartialEq for DOT11_CIPHER_DEFAULT_KEY_VALUE {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<DOT11_CIPHER_DEFAULT_KEY_VALUE>()) == 0 }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_NetworkManagement_Ndis"))]
impl ::core::cmp::Eq for DOT11_CIPHER_DEFAULT_KEY_VALUE {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_NetworkManagement_Ndis"))]
impl ::core::default::Default for DOT11_CIPHER_DEFAULT_KEY_VALUE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
pub const DOT11_CIPHER_DEFAULT_KEY_VALUE_REVISION_1: u32 = 1u32;
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct DOT11_CIPHER_KEY_MAPPING_KEY_VALUE {
    pub PeerMacAddr: [u8; 6],
    pub AlgorithmId: DOT11_CIPHER_ALGORITHM,
    pub Direction: DOT11_DIRECTION,
    pub bDelete: super::super::Foundation::BOOLEAN,
    pub bStatic: super::super::Foundation::BOOLEAN,
    pub usKeyLength: u16,
    pub ucKey: [u8; 1],
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for DOT11_CIPHER_KEY_MAPPING_KEY_VALUE {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for DOT11_CIPHER_KEY_MAPPING_KEY_VALUE {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for DOT11_CIPHER_KEY_MAPPING_KEY_VALUE {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for DOT11_CIPHER_KEY_MAPPING_KEY_VALUE {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<DOT11_CIPHER_KEY_MAPPING_KEY_VALUE>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for DOT11_CIPHER_KEY_MAPPING_KEY_VALUE {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for DOT11_CIPHER_KEY_MAPPING_KEY_VALUE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
pub const DOT11_CIPHER_KEY_MAPPING_KEY_VALUE_BYTE_ARRAY_REVISION_1: u32 = 1u32;
pub const DOT11_CONF_ALGO_TKIP: u32 = 2u32;
pub const DOT11_CONF_ALGO_WEP_RC4: u32 = 1u32;
#[repr(C)]
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
pub struct DOT11_CONNECTION_COMPLETION_PARAMETERS {
    pub Header: super::Ndis::NDIS_OBJECT_HEADER,
    pub uStatus: u32,
}
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
impl ::core::marker::Copy for DOT11_CONNECTION_COMPLETION_PARAMETERS {}
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
impl ::core::clone::Clone for DOT11_CONNECTION_COMPLETION_PARAMETERS {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
unsafe impl ::windows::core::Abi for DOT11_CONNECTION_COMPLETION_PARAMETERS {
    type Abi = Self;
}
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
impl ::core::cmp::PartialEq for DOT11_CONNECTION_COMPLETION_PARAMETERS {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<DOT11_CONNECTION_COMPLETION_PARAMETERS>()) == 0 }
    }
}
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
impl ::core::cmp::Eq for DOT11_CONNECTION_COMPLETION_PARAMETERS {}
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
impl ::core::default::Default for DOT11_CONNECTION_COMPLETION_PARAMETERS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
pub const DOT11_CONNECTION_COMPLETION_PARAMETERS_REVISION_1: u32 = 1u32;
#[repr(C)]
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
pub struct DOT11_CONNECTION_START_PARAMETERS {
    pub Header: super::Ndis::NDIS_OBJECT_HEADER,
    pub BSSType: DOT11_BSS_TYPE,
    pub AdhocBSSID: [u8; 6],
    pub AdhocSSID: DOT11_SSID,
}
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
impl ::core::marker::Copy for DOT11_CONNECTION_START_PARAMETERS {}
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
impl ::core::clone::Clone for DOT11_CONNECTION_START_PARAMETERS {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
unsafe impl ::windows::core::Abi for DOT11_CONNECTION_START_PARAMETERS {
    type Abi = Self;
}
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
impl ::core::cmp::PartialEq for DOT11_CONNECTION_START_PARAMETERS {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<DOT11_CONNECTION_START_PARAMETERS>()) == 0 }
    }
}
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
impl ::core::cmp::Eq for DOT11_CONNECTION_START_PARAMETERS {}
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
impl ::core::default::Default for DOT11_CONNECTION_START_PARAMETERS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
pub const DOT11_CONNECTION_START_PARAMETERS_REVISION_1: u32 = 1u32;
pub const DOT11_CONNECTION_STATUS_SUCCESS: u32 = 0u32;
#[repr(C)]
pub struct DOT11_COUNTERS_ENTRY {
    pub uTransmittedFragmentCount: u32,
    pub uMulticastTransmittedFrameCount: u32,
    pub uFailedCount: u32,
    pub uRetryCount: u32,
    pub uMultipleRetryCount: u32,
    pub uFrameDuplicateCount: u32,
    pub uRTSSuccessCount: u32,
    pub uRTSFailureCount: u32,
    pub uACKFailureCount: u32,
    pub uReceivedFragmentCount: u32,
    pub uMulticastReceivedFrameCount: u32,
    pub uFCSErrorCount: u32,
    pub uTransmittedFrameCount: u32,
}
impl ::core::marker::Copy for DOT11_COUNTERS_ENTRY {}
impl ::core::clone::Clone for DOT11_COUNTERS_ENTRY {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for DOT11_COUNTERS_ENTRY {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for DOT11_COUNTERS_ENTRY {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<DOT11_COUNTERS_ENTRY>()) == 0 }
    }
}
impl ::core::cmp::Eq for DOT11_COUNTERS_ENTRY {}
impl ::core::default::Default for DOT11_COUNTERS_ENTRY {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
pub struct DOT11_COUNTRY_OR_REGION_STRING_LIST {
    pub Header: super::Ndis::NDIS_OBJECT_HEADER,
    pub uNumOfEntries: u32,
    pub uTotalNumOfEntries: u32,
    pub CountryOrRegionStrings: [u8; 3],
}
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
impl ::core::marker::Copy for DOT11_COUNTRY_OR_REGION_STRING_LIST {}
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
impl ::core::clone::Clone for DOT11_COUNTRY_OR_REGION_STRING_LIST {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
unsafe impl ::windows::core::Abi for DOT11_COUNTRY_OR_REGION_STRING_LIST {
    type Abi = Self;
}
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
impl ::core::cmp::PartialEq for DOT11_COUNTRY_OR_REGION_STRING_LIST {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<DOT11_COUNTRY_OR_REGION_STRING_LIST>()) == 0 }
    }
}
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
impl ::core::cmp::Eq for DOT11_COUNTRY_OR_REGION_STRING_LIST {}
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
impl ::core::default::Default for DOT11_COUNTRY_OR_REGION_STRING_LIST {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
pub const DOT11_COUNTRY_OR_REGION_STRING_LIST_REVISION_1: u32 = 1u32;
#[repr(C)]
pub struct DOT11_CURRENT_OFFLOAD_CAPABILITY {
    pub uReserved: u32,
    pub uFlags: u32,
}
impl ::core::marker::Copy for DOT11_CURRENT_OFFLOAD_CAPABILITY {}
impl ::core::clone::Clone for DOT11_CURRENT_OFFLOAD_CAPABILITY {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for DOT11_CURRENT_OFFLOAD_CAPABILITY {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for DOT11_CURRENT_OFFLOAD_CAPABILITY {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<DOT11_CURRENT_OFFLOAD_CAPABILITY>()) == 0 }
    }
}
impl ::core::cmp::Eq for DOT11_CURRENT_OFFLOAD_CAPABILITY {}
impl ::core::default::Default for DOT11_CURRENT_OFFLOAD_CAPABILITY {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct DOT11_CURRENT_OPERATION_MODE {
    pub uReserved: u32,
    pub uCurrentOpMode: u32,
}
impl ::core::marker::Copy for DOT11_CURRENT_OPERATION_MODE {}
impl ::core::clone::Clone for DOT11_CURRENT_OPERATION_MODE {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for DOT11_CURRENT_OPERATION_MODE {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for DOT11_CURRENT_OPERATION_MODE {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<DOT11_CURRENT_OPERATION_MODE>()) == 0 }
    }
}
impl ::core::cmp::Eq for DOT11_CURRENT_OPERATION_MODE {}
impl ::core::default::Default for DOT11_CURRENT_OPERATION_MODE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct DOT11_CURRENT_OPTIONAL_CAPABILITY {
    pub uReserved: u32,
    pub bDot11CFPollable: super::super::Foundation::BOOLEAN,
    pub bDot11PCF: super::super::Foundation::BOOLEAN,
    pub bDot11PCFMPDUTransferToPC: super::super::Foundation::BOOLEAN,
    pub bStrictlyOrderedServiceClass: super::super::Foundation::BOOLEAN,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for DOT11_CURRENT_OPTIONAL_CAPABILITY {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for DOT11_CURRENT_OPTIONAL_CAPABILITY {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for DOT11_CURRENT_OPTIONAL_CAPABILITY {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for DOT11_CURRENT_OPTIONAL_CAPABILITY {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<DOT11_CURRENT_OPTIONAL_CAPABILITY>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for DOT11_CURRENT_OPTIONAL_CAPABILITY {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for DOT11_CURRENT_OPTIONAL_CAPABILITY {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct DOT11_DATA_RATE_MAPPING_ENTRY {
    pub ucDataRateIndex: u8,
    pub ucDataRateFlag: u8,
    pub usDataRateValue: u16,
}
impl ::core::marker::Copy for DOT11_DATA_RATE_MAPPING_ENTRY {}
impl ::core::clone::Clone for DOT11_DATA_RATE_MAPPING_ENTRY {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for DOT11_DATA_RATE_MAPPING_ENTRY {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for DOT11_DATA_RATE_MAPPING_ENTRY {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<DOT11_DATA_RATE_MAPPING_ENTRY>()) == 0 }
    }
}
impl ::core::cmp::Eq for DOT11_DATA_RATE_MAPPING_ENTRY {}
impl ::core::default::Default for DOT11_DATA_RATE_MAPPING_ENTRY {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
pub struct DOT11_DATA_RATE_MAPPING_TABLE {
    pub Header: super::Ndis::NDIS_OBJECT_HEADER,
    pub uDataRateMappingLength: u32,
    pub DataRateMappingEntries: [DOT11_DATA_RATE_MAPPING_ENTRY; 126],
}
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
impl ::core::marker::Copy for DOT11_DATA_RATE_MAPPING_TABLE {}
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
impl ::core::clone::Clone for DOT11_DATA_RATE_MAPPING_TABLE {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
unsafe impl ::windows::core::Abi for DOT11_DATA_RATE_MAPPING_TABLE {
    type Abi = Self;
}
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
impl ::core::cmp::PartialEq for DOT11_DATA_RATE_MAPPING_TABLE {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<DOT11_DATA_RATE_MAPPING_TABLE>()) == 0 }
    }
}
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
impl ::core::cmp::Eq for DOT11_DATA_RATE_MAPPING_TABLE {}
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
impl ::core::default::Default for DOT11_DATA_RATE_MAPPING_TABLE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
pub const DOT11_DATA_RATE_MAPPING_TABLE_REVISION_1: u32 = 1u32;
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct DOT11_DEFAULT_WEP_OFFLOAD {
    pub uReserved: u32,
    pub hOffloadContext: super::super::Foundation::HANDLE,
    pub hOffload: super::super::Foundation::HANDLE,
    pub dwIndex: u32,
    pub dot11OffloadType: DOT11_OFFLOAD_TYPE,
    pub dwAlgorithm: u32,
    pub uFlags: u32,
    pub dot11KeyDirection: DOT11_KEY_DIRECTION,
    pub ucMacAddress: [u8; 6],
    pub uNumOfRWsOnMe: u32,
    pub dot11IV48Counters: [DOT11_IV48_COUNTER; 16],
    pub usDot11RWBitMaps: [u16; 16],
    pub usKeyLength: u16,
    pub ucKey: [u8; 1],
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for DOT11_DEFAULT_WEP_OFFLOAD {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for DOT11_DEFAULT_WEP_OFFLOAD {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for DOT11_DEFAULT_WEP_OFFLOAD {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for DOT11_DEFAULT_WEP_OFFLOAD {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<DOT11_DEFAULT_WEP_OFFLOAD>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for DOT11_DEFAULT_WEP_OFFLOAD {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for DOT11_DEFAULT_WEP_OFFLOAD {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct DOT11_DEFAULT_WEP_UPLOAD {
    pub uReserved: u32,
    pub dot11OffloadType: DOT11_OFFLOAD_TYPE,
    pub hOffload: super::super::Foundation::HANDLE,
    pub uNumOfRWsUsed: u32,
    pub dot11IV48Counters: [DOT11_IV48_COUNTER; 16],
    pub usDot11RWBitMaps: [u16; 16],
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for DOT11_DEFAULT_WEP_UPLOAD {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for DOT11_DEFAULT_WEP_UPLOAD {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for DOT11_DEFAULT_WEP_UPLOAD {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for DOT11_DEFAULT_WEP_UPLOAD {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<DOT11_DEFAULT_WEP_UPLOAD>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for DOT11_DEFAULT_WEP_UPLOAD {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for DOT11_DEFAULT_WEP_UPLOAD {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
pub const DOT11_DEVICE_ENTRY_BYTE_ARRAY_REVISION_1: u32 = 1u32;
pub type DOT11_DIRECTION = i32;
pub const DOT11_DIR_INBOUND: DOT11_DIRECTION = 1i32;
pub const DOT11_DIR_OUTBOUND: DOT11_DIRECTION = 2i32;
pub const DOT11_DIR_BOTH: DOT11_DIRECTION = 3i32;
#[repr(C)]
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
pub struct DOT11_DISASSOCIATE_PEER_REQUEST {
    pub Header: super::Ndis::NDIS_OBJECT_HEADER,
    pub PeerMacAddr: [u8; 6],
    pub usReason: u16,
}
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
impl ::core::marker::Copy for DOT11_DISASSOCIATE_PEER_REQUEST {}
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
impl ::core::clone::Clone for DOT11_DISASSOCIATE_PEER_REQUEST {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
unsafe impl ::windows::core::Abi for DOT11_DISASSOCIATE_PEER_REQUEST {
    type Abi = Self;
}
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
impl ::core::cmp::PartialEq for DOT11_DISASSOCIATE_PEER_REQUEST {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<DOT11_DISASSOCIATE_PEER_REQUEST>()) == 0 }
    }
}
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
impl ::core::cmp::Eq for DOT11_DISASSOCIATE_PEER_REQUEST {}
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
impl ::core::default::Default for DOT11_DISASSOCIATE_PEER_REQUEST {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
pub const DOT11_DISASSOCIATE_PEER_REQUEST_REVISION_1: u32 = 1u32;
#[repr(C)]
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
pub struct DOT11_DISASSOCIATION_PARAMETERS {
    pub Header: super::Ndis::NDIS_OBJECT_HEADER,
    pub MacAddr: [u8; 6],
    pub uReason: u32,
    pub uIHVDataOffset: u32,
    pub uIHVDataSize: u32,
}
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
impl ::core::marker::Copy for DOT11_DISASSOCIATION_PARAMETERS {}
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
impl ::core::clone::Clone for DOT11_DISASSOCIATION_PARAMETERS {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
unsafe impl ::windows::core::Abi for DOT11_DISASSOCIATION_PARAMETERS {
    type Abi = Self;
}
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
impl ::core::cmp::PartialEq for DOT11_DISASSOCIATION_PARAMETERS {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<DOT11_DISASSOCIATION_PARAMETERS>()) == 0 }
    }
}
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
impl ::core::cmp::Eq for DOT11_DISASSOCIATION_PARAMETERS {}
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
impl ::core::default::Default for DOT11_DISASSOCIATION_PARAMETERS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
pub const DOT11_DISASSOCIATION_PARAMETERS_REVISION_1: u32 = 1u32;
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct DOT11_DIVERSITY_SELECTION_RX {
    pub uAntennaListIndex: u32,
    pub bDiversitySelectionRX: super::super::Foundation::BOOLEAN,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for DOT11_DIVERSITY_SELECTION_RX {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for DOT11_DIVERSITY_SELECTION_RX {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for DOT11_DIVERSITY_SELECTION_RX {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for DOT11_DIVERSITY_SELECTION_RX {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<DOT11_DIVERSITY_SELECTION_RX>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for DOT11_DIVERSITY_SELECTION_RX {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for DOT11_DIVERSITY_SELECTION_RX {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct DOT11_DIVERSITY_SELECTION_RX_LIST {
    pub uNumOfEntries: u32,
    pub uTotalNumOfEntries: u32,
    pub dot11DiversitySelectionRx: [DOT11_DIVERSITY_SELECTION_RX; 1],
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for DOT11_DIVERSITY_SELECTION_RX_LIST {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for DOT11_DIVERSITY_SELECTION_RX_LIST {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for DOT11_DIVERSITY_SELECTION_RX_LIST {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for DOT11_DIVERSITY_SELECTION_RX_LIST {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<DOT11_DIVERSITY_SELECTION_RX_LIST>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for DOT11_DIVERSITY_SELECTION_RX_LIST {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for DOT11_DIVERSITY_SELECTION_RX_LIST {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
pub type DOT11_DIVERSITY_SUPPORT = i32;
pub const dot11_diversity_support_unknown: DOT11_DIVERSITY_SUPPORT = 0i32;
pub const dot11_diversity_support_fixedlist: DOT11_DIVERSITY_SUPPORT = 1i32;
pub const dot11_diversity_support_notsupported: DOT11_DIVERSITY_SUPPORT = 2i32;
pub const dot11_diversity_support_dynamic: DOT11_DIVERSITY_SUPPORT = 3i32;
pub type DOT11_DS_INFO = i32;
pub const DOT11_DS_CHANGED: DOT11_DS_INFO = 0i32;
pub const DOT11_DS_UNCHANGED: DOT11_DS_INFO = 1i32;
pub const DOT11_DS_UNKNOWN: DOT11_DS_INFO = 2i32;
pub const DOT11_ENCAP_802_1H: u32 = 2u32;
#[repr(C)]
pub struct DOT11_ENCAP_ENTRY {
    pub usEtherType: u16,
    pub usEncapType: u16,
}
impl ::core::marker::Copy for DOT11_ENCAP_ENTRY {}
impl ::core::clone::Clone for DOT11_ENCAP_ENTRY {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for DOT11_ENCAP_ENTRY {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for DOT11_ENCAP_ENTRY {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<DOT11_ENCAP_ENTRY>()) == 0 }
    }
}
impl ::core::cmp::Eq for DOT11_ENCAP_ENTRY {}
impl ::core::default::Default for DOT11_ENCAP_ENTRY {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
pub const DOT11_ENCAP_RFC_1042: u32 = 1u32;
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct DOT11_ERP_PHY_ATTRIBUTES {
    pub HRDSSSAttributes: DOT11_HRDSSS_PHY_ATTRIBUTES,
    pub bERPPBCCOptionImplemented: super::super::Foundation::BOOLEAN,
    pub bDSSSOFDMOptionImplemented: super::super::Foundation::BOOLEAN,
    pub bShortSlotTimeOptionImplemented: super::super::Foundation::BOOLEAN,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for DOT11_ERP_PHY_ATTRIBUTES {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for DOT11_ERP_PHY_ATTRIBUTES {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for DOT11_ERP_PHY_ATTRIBUTES {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for DOT11_ERP_PHY_ATTRIBUTES {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<DOT11_ERP_PHY_ATTRIBUTES>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for DOT11_ERP_PHY_ATTRIBUTES {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for DOT11_ERP_PHY_ATTRIBUTES {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
pub const DOT11_EXEMPT_ALWAYS: u32 = 1u32;
pub const DOT11_EXEMPT_BOTH: u32 = 3u32;
pub const DOT11_EXEMPT_MULTICAST: u32 = 2u32;
pub const DOT11_EXEMPT_NO_EXEMPTION: u32 = 0u32;
pub const DOT11_EXEMPT_ON_KEY_MAPPING_KEY_UNAVAILABLE: u32 = 2u32;
pub const DOT11_EXEMPT_UNICAST: u32 = 1u32;
#[repr(C)]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_NetworkManagement_Ndis"))]
pub struct DOT11_EXTAP_ATTRIBUTES {
    pub Header: super::Ndis::NDIS_OBJECT_HEADER,
    pub uScanSSIDListSize: u32,
    pub uDesiredSSIDListSize: u32,
    pub uPrivacyExemptionListSize: u32,
    pub uAssociationTableSize: u32,
    pub uDefaultKeyTableSize: u32,
    pub uWEPKeyValueMaxLength: u32,
    pub bStrictlyOrderedServiceClassImplemented: super::super::Foundation::BOOLEAN,
    pub uNumSupportedCountryOrRegionStrings: u32,
    pub pSupportedCountryOrRegionStrings: *mut u8,
    pub uInfraNumSupportedUcastAlgoPairs: u32,
    pub pInfraSupportedUcastAlgoPairs: *mut DOT11_AUTH_CIPHER_PAIR,
    pub uInfraNumSupportedMcastAlgoPairs: u32,
    pub pInfraSupportedMcastAlgoPairs: *mut DOT11_AUTH_CIPHER_PAIR,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_NetworkManagement_Ndis"))]
impl ::core::marker::Copy for DOT11_EXTAP_ATTRIBUTES {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_NetworkManagement_Ndis"))]
impl ::core::clone::Clone for DOT11_EXTAP_ATTRIBUTES {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_NetworkManagement_Ndis"))]
unsafe impl ::windows::core::Abi for DOT11_EXTAP_ATTRIBUTES {
    type Abi = Self;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_NetworkManagement_Ndis"))]
impl ::core::cmp::PartialEq for DOT11_EXTAP_ATTRIBUTES {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<DOT11_EXTAP_ATTRIBUTES>()) == 0 }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_NetworkManagement_Ndis"))]
impl ::core::cmp::Eq for DOT11_EXTAP_ATTRIBUTES {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_NetworkManagement_Ndis"))]
impl ::core::default::Default for DOT11_EXTAP_ATTRIBUTES {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
pub const DOT11_EXTAP_ATTRIBUTES_REVISION_1: u32 = 1u32;
pub const DOT11_EXTAP_RECV_CONTEXT_REVISION_1: u32 = 1u32;
pub const DOT11_EXTAP_SEND_CONTEXT_REVISION_1: u32 = 1u32;
#[repr(C)]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_NetworkManagement_Ndis"))]
pub struct DOT11_EXTSTA_ATTRIBUTES {
    pub Header: super::Ndis::NDIS_OBJECT_HEADER,
    pub uScanSSIDListSize: u32,
    pub uDesiredBSSIDListSize: u32,
    pub uDesiredSSIDListSize: u32,
    pub uExcludedMacAddressListSize: u32,
    pub uPrivacyExemptionListSize: u32,
    pub uKeyMappingTableSize: u32,
    pub uDefaultKeyTableSize: u32,
    pub uWEPKeyValueMaxLength: u32,
    pub uPMKIDCacheSize: u32,
    pub uMaxNumPerSTADefaultKeyTables: u32,
    pub bStrictlyOrderedServiceClassImplemented: super::super::Foundation::BOOLEAN,
    pub ucSupportedQoSProtocolFlags: u8,
    pub bSafeModeImplemented: super::super::Foundation::BOOLEAN,
    pub uNumSupportedCountryOrRegionStrings: u32,
    pub pSupportedCountryOrRegionStrings: *mut u8,
    pub uInfraNumSupportedUcastAlgoPairs: u32,
    pub pInfraSupportedUcastAlgoPairs: *mut DOT11_AUTH_CIPHER_PAIR,
    pub uInfraNumSupportedMcastAlgoPairs: u32,
    pub pInfraSupportedMcastAlgoPairs: *mut DOT11_AUTH_CIPHER_PAIR,
    pub uAdhocNumSupportedUcastAlgoPairs: u32,
    pub pAdhocSupportedUcastAlgoPairs: *mut DOT11_AUTH_CIPHER_PAIR,
    pub uAdhocNumSupportedMcastAlgoPairs: u32,
    pub pAdhocSupportedMcastAlgoPairs: *mut DOT11_AUTH_CIPHER_PAIR,
    pub bAutoPowerSaveMode: super::super::Foundation::BOOLEAN,
    pub uMaxNetworkOffloadListSize: u32,
    pub bMFPCapable: super::super::Foundation::BOOLEAN,
    pub uInfraNumSupportedMcastMgmtAlgoPairs: u32,
    pub pInfraSupportedMcastMgmtAlgoPairs: *mut DOT11_AUTH_CIPHER_PAIR,
    pub bNeighborReportSupported: super::super::Foundation::BOOLEAN,
    pub bAPChannelReportSupported: super::super::Foundation::BOOLEAN,
    pub bActionFramesSupported: super::super::Foundation::BOOLEAN,
    pub bANQPQueryOffloadSupported: super::super::Foundation::BOOLEAN,
    pub bHESSIDConnectionSupported: super::super::Foundation::BOOLEAN,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_NetworkManagement_Ndis"))]
impl ::core::marker::Copy for DOT11_EXTSTA_ATTRIBUTES {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_NetworkManagement_Ndis"))]
impl ::core::clone::Clone for DOT11_EXTSTA_ATTRIBUTES {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_NetworkManagement_Ndis"))]
unsafe impl ::windows::core::Abi for DOT11_EXTSTA_ATTRIBUTES {
    type Abi = Self;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_NetworkManagement_Ndis"))]
impl ::core::cmp::PartialEq for DOT11_EXTSTA_ATTRIBUTES {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<DOT11_EXTSTA_ATTRIBUTES>()) == 0 }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_NetworkManagement_Ndis"))]
impl ::core::cmp::Eq for DOT11_EXTSTA_ATTRIBUTES {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_NetworkManagement_Ndis"))]
impl ::core::default::Default for DOT11_EXTSTA_ATTRIBUTES {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
pub const DOT11_EXTSTA_ATTRIBUTES_REVISION_1: u32 = 1u32;
pub const DOT11_EXTSTA_ATTRIBUTES_REVISION_2: u32 = 2u32;
pub const DOT11_EXTSTA_ATTRIBUTES_REVISION_3: u32 = 3u32;
pub const DOT11_EXTSTA_ATTRIBUTES_REVISION_4: u32 = 4u32;
pub const DOT11_EXTSTA_ATTRIBUTES_SAFEMODE_CERTIFIED: u32 = 2u32;
pub const DOT11_EXTSTA_ATTRIBUTES_SAFEMODE_OID_SUPPORTED: u32 = 1u32;
pub const DOT11_EXTSTA_ATTRIBUTES_SAFEMODE_RESERVED: u32 = 12u32;
#[repr(C)]
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
pub struct DOT11_EXTSTA_CAPABILITY {
    pub Header: super::Ndis::NDIS_OBJECT_HEADER,
    pub uScanSSIDListSize: u32,
    pub uDesiredBSSIDListSize: u32,
    pub uDesiredSSIDListSize: u32,
    pub uExcludedMacAddressListSize: u32,
    pub uPrivacyExemptionListSize: u32,
    pub uKeyMappingTableSize: u32,
    pub uDefaultKeyTableSize: u32,
    pub uWEPKeyValueMaxLength: u32,
    pub uPMKIDCacheSize: u32,
    pub uMaxNumPerSTADefaultKeyTables: u32,
}
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
impl ::core::marker::Copy for DOT11_EXTSTA_CAPABILITY {}
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
impl ::core::clone::Clone for DOT11_EXTSTA_CAPABILITY {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
unsafe impl ::windows::core::Abi for DOT11_EXTSTA_CAPABILITY {
    type Abi = Self;
}
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
impl ::core::cmp::PartialEq for DOT11_EXTSTA_CAPABILITY {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<DOT11_EXTSTA_CAPABILITY>()) == 0 }
    }
}
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
impl ::core::cmp::Eq for DOT11_EXTSTA_CAPABILITY {}
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
impl ::core::default::Default for DOT11_EXTSTA_CAPABILITY {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
pub const DOT11_EXTSTA_CAPABILITY_REVISION_1: u32 = 1u32;
#[repr(C)]
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
pub struct DOT11_EXTSTA_RECV_CONTEXT {
    pub Header: super::Ndis::NDIS_OBJECT_HEADER,
    pub uReceiveFlags: u32,
    pub uPhyId: u32,
    pub uChCenterFrequency: u32,
    pub usNumberOfMPDUsReceived: u16,
    pub lRSSI: i32,
    pub ucDataRate: u8,
    pub uSizeMediaSpecificInfo: u32,
    pub pvMediaSpecificInfo: *mut ::core::ffi::c_void,
    pub ullTimestamp: u64,
}
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
impl ::core::marker::Copy for DOT11_EXTSTA_RECV_CONTEXT {}
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
impl ::core::clone::Clone for DOT11_EXTSTA_RECV_CONTEXT {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
unsafe impl ::windows::core::Abi for DOT11_EXTSTA_RECV_CONTEXT {
    type Abi = Self;
}
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
impl ::core::cmp::PartialEq for DOT11_EXTSTA_RECV_CONTEXT {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<DOT11_EXTSTA_RECV_CONTEXT>()) == 0 }
    }
}
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
impl ::core::cmp::Eq for DOT11_EXTSTA_RECV_CONTEXT {}
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
impl ::core::default::Default for DOT11_EXTSTA_RECV_CONTEXT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
pub const DOT11_EXTSTA_RECV_CONTEXT_REVISION_1: u32 = 1u32;
#[repr(C)]
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
pub struct DOT11_EXTSTA_SEND_CONTEXT {
    pub Header: super::Ndis::NDIS_OBJECT_HEADER,
    pub usExemptionActionType: u16,
    pub uPhyId: u32,
    pub uDelayedSleepValue: u32,
    pub pvMediaSpecificInfo: *mut ::core::ffi::c_void,
    pub uSendFlags: u32,
}
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
impl ::core::marker::Copy for DOT11_EXTSTA_SEND_CONTEXT {}
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
impl ::core::clone::Clone for DOT11_EXTSTA_SEND_CONTEXT {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
unsafe impl ::windows::core::Abi for DOT11_EXTSTA_SEND_CONTEXT {
    type Abi = Self;
}
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
impl ::core::cmp::PartialEq for DOT11_EXTSTA_SEND_CONTEXT {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<DOT11_EXTSTA_SEND_CONTEXT>()) == 0 }
    }
}
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
impl ::core::cmp::Eq for DOT11_EXTSTA_SEND_CONTEXT {}
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
impl ::core::default::Default for DOT11_EXTSTA_SEND_CONTEXT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
pub const DOT11_EXTSTA_SEND_CONTEXT_REVISION_1: u32 = 1u32;
pub const DOT11_FLAGS_80211B_CHANNEL_AGILITY: u32 = 4u32;
pub const DOT11_FLAGS_80211B_PBCC: u32 = 2u32;
pub const DOT11_FLAGS_80211B_SHORT_PREAMBLE: u32 = 1u32;
pub const DOT11_FLAGS_80211G_BARKER_PREAMBLE_MODE: u32 = 128u32;
pub const DOT11_FLAGS_80211G_DSSS_OFDM: u32 = 16u32;
pub const DOT11_FLAGS_80211G_NON_ERP_PRESENT: u32 = 64u32;
pub const DOT11_FLAGS_80211G_USE_PROTECTION: u32 = 32u32;
pub const DOT11_FLAGS_PS_ON: u32 = 8u32;
#[repr(C)]
pub struct DOT11_FRAGMENT_DESCRIPTOR {
    pub uOffset: u32,
    pub uLength: u32,
}
impl ::core::marker::Copy for DOT11_FRAGMENT_DESCRIPTOR {}
impl ::core::clone::Clone for DOT11_FRAGMENT_DESCRIPTOR {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for DOT11_FRAGMENT_DESCRIPTOR {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for DOT11_FRAGMENT_DESCRIPTOR {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<DOT11_FRAGMENT_DESCRIPTOR>()) == 0 }
    }
}
impl ::core::cmp::Eq for DOT11_FRAGMENT_DESCRIPTOR {}
impl ::core::default::Default for DOT11_FRAGMENT_DESCRIPTOR {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
pub const DOT11_FREQUENCY_BANDS_LOWER: u32 = 1u32;
pub const DOT11_FREQUENCY_BANDS_MIDDLE: u32 = 2u32;
pub const DOT11_FREQUENCY_BANDS_UPPER: u32 = 4u32;
#[repr(C)]
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
pub struct DOT11_GO_NEGOTIATION_CONFIRMATION_SEND_COMPLETE_PARAMETERS {
    pub Header: super::Ndis::NDIS_OBJECT_HEADER,
    pub PeerDeviceAddress: [u8; 6],
    pub DialogToken: u8,
    pub Status: i32,
    pub uIEsOffset: u32,
    pub uIEsLength: u32,
}
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
impl ::core::marker::Copy for DOT11_GO_NEGOTIATION_CONFIRMATION_SEND_COMPLETE_PARAMETERS {}
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
impl ::core::clone::Clone for DOT11_GO_NEGOTIATION_CONFIRMATION_SEND_COMPLETE_PARAMETERS {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
unsafe impl ::windows::core::Abi for DOT11_GO_NEGOTIATION_CONFIRMATION_SEND_COMPLETE_PARAMETERS {
    type Abi = Self;
}
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
impl ::core::cmp::PartialEq for DOT11_GO_NEGOTIATION_CONFIRMATION_SEND_COMPLETE_PARAMETERS {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<DOT11_GO_NEGOTIATION_CONFIRMATION_SEND_COMPLETE_PARAMETERS>()) == 0 }
    }
}
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
impl ::core::cmp::Eq for DOT11_GO_NEGOTIATION_CONFIRMATION_SEND_COMPLETE_PARAMETERS {}
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
impl ::core::default::Default for DOT11_GO_NEGOTIATION_CONFIRMATION_SEND_COMPLETE_PARAMETERS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
pub const DOT11_GO_NEGOTIATION_CONFIRMATION_SEND_COMPLETE_PARAMETERS_REVISION_1: u32 = 1u32;
#[repr(C)]
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
pub struct DOT11_GO_NEGOTIATION_REQUEST_SEND_COMPLETE_PARAMETERS {
    pub Header: super::Ndis::NDIS_OBJECT_HEADER,
    pub PeerDeviceAddress: [u8; 6],
    pub DialogToken: u8,
    pub Status: i32,
    pub uIEsOffset: u32,
    pub uIEsLength: u32,
}
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
impl ::core::marker::Copy for DOT11_GO_NEGOTIATION_REQUEST_SEND_COMPLETE_PARAMETERS {}
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
impl ::core::clone::Clone for DOT11_GO_NEGOTIATION_REQUEST_SEND_COMPLETE_PARAMETERS {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
unsafe impl ::windows::core::Abi for DOT11_GO_NEGOTIATION_REQUEST_SEND_COMPLETE_PARAMETERS {
    type Abi = Self;
}
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
impl ::core::cmp::PartialEq for DOT11_GO_NEGOTIATION_REQUEST_SEND_COMPLETE_PARAMETERS {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<DOT11_GO_NEGOTIATION_REQUEST_SEND_COMPLETE_PARAMETERS>()) == 0 }
    }
}
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
impl ::core::cmp::Eq for DOT11_GO_NEGOTIATION_REQUEST_SEND_COMPLETE_PARAMETERS {}
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
impl ::core::default::Default for DOT11_GO_NEGOTIATION_REQUEST_SEND_COMPLETE_PARAMETERS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
pub const DOT11_GO_NEGOTIATION_REQUEST_SEND_COMPLETE_PARAMETERS_REVISION_1: u32 = 1u32;
#[repr(C)]
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
pub struct DOT11_GO_NEGOTIATION_RESPONSE_SEND_COMPLETE_PARAMETERS {
    pub Header: super::Ndis::NDIS_OBJECT_HEADER,
    pub PeerDeviceAddress: [u8; 6],
    pub DialogToken: u8,
    pub Status: i32,
    pub uIEsOffset: u32,
    pub uIEsLength: u32,
}
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
impl ::core::marker::Copy for DOT11_GO_NEGOTIATION_RESPONSE_SEND_COMPLETE_PARAMETERS {}
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
impl ::core::clone::Clone for DOT11_GO_NEGOTIATION_RESPONSE_SEND_COMPLETE_PARAMETERS {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
unsafe impl ::windows::core::Abi for DOT11_GO_NEGOTIATION_RESPONSE_SEND_COMPLETE_PARAMETERS {
    type Abi = Self;
}
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
impl ::core::cmp::PartialEq for DOT11_GO_NEGOTIATION_RESPONSE_SEND_COMPLETE_PARAMETERS {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<DOT11_GO_NEGOTIATION_RESPONSE_SEND_COMPLETE_PARAMETERS>()) == 0 }
    }
}
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
impl ::core::cmp::Eq for DOT11_GO_NEGOTIATION_RESPONSE_SEND_COMPLETE_PARAMETERS {}
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
impl ::core::default::Default for DOT11_GO_NEGOTIATION_RESPONSE_SEND_COMPLETE_PARAMETERS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
pub const DOT11_GO_NEGOTIATION_RESPONSE_SEND_COMPLETE_PARAMETERS_REVISION_1: u32 = 1u32;
pub const DOT11_HESSID_LENGTH: u32 = 6u32;
#[repr(C)]
pub struct DOT11_HOPPING_PATTERN_ENTRY {
    pub uHoppingPatternIndex: u32,
    pub uRandomTableFieldNumber: u32,
}
impl ::core::marker::Copy for DOT11_HOPPING_PATTERN_ENTRY {}
impl ::core::clone::Clone for DOT11_HOPPING_PATTERN_ENTRY {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for DOT11_HOPPING_PATTERN_ENTRY {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for DOT11_HOPPING_PATTERN_ENTRY {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<DOT11_HOPPING_PATTERN_ENTRY>()) == 0 }
    }
}
impl ::core::cmp::Eq for DOT11_HOPPING_PATTERN_ENTRY {}
impl ::core::default::Default for DOT11_HOPPING_PATTERN_ENTRY {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct DOT11_HOPPING_PATTERN_ENTRY_LIST {
    pub uNumOfEntries: u32,
    pub uTotalNumOfEntries: u32,
    pub dot11HoppingPatternEntry: [DOT11_HOPPING_PATTERN_ENTRY; 1],
}
impl ::core::marker::Copy for DOT11_HOPPING_PATTERN_ENTRY_LIST {}
impl ::core::clone::Clone for DOT11_HOPPING_PATTERN_ENTRY_LIST {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for DOT11_HOPPING_PATTERN_ENTRY_LIST {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for DOT11_HOPPING_PATTERN_ENTRY_LIST {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<DOT11_HOPPING_PATTERN_ENTRY_LIST>()) == 0 }
    }
}
impl ::core::cmp::Eq for DOT11_HOPPING_PATTERN_ENTRY_LIST {}
impl ::core::default::Default for DOT11_HOPPING_PATTERN_ENTRY_LIST {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
pub type DOT11_HOP_ALGO_ADOPTED = i32;
pub const dot11_hop_algo_current: DOT11_HOP_ALGO_ADOPTED = 0i32;
pub const dot11_hop_algo_hop_index: DOT11_HOP_ALGO_ADOPTED = 1i32;
pub const dot11_hop_algo_hcc: DOT11_HOP_ALGO_ADOPTED = 2i32;
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct DOT11_HRDSSS_PHY_ATTRIBUTES {
    pub bShortPreambleOptionImplemented: super::super::Foundation::BOOLEAN,
    pub bPBCCOptionImplemented: super::super::Foundation::BOOLEAN,
    pub bChannelAgilityPresent: super::super::Foundation::BOOLEAN,
    pub uHRCCAModeSupported: u32,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for DOT11_HRDSSS_PHY_ATTRIBUTES {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for DOT11_HRDSSS_PHY_ATTRIBUTES {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for DOT11_HRDSSS_PHY_ATTRIBUTES {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for DOT11_HRDSSS_PHY_ATTRIBUTES {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<DOT11_HRDSSS_PHY_ATTRIBUTES>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for DOT11_HRDSSS_PHY_ATTRIBUTES {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for DOT11_HRDSSS_PHY_ATTRIBUTES {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
pub const DOT11_HR_CCA_MODE_CS_AND_ED: u32 = 4u32;
pub const DOT11_HR_CCA_MODE_CS_ONLY: u32 = 2u32;
pub const DOT11_HR_CCA_MODE_CS_WITH_TIMER: u32 = 8u32;
pub const DOT11_HR_CCA_MODE_ED_ONLY: u32 = 1u32;
pub const DOT11_HR_CCA_MODE_HRCS_AND_ED: u32 = 16u32;
pub const DOT11_HW_DEFRAGMENTATION_SUPPORTED: u32 = 8u32;
pub const DOT11_HW_FRAGMENTATION_SUPPORTED: u32 = 4u32;
pub const DOT11_HW_MSDU_AUTH_SUPPORTED_RX: u32 = 32u32;
pub const DOT11_HW_MSDU_AUTH_SUPPORTED_TX: u32 = 16u32;
pub const DOT11_HW_WEP_SUPPORTED_RX: u32 = 2u32;
pub const DOT11_HW_WEP_SUPPORTED_TX: u32 = 1u32;
#[repr(C)]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_NetworkManagement_Ndis"))]
pub struct DOT11_IBSS_PARAMS {
    pub Header: super::Ndis::NDIS_OBJECT_HEADER,
    pub bJoinOnly: super::super::Foundation::BOOLEAN,
    pub uIEsOffset: u32,
    pub uIEsLength: u32,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_NetworkManagement_Ndis"))]
impl ::core::marker::Copy for DOT11_IBSS_PARAMS {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_NetworkManagement_Ndis"))]
impl ::core::clone::Clone for DOT11_IBSS_PARAMS {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_NetworkManagement_Ndis"))]
unsafe impl ::windows::core::Abi for DOT11_IBSS_PARAMS {
    type Abi = Self;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_NetworkManagement_Ndis"))]
impl ::core::cmp::PartialEq for DOT11_IBSS_PARAMS {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<DOT11_IBSS_PARAMS>()) == 0 }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_NetworkManagement_Ndis"))]
impl ::core::cmp::Eq for DOT11_IBSS_PARAMS {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_NetworkManagement_Ndis"))]
impl ::core::default::Default for DOT11_IBSS_PARAMS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
pub const DOT11_IBSS_PARAMS_REVISION_1: u32 = 1u32;
#[repr(C)]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_NetworkManagement_Ndis"))]
pub struct DOT11_INCOMING_ASSOC_COMPLETION_PARAMETERS {
    pub Header: super::Ndis::NDIS_OBJECT_HEADER,
    pub PeerMacAddr: [u8; 6],
    pub uStatus: u32,
    pub ucErrorSource: u8,
    pub bReAssocReq: super::super::Foundation::BOOLEAN,
    pub bReAssocResp: super::super::Foundation::BOOLEAN,
    pub uAssocReqOffset: u32,
    pub uAssocReqSize: u32,
    pub uAssocRespOffset: u32,
    pub uAssocRespSize: u32,
    pub AuthAlgo: DOT11_AUTH_ALGORITHM,
    pub UnicastCipher: DOT11_CIPHER_ALGORITHM,
    pub MulticastCipher: DOT11_CIPHER_ALGORITHM,
    pub uActivePhyListOffset: u32,
    pub uActivePhyListSize: u32,
    pub uBeaconOffset: u32,
    pub uBeaconSize: u32,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_NetworkManagement_Ndis"))]
impl ::core::marker::Copy for DOT11_INCOMING_ASSOC_COMPLETION_PARAMETERS {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_NetworkManagement_Ndis"))]
impl ::core::clone::Clone for DOT11_INCOMING_ASSOC_COMPLETION_PARAMETERS {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_NetworkManagement_Ndis"))]
unsafe impl ::windows::core::Abi for DOT11_INCOMING_ASSOC_COMPLETION_PARAMETERS {
    type Abi = Self;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_NetworkManagement_Ndis"))]
impl ::core::cmp::PartialEq for DOT11_INCOMING_ASSOC_COMPLETION_PARAMETERS {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<DOT11_INCOMING_ASSOC_COMPLETION_PARAMETERS>()) == 0 }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_NetworkManagement_Ndis"))]
impl ::core::cmp::Eq for DOT11_INCOMING_ASSOC_COMPLETION_PARAMETERS {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_NetworkManagement_Ndis"))]
impl ::core::default::Default for DOT11_INCOMING_ASSOC_COMPLETION_PARAMETERS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
pub const DOT11_INCOMING_ASSOC_COMPLETION_PARAMETERS_REVISION_1: u32 = 1u32;
#[repr(C)]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_NetworkManagement_Ndis"))]
pub struct DOT11_INCOMING_ASSOC_DECISION {
    pub Header: super::Ndis::NDIS_OBJECT_HEADER,
    pub PeerMacAddr: [u8; 6],
    pub bAccept: super::super::Foundation::BOOLEAN,
    pub usReasonCode: u16,
    pub uAssocResponseIEsOffset: u32,
    pub uAssocResponseIEsLength: u32,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_NetworkManagement_Ndis"))]
impl ::core::marker::Copy for DOT11_INCOMING_ASSOC_DECISION {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_NetworkManagement_Ndis"))]
impl ::core::clone::Clone for DOT11_INCOMING_ASSOC_DECISION {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_NetworkManagement_Ndis"))]
unsafe impl ::windows::core::Abi for DOT11_INCOMING_ASSOC_DECISION {
    type Abi = Self;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_NetworkManagement_Ndis"))]
impl ::core::cmp::PartialEq for DOT11_INCOMING_ASSOC_DECISION {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<DOT11_INCOMING_ASSOC_DECISION>()) == 0 }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_NetworkManagement_Ndis"))]
impl ::core::cmp::Eq for DOT11_INCOMING_ASSOC_DECISION {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_NetworkManagement_Ndis"))]
impl ::core::default::Default for DOT11_INCOMING_ASSOC_DECISION {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
pub const DOT11_INCOMING_ASSOC_DECISION_REVISION_1: u32 = 1u32;
pub const DOT11_INCOMING_ASSOC_DECISION_REVISION_2: u32 = 2u32;
#[repr(C)]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_NetworkManagement_Ndis"))]
pub struct DOT11_INCOMING_ASSOC_DECISION_V2 {
    pub Header: super::Ndis::NDIS_OBJECT_HEADER,
    pub PeerMacAddr: [u8; 6],
    pub bAccept: super::super::Foundation::BOOLEAN,
    pub usReasonCode: u16,
    pub uAssocResponseIEsOffset: u32,
    pub uAssocResponseIEsLength: u32,
    pub WFDStatus: u8,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_NetworkManagement_Ndis"))]
impl ::core::marker::Copy for DOT11_INCOMING_ASSOC_DECISION_V2 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_NetworkManagement_Ndis"))]
impl ::core::clone::Clone for DOT11_INCOMING_ASSOC_DECISION_V2 {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_NetworkManagement_Ndis"))]
unsafe impl ::windows::core::Abi for DOT11_INCOMING_ASSOC_DECISION_V2 {
    type Abi = Self;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_NetworkManagement_Ndis"))]
impl ::core::cmp::PartialEq for DOT11_INCOMING_ASSOC_DECISION_V2 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<DOT11_INCOMING_ASSOC_DECISION_V2>()) == 0 }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_NetworkManagement_Ndis"))]
impl ::core::cmp::Eq for DOT11_INCOMING_ASSOC_DECISION_V2 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_NetworkManagement_Ndis"))]
impl ::core::default::Default for DOT11_INCOMING_ASSOC_DECISION_V2 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_NetworkManagement_Ndis"))]
pub struct DOT11_INCOMING_ASSOC_REQUEST_RECEIVED_PARAMETERS {
    pub Header: super::Ndis::NDIS_OBJECT_HEADER,
    pub PeerMacAddr: [u8; 6],
    pub bReAssocReq: super::super::Foundation::BOOLEAN,
    pub uAssocReqOffset: u32,
    pub uAssocReqSize: u32,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_NetworkManagement_Ndis"))]
impl ::core::marker::Copy for DOT11_INCOMING_ASSOC_REQUEST_RECEIVED_PARAMETERS {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_NetworkManagement_Ndis"))]
impl ::core::clone::Clone for DOT11_INCOMING_ASSOC_REQUEST_RECEIVED_PARAMETERS {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_NetworkManagement_Ndis"))]
unsafe impl ::windows::core::Abi for DOT11_INCOMING_ASSOC_REQUEST_RECEIVED_PARAMETERS {
    type Abi = Self;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_NetworkManagement_Ndis"))]
impl ::core::cmp::PartialEq for DOT11_INCOMING_ASSOC_REQUEST_RECEIVED_PARAMETERS {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<DOT11_INCOMING_ASSOC_REQUEST_RECEIVED_PARAMETERS>()) == 0 }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_NetworkManagement_Ndis"))]
impl ::core::cmp::Eq for DOT11_INCOMING_ASSOC_REQUEST_RECEIVED_PARAMETERS {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_NetworkManagement_Ndis"))]
impl ::core::default::Default for DOT11_INCOMING_ASSOC_REQUEST_RECEIVED_PARAMETERS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
pub const DOT11_INCOMING_ASSOC_REQUEST_RECEIVED_PARAMETERS_REVISION_1: u32 = 1u32;
#[repr(C)]
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
pub struct DOT11_INCOMING_ASSOC_STARTED_PARAMETERS {
    pub Header: super::Ndis::NDIS_OBJECT_HEADER,
    pub PeerMacAddr: [u8; 6],
}
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
impl ::core::marker::Copy for DOT11_INCOMING_ASSOC_STARTED_PARAMETERS {}
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
impl ::core::clone::Clone for DOT11_INCOMING_ASSOC_STARTED_PARAMETERS {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
unsafe impl ::windows::core::Abi for DOT11_INCOMING_ASSOC_STARTED_PARAMETERS {
    type Abi = Self;
}
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
impl ::core::cmp::PartialEq for DOT11_INCOMING_ASSOC_STARTED_PARAMETERS {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<DOT11_INCOMING_ASSOC_STARTED_PARAMETERS>()) == 0 }
    }
}
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
impl ::core::cmp::Eq for DOT11_INCOMING_ASSOC_STARTED_PARAMETERS {}
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
impl ::core::default::Default for DOT11_INCOMING_ASSOC_STARTED_PARAMETERS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
pub const DOT11_INCOMING_ASSOC_STARTED_PARAMETERS_REVISION_1: u32 = 1u32;
pub const DOT11_INVALID_CHANNEL_NUMBER: u32 = 0u32;
#[repr(C)]
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
pub struct DOT11_INVITATION_REQUEST_SEND_COMPLETE_PARAMETERS {
    pub Header: super::Ndis::NDIS_OBJECT_HEADER,
    pub PeerDeviceAddress: [u8; 6],
    pub ReceiverAddress: [u8; 6],
    pub DialogToken: u8,
    pub Status: i32,
    pub uIEsOffset: u32,
    pub uIEsLength: u32,
}
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
impl ::core::marker::Copy for DOT11_INVITATION_REQUEST_SEND_COMPLETE_PARAMETERS {}
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
impl ::core::clone::Clone for DOT11_INVITATION_REQUEST_SEND_COMPLETE_PARAMETERS {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
unsafe impl ::windows::core::Abi for DOT11_INVITATION_REQUEST_SEND_COMPLETE_PARAMETERS {
    type Abi = Self;
}
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
impl ::core::cmp::PartialEq for DOT11_INVITATION_REQUEST_SEND_COMPLETE_PARAMETERS {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<DOT11_INVITATION_REQUEST_SEND_COMPLETE_PARAMETERS>()) == 0 }
    }
}
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
impl ::core::cmp::Eq for DOT11_INVITATION_REQUEST_SEND_COMPLETE_PARAMETERS {}
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
impl ::core::default::Default for DOT11_INVITATION_REQUEST_SEND_COMPLETE_PARAMETERS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
pub const DOT11_INVITATION_REQUEST_SEND_COMPLETE_PARAMETERS_REVISION_1: u32 = 1u32;
#[repr(C)]
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
pub struct DOT11_INVITATION_RESPONSE_SEND_COMPLETE_PARAMETERS {
    pub Header: super::Ndis::NDIS_OBJECT_HEADER,
    pub ReceiverDeviceAddress: [u8; 6],
    pub DialogToken: u8,
    pub Status: i32,
    pub uIEsOffset: u32,
    pub uIEsLength: u32,
}
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
impl ::core::marker::Copy for DOT11_INVITATION_RESPONSE_SEND_COMPLETE_PARAMETERS {}
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
impl ::core::clone::Clone for DOT11_INVITATION_RESPONSE_SEND_COMPLETE_PARAMETERS {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
unsafe impl ::windows::core::Abi for DOT11_INVITATION_RESPONSE_SEND_COMPLETE_PARAMETERS {
    type Abi = Self;
}
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
impl ::core::cmp::PartialEq for DOT11_INVITATION_RESPONSE_SEND_COMPLETE_PARAMETERS {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<DOT11_INVITATION_RESPONSE_SEND_COMPLETE_PARAMETERS>()) == 0 }
    }
}
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
impl ::core::cmp::Eq for DOT11_INVITATION_RESPONSE_SEND_COMPLETE_PARAMETERS {}
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
impl ::core::default::Default for DOT11_INVITATION_RESPONSE_SEND_COMPLETE_PARAMETERS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
pub const DOT11_INVITATION_RESPONSE_SEND_COMPLETE_PARAMETERS_REVISION_1: u32 = 1u32;
#[repr(C)]
pub struct DOT11_IV48_COUNTER {
    pub uIV32Counter: u32,
    pub usIV16Counter: u16,
}
impl ::core::marker::Copy for DOT11_IV48_COUNTER {}
impl ::core::clone::Clone for DOT11_IV48_COUNTER {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for DOT11_IV48_COUNTER {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for DOT11_IV48_COUNTER {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<DOT11_IV48_COUNTER>()) == 0 }
    }
}
impl ::core::cmp::Eq for DOT11_IV48_COUNTER {}
impl ::core::default::Default for DOT11_IV48_COUNTER {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct DOT11_JOIN_REQUEST {
    pub uJoinFailureTimeout: u32,
    pub OperationalRateSet: DOT11_RATE_SET,
    pub uChCenterFrequency: u32,
    pub dot11BSSDescription: DOT11_BSS_DESCRIPTION,
}
impl ::core::marker::Copy for DOT11_JOIN_REQUEST {}
impl ::core::clone::Clone for DOT11_JOIN_REQUEST {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for DOT11_JOIN_REQUEST {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for DOT11_JOIN_REQUEST {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<DOT11_JOIN_REQUEST>()) == 0 }
    }
}
impl ::core::cmp::Eq for DOT11_JOIN_REQUEST {}
impl ::core::default::Default for DOT11_JOIN_REQUEST {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct DOT11_KEY_ALGO_BIP {
    pub ucIPN: [u8; 6],
    pub ulBIPKeyLength: u32,
    pub ucBIPKey: [u8; 1],
}
impl ::core::marker::Copy for DOT11_KEY_ALGO_BIP {}
impl ::core::clone::Clone for DOT11_KEY_ALGO_BIP {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for DOT11_KEY_ALGO_BIP {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for DOT11_KEY_ALGO_BIP {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<DOT11_KEY_ALGO_BIP>()) == 0 }
    }
}
impl ::core::cmp::Eq for DOT11_KEY_ALGO_BIP {}
impl ::core::default::Default for DOT11_KEY_ALGO_BIP {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct DOT11_KEY_ALGO_BIP_GMAC_256 {
    pub ucIPN: [u8; 6],
    pub ulBIPGmac256KeyLength: u32,
    pub ucBIPGmac256Key: [u8; 1],
}
impl ::core::marker::Copy for DOT11_KEY_ALGO_BIP_GMAC_256 {}
impl ::core::clone::Clone for DOT11_KEY_ALGO_BIP_GMAC_256 {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for DOT11_KEY_ALGO_BIP_GMAC_256 {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for DOT11_KEY_ALGO_BIP_GMAC_256 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<DOT11_KEY_ALGO_BIP_GMAC_256>()) == 0 }
    }
}
impl ::core::cmp::Eq for DOT11_KEY_ALGO_BIP_GMAC_256 {}
impl ::core::default::Default for DOT11_KEY_ALGO_BIP_GMAC_256 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct DOT11_KEY_ALGO_CCMP {
    pub ucIV48Counter: [u8; 6],
    pub ulCCMPKeyLength: u32,
    pub ucCCMPKey: [u8; 1],
}
impl ::core::marker::Copy for DOT11_KEY_ALGO_CCMP {}
impl ::core::clone::Clone for DOT11_KEY_ALGO_CCMP {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for DOT11_KEY_ALGO_CCMP {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for DOT11_KEY_ALGO_CCMP {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<DOT11_KEY_ALGO_CCMP>()) == 0 }
    }
}
impl ::core::cmp::Eq for DOT11_KEY_ALGO_CCMP {}
impl ::core::default::Default for DOT11_KEY_ALGO_CCMP {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct DOT11_KEY_ALGO_GCMP {
    pub ucIV48Counter: [u8; 6],
    pub ulGCMPKeyLength: u32,
    pub ucGCMPKey: [u8; 1],
}
impl ::core::marker::Copy for DOT11_KEY_ALGO_GCMP {}
impl ::core::clone::Clone for DOT11_KEY_ALGO_GCMP {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for DOT11_KEY_ALGO_GCMP {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for DOT11_KEY_ALGO_GCMP {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<DOT11_KEY_ALGO_GCMP>()) == 0 }
    }
}
impl ::core::cmp::Eq for DOT11_KEY_ALGO_GCMP {}
impl ::core::default::Default for DOT11_KEY_ALGO_GCMP {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct DOT11_KEY_ALGO_GCMP_256 {
    pub ucIV48Counter: [u8; 6],
    pub ulGCMP256KeyLength: u32,
    pub ucGCMP256Key: [u8; 1],
}
impl ::core::marker::Copy for DOT11_KEY_ALGO_GCMP_256 {}
impl ::core::clone::Clone for DOT11_KEY_ALGO_GCMP_256 {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for DOT11_KEY_ALGO_GCMP_256 {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for DOT11_KEY_ALGO_GCMP_256 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<DOT11_KEY_ALGO_GCMP_256>()) == 0 }
    }
}
impl ::core::cmp::Eq for DOT11_KEY_ALGO_GCMP_256 {}
impl ::core::default::Default for DOT11_KEY_ALGO_GCMP_256 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct DOT11_KEY_ALGO_TKIP_MIC {
    pub ucIV48Counter: [u8; 6],
    pub ulTKIPKeyLength: u32,
    pub ulMICKeyLength: u32,
    pub ucTKIPMICKeys: [u8; 1],
}
impl ::core::marker::Copy for DOT11_KEY_ALGO_TKIP_MIC {}
impl ::core::clone::Clone for DOT11_KEY_ALGO_TKIP_MIC {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for DOT11_KEY_ALGO_TKIP_MIC {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for DOT11_KEY_ALGO_TKIP_MIC {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<DOT11_KEY_ALGO_TKIP_MIC>()) == 0 }
    }
}
impl ::core::cmp::Eq for DOT11_KEY_ALGO_TKIP_MIC {}
impl ::core::default::Default for DOT11_KEY_ALGO_TKIP_MIC {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
pub type DOT11_KEY_DIRECTION = i32;
pub const dot11_key_direction_both: DOT11_KEY_DIRECTION = 1i32;
pub const dot11_key_direction_inbound: DOT11_KEY_DIRECTION = 2i32;
pub const dot11_key_direction_outbound: DOT11_KEY_DIRECTION = 3i32;
#[repr(C)]
pub struct DOT11_LINK_QUALITY_ENTRY {
    pub PeerMacAddr: [u8; 6],
    pub ucLinkQuality: u8,
}
impl ::core::marker::Copy for DOT11_LINK_QUALITY_ENTRY {}
impl ::core::clone::Clone for DOT11_LINK_QUALITY_ENTRY {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for DOT11_LINK_QUALITY_ENTRY {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for DOT11_LINK_QUALITY_ENTRY {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<DOT11_LINK_QUALITY_ENTRY>()) == 0 }
    }
}
impl ::core::cmp::Eq for DOT11_LINK_QUALITY_ENTRY {}
impl ::core::default::Default for DOT11_LINK_QUALITY_ENTRY {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
pub struct DOT11_LINK_QUALITY_PARAMETERS {
    pub Header: super::Ndis::NDIS_OBJECT_HEADER,
    pub uLinkQualityListSize: u32,
    pub uLinkQualityListOffset: u32,
}
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
impl ::core::marker::Copy for DOT11_LINK_QUALITY_PARAMETERS {}
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
impl ::core::clone::Clone for DOT11_LINK_QUALITY_PARAMETERS {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
unsafe impl ::windows::core::Abi for DOT11_LINK_QUALITY_PARAMETERS {
    type Abi = Self;
}
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
impl ::core::cmp::PartialEq for DOT11_LINK_QUALITY_PARAMETERS {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<DOT11_LINK_QUALITY_PARAMETERS>()) == 0 }
    }
}
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
impl ::core::cmp::Eq for DOT11_LINK_QUALITY_PARAMETERS {}
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
impl ::core::default::Default for DOT11_LINK_QUALITY_PARAMETERS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
pub const DOT11_LINK_QUALITY_PARAMETERS_REVISION_1: u32 = 1u32;
#[repr(C)]
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
pub struct DOT11_MAC_ADDRESS_LIST {
    pub Header: super::Ndis::NDIS_OBJECT_HEADER,
    pub uNumOfEntries: u32,
    pub uTotalNumOfEntries: u32,
    pub MacAddrs: [u8; 6],
}
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
impl ::core::marker::Copy for DOT11_MAC_ADDRESS_LIST {}
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
impl ::core::clone::Clone for DOT11_MAC_ADDRESS_LIST {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
unsafe impl ::windows::core::Abi for DOT11_MAC_ADDRESS_LIST {
    type Abi = Self;
}
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
impl ::core::cmp::PartialEq for DOT11_MAC_ADDRESS_LIST {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<DOT11_MAC_ADDRESS_LIST>()) == 0 }
    }
}
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
impl ::core::cmp::Eq for DOT11_MAC_ADDRESS_LIST {}
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
impl ::core::default::Default for DOT11_MAC_ADDRESS_LIST {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
pub const DOT11_MAC_ADDRESS_LIST_REVISION_1: u32 = 1u32;
#[repr(C)]
pub struct DOT11_MAC_FRAME_STATISTICS {
    pub ullTransmittedFrameCount: u64,
    pub ullReceivedFrameCount: u64,
    pub ullTransmittedFailureFrameCount: u64,
    pub ullReceivedFailureFrameCount: u64,
    pub ullWEPExcludedCount: u64,
    pub ullTKIPLocalMICFailures: u64,
    pub ullTKIPReplays: u64,
    pub ullTKIPICVErrorCount: u64,
    pub ullCCMPReplays: u64,
    pub ullCCMPDecryptErrors: u64,
    pub ullWEPUndecryptableCount: u64,
    pub ullWEPICVErrorCount: u64,
    pub ullDecryptSuccessCount: u64,
    pub ullDecryptFailureCount: u64,
}
impl ::core::marker::Copy for DOT11_MAC_FRAME_STATISTICS {}
impl ::core::clone::Clone for DOT11_MAC_FRAME_STATISTICS {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for DOT11_MAC_FRAME_STATISTICS {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for DOT11_MAC_FRAME_STATISTICS {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<DOT11_MAC_FRAME_STATISTICS>()) == 0 }
    }
}
impl ::core::cmp::Eq for DOT11_MAC_FRAME_STATISTICS {}
impl ::core::default::Default for DOT11_MAC_FRAME_STATISTICS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct DOT11_MAC_INFO {
    pub uReserved: u32,
    pub uNdisPortNumber: u32,
    pub MacAddr: [u8; 6],
}
impl ::core::marker::Copy for DOT11_MAC_INFO {}
impl ::core::clone::Clone for DOT11_MAC_INFO {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for DOT11_MAC_INFO {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for DOT11_MAC_INFO {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<DOT11_MAC_INFO>()) == 0 }
    }
}
impl ::core::cmp::Eq for DOT11_MAC_INFO {}
impl ::core::default::Default for DOT11_MAC_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
pub struct DOT11_MAC_PARAMETERS {
    pub Header: super::Ndis::NDIS_OBJECT_HEADER,
    pub uOpmodeMask: u32,
}
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
impl ::core::marker::Copy for DOT11_MAC_PARAMETERS {}
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
impl ::core::clone::Clone for DOT11_MAC_PARAMETERS {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
unsafe impl ::windows::core::Abi for DOT11_MAC_PARAMETERS {
    type Abi = Self;
}
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
impl ::core::cmp::PartialEq for DOT11_MAC_PARAMETERS {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<DOT11_MAC_PARAMETERS>()) == 0 }
    }
}
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
impl ::core::cmp::Eq for DOT11_MAC_PARAMETERS {}
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
impl ::core::default::Default for DOT11_MAC_PARAMETERS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
pub const DOT11_MAC_PARAMETERS_REVISION_1: u32 = 1u32;
#[repr(C)]
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
pub struct DOT11_MANUFACTURING_CALLBACK_PARAMETERS {
    pub Header: super::Ndis::NDIS_OBJECT_HEADER,
    pub dot11ManufacturingCallbackType: DOT11_MANUFACTURING_CALLBACK_TYPE,
    pub uStatus: u32,
    pub pvContext: *mut ::core::ffi::c_void,
}
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
impl ::core::marker::Copy for DOT11_MANUFACTURING_CALLBACK_PARAMETERS {}
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
impl ::core::clone::Clone for DOT11_MANUFACTURING_CALLBACK_PARAMETERS {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
unsafe impl ::windows::core::Abi for DOT11_MANUFACTURING_CALLBACK_PARAMETERS {
    type Abi = Self;
}
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
impl ::core::cmp::PartialEq for DOT11_MANUFACTURING_CALLBACK_PARAMETERS {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<DOT11_MANUFACTURING_CALLBACK_PARAMETERS>()) == 0 }
    }
}
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
impl ::core::cmp::Eq for DOT11_MANUFACTURING_CALLBACK_PARAMETERS {}
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
impl ::core::default::Default for DOT11_MANUFACTURING_CALLBACK_PARAMETERS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
pub const DOT11_MANUFACTURING_CALLBACK_REVISION_1: u32 = 1u32;
pub type DOT11_MANUFACTURING_CALLBACK_TYPE = i32;
pub const dot11_manufacturing_callback_unknown: DOT11_MANUFACTURING_CALLBACK_TYPE = 0i32;
pub const dot11_manufacturing_callback_self_test_complete: DOT11_MANUFACTURING_CALLBACK_TYPE = 1i32;
pub const dot11_manufacturing_callback_sleep_complete: DOT11_MANUFACTURING_CALLBACK_TYPE = 2i32;
pub const dot11_manufacturing_callback_IHV_start: DOT11_MANUFACTURING_CALLBACK_TYPE = -2147483648i32;
pub const dot11_manufacturing_callback_IHV_end: DOT11_MANUFACTURING_CALLBACK_TYPE = -1i32;
#[repr(C)]
pub struct DOT11_MANUFACTURING_FUNCTIONAL_TEST_QUERY_ADC {
    pub Dot11Band: DOT11_BAND,
    pub uChannel: u32,
    pub ADCPowerLevel: i32,
}
impl ::core::marker::Copy for DOT11_MANUFACTURING_FUNCTIONAL_TEST_QUERY_ADC {}
impl ::core::clone::Clone for DOT11_MANUFACTURING_FUNCTIONAL_TEST_QUERY_ADC {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for DOT11_MANUFACTURING_FUNCTIONAL_TEST_QUERY_ADC {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for DOT11_MANUFACTURING_FUNCTIONAL_TEST_QUERY_ADC {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<DOT11_MANUFACTURING_FUNCTIONAL_TEST_QUERY_ADC>()) == 0 }
    }
}
impl ::core::cmp::Eq for DOT11_MANUFACTURING_FUNCTIONAL_TEST_QUERY_ADC {}
impl ::core::default::Default for DOT11_MANUFACTURING_FUNCTIONAL_TEST_QUERY_ADC {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct DOT11_MANUFACTURING_FUNCTIONAL_TEST_RX {
    pub bEnabled: super::super::Foundation::BOOLEAN,
    pub Dot11Band: DOT11_BAND,
    pub uChannel: u32,
    pub PowerLevel: i32,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for DOT11_MANUFACTURING_FUNCTIONAL_TEST_RX {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for DOT11_MANUFACTURING_FUNCTIONAL_TEST_RX {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for DOT11_MANUFACTURING_FUNCTIONAL_TEST_RX {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for DOT11_MANUFACTURING_FUNCTIONAL_TEST_RX {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<DOT11_MANUFACTURING_FUNCTIONAL_TEST_RX>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for DOT11_MANUFACTURING_FUNCTIONAL_TEST_RX {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for DOT11_MANUFACTURING_FUNCTIONAL_TEST_RX {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct DOT11_MANUFACTURING_FUNCTIONAL_TEST_TX {
    pub bEnable: super::super::Foundation::BOOLEAN,
    pub bOpenLoop: super::super::Foundation::BOOLEAN,
    pub Dot11Band: DOT11_BAND,
    pub uChannel: u32,
    pub uSetPowerLevel: u32,
    pub ADCPowerLevel: i32,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for DOT11_MANUFACTURING_FUNCTIONAL_TEST_TX {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for DOT11_MANUFACTURING_FUNCTIONAL_TEST_TX {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for DOT11_MANUFACTURING_FUNCTIONAL_TEST_TX {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for DOT11_MANUFACTURING_FUNCTIONAL_TEST_TX {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<DOT11_MANUFACTURING_FUNCTIONAL_TEST_TX>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for DOT11_MANUFACTURING_FUNCTIONAL_TEST_TX {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for DOT11_MANUFACTURING_FUNCTIONAL_TEST_TX {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct DOT11_MANUFACTURING_SELF_TEST_QUERY_RESULTS {
    pub SelfTestType: DOT11_MANUFACTURING_SELF_TEST_TYPE,
    pub uTestID: u32,
    pub bResult: super::super::Foundation::BOOLEAN,
    pub uPinFailedBitMask: u32,
    pub pvContext: *mut ::core::ffi::c_void,
    pub uBytesWrittenOut: u32,
    pub ucBufferOut: [u8; 1],
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for DOT11_MANUFACTURING_SELF_TEST_QUERY_RESULTS {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for DOT11_MANUFACTURING_SELF_TEST_QUERY_RESULTS {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for DOT11_MANUFACTURING_SELF_TEST_QUERY_RESULTS {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for DOT11_MANUFACTURING_SELF_TEST_QUERY_RESULTS {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<DOT11_MANUFACTURING_SELF_TEST_QUERY_RESULTS>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for DOT11_MANUFACTURING_SELF_TEST_QUERY_RESULTS {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for DOT11_MANUFACTURING_SELF_TEST_QUERY_RESULTS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct DOT11_MANUFACTURING_SELF_TEST_SET_PARAMS {
    pub SelfTestType: DOT11_MANUFACTURING_SELF_TEST_TYPE,
    pub uTestID: u32,
    pub uPinBitMask: u32,
    pub pvContext: *mut ::core::ffi::c_void,
    pub uBufferLength: u32,
    pub ucBufferIn: [u8; 1],
}
impl ::core::marker::Copy for DOT11_MANUFACTURING_SELF_TEST_SET_PARAMS {}
impl ::core::clone::Clone for DOT11_MANUFACTURING_SELF_TEST_SET_PARAMS {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for DOT11_MANUFACTURING_SELF_TEST_SET_PARAMS {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for DOT11_MANUFACTURING_SELF_TEST_SET_PARAMS {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<DOT11_MANUFACTURING_SELF_TEST_SET_PARAMS>()) == 0 }
    }
}
impl ::core::cmp::Eq for DOT11_MANUFACTURING_SELF_TEST_SET_PARAMS {}
impl ::core::default::Default for DOT11_MANUFACTURING_SELF_TEST_SET_PARAMS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
pub type DOT11_MANUFACTURING_SELF_TEST_TYPE = i32;
pub const DOT11_MANUFACTURING_SELF_TEST_TYPE_INTERFACE: DOT11_MANUFACTURING_SELF_TEST_TYPE = 1i32;
pub const DOT11_MANUFACTURING_SELF_TEST_TYPE_RF_INTERFACE: DOT11_MANUFACTURING_SELF_TEST_TYPE = 2i32;
pub const DOT11_MANUFACTURING_SELF_TEST_TYPE_BT_COEXISTENCE: DOT11_MANUFACTURING_SELF_TEST_TYPE = 3i32;
#[repr(C)]
pub struct DOT11_MANUFACTURING_TEST {
    pub dot11ManufacturingTestType: DOT11_MANUFACTURING_TEST_TYPE,
    pub uBufferLength: u32,
    pub ucBuffer: [u8; 1],
}
impl ::core::marker::Copy for DOT11_MANUFACTURING_TEST {}
impl ::core::clone::Clone for DOT11_MANUFACTURING_TEST {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for DOT11_MANUFACTURING_TEST {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for DOT11_MANUFACTURING_TEST {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<DOT11_MANUFACTURING_TEST>()) == 0 }
    }
}
impl ::core::cmp::Eq for DOT11_MANUFACTURING_TEST {}
impl ::core::default::Default for DOT11_MANUFACTURING_TEST {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct DOT11_MANUFACTURING_TEST_QUERY_DATA {
    pub uKey: u32,
    pub uOffset: u32,
    pub uBufferLength: u32,
    pub uBytesRead: u32,
    pub ucBufferOut: [u8; 1],
}
impl ::core::marker::Copy for DOT11_MANUFACTURING_TEST_QUERY_DATA {}
impl ::core::clone::Clone for DOT11_MANUFACTURING_TEST_QUERY_DATA {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for DOT11_MANUFACTURING_TEST_QUERY_DATA {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for DOT11_MANUFACTURING_TEST_QUERY_DATA {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<DOT11_MANUFACTURING_TEST_QUERY_DATA>()) == 0 }
    }
}
impl ::core::cmp::Eq for DOT11_MANUFACTURING_TEST_QUERY_DATA {}
impl ::core::default::Default for DOT11_MANUFACTURING_TEST_QUERY_DATA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
pub const DOT11_MANUFACTURING_TEST_REVISION_1: u32 = 1u32;
#[repr(C)]
pub struct DOT11_MANUFACTURING_TEST_SET_DATA {
    pub uKey: u32,
    pub uOffset: u32,
    pub uBufferLength: u32,
    pub ucBufferIn: [u8; 1],
}
impl ::core::marker::Copy for DOT11_MANUFACTURING_TEST_SET_DATA {}
impl ::core::clone::Clone for DOT11_MANUFACTURING_TEST_SET_DATA {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for DOT11_MANUFACTURING_TEST_SET_DATA {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for DOT11_MANUFACTURING_TEST_SET_DATA {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<DOT11_MANUFACTURING_TEST_SET_DATA>()) == 0 }
    }
}
impl ::core::cmp::Eq for DOT11_MANUFACTURING_TEST_SET_DATA {}
impl ::core::default::Default for DOT11_MANUFACTURING_TEST_SET_DATA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct DOT11_MANUFACTURING_TEST_SLEEP {
    pub uSleepTime: u32,
    pub pvContext: *mut ::core::ffi::c_void,
}
impl ::core::marker::Copy for DOT11_MANUFACTURING_TEST_SLEEP {}
impl ::core::clone::Clone for DOT11_MANUFACTURING_TEST_SLEEP {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for DOT11_MANUFACTURING_TEST_SLEEP {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for DOT11_MANUFACTURING_TEST_SLEEP {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<DOT11_MANUFACTURING_TEST_SLEEP>()) == 0 }
    }
}
impl ::core::cmp::Eq for DOT11_MANUFACTURING_TEST_SLEEP {}
impl ::core::default::Default for DOT11_MANUFACTURING_TEST_SLEEP {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
pub type DOT11_MANUFACTURING_TEST_TYPE = i32;
pub const dot11_manufacturing_test_unknown: DOT11_MANUFACTURING_TEST_TYPE = 0i32;
pub const dot11_manufacturing_test_self_start: DOT11_MANUFACTURING_TEST_TYPE = 1i32;
pub const dot11_manufacturing_test_self_query_result: DOT11_MANUFACTURING_TEST_TYPE = 2i32;
pub const dot11_manufacturing_test_rx: DOT11_MANUFACTURING_TEST_TYPE = 3i32;
pub const dot11_manufacturing_test_tx: DOT11_MANUFACTURING_TEST_TYPE = 4i32;
pub const dot11_manufacturing_test_query_adc: DOT11_MANUFACTURING_TEST_TYPE = 5i32;
pub const dot11_manufacturing_test_set_data: DOT11_MANUFACTURING_TEST_TYPE = 6i32;
pub const dot11_manufacturing_test_query_data: DOT11_MANUFACTURING_TEST_TYPE = 7i32;
pub const dot11_manufacturing_test_sleep: DOT11_MANUFACTURING_TEST_TYPE = 8i32;
pub const dot11_manufacturing_test_awake: DOT11_MANUFACTURING_TEST_TYPE = 9i32;
pub const dot11_manufacturing_test_IHV_start: DOT11_MANUFACTURING_TEST_TYPE = -2147483648i32;
pub const dot11_manufacturing_test_IHV_end: DOT11_MANUFACTURING_TEST_TYPE = -1i32;
pub const DOT11_MAX_CHANNEL_HINTS: u32 = 4u32;
pub const DOT11_MAX_NUM_DEFAULT_KEY: u32 = 4u32;
pub const DOT11_MAX_NUM_DEFAULT_KEY_MFP: u32 = 6u32;
pub const DOT11_MAX_NUM_OF_FRAGMENTS: u32 = 16u32;
pub const DOT11_MAX_PDU_SIZE: u32 = 2346u32;
pub const DOT11_MAX_REQUESTED_SERVICE_INFORMATION_LENGTH: u32 = 255u32;
#[repr(C)]
pub struct DOT11_MD_CAPABILITY_ENTRY_LIST {
    pub uNumOfEntries: u32,
    pub uTotalNumOfEntries: u32,
    pub dot11MDCapabilityEntry: [DOT11_MULTI_DOMAIN_CAPABILITY_ENTRY; 1],
}
impl ::core::marker::Copy for DOT11_MD_CAPABILITY_ENTRY_LIST {}
impl ::core::clone::Clone for DOT11_MD_CAPABILITY_ENTRY_LIST {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for DOT11_MD_CAPABILITY_ENTRY_LIST {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for DOT11_MD_CAPABILITY_ENTRY_LIST {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<DOT11_MD_CAPABILITY_ENTRY_LIST>()) == 0 }
    }
}
impl ::core::cmp::Eq for DOT11_MD_CAPABILITY_ENTRY_LIST {}
impl ::core::default::Default for DOT11_MD_CAPABILITY_ENTRY_LIST {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
pub const DOT11_MIN_PDU_SIZE: u32 = 256u32;
#[repr(C)]
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
pub struct DOT11_MPDU_MAX_LENGTH_INDICATION {
    pub Header: super::Ndis::NDIS_OBJECT_HEADER,
    pub uPhyId: u32,
    pub uMPDUMaxLength: u32,
}
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
impl ::core::marker::Copy for DOT11_MPDU_MAX_LENGTH_INDICATION {}
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
impl ::core::clone::Clone for DOT11_MPDU_MAX_LENGTH_INDICATION {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
unsafe impl ::windows::core::Abi for DOT11_MPDU_MAX_LENGTH_INDICATION {
    type Abi = Self;
}
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
impl ::core::cmp::PartialEq for DOT11_MPDU_MAX_LENGTH_INDICATION {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<DOT11_MPDU_MAX_LENGTH_INDICATION>()) == 0 }
    }
}
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
impl ::core::cmp::Eq for DOT11_MPDU_MAX_LENGTH_INDICATION {}
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
impl ::core::default::Default for DOT11_MPDU_MAX_LENGTH_INDICATION {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
pub const DOT11_MPDU_MAX_LENGTH_INDICATION_REVISION_1: u32 = 1u32;
#[repr(C)]
pub struct DOT11_MULTI_DOMAIN_CAPABILITY_ENTRY {
    pub uMultiDomainCapabilityIndex: u32,
    pub uFirstChannelNumber: u32,
    pub uNumberOfChannels: u32,
    pub lMaximumTransmitPowerLevel: i32,
}
impl ::core::marker::Copy for DOT11_MULTI_DOMAIN_CAPABILITY_ENTRY {}
impl ::core::clone::Clone for DOT11_MULTI_DOMAIN_CAPABILITY_ENTRY {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for DOT11_MULTI_DOMAIN_CAPABILITY_ENTRY {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for DOT11_MULTI_DOMAIN_CAPABILITY_ENTRY {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<DOT11_MULTI_DOMAIN_CAPABILITY_ENTRY>()) == 0 }
    }
}
impl ::core::cmp::Eq for DOT11_MULTI_DOMAIN_CAPABILITY_ENTRY {}
impl ::core::default::Default for DOT11_MULTI_DOMAIN_CAPABILITY_ENTRY {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct DOT11_NETWORK {
    pub dot11Ssid: DOT11_SSID,
    pub dot11BssType: DOT11_BSS_TYPE,
}
impl ::core::marker::Copy for DOT11_NETWORK {}
impl ::core::clone::Clone for DOT11_NETWORK {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for DOT11_NETWORK {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for DOT11_NETWORK {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<DOT11_NETWORK>()) == 0 }
    }
}
impl ::core::cmp::Eq for DOT11_NETWORK {}
impl ::core::default::Default for DOT11_NETWORK {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct DOT11_NETWORK_LIST {
    pub dwNumberOfItems: u32,
    pub dwIndex: u32,
    pub Network: [DOT11_NETWORK; 1],
}
impl ::core::marker::Copy for DOT11_NETWORK_LIST {}
impl ::core::clone::Clone for DOT11_NETWORK_LIST {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for DOT11_NETWORK_LIST {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for DOT11_NETWORK_LIST {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<DOT11_NETWORK_LIST>()) == 0 }
    }
}
impl ::core::cmp::Eq for DOT11_NETWORK_LIST {}
impl ::core::default::Default for DOT11_NETWORK_LIST {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct DOT11_NIC_SPECIFIC_EXTENSION {
    pub uBufferLength: u32,
    pub uTotalBufferLength: u32,
    pub ucBuffer: [u8; 1],
}
impl ::core::marker::Copy for DOT11_NIC_SPECIFIC_EXTENSION {}
impl ::core::clone::Clone for DOT11_NIC_SPECIFIC_EXTENSION {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for DOT11_NIC_SPECIFIC_EXTENSION {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for DOT11_NIC_SPECIFIC_EXTENSION {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<DOT11_NIC_SPECIFIC_EXTENSION>()) == 0 }
    }
}
impl ::core::cmp::Eq for DOT11_NIC_SPECIFIC_EXTENSION {}
impl ::core::default::Default for DOT11_NIC_SPECIFIC_EXTENSION {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
pub const DOT11_NLO_FLAG_SCAN_AT_SYSTEM_RESUME: u32 = 4u32;
pub const DOT11_NLO_FLAG_SCAN_ON_AOAC_PLATFORM: u32 = 2u32;
pub const DOT11_NLO_FLAG_STOP_NLO_INDICATION: u32 = 1u32;
#[repr(C)]
pub struct DOT11_OFDM_PHY_ATTRIBUTES {
    pub uFrequencyBandsSupported: u32,
}
impl ::core::marker::Copy for DOT11_OFDM_PHY_ATTRIBUTES {}
impl ::core::clone::Clone for DOT11_OFDM_PHY_ATTRIBUTES {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for DOT11_OFDM_PHY_ATTRIBUTES {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for DOT11_OFDM_PHY_ATTRIBUTES {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<DOT11_OFDM_PHY_ATTRIBUTES>()) == 0 }
    }
}
impl ::core::cmp::Eq for DOT11_OFDM_PHY_ATTRIBUTES {}
impl ::core::default::Default for DOT11_OFDM_PHY_ATTRIBUTES {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct DOT11_OFFLOAD_CAPABILITY {
    pub uReserved: u32,
    pub uFlags: u32,
    pub uSupportedWEPAlgorithms: u32,
    pub uNumOfReplayWindows: u32,
    pub uMaxWEPKeyMappingLength: u32,
    pub uSupportedAuthAlgorithms: u32,
    pub uMaxAuthKeyMappingLength: u32,
}
impl ::core::marker::Copy for DOT11_OFFLOAD_CAPABILITY {}
impl ::core::clone::Clone for DOT11_OFFLOAD_CAPABILITY {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for DOT11_OFFLOAD_CAPABILITY {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for DOT11_OFFLOAD_CAPABILITY {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<DOT11_OFFLOAD_CAPABILITY>()) == 0 }
    }
}
impl ::core::cmp::Eq for DOT11_OFFLOAD_CAPABILITY {}
impl ::core::default::Default for DOT11_OFFLOAD_CAPABILITY {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct DOT11_OFFLOAD_NETWORK {
    pub Ssid: DOT11_SSID,
    pub UnicastCipher: DOT11_CIPHER_ALGORITHM,
    pub AuthAlgo: DOT11_AUTH_ALGORITHM,
    pub Dot11ChannelHints: [DOT11_CHANNEL_HINT; 4],
}
impl ::core::marker::Copy for DOT11_OFFLOAD_NETWORK {}
impl ::core::clone::Clone for DOT11_OFFLOAD_NETWORK {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for DOT11_OFFLOAD_NETWORK {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for DOT11_OFFLOAD_NETWORK {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<DOT11_OFFLOAD_NETWORK>()) == 0 }
    }
}
impl ::core::cmp::Eq for DOT11_OFFLOAD_NETWORK {}
impl ::core::default::Default for DOT11_OFFLOAD_NETWORK {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
pub struct DOT11_OFFLOAD_NETWORK_LIST_INFO {
    pub Header: super::Ndis::NDIS_OBJECT_HEADER,
    pub ulFlags: u32,
    pub FastScanPeriod: u32,
    pub FastScanIterations: u32,
    pub SlowScanPeriod: u32,
    pub uNumOfEntries: u32,
    pub offloadNetworkList: [DOT11_OFFLOAD_NETWORK; 1],
}
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
impl ::core::marker::Copy for DOT11_OFFLOAD_NETWORK_LIST_INFO {}
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
impl ::core::clone::Clone for DOT11_OFFLOAD_NETWORK_LIST_INFO {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
unsafe impl ::windows::core::Abi for DOT11_OFFLOAD_NETWORK_LIST_INFO {
    type Abi = Self;
}
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
impl ::core::cmp::PartialEq for DOT11_OFFLOAD_NETWORK_LIST_INFO {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<DOT11_OFFLOAD_NETWORK_LIST_INFO>()) == 0 }
    }
}
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
impl ::core::cmp::Eq for DOT11_OFFLOAD_NETWORK_LIST_INFO {}
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
impl ::core::default::Default for DOT11_OFFLOAD_NETWORK_LIST_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
pub const DOT11_OFFLOAD_NETWORK_LIST_REVISION_1: u32 = 1u32;
#[repr(C)]
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
pub struct DOT11_OFFLOAD_NETWORK_STATUS_PARAMETERS {
    pub Header: super::Ndis::NDIS_OBJECT_HEADER,
    pub Status: i32,
}
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
impl ::core::marker::Copy for DOT11_OFFLOAD_NETWORK_STATUS_PARAMETERS {}
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
impl ::core::clone::Clone for DOT11_OFFLOAD_NETWORK_STATUS_PARAMETERS {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
unsafe impl ::windows::core::Abi for DOT11_OFFLOAD_NETWORK_STATUS_PARAMETERS {
    type Abi = Self;
}
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
impl ::core::cmp::PartialEq for DOT11_OFFLOAD_NETWORK_STATUS_PARAMETERS {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<DOT11_OFFLOAD_NETWORK_STATUS_PARAMETERS>()) == 0 }
    }
}
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
impl ::core::cmp::Eq for DOT11_OFFLOAD_NETWORK_STATUS_PARAMETERS {}
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
impl ::core::default::Default for DOT11_OFFLOAD_NETWORK_STATUS_PARAMETERS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
pub const DOT11_OFFLOAD_NETWORK_STATUS_PARAMETERS_REVISION_1: u32 = 1u32;
pub type DOT11_OFFLOAD_TYPE = i32;
pub const dot11_offload_type_wep: DOT11_OFFLOAD_TYPE = 1i32;
pub const dot11_offload_type_auth: DOT11_OFFLOAD_TYPE = 2i32;
#[repr(C)]
pub struct DOT11_OI {
    pub OILength: u16,
    pub OI: [u8; 5],
}
impl ::core::marker::Copy for DOT11_OI {}
impl ::core::clone::Clone for DOT11_OI {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for DOT11_OI {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for DOT11_OI {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<DOT11_OI>()) == 0 }
    }
}
impl ::core::cmp::Eq for DOT11_OI {}
impl ::core::default::Default for DOT11_OI {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
pub const DOT11_OI_MAX_LENGTH: u32 = 5u32;
pub const DOT11_OI_MIN_LENGTH: u32 = 3u32;
pub const DOT11_OPERATION_MODE_AP: u32 = 2u32;
#[repr(C)]
pub struct DOT11_OPERATION_MODE_CAPABILITY {
    pub uReserved: u32,
    pub uMajorVersion: u32,
    pub uMinorVersion: u32,
    pub uNumOfTXBuffers: u32,
    pub uNumOfRXBuffers: u32,
    pub uOpModeCapability: u32,
}
impl ::core::marker::Copy for DOT11_OPERATION_MODE_CAPABILITY {}
impl ::core::clone::Clone for DOT11_OPERATION_MODE_CAPABILITY {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for DOT11_OPERATION_MODE_CAPABILITY {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for DOT11_OPERATION_MODE_CAPABILITY {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<DOT11_OPERATION_MODE_CAPABILITY>()) == 0 }
    }
}
impl ::core::cmp::Eq for DOT11_OPERATION_MODE_CAPABILITY {}
impl ::core::default::Default for DOT11_OPERATION_MODE_CAPABILITY {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
pub const DOT11_OPERATION_MODE_EXTENSIBLE_AP: u32 = 8u32;
pub const DOT11_OPERATION_MODE_EXTENSIBLE_STATION: u32 = 4u32;
pub const DOT11_OPERATION_MODE_MANUFACTURING: u32 = 1073741824u32;
pub const DOT11_OPERATION_MODE_NETWORK_MONITOR: u32 = 2147483648u32;
pub const DOT11_OPERATION_MODE_STATION: u32 = 1u32;
pub const DOT11_OPERATION_MODE_UNKNOWN: u32 = 0u32;
pub const DOT11_OPERATION_MODE_WFD_CLIENT: u32 = 64u32;
pub const DOT11_OPERATION_MODE_WFD_DEVICE: u32 = 16u32;
pub const DOT11_OPERATION_MODE_WFD_GROUP_OWNER: u32 = 32u32;
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct DOT11_OPTIONAL_CAPABILITY {
    pub uReserved: u32,
    pub bDot11PCF: super::super::Foundation::BOOLEAN,
    pub bDot11PCFMPDUTransferToPC: super::super::Foundation::BOOLEAN,
    pub bStrictlyOrderedServiceClass: super::super::Foundation::BOOLEAN,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for DOT11_OPTIONAL_CAPABILITY {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for DOT11_OPTIONAL_CAPABILITY {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for DOT11_OPTIONAL_CAPABILITY {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for DOT11_OPTIONAL_CAPABILITY {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<DOT11_OPTIONAL_CAPABILITY>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for DOT11_OPTIONAL_CAPABILITY {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for DOT11_OPTIONAL_CAPABILITY {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
pub const DOT11_PACKET_TYPE_ALL_MULTICAST_CTRL: u32 = 4096u32;
pub const DOT11_PACKET_TYPE_ALL_MULTICAST_DATA: u32 = 16384u32;
pub const DOT11_PACKET_TYPE_ALL_MULTICAST_MGMT: u32 = 8192u32;
pub const DOT11_PACKET_TYPE_BROADCAST_CTRL: u32 = 64u32;
pub const DOT11_PACKET_TYPE_BROADCAST_DATA: u32 = 256u32;
pub const DOT11_PACKET_TYPE_BROADCAST_MGMT: u32 = 128u32;
pub const DOT11_PACKET_TYPE_DIRECTED_CTRL: u32 = 1u32;
pub const DOT11_PACKET_TYPE_DIRECTED_DATA: u32 = 4u32;
pub const DOT11_PACKET_TYPE_DIRECTED_MGMT: u32 = 2u32;
pub const DOT11_PACKET_TYPE_MULTICAST_CTRL: u32 = 8u32;
pub const DOT11_PACKET_TYPE_MULTICAST_DATA: u32 = 32u32;
pub const DOT11_PACKET_TYPE_MULTICAST_MGMT: u32 = 16u32;
pub const DOT11_PACKET_TYPE_PROMISCUOUS_CTRL: u32 = 512u32;
pub const DOT11_PACKET_TYPE_PROMISCUOUS_DATA: u32 = 2048u32;
pub const DOT11_PACKET_TYPE_PROMISCUOUS_MGMT: u32 = 1024u32;
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct DOT11_PEER_INFO {
    pub MacAddress: [u8; 6],
    pub usCapabilityInformation: u16,
    pub AuthAlgo: DOT11_AUTH_ALGORITHM,
    pub UnicastCipherAlgo: DOT11_CIPHER_ALGORITHM,
    pub MulticastCipherAlgo: DOT11_CIPHER_ALGORITHM,
    pub bWpsEnabled: super::super::Foundation::BOOLEAN,
    pub usListenInterval: u16,
    pub ucSupportedRates: [u8; 255],
    pub usAssociationID: u16,
    pub AssociationState: DOT11_ASSOCIATION_STATE,
    pub PowerMode: DOT11_POWER_MODE,
    pub liAssociationUpTime: i64,
    pub Statistics: DOT11_PEER_STATISTICS,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for DOT11_PEER_INFO {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for DOT11_PEER_INFO {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for DOT11_PEER_INFO {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for DOT11_PEER_INFO {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<DOT11_PEER_INFO>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for DOT11_PEER_INFO {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for DOT11_PEER_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_NetworkManagement_Ndis"))]
pub struct DOT11_PEER_INFO_LIST {
    pub Header: super::Ndis::NDIS_OBJECT_HEADER,
    pub uNumOfEntries: u32,
    pub uTotalNumOfEntries: u32,
    pub PeerInfo: [DOT11_PEER_INFO; 1],
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_NetworkManagement_Ndis"))]
impl ::core::marker::Copy for DOT11_PEER_INFO_LIST {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_NetworkManagement_Ndis"))]
impl ::core::clone::Clone for DOT11_PEER_INFO_LIST {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_NetworkManagement_Ndis"))]
unsafe impl ::windows::core::Abi for DOT11_PEER_INFO_LIST {
    type Abi = Self;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_NetworkManagement_Ndis"))]
impl ::core::cmp::PartialEq for DOT11_PEER_INFO_LIST {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<DOT11_PEER_INFO_LIST>()) == 0 }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_NetworkManagement_Ndis"))]
impl ::core::cmp::Eq for DOT11_PEER_INFO_LIST {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_NetworkManagement_Ndis"))]
impl ::core::default::Default for DOT11_PEER_INFO_LIST {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
pub const DOT11_PEER_INFO_LIST_REVISION_1: u32 = 1u32;
#[repr(C)]
pub struct DOT11_PEER_STATISTICS {
    pub ullDecryptSuccessCount: u64,
    pub ullDecryptFailureCount: u64,
    pub ullTxPacketSuccessCount: u64,
    pub ullTxPacketFailureCount: u64,
    pub ullRxPacketSuccessCount: u64,
    pub ullRxPacketFailureCount: u64,
}
impl ::core::marker::Copy for DOT11_PEER_STATISTICS {}
impl ::core::clone::Clone for DOT11_PEER_STATISTICS {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for DOT11_PEER_STATISTICS {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for DOT11_PEER_STATISTICS {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<DOT11_PEER_STATISTICS>()) == 0 }
    }
}
impl ::core::cmp::Eq for DOT11_PEER_STATISTICS {}
impl ::core::default::Default for DOT11_PEER_STATISTICS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct DOT11_PER_MSDU_COUNTERS {
    pub uTransmittedFragmentCount: u32,
    pub uRetryCount: u32,
    pub uRTSSuccessCount: u32,
    pub uRTSFailureCount: u32,
    pub uACKFailureCount: u32,
}
impl ::core::marker::Copy for DOT11_PER_MSDU_COUNTERS {}
impl ::core::clone::Clone for DOT11_PER_MSDU_COUNTERS {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for DOT11_PER_MSDU_COUNTERS {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for DOT11_PER_MSDU_COUNTERS {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<DOT11_PER_MSDU_COUNTERS>()) == 0 }
    }
}
impl ::core::cmp::Eq for DOT11_PER_MSDU_COUNTERS {}
impl ::core::default::Default for DOT11_PER_MSDU_COUNTERS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_NetworkManagement_Ndis"))]
pub struct DOT11_PHY_ATTRIBUTES {
    pub Header: super::Ndis::NDIS_OBJECT_HEADER,
    pub PhyType: DOT11_PHY_TYPE,
    pub bHardwarePhyState: super::super::Foundation::BOOLEAN,
    pub bSoftwarePhyState: super::super::Foundation::BOOLEAN,
    pub bCFPollable: super::super::Foundation::BOOLEAN,
    pub uMPDUMaxLength: u32,
    pub TempType: DOT11_TEMP_TYPE,
    pub DiversitySupport: DOT11_DIVERSITY_SUPPORT,
    pub PhySpecificAttributes: DOT11_PHY_ATTRIBUTES_0,
    pub uNumberSupportedPowerLevels: u32,
    pub TxPowerLevels: [u32; 8],
    pub uNumDataRateMappingEntries: u32,
    pub DataRateMappingEntries: [DOT11_DATA_RATE_MAPPING_ENTRY; 126],
    pub SupportedDataRatesValue: DOT11_SUPPORTED_DATA_RATES_VALUE_V2,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_NetworkManagement_Ndis"))]
impl ::core::marker::Copy for DOT11_PHY_ATTRIBUTES {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_NetworkManagement_Ndis"))]
impl ::core::clone::Clone for DOT11_PHY_ATTRIBUTES {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_NetworkManagement_Ndis"))]
unsafe impl ::windows::core::Abi for DOT11_PHY_ATTRIBUTES {
    type Abi = Self;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_NetworkManagement_Ndis"))]
impl ::core::cmp::PartialEq for DOT11_PHY_ATTRIBUTES {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<DOT11_PHY_ATTRIBUTES>()) == 0 }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_NetworkManagement_Ndis"))]
impl ::core::cmp::Eq for DOT11_PHY_ATTRIBUTES {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_NetworkManagement_Ndis"))]
impl ::core::default::Default for DOT11_PHY_ATTRIBUTES {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_NetworkManagement_Ndis"))]
pub union DOT11_PHY_ATTRIBUTES_0 {
    pub HRDSSSAttributes: DOT11_HRDSSS_PHY_ATTRIBUTES,
    pub OFDMAttributes: DOT11_OFDM_PHY_ATTRIBUTES,
    pub ERPAttributes: DOT11_ERP_PHY_ATTRIBUTES,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_NetworkManagement_Ndis"))]
impl ::core::marker::Copy for DOT11_PHY_ATTRIBUTES_0 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_NetworkManagement_Ndis"))]
impl ::core::clone::Clone for DOT11_PHY_ATTRIBUTES_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_NetworkManagement_Ndis"))]
unsafe impl ::windows::core::Abi for DOT11_PHY_ATTRIBUTES_0 {
    type Abi = Self;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_NetworkManagement_Ndis"))]
impl ::core::cmp::PartialEq for DOT11_PHY_ATTRIBUTES_0 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<DOT11_PHY_ATTRIBUTES_0>()) == 0 }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_NetworkManagement_Ndis"))]
impl ::core::cmp::Eq for DOT11_PHY_ATTRIBUTES_0 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_NetworkManagement_Ndis"))]
impl ::core::default::Default for DOT11_PHY_ATTRIBUTES_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
pub const DOT11_PHY_ATTRIBUTES_REVISION_1: u32 = 1u32;
#[repr(C)]
pub struct DOT11_PHY_FRAME_STATISTICS {
    pub ullTransmittedFrameCount: u64,
    pub ullMulticastTransmittedFrameCount: u64,
    pub ullFailedCount: u64,
    pub ullRetryCount: u64,
    pub ullMultipleRetryCount: u64,
    pub ullMaxTXLifetimeExceededCount: u64,
    pub ullTransmittedFragmentCount: u64,
    pub ullRTSSuccessCount: u64,
    pub ullRTSFailureCount: u64,
    pub ullACKFailureCount: u64,
    pub ullReceivedFrameCount: u64,
    pub ullMulticastReceivedFrameCount: u64,
    pub ullPromiscuousReceivedFrameCount: u64,
    pub ullMaxRXLifetimeExceededCount: u64,
    pub ullFrameDuplicateCount: u64,
    pub ullReceivedFragmentCount: u64,
    pub ullPromiscuousReceivedFragmentCount: u64,
    pub ullFCSErrorCount: u64,
}
impl ::core::marker::Copy for DOT11_PHY_FRAME_STATISTICS {}
impl ::core::clone::Clone for DOT11_PHY_FRAME_STATISTICS {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for DOT11_PHY_FRAME_STATISTICS {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for DOT11_PHY_FRAME_STATISTICS {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<DOT11_PHY_FRAME_STATISTICS>()) == 0 }
    }
}
impl ::core::cmp::Eq for DOT11_PHY_FRAME_STATISTICS {}
impl ::core::default::Default for DOT11_PHY_FRAME_STATISTICS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
pub struct DOT11_PHY_FREQUENCY_ADOPTED_PARAMETERS {
    pub Header: super::Ndis::NDIS_OBJECT_HEADER,
    pub ulPhyId: u32,
    pub Anonymous: DOT11_PHY_FREQUENCY_ADOPTED_PARAMETERS_0,
}
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
impl ::core::marker::Copy for DOT11_PHY_FREQUENCY_ADOPTED_PARAMETERS {}
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
impl ::core::clone::Clone for DOT11_PHY_FREQUENCY_ADOPTED_PARAMETERS {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
unsafe impl ::windows::core::Abi for DOT11_PHY_FREQUENCY_ADOPTED_PARAMETERS {
    type Abi = Self;
}
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
impl ::core::cmp::PartialEq for DOT11_PHY_FREQUENCY_ADOPTED_PARAMETERS {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<DOT11_PHY_FREQUENCY_ADOPTED_PARAMETERS>()) == 0 }
    }
}
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
impl ::core::cmp::Eq for DOT11_PHY_FREQUENCY_ADOPTED_PARAMETERS {}
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
impl ::core::default::Default for DOT11_PHY_FREQUENCY_ADOPTED_PARAMETERS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
pub union DOT11_PHY_FREQUENCY_ADOPTED_PARAMETERS_0 {
    pub ulChannel: u32,
    pub ulFrequency: u32,
}
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
impl ::core::marker::Copy for DOT11_PHY_FREQUENCY_ADOPTED_PARAMETERS_0 {}
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
impl ::core::clone::Clone for DOT11_PHY_FREQUENCY_ADOPTED_PARAMETERS_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
unsafe impl ::windows::core::Abi for DOT11_PHY_FREQUENCY_ADOPTED_PARAMETERS_0 {
    type Abi = Self;
}
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
impl ::core::cmp::PartialEq for DOT11_PHY_FREQUENCY_ADOPTED_PARAMETERS_0 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<DOT11_PHY_FREQUENCY_ADOPTED_PARAMETERS_0>()) == 0 }
    }
}
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
impl ::core::cmp::Eq for DOT11_PHY_FREQUENCY_ADOPTED_PARAMETERS_0 {}
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
impl ::core::default::Default for DOT11_PHY_FREQUENCY_ADOPTED_PARAMETERS_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
pub const DOT11_PHY_FREQUENCY_ADOPTED_PARAMETERS_REVISION_1: u32 = 1u32;
#[repr(C)]
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
pub struct DOT11_PHY_ID_LIST {
    pub Header: super::Ndis::NDIS_OBJECT_HEADER,
    pub uNumOfEntries: u32,
    pub uTotalNumOfEntries: u32,
    pub dot11PhyId: [u32; 1],
}
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
impl ::core::marker::Copy for DOT11_PHY_ID_LIST {}
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
impl ::core::clone::Clone for DOT11_PHY_ID_LIST {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
unsafe impl ::windows::core::Abi for DOT11_PHY_ID_LIST {
    type Abi = Self;
}
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
impl ::core::cmp::PartialEq for DOT11_PHY_ID_LIST {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<DOT11_PHY_ID_LIST>()) == 0 }
    }
}
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
impl ::core::cmp::Eq for DOT11_PHY_ID_LIST {}
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
impl ::core::default::Default for DOT11_PHY_ID_LIST {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
pub const DOT11_PHY_ID_LIST_REVISION_1: u32 = 1u32;
#[repr(C)]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_NetworkManagement_Ndis"))]
pub struct DOT11_PHY_STATE_PARAMETERS {
    pub Header: super::Ndis::NDIS_OBJECT_HEADER,
    pub uPhyId: u32,
    pub bHardwarePhyState: super::super::Foundation::BOOLEAN,
    pub bSoftwarePhyState: super::super::Foundation::BOOLEAN,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_NetworkManagement_Ndis"))]
impl ::core::marker::Copy for DOT11_PHY_STATE_PARAMETERS {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_NetworkManagement_Ndis"))]
impl ::core::clone::Clone for DOT11_PHY_STATE_PARAMETERS {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_NetworkManagement_Ndis"))]
unsafe impl ::windows::core::Abi for DOT11_PHY_STATE_PARAMETERS {
    type Abi = Self;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_NetworkManagement_Ndis"))]
impl ::core::cmp::PartialEq for DOT11_PHY_STATE_PARAMETERS {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<DOT11_PHY_STATE_PARAMETERS>()) == 0 }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_NetworkManagement_Ndis"))]
impl ::core::cmp::Eq for DOT11_PHY_STATE_PARAMETERS {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_NetworkManagement_Ndis"))]
impl ::core::default::Default for DOT11_PHY_STATE_PARAMETERS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
pub const DOT11_PHY_STATE_PARAMETERS_REVISION_1: u32 = 1u32;
pub type DOT11_PHY_TYPE = i32;
pub const dot11_phy_type_unknown: DOT11_PHY_TYPE = 0i32;
pub const dot11_phy_type_any: DOT11_PHY_TYPE = 0i32;
pub const dot11_phy_type_fhss: DOT11_PHY_TYPE = 1i32;
pub const dot11_phy_type_dsss: DOT11_PHY_TYPE = 2i32;
pub const dot11_phy_type_irbaseband: DOT11_PHY_TYPE = 3i32;
pub const dot11_phy_type_ofdm: DOT11_PHY_TYPE = 4i32;
pub const dot11_phy_type_hrdsss: DOT11_PHY_TYPE = 5i32;
pub const dot11_phy_type_erp: DOT11_PHY_TYPE = 6i32;
pub const dot11_phy_type_ht: DOT11_PHY_TYPE = 7i32;
pub const dot11_phy_type_vht: DOT11_PHY_TYPE = 8i32;
pub const dot11_phy_type_dmg: DOT11_PHY_TYPE = 9i32;
pub const dot11_phy_type_he: DOT11_PHY_TYPE = 10i32;
pub const dot11_phy_type_IHV_start: DOT11_PHY_TYPE = -2147483648i32;
pub const dot11_phy_type_IHV_end: DOT11_PHY_TYPE = -1i32;
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct DOT11_PHY_TYPE_INFO {
    pub dot11PhyType: DOT11_PHY_TYPE,
    pub bUseParameters: super::super::Foundation::BOOLEAN,
    pub uProbeDelay: u32,
    pub uMinChannelTime: u32,
    pub uMaxChannelTime: u32,
    pub ChDescriptionType: CH_DESCRIPTION_TYPE,
    pub uChannelListSize: u32,
    pub ucChannelListBuffer: [u8; 1],
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for DOT11_PHY_TYPE_INFO {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for DOT11_PHY_TYPE_INFO {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for DOT11_PHY_TYPE_INFO {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for DOT11_PHY_TYPE_INFO {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<DOT11_PHY_TYPE_INFO>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for DOT11_PHY_TYPE_INFO {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for DOT11_PHY_TYPE_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
pub struct DOT11_PHY_TYPE_LIST {
    pub Header: super::Ndis::NDIS_OBJECT_HEADER,
    pub uNumOfEntries: u32,
    pub uTotalNumOfEntries: u32,
    pub dot11PhyType: [DOT11_PHY_TYPE; 1],
}
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
impl ::core::marker::Copy for DOT11_PHY_TYPE_LIST {}
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
impl ::core::clone::Clone for DOT11_PHY_TYPE_LIST {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
unsafe impl ::windows::core::Abi for DOT11_PHY_TYPE_LIST {
    type Abi = Self;
}
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
impl ::core::cmp::PartialEq for DOT11_PHY_TYPE_LIST {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<DOT11_PHY_TYPE_LIST>()) == 0 }
    }
}
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
impl ::core::cmp::Eq for DOT11_PHY_TYPE_LIST {}
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
impl ::core::default::Default for DOT11_PHY_TYPE_LIST {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
pub const DOT11_PHY_TYPE_LIST_REVISION_1: u32 = 1u32;
#[repr(C)]
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
pub struct DOT11_PMKID_CANDIDATE_LIST_PARAMETERS {
    pub Header: super::Ndis::NDIS_OBJECT_HEADER,
    pub uCandidateListSize: u32,
    pub uCandidateListOffset: u32,
}
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
impl ::core::marker::Copy for DOT11_PMKID_CANDIDATE_LIST_PARAMETERS {}
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
impl ::core::clone::Clone for DOT11_PMKID_CANDIDATE_LIST_PARAMETERS {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
unsafe impl ::windows::core::Abi for DOT11_PMKID_CANDIDATE_LIST_PARAMETERS {
    type Abi = Self;
}
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
impl ::core::cmp::PartialEq for DOT11_PMKID_CANDIDATE_LIST_PARAMETERS {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<DOT11_PMKID_CANDIDATE_LIST_PARAMETERS>()) == 0 }
    }
}
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
impl ::core::cmp::Eq for DOT11_PMKID_CANDIDATE_LIST_PARAMETERS {}
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
impl ::core::default::Default for DOT11_PMKID_CANDIDATE_LIST_PARAMETERS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
pub const DOT11_PMKID_CANDIDATE_LIST_PARAMETERS_REVISION_1: u32 = 1u32;
#[repr(C)]
pub struct DOT11_PMKID_ENTRY {
    pub BSSID: [u8; 6],
    pub PMKID: [u8; 16],
    pub uFlags: u32,
}
impl ::core::marker::Copy for DOT11_PMKID_ENTRY {}
impl ::core::clone::Clone for DOT11_PMKID_ENTRY {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for DOT11_PMKID_ENTRY {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for DOT11_PMKID_ENTRY {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<DOT11_PMKID_ENTRY>()) == 0 }
    }
}
impl ::core::cmp::Eq for DOT11_PMKID_ENTRY {}
impl ::core::default::Default for DOT11_PMKID_ENTRY {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
pub struct DOT11_PMKID_LIST {
    pub Header: super::Ndis::NDIS_OBJECT_HEADER,
    pub uNumOfEntries: u32,
    pub uTotalNumOfEntries: u32,
    pub PMKIDs: [DOT11_PMKID_ENTRY; 1],
}
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
impl ::core::marker::Copy for DOT11_PMKID_LIST {}
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
impl ::core::clone::Clone for DOT11_PMKID_LIST {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
unsafe impl ::windows::core::Abi for DOT11_PMKID_LIST {
    type Abi = Self;
}
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
impl ::core::cmp::PartialEq for DOT11_PMKID_LIST {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<DOT11_PMKID_LIST>()) == 0 }
    }
}
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
impl ::core::cmp::Eq for DOT11_PMKID_LIST {}
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
impl ::core::default::Default for DOT11_PMKID_LIST {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
pub const DOT11_PMKID_LIST_REVISION_1: u32 = 1u32;
#[repr(C)]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_NetworkManagement_Ndis"))]
pub struct DOT11_PORT_STATE_NOTIFICATION {
    pub Header: super::Ndis::NDIS_OBJECT_HEADER,
    pub PeerMac: [u8; 6],
    pub bOpen: super::super::Foundation::BOOLEAN,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_NetworkManagement_Ndis"))]
impl ::core::marker::Copy for DOT11_PORT_STATE_NOTIFICATION {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_NetworkManagement_Ndis"))]
impl ::core::clone::Clone for DOT11_PORT_STATE_NOTIFICATION {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_NetworkManagement_Ndis"))]
unsafe impl ::windows::core::Abi for DOT11_PORT_STATE_NOTIFICATION {
    type Abi = Self;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_NetworkManagement_Ndis"))]
impl ::core::cmp::PartialEq for DOT11_PORT_STATE_NOTIFICATION {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<DOT11_PORT_STATE_NOTIFICATION>()) == 0 }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_NetworkManagement_Ndis"))]
impl ::core::cmp::Eq for DOT11_PORT_STATE_NOTIFICATION {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_NetworkManagement_Ndis"))]
impl ::core::default::Default for DOT11_PORT_STATE_NOTIFICATION {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
pub const DOT11_PORT_STATE_NOTIFICATION_REVISION_1: u32 = 1u32;
#[repr(C)]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_NetworkManagement_Ndis"))]
pub struct DOT11_POWER_MGMT_AUTO_MODE_ENABLED_INFO {
    pub Header: super::Ndis::NDIS_OBJECT_HEADER,
    pub bEnabled: super::super::Foundation::BOOLEAN,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_NetworkManagement_Ndis"))]
impl ::core::marker::Copy for DOT11_POWER_MGMT_AUTO_MODE_ENABLED_INFO {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_NetworkManagement_Ndis"))]
impl ::core::clone::Clone for DOT11_POWER_MGMT_AUTO_MODE_ENABLED_INFO {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_NetworkManagement_Ndis"))]
unsafe impl ::windows::core::Abi for DOT11_POWER_MGMT_AUTO_MODE_ENABLED_INFO {
    type Abi = Self;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_NetworkManagement_Ndis"))]
impl ::core::cmp::PartialEq for DOT11_POWER_MGMT_AUTO_MODE_ENABLED_INFO {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<DOT11_POWER_MGMT_AUTO_MODE_ENABLED_INFO>()) == 0 }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_NetworkManagement_Ndis"))]
impl ::core::cmp::Eq for DOT11_POWER_MGMT_AUTO_MODE_ENABLED_INFO {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_NetworkManagement_Ndis"))]
impl ::core::default::Default for DOT11_POWER_MGMT_AUTO_MODE_ENABLED_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
pub const DOT11_POWER_MGMT_AUTO_MODE_ENABLED_REVISION_1: u32 = 1u32;
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct DOT11_POWER_MGMT_MODE {
    pub dot11PowerMode: DOT11_POWER_MODE,
    pub uPowerSaveLevel: u32,
    pub usListenInterval: u16,
    pub usAID: u16,
    pub bReceiveDTIMs: super::super::Foundation::BOOLEAN,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for DOT11_POWER_MGMT_MODE {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for DOT11_POWER_MGMT_MODE {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for DOT11_POWER_MGMT_MODE {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for DOT11_POWER_MGMT_MODE {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<DOT11_POWER_MGMT_MODE>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for DOT11_POWER_MGMT_MODE {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for DOT11_POWER_MGMT_MODE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
pub struct DOT11_POWER_MGMT_MODE_STATUS_INFO {
    pub Header: super::Ndis::NDIS_OBJECT_HEADER,
    pub PowerSaveMode: DOT11_POWER_MODE,
    pub uPowerSaveLevel: u32,
    pub Reason: DOT11_POWER_MODE_REASON,
}
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
impl ::core::marker::Copy for DOT11_POWER_MGMT_MODE_STATUS_INFO {}
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
impl ::core::clone::Clone for DOT11_POWER_MGMT_MODE_STATUS_INFO {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
unsafe impl ::windows::core::Abi for DOT11_POWER_MGMT_MODE_STATUS_INFO {
    type Abi = Self;
}
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
impl ::core::cmp::PartialEq for DOT11_POWER_MGMT_MODE_STATUS_INFO {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<DOT11_POWER_MGMT_MODE_STATUS_INFO>()) == 0 }
    }
}
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
impl ::core::cmp::Eq for DOT11_POWER_MGMT_MODE_STATUS_INFO {}
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
impl ::core::default::Default for DOT11_POWER_MGMT_MODE_STATUS_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
pub const DOT11_POWER_MGMT_MODE_STATUS_INFO_REVISION_1: u32 = 1u32;
pub type DOT11_POWER_MODE = i32;
pub const dot11_power_mode_unknown: DOT11_POWER_MODE = 0i32;
pub const dot11_power_mode_active: DOT11_POWER_MODE = 1i32;
pub const dot11_power_mode_powersave: DOT11_POWER_MODE = 2i32;
pub type DOT11_POWER_MODE_REASON = i32;
pub const dot11_power_mode_reason_no_change: DOT11_POWER_MODE_REASON = 0i32;
pub const dot11_power_mode_reason_noncompliant_AP: DOT11_POWER_MODE_REASON = 1i32;
pub const dot11_power_mode_reason_legacy_WFD_device: DOT11_POWER_MODE_REASON = 2i32;
pub const dot11_power_mode_reason_compliant_AP: DOT11_POWER_MODE_REASON = 3i32;
pub const dot11_power_mode_reason_compliant_WFD_device: DOT11_POWER_MODE_REASON = 4i32;
pub const dot11_power_mode_reason_others: DOT11_POWER_MODE_REASON = 5i32;
pub const DOT11_POWER_SAVE_LEVEL_FAST_PSP: u32 = 2u32;
pub const DOT11_POWER_SAVE_LEVEL_MAX_PSP: u32 = 1u32;
pub const DOT11_POWER_SAVING_FAST_PSP: u32 = 8u32;
pub const DOT11_POWER_SAVING_MAXIMUM_LEVEL: u32 = 24u32;
pub const DOT11_POWER_SAVING_MAX_PSP: u32 = 16u32;
pub const DOT11_POWER_SAVING_NO_POWER_SAVING: u32 = 0u32;
pub const DOT11_PRIORITY_CONTENTION: u32 = 0u32;
pub const DOT11_PRIORITY_CONTENTION_FREE: u32 = 1u32;
#[repr(C)]
pub struct DOT11_PRIVACY_EXEMPTION {
    pub usEtherType: u16,
    pub usExemptionActionType: u16,
    pub usExemptionPacketType: u16,
}
impl ::core::marker::Copy for DOT11_PRIVACY_EXEMPTION {}
impl ::core::clone::Clone for DOT11_PRIVACY_EXEMPTION {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for DOT11_PRIVACY_EXEMPTION {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for DOT11_PRIVACY_EXEMPTION {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<DOT11_PRIVACY_EXEMPTION>()) == 0 }
    }
}
impl ::core::cmp::Eq for DOT11_PRIVACY_EXEMPTION {}
impl ::core::default::Default for DOT11_PRIVACY_EXEMPTION {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
pub struct DOT11_PRIVACY_EXEMPTION_LIST {
    pub Header: super::Ndis::NDIS_OBJECT_HEADER,
    pub uNumOfEntries: u32,
    pub uTotalNumOfEntries: u32,
    pub PrivacyExemptionEntries: [DOT11_PRIVACY_EXEMPTION; 1],
}
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
impl ::core::marker::Copy for DOT11_PRIVACY_EXEMPTION_LIST {}
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
impl ::core::clone::Clone for DOT11_PRIVACY_EXEMPTION_LIST {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
unsafe impl ::windows::core::Abi for DOT11_PRIVACY_EXEMPTION_LIST {
    type Abi = Self;
}
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
impl ::core::cmp::PartialEq for DOT11_PRIVACY_EXEMPTION_LIST {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<DOT11_PRIVACY_EXEMPTION_LIST>()) == 0 }
    }
}
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
impl ::core::cmp::Eq for DOT11_PRIVACY_EXEMPTION_LIST {}
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
impl ::core::default::Default for DOT11_PRIVACY_EXEMPTION_LIST {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
pub const DOT11_PRIVACY_EXEMPTION_LIST_REVISION_1: u32 = 1u32;
#[repr(C)]
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
pub struct DOT11_PROVISION_DISCOVERY_REQUEST_SEND_COMPLETE_PARAMETERS {
    pub Header: super::Ndis::NDIS_OBJECT_HEADER,
    pub PeerDeviceAddress: [u8; 6],
    pub ReceiverAddress: [u8; 6],
    pub DialogToken: u8,
    pub Status: i32,
    pub uIEsOffset: u32,
    pub uIEsLength: u32,
}
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
impl ::core::marker::Copy for DOT11_PROVISION_DISCOVERY_REQUEST_SEND_COMPLETE_PARAMETERS {}
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
impl ::core::clone::Clone for DOT11_PROVISION_DISCOVERY_REQUEST_SEND_COMPLETE_PARAMETERS {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
unsafe impl ::windows::core::Abi for DOT11_PROVISION_DISCOVERY_REQUEST_SEND_COMPLETE_PARAMETERS {
    type Abi = Self;
}
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
impl ::core::cmp::PartialEq for DOT11_PROVISION_DISCOVERY_REQUEST_SEND_COMPLETE_PARAMETERS {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<DOT11_PROVISION_DISCOVERY_REQUEST_SEND_COMPLETE_PARAMETERS>()) == 0 }
    }
}
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
impl ::core::cmp::Eq for DOT11_PROVISION_DISCOVERY_REQUEST_SEND_COMPLETE_PARAMETERS {}
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
impl ::core::default::Default for DOT11_PROVISION_DISCOVERY_REQUEST_SEND_COMPLETE_PARAMETERS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
pub const DOT11_PROVISION_DISCOVERY_REQUEST_SEND_COMPLETE_PARAMETERS_REVISION_1: u32 = 1u32;
#[repr(C)]
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
pub struct DOT11_PROVISION_DISCOVERY_RESPONSE_SEND_COMPLETE_PARAMETERS {
    pub Header: super::Ndis::NDIS_OBJECT_HEADER,
    pub ReceiverDeviceAddress: [u8; 6],
    pub DialogToken: u8,
    pub Status: i32,
    pub uIEsOffset: u32,
    pub uIEsLength: u32,
}
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
impl ::core::marker::Copy for DOT11_PROVISION_DISCOVERY_RESPONSE_SEND_COMPLETE_PARAMETERS {}
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
impl ::core::clone::Clone for DOT11_PROVISION_DISCOVERY_RESPONSE_SEND_COMPLETE_PARAMETERS {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
unsafe impl ::windows::core::Abi for DOT11_PROVISION_DISCOVERY_RESPONSE_SEND_COMPLETE_PARAMETERS {
    type Abi = Self;
}
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
impl ::core::cmp::PartialEq for DOT11_PROVISION_DISCOVERY_RESPONSE_SEND_COMPLETE_PARAMETERS {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<DOT11_PROVISION_DISCOVERY_RESPONSE_SEND_COMPLETE_PARAMETERS>()) == 0 }
    }
}
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
impl ::core::cmp::Eq for DOT11_PROVISION_DISCOVERY_RESPONSE_SEND_COMPLETE_PARAMETERS {}
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
impl ::core::default::Default for DOT11_PROVISION_DISCOVERY_RESPONSE_SEND_COMPLETE_PARAMETERS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
pub const DOT11_PROVISION_DISCOVERY_RESPONSE_SEND_COMPLETE_PARAMETERS_REVISION_1: u32 = 1u32;
pub const DOT11_PSD_IE_MAX_DATA_SIZE: u32 = 240u32;
pub const DOT11_PSD_IE_MAX_ENTRY_NUMBER: u32 = 5u32;
#[repr(C)]
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
pub struct DOT11_QOS_PARAMS {
    pub Header: super::Ndis::NDIS_OBJECT_HEADER,
    pub ucEnabledQoSProtocolFlags: u8,
}
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
impl ::core::marker::Copy for DOT11_QOS_PARAMS {}
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
impl ::core::clone::Clone for DOT11_QOS_PARAMS {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
unsafe impl ::windows::core::Abi for DOT11_QOS_PARAMS {
    type Abi = Self;
}
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
impl ::core::cmp::PartialEq for DOT11_QOS_PARAMS {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<DOT11_QOS_PARAMS>()) == 0 }
    }
}
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
impl ::core::cmp::Eq for DOT11_QOS_PARAMS {}
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
impl ::core::default::Default for DOT11_QOS_PARAMS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
pub const DOT11_QOS_PARAMS_REVISION_1: u32 = 1u32;
#[repr(C)]
pub struct DOT11_QOS_TX_DURATION {
    pub uNominalMSDUSize: u32,
    pub uMinPHYRate: u32,
    pub uDuration: u32,
}
impl ::core::marker::Copy for DOT11_QOS_TX_DURATION {}
impl ::core::clone::Clone for DOT11_QOS_TX_DURATION {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for DOT11_QOS_TX_DURATION {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for DOT11_QOS_TX_DURATION {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<DOT11_QOS_TX_DURATION>()) == 0 }
    }
}
impl ::core::cmp::Eq for DOT11_QOS_TX_DURATION {}
impl ::core::default::Default for DOT11_QOS_TX_DURATION {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct DOT11_QOS_TX_MEDIUM_TIME {
    pub dot11PeerAddress: [u8; 6],
    pub ucQoSPriority: u8,
    pub uMediumTimeAdmited: u32,
}
impl ::core::marker::Copy for DOT11_QOS_TX_MEDIUM_TIME {}
impl ::core::clone::Clone for DOT11_QOS_TX_MEDIUM_TIME {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for DOT11_QOS_TX_MEDIUM_TIME {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for DOT11_QOS_TX_MEDIUM_TIME {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<DOT11_QOS_TX_MEDIUM_TIME>()) == 0 }
    }
}
impl ::core::cmp::Eq for DOT11_QOS_TX_MEDIUM_TIME {}
impl ::core::default::Default for DOT11_QOS_TX_MEDIUM_TIME {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
pub type DOT11_RADIO_STATE = i32;
pub const dot11_radio_state_unknown: DOT11_RADIO_STATE = 0i32;
pub const dot11_radio_state_on: DOT11_RADIO_STATE = 1i32;
pub const dot11_radio_state_off: DOT11_RADIO_STATE = 2i32;
#[repr(C)]
pub struct DOT11_RATE_SET {
    pub uRateSetLength: u32,
    pub ucRateSet: [u8; 126],
}
impl ::core::marker::Copy for DOT11_RATE_SET {}
impl ::core::clone::Clone for DOT11_RATE_SET {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for DOT11_RATE_SET {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for DOT11_RATE_SET {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<DOT11_RATE_SET>()) == 0 }
    }
}
impl ::core::cmp::Eq for DOT11_RATE_SET {}
impl ::core::default::Default for DOT11_RATE_SET {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
pub const DOT11_RATE_SET_MAX_LENGTH: u32 = 126u32;
#[repr(C)]
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
pub struct DOT11_RECEIVED_GO_NEGOTIATION_CONFIRMATION_PARAMETERS {
    pub Header: super::Ndis::NDIS_OBJECT_HEADER,
    pub PeerDeviceAddress: [u8; 6],
    pub DialogToken: u8,
    pub uIEsOffset: u32,
    pub uIEsLength: u32,
}
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
impl ::core::marker::Copy for DOT11_RECEIVED_GO_NEGOTIATION_CONFIRMATION_PARAMETERS {}
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
impl ::core::clone::Clone for DOT11_RECEIVED_GO_NEGOTIATION_CONFIRMATION_PARAMETERS {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
unsafe impl ::windows::core::Abi for DOT11_RECEIVED_GO_NEGOTIATION_CONFIRMATION_PARAMETERS {
    type Abi = Self;
}
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
impl ::core::cmp::PartialEq for DOT11_RECEIVED_GO_NEGOTIATION_CONFIRMATION_PARAMETERS {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<DOT11_RECEIVED_GO_NEGOTIATION_CONFIRMATION_PARAMETERS>()) == 0 }
    }
}
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
impl ::core::cmp::Eq for DOT11_RECEIVED_GO_NEGOTIATION_CONFIRMATION_PARAMETERS {}
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
impl ::core::default::Default for DOT11_RECEIVED_GO_NEGOTIATION_CONFIRMATION_PARAMETERS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
pub const DOT11_RECEIVED_GO_NEGOTIATION_CONFIRMATION_PARAMETERS_REVISION_1: u32 = 1u32;
#[repr(C)]
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
pub struct DOT11_RECEIVED_GO_NEGOTIATION_REQUEST_PARAMETERS {
    pub Header: super::Ndis::NDIS_OBJECT_HEADER,
    pub PeerDeviceAddress: [u8; 6],
    pub DialogToken: u8,
    pub RequestContext: *mut ::core::ffi::c_void,
    pub uIEsOffset: u32,
    pub uIEsLength: u32,
}
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
impl ::core::marker::Copy for DOT11_RECEIVED_GO_NEGOTIATION_REQUEST_PARAMETERS {}
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
impl ::core::clone::Clone for DOT11_RECEIVED_GO_NEGOTIATION_REQUEST_PARAMETERS {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
unsafe impl ::windows::core::Abi for DOT11_RECEIVED_GO_NEGOTIATION_REQUEST_PARAMETERS {
    type Abi = Self;
}
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
impl ::core::cmp::PartialEq for DOT11_RECEIVED_GO_NEGOTIATION_REQUEST_PARAMETERS {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<DOT11_RECEIVED_GO_NEGOTIATION_REQUEST_PARAMETERS>()) == 0 }
    }
}
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
impl ::core::cmp::Eq for DOT11_RECEIVED_GO_NEGOTIATION_REQUEST_PARAMETERS {}
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
impl ::core::default::Default for DOT11_RECEIVED_GO_NEGOTIATION_REQUEST_PARAMETERS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
pub const DOT11_RECEIVED_GO_NEGOTIATION_REQUEST_PARAMETERS_REVISION_1: u32 = 1u32;
#[repr(C)]
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
pub struct DOT11_RECEIVED_GO_NEGOTIATION_RESPONSE_PARAMETERS {
    pub Header: super::Ndis::NDIS_OBJECT_HEADER,
    pub PeerDeviceAddress: [u8; 6],
    pub DialogToken: u8,
    pub ResponseContext: *mut ::core::ffi::c_void,
    pub uIEsOffset: u32,
    pub uIEsLength: u32,
}
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
impl ::core::marker::Copy for DOT11_RECEIVED_GO_NEGOTIATION_RESPONSE_PARAMETERS {}
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
impl ::core::clone::Clone for DOT11_RECEIVED_GO_NEGOTIATION_RESPONSE_PARAMETERS {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
unsafe impl ::windows::core::Abi for DOT11_RECEIVED_GO_NEGOTIATION_RESPONSE_PARAMETERS {
    type Abi = Self;
}
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
impl ::core::cmp::PartialEq for DOT11_RECEIVED_GO_NEGOTIATION_RESPONSE_PARAMETERS {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<DOT11_RECEIVED_GO_NEGOTIATION_RESPONSE_PARAMETERS>()) == 0 }
    }
}
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
impl ::core::cmp::Eq for DOT11_RECEIVED_GO_NEGOTIATION_RESPONSE_PARAMETERS {}
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
impl ::core::default::Default for DOT11_RECEIVED_GO_NEGOTIATION_RESPONSE_PARAMETERS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
pub const DOT11_RECEIVED_GO_NEGOTIATION_RESPONSE_PARAMETERS_REVISION_1: u32 = 1u32;
#[repr(C)]
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
pub struct DOT11_RECEIVED_INVITATION_REQUEST_PARAMETERS {
    pub Header: super::Ndis::NDIS_OBJECT_HEADER,
    pub TransmitterDeviceAddress: [u8; 6],
    pub BSSID: [u8; 6],
    pub DialogToken: u8,
    pub RequestContext: *mut ::core::ffi::c_void,
    pub uIEsOffset: u32,
    pub uIEsLength: u32,
}
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
impl ::core::marker::Copy for DOT11_RECEIVED_INVITATION_REQUEST_PARAMETERS {}
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
impl ::core::clone::Clone for DOT11_RECEIVED_INVITATION_REQUEST_PARAMETERS {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
unsafe impl ::windows::core::Abi for DOT11_RECEIVED_INVITATION_REQUEST_PARAMETERS {
    type Abi = Self;
}
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
impl ::core::cmp::PartialEq for DOT11_RECEIVED_INVITATION_REQUEST_PARAMETERS {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<DOT11_RECEIVED_INVITATION_REQUEST_PARAMETERS>()) == 0 }
    }
}
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
impl ::core::cmp::Eq for DOT11_RECEIVED_INVITATION_REQUEST_PARAMETERS {}
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
impl ::core::default::Default for DOT11_RECEIVED_INVITATION_REQUEST_PARAMETERS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
pub const DOT11_RECEIVED_INVITATION_REQUEST_PARAMETERS_REVISION_1: u32 = 1u32;
#[repr(C)]
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
pub struct DOT11_RECEIVED_INVITATION_RESPONSE_PARAMETERS {
    pub Header: super::Ndis::NDIS_OBJECT_HEADER,
    pub TransmitterDeviceAddress: [u8; 6],
    pub BSSID: [u8; 6],
    pub DialogToken: u8,
    pub uIEsOffset: u32,
    pub uIEsLength: u32,
}
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
impl ::core::marker::Copy for DOT11_RECEIVED_INVITATION_RESPONSE_PARAMETERS {}
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
impl ::core::clone::Clone for DOT11_RECEIVED_INVITATION_RESPONSE_PARAMETERS {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
unsafe impl ::windows::core::Abi for DOT11_RECEIVED_INVITATION_RESPONSE_PARAMETERS {
    type Abi = Self;
}
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
impl ::core::cmp::PartialEq for DOT11_RECEIVED_INVITATION_RESPONSE_PARAMETERS {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<DOT11_RECEIVED_INVITATION_RESPONSE_PARAMETERS>()) == 0 }
    }
}
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
impl ::core::cmp::Eq for DOT11_RECEIVED_INVITATION_RESPONSE_PARAMETERS {}
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
impl ::core::default::Default for DOT11_RECEIVED_INVITATION_RESPONSE_PARAMETERS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
pub const DOT11_RECEIVED_INVITATION_RESPONSE_PARAMETERS_REVISION_1: u32 = 1u32;
#[repr(C)]
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
pub struct DOT11_RECEIVED_PROVISION_DISCOVERY_REQUEST_PARAMETERS {
    pub Header: super::Ndis::NDIS_OBJECT_HEADER,
    pub TransmitterDeviceAddress: [u8; 6],
    pub BSSID: [u8; 6],
    pub DialogToken: u8,
    pub RequestContext: *mut ::core::ffi::c_void,
    pub uIEsOffset: u32,
    pub uIEsLength: u32,
}
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
impl ::core::marker::Copy for DOT11_RECEIVED_PROVISION_DISCOVERY_REQUEST_PARAMETERS {}
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
impl ::core::clone::Clone for DOT11_RECEIVED_PROVISION_DISCOVERY_REQUEST_PARAMETERS {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
unsafe impl ::windows::core::Abi for DOT11_RECEIVED_PROVISION_DISCOVERY_REQUEST_PARAMETERS {
    type Abi = Self;
}
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
impl ::core::cmp::PartialEq for DOT11_RECEIVED_PROVISION_DISCOVERY_REQUEST_PARAMETERS {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<DOT11_RECEIVED_PROVISION_DISCOVERY_REQUEST_PARAMETERS>()) == 0 }
    }
}
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
impl ::core::cmp::Eq for DOT11_RECEIVED_PROVISION_DISCOVERY_REQUEST_PARAMETERS {}
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
impl ::core::default::Default for DOT11_RECEIVED_PROVISION_DISCOVERY_REQUEST_PARAMETERS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
pub const DOT11_RECEIVED_PROVISION_DISCOVERY_REQUEST_PARAMETERS_REVISION_1: u32 = 1u32;
#[repr(C)]
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
pub struct DOT11_RECEIVED_PROVISION_DISCOVERY_RESPONSE_PARAMETERS {
    pub Header: super::Ndis::NDIS_OBJECT_HEADER,
    pub TransmitterDeviceAddress: [u8; 6],
    pub BSSID: [u8; 6],
    pub DialogToken: u8,
    pub uIEsOffset: u32,
    pub uIEsLength: u32,
}
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
impl ::core::marker::Copy for DOT11_RECEIVED_PROVISION_DISCOVERY_RESPONSE_PARAMETERS {}
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
impl ::core::clone::Clone for DOT11_RECEIVED_PROVISION_DISCOVERY_RESPONSE_PARAMETERS {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
unsafe impl ::windows::core::Abi for DOT11_RECEIVED_PROVISION_DISCOVERY_RESPONSE_PARAMETERS {
    type Abi = Self;
}
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
impl ::core::cmp::PartialEq for DOT11_RECEIVED_PROVISION_DISCOVERY_RESPONSE_PARAMETERS {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<DOT11_RECEIVED_PROVISION_DISCOVERY_RESPONSE_PARAMETERS>()) == 0 }
    }
}
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
impl ::core::cmp::Eq for DOT11_RECEIVED_PROVISION_DISCOVERY_RESPONSE_PARAMETERS {}
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
impl ::core::default::Default for DOT11_RECEIVED_PROVISION_DISCOVERY_RESPONSE_PARAMETERS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
pub const DOT11_RECEIVED_PROVISION_DISCOVERY_RESPONSE_PARAMETERS_REVISION_1: u32 = 1u32;
pub const DOT11_RECV_CONTEXT_REVISION_1: u32 = 1u32;
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct DOT11_RECV_EXTENSION_INFO {
    pub uVersion: u32,
    pub pvReserved: *mut ::core::ffi::c_void,
    pub dot11PhyType: DOT11_PHY_TYPE,
    pub uChCenterFrequency: u32,
    pub lRSSI: i32,
    pub lRSSIMin: i32,
    pub lRSSIMax: i32,
    pub uRSSI: u32,
    pub ucPriority: u8,
    pub ucDataRate: u8,
    pub ucPeerMacAddress: [u8; 6],
    pub dwExtendedStatus: u32,
    pub hWEPOffloadContext: super::super::Foundation::HANDLE,
    pub hAuthOffloadContext: super::super::Foundation::HANDLE,
    pub usWEPAppliedMask: u16,
    pub usWPAMSDUPriority: u16,
    pub dot11LowestIV48Counter: DOT11_IV48_COUNTER,
    pub usDot11LeftRWBitMap: u16,
    pub dot11HighestIV48Counter: DOT11_IV48_COUNTER,
    pub usDot11RightRWBitMap: u16,
    pub usNumberOfMPDUsReceived: u16,
    pub usNumberOfFragments: u16,
    pub pNdisPackets: [*mut ::core::ffi::c_void; 1],
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for DOT11_RECV_EXTENSION_INFO {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for DOT11_RECV_EXTENSION_INFO {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for DOT11_RECV_EXTENSION_INFO {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for DOT11_RECV_EXTENSION_INFO {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<DOT11_RECV_EXTENSION_INFO>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for DOT11_RECV_EXTENSION_INFO {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for DOT11_RECV_EXTENSION_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct DOT11_RECV_EXTENSION_INFO_V2 {
    pub uVersion: u32,
    pub pvReserved: *mut ::core::ffi::c_void,
    pub dot11PhyType: DOT11_PHY_TYPE,
    pub uChCenterFrequency: u32,
    pub lRSSI: i32,
    pub uRSSI: u32,
    pub ucPriority: u8,
    pub ucDataRate: u8,
    pub ucPeerMacAddress: [u8; 6],
    pub dwExtendedStatus: u32,
    pub hWEPOffloadContext: super::super::Foundation::HANDLE,
    pub hAuthOffloadContext: super::super::Foundation::HANDLE,
    pub usWEPAppliedMask: u16,
    pub usWPAMSDUPriority: u16,
    pub dot11LowestIV48Counter: DOT11_IV48_COUNTER,
    pub usDot11LeftRWBitMap: u16,
    pub dot11HighestIV48Counter: DOT11_IV48_COUNTER,
    pub usDot11RightRWBitMap: u16,
    pub usNumberOfMPDUsReceived: u16,
    pub usNumberOfFragments: u16,
    pub pNdisPackets: [*mut ::core::ffi::c_void; 1],
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for DOT11_RECV_EXTENSION_INFO_V2 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for DOT11_RECV_EXTENSION_INFO_V2 {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for DOT11_RECV_EXTENSION_INFO_V2 {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for DOT11_RECV_EXTENSION_INFO_V2 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<DOT11_RECV_EXTENSION_INFO_V2>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for DOT11_RECV_EXTENSION_INFO_V2 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for DOT11_RECV_EXTENSION_INFO_V2 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct DOT11_RECV_SENSITIVITY {
    pub ucDataRate: u8,
    pub lRSSIMin: i32,
    pub lRSSIMax: i32,
}
impl ::core::marker::Copy for DOT11_RECV_SENSITIVITY {}
impl ::core::clone::Clone for DOT11_RECV_SENSITIVITY {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for DOT11_RECV_SENSITIVITY {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for DOT11_RECV_SENSITIVITY {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<DOT11_RECV_SENSITIVITY>()) == 0 }
    }
}
impl ::core::cmp::Eq for DOT11_RECV_SENSITIVITY {}
impl ::core::default::Default for DOT11_RECV_SENSITIVITY {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct DOT11_RECV_SENSITIVITY_LIST {
    pub Anonymous: DOT11_RECV_SENSITIVITY_LIST_0,
    pub uNumOfEntries: u32,
    pub uTotalNumOfEntries: u32,
    pub dot11RecvSensitivity: [DOT11_RECV_SENSITIVITY; 1],
}
impl ::core::marker::Copy for DOT11_RECV_SENSITIVITY_LIST {}
impl ::core::clone::Clone for DOT11_RECV_SENSITIVITY_LIST {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for DOT11_RECV_SENSITIVITY_LIST {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for DOT11_RECV_SENSITIVITY_LIST {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<DOT11_RECV_SENSITIVITY_LIST>()) == 0 }
    }
}
impl ::core::cmp::Eq for DOT11_RECV_SENSITIVITY_LIST {}
impl ::core::default::Default for DOT11_RECV_SENSITIVITY_LIST {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub union DOT11_RECV_SENSITIVITY_LIST_0 {
    pub dot11PhyType: DOT11_PHY_TYPE,
    pub uPhyId: u32,
}
impl ::core::marker::Copy for DOT11_RECV_SENSITIVITY_LIST_0 {}
impl ::core::clone::Clone for DOT11_RECV_SENSITIVITY_LIST_0 {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for DOT11_RECV_SENSITIVITY_LIST_0 {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for DOT11_RECV_SENSITIVITY_LIST_0 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<DOT11_RECV_SENSITIVITY_LIST_0>()) == 0 }
    }
}
impl ::core::cmp::Eq for DOT11_RECV_SENSITIVITY_LIST_0 {}
impl ::core::default::Default for DOT11_RECV_SENSITIVITY_LIST_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct DOT11_REG_DOMAINS_SUPPORT_VALUE {
    pub uNumOfEntries: u32,
    pub uTotalNumOfEntries: u32,
    pub dot11RegDomainValue: [DOT11_REG_DOMAIN_VALUE; 1],
}
impl ::core::marker::Copy for DOT11_REG_DOMAINS_SUPPORT_VALUE {}
impl ::core::clone::Clone for DOT11_REG_DOMAINS_SUPPORT_VALUE {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for DOT11_REG_DOMAINS_SUPPORT_VALUE {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for DOT11_REG_DOMAINS_SUPPORT_VALUE {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<DOT11_REG_DOMAINS_SUPPORT_VALUE>()) == 0 }
    }
}
impl ::core::cmp::Eq for DOT11_REG_DOMAINS_SUPPORT_VALUE {}
impl ::core::default::Default for DOT11_REG_DOMAINS_SUPPORT_VALUE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
pub const DOT11_REG_DOMAIN_DOC: u32 = 32u32;
pub const DOT11_REG_DOMAIN_ETSI: u32 = 48u32;
pub const DOT11_REG_DOMAIN_FCC: u32 = 16u32;
pub const DOT11_REG_DOMAIN_FRANCE: u32 = 50u32;
pub const DOT11_REG_DOMAIN_MKK: u32 = 64u32;
pub const DOT11_REG_DOMAIN_OTHER: u32 = 0u32;
pub const DOT11_REG_DOMAIN_SPAIN: u32 = 49u32;
#[repr(C)]
pub struct DOT11_REG_DOMAIN_VALUE {
    pub uRegDomainsSupportIndex: u32,
    pub uRegDomainsSupportValue: u32,
}
impl ::core::marker::Copy for DOT11_REG_DOMAIN_VALUE {}
impl ::core::clone::Clone for DOT11_REG_DOMAIN_VALUE {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for DOT11_REG_DOMAIN_VALUE {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for DOT11_REG_DOMAIN_VALUE {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<DOT11_REG_DOMAIN_VALUE>()) == 0 }
    }
}
impl ::core::cmp::Eq for DOT11_REG_DOMAIN_VALUE {}
impl ::core::default::Default for DOT11_REG_DOMAIN_VALUE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct DOT11_RESET_REQUEST {
    pub dot11ResetType: DOT11_RESET_TYPE,
    pub dot11MacAddress: [u8; 6],
    pub bSetDefaultMIB: super::super::Foundation::BOOLEAN,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for DOT11_RESET_REQUEST {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for DOT11_RESET_REQUEST {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for DOT11_RESET_REQUEST {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for DOT11_RESET_REQUEST {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<DOT11_RESET_REQUEST>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for DOT11_RESET_REQUEST {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for DOT11_RESET_REQUEST {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
pub type DOT11_RESET_TYPE = i32;
pub const dot11_reset_type_phy: DOT11_RESET_TYPE = 1i32;
pub const dot11_reset_type_mac: DOT11_RESET_TYPE = 2i32;
pub const dot11_reset_type_phy_and_mac: DOT11_RESET_TYPE = 3i32;
#[repr(C)]
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
pub struct DOT11_ROAMING_COMPLETION_PARAMETERS {
    pub Header: super::Ndis::NDIS_OBJECT_HEADER,
    pub uStatus: u32,
}
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
impl ::core::marker::Copy for DOT11_ROAMING_COMPLETION_PARAMETERS {}
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
impl ::core::clone::Clone for DOT11_ROAMING_COMPLETION_PARAMETERS {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
unsafe impl ::windows::core::Abi for DOT11_ROAMING_COMPLETION_PARAMETERS {
    type Abi = Self;
}
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
impl ::core::cmp::PartialEq for DOT11_ROAMING_COMPLETION_PARAMETERS {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<DOT11_ROAMING_COMPLETION_PARAMETERS>()) == 0 }
    }
}
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
impl ::core::cmp::Eq for DOT11_ROAMING_COMPLETION_PARAMETERS {}
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
impl ::core::default::Default for DOT11_ROAMING_COMPLETION_PARAMETERS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
pub const DOT11_ROAMING_COMPLETION_PARAMETERS_REVISION_1: u32 = 1u32;
#[repr(C)]
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
pub struct DOT11_ROAMING_START_PARAMETERS {
    pub Header: super::Ndis::NDIS_OBJECT_HEADER,
    pub AdhocBSSID: [u8; 6],
    pub AdhocSSID: DOT11_SSID,
    pub uRoamingReason: u32,
}
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
impl ::core::marker::Copy for DOT11_ROAMING_START_PARAMETERS {}
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
impl ::core::clone::Clone for DOT11_ROAMING_START_PARAMETERS {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
unsafe impl ::windows::core::Abi for DOT11_ROAMING_START_PARAMETERS {
    type Abi = Self;
}
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
impl ::core::cmp::PartialEq for DOT11_ROAMING_START_PARAMETERS {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<DOT11_ROAMING_START_PARAMETERS>()) == 0 }
    }
}
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
impl ::core::cmp::Eq for DOT11_ROAMING_START_PARAMETERS {}
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
impl ::core::default::Default for DOT11_ROAMING_START_PARAMETERS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
pub const DOT11_ROAMING_START_PARAMETERS_REVISION_1: u32 = 1u32;
#[repr(C)]
pub struct DOT11_RSSI_RANGE {
    pub dot11PhyType: DOT11_PHY_TYPE,
    pub uRSSIMin: u32,
    pub uRSSIMax: u32,
}
impl ::core::marker::Copy for DOT11_RSSI_RANGE {}
impl ::core::clone::Clone for DOT11_RSSI_RANGE {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for DOT11_RSSI_RANGE {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for DOT11_RSSI_RANGE {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<DOT11_RSSI_RANGE>()) == 0 }
    }
}
impl ::core::cmp::Eq for DOT11_RSSI_RANGE {}
impl ::core::default::Default for DOT11_RSSI_RANGE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct DOT11_SCAN_REQUEST {
    pub dot11BSSType: DOT11_BSS_TYPE,
    pub dot11BSSID: [u8; 6],
    pub dot11SSID: DOT11_SSID,
    pub dot11ScanType: DOT11_SCAN_TYPE,
    pub bRestrictedScan: super::super::Foundation::BOOLEAN,
    pub bUseRequestIE: super::super::Foundation::BOOLEAN,
    pub uRequestIDsOffset: u32,
    pub uNumOfRequestIDs: u32,
    pub uPhyTypesOffset: u32,
    pub uNumOfPhyTypes: u32,
    pub uIEsOffset: u32,
    pub uIEsLength: u32,
    pub ucBuffer: [u8; 1],
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for DOT11_SCAN_REQUEST {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for DOT11_SCAN_REQUEST {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for DOT11_SCAN_REQUEST {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for DOT11_SCAN_REQUEST {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<DOT11_SCAN_REQUEST>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for DOT11_SCAN_REQUEST {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for DOT11_SCAN_REQUEST {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct DOT11_SCAN_REQUEST_V2 {
    pub dot11BSSType: DOT11_BSS_TYPE,
    pub dot11BSSID: [u8; 6],
    pub dot11ScanType: DOT11_SCAN_TYPE,
    pub bRestrictedScan: super::super::Foundation::BOOLEAN,
    pub udot11SSIDsOffset: u32,
    pub uNumOfdot11SSIDs: u32,
    pub bUseRequestIE: super::super::Foundation::BOOLEAN,
    pub uRequestIDsOffset: u32,
    pub uNumOfRequestIDs: u32,
    pub uPhyTypeInfosOffset: u32,
    pub uNumOfPhyTypeInfos: u32,
    pub uIEsOffset: u32,
    pub uIEsLength: u32,
    pub ucBuffer: [u8; 1],
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for DOT11_SCAN_REQUEST_V2 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for DOT11_SCAN_REQUEST_V2 {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for DOT11_SCAN_REQUEST_V2 {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for DOT11_SCAN_REQUEST_V2 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<DOT11_SCAN_REQUEST_V2>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for DOT11_SCAN_REQUEST_V2 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for DOT11_SCAN_REQUEST_V2 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
pub type DOT11_SCAN_TYPE = i32;
pub const dot11_scan_type_active: DOT11_SCAN_TYPE = 1i32;
pub const dot11_scan_type_passive: DOT11_SCAN_TYPE = 2i32;
pub const dot11_scan_type_auto: DOT11_SCAN_TYPE = 3i32;
pub const dot11_scan_type_forced: DOT11_SCAN_TYPE = -2147483648i32;
pub const DOT11_SEND_CONTEXT_REVISION_1: u32 = 1u32;
#[repr(C)]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_NetworkManagement_Ndis"))]
pub struct DOT11_SEND_GO_NEGOTIATION_CONFIRMATION_PARAMETERS {
    pub Header: super::Ndis::NDIS_OBJECT_HEADER,
    pub PeerDeviceAddress: [u8; 6],
    pub DialogToken: u8,
    pub ResponseContext: *mut ::core::ffi::c_void,
    pub uSendTimeout: u32,
    pub Status: u8,
    pub GroupCapability: u8,
    pub GroupID: DOT11_WFD_GROUP_ID,
    pub bUseGroupID: super::super::Foundation::BOOLEAN,
    pub uIEsOffset: u32,
    pub uIEsLength: u32,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_NetworkManagement_Ndis"))]
impl ::core::marker::Copy for DOT11_SEND_GO_NEGOTIATION_CONFIRMATION_PARAMETERS {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_NetworkManagement_Ndis"))]
impl ::core::clone::Clone for DOT11_SEND_GO_NEGOTIATION_CONFIRMATION_PARAMETERS {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_NetworkManagement_Ndis"))]
unsafe impl ::windows::core::Abi for DOT11_SEND_GO_NEGOTIATION_CONFIRMATION_PARAMETERS {
    type Abi = Self;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_NetworkManagement_Ndis"))]
impl ::core::cmp::PartialEq for DOT11_SEND_GO_NEGOTIATION_CONFIRMATION_PARAMETERS {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<DOT11_SEND_GO_NEGOTIATION_CONFIRMATION_PARAMETERS>()) == 0 }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_NetworkManagement_Ndis"))]
impl ::core::cmp::Eq for DOT11_SEND_GO_NEGOTIATION_CONFIRMATION_PARAMETERS {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_NetworkManagement_Ndis"))]
impl ::core::default::Default for DOT11_SEND_GO_NEGOTIATION_CONFIRMATION_PARAMETERS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
pub const DOT11_SEND_GO_NEGOTIATION_CONFIRMATION_PARAMETERS_REVISION_1: u32 = 1u32;
#[repr(C)]
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
pub struct DOT11_SEND_GO_NEGOTIATION_REQUEST_PARAMETERS {
    pub Header: super::Ndis::NDIS_OBJECT_HEADER,
    pub PeerDeviceAddress: [u8; 6],
    pub DialogToken: u8,
    pub uSendTimeout: u32,
    pub GroupOwnerIntent: DOT11_WFD_GO_INTENT,
    pub MinimumConfigTimeout: DOT11_WFD_CONFIGURATION_TIMEOUT,
    pub IntendedInterfaceAddress: [u8; 6],
    pub GroupCapability: u8,
    pub uIEsOffset: u32,
    pub uIEsLength: u32,
}
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
impl ::core::marker::Copy for DOT11_SEND_GO_NEGOTIATION_REQUEST_PARAMETERS {}
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
impl ::core::clone::Clone for DOT11_SEND_GO_NEGOTIATION_REQUEST_PARAMETERS {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
unsafe impl ::windows::core::Abi for DOT11_SEND_GO_NEGOTIATION_REQUEST_PARAMETERS {
    type Abi = Self;
}
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
impl ::core::cmp::PartialEq for DOT11_SEND_GO_NEGOTIATION_REQUEST_PARAMETERS {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<DOT11_SEND_GO_NEGOTIATION_REQUEST_PARAMETERS>()) == 0 }
    }
}
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
impl ::core::cmp::Eq for DOT11_SEND_GO_NEGOTIATION_REQUEST_PARAMETERS {}
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
impl ::core::default::Default for DOT11_SEND_GO_NEGOTIATION_REQUEST_PARAMETERS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
pub const DOT11_SEND_GO_NEGOTIATION_REQUEST_PARAMETERS_REVISION_1: u32 = 1u32;
#[repr(C)]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_NetworkManagement_Ndis"))]
pub struct DOT11_SEND_GO_NEGOTIATION_RESPONSE_PARAMETERS {
    pub Header: super::Ndis::NDIS_OBJECT_HEADER,
    pub PeerDeviceAddress: [u8; 6],
    pub DialogToken: u8,
    pub RequestContext: *mut ::core::ffi::c_void,
    pub uSendTimeout: u32,
    pub Status: u8,
    pub GroupOwnerIntent: DOT11_WFD_GO_INTENT,
    pub MinimumConfigTimeout: DOT11_WFD_CONFIGURATION_TIMEOUT,
    pub IntendedInterfaceAddress: [u8; 6],
    pub GroupCapability: u8,
    pub GroupID: DOT11_WFD_GROUP_ID,
    pub bUseGroupID: super::super::Foundation::BOOLEAN,
    pub uIEsOffset: u32,
    pub uIEsLength: u32,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_NetworkManagement_Ndis"))]
impl ::core::marker::Copy for DOT11_SEND_GO_NEGOTIATION_RESPONSE_PARAMETERS {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_NetworkManagement_Ndis"))]
impl ::core::clone::Clone for DOT11_SEND_GO_NEGOTIATION_RESPONSE_PARAMETERS {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_NetworkManagement_Ndis"))]
unsafe impl ::windows::core::Abi for DOT11_SEND_GO_NEGOTIATION_RESPONSE_PARAMETERS {
    type Abi = Self;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_NetworkManagement_Ndis"))]
impl ::core::cmp::PartialEq for DOT11_SEND_GO_NEGOTIATION_RESPONSE_PARAMETERS {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<DOT11_SEND_GO_NEGOTIATION_RESPONSE_PARAMETERS>()) == 0 }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_NetworkManagement_Ndis"))]
impl ::core::cmp::Eq for DOT11_SEND_GO_NEGOTIATION_RESPONSE_PARAMETERS {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_NetworkManagement_Ndis"))]
impl ::core::default::Default for DOT11_SEND_GO_NEGOTIATION_RESPONSE_PARAMETERS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
pub const DOT11_SEND_GO_NEGOTIATION_RESPONSE_PARAMETERS_REVISION_1: u32 = 1u32;
#[repr(C)]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_NetworkManagement_Ndis"))]
pub struct DOT11_SEND_INVITATION_REQUEST_PARAMETERS {
    pub Header: super::Ndis::NDIS_OBJECT_HEADER,
    pub DialogToken: u8,
    pub PeerDeviceAddress: [u8; 6],
    pub uSendTimeout: u32,
    pub MinimumConfigTimeout: DOT11_WFD_CONFIGURATION_TIMEOUT,
    pub InvitationFlags: DOT11_WFD_INVITATION_FLAGS,
    pub GroupBSSID: [u8; 6],
    pub bUseGroupBSSID: super::super::Foundation::BOOLEAN,
    pub OperatingChannel: DOT11_WFD_CHANNEL,
    pub bUseSpecifiedOperatingChannel: super::super::Foundation::BOOLEAN,
    pub GroupID: DOT11_WFD_GROUP_ID,
    pub bLocalGO: super::super::Foundation::BOOLEAN,
    pub uIEsOffset: u32,
    pub uIEsLength: u32,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_NetworkManagement_Ndis"))]
impl ::core::marker::Copy for DOT11_SEND_INVITATION_REQUEST_PARAMETERS {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_NetworkManagement_Ndis"))]
impl ::core::clone::Clone for DOT11_SEND_INVITATION_REQUEST_PARAMETERS {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_NetworkManagement_Ndis"))]
unsafe impl ::windows::core::Abi for DOT11_SEND_INVITATION_REQUEST_PARAMETERS {
    type Abi = Self;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_NetworkManagement_Ndis"))]
impl ::core::cmp::PartialEq for DOT11_SEND_INVITATION_REQUEST_PARAMETERS {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<DOT11_SEND_INVITATION_REQUEST_PARAMETERS>()) == 0 }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_NetworkManagement_Ndis"))]
impl ::core::cmp::Eq for DOT11_SEND_INVITATION_REQUEST_PARAMETERS {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_NetworkManagement_Ndis"))]
impl ::core::default::Default for DOT11_SEND_INVITATION_REQUEST_PARAMETERS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
pub const DOT11_SEND_INVITATION_REQUEST_PARAMETERS_REVISION_1: u32 = 1u32;
#[repr(C)]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_NetworkManagement_Ndis"))]
pub struct DOT11_SEND_INVITATION_RESPONSE_PARAMETERS {
    pub Header: super::Ndis::NDIS_OBJECT_HEADER,
    pub ReceiverDeviceAddress: [u8; 6],
    pub DialogToken: u8,
    pub RequestContext: *mut ::core::ffi::c_void,
    pub uSendTimeout: u32,
    pub Status: u8,
    pub MinimumConfigTimeout: DOT11_WFD_CONFIGURATION_TIMEOUT,
    pub GroupBSSID: [u8; 6],
    pub bUseGroupBSSID: super::super::Foundation::BOOLEAN,
    pub OperatingChannel: DOT11_WFD_CHANNEL,
    pub bUseSpecifiedOperatingChannel: super::super::Foundation::BOOLEAN,
    pub uIEsOffset: u32,
    pub uIEsLength: u32,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_NetworkManagement_Ndis"))]
impl ::core::marker::Copy for DOT11_SEND_INVITATION_RESPONSE_PARAMETERS {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_NetworkManagement_Ndis"))]
impl ::core::clone::Clone for DOT11_SEND_INVITATION_RESPONSE_PARAMETERS {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_NetworkManagement_Ndis"))]
unsafe impl ::windows::core::Abi for DOT11_SEND_INVITATION_RESPONSE_PARAMETERS {
    type Abi = Self;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_NetworkManagement_Ndis"))]
impl ::core::cmp::PartialEq for DOT11_SEND_INVITATION_RESPONSE_PARAMETERS {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<DOT11_SEND_INVITATION_RESPONSE_PARAMETERS>()) == 0 }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_NetworkManagement_Ndis"))]
impl ::core::cmp::Eq for DOT11_SEND_INVITATION_RESPONSE_PARAMETERS {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_NetworkManagement_Ndis"))]
impl ::core::default::Default for DOT11_SEND_INVITATION_RESPONSE_PARAMETERS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
pub const DOT11_SEND_INVITATION_RESPONSE_PARAMETERS_REVISION_1: u32 = 1u32;
#[repr(C)]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_NetworkManagement_Ndis"))]
pub struct DOT11_SEND_PROVISION_DISCOVERY_REQUEST_PARAMETERS {
    pub Header: super::Ndis::NDIS_OBJECT_HEADER,
    pub DialogToken: u8,
    pub PeerDeviceAddress: [u8; 6],
    pub uSendTimeout: u32,
    pub GroupCapability: u8,
    pub GroupID: DOT11_WFD_GROUP_ID,
    pub bUseGroupID: super::super::Foundation::BOOLEAN,
    pub uIEsOffset: u32,
    pub uIEsLength: u32,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_NetworkManagement_Ndis"))]
impl ::core::marker::Copy for DOT11_SEND_PROVISION_DISCOVERY_REQUEST_PARAMETERS {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_NetworkManagement_Ndis"))]
impl ::core::clone::Clone for DOT11_SEND_PROVISION_DISCOVERY_REQUEST_PARAMETERS {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_NetworkManagement_Ndis"))]
unsafe impl ::windows::core::Abi for DOT11_SEND_PROVISION_DISCOVERY_REQUEST_PARAMETERS {
    type Abi = Self;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_NetworkManagement_Ndis"))]
impl ::core::cmp::PartialEq for DOT11_SEND_PROVISION_DISCOVERY_REQUEST_PARAMETERS {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<DOT11_SEND_PROVISION_DISCOVERY_REQUEST_PARAMETERS>()) == 0 }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_NetworkManagement_Ndis"))]
impl ::core::cmp::Eq for DOT11_SEND_PROVISION_DISCOVERY_REQUEST_PARAMETERS {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_NetworkManagement_Ndis"))]
impl ::core::default::Default for DOT11_SEND_PROVISION_DISCOVERY_REQUEST_PARAMETERS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
pub const DOT11_SEND_PROVISION_DISCOVERY_REQUEST_PARAMETERS_REVISION_1: u32 = 1u32;
#[repr(C)]
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
pub struct DOT11_SEND_PROVISION_DISCOVERY_RESPONSE_PARAMETERS {
    pub Header: super::Ndis::NDIS_OBJECT_HEADER,
    pub ReceiverDeviceAddress: [u8; 6],
    pub DialogToken: u8,
    pub RequestContext: *mut ::core::ffi::c_void,
    pub uSendTimeout: u32,
    pub uIEsOffset: u32,
    pub uIEsLength: u32,
}
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
impl ::core::marker::Copy for DOT11_SEND_PROVISION_DISCOVERY_RESPONSE_PARAMETERS {}
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
impl ::core::clone::Clone for DOT11_SEND_PROVISION_DISCOVERY_RESPONSE_PARAMETERS {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
unsafe impl ::windows::core::Abi for DOT11_SEND_PROVISION_DISCOVERY_RESPONSE_PARAMETERS {
    type Abi = Self;
}
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
impl ::core::cmp::PartialEq for DOT11_SEND_PROVISION_DISCOVERY_RESPONSE_PARAMETERS {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<DOT11_SEND_PROVISION_DISCOVERY_RESPONSE_PARAMETERS>()) == 0 }
    }
}
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
impl ::core::cmp::Eq for DOT11_SEND_PROVISION_DISCOVERY_RESPONSE_PARAMETERS {}
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
impl ::core::default::Default for DOT11_SEND_PROVISION_DISCOVERY_RESPONSE_PARAMETERS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
pub const DOT11_SEND_PROVISION_DISCOVERY_RESPONSE_PARAMETERS_REVISION_1: u32 = 1u32;
pub const DOT11_SERVICE_CLASS_REORDERABLE_MULTICAST: u32 = 0u32;
pub const DOT11_SERVICE_CLASS_STRICTLY_ORDERED: u32 = 1u32;
#[repr(C)]
pub struct DOT11_SSID {
    pub uSSIDLength: u32,
    pub ucSSID: [u8; 32],
}
impl ::core::marker::Copy for DOT11_SSID {}
impl ::core::clone::Clone for DOT11_SSID {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for DOT11_SSID {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for DOT11_SSID {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<DOT11_SSID>()) == 0 }
    }
}
impl ::core::cmp::Eq for DOT11_SSID {}
impl ::core::default::Default for DOT11_SSID {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
pub struct DOT11_SSID_LIST {
    pub Header: super::Ndis::NDIS_OBJECT_HEADER,
    pub uNumOfEntries: u32,
    pub uTotalNumOfEntries: u32,
    pub SSIDs: [DOT11_SSID; 1],
}
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
impl ::core::marker::Copy for DOT11_SSID_LIST {}
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
impl ::core::clone::Clone for DOT11_SSID_LIST {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
unsafe impl ::windows::core::Abi for DOT11_SSID_LIST {
    type Abi = Self;
}
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
impl ::core::cmp::PartialEq for DOT11_SSID_LIST {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<DOT11_SSID_LIST>()) == 0 }
    }
}
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
impl ::core::cmp::Eq for DOT11_SSID_LIST {}
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
impl ::core::default::Default for DOT11_SSID_LIST {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
pub const DOT11_SSID_LIST_REVISION_1: u32 = 1u32;
pub const DOT11_SSID_MAX_LENGTH: u32 = 32u32;
#[repr(C)]
pub struct DOT11_START_REQUEST {
    pub uStartFailureTimeout: u32,
    pub OperationalRateSet: DOT11_RATE_SET,
    pub uChCenterFrequency: u32,
    pub dot11BSSDescription: DOT11_BSS_DESCRIPTION,
}
impl ::core::marker::Copy for DOT11_START_REQUEST {}
impl ::core::clone::Clone for DOT11_START_REQUEST {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for DOT11_START_REQUEST {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for DOT11_START_REQUEST {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<DOT11_START_REQUEST>()) == 0 }
    }
}
impl ::core::cmp::Eq for DOT11_START_REQUEST {}
impl ::core::default::Default for DOT11_START_REQUEST {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
pub struct DOT11_STATISTICS {
    pub Header: super::Ndis::NDIS_OBJECT_HEADER,
    pub ullFourWayHandshakeFailures: u64,
    pub ullTKIPCounterMeasuresInvoked: u64,
    pub ullReserved: u64,
    pub MacUcastCounters: DOT11_MAC_FRAME_STATISTICS,
    pub MacMcastCounters: DOT11_MAC_FRAME_STATISTICS,
    pub PhyCounters: [DOT11_PHY_FRAME_STATISTICS; 1],
}
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
impl ::core::marker::Copy for DOT11_STATISTICS {}
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
impl ::core::clone::Clone for DOT11_STATISTICS {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
unsafe impl ::windows::core::Abi for DOT11_STATISTICS {
    type Abi = Self;
}
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
impl ::core::cmp::PartialEq for DOT11_STATISTICS {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<DOT11_STATISTICS>()) == 0 }
    }
}
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
impl ::core::cmp::Eq for DOT11_STATISTICS {}
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
impl ::core::default::Default for DOT11_STATISTICS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
pub const DOT11_STATISTICS_REVISION_1: u32 = 1u32;
pub const DOT11_STATUS_AP_JOIN_CONFIRM: u32 = 5u32;
pub const DOT11_STATUS_AUTH_FAILED: u32 = 131072u32;
pub const DOT11_STATUS_AUTH_NOT_VERIFIED: u32 = 32768u32;
pub const DOT11_STATUS_AUTH_VERIFIED: u32 = 65536u32;
pub const DOT11_STATUS_ENCRYPTION_FAILED: u32 = 512u32;
pub const DOT11_STATUS_EXCESSIVE_DATA_LENGTH: u32 = 256u32;
pub const DOT11_STATUS_GENERATE_AUTH_FAILED: u32 = 16384u32;
pub const DOT11_STATUS_ICV_VERIFIED: u32 = 2048u32;
#[repr(C)]
pub struct DOT11_STATUS_INDICATION {
    pub uStatusType: u32,
    pub ndisStatus: i32,
}
impl ::core::marker::Copy for DOT11_STATUS_INDICATION {}
impl ::core::clone::Clone for DOT11_STATUS_INDICATION {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for DOT11_STATUS_INDICATION {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for DOT11_STATUS_INDICATION {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<DOT11_STATUS_INDICATION>()) == 0 }
    }
}
impl ::core::cmp::Eq for DOT11_STATUS_INDICATION {}
impl ::core::default::Default for DOT11_STATUS_INDICATION {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
pub const DOT11_STATUS_JOIN_CONFIRM: u32 = 2u32;
pub const DOT11_STATUS_MPDU_MAX_LENGTH_CHANGED: u32 = 6u32;
pub const DOT11_STATUS_PACKET_NOT_REASSEMBLED: u32 = 8192u32;
pub const DOT11_STATUS_PACKET_REASSEMBLED: u32 = 4096u32;
pub const DOT11_STATUS_PS_LIFETIME_EXPIRED: u32 = 262144u32;
pub const DOT11_STATUS_RESET_CONFIRM: u32 = 4u32;
pub const DOT11_STATUS_RETRY_LIMIT_EXCEEDED: u32 = 2u32;
pub const DOT11_STATUS_SCAN_CONFIRM: u32 = 1u32;
pub const DOT11_STATUS_START_CONFIRM: u32 = 3u32;
pub const DOT11_STATUS_SUCCESS: u32 = 1u32;
pub const DOT11_STATUS_UNAVAILABLE_BSS: u32 = 128u32;
pub const DOT11_STATUS_UNAVAILABLE_PRIORITY: u32 = 16u32;
pub const DOT11_STATUS_UNAVAILABLE_SERVICE_CLASS: u32 = 32u32;
pub const DOT11_STATUS_UNSUPPORTED_PRIORITY: u32 = 4u32;
pub const DOT11_STATUS_UNSUPPORTED_SERVICE_CLASS: u32 = 8u32;
pub const DOT11_STATUS_WEP_KEY_UNAVAILABLE: u32 = 1024u32;
pub const DOT11_STATUS_XMIT_MSDU_TIMER_EXPIRED: u32 = 64u32;
#[repr(C)]
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
pub struct DOT11_STOP_AP_PARAMETERS {
    pub Header: super::Ndis::NDIS_OBJECT_HEADER,
    pub ulReason: u32,
}
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
impl ::core::marker::Copy for DOT11_STOP_AP_PARAMETERS {}
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
impl ::core::clone::Clone for DOT11_STOP_AP_PARAMETERS {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
unsafe impl ::windows::core::Abi for DOT11_STOP_AP_PARAMETERS {
    type Abi = Self;
}
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
impl ::core::cmp::PartialEq for DOT11_STOP_AP_PARAMETERS {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<DOT11_STOP_AP_PARAMETERS>()) == 0 }
    }
}
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
impl ::core::cmp::Eq for DOT11_STOP_AP_PARAMETERS {}
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
impl ::core::default::Default for DOT11_STOP_AP_PARAMETERS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
pub const DOT11_STOP_AP_PARAMETERS_REVISION_1: u32 = 1u32;
pub const DOT11_STOP_AP_REASON_AP_ACTIVE: u32 = 3u32;
pub const DOT11_STOP_AP_REASON_CHANNEL_NOT_AVAILABLE: u32 = 2u32;
pub const DOT11_STOP_AP_REASON_FREQUENCY_NOT_AVAILABLE: u32 = 1u32;
pub const DOT11_STOP_AP_REASON_IHV_END: u32 = 4294967295u32;
pub const DOT11_STOP_AP_REASON_IHV_START: u32 = 4278190080u32;
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct DOT11_SUPPORTED_ANTENNA {
    pub uAntennaListIndex: u32,
    pub bSupportedAntenna: super::super::Foundation::BOOLEAN,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for DOT11_SUPPORTED_ANTENNA {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for DOT11_SUPPORTED_ANTENNA {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for DOT11_SUPPORTED_ANTENNA {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for DOT11_SUPPORTED_ANTENNA {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<DOT11_SUPPORTED_ANTENNA>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for DOT11_SUPPORTED_ANTENNA {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for DOT11_SUPPORTED_ANTENNA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct DOT11_SUPPORTED_ANTENNA_LIST {
    pub uNumOfEntries: u32,
    pub uTotalNumOfEntries: u32,
    pub dot11SupportedAntenna: [DOT11_SUPPORTED_ANTENNA; 1],
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for DOT11_SUPPORTED_ANTENNA_LIST {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for DOT11_SUPPORTED_ANTENNA_LIST {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for DOT11_SUPPORTED_ANTENNA_LIST {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for DOT11_SUPPORTED_ANTENNA_LIST {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<DOT11_SUPPORTED_ANTENNA_LIST>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for DOT11_SUPPORTED_ANTENNA_LIST {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for DOT11_SUPPORTED_ANTENNA_LIST {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct DOT11_SUPPORTED_DATA_RATES_VALUE {
    pub ucSupportedTxDataRatesValue: [u8; 8],
    pub ucSupportedRxDataRatesValue: [u8; 8],
}
impl ::core::marker::Copy for DOT11_SUPPORTED_DATA_RATES_VALUE {}
impl ::core::clone::Clone for DOT11_SUPPORTED_DATA_RATES_VALUE {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for DOT11_SUPPORTED_DATA_RATES_VALUE {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for DOT11_SUPPORTED_DATA_RATES_VALUE {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<DOT11_SUPPORTED_DATA_RATES_VALUE>()) == 0 }
    }
}
impl ::core::cmp::Eq for DOT11_SUPPORTED_DATA_RATES_VALUE {}
impl ::core::default::Default for DOT11_SUPPORTED_DATA_RATES_VALUE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct DOT11_SUPPORTED_DATA_RATES_VALUE_V2 {
    pub ucSupportedTxDataRatesValue: [u8; 255],
    pub ucSupportedRxDataRatesValue: [u8; 255],
}
impl ::core::marker::Copy for DOT11_SUPPORTED_DATA_RATES_VALUE_V2 {}
impl ::core::clone::Clone for DOT11_SUPPORTED_DATA_RATES_VALUE_V2 {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for DOT11_SUPPORTED_DATA_RATES_VALUE_V2 {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for DOT11_SUPPORTED_DATA_RATES_VALUE_V2 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<DOT11_SUPPORTED_DATA_RATES_VALUE_V2>()) == 0 }
    }
}
impl ::core::cmp::Eq for DOT11_SUPPORTED_DATA_RATES_VALUE_V2 {}
impl ::core::default::Default for DOT11_SUPPORTED_DATA_RATES_VALUE_V2 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct DOT11_SUPPORTED_DSSS_CHANNEL {
    pub uChannel: u32,
}
impl ::core::marker::Copy for DOT11_SUPPORTED_DSSS_CHANNEL {}
impl ::core::clone::Clone for DOT11_SUPPORTED_DSSS_CHANNEL {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for DOT11_SUPPORTED_DSSS_CHANNEL {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for DOT11_SUPPORTED_DSSS_CHANNEL {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<DOT11_SUPPORTED_DSSS_CHANNEL>()) == 0 }
    }
}
impl ::core::cmp::Eq for DOT11_SUPPORTED_DSSS_CHANNEL {}
impl ::core::default::Default for DOT11_SUPPORTED_DSSS_CHANNEL {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct DOT11_SUPPORTED_DSSS_CHANNEL_LIST {
    pub uNumOfEntries: u32,
    pub uTotalNumOfEntries: u32,
    pub dot11SupportedDSSSChannel: [DOT11_SUPPORTED_DSSS_CHANNEL; 1],
}
impl ::core::marker::Copy for DOT11_SUPPORTED_DSSS_CHANNEL_LIST {}
impl ::core::clone::Clone for DOT11_SUPPORTED_DSSS_CHANNEL_LIST {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for DOT11_SUPPORTED_DSSS_CHANNEL_LIST {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for DOT11_SUPPORTED_DSSS_CHANNEL_LIST {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<DOT11_SUPPORTED_DSSS_CHANNEL_LIST>()) == 0 }
    }
}
impl ::core::cmp::Eq for DOT11_SUPPORTED_DSSS_CHANNEL_LIST {}
impl ::core::default::Default for DOT11_SUPPORTED_DSSS_CHANNEL_LIST {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct DOT11_SUPPORTED_OFDM_FREQUENCY {
    pub uCenterFrequency: u32,
}
impl ::core::marker::Copy for DOT11_SUPPORTED_OFDM_FREQUENCY {}
impl ::core::clone::Clone for DOT11_SUPPORTED_OFDM_FREQUENCY {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for DOT11_SUPPORTED_OFDM_FREQUENCY {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for DOT11_SUPPORTED_OFDM_FREQUENCY {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<DOT11_SUPPORTED_OFDM_FREQUENCY>()) == 0 }
    }
}
impl ::core::cmp::Eq for DOT11_SUPPORTED_OFDM_FREQUENCY {}
impl ::core::default::Default for DOT11_SUPPORTED_OFDM_FREQUENCY {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct DOT11_SUPPORTED_OFDM_FREQUENCY_LIST {
    pub uNumOfEntries: u32,
    pub uTotalNumOfEntries: u32,
    pub dot11SupportedOFDMFrequency: [DOT11_SUPPORTED_OFDM_FREQUENCY; 1],
}
impl ::core::marker::Copy for DOT11_SUPPORTED_OFDM_FREQUENCY_LIST {}
impl ::core::clone::Clone for DOT11_SUPPORTED_OFDM_FREQUENCY_LIST {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for DOT11_SUPPORTED_OFDM_FREQUENCY_LIST {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for DOT11_SUPPORTED_OFDM_FREQUENCY_LIST {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<DOT11_SUPPORTED_OFDM_FREQUENCY_LIST>()) == 0 }
    }
}
impl ::core::cmp::Eq for DOT11_SUPPORTED_OFDM_FREQUENCY_LIST {}
impl ::core::default::Default for DOT11_SUPPORTED_OFDM_FREQUENCY_LIST {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct DOT11_SUPPORTED_PHY_TYPES {
    pub uNumOfEntries: u32,
    pub uTotalNumOfEntries: u32,
    pub dot11PHYType: [DOT11_PHY_TYPE; 1],
}
impl ::core::marker::Copy for DOT11_SUPPORTED_PHY_TYPES {}
impl ::core::clone::Clone for DOT11_SUPPORTED_PHY_TYPES {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for DOT11_SUPPORTED_PHY_TYPES {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for DOT11_SUPPORTED_PHY_TYPES {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<DOT11_SUPPORTED_PHY_TYPES>()) == 0 }
    }
}
impl ::core::cmp::Eq for DOT11_SUPPORTED_PHY_TYPES {}
impl ::core::default::Default for DOT11_SUPPORTED_PHY_TYPES {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct DOT11_SUPPORTED_POWER_LEVELS {
    pub uNumOfSupportedPowerLevels: u32,
    pub uTxPowerLevelValues: [u32; 8],
}
impl ::core::marker::Copy for DOT11_SUPPORTED_POWER_LEVELS {}
impl ::core::clone::Clone for DOT11_SUPPORTED_POWER_LEVELS {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for DOT11_SUPPORTED_POWER_LEVELS {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for DOT11_SUPPORTED_POWER_LEVELS {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<DOT11_SUPPORTED_POWER_LEVELS>()) == 0 }
    }
}
impl ::core::cmp::Eq for DOT11_SUPPORTED_POWER_LEVELS {}
impl ::core::default::Default for DOT11_SUPPORTED_POWER_LEVELS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
pub type DOT11_TEMP_TYPE = i32;
pub const dot11_temp_type_unknown: DOT11_TEMP_TYPE = 0i32;
pub const dot11_temp_type_1: DOT11_TEMP_TYPE = 1i32;
pub const dot11_temp_type_2: DOT11_TEMP_TYPE = 2i32;
#[repr(C)]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_NetworkManagement_Ndis"))]
pub struct DOT11_TKIPMIC_FAILURE_PARAMETERS {
    pub Header: super::Ndis::NDIS_OBJECT_HEADER,
    pub bDefaultKeyFailure: super::super::Foundation::BOOLEAN,
    pub uKeyIndex: u32,
    pub PeerMac: [u8; 6],
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_NetworkManagement_Ndis"))]
impl ::core::marker::Copy for DOT11_TKIPMIC_FAILURE_PARAMETERS {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_NetworkManagement_Ndis"))]
impl ::core::clone::Clone for DOT11_TKIPMIC_FAILURE_PARAMETERS {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_NetworkManagement_Ndis"))]
unsafe impl ::windows::core::Abi for DOT11_TKIPMIC_FAILURE_PARAMETERS {
    type Abi = Self;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_NetworkManagement_Ndis"))]
impl ::core::cmp::PartialEq for DOT11_TKIPMIC_FAILURE_PARAMETERS {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<DOT11_TKIPMIC_FAILURE_PARAMETERS>()) == 0 }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_NetworkManagement_Ndis"))]
impl ::core::cmp::Eq for DOT11_TKIPMIC_FAILURE_PARAMETERS {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_NetworkManagement_Ndis"))]
impl ::core::default::Default for DOT11_TKIPMIC_FAILURE_PARAMETERS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
pub const DOT11_TKIPMIC_FAILURE_PARAMETERS_REVISION_1: u32 = 1u32;
#[repr(C)]
pub struct DOT11_UPDATE_IE {
    pub dot11UpdateIEOp: DOT11_UPDATE_IE_OP,
    pub uBufferLength: u32,
    pub ucBuffer: [u8; 1],
}
impl ::core::marker::Copy for DOT11_UPDATE_IE {}
impl ::core::clone::Clone for DOT11_UPDATE_IE {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for DOT11_UPDATE_IE {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for DOT11_UPDATE_IE {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<DOT11_UPDATE_IE>()) == 0 }
    }
}
impl ::core::cmp::Eq for DOT11_UPDATE_IE {}
impl ::core::default::Default for DOT11_UPDATE_IE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
pub type DOT11_UPDATE_IE_OP = i32;
pub const dot11_update_ie_op_create_replace: DOT11_UPDATE_IE_OP = 1i32;
pub const dot11_update_ie_op_delete: DOT11_UPDATE_IE_OP = 2i32;
#[repr(C)]
pub struct DOT11_VENUEINFO {
    pub VenueGroup: u8,
    pub VenueType: u8,
}
impl ::core::marker::Copy for DOT11_VENUEINFO {}
impl ::core::clone::Clone for DOT11_VENUEINFO {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for DOT11_VENUEINFO {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for DOT11_VENUEINFO {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<DOT11_VENUEINFO>()) == 0 }
    }
}
impl ::core::cmp::Eq for DOT11_VENUEINFO {}
impl ::core::default::Default for DOT11_VENUEINFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
pub struct DOT11_VWIFI_ATTRIBUTES {
    pub Header: super::Ndis::NDIS_OBJECT_HEADER,
    pub uTotalNumOfEntries: u32,
    pub Combinations: [DOT11_VWIFI_COMBINATION; 1],
}
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
impl ::core::marker::Copy for DOT11_VWIFI_ATTRIBUTES {}
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
impl ::core::clone::Clone for DOT11_VWIFI_ATTRIBUTES {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
unsafe impl ::windows::core::Abi for DOT11_VWIFI_ATTRIBUTES {
    type Abi = Self;
}
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
impl ::core::cmp::PartialEq for DOT11_VWIFI_ATTRIBUTES {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<DOT11_VWIFI_ATTRIBUTES>()) == 0 }
    }
}
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
impl ::core::cmp::Eq for DOT11_VWIFI_ATTRIBUTES {}
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
impl ::core::default::Default for DOT11_VWIFI_ATTRIBUTES {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
pub const DOT11_VWIFI_ATTRIBUTES_REVISION_1: u32 = 1u32;
#[repr(C)]
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
pub struct DOT11_VWIFI_COMBINATION {
    pub Header: super::Ndis::NDIS_OBJECT_HEADER,
    pub uNumInfrastructure: u32,
    pub uNumAdhoc: u32,
    pub uNumSoftAP: u32,
}
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
impl ::core::marker::Copy for DOT11_VWIFI_COMBINATION {}
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
impl ::core::clone::Clone for DOT11_VWIFI_COMBINATION {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
unsafe impl ::windows::core::Abi for DOT11_VWIFI_COMBINATION {
    type Abi = Self;
}
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
impl ::core::cmp::PartialEq for DOT11_VWIFI_COMBINATION {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<DOT11_VWIFI_COMBINATION>()) == 0 }
    }
}
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
impl ::core::cmp::Eq for DOT11_VWIFI_COMBINATION {}
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
impl ::core::default::Default for DOT11_VWIFI_COMBINATION {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
pub const DOT11_VWIFI_COMBINATION_REVISION_1: u32 = 1u32;
pub const DOT11_VWIFI_COMBINATION_REVISION_2: u32 = 2u32;
pub const DOT11_VWIFI_COMBINATION_REVISION_3: u32 = 3u32;
#[repr(C)]
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
pub struct DOT11_VWIFI_COMBINATION_V2 {
    pub Header: super::Ndis::NDIS_OBJECT_HEADER,
    pub uNumInfrastructure: u32,
    pub uNumAdhoc: u32,
    pub uNumSoftAP: u32,
    pub uNumVirtualStation: u32,
}
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
impl ::core::marker::Copy for DOT11_VWIFI_COMBINATION_V2 {}
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
impl ::core::clone::Clone for DOT11_VWIFI_COMBINATION_V2 {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
unsafe impl ::windows::core::Abi for DOT11_VWIFI_COMBINATION_V2 {
    type Abi = Self;
}
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
impl ::core::cmp::PartialEq for DOT11_VWIFI_COMBINATION_V2 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<DOT11_VWIFI_COMBINATION_V2>()) == 0 }
    }
}
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
impl ::core::cmp::Eq for DOT11_VWIFI_COMBINATION_V2 {}
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
impl ::core::default::Default for DOT11_VWIFI_COMBINATION_V2 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
pub struct DOT11_VWIFI_COMBINATION_V3 {
    pub Header: super::Ndis::NDIS_OBJECT_HEADER,
    pub uNumInfrastructure: u32,
    pub uNumAdhoc: u32,
    pub uNumSoftAP: u32,
    pub uNumVirtualStation: u32,
    pub uNumWFDGroup: u32,
}
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
impl ::core::marker::Copy for DOT11_VWIFI_COMBINATION_V3 {}
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
impl ::core::clone::Clone for DOT11_VWIFI_COMBINATION_V3 {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
unsafe impl ::windows::core::Abi for DOT11_VWIFI_COMBINATION_V3 {
    type Abi = Self;
}
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
impl ::core::cmp::PartialEq for DOT11_VWIFI_COMBINATION_V3 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<DOT11_VWIFI_COMBINATION_V3>()) == 0 }
    }
}
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
impl ::core::cmp::Eq for DOT11_VWIFI_COMBINATION_V3 {}
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
impl ::core::default::Default for DOT11_VWIFI_COMBINATION_V3 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct DOT11_WEP_OFFLOAD {
    pub uReserved: u32,
    pub hOffloadContext: super::super::Foundation::HANDLE,
    pub hOffload: super::super::Foundation::HANDLE,
    pub dot11OffloadType: DOT11_OFFLOAD_TYPE,
    pub dwAlgorithm: u32,
    pub bRowIsOutbound: super::super::Foundation::BOOLEAN,
    pub bUseDefault: super::super::Foundation::BOOLEAN,
    pub uFlags: u32,
    pub ucMacAddress: [u8; 6],
    pub uNumOfRWsOnPeer: u32,
    pub uNumOfRWsOnMe: u32,
    pub dot11IV48Counters: [DOT11_IV48_COUNTER; 16],
    pub usDot11RWBitMaps: [u16; 16],
    pub usKeyLength: u16,
    pub ucKey: [u8; 1],
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for DOT11_WEP_OFFLOAD {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for DOT11_WEP_OFFLOAD {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for DOT11_WEP_OFFLOAD {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for DOT11_WEP_OFFLOAD {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<DOT11_WEP_OFFLOAD>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for DOT11_WEP_OFFLOAD {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for DOT11_WEP_OFFLOAD {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct DOT11_WEP_UPLOAD {
    pub uReserved: u32,
    pub dot11OffloadType: DOT11_OFFLOAD_TYPE,
    pub hOffload: super::super::Foundation::HANDLE,
    pub uNumOfRWsUsed: u32,
    pub dot11IV48Counters: [DOT11_IV48_COUNTER; 16],
    pub usDot11RWBitMaps: [u16; 16],
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for DOT11_WEP_UPLOAD {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for DOT11_WEP_UPLOAD {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for DOT11_WEP_UPLOAD {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for DOT11_WEP_UPLOAD {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<DOT11_WEP_UPLOAD>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for DOT11_WEP_UPLOAD {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for DOT11_WEP_UPLOAD {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
pub struct DOT11_WFD_ADDITIONAL_IE {
    pub Header: super::Ndis::NDIS_OBJECT_HEADER,
    pub uBeaconIEsOffset: u32,
    pub uBeaconIEsLength: u32,
    pub uProbeResponseIEsOffset: u32,
    pub uProbeResponseIEsLength: u32,
    pub uDefaultRequestIEsOffset: u32,
    pub uDefaultRequestIEsLength: u32,
}
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
impl ::core::marker::Copy for DOT11_WFD_ADDITIONAL_IE {}
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
impl ::core::clone::Clone for DOT11_WFD_ADDITIONAL_IE {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
unsafe impl ::windows::core::Abi for DOT11_WFD_ADDITIONAL_IE {
    type Abi = Self;
}
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
impl ::core::cmp::PartialEq for DOT11_WFD_ADDITIONAL_IE {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<DOT11_WFD_ADDITIONAL_IE>()) == 0 }
    }
}
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
impl ::core::cmp::Eq for DOT11_WFD_ADDITIONAL_IE {}
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
impl ::core::default::Default for DOT11_WFD_ADDITIONAL_IE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
pub const DOT11_WFD_ADDITIONAL_IE_REVISION_1: u32 = 1u32;
#[repr(C)]
pub struct DOT11_WFD_ADVERTISED_SERVICE_DESCRIPTOR {
    pub AdvertisementID: u32,
    pub ConfigMethods: u16,
    pub ServiceNameLength: u8,
    pub ServiceName: [u8; 255],
}
impl ::core::marker::Copy for DOT11_WFD_ADVERTISED_SERVICE_DESCRIPTOR {}
impl ::core::clone::Clone for DOT11_WFD_ADVERTISED_SERVICE_DESCRIPTOR {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for DOT11_WFD_ADVERTISED_SERVICE_DESCRIPTOR {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for DOT11_WFD_ADVERTISED_SERVICE_DESCRIPTOR {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<DOT11_WFD_ADVERTISED_SERVICE_DESCRIPTOR>()) == 0 }
    }
}
impl ::core::cmp::Eq for DOT11_WFD_ADVERTISED_SERVICE_DESCRIPTOR {}
impl ::core::default::Default for DOT11_WFD_ADVERTISED_SERVICE_DESCRIPTOR {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct DOT11_WFD_ADVERTISED_SERVICE_LIST {
    pub ServiceCount: u16,
    pub AdvertisedService: [DOT11_WFD_ADVERTISED_SERVICE_DESCRIPTOR; 1],
}
impl ::core::marker::Copy for DOT11_WFD_ADVERTISED_SERVICE_LIST {}
impl ::core::clone::Clone for DOT11_WFD_ADVERTISED_SERVICE_LIST {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for DOT11_WFD_ADVERTISED_SERVICE_LIST {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for DOT11_WFD_ADVERTISED_SERVICE_LIST {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<DOT11_WFD_ADVERTISED_SERVICE_LIST>()) == 0 }
    }
}
impl ::core::cmp::Eq for DOT11_WFD_ADVERTISED_SERVICE_LIST {}
impl ::core::default::Default for DOT11_WFD_ADVERTISED_SERVICE_LIST {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct DOT11_WFD_ADVERTISEMENT_ID {
    pub AdvertisementID: u32,
    pub ServiceAddress: [u8; 6],
}
impl ::core::marker::Copy for DOT11_WFD_ADVERTISEMENT_ID {}
impl ::core::clone::Clone for DOT11_WFD_ADVERTISEMENT_ID {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for DOT11_WFD_ADVERTISEMENT_ID {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for DOT11_WFD_ADVERTISEMENT_ID {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<DOT11_WFD_ADVERTISEMENT_ID>()) == 0 }
    }
}
impl ::core::cmp::Eq for DOT11_WFD_ADVERTISEMENT_ID {}
impl ::core::default::Default for DOT11_WFD_ADVERTISEMENT_ID {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
pub const DOT11_WFD_APS2_SERVICE_TYPE_MAX_LENGTH: u32 = 21u32;
pub const DOT11_WFD_ASP2_INSTANCE_NAME_MAX_LENGTH: u32 = 63u32;
#[repr(C)]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_NetworkManagement_Ndis"))]
pub struct DOT11_WFD_ATTRIBUTES {
    pub Header: super::Ndis::NDIS_OBJECT_HEADER,
    pub uNumConcurrentGORole: u32,
    pub uNumConcurrentClientRole: u32,
    pub WPSVersionsSupported: u32,
    pub bServiceDiscoverySupported: super::super::Foundation::BOOLEAN,
    pub bClientDiscoverabilitySupported: super::super::Foundation::BOOLEAN,
    pub bInfrastructureManagementSupported: super::super::Foundation::BOOLEAN,
    pub uMaxSecondaryDeviceTypeListSize: u32,
    pub DeviceAddress: [u8; 6],
    pub uInterfaceAddressListCount: u32,
    pub pInterfaceAddressList: *mut u8,
    pub uNumSupportedCountryOrRegionStrings: u32,
    pub pSupportedCountryOrRegionStrings: *mut u8,
    pub uDiscoveryFilterListSize: u32,
    pub uGORoleClientTableSize: u32,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_NetworkManagement_Ndis"))]
impl ::core::marker::Copy for DOT11_WFD_ATTRIBUTES {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_NetworkManagement_Ndis"))]
impl ::core::clone::Clone for DOT11_WFD_ATTRIBUTES {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_NetworkManagement_Ndis"))]
unsafe impl ::windows::core::Abi for DOT11_WFD_ATTRIBUTES {
    type Abi = Self;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_NetworkManagement_Ndis"))]
impl ::core::cmp::PartialEq for DOT11_WFD_ATTRIBUTES {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<DOT11_WFD_ATTRIBUTES>()) == 0 }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_NetworkManagement_Ndis"))]
impl ::core::cmp::Eq for DOT11_WFD_ATTRIBUTES {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_NetworkManagement_Ndis"))]
impl ::core::default::Default for DOT11_WFD_ATTRIBUTES {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
pub const DOT11_WFD_ATTRIBUTES_REVISION_1: u32 = 1u32;
#[repr(C)]
pub struct DOT11_WFD_CHANNEL {
    pub CountryRegionString: [u8; 3],
    pub OperatingClass: u8,
    pub ChannelNumber: u8,
}
impl ::core::marker::Copy for DOT11_WFD_CHANNEL {}
impl ::core::clone::Clone for DOT11_WFD_CHANNEL {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for DOT11_WFD_CHANNEL {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for DOT11_WFD_CHANNEL {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<DOT11_WFD_CHANNEL>()) == 0 }
    }
}
impl ::core::cmp::Eq for DOT11_WFD_CHANNEL {}
impl ::core::default::Default for DOT11_WFD_CHANNEL {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct DOT11_WFD_CONFIGURATION_TIMEOUT {
    pub GOTimeout: u8,
    pub ClientTimeout: u8,
}
impl ::core::marker::Copy for DOT11_WFD_CONFIGURATION_TIMEOUT {}
impl ::core::clone::Clone for DOT11_WFD_CONFIGURATION_TIMEOUT {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for DOT11_WFD_CONFIGURATION_TIMEOUT {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for DOT11_WFD_CONFIGURATION_TIMEOUT {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<DOT11_WFD_CONFIGURATION_TIMEOUT>()) == 0 }
    }
}
impl ::core::cmp::Eq for DOT11_WFD_CONFIGURATION_TIMEOUT {}
impl ::core::default::Default for DOT11_WFD_CONFIGURATION_TIMEOUT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
pub const DOT11_WFD_DEVICE_AUTO_AVAILABILITY: u32 = 16u32;
pub const DOT11_WFD_DEVICE_CAPABILITY_CONCURRENT_OPERATION: u32 = 4u32;
#[repr(C)]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_NetworkManagement_Ndis"))]
pub struct DOT11_WFD_DEVICE_CAPABILITY_CONFIG {
    pub Header: super::Ndis::NDIS_OBJECT_HEADER,
    pub bServiceDiscoveryEnabled: super::super::Foundation::BOOLEAN,
    pub bClientDiscoverabilityEnabled: super::super::Foundation::BOOLEAN,
    pub bConcurrentOperationSupported: super::super::Foundation::BOOLEAN,
    pub bInfrastructureManagementEnabled: super::super::Foundation::BOOLEAN,
    pub bDeviceLimitReached: super::super::Foundation::BOOLEAN,
    pub bInvitationProcedureEnabled: super::super::Foundation::BOOLEAN,
    pub WPSVersionsEnabled: u32,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_NetworkManagement_Ndis"))]
impl ::core::marker::Copy for DOT11_WFD_DEVICE_CAPABILITY_CONFIG {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_NetworkManagement_Ndis"))]
impl ::core::clone::Clone for DOT11_WFD_DEVICE_CAPABILITY_CONFIG {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_NetworkManagement_Ndis"))]
unsafe impl ::windows::core::Abi for DOT11_WFD_DEVICE_CAPABILITY_CONFIG {
    type Abi = Self;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_NetworkManagement_Ndis"))]
impl ::core::cmp::PartialEq for DOT11_WFD_DEVICE_CAPABILITY_CONFIG {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<DOT11_WFD_DEVICE_CAPABILITY_CONFIG>()) == 0 }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_NetworkManagement_Ndis"))]
impl ::core::cmp::Eq for DOT11_WFD_DEVICE_CAPABILITY_CONFIG {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_NetworkManagement_Ndis"))]
impl ::core::default::Default for DOT11_WFD_DEVICE_CAPABILITY_CONFIG {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
pub const DOT11_WFD_DEVICE_CAPABILITY_CONFIG_REVISION_1: u32 = 1u32;
pub const DOT11_WFD_DEVICE_CAPABILITY_P2P_CLIENT_DISCOVERABILITY: u32 = 2u32;
pub const DOT11_WFD_DEVICE_CAPABILITY_P2P_DEVICE_LIMIT: u32 = 16u32;
pub const DOT11_WFD_DEVICE_CAPABILITY_P2P_INFRASTRUCTURE_MANAGED: u32 = 8u32;
pub const DOT11_WFD_DEVICE_CAPABILITY_P2P_INVITATION_PROCEDURE: u32 = 32u32;
pub const DOT11_WFD_DEVICE_CAPABILITY_RESERVED_6: u32 = 64u32;
pub const DOT11_WFD_DEVICE_CAPABILITY_RESERVED_7: u32 = 128u32;
pub const DOT11_WFD_DEVICE_CAPABILITY_SERVICE_DISCOVERY: u32 = 1u32;
#[repr(C)]
pub struct DOT11_WFD_DEVICE_ENTRY {
    pub uPhyId: u32,
    pub PhySpecificInfo: DOT11_BSS_ENTRY_PHY_SPECIFIC_INFO,
    pub dot11BSSID: [u8; 6],
    pub dot11BSSType: DOT11_BSS_TYPE,
    pub TransmitterAddress: [u8; 6],
    pub lRSSI: i32,
    pub uLinkQuality: u32,
    pub usBeaconPeriod: u16,
    pub ullTimestamp: u64,
    pub ullBeaconHostTimestamp: u64,
    pub ullProbeResponseHostTimestamp: u64,
    pub usCapabilityInformation: u16,
    pub uBeaconIEsOffset: u32,
    pub uBeaconIEsLength: u32,
    pub uProbeResponseIEsOffset: u32,
    pub uProbeResponseIEsLength: u32,
}
impl ::core::marker::Copy for DOT11_WFD_DEVICE_ENTRY {}
impl ::core::clone::Clone for DOT11_WFD_DEVICE_ENTRY {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for DOT11_WFD_DEVICE_ENTRY {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for DOT11_WFD_DEVICE_ENTRY {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<DOT11_WFD_DEVICE_ENTRY>()) == 0 }
    }
}
impl ::core::cmp::Eq for DOT11_WFD_DEVICE_ENTRY {}
impl ::core::default::Default for DOT11_WFD_DEVICE_ENTRY {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
pub const DOT11_WFD_DEVICE_HIGH_AVAILABILITY: u32 = 24u32;
#[repr(C)]
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
pub struct DOT11_WFD_DEVICE_INFO {
    pub Header: super::Ndis::NDIS_OBJECT_HEADER,
    pub DeviceAddress: [u8; 6],
    pub ConfigMethods: u16,
    pub PrimaryDeviceType: DOT11_WFD_DEVICE_TYPE,
    pub DeviceName: DOT11_WPS_DEVICE_NAME,
}
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
impl ::core::marker::Copy for DOT11_WFD_DEVICE_INFO {}
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
impl ::core::clone::Clone for DOT11_WFD_DEVICE_INFO {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
unsafe impl ::windows::core::Abi for DOT11_WFD_DEVICE_INFO {
    type Abi = Self;
}
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
impl ::core::cmp::PartialEq for DOT11_WFD_DEVICE_INFO {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<DOT11_WFD_DEVICE_INFO>()) == 0 }
    }
}
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
impl ::core::cmp::Eq for DOT11_WFD_DEVICE_INFO {}
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
impl ::core::default::Default for DOT11_WFD_DEVICE_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
pub const DOT11_WFD_DEVICE_INFO_REVISION_1: u32 = 1u32;
#[repr(C)]
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
pub struct DOT11_WFD_DEVICE_LISTEN_CHANNEL {
    pub Header: super::Ndis::NDIS_OBJECT_HEADER,
    pub ChannelNumber: u8,
}
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
impl ::core::marker::Copy for DOT11_WFD_DEVICE_LISTEN_CHANNEL {}
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
impl ::core::clone::Clone for DOT11_WFD_DEVICE_LISTEN_CHANNEL {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
unsafe impl ::windows::core::Abi for DOT11_WFD_DEVICE_LISTEN_CHANNEL {
    type Abi = Self;
}
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
impl ::core::cmp::PartialEq for DOT11_WFD_DEVICE_LISTEN_CHANNEL {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<DOT11_WFD_DEVICE_LISTEN_CHANNEL>()) == 0 }
    }
}
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
impl ::core::cmp::Eq for DOT11_WFD_DEVICE_LISTEN_CHANNEL {}
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
impl ::core::default::Default for DOT11_WFD_DEVICE_LISTEN_CHANNEL {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
pub const DOT11_WFD_DEVICE_LISTEN_CHANNEL_REVISION_1: u32 = 1u32;
pub const DOT11_WFD_DEVICE_NOT_DISCOVERABLE: u32 = 0u32;
#[repr(C)]
pub struct DOT11_WFD_DEVICE_TYPE {
    pub CategoryID: u16,
    pub SubCategoryID: u16,
    pub OUI: [u8; 4],
}
impl ::core::marker::Copy for DOT11_WFD_DEVICE_TYPE {}
impl ::core::clone::Clone for DOT11_WFD_DEVICE_TYPE {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for DOT11_WFD_DEVICE_TYPE {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for DOT11_WFD_DEVICE_TYPE {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<DOT11_WFD_DEVICE_TYPE>()) == 0 }
    }
}
impl ::core::cmp::Eq for DOT11_WFD_DEVICE_TYPE {}
impl ::core::default::Default for DOT11_WFD_DEVICE_TYPE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
pub const DOT11_WFD_DISCOVER_COMPLETE_MAX_LIST_SIZE: u32 = 128u32;
#[repr(C)]
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
pub struct DOT11_WFD_DISCOVER_COMPLETE_PARAMETERS {
    pub Header: super::Ndis::NDIS_OBJECT_HEADER,
    pub Status: i32,
    pub uNumOfEntries: u32,
    pub uTotalNumOfEntries: u32,
    pub uListOffset: u32,
    pub uListLength: u32,
}
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
impl ::core::marker::Copy for DOT11_WFD_DISCOVER_COMPLETE_PARAMETERS {}
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
impl ::core::clone::Clone for DOT11_WFD_DISCOVER_COMPLETE_PARAMETERS {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
unsafe impl ::windows::core::Abi for DOT11_WFD_DISCOVER_COMPLETE_PARAMETERS {
    type Abi = Self;
}
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
impl ::core::cmp::PartialEq for DOT11_WFD_DISCOVER_COMPLETE_PARAMETERS {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<DOT11_WFD_DISCOVER_COMPLETE_PARAMETERS>()) == 0 }
    }
}
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
impl ::core::cmp::Eq for DOT11_WFD_DISCOVER_COMPLETE_PARAMETERS {}
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
impl ::core::default::Default for DOT11_WFD_DISCOVER_COMPLETE_PARAMETERS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
pub const DOT11_WFD_DISCOVER_COMPLETE_PARAMETERS_REVISION_1: u32 = 1u32;
#[repr(C)]
pub struct DOT11_WFD_DISCOVER_DEVICE_FILTER {
    pub DeviceID: [u8; 6],
    pub ucBitmask: u8,
    pub GroupSSID: DOT11_SSID,
}
impl ::core::marker::Copy for DOT11_WFD_DISCOVER_DEVICE_FILTER {}
impl ::core::clone::Clone for DOT11_WFD_DISCOVER_DEVICE_FILTER {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for DOT11_WFD_DISCOVER_DEVICE_FILTER {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for DOT11_WFD_DISCOVER_DEVICE_FILTER {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<DOT11_WFD_DISCOVER_DEVICE_FILTER>()) == 0 }
    }
}
impl ::core::cmp::Eq for DOT11_WFD_DISCOVER_DEVICE_FILTER {}
impl ::core::default::Default for DOT11_WFD_DISCOVER_DEVICE_FILTER {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_NetworkManagement_Ndis"))]
pub struct DOT11_WFD_DISCOVER_REQUEST {
    pub Header: super::Ndis::NDIS_OBJECT_HEADER,
    pub DiscoverType: DOT11_WFD_DISCOVER_TYPE,
    pub ScanType: DOT11_WFD_SCAN_TYPE,
    pub uDiscoverTimeout: u32,
    pub uDeviceFilterListOffset: u32,
    pub uNumDeviceFilters: u32,
    pub uIEsOffset: u32,
    pub uIEsLength: u32,
    pub bForceScanLegacyNetworks: super::super::Foundation::BOOLEAN,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_NetworkManagement_Ndis"))]
impl ::core::marker::Copy for DOT11_WFD_DISCOVER_REQUEST {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_NetworkManagement_Ndis"))]
impl ::core::clone::Clone for DOT11_WFD_DISCOVER_REQUEST {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_NetworkManagement_Ndis"))]
unsafe impl ::windows::core::Abi for DOT11_WFD_DISCOVER_REQUEST {
    type Abi = Self;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_NetworkManagement_Ndis"))]
impl ::core::cmp::PartialEq for DOT11_WFD_DISCOVER_REQUEST {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<DOT11_WFD_DISCOVER_REQUEST>()) == 0 }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_NetworkManagement_Ndis"))]
impl ::core::cmp::Eq for DOT11_WFD_DISCOVER_REQUEST {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_NetworkManagement_Ndis"))]
impl ::core::default::Default for DOT11_WFD_DISCOVER_REQUEST {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
pub const DOT11_WFD_DISCOVER_REQUEST_REVISION_1: u32 = 1u32;
pub type DOT11_WFD_DISCOVER_TYPE = i32;
pub const dot11_wfd_discover_type_scan_only: DOT11_WFD_DISCOVER_TYPE = 1i32;
pub const dot11_wfd_discover_type_find_only: DOT11_WFD_DISCOVER_TYPE = 2i32;
pub const dot11_wfd_discover_type_auto: DOT11_WFD_DISCOVER_TYPE = 3i32;
pub const dot11_wfd_discover_type_scan_social_channels: DOT11_WFD_DISCOVER_TYPE = 4i32;
pub const dot11_wfd_discover_type_forced: DOT11_WFD_DISCOVER_TYPE = -2147483648i32;
#[repr(C)]
pub struct DOT11_WFD_GO_INTENT {
    pub _bitfield: u8,
}
impl ::core::marker::Copy for DOT11_WFD_GO_INTENT {}
impl ::core::clone::Clone for DOT11_WFD_GO_INTENT {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for DOT11_WFD_GO_INTENT {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for DOT11_WFD_GO_INTENT {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<DOT11_WFD_GO_INTENT>()) == 0 }
    }
}
impl ::core::cmp::Eq for DOT11_WFD_GO_INTENT {}
impl ::core::default::Default for DOT11_WFD_GO_INTENT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
pub const DOT11_WFD_GROUP_CAPABILITY_CROSS_CONNECTION_SUPPORTED: u32 = 16u32;
pub const DOT11_WFD_GROUP_CAPABILITY_EAPOL_KEY_IP_ADDRESS_ALLOCATION_SUPPORTED: u32 = 128u32;
pub const DOT11_WFD_GROUP_CAPABILITY_GROUP_LIMIT_REACHED: u32 = 4u32;
pub const DOT11_WFD_GROUP_CAPABILITY_GROUP_OWNER: u32 = 1u32;
pub const DOT11_WFD_GROUP_CAPABILITY_INTRABSS_DISTRIBUTION_SUPPORTED: u32 = 8u32;
pub const DOT11_WFD_GROUP_CAPABILITY_IN_GROUP_FORMATION: u32 = 64u32;
pub const DOT11_WFD_GROUP_CAPABILITY_NONE: u32 = 0u32;
pub const DOT11_WFD_GROUP_CAPABILITY_PERSISTENT_GROUP: u32 = 2u32;
pub const DOT11_WFD_GROUP_CAPABILITY_PERSISTENT_RECONNECT_SUPPORTED: u32 = 32u32;
pub const DOT11_WFD_GROUP_CAPABILITY_RESERVED_7: u32 = 128u32;
#[repr(C)]
pub struct DOT11_WFD_GROUP_ID {
    pub DeviceAddress: [u8; 6],
    pub SSID: DOT11_SSID,
}
impl ::core::marker::Copy for DOT11_WFD_GROUP_ID {}
impl ::core::clone::Clone for DOT11_WFD_GROUP_ID {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for DOT11_WFD_GROUP_ID {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for DOT11_WFD_GROUP_ID {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<DOT11_WFD_GROUP_ID>()) == 0 }
    }
}
impl ::core::cmp::Eq for DOT11_WFD_GROUP_ID {}
impl ::core::default::Default for DOT11_WFD_GROUP_ID {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_NetworkManagement_Ndis"))]
pub struct DOT11_WFD_GROUP_JOIN_PARAMETERS {
    pub Header: super::Ndis::NDIS_OBJECT_HEADER,
    pub GOOperatingChannel: DOT11_WFD_CHANNEL,
    pub GOConfigTime: u32,
    pub bInGroupFormation: super::super::Foundation::BOOLEAN,
    pub bWaitForWPSReady: super::super::Foundation::BOOLEAN,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_NetworkManagement_Ndis"))]
impl ::core::marker::Copy for DOT11_WFD_GROUP_JOIN_PARAMETERS {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_NetworkManagement_Ndis"))]
impl ::core::clone::Clone for DOT11_WFD_GROUP_JOIN_PARAMETERS {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_NetworkManagement_Ndis"))]
unsafe impl ::windows::core::Abi for DOT11_WFD_GROUP_JOIN_PARAMETERS {
    type Abi = Self;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_NetworkManagement_Ndis"))]
impl ::core::cmp::PartialEq for DOT11_WFD_GROUP_JOIN_PARAMETERS {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<DOT11_WFD_GROUP_JOIN_PARAMETERS>()) == 0 }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_NetworkManagement_Ndis"))]
impl ::core::cmp::Eq for DOT11_WFD_GROUP_JOIN_PARAMETERS {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_NetworkManagement_Ndis"))]
impl ::core::default::Default for DOT11_WFD_GROUP_JOIN_PARAMETERS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
pub const DOT11_WFD_GROUP_JOIN_PARAMETERS_REVISION_1: u32 = 1u32;
#[repr(C)]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_NetworkManagement_Ndis"))]
pub struct DOT11_WFD_GROUP_OWNER_CAPABILITY_CONFIG {
    pub Header: super::Ndis::NDIS_OBJECT_HEADER,
    pub bPersistentGroupEnabled: super::super::Foundation::BOOLEAN,
    pub bIntraBSSDistributionSupported: super::super::Foundation::BOOLEAN,
    pub bCrossConnectionSupported: super::super::Foundation::BOOLEAN,
    pub bPersistentReconnectSupported: super::super::Foundation::BOOLEAN,
    pub bGroupFormationEnabled: super::super::Foundation::BOOLEAN,
    pub uMaximumGroupLimit: u32,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_NetworkManagement_Ndis"))]
impl ::core::marker::Copy for DOT11_WFD_GROUP_OWNER_CAPABILITY_CONFIG {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_NetworkManagement_Ndis"))]
impl ::core::clone::Clone for DOT11_WFD_GROUP_OWNER_CAPABILITY_CONFIG {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_NetworkManagement_Ndis"))]
unsafe impl ::windows::core::Abi for DOT11_WFD_GROUP_OWNER_CAPABILITY_CONFIG {
    type Abi = Self;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_NetworkManagement_Ndis"))]
impl ::core::cmp::PartialEq for DOT11_WFD_GROUP_OWNER_CAPABILITY_CONFIG {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<DOT11_WFD_GROUP_OWNER_CAPABILITY_CONFIG>()) == 0 }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_NetworkManagement_Ndis"))]
impl ::core::cmp::Eq for DOT11_WFD_GROUP_OWNER_CAPABILITY_CONFIG {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_NetworkManagement_Ndis"))]
impl ::core::default::Default for DOT11_WFD_GROUP_OWNER_CAPABILITY_CONFIG {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
pub const DOT11_WFD_GROUP_OWNER_CAPABILITY_CONFIG_REVISION_1: u32 = 1u32;
pub const DOT11_WFD_GROUP_OWNER_CAPABILITY_CONFIG_REVISION_2: u32 = 2u32;
#[repr(C)]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_NetworkManagement_Ndis"))]
pub struct DOT11_WFD_GROUP_OWNER_CAPABILITY_CONFIG_V2 {
    pub Header: super::Ndis::NDIS_OBJECT_HEADER,
    pub bPersistentGroupEnabled: super::super::Foundation::BOOLEAN,
    pub bIntraBSSDistributionSupported: super::super::Foundation::BOOLEAN,
    pub bCrossConnectionSupported: super::super::Foundation::BOOLEAN,
    pub bPersistentReconnectSupported: super::super::Foundation::BOOLEAN,
    pub bGroupFormationEnabled: super::super::Foundation::BOOLEAN,
    pub uMaximumGroupLimit: u32,
    pub bEapolKeyIpAddressAllocationSupported: super::super::Foundation::BOOLEAN,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_NetworkManagement_Ndis"))]
impl ::core::marker::Copy for DOT11_WFD_GROUP_OWNER_CAPABILITY_CONFIG_V2 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_NetworkManagement_Ndis"))]
impl ::core::clone::Clone for DOT11_WFD_GROUP_OWNER_CAPABILITY_CONFIG_V2 {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_NetworkManagement_Ndis"))]
unsafe impl ::windows::core::Abi for DOT11_WFD_GROUP_OWNER_CAPABILITY_CONFIG_V2 {
    type Abi = Self;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_NetworkManagement_Ndis"))]
impl ::core::cmp::PartialEq for DOT11_WFD_GROUP_OWNER_CAPABILITY_CONFIG_V2 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<DOT11_WFD_GROUP_OWNER_CAPABILITY_CONFIG_V2>()) == 0 }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_NetworkManagement_Ndis"))]
impl ::core::cmp::Eq for DOT11_WFD_GROUP_OWNER_CAPABILITY_CONFIG_V2 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_NetworkManagement_Ndis"))]
impl ::core::default::Default for DOT11_WFD_GROUP_OWNER_CAPABILITY_CONFIG_V2 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
pub struct DOT11_WFD_GROUP_START_PARAMETERS {
    pub Header: super::Ndis::NDIS_OBJECT_HEADER,
    pub AdvertisedOperatingChannel: DOT11_WFD_CHANNEL,
}
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
impl ::core::marker::Copy for DOT11_WFD_GROUP_START_PARAMETERS {}
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
impl ::core::clone::Clone for DOT11_WFD_GROUP_START_PARAMETERS {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
unsafe impl ::windows::core::Abi for DOT11_WFD_GROUP_START_PARAMETERS {
    type Abi = Self;
}
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
impl ::core::cmp::PartialEq for DOT11_WFD_GROUP_START_PARAMETERS {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<DOT11_WFD_GROUP_START_PARAMETERS>()) == 0 }
    }
}
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
impl ::core::cmp::Eq for DOT11_WFD_GROUP_START_PARAMETERS {}
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
impl ::core::default::Default for DOT11_WFD_GROUP_START_PARAMETERS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
pub const DOT11_WFD_GROUP_START_PARAMETERS_REVISION_1: u32 = 1u32;
#[repr(C)]
pub struct DOT11_WFD_INVITATION_FLAGS {
    pub _bitfield: u8,
}
impl ::core::marker::Copy for DOT11_WFD_INVITATION_FLAGS {}
impl ::core::clone::Clone for DOT11_WFD_INVITATION_FLAGS {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for DOT11_WFD_INVITATION_FLAGS {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for DOT11_WFD_INVITATION_FLAGS {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<DOT11_WFD_INVITATION_FLAGS>()) == 0 }
    }
}
impl ::core::cmp::Eq for DOT11_WFD_INVITATION_FLAGS {}
impl ::core::default::Default for DOT11_WFD_INVITATION_FLAGS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
pub const DOT11_WFD_MINOR_REASON_DISASSOCIATED_FROM_WLAN_CROSS_CONNECTION_POLICY: u32 = 1u32;
pub const DOT11_WFD_MINOR_REASON_DISASSOCIATED_INFRASTRUCTURE_MANAGED_POLICY: u32 = 4u32;
pub const DOT11_WFD_MINOR_REASON_DISASSOCIATED_NOT_MANAGED_INFRASTRUCTURE_CAPABLE: u32 = 2u32;
pub const DOT11_WFD_MINOR_REASON_DISASSOCIATED_WFD_COEXISTENCE_POLICY: u32 = 3u32;
pub const DOT11_WFD_MINOR_REASON_SUCCESS: u32 = 0u32;
pub type DOT11_WFD_SCAN_TYPE = i32;
pub const dot11_wfd_scan_type_active: DOT11_WFD_SCAN_TYPE = 1i32;
pub const dot11_wfd_scan_type_passive: DOT11_WFD_SCAN_TYPE = 2i32;
pub const dot11_wfd_scan_type_auto: DOT11_WFD_SCAN_TYPE = 3i32;
#[repr(C)]
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
pub struct DOT11_WFD_SECONDARY_DEVICE_TYPE_LIST {
    pub Header: super::Ndis::NDIS_OBJECT_HEADER,
    pub uNumOfEntries: u32,
    pub uTotalNumOfEntries: u32,
    pub SecondaryDeviceTypes: [DOT11_WFD_DEVICE_TYPE; 1],
}
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
impl ::core::marker::Copy for DOT11_WFD_SECONDARY_DEVICE_TYPE_LIST {}
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
impl ::core::clone::Clone for DOT11_WFD_SECONDARY_DEVICE_TYPE_LIST {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
unsafe impl ::windows::core::Abi for DOT11_WFD_SECONDARY_DEVICE_TYPE_LIST {
    type Abi = Self;
}
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
impl ::core::cmp::PartialEq for DOT11_WFD_SECONDARY_DEVICE_TYPE_LIST {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<DOT11_WFD_SECONDARY_DEVICE_TYPE_LIST>()) == 0 }
    }
}
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
impl ::core::cmp::Eq for DOT11_WFD_SECONDARY_DEVICE_TYPE_LIST {}
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
impl ::core::default::Default for DOT11_WFD_SECONDARY_DEVICE_TYPE_LIST {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
pub const DOT11_WFD_SECONDARY_DEVICE_TYPE_LIST_REVISION_1: u32 = 1u32;
#[repr(C)]
pub struct DOT11_WFD_SERVICE_HASH_LIST {
    pub ServiceHashCount: u16,
    pub ServiceHash: [u8; 6],
}
impl ::core::marker::Copy for DOT11_WFD_SERVICE_HASH_LIST {}
impl ::core::clone::Clone for DOT11_WFD_SERVICE_HASH_LIST {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for DOT11_WFD_SERVICE_HASH_LIST {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for DOT11_WFD_SERVICE_HASH_LIST {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<DOT11_WFD_SERVICE_HASH_LIST>()) == 0 }
    }
}
impl ::core::cmp::Eq for DOT11_WFD_SERVICE_HASH_LIST {}
impl ::core::default::Default for DOT11_WFD_SERVICE_HASH_LIST {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
pub const DOT11_WFD_SERVICE_INFORMATION_MAX_LENGTH: u32 = 65535u32;
pub const DOT11_WFD_SERVICE_NAME_MAX_LENGTH: u32 = 255u32;
#[repr(C)]
pub struct DOT11_WFD_SESSION_ID {
    pub SessionID: u32,
    pub SessionAddress: [u8; 6],
}
impl ::core::marker::Copy for DOT11_WFD_SESSION_ID {}
impl ::core::clone::Clone for DOT11_WFD_SESSION_ID {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for DOT11_WFD_SESSION_ID {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for DOT11_WFD_SESSION_ID {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<DOT11_WFD_SESSION_ID>()) == 0 }
    }
}
impl ::core::cmp::Eq for DOT11_WFD_SESSION_ID {}
impl ::core::default::Default for DOT11_WFD_SESSION_ID {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct DOT11_WFD_SESSION_INFO {
    pub uSessionInfoLength: u16,
    pub ucSessionInfo: [u8; 144],
}
impl ::core::marker::Copy for DOT11_WFD_SESSION_INFO {}
impl ::core::clone::Clone for DOT11_WFD_SESSION_INFO {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for DOT11_WFD_SESSION_INFO {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for DOT11_WFD_SESSION_INFO {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<DOT11_WFD_SESSION_INFO>()) == 0 }
    }
}
impl ::core::cmp::Eq for DOT11_WFD_SESSION_INFO {}
impl ::core::default::Default for DOT11_WFD_SESSION_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
pub const DOT11_WFD_SESSION_INFO_MAX_LENGTH: u32 = 144u32;
pub const DOT11_WFD_STATUS_FAILED_INCOMPATIBLE_PARAMETERS: u32 = 2u32;
pub const DOT11_WFD_STATUS_FAILED_INCOMPATIBLE_PROVISIONING_METHOD: u32 = 10u32;
pub const DOT11_WFD_STATUS_FAILED_INFORMATION_IS_UNAVAILABLE: u32 = 1u32;
pub const DOT11_WFD_STATUS_FAILED_INVALID_PARAMETERS: u32 = 4u32;
pub const DOT11_WFD_STATUS_FAILED_LIMIT_REACHED: u32 = 3u32;
pub const DOT11_WFD_STATUS_FAILED_MATCHING_MAX_INTENT: u32 = 9u32;
pub const DOT11_WFD_STATUS_FAILED_NO_COMMON_CHANNELS: u32 = 7u32;
pub const DOT11_WFD_STATUS_FAILED_PREVIOUS_PROTOCOL_ERROR: u32 = 6u32;
pub const DOT11_WFD_STATUS_FAILED_REJECTED_BY_USER: u32 = 11u32;
pub const DOT11_WFD_STATUS_FAILED_UNABLE_TO_ACCOMODATE_REQUEST: u32 = 5u32;
pub const DOT11_WFD_STATUS_FAILED_UNKNOWN_WFD_GROUP: u32 = 8u32;
pub const DOT11_WFD_STATUS_SUCCESS: u32 = 0u32;
pub const DOT11_WFD_STATUS_SUCCESS_ACCEPTED_BY_USER: u32 = 12u32;
#[repr(C)]
pub struct DOT11_WME_AC_PARAMETERS {
    pub ucAccessCategoryIndex: u8,
    pub ucAIFSN: u8,
    pub ucECWmin: u8,
    pub ucECWmax: u8,
    pub usTXOPLimit: u16,
}
impl ::core::marker::Copy for DOT11_WME_AC_PARAMETERS {}
impl ::core::clone::Clone for DOT11_WME_AC_PARAMETERS {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for DOT11_WME_AC_PARAMETERS {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for DOT11_WME_AC_PARAMETERS {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<DOT11_WME_AC_PARAMETERS>()) == 0 }
    }
}
impl ::core::cmp::Eq for DOT11_WME_AC_PARAMETERS {}
impl ::core::default::Default for DOT11_WME_AC_PARAMETERS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
pub const DOT11_WME_PACKET: u32 = 256u32;
#[repr(C)]
pub struct DOT11_WME_UPDATE_IE {
    pub uParamElemMinBeaconIntervals: u32,
    pub uWMEInfoElemOffset: u32,
    pub uWMEInfoElemLength: u32,
    pub uWMEParamElemOffset: u32,
    pub uWMEParamElemLength: u32,
    pub ucBuffer: [u8; 1],
}
impl ::core::marker::Copy for DOT11_WME_UPDATE_IE {}
impl ::core::clone::Clone for DOT11_WME_UPDATE_IE {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for DOT11_WME_UPDATE_IE {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for DOT11_WME_UPDATE_IE {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<DOT11_WME_UPDATE_IE>()) == 0 }
    }
}
impl ::core::cmp::Eq for DOT11_WME_UPDATE_IE {}
impl ::core::default::Default for DOT11_WME_UPDATE_IE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct DOT11_WPA_TSC {
    pub uReserved: u32,
    pub dot11OffloadType: DOT11_OFFLOAD_TYPE,
    pub hOffload: super::super::Foundation::HANDLE,
    pub dot11IV48Counter: DOT11_IV48_COUNTER,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for DOT11_WPA_TSC {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for DOT11_WPA_TSC {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for DOT11_WPA_TSC {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for DOT11_WPA_TSC {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<DOT11_WPA_TSC>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for DOT11_WPA_TSC {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for DOT11_WPA_TSC {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
pub type DOT11_WPS_CONFIG_METHOD = i32;
pub const DOT11_WPS_CONFIG_METHOD_NULL: DOT11_WPS_CONFIG_METHOD = 0i32;
pub const DOT11_WPS_CONFIG_METHOD_DISPLAY: DOT11_WPS_CONFIG_METHOD = 8i32;
pub const DOT11_WPS_CONFIG_METHOD_NFC_TAG: DOT11_WPS_CONFIG_METHOD = 32i32;
pub const DOT11_WPS_CONFIG_METHOD_NFC_INTERFACE: DOT11_WPS_CONFIG_METHOD = 64i32;
pub const DOT11_WPS_CONFIG_METHOD_PUSHBUTTON: DOT11_WPS_CONFIG_METHOD = 128i32;
pub const DOT11_WPS_CONFIG_METHOD_KEYPAD: DOT11_WPS_CONFIG_METHOD = 256i32;
pub const DOT11_WPS_CONFIG_METHOD_WFDS_DEFAULT: DOT11_WPS_CONFIG_METHOD = 4096i32;
#[repr(C)]
pub struct DOT11_WPS_DEVICE_NAME {
    pub uDeviceNameLength: u32,
    pub ucDeviceName: [u8; 32],
}
impl ::core::marker::Copy for DOT11_WPS_DEVICE_NAME {}
impl ::core::clone::Clone for DOT11_WPS_DEVICE_NAME {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for DOT11_WPS_DEVICE_NAME {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for DOT11_WPS_DEVICE_NAME {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<DOT11_WPS_DEVICE_NAME>()) == 0 }
    }
}
impl ::core::cmp::Eq for DOT11_WPS_DEVICE_NAME {}
impl ::core::default::Default for DOT11_WPS_DEVICE_NAME {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
pub const DOT11_WPS_DEVICE_NAME_MAX_LENGTH: u32 = 32u32;
pub type DOT11_WPS_DEVICE_PASSWORD_ID = i32;
pub const DOT11_WPS_PASSWORD_ID_DEFAULT: DOT11_WPS_DEVICE_PASSWORD_ID = 0i32;
pub const DOT11_WPS_PASSWORD_ID_USER_SPECIFIED: DOT11_WPS_DEVICE_PASSWORD_ID = 1i32;
pub const DOT11_WPS_PASSWORD_ID_MACHINE_SPECIFIED: DOT11_WPS_DEVICE_PASSWORD_ID = 2i32;
pub const DOT11_WPS_PASSWORD_ID_REKEY: DOT11_WPS_DEVICE_PASSWORD_ID = 3i32;
pub const DOT11_WPS_PASSWORD_ID_PUSHBUTTON: DOT11_WPS_DEVICE_PASSWORD_ID = 4i32;
pub const DOT11_WPS_PASSWORD_ID_REGISTRAR_SPECIFIED: DOT11_WPS_DEVICE_PASSWORD_ID = 5i32;
pub const DOT11_WPS_PASSWORD_ID_NFC_CONNECTION_HANDOVER: DOT11_WPS_DEVICE_PASSWORD_ID = 7i32;
pub const DOT11_WPS_PASSWORD_ID_WFD_SERVICES: DOT11_WPS_DEVICE_PASSWORD_ID = 8i32;
pub const DOT11_WPS_PASSWORD_ID_OOB_RANGE_MIN: DOT11_WPS_DEVICE_PASSWORD_ID = 16i32;
pub const DOT11_WPS_PASSWORD_ID_OOB_RANGE_MAX: DOT11_WPS_DEVICE_PASSWORD_ID = 65535i32;
pub const DOT11_WPS_MAX_MODEL_NAME_LENGTH: u32 = 32u32;
pub const DOT11_WPS_MAX_MODEL_NUMBER_LENGTH: u32 = 32u32;
pub const DOT11_WPS_MAX_PASSKEY_LENGTH: u32 = 8u32;
pub const DOT11_WPS_VERSION_1_0: u32 = 1u32;
pub const DOT11_WPS_VERSION_2_0: u32 = 2u32;
pub const DevProp_PciDevice_AcsCompatibleUpHierarchy_Enhanced: u32 = 4u32;
pub const DevProp_PciDevice_AcsCompatibleUpHierarchy_NoP2PSupported: u32 = 2u32;
pub const DevProp_PciDevice_AcsCompatibleUpHierarchy_NotSupported: u32 = 0u32;
pub const DevProp_PciDevice_AcsCompatibleUpHierarchy_SingleFunctionSupported: u32 = 1u32;
pub const DevProp_PciDevice_AcsCompatibleUpHierarchy_Supported: u32 = 3u32;
pub const DevProp_PciDevice_AcsSupport_Missing: u32 = 2u32;
pub const DevProp_PciDevice_AcsSupport_NotNeeded: u32 = 1u32;
pub const DevProp_PciDevice_AcsSupport_Present: u32 = 0u32;
pub const DevProp_PciDevice_BridgeType_PciConventional: u32 = 6u32;
pub const DevProp_PciDevice_BridgeType_PciExpressDownstreamSwitchPort: u32 = 10u32;
pub const DevProp_PciDevice_BridgeType_PciExpressEventCollector: u32 = 14u32;
pub const DevProp_PciDevice_BridgeType_PciExpressRootPort: u32 = 8u32;
pub const DevProp_PciDevice_BridgeType_PciExpressToPciXBridge: u32 = 11u32;
pub const DevProp_PciDevice_BridgeType_PciExpressTreatedAsPci: u32 = 13u32;
pub const DevProp_PciDevice_BridgeType_PciExpressUpstreamSwitchPort: u32 = 9u32;
pub const DevProp_PciDevice_BridgeType_PciX: u32 = 7u32;
pub const DevProp_PciDevice_BridgeType_PciXToExpressBridge: u32 = 12u32;
pub const DevProp_PciDevice_CurrentSpeedAndMode_PciX_Mode1_100Mhz: u32 = 2u32;
pub const DevProp_PciDevice_CurrentSpeedAndMode_PciX_Mode1_133MHZ: u32 = 3u32;
pub const DevProp_PciDevice_CurrentSpeedAndMode_PciX_Mode1_66Mhz: u32 = 1u32;
pub const DevProp_PciDevice_CurrentSpeedAndMode_PciX_Mode1_ECC_100Mhz: u32 = 6u32;
pub const DevProp_PciDevice_CurrentSpeedAndMode_PciX_Mode1_ECC_133Mhz: u32 = 7u32;
pub const DevProp_PciDevice_CurrentSpeedAndMode_PciX_Mode1_ECC_66Mhz: u32 = 5u32;
pub const DevProp_PciDevice_CurrentSpeedAndMode_PciX_Mode2_266_100MHz: u32 = 10u32;
pub const DevProp_PciDevice_CurrentSpeedAndMode_PciX_Mode2_266_133MHz: u32 = 11u32;
pub const DevProp_PciDevice_CurrentSpeedAndMode_PciX_Mode2_266_66MHz: u32 = 9u32;
pub const DevProp_PciDevice_CurrentSpeedAndMode_PciX_Mode2_533_100MHz: u32 = 14u32;
pub const DevProp_PciDevice_CurrentSpeedAndMode_PciX_Mode2_533_133MHz: u32 = 15u32;
pub const DevProp_PciDevice_CurrentSpeedAndMode_PciX_Mode2_533_66MHz: u32 = 13u32;
pub const DevProp_PciDevice_CurrentSpeedAndMode_PciX_Mode_Conventional_Pci: u32 = 0u32;
pub const DevProp_PciDevice_CurrentSpeedAndMode_Pci_Conventional_33MHz: u32 = 0u32;
pub const DevProp_PciDevice_CurrentSpeedAndMode_Pci_Conventional_66MHz: u32 = 1u32;
pub const DevProp_PciDevice_DeviceType_PciConventional: u32 = 0u32;
pub const DevProp_PciDevice_DeviceType_PciExpressEndpoint: u32 = 2u32;
pub const DevProp_PciDevice_DeviceType_PciExpressLegacyEndpoint: u32 = 3u32;
pub const DevProp_PciDevice_DeviceType_PciExpressRootComplexIntegratedEndpoint: u32 = 4u32;
pub const DevProp_PciDevice_DeviceType_PciExpressTreatedAsPci: u32 = 5u32;
pub const DevProp_PciDevice_DeviceType_PciX: u32 = 1u32;
pub const DevProp_PciDevice_InterruptType_LineBased: u32 = 1u32;
pub const DevProp_PciDevice_InterruptType_Msi: u32 = 2u32;
pub const DevProp_PciDevice_InterruptType_MsiX: u32 = 4u32;
pub const DevProp_PciDevice_SriovSupport_DidntGetVfBarSpace: u32 = 4u32;
pub const DevProp_PciDevice_SriovSupport_MissingAcs: u32 = 1u32;
pub const DevProp_PciDevice_SriovSupport_MissingPfDriver: u32 = 2u32;
pub const DevProp_PciDevice_SriovSupport_NoBusResource: u32 = 3u32;
pub const DevProp_PciDevice_SriovSupport_Ok: u32 = 0u32;
pub const DevProp_PciExpressDevice_LinkSpeed_Five_Gbps: u32 = 2u32;
pub const DevProp_PciExpressDevice_LinkSpeed_TwoAndHalf_Gbps: u32 = 1u32;
pub const DevProp_PciExpressDevice_LinkWidth_By_1: u32 = 1u32;
pub const DevProp_PciExpressDevice_LinkWidth_By_12: u32 = 12u32;
pub const DevProp_PciExpressDevice_LinkWidth_By_16: u32 = 16u32;
pub const DevProp_PciExpressDevice_LinkWidth_By_2: u32 = 2u32;
pub const DevProp_PciExpressDevice_LinkWidth_By_32: u32 = 32u32;
pub const DevProp_PciExpressDevice_LinkWidth_By_4: u32 = 4u32;
pub const DevProp_PciExpressDevice_LinkWidth_By_8: u32 = 8u32;
pub const DevProp_PciExpressDevice_PayloadOrRequestSize_1024Bytes: u32 = 3u32;
pub const DevProp_PciExpressDevice_PayloadOrRequestSize_128Bytes: u32 = 0u32;
pub const DevProp_PciExpressDevice_PayloadOrRequestSize_2048Bytes: u32 = 4u32;
pub const DevProp_PciExpressDevice_PayloadOrRequestSize_256Bytes: u32 = 1u32;
pub const DevProp_PciExpressDevice_PayloadOrRequestSize_4096Bytes: u32 = 5u32;
pub const DevProp_PciExpressDevice_PayloadOrRequestSize_512Bytes: u32 = 2u32;
pub const DevProp_PciExpressDevice_Spec_Version_10: u32 = 1u32;
pub const DevProp_PciExpressDevice_Spec_Version_11: u32 = 2u32;
pub const DevProp_PciRootBus_BusWidth_32Bits: u32 = 0u32;
pub const DevProp_PciRootBus_BusWidth_64Bits: u32 = 1u32;
pub const DevProp_PciRootBus_CurrentSpeedAndMode_Pci_Conventional_33Mhz: u32 = 0u32;
pub const DevProp_PciRootBus_CurrentSpeedAndMode_Pci_Conventional_66Mhz: u32 = 1u32;
pub const DevProp_PciRootBus_CurrentSpeedAndMode_Pci_X_266_Mode2_100Mhz: u32 = 9u32;
pub const DevProp_PciRootBus_CurrentSpeedAndMode_Pci_X_266_Mode2_133Mhz: u32 = 10u32;
pub const DevProp_PciRootBus_CurrentSpeedAndMode_Pci_X_266_Mode2_66Mhz: u32 = 8u32;
pub const DevProp_PciRootBus_CurrentSpeedAndMode_Pci_X_533_Mode2_100Mhz: u32 = 12u32;
pub const DevProp_PciRootBus_CurrentSpeedAndMode_Pci_X_533_Mode2_133Mhz: u32 = 13u32;
pub const DevProp_PciRootBus_CurrentSpeedAndMode_Pci_X_533_Mode2_66Mhz: u32 = 11u32;
pub const DevProp_PciRootBus_CurrentSpeedAndMode_Pci_X_Mode1_100Mhz: u32 = 3u32;
pub const DevProp_PciRootBus_CurrentSpeedAndMode_Pci_X_Mode1_133Mhz: u32 = 4u32;
pub const DevProp_PciRootBus_CurrentSpeedAndMode_Pci_X_Mode1_66Mhz: u32 = 2u32;
pub const DevProp_PciRootBus_CurrentSpeedAndMode_Pci_X_Mode1_ECC_100Mhz: u32 = 6u32;
pub const DevProp_PciRootBus_CurrentSpeedAndMode_Pci_X_Mode1_ECC_133Mhz: u32 = 7u32;
pub const DevProp_PciRootBus_CurrentSpeedAndMode_Pci_X_Mode1_ECC_66Mhz: u32 = 5u32;
pub const DevProp_PciRootBus_SecondaryInterface_PciConventional: u32 = 0u32;
pub const DevProp_PciRootBus_SecondaryInterface_PciExpress: u32 = 3u32;
pub const DevProp_PciRootBus_SecondaryInterface_PciXMode1: u32 = 1u32;
pub const DevProp_PciRootBus_SecondaryInterface_PciXMode2: u32 = 2u32;
pub const DevProp_PciRootBus_SupportedSpeedsAndModes_Pci_Conventional_33Mhz: u32 = 1u32;
pub const DevProp_PciRootBus_SupportedSpeedsAndModes_Pci_Conventional_66Mhz: u32 = 2u32;
pub const DevProp_PciRootBus_SupportedSpeedsAndModes_Pci_X_133Mhz: u32 = 8u32;
pub const DevProp_PciRootBus_SupportedSpeedsAndModes_Pci_X_266Mhz: u32 = 16u32;
pub const DevProp_PciRootBus_SupportedSpeedsAndModes_Pci_X_533Mhz: u32 = 32u32;
pub const DevProp_PciRootBus_SupportedSpeedsAndModes_Pci_X_66Mhz: u32 = 4u32;
pub const Dot11AdHocManager: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xdd06a84f_83bd_4d01_8ab9_2389fea0869e);
pub const GUID_AEPSERVICE_WIFIDIRECT_DEVICE: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xcc29827c_9caf_4928_99a9_18f7c2381389);
pub const GUID_DEVINTERFACE_ASP_INFRA_DEVICE: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xff823995_7a72_4c80_8757_c67ee13d1a49);
pub const GUID_DEVINTERFACE_WIFIDIRECT_DEVICE: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x439b20af_8955_405b_99f0_a62af0c68d43);
#[repr(transparent)]
pub struct IDot11AdHocInterface(::windows::core::IUnknown);
impl IDot11AdHocInterface {
    pub unsafe fn GetDeviceSignature(&self, psignature: *mut ::windows::core::GUID) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(psignature)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetFriendlyName(&self) -> ::windows::core::Result<super::super::Foundation::PWSTR> {
        let mut result__: super::super::Foundation::PWSTR = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::PWSTR>(result__)
    }
    pub unsafe fn IsDot11d(&self, pf11d: *mut u8) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self), ::core::mem::transmute(pf11d)).ok()
    }
    pub unsafe fn IsAdHocCapable(&self, pfadhoccapable: *mut u8) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).6)(::core::mem::transmute_copy(self), ::core::mem::transmute(pfadhoccapable)).ok()
    }
    pub unsafe fn IsRadioOn(&self, pfisradioon: *mut u8) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).7)(::core::mem::transmute_copy(self), ::core::mem::transmute(pfisradioon)).ok()
    }
    pub unsafe fn GetActiveNetwork(&self) -> ::windows::core::Result<IDot11AdHocNetwork> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).8)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<IDot11AdHocNetwork>(result__)
    }
    pub unsafe fn GetIEnumSecuritySettings(&self) -> ::windows::core::Result<IEnumDot11AdHocSecuritySettings> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).9)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<IEnumDot11AdHocSecuritySettings>(result__)
    }
    pub unsafe fn GetIEnumDot11AdHocNetworks(&self, pfilterguid: *const ::windows::core::GUID) -> ::windows::core::Result<IEnumDot11AdHocNetworks> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).10)(::core::mem::transmute_copy(self), ::core::mem::transmute(pfilterguid), ::core::mem::transmute(&mut result__)).from_abi::<IEnumDot11AdHocNetworks>(result__)
    }
    pub unsafe fn GetStatus(&self, pstate: *mut DOT11_ADHOC_NETWORK_CONNECTION_STATUS) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).11)(::core::mem::transmute_copy(self), ::core::mem::transmute(pstate)).ok()
    }
}
impl ::core::convert::From<IDot11AdHocInterface> for ::windows::core::IUnknown {
    fn from(value: IDot11AdHocInterface) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IDot11AdHocInterface> for ::windows::core::IUnknown {
    fn from(value: &IDot11AdHocInterface) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IDot11AdHocInterface {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &IDot11AdHocInterface {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IDot11AdHocInterface {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IDot11AdHocInterface {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IDot11AdHocInterface {}
unsafe impl ::windows::core::Interface for IDot11AdHocInterface {
    type Vtable = IDot11AdHocInterfaceVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x8f10cc2b_cf0d_42a0_acbe_e2de7007384d);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDot11AdHocInterfaceVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, psignature: *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppszname: *mut super::super::Foundation::PWSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pf11d: *mut u8) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pfadhoccapable: *mut u8) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pfisradioon: *mut u8) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppnetwork: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppenum: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pfilterguid: *const ::windows::core::GUID, ppenum: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pstate: *mut DOT11_ADHOC_NETWORK_CONNECTION_STATUS) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
pub struct IDot11AdHocInterfaceNotificationSink(::windows::core::IUnknown);
impl IDot11AdHocInterfaceNotificationSink {
    pub unsafe fn OnConnectionStatusChange(&self, estatus: DOT11_ADHOC_NETWORK_CONNECTION_STATUS) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(estatus)).ok()
    }
}
impl ::core::convert::From<IDot11AdHocInterfaceNotificationSink> for ::windows::core::IUnknown {
    fn from(value: IDot11AdHocInterfaceNotificationSink) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IDot11AdHocInterfaceNotificationSink> for ::windows::core::IUnknown {
    fn from(value: &IDot11AdHocInterfaceNotificationSink) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IDot11AdHocInterfaceNotificationSink {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &IDot11AdHocInterfaceNotificationSink {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IDot11AdHocInterfaceNotificationSink {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IDot11AdHocInterfaceNotificationSink {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IDot11AdHocInterfaceNotificationSink {}
unsafe impl ::windows::core::Interface for IDot11AdHocInterfaceNotificationSink {
    type Vtable = IDot11AdHocInterfaceNotificationSinkVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x8f10cc2f_cf0d_42a0_acbe_e2de7007384d);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDot11AdHocInterfaceNotificationSinkVtbl(pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT, pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32, pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32, pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, estatus: DOT11_ADHOC_NETWORK_CONNECTION_STATUS) -> ::windows::core::HRESULT);
#[repr(transparent)]
pub struct IDot11AdHocManager(::windows::core::IUnknown);
impl IDot11AdHocManager {
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn CreateNetwork<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>, Param3: ::windows::core::IntoParam<'a, IDot11AdHocInterface>, Param4: ::windows::core::IntoParam<'a, IDot11AdHocSecuritySettings>>(&self, name: Param0, password: Param1, geographicalid: i32, pinterface: Param3, psecurity: Param4, pcontextguid: *const ::windows::core::GUID) -> ::windows::core::Result<IDot11AdHocNetwork> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), name.into_param().abi(), password.into_param().abi(), ::core::mem::transmute(geographicalid), pinterface.into_param().abi(), psecurity.into_param().abi(), ::core::mem::transmute(pcontextguid), ::core::mem::transmute(&mut result__)).from_abi::<IDot11AdHocNetwork>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn CommitCreatedNetwork<'a, Param0: ::windows::core::IntoParam<'a, IDot11AdHocNetwork>, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::BOOLEAN>, Param2: ::windows::core::IntoParam<'a, super::super::Foundation::BOOLEAN>>(&self, piadhoc: Param0, fsaveprofile: Param1, fmakesavedprofileuserspecific: Param2) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), piadhoc.into_param().abi(), fsaveprofile.into_param().abi(), fmakesavedprofileuserspecific.into_param().abi()).ok()
    }
    pub unsafe fn GetIEnumDot11AdHocNetworks(&self, pcontextguid: *const ::windows::core::GUID) -> ::windows::core::Result<IEnumDot11AdHocNetworks> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self), ::core::mem::transmute(pcontextguid), ::core::mem::transmute(&mut result__)).from_abi::<IEnumDot11AdHocNetworks>(result__)
    }
    pub unsafe fn GetIEnumDot11AdHocInterfaces(&self) -> ::windows::core::Result<IEnumDot11AdHocInterfaces> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).6)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<IEnumDot11AdHocInterfaces>(result__)
    }
    pub unsafe fn GetNetwork(&self, networksignature: *const ::windows::core::GUID) -> ::windows::core::Result<IDot11AdHocNetwork> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).7)(::core::mem::transmute_copy(self), ::core::mem::transmute(networksignature), ::core::mem::transmute(&mut result__)).from_abi::<IDot11AdHocNetwork>(result__)
    }
}
impl ::core::convert::From<IDot11AdHocManager> for ::windows::core::IUnknown {
    fn from(value: IDot11AdHocManager) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IDot11AdHocManager> for ::windows::core::IUnknown {
    fn from(value: &IDot11AdHocManager) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IDot11AdHocManager {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &IDot11AdHocManager {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IDot11AdHocManager {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IDot11AdHocManager {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IDot11AdHocManager {}
unsafe impl ::windows::core::Interface for IDot11AdHocManager {
    type Vtable = IDot11AdHocManagerVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x8f10cc26_cf0d_42a0_acbe_e2de7007384d);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDot11AdHocManagerVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, name: super::super::Foundation::PWSTR, password: super::super::Foundation::PWSTR, geographicalid: i32, pinterface: ::windows::core::RawPtr, psecurity: ::windows::core::RawPtr, pcontextguid: *const ::windows::core::GUID, piadhoc: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, piadhoc: ::windows::core::RawPtr, fsaveprofile: super::super::Foundation::BOOLEAN, fmakesavedprofileuserspecific: super::super::Foundation::BOOLEAN) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pcontextguid: *const ::windows::core::GUID, ppenum: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppenum: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, networksignature: *const ::windows::core::GUID, pnetwork: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
pub struct IDot11AdHocManagerNotificationSink(::windows::core::IUnknown);
impl IDot11AdHocManagerNotificationSink {
    pub unsafe fn OnNetworkAdd<'a, Param0: ::windows::core::IntoParam<'a, IDot11AdHocNetwork>>(&self, piadhocnetwork: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), piadhocnetwork.into_param().abi()).ok()
    }
    pub unsafe fn OnNetworkRemove(&self, signature: *const ::windows::core::GUID) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(signature)).ok()
    }
    pub unsafe fn OnInterfaceAdd<'a, Param0: ::windows::core::IntoParam<'a, IDot11AdHocInterface>>(&self, piadhocinterface: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self), piadhocinterface.into_param().abi()).ok()
    }
    pub unsafe fn OnInterfaceRemove(&self, signature: *const ::windows::core::GUID) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).6)(::core::mem::transmute_copy(self), ::core::mem::transmute(signature)).ok()
    }
}
impl ::core::convert::From<IDot11AdHocManagerNotificationSink> for ::windows::core::IUnknown {
    fn from(value: IDot11AdHocManagerNotificationSink) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IDot11AdHocManagerNotificationSink> for ::windows::core::IUnknown {
    fn from(value: &IDot11AdHocManagerNotificationSink) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IDot11AdHocManagerNotificationSink {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &IDot11AdHocManagerNotificationSink {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IDot11AdHocManagerNotificationSink {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IDot11AdHocManagerNotificationSink {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IDot11AdHocManagerNotificationSink {}
unsafe impl ::windows::core::Interface for IDot11AdHocManagerNotificationSink {
    type Vtable = IDot11AdHocManagerNotificationSinkVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x8f10cc27_cf0d_42a0_acbe_e2de7007384d);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDot11AdHocManagerNotificationSinkVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, piadhocnetwork: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, signature: *const ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, piadhocinterface: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, signature: *const ::windows::core::GUID) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
pub struct IDot11AdHocNetwork(::windows::core::IUnknown);
impl IDot11AdHocNetwork {
    pub unsafe fn GetStatus(&self, estatus: *mut DOT11_ADHOC_NETWORK_CONNECTION_STATUS) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(estatus)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetSSID(&self) -> ::windows::core::Result<super::super::Foundation::PWSTR> {
        let mut result__: super::super::Foundation::PWSTR = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::PWSTR>(result__)
    }
    pub unsafe fn HasProfile(&self, pf11d: *mut u8) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self), ::core::mem::transmute(pf11d)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetProfileName(&self) -> ::windows::core::Result<super::super::Foundation::PWSTR> {
        let mut result__: super::super::Foundation::PWSTR = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).6)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::PWSTR>(result__)
    }
    pub unsafe fn DeleteProfile(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).7)(::core::mem::transmute_copy(self)).ok()
    }
    pub unsafe fn GetSignalQuality(&self, pustrengthvalue: *mut u32, pustrengthmax: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).8)(::core::mem::transmute_copy(self), ::core::mem::transmute(pustrengthvalue), ::core::mem::transmute(pustrengthmax)).ok()
    }
    pub unsafe fn GetSecuritySetting(&self) -> ::windows::core::Result<IDot11AdHocSecuritySettings> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).9)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<IDot11AdHocSecuritySettings>(result__)
    }
    pub unsafe fn GetContextGuid(&self, pcontextguid: *mut ::windows::core::GUID) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).10)(::core::mem::transmute_copy(self), ::core::mem::transmute(pcontextguid)).ok()
    }
    pub unsafe fn GetSignature(&self, psignature: *mut ::windows::core::GUID) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).11)(::core::mem::transmute_copy(self), ::core::mem::transmute(psignature)).ok()
    }
    pub unsafe fn GetInterface(&self) -> ::windows::core::Result<IDot11AdHocInterface> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).12)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<IDot11AdHocInterface>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Connect<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>, Param2: ::windows::core::IntoParam<'a, super::super::Foundation::BOOLEAN>, Param3: ::windows::core::IntoParam<'a, super::super::Foundation::BOOLEAN>>(&self, passphrase: Param0, geographicalid: i32, fsaveprofile: Param2, fmakesavedprofileuserspecific: Param3) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).13)(::core::mem::transmute_copy(self), passphrase.into_param().abi(), ::core::mem::transmute(geographicalid), fsaveprofile.into_param().abi(), fmakesavedprofileuserspecific.into_param().abi()).ok()
    }
    pub unsafe fn Disconnect(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).14)(::core::mem::transmute_copy(self)).ok()
    }
}
impl ::core::convert::From<IDot11AdHocNetwork> for ::windows::core::IUnknown {
    fn from(value: IDot11AdHocNetwork) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IDot11AdHocNetwork> for ::windows::core::IUnknown {
    fn from(value: &IDot11AdHocNetwork) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IDot11AdHocNetwork {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &IDot11AdHocNetwork {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IDot11AdHocNetwork {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IDot11AdHocNetwork {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IDot11AdHocNetwork {}
unsafe impl ::windows::core::Interface for IDot11AdHocNetwork {
    type Vtable = IDot11AdHocNetworkVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x8f10cc29_cf0d_42a0_acbe_e2de7007384d);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDot11AdHocNetworkVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, estatus: *mut DOT11_ADHOC_NETWORK_CONNECTION_STATUS) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppszwssid: *mut super::super::Foundation::PWSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pf11d: *mut u8) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppszwprofilename: *mut super::super::Foundation::PWSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pustrengthvalue: *mut u32, pustrengthmax: *mut u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, padhocsecuritysetting: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pcontextguid: *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, psignature: *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, padhocinterface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, passphrase: super::super::Foundation::PWSTR, geographicalid: i32, fsaveprofile: super::super::Foundation::BOOLEAN, fmakesavedprofileuserspecific: super::super::Foundation::BOOLEAN) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
pub struct IDot11AdHocNetworkNotificationSink(::windows::core::IUnknown);
impl IDot11AdHocNetworkNotificationSink {
    pub unsafe fn OnStatusChange(&self, estatus: DOT11_ADHOC_NETWORK_CONNECTION_STATUS) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(estatus)).ok()
    }
    pub unsafe fn OnConnectFail(&self, efailreason: DOT11_ADHOC_CONNECT_FAIL_REASON) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(efailreason)).ok()
    }
}
impl ::core::convert::From<IDot11AdHocNetworkNotificationSink> for ::windows::core::IUnknown {
    fn from(value: IDot11AdHocNetworkNotificationSink) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IDot11AdHocNetworkNotificationSink> for ::windows::core::IUnknown {
    fn from(value: &IDot11AdHocNetworkNotificationSink) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IDot11AdHocNetworkNotificationSink {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &IDot11AdHocNetworkNotificationSink {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IDot11AdHocNetworkNotificationSink {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IDot11AdHocNetworkNotificationSink {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IDot11AdHocNetworkNotificationSink {}
unsafe impl ::windows::core::Interface for IDot11AdHocNetworkNotificationSink {
    type Vtable = IDot11AdHocNetworkNotificationSinkVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x8f10cc2a_cf0d_42a0_acbe_e2de7007384d);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDot11AdHocNetworkNotificationSinkVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, estatus: DOT11_ADHOC_NETWORK_CONNECTION_STATUS) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, efailreason: DOT11_ADHOC_CONNECT_FAIL_REASON) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
pub struct IDot11AdHocSecuritySettings(::windows::core::IUnknown);
impl IDot11AdHocSecuritySettings {
    pub unsafe fn GetDot11AuthAlgorithm(&self, pauth: *mut DOT11_ADHOC_AUTH_ALGORITHM) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(pauth)).ok()
    }
    pub unsafe fn GetDot11CipherAlgorithm(&self, pcipher: *mut DOT11_ADHOC_CIPHER_ALGORITHM) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(pcipher)).ok()
    }
}
impl ::core::convert::From<IDot11AdHocSecuritySettings> for ::windows::core::IUnknown {
    fn from(value: IDot11AdHocSecuritySettings) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IDot11AdHocSecuritySettings> for ::windows::core::IUnknown {
    fn from(value: &IDot11AdHocSecuritySettings) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IDot11AdHocSecuritySettings {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &IDot11AdHocSecuritySettings {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IDot11AdHocSecuritySettings {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IDot11AdHocSecuritySettings {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IDot11AdHocSecuritySettings {}
unsafe impl ::windows::core::Interface for IDot11AdHocSecuritySettings {
    type Vtable = IDot11AdHocSecuritySettingsVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x8f10cc2e_cf0d_42a0_acbe_e2de7007384d);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDot11AdHocSecuritySettingsVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pauth: *mut DOT11_ADHOC_AUTH_ALGORITHM) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pcipher: *mut DOT11_ADHOC_CIPHER_ALGORITHM) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
pub struct IEnumDot11AdHocInterfaces(::windows::core::IUnknown);
impl IEnumDot11AdHocInterfaces {
    pub unsafe fn Next(&self, celt: u32, rgelt: *mut ::core::option::Option<IDot11AdHocInterface>, pceltfetched: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(celt), ::core::mem::transmute(rgelt), ::core::mem::transmute(pceltfetched)).ok()
    }
    pub unsafe fn Skip(&self, celt: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(celt)).ok()
    }
    pub unsafe fn Reset(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self)).ok()
    }
    pub unsafe fn Clone(&self) -> ::windows::core::Result<IEnumDot11AdHocInterfaces> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).6)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<IEnumDot11AdHocInterfaces>(result__)
    }
}
impl ::core::convert::From<IEnumDot11AdHocInterfaces> for ::windows::core::IUnknown {
    fn from(value: IEnumDot11AdHocInterfaces) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IEnumDot11AdHocInterfaces> for ::windows::core::IUnknown {
    fn from(value: &IEnumDot11AdHocInterfaces) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IEnumDot11AdHocInterfaces {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &IEnumDot11AdHocInterfaces {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IEnumDot11AdHocInterfaces {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IEnumDot11AdHocInterfaces {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IEnumDot11AdHocInterfaces {}
unsafe impl ::windows::core::Interface for IEnumDot11AdHocInterfaces {
    type Vtable = IEnumDot11AdHocInterfacesVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x8f10cc2c_cf0d_42a0_acbe_e2de7007384d);
}
#[repr(C)]
#[doc(hidden)]
pub struct IEnumDot11AdHocInterfacesVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, celt: u32, rgelt: *mut ::windows::core::RawPtr, pceltfetched: *mut u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, celt: u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppenum: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
pub struct IEnumDot11AdHocNetworks(::windows::core::IUnknown);
impl IEnumDot11AdHocNetworks {
    pub unsafe fn Next(&self, celt: u32, rgelt: *mut ::core::option::Option<IDot11AdHocNetwork>, pceltfetched: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(celt), ::core::mem::transmute(rgelt), ::core::mem::transmute(pceltfetched)).ok()
    }
    pub unsafe fn Skip(&self, celt: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(celt)).ok()
    }
    pub unsafe fn Reset(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self)).ok()
    }
    pub unsafe fn Clone(&self) -> ::windows::core::Result<IEnumDot11AdHocNetworks> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).6)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<IEnumDot11AdHocNetworks>(result__)
    }
}
impl ::core::convert::From<IEnumDot11AdHocNetworks> for ::windows::core::IUnknown {
    fn from(value: IEnumDot11AdHocNetworks) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IEnumDot11AdHocNetworks> for ::windows::core::IUnknown {
    fn from(value: &IEnumDot11AdHocNetworks) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IEnumDot11AdHocNetworks {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &IEnumDot11AdHocNetworks {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IEnumDot11AdHocNetworks {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IEnumDot11AdHocNetworks {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IEnumDot11AdHocNetworks {}
unsafe impl ::windows::core::Interface for IEnumDot11AdHocNetworks {
    type Vtable = IEnumDot11AdHocNetworksVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x8f10cc28_cf0d_42a0_acbe_e2de7007384d);
}
#[repr(C)]
#[doc(hidden)]
pub struct IEnumDot11AdHocNetworksVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, celt: u32, rgelt: *mut ::windows::core::RawPtr, pceltfetched: *mut u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, celt: u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppenum: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[repr(transparent)]
pub struct IEnumDot11AdHocSecuritySettings(::windows::core::IUnknown);
impl IEnumDot11AdHocSecuritySettings {
    pub unsafe fn Next(&self, celt: u32, rgelt: *mut ::core::option::Option<IDot11AdHocSecuritySettings>, pceltfetched: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(celt), ::core::mem::transmute(rgelt), ::core::mem::transmute(pceltfetched)).ok()
    }
    pub unsafe fn Skip(&self, celt: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(celt)).ok()
    }
    pub unsafe fn Reset(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self)).ok()
    }
    pub unsafe fn Clone(&self) -> ::windows::core::Result<IEnumDot11AdHocSecuritySettings> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).6)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<IEnumDot11AdHocSecuritySettings>(result__)
    }
}
impl ::core::convert::From<IEnumDot11AdHocSecuritySettings> for ::windows::core::IUnknown {
    fn from(value: IEnumDot11AdHocSecuritySettings) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IEnumDot11AdHocSecuritySettings> for ::windows::core::IUnknown {
    fn from(value: &IEnumDot11AdHocSecuritySettings) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IEnumDot11AdHocSecuritySettings {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &IEnumDot11AdHocSecuritySettings {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IEnumDot11AdHocSecuritySettings {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IEnumDot11AdHocSecuritySettings {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IEnumDot11AdHocSecuritySettings {}
unsafe impl ::windows::core::Interface for IEnumDot11AdHocSecuritySettings {
    type Vtable = IEnumDot11AdHocSecuritySettingsVtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x8f10cc2d_cf0d_42a0_acbe_e2de7007384d);
}
#[repr(C)]
#[doc(hidden)]
pub struct IEnumDot11AdHocSecuritySettingsVtbl(
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: &::windows::core::GUID, interface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> u32,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, celt: u32, rgelt: *mut ::windows::core::RawPtr, pceltfetched: *mut u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, celt: u32) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppenum: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
pub const L2_NOTIFICATION_CODE_GROUP_SIZE: u32 = 4096u32;
pub const L2_NOTIFICATION_CODE_PUBLIC_BEGIN: u32 = 0u32;
#[repr(C)]
pub struct L2_NOTIFICATION_DATA {
    pub NotificationSource: u32,
    pub NotificationCode: u32,
    pub InterfaceGuid: ::windows::core::GUID,
    pub dwDataSize: u32,
    pub pData: *mut ::core::ffi::c_void,
}
impl ::core::marker::Copy for L2_NOTIFICATION_DATA {}
impl ::core::clone::Clone for L2_NOTIFICATION_DATA {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for L2_NOTIFICATION_DATA {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for L2_NOTIFICATION_DATA {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<L2_NOTIFICATION_DATA>()) == 0 }
    }
}
impl ::core::cmp::Eq for L2_NOTIFICATION_DATA {}
impl ::core::default::Default for L2_NOTIFICATION_DATA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
pub const L2_NOTIFICATION_SOURCE_ALL: u32 = 65535u32;
pub const L2_NOTIFICATION_SOURCE_DOT3_AUTO_CONFIG: u32 = 1u32;
pub const L2_NOTIFICATION_SOURCE_NONE: u32 = 0u32;
pub const L2_NOTIFICATION_SOURCE_ONEX: u32 = 4u32;
pub const L2_NOTIFICATION_SOURCE_SECURITY: u32 = 2u32;
pub const L2_NOTIFICATION_SOURCE_WCM: u32 = 256u32;
pub const L2_NOTIFICATION_SOURCE_WCM_CSP: u32 = 512u32;
pub const L2_NOTIFICATION_SOURCE_WFD: u32 = 1024u32;
pub const L2_NOTIFICATION_SOURCE_WLAN_ACM: u32 = 8u32;
pub const L2_NOTIFICATION_SOURCE_WLAN_DEVICE_SERVICE: u32 = 2048u32;
pub const L2_NOTIFICATION_SOURCE_WLAN_HNWK: u32 = 128u32;
pub const L2_NOTIFICATION_SOURCE_WLAN_IHV: u32 = 64u32;
pub const L2_NOTIFICATION_SOURCE_WLAN_MSM: u32 = 16u32;
pub const L2_NOTIFICATION_SOURCE_WLAN_SECURITY: u32 = 32u32;
pub const L2_PROFILE_MAX_NAME_LENGTH: u32 = 256u32;
pub const L2_REASON_CODE_DOT11_AC_BASE: u32 = 131072u32;
pub const L2_REASON_CODE_DOT11_MSM_BASE: u32 = 196608u32;
pub const L2_REASON_CODE_DOT11_SECURITY_BASE: u32 = 262144u32;
pub const L2_REASON_CODE_DOT3_AC_BASE: u32 = 393216u32;
pub const L2_REASON_CODE_DOT3_MSM_BASE: u32 = 458752u32;
pub const L2_REASON_CODE_GEN_BASE: u32 = 65536u32;
pub const L2_REASON_CODE_GROUP_SIZE: u32 = 65536u32;
pub const L2_REASON_CODE_IHV_BASE: u32 = 589824u32;
pub const L2_REASON_CODE_ONEX_BASE: u32 = 327680u32;
pub const L2_REASON_CODE_PROFILE_BASE: u32 = 524288u32;
pub const L2_REASON_CODE_PROFILE_MISSING: u32 = 1u32;
pub const L2_REASON_CODE_RESERVED_BASE: u32 = 720896u32;
pub const L2_REASON_CODE_SUCCESS: u32 = 0u32;
pub const L2_REASON_CODE_UNKNOWN: u32 = 65537u32;
pub const L2_REASON_CODE_WIMAX_BASE: u32 = 655360u32;
pub const MAX_NUM_SUPPORTED_RATES: u32 = 8u32;
pub const MAX_NUM_SUPPORTED_RATES_V2: u32 = 255u32;
pub const NDIS_PACKET_TYPE_802_11_ALL_MULTICAST_DATA: u32 = 4u32;
pub const NDIS_PACKET_TYPE_802_11_BROADCAST_DATA: u32 = 8u32;
pub const NDIS_PACKET_TYPE_802_11_DIRECTED_DATA: u32 = 1u32;
pub const NDIS_PACKET_TYPE_802_11_MULTICAST_DATA: u32 = 2u32;
pub const NDIS_PACKET_TYPE_802_11_PROMISCUOUS_DATA: u32 = 32u32;
pub const OID_DOT11_AP_JOIN_REQUEST: u32 = 218170205u32;
pub const OID_DOT11_ATIM_WINDOW: u32 = 218170122u32;
pub const OID_DOT11_BEACON_PERIOD: u32 = 218170139u32;
pub const OID_DOT11_CCA_MODE_SUPPORTED: u32 = 218170166u32;
pub const OID_DOT11_CCA_WATCHDOG_COUNT_MAX: u32 = 218170170u32;
pub const OID_DOT11_CCA_WATCHDOG_COUNT_MIN: u32 = 218170172u32;
pub const OID_DOT11_CCA_WATCHDOG_TIMER_MAX: u32 = 218170169u32;
pub const OID_DOT11_CCA_WATCHDOG_TIMER_MIN: u32 = 218170171u32;
pub const OID_DOT11_CFP_MAX_DURATION: u32 = 218170136u32;
pub const OID_DOT11_CFP_PERIOD: u32 = 218170135u32;
pub const OID_DOT11_CF_POLLABLE: u32 = 218170134u32;
pub const OID_DOT11_CHANNEL_AGILITY_ENABLED: u32 = 218170184u32;
pub const OID_DOT11_CHANNEL_AGILITY_PRESENT: u32 = 218170183u32;
pub const OID_DOT11_COUNTERS_ENTRY: u32 = 218170149u32;
pub const OID_DOT11_COUNTRY_STRING: u32 = 218170188u32;
pub const OID_DOT11_CURRENT_ADDRESS: u32 = 218171138u32;
pub const OID_DOT11_CURRENT_CCA_MODE: u32 = 218170167u32;
pub const OID_DOT11_CURRENT_CHANNEL: u32 = 218170165u32;
pub const OID_DOT11_CURRENT_CHANNEL_NUMBER: u32 = 218170159u32;
pub const OID_DOT11_CURRENT_DWELL_TIME: u32 = 218170161u32;
pub const OID_DOT11_CURRENT_FREQUENCY: u32 = 218170178u32;
pub const OID_DOT11_CURRENT_INDEX: u32 = 218170164u32;
pub const OID_DOT11_CURRENT_OFFLOAD_CAPABILITY: u32 = 218170113u32;
pub const OID_DOT11_CURRENT_OPERATION_MODE: u32 = 218170120u32;
pub const OID_DOT11_CURRENT_OPTIONAL_CAPABILITY: u32 = 218170131u32;
pub const OID_DOT11_CURRENT_PACKET_FILTER: u32 = 218170121u32;
pub const OID_DOT11_CURRENT_PATTERN: u32 = 218170163u32;
pub const OID_DOT11_CURRENT_PHY_TYPE: u32 = 218170124u32;
pub const OID_DOT11_CURRENT_REG_DOMAIN: u32 = 218170151u32;
pub const OID_DOT11_CURRENT_RX_ANTENNA: u32 = 218170155u32;
pub const OID_DOT11_CURRENT_SET: u32 = 218170162u32;
pub const OID_DOT11_CURRENT_TX_ANTENNA: u32 = 218170153u32;
pub const OID_DOT11_CURRENT_TX_POWER_LEVEL: u32 = 218170157u32;
pub const OID_DOT11_DEFAULT_WEP_OFFLOAD: u32 = 218170116u32;
pub const OID_DOT11_DEFAULT_WEP_UPLOAD: u32 = 218170117u32;
pub const OID_DOT11_DIVERSITY_SELECTION_RX: u32 = 218170176u32;
pub const OID_DOT11_DIVERSITY_SUPPORT: u32 = 218170154u32;
pub const OID_DOT11_DSSS_OFDM_OPTION_ENABLED: u32 = 218170209u32;
pub const OID_DOT11_DSSS_OFDM_OPTION_IMPLEMENTED: u32 = 218170208u32;
pub const OID_DOT11_DTIM_PERIOD: u32 = 218170140u32;
pub const OID_DOT11_ED_THRESHOLD: u32 = 218170168u32;
pub const OID_DOT11_EHCC_CAPABILITY_ENABLED: u32 = 218170193u32;
pub const OID_DOT11_EHCC_CAPABILITY_IMPLEMENTED: u32 = 218170192u32;
pub const OID_DOT11_EHCC_NUMBER_OF_CHANNELS_FAMILY_INDEX: u32 = 218170191u32;
pub const OID_DOT11_EHCC_PRIME_RADIX: u32 = 218170190u32;
pub const OID_DOT11_ERP_PBCC_OPTION_ENABLED: u32 = 218170207u32;
pub const OID_DOT11_ERP_PBCC_OPTION_IMPLEMENTED: u32 = 218170206u32;
pub const OID_DOT11_FRAGMENTATION_THRESHOLD: u32 = 218170146u32;
pub const OID_DOT11_FREQUENCY_BANDS_SUPPORTED: u32 = 218170180u32;
pub const OID_DOT11_HOPPING_PATTERN: u32 = 218170199u32;
pub const OID_DOT11_HOP_ALGORITHM_ADOPTED: u32 = 218170194u32;
pub const OID_DOT11_HOP_MODULUS: u32 = 218170197u32;
pub const OID_DOT11_HOP_OFFSET: u32 = 218170198u32;
pub const OID_DOT11_HOP_TIME: u32 = 218170158u32;
pub const OID_DOT11_HR_CCA_MODE_SUPPORTED: u32 = 218170185u32;
pub const OID_DOT11_JOIN_REQUEST: u32 = 218170125u32;
pub const OID_DOT11_LONG_RETRY_LIMIT: u32 = 218170145u32;
pub const OID_DOT11_MAC_ADDRESS: u32 = 218170142u32;
pub const OID_DOT11_MAXIMUM_LIST_SIZE: u32 = 218171141u32;
pub const OID_DOT11_MAX_DWELL_TIME: u32 = 218170160u32;
pub const OID_DOT11_MAX_MAC_ADDRESS_STATES: u32 = 218170212u32;
pub const OID_DOT11_MAX_RECEIVE_LIFETIME: u32 = 218170148u32;
pub const OID_DOT11_MAX_TRANSMIT_MSDU_LIFETIME: u32 = 218170147u32;
pub const OID_DOT11_MEDIUM_OCCUPANCY_LIMIT: u32 = 218170133u32;
pub const OID_DOT11_MPDU_MAX_LENGTH: u32 = 218170118u32;
pub const OID_DOT11_MULTICAST_LIST: u32 = 218171140u32;
pub const OID_DOT11_MULTI_DOMAIN_CAPABILITY: u32 = 218170189u32;
pub const OID_DOT11_MULTI_DOMAIN_CAPABILITY_ENABLED: u32 = 218170187u32;
pub const OID_DOT11_MULTI_DOMAIN_CAPABILITY_IMPLEMENTED: u32 = 218170186u32;
pub const OID_DOT11_NDIS_START: u32 = 218170112u32;
pub const OID_DOT11_NIC_POWER_STATE: u32 = 218170129u32;
pub const OID_DOT11_NIC_SPECIFIC_EXTENSION: u32 = 218170204u32;
pub const OID_DOT11_NUMBER_OF_HOPPING_SETS: u32 = 218170196u32;
pub const OID_DOT11_OFFLOAD_CAPABILITY: u32 = 218170112u32;
pub const OID_DOT11_OPERATIONAL_RATE_SET: u32 = 218170138u32;
pub const OID_DOT11_OPERATION_MODE_CAPABILITY: u32 = 218170119u32;
pub const OID_DOT11_OPTIONAL_CAPABILITY: u32 = 218170130u32;
pub const OID_DOT11_PBCC_OPTION_IMPLEMENTED: u32 = 218170182u32;
pub const OID_DOT11_PERMANENT_ADDRESS: u32 = 218171139u32;
pub const OID_DOT11_POWER_MGMT_MODE: u32 = 218170137u32;
pub const OID_DOT11_PRIVATE_OIDS_START: u32 = 218171136u32;
pub const OID_DOT11_QOS_TX_DURATION: u32 = 218170219u32;
pub const OID_DOT11_QOS_TX_MEDIUM_TIME: u32 = 218170220u32;
pub const OID_DOT11_QOS_TX_QUEUES_SUPPORTED: u32 = 218170218u32;
pub const OID_DOT11_RANDOM_TABLE_FIELD_NUMBER: u32 = 218170200u32;
pub const OID_DOT11_RANDOM_TABLE_FLAG: u32 = 218170195u32;
pub const OID_DOT11_RECV_SENSITIVITY_LIST: u32 = 218170213u32;
pub const OID_DOT11_REG_DOMAINS_SUPPORT_VALUE: u32 = 218170173u32;
pub const OID_DOT11_RESET_REQUEST: u32 = 218170128u32;
pub const OID_DOT11_RF_USAGE: u32 = 218170203u32;
pub const OID_DOT11_RSSI_RANGE: u32 = 218170202u32;
pub const OID_DOT11_RTS_THRESHOLD: u32 = 218170143u32;
pub const OID_DOT11_SCAN_REQUEST: u32 = 218170123u32;
pub const OID_DOT11_SHORT_PREAMBLE_OPTION_IMPLEMENTED: u32 = 218170181u32;
pub const OID_DOT11_SHORT_RETRY_LIMIT: u32 = 218170144u32;
pub const OID_DOT11_SHORT_SLOT_TIME_OPTION_ENABLED: u32 = 218170211u32;
pub const OID_DOT11_SHORT_SLOT_TIME_OPTION_IMPLEMENTED: u32 = 218170210u32;
pub const OID_DOT11_START_REQUEST: u32 = 218170126u32;
pub const OID_DOT11_STATION_ID: u32 = 218170132u32;
pub const OID_DOT11_SUPPORTED_DATA_RATES_VALUE: u32 = 218170177u32;
pub const OID_DOT11_SUPPORTED_DSSS_CHANNEL_LIST: u32 = 218170222u32;
pub const OID_DOT11_SUPPORTED_OFDM_FREQUENCY_LIST: u32 = 218170221u32;
pub const OID_DOT11_SUPPORTED_PHY_TYPES: u32 = 218170150u32;
pub const OID_DOT11_SUPPORTED_POWER_LEVELS: u32 = 218170156u32;
pub const OID_DOT11_SUPPORTED_RX_ANTENNA: u32 = 218170175u32;
pub const OID_DOT11_SUPPORTED_TX_ANTENNA: u32 = 218170174u32;
pub const OID_DOT11_TEMP_TYPE: u32 = 218170152u32;
pub const OID_DOT11_TI_THRESHOLD: u32 = 218170179u32;
pub const OID_DOT11_UPDATE_IE: u32 = 218170127u32;
pub const OID_DOT11_WEP_ICV_ERROR_COUNT: u32 = 218170141u32;
pub const OID_DOT11_WEP_OFFLOAD: u32 = 218170114u32;
pub const OID_DOT11_WEP_UPLOAD: u32 = 218170115u32;
pub const OID_DOT11_WME_AC_PARAMETERS: u32 = 218170216u32;
pub const OID_DOT11_WME_ENABLED: u32 = 218170215u32;
pub const OID_DOT11_WME_IMPLEMENTED: u32 = 218170214u32;
pub const OID_DOT11_WME_UPDATE_IE: u32 = 218170217u32;
pub const OID_DOT11_WPA_TSC: u32 = 218170201u32;
pub type ONEX_AUTH_IDENTITY = i32;
pub const OneXAuthIdentityNone: ONEX_AUTH_IDENTITY = 0i32;
pub const OneXAuthIdentityMachine: ONEX_AUTH_IDENTITY = 1i32;
pub const OneXAuthIdentityUser: ONEX_AUTH_IDENTITY = 2i32;
pub const OneXAuthIdentityExplicitUser: ONEX_AUTH_IDENTITY = 3i32;
pub const OneXAuthIdentityGuest: ONEX_AUTH_IDENTITY = 4i32;
pub const OneXAuthIdentityInvalid: ONEX_AUTH_IDENTITY = 5i32;
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct ONEX_AUTH_PARAMS {
    pub fUpdatePending: super::super::Foundation::BOOL,
    pub oneXConnProfile: ONEX_VARIABLE_BLOB,
    pub authIdentity: ONEX_AUTH_IDENTITY,
    pub dwQuarantineState: u32,
    pub _bitfield: u32,
    pub dwSessionId: u32,
    pub hUserToken: super::super::Foundation::HANDLE,
    pub OneXUserProfile: ONEX_VARIABLE_BLOB,
    pub Identity: ONEX_VARIABLE_BLOB,
    pub UserName: ONEX_VARIABLE_BLOB,
    pub Domain: ONEX_VARIABLE_BLOB,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for ONEX_AUTH_PARAMS {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for ONEX_AUTH_PARAMS {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for ONEX_AUTH_PARAMS {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for ONEX_AUTH_PARAMS {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<ONEX_AUTH_PARAMS>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for ONEX_AUTH_PARAMS {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for ONEX_AUTH_PARAMS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
pub type ONEX_AUTH_RESTART_REASON = i32;
pub const OneXRestartReasonPeerInitiated: ONEX_AUTH_RESTART_REASON = 0i32;
pub const OneXRestartReasonMsmInitiated: ONEX_AUTH_RESTART_REASON = 1i32;
pub const OneXRestartReasonOneXHeldStateTimeout: ONEX_AUTH_RESTART_REASON = 2i32;
pub const OneXRestartReasonOneXAuthTimeout: ONEX_AUTH_RESTART_REASON = 3i32;
pub const OneXRestartReasonOneXConfigurationChanged: ONEX_AUTH_RESTART_REASON = 4i32;
pub const OneXRestartReasonOneXUserChanged: ONEX_AUTH_RESTART_REASON = 5i32;
pub const OneXRestartReasonQuarantineStateChanged: ONEX_AUTH_RESTART_REASON = 6i32;
pub const OneXRestartReasonAltCredsTrial: ONEX_AUTH_RESTART_REASON = 7i32;
pub const OneXRestartReasonInvalid: ONEX_AUTH_RESTART_REASON = 8i32;
pub type ONEX_AUTH_STATUS = i32;
pub const OneXAuthNotStarted: ONEX_AUTH_STATUS = 0i32;
pub const OneXAuthInProgress: ONEX_AUTH_STATUS = 1i32;
pub const OneXAuthNoAuthenticatorFound: ONEX_AUTH_STATUS = 2i32;
pub const OneXAuthSuccess: ONEX_AUTH_STATUS = 3i32;
pub const OneXAuthFailure: ONEX_AUTH_STATUS = 4i32;
pub const OneXAuthInvalid: ONEX_AUTH_STATUS = 5i32;
#[repr(C)]
#[cfg(feature = "Win32_Security_ExtensibleAuthenticationProtocol")]
pub struct ONEX_EAP_ERROR {
    pub dwWinError: u32,
    pub r#type: super::super::Security::ExtensibleAuthenticationProtocol::EAP_METHOD_TYPE,
    pub dwReasonCode: u32,
    pub rootCauseGuid: ::windows::core::GUID,
    pub repairGuid: ::windows::core::GUID,
    pub helpLinkGuid: ::windows::core::GUID,
    pub _bitfield: u32,
    pub RootCauseString: ONEX_VARIABLE_BLOB,
    pub RepairString: ONEX_VARIABLE_BLOB,
}
#[cfg(feature = "Win32_Security_ExtensibleAuthenticationProtocol")]
impl ::core::marker::Copy for ONEX_EAP_ERROR {}
#[cfg(feature = "Win32_Security_ExtensibleAuthenticationProtocol")]
impl ::core::clone::Clone for ONEX_EAP_ERROR {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Security_ExtensibleAuthenticationProtocol")]
unsafe impl ::windows::core::Abi for ONEX_EAP_ERROR {
    type Abi = Self;
}
#[cfg(feature = "Win32_Security_ExtensibleAuthenticationProtocol")]
impl ::core::cmp::PartialEq for ONEX_EAP_ERROR {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<ONEX_EAP_ERROR>()) == 0 }
    }
}
#[cfg(feature = "Win32_Security_ExtensibleAuthenticationProtocol")]
impl ::core::cmp::Eq for ONEX_EAP_ERROR {}
#[cfg(feature = "Win32_Security_ExtensibleAuthenticationProtocol")]
impl ::core::default::Default for ONEX_EAP_ERROR {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
pub type ONEX_EAP_METHOD_BACKEND_SUPPORT = i32;
pub const OneXEapMethodBackendSupportUnknown: ONEX_EAP_METHOD_BACKEND_SUPPORT = 0i32;
pub const OneXEapMethodBackendSupported: ONEX_EAP_METHOD_BACKEND_SUPPORT = 1i32;
pub const OneXEapMethodBackendUnsupported: ONEX_EAP_METHOD_BACKEND_SUPPORT = 2i32;
pub type ONEX_NOTIFICATION_TYPE = i32;
pub const OneXPublicNotificationBase: ONEX_NOTIFICATION_TYPE = 0i32;
pub const OneXNotificationTypeResultUpdate: ONEX_NOTIFICATION_TYPE = 1i32;
pub const OneXNotificationTypeAuthRestarted: ONEX_NOTIFICATION_TYPE = 2i32;
pub const OneXNotificationTypeEventInvalid: ONEX_NOTIFICATION_TYPE = 3i32;
pub const OneXNumNotifications: ONEX_NOTIFICATION_TYPE = 3i32;
pub type ONEX_REASON_CODE = i32;
pub const ONEX_REASON_CODE_SUCCESS: ONEX_REASON_CODE = 0i32;
pub const ONEX_REASON_START: ONEX_REASON_CODE = 327680i32;
pub const ONEX_UNABLE_TO_IDENTIFY_USER: ONEX_REASON_CODE = 327681i32;
pub const ONEX_IDENTITY_NOT_FOUND: ONEX_REASON_CODE = 327682i32;
pub const ONEX_UI_DISABLED: ONEX_REASON_CODE = 327683i32;
pub const ONEX_UI_FAILURE: ONEX_REASON_CODE = 327684i32;
pub const ONEX_EAP_FAILURE_RECEIVED: ONEX_REASON_CODE = 327685i32;
pub const ONEX_AUTHENTICATOR_NO_LONGER_PRESENT: ONEX_REASON_CODE = 327686i32;
pub const ONEX_NO_RESPONSE_TO_IDENTITY: ONEX_REASON_CODE = 327687i32;
pub const ONEX_PROFILE_VERSION_NOT_SUPPORTED: ONEX_REASON_CODE = 327688i32;
pub const ONEX_PROFILE_INVALID_LENGTH: ONEX_REASON_CODE = 327689i32;
pub const ONEX_PROFILE_DISALLOWED_EAP_TYPE: ONEX_REASON_CODE = 327690i32;
pub const ONEX_PROFILE_INVALID_EAP_TYPE_OR_FLAG: ONEX_REASON_CODE = 327691i32;
pub const ONEX_PROFILE_INVALID_ONEX_FLAGS: ONEX_REASON_CODE = 327692i32;
pub const ONEX_PROFILE_INVALID_TIMER_VALUE: ONEX_REASON_CODE = 327693i32;
pub const ONEX_PROFILE_INVALID_SUPPLICANT_MODE: ONEX_REASON_CODE = 327694i32;
pub const ONEX_PROFILE_INVALID_AUTH_MODE: ONEX_REASON_CODE = 327695i32;
pub const ONEX_PROFILE_INVALID_EAP_CONNECTION_PROPERTIES: ONEX_REASON_CODE = 327696i32;
pub const ONEX_UI_CANCELLED: ONEX_REASON_CODE = 327697i32;
pub const ONEX_PROFILE_INVALID_EXPLICIT_CREDENTIALS: ONEX_REASON_CODE = 327698i32;
pub const ONEX_PROFILE_EXPIRED_EXPLICIT_CREDENTIALS: ONEX_REASON_CODE = 327699i32;
pub const ONEX_UI_NOT_PERMITTED: ONEX_REASON_CODE = 327700i32;
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct ONEX_RESULT_UPDATE_DATA {
    pub oneXStatus: ONEX_STATUS,
    pub BackendSupport: ONEX_EAP_METHOD_BACKEND_SUPPORT,
    pub fBackendEngaged: super::super::Foundation::BOOL,
    pub _bitfield: u32,
    pub authParams: ONEX_VARIABLE_BLOB,
    pub eapError: ONEX_VARIABLE_BLOB,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for ONEX_RESULT_UPDATE_DATA {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for ONEX_RESULT_UPDATE_DATA {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for ONEX_RESULT_UPDATE_DATA {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for ONEX_RESULT_UPDATE_DATA {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<ONEX_RESULT_UPDATE_DATA>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for ONEX_RESULT_UPDATE_DATA {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for ONEX_RESULT_UPDATE_DATA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct ONEX_STATUS {
    pub authStatus: ONEX_AUTH_STATUS,
    pub dwReason: u32,
    pub dwError: u32,
}
impl ::core::marker::Copy for ONEX_STATUS {}
impl ::core::clone::Clone for ONEX_STATUS {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for ONEX_STATUS {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for ONEX_STATUS {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<ONEX_STATUS>()) == 0 }
    }
}
impl ::core::cmp::Eq for ONEX_STATUS {}
impl ::core::default::Default for ONEX_STATUS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct ONEX_USER_INFO {
    pub authIdentity: ONEX_AUTH_IDENTITY,
    pub _bitfield: u32,
    pub UserName: ONEX_VARIABLE_BLOB,
    pub DomainName: ONEX_VARIABLE_BLOB,
}
impl ::core::marker::Copy for ONEX_USER_INFO {}
impl ::core::clone::Clone for ONEX_USER_INFO {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for ONEX_USER_INFO {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for ONEX_USER_INFO {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<ONEX_USER_INFO>()) == 0 }
    }
}
impl ::core::cmp::Eq for ONEX_USER_INFO {}
impl ::core::default::Default for ONEX_USER_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct ONEX_VARIABLE_BLOB {
    pub dwSize: u32,
    pub dwOffset: u32,
}
impl ::core::marker::Copy for ONEX_VARIABLE_BLOB {}
impl ::core::clone::Clone for ONEX_VARIABLE_BLOB {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for ONEX_VARIABLE_BLOB {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for ONEX_VARIABLE_BLOB {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<ONEX_VARIABLE_BLOB>()) == 0 }
    }
}
impl ::core::cmp::Eq for ONEX_VARIABLE_BLOB {}
impl ::core::default::Default for ONEX_VARIABLE_BLOB {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn WFDCancelOpenSession<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::HANDLE>>(hsessionhandle: Param0) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn WFDCancelOpenSession(hsessionhandle: super::super::Foundation::HANDLE) -> u32;
        }
        ::core::mem::transmute(WFDCancelOpenSession(hsessionhandle.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn WFDCloseHandle<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::HANDLE>>(hclienthandle: Param0) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn WFDCloseHandle(hclienthandle: super::super::Foundation::HANDLE) -> u32;
        }
        ::core::mem::transmute(WFDCloseHandle(hclienthandle.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn WFDCloseSession<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::HANDLE>>(hsessionhandle: Param0) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn WFDCloseSession(hsessionhandle: super::super::Foundation::HANDLE) -> u32;
        }
        ::core::mem::transmute(WFDCloseSession(hsessionhandle.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn WFDOpenHandle(dwclientversion: u32, pdwnegotiatedversion: *mut u32, phclienthandle: *mut super::super::Foundation::HANDLE) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn WFDOpenHandle(dwclientversion: u32, pdwnegotiatedversion: *mut u32, phclienthandle: *mut super::super::Foundation::HANDLE) -> u32;
        }
        ::core::mem::transmute(WFDOpenHandle(::core::mem::transmute(dwclientversion), ::core::mem::transmute(pdwnegotiatedversion), ::core::mem::transmute(phclienthandle)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn WFDOpenLegacySession<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::HANDLE>>(hclienthandle: Param0, plegacymacaddress: *const *const u8, phsessionhandle: *mut super::super::Foundation::HANDLE, pguidsessioninterface: *mut ::windows::core::GUID) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn WFDOpenLegacySession(hclienthandle: super::super::Foundation::HANDLE, plegacymacaddress: *const *const u8, phsessionhandle: *mut super::super::Foundation::HANDLE, pguidsessioninterface: *mut ::windows::core::GUID) -> u32;
        }
        ::core::mem::transmute(WFDOpenLegacySession(hclienthandle.into_param().abi(), ::core::mem::transmute(plegacymacaddress), ::core::mem::transmute(phsessionhandle), ::core::mem::transmute(pguidsessioninterface)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct WFDSVC_CONNECTION_CAPABILITY {
    pub bNew: super::super::Foundation::BOOLEAN,
    pub bClient: super::super::Foundation::BOOLEAN,
    pub bGO: super::super::Foundation::BOOLEAN,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for WFDSVC_CONNECTION_CAPABILITY {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for WFDSVC_CONNECTION_CAPABILITY {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for WFDSVC_CONNECTION_CAPABILITY {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for WFDSVC_CONNECTION_CAPABILITY {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<WFDSVC_CONNECTION_CAPABILITY>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for WFDSVC_CONNECTION_CAPABILITY {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for WFDSVC_CONNECTION_CAPABILITY {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
pub const WFDSVC_CONNECTION_CAPABILITY_CLIENT: u32 = 2u32;
pub const WFDSVC_CONNECTION_CAPABILITY_GO: u32 = 4u32;
pub const WFDSVC_CONNECTION_CAPABILITY_NEW: u32 = 1u32;
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn WFDStartOpenSession<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::HANDLE>>(hclienthandle: Param0, pdeviceaddress: *const *const u8, pvcontext: *const ::core::ffi::c_void, pfncallback: WFD_OPEN_SESSION_COMPLETE_CALLBACK, phsessionhandle: *mut super::super::Foundation::HANDLE) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn WFDStartOpenSession(hclienthandle: super::super::Foundation::HANDLE, pdeviceaddress: *const *const u8, pvcontext: *const ::core::ffi::c_void, pfncallback: ::windows::core::RawPtr, phsessionhandle: *mut super::super::Foundation::HANDLE) -> u32;
        }
        ::core::mem::transmute(WFDStartOpenSession(hclienthandle.into_param().abi(), ::core::mem::transmute(pdeviceaddress), ::core::mem::transmute(pvcontext), ::core::mem::transmute(pfncallback), ::core::mem::transmute(phsessionhandle)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn WFDUpdateDeviceVisibility(pdeviceaddress: *const *const u8) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn WFDUpdateDeviceVisibility(pdeviceaddress: *const *const u8) -> u32;
        }
        ::core::mem::transmute(WFDUpdateDeviceVisibility(::core::mem::transmute(pdeviceaddress)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
pub const WFD_API_VERSION: u32 = 1u32;
pub const WFD_API_VERSION_1_0: u32 = 1u32;
#[repr(C)]
pub struct WFD_GROUP_ID {
    pub DeviceAddress: [u8; 6],
    pub GroupSSID: DOT11_SSID,
}
impl ::core::marker::Copy for WFD_GROUP_ID {}
impl ::core::clone::Clone for WFD_GROUP_ID {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for WFD_GROUP_ID {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for WFD_GROUP_ID {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<WFD_GROUP_ID>()) == 0 }
    }
}
impl ::core::cmp::Eq for WFD_GROUP_ID {}
impl ::core::default::Default for WFD_GROUP_ID {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
pub type WFD_OPEN_SESSION_COMPLETE_CALLBACK = ::core::option::Option<unsafe extern "system" fn(hsessionhandle: super::super::Foundation::HANDLE, pvcontext: *const ::core::ffi::c_void, guidsessioninterface: ::windows::core::GUID, dwerror: u32, dwreasoncode: u32)>;
pub type WFD_ROLE_TYPE = i32;
pub const WFD_ROLE_TYPE_NONE: WFD_ROLE_TYPE = 0i32;
pub const WFD_ROLE_TYPE_DEVICE: WFD_ROLE_TYPE = 1i32;
pub const WFD_ROLE_TYPE_GROUP_OWNER: WFD_ROLE_TYPE = 2i32;
pub const WFD_ROLE_TYPE_CLIENT: WFD_ROLE_TYPE = 4i32;
pub const WFD_ROLE_TYPE_MAX: WFD_ROLE_TYPE = 5i32;
pub type WLAN_ADHOC_NETWORK_STATE = i32;
pub const wlan_adhoc_network_state_formed: WLAN_ADHOC_NETWORK_STATE = 0i32;
pub const wlan_adhoc_network_state_connected: WLAN_ADHOC_NETWORK_STATE = 1i32;
pub const WLAN_API_VERSION: u32 = 2u32;
pub const WLAN_API_VERSION_1_0: u32 = 1u32;
pub const WLAN_API_VERSION_2_0: u32 = 2u32;
#[repr(C)]
pub struct WLAN_ASSOCIATION_ATTRIBUTES {
    pub dot11Ssid: DOT11_SSID,
    pub dot11BssType: DOT11_BSS_TYPE,
    pub dot11Bssid: [u8; 6],
    pub dot11PhyType: DOT11_PHY_TYPE,
    pub uDot11PhyIndex: u32,
    pub wlanSignalQuality: u32,
    pub ulRxRate: u32,
    pub ulTxRate: u32,
}
impl ::core::marker::Copy for WLAN_ASSOCIATION_ATTRIBUTES {}
impl ::core::clone::Clone for WLAN_ASSOCIATION_ATTRIBUTES {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for WLAN_ASSOCIATION_ATTRIBUTES {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for WLAN_ASSOCIATION_ATTRIBUTES {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<WLAN_ASSOCIATION_ATTRIBUTES>()) == 0 }
    }
}
impl ::core::cmp::Eq for WLAN_ASSOCIATION_ATTRIBUTES {}
impl ::core::default::Default for WLAN_ASSOCIATION_ATTRIBUTES {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct WLAN_AUTH_CIPHER_PAIR_LIST {
    pub dwNumberOfItems: u32,
    pub pAuthCipherPairList: [DOT11_AUTH_CIPHER_PAIR; 1],
}
impl ::core::marker::Copy for WLAN_AUTH_CIPHER_PAIR_LIST {}
impl ::core::clone::Clone for WLAN_AUTH_CIPHER_PAIR_LIST {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for WLAN_AUTH_CIPHER_PAIR_LIST {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for WLAN_AUTH_CIPHER_PAIR_LIST {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<WLAN_AUTH_CIPHER_PAIR_LIST>()) == 0 }
    }
}
impl ::core::cmp::Eq for WLAN_AUTH_CIPHER_PAIR_LIST {}
impl ::core::default::Default for WLAN_AUTH_CIPHER_PAIR_LIST {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
pub type WLAN_AUTOCONF_OPCODE = i32;
pub const wlan_autoconf_opcode_start: WLAN_AUTOCONF_OPCODE = 0i32;
pub const wlan_autoconf_opcode_show_denied_networks: WLAN_AUTOCONF_OPCODE = 1i32;
pub const wlan_autoconf_opcode_power_setting: WLAN_AUTOCONF_OPCODE = 2i32;
pub const wlan_autoconf_opcode_only_use_gp_profiles_for_allowed_networks: WLAN_AUTOCONF_OPCODE = 3i32;
pub const wlan_autoconf_opcode_allow_explicit_creds: WLAN_AUTOCONF_OPCODE = 4i32;
pub const wlan_autoconf_opcode_block_period: WLAN_AUTOCONF_OPCODE = 5i32;
pub const wlan_autoconf_opcode_allow_virtual_station_extensibility: WLAN_AUTOCONF_OPCODE = 6i32;
pub const wlan_autoconf_opcode_end: WLAN_AUTOCONF_OPCODE = 7i32;
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct WLAN_AVAILABLE_NETWORK {
    pub strProfileName: [u16; 256],
    pub dot11Ssid: DOT11_SSID,
    pub dot11BssType: DOT11_BSS_TYPE,
    pub uNumberOfBssids: u32,
    pub bNetworkConnectable: super::super::Foundation::BOOL,
    pub wlanNotConnectableReason: u32,
    pub uNumberOfPhyTypes: u32,
    pub dot11PhyTypes: [DOT11_PHY_TYPE; 8],
    pub bMorePhyTypes: super::super::Foundation::BOOL,
    pub wlanSignalQuality: u32,
    pub bSecurityEnabled: super::super::Foundation::BOOL,
    pub dot11DefaultAuthAlgorithm: DOT11_AUTH_ALGORITHM,
    pub dot11DefaultCipherAlgorithm: DOT11_CIPHER_ALGORITHM,
    pub dwFlags: u32,
    pub dwReserved: u32,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for WLAN_AVAILABLE_NETWORK {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for WLAN_AVAILABLE_NETWORK {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for WLAN_AVAILABLE_NETWORK {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for WLAN_AVAILABLE_NETWORK {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<WLAN_AVAILABLE_NETWORK>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for WLAN_AVAILABLE_NETWORK {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for WLAN_AVAILABLE_NETWORK {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
pub const WLAN_AVAILABLE_NETWORK_ANQP_SUPPORTED: u32 = 32u32;
pub const WLAN_AVAILABLE_NETWORK_AUTO_CONNECT_FAILED: u32 = 256u32;
pub const WLAN_AVAILABLE_NETWORK_CONNECTED: u32 = 1u32;
pub const WLAN_AVAILABLE_NETWORK_CONSOLE_USER_PROFILE: u32 = 4u32;
pub const WLAN_AVAILABLE_NETWORK_HAS_PROFILE: u32 = 2u32;
pub const WLAN_AVAILABLE_NETWORK_HOTSPOT2_DOMAIN: u32 = 64u32;
pub const WLAN_AVAILABLE_NETWORK_HOTSPOT2_ENABLED: u32 = 16u32;
pub const WLAN_AVAILABLE_NETWORK_HOTSPOT2_ROAMING: u32 = 128u32;
pub const WLAN_AVAILABLE_NETWORK_INCLUDE_ALL_ADHOC_PROFILES: u32 = 1u32;
pub const WLAN_AVAILABLE_NETWORK_INCLUDE_ALL_MANUAL_HIDDEN_PROFILES: u32 = 2u32;
pub const WLAN_AVAILABLE_NETWORK_INTERWORKING_SUPPORTED: u32 = 8u32;
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct WLAN_AVAILABLE_NETWORK_LIST {
    pub dwNumberOfItems: u32,
    pub dwIndex: u32,
    pub Network: [WLAN_AVAILABLE_NETWORK; 1],
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for WLAN_AVAILABLE_NETWORK_LIST {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for WLAN_AVAILABLE_NETWORK_LIST {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for WLAN_AVAILABLE_NETWORK_LIST {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for WLAN_AVAILABLE_NETWORK_LIST {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<WLAN_AVAILABLE_NETWORK_LIST>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for WLAN_AVAILABLE_NETWORK_LIST {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for WLAN_AVAILABLE_NETWORK_LIST {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct WLAN_AVAILABLE_NETWORK_LIST_V2 {
    pub dwNumberOfItems: u32,
    pub dwIndex: u32,
    pub Network: [WLAN_AVAILABLE_NETWORK_V2; 1],
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for WLAN_AVAILABLE_NETWORK_LIST_V2 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for WLAN_AVAILABLE_NETWORK_LIST_V2 {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for WLAN_AVAILABLE_NETWORK_LIST_V2 {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for WLAN_AVAILABLE_NETWORK_LIST_V2 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<WLAN_AVAILABLE_NETWORK_LIST_V2>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for WLAN_AVAILABLE_NETWORK_LIST_V2 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for WLAN_AVAILABLE_NETWORK_LIST_V2 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct WLAN_AVAILABLE_NETWORK_V2 {
    pub strProfileName: [u16; 256],
    pub dot11Ssid: DOT11_SSID,
    pub dot11BssType: DOT11_BSS_TYPE,
    pub uNumberOfBssids: u32,
    pub bNetworkConnectable: super::super::Foundation::BOOL,
    pub wlanNotConnectableReason: u32,
    pub uNumberOfPhyTypes: u32,
    pub dot11PhyTypes: [DOT11_PHY_TYPE; 8],
    pub bMorePhyTypes: super::super::Foundation::BOOL,
    pub wlanSignalQuality: u32,
    pub bSecurityEnabled: super::super::Foundation::BOOL,
    pub dot11DefaultAuthAlgorithm: DOT11_AUTH_ALGORITHM,
    pub dot11DefaultCipherAlgorithm: DOT11_CIPHER_ALGORITHM,
    pub dwFlags: u32,
    pub AccessNetworkOptions: DOT11_ACCESSNETWORKOPTIONS,
    pub dot11HESSID: [u8; 6],
    pub VenueInfo: DOT11_VENUEINFO,
    pub dwReserved: u32,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for WLAN_AVAILABLE_NETWORK_V2 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for WLAN_AVAILABLE_NETWORK_V2 {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for WLAN_AVAILABLE_NETWORK_V2 {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for WLAN_AVAILABLE_NETWORK_V2 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<WLAN_AVAILABLE_NETWORK_V2>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for WLAN_AVAILABLE_NETWORK_V2 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for WLAN_AVAILABLE_NETWORK_V2 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct WLAN_BSS_ENTRY {
    pub dot11Ssid: DOT11_SSID,
    pub uPhyId: u32,
    pub dot11Bssid: [u8; 6],
    pub dot11BssType: DOT11_BSS_TYPE,
    pub dot11BssPhyType: DOT11_PHY_TYPE,
    pub lRssi: i32,
    pub uLinkQuality: u32,
    pub bInRegDomain: super::super::Foundation::BOOLEAN,
    pub usBeaconPeriod: u16,
    pub ullTimestamp: u64,
    pub ullHostTimestamp: u64,
    pub usCapabilityInformation: u16,
    pub ulChCenterFrequency: u32,
    pub wlanRateSet: WLAN_RATE_SET,
    pub ulIeOffset: u32,
    pub ulIeSize: u32,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for WLAN_BSS_ENTRY {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for WLAN_BSS_ENTRY {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for WLAN_BSS_ENTRY {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for WLAN_BSS_ENTRY {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<WLAN_BSS_ENTRY>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for WLAN_BSS_ENTRY {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for WLAN_BSS_ENTRY {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct WLAN_BSS_LIST {
    pub dwTotalSize: u32,
    pub dwNumberOfItems: u32,
    pub wlanBssEntries: [WLAN_BSS_ENTRY; 1],
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for WLAN_BSS_LIST {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for WLAN_BSS_LIST {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for WLAN_BSS_LIST {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for WLAN_BSS_LIST {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<WLAN_BSS_LIST>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for WLAN_BSS_LIST {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for WLAN_BSS_LIST {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
pub const WLAN_CONNECTION_ADHOC_JOIN_ONLY: u32 = 2u32;
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct WLAN_CONNECTION_ATTRIBUTES {
    pub isState: WLAN_INTERFACE_STATE,
    pub wlanConnectionMode: WLAN_CONNECTION_MODE,
    pub strProfileName: [u16; 256],
    pub wlanAssociationAttributes: WLAN_ASSOCIATION_ATTRIBUTES,
    pub wlanSecurityAttributes: WLAN_SECURITY_ATTRIBUTES,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for WLAN_CONNECTION_ATTRIBUTES {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for WLAN_CONNECTION_ATTRIBUTES {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for WLAN_CONNECTION_ATTRIBUTES {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for WLAN_CONNECTION_ATTRIBUTES {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<WLAN_CONNECTION_ATTRIBUTES>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for WLAN_CONNECTION_ATTRIBUTES {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for WLAN_CONNECTION_ATTRIBUTES {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
pub const WLAN_CONNECTION_EAPOL_PASSTHROUGH: u32 = 8u32;
pub const WLAN_CONNECTION_HIDDEN_NETWORK: u32 = 1u32;
pub const WLAN_CONNECTION_IGNORE_PRIVACY_BIT: u32 = 4u32;
pub type WLAN_CONNECTION_MODE = i32;
pub const wlan_connection_mode_profile: WLAN_CONNECTION_MODE = 0i32;
pub const wlan_connection_mode_temporary_profile: WLAN_CONNECTION_MODE = 1i32;
pub const wlan_connection_mode_discovery_secure: WLAN_CONNECTION_MODE = 2i32;
pub const wlan_connection_mode_discovery_unsecure: WLAN_CONNECTION_MODE = 3i32;
pub const wlan_connection_mode_auto: WLAN_CONNECTION_MODE = 4i32;
pub const wlan_connection_mode_invalid: WLAN_CONNECTION_MODE = 5i32;
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct WLAN_CONNECTION_NOTIFICATION_DATA {
    pub wlanConnectionMode: WLAN_CONNECTION_MODE,
    pub strProfileName: [u16; 256],
    pub dot11Ssid: DOT11_SSID,
    pub dot11BssType: DOT11_BSS_TYPE,
    pub bSecurityEnabled: super::super::Foundation::BOOL,
    pub wlanReasonCode: u32,
    pub dwFlags: WLAN_CONNECTION_NOTIFICATION_FLAGS,
    pub strProfileXml: [u16; 1],
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for WLAN_CONNECTION_NOTIFICATION_DATA {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for WLAN_CONNECTION_NOTIFICATION_DATA {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for WLAN_CONNECTION_NOTIFICATION_DATA {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for WLAN_CONNECTION_NOTIFICATION_DATA {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<WLAN_CONNECTION_NOTIFICATION_DATA>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for WLAN_CONNECTION_NOTIFICATION_DATA {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for WLAN_CONNECTION_NOTIFICATION_DATA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
pub type WLAN_CONNECTION_NOTIFICATION_FLAGS = u32;
pub const WLAN_CONNECTION_NOTIFICATION_ADHOC_NETWORK_FORMED: WLAN_CONNECTION_NOTIFICATION_FLAGS = 1u32;
pub const WLAN_CONNECTION_NOTIFICATION_CONSOLE_USER_PROFILE: WLAN_CONNECTION_NOTIFICATION_FLAGS = 4u32;
#[repr(C)]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_NetworkManagement_Ndis"))]
pub struct WLAN_CONNECTION_PARAMETERS {
    pub wlanConnectionMode: WLAN_CONNECTION_MODE,
    pub strProfile: super::super::Foundation::PWSTR,
    pub pDot11Ssid: *mut DOT11_SSID,
    pub pDesiredBssidList: *mut DOT11_BSSID_LIST,
    pub dot11BssType: DOT11_BSS_TYPE,
    pub dwFlags: u32,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_NetworkManagement_Ndis"))]
impl ::core::marker::Copy for WLAN_CONNECTION_PARAMETERS {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_NetworkManagement_Ndis"))]
impl ::core::clone::Clone for WLAN_CONNECTION_PARAMETERS {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_NetworkManagement_Ndis"))]
unsafe impl ::windows::core::Abi for WLAN_CONNECTION_PARAMETERS {
    type Abi = Self;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_NetworkManagement_Ndis"))]
impl ::core::cmp::PartialEq for WLAN_CONNECTION_PARAMETERS {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<WLAN_CONNECTION_PARAMETERS>()) == 0 }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_NetworkManagement_Ndis"))]
impl ::core::cmp::Eq for WLAN_CONNECTION_PARAMETERS {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_NetworkManagement_Ndis"))]
impl ::core::default::Default for WLAN_CONNECTION_PARAMETERS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_NetworkManagement_Ndis"))]
pub struct WLAN_CONNECTION_PARAMETERS_V2 {
    pub wlanConnectionMode: WLAN_CONNECTION_MODE,
    pub strProfile: super::super::Foundation::PWSTR,
    pub pDot11Ssid: *mut DOT11_SSID,
    pub pDot11Hessid: *mut u8,
    pub pDesiredBssidList: *mut DOT11_BSSID_LIST,
    pub dot11BssType: DOT11_BSS_TYPE,
    pub dwFlags: u32,
    pub pDot11AccessNetworkOptions: *mut DOT11_ACCESSNETWORKOPTIONS,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_NetworkManagement_Ndis"))]
impl ::core::marker::Copy for WLAN_CONNECTION_PARAMETERS_V2 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_NetworkManagement_Ndis"))]
impl ::core::clone::Clone for WLAN_CONNECTION_PARAMETERS_V2 {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_NetworkManagement_Ndis"))]
unsafe impl ::windows::core::Abi for WLAN_CONNECTION_PARAMETERS_V2 {
    type Abi = Self;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_NetworkManagement_Ndis"))]
impl ::core::cmp::PartialEq for WLAN_CONNECTION_PARAMETERS_V2 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<WLAN_CONNECTION_PARAMETERS_V2>()) == 0 }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_NetworkManagement_Ndis"))]
impl ::core::cmp::Eq for WLAN_CONNECTION_PARAMETERS_V2 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_NetworkManagement_Ndis"))]
impl ::core::default::Default for WLAN_CONNECTION_PARAMETERS_V2 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
pub const WLAN_CONNECTION_PERSIST_DISCOVERY_PROFILE: u32 = 16u32;
pub const WLAN_CONNECTION_PERSIST_DISCOVERY_PROFILE_CONNECTION_MODE_AUTO: u32 = 32u32;
pub const WLAN_CONNECTION_PERSIST_DISCOVERY_PROFILE_OVERWRITE_EXISTING: u32 = 64u32;
#[repr(C)]
pub struct WLAN_COUNTRY_OR_REGION_STRING_LIST {
    pub dwNumberOfItems: u32,
    pub pCountryOrRegionStringList: [u8; 3],
}
impl ::core::marker::Copy for WLAN_COUNTRY_OR_REGION_STRING_LIST {}
impl ::core::clone::Clone for WLAN_COUNTRY_OR_REGION_STRING_LIST {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for WLAN_COUNTRY_OR_REGION_STRING_LIST {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for WLAN_COUNTRY_OR_REGION_STRING_LIST {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<WLAN_COUNTRY_OR_REGION_STRING_LIST>()) == 0 }
    }
}
impl ::core::cmp::Eq for WLAN_COUNTRY_OR_REGION_STRING_LIST {}
impl ::core::default::Default for WLAN_COUNTRY_OR_REGION_STRING_LIST {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct WLAN_DEVICE_SERVICE_GUID_LIST {
    pub dwNumberOfItems: u32,
    pub dwIndex: u32,
    pub DeviceService: [::windows::core::GUID; 1],
}
impl ::core::marker::Copy for WLAN_DEVICE_SERVICE_GUID_LIST {}
impl ::core::clone::Clone for WLAN_DEVICE_SERVICE_GUID_LIST {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for WLAN_DEVICE_SERVICE_GUID_LIST {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for WLAN_DEVICE_SERVICE_GUID_LIST {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<WLAN_DEVICE_SERVICE_GUID_LIST>()) == 0 }
    }
}
impl ::core::cmp::Eq for WLAN_DEVICE_SERVICE_GUID_LIST {}
impl ::core::default::Default for WLAN_DEVICE_SERVICE_GUID_LIST {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct WLAN_DEVICE_SERVICE_NOTIFICATION_DATA {
    pub DeviceService: ::windows::core::GUID,
    pub dwOpCode: u32,
    pub dwDataSize: u32,
    pub DataBlob: [u8; 1],
}
impl ::core::marker::Copy for WLAN_DEVICE_SERVICE_NOTIFICATION_DATA {}
impl ::core::clone::Clone for WLAN_DEVICE_SERVICE_NOTIFICATION_DATA {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for WLAN_DEVICE_SERVICE_NOTIFICATION_DATA {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for WLAN_DEVICE_SERVICE_NOTIFICATION_DATA {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<WLAN_DEVICE_SERVICE_NOTIFICATION_DATA>()) == 0 }
    }
}
impl ::core::cmp::Eq for WLAN_DEVICE_SERVICE_NOTIFICATION_DATA {}
impl ::core::default::Default for WLAN_DEVICE_SERVICE_NOTIFICATION_DATA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
pub type WLAN_FILTER_LIST_TYPE = i32;
pub const wlan_filter_list_type_gp_permit: WLAN_FILTER_LIST_TYPE = 0i32;
pub const wlan_filter_list_type_gp_deny: WLAN_FILTER_LIST_TYPE = 1i32;
pub const wlan_filter_list_type_user_permit: WLAN_FILTER_LIST_TYPE = 2i32;
pub const wlan_filter_list_type_user_deny: WLAN_FILTER_LIST_TYPE = 3i32;
#[repr(C)]
pub struct WLAN_HOSTED_NETWORK_CONNECTION_SETTINGS {
    pub hostedNetworkSSID: DOT11_SSID,
    pub dwMaxNumberOfPeers: u32,
}
impl ::core::marker::Copy for WLAN_HOSTED_NETWORK_CONNECTION_SETTINGS {}
impl ::core::clone::Clone for WLAN_HOSTED_NETWORK_CONNECTION_SETTINGS {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for WLAN_HOSTED_NETWORK_CONNECTION_SETTINGS {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for WLAN_HOSTED_NETWORK_CONNECTION_SETTINGS {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<WLAN_HOSTED_NETWORK_CONNECTION_SETTINGS>()) == 0 }
    }
}
impl ::core::cmp::Eq for WLAN_HOSTED_NETWORK_CONNECTION_SETTINGS {}
impl ::core::default::Default for WLAN_HOSTED_NETWORK_CONNECTION_SETTINGS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct WLAN_HOSTED_NETWORK_DATA_PEER_STATE_CHANGE {
    pub OldState: WLAN_HOSTED_NETWORK_PEER_STATE,
    pub NewState: WLAN_HOSTED_NETWORK_PEER_STATE,
    pub PeerStateChangeReason: WLAN_HOSTED_NETWORK_REASON,
}
impl ::core::marker::Copy for WLAN_HOSTED_NETWORK_DATA_PEER_STATE_CHANGE {}
impl ::core::clone::Clone for WLAN_HOSTED_NETWORK_DATA_PEER_STATE_CHANGE {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for WLAN_HOSTED_NETWORK_DATA_PEER_STATE_CHANGE {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for WLAN_HOSTED_NETWORK_DATA_PEER_STATE_CHANGE {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<WLAN_HOSTED_NETWORK_DATA_PEER_STATE_CHANGE>()) == 0 }
    }
}
impl ::core::cmp::Eq for WLAN_HOSTED_NETWORK_DATA_PEER_STATE_CHANGE {}
impl ::core::default::Default for WLAN_HOSTED_NETWORK_DATA_PEER_STATE_CHANGE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
pub type WLAN_HOSTED_NETWORK_NOTIFICATION_CODE = i32;
pub const wlan_hosted_network_state_change: WLAN_HOSTED_NETWORK_NOTIFICATION_CODE = 4096i32;
pub const wlan_hosted_network_peer_state_change: WLAN_HOSTED_NETWORK_NOTIFICATION_CODE = 4097i32;
pub const wlan_hosted_network_radio_state_change: WLAN_HOSTED_NETWORK_NOTIFICATION_CODE = 4098i32;
pub type WLAN_HOSTED_NETWORK_OPCODE = i32;
pub const wlan_hosted_network_opcode_connection_settings: WLAN_HOSTED_NETWORK_OPCODE = 0i32;
pub const wlan_hosted_network_opcode_security_settings: WLAN_HOSTED_NETWORK_OPCODE = 1i32;
pub const wlan_hosted_network_opcode_station_profile: WLAN_HOSTED_NETWORK_OPCODE = 2i32;
pub const wlan_hosted_network_opcode_enable: WLAN_HOSTED_NETWORK_OPCODE = 3i32;
pub type WLAN_HOSTED_NETWORK_PEER_AUTH_STATE = i32;
pub const wlan_hosted_network_peer_state_invalid: WLAN_HOSTED_NETWORK_PEER_AUTH_STATE = 0i32;
pub const wlan_hosted_network_peer_state_authenticated: WLAN_HOSTED_NETWORK_PEER_AUTH_STATE = 1i32;
#[repr(C)]
pub struct WLAN_HOSTED_NETWORK_PEER_STATE {
    pub PeerMacAddress: [u8; 6],
    pub PeerAuthState: WLAN_HOSTED_NETWORK_PEER_AUTH_STATE,
}
impl ::core::marker::Copy for WLAN_HOSTED_NETWORK_PEER_STATE {}
impl ::core::clone::Clone for WLAN_HOSTED_NETWORK_PEER_STATE {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for WLAN_HOSTED_NETWORK_PEER_STATE {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for WLAN_HOSTED_NETWORK_PEER_STATE {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<WLAN_HOSTED_NETWORK_PEER_STATE>()) == 0 }
    }
}
impl ::core::cmp::Eq for WLAN_HOSTED_NETWORK_PEER_STATE {}
impl ::core::default::Default for WLAN_HOSTED_NETWORK_PEER_STATE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct WLAN_HOSTED_NETWORK_RADIO_STATE {
    pub dot11SoftwareRadioState: DOT11_RADIO_STATE,
    pub dot11HardwareRadioState: DOT11_RADIO_STATE,
}
impl ::core::marker::Copy for WLAN_HOSTED_NETWORK_RADIO_STATE {}
impl ::core::clone::Clone for WLAN_HOSTED_NETWORK_RADIO_STATE {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for WLAN_HOSTED_NETWORK_RADIO_STATE {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for WLAN_HOSTED_NETWORK_RADIO_STATE {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<WLAN_HOSTED_NETWORK_RADIO_STATE>()) == 0 }
    }
}
impl ::core::cmp::Eq for WLAN_HOSTED_NETWORK_RADIO_STATE {}
impl ::core::default::Default for WLAN_HOSTED_NETWORK_RADIO_STATE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
pub type WLAN_HOSTED_NETWORK_REASON = i32;
pub const wlan_hosted_network_reason_success: WLAN_HOSTED_NETWORK_REASON = 0i32;
pub const wlan_hosted_network_reason_unspecified: WLAN_HOSTED_NETWORK_REASON = 1i32;
pub const wlan_hosted_network_reason_bad_parameters: WLAN_HOSTED_NETWORK_REASON = 2i32;
pub const wlan_hosted_network_reason_service_shutting_down: WLAN_HOSTED_NETWORK_REASON = 3i32;
pub const wlan_hosted_network_reason_insufficient_resources: WLAN_HOSTED_NETWORK_REASON = 4i32;
pub const wlan_hosted_network_reason_elevation_required: WLAN_HOSTED_NETWORK_REASON = 5i32;
pub const wlan_hosted_network_reason_read_only: WLAN_HOSTED_NETWORK_REASON = 6i32;
pub const wlan_hosted_network_reason_persistence_failed: WLAN_HOSTED_NETWORK_REASON = 7i32;
pub const wlan_hosted_network_reason_crypt_error: WLAN_HOSTED_NETWORK_REASON = 8i32;
pub const wlan_hosted_network_reason_impersonation: WLAN_HOSTED_NETWORK_REASON = 9i32;
pub const wlan_hosted_network_reason_stop_before_start: WLAN_HOSTED_NETWORK_REASON = 10i32;
pub const wlan_hosted_network_reason_interface_available: WLAN_HOSTED_NETWORK_REASON = 11i32;
pub const wlan_hosted_network_reason_interface_unavailable: WLAN_HOSTED_NETWORK_REASON = 12i32;
pub const wlan_hosted_network_reason_miniport_stopped: WLAN_HOSTED_NETWORK_REASON = 13i32;
pub const wlan_hosted_network_reason_miniport_started: WLAN_HOSTED_NETWORK_REASON = 14i32;
pub const wlan_hosted_network_reason_incompatible_connection_started: WLAN_HOSTED_NETWORK_REASON = 15i32;
pub const wlan_hosted_network_reason_incompatible_connection_stopped: WLAN_HOSTED_NETWORK_REASON = 16i32;
pub const wlan_hosted_network_reason_user_action: WLAN_HOSTED_NETWORK_REASON = 17i32;
pub const wlan_hosted_network_reason_client_abort: WLAN_HOSTED_NETWORK_REASON = 18i32;
pub const wlan_hosted_network_reason_ap_start_failed: WLAN_HOSTED_NETWORK_REASON = 19i32;
pub const wlan_hosted_network_reason_peer_arrived: WLAN_HOSTED_NETWORK_REASON = 20i32;
pub const wlan_hosted_network_reason_peer_departed: WLAN_HOSTED_NETWORK_REASON = 21i32;
pub const wlan_hosted_network_reason_peer_timeout: WLAN_HOSTED_NETWORK_REASON = 22i32;
pub const wlan_hosted_network_reason_gp_denied: WLAN_HOSTED_NETWORK_REASON = 23i32;
pub const wlan_hosted_network_reason_service_unavailable: WLAN_HOSTED_NETWORK_REASON = 24i32;
pub const wlan_hosted_network_reason_device_change: WLAN_HOSTED_NETWORK_REASON = 25i32;
pub const wlan_hosted_network_reason_properties_change: WLAN_HOSTED_NETWORK_REASON = 26i32;
pub const wlan_hosted_network_reason_virtual_station_blocking_use: WLAN_HOSTED_NETWORK_REASON = 27i32;
pub const wlan_hosted_network_reason_service_available_on_virtual_station: WLAN_HOSTED_NETWORK_REASON = 28i32;
#[repr(C)]
pub struct WLAN_HOSTED_NETWORK_SECURITY_SETTINGS {
    pub dot11AuthAlgo: DOT11_AUTH_ALGORITHM,
    pub dot11CipherAlgo: DOT11_CIPHER_ALGORITHM,
}
impl ::core::marker::Copy for WLAN_HOSTED_NETWORK_SECURITY_SETTINGS {}
impl ::core::clone::Clone for WLAN_HOSTED_NETWORK_SECURITY_SETTINGS {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for WLAN_HOSTED_NETWORK_SECURITY_SETTINGS {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for WLAN_HOSTED_NETWORK_SECURITY_SETTINGS {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<WLAN_HOSTED_NETWORK_SECURITY_SETTINGS>()) == 0 }
    }
}
impl ::core::cmp::Eq for WLAN_HOSTED_NETWORK_SECURITY_SETTINGS {}
impl ::core::default::Default for WLAN_HOSTED_NETWORK_SECURITY_SETTINGS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
pub type WLAN_HOSTED_NETWORK_STATE = i32;
pub const wlan_hosted_network_unavailable: WLAN_HOSTED_NETWORK_STATE = 0i32;
pub const wlan_hosted_network_idle: WLAN_HOSTED_NETWORK_STATE = 1i32;
pub const wlan_hosted_network_active: WLAN_HOSTED_NETWORK_STATE = 2i32;
#[repr(C)]
pub struct WLAN_HOSTED_NETWORK_STATE_CHANGE {
    pub OldState: WLAN_HOSTED_NETWORK_STATE,
    pub NewState: WLAN_HOSTED_NETWORK_STATE,
    pub StateChangeReason: WLAN_HOSTED_NETWORK_REASON,
}
impl ::core::marker::Copy for WLAN_HOSTED_NETWORK_STATE_CHANGE {}
impl ::core::clone::Clone for WLAN_HOSTED_NETWORK_STATE_CHANGE {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for WLAN_HOSTED_NETWORK_STATE_CHANGE {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for WLAN_HOSTED_NETWORK_STATE_CHANGE {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<WLAN_HOSTED_NETWORK_STATE_CHANGE>()) == 0 }
    }
}
impl ::core::cmp::Eq for WLAN_HOSTED_NETWORK_STATE_CHANGE {}
impl ::core::default::Default for WLAN_HOSTED_NETWORK_STATE_CHANGE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct WLAN_HOSTED_NETWORK_STATUS {
    pub HostedNetworkState: WLAN_HOSTED_NETWORK_STATE,
    pub IPDeviceID: ::windows::core::GUID,
    pub wlanHostedNetworkBSSID: [u8; 6],
    pub dot11PhyType: DOT11_PHY_TYPE,
    pub ulChannelFrequency: u32,
    pub dwNumberOfPeers: u32,
    pub PeerList: [WLAN_HOSTED_NETWORK_PEER_STATE; 1],
}
impl ::core::marker::Copy for WLAN_HOSTED_NETWORK_STATUS {}
impl ::core::clone::Clone for WLAN_HOSTED_NETWORK_STATUS {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for WLAN_HOSTED_NETWORK_STATUS {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for WLAN_HOSTED_NETWORK_STATUS {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<WLAN_HOSTED_NETWORK_STATUS>()) == 0 }
    }
}
impl ::core::cmp::Eq for WLAN_HOSTED_NETWORK_STATUS {}
impl ::core::default::Default for WLAN_HOSTED_NETWORK_STATUS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
pub type WLAN_IHV_CONTROL_TYPE = i32;
pub const wlan_ihv_control_type_service: WLAN_IHV_CONTROL_TYPE = 0i32;
pub const wlan_ihv_control_type_driver: WLAN_IHV_CONTROL_TYPE = 1i32;
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct WLAN_INTERFACE_CAPABILITY {
    pub interfaceType: WLAN_INTERFACE_TYPE,
    pub bDot11DSupported: super::super::Foundation::BOOL,
    pub dwMaxDesiredSsidListSize: u32,
    pub dwMaxDesiredBssidListSize: u32,
    pub dwNumberOfSupportedPhys: u32,
    pub dot11PhyTypes: [DOT11_PHY_TYPE; 64],
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for WLAN_INTERFACE_CAPABILITY {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for WLAN_INTERFACE_CAPABILITY {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for WLAN_INTERFACE_CAPABILITY {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for WLAN_INTERFACE_CAPABILITY {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<WLAN_INTERFACE_CAPABILITY>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for WLAN_INTERFACE_CAPABILITY {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for WLAN_INTERFACE_CAPABILITY {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct WLAN_INTERFACE_INFO {
    pub InterfaceGuid: ::windows::core::GUID,
    pub strInterfaceDescription: [u16; 256],
    pub isState: WLAN_INTERFACE_STATE,
}
impl ::core::marker::Copy for WLAN_INTERFACE_INFO {}
impl ::core::clone::Clone for WLAN_INTERFACE_INFO {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for WLAN_INTERFACE_INFO {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for WLAN_INTERFACE_INFO {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<WLAN_INTERFACE_INFO>()) == 0 }
    }
}
impl ::core::cmp::Eq for WLAN_INTERFACE_INFO {}
impl ::core::default::Default for WLAN_INTERFACE_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct WLAN_INTERFACE_INFO_LIST {
    pub dwNumberOfItems: u32,
    pub dwIndex: u32,
    pub InterfaceInfo: [WLAN_INTERFACE_INFO; 1],
}
impl ::core::marker::Copy for WLAN_INTERFACE_INFO_LIST {}
impl ::core::clone::Clone for WLAN_INTERFACE_INFO_LIST {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for WLAN_INTERFACE_INFO_LIST {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for WLAN_INTERFACE_INFO_LIST {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<WLAN_INTERFACE_INFO_LIST>()) == 0 }
    }
}
impl ::core::cmp::Eq for WLAN_INTERFACE_INFO_LIST {}
impl ::core::default::Default for WLAN_INTERFACE_INFO_LIST {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
pub type WLAN_INTERFACE_STATE = i32;
pub const wlan_interface_state_not_ready: WLAN_INTERFACE_STATE = 0i32;
pub const wlan_interface_state_connected: WLAN_INTERFACE_STATE = 1i32;
pub const wlan_interface_state_ad_hoc_network_formed: WLAN_INTERFACE_STATE = 2i32;
pub const wlan_interface_state_disconnecting: WLAN_INTERFACE_STATE = 3i32;
pub const wlan_interface_state_disconnected: WLAN_INTERFACE_STATE = 4i32;
pub const wlan_interface_state_associating: WLAN_INTERFACE_STATE = 5i32;
pub const wlan_interface_state_discovering: WLAN_INTERFACE_STATE = 6i32;
pub const wlan_interface_state_authenticating: WLAN_INTERFACE_STATE = 7i32;
pub type WLAN_INTERFACE_TYPE = i32;
pub const wlan_interface_type_emulated_802_11: WLAN_INTERFACE_TYPE = 0i32;
pub const wlan_interface_type_native_802_11: WLAN_INTERFACE_TYPE = 1i32;
pub const wlan_interface_type_invalid: WLAN_INTERFACE_TYPE = 2i32;
pub type WLAN_INTF_OPCODE = i32;
pub const wlan_intf_opcode_autoconf_start: WLAN_INTF_OPCODE = 0i32;
pub const wlan_intf_opcode_autoconf_enabled: WLAN_INTF_OPCODE = 1i32;
pub const wlan_intf_opcode_background_scan_enabled: WLAN_INTF_OPCODE = 2i32;
pub const wlan_intf_opcode_media_streaming_mode: WLAN_INTF_OPCODE = 3i32;
pub const wlan_intf_opcode_radio_state: WLAN_INTF_OPCODE = 4i32;
pub const wlan_intf_opcode_bss_type: WLAN_INTF_OPCODE = 5i32;
pub const wlan_intf_opcode_interface_state: WLAN_INTF_OPCODE = 6i32;
pub const wlan_intf_opcode_current_connection: WLAN_INTF_OPCODE = 7i32;
pub const wlan_intf_opcode_channel_number: WLAN_INTF_OPCODE = 8i32;
pub const wlan_intf_opcode_supported_infrastructure_auth_cipher_pairs: WLAN_INTF_OPCODE = 9i32;
pub const wlan_intf_opcode_supported_adhoc_auth_cipher_pairs: WLAN_INTF_OPCODE = 10i32;
pub const wlan_intf_opcode_supported_country_or_region_string_list: WLAN_INTF_OPCODE = 11i32;
pub const wlan_intf_opcode_current_operation_mode: WLAN_INTF_OPCODE = 12i32;
pub const wlan_intf_opcode_supported_safe_mode: WLAN_INTF_OPCODE = 13i32;
pub const wlan_intf_opcode_certified_safe_mode: WLAN_INTF_OPCODE = 14i32;
pub const wlan_intf_opcode_hosted_network_capable: WLAN_INTF_OPCODE = 15i32;
pub const wlan_intf_opcode_management_frame_protection_capable: WLAN_INTF_OPCODE = 16i32;
pub const wlan_intf_opcode_secondary_sta_interfaces: WLAN_INTF_OPCODE = 17i32;
pub const wlan_intf_opcode_secondary_sta_synchronized_connections: WLAN_INTF_OPCODE = 18i32;
pub const wlan_intf_opcode_autoconf_end: WLAN_INTF_OPCODE = 268435455i32;
pub const wlan_intf_opcode_msm_start: WLAN_INTF_OPCODE = 268435712i32;
pub const wlan_intf_opcode_statistics: WLAN_INTF_OPCODE = 268435713i32;
pub const wlan_intf_opcode_rssi: WLAN_INTF_OPCODE = 268435714i32;
pub const wlan_intf_opcode_msm_end: WLAN_INTF_OPCODE = 536870911i32;
pub const wlan_intf_opcode_security_start: WLAN_INTF_OPCODE = 536936448i32;
pub const wlan_intf_opcode_security_end: WLAN_INTF_OPCODE = 805306367i32;
pub const wlan_intf_opcode_ihv_start: WLAN_INTF_OPCODE = 805306368i32;
pub const wlan_intf_opcode_ihv_end: WLAN_INTF_OPCODE = 1073741823i32;
#[repr(C)]
pub struct WLAN_MAC_FRAME_STATISTICS {
    pub ullTransmittedFrameCount: u64,
    pub ullReceivedFrameCount: u64,
    pub ullWEPExcludedCount: u64,
    pub ullTKIPLocalMICFailures: u64,
    pub ullTKIPReplays: u64,
    pub ullTKIPICVErrorCount: u64,
    pub ullCCMPReplays: u64,
    pub ullCCMPDecryptErrors: u64,
    pub ullWEPUndecryptableCount: u64,
    pub ullWEPICVErrorCount: u64,
    pub ullDecryptSuccessCount: u64,
    pub ullDecryptFailureCount: u64,
}
impl ::core::marker::Copy for WLAN_MAC_FRAME_STATISTICS {}
impl ::core::clone::Clone for WLAN_MAC_FRAME_STATISTICS {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for WLAN_MAC_FRAME_STATISTICS {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for WLAN_MAC_FRAME_STATISTICS {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<WLAN_MAC_FRAME_STATISTICS>()) == 0 }
    }
}
impl ::core::cmp::Eq for WLAN_MAC_FRAME_STATISTICS {}
impl ::core::default::Default for WLAN_MAC_FRAME_STATISTICS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
pub const WLAN_MAX_NAME_LENGTH: u32 = 256u32;
pub const WLAN_MAX_PHY_INDEX: u32 = 64u32;
pub const WLAN_MAX_PHY_TYPE_NUMBER: u32 = 8u32;
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct WLAN_MSM_NOTIFICATION_DATA {
    pub wlanConnectionMode: WLAN_CONNECTION_MODE,
    pub strProfileName: [u16; 256],
    pub dot11Ssid: DOT11_SSID,
    pub dot11BssType: DOT11_BSS_TYPE,
    pub dot11MacAddr: [u8; 6],
    pub bSecurityEnabled: super::super::Foundation::BOOL,
    pub bFirstPeer: super::super::Foundation::BOOL,
    pub bLastPeer: super::super::Foundation::BOOL,
    pub wlanReasonCode: u32,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for WLAN_MSM_NOTIFICATION_DATA {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for WLAN_MSM_NOTIFICATION_DATA {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for WLAN_MSM_NOTIFICATION_DATA {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for WLAN_MSM_NOTIFICATION_DATA {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<WLAN_MSM_NOTIFICATION_DATA>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for WLAN_MSM_NOTIFICATION_DATA {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for WLAN_MSM_NOTIFICATION_DATA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
pub type WLAN_NOTIFICATION_ACM = i32;
pub const wlan_notification_acm_start: WLAN_NOTIFICATION_ACM = 0i32;
pub const wlan_notification_acm_autoconf_enabled: WLAN_NOTIFICATION_ACM = 1i32;
pub const wlan_notification_acm_autoconf_disabled: WLAN_NOTIFICATION_ACM = 2i32;
pub const wlan_notification_acm_background_scan_enabled: WLAN_NOTIFICATION_ACM = 3i32;
pub const wlan_notification_acm_background_scan_disabled: WLAN_NOTIFICATION_ACM = 4i32;
pub const wlan_notification_acm_bss_type_change: WLAN_NOTIFICATION_ACM = 5i32;
pub const wlan_notification_acm_power_setting_change: WLAN_NOTIFICATION_ACM = 6i32;
pub const wlan_notification_acm_scan_complete: WLAN_NOTIFICATION_ACM = 7i32;
pub const wlan_notification_acm_scan_fail: WLAN_NOTIFICATION_ACM = 8i32;
pub const wlan_notification_acm_connection_start: WLAN_NOTIFICATION_ACM = 9i32;
pub const wlan_notification_acm_connection_complete: WLAN_NOTIFICATION_ACM = 10i32;
pub const wlan_notification_acm_connection_attempt_fail: WLAN_NOTIFICATION_ACM = 11i32;
pub const wlan_notification_acm_filter_list_change: WLAN_NOTIFICATION_ACM = 12i32;
pub const wlan_notification_acm_interface_arrival: WLAN_NOTIFICATION_ACM = 13i32;
pub const wlan_notification_acm_interface_removal: WLAN_NOTIFICATION_ACM = 14i32;
pub const wlan_notification_acm_profile_change: WLAN_NOTIFICATION_ACM = 15i32;
pub const wlan_notification_acm_profile_name_change: WLAN_NOTIFICATION_ACM = 16i32;
pub const wlan_notification_acm_profiles_exhausted: WLAN_NOTIFICATION_ACM = 17i32;
pub const wlan_notification_acm_network_not_available: WLAN_NOTIFICATION_ACM = 18i32;
pub const wlan_notification_acm_network_available: WLAN_NOTIFICATION_ACM = 19i32;
pub const wlan_notification_acm_disconnecting: WLAN_NOTIFICATION_ACM = 20i32;
pub const wlan_notification_acm_disconnected: WLAN_NOTIFICATION_ACM = 21i32;
pub const wlan_notification_acm_adhoc_network_state_change: WLAN_NOTIFICATION_ACM = 22i32;
pub const wlan_notification_acm_profile_unblocked: WLAN_NOTIFICATION_ACM = 23i32;
pub const wlan_notification_acm_screen_power_change: WLAN_NOTIFICATION_ACM = 24i32;
pub const wlan_notification_acm_profile_blocked: WLAN_NOTIFICATION_ACM = 25i32;
pub const wlan_notification_acm_scan_list_refresh: WLAN_NOTIFICATION_ACM = 26i32;
pub const wlan_notification_acm_operational_state_change: WLAN_NOTIFICATION_ACM = 27i32;
pub const wlan_notification_acm_end: WLAN_NOTIFICATION_ACM = 28i32;
pub type WLAN_NOTIFICATION_CALLBACK = ::core::option::Option<unsafe extern "system" fn(param0: *mut L2_NOTIFICATION_DATA, param1: *mut ::core::ffi::c_void)>;
pub type WLAN_NOTIFICATION_MSM = i32;
pub const wlan_notification_msm_start: WLAN_NOTIFICATION_MSM = 0i32;
pub const wlan_notification_msm_associating: WLAN_NOTIFICATION_MSM = 1i32;
pub const wlan_notification_msm_associated: WLAN_NOTIFICATION_MSM = 2i32;
pub const wlan_notification_msm_authenticating: WLAN_NOTIFICATION_MSM = 3i32;
pub const wlan_notification_msm_connected: WLAN_NOTIFICATION_MSM = 4i32;
pub const wlan_notification_msm_roaming_start: WLAN_NOTIFICATION_MSM = 5i32;
pub const wlan_notification_msm_roaming_end: WLAN_NOTIFICATION_MSM = 6i32;
pub const wlan_notification_msm_radio_state_change: WLAN_NOTIFICATION_MSM = 7i32;
pub const wlan_notification_msm_signal_quality_change: WLAN_NOTIFICATION_MSM = 8i32;
pub const wlan_notification_msm_disassociating: WLAN_NOTIFICATION_MSM = 9i32;
pub const wlan_notification_msm_disconnected: WLAN_NOTIFICATION_MSM = 10i32;
pub const wlan_notification_msm_peer_join: WLAN_NOTIFICATION_MSM = 11i32;
pub const wlan_notification_msm_peer_leave: WLAN_NOTIFICATION_MSM = 12i32;
pub const wlan_notification_msm_adapter_removal: WLAN_NOTIFICATION_MSM = 13i32;
pub const wlan_notification_msm_adapter_operation_mode_change: WLAN_NOTIFICATION_MSM = 14i32;
pub const wlan_notification_msm_link_degraded: WLAN_NOTIFICATION_MSM = 15i32;
pub const wlan_notification_msm_link_improved: WLAN_NOTIFICATION_MSM = 16i32;
pub const wlan_notification_msm_end: WLAN_NOTIFICATION_MSM = 17i32;
pub type WLAN_NOTIFICATION_SECURITY = i32;
pub const wlan_notification_security_start: WLAN_NOTIFICATION_SECURITY = 0i32;
pub const wlan_notification_security_end: WLAN_NOTIFICATION_SECURITY = 1i32;
pub const WLAN_NOTIFICATION_SOURCE_ACM: u32 = 8u32;
pub const WLAN_NOTIFICATION_SOURCE_ALL: u32 = 65535u32;
pub const WLAN_NOTIFICATION_SOURCE_DEVICE_SERVICE: u32 = 2048u32;
pub const WLAN_NOTIFICATION_SOURCE_HNWK: u32 = 128u32;
pub const WLAN_NOTIFICATION_SOURCE_IHV: u32 = 64u32;
pub const WLAN_NOTIFICATION_SOURCE_MSM: u32 = 16u32;
pub const WLAN_NOTIFICATION_SOURCE_NONE: u32 = 0u32;
pub const WLAN_NOTIFICATION_SOURCE_ONEX: u32 = 4u32;
pub const WLAN_NOTIFICATION_SOURCE_SECURITY: u32 = 32u32;
pub type WLAN_OPCODE_VALUE_TYPE = i32;
pub const wlan_opcode_value_type_query_only: WLAN_OPCODE_VALUE_TYPE = 0i32;
pub const wlan_opcode_value_type_set_by_group_policy: WLAN_OPCODE_VALUE_TYPE = 1i32;
pub const wlan_opcode_value_type_set_by_user: WLAN_OPCODE_VALUE_TYPE = 2i32;
pub const wlan_opcode_value_type_invalid: WLAN_OPCODE_VALUE_TYPE = 3i32;
pub type WLAN_OPERATIONAL_STATE = i32;
pub const wlan_operational_state_unknown: WLAN_OPERATIONAL_STATE = 0i32;
pub const wlan_operational_state_off: WLAN_OPERATIONAL_STATE = 1i32;
pub const wlan_operational_state_on: WLAN_OPERATIONAL_STATE = 2i32;
pub const wlan_operational_state_going_off: WLAN_OPERATIONAL_STATE = 3i32;
pub const wlan_operational_state_going_on: WLAN_OPERATIONAL_STATE = 4i32;
#[repr(C)]
pub struct WLAN_PHY_FRAME_STATISTICS {
    pub ullTransmittedFrameCount: u64,
    pub ullMulticastTransmittedFrameCount: u64,
    pub ullFailedCount: u64,
    pub ullRetryCount: u64,
    pub ullMultipleRetryCount: u64,
    pub ullMaxTXLifetimeExceededCount: u64,
    pub ullTransmittedFragmentCount: u64,
    pub ullRTSSuccessCount: u64,
    pub ullRTSFailureCount: u64,
    pub ullACKFailureCount: u64,
    pub ullReceivedFrameCount: u64,
    pub ullMulticastReceivedFrameCount: u64,
    pub ullPromiscuousReceivedFrameCount: u64,
    pub ullMaxRXLifetimeExceededCount: u64,
    pub ullFrameDuplicateCount: u64,
    pub ullReceivedFragmentCount: u64,
    pub ullPromiscuousReceivedFragmentCount: u64,
    pub ullFCSErrorCount: u64,
}
impl ::core::marker::Copy for WLAN_PHY_FRAME_STATISTICS {}
impl ::core::clone::Clone for WLAN_PHY_FRAME_STATISTICS {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for WLAN_PHY_FRAME_STATISTICS {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for WLAN_PHY_FRAME_STATISTICS {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<WLAN_PHY_FRAME_STATISTICS>()) == 0 }
    }
}
impl ::core::cmp::Eq for WLAN_PHY_FRAME_STATISTICS {}
impl ::core::default::Default for WLAN_PHY_FRAME_STATISTICS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct WLAN_PHY_RADIO_STATE {
    pub dwPhyIndex: u32,
    pub dot11SoftwareRadioState: DOT11_RADIO_STATE,
    pub dot11HardwareRadioState: DOT11_RADIO_STATE,
}
impl ::core::marker::Copy for WLAN_PHY_RADIO_STATE {}
impl ::core::clone::Clone for WLAN_PHY_RADIO_STATE {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for WLAN_PHY_RADIO_STATE {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for WLAN_PHY_RADIO_STATE {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<WLAN_PHY_RADIO_STATE>()) == 0 }
    }
}
impl ::core::cmp::Eq for WLAN_PHY_RADIO_STATE {}
impl ::core::default::Default for WLAN_PHY_RADIO_STATE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
pub type WLAN_POWER_SETTING = i32;
pub const wlan_power_setting_no_saving: WLAN_POWER_SETTING = 0i32;
pub const wlan_power_setting_low_saving: WLAN_POWER_SETTING = 1i32;
pub const wlan_power_setting_medium_saving: WLAN_POWER_SETTING = 2i32;
pub const wlan_power_setting_maximum_saving: WLAN_POWER_SETTING = 3i32;
pub const wlan_power_setting_invalid: WLAN_POWER_SETTING = 4i32;
pub const WLAN_PROFILE_CONNECTION_MODE_AUTO: u32 = 131072u32;
pub const WLAN_PROFILE_CONNECTION_MODE_SET_BY_CLIENT: u32 = 65536u32;
pub const WLAN_PROFILE_GET_PLAINTEXT_KEY: u32 = 4u32;
pub const WLAN_PROFILE_GROUP_POLICY: u32 = 1u32;
#[repr(C)]
pub struct WLAN_PROFILE_INFO {
    pub strProfileName: [u16; 256],
    pub dwFlags: u32,
}
impl ::core::marker::Copy for WLAN_PROFILE_INFO {}
impl ::core::clone::Clone for WLAN_PROFILE_INFO {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for WLAN_PROFILE_INFO {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for WLAN_PROFILE_INFO {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<WLAN_PROFILE_INFO>()) == 0 }
    }
}
impl ::core::cmp::Eq for WLAN_PROFILE_INFO {}
impl ::core::default::Default for WLAN_PROFILE_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct WLAN_PROFILE_INFO_LIST {
    pub dwNumberOfItems: u32,
    pub dwIndex: u32,
    pub ProfileInfo: [WLAN_PROFILE_INFO; 1],
}
impl ::core::marker::Copy for WLAN_PROFILE_INFO_LIST {}
impl ::core::clone::Clone for WLAN_PROFILE_INFO_LIST {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for WLAN_PROFILE_INFO_LIST {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for WLAN_PROFILE_INFO_LIST {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<WLAN_PROFILE_INFO_LIST>()) == 0 }
    }
}
impl ::core::cmp::Eq for WLAN_PROFILE_INFO_LIST {}
impl ::core::default::Default for WLAN_PROFILE_INFO_LIST {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
pub const WLAN_PROFILE_USER: u32 = 2u32;
#[repr(C)]
pub struct WLAN_RADIO_STATE {
    pub dwNumberOfPhys: u32,
    pub PhyRadioState: [WLAN_PHY_RADIO_STATE; 64],
}
impl ::core::marker::Copy for WLAN_RADIO_STATE {}
impl ::core::clone::Clone for WLAN_RADIO_STATE {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for WLAN_RADIO_STATE {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for WLAN_RADIO_STATE {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<WLAN_RADIO_STATE>()) == 0 }
    }
}
impl ::core::cmp::Eq for WLAN_RADIO_STATE {}
impl ::core::default::Default for WLAN_RADIO_STATE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct WLAN_RATE_SET {
    pub uRateSetLength: u32,
    pub usRateSet: [u16; 126],
}
impl ::core::marker::Copy for WLAN_RATE_SET {}
impl ::core::clone::Clone for WLAN_RATE_SET {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for WLAN_RATE_SET {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for WLAN_RATE_SET {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<WLAN_RATE_SET>()) == 0 }
    }
}
impl ::core::cmp::Eq for WLAN_RATE_SET {}
impl ::core::default::Default for WLAN_RATE_SET {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct WLAN_RAW_DATA {
    pub dwDataSize: u32,
    pub DataBlob: [u8; 1],
}
impl ::core::marker::Copy for WLAN_RAW_DATA {}
impl ::core::clone::Clone for WLAN_RAW_DATA {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for WLAN_RAW_DATA {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for WLAN_RAW_DATA {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<WLAN_RAW_DATA>()) == 0 }
    }
}
impl ::core::cmp::Eq for WLAN_RAW_DATA {}
impl ::core::default::Default for WLAN_RAW_DATA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct WLAN_RAW_DATA_LIST {
    pub dwTotalSize: u32,
    pub dwNumberOfItems: u32,
    pub DataList: [WLAN_RAW_DATA_LIST_0; 1],
}
impl ::core::marker::Copy for WLAN_RAW_DATA_LIST {}
impl ::core::clone::Clone for WLAN_RAW_DATA_LIST {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for WLAN_RAW_DATA_LIST {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for WLAN_RAW_DATA_LIST {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<WLAN_RAW_DATA_LIST>()) == 0 }
    }
}
impl ::core::cmp::Eq for WLAN_RAW_DATA_LIST {}
impl ::core::default::Default for WLAN_RAW_DATA_LIST {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct WLAN_RAW_DATA_LIST_0 {
    pub dwDataOffset: u32,
    pub dwDataSize: u32,
}
impl ::core::marker::Copy for WLAN_RAW_DATA_LIST_0 {}
impl ::core::clone::Clone for WLAN_RAW_DATA_LIST_0 {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for WLAN_RAW_DATA_LIST_0 {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for WLAN_RAW_DATA_LIST_0 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<WLAN_RAW_DATA_LIST_0>()) == 0 }
    }
}
impl ::core::cmp::Eq for WLAN_RAW_DATA_LIST_0 {}
impl ::core::default::Default for WLAN_RAW_DATA_LIST_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
pub const WLAN_REASON_CODE_AC_BASE: u32 = 131072u32;
pub const WLAN_REASON_CODE_AC_CONNECT_BASE: u32 = 163840u32;
pub const WLAN_REASON_CODE_AC_END: u32 = 196607u32;
pub const WLAN_REASON_CODE_ADHOC_SECURITY_FAILURE: u32 = 229386u32;
pub const WLAN_REASON_CODE_AP_PROFILE_NOT_ALLOWED: u32 = 163856u32;
pub const WLAN_REASON_CODE_AP_PROFILE_NOT_ALLOWED_FOR_CLIENT: u32 = 163855u32;
pub const WLAN_REASON_CODE_AP_STARTING_FAILURE: u32 = 229395u32;
pub const WLAN_REASON_CODE_ASSOCIATION_FAILURE: u32 = 229378u32;
pub const WLAN_REASON_CODE_ASSOCIATION_TIMEOUT: u32 = 229379u32;
pub const WLAN_REASON_CODE_AUTO_AP_PROFILE_NOT_ALLOWED: u32 = 524313u32;
pub const WLAN_REASON_CODE_AUTO_CONNECTION_NOT_ALLOWED: u32 = 524314u32;
pub const WLAN_REASON_CODE_AUTO_SWITCH_SET_FOR_ADHOC: u32 = 524304u32;
pub const WLAN_REASON_CODE_AUTO_SWITCH_SET_FOR_MANUAL_CONNECTION: u32 = 524305u32;
pub const WLAN_REASON_CODE_BAD_MAX_NUMBER_OF_CLIENTS_FOR_AP: u32 = 524310u32;
pub const WLAN_REASON_CODE_BASE: u32 = 131072u32;
pub const WLAN_REASON_CODE_BSS_TYPE_NOT_ALLOWED: u32 = 163845u32;
pub const WLAN_REASON_CODE_BSS_TYPE_UNMATCH: u32 = 196611u32;
pub const WLAN_REASON_CODE_CONFLICT_SECURITY: u32 = 524299u32;
pub const WLAN_REASON_CODE_CONNECT_CALL_FAIL: u32 = 163849u32;
pub const WLAN_REASON_CODE_DATARATE_UNMATCH: u32 = 196613u32;
pub const WLAN_REASON_CODE_DISCONNECT_TIMEOUT: u32 = 229391u32;
pub const WLAN_REASON_CODE_DRIVER_DISCONNECTED: u32 = 229387u32;
pub const WLAN_REASON_CODE_DRIVER_OPERATION_FAILURE: u32 = 229388u32;
pub const WLAN_REASON_CODE_GP_DENIED: u32 = 163843u32;
pub const WLAN_REASON_CODE_HOTSPOT2_PROFILE_DENIED: u32 = 163857u32;
pub const WLAN_REASON_CODE_HOTSPOT2_PROFILE_NOT_ALLOWED: u32 = 524315u32;
pub const WLAN_REASON_CODE_IHV_CONNECTIVITY_NOT_SUPPORTED: u32 = 524309u32;
pub const WLAN_REASON_CODE_IHV_NOT_AVAILABLE: u32 = 229389u32;
pub const WLAN_REASON_CODE_IHV_NOT_RESPONDING: u32 = 229390u32;
pub const WLAN_REASON_CODE_IHV_OUI_MISMATCH: u32 = 524296u32;
pub const WLAN_REASON_CODE_IHV_OUI_MISSING: u32 = 524297u32;
pub const WLAN_REASON_CODE_IHV_SECURITY_NOT_SUPPORTED: u32 = 524295u32;
pub const WLAN_REASON_CODE_IHV_SECURITY_ONEX_MISSING: u32 = 524306u32;
pub const WLAN_REASON_CODE_IHV_SETTINGS_MISSING: u32 = 524298u32;
pub const WLAN_REASON_CODE_INTERNAL_FAILURE: u32 = 229392u32;
pub const WLAN_REASON_CODE_INVALID_ADHOC_CONNECTION_MODE: u32 = 524302u32;
pub const WLAN_REASON_CODE_INVALID_BSS_TYPE: u32 = 524301u32;
pub const WLAN_REASON_CODE_INVALID_CHANNEL: u32 = 524311u32;
pub const WLAN_REASON_CODE_INVALID_PHY_TYPE: u32 = 524293u32;
pub const WLAN_REASON_CODE_INVALID_PROFILE_NAME: u32 = 524291u32;
pub const WLAN_REASON_CODE_INVALID_PROFILE_SCHEMA: u32 = 524289u32;
pub const WLAN_REASON_CODE_INVALID_PROFILE_TYPE: u32 = 524292u32;
pub const WLAN_REASON_CODE_IN_BLOCKED_LIST: u32 = 163847u32;
pub const WLAN_REASON_CODE_IN_FAILED_LIST: u32 = 163846u32;
pub const WLAN_REASON_CODE_KEY_MISMATCH: u32 = 163853u32;
pub const WLAN_REASON_CODE_MSMSEC_AUTH_START_TIMEOUT: u32 = 294914u32;
pub const WLAN_REASON_CODE_MSMSEC_AUTH_SUCCESS_TIMEOUT: u32 = 294915u32;
pub const WLAN_REASON_CODE_MSMSEC_AUTH_WCN_COMPLETED: u32 = 294937u32;
pub const WLAN_REASON_CODE_MSMSEC_BASE: u32 = 262144u32;
pub const WLAN_REASON_CODE_MSMSEC_CANCELLED: u32 = 294929u32;
pub const WLAN_REASON_CODE_MSMSEC_CAPABILITY_DISCOVERY: u32 = 262165u32;
pub const WLAN_REASON_CODE_MSMSEC_CAPABILITY_MFP_NW_NIC: u32 = 262181u32;
pub const WLAN_REASON_CODE_MSMSEC_CAPABILITY_NETWORK: u32 = 262162u32;
pub const WLAN_REASON_CODE_MSMSEC_CAPABILITY_NIC: u32 = 262163u32;
pub const WLAN_REASON_CODE_MSMSEC_CAPABILITY_PROFILE: u32 = 262164u32;
pub const WLAN_REASON_CODE_MSMSEC_CAPABILITY_PROFILE_AUTH: u32 = 262174u32;
pub const WLAN_REASON_CODE_MSMSEC_CAPABILITY_PROFILE_CIPHER: u32 = 262175u32;
pub const WLAN_REASON_CODE_MSMSEC_CAPABILITY_PROFILE_SAFE_MODE_NIC: u32 = 262177u32;
pub const WLAN_REASON_CODE_MSMSEC_CAPABILITY_PROFILE_SAFE_MODE_NW: u32 = 262178u32;
pub const WLAN_REASON_CODE_MSMSEC_CONNECT_BASE: u32 = 294912u32;
pub const WLAN_REASON_CODE_MSMSEC_DOWNGRADE_DETECTED: u32 = 294931u32;
pub const WLAN_REASON_CODE_MSMSEC_END: u32 = 327679u32;
pub const WLAN_REASON_CODE_MSMSEC_FORCED_FAILURE: u32 = 294933u32;
pub const WLAN_REASON_CODE_MSMSEC_G1_MISSING_GRP_KEY: u32 = 294925u32;
pub const WLAN_REASON_CODE_MSMSEC_G1_MISSING_KEY_DATA: u32 = 294924u32;
pub const WLAN_REASON_CODE_MSMSEC_G1_MISSING_MGMT_GRP_KEY: u32 = 294939u32;
pub const WLAN_REASON_CODE_MSMSEC_KEY_FORMAT: u32 = 294930u32;
pub const WLAN_REASON_CODE_MSMSEC_KEY_START_TIMEOUT: u32 = 294916u32;
pub const WLAN_REASON_CODE_MSMSEC_KEY_SUCCESS_TIMEOUT: u32 = 294917u32;
pub const WLAN_REASON_CODE_MSMSEC_M2_MISSING_IE: u32 = 294936u32;
pub const WLAN_REASON_CODE_MSMSEC_M2_MISSING_KEY_DATA: u32 = 294935u32;
pub const WLAN_REASON_CODE_MSMSEC_M3_MISSING_GRP_KEY: u32 = 294920u32;
pub const WLAN_REASON_CODE_MSMSEC_M3_MISSING_IE: u32 = 294919u32;
pub const WLAN_REASON_CODE_MSMSEC_M3_MISSING_KEY_DATA: u32 = 294918u32;
pub const WLAN_REASON_CODE_MSMSEC_M3_MISSING_MGMT_GRP_KEY: u32 = 294938u32;
pub const WLAN_REASON_CODE_MSMSEC_M3_TOO_MANY_RSNIE: u32 = 294934u32;
pub const WLAN_REASON_CODE_MSMSEC_MAX: u32 = 327679u32;
pub const WLAN_REASON_CODE_MSMSEC_MIN: u32 = 262144u32;
pub const WLAN_REASON_CODE_MSMSEC_MIXED_CELL: u32 = 262169u32;
pub const WLAN_REASON_CODE_MSMSEC_NIC_FAILURE: u32 = 294928u32;
pub const WLAN_REASON_CODE_MSMSEC_NO_AUTHENTICATOR: u32 = 294927u32;
pub const WLAN_REASON_CODE_MSMSEC_NO_PAIRWISE_KEY: u32 = 294923u32;
pub const WLAN_REASON_CODE_MSMSEC_PEER_INDICATED_INSECURE: u32 = 294926u32;
pub const WLAN_REASON_CODE_MSMSEC_PROFILE_AUTH_TIMERS_INVALID: u32 = 262170u32;
pub const WLAN_REASON_CODE_MSMSEC_PROFILE_DUPLICATE_AUTH_CIPHER: u32 = 262151u32;
pub const WLAN_REASON_CODE_MSMSEC_PROFILE_INVALID_AUTH_CIPHER: u32 = 262153u32;
pub const WLAN_REASON_CODE_MSMSEC_PROFILE_INVALID_GKEY_INTV: u32 = 262171u32;
pub const WLAN_REASON_CODE_MSMSEC_PROFILE_INVALID_KEY_INDEX: u32 = 262145u32;
pub const WLAN_REASON_CODE_MSMSEC_PROFILE_INVALID_PMKCACHE_MODE: u32 = 262156u32;
pub const WLAN_REASON_CODE_MSMSEC_PROFILE_INVALID_PMKCACHE_SIZE: u32 = 262157u32;
pub const WLAN_REASON_CODE_MSMSEC_PROFILE_INVALID_PMKCACHE_TTL: u32 = 262158u32;
pub const WLAN_REASON_CODE_MSMSEC_PROFILE_INVALID_PREAUTH_MODE: u32 = 262159u32;
pub const WLAN_REASON_CODE_MSMSEC_PROFILE_INVALID_PREAUTH_THROTTLE: u32 = 262160u32;
pub const WLAN_REASON_CODE_MSMSEC_PROFILE_KEYMATERIAL_CHAR: u32 = 262167u32;
pub const WLAN_REASON_CODE_MSMSEC_PROFILE_KEY_LENGTH: u32 = 262147u32;
pub const WLAN_REASON_CODE_MSMSEC_PROFILE_KEY_UNMAPPED_CHAR: u32 = 262173u32;
pub const WLAN_REASON_CODE_MSMSEC_PROFILE_NO_AUTH_CIPHER_SPECIFIED: u32 = 262149u32;
pub const WLAN_REASON_CODE_MSMSEC_PROFILE_ONEX_DISABLED: u32 = 262154u32;
pub const WLAN_REASON_CODE_MSMSEC_PROFILE_ONEX_ENABLED: u32 = 262155u32;
pub const WLAN_REASON_CODE_MSMSEC_PROFILE_PASSPHRASE_CHAR: u32 = 262166u32;
pub const WLAN_REASON_CODE_MSMSEC_PROFILE_PREAUTH_ONLY_ENABLED: u32 = 262161u32;
pub const WLAN_REASON_CODE_MSMSEC_PROFILE_PSK_LENGTH: u32 = 262148u32;
pub const WLAN_REASON_CODE_MSMSEC_PROFILE_PSK_PRESENT: u32 = 262146u32;
pub const WLAN_REASON_CODE_MSMSEC_PROFILE_RAWDATA_INVALID: u32 = 262152u32;
pub const WLAN_REASON_CODE_MSMSEC_PROFILE_SAFE_MODE: u32 = 262176u32;
pub const WLAN_REASON_CODE_MSMSEC_PROFILE_TOO_MANY_AUTH_CIPHER_SPECIFIED: u32 = 262150u32;
pub const WLAN_REASON_CODE_MSMSEC_PROFILE_UNSUPPORTED_AUTH: u32 = 262179u32;
pub const WLAN_REASON_CODE_MSMSEC_PROFILE_UNSUPPORTED_CIPHER: u32 = 262180u32;
pub const WLAN_REASON_CODE_MSMSEC_PROFILE_WRONG_KEYTYPE: u32 = 262168u32;
pub const WLAN_REASON_CODE_MSMSEC_PR_IE_MATCHING: u32 = 294921u32;
pub const WLAN_REASON_CODE_MSMSEC_PSK_MISMATCH_SUSPECTED: u32 = 294932u32;
pub const WLAN_REASON_CODE_MSMSEC_SEC_IE_MATCHING: u32 = 294922u32;
pub const WLAN_REASON_CODE_MSMSEC_TRANSITION_NETWORK: u32 = 262172u32;
pub const WLAN_REASON_CODE_MSMSEC_UI_REQUEST_FAILURE: u32 = 294913u32;
pub const WLAN_REASON_CODE_MSM_BASE: u32 = 196608u32;
pub const WLAN_REASON_CODE_MSM_CONNECT_BASE: u32 = 229376u32;
pub const WLAN_REASON_CODE_MSM_END: u32 = 262143u32;
pub const WLAN_REASON_CODE_MSM_SECURITY_MISSING: u32 = 524294u32;
pub const WLAN_REASON_CODE_NETWORK_NOT_AVAILABLE: u32 = 163851u32;
pub const WLAN_REASON_CODE_NETWORK_NOT_COMPATIBLE: u32 = 131073u32;
pub const WLAN_REASON_CODE_NON_BROADCAST_SET_FOR_ADHOC: u32 = 524303u32;
pub const WLAN_REASON_CODE_NOT_VISIBLE: u32 = 163842u32;
pub const WLAN_REASON_CODE_NO_AUTO_CONNECTION: u32 = 163841u32;
pub const WLAN_REASON_CODE_NO_VISIBLE_AP: u32 = 229396u32;
pub const WLAN_REASON_CODE_OPERATION_MODE_NOT_SUPPORTED: u32 = 524312u32;
pub const WLAN_REASON_CODE_PHY_TYPE_UNMATCH: u32 = 196612u32;
pub const WLAN_REASON_CODE_PRE_SECURITY_FAILURE: u32 = 229380u32;
pub const WLAN_REASON_CODE_PROFILE_BASE: u32 = 524288u32;
pub const WLAN_REASON_CODE_PROFILE_CHANGED_OR_DELETED: u32 = 163852u32;
pub const WLAN_REASON_CODE_PROFILE_CONNECT_BASE: u32 = 557056u32;
pub const WLAN_REASON_CODE_PROFILE_END: u32 = 589823u32;
pub const WLAN_REASON_CODE_PROFILE_MISSING: u32 = 524290u32;
pub const WLAN_REASON_CODE_PROFILE_NOT_COMPATIBLE: u32 = 131074u32;
pub const WLAN_REASON_CODE_PROFILE_SSID_INVALID: u32 = 524307u32;
pub const WLAN_REASON_CODE_RANGE_SIZE: u32 = 65536u32;
pub const WLAN_REASON_CODE_RESERVED_BASE: u32 = 720896u32;
pub const WLAN_REASON_CODE_RESERVED_END: u32 = 786431u32;
pub const WLAN_REASON_CODE_ROAMING_FAILURE: u32 = 229384u32;
pub const WLAN_REASON_CODE_ROAMING_SECURITY_FAILURE: u32 = 229385u32;
pub const WLAN_REASON_CODE_SCAN_CALL_FAIL: u32 = 163850u32;
pub const WLAN_REASON_CODE_SECURITY_FAILURE: u32 = 229382u32;
pub const WLAN_REASON_CODE_SECURITY_MISSING: u32 = 524300u32;
pub const WLAN_REASON_CODE_SECURITY_TIMEOUT: u32 = 229383u32;
pub const WLAN_REASON_CODE_SSID_LIST_TOO_LONG: u32 = 163848u32;
pub const WLAN_REASON_CODE_START_SECURITY_FAILURE: u32 = 229381u32;
pub const WLAN_REASON_CODE_SUCCESS: u32 = 0u32;
pub const WLAN_REASON_CODE_TOO_MANY_SECURITY_ATTEMPTS: u32 = 229394u32;
pub const WLAN_REASON_CODE_TOO_MANY_SSID: u32 = 524308u32;
pub const WLAN_REASON_CODE_UI_REQUEST_TIMEOUT: u32 = 229393u32;
pub const WLAN_REASON_CODE_UNKNOWN: u32 = 65537u32;
pub const WLAN_REASON_CODE_UNSUPPORTED_SECURITY_SET: u32 = 196610u32;
pub const WLAN_REASON_CODE_UNSUPPORTED_SECURITY_SET_BY_OS: u32 = 196609u32;
pub const WLAN_REASON_CODE_USER_CANCELLED: u32 = 229377u32;
pub const WLAN_REASON_CODE_USER_DENIED: u32 = 163844u32;
pub const WLAN_REASON_CODE_USER_NOT_RESPOND: u32 = 163854u32;
pub type WLAN_SECURABLE_OBJECT = i32;
pub const wlan_secure_permit_list: WLAN_SECURABLE_OBJECT = 0i32;
pub const wlan_secure_deny_list: WLAN_SECURABLE_OBJECT = 1i32;
pub const wlan_secure_ac_enabled: WLAN_SECURABLE_OBJECT = 2i32;
pub const wlan_secure_bc_scan_enabled: WLAN_SECURABLE_OBJECT = 3i32;
pub const wlan_secure_bss_type: WLAN_SECURABLE_OBJECT = 4i32;
pub const wlan_secure_show_denied: WLAN_SECURABLE_OBJECT = 5i32;
pub const wlan_secure_interface_properties: WLAN_SECURABLE_OBJECT = 6i32;
pub const wlan_secure_ihv_control: WLAN_SECURABLE_OBJECT = 7i32;
pub const wlan_secure_all_user_profiles_order: WLAN_SECURABLE_OBJECT = 8i32;
pub const wlan_secure_add_new_all_user_profiles: WLAN_SECURABLE_OBJECT = 9i32;
pub const wlan_secure_add_new_per_user_profiles: WLAN_SECURABLE_OBJECT = 10i32;
pub const wlan_secure_media_streaming_mode_enabled: WLAN_SECURABLE_OBJECT = 11i32;
pub const wlan_secure_current_operation_mode: WLAN_SECURABLE_OBJECT = 12i32;
pub const wlan_secure_get_plaintext_key: WLAN_SECURABLE_OBJECT = 13i32;
pub const wlan_secure_hosted_network_elevated_access: WLAN_SECURABLE_OBJECT = 14i32;
pub const wlan_secure_virtual_station_extensibility: WLAN_SECURABLE_OBJECT = 15i32;
pub const wlan_secure_wfd_elevated_access: WLAN_SECURABLE_OBJECT = 16i32;
pub const WLAN_SECURABLE_OBJECT_COUNT: WLAN_SECURABLE_OBJECT = 17i32;
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct WLAN_SECURITY_ATTRIBUTES {
    pub bSecurityEnabled: super::super::Foundation::BOOL,
    pub bOneXEnabled: super::super::Foundation::BOOL,
    pub dot11AuthAlgorithm: DOT11_AUTH_ALGORITHM,
    pub dot11CipherAlgorithm: DOT11_CIPHER_ALGORITHM,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for WLAN_SECURITY_ATTRIBUTES {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for WLAN_SECURITY_ATTRIBUTES {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for WLAN_SECURITY_ATTRIBUTES {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for WLAN_SECURITY_ATTRIBUTES {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<WLAN_SECURITY_ATTRIBUTES>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for WLAN_SECURITY_ATTRIBUTES {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for WLAN_SECURITY_ATTRIBUTES {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
pub type WLAN_SET_EAPHOST_FLAGS = u32;
pub const WLAN_SET_EAPHOST_DATA_ALL_USERS: WLAN_SET_EAPHOST_FLAGS = 1u32;
#[repr(C)]
pub struct WLAN_STATISTICS {
    pub ullFourWayHandshakeFailures: u64,
    pub ullTKIPCounterMeasuresInvoked: u64,
    pub ullReserved: u64,
    pub MacUcastCounters: WLAN_MAC_FRAME_STATISTICS,
    pub MacMcastCounters: WLAN_MAC_FRAME_STATISTICS,
    pub dwNumberOfPhys: u32,
    pub PhyCounters: [WLAN_PHY_FRAME_STATISTICS; 1],
}
impl ::core::marker::Copy for WLAN_STATISTICS {}
impl ::core::clone::Clone for WLAN_STATISTICS {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for WLAN_STATISTICS {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for WLAN_STATISTICS {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<WLAN_STATISTICS>()) == 0 }
    }
}
impl ::core::cmp::Eq for WLAN_STATISTICS {}
impl ::core::default::Default for WLAN_STATISTICS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
pub const WLAN_UI_API_INITIAL_VERSION: u32 = 1u32;
pub const WLAN_UI_API_VERSION: u32 = 1u32;
pub type WL_DISPLAY_PAGES = i32;
pub const WLConnectionPage: WL_DISPLAY_PAGES = 0i32;
pub const WLSecurityPage: WL_DISPLAY_PAGES = 1i32;
pub const WLAdvPage: WL_DISPLAY_PAGES = 2i32;
#[inline]
pub unsafe fn WlanAllocateMemory(dwmemorysize: u32) -> *mut ::core::ffi::c_void {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn WlanAllocateMemory(dwmemorysize: u32) -> *mut ::core::ffi::c_void;
        }
        ::core::mem::transmute(WlanAllocateMemory(::core::mem::transmute(dwmemorysize)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn WlanCloseHandle<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::HANDLE>>(hclienthandle: Param0, preserved: *mut ::core::ffi::c_void) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn WlanCloseHandle(hclienthandle: super::super::Foundation::HANDLE, preserved: *mut ::core::ffi::c_void) -> u32;
        }
        ::core::mem::transmute(WlanCloseHandle(hclienthandle.into_param().abi(), ::core::mem::transmute(preserved)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_NetworkManagement_Ndis"))]
#[inline]
pub unsafe fn WlanConnect<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::HANDLE>>(hclienthandle: Param0, pinterfaceguid: *const ::windows::core::GUID, pconnectionparameters: *const WLAN_CONNECTION_PARAMETERS, preserved: *mut ::core::ffi::c_void) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn WlanConnect(hclienthandle: super::super::Foundation::HANDLE, pinterfaceguid: *const ::windows::core::GUID, pconnectionparameters: *const WLAN_CONNECTION_PARAMETERS, preserved: *mut ::core::ffi::c_void) -> u32;
        }
        ::core::mem::transmute(WlanConnect(hclienthandle.into_param().abi(), ::core::mem::transmute(pinterfaceguid), ::core::mem::transmute(pconnectionparameters), ::core::mem::transmute(preserved)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_NetworkManagement_Ndis"))]
#[inline]
pub unsafe fn WlanConnect2<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::HANDLE>>(hclienthandle: Param0, pinterfaceguid: *const ::windows::core::GUID, pconnectionparameters: *const WLAN_CONNECTION_PARAMETERS_V2, preserved: *mut ::core::ffi::c_void) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn WlanConnect2(hclienthandle: super::super::Foundation::HANDLE, pinterfaceguid: *const ::windows::core::GUID, pconnectionparameters: *const WLAN_CONNECTION_PARAMETERS_V2, preserved: *mut ::core::ffi::c_void) -> u32;
        }
        ::core::mem::transmute(WlanConnect2(hclienthandle.into_param().abi(), ::core::mem::transmute(pinterfaceguid), ::core::mem::transmute(pconnectionparameters), ::core::mem::transmute(preserved)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn WlanDeleteProfile<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::HANDLE>, Param2: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(hclienthandle: Param0, pinterfaceguid: *const ::windows::core::GUID, strprofilename: Param2, preserved: *mut ::core::ffi::c_void) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn WlanDeleteProfile(hclienthandle: super::super::Foundation::HANDLE, pinterfaceguid: *const ::windows::core::GUID, strprofilename: super::super::Foundation::PWSTR, preserved: *mut ::core::ffi::c_void) -> u32;
        }
        ::core::mem::transmute(WlanDeleteProfile(hclienthandle.into_param().abi(), ::core::mem::transmute(pinterfaceguid), strprofilename.into_param().abi(), ::core::mem::transmute(preserved)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn WlanDeviceServiceCommand<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::HANDLE>>(hclienthandle: Param0, pinterfaceguid: *const ::windows::core::GUID, pdeviceserviceguid: *const ::windows::core::GUID, dwopcode: u32, dwinbuffersize: u32, pinbuffer: *const ::core::ffi::c_void, dwoutbuffersize: u32, poutbuffer: *mut ::core::ffi::c_void, pdwbytesreturned: *mut u32) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn WlanDeviceServiceCommand(hclienthandle: super::super::Foundation::HANDLE, pinterfaceguid: *const ::windows::core::GUID, pdeviceserviceguid: *const ::windows::core::GUID, dwopcode: u32, dwinbuffersize: u32, pinbuffer: *const ::core::ffi::c_void, dwoutbuffersize: u32, poutbuffer: *mut ::core::ffi::c_void, pdwbytesreturned: *mut u32) -> u32;
        }
        ::core::mem::transmute(WlanDeviceServiceCommand(hclienthandle.into_param().abi(), ::core::mem::transmute(pinterfaceguid), ::core::mem::transmute(pdeviceserviceguid), ::core::mem::transmute(dwopcode), ::core::mem::transmute(dwinbuffersize), ::core::mem::transmute(pinbuffer), ::core::mem::transmute(dwoutbuffersize), ::core::mem::transmute(poutbuffer), ::core::mem::transmute(pdwbytesreturned)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn WlanDisconnect<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::HANDLE>>(hclienthandle: Param0, pinterfaceguid: *const ::windows::core::GUID, preserved: *mut ::core::ffi::c_void) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn WlanDisconnect(hclienthandle: super::super::Foundation::HANDLE, pinterfaceguid: *const ::windows::core::GUID, preserved: *mut ::core::ffi::c_void) -> u32;
        }
        ::core::mem::transmute(WlanDisconnect(hclienthandle.into_param().abi(), ::core::mem::transmute(pinterfaceguid), ::core::mem::transmute(preserved)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn WlanEnumInterfaces<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::HANDLE>>(hclienthandle: Param0, preserved: *mut ::core::ffi::c_void, ppinterfacelist: *mut *mut WLAN_INTERFACE_INFO_LIST) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn WlanEnumInterfaces(hclienthandle: super::super::Foundation::HANDLE, preserved: *mut ::core::ffi::c_void, ppinterfacelist: *mut *mut WLAN_INTERFACE_INFO_LIST) -> u32;
        }
        ::core::mem::transmute(WlanEnumInterfaces(hclienthandle.into_param().abi(), ::core::mem::transmute(preserved), ::core::mem::transmute(ppinterfacelist)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn WlanExtractPsdIEDataList<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::HANDLE>, Param3: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(hclienthandle: Param0, dwiedatasize: u32, prawiedata: *const u8, strformat: Param3, preserved: *mut ::core::ffi::c_void, pppsdiedatalist: *mut *mut WLAN_RAW_DATA_LIST) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn WlanExtractPsdIEDataList(hclienthandle: super::super::Foundation::HANDLE, dwiedatasize: u32, prawiedata: *const u8, strformat: super::super::Foundation::PWSTR, preserved: *mut ::core::ffi::c_void, pppsdiedatalist: *mut *mut WLAN_RAW_DATA_LIST) -> u32;
        }
        ::core::mem::transmute(WlanExtractPsdIEDataList(hclienthandle.into_param().abi(), ::core::mem::transmute(dwiedatasize), ::core::mem::transmute(prawiedata), strformat.into_param().abi(), ::core::mem::transmute(preserved), ::core::mem::transmute(pppsdiedatalist)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn WlanFreeMemory(pmemory: *const ::core::ffi::c_void) {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn WlanFreeMemory(pmemory: *const ::core::ffi::c_void);
        }
        WlanFreeMemory(::core::mem::transmute(pmemory))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn WlanGetAvailableNetworkList<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::HANDLE>>(hclienthandle: Param0, pinterfaceguid: *const ::windows::core::GUID, dwflags: u32, preserved: *mut ::core::ffi::c_void, ppavailablenetworklist: *mut *mut WLAN_AVAILABLE_NETWORK_LIST) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn WlanGetAvailableNetworkList(hclienthandle: super::super::Foundation::HANDLE, pinterfaceguid: *const ::windows::core::GUID, dwflags: u32, preserved: *mut ::core::ffi::c_void, ppavailablenetworklist: *mut *mut WLAN_AVAILABLE_NETWORK_LIST) -> u32;
        }
        ::core::mem::transmute(WlanGetAvailableNetworkList(hclienthandle.into_param().abi(), ::core::mem::transmute(pinterfaceguid), ::core::mem::transmute(dwflags), ::core::mem::transmute(preserved), ::core::mem::transmute(ppavailablenetworklist)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn WlanGetAvailableNetworkList2<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::HANDLE>>(hclienthandle: Param0, pinterfaceguid: *const ::windows::core::GUID, dwflags: u32, preserved: *mut ::core::ffi::c_void, ppavailablenetworklist: *mut *mut WLAN_AVAILABLE_NETWORK_LIST_V2) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn WlanGetAvailableNetworkList2(hclienthandle: super::super::Foundation::HANDLE, pinterfaceguid: *const ::windows::core::GUID, dwflags: u32, preserved: *mut ::core::ffi::c_void, ppavailablenetworklist: *mut *mut WLAN_AVAILABLE_NETWORK_LIST_V2) -> u32;
        }
        ::core::mem::transmute(WlanGetAvailableNetworkList2(hclienthandle.into_param().abi(), ::core::mem::transmute(pinterfaceguid), ::core::mem::transmute(dwflags), ::core::mem::transmute(preserved), ::core::mem::transmute(ppavailablenetworklist)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn WlanGetFilterList<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::HANDLE>>(hclienthandle: Param0, wlanfilterlisttype: WLAN_FILTER_LIST_TYPE, preserved: *mut ::core::ffi::c_void, ppnetworklist: *mut *mut DOT11_NETWORK_LIST) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn WlanGetFilterList(hclienthandle: super::super::Foundation::HANDLE, wlanfilterlisttype: WLAN_FILTER_LIST_TYPE, preserved: *mut ::core::ffi::c_void, ppnetworklist: *mut *mut DOT11_NETWORK_LIST) -> u32;
        }
        ::core::mem::transmute(WlanGetFilterList(hclienthandle.into_param().abi(), ::core::mem::transmute(wlanfilterlisttype), ::core::mem::transmute(preserved), ::core::mem::transmute(ppnetworklist)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn WlanGetInterfaceCapability<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::HANDLE>>(hclienthandle: Param0, pinterfaceguid: *const ::windows::core::GUID, preserved: *mut ::core::ffi::c_void, ppcapability: *mut *mut WLAN_INTERFACE_CAPABILITY) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn WlanGetInterfaceCapability(hclienthandle: super::super::Foundation::HANDLE, pinterfaceguid: *const ::windows::core::GUID, preserved: *mut ::core::ffi::c_void, ppcapability: *mut *mut WLAN_INTERFACE_CAPABILITY) -> u32;
        }
        ::core::mem::transmute(WlanGetInterfaceCapability(hclienthandle.into_param().abi(), ::core::mem::transmute(pinterfaceguid), ::core::mem::transmute(preserved), ::core::mem::transmute(ppcapability)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn WlanGetNetworkBssList<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::HANDLE>, Param4: ::windows::core::IntoParam<'a, super::super::Foundation::BOOL>>(hclienthandle: Param0, pinterfaceguid: *const ::windows::core::GUID, pdot11ssid: *const DOT11_SSID, dot11bsstype: DOT11_BSS_TYPE, bsecurityenabled: Param4, preserved: *mut ::core::ffi::c_void, ppwlanbsslist: *mut *mut WLAN_BSS_LIST) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn WlanGetNetworkBssList(hclienthandle: super::super::Foundation::HANDLE, pinterfaceguid: *const ::windows::core::GUID, pdot11ssid: *const DOT11_SSID, dot11bsstype: DOT11_BSS_TYPE, bsecurityenabled: super::super::Foundation::BOOL, preserved: *mut ::core::ffi::c_void, ppwlanbsslist: *mut *mut WLAN_BSS_LIST) -> u32;
        }
        ::core::mem::transmute(WlanGetNetworkBssList(hclienthandle.into_param().abi(), ::core::mem::transmute(pinterfaceguid), ::core::mem::transmute(pdot11ssid), ::core::mem::transmute(dot11bsstype), bsecurityenabled.into_param().abi(), ::core::mem::transmute(preserved), ::core::mem::transmute(ppwlanbsslist)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn WlanGetProfile<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::HANDLE>, Param2: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(hclienthandle: Param0, pinterfaceguid: *const ::windows::core::GUID, strprofilename: Param2, preserved: *mut ::core::ffi::c_void, pstrprofilexml: *mut super::super::Foundation::PWSTR, pdwflags: *mut u32, pdwgrantedaccess: *mut u32) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn WlanGetProfile(hclienthandle: super::super::Foundation::HANDLE, pinterfaceguid: *const ::windows::core::GUID, strprofilename: super::super::Foundation::PWSTR, preserved: *mut ::core::ffi::c_void, pstrprofilexml: *mut super::super::Foundation::PWSTR, pdwflags: *mut u32, pdwgrantedaccess: *mut u32) -> u32;
        }
        ::core::mem::transmute(WlanGetProfile(hclienthandle.into_param().abi(), ::core::mem::transmute(pinterfaceguid), strprofilename.into_param().abi(), ::core::mem::transmute(preserved), ::core::mem::transmute(pstrprofilexml), ::core::mem::transmute(pdwflags), ::core::mem::transmute(pdwgrantedaccess)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn WlanGetProfileCustomUserData<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::HANDLE>, Param2: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(hclienthandle: Param0, pinterfaceguid: *const ::windows::core::GUID, strprofilename: Param2, preserved: *mut ::core::ffi::c_void, pdwdatasize: *mut u32, ppdata: *mut *mut u8) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn WlanGetProfileCustomUserData(hclienthandle: super::super::Foundation::HANDLE, pinterfaceguid: *const ::windows::core::GUID, strprofilename: super::super::Foundation::PWSTR, preserved: *mut ::core::ffi::c_void, pdwdatasize: *mut u32, ppdata: *mut *mut u8) -> u32;
        }
        ::core::mem::transmute(WlanGetProfileCustomUserData(hclienthandle.into_param().abi(), ::core::mem::transmute(pinterfaceguid), strprofilename.into_param().abi(), ::core::mem::transmute(preserved), ::core::mem::transmute(pdwdatasize), ::core::mem::transmute(ppdata)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn WlanGetProfileList<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::HANDLE>>(hclienthandle: Param0, pinterfaceguid: *const ::windows::core::GUID, preserved: *mut ::core::ffi::c_void, ppprofilelist: *mut *mut WLAN_PROFILE_INFO_LIST) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn WlanGetProfileList(hclienthandle: super::super::Foundation::HANDLE, pinterfaceguid: *const ::windows::core::GUID, preserved: *mut ::core::ffi::c_void, ppprofilelist: *mut *mut WLAN_PROFILE_INFO_LIST) -> u32;
        }
        ::core::mem::transmute(WlanGetProfileList(hclienthandle.into_param().abi(), ::core::mem::transmute(pinterfaceguid), ::core::mem::transmute(preserved), ::core::mem::transmute(ppprofilelist)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn WlanGetSecuritySettings<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::HANDLE>>(hclienthandle: Param0, securableobject: WLAN_SECURABLE_OBJECT, pvaluetype: *mut WLAN_OPCODE_VALUE_TYPE, pstrcurrentsddl: *mut super::super::Foundation::PWSTR, pdwgrantedaccess: *mut u32) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn WlanGetSecuritySettings(hclienthandle: super::super::Foundation::HANDLE, securableobject: WLAN_SECURABLE_OBJECT, pvaluetype: *mut WLAN_OPCODE_VALUE_TYPE, pstrcurrentsddl: *mut super::super::Foundation::PWSTR, pdwgrantedaccess: *mut u32) -> u32;
        }
        ::core::mem::transmute(WlanGetSecuritySettings(hclienthandle.into_param().abi(), ::core::mem::transmute(securableobject), ::core::mem::transmute(pvaluetype), ::core::mem::transmute(pstrcurrentsddl), ::core::mem::transmute(pdwgrantedaccess)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn WlanGetSupportedDeviceServices<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::HANDLE>>(hclienthandle: Param0, pinterfaceguid: *const ::windows::core::GUID, ppdevsvcguidlist: *mut *mut WLAN_DEVICE_SERVICE_GUID_LIST) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn WlanGetSupportedDeviceServices(hclienthandle: super::super::Foundation::HANDLE, pinterfaceguid: *const ::windows::core::GUID, ppdevsvcguidlist: *mut *mut WLAN_DEVICE_SERVICE_GUID_LIST) -> u32;
        }
        ::core::mem::transmute(WlanGetSupportedDeviceServices(hclienthandle.into_param().abi(), ::core::mem::transmute(pinterfaceguid), ::core::mem::transmute(ppdevsvcguidlist)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn WlanHostedNetworkForceStart<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::HANDLE>>(hclienthandle: Param0, pfailreason: *mut WLAN_HOSTED_NETWORK_REASON, pvreserved: *mut ::core::ffi::c_void) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn WlanHostedNetworkForceStart(hclienthandle: super::super::Foundation::HANDLE, pfailreason: *mut WLAN_HOSTED_NETWORK_REASON, pvreserved: *mut ::core::ffi::c_void) -> u32;
        }
        ::core::mem::transmute(WlanHostedNetworkForceStart(hclienthandle.into_param().abi(), ::core::mem::transmute(pfailreason), ::core::mem::transmute(pvreserved)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn WlanHostedNetworkForceStop<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::HANDLE>>(hclienthandle: Param0, pfailreason: *mut WLAN_HOSTED_NETWORK_REASON, pvreserved: *mut ::core::ffi::c_void) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn WlanHostedNetworkForceStop(hclienthandle: super::super::Foundation::HANDLE, pfailreason: *mut WLAN_HOSTED_NETWORK_REASON, pvreserved: *mut ::core::ffi::c_void) -> u32;
        }
        ::core::mem::transmute(WlanHostedNetworkForceStop(hclienthandle.into_param().abi(), ::core::mem::transmute(pfailreason), ::core::mem::transmute(pvreserved)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn WlanHostedNetworkInitSettings<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::HANDLE>>(hclienthandle: Param0, pfailreason: *mut WLAN_HOSTED_NETWORK_REASON, pvreserved: *mut ::core::ffi::c_void) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn WlanHostedNetworkInitSettings(hclienthandle: super::super::Foundation::HANDLE, pfailreason: *mut WLAN_HOSTED_NETWORK_REASON, pvreserved: *mut ::core::ffi::c_void) -> u32;
        }
        ::core::mem::transmute(WlanHostedNetworkInitSettings(hclienthandle.into_param().abi(), ::core::mem::transmute(pfailreason), ::core::mem::transmute(pvreserved)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn WlanHostedNetworkQueryProperty<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::HANDLE>>(hclienthandle: Param0, opcode: WLAN_HOSTED_NETWORK_OPCODE, pdwdatasize: *mut u32, ppvdata: *mut *mut ::core::ffi::c_void, pwlanopcodevaluetype: *mut WLAN_OPCODE_VALUE_TYPE, pvreserved: *mut ::core::ffi::c_void) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn WlanHostedNetworkQueryProperty(hclienthandle: super::super::Foundation::HANDLE, opcode: WLAN_HOSTED_NETWORK_OPCODE, pdwdatasize: *mut u32, ppvdata: *mut *mut ::core::ffi::c_void, pwlanopcodevaluetype: *mut WLAN_OPCODE_VALUE_TYPE, pvreserved: *mut ::core::ffi::c_void) -> u32;
        }
        ::core::mem::transmute(WlanHostedNetworkQueryProperty(hclienthandle.into_param().abi(), ::core::mem::transmute(opcode), ::core::mem::transmute(pdwdatasize), ::core::mem::transmute(ppvdata), ::core::mem::transmute(pwlanopcodevaluetype), ::core::mem::transmute(pvreserved)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn WlanHostedNetworkQuerySecondaryKey<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::HANDLE>>(hclienthandle: Param0, pdwkeylength: *mut u32, ppuckeydata: *mut *mut u8, pbispassphrase: *mut super::super::Foundation::BOOL, pbpersistent: *mut super::super::Foundation::BOOL, pfailreason: *mut WLAN_HOSTED_NETWORK_REASON, pvreserved: *mut ::core::ffi::c_void) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn WlanHostedNetworkQuerySecondaryKey(hclienthandle: super::super::Foundation::HANDLE, pdwkeylength: *mut u32, ppuckeydata: *mut *mut u8, pbispassphrase: *mut super::super::Foundation::BOOL, pbpersistent: *mut super::super::Foundation::BOOL, pfailreason: *mut WLAN_HOSTED_NETWORK_REASON, pvreserved: *mut ::core::ffi::c_void) -> u32;
        }
        ::core::mem::transmute(WlanHostedNetworkQuerySecondaryKey(hclienthandle.into_param().abi(), ::core::mem::transmute(pdwkeylength), ::core::mem::transmute(ppuckeydata), ::core::mem::transmute(pbispassphrase), ::core::mem::transmute(pbpersistent), ::core::mem::transmute(pfailreason), ::core::mem::transmute(pvreserved)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn WlanHostedNetworkQueryStatus<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::HANDLE>>(hclienthandle: Param0, ppwlanhostednetworkstatus: *mut *mut WLAN_HOSTED_NETWORK_STATUS, pvreserved: *mut ::core::ffi::c_void) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn WlanHostedNetworkQueryStatus(hclienthandle: super::super::Foundation::HANDLE, ppwlanhostednetworkstatus: *mut *mut WLAN_HOSTED_NETWORK_STATUS, pvreserved: *mut ::core::ffi::c_void) -> u32;
        }
        ::core::mem::transmute(WlanHostedNetworkQueryStatus(hclienthandle.into_param().abi(), ::core::mem::transmute(ppwlanhostednetworkstatus), ::core::mem::transmute(pvreserved)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn WlanHostedNetworkRefreshSecuritySettings<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::HANDLE>>(hclienthandle: Param0, pfailreason: *mut WLAN_HOSTED_NETWORK_REASON, pvreserved: *mut ::core::ffi::c_void) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn WlanHostedNetworkRefreshSecuritySettings(hclienthandle: super::super::Foundation::HANDLE, pfailreason: *mut WLAN_HOSTED_NETWORK_REASON, pvreserved: *mut ::core::ffi::c_void) -> u32;
        }
        ::core::mem::transmute(WlanHostedNetworkRefreshSecuritySettings(hclienthandle.into_param().abi(), ::core::mem::transmute(pfailreason), ::core::mem::transmute(pvreserved)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn WlanHostedNetworkSetProperty<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::HANDLE>>(hclienthandle: Param0, opcode: WLAN_HOSTED_NETWORK_OPCODE, dwdatasize: u32, pvdata: *const ::core::ffi::c_void, pfailreason: *mut WLAN_HOSTED_NETWORK_REASON, pvreserved: *mut ::core::ffi::c_void) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn WlanHostedNetworkSetProperty(hclienthandle: super::super::Foundation::HANDLE, opcode: WLAN_HOSTED_NETWORK_OPCODE, dwdatasize: u32, pvdata: *const ::core::ffi::c_void, pfailreason: *mut WLAN_HOSTED_NETWORK_REASON, pvreserved: *mut ::core::ffi::c_void) -> u32;
        }
        ::core::mem::transmute(WlanHostedNetworkSetProperty(hclienthandle.into_param().abi(), ::core::mem::transmute(opcode), ::core::mem::transmute(dwdatasize), ::core::mem::transmute(pvdata), ::core::mem::transmute(pfailreason), ::core::mem::transmute(pvreserved)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn WlanHostedNetworkSetSecondaryKey<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::HANDLE>, Param3: ::windows::core::IntoParam<'a, super::super::Foundation::BOOL>, Param4: ::windows::core::IntoParam<'a, super::super::Foundation::BOOL>>(hclienthandle: Param0, dwkeylength: u32, puckeydata: *const u8, bispassphrase: Param3, bpersistent: Param4, pfailreason: *mut WLAN_HOSTED_NETWORK_REASON, pvreserved: *mut ::core::ffi::c_void) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn WlanHostedNetworkSetSecondaryKey(hclienthandle: super::super::Foundation::HANDLE, dwkeylength: u32, puckeydata: *const u8, bispassphrase: super::super::Foundation::BOOL, bpersistent: super::super::Foundation::BOOL, pfailreason: *mut WLAN_HOSTED_NETWORK_REASON, pvreserved: *mut ::core::ffi::c_void) -> u32;
        }
        ::core::mem::transmute(WlanHostedNetworkSetSecondaryKey(hclienthandle.into_param().abi(), ::core::mem::transmute(dwkeylength), ::core::mem::transmute(puckeydata), bispassphrase.into_param().abi(), bpersistent.into_param().abi(), ::core::mem::transmute(pfailreason), ::core::mem::transmute(pvreserved)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn WlanHostedNetworkStartUsing<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::HANDLE>>(hclienthandle: Param0, pfailreason: *mut WLAN_HOSTED_NETWORK_REASON, pvreserved: *mut ::core::ffi::c_void) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn WlanHostedNetworkStartUsing(hclienthandle: super::super::Foundation::HANDLE, pfailreason: *mut WLAN_HOSTED_NETWORK_REASON, pvreserved: *mut ::core::ffi::c_void) -> u32;
        }
        ::core::mem::transmute(WlanHostedNetworkStartUsing(hclienthandle.into_param().abi(), ::core::mem::transmute(pfailreason), ::core::mem::transmute(pvreserved)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn WlanHostedNetworkStopUsing<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::HANDLE>>(hclienthandle: Param0, pfailreason: *mut WLAN_HOSTED_NETWORK_REASON, pvreserved: *mut ::core::ffi::c_void) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn WlanHostedNetworkStopUsing(hclienthandle: super::super::Foundation::HANDLE, pfailreason: *mut WLAN_HOSTED_NETWORK_REASON, pvreserved: *mut ::core::ffi::c_void) -> u32;
        }
        ::core::mem::transmute(WlanHostedNetworkStopUsing(hclienthandle.into_param().abi(), ::core::mem::transmute(pfailreason), ::core::mem::transmute(pvreserved)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn WlanIhvControl<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::HANDLE>>(hclienthandle: Param0, pinterfaceguid: *const ::windows::core::GUID, r#type: WLAN_IHV_CONTROL_TYPE, dwinbuffersize: u32, pinbuffer: *const ::core::ffi::c_void, dwoutbuffersize: u32, poutbuffer: *mut ::core::ffi::c_void, pdwbytesreturned: *mut u32) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn WlanIhvControl(hclienthandle: super::super::Foundation::HANDLE, pinterfaceguid: *const ::windows::core::GUID, r#type: WLAN_IHV_CONTROL_TYPE, dwinbuffersize: u32, pinbuffer: *const ::core::ffi::c_void, dwoutbuffersize: u32, poutbuffer: *mut ::core::ffi::c_void, pdwbytesreturned: *mut u32) -> u32;
        }
        ::core::mem::transmute(WlanIhvControl(hclienthandle.into_param().abi(), ::core::mem::transmute(pinterfaceguid), ::core::mem::transmute(r#type), ::core::mem::transmute(dwinbuffersize), ::core::mem::transmute(pinbuffer), ::core::mem::transmute(dwoutbuffersize), ::core::mem::transmute(poutbuffer), ::core::mem::transmute(pdwbytesreturned)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn WlanOpenHandle(dwclientversion: u32, preserved: *mut ::core::ffi::c_void, pdwnegotiatedversion: *mut u32, phclienthandle: *mut super::super::Foundation::HANDLE) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn WlanOpenHandle(dwclientversion: u32, preserved: *mut ::core::ffi::c_void, pdwnegotiatedversion: *mut u32, phclienthandle: *mut super::super::Foundation::HANDLE) -> u32;
        }
        ::core::mem::transmute(WlanOpenHandle(::core::mem::transmute(dwclientversion), ::core::mem::transmute(preserved), ::core::mem::transmute(pdwnegotiatedversion), ::core::mem::transmute(phclienthandle)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn WlanQueryAutoConfigParameter<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::HANDLE>>(hclienthandle: Param0, opcode: WLAN_AUTOCONF_OPCODE, preserved: *mut ::core::ffi::c_void, pdwdatasize: *mut u32, ppdata: *mut *mut ::core::ffi::c_void, pwlanopcodevaluetype: *mut WLAN_OPCODE_VALUE_TYPE) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn WlanQueryAutoConfigParameter(hclienthandle: super::super::Foundation::HANDLE, opcode: WLAN_AUTOCONF_OPCODE, preserved: *mut ::core::ffi::c_void, pdwdatasize: *mut u32, ppdata: *mut *mut ::core::ffi::c_void, pwlanopcodevaluetype: *mut WLAN_OPCODE_VALUE_TYPE) -> u32;
        }
        ::core::mem::transmute(WlanQueryAutoConfigParameter(hclienthandle.into_param().abi(), ::core::mem::transmute(opcode), ::core::mem::transmute(preserved), ::core::mem::transmute(pdwdatasize), ::core::mem::transmute(ppdata), ::core::mem::transmute(pwlanopcodevaluetype)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn WlanQueryInterface<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::HANDLE>>(hclienthandle: Param0, pinterfaceguid: *const ::windows::core::GUID, opcode: WLAN_INTF_OPCODE, preserved: *mut ::core::ffi::c_void, pdwdatasize: *mut u32, ppdata: *mut *mut ::core::ffi::c_void, pwlanopcodevaluetype: *mut WLAN_OPCODE_VALUE_TYPE) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn WlanQueryInterface(hclienthandle: super::super::Foundation::HANDLE, pinterfaceguid: *const ::windows::core::GUID, opcode: WLAN_INTF_OPCODE, preserved: *mut ::core::ffi::c_void, pdwdatasize: *mut u32, ppdata: *mut *mut ::core::ffi::c_void, pwlanopcodevaluetype: *mut WLAN_OPCODE_VALUE_TYPE) -> u32;
        }
        ::core::mem::transmute(WlanQueryInterface(hclienthandle.into_param().abi(), ::core::mem::transmute(pinterfaceguid), ::core::mem::transmute(opcode), ::core::mem::transmute(preserved), ::core::mem::transmute(pdwdatasize), ::core::mem::transmute(ppdata), ::core::mem::transmute(pwlanopcodevaluetype)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn WlanReasonCodeToString<'a, Param2: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(dwreasoncode: u32, dwbuffersize: u32, pstringbuffer: Param2, preserved: *mut ::core::ffi::c_void) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn WlanReasonCodeToString(dwreasoncode: u32, dwbuffersize: u32, pstringbuffer: super::super::Foundation::PWSTR, preserved: *mut ::core::ffi::c_void) -> u32;
        }
        ::core::mem::transmute(WlanReasonCodeToString(::core::mem::transmute(dwreasoncode), ::core::mem::transmute(dwbuffersize), pstringbuffer.into_param().abi(), ::core::mem::transmute(preserved)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn WlanRegisterDeviceServiceNotification<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::HANDLE>>(hclienthandle: Param0, pdevsvcguidlist: *const WLAN_DEVICE_SERVICE_GUID_LIST) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn WlanRegisterDeviceServiceNotification(hclienthandle: super::super::Foundation::HANDLE, pdevsvcguidlist: *const WLAN_DEVICE_SERVICE_GUID_LIST) -> u32;
        }
        ::core::mem::transmute(WlanRegisterDeviceServiceNotification(hclienthandle.into_param().abi(), ::core::mem::transmute(pdevsvcguidlist)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn WlanRegisterNotification<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::HANDLE>, Param2: ::windows::core::IntoParam<'a, super::super::Foundation::BOOL>>(hclienthandle: Param0, dwnotifsource: u32, bignoreduplicate: Param2, funccallback: WLAN_NOTIFICATION_CALLBACK, pcallbackcontext: *const ::core::ffi::c_void, preserved: *mut ::core::ffi::c_void, pdwprevnotifsource: *mut u32) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn WlanRegisterNotification(hclienthandle: super::super::Foundation::HANDLE, dwnotifsource: u32, bignoreduplicate: super::super::Foundation::BOOL, funccallback: ::windows::core::RawPtr, pcallbackcontext: *const ::core::ffi::c_void, preserved: *mut ::core::ffi::c_void, pdwprevnotifsource: *mut u32) -> u32;
        }
        ::core::mem::transmute(WlanRegisterNotification(hclienthandle.into_param().abi(), ::core::mem::transmute(dwnotifsource), bignoreduplicate.into_param().abi(), ::core::mem::transmute(funccallback), ::core::mem::transmute(pcallbackcontext), ::core::mem::transmute(preserved), ::core::mem::transmute(pdwprevnotifsource)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn WlanRegisterVirtualStationNotification<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::HANDLE>, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::BOOL>>(hclienthandle: Param0, bregister: Param1, preserved: *mut ::core::ffi::c_void) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn WlanRegisterVirtualStationNotification(hclienthandle: super::super::Foundation::HANDLE, bregister: super::super::Foundation::BOOL, preserved: *mut ::core::ffi::c_void) -> u32;
        }
        ::core::mem::transmute(WlanRegisterVirtualStationNotification(hclienthandle.into_param().abi(), bregister.into_param().abi(), ::core::mem::transmute(preserved)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn WlanRenameProfile<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::HANDLE>, Param2: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>, Param3: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(hclienthandle: Param0, pinterfaceguid: *const ::windows::core::GUID, stroldprofilename: Param2, strnewprofilename: Param3, preserved: *mut ::core::ffi::c_void) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn WlanRenameProfile(hclienthandle: super::super::Foundation::HANDLE, pinterfaceguid: *const ::windows::core::GUID, stroldprofilename: super::super::Foundation::PWSTR, strnewprofilename: super::super::Foundation::PWSTR, preserved: *mut ::core::ffi::c_void) -> u32;
        }
        ::core::mem::transmute(WlanRenameProfile(hclienthandle.into_param().abi(), ::core::mem::transmute(pinterfaceguid), stroldprofilename.into_param().abi(), strnewprofilename.into_param().abi(), ::core::mem::transmute(preserved)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn WlanSaveTemporaryProfile<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::HANDLE>, Param2: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>, Param3: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>, Param5: ::windows::core::IntoParam<'a, super::super::Foundation::BOOL>>(hclienthandle: Param0, pinterfaceguid: *const ::windows::core::GUID, strprofilename: Param2, stralluserprofilesecurity: Param3, dwflags: u32, boverwrite: Param5, preserved: *mut ::core::ffi::c_void) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn WlanSaveTemporaryProfile(hclienthandle: super::super::Foundation::HANDLE, pinterfaceguid: *const ::windows::core::GUID, strprofilename: super::super::Foundation::PWSTR, stralluserprofilesecurity: super::super::Foundation::PWSTR, dwflags: u32, boverwrite: super::super::Foundation::BOOL, preserved: *mut ::core::ffi::c_void) -> u32;
        }
        ::core::mem::transmute(WlanSaveTemporaryProfile(hclienthandle.into_param().abi(), ::core::mem::transmute(pinterfaceguid), strprofilename.into_param().abi(), stralluserprofilesecurity.into_param().abi(), ::core::mem::transmute(dwflags), boverwrite.into_param().abi(), ::core::mem::transmute(preserved)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn WlanScan<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::HANDLE>>(hclienthandle: Param0, pinterfaceguid: *const ::windows::core::GUID, pdot11ssid: *const DOT11_SSID, piedata: *const WLAN_RAW_DATA, preserved: *mut ::core::ffi::c_void) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn WlanScan(hclienthandle: super::super::Foundation::HANDLE, pinterfaceguid: *const ::windows::core::GUID, pdot11ssid: *const DOT11_SSID, piedata: *const WLAN_RAW_DATA, preserved: *mut ::core::ffi::c_void) -> u32;
        }
        ::core::mem::transmute(WlanScan(hclienthandle.into_param().abi(), ::core::mem::transmute(pinterfaceguid), ::core::mem::transmute(pdot11ssid), ::core::mem::transmute(piedata), ::core::mem::transmute(preserved)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn WlanSetAutoConfigParameter<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::HANDLE>>(hclienthandle: Param0, opcode: WLAN_AUTOCONF_OPCODE, dwdatasize: u32, pdata: *const ::core::ffi::c_void, preserved: *mut ::core::ffi::c_void) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn WlanSetAutoConfigParameter(hclienthandle: super::super::Foundation::HANDLE, opcode: WLAN_AUTOCONF_OPCODE, dwdatasize: u32, pdata: *const ::core::ffi::c_void, preserved: *mut ::core::ffi::c_void) -> u32;
        }
        ::core::mem::transmute(WlanSetAutoConfigParameter(hclienthandle.into_param().abi(), ::core::mem::transmute(opcode), ::core::mem::transmute(dwdatasize), ::core::mem::transmute(pdata), ::core::mem::transmute(preserved)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn WlanSetFilterList<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::HANDLE>>(hclienthandle: Param0, wlanfilterlisttype: WLAN_FILTER_LIST_TYPE, pnetworklist: *const DOT11_NETWORK_LIST, preserved: *mut ::core::ffi::c_void) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn WlanSetFilterList(hclienthandle: super::super::Foundation::HANDLE, wlanfilterlisttype: WLAN_FILTER_LIST_TYPE, pnetworklist: *const DOT11_NETWORK_LIST, preserved: *mut ::core::ffi::c_void) -> u32;
        }
        ::core::mem::transmute(WlanSetFilterList(hclienthandle.into_param().abi(), ::core::mem::transmute(wlanfilterlisttype), ::core::mem::transmute(pnetworklist), ::core::mem::transmute(preserved)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn WlanSetInterface<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::HANDLE>>(hclienthandle: Param0, pinterfaceguid: *const ::windows::core::GUID, opcode: WLAN_INTF_OPCODE, dwdatasize: u32, pdata: *const ::core::ffi::c_void, preserved: *mut ::core::ffi::c_void) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn WlanSetInterface(hclienthandle: super::super::Foundation::HANDLE, pinterfaceguid: *const ::windows::core::GUID, opcode: WLAN_INTF_OPCODE, dwdatasize: u32, pdata: *const ::core::ffi::c_void, preserved: *mut ::core::ffi::c_void) -> u32;
        }
        ::core::mem::transmute(WlanSetInterface(hclienthandle.into_param().abi(), ::core::mem::transmute(pinterfaceguid), ::core::mem::transmute(opcode), ::core::mem::transmute(dwdatasize), ::core::mem::transmute(pdata), ::core::mem::transmute(preserved)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn WlanSetProfile<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::HANDLE>, Param3: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>, Param4: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>, Param5: ::windows::core::IntoParam<'a, super::super::Foundation::BOOL>>(hclienthandle: Param0, pinterfaceguid: *const ::windows::core::GUID, dwflags: u32, strprofilexml: Param3, stralluserprofilesecurity: Param4, boverwrite: Param5, preserved: *mut ::core::ffi::c_void, pdwreasoncode: *mut u32) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn WlanSetProfile(hclienthandle: super::super::Foundation::HANDLE, pinterfaceguid: *const ::windows::core::GUID, dwflags: u32, strprofilexml: super::super::Foundation::PWSTR, stralluserprofilesecurity: super::super::Foundation::PWSTR, boverwrite: super::super::Foundation::BOOL, preserved: *mut ::core::ffi::c_void, pdwreasoncode: *mut u32) -> u32;
        }
        ::core::mem::transmute(WlanSetProfile(hclienthandle.into_param().abi(), ::core::mem::transmute(pinterfaceguid), ::core::mem::transmute(dwflags), strprofilexml.into_param().abi(), stralluserprofilesecurity.into_param().abi(), boverwrite.into_param().abi(), ::core::mem::transmute(preserved), ::core::mem::transmute(pdwreasoncode)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn WlanSetProfileCustomUserData<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::HANDLE>, Param2: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(hclienthandle: Param0, pinterfaceguid: *const ::windows::core::GUID, strprofilename: Param2, dwdatasize: u32, pdata: *const u8, preserved: *mut ::core::ffi::c_void) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn WlanSetProfileCustomUserData(hclienthandle: super::super::Foundation::HANDLE, pinterfaceguid: *const ::windows::core::GUID, strprofilename: super::super::Foundation::PWSTR, dwdatasize: u32, pdata: *const u8, preserved: *mut ::core::ffi::c_void) -> u32;
        }
        ::core::mem::transmute(WlanSetProfileCustomUserData(hclienthandle.into_param().abi(), ::core::mem::transmute(pinterfaceguid), strprofilename.into_param().abi(), ::core::mem::transmute(dwdatasize), ::core::mem::transmute(pdata), ::core::mem::transmute(preserved)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_ExtensibleAuthenticationProtocol"))]
#[inline]
pub unsafe fn WlanSetProfileEapUserData<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::HANDLE>, Param2: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>, Param3: ::windows::core::IntoParam<'a, super::super::Security::ExtensibleAuthenticationProtocol::EAP_METHOD_TYPE>>(hclienthandle: Param0, pinterfaceguid: *const ::windows::core::GUID, strprofilename: Param2, eaptype: Param3, dwflags: WLAN_SET_EAPHOST_FLAGS, dweapuserdatasize: u32, pbeapuserdata: *const u8, preserved: *mut ::core::ffi::c_void) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn WlanSetProfileEapUserData(hclienthandle: super::super::Foundation::HANDLE, pinterfaceguid: *const ::windows::core::GUID, strprofilename: super::super::Foundation::PWSTR, eaptype: super::super::Security::ExtensibleAuthenticationProtocol::EAP_METHOD_TYPE, dwflags: WLAN_SET_EAPHOST_FLAGS, dweapuserdatasize: u32, pbeapuserdata: *const u8, preserved: *mut ::core::ffi::c_void) -> u32;
        }
        ::core::mem::transmute(WlanSetProfileEapUserData(hclienthandle.into_param().abi(), ::core::mem::transmute(pinterfaceguid), strprofilename.into_param().abi(), eaptype.into_param().abi(), ::core::mem::transmute(dwflags), ::core::mem::transmute(dweapuserdatasize), ::core::mem::transmute(pbeapuserdata), ::core::mem::transmute(preserved)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn WlanSetProfileEapXmlUserData<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::HANDLE>, Param2: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>, Param4: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(hclienthandle: Param0, pinterfaceguid: *const ::windows::core::GUID, strprofilename: Param2, dwflags: WLAN_SET_EAPHOST_FLAGS, streapxmluserdata: Param4, preserved: *mut ::core::ffi::c_void) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn WlanSetProfileEapXmlUserData(hclienthandle: super::super::Foundation::HANDLE, pinterfaceguid: *const ::windows::core::GUID, strprofilename: super::super::Foundation::PWSTR, dwflags: WLAN_SET_EAPHOST_FLAGS, streapxmluserdata: super::super::Foundation::PWSTR, preserved: *mut ::core::ffi::c_void) -> u32;
        }
        ::core::mem::transmute(WlanSetProfileEapXmlUserData(hclienthandle.into_param().abi(), ::core::mem::transmute(pinterfaceguid), strprofilename.into_param().abi(), ::core::mem::transmute(dwflags), streapxmluserdata.into_param().abi(), ::core::mem::transmute(preserved)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn WlanSetProfileList<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::HANDLE>>(hclienthandle: Param0, pinterfaceguid: *const ::windows::core::GUID, dwitems: u32, strprofilenames: *const super::super::Foundation::PWSTR, preserved: *mut ::core::ffi::c_void) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn WlanSetProfileList(hclienthandle: super::super::Foundation::HANDLE, pinterfaceguid: *const ::windows::core::GUID, dwitems: u32, strprofilenames: *const super::super::Foundation::PWSTR, preserved: *mut ::core::ffi::c_void) -> u32;
        }
        ::core::mem::transmute(WlanSetProfileList(hclienthandle.into_param().abi(), ::core::mem::transmute(pinterfaceguid), ::core::mem::transmute(dwitems), ::core::mem::transmute(strprofilenames), ::core::mem::transmute(preserved)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn WlanSetProfilePosition<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::HANDLE>, Param2: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(hclienthandle: Param0, pinterfaceguid: *const ::windows::core::GUID, strprofilename: Param2, dwposition: u32, preserved: *mut ::core::ffi::c_void) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn WlanSetProfilePosition(hclienthandle: super::super::Foundation::HANDLE, pinterfaceguid: *const ::windows::core::GUID, strprofilename: super::super::Foundation::PWSTR, dwposition: u32, preserved: *mut ::core::ffi::c_void) -> u32;
        }
        ::core::mem::transmute(WlanSetProfilePosition(hclienthandle.into_param().abi(), ::core::mem::transmute(pinterfaceguid), strprofilename.into_param().abi(), ::core::mem::transmute(dwposition), ::core::mem::transmute(preserved)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn WlanSetPsdIEDataList<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::HANDLE>, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(hclienthandle: Param0, strformat: Param1, ppsdiedatalist: *const WLAN_RAW_DATA_LIST, preserved: *mut ::core::ffi::c_void) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn WlanSetPsdIEDataList(hclienthandle: super::super::Foundation::HANDLE, strformat: super::super::Foundation::PWSTR, ppsdiedatalist: *const WLAN_RAW_DATA_LIST, preserved: *mut ::core::ffi::c_void) -> u32;
        }
        ::core::mem::transmute(WlanSetPsdIEDataList(hclienthandle.into_param().abi(), strformat.into_param().abi(), ::core::mem::transmute(ppsdiedatalist), ::core::mem::transmute(preserved)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn WlanSetSecuritySettings<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::HANDLE>, Param2: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(hclienthandle: Param0, securableobject: WLAN_SECURABLE_OBJECT, strmodifiedsddl: Param2) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn WlanSetSecuritySettings(hclienthandle: super::super::Foundation::HANDLE, securableobject: WLAN_SECURABLE_OBJECT, strmodifiedsddl: super::super::Foundation::PWSTR) -> u32;
        }
        ::core::mem::transmute(WlanSetSecuritySettings(hclienthandle.into_param().abi(), ::core::mem::transmute(securableobject), strmodifiedsddl.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn WlanUIEditProfile<'a, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>, Param3: ::windows::core::IntoParam<'a, super::super::Foundation::HWND>>(dwclientversion: u32, wstrprofilename: Param1, pinterfaceguid: *const ::windows::core::GUID, hwnd: Param3, wlstartpage: WL_DISPLAY_PAGES, preserved: *mut ::core::ffi::c_void, pwlanreasoncode: *mut u32) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn WlanUIEditProfile(dwclientversion: u32, wstrprofilename: super::super::Foundation::PWSTR, pinterfaceguid: *const ::windows::core::GUID, hwnd: super::super::Foundation::HWND, wlstartpage: WL_DISPLAY_PAGES, preserved: *mut ::core::ffi::c_void, pwlanreasoncode: *mut u32) -> u32;
        }
        ::core::mem::transmute(WlanUIEditProfile(::core::mem::transmute(dwclientversion), wstrprofilename.into_param().abi(), ::core::mem::transmute(pinterfaceguid), hwnd.into_param().abi(), ::core::mem::transmute(wlstartpage), ::core::mem::transmute(preserved), ::core::mem::transmute(pwlanreasoncode)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[repr(C)]
pub struct _DOT11_WME_AC_PARAMTERS_LIST {
    pub uNumOfEntries: u32,
    pub uTotalNumOfEntries: u32,
    pub dot11WMEACParameters: [DOT11_WME_AC_PARAMETERS; 1],
}
impl ::core::marker::Copy for _DOT11_WME_AC_PARAMTERS_LIST {}
impl ::core::clone::Clone for _DOT11_WME_AC_PARAMTERS_LIST {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for _DOT11_WME_AC_PARAMTERS_LIST {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for _DOT11_WME_AC_PARAMTERS_LIST {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<_DOT11_WME_AC_PARAMTERS_LIST>()) == 0 }
    }
}
impl ::core::cmp::Eq for _DOT11_WME_AC_PARAMTERS_LIST {}
impl ::core::default::Default for _DOT11_WME_AC_PARAMTERS_LIST {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}

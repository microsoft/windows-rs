#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, clashing_extern_declarations, clippy::all)]
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkPolicyServer\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct ACCOUNTINGPROPERTIES(pub i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkPolicyServer\"`*"]
pub const PROPERTY_ACCOUNTING_LOG_ACCOUNTING: ACCOUNTINGPROPERTIES = ACCOUNTINGPROPERTIES(1026i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkPolicyServer\"`*"]
pub const PROPERTY_ACCOUNTING_LOG_ACCOUNTING_INTERIM: ACCOUNTINGPROPERTIES = ACCOUNTINGPROPERTIES(1027i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkPolicyServer\"`*"]
pub const PROPERTY_ACCOUNTING_LOG_AUTHENTICATION: ACCOUNTINGPROPERTIES = ACCOUNTINGPROPERTIES(1028i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkPolicyServer\"`*"]
pub const PROPERTY_ACCOUNTING_LOG_OPEN_NEW_FREQUENCY: ACCOUNTINGPROPERTIES = ACCOUNTINGPROPERTIES(1029i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkPolicyServer\"`*"]
pub const PROPERTY_ACCOUNTING_LOG_OPEN_NEW_SIZE: ACCOUNTINGPROPERTIES = ACCOUNTINGPROPERTIES(1030i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkPolicyServer\"`*"]
pub const PROPERTY_ACCOUNTING_LOG_FILE_DIRECTORY: ACCOUNTINGPROPERTIES = ACCOUNTINGPROPERTIES(1031i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkPolicyServer\"`*"]
pub const PROPERTY_ACCOUNTING_LOG_IAS1_FORMAT: ACCOUNTINGPROPERTIES = ACCOUNTINGPROPERTIES(1032i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkPolicyServer\"`*"]
pub const PROPERTY_ACCOUNTING_LOG_ENABLE_LOGGING: ACCOUNTINGPROPERTIES = ACCOUNTINGPROPERTIES(1033i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkPolicyServer\"`*"]
pub const PROPERTY_ACCOUNTING_LOG_DELETE_IF_FULL: ACCOUNTINGPROPERTIES = ACCOUNTINGPROPERTIES(1034i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkPolicyServer\"`*"]
pub const PROPERTY_ACCOUNTING_SQL_MAX_SESSIONS: ACCOUNTINGPROPERTIES = ACCOUNTINGPROPERTIES(1035i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkPolicyServer\"`*"]
pub const PROPERTY_ACCOUNTING_LOG_AUTHENTICATION_INTERIM: ACCOUNTINGPROPERTIES = ACCOUNTINGPROPERTIES(1036i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkPolicyServer\"`*"]
pub const PROPERTY_ACCOUNTING_LOG_FILE_IS_BACKUP: ACCOUNTINGPROPERTIES = ACCOUNTINGPROPERTIES(1037i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkPolicyServer\"`*"]
pub const PROPERTY_ACCOUNTING_DISCARD_REQUEST_ON_FAILURE: ACCOUNTINGPROPERTIES = ACCOUNTINGPROPERTIES(1038i32);
impl ::core::marker::Copy for ACCOUNTINGPROPERTIES {}
impl ::core::clone::Clone for ACCOUNTINGPROPERTIES {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for ACCOUNTINGPROPERTIES {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for ACCOUNTINGPROPERTIES {
    type Abi = Self;
}
impl ::core::fmt::Debug for ACCOUNTINGPROPERTIES {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ACCOUNTINGPROPERTIES").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkPolicyServer\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct ATTRIBUTEFILTER(pub i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkPolicyServer\"`*"]
pub const ATTRIBUTE_FILTER_NONE: ATTRIBUTEFILTER = ATTRIBUTEFILTER(0i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkPolicyServer\"`*"]
pub const ATTRIBUTE_FILTER_VPN_DIALUP: ATTRIBUTEFILTER = ATTRIBUTEFILTER(1i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkPolicyServer\"`*"]
pub const ATTRIBUTE_FILTER_IEEE_802_1x: ATTRIBUTEFILTER = ATTRIBUTEFILTER(2i32);
impl ::core::marker::Copy for ATTRIBUTEFILTER {}
impl ::core::clone::Clone for ATTRIBUTEFILTER {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for ATTRIBUTEFILTER {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for ATTRIBUTEFILTER {
    type Abi = Self;
}
impl ::core::fmt::Debug for ATTRIBUTEFILTER {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ATTRIBUTEFILTER").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkPolicyServer\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct ATTRIBUTEID(pub u32);
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkPolicyServer\"`*"]
pub const ATTRIBUTE_UNDEFINED: ATTRIBUTEID = ATTRIBUTEID(0u32);
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkPolicyServer\"`*"]
pub const ATTRIBUTE_MIN_VALUE: ATTRIBUTEID = ATTRIBUTEID(1u32);
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkPolicyServer\"`*"]
pub const RADIUS_ATTRIBUTE_USER_NAME: ATTRIBUTEID = ATTRIBUTEID(1u32);
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkPolicyServer\"`*"]
pub const RADIUS_ATTRIBUTE_USER_PASSWORD: ATTRIBUTEID = ATTRIBUTEID(2u32);
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkPolicyServer\"`*"]
pub const RADIUS_ATTRIBUTE_CHAP_PASSWORD: ATTRIBUTEID = ATTRIBUTEID(3u32);
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkPolicyServer\"`*"]
pub const RADIUS_ATTRIBUTE_NAS_IP_ADDRESS: ATTRIBUTEID = ATTRIBUTEID(4u32);
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkPolicyServer\"`*"]
pub const RADIUS_ATTRIBUTE_NAS_PORT: ATTRIBUTEID = ATTRIBUTEID(5u32);
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkPolicyServer\"`*"]
pub const RADIUS_ATTRIBUTE_SERVICE_TYPE: ATTRIBUTEID = ATTRIBUTEID(6u32);
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkPolicyServer\"`*"]
pub const RADIUS_ATTRIBUTE_FRAMED_PROTOCOL: ATTRIBUTEID = ATTRIBUTEID(7u32);
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkPolicyServer\"`*"]
pub const RADIUS_ATTRIBUTE_FRAMED_IP_ADDRESS: ATTRIBUTEID = ATTRIBUTEID(8u32);
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkPolicyServer\"`*"]
pub const RADIUS_ATTRIBUTE_FRAMED_IP_NETMASK: ATTRIBUTEID = ATTRIBUTEID(9u32);
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkPolicyServer\"`*"]
pub const RADIUS_ATTRIBUTE_FRAMED_ROUTING: ATTRIBUTEID = ATTRIBUTEID(10u32);
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkPolicyServer\"`*"]
pub const RADIUS_ATTRIBUTE_FILTER_ID: ATTRIBUTEID = ATTRIBUTEID(11u32);
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkPolicyServer\"`*"]
pub const RADIUS_ATTRIBUTE_FRAMED_MTU: ATTRIBUTEID = ATTRIBUTEID(12u32);
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkPolicyServer\"`*"]
pub const RADIUS_ATTRIBUTE_FRAMED_COMPRESSION: ATTRIBUTEID = ATTRIBUTEID(13u32);
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkPolicyServer\"`*"]
pub const RADIUS_ATTRIBUTE_LOGIN_IP_HOST: ATTRIBUTEID = ATTRIBUTEID(14u32);
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkPolicyServer\"`*"]
pub const RADIUS_ATTRIBUTE_LOGIN_SERVICE: ATTRIBUTEID = ATTRIBUTEID(15u32);
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkPolicyServer\"`*"]
pub const RADIUS_ATTRIBUTE_LOGIN_TCP_PORT: ATTRIBUTEID = ATTRIBUTEID(16u32);
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkPolicyServer\"`*"]
pub const RADIUS_ATTRIBUTE_UNASSIGNED1: ATTRIBUTEID = ATTRIBUTEID(17u32);
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkPolicyServer\"`*"]
pub const RADIUS_ATTRIBUTE_REPLY_MESSAGE: ATTRIBUTEID = ATTRIBUTEID(18u32);
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkPolicyServer\"`*"]
pub const RADIUS_ATTRIBUTE_CALLBACK_NUMBER: ATTRIBUTEID = ATTRIBUTEID(19u32);
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkPolicyServer\"`*"]
pub const RADIUS_ATTRIBUTE_CALLBACK_ID: ATTRIBUTEID = ATTRIBUTEID(20u32);
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkPolicyServer\"`*"]
pub const RADIUS_ATTRIBUTE_UNASSIGNED2: ATTRIBUTEID = ATTRIBUTEID(21u32);
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkPolicyServer\"`*"]
pub const RADIUS_ATTRIBUTE_FRAMED_ROUTE: ATTRIBUTEID = ATTRIBUTEID(22u32);
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkPolicyServer\"`*"]
pub const RADIUS_ATTRIBUTE_FRAMED_IPX_NETWORK: ATTRIBUTEID = ATTRIBUTEID(23u32);
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkPolicyServer\"`*"]
pub const RADIUS_ATTRIBUTE_STATE: ATTRIBUTEID = ATTRIBUTEID(24u32);
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkPolicyServer\"`*"]
pub const RADIUS_ATTRIBUTE_CLASS: ATTRIBUTEID = ATTRIBUTEID(25u32);
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkPolicyServer\"`*"]
pub const RADIUS_ATTRIBUTE_VENDOR_SPECIFIC: ATTRIBUTEID = ATTRIBUTEID(26u32);
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkPolicyServer\"`*"]
pub const RADIUS_ATTRIBUTE_SESSION_TIMEOUT: ATTRIBUTEID = ATTRIBUTEID(27u32);
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkPolicyServer\"`*"]
pub const RADIUS_ATTRIBUTE_IDLE_TIMEOUT: ATTRIBUTEID = ATTRIBUTEID(28u32);
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkPolicyServer\"`*"]
pub const RADIUS_ATTRIBUTE_TERMINATION_ACTION: ATTRIBUTEID = ATTRIBUTEID(29u32);
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkPolicyServer\"`*"]
pub const RADIUS_ATTRIBUTE_CALLED_STATION_ID: ATTRIBUTEID = ATTRIBUTEID(30u32);
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkPolicyServer\"`*"]
pub const RADIUS_ATTRIBUTE_CALLING_STATION_ID: ATTRIBUTEID = ATTRIBUTEID(31u32);
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkPolicyServer\"`*"]
pub const RADIUS_ATTRIBUTE_NAS_IDENTIFIER: ATTRIBUTEID = ATTRIBUTEID(32u32);
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkPolicyServer\"`*"]
pub const RADIUS_ATTRIBUTE_PROXY_STATE: ATTRIBUTEID = ATTRIBUTEID(33u32);
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkPolicyServer\"`*"]
pub const RADIUS_ATTRIBUTE_LOGIN_LAT_SERVICE: ATTRIBUTEID = ATTRIBUTEID(34u32);
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkPolicyServer\"`*"]
pub const RADIUS_ATTRIBUTE_LOGIN_LAT_NODE: ATTRIBUTEID = ATTRIBUTEID(35u32);
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkPolicyServer\"`*"]
pub const RADIUS_ATTRIBUTE_LOGIN_LAT_GROUP: ATTRIBUTEID = ATTRIBUTEID(36u32);
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkPolicyServer\"`*"]
pub const RADIUS_ATTRIBUTE_FRAMED_APPLETALK_LINK: ATTRIBUTEID = ATTRIBUTEID(37u32);
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkPolicyServer\"`*"]
pub const RADIUS_ATTRIBUTE_FRAMED_APPLETALK_NET: ATTRIBUTEID = ATTRIBUTEID(38u32);
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkPolicyServer\"`*"]
pub const RADIUS_ATTRIBUTE_FRAMED_APPLETALK_ZONE: ATTRIBUTEID = ATTRIBUTEID(39u32);
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkPolicyServer\"`*"]
pub const RADIUS_ATTRIBUTE_ACCT_STATUS_TYPE: ATTRIBUTEID = ATTRIBUTEID(40u32);
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkPolicyServer\"`*"]
pub const RADIUS_ATTRIBUTE_ACCT_DELAY_TIME: ATTRIBUTEID = ATTRIBUTEID(41u32);
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkPolicyServer\"`*"]
pub const RADIUS_ATTRIBUTE_ACCT_INPUT_OCTETS: ATTRIBUTEID = ATTRIBUTEID(42u32);
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkPolicyServer\"`*"]
pub const RADIUS_ATTRIBUTE_ACCT_OUTPUT_OCTETS: ATTRIBUTEID = ATTRIBUTEID(43u32);
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkPolicyServer\"`*"]
pub const RADIUS_ATTRIBUTE_ACCT_SESSION_ID: ATTRIBUTEID = ATTRIBUTEID(44u32);
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkPolicyServer\"`*"]
pub const RADIUS_ATTRIBUTE_ACCT_AUTHENTIC: ATTRIBUTEID = ATTRIBUTEID(45u32);
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkPolicyServer\"`*"]
pub const RADIUS_ATTRIBUTE_ACCT_SESSION_TIME: ATTRIBUTEID = ATTRIBUTEID(46u32);
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkPolicyServer\"`*"]
pub const RADIUS_ATTRIBUTE_ACCT_INPUT_PACKETS: ATTRIBUTEID = ATTRIBUTEID(47u32);
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkPolicyServer\"`*"]
pub const RADIUS_ATTRIBUTE_ACCT_OUTPUT_PACKETS: ATTRIBUTEID = ATTRIBUTEID(48u32);
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkPolicyServer\"`*"]
pub const RADIUS_ATTRIBUTE_ACCT_TERMINATE_CAUSE: ATTRIBUTEID = ATTRIBUTEID(49u32);
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkPolicyServer\"`*"]
pub const RADIUS_ATTRIBUTE_ACCT_MULTI_SSN_ID: ATTRIBUTEID = ATTRIBUTEID(50u32);
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkPolicyServer\"`*"]
pub const RADIUS_ATTRIBUTE_ACCT_LINK_COUNT: ATTRIBUTEID = ATTRIBUTEID(51u32);
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkPolicyServer\"`*"]
pub const RADIUS_ATTRIBUTE_CHAP_CHALLENGE: ATTRIBUTEID = ATTRIBUTEID(60u32);
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkPolicyServer\"`*"]
pub const RADIUS_ATTRIBUTE_NAS_PORT_TYPE: ATTRIBUTEID = ATTRIBUTEID(61u32);
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkPolicyServer\"`*"]
pub const RADIUS_ATTRIBUTE_PORT_LIMIT: ATTRIBUTEID = ATTRIBUTEID(62u32);
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkPolicyServer\"`*"]
pub const RADIUS_ATTRIBUTE_LOGIN_LAT_PORT: ATTRIBUTEID = ATTRIBUTEID(63u32);
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkPolicyServer\"`*"]
pub const RADIUS_ATTRIBUTE_TUNNEL_TYPE: ATTRIBUTEID = ATTRIBUTEID(64u32);
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkPolicyServer\"`*"]
pub const RADIUS_ATTRIBUTE_TUNNEL_MEDIUM_TYPE: ATTRIBUTEID = ATTRIBUTEID(65u32);
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkPolicyServer\"`*"]
pub const RADIUS_ATTRIBUTE_TUNNEL_CLIENT_ENDPT: ATTRIBUTEID = ATTRIBUTEID(66u32);
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkPolicyServer\"`*"]
pub const RADIUS_ATTRIBUTE_TUNNEL_SERVER_ENDPT: ATTRIBUTEID = ATTRIBUTEID(67u32);
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkPolicyServer\"`*"]
pub const RADIUS_ATTRIBUTE_ACCT_TUNNEL_CONN: ATTRIBUTEID = ATTRIBUTEID(68u32);
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkPolicyServer\"`*"]
pub const RADIUS_ATTRIBUTE_TUNNEL_PASSWORD: ATTRIBUTEID = ATTRIBUTEID(69u32);
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkPolicyServer\"`*"]
pub const RADIUS_ATTRIBUTE_ARAP_PASSWORD: ATTRIBUTEID = ATTRIBUTEID(70u32);
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkPolicyServer\"`*"]
pub const RADIUS_ATTRIBUTE_ARAP_FEATURES: ATTRIBUTEID = ATTRIBUTEID(71u32);
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkPolicyServer\"`*"]
pub const RADIUS_ATTRIBUTE_ARAP_ZONE_ACCESS: ATTRIBUTEID = ATTRIBUTEID(72u32);
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkPolicyServer\"`*"]
pub const RADIUS_ATTRIBUTE_ARAP_SECURITY: ATTRIBUTEID = ATTRIBUTEID(73u32);
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkPolicyServer\"`*"]
pub const RADIUS_ATTRIBUTE_ARAP_SECURITY_DATA: ATTRIBUTEID = ATTRIBUTEID(74u32);
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkPolicyServer\"`*"]
pub const RADIUS_ATTRIBUTE_PASSWORD_RETRY: ATTRIBUTEID = ATTRIBUTEID(75u32);
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkPolicyServer\"`*"]
pub const RADIUS_ATTRIBUTE_PROMPT: ATTRIBUTEID = ATTRIBUTEID(76u32);
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkPolicyServer\"`*"]
pub const RADIUS_ATTRIBUTE_CONNECT_INFO: ATTRIBUTEID = ATTRIBUTEID(77u32);
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkPolicyServer\"`*"]
pub const RADIUS_ATTRIBUTE_CONFIGURATION_TOKEN: ATTRIBUTEID = ATTRIBUTEID(78u32);
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkPolicyServer\"`*"]
pub const RADIUS_ATTRIBUTE_EAP_MESSAGE: ATTRIBUTEID = ATTRIBUTEID(79u32);
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkPolicyServer\"`*"]
pub const RADIUS_ATTRIBUTE_SIGNATURE: ATTRIBUTEID = ATTRIBUTEID(80u32);
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkPolicyServer\"`*"]
pub const RADIUS_ATTRIBUTE_TUNNEL_PVT_GROUP_ID: ATTRIBUTEID = ATTRIBUTEID(81u32);
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkPolicyServer\"`*"]
pub const RADIUS_ATTRIBUTE_TUNNEL_ASSIGNMENT_ID: ATTRIBUTEID = ATTRIBUTEID(82u32);
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkPolicyServer\"`*"]
pub const RADIUS_ATTRIBUTE_TUNNEL_PREFERENCE: ATTRIBUTEID = ATTRIBUTEID(83u32);
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkPolicyServer\"`*"]
pub const RADIUS_ATTRIBUTE_ARAP_CHALLENGE_RESPONSE: ATTRIBUTEID = ATTRIBUTEID(84u32);
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkPolicyServer\"`*"]
pub const RADIUS_ATTRIBUTE_ACCT_INTERIM_INTERVAL: ATTRIBUTEID = ATTRIBUTEID(85u32);
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkPolicyServer\"`*"]
pub const RADIUS_ATTRIBUTE_NAS_IPv6_ADDRESS: ATTRIBUTEID = ATTRIBUTEID(95u32);
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkPolicyServer\"`*"]
pub const RADIUS_ATTRIBUTE_FRAMED_INTERFACE_ID: ATTRIBUTEID = ATTRIBUTEID(96u32);
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkPolicyServer\"`*"]
pub const RADIUS_ATTRIBUTE_FRAMED_IPv6_PREFIX: ATTRIBUTEID = ATTRIBUTEID(97u32);
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkPolicyServer\"`*"]
pub const RADIUS_ATTRIBUTE_LOGIN_IPv6_HOST: ATTRIBUTEID = ATTRIBUTEID(98u32);
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkPolicyServer\"`*"]
pub const RADIUS_ATTRIBUTE_FRAMED_IPv6_ROUTE: ATTRIBUTEID = ATTRIBUTEID(99u32);
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkPolicyServer\"`*"]
pub const RADIUS_ATTRIBUTE_FRAMED_IPv6_POOL: ATTRIBUTEID = ATTRIBUTEID(100u32);
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkPolicyServer\"`*"]
pub const IAS_ATTRIBUTE_SAVED_RADIUS_FRAMED_IP_ADDRESS: ATTRIBUTEID = ATTRIBUTEID(4096u32);
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkPolicyServer\"`*"]
pub const IAS_ATTRIBUTE_SAVED_RADIUS_CALLBACK_NUMBER: ATTRIBUTEID = ATTRIBUTEID(4097u32);
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkPolicyServer\"`*"]
pub const IAS_ATTRIBUTE_NP_CALLING_STATION_ID: ATTRIBUTEID = ATTRIBUTEID(4098u32);
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkPolicyServer\"`*"]
pub const IAS_ATTRIBUTE_SAVED_NP_CALLING_STATION_ID: ATTRIBUTEID = ATTRIBUTEID(4099u32);
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkPolicyServer\"`*"]
pub const IAS_ATTRIBUTE_SAVED_RADIUS_FRAMED_ROUTE: ATTRIBUTEID = ATTRIBUTEID(4100u32);
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkPolicyServer\"`*"]
pub const IAS_ATTRIBUTE_IGNORE_USER_DIALIN_PROPERTIES: ATTRIBUTEID = ATTRIBUTEID(4101u32);
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkPolicyServer\"`*"]
pub const IAS_ATTRIBUTE_NP_TIME_OF_DAY: ATTRIBUTEID = ATTRIBUTEID(4102u32);
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkPolicyServer\"`*"]
pub const IAS_ATTRIBUTE_NP_CALLED_STATION_ID: ATTRIBUTEID = ATTRIBUTEID(4103u32);
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkPolicyServer\"`*"]
pub const IAS_ATTRIBUTE_NP_ALLOWED_PORT_TYPES: ATTRIBUTEID = ATTRIBUTEID(4104u32);
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkPolicyServer\"`*"]
pub const IAS_ATTRIBUTE_NP_AUTHENTICATION_TYPE: ATTRIBUTEID = ATTRIBUTEID(4105u32);
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkPolicyServer\"`*"]
pub const IAS_ATTRIBUTE_NP_ALLOWED_EAP_TYPE: ATTRIBUTEID = ATTRIBUTEID(4106u32);
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkPolicyServer\"`*"]
pub const IAS_ATTRIBUTE_SHARED_SECRET: ATTRIBUTEID = ATTRIBUTEID(4107u32);
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkPolicyServer\"`*"]
pub const IAS_ATTRIBUTE_CLIENT_IP_ADDRESS: ATTRIBUTEID = ATTRIBUTEID(4108u32);
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkPolicyServer\"`*"]
pub const IAS_ATTRIBUTE_CLIENT_PACKET_HEADER: ATTRIBUTEID = ATTRIBUTEID(4109u32);
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkPolicyServer\"`*"]
pub const IAS_ATTRIBUTE_TOKEN_GROUPS: ATTRIBUTEID = ATTRIBUTEID(4110u32);
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkPolicyServer\"`*"]
pub const IAS_ATTRIBUTE_ALLOW_DIALIN: ATTRIBUTEID = ATTRIBUTEID(4111u32);
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkPolicyServer\"`*"]
pub const IAS_ATTRIBUTE_REQUEST_ID: ATTRIBUTEID = ATTRIBUTEID(4112u32);
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkPolicyServer\"`*"]
pub const IAS_ATTRIBUTE_MANIPULATION_TARGET: ATTRIBUTEID = ATTRIBUTEID(4113u32);
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkPolicyServer\"`*"]
pub const IAS_ATTRIBUTE_MANIPULATION_RULE: ATTRIBUTEID = ATTRIBUTEID(4114u32);
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkPolicyServer\"`*"]
pub const IAS_ATTRIBUTE_ORIGINAL_USER_NAME: ATTRIBUTEID = ATTRIBUTEID(4115u32);
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkPolicyServer\"`*"]
pub const IAS_ATTRIBUTE_CLIENT_VENDOR_TYPE: ATTRIBUTEID = ATTRIBUTEID(4116u32);
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkPolicyServer\"`*"]
pub const IAS_ATTRIBUTE_CLIENT_UDP_PORT: ATTRIBUTEID = ATTRIBUTEID(4117u32);
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkPolicyServer\"`*"]
pub const MS_ATTRIBUTE_CHAP_CHALLENGE: ATTRIBUTEID = ATTRIBUTEID(4118u32);
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkPolicyServer\"`*"]
pub const MS_ATTRIBUTE_CHAP_RESPONSE: ATTRIBUTEID = ATTRIBUTEID(4119u32);
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkPolicyServer\"`*"]
pub const MS_ATTRIBUTE_CHAP_DOMAIN: ATTRIBUTEID = ATTRIBUTEID(4120u32);
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkPolicyServer\"`*"]
pub const MS_ATTRIBUTE_CHAP_ERROR: ATTRIBUTEID = ATTRIBUTEID(4121u32);
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkPolicyServer\"`*"]
pub const MS_ATTRIBUTE_CHAP_CPW1: ATTRIBUTEID = ATTRIBUTEID(4122u32);
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkPolicyServer\"`*"]
pub const MS_ATTRIBUTE_CHAP_CPW2: ATTRIBUTEID = ATTRIBUTEID(4123u32);
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkPolicyServer\"`*"]
pub const MS_ATTRIBUTE_CHAP_LM_ENC_PW: ATTRIBUTEID = ATTRIBUTEID(4124u32);
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkPolicyServer\"`*"]
pub const MS_ATTRIBUTE_CHAP_NT_ENC_PW: ATTRIBUTEID = ATTRIBUTEID(4125u32);
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkPolicyServer\"`*"]
pub const MS_ATTRIBUTE_CHAP_MPPE_KEYS: ATTRIBUTEID = ATTRIBUTEID(4126u32);
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkPolicyServer\"`*"]
pub const IAS_ATTRIBUTE_AUTHENTICATION_TYPE: ATTRIBUTEID = ATTRIBUTEID(4127u32);
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkPolicyServer\"`*"]
pub const IAS_ATTRIBUTE_CLIENT_NAME: ATTRIBUTEID = ATTRIBUTEID(4128u32);
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkPolicyServer\"`*"]
pub const IAS_ATTRIBUTE_NT4_ACCOUNT_NAME: ATTRIBUTEID = ATTRIBUTEID(4129u32);
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkPolicyServer\"`*"]
pub const IAS_ATTRIBUTE_FULLY_QUALIFIED_USER_NAME: ATTRIBUTEID = ATTRIBUTEID(4130u32);
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkPolicyServer\"`*"]
pub const IAS_ATTRIBUTE_NTGROUPS: ATTRIBUTEID = ATTRIBUTEID(4131u32);
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkPolicyServer\"`*"]
pub const IAS_ATTRIBUTE_EAP_FRIENDLY_NAME: ATTRIBUTEID = ATTRIBUTEID(4132u32);
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkPolicyServer\"`*"]
pub const IAS_ATTRIBUTE_AUTH_PROVIDER_TYPE: ATTRIBUTEID = ATTRIBUTEID(4133u32);
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkPolicyServer\"`*"]
pub const MS_ATTRIBUTE_ACCT_AUTH_TYPE: ATTRIBUTEID = ATTRIBUTEID(4134u32);
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkPolicyServer\"`*"]
pub const MS_ATTRIBUTE_ACCT_EAP_TYPE: ATTRIBUTEID = ATTRIBUTEID(4135u32);
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkPolicyServer\"`*"]
pub const IAS_ATTRIBUTE_PACKET_TYPE: ATTRIBUTEID = ATTRIBUTEID(4136u32);
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkPolicyServer\"`*"]
pub const IAS_ATTRIBUTE_AUTH_PROVIDER_NAME: ATTRIBUTEID = ATTRIBUTEID(4137u32);
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkPolicyServer\"`*"]
pub const IAS_ATTRIBUTE_ACCT_PROVIDER_TYPE: ATTRIBUTEID = ATTRIBUTEID(4138u32);
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkPolicyServer\"`*"]
pub const IAS_ATTRIBUTE_ACCT_PROVIDER_NAME: ATTRIBUTEID = ATTRIBUTEID(4139u32);
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkPolicyServer\"`*"]
pub const MS_ATTRIBUTE_MPPE_SEND_KEY: ATTRIBUTEID = ATTRIBUTEID(4140u32);
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkPolicyServer\"`*"]
pub const MS_ATTRIBUTE_MPPE_RECV_KEY: ATTRIBUTEID = ATTRIBUTEID(4141u32);
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkPolicyServer\"`*"]
pub const IAS_ATTRIBUTE_REASON_CODE: ATTRIBUTEID = ATTRIBUTEID(4142u32);
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkPolicyServer\"`*"]
pub const MS_ATTRIBUTE_FILTER: ATTRIBUTEID = ATTRIBUTEID(4143u32);
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkPolicyServer\"`*"]
pub const MS_ATTRIBUTE_CHAP2_RESPONSE: ATTRIBUTEID = ATTRIBUTEID(4144u32);
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkPolicyServer\"`*"]
pub const MS_ATTRIBUTE_CHAP2_SUCCESS: ATTRIBUTEID = ATTRIBUTEID(4145u32);
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkPolicyServer\"`*"]
pub const MS_ATTRIBUTE_CHAP2_CPW: ATTRIBUTEID = ATTRIBUTEID(4146u32);
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkPolicyServer\"`*"]
pub const MS_ATTRIBUTE_RAS_VENDOR: ATTRIBUTEID = ATTRIBUTEID(4147u32);
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkPolicyServer\"`*"]
pub const MS_ATTRIBUTE_RAS_VERSION: ATTRIBUTEID = ATTRIBUTEID(4148u32);
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkPolicyServer\"`*"]
pub const IAS_ATTRIBUTE_NP_NAME: ATTRIBUTEID = ATTRIBUTEID(4149u32);
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkPolicyServer\"`*"]
pub const MS_ATTRIBUTE_PRIMARY_DNS_SERVER: ATTRIBUTEID = ATTRIBUTEID(4150u32);
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkPolicyServer\"`*"]
pub const MS_ATTRIBUTE_SECONDARY_DNS_SERVER: ATTRIBUTEID = ATTRIBUTEID(4151u32);
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkPolicyServer\"`*"]
pub const MS_ATTRIBUTE_PRIMARY_NBNS_SERVER: ATTRIBUTEID = ATTRIBUTEID(4152u32);
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkPolicyServer\"`*"]
pub const MS_ATTRIBUTE_SECONDARY_NBNS_SERVER: ATTRIBUTEID = ATTRIBUTEID(4153u32);
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkPolicyServer\"`*"]
pub const IAS_ATTRIBUTE_PROXY_POLICY_NAME: ATTRIBUTEID = ATTRIBUTEID(4154u32);
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkPolicyServer\"`*"]
pub const IAS_ATTRIBUTE_PROVIDER_TYPE: ATTRIBUTEID = ATTRIBUTEID(4155u32);
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkPolicyServer\"`*"]
pub const IAS_ATTRIBUTE_PROVIDER_NAME: ATTRIBUTEID = ATTRIBUTEID(4156u32);
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkPolicyServer\"`*"]
pub const IAS_ATTRIBUTE_REMOTE_SERVER_ADDRESS: ATTRIBUTEID = ATTRIBUTEID(4157u32);
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkPolicyServer\"`*"]
pub const IAS_ATTRIBUTE_GENERATE_CLASS_ATTRIBUTE: ATTRIBUTEID = ATTRIBUTEID(4158u32);
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkPolicyServer\"`*"]
pub const MS_ATTRIBUTE_RAS_CLIENT_NAME: ATTRIBUTEID = ATTRIBUTEID(4159u32);
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkPolicyServer\"`*"]
pub const MS_ATTRIBUTE_RAS_CLIENT_VERSION: ATTRIBUTEID = ATTRIBUTEID(4160u32);
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkPolicyServer\"`*"]
pub const IAS_ATTRIBUTE_ALLOWED_CERTIFICATE_EKU: ATTRIBUTEID = ATTRIBUTEID(4161u32);
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkPolicyServer\"`*"]
pub const IAS_ATTRIBUTE_EXTENSION_STATE: ATTRIBUTEID = ATTRIBUTEID(4162u32);
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkPolicyServer\"`*"]
pub const IAS_ATTRIBUTE_GENERATE_SESSION_TIMEOUT: ATTRIBUTEID = ATTRIBUTEID(4163u32);
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkPolicyServer\"`*"]
pub const IAS_ATTRIBUTE_SESSION_TIMEOUT: ATTRIBUTEID = ATTRIBUTEID(4164u32);
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkPolicyServer\"`*"]
pub const MS_ATTRIBUTE_QUARANTINE_IPFILTER: ATTRIBUTEID = ATTRIBUTEID(4165u32);
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkPolicyServer\"`*"]
pub const MS_ATTRIBUTE_QUARANTINE_SESSION_TIMEOUT: ATTRIBUTEID = ATTRIBUTEID(4166u32);
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkPolicyServer\"`*"]
pub const MS_ATTRIBUTE_USER_SECURITY_IDENTITY: ATTRIBUTEID = ATTRIBUTEID(4167u32);
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkPolicyServer\"`*"]
pub const IAS_ATTRIBUTE_REMOTE_RADIUS_TO_WINDOWS_USER_MAPPING: ATTRIBUTEID = ATTRIBUTEID(4168u32);
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkPolicyServer\"`*"]
pub const IAS_ATTRIBUTE_PASSPORT_USER_MAPPING_UPN_SUFFIX: ATTRIBUTEID = ATTRIBUTEID(4169u32);
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkPolicyServer\"`*"]
pub const IAS_ATTRIBUTE_TUNNEL_TAG: ATTRIBUTEID = ATTRIBUTEID(4170u32);
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkPolicyServer\"`*"]
pub const IAS_ATTRIBUTE_NP_PEAPUPFRONT_ENABLED: ATTRIBUTEID = ATTRIBUTEID(4171u32);
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkPolicyServer\"`*"]
pub const IAS_ATTRIBUTE_CERTIFICATE_EKU: ATTRIBUTEID = ATTRIBUTEID(8097u32);
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkPolicyServer\"`*"]
pub const IAS_ATTRIBUTE_EAP_CONFIG: ATTRIBUTEID = ATTRIBUTEID(8098u32);
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkPolicyServer\"`*"]
pub const IAS_ATTRIBUTE_PEAP_EMBEDDED_EAP_TYPEID: ATTRIBUTEID = ATTRIBUTEID(8099u32);
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkPolicyServer\"`*"]
pub const IAS_ATTRIBUTE_PEAP_FAST_ROAMED_SESSION: ATTRIBUTEID = ATTRIBUTEID(8100u32);
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkPolicyServer\"`*"]
pub const IAS_ATTRIBUTE_EAP_TYPEID: ATTRIBUTEID = ATTRIBUTEID(8101u32);
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkPolicyServer\"`*"]
pub const MS_ATTRIBUTE_EAP_TLV: ATTRIBUTEID = ATTRIBUTEID(8102u32);
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkPolicyServer\"`*"]
pub const IAS_ATTRIBUTE_REJECT_REASON_CODE: ATTRIBUTEID = ATTRIBUTEID(8103u32);
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkPolicyServer\"`*"]
pub const IAS_ATTRIBUTE_PROXY_EAP_CONFIG: ATTRIBUTEID = ATTRIBUTEID(8104u32);
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkPolicyServer\"`*"]
pub const IAS_ATTRIBUTE_EAP_SESSION: ATTRIBUTEID = ATTRIBUTEID(8105u32);
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkPolicyServer\"`*"]
pub const IAS_ATTRIBUTE_IS_REPLAY: ATTRIBUTEID = ATTRIBUTEID(8106u32);
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkPolicyServer\"`*"]
pub const IAS_ATTRIBUTE_CLEAR_TEXT_PASSWORD: ATTRIBUTEID = ATTRIBUTEID(8107u32);
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkPolicyServer\"`*"]
pub const MS_ATTRIBUTE_IDENTITY_TYPE: ATTRIBUTEID = ATTRIBUTEID(8108u32);
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkPolicyServer\"`*"]
pub const MS_ATTRIBUTE_SERVICE_CLASS: ATTRIBUTEID = ATTRIBUTEID(8109u32);
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkPolicyServer\"`*"]
pub const MS_ATTRIBUTE_QUARANTINE_USER_CLASS: ATTRIBUTEID = ATTRIBUTEID(8110u32);
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkPolicyServer\"`*"]
pub const MS_ATTRIBUTE_QUARANTINE_STATE: ATTRIBUTEID = ATTRIBUTEID(8111u32);
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkPolicyServer\"`*"]
pub const IAS_ATTRIBUTE_OVERRIDE_RAP_AUTH: ATTRIBUTEID = ATTRIBUTEID(8112u32);
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkPolicyServer\"`*"]
pub const IAS_ATTRIBUTE_PEAP_CHANNEL_UP: ATTRIBUTEID = ATTRIBUTEID(8113u32);
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkPolicyServer\"`*"]
pub const IAS_ATTRIBUTE_NAME_MAPPED: ATTRIBUTEID = ATTRIBUTEID(8114u32);
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkPolicyServer\"`*"]
pub const IAS_ATTRIBUTE_POLICY_ENFORCED: ATTRIBUTEID = ATTRIBUTEID(8115u32);
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkPolicyServer\"`*"]
pub const IAS_ATTRIBUTE_MACHINE_NTGROUPS: ATTRIBUTEID = ATTRIBUTEID(8116u32);
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkPolicyServer\"`*"]
pub const IAS_ATTRIBUTE_USER_NTGROUPS: ATTRIBUTEID = ATTRIBUTEID(8117u32);
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkPolicyServer\"`*"]
pub const IAS_ATTRIBUTE_MACHINE_TOKEN_GROUPS: ATTRIBUTEID = ATTRIBUTEID(8118u32);
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkPolicyServer\"`*"]
pub const IAS_ATTRIBUTE_USER_TOKEN_GROUPS: ATTRIBUTEID = ATTRIBUTEID(8119u32);
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkPolicyServer\"`*"]
pub const MS_ATTRIBUTE_QUARANTINE_GRACE_TIME: ATTRIBUTEID = ATTRIBUTEID(8120u32);
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkPolicyServer\"`*"]
pub const IAS_ATTRIBUTE_QUARANTINE_URL: ATTRIBUTEID = ATTRIBUTEID(8121u32);
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkPolicyServer\"`*"]
pub const IAS_ATTRIBUTE_QUARANTINE_FIXUP_SERVERS: ATTRIBUTEID = ATTRIBUTEID(8122u32);
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkPolicyServer\"`*"]
pub const MS_ATTRIBUTE_NOT_QUARANTINE_CAPABLE: ATTRIBUTEID = ATTRIBUTEID(8123u32);
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkPolicyServer\"`*"]
pub const IAS_ATTRIBUTE_QUARANTINE_SYSTEM_HEALTH_RESULT: ATTRIBUTEID = ATTRIBUTEID(8124u32);
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkPolicyServer\"`*"]
pub const IAS_ATTRIBUTE_QUARANTINE_SYSTEM_HEALTH_VALIDATORS: ATTRIBUTEID = ATTRIBUTEID(8125u32);
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkPolicyServer\"`*"]
pub const IAS_ATTRIBUTE_MACHINE_NAME: ATTRIBUTEID = ATTRIBUTEID(8126u32);
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkPolicyServer\"`*"]
pub const IAS_ATTRIBUTE_NT4_MACHINE_NAME: ATTRIBUTEID = ATTRIBUTEID(8127u32);
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkPolicyServer\"`*"]
pub const IAS_ATTRIBUTE_QUARANTINE_SESSION_HANDLE: ATTRIBUTEID = ATTRIBUTEID(8128u32);
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkPolicyServer\"`*"]
pub const IAS_ATTRIBUTE_FULLY_QUALIFIED_MACHINE_NAME: ATTRIBUTEID = ATTRIBUTEID(8129u32);
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkPolicyServer\"`*"]
pub const IAS_ATTRIBUTE_QUARANTINE_FIXUP_SERVERS_CONFIGURATION: ATTRIBUTEID = ATTRIBUTEID(8130u32);
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkPolicyServer\"`*"]
pub const IAS_ATTRIBUTE_CLIENT_QUARANTINE_COMPATIBLE: ATTRIBUTEID = ATTRIBUTEID(8131u32);
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkPolicyServer\"`*"]
pub const MS_ATTRIBUTE_NETWORK_ACCESS_SERVER_TYPE: ATTRIBUTEID = ATTRIBUTEID(8132u32);
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkPolicyServer\"`*"]
pub const IAS_ATTRIBUTE_QUARANTINE_SESSION_ID: ATTRIBUTEID = ATTRIBUTEID(8133u32);
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkPolicyServer\"`*"]
pub const MS_ATTRIBUTE_AFW_QUARANTINE_ZONE: ATTRIBUTEID = ATTRIBUTEID(8134u32);
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkPolicyServer\"`*"]
pub const MS_ATTRIBUTE_AFW_PROTECTION_LEVEL: ATTRIBUTEID = ATTRIBUTEID(8135u32);
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkPolicyServer\"`*"]
pub const IAS_ATTRIBUTE_QUARANTINE_UPDATE_NON_COMPLIANT: ATTRIBUTEID = ATTRIBUTEID(8136u32);
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkPolicyServer\"`*"]
pub const IAS_ATTRIBUTE_REQUEST_START_TIME: ATTRIBUTEID = ATTRIBUTEID(8137u32);
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkPolicyServer\"`*"]
pub const MS_ATTRIBUTE_MACHINE_NAME: ATTRIBUTEID = ATTRIBUTEID(8138u32);
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkPolicyServer\"`*"]
pub const IAS_ATTRIBUTE_CLIENT_IPv6_ADDRESS: ATTRIBUTEID = ATTRIBUTEID(8139u32);
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkPolicyServer\"`*"]
pub const IAS_ATTRIBUTE_SAVED_RADIUS_FRAMED_INTERFACE_ID: ATTRIBUTEID = ATTRIBUTEID(8140u32);
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkPolicyServer\"`*"]
pub const IAS_ATTRIBUTE_SAVED_RADIUS_FRAMED_IPv6_PREFIX: ATTRIBUTEID = ATTRIBUTEID(8141u32);
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkPolicyServer\"`*"]
pub const IAS_ATTRIBUTE_SAVED_RADIUS_FRAMED_IPv6_ROUTE: ATTRIBUTEID = ATTRIBUTEID(8142u32);
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkPolicyServer\"`*"]
pub const MS_ATTRIBUTE_QUARANTINE_GRACE_TIME_CONFIGURATION: ATTRIBUTEID = ATTRIBUTEID(8143u32);
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkPolicyServer\"`*"]
pub const MS_ATTRIBUTE_IPv6_FILTER: ATTRIBUTEID = ATTRIBUTEID(8144u32);
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkPolicyServer\"`*"]
pub const MS_ATTRIBUTE_IPV4_REMEDIATION_SERVERS: ATTRIBUTEID = ATTRIBUTEID(8145u32);
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkPolicyServer\"`*"]
pub const MS_ATTRIBUTE_IPV6_REMEDIATION_SERVERS: ATTRIBUTEID = ATTRIBUTEID(8146u32);
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkPolicyServer\"`*"]
pub const IAS_ATTRIBUTE_PROXY_RETRY_COUNT: ATTRIBUTEID = ATTRIBUTEID(8147u32);
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkPolicyServer\"`*"]
pub const IAS_ATTRIBUTE_MACHINE_INVENTORY: ATTRIBUTEID = ATTRIBUTEID(8148u32);
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkPolicyServer\"`*"]
pub const IAS_ATTRIBUTE_ABSOLUTE_TIME: ATTRIBUTEID = ATTRIBUTEID(8149u32);
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkPolicyServer\"`*"]
pub const MS_ATTRIBUTE_QUARANTINE_SOH: ATTRIBUTEID = ATTRIBUTEID(8150u32);
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkPolicyServer\"`*"]
pub const IAS_ATTRIBUTE_EAP_TYPES_CONFIGURED_IN_PROXYPOLICY: ATTRIBUTEID = ATTRIBUTEID(8151u32);
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkPolicyServer\"`*"]
pub const MS_ATTRIBUTE_HCAP_LOCATION_GROUP_NAME: ATTRIBUTEID = ATTRIBUTEID(8152u32);
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkPolicyServer\"`*"]
pub const MS_ATTRIBUTE_EXTENDED_QUARANTINE_STATE: ATTRIBUTEID = ATTRIBUTEID(8153u32);
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkPolicyServer\"`*"]
pub const IAS_ATTRIBUTE_SOH_CARRIER_EAPTLV: ATTRIBUTEID = ATTRIBUTEID(8154u32);
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkPolicyServer\"`*"]
pub const MS_ATTRIBUTE_HCAP_USER_GROUPS: ATTRIBUTEID = ATTRIBUTEID(8155u32);
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkPolicyServer\"`*"]
pub const IAS_ATTRIBUTE_SAVED_MACHINE_HEALTHCHECK_ONLY: ATTRIBUTEID = ATTRIBUTEID(8156u32);
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkPolicyServer\"`*"]
pub const IAS_ATTRIBUTE_POLICY_EVALUATED_SHV: ATTRIBUTEID = ATTRIBUTEID(8157u32);
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkPolicyServer\"`*"]
pub const MS_ATTRIBUTE_RAS_CORRELATION_ID: ATTRIBUTEID = ATTRIBUTEID(8158u32);
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkPolicyServer\"`*"]
pub const MS_ATTRIBUTE_HCAP_USER_NAME: ATTRIBUTEID = ATTRIBUTEID(8159u32);
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkPolicyServer\"`*"]
pub const IAS_ATTRIBUTE_NT4_HCAP_ACCOUNT_NAME: ATTRIBUTEID = ATTRIBUTEID(8160u32);
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkPolicyServer\"`*"]
pub const IAS_ATTRIBUTE_USER_TOKEN_SID: ATTRIBUTEID = ATTRIBUTEID(8161u32);
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkPolicyServer\"`*"]
pub const IAS_ATTRIBUTE_MACHINE_TOKEN_SID: ATTRIBUTEID = ATTRIBUTEID(8162u32);
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkPolicyServer\"`*"]
pub const IAS_ATTRIBUTE_MACHINE_VALIDATED: ATTRIBUTEID = ATTRIBUTEID(8163u32);
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkPolicyServer\"`*"]
pub const MS_ATTRIBUTE_USER_IPv4_ADDRESS: ATTRIBUTEID = ATTRIBUTEID(8164u32);
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkPolicyServer\"`*"]
pub const MS_ATTRIBUTE_USER_IPv6_ADDRESS: ATTRIBUTEID = ATTRIBUTEID(8165u32);
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkPolicyServer\"`*"]
pub const MS_ATTRIBUTE_TSG_DEVICE_REDIRECTION: ATTRIBUTEID = ATTRIBUTEID(8166u32);
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkPolicyServer\"`*"]
pub const IAS_ATTRIBUTE_ACCEPT_REASON_CODE: ATTRIBUTEID = ATTRIBUTEID(8167u32);
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkPolicyServer\"`*"]
pub const IAS_ATTRIBUTE_LOGGING_RESULT: ATTRIBUTEID = ATTRIBUTEID(8168u32);
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkPolicyServer\"`*"]
pub const IAS_ATTRIBUTE_SERVER_IP_ADDRESS: ATTRIBUTEID = ATTRIBUTEID(8169u32);
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkPolicyServer\"`*"]
pub const IAS_ATTRIBUTE_SERVER_IPv6_ADDRESS: ATTRIBUTEID = ATTRIBUTEID(8170u32);
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkPolicyServer\"`*"]
pub const IAS_ATTRIBUTE_RADIUS_USERNAME_ENCODING_ASCII: ATTRIBUTEID = ATTRIBUTEID(8171u32);
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkPolicyServer\"`*"]
pub const MS_ATTRIBUTE_RAS_ROUTING_DOMAIN_ID: ATTRIBUTEID = ATTRIBUTEID(8172u32);
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkPolicyServer\"`*"]
pub const IAS_ATTRIBUTE_CERTIFICATE_THUMBPRINT: ATTRIBUTEID = ATTRIBUTEID(8250u32);
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkPolicyServer\"`*"]
pub const RAS_ATTRIBUTE_ENCRYPTION_TYPE: ATTRIBUTEID = ATTRIBUTEID(4294967206u32);
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkPolicyServer\"`*"]
pub const RAS_ATTRIBUTE_ENCRYPTION_POLICY: ATTRIBUTEID = ATTRIBUTEID(4294967207u32);
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkPolicyServer\"`*"]
pub const RAS_ATTRIBUTE_BAP_REQUIRED: ATTRIBUTEID = ATTRIBUTEID(4294967208u32);
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkPolicyServer\"`*"]
pub const RAS_ATTRIBUTE_BAP_LINE_DOWN_TIME: ATTRIBUTEID = ATTRIBUTEID(4294967209u32);
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkPolicyServer\"`*"]
pub const RAS_ATTRIBUTE_BAP_LINE_DOWN_LIMIT: ATTRIBUTEID = ATTRIBUTEID(4294967210u32);
impl ::core::marker::Copy for ATTRIBUTEID {}
impl ::core::clone::Clone for ATTRIBUTEID {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for ATTRIBUTEID {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for ATTRIBUTEID {
    type Abi = Self;
}
impl ::core::fmt::Debug for ATTRIBUTEID {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ATTRIBUTEID").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkPolicyServer\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct ATTRIBUTEINFO(pub i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkPolicyServer\"`*"]
pub const NAME: ATTRIBUTEINFO = ATTRIBUTEINFO(1i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkPolicyServer\"`*"]
pub const SYNTAX: ATTRIBUTEINFO = ATTRIBUTEINFO(2i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkPolicyServer\"`*"]
pub const RESTRICTIONS: ATTRIBUTEINFO = ATTRIBUTEINFO(3i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkPolicyServer\"`*"]
pub const DESCRIPTION: ATTRIBUTEINFO = ATTRIBUTEINFO(4i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkPolicyServer\"`*"]
pub const VENDORID: ATTRIBUTEINFO = ATTRIBUTEINFO(5i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkPolicyServer\"`*"]
pub const LDAPNAME: ATTRIBUTEINFO = ATTRIBUTEINFO(6i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkPolicyServer\"`*"]
pub const VENDORTYPE: ATTRIBUTEINFO = ATTRIBUTEINFO(7i32);
impl ::core::marker::Copy for ATTRIBUTEINFO {}
impl ::core::clone::Clone for ATTRIBUTEINFO {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for ATTRIBUTEINFO {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for ATTRIBUTEINFO {
    type Abi = Self;
}
impl ::core::fmt::Debug for ATTRIBUTEINFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ATTRIBUTEINFO").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkPolicyServer\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct ATTRIBUTEPROPERTIES(pub i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkPolicyServer\"`*"]
pub const PROPERTY_ATTRIBUTE_ID: ATTRIBUTEPROPERTIES = ATTRIBUTEPROPERTIES(1024i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkPolicyServer\"`*"]
pub const PROPERTY_ATTRIBUTE_VENDOR_ID: ATTRIBUTEPROPERTIES = ATTRIBUTEPROPERTIES(1025i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkPolicyServer\"`*"]
pub const PROPERTY_ATTRIBUTE_VENDOR_TYPE_ID: ATTRIBUTEPROPERTIES = ATTRIBUTEPROPERTIES(1026i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkPolicyServer\"`*"]
pub const PROPERTY_ATTRIBUTE_IS_ENUMERABLE: ATTRIBUTEPROPERTIES = ATTRIBUTEPROPERTIES(1027i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkPolicyServer\"`*"]
pub const PROPERTY_ATTRIBUTE_ENUM_NAMES: ATTRIBUTEPROPERTIES = ATTRIBUTEPROPERTIES(1028i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkPolicyServer\"`*"]
pub const PROPERTY_ATTRIBUTE_ENUM_VALUES: ATTRIBUTEPROPERTIES = ATTRIBUTEPROPERTIES(1029i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkPolicyServer\"`*"]
pub const PROPERTY_ATTRIBUTE_SYNTAX: ATTRIBUTEPROPERTIES = ATTRIBUTEPROPERTIES(1030i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkPolicyServer\"`*"]
pub const PROPERTY_ATTRIBUTE_ALLOW_MULTIPLE: ATTRIBUTEPROPERTIES = ATTRIBUTEPROPERTIES(1031i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkPolicyServer\"`*"]
pub const PROPERTY_ATTRIBUTE_ALLOW_LOG_ORDINAL: ATTRIBUTEPROPERTIES = ATTRIBUTEPROPERTIES(1032i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkPolicyServer\"`*"]
pub const PROPERTY_ATTRIBUTE_ALLOW_IN_PROFILE: ATTRIBUTEPROPERTIES = ATTRIBUTEPROPERTIES(1033i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkPolicyServer\"`*"]
pub const PROPERTY_ATTRIBUTE_ALLOW_IN_CONDITION: ATTRIBUTEPROPERTIES = ATTRIBUTEPROPERTIES(1034i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkPolicyServer\"`*"]
pub const PROPERTY_ATTRIBUTE_DISPLAY_NAME: ATTRIBUTEPROPERTIES = ATTRIBUTEPROPERTIES(1035i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkPolicyServer\"`*"]
pub const PROPERTY_ATTRIBUTE_VALUE: ATTRIBUTEPROPERTIES = ATTRIBUTEPROPERTIES(1036i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkPolicyServer\"`*"]
pub const PROPERTY_ATTRIBUTE_ALLOW_IN_PROXY_PROFILE: ATTRIBUTEPROPERTIES = ATTRIBUTEPROPERTIES(1037i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkPolicyServer\"`*"]
pub const PROPERTY_ATTRIBUTE_ALLOW_IN_PROXY_CONDITION: ATTRIBUTEPROPERTIES = ATTRIBUTEPROPERTIES(1038i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkPolicyServer\"`*"]
pub const PROPERTY_ATTRIBUTE_ALLOW_IN_VPNDIALUP: ATTRIBUTEPROPERTIES = ATTRIBUTEPROPERTIES(1039i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkPolicyServer\"`*"]
pub const PROPERTY_ATTRIBUTE_ALLOW_IN_8021X: ATTRIBUTEPROPERTIES = ATTRIBUTEPROPERTIES(1040i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkPolicyServer\"`*"]
pub const PROPERTY_ATTRIBUTE_ENUM_FILTERS: ATTRIBUTEPROPERTIES = ATTRIBUTEPROPERTIES(1041i32);
impl ::core::marker::Copy for ATTRIBUTEPROPERTIES {}
impl ::core::clone::Clone for ATTRIBUTEPROPERTIES {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for ATTRIBUTEPROPERTIES {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for ATTRIBUTEPROPERTIES {
    type Abi = Self;
}
impl ::core::fmt::Debug for ATTRIBUTEPROPERTIES {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ATTRIBUTEPROPERTIES").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkPolicyServer\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct ATTRIBUTERESTRICTIONS(pub i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkPolicyServer\"`*"]
pub const MULTIVALUED: ATTRIBUTERESTRICTIONS = ATTRIBUTERESTRICTIONS(1i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkPolicyServer\"`*"]
pub const ALLOWEDINPROFILE: ATTRIBUTERESTRICTIONS = ATTRIBUTERESTRICTIONS(2i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkPolicyServer\"`*"]
pub const ALLOWEDINCONDITION: ATTRIBUTERESTRICTIONS = ATTRIBUTERESTRICTIONS(4i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkPolicyServer\"`*"]
pub const ALLOWEDINPROXYPROFILE: ATTRIBUTERESTRICTIONS = ATTRIBUTERESTRICTIONS(8i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkPolicyServer\"`*"]
pub const ALLOWEDINPROXYCONDITION: ATTRIBUTERESTRICTIONS = ATTRIBUTERESTRICTIONS(16i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkPolicyServer\"`*"]
pub const ALLOWEDINVPNDIALUP: ATTRIBUTERESTRICTIONS = ATTRIBUTERESTRICTIONS(32i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkPolicyServer\"`*"]
pub const ALLOWEDIN8021X: ATTRIBUTERESTRICTIONS = ATTRIBUTERESTRICTIONS(64i32);
impl ::core::marker::Copy for ATTRIBUTERESTRICTIONS {}
impl ::core::clone::Clone for ATTRIBUTERESTRICTIONS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for ATTRIBUTERESTRICTIONS {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for ATTRIBUTERESTRICTIONS {
    type Abi = Self;
}
impl ::core::fmt::Debug for ATTRIBUTERESTRICTIONS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ATTRIBUTERESTRICTIONS").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkPolicyServer\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct ATTRIBUTESYNTAX(pub i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkPolicyServer\"`*"]
pub const IAS_SYNTAX_BOOLEAN: ATTRIBUTESYNTAX = ATTRIBUTESYNTAX(1i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkPolicyServer\"`*"]
pub const IAS_SYNTAX_INTEGER: ATTRIBUTESYNTAX = ATTRIBUTESYNTAX(2i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkPolicyServer\"`*"]
pub const IAS_SYNTAX_ENUMERATOR: ATTRIBUTESYNTAX = ATTRIBUTESYNTAX(3i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkPolicyServer\"`*"]
pub const IAS_SYNTAX_INETADDR: ATTRIBUTESYNTAX = ATTRIBUTESYNTAX(4i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkPolicyServer\"`*"]
pub const IAS_SYNTAX_STRING: ATTRIBUTESYNTAX = ATTRIBUTESYNTAX(5i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkPolicyServer\"`*"]
pub const IAS_SYNTAX_OCTETSTRING: ATTRIBUTESYNTAX = ATTRIBUTESYNTAX(6i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkPolicyServer\"`*"]
pub const IAS_SYNTAX_UTCTIME: ATTRIBUTESYNTAX = ATTRIBUTESYNTAX(7i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkPolicyServer\"`*"]
pub const IAS_SYNTAX_PROVIDERSPECIFIC: ATTRIBUTESYNTAX = ATTRIBUTESYNTAX(8i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkPolicyServer\"`*"]
pub const IAS_SYNTAX_UNSIGNEDINTEGER: ATTRIBUTESYNTAX = ATTRIBUTESYNTAX(9i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkPolicyServer\"`*"]
pub const IAS_SYNTAX_INETADDR6: ATTRIBUTESYNTAX = ATTRIBUTESYNTAX(10i32);
impl ::core::marker::Copy for ATTRIBUTESYNTAX {}
impl ::core::clone::Clone for ATTRIBUTESYNTAX {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for ATTRIBUTESYNTAX {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for ATTRIBUTESYNTAX {
    type Abi = Self;
}
impl ::core::fmt::Debug for ATTRIBUTESYNTAX {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ATTRIBUTESYNTAX").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkPolicyServer\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct AUTHENTICATION_TYPE(pub i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkPolicyServer\"`*"]
pub const IAS_AUTH_INVALID: AUTHENTICATION_TYPE = AUTHENTICATION_TYPE(0i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkPolicyServer\"`*"]
pub const IAS_AUTH_PAP: AUTHENTICATION_TYPE = AUTHENTICATION_TYPE(1i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkPolicyServer\"`*"]
pub const IAS_AUTH_MD5CHAP: AUTHENTICATION_TYPE = AUTHENTICATION_TYPE(2i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkPolicyServer\"`*"]
pub const IAS_AUTH_MSCHAP: AUTHENTICATION_TYPE = AUTHENTICATION_TYPE(3i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkPolicyServer\"`*"]
pub const IAS_AUTH_MSCHAP2: AUTHENTICATION_TYPE = AUTHENTICATION_TYPE(4i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkPolicyServer\"`*"]
pub const IAS_AUTH_EAP: AUTHENTICATION_TYPE = AUTHENTICATION_TYPE(5i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkPolicyServer\"`*"]
pub const IAS_AUTH_ARAP: AUTHENTICATION_TYPE = AUTHENTICATION_TYPE(6i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkPolicyServer\"`*"]
pub const IAS_AUTH_NONE: AUTHENTICATION_TYPE = AUTHENTICATION_TYPE(7i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkPolicyServer\"`*"]
pub const IAS_AUTH_CUSTOM: AUTHENTICATION_TYPE = AUTHENTICATION_TYPE(8i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkPolicyServer\"`*"]
pub const IAS_AUTH_MSCHAP_CPW: AUTHENTICATION_TYPE = AUTHENTICATION_TYPE(9i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkPolicyServer\"`*"]
pub const IAS_AUTH_MSCHAP2_CPW: AUTHENTICATION_TYPE = AUTHENTICATION_TYPE(10i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkPolicyServer\"`*"]
pub const IAS_AUTH_PEAP: AUTHENTICATION_TYPE = AUTHENTICATION_TYPE(11i32);
impl ::core::marker::Copy for AUTHENTICATION_TYPE {}
impl ::core::clone::Clone for AUTHENTICATION_TYPE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for AUTHENTICATION_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for AUTHENTICATION_TYPE {
    type Abi = Self;
}
impl ::core::fmt::Debug for AUTHENTICATION_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("AUTHENTICATION_TYPE").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkPolicyServer\"`*"]
pub const AUTHSRV_AUTHORIZATION_VALUE_W: &'static str = "AuthorizationDLLs";
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkPolicyServer\"`*"]
pub const AUTHSRV_ENFORCE_NP_FOR_PAP_CHALLENGE_RESPONSE_VALUE_W: &'static str = "EnforceNetworkPolicyForPAPBasedChallengeResponse";
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkPolicyServer\"`*"]
pub const AUTHSRV_EXTENSIONS_VALUE_W: &'static str = "ExtensionDLLs";
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkPolicyServer\"`*"]
pub const AUTHSRV_PARAMETERS_KEY_W: &'static str = "System\\CurrentControlSet\\Services\\AuthSrv\\Parameters";
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkPolicyServer\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct CLIENTPROPERTIES(pub i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkPolicyServer\"`*"]
pub const PROPERTY_CLIENT_REQUIRE_SIGNATURE: CLIENTPROPERTIES = CLIENTPROPERTIES(1024i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkPolicyServer\"`*"]
pub const PROPERTY_CLIENT_UNUSED: CLIENTPROPERTIES = CLIENTPROPERTIES(1025i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkPolicyServer\"`*"]
pub const PROPERTY_CLIENT_SHARED_SECRET: CLIENTPROPERTIES = CLIENTPROPERTIES(1026i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkPolicyServer\"`*"]
pub const PROPERTY_CLIENT_NAS_MANUFACTURER: CLIENTPROPERTIES = CLIENTPROPERTIES(1027i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkPolicyServer\"`*"]
pub const PROPERTY_CLIENT_ADDRESS: CLIENTPROPERTIES = CLIENTPROPERTIES(1028i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkPolicyServer\"`*"]
pub const PROPERTY_CLIENT_QUARANTINE_COMPATIBLE: CLIENTPROPERTIES = CLIENTPROPERTIES(1029i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkPolicyServer\"`*"]
pub const PROPERTY_CLIENT_ENABLED: CLIENTPROPERTIES = CLIENTPROPERTIES(1030i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkPolicyServer\"`*"]
pub const PROPERTY_CLIENT_SECRET_TEMPLATE_GUID: CLIENTPROPERTIES = CLIENTPROPERTIES(1031i32);
impl ::core::marker::Copy for CLIENTPROPERTIES {}
impl ::core::clone::Clone for CLIENTPROPERTIES {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for CLIENTPROPERTIES {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for CLIENTPROPERTIES {
    type Abi = Self;
}
impl ::core::fmt::Debug for CLIENTPROPERTIES {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CLIENTPROPERTIES").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkPolicyServer\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct CONDITIONPROPERTIES(pub i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkPolicyServer\"`*"]
pub const PROPERTY_CONDITION_TEXT: CONDITIONPROPERTIES = CONDITIONPROPERTIES(1024i32);
impl ::core::marker::Copy for CONDITIONPROPERTIES {}
impl ::core::clone::Clone for CONDITIONPROPERTIES {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for CONDITIONPROPERTIES {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for CONDITIONPROPERTIES {
    type Abi = Self;
}
impl ::core::fmt::Debug for CONDITIONPROPERTIES {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CONDITIONPROPERTIES").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkPolicyServer\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct DICTIONARYPROPERTIES(pub i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkPolicyServer\"`*"]
pub const PROPERTY_DICTIONARY_ATTRIBUTES_COLLECTION: DICTIONARYPROPERTIES = DICTIONARYPROPERTIES(1024i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkPolicyServer\"`*"]
pub const PROPERTY_DICTIONARY_LOCATION: DICTIONARYPROPERTIES = DICTIONARYPROPERTIES(1025i32);
impl ::core::marker::Copy for DICTIONARYPROPERTIES {}
impl ::core::clone::Clone for DICTIONARYPROPERTIES {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for DICTIONARYPROPERTIES {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for DICTIONARYPROPERTIES {
    type Abi = Self;
}
impl ::core::fmt::Debug for DICTIONARYPROPERTIES {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DICTIONARYPROPERTIES").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkPolicyServer\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct IASCOMMONPROPERTIES(pub i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkPolicyServer\"`*"]
pub const PROPERTY_SDO_RESERVED: IASCOMMONPROPERTIES = IASCOMMONPROPERTIES(0i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkPolicyServer\"`*"]
pub const PROPERTY_SDO_CLASS: IASCOMMONPROPERTIES = IASCOMMONPROPERTIES(1i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkPolicyServer\"`*"]
pub const PROPERTY_SDO_NAME: IASCOMMONPROPERTIES = IASCOMMONPROPERTIES(2i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkPolicyServer\"`*"]
pub const PROPERTY_SDO_DESCRIPTION: IASCOMMONPROPERTIES = IASCOMMONPROPERTIES(3i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkPolicyServer\"`*"]
pub const PROPERTY_SDO_ID: IASCOMMONPROPERTIES = IASCOMMONPROPERTIES(4i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkPolicyServer\"`*"]
pub const PROPERTY_SDO_DATASTORE_NAME: IASCOMMONPROPERTIES = IASCOMMONPROPERTIES(5i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkPolicyServer\"`*"]
pub const PROPERTY_SDO_TEMPLATE_GUID: IASCOMMONPROPERTIES = IASCOMMONPROPERTIES(6i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkPolicyServer\"`*"]
pub const PROPERTY_SDO_OPAQUE: IASCOMMONPROPERTIES = IASCOMMONPROPERTIES(7i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkPolicyServer\"`*"]
pub const PROPERTY_SDO_START: IASCOMMONPROPERTIES = IASCOMMONPROPERTIES(1024i32);
impl ::core::marker::Copy for IASCOMMONPROPERTIES {}
impl ::core::clone::Clone for IASCOMMONPROPERTIES {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for IASCOMMONPROPERTIES {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for IASCOMMONPROPERTIES {
    type Abi = Self;
}
impl ::core::fmt::Debug for IASCOMMONPROPERTIES {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IASCOMMONPROPERTIES").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkPolicyServer\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct IASCOMPONENTPROPERTIES(pub i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkPolicyServer\"`*"]
pub const PROPERTY_COMPONENT_ID: IASCOMPONENTPROPERTIES = IASCOMPONENTPROPERTIES(1024i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkPolicyServer\"`*"]
pub const PROPERTY_COMPONENT_PROG_ID: IASCOMPONENTPROPERTIES = IASCOMPONENTPROPERTIES(1025i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkPolicyServer\"`*"]
pub const PROPERTY_COMPONENT_START: IASCOMPONENTPROPERTIES = IASCOMPONENTPROPERTIES(1026i32);
impl ::core::marker::Copy for IASCOMPONENTPROPERTIES {}
impl ::core::clone::Clone for IASCOMPONENTPROPERTIES {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for IASCOMPONENTPROPERTIES {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for IASCOMPONENTPROPERTIES {
    type Abi = Self;
}
impl ::core::fmt::Debug for IASCOMPONENTPROPERTIES {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IASCOMPONENTPROPERTIES").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkPolicyServer\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct IASDATASTORE(pub i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkPolicyServer\"`*"]
pub const DATA_STORE_LOCAL: IASDATASTORE = IASDATASTORE(0i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkPolicyServer\"`*"]
pub const DATA_STORE_DIRECTORY: IASDATASTORE = IASDATASTORE(1i32);
impl ::core::marker::Copy for IASDATASTORE {}
impl ::core::clone::Clone for IASDATASTORE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for IASDATASTORE {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for IASDATASTORE {
    type Abi = Self;
}
impl ::core::fmt::Debug for IASDATASTORE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IASDATASTORE").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkPolicyServer\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct IASDOMAINTYPE(pub i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkPolicyServer\"`*"]
pub const DOMAIN_TYPE_NONE: IASDOMAINTYPE = IASDOMAINTYPE(0i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkPolicyServer\"`*"]
pub const DOMAIN_TYPE_NT4: IASDOMAINTYPE = IASDOMAINTYPE(1i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkPolicyServer\"`*"]
pub const DOMAIN_TYPE_NT5: IASDOMAINTYPE = IASDOMAINTYPE(2i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkPolicyServer\"`*"]
pub const DOMAIN_TYPE_MIXED: IASDOMAINTYPE = IASDOMAINTYPE(3i32);
impl ::core::marker::Copy for IASDOMAINTYPE {}
impl ::core::clone::Clone for IASDOMAINTYPE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for IASDOMAINTYPE {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for IASDOMAINTYPE {
    type Abi = Self;
}
impl ::core::fmt::Debug for IASDOMAINTYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IASDOMAINTYPE").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkPolicyServer\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct IASOSTYPE(pub i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkPolicyServer\"`*"]
pub const SYSTEM_TYPE_NT4_WORKSTATION: IASOSTYPE = IASOSTYPE(0i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkPolicyServer\"`*"]
pub const SYSTEM_TYPE_NT5_WORKSTATION: IASOSTYPE = IASOSTYPE(1i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkPolicyServer\"`*"]
pub const SYSTEM_TYPE_NT6_WORKSTATION: IASOSTYPE = IASOSTYPE(2i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkPolicyServer\"`*"]
pub const SYSTEM_TYPE_NT6_1_WORKSTATION: IASOSTYPE = IASOSTYPE(3i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkPolicyServer\"`*"]
pub const SYSTEM_TYPE_NT6_2_WORKSTATION: IASOSTYPE = IASOSTYPE(4i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkPolicyServer\"`*"]
pub const SYSTEM_TYPE_NT6_3_WORKSTATION: IASOSTYPE = IASOSTYPE(5i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkPolicyServer\"`*"]
pub const SYSTEM_TYPE_NT10_0_WORKSTATION: IASOSTYPE = IASOSTYPE(6i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkPolicyServer\"`*"]
pub const SYSTEM_TYPE_NT4_SERVER: IASOSTYPE = IASOSTYPE(7i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkPolicyServer\"`*"]
pub const SYSTEM_TYPE_NT5_SERVER: IASOSTYPE = IASOSTYPE(8i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkPolicyServer\"`*"]
pub const SYSTEM_TYPE_NT6_SERVER: IASOSTYPE = IASOSTYPE(9i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkPolicyServer\"`*"]
pub const SYSTEM_TYPE_NT6_1_SERVER: IASOSTYPE = IASOSTYPE(10i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkPolicyServer\"`*"]
pub const SYSTEM_TYPE_NT6_2_SERVER: IASOSTYPE = IASOSTYPE(11i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkPolicyServer\"`*"]
pub const SYSTEM_TYPE_NT6_3_SERVER: IASOSTYPE = IASOSTYPE(12i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkPolicyServer\"`*"]
pub const SYSTEM_TYPE_NT10_0_SERVER: IASOSTYPE = IASOSTYPE(13i32);
impl ::core::marker::Copy for IASOSTYPE {}
impl ::core::clone::Clone for IASOSTYPE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for IASOSTYPE {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for IASOSTYPE {
    type Abi = Self;
}
impl ::core::fmt::Debug for IASOSTYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IASOSTYPE").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkPolicyServer\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct IASPROPERTIES(pub i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkPolicyServer\"`*"]
pub const PROPERTY_IAS_RADIUSSERVERGROUPS_COLLECTION: IASPROPERTIES = IASPROPERTIES(1024i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkPolicyServer\"`*"]
pub const PROPERTY_IAS_POLICIES_COLLECTION: IASPROPERTIES = IASPROPERTIES(1025i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkPolicyServer\"`*"]
pub const PROPERTY_IAS_PROFILES_COLLECTION: IASPROPERTIES = IASPROPERTIES(1026i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkPolicyServer\"`*"]
pub const PROPERTY_IAS_PROTOCOLS_COLLECTION: IASPROPERTIES = IASPROPERTIES(1027i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkPolicyServer\"`*"]
pub const PROPERTY_IAS_AUDITORS_COLLECTION: IASPROPERTIES = IASPROPERTIES(1028i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkPolicyServer\"`*"]
pub const PROPERTY_IAS_REQUESTHANDLERS_COLLECTION: IASPROPERTIES = IASPROPERTIES(1029i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkPolicyServer\"`*"]
pub const PROPERTY_IAS_PROXYPOLICIES_COLLECTION: IASPROPERTIES = IASPROPERTIES(1030i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkPolicyServer\"`*"]
pub const PROPERTY_IAS_PROXYPROFILES_COLLECTION: IASPROPERTIES = IASPROPERTIES(1031i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkPolicyServer\"`*"]
pub const PROPERTY_IAS_REMEDIATIONSERVERGROUPS_COLLECTION: IASPROPERTIES = IASPROPERTIES(1032i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkPolicyServer\"`*"]
pub const PROPERTY_IAS_SHVTEMPLATES_COLLECTION: IASPROPERTIES = IASPROPERTIES(1033i32);
impl ::core::marker::Copy for IASPROPERTIES {}
impl ::core::clone::Clone for IASPROPERTIES {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for IASPROPERTIES {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for IASPROPERTIES {
    type Abi = Self;
}
impl ::core::fmt::Debug for IASPROPERTIES {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IASPROPERTIES").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkPolicyServer\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct IDENTITY_TYPE(pub i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkPolicyServer\"`*"]
pub const IAS_IDENTITY_NO_DEFAULT: IDENTITY_TYPE = IDENTITY_TYPE(1i32);
impl ::core::marker::Copy for IDENTITY_TYPE {}
impl ::core::clone::Clone for IDENTITY_TYPE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for IDENTITY_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for IDENTITY_TYPE {
    type Abi = Self;
}
impl ::core::fmt::Debug for IDENTITY_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IDENTITY_TYPE").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkPolicyServer\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct IPFILTERPROPERTIES(pub i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkPolicyServer\"`*"]
pub const PROPERTY_IPFILTER_ATTRIBUTES_COLLECTION: IPFILTERPROPERTIES = IPFILTERPROPERTIES(1024i32);
impl ::core::marker::Copy for IPFILTERPROPERTIES {}
impl ::core::clone::Clone for IPFILTERPROPERTIES {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for IPFILTERPROPERTIES {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for IPFILTERPROPERTIES {
    type Abi = Self;
}
impl ::core::fmt::Debug for IPFILTERPROPERTIES {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IPFILTERPROPERTIES").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkPolicyServer\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
pub struct ISdo(::windows::core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl ISdo {
    #[doc = "*Required features: `\"Win32_NetworkManagement_NetworkPolicyServer\"`*"]
    pub unsafe fn GetPropertyInfo(&self, id: i32) -> ::windows::core::Result<::windows::core::IUnknown> {
        let mut result__: *mut ::core::ffi::c_void = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).GetPropertyInfo)(::core::mem::transmute_copy(self), ::core::mem::transmute(id), ::core::mem::transmute(&mut result__)).from_abi::<::windows::core::IUnknown>(result__)
    }
    #[doc = "*Required features: `\"Win32_NetworkManagement_NetworkPolicyServer\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn GetProperty(&self, id: i32) -> ::windows::core::Result<super::super::System::Com::VARIANT> {
        let mut result__: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT> = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).GetProperty)(::core::mem::transmute_copy(self), ::core::mem::transmute(id), ::core::mem::transmute(&mut result__)).from_abi::<super::super::System::Com::VARIANT>(result__)
    }
    #[doc = "*Required features: `\"Win32_NetworkManagement_NetworkPolicyServer\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn PutProperty(&self, id: i32, pvalue: *const super::super::System::Com::VARIANT) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).PutProperty)(::core::mem::transmute_copy(self), ::core::mem::transmute(id), ::core::mem::transmute(pvalue)).ok()
    }
    #[doc = "*Required features: `\"Win32_NetworkManagement_NetworkPolicyServer\"`*"]
    pub unsafe fn ResetProperty(&self, id: i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).ResetProperty)(::core::mem::transmute_copy(self), ::core::mem::transmute(id)).ok()
    }
    #[doc = "*Required features: `\"Win32_NetworkManagement_NetworkPolicyServer\"`*"]
    pub unsafe fn Apply(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Apply)(::core::mem::transmute_copy(self)).ok()
    }
    #[doc = "*Required features: `\"Win32_NetworkManagement_NetworkPolicyServer\"`*"]
    pub unsafe fn Restore(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Restore)(::core::mem::transmute_copy(self)).ok()
    }
    #[doc = "*Required features: `\"Win32_NetworkManagement_NetworkPolicyServer\"`*"]
    pub unsafe fn _NewEnum(&self) -> ::windows::core::Result<::windows::core::IUnknown> {
        let mut result__: *mut ::core::ffi::c_void = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self)._NewEnum)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<::windows::core::IUnknown>(result__)
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<ISdo> for ::windows::core::IUnknown {
    fn from(value: ISdo) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&ISdo> for ::windows::core::IUnknown {
    fn from(value: &ISdo) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for ISdo {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a ISdo {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<ISdo> for super::super::System::Com::IDispatch {
    fn from(value: ISdo) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&ISdo> for super::super::System::Com::IDispatch {
    fn from(value: &ISdo) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, super::super::System::Com::IDispatch> for ISdo {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::System::Com::IDispatch> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, super::super::System::Com::IDispatch> for &'a ISdo {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::System::Com::IDispatch> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for ISdo {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for ISdo {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for ISdo {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for ISdo {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ISdo").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Interface for ISdo {
    type Vtable = ISdo_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x56bc53de_96db_11d1_bf3f_000000000000);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct ISdo_Vtbl {
    pub base: super::super::System::Com::IDispatch_Vtbl,
    pub GetPropertyInfo: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, id: i32, pppropertyinfo: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub GetProperty: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, id: i32, pvalue: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    GetProperty: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub PutProperty: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, id: i32, pvalue: *const super::super::System::Com::VARIANT) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    PutProperty: usize,
    pub ResetProperty: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, id: i32) -> ::windows::core::HRESULT,
    pub Apply: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub Restore: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub _NewEnum: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppenumvariant: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkPolicyServer\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
pub struct ISdoCollection(::windows::core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl ISdoCollection {
    #[doc = "*Required features: `\"Win32_NetworkManagement_NetworkPolicyServer\"`*"]
    pub unsafe fn Count(&self) -> ::windows::core::Result<i32> {
        let mut result__: i32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).Count)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<i32>(result__)
    }
    #[doc = "*Required features: `\"Win32_NetworkManagement_NetworkPolicyServer\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub unsafe fn Add<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, bstrname: Param0, ppitem: *mut ::core::option::Option<super::super::System::Com::IDispatch>) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Add)(::core::mem::transmute_copy(self), bstrname.into_param().abi(), ::core::mem::transmute(ppitem)).ok()
    }
    #[doc = "*Required features: `\"Win32_NetworkManagement_NetworkPolicyServer\"`, `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn Remove<'a, Param0: ::windows::core::IntoParam<'a, super::super::System::Com::IDispatch>>(&self, pitem: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Remove)(::core::mem::transmute_copy(self), pitem.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_NetworkManagement_NetworkPolicyServer\"`*"]
    pub unsafe fn RemoveAll(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).RemoveAll)(::core::mem::transmute_copy(self)).ok()
    }
    #[doc = "*Required features: `\"Win32_NetworkManagement_NetworkPolicyServer\"`*"]
    pub unsafe fn Reload(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Reload)(::core::mem::transmute_copy(self)).ok()
    }
    #[doc = "*Required features: `\"Win32_NetworkManagement_NetworkPolicyServer\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn IsNameUnique<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, bstrname: Param0) -> ::windows::core::Result<i16> {
        let mut result__: i16 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).IsNameUnique)(::core::mem::transmute_copy(self), bstrname.into_param().abi(), ::core::mem::transmute(&mut result__)).from_abi::<i16>(result__)
    }
    #[doc = "*Required features: `\"Win32_NetworkManagement_NetworkPolicyServer\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn Item(&self, name: *const super::super::System::Com::VARIANT) -> ::windows::core::Result<super::super::System::Com::IDispatch> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).Item)(::core::mem::transmute_copy(self), ::core::mem::transmute(name), ::core::mem::transmute(&mut result__)).from_abi::<super::super::System::Com::IDispatch>(result__)
    }
    #[doc = "*Required features: `\"Win32_NetworkManagement_NetworkPolicyServer\"`*"]
    pub unsafe fn _NewEnum(&self) -> ::windows::core::Result<::windows::core::IUnknown> {
        let mut result__: *mut ::core::ffi::c_void = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self)._NewEnum)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<::windows::core::IUnknown>(result__)
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<ISdoCollection> for ::windows::core::IUnknown {
    fn from(value: ISdoCollection) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&ISdoCollection> for ::windows::core::IUnknown {
    fn from(value: &ISdoCollection) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for ISdoCollection {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a ISdoCollection {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<ISdoCollection> for super::super::System::Com::IDispatch {
    fn from(value: ISdoCollection) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&ISdoCollection> for super::super::System::Com::IDispatch {
    fn from(value: &ISdoCollection) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, super::super::System::Com::IDispatch> for ISdoCollection {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::System::Com::IDispatch> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, super::super::System::Com::IDispatch> for &'a ISdoCollection {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::System::Com::IDispatch> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for ISdoCollection {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for ISdoCollection {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for ISdoCollection {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for ISdoCollection {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ISdoCollection").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Interface for ISdoCollection {
    type Vtable = ISdoCollection_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x56bc53e2_96db_11d1_bf3f_000000000000);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct ISdoCollection_Vtbl {
    pub base: super::super::System::Com::IDispatch_Vtbl,
    pub Count: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pcount: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub Add: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, ppitem: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com")))]
    Add: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub Remove: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pitem: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    Remove: usize,
    pub RemoveAll: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub Reload: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub IsNameUnique: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, pbool: *mut i16) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    IsNameUnique: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub Item: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, name: *const super::super::System::Com::VARIANT, pitem: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    Item: usize,
    pub _NewEnum: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppenumvariant: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkPolicyServer\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
pub struct ISdoDictionaryOld(::windows::core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl ISdoDictionaryOld {
    #[doc = "*Required features: `\"Win32_NetworkManagement_NetworkPolicyServer\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn EnumAttributes(&self, id: *mut super::super::System::Com::VARIANT, pvalues: *mut super::super::System::Com::VARIANT) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).EnumAttributes)(::core::mem::transmute_copy(self), ::core::mem::transmute(id), ::core::mem::transmute(pvalues)).ok()
    }
    #[doc = "*Required features: `\"Win32_NetworkManagement_NetworkPolicyServer\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn GetAttributeInfo(&self, id: ATTRIBUTEID, pinfoids: *const super::super::System::Com::VARIANT) -> ::windows::core::Result<super::super::System::Com::VARIANT> {
        let mut result__: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT> = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).GetAttributeInfo)(::core::mem::transmute_copy(self), ::core::mem::transmute(id), ::core::mem::transmute(pinfoids), ::core::mem::transmute(&mut result__)).from_abi::<super::super::System::Com::VARIANT>(result__)
    }
    #[doc = "*Required features: `\"Win32_NetworkManagement_NetworkPolicyServer\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn EnumAttributeValues(&self, id: ATTRIBUTEID, pvalueids: *mut super::super::System::Com::VARIANT, pvaluesdesc: *mut super::super::System::Com::VARIANT) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).EnumAttributeValues)(::core::mem::transmute_copy(self), ::core::mem::transmute(id), ::core::mem::transmute(pvalueids), ::core::mem::transmute(pvaluesdesc)).ok()
    }
    #[doc = "*Required features: `\"Win32_NetworkManagement_NetworkPolicyServer\"`, `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn CreateAttribute(&self, id: ATTRIBUTEID) -> ::windows::core::Result<super::super::System::Com::IDispatch> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).CreateAttribute)(::core::mem::transmute_copy(self), ::core::mem::transmute(id), ::core::mem::transmute(&mut result__)).from_abi::<super::super::System::Com::IDispatch>(result__)
    }
    #[doc = "*Required features: `\"Win32_NetworkManagement_NetworkPolicyServer\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetAttributeID<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, bstrattributename: Param0) -> ::windows::core::Result<ATTRIBUTEID> {
        let mut result__: ATTRIBUTEID = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).GetAttributeID)(::core::mem::transmute_copy(self), bstrattributename.into_param().abi(), ::core::mem::transmute(&mut result__)).from_abi::<ATTRIBUTEID>(result__)
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<ISdoDictionaryOld> for ::windows::core::IUnknown {
    fn from(value: ISdoDictionaryOld) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&ISdoDictionaryOld> for ::windows::core::IUnknown {
    fn from(value: &ISdoDictionaryOld) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for ISdoDictionaryOld {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a ISdoDictionaryOld {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<ISdoDictionaryOld> for super::super::System::Com::IDispatch {
    fn from(value: ISdoDictionaryOld) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&ISdoDictionaryOld> for super::super::System::Com::IDispatch {
    fn from(value: &ISdoDictionaryOld) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, super::super::System::Com::IDispatch> for ISdoDictionaryOld {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::System::Com::IDispatch> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, super::super::System::Com::IDispatch> for &'a ISdoDictionaryOld {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::System::Com::IDispatch> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for ISdoDictionaryOld {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for ISdoDictionaryOld {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for ISdoDictionaryOld {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for ISdoDictionaryOld {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ISdoDictionaryOld").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Interface for ISdoDictionaryOld {
    type Vtable = ISdoDictionaryOld_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xd432e5f4_53d8_11d2_9a3a_00c04fb998ac);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct ISdoDictionaryOld_Vtbl {
    pub base: super::super::System::Com::IDispatch_Vtbl,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub EnumAttributes: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, id: *mut super::super::System::Com::VARIANT, pvalues: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    EnumAttributes: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub GetAttributeInfo: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, id: ATTRIBUTEID, pinfoids: *const super::super::System::Com::VARIANT, pinfovalues: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    GetAttributeInfo: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub EnumAttributeValues: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, id: ATTRIBUTEID, pvalueids: *mut super::super::System::Com::VARIANT, pvaluesdesc: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole")))]
    EnumAttributeValues: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub CreateAttribute: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, id: ATTRIBUTEID, ppattributeobject: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    CreateAttribute: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub GetAttributeID: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrattributename: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, pid: *mut ATTRIBUTEID) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetAttributeID: usize,
}
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkPolicyServer\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
pub struct ISdoMachine(::windows::core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl ISdoMachine {
    #[doc = "*Required features: `\"Win32_NetworkManagement_NetworkPolicyServer\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Attach<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, bstrcomputername: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Attach)(::core::mem::transmute_copy(self), bstrcomputername.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_NetworkManagement_NetworkPolicyServer\"`*"]
    pub unsafe fn GetDictionarySDO(&self) -> ::windows::core::Result<::windows::core::IUnknown> {
        let mut result__: *mut ::core::ffi::c_void = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).GetDictionarySDO)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<::windows::core::IUnknown>(result__)
    }
    #[doc = "*Required features: `\"Win32_NetworkManagement_NetworkPolicyServer\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetServiceSDO<'a, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, edatastore: IASDATASTORE, bstrservicename: Param1) -> ::windows::core::Result<::windows::core::IUnknown> {
        let mut result__: *mut ::core::ffi::c_void = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).GetServiceSDO)(::core::mem::transmute_copy(self), ::core::mem::transmute(edatastore), bstrservicename.into_param().abi(), ::core::mem::transmute(&mut result__)).from_abi::<::windows::core::IUnknown>(result__)
    }
    #[doc = "*Required features: `\"Win32_NetworkManagement_NetworkPolicyServer\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetUserSDO<'a, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, edatastore: IASDATASTORE, bstrusername: Param1) -> ::windows::core::Result<::windows::core::IUnknown> {
        let mut result__: *mut ::core::ffi::c_void = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).GetUserSDO)(::core::mem::transmute_copy(self), ::core::mem::transmute(edatastore), bstrusername.into_param().abi(), ::core::mem::transmute(&mut result__)).from_abi::<::windows::core::IUnknown>(result__)
    }
    #[doc = "*Required features: `\"Win32_NetworkManagement_NetworkPolicyServer\"`*"]
    pub unsafe fn GetOSType(&self) -> ::windows::core::Result<IASOSTYPE> {
        let mut result__: IASOSTYPE = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).GetOSType)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<IASOSTYPE>(result__)
    }
    #[doc = "*Required features: `\"Win32_NetworkManagement_NetworkPolicyServer\"`*"]
    pub unsafe fn GetDomainType(&self) -> ::windows::core::Result<IASDOMAINTYPE> {
        let mut result__: IASDOMAINTYPE = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).GetDomainType)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<IASDOMAINTYPE>(result__)
    }
    #[doc = "*Required features: `\"Win32_NetworkManagement_NetworkPolicyServer\"`*"]
    pub unsafe fn IsDirectoryAvailable(&self) -> ::windows::core::Result<i16> {
        let mut result__: i16 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).IsDirectoryAvailable)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<i16>(result__)
    }
    #[doc = "*Required features: `\"Win32_NetworkManagement_NetworkPolicyServer\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetAttachedComputer(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR> = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).GetAttachedComputer)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[doc = "*Required features: `\"Win32_NetworkManagement_NetworkPolicyServer\"`*"]
    pub unsafe fn GetSDOSchema(&self) -> ::windows::core::Result<::windows::core::IUnknown> {
        let mut result__: *mut ::core::ffi::c_void = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).GetSDOSchema)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<::windows::core::IUnknown>(result__)
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<ISdoMachine> for ::windows::core::IUnknown {
    fn from(value: ISdoMachine) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&ISdoMachine> for ::windows::core::IUnknown {
    fn from(value: &ISdoMachine) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for ISdoMachine {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a ISdoMachine {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<ISdoMachine> for super::super::System::Com::IDispatch {
    fn from(value: ISdoMachine) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&ISdoMachine> for super::super::System::Com::IDispatch {
    fn from(value: &ISdoMachine) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, super::super::System::Com::IDispatch> for ISdoMachine {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::System::Com::IDispatch> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, super::super::System::Com::IDispatch> for &'a ISdoMachine {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::System::Com::IDispatch> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for ISdoMachine {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for ISdoMachine {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for ISdoMachine {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for ISdoMachine {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ISdoMachine").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Interface for ISdoMachine {
    type Vtable = ISdoMachine_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x479f6e75_49a2_11d2_8eca_00c04fc2f519);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct ISdoMachine_Vtbl {
    pub base: super::super::System::Com::IDispatch_Vtbl,
    #[cfg(feature = "Win32_Foundation")]
    pub Attach: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrcomputername: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    Attach: usize,
    pub GetDictionarySDO: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppdictionarysdo: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub GetServiceSDO: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, edatastore: IASDATASTORE, bstrservicename: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, ppservicesdo: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetServiceSDO: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub GetUserSDO: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, edatastore: IASDATASTORE, bstrusername: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, ppusersdo: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetUserSDO: usize,
    pub GetOSType: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, eostype: *mut IASOSTYPE) -> ::windows::core::HRESULT,
    pub GetDomainType: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, edomaintype: *mut IASDOMAINTYPE) -> ::windows::core::HRESULT,
    pub IsDirectoryAvailable: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, booldirectoryavailable: *mut i16) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub GetAttachedComputer: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrcomputername: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetAttachedComputer: usize,
    pub GetSDOSchema: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppsdoschema: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkPolicyServer\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
pub struct ISdoMachine2(::windows::core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl ISdoMachine2 {
    #[doc = "*Required features: `\"Win32_NetworkManagement_NetworkPolicyServer\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Attach<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, bstrcomputername: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.Attach)(::core::mem::transmute_copy(self), bstrcomputername.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_NetworkManagement_NetworkPolicyServer\"`*"]
    pub unsafe fn GetDictionarySDO(&self) -> ::windows::core::Result<::windows::core::IUnknown> {
        let mut result__: *mut ::core::ffi::c_void = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.GetDictionarySDO)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<::windows::core::IUnknown>(result__)
    }
    #[doc = "*Required features: `\"Win32_NetworkManagement_NetworkPolicyServer\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetServiceSDO<'a, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, edatastore: IASDATASTORE, bstrservicename: Param1) -> ::windows::core::Result<::windows::core::IUnknown> {
        let mut result__: *mut ::core::ffi::c_void = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.GetServiceSDO)(::core::mem::transmute_copy(self), ::core::mem::transmute(edatastore), bstrservicename.into_param().abi(), ::core::mem::transmute(&mut result__)).from_abi::<::windows::core::IUnknown>(result__)
    }
    #[doc = "*Required features: `\"Win32_NetworkManagement_NetworkPolicyServer\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetUserSDO<'a, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, edatastore: IASDATASTORE, bstrusername: Param1) -> ::windows::core::Result<::windows::core::IUnknown> {
        let mut result__: *mut ::core::ffi::c_void = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.GetUserSDO)(::core::mem::transmute_copy(self), ::core::mem::transmute(edatastore), bstrusername.into_param().abi(), ::core::mem::transmute(&mut result__)).from_abi::<::windows::core::IUnknown>(result__)
    }
    #[doc = "*Required features: `\"Win32_NetworkManagement_NetworkPolicyServer\"`*"]
    pub unsafe fn GetOSType(&self) -> ::windows::core::Result<IASOSTYPE> {
        let mut result__: IASOSTYPE = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.GetOSType)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<IASOSTYPE>(result__)
    }
    #[doc = "*Required features: `\"Win32_NetworkManagement_NetworkPolicyServer\"`*"]
    pub unsafe fn GetDomainType(&self) -> ::windows::core::Result<IASDOMAINTYPE> {
        let mut result__: IASDOMAINTYPE = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.GetDomainType)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<IASDOMAINTYPE>(result__)
    }
    #[doc = "*Required features: `\"Win32_NetworkManagement_NetworkPolicyServer\"`*"]
    pub unsafe fn IsDirectoryAvailable(&self) -> ::windows::core::Result<i16> {
        let mut result__: i16 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.IsDirectoryAvailable)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<i16>(result__)
    }
    #[doc = "*Required features: `\"Win32_NetworkManagement_NetworkPolicyServer\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetAttachedComputer(&self) -> ::windows::core::Result<super::super::Foundation::BSTR> {
        let mut result__: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR> = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.GetAttachedComputer)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::BSTR>(result__)
    }
    #[doc = "*Required features: `\"Win32_NetworkManagement_NetworkPolicyServer\"`*"]
    pub unsafe fn GetSDOSchema(&self) -> ::windows::core::Result<::windows::core::IUnknown> {
        let mut result__: *mut ::core::ffi::c_void = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.GetSDOSchema)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<::windows::core::IUnknown>(result__)
    }
    #[doc = "*Required features: `\"Win32_NetworkManagement_NetworkPolicyServer\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetTemplatesSDO<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, bstrservicename: Param0) -> ::windows::core::Result<::windows::core::IUnknown> {
        let mut result__: *mut ::core::ffi::c_void = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).GetTemplatesSDO)(::core::mem::transmute_copy(self), bstrservicename.into_param().abi(), ::core::mem::transmute(&mut result__)).from_abi::<::windows::core::IUnknown>(result__)
    }
    #[doc = "*Required features: `\"Win32_NetworkManagement_NetworkPolicyServer\"`*"]
    pub unsafe fn EnableTemplates(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).EnableTemplates)(::core::mem::transmute_copy(self)).ok()
    }
    #[doc = "*Required features: `\"Win32_NetworkManagement_NetworkPolicyServer\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SyncConfigAgainstTemplates<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, bstrservicename: Param0, ppconfigroot: *mut ::core::option::Option<::windows::core::IUnknown>, pptemplatesroot: *mut ::core::option::Option<::windows::core::IUnknown>, bforcedsync: i16) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SyncConfigAgainstTemplates)(::core::mem::transmute_copy(self), bstrservicename.into_param().abi(), ::core::mem::transmute(ppconfigroot), ::core::mem::transmute(pptemplatesroot), ::core::mem::transmute(bforcedsync)).ok()
    }
    #[doc = "*Required features: `\"Win32_NetworkManagement_NetworkPolicyServer\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn ImportRemoteTemplates<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::IUnknown>, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>>(&self, plocaltemplatesroot: Param0, bstrremotemachinename: Param1) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).ImportRemoteTemplates)(::core::mem::transmute_copy(self), plocaltemplatesroot.into_param().abi(), bstrremotemachinename.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_NetworkManagement_NetworkPolicyServer\"`*"]
    pub unsafe fn Reload(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Reload)(::core::mem::transmute_copy(self)).ok()
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<ISdoMachine2> for ::windows::core::IUnknown {
    fn from(value: ISdoMachine2) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&ISdoMachine2> for ::windows::core::IUnknown {
    fn from(value: &ISdoMachine2) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for ISdoMachine2 {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a ISdoMachine2 {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<ISdoMachine2> for super::super::System::Com::IDispatch {
    fn from(value: ISdoMachine2) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&ISdoMachine2> for super::super::System::Com::IDispatch {
    fn from(value: &ISdoMachine2) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, super::super::System::Com::IDispatch> for ISdoMachine2 {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::System::Com::IDispatch> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, super::super::System::Com::IDispatch> for &'a ISdoMachine2 {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::System::Com::IDispatch> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<ISdoMachine2> for ISdoMachine {
    fn from(value: ISdoMachine2) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&ISdoMachine2> for ISdoMachine {
    fn from(value: &ISdoMachine2) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, ISdoMachine> for ISdoMachine2 {
    fn into_param(self) -> ::windows::core::Param<'a, ISdoMachine> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, ISdoMachine> for &'a ISdoMachine2 {
    fn into_param(self) -> ::windows::core::Param<'a, ISdoMachine> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for ISdoMachine2 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for ISdoMachine2 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for ISdoMachine2 {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for ISdoMachine2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ISdoMachine2").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Interface for ISdoMachine2 {
    type Vtable = ISdoMachine2_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x518e5ffe_d8ce_4f7e_a5db_b40a35419d3b);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct ISdoMachine2_Vtbl {
    pub base: ISdoMachine_Vtbl,
    #[cfg(feature = "Win32_Foundation")]
    pub GetTemplatesSDO: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrservicename: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, pptemplatessdo: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetTemplatesSDO: usize,
    pub EnableTemplates: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub SyncConfigAgainstTemplates: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrservicename: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, ppconfigroot: *mut *mut ::core::ffi::c_void, pptemplatesroot: *mut *mut ::core::ffi::c_void, bforcedsync: i16) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SyncConfigAgainstTemplates: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub ImportRemoteTemplates: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, plocaltemplatesroot: *mut ::core::ffi::c_void, bstrremotemachinename: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    ImportRemoteTemplates: usize,
    pub Reload: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkPolicyServer\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
pub struct ISdoServiceControl(::windows::core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl ISdoServiceControl {
    #[doc = "*Required features: `\"Win32_NetworkManagement_NetworkPolicyServer\"`*"]
    pub unsafe fn StartService(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).StartService)(::core::mem::transmute_copy(self)).ok()
    }
    #[doc = "*Required features: `\"Win32_NetworkManagement_NetworkPolicyServer\"`*"]
    pub unsafe fn StopService(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).StopService)(::core::mem::transmute_copy(self)).ok()
    }
    #[doc = "*Required features: `\"Win32_NetworkManagement_NetworkPolicyServer\"`*"]
    pub unsafe fn GetServiceStatus(&self) -> ::windows::core::Result<i32> {
        let mut result__: i32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).GetServiceStatus)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<i32>(result__)
    }
    #[doc = "*Required features: `\"Win32_NetworkManagement_NetworkPolicyServer\"`*"]
    pub unsafe fn ResetService(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).ResetService)(::core::mem::transmute_copy(self)).ok()
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<ISdoServiceControl> for ::windows::core::IUnknown {
    fn from(value: ISdoServiceControl) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&ISdoServiceControl> for ::windows::core::IUnknown {
    fn from(value: &ISdoServiceControl) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for ISdoServiceControl {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a ISdoServiceControl {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<ISdoServiceControl> for super::super::System::Com::IDispatch {
    fn from(value: ISdoServiceControl) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&ISdoServiceControl> for super::super::System::Com::IDispatch {
    fn from(value: &ISdoServiceControl) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, super::super::System::Com::IDispatch> for ISdoServiceControl {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::System::Com::IDispatch> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, super::super::System::Com::IDispatch> for &'a ISdoServiceControl {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::System::Com::IDispatch> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for ISdoServiceControl {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for ISdoServiceControl {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for ISdoServiceControl {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for ISdoServiceControl {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ISdoServiceControl").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Interface for ISdoServiceControl {
    type Vtable = ISdoServiceControl_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x479f6e74_49a2_11d2_8eca_00c04fc2f519);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct ISdoServiceControl_Vtbl {
    pub base: super::super::System::Com::IDispatch_Vtbl,
    pub StartService: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub StopService: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub GetServiceStatus: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, status: *mut i32) -> ::windows::core::HRESULT,
    pub ResetService: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkPolicyServer\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[repr(transparent)]
pub struct ITemplateSdo(::windows::core::IUnknown);
#[cfg(feature = "Win32_System_Com")]
impl ITemplateSdo {
    #[doc = "*Required features: `\"Win32_NetworkManagement_NetworkPolicyServer\"`*"]
    pub unsafe fn GetPropertyInfo(&self, id: i32) -> ::windows::core::Result<::windows::core::IUnknown> {
        let mut result__: *mut ::core::ffi::c_void = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.GetPropertyInfo)(::core::mem::transmute_copy(self), ::core::mem::transmute(id), ::core::mem::transmute(&mut result__)).from_abi::<::windows::core::IUnknown>(result__)
    }
    #[doc = "*Required features: `\"Win32_NetworkManagement_NetworkPolicyServer\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn GetProperty(&self, id: i32) -> ::windows::core::Result<super::super::System::Com::VARIANT> {
        let mut result__: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT> = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.GetProperty)(::core::mem::transmute_copy(self), ::core::mem::transmute(id), ::core::mem::transmute(&mut result__)).from_abi::<super::super::System::Com::VARIANT>(result__)
    }
    #[doc = "*Required features: `\"Win32_NetworkManagement_NetworkPolicyServer\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub unsafe fn PutProperty(&self, id: i32, pvalue: *const super::super::System::Com::VARIANT) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.PutProperty)(::core::mem::transmute_copy(self), ::core::mem::transmute(id), ::core::mem::transmute(pvalue)).ok()
    }
    #[doc = "*Required features: `\"Win32_NetworkManagement_NetworkPolicyServer\"`*"]
    pub unsafe fn ResetProperty(&self, id: i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.ResetProperty)(::core::mem::transmute_copy(self), ::core::mem::transmute(id)).ok()
    }
    #[doc = "*Required features: `\"Win32_NetworkManagement_NetworkPolicyServer\"`*"]
    pub unsafe fn Apply(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.Apply)(::core::mem::transmute_copy(self)).ok()
    }
    #[doc = "*Required features: `\"Win32_NetworkManagement_NetworkPolicyServer\"`*"]
    pub unsafe fn Restore(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.Restore)(::core::mem::transmute_copy(self)).ok()
    }
    #[doc = "*Required features: `\"Win32_NetworkManagement_NetworkPolicyServer\"`*"]
    pub unsafe fn _NewEnum(&self) -> ::windows::core::Result<::windows::core::IUnknown> {
        let mut result__: *mut ::core::ffi::c_void = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base._NewEnum)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<::windows::core::IUnknown>(result__)
    }
    #[doc = "*Required features: `\"Win32_NetworkManagement_NetworkPolicyServer\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub unsafe fn AddToCollection<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>, Param1: ::windows::core::IntoParam<'a, super::super::System::Com::IDispatch>>(&self, bstrname: Param0, pcollection: Param1, ppitem: *mut ::core::option::Option<super::super::System::Com::IDispatch>) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).AddToCollection)(::core::mem::transmute_copy(self), bstrname.into_param().abi(), pcollection.into_param().abi(), ::core::mem::transmute(ppitem)).ok()
    }
    #[doc = "*Required features: `\"Win32_NetworkManagement_NetworkPolicyServer\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub unsafe fn AddToSdo<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::BSTR>, Param1: ::windows::core::IntoParam<'a, super::super::System::Com::IDispatch>>(&self, bstrname: Param0, psdotarget: Param1, ppitem: *mut ::core::option::Option<super::super::System::Com::IDispatch>) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).AddToSdo)(::core::mem::transmute_copy(self), bstrname.into_param().abi(), psdotarget.into_param().abi(), ::core::mem::transmute(ppitem)).ok()
    }
    #[doc = "*Required features: `\"Win32_NetworkManagement_NetworkPolicyServer\"`, `\"Win32_System_Com\"`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn AddToSdoAsProperty<'a, Param0: ::windows::core::IntoParam<'a, super::super::System::Com::IDispatch>>(&self, psdotarget: Param0, id: i32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).AddToSdoAsProperty)(::core::mem::transmute_copy(self), psdotarget.into_param().abi(), ::core::mem::transmute(id)).ok()
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<ITemplateSdo> for ::windows::core::IUnknown {
    fn from(value: ITemplateSdo) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&ITemplateSdo> for ::windows::core::IUnknown {
    fn from(value: &ITemplateSdo) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for ITemplateSdo {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a ITemplateSdo {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<ITemplateSdo> for super::super::System::Com::IDispatch {
    fn from(value: ITemplateSdo) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&ITemplateSdo> for super::super::System::Com::IDispatch {
    fn from(value: &ITemplateSdo) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, super::super::System::Com::IDispatch> for ITemplateSdo {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::System::Com::IDispatch> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, super::super::System::Com::IDispatch> for &'a ITemplateSdo {
    fn into_param(self) -> ::windows::core::Param<'a, super::super::System::Com::IDispatch> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<ITemplateSdo> for ISdo {
    fn from(value: ITemplateSdo) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::convert::From<&ITemplateSdo> for ISdo {
    fn from(value: &ITemplateSdo) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, ISdo> for ITemplateSdo {
    fn into_param(self) -> ::windows::core::Param<'a, ISdo> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl<'a> ::windows::core::IntoParam<'a, ISdo> for &'a ITemplateSdo {
    fn into_param(self) -> ::windows::core::Param<'a, ISdo> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for ITemplateSdo {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for ITemplateSdo {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for ITemplateSdo {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for ITemplateSdo {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ITemplateSdo").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Interface for ITemplateSdo {
    type Vtable = ITemplateSdo_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x8aa85302_d2e2_4e20_8b1f_a571e437d6c9);
}
#[cfg(feature = "Win32_System_Com")]
#[repr(C)]
#[doc(hidden)]
pub struct ITemplateSdo_Vtbl {
    pub base: ISdo_Vtbl,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub AddToCollection: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, pcollection: ::windows::core::RawPtr, ppitem: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com")))]
    AddToCollection: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub AddToSdo: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, psdotarget: ::windows::core::RawPtr, ppitem: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com")))]
    AddToSdo: usize,
    #[cfg(feature = "Win32_System_Com")]
    pub AddToSdoAsProperty: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, psdotarget: ::windows::core::RawPtr, id: i32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    AddToSdoAsProperty: usize,
}
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkPolicyServer\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct NAMESPROPERTIES(pub i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkPolicyServer\"`*"]
pub const PROPERTY_NAMES_REALMS: NAMESPROPERTIES = NAMESPROPERTIES(1026i32);
impl ::core::marker::Copy for NAMESPROPERTIES {}
impl ::core::clone::Clone for NAMESPROPERTIES {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for NAMESPROPERTIES {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for NAMESPROPERTIES {
    type Abi = Self;
}
impl ::core::fmt::Debug for NAMESPROPERTIES {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("NAMESPROPERTIES").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkPolicyServer\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct NAPPROPERTIES(pub i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkPolicyServer\"`*"]
pub const PROPERTY_NAP_POLICIES_COLLECTION: NAPPROPERTIES = NAPPROPERTIES(1026i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkPolicyServer\"`*"]
pub const PROPERTY_SHV_TEMPLATES_COLLECTION: NAPPROPERTIES = NAPPROPERTIES(1027i32);
impl ::core::marker::Copy for NAPPROPERTIES {}
impl ::core::clone::Clone for NAPPROPERTIES {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for NAPPROPERTIES {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for NAPPROPERTIES {
    type Abi = Self;
}
impl ::core::fmt::Debug for NAPPROPERTIES {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("NAPPROPERTIES").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkPolicyServer\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct NEW_LOG_FILE_FREQUENCY(pub i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkPolicyServer\"`*"]
pub const IAS_LOGGING_UNLIMITED_SIZE: NEW_LOG_FILE_FREQUENCY = NEW_LOG_FILE_FREQUENCY(0i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkPolicyServer\"`*"]
pub const IAS_LOGGING_DAILY: NEW_LOG_FILE_FREQUENCY = NEW_LOG_FILE_FREQUENCY(1i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkPolicyServer\"`*"]
pub const IAS_LOGGING_WEEKLY: NEW_LOG_FILE_FREQUENCY = NEW_LOG_FILE_FREQUENCY(2i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkPolicyServer\"`*"]
pub const IAS_LOGGING_MONTHLY: NEW_LOG_FILE_FREQUENCY = NEW_LOG_FILE_FREQUENCY(3i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkPolicyServer\"`*"]
pub const IAS_LOGGING_WHEN_FILE_SIZE_REACHES: NEW_LOG_FILE_FREQUENCY = NEW_LOG_FILE_FREQUENCY(4i32);
impl ::core::marker::Copy for NEW_LOG_FILE_FREQUENCY {}
impl ::core::clone::Clone for NEW_LOG_FILE_FREQUENCY {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for NEW_LOG_FILE_FREQUENCY {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for NEW_LOG_FILE_FREQUENCY {
    type Abi = Self;
}
impl ::core::fmt::Debug for NEW_LOG_FILE_FREQUENCY {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("NEW_LOG_FILE_FREQUENCY").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkPolicyServer\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct NTEVENTLOGPROPERTIES(pub i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkPolicyServer\"`*"]
pub const PROPERTY_EVENTLOG_LOG_APPLICATION_EVENTS: NTEVENTLOGPROPERTIES = NTEVENTLOGPROPERTIES(1026i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkPolicyServer\"`*"]
pub const PROPERTY_EVENTLOG_LOG_MALFORMED: NTEVENTLOGPROPERTIES = NTEVENTLOGPROPERTIES(1027i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkPolicyServer\"`*"]
pub const PROPERTY_EVENTLOG_LOG_DEBUG: NTEVENTLOGPROPERTIES = NTEVENTLOGPROPERTIES(1028i32);
impl ::core::marker::Copy for NTEVENTLOGPROPERTIES {}
impl ::core::clone::Clone for NTEVENTLOGPROPERTIES {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for NTEVENTLOGPROPERTIES {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for NTEVENTLOGPROPERTIES {
    type Abi = Self;
}
impl ::core::fmt::Debug for NTEVENTLOGPROPERTIES {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("NTEVENTLOGPROPERTIES").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkPolicyServer\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct NTSAMPROPERTIES(pub i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkPolicyServer\"`*"]
pub const PROPERTY_NTSAM_ALLOW_LM_AUTHENTICATION: NTSAMPROPERTIES = NTSAMPROPERTIES(1026i32);
impl ::core::marker::Copy for NTSAMPROPERTIES {}
impl ::core::clone::Clone for NTSAMPROPERTIES {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for NTSAMPROPERTIES {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for NTSAMPROPERTIES {
    type Abi = Self;
}
impl ::core::fmt::Debug for NTSAMPROPERTIES {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("NTSAMPROPERTIES").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkPolicyServer\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct POLICYPROPERTIES(pub i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkPolicyServer\"`*"]
pub const PROPERTY_POLICY_CONSTRAINT: POLICYPROPERTIES = POLICYPROPERTIES(1024i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkPolicyServer\"`*"]
pub const PROPERTY_POLICY_MERIT: POLICYPROPERTIES = POLICYPROPERTIES(1025i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkPolicyServer\"`*"]
pub const PROPERTY_POLICY_UNUSED0: POLICYPROPERTIES = POLICYPROPERTIES(1026i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkPolicyServer\"`*"]
pub const PROPERTY_POLICY_UNUSED1: POLICYPROPERTIES = POLICYPROPERTIES(1027i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkPolicyServer\"`*"]
pub const PROPERTY_POLICY_PROFILE_NAME: POLICYPROPERTIES = POLICYPROPERTIES(1028i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkPolicyServer\"`*"]
pub const PROPERTY_POLICY_ACTION: POLICYPROPERTIES = POLICYPROPERTIES(1029i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkPolicyServer\"`*"]
pub const PROPERTY_POLICY_CONDITIONS_COLLECTION: POLICYPROPERTIES = POLICYPROPERTIES(1030i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkPolicyServer\"`*"]
pub const PROPERTY_POLICY_ENABLED: POLICYPROPERTIES = POLICYPROPERTIES(1031i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkPolicyServer\"`*"]
pub const PROPERTY_POLICY_SOURCETAG: POLICYPROPERTIES = POLICYPROPERTIES(1032i32);
impl ::core::marker::Copy for POLICYPROPERTIES {}
impl ::core::clone::Clone for POLICYPROPERTIES {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for POLICYPROPERTIES {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for POLICYPROPERTIES {
    type Abi = Self;
}
impl ::core::fmt::Debug for POLICYPROPERTIES {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("POLICYPROPERTIES").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkPolicyServer\"`*"]
pub type PRADIUS_EXTENSION_FREE_ATTRIBUTES = ::core::option::Option<unsafe extern "system" fn(pattrs: *mut RADIUS_ATTRIBUTE)>;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkPolicyServer\"`*"]
pub type PRADIUS_EXTENSION_INIT = ::core::option::Option<unsafe extern "system" fn() -> u32>;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkPolicyServer\"`*"]
pub type PRADIUS_EXTENSION_PROCESS = ::core::option::Option<unsafe extern "system" fn(pattrs: *const RADIUS_ATTRIBUTE, pfaction: *mut RADIUS_ACTION) -> u32>;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkPolicyServer\"`*"]
pub type PRADIUS_EXTENSION_PROCESS_2 = ::core::option::Option<unsafe extern "system" fn(pecb: *mut RADIUS_EXTENSION_CONTROL_BLOCK) -> u32>;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkPolicyServer\"`*"]
pub type PRADIUS_EXTENSION_PROCESS_EX = ::core::option::Option<unsafe extern "system" fn(pinattrs: *const RADIUS_ATTRIBUTE, poutattrs: *mut *mut RADIUS_ATTRIBUTE, pfaction: *mut RADIUS_ACTION) -> u32>;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkPolicyServer\"`*"]
pub type PRADIUS_EXTENSION_TERM = ::core::option::Option<unsafe extern "system" fn()>;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkPolicyServer\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct PROFILEPROPERTIES(pub i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkPolicyServer\"`*"]
pub const PROPERTY_PROFILE_ATTRIBUTES_COLLECTION: PROFILEPROPERTIES = PROFILEPROPERTIES(1024i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkPolicyServer\"`*"]
pub const PROPERTY_PROFILE_IPFILTER_TEMPLATE_GUID: PROFILEPROPERTIES = PROFILEPROPERTIES(1025i32);
impl ::core::marker::Copy for PROFILEPROPERTIES {}
impl ::core::clone::Clone for PROFILEPROPERTIES {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for PROFILEPROPERTIES {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for PROFILEPROPERTIES {
    type Abi = Self;
}
impl ::core::fmt::Debug for PROFILEPROPERTIES {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PROFILEPROPERTIES").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkPolicyServer\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct PROTOCOLPROPERTIES(pub i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkPolicyServer\"`*"]
pub const PROPERTY_PROTOCOL_REQUEST_HANDLER: PROTOCOLPROPERTIES = PROTOCOLPROPERTIES(1026i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkPolicyServer\"`*"]
pub const PROPERTY_PROTOCOL_START: PROTOCOLPROPERTIES = PROTOCOLPROPERTIES(1027i32);
impl ::core::marker::Copy for PROTOCOLPROPERTIES {}
impl ::core::clone::Clone for PROTOCOLPROPERTIES {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for PROTOCOLPROPERTIES {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for PROTOCOLPROPERTIES {
    type Abi = Self;
}
impl ::core::fmt::Debug for PROTOCOLPROPERTIES {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PROTOCOLPROPERTIES").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkPolicyServer\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct RADIUSPROPERTIES(pub i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkPolicyServer\"`*"]
pub const PROPERTY_RADIUS_ACCOUNTING_PORT: RADIUSPROPERTIES = RADIUSPROPERTIES(1027i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkPolicyServer\"`*"]
pub const PROPERTY_RADIUS_AUTHENTICATION_PORT: RADIUSPROPERTIES = RADIUSPROPERTIES(1028i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkPolicyServer\"`*"]
pub const PROPERTY_RADIUS_CLIENTS_COLLECTION: RADIUSPROPERTIES = RADIUSPROPERTIES(1029i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkPolicyServer\"`*"]
pub const PROPERTY_RADIUS_VENDORS_COLLECTION: RADIUSPROPERTIES = RADIUSPROPERTIES(1030i32);
impl ::core::marker::Copy for RADIUSPROPERTIES {}
impl ::core::clone::Clone for RADIUSPROPERTIES {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for RADIUSPROPERTIES {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for RADIUSPROPERTIES {
    type Abi = Self;
}
impl ::core::fmt::Debug for RADIUSPROPERTIES {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("RADIUSPROPERTIES").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkPolicyServer\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct RADIUSPROXYPROPERTIES(pub i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkPolicyServer\"`*"]
pub const PROPERTY_RADIUSPROXY_SERVERGROUPS: RADIUSPROXYPROPERTIES = RADIUSPROXYPROPERTIES(1026i32);
impl ::core::marker::Copy for RADIUSPROXYPROPERTIES {}
impl ::core::clone::Clone for RADIUSPROXYPROPERTIES {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for RADIUSPROXYPROPERTIES {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for RADIUSPROXYPROPERTIES {
    type Abi = Self;
}
impl ::core::fmt::Debug for RADIUSPROXYPROPERTIES {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("RADIUSPROXYPROPERTIES").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkPolicyServer\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct RADIUSSERVERGROUPPROPERTIES(pub i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkPolicyServer\"`*"]
pub const PROPERTY_RADIUSSERVERGROUP_SERVERS_COLLECTION: RADIUSSERVERGROUPPROPERTIES = RADIUSSERVERGROUPPROPERTIES(1024i32);
impl ::core::marker::Copy for RADIUSSERVERGROUPPROPERTIES {}
impl ::core::clone::Clone for RADIUSSERVERGROUPPROPERTIES {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for RADIUSSERVERGROUPPROPERTIES {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for RADIUSSERVERGROUPPROPERTIES {
    type Abi = Self;
}
impl ::core::fmt::Debug for RADIUSSERVERGROUPPROPERTIES {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("RADIUSSERVERGROUPPROPERTIES").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkPolicyServer\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct RADIUSSERVERPROPERTIES(pub i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkPolicyServer\"`*"]
pub const PROPERTY_RADIUSSERVER_AUTH_PORT: RADIUSSERVERPROPERTIES = RADIUSSERVERPROPERTIES(1024i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkPolicyServer\"`*"]
pub const PROPERTY_RADIUSSERVER_AUTH_SECRET: RADIUSSERVERPROPERTIES = RADIUSSERVERPROPERTIES(1025i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkPolicyServer\"`*"]
pub const PROPERTY_RADIUSSERVER_ACCT_PORT: RADIUSSERVERPROPERTIES = RADIUSSERVERPROPERTIES(1026i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkPolicyServer\"`*"]
pub const PROPERTY_RADIUSSERVER_ACCT_SECRET: RADIUSSERVERPROPERTIES = RADIUSSERVERPROPERTIES(1027i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkPolicyServer\"`*"]
pub const PROPERTY_RADIUSSERVER_ADDRESS: RADIUSSERVERPROPERTIES = RADIUSSERVERPROPERTIES(1028i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkPolicyServer\"`*"]
pub const PROPERTY_RADIUSSERVER_FORWARD_ACCT_ONOFF: RADIUSSERVERPROPERTIES = RADIUSSERVERPROPERTIES(1029i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkPolicyServer\"`*"]
pub const PROPERTY_RADIUSSERVER_PRIORITY: RADIUSSERVERPROPERTIES = RADIUSSERVERPROPERTIES(1030i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkPolicyServer\"`*"]
pub const PROPERTY_RADIUSSERVER_WEIGHT: RADIUSSERVERPROPERTIES = RADIUSSERVERPROPERTIES(1031i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkPolicyServer\"`*"]
pub const PROPERTY_RADIUSSERVER_TIMEOUT: RADIUSSERVERPROPERTIES = RADIUSSERVERPROPERTIES(1032i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkPolicyServer\"`*"]
pub const PROPERTY_RADIUSSERVER_MAX_LOST: RADIUSSERVERPROPERTIES = RADIUSSERVERPROPERTIES(1033i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkPolicyServer\"`*"]
pub const PROPERTY_RADIUSSERVER_BLACKOUT: RADIUSSERVERPROPERTIES = RADIUSSERVERPROPERTIES(1034i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkPolicyServer\"`*"]
pub const PROPERTY_RADIUSSERVER_SEND_SIGNATURE: RADIUSSERVERPROPERTIES = RADIUSSERVERPROPERTIES(1035i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkPolicyServer\"`*"]
pub const PROPERTY_RADIUSSERVER_AUTH_SECRET_TEMPLATE_GUID: RADIUSSERVERPROPERTIES = RADIUSSERVERPROPERTIES(1036i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkPolicyServer\"`*"]
pub const PROPERTY_RADIUSSERVER_ACCT_SECRET_TEMPLATE_GUID: RADIUSSERVERPROPERTIES = RADIUSSERVERPROPERTIES(1037i32);
impl ::core::marker::Copy for RADIUSSERVERPROPERTIES {}
impl ::core::clone::Clone for RADIUSSERVERPROPERTIES {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for RADIUSSERVERPROPERTIES {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for RADIUSSERVERPROPERTIES {
    type Abi = Self;
}
impl ::core::fmt::Debug for RADIUSSERVERPROPERTIES {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("RADIUSSERVERPROPERTIES").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkPolicyServer\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct RADIUS_ACTION(pub i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkPolicyServer\"`*"]
pub const raContinue: RADIUS_ACTION = RADIUS_ACTION(0i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkPolicyServer\"`*"]
pub const raReject: RADIUS_ACTION = RADIUS_ACTION(1i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkPolicyServer\"`*"]
pub const raAccept: RADIUS_ACTION = RADIUS_ACTION(2i32);
impl ::core::marker::Copy for RADIUS_ACTION {}
impl ::core::clone::Clone for RADIUS_ACTION {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for RADIUS_ACTION {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for RADIUS_ACTION {
    type Abi = Self;
}
impl ::core::fmt::Debug for RADIUS_ACTION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("RADIUS_ACTION").field(&self.0).finish()
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkPolicyServer\"`*"]
pub struct RADIUS_ATTRIBUTE {
    pub dwAttrType: u32,
    pub fDataType: RADIUS_DATA_TYPE,
    pub cbDataLength: u32,
    pub Anonymous: RADIUS_ATTRIBUTE_0,
}
impl ::core::marker::Copy for RADIUS_ATTRIBUTE {}
impl ::core::clone::Clone for RADIUS_ATTRIBUTE {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for RADIUS_ATTRIBUTE {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for RADIUS_ATTRIBUTE {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<RADIUS_ATTRIBUTE>()) == 0 }
    }
}
impl ::core::cmp::Eq for RADIUS_ATTRIBUTE {}
impl ::core::default::Default for RADIUS_ATTRIBUTE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkPolicyServer\"`*"]
pub union RADIUS_ATTRIBUTE_0 {
    pub dwValue: u32,
    pub lpValue: *const u8,
}
impl ::core::marker::Copy for RADIUS_ATTRIBUTE_0 {}
impl ::core::clone::Clone for RADIUS_ATTRIBUTE_0 {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for RADIUS_ATTRIBUTE_0 {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for RADIUS_ATTRIBUTE_0 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<RADIUS_ATTRIBUTE_0>()) == 0 }
    }
}
impl ::core::cmp::Eq for RADIUS_ATTRIBUTE_0 {}
impl ::core::default::Default for RADIUS_ATTRIBUTE_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkPolicyServer\"`*"]
pub struct RADIUS_ATTRIBUTE_ARRAY {
    pub cbSize: u32,
    pub Add: isize,
    pub AttributeAt: isize,
    pub GetSize: isize,
    pub InsertAt: isize,
    pub RemoveAt: isize,
    pub SetAt: isize,
}
impl ::core::marker::Copy for RADIUS_ATTRIBUTE_ARRAY {}
impl ::core::clone::Clone for RADIUS_ATTRIBUTE_ARRAY {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for RADIUS_ATTRIBUTE_ARRAY {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("RADIUS_ATTRIBUTE_ARRAY").field("cbSize", &self.cbSize).field("Add", &self.Add).field("AttributeAt", &self.AttributeAt).field("GetSize", &self.GetSize).field("InsertAt", &self.InsertAt).field("RemoveAt", &self.RemoveAt).field("SetAt", &self.SetAt).finish()
    }
}
unsafe impl ::windows::core::Abi for RADIUS_ATTRIBUTE_ARRAY {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for RADIUS_ATTRIBUTE_ARRAY {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<RADIUS_ATTRIBUTE_ARRAY>()) == 0 }
    }
}
impl ::core::cmp::Eq for RADIUS_ATTRIBUTE_ARRAY {}
impl ::core::default::Default for RADIUS_ATTRIBUTE_ARRAY {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkPolicyServer\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct RADIUS_ATTRIBUTE_TYPE(pub i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkPolicyServer\"`*"]
pub const ratMinimum: RADIUS_ATTRIBUTE_TYPE = RADIUS_ATTRIBUTE_TYPE(0i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkPolicyServer\"`*"]
pub const ratUserName: RADIUS_ATTRIBUTE_TYPE = RADIUS_ATTRIBUTE_TYPE(1i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkPolicyServer\"`*"]
pub const ratUserPassword: RADIUS_ATTRIBUTE_TYPE = RADIUS_ATTRIBUTE_TYPE(2i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkPolicyServer\"`*"]
pub const ratCHAPPassword: RADIUS_ATTRIBUTE_TYPE = RADIUS_ATTRIBUTE_TYPE(3i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkPolicyServer\"`*"]
pub const ratNASIPAddress: RADIUS_ATTRIBUTE_TYPE = RADIUS_ATTRIBUTE_TYPE(4i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkPolicyServer\"`*"]
pub const ratNASPort: RADIUS_ATTRIBUTE_TYPE = RADIUS_ATTRIBUTE_TYPE(5i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkPolicyServer\"`*"]
pub const ratServiceType: RADIUS_ATTRIBUTE_TYPE = RADIUS_ATTRIBUTE_TYPE(6i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkPolicyServer\"`*"]
pub const ratFramedProtocol: RADIUS_ATTRIBUTE_TYPE = RADIUS_ATTRIBUTE_TYPE(7i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkPolicyServer\"`*"]
pub const ratFramedIPAddress: RADIUS_ATTRIBUTE_TYPE = RADIUS_ATTRIBUTE_TYPE(8i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkPolicyServer\"`*"]
pub const ratFramedIPNetmask: RADIUS_ATTRIBUTE_TYPE = RADIUS_ATTRIBUTE_TYPE(9i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkPolicyServer\"`*"]
pub const ratFramedRouting: RADIUS_ATTRIBUTE_TYPE = RADIUS_ATTRIBUTE_TYPE(10i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkPolicyServer\"`*"]
pub const ratFilterId: RADIUS_ATTRIBUTE_TYPE = RADIUS_ATTRIBUTE_TYPE(11i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkPolicyServer\"`*"]
pub const ratFramedMTU: RADIUS_ATTRIBUTE_TYPE = RADIUS_ATTRIBUTE_TYPE(12i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkPolicyServer\"`*"]
pub const ratFramedCompression: RADIUS_ATTRIBUTE_TYPE = RADIUS_ATTRIBUTE_TYPE(13i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkPolicyServer\"`*"]
pub const ratLoginIPHost: RADIUS_ATTRIBUTE_TYPE = RADIUS_ATTRIBUTE_TYPE(14i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkPolicyServer\"`*"]
pub const ratLoginService: RADIUS_ATTRIBUTE_TYPE = RADIUS_ATTRIBUTE_TYPE(15i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkPolicyServer\"`*"]
pub const ratLoginPort: RADIUS_ATTRIBUTE_TYPE = RADIUS_ATTRIBUTE_TYPE(16i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkPolicyServer\"`*"]
pub const ratReplyMessage: RADIUS_ATTRIBUTE_TYPE = RADIUS_ATTRIBUTE_TYPE(18i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkPolicyServer\"`*"]
pub const ratCallbackNumber: RADIUS_ATTRIBUTE_TYPE = RADIUS_ATTRIBUTE_TYPE(19i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkPolicyServer\"`*"]
pub const ratCallbackId: RADIUS_ATTRIBUTE_TYPE = RADIUS_ATTRIBUTE_TYPE(20i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkPolicyServer\"`*"]
pub const ratFramedRoute: RADIUS_ATTRIBUTE_TYPE = RADIUS_ATTRIBUTE_TYPE(22i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkPolicyServer\"`*"]
pub const ratFramedIPXNetwork: RADIUS_ATTRIBUTE_TYPE = RADIUS_ATTRIBUTE_TYPE(23i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkPolicyServer\"`*"]
pub const ratState: RADIUS_ATTRIBUTE_TYPE = RADIUS_ATTRIBUTE_TYPE(24i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkPolicyServer\"`*"]
pub const ratClass: RADIUS_ATTRIBUTE_TYPE = RADIUS_ATTRIBUTE_TYPE(25i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkPolicyServer\"`*"]
pub const ratVendorSpecific: RADIUS_ATTRIBUTE_TYPE = RADIUS_ATTRIBUTE_TYPE(26i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkPolicyServer\"`*"]
pub const ratSessionTimeout: RADIUS_ATTRIBUTE_TYPE = RADIUS_ATTRIBUTE_TYPE(27i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkPolicyServer\"`*"]
pub const ratIdleTimeout: RADIUS_ATTRIBUTE_TYPE = RADIUS_ATTRIBUTE_TYPE(28i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkPolicyServer\"`*"]
pub const ratTerminationAction: RADIUS_ATTRIBUTE_TYPE = RADIUS_ATTRIBUTE_TYPE(29i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkPolicyServer\"`*"]
pub const ratCalledStationId: RADIUS_ATTRIBUTE_TYPE = RADIUS_ATTRIBUTE_TYPE(30i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkPolicyServer\"`*"]
pub const ratCallingStationId: RADIUS_ATTRIBUTE_TYPE = RADIUS_ATTRIBUTE_TYPE(31i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkPolicyServer\"`*"]
pub const ratNASIdentifier: RADIUS_ATTRIBUTE_TYPE = RADIUS_ATTRIBUTE_TYPE(32i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkPolicyServer\"`*"]
pub const ratProxyState: RADIUS_ATTRIBUTE_TYPE = RADIUS_ATTRIBUTE_TYPE(33i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkPolicyServer\"`*"]
pub const ratLoginLATService: RADIUS_ATTRIBUTE_TYPE = RADIUS_ATTRIBUTE_TYPE(34i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkPolicyServer\"`*"]
pub const ratLoginLATNode: RADIUS_ATTRIBUTE_TYPE = RADIUS_ATTRIBUTE_TYPE(35i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkPolicyServer\"`*"]
pub const ratLoginLATGroup: RADIUS_ATTRIBUTE_TYPE = RADIUS_ATTRIBUTE_TYPE(36i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkPolicyServer\"`*"]
pub const ratFramedAppleTalkLink: RADIUS_ATTRIBUTE_TYPE = RADIUS_ATTRIBUTE_TYPE(37i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkPolicyServer\"`*"]
pub const ratFramedAppleTalkNetwork: RADIUS_ATTRIBUTE_TYPE = RADIUS_ATTRIBUTE_TYPE(38i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkPolicyServer\"`*"]
pub const ratFramedAppleTalkZone: RADIUS_ATTRIBUTE_TYPE = RADIUS_ATTRIBUTE_TYPE(39i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkPolicyServer\"`*"]
pub const ratAcctStatusType: RADIUS_ATTRIBUTE_TYPE = RADIUS_ATTRIBUTE_TYPE(40i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkPolicyServer\"`*"]
pub const ratAcctDelayTime: RADIUS_ATTRIBUTE_TYPE = RADIUS_ATTRIBUTE_TYPE(41i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkPolicyServer\"`*"]
pub const ratAcctInputOctets: RADIUS_ATTRIBUTE_TYPE = RADIUS_ATTRIBUTE_TYPE(42i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkPolicyServer\"`*"]
pub const ratAcctOutputOctets: RADIUS_ATTRIBUTE_TYPE = RADIUS_ATTRIBUTE_TYPE(43i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkPolicyServer\"`*"]
pub const ratAcctSessionId: RADIUS_ATTRIBUTE_TYPE = RADIUS_ATTRIBUTE_TYPE(44i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkPolicyServer\"`*"]
pub const ratAcctAuthentic: RADIUS_ATTRIBUTE_TYPE = RADIUS_ATTRIBUTE_TYPE(45i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkPolicyServer\"`*"]
pub const ratAcctSessionTime: RADIUS_ATTRIBUTE_TYPE = RADIUS_ATTRIBUTE_TYPE(46i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkPolicyServer\"`*"]
pub const ratAcctInputPackets: RADIUS_ATTRIBUTE_TYPE = RADIUS_ATTRIBUTE_TYPE(47i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkPolicyServer\"`*"]
pub const ratAcctOutputPackets: RADIUS_ATTRIBUTE_TYPE = RADIUS_ATTRIBUTE_TYPE(48i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkPolicyServer\"`*"]
pub const ratAcctTerminationCause: RADIUS_ATTRIBUTE_TYPE = RADIUS_ATTRIBUTE_TYPE(49i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkPolicyServer\"`*"]
pub const ratCHAPChallenge: RADIUS_ATTRIBUTE_TYPE = RADIUS_ATTRIBUTE_TYPE(60i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkPolicyServer\"`*"]
pub const ratNASPortType: RADIUS_ATTRIBUTE_TYPE = RADIUS_ATTRIBUTE_TYPE(61i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkPolicyServer\"`*"]
pub const ratPortLimit: RADIUS_ATTRIBUTE_TYPE = RADIUS_ATTRIBUTE_TYPE(62i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkPolicyServer\"`*"]
pub const ratTunnelType: RADIUS_ATTRIBUTE_TYPE = RADIUS_ATTRIBUTE_TYPE(64i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkPolicyServer\"`*"]
pub const ratMediumType: RADIUS_ATTRIBUTE_TYPE = RADIUS_ATTRIBUTE_TYPE(65i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkPolicyServer\"`*"]
pub const ratTunnelPassword: RADIUS_ATTRIBUTE_TYPE = RADIUS_ATTRIBUTE_TYPE(69i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkPolicyServer\"`*"]
pub const ratTunnelPrivateGroupID: RADIUS_ATTRIBUTE_TYPE = RADIUS_ATTRIBUTE_TYPE(81i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkPolicyServer\"`*"]
pub const ratNASIPv6Address: RADIUS_ATTRIBUTE_TYPE = RADIUS_ATTRIBUTE_TYPE(95i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkPolicyServer\"`*"]
pub const ratFramedInterfaceId: RADIUS_ATTRIBUTE_TYPE = RADIUS_ATTRIBUTE_TYPE(96i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkPolicyServer\"`*"]
pub const ratFramedIPv6Prefix: RADIUS_ATTRIBUTE_TYPE = RADIUS_ATTRIBUTE_TYPE(97i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkPolicyServer\"`*"]
pub const ratLoginIPv6Host: RADIUS_ATTRIBUTE_TYPE = RADIUS_ATTRIBUTE_TYPE(98i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkPolicyServer\"`*"]
pub const ratFramedIPv6Route: RADIUS_ATTRIBUTE_TYPE = RADIUS_ATTRIBUTE_TYPE(99i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkPolicyServer\"`*"]
pub const ratFramedIPv6Pool: RADIUS_ATTRIBUTE_TYPE = RADIUS_ATTRIBUTE_TYPE(100i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkPolicyServer\"`*"]
pub const ratCode: RADIUS_ATTRIBUTE_TYPE = RADIUS_ATTRIBUTE_TYPE(262i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkPolicyServer\"`*"]
pub const ratIdentifier: RADIUS_ATTRIBUTE_TYPE = RADIUS_ATTRIBUTE_TYPE(263i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkPolicyServer\"`*"]
pub const ratAuthenticator: RADIUS_ATTRIBUTE_TYPE = RADIUS_ATTRIBUTE_TYPE(264i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkPolicyServer\"`*"]
pub const ratSrcIPAddress: RADIUS_ATTRIBUTE_TYPE = RADIUS_ATTRIBUTE_TYPE(265i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkPolicyServer\"`*"]
pub const ratSrcPort: RADIUS_ATTRIBUTE_TYPE = RADIUS_ATTRIBUTE_TYPE(266i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkPolicyServer\"`*"]
pub const ratProvider: RADIUS_ATTRIBUTE_TYPE = RADIUS_ATTRIBUTE_TYPE(267i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkPolicyServer\"`*"]
pub const ratStrippedUserName: RADIUS_ATTRIBUTE_TYPE = RADIUS_ATTRIBUTE_TYPE(268i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkPolicyServer\"`*"]
pub const ratFQUserName: RADIUS_ATTRIBUTE_TYPE = RADIUS_ATTRIBUTE_TYPE(269i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkPolicyServer\"`*"]
pub const ratPolicyName: RADIUS_ATTRIBUTE_TYPE = RADIUS_ATTRIBUTE_TYPE(270i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkPolicyServer\"`*"]
pub const ratUniqueId: RADIUS_ATTRIBUTE_TYPE = RADIUS_ATTRIBUTE_TYPE(271i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkPolicyServer\"`*"]
pub const ratExtensionState: RADIUS_ATTRIBUTE_TYPE = RADIUS_ATTRIBUTE_TYPE(272i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkPolicyServer\"`*"]
pub const ratEAPTLV: RADIUS_ATTRIBUTE_TYPE = RADIUS_ATTRIBUTE_TYPE(273i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkPolicyServer\"`*"]
pub const ratRejectReasonCode: RADIUS_ATTRIBUTE_TYPE = RADIUS_ATTRIBUTE_TYPE(274i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkPolicyServer\"`*"]
pub const ratCRPPolicyName: RADIUS_ATTRIBUTE_TYPE = RADIUS_ATTRIBUTE_TYPE(275i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkPolicyServer\"`*"]
pub const ratProviderName: RADIUS_ATTRIBUTE_TYPE = RADIUS_ATTRIBUTE_TYPE(276i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkPolicyServer\"`*"]
pub const ratClearTextPassword: RADIUS_ATTRIBUTE_TYPE = RADIUS_ATTRIBUTE_TYPE(277i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkPolicyServer\"`*"]
pub const ratSrcIPv6Address: RADIUS_ATTRIBUTE_TYPE = RADIUS_ATTRIBUTE_TYPE(278i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkPolicyServer\"`*"]
pub const ratCertificateThumbprint: RADIUS_ATTRIBUTE_TYPE = RADIUS_ATTRIBUTE_TYPE(279i32);
impl ::core::marker::Copy for RADIUS_ATTRIBUTE_TYPE {}
impl ::core::clone::Clone for RADIUS_ATTRIBUTE_TYPE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for RADIUS_ATTRIBUTE_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for RADIUS_ATTRIBUTE_TYPE {
    type Abi = Self;
}
impl ::core::fmt::Debug for RADIUS_ATTRIBUTE_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("RADIUS_ATTRIBUTE_TYPE").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkPolicyServer\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct RADIUS_AUTHENTICATION_PROVIDER(pub i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkPolicyServer\"`*"]
pub const rapUnknown: RADIUS_AUTHENTICATION_PROVIDER = RADIUS_AUTHENTICATION_PROVIDER(0i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkPolicyServer\"`*"]
pub const rapUsersFile: RADIUS_AUTHENTICATION_PROVIDER = RADIUS_AUTHENTICATION_PROVIDER(1i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkPolicyServer\"`*"]
pub const rapProxy: RADIUS_AUTHENTICATION_PROVIDER = RADIUS_AUTHENTICATION_PROVIDER(2i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkPolicyServer\"`*"]
pub const rapWindowsNT: RADIUS_AUTHENTICATION_PROVIDER = RADIUS_AUTHENTICATION_PROVIDER(3i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkPolicyServer\"`*"]
pub const rapMCIS: RADIUS_AUTHENTICATION_PROVIDER = RADIUS_AUTHENTICATION_PROVIDER(4i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkPolicyServer\"`*"]
pub const rapODBC: RADIUS_AUTHENTICATION_PROVIDER = RADIUS_AUTHENTICATION_PROVIDER(5i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkPolicyServer\"`*"]
pub const rapNone: RADIUS_AUTHENTICATION_PROVIDER = RADIUS_AUTHENTICATION_PROVIDER(6i32);
impl ::core::marker::Copy for RADIUS_AUTHENTICATION_PROVIDER {}
impl ::core::clone::Clone for RADIUS_AUTHENTICATION_PROVIDER {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for RADIUS_AUTHENTICATION_PROVIDER {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for RADIUS_AUTHENTICATION_PROVIDER {
    type Abi = Self;
}
impl ::core::fmt::Debug for RADIUS_AUTHENTICATION_PROVIDER {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("RADIUS_AUTHENTICATION_PROVIDER").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkPolicyServer\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct RADIUS_CODE(pub i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkPolicyServer\"`*"]
pub const rcUnknown: RADIUS_CODE = RADIUS_CODE(0i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkPolicyServer\"`*"]
pub const rcAccessRequest: RADIUS_CODE = RADIUS_CODE(1i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkPolicyServer\"`*"]
pub const rcAccessAccept: RADIUS_CODE = RADIUS_CODE(2i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkPolicyServer\"`*"]
pub const rcAccessReject: RADIUS_CODE = RADIUS_CODE(3i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkPolicyServer\"`*"]
pub const rcAccountingRequest: RADIUS_CODE = RADIUS_CODE(4i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkPolicyServer\"`*"]
pub const rcAccountingResponse: RADIUS_CODE = RADIUS_CODE(5i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkPolicyServer\"`*"]
pub const rcAccessChallenge: RADIUS_CODE = RADIUS_CODE(11i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkPolicyServer\"`*"]
pub const rcDiscard: RADIUS_CODE = RADIUS_CODE(256i32);
impl ::core::marker::Copy for RADIUS_CODE {}
impl ::core::clone::Clone for RADIUS_CODE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for RADIUS_CODE {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for RADIUS_CODE {
    type Abi = Self;
}
impl ::core::fmt::Debug for RADIUS_CODE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("RADIUS_CODE").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkPolicyServer\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct RADIUS_DATA_TYPE(pub i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkPolicyServer\"`*"]
pub const rdtUnknown: RADIUS_DATA_TYPE = RADIUS_DATA_TYPE(0i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkPolicyServer\"`*"]
pub const rdtString: RADIUS_DATA_TYPE = RADIUS_DATA_TYPE(1i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkPolicyServer\"`*"]
pub const rdtAddress: RADIUS_DATA_TYPE = RADIUS_DATA_TYPE(2i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkPolicyServer\"`*"]
pub const rdtInteger: RADIUS_DATA_TYPE = RADIUS_DATA_TYPE(3i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkPolicyServer\"`*"]
pub const rdtTime: RADIUS_DATA_TYPE = RADIUS_DATA_TYPE(4i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkPolicyServer\"`*"]
pub const rdtIpv6Address: RADIUS_DATA_TYPE = RADIUS_DATA_TYPE(5i32);
impl ::core::marker::Copy for RADIUS_DATA_TYPE {}
impl ::core::clone::Clone for RADIUS_DATA_TYPE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for RADIUS_DATA_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for RADIUS_DATA_TYPE {
    type Abi = Self;
}
impl ::core::fmt::Debug for RADIUS_DATA_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("RADIUS_DATA_TYPE").field(&self.0).finish()
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkPolicyServer\"`*"]
pub struct RADIUS_EXTENSION_CONTROL_BLOCK {
    pub cbSize: u32,
    pub dwVersion: u32,
    pub repPoint: RADIUS_EXTENSION_POINT,
    pub rcRequestType: RADIUS_CODE,
    pub rcResponseType: RADIUS_CODE,
    pub GetRequest: isize,
    pub GetResponse: isize,
    pub SetResponseType: isize,
}
impl ::core::marker::Copy for RADIUS_EXTENSION_CONTROL_BLOCK {}
impl ::core::clone::Clone for RADIUS_EXTENSION_CONTROL_BLOCK {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for RADIUS_EXTENSION_CONTROL_BLOCK {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("RADIUS_EXTENSION_CONTROL_BLOCK").field("cbSize", &self.cbSize).field("dwVersion", &self.dwVersion).field("repPoint", &self.repPoint).field("rcRequestType", &self.rcRequestType).field("rcResponseType", &self.rcResponseType).field("GetRequest", &self.GetRequest).field("GetResponse", &self.GetResponse).field("SetResponseType", &self.SetResponseType).finish()
    }
}
unsafe impl ::windows::core::Abi for RADIUS_EXTENSION_CONTROL_BLOCK {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for RADIUS_EXTENSION_CONTROL_BLOCK {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<RADIUS_EXTENSION_CONTROL_BLOCK>()) == 0 }
    }
}
impl ::core::cmp::Eq for RADIUS_EXTENSION_CONTROL_BLOCK {}
impl ::core::default::Default for RADIUS_EXTENSION_CONTROL_BLOCK {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkPolicyServer\"`*"]
pub const RADIUS_EXTENSION_FREE_ATTRIBUTES: &'static str = "RadiusExtensionFreeAttributes";
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkPolicyServer\"`*"]
pub const RADIUS_EXTENSION_INIT: &'static str = "RadiusExtensionInit";
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkPolicyServer\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct RADIUS_EXTENSION_POINT(pub i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkPolicyServer\"`*"]
pub const repAuthentication: RADIUS_EXTENSION_POINT = RADIUS_EXTENSION_POINT(0i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkPolicyServer\"`*"]
pub const repAuthorization: RADIUS_EXTENSION_POINT = RADIUS_EXTENSION_POINT(1i32);
impl ::core::marker::Copy for RADIUS_EXTENSION_POINT {}
impl ::core::clone::Clone for RADIUS_EXTENSION_POINT {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for RADIUS_EXTENSION_POINT {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for RADIUS_EXTENSION_POINT {
    type Abi = Self;
}
impl ::core::fmt::Debug for RADIUS_EXTENSION_POINT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("RADIUS_EXTENSION_POINT").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkPolicyServer\"`*"]
pub const RADIUS_EXTENSION_PROCESS: &'static str = "RadiusExtensionProcess";
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkPolicyServer\"`*"]
pub const RADIUS_EXTENSION_PROCESS2: &'static str = "RadiusExtensionProcess2";
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkPolicyServer\"`*"]
pub const RADIUS_EXTENSION_PROCESS_EX: &'static str = "RadiusExtensionProcessEx";
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkPolicyServer\"`*"]
pub const RADIUS_EXTENSION_TERM: &'static str = "RadiusExtensionTerm";
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkPolicyServer\"`*"]
pub const RADIUS_EXTENSION_VERSION: u32 = 1u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkPolicyServer\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct RADIUS_REJECT_REASON_CODE(pub i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkPolicyServer\"`*"]
pub const rrrcUndefined: RADIUS_REJECT_REASON_CODE = RADIUS_REJECT_REASON_CODE(0i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkPolicyServer\"`*"]
pub const rrrcAccountUnknown: RADIUS_REJECT_REASON_CODE = RADIUS_REJECT_REASON_CODE(1i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkPolicyServer\"`*"]
pub const rrrcAccountDisabled: RADIUS_REJECT_REASON_CODE = RADIUS_REJECT_REASON_CODE(2i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkPolicyServer\"`*"]
pub const rrrcAccountExpired: RADIUS_REJECT_REASON_CODE = RADIUS_REJECT_REASON_CODE(3i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkPolicyServer\"`*"]
pub const rrrcAuthenticationFailure: RADIUS_REJECT_REASON_CODE = RADIUS_REJECT_REASON_CODE(4i32);
impl ::core::marker::Copy for RADIUS_REJECT_REASON_CODE {}
impl ::core::clone::Clone for RADIUS_REJECT_REASON_CODE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for RADIUS_REJECT_REASON_CODE {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for RADIUS_REJECT_REASON_CODE {
    type Abi = Self;
}
impl ::core::fmt::Debug for RADIUS_REJECT_REASON_CODE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("RADIUS_REJECT_REASON_CODE").field(&self.0).finish()
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkPolicyServer\"`*"]
pub struct RADIUS_VSA_FORMAT {
    pub VendorId: [u8; 4],
    pub VendorType: u8,
    pub VendorLength: u8,
    pub AttributeSpecific: [u8; 1],
}
impl ::core::marker::Copy for RADIUS_VSA_FORMAT {}
impl ::core::clone::Clone for RADIUS_VSA_FORMAT {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for RADIUS_VSA_FORMAT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("RADIUS_VSA_FORMAT").field("VendorId", &self.VendorId).field("VendorType", &self.VendorType).field("VendorLength", &self.VendorLength).field("AttributeSpecific", &self.AttributeSpecific).finish()
    }
}
unsafe impl ::windows::core::Abi for RADIUS_VSA_FORMAT {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for RADIUS_VSA_FORMAT {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<RADIUS_VSA_FORMAT>()) == 0 }
    }
}
impl ::core::cmp::Eq for RADIUS_VSA_FORMAT {}
impl ::core::default::Default for RADIUS_VSA_FORMAT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkPolicyServer\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct REMEDIATIONSERVERGROUPPROPERTIES(pub i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkPolicyServer\"`*"]
pub const PROPERTY_REMEDIATIONSERVERGROUP_SERVERS_COLLECTION: REMEDIATIONSERVERGROUPPROPERTIES = REMEDIATIONSERVERGROUPPROPERTIES(1024i32);
impl ::core::marker::Copy for REMEDIATIONSERVERGROUPPROPERTIES {}
impl ::core::clone::Clone for REMEDIATIONSERVERGROUPPROPERTIES {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for REMEDIATIONSERVERGROUPPROPERTIES {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for REMEDIATIONSERVERGROUPPROPERTIES {
    type Abi = Self;
}
impl ::core::fmt::Debug for REMEDIATIONSERVERGROUPPROPERTIES {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("REMEDIATIONSERVERGROUPPROPERTIES").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkPolicyServer\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct REMEDIATIONSERVERPROPERTIES(pub i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkPolicyServer\"`*"]
pub const PROPERTY_REMEDIATIONSERVER_ADDRESS: REMEDIATIONSERVERPROPERTIES = REMEDIATIONSERVERPROPERTIES(1024i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkPolicyServer\"`*"]
pub const PROPERTY_REMEDIATIONSERVER_FRIENDLY_NAME: REMEDIATIONSERVERPROPERTIES = REMEDIATIONSERVERPROPERTIES(1025i32);
impl ::core::marker::Copy for REMEDIATIONSERVERPROPERTIES {}
impl ::core::clone::Clone for REMEDIATIONSERVERPROPERTIES {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for REMEDIATIONSERVERPROPERTIES {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for REMEDIATIONSERVERPROPERTIES {
    type Abi = Self;
}
impl ::core::fmt::Debug for REMEDIATIONSERVERPROPERTIES {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("REMEDIATIONSERVERPROPERTIES").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkPolicyServer\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct REMEDIATIONSERVERSPROPERTIES(pub i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkPolicyServer\"`*"]
pub const PROPERTY_REMEDIATIONSERVERS_SERVERGROUPS: REMEDIATIONSERVERSPROPERTIES = REMEDIATIONSERVERSPROPERTIES(1026i32);
impl ::core::marker::Copy for REMEDIATIONSERVERSPROPERTIES {}
impl ::core::clone::Clone for REMEDIATIONSERVERSPROPERTIES {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for REMEDIATIONSERVERSPROPERTIES {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for REMEDIATIONSERVERSPROPERTIES {
    type Abi = Self;
}
impl ::core::fmt::Debug for REMEDIATIONSERVERSPROPERTIES {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("REMEDIATIONSERVERSPROPERTIES").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkPolicyServer\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct SERVICE_TYPE(pub i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkPolicyServer\"`*"]
pub const SERVICE_TYPE_IAS: SERVICE_TYPE = SERVICE_TYPE(0i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkPolicyServer\"`*"]
pub const SERVICE_TYPE_RAS: SERVICE_TYPE = SERVICE_TYPE(1i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkPolicyServer\"`*"]
pub const SERVICE_TYPE_RAMGMTSVC: SERVICE_TYPE = SERVICE_TYPE(2i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkPolicyServer\"`*"]
pub const SERVICE_TYPE_MAX: SERVICE_TYPE = SERVICE_TYPE(3i32);
impl ::core::marker::Copy for SERVICE_TYPE {}
impl ::core::clone::Clone for SERVICE_TYPE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for SERVICE_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for SERVICE_TYPE {
    type Abi = Self;
}
impl ::core::fmt::Debug for SERVICE_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SERVICE_TYPE").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkPolicyServer\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct SHAREDSECRETPROPERTIES(pub i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkPolicyServer\"`*"]
pub const PROPERTY_SHAREDSECRET_STRING: SHAREDSECRETPROPERTIES = SHAREDSECRETPROPERTIES(1024i32);
impl ::core::marker::Copy for SHAREDSECRETPROPERTIES {}
impl ::core::clone::Clone for SHAREDSECRETPROPERTIES {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for SHAREDSECRETPROPERTIES {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for SHAREDSECRETPROPERTIES {
    type Abi = Self;
}
impl ::core::fmt::Debug for SHAREDSECRETPROPERTIES {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SHAREDSECRETPROPERTIES").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkPolicyServer\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct SHVTEMPLATEPROPERTIES(pub i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkPolicyServer\"`*"]
pub const PROPERTY_SHV_COMBINATION_TYPE: SHVTEMPLATEPROPERTIES = SHVTEMPLATEPROPERTIES(1024i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkPolicyServer\"`*"]
pub const PROPERTY_SHV_LIST: SHVTEMPLATEPROPERTIES = SHVTEMPLATEPROPERTIES(1025i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkPolicyServer\"`*"]
pub const PROPERTY_SHVCONFIG_LIST: SHVTEMPLATEPROPERTIES = SHVTEMPLATEPROPERTIES(1026i32);
impl ::core::marker::Copy for SHVTEMPLATEPROPERTIES {}
impl ::core::clone::Clone for SHVTEMPLATEPROPERTIES {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for SHVTEMPLATEPROPERTIES {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for SHVTEMPLATEPROPERTIES {
    type Abi = Self;
}
impl ::core::fmt::Debug for SHVTEMPLATEPROPERTIES {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SHVTEMPLATEPROPERTIES").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkPolicyServer\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct SHV_COMBINATION_TYPE(pub i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkPolicyServer\"`*"]
pub const SHV_COMBINATION_TYPE_ALL_PASS: SHV_COMBINATION_TYPE = SHV_COMBINATION_TYPE(0i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkPolicyServer\"`*"]
pub const SHV_COMBINATION_TYPE_ALL_FAIL: SHV_COMBINATION_TYPE = SHV_COMBINATION_TYPE(1i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkPolicyServer\"`*"]
pub const SHV_COMBINATION_TYPE_ONE_OR_MORE_PASS: SHV_COMBINATION_TYPE = SHV_COMBINATION_TYPE(2i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkPolicyServer\"`*"]
pub const SHV_COMBINATION_TYPE_ONE_OR_MORE_FAIL: SHV_COMBINATION_TYPE = SHV_COMBINATION_TYPE(3i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkPolicyServer\"`*"]
pub const SHV_COMBINATION_TYPE_ONE_OR_MORE_INFECTED: SHV_COMBINATION_TYPE = SHV_COMBINATION_TYPE(4i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkPolicyServer\"`*"]
pub const SHV_COMBINATION_TYPE_ONE_OR_MORE_TRANSITIONAL: SHV_COMBINATION_TYPE = SHV_COMBINATION_TYPE(5i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkPolicyServer\"`*"]
pub const SHV_COMBINATION_TYPE_ONE_OR_MORE_UNKNOWN: SHV_COMBINATION_TYPE = SHV_COMBINATION_TYPE(6i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkPolicyServer\"`*"]
pub const SHV_COMBINATION_TYPE_MAX: SHV_COMBINATION_TYPE = SHV_COMBINATION_TYPE(7i32);
impl ::core::marker::Copy for SHV_COMBINATION_TYPE {}
impl ::core::clone::Clone for SHV_COMBINATION_TYPE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for SHV_COMBINATION_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for SHV_COMBINATION_TYPE {
    type Abi = Self;
}
impl ::core::fmt::Debug for SHV_COMBINATION_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SHV_COMBINATION_TYPE").field(&self.0).finish()
    }
}
pub const SdoMachine: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xe9218ae7_9e91_11d1_bf60_0080c7846bc0);
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkPolicyServer\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct TEMPLATESPROPERTIES(pub i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkPolicyServer\"`*"]
pub const PROPERTY_TEMPLATES_POLICIES_TEMPLATES: TEMPLATESPROPERTIES = TEMPLATESPROPERTIES(1024i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkPolicyServer\"`*"]
pub const PROPERTY_TEMPLATES_PROFILES_TEMPLATES: TEMPLATESPROPERTIES = TEMPLATESPROPERTIES(1025i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkPolicyServer\"`*"]
pub const PROPERTY_TEMPLATES_PROFILES_COLLECTION: TEMPLATESPROPERTIES = TEMPLATESPROPERTIES(1026i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkPolicyServer\"`*"]
pub const PROPERTY_TEMPLATES_PROXYPOLICIES_TEMPLATES: TEMPLATESPROPERTIES = TEMPLATESPROPERTIES(1027i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkPolicyServer\"`*"]
pub const PROPERTY_TEMPLATES_PROXYPROFILES_TEMPLATES: TEMPLATESPROPERTIES = TEMPLATESPROPERTIES(1028i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkPolicyServer\"`*"]
pub const PROPERTY_TEMPLATES_PROXYPROFILES_COLLECTION: TEMPLATESPROPERTIES = TEMPLATESPROPERTIES(1029i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkPolicyServer\"`*"]
pub const PROPERTY_TEMPLATES_REMEDIATIONSERVERGROUPS_TEMPLATES: TEMPLATESPROPERTIES = TEMPLATESPROPERTIES(1030i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkPolicyServer\"`*"]
pub const PROPERTY_TEMPLATES_SHVTEMPLATES_TEMPLATES: TEMPLATESPROPERTIES = TEMPLATESPROPERTIES(1031i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkPolicyServer\"`*"]
pub const PROPERTY_TEMPLATES_CLIENTS_TEMPLATES: TEMPLATESPROPERTIES = TEMPLATESPROPERTIES(1032i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkPolicyServer\"`*"]
pub const PROPERTY_TEMPLATES_RADIUSSERVERS_TEMPLATES: TEMPLATESPROPERTIES = TEMPLATESPROPERTIES(1033i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkPolicyServer\"`*"]
pub const PROPERTY_TEMPLATES_SHAREDSECRETS_TEMPLATES: TEMPLATESPROPERTIES = TEMPLATESPROPERTIES(1034i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkPolicyServer\"`*"]
pub const PROPERTY_TEMPLATES_IPFILTERS_TEMPLATES: TEMPLATESPROPERTIES = TEMPLATESPROPERTIES(1035i32);
impl ::core::marker::Copy for TEMPLATESPROPERTIES {}
impl ::core::clone::Clone for TEMPLATESPROPERTIES {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for TEMPLATESPROPERTIES {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for TEMPLATESPROPERTIES {
    type Abi = Self;
}
impl ::core::fmt::Debug for TEMPLATESPROPERTIES {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("TEMPLATESPROPERTIES").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkPolicyServer\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct USERPROPERTIES(pub i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkPolicyServer\"`*"]
pub const PROPERTY_USER_CALLING_STATION_ID: USERPROPERTIES = USERPROPERTIES(1024i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkPolicyServer\"`*"]
pub const PROPERTY_USER_SAVED_CALLING_STATION_ID: USERPROPERTIES = USERPROPERTIES(1025i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkPolicyServer\"`*"]
pub const PROPERTY_USER_RADIUS_CALLBACK_NUMBER: USERPROPERTIES = USERPROPERTIES(1026i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkPolicyServer\"`*"]
pub const PROPERTY_USER_RADIUS_FRAMED_ROUTE: USERPROPERTIES = USERPROPERTIES(1027i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkPolicyServer\"`*"]
pub const PROPERTY_USER_RADIUS_FRAMED_IP_ADDRESS: USERPROPERTIES = USERPROPERTIES(1028i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkPolicyServer\"`*"]
pub const PROPERTY_USER_SAVED_RADIUS_CALLBACK_NUMBER: USERPROPERTIES = USERPROPERTIES(1029i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkPolicyServer\"`*"]
pub const PROPERTY_USER_SAVED_RADIUS_FRAMED_ROUTE: USERPROPERTIES = USERPROPERTIES(1030i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkPolicyServer\"`*"]
pub const PROPERTY_USER_SAVED_RADIUS_FRAMED_IP_ADDRESS: USERPROPERTIES = USERPROPERTIES(1031i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkPolicyServer\"`*"]
pub const PROPERTY_USER_ALLOW_DIALIN: USERPROPERTIES = USERPROPERTIES(1032i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkPolicyServer\"`*"]
pub const PROPERTY_USER_SERVICE_TYPE: USERPROPERTIES = USERPROPERTIES(1033i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkPolicyServer\"`*"]
pub const PROPERTY_USER_RADIUS_FRAMED_IPV6_ROUTE: USERPROPERTIES = USERPROPERTIES(1034i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkPolicyServer\"`*"]
pub const PROPERTY_USER_SAVED_RADIUS_FRAMED_IPV6_ROUTE: USERPROPERTIES = USERPROPERTIES(1035i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkPolicyServer\"`*"]
pub const PROPERTY_USER_RADIUS_FRAMED_INTERFACE_ID: USERPROPERTIES = USERPROPERTIES(1036i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkPolicyServer\"`*"]
pub const PROPERTY_USER_SAVED_RADIUS_FRAMED_INTERFACE_ID: USERPROPERTIES = USERPROPERTIES(1037i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkPolicyServer\"`*"]
pub const PROPERTY_USER_RADIUS_FRAMED_IPV6_PREFIX: USERPROPERTIES = USERPROPERTIES(1038i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkPolicyServer\"`*"]
pub const PROPERTY_USER_SAVED_RADIUS_FRAMED_IPV6_PREFIX: USERPROPERTIES = USERPROPERTIES(1039i32);
impl ::core::marker::Copy for USERPROPERTIES {}
impl ::core::clone::Clone for USERPROPERTIES {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for USERPROPERTIES {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for USERPROPERTIES {
    type Abi = Self;
}
impl ::core::fmt::Debug for USERPROPERTIES {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("USERPROPERTIES").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkPolicyServer\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct VENDORPROPERTIES(pub i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkPolicyServer\"`*"]
pub const PROPERTY_NAS_VENDOR_ID: VENDORPROPERTIES = VENDORPROPERTIES(1024i32);
impl ::core::marker::Copy for VENDORPROPERTIES {}
impl ::core::clone::Clone for VENDORPROPERTIES {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for VENDORPROPERTIES {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for VENDORPROPERTIES {
    type Abi = Self;
}
impl ::core::fmt::Debug for VENDORPROPERTIES {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("VENDORPROPERTIES").field(&self.0).finish()
    }
}
#[cfg(feature = "implement")]
::core::include!("impl.rs");

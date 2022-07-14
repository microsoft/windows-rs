#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`*"]
pub const ALLOW_NO_AUTH: u32 = 1u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`*"]
pub const ATADDRESSLEN: u32 = 32u32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct AUTH_VALIDATION_EX {
    pub Header: MPRAPI_OBJECT_HEADER,
    pub hRasConnection: super::super::Foundation::HANDLE,
    pub wszUserName: [u16; 257],
    pub wszLogonDomain: [u16; 16],
    pub AuthInfoSize: u32,
    pub AuthInfo: [u8; 1],
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for AUTH_VALIDATION_EX {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for AUTH_VALIDATION_EX {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for AUTH_VALIDATION_EX {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("AUTH_VALIDATION_EX").field("Header", &self.Header).field("hRasConnection", &self.hRasConnection).field("wszUserName", &self.wszUserName).field("wszLogonDomain", &self.wszLogonDomain).field("AuthInfoSize", &self.AuthInfoSize).field("AuthInfo", &self.AuthInfo).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for AUTH_VALIDATION_EX {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for AUTH_VALIDATION_EX {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<AUTH_VALIDATION_EX>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for AUTH_VALIDATION_EX {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for AUTH_VALIDATION_EX {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`*"]
pub const DO_NOT_ALLOW_NO_AUTH: u32 = 0u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`*"]
pub const ERROR_ACCESSING_TCPCFGDLL: u32 = 727u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`*"]
pub const ERROR_ACCT_DISABLED: u32 = 647u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`*"]
pub const ERROR_ACCT_EXPIRED: u32 = 708u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`*"]
pub const ERROR_ACTION_REQUIRED: u32 = 877u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`*"]
pub const ERROR_ALLOCATING_MEMORY: u32 = 664u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`*"]
pub const ERROR_ALREADY_DISCONNECTING: u32 = 617u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`*"]
pub const ERROR_ASYNC_REQUEST_PENDING: u32 = 616u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`*"]
pub const ERROR_AUTHENTICATION_FAILURE: u32 = 691u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`*"]
pub const ERROR_AUTH_INTERNAL: u32 = 645u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`*"]
pub const ERROR_AUTOMATIC_VPN_FAILED: u32 = 800u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`*"]
pub const ERROR_BAD_ADDRESS_SPECIFIED: u32 = 769u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`*"]
pub const ERROR_BAD_CALLBACK_NUMBER: u32 = 704u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`*"]
pub const ERROR_BAD_PHONE_NUMBER: u32 = 749u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`*"]
pub const ERROR_BAD_STRING: u32 = 637u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`*"]
pub const ERROR_BAD_USAGE_IN_INI_FILE: u32 = 669u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`*"]
pub const ERROR_BIPLEX_PORT_NOT_AVAILABLE: u32 = 712u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`*"]
pub const ERROR_BLOCKED: u32 = 775u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`*"]
pub const ERROR_BROADBAND_ACTIVE: u32 = 813u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`*"]
pub const ERROR_BROADBAND_NO_NIC: u32 = 814u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`*"]
pub const ERROR_BROADBAND_TIMEOUT: u32 = 815u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`*"]
pub const ERROR_BUFFER_INVALID: u32 = 610u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`*"]
pub const ERROR_BUFFER_TOO_SMALL: u32 = 603u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`*"]
pub const ERROR_BUNDLE_NOT_FOUND: u32 = 754u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`*"]
pub const ERROR_CANNOT_DELETE: u32 = 817u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`*"]
pub const ERROR_CANNOT_DO_CUSTOMDIAL: u32 = 755u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`*"]
pub const ERROR_CANNOT_FIND_PHONEBOOK_ENTRY: u32 = 623u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`*"]
pub const ERROR_CANNOT_GET_LANA: u32 = 639u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`*"]
pub const ERROR_CANNOT_INITIATE_MOBIKE_UPDATE: u32 = 844u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`*"]
pub const ERROR_CANNOT_LOAD_PHONEBOOK: u32 = 622u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`*"]
pub const ERROR_CANNOT_LOAD_STRING: u32 = 626u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`*"]
pub const ERROR_CANNOT_OPEN_PHONEBOOK: u32 = 621u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`*"]
pub const ERROR_CANNOT_PROJECT_CLIENT: u32 = 634u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`*"]
pub const ERROR_CANNOT_SET_PORT_INFO: u32 = 605u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`*"]
pub const ERROR_CANNOT_SHARE_CONNECTION: u32 = 763u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`*"]
pub const ERROR_CANNOT_USE_LOGON_CREDENTIALS: u32 = 739u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`*"]
pub const ERROR_CANNOT_WRITE_PHONEBOOK: u32 = 624u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`*"]
pub const ERROR_CERT_FOR_ENCRYPTION_NOT_FOUND: u32 = 781u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`*"]
pub const ERROR_CHANGING_PASSWORD: u32 = 709u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`*"]
pub const ERROR_CMD_TOO_LONG: u32 = 700u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`*"]
pub const ERROR_CONGESTION: u32 = 771u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`*"]
pub const ERROR_CONNECTING_DEVICE_NOT_FOUND: u32 = 797u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`*"]
pub const ERROR_CONNECTION_ALREADY_SHARED: u32 = 758u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`*"]
pub const ERROR_CONNECTION_REJECT: u32 = 770u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`*"]
pub const ERROR_CORRUPT_PHONEBOOK: u32 = 625u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`*"]
pub const ERROR_DCB_NOT_FOUND: u32 = 694u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`*"]
pub const ERROR_DEFAULTOFF_MACRO_NOT_FOUND: u32 = 656u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`*"]
pub const ERROR_DEVICENAME_NOT_FOUND: u32 = 659u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`*"]
pub const ERROR_DEVICENAME_TOO_LONG: u32 = 658u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`*"]
pub const ERROR_DEVICETYPE_DOES_NOT_EXIST: u32 = 609u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`*"]
pub const ERROR_DEVICE_COMPLIANCE: u32 = 875u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`*"]
pub const ERROR_DEVICE_DOES_NOT_EXIST: u32 = 608u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`*"]
pub const ERROR_DEVICE_NOT_READY: u32 = 666u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`*"]
pub const ERROR_DIAL_ALREADY_IN_PROGRESS: u32 = 756u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`*"]
pub const ERROR_DISCONNECTION: u32 = 628u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`*"]
pub const ERROR_DNSNAME_NOT_RESOLVABLE: u32 = 868u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`*"]
pub const ERROR_DONOTDISTURB: u32 = 776u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`*"]
pub const ERROR_EAPTLS_CACHE_CREDENTIALS_INVALID: u32 = 826u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`*"]
pub const ERROR_EAPTLS_PASSWD_INVALID: u32 = 869u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`*"]
pub const ERROR_EAPTLS_SCARD_CACHE_CREDENTIALS_INVALID: u32 = 847u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`*"]
pub const ERROR_EAP_METHOD_DOES_NOT_SUPPORT_SSO: u32 = 851u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`*"]
pub const ERROR_EAP_METHOD_NOT_INSTALLED: u32 = 850u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`*"]
pub const ERROR_EAP_METHOD_OPERATION_NOT_SUPPORTED: u32 = 852u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`*"]
pub const ERROR_EAP_SERVER_CERT_EXPIRED: u32 = 858u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`*"]
pub const ERROR_EAP_SERVER_CERT_INVALID: u32 = 857u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`*"]
pub const ERROR_EAP_SERVER_CERT_OTHER_ERROR: u32 = 860u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`*"]
pub const ERROR_EAP_SERVER_CERT_REVOKED: u32 = 859u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`*"]
pub const ERROR_EAP_SERVER_ROOT_CERT_INVALID: u32 = 865u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`*"]
pub const ERROR_EAP_SERVER_ROOT_CERT_NAME_REQUIRED: u32 = 866u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`*"]
pub const ERROR_EAP_SERVER_ROOT_CERT_NOT_FOUND: u32 = 864u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`*"]
pub const ERROR_EAP_USER_CERT_EXPIRED: u32 = 854u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`*"]
pub const ERROR_EAP_USER_CERT_INVALID: u32 = 853u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`*"]
pub const ERROR_EAP_USER_CERT_OTHER_ERROR: u32 = 856u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`*"]
pub const ERROR_EAP_USER_CERT_REVOKED: u32 = 855u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`*"]
pub const ERROR_EAP_USER_ROOT_CERT_EXPIRED: u32 = 863u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`*"]
pub const ERROR_EAP_USER_ROOT_CERT_INVALID: u32 = 862u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`*"]
pub const ERROR_EAP_USER_ROOT_CERT_NOT_FOUND: u32 = 861u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`*"]
pub const ERROR_EMPTY_INI_FILE: u32 = 690u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`*"]
pub const ERROR_EVENT_INVALID: u32 = 607u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`*"]
pub const ERROR_FAILED_CP_REQUIRED: u32 = 841u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`*"]
pub const ERROR_FAILED_TO_ENCRYPT: u32 = 768u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`*"]
pub const ERROR_FAST_USER_SWITCH: u32 = 831u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`*"]
pub const ERROR_FEATURE_DEPRECATED: u32 = 816u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`*"]
pub const ERROR_FILE_COULD_NOT_BE_OPENED: u32 = 657u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`*"]
pub const ERROR_FROM_DEVICE: u32 = 651u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`*"]
pub const ERROR_HANGUP_FAILED: u32 = 753u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`*"]
pub const ERROR_HARDWARE_FAILURE: u32 = 630u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`*"]
pub const ERROR_HIBERNATION: u32 = 832u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`*"]
pub const ERROR_IDLE_TIMEOUT: u32 = 828u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`*"]
pub const ERROR_IKEV2_PSK_INTERFACE_ALREADY_EXISTS: u32 = 870u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`*"]
pub const ERROR_INCOMPATIBLE: u32 = 772u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`*"]
pub const ERROR_INTERACTIVE_MODE: u32 = 703u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`*"]
pub const ERROR_INTERNAL_ADDRESS_FAILURE: u32 = 840u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`*"]
pub const ERROR_INVALID_AUTH_STATE: u32 = 705u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`*"]
pub const ERROR_INVALID_CALLBACK_NUMBER: u32 = 751u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`*"]
pub const ERROR_INVALID_COMPRESSION_SPECIFIED: u32 = 613u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`*"]
pub const ERROR_INVALID_DESTINATION_IP: u32 = 871u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`*"]
pub const ERROR_INVALID_FUNCTION_FOR_ENTRY: u32 = 780u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`*"]
pub const ERROR_INVALID_INTERFACE_CONFIG: u32 = 872u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`*"]
pub const ERROR_INVALID_MSCHAPV2_CONFIG: u32 = 805u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`*"]
pub const ERROR_INVALID_PEAP_COOKIE_ATTRIBUTES: u32 = 849u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`*"]
pub const ERROR_INVALID_PEAP_COOKIE_CONFIG: u32 = 803u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`*"]
pub const ERROR_INVALID_PEAP_COOKIE_USER: u32 = 804u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`*"]
pub const ERROR_INVALID_PORT_HANDLE: u32 = 601u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`*"]
pub const ERROR_INVALID_PREFERENCES: u32 = 846u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`*"]
pub const ERROR_INVALID_SERVER_CERT: u32 = 835u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`*"]
pub const ERROR_INVALID_SIZE: u32 = 632u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`*"]
pub const ERROR_INVALID_SMM: u32 = 745u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`*"]
pub const ERROR_INVALID_TUNNELID: u32 = 837u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`*"]
pub const ERROR_INVALID_VPNSTRATEGY: u32 = 825u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`*"]
pub const ERROR_IN_COMMAND: u32 = 681u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`*"]
pub const ERROR_IPSEC_SERVICE_STOPPED: u32 = 827u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`*"]
pub const ERROR_IPXCP_DIALOUT_ALREADY_ACTIVE: u32 = 726u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`*"]
pub const ERROR_IPXCP_NET_NUMBER_CONFLICT: u32 = 744u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`*"]
pub const ERROR_IPXCP_NO_DIALIN_CONFIGURED: u32 = 725u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`*"]
pub const ERROR_IPXCP_NO_DIALOUT_CONFIGURED: u32 = 724u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`*"]
pub const ERROR_IP_CONFIGURATION: u32 = 716u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`*"]
pub const ERROR_KEY_NOT_FOUND: u32 = 627u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`*"]
pub const ERROR_LINE_BUSY: u32 = 676u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`*"]
pub const ERROR_LINK_FAILURE: u32 = 829u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`*"]
pub const ERROR_MACRO_NOT_DEFINED: u32 = 654u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`*"]
pub const ERROR_MACRO_NOT_FOUND: u32 = 653u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`*"]
pub const ERROR_MESSAGE_MACRO_NOT_FOUND: u32 = 655u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`*"]
pub const ERROR_MOBIKE_DISABLED: u32 = 843u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`*"]
pub const ERROR_NAME_EXISTS_ON_NET: u32 = 642u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`*"]
pub const ERROR_NETBIOS_ERROR: u32 = 640u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`*"]
pub const ERROR_NOT_BINARY_MACRO: u32 = 693u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`*"]
pub const ERROR_NOT_NAP_CAPABLE: u32 = 836u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`*"]
pub const ERROR_NO_ACTIVE_ISDN_LINES: u32 = 713u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`*"]
pub const ERROR_NO_ANSWER: u32 = 678u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`*"]
pub const ERROR_NO_CARRIER: u32 = 679u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`*"]
pub const ERROR_NO_CERTIFICATE: u32 = 766u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`*"]
pub const ERROR_NO_COMMAND_FOUND: u32 = 661u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`*"]
pub const ERROR_NO_CONNECTION: u32 = 668u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`*"]
pub const ERROR_NO_DIALIN_PERMISSION: u32 = 649u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`*"]
pub const ERROR_NO_DIALTONE: u32 = 680u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`*"]
pub const ERROR_NO_DIFF_USER_AT_LOGON: u32 = 784u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`*"]
pub const ERROR_NO_EAPTLS_CERTIFICATE: u32 = 798u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`*"]
pub const ERROR_NO_ENDPOINTS: u32 = 620u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`*"]
pub const ERROR_NO_IP_ADDRESSES: u32 = 717u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`*"]
pub const ERROR_NO_IP_RAS_ADAPTER: u32 = 728u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`*"]
pub const ERROR_NO_ISDN_CHANNELS_AVAILABLE: u32 = 714u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`*"]
pub const ERROR_NO_LOCAL_ENCRYPTION: u32 = 741u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`*"]
pub const ERROR_NO_MAC_FOR_PORT: u32 = 747u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`*"]
pub const ERROR_NO_REG_CERT_AT_LOGON: u32 = 785u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`*"]
pub const ERROR_NO_REMOTE_ENCRYPTION: u32 = 742u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`*"]
pub const ERROR_NO_RESPONSES: u32 = 660u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`*"]
pub const ERROR_NO_SMART_CARD_READER: u32 = 764u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`*"]
pub const ERROR_NUMBERCHANGED: u32 = 773u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`*"]
pub const ERROR_OAKLEY_ATTRIB_FAIL: u32 = 788u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`*"]
pub const ERROR_OAKLEY_AUTH_FAIL: u32 = 787u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`*"]
pub const ERROR_OAKLEY_ERROR: u32 = 793u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`*"]
pub const ERROR_OAKLEY_GENERAL_PROCESSING: u32 = 789u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`*"]
pub const ERROR_OAKLEY_NO_CERT: u32 = 786u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`*"]
pub const ERROR_OAKLEY_NO_PEER_CERT: u32 = 790u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`*"]
pub const ERROR_OAKLEY_NO_POLICY: u32 = 791u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`*"]
pub const ERROR_OAKLEY_TIMED_OUT: u32 = 792u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`*"]
pub const ERROR_OUTOFORDER: u32 = 777u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`*"]
pub const ERROR_OUT_OF_BUFFERS: u32 = 614u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`*"]
pub const ERROR_OVERRUN: u32 = 710u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`*"]
pub const ERROR_PARTIAL_RESPONSE_LOOPING: u32 = 697u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`*"]
pub const ERROR_PASSWD_EXPIRED: u32 = 648u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`*"]
pub const ERROR_PEAP_CRYPTOBINDING_INVALID: u32 = 823u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`*"]
pub const ERROR_PEAP_CRYPTOBINDING_NOTRECEIVED: u32 = 824u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`*"]
pub const ERROR_PEAP_IDENTITY_MISMATCH: u32 = 867u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`*"]
pub const ERROR_PEAP_SERVER_REJECTED_CLIENT_TLV: u32 = 845u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`*"]
pub const ERROR_PHONE_NUMBER_TOO_LONG: u32 = 723u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`*"]
pub const ERROR_PLUGIN_NOT_INSTALLED: u32 = 876u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`*"]
pub const ERROR_PORT_ALREADY_OPEN: u32 = 602u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`*"]
pub const ERROR_PORT_DISCONNECTED: u32 = 619u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`*"]
pub const ERROR_PORT_NOT_AVAILABLE: u32 = 633u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`*"]
pub const ERROR_PORT_NOT_CONFIGURED: u32 = 665u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`*"]
pub const ERROR_PORT_NOT_CONNECTED: u32 = 606u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`*"]
pub const ERROR_PORT_NOT_FOUND: u32 = 615u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`*"]
pub const ERROR_PORT_NOT_OPEN: u32 = 618u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`*"]
pub const ERROR_PORT_OR_DEVICE: u32 = 692u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`*"]
pub const ERROR_PPP_CP_REJECTED: u32 = 733u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`*"]
pub const ERROR_PPP_INVALID_PACKET: u32 = 722u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`*"]
pub const ERROR_PPP_LCP_TERMINATED: u32 = 734u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`*"]
pub const ERROR_PPP_LOOPBACK_DETECTED: u32 = 737u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`*"]
pub const ERROR_PPP_NCP_TERMINATED: u32 = 736u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`*"]
pub const ERROR_PPP_NOT_CONVERGING: u32 = 732u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`*"]
pub const ERROR_PPP_NO_ADDRESS_ASSIGNED: u32 = 738u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`*"]
pub const ERROR_PPP_NO_PROTOCOLS_CONFIGURED: u32 = 720u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`*"]
pub const ERROR_PPP_NO_RESPONSE: u32 = 721u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`*"]
pub const ERROR_PPP_REMOTE_TERMINATED: u32 = 719u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`*"]
pub const ERROR_PPP_REQUIRED_ADDRESS_REJECTED: u32 = 735u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`*"]
pub const ERROR_PPP_TIMEOUT: u32 = 718u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`*"]
pub const ERROR_PROJECTION_NOT_COMPLETE: u32 = 730u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`*"]
pub const ERROR_PROTOCOL_ENGINE_DISABLED: u32 = 839u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`*"]
pub const ERROR_PROTOCOL_NOT_CONFIGURED: u32 = 731u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`*"]
pub const ERROR_RASAUTO_CANNOT_INITIALIZE: u32 = 757u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`*"]
pub const ERROR_RASMAN_CANNOT_INITIALIZE: u32 = 711u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`*"]
pub const ERROR_RASMAN_SERVICE_STOPPED: u32 = 834u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`*"]
pub const ERROR_RASQEC_CONN_DOESNOTEXIST: u32 = 821u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`*"]
pub const ERROR_RASQEC_NAPAGENT_NOT_CONNECTED: u32 = 820u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`*"]
pub const ERROR_RASQEC_NAPAGENT_NOT_ENABLED: u32 = 819u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`*"]
pub const ERROR_RASQEC_RESOURCE_CREATION_FAILED: u32 = 818u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`*"]
pub const ERROR_RASQEC_TIMEOUT: u32 = 822u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`*"]
pub const ERROR_READING_DEFAULTOFF: u32 = 689u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`*"]
pub const ERROR_READING_DEVICENAME: u32 = 672u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`*"]
pub const ERROR_READING_DEVICETYPE: u32 = 671u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`*"]
pub const ERROR_READING_INI_FILE: u32 = 667u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`*"]
pub const ERROR_READING_MAXCARRIERBPS: u32 = 675u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`*"]
pub const ERROR_READING_MAXCONNECTBPS: u32 = 674u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`*"]
pub const ERROR_READING_SCARD: u32 = 802u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`*"]
pub const ERROR_READING_SECTIONNAME: u32 = 670u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`*"]
pub const ERROR_READING_USAGE: u32 = 673u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`*"]
pub const ERROR_RECV_BUF_FULL: u32 = 699u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`*"]
pub const ERROR_REMOTE_DISCONNECTION: u32 = 629u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`*"]
pub const ERROR_REMOTE_REQUIRES_ENCRYPTION: u32 = 743u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`*"]
pub const ERROR_REQUEST_TIMEOUT: u32 = 638u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`*"]
pub const ERROR_RESTRICTED_LOGON_HOURS: u32 = 646u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`*"]
pub const ERROR_ROUTE_NOT_ALLOCATED: u32 = 612u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`*"]
pub const ERROR_ROUTE_NOT_AVAILABLE: u32 = 611u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`*"]
pub const ERROR_SCRIPT_SYNTAX: u32 = 752u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`*"]
pub const ERROR_SERVER_GENERAL_NET_FAILURE: u32 = 643u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`*"]
pub const ERROR_SERVER_NOT_RESPONDING: u32 = 650u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`*"]
pub const ERROR_SERVER_OUT_OF_RESOURCES: u32 = 641u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`*"]
pub const ERROR_SERVER_POLICY: u32 = 812u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`*"]
pub const ERROR_SHARE_CONNECTION_FAILED: u32 = 761u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`*"]
pub const ERROR_SHARING_ADDRESS_EXISTS: u32 = 765u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`*"]
pub const ERROR_SHARING_CHANGE_FAILED: u32 = 759u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`*"]
pub const ERROR_SHARING_HOST_ADDRESS_CONFLICT: u32 = 799u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`*"]
pub const ERROR_SHARING_MULTIPLE_ADDRESSES: u32 = 767u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`*"]
pub const ERROR_SHARING_NO_PRIVATE_LAN: u32 = 783u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`*"]
pub const ERROR_SHARING_PRIVATE_INSTALL: u32 = 762u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`*"]
pub const ERROR_SHARING_ROUTER_INSTALL: u32 = 760u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`*"]
pub const ERROR_SHARING_RRAS_CONFLICT: u32 = 782u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`*"]
pub const ERROR_SLIP_REQUIRES_IP: u32 = 729u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`*"]
pub const ERROR_SMART_CARD_REQUIRED: u32 = 779u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`*"]
pub const ERROR_SMM_TIMEOUT: u32 = 748u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`*"]
pub const ERROR_SMM_UNINITIALIZED: u32 = 746u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`*"]
pub const ERROR_SSO_CERT_MISSING: u32 = 874u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`*"]
pub const ERROR_SSTP_COOKIE_SET_FAILURE: u32 = 848u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`*"]
pub const ERROR_STATE_MACHINES_ALREADY_STARTED: u32 = 696u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`*"]
pub const ERROR_STATE_MACHINES_NOT_STARTED: u32 = 695u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`*"]
pub const ERROR_SYSTEM_SUSPENDED: u32 = 833u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`*"]
pub const ERROR_TAPI_CONFIGURATION: u32 = 740u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`*"]
pub const ERROR_TEMPFAILURE: u32 = 774u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`*"]
pub const ERROR_TOO_MANY_LINE_ERRORS: u32 = 715u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`*"]
pub const ERROR_TS_UNACCEPTABLE: u32 = 842u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`*"]
pub const ERROR_UNABLE_TO_AUTHENTICATE_SERVER: u32 = 778u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`*"]
pub const ERROR_UNEXPECTED_RESPONSE: u32 = 702u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`*"]
pub const ERROR_UNKNOWN: u32 = 635u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`*"]
pub const ERROR_UNKNOWN_DEVICE_TYPE: u32 = 663u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`*"]
pub const ERROR_UNKNOWN_FRAMED_PROTOCOL: u32 = 794u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`*"]
pub const ERROR_UNKNOWN_RESPONSE_KEY: u32 = 698u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`*"]
pub const ERROR_UNKNOWN_SERVICE_TYPE: u32 = 796u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`*"]
pub const ERROR_UNRECOGNIZED_RESPONSE: u32 = 652u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`*"]
pub const ERROR_UNSUPPORTED_BPS: u32 = 701u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`*"]
pub const ERROR_UPDATECONNECTION_REQUEST_IN_PROCESS: u32 = 838u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`*"]
pub const ERROR_USER_DISCONNECTION: u32 = 631u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`*"]
pub const ERROR_USER_LOGOFF: u32 = 830u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`*"]
pub const ERROR_VALIDATING_SERVER_CERT: u32 = 801u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`*"]
pub const ERROR_VOICE_ANSWER: u32 = 677u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`*"]
pub const ERROR_VPN_BAD_CERT: u32 = 810u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`*"]
pub const ERROR_VPN_BAD_PSK: u32 = 811u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`*"]
pub const ERROR_VPN_DISCONNECT: u32 = 807u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`*"]
pub const ERROR_VPN_GRE_BLOCKED: u32 = 806u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`*"]
pub const ERROR_VPN_PLUGIN_GENERIC: u32 = 873u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`*"]
pub const ERROR_VPN_REFUSED: u32 = 808u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`*"]
pub const ERROR_VPN_TIMEOUT: u32 = 809u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`*"]
pub const ERROR_WRITING_DEFAULTOFF: u32 = 688u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`*"]
pub const ERROR_WRITING_DEVICENAME: u32 = 684u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`*"]
pub const ERROR_WRITING_DEVICETYPE: u32 = 683u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`*"]
pub const ERROR_WRITING_INITBPS: u32 = 706u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`*"]
pub const ERROR_WRITING_MAXCARRIERBPS: u32 = 686u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`*"]
pub const ERROR_WRITING_MAXCONNECTBPS: u32 = 685u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`*"]
pub const ERROR_WRITING_SECTIONNAME: u32 = 682u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`*"]
pub const ERROR_WRITING_USAGE: u32 = 687u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`*"]
pub const ERROR_WRONG_DEVICE_ATTACHED: u32 = 636u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`*"]
pub const ERROR_WRONG_INFO_SPECIFIED: u32 = 604u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`*"]
pub const ERROR_WRONG_KEY_SPECIFIED: u32 = 662u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`*"]
pub const ERROR_WRONG_MODULE: u32 = 750u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`*"]
pub const ERROR_WRONG_TUNNEL_TYPE: u32 = 795u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`*"]
pub const ERROR_X25_DIAGNOSTIC: u32 = 707u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`*"]
pub const ET_None: u32 = 0u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`*"]
pub const ET_Optional: u32 = 3u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`*"]
pub const ET_Require: u32 = 1u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`*"]
pub const ET_RequireMax: u32 = 2u32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`*"]
pub struct GRE_CONFIG_PARAMS0 {
    pub dwNumPorts: u32,
    pub dwPortFlags: u32,
}
impl ::core::marker::Copy for GRE_CONFIG_PARAMS0 {}
impl ::core::clone::Clone for GRE_CONFIG_PARAMS0 {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for GRE_CONFIG_PARAMS0 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("GRE_CONFIG_PARAMS0").field("dwNumPorts", &self.dwNumPorts).field("dwPortFlags", &self.dwPortFlags).finish()
    }
}
unsafe impl ::windows::core::Abi for GRE_CONFIG_PARAMS0 {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for GRE_CONFIG_PARAMS0 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<GRE_CONFIG_PARAMS0>()) == 0 }
    }
}
impl ::core::cmp::Eq for GRE_CONFIG_PARAMS0 {}
impl ::core::default::Default for GRE_CONFIG_PARAMS0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct HRASCONN(pub isize);
impl HRASCONN {
    pub fn is_invalid(&self) -> bool {
        self.0 == -1 || self.0 == 0
    }
}
impl ::core::default::Default for HRASCONN {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::clone::Clone for HRASCONN {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::marker::Copy for HRASCONN {}
impl ::core::fmt::Debug for HRASCONN {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("HRASCONN").field(&self.0).finish()
    }
}
impl ::core::convert::From<::core::option::Option<HRASCONN>> for HRASCONN {
    fn from(optional: ::core::option::Option<HRASCONN>) -> HRASCONN {
        optional.unwrap_or_default()
    }
}
unsafe impl ::windows::core::Abi for HRASCONN {
    type Abi = Self;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`, `\"Win32_Foundation\"`, `\"Win32_Security_Cryptography\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography"))]
pub struct IKEV2_CONFIG_PARAMS {
    pub dwNumPorts: u32,
    pub dwPortFlags: u32,
    pub dwTunnelConfigParamFlags: u32,
    pub TunnelConfigParams: IKEV2_TUNNEL_CONFIG_PARAMS4,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography"))]
impl ::core::marker::Copy for IKEV2_CONFIG_PARAMS {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography"))]
impl ::core::clone::Clone for IKEV2_CONFIG_PARAMS {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography"))]
impl ::core::fmt::Debug for IKEV2_CONFIG_PARAMS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("IKEV2_CONFIG_PARAMS").field("dwNumPorts", &self.dwNumPorts).field("dwPortFlags", &self.dwPortFlags).field("dwTunnelConfigParamFlags", &self.dwTunnelConfigParamFlags).field("TunnelConfigParams", &self.TunnelConfigParams).finish()
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography"))]
unsafe impl ::windows::core::Abi for IKEV2_CONFIG_PARAMS {
    type Abi = Self;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography"))]
impl ::core::cmp::PartialEq for IKEV2_CONFIG_PARAMS {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<IKEV2_CONFIG_PARAMS>()) == 0 }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography"))]
impl ::core::cmp::Eq for IKEV2_CONFIG_PARAMS {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography"))]
impl ::core::default::Default for IKEV2_CONFIG_PARAMS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct IKEV2_ID_PAYLOAD_TYPE(pub i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`*"]
pub const IKEV2_ID_PAYLOAD_TYPE_INVALID: IKEV2_ID_PAYLOAD_TYPE = IKEV2_ID_PAYLOAD_TYPE(0i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`*"]
pub const IKEV2_ID_PAYLOAD_TYPE_IPV4_ADDR: IKEV2_ID_PAYLOAD_TYPE = IKEV2_ID_PAYLOAD_TYPE(1i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`*"]
pub const IKEV2_ID_PAYLOAD_TYPE_FQDN: IKEV2_ID_PAYLOAD_TYPE = IKEV2_ID_PAYLOAD_TYPE(2i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`*"]
pub const IKEV2_ID_PAYLOAD_TYPE_RFC822_ADDR: IKEV2_ID_PAYLOAD_TYPE = IKEV2_ID_PAYLOAD_TYPE(3i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`*"]
pub const IKEV2_ID_PAYLOAD_TYPE_RESERVED1: IKEV2_ID_PAYLOAD_TYPE = IKEV2_ID_PAYLOAD_TYPE(4i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`*"]
pub const IKEV2_ID_PAYLOAD_TYPE_ID_IPV6_ADDR: IKEV2_ID_PAYLOAD_TYPE = IKEV2_ID_PAYLOAD_TYPE(5i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`*"]
pub const IKEV2_ID_PAYLOAD_TYPE_RESERVED2: IKEV2_ID_PAYLOAD_TYPE = IKEV2_ID_PAYLOAD_TYPE(6i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`*"]
pub const IKEV2_ID_PAYLOAD_TYPE_RESERVED3: IKEV2_ID_PAYLOAD_TYPE = IKEV2_ID_PAYLOAD_TYPE(7i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`*"]
pub const IKEV2_ID_PAYLOAD_TYPE_RESERVED4: IKEV2_ID_PAYLOAD_TYPE = IKEV2_ID_PAYLOAD_TYPE(8i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`*"]
pub const IKEV2_ID_PAYLOAD_TYPE_DER_ASN1_DN: IKEV2_ID_PAYLOAD_TYPE = IKEV2_ID_PAYLOAD_TYPE(9i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`*"]
pub const IKEV2_ID_PAYLOAD_TYPE_DER_ASN1_GN: IKEV2_ID_PAYLOAD_TYPE = IKEV2_ID_PAYLOAD_TYPE(10i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`*"]
pub const IKEV2_ID_PAYLOAD_TYPE_KEY_ID: IKEV2_ID_PAYLOAD_TYPE = IKEV2_ID_PAYLOAD_TYPE(11i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`*"]
pub const IKEV2_ID_PAYLOAD_TYPE_MAX: IKEV2_ID_PAYLOAD_TYPE = IKEV2_ID_PAYLOAD_TYPE(12i32);
impl ::core::marker::Copy for IKEV2_ID_PAYLOAD_TYPE {}
impl ::core::clone::Clone for IKEV2_ID_PAYLOAD_TYPE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for IKEV2_ID_PAYLOAD_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for IKEV2_ID_PAYLOAD_TYPE {
    type Abi = Self;
}
impl ::core::fmt::Debug for IKEV2_ID_PAYLOAD_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IKEV2_ID_PAYLOAD_TYPE").field(&self.0).finish()
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`*"]
pub struct IKEV2_PROJECTION_INFO {
    pub dwIPv4NegotiationError: u32,
    pub wszAddress: [u16; 16],
    pub wszRemoteAddress: [u16; 16],
    pub IPv4SubInterfaceIndex: u64,
    pub dwIPv6NegotiationError: u32,
    pub bInterfaceIdentifier: [u8; 8],
    pub bRemoteInterfaceIdentifier: [u8; 8],
    pub bPrefix: [u8; 8],
    pub dwPrefixLength: u32,
    pub IPv6SubInterfaceIndex: u64,
    pub dwOptions: u32,
    pub dwAuthenticationProtocol: u32,
    pub dwEapTypeId: u32,
    pub dwCompressionAlgorithm: u32,
    pub dwEncryptionMethod: u32,
}
impl ::core::marker::Copy for IKEV2_PROJECTION_INFO {}
impl ::core::clone::Clone for IKEV2_PROJECTION_INFO {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for IKEV2_PROJECTION_INFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("IKEV2_PROJECTION_INFO")
            .field("dwIPv4NegotiationError", &self.dwIPv4NegotiationError)
            .field("wszAddress", &self.wszAddress)
            .field("wszRemoteAddress", &self.wszRemoteAddress)
            .field("IPv4SubInterfaceIndex", &self.IPv4SubInterfaceIndex)
            .field("dwIPv6NegotiationError", &self.dwIPv6NegotiationError)
            .field("bInterfaceIdentifier", &self.bInterfaceIdentifier)
            .field("bRemoteInterfaceIdentifier", &self.bRemoteInterfaceIdentifier)
            .field("bPrefix", &self.bPrefix)
            .field("dwPrefixLength", &self.dwPrefixLength)
            .field("IPv6SubInterfaceIndex", &self.IPv6SubInterfaceIndex)
            .field("dwOptions", &self.dwOptions)
            .field("dwAuthenticationProtocol", &self.dwAuthenticationProtocol)
            .field("dwEapTypeId", &self.dwEapTypeId)
            .field("dwCompressionAlgorithm", &self.dwCompressionAlgorithm)
            .field("dwEncryptionMethod", &self.dwEncryptionMethod)
            .finish()
    }
}
unsafe impl ::windows::core::Abi for IKEV2_PROJECTION_INFO {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for IKEV2_PROJECTION_INFO {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<IKEV2_PROJECTION_INFO>()) == 0 }
    }
}
impl ::core::cmp::Eq for IKEV2_PROJECTION_INFO {}
impl ::core::default::Default for IKEV2_PROJECTION_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`*"]
pub struct IKEV2_PROJECTION_INFO2 {
    pub dwIPv4NegotiationError: u32,
    pub wszAddress: [u16; 16],
    pub wszRemoteAddress: [u16; 16],
    pub IPv4SubInterfaceIndex: u64,
    pub dwIPv6NegotiationError: u32,
    pub bInterfaceIdentifier: [u8; 8],
    pub bRemoteInterfaceIdentifier: [u8; 8],
    pub bPrefix: [u8; 8],
    pub dwPrefixLength: u32,
    pub IPv6SubInterfaceIndex: u64,
    pub dwOptions: u32,
    pub dwAuthenticationProtocol: u32,
    pub dwEapTypeId: u32,
    pub dwEmbeddedEAPTypeId: u32,
    pub dwCompressionAlgorithm: u32,
    pub dwEncryptionMethod: u32,
}
impl ::core::marker::Copy for IKEV2_PROJECTION_INFO2 {}
impl ::core::clone::Clone for IKEV2_PROJECTION_INFO2 {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for IKEV2_PROJECTION_INFO2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("IKEV2_PROJECTION_INFO2")
            .field("dwIPv4NegotiationError", &self.dwIPv4NegotiationError)
            .field("wszAddress", &self.wszAddress)
            .field("wszRemoteAddress", &self.wszRemoteAddress)
            .field("IPv4SubInterfaceIndex", &self.IPv4SubInterfaceIndex)
            .field("dwIPv6NegotiationError", &self.dwIPv6NegotiationError)
            .field("bInterfaceIdentifier", &self.bInterfaceIdentifier)
            .field("bRemoteInterfaceIdentifier", &self.bRemoteInterfaceIdentifier)
            .field("bPrefix", &self.bPrefix)
            .field("dwPrefixLength", &self.dwPrefixLength)
            .field("IPv6SubInterfaceIndex", &self.IPv6SubInterfaceIndex)
            .field("dwOptions", &self.dwOptions)
            .field("dwAuthenticationProtocol", &self.dwAuthenticationProtocol)
            .field("dwEapTypeId", &self.dwEapTypeId)
            .field("dwEmbeddedEAPTypeId", &self.dwEmbeddedEAPTypeId)
            .field("dwCompressionAlgorithm", &self.dwCompressionAlgorithm)
            .field("dwEncryptionMethod", &self.dwEncryptionMethod)
            .finish()
    }
}
unsafe impl ::windows::core::Abi for IKEV2_PROJECTION_INFO2 {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for IKEV2_PROJECTION_INFO2 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<IKEV2_PROJECTION_INFO2>()) == 0 }
    }
}
impl ::core::cmp::Eq for IKEV2_PROJECTION_INFO2 {}
impl ::core::default::Default for IKEV2_PROJECTION_INFO2 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`, `\"Win32_Security_Cryptography\"`*"]
#[cfg(feature = "Win32_Security_Cryptography")]
pub struct IKEV2_TUNNEL_CONFIG_PARAMS2 {
    pub dwIdleTimeout: u32,
    pub dwNetworkBlackoutTime: u32,
    pub dwSaLifeTime: u32,
    pub dwSaDataSizeForRenegotiation: u32,
    pub dwConfigOptions: u32,
    pub dwTotalCertificates: u32,
    pub certificateNames: *mut super::super::Security::Cryptography::CRYPTOAPI_BLOB,
    pub machineCertificateName: super::super::Security::Cryptography::CRYPTOAPI_BLOB,
    pub dwEncryptionType: u32,
    pub customPolicy: *mut ROUTER_CUSTOM_IKEv2_POLICY0,
}
#[cfg(feature = "Win32_Security_Cryptography")]
impl ::core::marker::Copy for IKEV2_TUNNEL_CONFIG_PARAMS2 {}
#[cfg(feature = "Win32_Security_Cryptography")]
impl ::core::clone::Clone for IKEV2_TUNNEL_CONFIG_PARAMS2 {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Security_Cryptography")]
impl ::core::fmt::Debug for IKEV2_TUNNEL_CONFIG_PARAMS2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("IKEV2_TUNNEL_CONFIG_PARAMS2")
            .field("dwIdleTimeout", &self.dwIdleTimeout)
            .field("dwNetworkBlackoutTime", &self.dwNetworkBlackoutTime)
            .field("dwSaLifeTime", &self.dwSaLifeTime)
            .field("dwSaDataSizeForRenegotiation", &self.dwSaDataSizeForRenegotiation)
            .field("dwConfigOptions", &self.dwConfigOptions)
            .field("dwTotalCertificates", &self.dwTotalCertificates)
            .field("certificateNames", &self.certificateNames)
            .field("machineCertificateName", &self.machineCertificateName)
            .field("dwEncryptionType", &self.dwEncryptionType)
            .field("customPolicy", &self.customPolicy)
            .finish()
    }
}
#[cfg(feature = "Win32_Security_Cryptography")]
unsafe impl ::windows::core::Abi for IKEV2_TUNNEL_CONFIG_PARAMS2 {
    type Abi = Self;
}
#[cfg(feature = "Win32_Security_Cryptography")]
impl ::core::cmp::PartialEq for IKEV2_TUNNEL_CONFIG_PARAMS2 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<IKEV2_TUNNEL_CONFIG_PARAMS2>()) == 0 }
    }
}
#[cfg(feature = "Win32_Security_Cryptography")]
impl ::core::cmp::Eq for IKEV2_TUNNEL_CONFIG_PARAMS2 {}
#[cfg(feature = "Win32_Security_Cryptography")]
impl ::core::default::Default for IKEV2_TUNNEL_CONFIG_PARAMS2 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`, `\"Win32_Foundation\"`, `\"Win32_Security_Cryptography\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography"))]
pub struct IKEV2_TUNNEL_CONFIG_PARAMS3 {
    pub dwIdleTimeout: u32,
    pub dwNetworkBlackoutTime: u32,
    pub dwSaLifeTime: u32,
    pub dwSaDataSizeForRenegotiation: u32,
    pub dwConfigOptions: u32,
    pub dwTotalCertificates: u32,
    pub certificateNames: *mut super::super::Security::Cryptography::CRYPTOAPI_BLOB,
    pub machineCertificateName: super::super::Security::Cryptography::CRYPTOAPI_BLOB,
    pub dwEncryptionType: u32,
    pub customPolicy: *mut ROUTER_CUSTOM_IKEv2_POLICY0,
    pub dwTotalEkus: u32,
    pub certificateEKUs: *mut MPR_CERT_EKU,
    pub machineCertificateHash: super::super::Security::Cryptography::CRYPTOAPI_BLOB,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography"))]
impl ::core::marker::Copy for IKEV2_TUNNEL_CONFIG_PARAMS3 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography"))]
impl ::core::clone::Clone for IKEV2_TUNNEL_CONFIG_PARAMS3 {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography"))]
impl ::core::fmt::Debug for IKEV2_TUNNEL_CONFIG_PARAMS3 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("IKEV2_TUNNEL_CONFIG_PARAMS3")
            .field("dwIdleTimeout", &self.dwIdleTimeout)
            .field("dwNetworkBlackoutTime", &self.dwNetworkBlackoutTime)
            .field("dwSaLifeTime", &self.dwSaLifeTime)
            .field("dwSaDataSizeForRenegotiation", &self.dwSaDataSizeForRenegotiation)
            .field("dwConfigOptions", &self.dwConfigOptions)
            .field("dwTotalCertificates", &self.dwTotalCertificates)
            .field("certificateNames", &self.certificateNames)
            .field("machineCertificateName", &self.machineCertificateName)
            .field("dwEncryptionType", &self.dwEncryptionType)
            .field("customPolicy", &self.customPolicy)
            .field("dwTotalEkus", &self.dwTotalEkus)
            .field("certificateEKUs", &self.certificateEKUs)
            .field("machineCertificateHash", &self.machineCertificateHash)
            .finish()
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography"))]
unsafe impl ::windows::core::Abi for IKEV2_TUNNEL_CONFIG_PARAMS3 {
    type Abi = Self;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography"))]
impl ::core::cmp::PartialEq for IKEV2_TUNNEL_CONFIG_PARAMS3 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<IKEV2_TUNNEL_CONFIG_PARAMS3>()) == 0 }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography"))]
impl ::core::cmp::Eq for IKEV2_TUNNEL_CONFIG_PARAMS3 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography"))]
impl ::core::default::Default for IKEV2_TUNNEL_CONFIG_PARAMS3 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`, `\"Win32_Foundation\"`, `\"Win32_Security_Cryptography\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography"))]
pub struct IKEV2_TUNNEL_CONFIG_PARAMS4 {
    pub dwIdleTimeout: u32,
    pub dwNetworkBlackoutTime: u32,
    pub dwSaLifeTime: u32,
    pub dwSaDataSizeForRenegotiation: u32,
    pub dwConfigOptions: u32,
    pub dwTotalCertificates: u32,
    pub certificateNames: *mut super::super::Security::Cryptography::CRYPTOAPI_BLOB,
    pub machineCertificateName: super::super::Security::Cryptography::CRYPTOAPI_BLOB,
    pub dwEncryptionType: u32,
    pub customPolicy: *mut ROUTER_CUSTOM_IKEv2_POLICY0,
    pub dwTotalEkus: u32,
    pub certificateEKUs: *mut MPR_CERT_EKU,
    pub machineCertificateHash: super::super::Security::Cryptography::CRYPTOAPI_BLOB,
    pub dwMmSaLifeTime: u32,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography"))]
impl ::core::marker::Copy for IKEV2_TUNNEL_CONFIG_PARAMS4 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography"))]
impl ::core::clone::Clone for IKEV2_TUNNEL_CONFIG_PARAMS4 {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography"))]
impl ::core::fmt::Debug for IKEV2_TUNNEL_CONFIG_PARAMS4 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("IKEV2_TUNNEL_CONFIG_PARAMS4")
            .field("dwIdleTimeout", &self.dwIdleTimeout)
            .field("dwNetworkBlackoutTime", &self.dwNetworkBlackoutTime)
            .field("dwSaLifeTime", &self.dwSaLifeTime)
            .field("dwSaDataSizeForRenegotiation", &self.dwSaDataSizeForRenegotiation)
            .field("dwConfigOptions", &self.dwConfigOptions)
            .field("dwTotalCertificates", &self.dwTotalCertificates)
            .field("certificateNames", &self.certificateNames)
            .field("machineCertificateName", &self.machineCertificateName)
            .field("dwEncryptionType", &self.dwEncryptionType)
            .field("customPolicy", &self.customPolicy)
            .field("dwTotalEkus", &self.dwTotalEkus)
            .field("certificateEKUs", &self.certificateEKUs)
            .field("machineCertificateHash", &self.machineCertificateHash)
            .field("dwMmSaLifeTime", &self.dwMmSaLifeTime)
            .finish()
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography"))]
unsafe impl ::windows::core::Abi for IKEV2_TUNNEL_CONFIG_PARAMS4 {
    type Abi = Self;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography"))]
impl ::core::cmp::PartialEq for IKEV2_TUNNEL_CONFIG_PARAMS4 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<IKEV2_TUNNEL_CONFIG_PARAMS4>()) == 0 }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography"))]
impl ::core::cmp::Eq for IKEV2_TUNNEL_CONFIG_PARAMS4 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography"))]
impl ::core::default::Default for IKEV2_TUNNEL_CONFIG_PARAMS4 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`*"]
pub const IPADDRESSLEN: u32 = 15u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`*"]
pub const IPV6_ADDRESS_LEN_IN_BYTES: u32 = 16u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`*"]
pub const IPXADDRESSLEN: u32 = 22u32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`*"]
pub struct L2TP_CONFIG_PARAMS0 {
    pub dwNumPorts: u32,
    pub dwPortFlags: u32,
}
impl ::core::marker::Copy for L2TP_CONFIG_PARAMS0 {}
impl ::core::clone::Clone for L2TP_CONFIG_PARAMS0 {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for L2TP_CONFIG_PARAMS0 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("L2TP_CONFIG_PARAMS0").field("dwNumPorts", &self.dwNumPorts).field("dwPortFlags", &self.dwPortFlags).finish()
    }
}
unsafe impl ::windows::core::Abi for L2TP_CONFIG_PARAMS0 {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for L2TP_CONFIG_PARAMS0 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<L2TP_CONFIG_PARAMS0>()) == 0 }
    }
}
impl ::core::cmp::Eq for L2TP_CONFIG_PARAMS0 {}
impl ::core::default::Default for L2TP_CONFIG_PARAMS0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`*"]
pub struct L2TP_CONFIG_PARAMS1 {
    pub dwNumPorts: u32,
    pub dwPortFlags: u32,
    pub dwTunnelConfigParamFlags: u32,
    pub TunnelConfigParams: L2TP_TUNNEL_CONFIG_PARAMS2,
}
impl ::core::marker::Copy for L2TP_CONFIG_PARAMS1 {}
impl ::core::clone::Clone for L2TP_CONFIG_PARAMS1 {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for L2TP_CONFIG_PARAMS1 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("L2TP_CONFIG_PARAMS1").field("dwNumPorts", &self.dwNumPorts).field("dwPortFlags", &self.dwPortFlags).field("dwTunnelConfigParamFlags", &self.dwTunnelConfigParamFlags).field("TunnelConfigParams", &self.TunnelConfigParams).finish()
    }
}
unsafe impl ::windows::core::Abi for L2TP_CONFIG_PARAMS1 {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for L2TP_CONFIG_PARAMS1 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<L2TP_CONFIG_PARAMS1>()) == 0 }
    }
}
impl ::core::cmp::Eq for L2TP_CONFIG_PARAMS1 {}
impl ::core::default::Default for L2TP_CONFIG_PARAMS1 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`*"]
pub struct L2TP_TUNNEL_CONFIG_PARAMS1 {
    pub dwIdleTimeout: u32,
    pub dwEncryptionType: u32,
    pub dwSaLifeTime: u32,
    pub dwSaDataSizeForRenegotiation: u32,
    pub customPolicy: *mut ROUTER_CUSTOM_IKEv2_POLICY0,
}
impl ::core::marker::Copy for L2TP_TUNNEL_CONFIG_PARAMS1 {}
impl ::core::clone::Clone for L2TP_TUNNEL_CONFIG_PARAMS1 {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for L2TP_TUNNEL_CONFIG_PARAMS1 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("L2TP_TUNNEL_CONFIG_PARAMS1").field("dwIdleTimeout", &self.dwIdleTimeout).field("dwEncryptionType", &self.dwEncryptionType).field("dwSaLifeTime", &self.dwSaLifeTime).field("dwSaDataSizeForRenegotiation", &self.dwSaDataSizeForRenegotiation).field("customPolicy", &self.customPolicy).finish()
    }
}
unsafe impl ::windows::core::Abi for L2TP_TUNNEL_CONFIG_PARAMS1 {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for L2TP_TUNNEL_CONFIG_PARAMS1 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<L2TP_TUNNEL_CONFIG_PARAMS1>()) == 0 }
    }
}
impl ::core::cmp::Eq for L2TP_TUNNEL_CONFIG_PARAMS1 {}
impl ::core::default::Default for L2TP_TUNNEL_CONFIG_PARAMS1 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`*"]
pub struct L2TP_TUNNEL_CONFIG_PARAMS2 {
    pub dwIdleTimeout: u32,
    pub dwEncryptionType: u32,
    pub dwSaLifeTime: u32,
    pub dwSaDataSizeForRenegotiation: u32,
    pub customPolicy: *mut ROUTER_CUSTOM_IKEv2_POLICY0,
    pub dwMmSaLifeTime: u32,
}
impl ::core::marker::Copy for L2TP_TUNNEL_CONFIG_PARAMS2 {}
impl ::core::clone::Clone for L2TP_TUNNEL_CONFIG_PARAMS2 {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for L2TP_TUNNEL_CONFIG_PARAMS2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("L2TP_TUNNEL_CONFIG_PARAMS2").field("dwIdleTimeout", &self.dwIdleTimeout).field("dwEncryptionType", &self.dwEncryptionType).field("dwSaLifeTime", &self.dwSaLifeTime).field("dwSaDataSizeForRenegotiation", &self.dwSaDataSizeForRenegotiation).field("customPolicy", &self.customPolicy).field("dwMmSaLifeTime", &self.dwMmSaLifeTime).finish()
    }
}
unsafe impl ::windows::core::Abi for L2TP_TUNNEL_CONFIG_PARAMS2 {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for L2TP_TUNNEL_CONFIG_PARAMS2 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<L2TP_TUNNEL_CONFIG_PARAMS2>()) == 0 }
    }
}
impl ::core::cmp::Eq for L2TP_TUNNEL_CONFIG_PARAMS2 {}
impl ::core::default::Default for L2TP_TUNNEL_CONFIG_PARAMS2 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`*"]
pub const MAXIPADRESSLEN: u32 = 64u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`*"]
pub const MAX_SSTP_HASH_SIZE: u32 = 32u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`*"]
pub const METHOD_BGP4_AS_PATH: u32 = 1u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`*"]
pub const METHOD_BGP4_NEXTHOP_ATTR: u32 = 8u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`*"]
pub const METHOD_BGP4_PA_ORIGIN: u32 = 4u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`*"]
pub const METHOD_BGP4_PEER_ID: u32 = 2u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`*"]
pub const METHOD_RIP2_NEIGHBOUR_ADDR: u32 = 1u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`*"]
pub const METHOD_RIP2_OUTBOUND_INTF: u32 = 2u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`*"]
pub const METHOD_RIP2_ROUTE_TAG: u32 = 4u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`*"]
pub const METHOD_RIP2_ROUTE_TIMESTAMP: u32 = 8u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`*"]
pub const METHOD_TYPE_ALL_METHODS: u32 = 4294967295u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct MGM_ENUM_TYPES(pub i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`*"]
pub const ANY_SOURCE: MGM_ENUM_TYPES = MGM_ENUM_TYPES(0i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`*"]
pub const ALL_SOURCES: MGM_ENUM_TYPES = MGM_ENUM_TYPES(1i32);
impl ::core::marker::Copy for MGM_ENUM_TYPES {}
impl ::core::clone::Clone for MGM_ENUM_TYPES {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for MGM_ENUM_TYPES {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for MGM_ENUM_TYPES {
    type Abi = Self;
}
impl ::core::fmt::Debug for MGM_ENUM_TYPES {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MGM_ENUM_TYPES").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`*"]
pub const MGM_FORWARD_STATE_FLAG: u32 = 2u32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct MGM_IF_ENTRY {
    pub dwIfIndex: u32,
    pub dwIfNextHopAddr: u32,
    pub bIGMP: super::super::Foundation::BOOL,
    pub bIsEnabled: super::super::Foundation::BOOL,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for MGM_IF_ENTRY {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for MGM_IF_ENTRY {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for MGM_IF_ENTRY {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("MGM_IF_ENTRY").field("dwIfIndex", &self.dwIfIndex).field("dwIfNextHopAddr", &self.dwIfNextHopAddr).field("bIGMP", &self.bIGMP).field("bIsEnabled", &self.bIsEnabled).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for MGM_IF_ENTRY {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for MGM_IF_ENTRY {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<MGM_IF_ENTRY>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for MGM_IF_ENTRY {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for MGM_IF_ENTRY {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`*"]
pub const MGM_JOIN_STATE_FLAG: u32 = 1u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`*"]
pub const MGM_MFE_STATS_0: u32 = 1u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`*"]
pub const MGM_MFE_STATS_1: u32 = 2u32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`, `\"Win32_Foundation\"`, `\"Win32_Networking_WinSock\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
pub struct MPRAPI_ADMIN_DLL_CALLBACKS {
    pub revision: u8,
    pub lpfnMprAdminGetIpAddressForUser: PMPRADMINGETIPADDRESSFORUSER,
    pub lpfnMprAdminReleaseIpAddress: PMPRADMINRELEASEIPADRESS,
    pub lpfnMprAdminGetIpv6AddressForUser: PMPRADMINGETIPV6ADDRESSFORUSER,
    pub lpfnMprAdminReleaseIpV6AddressForUser: PMPRADMINRELEASEIPV6ADDRESSFORUSER,
    pub lpfnRasAdminAcceptNewLink: PMPRADMINACCEPTNEWLINK,
    pub lpfnRasAdminLinkHangupNotification: PMPRADMINLINKHANGUPNOTIFICATION,
    pub lpfnRasAdminTerminateDll: PMPRADMINTERMINATEDLL,
    pub lpfnRasAdminAcceptNewConnectionEx: PMPRADMINACCEPTNEWCONNECTIONEX,
    pub lpfnRasAdminAcceptEndpointChangeEx: PMPRADMINACCEPTTUNNELENDPOINTCHANGEEX,
    pub lpfnRasAdminAcceptReauthenticationEx: PMPRADMINACCEPTREAUTHENTICATIONEX,
    pub lpfnRasAdminConnectionHangupNotificationEx: PMPRADMINCONNECTIONHANGUPNOTIFICATIONEX,
    pub lpfnRASValidatePreAuthenticatedConnectionEx: PMPRADMINRASVALIDATEPREAUTHENTICATEDCONNECTIONEX,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
impl ::core::marker::Copy for MPRAPI_ADMIN_DLL_CALLBACKS {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
impl ::core::clone::Clone for MPRAPI_ADMIN_DLL_CALLBACKS {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
impl ::core::fmt::Debug for MPRAPI_ADMIN_DLL_CALLBACKS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("MPRAPI_ADMIN_DLL_CALLBACKS")
            .field("revision", &self.revision)
            .field("lpfnMprAdminGetIpAddressForUser", &self.lpfnMprAdminGetIpAddressForUser.map(|f| f as usize))
            .field("lpfnMprAdminReleaseIpAddress", &self.lpfnMprAdminReleaseIpAddress.map(|f| f as usize))
            .field("lpfnMprAdminGetIpv6AddressForUser", &self.lpfnMprAdminGetIpv6AddressForUser.map(|f| f as usize))
            .field("lpfnMprAdminReleaseIpV6AddressForUser", &self.lpfnMprAdminReleaseIpV6AddressForUser.map(|f| f as usize))
            .field("lpfnRasAdminAcceptNewLink", &self.lpfnRasAdminAcceptNewLink.map(|f| f as usize))
            .field("lpfnRasAdminLinkHangupNotification", &self.lpfnRasAdminLinkHangupNotification.map(|f| f as usize))
            .field("lpfnRasAdminTerminateDll", &self.lpfnRasAdminTerminateDll.map(|f| f as usize))
            .field("lpfnRasAdminAcceptNewConnectionEx", &self.lpfnRasAdminAcceptNewConnectionEx.map(|f| f as usize))
            .field("lpfnRasAdminAcceptEndpointChangeEx", &self.lpfnRasAdminAcceptEndpointChangeEx.map(|f| f as usize))
            .field("lpfnRasAdminAcceptReauthenticationEx", &self.lpfnRasAdminAcceptReauthenticationEx.map(|f| f as usize))
            .field("lpfnRasAdminConnectionHangupNotificationEx", &self.lpfnRasAdminConnectionHangupNotificationEx.map(|f| f as usize))
            .field("lpfnRASValidatePreAuthenticatedConnectionEx", &self.lpfnRASValidatePreAuthenticatedConnectionEx.map(|f| f as usize))
            .finish()
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
unsafe impl ::windows::core::Abi for MPRAPI_ADMIN_DLL_CALLBACKS {
    type Abi = Self;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
impl ::core::cmp::PartialEq for MPRAPI_ADMIN_DLL_CALLBACKS {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<MPRAPI_ADMIN_DLL_CALLBACKS>()) == 0 }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
impl ::core::cmp::Eq for MPRAPI_ADMIN_DLL_CALLBACKS {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
impl ::core::default::Default for MPRAPI_ADMIN_DLL_CALLBACKS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`*"]
pub const MPRAPI_ADMIN_DLL_VERSION_1: u32 = 1u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`*"]
pub const MPRAPI_ADMIN_DLL_VERSION_2: u32 = 2u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`*"]
pub const MPRAPI_IF_CUSTOM_CONFIG_FOR_IKEV2: u32 = 1u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`*"]
pub const MPRAPI_IKEV2_AUTH_USING_CERT: u32 = 1u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`*"]
pub const MPRAPI_IKEV2_AUTH_USING_EAP: u32 = 2u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`*"]
pub const MPRAPI_IKEV2_PROJECTION_INFO_TYPE: u32 = 2u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`*"]
pub const MPRAPI_IKEV2_SET_TUNNEL_CONFIG_PARAMS: u32 = 1u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`*"]
pub const MPRAPI_L2TP_SET_TUNNEL_CONFIG_PARAMS: u32 = 1u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`*"]
pub const MPRAPI_MPR_IF_CUSTOM_CONFIG_OBJECT_REVISION_1: u32 = 1u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`*"]
pub const MPRAPI_MPR_IF_CUSTOM_CONFIG_OBJECT_REVISION_2: u32 = 2u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`*"]
pub const MPRAPI_MPR_IF_CUSTOM_CONFIG_OBJECT_REVISION_3: u32 = 3u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`*"]
pub const MPRAPI_MPR_SERVER_OBJECT_REVISION_1: u32 = 1u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`*"]
pub const MPRAPI_MPR_SERVER_OBJECT_REVISION_2: u32 = 2u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`*"]
pub const MPRAPI_MPR_SERVER_OBJECT_REVISION_3: u32 = 3u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`*"]
pub const MPRAPI_MPR_SERVER_OBJECT_REVISION_4: u32 = 4u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`*"]
pub const MPRAPI_MPR_SERVER_OBJECT_REVISION_5: u32 = 5u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`*"]
pub const MPRAPI_MPR_SERVER_SET_CONFIG_OBJECT_REVISION_1: u32 = 1u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`*"]
pub const MPRAPI_MPR_SERVER_SET_CONFIG_OBJECT_REVISION_2: u32 = 2u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`*"]
pub const MPRAPI_MPR_SERVER_SET_CONFIG_OBJECT_REVISION_3: u32 = 3u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`*"]
pub const MPRAPI_MPR_SERVER_SET_CONFIG_OBJECT_REVISION_4: u32 = 4u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`*"]
pub const MPRAPI_MPR_SERVER_SET_CONFIG_OBJECT_REVISION_5: u32 = 5u32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`*"]
pub struct MPRAPI_OBJECT_HEADER {
    pub revision: u8,
    pub r#type: u8,
    pub size: u16,
}
impl ::core::marker::Copy for MPRAPI_OBJECT_HEADER {}
impl ::core::clone::Clone for MPRAPI_OBJECT_HEADER {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for MPRAPI_OBJECT_HEADER {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("MPRAPI_OBJECT_HEADER").field("revision", &self.revision).field("type", &self.r#type).field("size", &self.size).finish()
    }
}
unsafe impl ::windows::core::Abi for MPRAPI_OBJECT_HEADER {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for MPRAPI_OBJECT_HEADER {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<MPRAPI_OBJECT_HEADER>()) == 0 }
    }
}
impl ::core::cmp::Eq for MPRAPI_OBJECT_HEADER {}
impl ::core::default::Default for MPRAPI_OBJECT_HEADER {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct MPRAPI_OBJECT_TYPE(pub i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`*"]
pub const MPRAPI_OBJECT_TYPE_RAS_CONNECTION_OBJECT: MPRAPI_OBJECT_TYPE = MPRAPI_OBJECT_TYPE(1i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`*"]
pub const MPRAPI_OBJECT_TYPE_MPR_SERVER_OBJECT: MPRAPI_OBJECT_TYPE = MPRAPI_OBJECT_TYPE(2i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`*"]
pub const MPRAPI_OBJECT_TYPE_MPR_SERVER_SET_CONFIG_OBJECT: MPRAPI_OBJECT_TYPE = MPRAPI_OBJECT_TYPE(3i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`*"]
pub const MPRAPI_OBJECT_TYPE_AUTH_VALIDATION_OBJECT: MPRAPI_OBJECT_TYPE = MPRAPI_OBJECT_TYPE(4i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`*"]
pub const MPRAPI_OBJECT_TYPE_UPDATE_CONNECTION_OBJECT: MPRAPI_OBJECT_TYPE = MPRAPI_OBJECT_TYPE(5i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`*"]
pub const MPRAPI_OBJECT_TYPE_IF_CUSTOM_CONFIG_OBJECT: MPRAPI_OBJECT_TYPE = MPRAPI_OBJECT_TYPE(6i32);
impl ::core::marker::Copy for MPRAPI_OBJECT_TYPE {}
impl ::core::clone::Clone for MPRAPI_OBJECT_TYPE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for MPRAPI_OBJECT_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for MPRAPI_OBJECT_TYPE {
    type Abi = Self;
}
impl ::core::fmt::Debug for MPRAPI_OBJECT_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MPRAPI_OBJECT_TYPE").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`*"]
pub const MPRAPI_PPP_PROJECTION_INFO_TYPE: u32 = 1u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`*"]
pub const MPRAPI_RAS_CONNECTION_OBJECT_REVISION_1: u32 = 1u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`*"]
pub const MPRAPI_RAS_UPDATE_CONNECTION_OBJECT_REVISION_1: u32 = 1u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`*"]
pub const MPRAPI_SET_CONFIG_PROTOCOL_FOR_GRE: u32 = 16u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`*"]
pub const MPRAPI_SET_CONFIG_PROTOCOL_FOR_IKEV2: u32 = 8u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`*"]
pub const MPRAPI_SET_CONFIG_PROTOCOL_FOR_L2TP: u32 = 2u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`*"]
pub const MPRAPI_SET_CONFIG_PROTOCOL_FOR_PPTP: u32 = 1u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`*"]
pub const MPRAPI_SET_CONFIG_PROTOCOL_FOR_SSTP: u32 = 4u32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`, `\"Win32_Foundation\"`, `\"Win32_Security_Cryptography\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography"))]
pub struct MPRAPI_TUNNEL_CONFIG_PARAMS0 {
    pub IkeConfigParams: IKEV2_CONFIG_PARAMS,
    pub PptpConfigParams: PPTP_CONFIG_PARAMS,
    pub L2tpConfigParams: L2TP_CONFIG_PARAMS1,
    pub SstpConfigParams: SSTP_CONFIG_PARAMS,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography"))]
impl ::core::marker::Copy for MPRAPI_TUNNEL_CONFIG_PARAMS0 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography"))]
impl ::core::clone::Clone for MPRAPI_TUNNEL_CONFIG_PARAMS0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography"))]
impl ::core::fmt::Debug for MPRAPI_TUNNEL_CONFIG_PARAMS0 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("MPRAPI_TUNNEL_CONFIG_PARAMS0").field("IkeConfigParams", &self.IkeConfigParams).field("PptpConfigParams", &self.PptpConfigParams).field("L2tpConfigParams", &self.L2tpConfigParams).field("SstpConfigParams", &self.SstpConfigParams).finish()
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography"))]
unsafe impl ::windows::core::Abi for MPRAPI_TUNNEL_CONFIG_PARAMS0 {
    type Abi = Self;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography"))]
impl ::core::cmp::PartialEq for MPRAPI_TUNNEL_CONFIG_PARAMS0 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<MPRAPI_TUNNEL_CONFIG_PARAMS0>()) == 0 }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography"))]
impl ::core::cmp::Eq for MPRAPI_TUNNEL_CONFIG_PARAMS0 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography"))]
impl ::core::default::Default for MPRAPI_TUNNEL_CONFIG_PARAMS0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`, `\"Win32_Foundation\"`, `\"Win32_Security_Cryptography\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography"))]
pub struct MPRAPI_TUNNEL_CONFIG_PARAMS1 {
    pub IkeConfigParams: IKEV2_CONFIG_PARAMS,
    pub PptpConfigParams: PPTP_CONFIG_PARAMS,
    pub L2tpConfigParams: L2TP_CONFIG_PARAMS1,
    pub SstpConfigParams: SSTP_CONFIG_PARAMS,
    pub GREConfigParams: GRE_CONFIG_PARAMS0,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography"))]
impl ::core::marker::Copy for MPRAPI_TUNNEL_CONFIG_PARAMS1 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography"))]
impl ::core::clone::Clone for MPRAPI_TUNNEL_CONFIG_PARAMS1 {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography"))]
impl ::core::fmt::Debug for MPRAPI_TUNNEL_CONFIG_PARAMS1 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("MPRAPI_TUNNEL_CONFIG_PARAMS1").field("IkeConfigParams", &self.IkeConfigParams).field("PptpConfigParams", &self.PptpConfigParams).field("L2tpConfigParams", &self.L2tpConfigParams).field("SstpConfigParams", &self.SstpConfigParams).field("GREConfigParams", &self.GREConfigParams).finish()
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography"))]
unsafe impl ::windows::core::Abi for MPRAPI_TUNNEL_CONFIG_PARAMS1 {
    type Abi = Self;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography"))]
impl ::core::cmp::PartialEq for MPRAPI_TUNNEL_CONFIG_PARAMS1 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<MPRAPI_TUNNEL_CONFIG_PARAMS1>()) == 0 }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography"))]
impl ::core::cmp::Eq for MPRAPI_TUNNEL_CONFIG_PARAMS1 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography"))]
impl ::core::default::Default for MPRAPI_TUNNEL_CONFIG_PARAMS1 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`*"]
pub const MPRDT_Atm: &str = "ATM";
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`*"]
pub const MPRDT_FrameRelay: &str = "FRAMERELAY";
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`*"]
pub const MPRDT_Generic: &str = "GENERIC";
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`*"]
pub const MPRDT_Irda: &str = "IRDA";
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`*"]
pub const MPRDT_Isdn: &str = "isdn";
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`*"]
pub const MPRDT_Modem: &str = "modem";
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`*"]
pub const MPRDT_Pad: &str = "pad";
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`*"]
pub const MPRDT_Parallel: &str = "PARALLEL";
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`*"]
pub const MPRDT_SW56: &str = "SW56";
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`*"]
pub const MPRDT_Serial: &str = "SERIAL";
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`*"]
pub const MPRDT_Sonet: &str = "SONET";
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`*"]
pub const MPRDT_Vpn: &str = "vpn";
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`*"]
pub const MPRDT_X25: &str = "x25";
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`*"]
pub const MPRET_Direct: u32 = 3u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`*"]
pub const MPRET_Phone: u32 = 1u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`*"]
pub const MPRET_Vpn: u32 = 2u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`*"]
pub const MPRIDS_Disabled: u32 = 4294967295u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`*"]
pub const MPRIDS_UseGlobalValue: u32 = 0u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`*"]
pub const MPRIO_DisableLcpExtensions: u32 = 32u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`*"]
pub const MPRIO_IpHeaderCompression: u32 = 8u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`*"]
pub const MPRIO_IpSecPreSharedKey: u32 = 2147483648u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`*"]
pub const MPRIO_NetworkLogon: u32 = 8192u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`*"]
pub const MPRIO_PromoteAlternates: u32 = 32768u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`*"]
pub const MPRIO_RemoteDefaultGateway: u32 = 16u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`*"]
pub const MPRIO_RequireCHAP: u32 = 134217728u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`*"]
pub const MPRIO_RequireDataEncryption: u32 = 4096u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`*"]
pub const MPRIO_RequireEAP: u32 = 131072u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`*"]
pub const MPRIO_RequireEncryptedPw: u32 = 1024u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`*"]
pub const MPRIO_RequireMachineCertificates: u32 = 16777216u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`*"]
pub const MPRIO_RequireMsCHAP: u32 = 268435456u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`*"]
pub const MPRIO_RequireMsCHAP2: u32 = 536870912u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`*"]
pub const MPRIO_RequireMsEncryptedPw: u32 = 2048u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`*"]
pub const MPRIO_RequirePAP: u32 = 262144u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`*"]
pub const MPRIO_RequireSPAP: u32 = 524288u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`*"]
pub const MPRIO_SecureLocalFiles: u32 = 65536u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`*"]
pub const MPRIO_SharedPhoneNumbers: u32 = 8388608u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`*"]
pub const MPRIO_SpecificIpAddr: u32 = 2u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`*"]
pub const MPRIO_SpecificNameServers: u32 = 4u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`*"]
pub const MPRIO_SwCompression: u32 = 512u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`*"]
pub const MPRIO_UsePreSharedKeyForIkev2Initiator: u32 = 33554432u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`*"]
pub const MPRIO_UsePreSharedKeyForIkev2Responder: u32 = 67108864u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`*"]
pub const MPRNP_Ip: u32 = 4u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`*"]
pub const MPRNP_Ipv6: u32 = 8u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`*"]
pub const MPRNP_Ipx: u32 = 2u32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct MPR_CERT_EKU {
    pub dwSize: u32,
    pub IsEKUOID: super::super::Foundation::BOOL,
    pub pwszEKU: ::windows::core::PWSTR,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for MPR_CERT_EKU {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for MPR_CERT_EKU {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for MPR_CERT_EKU {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("MPR_CERT_EKU").field("dwSize", &self.dwSize).field("IsEKUOID", &self.IsEKUOID).field("pwszEKU", &self.pwszEKU).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for MPR_CERT_EKU {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for MPR_CERT_EKU {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<MPR_CERT_EKU>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for MPR_CERT_EKU {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for MPR_CERT_EKU {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`*"]
pub struct MPR_CREDENTIALSEX_0 {
    pub dwSize: u32,
    pub lpbCredentialsInfo: *mut u8,
}
impl ::core::marker::Copy for MPR_CREDENTIALSEX_0 {}
impl ::core::clone::Clone for MPR_CREDENTIALSEX_0 {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for MPR_CREDENTIALSEX_0 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("MPR_CREDENTIALSEX_0").field("dwSize", &self.dwSize).field("lpbCredentialsInfo", &self.lpbCredentialsInfo).finish()
    }
}
unsafe impl ::windows::core::Abi for MPR_CREDENTIALSEX_0 {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for MPR_CREDENTIALSEX_0 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<MPR_CREDENTIALSEX_0>()) == 0 }
    }
}
impl ::core::cmp::Eq for MPR_CREDENTIALSEX_0 {}
impl ::core::default::Default for MPR_CREDENTIALSEX_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`*"]
pub struct MPR_CREDENTIALSEX_1 {
    pub dwSize: u32,
    pub lpbCredentialsInfo: *mut u8,
}
impl ::core::marker::Copy for MPR_CREDENTIALSEX_1 {}
impl ::core::clone::Clone for MPR_CREDENTIALSEX_1 {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for MPR_CREDENTIALSEX_1 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("MPR_CREDENTIALSEX_1").field("dwSize", &self.dwSize).field("lpbCredentialsInfo", &self.lpbCredentialsInfo).finish()
    }
}
unsafe impl ::windows::core::Abi for MPR_CREDENTIALSEX_1 {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for MPR_CREDENTIALSEX_1 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<MPR_CREDENTIALSEX_1>()) == 0 }
    }
}
impl ::core::cmp::Eq for MPR_CREDENTIALSEX_1 {}
impl ::core::default::Default for MPR_CREDENTIALSEX_1 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`*"]
pub struct MPR_DEVICE_0 {
    pub szDeviceType: [u16; 17],
    pub szDeviceName: [u16; 129],
}
impl ::core::marker::Copy for MPR_DEVICE_0 {}
impl ::core::clone::Clone for MPR_DEVICE_0 {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for MPR_DEVICE_0 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("MPR_DEVICE_0").field("szDeviceType", &self.szDeviceType).field("szDeviceName", &self.szDeviceName).finish()
    }
}
unsafe impl ::windows::core::Abi for MPR_DEVICE_0 {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for MPR_DEVICE_0 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<MPR_DEVICE_0>()) == 0 }
    }
}
impl ::core::cmp::Eq for MPR_DEVICE_0 {}
impl ::core::default::Default for MPR_DEVICE_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`*"]
pub struct MPR_DEVICE_1 {
    pub szDeviceType: [u16; 17],
    pub szDeviceName: [u16; 129],
    pub szLocalPhoneNumber: [u16; 129],
    pub szAlternates: ::windows::core::PWSTR,
}
impl ::core::marker::Copy for MPR_DEVICE_1 {}
impl ::core::clone::Clone for MPR_DEVICE_1 {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for MPR_DEVICE_1 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("MPR_DEVICE_1").field("szDeviceType", &self.szDeviceType).field("szDeviceName", &self.szDeviceName).field("szLocalPhoneNumber", &self.szLocalPhoneNumber).field("szAlternates", &self.szAlternates).finish()
    }
}
unsafe impl ::windows::core::Abi for MPR_DEVICE_1 {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for MPR_DEVICE_1 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<MPR_DEVICE_1>()) == 0 }
    }
}
impl ::core::cmp::Eq for MPR_DEVICE_1 {}
impl ::core::default::Default for MPR_DEVICE_1 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`*"]
pub const MPR_ENABLE_RAS_ON_DEVICE: u32 = 1u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`*"]
pub const MPR_ENABLE_ROUTING_ON_DEVICE: u32 = 2u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct MPR_ET(pub u32);
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`*"]
pub const MPR_ET_None: MPR_ET = MPR_ET(0u32);
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`*"]
pub const MPR_ET_Require: MPR_ET = MPR_ET(1u32);
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`*"]
pub const MPR_ET_RequireMax: MPR_ET = MPR_ET(2u32);
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`*"]
pub const MPR_ET_Optional: MPR_ET = MPR_ET(3u32);
impl ::core::marker::Copy for MPR_ET {}
impl ::core::clone::Clone for MPR_ET {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for MPR_ET {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for MPR_ET {
    type Abi = Self;
}
impl ::core::fmt::Debug for MPR_ET {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MPR_ET").field(&self.0).finish()
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct MPR_FILTER_0 {
    pub fEnable: super::super::Foundation::BOOL,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for MPR_FILTER_0 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for MPR_FILTER_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for MPR_FILTER_0 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("MPR_FILTER_0").field("fEnable", &self.fEnable).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for MPR_FILTER_0 {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for MPR_FILTER_0 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<MPR_FILTER_0>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for MPR_FILTER_0 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for MPR_FILTER_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct MPR_IFTRANSPORT_0 {
    pub dwTransportId: u32,
    pub hIfTransport: super::super::Foundation::HANDLE,
    pub wszIfTransportName: [u16; 41],
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for MPR_IFTRANSPORT_0 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for MPR_IFTRANSPORT_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for MPR_IFTRANSPORT_0 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("MPR_IFTRANSPORT_0").field("dwTransportId", &self.dwTransportId).field("hIfTransport", &self.hIfTransport).field("wszIfTransportName", &self.wszIfTransportName).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for MPR_IFTRANSPORT_0 {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for MPR_IFTRANSPORT_0 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<MPR_IFTRANSPORT_0>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for MPR_IFTRANSPORT_0 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for MPR_IFTRANSPORT_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`, `\"Win32_Security_Cryptography\"`*"]
#[cfg(feature = "Win32_Security_Cryptography")]
pub struct MPR_IF_CUSTOMINFOEX0 {
    pub Header: MPRAPI_OBJECT_HEADER,
    pub dwFlags: u32,
    pub customIkev2Config: ROUTER_IKEv2_IF_CUSTOM_CONFIG0,
}
#[cfg(feature = "Win32_Security_Cryptography")]
impl ::core::marker::Copy for MPR_IF_CUSTOMINFOEX0 {}
#[cfg(feature = "Win32_Security_Cryptography")]
impl ::core::clone::Clone for MPR_IF_CUSTOMINFOEX0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Security_Cryptography")]
impl ::core::fmt::Debug for MPR_IF_CUSTOMINFOEX0 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("MPR_IF_CUSTOMINFOEX0").field("Header", &self.Header).field("dwFlags", &self.dwFlags).field("customIkev2Config", &self.customIkev2Config).finish()
    }
}
#[cfg(feature = "Win32_Security_Cryptography")]
unsafe impl ::windows::core::Abi for MPR_IF_CUSTOMINFOEX0 {
    type Abi = Self;
}
#[cfg(feature = "Win32_Security_Cryptography")]
impl ::core::cmp::PartialEq for MPR_IF_CUSTOMINFOEX0 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<MPR_IF_CUSTOMINFOEX0>()) == 0 }
    }
}
#[cfg(feature = "Win32_Security_Cryptography")]
impl ::core::cmp::Eq for MPR_IF_CUSTOMINFOEX0 {}
#[cfg(feature = "Win32_Security_Cryptography")]
impl ::core::default::Default for MPR_IF_CUSTOMINFOEX0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`, `\"Win32_Security_Cryptography\"`*"]
#[cfg(feature = "Win32_Security_Cryptography")]
pub struct MPR_IF_CUSTOMINFOEX1 {
    pub Header: MPRAPI_OBJECT_HEADER,
    pub dwFlags: u32,
    pub customIkev2Config: ROUTER_IKEv2_IF_CUSTOM_CONFIG1,
}
#[cfg(feature = "Win32_Security_Cryptography")]
impl ::core::marker::Copy for MPR_IF_CUSTOMINFOEX1 {}
#[cfg(feature = "Win32_Security_Cryptography")]
impl ::core::clone::Clone for MPR_IF_CUSTOMINFOEX1 {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Security_Cryptography")]
impl ::core::fmt::Debug for MPR_IF_CUSTOMINFOEX1 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("MPR_IF_CUSTOMINFOEX1").field("Header", &self.Header).field("dwFlags", &self.dwFlags).field("customIkev2Config", &self.customIkev2Config).finish()
    }
}
#[cfg(feature = "Win32_Security_Cryptography")]
unsafe impl ::windows::core::Abi for MPR_IF_CUSTOMINFOEX1 {
    type Abi = Self;
}
#[cfg(feature = "Win32_Security_Cryptography")]
impl ::core::cmp::PartialEq for MPR_IF_CUSTOMINFOEX1 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<MPR_IF_CUSTOMINFOEX1>()) == 0 }
    }
}
#[cfg(feature = "Win32_Security_Cryptography")]
impl ::core::cmp::Eq for MPR_IF_CUSTOMINFOEX1 {}
#[cfg(feature = "Win32_Security_Cryptography")]
impl ::core::default::Default for MPR_IF_CUSTOMINFOEX1 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`, `\"Win32_Networking_WinSock\"`, `\"Win32_Security_Cryptography\"`*"]
#[cfg(all(feature = "Win32_Networking_WinSock", feature = "Win32_Security_Cryptography"))]
pub struct MPR_IF_CUSTOMINFOEX2 {
    pub Header: MPRAPI_OBJECT_HEADER,
    pub dwFlags: u32,
    pub customIkev2Config: ROUTER_IKEv2_IF_CUSTOM_CONFIG2,
}
#[cfg(all(feature = "Win32_Networking_WinSock", feature = "Win32_Security_Cryptography"))]
impl ::core::marker::Copy for MPR_IF_CUSTOMINFOEX2 {}
#[cfg(all(feature = "Win32_Networking_WinSock", feature = "Win32_Security_Cryptography"))]
impl ::core::clone::Clone for MPR_IF_CUSTOMINFOEX2 {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(all(feature = "Win32_Networking_WinSock", feature = "Win32_Security_Cryptography"))]
impl ::core::fmt::Debug for MPR_IF_CUSTOMINFOEX2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("MPR_IF_CUSTOMINFOEX2").field("Header", &self.Header).field("dwFlags", &self.dwFlags).field("customIkev2Config", &self.customIkev2Config).finish()
    }
}
#[cfg(all(feature = "Win32_Networking_WinSock", feature = "Win32_Security_Cryptography"))]
unsafe impl ::windows::core::Abi for MPR_IF_CUSTOMINFOEX2 {
    type Abi = Self;
}
#[cfg(all(feature = "Win32_Networking_WinSock", feature = "Win32_Security_Cryptography"))]
impl ::core::cmp::PartialEq for MPR_IF_CUSTOMINFOEX2 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<MPR_IF_CUSTOMINFOEX2>()) == 0 }
    }
}
#[cfg(all(feature = "Win32_Networking_WinSock", feature = "Win32_Security_Cryptography"))]
impl ::core::cmp::Eq for MPR_IF_CUSTOMINFOEX2 {}
#[cfg(all(feature = "Win32_Networking_WinSock", feature = "Win32_Security_Cryptography"))]
impl ::core::default::Default for MPR_IF_CUSTOMINFOEX2 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct MPR_INTERFACE_0 {
    pub wszInterfaceName: [u16; 257],
    pub hInterface: super::super::Foundation::HANDLE,
    pub fEnabled: super::super::Foundation::BOOL,
    pub dwIfType: ROUTER_INTERFACE_TYPE,
    pub dwConnectionState: ROUTER_CONNECTION_STATE,
    pub fUnReachabilityReasons: u32,
    pub dwLastError: u32,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for MPR_INTERFACE_0 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for MPR_INTERFACE_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for MPR_INTERFACE_0 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("MPR_INTERFACE_0").field("wszInterfaceName", &self.wszInterfaceName).field("hInterface", &self.hInterface).field("fEnabled", &self.fEnabled).field("dwIfType", &self.dwIfType).field("dwConnectionState", &self.dwConnectionState).field("fUnReachabilityReasons", &self.fUnReachabilityReasons).field("dwLastError", &self.dwLastError).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for MPR_INTERFACE_0 {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for MPR_INTERFACE_0 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<MPR_INTERFACE_0>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for MPR_INTERFACE_0 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for MPR_INTERFACE_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct MPR_INTERFACE_1 {
    pub wszInterfaceName: [u16; 257],
    pub hInterface: super::super::Foundation::HANDLE,
    pub fEnabled: super::super::Foundation::BOOL,
    pub dwIfType: ROUTER_INTERFACE_TYPE,
    pub dwConnectionState: ROUTER_CONNECTION_STATE,
    pub fUnReachabilityReasons: u32,
    pub dwLastError: u32,
    pub lpwsDialoutHoursRestriction: ::windows::core::PWSTR,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for MPR_INTERFACE_1 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for MPR_INTERFACE_1 {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for MPR_INTERFACE_1 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("MPR_INTERFACE_1").field("wszInterfaceName", &self.wszInterfaceName).field("hInterface", &self.hInterface).field("fEnabled", &self.fEnabled).field("dwIfType", &self.dwIfType).field("dwConnectionState", &self.dwConnectionState).field("fUnReachabilityReasons", &self.fUnReachabilityReasons).field("dwLastError", &self.dwLastError).field("lpwsDialoutHoursRestriction", &self.lpwsDialoutHoursRestriction).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for MPR_INTERFACE_1 {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for MPR_INTERFACE_1 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<MPR_INTERFACE_1>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for MPR_INTERFACE_1 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for MPR_INTERFACE_1 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct MPR_INTERFACE_2 {
    pub wszInterfaceName: [u16; 257],
    pub hInterface: super::super::Foundation::HANDLE,
    pub fEnabled: super::super::Foundation::BOOL,
    pub dwIfType: ROUTER_INTERFACE_TYPE,
    pub dwConnectionState: ROUTER_CONNECTION_STATE,
    pub fUnReachabilityReasons: u32,
    pub dwLastError: u32,
    pub dwfOptions: u32,
    pub szLocalPhoneNumber: [u16; 129],
    pub szAlternates: ::windows::core::PWSTR,
    pub ipaddr: u32,
    pub ipaddrDns: u32,
    pub ipaddrDnsAlt: u32,
    pub ipaddrWins: u32,
    pub ipaddrWinsAlt: u32,
    pub dwfNetProtocols: u32,
    pub szDeviceType: [u16; 17],
    pub szDeviceName: [u16; 129],
    pub szX25PadType: [u16; 33],
    pub szX25Address: [u16; 201],
    pub szX25Facilities: [u16; 201],
    pub szX25UserData: [u16; 201],
    pub dwChannels: u32,
    pub dwSubEntries: u32,
    pub dwDialMode: MPR_INTERFACE_DIAL_MODE,
    pub dwDialExtraPercent: u32,
    pub dwDialExtraSampleSeconds: u32,
    pub dwHangUpExtraPercent: u32,
    pub dwHangUpExtraSampleSeconds: u32,
    pub dwIdleDisconnectSeconds: u32,
    pub dwType: u32,
    pub dwEncryptionType: MPR_ET,
    pub dwCustomAuthKey: u32,
    pub dwCustomAuthDataSize: u32,
    pub lpbCustomAuthData: *mut u8,
    pub guidId: ::windows::core::GUID,
    pub dwVpnStrategy: MPR_VS,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for MPR_INTERFACE_2 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for MPR_INTERFACE_2 {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for MPR_INTERFACE_2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("MPR_INTERFACE_2")
            .field("wszInterfaceName", &self.wszInterfaceName)
            .field("hInterface", &self.hInterface)
            .field("fEnabled", &self.fEnabled)
            .field("dwIfType", &self.dwIfType)
            .field("dwConnectionState", &self.dwConnectionState)
            .field("fUnReachabilityReasons", &self.fUnReachabilityReasons)
            .field("dwLastError", &self.dwLastError)
            .field("dwfOptions", &self.dwfOptions)
            .field("szLocalPhoneNumber", &self.szLocalPhoneNumber)
            .field("szAlternates", &self.szAlternates)
            .field("ipaddr", &self.ipaddr)
            .field("ipaddrDns", &self.ipaddrDns)
            .field("ipaddrDnsAlt", &self.ipaddrDnsAlt)
            .field("ipaddrWins", &self.ipaddrWins)
            .field("ipaddrWinsAlt", &self.ipaddrWinsAlt)
            .field("dwfNetProtocols", &self.dwfNetProtocols)
            .field("szDeviceType", &self.szDeviceType)
            .field("szDeviceName", &self.szDeviceName)
            .field("szX25PadType", &self.szX25PadType)
            .field("szX25Address", &self.szX25Address)
            .field("szX25Facilities", &self.szX25Facilities)
            .field("szX25UserData", &self.szX25UserData)
            .field("dwChannels", &self.dwChannels)
            .field("dwSubEntries", &self.dwSubEntries)
            .field("dwDialMode", &self.dwDialMode)
            .field("dwDialExtraPercent", &self.dwDialExtraPercent)
            .field("dwDialExtraSampleSeconds", &self.dwDialExtraSampleSeconds)
            .field("dwHangUpExtraPercent", &self.dwHangUpExtraPercent)
            .field("dwHangUpExtraSampleSeconds", &self.dwHangUpExtraSampleSeconds)
            .field("dwIdleDisconnectSeconds", &self.dwIdleDisconnectSeconds)
            .field("dwType", &self.dwType)
            .field("dwEncryptionType", &self.dwEncryptionType)
            .field("dwCustomAuthKey", &self.dwCustomAuthKey)
            .field("dwCustomAuthDataSize", &self.dwCustomAuthDataSize)
            .field("lpbCustomAuthData", &self.lpbCustomAuthData)
            .field("guidId", &self.guidId)
            .field("dwVpnStrategy", &self.dwVpnStrategy)
            .finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for MPR_INTERFACE_2 {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for MPR_INTERFACE_2 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<MPR_INTERFACE_2>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for MPR_INTERFACE_2 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for MPR_INTERFACE_2 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`, `\"Win32_Foundation\"`, `\"Win32_Networking_WinSock\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
pub struct MPR_INTERFACE_3 {
    pub wszInterfaceName: [u16; 257],
    pub hInterface: super::super::Foundation::HANDLE,
    pub fEnabled: super::super::Foundation::BOOL,
    pub dwIfType: ROUTER_INTERFACE_TYPE,
    pub dwConnectionState: ROUTER_CONNECTION_STATE,
    pub fUnReachabilityReasons: u32,
    pub dwLastError: u32,
    pub dwfOptions: u32,
    pub szLocalPhoneNumber: [u16; 129],
    pub szAlternates: ::windows::core::PWSTR,
    pub ipaddr: u32,
    pub ipaddrDns: u32,
    pub ipaddrDnsAlt: u32,
    pub ipaddrWins: u32,
    pub ipaddrWinsAlt: u32,
    pub dwfNetProtocols: u32,
    pub szDeviceType: [u16; 17],
    pub szDeviceName: [u16; 129],
    pub szX25PadType: [u16; 33],
    pub szX25Address: [u16; 201],
    pub szX25Facilities: [u16; 201],
    pub szX25UserData: [u16; 201],
    pub dwChannels: u32,
    pub dwSubEntries: u32,
    pub dwDialMode: MPR_INTERFACE_DIAL_MODE,
    pub dwDialExtraPercent: u32,
    pub dwDialExtraSampleSeconds: u32,
    pub dwHangUpExtraPercent: u32,
    pub dwHangUpExtraSampleSeconds: u32,
    pub dwIdleDisconnectSeconds: u32,
    pub dwType: u32,
    pub dwEncryptionType: MPR_ET,
    pub dwCustomAuthKey: u32,
    pub dwCustomAuthDataSize: u32,
    pub lpbCustomAuthData: *mut u8,
    pub guidId: ::windows::core::GUID,
    pub dwVpnStrategy: MPR_VS,
    pub AddressCount: u32,
    pub ipv6addrDns: super::super::Networking::WinSock::IN6_ADDR,
    pub ipv6addrDnsAlt: super::super::Networking::WinSock::IN6_ADDR,
    pub ipv6addr: *mut super::super::Networking::WinSock::IN6_ADDR,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
impl ::core::marker::Copy for MPR_INTERFACE_3 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
impl ::core::clone::Clone for MPR_INTERFACE_3 {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
unsafe impl ::windows::core::Abi for MPR_INTERFACE_3 {
    type Abi = Self;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
impl ::core::cmp::PartialEq for MPR_INTERFACE_3 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<MPR_INTERFACE_3>()) == 0 }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
impl ::core::cmp::Eq for MPR_INTERFACE_3 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
impl ::core::default::Default for MPR_INTERFACE_3 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`*"]
pub const MPR_INTERFACE_ADMIN_DISABLED: u32 = 2u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`*"]
pub const MPR_INTERFACE_CONNECTION_FAILURE: u32 = 4u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`*"]
pub const MPR_INTERFACE_DIALOUT_HOURS_RESTRICTION: u32 = 16u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct MPR_INTERFACE_DIAL_MODE(pub u32);
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`*"]
pub const MPRDM_DialFirst: MPR_INTERFACE_DIAL_MODE = MPR_INTERFACE_DIAL_MODE(0u32);
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`*"]
pub const MPRDM_DialAll: MPR_INTERFACE_DIAL_MODE = MPR_INTERFACE_DIAL_MODE(1u32);
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`*"]
pub const MPRDM_DialAsNeeded: MPR_INTERFACE_DIAL_MODE = MPR_INTERFACE_DIAL_MODE(2u32);
impl ::core::marker::Copy for MPR_INTERFACE_DIAL_MODE {}
impl ::core::clone::Clone for MPR_INTERFACE_DIAL_MODE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for MPR_INTERFACE_DIAL_MODE {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for MPR_INTERFACE_DIAL_MODE {
    type Abi = Self;
}
impl ::core::fmt::Debug for MPR_INTERFACE_DIAL_MODE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MPR_INTERFACE_DIAL_MODE").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`*"]
pub const MPR_INTERFACE_NO_DEVICE: u32 = 64u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`*"]
pub const MPR_INTERFACE_NO_MEDIA_SENSE: u32 = 32u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`*"]
pub const MPR_INTERFACE_OUT_OF_RESOURCES: u32 = 1u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`*"]
pub const MPR_INTERFACE_SERVICE_PAUSED: u32 = 8u32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`*"]
pub struct MPR_IPINIP_INTERFACE_0 {
    pub wszFriendlyName: [u16; 257],
    pub Guid: ::windows::core::GUID,
}
impl ::core::marker::Copy for MPR_IPINIP_INTERFACE_0 {}
impl ::core::clone::Clone for MPR_IPINIP_INTERFACE_0 {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for MPR_IPINIP_INTERFACE_0 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("MPR_IPINIP_INTERFACE_0").field("wszFriendlyName", &self.wszFriendlyName).field("Guid", &self.Guid).finish()
    }
}
unsafe impl ::windows::core::Abi for MPR_IPINIP_INTERFACE_0 {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for MPR_IPINIP_INTERFACE_0 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<MPR_IPINIP_INTERFACE_0>()) == 0 }
    }
}
impl ::core::cmp::Eq for MPR_IPINIP_INTERFACE_0 {}
impl ::core::default::Default for MPR_IPINIP_INTERFACE_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`*"]
pub const MPR_MaxAreaCode: u32 = 10u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`*"]
pub const MPR_MaxCallbackNumber: u32 = 128u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`*"]
pub const MPR_MaxDeviceName: u32 = 128u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`*"]
pub const MPR_MaxDeviceType: u32 = 16u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`*"]
pub const MPR_MaxEntryName: u32 = 256u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`*"]
pub const MPR_MaxFacilities: u32 = 200u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`*"]
pub const MPR_MaxIpAddress: u32 = 15u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`*"]
pub const MPR_MaxIpxAddress: u32 = 21u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`*"]
pub const MPR_MaxPadType: u32 = 32u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`*"]
pub const MPR_MaxPhoneNumber: u32 = 128u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`*"]
pub const MPR_MaxUserData: u32 = 200u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`*"]
pub const MPR_MaxX25Address: u32 = 200u32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct MPR_SERVER_0 {
    pub fLanOnlyMode: super::super::Foundation::BOOL,
    pub dwUpTime: u32,
    pub dwTotalPorts: u32,
    pub dwPortsInUse: u32,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for MPR_SERVER_0 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for MPR_SERVER_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for MPR_SERVER_0 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("MPR_SERVER_0").field("fLanOnlyMode", &self.fLanOnlyMode).field("dwUpTime", &self.dwUpTime).field("dwTotalPorts", &self.dwTotalPorts).field("dwPortsInUse", &self.dwPortsInUse).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for MPR_SERVER_0 {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for MPR_SERVER_0 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<MPR_SERVER_0>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for MPR_SERVER_0 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for MPR_SERVER_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`*"]
pub struct MPR_SERVER_1 {
    pub dwNumPptpPorts: u32,
    pub dwPptpPortFlags: u32,
    pub dwNumL2tpPorts: u32,
    pub dwL2tpPortFlags: u32,
}
impl ::core::marker::Copy for MPR_SERVER_1 {}
impl ::core::clone::Clone for MPR_SERVER_1 {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for MPR_SERVER_1 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("MPR_SERVER_1").field("dwNumPptpPorts", &self.dwNumPptpPorts).field("dwPptpPortFlags", &self.dwPptpPortFlags).field("dwNumL2tpPorts", &self.dwNumL2tpPorts).field("dwL2tpPortFlags", &self.dwL2tpPortFlags).finish()
    }
}
unsafe impl ::windows::core::Abi for MPR_SERVER_1 {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for MPR_SERVER_1 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<MPR_SERVER_1>()) == 0 }
    }
}
impl ::core::cmp::Eq for MPR_SERVER_1 {}
impl ::core::default::Default for MPR_SERVER_1 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`*"]
pub struct MPR_SERVER_2 {
    pub dwNumPptpPorts: u32,
    pub dwPptpPortFlags: u32,
    pub dwNumL2tpPorts: u32,
    pub dwL2tpPortFlags: u32,
    pub dwNumSstpPorts: u32,
    pub dwSstpPortFlags: u32,
}
impl ::core::marker::Copy for MPR_SERVER_2 {}
impl ::core::clone::Clone for MPR_SERVER_2 {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for MPR_SERVER_2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("MPR_SERVER_2").field("dwNumPptpPorts", &self.dwNumPptpPorts).field("dwPptpPortFlags", &self.dwPptpPortFlags).field("dwNumL2tpPorts", &self.dwNumL2tpPorts).field("dwL2tpPortFlags", &self.dwL2tpPortFlags).field("dwNumSstpPorts", &self.dwNumSstpPorts).field("dwSstpPortFlags", &self.dwSstpPortFlags).finish()
    }
}
unsafe impl ::windows::core::Abi for MPR_SERVER_2 {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for MPR_SERVER_2 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<MPR_SERVER_2>()) == 0 }
    }
}
impl ::core::cmp::Eq for MPR_SERVER_2 {}
impl ::core::default::Default for MPR_SERVER_2 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`, `\"Win32_Foundation\"`, `\"Win32_Security_Cryptography\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography"))]
pub struct MPR_SERVER_EX0 {
    pub Header: MPRAPI_OBJECT_HEADER,
    pub fLanOnlyMode: u32,
    pub dwUpTime: u32,
    pub dwTotalPorts: u32,
    pub dwPortsInUse: u32,
    pub Reserved: u32,
    pub ConfigParams: MPRAPI_TUNNEL_CONFIG_PARAMS0,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography"))]
impl ::core::marker::Copy for MPR_SERVER_EX0 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography"))]
impl ::core::clone::Clone for MPR_SERVER_EX0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography"))]
impl ::core::fmt::Debug for MPR_SERVER_EX0 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("MPR_SERVER_EX0").field("Header", &self.Header).field("fLanOnlyMode", &self.fLanOnlyMode).field("dwUpTime", &self.dwUpTime).field("dwTotalPorts", &self.dwTotalPorts).field("dwPortsInUse", &self.dwPortsInUse).field("Reserved", &self.Reserved).field("ConfigParams", &self.ConfigParams).finish()
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography"))]
unsafe impl ::windows::core::Abi for MPR_SERVER_EX0 {
    type Abi = Self;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography"))]
impl ::core::cmp::PartialEq for MPR_SERVER_EX0 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<MPR_SERVER_EX0>()) == 0 }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography"))]
impl ::core::cmp::Eq for MPR_SERVER_EX0 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography"))]
impl ::core::default::Default for MPR_SERVER_EX0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`, `\"Win32_Foundation\"`, `\"Win32_Security_Cryptography\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography"))]
pub struct MPR_SERVER_EX1 {
    pub Header: MPRAPI_OBJECT_HEADER,
    pub fLanOnlyMode: u32,
    pub dwUpTime: u32,
    pub dwTotalPorts: u32,
    pub dwPortsInUse: u32,
    pub Reserved: u32,
    pub ConfigParams: MPRAPI_TUNNEL_CONFIG_PARAMS1,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography"))]
impl ::core::marker::Copy for MPR_SERVER_EX1 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography"))]
impl ::core::clone::Clone for MPR_SERVER_EX1 {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography"))]
impl ::core::fmt::Debug for MPR_SERVER_EX1 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("MPR_SERVER_EX1").field("Header", &self.Header).field("fLanOnlyMode", &self.fLanOnlyMode).field("dwUpTime", &self.dwUpTime).field("dwTotalPorts", &self.dwTotalPorts).field("dwPortsInUse", &self.dwPortsInUse).field("Reserved", &self.Reserved).field("ConfigParams", &self.ConfigParams).finish()
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography"))]
unsafe impl ::windows::core::Abi for MPR_SERVER_EX1 {
    type Abi = Self;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography"))]
impl ::core::cmp::PartialEq for MPR_SERVER_EX1 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<MPR_SERVER_EX1>()) == 0 }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography"))]
impl ::core::cmp::Eq for MPR_SERVER_EX1 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography"))]
impl ::core::default::Default for MPR_SERVER_EX1 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`, `\"Win32_Foundation\"`, `\"Win32_Security_Cryptography\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography"))]
pub struct MPR_SERVER_SET_CONFIG_EX0 {
    pub Header: MPRAPI_OBJECT_HEADER,
    pub setConfigForProtocols: u32,
    pub ConfigParams: MPRAPI_TUNNEL_CONFIG_PARAMS0,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography"))]
impl ::core::marker::Copy for MPR_SERVER_SET_CONFIG_EX0 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography"))]
impl ::core::clone::Clone for MPR_SERVER_SET_CONFIG_EX0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography"))]
impl ::core::fmt::Debug for MPR_SERVER_SET_CONFIG_EX0 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("MPR_SERVER_SET_CONFIG_EX0").field("Header", &self.Header).field("setConfigForProtocols", &self.setConfigForProtocols).field("ConfigParams", &self.ConfigParams).finish()
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography"))]
unsafe impl ::windows::core::Abi for MPR_SERVER_SET_CONFIG_EX0 {
    type Abi = Self;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography"))]
impl ::core::cmp::PartialEq for MPR_SERVER_SET_CONFIG_EX0 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<MPR_SERVER_SET_CONFIG_EX0>()) == 0 }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography"))]
impl ::core::cmp::Eq for MPR_SERVER_SET_CONFIG_EX0 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography"))]
impl ::core::default::Default for MPR_SERVER_SET_CONFIG_EX0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`, `\"Win32_Foundation\"`, `\"Win32_Security_Cryptography\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography"))]
pub struct MPR_SERVER_SET_CONFIG_EX1 {
    pub Header: MPRAPI_OBJECT_HEADER,
    pub setConfigForProtocols: u32,
    pub ConfigParams: MPRAPI_TUNNEL_CONFIG_PARAMS1,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography"))]
impl ::core::marker::Copy for MPR_SERVER_SET_CONFIG_EX1 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography"))]
impl ::core::clone::Clone for MPR_SERVER_SET_CONFIG_EX1 {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography"))]
impl ::core::fmt::Debug for MPR_SERVER_SET_CONFIG_EX1 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("MPR_SERVER_SET_CONFIG_EX1").field("Header", &self.Header).field("setConfigForProtocols", &self.setConfigForProtocols).field("ConfigParams", &self.ConfigParams).finish()
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography"))]
unsafe impl ::windows::core::Abi for MPR_SERVER_SET_CONFIG_EX1 {
    type Abi = Self;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography"))]
impl ::core::cmp::PartialEq for MPR_SERVER_SET_CONFIG_EX1 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<MPR_SERVER_SET_CONFIG_EX1>()) == 0 }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography"))]
impl ::core::cmp::Eq for MPR_SERVER_SET_CONFIG_EX1 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography"))]
impl ::core::default::Default for MPR_SERVER_SET_CONFIG_EX1 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct MPR_TRANSPORT_0 {
    pub dwTransportId: u32,
    pub hTransport: super::super::Foundation::HANDLE,
    pub wszTransportName: [u16; 41],
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for MPR_TRANSPORT_0 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for MPR_TRANSPORT_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for MPR_TRANSPORT_0 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("MPR_TRANSPORT_0").field("dwTransportId", &self.dwTransportId).field("hTransport", &self.hTransport).field("wszTransportName", &self.wszTransportName).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for MPR_TRANSPORT_0 {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for MPR_TRANSPORT_0 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<MPR_TRANSPORT_0>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for MPR_TRANSPORT_0 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for MPR_TRANSPORT_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`, `\"Win32_Networking_WinSock\"`*"]
#[cfg(feature = "Win32_Networking_WinSock")]
pub struct MPR_VPN_TRAFFIC_SELECTORS {
    pub numTsi: u32,
    pub numTsr: u32,
    pub tsI: *mut _MPR_VPN_SELECTOR,
    pub tsR: *mut _MPR_VPN_SELECTOR,
}
#[cfg(feature = "Win32_Networking_WinSock")]
impl ::core::marker::Copy for MPR_VPN_TRAFFIC_SELECTORS {}
#[cfg(feature = "Win32_Networking_WinSock")]
impl ::core::clone::Clone for MPR_VPN_TRAFFIC_SELECTORS {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Networking_WinSock")]
impl ::core::fmt::Debug for MPR_VPN_TRAFFIC_SELECTORS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("MPR_VPN_TRAFFIC_SELECTORS").field("numTsi", &self.numTsi).field("numTsr", &self.numTsr).field("tsI", &self.tsI).field("tsR", &self.tsR).finish()
    }
}
#[cfg(feature = "Win32_Networking_WinSock")]
unsafe impl ::windows::core::Abi for MPR_VPN_TRAFFIC_SELECTORS {
    type Abi = Self;
}
#[cfg(feature = "Win32_Networking_WinSock")]
impl ::core::cmp::PartialEq for MPR_VPN_TRAFFIC_SELECTORS {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<MPR_VPN_TRAFFIC_SELECTORS>()) == 0 }
    }
}
#[cfg(feature = "Win32_Networking_WinSock")]
impl ::core::cmp::Eq for MPR_VPN_TRAFFIC_SELECTORS {}
#[cfg(feature = "Win32_Networking_WinSock")]
impl ::core::default::Default for MPR_VPN_TRAFFIC_SELECTORS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct MPR_VPN_TS_TYPE(pub i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`*"]
pub const MPR_VPN_TS_IPv4_ADDR_RANGE: MPR_VPN_TS_TYPE = MPR_VPN_TS_TYPE(7i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`*"]
pub const MPR_VPN_TS_IPv6_ADDR_RANGE: MPR_VPN_TS_TYPE = MPR_VPN_TS_TYPE(8i32);
impl ::core::marker::Copy for MPR_VPN_TS_TYPE {}
impl ::core::clone::Clone for MPR_VPN_TS_TYPE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for MPR_VPN_TS_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for MPR_VPN_TS_TYPE {
    type Abi = Self;
}
impl ::core::fmt::Debug for MPR_VPN_TS_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MPR_VPN_TS_TYPE").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct MPR_VS(pub u32);
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`*"]
pub const MPR_VS_Default: MPR_VS = MPR_VS(0u32);
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`*"]
pub const MPR_VS_PptpOnly: MPR_VS = MPR_VS(1u32);
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`*"]
pub const MPR_VS_PptpFirst: MPR_VS = MPR_VS(2u32);
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`*"]
pub const MPR_VS_L2tpOnly: MPR_VS = MPR_VS(3u32);
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`*"]
pub const MPR_VS_L2tpFirst: MPR_VS = MPR_VS(4u32);
impl ::core::marker::Copy for MPR_VS {}
impl ::core::clone::Clone for MPR_VS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for MPR_VS {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for MPR_VS {
    type Abi = Self;
}
impl ::core::fmt::Debug for MPR_VS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MPR_VS").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`*"]
pub const MPR_VS_Ikev2First: u32 = 8u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`*"]
pub const MPR_VS_Ikev2Only: u32 = 7u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn MgmAddGroupMembershipEntry<'a, P0>(hprotocol: P0, dwsourceaddr: u32, dwsourcemask: u32, dwgroupaddr: u32, dwgroupmask: u32, dwifindex: u32, dwifnexthopipaddr: u32, dwflags: u32) -> u32
where
    P0: ::std::convert::Into<super::super::Foundation::HANDLE>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn MgmAddGroupMembershipEntry(hprotocol: super::super::Foundation::HANDLE, dwsourceaddr: u32, dwsourcemask: u32, dwgroupaddr: u32, dwgroupmask: u32, dwifindex: u32, dwifnexthopipaddr: u32, dwflags: u32) -> u32;
    }
    MgmAddGroupMembershipEntry(hprotocol.into(), dwsourceaddr, dwsourcemask, dwgroupaddr, dwgroupmask, dwifindex, dwifnexthopipaddr, dwflags)
}
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn MgmDeRegisterMProtocol<'a, P0>(hprotocol: P0) -> u32
where
    P0: ::std::convert::Into<super::super::Foundation::HANDLE>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn MgmDeRegisterMProtocol(hprotocol: super::super::Foundation::HANDLE) -> u32;
    }
    MgmDeRegisterMProtocol(hprotocol.into())
}
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn MgmDeleteGroupMembershipEntry<'a, P0>(hprotocol: P0, dwsourceaddr: u32, dwsourcemask: u32, dwgroupaddr: u32, dwgroupmask: u32, dwifindex: u32, dwifnexthopipaddr: u32, dwflags: u32) -> u32
where
    P0: ::std::convert::Into<super::super::Foundation::HANDLE>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn MgmDeleteGroupMembershipEntry(hprotocol: super::super::Foundation::HANDLE, dwsourceaddr: u32, dwsourcemask: u32, dwgroupaddr: u32, dwgroupmask: u32, dwifindex: u32, dwifnexthopipaddr: u32, dwflags: u32) -> u32;
    }
    MgmDeleteGroupMembershipEntry(hprotocol.into(), dwsourceaddr, dwsourcemask, dwgroupaddr, dwgroupmask, dwifindex, dwifnexthopipaddr, dwflags)
}
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`*"]
#[inline]
pub unsafe fn MgmGetFirstMfe(pdwbuffersize: *mut u32, pbbuffer: *mut u8, pdwnumentries: *mut u32) -> u32 {
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn MgmGetFirstMfe(pdwbuffersize: *mut u32, pbbuffer: *mut u8, pdwnumentries: *mut u32) -> u32;
    }
    MgmGetFirstMfe(::core::mem::transmute(pdwbuffersize), ::core::mem::transmute(pbbuffer), ::core::mem::transmute(pdwnumentries))
}
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`*"]
#[inline]
pub unsafe fn MgmGetFirstMfeStats(pdwbuffersize: *mut u32, pbbuffer: *mut u8, pdwnumentries: *mut u32, dwflags: u32) -> u32 {
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn MgmGetFirstMfeStats(pdwbuffersize: *mut u32, pbbuffer: *mut u8, pdwnumentries: *mut u32, dwflags: u32) -> u32;
    }
    MgmGetFirstMfeStats(::core::mem::transmute(pdwbuffersize), ::core::mem::transmute(pbbuffer), ::core::mem::transmute(pdwnumentries), dwflags)
}
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`, `\"Win32_NetworkManagement_IpHelper\"`*"]
#[cfg(feature = "Win32_NetworkManagement_IpHelper")]
#[inline]
pub unsafe fn MgmGetMfe(pimm: *mut super::IpHelper::MIB_IPMCAST_MFE, pdwbuffersize: *mut u32, pbbuffer: *mut u8) -> u32 {
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn MgmGetMfe(pimm: *mut super::IpHelper::MIB_IPMCAST_MFE, pdwbuffersize: *mut u32, pbbuffer: *mut u8) -> u32;
    }
    MgmGetMfe(::core::mem::transmute(pimm), ::core::mem::transmute(pdwbuffersize), ::core::mem::transmute(pbbuffer))
}
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`, `\"Win32_NetworkManagement_IpHelper\"`*"]
#[cfg(feature = "Win32_NetworkManagement_IpHelper")]
#[inline]
pub unsafe fn MgmGetMfeStats(pimm: *mut super::IpHelper::MIB_IPMCAST_MFE, pdwbuffersize: *mut u32, pbbuffer: *mut u8, dwflags: u32) -> u32 {
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn MgmGetMfeStats(pimm: *mut super::IpHelper::MIB_IPMCAST_MFE, pdwbuffersize: *mut u32, pbbuffer: *mut u8, dwflags: u32) -> u32;
    }
    MgmGetMfeStats(::core::mem::transmute(pimm), ::core::mem::transmute(pdwbuffersize), ::core::mem::transmute(pbbuffer), dwflags)
}
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`, `\"Win32_NetworkManagement_IpHelper\"`*"]
#[cfg(feature = "Win32_NetworkManagement_IpHelper")]
#[inline]
pub unsafe fn MgmGetNextMfe(pimmstart: *mut super::IpHelper::MIB_IPMCAST_MFE, pdwbuffersize: *mut u32, pbbuffer: *mut u8, pdwnumentries: *mut u32) -> u32 {
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn MgmGetNextMfe(pimmstart: *mut super::IpHelper::MIB_IPMCAST_MFE, pdwbuffersize: *mut u32, pbbuffer: *mut u8, pdwnumentries: *mut u32) -> u32;
    }
    MgmGetNextMfe(::core::mem::transmute(pimmstart), ::core::mem::transmute(pdwbuffersize), ::core::mem::transmute(pbbuffer), ::core::mem::transmute(pdwnumentries))
}
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`, `\"Win32_NetworkManagement_IpHelper\"`*"]
#[cfg(feature = "Win32_NetworkManagement_IpHelper")]
#[inline]
pub unsafe fn MgmGetNextMfeStats(pimmstart: *mut super::IpHelper::MIB_IPMCAST_MFE, pdwbuffersize: *mut u32, pbbuffer: *mut u8, pdwnumentries: *mut u32, dwflags: u32) -> u32 {
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn MgmGetNextMfeStats(pimmstart: *mut super::IpHelper::MIB_IPMCAST_MFE, pdwbuffersize: *mut u32, pbbuffer: *mut u8, pdwnumentries: *mut u32, dwflags: u32) -> u32;
    }
    MgmGetNextMfeStats(::core::mem::transmute(pimmstart), ::core::mem::transmute(pdwbuffersize), ::core::mem::transmute(pbbuffer), ::core::mem::transmute(pdwnumentries), dwflags)
}
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`*"]
#[inline]
pub unsafe fn MgmGetProtocolOnInterface(dwifindex: u32, dwifnexthopaddr: u32, pdwifprotocolid: *mut u32, pdwifcomponentid: *mut u32) -> u32 {
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn MgmGetProtocolOnInterface(dwifindex: u32, dwifnexthopaddr: u32, pdwifprotocolid: *mut u32, pdwifcomponentid: *mut u32) -> u32;
    }
    MgmGetProtocolOnInterface(dwifindex, dwifnexthopaddr, ::core::mem::transmute(pdwifprotocolid), ::core::mem::transmute(pdwifcomponentid))
}
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn MgmGroupEnumerationEnd<'a, P0>(henum: P0) -> u32
where
    P0: ::std::convert::Into<super::super::Foundation::HANDLE>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn MgmGroupEnumerationEnd(henum: super::super::Foundation::HANDLE) -> u32;
    }
    MgmGroupEnumerationEnd(henum.into())
}
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn MgmGroupEnumerationGetNext<'a, P0>(henum: P0, pdwbuffersize: *mut u32, pbbuffer: *mut u8, pdwnumentries: *mut u32) -> u32
where
    P0: ::std::convert::Into<super::super::Foundation::HANDLE>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn MgmGroupEnumerationGetNext(henum: super::super::Foundation::HANDLE, pdwbuffersize: *mut u32, pbbuffer: *mut u8, pdwnumentries: *mut u32) -> u32;
    }
    MgmGroupEnumerationGetNext(henum.into(), ::core::mem::transmute(pdwbuffersize), ::core::mem::transmute(pbbuffer), ::core::mem::transmute(pdwnumentries))
}
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn MgmGroupEnumerationStart<'a, P0>(hprotocol: P0, metenumtype: MGM_ENUM_TYPES, phenumhandle: *mut super::super::Foundation::HANDLE) -> u32
where
    P0: ::std::convert::Into<super::super::Foundation::HANDLE>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn MgmGroupEnumerationStart(hprotocol: super::super::Foundation::HANDLE, metenumtype: MGM_ENUM_TYPES, phenumhandle: *mut super::super::Foundation::HANDLE) -> u32;
    }
    MgmGroupEnumerationStart(hprotocol.into(), metenumtype, ::core::mem::transmute(phenumhandle))
}
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn MgmRegisterMProtocol(prpiinfo: *mut ROUTING_PROTOCOL_CONFIG, dwprotocolid: u32, dwcomponentid: u32, phprotocol: *mut super::super::Foundation::HANDLE) -> u32 {
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn MgmRegisterMProtocol(prpiinfo: *mut ROUTING_PROTOCOL_CONFIG, dwprotocolid: u32, dwcomponentid: u32, phprotocol: *mut super::super::Foundation::HANDLE) -> u32;
    }
    MgmRegisterMProtocol(::core::mem::transmute(prpiinfo), dwprotocolid, dwcomponentid, ::core::mem::transmute(phprotocol))
}
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn MgmReleaseInterfaceOwnership<'a, P0>(hprotocol: P0, dwifindex: u32, dwifnexthopaddr: u32) -> u32
where
    P0: ::std::convert::Into<super::super::Foundation::HANDLE>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn MgmReleaseInterfaceOwnership(hprotocol: super::super::Foundation::HANDLE, dwifindex: u32, dwifnexthopaddr: u32) -> u32;
    }
    MgmReleaseInterfaceOwnership(hprotocol.into(), dwifindex, dwifnexthopaddr)
}
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn MgmTakeInterfaceOwnership<'a, P0>(hprotocol: P0, dwifindex: u32, dwifnexthopaddr: u32) -> u32
where
    P0: ::std::convert::Into<super::super::Foundation::HANDLE>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn MgmTakeInterfaceOwnership(hprotocol: super::super::Foundation::HANDLE, dwifindex: u32, dwifnexthopaddr: u32) -> u32;
    }
    MgmTakeInterfaceOwnership(hprotocol.into(), dwifindex, dwifnexthopaddr)
}
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`*"]
#[inline]
pub unsafe fn MprAdminBufferFree(pbuffer: *const ::core::ffi::c_void) -> u32 {
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn MprAdminBufferFree(pbuffer: *const ::core::ffi::c_void) -> u32;
    }
    MprAdminBufferFree(::core::mem::transmute(pbuffer))
}
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn MprAdminConnectionClearStats<'a, P0>(hrasserver: isize, hrasconnection: P0) -> u32
where
    P0: ::std::convert::Into<super::super::Foundation::HANDLE>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn MprAdminConnectionClearStats(hrasserver: isize, hrasconnection: super::super::Foundation::HANDLE) -> u32;
    }
    MprAdminConnectionClearStats(hrasserver, hrasconnection.into())
}
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`*"]
#[inline]
pub unsafe fn MprAdminConnectionEnum(hrasserver: isize, dwlevel: u32, lplpbbuffer: *mut *mut u8, dwprefmaxlen: u32, lpdwentriesread: *mut u32, lpdwtotalentries: *mut u32, lpdwresumehandle: *const u32) -> u32 {
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn MprAdminConnectionEnum(hrasserver: isize, dwlevel: u32, lplpbbuffer: *mut *mut u8, dwprefmaxlen: u32, lpdwentriesread: *mut u32, lpdwtotalentries: *mut u32, lpdwresumehandle: *const u32) -> u32;
    }
    MprAdminConnectionEnum(hrasserver, dwlevel, ::core::mem::transmute(lplpbbuffer), dwprefmaxlen, ::core::mem::transmute(lpdwentriesread), ::core::mem::transmute(lpdwtotalentries), ::core::mem::transmute(lpdwresumehandle))
}
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn MprAdminConnectionEnumEx(hrasserver: isize, pobjectheader: *const MPRAPI_OBJECT_HEADER, dwpreferedmaxlen: u32, lpdwentriesread: *mut u32, lpdwtotalentries: *mut u32, pprasconn: *mut *mut RAS_CONNECTION_EX, lpdwresumehandle: *const u32) -> u32 {
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn MprAdminConnectionEnumEx(hrasserver: isize, pobjectheader: *const MPRAPI_OBJECT_HEADER, dwpreferedmaxlen: u32, lpdwentriesread: *mut u32, lpdwtotalentries: *mut u32, pprasconn: *mut *mut RAS_CONNECTION_EX, lpdwresumehandle: *const u32) -> u32;
    }
    MprAdminConnectionEnumEx(hrasserver, ::core::mem::transmute(pobjectheader), dwpreferedmaxlen, ::core::mem::transmute(lpdwentriesread), ::core::mem::transmute(lpdwtotalentries), ::core::mem::transmute(pprasconn), ::core::mem::transmute(lpdwresumehandle))
}
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn MprAdminConnectionGetInfo<'a, P0>(hrasserver: isize, dwlevel: u32, hrasconnection: P0, lplpbbuffer: *mut *mut u8) -> u32
where
    P0: ::std::convert::Into<super::super::Foundation::HANDLE>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn MprAdminConnectionGetInfo(hrasserver: isize, dwlevel: u32, hrasconnection: super::super::Foundation::HANDLE, lplpbbuffer: *mut *mut u8) -> u32;
    }
    MprAdminConnectionGetInfo(hrasserver, dwlevel, hrasconnection.into(), ::core::mem::transmute(lplpbbuffer))
}
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn MprAdminConnectionGetInfoEx<'a, P0>(hrasserver: isize, hrasconnection: P0, prasconnection: *mut RAS_CONNECTION_EX) -> u32
where
    P0: ::std::convert::Into<super::super::Foundation::HANDLE>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn MprAdminConnectionGetInfoEx(hrasserver: isize, hrasconnection: super::super::Foundation::HANDLE, prasconnection: *mut RAS_CONNECTION_EX) -> u32;
    }
    MprAdminConnectionGetInfoEx(hrasserver, hrasconnection.into(), ::core::mem::transmute(prasconnection))
}
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn MprAdminConnectionRemoveQuarantine<'a, P0, P1, P2>(hrasserver: P0, hrasconnection: P1, fisipaddress: P2) -> u32
where
    P0: ::std::convert::Into<super::super::Foundation::HANDLE>,
    P1: ::std::convert::Into<super::super::Foundation::HANDLE>,
    P2: ::std::convert::Into<super::super::Foundation::BOOL>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn MprAdminConnectionRemoveQuarantine(hrasserver: super::super::Foundation::HANDLE, hrasconnection: super::super::Foundation::HANDLE, fisipaddress: super::super::Foundation::BOOL) -> u32;
    }
    MprAdminConnectionRemoveQuarantine(hrasserver.into(), hrasconnection.into(), fisipaddress.into())
}
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn MprAdminDeregisterConnectionNotification<'a, P0>(hmprserver: isize, heventnotification: P0) -> u32
where
    P0: ::std::convert::Into<super::super::Foundation::HANDLE>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn MprAdminDeregisterConnectionNotification(hmprserver: isize, heventnotification: super::super::Foundation::HANDLE) -> u32;
    }
    MprAdminDeregisterConnectionNotification(hmprserver, heventnotification.into())
}
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`*"]
#[inline]
pub unsafe fn MprAdminDeviceEnum(hmprserver: isize, dwlevel: u32, lplpbbuffer: *mut *mut u8, lpdwtotalentries: *mut u32) -> u32 {
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn MprAdminDeviceEnum(hmprserver: isize, dwlevel: u32, lplpbbuffer: *mut *mut u8, lpdwtotalentries: *mut u32) -> u32;
    }
    MprAdminDeviceEnum(hmprserver, dwlevel, ::core::mem::transmute(lplpbbuffer), ::core::mem::transmute(lpdwtotalentries))
}
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn MprAdminEstablishDomainRasServer<'a, P0, P1, P2>(pszdomain: P0, pszmachine: P1, benable: P2) -> u32
where
    P0: ::std::convert::Into<::windows::core::PCWSTR>,
    P1: ::std::convert::Into<::windows::core::PCWSTR>,
    P2: ::std::convert::Into<super::super::Foundation::BOOL>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn MprAdminEstablishDomainRasServer(pszdomain: ::windows::core::PCWSTR, pszmachine: ::windows::core::PCWSTR, benable: super::super::Foundation::BOOL) -> u32;
    }
    MprAdminEstablishDomainRasServer(pszdomain.into(), pszmachine.into(), benable.into())
}
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`*"]
#[inline]
pub unsafe fn MprAdminGetErrorString(dwerror: u32, lplpwserrorstring: *mut ::windows::core::PWSTR) -> u32 {
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn MprAdminGetErrorString(dwerror: u32, lplpwserrorstring: *mut ::windows::core::PWSTR) -> u32;
    }
    MprAdminGetErrorString(dwerror, ::core::mem::transmute(lplpwserrorstring))
}
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`*"]
#[inline]
pub unsafe fn MprAdminGetPDCServer<'a, P0, P1>(lpszdomain: P0, lpszserver: P1, lpszpdcserver: ::windows::core::PWSTR) -> u32
where
    P0: ::std::convert::Into<::windows::core::PCWSTR>,
    P1: ::std::convert::Into<::windows::core::PCWSTR>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn MprAdminGetPDCServer(lpszdomain: ::windows::core::PCWSTR, lpszserver: ::windows::core::PCWSTR, lpszpdcserver: ::windows::core::PWSTR) -> u32;
    }
    MprAdminGetPDCServer(lpszdomain.into(), lpszserver.into(), ::core::mem::transmute(lpszpdcserver))
}
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn MprAdminInterfaceConnect<'a, P0, P1, P2>(hmprserver: isize, hinterface: P0, hevent: P1, fsynchronous: P2) -> u32
where
    P0: ::std::convert::Into<super::super::Foundation::HANDLE>,
    P1: ::std::convert::Into<super::super::Foundation::HANDLE>,
    P2: ::std::convert::Into<super::super::Foundation::BOOL>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn MprAdminInterfaceConnect(hmprserver: isize, hinterface: super::super::Foundation::HANDLE, hevent: super::super::Foundation::HANDLE, fsynchronous: super::super::Foundation::BOOL) -> u32;
    }
    MprAdminInterfaceConnect(hmprserver, hinterface.into(), hevent.into(), fsynchronous.into())
}
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn MprAdminInterfaceCreate(hmprserver: isize, dwlevel: u32, lpbbuffer: *const u8, phinterface: *mut super::super::Foundation::HANDLE) -> u32 {
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn MprAdminInterfaceCreate(hmprserver: isize, dwlevel: u32, lpbbuffer: *const u8, phinterface: *mut super::super::Foundation::HANDLE) -> u32;
    }
    MprAdminInterfaceCreate(hmprserver, dwlevel, ::core::mem::transmute(lpbbuffer), ::core::mem::transmute(phinterface))
}
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn MprAdminInterfaceDelete<'a, P0>(hmprserver: isize, hinterface: P0) -> u32
where
    P0: ::std::convert::Into<super::super::Foundation::HANDLE>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn MprAdminInterfaceDelete(hmprserver: isize, hinterface: super::super::Foundation::HANDLE) -> u32;
    }
    MprAdminInterfaceDelete(hmprserver, hinterface.into())
}
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn MprAdminInterfaceDeviceGetInfo<'a, P0>(hmprserver: isize, hinterface: P0, dwindex: u32, dwlevel: u32, lplpbuffer: *mut *mut u8) -> u32
where
    P0: ::std::convert::Into<super::super::Foundation::HANDLE>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn MprAdminInterfaceDeviceGetInfo(hmprserver: isize, hinterface: super::super::Foundation::HANDLE, dwindex: u32, dwlevel: u32, lplpbuffer: *mut *mut u8) -> u32;
    }
    MprAdminInterfaceDeviceGetInfo(hmprserver, hinterface.into(), dwindex, dwlevel, ::core::mem::transmute(lplpbuffer))
}
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn MprAdminInterfaceDeviceSetInfo<'a, P0>(hmprserver: isize, hinterface: P0, dwindex: u32, dwlevel: u32, lpbbuffer: *const u8) -> u32
where
    P0: ::std::convert::Into<super::super::Foundation::HANDLE>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn MprAdminInterfaceDeviceSetInfo(hmprserver: isize, hinterface: super::super::Foundation::HANDLE, dwindex: u32, dwlevel: u32, lpbbuffer: *const u8) -> u32;
    }
    MprAdminInterfaceDeviceSetInfo(hmprserver, hinterface.into(), dwindex, dwlevel, ::core::mem::transmute(lpbbuffer))
}
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn MprAdminInterfaceDisconnect<'a, P0>(hmprserver: isize, hinterface: P0) -> u32
where
    P0: ::std::convert::Into<super::super::Foundation::HANDLE>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn MprAdminInterfaceDisconnect(hmprserver: isize, hinterface: super::super::Foundation::HANDLE) -> u32;
    }
    MprAdminInterfaceDisconnect(hmprserver, hinterface.into())
}
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`*"]
#[inline]
pub unsafe fn MprAdminInterfaceEnum(hmprserver: isize, dwlevel: u32, lplpbbuffer: *mut *mut u8, dwprefmaxlen: u32, lpdwentriesread: *mut u32, lpdwtotalentries: *mut u32, lpdwresumehandle: *const u32) -> u32 {
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn MprAdminInterfaceEnum(hmprserver: isize, dwlevel: u32, lplpbbuffer: *mut *mut u8, dwprefmaxlen: u32, lpdwentriesread: *mut u32, lpdwtotalentries: *mut u32, lpdwresumehandle: *const u32) -> u32;
    }
    MprAdminInterfaceEnum(hmprserver, dwlevel, ::core::mem::transmute(lplpbbuffer), dwprefmaxlen, ::core::mem::transmute(lpdwentriesread), ::core::mem::transmute(lpdwtotalentries), ::core::mem::transmute(lpdwresumehandle))
}
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`*"]
#[inline]
pub unsafe fn MprAdminInterfaceGetCredentials<'a, P0, P1>(lpwsserver: P0, lpwsinterfacename: P1, lpwsusername: ::windows::core::PWSTR, lpwspassword: ::windows::core::PWSTR, lpwsdomainname: ::windows::core::PWSTR) -> u32
where
    P0: ::std::convert::Into<::windows::core::PCWSTR>,
    P1: ::std::convert::Into<::windows::core::PCWSTR>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn MprAdminInterfaceGetCredentials(lpwsserver: ::windows::core::PCWSTR, lpwsinterfacename: ::windows::core::PCWSTR, lpwsusername: ::windows::core::PWSTR, lpwspassword: ::windows::core::PWSTR, lpwsdomainname: ::windows::core::PWSTR) -> u32;
    }
    MprAdminInterfaceGetCredentials(lpwsserver.into(), lpwsinterfacename.into(), ::core::mem::transmute(lpwsusername), ::core::mem::transmute(lpwspassword), ::core::mem::transmute(lpwsdomainname))
}
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn MprAdminInterfaceGetCredentialsEx<'a, P0>(hmprserver: isize, hinterface: P0, dwlevel: u32, lplpbbuffer: *mut *mut u8) -> u32
where
    P0: ::std::convert::Into<super::super::Foundation::HANDLE>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn MprAdminInterfaceGetCredentialsEx(hmprserver: isize, hinterface: super::super::Foundation::HANDLE, dwlevel: u32, lplpbbuffer: *mut *mut u8) -> u32;
    }
    MprAdminInterfaceGetCredentialsEx(hmprserver, hinterface.into(), dwlevel, ::core::mem::transmute(lplpbbuffer))
}
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`, `\"Win32_Foundation\"`, `\"Win32_Networking_WinSock\"`, `\"Win32_Security_Cryptography\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock", feature = "Win32_Security_Cryptography"))]
#[inline]
pub unsafe fn MprAdminInterfaceGetCustomInfoEx<'a, P0>(hmprserver: isize, hinterface: P0, pcustominfo: *mut MPR_IF_CUSTOMINFOEX2) -> u32
where
    P0: ::std::convert::Into<super::super::Foundation::HANDLE>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn MprAdminInterfaceGetCustomInfoEx(hmprserver: isize, hinterface: super::super::Foundation::HANDLE, pcustominfo: *mut MPR_IF_CUSTOMINFOEX2) -> u32;
    }
    MprAdminInterfaceGetCustomInfoEx(hmprserver, hinterface.into(), ::core::mem::transmute(pcustominfo))
}
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn MprAdminInterfaceGetHandle<'a, P0, P1>(hmprserver: isize, lpwsinterfacename: P0, phinterface: *mut super::super::Foundation::HANDLE, fincludeclientinterfaces: P1) -> u32
where
    P0: ::std::convert::Into<::windows::core::PCWSTR>,
    P1: ::std::convert::Into<super::super::Foundation::BOOL>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn MprAdminInterfaceGetHandle(hmprserver: isize, lpwsinterfacename: ::windows::core::PCWSTR, phinterface: *mut super::super::Foundation::HANDLE, fincludeclientinterfaces: super::super::Foundation::BOOL) -> u32;
    }
    MprAdminInterfaceGetHandle(hmprserver, lpwsinterfacename.into(), ::core::mem::transmute(phinterface), fincludeclientinterfaces.into())
}
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn MprAdminInterfaceGetInfo<'a, P0>(hmprserver: isize, hinterface: P0, dwlevel: u32, lplpbbuffer: *const *const u8) -> u32
where
    P0: ::std::convert::Into<super::super::Foundation::HANDLE>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn MprAdminInterfaceGetInfo(hmprserver: isize, hinterface: super::super::Foundation::HANDLE, dwlevel: u32, lplpbbuffer: *const *const u8) -> u32;
    }
    MprAdminInterfaceGetInfo(hmprserver, hinterface.into(), dwlevel, ::core::mem::transmute(lplpbbuffer))
}
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn MprAdminInterfaceQueryUpdateResult<'a, P0>(hmprserver: isize, hinterface: P0, dwprotocolid: u32, lpdwupdateresult: *mut u32) -> u32
where
    P0: ::std::convert::Into<super::super::Foundation::HANDLE>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn MprAdminInterfaceQueryUpdateResult(hmprserver: isize, hinterface: super::super::Foundation::HANDLE, dwprotocolid: u32, lpdwupdateresult: *mut u32) -> u32;
    }
    MprAdminInterfaceQueryUpdateResult(hmprserver, hinterface.into(), dwprotocolid, ::core::mem::transmute(lpdwupdateresult))
}
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`*"]
#[inline]
pub unsafe fn MprAdminInterfaceSetCredentials<'a, P0, P1, P2, P3, P4>(lpwsserver: P0, lpwsinterfacename: P1, lpwsusername: P2, lpwsdomainname: P3, lpwspassword: P4) -> u32
where
    P0: ::std::convert::Into<::windows::core::PCWSTR>,
    P1: ::std::convert::Into<::windows::core::PCWSTR>,
    P2: ::std::convert::Into<::windows::core::PCWSTR>,
    P3: ::std::convert::Into<::windows::core::PCWSTR>,
    P4: ::std::convert::Into<::windows::core::PCWSTR>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn MprAdminInterfaceSetCredentials(lpwsserver: ::windows::core::PCWSTR, lpwsinterfacename: ::windows::core::PCWSTR, lpwsusername: ::windows::core::PCWSTR, lpwsdomainname: ::windows::core::PCWSTR, lpwspassword: ::windows::core::PCWSTR) -> u32;
    }
    MprAdminInterfaceSetCredentials(lpwsserver.into(), lpwsinterfacename.into(), lpwsusername.into(), lpwsdomainname.into(), lpwspassword.into())
}
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn MprAdminInterfaceSetCredentialsEx<'a, P0>(hmprserver: isize, hinterface: P0, dwlevel: u32, lpbbuffer: *const u8) -> u32
where
    P0: ::std::convert::Into<super::super::Foundation::HANDLE>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn MprAdminInterfaceSetCredentialsEx(hmprserver: isize, hinterface: super::super::Foundation::HANDLE, dwlevel: u32, lpbbuffer: *const u8) -> u32;
    }
    MprAdminInterfaceSetCredentialsEx(hmprserver, hinterface.into(), dwlevel, ::core::mem::transmute(lpbbuffer))
}
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`, `\"Win32_Foundation\"`, `\"Win32_Networking_WinSock\"`, `\"Win32_Security_Cryptography\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock", feature = "Win32_Security_Cryptography"))]
#[inline]
pub unsafe fn MprAdminInterfaceSetCustomInfoEx<'a, P0>(hmprserver: isize, hinterface: P0, pcustominfo: *const MPR_IF_CUSTOMINFOEX2) -> u32
where
    P0: ::std::convert::Into<super::super::Foundation::HANDLE>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn MprAdminInterfaceSetCustomInfoEx(hmprserver: isize, hinterface: super::super::Foundation::HANDLE, pcustominfo: *const MPR_IF_CUSTOMINFOEX2) -> u32;
    }
    MprAdminInterfaceSetCustomInfoEx(hmprserver, hinterface.into(), ::core::mem::transmute(pcustominfo))
}
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn MprAdminInterfaceSetInfo<'a, P0>(hmprserver: isize, hinterface: P0, dwlevel: u32, lpbbuffer: *const u8) -> u32
where
    P0: ::std::convert::Into<super::super::Foundation::HANDLE>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn MprAdminInterfaceSetInfo(hmprserver: isize, hinterface: super::super::Foundation::HANDLE, dwlevel: u32, lpbbuffer: *const u8) -> u32;
    }
    MprAdminInterfaceSetInfo(hmprserver, hinterface.into(), dwlevel, ::core::mem::transmute(lpbbuffer))
}
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn MprAdminInterfaceTransportAdd<'a, P0>(hmprserver: isize, hinterface: P0, dwtransportid: u32, pinterfaceinfo: *const u8, dwinterfaceinfosize: u32) -> u32
where
    P0: ::std::convert::Into<super::super::Foundation::HANDLE>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn MprAdminInterfaceTransportAdd(hmprserver: isize, hinterface: super::super::Foundation::HANDLE, dwtransportid: u32, pinterfaceinfo: *const u8, dwinterfaceinfosize: u32) -> u32;
    }
    MprAdminInterfaceTransportAdd(hmprserver, hinterface.into(), dwtransportid, ::core::mem::transmute(pinterfaceinfo), dwinterfaceinfosize)
}
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn MprAdminInterfaceTransportGetInfo<'a, P0>(hmprserver: isize, hinterface: P0, dwtransportid: u32, ppinterfaceinfo: *mut *mut u8, lpdwinterfaceinfosize: *mut u32) -> u32
where
    P0: ::std::convert::Into<super::super::Foundation::HANDLE>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn MprAdminInterfaceTransportGetInfo(hmprserver: isize, hinterface: super::super::Foundation::HANDLE, dwtransportid: u32, ppinterfaceinfo: *mut *mut u8, lpdwinterfaceinfosize: *mut u32) -> u32;
    }
    MprAdminInterfaceTransportGetInfo(hmprserver, hinterface.into(), dwtransportid, ::core::mem::transmute(ppinterfaceinfo), ::core::mem::transmute(lpdwinterfaceinfosize))
}
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn MprAdminInterfaceTransportRemove<'a, P0>(hmprserver: isize, hinterface: P0, dwtransportid: u32) -> u32
where
    P0: ::std::convert::Into<super::super::Foundation::HANDLE>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn MprAdminInterfaceTransportRemove(hmprserver: isize, hinterface: super::super::Foundation::HANDLE, dwtransportid: u32) -> u32;
    }
    MprAdminInterfaceTransportRemove(hmprserver, hinterface.into(), dwtransportid)
}
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn MprAdminInterfaceTransportSetInfo<'a, P0>(hmprserver: isize, hinterface: P0, dwtransportid: u32, pinterfaceinfo: *const u8, dwinterfaceinfosize: u32) -> u32
where
    P0: ::std::convert::Into<super::super::Foundation::HANDLE>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn MprAdminInterfaceTransportSetInfo(hmprserver: isize, hinterface: super::super::Foundation::HANDLE, dwtransportid: u32, pinterfaceinfo: *const u8, dwinterfaceinfosize: u32) -> u32;
    }
    MprAdminInterfaceTransportSetInfo(hmprserver, hinterface.into(), dwtransportid, ::core::mem::transmute(pinterfaceinfo), dwinterfaceinfosize)
}
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn MprAdminInterfaceUpdatePhonebookInfo<'a, P0>(hmprserver: isize, hinterface: P0) -> u32
where
    P0: ::std::convert::Into<super::super::Foundation::HANDLE>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn MprAdminInterfaceUpdatePhonebookInfo(hmprserver: isize, hinterface: super::super::Foundation::HANDLE) -> u32;
    }
    MprAdminInterfaceUpdatePhonebookInfo(hmprserver, hinterface.into())
}
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn MprAdminInterfaceUpdateRoutes<'a, P0, P1>(hmprserver: isize, hinterface: P0, dwprotocolid: u32, hevent: P1) -> u32
where
    P0: ::std::convert::Into<super::super::Foundation::HANDLE>,
    P1: ::std::convert::Into<super::super::Foundation::HANDLE>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn MprAdminInterfaceUpdateRoutes(hmprserver: isize, hinterface: super::super::Foundation::HANDLE, dwprotocolid: u32, hevent: super::super::Foundation::HANDLE) -> u32;
    }
    MprAdminInterfaceUpdateRoutes(hmprserver, hinterface.into(), dwprotocolid, hevent.into())
}
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn MprAdminIsDomainRasServer<'a, P0, P1>(pszdomain: P0, pszmachine: P1, pbisrasserver: *mut super::super::Foundation::BOOL) -> u32
where
    P0: ::std::convert::Into<::windows::core::PCWSTR>,
    P1: ::std::convert::Into<::windows::core::PCWSTR>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn MprAdminIsDomainRasServer(pszdomain: ::windows::core::PCWSTR, pszmachine: ::windows::core::PCWSTR, pbisrasserver: *mut super::super::Foundation::BOOL) -> u32;
    }
    MprAdminIsDomainRasServer(pszdomain.into(), pszmachine.into(), ::core::mem::transmute(pbisrasserver))
}
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn MprAdminIsServiceInitialized<'a, P0>(lpwsservername: P0, fisserviceinitialized: *const super::super::Foundation::BOOL) -> u32
where
    P0: ::std::convert::Into<::windows::core::PCWSTR>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn MprAdminIsServiceInitialized(lpwsservername: ::windows::core::PCWSTR, fisserviceinitialized: *const super::super::Foundation::BOOL) -> u32;
    }
    MprAdminIsServiceInitialized(lpwsservername.into(), ::core::mem::transmute(fisserviceinitialized))
}
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn MprAdminIsServiceRunning<'a, P0>(lpwsservername: P0) -> super::super::Foundation::BOOL
where
    P0: ::std::convert::Into<::windows::core::PCWSTR>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn MprAdminIsServiceRunning(lpwsservername: ::windows::core::PCWSTR) -> super::super::Foundation::BOOL;
    }
    MprAdminIsServiceRunning(lpwsservername.into())
}
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`*"]
#[inline]
pub unsafe fn MprAdminMIBBufferFree(pbuffer: *const ::core::ffi::c_void) -> u32 {
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn MprAdminMIBBufferFree(pbuffer: *const ::core::ffi::c_void) -> u32;
    }
    MprAdminMIBBufferFree(::core::mem::transmute(pbuffer))
}
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`*"]
#[inline]
pub unsafe fn MprAdminMIBEntryCreate(hmibserver: isize, dwpid: u32, dwroutingpid: u32, lpentry: *const ::core::ffi::c_void, dwentrysize: u32) -> u32 {
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn MprAdminMIBEntryCreate(hmibserver: isize, dwpid: u32, dwroutingpid: u32, lpentry: *const ::core::ffi::c_void, dwentrysize: u32) -> u32;
    }
    MprAdminMIBEntryCreate(hmibserver, dwpid, dwroutingpid, ::core::mem::transmute(lpentry), dwentrysize)
}
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`*"]
#[inline]
pub unsafe fn MprAdminMIBEntryDelete(hmibserver: isize, dwprotocolid: u32, dwroutingpid: u32, lpentry: *const ::core::ffi::c_void, dwentrysize: u32) -> u32 {
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn MprAdminMIBEntryDelete(hmibserver: isize, dwprotocolid: u32, dwroutingpid: u32, lpentry: *const ::core::ffi::c_void, dwentrysize: u32) -> u32;
    }
    MprAdminMIBEntryDelete(hmibserver, dwprotocolid, dwroutingpid, ::core::mem::transmute(lpentry), dwentrysize)
}
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`*"]
#[inline]
pub unsafe fn MprAdminMIBEntryGet(hmibserver: isize, dwprotocolid: u32, dwroutingpid: u32, lpinentry: *const ::core::ffi::c_void, dwinentrysize: u32, lplpoutentry: *mut *mut ::core::ffi::c_void, lpoutentrysize: *mut u32) -> u32 {
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn MprAdminMIBEntryGet(hmibserver: isize, dwprotocolid: u32, dwroutingpid: u32, lpinentry: *const ::core::ffi::c_void, dwinentrysize: u32, lplpoutentry: *mut *mut ::core::ffi::c_void, lpoutentrysize: *mut u32) -> u32;
    }
    MprAdminMIBEntryGet(hmibserver, dwprotocolid, dwroutingpid, ::core::mem::transmute(lpinentry), dwinentrysize, ::core::mem::transmute(lplpoutentry), ::core::mem::transmute(lpoutentrysize))
}
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`*"]
#[inline]
pub unsafe fn MprAdminMIBEntryGetFirst(hmibserver: isize, dwprotocolid: u32, dwroutingpid: u32, lpinentry: *const ::core::ffi::c_void, dwinentrysize: u32, lplpoutentry: *mut *mut ::core::ffi::c_void, lpoutentrysize: *mut u32) -> u32 {
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn MprAdminMIBEntryGetFirst(hmibserver: isize, dwprotocolid: u32, dwroutingpid: u32, lpinentry: *const ::core::ffi::c_void, dwinentrysize: u32, lplpoutentry: *mut *mut ::core::ffi::c_void, lpoutentrysize: *mut u32) -> u32;
    }
    MprAdminMIBEntryGetFirst(hmibserver, dwprotocolid, dwroutingpid, ::core::mem::transmute(lpinentry), dwinentrysize, ::core::mem::transmute(lplpoutentry), ::core::mem::transmute(lpoutentrysize))
}
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`*"]
#[inline]
pub unsafe fn MprAdminMIBEntryGetNext(hmibserver: isize, dwprotocolid: u32, dwroutingpid: u32, lpinentry: *const ::core::ffi::c_void, dwinentrysize: u32, lplpoutentry: *mut *mut ::core::ffi::c_void, lpoutentrysize: *mut u32) -> u32 {
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn MprAdminMIBEntryGetNext(hmibserver: isize, dwprotocolid: u32, dwroutingpid: u32, lpinentry: *const ::core::ffi::c_void, dwinentrysize: u32, lplpoutentry: *mut *mut ::core::ffi::c_void, lpoutentrysize: *mut u32) -> u32;
    }
    MprAdminMIBEntryGetNext(hmibserver, dwprotocolid, dwroutingpid, ::core::mem::transmute(lpinentry), dwinentrysize, ::core::mem::transmute(lplpoutentry), ::core::mem::transmute(lpoutentrysize))
}
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`*"]
#[inline]
pub unsafe fn MprAdminMIBEntrySet(hmibserver: isize, dwprotocolid: u32, dwroutingpid: u32, lpentry: *const ::core::ffi::c_void, dwentrysize: u32) -> u32 {
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn MprAdminMIBEntrySet(hmibserver: isize, dwprotocolid: u32, dwroutingpid: u32, lpentry: *const ::core::ffi::c_void, dwentrysize: u32) -> u32;
    }
    MprAdminMIBEntrySet(hmibserver, dwprotocolid, dwroutingpid, ::core::mem::transmute(lpentry), dwentrysize)
}
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`*"]
#[inline]
pub unsafe fn MprAdminMIBServerConnect<'a, P0>(lpwsservername: P0, phmibserver: *mut isize) -> u32
where
    P0: ::std::convert::Into<::windows::core::PCWSTR>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn MprAdminMIBServerConnect(lpwsservername: ::windows::core::PCWSTR, phmibserver: *mut isize) -> u32;
    }
    MprAdminMIBServerConnect(lpwsservername.into(), ::core::mem::transmute(phmibserver))
}
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`*"]
#[inline]
pub unsafe fn MprAdminMIBServerDisconnect(hmibserver: isize) {
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn MprAdminMIBServerDisconnect(hmibserver: isize);
    }
    MprAdminMIBServerDisconnect(hmibserver)
}
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn MprAdminPortClearStats<'a, P0>(hrasserver: isize, hport: P0) -> u32
where
    P0: ::std::convert::Into<super::super::Foundation::HANDLE>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn MprAdminPortClearStats(hrasserver: isize, hport: super::super::Foundation::HANDLE) -> u32;
    }
    MprAdminPortClearStats(hrasserver, hport.into())
}
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn MprAdminPortDisconnect<'a, P0>(hrasserver: isize, hport: P0) -> u32
where
    P0: ::std::convert::Into<super::super::Foundation::HANDLE>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn MprAdminPortDisconnect(hrasserver: isize, hport: super::super::Foundation::HANDLE) -> u32;
    }
    MprAdminPortDisconnect(hrasserver, hport.into())
}
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn MprAdminPortEnum<'a, P0>(hrasserver: isize, dwlevel: u32, hrasconnection: P0, lplpbbuffer: *mut *mut u8, dwprefmaxlen: u32, lpdwentriesread: *mut u32, lpdwtotalentries: *mut u32, lpdwresumehandle: *const u32) -> u32
where
    P0: ::std::convert::Into<super::super::Foundation::HANDLE>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn MprAdminPortEnum(hrasserver: isize, dwlevel: u32, hrasconnection: super::super::Foundation::HANDLE, lplpbbuffer: *mut *mut u8, dwprefmaxlen: u32, lpdwentriesread: *mut u32, lpdwtotalentries: *mut u32, lpdwresumehandle: *const u32) -> u32;
    }
    MprAdminPortEnum(hrasserver, dwlevel, hrasconnection.into(), ::core::mem::transmute(lplpbbuffer), dwprefmaxlen, ::core::mem::transmute(lpdwentriesread), ::core::mem::transmute(lpdwtotalentries), ::core::mem::transmute(lpdwresumehandle))
}
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn MprAdminPortGetInfo<'a, P0>(hrasserver: isize, dwlevel: u32, hport: P0, lplpbbuffer: *mut *mut u8) -> u32
where
    P0: ::std::convert::Into<super::super::Foundation::HANDLE>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn MprAdminPortGetInfo(hrasserver: isize, dwlevel: u32, hport: super::super::Foundation::HANDLE, lplpbbuffer: *mut *mut u8) -> u32;
    }
    MprAdminPortGetInfo(hrasserver, dwlevel, hport.into(), ::core::mem::transmute(lplpbbuffer))
}
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn MprAdminPortReset<'a, P0>(hrasserver: isize, hport: P0) -> u32
where
    P0: ::std::convert::Into<super::super::Foundation::HANDLE>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn MprAdminPortReset(hrasserver: isize, hport: super::super::Foundation::HANDLE) -> u32;
    }
    MprAdminPortReset(hrasserver, hport.into())
}
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn MprAdminRegisterConnectionNotification<'a, P0>(hmprserver: isize, heventnotification: P0) -> u32
where
    P0: ::std::convert::Into<super::super::Foundation::HANDLE>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn MprAdminRegisterConnectionNotification(hmprserver: isize, heventnotification: super::super::Foundation::HANDLE) -> u32;
    }
    MprAdminRegisterConnectionNotification(hmprserver, heventnotification.into())
}
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn MprAdminSendUserMessage<'a, P0, P1>(hmprserver: isize, hconnection: P0, lpwszmessage: P1) -> u32
where
    P0: ::std::convert::Into<super::super::Foundation::HANDLE>,
    P1: ::std::convert::Into<::windows::core::PCWSTR>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn MprAdminSendUserMessage(hmprserver: isize, hconnection: super::super::Foundation::HANDLE, lpwszmessage: ::windows::core::PCWSTR) -> u32;
    }
    MprAdminSendUserMessage(hmprserver, hconnection.into(), lpwszmessage.into())
}
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`*"]
#[inline]
pub unsafe fn MprAdminServerConnect<'a, P0>(lpwsservername: P0, phmprserver: *mut isize) -> u32
where
    P0: ::std::convert::Into<::windows::core::PCWSTR>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn MprAdminServerConnect(lpwsservername: ::windows::core::PCWSTR, phmprserver: *mut isize) -> u32;
    }
    MprAdminServerConnect(lpwsservername.into(), ::core::mem::transmute(phmprserver))
}
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`*"]
#[inline]
pub unsafe fn MprAdminServerDisconnect(hmprserver: isize) {
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn MprAdminServerDisconnect(hmprserver: isize);
    }
    MprAdminServerDisconnect(hmprserver)
}
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`*"]
#[inline]
pub unsafe fn MprAdminServerGetCredentials(hmprserver: isize, dwlevel: u32, lplpbbuffer: *const *const u8) -> u32 {
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn MprAdminServerGetCredentials(hmprserver: isize, dwlevel: u32, lplpbbuffer: *const *const u8) -> u32;
    }
    MprAdminServerGetCredentials(hmprserver, dwlevel, ::core::mem::transmute(lplpbbuffer))
}
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`*"]
#[inline]
pub unsafe fn MprAdminServerGetInfo(hmprserver: isize, dwlevel: u32, lplpbbuffer: *mut *mut u8) -> u32 {
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn MprAdminServerGetInfo(hmprserver: isize, dwlevel: u32, lplpbbuffer: *mut *mut u8) -> u32;
    }
    MprAdminServerGetInfo(hmprserver, dwlevel, ::core::mem::transmute(lplpbbuffer))
}
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`, `\"Win32_Foundation\"`, `\"Win32_Security_Cryptography\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography"))]
#[inline]
pub unsafe fn MprAdminServerGetInfoEx(hmprserver: isize, pserverinfo: *mut MPR_SERVER_EX1) -> u32 {
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn MprAdminServerGetInfoEx(hmprserver: isize, pserverinfo: *mut MPR_SERVER_EX1) -> u32;
    }
    MprAdminServerGetInfoEx(hmprserver, ::core::mem::transmute(pserverinfo))
}
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`*"]
#[inline]
pub unsafe fn MprAdminServerSetCredentials(hmprserver: isize, dwlevel: u32, lpbbuffer: *const u8) -> u32 {
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn MprAdminServerSetCredentials(hmprserver: isize, dwlevel: u32, lpbbuffer: *const u8) -> u32;
    }
    MprAdminServerSetCredentials(hmprserver, dwlevel, ::core::mem::transmute(lpbbuffer))
}
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`*"]
#[inline]
pub unsafe fn MprAdminServerSetInfo(hmprserver: isize, dwlevel: u32, lpbbuffer: *const u8) -> u32 {
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn MprAdminServerSetInfo(hmprserver: isize, dwlevel: u32, lpbbuffer: *const u8) -> u32;
    }
    MprAdminServerSetInfo(hmprserver, dwlevel, ::core::mem::transmute(lpbbuffer))
}
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`, `\"Win32_Foundation\"`, `\"Win32_Security_Cryptography\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography"))]
#[inline]
pub unsafe fn MprAdminServerSetInfoEx(hmprserver: isize, pserverinfo: *const MPR_SERVER_SET_CONFIG_EX1) -> u32 {
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn MprAdminServerSetInfoEx(hmprserver: isize, pserverinfo: *const MPR_SERVER_SET_CONFIG_EX1) -> u32;
    }
    MprAdminServerSetInfoEx(hmprserver, ::core::mem::transmute(pserverinfo))
}
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`*"]
#[inline]
pub unsafe fn MprAdminTransportCreate<'a, P0, P1>(hmprserver: isize, dwtransportid: u32, lpwstransportname: P0, pglobalinfo: *const u8, dwglobalinfosize: u32, pclientinterfaceinfo: *const u8, dwclientinterfaceinfosize: u32, lpwsdllpath: P1) -> u32
where
    P0: ::std::convert::Into<::windows::core::PCWSTR>,
    P1: ::std::convert::Into<::windows::core::PCWSTR>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn MprAdminTransportCreate(hmprserver: isize, dwtransportid: u32, lpwstransportname: ::windows::core::PCWSTR, pglobalinfo: *const u8, dwglobalinfosize: u32, pclientinterfaceinfo: *const u8, dwclientinterfaceinfosize: u32, lpwsdllpath: ::windows::core::PCWSTR) -> u32;
    }
    MprAdminTransportCreate(hmprserver, dwtransportid, lpwstransportname.into(), ::core::mem::transmute(pglobalinfo), dwglobalinfosize, ::core::mem::transmute(pclientinterfaceinfo), dwclientinterfaceinfosize, lpwsdllpath.into())
}
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`*"]
#[inline]
pub unsafe fn MprAdminTransportGetInfo(hmprserver: isize, dwtransportid: u32, ppglobalinfo: *mut *mut u8, lpdwglobalinfosize: *mut u32, ppclientinterfaceinfo: *mut *mut u8, lpdwclientinterfaceinfosize: *mut u32) -> u32 {
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn MprAdminTransportGetInfo(hmprserver: isize, dwtransportid: u32, ppglobalinfo: *mut *mut u8, lpdwglobalinfosize: *mut u32, ppclientinterfaceinfo: *mut *mut u8, lpdwclientinterfaceinfosize: *mut u32) -> u32;
    }
    MprAdminTransportGetInfo(hmprserver, dwtransportid, ::core::mem::transmute(ppglobalinfo), ::core::mem::transmute(lpdwglobalinfosize), ::core::mem::transmute(ppclientinterfaceinfo), ::core::mem::transmute(lpdwclientinterfaceinfosize))
}
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`*"]
#[inline]
pub unsafe fn MprAdminTransportSetInfo(hmprserver: isize, dwtransportid: u32, pglobalinfo: *const u8, dwglobalinfosize: u32, pclientinterfaceinfo: *const u8, dwclientinterfaceinfosize: u32) -> u32 {
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn MprAdminTransportSetInfo(hmprserver: isize, dwtransportid: u32, pglobalinfo: *const u8, dwglobalinfosize: u32, pclientinterfaceinfo: *const u8, dwclientinterfaceinfosize: u32) -> u32;
    }
    MprAdminTransportSetInfo(hmprserver, dwtransportid, ::core::mem::transmute(pglobalinfo), dwglobalinfosize, ::core::mem::transmute(pclientinterfaceinfo), dwclientinterfaceinfosize)
}
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn MprAdminUpdateConnection<'a, P0>(hrasserver: isize, hrasconnection: P0, prasupdateconnection: *const RAS_UPDATE_CONNECTION) -> u32
where
    P0: ::std::convert::Into<super::super::Foundation::HANDLE>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn MprAdminUpdateConnection(hrasserver: isize, hrasconnection: super::super::Foundation::HANDLE, prasupdateconnection: *const RAS_UPDATE_CONNECTION) -> u32;
    }
    MprAdminUpdateConnection(hrasserver, hrasconnection.into(), ::core::mem::transmute(prasupdateconnection))
}
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`*"]
#[inline]
pub unsafe fn MprAdminUserGetInfo<'a, P0, P1>(lpszserver: P0, lpszuser: P1, dwlevel: u32, lpbbuffer: *mut u8) -> u32
where
    P0: ::std::convert::Into<::windows::core::PCWSTR>,
    P1: ::std::convert::Into<::windows::core::PCWSTR>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn MprAdminUserGetInfo(lpszserver: ::windows::core::PCWSTR, lpszuser: ::windows::core::PCWSTR, dwlevel: u32, lpbbuffer: *mut u8) -> u32;
    }
    MprAdminUserGetInfo(lpszserver.into(), lpszuser.into(), dwlevel, ::core::mem::transmute(lpbbuffer))
}
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`*"]
#[inline]
pub unsafe fn MprAdminUserSetInfo<'a, P0, P1>(lpszserver: P0, lpszuser: P1, dwlevel: u32, lpbbuffer: *const u8) -> u32
where
    P0: ::std::convert::Into<::windows::core::PCWSTR>,
    P1: ::std::convert::Into<::windows::core::PCWSTR>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn MprAdminUserSetInfo(lpszserver: ::windows::core::PCWSTR, lpszuser: ::windows::core::PCWSTR, dwlevel: u32, lpbbuffer: *const u8) -> u32;
    }
    MprAdminUserSetInfo(lpszserver.into(), lpszuser.into(), dwlevel, ::core::mem::transmute(lpbbuffer))
}
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`*"]
#[inline]
pub unsafe fn MprConfigBufferFree(pbuffer: *const ::core::ffi::c_void) -> u32 {
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn MprConfigBufferFree(pbuffer: *const ::core::ffi::c_void) -> u32;
    }
    MprConfigBufferFree(::core::mem::transmute(pbuffer))
}
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn MprConfigFilterGetInfo<'a, P0>(hmprconfig: P0, dwlevel: u32, dwtransportid: u32, lpbuffer: *mut u8) -> u32
where
    P0: ::std::convert::Into<super::super::Foundation::HANDLE>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn MprConfigFilterGetInfo(hmprconfig: super::super::Foundation::HANDLE, dwlevel: u32, dwtransportid: u32, lpbuffer: *mut u8) -> u32;
    }
    MprConfigFilterGetInfo(hmprconfig.into(), dwlevel, dwtransportid, ::core::mem::transmute(lpbuffer))
}
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn MprConfigFilterSetInfo<'a, P0>(hmprconfig: P0, dwlevel: u32, dwtransportid: u32, lpbuffer: *const u8) -> u32
where
    P0: ::std::convert::Into<super::super::Foundation::HANDLE>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn MprConfigFilterSetInfo(hmprconfig: super::super::Foundation::HANDLE, dwlevel: u32, dwtransportid: u32, lpbuffer: *const u8) -> u32;
    }
    MprConfigFilterSetInfo(hmprconfig.into(), dwlevel, dwtransportid, ::core::mem::transmute(lpbuffer))
}
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn MprConfigGetFriendlyName<'a, P0, P1>(hmprconfig: P0, pszguidname: P1, pszbuffer: ::windows::core::PWSTR, dwbuffersize: u32) -> u32
where
    P0: ::std::convert::Into<super::super::Foundation::HANDLE>,
    P1: ::std::convert::Into<::windows::core::PCWSTR>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn MprConfigGetFriendlyName(hmprconfig: super::super::Foundation::HANDLE, pszguidname: ::windows::core::PCWSTR, pszbuffer: ::windows::core::PWSTR, dwbuffersize: u32) -> u32;
    }
    MprConfigGetFriendlyName(hmprconfig.into(), pszguidname.into(), ::core::mem::transmute(pszbuffer), dwbuffersize)
}
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn MprConfigGetGuidName<'a, P0, P1>(hmprconfig: P0, pszfriendlyname: P1, pszbuffer: ::windows::core::PWSTR, dwbuffersize: u32) -> u32
where
    P0: ::std::convert::Into<super::super::Foundation::HANDLE>,
    P1: ::std::convert::Into<::windows::core::PCWSTR>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn MprConfigGetGuidName(hmprconfig: super::super::Foundation::HANDLE, pszfriendlyname: ::windows::core::PCWSTR, pszbuffer: ::windows::core::PWSTR, dwbuffersize: u32) -> u32;
    }
    MprConfigGetGuidName(hmprconfig.into(), pszfriendlyname.into(), ::core::mem::transmute(pszbuffer), dwbuffersize)
}
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn MprConfigInterfaceCreate<'a, P0>(hmprconfig: P0, dwlevel: u32, lpbbuffer: *const u8, phrouterinterface: *mut super::super::Foundation::HANDLE) -> u32
where
    P0: ::std::convert::Into<super::super::Foundation::HANDLE>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn MprConfigInterfaceCreate(hmprconfig: super::super::Foundation::HANDLE, dwlevel: u32, lpbbuffer: *const u8, phrouterinterface: *mut super::super::Foundation::HANDLE) -> u32;
    }
    MprConfigInterfaceCreate(hmprconfig.into(), dwlevel, ::core::mem::transmute(lpbbuffer), ::core::mem::transmute(phrouterinterface))
}
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn MprConfigInterfaceDelete<'a, P0, P1>(hmprconfig: P0, hrouterinterface: P1) -> u32
where
    P0: ::std::convert::Into<super::super::Foundation::HANDLE>,
    P1: ::std::convert::Into<super::super::Foundation::HANDLE>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn MprConfigInterfaceDelete(hmprconfig: super::super::Foundation::HANDLE, hrouterinterface: super::super::Foundation::HANDLE) -> u32;
    }
    MprConfigInterfaceDelete(hmprconfig.into(), hrouterinterface.into())
}
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn MprConfigInterfaceEnum<'a, P0>(hmprconfig: P0, dwlevel: u32, lplpbuffer: *mut *mut u8, dwprefmaxlen: u32, lpdwentriesread: *mut u32, lpdwtotalentries: *mut u32, lpdwresumehandle: *mut u32) -> u32
where
    P0: ::std::convert::Into<super::super::Foundation::HANDLE>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn MprConfigInterfaceEnum(hmprconfig: super::super::Foundation::HANDLE, dwlevel: u32, lplpbuffer: *mut *mut u8, dwprefmaxlen: u32, lpdwentriesread: *mut u32, lpdwtotalentries: *mut u32, lpdwresumehandle: *mut u32) -> u32;
    }
    MprConfigInterfaceEnum(hmprconfig.into(), dwlevel, ::core::mem::transmute(lplpbuffer), dwprefmaxlen, ::core::mem::transmute(lpdwentriesread), ::core::mem::transmute(lpdwtotalentries), ::core::mem::transmute(lpdwresumehandle))
}
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`, `\"Win32_Foundation\"`, `\"Win32_Networking_WinSock\"`, `\"Win32_Security_Cryptography\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock", feature = "Win32_Security_Cryptography"))]
#[inline]
pub unsafe fn MprConfigInterfaceGetCustomInfoEx<'a, P0, P1>(hmprconfig: P0, hrouterinterface: P1, pcustominfo: *mut MPR_IF_CUSTOMINFOEX2) -> u32
where
    P0: ::std::convert::Into<super::super::Foundation::HANDLE>,
    P1: ::std::convert::Into<super::super::Foundation::HANDLE>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn MprConfigInterfaceGetCustomInfoEx(hmprconfig: super::super::Foundation::HANDLE, hrouterinterface: super::super::Foundation::HANDLE, pcustominfo: *mut MPR_IF_CUSTOMINFOEX2) -> u32;
    }
    MprConfigInterfaceGetCustomInfoEx(hmprconfig.into(), hrouterinterface.into(), ::core::mem::transmute(pcustominfo))
}
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn MprConfigInterfaceGetHandle<'a, P0, P1>(hmprconfig: P0, lpwsinterfacename: P1, phrouterinterface: *mut super::super::Foundation::HANDLE) -> u32
where
    P0: ::std::convert::Into<super::super::Foundation::HANDLE>,
    P1: ::std::convert::Into<::windows::core::PCWSTR>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn MprConfigInterfaceGetHandle(hmprconfig: super::super::Foundation::HANDLE, lpwsinterfacename: ::windows::core::PCWSTR, phrouterinterface: *mut super::super::Foundation::HANDLE) -> u32;
    }
    MprConfigInterfaceGetHandle(hmprconfig.into(), lpwsinterfacename.into(), ::core::mem::transmute(phrouterinterface))
}
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn MprConfigInterfaceGetInfo<'a, P0, P1>(hmprconfig: P0, hrouterinterface: P1, dwlevel: u32, lplpbuffer: *mut *mut u8, lpdwbuffersize: *mut u32) -> u32
where
    P0: ::std::convert::Into<super::super::Foundation::HANDLE>,
    P1: ::std::convert::Into<super::super::Foundation::HANDLE>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn MprConfigInterfaceGetInfo(hmprconfig: super::super::Foundation::HANDLE, hrouterinterface: super::super::Foundation::HANDLE, dwlevel: u32, lplpbuffer: *mut *mut u8, lpdwbuffersize: *mut u32) -> u32;
    }
    MprConfigInterfaceGetInfo(hmprconfig.into(), hrouterinterface.into(), dwlevel, ::core::mem::transmute(lplpbuffer), ::core::mem::transmute(lpdwbuffersize))
}
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`, `\"Win32_Foundation\"`, `\"Win32_Networking_WinSock\"`, `\"Win32_Security_Cryptography\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock", feature = "Win32_Security_Cryptography"))]
#[inline]
pub unsafe fn MprConfigInterfaceSetCustomInfoEx<'a, P0, P1>(hmprconfig: P0, hrouterinterface: P1, pcustominfo: *const MPR_IF_CUSTOMINFOEX2) -> u32
where
    P0: ::std::convert::Into<super::super::Foundation::HANDLE>,
    P1: ::std::convert::Into<super::super::Foundation::HANDLE>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn MprConfigInterfaceSetCustomInfoEx(hmprconfig: super::super::Foundation::HANDLE, hrouterinterface: super::super::Foundation::HANDLE, pcustominfo: *const MPR_IF_CUSTOMINFOEX2) -> u32;
    }
    MprConfigInterfaceSetCustomInfoEx(hmprconfig.into(), hrouterinterface.into(), ::core::mem::transmute(pcustominfo))
}
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn MprConfigInterfaceSetInfo<'a, P0, P1>(hmprconfig: P0, hrouterinterface: P1, dwlevel: u32, lpbbuffer: *const u8) -> u32
where
    P0: ::std::convert::Into<super::super::Foundation::HANDLE>,
    P1: ::std::convert::Into<super::super::Foundation::HANDLE>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn MprConfigInterfaceSetInfo(hmprconfig: super::super::Foundation::HANDLE, hrouterinterface: super::super::Foundation::HANDLE, dwlevel: u32, lpbbuffer: *const u8) -> u32;
    }
    MprConfigInterfaceSetInfo(hmprconfig.into(), hrouterinterface.into(), dwlevel, ::core::mem::transmute(lpbbuffer))
}
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn MprConfigInterfaceTransportAdd<'a, P0, P1, P2>(hmprconfig: P0, hrouterinterface: P1, dwtransportid: u32, lpwstransportname: P2, pinterfaceinfo: *const u8, dwinterfaceinfosize: u32, phrouteriftransport: *mut super::super::Foundation::HANDLE) -> u32
where
    P0: ::std::convert::Into<super::super::Foundation::HANDLE>,
    P1: ::std::convert::Into<super::super::Foundation::HANDLE>,
    P2: ::std::convert::Into<::windows::core::PCWSTR>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn MprConfigInterfaceTransportAdd(hmprconfig: super::super::Foundation::HANDLE, hrouterinterface: super::super::Foundation::HANDLE, dwtransportid: u32, lpwstransportname: ::windows::core::PCWSTR, pinterfaceinfo: *const u8, dwinterfaceinfosize: u32, phrouteriftransport: *mut super::super::Foundation::HANDLE) -> u32;
    }
    MprConfigInterfaceTransportAdd(hmprconfig.into(), hrouterinterface.into(), dwtransportid, lpwstransportname.into(), ::core::mem::transmute(pinterfaceinfo), dwinterfaceinfosize, ::core::mem::transmute(phrouteriftransport))
}
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn MprConfigInterfaceTransportEnum<'a, P0, P1>(hmprconfig: P0, hrouterinterface: P1, dwlevel: u32, lplpbuffer: *mut *mut u8, dwprefmaxlen: u32, lpdwentriesread: *mut u32, lpdwtotalentries: *mut u32, lpdwresumehandle: *mut u32) -> u32
where
    P0: ::std::convert::Into<super::super::Foundation::HANDLE>,
    P1: ::std::convert::Into<super::super::Foundation::HANDLE>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn MprConfigInterfaceTransportEnum(hmprconfig: super::super::Foundation::HANDLE, hrouterinterface: super::super::Foundation::HANDLE, dwlevel: u32, lplpbuffer: *mut *mut u8, dwprefmaxlen: u32, lpdwentriesread: *mut u32, lpdwtotalentries: *mut u32, lpdwresumehandle: *mut u32) -> u32;
    }
    MprConfigInterfaceTransportEnum(hmprconfig.into(), hrouterinterface.into(), dwlevel, ::core::mem::transmute(lplpbuffer), dwprefmaxlen, ::core::mem::transmute(lpdwentriesread), ::core::mem::transmute(lpdwtotalentries), ::core::mem::transmute(lpdwresumehandle))
}
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn MprConfigInterfaceTransportGetHandle<'a, P0, P1>(hmprconfig: P0, hrouterinterface: P1, dwtransportid: u32, phrouteriftransport: *mut super::super::Foundation::HANDLE) -> u32
where
    P0: ::std::convert::Into<super::super::Foundation::HANDLE>,
    P1: ::std::convert::Into<super::super::Foundation::HANDLE>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn MprConfigInterfaceTransportGetHandle(hmprconfig: super::super::Foundation::HANDLE, hrouterinterface: super::super::Foundation::HANDLE, dwtransportid: u32, phrouteriftransport: *mut super::super::Foundation::HANDLE) -> u32;
    }
    MprConfigInterfaceTransportGetHandle(hmprconfig.into(), hrouterinterface.into(), dwtransportid, ::core::mem::transmute(phrouteriftransport))
}
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn MprConfigInterfaceTransportGetInfo<'a, P0, P1, P2>(hmprconfig: P0, hrouterinterface: P1, hrouteriftransport: P2, ppinterfaceinfo: *mut *mut u8, lpdwinterfaceinfosize: *mut u32) -> u32
where
    P0: ::std::convert::Into<super::super::Foundation::HANDLE>,
    P1: ::std::convert::Into<super::super::Foundation::HANDLE>,
    P2: ::std::convert::Into<super::super::Foundation::HANDLE>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn MprConfigInterfaceTransportGetInfo(hmprconfig: super::super::Foundation::HANDLE, hrouterinterface: super::super::Foundation::HANDLE, hrouteriftransport: super::super::Foundation::HANDLE, ppinterfaceinfo: *mut *mut u8, lpdwinterfaceinfosize: *mut u32) -> u32;
    }
    MprConfigInterfaceTransportGetInfo(hmprconfig.into(), hrouterinterface.into(), hrouteriftransport.into(), ::core::mem::transmute(ppinterfaceinfo), ::core::mem::transmute(lpdwinterfaceinfosize))
}
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn MprConfigInterfaceTransportRemove<'a, P0, P1, P2>(hmprconfig: P0, hrouterinterface: P1, hrouteriftransport: P2) -> u32
where
    P0: ::std::convert::Into<super::super::Foundation::HANDLE>,
    P1: ::std::convert::Into<super::super::Foundation::HANDLE>,
    P2: ::std::convert::Into<super::super::Foundation::HANDLE>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn MprConfigInterfaceTransportRemove(hmprconfig: super::super::Foundation::HANDLE, hrouterinterface: super::super::Foundation::HANDLE, hrouteriftransport: super::super::Foundation::HANDLE) -> u32;
    }
    MprConfigInterfaceTransportRemove(hmprconfig.into(), hrouterinterface.into(), hrouteriftransport.into())
}
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn MprConfigInterfaceTransportSetInfo<'a, P0, P1, P2>(hmprconfig: P0, hrouterinterface: P1, hrouteriftransport: P2, pinterfaceinfo: *const u8, dwinterfaceinfosize: u32) -> u32
where
    P0: ::std::convert::Into<super::super::Foundation::HANDLE>,
    P1: ::std::convert::Into<super::super::Foundation::HANDLE>,
    P2: ::std::convert::Into<super::super::Foundation::HANDLE>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn MprConfigInterfaceTransportSetInfo(hmprconfig: super::super::Foundation::HANDLE, hrouterinterface: super::super::Foundation::HANDLE, hrouteriftransport: super::super::Foundation::HANDLE, pinterfaceinfo: *const u8, dwinterfaceinfosize: u32) -> u32;
    }
    MprConfigInterfaceTransportSetInfo(hmprconfig.into(), hrouterinterface.into(), hrouteriftransport.into(), ::core::mem::transmute(pinterfaceinfo), dwinterfaceinfosize)
}
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn MprConfigServerBackup<'a, P0, P1>(hmprconfig: P0, lpwspath: P1) -> u32
where
    P0: ::std::convert::Into<super::super::Foundation::HANDLE>,
    P1: ::std::convert::Into<::windows::core::PCWSTR>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn MprConfigServerBackup(hmprconfig: super::super::Foundation::HANDLE, lpwspath: ::windows::core::PCWSTR) -> u32;
    }
    MprConfigServerBackup(hmprconfig.into(), lpwspath.into())
}
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn MprConfigServerConnect<'a, P0>(lpwsservername: P0, phmprconfig: *mut super::super::Foundation::HANDLE) -> u32
where
    P0: ::std::convert::Into<::windows::core::PCWSTR>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn MprConfigServerConnect(lpwsservername: ::windows::core::PCWSTR, phmprconfig: *mut super::super::Foundation::HANDLE) -> u32;
    }
    MprConfigServerConnect(lpwsservername.into(), ::core::mem::transmute(phmprconfig))
}
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn MprConfigServerDisconnect<'a, P0>(hmprconfig: P0)
where
    P0: ::std::convert::Into<super::super::Foundation::HANDLE>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn MprConfigServerDisconnect(hmprconfig: super::super::Foundation::HANDLE);
    }
    MprConfigServerDisconnect(hmprconfig.into())
}
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn MprConfigServerGetInfo<'a, P0>(hmprconfig: P0, dwlevel: u32, lplpbbuffer: *mut *mut u8) -> u32
where
    P0: ::std::convert::Into<super::super::Foundation::HANDLE>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn MprConfigServerGetInfo(hmprconfig: super::super::Foundation::HANDLE, dwlevel: u32, lplpbbuffer: *mut *mut u8) -> u32;
    }
    MprConfigServerGetInfo(hmprconfig.into(), dwlevel, ::core::mem::transmute(lplpbbuffer))
}
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`, `\"Win32_Foundation\"`, `\"Win32_Security_Cryptography\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography"))]
#[inline]
pub unsafe fn MprConfigServerGetInfoEx<'a, P0>(hmprconfig: P0, pserverinfo: *mut MPR_SERVER_EX1) -> u32
where
    P0: ::std::convert::Into<super::super::Foundation::HANDLE>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn MprConfigServerGetInfoEx(hmprconfig: super::super::Foundation::HANDLE, pserverinfo: *mut MPR_SERVER_EX1) -> u32;
    }
    MprConfigServerGetInfoEx(hmprconfig.into(), ::core::mem::transmute(pserverinfo))
}
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`*"]
#[inline]
pub unsafe fn MprConfigServerInstall(dwlevel: u32, pbuffer: *const ::core::ffi::c_void) -> u32 {
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn MprConfigServerInstall(dwlevel: u32, pbuffer: *const ::core::ffi::c_void) -> u32;
    }
    MprConfigServerInstall(dwlevel, ::core::mem::transmute(pbuffer))
}
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn MprConfigServerRefresh<'a, P0>(hmprconfig: P0) -> u32
where
    P0: ::std::convert::Into<super::super::Foundation::HANDLE>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn MprConfigServerRefresh(hmprconfig: super::super::Foundation::HANDLE) -> u32;
    }
    MprConfigServerRefresh(hmprconfig.into())
}
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn MprConfigServerRestore<'a, P0, P1>(hmprconfig: P0, lpwspath: P1) -> u32
where
    P0: ::std::convert::Into<super::super::Foundation::HANDLE>,
    P1: ::std::convert::Into<::windows::core::PCWSTR>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn MprConfigServerRestore(hmprconfig: super::super::Foundation::HANDLE, lpwspath: ::windows::core::PCWSTR) -> u32;
    }
    MprConfigServerRestore(hmprconfig.into(), lpwspath.into())
}
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`*"]
#[inline]
pub unsafe fn MprConfigServerSetInfo(hmprserver: isize, dwlevel: u32, lpbbuffer: *const u8) -> u32 {
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn MprConfigServerSetInfo(hmprserver: isize, dwlevel: u32, lpbbuffer: *const u8) -> u32;
    }
    MprConfigServerSetInfo(hmprserver, dwlevel, ::core::mem::transmute(lpbbuffer))
}
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`, `\"Win32_Foundation\"`, `\"Win32_Security_Cryptography\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography"))]
#[inline]
pub unsafe fn MprConfigServerSetInfoEx<'a, P0>(hmprconfig: P0, psetserverconfig: *const MPR_SERVER_SET_CONFIG_EX1) -> u32
where
    P0: ::std::convert::Into<super::super::Foundation::HANDLE>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn MprConfigServerSetInfoEx(hmprconfig: super::super::Foundation::HANDLE, psetserverconfig: *const MPR_SERVER_SET_CONFIG_EX1) -> u32;
    }
    MprConfigServerSetInfoEx(hmprconfig.into(), ::core::mem::transmute(psetserverconfig))
}
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn MprConfigTransportCreate<'a, P0, P1, P2>(hmprconfig: P0, dwtransportid: u32, lpwstransportname: P1, pglobalinfo: *const u8, dwglobalinfosize: u32, pclientinterfaceinfo: *const u8, dwclientinterfaceinfosize: u32, lpwsdllpath: P2, phroutertransport: *mut super::super::Foundation::HANDLE) -> u32
where
    P0: ::std::convert::Into<super::super::Foundation::HANDLE>,
    P1: ::std::convert::Into<::windows::core::PCWSTR>,
    P2: ::std::convert::Into<::windows::core::PCWSTR>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn MprConfigTransportCreate(hmprconfig: super::super::Foundation::HANDLE, dwtransportid: u32, lpwstransportname: ::windows::core::PCWSTR, pglobalinfo: *const u8, dwglobalinfosize: u32, pclientinterfaceinfo: *const u8, dwclientinterfaceinfosize: u32, lpwsdllpath: ::windows::core::PCWSTR, phroutertransport: *mut super::super::Foundation::HANDLE) -> u32;
    }
    MprConfigTransportCreate(hmprconfig.into(), dwtransportid, lpwstransportname.into(), ::core::mem::transmute(pglobalinfo), dwglobalinfosize, ::core::mem::transmute(pclientinterfaceinfo), dwclientinterfaceinfosize, lpwsdllpath.into(), ::core::mem::transmute(phroutertransport))
}
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn MprConfigTransportDelete<'a, P0, P1>(hmprconfig: P0, hroutertransport: P1) -> u32
where
    P0: ::std::convert::Into<super::super::Foundation::HANDLE>,
    P1: ::std::convert::Into<super::super::Foundation::HANDLE>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn MprConfigTransportDelete(hmprconfig: super::super::Foundation::HANDLE, hroutertransport: super::super::Foundation::HANDLE) -> u32;
    }
    MprConfigTransportDelete(hmprconfig.into(), hroutertransport.into())
}
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn MprConfigTransportEnum<'a, P0>(hmprconfig: P0, dwlevel: u32, lplpbuffer: *mut *mut u8, dwprefmaxlen: u32, lpdwentriesread: *mut u32, lpdwtotalentries: *mut u32, lpdwresumehandle: *mut u32) -> u32
where
    P0: ::std::convert::Into<super::super::Foundation::HANDLE>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn MprConfigTransportEnum(hmprconfig: super::super::Foundation::HANDLE, dwlevel: u32, lplpbuffer: *mut *mut u8, dwprefmaxlen: u32, lpdwentriesread: *mut u32, lpdwtotalentries: *mut u32, lpdwresumehandle: *mut u32) -> u32;
    }
    MprConfigTransportEnum(hmprconfig.into(), dwlevel, ::core::mem::transmute(lplpbuffer), dwprefmaxlen, ::core::mem::transmute(lpdwentriesread), ::core::mem::transmute(lpdwtotalentries), ::core::mem::transmute(lpdwresumehandle))
}
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn MprConfigTransportGetHandle<'a, P0>(hmprconfig: P0, dwtransportid: u32, phroutertransport: *mut super::super::Foundation::HANDLE) -> u32
where
    P0: ::std::convert::Into<super::super::Foundation::HANDLE>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn MprConfigTransportGetHandle(hmprconfig: super::super::Foundation::HANDLE, dwtransportid: u32, phroutertransport: *mut super::super::Foundation::HANDLE) -> u32;
    }
    MprConfigTransportGetHandle(hmprconfig.into(), dwtransportid, ::core::mem::transmute(phroutertransport))
}
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn MprConfigTransportGetInfo<'a, P0, P1>(hmprconfig: P0, hroutertransport: P1, ppglobalinfo: *mut *mut u8, lpdwglobalinfosize: *mut u32, ppclientinterfaceinfo: *mut *mut u8, lpdwclientinterfaceinfosize: *mut u32, lplpwsdllpath: *mut ::windows::core::PWSTR) -> u32
where
    P0: ::std::convert::Into<super::super::Foundation::HANDLE>,
    P1: ::std::convert::Into<super::super::Foundation::HANDLE>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn MprConfigTransportGetInfo(hmprconfig: super::super::Foundation::HANDLE, hroutertransport: super::super::Foundation::HANDLE, ppglobalinfo: *mut *mut u8, lpdwglobalinfosize: *mut u32, ppclientinterfaceinfo: *mut *mut u8, lpdwclientinterfaceinfosize: *mut u32, lplpwsdllpath: *mut ::windows::core::PWSTR) -> u32;
    }
    MprConfigTransportGetInfo(hmprconfig.into(), hroutertransport.into(), ::core::mem::transmute(ppglobalinfo), ::core::mem::transmute(lpdwglobalinfosize), ::core::mem::transmute(ppclientinterfaceinfo), ::core::mem::transmute(lpdwclientinterfaceinfosize), ::core::mem::transmute(lplpwsdllpath))
}
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn MprConfigTransportSetInfo<'a, P0, P1, P2>(hmprconfig: P0, hroutertransport: P1, pglobalinfo: *const u8, dwglobalinfosize: u32, pclientinterfaceinfo: *const u8, dwclientinterfaceinfosize: u32, lpwsdllpath: P2) -> u32
where
    P0: ::std::convert::Into<super::super::Foundation::HANDLE>,
    P1: ::std::convert::Into<super::super::Foundation::HANDLE>,
    P2: ::std::convert::Into<::windows::core::PCWSTR>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn MprConfigTransportSetInfo(hmprconfig: super::super::Foundation::HANDLE, hroutertransport: super::super::Foundation::HANDLE, pglobalinfo: *const u8, dwglobalinfosize: u32, pclientinterfaceinfo: *const u8, dwclientinterfaceinfosize: u32, lpwsdllpath: ::windows::core::PCWSTR) -> u32;
    }
    MprConfigTransportSetInfo(hmprconfig.into(), hroutertransport.into(), ::core::mem::transmute(pglobalinfo), dwglobalinfosize, ::core::mem::transmute(pclientinterfaceinfo), dwclientinterfaceinfosize, lpwsdllpath.into())
}
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`*"]
#[inline]
pub unsafe fn MprInfoBlockAdd(lpheader: *const ::core::ffi::c_void, dwinfotype: u32, dwitemsize: u32, dwitemcount: u32, lpitemdata: *const u8, lplpnewheader: *mut *mut ::core::ffi::c_void) -> u32 {
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn MprInfoBlockAdd(lpheader: *const ::core::ffi::c_void, dwinfotype: u32, dwitemsize: u32, dwitemcount: u32, lpitemdata: *const u8, lplpnewheader: *mut *mut ::core::ffi::c_void) -> u32;
    }
    MprInfoBlockAdd(::core::mem::transmute(lpheader), dwinfotype, dwitemsize, dwitemcount, ::core::mem::transmute(lpitemdata), ::core::mem::transmute(lplpnewheader))
}
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`*"]
#[inline]
pub unsafe fn MprInfoBlockFind(lpheader: *const ::core::ffi::c_void, dwinfotype: u32, lpdwitemsize: *mut u32, lpdwitemcount: *mut u32, lplpitemdata: *mut *mut u8) -> u32 {
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn MprInfoBlockFind(lpheader: *const ::core::ffi::c_void, dwinfotype: u32, lpdwitemsize: *mut u32, lpdwitemcount: *mut u32, lplpitemdata: *mut *mut u8) -> u32;
    }
    MprInfoBlockFind(::core::mem::transmute(lpheader), dwinfotype, ::core::mem::transmute(lpdwitemsize), ::core::mem::transmute(lpdwitemcount), ::core::mem::transmute(lplpitemdata))
}
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`*"]
#[inline]
pub unsafe fn MprInfoBlockQuerySize(lpheader: *const ::core::ffi::c_void) -> u32 {
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn MprInfoBlockQuerySize(lpheader: *const ::core::ffi::c_void) -> u32;
    }
    MprInfoBlockQuerySize(::core::mem::transmute(lpheader))
}
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`*"]
#[inline]
pub unsafe fn MprInfoBlockRemove(lpheader: *const ::core::ffi::c_void, dwinfotype: u32, lplpnewheader: *mut *mut ::core::ffi::c_void) -> u32 {
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn MprInfoBlockRemove(lpheader: *const ::core::ffi::c_void, dwinfotype: u32, lplpnewheader: *mut *mut ::core::ffi::c_void) -> u32;
    }
    MprInfoBlockRemove(::core::mem::transmute(lpheader), dwinfotype, ::core::mem::transmute(lplpnewheader))
}
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`*"]
#[inline]
pub unsafe fn MprInfoBlockSet(lpheader: *const ::core::ffi::c_void, dwinfotype: u32, dwitemsize: u32, dwitemcount: u32, lpitemdata: *const u8, lplpnewheader: *mut *mut ::core::ffi::c_void) -> u32 {
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn MprInfoBlockSet(lpheader: *const ::core::ffi::c_void, dwinfotype: u32, dwitemsize: u32, dwitemcount: u32, lpitemdata: *const u8, lplpnewheader: *mut *mut ::core::ffi::c_void) -> u32;
    }
    MprInfoBlockSet(::core::mem::transmute(lpheader), dwinfotype, dwitemsize, dwitemcount, ::core::mem::transmute(lpitemdata), ::core::mem::transmute(lplpnewheader))
}
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`*"]
#[inline]
pub unsafe fn MprInfoCreate(dwversion: u32, lplpnewheader: *mut *mut ::core::ffi::c_void) -> u32 {
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn MprInfoCreate(dwversion: u32, lplpnewheader: *mut *mut ::core::ffi::c_void) -> u32;
    }
    MprInfoCreate(dwversion, ::core::mem::transmute(lplpnewheader))
}
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`*"]
#[inline]
pub unsafe fn MprInfoDelete(lpheader: *const ::core::ffi::c_void) -> u32 {
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn MprInfoDelete(lpheader: *const ::core::ffi::c_void) -> u32;
    }
    MprInfoDelete(::core::mem::transmute(lpheader))
}
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`*"]
#[inline]
pub unsafe fn MprInfoDuplicate(lpheader: *const ::core::ffi::c_void, lplpnewheader: *mut *mut ::core::ffi::c_void) -> u32 {
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn MprInfoDuplicate(lpheader: *const ::core::ffi::c_void, lplpnewheader: *mut *mut ::core::ffi::c_void) -> u32;
    }
    MprInfoDuplicate(::core::mem::transmute(lpheader), ::core::mem::transmute(lplpnewheader))
}
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`*"]
#[inline]
pub unsafe fn MprInfoRemoveAll(lpheader: *const ::core::ffi::c_void, lplpnewheader: *mut *mut ::core::ffi::c_void) -> u32 {
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn MprInfoRemoveAll(lpheader: *const ::core::ffi::c_void, lplpnewheader: *mut *mut ::core::ffi::c_void) -> u32;
    }
    MprInfoRemoveAll(::core::mem::transmute(lpheader), ::core::mem::transmute(lplpnewheader))
}
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub type ORASADFUNC = ::core::option::Option<unsafe extern "system" fn(param0: super::super::Foundation::HWND, param1: ::windows::core::PCSTR, param2: u32, param3: *mut u32) -> super::super::Foundation::BOOL>;
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`*"]
pub const PENDING: u32 = 600u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`*"]
pub type PFNRASFREEBUFFER = ::core::option::Option<unsafe extern "system" fn(pbufer: *mut u8) -> u32>;
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`*"]
pub type PFNRASGETBUFFER = ::core::option::Option<unsafe extern "system" fn(ppbuffer: *mut *mut u8, pdwsize: *mut u32) -> u32>;
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub type PFNRASRECEIVEBUFFER = ::core::option::Option<unsafe extern "system" fn(hport: super::super::Foundation::HANDLE, pbuffer: *mut u8, pdwsize: *mut u32, dwtimeout: u32, hevent: super::super::Foundation::HANDLE) -> u32>;
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub type PFNRASRETRIEVEBUFFER = ::core::option::Option<unsafe extern "system" fn(hport: super::super::Foundation::HANDLE, pbuffer: *mut u8, pdwsize: *mut u32) -> u32>;
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub type PFNRASSENDBUFFER = ::core::option::Option<unsafe extern "system" fn(hport: super::super::Foundation::HANDLE, pbuffer: *mut u8, dwsize: u32) -> u32>;
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub type PFNRASSETCOMMSETTINGS = ::core::option::Option<unsafe extern "system" fn(hport: super::super::Foundation::HANDLE, prascommsettings: *mut RASCOMMSETTINGS, pvreserved: *mut ::core::ffi::c_void) -> u32>;
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`*"]
pub const PID_ATALK: u32 = 41u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`*"]
pub const PID_IP: u32 = 33u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`*"]
pub const PID_IPV6: u32 = 87u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`*"]
pub const PID_IPX: u32 = 43u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`*"]
pub const PID_NBF: u32 = 63u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub type PMGM_CREATION_ALERT_CALLBACK = ::core::option::Option<unsafe extern "system" fn(dwsourceaddr: u32, dwsourcemask: u32, dwgroupaddr: u32, dwgroupmask: u32, dwinifindex: u32, dwinifnexthopaddr: u32, dwifcount: u32, pmieoutiflist: *mut MGM_IF_ENTRY) -> u32>;
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`*"]
pub type PMGM_DISABLE_IGMP_CALLBACK = ::core::option::Option<unsafe extern "system" fn(dwifindex: u32, dwifnexthopaddr: u32) -> u32>;
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`*"]
pub type PMGM_ENABLE_IGMP_CALLBACK = ::core::option::Option<unsafe extern "system" fn(dwifindex: u32, dwifnexthopaddr: u32) -> u32>;
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub type PMGM_JOIN_ALERT_CALLBACK = ::core::option::Option<unsafe extern "system" fn(dwsourceaddr: u32, dwsourcemask: u32, dwgroupaddr: u32, dwgroupmask: u32, bmemberupdate: super::super::Foundation::BOOL) -> u32>;
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`*"]
pub type PMGM_LOCAL_JOIN_CALLBACK = ::core::option::Option<unsafe extern "system" fn(dwsourceaddr: u32, dwsourcemask: u32, dwgroupaddr: u32, dwgroupmask: u32, dwifindex: u32, dwifnexthopaddr: u32) -> u32>;
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`*"]
pub type PMGM_LOCAL_LEAVE_CALLBACK = ::core::option::Option<unsafe extern "system" fn(dwsourceaddr: u32, dwsourcemask: u32, dwgroupaddr: u32, dwgroupmask: u32, dwifindex: u32, dwifnexthopaddr: u32) -> u32>;
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub type PMGM_PRUNE_ALERT_CALLBACK = ::core::option::Option<unsafe extern "system" fn(dwsourceaddr: u32, dwsourcemask: u32, dwgroupaddr: u32, dwgroupmask: u32, dwifindex: u32, dwifnexthopaddr: u32, bmemberdelete: super::super::Foundation::BOOL, pdwtimeout: *mut u32) -> u32>;
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`*"]
pub type PMGM_RPF_CALLBACK = ::core::option::Option<unsafe extern "system" fn(dwsourceaddr: u32, dwsourcemask: u32, dwgroupaddr: u32, dwgroupmask: u32, pdwinifindex: *mut u32, pdwinifnexthopaddr: *mut u32, pdwupstreamnbr: *mut u32, dwhdrsize: u32, pbpackethdr: *mut u8, pbroute: *mut u8) -> u32>;
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`*"]
pub type PMGM_WRONG_IF_CALLBACK = ::core::option::Option<unsafe extern "system" fn(dwsourceaddr: u32, dwgroupaddr: u32, dwifindex: u32, dwifnexthopaddr: u32, dwhdrsize: u32, pbpackethdr: *mut u8) -> u32>;
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub type PMPRADMINACCEPTNEWCONNECTION = ::core::option::Option<unsafe extern "system" fn(param0: *mut RAS_CONNECTION_0, param1: *mut RAS_CONNECTION_1) -> super::super::Foundation::BOOL>;
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub type PMPRADMINACCEPTNEWCONNECTION2 = ::core::option::Option<unsafe extern "system" fn(param0: *mut RAS_CONNECTION_0, param1: *mut RAS_CONNECTION_1, param2: *mut RAS_CONNECTION_2) -> super::super::Foundation::BOOL>;
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub type PMPRADMINACCEPTNEWCONNECTION3 = ::core::option::Option<unsafe extern "system" fn(param0: *mut RAS_CONNECTION_0, param1: *mut RAS_CONNECTION_1, param2: *mut RAS_CONNECTION_2, param3: *mut RAS_CONNECTION_3) -> super::super::Foundation::BOOL>;
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub type PMPRADMINACCEPTNEWCONNECTIONEX = ::core::option::Option<unsafe extern "system" fn(param0: *mut RAS_CONNECTION_EX) -> super::super::Foundation::BOOL>;
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub type PMPRADMINACCEPTNEWLINK = ::core::option::Option<unsafe extern "system" fn(param0: *mut RAS_PORT_0, param1: *mut RAS_PORT_1) -> super::super::Foundation::BOOL>;
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub type PMPRADMINACCEPTREAUTHENTICATION = ::core::option::Option<unsafe extern "system" fn(param0: *mut RAS_CONNECTION_0, param1: *mut RAS_CONNECTION_1, param2: *mut RAS_CONNECTION_2, param3: *mut RAS_CONNECTION_3) -> super::super::Foundation::BOOL>;
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub type PMPRADMINACCEPTREAUTHENTICATIONEX = ::core::option::Option<unsafe extern "system" fn(param0: *mut RAS_CONNECTION_EX) -> super::super::Foundation::BOOL>;
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub type PMPRADMINACCEPTTUNNELENDPOINTCHANGEEX = ::core::option::Option<unsafe extern "system" fn(param0: *mut RAS_CONNECTION_EX) -> super::super::Foundation::BOOL>;
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub type PMPRADMINCONNECTIONHANGUPNOTIFICATION = ::core::option::Option<unsafe extern "system" fn(param0: *mut RAS_CONNECTION_0, param1: *mut RAS_CONNECTION_1)>;
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub type PMPRADMINCONNECTIONHANGUPNOTIFICATION2 = ::core::option::Option<unsafe extern "system" fn(param0: *mut RAS_CONNECTION_0, param1: *mut RAS_CONNECTION_1, param2: *mut RAS_CONNECTION_2)>;
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub type PMPRADMINCONNECTIONHANGUPNOTIFICATION3 = ::core::option::Option<unsafe extern "system" fn(param0: *mut RAS_CONNECTION_0, param1: *mut RAS_CONNECTION_1, param2: *mut RAS_CONNECTION_2, param3: RAS_CONNECTION_3)>;
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub type PMPRADMINCONNECTIONHANGUPNOTIFICATIONEX = ::core::option::Option<unsafe extern "system" fn(param0: *mut RAS_CONNECTION_EX)>;
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub type PMPRADMINGETIPADDRESSFORUSER = ::core::option::Option<unsafe extern "system" fn(param0: ::windows::core::PCWSTR, param1: ::windows::core::PCWSTR, param2: *mut u32, param3: *mut super::super::Foundation::BOOL) -> u32>;
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`, `\"Win32_Foundation\"`, `\"Win32_Networking_WinSock\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
pub type PMPRADMINGETIPV6ADDRESSFORUSER = ::core::option::Option<unsafe extern "system" fn(param0: ::windows::core::PCWSTR, param1: ::windows::core::PCWSTR, param2: *mut super::super::Networking::WinSock::IN6_ADDR, param3: *mut super::super::Foundation::BOOL) -> u32>;
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub type PMPRADMINLINKHANGUPNOTIFICATION = ::core::option::Option<unsafe extern "system" fn(param0: *mut RAS_PORT_0, param1: *mut RAS_PORT_1)>;
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub type PMPRADMINRASVALIDATEPREAUTHENTICATEDCONNECTIONEX = ::core::option::Option<unsafe extern "system" fn(param0: *mut AUTH_VALIDATION_EX) -> u32>;
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`*"]
pub type PMPRADMINRELEASEIPADRESS = ::core::option::Option<unsafe extern "system" fn(param0: ::windows::core::PCWSTR, param1: ::windows::core::PCWSTR, param2: *mut u32)>;
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`, `\"Win32_Networking_WinSock\"`*"]
#[cfg(feature = "Win32_Networking_WinSock")]
pub type PMPRADMINRELEASEIPV6ADDRESSFORUSER = ::core::option::Option<unsafe extern "system" fn(param0: ::windows::core::PCWSTR, param1: ::windows::core::PCWSTR, param2: *mut super::super::Networking::WinSock::IN6_ADDR)>;
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`*"]
pub type PMPRADMINTERMINATEDLL = ::core::option::Option<unsafe extern "system" fn() -> u32>;
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`*"]
pub struct PPP_ATCP_INFO {
    pub dwError: u32,
    pub wszAddress: [u16; 33],
}
impl ::core::marker::Copy for PPP_ATCP_INFO {}
impl ::core::clone::Clone for PPP_ATCP_INFO {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for PPP_ATCP_INFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("PPP_ATCP_INFO").field("dwError", &self.dwError).field("wszAddress", &self.wszAddress).finish()
    }
}
unsafe impl ::windows::core::Abi for PPP_ATCP_INFO {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for PPP_ATCP_INFO {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<PPP_ATCP_INFO>()) == 0 }
    }
}
impl ::core::cmp::Eq for PPP_ATCP_INFO {}
impl ::core::default::Default for PPP_ATCP_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`*"]
pub const PPP_CCP_COMPRESSION: u32 = 1u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`*"]
pub const PPP_CCP_ENCRYPTION128BIT: u32 = 64u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`*"]
pub const PPP_CCP_ENCRYPTION40BIT: u32 = 32u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`*"]
pub const PPP_CCP_ENCRYPTION40BITOLD: u32 = 16u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`*"]
pub const PPP_CCP_ENCRYPTION56BIT: u32 = 128u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`*"]
pub const PPP_CCP_HISTORYLESS: u32 = 16777216u32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`*"]
pub struct PPP_CCP_INFO {
    pub dwError: u32,
    pub dwCompressionAlgorithm: u32,
    pub dwOptions: u32,
    pub dwRemoteCompressionAlgorithm: u32,
    pub dwRemoteOptions: u32,
}
impl ::core::marker::Copy for PPP_CCP_INFO {}
impl ::core::clone::Clone for PPP_CCP_INFO {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for PPP_CCP_INFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("PPP_CCP_INFO").field("dwError", &self.dwError).field("dwCompressionAlgorithm", &self.dwCompressionAlgorithm).field("dwOptions", &self.dwOptions).field("dwRemoteCompressionAlgorithm", &self.dwRemoteCompressionAlgorithm).field("dwRemoteOptions", &self.dwRemoteOptions).finish()
    }
}
unsafe impl ::windows::core::Abi for PPP_CCP_INFO {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for PPP_CCP_INFO {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<PPP_CCP_INFO>()) == 0 }
    }
}
impl ::core::cmp::Eq for PPP_CCP_INFO {}
impl ::core::default::Default for PPP_CCP_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`*"]
pub struct PPP_INFO {
    pub nbf: PPP_NBFCP_INFO,
    pub ip: PPP_IPCP_INFO,
    pub ipx: PPP_IPXCP_INFO,
    pub at: PPP_ATCP_INFO,
}
impl ::core::marker::Copy for PPP_INFO {}
impl ::core::clone::Clone for PPP_INFO {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for PPP_INFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("PPP_INFO").field("nbf", &self.nbf).field("ip", &self.ip).field("ipx", &self.ipx).field("at", &self.at).finish()
    }
}
unsafe impl ::windows::core::Abi for PPP_INFO {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for PPP_INFO {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<PPP_INFO>()) == 0 }
    }
}
impl ::core::cmp::Eq for PPP_INFO {}
impl ::core::default::Default for PPP_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`*"]
pub struct PPP_INFO_2 {
    pub nbf: PPP_NBFCP_INFO,
    pub ip: PPP_IPCP_INFO2,
    pub ipx: PPP_IPXCP_INFO,
    pub at: PPP_ATCP_INFO,
    pub ccp: PPP_CCP_INFO,
    pub lcp: PPP_LCP_INFO,
}
impl ::core::marker::Copy for PPP_INFO_2 {}
impl ::core::clone::Clone for PPP_INFO_2 {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for PPP_INFO_2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("PPP_INFO_2").field("nbf", &self.nbf).field("ip", &self.ip).field("ipx", &self.ipx).field("at", &self.at).field("ccp", &self.ccp).field("lcp", &self.lcp).finish()
    }
}
unsafe impl ::windows::core::Abi for PPP_INFO_2 {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for PPP_INFO_2 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<PPP_INFO_2>()) == 0 }
    }
}
impl ::core::cmp::Eq for PPP_INFO_2 {}
impl ::core::default::Default for PPP_INFO_2 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`*"]
pub struct PPP_INFO_3 {
    pub nbf: PPP_NBFCP_INFO,
    pub ip: PPP_IPCP_INFO2,
    pub ipv6: PPP_IPV6_CP_INFO,
    pub ccp: PPP_CCP_INFO,
    pub lcp: PPP_LCP_INFO,
}
impl ::core::marker::Copy for PPP_INFO_3 {}
impl ::core::clone::Clone for PPP_INFO_3 {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for PPP_INFO_3 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("PPP_INFO_3").field("nbf", &self.nbf).field("ip", &self.ip).field("ipv6", &self.ipv6).field("ccp", &self.ccp).field("lcp", &self.lcp).finish()
    }
}
unsafe impl ::windows::core::Abi for PPP_INFO_3 {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for PPP_INFO_3 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<PPP_INFO_3>()) == 0 }
    }
}
impl ::core::cmp::Eq for PPP_INFO_3 {}
impl ::core::default::Default for PPP_INFO_3 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`*"]
pub struct PPP_IPCP_INFO {
    pub dwError: u32,
    pub wszAddress: [u16; 16],
    pub wszRemoteAddress: [u16; 16],
}
impl ::core::marker::Copy for PPP_IPCP_INFO {}
impl ::core::clone::Clone for PPP_IPCP_INFO {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for PPP_IPCP_INFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("PPP_IPCP_INFO").field("dwError", &self.dwError).field("wszAddress", &self.wszAddress).field("wszRemoteAddress", &self.wszRemoteAddress).finish()
    }
}
unsafe impl ::windows::core::Abi for PPP_IPCP_INFO {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for PPP_IPCP_INFO {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<PPP_IPCP_INFO>()) == 0 }
    }
}
impl ::core::cmp::Eq for PPP_IPCP_INFO {}
impl ::core::default::Default for PPP_IPCP_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`*"]
pub struct PPP_IPCP_INFO2 {
    pub dwError: u32,
    pub wszAddress: [u16; 16],
    pub wszRemoteAddress: [u16; 16],
    pub dwOptions: u32,
    pub dwRemoteOptions: u32,
}
impl ::core::marker::Copy for PPP_IPCP_INFO2 {}
impl ::core::clone::Clone for PPP_IPCP_INFO2 {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for PPP_IPCP_INFO2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("PPP_IPCP_INFO2").field("dwError", &self.dwError).field("wszAddress", &self.wszAddress).field("wszRemoteAddress", &self.wszRemoteAddress).field("dwOptions", &self.dwOptions).field("dwRemoteOptions", &self.dwRemoteOptions).finish()
    }
}
unsafe impl ::windows::core::Abi for PPP_IPCP_INFO2 {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for PPP_IPCP_INFO2 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<PPP_IPCP_INFO2>()) == 0 }
    }
}
impl ::core::cmp::Eq for PPP_IPCP_INFO2 {}
impl ::core::default::Default for PPP_IPCP_INFO2 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`*"]
pub const PPP_IPCP_VJ: u32 = 1u32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`*"]
pub struct PPP_IPV6_CP_INFO {
    pub dwVersion: u32,
    pub dwSize: u32,
    pub dwError: u32,
    pub bInterfaceIdentifier: [u8; 8],
    pub bRemoteInterfaceIdentifier: [u8; 8],
    pub dwOptions: u32,
    pub dwRemoteOptions: u32,
    pub bPrefix: [u8; 8],
    pub dwPrefixLength: u32,
}
impl ::core::marker::Copy for PPP_IPV6_CP_INFO {}
impl ::core::clone::Clone for PPP_IPV6_CP_INFO {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for PPP_IPV6_CP_INFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("PPP_IPV6_CP_INFO").field("dwVersion", &self.dwVersion).field("dwSize", &self.dwSize).field("dwError", &self.dwError).field("bInterfaceIdentifier", &self.bInterfaceIdentifier).field("bRemoteInterfaceIdentifier", &self.bRemoteInterfaceIdentifier).field("dwOptions", &self.dwOptions).field("dwRemoteOptions", &self.dwRemoteOptions).field("bPrefix", &self.bPrefix).field("dwPrefixLength", &self.dwPrefixLength).finish()
    }
}
unsafe impl ::windows::core::Abi for PPP_IPV6_CP_INFO {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for PPP_IPV6_CP_INFO {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<PPP_IPV6_CP_INFO>()) == 0 }
    }
}
impl ::core::cmp::Eq for PPP_IPV6_CP_INFO {}
impl ::core::default::Default for PPP_IPV6_CP_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`*"]
pub struct PPP_IPXCP_INFO {
    pub dwError: u32,
    pub wszAddress: [u16; 23],
}
impl ::core::marker::Copy for PPP_IPXCP_INFO {}
impl ::core::clone::Clone for PPP_IPXCP_INFO {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for PPP_IPXCP_INFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("PPP_IPXCP_INFO").field("dwError", &self.dwError).field("wszAddress", &self.wszAddress).finish()
    }
}
unsafe impl ::windows::core::Abi for PPP_IPXCP_INFO {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for PPP_IPXCP_INFO {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<PPP_IPXCP_INFO>()) == 0 }
    }
}
impl ::core::cmp::Eq for PPP_IPXCP_INFO {}
impl ::core::default::Default for PPP_IPXCP_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct PPP_LCP(pub u32);
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`*"]
pub const PPP_LCP_PAP: PPP_LCP = PPP_LCP(49187u32);
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`*"]
pub const PPP_LCP_CHAP: PPP_LCP = PPP_LCP(49699u32);
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`*"]
pub const PPP_LCP_EAP: PPP_LCP = PPP_LCP(49703u32);
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`*"]
pub const PPP_LCP_SPAP: PPP_LCP = PPP_LCP(49191u32);
impl ::core::marker::Copy for PPP_LCP {}
impl ::core::clone::Clone for PPP_LCP {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for PPP_LCP {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for PPP_LCP {
    type Abi = Self;
}
impl ::core::fmt::Debug for PPP_LCP {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PPP_LCP").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`*"]
pub const PPP_LCP_3_DES: u32 = 32u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`*"]
pub const PPP_LCP_ACFC: u32 = 4u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`*"]
pub const PPP_LCP_AES_128: u32 = 64u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`*"]
pub const PPP_LCP_AES_192: u32 = 256u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`*"]
pub const PPP_LCP_AES_256: u32 = 128u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`*"]
pub const PPP_LCP_DES_56: u32 = 16u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`*"]
pub const PPP_LCP_GCM_AES_128: u32 = 512u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`*"]
pub const PPP_LCP_GCM_AES_192: u32 = 1024u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`*"]
pub const PPP_LCP_GCM_AES_256: u32 = 2048u32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`*"]
pub struct PPP_LCP_INFO {
    pub dwError: u32,
    pub dwAuthenticationProtocol: PPP_LCP,
    pub dwAuthenticationData: PPP_LCP_INFO_AUTH_DATA,
    pub dwRemoteAuthenticationProtocol: u32,
    pub dwRemoteAuthenticationData: u32,
    pub dwTerminateReason: u32,
    pub dwRemoteTerminateReason: u32,
    pub dwOptions: u32,
    pub dwRemoteOptions: u32,
    pub dwEapTypeId: u32,
    pub dwRemoteEapTypeId: u32,
}
impl ::core::marker::Copy for PPP_LCP_INFO {}
impl ::core::clone::Clone for PPP_LCP_INFO {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for PPP_LCP_INFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("PPP_LCP_INFO")
            .field("dwError", &self.dwError)
            .field("dwAuthenticationProtocol", &self.dwAuthenticationProtocol)
            .field("dwAuthenticationData", &self.dwAuthenticationData)
            .field("dwRemoteAuthenticationProtocol", &self.dwRemoteAuthenticationProtocol)
            .field("dwRemoteAuthenticationData", &self.dwRemoteAuthenticationData)
            .field("dwTerminateReason", &self.dwTerminateReason)
            .field("dwRemoteTerminateReason", &self.dwRemoteTerminateReason)
            .field("dwOptions", &self.dwOptions)
            .field("dwRemoteOptions", &self.dwRemoteOptions)
            .field("dwEapTypeId", &self.dwEapTypeId)
            .field("dwRemoteEapTypeId", &self.dwRemoteEapTypeId)
            .finish()
    }
}
unsafe impl ::windows::core::Abi for PPP_LCP_INFO {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for PPP_LCP_INFO {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<PPP_LCP_INFO>()) == 0 }
    }
}
impl ::core::cmp::Eq for PPP_LCP_INFO {}
impl ::core::default::Default for PPP_LCP_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct PPP_LCP_INFO_AUTH_DATA(pub u32);
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`*"]
pub const PPP_LCP_CHAP_MD5: PPP_LCP_INFO_AUTH_DATA = PPP_LCP_INFO_AUTH_DATA(5u32);
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`*"]
pub const PPP_LCP_CHAP_MS: PPP_LCP_INFO_AUTH_DATA = PPP_LCP_INFO_AUTH_DATA(128u32);
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`*"]
pub const PPP_LCP_CHAP_MSV2: PPP_LCP_INFO_AUTH_DATA = PPP_LCP_INFO_AUTH_DATA(129u32);
impl ::core::marker::Copy for PPP_LCP_INFO_AUTH_DATA {}
impl ::core::clone::Clone for PPP_LCP_INFO_AUTH_DATA {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for PPP_LCP_INFO_AUTH_DATA {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for PPP_LCP_INFO_AUTH_DATA {
    type Abi = Self;
}
impl ::core::fmt::Debug for PPP_LCP_INFO_AUTH_DATA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PPP_LCP_INFO_AUTH_DATA").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`*"]
pub const PPP_LCP_MULTILINK_FRAMING: u32 = 1u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`*"]
pub const PPP_LCP_PFC: u32 = 2u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`*"]
pub const PPP_LCP_SSHF: u32 = 8u32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`*"]
pub struct PPP_NBFCP_INFO {
    pub dwError: u32,
    pub wszWksta: [u16; 17],
}
impl ::core::marker::Copy for PPP_NBFCP_INFO {}
impl ::core::clone::Clone for PPP_NBFCP_INFO {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for PPP_NBFCP_INFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("PPP_NBFCP_INFO").field("dwError", &self.dwError).field("wszWksta", &self.wszWksta).finish()
    }
}
unsafe impl ::windows::core::Abi for PPP_NBFCP_INFO {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for PPP_NBFCP_INFO {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<PPP_NBFCP_INFO>()) == 0 }
    }
}
impl ::core::cmp::Eq for PPP_NBFCP_INFO {}
impl ::core::default::Default for PPP_NBFCP_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`*"]
pub struct PPP_PROJECTION_INFO {
    pub dwIPv4NegotiationError: u32,
    pub wszAddress: [u16; 16],
    pub wszRemoteAddress: [u16; 16],
    pub dwIPv4Options: u32,
    pub dwIPv4RemoteOptions: u32,
    pub IPv4SubInterfaceIndex: u64,
    pub dwIPv6NegotiationError: u32,
    pub bInterfaceIdentifier: [u8; 8],
    pub bRemoteInterfaceIdentifier: [u8; 8],
    pub bPrefix: [u8; 8],
    pub dwPrefixLength: u32,
    pub IPv6SubInterfaceIndex: u64,
    pub dwLcpError: u32,
    pub dwAuthenticationProtocol: PPP_LCP,
    pub dwAuthenticationData: PPP_LCP_INFO_AUTH_DATA,
    pub dwRemoteAuthenticationProtocol: PPP_LCP,
    pub dwRemoteAuthenticationData: PPP_LCP_INFO_AUTH_DATA,
    pub dwLcpTerminateReason: u32,
    pub dwLcpRemoteTerminateReason: u32,
    pub dwLcpOptions: u32,
    pub dwLcpRemoteOptions: u32,
    pub dwEapTypeId: u32,
    pub dwRemoteEapTypeId: u32,
    pub dwCcpError: u32,
    pub dwCompressionAlgorithm: u32,
    pub dwCcpOptions: u32,
    pub dwRemoteCompressionAlgorithm: u32,
    pub dwCcpRemoteOptions: u32,
}
impl ::core::marker::Copy for PPP_PROJECTION_INFO {}
impl ::core::clone::Clone for PPP_PROJECTION_INFO {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for PPP_PROJECTION_INFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("PPP_PROJECTION_INFO")
            .field("dwIPv4NegotiationError", &self.dwIPv4NegotiationError)
            .field("wszAddress", &self.wszAddress)
            .field("wszRemoteAddress", &self.wszRemoteAddress)
            .field("dwIPv4Options", &self.dwIPv4Options)
            .field("dwIPv4RemoteOptions", &self.dwIPv4RemoteOptions)
            .field("IPv4SubInterfaceIndex", &self.IPv4SubInterfaceIndex)
            .field("dwIPv6NegotiationError", &self.dwIPv6NegotiationError)
            .field("bInterfaceIdentifier", &self.bInterfaceIdentifier)
            .field("bRemoteInterfaceIdentifier", &self.bRemoteInterfaceIdentifier)
            .field("bPrefix", &self.bPrefix)
            .field("dwPrefixLength", &self.dwPrefixLength)
            .field("IPv6SubInterfaceIndex", &self.IPv6SubInterfaceIndex)
            .field("dwLcpError", &self.dwLcpError)
            .field("dwAuthenticationProtocol", &self.dwAuthenticationProtocol)
            .field("dwAuthenticationData", &self.dwAuthenticationData)
            .field("dwRemoteAuthenticationProtocol", &self.dwRemoteAuthenticationProtocol)
            .field("dwRemoteAuthenticationData", &self.dwRemoteAuthenticationData)
            .field("dwLcpTerminateReason", &self.dwLcpTerminateReason)
            .field("dwLcpRemoteTerminateReason", &self.dwLcpRemoteTerminateReason)
            .field("dwLcpOptions", &self.dwLcpOptions)
            .field("dwLcpRemoteOptions", &self.dwLcpRemoteOptions)
            .field("dwEapTypeId", &self.dwEapTypeId)
            .field("dwRemoteEapTypeId", &self.dwRemoteEapTypeId)
            .field("dwCcpError", &self.dwCcpError)
            .field("dwCompressionAlgorithm", &self.dwCompressionAlgorithm)
            .field("dwCcpOptions", &self.dwCcpOptions)
            .field("dwRemoteCompressionAlgorithm", &self.dwRemoteCompressionAlgorithm)
            .field("dwCcpRemoteOptions", &self.dwCcpRemoteOptions)
            .finish()
    }
}
unsafe impl ::windows::core::Abi for PPP_PROJECTION_INFO {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for PPP_PROJECTION_INFO {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<PPP_PROJECTION_INFO>()) == 0 }
    }
}
impl ::core::cmp::Eq for PPP_PROJECTION_INFO {}
impl ::core::default::Default for PPP_PROJECTION_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`*"]
pub struct PPP_PROJECTION_INFO2 {
    pub dwIPv4NegotiationError: u32,
    pub wszAddress: [u16; 16],
    pub wszRemoteAddress: [u16; 16],
    pub dwIPv4Options: u32,
    pub dwIPv4RemoteOptions: u32,
    pub IPv4SubInterfaceIndex: u64,
    pub dwIPv6NegotiationError: u32,
    pub bInterfaceIdentifier: [u8; 8],
    pub bRemoteInterfaceIdentifier: [u8; 8],
    pub bPrefix: [u8; 8],
    pub dwPrefixLength: u32,
    pub IPv6SubInterfaceIndex: u64,
    pub dwLcpError: u32,
    pub dwAuthenticationProtocol: PPP_LCP,
    pub dwAuthenticationData: PPP_LCP_INFO_AUTH_DATA,
    pub dwRemoteAuthenticationProtocol: PPP_LCP,
    pub dwRemoteAuthenticationData: PPP_LCP_INFO_AUTH_DATA,
    pub dwLcpTerminateReason: u32,
    pub dwLcpRemoteTerminateReason: u32,
    pub dwLcpOptions: u32,
    pub dwLcpRemoteOptions: u32,
    pub dwEapTypeId: u32,
    pub dwEmbeddedEAPTypeId: u32,
    pub dwRemoteEapTypeId: u32,
    pub dwCcpError: u32,
    pub dwCompressionAlgorithm: u32,
    pub dwCcpOptions: u32,
    pub dwRemoteCompressionAlgorithm: u32,
    pub dwCcpRemoteOptions: u32,
}
impl ::core::marker::Copy for PPP_PROJECTION_INFO2 {}
impl ::core::clone::Clone for PPP_PROJECTION_INFO2 {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for PPP_PROJECTION_INFO2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("PPP_PROJECTION_INFO2")
            .field("dwIPv4NegotiationError", &self.dwIPv4NegotiationError)
            .field("wszAddress", &self.wszAddress)
            .field("wszRemoteAddress", &self.wszRemoteAddress)
            .field("dwIPv4Options", &self.dwIPv4Options)
            .field("dwIPv4RemoteOptions", &self.dwIPv4RemoteOptions)
            .field("IPv4SubInterfaceIndex", &self.IPv4SubInterfaceIndex)
            .field("dwIPv6NegotiationError", &self.dwIPv6NegotiationError)
            .field("bInterfaceIdentifier", &self.bInterfaceIdentifier)
            .field("bRemoteInterfaceIdentifier", &self.bRemoteInterfaceIdentifier)
            .field("bPrefix", &self.bPrefix)
            .field("dwPrefixLength", &self.dwPrefixLength)
            .field("IPv6SubInterfaceIndex", &self.IPv6SubInterfaceIndex)
            .field("dwLcpError", &self.dwLcpError)
            .field("dwAuthenticationProtocol", &self.dwAuthenticationProtocol)
            .field("dwAuthenticationData", &self.dwAuthenticationData)
            .field("dwRemoteAuthenticationProtocol", &self.dwRemoteAuthenticationProtocol)
            .field("dwRemoteAuthenticationData", &self.dwRemoteAuthenticationData)
            .field("dwLcpTerminateReason", &self.dwLcpTerminateReason)
            .field("dwLcpRemoteTerminateReason", &self.dwLcpRemoteTerminateReason)
            .field("dwLcpOptions", &self.dwLcpOptions)
            .field("dwLcpRemoteOptions", &self.dwLcpRemoteOptions)
            .field("dwEapTypeId", &self.dwEapTypeId)
            .field("dwEmbeddedEAPTypeId", &self.dwEmbeddedEAPTypeId)
            .field("dwRemoteEapTypeId", &self.dwRemoteEapTypeId)
            .field("dwCcpError", &self.dwCcpError)
            .field("dwCompressionAlgorithm", &self.dwCompressionAlgorithm)
            .field("dwCcpOptions", &self.dwCcpOptions)
            .field("dwRemoteCompressionAlgorithm", &self.dwRemoteCompressionAlgorithm)
            .field("dwCcpRemoteOptions", &self.dwCcpRemoteOptions)
            .finish()
    }
}
unsafe impl ::windows::core::Abi for PPP_PROJECTION_INFO2 {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for PPP_PROJECTION_INFO2 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<PPP_PROJECTION_INFO2>()) == 0 }
    }
}
impl ::core::cmp::Eq for PPP_PROJECTION_INFO2 {}
impl ::core::default::Default for PPP_PROJECTION_INFO2 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`*"]
pub struct PPTP_CONFIG_PARAMS {
    pub dwNumPorts: u32,
    pub dwPortFlags: u32,
}
impl ::core::marker::Copy for PPTP_CONFIG_PARAMS {}
impl ::core::clone::Clone for PPTP_CONFIG_PARAMS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for PPTP_CONFIG_PARAMS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("PPTP_CONFIG_PARAMS").field("dwNumPorts", &self.dwNumPorts).field("dwPortFlags", &self.dwPortFlags).finish()
    }
}
unsafe impl ::windows::core::Abi for PPTP_CONFIG_PARAMS {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for PPTP_CONFIG_PARAMS {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<PPTP_CONFIG_PARAMS>()) == 0 }
    }
}
impl ::core::cmp::Eq for PPTP_CONFIG_PARAMS {}
impl ::core::default::Default for PPTP_CONFIG_PARAMS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`*"]
pub struct PROJECTION_INFO {
    pub projectionInfoType: u8,
    pub Anonymous: PROJECTION_INFO_0,
}
impl ::core::marker::Copy for PROJECTION_INFO {}
impl ::core::clone::Clone for PROJECTION_INFO {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for PROJECTION_INFO {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for PROJECTION_INFO {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<PROJECTION_INFO>()) == 0 }
    }
}
impl ::core::cmp::Eq for PROJECTION_INFO {}
impl ::core::default::Default for PROJECTION_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`*"]
pub union PROJECTION_INFO_0 {
    pub PppProjectionInfo: PPP_PROJECTION_INFO,
    pub Ikev2ProjectionInfo: IKEV2_PROJECTION_INFO,
}
impl ::core::marker::Copy for PROJECTION_INFO_0 {}
impl ::core::clone::Clone for PROJECTION_INFO_0 {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for PROJECTION_INFO_0 {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for PROJECTION_INFO_0 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<PROJECTION_INFO_0>()) == 0 }
    }
}
impl ::core::cmp::Eq for PROJECTION_INFO_0 {}
impl ::core::default::Default for PROJECTION_INFO_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`*"]
pub struct PROJECTION_INFO2 {
    pub projectionInfoType: u8,
    pub Anonymous: PROJECTION_INFO2_0,
}
impl ::core::marker::Copy for PROJECTION_INFO2 {}
impl ::core::clone::Clone for PROJECTION_INFO2 {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for PROJECTION_INFO2 {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for PROJECTION_INFO2 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<PROJECTION_INFO2>()) == 0 }
    }
}
impl ::core::cmp::Eq for PROJECTION_INFO2 {}
impl ::core::default::Default for PROJECTION_INFO2 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`*"]
pub union PROJECTION_INFO2_0 {
    pub PppProjectionInfo: PPP_PROJECTION_INFO2,
    pub Ikev2ProjectionInfo: IKEV2_PROJECTION_INFO2,
}
impl ::core::marker::Copy for PROJECTION_INFO2_0 {}
impl ::core::clone::Clone for PROJECTION_INFO2_0 {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for PROJECTION_INFO2_0 {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for PROJECTION_INFO2_0 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<PROJECTION_INFO2_0>()) == 0 }
    }
}
impl ::core::cmp::Eq for PROJECTION_INFO2_0 {}
impl ::core::default::Default for PROJECTION_INFO2_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`*"]
pub const RASADFLG_PositionDlg: u32 = 1u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub type RASADFUNCA = ::core::option::Option<unsafe extern "system" fn(param0: ::windows::core::PCSTR, param1: ::windows::core::PCSTR, param2: *mut RASADPARAMS, param3: *mut u32) -> super::super::Foundation::BOOL>;
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub type RASADFUNCW = ::core::option::Option<unsafe extern "system" fn(param0: ::windows::core::PCWSTR, param1: ::windows::core::PCWSTR, param2: *mut RASADPARAMS, param3: *mut u32) -> super::super::Foundation::BOOL>;
#[repr(C, packed(4))]
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct RASADPARAMS {
    pub dwSize: u32,
    pub hwndOwner: super::super::Foundation::HWND,
    pub dwFlags: u32,
    pub xDlg: i32,
    pub yDlg: i32,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for RASADPARAMS {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for RASADPARAMS {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for RASADPARAMS {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for RASADPARAMS {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<RASADPARAMS>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for RASADPARAMS {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for RASADPARAMS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`*"]
pub const RASADP_ConnectionQueryTimeout: u32 = 4u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`*"]
pub const RASADP_DisableConnectionQuery: u32 = 0u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`*"]
pub const RASADP_FailedConnectionTimeout: u32 = 3u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`*"]
pub const RASADP_LoginSessionDisable: u32 = 1u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`*"]
pub const RASADP_SavedAddressesLimit: u32 = 2u32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct RASAMBA {
    pub dwSize: u32,
    pub dwError: u32,
    pub szNetBiosError: [super::super::Foundation::CHAR; 17],
    pub bLana: u8,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for RASAMBA {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for RASAMBA {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for RASAMBA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("RASAMBA").field("dwSize", &self.dwSize).field("dwError", &self.dwError).field("szNetBiosError", &self.szNetBiosError).field("bLana", &self.bLana).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for RASAMBA {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for RASAMBA {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<RASAMBA>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for RASAMBA {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for RASAMBA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`*"]
pub struct RASAMBW {
    pub dwSize: u32,
    pub dwError: u32,
    pub szNetBiosError: [u16; 17],
    pub bLana: u8,
}
impl ::core::marker::Copy for RASAMBW {}
impl ::core::clone::Clone for RASAMBW {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for RASAMBW {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("RASAMBW").field("dwSize", &self.dwSize).field("dwError", &self.dwError).field("szNetBiosError", &self.szNetBiosError).field("bLana", &self.bLana).finish()
    }
}
unsafe impl ::windows::core::Abi for RASAMBW {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for RASAMBW {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<RASAMBW>()) == 0 }
    }
}
impl ::core::cmp::Eq for RASAMBW {}
impl ::core::default::Default for RASAMBW {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct RASAPIVERSION(pub i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`*"]
pub const RASAPIVERSION_500: RASAPIVERSION = RASAPIVERSION(1i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`*"]
pub const RASAPIVERSION_501: RASAPIVERSION = RASAPIVERSION(2i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`*"]
pub const RASAPIVERSION_600: RASAPIVERSION = RASAPIVERSION(3i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`*"]
pub const RASAPIVERSION_601: RASAPIVERSION = RASAPIVERSION(4i32);
impl ::core::marker::Copy for RASAPIVERSION {}
impl ::core::clone::Clone for RASAPIVERSION {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for RASAPIVERSION {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for RASAPIVERSION {
    type Abi = Self;
}
impl ::core::fmt::Debug for RASAPIVERSION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("RASAPIVERSION").field(&self.0).finish()
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct RASAUTODIALENTRYA {
    pub dwSize: u32,
    pub dwFlags: u32,
    pub dwDialingLocation: u32,
    pub szEntry: [super::super::Foundation::CHAR; 257],
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for RASAUTODIALENTRYA {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for RASAUTODIALENTRYA {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for RASAUTODIALENTRYA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("RASAUTODIALENTRYA").field("dwSize", &self.dwSize).field("dwFlags", &self.dwFlags).field("dwDialingLocation", &self.dwDialingLocation).field("szEntry", &self.szEntry).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for RASAUTODIALENTRYA {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for RASAUTODIALENTRYA {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<RASAUTODIALENTRYA>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for RASAUTODIALENTRYA {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for RASAUTODIALENTRYA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`*"]
pub struct RASAUTODIALENTRYW {
    pub dwSize: u32,
    pub dwFlags: u32,
    pub dwDialingLocation: u32,
    pub szEntry: [u16; 257],
}
impl ::core::marker::Copy for RASAUTODIALENTRYW {}
impl ::core::clone::Clone for RASAUTODIALENTRYW {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for RASAUTODIALENTRYW {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("RASAUTODIALENTRYW").field("dwSize", &self.dwSize).field("dwFlags", &self.dwFlags).field("dwDialingLocation", &self.dwDialingLocation).field("szEntry", &self.szEntry).finish()
    }
}
unsafe impl ::windows::core::Abi for RASAUTODIALENTRYW {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for RASAUTODIALENTRYW {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<RASAUTODIALENTRYW>()) == 0 }
    }
}
impl ::core::cmp::Eq for RASAUTODIALENTRYW {}
impl ::core::default::Default for RASAUTODIALENTRYW {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`*"]
pub const RASBASE: u32 = 600u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`*"]
pub const RASBASEEND: u32 = 877u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`*"]
pub const RASCCPCA_MPPC: u32 = 6u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`*"]
pub const RASCCPCA_STAC: u32 = 5u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`*"]
pub const RASCCPO_Compression: u32 = 1u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`*"]
pub const RASCCPO_Encryption128bit: u32 = 64u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`*"]
pub const RASCCPO_Encryption40bit: u32 = 32u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`*"]
pub const RASCCPO_Encryption56bit: u32 = 16u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`*"]
pub const RASCCPO_HistoryLess: u32 = 2u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`*"]
pub const RASCF_AllUsers: u32 = 1u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`*"]
pub const RASCF_GlobalCreds: u32 = 2u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`*"]
pub const RASCF_OwnerKnown: u32 = 4u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`*"]
pub const RASCF_OwnerMatch: u32 = 8u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`*"]
pub const RASCM_DDMPreSharedKey: u32 = 64u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`*"]
pub const RASCM_DefaultCreds: u32 = 8u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`*"]
pub const RASCM_Domain: u32 = 4u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`*"]
pub const RASCM_Password: u32 = 2u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`*"]
pub const RASCM_PreSharedKey: u32 = 16u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`*"]
pub const RASCM_ServerPreSharedKey: u32 = 32u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`*"]
pub const RASCM_UserName: u32 = 1u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`*"]
pub const RASCN_BandwidthAdded: u32 = 4u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`*"]
pub const RASCN_BandwidthRemoved: u32 = 8u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`*"]
pub const RASCN_Connection: u32 = 1u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`*"]
pub const RASCN_Disconnection: u32 = 2u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`*"]
pub const RASCN_Dormant: u32 = 16u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`*"]
pub const RASCN_EPDGPacketArrival: u32 = 64u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`*"]
pub const RASCN_ReConnection: u32 = 32u32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`*"]
pub struct RASCOMMSETTINGS {
    pub dwSize: u32,
    pub bParity: u8,
    pub bStop: u8,
    pub bByteSize: u8,
    pub bAlign: u8,
}
impl ::core::marker::Copy for RASCOMMSETTINGS {}
impl ::core::clone::Clone for RASCOMMSETTINGS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for RASCOMMSETTINGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("RASCOMMSETTINGS").field("dwSize", &self.dwSize).field("bParity", &self.bParity).field("bStop", &self.bStop).field("bByteSize", &self.bByteSize).field("bAlign", &self.bAlign).finish()
    }
}
unsafe impl ::windows::core::Abi for RASCOMMSETTINGS {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for RASCOMMSETTINGS {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<RASCOMMSETTINGS>()) == 0 }
    }
}
impl ::core::cmp::Eq for RASCOMMSETTINGS {}
impl ::core::default::Default for RASCOMMSETTINGS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C, packed(4))]
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct RASCONNA {
    pub dwSize: u32,
    pub hrasconn: HRASCONN,
    pub szEntryName: [super::super::Foundation::CHAR; 257],
    pub szDeviceType: [super::super::Foundation::CHAR; 17],
    pub szDeviceName: [super::super::Foundation::CHAR; 129],
    pub szPhonebook: [super::super::Foundation::CHAR; 260],
    pub dwSubEntry: u32,
    pub guidEntry: ::windows::core::GUID,
    pub dwFlags: u32,
    pub luid: super::super::Foundation::LUID,
    pub guidCorrelationId: ::windows::core::GUID,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for RASCONNA {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for RASCONNA {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for RASCONNA {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for RASCONNA {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<RASCONNA>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for RASCONNA {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for RASCONNA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct RASCONNSTATE(pub i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`*"]
pub const RASCS_OpenPort: RASCONNSTATE = RASCONNSTATE(0i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`*"]
pub const RASCS_PortOpened: RASCONNSTATE = RASCONNSTATE(1i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`*"]
pub const RASCS_ConnectDevice: RASCONNSTATE = RASCONNSTATE(2i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`*"]
pub const RASCS_DeviceConnected: RASCONNSTATE = RASCONNSTATE(3i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`*"]
pub const RASCS_AllDevicesConnected: RASCONNSTATE = RASCONNSTATE(4i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`*"]
pub const RASCS_Authenticate: RASCONNSTATE = RASCONNSTATE(5i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`*"]
pub const RASCS_AuthNotify: RASCONNSTATE = RASCONNSTATE(6i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`*"]
pub const RASCS_AuthRetry: RASCONNSTATE = RASCONNSTATE(7i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`*"]
pub const RASCS_AuthCallback: RASCONNSTATE = RASCONNSTATE(8i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`*"]
pub const RASCS_AuthChangePassword: RASCONNSTATE = RASCONNSTATE(9i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`*"]
pub const RASCS_AuthProject: RASCONNSTATE = RASCONNSTATE(10i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`*"]
pub const RASCS_AuthLinkSpeed: RASCONNSTATE = RASCONNSTATE(11i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`*"]
pub const RASCS_AuthAck: RASCONNSTATE = RASCONNSTATE(12i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`*"]
pub const RASCS_ReAuthenticate: RASCONNSTATE = RASCONNSTATE(13i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`*"]
pub const RASCS_Authenticated: RASCONNSTATE = RASCONNSTATE(14i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`*"]
pub const RASCS_PrepareForCallback: RASCONNSTATE = RASCONNSTATE(15i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`*"]
pub const RASCS_WaitForModemReset: RASCONNSTATE = RASCONNSTATE(16i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`*"]
pub const RASCS_WaitForCallback: RASCONNSTATE = RASCONNSTATE(17i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`*"]
pub const RASCS_Projected: RASCONNSTATE = RASCONNSTATE(18i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`*"]
pub const RASCS_StartAuthentication: RASCONNSTATE = RASCONNSTATE(19i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`*"]
pub const RASCS_CallbackComplete: RASCONNSTATE = RASCONNSTATE(20i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`*"]
pub const RASCS_LogonNetwork: RASCONNSTATE = RASCONNSTATE(21i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`*"]
pub const RASCS_SubEntryConnected: RASCONNSTATE = RASCONNSTATE(22i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`*"]
pub const RASCS_SubEntryDisconnected: RASCONNSTATE = RASCONNSTATE(23i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`*"]
pub const RASCS_ApplySettings: RASCONNSTATE = RASCONNSTATE(24i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`*"]
pub const RASCS_Interactive: RASCONNSTATE = RASCONNSTATE(4096i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`*"]
pub const RASCS_RetryAuthentication: RASCONNSTATE = RASCONNSTATE(4097i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`*"]
pub const RASCS_CallbackSetByCaller: RASCONNSTATE = RASCONNSTATE(4098i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`*"]
pub const RASCS_PasswordExpired: RASCONNSTATE = RASCONNSTATE(4099i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`*"]
pub const RASCS_InvokeEapUI: RASCONNSTATE = RASCONNSTATE(4100i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`*"]
pub const RASCS_Connected: RASCONNSTATE = RASCONNSTATE(8192i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`*"]
pub const RASCS_Disconnected: RASCONNSTATE = RASCONNSTATE(8193i32);
impl ::core::marker::Copy for RASCONNSTATE {}
impl ::core::clone::Clone for RASCONNSTATE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for RASCONNSTATE {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for RASCONNSTATE {
    type Abi = Self;
}
impl ::core::fmt::Debug for RASCONNSTATE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("RASCONNSTATE").field(&self.0).finish()
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`, `\"Win32_Foundation\"`, `\"Win32_Networking_WinSock\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
pub struct RASCONNSTATUSA {
    pub dwSize: u32,
    pub rasconnstate: RASCONNSTATE,
    pub dwError: u32,
    pub szDeviceType: [super::super::Foundation::CHAR; 17],
    pub szDeviceName: [super::super::Foundation::CHAR; 129],
    pub szPhoneNumber: [super::super::Foundation::CHAR; 129],
    pub localEndPoint: RASTUNNELENDPOINT,
    pub remoteEndPoint: RASTUNNELENDPOINT,
    pub rasconnsubstate: RASCONNSUBSTATE,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
impl ::core::marker::Copy for RASCONNSTATUSA {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
impl ::core::clone::Clone for RASCONNSTATUSA {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
unsafe impl ::windows::core::Abi for RASCONNSTATUSA {
    type Abi = Self;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
impl ::core::cmp::PartialEq for RASCONNSTATUSA {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<RASCONNSTATUSA>()) == 0 }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
impl ::core::cmp::Eq for RASCONNSTATUSA {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
impl ::core::default::Default for RASCONNSTATUSA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`, `\"Win32_Networking_WinSock\"`*"]
#[cfg(feature = "Win32_Networking_WinSock")]
pub struct RASCONNSTATUSW {
    pub dwSize: u32,
    pub rasconnstate: RASCONNSTATE,
    pub dwError: u32,
    pub szDeviceType: [u16; 17],
    pub szDeviceName: [u16; 129],
    pub szPhoneNumber: [u16; 129],
    pub localEndPoint: RASTUNNELENDPOINT,
    pub remoteEndPoint: RASTUNNELENDPOINT,
    pub rasconnsubstate: RASCONNSUBSTATE,
}
#[cfg(feature = "Win32_Networking_WinSock")]
impl ::core::marker::Copy for RASCONNSTATUSW {}
#[cfg(feature = "Win32_Networking_WinSock")]
impl ::core::clone::Clone for RASCONNSTATUSW {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Networking_WinSock")]
unsafe impl ::windows::core::Abi for RASCONNSTATUSW {
    type Abi = Self;
}
#[cfg(feature = "Win32_Networking_WinSock")]
impl ::core::cmp::PartialEq for RASCONNSTATUSW {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<RASCONNSTATUSW>()) == 0 }
    }
}
#[cfg(feature = "Win32_Networking_WinSock")]
impl ::core::cmp::Eq for RASCONNSTATUSW {}
#[cfg(feature = "Win32_Networking_WinSock")]
impl ::core::default::Default for RASCONNSTATUSW {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct RASCONNSUBSTATE(pub i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`*"]
pub const RASCSS_None: RASCONNSUBSTATE = RASCONNSUBSTATE(0i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`*"]
pub const RASCSS_Dormant: RASCONNSUBSTATE = RASCONNSUBSTATE(1i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`*"]
pub const RASCSS_Reconnecting: RASCONNSUBSTATE = RASCONNSUBSTATE(2i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`*"]
pub const RASCSS_Reconnected: RASCONNSUBSTATE = RASCONNSUBSTATE(8192i32);
impl ::core::marker::Copy for RASCONNSUBSTATE {}
impl ::core::clone::Clone for RASCONNSUBSTATE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for RASCONNSUBSTATE {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for RASCONNSUBSTATE {
    type Abi = Self;
}
impl ::core::fmt::Debug for RASCONNSUBSTATE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("RASCONNSUBSTATE").field(&self.0).finish()
    }
}
#[repr(C, packed(4))]
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct RASCONNW {
    pub dwSize: u32,
    pub hrasconn: HRASCONN,
    pub szEntryName: [u16; 257],
    pub szDeviceType: [u16; 17],
    pub szDeviceName: [u16; 129],
    pub szPhonebook: [u16; 260],
    pub dwSubEntry: u32,
    pub guidEntry: ::windows::core::GUID,
    pub dwFlags: u32,
    pub luid: super::super::Foundation::LUID,
    pub guidCorrelationId: ::windows::core::GUID,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for RASCONNW {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for RASCONNW {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for RASCONNW {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for RASCONNW {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<RASCONNW>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for RASCONNW {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for RASCONNW {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct RASCREDENTIALSA {
    pub dwSize: u32,
    pub dwMask: u32,
    pub szUserName: [super::super::Foundation::CHAR; 257],
    pub szPassword: [super::super::Foundation::CHAR; 257],
    pub szDomain: [super::super::Foundation::CHAR; 16],
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for RASCREDENTIALSA {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for RASCREDENTIALSA {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for RASCREDENTIALSA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("RASCREDENTIALSA").field("dwSize", &self.dwSize).field("dwMask", &self.dwMask).field("szUserName", &self.szUserName).field("szPassword", &self.szPassword).field("szDomain", &self.szDomain).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for RASCREDENTIALSA {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for RASCREDENTIALSA {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<RASCREDENTIALSA>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for RASCREDENTIALSA {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for RASCREDENTIALSA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`*"]
pub struct RASCREDENTIALSW {
    pub dwSize: u32,
    pub dwMask: u32,
    pub szUserName: [u16; 257],
    pub szPassword: [u16; 257],
    pub szDomain: [u16; 16],
}
impl ::core::marker::Copy for RASCREDENTIALSW {}
impl ::core::clone::Clone for RASCREDENTIALSW {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for RASCREDENTIALSW {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("RASCREDENTIALSW").field("dwSize", &self.dwSize).field("dwMask", &self.dwMask).field("szUserName", &self.szUserName).field("szPassword", &self.szPassword).field("szDomain", &self.szDomain).finish()
    }
}
unsafe impl ::windows::core::Abi for RASCREDENTIALSW {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for RASCREDENTIALSW {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<RASCREDENTIALSW>()) == 0 }
    }
}
impl ::core::cmp::Eq for RASCREDENTIALSW {}
impl ::core::default::Default for RASCREDENTIALSW {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`*"]
pub const RASCSS_DONE: u32 = 8192u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`*"]
pub const RASCS_DONE: u32 = 8192u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`*"]
pub const RASCS_PAUSED: u32 = 4096u32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`*"]
pub struct RASCTRYINFO {
    pub dwSize: u32,
    pub dwCountryID: u32,
    pub dwNextCountryID: u32,
    pub dwCountryCode: u32,
    pub dwCountryNameOffset: u32,
}
impl ::core::marker::Copy for RASCTRYINFO {}
impl ::core::clone::Clone for RASCTRYINFO {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for RASCTRYINFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("RASCTRYINFO").field("dwSize", &self.dwSize).field("dwCountryID", &self.dwCountryID).field("dwNextCountryID", &self.dwNextCountryID).field("dwCountryCode", &self.dwCountryCode).field("dwCountryNameOffset", &self.dwCountryNameOffset).finish()
    }
}
unsafe impl ::windows::core::Abi for RASCTRYINFO {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for RASCTRYINFO {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<RASCTRYINFO>()) == 0 }
    }
}
impl ::core::cmp::Eq for RASCTRYINFO {}
impl ::core::default::Default for RASCTRYINFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C, packed(4))]
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct RASCUSTOMSCRIPTEXTENSIONS {
    pub dwSize: u32,
    pub pfnRasSetCommSettings: PFNRASSETCOMMSETTINGS,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for RASCUSTOMSCRIPTEXTENSIONS {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for RASCUSTOMSCRIPTEXTENSIONS {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for RASCUSTOMSCRIPTEXTENSIONS {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for RASCUSTOMSCRIPTEXTENSIONS {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<RASCUSTOMSCRIPTEXTENSIONS>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for RASCUSTOMSCRIPTEXTENSIONS {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for RASCUSTOMSCRIPTEXTENSIONS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`*"]
pub const RASDDFLAG_AoacRedial: u32 = 4u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`*"]
pub const RASDDFLAG_LinkFailure: u32 = 2147483648u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`*"]
pub const RASDDFLAG_NoPrompt: u32 = 2u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`*"]
pub const RASDDFLAG_PositionDlg: u32 = 1u32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct RASDEVINFOA {
    pub dwSize: u32,
    pub szDeviceType: [super::super::Foundation::CHAR; 17],
    pub szDeviceName: [super::super::Foundation::CHAR; 129],
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for RASDEVINFOA {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for RASDEVINFOA {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for RASDEVINFOA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("RASDEVINFOA").field("dwSize", &self.dwSize).field("szDeviceType", &self.szDeviceType).field("szDeviceName", &self.szDeviceName).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for RASDEVINFOA {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for RASDEVINFOA {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<RASDEVINFOA>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for RASDEVINFOA {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for RASDEVINFOA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`*"]
pub struct RASDEVINFOW {
    pub dwSize: u32,
    pub szDeviceType: [u16; 17],
    pub szDeviceName: [u16; 129],
}
impl ::core::marker::Copy for RASDEVINFOW {}
impl ::core::clone::Clone for RASDEVINFOW {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for RASDEVINFOW {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("RASDEVINFOW").field("dwSize", &self.dwSize).field("szDeviceType", &self.szDeviceType).field("szDeviceName", &self.szDeviceName).finish()
    }
}
unsafe impl ::windows::core::Abi for RASDEVINFOW {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for RASDEVINFOW {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<RASDEVINFOW>()) == 0 }
    }
}
impl ::core::cmp::Eq for RASDEVINFOW {}
impl ::core::default::Default for RASDEVINFOW {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C, packed(4))]
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`*"]
pub struct RASDEVSPECIFICINFO {
    pub dwSize: u32,
    pub pbDevSpecificInfo: *mut u8,
}
impl ::core::marker::Copy for RASDEVSPECIFICINFO {}
impl ::core::clone::Clone for RASDEVSPECIFICINFO {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for RASDEVSPECIFICINFO {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for RASDEVSPECIFICINFO {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<RASDEVSPECIFICINFO>()) == 0 }
    }
}
impl ::core::cmp::Eq for RASDEVSPECIFICINFO {}
impl ::core::default::Default for RASDEVSPECIFICINFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C, packed(4))]
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct RASDIALDLG {
    pub dwSize: u32,
    pub hwndOwner: super::super::Foundation::HWND,
    pub dwFlags: u32,
    pub xDlg: i32,
    pub yDlg: i32,
    pub dwSubEntry: u32,
    pub dwError: u32,
    pub reserved: usize,
    pub reserved2: usize,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for RASDIALDLG {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for RASDIALDLG {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for RASDIALDLG {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for RASDIALDLG {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<RASDIALDLG>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for RASDIALDLG {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for RASDIALDLG {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`*"]
pub const RASDIALEVENT: &str = "RasDialEvent";
#[repr(C, packed(4))]
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct RASDIALEXTENSIONS {
    pub dwSize: u32,
    pub dwfOptions: u32,
    pub hwndParent: super::super::Foundation::HWND,
    pub reserved: usize,
    pub reserved1: usize,
    pub RasEapInfo: RASEAPINFO,
    pub fSkipPppAuth: super::super::Foundation::BOOL,
    pub RasDevSpecificInfo: RASDEVSPECIFICINFO,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for RASDIALEXTENSIONS {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for RASDIALEXTENSIONS {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for RASDIALEXTENSIONS {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for RASDIALEXTENSIONS {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<RASDIALEXTENSIONS>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for RASDIALEXTENSIONS {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for RASDIALEXTENSIONS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`*"]
pub type RASDIALFUNC = ::core::option::Option<unsafe extern "system" fn(param0: u32, param1: RASCONNSTATE, param2: u32)>;
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`*"]
pub type RASDIALFUNC1 = ::core::option::Option<unsafe extern "system" fn(param0: HRASCONN, param1: u32, param2: RASCONNSTATE, param3: u32, param4: u32)>;
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`*"]
pub type RASDIALFUNC2 = ::core::option::Option<unsafe extern "system" fn(param0: usize, param1: u32, param2: HRASCONN, param3: u32, param4: RASCONNSTATE, param5: u32, param6: u32) -> u32>;
#[repr(C, packed(4))]
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct RASDIALPARAMSA {
    pub dwSize: u32,
    pub szEntryName: [super::super::Foundation::CHAR; 257],
    pub szPhoneNumber: [super::super::Foundation::CHAR; 129],
    pub szCallbackNumber: [super::super::Foundation::CHAR; 129],
    pub szUserName: [super::super::Foundation::CHAR; 257],
    pub szPassword: [super::super::Foundation::CHAR; 257],
    pub szDomain: [super::super::Foundation::CHAR; 16],
    pub dwSubEntry: u32,
    pub dwCallbackId: usize,
    pub dwIfIndex: u32,
    pub szEncPassword: ::windows::core::PSTR,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for RASDIALPARAMSA {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for RASDIALPARAMSA {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for RASDIALPARAMSA {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for RASDIALPARAMSA {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<RASDIALPARAMSA>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for RASDIALPARAMSA {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for RASDIALPARAMSA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C, packed(4))]
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`*"]
pub struct RASDIALPARAMSW {
    pub dwSize: u32,
    pub szEntryName: [u16; 257],
    pub szPhoneNumber: [u16; 129],
    pub szCallbackNumber: [u16; 129],
    pub szUserName: [u16; 257],
    pub szPassword: [u16; 257],
    pub szDomain: [u16; 16],
    pub dwSubEntry: u32,
    pub dwCallbackId: usize,
    pub dwIfIndex: u32,
    pub szEncPassword: ::windows::core::PWSTR,
}
impl ::core::marker::Copy for RASDIALPARAMSW {}
impl ::core::clone::Clone for RASDIALPARAMSW {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for RASDIALPARAMSW {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for RASDIALPARAMSW {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<RASDIALPARAMSW>()) == 0 }
    }
}
impl ::core::cmp::Eq for RASDIALPARAMSW {}
impl ::core::default::Default for RASDIALPARAMSW {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`*"]
pub const RASDT_Atm: &str = "ATM";
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`*"]
pub const RASDT_FrameRelay: &str = "FRAMERELAY";
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`*"]
pub const RASDT_Generic: &str = "GENERIC";
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`*"]
pub const RASDT_Irda: &str = "IRDA";
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`*"]
pub const RASDT_Isdn: &str = "isdn";
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`*"]
pub const RASDT_Modem: &str = "modem";
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`*"]
pub const RASDT_PPPoE: &str = "PPPoE";
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`*"]
pub const RASDT_Pad: &str = "pad";
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`*"]
pub const RASDT_Parallel: &str = "PARALLEL";
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`*"]
pub const RASDT_SW56: &str = "SW56";
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`*"]
pub const RASDT_Serial: &str = "SERIAL";
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`*"]
pub const RASDT_Sonet: &str = "SONET";
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`*"]
pub const RASDT_Vpn: &str = "vpn";
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`*"]
pub const RASDT_X25: &str = "x25";
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`*"]
pub const RASEAPF_Logon: u32 = 4u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`*"]
pub const RASEAPF_NonInteractive: u32 = 2u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`*"]
pub const RASEAPF_Preview: u32 = 8u32;
#[repr(C, packed(4))]
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`*"]
pub struct RASEAPINFO {
    pub dwSizeofEapInfo: u32,
    pub pbEapInfo: *mut u8,
}
impl ::core::marker::Copy for RASEAPINFO {}
impl ::core::clone::Clone for RASEAPINFO {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for RASEAPINFO {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for RASEAPINFO {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<RASEAPINFO>()) == 0 }
    }
}
impl ::core::cmp::Eq for RASEAPINFO {}
impl ::core::default::Default for RASEAPINFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct RASEAPUSERIDENTITYA {
    pub szUserName: [super::super::Foundation::CHAR; 257],
    pub dwSizeofEapInfo: u32,
    pub pbEapInfo: [u8; 1],
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for RASEAPUSERIDENTITYA {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for RASEAPUSERIDENTITYA {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for RASEAPUSERIDENTITYA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("RASEAPUSERIDENTITYA").field("szUserName", &self.szUserName).field("dwSizeofEapInfo", &self.dwSizeofEapInfo).field("pbEapInfo", &self.pbEapInfo).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for RASEAPUSERIDENTITYA {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for RASEAPUSERIDENTITYA {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<RASEAPUSERIDENTITYA>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for RASEAPUSERIDENTITYA {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for RASEAPUSERIDENTITYA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`*"]
pub struct RASEAPUSERIDENTITYW {
    pub szUserName: [u16; 257],
    pub dwSizeofEapInfo: u32,
    pub pbEapInfo: [u8; 1],
}
impl ::core::marker::Copy for RASEAPUSERIDENTITYW {}
impl ::core::clone::Clone for RASEAPUSERIDENTITYW {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for RASEAPUSERIDENTITYW {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("RASEAPUSERIDENTITYW").field("szUserName", &self.szUserName).field("dwSizeofEapInfo", &self.dwSizeofEapInfo).field("pbEapInfo", &self.pbEapInfo).finish()
    }
}
unsafe impl ::windows::core::Abi for RASEAPUSERIDENTITYW {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for RASEAPUSERIDENTITYW {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<RASEAPUSERIDENTITYW>()) == 0 }
    }
}
impl ::core::cmp::Eq for RASEAPUSERIDENTITYW {}
impl ::core::default::Default for RASEAPUSERIDENTITYW {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`*"]
pub const RASEDFLAG_CloneEntry: u32 = 4u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`*"]
pub const RASEDFLAG_IncomingConnection: u32 = 1024u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`*"]
pub const RASEDFLAG_InternetEntry: u32 = 256u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`*"]
pub const RASEDFLAG_NAT: u32 = 512u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`*"]
pub const RASEDFLAG_NewBroadbandEntry: u32 = 128u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`*"]
pub const RASEDFLAG_NewDirectEntry: u32 = 64u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`*"]
pub const RASEDFLAG_NewEntry: u32 = 2u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`*"]
pub const RASEDFLAG_NewPhoneEntry: u32 = 16u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`*"]
pub const RASEDFLAG_NewTunnelEntry: u32 = 32u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`*"]
pub const RASEDFLAG_NoRename: u32 = 8u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`*"]
pub const RASEDFLAG_PositionDlg: u32 = 1u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`*"]
pub const RASEDFLAG_ShellOwned: u32 = 1073741824u32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`, `\"Win32_Foundation\"`, `\"Win32_Networking_WinSock\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
pub struct RASENTRYA {
    pub dwSize: u32,
    pub dwfOptions: u32,
    pub dwCountryID: u32,
    pub dwCountryCode: u32,
    pub szAreaCode: [super::super::Foundation::CHAR; 11],
    pub szLocalPhoneNumber: [super::super::Foundation::CHAR; 129],
    pub dwAlternateOffset: u32,
    pub ipaddr: RASIPADDR,
    pub ipaddrDns: RASIPADDR,
    pub ipaddrDnsAlt: RASIPADDR,
    pub ipaddrWins: RASIPADDR,
    pub ipaddrWinsAlt: RASIPADDR,
    pub dwFrameSize: u32,
    pub dwfNetProtocols: u32,
    pub dwFramingProtocol: u32,
    pub szScript: [super::super::Foundation::CHAR; 260],
    pub szAutodialDll: [super::super::Foundation::CHAR; 260],
    pub szAutodialFunc: [super::super::Foundation::CHAR; 260],
    pub szDeviceType: [super::super::Foundation::CHAR; 17],
    pub szDeviceName: [super::super::Foundation::CHAR; 129],
    pub szX25PadType: [super::super::Foundation::CHAR; 33],
    pub szX25Address: [super::super::Foundation::CHAR; 201],
    pub szX25Facilities: [super::super::Foundation::CHAR; 201],
    pub szX25UserData: [super::super::Foundation::CHAR; 201],
    pub dwChannels: u32,
    pub dwReserved1: u32,
    pub dwReserved2: u32,
    pub dwSubEntries: u32,
    pub dwDialMode: RASENTRY_DIAL_MODE,
    pub dwDialExtraPercent: u32,
    pub dwDialExtraSampleSeconds: u32,
    pub dwHangUpExtraPercent: u32,
    pub dwHangUpExtraSampleSeconds: u32,
    pub dwIdleDisconnectSeconds: u32,
    pub dwType: u32,
    pub dwEncryptionType: u32,
    pub dwCustomAuthKey: u32,
    pub guidId: ::windows::core::GUID,
    pub szCustomDialDll: [super::super::Foundation::CHAR; 260],
    pub dwVpnStrategy: u32,
    pub dwfOptions2: u32,
    pub dwfOptions3: u32,
    pub szDnsSuffix: [super::super::Foundation::CHAR; 256],
    pub dwTcpWindowSize: u32,
    pub szPrerequisitePbk: [super::super::Foundation::CHAR; 260],
    pub szPrerequisiteEntry: [super::super::Foundation::CHAR; 257],
    pub dwRedialCount: u32,
    pub dwRedialPause: u32,
    pub ipv6addrDns: super::super::Networking::WinSock::IN6_ADDR,
    pub ipv6addrDnsAlt: super::super::Networking::WinSock::IN6_ADDR,
    pub dwIPv4InterfaceMetric: u32,
    pub dwIPv6InterfaceMetric: u32,
    pub ipv6addr: super::super::Networking::WinSock::IN6_ADDR,
    pub dwIPv6PrefixLength: u32,
    pub dwNetworkOutageTime: u32,
    pub szIDi: [super::super::Foundation::CHAR; 257],
    pub szIDr: [super::super::Foundation::CHAR; 257],
    pub fIsImsConfig: super::super::Foundation::BOOL,
    pub IdiType: IKEV2_ID_PAYLOAD_TYPE,
    pub IdrType: IKEV2_ID_PAYLOAD_TYPE,
    pub fDisableIKEv2Fragmentation: super::super::Foundation::BOOL,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
impl ::core::marker::Copy for RASENTRYA {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
impl ::core::clone::Clone for RASENTRYA {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
unsafe impl ::windows::core::Abi for RASENTRYA {
    type Abi = Self;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
impl ::core::cmp::PartialEq for RASENTRYA {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<RASENTRYA>()) == 0 }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
impl ::core::cmp::Eq for RASENTRYA {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
impl ::core::default::Default for RASENTRYA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C, packed(4))]
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct RASENTRYDLGA {
    pub dwSize: u32,
    pub hwndOwner: super::super::Foundation::HWND,
    pub dwFlags: u32,
    pub xDlg: i32,
    pub yDlg: i32,
    pub szEntry: [super::super::Foundation::CHAR; 257],
    pub dwError: u32,
    pub reserved: usize,
    pub reserved2: usize,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for RASENTRYDLGA {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for RASENTRYDLGA {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for RASENTRYDLGA {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for RASENTRYDLGA {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<RASENTRYDLGA>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for RASENTRYDLGA {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for RASENTRYDLGA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C, packed(4))]
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct RASENTRYDLGW {
    pub dwSize: u32,
    pub hwndOwner: super::super::Foundation::HWND,
    pub dwFlags: u32,
    pub xDlg: i32,
    pub yDlg: i32,
    pub szEntry: [u16; 257],
    pub dwError: u32,
    pub reserved: usize,
    pub reserved2: usize,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for RASENTRYDLGW {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for RASENTRYDLGW {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for RASENTRYDLGW {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for RASENTRYDLGW {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<RASENTRYDLGW>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for RASENTRYDLGW {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for RASENTRYDLGW {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct RASENTRYNAMEA {
    pub dwSize: u32,
    pub szEntryName: [super::super::Foundation::CHAR; 257],
    pub dwFlags: u32,
    pub szPhonebookPath: [super::super::Foundation::CHAR; 261],
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for RASENTRYNAMEA {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for RASENTRYNAMEA {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for RASENTRYNAMEA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("RASENTRYNAMEA").field("dwSize", &self.dwSize).field("szEntryName", &self.szEntryName).field("dwFlags", &self.dwFlags).field("szPhonebookPath", &self.szPhonebookPath).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for RASENTRYNAMEA {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for RASENTRYNAMEA {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<RASENTRYNAMEA>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for RASENTRYNAMEA {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for RASENTRYNAMEA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`*"]
pub struct RASENTRYNAMEW {
    pub dwSize: u32,
    pub szEntryName: [u16; 257],
    pub dwFlags: u32,
    pub szPhonebookPath: [u16; 261],
}
impl ::core::marker::Copy for RASENTRYNAMEW {}
impl ::core::clone::Clone for RASENTRYNAMEW {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for RASENTRYNAMEW {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("RASENTRYNAMEW").field("dwSize", &self.dwSize).field("szEntryName", &self.szEntryName).field("dwFlags", &self.dwFlags).field("szPhonebookPath", &self.szPhonebookPath).finish()
    }
}
unsafe impl ::windows::core::Abi for RASENTRYNAMEW {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for RASENTRYNAMEW {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<RASENTRYNAMEW>()) == 0 }
    }
}
impl ::core::cmp::Eq for RASENTRYNAMEW {}
impl ::core::default::Default for RASENTRYNAMEW {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`, `\"Win32_Foundation\"`, `\"Win32_Networking_WinSock\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
pub struct RASENTRYW {
    pub dwSize: u32,
    pub dwfOptions: u32,
    pub dwCountryID: u32,
    pub dwCountryCode: u32,
    pub szAreaCode: [u16; 11],
    pub szLocalPhoneNumber: [u16; 129],
    pub dwAlternateOffset: u32,
    pub ipaddr: RASIPADDR,
    pub ipaddrDns: RASIPADDR,
    pub ipaddrDnsAlt: RASIPADDR,
    pub ipaddrWins: RASIPADDR,
    pub ipaddrWinsAlt: RASIPADDR,
    pub dwFrameSize: u32,
    pub dwfNetProtocols: u32,
    pub dwFramingProtocol: u32,
    pub szScript: [u16; 260],
    pub szAutodialDll: [u16; 260],
    pub szAutodialFunc: [u16; 260],
    pub szDeviceType: [u16; 17],
    pub szDeviceName: [u16; 129],
    pub szX25PadType: [u16; 33],
    pub szX25Address: [u16; 201],
    pub szX25Facilities: [u16; 201],
    pub szX25UserData: [u16; 201],
    pub dwChannels: u32,
    pub dwReserved1: u32,
    pub dwReserved2: u32,
    pub dwSubEntries: u32,
    pub dwDialMode: RASENTRY_DIAL_MODE,
    pub dwDialExtraPercent: u32,
    pub dwDialExtraSampleSeconds: u32,
    pub dwHangUpExtraPercent: u32,
    pub dwHangUpExtraSampleSeconds: u32,
    pub dwIdleDisconnectSeconds: u32,
    pub dwType: u32,
    pub dwEncryptionType: u32,
    pub dwCustomAuthKey: u32,
    pub guidId: ::windows::core::GUID,
    pub szCustomDialDll: [u16; 260],
    pub dwVpnStrategy: u32,
    pub dwfOptions2: u32,
    pub dwfOptions3: u32,
    pub szDnsSuffix: [u16; 256],
    pub dwTcpWindowSize: u32,
    pub szPrerequisitePbk: [u16; 260],
    pub szPrerequisiteEntry: [u16; 257],
    pub dwRedialCount: u32,
    pub dwRedialPause: u32,
    pub ipv6addrDns: super::super::Networking::WinSock::IN6_ADDR,
    pub ipv6addrDnsAlt: super::super::Networking::WinSock::IN6_ADDR,
    pub dwIPv4InterfaceMetric: u32,
    pub dwIPv6InterfaceMetric: u32,
    pub ipv6addr: super::super::Networking::WinSock::IN6_ADDR,
    pub dwIPv6PrefixLength: u32,
    pub dwNetworkOutageTime: u32,
    pub szIDi: [u16; 257],
    pub szIDr: [u16; 257],
    pub fIsImsConfig: super::super::Foundation::BOOL,
    pub IdiType: IKEV2_ID_PAYLOAD_TYPE,
    pub IdrType: IKEV2_ID_PAYLOAD_TYPE,
    pub fDisableIKEv2Fragmentation: super::super::Foundation::BOOL,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
impl ::core::marker::Copy for RASENTRYW {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
impl ::core::clone::Clone for RASENTRYW {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
unsafe impl ::windows::core::Abi for RASENTRYW {
    type Abi = Self;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
impl ::core::cmp::PartialEq for RASENTRYW {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<RASENTRYW>()) == 0 }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
impl ::core::cmp::Eq for RASENTRYW {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
impl ::core::default::Default for RASENTRYW {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct RASENTRY_DIAL_MODE(pub u32);
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`*"]
pub const RASEDM_DialAll: RASENTRY_DIAL_MODE = RASENTRY_DIAL_MODE(1u32);
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`*"]
pub const RASEDM_DialAsNeeded: RASENTRY_DIAL_MODE = RASENTRY_DIAL_MODE(2u32);
impl ::core::marker::Copy for RASENTRY_DIAL_MODE {}
impl ::core::clone::Clone for RASENTRY_DIAL_MODE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for RASENTRY_DIAL_MODE {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for RASENTRY_DIAL_MODE {
    type Abi = Self;
}
impl ::core::fmt::Debug for RASENTRY_DIAL_MODE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("RASENTRY_DIAL_MODE").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`*"]
pub const RASEO2_AuthTypeIsOtp: u32 = 268435456u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`*"]
pub const RASEO2_AutoTriggerCapable: u32 = 67108864u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`*"]
pub const RASEO2_CacheCredentials: u32 = 33554432u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`*"]
pub const RASEO2_DisableClassBasedStaticRoute: u32 = 524288u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`*"]
pub const RASEO2_DisableIKENameEkuCheck: u32 = 262144u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`*"]
pub const RASEO2_DisableMobility: u32 = 2097152u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`*"]
pub const RASEO2_DisableNbtOverIP: u32 = 64u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`*"]
pub const RASEO2_DontNegotiateMultilink: u32 = 4u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`*"]
pub const RASEO2_DontUseRasCredentials: u32 = 8u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`*"]
pub const RASEO2_IPv4ExplicitMetric: u32 = 65536u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`*"]
pub const RASEO2_IPv6ExplicitMetric: u32 = 131072u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`*"]
pub const RASEO2_IPv6RemoteDefaultGateway: u32 = 8192u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`*"]
pub const RASEO2_IPv6SpecificNameServers: u32 = 4096u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`*"]
pub const RASEO2_Internet: u32 = 32u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`*"]
pub const RASEO2_IsAlwaysOn: u32 = 536870912u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`*"]
pub const RASEO2_IsPrivateNetwork: u32 = 1073741824u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`*"]
pub const RASEO2_IsThirdPartyProfile: u32 = 134217728u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`*"]
pub const RASEO2_PlumbIKEv2TSAsRoutes: u32 = 2147483648u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`*"]
pub const RASEO2_ReconnectIfDropped: u32 = 256u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`*"]
pub const RASEO2_RegisterIpWithDNS: u32 = 16384u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`*"]
pub const RASEO2_RequireMachineCertificates: u32 = 4194304u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`*"]
pub const RASEO2_SecureClientForMSNet: u32 = 2u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`*"]
pub const RASEO2_SecureFileAndPrint: u32 = 1u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`*"]
pub const RASEO2_SecureRoutingCompartment: u32 = 1024u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`*"]
pub const RASEO2_SharePhoneNumbers: u32 = 512u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`*"]
pub const RASEO2_SpecificIPv6Addr: u32 = 1048576u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`*"]
pub const RASEO2_UseDNSSuffixForRegistration: u32 = 32768u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`*"]
pub const RASEO2_UseGlobalDeviceSettings: u32 = 128u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`*"]
pub const RASEO2_UsePreSharedKey: u32 = 16u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`*"]
pub const RASEO2_UsePreSharedKeyForIkev2Initiator: u32 = 8388608u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`*"]
pub const RASEO2_UsePreSharedKeyForIkev2Responder: u32 = 16777216u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`*"]
pub const RASEO2_UseTypicalSettings: u32 = 2048u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`*"]
pub const RASEO_Custom: u32 = 1048576u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`*"]
pub const RASEO_CustomScript: u32 = 2147483648u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`*"]
pub const RASEO_DisableLcpExtensions: u32 = 32u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`*"]
pub const RASEO_IpHeaderCompression: u32 = 8u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`*"]
pub const RASEO_ModemLights: u32 = 256u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`*"]
pub const RASEO_NetworkLogon: u32 = 8192u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`*"]
pub const RASEO_PreviewDomain: u32 = 33554432u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`*"]
pub const RASEO_PreviewPhoneNumber: u32 = 2097152u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`*"]
pub const RASEO_PreviewUserPw: u32 = 16777216u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`*"]
pub const RASEO_PromoteAlternates: u32 = 32768u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`*"]
pub const RASEO_RemoteDefaultGateway: u32 = 16u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`*"]
pub const RASEO_RequireCHAP: u32 = 134217728u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`*"]
pub const RASEO_RequireDataEncryption: u32 = 4096u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`*"]
pub const RASEO_RequireEAP: u32 = 131072u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`*"]
pub const RASEO_RequireEncryptedPw: u32 = 1024u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`*"]
pub const RASEO_RequireMsCHAP: u32 = 268435456u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`*"]
pub const RASEO_RequireMsCHAP2: u32 = 536870912u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`*"]
pub const RASEO_RequireMsEncryptedPw: u32 = 2048u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`*"]
pub const RASEO_RequirePAP: u32 = 262144u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`*"]
pub const RASEO_RequireSPAP: u32 = 524288u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`*"]
pub const RASEO_RequireW95MSCHAP: u32 = 1073741824u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`*"]
pub const RASEO_SecureLocalFiles: u32 = 65536u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`*"]
pub const RASEO_SharedPhoneNumbers: u32 = 8388608u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`*"]
pub const RASEO_ShowDialingProgress: u32 = 67108864u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`*"]
pub const RASEO_SpecificIpAddr: u32 = 2u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`*"]
pub const RASEO_SpecificNameServers: u32 = 4u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`*"]
pub const RASEO_SwCompression: u32 = 512u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`*"]
pub const RASEO_TerminalAfterDial: u32 = 128u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`*"]
pub const RASEO_TerminalBeforeDial: u32 = 64u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`*"]
pub const RASEO_UseCountryAndAreaCodes: u32 = 1u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`*"]
pub const RASEO_UseLogonCredentials: u32 = 16384u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`*"]
pub const RASET_Broadband: u32 = 5u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`*"]
pub const RASET_Direct: u32 = 3u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`*"]
pub const RASET_Internet: u32 = 4u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`*"]
pub const RASET_Phone: u32 = 1u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`*"]
pub const RASET_Vpn: u32 = 2u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`*"]
pub const RASFP_Ppp: u32 = 1u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`*"]
pub const RASFP_Ras: u32 = 4u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`*"]
pub const RASFP_Slip: u32 = 2u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`*"]
pub const RASIDS_Disabled: u32 = 4294967295u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`*"]
pub const RASIDS_UseGlobalValue: u32 = 0u32;
#[repr(C, packed(4))]
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`, `\"Win32_Networking_WinSock\"`*"]
#[cfg(feature = "Win32_Networking_WinSock")]
pub struct RASIKEV2_PROJECTION_INFO {
    pub dwIPv4NegotiationError: u32,
    pub ipv4Address: super::super::Networking::WinSock::IN_ADDR,
    pub ipv4ServerAddress: super::super::Networking::WinSock::IN_ADDR,
    pub dwIPv6NegotiationError: u32,
    pub ipv6Address: super::super::Networking::WinSock::IN6_ADDR,
    pub ipv6ServerAddress: super::super::Networking::WinSock::IN6_ADDR,
    pub dwPrefixLength: u32,
    pub dwAuthenticationProtocol: u32,
    pub dwEapTypeId: u32,
    pub dwFlags: RASIKEV_PROJECTION_INFO_FLAGS,
    pub dwEncryptionMethod: u32,
    pub numIPv4ServerAddresses: u32,
    pub ipv4ServerAddresses: *mut super::super::Networking::WinSock::IN_ADDR,
    pub numIPv6ServerAddresses: u32,
    pub ipv6ServerAddresses: *mut super::super::Networking::WinSock::IN6_ADDR,
}
#[cfg(feature = "Win32_Networking_WinSock")]
impl ::core::marker::Copy for RASIKEV2_PROJECTION_INFO {}
#[cfg(feature = "Win32_Networking_WinSock")]
impl ::core::clone::Clone for RASIKEV2_PROJECTION_INFO {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Networking_WinSock")]
unsafe impl ::windows::core::Abi for RASIKEV2_PROJECTION_INFO {
    type Abi = Self;
}
#[cfg(feature = "Win32_Networking_WinSock")]
impl ::core::cmp::PartialEq for RASIKEV2_PROJECTION_INFO {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<RASIKEV2_PROJECTION_INFO>()) == 0 }
    }
}
#[cfg(feature = "Win32_Networking_WinSock")]
impl ::core::cmp::Eq for RASIKEV2_PROJECTION_INFO {}
#[cfg(feature = "Win32_Networking_WinSock")]
impl ::core::default::Default for RASIKEV2_PROJECTION_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct RASIKEV_PROJECTION_INFO_FLAGS(pub u32);
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`*"]
pub const RASIKEv2_FLAGS_MOBIKESUPPORTED: RASIKEV_PROJECTION_INFO_FLAGS = RASIKEV_PROJECTION_INFO_FLAGS(1u32);
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`*"]
pub const RASIKEv2_FLAGS_BEHIND_NAT: RASIKEV_PROJECTION_INFO_FLAGS = RASIKEV_PROJECTION_INFO_FLAGS(2u32);
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`*"]
pub const RASIKEv2_FLAGS_SERVERBEHIND_NAT: RASIKEV_PROJECTION_INFO_FLAGS = RASIKEV_PROJECTION_INFO_FLAGS(4u32);
impl ::core::marker::Copy for RASIKEV_PROJECTION_INFO_FLAGS {}
impl ::core::clone::Clone for RASIKEV_PROJECTION_INFO_FLAGS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for RASIKEV_PROJECTION_INFO_FLAGS {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for RASIKEV_PROJECTION_INFO_FLAGS {
    type Abi = Self;
}
impl ::core::fmt::Debug for RASIKEV_PROJECTION_INFO_FLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("RASIKEV_PROJECTION_INFO_FLAGS").field(&self.0).finish()
    }
}
impl ::core::ops::BitOr for RASIKEV_PROJECTION_INFO_FLAGS {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for RASIKEV_PROJECTION_INFO_FLAGS {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for RASIKEV_PROJECTION_INFO_FLAGS {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for RASIKEV_PROJECTION_INFO_FLAGS {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for RASIKEV_PROJECTION_INFO_FLAGS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`*"]
pub const RASIKEv2_AUTH_EAP: u32 = 2u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`*"]
pub const RASIKEv2_AUTH_MACHINECERTIFICATES: u32 = 1u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`*"]
pub const RASIKEv2_AUTH_PSK: u32 = 3u32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`*"]
pub struct RASIPADDR {
    pub a: u8,
    pub b: u8,
    pub c: u8,
    pub d: u8,
}
impl ::core::marker::Copy for RASIPADDR {}
impl ::core::clone::Clone for RASIPADDR {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for RASIPADDR {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("RASIPADDR").field("a", &self.a).field("b", &self.b).field("c", &self.c).field("d", &self.d).finish()
    }
}
unsafe impl ::windows::core::Abi for RASIPADDR {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for RASIPADDR {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<RASIPADDR>()) == 0 }
    }
}
impl ::core::cmp::Eq for RASIPADDR {}
impl ::core::default::Default for RASIPADDR {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`*"]
pub const RASIPO_VJ: u32 = 1u32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`*"]
pub struct RASIPXW {
    pub dwSize: u32,
    pub dwError: u32,
    pub szIpxAddress: [u16; 22],
}
impl ::core::marker::Copy for RASIPXW {}
impl ::core::clone::Clone for RASIPXW {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for RASIPXW {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("RASIPXW").field("dwSize", &self.dwSize).field("dwError", &self.dwError).field("szIpxAddress", &self.szIpxAddress).finish()
    }
}
unsafe impl ::windows::core::Abi for RASIPXW {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for RASIPXW {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<RASIPXW>()) == 0 }
    }
}
impl ::core::cmp::Eq for RASIPXW {}
impl ::core::default::Default for RASIPXW {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`*"]
pub const RASLCPO_3_DES: u32 = 16u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`*"]
pub const RASLCPO_ACFC: u32 = 2u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`*"]
pub const RASLCPO_AES_128: u32 = 32u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`*"]
pub const RASLCPO_AES_192: u32 = 128u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`*"]
pub const RASLCPO_AES_256: u32 = 64u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`*"]
pub const RASLCPO_DES_56: u32 = 8u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`*"]
pub const RASLCPO_GCM_AES_128: u32 = 256u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`*"]
pub const RASLCPO_GCM_AES_192: u32 = 512u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`*"]
pub const RASLCPO_GCM_AES_256: u32 = 1024u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`*"]
pub const RASLCPO_PFC: u32 = 1u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`*"]
pub const RASLCPO_SSHF: u32 = 4u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`*"]
pub const RASNAP_ProbationTime: u32 = 1u32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct RASNOUSERA {
    pub dwSize: u32,
    pub dwFlags: u32,
    pub dwTimeoutMs: u32,
    pub szUserName: [super::super::Foundation::CHAR; 257],
    pub szPassword: [super::super::Foundation::CHAR; 257],
    pub szDomain: [super::super::Foundation::CHAR; 16],
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for RASNOUSERA {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for RASNOUSERA {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for RASNOUSERA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("RASNOUSERA").field("dwSize", &self.dwSize).field("dwFlags", &self.dwFlags).field("dwTimeoutMs", &self.dwTimeoutMs).field("szUserName", &self.szUserName).field("szPassword", &self.szPassword).field("szDomain", &self.szDomain).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for RASNOUSERA {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for RASNOUSERA {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<RASNOUSERA>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for RASNOUSERA {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for RASNOUSERA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`*"]
pub struct RASNOUSERW {
    pub dwSize: u32,
    pub dwFlags: u32,
    pub dwTimeoutMs: u32,
    pub szUserName: [u16; 257],
    pub szPassword: [u16; 257],
    pub szDomain: [u16; 16],
}
impl ::core::marker::Copy for RASNOUSERW {}
impl ::core::clone::Clone for RASNOUSERW {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for RASNOUSERW {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("RASNOUSERW").field("dwSize", &self.dwSize).field("dwFlags", &self.dwFlags).field("dwTimeoutMs", &self.dwTimeoutMs).field("szUserName", &self.szUserName).field("szPassword", &self.szPassword).field("szDomain", &self.szDomain).finish()
    }
}
unsafe impl ::windows::core::Abi for RASNOUSERW {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for RASNOUSERW {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<RASNOUSERW>()) == 0 }
    }
}
impl ::core::cmp::Eq for RASNOUSERW {}
impl ::core::default::Default for RASNOUSERW {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`*"]
pub const RASNOUSER_SmartCard: u32 = 1u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`*"]
pub const RASNP_Ip: u32 = 4u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`*"]
pub const RASNP_Ipv6: u32 = 8u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`*"]
pub const RASNP_Ipx: u32 = 2u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`*"]
pub const RASNP_NetBEUI: u32 = 1u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`*"]
pub const RASPBDEVENT_AddEntry: u32 = 1u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`*"]
pub const RASPBDEVENT_DialEntry: u32 = 4u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`*"]
pub const RASPBDEVENT_EditEntry: u32 = 2u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`*"]
pub const RASPBDEVENT_EditGlobals: u32 = 5u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`*"]
pub const RASPBDEVENT_NoUser: u32 = 6u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`*"]
pub const RASPBDEVENT_NoUserEdit: u32 = 7u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`*"]
pub const RASPBDEVENT_RemoveEntry: u32 = 3u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`*"]
pub const RASPBDFLAG_ForceCloseOnDial: u32 = 2u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`*"]
pub const RASPBDFLAG_NoUser: u32 = 16u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`*"]
pub const RASPBDFLAG_PositionDlg: u32 = 1u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`*"]
pub const RASPBDFLAG_UpdateDefaults: u32 = 2147483648u32;
#[repr(C, packed(4))]
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct RASPBDLGA {
    pub dwSize: u32,
    pub hwndOwner: super::super::Foundation::HWND,
    pub dwFlags: u32,
    pub xDlg: i32,
    pub yDlg: i32,
    pub dwCallbackId: usize,
    pub pCallback: RASPBDLGFUNCA,
    pub dwError: u32,
    pub reserved: usize,
    pub reserved2: usize,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for RASPBDLGA {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for RASPBDLGA {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for RASPBDLGA {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for RASPBDLGA {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<RASPBDLGA>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for RASPBDLGA {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for RASPBDLGA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`*"]
pub type RASPBDLGFUNCA = ::core::option::Option<unsafe extern "system" fn(param0: usize, param1: u32, param2: ::windows::core::PCSTR, param3: *mut ::core::ffi::c_void)>;
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`*"]
pub type RASPBDLGFUNCW = ::core::option::Option<unsafe extern "system" fn(param0: usize, param1: u32, param2: ::windows::core::PCWSTR, param3: *mut ::core::ffi::c_void)>;
#[repr(C, packed(4))]
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct RASPBDLGW {
    pub dwSize: u32,
    pub hwndOwner: super::super::Foundation::HWND,
    pub dwFlags: u32,
    pub xDlg: i32,
    pub yDlg: i32,
    pub dwCallbackId: usize,
    pub pCallback: RASPBDLGFUNCW,
    pub dwError: u32,
    pub reserved: usize,
    pub reserved2: usize,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for RASPBDLGW {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for RASPBDLGW {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for RASPBDLGW {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for RASPBDLGW {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<RASPBDLGW>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for RASPBDLGW {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for RASPBDLGW {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`*"]
pub struct RASPPPCCP {
    pub dwSize: u32,
    pub dwError: u32,
    pub dwCompressionAlgorithm: u32,
    pub dwOptions: u32,
    pub dwServerCompressionAlgorithm: u32,
    pub dwServerOptions: u32,
}
impl ::core::marker::Copy for RASPPPCCP {}
impl ::core::clone::Clone for RASPPPCCP {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for RASPPPCCP {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("RASPPPCCP").field("dwSize", &self.dwSize).field("dwError", &self.dwError).field("dwCompressionAlgorithm", &self.dwCompressionAlgorithm).field("dwOptions", &self.dwOptions).field("dwServerCompressionAlgorithm", &self.dwServerCompressionAlgorithm).field("dwServerOptions", &self.dwServerOptions).finish()
    }
}
unsafe impl ::windows::core::Abi for RASPPPCCP {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for RASPPPCCP {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<RASPPPCCP>()) == 0 }
    }
}
impl ::core::cmp::Eq for RASPPPCCP {}
impl ::core::default::Default for RASPPPCCP {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct RASPPPIPA {
    pub dwSize: u32,
    pub dwError: u32,
    pub szIpAddress: [super::super::Foundation::CHAR; 16],
    pub szServerIpAddress: [super::super::Foundation::CHAR; 16],
    pub dwOptions: u32,
    pub dwServerOptions: u32,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for RASPPPIPA {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for RASPPPIPA {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for RASPPPIPA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("RASPPPIPA").field("dwSize", &self.dwSize).field("dwError", &self.dwError).field("szIpAddress", &self.szIpAddress).field("szServerIpAddress", &self.szServerIpAddress).field("dwOptions", &self.dwOptions).field("dwServerOptions", &self.dwServerOptions).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for RASPPPIPA {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for RASPPPIPA {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<RASPPPIPA>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for RASPPPIPA {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for RASPPPIPA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`*"]
pub struct RASPPPIPV6 {
    pub dwSize: u32,
    pub dwError: u32,
    pub bLocalInterfaceIdentifier: [u8; 8],
    pub bPeerInterfaceIdentifier: [u8; 8],
    pub bLocalCompressionProtocol: [u8; 2],
    pub bPeerCompressionProtocol: [u8; 2],
}
impl ::core::marker::Copy for RASPPPIPV6 {}
impl ::core::clone::Clone for RASPPPIPV6 {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for RASPPPIPV6 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("RASPPPIPV6").field("dwSize", &self.dwSize).field("dwError", &self.dwError).field("bLocalInterfaceIdentifier", &self.bLocalInterfaceIdentifier).field("bPeerInterfaceIdentifier", &self.bPeerInterfaceIdentifier).field("bLocalCompressionProtocol", &self.bLocalCompressionProtocol).field("bPeerCompressionProtocol", &self.bPeerCompressionProtocol).finish()
    }
}
unsafe impl ::windows::core::Abi for RASPPPIPV6 {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for RASPPPIPV6 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<RASPPPIPV6>()) == 0 }
    }
}
impl ::core::cmp::Eq for RASPPPIPV6 {}
impl ::core::default::Default for RASPPPIPV6 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`*"]
pub struct RASPPPIPW {
    pub dwSize: u32,
    pub dwError: u32,
    pub szIpAddress: [u16; 16],
    pub szServerIpAddress: [u16; 16],
    pub dwOptions: u32,
    pub dwServerOptions: u32,
}
impl ::core::marker::Copy for RASPPPIPW {}
impl ::core::clone::Clone for RASPPPIPW {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for RASPPPIPW {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("RASPPPIPW").field("dwSize", &self.dwSize).field("dwError", &self.dwError).field("szIpAddress", &self.szIpAddress).field("szServerIpAddress", &self.szServerIpAddress).field("dwOptions", &self.dwOptions).field("dwServerOptions", &self.dwServerOptions).finish()
    }
}
unsafe impl ::windows::core::Abi for RASPPPIPW {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for RASPPPIPW {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<RASPPPIPW>()) == 0 }
    }
}
impl ::core::cmp::Eq for RASPPPIPW {}
impl ::core::default::Default for RASPPPIPW {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct RASPPPIPXA {
    pub dwSize: u32,
    pub dwError: u32,
    pub szIpxAddress: [super::super::Foundation::CHAR; 22],
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for RASPPPIPXA {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for RASPPPIPXA {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for RASPPPIPXA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("RASPPPIPXA").field("dwSize", &self.dwSize).field("dwError", &self.dwError).field("szIpxAddress", &self.szIpxAddress).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for RASPPPIPXA {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for RASPPPIPXA {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<RASPPPIPXA>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for RASPPPIPXA {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for RASPPPIPXA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct RASPPPLCPA {
    pub dwSize: u32,
    pub fBundled: super::super::Foundation::BOOL,
    pub dwError: u32,
    pub dwAuthenticationProtocol: u32,
    pub dwAuthenticationData: u32,
    pub dwEapTypeId: u32,
    pub dwServerAuthenticationProtocol: u32,
    pub dwServerAuthenticationData: u32,
    pub dwServerEapTypeId: u32,
    pub fMultilink: super::super::Foundation::BOOL,
    pub dwTerminateReason: u32,
    pub dwServerTerminateReason: u32,
    pub szReplyMessage: [super::super::Foundation::CHAR; 1024],
    pub dwOptions: u32,
    pub dwServerOptions: u32,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for RASPPPLCPA {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for RASPPPLCPA {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for RASPPPLCPA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("RASPPPLCPA")
            .field("dwSize", &self.dwSize)
            .field("fBundled", &self.fBundled)
            .field("dwError", &self.dwError)
            .field("dwAuthenticationProtocol", &self.dwAuthenticationProtocol)
            .field("dwAuthenticationData", &self.dwAuthenticationData)
            .field("dwEapTypeId", &self.dwEapTypeId)
            .field("dwServerAuthenticationProtocol", &self.dwServerAuthenticationProtocol)
            .field("dwServerAuthenticationData", &self.dwServerAuthenticationData)
            .field("dwServerEapTypeId", &self.dwServerEapTypeId)
            .field("fMultilink", &self.fMultilink)
            .field("dwTerminateReason", &self.dwTerminateReason)
            .field("dwServerTerminateReason", &self.dwServerTerminateReason)
            .field("szReplyMessage", &self.szReplyMessage)
            .field("dwOptions", &self.dwOptions)
            .field("dwServerOptions", &self.dwServerOptions)
            .finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for RASPPPLCPA {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for RASPPPLCPA {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<RASPPPLCPA>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for RASPPPLCPA {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for RASPPPLCPA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct RASPPPLCPW {
    pub dwSize: u32,
    pub fBundled: super::super::Foundation::BOOL,
    pub dwError: u32,
    pub dwAuthenticationProtocol: u32,
    pub dwAuthenticationData: u32,
    pub dwEapTypeId: u32,
    pub dwServerAuthenticationProtocol: u32,
    pub dwServerAuthenticationData: u32,
    pub dwServerEapTypeId: u32,
    pub fMultilink: super::super::Foundation::BOOL,
    pub dwTerminateReason: u32,
    pub dwServerTerminateReason: u32,
    pub szReplyMessage: [u16; 1024],
    pub dwOptions: u32,
    pub dwServerOptions: u32,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for RASPPPLCPW {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for RASPPPLCPW {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for RASPPPLCPW {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("RASPPPLCPW")
            .field("dwSize", &self.dwSize)
            .field("fBundled", &self.fBundled)
            .field("dwError", &self.dwError)
            .field("dwAuthenticationProtocol", &self.dwAuthenticationProtocol)
            .field("dwAuthenticationData", &self.dwAuthenticationData)
            .field("dwEapTypeId", &self.dwEapTypeId)
            .field("dwServerAuthenticationProtocol", &self.dwServerAuthenticationProtocol)
            .field("dwServerAuthenticationData", &self.dwServerAuthenticationData)
            .field("dwServerEapTypeId", &self.dwServerEapTypeId)
            .field("fMultilink", &self.fMultilink)
            .field("dwTerminateReason", &self.dwTerminateReason)
            .field("dwServerTerminateReason", &self.dwServerTerminateReason)
            .field("szReplyMessage", &self.szReplyMessage)
            .field("dwOptions", &self.dwOptions)
            .field("dwServerOptions", &self.dwServerOptions)
            .finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for RASPPPLCPW {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for RASPPPLCPW {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<RASPPPLCPW>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for RASPPPLCPW {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for RASPPPLCPW {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct RASPPPNBFA {
    pub dwSize: u32,
    pub dwError: u32,
    pub dwNetBiosError: u32,
    pub szNetBiosError: [super::super::Foundation::CHAR; 17],
    pub szWorkstationName: [super::super::Foundation::CHAR; 17],
    pub bLana: u8,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for RASPPPNBFA {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for RASPPPNBFA {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for RASPPPNBFA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("RASPPPNBFA").field("dwSize", &self.dwSize).field("dwError", &self.dwError).field("dwNetBiosError", &self.dwNetBiosError).field("szNetBiosError", &self.szNetBiosError).field("szWorkstationName", &self.szWorkstationName).field("bLana", &self.bLana).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for RASPPPNBFA {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for RASPPPNBFA {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<RASPPPNBFA>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for RASPPPNBFA {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for RASPPPNBFA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`*"]
pub struct RASPPPNBFW {
    pub dwSize: u32,
    pub dwError: u32,
    pub dwNetBiosError: u32,
    pub szNetBiosError: [u16; 17],
    pub szWorkstationName: [u16; 17],
    pub bLana: u8,
}
impl ::core::marker::Copy for RASPPPNBFW {}
impl ::core::clone::Clone for RASPPPNBFW {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for RASPPPNBFW {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("RASPPPNBFW").field("dwSize", &self.dwSize).field("dwError", &self.dwError).field("dwNetBiosError", &self.dwNetBiosError).field("szNetBiosError", &self.szNetBiosError).field("szWorkstationName", &self.szWorkstationName).field("bLana", &self.bLana).finish()
    }
}
unsafe impl ::windows::core::Abi for RASPPPNBFW {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for RASPPPNBFW {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<RASPPPNBFW>()) == 0 }
    }
}
impl ::core::cmp::Eq for RASPPPNBFW {}
impl ::core::default::Default for RASPPPNBFW {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`, `\"Win32_Foundation\"`, `\"Win32_Networking_WinSock\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
pub struct RASPPP_PROJECTION_INFO {
    pub dwIPv4NegotiationError: u32,
    pub ipv4Address: super::super::Networking::WinSock::IN_ADDR,
    pub ipv4ServerAddress: super::super::Networking::WinSock::IN_ADDR,
    pub dwIPv4Options: u32,
    pub dwIPv4ServerOptions: u32,
    pub dwIPv6NegotiationError: u32,
    pub bInterfaceIdentifier: [u8; 8],
    pub bServerInterfaceIdentifier: [u8; 8],
    pub fBundled: super::super::Foundation::BOOL,
    pub fMultilink: super::super::Foundation::BOOL,
    pub dwAuthenticationProtocol: RASPPP_PROJECTION_INFO_SERVER_AUTH_PROTOCOL,
    pub dwAuthenticationData: RASPPP_PROJECTION_INFO_SERVER_AUTH_DATA,
    pub dwServerAuthenticationProtocol: RASPPP_PROJECTION_INFO_SERVER_AUTH_PROTOCOL,
    pub dwServerAuthenticationData: RASPPP_PROJECTION_INFO_SERVER_AUTH_DATA,
    pub dwEapTypeId: u32,
    pub dwServerEapTypeId: u32,
    pub dwLcpOptions: u32,
    pub dwLcpServerOptions: u32,
    pub dwCcpError: u32,
    pub dwCcpCompressionAlgorithm: u32,
    pub dwCcpServerCompressionAlgorithm: u32,
    pub dwCcpOptions: u32,
    pub dwCcpServerOptions: u32,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
impl ::core::marker::Copy for RASPPP_PROJECTION_INFO {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
impl ::core::clone::Clone for RASPPP_PROJECTION_INFO {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
unsafe impl ::windows::core::Abi for RASPPP_PROJECTION_INFO {
    type Abi = Self;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
impl ::core::cmp::PartialEq for RASPPP_PROJECTION_INFO {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<RASPPP_PROJECTION_INFO>()) == 0 }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
impl ::core::cmp::Eq for RASPPP_PROJECTION_INFO {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
impl ::core::default::Default for RASPPP_PROJECTION_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct RASPPP_PROJECTION_INFO_SERVER_AUTH_DATA(pub u32);
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`*"]
pub const RASLCPAD_CHAP_MD5: RASPPP_PROJECTION_INFO_SERVER_AUTH_DATA = RASPPP_PROJECTION_INFO_SERVER_AUTH_DATA(5u32);
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`*"]
pub const RASLCPAD_CHAP_MS: RASPPP_PROJECTION_INFO_SERVER_AUTH_DATA = RASPPP_PROJECTION_INFO_SERVER_AUTH_DATA(128u32);
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`*"]
pub const RASLCPAD_CHAP_MSV2: RASPPP_PROJECTION_INFO_SERVER_AUTH_DATA = RASPPP_PROJECTION_INFO_SERVER_AUTH_DATA(129u32);
impl ::core::marker::Copy for RASPPP_PROJECTION_INFO_SERVER_AUTH_DATA {}
impl ::core::clone::Clone for RASPPP_PROJECTION_INFO_SERVER_AUTH_DATA {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for RASPPP_PROJECTION_INFO_SERVER_AUTH_DATA {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for RASPPP_PROJECTION_INFO_SERVER_AUTH_DATA {
    type Abi = Self;
}
impl ::core::fmt::Debug for RASPPP_PROJECTION_INFO_SERVER_AUTH_DATA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("RASPPP_PROJECTION_INFO_SERVER_AUTH_DATA").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct RASPPP_PROJECTION_INFO_SERVER_AUTH_PROTOCOL(pub u32);
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`*"]
pub const RASLCPAP_PAP: RASPPP_PROJECTION_INFO_SERVER_AUTH_PROTOCOL = RASPPP_PROJECTION_INFO_SERVER_AUTH_PROTOCOL(49187u32);
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`*"]
pub const RASLCPAP_SPAP: RASPPP_PROJECTION_INFO_SERVER_AUTH_PROTOCOL = RASPPP_PROJECTION_INFO_SERVER_AUTH_PROTOCOL(49191u32);
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`*"]
pub const RASLCPAP_CHAP: RASPPP_PROJECTION_INFO_SERVER_AUTH_PROTOCOL = RASPPP_PROJECTION_INFO_SERVER_AUTH_PROTOCOL(49699u32);
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`*"]
pub const RASLCPAP_EAP: RASPPP_PROJECTION_INFO_SERVER_AUTH_PROTOCOL = RASPPP_PROJECTION_INFO_SERVER_AUTH_PROTOCOL(49703u32);
impl ::core::marker::Copy for RASPPP_PROJECTION_INFO_SERVER_AUTH_PROTOCOL {}
impl ::core::clone::Clone for RASPPP_PROJECTION_INFO_SERVER_AUTH_PROTOCOL {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for RASPPP_PROJECTION_INFO_SERVER_AUTH_PROTOCOL {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for RASPPP_PROJECTION_INFO_SERVER_AUTH_PROTOCOL {
    type Abi = Self;
}
impl ::core::fmt::Debug for RASPPP_PROJECTION_INFO_SERVER_AUTH_PROTOCOL {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("RASPPP_PROJECTION_INFO_SERVER_AUTH_PROTOCOL").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`*"]
pub const RASPRIV2_DialinPolicy: u32 = 1u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`*"]
pub const RASPRIV_AdminSetCallback: u32 = 2u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`*"]
pub const RASPRIV_CallerSetCallback: u32 = 4u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`*"]
pub const RASPRIV_DialinPrivilege: u32 = 8u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`*"]
pub const RASPRIV_NoCallback: u32 = 1u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct RASPROJECTION(pub i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`*"]
pub const RASP_Amb: RASPROJECTION = RASPROJECTION(65536i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`*"]
pub const RASP_PppNbf: RASPROJECTION = RASPROJECTION(32831i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`*"]
pub const RASP_PppIpx: RASPROJECTION = RASPROJECTION(32811i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`*"]
pub const RASP_PppIp: RASPROJECTION = RASPROJECTION(32801i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`*"]
pub const RASP_PppCcp: RASPROJECTION = RASPROJECTION(33021i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`*"]
pub const RASP_PppLcp: RASPROJECTION = RASPROJECTION(49185i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`*"]
pub const RASP_PppIpv6: RASPROJECTION = RASPROJECTION(32855i32);
impl ::core::marker::Copy for RASPROJECTION {}
impl ::core::clone::Clone for RASPROJECTION {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for RASPROJECTION {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for RASPROJECTION {
    type Abi = Self;
}
impl ::core::fmt::Debug for RASPROJECTION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("RASPROJECTION").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct RASPROJECTION_INFO_TYPE(pub i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`*"]
pub const PROJECTION_INFO_TYPE_PPP: RASPROJECTION_INFO_TYPE = RASPROJECTION_INFO_TYPE(1i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`*"]
pub const PROJECTION_INFO_TYPE_IKEv2: RASPROJECTION_INFO_TYPE = RASPROJECTION_INFO_TYPE(2i32);
impl ::core::marker::Copy for RASPROJECTION_INFO_TYPE {}
impl ::core::clone::Clone for RASPROJECTION_INFO_TYPE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for RASPROJECTION_INFO_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for RASPROJECTION_INFO_TYPE {
    type Abi = Self;
}
impl ::core::fmt::Debug for RASPROJECTION_INFO_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("RASPROJECTION_INFO_TYPE").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`*"]
pub type RASSECURITYPROC = ::core::option::Option<unsafe extern "system" fn() -> u32>;
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct RASSUBENTRYA {
    pub dwSize: u32,
    pub dwfFlags: u32,
    pub szDeviceType: [super::super::Foundation::CHAR; 17],
    pub szDeviceName: [super::super::Foundation::CHAR; 129],
    pub szLocalPhoneNumber: [super::super::Foundation::CHAR; 129],
    pub dwAlternateOffset: u32,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for RASSUBENTRYA {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for RASSUBENTRYA {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for RASSUBENTRYA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("RASSUBENTRYA").field("dwSize", &self.dwSize).field("dwfFlags", &self.dwfFlags).field("szDeviceType", &self.szDeviceType).field("szDeviceName", &self.szDeviceName).field("szLocalPhoneNumber", &self.szLocalPhoneNumber).field("dwAlternateOffset", &self.dwAlternateOffset).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for RASSUBENTRYA {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for RASSUBENTRYA {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<RASSUBENTRYA>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for RASSUBENTRYA {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for RASSUBENTRYA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`*"]
pub struct RASSUBENTRYW {
    pub dwSize: u32,
    pub dwfFlags: u32,
    pub szDeviceType: [u16; 17],
    pub szDeviceName: [u16; 129],
    pub szLocalPhoneNumber: [u16; 129],
    pub dwAlternateOffset: u32,
}
impl ::core::marker::Copy for RASSUBENTRYW {}
impl ::core::clone::Clone for RASSUBENTRYW {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for RASSUBENTRYW {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("RASSUBENTRYW").field("dwSize", &self.dwSize).field("dwfFlags", &self.dwfFlags).field("szDeviceType", &self.szDeviceType).field("szDeviceName", &self.szDeviceName).field("szLocalPhoneNumber", &self.szLocalPhoneNumber).field("dwAlternateOffset", &self.dwAlternateOffset).finish()
    }
}
unsafe impl ::windows::core::Abi for RASSUBENTRYW {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for RASSUBENTRYW {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<RASSUBENTRYW>()) == 0 }
    }
}
impl ::core::cmp::Eq for RASSUBENTRYW {}
impl ::core::default::Default for RASSUBENTRYW {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`, `\"Win32_Networking_WinSock\"`*"]
#[cfg(feature = "Win32_Networking_WinSock")]
pub struct RASTUNNELENDPOINT {
    pub dwType: u32,
    pub Anonymous: RASTUNNELENDPOINT_0,
}
#[cfg(feature = "Win32_Networking_WinSock")]
impl ::core::marker::Copy for RASTUNNELENDPOINT {}
#[cfg(feature = "Win32_Networking_WinSock")]
impl ::core::clone::Clone for RASTUNNELENDPOINT {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Networking_WinSock")]
unsafe impl ::windows::core::Abi for RASTUNNELENDPOINT {
    type Abi = Self;
}
#[cfg(feature = "Win32_Networking_WinSock")]
impl ::core::cmp::PartialEq for RASTUNNELENDPOINT {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<RASTUNNELENDPOINT>()) == 0 }
    }
}
#[cfg(feature = "Win32_Networking_WinSock")]
impl ::core::cmp::Eq for RASTUNNELENDPOINT {}
#[cfg(feature = "Win32_Networking_WinSock")]
impl ::core::default::Default for RASTUNNELENDPOINT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`, `\"Win32_Networking_WinSock\"`*"]
#[cfg(feature = "Win32_Networking_WinSock")]
pub union RASTUNNELENDPOINT_0 {
    pub ipv4: super::super::Networking::WinSock::IN_ADDR,
    pub ipv6: super::super::Networking::WinSock::IN6_ADDR,
}
#[cfg(feature = "Win32_Networking_WinSock")]
impl ::core::marker::Copy for RASTUNNELENDPOINT_0 {}
#[cfg(feature = "Win32_Networking_WinSock")]
impl ::core::clone::Clone for RASTUNNELENDPOINT_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Networking_WinSock")]
unsafe impl ::windows::core::Abi for RASTUNNELENDPOINT_0 {
    type Abi = Self;
}
#[cfg(feature = "Win32_Networking_WinSock")]
impl ::core::cmp::PartialEq for RASTUNNELENDPOINT_0 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<RASTUNNELENDPOINT_0>()) == 0 }
    }
}
#[cfg(feature = "Win32_Networking_WinSock")]
impl ::core::cmp::Eq for RASTUNNELENDPOINT_0 {}
#[cfg(feature = "Win32_Networking_WinSock")]
impl ::core::default::Default for RASTUNNELENDPOINT_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`*"]
pub const RASTUNNELENDPOINT_IPv4: u32 = 1u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`*"]
pub const RASTUNNELENDPOINT_IPv6: u32 = 2u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`*"]
pub const RASTUNNELENDPOINT_UNKNOWN: u32 = 0u32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`, `\"Win32_Networking_WinSock\"`*"]
#[cfg(feature = "Win32_Networking_WinSock")]
pub struct RASUPDATECONN {
    pub version: RASAPIVERSION,
    pub dwSize: u32,
    pub dwFlags: u32,
    pub dwIfIndex: u32,
    pub localEndPoint: RASTUNNELENDPOINT,
    pub remoteEndPoint: RASTUNNELENDPOINT,
}
#[cfg(feature = "Win32_Networking_WinSock")]
impl ::core::marker::Copy for RASUPDATECONN {}
#[cfg(feature = "Win32_Networking_WinSock")]
impl ::core::clone::Clone for RASUPDATECONN {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Networking_WinSock")]
unsafe impl ::windows::core::Abi for RASUPDATECONN {
    type Abi = Self;
}
#[cfg(feature = "Win32_Networking_WinSock")]
impl ::core::cmp::PartialEq for RASUPDATECONN {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<RASUPDATECONN>()) == 0 }
    }
}
#[cfg(feature = "Win32_Networking_WinSock")]
impl ::core::cmp::Eq for RASUPDATECONN {}
#[cfg(feature = "Win32_Networking_WinSock")]
impl ::core::default::Default for RASUPDATECONN {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct RAS_CONNECTION_0 {
    pub hConnection: super::super::Foundation::HANDLE,
    pub hInterface: super::super::Foundation::HANDLE,
    pub dwConnectDuration: u32,
    pub dwInterfaceType: ROUTER_INTERFACE_TYPE,
    pub dwConnectionFlags: RAS_FLAGS,
    pub wszInterfaceName: [u16; 257],
    pub wszUserName: [u16; 257],
    pub wszLogonDomain: [u16; 16],
    pub wszRemoteComputer: [u16; 17],
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for RAS_CONNECTION_0 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for RAS_CONNECTION_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for RAS_CONNECTION_0 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("RAS_CONNECTION_0").field("hConnection", &self.hConnection).field("hInterface", &self.hInterface).field("dwConnectDuration", &self.dwConnectDuration).field("dwInterfaceType", &self.dwInterfaceType).field("dwConnectionFlags", &self.dwConnectionFlags).field("wszInterfaceName", &self.wszInterfaceName).field("wszUserName", &self.wszUserName).field("wszLogonDomain", &self.wszLogonDomain).field("wszRemoteComputer", &self.wszRemoteComputer).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for RAS_CONNECTION_0 {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for RAS_CONNECTION_0 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<RAS_CONNECTION_0>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for RAS_CONNECTION_0 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for RAS_CONNECTION_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct RAS_CONNECTION_1 {
    pub hConnection: super::super::Foundation::HANDLE,
    pub hInterface: super::super::Foundation::HANDLE,
    pub PppInfo: PPP_INFO,
    pub dwBytesXmited: u32,
    pub dwBytesRcved: u32,
    pub dwFramesXmited: u32,
    pub dwFramesRcved: u32,
    pub dwCrcErr: u32,
    pub dwTimeoutErr: u32,
    pub dwAlignmentErr: u32,
    pub dwHardwareOverrunErr: u32,
    pub dwFramingErr: u32,
    pub dwBufferOverrunErr: u32,
    pub dwCompressionRatioIn: u32,
    pub dwCompressionRatioOut: u32,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for RAS_CONNECTION_1 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for RAS_CONNECTION_1 {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for RAS_CONNECTION_1 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("RAS_CONNECTION_1")
            .field("hConnection", &self.hConnection)
            .field("hInterface", &self.hInterface)
            .field("PppInfo", &self.PppInfo)
            .field("dwBytesXmited", &self.dwBytesXmited)
            .field("dwBytesRcved", &self.dwBytesRcved)
            .field("dwFramesXmited", &self.dwFramesXmited)
            .field("dwFramesRcved", &self.dwFramesRcved)
            .field("dwCrcErr", &self.dwCrcErr)
            .field("dwTimeoutErr", &self.dwTimeoutErr)
            .field("dwAlignmentErr", &self.dwAlignmentErr)
            .field("dwHardwareOverrunErr", &self.dwHardwareOverrunErr)
            .field("dwFramingErr", &self.dwFramingErr)
            .field("dwBufferOverrunErr", &self.dwBufferOverrunErr)
            .field("dwCompressionRatioIn", &self.dwCompressionRatioIn)
            .field("dwCompressionRatioOut", &self.dwCompressionRatioOut)
            .finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for RAS_CONNECTION_1 {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for RAS_CONNECTION_1 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<RAS_CONNECTION_1>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for RAS_CONNECTION_1 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for RAS_CONNECTION_1 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct RAS_CONNECTION_2 {
    pub hConnection: super::super::Foundation::HANDLE,
    pub wszUserName: [u16; 257],
    pub dwInterfaceType: ROUTER_INTERFACE_TYPE,
    pub guid: ::windows::core::GUID,
    pub PppInfo2: PPP_INFO_2,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for RAS_CONNECTION_2 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for RAS_CONNECTION_2 {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for RAS_CONNECTION_2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("RAS_CONNECTION_2").field("hConnection", &self.hConnection).field("wszUserName", &self.wszUserName).field("dwInterfaceType", &self.dwInterfaceType).field("guid", &self.guid).field("PppInfo2", &self.PppInfo2).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for RAS_CONNECTION_2 {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for RAS_CONNECTION_2 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<RAS_CONNECTION_2>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for RAS_CONNECTION_2 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for RAS_CONNECTION_2 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct RAS_CONNECTION_3 {
    pub dwVersion: u32,
    pub dwSize: u32,
    pub hConnection: super::super::Foundation::HANDLE,
    pub wszUserName: [u16; 257],
    pub dwInterfaceType: ROUTER_INTERFACE_TYPE,
    pub guid: ::windows::core::GUID,
    pub PppInfo3: PPP_INFO_3,
    pub rasQuarState: RAS_QUARANTINE_STATE,
    pub timer: super::super::Foundation::FILETIME,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for RAS_CONNECTION_3 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for RAS_CONNECTION_3 {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for RAS_CONNECTION_3 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("RAS_CONNECTION_3").field("dwVersion", &self.dwVersion).field("dwSize", &self.dwSize).field("hConnection", &self.hConnection).field("wszUserName", &self.wszUserName).field("dwInterfaceType", &self.dwInterfaceType).field("guid", &self.guid).field("PppInfo3", &self.PppInfo3).field("rasQuarState", &self.rasQuarState).field("timer", &self.timer).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for RAS_CONNECTION_3 {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for RAS_CONNECTION_3 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<RAS_CONNECTION_3>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for RAS_CONNECTION_3 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for RAS_CONNECTION_3 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct RAS_CONNECTION_4 {
    pub dwConnectDuration: u32,
    pub dwInterfaceType: ROUTER_INTERFACE_TYPE,
    pub dwConnectionFlags: RAS_FLAGS,
    pub wszInterfaceName: [u16; 257],
    pub wszUserName: [u16; 257],
    pub wszLogonDomain: [u16; 16],
    pub wszRemoteComputer: [u16; 17],
    pub guid: ::windows::core::GUID,
    pub rasQuarState: RAS_QUARANTINE_STATE,
    pub probationTime: super::super::Foundation::FILETIME,
    pub connectionStartTime: super::super::Foundation::FILETIME,
    pub ullBytesXmited: u64,
    pub ullBytesRcved: u64,
    pub dwFramesXmited: u32,
    pub dwFramesRcved: u32,
    pub dwCrcErr: u32,
    pub dwTimeoutErr: u32,
    pub dwAlignmentErr: u32,
    pub dwHardwareOverrunErr: u32,
    pub dwFramingErr: u32,
    pub dwBufferOverrunErr: u32,
    pub dwCompressionRatioIn: u32,
    pub dwCompressionRatioOut: u32,
    pub dwNumSwitchOvers: u32,
    pub wszRemoteEndpointAddress: [u16; 65],
    pub wszLocalEndpointAddress: [u16; 65],
    pub ProjectionInfo: PROJECTION_INFO2,
    pub hConnection: super::super::Foundation::HANDLE,
    pub hInterface: super::super::Foundation::HANDLE,
    pub dwDeviceType: u32,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for RAS_CONNECTION_4 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for RAS_CONNECTION_4 {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for RAS_CONNECTION_4 {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for RAS_CONNECTION_4 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<RAS_CONNECTION_4>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for RAS_CONNECTION_4 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for RAS_CONNECTION_4 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct RAS_CONNECTION_EX {
    pub Header: MPRAPI_OBJECT_HEADER,
    pub dwConnectDuration: u32,
    pub dwInterfaceType: ROUTER_INTERFACE_TYPE,
    pub dwConnectionFlags: RAS_FLAGS,
    pub wszInterfaceName: [u16; 257],
    pub wszUserName: [u16; 257],
    pub wszLogonDomain: [u16; 16],
    pub wszRemoteComputer: [u16; 17],
    pub guid: ::windows::core::GUID,
    pub rasQuarState: RAS_QUARANTINE_STATE,
    pub probationTime: super::super::Foundation::FILETIME,
    pub dwBytesXmited: u32,
    pub dwBytesRcved: u32,
    pub dwFramesXmited: u32,
    pub dwFramesRcved: u32,
    pub dwCrcErr: u32,
    pub dwTimeoutErr: u32,
    pub dwAlignmentErr: u32,
    pub dwHardwareOverrunErr: u32,
    pub dwFramingErr: u32,
    pub dwBufferOverrunErr: u32,
    pub dwCompressionRatioIn: u32,
    pub dwCompressionRatioOut: u32,
    pub dwNumSwitchOvers: u32,
    pub wszRemoteEndpointAddress: [u16; 65],
    pub wszLocalEndpointAddress: [u16; 65],
    pub ProjectionInfo: PROJECTION_INFO,
    pub hConnection: super::super::Foundation::HANDLE,
    pub hInterface: super::super::Foundation::HANDLE,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for RAS_CONNECTION_EX {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for RAS_CONNECTION_EX {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for RAS_CONNECTION_EX {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for RAS_CONNECTION_EX {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<RAS_CONNECTION_EX>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for RAS_CONNECTION_EX {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for RAS_CONNECTION_EX {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct RAS_FLAGS(pub u32);
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`*"]
pub const RAS_FLAGS_PPP_CONNECTION: RAS_FLAGS = RAS_FLAGS(1u32);
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`*"]
pub const RAS_FLAGS_MESSENGER_PRESENT: RAS_FLAGS = RAS_FLAGS(2u32);
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`*"]
pub const RAS_FLAGS_QUARANTINE_PRESENT: RAS_FLAGS = RAS_FLAGS(8u32);
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`*"]
pub const RAS_FLAGS_ARAP_CONNECTION: RAS_FLAGS = RAS_FLAGS(16u32);
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`*"]
pub const RAS_FLAGS_IKEV2_CONNECTION: RAS_FLAGS = RAS_FLAGS(16u32);
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`*"]
pub const RAS_FLAGS_DORMANT: RAS_FLAGS = RAS_FLAGS(32u32);
impl ::core::marker::Copy for RAS_FLAGS {}
impl ::core::clone::Clone for RAS_FLAGS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for RAS_FLAGS {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for RAS_FLAGS {
    type Abi = Self;
}
impl ::core::fmt::Debug for RAS_FLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("RAS_FLAGS").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`*"]
pub const RAS_FLAGS_RAS_CONNECTION: u32 = 4u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct RAS_HARDWARE_CONDITION(pub i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`*"]
pub const RAS_HARDWARE_OPERATIONAL: RAS_HARDWARE_CONDITION = RAS_HARDWARE_CONDITION(0i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`*"]
pub const RAS_HARDWARE_FAILURE: RAS_HARDWARE_CONDITION = RAS_HARDWARE_CONDITION(1i32);
impl ::core::marker::Copy for RAS_HARDWARE_CONDITION {}
impl ::core::clone::Clone for RAS_HARDWARE_CONDITION {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for RAS_HARDWARE_CONDITION {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for RAS_HARDWARE_CONDITION {
    type Abi = Self;
}
impl ::core::fmt::Debug for RAS_HARDWARE_CONDITION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("RAS_HARDWARE_CONDITION").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`*"]
pub const RAS_MaxAreaCode: u32 = 10u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`*"]
pub const RAS_MaxCallbackNumber: u32 = 128u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`*"]
pub const RAS_MaxDeviceName: u32 = 128u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`*"]
pub const RAS_MaxDeviceType: u32 = 16u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`*"]
pub const RAS_MaxDnsSuffix: u32 = 256u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`*"]
pub const RAS_MaxEntryName: u32 = 256u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`*"]
pub const RAS_MaxFacilities: u32 = 200u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`*"]
pub const RAS_MaxIDSize: u32 = 256u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`*"]
pub const RAS_MaxIpAddress: u32 = 15u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`*"]
pub const RAS_MaxIpxAddress: u32 = 21u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`*"]
pub const RAS_MaxPadType: u32 = 32u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`*"]
pub const RAS_MaxPhoneNumber: u32 = 128u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`*"]
pub const RAS_MaxReplyMessage: u32 = 1024u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`*"]
pub const RAS_MaxUserData: u32 = 200u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`*"]
pub const RAS_MaxX25Address: u32 = 200u32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct RAS_PORT_0 {
    pub hPort: super::super::Foundation::HANDLE,
    pub hConnection: super::super::Foundation::HANDLE,
    pub dwPortCondition: RAS_PORT_CONDITION,
    pub dwTotalNumberOfCalls: u32,
    pub dwConnectDuration: u32,
    pub wszPortName: [u16; 17],
    pub wszMediaName: [u16; 17],
    pub wszDeviceName: [u16; 129],
    pub wszDeviceType: [u16; 17],
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for RAS_PORT_0 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for RAS_PORT_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for RAS_PORT_0 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("RAS_PORT_0").field("hPort", &self.hPort).field("hConnection", &self.hConnection).field("dwPortCondition", &self.dwPortCondition).field("dwTotalNumberOfCalls", &self.dwTotalNumberOfCalls).field("dwConnectDuration", &self.dwConnectDuration).field("wszPortName", &self.wszPortName).field("wszMediaName", &self.wszMediaName).field("wszDeviceName", &self.wszDeviceName).field("wszDeviceType", &self.wszDeviceType).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for RAS_PORT_0 {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for RAS_PORT_0 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<RAS_PORT_0>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for RAS_PORT_0 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for RAS_PORT_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct RAS_PORT_1 {
    pub hPort: super::super::Foundation::HANDLE,
    pub hConnection: super::super::Foundation::HANDLE,
    pub dwHardwareCondition: RAS_HARDWARE_CONDITION,
    pub dwLineSpeed: u32,
    pub dwBytesXmited: u32,
    pub dwBytesRcved: u32,
    pub dwFramesXmited: u32,
    pub dwFramesRcved: u32,
    pub dwCrcErr: u32,
    pub dwTimeoutErr: u32,
    pub dwAlignmentErr: u32,
    pub dwHardwareOverrunErr: u32,
    pub dwFramingErr: u32,
    pub dwBufferOverrunErr: u32,
    pub dwCompressionRatioIn: u32,
    pub dwCompressionRatioOut: u32,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for RAS_PORT_1 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for RAS_PORT_1 {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for RAS_PORT_1 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("RAS_PORT_1")
            .field("hPort", &self.hPort)
            .field("hConnection", &self.hConnection)
            .field("dwHardwareCondition", &self.dwHardwareCondition)
            .field("dwLineSpeed", &self.dwLineSpeed)
            .field("dwBytesXmited", &self.dwBytesXmited)
            .field("dwBytesRcved", &self.dwBytesRcved)
            .field("dwFramesXmited", &self.dwFramesXmited)
            .field("dwFramesRcved", &self.dwFramesRcved)
            .field("dwCrcErr", &self.dwCrcErr)
            .field("dwTimeoutErr", &self.dwTimeoutErr)
            .field("dwAlignmentErr", &self.dwAlignmentErr)
            .field("dwHardwareOverrunErr", &self.dwHardwareOverrunErr)
            .field("dwFramingErr", &self.dwFramingErr)
            .field("dwBufferOverrunErr", &self.dwBufferOverrunErr)
            .field("dwCompressionRatioIn", &self.dwCompressionRatioIn)
            .field("dwCompressionRatioOut", &self.dwCompressionRatioOut)
            .finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for RAS_PORT_1 {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for RAS_PORT_1 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<RAS_PORT_1>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for RAS_PORT_1 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for RAS_PORT_1 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct RAS_PORT_2 {
    pub hPort: super::super::Foundation::HANDLE,
    pub hConnection: super::super::Foundation::HANDLE,
    pub dwConn_State: u32,
    pub wszPortName: [u16; 17],
    pub wszMediaName: [u16; 17],
    pub wszDeviceName: [u16; 129],
    pub wszDeviceType: [u16; 17],
    pub dwHardwareCondition: RAS_HARDWARE_CONDITION,
    pub dwLineSpeed: u32,
    pub dwCrcErr: u32,
    pub dwSerialOverRunErrs: u32,
    pub dwTimeoutErr: u32,
    pub dwAlignmentErr: u32,
    pub dwHardwareOverrunErr: u32,
    pub dwFramingErr: u32,
    pub dwBufferOverrunErr: u32,
    pub dwCompressionRatioIn: u32,
    pub dwCompressionRatioOut: u32,
    pub dwTotalErrors: u32,
    pub ullBytesXmited: u64,
    pub ullBytesRcved: u64,
    pub ullFramesXmited: u64,
    pub ullFramesRcved: u64,
    pub ullBytesTxUncompressed: u64,
    pub ullBytesTxCompressed: u64,
    pub ullBytesRcvUncompressed: u64,
    pub ullBytesRcvCompressed: u64,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for RAS_PORT_2 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for RAS_PORT_2 {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for RAS_PORT_2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("RAS_PORT_2")
            .field("hPort", &self.hPort)
            .field("hConnection", &self.hConnection)
            .field("dwConn_State", &self.dwConn_State)
            .field("wszPortName", &self.wszPortName)
            .field("wszMediaName", &self.wszMediaName)
            .field("wszDeviceName", &self.wszDeviceName)
            .field("wszDeviceType", &self.wszDeviceType)
            .field("dwHardwareCondition", &self.dwHardwareCondition)
            .field("dwLineSpeed", &self.dwLineSpeed)
            .field("dwCrcErr", &self.dwCrcErr)
            .field("dwSerialOverRunErrs", &self.dwSerialOverRunErrs)
            .field("dwTimeoutErr", &self.dwTimeoutErr)
            .field("dwAlignmentErr", &self.dwAlignmentErr)
            .field("dwHardwareOverrunErr", &self.dwHardwareOverrunErr)
            .field("dwFramingErr", &self.dwFramingErr)
            .field("dwBufferOverrunErr", &self.dwBufferOverrunErr)
            .field("dwCompressionRatioIn", &self.dwCompressionRatioIn)
            .field("dwCompressionRatioOut", &self.dwCompressionRatioOut)
            .field("dwTotalErrors", &self.dwTotalErrors)
            .field("ullBytesXmited", &self.ullBytesXmited)
            .field("ullBytesRcved", &self.ullBytesRcved)
            .field("ullFramesXmited", &self.ullFramesXmited)
            .field("ullFramesRcved", &self.ullFramesRcved)
            .field("ullBytesTxUncompressed", &self.ullBytesTxUncompressed)
            .field("ullBytesTxCompressed", &self.ullBytesTxCompressed)
            .field("ullBytesRcvUncompressed", &self.ullBytesRcvUncompressed)
            .field("ullBytesRcvCompressed", &self.ullBytesRcvCompressed)
            .finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for RAS_PORT_2 {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for RAS_PORT_2 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<RAS_PORT_2>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for RAS_PORT_2 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for RAS_PORT_2 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct RAS_PORT_CONDITION(pub i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`*"]
pub const RAS_PORT_NON_OPERATIONAL: RAS_PORT_CONDITION = RAS_PORT_CONDITION(0i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`*"]
pub const RAS_PORT_DISCONNECTED: RAS_PORT_CONDITION = RAS_PORT_CONDITION(1i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`*"]
pub const RAS_PORT_CALLING_BACK: RAS_PORT_CONDITION = RAS_PORT_CONDITION(2i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`*"]
pub const RAS_PORT_LISTENING: RAS_PORT_CONDITION = RAS_PORT_CONDITION(3i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`*"]
pub const RAS_PORT_AUTHENTICATING: RAS_PORT_CONDITION = RAS_PORT_CONDITION(4i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`*"]
pub const RAS_PORT_AUTHENTICATED: RAS_PORT_CONDITION = RAS_PORT_CONDITION(5i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`*"]
pub const RAS_PORT_INITIALIZING: RAS_PORT_CONDITION = RAS_PORT_CONDITION(6i32);
impl ::core::marker::Copy for RAS_PORT_CONDITION {}
impl ::core::clone::Clone for RAS_PORT_CONDITION {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for RAS_PORT_CONDITION {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for RAS_PORT_CONDITION {
    type Abi = Self;
}
impl ::core::fmt::Debug for RAS_PORT_CONDITION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("RAS_PORT_CONDITION").field(&self.0).finish()
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`, `\"Win32_Foundation\"`, `\"Win32_Networking_WinSock\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
pub struct RAS_PROJECTION_INFO {
    pub version: RASAPIVERSION,
    pub r#type: RASPROJECTION_INFO_TYPE,
    pub Anonymous: RAS_PROJECTION_INFO_0,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
impl ::core::marker::Copy for RAS_PROJECTION_INFO {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
impl ::core::clone::Clone for RAS_PROJECTION_INFO {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
unsafe impl ::windows::core::Abi for RAS_PROJECTION_INFO {
    type Abi = Self;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
impl ::core::cmp::PartialEq for RAS_PROJECTION_INFO {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<RAS_PROJECTION_INFO>()) == 0 }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
impl ::core::cmp::Eq for RAS_PROJECTION_INFO {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
impl ::core::default::Default for RAS_PROJECTION_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`, `\"Win32_Foundation\"`, `\"Win32_Networking_WinSock\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
pub union RAS_PROJECTION_INFO_0 {
    pub ppp: RASPPP_PROJECTION_INFO,
    pub ikev2: RASIKEV2_PROJECTION_INFO,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
impl ::core::marker::Copy for RAS_PROJECTION_INFO_0 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
impl ::core::clone::Clone for RAS_PROJECTION_INFO_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
unsafe impl ::windows::core::Abi for RAS_PROJECTION_INFO_0 {
    type Abi = Self;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
impl ::core::cmp::PartialEq for RAS_PROJECTION_INFO_0 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<RAS_PROJECTION_INFO_0>()) == 0 }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
impl ::core::cmp::Eq for RAS_PROJECTION_INFO_0 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
impl ::core::default::Default for RAS_PROJECTION_INFO_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct RAS_QUARANTINE_STATE(pub i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`*"]
pub const RAS_QUAR_STATE_NORMAL: RAS_QUARANTINE_STATE = RAS_QUARANTINE_STATE(0i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`*"]
pub const RAS_QUAR_STATE_QUARANTINE: RAS_QUARANTINE_STATE = RAS_QUARANTINE_STATE(1i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`*"]
pub const RAS_QUAR_STATE_PROBATION: RAS_QUARANTINE_STATE = RAS_QUARANTINE_STATE(2i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`*"]
pub const RAS_QUAR_STATE_NOT_CAPABLE: RAS_QUARANTINE_STATE = RAS_QUARANTINE_STATE(3i32);
impl ::core::marker::Copy for RAS_QUARANTINE_STATE {}
impl ::core::clone::Clone for RAS_QUARANTINE_STATE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for RAS_QUARANTINE_STATE {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for RAS_QUARANTINE_STATE {
    type Abi = Self;
}
impl ::core::fmt::Debug for RAS_QUARANTINE_STATE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("RAS_QUARANTINE_STATE").field(&self.0).finish()
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct RAS_SECURITY_INFO {
    pub LastError: u32,
    pub BytesReceived: u32,
    pub DeviceName: [super::super::Foundation::CHAR; 129],
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for RAS_SECURITY_INFO {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for RAS_SECURITY_INFO {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for RAS_SECURITY_INFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("RAS_SECURITY_INFO").field("LastError", &self.LastError).field("BytesReceived", &self.BytesReceived).field("DeviceName", &self.DeviceName).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for RAS_SECURITY_INFO {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for RAS_SECURITY_INFO {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<RAS_SECURITY_INFO>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for RAS_SECURITY_INFO {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for RAS_SECURITY_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`*"]
pub struct RAS_STATS {
    pub dwSize: u32,
    pub dwBytesXmited: u32,
    pub dwBytesRcved: u32,
    pub dwFramesXmited: u32,
    pub dwFramesRcved: u32,
    pub dwCrcErr: u32,
    pub dwTimeoutErr: u32,
    pub dwAlignmentErr: u32,
    pub dwHardwareOverrunErr: u32,
    pub dwFramingErr: u32,
    pub dwBufferOverrunErr: u32,
    pub dwCompressionRatioIn: u32,
    pub dwCompressionRatioOut: u32,
    pub dwBps: u32,
    pub dwConnectDuration: u32,
}
impl ::core::marker::Copy for RAS_STATS {}
impl ::core::clone::Clone for RAS_STATS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for RAS_STATS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("RAS_STATS")
            .field("dwSize", &self.dwSize)
            .field("dwBytesXmited", &self.dwBytesXmited)
            .field("dwBytesRcved", &self.dwBytesRcved)
            .field("dwFramesXmited", &self.dwFramesXmited)
            .field("dwFramesRcved", &self.dwFramesRcved)
            .field("dwCrcErr", &self.dwCrcErr)
            .field("dwTimeoutErr", &self.dwTimeoutErr)
            .field("dwAlignmentErr", &self.dwAlignmentErr)
            .field("dwHardwareOverrunErr", &self.dwHardwareOverrunErr)
            .field("dwFramingErr", &self.dwFramingErr)
            .field("dwBufferOverrunErr", &self.dwBufferOverrunErr)
            .field("dwCompressionRatioIn", &self.dwCompressionRatioIn)
            .field("dwCompressionRatioOut", &self.dwCompressionRatioOut)
            .field("dwBps", &self.dwBps)
            .field("dwConnectDuration", &self.dwConnectDuration)
            .finish()
    }
}
unsafe impl ::windows::core::Abi for RAS_STATS {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for RAS_STATS {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<RAS_STATS>()) == 0 }
    }
}
impl ::core::cmp::Eq for RAS_STATS {}
impl ::core::default::Default for RAS_STATS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`*"]
pub struct RAS_UPDATE_CONNECTION {
    pub Header: MPRAPI_OBJECT_HEADER,
    pub dwIfIndex: u32,
    pub wszLocalEndpointAddress: [u16; 65],
    pub wszRemoteEndpointAddress: [u16; 65],
}
impl ::core::marker::Copy for RAS_UPDATE_CONNECTION {}
impl ::core::clone::Clone for RAS_UPDATE_CONNECTION {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for RAS_UPDATE_CONNECTION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("RAS_UPDATE_CONNECTION").field("Header", &self.Header).field("dwIfIndex", &self.dwIfIndex).field("wszLocalEndpointAddress", &self.wszLocalEndpointAddress).field("wszRemoteEndpointAddress", &self.wszRemoteEndpointAddress).finish()
    }
}
unsafe impl ::windows::core::Abi for RAS_UPDATE_CONNECTION {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for RAS_UPDATE_CONNECTION {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<RAS_UPDATE_CONNECTION>()) == 0 }
    }
}
impl ::core::cmp::Eq for RAS_UPDATE_CONNECTION {}
impl ::core::default::Default for RAS_UPDATE_CONNECTION {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`*"]
pub struct RAS_USER_0 {
    pub bfPrivilege: u8,
    pub wszPhoneNumber: [u16; 129],
}
impl ::core::marker::Copy for RAS_USER_0 {}
impl ::core::clone::Clone for RAS_USER_0 {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for RAS_USER_0 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("RAS_USER_0").field("bfPrivilege", &self.bfPrivilege).field("wszPhoneNumber", &self.wszPhoneNumber).finish()
    }
}
unsafe impl ::windows::core::Abi for RAS_USER_0 {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for RAS_USER_0 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<RAS_USER_0>()) == 0 }
    }
}
impl ::core::cmp::Eq for RAS_USER_0 {}
impl ::core::default::Default for RAS_USER_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`*"]
pub struct RAS_USER_1 {
    pub bfPrivilege: u8,
    pub wszPhoneNumber: [u16; 129],
    pub bfPrivilege2: u8,
}
impl ::core::marker::Copy for RAS_USER_1 {}
impl ::core::clone::Clone for RAS_USER_1 {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for RAS_USER_1 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("RAS_USER_1").field("bfPrivilege", &self.bfPrivilege).field("wszPhoneNumber", &self.wszPhoneNumber).field("bfPrivilege2", &self.bfPrivilege2).finish()
    }
}
unsafe impl ::windows::core::Abi for RAS_USER_1 {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for RAS_USER_1 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<RAS_USER_1>()) == 0 }
    }
}
impl ::core::cmp::Eq for RAS_USER_1 {}
impl ::core::default::Default for RAS_USER_1 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`*"]
pub const RCD_AllUsers: u32 = 1u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`*"]
pub const RCD_Eap: u32 = 2u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`*"]
pub const RCD_Logon: u32 = 4u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`*"]
pub const RCD_SingleUser: u32 = 0u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`*"]
pub const RDEOPT_CustomDial: u32 = 4096u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`*"]
pub const RDEOPT_DisableConnectedUI: u32 = 64u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`*"]
pub const RDEOPT_DisableReconnect: u32 = 256u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`*"]
pub const RDEOPT_DisableReconnectUI: u32 = 128u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`*"]
pub const RDEOPT_EapInfoCryptInCapable: u32 = 32768u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`*"]
pub const RDEOPT_IgnoreModemSpeaker: u32 = 4u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`*"]
pub const RDEOPT_IgnoreSoftwareCompression: u32 = 16u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`*"]
pub const RDEOPT_InvokeAutoTriggerCredentialUI: u32 = 16384u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`*"]
pub const RDEOPT_NoUser: u32 = 512u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`*"]
pub const RDEOPT_PauseOnScript: u32 = 1024u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`*"]
pub const RDEOPT_PausedStates: u32 = 2u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`*"]
pub const RDEOPT_Router: u32 = 2048u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`*"]
pub const RDEOPT_SetModemSpeaker: u32 = 8u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`*"]
pub const RDEOPT_SetSoftwareCompression: u32 = 32u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`*"]
pub const RDEOPT_UseCustomScripting: u32 = 8192u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`*"]
pub const RDEOPT_UsePrefixSuffix: u32 = 1u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`*"]
pub const REN_AllUsers: u32 = 1u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`*"]
pub const REN_User: u32 = 0u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct ROUTER_CONNECTION_STATE(pub i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`*"]
pub const ROUTER_IF_STATE_UNREACHABLE: ROUTER_CONNECTION_STATE = ROUTER_CONNECTION_STATE(0i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`*"]
pub const ROUTER_IF_STATE_DISCONNECTED: ROUTER_CONNECTION_STATE = ROUTER_CONNECTION_STATE(1i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`*"]
pub const ROUTER_IF_STATE_CONNECTING: ROUTER_CONNECTION_STATE = ROUTER_CONNECTION_STATE(2i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`*"]
pub const ROUTER_IF_STATE_CONNECTED: ROUTER_CONNECTION_STATE = ROUTER_CONNECTION_STATE(3i32);
impl ::core::marker::Copy for ROUTER_CONNECTION_STATE {}
impl ::core::clone::Clone for ROUTER_CONNECTION_STATE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for ROUTER_CONNECTION_STATE {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for ROUTER_CONNECTION_STATE {
    type Abi = Self;
}
impl ::core::fmt::Debug for ROUTER_CONNECTION_STATE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ROUTER_CONNECTION_STATE").field(&self.0).finish()
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`*"]
pub struct ROUTER_CUSTOM_IKEv2_POLICY0 {
    pub dwIntegrityMethod: u32,
    pub dwEncryptionMethod: u32,
    pub dwCipherTransformConstant: u32,
    pub dwAuthTransformConstant: u32,
    pub dwPfsGroup: u32,
    pub dwDhGroup: u32,
}
impl ::core::marker::Copy for ROUTER_CUSTOM_IKEv2_POLICY0 {}
impl ::core::clone::Clone for ROUTER_CUSTOM_IKEv2_POLICY0 {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for ROUTER_CUSTOM_IKEv2_POLICY0 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("ROUTER_CUSTOM_IKEv2_POLICY0").field("dwIntegrityMethod", &self.dwIntegrityMethod).field("dwEncryptionMethod", &self.dwEncryptionMethod).field("dwCipherTransformConstant", &self.dwCipherTransformConstant).field("dwAuthTransformConstant", &self.dwAuthTransformConstant).field("dwPfsGroup", &self.dwPfsGroup).field("dwDhGroup", &self.dwDhGroup).finish()
    }
}
unsafe impl ::windows::core::Abi for ROUTER_CUSTOM_IKEv2_POLICY0 {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for ROUTER_CUSTOM_IKEv2_POLICY0 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<ROUTER_CUSTOM_IKEv2_POLICY0>()) == 0 }
    }
}
impl ::core::cmp::Eq for ROUTER_CUSTOM_IKEv2_POLICY0 {}
impl ::core::default::Default for ROUTER_CUSTOM_IKEv2_POLICY0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`, `\"Win32_Security_Cryptography\"`*"]
#[cfg(feature = "Win32_Security_Cryptography")]
pub struct ROUTER_IKEv2_IF_CUSTOM_CONFIG0 {
    pub dwSaLifeTime: u32,
    pub dwSaDataSize: u32,
    pub certificateName: super::super::Security::Cryptography::CRYPTOAPI_BLOB,
    pub customPolicy: *mut ROUTER_CUSTOM_IKEv2_POLICY0,
}
#[cfg(feature = "Win32_Security_Cryptography")]
impl ::core::marker::Copy for ROUTER_IKEv2_IF_CUSTOM_CONFIG0 {}
#[cfg(feature = "Win32_Security_Cryptography")]
impl ::core::clone::Clone for ROUTER_IKEv2_IF_CUSTOM_CONFIG0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Security_Cryptography")]
impl ::core::fmt::Debug for ROUTER_IKEv2_IF_CUSTOM_CONFIG0 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("ROUTER_IKEv2_IF_CUSTOM_CONFIG0").field("dwSaLifeTime", &self.dwSaLifeTime).field("dwSaDataSize", &self.dwSaDataSize).field("certificateName", &self.certificateName).field("customPolicy", &self.customPolicy).finish()
    }
}
#[cfg(feature = "Win32_Security_Cryptography")]
unsafe impl ::windows::core::Abi for ROUTER_IKEv2_IF_CUSTOM_CONFIG0 {
    type Abi = Self;
}
#[cfg(feature = "Win32_Security_Cryptography")]
impl ::core::cmp::PartialEq for ROUTER_IKEv2_IF_CUSTOM_CONFIG0 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<ROUTER_IKEv2_IF_CUSTOM_CONFIG0>()) == 0 }
    }
}
#[cfg(feature = "Win32_Security_Cryptography")]
impl ::core::cmp::Eq for ROUTER_IKEv2_IF_CUSTOM_CONFIG0 {}
#[cfg(feature = "Win32_Security_Cryptography")]
impl ::core::default::Default for ROUTER_IKEv2_IF_CUSTOM_CONFIG0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`, `\"Win32_Security_Cryptography\"`*"]
#[cfg(feature = "Win32_Security_Cryptography")]
pub struct ROUTER_IKEv2_IF_CUSTOM_CONFIG1 {
    pub dwSaLifeTime: u32,
    pub dwSaDataSize: u32,
    pub certificateName: super::super::Security::Cryptography::CRYPTOAPI_BLOB,
    pub customPolicy: *mut ROUTER_CUSTOM_IKEv2_POLICY0,
    pub certificateHash: super::super::Security::Cryptography::CRYPTOAPI_BLOB,
}
#[cfg(feature = "Win32_Security_Cryptography")]
impl ::core::marker::Copy for ROUTER_IKEv2_IF_CUSTOM_CONFIG1 {}
#[cfg(feature = "Win32_Security_Cryptography")]
impl ::core::clone::Clone for ROUTER_IKEv2_IF_CUSTOM_CONFIG1 {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Security_Cryptography")]
impl ::core::fmt::Debug for ROUTER_IKEv2_IF_CUSTOM_CONFIG1 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("ROUTER_IKEv2_IF_CUSTOM_CONFIG1").field("dwSaLifeTime", &self.dwSaLifeTime).field("dwSaDataSize", &self.dwSaDataSize).field("certificateName", &self.certificateName).field("customPolicy", &self.customPolicy).field("certificateHash", &self.certificateHash).finish()
    }
}
#[cfg(feature = "Win32_Security_Cryptography")]
unsafe impl ::windows::core::Abi for ROUTER_IKEv2_IF_CUSTOM_CONFIG1 {
    type Abi = Self;
}
#[cfg(feature = "Win32_Security_Cryptography")]
impl ::core::cmp::PartialEq for ROUTER_IKEv2_IF_CUSTOM_CONFIG1 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<ROUTER_IKEv2_IF_CUSTOM_CONFIG1>()) == 0 }
    }
}
#[cfg(feature = "Win32_Security_Cryptography")]
impl ::core::cmp::Eq for ROUTER_IKEv2_IF_CUSTOM_CONFIG1 {}
#[cfg(feature = "Win32_Security_Cryptography")]
impl ::core::default::Default for ROUTER_IKEv2_IF_CUSTOM_CONFIG1 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`, `\"Win32_Networking_WinSock\"`, `\"Win32_Security_Cryptography\"`*"]
#[cfg(all(feature = "Win32_Networking_WinSock", feature = "Win32_Security_Cryptography"))]
pub struct ROUTER_IKEv2_IF_CUSTOM_CONFIG2 {
    pub dwSaLifeTime: u32,
    pub dwSaDataSize: u32,
    pub certificateName: super::super::Security::Cryptography::CRYPTOAPI_BLOB,
    pub customPolicy: *mut ROUTER_CUSTOM_IKEv2_POLICY0,
    pub certificateHash: super::super::Security::Cryptography::CRYPTOAPI_BLOB,
    pub dwMmSaLifeTime: u32,
    pub vpnTrafficSelectors: MPR_VPN_TRAFFIC_SELECTORS,
}
#[cfg(all(feature = "Win32_Networking_WinSock", feature = "Win32_Security_Cryptography"))]
impl ::core::marker::Copy for ROUTER_IKEv2_IF_CUSTOM_CONFIG2 {}
#[cfg(all(feature = "Win32_Networking_WinSock", feature = "Win32_Security_Cryptography"))]
impl ::core::clone::Clone for ROUTER_IKEv2_IF_CUSTOM_CONFIG2 {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(all(feature = "Win32_Networking_WinSock", feature = "Win32_Security_Cryptography"))]
impl ::core::fmt::Debug for ROUTER_IKEv2_IF_CUSTOM_CONFIG2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("ROUTER_IKEv2_IF_CUSTOM_CONFIG2").field("dwSaLifeTime", &self.dwSaLifeTime).field("dwSaDataSize", &self.dwSaDataSize).field("certificateName", &self.certificateName).field("customPolicy", &self.customPolicy).field("certificateHash", &self.certificateHash).field("dwMmSaLifeTime", &self.dwMmSaLifeTime).field("vpnTrafficSelectors", &self.vpnTrafficSelectors).finish()
    }
}
#[cfg(all(feature = "Win32_Networking_WinSock", feature = "Win32_Security_Cryptography"))]
unsafe impl ::windows::core::Abi for ROUTER_IKEv2_IF_CUSTOM_CONFIG2 {
    type Abi = Self;
}
#[cfg(all(feature = "Win32_Networking_WinSock", feature = "Win32_Security_Cryptography"))]
impl ::core::cmp::PartialEq for ROUTER_IKEv2_IF_CUSTOM_CONFIG2 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<ROUTER_IKEv2_IF_CUSTOM_CONFIG2>()) == 0 }
    }
}
#[cfg(all(feature = "Win32_Networking_WinSock", feature = "Win32_Security_Cryptography"))]
impl ::core::cmp::Eq for ROUTER_IKEv2_IF_CUSTOM_CONFIG2 {}
#[cfg(all(feature = "Win32_Networking_WinSock", feature = "Win32_Security_Cryptography"))]
impl ::core::default::Default for ROUTER_IKEv2_IF_CUSTOM_CONFIG2 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct ROUTER_INTERFACE_TYPE(pub i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`*"]
pub const ROUTER_IF_TYPE_CLIENT: ROUTER_INTERFACE_TYPE = ROUTER_INTERFACE_TYPE(0i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`*"]
pub const ROUTER_IF_TYPE_HOME_ROUTER: ROUTER_INTERFACE_TYPE = ROUTER_INTERFACE_TYPE(1i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`*"]
pub const ROUTER_IF_TYPE_FULL_ROUTER: ROUTER_INTERFACE_TYPE = ROUTER_INTERFACE_TYPE(2i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`*"]
pub const ROUTER_IF_TYPE_DEDICATED: ROUTER_INTERFACE_TYPE = ROUTER_INTERFACE_TYPE(3i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`*"]
pub const ROUTER_IF_TYPE_INTERNAL: ROUTER_INTERFACE_TYPE = ROUTER_INTERFACE_TYPE(4i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`*"]
pub const ROUTER_IF_TYPE_LOOPBACK: ROUTER_INTERFACE_TYPE = ROUTER_INTERFACE_TYPE(5i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`*"]
pub const ROUTER_IF_TYPE_TUNNEL1: ROUTER_INTERFACE_TYPE = ROUTER_INTERFACE_TYPE(6i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`*"]
pub const ROUTER_IF_TYPE_DIALOUT: ROUTER_INTERFACE_TYPE = ROUTER_INTERFACE_TYPE(7i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`*"]
pub const ROUTER_IF_TYPE_MAX: ROUTER_INTERFACE_TYPE = ROUTER_INTERFACE_TYPE(8i32);
impl ::core::marker::Copy for ROUTER_INTERFACE_TYPE {}
impl ::core::clone::Clone for ROUTER_INTERFACE_TYPE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for ROUTER_INTERFACE_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for ROUTER_INTERFACE_TYPE {
    type Abi = Self;
}
impl ::core::fmt::Debug for ROUTER_INTERFACE_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ROUTER_INTERFACE_TYPE").field(&self.0).finish()
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct ROUTING_PROTOCOL_CONFIG {
    pub dwCallbackFlags: u32,
    pub pfnRpfCallback: PMGM_RPF_CALLBACK,
    pub pfnCreationAlertCallback: PMGM_CREATION_ALERT_CALLBACK,
    pub pfnPruneAlertCallback: PMGM_PRUNE_ALERT_CALLBACK,
    pub pfnJoinAlertCallback: PMGM_JOIN_ALERT_CALLBACK,
    pub pfnWrongIfCallback: PMGM_WRONG_IF_CALLBACK,
    pub pfnLocalJoinCallback: PMGM_LOCAL_JOIN_CALLBACK,
    pub pfnLocalLeaveCallback: PMGM_LOCAL_LEAVE_CALLBACK,
    pub pfnDisableIgmpCallback: PMGM_DISABLE_IGMP_CALLBACK,
    pub pfnEnableIgmpCallback: PMGM_ENABLE_IGMP_CALLBACK,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for ROUTING_PROTOCOL_CONFIG {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for ROUTING_PROTOCOL_CONFIG {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for ROUTING_PROTOCOL_CONFIG {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("ROUTING_PROTOCOL_CONFIG")
            .field("dwCallbackFlags", &self.dwCallbackFlags)
            .field("pfnRpfCallback", &self.pfnRpfCallback.map(|f| f as usize))
            .field("pfnCreationAlertCallback", &self.pfnCreationAlertCallback.map(|f| f as usize))
            .field("pfnPruneAlertCallback", &self.pfnPruneAlertCallback.map(|f| f as usize))
            .field("pfnJoinAlertCallback", &self.pfnJoinAlertCallback.map(|f| f as usize))
            .field("pfnWrongIfCallback", &self.pfnWrongIfCallback.map(|f| f as usize))
            .field("pfnLocalJoinCallback", &self.pfnLocalJoinCallback.map(|f| f as usize))
            .field("pfnLocalLeaveCallback", &self.pfnLocalLeaveCallback.map(|f| f as usize))
            .field("pfnDisableIgmpCallback", &self.pfnDisableIgmpCallback.map(|f| f as usize))
            .field("pfnEnableIgmpCallback", &self.pfnEnableIgmpCallback.map(|f| f as usize))
            .finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for ROUTING_PROTOCOL_CONFIG {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for ROUTING_PROTOCOL_CONFIG {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<ROUTING_PROTOCOL_CONFIG>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for ROUTING_PROTOCOL_CONFIG {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for ROUTING_PROTOCOL_CONFIG {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`*"]
pub const RRAS_SERVICE_NAME: &str = "RemoteAccess";
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`*"]
pub const RTM_BLOCK_METHODS: u32 = 1u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`*"]
pub const RTM_CHANGE_TYPE_ALL: u32 = 1u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`*"]
pub const RTM_CHANGE_TYPE_BEST: u32 = 2u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`*"]
pub const RTM_CHANGE_TYPE_FORWARDING: u32 = 4u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`*"]
pub const RTM_DEST_FLAG_DONT_FORWARD: u32 = 4u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`*"]
pub const RTM_DEST_FLAG_FWD_ENGIN_ADD: u32 = 2u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`*"]
pub const RTM_DEST_FLAG_NATURAL_NET: u32 = 1u32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct RTM_DEST_INFO {
    pub DestHandle: isize,
    pub DestAddress: RTM_NET_ADDRESS,
    pub LastChanged: super::super::Foundation::FILETIME,
    pub BelongsToViews: u32,
    pub NumberOfViews: u32,
    pub ViewInfo: [RTM_DEST_INFO_0; 1],
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for RTM_DEST_INFO {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for RTM_DEST_INFO {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for RTM_DEST_INFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("RTM_DEST_INFO").field("DestHandle", &self.DestHandle).field("DestAddress", &self.DestAddress).field("LastChanged", &self.LastChanged).field("BelongsToViews", &self.BelongsToViews).field("NumberOfViews", &self.NumberOfViews).field("ViewInfo", &self.ViewInfo).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for RTM_DEST_INFO {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for RTM_DEST_INFO {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<RTM_DEST_INFO>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for RTM_DEST_INFO {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for RTM_DEST_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct RTM_DEST_INFO_0 {
    pub ViewId: i32,
    pub NumRoutes: u32,
    pub Route: isize,
    pub Owner: isize,
    pub DestFlags: u32,
    pub HoldRoute: isize,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for RTM_DEST_INFO_0 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for RTM_DEST_INFO_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for RTM_DEST_INFO_0 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("RTM_DEST_INFO_0").field("ViewId", &self.ViewId).field("NumRoutes", &self.NumRoutes).field("Route", &self.Route).field("Owner", &self.Owner).field("DestFlags", &self.DestFlags).field("HoldRoute", &self.HoldRoute).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for RTM_DEST_INFO_0 {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for RTM_DEST_INFO_0 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<RTM_DEST_INFO_0>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for RTM_DEST_INFO_0 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for RTM_DEST_INFO_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`*"]
pub type RTM_ENTITY_EXPORT_METHOD = ::core::option::Option<unsafe extern "system" fn(callerhandle: isize, calleehandle: isize, input: *mut RTM_ENTITY_METHOD_INPUT, output: *mut RTM_ENTITY_METHOD_OUTPUT)>;
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`*"]
pub struct RTM_ENTITY_EXPORT_METHODS {
    pub NumMethods: u32,
    pub Methods: [RTM_ENTITY_EXPORT_METHOD; 1],
}
impl ::core::marker::Copy for RTM_ENTITY_EXPORT_METHODS {}
impl ::core::clone::Clone for RTM_ENTITY_EXPORT_METHODS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for RTM_ENTITY_EXPORT_METHODS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("RTM_ENTITY_EXPORT_METHODS").field("NumMethods", &self.NumMethods).finish()
    }
}
unsafe impl ::windows::core::Abi for RTM_ENTITY_EXPORT_METHODS {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for RTM_ENTITY_EXPORT_METHODS {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<RTM_ENTITY_EXPORT_METHODS>()) == 0 }
    }
}
impl ::core::cmp::Eq for RTM_ENTITY_EXPORT_METHODS {}
impl ::core::default::Default for RTM_ENTITY_EXPORT_METHODS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`*"]
pub struct RTM_ENTITY_ID {
    pub Anonymous: RTM_ENTITY_ID_0,
}
impl ::core::marker::Copy for RTM_ENTITY_ID {}
impl ::core::clone::Clone for RTM_ENTITY_ID {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for RTM_ENTITY_ID {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for RTM_ENTITY_ID {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<RTM_ENTITY_ID>()) == 0 }
    }
}
impl ::core::cmp::Eq for RTM_ENTITY_ID {}
impl ::core::default::Default for RTM_ENTITY_ID {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`*"]
pub union RTM_ENTITY_ID_0 {
    pub Anonymous: RTM_ENTITY_ID_0_0,
    pub EntityId: u64,
}
impl ::core::marker::Copy for RTM_ENTITY_ID_0 {}
impl ::core::clone::Clone for RTM_ENTITY_ID_0 {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for RTM_ENTITY_ID_0 {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for RTM_ENTITY_ID_0 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<RTM_ENTITY_ID_0>()) == 0 }
    }
}
impl ::core::cmp::Eq for RTM_ENTITY_ID_0 {}
impl ::core::default::Default for RTM_ENTITY_ID_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`*"]
pub struct RTM_ENTITY_ID_0_0 {
    pub EntityProtocolId: u32,
    pub EntityInstanceId: u32,
}
impl ::core::marker::Copy for RTM_ENTITY_ID_0_0 {}
impl ::core::clone::Clone for RTM_ENTITY_ID_0_0 {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for RTM_ENTITY_ID_0_0 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("RTM_ENTITY_ID_0_0").field("EntityProtocolId", &self.EntityProtocolId).field("EntityInstanceId", &self.EntityInstanceId).finish()
    }
}
unsafe impl ::windows::core::Abi for RTM_ENTITY_ID_0_0 {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for RTM_ENTITY_ID_0_0 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<RTM_ENTITY_ID_0_0>()) == 0 }
    }
}
impl ::core::cmp::Eq for RTM_ENTITY_ID_0_0 {}
impl ::core::default::Default for RTM_ENTITY_ID_0_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`*"]
pub struct RTM_ENTITY_INFO {
    pub RtmInstanceId: u16,
    pub AddressFamily: u16,
    pub EntityId: RTM_ENTITY_ID,
}
impl ::core::marker::Copy for RTM_ENTITY_INFO {}
impl ::core::clone::Clone for RTM_ENTITY_INFO {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for RTM_ENTITY_INFO {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for RTM_ENTITY_INFO {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<RTM_ENTITY_INFO>()) == 0 }
    }
}
impl ::core::cmp::Eq for RTM_ENTITY_INFO {}
impl ::core::default::Default for RTM_ENTITY_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`*"]
pub struct RTM_ENTITY_METHOD_INPUT {
    pub MethodType: u32,
    pub InputSize: u32,
    pub InputData: [u8; 1],
}
impl ::core::marker::Copy for RTM_ENTITY_METHOD_INPUT {}
impl ::core::clone::Clone for RTM_ENTITY_METHOD_INPUT {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for RTM_ENTITY_METHOD_INPUT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("RTM_ENTITY_METHOD_INPUT").field("MethodType", &self.MethodType).field("InputSize", &self.InputSize).field("InputData", &self.InputData).finish()
    }
}
unsafe impl ::windows::core::Abi for RTM_ENTITY_METHOD_INPUT {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for RTM_ENTITY_METHOD_INPUT {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<RTM_ENTITY_METHOD_INPUT>()) == 0 }
    }
}
impl ::core::cmp::Eq for RTM_ENTITY_METHOD_INPUT {}
impl ::core::default::Default for RTM_ENTITY_METHOD_INPUT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`*"]
pub struct RTM_ENTITY_METHOD_OUTPUT {
    pub MethodType: u32,
    pub MethodStatus: u32,
    pub OutputSize: u32,
    pub OutputData: [u8; 1],
}
impl ::core::marker::Copy for RTM_ENTITY_METHOD_OUTPUT {}
impl ::core::clone::Clone for RTM_ENTITY_METHOD_OUTPUT {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for RTM_ENTITY_METHOD_OUTPUT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("RTM_ENTITY_METHOD_OUTPUT").field("MethodType", &self.MethodType).field("MethodStatus", &self.MethodStatus).field("OutputSize", &self.OutputSize).field("OutputData", &self.OutputData).finish()
    }
}
unsafe impl ::windows::core::Abi for RTM_ENTITY_METHOD_OUTPUT {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for RTM_ENTITY_METHOD_OUTPUT {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<RTM_ENTITY_METHOD_OUTPUT>()) == 0 }
    }
}
impl ::core::cmp::Eq for RTM_ENTITY_METHOD_OUTPUT {}
impl ::core::default::Default for RTM_ENTITY_METHOD_OUTPUT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`*"]
pub const RTM_ENUM_ALL_DESTS: u32 = 0u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`*"]
pub const RTM_ENUM_ALL_ROUTES: u32 = 0u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`*"]
pub const RTM_ENUM_NEXT: u32 = 1u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`*"]
pub const RTM_ENUM_OWN_DESTS: u32 = 16777216u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`*"]
pub const RTM_ENUM_OWN_ROUTES: u32 = 65536u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`*"]
pub const RTM_ENUM_RANGE: u32 = 2u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`*"]
pub const RTM_ENUM_START: u32 = 0u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`*"]
pub type RTM_EVENT_CALLBACK = ::core::option::Option<unsafe extern "system" fn(rtmreghandle: isize, eventtype: RTM_EVENT_TYPE, context1: *mut ::core::ffi::c_void, context2: *mut ::core::ffi::c_void) -> u32>;
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct RTM_EVENT_TYPE(pub i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`*"]
pub const RTM_ENTITY_REGISTERED: RTM_EVENT_TYPE = RTM_EVENT_TYPE(0i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`*"]
pub const RTM_ENTITY_DEREGISTERED: RTM_EVENT_TYPE = RTM_EVENT_TYPE(1i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`*"]
pub const RTM_ROUTE_EXPIRED: RTM_EVENT_TYPE = RTM_EVENT_TYPE(2i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`*"]
pub const RTM_CHANGE_NOTIFICATION: RTM_EVENT_TYPE = RTM_EVENT_TYPE(3i32);
impl ::core::marker::Copy for RTM_EVENT_TYPE {}
impl ::core::clone::Clone for RTM_EVENT_TYPE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for RTM_EVENT_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for RTM_EVENT_TYPE {
    type Abi = Self;
}
impl ::core::fmt::Debug for RTM_EVENT_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("RTM_EVENT_TYPE").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`*"]
pub const RTM_MATCH_FULL: u32 = 65535u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`*"]
pub const RTM_MATCH_INTERFACE: u32 = 16u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`*"]
pub const RTM_MATCH_NEIGHBOUR: u32 = 2u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`*"]
pub const RTM_MATCH_NEXTHOP: u32 = 8u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`*"]
pub const RTM_MATCH_NONE: u32 = 0u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`*"]
pub const RTM_MATCH_OWNER: u32 = 1u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`*"]
pub const RTM_MATCH_PREF: u32 = 4u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`*"]
pub const RTM_MAX_ADDRESS_SIZE: u32 = 16u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`*"]
pub const RTM_MAX_VIEWS: u32 = 32u32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`*"]
pub struct RTM_NET_ADDRESS {
    pub AddressFamily: u16,
    pub NumBits: u16,
    pub AddrBits: [u8; 16],
}
impl ::core::marker::Copy for RTM_NET_ADDRESS {}
impl ::core::clone::Clone for RTM_NET_ADDRESS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for RTM_NET_ADDRESS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("RTM_NET_ADDRESS").field("AddressFamily", &self.AddressFamily).field("NumBits", &self.NumBits).field("AddrBits", &self.AddrBits).finish()
    }
}
unsafe impl ::windows::core::Abi for RTM_NET_ADDRESS {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for RTM_NET_ADDRESS {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<RTM_NET_ADDRESS>()) == 0 }
    }
}
impl ::core::cmp::Eq for RTM_NET_ADDRESS {}
impl ::core::default::Default for RTM_NET_ADDRESS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`*"]
pub const RTM_NEXTHOP_CHANGE_NEW: u32 = 1u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`*"]
pub const RTM_NEXTHOP_FLAGS_DOWN: u32 = 2u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`*"]
pub const RTM_NEXTHOP_FLAGS_REMOTE: u32 = 1u32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`*"]
pub struct RTM_NEXTHOP_INFO {
    pub NextHopAddress: RTM_NET_ADDRESS,
    pub NextHopOwner: isize,
    pub InterfaceIndex: u32,
    pub State: u16,
    pub Flags: u16,
    pub EntitySpecificInfo: *mut ::core::ffi::c_void,
    pub RemoteNextHop: isize,
}
impl ::core::marker::Copy for RTM_NEXTHOP_INFO {}
impl ::core::clone::Clone for RTM_NEXTHOP_INFO {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for RTM_NEXTHOP_INFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("RTM_NEXTHOP_INFO").field("NextHopAddress", &self.NextHopAddress).field("NextHopOwner", &self.NextHopOwner).field("InterfaceIndex", &self.InterfaceIndex).field("State", &self.State).field("Flags", &self.Flags).field("EntitySpecificInfo", &self.EntitySpecificInfo).field("RemoteNextHop", &self.RemoteNextHop).finish()
    }
}
unsafe impl ::windows::core::Abi for RTM_NEXTHOP_INFO {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for RTM_NEXTHOP_INFO {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<RTM_NEXTHOP_INFO>()) == 0 }
    }
}
impl ::core::cmp::Eq for RTM_NEXTHOP_INFO {}
impl ::core::default::Default for RTM_NEXTHOP_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`*"]
pub struct RTM_NEXTHOP_LIST {
    pub NumNextHops: u16,
    pub NextHops: [isize; 1],
}
impl ::core::marker::Copy for RTM_NEXTHOP_LIST {}
impl ::core::clone::Clone for RTM_NEXTHOP_LIST {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for RTM_NEXTHOP_LIST {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("RTM_NEXTHOP_LIST").field("NumNextHops", &self.NumNextHops).field("NextHops", &self.NextHops).finish()
    }
}
unsafe impl ::windows::core::Abi for RTM_NEXTHOP_LIST {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for RTM_NEXTHOP_LIST {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<RTM_NEXTHOP_LIST>()) == 0 }
    }
}
impl ::core::cmp::Eq for RTM_NEXTHOP_LIST {}
impl ::core::default::Default for RTM_NEXTHOP_LIST {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`*"]
pub const RTM_NEXTHOP_STATE_CREATED: u32 = 0u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`*"]
pub const RTM_NEXTHOP_STATE_DELETED: u32 = 1u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`*"]
pub const RTM_NOTIFY_ONLY_MARKED_DESTS: u32 = 65536u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`*"]
pub const RTM_NUM_CHANGE_TYPES: u32 = 3u32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`*"]
pub struct RTM_PREF_INFO {
    pub Metric: u32,
    pub Preference: u32,
}
impl ::core::marker::Copy for RTM_PREF_INFO {}
impl ::core::clone::Clone for RTM_PREF_INFO {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for RTM_PREF_INFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("RTM_PREF_INFO").field("Metric", &self.Metric).field("Preference", &self.Preference).finish()
    }
}
unsafe impl ::windows::core::Abi for RTM_PREF_INFO {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for RTM_PREF_INFO {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<RTM_PREF_INFO>()) == 0 }
    }
}
impl ::core::cmp::Eq for RTM_PREF_INFO {}
impl ::core::default::Default for RTM_PREF_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`*"]
pub struct RTM_REGN_PROFILE {
    pub MaxNextHopsInRoute: u32,
    pub MaxHandlesInEnum: u32,
    pub ViewsSupported: u32,
    pub NumberOfViews: u32,
}
impl ::core::marker::Copy for RTM_REGN_PROFILE {}
impl ::core::clone::Clone for RTM_REGN_PROFILE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for RTM_REGN_PROFILE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("RTM_REGN_PROFILE").field("MaxNextHopsInRoute", &self.MaxNextHopsInRoute).field("MaxHandlesInEnum", &self.MaxHandlesInEnum).field("ViewsSupported", &self.ViewsSupported).field("NumberOfViews", &self.NumberOfViews).finish()
    }
}
unsafe impl ::windows::core::Abi for RTM_REGN_PROFILE {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for RTM_REGN_PROFILE {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<RTM_REGN_PROFILE>()) == 0 }
    }
}
impl ::core::cmp::Eq for RTM_REGN_PROFILE {}
impl ::core::default::Default for RTM_REGN_PROFILE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`*"]
pub const RTM_RESUME_METHODS: u32 = 0u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`*"]
pub const RTM_ROUTE_CHANGE_BEST: u32 = 65536u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`*"]
pub const RTM_ROUTE_CHANGE_FIRST: u32 = 1u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`*"]
pub const RTM_ROUTE_CHANGE_NEW: u32 = 2u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`*"]
pub const RTM_ROUTE_FLAGS_BLACKHOLE: u32 = 2u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`*"]
pub const RTM_ROUTE_FLAGS_DISCARD: u32 = 4u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`*"]
pub const RTM_ROUTE_FLAGS_INACTIVE: u32 = 8u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`*"]
pub const RTM_ROUTE_FLAGS_LIMITED_BC: u32 = 1024u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`*"]
pub const RTM_ROUTE_FLAGS_LOCAL: u32 = 16u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`*"]
pub const RTM_ROUTE_FLAGS_LOCAL_MCAST: u32 = 512u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`*"]
pub const RTM_ROUTE_FLAGS_LOOPBACK: u32 = 128u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`*"]
pub const RTM_ROUTE_FLAGS_MARTIAN: u32 = 1u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`*"]
pub const RTM_ROUTE_FLAGS_MCAST: u32 = 256u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`*"]
pub const RTM_ROUTE_FLAGS_MYSELF: u32 = 64u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`*"]
pub const RTM_ROUTE_FLAGS_ONES_NETBC: u32 = 16384u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`*"]
pub const RTM_ROUTE_FLAGS_ONES_SUBNETBC: u32 = 32768u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`*"]
pub const RTM_ROUTE_FLAGS_REMOTE: u32 = 32u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`*"]
pub const RTM_ROUTE_FLAGS_ZEROS_NETBC: u32 = 4096u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`*"]
pub const RTM_ROUTE_FLAGS_ZEROS_SUBNETBC: u32 = 8192u32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`*"]
pub struct RTM_ROUTE_INFO {
    pub DestHandle: isize,
    pub RouteOwner: isize,
    pub Neighbour: isize,
    pub State: u8,
    pub Flags1: u8,
    pub Flags: u16,
    pub PrefInfo: RTM_PREF_INFO,
    pub BelongsToViews: u32,
    pub EntitySpecificInfo: *mut ::core::ffi::c_void,
    pub NextHopsList: RTM_NEXTHOP_LIST,
}
impl ::core::marker::Copy for RTM_ROUTE_INFO {}
impl ::core::clone::Clone for RTM_ROUTE_INFO {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for RTM_ROUTE_INFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("RTM_ROUTE_INFO").field("DestHandle", &self.DestHandle).field("RouteOwner", &self.RouteOwner).field("Neighbour", &self.Neighbour).field("State", &self.State).field("Flags1", &self.Flags1).field("Flags", &self.Flags).field("PrefInfo", &self.PrefInfo).field("BelongsToViews", &self.BelongsToViews).field("EntitySpecificInfo", &self.EntitySpecificInfo).field("NextHopsList", &self.NextHopsList).finish()
    }
}
unsafe impl ::windows::core::Abi for RTM_ROUTE_INFO {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for RTM_ROUTE_INFO {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<RTM_ROUTE_INFO>()) == 0 }
    }
}
impl ::core::cmp::Eq for RTM_ROUTE_INFO {}
impl ::core::default::Default for RTM_ROUTE_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`*"]
pub const RTM_ROUTE_STATE_CREATED: u32 = 0u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`*"]
pub const RTM_ROUTE_STATE_DELETED: u32 = 2u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`*"]
pub const RTM_ROUTE_STATE_DELETING: u32 = 1u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`*"]
pub const RTM_VIEW_ID_MCAST: u32 = 1u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`*"]
pub const RTM_VIEW_ID_UCAST: u32 = 0u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`*"]
pub const RTM_VIEW_MASK_ALL: u32 = 4294967295u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`*"]
pub const RTM_VIEW_MASK_ANY: u32 = 0u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`*"]
pub const RTM_VIEW_MASK_MCAST: u32 = 2u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`*"]
pub const RTM_VIEW_MASK_NONE: u32 = 0u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`*"]
pub const RTM_VIEW_MASK_SIZE: u32 = 32u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`*"]
pub const RTM_VIEW_MASK_UCAST: u32 = 1u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`*"]
#[inline]
pub unsafe fn RasClearConnectionStatistics<'a, P0>(hrasconn: P0) -> u32
where
    P0: ::std::convert::Into<HRASCONN>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn RasClearConnectionStatistics(hrasconn: HRASCONN) -> u32;
    }
    RasClearConnectionStatistics(hrasconn.into())
}
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`*"]
#[inline]
pub unsafe fn RasClearLinkStatistics<'a, P0>(hrasconn: P0, dwsubentry: u32) -> u32
where
    P0: ::std::convert::Into<HRASCONN>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn RasClearLinkStatistics(hrasconn: HRASCONN, dwsubentry: u32) -> u32;
    }
    RasClearLinkStatistics(hrasconn.into(), dwsubentry)
}
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn RasConnectionNotificationA<'a, P0, P1>(param0: P0, param1: P1, param2: u32) -> u32
where
    P0: ::std::convert::Into<HRASCONN>,
    P1: ::std::convert::Into<super::super::Foundation::HANDLE>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn RasConnectionNotificationA(param0: HRASCONN, param1: super::super::Foundation::HANDLE, param2: u32) -> u32;
    }
    RasConnectionNotificationA(param0.into(), param1.into(), param2)
}
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn RasConnectionNotificationW<'a, P0, P1>(param0: P0, param1: P1, param2: u32) -> u32
where
    P0: ::std::convert::Into<HRASCONN>,
    P1: ::std::convert::Into<super::super::Foundation::HANDLE>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn RasConnectionNotificationW(param0: HRASCONN, param1: super::super::Foundation::HANDLE, param2: u32) -> u32;
    }
    RasConnectionNotificationW(param0.into(), param1.into(), param2)
}
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn RasCreatePhonebookEntryA<'a, P0, P1>(param0: P0, param1: P1) -> u32
where
    P0: ::std::convert::Into<super::super::Foundation::HWND>,
    P1: ::std::convert::Into<::windows::core::PCSTR>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn RasCreatePhonebookEntryA(param0: super::super::Foundation::HWND, param1: ::windows::core::PCSTR) -> u32;
    }
    RasCreatePhonebookEntryA(param0.into(), param1.into())
}
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn RasCreatePhonebookEntryW<'a, P0, P1>(param0: P0, param1: P1) -> u32
where
    P0: ::std::convert::Into<super::super::Foundation::HWND>,
    P1: ::std::convert::Into<::windows::core::PCWSTR>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn RasCreatePhonebookEntryW(param0: super::super::Foundation::HWND, param1: ::windows::core::PCWSTR) -> u32;
    }
    RasCreatePhonebookEntryW(param0.into(), param1.into())
}
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`*"]
pub type RasCustomDeleteEntryNotifyFn = ::core::option::Option<unsafe extern "system" fn(lpszphonebook: ::windows::core::PCWSTR, lpszentry: ::windows::core::PCWSTR, dwflags: u32) -> u32>;
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub type RasCustomDialDlgFn = ::core::option::Option<unsafe extern "system" fn(hinstdll: super::super::Foundation::HINSTANCE, dwflags: u32, lpszphonebook: ::windows::core::PCWSTR, lpszentry: ::windows::core::PCWSTR, lpszphonenumber: ::windows::core::PCWSTR, lpinfo: *mut RASDIALDLG, pvinfo: *mut ::core::ffi::c_void) -> super::super::Foundation::BOOL>;
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub type RasCustomDialFn = ::core::option::Option<unsafe extern "system" fn(hinstdll: super::super::Foundation::HINSTANCE, lprasdialextensions: *mut RASDIALEXTENSIONS, lpszphonebook: ::windows::core::PCWSTR, lprasdialparams: *mut RASDIALPARAMSA, dwnotifiertype: u32, lpvnotifier: *mut ::core::ffi::c_void, lphrasconn: *mut HRASCONN, dwflags: u32) -> u32>;
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub type RasCustomEntryDlgFn = ::core::option::Option<unsafe extern "system" fn(hinstdll: super::super::Foundation::HINSTANCE, lpszphonebook: ::windows::core::PCWSTR, lpszentry: ::windows::core::PCWSTR, lpinfo: *mut RASENTRYDLGA, dwflags: u32) -> super::super::Foundation::BOOL>;
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`*"]
pub type RasCustomHangUpFn = ::core::option::Option<unsafe extern "system" fn(hrasconn: HRASCONN) -> u32>;
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub type RasCustomScriptExecuteFn = ::core::option::Option<unsafe extern "system" fn(hport: super::super::Foundation::HANDLE, lpszphonebook: ::windows::core::PCWSTR, lpszentryname: ::windows::core::PCWSTR, pfnrasgetbuffer: PFNRASGETBUFFER, pfnrasfreebuffer: PFNRASFREEBUFFER, pfnrassendbuffer: PFNRASSENDBUFFER, pfnrasreceivebuffer: PFNRASRECEIVEBUFFER, pfnrasretrievebuffer: PFNRASRETRIEVEBUFFER, hwnd: super::super::Foundation::HWND, prasdialparams: *mut RASDIALPARAMSA, pvreserved: *mut ::core::ffi::c_void) -> u32>;
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`*"]
#[inline]
pub unsafe fn RasDeleteEntryA<'a, P0, P1>(param0: P0, param1: P1) -> u32
where
    P0: ::std::convert::Into<::windows::core::PCSTR>,
    P1: ::std::convert::Into<::windows::core::PCSTR>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn RasDeleteEntryA(param0: ::windows::core::PCSTR, param1: ::windows::core::PCSTR) -> u32;
    }
    RasDeleteEntryA(param0.into(), param1.into())
}
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`*"]
#[inline]
pub unsafe fn RasDeleteEntryW<'a, P0, P1>(param0: P0, param1: P1) -> u32
where
    P0: ::std::convert::Into<::windows::core::PCWSTR>,
    P1: ::std::convert::Into<::windows::core::PCWSTR>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn RasDeleteEntryW(param0: ::windows::core::PCWSTR, param1: ::windows::core::PCWSTR) -> u32;
    }
    RasDeleteEntryW(param0.into(), param1.into())
}
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`*"]
#[inline]
pub unsafe fn RasDeleteSubEntryA<'a, P0, P1>(pszphonebook: P0, pszentry: P1, dwsubentryid: u32) -> u32
where
    P0: ::std::convert::Into<::windows::core::PCSTR>,
    P1: ::std::convert::Into<::windows::core::PCSTR>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn RasDeleteSubEntryA(pszphonebook: ::windows::core::PCSTR, pszentry: ::windows::core::PCSTR, dwsubentryid: u32) -> u32;
    }
    RasDeleteSubEntryA(pszphonebook.into(), pszentry.into(), dwsubentryid)
}
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`*"]
#[inline]
pub unsafe fn RasDeleteSubEntryW<'a, P0, P1>(pszphonebook: P0, pszentry: P1, dwsubentryid: u32) -> u32
where
    P0: ::std::convert::Into<::windows::core::PCWSTR>,
    P1: ::std::convert::Into<::windows::core::PCWSTR>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn RasDeleteSubEntryW(pszphonebook: ::windows::core::PCWSTR, pszentry: ::windows::core::PCWSTR, dwsubentryid: u32) -> u32;
    }
    RasDeleteSubEntryW(pszphonebook.into(), pszentry.into(), dwsubentryid)
}
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn RasDialA<'a, P0>(param0: *const RASDIALEXTENSIONS, param1: P0, param2: *const RASDIALPARAMSA, param3: u32, param4: *const ::core::ffi::c_void, param5: *mut HRASCONN) -> u32
where
    P0: ::std::convert::Into<::windows::core::PCSTR>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn RasDialA(param0: *const RASDIALEXTENSIONS, param1: ::windows::core::PCSTR, param2: *const RASDIALPARAMSA, param3: u32, param4: *const ::core::ffi::c_void, param5: *mut HRASCONN) -> u32;
    }
    RasDialA(::core::mem::transmute(param0), param1.into(), ::core::mem::transmute(param2), param3, ::core::mem::transmute(param4), ::core::mem::transmute(param5))
}
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn RasDialDlgA<'a, P0, P1, P2>(lpszphonebook: P0, lpszentry: P1, lpszphonenumber: P2, lpinfo: *mut RASDIALDLG) -> super::super::Foundation::BOOL
where
    P0: ::std::convert::Into<::windows::core::PCSTR>,
    P1: ::std::convert::Into<::windows::core::PCSTR>,
    P2: ::std::convert::Into<::windows::core::PCSTR>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn RasDialDlgA(lpszphonebook: ::windows::core::PCSTR, lpszentry: ::windows::core::PCSTR, lpszphonenumber: ::windows::core::PCSTR, lpinfo: *mut RASDIALDLG) -> super::super::Foundation::BOOL;
    }
    RasDialDlgA(lpszphonebook.into(), lpszentry.into(), lpszphonenumber.into(), ::core::mem::transmute(lpinfo))
}
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn RasDialDlgW<'a, P0, P1, P2>(lpszphonebook: P0, lpszentry: P1, lpszphonenumber: P2, lpinfo: *mut RASDIALDLG) -> super::super::Foundation::BOOL
where
    P0: ::std::convert::Into<::windows::core::PCWSTR>,
    P1: ::std::convert::Into<::windows::core::PCWSTR>,
    P2: ::std::convert::Into<::windows::core::PCWSTR>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn RasDialDlgW(lpszphonebook: ::windows::core::PCWSTR, lpszentry: ::windows::core::PCWSTR, lpszphonenumber: ::windows::core::PCWSTR, lpinfo: *mut RASDIALDLG) -> super::super::Foundation::BOOL;
    }
    RasDialDlgW(lpszphonebook.into(), lpszentry.into(), lpszphonenumber.into(), ::core::mem::transmute(lpinfo))
}
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn RasDialW<'a, P0>(param0: *const RASDIALEXTENSIONS, param1: P0, param2: *const RASDIALPARAMSW, param3: u32, param4: *const ::core::ffi::c_void, param5: *mut HRASCONN) -> u32
where
    P0: ::std::convert::Into<::windows::core::PCWSTR>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn RasDialW(param0: *const RASDIALEXTENSIONS, param1: ::windows::core::PCWSTR, param2: *const RASDIALPARAMSW, param3: u32, param4: *const ::core::ffi::c_void, param5: *mut HRASCONN) -> u32;
    }
    RasDialW(::core::mem::transmute(param0), param1.into(), ::core::mem::transmute(param2), param3, ::core::mem::transmute(param4), ::core::mem::transmute(param5))
}
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn RasEditPhonebookEntryA<'a, P0, P1, P2>(param0: P0, param1: P1, param2: P2) -> u32
where
    P0: ::std::convert::Into<super::super::Foundation::HWND>,
    P1: ::std::convert::Into<::windows::core::PCSTR>,
    P2: ::std::convert::Into<::windows::core::PCSTR>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn RasEditPhonebookEntryA(param0: super::super::Foundation::HWND, param1: ::windows::core::PCSTR, param2: ::windows::core::PCSTR) -> u32;
    }
    RasEditPhonebookEntryA(param0.into(), param1.into(), param2.into())
}
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn RasEditPhonebookEntryW<'a, P0, P1, P2>(param0: P0, param1: P1, param2: P2) -> u32
where
    P0: ::std::convert::Into<super::super::Foundation::HWND>,
    P1: ::std::convert::Into<::windows::core::PCWSTR>,
    P2: ::std::convert::Into<::windows::core::PCWSTR>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn RasEditPhonebookEntryW(param0: super::super::Foundation::HWND, param1: ::windows::core::PCWSTR, param2: ::windows::core::PCWSTR) -> u32;
    }
    RasEditPhonebookEntryW(param0.into(), param1.into(), param2.into())
}
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn RasEntryDlgA<'a, P0, P1>(lpszphonebook: P0, lpszentry: P1, lpinfo: *mut RASENTRYDLGA) -> super::super::Foundation::BOOL
where
    P0: ::std::convert::Into<::windows::core::PCSTR>,
    P1: ::std::convert::Into<::windows::core::PCSTR>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn RasEntryDlgA(lpszphonebook: ::windows::core::PCSTR, lpszentry: ::windows::core::PCSTR, lpinfo: *mut RASENTRYDLGA) -> super::super::Foundation::BOOL;
    }
    RasEntryDlgA(lpszphonebook.into(), lpszentry.into(), ::core::mem::transmute(lpinfo))
}
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn RasEntryDlgW<'a, P0, P1>(lpszphonebook: P0, lpszentry: P1, lpinfo: *mut RASENTRYDLGW) -> super::super::Foundation::BOOL
where
    P0: ::std::convert::Into<::windows::core::PCWSTR>,
    P1: ::std::convert::Into<::windows::core::PCWSTR>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn RasEntryDlgW(lpszphonebook: ::windows::core::PCWSTR, lpszentry: ::windows::core::PCWSTR, lpinfo: *mut RASENTRYDLGW) -> super::super::Foundation::BOOL;
    }
    RasEntryDlgW(lpszphonebook.into(), lpszentry.into(), ::core::mem::transmute(lpinfo))
}
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`*"]
#[inline]
pub unsafe fn RasEnumAutodialAddressesA(lpprasautodialaddresses: *mut ::windows::core::PSTR, lpdwcbrasautodialaddresses: *mut u32, lpdwcrasautodialaddresses: *mut u32) -> u32 {
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn RasEnumAutodialAddressesA(lpprasautodialaddresses: *mut ::windows::core::PSTR, lpdwcbrasautodialaddresses: *mut u32, lpdwcrasautodialaddresses: *mut u32) -> u32;
    }
    RasEnumAutodialAddressesA(::core::mem::transmute(lpprasautodialaddresses), ::core::mem::transmute(lpdwcbrasautodialaddresses), ::core::mem::transmute(lpdwcrasautodialaddresses))
}
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`*"]
#[inline]
pub unsafe fn RasEnumAutodialAddressesW(lpprasautodialaddresses: *mut ::windows::core::PWSTR, lpdwcbrasautodialaddresses: *mut u32, lpdwcrasautodialaddresses: *mut u32) -> u32 {
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn RasEnumAutodialAddressesW(lpprasautodialaddresses: *mut ::windows::core::PWSTR, lpdwcbrasautodialaddresses: *mut u32, lpdwcrasautodialaddresses: *mut u32) -> u32;
    }
    RasEnumAutodialAddressesW(::core::mem::transmute(lpprasautodialaddresses), ::core::mem::transmute(lpdwcbrasautodialaddresses), ::core::mem::transmute(lpdwcrasautodialaddresses))
}
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn RasEnumConnectionsA(param0: *mut RASCONNA, param1: *mut u32, param2: *mut u32) -> u32 {
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn RasEnumConnectionsA(param0: *mut RASCONNA, param1: *mut u32, param2: *mut u32) -> u32;
    }
    RasEnumConnectionsA(::core::mem::transmute(param0), ::core::mem::transmute(param1), ::core::mem::transmute(param2))
}
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn RasEnumConnectionsW(param0: *mut RASCONNW, param1: *mut u32, param2: *mut u32) -> u32 {
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn RasEnumConnectionsW(param0: *mut RASCONNW, param1: *mut u32, param2: *mut u32) -> u32;
    }
    RasEnumConnectionsW(::core::mem::transmute(param0), ::core::mem::transmute(param1), ::core::mem::transmute(param2))
}
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn RasEnumDevicesA(param0: *mut RASDEVINFOA, param1: *mut u32, param2: *mut u32) -> u32 {
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn RasEnumDevicesA(param0: *mut RASDEVINFOA, param1: *mut u32, param2: *mut u32) -> u32;
    }
    RasEnumDevicesA(::core::mem::transmute(param0), ::core::mem::transmute(param1), ::core::mem::transmute(param2))
}
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`*"]
#[inline]
pub unsafe fn RasEnumDevicesW(param0: *mut RASDEVINFOW, param1: *mut u32, param2: *mut u32) -> u32 {
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn RasEnumDevicesW(param0: *mut RASDEVINFOW, param1: *mut u32, param2: *mut u32) -> u32;
    }
    RasEnumDevicesW(::core::mem::transmute(param0), ::core::mem::transmute(param1), ::core::mem::transmute(param2))
}
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn RasEnumEntriesA<'a, P0, P1>(param0: P0, param1: P1, param2: *mut RASENTRYNAMEA, param3: *mut u32, param4: *mut u32) -> u32
where
    P0: ::std::convert::Into<::windows::core::PCSTR>,
    P1: ::std::convert::Into<::windows::core::PCSTR>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn RasEnumEntriesA(param0: ::windows::core::PCSTR, param1: ::windows::core::PCSTR, param2: *mut RASENTRYNAMEA, param3: *mut u32, param4: *mut u32) -> u32;
    }
    RasEnumEntriesA(param0.into(), param1.into(), ::core::mem::transmute(param2), ::core::mem::transmute(param3), ::core::mem::transmute(param4))
}
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`*"]
#[inline]
pub unsafe fn RasEnumEntriesW<'a, P0, P1>(param0: P0, param1: P1, param2: *mut RASENTRYNAMEW, param3: *mut u32, param4: *mut u32) -> u32
where
    P0: ::std::convert::Into<::windows::core::PCWSTR>,
    P1: ::std::convert::Into<::windows::core::PCWSTR>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn RasEnumEntriesW(param0: ::windows::core::PCWSTR, param1: ::windows::core::PCWSTR, param2: *mut RASENTRYNAMEW, param3: *mut u32, param4: *mut u32) -> u32;
    }
    RasEnumEntriesW(param0.into(), param1.into(), ::core::mem::transmute(param2), ::core::mem::transmute(param3), ::core::mem::transmute(param4))
}
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn RasFreeEapUserIdentityA(praseapuseridentity: *const RASEAPUSERIDENTITYA) {
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn RasFreeEapUserIdentityA(praseapuseridentity: *const RASEAPUSERIDENTITYA);
    }
    RasFreeEapUserIdentityA(::core::mem::transmute(praseapuseridentity))
}
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`*"]
#[inline]
pub unsafe fn RasFreeEapUserIdentityW(praseapuseridentity: *const RASEAPUSERIDENTITYW) {
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn RasFreeEapUserIdentityW(praseapuseridentity: *const RASEAPUSERIDENTITYW);
    }
    RasFreeEapUserIdentityW(::core::mem::transmute(praseapuseridentity))
}
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn RasGetAutodialAddressA<'a, P0>(param0: P0, param1: *const u32, param2: *mut RASAUTODIALENTRYA, param3: *mut u32, param4: *mut u32) -> u32
where
    P0: ::std::convert::Into<::windows::core::PCSTR>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn RasGetAutodialAddressA(param0: ::windows::core::PCSTR, param1: *const u32, param2: *mut RASAUTODIALENTRYA, param3: *mut u32, param4: *mut u32) -> u32;
    }
    RasGetAutodialAddressA(param0.into(), ::core::mem::transmute(param1), ::core::mem::transmute(param2), ::core::mem::transmute(param3), ::core::mem::transmute(param4))
}
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`*"]
#[inline]
pub unsafe fn RasGetAutodialAddressW<'a, P0>(param0: P0, param1: *const u32, param2: *mut RASAUTODIALENTRYW, param3: *mut u32, param4: *mut u32) -> u32
where
    P0: ::std::convert::Into<::windows::core::PCWSTR>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn RasGetAutodialAddressW(param0: ::windows::core::PCWSTR, param1: *const u32, param2: *mut RASAUTODIALENTRYW, param3: *mut u32, param4: *mut u32) -> u32;
    }
    RasGetAutodialAddressW(param0.into(), ::core::mem::transmute(param1), ::core::mem::transmute(param2), ::core::mem::transmute(param3), ::core::mem::transmute(param4))
}
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`*"]
#[inline]
pub unsafe fn RasGetAutodialEnableA(param0: u32, param1: *mut i32) -> u32 {
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn RasGetAutodialEnableA(param0: u32, param1: *mut i32) -> u32;
    }
    RasGetAutodialEnableA(param0, ::core::mem::transmute(param1))
}
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`*"]
#[inline]
pub unsafe fn RasGetAutodialEnableW(param0: u32, param1: *mut i32) -> u32 {
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn RasGetAutodialEnableW(param0: u32, param1: *mut i32) -> u32;
    }
    RasGetAutodialEnableW(param0, ::core::mem::transmute(param1))
}
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`*"]
#[inline]
pub unsafe fn RasGetAutodialParamA(param0: u32, param1: *mut ::core::ffi::c_void, param2: *mut u32) -> u32 {
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn RasGetAutodialParamA(param0: u32, param1: *mut ::core::ffi::c_void, param2: *mut u32) -> u32;
    }
    RasGetAutodialParamA(param0, ::core::mem::transmute(param1), ::core::mem::transmute(param2))
}
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`*"]
#[inline]
pub unsafe fn RasGetAutodialParamW(param0: u32, param1: *mut ::core::ffi::c_void, param2: *mut u32) -> u32 {
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn RasGetAutodialParamW(param0: u32, param1: *mut ::core::ffi::c_void, param2: *mut u32) -> u32;
    }
    RasGetAutodialParamW(param0, ::core::mem::transmute(param1), ::core::mem::transmute(param2))
}
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`, `\"Win32_Foundation\"`, `\"Win32_Networking_WinSock\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
#[inline]
pub unsafe fn RasGetConnectStatusA<'a, P0>(param0: P0, param1: *mut RASCONNSTATUSA) -> u32
where
    P0: ::std::convert::Into<HRASCONN>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn RasGetConnectStatusA(param0: HRASCONN, param1: *mut RASCONNSTATUSA) -> u32;
    }
    RasGetConnectStatusA(param0.into(), ::core::mem::transmute(param1))
}
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`, `\"Win32_Networking_WinSock\"`*"]
#[cfg(feature = "Win32_Networking_WinSock")]
#[inline]
pub unsafe fn RasGetConnectStatusW<'a, P0>(param0: P0, param1: *mut RASCONNSTATUSW) -> u32
where
    P0: ::std::convert::Into<HRASCONN>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn RasGetConnectStatusW(param0: HRASCONN, param1: *mut RASCONNSTATUSW) -> u32;
    }
    RasGetConnectStatusW(param0.into(), ::core::mem::transmute(param1))
}
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`*"]
#[inline]
pub unsafe fn RasGetConnectionStatistics<'a, P0>(hrasconn: P0, lpstatistics: *mut RAS_STATS) -> u32
where
    P0: ::std::convert::Into<HRASCONN>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn RasGetConnectionStatistics(hrasconn: HRASCONN, lpstatistics: *mut RAS_STATS) -> u32;
    }
    RasGetConnectionStatistics(hrasconn.into(), ::core::mem::transmute(lpstatistics))
}
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`*"]
#[inline]
pub unsafe fn RasGetCountryInfoA(param0: *mut RASCTRYINFO, param1: *mut u32) -> u32 {
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn RasGetCountryInfoA(param0: *mut RASCTRYINFO, param1: *mut u32) -> u32;
    }
    RasGetCountryInfoA(::core::mem::transmute(param0), ::core::mem::transmute(param1))
}
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`*"]
#[inline]
pub unsafe fn RasGetCountryInfoW(param0: *mut RASCTRYINFO, param1: *mut u32) -> u32 {
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn RasGetCountryInfoW(param0: *mut RASCTRYINFO, param1: *mut u32) -> u32;
    }
    RasGetCountryInfoW(::core::mem::transmute(param0), ::core::mem::transmute(param1))
}
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn RasGetCredentialsA<'a, P0, P1>(param0: P0, param1: P1, param2: *mut RASCREDENTIALSA) -> u32
where
    P0: ::std::convert::Into<::windows::core::PCSTR>,
    P1: ::std::convert::Into<::windows::core::PCSTR>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn RasGetCredentialsA(param0: ::windows::core::PCSTR, param1: ::windows::core::PCSTR, param2: *mut RASCREDENTIALSA) -> u32;
    }
    RasGetCredentialsA(param0.into(), param1.into(), ::core::mem::transmute(param2))
}
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`*"]
#[inline]
pub unsafe fn RasGetCredentialsW<'a, P0, P1>(param0: P0, param1: P1, param2: *mut RASCREDENTIALSW) -> u32
where
    P0: ::std::convert::Into<::windows::core::PCWSTR>,
    P1: ::std::convert::Into<::windows::core::PCWSTR>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn RasGetCredentialsW(param0: ::windows::core::PCWSTR, param1: ::windows::core::PCWSTR, param2: *mut RASCREDENTIALSW) -> u32;
    }
    RasGetCredentialsW(param0.into(), param1.into(), ::core::mem::transmute(param2))
}
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`*"]
#[inline]
pub unsafe fn RasGetCustomAuthDataA<'a, P0, P1>(pszphonebook: P0, pszentry: P1, pbcustomauthdata: *mut u8, pdwsizeofcustomauthdata: *mut u32) -> u32
where
    P0: ::std::convert::Into<::windows::core::PCSTR>,
    P1: ::std::convert::Into<::windows::core::PCSTR>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn RasGetCustomAuthDataA(pszphonebook: ::windows::core::PCSTR, pszentry: ::windows::core::PCSTR, pbcustomauthdata: *mut u8, pdwsizeofcustomauthdata: *mut u32) -> u32;
    }
    RasGetCustomAuthDataA(pszphonebook.into(), pszentry.into(), ::core::mem::transmute(pbcustomauthdata), ::core::mem::transmute(pdwsizeofcustomauthdata))
}
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`*"]
#[inline]
pub unsafe fn RasGetCustomAuthDataW<'a, P0, P1>(pszphonebook: P0, pszentry: P1, pbcustomauthdata: *mut u8, pdwsizeofcustomauthdata: *mut u32) -> u32
where
    P0: ::std::convert::Into<::windows::core::PCWSTR>,
    P1: ::std::convert::Into<::windows::core::PCWSTR>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn RasGetCustomAuthDataW(pszphonebook: ::windows::core::PCWSTR, pszentry: ::windows::core::PCWSTR, pbcustomauthdata: *mut u8, pdwsizeofcustomauthdata: *mut u32) -> u32;
    }
    RasGetCustomAuthDataW(pszphonebook.into(), pszentry.into(), ::core::mem::transmute(pbcustomauthdata), ::core::mem::transmute(pdwsizeofcustomauthdata))
}
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn RasGetEapUserDataA<'a, P0, P1, P2>(htoken: P0, pszphonebook: P1, pszentry: P2, pbeapdata: *mut u8, pdwsizeofeapdata: *mut u32) -> u32
where
    P0: ::std::convert::Into<super::super::Foundation::HANDLE>,
    P1: ::std::convert::Into<::windows::core::PCSTR>,
    P2: ::std::convert::Into<::windows::core::PCSTR>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn RasGetEapUserDataA(htoken: super::super::Foundation::HANDLE, pszphonebook: ::windows::core::PCSTR, pszentry: ::windows::core::PCSTR, pbeapdata: *mut u8, pdwsizeofeapdata: *mut u32) -> u32;
    }
    RasGetEapUserDataA(htoken.into(), pszphonebook.into(), pszentry.into(), ::core::mem::transmute(pbeapdata), ::core::mem::transmute(pdwsizeofeapdata))
}
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn RasGetEapUserDataW<'a, P0, P1, P2>(htoken: P0, pszphonebook: P1, pszentry: P2, pbeapdata: *mut u8, pdwsizeofeapdata: *mut u32) -> u32
where
    P0: ::std::convert::Into<super::super::Foundation::HANDLE>,
    P1: ::std::convert::Into<::windows::core::PCWSTR>,
    P2: ::std::convert::Into<::windows::core::PCWSTR>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn RasGetEapUserDataW(htoken: super::super::Foundation::HANDLE, pszphonebook: ::windows::core::PCWSTR, pszentry: ::windows::core::PCWSTR, pbeapdata: *mut u8, pdwsizeofeapdata: *mut u32) -> u32;
    }
    RasGetEapUserDataW(htoken.into(), pszphonebook.into(), pszentry.into(), ::core::mem::transmute(pbeapdata), ::core::mem::transmute(pdwsizeofeapdata))
}
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn RasGetEapUserIdentityA<'a, P0, P1, P2>(pszphonebook: P0, pszentry: P1, dwflags: u32, hwnd: P2, ppraseapuseridentity: *mut *mut RASEAPUSERIDENTITYA) -> u32
where
    P0: ::std::convert::Into<::windows::core::PCSTR>,
    P1: ::std::convert::Into<::windows::core::PCSTR>,
    P2: ::std::convert::Into<super::super::Foundation::HWND>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn RasGetEapUserIdentityA(pszphonebook: ::windows::core::PCSTR, pszentry: ::windows::core::PCSTR, dwflags: u32, hwnd: super::super::Foundation::HWND, ppraseapuseridentity: *mut *mut RASEAPUSERIDENTITYA) -> u32;
    }
    RasGetEapUserIdentityA(pszphonebook.into(), pszentry.into(), dwflags, hwnd.into(), ::core::mem::transmute(ppraseapuseridentity))
}
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn RasGetEapUserIdentityW<'a, P0, P1, P2>(pszphonebook: P0, pszentry: P1, dwflags: u32, hwnd: P2, ppraseapuseridentity: *mut *mut RASEAPUSERIDENTITYW) -> u32
where
    P0: ::std::convert::Into<::windows::core::PCWSTR>,
    P1: ::std::convert::Into<::windows::core::PCWSTR>,
    P2: ::std::convert::Into<super::super::Foundation::HWND>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn RasGetEapUserIdentityW(pszphonebook: ::windows::core::PCWSTR, pszentry: ::windows::core::PCWSTR, dwflags: u32, hwnd: super::super::Foundation::HWND, ppraseapuseridentity: *mut *mut RASEAPUSERIDENTITYW) -> u32;
    }
    RasGetEapUserIdentityW(pszphonebook.into(), pszentry.into(), dwflags, hwnd.into(), ::core::mem::transmute(ppraseapuseridentity))
}
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn RasGetEntryDialParamsA<'a, P0>(param0: P0, param1: *mut RASDIALPARAMSA, param2: *mut i32) -> u32
where
    P0: ::std::convert::Into<::windows::core::PCSTR>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn RasGetEntryDialParamsA(param0: ::windows::core::PCSTR, param1: *mut RASDIALPARAMSA, param2: *mut i32) -> u32;
    }
    RasGetEntryDialParamsA(param0.into(), ::core::mem::transmute(param1), ::core::mem::transmute(param2))
}
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`*"]
#[inline]
pub unsafe fn RasGetEntryDialParamsW<'a, P0>(param0: P0, param1: *mut RASDIALPARAMSW, param2: *mut i32) -> u32
where
    P0: ::std::convert::Into<::windows::core::PCWSTR>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn RasGetEntryDialParamsW(param0: ::windows::core::PCWSTR, param1: *mut RASDIALPARAMSW, param2: *mut i32) -> u32;
    }
    RasGetEntryDialParamsW(param0.into(), ::core::mem::transmute(param1), ::core::mem::transmute(param2))
}
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`, `\"Win32_Foundation\"`, `\"Win32_Networking_WinSock\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
#[inline]
pub unsafe fn RasGetEntryPropertiesA<'a, P0, P1>(param0: P0, param1: P1, param2: *mut RASENTRYA, param3: *mut u32, param4: *mut u8, param5: *mut u32) -> u32
where
    P0: ::std::convert::Into<::windows::core::PCSTR>,
    P1: ::std::convert::Into<::windows::core::PCSTR>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn RasGetEntryPropertiesA(param0: ::windows::core::PCSTR, param1: ::windows::core::PCSTR, param2: *mut RASENTRYA, param3: *mut u32, param4: *mut u8, param5: *mut u32) -> u32;
    }
    RasGetEntryPropertiesA(param0.into(), param1.into(), ::core::mem::transmute(param2), ::core::mem::transmute(param3), ::core::mem::transmute(param4), ::core::mem::transmute(param5))
}
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`, `\"Win32_Foundation\"`, `\"Win32_Networking_WinSock\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
#[inline]
pub unsafe fn RasGetEntryPropertiesW<'a, P0, P1>(param0: P0, param1: P1, param2: *mut RASENTRYW, param3: *mut u32, param4: *mut u8, param5: *mut u32) -> u32
where
    P0: ::std::convert::Into<::windows::core::PCWSTR>,
    P1: ::std::convert::Into<::windows::core::PCWSTR>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn RasGetEntryPropertiesW(param0: ::windows::core::PCWSTR, param1: ::windows::core::PCWSTR, param2: *mut RASENTRYW, param3: *mut u32, param4: *mut u8, param5: *mut u32) -> u32;
    }
    RasGetEntryPropertiesW(param0.into(), param1.into(), ::core::mem::transmute(param2), ::core::mem::transmute(param3), ::core::mem::transmute(param4), ::core::mem::transmute(param5))
}
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`*"]
#[inline]
pub unsafe fn RasGetErrorStringA(resourceid: u32, lpszstring: &mut [u8]) -> u32 {
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn RasGetErrorStringA(resourceid: u32, lpszstring: ::windows::core::PSTR, inbufsize: u32) -> u32;
    }
    RasGetErrorStringA(resourceid, ::core::mem::transmute(::windows::core::as_mut_ptr_or_null(lpszstring)), lpszstring.len() as _)
}
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`*"]
#[inline]
pub unsafe fn RasGetErrorStringW(resourceid: u32, lpszstring: &mut [u16]) -> u32 {
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn RasGetErrorStringW(resourceid: u32, lpszstring: ::windows::core::PWSTR, inbufsize: u32) -> u32;
    }
    RasGetErrorStringW(resourceid, ::core::mem::transmute(::windows::core::as_mut_ptr_or_null(lpszstring)), lpszstring.len() as _)
}
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`*"]
#[inline]
pub unsafe fn RasGetLinkStatistics<'a, P0>(hrasconn: P0, dwsubentry: u32, lpstatistics: *mut RAS_STATS) -> u32
where
    P0: ::std::convert::Into<HRASCONN>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn RasGetLinkStatistics(hrasconn: HRASCONN, dwsubentry: u32, lpstatistics: *mut RAS_STATS) -> u32;
    }
    RasGetLinkStatistics(hrasconn.into(), dwsubentry, ::core::mem::transmute(lpstatistics))
}
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`*"]
#[inline]
pub unsafe fn RasGetPCscf(lpszpcscf: ::windows::core::PWSTR) -> u32 {
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn RasGetPCscf(lpszpcscf: ::windows::core::PWSTR) -> u32;
    }
    RasGetPCscf(::core::mem::transmute(lpszpcscf))
}
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`*"]
#[inline]
pub unsafe fn RasGetProjectionInfoA<'a, P0>(param0: P0, param1: RASPROJECTION, param2: *mut ::core::ffi::c_void, param3: *mut u32) -> u32
where
    P0: ::std::convert::Into<HRASCONN>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn RasGetProjectionInfoA(param0: HRASCONN, param1: RASPROJECTION, param2: *mut ::core::ffi::c_void, param3: *mut u32) -> u32;
    }
    RasGetProjectionInfoA(param0.into(), param1, ::core::mem::transmute(param2), ::core::mem::transmute(param3))
}
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`, `\"Win32_Foundation\"`, `\"Win32_Networking_WinSock\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
#[inline]
pub unsafe fn RasGetProjectionInfoEx<'a, P0>(hrasconn: P0, prasprojection: *mut RAS_PROJECTION_INFO, lpdwsize: *mut u32) -> u32
where
    P0: ::std::convert::Into<HRASCONN>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn RasGetProjectionInfoEx(hrasconn: HRASCONN, prasprojection: *mut RAS_PROJECTION_INFO, lpdwsize: *mut u32) -> u32;
    }
    RasGetProjectionInfoEx(hrasconn.into(), ::core::mem::transmute(prasprojection), ::core::mem::transmute(lpdwsize))
}
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`*"]
#[inline]
pub unsafe fn RasGetProjectionInfoW<'a, P0>(param0: P0, param1: RASPROJECTION, param2: *mut ::core::ffi::c_void, param3: *mut u32) -> u32
where
    P0: ::std::convert::Into<HRASCONN>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn RasGetProjectionInfoW(param0: HRASCONN, param1: RASPROJECTION, param2: *mut ::core::ffi::c_void, param3: *mut u32) -> u32;
    }
    RasGetProjectionInfoW(param0.into(), param1, ::core::mem::transmute(param2), ::core::mem::transmute(param3))
}
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`*"]
#[inline]
pub unsafe fn RasGetSubEntryHandleA<'a, P0>(param0: P0, param1: u32, param2: *mut HRASCONN) -> u32
where
    P0: ::std::convert::Into<HRASCONN>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn RasGetSubEntryHandleA(param0: HRASCONN, param1: u32, param2: *mut HRASCONN) -> u32;
    }
    RasGetSubEntryHandleA(param0.into(), param1, ::core::mem::transmute(param2))
}
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`*"]
#[inline]
pub unsafe fn RasGetSubEntryHandleW<'a, P0>(param0: P0, param1: u32, param2: *mut HRASCONN) -> u32
where
    P0: ::std::convert::Into<HRASCONN>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn RasGetSubEntryHandleW(param0: HRASCONN, param1: u32, param2: *mut HRASCONN) -> u32;
    }
    RasGetSubEntryHandleW(param0.into(), param1, ::core::mem::transmute(param2))
}
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn RasGetSubEntryPropertiesA<'a, P0, P1>(param0: P0, param1: P1, param2: u32, param3: *mut RASSUBENTRYA, param4: *mut u32, param5: *mut u8, param6: *mut u32) -> u32
where
    P0: ::std::convert::Into<::windows::core::PCSTR>,
    P1: ::std::convert::Into<::windows::core::PCSTR>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn RasGetSubEntryPropertiesA(param0: ::windows::core::PCSTR, param1: ::windows::core::PCSTR, param2: u32, param3: *mut RASSUBENTRYA, param4: *mut u32, param5: *mut u8, param6: *mut u32) -> u32;
    }
    RasGetSubEntryPropertiesA(param0.into(), param1.into(), param2, ::core::mem::transmute(param3), ::core::mem::transmute(param4), ::core::mem::transmute(param5), ::core::mem::transmute(param6))
}
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`*"]
#[inline]
pub unsafe fn RasGetSubEntryPropertiesW<'a, P0, P1>(param0: P0, param1: P1, param2: u32, param3: *mut RASSUBENTRYW, param4: *mut u32, param5: *mut u8, param6: *mut u32) -> u32
where
    P0: ::std::convert::Into<::windows::core::PCWSTR>,
    P1: ::std::convert::Into<::windows::core::PCWSTR>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn RasGetSubEntryPropertiesW(param0: ::windows::core::PCWSTR, param1: ::windows::core::PCWSTR, param2: u32, param3: *mut RASSUBENTRYW, param4: *mut u32, param5: *mut u8, param6: *mut u32) -> u32;
    }
    RasGetSubEntryPropertiesW(param0.into(), param1.into(), param2, ::core::mem::transmute(param3), ::core::mem::transmute(param4), ::core::mem::transmute(param5), ::core::mem::transmute(param6))
}
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`*"]
#[inline]
pub unsafe fn RasHangUpA<'a, P0>(param0: P0) -> u32
where
    P0: ::std::convert::Into<HRASCONN>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn RasHangUpA(param0: HRASCONN) -> u32;
    }
    RasHangUpA(param0.into())
}
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`*"]
#[inline]
pub unsafe fn RasHangUpW<'a, P0>(param0: P0) -> u32
where
    P0: ::std::convert::Into<HRASCONN>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn RasHangUpW(param0: HRASCONN) -> u32;
    }
    RasHangUpW(param0.into())
}
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn RasInvokeEapUI<'a, P0, P1>(param0: P0, param1: u32, param2: *const RASDIALEXTENSIONS, param3: P1) -> u32
where
    P0: ::std::convert::Into<HRASCONN>,
    P1: ::std::convert::Into<super::super::Foundation::HWND>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn RasInvokeEapUI(param0: HRASCONN, param1: u32, param2: *const RASDIALEXTENSIONS, param3: super::super::Foundation::HWND) -> u32;
    }
    RasInvokeEapUI(param0.into(), param1, ::core::mem::transmute(param2), param3.into())
}
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn RasPhonebookDlgA<'a, P0, P1>(lpszphonebook: P0, lpszentry: P1, lpinfo: *mut RASPBDLGA) -> super::super::Foundation::BOOL
where
    P0: ::std::convert::Into<::windows::core::PCSTR>,
    P1: ::std::convert::Into<::windows::core::PCSTR>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn RasPhonebookDlgA(lpszphonebook: ::windows::core::PCSTR, lpszentry: ::windows::core::PCSTR, lpinfo: *mut RASPBDLGA) -> super::super::Foundation::BOOL;
    }
    RasPhonebookDlgA(lpszphonebook.into(), lpszentry.into(), ::core::mem::transmute(lpinfo))
}
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn RasPhonebookDlgW<'a, P0, P1>(lpszphonebook: P0, lpszentry: P1, lpinfo: *mut RASPBDLGW) -> super::super::Foundation::BOOL
where
    P0: ::std::convert::Into<::windows::core::PCWSTR>,
    P1: ::std::convert::Into<::windows::core::PCWSTR>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn RasPhonebookDlgW(lpszphonebook: ::windows::core::PCWSTR, lpszentry: ::windows::core::PCWSTR, lpinfo: *mut RASPBDLGW) -> super::super::Foundation::BOOL;
    }
    RasPhonebookDlgW(lpszphonebook.into(), lpszentry.into(), ::core::mem::transmute(lpinfo))
}
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`*"]
#[inline]
pub unsafe fn RasRenameEntryA<'a, P0, P1, P2>(param0: P0, param1: P1, param2: P2) -> u32
where
    P0: ::std::convert::Into<::windows::core::PCSTR>,
    P1: ::std::convert::Into<::windows::core::PCSTR>,
    P2: ::std::convert::Into<::windows::core::PCSTR>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn RasRenameEntryA(param0: ::windows::core::PCSTR, param1: ::windows::core::PCSTR, param2: ::windows::core::PCSTR) -> u32;
    }
    RasRenameEntryA(param0.into(), param1.into(), param2.into())
}
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`*"]
#[inline]
pub unsafe fn RasRenameEntryW<'a, P0, P1, P2>(param0: P0, param1: P1, param2: P2) -> u32
where
    P0: ::std::convert::Into<::windows::core::PCWSTR>,
    P1: ::std::convert::Into<::windows::core::PCWSTR>,
    P2: ::std::convert::Into<::windows::core::PCWSTR>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn RasRenameEntryW(param0: ::windows::core::PCWSTR, param1: ::windows::core::PCWSTR, param2: ::windows::core::PCWSTR) -> u32;
    }
    RasRenameEntryW(param0.into(), param1.into(), param2.into())
}
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn RasSetAutodialAddressA<'a, P0>(param0: P0, param1: u32, param2: *const RASAUTODIALENTRYA, param3: u32, param4: u32) -> u32
where
    P0: ::std::convert::Into<::windows::core::PCSTR>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn RasSetAutodialAddressA(param0: ::windows::core::PCSTR, param1: u32, param2: *const RASAUTODIALENTRYA, param3: u32, param4: u32) -> u32;
    }
    RasSetAutodialAddressA(param0.into(), param1, ::core::mem::transmute(param2), param3, param4)
}
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`*"]
#[inline]
pub unsafe fn RasSetAutodialAddressW<'a, P0>(param0: P0, param1: u32, param2: *const RASAUTODIALENTRYW, param3: u32, param4: u32) -> u32
where
    P0: ::std::convert::Into<::windows::core::PCWSTR>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn RasSetAutodialAddressW(param0: ::windows::core::PCWSTR, param1: u32, param2: *const RASAUTODIALENTRYW, param3: u32, param4: u32) -> u32;
    }
    RasSetAutodialAddressW(param0.into(), param1, ::core::mem::transmute(param2), param3, param4)
}
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn RasSetAutodialEnableA<'a, P0>(param0: u32, param1: P0) -> u32
where
    P0: ::std::convert::Into<super::super::Foundation::BOOL>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn RasSetAutodialEnableA(param0: u32, param1: super::super::Foundation::BOOL) -> u32;
    }
    RasSetAutodialEnableA(param0, param1.into())
}
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn RasSetAutodialEnableW<'a, P0>(param0: u32, param1: P0) -> u32
where
    P0: ::std::convert::Into<super::super::Foundation::BOOL>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn RasSetAutodialEnableW(param0: u32, param1: super::super::Foundation::BOOL) -> u32;
    }
    RasSetAutodialEnableW(param0, param1.into())
}
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`*"]
#[inline]
pub unsafe fn RasSetAutodialParamA(param0: u32, param1: *const ::core::ffi::c_void, param2: u32) -> u32 {
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn RasSetAutodialParamA(param0: u32, param1: *const ::core::ffi::c_void, param2: u32) -> u32;
    }
    RasSetAutodialParamA(param0, ::core::mem::transmute(param1), param2)
}
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`*"]
#[inline]
pub unsafe fn RasSetAutodialParamW(param0: u32, param1: *const ::core::ffi::c_void, param2: u32) -> u32 {
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn RasSetAutodialParamW(param0: u32, param1: *const ::core::ffi::c_void, param2: u32) -> u32;
    }
    RasSetAutodialParamW(param0, ::core::mem::transmute(param1), param2)
}
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn RasSetCredentialsA<'a, P0, P1, P2>(param0: P0, param1: P1, param2: *const RASCREDENTIALSA, param3: P2) -> u32
where
    P0: ::std::convert::Into<::windows::core::PCSTR>,
    P1: ::std::convert::Into<::windows::core::PCSTR>,
    P2: ::std::convert::Into<super::super::Foundation::BOOL>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn RasSetCredentialsA(param0: ::windows::core::PCSTR, param1: ::windows::core::PCSTR, param2: *const RASCREDENTIALSA, param3: super::super::Foundation::BOOL) -> u32;
    }
    RasSetCredentialsA(param0.into(), param1.into(), ::core::mem::transmute(param2), param3.into())
}
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn RasSetCredentialsW<'a, P0, P1, P2>(param0: P0, param1: P1, param2: *const RASCREDENTIALSW, param3: P2) -> u32
where
    P0: ::std::convert::Into<::windows::core::PCWSTR>,
    P1: ::std::convert::Into<::windows::core::PCWSTR>,
    P2: ::std::convert::Into<super::super::Foundation::BOOL>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn RasSetCredentialsW(param0: ::windows::core::PCWSTR, param1: ::windows::core::PCWSTR, param2: *const RASCREDENTIALSW, param3: super::super::Foundation::BOOL) -> u32;
    }
    RasSetCredentialsW(param0.into(), param1.into(), ::core::mem::transmute(param2), param3.into())
}
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`*"]
#[inline]
pub unsafe fn RasSetCustomAuthDataA<'a, P0, P1>(pszphonebook: P0, pszentry: P1, pbcustomauthdata: *const u8, dwsizeofcustomauthdata: u32) -> u32
where
    P0: ::std::convert::Into<::windows::core::PCSTR>,
    P1: ::std::convert::Into<::windows::core::PCSTR>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn RasSetCustomAuthDataA(pszphonebook: ::windows::core::PCSTR, pszentry: ::windows::core::PCSTR, pbcustomauthdata: *const u8, dwsizeofcustomauthdata: u32) -> u32;
    }
    RasSetCustomAuthDataA(pszphonebook.into(), pszentry.into(), ::core::mem::transmute(pbcustomauthdata), dwsizeofcustomauthdata)
}
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`*"]
#[inline]
pub unsafe fn RasSetCustomAuthDataW<'a, P0, P1>(pszphonebook: P0, pszentry: P1, pbcustomauthdata: *const u8, dwsizeofcustomauthdata: u32) -> u32
where
    P0: ::std::convert::Into<::windows::core::PCWSTR>,
    P1: ::std::convert::Into<::windows::core::PCWSTR>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn RasSetCustomAuthDataW(pszphonebook: ::windows::core::PCWSTR, pszentry: ::windows::core::PCWSTR, pbcustomauthdata: *const u8, dwsizeofcustomauthdata: u32) -> u32;
    }
    RasSetCustomAuthDataW(pszphonebook.into(), pszentry.into(), ::core::mem::transmute(pbcustomauthdata), dwsizeofcustomauthdata)
}
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn RasSetEapUserDataA<'a, P0, P1, P2>(htoken: P0, pszphonebook: P1, pszentry: P2, pbeapdata: *const u8, dwsizeofeapdata: u32) -> u32
where
    P0: ::std::convert::Into<super::super::Foundation::HANDLE>,
    P1: ::std::convert::Into<::windows::core::PCSTR>,
    P2: ::std::convert::Into<::windows::core::PCSTR>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn RasSetEapUserDataA(htoken: super::super::Foundation::HANDLE, pszphonebook: ::windows::core::PCSTR, pszentry: ::windows::core::PCSTR, pbeapdata: *const u8, dwsizeofeapdata: u32) -> u32;
    }
    RasSetEapUserDataA(htoken.into(), pszphonebook.into(), pszentry.into(), ::core::mem::transmute(pbeapdata), dwsizeofeapdata)
}
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn RasSetEapUserDataW<'a, P0, P1, P2>(htoken: P0, pszphonebook: P1, pszentry: P2, pbeapdata: *const u8, dwsizeofeapdata: u32) -> u32
where
    P0: ::std::convert::Into<super::super::Foundation::HANDLE>,
    P1: ::std::convert::Into<::windows::core::PCWSTR>,
    P2: ::std::convert::Into<::windows::core::PCWSTR>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn RasSetEapUserDataW(htoken: super::super::Foundation::HANDLE, pszphonebook: ::windows::core::PCWSTR, pszentry: ::windows::core::PCWSTR, pbeapdata: *const u8, dwsizeofeapdata: u32) -> u32;
    }
    RasSetEapUserDataW(htoken.into(), pszphonebook.into(), pszentry.into(), ::core::mem::transmute(pbeapdata), dwsizeofeapdata)
}
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn RasSetEntryDialParamsA<'a, P0, P1>(param0: P0, param1: *const RASDIALPARAMSA, param2: P1) -> u32
where
    P0: ::std::convert::Into<::windows::core::PCSTR>,
    P1: ::std::convert::Into<super::super::Foundation::BOOL>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn RasSetEntryDialParamsA(param0: ::windows::core::PCSTR, param1: *const RASDIALPARAMSA, param2: super::super::Foundation::BOOL) -> u32;
    }
    RasSetEntryDialParamsA(param0.into(), ::core::mem::transmute(param1), param2.into())
}
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn RasSetEntryDialParamsW<'a, P0, P1>(param0: P0, param1: *const RASDIALPARAMSW, param2: P1) -> u32
where
    P0: ::std::convert::Into<::windows::core::PCWSTR>,
    P1: ::std::convert::Into<super::super::Foundation::BOOL>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn RasSetEntryDialParamsW(param0: ::windows::core::PCWSTR, param1: *const RASDIALPARAMSW, param2: super::super::Foundation::BOOL) -> u32;
    }
    RasSetEntryDialParamsW(param0.into(), ::core::mem::transmute(param1), param2.into())
}
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`, `\"Win32_Foundation\"`, `\"Win32_Networking_WinSock\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
#[inline]
pub unsafe fn RasSetEntryPropertiesA<'a, P0, P1>(param0: P0, param1: P1, param2: *const RASENTRYA, param3: u32, param4: *const u8, param5: u32) -> u32
where
    P0: ::std::convert::Into<::windows::core::PCSTR>,
    P1: ::std::convert::Into<::windows::core::PCSTR>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn RasSetEntryPropertiesA(param0: ::windows::core::PCSTR, param1: ::windows::core::PCSTR, param2: *const RASENTRYA, param3: u32, param4: *const u8, param5: u32) -> u32;
    }
    RasSetEntryPropertiesA(param0.into(), param1.into(), ::core::mem::transmute(param2), param3, ::core::mem::transmute(param4), param5)
}
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`, `\"Win32_Foundation\"`, `\"Win32_Networking_WinSock\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
#[inline]
pub unsafe fn RasSetEntryPropertiesW<'a, P0, P1>(param0: P0, param1: P1, param2: *const RASENTRYW, param3: u32, param4: *const u8, param5: u32) -> u32
where
    P0: ::std::convert::Into<::windows::core::PCWSTR>,
    P1: ::std::convert::Into<::windows::core::PCWSTR>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn RasSetEntryPropertiesW(param0: ::windows::core::PCWSTR, param1: ::windows::core::PCWSTR, param2: *const RASENTRYW, param3: u32, param4: *const u8, param5: u32) -> u32;
    }
    RasSetEntryPropertiesW(param0.into(), param1.into(), ::core::mem::transmute(param2), param3, ::core::mem::transmute(param4), param5)
}
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn RasSetSubEntryPropertiesA<'a, P0, P1>(param0: P0, param1: P1, param2: u32, param3: *const RASSUBENTRYA, param4: u32, param5: *const u8, param6: u32) -> u32
where
    P0: ::std::convert::Into<::windows::core::PCSTR>,
    P1: ::std::convert::Into<::windows::core::PCSTR>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn RasSetSubEntryPropertiesA(param0: ::windows::core::PCSTR, param1: ::windows::core::PCSTR, param2: u32, param3: *const RASSUBENTRYA, param4: u32, param5: *const u8, param6: u32) -> u32;
    }
    RasSetSubEntryPropertiesA(param0.into(), param1.into(), param2, ::core::mem::transmute(param3), param4, ::core::mem::transmute(param5), param6)
}
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`*"]
#[inline]
pub unsafe fn RasSetSubEntryPropertiesW<'a, P0, P1>(param0: P0, param1: P1, param2: u32, param3: *const RASSUBENTRYW, param4: u32, param5: *const u8, param6: u32) -> u32
where
    P0: ::std::convert::Into<::windows::core::PCWSTR>,
    P1: ::std::convert::Into<::windows::core::PCWSTR>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn RasSetSubEntryPropertiesW(param0: ::windows::core::PCWSTR, param1: ::windows::core::PCWSTR, param2: u32, param3: *const RASSUBENTRYW, param4: u32, param5: *const u8, param6: u32) -> u32;
    }
    RasSetSubEntryPropertiesW(param0.into(), param1.into(), param2, ::core::mem::transmute(param3), param4, ::core::mem::transmute(param5), param6)
}
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`, `\"Win32_Networking_WinSock\"`*"]
#[cfg(feature = "Win32_Networking_WinSock")]
#[inline]
pub unsafe fn RasUpdateConnection<'a, P0>(hrasconn: P0, lprasupdateconn: *const RASUPDATECONN) -> u32
where
    P0: ::std::convert::Into<HRASCONN>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn RasUpdateConnection(hrasconn: HRASCONN, lprasupdateconn: *const RASUPDATECONN) -> u32;
    }
    RasUpdateConnection(hrasconn.into(), ::core::mem::transmute(lprasupdateconn))
}
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`*"]
#[inline]
pub unsafe fn RasValidateEntryNameA<'a, P0, P1>(param0: P0, param1: P1) -> u32
where
    P0: ::std::convert::Into<::windows::core::PCSTR>,
    P1: ::std::convert::Into<::windows::core::PCSTR>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn RasValidateEntryNameA(param0: ::windows::core::PCSTR, param1: ::windows::core::PCSTR) -> u32;
    }
    RasValidateEntryNameA(param0.into(), param1.into())
}
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`*"]
#[inline]
pub unsafe fn RasValidateEntryNameW<'a, P0, P1>(param0: P0, param1: P1) -> u32
where
    P0: ::std::convert::Into<::windows::core::PCWSTR>,
    P1: ::std::convert::Into<::windows::core::PCWSTR>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn RasValidateEntryNameW(param0: ::windows::core::PCWSTR, param1: ::windows::core::PCWSTR) -> u32;
    }
    RasValidateEntryNameW(param0.into(), param1.into())
}
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`*"]
#[inline]
pub unsafe fn RtmAddNextHop(rtmreghandle: isize, nexthopinfo: *mut RTM_NEXTHOP_INFO, nexthophandle: *mut isize, changeflags: *mut u32) -> u32 {
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn RtmAddNextHop(rtmreghandle: isize, nexthopinfo: *mut RTM_NEXTHOP_INFO, nexthophandle: *mut isize, changeflags: *mut u32) -> u32;
    }
    RtmAddNextHop(rtmreghandle, ::core::mem::transmute(nexthopinfo), ::core::mem::transmute(nexthophandle), ::core::mem::transmute(changeflags))
}
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`*"]
#[inline]
pub unsafe fn RtmAddRouteToDest(rtmreghandle: isize, routehandle: *mut isize, destaddress: *mut RTM_NET_ADDRESS, routeinfo: *mut RTM_ROUTE_INFO, timetolive: u32, routelisthandle: isize, notifytype: u32, notifyhandle: isize, changeflags: *mut u32) -> u32 {
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn RtmAddRouteToDest(rtmreghandle: isize, routehandle: *mut isize, destaddress: *mut RTM_NET_ADDRESS, routeinfo: *mut RTM_ROUTE_INFO, timetolive: u32, routelisthandle: isize, notifytype: u32, notifyhandle: isize, changeflags: *mut u32) -> u32;
    }
    RtmAddRouteToDest(rtmreghandle, ::core::mem::transmute(routehandle), ::core::mem::transmute(destaddress), ::core::mem::transmute(routeinfo), timetolive, routelisthandle, notifytype, notifyhandle, ::core::mem::transmute(changeflags))
}
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn RtmBlockMethods<'a, P0>(rtmreghandle: isize, targethandle: P0, targettype: u8, blockingflag: u32) -> u32
where
    P0: ::std::convert::Into<super::super::Foundation::HANDLE>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn RtmBlockMethods(rtmreghandle: isize, targethandle: super::super::Foundation::HANDLE, targettype: u8, blockingflag: u32) -> u32;
    }
    RtmBlockMethods(rtmreghandle, targethandle.into(), targettype, blockingflag)
}
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`, `\"Win32_Networking_WinSock\"`*"]
#[cfg(feature = "Win32_Networking_WinSock")]
#[inline]
pub unsafe fn RtmConvertIpv6AddressAndLengthToNetAddress(pnetaddress: *mut RTM_NET_ADDRESS, address: super::super::Networking::WinSock::IN6_ADDR, dwlength: u32, dwaddresssize: u32) -> u32 {
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn RtmConvertIpv6AddressAndLengthToNetAddress(pnetaddress: *mut RTM_NET_ADDRESS, address: super::super::Networking::WinSock::IN6_ADDR, dwlength: u32, dwaddresssize: u32) -> u32;
    }
    RtmConvertIpv6AddressAndLengthToNetAddress(::core::mem::transmute(pnetaddress), ::core::mem::transmute(address), dwlength, dwaddresssize)
}
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`, `\"Win32_Networking_WinSock\"`*"]
#[cfg(feature = "Win32_Networking_WinSock")]
#[inline]
pub unsafe fn RtmConvertNetAddressToIpv6AddressAndLength(pnetaddress: *mut RTM_NET_ADDRESS, paddress: *mut super::super::Networking::WinSock::IN6_ADDR, plength: *mut u32, dwaddresssize: u32) -> u32 {
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn RtmConvertNetAddressToIpv6AddressAndLength(pnetaddress: *mut RTM_NET_ADDRESS, paddress: *mut super::super::Networking::WinSock::IN6_ADDR, plength: *mut u32, dwaddresssize: u32) -> u32;
    }
    RtmConvertNetAddressToIpv6AddressAndLength(::core::mem::transmute(pnetaddress), ::core::mem::transmute(paddress), ::core::mem::transmute(plength), dwaddresssize)
}
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`*"]
#[inline]
pub unsafe fn RtmCreateDestEnum(rtmreghandle: isize, targetviews: u32, enumflags: u32, netaddress: *mut RTM_NET_ADDRESS, protocolid: u32, rtmenumhandle: *mut isize) -> u32 {
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn RtmCreateDestEnum(rtmreghandle: isize, targetviews: u32, enumflags: u32, netaddress: *mut RTM_NET_ADDRESS, protocolid: u32, rtmenumhandle: *mut isize) -> u32;
    }
    RtmCreateDestEnum(rtmreghandle, targetviews, enumflags, ::core::mem::transmute(netaddress), protocolid, ::core::mem::transmute(rtmenumhandle))
}
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`*"]
#[inline]
pub unsafe fn RtmCreateNextHopEnum(rtmreghandle: isize, enumflags: u32, netaddress: *mut RTM_NET_ADDRESS, rtmenumhandle: *mut isize) -> u32 {
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn RtmCreateNextHopEnum(rtmreghandle: isize, enumflags: u32, netaddress: *mut RTM_NET_ADDRESS, rtmenumhandle: *mut isize) -> u32;
    }
    RtmCreateNextHopEnum(rtmreghandle, enumflags, ::core::mem::transmute(netaddress), ::core::mem::transmute(rtmenumhandle))
}
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`*"]
#[inline]
pub unsafe fn RtmCreateRouteEnum(rtmreghandle: isize, desthandle: isize, targetviews: u32, enumflags: u32, startdest: *mut RTM_NET_ADDRESS, matchingflags: u32, criteriaroute: *mut RTM_ROUTE_INFO, criteriainterface: u32, rtmenumhandle: *mut isize) -> u32 {
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn RtmCreateRouteEnum(rtmreghandle: isize, desthandle: isize, targetviews: u32, enumflags: u32, startdest: *mut RTM_NET_ADDRESS, matchingflags: u32, criteriaroute: *mut RTM_ROUTE_INFO, criteriainterface: u32, rtmenumhandle: *mut isize) -> u32;
    }
    RtmCreateRouteEnum(rtmreghandle, desthandle, targetviews, enumflags, ::core::mem::transmute(startdest), matchingflags, ::core::mem::transmute(criteriaroute), criteriainterface, ::core::mem::transmute(rtmenumhandle))
}
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`*"]
#[inline]
pub unsafe fn RtmCreateRouteList(rtmreghandle: isize, routelisthandle: *mut isize) -> u32 {
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn RtmCreateRouteList(rtmreghandle: isize, routelisthandle: *mut isize) -> u32;
    }
    RtmCreateRouteList(rtmreghandle, ::core::mem::transmute(routelisthandle))
}
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`*"]
#[inline]
pub unsafe fn RtmCreateRouteListEnum(rtmreghandle: isize, routelisthandle: isize, rtmenumhandle: *mut isize) -> u32 {
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn RtmCreateRouteListEnum(rtmreghandle: isize, routelisthandle: isize, rtmenumhandle: *mut isize) -> u32;
    }
    RtmCreateRouteListEnum(rtmreghandle, routelisthandle, ::core::mem::transmute(rtmenumhandle))
}
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`*"]
#[inline]
pub unsafe fn RtmDeleteEnumHandle(rtmreghandle: isize, enumhandle: isize) -> u32 {
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn RtmDeleteEnumHandle(rtmreghandle: isize, enumhandle: isize) -> u32;
    }
    RtmDeleteEnumHandle(rtmreghandle, enumhandle)
}
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`*"]
#[inline]
pub unsafe fn RtmDeleteNextHop(rtmreghandle: isize, nexthophandle: isize, nexthopinfo: *mut RTM_NEXTHOP_INFO) -> u32 {
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn RtmDeleteNextHop(rtmreghandle: isize, nexthophandle: isize, nexthopinfo: *mut RTM_NEXTHOP_INFO) -> u32;
    }
    RtmDeleteNextHop(rtmreghandle, nexthophandle, ::core::mem::transmute(nexthopinfo))
}
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`*"]
#[inline]
pub unsafe fn RtmDeleteRouteList(rtmreghandle: isize, routelisthandle: isize) -> u32 {
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn RtmDeleteRouteList(rtmreghandle: isize, routelisthandle: isize) -> u32;
    }
    RtmDeleteRouteList(rtmreghandle, routelisthandle)
}
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`*"]
#[inline]
pub unsafe fn RtmDeleteRouteToDest(rtmreghandle: isize, routehandle: isize, changeflags: *mut u32) -> u32 {
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn RtmDeleteRouteToDest(rtmreghandle: isize, routehandle: isize, changeflags: *mut u32) -> u32;
    }
    RtmDeleteRouteToDest(rtmreghandle, routehandle, ::core::mem::transmute(changeflags))
}
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`*"]
#[inline]
pub unsafe fn RtmDeregisterEntity(rtmreghandle: isize) -> u32 {
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn RtmDeregisterEntity(rtmreghandle: isize) -> u32;
    }
    RtmDeregisterEntity(rtmreghandle)
}
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`*"]
#[inline]
pub unsafe fn RtmDeregisterFromChangeNotification(rtmreghandle: isize, notifyhandle: isize) -> u32 {
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn RtmDeregisterFromChangeNotification(rtmreghandle: isize, notifyhandle: isize) -> u32;
    }
    RtmDeregisterFromChangeNotification(rtmreghandle, notifyhandle)
}
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`*"]
#[inline]
pub unsafe fn RtmFindNextHop(rtmreghandle: isize, nexthopinfo: *mut RTM_NEXTHOP_INFO, nexthophandle: *mut isize, nexthoppointer: *mut *mut RTM_NEXTHOP_INFO) -> u32 {
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn RtmFindNextHop(rtmreghandle: isize, nexthopinfo: *mut RTM_NEXTHOP_INFO, nexthophandle: *mut isize, nexthoppointer: *mut *mut RTM_NEXTHOP_INFO) -> u32;
    }
    RtmFindNextHop(rtmreghandle, ::core::mem::transmute(nexthopinfo), ::core::mem::transmute(nexthophandle), ::core::mem::transmute(nexthoppointer))
}
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn RtmGetChangeStatus(rtmreghandle: isize, notifyhandle: isize, desthandle: isize, changestatus: *mut super::super::Foundation::BOOL) -> u32 {
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn RtmGetChangeStatus(rtmreghandle: isize, notifyhandle: isize, desthandle: isize, changestatus: *mut super::super::Foundation::BOOL) -> u32;
    }
    RtmGetChangeStatus(rtmreghandle, notifyhandle, desthandle, ::core::mem::transmute(changestatus))
}
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn RtmGetChangedDests(rtmreghandle: isize, notifyhandle: isize, numdests: *mut u32, changeddests: *mut RTM_DEST_INFO) -> u32 {
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn RtmGetChangedDests(rtmreghandle: isize, notifyhandle: isize, numdests: *mut u32, changeddests: *mut RTM_DEST_INFO) -> u32;
    }
    RtmGetChangedDests(rtmreghandle, notifyhandle, ::core::mem::transmute(numdests), ::core::mem::transmute(changeddests))
}
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn RtmGetDestInfo(rtmreghandle: isize, desthandle: isize, protocolid: u32, targetviews: u32, destinfo: *mut RTM_DEST_INFO) -> u32 {
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn RtmGetDestInfo(rtmreghandle: isize, desthandle: isize, protocolid: u32, targetviews: u32, destinfo: *mut RTM_DEST_INFO) -> u32;
    }
    RtmGetDestInfo(rtmreghandle, desthandle, protocolid, targetviews, ::core::mem::transmute(destinfo))
}
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`*"]
#[inline]
pub unsafe fn RtmGetEntityInfo(rtmreghandle: isize, entityhandle: isize, entityinfo: *mut RTM_ENTITY_INFO) -> u32 {
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn RtmGetEntityInfo(rtmreghandle: isize, entityhandle: isize, entityinfo: *mut RTM_ENTITY_INFO) -> u32;
    }
    RtmGetEntityInfo(rtmreghandle, entityhandle, ::core::mem::transmute(entityinfo))
}
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`*"]
#[inline]
pub unsafe fn RtmGetEntityMethods(rtmreghandle: isize, entityhandle: isize, nummethods: *mut u32, exptmethods: *mut RTM_ENTITY_EXPORT_METHOD) -> u32 {
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn RtmGetEntityMethods(rtmreghandle: isize, entityhandle: isize, nummethods: *mut u32, exptmethods: *mut *mut ::core::ffi::c_void) -> u32;
    }
    RtmGetEntityMethods(rtmreghandle, entityhandle, ::core::mem::transmute(nummethods), ::core::mem::transmute(exptmethods))
}
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn RtmGetEnumDests(rtmreghandle: isize, enumhandle: isize, numdests: *mut u32, destinfos: *mut RTM_DEST_INFO) -> u32 {
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn RtmGetEnumDests(rtmreghandle: isize, enumhandle: isize, numdests: *mut u32, destinfos: *mut RTM_DEST_INFO) -> u32;
    }
    RtmGetEnumDests(rtmreghandle, enumhandle, ::core::mem::transmute(numdests), ::core::mem::transmute(destinfos))
}
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`*"]
#[inline]
pub unsafe fn RtmGetEnumNextHops(rtmreghandle: isize, enumhandle: isize, numnexthops: *mut u32, nexthophandles: *mut isize) -> u32 {
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn RtmGetEnumNextHops(rtmreghandle: isize, enumhandle: isize, numnexthops: *mut u32, nexthophandles: *mut isize) -> u32;
    }
    RtmGetEnumNextHops(rtmreghandle, enumhandle, ::core::mem::transmute(numnexthops), ::core::mem::transmute(nexthophandles))
}
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`*"]
#[inline]
pub unsafe fn RtmGetEnumRoutes(rtmreghandle: isize, enumhandle: isize, numroutes: *mut u32, routehandles: *mut isize) -> u32 {
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn RtmGetEnumRoutes(rtmreghandle: isize, enumhandle: isize, numroutes: *mut u32, routehandles: *mut isize) -> u32;
    }
    RtmGetEnumRoutes(rtmreghandle, enumhandle, ::core::mem::transmute(numroutes), ::core::mem::transmute(routehandles))
}
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn RtmGetExactMatchDestination(rtmreghandle: isize, destaddress: *mut RTM_NET_ADDRESS, protocolid: u32, targetviews: u32, destinfo: *mut RTM_DEST_INFO) -> u32 {
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn RtmGetExactMatchDestination(rtmreghandle: isize, destaddress: *mut RTM_NET_ADDRESS, protocolid: u32, targetviews: u32, destinfo: *mut RTM_DEST_INFO) -> u32;
    }
    RtmGetExactMatchDestination(rtmreghandle, ::core::mem::transmute(destaddress), protocolid, targetviews, ::core::mem::transmute(destinfo))
}
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`*"]
#[inline]
pub unsafe fn RtmGetExactMatchRoute(rtmreghandle: isize, destaddress: *mut RTM_NET_ADDRESS, matchingflags: u32, routeinfo: *mut RTM_ROUTE_INFO, interfaceindex: u32, targetviews: u32, routehandle: *mut isize) -> u32 {
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn RtmGetExactMatchRoute(rtmreghandle: isize, destaddress: *mut RTM_NET_ADDRESS, matchingflags: u32, routeinfo: *mut RTM_ROUTE_INFO, interfaceindex: u32, targetviews: u32, routehandle: *mut isize) -> u32;
    }
    RtmGetExactMatchRoute(rtmreghandle, ::core::mem::transmute(destaddress), matchingflags, ::core::mem::transmute(routeinfo), interfaceindex, targetviews, ::core::mem::transmute(routehandle))
}
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn RtmGetLessSpecificDestination(rtmreghandle: isize, desthandle: isize, protocolid: u32, targetviews: u32, destinfo: *mut RTM_DEST_INFO) -> u32 {
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn RtmGetLessSpecificDestination(rtmreghandle: isize, desthandle: isize, protocolid: u32, targetviews: u32, destinfo: *mut RTM_DEST_INFO) -> u32;
    }
    RtmGetLessSpecificDestination(rtmreghandle, desthandle, protocolid, targetviews, ::core::mem::transmute(destinfo))
}
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`*"]
#[inline]
pub unsafe fn RtmGetListEnumRoutes(rtmreghandle: isize, enumhandle: isize, numroutes: *mut u32, routehandles: *mut isize) -> u32 {
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn RtmGetListEnumRoutes(rtmreghandle: isize, enumhandle: isize, numroutes: *mut u32, routehandles: *mut isize) -> u32;
    }
    RtmGetListEnumRoutes(rtmreghandle, enumhandle, ::core::mem::transmute(numroutes), ::core::mem::transmute(routehandles))
}
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn RtmGetMostSpecificDestination(rtmreghandle: isize, destaddress: *mut RTM_NET_ADDRESS, protocolid: u32, targetviews: u32, destinfo: *mut RTM_DEST_INFO) -> u32 {
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn RtmGetMostSpecificDestination(rtmreghandle: isize, destaddress: *mut RTM_NET_ADDRESS, protocolid: u32, targetviews: u32, destinfo: *mut RTM_DEST_INFO) -> u32;
    }
    RtmGetMostSpecificDestination(rtmreghandle, ::core::mem::transmute(destaddress), protocolid, targetviews, ::core::mem::transmute(destinfo))
}
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`*"]
#[inline]
pub unsafe fn RtmGetNextHopInfo(rtmreghandle: isize, nexthophandle: isize, nexthopinfo: *mut RTM_NEXTHOP_INFO) -> u32 {
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn RtmGetNextHopInfo(rtmreghandle: isize, nexthophandle: isize, nexthopinfo: *mut RTM_NEXTHOP_INFO) -> u32;
    }
    RtmGetNextHopInfo(rtmreghandle, nexthophandle, ::core::mem::transmute(nexthopinfo))
}
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`*"]
#[inline]
pub unsafe fn RtmGetNextHopPointer(rtmreghandle: isize, nexthophandle: isize, nexthoppointer: *mut *mut RTM_NEXTHOP_INFO) -> u32 {
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn RtmGetNextHopPointer(rtmreghandle: isize, nexthophandle: isize, nexthoppointer: *mut *mut RTM_NEXTHOP_INFO) -> u32;
    }
    RtmGetNextHopPointer(rtmreghandle, nexthophandle, ::core::mem::transmute(nexthoppointer))
}
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`*"]
#[inline]
pub unsafe fn RtmGetOpaqueInformationPointer(rtmreghandle: isize, desthandle: isize, opaqueinfopointer: *mut *mut ::core::ffi::c_void) -> u32 {
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn RtmGetOpaqueInformationPointer(rtmreghandle: isize, desthandle: isize, opaqueinfopointer: *mut *mut ::core::ffi::c_void) -> u32;
    }
    RtmGetOpaqueInformationPointer(rtmreghandle, desthandle, ::core::mem::transmute(opaqueinfopointer))
}
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`*"]
#[inline]
pub unsafe fn RtmGetRegisteredEntities(rtmreghandle: isize, numentities: *mut u32, entityhandles: *mut isize, entityinfos: *mut RTM_ENTITY_INFO) -> u32 {
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn RtmGetRegisteredEntities(rtmreghandle: isize, numentities: *mut u32, entityhandles: *mut isize, entityinfos: *mut RTM_ENTITY_INFO) -> u32;
    }
    RtmGetRegisteredEntities(rtmreghandle, ::core::mem::transmute(numentities), ::core::mem::transmute(entityhandles), ::core::mem::transmute(entityinfos))
}
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`*"]
#[inline]
pub unsafe fn RtmGetRouteInfo(rtmreghandle: isize, routehandle: isize, routeinfo: *mut RTM_ROUTE_INFO, destaddress: *mut RTM_NET_ADDRESS) -> u32 {
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn RtmGetRouteInfo(rtmreghandle: isize, routehandle: isize, routeinfo: *mut RTM_ROUTE_INFO, destaddress: *mut RTM_NET_ADDRESS) -> u32;
    }
    RtmGetRouteInfo(rtmreghandle, routehandle, ::core::mem::transmute(routeinfo), ::core::mem::transmute(destaddress))
}
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`*"]
#[inline]
pub unsafe fn RtmGetRoutePointer(rtmreghandle: isize, routehandle: isize, routepointer: *mut *mut RTM_ROUTE_INFO) -> u32 {
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn RtmGetRoutePointer(rtmreghandle: isize, routehandle: isize, routepointer: *mut *mut RTM_ROUTE_INFO) -> u32;
    }
    RtmGetRoutePointer(rtmreghandle, routehandle, ::core::mem::transmute(routepointer))
}
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`*"]
#[inline]
pub unsafe fn RtmHoldDestination(rtmreghandle: isize, desthandle: isize, targetviews: u32, holdtime: u32) -> u32 {
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn RtmHoldDestination(rtmreghandle: isize, desthandle: isize, targetviews: u32, holdtime: u32) -> u32;
    }
    RtmHoldDestination(rtmreghandle, desthandle, targetviews, holdtime)
}
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`*"]
#[inline]
pub unsafe fn RtmIgnoreChangedDests(rtmreghandle: isize, notifyhandle: isize, numdests: u32, changeddests: *mut isize) -> u32 {
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn RtmIgnoreChangedDests(rtmreghandle: isize, notifyhandle: isize, numdests: u32, changeddests: *mut isize) -> u32;
    }
    RtmIgnoreChangedDests(rtmreghandle, notifyhandle, numdests, ::core::mem::transmute(changeddests))
}
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`*"]
#[inline]
pub unsafe fn RtmInsertInRouteList(rtmreghandle: isize, routelisthandle: isize, numroutes: u32, routehandles: *mut isize) -> u32 {
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn RtmInsertInRouteList(rtmreghandle: isize, routelisthandle: isize, numroutes: u32, routehandles: *mut isize) -> u32;
    }
    RtmInsertInRouteList(rtmreghandle, routelisthandle, numroutes, ::core::mem::transmute(routehandles))
}
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`*"]
#[inline]
pub unsafe fn RtmInvokeMethod(rtmreghandle: isize, entityhandle: isize, input: *mut RTM_ENTITY_METHOD_INPUT, outputsize: *mut u32, output: *mut RTM_ENTITY_METHOD_OUTPUT) -> u32 {
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn RtmInvokeMethod(rtmreghandle: isize, entityhandle: isize, input: *mut RTM_ENTITY_METHOD_INPUT, outputsize: *mut u32, output: *mut RTM_ENTITY_METHOD_OUTPUT) -> u32;
    }
    RtmInvokeMethod(rtmreghandle, entityhandle, ::core::mem::transmute(input), ::core::mem::transmute(outputsize), ::core::mem::transmute(output))
}
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`*"]
#[inline]
pub unsafe fn RtmIsBestRoute(rtmreghandle: isize, routehandle: isize, bestinviews: *mut u32) -> u32 {
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn RtmIsBestRoute(rtmreghandle: isize, routehandle: isize, bestinviews: *mut u32) -> u32;
    }
    RtmIsBestRoute(rtmreghandle, routehandle, ::core::mem::transmute(bestinviews))
}
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn RtmIsMarkedForChangeNotification(rtmreghandle: isize, notifyhandle: isize, desthandle: isize, destmarked: *mut super::super::Foundation::BOOL) -> u32 {
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn RtmIsMarkedForChangeNotification(rtmreghandle: isize, notifyhandle: isize, desthandle: isize, destmarked: *mut super::super::Foundation::BOOL) -> u32;
    }
    RtmIsMarkedForChangeNotification(rtmreghandle, notifyhandle, desthandle, ::core::mem::transmute(destmarked))
}
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn RtmLockDestination<'a, P0, P1>(rtmreghandle: isize, desthandle: isize, exclusive: P0, lockdest: P1) -> u32
where
    P0: ::std::convert::Into<super::super::Foundation::BOOL>,
    P1: ::std::convert::Into<super::super::Foundation::BOOL>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn RtmLockDestination(rtmreghandle: isize, desthandle: isize, exclusive: super::super::Foundation::BOOL, lockdest: super::super::Foundation::BOOL) -> u32;
    }
    RtmLockDestination(rtmreghandle, desthandle, exclusive.into(), lockdest.into())
}
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn RtmLockNextHop<'a, P0, P1>(rtmreghandle: isize, nexthophandle: isize, exclusive: P0, locknexthop: P1, nexthoppointer: *mut *mut RTM_NEXTHOP_INFO) -> u32
where
    P0: ::std::convert::Into<super::super::Foundation::BOOL>,
    P1: ::std::convert::Into<super::super::Foundation::BOOL>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn RtmLockNextHop(rtmreghandle: isize, nexthophandle: isize, exclusive: super::super::Foundation::BOOL, locknexthop: super::super::Foundation::BOOL, nexthoppointer: *mut *mut RTM_NEXTHOP_INFO) -> u32;
    }
    RtmLockNextHop(rtmreghandle, nexthophandle, exclusive.into(), locknexthop.into(), ::core::mem::transmute(nexthoppointer))
}
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn RtmLockRoute<'a, P0, P1>(rtmreghandle: isize, routehandle: isize, exclusive: P0, lockroute: P1, routepointer: *mut *mut RTM_ROUTE_INFO) -> u32
where
    P0: ::std::convert::Into<super::super::Foundation::BOOL>,
    P1: ::std::convert::Into<super::super::Foundation::BOOL>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn RtmLockRoute(rtmreghandle: isize, routehandle: isize, exclusive: super::super::Foundation::BOOL, lockroute: super::super::Foundation::BOOL, routepointer: *mut *mut RTM_ROUTE_INFO) -> u32;
    }
    RtmLockRoute(rtmreghandle, routehandle, exclusive.into(), lockroute.into(), ::core::mem::transmute(routepointer))
}
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn RtmMarkDestForChangeNotification<'a, P0>(rtmreghandle: isize, notifyhandle: isize, desthandle: isize, markdest: P0) -> u32
where
    P0: ::std::convert::Into<super::super::Foundation::BOOL>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn RtmMarkDestForChangeNotification(rtmreghandle: isize, notifyhandle: isize, desthandle: isize, markdest: super::super::Foundation::BOOL) -> u32;
    }
    RtmMarkDestForChangeNotification(rtmreghandle, notifyhandle, desthandle, markdest.into())
}
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn RtmReferenceHandles(rtmreghandle: isize, numhandles: u32, rtmhandles: *mut super::super::Foundation::HANDLE) -> u32 {
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn RtmReferenceHandles(rtmreghandle: isize, numhandles: u32, rtmhandles: *mut super::super::Foundation::HANDLE) -> u32;
    }
    RtmReferenceHandles(rtmreghandle, numhandles, ::core::mem::transmute(rtmhandles))
}
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn RtmRegisterEntity<'a, P0>(rtmentityinfo: *mut RTM_ENTITY_INFO, exportmethods: *mut RTM_ENTITY_EXPORT_METHODS, eventcallback: RTM_EVENT_CALLBACK, reserveopaquepointer: P0, rtmregprofile: *mut RTM_REGN_PROFILE, rtmreghandle: *mut isize) -> u32
where
    P0: ::std::convert::Into<super::super::Foundation::BOOL>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn RtmRegisterEntity(rtmentityinfo: *mut RTM_ENTITY_INFO, exportmethods: *mut RTM_ENTITY_EXPORT_METHODS, eventcallback: *mut ::core::ffi::c_void, reserveopaquepointer: super::super::Foundation::BOOL, rtmregprofile: *mut RTM_REGN_PROFILE, rtmreghandle: *mut isize) -> u32;
    }
    RtmRegisterEntity(::core::mem::transmute(rtmentityinfo), ::core::mem::transmute(exportmethods), ::core::mem::transmute(eventcallback), reserveopaquepointer.into(), ::core::mem::transmute(rtmregprofile), ::core::mem::transmute(rtmreghandle))
}
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`*"]
#[inline]
pub unsafe fn RtmRegisterForChangeNotification(rtmreghandle: isize, targetviews: u32, notifyflags: u32, notifycontext: *mut ::core::ffi::c_void, notifyhandle: *mut isize) -> u32 {
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn RtmRegisterForChangeNotification(rtmreghandle: isize, targetviews: u32, notifyflags: u32, notifycontext: *mut ::core::ffi::c_void, notifyhandle: *mut isize) -> u32;
    }
    RtmRegisterForChangeNotification(rtmreghandle, targetviews, notifyflags, ::core::mem::transmute(notifycontext), ::core::mem::transmute(notifyhandle))
}
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn RtmReleaseChangedDests(rtmreghandle: isize, notifyhandle: isize, numdests: u32, changeddests: *mut RTM_DEST_INFO) -> u32 {
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn RtmReleaseChangedDests(rtmreghandle: isize, notifyhandle: isize, numdests: u32, changeddests: *mut RTM_DEST_INFO) -> u32;
    }
    RtmReleaseChangedDests(rtmreghandle, notifyhandle, numdests, ::core::mem::transmute(changeddests))
}
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn RtmReleaseDestInfo(rtmreghandle: isize, destinfo: *mut RTM_DEST_INFO) -> u32 {
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn RtmReleaseDestInfo(rtmreghandle: isize, destinfo: *mut RTM_DEST_INFO) -> u32;
    }
    RtmReleaseDestInfo(rtmreghandle, ::core::mem::transmute(destinfo))
}
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn RtmReleaseDests(rtmreghandle: isize, numdests: u32, destinfos: *mut RTM_DEST_INFO) -> u32 {
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn RtmReleaseDests(rtmreghandle: isize, numdests: u32, destinfos: *mut RTM_DEST_INFO) -> u32;
    }
    RtmReleaseDests(rtmreghandle, numdests, ::core::mem::transmute(destinfos))
}
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`*"]
#[inline]
pub unsafe fn RtmReleaseEntities(rtmreghandle: isize, numentities: u32, entityhandles: *mut isize) -> u32 {
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn RtmReleaseEntities(rtmreghandle: isize, numentities: u32, entityhandles: *mut isize) -> u32;
    }
    RtmReleaseEntities(rtmreghandle, numentities, ::core::mem::transmute(entityhandles))
}
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`*"]
#[inline]
pub unsafe fn RtmReleaseEntityInfo(rtmreghandle: isize, entityinfo: *mut RTM_ENTITY_INFO) -> u32 {
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn RtmReleaseEntityInfo(rtmreghandle: isize, entityinfo: *mut RTM_ENTITY_INFO) -> u32;
    }
    RtmReleaseEntityInfo(rtmreghandle, ::core::mem::transmute(entityinfo))
}
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`*"]
#[inline]
pub unsafe fn RtmReleaseNextHopInfo(rtmreghandle: isize, nexthopinfo: *mut RTM_NEXTHOP_INFO) -> u32 {
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn RtmReleaseNextHopInfo(rtmreghandle: isize, nexthopinfo: *mut RTM_NEXTHOP_INFO) -> u32;
    }
    RtmReleaseNextHopInfo(rtmreghandle, ::core::mem::transmute(nexthopinfo))
}
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`*"]
#[inline]
pub unsafe fn RtmReleaseNextHops(rtmreghandle: isize, numnexthops: u32, nexthophandles: *mut isize) -> u32 {
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn RtmReleaseNextHops(rtmreghandle: isize, numnexthops: u32, nexthophandles: *mut isize) -> u32;
    }
    RtmReleaseNextHops(rtmreghandle, numnexthops, ::core::mem::transmute(nexthophandles))
}
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`*"]
#[inline]
pub unsafe fn RtmReleaseRouteInfo(rtmreghandle: isize, routeinfo: *mut RTM_ROUTE_INFO) -> u32 {
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn RtmReleaseRouteInfo(rtmreghandle: isize, routeinfo: *mut RTM_ROUTE_INFO) -> u32;
    }
    RtmReleaseRouteInfo(rtmreghandle, ::core::mem::transmute(routeinfo))
}
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`*"]
#[inline]
pub unsafe fn RtmReleaseRoutes(rtmreghandle: isize, numroutes: u32, routehandles: *mut isize) -> u32 {
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn RtmReleaseRoutes(rtmreghandle: isize, numroutes: u32, routehandles: *mut isize) -> u32;
    }
    RtmReleaseRoutes(rtmreghandle, numroutes, ::core::mem::transmute(routehandles))
}
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`*"]
#[inline]
pub unsafe fn RtmUpdateAndUnlockRoute(rtmreghandle: isize, routehandle: isize, timetolive: u32, routelisthandle: isize, notifytype: u32, notifyhandle: isize, changeflags: *mut u32) -> u32 {
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn RtmUpdateAndUnlockRoute(rtmreghandle: isize, routehandle: isize, timetolive: u32, routelisthandle: isize, notifytype: u32, notifyhandle: isize, changeflags: *mut u32) -> u32;
    }
    RtmUpdateAndUnlockRoute(rtmreghandle, routehandle, timetolive, routelisthandle, notifytype, notifyhandle, ::core::mem::transmute(changeflags))
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct SECURITY_MESSAGE {
    pub dwMsgId: SECURITY_MESSAGE_MSG_ID,
    pub hPort: isize,
    pub dwError: u32,
    pub UserName: [super::super::Foundation::CHAR; 257],
    pub Domain: [super::super::Foundation::CHAR; 16],
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for SECURITY_MESSAGE {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for SECURITY_MESSAGE {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for SECURITY_MESSAGE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SECURITY_MESSAGE").field("dwMsgId", &self.dwMsgId).field("hPort", &self.hPort).field("dwError", &self.dwError).field("UserName", &self.UserName).field("Domain", &self.Domain).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for SECURITY_MESSAGE {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for SECURITY_MESSAGE {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<SECURITY_MESSAGE>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for SECURITY_MESSAGE {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for SECURITY_MESSAGE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct SECURITY_MESSAGE_MSG_ID(pub u32);
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`*"]
pub const SECURITYMSG_SUCCESS: SECURITY_MESSAGE_MSG_ID = SECURITY_MESSAGE_MSG_ID(1u32);
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`*"]
pub const SECURITYMSG_FAILURE: SECURITY_MESSAGE_MSG_ID = SECURITY_MESSAGE_MSG_ID(2u32);
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`*"]
pub const SECURITYMSG_ERROR: SECURITY_MESSAGE_MSG_ID = SECURITY_MESSAGE_MSG_ID(3u32);
impl ::core::marker::Copy for SECURITY_MESSAGE_MSG_ID {}
impl ::core::clone::Clone for SECURITY_MESSAGE_MSG_ID {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for SECURITY_MESSAGE_MSG_ID {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for SECURITY_MESSAGE_MSG_ID {
    type Abi = Self;
}
impl ::core::fmt::Debug for SECURITY_MESSAGE_MSG_ID {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SECURITY_MESSAGE_MSG_ID").field(&self.0).finish()
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`*"]
pub struct SOURCE_GROUP_ENTRY {
    pub dwSourceAddr: u32,
    pub dwSourceMask: u32,
    pub dwGroupAddr: u32,
    pub dwGroupMask: u32,
}
impl ::core::marker::Copy for SOURCE_GROUP_ENTRY {}
impl ::core::clone::Clone for SOURCE_GROUP_ENTRY {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for SOURCE_GROUP_ENTRY {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SOURCE_GROUP_ENTRY").field("dwSourceAddr", &self.dwSourceAddr).field("dwSourceMask", &self.dwSourceMask).field("dwGroupAddr", &self.dwGroupAddr).field("dwGroupMask", &self.dwGroupMask).finish()
    }
}
unsafe impl ::windows::core::Abi for SOURCE_GROUP_ENTRY {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for SOURCE_GROUP_ENTRY {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<SOURCE_GROUP_ENTRY>()) == 0 }
    }
}
impl ::core::cmp::Eq for SOURCE_GROUP_ENTRY {}
impl ::core::default::Default for SOURCE_GROUP_ENTRY {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`, `\"Win32_Foundation\"`, `\"Win32_Security_Cryptography\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography"))]
pub struct SSTP_CERT_INFO {
    pub isDefault: super::super::Foundation::BOOL,
    pub certBlob: super::super::Security::Cryptography::CRYPTOAPI_BLOB,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography"))]
impl ::core::marker::Copy for SSTP_CERT_INFO {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography"))]
impl ::core::clone::Clone for SSTP_CERT_INFO {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography"))]
impl ::core::fmt::Debug for SSTP_CERT_INFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SSTP_CERT_INFO").field("isDefault", &self.isDefault).field("certBlob", &self.certBlob).finish()
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography"))]
unsafe impl ::windows::core::Abi for SSTP_CERT_INFO {
    type Abi = Self;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography"))]
impl ::core::cmp::PartialEq for SSTP_CERT_INFO {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<SSTP_CERT_INFO>()) == 0 }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography"))]
impl ::core::cmp::Eq for SSTP_CERT_INFO {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography"))]
impl ::core::default::Default for SSTP_CERT_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`, `\"Win32_Foundation\"`, `\"Win32_Security_Cryptography\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography"))]
pub struct SSTP_CONFIG_PARAMS {
    pub dwNumPorts: u32,
    pub dwPortFlags: u32,
    pub isUseHttps: super::super::Foundation::BOOL,
    pub certAlgorithm: u32,
    pub sstpCertDetails: SSTP_CERT_INFO,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography"))]
impl ::core::marker::Copy for SSTP_CONFIG_PARAMS {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography"))]
impl ::core::clone::Clone for SSTP_CONFIG_PARAMS {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography"))]
impl ::core::fmt::Debug for SSTP_CONFIG_PARAMS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SSTP_CONFIG_PARAMS").field("dwNumPorts", &self.dwNumPorts).field("dwPortFlags", &self.dwPortFlags).field("isUseHttps", &self.isUseHttps).field("certAlgorithm", &self.certAlgorithm).field("sstpCertDetails", &self.sstpCertDetails).finish()
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography"))]
unsafe impl ::windows::core::Abi for SSTP_CONFIG_PARAMS {
    type Abi = Self;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography"))]
impl ::core::cmp::PartialEq for SSTP_CONFIG_PARAMS {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<SSTP_CONFIG_PARAMS>()) == 0 }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography"))]
impl ::core::cmp::Eq for SSTP_CONFIG_PARAMS {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography"))]
impl ::core::default::Default for SSTP_CONFIG_PARAMS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`, `\"Win32_Networking_WinSock\"`*"]
#[cfg(feature = "Win32_Networking_WinSock")]
pub struct VPN_TS_IP_ADDRESS {
    pub Type: u16,
    pub Anonymous: VPN_TS_IP_ADDRESS_0,
}
#[cfg(feature = "Win32_Networking_WinSock")]
impl ::core::marker::Copy for VPN_TS_IP_ADDRESS {}
#[cfg(feature = "Win32_Networking_WinSock")]
impl ::core::clone::Clone for VPN_TS_IP_ADDRESS {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Networking_WinSock")]
unsafe impl ::windows::core::Abi for VPN_TS_IP_ADDRESS {
    type Abi = Self;
}
#[cfg(feature = "Win32_Networking_WinSock")]
impl ::core::cmp::PartialEq for VPN_TS_IP_ADDRESS {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<VPN_TS_IP_ADDRESS>()) == 0 }
    }
}
#[cfg(feature = "Win32_Networking_WinSock")]
impl ::core::cmp::Eq for VPN_TS_IP_ADDRESS {}
#[cfg(feature = "Win32_Networking_WinSock")]
impl ::core::default::Default for VPN_TS_IP_ADDRESS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`, `\"Win32_Networking_WinSock\"`*"]
#[cfg(feature = "Win32_Networking_WinSock")]
pub union VPN_TS_IP_ADDRESS_0 {
    pub v4: super::super::Networking::WinSock::IN_ADDR,
    pub v6: super::super::Networking::WinSock::IN6_ADDR,
}
#[cfg(feature = "Win32_Networking_WinSock")]
impl ::core::marker::Copy for VPN_TS_IP_ADDRESS_0 {}
#[cfg(feature = "Win32_Networking_WinSock")]
impl ::core::clone::Clone for VPN_TS_IP_ADDRESS_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Networking_WinSock")]
unsafe impl ::windows::core::Abi for VPN_TS_IP_ADDRESS_0 {
    type Abi = Self;
}
#[cfg(feature = "Win32_Networking_WinSock")]
impl ::core::cmp::PartialEq for VPN_TS_IP_ADDRESS_0 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<VPN_TS_IP_ADDRESS_0>()) == 0 }
    }
}
#[cfg(feature = "Win32_Networking_WinSock")]
impl ::core::cmp::Eq for VPN_TS_IP_ADDRESS_0 {}
#[cfg(feature = "Win32_Networking_WinSock")]
impl ::core::default::Default for VPN_TS_IP_ADDRESS_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`*"]
pub const VS_Default: u32 = 0u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`*"]
pub const VS_GREOnly: u32 = 9u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`*"]
pub const VS_Ikev2First: u32 = 8u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`*"]
pub const VS_Ikev2Only: u32 = 7u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`*"]
pub const VS_Ikev2Sstp: u32 = 14u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`*"]
pub const VS_L2tpFirst: u32 = 4u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`*"]
pub const VS_L2tpOnly: u32 = 3u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`*"]
pub const VS_L2tpSstp: u32 = 13u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`*"]
pub const VS_PptpFirst: u32 = 2u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`*"]
pub const VS_PptpOnly: u32 = 1u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`*"]
pub const VS_PptpSstp: u32 = 12u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`*"]
pub const VS_ProtocolList: u32 = 15u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`*"]
pub const VS_SstpFirst: u32 = 6u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`*"]
pub const VS_SstpOnly: u32 = 5u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`*"]
pub const WARNING_MSG_ALIAS_NOT_ADDED: u32 = 644u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`*"]
pub const WM_RASDIALEVENT: u32 = 52429u32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_Rras\"`, `\"Win32_Networking_WinSock\"`*"]
#[cfg(feature = "Win32_Networking_WinSock")]
pub struct _MPR_VPN_SELECTOR {
    pub r#type: MPR_VPN_TS_TYPE,
    pub protocolId: u8,
    pub portStart: u16,
    pub portEnd: u16,
    pub tsPayloadId: u16,
    pub addrStart: VPN_TS_IP_ADDRESS,
    pub addrEnd: VPN_TS_IP_ADDRESS,
}
#[cfg(feature = "Win32_Networking_WinSock")]
impl ::core::marker::Copy for _MPR_VPN_SELECTOR {}
#[cfg(feature = "Win32_Networking_WinSock")]
impl ::core::clone::Clone for _MPR_VPN_SELECTOR {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Networking_WinSock")]
unsafe impl ::windows::core::Abi for _MPR_VPN_SELECTOR {
    type Abi = Self;
}
#[cfg(feature = "Win32_Networking_WinSock")]
impl ::core::cmp::PartialEq for _MPR_VPN_SELECTOR {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<_MPR_VPN_SELECTOR>()) == 0 }
    }
}
#[cfg(feature = "Win32_Networking_WinSock")]
impl ::core::cmp::Eq for _MPR_VPN_SELECTOR {}
#[cfg(feature = "Win32_Networking_WinSock")]
impl ::core::default::Default for _MPR_VPN_SELECTOR {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "implement")]
::core::include!("impl.rs");

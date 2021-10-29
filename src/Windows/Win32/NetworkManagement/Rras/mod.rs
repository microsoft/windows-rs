#![allow(unused_variables, non_upper_case_globals, non_snake_case, unused_unsafe, non_camel_case_types, dead_code, clippy::all)]
pub const ALLOW_NO_AUTH: u32 = 1u32;
pub const ATADDRESSLEN: u32 = 32u32;
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
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
impl AUTH_VALIDATION_EX {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for AUTH_VALIDATION_EX {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::fmt::Debug for AUTH_VALIDATION_EX {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("AUTH_VALIDATION_EX").field("Header", &self.Header).field("hRasConnection", &self.hRasConnection).field("wszUserName", &self.wszUserName).field("wszLogonDomain", &self.wszLogonDomain).field("AuthInfoSize", &self.AuthInfoSize).field("AuthInfo", &self.AuthInfo).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for AUTH_VALIDATION_EX {
    fn eq(&self, other: &Self) -> bool {
        self.Header == other.Header && self.hRasConnection == other.hRasConnection && self.wszUserName == other.wszUserName && self.wszLogonDomain == other.wszLogonDomain && self.AuthInfoSize == other.AuthInfoSize && self.AuthInfo == other.AuthInfo
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for AUTH_VALIDATION_EX {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for AUTH_VALIDATION_EX {
    type Abi = Self;
    type DefaultType = Self;
}
pub const DO_NOT_ALLOW_NO_AUTH: u32 = 0u32;
pub const ERROR_ACCESSING_TCPCFGDLL: u32 = 727u32;
pub const ERROR_ACCT_DISABLED: u32 = 647u32;
pub const ERROR_ACCT_EXPIRED: u32 = 708u32;
pub const ERROR_ACTION_REQUIRED: u32 = 877u32;
pub const ERROR_ALLOCATING_MEMORY: u32 = 664u32;
pub const ERROR_ALREADY_DISCONNECTING: u32 = 617u32;
pub const ERROR_ASYNC_REQUEST_PENDING: u32 = 616u32;
pub const ERROR_AUTHENTICATION_FAILURE: u32 = 691u32;
pub const ERROR_AUTH_INTERNAL: u32 = 645u32;
pub const ERROR_AUTOMATIC_VPN_FAILED: u32 = 800u32;
pub const ERROR_BAD_ADDRESS_SPECIFIED: u32 = 769u32;
pub const ERROR_BAD_CALLBACK_NUMBER: u32 = 704u32;
pub const ERROR_BAD_PHONE_NUMBER: u32 = 749u32;
pub const ERROR_BAD_STRING: u32 = 637u32;
pub const ERROR_BAD_USAGE_IN_INI_FILE: u32 = 669u32;
pub const ERROR_BIPLEX_PORT_NOT_AVAILABLE: u32 = 712u32;
pub const ERROR_BLOCKED: u32 = 775u32;
pub const ERROR_BROADBAND_ACTIVE: u32 = 813u32;
pub const ERROR_BROADBAND_NO_NIC: u32 = 814u32;
pub const ERROR_BROADBAND_TIMEOUT: u32 = 815u32;
pub const ERROR_BUFFER_INVALID: u32 = 610u32;
pub const ERROR_BUFFER_TOO_SMALL: u32 = 603u32;
pub const ERROR_BUNDLE_NOT_FOUND: u32 = 754u32;
pub const ERROR_CANNOT_DELETE: u32 = 817u32;
pub const ERROR_CANNOT_DO_CUSTOMDIAL: u32 = 755u32;
pub const ERROR_CANNOT_FIND_PHONEBOOK_ENTRY: u32 = 623u32;
pub const ERROR_CANNOT_GET_LANA: u32 = 639u32;
pub const ERROR_CANNOT_INITIATE_MOBIKE_UPDATE: u32 = 844u32;
pub const ERROR_CANNOT_LOAD_PHONEBOOK: u32 = 622u32;
pub const ERROR_CANNOT_LOAD_STRING: u32 = 626u32;
pub const ERROR_CANNOT_OPEN_PHONEBOOK: u32 = 621u32;
pub const ERROR_CANNOT_PROJECT_CLIENT: u32 = 634u32;
pub const ERROR_CANNOT_SET_PORT_INFO: u32 = 605u32;
pub const ERROR_CANNOT_SHARE_CONNECTION: u32 = 763u32;
pub const ERROR_CANNOT_USE_LOGON_CREDENTIALS: u32 = 739u32;
pub const ERROR_CANNOT_WRITE_PHONEBOOK: u32 = 624u32;
pub const ERROR_CERT_FOR_ENCRYPTION_NOT_FOUND: u32 = 781u32;
pub const ERROR_CHANGING_PASSWORD: u32 = 709u32;
pub const ERROR_CMD_TOO_LONG: u32 = 700u32;
pub const ERROR_CONGESTION: u32 = 771u32;
pub const ERROR_CONNECTING_DEVICE_NOT_FOUND: u32 = 797u32;
pub const ERROR_CONNECTION_ALREADY_SHARED: u32 = 758u32;
pub const ERROR_CONNECTION_REJECT: u32 = 770u32;
pub const ERROR_CORRUPT_PHONEBOOK: u32 = 625u32;
pub const ERROR_DCB_NOT_FOUND: u32 = 694u32;
pub const ERROR_DEFAULTOFF_MACRO_NOT_FOUND: u32 = 656u32;
pub const ERROR_DEVICENAME_NOT_FOUND: u32 = 659u32;
pub const ERROR_DEVICENAME_TOO_LONG: u32 = 658u32;
pub const ERROR_DEVICETYPE_DOES_NOT_EXIST: u32 = 609u32;
pub const ERROR_DEVICE_COMPLIANCE: u32 = 875u32;
pub const ERROR_DEVICE_DOES_NOT_EXIST: u32 = 608u32;
pub const ERROR_DEVICE_NOT_READY: u32 = 666u32;
pub const ERROR_DIAL_ALREADY_IN_PROGRESS: u32 = 756u32;
pub const ERROR_DISCONNECTION: u32 = 628u32;
pub const ERROR_DNSNAME_NOT_RESOLVABLE: u32 = 868u32;
pub const ERROR_DONOTDISTURB: u32 = 776u32;
pub const ERROR_EAPTLS_CACHE_CREDENTIALS_INVALID: u32 = 826u32;
pub const ERROR_EAPTLS_PASSWD_INVALID: u32 = 869u32;
pub const ERROR_EAPTLS_SCARD_CACHE_CREDENTIALS_INVALID: u32 = 847u32;
pub const ERROR_EAP_METHOD_DOES_NOT_SUPPORT_SSO: u32 = 851u32;
pub const ERROR_EAP_METHOD_NOT_INSTALLED: u32 = 850u32;
pub const ERROR_EAP_METHOD_OPERATION_NOT_SUPPORTED: u32 = 852u32;
pub const ERROR_EAP_SERVER_CERT_EXPIRED: u32 = 858u32;
pub const ERROR_EAP_SERVER_CERT_INVALID: u32 = 857u32;
pub const ERROR_EAP_SERVER_CERT_OTHER_ERROR: u32 = 860u32;
pub const ERROR_EAP_SERVER_CERT_REVOKED: u32 = 859u32;
pub const ERROR_EAP_SERVER_ROOT_CERT_INVALID: u32 = 865u32;
pub const ERROR_EAP_SERVER_ROOT_CERT_NAME_REQUIRED: u32 = 866u32;
pub const ERROR_EAP_SERVER_ROOT_CERT_NOT_FOUND: u32 = 864u32;
pub const ERROR_EAP_USER_CERT_EXPIRED: u32 = 854u32;
pub const ERROR_EAP_USER_CERT_INVALID: u32 = 853u32;
pub const ERROR_EAP_USER_CERT_OTHER_ERROR: u32 = 856u32;
pub const ERROR_EAP_USER_CERT_REVOKED: u32 = 855u32;
pub const ERROR_EAP_USER_ROOT_CERT_EXPIRED: u32 = 863u32;
pub const ERROR_EAP_USER_ROOT_CERT_INVALID: u32 = 862u32;
pub const ERROR_EAP_USER_ROOT_CERT_NOT_FOUND: u32 = 861u32;
pub const ERROR_EMPTY_INI_FILE: u32 = 690u32;
pub const ERROR_EVENT_INVALID: u32 = 607u32;
pub const ERROR_FAILED_CP_REQUIRED: u32 = 841u32;
pub const ERROR_FAILED_TO_ENCRYPT: u32 = 768u32;
pub const ERROR_FAST_USER_SWITCH: u32 = 831u32;
pub const ERROR_FEATURE_DEPRECATED: u32 = 816u32;
pub const ERROR_FILE_COULD_NOT_BE_OPENED: u32 = 657u32;
pub const ERROR_FROM_DEVICE: u32 = 651u32;
pub const ERROR_HANGUP_FAILED: u32 = 753u32;
pub const ERROR_HARDWARE_FAILURE: u32 = 630u32;
pub const ERROR_HIBERNATION: u32 = 832u32;
pub const ERROR_IDLE_TIMEOUT: u32 = 828u32;
pub const ERROR_IKEV2_PSK_INTERFACE_ALREADY_EXISTS: u32 = 870u32;
pub const ERROR_INCOMPATIBLE: u32 = 772u32;
pub const ERROR_INTERACTIVE_MODE: u32 = 703u32;
pub const ERROR_INTERNAL_ADDRESS_FAILURE: u32 = 840u32;
pub const ERROR_INVALID_AUTH_STATE: u32 = 705u32;
pub const ERROR_INVALID_CALLBACK_NUMBER: u32 = 751u32;
pub const ERROR_INVALID_COMPRESSION_SPECIFIED: u32 = 613u32;
pub const ERROR_INVALID_DESTINATION_IP: u32 = 871u32;
pub const ERROR_INVALID_FUNCTION_FOR_ENTRY: u32 = 780u32;
pub const ERROR_INVALID_INTERFACE_CONFIG: u32 = 872u32;
pub const ERROR_INVALID_MSCHAPV2_CONFIG: u32 = 805u32;
pub const ERROR_INVALID_PEAP_COOKIE_ATTRIBUTES: u32 = 849u32;
pub const ERROR_INVALID_PEAP_COOKIE_CONFIG: u32 = 803u32;
pub const ERROR_INVALID_PEAP_COOKIE_USER: u32 = 804u32;
pub const ERROR_INVALID_PORT_HANDLE: u32 = 601u32;
pub const ERROR_INVALID_PREFERENCES: u32 = 846u32;
pub const ERROR_INVALID_SERVER_CERT: u32 = 835u32;
pub const ERROR_INVALID_SIZE: u32 = 632u32;
pub const ERROR_INVALID_SMM: u32 = 745u32;
pub const ERROR_INVALID_TUNNELID: u32 = 837u32;
pub const ERROR_INVALID_VPNSTRATEGY: u32 = 825u32;
pub const ERROR_IN_COMMAND: u32 = 681u32;
pub const ERROR_IPSEC_SERVICE_STOPPED: u32 = 827u32;
pub const ERROR_IPXCP_DIALOUT_ALREADY_ACTIVE: u32 = 726u32;
pub const ERROR_IPXCP_NET_NUMBER_CONFLICT: u32 = 744u32;
pub const ERROR_IPXCP_NO_DIALIN_CONFIGURED: u32 = 725u32;
pub const ERROR_IPXCP_NO_DIALOUT_CONFIGURED: u32 = 724u32;
pub const ERROR_IP_CONFIGURATION: u32 = 716u32;
pub const ERROR_KEY_NOT_FOUND: u32 = 627u32;
pub const ERROR_LINE_BUSY: u32 = 676u32;
pub const ERROR_LINK_FAILURE: u32 = 829u32;
pub const ERROR_MACRO_NOT_DEFINED: u32 = 654u32;
pub const ERROR_MACRO_NOT_FOUND: u32 = 653u32;
pub const ERROR_MESSAGE_MACRO_NOT_FOUND: u32 = 655u32;
pub const ERROR_MOBIKE_DISABLED: u32 = 843u32;
pub const ERROR_NAME_EXISTS_ON_NET: u32 = 642u32;
pub const ERROR_NETBIOS_ERROR: u32 = 640u32;
pub const ERROR_NOT_BINARY_MACRO: u32 = 693u32;
pub const ERROR_NOT_NAP_CAPABLE: u32 = 836u32;
pub const ERROR_NO_ACTIVE_ISDN_LINES: u32 = 713u32;
pub const ERROR_NO_ANSWER: u32 = 678u32;
pub const ERROR_NO_CARRIER: u32 = 679u32;
pub const ERROR_NO_CERTIFICATE: u32 = 766u32;
pub const ERROR_NO_COMMAND_FOUND: u32 = 661u32;
pub const ERROR_NO_CONNECTION: u32 = 668u32;
pub const ERROR_NO_DIALIN_PERMISSION: u32 = 649u32;
pub const ERROR_NO_DIALTONE: u32 = 680u32;
pub const ERROR_NO_DIFF_USER_AT_LOGON: u32 = 784u32;
pub const ERROR_NO_EAPTLS_CERTIFICATE: u32 = 798u32;
pub const ERROR_NO_ENDPOINTS: u32 = 620u32;
pub const ERROR_NO_IP_ADDRESSES: u32 = 717u32;
pub const ERROR_NO_IP_RAS_ADAPTER: u32 = 728u32;
pub const ERROR_NO_ISDN_CHANNELS_AVAILABLE: u32 = 714u32;
pub const ERROR_NO_LOCAL_ENCRYPTION: u32 = 741u32;
pub const ERROR_NO_MAC_FOR_PORT: u32 = 747u32;
pub const ERROR_NO_REG_CERT_AT_LOGON: u32 = 785u32;
pub const ERROR_NO_REMOTE_ENCRYPTION: u32 = 742u32;
pub const ERROR_NO_RESPONSES: u32 = 660u32;
pub const ERROR_NO_SMART_CARD_READER: u32 = 764u32;
pub const ERROR_NUMBERCHANGED: u32 = 773u32;
pub const ERROR_OAKLEY_ATTRIB_FAIL: u32 = 788u32;
pub const ERROR_OAKLEY_AUTH_FAIL: u32 = 787u32;
pub const ERROR_OAKLEY_ERROR: u32 = 793u32;
pub const ERROR_OAKLEY_GENERAL_PROCESSING: u32 = 789u32;
pub const ERROR_OAKLEY_NO_CERT: u32 = 786u32;
pub const ERROR_OAKLEY_NO_PEER_CERT: u32 = 790u32;
pub const ERROR_OAKLEY_NO_POLICY: u32 = 791u32;
pub const ERROR_OAKLEY_TIMED_OUT: u32 = 792u32;
pub const ERROR_OUTOFORDER: u32 = 777u32;
pub const ERROR_OUT_OF_BUFFERS: u32 = 614u32;
pub const ERROR_OVERRUN: u32 = 710u32;
pub const ERROR_PARTIAL_RESPONSE_LOOPING: u32 = 697u32;
pub const ERROR_PASSWD_EXPIRED: u32 = 648u32;
pub const ERROR_PEAP_CRYPTOBINDING_INVALID: u32 = 823u32;
pub const ERROR_PEAP_CRYPTOBINDING_NOTRECEIVED: u32 = 824u32;
pub const ERROR_PEAP_IDENTITY_MISMATCH: u32 = 867u32;
pub const ERROR_PEAP_SERVER_REJECTED_CLIENT_TLV: u32 = 845u32;
pub const ERROR_PHONE_NUMBER_TOO_LONG: u32 = 723u32;
pub const ERROR_PLUGIN_NOT_INSTALLED: u32 = 876u32;
pub const ERROR_PORT_ALREADY_OPEN: u32 = 602u32;
pub const ERROR_PORT_DISCONNECTED: u32 = 619u32;
pub const ERROR_PORT_NOT_AVAILABLE: u32 = 633u32;
pub const ERROR_PORT_NOT_CONFIGURED: u32 = 665u32;
pub const ERROR_PORT_NOT_CONNECTED: u32 = 606u32;
pub const ERROR_PORT_NOT_FOUND: u32 = 615u32;
pub const ERROR_PORT_NOT_OPEN: u32 = 618u32;
pub const ERROR_PORT_OR_DEVICE: u32 = 692u32;
pub const ERROR_PPP_CP_REJECTED: u32 = 733u32;
pub const ERROR_PPP_INVALID_PACKET: u32 = 722u32;
pub const ERROR_PPP_LCP_TERMINATED: u32 = 734u32;
pub const ERROR_PPP_LOOPBACK_DETECTED: u32 = 737u32;
pub const ERROR_PPP_NCP_TERMINATED: u32 = 736u32;
pub const ERROR_PPP_NOT_CONVERGING: u32 = 732u32;
pub const ERROR_PPP_NO_ADDRESS_ASSIGNED: u32 = 738u32;
pub const ERROR_PPP_NO_PROTOCOLS_CONFIGURED: u32 = 720u32;
pub const ERROR_PPP_NO_RESPONSE: u32 = 721u32;
pub const ERROR_PPP_REMOTE_TERMINATED: u32 = 719u32;
pub const ERROR_PPP_REQUIRED_ADDRESS_REJECTED: u32 = 735u32;
pub const ERROR_PPP_TIMEOUT: u32 = 718u32;
pub const ERROR_PROJECTION_NOT_COMPLETE: u32 = 730u32;
pub const ERROR_PROTOCOL_ENGINE_DISABLED: u32 = 839u32;
pub const ERROR_PROTOCOL_NOT_CONFIGURED: u32 = 731u32;
pub const ERROR_RASAUTO_CANNOT_INITIALIZE: u32 = 757u32;
pub const ERROR_RASMAN_CANNOT_INITIALIZE: u32 = 711u32;
pub const ERROR_RASMAN_SERVICE_STOPPED: u32 = 834u32;
pub const ERROR_RASQEC_CONN_DOESNOTEXIST: u32 = 821u32;
pub const ERROR_RASQEC_NAPAGENT_NOT_CONNECTED: u32 = 820u32;
pub const ERROR_RASQEC_NAPAGENT_NOT_ENABLED: u32 = 819u32;
pub const ERROR_RASQEC_RESOURCE_CREATION_FAILED: u32 = 818u32;
pub const ERROR_RASQEC_TIMEOUT: u32 = 822u32;
pub const ERROR_READING_DEFAULTOFF: u32 = 689u32;
pub const ERROR_READING_DEVICENAME: u32 = 672u32;
pub const ERROR_READING_DEVICETYPE: u32 = 671u32;
pub const ERROR_READING_INI_FILE: u32 = 667u32;
pub const ERROR_READING_MAXCARRIERBPS: u32 = 675u32;
pub const ERROR_READING_MAXCONNECTBPS: u32 = 674u32;
pub const ERROR_READING_SCARD: u32 = 802u32;
pub const ERROR_READING_SECTIONNAME: u32 = 670u32;
pub const ERROR_READING_USAGE: u32 = 673u32;
pub const ERROR_RECV_BUF_FULL: u32 = 699u32;
pub const ERROR_REMOTE_DISCONNECTION: u32 = 629u32;
pub const ERROR_REMOTE_REQUIRES_ENCRYPTION: u32 = 743u32;
pub const ERROR_REQUEST_TIMEOUT: u32 = 638u32;
pub const ERROR_RESTRICTED_LOGON_HOURS: u32 = 646u32;
pub const ERROR_ROUTE_NOT_ALLOCATED: u32 = 612u32;
pub const ERROR_ROUTE_NOT_AVAILABLE: u32 = 611u32;
pub const ERROR_SCRIPT_SYNTAX: u32 = 752u32;
pub const ERROR_SERVER_GENERAL_NET_FAILURE: u32 = 643u32;
pub const ERROR_SERVER_NOT_RESPONDING: u32 = 650u32;
pub const ERROR_SERVER_OUT_OF_RESOURCES: u32 = 641u32;
pub const ERROR_SERVER_POLICY: u32 = 812u32;
pub const ERROR_SHARE_CONNECTION_FAILED: u32 = 761u32;
pub const ERROR_SHARING_ADDRESS_EXISTS: u32 = 765u32;
pub const ERROR_SHARING_CHANGE_FAILED: u32 = 759u32;
pub const ERROR_SHARING_HOST_ADDRESS_CONFLICT: u32 = 799u32;
pub const ERROR_SHARING_MULTIPLE_ADDRESSES: u32 = 767u32;
pub const ERROR_SHARING_NO_PRIVATE_LAN: u32 = 783u32;
pub const ERROR_SHARING_PRIVATE_INSTALL: u32 = 762u32;
pub const ERROR_SHARING_ROUTER_INSTALL: u32 = 760u32;
pub const ERROR_SHARING_RRAS_CONFLICT: u32 = 782u32;
pub const ERROR_SLIP_REQUIRES_IP: u32 = 729u32;
pub const ERROR_SMART_CARD_REQUIRED: u32 = 779u32;
pub const ERROR_SMM_TIMEOUT: u32 = 748u32;
pub const ERROR_SMM_UNINITIALIZED: u32 = 746u32;
pub const ERROR_SSO_CERT_MISSING: u32 = 874u32;
pub const ERROR_SSTP_COOKIE_SET_FAILURE: u32 = 848u32;
pub const ERROR_STATE_MACHINES_ALREADY_STARTED: u32 = 696u32;
pub const ERROR_STATE_MACHINES_NOT_STARTED: u32 = 695u32;
pub const ERROR_SYSTEM_SUSPENDED: u32 = 833u32;
pub const ERROR_TAPI_CONFIGURATION: u32 = 740u32;
pub const ERROR_TEMPFAILURE: u32 = 774u32;
pub const ERROR_TOO_MANY_LINE_ERRORS: u32 = 715u32;
pub const ERROR_TS_UNACCEPTABLE: u32 = 842u32;
pub const ERROR_UNABLE_TO_AUTHENTICATE_SERVER: u32 = 778u32;
pub const ERROR_UNEXPECTED_RESPONSE: u32 = 702u32;
pub const ERROR_UNKNOWN: u32 = 635u32;
pub const ERROR_UNKNOWN_DEVICE_TYPE: u32 = 663u32;
pub const ERROR_UNKNOWN_FRAMED_PROTOCOL: u32 = 794u32;
pub const ERROR_UNKNOWN_RESPONSE_KEY: u32 = 698u32;
pub const ERROR_UNKNOWN_SERVICE_TYPE: u32 = 796u32;
pub const ERROR_UNRECOGNIZED_RESPONSE: u32 = 652u32;
pub const ERROR_UNSUPPORTED_BPS: u32 = 701u32;
pub const ERROR_UPDATECONNECTION_REQUEST_IN_PROCESS: u32 = 838u32;
pub const ERROR_USER_DISCONNECTION: u32 = 631u32;
pub const ERROR_USER_LOGOFF: u32 = 830u32;
pub const ERROR_VALIDATING_SERVER_CERT: u32 = 801u32;
pub const ERROR_VOICE_ANSWER: u32 = 677u32;
pub const ERROR_VPN_BAD_CERT: u32 = 810u32;
pub const ERROR_VPN_BAD_PSK: u32 = 811u32;
pub const ERROR_VPN_DISCONNECT: u32 = 807u32;
pub const ERROR_VPN_GRE_BLOCKED: u32 = 806u32;
pub const ERROR_VPN_PLUGIN_GENERIC: u32 = 873u32;
pub const ERROR_VPN_REFUSED: u32 = 808u32;
pub const ERROR_VPN_TIMEOUT: u32 = 809u32;
pub const ERROR_WRITING_DEFAULTOFF: u32 = 688u32;
pub const ERROR_WRITING_DEVICENAME: u32 = 684u32;
pub const ERROR_WRITING_DEVICETYPE: u32 = 683u32;
pub const ERROR_WRITING_INITBPS: u32 = 706u32;
pub const ERROR_WRITING_MAXCARRIERBPS: u32 = 686u32;
pub const ERROR_WRITING_MAXCONNECTBPS: u32 = 685u32;
pub const ERROR_WRITING_SECTIONNAME: u32 = 682u32;
pub const ERROR_WRITING_USAGE: u32 = 687u32;
pub const ERROR_WRONG_DEVICE_ATTACHED: u32 = 636u32;
pub const ERROR_WRONG_INFO_SPECIFIED: u32 = 604u32;
pub const ERROR_WRONG_KEY_SPECIFIED: u32 = 662u32;
pub const ERROR_WRONG_MODULE: u32 = 750u32;
pub const ERROR_WRONG_TUNNEL_TYPE: u32 = 795u32;
pub const ERROR_X25_DIAGNOSTIC: u32 = 707u32;
pub const ET_None: u32 = 0u32;
pub const ET_Optional: u32 = 3u32;
pub const ET_Require: u32 = 1u32;
pub const ET_RequireMax: u32 = 2u32;
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
pub struct GRE_CONFIG_PARAMS0 {
    pub dwNumPorts: u32,
    pub dwPortFlags: u32,
}
impl GRE_CONFIG_PARAMS0 {}
impl ::std::default::Default for GRE_CONFIG_PARAMS0 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for GRE_CONFIG_PARAMS0 {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("GRE_CONFIG_PARAMS0").field("dwNumPorts", &self.dwNumPorts).field("dwPortFlags", &self.dwPortFlags).finish()
    }
}
impl ::std::cmp::PartialEq for GRE_CONFIG_PARAMS0 {
    fn eq(&self, other: &Self) -> bool {
        self.dwNumPorts == other.dwNumPorts && self.dwPortFlags == other.dwPortFlags
    }
}
impl ::std::cmp::Eq for GRE_CONFIG_PARAMS0 {}
unsafe impl ::windows::runtime::Abi for GRE_CONFIG_PARAMS0 {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy, :: std :: fmt :: Debug, :: std :: cmp :: PartialEq, :: std :: cmp :: Eq)]
#[repr(transparent)]
pub struct HRASCONN(pub isize);
impl ::std::default::Default for HRASCONN {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
unsafe impl ::windows::runtime::Handle for HRASCONN {}
unsafe impl ::windows::runtime::Abi for HRASCONN {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography"))]
pub struct IKEV2_CONFIG_PARAMS {
    pub dwNumPorts: u32,
    pub dwPortFlags: u32,
    pub dwTunnelConfigParamFlags: u32,
    pub TunnelConfigParams: IKEV2_TUNNEL_CONFIG_PARAMS4,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography"))]
impl IKEV2_CONFIG_PARAMS {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography"))]
impl ::std::default::Default for IKEV2_CONFIG_PARAMS {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography"))]
impl ::std::fmt::Debug for IKEV2_CONFIG_PARAMS {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("IKEV2_CONFIG_PARAMS").field("dwNumPorts", &self.dwNumPorts).field("dwPortFlags", &self.dwPortFlags).field("dwTunnelConfigParamFlags", &self.dwTunnelConfigParamFlags).field("TunnelConfigParams", &self.TunnelConfigParams).finish()
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography"))]
impl ::std::cmp::PartialEq for IKEV2_CONFIG_PARAMS {
    fn eq(&self, other: &Self) -> bool {
        self.dwNumPorts == other.dwNumPorts && self.dwPortFlags == other.dwPortFlags && self.dwTunnelConfigParamFlags == other.dwTunnelConfigParamFlags && self.TunnelConfigParams == other.TunnelConfigParams
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography"))]
impl ::std::cmp::Eq for IKEV2_CONFIG_PARAMS {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography"))]
unsafe impl ::windows::runtime::Abi for IKEV2_CONFIG_PARAMS {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct IKEV2_ID_PAYLOAD_TYPE(pub i32);
pub const IKEV2_ID_PAYLOAD_TYPE_INVALID: IKEV2_ID_PAYLOAD_TYPE = IKEV2_ID_PAYLOAD_TYPE(0i32);
pub const IKEV2_ID_PAYLOAD_TYPE_IPV4_ADDR: IKEV2_ID_PAYLOAD_TYPE = IKEV2_ID_PAYLOAD_TYPE(1i32);
pub const IKEV2_ID_PAYLOAD_TYPE_FQDN: IKEV2_ID_PAYLOAD_TYPE = IKEV2_ID_PAYLOAD_TYPE(2i32);
pub const IKEV2_ID_PAYLOAD_TYPE_RFC822_ADDR: IKEV2_ID_PAYLOAD_TYPE = IKEV2_ID_PAYLOAD_TYPE(3i32);
pub const IKEV2_ID_PAYLOAD_TYPE_RESERVED1: IKEV2_ID_PAYLOAD_TYPE = IKEV2_ID_PAYLOAD_TYPE(4i32);
pub const IKEV2_ID_PAYLOAD_TYPE_ID_IPV6_ADDR: IKEV2_ID_PAYLOAD_TYPE = IKEV2_ID_PAYLOAD_TYPE(5i32);
pub const IKEV2_ID_PAYLOAD_TYPE_RESERVED2: IKEV2_ID_PAYLOAD_TYPE = IKEV2_ID_PAYLOAD_TYPE(6i32);
pub const IKEV2_ID_PAYLOAD_TYPE_RESERVED3: IKEV2_ID_PAYLOAD_TYPE = IKEV2_ID_PAYLOAD_TYPE(7i32);
pub const IKEV2_ID_PAYLOAD_TYPE_RESERVED4: IKEV2_ID_PAYLOAD_TYPE = IKEV2_ID_PAYLOAD_TYPE(8i32);
pub const IKEV2_ID_PAYLOAD_TYPE_DER_ASN1_DN: IKEV2_ID_PAYLOAD_TYPE = IKEV2_ID_PAYLOAD_TYPE(9i32);
pub const IKEV2_ID_PAYLOAD_TYPE_DER_ASN1_GN: IKEV2_ID_PAYLOAD_TYPE = IKEV2_ID_PAYLOAD_TYPE(10i32);
pub const IKEV2_ID_PAYLOAD_TYPE_KEY_ID: IKEV2_ID_PAYLOAD_TYPE = IKEV2_ID_PAYLOAD_TYPE(11i32);
pub const IKEV2_ID_PAYLOAD_TYPE_MAX: IKEV2_ID_PAYLOAD_TYPE = IKEV2_ID_PAYLOAD_TYPE(12i32);
impl ::std::convert::From<i32> for IKEV2_ID_PAYLOAD_TYPE {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for IKEV2_ID_PAYLOAD_TYPE {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
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
impl IKEV2_PROJECTION_INFO {}
impl ::std::default::Default for IKEV2_PROJECTION_INFO {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for IKEV2_PROJECTION_INFO {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("IKEV2_PROJECTION_INFO")
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
impl ::std::cmp::PartialEq for IKEV2_PROJECTION_INFO {
    fn eq(&self, other: &Self) -> bool {
        self.dwIPv4NegotiationError == other.dwIPv4NegotiationError
            && self.wszAddress == other.wszAddress
            && self.wszRemoteAddress == other.wszRemoteAddress
            && self.IPv4SubInterfaceIndex == other.IPv4SubInterfaceIndex
            && self.dwIPv6NegotiationError == other.dwIPv6NegotiationError
            && self.bInterfaceIdentifier == other.bInterfaceIdentifier
            && self.bRemoteInterfaceIdentifier == other.bRemoteInterfaceIdentifier
            && self.bPrefix == other.bPrefix
            && self.dwPrefixLength == other.dwPrefixLength
            && self.IPv6SubInterfaceIndex == other.IPv6SubInterfaceIndex
            && self.dwOptions == other.dwOptions
            && self.dwAuthenticationProtocol == other.dwAuthenticationProtocol
            && self.dwEapTypeId == other.dwEapTypeId
            && self.dwCompressionAlgorithm == other.dwCompressionAlgorithm
            && self.dwEncryptionMethod == other.dwEncryptionMethod
    }
}
impl ::std::cmp::Eq for IKEV2_PROJECTION_INFO {}
unsafe impl ::windows::runtime::Abi for IKEV2_PROJECTION_INFO {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
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
impl IKEV2_PROJECTION_INFO2 {}
impl ::std::default::Default for IKEV2_PROJECTION_INFO2 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for IKEV2_PROJECTION_INFO2 {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("IKEV2_PROJECTION_INFO2")
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
impl ::std::cmp::PartialEq for IKEV2_PROJECTION_INFO2 {
    fn eq(&self, other: &Self) -> bool {
        self.dwIPv4NegotiationError == other.dwIPv4NegotiationError
            && self.wszAddress == other.wszAddress
            && self.wszRemoteAddress == other.wszRemoteAddress
            && self.IPv4SubInterfaceIndex == other.IPv4SubInterfaceIndex
            && self.dwIPv6NegotiationError == other.dwIPv6NegotiationError
            && self.bInterfaceIdentifier == other.bInterfaceIdentifier
            && self.bRemoteInterfaceIdentifier == other.bRemoteInterfaceIdentifier
            && self.bPrefix == other.bPrefix
            && self.dwPrefixLength == other.dwPrefixLength
            && self.IPv6SubInterfaceIndex == other.IPv6SubInterfaceIndex
            && self.dwOptions == other.dwOptions
            && self.dwAuthenticationProtocol == other.dwAuthenticationProtocol
            && self.dwEapTypeId == other.dwEapTypeId
            && self.dwEmbeddedEAPTypeId == other.dwEmbeddedEAPTypeId
            && self.dwCompressionAlgorithm == other.dwCompressionAlgorithm
            && self.dwEncryptionMethod == other.dwEncryptionMethod
    }
}
impl ::std::cmp::Eq for IKEV2_PROJECTION_INFO2 {}
unsafe impl ::windows::runtime::Abi for IKEV2_PROJECTION_INFO2 {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
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
impl IKEV2_TUNNEL_CONFIG_PARAMS2 {}
#[cfg(feature = "Win32_Security_Cryptography")]
impl ::std::default::Default for IKEV2_TUNNEL_CONFIG_PARAMS2 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Security_Cryptography")]
impl ::std::fmt::Debug for IKEV2_TUNNEL_CONFIG_PARAMS2 {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("IKEV2_TUNNEL_CONFIG_PARAMS2")
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
impl ::std::cmp::PartialEq for IKEV2_TUNNEL_CONFIG_PARAMS2 {
    fn eq(&self, other: &Self) -> bool {
        self.dwIdleTimeout == other.dwIdleTimeout
            && self.dwNetworkBlackoutTime == other.dwNetworkBlackoutTime
            && self.dwSaLifeTime == other.dwSaLifeTime
            && self.dwSaDataSizeForRenegotiation == other.dwSaDataSizeForRenegotiation
            && self.dwConfigOptions == other.dwConfigOptions
            && self.dwTotalCertificates == other.dwTotalCertificates
            && self.certificateNames == other.certificateNames
            && self.machineCertificateName == other.machineCertificateName
            && self.dwEncryptionType == other.dwEncryptionType
            && self.customPolicy == other.customPolicy
    }
}
#[cfg(feature = "Win32_Security_Cryptography")]
impl ::std::cmp::Eq for IKEV2_TUNNEL_CONFIG_PARAMS2 {}
#[cfg(feature = "Win32_Security_Cryptography")]
unsafe impl ::windows::runtime::Abi for IKEV2_TUNNEL_CONFIG_PARAMS2 {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
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
impl IKEV2_TUNNEL_CONFIG_PARAMS3 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography"))]
impl ::std::default::Default for IKEV2_TUNNEL_CONFIG_PARAMS3 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography"))]
impl ::std::fmt::Debug for IKEV2_TUNNEL_CONFIG_PARAMS3 {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("IKEV2_TUNNEL_CONFIG_PARAMS3")
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
impl ::std::cmp::PartialEq for IKEV2_TUNNEL_CONFIG_PARAMS3 {
    fn eq(&self, other: &Self) -> bool {
        self.dwIdleTimeout == other.dwIdleTimeout
            && self.dwNetworkBlackoutTime == other.dwNetworkBlackoutTime
            && self.dwSaLifeTime == other.dwSaLifeTime
            && self.dwSaDataSizeForRenegotiation == other.dwSaDataSizeForRenegotiation
            && self.dwConfigOptions == other.dwConfigOptions
            && self.dwTotalCertificates == other.dwTotalCertificates
            && self.certificateNames == other.certificateNames
            && self.machineCertificateName == other.machineCertificateName
            && self.dwEncryptionType == other.dwEncryptionType
            && self.customPolicy == other.customPolicy
            && self.dwTotalEkus == other.dwTotalEkus
            && self.certificateEKUs == other.certificateEKUs
            && self.machineCertificateHash == other.machineCertificateHash
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography"))]
impl ::std::cmp::Eq for IKEV2_TUNNEL_CONFIG_PARAMS3 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography"))]
unsafe impl ::windows::runtime::Abi for IKEV2_TUNNEL_CONFIG_PARAMS3 {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
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
impl IKEV2_TUNNEL_CONFIG_PARAMS4 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography"))]
impl ::std::default::Default for IKEV2_TUNNEL_CONFIG_PARAMS4 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography"))]
impl ::std::fmt::Debug for IKEV2_TUNNEL_CONFIG_PARAMS4 {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("IKEV2_TUNNEL_CONFIG_PARAMS4")
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
impl ::std::cmp::PartialEq for IKEV2_TUNNEL_CONFIG_PARAMS4 {
    fn eq(&self, other: &Self) -> bool {
        self.dwIdleTimeout == other.dwIdleTimeout
            && self.dwNetworkBlackoutTime == other.dwNetworkBlackoutTime
            && self.dwSaLifeTime == other.dwSaLifeTime
            && self.dwSaDataSizeForRenegotiation == other.dwSaDataSizeForRenegotiation
            && self.dwConfigOptions == other.dwConfigOptions
            && self.dwTotalCertificates == other.dwTotalCertificates
            && self.certificateNames == other.certificateNames
            && self.machineCertificateName == other.machineCertificateName
            && self.dwEncryptionType == other.dwEncryptionType
            && self.customPolicy == other.customPolicy
            && self.dwTotalEkus == other.dwTotalEkus
            && self.certificateEKUs == other.certificateEKUs
            && self.machineCertificateHash == other.machineCertificateHash
            && self.dwMmSaLifeTime == other.dwMmSaLifeTime
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography"))]
impl ::std::cmp::Eq for IKEV2_TUNNEL_CONFIG_PARAMS4 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography"))]
unsafe impl ::windows::runtime::Abi for IKEV2_TUNNEL_CONFIG_PARAMS4 {
    type Abi = Self;
    type DefaultType = Self;
}
pub const IPADDRESSLEN: u32 = 15u32;
pub const IPV6_ADDRESS_LEN_IN_BYTES: u32 = 16u32;
pub const IPXADDRESSLEN: u32 = 22u32;
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
pub struct L2TP_CONFIG_PARAMS0 {
    pub dwNumPorts: u32,
    pub dwPortFlags: u32,
}
impl L2TP_CONFIG_PARAMS0 {}
impl ::std::default::Default for L2TP_CONFIG_PARAMS0 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for L2TP_CONFIG_PARAMS0 {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("L2TP_CONFIG_PARAMS0").field("dwNumPorts", &self.dwNumPorts).field("dwPortFlags", &self.dwPortFlags).finish()
    }
}
impl ::std::cmp::PartialEq for L2TP_CONFIG_PARAMS0 {
    fn eq(&self, other: &Self) -> bool {
        self.dwNumPorts == other.dwNumPorts && self.dwPortFlags == other.dwPortFlags
    }
}
impl ::std::cmp::Eq for L2TP_CONFIG_PARAMS0 {}
unsafe impl ::windows::runtime::Abi for L2TP_CONFIG_PARAMS0 {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
pub struct L2TP_CONFIG_PARAMS1 {
    pub dwNumPorts: u32,
    pub dwPortFlags: u32,
    pub dwTunnelConfigParamFlags: u32,
    pub TunnelConfigParams: L2TP_TUNNEL_CONFIG_PARAMS2,
}
impl L2TP_CONFIG_PARAMS1 {}
impl ::std::default::Default for L2TP_CONFIG_PARAMS1 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for L2TP_CONFIG_PARAMS1 {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("L2TP_CONFIG_PARAMS1").field("dwNumPorts", &self.dwNumPorts).field("dwPortFlags", &self.dwPortFlags).field("dwTunnelConfigParamFlags", &self.dwTunnelConfigParamFlags).field("TunnelConfigParams", &self.TunnelConfigParams).finish()
    }
}
impl ::std::cmp::PartialEq for L2TP_CONFIG_PARAMS1 {
    fn eq(&self, other: &Self) -> bool {
        self.dwNumPorts == other.dwNumPorts && self.dwPortFlags == other.dwPortFlags && self.dwTunnelConfigParamFlags == other.dwTunnelConfigParamFlags && self.TunnelConfigParams == other.TunnelConfigParams
    }
}
impl ::std::cmp::Eq for L2TP_CONFIG_PARAMS1 {}
unsafe impl ::windows::runtime::Abi for L2TP_CONFIG_PARAMS1 {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
pub struct L2TP_TUNNEL_CONFIG_PARAMS1 {
    pub dwIdleTimeout: u32,
    pub dwEncryptionType: u32,
    pub dwSaLifeTime: u32,
    pub dwSaDataSizeForRenegotiation: u32,
    pub customPolicy: *mut ROUTER_CUSTOM_IKEv2_POLICY0,
}
impl L2TP_TUNNEL_CONFIG_PARAMS1 {}
impl ::std::default::Default for L2TP_TUNNEL_CONFIG_PARAMS1 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for L2TP_TUNNEL_CONFIG_PARAMS1 {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("L2TP_TUNNEL_CONFIG_PARAMS1")
            .field("dwIdleTimeout", &self.dwIdleTimeout)
            .field("dwEncryptionType", &self.dwEncryptionType)
            .field("dwSaLifeTime", &self.dwSaLifeTime)
            .field("dwSaDataSizeForRenegotiation", &self.dwSaDataSizeForRenegotiation)
            .field("customPolicy", &self.customPolicy)
            .finish()
    }
}
impl ::std::cmp::PartialEq for L2TP_TUNNEL_CONFIG_PARAMS1 {
    fn eq(&self, other: &Self) -> bool {
        self.dwIdleTimeout == other.dwIdleTimeout && self.dwEncryptionType == other.dwEncryptionType && self.dwSaLifeTime == other.dwSaLifeTime && self.dwSaDataSizeForRenegotiation == other.dwSaDataSizeForRenegotiation && self.customPolicy == other.customPolicy
    }
}
impl ::std::cmp::Eq for L2TP_TUNNEL_CONFIG_PARAMS1 {}
unsafe impl ::windows::runtime::Abi for L2TP_TUNNEL_CONFIG_PARAMS1 {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
pub struct L2TP_TUNNEL_CONFIG_PARAMS2 {
    pub dwIdleTimeout: u32,
    pub dwEncryptionType: u32,
    pub dwSaLifeTime: u32,
    pub dwSaDataSizeForRenegotiation: u32,
    pub customPolicy: *mut ROUTER_CUSTOM_IKEv2_POLICY0,
    pub dwMmSaLifeTime: u32,
}
impl L2TP_TUNNEL_CONFIG_PARAMS2 {}
impl ::std::default::Default for L2TP_TUNNEL_CONFIG_PARAMS2 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for L2TP_TUNNEL_CONFIG_PARAMS2 {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("L2TP_TUNNEL_CONFIG_PARAMS2")
            .field("dwIdleTimeout", &self.dwIdleTimeout)
            .field("dwEncryptionType", &self.dwEncryptionType)
            .field("dwSaLifeTime", &self.dwSaLifeTime)
            .field("dwSaDataSizeForRenegotiation", &self.dwSaDataSizeForRenegotiation)
            .field("customPolicy", &self.customPolicy)
            .field("dwMmSaLifeTime", &self.dwMmSaLifeTime)
            .finish()
    }
}
impl ::std::cmp::PartialEq for L2TP_TUNNEL_CONFIG_PARAMS2 {
    fn eq(&self, other: &Self) -> bool {
        self.dwIdleTimeout == other.dwIdleTimeout && self.dwEncryptionType == other.dwEncryptionType && self.dwSaLifeTime == other.dwSaLifeTime && self.dwSaDataSizeForRenegotiation == other.dwSaDataSizeForRenegotiation && self.customPolicy == other.customPolicy && self.dwMmSaLifeTime == other.dwMmSaLifeTime
    }
}
impl ::std::cmp::Eq for L2TP_TUNNEL_CONFIG_PARAMS2 {}
unsafe impl ::windows::runtime::Abi for L2TP_TUNNEL_CONFIG_PARAMS2 {
    type Abi = Self;
    type DefaultType = Self;
}
pub const MAXIPADRESSLEN: u32 = 64u32;
pub const MAX_SSTP_HASH_SIZE: u32 = 32u32;
pub const METHOD_BGP4_AS_PATH: u32 = 1u32;
pub const METHOD_BGP4_NEXTHOP_ATTR: u32 = 8u32;
pub const METHOD_BGP4_PA_ORIGIN: u32 = 4u32;
pub const METHOD_BGP4_PEER_ID: u32 = 2u32;
pub const METHOD_RIP2_NEIGHBOUR_ADDR: u32 = 1u32;
pub const METHOD_RIP2_OUTBOUND_INTF: u32 = 2u32;
pub const METHOD_RIP2_ROUTE_TAG: u32 = 4u32;
pub const METHOD_RIP2_ROUTE_TIMESTAMP: u32 = 8u32;
pub const METHOD_TYPE_ALL_METHODS: u32 = 4294967295u32;
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct MGM_ENUM_TYPES(pub i32);
pub const ANY_SOURCE: MGM_ENUM_TYPES = MGM_ENUM_TYPES(0i32);
pub const ALL_SOURCES: MGM_ENUM_TYPES = MGM_ENUM_TYPES(1i32);
impl ::std::convert::From<i32> for MGM_ENUM_TYPES {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for MGM_ENUM_TYPES {
    type Abi = Self;
    type DefaultType = Self;
}
pub const MGM_FORWARD_STATE_FLAG: u32 = 2u32;
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct MGM_IF_ENTRY {
    pub dwIfIndex: u32,
    pub dwIfNextHopAddr: u32,
    pub bIGMP: super::super::Foundation::BOOL,
    pub bIsEnabled: super::super::Foundation::BOOL,
}
#[cfg(feature = "Win32_Foundation")]
impl MGM_IF_ENTRY {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for MGM_IF_ENTRY {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::fmt::Debug for MGM_IF_ENTRY {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("MGM_IF_ENTRY").field("dwIfIndex", &self.dwIfIndex).field("dwIfNextHopAddr", &self.dwIfNextHopAddr).field("bIGMP", &self.bIGMP).field("bIsEnabled", &self.bIsEnabled).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for MGM_IF_ENTRY {
    fn eq(&self, other: &Self) -> bool {
        self.dwIfIndex == other.dwIfIndex && self.dwIfNextHopAddr == other.dwIfNextHopAddr && self.bIGMP == other.bIGMP && self.bIsEnabled == other.bIsEnabled
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for MGM_IF_ENTRY {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for MGM_IF_ENTRY {
    type Abi = Self;
    type DefaultType = Self;
}
pub const MGM_JOIN_STATE_FLAG: u32 = 1u32;
pub const MGM_MFE_STATS_0: u32 = 1u32;
pub const MGM_MFE_STATS_1: u32 = 2u32;
#[derive(:: std :: clone :: Clone)]
#[repr(C)]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
pub struct MPRAPI_ADMIN_DLL_CALLBACKS {
    pub revision: u8,
    pub lpfnMprAdminGetIpAddressForUser: ::std::option::Option<PMPRADMINGETIPADDRESSFORUSER>,
    pub lpfnMprAdminReleaseIpAddress: ::std::option::Option<PMPRADMINRELEASEIPADRESS>,
    pub lpfnMprAdminGetIpv6AddressForUser: ::std::option::Option<PMPRADMINGETIPV6ADDRESSFORUSER>,
    pub lpfnMprAdminReleaseIpV6AddressForUser: ::std::option::Option<PMPRADMINRELEASEIPV6ADDRESSFORUSER>,
    pub lpfnRasAdminAcceptNewLink: ::std::option::Option<PMPRADMINACCEPTNEWLINK>,
    pub lpfnRasAdminLinkHangupNotification: ::std::option::Option<PMPRADMINLINKHANGUPNOTIFICATION>,
    pub lpfnRasAdminTerminateDll: ::std::option::Option<PMPRADMINTERMINATEDLL>,
    pub lpfnRasAdminAcceptNewConnectionEx: ::std::option::Option<PMPRADMINACCEPTNEWCONNECTIONEX>,
    pub lpfnRasAdminAcceptEndpointChangeEx: ::std::option::Option<PMPRADMINACCEPTTUNNELENDPOINTCHANGEEX>,
    pub lpfnRasAdminAcceptReauthenticationEx: ::std::option::Option<PMPRADMINACCEPTREAUTHENTICATIONEX>,
    pub lpfnRasAdminConnectionHangupNotificationEx: ::std::option::Option<PMPRADMINCONNECTIONHANGUPNOTIFICATIONEX>,
    pub lpfnRASValidatePreAuthenticatedConnectionEx: ::std::option::Option<PMPRADMINRASVALIDATEPREAUTHENTICATEDCONNECTIONEX>,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
impl MPRAPI_ADMIN_DLL_CALLBACKS {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
impl ::std::default::Default for MPRAPI_ADMIN_DLL_CALLBACKS {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
impl ::std::fmt::Debug for MPRAPI_ADMIN_DLL_CALLBACKS {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("MPRAPI_ADMIN_DLL_CALLBACKS").field("revision", &self.revision).finish()
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
impl ::std::cmp::PartialEq for MPRAPI_ADMIN_DLL_CALLBACKS {
    fn eq(&self, other: &Self) -> bool {
        self.revision == other.revision
            && self.lpfnMprAdminGetIpAddressForUser.map(|f| f as usize) == other.lpfnMprAdminGetIpAddressForUser.map(|f| f as usize)
            && self.lpfnMprAdminReleaseIpAddress.map(|f| f as usize) == other.lpfnMprAdminReleaseIpAddress.map(|f| f as usize)
            && self.lpfnMprAdminGetIpv6AddressForUser.map(|f| f as usize) == other.lpfnMprAdminGetIpv6AddressForUser.map(|f| f as usize)
            && self.lpfnMprAdminReleaseIpV6AddressForUser.map(|f| f as usize) == other.lpfnMprAdminReleaseIpV6AddressForUser.map(|f| f as usize)
            && self.lpfnRasAdminAcceptNewLink.map(|f| f as usize) == other.lpfnRasAdminAcceptNewLink.map(|f| f as usize)
            && self.lpfnRasAdminLinkHangupNotification.map(|f| f as usize) == other.lpfnRasAdminLinkHangupNotification.map(|f| f as usize)
            && self.lpfnRasAdminTerminateDll.map(|f| f as usize) == other.lpfnRasAdminTerminateDll.map(|f| f as usize)
            && self.lpfnRasAdminAcceptNewConnectionEx.map(|f| f as usize) == other.lpfnRasAdminAcceptNewConnectionEx.map(|f| f as usize)
            && self.lpfnRasAdminAcceptEndpointChangeEx.map(|f| f as usize) == other.lpfnRasAdminAcceptEndpointChangeEx.map(|f| f as usize)
            && self.lpfnRasAdminAcceptReauthenticationEx.map(|f| f as usize) == other.lpfnRasAdminAcceptReauthenticationEx.map(|f| f as usize)
            && self.lpfnRasAdminConnectionHangupNotificationEx.map(|f| f as usize) == other.lpfnRasAdminConnectionHangupNotificationEx.map(|f| f as usize)
            && self.lpfnRASValidatePreAuthenticatedConnectionEx.map(|f| f as usize) == other.lpfnRASValidatePreAuthenticatedConnectionEx.map(|f| f as usize)
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
impl ::std::cmp::Eq for MPRAPI_ADMIN_DLL_CALLBACKS {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
unsafe impl ::windows::runtime::Abi for MPRAPI_ADMIN_DLL_CALLBACKS {
    type Abi = ::std::mem::ManuallyDrop<Self>;
    type DefaultType = Self;
}
pub const MPRAPI_ADMIN_DLL_VERSION_1: u32 = 1u32;
pub const MPRAPI_ADMIN_DLL_VERSION_2: u32 = 2u32;
pub const MPRAPI_IF_CUSTOM_CONFIG_FOR_IKEV2: u32 = 1u32;
pub const MPRAPI_IKEV2_AUTH_USING_CERT: u32 = 1u32;
pub const MPRAPI_IKEV2_AUTH_USING_EAP: u32 = 2u32;
pub const MPRAPI_IKEV2_PROJECTION_INFO_TYPE: u32 = 2u32;
pub const MPRAPI_IKEV2_SET_TUNNEL_CONFIG_PARAMS: u32 = 1u32;
pub const MPRAPI_L2TP_SET_TUNNEL_CONFIG_PARAMS: u32 = 1u32;
pub const MPRAPI_MPR_IF_CUSTOM_CONFIG_OBJECT_REVISION_1: u32 = 1u32;
pub const MPRAPI_MPR_IF_CUSTOM_CONFIG_OBJECT_REVISION_2: u32 = 2u32;
pub const MPRAPI_MPR_IF_CUSTOM_CONFIG_OBJECT_REVISION_3: u32 = 3u32;
pub const MPRAPI_MPR_SERVER_OBJECT_REVISION_1: u32 = 1u32;
pub const MPRAPI_MPR_SERVER_OBJECT_REVISION_2: u32 = 2u32;
pub const MPRAPI_MPR_SERVER_OBJECT_REVISION_3: u32 = 3u32;
pub const MPRAPI_MPR_SERVER_OBJECT_REVISION_4: u32 = 4u32;
pub const MPRAPI_MPR_SERVER_OBJECT_REVISION_5: u32 = 5u32;
pub const MPRAPI_MPR_SERVER_SET_CONFIG_OBJECT_REVISION_1: u32 = 1u32;
pub const MPRAPI_MPR_SERVER_SET_CONFIG_OBJECT_REVISION_2: u32 = 2u32;
pub const MPRAPI_MPR_SERVER_SET_CONFIG_OBJECT_REVISION_3: u32 = 3u32;
pub const MPRAPI_MPR_SERVER_SET_CONFIG_OBJECT_REVISION_4: u32 = 4u32;
pub const MPRAPI_MPR_SERVER_SET_CONFIG_OBJECT_REVISION_5: u32 = 5u32;
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
pub struct MPRAPI_OBJECT_HEADER {
    pub revision: u8,
    pub r#type: u8,
    pub size: u16,
}
impl MPRAPI_OBJECT_HEADER {}
impl ::std::default::Default for MPRAPI_OBJECT_HEADER {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for MPRAPI_OBJECT_HEADER {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("MPRAPI_OBJECT_HEADER").field("revision", &self.revision).field("r#type", &self.r#type).field("size", &self.size).finish()
    }
}
impl ::std::cmp::PartialEq for MPRAPI_OBJECT_HEADER {
    fn eq(&self, other: &Self) -> bool {
        self.revision == other.revision && self.r#type == other.r#type && self.size == other.size
    }
}
impl ::std::cmp::Eq for MPRAPI_OBJECT_HEADER {}
unsafe impl ::windows::runtime::Abi for MPRAPI_OBJECT_HEADER {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct MPRAPI_OBJECT_TYPE(pub i32);
pub const MPRAPI_OBJECT_TYPE_RAS_CONNECTION_OBJECT: MPRAPI_OBJECT_TYPE = MPRAPI_OBJECT_TYPE(1i32);
pub const MPRAPI_OBJECT_TYPE_MPR_SERVER_OBJECT: MPRAPI_OBJECT_TYPE = MPRAPI_OBJECT_TYPE(2i32);
pub const MPRAPI_OBJECT_TYPE_MPR_SERVER_SET_CONFIG_OBJECT: MPRAPI_OBJECT_TYPE = MPRAPI_OBJECT_TYPE(3i32);
pub const MPRAPI_OBJECT_TYPE_AUTH_VALIDATION_OBJECT: MPRAPI_OBJECT_TYPE = MPRAPI_OBJECT_TYPE(4i32);
pub const MPRAPI_OBJECT_TYPE_UPDATE_CONNECTION_OBJECT: MPRAPI_OBJECT_TYPE = MPRAPI_OBJECT_TYPE(5i32);
pub const MPRAPI_OBJECT_TYPE_IF_CUSTOM_CONFIG_OBJECT: MPRAPI_OBJECT_TYPE = MPRAPI_OBJECT_TYPE(6i32);
impl ::std::convert::From<i32> for MPRAPI_OBJECT_TYPE {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for MPRAPI_OBJECT_TYPE {
    type Abi = Self;
    type DefaultType = Self;
}
pub const MPRAPI_PPP_PROJECTION_INFO_TYPE: u32 = 1u32;
pub const MPRAPI_RAS_CONNECTION_OBJECT_REVISION_1: u32 = 1u32;
pub const MPRAPI_RAS_UPDATE_CONNECTION_OBJECT_REVISION_1: u32 = 1u32;
pub const MPRAPI_SET_CONFIG_PROTOCOL_FOR_GRE: u32 = 16u32;
pub const MPRAPI_SET_CONFIG_PROTOCOL_FOR_IKEV2: u32 = 8u32;
pub const MPRAPI_SET_CONFIG_PROTOCOL_FOR_L2TP: u32 = 2u32;
pub const MPRAPI_SET_CONFIG_PROTOCOL_FOR_PPTP: u32 = 1u32;
pub const MPRAPI_SET_CONFIG_PROTOCOL_FOR_SSTP: u32 = 4u32;
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography"))]
pub struct MPRAPI_TUNNEL_CONFIG_PARAMS0 {
    pub IkeConfigParams: IKEV2_CONFIG_PARAMS,
    pub PptpConfigParams: PPTP_CONFIG_PARAMS,
    pub L2tpConfigParams: L2TP_CONFIG_PARAMS1,
    pub SstpConfigParams: SSTP_CONFIG_PARAMS,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography"))]
impl MPRAPI_TUNNEL_CONFIG_PARAMS0 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography"))]
impl ::std::default::Default for MPRAPI_TUNNEL_CONFIG_PARAMS0 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography"))]
impl ::std::fmt::Debug for MPRAPI_TUNNEL_CONFIG_PARAMS0 {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("MPRAPI_TUNNEL_CONFIG_PARAMS0").field("IkeConfigParams", &self.IkeConfigParams).field("PptpConfigParams", &self.PptpConfigParams).field("L2tpConfigParams", &self.L2tpConfigParams).field("SstpConfigParams", &self.SstpConfigParams).finish()
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography"))]
impl ::std::cmp::PartialEq for MPRAPI_TUNNEL_CONFIG_PARAMS0 {
    fn eq(&self, other: &Self) -> bool {
        self.IkeConfigParams == other.IkeConfigParams && self.PptpConfigParams == other.PptpConfigParams && self.L2tpConfigParams == other.L2tpConfigParams && self.SstpConfigParams == other.SstpConfigParams
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography"))]
impl ::std::cmp::Eq for MPRAPI_TUNNEL_CONFIG_PARAMS0 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography"))]
unsafe impl ::windows::runtime::Abi for MPRAPI_TUNNEL_CONFIG_PARAMS0 {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography"))]
pub struct MPRAPI_TUNNEL_CONFIG_PARAMS1 {
    pub IkeConfigParams: IKEV2_CONFIG_PARAMS,
    pub PptpConfigParams: PPTP_CONFIG_PARAMS,
    pub L2tpConfigParams: L2TP_CONFIG_PARAMS1,
    pub SstpConfigParams: SSTP_CONFIG_PARAMS,
    pub GREConfigParams: GRE_CONFIG_PARAMS0,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography"))]
impl MPRAPI_TUNNEL_CONFIG_PARAMS1 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography"))]
impl ::std::default::Default for MPRAPI_TUNNEL_CONFIG_PARAMS1 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography"))]
impl ::std::fmt::Debug for MPRAPI_TUNNEL_CONFIG_PARAMS1 {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("MPRAPI_TUNNEL_CONFIG_PARAMS1")
            .field("IkeConfigParams", &self.IkeConfigParams)
            .field("PptpConfigParams", &self.PptpConfigParams)
            .field("L2tpConfigParams", &self.L2tpConfigParams)
            .field("SstpConfigParams", &self.SstpConfigParams)
            .field("GREConfigParams", &self.GREConfigParams)
            .finish()
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography"))]
impl ::std::cmp::PartialEq for MPRAPI_TUNNEL_CONFIG_PARAMS1 {
    fn eq(&self, other: &Self) -> bool {
        self.IkeConfigParams == other.IkeConfigParams && self.PptpConfigParams == other.PptpConfigParams && self.L2tpConfigParams == other.L2tpConfigParams && self.SstpConfigParams == other.SstpConfigParams && self.GREConfigParams == other.GREConfigParams
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography"))]
impl ::std::cmp::Eq for MPRAPI_TUNNEL_CONFIG_PARAMS1 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography"))]
unsafe impl ::windows::runtime::Abi for MPRAPI_TUNNEL_CONFIG_PARAMS1 {
    type Abi = Self;
    type DefaultType = Self;
}
pub const MPRET_Direct: u32 = 3u32;
pub const MPRET_Phone: u32 = 1u32;
pub const MPRET_Vpn: u32 = 2u32;
pub const MPRIDS_Disabled: u32 = 4294967295u32;
pub const MPRIDS_UseGlobalValue: u32 = 0u32;
pub const MPRIO_DisableLcpExtensions: u32 = 32u32;
pub const MPRIO_IpHeaderCompression: u32 = 8u32;
pub const MPRIO_IpSecPreSharedKey: u32 = 2147483648u32;
pub const MPRIO_NetworkLogon: u32 = 8192u32;
pub const MPRIO_PromoteAlternates: u32 = 32768u32;
pub const MPRIO_RemoteDefaultGateway: u32 = 16u32;
pub const MPRIO_RequireCHAP: u32 = 134217728u32;
pub const MPRIO_RequireDataEncryption: u32 = 4096u32;
pub const MPRIO_RequireEAP: u32 = 131072u32;
pub const MPRIO_RequireEncryptedPw: u32 = 1024u32;
pub const MPRIO_RequireMachineCertificates: u32 = 16777216u32;
pub const MPRIO_RequireMsCHAP: u32 = 268435456u32;
pub const MPRIO_RequireMsCHAP2: u32 = 536870912u32;
pub const MPRIO_RequireMsEncryptedPw: u32 = 2048u32;
pub const MPRIO_RequirePAP: u32 = 262144u32;
pub const MPRIO_RequireSPAP: u32 = 524288u32;
pub const MPRIO_SecureLocalFiles: u32 = 65536u32;
pub const MPRIO_SharedPhoneNumbers: u32 = 8388608u32;
pub const MPRIO_SpecificIpAddr: u32 = 2u32;
pub const MPRIO_SpecificNameServers: u32 = 4u32;
pub const MPRIO_SwCompression: u32 = 512u32;
pub const MPRIO_UsePreSharedKeyForIkev2Initiator: u32 = 33554432u32;
pub const MPRIO_UsePreSharedKeyForIkev2Responder: u32 = 67108864u32;
pub const MPRNP_Ip: u32 = 4u32;
pub const MPRNP_Ipv6: u32 = 8u32;
pub const MPRNP_Ipx: u32 = 2u32;
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct MPR_CERT_EKU {
    pub dwSize: u32,
    pub IsEKUOID: super::super::Foundation::BOOL,
    pub pwszEKU: super::super::Foundation::PWSTR,
}
#[cfg(feature = "Win32_Foundation")]
impl MPR_CERT_EKU {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for MPR_CERT_EKU {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::fmt::Debug for MPR_CERT_EKU {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("MPR_CERT_EKU").field("dwSize", &self.dwSize).field("IsEKUOID", &self.IsEKUOID).field("pwszEKU", &self.pwszEKU).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for MPR_CERT_EKU {
    fn eq(&self, other: &Self) -> bool {
        self.dwSize == other.dwSize && self.IsEKUOID == other.IsEKUOID && self.pwszEKU == other.pwszEKU
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for MPR_CERT_EKU {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for MPR_CERT_EKU {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
pub struct MPR_CREDENTIALSEX_0 {
    pub dwSize: u32,
    pub lpbCredentialsInfo: *mut u8,
}
impl MPR_CREDENTIALSEX_0 {}
impl ::std::default::Default for MPR_CREDENTIALSEX_0 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for MPR_CREDENTIALSEX_0 {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("MPR_CREDENTIALSEX_0").field("dwSize", &self.dwSize).field("lpbCredentialsInfo", &self.lpbCredentialsInfo).finish()
    }
}
impl ::std::cmp::PartialEq for MPR_CREDENTIALSEX_0 {
    fn eq(&self, other: &Self) -> bool {
        self.dwSize == other.dwSize && self.lpbCredentialsInfo == other.lpbCredentialsInfo
    }
}
impl ::std::cmp::Eq for MPR_CREDENTIALSEX_0 {}
unsafe impl ::windows::runtime::Abi for MPR_CREDENTIALSEX_0 {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
pub struct MPR_CREDENTIALSEX_1 {
    pub dwSize: u32,
    pub lpbCredentialsInfo: *mut u8,
}
impl MPR_CREDENTIALSEX_1 {}
impl ::std::default::Default for MPR_CREDENTIALSEX_1 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for MPR_CREDENTIALSEX_1 {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("MPR_CREDENTIALSEX_1").field("dwSize", &self.dwSize).field("lpbCredentialsInfo", &self.lpbCredentialsInfo).finish()
    }
}
impl ::std::cmp::PartialEq for MPR_CREDENTIALSEX_1 {
    fn eq(&self, other: &Self) -> bool {
        self.dwSize == other.dwSize && self.lpbCredentialsInfo == other.lpbCredentialsInfo
    }
}
impl ::std::cmp::Eq for MPR_CREDENTIALSEX_1 {}
unsafe impl ::windows::runtime::Abi for MPR_CREDENTIALSEX_1 {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
pub struct MPR_DEVICE_0 {
    pub szDeviceType: [u16; 17],
    pub szDeviceName: [u16; 129],
}
impl MPR_DEVICE_0 {}
impl ::std::default::Default for MPR_DEVICE_0 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for MPR_DEVICE_0 {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("MPR_DEVICE_0").field("szDeviceType", &self.szDeviceType).field("szDeviceName", &self.szDeviceName).finish()
    }
}
impl ::std::cmp::PartialEq for MPR_DEVICE_0 {
    fn eq(&self, other: &Self) -> bool {
        self.szDeviceType == other.szDeviceType && self.szDeviceName == other.szDeviceName
    }
}
impl ::std::cmp::Eq for MPR_DEVICE_0 {}
unsafe impl ::windows::runtime::Abi for MPR_DEVICE_0 {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct MPR_DEVICE_1 {
    pub szDeviceType: [u16; 17],
    pub szDeviceName: [u16; 129],
    pub szLocalPhoneNumber: [u16; 129],
    pub szAlternates: super::super::Foundation::PWSTR,
}
#[cfg(feature = "Win32_Foundation")]
impl MPR_DEVICE_1 {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for MPR_DEVICE_1 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::fmt::Debug for MPR_DEVICE_1 {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("MPR_DEVICE_1").field("szDeviceType", &self.szDeviceType).field("szDeviceName", &self.szDeviceName).field("szLocalPhoneNumber", &self.szLocalPhoneNumber).field("szAlternates", &self.szAlternates).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for MPR_DEVICE_1 {
    fn eq(&self, other: &Self) -> bool {
        self.szDeviceType == other.szDeviceType && self.szDeviceName == other.szDeviceName && self.szLocalPhoneNumber == other.szLocalPhoneNumber && self.szAlternates == other.szAlternates
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for MPR_DEVICE_1 {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for MPR_DEVICE_1 {
    type Abi = Self;
    type DefaultType = Self;
}
pub const MPR_ENABLE_RAS_ON_DEVICE: u32 = 1u32;
pub const MPR_ENABLE_ROUTING_ON_DEVICE: u32 = 2u32;
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct MPR_ET(pub u32);
pub const MPR_ET_None: MPR_ET = MPR_ET(0u32);
pub const MPR_ET_Require: MPR_ET = MPR_ET(1u32);
pub const MPR_ET_RequireMax: MPR_ET = MPR_ET(2u32);
pub const MPR_ET_Optional: MPR_ET = MPR_ET(3u32);
impl ::std::convert::From<u32> for MPR_ET {
    fn from(value: u32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for MPR_ET {
    type Abi = Self;
    type DefaultType = Self;
}
impl ::std::ops::BitOr for MPR_ET {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self {
        Self(self.0 | rhs.0)
    }
}
impl ::std::ops::BitAnd for MPR_ET {
    type Output = Self;
    fn bitand(self, rhs: Self) -> Self {
        Self(self.0 & rhs.0)
    }
}
impl ::std::ops::BitOrAssign for MPR_ET {
    fn bitor_assign(&mut self, rhs: Self) {
        self.0.bitor_assign(rhs.0)
    }
}
impl ::std::ops::BitAndAssign for MPR_ET {
    fn bitand_assign(&mut self, rhs: Self) {
        self.0.bitand_assign(rhs.0)
    }
}
impl ::std::ops::Not for MPR_ET {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct MPR_FILTER_0 {
    pub fEnable: super::super::Foundation::BOOL,
}
#[cfg(feature = "Win32_Foundation")]
impl MPR_FILTER_0 {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for MPR_FILTER_0 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::fmt::Debug for MPR_FILTER_0 {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("MPR_FILTER_0").field("fEnable", &self.fEnable).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for MPR_FILTER_0 {
    fn eq(&self, other: &Self) -> bool {
        self.fEnable == other.fEnable
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for MPR_FILTER_0 {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for MPR_FILTER_0 {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct MPR_IFTRANSPORT_0 {
    pub dwTransportId: u32,
    pub hIfTransport: super::super::Foundation::HANDLE,
    pub wszIfTransportName: [u16; 41],
}
#[cfg(feature = "Win32_Foundation")]
impl MPR_IFTRANSPORT_0 {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for MPR_IFTRANSPORT_0 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::fmt::Debug for MPR_IFTRANSPORT_0 {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("MPR_IFTRANSPORT_0").field("dwTransportId", &self.dwTransportId).field("hIfTransport", &self.hIfTransport).field("wszIfTransportName", &self.wszIfTransportName).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for MPR_IFTRANSPORT_0 {
    fn eq(&self, other: &Self) -> bool {
        self.dwTransportId == other.dwTransportId && self.hIfTransport == other.hIfTransport && self.wszIfTransportName == other.wszIfTransportName
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for MPR_IFTRANSPORT_0 {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for MPR_IFTRANSPORT_0 {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Security_Cryptography")]
pub struct MPR_IF_CUSTOMINFOEX0 {
    pub Header: MPRAPI_OBJECT_HEADER,
    pub dwFlags: u32,
    pub customIkev2Config: ROUTER_IKEv2_IF_CUSTOM_CONFIG0,
}
#[cfg(feature = "Win32_Security_Cryptography")]
impl MPR_IF_CUSTOMINFOEX0 {}
#[cfg(feature = "Win32_Security_Cryptography")]
impl ::std::default::Default for MPR_IF_CUSTOMINFOEX0 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Security_Cryptography")]
impl ::std::fmt::Debug for MPR_IF_CUSTOMINFOEX0 {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("MPR_IF_CUSTOMINFOEX0").field("Header", &self.Header).field("dwFlags", &self.dwFlags).field("customIkev2Config", &self.customIkev2Config).finish()
    }
}
#[cfg(feature = "Win32_Security_Cryptography")]
impl ::std::cmp::PartialEq for MPR_IF_CUSTOMINFOEX0 {
    fn eq(&self, other: &Self) -> bool {
        self.Header == other.Header && self.dwFlags == other.dwFlags && self.customIkev2Config == other.customIkev2Config
    }
}
#[cfg(feature = "Win32_Security_Cryptography")]
impl ::std::cmp::Eq for MPR_IF_CUSTOMINFOEX0 {}
#[cfg(feature = "Win32_Security_Cryptography")]
unsafe impl ::windows::runtime::Abi for MPR_IF_CUSTOMINFOEX0 {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Security_Cryptography")]
pub struct MPR_IF_CUSTOMINFOEX1 {
    pub Header: MPRAPI_OBJECT_HEADER,
    pub dwFlags: u32,
    pub customIkev2Config: ROUTER_IKEv2_IF_CUSTOM_CONFIG1,
}
#[cfg(feature = "Win32_Security_Cryptography")]
impl MPR_IF_CUSTOMINFOEX1 {}
#[cfg(feature = "Win32_Security_Cryptography")]
impl ::std::default::Default for MPR_IF_CUSTOMINFOEX1 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Security_Cryptography")]
impl ::std::fmt::Debug for MPR_IF_CUSTOMINFOEX1 {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("MPR_IF_CUSTOMINFOEX1").field("Header", &self.Header).field("dwFlags", &self.dwFlags).field("customIkev2Config", &self.customIkev2Config).finish()
    }
}
#[cfg(feature = "Win32_Security_Cryptography")]
impl ::std::cmp::PartialEq for MPR_IF_CUSTOMINFOEX1 {
    fn eq(&self, other: &Self) -> bool {
        self.Header == other.Header && self.dwFlags == other.dwFlags && self.customIkev2Config == other.customIkev2Config
    }
}
#[cfg(feature = "Win32_Security_Cryptography")]
impl ::std::cmp::Eq for MPR_IF_CUSTOMINFOEX1 {}
#[cfg(feature = "Win32_Security_Cryptography")]
unsafe impl ::windows::runtime::Abi for MPR_IF_CUSTOMINFOEX1 {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(all(feature = "Win32_Networking_WinSock", feature = "Win32_Security_Cryptography"))]
pub struct MPR_IF_CUSTOMINFOEX2 {
    pub Header: MPRAPI_OBJECT_HEADER,
    pub dwFlags: u32,
    pub customIkev2Config: ROUTER_IKEv2_IF_CUSTOM_CONFIG2,
}
#[cfg(all(feature = "Win32_Networking_WinSock", feature = "Win32_Security_Cryptography"))]
impl MPR_IF_CUSTOMINFOEX2 {}
#[cfg(all(feature = "Win32_Networking_WinSock", feature = "Win32_Security_Cryptography"))]
impl ::std::default::Default for MPR_IF_CUSTOMINFOEX2 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(all(feature = "Win32_Networking_WinSock", feature = "Win32_Security_Cryptography"))]
impl ::std::fmt::Debug for MPR_IF_CUSTOMINFOEX2 {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("MPR_IF_CUSTOMINFOEX2").field("Header", &self.Header).field("dwFlags", &self.dwFlags).field("customIkev2Config", &self.customIkev2Config).finish()
    }
}
#[cfg(all(feature = "Win32_Networking_WinSock", feature = "Win32_Security_Cryptography"))]
impl ::std::cmp::PartialEq for MPR_IF_CUSTOMINFOEX2 {
    fn eq(&self, other: &Self) -> bool {
        self.Header == other.Header && self.dwFlags == other.dwFlags && self.customIkev2Config == other.customIkev2Config
    }
}
#[cfg(all(feature = "Win32_Networking_WinSock", feature = "Win32_Security_Cryptography"))]
impl ::std::cmp::Eq for MPR_IF_CUSTOMINFOEX2 {}
#[cfg(all(feature = "Win32_Networking_WinSock", feature = "Win32_Security_Cryptography"))]
unsafe impl ::windows::runtime::Abi for MPR_IF_CUSTOMINFOEX2 {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
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
impl MPR_INTERFACE_0 {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for MPR_INTERFACE_0 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::fmt::Debug for MPR_INTERFACE_0 {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("MPR_INTERFACE_0")
            .field("wszInterfaceName", &self.wszInterfaceName)
            .field("hInterface", &self.hInterface)
            .field("fEnabled", &self.fEnabled)
            .field("dwIfType", &self.dwIfType)
            .field("dwConnectionState", &self.dwConnectionState)
            .field("fUnReachabilityReasons", &self.fUnReachabilityReasons)
            .field("dwLastError", &self.dwLastError)
            .finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for MPR_INTERFACE_0 {
    fn eq(&self, other: &Self) -> bool {
        self.wszInterfaceName == other.wszInterfaceName && self.hInterface == other.hInterface && self.fEnabled == other.fEnabled && self.dwIfType == other.dwIfType && self.dwConnectionState == other.dwConnectionState && self.fUnReachabilityReasons == other.fUnReachabilityReasons && self.dwLastError == other.dwLastError
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for MPR_INTERFACE_0 {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for MPR_INTERFACE_0 {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct MPR_INTERFACE_1 {
    pub wszInterfaceName: [u16; 257],
    pub hInterface: super::super::Foundation::HANDLE,
    pub fEnabled: super::super::Foundation::BOOL,
    pub dwIfType: ROUTER_INTERFACE_TYPE,
    pub dwConnectionState: ROUTER_CONNECTION_STATE,
    pub fUnReachabilityReasons: u32,
    pub dwLastError: u32,
    pub lpwsDialoutHoursRestriction: super::super::Foundation::PWSTR,
}
#[cfg(feature = "Win32_Foundation")]
impl MPR_INTERFACE_1 {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for MPR_INTERFACE_1 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::fmt::Debug for MPR_INTERFACE_1 {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("MPR_INTERFACE_1")
            .field("wszInterfaceName", &self.wszInterfaceName)
            .field("hInterface", &self.hInterface)
            .field("fEnabled", &self.fEnabled)
            .field("dwIfType", &self.dwIfType)
            .field("dwConnectionState", &self.dwConnectionState)
            .field("fUnReachabilityReasons", &self.fUnReachabilityReasons)
            .field("dwLastError", &self.dwLastError)
            .field("lpwsDialoutHoursRestriction", &self.lpwsDialoutHoursRestriction)
            .finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for MPR_INTERFACE_1 {
    fn eq(&self, other: &Self) -> bool {
        self.wszInterfaceName == other.wszInterfaceName && self.hInterface == other.hInterface && self.fEnabled == other.fEnabled && self.dwIfType == other.dwIfType && self.dwConnectionState == other.dwConnectionState && self.fUnReachabilityReasons == other.fUnReachabilityReasons && self.dwLastError == other.dwLastError && self.lpwsDialoutHoursRestriction == other.lpwsDialoutHoursRestriction
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for MPR_INTERFACE_1 {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for MPR_INTERFACE_1 {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
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
    pub szAlternates: super::super::Foundation::PWSTR,
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
    pub guidId: ::windows::runtime::GUID,
    pub dwVpnStrategy: MPR_VS,
}
#[cfg(feature = "Win32_Foundation")]
impl MPR_INTERFACE_2 {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for MPR_INTERFACE_2 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::fmt::Debug for MPR_INTERFACE_2 {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("MPR_INTERFACE_2")
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
impl ::std::cmp::PartialEq for MPR_INTERFACE_2 {
    fn eq(&self, other: &Self) -> bool {
        self.wszInterfaceName == other.wszInterfaceName
            && self.hInterface == other.hInterface
            && self.fEnabled == other.fEnabled
            && self.dwIfType == other.dwIfType
            && self.dwConnectionState == other.dwConnectionState
            && self.fUnReachabilityReasons == other.fUnReachabilityReasons
            && self.dwLastError == other.dwLastError
            && self.dwfOptions == other.dwfOptions
            && self.szLocalPhoneNumber == other.szLocalPhoneNumber
            && self.szAlternates == other.szAlternates
            && self.ipaddr == other.ipaddr
            && self.ipaddrDns == other.ipaddrDns
            && self.ipaddrDnsAlt == other.ipaddrDnsAlt
            && self.ipaddrWins == other.ipaddrWins
            && self.ipaddrWinsAlt == other.ipaddrWinsAlt
            && self.dwfNetProtocols == other.dwfNetProtocols
            && self.szDeviceType == other.szDeviceType
            && self.szDeviceName == other.szDeviceName
            && self.szX25PadType == other.szX25PadType
            && self.szX25Address == other.szX25Address
            && self.szX25Facilities == other.szX25Facilities
            && self.szX25UserData == other.szX25UserData
            && self.dwChannels == other.dwChannels
            && self.dwSubEntries == other.dwSubEntries
            && self.dwDialMode == other.dwDialMode
            && self.dwDialExtraPercent == other.dwDialExtraPercent
            && self.dwDialExtraSampleSeconds == other.dwDialExtraSampleSeconds
            && self.dwHangUpExtraPercent == other.dwHangUpExtraPercent
            && self.dwHangUpExtraSampleSeconds == other.dwHangUpExtraSampleSeconds
            && self.dwIdleDisconnectSeconds == other.dwIdleDisconnectSeconds
            && self.dwType == other.dwType
            && self.dwEncryptionType == other.dwEncryptionType
            && self.dwCustomAuthKey == other.dwCustomAuthKey
            && self.dwCustomAuthDataSize == other.dwCustomAuthDataSize
            && self.lpbCustomAuthData == other.lpbCustomAuthData
            && self.guidId == other.guidId
            && self.dwVpnStrategy == other.dwVpnStrategy
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for MPR_INTERFACE_2 {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for MPR_INTERFACE_2 {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
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
    pub szAlternates: super::super::Foundation::PWSTR,
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
    pub guidId: ::windows::runtime::GUID,
    pub dwVpnStrategy: MPR_VS,
    pub AddressCount: u32,
    pub ipv6addrDns: super::super::Networking::WinSock::IN6_ADDR,
    pub ipv6addrDnsAlt: super::super::Networking::WinSock::IN6_ADDR,
    pub ipv6addr: *mut super::super::Networking::WinSock::IN6_ADDR,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
impl MPR_INTERFACE_3 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
impl ::std::default::Default for MPR_INTERFACE_3 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
impl ::std::cmp::PartialEq for MPR_INTERFACE_3 {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
impl ::std::cmp::Eq for MPR_INTERFACE_3 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
unsafe impl ::windows::runtime::Abi for MPR_INTERFACE_3 {
    type Abi = Self;
    type DefaultType = Self;
}
pub const MPR_INTERFACE_ADMIN_DISABLED: u32 = 2u32;
pub const MPR_INTERFACE_CONNECTION_FAILURE: u32 = 4u32;
pub const MPR_INTERFACE_DIALOUT_HOURS_RESTRICTION: u32 = 16u32;
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct MPR_INTERFACE_DIAL_MODE(pub u32);
pub const MPRDM_DialFirst: MPR_INTERFACE_DIAL_MODE = MPR_INTERFACE_DIAL_MODE(0u32);
pub const MPRDM_DialAll: MPR_INTERFACE_DIAL_MODE = MPR_INTERFACE_DIAL_MODE(1u32);
pub const MPRDM_DialAsNeeded: MPR_INTERFACE_DIAL_MODE = MPR_INTERFACE_DIAL_MODE(2u32);
impl ::std::convert::From<u32> for MPR_INTERFACE_DIAL_MODE {
    fn from(value: u32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for MPR_INTERFACE_DIAL_MODE {
    type Abi = Self;
    type DefaultType = Self;
}
impl ::std::ops::BitOr for MPR_INTERFACE_DIAL_MODE {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self {
        Self(self.0 | rhs.0)
    }
}
impl ::std::ops::BitAnd for MPR_INTERFACE_DIAL_MODE {
    type Output = Self;
    fn bitand(self, rhs: Self) -> Self {
        Self(self.0 & rhs.0)
    }
}
impl ::std::ops::BitOrAssign for MPR_INTERFACE_DIAL_MODE {
    fn bitor_assign(&mut self, rhs: Self) {
        self.0.bitor_assign(rhs.0)
    }
}
impl ::std::ops::BitAndAssign for MPR_INTERFACE_DIAL_MODE {
    fn bitand_assign(&mut self, rhs: Self) {
        self.0.bitand_assign(rhs.0)
    }
}
impl ::std::ops::Not for MPR_INTERFACE_DIAL_MODE {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
pub const MPR_INTERFACE_NO_DEVICE: u32 = 64u32;
pub const MPR_INTERFACE_NO_MEDIA_SENSE: u32 = 32u32;
pub const MPR_INTERFACE_OUT_OF_RESOURCES: u32 = 1u32;
pub const MPR_INTERFACE_SERVICE_PAUSED: u32 = 8u32;
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
pub struct MPR_IPINIP_INTERFACE_0 {
    pub wszFriendlyName: [u16; 257],
    pub Guid: ::windows::runtime::GUID,
}
impl MPR_IPINIP_INTERFACE_0 {}
impl ::std::default::Default for MPR_IPINIP_INTERFACE_0 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for MPR_IPINIP_INTERFACE_0 {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("MPR_IPINIP_INTERFACE_0").field("wszFriendlyName", &self.wszFriendlyName).field("Guid", &self.Guid).finish()
    }
}
impl ::std::cmp::PartialEq for MPR_IPINIP_INTERFACE_0 {
    fn eq(&self, other: &Self) -> bool {
        self.wszFriendlyName == other.wszFriendlyName && self.Guid == other.Guid
    }
}
impl ::std::cmp::Eq for MPR_IPINIP_INTERFACE_0 {}
unsafe impl ::windows::runtime::Abi for MPR_IPINIP_INTERFACE_0 {
    type Abi = Self;
    type DefaultType = Self;
}
pub const MPR_MaxAreaCode: u32 = 10u32;
pub const MPR_MaxCallbackNumber: u32 = 128u32;
pub const MPR_MaxDeviceName: u32 = 128u32;
pub const MPR_MaxDeviceType: u32 = 16u32;
pub const MPR_MaxEntryName: u32 = 256u32;
pub const MPR_MaxFacilities: u32 = 200u32;
pub const MPR_MaxIpAddress: u32 = 15u32;
pub const MPR_MaxIpxAddress: u32 = 21u32;
pub const MPR_MaxPadType: u32 = 32u32;
pub const MPR_MaxPhoneNumber: u32 = 128u32;
pub const MPR_MaxUserData: u32 = 200u32;
pub const MPR_MaxX25Address: u32 = 200u32;
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct MPR_SERVER_0 {
    pub fLanOnlyMode: super::super::Foundation::BOOL,
    pub dwUpTime: u32,
    pub dwTotalPorts: u32,
    pub dwPortsInUse: u32,
}
#[cfg(feature = "Win32_Foundation")]
impl MPR_SERVER_0 {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for MPR_SERVER_0 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::fmt::Debug for MPR_SERVER_0 {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("MPR_SERVER_0").field("fLanOnlyMode", &self.fLanOnlyMode).field("dwUpTime", &self.dwUpTime).field("dwTotalPorts", &self.dwTotalPorts).field("dwPortsInUse", &self.dwPortsInUse).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for MPR_SERVER_0 {
    fn eq(&self, other: &Self) -> bool {
        self.fLanOnlyMode == other.fLanOnlyMode && self.dwUpTime == other.dwUpTime && self.dwTotalPorts == other.dwTotalPorts && self.dwPortsInUse == other.dwPortsInUse
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for MPR_SERVER_0 {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for MPR_SERVER_0 {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
pub struct MPR_SERVER_1 {
    pub dwNumPptpPorts: u32,
    pub dwPptpPortFlags: u32,
    pub dwNumL2tpPorts: u32,
    pub dwL2tpPortFlags: u32,
}
impl MPR_SERVER_1 {}
impl ::std::default::Default for MPR_SERVER_1 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for MPR_SERVER_1 {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("MPR_SERVER_1").field("dwNumPptpPorts", &self.dwNumPptpPorts).field("dwPptpPortFlags", &self.dwPptpPortFlags).field("dwNumL2tpPorts", &self.dwNumL2tpPorts).field("dwL2tpPortFlags", &self.dwL2tpPortFlags).finish()
    }
}
impl ::std::cmp::PartialEq for MPR_SERVER_1 {
    fn eq(&self, other: &Self) -> bool {
        self.dwNumPptpPorts == other.dwNumPptpPorts && self.dwPptpPortFlags == other.dwPptpPortFlags && self.dwNumL2tpPorts == other.dwNumL2tpPorts && self.dwL2tpPortFlags == other.dwL2tpPortFlags
    }
}
impl ::std::cmp::Eq for MPR_SERVER_1 {}
unsafe impl ::windows::runtime::Abi for MPR_SERVER_1 {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
pub struct MPR_SERVER_2 {
    pub dwNumPptpPorts: u32,
    pub dwPptpPortFlags: u32,
    pub dwNumL2tpPorts: u32,
    pub dwL2tpPortFlags: u32,
    pub dwNumSstpPorts: u32,
    pub dwSstpPortFlags: u32,
}
impl MPR_SERVER_2 {}
impl ::std::default::Default for MPR_SERVER_2 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for MPR_SERVER_2 {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("MPR_SERVER_2")
            .field("dwNumPptpPorts", &self.dwNumPptpPorts)
            .field("dwPptpPortFlags", &self.dwPptpPortFlags)
            .field("dwNumL2tpPorts", &self.dwNumL2tpPorts)
            .field("dwL2tpPortFlags", &self.dwL2tpPortFlags)
            .field("dwNumSstpPorts", &self.dwNumSstpPorts)
            .field("dwSstpPortFlags", &self.dwSstpPortFlags)
            .finish()
    }
}
impl ::std::cmp::PartialEq for MPR_SERVER_2 {
    fn eq(&self, other: &Self) -> bool {
        self.dwNumPptpPorts == other.dwNumPptpPorts && self.dwPptpPortFlags == other.dwPptpPortFlags && self.dwNumL2tpPorts == other.dwNumL2tpPorts && self.dwL2tpPortFlags == other.dwL2tpPortFlags && self.dwNumSstpPorts == other.dwNumSstpPorts && self.dwSstpPortFlags == other.dwSstpPortFlags
    }
}
impl ::std::cmp::Eq for MPR_SERVER_2 {}
unsafe impl ::windows::runtime::Abi for MPR_SERVER_2 {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
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
impl MPR_SERVER_EX0 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography"))]
impl ::std::default::Default for MPR_SERVER_EX0 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography"))]
impl ::std::fmt::Debug for MPR_SERVER_EX0 {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("MPR_SERVER_EX0")
            .field("Header", &self.Header)
            .field("fLanOnlyMode", &self.fLanOnlyMode)
            .field("dwUpTime", &self.dwUpTime)
            .field("dwTotalPorts", &self.dwTotalPorts)
            .field("dwPortsInUse", &self.dwPortsInUse)
            .field("Reserved", &self.Reserved)
            .field("ConfigParams", &self.ConfigParams)
            .finish()
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography"))]
impl ::std::cmp::PartialEq for MPR_SERVER_EX0 {
    fn eq(&self, other: &Self) -> bool {
        self.Header == other.Header && self.fLanOnlyMode == other.fLanOnlyMode && self.dwUpTime == other.dwUpTime && self.dwTotalPorts == other.dwTotalPorts && self.dwPortsInUse == other.dwPortsInUse && self.Reserved == other.Reserved && self.ConfigParams == other.ConfigParams
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography"))]
impl ::std::cmp::Eq for MPR_SERVER_EX0 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography"))]
unsafe impl ::windows::runtime::Abi for MPR_SERVER_EX0 {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
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
impl MPR_SERVER_EX1 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography"))]
impl ::std::default::Default for MPR_SERVER_EX1 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography"))]
impl ::std::fmt::Debug for MPR_SERVER_EX1 {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("MPR_SERVER_EX1")
            .field("Header", &self.Header)
            .field("fLanOnlyMode", &self.fLanOnlyMode)
            .field("dwUpTime", &self.dwUpTime)
            .field("dwTotalPorts", &self.dwTotalPorts)
            .field("dwPortsInUse", &self.dwPortsInUse)
            .field("Reserved", &self.Reserved)
            .field("ConfigParams", &self.ConfigParams)
            .finish()
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography"))]
impl ::std::cmp::PartialEq for MPR_SERVER_EX1 {
    fn eq(&self, other: &Self) -> bool {
        self.Header == other.Header && self.fLanOnlyMode == other.fLanOnlyMode && self.dwUpTime == other.dwUpTime && self.dwTotalPorts == other.dwTotalPorts && self.dwPortsInUse == other.dwPortsInUse && self.Reserved == other.Reserved && self.ConfigParams == other.ConfigParams
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography"))]
impl ::std::cmp::Eq for MPR_SERVER_EX1 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography"))]
unsafe impl ::windows::runtime::Abi for MPR_SERVER_EX1 {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography"))]
pub struct MPR_SERVER_SET_CONFIG_EX0 {
    pub Header: MPRAPI_OBJECT_HEADER,
    pub setConfigForProtocols: u32,
    pub ConfigParams: MPRAPI_TUNNEL_CONFIG_PARAMS0,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography"))]
impl MPR_SERVER_SET_CONFIG_EX0 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography"))]
impl ::std::default::Default for MPR_SERVER_SET_CONFIG_EX0 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography"))]
impl ::std::fmt::Debug for MPR_SERVER_SET_CONFIG_EX0 {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("MPR_SERVER_SET_CONFIG_EX0").field("Header", &self.Header).field("setConfigForProtocols", &self.setConfigForProtocols).field("ConfigParams", &self.ConfigParams).finish()
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography"))]
impl ::std::cmp::PartialEq for MPR_SERVER_SET_CONFIG_EX0 {
    fn eq(&self, other: &Self) -> bool {
        self.Header == other.Header && self.setConfigForProtocols == other.setConfigForProtocols && self.ConfigParams == other.ConfigParams
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography"))]
impl ::std::cmp::Eq for MPR_SERVER_SET_CONFIG_EX0 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography"))]
unsafe impl ::windows::runtime::Abi for MPR_SERVER_SET_CONFIG_EX0 {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography"))]
pub struct MPR_SERVER_SET_CONFIG_EX1 {
    pub Header: MPRAPI_OBJECT_HEADER,
    pub setConfigForProtocols: u32,
    pub ConfigParams: MPRAPI_TUNNEL_CONFIG_PARAMS1,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography"))]
impl MPR_SERVER_SET_CONFIG_EX1 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography"))]
impl ::std::default::Default for MPR_SERVER_SET_CONFIG_EX1 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography"))]
impl ::std::fmt::Debug for MPR_SERVER_SET_CONFIG_EX1 {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("MPR_SERVER_SET_CONFIG_EX1").field("Header", &self.Header).field("setConfigForProtocols", &self.setConfigForProtocols).field("ConfigParams", &self.ConfigParams).finish()
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography"))]
impl ::std::cmp::PartialEq for MPR_SERVER_SET_CONFIG_EX1 {
    fn eq(&self, other: &Self) -> bool {
        self.Header == other.Header && self.setConfigForProtocols == other.setConfigForProtocols && self.ConfigParams == other.ConfigParams
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography"))]
impl ::std::cmp::Eq for MPR_SERVER_SET_CONFIG_EX1 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography"))]
unsafe impl ::windows::runtime::Abi for MPR_SERVER_SET_CONFIG_EX1 {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct MPR_TRANSPORT_0 {
    pub dwTransportId: u32,
    pub hTransport: super::super::Foundation::HANDLE,
    pub wszTransportName: [u16; 41],
}
#[cfg(feature = "Win32_Foundation")]
impl MPR_TRANSPORT_0 {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for MPR_TRANSPORT_0 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::fmt::Debug for MPR_TRANSPORT_0 {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("MPR_TRANSPORT_0").field("dwTransportId", &self.dwTransportId).field("hTransport", &self.hTransport).field("wszTransportName", &self.wszTransportName).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for MPR_TRANSPORT_0 {
    fn eq(&self, other: &Self) -> bool {
        self.dwTransportId == other.dwTransportId && self.hTransport == other.hTransport && self.wszTransportName == other.wszTransportName
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for MPR_TRANSPORT_0 {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for MPR_TRANSPORT_0 {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Networking_WinSock")]
pub struct MPR_VPN_TRAFFIC_SELECTORS {
    pub numTsi: u32,
    pub numTsr: u32,
    pub tsI: *mut _MPR_VPN_SELECTOR,
    pub tsR: *mut _MPR_VPN_SELECTOR,
}
#[cfg(feature = "Win32_Networking_WinSock")]
impl MPR_VPN_TRAFFIC_SELECTORS {}
#[cfg(feature = "Win32_Networking_WinSock")]
impl ::std::default::Default for MPR_VPN_TRAFFIC_SELECTORS {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Networking_WinSock")]
impl ::std::fmt::Debug for MPR_VPN_TRAFFIC_SELECTORS {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("MPR_VPN_TRAFFIC_SELECTORS").field("numTsi", &self.numTsi).field("numTsr", &self.numTsr).field("tsI", &self.tsI).field("tsR", &self.tsR).finish()
    }
}
#[cfg(feature = "Win32_Networking_WinSock")]
impl ::std::cmp::PartialEq for MPR_VPN_TRAFFIC_SELECTORS {
    fn eq(&self, other: &Self) -> bool {
        self.numTsi == other.numTsi && self.numTsr == other.numTsr && self.tsI == other.tsI && self.tsR == other.tsR
    }
}
#[cfg(feature = "Win32_Networking_WinSock")]
impl ::std::cmp::Eq for MPR_VPN_TRAFFIC_SELECTORS {}
#[cfg(feature = "Win32_Networking_WinSock")]
unsafe impl ::windows::runtime::Abi for MPR_VPN_TRAFFIC_SELECTORS {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct MPR_VPN_TS_TYPE(pub i32);
pub const MPR_VPN_TS_IPv4_ADDR_RANGE: MPR_VPN_TS_TYPE = MPR_VPN_TS_TYPE(7i32);
pub const MPR_VPN_TS_IPv6_ADDR_RANGE: MPR_VPN_TS_TYPE = MPR_VPN_TS_TYPE(8i32);
impl ::std::convert::From<i32> for MPR_VPN_TS_TYPE {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for MPR_VPN_TS_TYPE {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct MPR_VS(pub u32);
pub const MPR_VS_Default: MPR_VS = MPR_VS(0u32);
pub const MPR_VS_PptpOnly: MPR_VS = MPR_VS(1u32);
pub const MPR_VS_PptpFirst: MPR_VS = MPR_VS(2u32);
pub const MPR_VS_L2tpOnly: MPR_VS = MPR_VS(3u32);
pub const MPR_VS_L2tpFirst: MPR_VS = MPR_VS(4u32);
impl ::std::convert::From<u32> for MPR_VS {
    fn from(value: u32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for MPR_VS {
    type Abi = Self;
    type DefaultType = Self;
}
impl ::std::ops::BitOr for MPR_VS {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self {
        Self(self.0 | rhs.0)
    }
}
impl ::std::ops::BitAnd for MPR_VS {
    type Output = Self;
    fn bitand(self, rhs: Self) -> Self {
        Self(self.0 & rhs.0)
    }
}
impl ::std::ops::BitOrAssign for MPR_VS {
    fn bitor_assign(&mut self, rhs: Self) {
        self.0.bitor_assign(rhs.0)
    }
}
impl ::std::ops::BitAndAssign for MPR_VS {
    fn bitand_assign(&mut self, rhs: Self) {
        self.0.bitand_assign(rhs.0)
    }
}
impl ::std::ops::Not for MPR_VS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
pub const MPR_VS_Ikev2First: u32 = 8u32;
pub const MPR_VS_Ikev2Only: u32 = 7u32;
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn MgmAddGroupMembershipEntry<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::HANDLE>>(hprotocol: Param0, dwsourceaddr: u32, dwsourcemask: u32, dwgroupaddr: u32, dwgroupmask: u32, dwifindex: u32, dwifnexthopipaddr: u32, dwflags: u32) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn MgmAddGroupMembershipEntry(hprotocol: super::super::Foundation::HANDLE, dwsourceaddr: u32, dwsourcemask: u32, dwgroupaddr: u32, dwgroupmask: u32, dwifindex: u32, dwifnexthopipaddr: u32, dwflags: u32) -> u32;
        }
        ::std::mem::transmute(MgmAddGroupMembershipEntry(
            hprotocol.into_param().abi(),
            ::std::mem::transmute(dwsourceaddr),
            ::std::mem::transmute(dwsourcemask),
            ::std::mem::transmute(dwgroupaddr),
            ::std::mem::transmute(dwgroupmask),
            ::std::mem::transmute(dwifindex),
            ::std::mem::transmute(dwifnexthopipaddr),
            ::std::mem::transmute(dwflags),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn MgmDeRegisterMProtocol<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::HANDLE>>(hprotocol: Param0) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn MgmDeRegisterMProtocol(hprotocol: super::super::Foundation::HANDLE) -> u32;
        }
        ::std::mem::transmute(MgmDeRegisterMProtocol(hprotocol.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn MgmDeleteGroupMembershipEntry<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::HANDLE>>(hprotocol: Param0, dwsourceaddr: u32, dwsourcemask: u32, dwgroupaddr: u32, dwgroupmask: u32, dwifindex: u32, dwifnexthopipaddr: u32, dwflags: u32) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn MgmDeleteGroupMembershipEntry(hprotocol: super::super::Foundation::HANDLE, dwsourceaddr: u32, dwsourcemask: u32, dwgroupaddr: u32, dwgroupmask: u32, dwifindex: u32, dwifnexthopipaddr: u32, dwflags: u32) -> u32;
        }
        ::std::mem::transmute(MgmDeleteGroupMembershipEntry(
            hprotocol.into_param().abi(),
            ::std::mem::transmute(dwsourceaddr),
            ::std::mem::transmute(dwsourcemask),
            ::std::mem::transmute(dwgroupaddr),
            ::std::mem::transmute(dwgroupmask),
            ::std::mem::transmute(dwifindex),
            ::std::mem::transmute(dwifnexthopipaddr),
            ::std::mem::transmute(dwflags),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn MgmGetFirstMfe(pdwbuffersize: *mut u32, pbbuffer: *mut u8, pdwnumentries: *mut u32) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn MgmGetFirstMfe(pdwbuffersize: *mut u32, pbbuffer: *mut u8, pdwnumentries: *mut u32) -> u32;
        }
        ::std::mem::transmute(MgmGetFirstMfe(::std::mem::transmute(pdwbuffersize), ::std::mem::transmute(pbbuffer), ::std::mem::transmute(pdwnumentries)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn MgmGetFirstMfeStats(pdwbuffersize: *mut u32, pbbuffer: *mut u8, pdwnumentries: *mut u32, dwflags: u32) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn MgmGetFirstMfeStats(pdwbuffersize: *mut u32, pbbuffer: *mut u8, pdwnumentries: *mut u32, dwflags: u32) -> u32;
        }
        ::std::mem::transmute(MgmGetFirstMfeStats(::std::mem::transmute(pdwbuffersize), ::std::mem::transmute(pbbuffer), ::std::mem::transmute(pdwnumentries), ::std::mem::transmute(dwflags)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_NetworkManagement_IpHelper")]
#[inline]
pub unsafe fn MgmGetMfe(pimm: *mut super::IpHelper::MIB_IPMCAST_MFE, pdwbuffersize: *mut u32, pbbuffer: *mut u8) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn MgmGetMfe(pimm: *mut super::IpHelper::MIB_IPMCAST_MFE, pdwbuffersize: *mut u32, pbbuffer: *mut u8) -> u32;
        }
        ::std::mem::transmute(MgmGetMfe(::std::mem::transmute(pimm), ::std::mem::transmute(pdwbuffersize), ::std::mem::transmute(pbbuffer)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_NetworkManagement_IpHelper")]
#[inline]
pub unsafe fn MgmGetMfeStats(pimm: *mut super::IpHelper::MIB_IPMCAST_MFE, pdwbuffersize: *mut u32, pbbuffer: *mut u8, dwflags: u32) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn MgmGetMfeStats(pimm: *mut super::IpHelper::MIB_IPMCAST_MFE, pdwbuffersize: *mut u32, pbbuffer: *mut u8, dwflags: u32) -> u32;
        }
        ::std::mem::transmute(MgmGetMfeStats(::std::mem::transmute(pimm), ::std::mem::transmute(pdwbuffersize), ::std::mem::transmute(pbbuffer), ::std::mem::transmute(dwflags)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_NetworkManagement_IpHelper")]
#[inline]
pub unsafe fn MgmGetNextMfe(pimmstart: *mut super::IpHelper::MIB_IPMCAST_MFE, pdwbuffersize: *mut u32, pbbuffer: *mut u8, pdwnumentries: *mut u32) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn MgmGetNextMfe(pimmstart: *mut super::IpHelper::MIB_IPMCAST_MFE, pdwbuffersize: *mut u32, pbbuffer: *mut u8, pdwnumentries: *mut u32) -> u32;
        }
        ::std::mem::transmute(MgmGetNextMfe(::std::mem::transmute(pimmstart), ::std::mem::transmute(pdwbuffersize), ::std::mem::transmute(pbbuffer), ::std::mem::transmute(pdwnumentries)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_NetworkManagement_IpHelper")]
#[inline]
pub unsafe fn MgmGetNextMfeStats(pimmstart: *mut super::IpHelper::MIB_IPMCAST_MFE, pdwbuffersize: *mut u32, pbbuffer: *mut u8, pdwnumentries: *mut u32, dwflags: u32) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn MgmGetNextMfeStats(pimmstart: *mut super::IpHelper::MIB_IPMCAST_MFE, pdwbuffersize: *mut u32, pbbuffer: *mut u8, pdwnumentries: *mut u32, dwflags: u32) -> u32;
        }
        ::std::mem::transmute(MgmGetNextMfeStats(::std::mem::transmute(pimmstart), ::std::mem::transmute(pdwbuffersize), ::std::mem::transmute(pbbuffer), ::std::mem::transmute(pdwnumentries), ::std::mem::transmute(dwflags)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn MgmGetProtocolOnInterface(dwifindex: u32, dwifnexthopaddr: u32, pdwifprotocolid: *mut u32, pdwifcomponentid: *mut u32) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn MgmGetProtocolOnInterface(dwifindex: u32, dwifnexthopaddr: u32, pdwifprotocolid: *mut u32, pdwifcomponentid: *mut u32) -> u32;
        }
        ::std::mem::transmute(MgmGetProtocolOnInterface(::std::mem::transmute(dwifindex), ::std::mem::transmute(dwifnexthopaddr), ::std::mem::transmute(pdwifprotocolid), ::std::mem::transmute(pdwifcomponentid)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn MgmGroupEnumerationEnd<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::HANDLE>>(henum: Param0) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn MgmGroupEnumerationEnd(henum: super::super::Foundation::HANDLE) -> u32;
        }
        ::std::mem::transmute(MgmGroupEnumerationEnd(henum.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn MgmGroupEnumerationGetNext<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::HANDLE>>(henum: Param0, pdwbuffersize: *mut u32, pbbuffer: *mut u8, pdwnumentries: *mut u32) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn MgmGroupEnumerationGetNext(henum: super::super::Foundation::HANDLE, pdwbuffersize: *mut u32, pbbuffer: *mut u8, pdwnumentries: *mut u32) -> u32;
        }
        ::std::mem::transmute(MgmGroupEnumerationGetNext(henum.into_param().abi(), ::std::mem::transmute(pdwbuffersize), ::std::mem::transmute(pbbuffer), ::std::mem::transmute(pdwnumentries)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn MgmGroupEnumerationStart<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::HANDLE>>(hprotocol: Param0, metenumtype: MGM_ENUM_TYPES, phenumhandle: *mut super::super::Foundation::HANDLE) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn MgmGroupEnumerationStart(hprotocol: super::super::Foundation::HANDLE, metenumtype: MGM_ENUM_TYPES, phenumhandle: *mut super::super::Foundation::HANDLE) -> u32;
        }
        ::std::mem::transmute(MgmGroupEnumerationStart(hprotocol.into_param().abi(), ::std::mem::transmute(metenumtype), ::std::mem::transmute(phenumhandle)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn MgmRegisterMProtocol(prpiinfo: *mut ROUTING_PROTOCOL_CONFIG, dwprotocolid: u32, dwcomponentid: u32, phprotocol: *mut super::super::Foundation::HANDLE) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn MgmRegisterMProtocol(prpiinfo: *mut ::std::mem::ManuallyDrop<ROUTING_PROTOCOL_CONFIG>, dwprotocolid: u32, dwcomponentid: u32, phprotocol: *mut super::super::Foundation::HANDLE) -> u32;
        }
        ::std::mem::transmute(MgmRegisterMProtocol(::std::mem::transmute(prpiinfo), ::std::mem::transmute(dwprotocolid), ::std::mem::transmute(dwcomponentid), ::std::mem::transmute(phprotocol)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn MgmReleaseInterfaceOwnership<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::HANDLE>>(hprotocol: Param0, dwifindex: u32, dwifnexthopaddr: u32) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn MgmReleaseInterfaceOwnership(hprotocol: super::super::Foundation::HANDLE, dwifindex: u32, dwifnexthopaddr: u32) -> u32;
        }
        ::std::mem::transmute(MgmReleaseInterfaceOwnership(hprotocol.into_param().abi(), ::std::mem::transmute(dwifindex), ::std::mem::transmute(dwifnexthopaddr)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn MgmTakeInterfaceOwnership<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::HANDLE>>(hprotocol: Param0, dwifindex: u32, dwifnexthopaddr: u32) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn MgmTakeInterfaceOwnership(hprotocol: super::super::Foundation::HANDLE, dwifindex: u32, dwifnexthopaddr: u32) -> u32;
        }
        ::std::mem::transmute(MgmTakeInterfaceOwnership(hprotocol.into_param().abi(), ::std::mem::transmute(dwifindex), ::std::mem::transmute(dwifnexthopaddr)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn MprAdminBufferFree(pbuffer: *const ::std::ffi::c_void) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn MprAdminBufferFree(pbuffer: *const ::std::ffi::c_void) -> u32;
        }
        ::std::mem::transmute(MprAdminBufferFree(::std::mem::transmute(pbuffer)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn MprAdminConnectionClearStats<'a, Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::HANDLE>>(hrasserver: isize, hrasconnection: Param1) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn MprAdminConnectionClearStats(hrasserver: isize, hrasconnection: super::super::Foundation::HANDLE) -> u32;
        }
        ::std::mem::transmute(MprAdminConnectionClearStats(::std::mem::transmute(hrasserver), hrasconnection.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn MprAdminConnectionEnum(hrasserver: isize, dwlevel: u32, lplpbbuffer: *mut *mut u8, dwprefmaxlen: u32, lpdwentriesread: *mut u32, lpdwtotalentries: *mut u32, lpdwresumehandle: *const u32) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn MprAdminConnectionEnum(hrasserver: isize, dwlevel: u32, lplpbbuffer: *mut *mut u8, dwprefmaxlen: u32, lpdwentriesread: *mut u32, lpdwtotalentries: *mut u32, lpdwresumehandle: *const u32) -> u32;
        }
        ::std::mem::transmute(MprAdminConnectionEnum(::std::mem::transmute(hrasserver), ::std::mem::transmute(dwlevel), ::std::mem::transmute(lplpbbuffer), ::std::mem::transmute(dwprefmaxlen), ::std::mem::transmute(lpdwentriesread), ::std::mem::transmute(lpdwtotalentries), ::std::mem::transmute(lpdwresumehandle)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn MprAdminConnectionEnumEx(hrasserver: isize, pobjectheader: *const MPRAPI_OBJECT_HEADER, dwpreferedmaxlen: u32, lpdwentriesread: *mut u32, lpdwtotalentries: *mut u32, pprasconn: *mut *mut RAS_CONNECTION_EX, lpdwresumehandle: *const u32) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn MprAdminConnectionEnumEx(hrasserver: isize, pobjectheader: *const MPRAPI_OBJECT_HEADER, dwpreferedmaxlen: u32, lpdwentriesread: *mut u32, lpdwtotalentries: *mut u32, pprasconn: *mut *mut RAS_CONNECTION_EX, lpdwresumehandle: *const u32) -> u32;
        }
        ::std::mem::transmute(MprAdminConnectionEnumEx(::std::mem::transmute(hrasserver), ::std::mem::transmute(pobjectheader), ::std::mem::transmute(dwpreferedmaxlen), ::std::mem::transmute(lpdwentriesread), ::std::mem::transmute(lpdwtotalentries), ::std::mem::transmute(pprasconn), ::std::mem::transmute(lpdwresumehandle)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn MprAdminConnectionGetInfo<'a, Param2: ::windows::runtime::IntoParam<'a, super::super::Foundation::HANDLE>>(hrasserver: isize, dwlevel: u32, hrasconnection: Param2, lplpbbuffer: *mut *mut u8) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn MprAdminConnectionGetInfo(hrasserver: isize, dwlevel: u32, hrasconnection: super::super::Foundation::HANDLE, lplpbbuffer: *mut *mut u8) -> u32;
        }
        ::std::mem::transmute(MprAdminConnectionGetInfo(::std::mem::transmute(hrasserver), ::std::mem::transmute(dwlevel), hrasconnection.into_param().abi(), ::std::mem::transmute(lplpbbuffer)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn MprAdminConnectionGetInfoEx<'a, Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::HANDLE>>(hrasserver: isize, hrasconnection: Param1, prasconnection: *mut RAS_CONNECTION_EX) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn MprAdminConnectionGetInfoEx(hrasserver: isize, hrasconnection: super::super::Foundation::HANDLE, prasconnection: *mut RAS_CONNECTION_EX) -> u32;
        }
        ::std::mem::transmute(MprAdminConnectionGetInfoEx(::std::mem::transmute(hrasserver), hrasconnection.into_param().abi(), ::std::mem::transmute(prasconnection)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn MprAdminConnectionRemoveQuarantine<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::HANDLE>, Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::HANDLE>, Param2: ::windows::runtime::IntoParam<'a, super::super::Foundation::BOOL>>(hrasserver: Param0, hrasconnection: Param1, fisipaddress: Param2) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn MprAdminConnectionRemoveQuarantine(hrasserver: super::super::Foundation::HANDLE, hrasconnection: super::super::Foundation::HANDLE, fisipaddress: super::super::Foundation::BOOL) -> u32;
        }
        ::std::mem::transmute(MprAdminConnectionRemoveQuarantine(hrasserver.into_param().abi(), hrasconnection.into_param().abi(), fisipaddress.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn MprAdminDeregisterConnectionNotification<'a, Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::HANDLE>>(hmprserver: isize, heventnotification: Param1) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn MprAdminDeregisterConnectionNotification(hmprserver: isize, heventnotification: super::super::Foundation::HANDLE) -> u32;
        }
        ::std::mem::transmute(MprAdminDeregisterConnectionNotification(::std::mem::transmute(hmprserver), heventnotification.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn MprAdminDeviceEnum(hmprserver: isize, dwlevel: u32, lplpbbuffer: *mut *mut u8, lpdwtotalentries: *mut u32) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn MprAdminDeviceEnum(hmprserver: isize, dwlevel: u32, lplpbbuffer: *mut *mut u8, lpdwtotalentries: *mut u32) -> u32;
        }
        ::std::mem::transmute(MprAdminDeviceEnum(::std::mem::transmute(hmprserver), ::std::mem::transmute(dwlevel), ::std::mem::transmute(lplpbbuffer), ::std::mem::transmute(lpdwtotalentries)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn MprAdminEstablishDomainRasServer<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>, Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>, Param2: ::windows::runtime::IntoParam<'a, super::super::Foundation::BOOL>>(pszdomain: Param0, pszmachine: Param1, benable: Param2) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn MprAdminEstablishDomainRasServer(pszdomain: super::super::Foundation::PWSTR, pszmachine: super::super::Foundation::PWSTR, benable: super::super::Foundation::BOOL) -> u32;
        }
        ::std::mem::transmute(MprAdminEstablishDomainRasServer(pszdomain.into_param().abi(), pszmachine.into_param().abi(), benable.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn MprAdminGetErrorString(dwerror: u32, lplpwserrorstring: *mut super::super::Foundation::PWSTR) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn MprAdminGetErrorString(dwerror: u32, lplpwserrorstring: *mut super::super::Foundation::PWSTR) -> u32;
        }
        ::std::mem::transmute(MprAdminGetErrorString(::std::mem::transmute(dwerror), ::std::mem::transmute(lplpwserrorstring)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn MprAdminGetPDCServer<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>, Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>>(lpszdomain: Param0, lpszserver: Param1, lpszpdcserver: super::super::Foundation::PWSTR) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn MprAdminGetPDCServer(lpszdomain: super::super::Foundation::PWSTR, lpszserver: super::super::Foundation::PWSTR, lpszpdcserver: super::super::Foundation::PWSTR) -> u32;
        }
        ::std::mem::transmute(MprAdminGetPDCServer(lpszdomain.into_param().abi(), lpszserver.into_param().abi(), ::std::mem::transmute(lpszpdcserver)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn MprAdminInterfaceConnect<'a, Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::HANDLE>, Param2: ::windows::runtime::IntoParam<'a, super::super::Foundation::HANDLE>, Param3: ::windows::runtime::IntoParam<'a, super::super::Foundation::BOOL>>(hmprserver: isize, hinterface: Param1, hevent: Param2, fsynchronous: Param3) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn MprAdminInterfaceConnect(hmprserver: isize, hinterface: super::super::Foundation::HANDLE, hevent: super::super::Foundation::HANDLE, fsynchronous: super::super::Foundation::BOOL) -> u32;
        }
        ::std::mem::transmute(MprAdminInterfaceConnect(::std::mem::transmute(hmprserver), hinterface.into_param().abi(), hevent.into_param().abi(), fsynchronous.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn MprAdminInterfaceCreate(hmprserver: isize, dwlevel: u32, lpbbuffer: *const u8, phinterface: *mut super::super::Foundation::HANDLE) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn MprAdminInterfaceCreate(hmprserver: isize, dwlevel: u32, lpbbuffer: *const u8, phinterface: *mut super::super::Foundation::HANDLE) -> u32;
        }
        ::std::mem::transmute(MprAdminInterfaceCreate(::std::mem::transmute(hmprserver), ::std::mem::transmute(dwlevel), ::std::mem::transmute(lpbbuffer), ::std::mem::transmute(phinterface)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn MprAdminInterfaceDelete<'a, Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::HANDLE>>(hmprserver: isize, hinterface: Param1) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn MprAdminInterfaceDelete(hmprserver: isize, hinterface: super::super::Foundation::HANDLE) -> u32;
        }
        ::std::mem::transmute(MprAdminInterfaceDelete(::std::mem::transmute(hmprserver), hinterface.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn MprAdminInterfaceDeviceGetInfo<'a, Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::HANDLE>>(hmprserver: isize, hinterface: Param1, dwindex: u32, dwlevel: u32, lplpbuffer: *mut *mut u8) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn MprAdminInterfaceDeviceGetInfo(hmprserver: isize, hinterface: super::super::Foundation::HANDLE, dwindex: u32, dwlevel: u32, lplpbuffer: *mut *mut u8) -> u32;
        }
        ::std::mem::transmute(MprAdminInterfaceDeviceGetInfo(::std::mem::transmute(hmprserver), hinterface.into_param().abi(), ::std::mem::transmute(dwindex), ::std::mem::transmute(dwlevel), ::std::mem::transmute(lplpbuffer)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn MprAdminInterfaceDeviceSetInfo<'a, Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::HANDLE>>(hmprserver: isize, hinterface: Param1, dwindex: u32, dwlevel: u32, lpbbuffer: *const u8) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn MprAdminInterfaceDeviceSetInfo(hmprserver: isize, hinterface: super::super::Foundation::HANDLE, dwindex: u32, dwlevel: u32, lpbbuffer: *const u8) -> u32;
        }
        ::std::mem::transmute(MprAdminInterfaceDeviceSetInfo(::std::mem::transmute(hmprserver), hinterface.into_param().abi(), ::std::mem::transmute(dwindex), ::std::mem::transmute(dwlevel), ::std::mem::transmute(lpbbuffer)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn MprAdminInterfaceDisconnect<'a, Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::HANDLE>>(hmprserver: isize, hinterface: Param1) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn MprAdminInterfaceDisconnect(hmprserver: isize, hinterface: super::super::Foundation::HANDLE) -> u32;
        }
        ::std::mem::transmute(MprAdminInterfaceDisconnect(::std::mem::transmute(hmprserver), hinterface.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn MprAdminInterfaceEnum(hmprserver: isize, dwlevel: u32, lplpbbuffer: *mut *mut u8, dwprefmaxlen: u32, lpdwentriesread: *mut u32, lpdwtotalentries: *mut u32, lpdwresumehandle: *const u32) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn MprAdminInterfaceEnum(hmprserver: isize, dwlevel: u32, lplpbbuffer: *mut *mut u8, dwprefmaxlen: u32, lpdwentriesread: *mut u32, lpdwtotalentries: *mut u32, lpdwresumehandle: *const u32) -> u32;
        }
        ::std::mem::transmute(MprAdminInterfaceEnum(::std::mem::transmute(hmprserver), ::std::mem::transmute(dwlevel), ::std::mem::transmute(lplpbbuffer), ::std::mem::transmute(dwprefmaxlen), ::std::mem::transmute(lpdwentriesread), ::std::mem::transmute(lpdwtotalentries), ::std::mem::transmute(lpdwresumehandle)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn MprAdminInterfaceGetCredentials<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>, Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>>(lpwsserver: Param0, lpwsinterfacename: Param1, lpwsusername: super::super::Foundation::PWSTR, lpwspassword: super::super::Foundation::PWSTR, lpwsdomainname: super::super::Foundation::PWSTR) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn MprAdminInterfaceGetCredentials(lpwsserver: super::super::Foundation::PWSTR, lpwsinterfacename: super::super::Foundation::PWSTR, lpwsusername: super::super::Foundation::PWSTR, lpwspassword: super::super::Foundation::PWSTR, lpwsdomainname: super::super::Foundation::PWSTR) -> u32;
        }
        ::std::mem::transmute(MprAdminInterfaceGetCredentials(lpwsserver.into_param().abi(), lpwsinterfacename.into_param().abi(), ::std::mem::transmute(lpwsusername), ::std::mem::transmute(lpwspassword), ::std::mem::transmute(lpwsdomainname)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn MprAdminInterfaceGetCredentialsEx<'a, Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::HANDLE>>(hmprserver: isize, hinterface: Param1, dwlevel: u32, lplpbbuffer: *mut *mut u8) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn MprAdminInterfaceGetCredentialsEx(hmprserver: isize, hinterface: super::super::Foundation::HANDLE, dwlevel: u32, lplpbbuffer: *mut *mut u8) -> u32;
        }
        ::std::mem::transmute(MprAdminInterfaceGetCredentialsEx(::std::mem::transmute(hmprserver), hinterface.into_param().abi(), ::std::mem::transmute(dwlevel), ::std::mem::transmute(lplpbbuffer)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock", feature = "Win32_Security_Cryptography"))]
#[inline]
pub unsafe fn MprAdminInterfaceGetCustomInfoEx<'a, Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::HANDLE>>(hmprserver: isize, hinterface: Param1, pcustominfo: *mut MPR_IF_CUSTOMINFOEX2) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn MprAdminInterfaceGetCustomInfoEx(hmprserver: isize, hinterface: super::super::Foundation::HANDLE, pcustominfo: *mut MPR_IF_CUSTOMINFOEX2) -> u32;
        }
        ::std::mem::transmute(MprAdminInterfaceGetCustomInfoEx(::std::mem::transmute(hmprserver), hinterface.into_param().abi(), ::std::mem::transmute(pcustominfo)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn MprAdminInterfaceGetHandle<'a, Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>, Param3: ::windows::runtime::IntoParam<'a, super::super::Foundation::BOOL>>(hmprserver: isize, lpwsinterfacename: Param1, phinterface: *mut super::super::Foundation::HANDLE, fincludeclientinterfaces: Param3) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn MprAdminInterfaceGetHandle(hmprserver: isize, lpwsinterfacename: super::super::Foundation::PWSTR, phinterface: *mut super::super::Foundation::HANDLE, fincludeclientinterfaces: super::super::Foundation::BOOL) -> u32;
        }
        ::std::mem::transmute(MprAdminInterfaceGetHandle(::std::mem::transmute(hmprserver), lpwsinterfacename.into_param().abi(), ::std::mem::transmute(phinterface), fincludeclientinterfaces.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn MprAdminInterfaceGetInfo<'a, Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::HANDLE>>(hmprserver: isize, hinterface: Param1, dwlevel: u32, lplpbbuffer: *const *const u8) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn MprAdminInterfaceGetInfo(hmprserver: isize, hinterface: super::super::Foundation::HANDLE, dwlevel: u32, lplpbbuffer: *const *const u8) -> u32;
        }
        ::std::mem::transmute(MprAdminInterfaceGetInfo(::std::mem::transmute(hmprserver), hinterface.into_param().abi(), ::std::mem::transmute(dwlevel), ::std::mem::transmute(lplpbbuffer)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn MprAdminInterfaceQueryUpdateResult<'a, Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::HANDLE>>(hmprserver: isize, hinterface: Param1, dwprotocolid: u32, lpdwupdateresult: *mut u32) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn MprAdminInterfaceQueryUpdateResult(hmprserver: isize, hinterface: super::super::Foundation::HANDLE, dwprotocolid: u32, lpdwupdateresult: *mut u32) -> u32;
        }
        ::std::mem::transmute(MprAdminInterfaceQueryUpdateResult(::std::mem::transmute(hmprserver), hinterface.into_param().abi(), ::std::mem::transmute(dwprotocolid), ::std::mem::transmute(lpdwupdateresult)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn MprAdminInterfaceSetCredentials<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>, Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>, Param2: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>, Param3: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>, Param4: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>>(
    lpwsserver: Param0,
    lpwsinterfacename: Param1,
    lpwsusername: Param2,
    lpwsdomainname: Param3,
    lpwspassword: Param4,
) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn MprAdminInterfaceSetCredentials(lpwsserver: super::super::Foundation::PWSTR, lpwsinterfacename: super::super::Foundation::PWSTR, lpwsusername: super::super::Foundation::PWSTR, lpwsdomainname: super::super::Foundation::PWSTR, lpwspassword: super::super::Foundation::PWSTR) -> u32;
        }
        ::std::mem::transmute(MprAdminInterfaceSetCredentials(lpwsserver.into_param().abi(), lpwsinterfacename.into_param().abi(), lpwsusername.into_param().abi(), lpwsdomainname.into_param().abi(), lpwspassword.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn MprAdminInterfaceSetCredentialsEx<'a, Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::HANDLE>>(hmprserver: isize, hinterface: Param1, dwlevel: u32, lpbbuffer: *const u8) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn MprAdminInterfaceSetCredentialsEx(hmprserver: isize, hinterface: super::super::Foundation::HANDLE, dwlevel: u32, lpbbuffer: *const u8) -> u32;
        }
        ::std::mem::transmute(MprAdminInterfaceSetCredentialsEx(::std::mem::transmute(hmprserver), hinterface.into_param().abi(), ::std::mem::transmute(dwlevel), ::std::mem::transmute(lpbbuffer)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock", feature = "Win32_Security_Cryptography"))]
#[inline]
pub unsafe fn MprAdminInterfaceSetCustomInfoEx<'a, Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::HANDLE>>(hmprserver: isize, hinterface: Param1, pcustominfo: *const MPR_IF_CUSTOMINFOEX2) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn MprAdminInterfaceSetCustomInfoEx(hmprserver: isize, hinterface: super::super::Foundation::HANDLE, pcustominfo: *const MPR_IF_CUSTOMINFOEX2) -> u32;
        }
        ::std::mem::transmute(MprAdminInterfaceSetCustomInfoEx(::std::mem::transmute(hmprserver), hinterface.into_param().abi(), ::std::mem::transmute(pcustominfo)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn MprAdminInterfaceSetInfo<'a, Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::HANDLE>>(hmprserver: isize, hinterface: Param1, dwlevel: u32, lpbbuffer: *const u8) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn MprAdminInterfaceSetInfo(hmprserver: isize, hinterface: super::super::Foundation::HANDLE, dwlevel: u32, lpbbuffer: *const u8) -> u32;
        }
        ::std::mem::transmute(MprAdminInterfaceSetInfo(::std::mem::transmute(hmprserver), hinterface.into_param().abi(), ::std::mem::transmute(dwlevel), ::std::mem::transmute(lpbbuffer)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn MprAdminInterfaceTransportAdd<'a, Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::HANDLE>>(hmprserver: isize, hinterface: Param1, dwtransportid: u32, pinterfaceinfo: *const u8, dwinterfaceinfosize: u32) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn MprAdminInterfaceTransportAdd(hmprserver: isize, hinterface: super::super::Foundation::HANDLE, dwtransportid: u32, pinterfaceinfo: *const u8, dwinterfaceinfosize: u32) -> u32;
        }
        ::std::mem::transmute(MprAdminInterfaceTransportAdd(::std::mem::transmute(hmprserver), hinterface.into_param().abi(), ::std::mem::transmute(dwtransportid), ::std::mem::transmute(pinterfaceinfo), ::std::mem::transmute(dwinterfaceinfosize)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn MprAdminInterfaceTransportGetInfo<'a, Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::HANDLE>>(hmprserver: isize, hinterface: Param1, dwtransportid: u32, ppinterfaceinfo: *mut *mut u8, lpdwinterfaceinfosize: *mut u32) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn MprAdminInterfaceTransportGetInfo(hmprserver: isize, hinterface: super::super::Foundation::HANDLE, dwtransportid: u32, ppinterfaceinfo: *mut *mut u8, lpdwinterfaceinfosize: *mut u32) -> u32;
        }
        ::std::mem::transmute(MprAdminInterfaceTransportGetInfo(::std::mem::transmute(hmprserver), hinterface.into_param().abi(), ::std::mem::transmute(dwtransportid), ::std::mem::transmute(ppinterfaceinfo), ::std::mem::transmute(lpdwinterfaceinfosize)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn MprAdminInterfaceTransportRemove<'a, Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::HANDLE>>(hmprserver: isize, hinterface: Param1, dwtransportid: u32) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn MprAdminInterfaceTransportRemove(hmprserver: isize, hinterface: super::super::Foundation::HANDLE, dwtransportid: u32) -> u32;
        }
        ::std::mem::transmute(MprAdminInterfaceTransportRemove(::std::mem::transmute(hmprserver), hinterface.into_param().abi(), ::std::mem::transmute(dwtransportid)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn MprAdminInterfaceTransportSetInfo<'a, Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::HANDLE>>(hmprserver: isize, hinterface: Param1, dwtransportid: u32, pinterfaceinfo: *const u8, dwinterfaceinfosize: u32) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn MprAdminInterfaceTransportSetInfo(hmprserver: isize, hinterface: super::super::Foundation::HANDLE, dwtransportid: u32, pinterfaceinfo: *const u8, dwinterfaceinfosize: u32) -> u32;
        }
        ::std::mem::transmute(MprAdminInterfaceTransportSetInfo(::std::mem::transmute(hmprserver), hinterface.into_param().abi(), ::std::mem::transmute(dwtransportid), ::std::mem::transmute(pinterfaceinfo), ::std::mem::transmute(dwinterfaceinfosize)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn MprAdminInterfaceUpdatePhonebookInfo<'a, Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::HANDLE>>(hmprserver: isize, hinterface: Param1) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn MprAdminInterfaceUpdatePhonebookInfo(hmprserver: isize, hinterface: super::super::Foundation::HANDLE) -> u32;
        }
        ::std::mem::transmute(MprAdminInterfaceUpdatePhonebookInfo(::std::mem::transmute(hmprserver), hinterface.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn MprAdminInterfaceUpdateRoutes<'a, Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::HANDLE>, Param3: ::windows::runtime::IntoParam<'a, super::super::Foundation::HANDLE>>(hmprserver: isize, hinterface: Param1, dwprotocolid: u32, hevent: Param3) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn MprAdminInterfaceUpdateRoutes(hmprserver: isize, hinterface: super::super::Foundation::HANDLE, dwprotocolid: u32, hevent: super::super::Foundation::HANDLE) -> u32;
        }
        ::std::mem::transmute(MprAdminInterfaceUpdateRoutes(::std::mem::transmute(hmprserver), hinterface.into_param().abi(), ::std::mem::transmute(dwprotocolid), hevent.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn MprAdminIsDomainRasServer<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>, Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>>(pszdomain: Param0, pszmachine: Param1, pbisrasserver: *mut super::super::Foundation::BOOL) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn MprAdminIsDomainRasServer(pszdomain: super::super::Foundation::PWSTR, pszmachine: super::super::Foundation::PWSTR, pbisrasserver: *mut super::super::Foundation::BOOL) -> u32;
        }
        ::std::mem::transmute(MprAdminIsDomainRasServer(pszdomain.into_param().abi(), pszmachine.into_param().abi(), ::std::mem::transmute(pbisrasserver)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn MprAdminIsServiceInitialized<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>>(lpwsservername: Param0, fisserviceinitialized: *const super::super::Foundation::BOOL) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn MprAdminIsServiceInitialized(lpwsservername: super::super::Foundation::PWSTR, fisserviceinitialized: *const super::super::Foundation::BOOL) -> u32;
        }
        ::std::mem::transmute(MprAdminIsServiceInitialized(lpwsservername.into_param().abi(), ::std::mem::transmute(fisserviceinitialized)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn MprAdminIsServiceRunning<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>>(lpwsservername: Param0) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn MprAdminIsServiceRunning(lpwsservername: super::super::Foundation::PWSTR) -> super::super::Foundation::BOOL;
        }
        ::std::mem::transmute(MprAdminIsServiceRunning(lpwsservername.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn MprAdminMIBBufferFree(pbuffer: *const ::std::ffi::c_void) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn MprAdminMIBBufferFree(pbuffer: *const ::std::ffi::c_void) -> u32;
        }
        ::std::mem::transmute(MprAdminMIBBufferFree(::std::mem::transmute(pbuffer)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn MprAdminMIBEntryCreate(hmibserver: isize, dwpid: u32, dwroutingpid: u32, lpentry: *const ::std::ffi::c_void, dwentrysize: u32) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn MprAdminMIBEntryCreate(hmibserver: isize, dwpid: u32, dwroutingpid: u32, lpentry: *const ::std::ffi::c_void, dwentrysize: u32) -> u32;
        }
        ::std::mem::transmute(MprAdminMIBEntryCreate(::std::mem::transmute(hmibserver), ::std::mem::transmute(dwpid), ::std::mem::transmute(dwroutingpid), ::std::mem::transmute(lpentry), ::std::mem::transmute(dwentrysize)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn MprAdminMIBEntryDelete(hmibserver: isize, dwprotocolid: u32, dwroutingpid: u32, lpentry: *const ::std::ffi::c_void, dwentrysize: u32) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn MprAdminMIBEntryDelete(hmibserver: isize, dwprotocolid: u32, dwroutingpid: u32, lpentry: *const ::std::ffi::c_void, dwentrysize: u32) -> u32;
        }
        ::std::mem::transmute(MprAdminMIBEntryDelete(::std::mem::transmute(hmibserver), ::std::mem::transmute(dwprotocolid), ::std::mem::transmute(dwroutingpid), ::std::mem::transmute(lpentry), ::std::mem::transmute(dwentrysize)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn MprAdminMIBEntryGet(hmibserver: isize, dwprotocolid: u32, dwroutingpid: u32, lpinentry: *const ::std::ffi::c_void, dwinentrysize: u32, lplpoutentry: *mut *mut ::std::ffi::c_void, lpoutentrysize: *mut u32) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn MprAdminMIBEntryGet(hmibserver: isize, dwprotocolid: u32, dwroutingpid: u32, lpinentry: *const ::std::ffi::c_void, dwinentrysize: u32, lplpoutentry: *mut *mut ::std::ffi::c_void, lpoutentrysize: *mut u32) -> u32;
        }
        ::std::mem::transmute(MprAdminMIBEntryGet(::std::mem::transmute(hmibserver), ::std::mem::transmute(dwprotocolid), ::std::mem::transmute(dwroutingpid), ::std::mem::transmute(lpinentry), ::std::mem::transmute(dwinentrysize), ::std::mem::transmute(lplpoutentry), ::std::mem::transmute(lpoutentrysize)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn MprAdminMIBEntryGetFirst(hmibserver: isize, dwprotocolid: u32, dwroutingpid: u32, lpinentry: *const ::std::ffi::c_void, dwinentrysize: u32, lplpoutentry: *mut *mut ::std::ffi::c_void, lpoutentrysize: *mut u32) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn MprAdminMIBEntryGetFirst(hmibserver: isize, dwprotocolid: u32, dwroutingpid: u32, lpinentry: *const ::std::ffi::c_void, dwinentrysize: u32, lplpoutentry: *mut *mut ::std::ffi::c_void, lpoutentrysize: *mut u32) -> u32;
        }
        ::std::mem::transmute(MprAdminMIBEntryGetFirst(::std::mem::transmute(hmibserver), ::std::mem::transmute(dwprotocolid), ::std::mem::transmute(dwroutingpid), ::std::mem::transmute(lpinentry), ::std::mem::transmute(dwinentrysize), ::std::mem::transmute(lplpoutentry), ::std::mem::transmute(lpoutentrysize)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn MprAdminMIBEntryGetNext(hmibserver: isize, dwprotocolid: u32, dwroutingpid: u32, lpinentry: *const ::std::ffi::c_void, dwinentrysize: u32, lplpoutentry: *mut *mut ::std::ffi::c_void, lpoutentrysize: *mut u32) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn MprAdminMIBEntryGetNext(hmibserver: isize, dwprotocolid: u32, dwroutingpid: u32, lpinentry: *const ::std::ffi::c_void, dwinentrysize: u32, lplpoutentry: *mut *mut ::std::ffi::c_void, lpoutentrysize: *mut u32) -> u32;
        }
        ::std::mem::transmute(MprAdminMIBEntryGetNext(::std::mem::transmute(hmibserver), ::std::mem::transmute(dwprotocolid), ::std::mem::transmute(dwroutingpid), ::std::mem::transmute(lpinentry), ::std::mem::transmute(dwinentrysize), ::std::mem::transmute(lplpoutentry), ::std::mem::transmute(lpoutentrysize)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn MprAdminMIBEntrySet(hmibserver: isize, dwprotocolid: u32, dwroutingpid: u32, lpentry: *const ::std::ffi::c_void, dwentrysize: u32) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn MprAdminMIBEntrySet(hmibserver: isize, dwprotocolid: u32, dwroutingpid: u32, lpentry: *const ::std::ffi::c_void, dwentrysize: u32) -> u32;
        }
        ::std::mem::transmute(MprAdminMIBEntrySet(::std::mem::transmute(hmibserver), ::std::mem::transmute(dwprotocolid), ::std::mem::transmute(dwroutingpid), ::std::mem::transmute(lpentry), ::std::mem::transmute(dwentrysize)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn MprAdminMIBServerConnect<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>>(lpwsservername: Param0, phmibserver: *mut isize) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn MprAdminMIBServerConnect(lpwsservername: super::super::Foundation::PWSTR, phmibserver: *mut isize) -> u32;
        }
        ::std::mem::transmute(MprAdminMIBServerConnect(lpwsservername.into_param().abi(), ::std::mem::transmute(phmibserver)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn MprAdminMIBServerDisconnect(hmibserver: isize) {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn MprAdminMIBServerDisconnect(hmibserver: isize);
        }
        ::std::mem::transmute(MprAdminMIBServerDisconnect(::std::mem::transmute(hmibserver)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn MprAdminPortClearStats<'a, Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::HANDLE>>(hrasserver: isize, hport: Param1) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn MprAdminPortClearStats(hrasserver: isize, hport: super::super::Foundation::HANDLE) -> u32;
        }
        ::std::mem::transmute(MprAdminPortClearStats(::std::mem::transmute(hrasserver), hport.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn MprAdminPortDisconnect<'a, Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::HANDLE>>(hrasserver: isize, hport: Param1) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn MprAdminPortDisconnect(hrasserver: isize, hport: super::super::Foundation::HANDLE) -> u32;
        }
        ::std::mem::transmute(MprAdminPortDisconnect(::std::mem::transmute(hrasserver), hport.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn MprAdminPortEnum<'a, Param2: ::windows::runtime::IntoParam<'a, super::super::Foundation::HANDLE>>(hrasserver: isize, dwlevel: u32, hrasconnection: Param2, lplpbbuffer: *mut *mut u8, dwprefmaxlen: u32, lpdwentriesread: *mut u32, lpdwtotalentries: *mut u32, lpdwresumehandle: *const u32) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn MprAdminPortEnum(hrasserver: isize, dwlevel: u32, hrasconnection: super::super::Foundation::HANDLE, lplpbbuffer: *mut *mut u8, dwprefmaxlen: u32, lpdwentriesread: *mut u32, lpdwtotalentries: *mut u32, lpdwresumehandle: *const u32) -> u32;
        }
        ::std::mem::transmute(MprAdminPortEnum(
            ::std::mem::transmute(hrasserver),
            ::std::mem::transmute(dwlevel),
            hrasconnection.into_param().abi(),
            ::std::mem::transmute(lplpbbuffer),
            ::std::mem::transmute(dwprefmaxlen),
            ::std::mem::transmute(lpdwentriesread),
            ::std::mem::transmute(lpdwtotalentries),
            ::std::mem::transmute(lpdwresumehandle),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn MprAdminPortGetInfo<'a, Param2: ::windows::runtime::IntoParam<'a, super::super::Foundation::HANDLE>>(hrasserver: isize, dwlevel: u32, hport: Param2, lplpbbuffer: *mut *mut u8) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn MprAdminPortGetInfo(hrasserver: isize, dwlevel: u32, hport: super::super::Foundation::HANDLE, lplpbbuffer: *mut *mut u8) -> u32;
        }
        ::std::mem::transmute(MprAdminPortGetInfo(::std::mem::transmute(hrasserver), ::std::mem::transmute(dwlevel), hport.into_param().abi(), ::std::mem::transmute(lplpbbuffer)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn MprAdminPortReset<'a, Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::HANDLE>>(hrasserver: isize, hport: Param1) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn MprAdminPortReset(hrasserver: isize, hport: super::super::Foundation::HANDLE) -> u32;
        }
        ::std::mem::transmute(MprAdminPortReset(::std::mem::transmute(hrasserver), hport.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn MprAdminRegisterConnectionNotification<'a, Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::HANDLE>>(hmprserver: isize, heventnotification: Param1) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn MprAdminRegisterConnectionNotification(hmprserver: isize, heventnotification: super::super::Foundation::HANDLE) -> u32;
        }
        ::std::mem::transmute(MprAdminRegisterConnectionNotification(::std::mem::transmute(hmprserver), heventnotification.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn MprAdminSendUserMessage<'a, Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::HANDLE>, Param2: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>>(hmprserver: isize, hconnection: Param1, lpwszmessage: Param2) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn MprAdminSendUserMessage(hmprserver: isize, hconnection: super::super::Foundation::HANDLE, lpwszmessage: super::super::Foundation::PWSTR) -> u32;
        }
        ::std::mem::transmute(MprAdminSendUserMessage(::std::mem::transmute(hmprserver), hconnection.into_param().abi(), lpwszmessage.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn MprAdminServerConnect<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>>(lpwsservername: Param0, phmprserver: *mut isize) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn MprAdminServerConnect(lpwsservername: super::super::Foundation::PWSTR, phmprserver: *mut isize) -> u32;
        }
        ::std::mem::transmute(MprAdminServerConnect(lpwsservername.into_param().abi(), ::std::mem::transmute(phmprserver)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn MprAdminServerDisconnect(hmprserver: isize) {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn MprAdminServerDisconnect(hmprserver: isize);
        }
        ::std::mem::transmute(MprAdminServerDisconnect(::std::mem::transmute(hmprserver)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn MprAdminServerGetCredentials(hmprserver: isize, dwlevel: u32, lplpbbuffer: *const *const u8) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn MprAdminServerGetCredentials(hmprserver: isize, dwlevel: u32, lplpbbuffer: *const *const u8) -> u32;
        }
        ::std::mem::transmute(MprAdminServerGetCredentials(::std::mem::transmute(hmprserver), ::std::mem::transmute(dwlevel), ::std::mem::transmute(lplpbbuffer)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn MprAdminServerGetInfo(hmprserver: isize, dwlevel: u32, lplpbbuffer: *mut *mut u8) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn MprAdminServerGetInfo(hmprserver: isize, dwlevel: u32, lplpbbuffer: *mut *mut u8) -> u32;
        }
        ::std::mem::transmute(MprAdminServerGetInfo(::std::mem::transmute(hmprserver), ::std::mem::transmute(dwlevel), ::std::mem::transmute(lplpbbuffer)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography"))]
#[inline]
pub unsafe fn MprAdminServerGetInfoEx(hmprserver: isize, pserverinfo: *mut MPR_SERVER_EX1) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn MprAdminServerGetInfoEx(hmprserver: isize, pserverinfo: *mut MPR_SERVER_EX1) -> u32;
        }
        ::std::mem::transmute(MprAdminServerGetInfoEx(::std::mem::transmute(hmprserver), ::std::mem::transmute(pserverinfo)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn MprAdminServerSetCredentials(hmprserver: isize, dwlevel: u32, lpbbuffer: *const u8) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn MprAdminServerSetCredentials(hmprserver: isize, dwlevel: u32, lpbbuffer: *const u8) -> u32;
        }
        ::std::mem::transmute(MprAdminServerSetCredentials(::std::mem::transmute(hmprserver), ::std::mem::transmute(dwlevel), ::std::mem::transmute(lpbbuffer)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn MprAdminServerSetInfo(hmprserver: isize, dwlevel: u32, lpbbuffer: *const u8) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn MprAdminServerSetInfo(hmprserver: isize, dwlevel: u32, lpbbuffer: *const u8) -> u32;
        }
        ::std::mem::transmute(MprAdminServerSetInfo(::std::mem::transmute(hmprserver), ::std::mem::transmute(dwlevel), ::std::mem::transmute(lpbbuffer)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography"))]
#[inline]
pub unsafe fn MprAdminServerSetInfoEx(hmprserver: isize, pserverinfo: *const MPR_SERVER_SET_CONFIG_EX1) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn MprAdminServerSetInfoEx(hmprserver: isize, pserverinfo: *const MPR_SERVER_SET_CONFIG_EX1) -> u32;
        }
        ::std::mem::transmute(MprAdminServerSetInfoEx(::std::mem::transmute(hmprserver), ::std::mem::transmute(pserverinfo)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn MprAdminTransportCreate<'a, Param2: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>, Param7: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>>(hmprserver: isize, dwtransportid: u32, lpwstransportname: Param2, pglobalinfo: *const u8, dwglobalinfosize: u32, pclientinterfaceinfo: *const u8, dwclientinterfaceinfosize: u32, lpwsdllpath: Param7) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn MprAdminTransportCreate(hmprserver: isize, dwtransportid: u32, lpwstransportname: super::super::Foundation::PWSTR, pglobalinfo: *const u8, dwglobalinfosize: u32, pclientinterfaceinfo: *const u8, dwclientinterfaceinfosize: u32, lpwsdllpath: super::super::Foundation::PWSTR) -> u32;
        }
        ::std::mem::transmute(MprAdminTransportCreate(
            ::std::mem::transmute(hmprserver),
            ::std::mem::transmute(dwtransportid),
            lpwstransportname.into_param().abi(),
            ::std::mem::transmute(pglobalinfo),
            ::std::mem::transmute(dwglobalinfosize),
            ::std::mem::transmute(pclientinterfaceinfo),
            ::std::mem::transmute(dwclientinterfaceinfosize),
            lpwsdllpath.into_param().abi(),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn MprAdminTransportGetInfo(hmprserver: isize, dwtransportid: u32, ppglobalinfo: *mut *mut u8, lpdwglobalinfosize: *mut u32, ppclientinterfaceinfo: *mut *mut u8, lpdwclientinterfaceinfosize: *mut u32) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn MprAdminTransportGetInfo(hmprserver: isize, dwtransportid: u32, ppglobalinfo: *mut *mut u8, lpdwglobalinfosize: *mut u32, ppclientinterfaceinfo: *mut *mut u8, lpdwclientinterfaceinfosize: *mut u32) -> u32;
        }
        ::std::mem::transmute(MprAdminTransportGetInfo(::std::mem::transmute(hmprserver), ::std::mem::transmute(dwtransportid), ::std::mem::transmute(ppglobalinfo), ::std::mem::transmute(lpdwglobalinfosize), ::std::mem::transmute(ppclientinterfaceinfo), ::std::mem::transmute(lpdwclientinterfaceinfosize)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn MprAdminTransportSetInfo(hmprserver: isize, dwtransportid: u32, pglobalinfo: *const u8, dwglobalinfosize: u32, pclientinterfaceinfo: *const u8, dwclientinterfaceinfosize: u32) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn MprAdminTransportSetInfo(hmprserver: isize, dwtransportid: u32, pglobalinfo: *const u8, dwglobalinfosize: u32, pclientinterfaceinfo: *const u8, dwclientinterfaceinfosize: u32) -> u32;
        }
        ::std::mem::transmute(MprAdminTransportSetInfo(::std::mem::transmute(hmprserver), ::std::mem::transmute(dwtransportid), ::std::mem::transmute(pglobalinfo), ::std::mem::transmute(dwglobalinfosize), ::std::mem::transmute(pclientinterfaceinfo), ::std::mem::transmute(dwclientinterfaceinfosize)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn MprAdminUpdateConnection<'a, Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::HANDLE>>(hrasserver: isize, hrasconnection: Param1, prasupdateconnection: *const RAS_UPDATE_CONNECTION) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn MprAdminUpdateConnection(hrasserver: isize, hrasconnection: super::super::Foundation::HANDLE, prasupdateconnection: *const RAS_UPDATE_CONNECTION) -> u32;
        }
        ::std::mem::transmute(MprAdminUpdateConnection(::std::mem::transmute(hrasserver), hrasconnection.into_param().abi(), ::std::mem::transmute(prasupdateconnection)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn MprAdminUserGetInfo<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>, Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>>(lpszserver: Param0, lpszuser: Param1, dwlevel: u32, lpbbuffer: *mut u8) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn MprAdminUserGetInfo(lpszserver: super::super::Foundation::PWSTR, lpszuser: super::super::Foundation::PWSTR, dwlevel: u32, lpbbuffer: *mut u8) -> u32;
        }
        ::std::mem::transmute(MprAdminUserGetInfo(lpszserver.into_param().abi(), lpszuser.into_param().abi(), ::std::mem::transmute(dwlevel), ::std::mem::transmute(lpbbuffer)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn MprAdminUserSetInfo<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>, Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>>(lpszserver: Param0, lpszuser: Param1, dwlevel: u32, lpbbuffer: *const u8) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn MprAdminUserSetInfo(lpszserver: super::super::Foundation::PWSTR, lpszuser: super::super::Foundation::PWSTR, dwlevel: u32, lpbbuffer: *const u8) -> u32;
        }
        ::std::mem::transmute(MprAdminUserSetInfo(lpszserver.into_param().abi(), lpszuser.into_param().abi(), ::std::mem::transmute(dwlevel), ::std::mem::transmute(lpbbuffer)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn MprConfigBufferFree(pbuffer: *const ::std::ffi::c_void) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn MprConfigBufferFree(pbuffer: *const ::std::ffi::c_void) -> u32;
        }
        ::std::mem::transmute(MprConfigBufferFree(::std::mem::transmute(pbuffer)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn MprConfigFilterGetInfo<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::HANDLE>>(hmprconfig: Param0, dwlevel: u32, dwtransportid: u32, lpbuffer: *mut u8) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn MprConfigFilterGetInfo(hmprconfig: super::super::Foundation::HANDLE, dwlevel: u32, dwtransportid: u32, lpbuffer: *mut u8) -> u32;
        }
        ::std::mem::transmute(MprConfigFilterGetInfo(hmprconfig.into_param().abi(), ::std::mem::transmute(dwlevel), ::std::mem::transmute(dwtransportid), ::std::mem::transmute(lpbuffer)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn MprConfigFilterSetInfo<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::HANDLE>>(hmprconfig: Param0, dwlevel: u32, dwtransportid: u32, lpbuffer: *const u8) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn MprConfigFilterSetInfo(hmprconfig: super::super::Foundation::HANDLE, dwlevel: u32, dwtransportid: u32, lpbuffer: *const u8) -> u32;
        }
        ::std::mem::transmute(MprConfigFilterSetInfo(hmprconfig.into_param().abi(), ::std::mem::transmute(dwlevel), ::std::mem::transmute(dwtransportid), ::std::mem::transmute(lpbuffer)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn MprConfigGetFriendlyName<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::HANDLE>, Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>>(hmprconfig: Param0, pszguidname: Param1, pszbuffer: super::super::Foundation::PWSTR, dwbuffersize: u32) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn MprConfigGetFriendlyName(hmprconfig: super::super::Foundation::HANDLE, pszguidname: super::super::Foundation::PWSTR, pszbuffer: super::super::Foundation::PWSTR, dwbuffersize: u32) -> u32;
        }
        ::std::mem::transmute(MprConfigGetFriendlyName(hmprconfig.into_param().abi(), pszguidname.into_param().abi(), ::std::mem::transmute(pszbuffer), ::std::mem::transmute(dwbuffersize)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn MprConfigGetGuidName<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::HANDLE>, Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>>(hmprconfig: Param0, pszfriendlyname: Param1, pszbuffer: super::super::Foundation::PWSTR, dwbuffersize: u32) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn MprConfigGetGuidName(hmprconfig: super::super::Foundation::HANDLE, pszfriendlyname: super::super::Foundation::PWSTR, pszbuffer: super::super::Foundation::PWSTR, dwbuffersize: u32) -> u32;
        }
        ::std::mem::transmute(MprConfigGetGuidName(hmprconfig.into_param().abi(), pszfriendlyname.into_param().abi(), ::std::mem::transmute(pszbuffer), ::std::mem::transmute(dwbuffersize)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn MprConfigInterfaceCreate<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::HANDLE>>(hmprconfig: Param0, dwlevel: u32, lpbbuffer: *const u8, phrouterinterface: *mut super::super::Foundation::HANDLE) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn MprConfigInterfaceCreate(hmprconfig: super::super::Foundation::HANDLE, dwlevel: u32, lpbbuffer: *const u8, phrouterinterface: *mut super::super::Foundation::HANDLE) -> u32;
        }
        ::std::mem::transmute(MprConfigInterfaceCreate(hmprconfig.into_param().abi(), ::std::mem::transmute(dwlevel), ::std::mem::transmute(lpbbuffer), ::std::mem::transmute(phrouterinterface)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn MprConfigInterfaceDelete<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::HANDLE>, Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::HANDLE>>(hmprconfig: Param0, hrouterinterface: Param1) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn MprConfigInterfaceDelete(hmprconfig: super::super::Foundation::HANDLE, hrouterinterface: super::super::Foundation::HANDLE) -> u32;
        }
        ::std::mem::transmute(MprConfigInterfaceDelete(hmprconfig.into_param().abi(), hrouterinterface.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn MprConfigInterfaceEnum<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::HANDLE>>(hmprconfig: Param0, dwlevel: u32, lplpbuffer: *mut *mut u8, dwprefmaxlen: u32, lpdwentriesread: *mut u32, lpdwtotalentries: *mut u32, lpdwresumehandle: *mut u32) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn MprConfigInterfaceEnum(hmprconfig: super::super::Foundation::HANDLE, dwlevel: u32, lplpbuffer: *mut *mut u8, dwprefmaxlen: u32, lpdwentriesread: *mut u32, lpdwtotalentries: *mut u32, lpdwresumehandle: *mut u32) -> u32;
        }
        ::std::mem::transmute(MprConfigInterfaceEnum(hmprconfig.into_param().abi(), ::std::mem::transmute(dwlevel), ::std::mem::transmute(lplpbuffer), ::std::mem::transmute(dwprefmaxlen), ::std::mem::transmute(lpdwentriesread), ::std::mem::transmute(lpdwtotalentries), ::std::mem::transmute(lpdwresumehandle)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock", feature = "Win32_Security_Cryptography"))]
#[inline]
pub unsafe fn MprConfigInterfaceGetCustomInfoEx<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::HANDLE>, Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::HANDLE>>(hmprconfig: Param0, hrouterinterface: Param1, pcustominfo: *mut MPR_IF_CUSTOMINFOEX2) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn MprConfigInterfaceGetCustomInfoEx(hmprconfig: super::super::Foundation::HANDLE, hrouterinterface: super::super::Foundation::HANDLE, pcustominfo: *mut MPR_IF_CUSTOMINFOEX2) -> u32;
        }
        ::std::mem::transmute(MprConfigInterfaceGetCustomInfoEx(hmprconfig.into_param().abi(), hrouterinterface.into_param().abi(), ::std::mem::transmute(pcustominfo)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn MprConfigInterfaceGetHandle<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::HANDLE>, Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>>(hmprconfig: Param0, lpwsinterfacename: Param1, phrouterinterface: *mut super::super::Foundation::HANDLE) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn MprConfigInterfaceGetHandle(hmprconfig: super::super::Foundation::HANDLE, lpwsinterfacename: super::super::Foundation::PWSTR, phrouterinterface: *mut super::super::Foundation::HANDLE) -> u32;
        }
        ::std::mem::transmute(MprConfigInterfaceGetHandle(hmprconfig.into_param().abi(), lpwsinterfacename.into_param().abi(), ::std::mem::transmute(phrouterinterface)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn MprConfigInterfaceGetInfo<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::HANDLE>, Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::HANDLE>>(hmprconfig: Param0, hrouterinterface: Param1, dwlevel: u32, lplpbuffer: *mut *mut u8, lpdwbuffersize: *mut u32) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn MprConfigInterfaceGetInfo(hmprconfig: super::super::Foundation::HANDLE, hrouterinterface: super::super::Foundation::HANDLE, dwlevel: u32, lplpbuffer: *mut *mut u8, lpdwbuffersize: *mut u32) -> u32;
        }
        ::std::mem::transmute(MprConfigInterfaceGetInfo(hmprconfig.into_param().abi(), hrouterinterface.into_param().abi(), ::std::mem::transmute(dwlevel), ::std::mem::transmute(lplpbuffer), ::std::mem::transmute(lpdwbuffersize)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock", feature = "Win32_Security_Cryptography"))]
#[inline]
pub unsafe fn MprConfigInterfaceSetCustomInfoEx<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::HANDLE>, Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::HANDLE>>(hmprconfig: Param0, hrouterinterface: Param1, pcustominfo: *const MPR_IF_CUSTOMINFOEX2) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn MprConfigInterfaceSetCustomInfoEx(hmprconfig: super::super::Foundation::HANDLE, hrouterinterface: super::super::Foundation::HANDLE, pcustominfo: *const MPR_IF_CUSTOMINFOEX2) -> u32;
        }
        ::std::mem::transmute(MprConfigInterfaceSetCustomInfoEx(hmprconfig.into_param().abi(), hrouterinterface.into_param().abi(), ::std::mem::transmute(pcustominfo)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn MprConfigInterfaceSetInfo<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::HANDLE>, Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::HANDLE>>(hmprconfig: Param0, hrouterinterface: Param1, dwlevel: u32, lpbbuffer: *const u8) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn MprConfigInterfaceSetInfo(hmprconfig: super::super::Foundation::HANDLE, hrouterinterface: super::super::Foundation::HANDLE, dwlevel: u32, lpbbuffer: *const u8) -> u32;
        }
        ::std::mem::transmute(MprConfigInterfaceSetInfo(hmprconfig.into_param().abi(), hrouterinterface.into_param().abi(), ::std::mem::transmute(dwlevel), ::std::mem::transmute(lpbbuffer)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn MprConfigInterfaceTransportAdd<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::HANDLE>, Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::HANDLE>, Param3: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>>(hmprconfig: Param0, hrouterinterface: Param1, dwtransportid: u32, lpwstransportname: Param3, pinterfaceinfo: *const u8, dwinterfaceinfosize: u32, phrouteriftransport: *mut super::super::Foundation::HANDLE) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn MprConfigInterfaceTransportAdd(hmprconfig: super::super::Foundation::HANDLE, hrouterinterface: super::super::Foundation::HANDLE, dwtransportid: u32, lpwstransportname: super::super::Foundation::PWSTR, pinterfaceinfo: *const u8, dwinterfaceinfosize: u32, phrouteriftransport: *mut super::super::Foundation::HANDLE) -> u32;
        }
        ::std::mem::transmute(MprConfigInterfaceTransportAdd(
            hmprconfig.into_param().abi(),
            hrouterinterface.into_param().abi(),
            ::std::mem::transmute(dwtransportid),
            lpwstransportname.into_param().abi(),
            ::std::mem::transmute(pinterfaceinfo),
            ::std::mem::transmute(dwinterfaceinfosize),
            ::std::mem::transmute(phrouteriftransport),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn MprConfigInterfaceTransportEnum<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::HANDLE>, Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::HANDLE>>(hmprconfig: Param0, hrouterinterface: Param1, dwlevel: u32, lplpbuffer: *mut *mut u8, dwprefmaxlen: u32, lpdwentriesread: *mut u32, lpdwtotalentries: *mut u32, lpdwresumehandle: *mut u32) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn MprConfigInterfaceTransportEnum(hmprconfig: super::super::Foundation::HANDLE, hrouterinterface: super::super::Foundation::HANDLE, dwlevel: u32, lplpbuffer: *mut *mut u8, dwprefmaxlen: u32, lpdwentriesread: *mut u32, lpdwtotalentries: *mut u32, lpdwresumehandle: *mut u32) -> u32;
        }
        ::std::mem::transmute(MprConfigInterfaceTransportEnum(
            hmprconfig.into_param().abi(),
            hrouterinterface.into_param().abi(),
            ::std::mem::transmute(dwlevel),
            ::std::mem::transmute(lplpbuffer),
            ::std::mem::transmute(dwprefmaxlen),
            ::std::mem::transmute(lpdwentriesread),
            ::std::mem::transmute(lpdwtotalentries),
            ::std::mem::transmute(lpdwresumehandle),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn MprConfigInterfaceTransportGetHandle<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::HANDLE>, Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::HANDLE>>(hmprconfig: Param0, hrouterinterface: Param1, dwtransportid: u32, phrouteriftransport: *mut super::super::Foundation::HANDLE) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn MprConfigInterfaceTransportGetHandle(hmprconfig: super::super::Foundation::HANDLE, hrouterinterface: super::super::Foundation::HANDLE, dwtransportid: u32, phrouteriftransport: *mut super::super::Foundation::HANDLE) -> u32;
        }
        ::std::mem::transmute(MprConfigInterfaceTransportGetHandle(hmprconfig.into_param().abi(), hrouterinterface.into_param().abi(), ::std::mem::transmute(dwtransportid), ::std::mem::transmute(phrouteriftransport)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn MprConfigInterfaceTransportGetInfo<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::HANDLE>, Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::HANDLE>, Param2: ::windows::runtime::IntoParam<'a, super::super::Foundation::HANDLE>>(hmprconfig: Param0, hrouterinterface: Param1, hrouteriftransport: Param2, ppinterfaceinfo: *mut *mut u8, lpdwinterfaceinfosize: *mut u32) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn MprConfigInterfaceTransportGetInfo(hmprconfig: super::super::Foundation::HANDLE, hrouterinterface: super::super::Foundation::HANDLE, hrouteriftransport: super::super::Foundation::HANDLE, ppinterfaceinfo: *mut *mut u8, lpdwinterfaceinfosize: *mut u32) -> u32;
        }
        ::std::mem::transmute(MprConfigInterfaceTransportGetInfo(hmprconfig.into_param().abi(), hrouterinterface.into_param().abi(), hrouteriftransport.into_param().abi(), ::std::mem::transmute(ppinterfaceinfo), ::std::mem::transmute(lpdwinterfaceinfosize)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn MprConfigInterfaceTransportRemove<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::HANDLE>, Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::HANDLE>, Param2: ::windows::runtime::IntoParam<'a, super::super::Foundation::HANDLE>>(hmprconfig: Param0, hrouterinterface: Param1, hrouteriftransport: Param2) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn MprConfigInterfaceTransportRemove(hmprconfig: super::super::Foundation::HANDLE, hrouterinterface: super::super::Foundation::HANDLE, hrouteriftransport: super::super::Foundation::HANDLE) -> u32;
        }
        ::std::mem::transmute(MprConfigInterfaceTransportRemove(hmprconfig.into_param().abi(), hrouterinterface.into_param().abi(), hrouteriftransport.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn MprConfigInterfaceTransportSetInfo<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::HANDLE>, Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::HANDLE>, Param2: ::windows::runtime::IntoParam<'a, super::super::Foundation::HANDLE>>(hmprconfig: Param0, hrouterinterface: Param1, hrouteriftransport: Param2, pinterfaceinfo: *const u8, dwinterfaceinfosize: u32) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn MprConfigInterfaceTransportSetInfo(hmprconfig: super::super::Foundation::HANDLE, hrouterinterface: super::super::Foundation::HANDLE, hrouteriftransport: super::super::Foundation::HANDLE, pinterfaceinfo: *const u8, dwinterfaceinfosize: u32) -> u32;
        }
        ::std::mem::transmute(MprConfigInterfaceTransportSetInfo(hmprconfig.into_param().abi(), hrouterinterface.into_param().abi(), hrouteriftransport.into_param().abi(), ::std::mem::transmute(pinterfaceinfo), ::std::mem::transmute(dwinterfaceinfosize)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn MprConfigServerBackup<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::HANDLE>, Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>>(hmprconfig: Param0, lpwspath: Param1) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn MprConfigServerBackup(hmprconfig: super::super::Foundation::HANDLE, lpwspath: super::super::Foundation::PWSTR) -> u32;
        }
        ::std::mem::transmute(MprConfigServerBackup(hmprconfig.into_param().abi(), lpwspath.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn MprConfigServerConnect<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>>(lpwsservername: Param0, phmprconfig: *mut super::super::Foundation::HANDLE) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn MprConfigServerConnect(lpwsservername: super::super::Foundation::PWSTR, phmprconfig: *mut super::super::Foundation::HANDLE) -> u32;
        }
        ::std::mem::transmute(MprConfigServerConnect(lpwsservername.into_param().abi(), ::std::mem::transmute(phmprconfig)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn MprConfigServerDisconnect<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::HANDLE>>(hmprconfig: Param0) {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn MprConfigServerDisconnect(hmprconfig: super::super::Foundation::HANDLE);
        }
        ::std::mem::transmute(MprConfigServerDisconnect(hmprconfig.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn MprConfigServerGetInfo<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::HANDLE>>(hmprconfig: Param0, dwlevel: u32, lplpbbuffer: *mut *mut u8) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn MprConfigServerGetInfo(hmprconfig: super::super::Foundation::HANDLE, dwlevel: u32, lplpbbuffer: *mut *mut u8) -> u32;
        }
        ::std::mem::transmute(MprConfigServerGetInfo(hmprconfig.into_param().abi(), ::std::mem::transmute(dwlevel), ::std::mem::transmute(lplpbbuffer)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography"))]
#[inline]
pub unsafe fn MprConfigServerGetInfoEx<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::HANDLE>>(hmprconfig: Param0, pserverinfo: *mut MPR_SERVER_EX1) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn MprConfigServerGetInfoEx(hmprconfig: super::super::Foundation::HANDLE, pserverinfo: *mut MPR_SERVER_EX1) -> u32;
        }
        ::std::mem::transmute(MprConfigServerGetInfoEx(hmprconfig.into_param().abi(), ::std::mem::transmute(pserverinfo)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn MprConfigServerInstall(dwlevel: u32, pbuffer: *const ::std::ffi::c_void) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn MprConfigServerInstall(dwlevel: u32, pbuffer: *const ::std::ffi::c_void) -> u32;
        }
        ::std::mem::transmute(MprConfigServerInstall(::std::mem::transmute(dwlevel), ::std::mem::transmute(pbuffer)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn MprConfigServerRefresh<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::HANDLE>>(hmprconfig: Param0) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn MprConfigServerRefresh(hmprconfig: super::super::Foundation::HANDLE) -> u32;
        }
        ::std::mem::transmute(MprConfigServerRefresh(hmprconfig.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn MprConfigServerRestore<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::HANDLE>, Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>>(hmprconfig: Param0, lpwspath: Param1) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn MprConfigServerRestore(hmprconfig: super::super::Foundation::HANDLE, lpwspath: super::super::Foundation::PWSTR) -> u32;
        }
        ::std::mem::transmute(MprConfigServerRestore(hmprconfig.into_param().abi(), lpwspath.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn MprConfigServerSetInfo(hmprserver: isize, dwlevel: u32, lpbbuffer: *const u8) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn MprConfigServerSetInfo(hmprserver: isize, dwlevel: u32, lpbbuffer: *const u8) -> u32;
        }
        ::std::mem::transmute(MprConfigServerSetInfo(::std::mem::transmute(hmprserver), ::std::mem::transmute(dwlevel), ::std::mem::transmute(lpbbuffer)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography"))]
#[inline]
pub unsafe fn MprConfigServerSetInfoEx<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::HANDLE>>(hmprconfig: Param0, psetserverconfig: *const MPR_SERVER_SET_CONFIG_EX1) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn MprConfigServerSetInfoEx(hmprconfig: super::super::Foundation::HANDLE, psetserverconfig: *const MPR_SERVER_SET_CONFIG_EX1) -> u32;
        }
        ::std::mem::transmute(MprConfigServerSetInfoEx(hmprconfig.into_param().abi(), ::std::mem::transmute(psetserverconfig)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn MprConfigTransportCreate<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::HANDLE>, Param2: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>, Param7: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>>(
    hmprconfig: Param0,
    dwtransportid: u32,
    lpwstransportname: Param2,
    pglobalinfo: *const u8,
    dwglobalinfosize: u32,
    pclientinterfaceinfo: *const u8,
    dwclientinterfaceinfosize: u32,
    lpwsdllpath: Param7,
    phroutertransport: *mut super::super::Foundation::HANDLE,
) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn MprConfigTransportCreate(hmprconfig: super::super::Foundation::HANDLE, dwtransportid: u32, lpwstransportname: super::super::Foundation::PWSTR, pglobalinfo: *const u8, dwglobalinfosize: u32, pclientinterfaceinfo: *const u8, dwclientinterfaceinfosize: u32, lpwsdllpath: super::super::Foundation::PWSTR, phroutertransport: *mut super::super::Foundation::HANDLE) -> u32;
        }
        ::std::mem::transmute(MprConfigTransportCreate(
            hmprconfig.into_param().abi(),
            ::std::mem::transmute(dwtransportid),
            lpwstransportname.into_param().abi(),
            ::std::mem::transmute(pglobalinfo),
            ::std::mem::transmute(dwglobalinfosize),
            ::std::mem::transmute(pclientinterfaceinfo),
            ::std::mem::transmute(dwclientinterfaceinfosize),
            lpwsdllpath.into_param().abi(),
            ::std::mem::transmute(phroutertransport),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn MprConfigTransportDelete<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::HANDLE>, Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::HANDLE>>(hmprconfig: Param0, hroutertransport: Param1) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn MprConfigTransportDelete(hmprconfig: super::super::Foundation::HANDLE, hroutertransport: super::super::Foundation::HANDLE) -> u32;
        }
        ::std::mem::transmute(MprConfigTransportDelete(hmprconfig.into_param().abi(), hroutertransport.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn MprConfigTransportEnum<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::HANDLE>>(hmprconfig: Param0, dwlevel: u32, lplpbuffer: *mut *mut u8, dwprefmaxlen: u32, lpdwentriesread: *mut u32, lpdwtotalentries: *mut u32, lpdwresumehandle: *mut u32) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn MprConfigTransportEnum(hmprconfig: super::super::Foundation::HANDLE, dwlevel: u32, lplpbuffer: *mut *mut u8, dwprefmaxlen: u32, lpdwentriesread: *mut u32, lpdwtotalentries: *mut u32, lpdwresumehandle: *mut u32) -> u32;
        }
        ::std::mem::transmute(MprConfigTransportEnum(hmprconfig.into_param().abi(), ::std::mem::transmute(dwlevel), ::std::mem::transmute(lplpbuffer), ::std::mem::transmute(dwprefmaxlen), ::std::mem::transmute(lpdwentriesread), ::std::mem::transmute(lpdwtotalentries), ::std::mem::transmute(lpdwresumehandle)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn MprConfigTransportGetHandle<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::HANDLE>>(hmprconfig: Param0, dwtransportid: u32, phroutertransport: *mut super::super::Foundation::HANDLE) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn MprConfigTransportGetHandle(hmprconfig: super::super::Foundation::HANDLE, dwtransportid: u32, phroutertransport: *mut super::super::Foundation::HANDLE) -> u32;
        }
        ::std::mem::transmute(MprConfigTransportGetHandle(hmprconfig.into_param().abi(), ::std::mem::transmute(dwtransportid), ::std::mem::transmute(phroutertransport)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn MprConfigTransportGetInfo<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::HANDLE>, Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::HANDLE>>(hmprconfig: Param0, hroutertransport: Param1, ppglobalinfo: *mut *mut u8, lpdwglobalinfosize: *mut u32, ppclientinterfaceinfo: *mut *mut u8, lpdwclientinterfaceinfosize: *mut u32, lplpwsdllpath: *mut super::super::Foundation::PWSTR) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn MprConfigTransportGetInfo(hmprconfig: super::super::Foundation::HANDLE, hroutertransport: super::super::Foundation::HANDLE, ppglobalinfo: *mut *mut u8, lpdwglobalinfosize: *mut u32, ppclientinterfaceinfo: *mut *mut u8, lpdwclientinterfaceinfosize: *mut u32, lplpwsdllpath: *mut super::super::Foundation::PWSTR) -> u32;
        }
        ::std::mem::transmute(MprConfigTransportGetInfo(
            hmprconfig.into_param().abi(),
            hroutertransport.into_param().abi(),
            ::std::mem::transmute(ppglobalinfo),
            ::std::mem::transmute(lpdwglobalinfosize),
            ::std::mem::transmute(ppclientinterfaceinfo),
            ::std::mem::transmute(lpdwclientinterfaceinfosize),
            ::std::mem::transmute(lplpwsdllpath),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn MprConfigTransportSetInfo<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::HANDLE>, Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::HANDLE>, Param6: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>>(hmprconfig: Param0, hroutertransport: Param1, pglobalinfo: *const u8, dwglobalinfosize: u32, pclientinterfaceinfo: *const u8, dwclientinterfaceinfosize: u32, lpwsdllpath: Param6) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn MprConfigTransportSetInfo(hmprconfig: super::super::Foundation::HANDLE, hroutertransport: super::super::Foundation::HANDLE, pglobalinfo: *const u8, dwglobalinfosize: u32, pclientinterfaceinfo: *const u8, dwclientinterfaceinfosize: u32, lpwsdllpath: super::super::Foundation::PWSTR) -> u32;
        }
        ::std::mem::transmute(MprConfigTransportSetInfo(hmprconfig.into_param().abi(), hroutertransport.into_param().abi(), ::std::mem::transmute(pglobalinfo), ::std::mem::transmute(dwglobalinfosize), ::std::mem::transmute(pclientinterfaceinfo), ::std::mem::transmute(dwclientinterfaceinfosize), lpwsdllpath.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn MprInfoBlockAdd(lpheader: *const ::std::ffi::c_void, dwinfotype: u32, dwitemsize: u32, dwitemcount: u32, lpitemdata: *const u8, lplpnewheader: *mut *mut ::std::ffi::c_void) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn MprInfoBlockAdd(lpheader: *const ::std::ffi::c_void, dwinfotype: u32, dwitemsize: u32, dwitemcount: u32, lpitemdata: *const u8, lplpnewheader: *mut *mut ::std::ffi::c_void) -> u32;
        }
        ::std::mem::transmute(MprInfoBlockAdd(::std::mem::transmute(lpheader), ::std::mem::transmute(dwinfotype), ::std::mem::transmute(dwitemsize), ::std::mem::transmute(dwitemcount), ::std::mem::transmute(lpitemdata), ::std::mem::transmute(lplpnewheader)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn MprInfoBlockFind(lpheader: *const ::std::ffi::c_void, dwinfotype: u32, lpdwitemsize: *mut u32, lpdwitemcount: *mut u32, lplpitemdata: *mut *mut u8) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn MprInfoBlockFind(lpheader: *const ::std::ffi::c_void, dwinfotype: u32, lpdwitemsize: *mut u32, lpdwitemcount: *mut u32, lplpitemdata: *mut *mut u8) -> u32;
        }
        ::std::mem::transmute(MprInfoBlockFind(::std::mem::transmute(lpheader), ::std::mem::transmute(dwinfotype), ::std::mem::transmute(lpdwitemsize), ::std::mem::transmute(lpdwitemcount), ::std::mem::transmute(lplpitemdata)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn MprInfoBlockQuerySize(lpheader: *const ::std::ffi::c_void) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn MprInfoBlockQuerySize(lpheader: *const ::std::ffi::c_void) -> u32;
        }
        ::std::mem::transmute(MprInfoBlockQuerySize(::std::mem::transmute(lpheader)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn MprInfoBlockRemove(lpheader: *const ::std::ffi::c_void, dwinfotype: u32, lplpnewheader: *mut *mut ::std::ffi::c_void) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn MprInfoBlockRemove(lpheader: *const ::std::ffi::c_void, dwinfotype: u32, lplpnewheader: *mut *mut ::std::ffi::c_void) -> u32;
        }
        ::std::mem::transmute(MprInfoBlockRemove(::std::mem::transmute(lpheader), ::std::mem::transmute(dwinfotype), ::std::mem::transmute(lplpnewheader)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn MprInfoBlockSet(lpheader: *const ::std::ffi::c_void, dwinfotype: u32, dwitemsize: u32, dwitemcount: u32, lpitemdata: *const u8, lplpnewheader: *mut *mut ::std::ffi::c_void) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn MprInfoBlockSet(lpheader: *const ::std::ffi::c_void, dwinfotype: u32, dwitemsize: u32, dwitemcount: u32, lpitemdata: *const u8, lplpnewheader: *mut *mut ::std::ffi::c_void) -> u32;
        }
        ::std::mem::transmute(MprInfoBlockSet(::std::mem::transmute(lpheader), ::std::mem::transmute(dwinfotype), ::std::mem::transmute(dwitemsize), ::std::mem::transmute(dwitemcount), ::std::mem::transmute(lpitemdata), ::std::mem::transmute(lplpnewheader)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn MprInfoCreate(dwversion: u32, lplpnewheader: *mut *mut ::std::ffi::c_void) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn MprInfoCreate(dwversion: u32, lplpnewheader: *mut *mut ::std::ffi::c_void) -> u32;
        }
        ::std::mem::transmute(MprInfoCreate(::std::mem::transmute(dwversion), ::std::mem::transmute(lplpnewheader)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn MprInfoDelete(lpheader: *const ::std::ffi::c_void) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn MprInfoDelete(lpheader: *const ::std::ffi::c_void) -> u32;
        }
        ::std::mem::transmute(MprInfoDelete(::std::mem::transmute(lpheader)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn MprInfoDuplicate(lpheader: *const ::std::ffi::c_void, lplpnewheader: *mut *mut ::std::ffi::c_void) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn MprInfoDuplicate(lpheader: *const ::std::ffi::c_void, lplpnewheader: *mut *mut ::std::ffi::c_void) -> u32;
        }
        ::std::mem::transmute(MprInfoDuplicate(::std::mem::transmute(lpheader), ::std::mem::transmute(lplpnewheader)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn MprInfoRemoveAll(lpheader: *const ::std::ffi::c_void, lplpnewheader: *mut *mut ::std::ffi::c_void) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn MprInfoRemoveAll(lpheader: *const ::std::ffi::c_void, lplpnewheader: *mut *mut ::std::ffi::c_void) -> u32;
        }
        ::std::mem::transmute(MprInfoRemoveAll(::std::mem::transmute(lpheader), ::std::mem::transmute(lplpnewheader)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
pub type ORASADFUNC = unsafe extern "system" fn(param0: super::super::Foundation::HWND, param1: super::super::Foundation::PSTR, param2: u32, param3: *mut u32) -> super::super::Foundation::BOOL;
pub const PENDING: u32 = 600u32;
pub type PFNRASFREEBUFFER = unsafe extern "system" fn(pbufer: *mut u8) -> u32;
pub type PFNRASGETBUFFER = unsafe extern "system" fn(ppbuffer: *mut *mut u8, pdwsize: *mut u32) -> u32;
#[cfg(feature = "Win32_Foundation")]
pub type PFNRASRECEIVEBUFFER = unsafe extern "system" fn(hport: super::super::Foundation::HANDLE, pbuffer: *mut u8, pdwsize: *mut u32, dwtimeout: u32, hevent: super::super::Foundation::HANDLE) -> u32;
#[cfg(feature = "Win32_Foundation")]
pub type PFNRASRETRIEVEBUFFER = unsafe extern "system" fn(hport: super::super::Foundation::HANDLE, pbuffer: *mut u8, pdwsize: *mut u32) -> u32;
#[cfg(feature = "Win32_Foundation")]
pub type PFNRASSENDBUFFER = unsafe extern "system" fn(hport: super::super::Foundation::HANDLE, pbuffer: *mut u8, dwsize: u32) -> u32;
#[cfg(feature = "Win32_Foundation")]
pub type PFNRASSETCOMMSETTINGS = unsafe extern "system" fn(hport: super::super::Foundation::HANDLE, prascommsettings: *mut RASCOMMSETTINGS, pvreserved: *mut ::std::ffi::c_void) -> u32;
pub const PID_ATALK: u32 = 41u32;
pub const PID_IP: u32 = 33u32;
pub const PID_IPV6: u32 = 87u32;
pub const PID_IPX: u32 = 43u32;
pub const PID_NBF: u32 = 63u32;
#[cfg(feature = "Win32_Foundation")]
pub type PMGM_CREATION_ALERT_CALLBACK = unsafe extern "system" fn(dwsourceaddr: u32, dwsourcemask: u32, dwgroupaddr: u32, dwgroupmask: u32, dwinifindex: u32, dwinifnexthopaddr: u32, dwifcount: u32, pmieoutiflist: *mut MGM_IF_ENTRY) -> u32;
pub type PMGM_DISABLE_IGMP_CALLBACK = unsafe extern "system" fn(dwifindex: u32, dwifnexthopaddr: u32) -> u32;
pub type PMGM_ENABLE_IGMP_CALLBACK = unsafe extern "system" fn(dwifindex: u32, dwifnexthopaddr: u32) -> u32;
#[cfg(feature = "Win32_Foundation")]
pub type PMGM_JOIN_ALERT_CALLBACK = unsafe extern "system" fn(dwsourceaddr: u32, dwsourcemask: u32, dwgroupaddr: u32, dwgroupmask: u32, bmemberupdate: super::super::Foundation::BOOL) -> u32;
pub type PMGM_LOCAL_JOIN_CALLBACK = unsafe extern "system" fn(dwsourceaddr: u32, dwsourcemask: u32, dwgroupaddr: u32, dwgroupmask: u32, dwifindex: u32, dwifnexthopaddr: u32) -> u32;
pub type PMGM_LOCAL_LEAVE_CALLBACK = unsafe extern "system" fn(dwsourceaddr: u32, dwsourcemask: u32, dwgroupaddr: u32, dwgroupmask: u32, dwifindex: u32, dwifnexthopaddr: u32) -> u32;
#[cfg(feature = "Win32_Foundation")]
pub type PMGM_PRUNE_ALERT_CALLBACK = unsafe extern "system" fn(dwsourceaddr: u32, dwsourcemask: u32, dwgroupaddr: u32, dwgroupmask: u32, dwifindex: u32, dwifnexthopaddr: u32, bmemberdelete: super::super::Foundation::BOOL, pdwtimeout: *mut u32) -> u32;
pub type PMGM_RPF_CALLBACK = unsafe extern "system" fn(dwsourceaddr: u32, dwsourcemask: u32, dwgroupaddr: u32, dwgroupmask: u32, pdwinifindex: *mut u32, pdwinifnexthopaddr: *mut u32, pdwupstreamnbr: *mut u32, dwhdrsize: u32, pbpackethdr: *mut u8, pbroute: *mut u8) -> u32;
pub type PMGM_WRONG_IF_CALLBACK = unsafe extern "system" fn(dwsourceaddr: u32, dwgroupaddr: u32, dwifindex: u32, dwifnexthopaddr: u32, dwhdrsize: u32, pbpackethdr: *mut u8) -> u32;
#[cfg(feature = "Win32_Foundation")]
pub type PMPRADMINACCEPTNEWCONNECTION = unsafe extern "system" fn(param0: *mut RAS_CONNECTION_0, param1: *mut RAS_CONNECTION_1) -> super::super::Foundation::BOOL;
#[cfg(feature = "Win32_Foundation")]
pub type PMPRADMINACCEPTNEWCONNECTION2 = unsafe extern "system" fn(param0: *mut RAS_CONNECTION_0, param1: *mut RAS_CONNECTION_1, param2: *mut RAS_CONNECTION_2) -> super::super::Foundation::BOOL;
#[cfg(feature = "Win32_Foundation")]
pub type PMPRADMINACCEPTNEWCONNECTION3 = unsafe extern "system" fn(param0: *mut RAS_CONNECTION_0, param1: *mut RAS_CONNECTION_1, param2: *mut RAS_CONNECTION_2, param3: *mut RAS_CONNECTION_3) -> super::super::Foundation::BOOL;
#[cfg(feature = "Win32_Foundation")]
pub type PMPRADMINACCEPTNEWCONNECTIONEX = unsafe extern "system" fn(param0: *mut RAS_CONNECTION_EX) -> super::super::Foundation::BOOL;
#[cfg(feature = "Win32_Foundation")]
pub type PMPRADMINACCEPTNEWLINK = unsafe extern "system" fn(param0: *mut RAS_PORT_0, param1: *mut RAS_PORT_1) -> super::super::Foundation::BOOL;
#[cfg(feature = "Win32_Foundation")]
pub type PMPRADMINACCEPTREAUTHENTICATION = unsafe extern "system" fn(param0: *mut RAS_CONNECTION_0, param1: *mut RAS_CONNECTION_1, param2: *mut RAS_CONNECTION_2, param3: *mut RAS_CONNECTION_3) -> super::super::Foundation::BOOL;
#[cfg(feature = "Win32_Foundation")]
pub type PMPRADMINACCEPTREAUTHENTICATIONEX = unsafe extern "system" fn(param0: *mut RAS_CONNECTION_EX) -> super::super::Foundation::BOOL;
#[cfg(feature = "Win32_Foundation")]
pub type PMPRADMINACCEPTTUNNELENDPOINTCHANGEEX = unsafe extern "system" fn(param0: *mut RAS_CONNECTION_EX) -> super::super::Foundation::BOOL;
#[cfg(feature = "Win32_Foundation")]
pub type PMPRADMINCONNECTIONHANGUPNOTIFICATION = unsafe extern "system" fn(param0: *mut RAS_CONNECTION_0, param1: *mut RAS_CONNECTION_1);
#[cfg(feature = "Win32_Foundation")]
pub type PMPRADMINCONNECTIONHANGUPNOTIFICATION2 = unsafe extern "system" fn(param0: *mut RAS_CONNECTION_0, param1: *mut RAS_CONNECTION_1, param2: *mut RAS_CONNECTION_2);
#[cfg(feature = "Win32_Foundation")]
pub type PMPRADMINCONNECTIONHANGUPNOTIFICATION3 = unsafe extern "system" fn(param0: *mut RAS_CONNECTION_0, param1: *mut RAS_CONNECTION_1, param2: *mut RAS_CONNECTION_2, param3: RAS_CONNECTION_3);
#[cfg(feature = "Win32_Foundation")]
pub type PMPRADMINCONNECTIONHANGUPNOTIFICATIONEX = unsafe extern "system" fn(param0: *mut RAS_CONNECTION_EX);
#[cfg(feature = "Win32_Foundation")]
pub type PMPRADMINGETIPADDRESSFORUSER = unsafe extern "system" fn(param0: super::super::Foundation::PWSTR, param1: super::super::Foundation::PWSTR, param2: *mut u32, param3: *mut super::super::Foundation::BOOL) -> u32;
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
pub type PMPRADMINGETIPV6ADDRESSFORUSER = unsafe extern "system" fn(param0: super::super::Foundation::PWSTR, param1: super::super::Foundation::PWSTR, param2: *mut super::super::Networking::WinSock::IN6_ADDR, param3: *mut super::super::Foundation::BOOL) -> u32;
#[cfg(feature = "Win32_Foundation")]
pub type PMPRADMINLINKHANGUPNOTIFICATION = unsafe extern "system" fn(param0: *mut RAS_PORT_0, param1: *mut RAS_PORT_1);
#[cfg(feature = "Win32_Foundation")]
pub type PMPRADMINRASVALIDATEPREAUTHENTICATEDCONNECTIONEX = unsafe extern "system" fn(param0: *mut AUTH_VALIDATION_EX) -> u32;
#[cfg(feature = "Win32_Foundation")]
pub type PMPRADMINRELEASEIPADRESS = unsafe extern "system" fn(param0: super::super::Foundation::PWSTR, param1: super::super::Foundation::PWSTR, param2: *mut u32);
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
pub type PMPRADMINRELEASEIPV6ADDRESSFORUSER = unsafe extern "system" fn(param0: super::super::Foundation::PWSTR, param1: super::super::Foundation::PWSTR, param2: *mut super::super::Networking::WinSock::IN6_ADDR);
pub type PMPRADMINTERMINATEDLL = unsafe extern "system" fn() -> u32;
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
pub struct PPP_ATCP_INFO {
    pub dwError: u32,
    pub wszAddress: [u16; 33],
}
impl PPP_ATCP_INFO {}
impl ::std::default::Default for PPP_ATCP_INFO {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for PPP_ATCP_INFO {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("PPP_ATCP_INFO").field("dwError", &self.dwError).field("wszAddress", &self.wszAddress).finish()
    }
}
impl ::std::cmp::PartialEq for PPP_ATCP_INFO {
    fn eq(&self, other: &Self) -> bool {
        self.dwError == other.dwError && self.wszAddress == other.wszAddress
    }
}
impl ::std::cmp::Eq for PPP_ATCP_INFO {}
unsafe impl ::windows::runtime::Abi for PPP_ATCP_INFO {
    type Abi = Self;
    type DefaultType = Self;
}
pub const PPP_CCP_COMPRESSION: u32 = 1u32;
pub const PPP_CCP_ENCRYPTION128BIT: u32 = 64u32;
pub const PPP_CCP_ENCRYPTION40BIT: u32 = 32u32;
pub const PPP_CCP_ENCRYPTION40BITOLD: u32 = 16u32;
pub const PPP_CCP_ENCRYPTION56BIT: u32 = 128u32;
pub const PPP_CCP_HISTORYLESS: u32 = 16777216u32;
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
pub struct PPP_CCP_INFO {
    pub dwError: u32,
    pub dwCompressionAlgorithm: u32,
    pub dwOptions: u32,
    pub dwRemoteCompressionAlgorithm: u32,
    pub dwRemoteOptions: u32,
}
impl PPP_CCP_INFO {}
impl ::std::default::Default for PPP_CCP_INFO {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for PPP_CCP_INFO {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("PPP_CCP_INFO").field("dwError", &self.dwError).field("dwCompressionAlgorithm", &self.dwCompressionAlgorithm).field("dwOptions", &self.dwOptions).field("dwRemoteCompressionAlgorithm", &self.dwRemoteCompressionAlgorithm).field("dwRemoteOptions", &self.dwRemoteOptions).finish()
    }
}
impl ::std::cmp::PartialEq for PPP_CCP_INFO {
    fn eq(&self, other: &Self) -> bool {
        self.dwError == other.dwError && self.dwCompressionAlgorithm == other.dwCompressionAlgorithm && self.dwOptions == other.dwOptions && self.dwRemoteCompressionAlgorithm == other.dwRemoteCompressionAlgorithm && self.dwRemoteOptions == other.dwRemoteOptions
    }
}
impl ::std::cmp::Eq for PPP_CCP_INFO {}
unsafe impl ::windows::runtime::Abi for PPP_CCP_INFO {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
pub struct PPP_INFO {
    pub nbf: PPP_NBFCP_INFO,
    pub ip: PPP_IPCP_INFO,
    pub ipx: PPP_IPXCP_INFO,
    pub at: PPP_ATCP_INFO,
}
impl PPP_INFO {}
impl ::std::default::Default for PPP_INFO {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for PPP_INFO {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("PPP_INFO").field("nbf", &self.nbf).field("ip", &self.ip).field("ipx", &self.ipx).field("at", &self.at).finish()
    }
}
impl ::std::cmp::PartialEq for PPP_INFO {
    fn eq(&self, other: &Self) -> bool {
        self.nbf == other.nbf && self.ip == other.ip && self.ipx == other.ipx && self.at == other.at
    }
}
impl ::std::cmp::Eq for PPP_INFO {}
unsafe impl ::windows::runtime::Abi for PPP_INFO {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
pub struct PPP_INFO_2 {
    pub nbf: PPP_NBFCP_INFO,
    pub ip: PPP_IPCP_INFO2,
    pub ipx: PPP_IPXCP_INFO,
    pub at: PPP_ATCP_INFO,
    pub ccp: PPP_CCP_INFO,
    pub lcp: PPP_LCP_INFO,
}
impl PPP_INFO_2 {}
impl ::std::default::Default for PPP_INFO_2 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for PPP_INFO_2 {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("PPP_INFO_2").field("nbf", &self.nbf).field("ip", &self.ip).field("ipx", &self.ipx).field("at", &self.at).field("ccp", &self.ccp).field("lcp", &self.lcp).finish()
    }
}
impl ::std::cmp::PartialEq for PPP_INFO_2 {
    fn eq(&self, other: &Self) -> bool {
        self.nbf == other.nbf && self.ip == other.ip && self.ipx == other.ipx && self.at == other.at && self.ccp == other.ccp && self.lcp == other.lcp
    }
}
impl ::std::cmp::Eq for PPP_INFO_2 {}
unsafe impl ::windows::runtime::Abi for PPP_INFO_2 {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
pub struct PPP_INFO_3 {
    pub nbf: PPP_NBFCP_INFO,
    pub ip: PPP_IPCP_INFO2,
    pub ipv6: PPP_IPV6_CP_INFO,
    pub ccp: PPP_CCP_INFO,
    pub lcp: PPP_LCP_INFO,
}
impl PPP_INFO_3 {}
impl ::std::default::Default for PPP_INFO_3 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for PPP_INFO_3 {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("PPP_INFO_3").field("nbf", &self.nbf).field("ip", &self.ip).field("ipv6", &self.ipv6).field("ccp", &self.ccp).field("lcp", &self.lcp).finish()
    }
}
impl ::std::cmp::PartialEq for PPP_INFO_3 {
    fn eq(&self, other: &Self) -> bool {
        self.nbf == other.nbf && self.ip == other.ip && self.ipv6 == other.ipv6 && self.ccp == other.ccp && self.lcp == other.lcp
    }
}
impl ::std::cmp::Eq for PPP_INFO_3 {}
unsafe impl ::windows::runtime::Abi for PPP_INFO_3 {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
pub struct PPP_IPCP_INFO {
    pub dwError: u32,
    pub wszAddress: [u16; 16],
    pub wszRemoteAddress: [u16; 16],
}
impl PPP_IPCP_INFO {}
impl ::std::default::Default for PPP_IPCP_INFO {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for PPP_IPCP_INFO {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("PPP_IPCP_INFO").field("dwError", &self.dwError).field("wszAddress", &self.wszAddress).field("wszRemoteAddress", &self.wszRemoteAddress).finish()
    }
}
impl ::std::cmp::PartialEq for PPP_IPCP_INFO {
    fn eq(&self, other: &Self) -> bool {
        self.dwError == other.dwError && self.wszAddress == other.wszAddress && self.wszRemoteAddress == other.wszRemoteAddress
    }
}
impl ::std::cmp::Eq for PPP_IPCP_INFO {}
unsafe impl ::windows::runtime::Abi for PPP_IPCP_INFO {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
pub struct PPP_IPCP_INFO2 {
    pub dwError: u32,
    pub wszAddress: [u16; 16],
    pub wszRemoteAddress: [u16; 16],
    pub dwOptions: u32,
    pub dwRemoteOptions: u32,
}
impl PPP_IPCP_INFO2 {}
impl ::std::default::Default for PPP_IPCP_INFO2 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for PPP_IPCP_INFO2 {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("PPP_IPCP_INFO2").field("dwError", &self.dwError).field("wszAddress", &self.wszAddress).field("wszRemoteAddress", &self.wszRemoteAddress).field("dwOptions", &self.dwOptions).field("dwRemoteOptions", &self.dwRemoteOptions).finish()
    }
}
impl ::std::cmp::PartialEq for PPP_IPCP_INFO2 {
    fn eq(&self, other: &Self) -> bool {
        self.dwError == other.dwError && self.wszAddress == other.wszAddress && self.wszRemoteAddress == other.wszRemoteAddress && self.dwOptions == other.dwOptions && self.dwRemoteOptions == other.dwRemoteOptions
    }
}
impl ::std::cmp::Eq for PPP_IPCP_INFO2 {}
unsafe impl ::windows::runtime::Abi for PPP_IPCP_INFO2 {
    type Abi = Self;
    type DefaultType = Self;
}
pub const PPP_IPCP_VJ: u32 = 1u32;
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
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
impl PPP_IPV6_CP_INFO {}
impl ::std::default::Default for PPP_IPV6_CP_INFO {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for PPP_IPV6_CP_INFO {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("PPP_IPV6_CP_INFO")
            .field("dwVersion", &self.dwVersion)
            .field("dwSize", &self.dwSize)
            .field("dwError", &self.dwError)
            .field("bInterfaceIdentifier", &self.bInterfaceIdentifier)
            .field("bRemoteInterfaceIdentifier", &self.bRemoteInterfaceIdentifier)
            .field("dwOptions", &self.dwOptions)
            .field("dwRemoteOptions", &self.dwRemoteOptions)
            .field("bPrefix", &self.bPrefix)
            .field("dwPrefixLength", &self.dwPrefixLength)
            .finish()
    }
}
impl ::std::cmp::PartialEq for PPP_IPV6_CP_INFO {
    fn eq(&self, other: &Self) -> bool {
        self.dwVersion == other.dwVersion && self.dwSize == other.dwSize && self.dwError == other.dwError && self.bInterfaceIdentifier == other.bInterfaceIdentifier && self.bRemoteInterfaceIdentifier == other.bRemoteInterfaceIdentifier && self.dwOptions == other.dwOptions && self.dwRemoteOptions == other.dwRemoteOptions && self.bPrefix == other.bPrefix && self.dwPrefixLength == other.dwPrefixLength
    }
}
impl ::std::cmp::Eq for PPP_IPV6_CP_INFO {}
unsafe impl ::windows::runtime::Abi for PPP_IPV6_CP_INFO {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
pub struct PPP_IPXCP_INFO {
    pub dwError: u32,
    pub wszAddress: [u16; 23],
}
impl PPP_IPXCP_INFO {}
impl ::std::default::Default for PPP_IPXCP_INFO {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for PPP_IPXCP_INFO {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("PPP_IPXCP_INFO").field("dwError", &self.dwError).field("wszAddress", &self.wszAddress).finish()
    }
}
impl ::std::cmp::PartialEq for PPP_IPXCP_INFO {
    fn eq(&self, other: &Self) -> bool {
        self.dwError == other.dwError && self.wszAddress == other.wszAddress
    }
}
impl ::std::cmp::Eq for PPP_IPXCP_INFO {}
unsafe impl ::windows::runtime::Abi for PPP_IPXCP_INFO {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct PPP_LCP(pub u32);
pub const PPP_LCP_PAP: PPP_LCP = PPP_LCP(49187u32);
pub const PPP_LCP_CHAP: PPP_LCP = PPP_LCP(49699u32);
pub const PPP_LCP_EAP: PPP_LCP = PPP_LCP(49703u32);
pub const PPP_LCP_SPAP: PPP_LCP = PPP_LCP(49191u32);
impl ::std::convert::From<u32> for PPP_LCP {
    fn from(value: u32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for PPP_LCP {
    type Abi = Self;
    type DefaultType = Self;
}
impl ::std::ops::BitOr for PPP_LCP {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self {
        Self(self.0 | rhs.0)
    }
}
impl ::std::ops::BitAnd for PPP_LCP {
    type Output = Self;
    fn bitand(self, rhs: Self) -> Self {
        Self(self.0 & rhs.0)
    }
}
impl ::std::ops::BitOrAssign for PPP_LCP {
    fn bitor_assign(&mut self, rhs: Self) {
        self.0.bitor_assign(rhs.0)
    }
}
impl ::std::ops::BitAndAssign for PPP_LCP {
    fn bitand_assign(&mut self, rhs: Self) {
        self.0.bitand_assign(rhs.0)
    }
}
impl ::std::ops::Not for PPP_LCP {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
pub const PPP_LCP_3_DES: u32 = 32u32;
pub const PPP_LCP_ACFC: u32 = 4u32;
pub const PPP_LCP_AES_128: u32 = 64u32;
pub const PPP_LCP_AES_192: u32 = 256u32;
pub const PPP_LCP_AES_256: u32 = 128u32;
pub const PPP_LCP_DES_56: u32 = 16u32;
pub const PPP_LCP_GCM_AES_128: u32 = 512u32;
pub const PPP_LCP_GCM_AES_192: u32 = 1024u32;
pub const PPP_LCP_GCM_AES_256: u32 = 2048u32;
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
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
impl PPP_LCP_INFO {}
impl ::std::default::Default for PPP_LCP_INFO {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for PPP_LCP_INFO {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("PPP_LCP_INFO")
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
impl ::std::cmp::PartialEq for PPP_LCP_INFO {
    fn eq(&self, other: &Self) -> bool {
        self.dwError == other.dwError
            && self.dwAuthenticationProtocol == other.dwAuthenticationProtocol
            && self.dwAuthenticationData == other.dwAuthenticationData
            && self.dwRemoteAuthenticationProtocol == other.dwRemoteAuthenticationProtocol
            && self.dwRemoteAuthenticationData == other.dwRemoteAuthenticationData
            && self.dwTerminateReason == other.dwTerminateReason
            && self.dwRemoteTerminateReason == other.dwRemoteTerminateReason
            && self.dwOptions == other.dwOptions
            && self.dwRemoteOptions == other.dwRemoteOptions
            && self.dwEapTypeId == other.dwEapTypeId
            && self.dwRemoteEapTypeId == other.dwRemoteEapTypeId
    }
}
impl ::std::cmp::Eq for PPP_LCP_INFO {}
unsafe impl ::windows::runtime::Abi for PPP_LCP_INFO {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct PPP_LCP_INFO_AUTH_DATA(pub u32);
pub const PPP_LCP_CHAP_MD5: PPP_LCP_INFO_AUTH_DATA = PPP_LCP_INFO_AUTH_DATA(5u32);
pub const PPP_LCP_CHAP_MS: PPP_LCP_INFO_AUTH_DATA = PPP_LCP_INFO_AUTH_DATA(128u32);
pub const PPP_LCP_CHAP_MSV2: PPP_LCP_INFO_AUTH_DATA = PPP_LCP_INFO_AUTH_DATA(129u32);
impl ::std::convert::From<u32> for PPP_LCP_INFO_AUTH_DATA {
    fn from(value: u32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for PPP_LCP_INFO_AUTH_DATA {
    type Abi = Self;
    type DefaultType = Self;
}
impl ::std::ops::BitOr for PPP_LCP_INFO_AUTH_DATA {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self {
        Self(self.0 | rhs.0)
    }
}
impl ::std::ops::BitAnd for PPP_LCP_INFO_AUTH_DATA {
    type Output = Self;
    fn bitand(self, rhs: Self) -> Self {
        Self(self.0 & rhs.0)
    }
}
impl ::std::ops::BitOrAssign for PPP_LCP_INFO_AUTH_DATA {
    fn bitor_assign(&mut self, rhs: Self) {
        self.0.bitor_assign(rhs.0)
    }
}
impl ::std::ops::BitAndAssign for PPP_LCP_INFO_AUTH_DATA {
    fn bitand_assign(&mut self, rhs: Self) {
        self.0.bitand_assign(rhs.0)
    }
}
impl ::std::ops::Not for PPP_LCP_INFO_AUTH_DATA {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
pub const PPP_LCP_MULTILINK_FRAMING: u32 = 1u32;
pub const PPP_LCP_PFC: u32 = 2u32;
pub const PPP_LCP_SSHF: u32 = 8u32;
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
pub struct PPP_NBFCP_INFO {
    pub dwError: u32,
    pub wszWksta: [u16; 17],
}
impl PPP_NBFCP_INFO {}
impl ::std::default::Default for PPP_NBFCP_INFO {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for PPP_NBFCP_INFO {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("PPP_NBFCP_INFO").field("dwError", &self.dwError).field("wszWksta", &self.wszWksta).finish()
    }
}
impl ::std::cmp::PartialEq for PPP_NBFCP_INFO {
    fn eq(&self, other: &Self) -> bool {
        self.dwError == other.dwError && self.wszWksta == other.wszWksta
    }
}
impl ::std::cmp::Eq for PPP_NBFCP_INFO {}
unsafe impl ::windows::runtime::Abi for PPP_NBFCP_INFO {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
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
impl PPP_PROJECTION_INFO {}
impl ::std::default::Default for PPP_PROJECTION_INFO {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for PPP_PROJECTION_INFO {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("PPP_PROJECTION_INFO")
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
impl ::std::cmp::PartialEq for PPP_PROJECTION_INFO {
    fn eq(&self, other: &Self) -> bool {
        self.dwIPv4NegotiationError == other.dwIPv4NegotiationError
            && self.wszAddress == other.wszAddress
            && self.wszRemoteAddress == other.wszRemoteAddress
            && self.dwIPv4Options == other.dwIPv4Options
            && self.dwIPv4RemoteOptions == other.dwIPv4RemoteOptions
            && self.IPv4SubInterfaceIndex == other.IPv4SubInterfaceIndex
            && self.dwIPv6NegotiationError == other.dwIPv6NegotiationError
            && self.bInterfaceIdentifier == other.bInterfaceIdentifier
            && self.bRemoteInterfaceIdentifier == other.bRemoteInterfaceIdentifier
            && self.bPrefix == other.bPrefix
            && self.dwPrefixLength == other.dwPrefixLength
            && self.IPv6SubInterfaceIndex == other.IPv6SubInterfaceIndex
            && self.dwLcpError == other.dwLcpError
            && self.dwAuthenticationProtocol == other.dwAuthenticationProtocol
            && self.dwAuthenticationData == other.dwAuthenticationData
            && self.dwRemoteAuthenticationProtocol == other.dwRemoteAuthenticationProtocol
            && self.dwRemoteAuthenticationData == other.dwRemoteAuthenticationData
            && self.dwLcpTerminateReason == other.dwLcpTerminateReason
            && self.dwLcpRemoteTerminateReason == other.dwLcpRemoteTerminateReason
            && self.dwLcpOptions == other.dwLcpOptions
            && self.dwLcpRemoteOptions == other.dwLcpRemoteOptions
            && self.dwEapTypeId == other.dwEapTypeId
            && self.dwRemoteEapTypeId == other.dwRemoteEapTypeId
            && self.dwCcpError == other.dwCcpError
            && self.dwCompressionAlgorithm == other.dwCompressionAlgorithm
            && self.dwCcpOptions == other.dwCcpOptions
            && self.dwRemoteCompressionAlgorithm == other.dwRemoteCompressionAlgorithm
            && self.dwCcpRemoteOptions == other.dwCcpRemoteOptions
    }
}
impl ::std::cmp::Eq for PPP_PROJECTION_INFO {}
unsafe impl ::windows::runtime::Abi for PPP_PROJECTION_INFO {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
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
impl PPP_PROJECTION_INFO2 {}
impl ::std::default::Default for PPP_PROJECTION_INFO2 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for PPP_PROJECTION_INFO2 {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("PPP_PROJECTION_INFO2")
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
impl ::std::cmp::PartialEq for PPP_PROJECTION_INFO2 {
    fn eq(&self, other: &Self) -> bool {
        self.dwIPv4NegotiationError == other.dwIPv4NegotiationError
            && self.wszAddress == other.wszAddress
            && self.wszRemoteAddress == other.wszRemoteAddress
            && self.dwIPv4Options == other.dwIPv4Options
            && self.dwIPv4RemoteOptions == other.dwIPv4RemoteOptions
            && self.IPv4SubInterfaceIndex == other.IPv4SubInterfaceIndex
            && self.dwIPv6NegotiationError == other.dwIPv6NegotiationError
            && self.bInterfaceIdentifier == other.bInterfaceIdentifier
            && self.bRemoteInterfaceIdentifier == other.bRemoteInterfaceIdentifier
            && self.bPrefix == other.bPrefix
            && self.dwPrefixLength == other.dwPrefixLength
            && self.IPv6SubInterfaceIndex == other.IPv6SubInterfaceIndex
            && self.dwLcpError == other.dwLcpError
            && self.dwAuthenticationProtocol == other.dwAuthenticationProtocol
            && self.dwAuthenticationData == other.dwAuthenticationData
            && self.dwRemoteAuthenticationProtocol == other.dwRemoteAuthenticationProtocol
            && self.dwRemoteAuthenticationData == other.dwRemoteAuthenticationData
            && self.dwLcpTerminateReason == other.dwLcpTerminateReason
            && self.dwLcpRemoteTerminateReason == other.dwLcpRemoteTerminateReason
            && self.dwLcpOptions == other.dwLcpOptions
            && self.dwLcpRemoteOptions == other.dwLcpRemoteOptions
            && self.dwEapTypeId == other.dwEapTypeId
            && self.dwEmbeddedEAPTypeId == other.dwEmbeddedEAPTypeId
            && self.dwRemoteEapTypeId == other.dwRemoteEapTypeId
            && self.dwCcpError == other.dwCcpError
            && self.dwCompressionAlgorithm == other.dwCompressionAlgorithm
            && self.dwCcpOptions == other.dwCcpOptions
            && self.dwRemoteCompressionAlgorithm == other.dwRemoteCompressionAlgorithm
            && self.dwCcpRemoteOptions == other.dwCcpRemoteOptions
    }
}
impl ::std::cmp::Eq for PPP_PROJECTION_INFO2 {}
unsafe impl ::windows::runtime::Abi for PPP_PROJECTION_INFO2 {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
pub struct PPTP_CONFIG_PARAMS {
    pub dwNumPorts: u32,
    pub dwPortFlags: u32,
}
impl PPTP_CONFIG_PARAMS {}
impl ::std::default::Default for PPTP_CONFIG_PARAMS {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for PPTP_CONFIG_PARAMS {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("PPTP_CONFIG_PARAMS").field("dwNumPorts", &self.dwNumPorts).field("dwPortFlags", &self.dwPortFlags).finish()
    }
}
impl ::std::cmp::PartialEq for PPTP_CONFIG_PARAMS {
    fn eq(&self, other: &Self) -> bool {
        self.dwNumPorts == other.dwNumPorts && self.dwPortFlags == other.dwPortFlags
    }
}
impl ::std::cmp::Eq for PPTP_CONFIG_PARAMS {}
unsafe impl ::windows::runtime::Abi for PPTP_CONFIG_PARAMS {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
pub struct PROJECTION_INFO {
    pub projectionInfoType: u8,
    pub Anonymous: PROJECTION_INFO_0,
}
impl PROJECTION_INFO {}
impl ::std::default::Default for PROJECTION_INFO {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::cmp::PartialEq for PROJECTION_INFO {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
impl ::std::cmp::Eq for PROJECTION_INFO {}
unsafe impl ::windows::runtime::Abi for PROJECTION_INFO {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
pub union PROJECTION_INFO_0 {
    pub PppProjectionInfo: PPP_PROJECTION_INFO,
    pub Ikev2ProjectionInfo: IKEV2_PROJECTION_INFO,
}
impl PROJECTION_INFO_0 {}
impl ::std::default::Default for PROJECTION_INFO_0 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::cmp::PartialEq for PROJECTION_INFO_0 {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
impl ::std::cmp::Eq for PROJECTION_INFO_0 {}
unsafe impl ::windows::runtime::Abi for PROJECTION_INFO_0 {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
pub struct PROJECTION_INFO2 {
    pub projectionInfoType: u8,
    pub Anonymous: PROJECTION_INFO2_0,
}
impl PROJECTION_INFO2 {}
impl ::std::default::Default for PROJECTION_INFO2 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::cmp::PartialEq for PROJECTION_INFO2 {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
impl ::std::cmp::Eq for PROJECTION_INFO2 {}
unsafe impl ::windows::runtime::Abi for PROJECTION_INFO2 {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
pub union PROJECTION_INFO2_0 {
    pub PppProjectionInfo: PPP_PROJECTION_INFO2,
    pub Ikev2ProjectionInfo: IKEV2_PROJECTION_INFO2,
}
impl PROJECTION_INFO2_0 {}
impl ::std::default::Default for PROJECTION_INFO2_0 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::cmp::PartialEq for PROJECTION_INFO2_0 {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
impl ::std::cmp::Eq for PROJECTION_INFO2_0 {}
unsafe impl ::windows::runtime::Abi for PROJECTION_INFO2_0 {
    type Abi = Self;
    type DefaultType = Self;
}
pub const RASADFLG_PositionDlg: u32 = 1u32;
#[cfg(feature = "Win32_Foundation")]
pub type RASADFUNCA = unsafe extern "system" fn(param0: super::super::Foundation::PSTR, param1: super::super::Foundation::PSTR, param2: *mut RASADPARAMS, param3: *mut u32) -> super::super::Foundation::BOOL;
#[cfg(feature = "Win32_Foundation")]
pub type RASADFUNCW = unsafe extern "system" fn(param0: super::super::Foundation::PWSTR, param1: super::super::Foundation::PWSTR, param2: *mut RASADPARAMS, param3: *mut u32) -> super::super::Foundation::BOOL;
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C, packed(4))]
#[cfg(feature = "Win32_Foundation")]
pub struct RASADPARAMS {
    pub dwSize: u32,
    pub hwndOwner: super::super::Foundation::HWND,
    pub dwFlags: u32,
    pub xDlg: i32,
    pub yDlg: i32,
}
#[cfg(feature = "Win32_Foundation")]
impl RASADPARAMS {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for RASADPARAMS {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for RASADPARAMS {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for RASADPARAMS {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for RASADPARAMS {
    type Abi = Self;
    type DefaultType = Self;
}
pub const RASADP_ConnectionQueryTimeout: u32 = 4u32;
pub const RASADP_DisableConnectionQuery: u32 = 0u32;
pub const RASADP_FailedConnectionTimeout: u32 = 3u32;
pub const RASADP_LoginSessionDisable: u32 = 1u32;
pub const RASADP_SavedAddressesLimit: u32 = 2u32;
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct RASAMBA {
    pub dwSize: u32,
    pub dwError: u32,
    pub szNetBiosError: [super::super::Foundation::CHAR; 17],
    pub bLana: u8,
}
#[cfg(feature = "Win32_Foundation")]
impl RASAMBA {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for RASAMBA {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::fmt::Debug for RASAMBA {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("RASAMBA").field("dwSize", &self.dwSize).field("dwError", &self.dwError).field("szNetBiosError", &self.szNetBiosError).field("bLana", &self.bLana).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for RASAMBA {
    fn eq(&self, other: &Self) -> bool {
        self.dwSize == other.dwSize && self.dwError == other.dwError && self.szNetBiosError == other.szNetBiosError && self.bLana == other.bLana
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for RASAMBA {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for RASAMBA {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
pub struct RASAMBW {
    pub dwSize: u32,
    pub dwError: u32,
    pub szNetBiosError: [u16; 17],
    pub bLana: u8,
}
impl RASAMBW {}
impl ::std::default::Default for RASAMBW {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for RASAMBW {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("RASAMBW").field("dwSize", &self.dwSize).field("dwError", &self.dwError).field("szNetBiosError", &self.szNetBiosError).field("bLana", &self.bLana).finish()
    }
}
impl ::std::cmp::PartialEq for RASAMBW {
    fn eq(&self, other: &Self) -> bool {
        self.dwSize == other.dwSize && self.dwError == other.dwError && self.szNetBiosError == other.szNetBiosError && self.bLana == other.bLana
    }
}
impl ::std::cmp::Eq for RASAMBW {}
unsafe impl ::windows::runtime::Abi for RASAMBW {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct RASAPIVERSION(pub i32);
pub const RASAPIVERSION_500: RASAPIVERSION = RASAPIVERSION(1i32);
pub const RASAPIVERSION_501: RASAPIVERSION = RASAPIVERSION(2i32);
pub const RASAPIVERSION_600: RASAPIVERSION = RASAPIVERSION(3i32);
pub const RASAPIVERSION_601: RASAPIVERSION = RASAPIVERSION(4i32);
impl ::std::convert::From<i32> for RASAPIVERSION {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for RASAPIVERSION {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct RASAUTODIALENTRYA {
    pub dwSize: u32,
    pub dwFlags: u32,
    pub dwDialingLocation: u32,
    pub szEntry: [super::super::Foundation::CHAR; 257],
}
#[cfg(feature = "Win32_Foundation")]
impl RASAUTODIALENTRYA {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for RASAUTODIALENTRYA {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::fmt::Debug for RASAUTODIALENTRYA {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("RASAUTODIALENTRYA").field("dwSize", &self.dwSize).field("dwFlags", &self.dwFlags).field("dwDialingLocation", &self.dwDialingLocation).field("szEntry", &self.szEntry).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for RASAUTODIALENTRYA {
    fn eq(&self, other: &Self) -> bool {
        self.dwSize == other.dwSize && self.dwFlags == other.dwFlags && self.dwDialingLocation == other.dwDialingLocation && self.szEntry == other.szEntry
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for RASAUTODIALENTRYA {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for RASAUTODIALENTRYA {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
pub struct RASAUTODIALENTRYW {
    pub dwSize: u32,
    pub dwFlags: u32,
    pub dwDialingLocation: u32,
    pub szEntry: [u16; 257],
}
impl RASAUTODIALENTRYW {}
impl ::std::default::Default for RASAUTODIALENTRYW {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for RASAUTODIALENTRYW {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("RASAUTODIALENTRYW").field("dwSize", &self.dwSize).field("dwFlags", &self.dwFlags).field("dwDialingLocation", &self.dwDialingLocation).field("szEntry", &self.szEntry).finish()
    }
}
impl ::std::cmp::PartialEq for RASAUTODIALENTRYW {
    fn eq(&self, other: &Self) -> bool {
        self.dwSize == other.dwSize && self.dwFlags == other.dwFlags && self.dwDialingLocation == other.dwDialingLocation && self.szEntry == other.szEntry
    }
}
impl ::std::cmp::Eq for RASAUTODIALENTRYW {}
unsafe impl ::windows::runtime::Abi for RASAUTODIALENTRYW {
    type Abi = Self;
    type DefaultType = Self;
}
pub const RASBASE: u32 = 600u32;
pub const RASBASEEND: u32 = 877u32;
pub const RASCCPCA_MPPC: u32 = 6u32;
pub const RASCCPCA_STAC: u32 = 5u32;
pub const RASCCPO_Compression: u32 = 1u32;
pub const RASCCPO_Encryption128bit: u32 = 64u32;
pub const RASCCPO_Encryption40bit: u32 = 32u32;
pub const RASCCPO_Encryption56bit: u32 = 16u32;
pub const RASCCPO_HistoryLess: u32 = 2u32;
pub const RASCF_AllUsers: u32 = 1u32;
pub const RASCF_GlobalCreds: u32 = 2u32;
pub const RASCF_OwnerKnown: u32 = 4u32;
pub const RASCF_OwnerMatch: u32 = 8u32;
pub const RASCM_DDMPreSharedKey: u32 = 64u32;
pub const RASCM_DefaultCreds: u32 = 8u32;
pub const RASCM_Domain: u32 = 4u32;
pub const RASCM_Password: u32 = 2u32;
pub const RASCM_PreSharedKey: u32 = 16u32;
pub const RASCM_ServerPreSharedKey: u32 = 32u32;
pub const RASCM_UserName: u32 = 1u32;
pub const RASCN_BandwidthAdded: u32 = 4u32;
pub const RASCN_BandwidthRemoved: u32 = 8u32;
pub const RASCN_Connection: u32 = 1u32;
pub const RASCN_Disconnection: u32 = 2u32;
pub const RASCN_Dormant: u32 = 16u32;
pub const RASCN_EPDGPacketArrival: u32 = 64u32;
pub const RASCN_ReConnection: u32 = 32u32;
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
pub struct RASCOMMSETTINGS {
    pub dwSize: u32,
    pub bParity: u8,
    pub bStop: u8,
    pub bByteSize: u8,
    pub bAlign: u8,
}
impl RASCOMMSETTINGS {}
impl ::std::default::Default for RASCOMMSETTINGS {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for RASCOMMSETTINGS {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("RASCOMMSETTINGS").field("dwSize", &self.dwSize).field("bParity", &self.bParity).field("bStop", &self.bStop).field("bByteSize", &self.bByteSize).field("bAlign", &self.bAlign).finish()
    }
}
impl ::std::cmp::PartialEq for RASCOMMSETTINGS {
    fn eq(&self, other: &Self) -> bool {
        self.dwSize == other.dwSize && self.bParity == other.bParity && self.bStop == other.bStop && self.bByteSize == other.bByteSize && self.bAlign == other.bAlign
    }
}
impl ::std::cmp::Eq for RASCOMMSETTINGS {}
unsafe impl ::windows::runtime::Abi for RASCOMMSETTINGS {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C, packed(4))]
#[cfg(feature = "Win32_Foundation")]
pub struct RASCONNA {
    pub dwSize: u32,
    pub hrasconn: HRASCONN,
    pub szEntryName: [super::super::Foundation::CHAR; 257],
    pub szDeviceType: [super::super::Foundation::CHAR; 17],
    pub szDeviceName: [super::super::Foundation::CHAR; 129],
    pub szPhonebook: [super::super::Foundation::CHAR; 260],
    pub dwSubEntry: u32,
    pub guidEntry: ::windows::runtime::GUID,
    pub dwFlags: u32,
    pub luid: super::super::Foundation::LUID,
    pub guidCorrelationId: ::windows::runtime::GUID,
}
#[cfg(feature = "Win32_Foundation")]
impl RASCONNA {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for RASCONNA {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for RASCONNA {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for RASCONNA {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for RASCONNA {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct RASCONNSTATE(pub i32);
pub const RASCS_OpenPort: RASCONNSTATE = RASCONNSTATE(0i32);
pub const RASCS_PortOpened: RASCONNSTATE = RASCONNSTATE(1i32);
pub const RASCS_ConnectDevice: RASCONNSTATE = RASCONNSTATE(2i32);
pub const RASCS_DeviceConnected: RASCONNSTATE = RASCONNSTATE(3i32);
pub const RASCS_AllDevicesConnected: RASCONNSTATE = RASCONNSTATE(4i32);
pub const RASCS_Authenticate: RASCONNSTATE = RASCONNSTATE(5i32);
pub const RASCS_AuthNotify: RASCONNSTATE = RASCONNSTATE(6i32);
pub const RASCS_AuthRetry: RASCONNSTATE = RASCONNSTATE(7i32);
pub const RASCS_AuthCallback: RASCONNSTATE = RASCONNSTATE(8i32);
pub const RASCS_AuthChangePassword: RASCONNSTATE = RASCONNSTATE(9i32);
pub const RASCS_AuthProject: RASCONNSTATE = RASCONNSTATE(10i32);
pub const RASCS_AuthLinkSpeed: RASCONNSTATE = RASCONNSTATE(11i32);
pub const RASCS_AuthAck: RASCONNSTATE = RASCONNSTATE(12i32);
pub const RASCS_ReAuthenticate: RASCONNSTATE = RASCONNSTATE(13i32);
pub const RASCS_Authenticated: RASCONNSTATE = RASCONNSTATE(14i32);
pub const RASCS_PrepareForCallback: RASCONNSTATE = RASCONNSTATE(15i32);
pub const RASCS_WaitForModemReset: RASCONNSTATE = RASCONNSTATE(16i32);
pub const RASCS_WaitForCallback: RASCONNSTATE = RASCONNSTATE(17i32);
pub const RASCS_Projected: RASCONNSTATE = RASCONNSTATE(18i32);
pub const RASCS_StartAuthentication: RASCONNSTATE = RASCONNSTATE(19i32);
pub const RASCS_CallbackComplete: RASCONNSTATE = RASCONNSTATE(20i32);
pub const RASCS_LogonNetwork: RASCONNSTATE = RASCONNSTATE(21i32);
pub const RASCS_SubEntryConnected: RASCONNSTATE = RASCONNSTATE(22i32);
pub const RASCS_SubEntryDisconnected: RASCONNSTATE = RASCONNSTATE(23i32);
pub const RASCS_ApplySettings: RASCONNSTATE = RASCONNSTATE(24i32);
pub const RASCS_Interactive: RASCONNSTATE = RASCONNSTATE(4096i32);
pub const RASCS_RetryAuthentication: RASCONNSTATE = RASCONNSTATE(4097i32);
pub const RASCS_CallbackSetByCaller: RASCONNSTATE = RASCONNSTATE(4098i32);
pub const RASCS_PasswordExpired: RASCONNSTATE = RASCONNSTATE(4099i32);
pub const RASCS_InvokeEapUI: RASCONNSTATE = RASCONNSTATE(4100i32);
pub const RASCS_Connected: RASCONNSTATE = RASCONNSTATE(8192i32);
pub const RASCS_Disconnected: RASCONNSTATE = RASCONNSTATE(8193i32);
impl ::std::convert::From<i32> for RASCONNSTATE {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for RASCONNSTATE {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
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
impl RASCONNSTATUSA {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
impl ::std::default::Default for RASCONNSTATUSA {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
impl ::std::cmp::PartialEq for RASCONNSTATUSA {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
impl ::std::cmp::Eq for RASCONNSTATUSA {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
unsafe impl ::windows::runtime::Abi for RASCONNSTATUSA {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
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
impl RASCONNSTATUSW {}
#[cfg(feature = "Win32_Networking_WinSock")]
impl ::std::default::Default for RASCONNSTATUSW {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Networking_WinSock")]
impl ::std::cmp::PartialEq for RASCONNSTATUSW {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
#[cfg(feature = "Win32_Networking_WinSock")]
impl ::std::cmp::Eq for RASCONNSTATUSW {}
#[cfg(feature = "Win32_Networking_WinSock")]
unsafe impl ::windows::runtime::Abi for RASCONNSTATUSW {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct RASCONNSUBSTATE(pub i32);
pub const RASCSS_None: RASCONNSUBSTATE = RASCONNSUBSTATE(0i32);
pub const RASCSS_Dormant: RASCONNSUBSTATE = RASCONNSUBSTATE(1i32);
pub const RASCSS_Reconnecting: RASCONNSUBSTATE = RASCONNSUBSTATE(2i32);
pub const RASCSS_Reconnected: RASCONNSUBSTATE = RASCONNSUBSTATE(8192i32);
impl ::std::convert::From<i32> for RASCONNSUBSTATE {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for RASCONNSUBSTATE {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C, packed(4))]
#[cfg(feature = "Win32_Foundation")]
pub struct RASCONNW {
    pub dwSize: u32,
    pub hrasconn: HRASCONN,
    pub szEntryName: [u16; 257],
    pub szDeviceType: [u16; 17],
    pub szDeviceName: [u16; 129],
    pub szPhonebook: [u16; 260],
    pub dwSubEntry: u32,
    pub guidEntry: ::windows::runtime::GUID,
    pub dwFlags: u32,
    pub luid: super::super::Foundation::LUID,
    pub guidCorrelationId: ::windows::runtime::GUID,
}
#[cfg(feature = "Win32_Foundation")]
impl RASCONNW {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for RASCONNW {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for RASCONNW {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for RASCONNW {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for RASCONNW {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct RASCREDENTIALSA {
    pub dwSize: u32,
    pub dwMask: u32,
    pub szUserName: [super::super::Foundation::CHAR; 257],
    pub szPassword: [super::super::Foundation::CHAR; 257],
    pub szDomain: [super::super::Foundation::CHAR; 16],
}
#[cfg(feature = "Win32_Foundation")]
impl RASCREDENTIALSA {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for RASCREDENTIALSA {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::fmt::Debug for RASCREDENTIALSA {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("RASCREDENTIALSA").field("dwSize", &self.dwSize).field("dwMask", &self.dwMask).field("szUserName", &self.szUserName).field("szPassword", &self.szPassword).field("szDomain", &self.szDomain).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for RASCREDENTIALSA {
    fn eq(&self, other: &Self) -> bool {
        self.dwSize == other.dwSize && self.dwMask == other.dwMask && self.szUserName == other.szUserName && self.szPassword == other.szPassword && self.szDomain == other.szDomain
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for RASCREDENTIALSA {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for RASCREDENTIALSA {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
pub struct RASCREDENTIALSW {
    pub dwSize: u32,
    pub dwMask: u32,
    pub szUserName: [u16; 257],
    pub szPassword: [u16; 257],
    pub szDomain: [u16; 16],
}
impl RASCREDENTIALSW {}
impl ::std::default::Default for RASCREDENTIALSW {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for RASCREDENTIALSW {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("RASCREDENTIALSW").field("dwSize", &self.dwSize).field("dwMask", &self.dwMask).field("szUserName", &self.szUserName).field("szPassword", &self.szPassword).field("szDomain", &self.szDomain).finish()
    }
}
impl ::std::cmp::PartialEq for RASCREDENTIALSW {
    fn eq(&self, other: &Self) -> bool {
        self.dwSize == other.dwSize && self.dwMask == other.dwMask && self.szUserName == other.szUserName && self.szPassword == other.szPassword && self.szDomain == other.szDomain
    }
}
impl ::std::cmp::Eq for RASCREDENTIALSW {}
unsafe impl ::windows::runtime::Abi for RASCREDENTIALSW {
    type Abi = Self;
    type DefaultType = Self;
}
pub const RASCSS_DONE: u32 = 8192u32;
pub const RASCS_DONE: u32 = 8192u32;
pub const RASCS_PAUSED: u32 = 4096u32;
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
pub struct RASCTRYINFO {
    pub dwSize: u32,
    pub dwCountryID: u32,
    pub dwNextCountryID: u32,
    pub dwCountryCode: u32,
    pub dwCountryNameOffset: u32,
}
impl RASCTRYINFO {}
impl ::std::default::Default for RASCTRYINFO {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for RASCTRYINFO {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("RASCTRYINFO").field("dwSize", &self.dwSize).field("dwCountryID", &self.dwCountryID).field("dwNextCountryID", &self.dwNextCountryID).field("dwCountryCode", &self.dwCountryCode).field("dwCountryNameOffset", &self.dwCountryNameOffset).finish()
    }
}
impl ::std::cmp::PartialEq for RASCTRYINFO {
    fn eq(&self, other: &Self) -> bool {
        self.dwSize == other.dwSize && self.dwCountryID == other.dwCountryID && self.dwNextCountryID == other.dwNextCountryID && self.dwCountryCode == other.dwCountryCode && self.dwCountryNameOffset == other.dwCountryNameOffset
    }
}
impl ::std::cmp::Eq for RASCTRYINFO {}
unsafe impl ::windows::runtime::Abi for RASCTRYINFO {
    type Abi = Self;
    type DefaultType = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::clone::Clone for RASCUSTOMSCRIPTEXTENSIONS {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
#[repr(C, packed(4))]
#[cfg(feature = "Win32_Foundation")]
pub struct RASCUSTOMSCRIPTEXTENSIONS {
    pub dwSize: u32,
    pub pfnRasSetCommSettings: ::std::option::Option<PFNRASSETCOMMSETTINGS>,
}
#[cfg(feature = "Win32_Foundation")]
impl RASCUSTOMSCRIPTEXTENSIONS {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for RASCUSTOMSCRIPTEXTENSIONS {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for RASCUSTOMSCRIPTEXTENSIONS {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for RASCUSTOMSCRIPTEXTENSIONS {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for RASCUSTOMSCRIPTEXTENSIONS {
    type Abi = ::std::mem::ManuallyDrop<Self>;
    type DefaultType = Self;
}
pub const RASDDFLAG_AoacRedial: u32 = 4u32;
pub const RASDDFLAG_LinkFailure: u32 = 2147483648u32;
pub const RASDDFLAG_NoPrompt: u32 = 2u32;
pub const RASDDFLAG_PositionDlg: u32 = 1u32;
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct RASDEVINFOA {
    pub dwSize: u32,
    pub szDeviceType: [super::super::Foundation::CHAR; 17],
    pub szDeviceName: [super::super::Foundation::CHAR; 129],
}
#[cfg(feature = "Win32_Foundation")]
impl RASDEVINFOA {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for RASDEVINFOA {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::fmt::Debug for RASDEVINFOA {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("RASDEVINFOA").field("dwSize", &self.dwSize).field("szDeviceType", &self.szDeviceType).field("szDeviceName", &self.szDeviceName).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for RASDEVINFOA {
    fn eq(&self, other: &Self) -> bool {
        self.dwSize == other.dwSize && self.szDeviceType == other.szDeviceType && self.szDeviceName == other.szDeviceName
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for RASDEVINFOA {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for RASDEVINFOA {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
pub struct RASDEVINFOW {
    pub dwSize: u32,
    pub szDeviceType: [u16; 17],
    pub szDeviceName: [u16; 129],
}
impl RASDEVINFOW {}
impl ::std::default::Default for RASDEVINFOW {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for RASDEVINFOW {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("RASDEVINFOW").field("dwSize", &self.dwSize).field("szDeviceType", &self.szDeviceType).field("szDeviceName", &self.szDeviceName).finish()
    }
}
impl ::std::cmp::PartialEq for RASDEVINFOW {
    fn eq(&self, other: &Self) -> bool {
        self.dwSize == other.dwSize && self.szDeviceType == other.szDeviceType && self.szDeviceName == other.szDeviceName
    }
}
impl ::std::cmp::Eq for RASDEVINFOW {}
unsafe impl ::windows::runtime::Abi for RASDEVINFOW {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C, packed(4))]
pub struct RASDEVSPECIFICINFO {
    pub dwSize: u32,
    pub pbDevSpecificInfo: *mut u8,
}
impl RASDEVSPECIFICINFO {}
impl ::std::default::Default for RASDEVSPECIFICINFO {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::cmp::PartialEq for RASDEVSPECIFICINFO {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
impl ::std::cmp::Eq for RASDEVSPECIFICINFO {}
unsafe impl ::windows::runtime::Abi for RASDEVSPECIFICINFO {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C, packed(4))]
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
impl RASDIALDLG {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for RASDIALDLG {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for RASDIALDLG {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for RASDIALDLG {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for RASDIALDLG {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C, packed(4))]
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
impl RASDIALEXTENSIONS {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for RASDIALEXTENSIONS {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for RASDIALEXTENSIONS {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for RASDIALEXTENSIONS {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for RASDIALEXTENSIONS {
    type Abi = Self;
    type DefaultType = Self;
}
pub type RASDIALFUNC = unsafe extern "system" fn(param0: u32, param1: RASCONNSTATE, param2: u32);
pub type RASDIALFUNC1 = unsafe extern "system" fn(param0: HRASCONN, param1: u32, param2: RASCONNSTATE, param3: u32, param4: u32);
pub type RASDIALFUNC2 = unsafe extern "system" fn(param0: usize, param1: u32, param2: HRASCONN, param3: u32, param4: RASCONNSTATE, param5: u32, param6: u32) -> u32;
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C, packed(4))]
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
    pub szEncPassword: super::super::Foundation::PSTR,
}
#[cfg(feature = "Win32_Foundation")]
impl RASDIALPARAMSA {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for RASDIALPARAMSA {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for RASDIALPARAMSA {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for RASDIALPARAMSA {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for RASDIALPARAMSA {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C, packed(4))]
#[cfg(feature = "Win32_Foundation")]
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
    pub szEncPassword: super::super::Foundation::PWSTR,
}
#[cfg(feature = "Win32_Foundation")]
impl RASDIALPARAMSW {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for RASDIALPARAMSW {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for RASDIALPARAMSW {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for RASDIALPARAMSW {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for RASDIALPARAMSW {
    type Abi = Self;
    type DefaultType = Self;
}
pub const RASEAPF_Logon: u32 = 4u32;
pub const RASEAPF_NonInteractive: u32 = 2u32;
pub const RASEAPF_Preview: u32 = 8u32;
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C, packed(4))]
pub struct RASEAPINFO {
    pub dwSizeofEapInfo: u32,
    pub pbEapInfo: *mut u8,
}
impl RASEAPINFO {}
impl ::std::default::Default for RASEAPINFO {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::cmp::PartialEq for RASEAPINFO {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
impl ::std::cmp::Eq for RASEAPINFO {}
unsafe impl ::windows::runtime::Abi for RASEAPINFO {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct RASEAPUSERIDENTITYA {
    pub szUserName: [super::super::Foundation::CHAR; 257],
    pub dwSizeofEapInfo: u32,
    pub pbEapInfo: [u8; 1],
}
#[cfg(feature = "Win32_Foundation")]
impl RASEAPUSERIDENTITYA {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for RASEAPUSERIDENTITYA {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::fmt::Debug for RASEAPUSERIDENTITYA {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("RASEAPUSERIDENTITYA").field("szUserName", &self.szUserName).field("dwSizeofEapInfo", &self.dwSizeofEapInfo).field("pbEapInfo", &self.pbEapInfo).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for RASEAPUSERIDENTITYA {
    fn eq(&self, other: &Self) -> bool {
        self.szUserName == other.szUserName && self.dwSizeofEapInfo == other.dwSizeofEapInfo && self.pbEapInfo == other.pbEapInfo
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for RASEAPUSERIDENTITYA {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for RASEAPUSERIDENTITYA {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
pub struct RASEAPUSERIDENTITYW {
    pub szUserName: [u16; 257],
    pub dwSizeofEapInfo: u32,
    pub pbEapInfo: [u8; 1],
}
impl RASEAPUSERIDENTITYW {}
impl ::std::default::Default for RASEAPUSERIDENTITYW {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for RASEAPUSERIDENTITYW {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("RASEAPUSERIDENTITYW").field("szUserName", &self.szUserName).field("dwSizeofEapInfo", &self.dwSizeofEapInfo).field("pbEapInfo", &self.pbEapInfo).finish()
    }
}
impl ::std::cmp::PartialEq for RASEAPUSERIDENTITYW {
    fn eq(&self, other: &Self) -> bool {
        self.szUserName == other.szUserName && self.dwSizeofEapInfo == other.dwSizeofEapInfo && self.pbEapInfo == other.pbEapInfo
    }
}
impl ::std::cmp::Eq for RASEAPUSERIDENTITYW {}
unsafe impl ::windows::runtime::Abi for RASEAPUSERIDENTITYW {
    type Abi = Self;
    type DefaultType = Self;
}
pub const RASEDFLAG_CloneEntry: u32 = 4u32;
pub const RASEDFLAG_IncomingConnection: u32 = 1024u32;
pub const RASEDFLAG_InternetEntry: u32 = 256u32;
pub const RASEDFLAG_NAT: u32 = 512u32;
pub const RASEDFLAG_NewBroadbandEntry: u32 = 128u32;
pub const RASEDFLAG_NewDirectEntry: u32 = 64u32;
pub const RASEDFLAG_NewEntry: u32 = 2u32;
pub const RASEDFLAG_NewPhoneEntry: u32 = 16u32;
pub const RASEDFLAG_NewTunnelEntry: u32 = 32u32;
pub const RASEDFLAG_NoRename: u32 = 8u32;
pub const RASEDFLAG_PositionDlg: u32 = 1u32;
pub const RASEDFLAG_ShellOwned: u32 = 1073741824u32;
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
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
    pub guidId: ::windows::runtime::GUID,
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
impl RASENTRYA {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
impl ::std::default::Default for RASENTRYA {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
impl ::std::cmp::PartialEq for RASENTRYA {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
impl ::std::cmp::Eq for RASENTRYA {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
unsafe impl ::windows::runtime::Abi for RASENTRYA {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C, packed(4))]
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
impl RASENTRYDLGA {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for RASENTRYDLGA {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for RASENTRYDLGA {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for RASENTRYDLGA {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for RASENTRYDLGA {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C, packed(4))]
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
impl RASENTRYDLGW {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for RASENTRYDLGW {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for RASENTRYDLGW {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for RASENTRYDLGW {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for RASENTRYDLGW {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct RASENTRYNAMEA {
    pub dwSize: u32,
    pub szEntryName: [super::super::Foundation::CHAR; 257],
    pub dwFlags: u32,
    pub szPhonebookPath: [super::super::Foundation::CHAR; 261],
}
#[cfg(feature = "Win32_Foundation")]
impl RASENTRYNAMEA {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for RASENTRYNAMEA {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::fmt::Debug for RASENTRYNAMEA {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("RASENTRYNAMEA").field("dwSize", &self.dwSize).field("szEntryName", &self.szEntryName).field("dwFlags", &self.dwFlags).field("szPhonebookPath", &self.szPhonebookPath).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for RASENTRYNAMEA {
    fn eq(&self, other: &Self) -> bool {
        self.dwSize == other.dwSize && self.szEntryName == other.szEntryName && self.dwFlags == other.dwFlags && self.szPhonebookPath == other.szPhonebookPath
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for RASENTRYNAMEA {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for RASENTRYNAMEA {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
pub struct RASENTRYNAMEW {
    pub dwSize: u32,
    pub szEntryName: [u16; 257],
    pub dwFlags: u32,
    pub szPhonebookPath: [u16; 261],
}
impl RASENTRYNAMEW {}
impl ::std::default::Default for RASENTRYNAMEW {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for RASENTRYNAMEW {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("RASENTRYNAMEW").field("dwSize", &self.dwSize).field("szEntryName", &self.szEntryName).field("dwFlags", &self.dwFlags).field("szPhonebookPath", &self.szPhonebookPath).finish()
    }
}
impl ::std::cmp::PartialEq for RASENTRYNAMEW {
    fn eq(&self, other: &Self) -> bool {
        self.dwSize == other.dwSize && self.szEntryName == other.szEntryName && self.dwFlags == other.dwFlags && self.szPhonebookPath == other.szPhonebookPath
    }
}
impl ::std::cmp::Eq for RASENTRYNAMEW {}
unsafe impl ::windows::runtime::Abi for RASENTRYNAMEW {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
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
    pub guidId: ::windows::runtime::GUID,
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
impl RASENTRYW {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
impl ::std::default::Default for RASENTRYW {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
impl ::std::cmp::PartialEq for RASENTRYW {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
impl ::std::cmp::Eq for RASENTRYW {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
unsafe impl ::windows::runtime::Abi for RASENTRYW {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct RASENTRY_DIAL_MODE(pub u32);
pub const RASEDM_DialAll: RASENTRY_DIAL_MODE = RASENTRY_DIAL_MODE(1u32);
pub const RASEDM_DialAsNeeded: RASENTRY_DIAL_MODE = RASENTRY_DIAL_MODE(2u32);
impl ::std::convert::From<u32> for RASENTRY_DIAL_MODE {
    fn from(value: u32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for RASENTRY_DIAL_MODE {
    type Abi = Self;
    type DefaultType = Self;
}
impl ::std::ops::BitOr for RASENTRY_DIAL_MODE {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self {
        Self(self.0 | rhs.0)
    }
}
impl ::std::ops::BitAnd for RASENTRY_DIAL_MODE {
    type Output = Self;
    fn bitand(self, rhs: Self) -> Self {
        Self(self.0 & rhs.0)
    }
}
impl ::std::ops::BitOrAssign for RASENTRY_DIAL_MODE {
    fn bitor_assign(&mut self, rhs: Self) {
        self.0.bitor_assign(rhs.0)
    }
}
impl ::std::ops::BitAndAssign for RASENTRY_DIAL_MODE {
    fn bitand_assign(&mut self, rhs: Self) {
        self.0.bitand_assign(rhs.0)
    }
}
impl ::std::ops::Not for RASENTRY_DIAL_MODE {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
pub const RASEO2_AuthTypeIsOtp: u32 = 268435456u32;
pub const RASEO2_AutoTriggerCapable: u32 = 67108864u32;
pub const RASEO2_CacheCredentials: u32 = 33554432u32;
pub const RASEO2_DisableClassBasedStaticRoute: u32 = 524288u32;
pub const RASEO2_DisableIKENameEkuCheck: u32 = 262144u32;
pub const RASEO2_DisableMobility: u32 = 2097152u32;
pub const RASEO2_DisableNbtOverIP: u32 = 64u32;
pub const RASEO2_DontNegotiateMultilink: u32 = 4u32;
pub const RASEO2_DontUseRasCredentials: u32 = 8u32;
pub const RASEO2_IPv4ExplicitMetric: u32 = 65536u32;
pub const RASEO2_IPv6ExplicitMetric: u32 = 131072u32;
pub const RASEO2_IPv6RemoteDefaultGateway: u32 = 8192u32;
pub const RASEO2_IPv6SpecificNameServers: u32 = 4096u32;
pub const RASEO2_Internet: u32 = 32u32;
pub const RASEO2_IsAlwaysOn: u32 = 536870912u32;
pub const RASEO2_IsPrivateNetwork: u32 = 1073741824u32;
pub const RASEO2_IsThirdPartyProfile: u32 = 134217728u32;
pub const RASEO2_PlumbIKEv2TSAsRoutes: u32 = 2147483648u32;
pub const RASEO2_ReconnectIfDropped: u32 = 256u32;
pub const RASEO2_RegisterIpWithDNS: u32 = 16384u32;
pub const RASEO2_RequireMachineCertificates: u32 = 4194304u32;
pub const RASEO2_SecureClientForMSNet: u32 = 2u32;
pub const RASEO2_SecureFileAndPrint: u32 = 1u32;
pub const RASEO2_SecureRoutingCompartment: u32 = 1024u32;
pub const RASEO2_SharePhoneNumbers: u32 = 512u32;
pub const RASEO2_SpecificIPv6Addr: u32 = 1048576u32;
pub const RASEO2_UseDNSSuffixForRegistration: u32 = 32768u32;
pub const RASEO2_UseGlobalDeviceSettings: u32 = 128u32;
pub const RASEO2_UsePreSharedKey: u32 = 16u32;
pub const RASEO2_UsePreSharedKeyForIkev2Initiator: u32 = 8388608u32;
pub const RASEO2_UsePreSharedKeyForIkev2Responder: u32 = 16777216u32;
pub const RASEO2_UseTypicalSettings: u32 = 2048u32;
pub const RASEO_Custom: u32 = 1048576u32;
pub const RASEO_CustomScript: u32 = 2147483648u32;
pub const RASEO_DisableLcpExtensions: u32 = 32u32;
pub const RASEO_IpHeaderCompression: u32 = 8u32;
pub const RASEO_ModemLights: u32 = 256u32;
pub const RASEO_NetworkLogon: u32 = 8192u32;
pub const RASEO_PreviewDomain: u32 = 33554432u32;
pub const RASEO_PreviewPhoneNumber: u32 = 2097152u32;
pub const RASEO_PreviewUserPw: u32 = 16777216u32;
pub const RASEO_PromoteAlternates: u32 = 32768u32;
pub const RASEO_RemoteDefaultGateway: u32 = 16u32;
pub const RASEO_RequireCHAP: u32 = 134217728u32;
pub const RASEO_RequireDataEncryption: u32 = 4096u32;
pub const RASEO_RequireEAP: u32 = 131072u32;
pub const RASEO_RequireEncryptedPw: u32 = 1024u32;
pub const RASEO_RequireMsCHAP: u32 = 268435456u32;
pub const RASEO_RequireMsCHAP2: u32 = 536870912u32;
pub const RASEO_RequireMsEncryptedPw: u32 = 2048u32;
pub const RASEO_RequirePAP: u32 = 262144u32;
pub const RASEO_RequireSPAP: u32 = 524288u32;
pub const RASEO_RequireW95MSCHAP: u32 = 1073741824u32;
pub const RASEO_SecureLocalFiles: u32 = 65536u32;
pub const RASEO_SharedPhoneNumbers: u32 = 8388608u32;
pub const RASEO_ShowDialingProgress: u32 = 67108864u32;
pub const RASEO_SpecificIpAddr: u32 = 2u32;
pub const RASEO_SpecificNameServers: u32 = 4u32;
pub const RASEO_SwCompression: u32 = 512u32;
pub const RASEO_TerminalAfterDial: u32 = 128u32;
pub const RASEO_TerminalBeforeDial: u32 = 64u32;
pub const RASEO_UseCountryAndAreaCodes: u32 = 1u32;
pub const RASEO_UseLogonCredentials: u32 = 16384u32;
pub const RASET_Broadband: u32 = 5u32;
pub const RASET_Direct: u32 = 3u32;
pub const RASET_Internet: u32 = 4u32;
pub const RASET_Phone: u32 = 1u32;
pub const RASET_Vpn: u32 = 2u32;
pub const RASFP_Ppp: u32 = 1u32;
pub const RASFP_Ras: u32 = 4u32;
pub const RASFP_Slip: u32 = 2u32;
pub const RASIDS_Disabled: u32 = 4294967295u32;
pub const RASIDS_UseGlobalValue: u32 = 0u32;
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C, packed(4))]
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
impl RASIKEV2_PROJECTION_INFO {}
#[cfg(feature = "Win32_Networking_WinSock")]
impl ::std::default::Default for RASIKEV2_PROJECTION_INFO {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Networking_WinSock")]
impl ::std::cmp::PartialEq for RASIKEV2_PROJECTION_INFO {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
#[cfg(feature = "Win32_Networking_WinSock")]
impl ::std::cmp::Eq for RASIKEV2_PROJECTION_INFO {}
#[cfg(feature = "Win32_Networking_WinSock")]
unsafe impl ::windows::runtime::Abi for RASIKEV2_PROJECTION_INFO {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct RASIKEV_PROJECTION_INFO_FLAGS(pub u32);
pub const RASIKEv2_FLAGS_MOBIKESUPPORTED: RASIKEV_PROJECTION_INFO_FLAGS = RASIKEV_PROJECTION_INFO_FLAGS(1u32);
pub const RASIKEv2_FLAGS_BEHIND_NAT: RASIKEV_PROJECTION_INFO_FLAGS = RASIKEV_PROJECTION_INFO_FLAGS(2u32);
pub const RASIKEv2_FLAGS_SERVERBEHIND_NAT: RASIKEV_PROJECTION_INFO_FLAGS = RASIKEV_PROJECTION_INFO_FLAGS(4u32);
impl ::std::convert::From<u32> for RASIKEV_PROJECTION_INFO_FLAGS {
    fn from(value: u32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for RASIKEV_PROJECTION_INFO_FLAGS {
    type Abi = Self;
    type DefaultType = Self;
}
impl ::std::ops::BitOr for RASIKEV_PROJECTION_INFO_FLAGS {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self {
        Self(self.0 | rhs.0)
    }
}
impl ::std::ops::BitAnd for RASIKEV_PROJECTION_INFO_FLAGS {
    type Output = Self;
    fn bitand(self, rhs: Self) -> Self {
        Self(self.0 & rhs.0)
    }
}
impl ::std::ops::BitOrAssign for RASIKEV_PROJECTION_INFO_FLAGS {
    fn bitor_assign(&mut self, rhs: Self) {
        self.0.bitor_assign(rhs.0)
    }
}
impl ::std::ops::BitAndAssign for RASIKEV_PROJECTION_INFO_FLAGS {
    fn bitand_assign(&mut self, rhs: Self) {
        self.0.bitand_assign(rhs.0)
    }
}
impl ::std::ops::Not for RASIKEV_PROJECTION_INFO_FLAGS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
pub const RASIKEv2_AUTH_EAP: u32 = 2u32;
pub const RASIKEv2_AUTH_MACHINECERTIFICATES: u32 = 1u32;
pub const RASIKEv2_AUTH_PSK: u32 = 3u32;
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
pub struct RASIPADDR {
    pub a: u8,
    pub b: u8,
    pub c: u8,
    pub d: u8,
}
impl RASIPADDR {}
impl ::std::default::Default for RASIPADDR {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for RASIPADDR {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("RASIPADDR").field("a", &self.a).field("b", &self.b).field("c", &self.c).field("d", &self.d).finish()
    }
}
impl ::std::cmp::PartialEq for RASIPADDR {
    fn eq(&self, other: &Self) -> bool {
        self.a == other.a && self.b == other.b && self.c == other.c && self.d == other.d
    }
}
impl ::std::cmp::Eq for RASIPADDR {}
unsafe impl ::windows::runtime::Abi for RASIPADDR {
    type Abi = Self;
    type DefaultType = Self;
}
pub const RASIPO_VJ: u32 = 1u32;
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
pub struct RASIPXW {
    pub dwSize: u32,
    pub dwError: u32,
    pub szIpxAddress: [u16; 22],
}
impl RASIPXW {}
impl ::std::default::Default for RASIPXW {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for RASIPXW {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("RASIPXW").field("dwSize", &self.dwSize).field("dwError", &self.dwError).field("szIpxAddress", &self.szIpxAddress).finish()
    }
}
impl ::std::cmp::PartialEq for RASIPXW {
    fn eq(&self, other: &Self) -> bool {
        self.dwSize == other.dwSize && self.dwError == other.dwError && self.szIpxAddress == other.szIpxAddress
    }
}
impl ::std::cmp::Eq for RASIPXW {}
unsafe impl ::windows::runtime::Abi for RASIPXW {
    type Abi = Self;
    type DefaultType = Self;
}
pub const RASLCPO_3_DES: u32 = 16u32;
pub const RASLCPO_ACFC: u32 = 2u32;
pub const RASLCPO_AES_128: u32 = 32u32;
pub const RASLCPO_AES_192: u32 = 128u32;
pub const RASLCPO_AES_256: u32 = 64u32;
pub const RASLCPO_DES_56: u32 = 8u32;
pub const RASLCPO_GCM_AES_128: u32 = 256u32;
pub const RASLCPO_GCM_AES_192: u32 = 512u32;
pub const RASLCPO_GCM_AES_256: u32 = 1024u32;
pub const RASLCPO_PFC: u32 = 1u32;
pub const RASLCPO_SSHF: u32 = 4u32;
pub const RASNAP_ProbationTime: u32 = 1u32;
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
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
impl RASNOUSERA {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for RASNOUSERA {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::fmt::Debug for RASNOUSERA {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("RASNOUSERA").field("dwSize", &self.dwSize).field("dwFlags", &self.dwFlags).field("dwTimeoutMs", &self.dwTimeoutMs).field("szUserName", &self.szUserName).field("szPassword", &self.szPassword).field("szDomain", &self.szDomain).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for RASNOUSERA {
    fn eq(&self, other: &Self) -> bool {
        self.dwSize == other.dwSize && self.dwFlags == other.dwFlags && self.dwTimeoutMs == other.dwTimeoutMs && self.szUserName == other.szUserName && self.szPassword == other.szPassword && self.szDomain == other.szDomain
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for RASNOUSERA {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for RASNOUSERA {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
pub struct RASNOUSERW {
    pub dwSize: u32,
    pub dwFlags: u32,
    pub dwTimeoutMs: u32,
    pub szUserName: [u16; 257],
    pub szPassword: [u16; 257],
    pub szDomain: [u16; 16],
}
impl RASNOUSERW {}
impl ::std::default::Default for RASNOUSERW {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for RASNOUSERW {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("RASNOUSERW").field("dwSize", &self.dwSize).field("dwFlags", &self.dwFlags).field("dwTimeoutMs", &self.dwTimeoutMs).field("szUserName", &self.szUserName).field("szPassword", &self.szPassword).field("szDomain", &self.szDomain).finish()
    }
}
impl ::std::cmp::PartialEq for RASNOUSERW {
    fn eq(&self, other: &Self) -> bool {
        self.dwSize == other.dwSize && self.dwFlags == other.dwFlags && self.dwTimeoutMs == other.dwTimeoutMs && self.szUserName == other.szUserName && self.szPassword == other.szPassword && self.szDomain == other.szDomain
    }
}
impl ::std::cmp::Eq for RASNOUSERW {}
unsafe impl ::windows::runtime::Abi for RASNOUSERW {
    type Abi = Self;
    type DefaultType = Self;
}
pub const RASNOUSER_SmartCard: u32 = 1u32;
pub const RASNP_Ip: u32 = 4u32;
pub const RASNP_Ipv6: u32 = 8u32;
pub const RASNP_Ipx: u32 = 2u32;
pub const RASNP_NetBEUI: u32 = 1u32;
pub const RASPBDEVENT_AddEntry: u32 = 1u32;
pub const RASPBDEVENT_DialEntry: u32 = 4u32;
pub const RASPBDEVENT_EditEntry: u32 = 2u32;
pub const RASPBDEVENT_EditGlobals: u32 = 5u32;
pub const RASPBDEVENT_NoUser: u32 = 6u32;
pub const RASPBDEVENT_NoUserEdit: u32 = 7u32;
pub const RASPBDEVENT_RemoveEntry: u32 = 3u32;
pub const RASPBDFLAG_ForceCloseOnDial: u32 = 2u32;
pub const RASPBDFLAG_NoUser: u32 = 16u32;
pub const RASPBDFLAG_PositionDlg: u32 = 1u32;
pub const RASPBDFLAG_UpdateDefaults: u32 = 2147483648u32;
#[cfg(feature = "Win32_Foundation")]
impl ::std::clone::Clone for RASPBDLGA {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
#[repr(C, packed(4))]
#[cfg(feature = "Win32_Foundation")]
pub struct RASPBDLGA {
    pub dwSize: u32,
    pub hwndOwner: super::super::Foundation::HWND,
    pub dwFlags: u32,
    pub xDlg: i32,
    pub yDlg: i32,
    pub dwCallbackId: usize,
    pub pCallback: ::std::option::Option<RASPBDLGFUNCA>,
    pub dwError: u32,
    pub reserved: usize,
    pub reserved2: usize,
}
#[cfg(feature = "Win32_Foundation")]
impl RASPBDLGA {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for RASPBDLGA {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for RASPBDLGA {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for RASPBDLGA {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for RASPBDLGA {
    type Abi = ::std::mem::ManuallyDrop<Self>;
    type DefaultType = Self;
}
#[cfg(feature = "Win32_Foundation")]
pub type RASPBDLGFUNCA = unsafe extern "system" fn(param0: usize, param1: u32, param2: super::super::Foundation::PSTR, param3: *mut ::std::ffi::c_void);
#[cfg(feature = "Win32_Foundation")]
pub type RASPBDLGFUNCW = unsafe extern "system" fn(param0: usize, param1: u32, param2: super::super::Foundation::PWSTR, param3: *mut ::std::ffi::c_void);
#[cfg(feature = "Win32_Foundation")]
impl ::std::clone::Clone for RASPBDLGW {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
#[repr(C, packed(4))]
#[cfg(feature = "Win32_Foundation")]
pub struct RASPBDLGW {
    pub dwSize: u32,
    pub hwndOwner: super::super::Foundation::HWND,
    pub dwFlags: u32,
    pub xDlg: i32,
    pub yDlg: i32,
    pub dwCallbackId: usize,
    pub pCallback: ::std::option::Option<RASPBDLGFUNCW>,
    pub dwError: u32,
    pub reserved: usize,
    pub reserved2: usize,
}
#[cfg(feature = "Win32_Foundation")]
impl RASPBDLGW {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for RASPBDLGW {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for RASPBDLGW {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for RASPBDLGW {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for RASPBDLGW {
    type Abi = ::std::mem::ManuallyDrop<Self>;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
pub struct RASPPPCCP {
    pub dwSize: u32,
    pub dwError: u32,
    pub dwCompressionAlgorithm: u32,
    pub dwOptions: u32,
    pub dwServerCompressionAlgorithm: u32,
    pub dwServerOptions: u32,
}
impl RASPPPCCP {}
impl ::std::default::Default for RASPPPCCP {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for RASPPPCCP {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("RASPPPCCP")
            .field("dwSize", &self.dwSize)
            .field("dwError", &self.dwError)
            .field("dwCompressionAlgorithm", &self.dwCompressionAlgorithm)
            .field("dwOptions", &self.dwOptions)
            .field("dwServerCompressionAlgorithm", &self.dwServerCompressionAlgorithm)
            .field("dwServerOptions", &self.dwServerOptions)
            .finish()
    }
}
impl ::std::cmp::PartialEq for RASPPPCCP {
    fn eq(&self, other: &Self) -> bool {
        self.dwSize == other.dwSize && self.dwError == other.dwError && self.dwCompressionAlgorithm == other.dwCompressionAlgorithm && self.dwOptions == other.dwOptions && self.dwServerCompressionAlgorithm == other.dwServerCompressionAlgorithm && self.dwServerOptions == other.dwServerOptions
    }
}
impl ::std::cmp::Eq for RASPPPCCP {}
unsafe impl ::windows::runtime::Abi for RASPPPCCP {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
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
impl RASPPPIPA {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for RASPPPIPA {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::fmt::Debug for RASPPPIPA {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("RASPPPIPA").field("dwSize", &self.dwSize).field("dwError", &self.dwError).field("szIpAddress", &self.szIpAddress).field("szServerIpAddress", &self.szServerIpAddress).field("dwOptions", &self.dwOptions).field("dwServerOptions", &self.dwServerOptions).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for RASPPPIPA {
    fn eq(&self, other: &Self) -> bool {
        self.dwSize == other.dwSize && self.dwError == other.dwError && self.szIpAddress == other.szIpAddress && self.szServerIpAddress == other.szServerIpAddress && self.dwOptions == other.dwOptions && self.dwServerOptions == other.dwServerOptions
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for RASPPPIPA {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for RASPPPIPA {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
pub struct RASPPPIPV6 {
    pub dwSize: u32,
    pub dwError: u32,
    pub bLocalInterfaceIdentifier: [u8; 8],
    pub bPeerInterfaceIdentifier: [u8; 8],
    pub bLocalCompressionProtocol: [u8; 2],
    pub bPeerCompressionProtocol: [u8; 2],
}
impl RASPPPIPV6 {}
impl ::std::default::Default for RASPPPIPV6 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for RASPPPIPV6 {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("RASPPPIPV6")
            .field("dwSize", &self.dwSize)
            .field("dwError", &self.dwError)
            .field("bLocalInterfaceIdentifier", &self.bLocalInterfaceIdentifier)
            .field("bPeerInterfaceIdentifier", &self.bPeerInterfaceIdentifier)
            .field("bLocalCompressionProtocol", &self.bLocalCompressionProtocol)
            .field("bPeerCompressionProtocol", &self.bPeerCompressionProtocol)
            .finish()
    }
}
impl ::std::cmp::PartialEq for RASPPPIPV6 {
    fn eq(&self, other: &Self) -> bool {
        self.dwSize == other.dwSize && self.dwError == other.dwError && self.bLocalInterfaceIdentifier == other.bLocalInterfaceIdentifier && self.bPeerInterfaceIdentifier == other.bPeerInterfaceIdentifier && self.bLocalCompressionProtocol == other.bLocalCompressionProtocol && self.bPeerCompressionProtocol == other.bPeerCompressionProtocol
    }
}
impl ::std::cmp::Eq for RASPPPIPV6 {}
unsafe impl ::windows::runtime::Abi for RASPPPIPV6 {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
pub struct RASPPPIPW {
    pub dwSize: u32,
    pub dwError: u32,
    pub szIpAddress: [u16; 16],
    pub szServerIpAddress: [u16; 16],
    pub dwOptions: u32,
    pub dwServerOptions: u32,
}
impl RASPPPIPW {}
impl ::std::default::Default for RASPPPIPW {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for RASPPPIPW {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("RASPPPIPW").field("dwSize", &self.dwSize).field("dwError", &self.dwError).field("szIpAddress", &self.szIpAddress).field("szServerIpAddress", &self.szServerIpAddress).field("dwOptions", &self.dwOptions).field("dwServerOptions", &self.dwServerOptions).finish()
    }
}
impl ::std::cmp::PartialEq for RASPPPIPW {
    fn eq(&self, other: &Self) -> bool {
        self.dwSize == other.dwSize && self.dwError == other.dwError && self.szIpAddress == other.szIpAddress && self.szServerIpAddress == other.szServerIpAddress && self.dwOptions == other.dwOptions && self.dwServerOptions == other.dwServerOptions
    }
}
impl ::std::cmp::Eq for RASPPPIPW {}
unsafe impl ::windows::runtime::Abi for RASPPPIPW {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct RASPPPIPXA {
    pub dwSize: u32,
    pub dwError: u32,
    pub szIpxAddress: [super::super::Foundation::CHAR; 22],
}
#[cfg(feature = "Win32_Foundation")]
impl RASPPPIPXA {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for RASPPPIPXA {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::fmt::Debug for RASPPPIPXA {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("RASPPPIPXA").field("dwSize", &self.dwSize).field("dwError", &self.dwError).field("szIpxAddress", &self.szIpxAddress).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for RASPPPIPXA {
    fn eq(&self, other: &Self) -> bool {
        self.dwSize == other.dwSize && self.dwError == other.dwError && self.szIpxAddress == other.szIpxAddress
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for RASPPPIPXA {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for RASPPPIPXA {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
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
impl RASPPPLCPA {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for RASPPPLCPA {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::fmt::Debug for RASPPPLCPA {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("RASPPPLCPA")
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
impl ::std::cmp::PartialEq for RASPPPLCPA {
    fn eq(&self, other: &Self) -> bool {
        self.dwSize == other.dwSize
            && self.fBundled == other.fBundled
            && self.dwError == other.dwError
            && self.dwAuthenticationProtocol == other.dwAuthenticationProtocol
            && self.dwAuthenticationData == other.dwAuthenticationData
            && self.dwEapTypeId == other.dwEapTypeId
            && self.dwServerAuthenticationProtocol == other.dwServerAuthenticationProtocol
            && self.dwServerAuthenticationData == other.dwServerAuthenticationData
            && self.dwServerEapTypeId == other.dwServerEapTypeId
            && self.fMultilink == other.fMultilink
            && self.dwTerminateReason == other.dwTerminateReason
            && self.dwServerTerminateReason == other.dwServerTerminateReason
            && self.szReplyMessage == other.szReplyMessage
            && self.dwOptions == other.dwOptions
            && self.dwServerOptions == other.dwServerOptions
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for RASPPPLCPA {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for RASPPPLCPA {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
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
impl RASPPPLCPW {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for RASPPPLCPW {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::fmt::Debug for RASPPPLCPW {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("RASPPPLCPW")
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
impl ::std::cmp::PartialEq for RASPPPLCPW {
    fn eq(&self, other: &Self) -> bool {
        self.dwSize == other.dwSize
            && self.fBundled == other.fBundled
            && self.dwError == other.dwError
            && self.dwAuthenticationProtocol == other.dwAuthenticationProtocol
            && self.dwAuthenticationData == other.dwAuthenticationData
            && self.dwEapTypeId == other.dwEapTypeId
            && self.dwServerAuthenticationProtocol == other.dwServerAuthenticationProtocol
            && self.dwServerAuthenticationData == other.dwServerAuthenticationData
            && self.dwServerEapTypeId == other.dwServerEapTypeId
            && self.fMultilink == other.fMultilink
            && self.dwTerminateReason == other.dwTerminateReason
            && self.dwServerTerminateReason == other.dwServerTerminateReason
            && self.szReplyMessage == other.szReplyMessage
            && self.dwOptions == other.dwOptions
            && self.dwServerOptions == other.dwServerOptions
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for RASPPPLCPW {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for RASPPPLCPW {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
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
impl RASPPPNBFA {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for RASPPPNBFA {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::fmt::Debug for RASPPPNBFA {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("RASPPPNBFA").field("dwSize", &self.dwSize).field("dwError", &self.dwError).field("dwNetBiosError", &self.dwNetBiosError).field("szNetBiosError", &self.szNetBiosError).field("szWorkstationName", &self.szWorkstationName).field("bLana", &self.bLana).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for RASPPPNBFA {
    fn eq(&self, other: &Self) -> bool {
        self.dwSize == other.dwSize && self.dwError == other.dwError && self.dwNetBiosError == other.dwNetBiosError && self.szNetBiosError == other.szNetBiosError && self.szWorkstationName == other.szWorkstationName && self.bLana == other.bLana
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for RASPPPNBFA {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for RASPPPNBFA {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
pub struct RASPPPNBFW {
    pub dwSize: u32,
    pub dwError: u32,
    pub dwNetBiosError: u32,
    pub szNetBiosError: [u16; 17],
    pub szWorkstationName: [u16; 17],
    pub bLana: u8,
}
impl RASPPPNBFW {}
impl ::std::default::Default for RASPPPNBFW {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for RASPPPNBFW {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("RASPPPNBFW").field("dwSize", &self.dwSize).field("dwError", &self.dwError).field("dwNetBiosError", &self.dwNetBiosError).field("szNetBiosError", &self.szNetBiosError).field("szWorkstationName", &self.szWorkstationName).field("bLana", &self.bLana).finish()
    }
}
impl ::std::cmp::PartialEq for RASPPPNBFW {
    fn eq(&self, other: &Self) -> bool {
        self.dwSize == other.dwSize && self.dwError == other.dwError && self.dwNetBiosError == other.dwNetBiosError && self.szNetBiosError == other.szNetBiosError && self.szWorkstationName == other.szWorkstationName && self.bLana == other.bLana
    }
}
impl ::std::cmp::Eq for RASPPPNBFW {}
unsafe impl ::windows::runtime::Abi for RASPPPNBFW {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
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
impl RASPPP_PROJECTION_INFO {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
impl ::std::default::Default for RASPPP_PROJECTION_INFO {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
impl ::std::cmp::PartialEq for RASPPP_PROJECTION_INFO {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
impl ::std::cmp::Eq for RASPPP_PROJECTION_INFO {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
unsafe impl ::windows::runtime::Abi for RASPPP_PROJECTION_INFO {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct RASPPP_PROJECTION_INFO_SERVER_AUTH_DATA(pub u32);
pub const RASLCPAD_CHAP_MD5: RASPPP_PROJECTION_INFO_SERVER_AUTH_DATA = RASPPP_PROJECTION_INFO_SERVER_AUTH_DATA(5u32);
pub const RASLCPAD_CHAP_MS: RASPPP_PROJECTION_INFO_SERVER_AUTH_DATA = RASPPP_PROJECTION_INFO_SERVER_AUTH_DATA(128u32);
pub const RASLCPAD_CHAP_MSV2: RASPPP_PROJECTION_INFO_SERVER_AUTH_DATA = RASPPP_PROJECTION_INFO_SERVER_AUTH_DATA(129u32);
impl ::std::convert::From<u32> for RASPPP_PROJECTION_INFO_SERVER_AUTH_DATA {
    fn from(value: u32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for RASPPP_PROJECTION_INFO_SERVER_AUTH_DATA {
    type Abi = Self;
    type DefaultType = Self;
}
impl ::std::ops::BitOr for RASPPP_PROJECTION_INFO_SERVER_AUTH_DATA {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self {
        Self(self.0 | rhs.0)
    }
}
impl ::std::ops::BitAnd for RASPPP_PROJECTION_INFO_SERVER_AUTH_DATA {
    type Output = Self;
    fn bitand(self, rhs: Self) -> Self {
        Self(self.0 & rhs.0)
    }
}
impl ::std::ops::BitOrAssign for RASPPP_PROJECTION_INFO_SERVER_AUTH_DATA {
    fn bitor_assign(&mut self, rhs: Self) {
        self.0.bitor_assign(rhs.0)
    }
}
impl ::std::ops::BitAndAssign for RASPPP_PROJECTION_INFO_SERVER_AUTH_DATA {
    fn bitand_assign(&mut self, rhs: Self) {
        self.0.bitand_assign(rhs.0)
    }
}
impl ::std::ops::Not for RASPPP_PROJECTION_INFO_SERVER_AUTH_DATA {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct RASPPP_PROJECTION_INFO_SERVER_AUTH_PROTOCOL(pub u32);
pub const RASLCPAP_PAP: RASPPP_PROJECTION_INFO_SERVER_AUTH_PROTOCOL = RASPPP_PROJECTION_INFO_SERVER_AUTH_PROTOCOL(49187u32);
pub const RASLCPAP_SPAP: RASPPP_PROJECTION_INFO_SERVER_AUTH_PROTOCOL = RASPPP_PROJECTION_INFO_SERVER_AUTH_PROTOCOL(49191u32);
pub const RASLCPAP_CHAP: RASPPP_PROJECTION_INFO_SERVER_AUTH_PROTOCOL = RASPPP_PROJECTION_INFO_SERVER_AUTH_PROTOCOL(49699u32);
pub const RASLCPAP_EAP: RASPPP_PROJECTION_INFO_SERVER_AUTH_PROTOCOL = RASPPP_PROJECTION_INFO_SERVER_AUTH_PROTOCOL(49703u32);
impl ::std::convert::From<u32> for RASPPP_PROJECTION_INFO_SERVER_AUTH_PROTOCOL {
    fn from(value: u32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for RASPPP_PROJECTION_INFO_SERVER_AUTH_PROTOCOL {
    type Abi = Self;
    type DefaultType = Self;
}
impl ::std::ops::BitOr for RASPPP_PROJECTION_INFO_SERVER_AUTH_PROTOCOL {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self {
        Self(self.0 | rhs.0)
    }
}
impl ::std::ops::BitAnd for RASPPP_PROJECTION_INFO_SERVER_AUTH_PROTOCOL {
    type Output = Self;
    fn bitand(self, rhs: Self) -> Self {
        Self(self.0 & rhs.0)
    }
}
impl ::std::ops::BitOrAssign for RASPPP_PROJECTION_INFO_SERVER_AUTH_PROTOCOL {
    fn bitor_assign(&mut self, rhs: Self) {
        self.0.bitor_assign(rhs.0)
    }
}
impl ::std::ops::BitAndAssign for RASPPP_PROJECTION_INFO_SERVER_AUTH_PROTOCOL {
    fn bitand_assign(&mut self, rhs: Self) {
        self.0.bitand_assign(rhs.0)
    }
}
impl ::std::ops::Not for RASPPP_PROJECTION_INFO_SERVER_AUTH_PROTOCOL {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
pub const RASPRIV2_DialinPolicy: u32 = 1u32;
pub const RASPRIV_AdminSetCallback: u32 = 2u32;
pub const RASPRIV_CallerSetCallback: u32 = 4u32;
pub const RASPRIV_DialinPrivilege: u32 = 8u32;
pub const RASPRIV_NoCallback: u32 = 1u32;
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct RASPROJECTION(pub i32);
pub const RASP_Amb: RASPROJECTION = RASPROJECTION(65536i32);
pub const RASP_PppNbf: RASPROJECTION = RASPROJECTION(32831i32);
pub const RASP_PppIpx: RASPROJECTION = RASPROJECTION(32811i32);
pub const RASP_PppIp: RASPROJECTION = RASPROJECTION(32801i32);
pub const RASP_PppCcp: RASPROJECTION = RASPROJECTION(33021i32);
pub const RASP_PppLcp: RASPROJECTION = RASPROJECTION(49185i32);
pub const RASP_PppIpv6: RASPROJECTION = RASPROJECTION(32855i32);
impl ::std::convert::From<i32> for RASPROJECTION {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for RASPROJECTION {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct RASPROJECTION_INFO_TYPE(pub i32);
pub const PROJECTION_INFO_TYPE_PPP: RASPROJECTION_INFO_TYPE = RASPROJECTION_INFO_TYPE(1i32);
pub const PROJECTION_INFO_TYPE_IKEv2: RASPROJECTION_INFO_TYPE = RASPROJECTION_INFO_TYPE(2i32);
impl ::std::convert::From<i32> for RASPROJECTION_INFO_TYPE {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for RASPROJECTION_INFO_TYPE {
    type Abi = Self;
    type DefaultType = Self;
}
pub type RASSECURITYPROC = unsafe extern "system" fn() -> u32;
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
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
impl RASSUBENTRYA {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for RASSUBENTRYA {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::fmt::Debug for RASSUBENTRYA {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("RASSUBENTRYA").field("dwSize", &self.dwSize).field("dwfFlags", &self.dwfFlags).field("szDeviceType", &self.szDeviceType).field("szDeviceName", &self.szDeviceName).field("szLocalPhoneNumber", &self.szLocalPhoneNumber).field("dwAlternateOffset", &self.dwAlternateOffset).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for RASSUBENTRYA {
    fn eq(&self, other: &Self) -> bool {
        self.dwSize == other.dwSize && self.dwfFlags == other.dwfFlags && self.szDeviceType == other.szDeviceType && self.szDeviceName == other.szDeviceName && self.szLocalPhoneNumber == other.szLocalPhoneNumber && self.dwAlternateOffset == other.dwAlternateOffset
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for RASSUBENTRYA {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for RASSUBENTRYA {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
pub struct RASSUBENTRYW {
    pub dwSize: u32,
    pub dwfFlags: u32,
    pub szDeviceType: [u16; 17],
    pub szDeviceName: [u16; 129],
    pub szLocalPhoneNumber: [u16; 129],
    pub dwAlternateOffset: u32,
}
impl RASSUBENTRYW {}
impl ::std::default::Default for RASSUBENTRYW {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for RASSUBENTRYW {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("RASSUBENTRYW").field("dwSize", &self.dwSize).field("dwfFlags", &self.dwfFlags).field("szDeviceType", &self.szDeviceType).field("szDeviceName", &self.szDeviceName).field("szLocalPhoneNumber", &self.szLocalPhoneNumber).field("dwAlternateOffset", &self.dwAlternateOffset).finish()
    }
}
impl ::std::cmp::PartialEq for RASSUBENTRYW {
    fn eq(&self, other: &Self) -> bool {
        self.dwSize == other.dwSize && self.dwfFlags == other.dwfFlags && self.szDeviceType == other.szDeviceType && self.szDeviceName == other.szDeviceName && self.szLocalPhoneNumber == other.szLocalPhoneNumber && self.dwAlternateOffset == other.dwAlternateOffset
    }
}
impl ::std::cmp::Eq for RASSUBENTRYW {}
unsafe impl ::windows::runtime::Abi for RASSUBENTRYW {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Networking_WinSock")]
pub struct RASTUNNELENDPOINT {
    pub dwType: u32,
    pub Anonymous: RASTUNNELENDPOINT_0,
}
#[cfg(feature = "Win32_Networking_WinSock")]
impl RASTUNNELENDPOINT {}
#[cfg(feature = "Win32_Networking_WinSock")]
impl ::std::default::Default for RASTUNNELENDPOINT {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Networking_WinSock")]
impl ::std::cmp::PartialEq for RASTUNNELENDPOINT {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
#[cfg(feature = "Win32_Networking_WinSock")]
impl ::std::cmp::Eq for RASTUNNELENDPOINT {}
#[cfg(feature = "Win32_Networking_WinSock")]
unsafe impl ::windows::runtime::Abi for RASTUNNELENDPOINT {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Networking_WinSock")]
pub union RASTUNNELENDPOINT_0 {
    pub ipv4: super::super::Networking::WinSock::IN_ADDR,
    pub ipv6: super::super::Networking::WinSock::IN6_ADDR,
}
#[cfg(feature = "Win32_Networking_WinSock")]
impl RASTUNNELENDPOINT_0 {}
#[cfg(feature = "Win32_Networking_WinSock")]
impl ::std::default::Default for RASTUNNELENDPOINT_0 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Networking_WinSock")]
impl ::std::cmp::PartialEq for RASTUNNELENDPOINT_0 {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
#[cfg(feature = "Win32_Networking_WinSock")]
impl ::std::cmp::Eq for RASTUNNELENDPOINT_0 {}
#[cfg(feature = "Win32_Networking_WinSock")]
unsafe impl ::windows::runtime::Abi for RASTUNNELENDPOINT_0 {
    type Abi = Self;
    type DefaultType = Self;
}
pub const RASTUNNELENDPOINT_IPv4: u32 = 1u32;
pub const RASTUNNELENDPOINT_IPv6: u32 = 2u32;
pub const RASTUNNELENDPOINT_UNKNOWN: u32 = 0u32;
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
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
impl RASUPDATECONN {}
#[cfg(feature = "Win32_Networking_WinSock")]
impl ::std::default::Default for RASUPDATECONN {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Networking_WinSock")]
impl ::std::cmp::PartialEq for RASUPDATECONN {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
#[cfg(feature = "Win32_Networking_WinSock")]
impl ::std::cmp::Eq for RASUPDATECONN {}
#[cfg(feature = "Win32_Networking_WinSock")]
unsafe impl ::windows::runtime::Abi for RASUPDATECONN {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
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
impl RAS_CONNECTION_0 {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for RAS_CONNECTION_0 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::fmt::Debug for RAS_CONNECTION_0 {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("RAS_CONNECTION_0")
            .field("hConnection", &self.hConnection)
            .field("hInterface", &self.hInterface)
            .field("dwConnectDuration", &self.dwConnectDuration)
            .field("dwInterfaceType", &self.dwInterfaceType)
            .field("dwConnectionFlags", &self.dwConnectionFlags)
            .field("wszInterfaceName", &self.wszInterfaceName)
            .field("wszUserName", &self.wszUserName)
            .field("wszLogonDomain", &self.wszLogonDomain)
            .field("wszRemoteComputer", &self.wszRemoteComputer)
            .finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for RAS_CONNECTION_0 {
    fn eq(&self, other: &Self) -> bool {
        self.hConnection == other.hConnection && self.hInterface == other.hInterface && self.dwConnectDuration == other.dwConnectDuration && self.dwInterfaceType == other.dwInterfaceType && self.dwConnectionFlags == other.dwConnectionFlags && self.wszInterfaceName == other.wszInterfaceName && self.wszUserName == other.wszUserName && self.wszLogonDomain == other.wszLogonDomain && self.wszRemoteComputer == other.wszRemoteComputer
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for RAS_CONNECTION_0 {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for RAS_CONNECTION_0 {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
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
impl RAS_CONNECTION_1 {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for RAS_CONNECTION_1 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::fmt::Debug for RAS_CONNECTION_1 {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("RAS_CONNECTION_1")
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
impl ::std::cmp::PartialEq for RAS_CONNECTION_1 {
    fn eq(&self, other: &Self) -> bool {
        self.hConnection == other.hConnection
            && self.hInterface == other.hInterface
            && self.PppInfo == other.PppInfo
            && self.dwBytesXmited == other.dwBytesXmited
            && self.dwBytesRcved == other.dwBytesRcved
            && self.dwFramesXmited == other.dwFramesXmited
            && self.dwFramesRcved == other.dwFramesRcved
            && self.dwCrcErr == other.dwCrcErr
            && self.dwTimeoutErr == other.dwTimeoutErr
            && self.dwAlignmentErr == other.dwAlignmentErr
            && self.dwHardwareOverrunErr == other.dwHardwareOverrunErr
            && self.dwFramingErr == other.dwFramingErr
            && self.dwBufferOverrunErr == other.dwBufferOverrunErr
            && self.dwCompressionRatioIn == other.dwCompressionRatioIn
            && self.dwCompressionRatioOut == other.dwCompressionRatioOut
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for RAS_CONNECTION_1 {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for RAS_CONNECTION_1 {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct RAS_CONNECTION_2 {
    pub hConnection: super::super::Foundation::HANDLE,
    pub wszUserName: [u16; 257],
    pub dwInterfaceType: ROUTER_INTERFACE_TYPE,
    pub guid: ::windows::runtime::GUID,
    pub PppInfo2: PPP_INFO_2,
}
#[cfg(feature = "Win32_Foundation")]
impl RAS_CONNECTION_2 {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for RAS_CONNECTION_2 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::fmt::Debug for RAS_CONNECTION_2 {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("RAS_CONNECTION_2").field("hConnection", &self.hConnection).field("wszUserName", &self.wszUserName).field("dwInterfaceType", &self.dwInterfaceType).field("guid", &self.guid).field("PppInfo2", &self.PppInfo2).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for RAS_CONNECTION_2 {
    fn eq(&self, other: &Self) -> bool {
        self.hConnection == other.hConnection && self.wszUserName == other.wszUserName && self.dwInterfaceType == other.dwInterfaceType && self.guid == other.guid && self.PppInfo2 == other.PppInfo2
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for RAS_CONNECTION_2 {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for RAS_CONNECTION_2 {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct RAS_CONNECTION_3 {
    pub dwVersion: u32,
    pub dwSize: u32,
    pub hConnection: super::super::Foundation::HANDLE,
    pub wszUserName: [u16; 257],
    pub dwInterfaceType: ROUTER_INTERFACE_TYPE,
    pub guid: ::windows::runtime::GUID,
    pub PppInfo3: PPP_INFO_3,
    pub rasQuarState: RAS_QUARANTINE_STATE,
    pub timer: super::super::Foundation::FILETIME,
}
#[cfg(feature = "Win32_Foundation")]
impl RAS_CONNECTION_3 {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for RAS_CONNECTION_3 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::fmt::Debug for RAS_CONNECTION_3 {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("RAS_CONNECTION_3")
            .field("dwVersion", &self.dwVersion)
            .field("dwSize", &self.dwSize)
            .field("hConnection", &self.hConnection)
            .field("wszUserName", &self.wszUserName)
            .field("dwInterfaceType", &self.dwInterfaceType)
            .field("guid", &self.guid)
            .field("PppInfo3", &self.PppInfo3)
            .field("rasQuarState", &self.rasQuarState)
            .field("timer", &self.timer)
            .finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for RAS_CONNECTION_3 {
    fn eq(&self, other: &Self) -> bool {
        self.dwVersion == other.dwVersion && self.dwSize == other.dwSize && self.hConnection == other.hConnection && self.wszUserName == other.wszUserName && self.dwInterfaceType == other.dwInterfaceType && self.guid == other.guid && self.PppInfo3 == other.PppInfo3 && self.rasQuarState == other.rasQuarState && self.timer == other.timer
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for RAS_CONNECTION_3 {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for RAS_CONNECTION_3 {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct RAS_CONNECTION_4 {
    pub dwConnectDuration: u32,
    pub dwInterfaceType: ROUTER_INTERFACE_TYPE,
    pub dwConnectionFlags: RAS_FLAGS,
    pub wszInterfaceName: [u16; 257],
    pub wszUserName: [u16; 257],
    pub wszLogonDomain: [u16; 16],
    pub wszRemoteComputer: [u16; 17],
    pub guid: ::windows::runtime::GUID,
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
impl RAS_CONNECTION_4 {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for RAS_CONNECTION_4 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for RAS_CONNECTION_4 {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for RAS_CONNECTION_4 {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for RAS_CONNECTION_4 {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
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
    pub guid: ::windows::runtime::GUID,
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
impl RAS_CONNECTION_EX {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for RAS_CONNECTION_EX {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for RAS_CONNECTION_EX {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for RAS_CONNECTION_EX {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for RAS_CONNECTION_EX {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct RAS_FLAGS(pub u32);
pub const RAS_FLAGS_PPP_CONNECTION: RAS_FLAGS = RAS_FLAGS(1u32);
pub const RAS_FLAGS_MESSENGER_PRESENT: RAS_FLAGS = RAS_FLAGS(2u32);
pub const RAS_FLAGS_QUARANTINE_PRESENT: RAS_FLAGS = RAS_FLAGS(8u32);
pub const RAS_FLAGS_ARAP_CONNECTION: RAS_FLAGS = RAS_FLAGS(16u32);
pub const RAS_FLAGS_IKEV2_CONNECTION: RAS_FLAGS = RAS_FLAGS(16u32);
pub const RAS_FLAGS_DORMANT: RAS_FLAGS = RAS_FLAGS(32u32);
impl ::std::convert::From<u32> for RAS_FLAGS {
    fn from(value: u32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for RAS_FLAGS {
    type Abi = Self;
    type DefaultType = Self;
}
impl ::std::ops::BitOr for RAS_FLAGS {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self {
        Self(self.0 | rhs.0)
    }
}
impl ::std::ops::BitAnd for RAS_FLAGS {
    type Output = Self;
    fn bitand(self, rhs: Self) -> Self {
        Self(self.0 & rhs.0)
    }
}
impl ::std::ops::BitOrAssign for RAS_FLAGS {
    fn bitor_assign(&mut self, rhs: Self) {
        self.0.bitor_assign(rhs.0)
    }
}
impl ::std::ops::BitAndAssign for RAS_FLAGS {
    fn bitand_assign(&mut self, rhs: Self) {
        self.0.bitand_assign(rhs.0)
    }
}
impl ::std::ops::Not for RAS_FLAGS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
pub const RAS_FLAGS_RAS_CONNECTION: u32 = 4u32;
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct RAS_HARDWARE_CONDITION(pub i32);
pub const RAS_HARDWARE_OPERATIONAL: RAS_HARDWARE_CONDITION = RAS_HARDWARE_CONDITION(0i32);
pub const RAS_HARDWARE_FAILURE: RAS_HARDWARE_CONDITION = RAS_HARDWARE_CONDITION(1i32);
impl ::std::convert::From<i32> for RAS_HARDWARE_CONDITION {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for RAS_HARDWARE_CONDITION {
    type Abi = Self;
    type DefaultType = Self;
}
pub const RAS_MaxAreaCode: u32 = 10u32;
pub const RAS_MaxCallbackNumber: u32 = 128u32;
pub const RAS_MaxDeviceName: u32 = 128u32;
pub const RAS_MaxDeviceType: u32 = 16u32;
pub const RAS_MaxDnsSuffix: u32 = 256u32;
pub const RAS_MaxEntryName: u32 = 256u32;
pub const RAS_MaxFacilities: u32 = 200u32;
pub const RAS_MaxIDSize: u32 = 256u32;
pub const RAS_MaxIpAddress: u32 = 15u32;
pub const RAS_MaxIpxAddress: u32 = 21u32;
pub const RAS_MaxPadType: u32 = 32u32;
pub const RAS_MaxPhoneNumber: u32 = 128u32;
pub const RAS_MaxReplyMessage: u32 = 1024u32;
pub const RAS_MaxUserData: u32 = 200u32;
pub const RAS_MaxX25Address: u32 = 200u32;
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
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
impl RAS_PORT_0 {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for RAS_PORT_0 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::fmt::Debug for RAS_PORT_0 {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("RAS_PORT_0")
            .field("hPort", &self.hPort)
            .field("hConnection", &self.hConnection)
            .field("dwPortCondition", &self.dwPortCondition)
            .field("dwTotalNumberOfCalls", &self.dwTotalNumberOfCalls)
            .field("dwConnectDuration", &self.dwConnectDuration)
            .field("wszPortName", &self.wszPortName)
            .field("wszMediaName", &self.wszMediaName)
            .field("wszDeviceName", &self.wszDeviceName)
            .field("wszDeviceType", &self.wszDeviceType)
            .finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for RAS_PORT_0 {
    fn eq(&self, other: &Self) -> bool {
        self.hPort == other.hPort && self.hConnection == other.hConnection && self.dwPortCondition == other.dwPortCondition && self.dwTotalNumberOfCalls == other.dwTotalNumberOfCalls && self.dwConnectDuration == other.dwConnectDuration && self.wszPortName == other.wszPortName && self.wszMediaName == other.wszMediaName && self.wszDeviceName == other.wszDeviceName && self.wszDeviceType == other.wszDeviceType
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for RAS_PORT_0 {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for RAS_PORT_0 {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
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
impl RAS_PORT_1 {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for RAS_PORT_1 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::fmt::Debug for RAS_PORT_1 {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("RAS_PORT_1")
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
impl ::std::cmp::PartialEq for RAS_PORT_1 {
    fn eq(&self, other: &Self) -> bool {
        self.hPort == other.hPort
            && self.hConnection == other.hConnection
            && self.dwHardwareCondition == other.dwHardwareCondition
            && self.dwLineSpeed == other.dwLineSpeed
            && self.dwBytesXmited == other.dwBytesXmited
            && self.dwBytesRcved == other.dwBytesRcved
            && self.dwFramesXmited == other.dwFramesXmited
            && self.dwFramesRcved == other.dwFramesRcved
            && self.dwCrcErr == other.dwCrcErr
            && self.dwTimeoutErr == other.dwTimeoutErr
            && self.dwAlignmentErr == other.dwAlignmentErr
            && self.dwHardwareOverrunErr == other.dwHardwareOverrunErr
            && self.dwFramingErr == other.dwFramingErr
            && self.dwBufferOverrunErr == other.dwBufferOverrunErr
            && self.dwCompressionRatioIn == other.dwCompressionRatioIn
            && self.dwCompressionRatioOut == other.dwCompressionRatioOut
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for RAS_PORT_1 {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for RAS_PORT_1 {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
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
impl RAS_PORT_2 {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for RAS_PORT_2 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::fmt::Debug for RAS_PORT_2 {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("RAS_PORT_2")
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
impl ::std::cmp::PartialEq for RAS_PORT_2 {
    fn eq(&self, other: &Self) -> bool {
        self.hPort == other.hPort
            && self.hConnection == other.hConnection
            && self.dwConn_State == other.dwConn_State
            && self.wszPortName == other.wszPortName
            && self.wszMediaName == other.wszMediaName
            && self.wszDeviceName == other.wszDeviceName
            && self.wszDeviceType == other.wszDeviceType
            && self.dwHardwareCondition == other.dwHardwareCondition
            && self.dwLineSpeed == other.dwLineSpeed
            && self.dwCrcErr == other.dwCrcErr
            && self.dwSerialOverRunErrs == other.dwSerialOverRunErrs
            && self.dwTimeoutErr == other.dwTimeoutErr
            && self.dwAlignmentErr == other.dwAlignmentErr
            && self.dwHardwareOverrunErr == other.dwHardwareOverrunErr
            && self.dwFramingErr == other.dwFramingErr
            && self.dwBufferOverrunErr == other.dwBufferOverrunErr
            && self.dwCompressionRatioIn == other.dwCompressionRatioIn
            && self.dwCompressionRatioOut == other.dwCompressionRatioOut
            && self.dwTotalErrors == other.dwTotalErrors
            && self.ullBytesXmited == other.ullBytesXmited
            && self.ullBytesRcved == other.ullBytesRcved
            && self.ullFramesXmited == other.ullFramesXmited
            && self.ullFramesRcved == other.ullFramesRcved
            && self.ullBytesTxUncompressed == other.ullBytesTxUncompressed
            && self.ullBytesTxCompressed == other.ullBytesTxCompressed
            && self.ullBytesRcvUncompressed == other.ullBytesRcvUncompressed
            && self.ullBytesRcvCompressed == other.ullBytesRcvCompressed
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for RAS_PORT_2 {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for RAS_PORT_2 {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct RAS_PORT_CONDITION(pub i32);
pub const RAS_PORT_NON_OPERATIONAL: RAS_PORT_CONDITION = RAS_PORT_CONDITION(0i32);
pub const RAS_PORT_DISCONNECTED: RAS_PORT_CONDITION = RAS_PORT_CONDITION(1i32);
pub const RAS_PORT_CALLING_BACK: RAS_PORT_CONDITION = RAS_PORT_CONDITION(2i32);
pub const RAS_PORT_LISTENING: RAS_PORT_CONDITION = RAS_PORT_CONDITION(3i32);
pub const RAS_PORT_AUTHENTICATING: RAS_PORT_CONDITION = RAS_PORT_CONDITION(4i32);
pub const RAS_PORT_AUTHENTICATED: RAS_PORT_CONDITION = RAS_PORT_CONDITION(5i32);
pub const RAS_PORT_INITIALIZING: RAS_PORT_CONDITION = RAS_PORT_CONDITION(6i32);
impl ::std::convert::From<i32> for RAS_PORT_CONDITION {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for RAS_PORT_CONDITION {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
pub struct RAS_PROJECTION_INFO {
    pub version: RASAPIVERSION,
    pub r#type: RASPROJECTION_INFO_TYPE,
    pub Anonymous: RAS_PROJECTION_INFO_0,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
impl RAS_PROJECTION_INFO {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
impl ::std::default::Default for RAS_PROJECTION_INFO {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
impl ::std::cmp::PartialEq for RAS_PROJECTION_INFO {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
impl ::std::cmp::Eq for RAS_PROJECTION_INFO {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
unsafe impl ::windows::runtime::Abi for RAS_PROJECTION_INFO {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
pub union RAS_PROJECTION_INFO_0 {
    pub ppp: RASPPP_PROJECTION_INFO,
    pub ikev2: RASIKEV2_PROJECTION_INFO,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
impl RAS_PROJECTION_INFO_0 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
impl ::std::default::Default for RAS_PROJECTION_INFO_0 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
impl ::std::cmp::PartialEq for RAS_PROJECTION_INFO_0 {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
impl ::std::cmp::Eq for RAS_PROJECTION_INFO_0 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
unsafe impl ::windows::runtime::Abi for RAS_PROJECTION_INFO_0 {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct RAS_QUARANTINE_STATE(pub i32);
pub const RAS_QUAR_STATE_NORMAL: RAS_QUARANTINE_STATE = RAS_QUARANTINE_STATE(0i32);
pub const RAS_QUAR_STATE_QUARANTINE: RAS_QUARANTINE_STATE = RAS_QUARANTINE_STATE(1i32);
pub const RAS_QUAR_STATE_PROBATION: RAS_QUARANTINE_STATE = RAS_QUARANTINE_STATE(2i32);
pub const RAS_QUAR_STATE_NOT_CAPABLE: RAS_QUARANTINE_STATE = RAS_QUARANTINE_STATE(3i32);
impl ::std::convert::From<i32> for RAS_QUARANTINE_STATE {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for RAS_QUARANTINE_STATE {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct RAS_SECURITY_INFO {
    pub LastError: u32,
    pub BytesReceived: u32,
    pub DeviceName: [super::super::Foundation::CHAR; 129],
}
#[cfg(feature = "Win32_Foundation")]
impl RAS_SECURITY_INFO {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for RAS_SECURITY_INFO {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::fmt::Debug for RAS_SECURITY_INFO {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("RAS_SECURITY_INFO").field("LastError", &self.LastError).field("BytesReceived", &self.BytesReceived).field("DeviceName", &self.DeviceName).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for RAS_SECURITY_INFO {
    fn eq(&self, other: &Self) -> bool {
        self.LastError == other.LastError && self.BytesReceived == other.BytesReceived && self.DeviceName == other.DeviceName
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for RAS_SECURITY_INFO {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for RAS_SECURITY_INFO {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
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
impl RAS_STATS {}
impl ::std::default::Default for RAS_STATS {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for RAS_STATS {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("RAS_STATS")
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
impl ::std::cmp::PartialEq for RAS_STATS {
    fn eq(&self, other: &Self) -> bool {
        self.dwSize == other.dwSize
            && self.dwBytesXmited == other.dwBytesXmited
            && self.dwBytesRcved == other.dwBytesRcved
            && self.dwFramesXmited == other.dwFramesXmited
            && self.dwFramesRcved == other.dwFramesRcved
            && self.dwCrcErr == other.dwCrcErr
            && self.dwTimeoutErr == other.dwTimeoutErr
            && self.dwAlignmentErr == other.dwAlignmentErr
            && self.dwHardwareOverrunErr == other.dwHardwareOverrunErr
            && self.dwFramingErr == other.dwFramingErr
            && self.dwBufferOverrunErr == other.dwBufferOverrunErr
            && self.dwCompressionRatioIn == other.dwCompressionRatioIn
            && self.dwCompressionRatioOut == other.dwCompressionRatioOut
            && self.dwBps == other.dwBps
            && self.dwConnectDuration == other.dwConnectDuration
    }
}
impl ::std::cmp::Eq for RAS_STATS {}
unsafe impl ::windows::runtime::Abi for RAS_STATS {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
pub struct RAS_UPDATE_CONNECTION {
    pub Header: MPRAPI_OBJECT_HEADER,
    pub dwIfIndex: u32,
    pub wszLocalEndpointAddress: [u16; 65],
    pub wszRemoteEndpointAddress: [u16; 65],
}
impl RAS_UPDATE_CONNECTION {}
impl ::std::default::Default for RAS_UPDATE_CONNECTION {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for RAS_UPDATE_CONNECTION {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("RAS_UPDATE_CONNECTION").field("Header", &self.Header).field("dwIfIndex", &self.dwIfIndex).field("wszLocalEndpointAddress", &self.wszLocalEndpointAddress).field("wszRemoteEndpointAddress", &self.wszRemoteEndpointAddress).finish()
    }
}
impl ::std::cmp::PartialEq for RAS_UPDATE_CONNECTION {
    fn eq(&self, other: &Self) -> bool {
        self.Header == other.Header && self.dwIfIndex == other.dwIfIndex && self.wszLocalEndpointAddress == other.wszLocalEndpointAddress && self.wszRemoteEndpointAddress == other.wszRemoteEndpointAddress
    }
}
impl ::std::cmp::Eq for RAS_UPDATE_CONNECTION {}
unsafe impl ::windows::runtime::Abi for RAS_UPDATE_CONNECTION {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
pub struct RAS_USER_0 {
    pub bfPrivilege: u8,
    pub wszPhoneNumber: [u16; 129],
}
impl RAS_USER_0 {}
impl ::std::default::Default for RAS_USER_0 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for RAS_USER_0 {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("RAS_USER_0").field("bfPrivilege", &self.bfPrivilege).field("wszPhoneNumber", &self.wszPhoneNumber).finish()
    }
}
impl ::std::cmp::PartialEq for RAS_USER_0 {
    fn eq(&self, other: &Self) -> bool {
        self.bfPrivilege == other.bfPrivilege && self.wszPhoneNumber == other.wszPhoneNumber
    }
}
impl ::std::cmp::Eq for RAS_USER_0 {}
unsafe impl ::windows::runtime::Abi for RAS_USER_0 {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
pub struct RAS_USER_1 {
    pub bfPrivilege: u8,
    pub wszPhoneNumber: [u16; 129],
    pub bfPrivilege2: u8,
}
impl RAS_USER_1 {}
impl ::std::default::Default for RAS_USER_1 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for RAS_USER_1 {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("RAS_USER_1").field("bfPrivilege", &self.bfPrivilege).field("wszPhoneNumber", &self.wszPhoneNumber).field("bfPrivilege2", &self.bfPrivilege2).finish()
    }
}
impl ::std::cmp::PartialEq for RAS_USER_1 {
    fn eq(&self, other: &Self) -> bool {
        self.bfPrivilege == other.bfPrivilege && self.wszPhoneNumber == other.wszPhoneNumber && self.bfPrivilege2 == other.bfPrivilege2
    }
}
impl ::std::cmp::Eq for RAS_USER_1 {}
unsafe impl ::windows::runtime::Abi for RAS_USER_1 {
    type Abi = Self;
    type DefaultType = Self;
}
pub const RCD_AllUsers: u32 = 1u32;
pub const RCD_Eap: u32 = 2u32;
pub const RCD_Logon: u32 = 4u32;
pub const RCD_SingleUser: u32 = 0u32;
pub const RDEOPT_CustomDial: u32 = 4096u32;
pub const RDEOPT_DisableConnectedUI: u32 = 64u32;
pub const RDEOPT_DisableReconnect: u32 = 256u32;
pub const RDEOPT_DisableReconnectUI: u32 = 128u32;
pub const RDEOPT_EapInfoCryptInCapable: u32 = 32768u32;
pub const RDEOPT_IgnoreModemSpeaker: u32 = 4u32;
pub const RDEOPT_IgnoreSoftwareCompression: u32 = 16u32;
pub const RDEOPT_InvokeAutoTriggerCredentialUI: u32 = 16384u32;
pub const RDEOPT_NoUser: u32 = 512u32;
pub const RDEOPT_PauseOnScript: u32 = 1024u32;
pub const RDEOPT_PausedStates: u32 = 2u32;
pub const RDEOPT_Router: u32 = 2048u32;
pub const RDEOPT_SetModemSpeaker: u32 = 8u32;
pub const RDEOPT_SetSoftwareCompression: u32 = 32u32;
pub const RDEOPT_UseCustomScripting: u32 = 8192u32;
pub const RDEOPT_UsePrefixSuffix: u32 = 1u32;
pub const REN_AllUsers: u32 = 1u32;
pub const REN_User: u32 = 0u32;
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct ROUTER_CONNECTION_STATE(pub i32);
pub const ROUTER_IF_STATE_UNREACHABLE: ROUTER_CONNECTION_STATE = ROUTER_CONNECTION_STATE(0i32);
pub const ROUTER_IF_STATE_DISCONNECTED: ROUTER_CONNECTION_STATE = ROUTER_CONNECTION_STATE(1i32);
pub const ROUTER_IF_STATE_CONNECTING: ROUTER_CONNECTION_STATE = ROUTER_CONNECTION_STATE(2i32);
pub const ROUTER_IF_STATE_CONNECTED: ROUTER_CONNECTION_STATE = ROUTER_CONNECTION_STATE(3i32);
impl ::std::convert::From<i32> for ROUTER_CONNECTION_STATE {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for ROUTER_CONNECTION_STATE {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
pub struct ROUTER_CUSTOM_IKEv2_POLICY0 {
    pub dwIntegrityMethod: u32,
    pub dwEncryptionMethod: u32,
    pub dwCipherTransformConstant: u32,
    pub dwAuthTransformConstant: u32,
    pub dwPfsGroup: u32,
    pub dwDhGroup: u32,
}
impl ROUTER_CUSTOM_IKEv2_POLICY0 {}
impl ::std::default::Default for ROUTER_CUSTOM_IKEv2_POLICY0 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for ROUTER_CUSTOM_IKEv2_POLICY0 {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("ROUTER_CUSTOM_IKEv2_POLICY0")
            .field("dwIntegrityMethod", &self.dwIntegrityMethod)
            .field("dwEncryptionMethod", &self.dwEncryptionMethod)
            .field("dwCipherTransformConstant", &self.dwCipherTransformConstant)
            .field("dwAuthTransformConstant", &self.dwAuthTransformConstant)
            .field("dwPfsGroup", &self.dwPfsGroup)
            .field("dwDhGroup", &self.dwDhGroup)
            .finish()
    }
}
impl ::std::cmp::PartialEq for ROUTER_CUSTOM_IKEv2_POLICY0 {
    fn eq(&self, other: &Self) -> bool {
        self.dwIntegrityMethod == other.dwIntegrityMethod && self.dwEncryptionMethod == other.dwEncryptionMethod && self.dwCipherTransformConstant == other.dwCipherTransformConstant && self.dwAuthTransformConstant == other.dwAuthTransformConstant && self.dwPfsGroup == other.dwPfsGroup && self.dwDhGroup == other.dwDhGroup
    }
}
impl ::std::cmp::Eq for ROUTER_CUSTOM_IKEv2_POLICY0 {}
unsafe impl ::windows::runtime::Abi for ROUTER_CUSTOM_IKEv2_POLICY0 {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Security_Cryptography")]
pub struct ROUTER_IKEv2_IF_CUSTOM_CONFIG0 {
    pub dwSaLifeTime: u32,
    pub dwSaDataSize: u32,
    pub certificateName: super::super::Security::Cryptography::CRYPTOAPI_BLOB,
    pub customPolicy: *mut ROUTER_CUSTOM_IKEv2_POLICY0,
}
#[cfg(feature = "Win32_Security_Cryptography")]
impl ROUTER_IKEv2_IF_CUSTOM_CONFIG0 {}
#[cfg(feature = "Win32_Security_Cryptography")]
impl ::std::default::Default for ROUTER_IKEv2_IF_CUSTOM_CONFIG0 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Security_Cryptography")]
impl ::std::fmt::Debug for ROUTER_IKEv2_IF_CUSTOM_CONFIG0 {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("ROUTER_IKEv2_IF_CUSTOM_CONFIG0").field("dwSaLifeTime", &self.dwSaLifeTime).field("dwSaDataSize", &self.dwSaDataSize).field("certificateName", &self.certificateName).field("customPolicy", &self.customPolicy).finish()
    }
}
#[cfg(feature = "Win32_Security_Cryptography")]
impl ::std::cmp::PartialEq for ROUTER_IKEv2_IF_CUSTOM_CONFIG0 {
    fn eq(&self, other: &Self) -> bool {
        self.dwSaLifeTime == other.dwSaLifeTime && self.dwSaDataSize == other.dwSaDataSize && self.certificateName == other.certificateName && self.customPolicy == other.customPolicy
    }
}
#[cfg(feature = "Win32_Security_Cryptography")]
impl ::std::cmp::Eq for ROUTER_IKEv2_IF_CUSTOM_CONFIG0 {}
#[cfg(feature = "Win32_Security_Cryptography")]
unsafe impl ::windows::runtime::Abi for ROUTER_IKEv2_IF_CUSTOM_CONFIG0 {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Security_Cryptography")]
pub struct ROUTER_IKEv2_IF_CUSTOM_CONFIG1 {
    pub dwSaLifeTime: u32,
    pub dwSaDataSize: u32,
    pub certificateName: super::super::Security::Cryptography::CRYPTOAPI_BLOB,
    pub customPolicy: *mut ROUTER_CUSTOM_IKEv2_POLICY0,
    pub certificateHash: super::super::Security::Cryptography::CRYPTOAPI_BLOB,
}
#[cfg(feature = "Win32_Security_Cryptography")]
impl ROUTER_IKEv2_IF_CUSTOM_CONFIG1 {}
#[cfg(feature = "Win32_Security_Cryptography")]
impl ::std::default::Default for ROUTER_IKEv2_IF_CUSTOM_CONFIG1 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Security_Cryptography")]
impl ::std::fmt::Debug for ROUTER_IKEv2_IF_CUSTOM_CONFIG1 {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("ROUTER_IKEv2_IF_CUSTOM_CONFIG1").field("dwSaLifeTime", &self.dwSaLifeTime).field("dwSaDataSize", &self.dwSaDataSize).field("certificateName", &self.certificateName).field("customPolicy", &self.customPolicy).field("certificateHash", &self.certificateHash).finish()
    }
}
#[cfg(feature = "Win32_Security_Cryptography")]
impl ::std::cmp::PartialEq for ROUTER_IKEv2_IF_CUSTOM_CONFIG1 {
    fn eq(&self, other: &Self) -> bool {
        self.dwSaLifeTime == other.dwSaLifeTime && self.dwSaDataSize == other.dwSaDataSize && self.certificateName == other.certificateName && self.customPolicy == other.customPolicy && self.certificateHash == other.certificateHash
    }
}
#[cfg(feature = "Win32_Security_Cryptography")]
impl ::std::cmp::Eq for ROUTER_IKEv2_IF_CUSTOM_CONFIG1 {}
#[cfg(feature = "Win32_Security_Cryptography")]
unsafe impl ::windows::runtime::Abi for ROUTER_IKEv2_IF_CUSTOM_CONFIG1 {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
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
impl ROUTER_IKEv2_IF_CUSTOM_CONFIG2 {}
#[cfg(all(feature = "Win32_Networking_WinSock", feature = "Win32_Security_Cryptography"))]
impl ::std::default::Default for ROUTER_IKEv2_IF_CUSTOM_CONFIG2 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(all(feature = "Win32_Networking_WinSock", feature = "Win32_Security_Cryptography"))]
impl ::std::fmt::Debug for ROUTER_IKEv2_IF_CUSTOM_CONFIG2 {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("ROUTER_IKEv2_IF_CUSTOM_CONFIG2")
            .field("dwSaLifeTime", &self.dwSaLifeTime)
            .field("dwSaDataSize", &self.dwSaDataSize)
            .field("certificateName", &self.certificateName)
            .field("customPolicy", &self.customPolicy)
            .field("certificateHash", &self.certificateHash)
            .field("dwMmSaLifeTime", &self.dwMmSaLifeTime)
            .field("vpnTrafficSelectors", &self.vpnTrafficSelectors)
            .finish()
    }
}
#[cfg(all(feature = "Win32_Networking_WinSock", feature = "Win32_Security_Cryptography"))]
impl ::std::cmp::PartialEq for ROUTER_IKEv2_IF_CUSTOM_CONFIG2 {
    fn eq(&self, other: &Self) -> bool {
        self.dwSaLifeTime == other.dwSaLifeTime && self.dwSaDataSize == other.dwSaDataSize && self.certificateName == other.certificateName && self.customPolicy == other.customPolicy && self.certificateHash == other.certificateHash && self.dwMmSaLifeTime == other.dwMmSaLifeTime && self.vpnTrafficSelectors == other.vpnTrafficSelectors
    }
}
#[cfg(all(feature = "Win32_Networking_WinSock", feature = "Win32_Security_Cryptography"))]
impl ::std::cmp::Eq for ROUTER_IKEv2_IF_CUSTOM_CONFIG2 {}
#[cfg(all(feature = "Win32_Networking_WinSock", feature = "Win32_Security_Cryptography"))]
unsafe impl ::windows::runtime::Abi for ROUTER_IKEv2_IF_CUSTOM_CONFIG2 {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct ROUTER_INTERFACE_TYPE(pub i32);
pub const ROUTER_IF_TYPE_CLIENT: ROUTER_INTERFACE_TYPE = ROUTER_INTERFACE_TYPE(0i32);
pub const ROUTER_IF_TYPE_HOME_ROUTER: ROUTER_INTERFACE_TYPE = ROUTER_INTERFACE_TYPE(1i32);
pub const ROUTER_IF_TYPE_FULL_ROUTER: ROUTER_INTERFACE_TYPE = ROUTER_INTERFACE_TYPE(2i32);
pub const ROUTER_IF_TYPE_DEDICATED: ROUTER_INTERFACE_TYPE = ROUTER_INTERFACE_TYPE(3i32);
pub const ROUTER_IF_TYPE_INTERNAL: ROUTER_INTERFACE_TYPE = ROUTER_INTERFACE_TYPE(4i32);
pub const ROUTER_IF_TYPE_LOOPBACK: ROUTER_INTERFACE_TYPE = ROUTER_INTERFACE_TYPE(5i32);
pub const ROUTER_IF_TYPE_TUNNEL1: ROUTER_INTERFACE_TYPE = ROUTER_INTERFACE_TYPE(6i32);
pub const ROUTER_IF_TYPE_DIALOUT: ROUTER_INTERFACE_TYPE = ROUTER_INTERFACE_TYPE(7i32);
pub const ROUTER_IF_TYPE_MAX: ROUTER_INTERFACE_TYPE = ROUTER_INTERFACE_TYPE(8i32);
impl ::std::convert::From<i32> for ROUTER_INTERFACE_TYPE {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for ROUTER_INTERFACE_TYPE {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct ROUTING_PROTOCOL_CONFIG {
    pub dwCallbackFlags: u32,
    pub pfnRpfCallback: ::std::option::Option<PMGM_RPF_CALLBACK>,
    pub pfnCreationAlertCallback: ::std::option::Option<PMGM_CREATION_ALERT_CALLBACK>,
    pub pfnPruneAlertCallback: ::std::option::Option<PMGM_PRUNE_ALERT_CALLBACK>,
    pub pfnJoinAlertCallback: ::std::option::Option<PMGM_JOIN_ALERT_CALLBACK>,
    pub pfnWrongIfCallback: ::std::option::Option<PMGM_WRONG_IF_CALLBACK>,
    pub pfnLocalJoinCallback: ::std::option::Option<PMGM_LOCAL_JOIN_CALLBACK>,
    pub pfnLocalLeaveCallback: ::std::option::Option<PMGM_LOCAL_LEAVE_CALLBACK>,
    pub pfnDisableIgmpCallback: ::std::option::Option<PMGM_DISABLE_IGMP_CALLBACK>,
    pub pfnEnableIgmpCallback: ::std::option::Option<PMGM_ENABLE_IGMP_CALLBACK>,
}
#[cfg(feature = "Win32_Foundation")]
impl ROUTING_PROTOCOL_CONFIG {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for ROUTING_PROTOCOL_CONFIG {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::fmt::Debug for ROUTING_PROTOCOL_CONFIG {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("ROUTING_PROTOCOL_CONFIG").field("dwCallbackFlags", &self.dwCallbackFlags).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for ROUTING_PROTOCOL_CONFIG {
    fn eq(&self, other: &Self) -> bool {
        self.dwCallbackFlags == other.dwCallbackFlags
            && self.pfnRpfCallback.map(|f| f as usize) == other.pfnRpfCallback.map(|f| f as usize)
            && self.pfnCreationAlertCallback.map(|f| f as usize) == other.pfnCreationAlertCallback.map(|f| f as usize)
            && self.pfnPruneAlertCallback.map(|f| f as usize) == other.pfnPruneAlertCallback.map(|f| f as usize)
            && self.pfnJoinAlertCallback.map(|f| f as usize) == other.pfnJoinAlertCallback.map(|f| f as usize)
            && self.pfnWrongIfCallback.map(|f| f as usize) == other.pfnWrongIfCallback.map(|f| f as usize)
            && self.pfnLocalJoinCallback.map(|f| f as usize) == other.pfnLocalJoinCallback.map(|f| f as usize)
            && self.pfnLocalLeaveCallback.map(|f| f as usize) == other.pfnLocalLeaveCallback.map(|f| f as usize)
            && self.pfnDisableIgmpCallback.map(|f| f as usize) == other.pfnDisableIgmpCallback.map(|f| f as usize)
            && self.pfnEnableIgmpCallback.map(|f| f as usize) == other.pfnEnableIgmpCallback.map(|f| f as usize)
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for ROUTING_PROTOCOL_CONFIG {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for ROUTING_PROTOCOL_CONFIG {
    type Abi = ::std::mem::ManuallyDrop<Self>;
    type DefaultType = Self;
}
pub const RTM_BLOCK_METHODS: u32 = 1u32;
pub const RTM_CHANGE_TYPE_ALL: u32 = 1u32;
pub const RTM_CHANGE_TYPE_BEST: u32 = 2u32;
pub const RTM_CHANGE_TYPE_FORWARDING: u32 = 4u32;
pub const RTM_DEST_FLAG_DONT_FORWARD: u32 = 4u32;
pub const RTM_DEST_FLAG_FWD_ENGIN_ADD: u32 = 2u32;
pub const RTM_DEST_FLAG_NATURAL_NET: u32 = 1u32;
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
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
impl RTM_DEST_INFO {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for RTM_DEST_INFO {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::fmt::Debug for RTM_DEST_INFO {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("RTM_DEST_INFO").field("DestHandle", &self.DestHandle).field("DestAddress", &self.DestAddress).field("LastChanged", &self.LastChanged).field("BelongsToViews", &self.BelongsToViews).field("NumberOfViews", &self.NumberOfViews).field("ViewInfo", &self.ViewInfo).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for RTM_DEST_INFO {
    fn eq(&self, other: &Self) -> bool {
        self.DestHandle == other.DestHandle && self.DestAddress == other.DestAddress && self.LastChanged == other.LastChanged && self.BelongsToViews == other.BelongsToViews && self.NumberOfViews == other.NumberOfViews && self.ViewInfo == other.ViewInfo
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for RTM_DEST_INFO {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for RTM_DEST_INFO {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
pub struct RTM_DEST_INFO_0 {
    pub ViewId: i32,
    pub NumRoutes: u32,
    pub Route: isize,
    pub Owner: isize,
    pub DestFlags: u32,
    pub HoldRoute: isize,
}
impl RTM_DEST_INFO_0 {}
impl ::std::default::Default for RTM_DEST_INFO_0 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for RTM_DEST_INFO_0 {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("_Anonymous_e__Struct").field("ViewId", &self.ViewId).field("NumRoutes", &self.NumRoutes).field("Route", &self.Route).field("Owner", &self.Owner).field("DestFlags", &self.DestFlags).field("HoldRoute", &self.HoldRoute).finish()
    }
}
impl ::std::cmp::PartialEq for RTM_DEST_INFO_0 {
    fn eq(&self, other: &Self) -> bool {
        self.ViewId == other.ViewId && self.NumRoutes == other.NumRoutes && self.Route == other.Route && self.Owner == other.Owner && self.DestFlags == other.DestFlags && self.HoldRoute == other.HoldRoute
    }
}
impl ::std::cmp::Eq for RTM_DEST_INFO_0 {}
unsafe impl ::windows::runtime::Abi for RTM_DEST_INFO_0 {
    type Abi = Self;
    type DefaultType = Self;
}
pub type RTM_ENTITY_EXPORT_METHOD = unsafe extern "system" fn(callerhandle: isize, calleehandle: isize, input: *mut RTM_ENTITY_METHOD_INPUT, output: *mut RTM_ENTITY_METHOD_OUTPUT);
#[derive(:: std :: clone :: Clone)]
#[repr(C)]
pub struct RTM_ENTITY_EXPORT_METHODS {
    pub NumMethods: u32,
    pub Methods: [::std::option::Option<RTM_ENTITY_EXPORT_METHOD>; 1],
}
impl RTM_ENTITY_EXPORT_METHODS {}
impl ::std::default::Default for RTM_ENTITY_EXPORT_METHODS {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::cmp::PartialEq for RTM_ENTITY_EXPORT_METHODS {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
impl ::std::cmp::Eq for RTM_ENTITY_EXPORT_METHODS {}
unsafe impl ::windows::runtime::Abi for RTM_ENTITY_EXPORT_METHODS {
    type Abi = ::std::mem::ManuallyDrop<Self>;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
pub struct RTM_ENTITY_ID {
    pub Anonymous: RTM_ENTITY_ID_0,
}
impl RTM_ENTITY_ID {}
impl ::std::default::Default for RTM_ENTITY_ID {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::cmp::PartialEq for RTM_ENTITY_ID {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
impl ::std::cmp::Eq for RTM_ENTITY_ID {}
unsafe impl ::windows::runtime::Abi for RTM_ENTITY_ID {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
pub union RTM_ENTITY_ID_0 {
    pub Anonymous: RTM_ENTITY_ID_0_0,
    pub EntityId: u64,
}
impl RTM_ENTITY_ID_0 {}
impl ::std::default::Default for RTM_ENTITY_ID_0 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::cmp::PartialEq for RTM_ENTITY_ID_0 {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
impl ::std::cmp::Eq for RTM_ENTITY_ID_0 {}
unsafe impl ::windows::runtime::Abi for RTM_ENTITY_ID_0 {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
pub struct RTM_ENTITY_ID_0_0 {
    pub EntityProtocolId: u32,
    pub EntityInstanceId: u32,
}
impl RTM_ENTITY_ID_0_0 {}
impl ::std::default::Default for RTM_ENTITY_ID_0_0 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for RTM_ENTITY_ID_0_0 {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("_Anonymous_e__Struct").field("EntityProtocolId", &self.EntityProtocolId).field("EntityInstanceId", &self.EntityInstanceId).finish()
    }
}
impl ::std::cmp::PartialEq for RTM_ENTITY_ID_0_0 {
    fn eq(&self, other: &Self) -> bool {
        self.EntityProtocolId == other.EntityProtocolId && self.EntityInstanceId == other.EntityInstanceId
    }
}
impl ::std::cmp::Eq for RTM_ENTITY_ID_0_0 {}
unsafe impl ::windows::runtime::Abi for RTM_ENTITY_ID_0_0 {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
pub struct RTM_ENTITY_INFO {
    pub RtmInstanceId: u16,
    pub AddressFamily: u16,
    pub EntityId: RTM_ENTITY_ID,
}
impl RTM_ENTITY_INFO {}
impl ::std::default::Default for RTM_ENTITY_INFO {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::cmp::PartialEq for RTM_ENTITY_INFO {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
impl ::std::cmp::Eq for RTM_ENTITY_INFO {}
unsafe impl ::windows::runtime::Abi for RTM_ENTITY_INFO {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
pub struct RTM_ENTITY_METHOD_INPUT {
    pub MethodType: u32,
    pub InputSize: u32,
    pub InputData: [u8; 1],
}
impl RTM_ENTITY_METHOD_INPUT {}
impl ::std::default::Default for RTM_ENTITY_METHOD_INPUT {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for RTM_ENTITY_METHOD_INPUT {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("RTM_ENTITY_METHOD_INPUT").field("MethodType", &self.MethodType).field("InputSize", &self.InputSize).field("InputData", &self.InputData).finish()
    }
}
impl ::std::cmp::PartialEq for RTM_ENTITY_METHOD_INPUT {
    fn eq(&self, other: &Self) -> bool {
        self.MethodType == other.MethodType && self.InputSize == other.InputSize && self.InputData == other.InputData
    }
}
impl ::std::cmp::Eq for RTM_ENTITY_METHOD_INPUT {}
unsafe impl ::windows::runtime::Abi for RTM_ENTITY_METHOD_INPUT {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
pub struct RTM_ENTITY_METHOD_OUTPUT {
    pub MethodType: u32,
    pub MethodStatus: u32,
    pub OutputSize: u32,
    pub OutputData: [u8; 1],
}
impl RTM_ENTITY_METHOD_OUTPUT {}
impl ::std::default::Default for RTM_ENTITY_METHOD_OUTPUT {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for RTM_ENTITY_METHOD_OUTPUT {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("RTM_ENTITY_METHOD_OUTPUT").field("MethodType", &self.MethodType).field("MethodStatus", &self.MethodStatus).field("OutputSize", &self.OutputSize).field("OutputData", &self.OutputData).finish()
    }
}
impl ::std::cmp::PartialEq for RTM_ENTITY_METHOD_OUTPUT {
    fn eq(&self, other: &Self) -> bool {
        self.MethodType == other.MethodType && self.MethodStatus == other.MethodStatus && self.OutputSize == other.OutputSize && self.OutputData == other.OutputData
    }
}
impl ::std::cmp::Eq for RTM_ENTITY_METHOD_OUTPUT {}
unsafe impl ::windows::runtime::Abi for RTM_ENTITY_METHOD_OUTPUT {
    type Abi = Self;
    type DefaultType = Self;
}
pub const RTM_ENUM_ALL_DESTS: u32 = 0u32;
pub const RTM_ENUM_ALL_ROUTES: u32 = 0u32;
pub const RTM_ENUM_NEXT: u32 = 1u32;
pub const RTM_ENUM_OWN_DESTS: u32 = 16777216u32;
pub const RTM_ENUM_OWN_ROUTES: u32 = 65536u32;
pub const RTM_ENUM_RANGE: u32 = 2u32;
pub const RTM_ENUM_START: u32 = 0u32;
pub type RTM_EVENT_CALLBACK = unsafe extern "system" fn(rtmreghandle: isize, eventtype: RTM_EVENT_TYPE, context1: *mut ::std::ffi::c_void, context2: *mut ::std::ffi::c_void) -> u32;
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct RTM_EVENT_TYPE(pub i32);
pub const RTM_ENTITY_REGISTERED: RTM_EVENT_TYPE = RTM_EVENT_TYPE(0i32);
pub const RTM_ENTITY_DEREGISTERED: RTM_EVENT_TYPE = RTM_EVENT_TYPE(1i32);
pub const RTM_ROUTE_EXPIRED: RTM_EVENT_TYPE = RTM_EVENT_TYPE(2i32);
pub const RTM_CHANGE_NOTIFICATION: RTM_EVENT_TYPE = RTM_EVENT_TYPE(3i32);
impl ::std::convert::From<i32> for RTM_EVENT_TYPE {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for RTM_EVENT_TYPE {
    type Abi = Self;
    type DefaultType = Self;
}
pub const RTM_MATCH_FULL: u32 = 65535u32;
pub const RTM_MATCH_INTERFACE: u32 = 16u32;
pub const RTM_MATCH_NEIGHBOUR: u32 = 2u32;
pub const RTM_MATCH_NEXTHOP: u32 = 8u32;
pub const RTM_MATCH_NONE: u32 = 0u32;
pub const RTM_MATCH_OWNER: u32 = 1u32;
pub const RTM_MATCH_PREF: u32 = 4u32;
pub const RTM_MAX_ADDRESS_SIZE: u32 = 16u32;
pub const RTM_MAX_VIEWS: u32 = 32u32;
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
pub struct RTM_NET_ADDRESS {
    pub AddressFamily: u16,
    pub NumBits: u16,
    pub AddrBits: [u8; 16],
}
impl RTM_NET_ADDRESS {}
impl ::std::default::Default for RTM_NET_ADDRESS {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for RTM_NET_ADDRESS {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("RTM_NET_ADDRESS").field("AddressFamily", &self.AddressFamily).field("NumBits", &self.NumBits).field("AddrBits", &self.AddrBits).finish()
    }
}
impl ::std::cmp::PartialEq for RTM_NET_ADDRESS {
    fn eq(&self, other: &Self) -> bool {
        self.AddressFamily == other.AddressFamily && self.NumBits == other.NumBits && self.AddrBits == other.AddrBits
    }
}
impl ::std::cmp::Eq for RTM_NET_ADDRESS {}
unsafe impl ::windows::runtime::Abi for RTM_NET_ADDRESS {
    type Abi = Self;
    type DefaultType = Self;
}
pub const RTM_NEXTHOP_CHANGE_NEW: u32 = 1u32;
pub const RTM_NEXTHOP_FLAGS_DOWN: u32 = 2u32;
pub const RTM_NEXTHOP_FLAGS_REMOTE: u32 = 1u32;
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
pub struct RTM_NEXTHOP_INFO {
    pub NextHopAddress: RTM_NET_ADDRESS,
    pub NextHopOwner: isize,
    pub InterfaceIndex: u32,
    pub State: u16,
    pub Flags: u16,
    pub EntitySpecificInfo: *mut ::std::ffi::c_void,
    pub RemoteNextHop: isize,
}
impl RTM_NEXTHOP_INFO {}
impl ::std::default::Default for RTM_NEXTHOP_INFO {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for RTM_NEXTHOP_INFO {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("RTM_NEXTHOP_INFO")
            .field("NextHopAddress", &self.NextHopAddress)
            .field("NextHopOwner", &self.NextHopOwner)
            .field("InterfaceIndex", &self.InterfaceIndex)
            .field("State", &self.State)
            .field("Flags", &self.Flags)
            .field("EntitySpecificInfo", &self.EntitySpecificInfo)
            .field("RemoteNextHop", &self.RemoteNextHop)
            .finish()
    }
}
impl ::std::cmp::PartialEq for RTM_NEXTHOP_INFO {
    fn eq(&self, other: &Self) -> bool {
        self.NextHopAddress == other.NextHopAddress && self.NextHopOwner == other.NextHopOwner && self.InterfaceIndex == other.InterfaceIndex && self.State == other.State && self.Flags == other.Flags && self.EntitySpecificInfo == other.EntitySpecificInfo && self.RemoteNextHop == other.RemoteNextHop
    }
}
impl ::std::cmp::Eq for RTM_NEXTHOP_INFO {}
unsafe impl ::windows::runtime::Abi for RTM_NEXTHOP_INFO {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
pub struct RTM_NEXTHOP_LIST {
    pub NumNextHops: u16,
    pub NextHops: [isize; 1],
}
impl RTM_NEXTHOP_LIST {}
impl ::std::default::Default for RTM_NEXTHOP_LIST {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for RTM_NEXTHOP_LIST {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("RTM_NEXTHOP_LIST").field("NumNextHops", &self.NumNextHops).field("NextHops", &self.NextHops).finish()
    }
}
impl ::std::cmp::PartialEq for RTM_NEXTHOP_LIST {
    fn eq(&self, other: &Self) -> bool {
        self.NumNextHops == other.NumNextHops && self.NextHops == other.NextHops
    }
}
impl ::std::cmp::Eq for RTM_NEXTHOP_LIST {}
unsafe impl ::windows::runtime::Abi for RTM_NEXTHOP_LIST {
    type Abi = Self;
    type DefaultType = Self;
}
pub const RTM_NEXTHOP_STATE_CREATED: u32 = 0u32;
pub const RTM_NEXTHOP_STATE_DELETED: u32 = 1u32;
pub const RTM_NOTIFY_ONLY_MARKED_DESTS: u32 = 65536u32;
pub const RTM_NUM_CHANGE_TYPES: u32 = 3u32;
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
pub struct RTM_PREF_INFO {
    pub Metric: u32,
    pub Preference: u32,
}
impl RTM_PREF_INFO {}
impl ::std::default::Default for RTM_PREF_INFO {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for RTM_PREF_INFO {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("RTM_PREF_INFO").field("Metric", &self.Metric).field("Preference", &self.Preference).finish()
    }
}
impl ::std::cmp::PartialEq for RTM_PREF_INFO {
    fn eq(&self, other: &Self) -> bool {
        self.Metric == other.Metric && self.Preference == other.Preference
    }
}
impl ::std::cmp::Eq for RTM_PREF_INFO {}
unsafe impl ::windows::runtime::Abi for RTM_PREF_INFO {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
pub struct RTM_REGN_PROFILE {
    pub MaxNextHopsInRoute: u32,
    pub MaxHandlesInEnum: u32,
    pub ViewsSupported: u32,
    pub NumberOfViews: u32,
}
impl RTM_REGN_PROFILE {}
impl ::std::default::Default for RTM_REGN_PROFILE {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for RTM_REGN_PROFILE {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("RTM_REGN_PROFILE").field("MaxNextHopsInRoute", &self.MaxNextHopsInRoute).field("MaxHandlesInEnum", &self.MaxHandlesInEnum).field("ViewsSupported", &self.ViewsSupported).field("NumberOfViews", &self.NumberOfViews).finish()
    }
}
impl ::std::cmp::PartialEq for RTM_REGN_PROFILE {
    fn eq(&self, other: &Self) -> bool {
        self.MaxNextHopsInRoute == other.MaxNextHopsInRoute && self.MaxHandlesInEnum == other.MaxHandlesInEnum && self.ViewsSupported == other.ViewsSupported && self.NumberOfViews == other.NumberOfViews
    }
}
impl ::std::cmp::Eq for RTM_REGN_PROFILE {}
unsafe impl ::windows::runtime::Abi for RTM_REGN_PROFILE {
    type Abi = Self;
    type DefaultType = Self;
}
pub const RTM_RESUME_METHODS: u32 = 0u32;
pub const RTM_ROUTE_CHANGE_BEST: u32 = 65536u32;
pub const RTM_ROUTE_CHANGE_FIRST: u32 = 1u32;
pub const RTM_ROUTE_CHANGE_NEW: u32 = 2u32;
pub const RTM_ROUTE_FLAGS_BLACKHOLE: u32 = 2u32;
pub const RTM_ROUTE_FLAGS_DISCARD: u32 = 4u32;
pub const RTM_ROUTE_FLAGS_INACTIVE: u32 = 8u32;
pub const RTM_ROUTE_FLAGS_LIMITED_BC: u32 = 1024u32;
pub const RTM_ROUTE_FLAGS_LOCAL: u32 = 16u32;
pub const RTM_ROUTE_FLAGS_LOCAL_MCAST: u32 = 512u32;
pub const RTM_ROUTE_FLAGS_LOOPBACK: u32 = 128u32;
pub const RTM_ROUTE_FLAGS_MARTIAN: u32 = 1u32;
pub const RTM_ROUTE_FLAGS_MCAST: u32 = 256u32;
pub const RTM_ROUTE_FLAGS_MYSELF: u32 = 64u32;
pub const RTM_ROUTE_FLAGS_ONES_NETBC: u32 = 16384u32;
pub const RTM_ROUTE_FLAGS_ONES_SUBNETBC: u32 = 32768u32;
pub const RTM_ROUTE_FLAGS_REMOTE: u32 = 32u32;
pub const RTM_ROUTE_FLAGS_ZEROS_NETBC: u32 = 4096u32;
pub const RTM_ROUTE_FLAGS_ZEROS_SUBNETBC: u32 = 8192u32;
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
pub struct RTM_ROUTE_INFO {
    pub DestHandle: isize,
    pub RouteOwner: isize,
    pub Neighbour: isize,
    pub State: u8,
    pub Flags1: u8,
    pub Flags: u16,
    pub PrefInfo: RTM_PREF_INFO,
    pub BelongsToViews: u32,
    pub EntitySpecificInfo: *mut ::std::ffi::c_void,
    pub NextHopsList: RTM_NEXTHOP_LIST,
}
impl RTM_ROUTE_INFO {}
impl ::std::default::Default for RTM_ROUTE_INFO {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for RTM_ROUTE_INFO {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("RTM_ROUTE_INFO")
            .field("DestHandle", &self.DestHandle)
            .field("RouteOwner", &self.RouteOwner)
            .field("Neighbour", &self.Neighbour)
            .field("State", &self.State)
            .field("Flags1", &self.Flags1)
            .field("Flags", &self.Flags)
            .field("PrefInfo", &self.PrefInfo)
            .field("BelongsToViews", &self.BelongsToViews)
            .field("EntitySpecificInfo", &self.EntitySpecificInfo)
            .field("NextHopsList", &self.NextHopsList)
            .finish()
    }
}
impl ::std::cmp::PartialEq for RTM_ROUTE_INFO {
    fn eq(&self, other: &Self) -> bool {
        self.DestHandle == other.DestHandle && self.RouteOwner == other.RouteOwner && self.Neighbour == other.Neighbour && self.State == other.State && self.Flags1 == other.Flags1 && self.Flags == other.Flags && self.PrefInfo == other.PrefInfo && self.BelongsToViews == other.BelongsToViews && self.EntitySpecificInfo == other.EntitySpecificInfo && self.NextHopsList == other.NextHopsList
    }
}
impl ::std::cmp::Eq for RTM_ROUTE_INFO {}
unsafe impl ::windows::runtime::Abi for RTM_ROUTE_INFO {
    type Abi = Self;
    type DefaultType = Self;
}
pub const RTM_ROUTE_STATE_CREATED: u32 = 0u32;
pub const RTM_ROUTE_STATE_DELETED: u32 = 2u32;
pub const RTM_ROUTE_STATE_DELETING: u32 = 1u32;
pub const RTM_VIEW_ID_MCAST: u32 = 1u32;
pub const RTM_VIEW_ID_UCAST: u32 = 0u32;
pub const RTM_VIEW_MASK_ALL: u32 = 4294967295u32;
pub const RTM_VIEW_MASK_ANY: u32 = 0u32;
pub const RTM_VIEW_MASK_MCAST: u32 = 2u32;
pub const RTM_VIEW_MASK_NONE: u32 = 0u32;
pub const RTM_VIEW_MASK_SIZE: u32 = 32u32;
pub const RTM_VIEW_MASK_UCAST: u32 = 1u32;
#[inline]
pub unsafe fn RasClearConnectionStatistics<'a, Param0: ::windows::runtime::IntoParam<'a, HRASCONN>>(hrasconn: Param0) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn RasClearConnectionStatistics(hrasconn: HRASCONN) -> u32;
        }
        ::std::mem::transmute(RasClearConnectionStatistics(hrasconn.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn RasClearLinkStatistics<'a, Param0: ::windows::runtime::IntoParam<'a, HRASCONN>>(hrasconn: Param0, dwsubentry: u32) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn RasClearLinkStatistics(hrasconn: HRASCONN, dwsubentry: u32) -> u32;
        }
        ::std::mem::transmute(RasClearLinkStatistics(hrasconn.into_param().abi(), ::std::mem::transmute(dwsubentry)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn RasConnectionNotificationA<'a, Param0: ::windows::runtime::IntoParam<'a, HRASCONN>, Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::HANDLE>>(param0: Param0, param1: Param1, param2: u32) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn RasConnectionNotificationA(param0: HRASCONN, param1: super::super::Foundation::HANDLE, param2: u32) -> u32;
        }
        ::std::mem::transmute(RasConnectionNotificationA(param0.into_param().abi(), param1.into_param().abi(), ::std::mem::transmute(param2)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn RasConnectionNotificationW<'a, Param0: ::windows::runtime::IntoParam<'a, HRASCONN>, Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::HANDLE>>(param0: Param0, param1: Param1, param2: u32) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn RasConnectionNotificationW(param0: HRASCONN, param1: super::super::Foundation::HANDLE, param2: u32) -> u32;
        }
        ::std::mem::transmute(RasConnectionNotificationW(param0.into_param().abi(), param1.into_param().abi(), ::std::mem::transmute(param2)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn RasCreatePhonebookEntryA<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::HWND>, Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::PSTR>>(param0: Param0, param1: Param1) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn RasCreatePhonebookEntryA(param0: super::super::Foundation::HWND, param1: super::super::Foundation::PSTR) -> u32;
        }
        ::std::mem::transmute(RasCreatePhonebookEntryA(param0.into_param().abi(), param1.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn RasCreatePhonebookEntryW<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::HWND>, Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>>(param0: Param0, param1: Param1) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn RasCreatePhonebookEntryW(param0: super::super::Foundation::HWND, param1: super::super::Foundation::PWSTR) -> u32;
        }
        ::std::mem::transmute(RasCreatePhonebookEntryW(param0.into_param().abi(), param1.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
pub type RasCustomDeleteEntryNotifyFn = unsafe extern "system" fn(lpszphonebook: super::super::Foundation::PWSTR, lpszentry: super::super::Foundation::PWSTR, dwflags: u32) -> u32;
#[cfg(feature = "Win32_Foundation")]
pub type RasCustomDialDlgFn = unsafe extern "system" fn(hinstdll: super::super::Foundation::HINSTANCE, dwflags: u32, lpszphonebook: super::super::Foundation::PWSTR, lpszentry: super::super::Foundation::PWSTR, lpszphonenumber: super::super::Foundation::PWSTR, lpinfo: *mut RASDIALDLG, pvinfo: *mut ::std::ffi::c_void) -> super::super::Foundation::BOOL;
#[cfg(feature = "Win32_Foundation")]
pub type RasCustomDialFn = unsafe extern "system" fn(hinstdll: super::super::Foundation::HINSTANCE, lprasdialextensions: *mut RASDIALEXTENSIONS, lpszphonebook: super::super::Foundation::PWSTR, lprasdialparams: *mut RASDIALPARAMSA, dwnotifiertype: u32, lpvnotifier: *mut ::std::ffi::c_void, lphrasconn: *mut HRASCONN, dwflags: u32) -> u32;
#[cfg(feature = "Win32_Foundation")]
pub type RasCustomEntryDlgFn = unsafe extern "system" fn(hinstdll: super::super::Foundation::HINSTANCE, lpszphonebook: super::super::Foundation::PWSTR, lpszentry: super::super::Foundation::PWSTR, lpinfo: *mut RASENTRYDLGA, dwflags: u32) -> super::super::Foundation::BOOL;
pub type RasCustomHangUpFn = unsafe extern "system" fn(hrasconn: HRASCONN) -> u32;
#[cfg(feature = "Win32_Foundation")]
pub type RasCustomScriptExecuteFn = unsafe extern "system" fn(
    hport: super::super::Foundation::HANDLE,
    lpszphonebook: super::super::Foundation::PWSTR,
    lpszentryname: super::super::Foundation::PWSTR,
    pfnrasgetbuffer: ::windows::runtime::RawPtr,
    pfnrasfreebuffer: ::windows::runtime::RawPtr,
    pfnrassendbuffer: ::windows::runtime::RawPtr,
    pfnrasreceivebuffer: ::windows::runtime::RawPtr,
    pfnrasretrievebuffer: ::windows::runtime::RawPtr,
    hwnd: super::super::Foundation::HWND,
    prasdialparams: *mut RASDIALPARAMSA,
    pvreserved: *mut ::std::ffi::c_void,
) -> u32;
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn RasDeleteEntryA<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PSTR>, Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::PSTR>>(param0: Param0, param1: Param1) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn RasDeleteEntryA(param0: super::super::Foundation::PSTR, param1: super::super::Foundation::PSTR) -> u32;
        }
        ::std::mem::transmute(RasDeleteEntryA(param0.into_param().abi(), param1.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn RasDeleteEntryW<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>, Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>>(param0: Param0, param1: Param1) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn RasDeleteEntryW(param0: super::super::Foundation::PWSTR, param1: super::super::Foundation::PWSTR) -> u32;
        }
        ::std::mem::transmute(RasDeleteEntryW(param0.into_param().abi(), param1.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn RasDeleteSubEntryA<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PSTR>, Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::PSTR>>(pszphonebook: Param0, pszentry: Param1, dwsubentryid: u32) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn RasDeleteSubEntryA(pszphonebook: super::super::Foundation::PSTR, pszentry: super::super::Foundation::PSTR, dwsubentryid: u32) -> u32;
        }
        ::std::mem::transmute(RasDeleteSubEntryA(pszphonebook.into_param().abi(), pszentry.into_param().abi(), ::std::mem::transmute(dwsubentryid)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn RasDeleteSubEntryW<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>, Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>>(pszphonebook: Param0, pszentry: Param1, dwsubentryid: u32) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn RasDeleteSubEntryW(pszphonebook: super::super::Foundation::PWSTR, pszentry: super::super::Foundation::PWSTR, dwsubentryid: u32) -> u32;
        }
        ::std::mem::transmute(RasDeleteSubEntryW(pszphonebook.into_param().abi(), pszentry.into_param().abi(), ::std::mem::transmute(dwsubentryid)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn RasDialA<'a, Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::PSTR>>(param0: *const RASDIALEXTENSIONS, param1: Param1, param2: *const RASDIALPARAMSA, param3: u32, param4: *const ::std::ffi::c_void, param5: *mut HRASCONN) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn RasDialA(param0: *const RASDIALEXTENSIONS, param1: super::super::Foundation::PSTR, param2: *const RASDIALPARAMSA, param3: u32, param4: *const ::std::ffi::c_void, param5: *mut HRASCONN) -> u32;
        }
        ::std::mem::transmute(RasDialA(::std::mem::transmute(param0), param1.into_param().abi(), ::std::mem::transmute(param2), ::std::mem::transmute(param3), ::std::mem::transmute(param4), ::std::mem::transmute(param5)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn RasDialDlgA<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PSTR>, Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::PSTR>, Param2: ::windows::runtime::IntoParam<'a, super::super::Foundation::PSTR>>(lpszphonebook: Param0, lpszentry: Param1, lpszphonenumber: Param2, lpinfo: *mut RASDIALDLG) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn RasDialDlgA(lpszphonebook: super::super::Foundation::PSTR, lpszentry: super::super::Foundation::PSTR, lpszphonenumber: super::super::Foundation::PSTR, lpinfo: *mut RASDIALDLG) -> super::super::Foundation::BOOL;
        }
        ::std::mem::transmute(RasDialDlgA(lpszphonebook.into_param().abi(), lpszentry.into_param().abi(), lpszphonenumber.into_param().abi(), ::std::mem::transmute(lpinfo)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn RasDialDlgW<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>, Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>, Param2: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>>(lpszphonebook: Param0, lpszentry: Param1, lpszphonenumber: Param2, lpinfo: *mut RASDIALDLG) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn RasDialDlgW(lpszphonebook: super::super::Foundation::PWSTR, lpszentry: super::super::Foundation::PWSTR, lpszphonenumber: super::super::Foundation::PWSTR, lpinfo: *mut RASDIALDLG) -> super::super::Foundation::BOOL;
        }
        ::std::mem::transmute(RasDialDlgW(lpszphonebook.into_param().abi(), lpszentry.into_param().abi(), lpszphonenumber.into_param().abi(), ::std::mem::transmute(lpinfo)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn RasDialW<'a, Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>>(param0: *const RASDIALEXTENSIONS, param1: Param1, param2: *const RASDIALPARAMSW, param3: u32, param4: *const ::std::ffi::c_void, param5: *mut HRASCONN) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn RasDialW(param0: *const RASDIALEXTENSIONS, param1: super::super::Foundation::PWSTR, param2: *const RASDIALPARAMSW, param3: u32, param4: *const ::std::ffi::c_void, param5: *mut HRASCONN) -> u32;
        }
        ::std::mem::transmute(RasDialW(::std::mem::transmute(param0), param1.into_param().abi(), ::std::mem::transmute(param2), ::std::mem::transmute(param3), ::std::mem::transmute(param4), ::std::mem::transmute(param5)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn RasEditPhonebookEntryA<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::HWND>, Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::PSTR>, Param2: ::windows::runtime::IntoParam<'a, super::super::Foundation::PSTR>>(param0: Param0, param1: Param1, param2: Param2) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn RasEditPhonebookEntryA(param0: super::super::Foundation::HWND, param1: super::super::Foundation::PSTR, param2: super::super::Foundation::PSTR) -> u32;
        }
        ::std::mem::transmute(RasEditPhonebookEntryA(param0.into_param().abi(), param1.into_param().abi(), param2.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn RasEditPhonebookEntryW<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::HWND>, Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>, Param2: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>>(param0: Param0, param1: Param1, param2: Param2) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn RasEditPhonebookEntryW(param0: super::super::Foundation::HWND, param1: super::super::Foundation::PWSTR, param2: super::super::Foundation::PWSTR) -> u32;
        }
        ::std::mem::transmute(RasEditPhonebookEntryW(param0.into_param().abi(), param1.into_param().abi(), param2.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn RasEntryDlgA<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PSTR>, Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::PSTR>>(lpszphonebook: Param0, lpszentry: Param1, lpinfo: *mut RASENTRYDLGA) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn RasEntryDlgA(lpszphonebook: super::super::Foundation::PSTR, lpszentry: super::super::Foundation::PSTR, lpinfo: *mut RASENTRYDLGA) -> super::super::Foundation::BOOL;
        }
        ::std::mem::transmute(RasEntryDlgA(lpszphonebook.into_param().abi(), lpszentry.into_param().abi(), ::std::mem::transmute(lpinfo)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn RasEntryDlgW<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>, Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>>(lpszphonebook: Param0, lpszentry: Param1, lpinfo: *mut RASENTRYDLGW) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn RasEntryDlgW(lpszphonebook: super::super::Foundation::PWSTR, lpszentry: super::super::Foundation::PWSTR, lpinfo: *mut RASENTRYDLGW) -> super::super::Foundation::BOOL;
        }
        ::std::mem::transmute(RasEntryDlgW(lpszphonebook.into_param().abi(), lpszentry.into_param().abi(), ::std::mem::transmute(lpinfo)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn RasEnumAutodialAddressesA(lpprasautodialaddresses: *mut super::super::Foundation::PSTR, lpdwcbrasautodialaddresses: *mut u32, lpdwcrasautodialaddresses: *mut u32) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn RasEnumAutodialAddressesA(lpprasautodialaddresses: *mut super::super::Foundation::PSTR, lpdwcbrasautodialaddresses: *mut u32, lpdwcrasautodialaddresses: *mut u32) -> u32;
        }
        ::std::mem::transmute(RasEnumAutodialAddressesA(::std::mem::transmute(lpprasautodialaddresses), ::std::mem::transmute(lpdwcbrasautodialaddresses), ::std::mem::transmute(lpdwcrasautodialaddresses)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn RasEnumAutodialAddressesW(lpprasautodialaddresses: *mut super::super::Foundation::PWSTR, lpdwcbrasautodialaddresses: *mut u32, lpdwcrasautodialaddresses: *mut u32) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn RasEnumAutodialAddressesW(lpprasautodialaddresses: *mut super::super::Foundation::PWSTR, lpdwcbrasautodialaddresses: *mut u32, lpdwcrasautodialaddresses: *mut u32) -> u32;
        }
        ::std::mem::transmute(RasEnumAutodialAddressesW(::std::mem::transmute(lpprasautodialaddresses), ::std::mem::transmute(lpdwcbrasautodialaddresses), ::std::mem::transmute(lpdwcrasautodialaddresses)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn RasEnumConnectionsA(param0: *mut RASCONNA, param1: *mut u32, param2: *mut u32) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn RasEnumConnectionsA(param0: *mut RASCONNA, param1: *mut u32, param2: *mut u32) -> u32;
        }
        ::std::mem::transmute(RasEnumConnectionsA(::std::mem::transmute(param0), ::std::mem::transmute(param1), ::std::mem::transmute(param2)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn RasEnumConnectionsW(param0: *mut RASCONNW, param1: *mut u32, param2: *mut u32) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn RasEnumConnectionsW(param0: *mut RASCONNW, param1: *mut u32, param2: *mut u32) -> u32;
        }
        ::std::mem::transmute(RasEnumConnectionsW(::std::mem::transmute(param0), ::std::mem::transmute(param1), ::std::mem::transmute(param2)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn RasEnumDevicesA(param0: *mut RASDEVINFOA, param1: *mut u32, param2: *mut u32) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn RasEnumDevicesA(param0: *mut RASDEVINFOA, param1: *mut u32, param2: *mut u32) -> u32;
        }
        ::std::mem::transmute(RasEnumDevicesA(::std::mem::transmute(param0), ::std::mem::transmute(param1), ::std::mem::transmute(param2)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn RasEnumDevicesW(param0: *mut RASDEVINFOW, param1: *mut u32, param2: *mut u32) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn RasEnumDevicesW(param0: *mut RASDEVINFOW, param1: *mut u32, param2: *mut u32) -> u32;
        }
        ::std::mem::transmute(RasEnumDevicesW(::std::mem::transmute(param0), ::std::mem::transmute(param1), ::std::mem::transmute(param2)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn RasEnumEntriesA<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PSTR>, Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::PSTR>>(param0: Param0, param1: Param1, param2: *mut RASENTRYNAMEA, param3: *mut u32, param4: *mut u32) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn RasEnumEntriesA(param0: super::super::Foundation::PSTR, param1: super::super::Foundation::PSTR, param2: *mut RASENTRYNAMEA, param3: *mut u32, param4: *mut u32) -> u32;
        }
        ::std::mem::transmute(RasEnumEntriesA(param0.into_param().abi(), param1.into_param().abi(), ::std::mem::transmute(param2), ::std::mem::transmute(param3), ::std::mem::transmute(param4)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn RasEnumEntriesW<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>, Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>>(param0: Param0, param1: Param1, param2: *mut RASENTRYNAMEW, param3: *mut u32, param4: *mut u32) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn RasEnumEntriesW(param0: super::super::Foundation::PWSTR, param1: super::super::Foundation::PWSTR, param2: *mut RASENTRYNAMEW, param3: *mut u32, param4: *mut u32) -> u32;
        }
        ::std::mem::transmute(RasEnumEntriesW(param0.into_param().abi(), param1.into_param().abi(), ::std::mem::transmute(param2), ::std::mem::transmute(param3), ::std::mem::transmute(param4)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn RasFreeEapUserIdentityA(praseapuseridentity: *const RASEAPUSERIDENTITYA) {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn RasFreeEapUserIdentityA(praseapuseridentity: *const RASEAPUSERIDENTITYA);
        }
        ::std::mem::transmute(RasFreeEapUserIdentityA(::std::mem::transmute(praseapuseridentity)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn RasFreeEapUserIdentityW(praseapuseridentity: *const RASEAPUSERIDENTITYW) {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn RasFreeEapUserIdentityW(praseapuseridentity: *const RASEAPUSERIDENTITYW);
        }
        ::std::mem::transmute(RasFreeEapUserIdentityW(::std::mem::transmute(praseapuseridentity)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn RasGetAutodialAddressA<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PSTR>>(param0: Param0, param1: *const u32, param2: *mut RASAUTODIALENTRYA, param3: *mut u32, param4: *mut u32) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn RasGetAutodialAddressA(param0: super::super::Foundation::PSTR, param1: *const u32, param2: *mut RASAUTODIALENTRYA, param3: *mut u32, param4: *mut u32) -> u32;
        }
        ::std::mem::transmute(RasGetAutodialAddressA(param0.into_param().abi(), ::std::mem::transmute(param1), ::std::mem::transmute(param2), ::std::mem::transmute(param3), ::std::mem::transmute(param4)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn RasGetAutodialAddressW<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>>(param0: Param0, param1: *const u32, param2: *mut RASAUTODIALENTRYW, param3: *mut u32, param4: *mut u32) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn RasGetAutodialAddressW(param0: super::super::Foundation::PWSTR, param1: *const u32, param2: *mut RASAUTODIALENTRYW, param3: *mut u32, param4: *mut u32) -> u32;
        }
        ::std::mem::transmute(RasGetAutodialAddressW(param0.into_param().abi(), ::std::mem::transmute(param1), ::std::mem::transmute(param2), ::std::mem::transmute(param3), ::std::mem::transmute(param4)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn RasGetAutodialEnableA(param0: u32, param1: *mut i32) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn RasGetAutodialEnableA(param0: u32, param1: *mut i32) -> u32;
        }
        ::std::mem::transmute(RasGetAutodialEnableA(::std::mem::transmute(param0), ::std::mem::transmute(param1)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn RasGetAutodialEnableW(param0: u32, param1: *mut i32) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn RasGetAutodialEnableW(param0: u32, param1: *mut i32) -> u32;
        }
        ::std::mem::transmute(RasGetAutodialEnableW(::std::mem::transmute(param0), ::std::mem::transmute(param1)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn RasGetAutodialParamA(param0: u32, param1: *mut ::std::ffi::c_void, param2: *mut u32) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn RasGetAutodialParamA(param0: u32, param1: *mut ::std::ffi::c_void, param2: *mut u32) -> u32;
        }
        ::std::mem::transmute(RasGetAutodialParamA(::std::mem::transmute(param0), ::std::mem::transmute(param1), ::std::mem::transmute(param2)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn RasGetAutodialParamW(param0: u32, param1: *mut ::std::ffi::c_void, param2: *mut u32) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn RasGetAutodialParamW(param0: u32, param1: *mut ::std::ffi::c_void, param2: *mut u32) -> u32;
        }
        ::std::mem::transmute(RasGetAutodialParamW(::std::mem::transmute(param0), ::std::mem::transmute(param1), ::std::mem::transmute(param2)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
#[inline]
pub unsafe fn RasGetConnectStatusA<'a, Param0: ::windows::runtime::IntoParam<'a, HRASCONN>>(param0: Param0, param1: *mut RASCONNSTATUSA) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn RasGetConnectStatusA(param0: HRASCONN, param1: *mut RASCONNSTATUSA) -> u32;
        }
        ::std::mem::transmute(RasGetConnectStatusA(param0.into_param().abi(), ::std::mem::transmute(param1)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Networking_WinSock")]
#[inline]
pub unsafe fn RasGetConnectStatusW<'a, Param0: ::windows::runtime::IntoParam<'a, HRASCONN>>(param0: Param0, param1: *mut RASCONNSTATUSW) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn RasGetConnectStatusW(param0: HRASCONN, param1: *mut RASCONNSTATUSW) -> u32;
        }
        ::std::mem::transmute(RasGetConnectStatusW(param0.into_param().abi(), ::std::mem::transmute(param1)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn RasGetConnectionStatistics<'a, Param0: ::windows::runtime::IntoParam<'a, HRASCONN>>(hrasconn: Param0, lpstatistics: *mut RAS_STATS) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn RasGetConnectionStatistics(hrasconn: HRASCONN, lpstatistics: *mut RAS_STATS) -> u32;
        }
        ::std::mem::transmute(RasGetConnectionStatistics(hrasconn.into_param().abi(), ::std::mem::transmute(lpstatistics)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn RasGetCountryInfoA(param0: *mut RASCTRYINFO, param1: *mut u32) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn RasGetCountryInfoA(param0: *mut RASCTRYINFO, param1: *mut u32) -> u32;
        }
        ::std::mem::transmute(RasGetCountryInfoA(::std::mem::transmute(param0), ::std::mem::transmute(param1)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn RasGetCountryInfoW(param0: *mut RASCTRYINFO, param1: *mut u32) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn RasGetCountryInfoW(param0: *mut RASCTRYINFO, param1: *mut u32) -> u32;
        }
        ::std::mem::transmute(RasGetCountryInfoW(::std::mem::transmute(param0), ::std::mem::transmute(param1)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn RasGetCredentialsA<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PSTR>, Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::PSTR>>(param0: Param0, param1: Param1, param2: *mut RASCREDENTIALSA) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn RasGetCredentialsA(param0: super::super::Foundation::PSTR, param1: super::super::Foundation::PSTR, param2: *mut RASCREDENTIALSA) -> u32;
        }
        ::std::mem::transmute(RasGetCredentialsA(param0.into_param().abi(), param1.into_param().abi(), ::std::mem::transmute(param2)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn RasGetCredentialsW<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>, Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>>(param0: Param0, param1: Param1, param2: *mut RASCREDENTIALSW) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn RasGetCredentialsW(param0: super::super::Foundation::PWSTR, param1: super::super::Foundation::PWSTR, param2: *mut RASCREDENTIALSW) -> u32;
        }
        ::std::mem::transmute(RasGetCredentialsW(param0.into_param().abi(), param1.into_param().abi(), ::std::mem::transmute(param2)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn RasGetCustomAuthDataA<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PSTR>, Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::PSTR>>(pszphonebook: Param0, pszentry: Param1, pbcustomauthdata: *mut u8, pdwsizeofcustomauthdata: *mut u32) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn RasGetCustomAuthDataA(pszphonebook: super::super::Foundation::PSTR, pszentry: super::super::Foundation::PSTR, pbcustomauthdata: *mut u8, pdwsizeofcustomauthdata: *mut u32) -> u32;
        }
        ::std::mem::transmute(RasGetCustomAuthDataA(pszphonebook.into_param().abi(), pszentry.into_param().abi(), ::std::mem::transmute(pbcustomauthdata), ::std::mem::transmute(pdwsizeofcustomauthdata)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn RasGetCustomAuthDataW<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>, Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>>(pszphonebook: Param0, pszentry: Param1, pbcustomauthdata: *mut u8, pdwsizeofcustomauthdata: *mut u32) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn RasGetCustomAuthDataW(pszphonebook: super::super::Foundation::PWSTR, pszentry: super::super::Foundation::PWSTR, pbcustomauthdata: *mut u8, pdwsizeofcustomauthdata: *mut u32) -> u32;
        }
        ::std::mem::transmute(RasGetCustomAuthDataW(pszphonebook.into_param().abi(), pszentry.into_param().abi(), ::std::mem::transmute(pbcustomauthdata), ::std::mem::transmute(pdwsizeofcustomauthdata)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn RasGetEapUserDataA<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::HANDLE>, Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::PSTR>, Param2: ::windows::runtime::IntoParam<'a, super::super::Foundation::PSTR>>(htoken: Param0, pszphonebook: Param1, pszentry: Param2, pbeapdata: *mut u8, pdwsizeofeapdata: *mut u32) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn RasGetEapUserDataA(htoken: super::super::Foundation::HANDLE, pszphonebook: super::super::Foundation::PSTR, pszentry: super::super::Foundation::PSTR, pbeapdata: *mut u8, pdwsizeofeapdata: *mut u32) -> u32;
        }
        ::std::mem::transmute(RasGetEapUserDataA(htoken.into_param().abi(), pszphonebook.into_param().abi(), pszentry.into_param().abi(), ::std::mem::transmute(pbeapdata), ::std::mem::transmute(pdwsizeofeapdata)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn RasGetEapUserDataW<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::HANDLE>, Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>, Param2: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>>(htoken: Param0, pszphonebook: Param1, pszentry: Param2, pbeapdata: *mut u8, pdwsizeofeapdata: *mut u32) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn RasGetEapUserDataW(htoken: super::super::Foundation::HANDLE, pszphonebook: super::super::Foundation::PWSTR, pszentry: super::super::Foundation::PWSTR, pbeapdata: *mut u8, pdwsizeofeapdata: *mut u32) -> u32;
        }
        ::std::mem::transmute(RasGetEapUserDataW(htoken.into_param().abi(), pszphonebook.into_param().abi(), pszentry.into_param().abi(), ::std::mem::transmute(pbeapdata), ::std::mem::transmute(pdwsizeofeapdata)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn RasGetEapUserIdentityA<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PSTR>, Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::PSTR>, Param3: ::windows::runtime::IntoParam<'a, super::super::Foundation::HWND>>(pszphonebook: Param0, pszentry: Param1, dwflags: u32, hwnd: Param3, ppraseapuseridentity: *mut *mut RASEAPUSERIDENTITYA) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn RasGetEapUserIdentityA(pszphonebook: super::super::Foundation::PSTR, pszentry: super::super::Foundation::PSTR, dwflags: u32, hwnd: super::super::Foundation::HWND, ppraseapuseridentity: *mut *mut RASEAPUSERIDENTITYA) -> u32;
        }
        ::std::mem::transmute(RasGetEapUserIdentityA(pszphonebook.into_param().abi(), pszentry.into_param().abi(), ::std::mem::transmute(dwflags), hwnd.into_param().abi(), ::std::mem::transmute(ppraseapuseridentity)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn RasGetEapUserIdentityW<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>, Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>, Param3: ::windows::runtime::IntoParam<'a, super::super::Foundation::HWND>>(pszphonebook: Param0, pszentry: Param1, dwflags: u32, hwnd: Param3, ppraseapuseridentity: *mut *mut RASEAPUSERIDENTITYW) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn RasGetEapUserIdentityW(pszphonebook: super::super::Foundation::PWSTR, pszentry: super::super::Foundation::PWSTR, dwflags: u32, hwnd: super::super::Foundation::HWND, ppraseapuseridentity: *mut *mut RASEAPUSERIDENTITYW) -> u32;
        }
        ::std::mem::transmute(RasGetEapUserIdentityW(pszphonebook.into_param().abi(), pszentry.into_param().abi(), ::std::mem::transmute(dwflags), hwnd.into_param().abi(), ::std::mem::transmute(ppraseapuseridentity)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn RasGetEntryDialParamsA<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PSTR>>(param0: Param0, param1: *mut RASDIALPARAMSA, param2: *mut i32) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn RasGetEntryDialParamsA(param0: super::super::Foundation::PSTR, param1: *mut RASDIALPARAMSA, param2: *mut i32) -> u32;
        }
        ::std::mem::transmute(RasGetEntryDialParamsA(param0.into_param().abi(), ::std::mem::transmute(param1), ::std::mem::transmute(param2)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn RasGetEntryDialParamsW<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>>(param0: Param0, param1: *mut RASDIALPARAMSW, param2: *mut i32) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn RasGetEntryDialParamsW(param0: super::super::Foundation::PWSTR, param1: *mut RASDIALPARAMSW, param2: *mut i32) -> u32;
        }
        ::std::mem::transmute(RasGetEntryDialParamsW(param0.into_param().abi(), ::std::mem::transmute(param1), ::std::mem::transmute(param2)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
#[inline]
pub unsafe fn RasGetEntryPropertiesA<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PSTR>, Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::PSTR>>(param0: Param0, param1: Param1, param2: *mut RASENTRYA, param3: *mut u32, param4: *mut u8, param5: *mut u32) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn RasGetEntryPropertiesA(param0: super::super::Foundation::PSTR, param1: super::super::Foundation::PSTR, param2: *mut RASENTRYA, param3: *mut u32, param4: *mut u8, param5: *mut u32) -> u32;
        }
        ::std::mem::transmute(RasGetEntryPropertiesA(param0.into_param().abi(), param1.into_param().abi(), ::std::mem::transmute(param2), ::std::mem::transmute(param3), ::std::mem::transmute(param4), ::std::mem::transmute(param5)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
#[inline]
pub unsafe fn RasGetEntryPropertiesW<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>, Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>>(param0: Param0, param1: Param1, param2: *mut RASENTRYW, param3: *mut u32, param4: *mut u8, param5: *mut u32) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn RasGetEntryPropertiesW(param0: super::super::Foundation::PWSTR, param1: super::super::Foundation::PWSTR, param2: *mut RASENTRYW, param3: *mut u32, param4: *mut u8, param5: *mut u32) -> u32;
        }
        ::std::mem::transmute(RasGetEntryPropertiesW(param0.into_param().abi(), param1.into_param().abi(), ::std::mem::transmute(param2), ::std::mem::transmute(param3), ::std::mem::transmute(param4), ::std::mem::transmute(param5)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn RasGetErrorStringA(resourceid: u32, lpszstring: super::super::Foundation::PSTR, inbufsize: u32) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn RasGetErrorStringA(resourceid: u32, lpszstring: super::super::Foundation::PSTR, inbufsize: u32) -> u32;
        }
        ::std::mem::transmute(RasGetErrorStringA(::std::mem::transmute(resourceid), ::std::mem::transmute(lpszstring), ::std::mem::transmute(inbufsize)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn RasGetErrorStringW(resourceid: u32, lpszstring: super::super::Foundation::PWSTR, inbufsize: u32) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn RasGetErrorStringW(resourceid: u32, lpszstring: super::super::Foundation::PWSTR, inbufsize: u32) -> u32;
        }
        ::std::mem::transmute(RasGetErrorStringW(::std::mem::transmute(resourceid), ::std::mem::transmute(lpszstring), ::std::mem::transmute(inbufsize)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn RasGetLinkStatistics<'a, Param0: ::windows::runtime::IntoParam<'a, HRASCONN>>(hrasconn: Param0, dwsubentry: u32, lpstatistics: *mut RAS_STATS) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn RasGetLinkStatistics(hrasconn: HRASCONN, dwsubentry: u32, lpstatistics: *mut RAS_STATS) -> u32;
        }
        ::std::mem::transmute(RasGetLinkStatistics(hrasconn.into_param().abi(), ::std::mem::transmute(dwsubentry), ::std::mem::transmute(lpstatistics)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn RasGetPCscf(lpszpcscf: super::super::Foundation::PWSTR) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn RasGetPCscf(lpszpcscf: super::super::Foundation::PWSTR) -> u32;
        }
        ::std::mem::transmute(RasGetPCscf(::std::mem::transmute(lpszpcscf)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn RasGetProjectionInfoA<'a, Param0: ::windows::runtime::IntoParam<'a, HRASCONN>>(param0: Param0, param1: RASPROJECTION, param2: *mut ::std::ffi::c_void, param3: *mut u32) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn RasGetProjectionInfoA(param0: HRASCONN, param1: RASPROJECTION, param2: *mut ::std::ffi::c_void, param3: *mut u32) -> u32;
        }
        ::std::mem::transmute(RasGetProjectionInfoA(param0.into_param().abi(), ::std::mem::transmute(param1), ::std::mem::transmute(param2), ::std::mem::transmute(param3)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
#[inline]
pub unsafe fn RasGetProjectionInfoEx<'a, Param0: ::windows::runtime::IntoParam<'a, HRASCONN>>(hrasconn: Param0, prasprojection: *mut RAS_PROJECTION_INFO, lpdwsize: *mut u32) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn RasGetProjectionInfoEx(hrasconn: HRASCONN, prasprojection: *mut RAS_PROJECTION_INFO, lpdwsize: *mut u32) -> u32;
        }
        ::std::mem::transmute(RasGetProjectionInfoEx(hrasconn.into_param().abi(), ::std::mem::transmute(prasprojection), ::std::mem::transmute(lpdwsize)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn RasGetProjectionInfoW<'a, Param0: ::windows::runtime::IntoParam<'a, HRASCONN>>(param0: Param0, param1: RASPROJECTION, param2: *mut ::std::ffi::c_void, param3: *mut u32) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn RasGetProjectionInfoW(param0: HRASCONN, param1: RASPROJECTION, param2: *mut ::std::ffi::c_void, param3: *mut u32) -> u32;
        }
        ::std::mem::transmute(RasGetProjectionInfoW(param0.into_param().abi(), ::std::mem::transmute(param1), ::std::mem::transmute(param2), ::std::mem::transmute(param3)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn RasGetSubEntryHandleA<'a, Param0: ::windows::runtime::IntoParam<'a, HRASCONN>>(param0: Param0, param1: u32, param2: *mut HRASCONN) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn RasGetSubEntryHandleA(param0: HRASCONN, param1: u32, param2: *mut HRASCONN) -> u32;
        }
        ::std::mem::transmute(RasGetSubEntryHandleA(param0.into_param().abi(), ::std::mem::transmute(param1), ::std::mem::transmute(param2)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn RasGetSubEntryHandleW<'a, Param0: ::windows::runtime::IntoParam<'a, HRASCONN>>(param0: Param0, param1: u32, param2: *mut HRASCONN) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn RasGetSubEntryHandleW(param0: HRASCONN, param1: u32, param2: *mut HRASCONN) -> u32;
        }
        ::std::mem::transmute(RasGetSubEntryHandleW(param0.into_param().abi(), ::std::mem::transmute(param1), ::std::mem::transmute(param2)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn RasGetSubEntryPropertiesA<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PSTR>, Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::PSTR>>(param0: Param0, param1: Param1, param2: u32, param3: *mut RASSUBENTRYA, param4: *mut u32, param5: *mut u8, param6: *mut u32) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn RasGetSubEntryPropertiesA(param0: super::super::Foundation::PSTR, param1: super::super::Foundation::PSTR, param2: u32, param3: *mut RASSUBENTRYA, param4: *mut u32, param5: *mut u8, param6: *mut u32) -> u32;
        }
        ::std::mem::transmute(RasGetSubEntryPropertiesA(param0.into_param().abi(), param1.into_param().abi(), ::std::mem::transmute(param2), ::std::mem::transmute(param3), ::std::mem::transmute(param4), ::std::mem::transmute(param5), ::std::mem::transmute(param6)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn RasGetSubEntryPropertiesW<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>, Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>>(param0: Param0, param1: Param1, param2: u32, param3: *mut RASSUBENTRYW, param4: *mut u32, param5: *mut u8, param6: *mut u32) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn RasGetSubEntryPropertiesW(param0: super::super::Foundation::PWSTR, param1: super::super::Foundation::PWSTR, param2: u32, param3: *mut RASSUBENTRYW, param4: *mut u32, param5: *mut u8, param6: *mut u32) -> u32;
        }
        ::std::mem::transmute(RasGetSubEntryPropertiesW(param0.into_param().abi(), param1.into_param().abi(), ::std::mem::transmute(param2), ::std::mem::transmute(param3), ::std::mem::transmute(param4), ::std::mem::transmute(param5), ::std::mem::transmute(param6)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn RasHangUpA<'a, Param0: ::windows::runtime::IntoParam<'a, HRASCONN>>(param0: Param0) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn RasHangUpA(param0: HRASCONN) -> u32;
        }
        ::std::mem::transmute(RasHangUpA(param0.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn RasHangUpW<'a, Param0: ::windows::runtime::IntoParam<'a, HRASCONN>>(param0: Param0) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn RasHangUpW(param0: HRASCONN) -> u32;
        }
        ::std::mem::transmute(RasHangUpW(param0.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn RasInvokeEapUI<'a, Param0: ::windows::runtime::IntoParam<'a, HRASCONN>, Param3: ::windows::runtime::IntoParam<'a, super::super::Foundation::HWND>>(param0: Param0, param1: u32, param2: *const RASDIALEXTENSIONS, param3: Param3) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn RasInvokeEapUI(param0: HRASCONN, param1: u32, param2: *const RASDIALEXTENSIONS, param3: super::super::Foundation::HWND) -> u32;
        }
        ::std::mem::transmute(RasInvokeEapUI(param0.into_param().abi(), ::std::mem::transmute(param1), ::std::mem::transmute(param2), param3.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn RasPhonebookDlgA<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PSTR>, Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::PSTR>>(lpszphonebook: Param0, lpszentry: Param1, lpinfo: *mut RASPBDLGA) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn RasPhonebookDlgA(lpszphonebook: super::super::Foundation::PSTR, lpszentry: super::super::Foundation::PSTR, lpinfo: *mut ::std::mem::ManuallyDrop<RASPBDLGA>) -> super::super::Foundation::BOOL;
        }
        ::std::mem::transmute(RasPhonebookDlgA(lpszphonebook.into_param().abi(), lpszentry.into_param().abi(), ::std::mem::transmute(lpinfo)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn RasPhonebookDlgW<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>, Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>>(lpszphonebook: Param0, lpszentry: Param1, lpinfo: *mut RASPBDLGW) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn RasPhonebookDlgW(lpszphonebook: super::super::Foundation::PWSTR, lpszentry: super::super::Foundation::PWSTR, lpinfo: *mut ::std::mem::ManuallyDrop<RASPBDLGW>) -> super::super::Foundation::BOOL;
        }
        ::std::mem::transmute(RasPhonebookDlgW(lpszphonebook.into_param().abi(), lpszentry.into_param().abi(), ::std::mem::transmute(lpinfo)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn RasRenameEntryA<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PSTR>, Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::PSTR>, Param2: ::windows::runtime::IntoParam<'a, super::super::Foundation::PSTR>>(param0: Param0, param1: Param1, param2: Param2) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn RasRenameEntryA(param0: super::super::Foundation::PSTR, param1: super::super::Foundation::PSTR, param2: super::super::Foundation::PSTR) -> u32;
        }
        ::std::mem::transmute(RasRenameEntryA(param0.into_param().abi(), param1.into_param().abi(), param2.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn RasRenameEntryW<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>, Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>, Param2: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>>(param0: Param0, param1: Param1, param2: Param2) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn RasRenameEntryW(param0: super::super::Foundation::PWSTR, param1: super::super::Foundation::PWSTR, param2: super::super::Foundation::PWSTR) -> u32;
        }
        ::std::mem::transmute(RasRenameEntryW(param0.into_param().abi(), param1.into_param().abi(), param2.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn RasSetAutodialAddressA<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PSTR>>(param0: Param0, param1: u32, param2: *const RASAUTODIALENTRYA, param3: u32, param4: u32) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn RasSetAutodialAddressA(param0: super::super::Foundation::PSTR, param1: u32, param2: *const RASAUTODIALENTRYA, param3: u32, param4: u32) -> u32;
        }
        ::std::mem::transmute(RasSetAutodialAddressA(param0.into_param().abi(), ::std::mem::transmute(param1), ::std::mem::transmute(param2), ::std::mem::transmute(param3), ::std::mem::transmute(param4)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn RasSetAutodialAddressW<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>>(param0: Param0, param1: u32, param2: *const RASAUTODIALENTRYW, param3: u32, param4: u32) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn RasSetAutodialAddressW(param0: super::super::Foundation::PWSTR, param1: u32, param2: *const RASAUTODIALENTRYW, param3: u32, param4: u32) -> u32;
        }
        ::std::mem::transmute(RasSetAutodialAddressW(param0.into_param().abi(), ::std::mem::transmute(param1), ::std::mem::transmute(param2), ::std::mem::transmute(param3), ::std::mem::transmute(param4)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn RasSetAutodialEnableA<'a, Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::BOOL>>(param0: u32, param1: Param1) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn RasSetAutodialEnableA(param0: u32, param1: super::super::Foundation::BOOL) -> u32;
        }
        ::std::mem::transmute(RasSetAutodialEnableA(::std::mem::transmute(param0), param1.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn RasSetAutodialEnableW<'a, Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::BOOL>>(param0: u32, param1: Param1) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn RasSetAutodialEnableW(param0: u32, param1: super::super::Foundation::BOOL) -> u32;
        }
        ::std::mem::transmute(RasSetAutodialEnableW(::std::mem::transmute(param0), param1.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn RasSetAutodialParamA(param0: u32, param1: *const ::std::ffi::c_void, param2: u32) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn RasSetAutodialParamA(param0: u32, param1: *const ::std::ffi::c_void, param2: u32) -> u32;
        }
        ::std::mem::transmute(RasSetAutodialParamA(::std::mem::transmute(param0), ::std::mem::transmute(param1), ::std::mem::transmute(param2)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn RasSetAutodialParamW(param0: u32, param1: *const ::std::ffi::c_void, param2: u32) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn RasSetAutodialParamW(param0: u32, param1: *const ::std::ffi::c_void, param2: u32) -> u32;
        }
        ::std::mem::transmute(RasSetAutodialParamW(::std::mem::transmute(param0), ::std::mem::transmute(param1), ::std::mem::transmute(param2)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn RasSetCredentialsA<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PSTR>, Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::PSTR>, Param3: ::windows::runtime::IntoParam<'a, super::super::Foundation::BOOL>>(param0: Param0, param1: Param1, param2: *const RASCREDENTIALSA, param3: Param3) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn RasSetCredentialsA(param0: super::super::Foundation::PSTR, param1: super::super::Foundation::PSTR, param2: *const RASCREDENTIALSA, param3: super::super::Foundation::BOOL) -> u32;
        }
        ::std::mem::transmute(RasSetCredentialsA(param0.into_param().abi(), param1.into_param().abi(), ::std::mem::transmute(param2), param3.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn RasSetCredentialsW<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>, Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>, Param3: ::windows::runtime::IntoParam<'a, super::super::Foundation::BOOL>>(param0: Param0, param1: Param1, param2: *const RASCREDENTIALSW, param3: Param3) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn RasSetCredentialsW(param0: super::super::Foundation::PWSTR, param1: super::super::Foundation::PWSTR, param2: *const RASCREDENTIALSW, param3: super::super::Foundation::BOOL) -> u32;
        }
        ::std::mem::transmute(RasSetCredentialsW(param0.into_param().abi(), param1.into_param().abi(), ::std::mem::transmute(param2), param3.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn RasSetCustomAuthDataA<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PSTR>, Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::PSTR>>(pszphonebook: Param0, pszentry: Param1, pbcustomauthdata: *const u8, dwsizeofcustomauthdata: u32) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn RasSetCustomAuthDataA(pszphonebook: super::super::Foundation::PSTR, pszentry: super::super::Foundation::PSTR, pbcustomauthdata: *const u8, dwsizeofcustomauthdata: u32) -> u32;
        }
        ::std::mem::transmute(RasSetCustomAuthDataA(pszphonebook.into_param().abi(), pszentry.into_param().abi(), ::std::mem::transmute(pbcustomauthdata), ::std::mem::transmute(dwsizeofcustomauthdata)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn RasSetCustomAuthDataW<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>, Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>>(pszphonebook: Param0, pszentry: Param1, pbcustomauthdata: *const u8, dwsizeofcustomauthdata: u32) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn RasSetCustomAuthDataW(pszphonebook: super::super::Foundation::PWSTR, pszentry: super::super::Foundation::PWSTR, pbcustomauthdata: *const u8, dwsizeofcustomauthdata: u32) -> u32;
        }
        ::std::mem::transmute(RasSetCustomAuthDataW(pszphonebook.into_param().abi(), pszentry.into_param().abi(), ::std::mem::transmute(pbcustomauthdata), ::std::mem::transmute(dwsizeofcustomauthdata)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn RasSetEapUserDataA<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::HANDLE>, Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::PSTR>, Param2: ::windows::runtime::IntoParam<'a, super::super::Foundation::PSTR>>(htoken: Param0, pszphonebook: Param1, pszentry: Param2, pbeapdata: *const u8, dwsizeofeapdata: u32) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn RasSetEapUserDataA(htoken: super::super::Foundation::HANDLE, pszphonebook: super::super::Foundation::PSTR, pszentry: super::super::Foundation::PSTR, pbeapdata: *const u8, dwsizeofeapdata: u32) -> u32;
        }
        ::std::mem::transmute(RasSetEapUserDataA(htoken.into_param().abi(), pszphonebook.into_param().abi(), pszentry.into_param().abi(), ::std::mem::transmute(pbeapdata), ::std::mem::transmute(dwsizeofeapdata)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn RasSetEapUserDataW<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::HANDLE>, Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>, Param2: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>>(htoken: Param0, pszphonebook: Param1, pszentry: Param2, pbeapdata: *const u8, dwsizeofeapdata: u32) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn RasSetEapUserDataW(htoken: super::super::Foundation::HANDLE, pszphonebook: super::super::Foundation::PWSTR, pszentry: super::super::Foundation::PWSTR, pbeapdata: *const u8, dwsizeofeapdata: u32) -> u32;
        }
        ::std::mem::transmute(RasSetEapUserDataW(htoken.into_param().abi(), pszphonebook.into_param().abi(), pszentry.into_param().abi(), ::std::mem::transmute(pbeapdata), ::std::mem::transmute(dwsizeofeapdata)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn RasSetEntryDialParamsA<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PSTR>, Param2: ::windows::runtime::IntoParam<'a, super::super::Foundation::BOOL>>(param0: Param0, param1: *const RASDIALPARAMSA, param2: Param2) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn RasSetEntryDialParamsA(param0: super::super::Foundation::PSTR, param1: *const RASDIALPARAMSA, param2: super::super::Foundation::BOOL) -> u32;
        }
        ::std::mem::transmute(RasSetEntryDialParamsA(param0.into_param().abi(), ::std::mem::transmute(param1), param2.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn RasSetEntryDialParamsW<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>, Param2: ::windows::runtime::IntoParam<'a, super::super::Foundation::BOOL>>(param0: Param0, param1: *const RASDIALPARAMSW, param2: Param2) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn RasSetEntryDialParamsW(param0: super::super::Foundation::PWSTR, param1: *const RASDIALPARAMSW, param2: super::super::Foundation::BOOL) -> u32;
        }
        ::std::mem::transmute(RasSetEntryDialParamsW(param0.into_param().abi(), ::std::mem::transmute(param1), param2.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
#[inline]
pub unsafe fn RasSetEntryPropertiesA<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PSTR>, Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::PSTR>>(param0: Param0, param1: Param1, param2: *const RASENTRYA, param3: u32, param4: *const u8, param5: u32) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn RasSetEntryPropertiesA(param0: super::super::Foundation::PSTR, param1: super::super::Foundation::PSTR, param2: *const RASENTRYA, param3: u32, param4: *const u8, param5: u32) -> u32;
        }
        ::std::mem::transmute(RasSetEntryPropertiesA(param0.into_param().abi(), param1.into_param().abi(), ::std::mem::transmute(param2), ::std::mem::transmute(param3), ::std::mem::transmute(param4), ::std::mem::transmute(param5)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
#[inline]
pub unsafe fn RasSetEntryPropertiesW<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>, Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>>(param0: Param0, param1: Param1, param2: *const RASENTRYW, param3: u32, param4: *const u8, param5: u32) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn RasSetEntryPropertiesW(param0: super::super::Foundation::PWSTR, param1: super::super::Foundation::PWSTR, param2: *const RASENTRYW, param3: u32, param4: *const u8, param5: u32) -> u32;
        }
        ::std::mem::transmute(RasSetEntryPropertiesW(param0.into_param().abi(), param1.into_param().abi(), ::std::mem::transmute(param2), ::std::mem::transmute(param3), ::std::mem::transmute(param4), ::std::mem::transmute(param5)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn RasSetSubEntryPropertiesA<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PSTR>, Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::PSTR>>(param0: Param0, param1: Param1, param2: u32, param3: *const RASSUBENTRYA, param4: u32, param5: *const u8, param6: u32) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn RasSetSubEntryPropertiesA(param0: super::super::Foundation::PSTR, param1: super::super::Foundation::PSTR, param2: u32, param3: *const RASSUBENTRYA, param4: u32, param5: *const u8, param6: u32) -> u32;
        }
        ::std::mem::transmute(RasSetSubEntryPropertiesA(param0.into_param().abi(), param1.into_param().abi(), ::std::mem::transmute(param2), ::std::mem::transmute(param3), ::std::mem::transmute(param4), ::std::mem::transmute(param5), ::std::mem::transmute(param6)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn RasSetSubEntryPropertiesW<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>, Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>>(param0: Param0, param1: Param1, param2: u32, param3: *const RASSUBENTRYW, param4: u32, param5: *const u8, param6: u32) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn RasSetSubEntryPropertiesW(param0: super::super::Foundation::PWSTR, param1: super::super::Foundation::PWSTR, param2: u32, param3: *const RASSUBENTRYW, param4: u32, param5: *const u8, param6: u32) -> u32;
        }
        ::std::mem::transmute(RasSetSubEntryPropertiesW(param0.into_param().abi(), param1.into_param().abi(), ::std::mem::transmute(param2), ::std::mem::transmute(param3), ::std::mem::transmute(param4), ::std::mem::transmute(param5), ::std::mem::transmute(param6)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Networking_WinSock")]
#[inline]
pub unsafe fn RasUpdateConnection<'a, Param0: ::windows::runtime::IntoParam<'a, HRASCONN>>(hrasconn: Param0, lprasupdateconn: *const RASUPDATECONN) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn RasUpdateConnection(hrasconn: HRASCONN, lprasupdateconn: *const RASUPDATECONN) -> u32;
        }
        ::std::mem::transmute(RasUpdateConnection(hrasconn.into_param().abi(), ::std::mem::transmute(lprasupdateconn)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn RasValidateEntryNameA<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PSTR>, Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::PSTR>>(param0: Param0, param1: Param1) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn RasValidateEntryNameA(param0: super::super::Foundation::PSTR, param1: super::super::Foundation::PSTR) -> u32;
        }
        ::std::mem::transmute(RasValidateEntryNameA(param0.into_param().abi(), param1.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn RasValidateEntryNameW<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>, Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>>(param0: Param0, param1: Param1) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn RasValidateEntryNameW(param0: super::super::Foundation::PWSTR, param1: super::super::Foundation::PWSTR) -> u32;
        }
        ::std::mem::transmute(RasValidateEntryNameW(param0.into_param().abi(), param1.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn RtmAddNextHop(rtmreghandle: isize, nexthopinfo: *mut RTM_NEXTHOP_INFO, nexthophandle: *mut isize, changeflags: *mut u32) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn RtmAddNextHop(rtmreghandle: isize, nexthopinfo: *mut RTM_NEXTHOP_INFO, nexthophandle: *mut isize, changeflags: *mut u32) -> u32;
        }
        ::std::mem::transmute(RtmAddNextHop(::std::mem::transmute(rtmreghandle), ::std::mem::transmute(nexthopinfo), ::std::mem::transmute(nexthophandle), ::std::mem::transmute(changeflags)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn RtmAddRouteToDest(rtmreghandle: isize, routehandle: *mut isize, destaddress: *mut RTM_NET_ADDRESS, routeinfo: *mut RTM_ROUTE_INFO, timetolive: u32, routelisthandle: isize, notifytype: u32, notifyhandle: isize, changeflags: *mut u32) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn RtmAddRouteToDest(rtmreghandle: isize, routehandle: *mut isize, destaddress: *mut RTM_NET_ADDRESS, routeinfo: *mut RTM_ROUTE_INFO, timetolive: u32, routelisthandle: isize, notifytype: u32, notifyhandle: isize, changeflags: *mut u32) -> u32;
        }
        ::std::mem::transmute(RtmAddRouteToDest(
            ::std::mem::transmute(rtmreghandle),
            ::std::mem::transmute(routehandle),
            ::std::mem::transmute(destaddress),
            ::std::mem::transmute(routeinfo),
            ::std::mem::transmute(timetolive),
            ::std::mem::transmute(routelisthandle),
            ::std::mem::transmute(notifytype),
            ::std::mem::transmute(notifyhandle),
            ::std::mem::transmute(changeflags),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn RtmBlockMethods<'a, Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::HANDLE>>(rtmreghandle: isize, targethandle: Param1, targettype: u8, blockingflag: u32) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn RtmBlockMethods(rtmreghandle: isize, targethandle: super::super::Foundation::HANDLE, targettype: u8, blockingflag: u32) -> u32;
        }
        ::std::mem::transmute(RtmBlockMethods(::std::mem::transmute(rtmreghandle), targethandle.into_param().abi(), ::std::mem::transmute(targettype), ::std::mem::transmute(blockingflag)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Networking_WinSock")]
#[inline]
pub unsafe fn RtmConvertIpv6AddressAndLengthToNetAddress<'a, Param1: ::windows::runtime::IntoParam<'a, super::super::Networking::WinSock::IN6_ADDR>>(pnetaddress: *mut RTM_NET_ADDRESS, address: Param1, dwlength: u32, dwaddresssize: u32) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn RtmConvertIpv6AddressAndLengthToNetAddress(pnetaddress: *mut RTM_NET_ADDRESS, address: super::super::Networking::WinSock::IN6_ADDR, dwlength: u32, dwaddresssize: u32) -> u32;
        }
        ::std::mem::transmute(RtmConvertIpv6AddressAndLengthToNetAddress(::std::mem::transmute(pnetaddress), address.into_param().abi(), ::std::mem::transmute(dwlength), ::std::mem::transmute(dwaddresssize)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Networking_WinSock")]
#[inline]
pub unsafe fn RtmConvertNetAddressToIpv6AddressAndLength(pnetaddress: *mut RTM_NET_ADDRESS, paddress: *mut super::super::Networking::WinSock::IN6_ADDR, plength: *mut u32, dwaddresssize: u32) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn RtmConvertNetAddressToIpv6AddressAndLength(pnetaddress: *mut RTM_NET_ADDRESS, paddress: *mut super::super::Networking::WinSock::IN6_ADDR, plength: *mut u32, dwaddresssize: u32) -> u32;
        }
        ::std::mem::transmute(RtmConvertNetAddressToIpv6AddressAndLength(::std::mem::transmute(pnetaddress), ::std::mem::transmute(paddress), ::std::mem::transmute(plength), ::std::mem::transmute(dwaddresssize)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn RtmCreateDestEnum(rtmreghandle: isize, targetviews: u32, enumflags: u32, netaddress: *mut RTM_NET_ADDRESS, protocolid: u32, rtmenumhandle: *mut isize) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn RtmCreateDestEnum(rtmreghandle: isize, targetviews: u32, enumflags: u32, netaddress: *mut RTM_NET_ADDRESS, protocolid: u32, rtmenumhandle: *mut isize) -> u32;
        }
        ::std::mem::transmute(RtmCreateDestEnum(::std::mem::transmute(rtmreghandle), ::std::mem::transmute(targetviews), ::std::mem::transmute(enumflags), ::std::mem::transmute(netaddress), ::std::mem::transmute(protocolid), ::std::mem::transmute(rtmenumhandle)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn RtmCreateNextHopEnum(rtmreghandle: isize, enumflags: u32, netaddress: *mut RTM_NET_ADDRESS, rtmenumhandle: *mut isize) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn RtmCreateNextHopEnum(rtmreghandle: isize, enumflags: u32, netaddress: *mut RTM_NET_ADDRESS, rtmenumhandle: *mut isize) -> u32;
        }
        ::std::mem::transmute(RtmCreateNextHopEnum(::std::mem::transmute(rtmreghandle), ::std::mem::transmute(enumflags), ::std::mem::transmute(netaddress), ::std::mem::transmute(rtmenumhandle)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn RtmCreateRouteEnum(rtmreghandle: isize, desthandle: isize, targetviews: u32, enumflags: u32, startdest: *mut RTM_NET_ADDRESS, matchingflags: u32, criteriaroute: *mut RTM_ROUTE_INFO, criteriainterface: u32, rtmenumhandle: *mut isize) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn RtmCreateRouteEnum(rtmreghandle: isize, desthandle: isize, targetviews: u32, enumflags: u32, startdest: *mut RTM_NET_ADDRESS, matchingflags: u32, criteriaroute: *mut RTM_ROUTE_INFO, criteriainterface: u32, rtmenumhandle: *mut isize) -> u32;
        }
        ::std::mem::transmute(RtmCreateRouteEnum(
            ::std::mem::transmute(rtmreghandle),
            ::std::mem::transmute(desthandle),
            ::std::mem::transmute(targetviews),
            ::std::mem::transmute(enumflags),
            ::std::mem::transmute(startdest),
            ::std::mem::transmute(matchingflags),
            ::std::mem::transmute(criteriaroute),
            ::std::mem::transmute(criteriainterface),
            ::std::mem::transmute(rtmenumhandle),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn RtmCreateRouteList(rtmreghandle: isize, routelisthandle: *mut isize) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn RtmCreateRouteList(rtmreghandle: isize, routelisthandle: *mut isize) -> u32;
        }
        ::std::mem::transmute(RtmCreateRouteList(::std::mem::transmute(rtmreghandle), ::std::mem::transmute(routelisthandle)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn RtmCreateRouteListEnum(rtmreghandle: isize, routelisthandle: isize, rtmenumhandle: *mut isize) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn RtmCreateRouteListEnum(rtmreghandle: isize, routelisthandle: isize, rtmenumhandle: *mut isize) -> u32;
        }
        ::std::mem::transmute(RtmCreateRouteListEnum(::std::mem::transmute(rtmreghandle), ::std::mem::transmute(routelisthandle), ::std::mem::transmute(rtmenumhandle)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn RtmDeleteEnumHandle(rtmreghandle: isize, enumhandle: isize) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn RtmDeleteEnumHandle(rtmreghandle: isize, enumhandle: isize) -> u32;
        }
        ::std::mem::transmute(RtmDeleteEnumHandle(::std::mem::transmute(rtmreghandle), ::std::mem::transmute(enumhandle)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn RtmDeleteNextHop(rtmreghandle: isize, nexthophandle: isize, nexthopinfo: *mut RTM_NEXTHOP_INFO) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn RtmDeleteNextHop(rtmreghandle: isize, nexthophandle: isize, nexthopinfo: *mut RTM_NEXTHOP_INFO) -> u32;
        }
        ::std::mem::transmute(RtmDeleteNextHop(::std::mem::transmute(rtmreghandle), ::std::mem::transmute(nexthophandle), ::std::mem::transmute(nexthopinfo)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn RtmDeleteRouteList(rtmreghandle: isize, routelisthandle: isize) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn RtmDeleteRouteList(rtmreghandle: isize, routelisthandle: isize) -> u32;
        }
        ::std::mem::transmute(RtmDeleteRouteList(::std::mem::transmute(rtmreghandle), ::std::mem::transmute(routelisthandle)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn RtmDeleteRouteToDest(rtmreghandle: isize, routehandle: isize, changeflags: *mut u32) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn RtmDeleteRouteToDest(rtmreghandle: isize, routehandle: isize, changeflags: *mut u32) -> u32;
        }
        ::std::mem::transmute(RtmDeleteRouteToDest(::std::mem::transmute(rtmreghandle), ::std::mem::transmute(routehandle), ::std::mem::transmute(changeflags)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn RtmDeregisterEntity(rtmreghandle: isize) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn RtmDeregisterEntity(rtmreghandle: isize) -> u32;
        }
        ::std::mem::transmute(RtmDeregisterEntity(::std::mem::transmute(rtmreghandle)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn RtmDeregisterFromChangeNotification(rtmreghandle: isize, notifyhandle: isize) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn RtmDeregisterFromChangeNotification(rtmreghandle: isize, notifyhandle: isize) -> u32;
        }
        ::std::mem::transmute(RtmDeregisterFromChangeNotification(::std::mem::transmute(rtmreghandle), ::std::mem::transmute(notifyhandle)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn RtmFindNextHop(rtmreghandle: isize, nexthopinfo: *mut RTM_NEXTHOP_INFO, nexthophandle: *mut isize, nexthoppointer: *mut *mut RTM_NEXTHOP_INFO) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn RtmFindNextHop(rtmreghandle: isize, nexthopinfo: *mut RTM_NEXTHOP_INFO, nexthophandle: *mut isize, nexthoppointer: *mut *mut RTM_NEXTHOP_INFO) -> u32;
        }
        ::std::mem::transmute(RtmFindNextHop(::std::mem::transmute(rtmreghandle), ::std::mem::transmute(nexthopinfo), ::std::mem::transmute(nexthophandle), ::std::mem::transmute(nexthoppointer)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn RtmGetChangeStatus(rtmreghandle: isize, notifyhandle: isize, desthandle: isize, changestatus: *mut super::super::Foundation::BOOL) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn RtmGetChangeStatus(rtmreghandle: isize, notifyhandle: isize, desthandle: isize, changestatus: *mut super::super::Foundation::BOOL) -> u32;
        }
        ::std::mem::transmute(RtmGetChangeStatus(::std::mem::transmute(rtmreghandle), ::std::mem::transmute(notifyhandle), ::std::mem::transmute(desthandle), ::std::mem::transmute(changestatus)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn RtmGetChangedDests(rtmreghandle: isize, notifyhandle: isize, numdests: *mut u32, changeddests: *mut RTM_DEST_INFO) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn RtmGetChangedDests(rtmreghandle: isize, notifyhandle: isize, numdests: *mut u32, changeddests: *mut RTM_DEST_INFO) -> u32;
        }
        ::std::mem::transmute(RtmGetChangedDests(::std::mem::transmute(rtmreghandle), ::std::mem::transmute(notifyhandle), ::std::mem::transmute(numdests), ::std::mem::transmute(changeddests)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn RtmGetDestInfo(rtmreghandle: isize, desthandle: isize, protocolid: u32, targetviews: u32, destinfo: *mut RTM_DEST_INFO) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn RtmGetDestInfo(rtmreghandle: isize, desthandle: isize, protocolid: u32, targetviews: u32, destinfo: *mut RTM_DEST_INFO) -> u32;
        }
        ::std::mem::transmute(RtmGetDestInfo(::std::mem::transmute(rtmreghandle), ::std::mem::transmute(desthandle), ::std::mem::transmute(protocolid), ::std::mem::transmute(targetviews), ::std::mem::transmute(destinfo)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn RtmGetEntityInfo(rtmreghandle: isize, entityhandle: isize, entityinfo: *mut RTM_ENTITY_INFO) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn RtmGetEntityInfo(rtmreghandle: isize, entityhandle: isize, entityinfo: *mut RTM_ENTITY_INFO) -> u32;
        }
        ::std::mem::transmute(RtmGetEntityInfo(::std::mem::transmute(rtmreghandle), ::std::mem::transmute(entityhandle), ::std::mem::transmute(entityinfo)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn RtmGetEntityMethods(rtmreghandle: isize, entityhandle: isize, nummethods: *mut u32, exptmethods: *mut ::std::option::Option<RTM_ENTITY_EXPORT_METHOD>) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn RtmGetEntityMethods(rtmreghandle: isize, entityhandle: isize, nummethods: *mut u32, exptmethods: *mut ::windows::runtime::RawPtr) -> u32;
        }
        ::std::mem::transmute(RtmGetEntityMethods(::std::mem::transmute(rtmreghandle), ::std::mem::transmute(entityhandle), ::std::mem::transmute(nummethods), ::std::mem::transmute(exptmethods)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn RtmGetEnumDests(rtmreghandle: isize, enumhandle: isize, numdests: *mut u32, destinfos: *mut RTM_DEST_INFO) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn RtmGetEnumDests(rtmreghandle: isize, enumhandle: isize, numdests: *mut u32, destinfos: *mut RTM_DEST_INFO) -> u32;
        }
        ::std::mem::transmute(RtmGetEnumDests(::std::mem::transmute(rtmreghandle), ::std::mem::transmute(enumhandle), ::std::mem::transmute(numdests), ::std::mem::transmute(destinfos)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn RtmGetEnumNextHops(rtmreghandle: isize, enumhandle: isize, numnexthops: *mut u32, nexthophandles: *mut isize) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn RtmGetEnumNextHops(rtmreghandle: isize, enumhandle: isize, numnexthops: *mut u32, nexthophandles: *mut isize) -> u32;
        }
        ::std::mem::transmute(RtmGetEnumNextHops(::std::mem::transmute(rtmreghandle), ::std::mem::transmute(enumhandle), ::std::mem::transmute(numnexthops), ::std::mem::transmute(nexthophandles)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn RtmGetEnumRoutes(rtmreghandle: isize, enumhandle: isize, numroutes: *mut u32, routehandles: *mut isize) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn RtmGetEnumRoutes(rtmreghandle: isize, enumhandle: isize, numroutes: *mut u32, routehandles: *mut isize) -> u32;
        }
        ::std::mem::transmute(RtmGetEnumRoutes(::std::mem::transmute(rtmreghandle), ::std::mem::transmute(enumhandle), ::std::mem::transmute(numroutes), ::std::mem::transmute(routehandles)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn RtmGetExactMatchDestination(rtmreghandle: isize, destaddress: *mut RTM_NET_ADDRESS, protocolid: u32, targetviews: u32, destinfo: *mut RTM_DEST_INFO) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn RtmGetExactMatchDestination(rtmreghandle: isize, destaddress: *mut RTM_NET_ADDRESS, protocolid: u32, targetviews: u32, destinfo: *mut RTM_DEST_INFO) -> u32;
        }
        ::std::mem::transmute(RtmGetExactMatchDestination(::std::mem::transmute(rtmreghandle), ::std::mem::transmute(destaddress), ::std::mem::transmute(protocolid), ::std::mem::transmute(targetviews), ::std::mem::transmute(destinfo)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn RtmGetExactMatchRoute(rtmreghandle: isize, destaddress: *mut RTM_NET_ADDRESS, matchingflags: u32, routeinfo: *mut RTM_ROUTE_INFO, interfaceindex: u32, targetviews: u32, routehandle: *mut isize) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn RtmGetExactMatchRoute(rtmreghandle: isize, destaddress: *mut RTM_NET_ADDRESS, matchingflags: u32, routeinfo: *mut RTM_ROUTE_INFO, interfaceindex: u32, targetviews: u32, routehandle: *mut isize) -> u32;
        }
        ::std::mem::transmute(RtmGetExactMatchRoute(::std::mem::transmute(rtmreghandle), ::std::mem::transmute(destaddress), ::std::mem::transmute(matchingflags), ::std::mem::transmute(routeinfo), ::std::mem::transmute(interfaceindex), ::std::mem::transmute(targetviews), ::std::mem::transmute(routehandle)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn RtmGetLessSpecificDestination(rtmreghandle: isize, desthandle: isize, protocolid: u32, targetviews: u32, destinfo: *mut RTM_DEST_INFO) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn RtmGetLessSpecificDestination(rtmreghandle: isize, desthandle: isize, protocolid: u32, targetviews: u32, destinfo: *mut RTM_DEST_INFO) -> u32;
        }
        ::std::mem::transmute(RtmGetLessSpecificDestination(::std::mem::transmute(rtmreghandle), ::std::mem::transmute(desthandle), ::std::mem::transmute(protocolid), ::std::mem::transmute(targetviews), ::std::mem::transmute(destinfo)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn RtmGetListEnumRoutes(rtmreghandle: isize, enumhandle: isize, numroutes: *mut u32, routehandles: *mut isize) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn RtmGetListEnumRoutes(rtmreghandle: isize, enumhandle: isize, numroutes: *mut u32, routehandles: *mut isize) -> u32;
        }
        ::std::mem::transmute(RtmGetListEnumRoutes(::std::mem::transmute(rtmreghandle), ::std::mem::transmute(enumhandle), ::std::mem::transmute(numroutes), ::std::mem::transmute(routehandles)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn RtmGetMostSpecificDestination(rtmreghandle: isize, destaddress: *mut RTM_NET_ADDRESS, protocolid: u32, targetviews: u32, destinfo: *mut RTM_DEST_INFO) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn RtmGetMostSpecificDestination(rtmreghandle: isize, destaddress: *mut RTM_NET_ADDRESS, protocolid: u32, targetviews: u32, destinfo: *mut RTM_DEST_INFO) -> u32;
        }
        ::std::mem::transmute(RtmGetMostSpecificDestination(::std::mem::transmute(rtmreghandle), ::std::mem::transmute(destaddress), ::std::mem::transmute(protocolid), ::std::mem::transmute(targetviews), ::std::mem::transmute(destinfo)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn RtmGetNextHopInfo(rtmreghandle: isize, nexthophandle: isize, nexthopinfo: *mut RTM_NEXTHOP_INFO) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn RtmGetNextHopInfo(rtmreghandle: isize, nexthophandle: isize, nexthopinfo: *mut RTM_NEXTHOP_INFO) -> u32;
        }
        ::std::mem::transmute(RtmGetNextHopInfo(::std::mem::transmute(rtmreghandle), ::std::mem::transmute(nexthophandle), ::std::mem::transmute(nexthopinfo)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn RtmGetNextHopPointer(rtmreghandle: isize, nexthophandle: isize, nexthoppointer: *mut *mut RTM_NEXTHOP_INFO) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn RtmGetNextHopPointer(rtmreghandle: isize, nexthophandle: isize, nexthoppointer: *mut *mut RTM_NEXTHOP_INFO) -> u32;
        }
        ::std::mem::transmute(RtmGetNextHopPointer(::std::mem::transmute(rtmreghandle), ::std::mem::transmute(nexthophandle), ::std::mem::transmute(nexthoppointer)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn RtmGetOpaqueInformationPointer(rtmreghandle: isize, desthandle: isize, opaqueinfopointer: *mut *mut ::std::ffi::c_void) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn RtmGetOpaqueInformationPointer(rtmreghandle: isize, desthandle: isize, opaqueinfopointer: *mut *mut ::std::ffi::c_void) -> u32;
        }
        ::std::mem::transmute(RtmGetOpaqueInformationPointer(::std::mem::transmute(rtmreghandle), ::std::mem::transmute(desthandle), ::std::mem::transmute(opaqueinfopointer)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn RtmGetRegisteredEntities(rtmreghandle: isize, numentities: *mut u32, entityhandles: *mut isize, entityinfos: *mut RTM_ENTITY_INFO) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn RtmGetRegisteredEntities(rtmreghandle: isize, numentities: *mut u32, entityhandles: *mut isize, entityinfos: *mut RTM_ENTITY_INFO) -> u32;
        }
        ::std::mem::transmute(RtmGetRegisteredEntities(::std::mem::transmute(rtmreghandle), ::std::mem::transmute(numentities), ::std::mem::transmute(entityhandles), ::std::mem::transmute(entityinfos)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn RtmGetRouteInfo(rtmreghandle: isize, routehandle: isize, routeinfo: *mut RTM_ROUTE_INFO, destaddress: *mut RTM_NET_ADDRESS) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn RtmGetRouteInfo(rtmreghandle: isize, routehandle: isize, routeinfo: *mut RTM_ROUTE_INFO, destaddress: *mut RTM_NET_ADDRESS) -> u32;
        }
        ::std::mem::transmute(RtmGetRouteInfo(::std::mem::transmute(rtmreghandle), ::std::mem::transmute(routehandle), ::std::mem::transmute(routeinfo), ::std::mem::transmute(destaddress)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn RtmGetRoutePointer(rtmreghandle: isize, routehandle: isize, routepointer: *mut *mut RTM_ROUTE_INFO) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn RtmGetRoutePointer(rtmreghandle: isize, routehandle: isize, routepointer: *mut *mut RTM_ROUTE_INFO) -> u32;
        }
        ::std::mem::transmute(RtmGetRoutePointer(::std::mem::transmute(rtmreghandle), ::std::mem::transmute(routehandle), ::std::mem::transmute(routepointer)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn RtmHoldDestination(rtmreghandle: isize, desthandle: isize, targetviews: u32, holdtime: u32) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn RtmHoldDestination(rtmreghandle: isize, desthandle: isize, targetviews: u32, holdtime: u32) -> u32;
        }
        ::std::mem::transmute(RtmHoldDestination(::std::mem::transmute(rtmreghandle), ::std::mem::transmute(desthandle), ::std::mem::transmute(targetviews), ::std::mem::transmute(holdtime)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn RtmIgnoreChangedDests(rtmreghandle: isize, notifyhandle: isize, numdests: u32, changeddests: *mut isize) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn RtmIgnoreChangedDests(rtmreghandle: isize, notifyhandle: isize, numdests: u32, changeddests: *mut isize) -> u32;
        }
        ::std::mem::transmute(RtmIgnoreChangedDests(::std::mem::transmute(rtmreghandle), ::std::mem::transmute(notifyhandle), ::std::mem::transmute(numdests), ::std::mem::transmute(changeddests)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn RtmInsertInRouteList(rtmreghandle: isize, routelisthandle: isize, numroutes: u32, routehandles: *mut isize) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn RtmInsertInRouteList(rtmreghandle: isize, routelisthandle: isize, numroutes: u32, routehandles: *mut isize) -> u32;
        }
        ::std::mem::transmute(RtmInsertInRouteList(::std::mem::transmute(rtmreghandle), ::std::mem::transmute(routelisthandle), ::std::mem::transmute(numroutes), ::std::mem::transmute(routehandles)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn RtmInvokeMethod(rtmreghandle: isize, entityhandle: isize, input: *mut RTM_ENTITY_METHOD_INPUT, outputsize: *mut u32, output: *mut RTM_ENTITY_METHOD_OUTPUT) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn RtmInvokeMethod(rtmreghandle: isize, entityhandle: isize, input: *mut RTM_ENTITY_METHOD_INPUT, outputsize: *mut u32, output: *mut RTM_ENTITY_METHOD_OUTPUT) -> u32;
        }
        ::std::mem::transmute(RtmInvokeMethod(::std::mem::transmute(rtmreghandle), ::std::mem::transmute(entityhandle), ::std::mem::transmute(input), ::std::mem::transmute(outputsize), ::std::mem::transmute(output)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn RtmIsBestRoute(rtmreghandle: isize, routehandle: isize, bestinviews: *mut u32) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn RtmIsBestRoute(rtmreghandle: isize, routehandle: isize, bestinviews: *mut u32) -> u32;
        }
        ::std::mem::transmute(RtmIsBestRoute(::std::mem::transmute(rtmreghandle), ::std::mem::transmute(routehandle), ::std::mem::transmute(bestinviews)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn RtmIsMarkedForChangeNotification(rtmreghandle: isize, notifyhandle: isize, desthandle: isize, destmarked: *mut super::super::Foundation::BOOL) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn RtmIsMarkedForChangeNotification(rtmreghandle: isize, notifyhandle: isize, desthandle: isize, destmarked: *mut super::super::Foundation::BOOL) -> u32;
        }
        ::std::mem::transmute(RtmIsMarkedForChangeNotification(::std::mem::transmute(rtmreghandle), ::std::mem::transmute(notifyhandle), ::std::mem::transmute(desthandle), ::std::mem::transmute(destmarked)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn RtmLockDestination<'a, Param2: ::windows::runtime::IntoParam<'a, super::super::Foundation::BOOL>, Param3: ::windows::runtime::IntoParam<'a, super::super::Foundation::BOOL>>(rtmreghandle: isize, desthandle: isize, exclusive: Param2, lockdest: Param3) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn RtmLockDestination(rtmreghandle: isize, desthandle: isize, exclusive: super::super::Foundation::BOOL, lockdest: super::super::Foundation::BOOL) -> u32;
        }
        ::std::mem::transmute(RtmLockDestination(::std::mem::transmute(rtmreghandle), ::std::mem::transmute(desthandle), exclusive.into_param().abi(), lockdest.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn RtmLockNextHop<'a, Param2: ::windows::runtime::IntoParam<'a, super::super::Foundation::BOOL>, Param3: ::windows::runtime::IntoParam<'a, super::super::Foundation::BOOL>>(rtmreghandle: isize, nexthophandle: isize, exclusive: Param2, locknexthop: Param3, nexthoppointer: *mut *mut RTM_NEXTHOP_INFO) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn RtmLockNextHop(rtmreghandle: isize, nexthophandle: isize, exclusive: super::super::Foundation::BOOL, locknexthop: super::super::Foundation::BOOL, nexthoppointer: *mut *mut RTM_NEXTHOP_INFO) -> u32;
        }
        ::std::mem::transmute(RtmLockNextHop(::std::mem::transmute(rtmreghandle), ::std::mem::transmute(nexthophandle), exclusive.into_param().abi(), locknexthop.into_param().abi(), ::std::mem::transmute(nexthoppointer)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn RtmLockRoute<'a, Param2: ::windows::runtime::IntoParam<'a, super::super::Foundation::BOOL>, Param3: ::windows::runtime::IntoParam<'a, super::super::Foundation::BOOL>>(rtmreghandle: isize, routehandle: isize, exclusive: Param2, lockroute: Param3, routepointer: *mut *mut RTM_ROUTE_INFO) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn RtmLockRoute(rtmreghandle: isize, routehandle: isize, exclusive: super::super::Foundation::BOOL, lockroute: super::super::Foundation::BOOL, routepointer: *mut *mut RTM_ROUTE_INFO) -> u32;
        }
        ::std::mem::transmute(RtmLockRoute(::std::mem::transmute(rtmreghandle), ::std::mem::transmute(routehandle), exclusive.into_param().abi(), lockroute.into_param().abi(), ::std::mem::transmute(routepointer)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn RtmMarkDestForChangeNotification<'a, Param3: ::windows::runtime::IntoParam<'a, super::super::Foundation::BOOL>>(rtmreghandle: isize, notifyhandle: isize, desthandle: isize, markdest: Param3) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn RtmMarkDestForChangeNotification(rtmreghandle: isize, notifyhandle: isize, desthandle: isize, markdest: super::super::Foundation::BOOL) -> u32;
        }
        ::std::mem::transmute(RtmMarkDestForChangeNotification(::std::mem::transmute(rtmreghandle), ::std::mem::transmute(notifyhandle), ::std::mem::transmute(desthandle), markdest.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn RtmReferenceHandles(rtmreghandle: isize, numhandles: u32, rtmhandles: *mut super::super::Foundation::HANDLE) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn RtmReferenceHandles(rtmreghandle: isize, numhandles: u32, rtmhandles: *mut super::super::Foundation::HANDLE) -> u32;
        }
        ::std::mem::transmute(RtmReferenceHandles(::std::mem::transmute(rtmreghandle), ::std::mem::transmute(numhandles), ::std::mem::transmute(rtmhandles)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn RtmRegisterEntity<'a, Param3: ::windows::runtime::IntoParam<'a, super::super::Foundation::BOOL>>(rtmentityinfo: *mut RTM_ENTITY_INFO, exportmethods: *mut RTM_ENTITY_EXPORT_METHODS, eventcallback: ::std::option::Option<RTM_EVENT_CALLBACK>, reserveopaquepointer: Param3, rtmregprofile: *mut RTM_REGN_PROFILE, rtmreghandle: *mut isize) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn RtmRegisterEntity(rtmentityinfo: *mut RTM_ENTITY_INFO, exportmethods: *mut ::std::mem::ManuallyDrop<RTM_ENTITY_EXPORT_METHODS>, eventcallback: ::windows::runtime::RawPtr, reserveopaquepointer: super::super::Foundation::BOOL, rtmregprofile: *mut RTM_REGN_PROFILE, rtmreghandle: *mut isize) -> u32;
        }
        ::std::mem::transmute(RtmRegisterEntity(::std::mem::transmute(rtmentityinfo), ::std::mem::transmute(exportmethods), ::std::mem::transmute(eventcallback), reserveopaquepointer.into_param().abi(), ::std::mem::transmute(rtmregprofile), ::std::mem::transmute(rtmreghandle)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn RtmRegisterForChangeNotification(rtmreghandle: isize, targetviews: u32, notifyflags: u32, notifycontext: *mut ::std::ffi::c_void, notifyhandle: *mut isize) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn RtmRegisterForChangeNotification(rtmreghandle: isize, targetviews: u32, notifyflags: u32, notifycontext: *mut ::std::ffi::c_void, notifyhandle: *mut isize) -> u32;
        }
        ::std::mem::transmute(RtmRegisterForChangeNotification(::std::mem::transmute(rtmreghandle), ::std::mem::transmute(targetviews), ::std::mem::transmute(notifyflags), ::std::mem::transmute(notifycontext), ::std::mem::transmute(notifyhandle)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn RtmReleaseChangedDests(rtmreghandle: isize, notifyhandle: isize, numdests: u32, changeddests: *mut RTM_DEST_INFO) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn RtmReleaseChangedDests(rtmreghandle: isize, notifyhandle: isize, numdests: u32, changeddests: *mut RTM_DEST_INFO) -> u32;
        }
        ::std::mem::transmute(RtmReleaseChangedDests(::std::mem::transmute(rtmreghandle), ::std::mem::transmute(notifyhandle), ::std::mem::transmute(numdests), ::std::mem::transmute(changeddests)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn RtmReleaseDestInfo(rtmreghandle: isize, destinfo: *mut RTM_DEST_INFO) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn RtmReleaseDestInfo(rtmreghandle: isize, destinfo: *mut RTM_DEST_INFO) -> u32;
        }
        ::std::mem::transmute(RtmReleaseDestInfo(::std::mem::transmute(rtmreghandle), ::std::mem::transmute(destinfo)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn RtmReleaseDests(rtmreghandle: isize, numdests: u32, destinfos: *mut RTM_DEST_INFO) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn RtmReleaseDests(rtmreghandle: isize, numdests: u32, destinfos: *mut RTM_DEST_INFO) -> u32;
        }
        ::std::mem::transmute(RtmReleaseDests(::std::mem::transmute(rtmreghandle), ::std::mem::transmute(numdests), ::std::mem::transmute(destinfos)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn RtmReleaseEntities(rtmreghandle: isize, numentities: u32, entityhandles: *mut isize) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn RtmReleaseEntities(rtmreghandle: isize, numentities: u32, entityhandles: *mut isize) -> u32;
        }
        ::std::mem::transmute(RtmReleaseEntities(::std::mem::transmute(rtmreghandle), ::std::mem::transmute(numentities), ::std::mem::transmute(entityhandles)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn RtmReleaseEntityInfo(rtmreghandle: isize, entityinfo: *mut RTM_ENTITY_INFO) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn RtmReleaseEntityInfo(rtmreghandle: isize, entityinfo: *mut RTM_ENTITY_INFO) -> u32;
        }
        ::std::mem::transmute(RtmReleaseEntityInfo(::std::mem::transmute(rtmreghandle), ::std::mem::transmute(entityinfo)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn RtmReleaseNextHopInfo(rtmreghandle: isize, nexthopinfo: *mut RTM_NEXTHOP_INFO) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn RtmReleaseNextHopInfo(rtmreghandle: isize, nexthopinfo: *mut RTM_NEXTHOP_INFO) -> u32;
        }
        ::std::mem::transmute(RtmReleaseNextHopInfo(::std::mem::transmute(rtmreghandle), ::std::mem::transmute(nexthopinfo)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn RtmReleaseNextHops(rtmreghandle: isize, numnexthops: u32, nexthophandles: *mut isize) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn RtmReleaseNextHops(rtmreghandle: isize, numnexthops: u32, nexthophandles: *mut isize) -> u32;
        }
        ::std::mem::transmute(RtmReleaseNextHops(::std::mem::transmute(rtmreghandle), ::std::mem::transmute(numnexthops), ::std::mem::transmute(nexthophandles)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn RtmReleaseRouteInfo(rtmreghandle: isize, routeinfo: *mut RTM_ROUTE_INFO) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn RtmReleaseRouteInfo(rtmreghandle: isize, routeinfo: *mut RTM_ROUTE_INFO) -> u32;
        }
        ::std::mem::transmute(RtmReleaseRouteInfo(::std::mem::transmute(rtmreghandle), ::std::mem::transmute(routeinfo)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn RtmReleaseRoutes(rtmreghandle: isize, numroutes: u32, routehandles: *mut isize) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn RtmReleaseRoutes(rtmreghandle: isize, numroutes: u32, routehandles: *mut isize) -> u32;
        }
        ::std::mem::transmute(RtmReleaseRoutes(::std::mem::transmute(rtmreghandle), ::std::mem::transmute(numroutes), ::std::mem::transmute(routehandles)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[inline]
pub unsafe fn RtmUpdateAndUnlockRoute(rtmreghandle: isize, routehandle: isize, timetolive: u32, routelisthandle: isize, notifytype: u32, notifyhandle: isize, changeflags: *mut u32) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn RtmUpdateAndUnlockRoute(rtmreghandle: isize, routehandle: isize, timetolive: u32, routelisthandle: isize, notifytype: u32, notifyhandle: isize, changeflags: *mut u32) -> u32;
        }
        ::std::mem::transmute(RtmUpdateAndUnlockRoute(::std::mem::transmute(rtmreghandle), ::std::mem::transmute(routehandle), ::std::mem::transmute(timetolive), ::std::mem::transmute(routelisthandle), ::std::mem::transmute(notifytype), ::std::mem::transmute(notifyhandle), ::std::mem::transmute(changeflags)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct SECURITY_MESSAGE {
    pub dwMsgId: SECURITY_MESSAGE_MSG_ID,
    pub hPort: isize,
    pub dwError: u32,
    pub UserName: [super::super::Foundation::CHAR; 257],
    pub Domain: [super::super::Foundation::CHAR; 16],
}
#[cfg(feature = "Win32_Foundation")]
impl SECURITY_MESSAGE {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for SECURITY_MESSAGE {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::fmt::Debug for SECURITY_MESSAGE {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("SECURITY_MESSAGE").field("dwMsgId", &self.dwMsgId).field("hPort", &self.hPort).field("dwError", &self.dwError).field("UserName", &self.UserName).field("Domain", &self.Domain).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for SECURITY_MESSAGE {
    fn eq(&self, other: &Self) -> bool {
        self.dwMsgId == other.dwMsgId && self.hPort == other.hPort && self.dwError == other.dwError && self.UserName == other.UserName && self.Domain == other.Domain
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for SECURITY_MESSAGE {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for SECURITY_MESSAGE {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct SECURITY_MESSAGE_MSG_ID(pub u32);
pub const SECURITYMSG_SUCCESS: SECURITY_MESSAGE_MSG_ID = SECURITY_MESSAGE_MSG_ID(1u32);
pub const SECURITYMSG_FAILURE: SECURITY_MESSAGE_MSG_ID = SECURITY_MESSAGE_MSG_ID(2u32);
pub const SECURITYMSG_ERROR: SECURITY_MESSAGE_MSG_ID = SECURITY_MESSAGE_MSG_ID(3u32);
impl ::std::convert::From<u32> for SECURITY_MESSAGE_MSG_ID {
    fn from(value: u32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for SECURITY_MESSAGE_MSG_ID {
    type Abi = Self;
    type DefaultType = Self;
}
impl ::std::ops::BitOr for SECURITY_MESSAGE_MSG_ID {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self {
        Self(self.0 | rhs.0)
    }
}
impl ::std::ops::BitAnd for SECURITY_MESSAGE_MSG_ID {
    type Output = Self;
    fn bitand(self, rhs: Self) -> Self {
        Self(self.0 & rhs.0)
    }
}
impl ::std::ops::BitOrAssign for SECURITY_MESSAGE_MSG_ID {
    fn bitor_assign(&mut self, rhs: Self) {
        self.0.bitor_assign(rhs.0)
    }
}
impl ::std::ops::BitAndAssign for SECURITY_MESSAGE_MSG_ID {
    fn bitand_assign(&mut self, rhs: Self) {
        self.0.bitand_assign(rhs.0)
    }
}
impl ::std::ops::Not for SECURITY_MESSAGE_MSG_ID {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
pub struct SOURCE_GROUP_ENTRY {
    pub dwSourceAddr: u32,
    pub dwSourceMask: u32,
    pub dwGroupAddr: u32,
    pub dwGroupMask: u32,
}
impl SOURCE_GROUP_ENTRY {}
impl ::std::default::Default for SOURCE_GROUP_ENTRY {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for SOURCE_GROUP_ENTRY {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("SOURCE_GROUP_ENTRY").field("dwSourceAddr", &self.dwSourceAddr).field("dwSourceMask", &self.dwSourceMask).field("dwGroupAddr", &self.dwGroupAddr).field("dwGroupMask", &self.dwGroupMask).finish()
    }
}
impl ::std::cmp::PartialEq for SOURCE_GROUP_ENTRY {
    fn eq(&self, other: &Self) -> bool {
        self.dwSourceAddr == other.dwSourceAddr && self.dwSourceMask == other.dwSourceMask && self.dwGroupAddr == other.dwGroupAddr && self.dwGroupMask == other.dwGroupMask
    }
}
impl ::std::cmp::Eq for SOURCE_GROUP_ENTRY {}
unsafe impl ::windows::runtime::Abi for SOURCE_GROUP_ENTRY {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography"))]
pub struct SSTP_CERT_INFO {
    pub isDefault: super::super::Foundation::BOOL,
    pub certBlob: super::super::Security::Cryptography::CRYPTOAPI_BLOB,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography"))]
impl SSTP_CERT_INFO {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography"))]
impl ::std::default::Default for SSTP_CERT_INFO {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography"))]
impl ::std::fmt::Debug for SSTP_CERT_INFO {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("SSTP_CERT_INFO").field("isDefault", &self.isDefault).field("certBlob", &self.certBlob).finish()
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography"))]
impl ::std::cmp::PartialEq for SSTP_CERT_INFO {
    fn eq(&self, other: &Self) -> bool {
        self.isDefault == other.isDefault && self.certBlob == other.certBlob
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography"))]
impl ::std::cmp::Eq for SSTP_CERT_INFO {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography"))]
unsafe impl ::windows::runtime::Abi for SSTP_CERT_INFO {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography"))]
pub struct SSTP_CONFIG_PARAMS {
    pub dwNumPorts: u32,
    pub dwPortFlags: u32,
    pub isUseHttps: super::super::Foundation::BOOL,
    pub certAlgorithm: u32,
    pub sstpCertDetails: SSTP_CERT_INFO,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography"))]
impl SSTP_CONFIG_PARAMS {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography"))]
impl ::std::default::Default for SSTP_CONFIG_PARAMS {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography"))]
impl ::std::fmt::Debug for SSTP_CONFIG_PARAMS {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("SSTP_CONFIG_PARAMS").field("dwNumPorts", &self.dwNumPorts).field("dwPortFlags", &self.dwPortFlags).field("isUseHttps", &self.isUseHttps).field("certAlgorithm", &self.certAlgorithm).field("sstpCertDetails", &self.sstpCertDetails).finish()
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography"))]
impl ::std::cmp::PartialEq for SSTP_CONFIG_PARAMS {
    fn eq(&self, other: &Self) -> bool {
        self.dwNumPorts == other.dwNumPorts && self.dwPortFlags == other.dwPortFlags && self.isUseHttps == other.isUseHttps && self.certAlgorithm == other.certAlgorithm && self.sstpCertDetails == other.sstpCertDetails
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography"))]
impl ::std::cmp::Eq for SSTP_CONFIG_PARAMS {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography"))]
unsafe impl ::windows::runtime::Abi for SSTP_CONFIG_PARAMS {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Networking_WinSock")]
pub struct VPN_TS_IP_ADDRESS {
    pub Type: u16,
    pub Anonymous: VPN_TS_IP_ADDRESS_0,
}
#[cfg(feature = "Win32_Networking_WinSock")]
impl VPN_TS_IP_ADDRESS {}
#[cfg(feature = "Win32_Networking_WinSock")]
impl ::std::default::Default for VPN_TS_IP_ADDRESS {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Networking_WinSock")]
impl ::std::cmp::PartialEq for VPN_TS_IP_ADDRESS {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
#[cfg(feature = "Win32_Networking_WinSock")]
impl ::std::cmp::Eq for VPN_TS_IP_ADDRESS {}
#[cfg(feature = "Win32_Networking_WinSock")]
unsafe impl ::windows::runtime::Abi for VPN_TS_IP_ADDRESS {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Networking_WinSock")]
pub union VPN_TS_IP_ADDRESS_0 {
    pub v4: super::super::Networking::WinSock::IN_ADDR,
    pub v6: super::super::Networking::WinSock::IN6_ADDR,
}
#[cfg(feature = "Win32_Networking_WinSock")]
impl VPN_TS_IP_ADDRESS_0 {}
#[cfg(feature = "Win32_Networking_WinSock")]
impl ::std::default::Default for VPN_TS_IP_ADDRESS_0 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Networking_WinSock")]
impl ::std::cmp::PartialEq for VPN_TS_IP_ADDRESS_0 {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
#[cfg(feature = "Win32_Networking_WinSock")]
impl ::std::cmp::Eq for VPN_TS_IP_ADDRESS_0 {}
#[cfg(feature = "Win32_Networking_WinSock")]
unsafe impl ::windows::runtime::Abi for VPN_TS_IP_ADDRESS_0 {
    type Abi = Self;
    type DefaultType = Self;
}
pub const VS_Default: u32 = 0u32;
pub const VS_GREOnly: u32 = 9u32;
pub const VS_Ikev2First: u32 = 8u32;
pub const VS_Ikev2Only: u32 = 7u32;
pub const VS_Ikev2Sstp: u32 = 14u32;
pub const VS_L2tpFirst: u32 = 4u32;
pub const VS_L2tpOnly: u32 = 3u32;
pub const VS_L2tpSstp: u32 = 13u32;
pub const VS_PptpFirst: u32 = 2u32;
pub const VS_PptpOnly: u32 = 1u32;
pub const VS_PptpSstp: u32 = 12u32;
pub const VS_ProtocolList: u32 = 15u32;
pub const VS_SstpFirst: u32 = 6u32;
pub const VS_SstpOnly: u32 = 5u32;
pub const WARNING_MSG_ALIAS_NOT_ADDED: u32 = 644u32;
pub const WM_RASDIALEVENT: u32 = 52429u32;
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
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
impl _MPR_VPN_SELECTOR {}
#[cfg(feature = "Win32_Networking_WinSock")]
impl ::std::default::Default for _MPR_VPN_SELECTOR {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Networking_WinSock")]
impl ::std::cmp::PartialEq for _MPR_VPN_SELECTOR {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
#[cfg(feature = "Win32_Networking_WinSock")]
impl ::std::cmp::Eq for _MPR_VPN_SELECTOR {}
#[cfg(feature = "Win32_Networking_WinSock")]
unsafe impl ::windows::runtime::Abi for _MPR_VPN_SELECTOR {
    type Abi = Self;
    type DefaultType = Self;
}
